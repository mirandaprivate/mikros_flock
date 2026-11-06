//! One binary circuit, identical public R1CS matrices and native PCS in both arms.
//! Mikros uses its GF(256) Hadamard reduction and its own ordinary IP kernel.
//! Public matrix terminals are evaluated directly; there is no copy argument.
#![recursion_limit = "256"]
#![allow(dead_code)]

#[path = "e2e_plonk/fixture.rs"]
mod fixture;
#[path = "e2e_plonk/flock_r1cs.rs"]
mod flock_r1cs;
#[path = "e2e_common/grinding.rs"]
mod grinding;
#[path = "e2e_plonk/queries.rs"]
mod queries;

use fixture::{Circuit, Mode};
use flock_core::{
    challenger::{Challenger, FsChallenger},
    lincheck::LincheckCircuit,
    pcs::{
        self, Commitment,
        functional::{TensorClaim, row_weights},
        merged_tensor,
    },
    scratch,
};
use flock_field::{F128, F256Unreduced};
use grinding::GrindingChallenger;
use mikros::{InnerProductProof, MatrixClaim, hadamard, inner_product};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    env,
    io::{self, Write},
    time::Instant,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Proof {
    witness: Commitment,
    hadamard: hadamard::Proof,
    linear_ip: InnerProductProof,
    nonces: Vec<u64>,
    witness_pcs: merged_tensor::Proof,
}

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}
fn event(stage: &str) {
    eprintln!("stage={stage}");
}
fn digest<T: Serialize + ?Sized>(value: &T) -> String {
    blake3::hash(&bincode::serialize(value).unwrap())
        .to_hex()
        .to_string()
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
fn bind(c: &Circuit, witness: &Commitment) -> FsChallenger {
    let mut ch = FsChallenger::new(b"mikros-single-public-r1cs-native-pcs-v1");
    ch.observe_bytes(&c.digest());
    ch.observe_bytes(flock_r1cs::matrix_digest(c).as_bytes());
    ch.observe_label(b"one-full-matrix-C-identity-constant-col0-kappa128");
    ch.observe_bytes(&bincode::serialize(witness).unwrap());
    ch
}

fn check_query(circuit: &dyn LincheckCircuit, query: &MatrixClaim) -> Result<(), String> {
    let n = circuit.n_cols();
    if !n.is_power_of_two()
        || n < 128
        || query.column_weights.len() != 128
        || query.row_factors.len() != n.ilog2() as usize - 7
    {
        return Err("public matrix query dimensions".into());
    }
    Ok(())
}

/// Exact bit-index weights. Arbitrary/zero-sum row factors are never normalized.
fn query_weights(circuit: &dyn LincheckCircuit, query: &MatrixClaim) -> Result<Vec<F128>, String> {
    check_query(circuit, query)?;
    let low_factors: Vec<_> = query.row_factors.iter().rev().copied().collect();
    let rows = row_weights(&low_factors);
    Ok((0..circuit.n_cols())
        .into_par_iter()
        .map(|i| rows[i >> 7] * query.column_weights[i & 127])
        .collect())
}

fn linear_challenges<C: Challenger>(
    circuit: &dyn LincheckCircuit,
    a: &MatrixClaim,
    b: &MatrixClaim,
    ch: &mut C,
) -> Result<(F128, F128, F128), String> {
    check_query(circuit, a)?;
    check_query(circuit, b)?;
    if circuit.const_pin_col() != Some(0) {
        return Err("public R1CS constant pin".into());
    }
    ch.observe_label(b"mikros-public-r1cs-batched-linear-claims-v1");
    ch.observe_bytes(&(circuit.n_cols() as u64).to_le_bytes());
    for query in [a, b] {
        ch.observe_f128_slice(&query.column_weights);
        for pair in &query.row_factors {
            ch.observe_f128_slice(pair);
        }
        ch.observe_f128(query.value);
    }
    // Both complete upstream claims are fixed before either coefficient.
    let alpha = ch.sample_f128();
    let beta = ch.sample_f128();
    Ok((alpha, beta, alpha * a.value + b.value + beta))
}

/// Only one side's row weights are live at a time. No unused A/B fold is built.
fn linear_weights(
    circuit: &dyn LincheckCircuit,
    a: &MatrixClaim,
    b: &MatrixClaim,
    alpha: F128,
    beta: F128,
) -> Result<Vec<F128>, String> {
    let fold = |query: &MatrixClaim, left| -> Result<Vec<F128>, String> {
        let weights = query_weights(circuit, query)?;
        let result = circuit.fold_side(&weights, left);
        if result.len() != circuit.n_cols() {
            return Err("public matrix fold dimensions".into());
        }
        Ok(result) // weights are dropped before the next matrix is expanded.
    };
    let mut combined = fold(a, true)?;
    let right = fold(b, false)?;
    combined
        .par_iter_mut()
        .zip(&right)
        .for_each(|(a, &b)| *a = alpha * *a + b);
    drop(right);
    combined[0] += beta;
    Ok(combined)
}

#[cfg(test)]
fn expand_binary(
    z: &[F128],
    len: usize,
    weights: &[F128],
    target: F128,
) -> Result<Vec<F128>, String> {
    if weights.len() != len || len > z.len() * 128 {
        return Err("linear witness shape".into());
    }
    let actual = weights
        .par_iter()
        .enumerate()
        .map(|(i, &weight)| {
            if fixture::bit(z, i) {
                weight
            } else {
                F128::ZERO
            }
        })
        .reduce(|| F128::ZERO, |a, b| a + b);
    if actual != target {
        return Err("public linear identity or constant pin".into());
    }
    Ok((0..len)
        .into_par_iter()
        .map(|i| {
            if fixture::bit(z, i) {
                F128::ONE
            } else {
                F128::ZERO
            }
        })
        .collect())
}

/// Direct public MLE evaluation with two small equality tensors, not a second
/// length-4N F128 allocation. `point` follows the IP's high-first convention.
fn public_mle(values: &[F128], point: &[F128]) -> Result<F128, String> {
    if point.len() >= usize::BITS as usize || values.len() != 1usize << point.len() {
        return Err("public terminal dimensions".into());
    }
    let low: Vec<_> = point.iter().rev().copied().collect();
    let cut = low.len() / 2;
    let inner = row_weights(&queries::pairs(&low[..cut]));
    let outer = row_weights(&queries::pairs(&low[cut..]));
    Ok(values
        .par_chunks(inner.len())
        .zip(&outer)
        .map(|(chunk, &factor)| {
            chunk
                .iter()
                .zip(&inner)
                .fold(F256Unreduced::ZERO, |sum, (&x, &w)| {
                    sum ^ x.mul_unreduced(w)
                })
                .reduce()
                * factor
        })
        .reduce(|| F128::ZERO, |a, b| a + b))
}

fn bit_mle_claim(point: &[F128], value: F128, log_words: usize) -> Result<TensorClaim, String> {
    if point.len() < 7 || point.len() - 7 > log_words {
        return Err("binary MLE query dimensions".into());
    }
    let low: Vec<_> = point.iter().rev().copied().collect();
    Ok(queries::lift(
        TensorClaim {
            columns: row_weights(&queries::pairs(&low[..7])),
            rows: queries::pairs(&low[7..]),
            value,
        },
        0,
        log_words,
    ))
}

fn pcs_claims(
    c: &Circuit,
    log_words: usize,
    source_c: &MatrixClaim,
    point: &[F128],
    value: F128,
) -> Result<Vec<TensorClaim>, String> {
    let output = 2 * c.n();
    let mut output_columns = vec![F128::ZERO; 128];
    output_columns[output & 127] = F128::ONE;
    Ok(vec![
        bit_mle_claim(point, value, log_words)?,
        queries::lift(queries::matrix(source_c.clone()), 0, log_words),
        TensorClaim {
            columns: output_columns,
            rows: (0..log_words)
                .map(|j| {
                    if (output >> (j + 7)) & 1 == 0 {
                        [F128::ONE, F128::ZERO]
                    } else {
                        [F128::ZERO, F128::ONE]
                    }
                })
                .collect(),
            value: c.io,
        },
    ])
}

fn prove(s: &flock_r1cs::Setup<'_>, z: Vec<F128>) -> Result<(Proof, Value, FsChallenger), String> {
    let c = s.circuit;
    let m = c.log_gates + 2;
    let logical_words = (4 * c.n()) / 128;
    let begin = Instant::now();
    event("online_witness_commitment");
    let t = Instant::now();
    let (commitment, data) = pcs::commit(&z, &s.params);
    let commit_ms = ms(t);
    let mut ch = GrindingChallenger::prover(bind(c, &commitment));
    event("online_native_matrix_images");
    let t = Instant::now();
    let [a, b] = flock_r1cs::matrix_images(c, &z);
    let matrix_apply_ms = ms(t);
    event("online_gate_piop");
    let t = Instant::now();
    let (hadamard, source) = hadamard::prove(
        fixture::bytes(&a),
        fixture::bytes(&b),
        fixture::bytes(&z[..logical_words]),
        m,
        mikros::Kernel::PairAggregate,
        &mut ch,
    )
    .map_err(|e| format!("Mikros R1CS Hadamard: {e}"))?;
    let gates_ms = ms(t);
    drop(a);
    drop(b);
    scratch::clear();
    event("online_public_csc_linear_prepare");
    let t = Instant::now();
    let (alpha, beta, _target) = linear_challenges(&s.matrix, &source.a, &source.b, &mut ch)?;
    let public = linear_weights(&s.matrix, &source.a, &source.b, alpha, beta)?;
    let linear_prepare_ms = ms(t);
    // The native IP reads packed witness bits directly in its first fold.
    // There is no separate F128 witness conversion; the IP and full verifier
    // check the target. Independent source scans belong only in tests.
    let lincheck_input_ms = 0.0;
    event("online_mikros_linear_ip");
    let t = Instant::now();
    let (linear_ip, point) = inner_product::prove_native_binary_left(
        fixture::bytes(&z[..logical_words]),
        public,
        &mut ch,
    );
    let lincheck_ms = ms(t);
    let queries = pcs_claims(
        c,
        z.len().ilog2() as usize,
        &source.c,
        &point,
        linear_ip.final_a,
    )?;
    let (mut ch, nonces) = ch.finish()?;
    let piop_blake3 = digest(&(&hadamard, &linear_ip, &nonces));
    let terminal_claims_blake3 = digest(&queries);
    event("online_witness_pcs_open");
    let t = Instant::now();
    let witness_pcs = merged_tensor::open_identity(
        z,
        &data,
        &commitment,
        &queries,
        &fixture::union(&s.registry),
        &mut ch,
    )
    .map_err(|e| format!("native PCS open: {e:?}"))?;
    let witness_pcs_open_ms = ms(t);
    let witness_ring_claims = witness_pcs
        .merged
        .as_ref()
        .map_or(0, |p| p.ring_switches.len());
    drop(data);
    scratch::clear();
    let proof = Proof {
        witness: commitment,
        hadamard,
        linear_ip,
        nonces,
        witness_pcs,
    };
    let times = json!({"commit_ms":commit_ms,"matrix_apply_ms":matrix_apply_ms,
        "gates_ms":gates_ms,"linear_prepare_ms":linear_prepare_ms,"lincheck_input_ms":lincheck_input_ms,
        "lincheck_ms":lincheck_ms,"witness_pcs_open_ms":witness_pcs_open_ms,
        "native_pcs_claims":3,"witness_claims":3,"public_claims":0,
        "witness_commitments":1,"public_commitments":0,"witness_ring_claims":witness_ring_claims,
        "piop_blake3":piop_blake3,"terminal_claims_blake3":terminal_claims_blake3,
        "prover_ms":ms(begin)});
    Ok((proof, times, ch))
}

/// Only the public circuit/CSC and proof are available here. No source oracle.
fn verify(
    c: &Circuit,
    matrix: &dyn LincheckCircuit,
    p: &Proof,
) -> Result<(FsChallenger, Value), String> {
    let log_words = (4 * c.wire_words()).max(1 << 15).ilog2() as usize;
    let registry = fixture::registry(log_words, c.wire_words().ilog2() as usize);
    let union = fixture::union(&registry);
    let m = c.log_gates + 2;
    if p.witness.params != fixture::parameters(&union)
        || matrix.n_cols() != 4 * c.n()
        || p.linear_ip.rounds.len() != m
    {
        return Err("Mikros public R1CS proof shape or PCS parameters".into());
    }
    let mut ch = GrindingChallenger::verifier(bind(c, &p.witness), &p.nonces);
    let t = Instant::now();
    let source =
        hadamard::verify(&p.hadamard, m, &mut ch).map_err(|e| format!("Hadamard replay: {e}"))?;
    let gates_ms = ms(t);
    let t = Instant::now();
    let (alpha, beta, target) = linear_challenges(matrix, &source.a, &source.b, &mut ch)?;
    let point = inner_product::verify(&p.linear_ip, target, &mut ch)
        .map_err(|e| format!("linear IP replay: {e}"))?;
    let public = linear_weights(matrix, &source.a, &source.b, alpha, beta)?;
    if public_mle(&public, &point)? != p.linear_ip.final_b {
        return Err("public matrix terminal".into());
    }
    drop(public);
    let lincheck_ms = ms(t);
    let queries = pcs_claims(c, log_words, &source.c, &point, p.linear_ip.final_a)?;
    let (mut ch, _) = ch.finish()?;
    let t = Instant::now();
    merged_tensor::verify(&p.witness, &queries, &p.witness_pcs, &union, &mut ch)
        .map_err(|e| format!("native PCS verify: {e:?}"))?;
    Ok((
        ch,
        json!({"gates_ms":gates_ms,"lincheck_ms":lincheck_ms,"witness_pcs_ms":ms(t)}),
    ))
}

fn run_mikros(c: &Circuit, repetitions: usize, warmups: usize) -> Result<(), String> {
    let s = flock_r1cs::Setup::new(c)?;
    println!(
        "{}",
        json!({"type":"metadata","mode":"single","system":"mikros",
        "log_gates":c.log_gates,"instances":1,"threads":rayon::current_num_threads(),
        "all_core_threads":flock_parallel::all_core_pool().current_num_threads(),"seed":c.seed,
        "repetitions":repetitions,"warmups":warmups,"baseline_skip":7,"compression_kappa":128,
        "fixture_digest":c.digest().iter().map(|b|format!("{b:02x}")).collect::<String>(),
        "matrix_digest":flock_r1cs::matrix_digest(c),"witness_digest":s.witness_digest,
        "matrix_digest_scope":"public row_columns recipe and dimensions; not serialized CSC bytes",
        "arithmetization":"native R1CS","matrix_authentication":"public CSC","permutation_proof":false,
        "public_io":[c.io.lo,c.io.hi],"genuine_gate_count":c.n(),"r1cs_rows":4*c.n(),
        "r1cs_outer_blocks":1,"r1cs_matrix_log":c.log_gates+2,"C":"identity",
        "witness_claims":3,"public_claims":0,"witness_commitments":1,"public_commitments":0,
        "witness_layout":"constant; N private input bits; N gate outputs; zero padding",
        "witness_words":s.witness.len(),"pcs_backend":"FLOCK native Ligerito Fast",
        "pcs_packed_direct":env::var_os("FLOCK_PCS_PACKED_DIRECT").is_some_and(|v|v=="1"),
        "linear_reduction":"Mikros ordinary IP on binary witness and alpha A^T lambda_A + B^T lambda_B + beta e_const; direct public CSC terminal",
        "linear_input_conversion":"packed binary witness conversion fused into the first native IP fold; no full expanded witness vector",
        "timing_scope":"online witness commit, A/B matrix application, packed Hadamard, public CSC weights, binary first-fold conversion, Mikros ordinary IP and native PCS; fixed matrix setup and witness generation offline; serialization included",
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
            "[public R1CS] mikros {kind} {}/{} start (2^{} gates)",
            repetition + 1,
            count,
            c.log_gates
        );
        let owned = s.witness.clone();
        scratch::clear();
        let (proof, mut times, mut prover) = prove(&s, owned)?;
        let t = Instant::now();
        let encoded = bincode::serialize(&proof).map_err(|e| e.to_string())?;
        let serialization_ms = ms(t);
        times["serialization_ms"] = json!(serialization_ms);
        times["prover_ms"] = json!(times["prover_ms"].as_f64().unwrap() + serialization_ms);
        let peak_rss_kib = peak_rss_kib();
        event("verify_full_proof");
        let t = Instant::now();
        let decoded: Proof = bincode::deserialize(&encoded).map_err(|e| e.to_string())?;
        let (mut verifier, verifier_times) = verify(c, &s.matrix, &decoded)?;
        let verify_ms = ms(t);
        if prover.sample_f128() != verifier.sample_f128() {
            return Err("Mikros public R1CS transcript mismatch".into());
        }
        let row = json!({"type":"sample","mode":"single","system":"mikros","log_gates":c.log_gates,
            "gates_per_instance":c.n(),"instances":1,"total_gates":c.n(),"threads":rayon::current_num_threads(),
            "seed":c.seed,"repetition":repetition,"warmup":warmup,"times":times,"verifier_times":verifier_times,
            "offline_ms":s.offline_ms,"verify_ms":verify_ms,"proof_bytes":encoded.len(),
            "proof_blake3":blake3::hash(&encoded).to_hex().to_string(),"witness_commitment_blake3":digest(&proof.witness),
            "witness_pcs_params":s.params,"verified":true,"transcript_match":true,"pcs_config_match":true,
            "peak_rss_kib":peak_rss_kib,"piop_blake3":times["piop_blake3"],"terminal_claims_blake3":times["terminal_claims_blake3"],
            "pcs_path":"FLOCK native witness commitment + merged_tensor/open_batch_merged + Ligerito Fast",
            "memory_policy":"sequential public CSC folds; one combined linear vector; retain witness PCS data; fixed CSC offline",
            "rss_scope":"process VmHWM after proving, before verification; includes public matrix setup and prior repetitions",
            "circuit":"one connected alternating AND/XOR circuit; one public final output"});
        eprintln!(
            "[public R1CS] mikros {kind} {}/{} finished: prover={:.3} ms verifier={verify_ms:.3} ms proof={} bytes",
            repetition + 1,
            count,
            times["prover_ms"].as_f64().unwrap(),
            encoded.len()
        );
        println!("{row}");
        io::stdout().flush().map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub fn run_with_args(args: Vec<String>) -> Result<(), String> {
    if env::var_os("FLOCK_NO_GRIND").is_some() {
        return Err("FLOCK_NO_GRIND is not supported".into());
    }
    let argument = |key: &str, default: &str| -> Result<String, String> {
        match args.iter().position(|v| v == key) {
            Some(index) => args
                .get(index + 1)
                .cloned()
                .ok_or_else(|| format!("missing value for {key}")),
            None => Ok(default.into()),
        }
    };
    let number = |key, default| -> Result<usize, String> {
        argument(key, default)?
            .parse()
            .map_err(|_| format!("invalid {key}"))
    };
    if argument("--mode", "single")? != "single" {
        return Err("public matrix comparison is single-instance".into());
    }
    let system = argument("--system", "mikros")?;
    if system != "mikros" && system != "flock" {
        return Err("unknown system".into());
    }
    let log_gates = number("--log-gates", "30")?;
    let threads = number("--threads", "64")?;
    let repetitions = number("--repetitions", "1")?;
    let warmups = number("--warmups", "0")?;
    let seed = argument("--seed", "97")?
        .parse()
        .map_err(|_| "invalid seed")?;
    if !(14..=30).contains(&log_gates) || threads == 0 || repetitions == 0 {
        return Err("require log-gates in 14..=30 and positive threads/repetitions".into());
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .stack_size(8 << 20)
        .build_global()
        .map_err(|e| e.to_string())?;
    if !flock_parallel::init_all_core_pool(threads)
        || flock_parallel::all_core_pool().current_num_threads() != threads
    {
        return Err("all-core pool configuration".into());
    }
    let c = Circuit::new(log_gates, Mode::Single, seed);
    if system == "flock" {
        flock_r1cs::run(&c, repetitions, warmups)
    } else {
        run_mikros(&c, repetitions, warmups)
    }
}

fn main() -> Result<(), String> {
    run_with_args(env::args().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::{lincheck::CscCircuit, r1cs::SparseBinaryMatrix};

    #[test]
    fn arbitrary_weights_bit_order_and_zero_sum_pairs_match_direct_oracle() {
        let n = 512;
        let a = SparseBinaryMatrix::new(n, n, (0..n).map(|i| vec![i, (3 * i + 1) % n]).collect());
        let b = SparseBinaryMatrix::new(n, n, (0..n).map(|i| vec![(7 * i + 2) % n]).collect());
        let matrix = CscCircuit::from_matrices(&a, &b).with_const_pin(Some(0));
        let mut z = vec![F128::new(0x1234512345123451, 0xaabbccdd11223344); n / 128];
        z[0].lo |= 1;
        let query = |left: bool| {
            let columns = (0..128)
                .map(|j| F128::new(j * 17 + 1, j * 31 + 9))
                .collect::<Vec<_>>();
            let factors = if left {
                vec![[F128::new(7, 4); 2], [F128::ONE, F128::new(9, 3)]]
            } else {
                vec![[F128::new(2, 8), F128::new(5, 6)], [F128::ZERO, F128::ONE]]
            };
            let weights = row_weights(&factors.iter().rev().copied().collect::<Vec<_>>());
            let rows = if left { &a.rows } else { &b.rows };
            let value = rows.iter().enumerate().fold(F128::ZERO, |v, (i, cols)| {
                if cols.iter().fold(false, |v, &j| v ^ fixture::bit(&z, j)) {
                    v + weights[i / 128] * columns[i % 128]
                } else {
                    v
                }
            });
            MatrixClaim {
                column_weights: columns,
                row_factors: factors,
                value,
            }
        };
        let qa = query(true);
        let qb = query(false);
        let (alpha, beta) = (F128::new(3, 9), F128::new(7, 2));
        let target = alpha * qa.value + qb.value + beta;
        let g = linear_weights(&matrix, &qa, &qb, alpha, beta).unwrap();
        let expanded = expand_binary(&z, n, &g, target).unwrap();
        let point: Vec<_> = (0..9).map(|i| F128::new(i + 11, i + 37)).collect();
        let claim = bit_mle_claim(&point, F128::ZERO, 15).unwrap();
        let mut padded = z.clone();
        padded.resize(1 << 15, F128::ZERO);
        assert_eq!(
            queries::evaluate(&padded, &claim),
            public_mle(&expanded, &point).unwrap()
        );
        let mut pc = FsChallenger::new(b"public-r1cs-native-ip-oracle");
        let (proof, output_point) =
            inner_product::prove_native_owned(expanded.clone(), g.clone(), &mut pc);
        let mut reference = FsChallenger::new(b"public-r1cs-native-ip-oracle");
        assert_eq!(
            (proof.clone(), output_point.clone()),
            inner_product::prove(expanded, g.clone(), &mut reference)
        );
        assert_eq!(pc.sample_f128(), reference.sample_f128());
        assert_eq!(proof.final_b, public_mle(&g, &output_point).unwrap());
        assert_eq!(
            queries::evaluate(&z, &bit_mle_claim(&output_point, proof.final_a, 2).unwrap()),
            proof.final_a
        );
        let zeros = vec![F128::ZERO; n / 128];
        assert!(expand_binary(&zeros, n, &g, beta).is_err());
        assert!(
            linear_weights(
                &matrix,
                &MatrixClaim {
                    column_weights: vec![],
                    ..qa
                },
                &qb,
                alpha,
                beta
            )
            .is_err()
        );
    }

    #[test]
    fn complete_native_pcs_roundtrip_and_tampering() {
        let c = Circuit::new(14, Mode::Single, 97);
        let setup = flock_r1cs::Setup::new(&c).unwrap();
        let (proof, _, mut pc) = prove(&setup, setup.witness.clone()).unwrap();
        let encoded = bincode::serialize(&proof).unwrap();
        let proof: Proof = bincode::deserialize(&encoded).unwrap();
        let (mut vc, _) = verify(&c, &setup.matrix, &proof).unwrap();
        assert_eq!(pc.sample_f128(), vc.sample_f128());
        for mutate in 0..9 {
            let mut bad = proof.clone();
            match mutate {
                0 => bad.hadamard.inner_product.final_a += F128::ONE,
                1 => bad.hadamard.inner_product.final_b += F128::ONE,
                2 => bad.hadamard.projected_c += F128::ONE,
                3 => bad.linear_ip.final_a += F128::ONE,
                4 => bad.linear_ip.final_b += F128::ONE,
                5 => {
                    bad.linear_ip.rounds.pop();
                }
                6 => {
                    bad.nonces.pop();
                }
                7 => bad.witness_pcs.merged = None,
                _ => bad.witness_pcs.component_values[0] += F128::ONE,
            }
            assert!(
                verify(&c, &setup.matrix, &bad).is_err(),
                "mutation {mutate}"
            );
        }
        let mut wrong_output = c.clone();
        wrong_output.io += F128::ONE;
        assert!(verify(&wrong_output, &setup.matrix, &proof).is_err());
        struct WrongMatrix<'a>(&'a dyn LincheckCircuit);
        impl LincheckCircuit for WrongMatrix<'_> {
            fn n_cols(&self) -> usize {
                self.0.n_cols()
            }
            fn const_pin_col(&self) -> Option<usize> {
                Some(0)
            }
            fn fold_alpha_batched(&self, alpha: F128, weights: &[F128]) -> Vec<F128> {
                let mut values = self.0.fold_alpha_batched(alpha, weights);
                values[0] += alpha * weights[0];
                values
            }
            fn fold_side(&self, weights: &[F128], left: bool) -> Vec<F128> {
                let mut values = self.0.fold_side(weights, left);
                if left {
                    values[0] += weights[0];
                }
                values
            }
        }
        assert!(verify(&c, &WrongMatrix(&setup.matrix), &proof).is_err());
        // Exercise actual invalid assignments, rather than only corrupting a
        // proof transcript: AND output and XOR output.
        for case in 0..2 {
            let mut invalid = setup.witness.clone();
            let index = c.n() + 1 + case;
            let bit = index % 128;
            if bit < 64 {
                invalid[index / 128].lo ^= 1u64 << bit;
            } else {
                invalid[index / 128].hi ^= 1u64 << (bit - 64);
            }
            if let Ok((invalid_proof, _, _)) = prove(&setup, invalid) {
                assert!(
                    verify(&c, &setup.matrix, &invalid_proof).is_err(),
                    "invalid assignment {case}"
                );
            }
        }
        // All-zero witness satisfies the homogeneous gate relation. Set its
        // public output to zero too, so only the beta constant-one pin rejects
        // the production proof; every native PCS opening must still succeed.
        let mut zero_c = c.clone();
        zero_c.io = F128::ZERO;
        let zero_setup = flock_r1cs::Setup::new(&zero_c).unwrap();
        let zero_witness = vec![F128::ZERO; zero_setup.witness.len()];
        let (zero_proof, _, _) = prove(&zero_setup, zero_witness).unwrap();
        match verify(&zero_c, &zero_setup.matrix, &zero_proof) {
            Err(error) => assert!(error.starts_with("linear IP replay:"), "{error}"),
            Ok(_) => panic!("zero constant-one witness was accepted"),
        }
    }
}
