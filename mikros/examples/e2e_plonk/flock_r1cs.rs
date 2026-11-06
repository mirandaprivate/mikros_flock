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
    if c.near_full {
        if row <= fixture::near_full_circuit::INPUTS {
            return ([if left { row } else { 0 }, 0], 1);
        }
        let gate = row - fixture::near_full_circuit::INPUTS - 1;
        let [a, b] = fixture::near_full_circuit::gate_operands(c.log_gates, gate);
        return if gate & 1 == 0 {
            ([if left { a } else { b }, 0], 1)
        } else if left {
            ([a, b], 2)
        } else {
            ([0, 0], 1)
        };
    }
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

fn csc_entry_counts(log_gates: usize) -> Result<(usize, usize, usize), String> {
    let shift = u32::try_from(log_gates).map_err(|_| "circuit exponent overflow")?;
    let n = 1usize.checked_shl(shift).ok_or("circuit size overflow")?;
    let rows = n.checked_mul(4).ok_or("matrix dimension overflow")?;
    if rows == 0 || u32::try_from(rows - 1).is_err() {
        return Err("matrix row indices exceed u32".into());
    }
    let a = n
        .checked_mul(2)
        .and_then(|v| v.checked_add(n / 2 + 1))
        .ok_or("A nonzero count overflow")?;
    let b = n
        .checked_mul(2)
        .and_then(|v| v.checked_add(1))
        .ok_or("B nonzero count overflow")?;
    u32::try_from(a).map_err(|_| "A nonzero count exceeds u32")?;
    u32::try_from(b).map_err(|_| "B nonzero count exceeds u32")?;
    Ok((rows, a, b))
}

fn public_csc(c: &Circuit) -> Result<CscCircuit, String> {
    if c.near_full {
        return near_full_public_csc(c);
    }
    let n = c.n();
    let half = n / 2;
    let (rows, a_count, b_count) = csc_entry_counts(c.log_gates)?;
    // Column lists are exactly the transpose of row_columns, in increasing
    // row order. Closed-form prefixes avoid both CSR allocations and atomic
    // scatter. In particular, the long B constant column is filled in parallel.
    let a_col_ptr = (0..=rows)
        .into_par_iter()
        .map(|column| {
            let identities = column.min(n + 1);
            let input_uses = 2 * column.saturating_sub(1).min(half);
            let output_uses = column.saturating_sub(n + 1).min(n).div_ceil(2);
            (identities + input_uses + output_uses) as u32
        })
        .collect();
    let b_col_ptr = (0..=rows)
        .into_par_iter()
        .map(|column| {
            if column == 0 {
                0
            } else {
                (n + 1 + half + column.saturating_sub(half + 1).min(half)) as u32
            }
        })
        .collect();
    let a_rows = (0..a_count)
        .into_par_iter()
        .map(|entry| {
            let row = if entry == 0 {
                0
            } else if entry <= 3 * half {
                let j = (entry - 1) / 3;
                match (entry - 1) % 3 {
                    0 => 1 + j,
                    1 if j == 0 => n + 1,
                    2 if j == 0 => 2 * n,
                    1 => n + 2 * j,
                    _ => n + 1 + 2 * j,
                }
            } else if entry <= 4 * half {
                1 + half + entry - 1 - 3 * half
            } else {
                n + 2 + 2 * (entry - 1 - 4 * half)
            };
            row as u32
        })
        .collect();
    let b_rows = (0..b_count)
        .into_par_iter()
        .map(|entry| {
            let row = if entry <= n {
                entry
            } else if entry <= n + half {
                n + 2 + 2 * (entry - n - 1)
            } else {
                n + 1 + 2 * (entry - n - half - 1)
            };
            row as u32
        })
        .collect();
    CscCircuit::from_csc_parts(rows, a_col_ptr, a_rows, b_col_ptr, b_rows)
        .map(|matrix| matrix.with_const_pin(Some(0)))
        .map_err(str::to_string)
}

/// Transpose the chain recipe directly into sorted column lists. Only the 256
/// input columns require visiting a block permutation; every output is used
/// once by the following gate. This needs no CSR, sorting, or atomic scatter.
fn near_full_public_csc(c: &Circuit) -> Result<CscCircuit, String> {
    use fixture::near_full_circuit::{INPUTS, gate_operands, right_gate_in_block};
    let rows = c.r1cs_rows();
    let gates = c.genuine_gate_count();
    let a_count = rows.checked_add(gates / 2).ok_or("A count overflow")?;
    u32::try_from(a_count).map_err(|_| "A count exceeds u32")?;
    u32::try_from(rows).map_err(|_| "B count exceeds u32")?;
    let blocks = gates.div_ceil(INPUTS);
    let first_left = gate_operands(c.log_gates, 0)[0];
    let mut a_col_ptr = Vec::with_capacity(rows + 1);
    let mut b_col_ptr = Vec::with_capacity(rows + 1);
    let mut a_rows = Vec::with_capacity(a_count);
    let mut b_rows = Vec::with_capacity(rows);
    a_col_ptr.push(0);
    b_col_ptr.push(0);

    // Column zero: the pinned constant's A identity, and B's identity/XOR rows.
    a_rows.push(0);
    b_rows.extend((0..=INPUTS).map(|row| row as u32));
    b_rows.extend((1..gates).step_by(2).map(|gate| (INPUTS + 1 + gate) as u32));
    a_col_ptr.push(a_rows.len() as u32);
    b_col_ptr.push(b_rows.len() as u32);

    for input in 1..=INPUTS {
        a_rows.push(input as u32);
        if input == first_left {
            a_rows.push((INPUTS + 1) as u32);
        }
        for block in 0..blocks {
            let gate = block * INPUTS + right_gate_in_block(input, block);
            if gate >= gates {
                continue;
            }
            let row = (INPUTS + 1 + gate) as u32;
            if gate & 1 == 0 {
                b_rows.push(row);
            } else {
                a_rows.push(row);
            }
        }
        a_col_ptr.push(a_rows.len() as u32);
        b_col_ptr.push(b_rows.len() as u32);
    }
    // Outputs 0..G-2 each feed the next gate's left operand. The public final
    // output has no downstream use; its value is authenticated separately.
    for output in 0..gates {
        if output + 1 < gates {
            a_rows.push((INPUTS + 2 + output) as u32);
        }
        a_col_ptr.push(a_rows.len() as u32);
        b_col_ptr.push(b_rows.len() as u32);
    }
    debug_assert_eq!(a_rows.len(), a_count);
    debug_assert_eq!(b_rows.len(), rows);
    CscCircuit::from_csc_parts(rows, a_col_ptr, a_rows, b_col_ptr, b_rows)
        .map(|matrix| matrix.with_const_pin(Some(0)))
        .map_err(str::to_string)
}

fn witness_words(c: &Circuit) -> usize {
    (c.r1cs_rows() / 128).max(1 << 15)
}

fn matrix_log(c: &Circuit) -> usize {
    c.r1cs_rows().ilog2() as usize
}

fn useful_bits(c: &Circuit) -> usize {
    if c.near_full {
        c.r1cs_rows()
    } else {
        2 * c.n() + 1
    }
}

fn output_index(c: &Circuit) -> usize {
    if c.near_full {
        c.r1cs_rows() - 1
    } else {
        2 * c.n()
    }
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
    if c.near_full {
        let mut words = fixture::near_full_circuit::compact_witness(c.log_gates, c.seed);
        words.resize(witness_words(c), F128::ZERO);
        return words;
    }
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

pub(super) fn matrix_images(c: &Circuit, z: &[F128]) -> [Vec<F128>; 2] {
    [true, false].map(|left| {
        packed(c.r1cs_rows(), |row| {
            let (columns, len) = row_columns(c, row, left);
            columns[..len]
                .iter()
                .fold(false, |value, &column| value ^ fixture::bit(z, column))
        })
    })
}

pub(super) struct Setup<'a> {
    pub(super) circuit: &'a Circuit,
    pub(super) matrix: CscCircuit,
    pub(super) witness: Vec<F128>,
    pub(super) registry: Registry,
    pub(super) params: PcsParams,
    pub(super) offline_ms: f64,
    pub(super) witness_digest: String,
}
impl<'a> Setup<'a> {
    pub(super) fn new(c: &'a Circuit) -> Result<Self, String> {
        if c.mode != Mode::Single || c.log_gates > 30 {
            return Err(
                "single-instance public CSC requires single mode and at most 2^30 gates".into(),
            );
        }
        event("offline_native_r1cs_setup");
        let start = Instant::now();
        let log_words = witness_words(c).ilog2() as usize;
        let registry = fixture::registry(log_words, c.wire_words().ilog2() as usize);
        let params = fixture::parameters(&fixture::union(&registry));
        params
            .ligerito_prover_config()
            .map_err(|e| format!("R1CS PCS parameters: {e:?}"))?;
        let matrix = public_csc(c)?;
        let witness = witness(c);
        let witness_digest = blake3::hash(fixture::bytes(&witness)).to_hex().to_string();
        Ok(Self {
            circuit: c,
            matrix,
            witness,
            registry,
            params,
            offline_ms: ms(start),
            witness_digest,
        })
    }
}

/// Identifies the public matrix construction, rather than a private witness
/// or a materialized matrix byte stream. Both comparison arms use this recipe.
pub(super) fn matrix_digest(c: &Circuit) -> String {
    let mut hash = blake3::Hasher::new();
    hash.update(if c.near_full {
        b"single-near-full-and-xor-native-r1cs-affine-inputs256-v1-C-identity-constant-col0"
            .as_slice()
    } else {
        b"single-and-xor-native-r1cs-row-columns-v1-C-identity-constant-col0".as_slice()
    });
    hash.update(&(c.log_gates as u64).to_le_bytes());
    hash.update(&(c.r1cs_rows() as u64).to_le_bytes());
    hash.finalize().to_hex().to_string()
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
    ch.observe_bytes(&(matrix_log(c) as u64).to_le_bytes());
    ch.observe_label(b"one-full-matrix-C-identity-constant-zero-column-skip7");
    ch.observe_bytes(&bincode::serialize(witness).unwrap());
    ch
}

fn input_point(c: &Circuit, zc: &ZerocheckClaim) -> Result<QuirkyPoint, String> {
    if zc.mlv_challenges.len() != matrix_log(c) - 7 {
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
    let index = output_index(c);
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
    let m = matrix_log(c);
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
        useful_bits(c),
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
        "public_claims":0,"witness_commitments":1,"public_commitments":0,
        "piop_blake3":piop_blake3,"terminal_claims_blake3":terminal_claims_blake3,
        "prover_ms":ms(begin)});
    Ok((proof, times, ch))
}

/// Verifies against the fixed public CSC and statement only; no witness or
/// generated A/B vectors are available to this routine. Full public-matrix
/// folding in native lincheck is included in verifier time.
fn verify(c: &Circuit, matrix: &CscCircuit, p: &Proof) -> Result<(FsChallenger, Value), String> {
    let log_words = witness_words(c).ilog2() as usize;
    let registry = fixture::registry(log_words, c.wire_words().ilog2() as usize);
    let union = fixture::union(&registry);
    if p.witness.params != fixture::parameters(&union) || !p.lincheck.matrix_evals.is_empty() {
        return Err("native single R1CS PCS shape or deferred matrix claims".into());
    }
    let m = matrix_log(c);
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
        "compression_kappa":128,"witness_digest":s.witness_digest,"matrix_digest":matrix_digest(c),
        "matrix_digest_scope":"public row_columns recipe and dimensions; not serialized CSC bytes",
        "witness_claims":3,"public_claims":0,"witness_commitments":1,"public_commitments":0,
        "fixture_digest":c.digest().iter().map(|b|format!("{b:02x}")).collect::<String>(),
        "arithmetization":"native R1CS","matrix_authentication":"public CSC","permutation_proof":false,
        "public_io":[c.io.lo,c.io.hi],"genuine_gate_count":c.genuine_gate_count(),"r1cs_rows":c.r1cs_rows(),
        "circuit_layout":if c.near_full { "near-full" } else { "paired" },
        "private_input_bits":if c.near_full { fixture::near_full_circuit::INPUTS } else { c.n() },
        "plonk_domain_size":c.n(),"copy_domain_size":4*c.n(),
        "r1cs_outer_blocks":1,"r1cs_matrix_log":matrix_log(c),"C":"identity",
        "r1cs_auxiliary_rows":if c.near_full { "one pinned constant row; 256 private input identity rows; no padding" } else { "one pinned constant row, N input identity rows, 2N-1 zero-padding rows" },
        "witness_layout":if c.near_full { "constant; 256 private input bits; N-257 gate outputs" } else { "constant; N private input bits; N gate outputs; zero padding" },
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
            "[single constraints] flock {kind} {}/{} start ({} genuine gates, {} public CSC rows)",
            repetition + 1,
            count,
            c.genuine_gate_count(),
            c.r1cs_rows()
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
            "gates_per_instance":c.genuine_gate_count(),"instances":1,"total_gates":c.genuine_gate_count(),"threads":rayon::current_num_threads(),
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
    use flock_core::r1cs::SparseBinaryMatrix;

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn native_csc_n30_capacity_without_allocating_matrices() {
        let (rows, a, b) = csc_entry_counts(30).unwrap();
        assert_eq!(rows, 1usize << 32);
        assert_eq!(a, (5usize << 29) + 1);
        assert_eq!(b, (1usize << 31) + 1);
        assert_eq!(
            (2 * (rows + 1) + a + b) * size_of::<u32>(),
            (50usize << 30) + 16
        );
        assert!(csc_entry_counts(31).is_err());
        assert!(csc_entry_counts(usize::MAX).is_err());
    }

    #[test]
    fn near_full_csc_matches_independent_row_transpose_and_statement() {
        for log in [14, 16] {
            let c = Circuit::new_near_full(log, Mode::Single, 97);
            let rows = c.r1cs_rows();
            assert_eq!(rows, 1usize << log);
            assert_eq!(c.genuine_gate_count(), rows - 257);
            let z = witness(&c);
            let [a, b] = matrix_images(&c, &z);
            for row in 0..rows {
                assert_eq!(
                    fixture::bit(&a, row) & fixture::bit(&b, row),
                    fixture::bit(&z, row),
                    "log={log} row={row}"
                );
            }
            assert!(fixture::bit(&z, 0));
            assert_eq!(fixture::bit(&z, rows - 1), c.io == F128::ONE);
            let row_matrix = |left| {
                SparseBinaryMatrix::new(
                    rows,
                    rows,
                    (0..rows)
                        .map(|row| {
                            let (columns, len) = row_columns(&c, row, left);
                            columns[..len].to_vec()
                        })
                        .collect(),
                )
            };
            let reference = CscCircuit::from_matrices(&row_matrix(true), &row_matrix(false))
                .with_const_pin(Some(0));
            let matrix = public_csc(&c).unwrap();
            let weights: Vec<_> = (0..rows)
                .map(|i| {
                    F128::new(
                        (i as u64 + 31).wrapping_mul(0x9e3779b97f4a7c15),
                        (i as u64).rotate_left(29),
                    )
                })
                .collect();
            assert_eq!(matrix.const_pin_col(), Some(0));
            assert_eq!(matrix.n_cols(), rows);
            for side in [true, false] {
                assert_eq!(
                    matrix.fold_side(&weights, side),
                    reference.fold_side(&weights, side)
                );
            }
            assert_eq!(
                matrix.fold_alpha_batched(F128::new(73, 19), &weights),
                reference.fold_alpha_batched(F128::new(73, 19), &weights)
            );
        }
    }

    #[test]
    fn near_full_native_roundtrip_rejects_output_matrix_and_pcs_tampering() {
        let c = Circuit::new_near_full(14, Mode::Single, 97);
        let setup = Setup::new(&c).unwrap();
        let (proof, _, mut prover) = prove(&setup, setup.witness.clone()).unwrap();
        let encoded = bincode::serialize(&proof).unwrap();
        let proof: Proof = bincode::deserialize(&encoded).unwrap();
        let (mut verifier, _) = verify(&c, &setup.matrix, &proof).unwrap();
        assert_eq!(prover.sample_f128(), verifier.sample_f128());
        let mut wrong_output = c.clone();
        wrong_output.io += F128::ONE;
        assert!(verify(&wrong_output, &setup.matrix, &proof).is_err());
        let mut bad = proof.clone();
        bad.lincheck.z_partial[0] += F128::ONE;
        assert!(verify(&c, &setup.matrix, &bad).is_err());
        let mut bad = proof.clone();
        bad.nonces.pop();
        assert!(verify(&c, &setup.matrix, &bad).is_err());
        let mut bad = proof.clone();
        bad.witness_pcs.merged = None;
        assert!(verify(&c, &setup.matrix, &bad).is_err());
        let rows = c.r1cs_rows();
        let row_matrix = |left| {
            SparseBinaryMatrix::new(
                rows,
                rows,
                (0..rows)
                    .map(|row| {
                        let (columns, len) = row_columns(&c, row, left);
                        if left && row == 0 {
                            vec![]
                        } else {
                            columns[..len].to_vec()
                        }
                    })
                    .collect(),
            )
        };
        let wrong = CscCircuit::from_matrices(&row_matrix(true), &row_matrix(false))
            .with_const_pin(Some(0));
        assert!(verify(&c, &wrong, &proof).is_err());
        let legacy = Circuit::new(14, Mode::Single, 97);
        assert_ne!(matrix_digest(&c), matrix_digest(&legacy));
        assert_ne!(c.digest(), legacy.digest());
    }

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
        let matrix = public_csc(&c).unwrap();
        let row_matrix = |left| {
            SparseBinaryMatrix::new(
                4 * c.n(),
                4 * c.n(),
                (0..4 * c.n())
                    .map(|row| {
                        let (columns, len) = row_columns(&c, row, left);
                        columns[..len].to_vec()
                    })
                    .collect(),
            )
        };
        let reference = CscCircuit::from_matrices(&row_matrix(true), &row_matrix(false))
            .with_const_pin(Some(0));
        assert_eq!(matrix.const_pin_col(), reference.const_pin_col());
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
            assert_eq!(actual, reference.fold_side(&weights, left));
        }
        let alpha = F128::new(73, 19);
        assert_eq!(
            matrix.fold_alpha_batched(alpha, &weights),
            reference.fold_alpha_batched(alpha, &weights)
        );
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
