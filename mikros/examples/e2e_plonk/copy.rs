//! Copy reductions and explicit source-claim expansion for the PLONK fixture.
//! The verifier uses only public circuit shape, wiring and I/O. Every private
//! wire value and every permuted-label value is returned for PCS authentication.

use super::{
    fixture::{Circuit, Mode, bit, from_raw},
    gates::Claims,
    queries::*,
};
use flock_core::{
    challenger::Challenger,
    pcs::{functional::TensorClaim, ring_switch::claim_check},
    product_gkr::{self, BatchedGrinding, LiveMask, ProductGkrBatchedProof},
};
use flock_field::F128;
use mikros::extension_product;
use mikros_permutation::{
    field::SmallField,
    packed,
    permutation::{self, CommittedStatement},
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{mem::MaybeUninit, time::Instant};

struct CopyTrace {
    copy: &'static str,
    start: Option<Instant>,
}
impl CopyTrace {
    fn new(copy: &'static str) -> Self {
        let enabled = std::env::var_os("E2E_PIOP_TRACE")
            .is_some_and(|value| !value.is_empty() && value != "0");
        Self {
            copy,
            start: enabled.then(Instant::now),
        }
    }
    fn lap(&mut self, stage: &str) {
        if let Some(start) = self.start {
            let ms = start.elapsed().as_secs_f64() * 1000.0;
            eprintln!("e2e_piop_trace copy={} stage={stage} ms={ms:.6}", self.copy);
            self.start = Some(Instant::now());
        }
    }
    fn profile<T: Serialize>(&mut self, profile: &T) {
        if self.start.is_some() {
            eprintln!(
                "e2e_piop_trace copy={} tree_profile={}",
                self.copy,
                serde_json::to_string(profile).unwrap()
            );
            self.start = Some(Instant::now());
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SingleProof {
    pub permutation: packed::Proof,
    /// Side 0 is sigma, side 1 is identity. Values include block-prefix weights.
    pub wire_values: [[F128; 3]; 2],
    /// Wire-major, then low-to-high label bits: exactly 3*h entries.
    pub label_values: Vec<F128>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MultipleProof {
    pub product: extension_product::Proof,
    /// Side 0 is identity, side 1 is sigma. Each has its own terminal point.
    pub wire_values: [[F128; 3]; 2],
    pub label_values: [F128; 3],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlockProof {
    pub product: ProductGkrBatchedProof,
    /// Native Flock shares one terminal point, so these three values suffice.
    pub wire_values: [F128; 3],
    /// 3*h binary digit values in single mode, three label values in multiple.
    pub label_values: Vec<F128>,
}

fn check_circuit(c: &Circuit) -> Result<(), String> {
    if !(14..=30).contains(&c.log_gates)
        || (c.mode == Mode::Single && c.io != F128::ZERO && c.io != F128::ONE)
    {
        return Err("copy circuit shape or public I/O".into());
    }
    Ok(())
}

fn check_layout(c: &Circuit, log_z: usize, log_public: usize) -> Result<(), String> {
    check_circuit(c)?;
    let expected_z = (4 * c.wire_words()).max(1 << 15).ilog2() as usize;
    let expected_public = c.public_words().ilog2() as usize;
    if log_z != expected_z || log_public != expected_public {
        return Err("copy source commitment shape".into());
    }
    Ok(())
}

fn input_layout(c: &Circuit, z: &[F128], public: &[F128]) -> Result<(usize, usize), String> {
    if !z.len().is_power_of_two() || !public.len().is_power_of_two() {
        return Err("copy source lengths".into());
    }
    let logs = (z.len().ilog2() as usize, public.len().ilog2() as usize);
    check_layout(c, logs.0, logs.1)?;
    Ok(logs)
}

fn mask(c: &Circuit) -> LiveMask {
    LiveMask {
        nu: c.log_gates,
        counts: vec![c.n(), c.n(), c.n(), 1],
    }
}

fn point_weight(point: &[F128], index: usize) -> F128 {
    point.iter().enumerate().fold(F128::ONE, |value, (i, &x)| {
        value
            * if index >> i & 1 == 0 {
                F128::ONE + x
            } else {
                x
            }
    })
}

fn block_weight(point: &[F128], log_gates: usize, block: usize) -> F128 {
    point_weight(&point[log_gates..], block)
}

/// Restrict a length-4N bit functional to one N-bit wire. Keep the prefix
/// scale on the columns, including a zero prefix; never divide by that scale.
fn restrict_binary(full: &TensorClaim, block: usize) -> TensorClaim {
    let n = full.rows.len();
    let prefix = (0..2).fold(F128::ONE, |v, i| v * full.rows[n - 2 + i][(block >> i) & 1]);
    let mut query = TensorClaim {
        columns: full.columns.clone(),
        rows: full.rows[..n - 2].to_vec(),
        value: F128::ZERO,
    };
    for column in &mut query.columns {
        *column *= prefix;
    }
    query
}

/// Regroup a kappa-column packed-K query into a 128-column binary query.
/// Its rho is arbitrary: it is multiplied by low row factors, not reinterpreted
/// as an equality point. The resulting full query still contains both block bits.
fn packed_binary_query(c: &Circuit, claim: &packed::Claim) -> Result<TensorClaim, String> {
    let d = (c.h() + 1).next_power_of_two();
    let kappa = 128 / d;
    let s = kappa.ilog2() as usize;
    if claim.column_weights.len() != kappa || claim.row_point.len() != c.h() - s {
        return Err("packed copy terminal shape".into());
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

fn binary_point_query(c: &Circuit, point: &[F128]) -> Result<TensorClaim, String> {
    if point.len() != c.h() {
        return Err("binary copy point shape".into());
    }
    Ok(TensorClaim {
        columns: tensor(&point[..7]),
        rows: pairs(&point[7..]),
        value: F128::ZERO,
    })
}

fn binary_index_weight(full: &TensorClaim, index: usize) -> F128 {
    full.rows
        .iter()
        .enumerate()
        .fold(full.columns[index & 127], |v, (j, pair)| {
            v * pair[(index >> (7 + j)) & 1]
        })
}

/// Identity-label contribution on the three complete wire blocks. All tensor
/// row pairs here are ordinary MLE factors; only the 128 columns are arbitrary.
fn binary_identity_wires(c: &Circuit, full: &TensorClaim, basis: &[F128]) -> F128 {
    let column_mass = full.columns.iter().copied().fold(F128::ZERO, |a, b| a + b);
    let low_id = (0..128).fold(F128::ZERO, |sum, column| {
        let label = (0..7).fold(F128::ZERO, |v, bit| {
            if column >> bit & 1 == 1 {
                v + basis[bit]
            } else {
                v
            }
        });
        sum + full.columns[column] * label
    });
    let middle_id = (7..c.log_gates).fold(F128::ZERO, |v, j| v + basis[j] * full.rows[j - 7][1]);
    (0..3).fold(F128::ZERO, |sum, block| {
        let prefix = (0..2).fold(F128::ONE, |v, j| {
            v * full.rows[c.log_gates - 7 + j][(block >> j) & 1]
        });
        let high_id = (0..2).fold(F128::ZERO, |v, j| {
            if block >> j & 1 == 1 {
                v + basis[c.log_gates + j]
            } else {
                v
            }
        });
        sum + prefix * (low_id + column_mass * (middle_id + high_id))
    })
}

fn binary_wire_queries(full: &TensorClaim, values: &[F128; 3], log_z: usize) -> Vec<TensorClaim> {
    (0..3)
        .map(|wire| {
            let mut query = restrict_binary(full, wire);
            query.value = values[wire];
            lift(query, wire, log_z)
        })
        .collect()
}

fn binary_label_queries(
    c: &Circuit,
    full: &TensorClaim,
    values: &[F128],
    log_public: usize,
) -> Result<Vec<TensorClaim>, String> {
    if values.len() != 3 * c.h() {
        return Err("copy binary label message shape".into());
    }
    Ok((0..3)
        .flat_map(|wire| (0..c.h()).map(move |bit| (wire, bit)))
        .map(|(wire, bit)| {
            let mut query = restrict_binary(full, wire);
            query.value = values[wire * c.h() + bit];
            lift(query, 5 + wire * c.h() + bit, log_public)
        })
        .collect())
}

fn evaluate_binary_wires(c: &Circuit, z: &[F128], full: &TensorClaim) -> [F128; 3] {
    let m = c.wire_words();
    std::array::from_fn(|wire| evaluate(&z[wire * m..(wire + 1) * m], &restrict_binary(full, wire)))
}

fn evaluate_binary_labels(c: &Circuit, public: &[F128], full: &TensorClaim) -> Vec<F128> {
    let m = c.wire_words();
    (0..3 * c.h())
        .map(|i| {
            let block = 5 + i;
            evaluate(
                &public[block * m..(block + 1) * m],
                &restrict_binary(full, i / c.h()),
            )
        })
        .collect()
}

fn label_sum(values: &[F128], h: usize, basis: &[F128]) -> F128 {
    values
        .iter()
        .enumerate()
        .fold(F128::ZERO, |sum, (i, &value)| sum + value * basis[i % h])
}

/// Transparent rejection sampling for only the packed permutation's first z.
/// Every observed byte and every accepted challenge retains the old packed
/// transcript order; only an actual zero draw requests an additional squeeze.
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
    if c.mode != Mode::Single || z.len() != (4 * c.wire_words()).max(1 << 15) {
        return Err("single copy preparation shape".into());
    }
    let source = permutation::prepare(c.single_copy_source(z, 0), c.single_copy_source(z, 1))
        .map_err(|e| format!("single copy source: {e}"))?;
    packed::prepare(&source).map_err(|e| format!("single packed preparation: {e}"))
}

fn single_expand(
    c: &Circuit,
    log_z: usize,
    log_public: usize,
    proof: &SingleProof,
    terminals: &[packed::Claim; 2],
) -> Result<Claims, String> {
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
            if proof.label_values.len() != 3 * c.h() {
                return Err("single copy labels".into());
            }
            label_sum(&proof.label_values, c.h(), basis)
        } else {
            binary_identity_wires(c, &full[side], basis)
        };
        let io_index = 3 * c.n();
        let io_label = if side == 0 {
            c.sigma(io_index)
        } else {
            io_index
        };
        let io = binary_index_weight(&full[side], io_index)
            * (field.embed(io_label as u128) + basis[d - 1] * c.io);
        if labels + basis[d - 1] * wire + io != terminals[side].value {
            return Err(format!("single copy side {side} recombination"));
        }
        output.witness.extend(binary_wire_queries(
            &full[side],
            &proof.wire_values[side],
            log_z,
        ));
    }
    output.public = binary_label_queries(c, &full[0], &proof.label_values, log_public)?;
    Ok(output)
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
    if c.mode != Mode::Single || prepared.statement() != &statement {
        return Err("single prepared copy shape".into());
    }
    let mut trace = CopyTrace::new("mikros-single");
    ch.observe_label(b"mikros-plonk-single-copy-v1");
    let (permutation, terminals, tree_timings) = packed::prove(
        prepared,
        &mut NonzeroFirst {
            inner: ch,
            first: true,
        },
    )
    .map_err(|e| format!("single packed copy: {e}"))?;
    trace.lap("tree");
    trace.profile(&tree_timings);
    let full = [
        packed_binary_query(c, &terminals[0])?,
        packed_binary_query(c, &terminals[1])?,
    ];
    let wire_values = std::array::from_fn(|side| evaluate_binary_wires(c, z, &full[side]));
    trace.lap("wire_terminals");
    let label_values = evaluate_binary_labels(c, public, &full[0]);
    trace.lap("label_terminals");
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
    trace.lap("claim_construction");
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
    if c.mode != Mode::Single || proof.label_values.len() != 3 * c.h() {
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

fn ordinary_wire_query(c: &Circuit, point: &[F128], wire: usize) -> TensorClaim {
    scale(
        ordinary(&point[..c.log_gates], F128::ZERO),
        block_weight(point, c.log_gates, wire),
    )
}

fn ordinary_wire_queries(
    c: &Circuit,
    point: &[F128],
    values: &[F128; 3],
    log_z: usize,
) -> Vec<TensorClaim> {
    (0..3)
        .map(|wire| {
            let mut query = ordinary_wire_query(c, point, wire);
            query.value = values[wire];
            lift(query, wire, log_z)
        })
        .collect()
}

fn ordinary_label_query(c: &Circuit, point: &[F128], wire: usize) -> TensorClaim {
    scale(
        TensorClaim {
            columns: label_columns(5 + wire * c.h(), c.h()),
            rows: pairs(&point[..c.log_gates]),
            value: F128::ZERO,
        },
        block_weight(point, c.log_gates, wire),
    )
}

fn ordinary_label_queries(
    c: &Circuit,
    point: &[F128],
    values: &[F128; 3],
    log_public: usize,
) -> Vec<TensorClaim> {
    (0..3)
        .map(|wire| {
            let mut query = ordinary_label_query(c, point, wire);
            query.value = values[wire];
            lift(query, 0, log_public)
        })
        .collect()
}

fn evaluate_ordinary_wires(c: &Circuit, z: &[F128], point: &[F128], reference: bool) -> [F128; 3] {
    std::array::from_fn(|wire| {
        evaluate_ordinary(
            &z[wire * c.n()..(wire + 1) * c.n()],
            &point[..c.log_gates],
            block_weight(point, c.log_gates, wire),
            reference,
        )
    })
}

fn evaluate_paired_ordinary_wires(c: &Circuit, z: &[F128], points: [&[F128]; 2]) -> [[F128; 3]; 2] {
    let evaluation = PairedOrdinaryEvaluation::new(points.map(|point| &point[..c.log_gates]));
    let mut values = [[F128::ZERO; 3]; 2];
    for wire in 0..3 {
        let pair = evaluation.evaluate(
            &z[wire * c.n()..(wire + 1) * c.n()],
            points.map(|point| block_weight(point, c.log_gates, wire)),
        );
        for side in 0..2 {
            values[side][wire] = pair[side];
        }
    }
    values
}

fn paired_terminals_enabled() -> bool {
    match std::env::var("E2E_COPY_PAIRED_TERMINALS") {
        Ok(value) if value == "0" => false,
        Ok(value) if value == "1" => true,
        Err(std::env::VarError::NotPresent) => false,
        value => panic!("invalid E2E_COPY_PAIRED_TERMINALS: {value:?}; expected 0 or 1"),
    }
}

fn evaluate_ordinary_labels(
    c: &Circuit,
    public: &[F128],
    point: &[F128],
    reference: bool,
) -> [F128; 3] {
    if reference {
        return std::array::from_fn(|wire| {
            evaluate(&public[..c.n()], &ordinary_label_query(c, point, wire))
        });
    }
    // Each public label is a different column functional at the SAME row
    // point. Fold the source once, then retain each block's prefix scale.
    let folded = fold_columns(&public[..c.n()], &pairs(&point[..c.log_gates]));
    std::array::from_fn(|wire| {
        claim_check(&label_columns(5 + wire * c.h(), c.h()), &folded)
            * block_weight(point, c.log_gates, wire)
    })
}

fn multiple_start<C: Challenger>(c: &Circuit, ch: &mut C) -> (F128, F128) {
    ch.observe_label(b"mikros-plonk-multiple-copy-v1");
    ch.observe_bytes(&(c.log_gates as u64).to_le_bytes());
    let challenges = ch.sample_f128_vec(2);
    (challenges[0], challenges[1])
}

fn leaf_cache_enabled() -> bool {
    match std::env::var("E2E_COPY_LEAF_CACHE") {
        Ok(value) if value == "off" => false,
        Ok(value) if value == "on" => true,
        Err(std::env::VarError::NotPresent) => true,
        value => panic!("invalid E2E_COPY_LEAF_CACHE: {value:?}; expected on or off"),
    }
}

/// Cache only the challenge-dependent y*w for the three live wire blocks.
/// It is produced online and reused by both independently challenged trees.
/// Public I/O and the implicit one-padding need no full-size cache entries.
fn scaled_copy_words(words: &[F128], y: F128) -> Result<Vec<F128>, String> {
    const CHUNK: usize = 4096;
    let mut out = Vec::<F128>::new();
    out.try_reserve_exact(words.len())
        .map_err(|e| format!("copy leaf cache allocation: {e}"))?;
    let destination = &mut out.spare_capacity_mut()[..words.len()];
    let fill = |(block, dst): (usize, &mut [MaybeUninit<F128>])| {
        let source = &words[block * CHUNK..block * CHUNK + dst.len()];
        #[allow(unused_mut)]
        let mut i = 0;
        #[cfg(all(
            target_arch = "x86_64",
            target_feature = "avx512f",
            target_feature = "vpclmulqdq"
        ))]
        // SAFETY: the ISA is compile-time gated. Each iteration reads four
        // source words and initializes four disjoint destination elements.
        unsafe {
            use flock_field::gf2_128::x86_64::{f128x4_loadu, f128x4_set, ghash_mul_x4};
            use std::arch::x86_64::_mm512_storeu_si512;
            let scale = f128x4_set(y, y, y, y);
            while i + 4 <= source.len() {
                _mm512_storeu_si512(
                    dst.as_mut_ptr().add(i).cast(),
                    ghash_mul_x4(f128x4_loadu(source.as_ptr().add(i)), scale),
                );
                i += 4;
            }
        }
        for (dst, &word) in dst[i..].iter_mut().zip(&source[i..]) {
            dst.write(y * word);
        }
    };
    if words.len() < CHUNK {
        fill((0, destination));
    } else {
        destination.par_chunks_mut(CHUNK).enumerate().for_each(fill);
    }
    // SAFETY: every slot is initialized above, including partial chunks; Rayon
    // has joined. On panic the Vec length stays zero and no slot is read.
    unsafe {
        out.set_len(words.len());
    }
    Ok(out)
}

fn multiple_expand(
    c: &Circuit,
    log_z: usize,
    log_public: usize,
    proof: &MultipleProof,
    terminals: &[extension_product::LeafClaim; 2],
    y: F128,
    z: F128,
) -> Result<Claims, String> {
    if terminals.iter().any(|t| t.point.len() != c.h()) {
        return Err("multiple copy endpoint shape".into());
    }
    let live = mask(c);
    let identity_basis = product_gkr::s_id_basis(c.h());
    let mut output = Claims::default();
    for side in 0..2 {
        let point = &terminals[side].point;
        let io_weight = point_weight(point, 3 * c.n());
        let wire = proof.wire_values[side]
            .iter()
            .copied()
            .fold(F128::ZERO, |a, b| a + b)
            + io_weight * c.io;
        let label = if side == 0 {
            live.masked_id_eval(&identity_basis, point)
        } else {
            proof
                .label_values
                .iter()
                .copied()
                .fold(F128::ZERO, |a, b| a + b)
                + io_weight * from_raw(c.sigma(3 * c.n()) as u128)
        };
        let live_weight = live.live_eval(point);
        if z * live_weight + label + y * wire + F128::ONE + live_weight != terminals[side].value {
            return Err(format!("multiple copy side {side} recombination"));
        }
        output.witness.extend(ordinary_wire_queries(
            c,
            point,
            &proof.wire_values[side],
            log_z,
        ));
    }
    output.public = ordinary_label_queries(c, &terminals[1].point, &proof.label_values, log_public);
    Ok(output)
}

pub fn multiple_prove<C: Challenger>(
    c: &Circuit,
    witness: &[F128],
    public: &[F128],
    ch: &mut C,
) -> Result<(MultipleProof, Claims), String> {
    multiple_prove_with_evaluation(
        c,
        witness,
        public,
        CopyEvaluation::from_env(),
        leaf_cache_enabled(),
        paired_terminals_enabled(),
        ch,
    )
}

fn multiple_prove_with_evaluation<C: Challenger>(
    c: &Circuit,
    witness: &[F128],
    public: &[F128],
    evaluation: CopyEvaluation,
    leaf_cache: bool,
    paired_terminals: bool,
    ch: &mut C,
) -> Result<(MultipleProof, Claims), String> {
    let (log_z, log_public) = input_layout(c, witness, public)?;
    if c.mode != Mode::Multiple {
        return Err("multiple copy mode".into());
    }
    let mut trace = CopyTrace::new("mikros-multiple");
    let (y, z) = multiple_start(c, ch);
    trace.lap("fingerprint");
    let scaled = if leaf_cache {
        Some(scaled_copy_words(&witness[..3 * c.n()], y)?)
    } else {
        None
    };
    trace.lap(if leaf_cache {
        "leaf_cache_build"
    } else {
        "leaf_cache_disabled"
    });
    let leaves = |side: usize, i: usize| {
        if i >= c.live_len() {
            return F128::ONE;
        }
        let label = if side == 0 { i } else { c.sigma(i) };
        let product = match &scaled {
            Some(values) if i < 3 * c.n() => values[i],
            _ => y * if i == 3 * c.n() { c.io } else { witness[i] },
        };
        z + from_raw(label as u128) + product
    };
    let (product, terminals, tree_timings) = extension_product::prove_pair(c.h(), leaves, ch)
        .map_err(|e| format!("multiple copy tree: {e}"))?;
    trace.lap("tree");
    trace.profile(&tree_timings);
    // Both trees have completed; terminal evaluation uses the original witness
    // and therefore needs no cached leaf products. Release inside online time.
    drop(scaled);
    trace.lap("leaf_cache_release");
    let paired = paired_terminals && !evaluation.ordinary_reference;
    let wire_values = if paired {
        evaluate_paired_ordinary_wires(c, witness, [&terminals[0].point, &terminals[1].point])
    } else {
        std::array::from_fn(|side| {
            evaluate_ordinary_wires(
                c,
                witness,
                &terminals[side].point,
                evaluation.ordinary_reference,
            )
        })
    };
    trace.lap("wire_terminals");
    if trace.start.is_some() {
        eprintln!(
            "mikros_copy_wire_terminals paired_requested={paired_terminals} paired_active={paired}"
        );
    }
    let label_values =
        evaluate_ordinary_labels(c, public, &terminals[1].point, evaluation.label_reference);
    trace.lap("label_terminals");
    let proof = MultipleProof {
        product,
        wire_values,
        label_values,
    };
    for values in &proof.wire_values {
        ch.observe_f128_slice(values);
    }
    ch.observe_f128_slice(&proof.label_values);
    let claims = multiple_expand(c, log_z, log_public, &proof, &terminals, y, z)?;
    trace.lap("claim_construction");
    Ok((proof, claims))
}

pub fn multiple_verify<C: Challenger>(
    c: &Circuit,
    log_z: usize,
    log_public: usize,
    proof: &MultipleProof,
    ch: &mut C,
) -> Result<Claims, String> {
    check_layout(c, log_z, log_public)?;
    if c.mode != Mode::Multiple {
        return Err("multiple copy mode".into());
    }
    let (y, z) = multiple_start(c, ch);
    let terminals = extension_product::verify_pair(c.h(), &proof.product, ch)
        .map_err(|e| format!("multiple copy replay: {e}"))?;
    for values in &proof.wire_values {
        ch.observe_f128_slice(values);
    }
    ch.observe_f128_slice(&proof.label_values);
    multiple_expand(c, log_z, log_public, proof, &terminals, y, z)
}

/// Native Flock's single-execution input conversion, performed offline. Its
/// multiple-execution path borrows the original packed witness instead.
pub fn prepare_flock_single(c: &Circuit, z: &[F128]) -> Result<Vec<F128>, String> {
    check_circuit(c)?;
    if c.mode != Mode::Single || z.len() != (4 * c.wire_words()).max(1 << 15) {
        return Err("Flock single copy preparation".into());
    }
    Ok((0..c.copy_len())
        .into_par_iter()
        .map(|i| {
            if i >= c.live_len() {
                F128::ZERO
            } else if i == 3 * c.n() {
                c.io
            } else if bit(z, i) {
                F128::ONE
            } else {
                F128::ZERO
            }
        })
        .collect())
}

struct RestoreIo<'a> {
    words: &'a mut [F128],
    index: usize,
    previous: F128,
}
impl Drop for RestoreIo<'_> {
    fn drop(&mut self) {
        self.words[self.index] = self.previous;
    }
}

fn flock_expand(
    c: &Circuit,
    log_z: usize,
    log_public: usize,
    proof: &FlockProof,
    terminal: &product_gkr::ProductGkrBatchedClaim,
) -> Result<Claims, String> {
    if terminal.rho.len() != c.h() || terminal.f_eval != terminal.g_eval {
        return Err("Flock copy shared source".into());
    }
    let io_weight = point_weight(&terminal.rho, 3 * c.n());
    if proof
        .wire_values
        .iter()
        .copied()
        .fold(F128::ZERO, |a, b| a + b)
        + io_weight * c.io
        != terminal.f_eval
    {
        return Err("Flock copy wire recombination".into());
    }
    let (witness, public, label) = if c.mode == Mode::Single {
        let full = binary_point_query(c, &terminal.rho)?;
        let label = label_sum(&proof.label_values, c.h(), &product_gkr::s_id_basis(c.h()));
        (
            binary_wire_queries(&full, &proof.wire_values, log_z),
            binary_label_queries(c, &full, &proof.label_values, log_public)?,
            label,
        )
    } else {
        let values: [F128; 3] = proof
            .label_values
            .as_slice()
            .try_into()
            .map_err(|_| "Flock copy label shape")?;
        (
            ordinary_wire_queries(c, &terminal.rho, &proof.wire_values, log_z),
            ordinary_label_queries(c, &terminal.rho, &values, log_public),
            values.into_iter().fold(F128::ZERO, |a, b| a + b),
        )
    };
    if label + io_weight * from_raw(c.sigma(3 * c.n()) as u128) != terminal.s_sigma_eval {
        return Err("Flock masked sigma recombination".into());
    }
    Ok(Claims { witness, public })
}

pub fn flock_prove<C: Challenger>(
    c: &Circuit,
    z: &mut [F128],
    public: &[F128],
    single_values: Option<&[F128]>,
    sigma: &[usize],
    grinding: BatchedGrinding,
    ch: &mut C,
) -> Result<(FlockProof, Claims), String> {
    flock_prove_with_evaluation(
        c,
        z,
        public,
        single_values,
        sigma,
        grinding,
        CopyEvaluation::from_env(),
        ch,
    )
}

fn flock_prove_with_evaluation<C: Challenger>(
    c: &Circuit,
    z: &mut [F128],
    public: &[F128],
    single_values: Option<&[F128]>,
    sigma: &[usize],
    grinding: BatchedGrinding,
    evaluation: CopyEvaluation,
    ch: &mut C,
) -> Result<(FlockProof, Claims), String> {
    let (log_z, log_public) = input_layout(c, z, public)?;
    if sigma.len() != c.copy_len() {
        return Err("Flock sigma length".into());
    }
    let mut trace = CopyTrace::new(if c.mode == Mode::Single {
        "flock-single"
    } else {
        "flock-multiple"
    });
    ch.observe_label(b"flock-plonk-copy-native-v1");
    let live = mask(c);
    let (product, terminal) = if c.mode == Mode::Single {
        let values = single_values.ok_or("missing Flock single offline conversion")?;
        if values.len() != c.copy_len() {
            return Err("Flock single conversion length".into());
        }
        product_gkr::prove_batched_bounded_with_grinding(values, values, sigma, &live, grinding, ch)
    } else {
        let index = 3 * c.n();
        if z[index] != F128::ZERO {
            return Err("committed witness I/O padding must be zero".into());
        }
        let previous = z[index];
        z[index] = c.io;
        let guard = RestoreIo {
            words: &mut *z,
            index,
            previous,
        };
        let result = product_gkr::prove_batched_bounded_with_grinding(
            &guard.words[..c.copy_len()],
            &guard.words[..c.copy_len()],
            sigma,
            &live,
            grinding,
            ch,
        );
        drop(guard);
        result
    };
    // The native FLOCK product entry samples its fingerprint internally, so
    // this stage includes that work; Mikros's separate fingerprint is labeled.
    trace.lap("tree_including_fingerprint");
    let binary = if c.mode == Mode::Single {
        Some(binary_point_query(c, &terminal.rho)?)
    } else {
        None
    };
    let wire_values = if let Some(full) = &binary {
        evaluate_binary_wires(c, z, full)
    } else {
        evaluate_ordinary_wires(c, z, &terminal.rho, evaluation.ordinary_reference)
    };
    trace.lap("wire_terminals");
    let label_values = if let Some(full) = &binary {
        evaluate_binary_labels(c, public, full)
    } else {
        evaluate_ordinary_labels(c, public, &terminal.rho, evaluation.label_reference).to_vec()
    };
    trace.lap("label_terminals");
    let proof = FlockProof {
        product,
        wire_values,
        label_values,
    };
    ch.observe_f128_slice(&proof.wire_values);
    ch.observe_f128_slice(&proof.label_values);
    let claims = flock_expand(c, log_z, log_public, &proof, &terminal)?;
    trace.lap("claim_construction");
    Ok((proof, claims))
}

pub fn flock_verify<C: Challenger>(
    c: &Circuit,
    log_z: usize,
    log_public: usize,
    proof: &FlockProof,
    grinding: BatchedGrinding,
    ch: &mut C,
) -> Result<Claims, String> {
    check_layout(c, log_z, log_public)?;
    if proof.label_values.len() != if c.mode == Mode::Single { 3 * c.h() } else { 3 } {
        return Err("Flock copy label shape".into());
    }
    ch.observe_label(b"flock-plonk-copy-native-v1");
    // Defer the masked sigma evaluation to the SAME public commitment as the
    // Mikros path. The native verifier's algebra still checks it in both leaf
    // identities; flock_expand authenticates its complete public decomposition.
    let terminal = product_gkr::verify_batched_with_grinding(
        c.h(),
        &proof.product,
        Some(&mask(c)),
        grinding,
        ch,
    )
    .map_err(|e| format!("Flock native copy replay: {e:?}"))?;
    ch.observe_f128_slice(&proof.wire_values);
    ch.observe_f128_slice(&proof.label_values);
    flock_expand(c, log_z, log_public, proof, &terminal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::challenger::FsChallenger;
    use flock_field::test_rng::Rng;

    fn circuit(mode: Mode) -> Circuit {
        (1..100)
            .map(|seed| Circuit::new(14, mode, seed))
            .find(|c| c.io != F128::ZERO)
            .unwrap()
    }
    fn assert_claims(z: &[F128], public: &[F128], claims: &Claims) {
        for q in &claims.witness {
            assert_eq!(evaluate(z, q), q.value);
        }
        for q in &claims.public {
            assert_eq!(evaluate(public, q), q.value);
        }
    }
    fn ch() -> FsChallenger {
        FsChallenger::new(b"plonk-copy-tests-v1")
    }

    #[test]
    fn shared_public_fold_and_ordinary_dot_preserve_zero_prefixes() {
        let c = circuit(Mode::Multiple);
        let z = c.witness();
        let public = c.public();
        let mut rng = Rng::new(0x50A_AED26);
        let mut point = rng.f128_vec(c.h());
        for prefix in [
            None,
            Some([F128::ZERO; 2]),
            Some([F128::ONE, F128::ZERO]),
            Some([F128::ZERO, F128::ONE]),
        ] {
            if let Some(p) = prefix {
                point[c.log_gates..].copy_from_slice(&p);
            }
            assert_eq!(
                evaluate_ordinary_wires(&c, &z, &point, false),
                evaluate_ordinary_wires(&c, &z, &point, true)
            );
            assert_eq!(
                evaluate_ordinary_labels(&c, &public, &point, false),
                evaluate_ordinary_labels(&c, &public, &point, true)
            );
        }
    }

    #[test]
    fn scaled_copy_cache_matches_scalar_at_simd_and_chunk_boundaries() {
        let mut rng = Rng::new(0xCA_C4E26);
        for len in [0, 1, 3, 4, 5, 4095, 4096, 4097, 8193] {
            let words = rng.f128_vec(len);
            for y in [F128::ZERO, F128::ONE, rng.f128()] {
                let expected: Vec<_> = words.iter().map(|&word| y * word).collect();
                assert_eq!(scaled_copy_words(&words, y).unwrap(), expected);
            }
        }
    }

    #[test]
    fn paired_copy_terminals_keep_independent_points_and_zero_block_prefixes() {
        let c = circuit(Mode::Multiple);
        let z = c.witness();
        let mut rng = Rng::new(0xC092_0A26);
        for mut points in [
            [rng.f128_vec(c.h()), rng.f128_vec(c.h())],
            [vec![F128::ZERO; c.h()], vec![F128::ONE; c.h()]],
            [vec![F128::ONE; c.h()], vec![F128::ONE; c.h()]],
        ] {
            for prefix in [None, Some([F128::ZERO; 2]), Some([F128::ONE, F128::ZERO])] {
                if let Some(prefix) = prefix {
                    points[0][c.log_gates..].copy_from_slice(&prefix);
                }
                let actual =
                    evaluate_paired_ordinary_wires(&c, &z, points.each_ref().map(Vec::as_slice));
                for side in 0..2 {
                    assert_eq!(
                        actual[side],
                        evaluate_ordinary_wires(&c, &z, &points[side], true)
                    );
                }
            }
        }
    }

    #[test]
    fn both_copy_systems_keep_proof_claims_and_transcript_under_all_evaluation_modes() {
        let c = circuit(Mode::Multiple);
        let mut z = c.witness();
        let public = c.public();
        let sigma = c.sigma_table();
        let log_z = z.len().ilog2() as usize;
        let log_public = public.len().ilog2() as usize;
        for flock in [false, true] {
            let mut baseline = None;
            for (ordinary_reference, label_reference, leaf_cache, paired_terminals) in [
                (true, true, false, false),
                (false, true, false, false),
                (true, false, false, false),
                (false, false, false, false),
                (true, true, true, false),
                (false, true, true, false),
                (true, false, true, false),
                (false, false, true, false),
                (false, false, false, true),
                (false, false, true, true),
                (true, false, false, true),
            ] {
                if flock && (leaf_cache || paired_terminals) {
                    continue;
                }
                let evaluation = CopyEvaluation {
                    ordinary_reference,
                    label_reference,
                };
                let mut pc = ch();
                let (encoded, claims, verifier_claims, mut vc) = if flock {
                    let (proof, claims) = flock_prove_with_evaluation(
                        &c,
                        &mut z,
                        &public,
                        None,
                        &sigma,
                        BatchedGrinding::disabled(),
                        evaluation,
                        &mut pc,
                    )
                    .unwrap();
                    let mut vc = ch();
                    let verifier_claims = flock_verify(
                        &c,
                        log_z,
                        log_public,
                        &proof,
                        BatchedGrinding::disabled(),
                        &mut vc,
                    )
                    .unwrap();
                    (
                        bincode::serialize(&proof).unwrap(),
                        claims,
                        verifier_claims,
                        vc,
                    )
                } else {
                    let (proof, claims) = multiple_prove_with_evaluation(
                        &c,
                        &z,
                        &public,
                        evaluation,
                        leaf_cache,
                        paired_terminals,
                        &mut pc,
                    )
                    .unwrap();
                    let mut vc = ch();
                    let verifier_claims =
                        multiple_verify(&c, log_z, log_public, &proof, &mut vc).unwrap();
                    (
                        bincode::serialize(&proof).unwrap(),
                        claims,
                        verifier_claims,
                        vc,
                    )
                };
                assert_eq!(claims.witness, verifier_claims.witness);
                assert_eq!(claims.public, verifier_claims.public);
                assert_claims(&z, &public, &claims);
                let next = pc.sample_f128();
                assert_eq!(next, vc.sample_f128());
                let state = (encoded, claims.witness, claims.public, next);
                if let Some(expected) = &baseline {
                    assert_eq!(expected, &state);
                } else {
                    baseline = Some(state);
                }
            }
        }
    }

    #[test]
    fn arbitrary_packed_columns_regroup_without_normalization() {
        let c = circuit(Mode::Single);
        let z = c.witness();
        let public = c.public();
        let d = (c.h() + 1).next_power_of_two();
        let kappa = 128 / d;
        let field = SmallField::for_width(d).unwrap();
        let mut rng = Rng::new(0xC09_2E62);
        for case in 0..4 {
            let mut point = rng.f128_vec(c.h() - kappa.ilog2() as usize);
            let mut columns = rng.f128_vec(kappa);
            if case == 1 {
                columns.fill(F128::ZERO);
            }
            if case == 2 {
                columns.fill(F128::ONE);
            }
            if case == 3 {
                *point.last_mut().unwrap() = F128::ONE;
            }
            let mut terminals: [packed::Claim; 2] = std::array::from_fn(|_| packed::Claim {
                row_point: point.clone(),
                column_weights: columns.clone(),
                value: F128::ZERO,
            });
            let full = packed_binary_query(&c, &terminals[0]).unwrap();
            for side in 0..2 {
                let source = c.single_copy_source(&z, side);
                terminals[side].value = (0..c.copy_len()).fold(F128::ZERO, |v, i| {
                    let w = columns[i & (kappa - 1)] * point_weight(&point, i / kappa);
                    assert_eq!(w, binary_index_weight(&full, i));
                    v + w * field.embed(source.row(i))
                });
            }
            let empty = packed::Proof {
                roots: [F128::ZERO; 2],
                extension: std::array::from_fn(|_| permutation::TreeProof { layers: vec![] }),
                packed_layers: vec![],
            };
            let proof = SingleProof {
                permutation: empty,
                wire_values: [evaluate_binary_wires(&c, &z, &full); 2],
                label_values: evaluate_binary_labels(&c, &public, &full),
            };
            let claims = single_expand(
                &c,
                z.len().ilog2() as usize,
                public.len().ilog2() as usize,
                &proof,
                &terminals,
            )
            .unwrap();
            assert_claims(&z, &public, &claims);
        }
    }

    #[test]
    fn single_copy_and_native_flock_authenticate_nonzero_io_and_all_sources() {
        let c = circuit(Mode::Single);
        let mut z = c.witness();
        let public = c.public();
        let logs = (z.len().ilog2() as usize, public.len().ilog2() as usize);
        let prepared = prepare_single(&c, &z).unwrap();
        let mut prover = ch();
        let (proof, _) = single_prove(&c, &z, &public, &prepared, &mut prover).unwrap();
        let mut verifier = ch();
        let claims = single_verify(&c, logs.0, logs.1, &proof, &mut verifier).unwrap();
        assert_eq!(claims.witness.len(), 6);
        assert_eq!(claims.public.len(), 3 * c.h());
        assert_claims(&z, &public, &claims);
        assert_eq!(prover.sample_f128(), verifier.sample_f128());
        let mut bad = proof.clone();
        bad.label_values[0] += F128::ONE;
        assert!(single_verify(&c, logs.0, logs.1, &bad, &mut ch()).is_err());
        let mut changed_io = c.clone();
        changed_io.io += F128::ONE;
        assert!(single_verify(&changed_io, logs.0, logs.1, &proof, &mut ch()).is_err());
        let values = prepare_flock_single(&c, &z).unwrap();
        let sigma = c.sigma_table();
        let mut prover = ch();
        let (proof, _) = flock_prove(
            &c,
            &mut z,
            &public,
            Some(&values),
            &sigma,
            BatchedGrinding::disabled(),
            &mut prover,
        )
        .unwrap();
        let mut verifier = ch();
        let claims = flock_verify(
            &c,
            logs.0,
            logs.1,
            &proof,
            BatchedGrinding::disabled(),
            &mut verifier,
        )
        .unwrap();
        assert_eq!(claims.witness.len(), 3);
        assert_eq!(claims.public.len(), 3 * c.h());
        assert_claims(&z, &public, &claims);
        assert_eq!(prover.sample_f128(), verifier.sample_f128());
    }

    #[test]
    fn multiple_copy_and_native_flock_preserve_public_io_and_witness_padding() {
        let c = circuit(Mode::Multiple);
        let mut z = c.witness();
        let public = c.public();
        let logs = (z.len().ilog2() as usize, public.len().ilog2() as usize);
        let mut prover = ch();
        let (proof, _) = multiple_prove(&c, &z, &public, &mut prover).unwrap();
        let mut verifier = ch();
        let claims = multiple_verify(&c, logs.0, logs.1, &proof, &mut verifier).unwrap();
        assert_eq!(claims.witness.len(), 6);
        assert_eq!(claims.public.len(), 3);
        assert_claims(&z, &public, &claims);
        assert_eq!(prover.sample_f128(), verifier.sample_f128());
        let mut bad = proof.clone();
        bad.wire_values[0][0] += F128::ONE;
        assert!(multiple_verify(&c, logs.0, logs.1, &bad, &mut ch()).is_err());
        let before = blake3::hash(super::super::fixture::bytes(&z));
        let sigma = c.sigma_table();
        let mut prover = ch();
        let (proof, _) = flock_prove(
            &c,
            &mut z,
            &public,
            None,
            &sigma,
            BatchedGrinding::disabled(),
            &mut prover,
        )
        .unwrap();
        assert_eq!(z[3 * c.n()], F128::ZERO);
        assert_eq!(blake3::hash(super::super::fixture::bytes(&z)), before);
        let mut verifier = ch();
        let claims = flock_verify(
            &c,
            logs.0,
            logs.1,
            &proof,
            BatchedGrinding::disabled(),
            &mut verifier,
        )
        .unwrap();
        assert_claims(&z, &public, &claims);
        assert_eq!(prover.sample_f128(), verifier.sample_f128());
        let mut bad = proof.clone();
        bad.label_values[1] += F128::ONE;
        assert!(
            flock_verify(
                &c,
                logs.0,
                logs.1,
                &bad,
                BatchedGrinding::disabled(),
                &mut ch()
            )
            .is_err()
        );
    }

    #[test]
    fn io_guard_restores_on_unwind() {
        let mut words = vec![F128::ZERO; 4];
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            words[3] = F128::ONE;
            let _guard = RestoreIo {
                words: &mut words,
                index: 3,
                previous: F128::ZERO,
            };
            panic!("test unwind");
        }));
        assert!(result.is_err());
        assert_eq!(words[3], F128::ZERO);
    }

    #[test]
    fn nonzero_filter_applies_only_to_the_first_packed_challenge() {
        struct Script {
            values: std::collections::VecDeque<F128>,
        }
        impl Challenger for Script {
            fn observe_f128(&mut self, _: F128) {}
            fn sample_f128(&mut self) -> F128 {
                self.values.pop_front().unwrap()
            }
            fn fork_from_seed(&self, _: [F128; 2], _: &'static [u8]) -> Self {
                panic!("sequential test")
            }
        }
        let mut source = Script {
            values: [F128::ZERO, F128::ZERO, F128::ONE, F128::ZERO].into(),
        };
        let mut filtered = NonzeroFirst {
            inner: &mut source,
            first: true,
        };
        assert_eq!(filtered.sample_f128(), F128::ONE);
        assert_eq!(filtered.sample_f128(), F128::ZERO);
        assert!(source.values.is_empty());
    }
}
