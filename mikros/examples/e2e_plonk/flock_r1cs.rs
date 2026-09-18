//! One connected AND/XOR circuit through FLOCK's public-CSC R1CS path.
//! C is the identity. Matrix application and the single-instance lincheck
//! input conversion are online; fixed matrices and witness generation are not.

use super::{
    fixture::{self, Circuit, Mode},
    grinding::GrindingChallenger,
    queries::{lift, pairs},
};
use flock_core::{
    challenger::{Challenger, FsChallenger},
    lincheck::{self, CscCircuit, LincheckGrinding, LincheckProof, QuirkyPoint, SkipPoint},
    pcs::{self, Commitment, PcsParams, functional::TensorClaim, merged_tensor},
    r1cs::SparseBinaryMatrix,
    schedule::Registry,
    scratch,
    zerocheck::{self, ZerocheckClaim, ZerocheckProof},
};
use flock_field::F128;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    io::{self, Write},
    time::Instant,
};

fn ms(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}
fn digest<T: Serialize + ?Sized>(value: &T) -> String {
    blake3::hash(&bincode::serialize(value).unwrap())
        .to_hex()
        .to_string()
}
fn event(stage: &str) {
    eprintln!("stage={stage}");
}
fn peak_rss_kib() -> Option<u64> {
    std::fs::read_to_string("/proc/self/status")
        .ok()?
        .lines()
        .find_map(|line| {
            line.strip_prefix("VmHWM:")?
                .split_whitespace()
                .next()?
                .parse()
                .ok()
        })
}

/// [1, a_0..a_(N/2-1), b_0..b_(N/2-1), gate_0..gate_(N-1), zeros].
/// The 2N+1 live variables occupy one 4N-row matrix, not repeated small blocks.
fn row_columns(c: &Circuit, row: usize, left: bool) -> ([usize; 2], usize) {
    let n = c.n();
    if row >= 2 * n + 1 {
        return ([0, 0], 0);
    }
    if row <= n {
        return ([if left { row } else { 0 }, 0], 1);
    }
    let gate = row - n - 1;
    let j = gate / 2;
    if gate & 1 == 0 {
        ([if left { 1 + j } else { 1 + n / 2 + j }, 0], 1)
    } else if left {
        ([row - 1, 1 + ((j + 1) & (n / 2 - 1))], 2)
    } else {
        ([0, 0], 1)
    }
}

fn public_csc(c: &Circuit) -> CscCircuit {
    let rows = 4 * c.n();
    let matrix = |left| {
        SparseBinaryMatrix::new(
            rows,
            rows,
            (0..rows)
                .into_par_iter()
                .map(|row| {
                    let (columns, len) = row_columns(c, row, left);
                    columns[..len].to_vec()
                })
                .collect(),
        )
    };
    let a = matrix(true);
    let b = matrix(false);
    // CSC owns its compact transpose; these CSR construction buffers are
    // released before proving. C=I needs no stored matrix or PCS commitment.
    CscCircuit::from_matrices(&a, &b).with_const_pin(Some(0))
}

fn packed(rows: usize, value: impl Fn(usize) -> bool + Sync) -> Vec<F128> {
    let mut words = vec![F128::ZERO; rows / 128];
    words.par_iter_mut().enumerate().for_each(|(word, out)| {
        let mut lo = 0;
        let mut hi = 0;
        for bit in 0..64 {
            lo |= u64::from(value(128 * word + bit)) << bit;
            hi |= u64::from(value(128 * word + 64 + bit)) << bit;
        }
        *out = F128::new(lo, hi);
    });
    words
}

fn witness(c: &Circuit) -> Vec<F128> {
    let n = c.n();
    let mut z = packed(4 * n, |i| {
        if i == 0 {
            true
        } else if i <= n / 2 {
            c.word(0, 2 * (i - 1)).lo & 1 != 0
        } else if i <= n {
            c.word(1, 2 * (i - n / 2 - 1)).lo & 1 != 0
        } else if i <= 2 * n {
            c.word(2, i - n - 1).lo & 1 != 0
        } else {
            false
        }
    });
    z.resize(z.len().max(1 << 15), F128::ZERO);
    z
}

fn matrix_images(c: &Circuit, z: &[F128]) -> [Vec<F128>; 2] {
    [true, false].map(|left| {
        packed(4 * c.n(), |row| {
            let (columns, len) = row_columns(c, row, left);
            columns[..len]
                .iter()
                .fold(false, |value, &column| value ^ fixture::bit(z, column))
        })
    })
}

struct Setup<'a> {
    circuit: &'a Circuit,
    matrix: CscCircuit,
    witness: Vec<F128>,
    registry: Registry,
    params: PcsParams,
    offline_ms: f64,
}
impl<'a> Setup<'a> {
    fn new(c: &'a Circuit) -> Result<Self, String> {
        if c.mode != Mode::Single || c.log_gates > 29 {
            return Err(
                "single-instance public CSC requires single mode and 4N < 2^32 rows".into(),
            );
        }
        event("offline_native_r1cs_setup");
        let start = Instant::now();
        let log_words = (4 * c.wire_words()).max(1 << 15).ilog2() as usize;
        let registry = fixture::registry(log_words, c.wire_words().ilog2() as usize);
        let params = fixture::parameters(&fixture::union(&registry));
        params
            .ligerito_prover_config()
            .map_err(|e| format!("R1CS PCS parameters: {e:?}"))?;
        let matrix = public_csc(c);
        let witness = witness(c);
        Ok(Self {
            circuit: c,
            matrix,
            witness,
            registry,
            params,
            offline_ms: ms(start),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Proof {
    witness: Commitment,
    zerocheck: ZerocheckProof,
    lincheck: LincheckProof,
    nonces: Vec<u64>,
    witness_pcs: merged_tensor::Proof,
}

fn bind(c: &Circuit, witness: &Commitment) -> FsChallenger {
    let mut ch = FsChallenger::new(b"flock-single-connected-and-xor-public-csc-v1");
    ch.observe_bytes(&c.digest());
    ch.observe_bytes(&((c.log_gates + 2) as u64).to_le_bytes());
    ch.observe_label(b"one-full-matrix-C-identity-constant-zero-column-skip7");
    ch.observe_bytes(&bincode::serialize(witness).unwrap());
    ch
}

fn input_point(c: &Circuit, zc: &ZerocheckClaim) -> Result<QuirkyPoint, String> {
    if zc.mlv_challenges.len() != c.log_gates + 2 - 7 {
        return Err("native single-instance zerocheck dimension".into());
    }
    Ok(QuirkyPoint {
        z_skip: SkipPoint::Phi8(zc.z),
        x_inner_rest: zc.mlv_challenges.clone(),
        x_outer: vec![],
    })
}

fn claims(
    c: &Circuit,
    log_words: usize,
    zc: &ZerocheckClaim,
    lc: &lincheck::LincheckClaim,
) -> Vec<TensorClaim> {
    let index = 2 * c.n(); // Last gate's output in the R1CS witness.
    let mut output_columns = vec![F128::ZERO; 128];
    output_columns[index & 127] = F128::ONE;
    vec![
        lift(
            TensorClaim {
                columns: lc.r_inner_skip.weights(7),
                rows: pairs(&lc.r_inner_rest),
                value: lc.w,
            },
            0,
            log_words,
        ),
        lift(
            TensorClaim {
                columns: SkipPoint::Phi8(zc.z).weights(7),
                rows: pairs(&zc.r_rest),
                value: zc.c_eval,
            },
            0,
            log_words,
        ),
        TensorClaim {
            columns: output_columns,
            rows: (0..log_words)
                .map(|j| {
                    if (index >> (j + 7)) & 1 == 0 {
                        [F128::ONE, F128::ZERO]
                    } else {
                        [F128::ZERO, F128::ONE]
                    }
                })
                .collect(),
            value: c.io,
        },
    ]
}

fn prove(s: &Setup<'_>, z: Vec<F128>) -> Result<(Proof, Value, FsChallenger), String> {
    let c = s.circuit;
    let m = c.log_gates + 2;
    let logical_words = 1usize << (m - 7);
    let begin = Instant::now();
    event("online_witness_commitment");
    let start = Instant::now();
    let (commitment, data) = pcs::commit(&z, &s.params);
    let commit_ms = ms(start);
    let mut ch = GrindingChallenger::prover(bind(c, &commitment));
    event("online_native_matrix_images");
    let start = Instant::now();
    let [a, b] = matrix_images(c, &z);
    let matrix_apply_ms = ms(start);
    event("online_gate_piop");
    let start = Instant::now();
    let (zerocheck, zc) = zerocheck::prove_packed_skip7(
        fixture::bytes(&a),
        fixture::bytes(&b),
        fixture::bytes(&z[..logical_words]),
        m,
        &mut ch,
    );
    let gates_ms = ms(start);
    drop(a);
    drop(b);
    scratch::clear();
    event("online_lincheck_input_conversion");
    let start = Instant::now();
    let input = lincheck::pack_z_lincheck_from_packed(&z[..logical_words], m, m);
    let lincheck_input_ms = ms(start);
    event("online_public_csc_lincheck");
    let start = Instant::now();
    let (lincheck, lc) = lincheck::prove_padded_with_grinding(
        &input,
        m,
        m,
        7,
        2 * c.n() + 1,
        &s.matrix,
        &input_point(c, &zc)?,
        LincheckGrinding::disabled(),
        &mut ch,
    );
    let lincheck_ms = ms(start);
    drop(input);
    scratch::clear();
    let queries = claims(c, z.len().ilog2() as usize, &zc, &lc);
    let (mut ch, nonces) = ch.finish()?;
    let piop_blake3 = digest(&(&zerocheck, &lincheck, &nonces));
    let terminal_claims_blake3 = digest(&queries);
    event("online_witness_pcs_open");
    let start = Instant::now();
    let witness_pcs = merged_tensor::open_identity(
        z,
        &data,
        &commitment,
        &queries,
        &fixture::union(&s.registry),
        &mut ch,
    )
    .map_err(|e| format!("native R1CS PCS open: {e:?}"))?;
    let witness_pcs_open_ms = ms(start);
    drop(data);
    scratch::clear();
    let witness_ring_claims = witness_pcs
        .merged
        .as_ref()
        .map_or(0, |p| p.ring_switches.len());
    let proof = Proof {
        witness: commitment,
        zerocheck,
        lincheck,
        nonces,
        witness_pcs,
    };
    let times = json!({"commit_ms":commit_ms,"matrix_apply_ms":matrix_apply_ms,
        "gates_ms":gates_ms,"lincheck_input_ms":lincheck_input_ms,"lincheck_ms":lincheck_ms,
        "witness_pcs_open_ms":witness_pcs_open_ms,"native_pcs_claims":queries.len(),
        "witness_claims":queries.len(),"witness_ring_claims":witness_ring_claims,
        "piop_blake3":piop_blake3,"terminal_claims_blake3":terminal_claims_blake3,
        "prover_ms":ms(begin)});
    Ok((proof, times, ch))
}

/// Verifies against the fixed public CSC and statement only; no witness or
/// generated A/B vectors are available to this routine. Full public-matrix
/// folding in native lincheck is included in verifier time.
fn verify(c: &Circuit, matrix: &CscCircuit, p: &Proof) -> Result<(FsChallenger, Value), String> {
    let log_words = (4 * c.wire_words()).max(1 << 15).ilog2() as usize;
    let registry = fixture::registry(log_words, c.wire_words().ilog2() as usize);
    let union = fixture::union(&registry);
    if p.witness.params != fixture::parameters(&union) || !p.lincheck.matrix_evals.is_empty() {
        return Err("native single R1CS PCS shape or deferred matrix claims".into());
    }
    let m = c.log_gates + 2;
    let mut ch = GrindingChallenger::verifier(bind(c, &p.witness), &p.nonces);
    let start = Instant::now();
    let zc = zerocheck::verify_skip7(m, &p.zerocheck, &mut ch)
        .map_err(|e| format!("native R1CS zerocheck: {e:?}"))?;
    let gates_ms = ms(start);
    let start = Instant::now();
    let lc = lincheck::verify_with_grinding(
        m,
        m,
        7,
        matrix,
        &input_point(c, &zc)?,
        zc.a_eval,
        zc.b_eval,
        &p.lincheck,
        LincheckGrinding::disabled(),
        &mut ch,
    )
    .map_err(|e| format!("native public-CSC lincheck: {e:?}"))?;
    let lincheck_ms = ms(start);
    let queries = claims(c, log_words, &zc, &lc);
    let (mut ch, _) = ch.finish()?;
    let start = Instant::now();
    merged_tensor::verify(&p.witness, &queries, &p.witness_pcs, &union, &mut ch)
        .map_err(|e| format!("native R1CS PCS verify: {e:?}"))?;
    Ok((
        ch,
        json!({"gates_ms":gates_ms,"lincheck_ms":lincheck_ms,"witness_pcs_ms":ms(start)}),
    ))
}

pub fn run(c: &Circuit, repetitions: usize, warmups: usize) -> Result<(), String> {
    let s = Setup::new(c)?;
    println!(
        "{}",
        json!({"type":"metadata","mode":"single","system":"flock",
        "log_gates":c.log_gates,"instances":1,"threads":rayon::current_num_threads(),
        "all_core_threads":flock_parallel::all_core_pool().current_num_threads(),"seed":c.seed,
        "repetitions":repetitions,"warmups":warmups,"baseline_skip":7,
        "fixture_digest":c.digest().iter().map(|b|format!("{b:02x}")).collect::<String>(),
        "arithmetization":"native R1CS","matrix_authentication":"public CSC","permutation_proof":false,
        "public_io":[c.io.lo,c.io.hi],"genuine_gate_count":c.n(),"r1cs_rows":4*c.n(),
        "r1cs_outer_blocks":1,"r1cs_matrix_log":c.log_gates+2,"C":"identity",
        "r1cs_auxiliary_rows":"one pinned constant row, N input identity rows, 2N-1 zero-padding rows",
        "witness_layout":"constant; N private input bits; N gate outputs; zero padding",
        "witness_words":s.witness.len(),"pcs_backend":"FLOCK native Ligerito Fast",
        "pcs_packed_direct":std::env::var_os("FLOCK_PCS_PACKED_DIRECT").is_some_and(|v|v=="1"),
        "native_lincheck_extension":"zero-dimensional outer tensor uses ordinary packed bits",
        "timing_scope":"online witness commit, A/B matrix application, skip7 zerocheck, input conversion, public-CSC lincheck and native PCS; witness generation and fixed CSC construction offline; serialization included",
        "target_features":{"gfni":cfg!(target_feature="gfni"),"avx512f":cfg!(target_feature="avx512f"),"avx512bw":cfg!(target_feature="avx512bw"),"vpclmulqdq":cfg!(target_feature="vpclmulqdq")}})
    );
    io::stdout().flush().map_err(|e| e.to_string())?;
    for iteration in 0..warmups + repetitions {
        let warmup = iteration < warmups;
        let repetition = if warmup {
            iteration
        } else {
            iteration - warmups
        };
        let count = if warmup { warmups } else { repetitions };
        let kind = if warmup { "warmup" } else { "sample" };
        eprintln!(
            "[single constraints] flock {kind} {}/{} start (2^{} gates, public CSC)",
            repetition + 1,
            count,
            c.log_gates
        );
        let owned = s.witness.clone();
        scratch::clear();
        let (proof, mut times, mut prover) = prove(&s, owned)?;
        let start = Instant::now();
        let encoded = bincode::serialize(&proof).map_err(|e| e.to_string())?;
        let serialization_ms = ms(start);
        times["serialization_ms"] = json!(serialization_ms);
        times["prover_ms"] = json!(times["prover_ms"].as_f64().unwrap() + serialization_ms);
        let peak_rss_kib = peak_rss_kib();
        event("verify_full_proof");
        let start = Instant::now();
        let decoded: Proof = bincode::deserialize(&encoded).map_err(|e| e.to_string())?;
        let (mut verifier, verifier_times) = verify(c, &s.matrix, &decoded)?;
        let verify_ms = ms(start);
        if prover.sample_f128() != verifier.sample_f128() {
            return Err("native R1CS prover/verifier transcript mismatch".into());
        }
        let row = json!({"type":"sample","mode":"single","system":"flock","log_gates":c.log_gates,
            "gates_per_instance":c.n(),"instances":1,"total_gates":c.n(),"threads":rayon::current_num_threads(),
            "seed":c.seed,"repetition":repetition,"warmup":warmup,"times":times,"verifier_times":verifier_times,
            "offline_ms":s.offline_ms,"verify_ms":verify_ms,"proof_bytes":encoded.len(),"proof_blake3":blake3::hash(&encoded).to_hex().to_string(),
            "witness_commitment_blake3":digest(&proof.witness),"witness_pcs_params":s.params,
            "verified":true,"transcript_match":true,"pcs_config_match":true,"peak_rss_kib":peak_rss_kib,
            "piop_blake3":times["piop_blake3"],"terminal_claims_blake3":times["terminal_claims_blake3"],
            "pcs_path":"FLOCK native witness commitment + merged_tensor/open_batch_merged + Ligerito Fast",
            "memory_policy":"retain witness PCS data; no public or sigma commitment; fixed CSC offline",
            "rss_scope":"process VmHWM after proving, before verification; includes public matrix setup and prior repetitions",
            "circuit":"one connected alternating AND/XOR circuit; one public final output"});
        eprintln!(
            "[single constraints] flock {kind} {}/{} finished: prover={:.3} ms verifier={verify_ms:.3} ms proof={} bytes",
            repetition + 1,
            count,
            row["times"]["prover_ms"].as_f64().unwrap(),
            encoded.len()
        );
        println!("{row}");
        io::stdout().flush().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::lincheck::LincheckCircuit;

    #[test]
    fn native_r1cs_encodes_the_same_gates_and_full_public_matrices() {
        let c = Circuit::new(14, Mode::Single, 97);
        let z = witness(&c);
        let [a, b] = matrix_images(&c, &z);
        for row in 0..4 * c.n() {
            assert_eq!(
                fixture::bit(&a, row) & fixture::bit(&b, row),
                fixture::bit(&z, row)
            );
        }
        assert_eq!(fixture::bit(&z, 2 * c.n()), c.io == F128::ONE);
        let matrix = public_csc(&c);
        let weights: Vec<_> = (0..4 * c.n())
            .map(|i| F128::new(i as u64 + 3, (i as u64).rotate_left(9)))
            .collect();
        for left in [true, false] {
            let actual = matrix.fold_side(&weights, left);
            let mut expected = vec![F128::ZERO; weights.len()];
            for (row, &weight) in weights.iter().enumerate() {
                let (columns, len) = row_columns(&c, row, left);
                for &column in &columns[..len] {
                    expected[column] += weight;
                }
            }
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn native_single_r1cs_roundtrip_and_tampering() {
        let c = Circuit::new(14, Mode::Single, 97);
        let setup = Setup::new(&c).unwrap();
        let (proof, _, mut prover) = prove(&setup, setup.witness.clone()).unwrap();
        let encoded = bincode::serialize(&proof).unwrap();
        let proof: Proof = bincode::deserialize(&encoded).unwrap();
        let (mut verifier, _) = verify(&c, &setup.matrix, &proof).unwrap();
        assert_eq!(prover.sample_f128(), verifier.sample_f128());
        let mut bad = proof.clone();
        bad.lincheck.z_partial[0] += F128::ONE;
        assert!(verify(&c, &setup.matrix, &bad).is_err());
        let mut bad = proof.clone();
        bad.nonces.pop();
        assert!(verify(&c, &setup.matrix, &bad).is_err());
        let mut bad = proof.clone();
        bad.witness_pcs.merged = None;
        assert!(verify(&c, &setup.matrix, &bad).is_err());
        let mut wrong_output = c.clone();
        wrong_output.io += F128::ONE;
        assert!(verify(&wrong_output, &setup.matrix, &proof).is_err());
        // Keep the original circuit, public output, proof and transcript.
        // Removing A[0,0] changes its product with the pinned constant one.
        let rows = 4 * c.n();
        let mut a_rows: Vec<_> = (0..rows)
            .map(|row| {
                let (columns, len) = row_columns(&c, row, true);
                columns[..len].to_vec()
            })
            .collect();
        a_rows[0].clear();
        let b_rows = (0..rows)
            .map(|row| {
                let (columns, len) = row_columns(&c, row, false);
                columns[..len].to_vec()
            })
            .collect();
        let wrong_matrix = CscCircuit::from_matrices(
            &SparseBinaryMatrix::new(rows, rows, a_rows),
            &SparseBinaryMatrix::new(rows, rows, b_rows),
        )
        .with_const_pin(Some(0));
        assert!(verify(&c, &wrong_matrix, &proof).is_err());
    }
}
