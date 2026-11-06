use super::{
    fixture::{self, Circuit},
    grinding::GrindingChallenger,
    hadamard, pcs,
};
use bincode::Options;
use flock_core::{
    challenger::{Challenger, FsChallenger},
    lincheck::LincheckCircuit,
    pcs::functional::{TensorClaim, row_weights},
};
use flock_field::{F128, F256Unreduced};
use mikros::{MatrixClaim, multiple_field_image as image, packed_opening};
use mikros_permutation::field::SmallField;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    io::{self, Write},
    time::Instant,
};

type Result<T> = std::result::Result<T, String>;
fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}
fn event(stage: &str) {
    eprintln!("stage={stage}");
}
fn pairs(point: &[F128]) -> Vec<[F128; 2]> {
    point.iter().map(|&x| [F128::ONE + x, x]).collect()
}
fn fixed(index: usize, log: usize) -> Vec<[F128; 2]> {
    (0..log)
        .map(|bit| {
            if index >> bit & 1 == 0 {
                [F128::ONE, F128::ZERO]
            } else {
                [F128::ZERO, F128::ONE]
            }
        })
        .collect()
}
fn dot(a: &[F128], b: &[F128]) -> F128 {
    a.iter()
        .zip(b)
        .fold(F256Unreduced::ZERO, |sum, (&a, &b)| {
            sum ^ a.mul_unreduced(b)
        })
        .reduce()
}

/// The same paired MUL/ADD wiring as the single-instance fixture, represented
/// as public 0/1 R1CS matrices. C=I; one constant plus N inputs and N outputs
/// require 4N padded rows. Padding is constrained to zero, never omitted.
#[derive(Clone)]
struct Matrix {
    gates: usize,
}
impl Matrix {
    fn rows(&self) -> usize {
        4 * self.gates
    }
    fn indices(&self, row: usize, left: bool) -> [Option<usize>; 2] {
        if row <= self.gates {
            return [Some(if left { row } else { 0 }), None];
        }
        if row <= 2 * self.gates {
            let gate = row - self.gates - 1;
            if gate & 1 == 0 {
                return [Some(1 + gate + usize::from(!left)), None];
            }
            return if left {
                [Some(row - 1), Some(1 + ((gate + 1) % self.gates))]
            } else {
                [Some(0), None]
            };
        }
        [None, None]
    }
    fn images(&self, z: &[F128]) -> [Vec<F128>; 2] {
        std::array::from_fn(|side| {
            z.par_chunks_exact(self.rows())
                .flat_map_iter(|block| {
                    (0..self.rows()).map(move |row| {
                        self.indices(row, side == 0)
                            .into_iter()
                            .flatten()
                            .fold(F128::ZERO, |sum, col| sum + block[col])
                    })
                })
                .collect()
        })
    }
}
impl LincheckCircuit for Matrix {
    fn n_cols(&self) -> usize {
        self.rows()
    }
    fn const_pin_col(&self) -> Option<usize> {
        Some(0)
    }
    fn fold_alpha_batched(&self, alpha: F128, xi: &[F128]) -> Vec<F128> {
        let (a, b) = self.fold_split(xi);
        a.into_iter().zip(b).map(|(a, b)| alpha * a + b).collect()
    }
    fn fold_split(&self, xi: &[F128]) -> (Vec<F128>, Vec<F128>) {
        (self.fold_side(xi, true), self.fold_side(xi, false))
    }
    fn fold_side(&self, xi: &[F128], left: bool) -> Vec<F128> {
        assert_eq!(xi.len(), self.rows());
        let mut out = vec![F128::ZERO; self.rows()];
        for (row, &weight) in xi.iter().enumerate() {
            for col in self.indices(row, left).into_iter().flatten() {
                out[col] += weight;
            }
        }
        out
    }
}

struct Setup {
    matrix: Matrix,
    bits: usize,
    instances: usize,
    kappa: usize,
    groups: usize,
    witness: Vec<F128>,
    outputs: Vec<F128>,
    packing: image::FieldPacking,
    source: pcs::Source,
    seed: u64,
    offline_ms: f64,
}
impl Setup {
    fn new(log: usize, bits: usize, instances: usize, seed: u64) -> Result<Self> {
        let start = Instant::now();
        let first = Circuit::new(log, bits, seed)?;
        let kappa = 128 / bits;
        if !instances.is_power_of_two() || instances < kappa || instances > 256 {
            return Err("batch instance shape".into());
        }
        let groups = instances / kappa;
        let matrix = Matrix { gates: 1 << log };
        event("offline_field_and_interpolation_tables");
        hadamard::warmup(bits)?;
        let packing = image::FieldPacking::new(
            bits,
            SmallField::for_width(bits)
                .map_err(|e| e.to_string())?
                .basis(),
        )?;
        let circuits: Vec<_> = (0..instances)
            .map(|j| {
                Circuit::new(
                    log,
                    bits,
                    seed.wrapping_add((j as u64).wrapping_mul(0x9e3779b97f4a7c15)),
                )
            })
            .collect::<Result<_>>()?;
        assert_eq!(circuits[0].digest(), first.digest());
        event("offline_canonical_witness_generation");
        let witness: Vec<_> = (0..groups * matrix.rows())
            .into_par_iter()
            .map(|index| {
                let group = index / matrix.rows();
                let row = index % matrix.rows();
                let mut word = 0u128;
                for lane in 0..kappa {
                    let c = &circuits[group * kappa + lane];
                    let value = if row == 0 {
                        1
                    } else if row <= matrix.gates {
                        let input = row - 1;
                        c.value(input & 1, input & !1)
                    } else if row <= 2 * matrix.gates {
                        c.value(2, row - matrix.gates - 1)
                    } else {
                        0
                    };
                    word |= value << (lane * bits);
                }
                fixture::from_raw(word)
            })
            .collect();
        let outputs = (0..groups)
            .map(|group| {
                fixture::from_raw((0..kappa).fold(0u128, |word, lane| {
                    word | (circuits[group * kappa + lane].io << (lane * bits))
                }))
            })
            .collect();
        event("offline_additive_fri_setup");
        let source =
            pcs::Source::new(witness.len(), 0x4249545749445448 ^ log as u64 ^ bits as u64)?;
        Ok(Self {
            matrix,
            bits,
            instances,
            kappa,
            groups,
            witness,
            outputs,
            packing,
            source,
            seed,
            offline_ms: ms(start),
        })
    }
    fn bind(&self, commitment: &pcs::Commitment) -> FsChallenger {
        let mut ch = FsChallenger::new(b"mikros-shared-paired-r1cs-field-fri-v1");
        for value in [
            self.matrix.gates,
            self.matrix.rows(),
            self.bits,
            self.instances,
            self.kappa,
        ] {
            ch.observe_bytes(&(value as u64).to_le_bytes());
        }
        ch.observe_label(
            b"paired-mul-add-public-binary-matrices-constant-inputs-outputs-padding-v1",
        );
        ch.observe_f128_slice(&self.outputs);
        ch.observe_bytes(&self.source.verifier_key().parameter_digest());
        ch.observe_bytes(&bincode::serialize(commitment).unwrap());
        ch
    }
    fn identity_claim(&self, group: usize, source: &hadamard::Claim) -> TensorClaim {
        let basis = SmallField::for_width(self.bits).unwrap().basis();
        let columns = source
            .columns
            .iter()
            .flat_map(|&weight| basis.iter().map(move |&bit| weight * bit))
            .collect();
        let mut rows = source.rows.clone();
        rows.extend(fixed(group, self.groups.ilog2() as usize));
        TensorClaim {
            columns,
            rows,
            value: source.value,
        }
    }
    fn public_claims(&self, ch: &mut impl Challenger) -> Vec<TensorClaim> {
        ch.observe_label(b"mikros-field-batch-public-constant-and-all-outputs-v1");
        let group_point = ch.sample_f128_vec(self.groups.ilog2() as usize);
        let group_rows = pairs(&group_point);
        let weights = row_weights(&group_rows);
        let one =
            fixture::from_raw((0..self.kappa).fold(0u128, |v, j| v | (1u128 << (j * self.bits))));
        [
            (0, one),
            (2 * self.matrix.gates, dot(&self.outputs, &weights)),
        ]
        .into_iter()
        .map(|(row, value)| {
            let mut rows = fixed(row, self.matrix.rows().ilog2() as usize);
            rows.extend(group_rows.clone());
            TensorClaim {
                columns: fixture::basis(),
                rows,
                value,
            }
        })
        .collect()
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct Proof {
    commitment: pcs::Commitment,
    hadamard: hadamard::batch::Proof,
    images: Vec<image::PublicBatchProof>,
    group_values: Vec<Vec<F128>>,
    nonces: Vec<u64>,
    opening: packed_opening::Proof,
    opening_nonces: Vec<u64>,
    witness_pcs: Vec<u8>,
}

fn group_terminal(
    s: &Setup,
    claim: &image::PublicBatchClaims,
    values: &[F128],
    ch: &mut impl Challenger,
) -> Result<TensorClaim> {
    if values.len() != s.groups
        || claim.weights.len() != s.groups
        || claim.columns.len() != 128
        || claim.witness.point.len() != s.matrix.rows().ilog2() as usize
    {
        return Err("batch witness terminal shape".into());
    }
    ch.observe_label(b"mikros-field-batch-terminal-vector-v1");
    ch.observe_f128_slice(values);
    if dot(values, &claim.weights) != claim.witness.value {
        return Err("batch weighted witness terminal".into());
    }
    let group_point = ch.sample_f128_vec(s.groups.ilog2() as usize);
    let mut rows = pairs(&claim.witness.point);
    let group_rows = pairs(&group_point);
    let value = dot(values, &row_weights(&group_rows));
    rows.extend(group_rows);
    Ok(TensorClaim {
        columns: claim.columns.clone(),
        rows,
        value,
    })
}
fn image_source(claim: &hadamard::Claim) -> MatrixClaim {
    MatrixClaim {
        row_factors: claim.rows.iter().rev().copied().collect(),
        column_weights: claim.columns.clone(),
        value: claim.value,
    }
}

fn prove(s: &Setup) -> Result<(Proof, FsChallenger, Value)> {
    let start = Instant::now();
    event("online_witness_fri_commitment");
    let t = Instant::now();
    let (commitment, cache) = s.source.commit(&s.witness)?;
    let commit_ms = ms(t);
    let mut ch = GrindingChallenger::prover(s.bind(&commitment));
    event("online_public_matrix_images");
    let t = Instant::now();
    let [a, b] = s.matrix.images(&s.witness);
    let matrix_ms = ms(t);
    let n = s.matrix.rows();
    let a_blocks: Vec<_> = a.chunks_exact(n).collect();
    let b_blocks: Vec<_> = b.chunks_exact(n).collect();
    let w_blocks: Vec<_> = s.witness.chunks_exact(n).collect();
    event("online_packed_hadamards");
    let t = Instant::now();
    let (hadamard, claims) =
        hadamard::batch::prove(&a_blocks, &b_blocks, &w_blocks, s.bits, &mut ch)?;
    let gate_ms = ms(t);
    let mut images = Vec::new();
    let mut group_values = Vec::new();
    let mut queries = Vec::new();
    event("online_shared_public_matrix_reductions");
    let t = Instant::now();
    for side in 0..2 {
        let sources: Vec<_> = claims.iter().map(|q| image_source(&q[side])).collect();
        let (proof, claim) = image::prove_public_batch(
            &s.matrix,
            side == 0,
            &s.packing,
            &w_blocks,
            if side == 0 { &a_blocks } else { &b_blocks },
            &sources,
            &mut ch,
        )?;
        // Preserve independent block weights. Fix each block's endpoint before
        // a fresh tensor challenge combines them for one PCS query.
        let weights = row_weights(&pairs(&claim.witness.point));
        let values: Vec<_> = w_blocks
            .iter()
            .map(|words| {
                words
                    .par_iter()
                    .zip(&weights)
                    .map(|(&w, &r)| s.packing.pack_word(w).mul_unreduced(r))
                    .reduce(|| F256Unreduced::ZERO, |a, b| a ^ b)
                    .reduce()
            })
            .collect();
        queries.push(group_terminal(s, &claim, &values, &mut ch)?);
        images.push(proof);
        group_values.push(values);
    }
    let images_ms = ms(t);
    drop(a_blocks);
    drop(b_blocks);
    drop(a);
    drop(b);
    queries.extend(
        claims
            .iter()
            .enumerate()
            .map(|(g, q)| s.identity_claim(g, &q[2])),
    );
    queries.extend(s.public_claims(&mut ch));
    let reduction_grinding_ms = ch.grinding_ms();
    let (ch, nonces) = ch.finish()?;
    event("online_packed_opening_reduction");
    let t = Instant::now();
    let mut ch = GrindingChallenger::prover(ch);
    let (opening, terminal) = packed_opening::prove(&[(&s.witness, &queries)], &mut ch)?;
    let opening_grinding_ms = ch.grinding_ms();
    let (mut ch, opening_nonces) = ch.finish()?;
    let packed_opening_ms = ms(t);
    if terminal.len() != 1 {
        return Err("batch canonical PCS terminal".into());
    }
    event("online_witness_fri_open");
    let t = Instant::now();
    let witness_pcs = s.source.open(&commitment, &terminal[0], cache)?;
    ch.observe_label(pcs::WITNESS_PROOF_LABEL);
    ch.observe_bytes(&witness_pcs);
    let witness_pcs_open_ms = ms(t);
    let times = json!({"prover_ms":ms(start),"commit_ms":commit_ms,"matrix_apply_ms":matrix_ms,
        "gates_ms":gate_ms,"images_ms":images_ms,"packed_opening_ms":packed_opening_ms,
        "witness_pcs_open_ms":witness_pcs_open_ms,"piop_grinding_ms":reduction_grinding_ms,
        "opening_grinding_ms":opening_grinding_ms,"pcs_claims":1,"source_claims":queries.len()});
    Ok((
        Proof {
            commitment,
            hadamard,
            images,
            group_values,
            nonces,
            opening,
            opening_nonces,
            witness_pcs,
        },
        ch,
        times,
    ))
}
fn verify(s: &Setup, p: &Proof) -> Result<FsChallenger> {
    if p.images.len() != 2 || p.group_values.len() != 2 {
        return Err("batch image count".into());
    }
    let mut ch = GrindingChallenger::verifier(s.bind(&p.commitment), &p.nonces);
    let claims = hadamard::batch::verify(s.matrix.rows(), s.bits, s.groups, &p.hadamard, &mut ch)?;
    let mut queries = Vec::new();
    for side in 0..2 {
        let sources: Vec<_> = claims.iter().map(|q| image_source(&q[side])).collect();
        let claim = image::verify_public_batch(
            &s.matrix,
            side == 0,
            &s.packing,
            &sources,
            &p.images[side],
            &mut ch,
        )?;
        queries.push(group_terminal(s, &claim, &p.group_values[side], &mut ch)?);
    }
    queries.extend(
        claims
            .iter()
            .enumerate()
            .map(|(g, q)| s.identity_claim(g, &q[2])),
    );
    queries.extend(s.public_claims(&mut ch));
    let (ch, _) = ch.finish()?;
    let mut ch = GrindingChallenger::verifier(ch, &p.opening_nonces);
    let terminal = packed_opening::verify(
        &[s.source.verifier_key().words().ilog2() as usize],
        &[&queries],
        &p.opening,
        &mut ch,
    )?;
    let (mut ch, _) = ch.finish()?;
    if terminal.len() != 1 {
        return Err("batch verification terminal count".into());
    }
    pcs::verify(
        s.source.verifier_key(),
        &p.commitment,
        &terminal[0],
        &p.witness_pcs,
    )?;
    ch.observe_label(pcs::WITNESS_PROOF_LABEL);
    ch.observe_bytes(&p.witness_pcs);
    Ok(ch)
}
fn decode(bytes: &[u8]) -> Result<Proof> {
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(bytes.len() as u64)
        .reject_trailing_bytes()
        .deserialize(bytes)
        .map_err(|e| e.to_string())
}
fn rss() -> usize {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines().find_map(|l| {
                l.strip_prefix("VmHWM:")
                    .and_then(|v| v.split_whitespace().next()?.parse().ok())
            })
        })
        .unwrap_or(0)
}
fn emit(v: Value) -> Result<()> {
    println!("{v}");
    io::stdout().flush().map_err(|e| e.to_string())
}
pub fn run_with_args(args: Vec<String>) -> Result<()> {
    let (mut log, mut bits, mut threads, mut reps, mut warmups, mut seed) = (20, 1, 1, 1, 0, 97u64);
    let mut args = args.iter();
    while let Some(key) = args.next() {
        let value = args.next().ok_or_else(|| format!("missing {key}"))?;
        match key.as_str() {
            "--log-gates" => log = value.parse().map_err(|_| "log gates")?,
            "--bits" => bits = value.parse().map_err(|_| "bits")?,
            "--threads" => threads = value.parse().map_err(|_| "threads")?,
            "--repetitions" => reps = value.parse().map_err(|_| "repetitions")?,
            "--warmups" => warmups = value.parse().map_err(|_| "warmups")?,
            "--seed" => seed = value.parse().map_err(|_| "seed")?,
            _ => return Err(format!("unknown argument {key}")),
        }
    }
    if threads == 0
        || threads > 256
        || reps == 0
        || reps > 100
        || warmups > 10
        || std::env::var_os("FLOCK_NO_GRIND").is_some()
    {
        return Err("invalid sampling configuration".into());
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .stack_size(8 << 20)
        .build_global()
        .map_err(|e| e.to_string())?;
    if !flock_parallel::init_all_core_pool(threads) {
        return Err("all-core pool already initialized".into());
    }
    let setup = Setup::new(log, bits, 256, seed)?;
    emit(
        json!({"type":"metadata","system":"mikros","mode":"multiple","arithmetization":"Packed R1CS",
        "value_bits":bits,"base_field_bits":bits,"field_semantics":"GF(2^b), field ADD/MUL",
        "kappa":setup.kappa,"instance_packing_width":setup.kappa,"packed_groups":setup.groups,
        "instances":256,"gates_per_instance":1usize<<log,"total_gates":256usize<<log,"genuine_gate_count":1usize<<log,
        "r1cs_rows":setup.matrix.rows(),"log_gates":log,"threads":threads,"seed":seed,"repetitions":reps,"warmups":warmups,
        "pcs_backend":pcs::BACKEND_NAME,"pcs_profile":pcs::profile(),"witness_pcs_parameters":setup.source.verifier_key().parameter_profile(),
        "witness_words":setup.witness.len(),"public_words":0,"pcs_claims":1,"pcs_complete":true,
        "matrix_authentication":"public shared binary matrix walker; C=I; explicit verifier marginal evaluation",
        "permutation_proof":false,"circuit_layout":"paired","packing_endpoint":if bits==128 {"full-width kappa=1"}else{"packed"},
        "public_io_authentication":"all instance outputs and constant-one coordinates via packed opening",
        "timing_scope":"canonical input preparation offline; matrix images, witness commit, reductions, public matrix verification, PCS and serialization online"}),
    )?;
    for run in 0..warmups + reps {
        let warmup = run < warmups;
        let repetition = if warmup { run } else { run - warmups };
        eprintln!(
            "[Packed R1CS FRI] b={bits} {} {}/{} start",
            if warmup { "warmup" } else { "sample" },
            repetition + 1,
            if warmup { warmups } else { reps }
        );
        let (proof, mut pc, mut times) = prove(&setup)?;
        let t = Instant::now();
        let bytes = bincode::serialize(&proof).map_err(|e| e.to_string())?;
        let serialization = ms(t);
        times["serialization_ms"] = json!(serialization);
        times["prover_ms"] = json!(times["prover_ms"].as_f64().unwrap() + serialization);
        let peak = rss();
        event("verify_full_proof");
        let t = Instant::now();
        let mut vc = verify(&setup, &decode(&bytes)?)?;
        let verify_ms = ms(t);
        if pc.sample_f128_vec(4) != vc.sample_f128_vec(4) {
            return Err("batch full transcript mismatch".into());
        }
        eprintln!(
            "[Packed R1CS FRI] b={bits} sample {}/{} finished: prover={:.3} ms verifier={verify_ms:.3} ms proof={} bytes",
            repetition + 1,
            reps,
            times["prover_ms"].as_f64().unwrap(),
            bytes.len()
        );
        emit(
            json!({"type":"sample","value_bits":bits,"instances":256,"warmup":warmup,"repetition":repetition,
            "gates_per_instance":1usize<<log,"total_gates":256usize<<log,"verified":true,"transcript_match":true,"pcs_complete":true,
            "times":times,"verify_ms":verify_ms,"proof_bytes":bytes.len(),"peak_rss_kib":peak,"offline_ms":setup.offline_ms,
            "witness_pcs_proof_bytes":proof.witness_pcs.len(),
            "proof_blake3":blake3::hash(&bytes).to_hex().to_string()}),
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paired_matrix_matches_single_fixture_all_widths() {
        for bits in [1, 2, 4, 8, 16, 32, 64, 128] {
            let s = Setup::new(14, bits, 2 * (128 / bits), 97).unwrap();
            let [a, b] = s.matrix.images(&s.witness);
            let field = mikros_permutation::field::FastField::for_width(bits).unwrap();
            let mask = field.base.mask();
            for index in [
                0,
                1,
                2,
                s.matrix.gates,
                s.matrix.gates + 1,
                s.matrix.gates + 2,
                2 * s.matrix.gates,
                2 * s.matrix.gates + 1,
                s.matrix.rows() - 1,
            ] {
                for group in 0..s.groups {
                    for lane in 0..s.kappa {
                        let i = group * s.matrix.rows() + index;
                        let shift = lane * bits;
                        assert_eq!(
                            field.multiply(
                                (fixture::raw(a[i]) >> shift) & mask,
                                (fixture::raw(b[i]) >> shift) & mask
                            ),
                            (fixture::raw(s.witness[i]) >> shift) & mask,
                            "b={bits}, row={index}"
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn all_widths_complete_batch_and_tampering() {
        for bits in [1, 2, 4, 8, 16, 32, 64, 128] {
            let mut s = Setup::new(14, bits, 2 * (128 / bits), 97).unwrap();
            let (p, mut pc, _) = prove(&s).unwrap();
            let bytes = bincode::serialize(&p).unwrap();
            // Verification must depend only on the public statement, VK and proof.
            s.witness.clear();
            let mut vc = verify(&s, &decode(&bytes).unwrap()).unwrap();
            assert_eq!(pc.sample_f128_vec(4), vc.sample_f128_vec(4));
            let mut bad = p.clone();
            bad.group_values[0][0] += F128::ONE;
            assert!(verify(&s, &bad).is_err());
            let mut bad = p.clone();
            bad.images[0].columns[0][0] += F128::ONE;
            assert!(verify(&s, &bad).is_err());
            let mut bad = p.clone();
            bad.witness_pcs[0] ^= 1;
            assert!(verify(&s, &bad).is_err());
            s.outputs[0] += F128::ONE;
            assert!(verify(&s, &p).is_err());
            flock_core::scratch::clear();
            mikros_permutation::tree_pool::clear_all();
        }
    }

    #[test]
    fn constant_and_padding_are_constrained() {
        for bits in [1, 128] {
            let mut s = Setup::new(14, bits, 2 * (128 / bits), 97).unwrap();
            let padding = 2 * s.matrix.gates + 1;
            s.witness[padding] += F128::ONE;
            assert!(prove(&s).is_err(), "nonzero padding accepted at b={bits}");
            s.witness.fill(F128::ZERO);
            s.outputs.fill(F128::ZERO);
            // The zero assignment satisfies all homogeneous matrix relations;
            // only the authenticated constant-one row rules it out.
            assert!(prove(&s).is_err(), "zero constant accepted at b={bits}");
            flock_core::scratch::clear();
            mikros_permutation::tree_pool::clear_all();
        }
    }
}
