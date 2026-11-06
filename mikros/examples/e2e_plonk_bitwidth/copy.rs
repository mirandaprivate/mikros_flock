//! Packed PLONK copy argument with b-bit values and public position digits.
//! All returned claims address the original raw packed PCS inputs. The verifier
//! needs only the circuit, commitments' dimensions and proof messages.

use super::{
    fixture::{Circuit, Claims},
    queries::{evaluate, lift, pairs, tensor},
};
use flock_core::{
    challenger::Challenger,
    pcs::{
        functional::{TensorClaim, row_weights},
        ring_switch::{
            claim_check, fold_1b_rows_naive, fold_1b_rows_split, fold_1b_rows_split_2way,
        },
    },
    zerocheck::PaddingSpec,
};
use flock_field::F128;
use mikros_permutation::{
    bits::PackedBits,
    field::SmallField,
    packed::{self, BoundedPrepared},
    permutation::{self, CommittedStatement},
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[path = "wide_copy.rs"]
mod wide_copy;

fn profile_enabled() -> bool {
    std::env::var_os("MIKROS_PERMUTATION_PROFILE").is_some_and(|value| value == "1")
}

fn elapsed_ms(start: Option<Instant>) -> f64 {
    start.map_or(0.0, |start| start.elapsed().as_secs_f64() * 1000.0)
}

/// The wrapper adds a bincode discriminant; the packed variant retains its
/// original protocol messages and Fiat--Shamir challenges byte-for-byte.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Proof {
    Packed(PackedProof),
    Native128(NativeProof),
    Wide256(wide_copy::Proof),
}

pub enum Prepared {
    Packed(BoundedPrepared),
    Native128(permutation::PreparedPermutation),
    Wide256(wide_copy::Prepared),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NativeProof {
    pub permutation: permutation::Proof,
    pub wire_values: [[F128; 3]; 2],
    pub label_values: Vec<F128>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackedProof {
    pub permutation: packed::Proof,
    /// Already embedded in the high b coordinates of the tag field.
    /// Side zero is sigma; side one is the identity labelling.
    pub wire_values: [[F128; 3]; 2],
    /// Wire-major, then low-to-high b-bit label digit; already tag-embedded.
    pub label_values: Vec<F128>,
}

fn shape(c: &Circuit) -> Result<CommittedStatement, String> {
    c.validate()?;
    CommittedStatement::new(c.copy_len(), c.tag_bits()).map_err(|e| e.to_string())
}

fn check_layout(c: &Circuit, log_z: usize, log_public: usize) -> Result<(), String> {
    c.validate()?;
    if log_z != c.witness_words().ilog2() as usize
        || log_public != c.public_words().ilog2() as usize
    {
        return Err("bitwidth copy commitment dimensions".into());
    }
    Ok(())
}

/// Warm public field/domain tables using a tiny, fixed all-zero source only.
/// No actual witness values or witness-dependent coefficient buffers survive.
pub fn warmup(c: &Circuit) -> Result<(), String> {
    c.validate()?;
    if c.tag_bits() > 64 {
        SmallField::for_width(128).map_err(|e| e.to_string())?;
        return Ok(());
    }
    shape(c)?;
    let kappa = 128 / c.tag_bits();
    let rows = (kappa * kappa).max(16);
    let source =
        PackedBits::from_fn_parallel(rows, c.tag_bits(), |_| 0).map_err(|e| e.to_string())?;
    let source = permutation::prepare(source.clone(), source).map_err(|e| e.to_string())?;
    drop(packed::prepare_bounded(&source).map_err(|e| e.to_string())?);
    Ok(())
}

/// Online preparation: both source tags and every coefficient depend on z.
pub fn prepare(c: &Circuit, z: &[F128]) -> Result<Prepared, String> {
    let reference = std::env::var_os("MIKROS_PERMUTATION_REFERENCE_PREPARATION")
        .is_some_and(|value| value == "1");
    prepare_with_source(c, z, reference, profile_enabled())
}

fn prepare_with_source(
    c: &Circuit,
    z: &[F128],
    reference: bool,
    profile: bool,
) -> Result<Prepared, String> {
    let total_start = profile.then(Instant::now);
    c.validate()?;
    if c.tag_bits() == 256 {
        return wide_copy::prepare(c, z).map(Prepared::Wide256);
    }
    let statement = shape(c)?;
    if z.len() != c.witness_words() {
        return Err("bitwidth copy witness dimensions".into());
    }
    if c.tag_bits() <= 64 && !reference {
        let start = profile.then(Instant::now);
        let live_len = c.live_len();
        let io_index = 3 * c.n();
        let value_shift = c.tag_bits() - c.value_bits;
        let prepared = packed::prepare_bounded_from_fn(statement, |index| {
            if index >= live_len {
                return [0, 0];
            }
            // Both sides contain the same witness value. Only their public
            // label differs; retain the original padding and IO convention.
            let value = if index == io_index {
                c.io
            } else {
                c.packed_value(z, index)
            };
            let value = value << value_shift;
            [c.sigma(index) as u128 | value, index as u128 | value]
        })
        .map_err(|e| format!("bitwidth direct copy preparation: {e}"))?;
        let packed_prepare_ms = elapsed_ms(start);
        if profile {
            eprintln!(
                "PERMUTATION_PROFILE {}",
                serde_json::json!({
                    "stage": "prepare", "mode": "packed", "log_gates": c.log_gates,
                    "value_bits": c.value_bits, "direct_rows": true,
                    "packed_prepare_ms": packed_prepare_ms, "total_ms": elapsed_ms(total_start),
                })
            );
        }
        return Ok(Prepared::Packed(prepared));
    }
    let start = profile.then(Instant::now);
    let source = |side| {
        PackedBits::from_fn_parallel(c.copy_len(), c.tag_bits(), |index| {
            if index >= c.live_len() {
                return 0;
            }
            let label = if side == 0 { c.sigma(index) } else { index };
            let value = if index == 3 * c.n() {
                c.io
            } else {
                c.packed_value(z, index)
            };
            label as u128 | (value << (c.tag_bits() - c.value_bits))
        })
        .map_err(|e| format!("bitwidth copy source: {e}"))
    };
    let (left, right) = (source(0)?, source(1)?);
    let source_vectors_ms = elapsed_ms(start);
    let start = profile.then(Instant::now);
    let source =
        permutation::prepare(left, right).map_err(|e| format!("bitwidth copy sources: {e}"))?;
    let source_statement_ms = elapsed_ms(start);
    if c.tag_bits() == 128 {
        if profile {
            eprintln!(
                "PERMUTATION_PROFILE {}",
                serde_json::json!({
                    "stage": "prepare", "mode": "native128", "log_gates": c.log_gates,
                    "value_bits": c.value_bits, "source_vectors_ms": source_vectors_ms,
                    "source_statement_ms": source_statement_ms, "direct_rows": false,
                    "total_ms": elapsed_ms(total_start),
                })
            );
        }
        Ok(Prepared::Native128(source))
    } else {
        let start = profile.then(Instant::now);
        let prepared = packed::prepare_bounded(&source)
            .map(Prepared::Packed)
            .map_err(|e| format!("bitwidth copy preparation: {e}"))?;
        let packed_prepare_ms = elapsed_ms(start);
        let start = profile.then(Instant::now);
        drop(source);
        let source_release_ms = elapsed_ms(start);
        if profile {
            eprintln!(
                "PERMUTATION_PROFILE {}",
                serde_json::json!({
                    "stage": "prepare", "mode": "packed", "log_gates": c.log_gates,
                    "value_bits": c.value_bits, "source_vectors_ms": source_vectors_ms,
                    "source_statement_ms": source_statement_ms,
                    "packed_prepare_ms": packed_prepare_ms, "source_release_ms": source_release_ms,
                    "direct_rows": false, "total_ms": elapsed_ms(total_start),
                })
            );
        }
        Ok(prepared)
    }
}

/// One scalar coefficient per value slot, plus packed-word row coordinates.
struct Weights {
    columns: Vec<F128>,
    rows: Vec<[F128; 2]>,
}

fn terminal_weights(c: &Circuit, claim: &packed::Claim) -> Result<Weights, String> {
    let copy_kappa = 128 / c.tag_bits();
    let copy_log = copy_kappa.ilog2() as usize;
    let word_log = c.kappa().ilog2() as usize;
    if claim.column_weights.len() != copy_kappa || claim.row_point.len() != c.h() - copy_log {
        return Err("bitwidth copy terminal dimensions".into());
    }
    let fold = word_log - copy_log;
    let low = tensor(&claim.row_point[..fold]);
    Ok(Weights {
        columns: (0..c.kappa())
            .map(|j| claim.column_weights[j & (copy_kappa - 1)] * low[j >> copy_log])
            .collect(),
        rows: pairs(&claim.row_point[fold..]),
    })
}

impl Weights {
    fn at(&self, index: usize) -> F128 {
        let log = self.columns.len().ilog2() as usize;
        self.rows.iter().enumerate().fold(
            self.columns[index & (self.columns.len() - 1)],
            |value, (j, row)| value * row[(index >> (log + j)) & 1],
        )
    }

    /// Restrict to a complete N-value wire; preserve zero/nonzero block scales
    /// in column weights rather than normalizing or dividing by them.
    fn wire_query(&self, c: &Circuit, wire: usize, coefficients: &[F128]) -> TensorClaim {
        assert_eq!(coefficients.len(), c.value_bits);
        let n = self.rows.len();
        let prefix = self.rows[n - 2][wire & 1] * self.rows[n - 1][wire >> 1];
        TensorClaim {
            columns: self
                .columns
                .iter()
                .flat_map(|&column| coefficients.iter().map(move |&bit| prefix * column * bit))
                .collect(),
            rows: self.rows[..n - 2].to_vec(),
            value: F128::ZERO,
        }
    }
}

/// Public identity labels on the three live witness blocks. Ordinary MLE row
/// pairs sum to one; arbitrary column weights and zero block scales are kept.
fn identity_labels(c: &Circuit, weights: &Weights, basis: &[F128]) -> F128 {
    let low_bits = c.kappa().ilog2() as usize;
    let mass = weights
        .columns
        .iter()
        .copied()
        .fold(F128::ZERO, |a, b| a + b);
    let low = weights
        .columns
        .iter()
        .enumerate()
        .fold(F128::ZERO, |sum, (j, &weight)| {
            let label = (0..low_bits).fold(F128::ZERO, |value, bit| {
                if j >> bit & 1 == 1 {
                    value + basis[bit]
                } else {
                    value
                }
            });
            sum + weight * label
        });
    let middle = (low_bits..c.log_gates).fold(F128::ZERO, |sum, bit| {
        sum + basis[bit] * weights.rows[bit - low_bits][1]
    });
    (0..3).fold(F128::ZERO, |sum, block| {
        let prefix = weights.rows[c.log_gates - low_bits][block & 1]
            * weights.rows[c.log_gates - low_bits + 1][block >> 1];
        let high = (0..2).fold(F128::ZERO, |value, bit| {
            if block >> bit & 1 == 1 {
                value + basis[c.log_gates + bit]
            } else {
                value
            }
        });
        sum + prefix * (low + mass * (middle + high))
    })
}

fn expand(
    c: &Circuit,
    log_z: usize,
    log_public: usize,
    wire_values: &[[F128; 3]; 2],
    label_values: &[F128],
    terminals: &[packed::Claim; 2],
) -> Result<Claims, String> {
    if label_values.len() != 3 * c.label_digits() {
        return Err("bitwidth copy label message dimensions".into());
    }
    let field = SmallField::for_width(c.tag_bits()).map_err(|e| e.to_string())?;
    let basis = field.basis();
    let value_basis = &basis[c.tag_bits() - c.value_bits..];
    let mut claims = Claims::default();
    for side in 0..2 {
        let weights = terminal_weights(c, &terminals[side])?;
        let labels = if side == 0 {
            label_values.iter().copied().fold(F128::ZERO, |a, b| a + b)
        } else {
            identity_labels(c, &weights, basis)
        };
        let wires = wire_values[side]
            .iter()
            .copied()
            .fold(F128::ZERO, |a, b| a + b);
        let io_index = 3 * c.n();
        let io_label = if side == 0 {
            c.sigma(io_index)
        } else {
            io_index
        };
        let io = weights.at(io_index)
            * field.embed(io_label as u128 | (c.io << (c.tag_bits() - c.value_bits)));
        if wires + labels + io != terminals[side].value {
            return Err(format!("bitwidth copy side {side} recombination"));
        }
        for wire in 0..3 {
            let mut q = weights.wire_query(c, wire, value_basis);
            q.value = wire_values[side][wire];
            claims.witness.push(lift(q, wire, log_z));
        }
        if side == 0 {
            for wire in 0..3 {
                for digit in 0..c.label_digits() {
                    let start = digit * c.value_bits;
                    let mut q = weights.wire_query(c, wire, &basis[start..start + c.value_bits]);
                    q.value = label_values[wire * c.label_digits() + digit];
                    claims
                        .public
                        .push(lift(q, 5 + wire * c.label_digits() + digit, log_public));
                }
            }
        }
    }
    Ok(claims)
}

fn observe_values<C: Challenger>(wire_values: &[[F128; 3]; 2], label_values: &[F128], ch: &mut C) {
    for values in wire_values {
        ch.observe_f128_slice(values);
    }
    ch.observe_f128_slice(label_values);
}

#[derive(Default, Serialize)]
struct TerminalTimings {
    row_weights_ms: f64,
    witness_ms: f64,
    labels_ms: f64,
}

struct TerminalValues {
    wires: [[F128; 3]; 2],
    labels: Vec<F128>,
    timings: TerminalTimings,
}

/// Query-dependent row tensors are identical for every wire and label digit
/// on one side. Keep these small factors online and reuse the native fold.
struct TerminalRowWeights {
    lo: Vec<F128>,
    hi: Vec<F128>,
    log_rows: usize,
}

impl TerminalRowWeights {
    fn new(rows: &[[F128; 2]]) -> Self {
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

    fn fold(&self, words: &[F128]) -> Vec<F128> {
        assert_eq!(words.len(), 1usize << self.log_rows);
        if self.log_rows < 4 {
            fold_1b_rows_naive(words, &self.lo)
        } else {
            fold_1b_rows_split(
                words,
                &self.lo,
                &self.hi,
                &PaddingSpec::dense(self.log_rows + 7),
            )
        }
    }

    fn fold_pair(&self, other: &Self, words: &[F128]) -> (Vec<F128>, Vec<F128>) {
        assert_eq!(self.log_rows, other.log_rows);
        if self.log_rows < 4 {
            (self.fold(words), other.fold(words))
        } else {
            fold_1b_rows_split_2way(
                words,
                &self.lo,
                &self.hi,
                &other.lo,
                &other.hi,
                &PaddingSpec::dense(self.log_rows + 7),
            )
        }
    }
}

/// Original evaluation order retained for exact same-binary ablations.
fn terminal_values_reference(
    c: &Circuit,
    z: &[F128],
    public: &[F128],
    terminals: &[packed::Claim; 2],
    basis: &[F128],
    profile: bool,
) -> Result<TerminalValues, String> {
    let mut values = TerminalValues {
        wires: [[F128::ZERO; 3]; 2],
        labels: vec![],
        timings: TerminalTimings::default(),
    };
    let m = c.wire_words();
    for side in 0..2 {
        let start = profile.then(Instant::now);
        let weights = terminal_weights(c, &terminals[side])?;
        values.timings.row_weights_ms += elapsed_ms(start);
        let start = profile.then(Instant::now);
        for wire in 0..3 {
            let q = weights.wire_query(c, wire, &basis[c.tag_bits() - c.value_bits..]);
            values.wires[side][wire] = evaluate(&z[wire * m..(wire + 1) * m], &q);
        }
        values.timings.witness_ms += elapsed_ms(start);
        if side == 0 {
            let start = profile.then(Instant::now);
            for wire in 0..3 {
                for digit in 0..c.label_digits() {
                    let bit = digit * c.value_bits;
                    let q = weights.wire_query(c, wire, &basis[bit..bit + c.value_bits]);
                    let block = 5 + wire * c.label_digits() + digit;
                    values
                        .labels
                        .push(evaluate(&public[block * m..(block + 1) * m], &q));
                }
            }
            values.timings.labels_ms += elapsed_ms(start);
        }
    }
    Ok(values)
}

fn terminal_values_shared(
    c: &Circuit,
    z: &[F128],
    public: &[F128],
    terminals: &[packed::Claim; 2],
    basis: &[F128],
    profile: bool,
) -> Result<TerminalValues, String> {
    let start = profile.then(Instant::now);
    let weights = [
        terminal_weights(c, &terminals[0])?,
        terminal_weights(c, &terminals[1])?,
    ];
    let rows = weights
        .each_ref()
        .map(|w| TerminalRowWeights::new(&w.rows[..w.rows.len() - 2]));
    let mut values = TerminalValues {
        wires: [[F128::ZERO; 3]; 2],
        labels: Vec::with_capacity(3 * c.label_digits()),
        timings: TerminalTimings {
            row_weights_ms: elapsed_ms(start),
            ..TerminalTimings::default()
        },
    };
    let m = c.wire_words();
    let start = profile.then(Instant::now);
    for wire in 0..3 {
        // Two independent points share only input loads and bit transposes;
        // the existing two-way kernel keeps their accumulators separate.
        let (left, right) = rows[0].fold_pair(&rows[1], &z[wire * m..(wire + 1) * m]);
        for (side, folded) in [left, right].iter().enumerate() {
            let q = weights[side].wire_query(c, wire, &basis[c.tag_bits() - c.value_bits..]);
            values.wires[side][wire] = claim_check(&q.columns, folded);
        }
    }
    values.timings.witness_ms = elapsed_ms(start);
    let start = profile.then(Instant::now);
    // Each label digit is a DIFFERENT public source block. Reuse its row
    // factors, not its folded values. Four outer jobs bound simultaneous
    // subset tables while leaving the fold's existing Rayon parallelism free.
    let count = 3 * c.label_digits();
    for begin in (0..count).step_by(4) {
        let batch: Vec<F128> = (begin..(begin + 4).min(count))
            .into_par_iter()
            .map(|index| {
                let (wire, digit) = (index / c.label_digits(), index % c.label_digits());
                let bit = digit * c.value_bits;
                let q = weights[0].wire_query(c, wire, &basis[bit..bit + c.value_bits]);
                let block = 5 + index;
                claim_check(
                    &q.columns,
                    &rows[0].fold(&public[block * m..(block + 1) * m]),
                )
            })
            .collect();
        // Indexed collection preserves the exact wire-major/digit order.
        values.labels.extend(batch);
    }
    values.timings.labels_ms = elapsed_ms(start);
    Ok(values)
}

fn prove_packed<C: Challenger>(
    c: &Circuit,
    z: &[F128],
    public: &[F128],
    prepared: BoundedPrepared,
    ch: &mut C,
) -> Result<(PackedProof, Claims), String> {
    let reference = std::env::var_os("MIKROS_PERMUTATION_REFERENCE_TERMINALS")
        .is_some_and(|value| value == "1");
    prove_packed_with_terminals(c, z, public, prepared, ch, reference, profile_enabled())
}

fn prove_packed_with_terminals<C: Challenger>(
    c: &Circuit,
    z: &[F128],
    public: &[F128],
    prepared: BoundedPrepared,
    ch: &mut C,
    reference: bool,
    profile: bool,
) -> Result<(PackedProof, Claims), String> {
    let statement = shape(c)?;
    if z.len() != c.witness_words()
        || public.len() != c.public_words()
        || prepared.statement() != &statement
    {
        return Err("bitwidth copy source or prepared dimensions".into());
    }
    ch.observe_label(b"mikros-plonk-subfield-copy-v1");
    ch.observe_bytes(&(c.value_bits as u64).to_le_bytes());
    let start = profile.then(Instant::now);
    let (permutation, terminals, timings) = packed::prove_bounded_compact(
        prepared,
        &mut NonzeroFirst {
            inner: ch,
            first: true,
        },
    )
    .map_err(|e| format!("bitwidth packed copy: {e}"))?;
    let packed_ms = elapsed_ms(start);
    let start = profile.then(Instant::now);
    let field = SmallField::for_width(c.tag_bits()).map_err(|e| e.to_string())?;
    let basis = field.basis();
    let values = if reference {
        terminal_values_reference(c, z, public, &terminals, basis, profile)?
    } else {
        terminal_values_shared(c, z, public, &terminals, basis, profile)?
    };
    let terminal_ms = elapsed_ms(start);
    let proof = PackedProof {
        permutation,
        wire_values: values.wires,
        label_values: values.labels,
    };
    let start = profile.then(Instant::now);
    observe_values(&proof.wire_values, &proof.label_values, ch);
    let claims = expand(
        c,
        z.len().ilog2() as usize,
        public.len().ilog2() as usize,
        &proof.wire_values,
        &proof.label_values,
        &terminals,
    )?;
    if profile {
        eprintln!(
            "PERMUTATION_PROFILE {}",
            serde_json::json!({
                "stage": "prove_packed", "log_gates": c.log_gates, "value_bits": c.value_bits,
                "reference_terminals": reference, "packed_ms": packed_ms, "tree": timings,
                "terminal_ms": terminal_ms, "terminal_timings": values.timings,
                "observe_expand_ms": elapsed_ms(start), "label_queries": proof.label_values.len(),
            })
        );
    }
    Ok((proof, claims))
}

fn verify_packed<C: Challenger>(
    c: &Circuit,
    log_z: usize,
    log_public: usize,
    proof: &PackedProof,
    ch: &mut C,
) -> Result<Claims, String> {
    let statement = shape(c)?;
    check_layout(c, log_z, log_public)?;
    if proof.label_values.len() != 3 * c.label_digits() {
        return Err("bitwidth copy label message dimensions".into());
    }
    ch.observe_label(b"mikros-plonk-subfield-copy-v1");
    ch.observe_bytes(&(c.value_bits as u64).to_le_bytes());
    let terminals = packed::verify(
        &statement,
        &proof.permutation,
        &mut NonzeroFirst {
            inner: ch,
            first: true,
        },
    )
    .map_err(|e| format!("bitwidth packed replay: {e}"))?;
    observe_values(&proof.wire_values, &proof.label_values, ch);
    expand(
        c,
        log_z,
        log_public,
        &proof.wire_values,
        &proof.label_values,
        &terminals,
    )
}

fn native_terminals(
    c: &Circuit,
    claims: permutation::CommittedClaims,
) -> Result<[packed::Claim; 2], String> {
    use mikros_permutation::claims::{LinearQuery, Source};
    let basis = SmallField::for_width(128).map_err(|e| e.to_string())?;
    let mut out = Vec::with_capacity(2);
    for (side, q) in claims.evaluations.into_iter().enumerate() {
        let LinearQuery::Rows {
            point,
            basis: coefficients,
            row_offset: 0,
        } = q.query
        else {
            return Err("native copy terminal form".into());
        };
        if point.len() != c.h()
            || coefficients != basis.basis()
            || q.source != [Source::Left, Source::Right][side]
        {
            return Err("native copy terminal dimensions".into());
        }
        out.push(packed::Claim {
            row_point: point,
            column_weights: vec![F128::ONE],
            value: q.value,
        });
    }
    Ok(out.try_into().unwrap())
}

fn prove_native<C: Challenger>(
    c: &Circuit,
    z: &[F128],
    public: &[F128],
    prepared: permutation::PreparedPermutation,
    ch: &mut C,
) -> Result<(NativeProof, Claims), String> {
    let statement = shape(c)?;
    if c.tag_bits() != 128
        || z.len() != c.witness_words()
        || public.len() != c.public_words()
        || prepared.statement() != &statement
    {
        return Err("native copy source dimensions".into());
    }
    ch.observe_label(b"mikros-plonk-subfield-copy-native128-v1");
    ch.observe_bytes(&(c.value_bits as u64).to_le_bytes());
    let (permutation, source_claims, _, _) = permutation::prove_committed_lockstep(
        &prepared,
        permutation::Options::default(),
        mikros_permutation::profile::Engine::Registers,
        &mut NonzeroFirst {
            inner: ch,
            first: true,
        },
    )
    .map_err(|e| format!("native copy proof: {e}"))?;
    drop(prepared);
    let terminals = native_terminals(c, source_claims)?;
    let basis = SmallField::for_width(128).map_err(|e| e.to_string())?;
    let mut proof = NativeProof {
        permutation,
        wire_values: [[F128::ZERO; 3]; 2],
        label_values: vec![],
    };
    let m = c.wire_words();
    for side in 0..2 {
        let weights = terminal_weights(c, &terminals[side])?;
        for wire in 0..3 {
            let q = weights.wire_query(c, wire, &basis.basis()[128 - c.value_bits..]);
            proof.wire_values[side][wire] = evaluate(&z[wire * m..(wire + 1) * m], &q);
        }
        if side == 0 {
            for wire in 0..3 {
                for digit in 0..c.label_digits() {
                    let start = digit * c.value_bits;
                    let q =
                        weights.wire_query(c, wire, &basis.basis()[start..start + c.value_bits]);
                    let block = 5 + wire * c.label_digits() + digit;
                    proof
                        .label_values
                        .push(evaluate(&public[block * m..(block + 1) * m], &q));
                }
            }
        }
    }
    observe_values(&proof.wire_values, &proof.label_values, ch);
    let claims = expand(
        c,
        z.len().ilog2() as usize,
        public.len().ilog2() as usize,
        &proof.wire_values,
        &proof.label_values,
        &terminals,
    )?;
    Ok((proof, claims))
}

fn verify_native<C: Challenger>(
    c: &Circuit,
    log_z: usize,
    log_public: usize,
    proof: &NativeProof,
    ch: &mut C,
) -> Result<Claims, String> {
    let statement = shape(c)?;
    check_layout(c, log_z, log_public)?;
    if c.tag_bits() != 128 || proof.label_values.len() != 3 * c.label_digits() {
        return Err("native copy proof dimensions".into());
    }
    ch.observe_label(b"mikros-plonk-subfield-copy-native128-v1");
    ch.observe_bytes(&(c.value_bits as u64).to_le_bytes());
    let terminals = native_terminals(
        c,
        permutation::verify_committed_lockstep(
            &statement,
            &proof.permutation,
            &mut NonzeroFirst {
                inner: ch,
                first: true,
            },
        )
        .map_err(|e| format!("native copy replay: {e}"))?,
    )?;
    observe_values(&proof.wire_values, &proof.label_values, ch);
    expand(
        c,
        log_z,
        log_public,
        &proof.wire_values,
        &proof.label_values,
        &terminals,
    )
}

pub fn prove<C: Challenger>(
    c: &Circuit,
    z: &[F128],
    public: &[F128],
    prepared: Prepared,
    ch: &mut C,
) -> Result<(Proof, Claims), String> {
    c.validate()?;
    match prepared {
        Prepared::Packed(p) if c.tag_bits() <= 64 => {
            prove_packed(c, z, public, p, ch).map(|(p, q)| (Proof::Packed(p), q))
        }
        Prepared::Native128(p) if c.tag_bits() == 128 => {
            prove_native(c, z, public, p, ch).map(|(p, q)| (Proof::Native128(p), q))
        }
        Prepared::Wide256(p) if c.tag_bits() == 256 => {
            wide_copy::prove(c, z, public, p, ch).map(|(p, q)| (Proof::Wide256(p), q))
        }
        _ => Err("copy prepared field mismatch".into()),
    }
}

pub fn verify<C: Challenger>(
    c: &Circuit,
    log_z: usize,
    log_public: usize,
    proof: &Proof,
    ch: &mut C,
) -> Result<Claims, String> {
    c.validate()?;
    match proof {
        Proof::Packed(p) if c.tag_bits() <= 64 => verify_packed(c, log_z, log_public, p, ch),
        Proof::Native128(p) if c.tag_bits() == 128 => verify_native(c, log_z, log_public, p, ch),
        Proof::Wide256(p) if c.tag_bits() == 256 => wide_copy::verify(c, log_z, log_public, p, ch),
        _ => Err("copy proof field mismatch".into()),
    }
}

/// Preserve the native packed transcript, rejecting only an actual first zero.
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

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::challenger::FsChallenger;
    use flock_field::test_rng::Rng;

    fn challenger(c: &Circuit) -> FsChallenger {
        let mut ch = FsChallenger::new(b"plonk-subfield-copy-tests-v1");
        ch.observe_bytes(&c.digest());
        ch
    }

    fn authenticate(claims: &Claims, z: &[F128], public: &[F128]) -> bool {
        claims.witness.iter().all(|q| evaluate(z, q) == q.value)
            && claims.public.iter().all(|q| evaluate(public, q) == q.value)
    }

    #[test]
    fn shared_terminal_folds_match_reference_at_boolean_and_random_points() {
        let mut rng = Rng::new(0xC0F1_128);
        for bits in [1, 2, 4, 8, 16, 32, 64] {
            let c = (0..64)
                .find_map(|seed| {
                    let c = Circuit::new(14, bits, seed).unwrap();
                    (c.io != 0).then_some(c)
                })
                .expect("test fixture with nonzero public output");
            // Dense arbitrary packed inputs exercise every source bit, not
            // only the structured label patterns of the circuit fixture.
            let z = rng.f128_vec(c.witness_words());
            let public = rng.f128_vec(c.public_words());
            let field = SmallField::for_width(c.tag_bits()).unwrap();
            for case in 0..4 {
                let terminals = std::array::from_fn(|side| {
                    let len = c.h() - c.copy_kappa().ilog2() as usize;
                    let point = match case {
                        0 => vec![F128::ZERO; len],
                        1 => vec![F128::ONE; len],
                        2 => vec![[F128::ZERO, F128::ONE][side]; len],
                        _ => {
                            let mut point = rng.f128_vec(len);
                            // Exercise zero block-prefix scales on different
                            // sides without ever normalizing or dividing.
                            point[len - 1] = [F128::ZERO, F128::ONE][side];
                            point
                        }
                    };
                    packed::Claim {
                        row_point: point,
                        column_weights: rng.f128_vec(c.copy_kappa()),
                        value: F128::ZERO,
                    }
                });
                let expected =
                    terminal_values_reference(&c, &z, &public, &terminals, field.basis(), false)
                        .unwrap();
                let actual =
                    terminal_values_shared(&c, &z, &public, &terminals, field.basis(), false)
                        .unwrap();
                assert_eq!(actual.wires, expected.wires, "b={bits}, case={case}");
                assert_eq!(actual.labels, expected.labels, "b={bits}, case={case}");
                assert_eq!(actual.labels.len(), 3 * c.label_digits());
            }
        }
    }

    #[test]
    fn packed_reference_and_shared_terminals_preserve_proof_claims_and_transcript() {
        for bits in [1, 8, 32] {
            let c = (0..64)
                .find_map(|seed| {
                    let c = Circuit::new(14, bits, seed).unwrap();
                    (c.io != 0).then_some(c)
                })
                .expect("test fixture with nonzero public output");
            let z = c.witness();
            let public = c.public();
            let mut outputs = Vec::new();
            for reference in [true, false] {
                let Prepared::Packed(prepared) = prepare(&c, &z).unwrap() else {
                    panic!("expected packed copy");
                };
                let mut ch = challenger(&c);
                let (proof, claims) = prove_packed_with_terminals(
                    &c, &z, &public, prepared, &mut ch, reference, false,
                )
                .unwrap();
                assert!(authenticate(&claims, &z, &public));
                outputs.push((
                    bincode::serialize(&proof).unwrap(),
                    claims,
                    ch.sample_f128(),
                ));
            }
            assert_eq!(
                outputs[0].0, outputs[1].0,
                "serialized packed proof b={bits}"
            );
            assert_eq!(
                outputs[0].1.witness, outputs[1].1.witness,
                "witness claims b={bits}"
            );
            assert_eq!(
                outputs[0].1.public, outputs[1].1.public,
                "public claims b={bits}"
            );
            assert_eq!(
                outputs[0].2, outputs[1].2,
                "next transcript challenge b={bits}"
            );
        }
    }

    #[test]
    fn direct_and_bitplane_preparation_preserve_complete_packed_proof() {
        for bits in [1, 2, 8, 16, 32] {
            let c = (0..64)
                .find_map(|seed| {
                    let c = Circuit::new(14, bits, seed).unwrap();
                    (c.io != 0).then_some(c)
                })
                .expect("test fixture with nonzero public output");
            let z = c.witness();
            let public = c.public();
            let mut outputs = Vec::new();
            for reference in [true, false] {
                let Prepared::Packed(prepared) =
                    prepare_with_source(&c, &z, reference, false).unwrap()
                else {
                    panic!("expected packed copy");
                };
                let mut ch = challenger(&c);
                let (proof, claims) =
                    prove_packed_with_terminals(&c, &z, &public, prepared, &mut ch, false, false)
                        .unwrap();
                let mut verifier = challenger(&c);
                let verified = verify_packed(
                    &c,
                    z.len().ilog2() as usize,
                    public.len().ilog2() as usize,
                    &proof,
                    &mut verifier,
                )
                .unwrap();
                assert_eq!(verified.witness, claims.witness);
                assert_eq!(verified.public, claims.public);
                assert!(authenticate(&claims, &z, &public));
                let next = ch.sample_f128();
                assert_eq!(next, verifier.sample_f128());
                outputs.push((
                    bincode::serialize(&Proof::Packed(proof)).unwrap(),
                    claims,
                    next,
                ));
            }
            assert_eq!(outputs[0].0, outputs[1].0, "serialized proof b={bits}");
            assert_eq!(
                outputs[0].1.witness, outputs[1].1.witness,
                "witness claims b={bits}"
            );
            assert_eq!(
                outputs[0].1.public, outputs[1].1.public,
                "public claims b={bits}"
            );
            assert_eq!(
                outputs[0].2, outputs[1].2,
                "next transcript challenge b={bits}"
            );
        }
    }

    #[test]
    fn full_terminals_all_widths_and_tampering() {
        for bits in [1, 2, 4, 8, 16, 32, 64, 128] {
            let c = Circuit::new(14, bits, 97).unwrap();
            let z = c.witness();
            let public = c.public();
            warmup(&c).unwrap();
            let prepared = prepare(&c, &z).unwrap();
            let mut pc = challenger(&c);
            let (proof, claims) = prove(&c, &z, &public, prepared, &mut pc).unwrap();
            assert!(
                authenticate(&claims, &z, &public),
                "prover sources b={bits}"
            );
            assert_eq!(claims.witness.len(), 6);
            assert_eq!(claims.public.len(), 3 * c.label_digits());
            let proof: Proof = bincode::deserialize(&bincode::serialize(&proof).unwrap()).unwrap();
            let mut vc = challenger(&c);
            let got = verify(
                &c,
                z.len().ilog2() as usize,
                public.len().ilog2() as usize,
                &proof,
                &mut vc,
            )
            .unwrap();
            assert!(authenticate(&got, &z, &public), "verifier sources b={bits}");
            assert_eq!(pc.sample_f128(), vc.sample_f128());
            for mutate in 0..4 {
                let mut bad = proof.clone();
                match &mut bad {
                    Proof::Packed(bad) => match mutate {
                        0 => bad.wire_values[0][0] += F128::ONE,
                        1 => bad.label_values[0] += F128::ONE,
                        2 => {
                            bad.label_values.pop();
                        }
                        _ => bad.permutation.roots[0] += F128::ONE,
                    },
                    Proof::Native128(bad) => match mutate {
                        0 => bad.wire_values[0][0] += F128::ONE,
                        1 => bad.label_values[0] += F128::ONE,
                        2 => {
                            bad.label_values.pop();
                        }
                        _ => bad.permutation.roots[0] += F128::ONE,
                    },
                    Proof::Wide256(bad) => match mutate {
                        0 => bad.wire_values[0][0].c1 += F128::ONE,
                        1 => bad.label_values[0].c0 += F128::ONE,
                        2 => {
                            bad.label_values.pop();
                        }
                        _ => bad.tree.roots[0].c1 += F128::ONE,
                    },
                }
                assert!(
                    verify(
                        &c,
                        z.len().ilog2() as usize,
                        public.len().ilog2() as usize,
                        &bad,
                        &mut challenger(&c)
                    )
                    .is_err(),
                    "proof mutation {mutate}, b={bits}"
                );
            }
            if let Proof::Wide256(wide) = &proof {
                for component in 0..2 {
                    let mut bad = wide.clone();
                    // In characteristic two these changes cancel in the
                    // terminal recombination. They must still be rejected:
                    // witness c0 and public-label c1 are identically zero,
                    // and omitting their PCS queries cannot admit a value.
                    if component == 0 {
                        bad.wire_values[0][0].c0 += F128::ONE;
                        bad.label_values[0].c0 += F128::ONE;
                    } else {
                        bad.wire_values[0][0].c1 += F128::ONE;
                        bad.label_values[0].c1 += F128::ONE;
                    }
                    let error = verify(
                        &c,
                        z.len().ilog2() as usize,
                        public.len().ilog2() as usize,
                        &Proof::Wide256(bad),
                        &mut challenger(&c),
                    )
                    .err()
                    .expect("cancelling coordinate attack must be rejected");
                    assert!(
                        error.contains("nonzero empty coordinate"),
                        "coordinate {component}: {error}"
                    );
                }
            }
            let mut bad_z = z.clone();
            if bits <= 64 {
                bad_z[0].lo ^= 1u64 << (bits - 1);
            } else {
                bad_z[0].hi ^= 1u64 << (bits - 65);
            }
            assert!(
                !authenticate(&got, &bad_z, &public),
                "highest witness bit b={bits}"
            );
            let mut bad_public = public.clone();
            bad_public[5 * c.wire_words()].lo ^= 1;
            assert!(!authenticate(&got, &z, &bad_public), "label bit b={bits}");
            let mut bad_c = c.clone();
            bad_c.io ^= 1;
            assert!(
                verify(
                    &bad_c,
                    z.len().ilog2() as usize,
                    public.len().ilog2() as usize,
                    &proof,
                    &mut challenger(&c)
                )
                .is_err(),
                "changed public output b={bits}"
            );
        }
    }

    #[test]
    fn identity_and_bit_linear_queries_match_independent_tag_oracle() {
        for bits in [1, 2, 4, 8, 16, 32, 64] {
            let c = Circuit::new(14, bits, 31).unwrap();
            let z = c.witness();
            let field = SmallField::for_width(c.tag_bits()).unwrap();
            let copy_kappa = 128 / c.tag_bits();
            for zero_prefix in [false, true] {
                let mut point: Vec<_> = (0..c.h() - copy_kappa.ilog2() as usize)
                    .map(|j| F128::new(23 + 17 * j as u64, j as u64))
                    .collect();
                if zero_prefix {
                    *point.last_mut().unwrap() = F128::ZERO;
                }
                let terminal = packed::Claim {
                    row_point: point.clone(),
                    column_weights: (0..copy_kappa)
                        .map(|j| F128::new(71 + j as u64, 1))
                        .collect(),
                    value: F128::ZERO,
                };
                let weights = terminal_weights(&c, &terminal).unwrap();
                let direct_weight = |index: usize| {
                    point.iter().enumerate().fold(
                        terminal.column_weights[index & (copy_kappa - 1)],
                        |acc, (j, &r)| {
                            acc * if index >> (copy_kappa.ilog2() as usize + j) & 1 == 0 {
                                F128::ONE + r
                            } else {
                                r
                            }
                        },
                    )
                };
                let expected = (0..3 * c.n()).fold(F128::ZERO, |sum, index| {
                    assert_eq!(weights.at(index), direct_weight(index));
                    sum + direct_weight(index) * field.embed(index as u128)
                });
                assert_eq!(identity_labels(&c, &weights, field.basis()), expected);
                for wire in 0..3 {
                    let q = weights.wire_query(&c, wire, &field.basis()[c.tag_bits() - bits..]);
                    let expected = (0..c.n()).fold(F128::ZERO, |sum, gate| {
                        let index = wire * c.n() + gate;
                        sum + direct_weight(index)
                            * field.embed(c.packed_value(&z, index) << (c.tag_bits() - bits))
                    });
                    assert_eq!(
                        evaluate(&z[wire * c.wire_words()..(wire + 1) * c.wire_words()], &q),
                        expected,
                        "raw source mapping b={bits}, wire={wire}"
                    );
                }
            }
        }
    }
}
