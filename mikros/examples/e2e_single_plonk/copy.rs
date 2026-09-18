//! Single-instance PLONK copy argument over the position/value subfield.
//! All private wire and permuted-position terminals are deferred PCS claims.
//! Fixed position-label interpolation is public circuit preprocessing. Applying
//! current witness/I/O bits and constructing their coefficients remains online.

use super::{fixture::Circuit, gates::Claims, queries::*};
use flock_core::{
    challenger::Challenger,
    pcs::{
        functional::{TensorClaim, row_weights},
        ring_switch::{claim_check, fold_1b_rows_naive, fold_1b_rows_split},
    },
    zerocheck::PaddingSpec,
};
use flock_field::F128;
use mikros_permutation::{
    field::SmallField,
    packed,
    permutation::{self, CommittedStatement},
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SingleProof {
    pub permutation: packed::Proof,
    /// Sigma, then identity; each value includes its wire-block prefix.
    pub wire_values: [[F128; 3]; 2],
    /// Wire-major, then low-to-high position digits: exactly 3*h values.
    pub label_values: Vec<F128>,
}

fn check_circuit(c: &Circuit) -> Result<(), String> {
    if !(7..=30).contains(&c.log_gates) || c.io.len() > c.n() {
        return Err("single copy circuit shape".into());
    }
    let d = (c.h() + 1).next_power_of_two();
    if d > 64 || c.copy_len() < (128 / d) * (128 / d) {
        return Err("single copy subfield shape".into());
    }
    Ok(())
}

fn check_layout(c: &Circuit, log_z: usize, log_public: usize) -> Result<(), String> {
    check_circuit(c)?;
    if log_z != c.witness_words().ilog2() as usize
        || log_public != c.public_words().ilog2() as usize
    {
        return Err("single copy source commitment shape".into());
    }
    Ok(())
}

fn input_layout(c: &Circuit, z: &[F128], public: &[F128]) -> Result<(usize, usize), String> {
    if !z.len().is_power_of_two() || !public.len().is_power_of_two() {
        return Err("single copy source lengths".into());
    }
    let logs = (z.len().ilog2() as usize, public.len().ilog2() as usize);
    check_layout(c, logs.0, logs.1)?;
    Ok(logs)
}

fn prefix(full: &TensorClaim, block: usize) -> F128 {
    let n = full.rows.len();
    full.rows[n - 2][block & 1] * full.rows[n - 1][(block >> 1) & 1]
}

/// Keep even a zero block prefix on the columns; never normalize by it.
fn restrict_binary(full: &TensorClaim, block: usize) -> TensorClaim {
    let scale = prefix(full, block);
    TensorClaim {
        columns: full.columns.iter().map(|&v| scale * v).collect(),
        rows: full.rows[..full.rows.len() - 2].to_vec(),
        value: F128::ZERO,
    }
}

/// Arbitrary K-column weights become binary columns by absorbing the lowest
/// row factors. Both high wire-block bits remain in the tensor row query.
fn packed_binary_query(c: &Circuit, claim: &packed::Claim) -> Result<TensorClaim, String> {
    let d = (c.h() + 1).next_power_of_two();
    let kappa = 128 / d;
    let s = kappa.ilog2() as usize;
    if claim.column_weights.len() != kappa || claim.row_point.len() != c.h() - s {
        return Err("single packed copy terminal shape".into());
    }
    let low = tensor(&claim.row_point[..7 - s]);
    Ok(TensorClaim {
        columns: (0..128)
            .map(|i| claim.column_weights[i & (kappa - 1)] * low[i >> s])
            .collect(),
        rows: pairs(&claim.row_point[7 - s..]),
        value: claim.value,
    })
}

/// Identity labels over three complete wire blocks. Row factors are MLE
/// factors of total mass one; the binary column weights remain arbitrary.
fn identity_wires(c: &Circuit, full: &TensorClaim, basis: &[F128]) -> F128 {
    let column_mass = full.columns.iter().copied().fold(F128::ZERO, |a, b| a + b);
    let low = (0..128).fold(F128::ZERO, |sum, column| {
        let label = (0..7).fold(F128::ZERO, |v, bit| {
            if column >> bit & 1 == 1 {
                v + basis[bit]
            } else {
                v
            }
        });
        sum + full.columns[column] * label
    });
    let middle = (7..c.log_gates).fold(F128::ZERO, |v, j| v + basis[j] * full.rows[j - 7][1]);
    (0..3).fold(F128::ZERO, |sum, block| {
        let high = (0..2).fold(F128::ZERO, |v, j| {
            if block >> j & 1 == 1 {
                v + basis[c.log_gates + j]
            } else {
                v
            }
        });
        sum + prefix(full, block) * (low + column_mass * (middle + high))
    })
}

/// The public interval starts on a 128-bit boundary. Reuse its row weight
/// across each chunk, including the final partial chunk; no padded labels enter.
fn public_contribution(c: &Circuit, full: &TensorClaim, side: usize, field: &SmallField) -> F128 {
    let value_basis = field.basis()[field.bits - 1];
    c.io.chunks(128)
        .enumerate()
        .fold(F128::ZERO, |sum, (word, bits)| {
            let row = 3 * c.wire_words() + word;
            let weight = full
                .rows
                .iter()
                .enumerate()
                .fold(F128::ONE, |v, (j, pair)| v * pair[(row >> j) & 1]);
            let chunk = bits
                .iter()
                .enumerate()
                .fold(F128::ZERO, |v, (column, &bit)| {
                    let index = 3 * c.n() + 128 * word + column;
                    let label = if side == 0 { c.sigma(index) } else { index };
                    let value =
                        field.embed(label as u128) + if bit { value_basis } else { F128::ZERO };
                    v + full.columns[column] * value
                });
            sum + weight * chunk
        })
}

/// Build challenge-dependent row tensors once per terminal, shared by all
/// three wire folds and all 3*h position-digit folds on that side.
struct BinaryEvaluation {
    lo: Vec<F128>,
    hi: Vec<F128>,
    log_rows: usize,
}
impl BinaryEvaluation {
    fn new(full: &TensorClaim) -> Self {
        let rows = &full.rows[..full.rows.len() - 2];
        let split = if rows.len() < 4 {
            rows.len()
        } else {
            rows.len().div_ceil(2).max(4)
        };
        Self {
            lo: row_weights(&rows[..split]),
            hi: row_weights(&rows[split..]),
            log_rows: rows.len(),
        }
    }
    fn evaluate(&self, words: &[F128], columns: &[F128]) -> F128 {
        let folded = if self.log_rows < 4 {
            fold_1b_rows_naive(words, &self.lo)
        } else {
            fold_1b_rows_split(
                words,
                &self.lo,
                &self.hi,
                &PaddingSpec::dense(self.log_rows + 7),
            )
        };
        claim_check(columns, &folded)
    }
}

fn evaluate_terminals(
    c: &Circuit,
    z: &[F128],
    public: &[F128],
    full: &[TensorClaim; 2],
) -> ([[F128; 3]; 2], Vec<F128>) {
    let m = c.wire_words();
    let evaluators = full.each_ref().map(BinaryEvaluation::new);
    let wire_values = std::array::from_fn(|side| {
        std::array::from_fn(|wire| {
            prefix(&full[side], wire)
                * evaluators[side].evaluate(&z[wire * m..(wire + 1) * m], &full[side].columns)
        })
    });
    let label_values = (0..3 * c.h())
        .into_par_iter()
        .map(|i| {
            let block = 5 + i;
            prefix(&full[0], i / c.h())
                * evaluators[0].evaluate(&public[block * m..(block + 1) * m], &full[0].columns)
        })
        .collect();
    (wire_values, label_values)
}

fn single_expand(
    c: &Circuit,
    log_z: usize,
    log_public: usize,
    proof: &SingleProof,
    terminals: &[packed::Claim; 2],
) -> Result<Claims, String> {
    if proof.label_values.len() != 3 * c.h() {
        return Err("single copy label message shape".into());
    }
    let d = (c.h() + 1).next_power_of_two();
    let field = SmallField::for_width(d).map_err(|e| e.to_string())?;
    let basis = field.basis();
    let full = [
        packed_binary_query(c, &terminals[0])?,
        packed_binary_query(c, &terminals[1])?,
    ];
    let mut output = Claims::default();
    for side in 0..2 {
        let wire = proof.wire_values[side]
            .iter()
            .copied()
            .fold(F128::ZERO, |a, b| a + b);
        let labels = if side == 0 {
            proof
                .label_values
                .iter()
                .enumerate()
                .fold(F128::ZERO, |sum, (i, &value)| {
                    sum + value * basis[i % c.h()]
                })
        } else {
            identity_wires(c, &full[side], basis)
        };
        if labels + basis[d - 1] * wire + public_contribution(c, &full[side], side, field)
            != terminals[side].value
        {
            return Err(format!("single copy side {side} recombination"));
        }
        output.witness.extend((0..3).map(|block| {
            let mut q = restrict_binary(&full[side], block);
            q.value = proof.wire_values[side][block];
            lift(q, block, log_z)
        }));
    }
    output.public = (0..3 * c.h())
        .map(|i| {
            let mut q = restrict_binary(&full[0], i / c.h());
            q.value = proof.label_values[i];
            lift(q, 5 + i, log_public)
        })
        .collect();
    Ok(output)
}

/// Rejection sampling applies only to the initial fingerprint challenge.
/// Zero-padded source entries give a common z^padding factor, so z must be nonzero.
struct NonzeroFirst<'a, C> {
    inner: &'a mut C,
    first: bool,
}
impl<C: Challenger> Challenger for NonzeroFirst<'_, C> {
    fn observe_label(&mut self, label: &[u8]) {
        self.inner.observe_label(label);
    }
    fn observe_bytes(&mut self, bytes: &[u8]) {
        self.inner.observe_bytes(bytes);
    }
    fn observe_f128(&mut self, value: F128) {
        self.inner.observe_f128(value);
    }
    fn observe_f128_slice(&mut self, values: &[F128]) {
        self.inner.observe_f128_slice(values);
    }
    fn sample_f128(&mut self) -> F128 {
        if !self.first {
            return self.inner.sample_f128();
        }
        self.first = false;
        loop {
            let value = self.inner.sample_f128();
            if !value.is_zero() {
                return value;
            }
        }
    }
    fn sample_f128_vec(&mut self, n: usize) -> Vec<F128> {
        if !self.first {
            return self.inner.sample_f128_vec(n);
        }
        (0..n).map(|_| self.sample_f128()).collect()
    }
    fn hash_kind(&self) -> flock_hash::HashKind {
        self.inner.hash_kind()
    }
    fn fork_from_seed(&self, _: [F128; 2], _: &'static [u8]) -> Self {
        panic!("copy rejection wrapper cannot fork")
    }
}

pub fn prepare_single(c: &Circuit, z: &[F128]) -> Result<packed::Prepared, String> {
    check_circuit(c)?;
    if z.len() != c.witness_words() {
        return Err("single copy preparation shape".into());
    }
    let source = permutation::prepare(c.single_copy_source(z, 0), c.single_copy_source(z, 1))
        .map_err(|e| format!("single copy source: {e}"))?;
    packed::prepare(&source).map_err(|e| format!("single packed preparation: {e}"))
}

/// Public circuit state only: neither witness bits nor current public I/O values
/// are captured. A plan can therefore be reused for new inputs on this circuit.
pub struct Preprocessing {
    plan: packed::CopyPreparationPlan,
    structure_digest: [u8; 32],
    io_len: usize,
}

impl Preprocessing {
    pub fn resident_bytes(&self) -> usize {
        self.plan.resident_bytes()
    }
}

pub fn preprocess(c: &Circuit) -> Result<Preprocessing, String> {
    check_circuit(c)?;
    let statement = CommittedStatement::new(c.copy_len(), (c.h() + 1).next_power_of_two())
        .map_err(|e| e.to_string())?;
    let labels = std::array::from_fn(|side| {
        (0..c.copy_len())
            .into_par_iter()
            .map(|i| {
                if i >= c.live_len() {
                    0
                } else if side == 0 {
                    c.sigma(i) as u128
                } else {
                    i as u128
                }
            })
            .collect()
    });
    Ok(Preprocessing {
        plan: packed::CopyPreparationPlan::new(statement, labels, c.live_len())
            .map_err(|e| format!("public copy preprocessing: {e}"))?,
        structure_digest: c.structure_digest,
        io_len: c.io.len(),
    })
}

#[derive(Default)]
pub struct PreparationTimings {
    pub input_ms: f64,
    pub layers_ms: f64,
    pub samples_ms: f64,
}

/// The reference switch selects the previous v6 SWAR path. Both modes still
/// charge witness-dependent construction, allocations and conversion online.
pub fn prepare_online(
    c: &Circuit,
    z: &[F128],
    public: &Preprocessing,
) -> Result<(packed::Prepared, PreparationTimings), String> {
    prepare_online_impl(
        c,
        z,
        public,
        std::env::var_os("MIKROS_SINGLE_COPY_REFERENCE").is_some_and(|v| v == "1"),
    )
}

fn prepare_online_impl(
    c: &Circuit,
    z: &[F128],
    public: &Preprocessing,
    reference: bool,
) -> Result<(packed::Prepared, PreparationTimings), String> {
    check_circuit(c)?;
    if z.len() != c.witness_words()
        || public.structure_digest != c.structure_digest
        || public.io_len != c.io.len()
        || public.plan.statement().rows != c.copy_len()
        || public.plan.statement().width != (c.h() + 1).next_power_of_two()
    {
        return Err("single copy preprocessing mismatch".into());
    }
    if reference {
        let start = Instant::now();
        let source = permutation::prepare(c.single_copy_source(z, 0), c.single_copy_source(z, 1))
            .map_err(|e| format!("single copy source: {e}"))?;
        let input_ms = start.elapsed().as_secs_f64() * 1000.0;
        let start = Instant::now();
        let result =
            packed::prepare(&source).map_err(|e| format!("single packed preparation: {e}"))?;
        Ok((
            result,
            PreparationTimings {
                input_ms,
                // The v6 reference constructs coefficient layers and samples in
                // one call. Report the combined cost here, never as a fake split.
                layers_ms: start.elapsed().as_secs_f64() * 1000.0,
                samples_ms: 0.0,
            },
        ))
    } else {
        let (result, times) = public
            .plan
            .prepare(|i| {
                if i < 3 * c.n() {
                    super::fixture::bit(z, i)
                } else if i < c.live_len() {
                    c.io[i - 3 * c.n()]
                } else {
                    false
                }
            })
            .map_err(|e| format!("single copy online preparation: {e}"))?;
        Ok((
            result,
            PreparationTimings {
                input_ms: times.leaf_ms,
                layers_ms: times.layers_ms,
                samples_ms: times.samples_ms,
            },
        ))
    }
}

pub fn single_prove<C: Challenger>(
    c: &Circuit,
    z: &[F128],
    public: &[F128],
    prepared: &packed::Prepared,
    ch: &mut C,
) -> Result<(SingleProof, Claims), String> {
    let (log_z, log_public) = input_layout(c, z, public)?;
    let statement = CommittedStatement::new(c.copy_len(), (c.h() + 1).next_power_of_two())
        .map_err(|e| e.to_string())?;
    if prepared.statement() != &statement {
        return Err("single prepared copy shape".into());
    }
    let trace = std::env::var_os("E2E_PIOP_TRACE").is_some_and(|v| !v.is_empty() && v != "0");
    let start = trace.then(Instant::now);
    ch.observe_label(b"mikros-plonk-single-copy-v1");
    let (permutation, terminals, timings) = packed::prove(
        prepared,
        &mut NonzeroFirst {
            inner: ch,
            first: true,
        },
    )
    .map_err(|e| format!("single packed copy: {e}"))?;
    if let Some(start) = start {
        eprintln!(
            "e2e_piop_trace copy=mikros-single stage=tree ms={:.6}",
            start.elapsed().as_secs_f64() * 1000.0
        );
        eprintln!(
            "e2e_piop_trace copy=mikros-single tree_profile={}",
            serde_json::to_string(&timings).unwrap()
        );
    }
    let start = trace.then(Instant::now);
    let full = [
        packed_binary_query(c, &terminals[0])?,
        packed_binary_query(c, &terminals[1])?,
    ];
    let (wire_values, label_values) = evaluate_terminals(c, z, public, &full);
    let proof = SingleProof {
        permutation,
        wire_values,
        label_values,
    };
    for values in &proof.wire_values {
        ch.observe_f128_slice(values);
    }
    ch.observe_f128_slice(&proof.label_values);
    let claims = single_expand(c, log_z, log_public, &proof, &terminals)?;
    if let Some(start) = start {
        eprintln!(
            "e2e_piop_trace copy=mikros-single stage=source_terminals ms={:.6}",
            start.elapsed().as_secs_f64() * 1000.0
        );
    }
    Ok((proof, claims))
}

pub fn single_verify<C: Challenger>(
    c: &Circuit,
    log_z: usize,
    log_public: usize,
    proof: &SingleProof,
    ch: &mut C,
) -> Result<Claims, String> {
    check_layout(c, log_z, log_public)?;
    if proof.label_values.len() != 3 * c.h() {
        return Err("single copy proof shape".into());
    }
    let statement = CommittedStatement::new(c.copy_len(), (c.h() + 1).next_power_of_two())
        .map_err(|e| e.to_string())?;
    ch.observe_label(b"mikros-plonk-single-copy-v1");
    let terminals = packed::verify(
        &statement,
        &proof.permutation,
        &mut NonzeroFirst {
            inner: ch,
            first: true,
        },
    )
    .map_err(|e| format!("single packed replay: {e}"))?;
    for values in &proof.wire_values {
        ch.observe_f128_slice(values);
    }
    ch.observe_f128_slice(&proof.label_values);
    single_expand(c, log_z, log_public, proof, &terminals)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::challenger::FsChallenger;
    use flock_field::test_rng::Rng;

    fn ch() -> FsChallenger {
        FsChallenger::new(b"single-hash-plonk-copy-tests-v1")
    }
    fn assert_claims(z: &[F128], public: &[F128], claims: &Claims) {
        assert_eq!(claims.witness.len(), 6);
        for q in &claims.witness {
            assert_eq!(evaluate(z, q), q.value);
        }
        for q in &claims.public {
            assert_eq!(evaluate(public, q), q.value);
        }
    }
    fn point_weight(point: &[F128], index: usize) -> F128 {
        point.iter().enumerate().fold(F128::ONE, |v, (j, &x)| {
            v * if index >> j & 1 == 0 {
                F128::ONE + x
            } else {
                x
            }
        })
    }

    #[test]
    fn shared_row_tables_match_scalar_binary_oracle_and_reference() {
        let mut rng = Rng::new(0x51A_2026);
        for log_rows in [0usize, 1, 3, 4, 7, 9] {
            let words = rng.f128_vec(1usize << log_rows);
            for zero_prefix in [false, true] {
                let mut full = TensorClaim {
                    columns: rng.f128_vec(128),
                    rows: pairs(&rng.f128_vec(log_rows + 2)),
                    value: F128::ZERO,
                };
                if zero_prefix {
                    full.rows[log_rows] = [F128::ONE, F128::ZERO];
                }
                let evaluation = BinaryEvaluation::new(&full);
                for block in 0..3 {
                    let q = restrict_binary(&full, block);
                    let actual = prefix(&full, block) * evaluation.evaluate(&words, &full.columns);
                    assert_eq!(actual, evaluate(&words, &q));
                    let expected = words
                        .iter()
                        .enumerate()
                        .fold(F128::ZERO, |sum, (row, word)| {
                            let weight = q
                                .rows
                                .iter()
                                .enumerate()
                                .fold(F128::ONE, |v, (j, pair)| v * pair[(row >> j) & 1]);
                            let bits = word.lo as u128 | ((word.hi as u128) << 64);
                            let value = (0..128)
                                .filter(|&j| (bits >> j) & 1 == 1)
                                .fold(F128::ZERO, |v, j| v + q.columns[j]);
                            sum + weight * value
                        });
                    assert_eq!(actual, expected);
                }
            }
        }
    }

    #[test]
    fn arbitrary_copy_terminals_match_direct_source_and_reference_folds() {
        let (c, z) = super::super::fixture::tiny_fixture();
        let public = c.public();
        let d = (c.h() + 1).next_power_of_two();
        let field = SmallField::for_width(d).unwrap();
        let kappa = 128 / d;
        let mut rng = Rng::new(0xC09_2026);
        for case in 0..4 {
            let mut terminals: [packed::Claim; 2] = std::array::from_fn(|_| packed::Claim {
                row_point: rng.f128_vec(c.h() - kappa.ilog2() as usize),
                column_weights: rng.f128_vec(kappa),
                value: F128::ZERO,
            });
            for q in &mut terminals {
                if case == 1 {
                    q.column_weights.fill(F128::ZERO);
                }
                if case == 2 {
                    q.column_weights.fill(F128::ONE);
                }
                if case == 3 {
                    *q.row_point.last_mut().unwrap() = F128::ONE;
                }
            }
            let full = terminals
                .each_ref()
                .map(|q| packed_binary_query(&c, q).unwrap());
            for side in 0..2 {
                let source = c.single_copy_source(&z, side);
                assert!((c.live_len()..c.copy_len()).all(|i| source.row(i) == 0));
                terminals[side].value = (0..c.copy_len()).fold(F128::ZERO, |sum, i| {
                    let q = &terminals[side];
                    let weight =
                        q.column_weights[i & (kappa - 1)] * point_weight(&q.row_point, i / kappa);
                    sum + weight * field.embed(source.row(i))
                });
            }
            let (wire_values, label_values) = evaluate_terminals(&c, &z, &public, &full);
            let m = c.wire_words();
            for side in 0..2 {
                for wire in 0..3 {
                    assert_eq!(
                        wire_values[side][wire],
                        evaluate(
                            &z[wire * m..(wire + 1) * m],
                            &restrict_binary(&full[side], wire)
                        )
                    );
                }
            }
            for i in 0..3 * c.h() {
                assert_eq!(
                    label_values[i],
                    evaluate(
                        &public[(5 + i) * m..(6 + i) * m],
                        &restrict_binary(&full[0], i / c.h())
                    )
                );
            }
            let proof = SingleProof {
                permutation: packed::Proof {
                    roots: [F128::ZERO; 2],
                    extension: std::array::from_fn(|_| permutation::TreeProof { layers: vec![] }),
                    packed_layers: vec![],
                },
                wire_values,
                label_values,
            };
            let claims = single_expand(
                &c,
                z.len().ilog2() as usize,
                public.len().ilog2() as usize,
                &proof,
                &terminals,
            )
            .unwrap();
            assert_eq!(claims.public.len(), 3 * c.h());
            assert_claims(&z, &public, &claims);
        }
    }

    #[test]
    fn single_copy_replay_authenticates_all_io_and_rejects_tampering() {
        let (c, z) = super::super::fixture::tiny_fixture();
        let public = c.public();
        let logs = (z.len().ilog2() as usize, public.len().ilog2() as usize);
        let prepared = prepare_single(&c, &z).unwrap();
        let mut prover = ch();
        let (proof, claims) = single_prove(&c, &z, &public, &prepared, &mut prover).unwrap();
        let mut verifier = ch();
        let verified = single_verify(&c, logs.0, logs.1, &proof, &mut verifier).unwrap();
        assert_eq!(claims.witness, verified.witness);
        assert_eq!(claims.public, verified.public);
        assert_claims(&z, &public, &verified);
        assert_eq!(prover.sample_f128(), verifier.sample_f128());
        for i in 0..c.io.len() {
            let mut wrong = c.clone();
            wrong.io[i] = !wrong.io[i];
            assert!(single_verify(&wrong, logs.0, logs.1, &proof, &mut ch()).is_err());
        }
        for i in 0..proof.label_values.len() {
            let mut bad = proof.clone();
            bad.label_values[i] += F128::ONE;
            assert!(single_verify(&c, logs.0, logs.1, &bad, &mut ch()).is_err());
        }
        for side in 0..2 {
            for wire in 0..3 {
                let mut bad = proof.clone();
                bad.wire_values[side][wire] += F128::ONE;
                assert!(single_verify(&c, logs.0, logs.1, &bad, &mut ch()).is_err());
            }
        }
        let mut bad = proof.clone();
        bad.permutation.roots[0] += F128::ONE;
        assert!(single_verify(&c, logs.0, logs.1, &bad, &mut ch()).is_err());
        let mut bad = proof.clone();
        bad.label_values.pop();
        assert!(single_verify(&c, logs.0, logs.1, &bad, &mut ch()).is_err());
        assert!(single_verify(&c, logs.0 + 1, logs.1, &proof, &mut ch()).is_err());
    }

    #[test]
    fn public_preprocessing_reuses_labels_but_reads_new_witness_and_io() {
        let (c, z) = super::super::fixture::tiny_fixture();
        let cache = preprocess(&c).unwrap();
        let public = c.public();
        for value in [None, Some(false), Some(true)] {
            let mut current = c.clone();
            let mut witness = z.clone();
            if let Some(bit) = value {
                current.io.fill(bit);
                witness.fill(F128::ZERO);
                if bit {
                    witness[..3 * current.wire_words()].fill(F128::new(u64::MAX, u64::MAX));
                }
            }
            let (reference, _) = prepare_online_impl(&current, &witness, &cache, true).unwrap();
            let (optimized, _) = prepare_online_impl(&current, &witness, &cache, false).unwrap();
            let mut reference_ch = ch();
            let mut optimized_ch = ch();
            let (reference_proof, reference_claims) =
                single_prove(&current, &witness, &public, &reference, &mut reference_ch).unwrap();
            let (optimized_proof, optimized_claims) =
                single_prove(&current, &witness, &public, &optimized, &mut optimized_ch).unwrap();
            assert_eq!(
                bincode::serialize(&reference_proof).unwrap(),
                bincode::serialize(&optimized_proof).unwrap()
            );
            assert_eq!(reference_claims.witness, optimized_claims.witness);
            assert_eq!(reference_claims.public, optimized_claims.public);
            assert_eq!(reference_ch.sample_f128(), optimized_ch.sample_f128());
            let mut verifier = ch();
            let verified = single_verify(
                &current,
                witness.len().ilog2() as usize,
                public.len().ilog2() as usize,
                &optimized_proof,
                &mut verifier,
            )
            .unwrap();
            assert_claims(&witness, &public, &verified);
        }
        let mut changed = c.clone();
        changed.structure_digest[0] ^= 1;
        assert!(prepare_online_impl(&changed, &z, &cache, false).is_err());
        changed = c.clone();
        changed.io.pop();
        assert!(prepare_online_impl(&changed, &z, &cache, false).is_err());
    }

    #[test]
    fn nonzero_filter_only_rejects_initial_zero_challenges() {
        struct Script(std::collections::VecDeque<F128>);
        impl Challenger for Script {
            fn observe_f128(&mut self, _: F128) {}
            fn sample_f128(&mut self) -> F128 {
                self.0.pop_front().unwrap()
            }
            fn fork_from_seed(&self, _: [F128; 2], _: &'static [u8]) -> Self {
                panic!("sequential test")
            }
        }
        let mut source = Script([F128::ZERO, F128::ZERO, F128::ONE, F128::ZERO].into());
        let mut filtered = NonzeroFirst {
            inner: &mut source,
            first: true,
        };
        assert_eq!(filtered.sample_f128(), F128::ONE);
        assert_eq!(filtered.sample_f128(), F128::ZERO);
        assert!(source.0.is_empty());
    }
}
