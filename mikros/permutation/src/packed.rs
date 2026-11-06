//! Packed polynomial-tree permutation reduction from the revised manuscript.
//!
//! The public index convention is A[row, column] = source[kappa * row + column].
//! We pair adjacent rows, rather than manuscript upper/lower halves. These are
//! fixed index permutations; source values and their field embedding are intact.
//! Coefficient construction is challenge-independent, but depends on source
//! values and is charged online by copy-proof callers. CopyPreparationPlan
//! moves only fixed public label geometry and linear tables to setup. Every
//! challenge-dependent contraction, aggregation and proof message stays online.

use crate::{
    Error,
    bits::{PackedBits, decode_eight},
    field::{FastField, PolynomialEvaluation, SmallField, evaluate_linear, linear_tables},
    permutation::{CommittedStatement, PreparedPermutation},
    product_tree::Tree,
    profile::{Timings, elapsed},
    sumcheck::{LayerProof, TreeProof},
};
use flock_core::zerocheck::univariate_skip::{SplitEqGhash, build_eq};
use flock_field::F128;
use flock_transcript::challenger::Challenger;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    sync::{Arc, OnceLock},
    time::Instant,
};

mod aggregate_gfni;
mod aggregate_kernel;
mod aggregate_stream32;
mod aggregate_stream64;
mod aggregate_tower;
mod copy_preparation;
mod domain;
mod projection_kernel;
mod row_preparation;
pub use copy_preparation::{CopyPreparationPlan, CopyPreparationTimings};
use domain::Domain;
pub use row_preparation::prepare_bounded_from_fn;

/// A deferred source query after removal of the public grand-product shift.
/// Rows use low-bit-first MLE coordinates; columns need not be an MLE tensor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claim {
    pub row_point: Vec<F128>,
    pub column_weights: Vec<F128>,
    pub value: F128,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackedLayerProof {
    pub columns: Vec<F128>,
    /// Coefficients in increasing monomial order, exactly kappa - 1 entries.
    pub quotient: Vec<F128>,
    pub sumcheck: LayerProof,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    pub roots: [F128; 2],
    pub extension: [TreeProof; 2],
    /// Descending polynomial-tree levels, with both independently checked sides.
    pub packed_layers: Vec<[PackedLayerProof; 2]>,
}

impl Proof {
    pub fn field_elements(&self) -> usize {
        2 + self
            .extension
            .iter()
            .flat_map(|p| &p.layers)
            .map(|p| 3 * p.rounds.len() + 2)
            .sum::<usize>()
            + self
                .packed_layers
                .iter()
                .flatten()
                .map(|p| p.columns.len() + p.quotient.len() + 3 * p.sumcheck.rounds.len() + 2)
                .sum::<usize>()
    }
}

/// Owned, immutable, challenge-independent preparation for both input oracles.
pub struct Prepared {
    statement: CommittedStatement,
    kappa: usize,
    domain: Arc<Domain>,
    trees: [PreparedTree; 2],
}

/// Consumed by the bounded prover. Off-domain sample polynomials are produced
/// only when their messages are needed, and coefficient layers are released
/// after their last use. This changes storage lifetime, not proof messages.
pub struct BoundedPrepared(Prepared);

impl BoundedPrepared {
    pub fn statement(&self) -> &CommittedStatement {
        self.0.statement()
    }
    pub fn resident_bytes(&self) -> usize {
        self.0.resident_bytes()
    }
}

impl Prepared {
    pub fn statement(&self) -> &CommittedStatement {
        &self.statement
    }
    pub fn kappa(&self) -> usize {
        self.kappa
    }
    /// Owned coefficient capacities, including precomputed off-domain products.
    /// Shared public field/domain tables and original source storage are excluded.
    pub fn resident_bytes(&self) -> usize {
        self.trees
            .iter()
            .map(|t| {
                t.layers
                    .iter()
                    .chain(&t.samples)
                    .map(|p| p.bytes.capacity())
                    .sum::<usize>()
            })
            .sum()
    }
}

struct PreparedTree {
    layers: Vec<Polys>,
    samples: Vec<Polys>,
}

/// Fixed-width, monic polynomials. The leading one is implicit; all other
/// coefficients use the actual field width, including packed single-bit fields.
#[derive(Clone)]
struct Polys {
    count: usize,
    degree: usize,
    bits: usize,
    stride: usize,
    bytes: Vec<u8>,
}

pub fn prepare(source: &PreparedPermutation) -> Result<Prepared, Error> {
    let reference = std::env::var_os("MIKROS_COPY_PREP_REFERENCE").is_some_and(|v| v == "1");
    prepare_impl(source, reference)
}

/// Witness-dependent preparation for a one-shot proof with bounded storage.
/// This and the deferred sample construction both belong in online time.
pub fn prepare_bounded(source: &PreparedPermutation) -> Result<BoundedPrepared, Error> {
    let (kappa, _) = shape(source.statement())?;
    let domain = domain(source.statement().width)?;
    let reference = std::env::var_os("MIKROS_COPY_PREP_REFERENCE").is_some_and(|v| v == "1");
    let trees = [
        prepare_tree_impl(source.left(), kappa, &domain, reference, false),
        prepare_tree_impl(source.right(), kappa, &domain, reference, false),
    ];
    Ok(BoundedPrepared(Prepared {
        statement: *source.statement(),
        kappa,
        domain,
        trees,
    }))
}

fn prepare_impl(source: &PreparedPermutation, reference: bool) -> Result<Prepared, Error> {
    let (kappa, _) = shape(source.statement())?;
    let domain = domain(source.statement().width)?;
    let trees = [
        prepare_tree(source.left(), kappa, &domain, reference),
        prepare_tree(source.right(), kappa, &domain, reference),
    ];
    Ok(Prepared {
        statement: *source.statement(),
        kappa,
        domain,
        trees,
    })
}

fn shape(statement: &CommittedStatement) -> Result<(usize, usize), Error> {
    let field = SmallField::for_width(statement.width)?;
    let kappa = 128 / field.bits;
    let len = crate::padded_len(statement.rows)?;
    if kappa < 2 || len < kappa * kappa {
        return Err(Error::Length);
    }
    Ok((kappa, len.ilog2() as usize))
}

fn domain(width: usize) -> Result<Arc<Domain>, Error> {
    static DOMAINS: [OnceLock<Arc<Domain>>; 7] = [const { OnceLock::new() }; 7];
    let base = FastField::for_width(width)?;
    if base.base.bits > 64 {
        return Err(Error::Width);
    }
    Ok(DOMAINS[base.base.bits.ilog2() as usize]
        .get_or_init(|| Arc::new(Domain::new(base)))
        .clone())
}

impl Polys {
    /// Convert eight bit-sliced source rows together into the initial monic
    /// linear polynomials. The common SWAR transpose replaces a separate
    /// width-bit scan for every row; padded rows remain canonical zero.
    fn leaves(source: &PackedBits, count: usize, bits: usize, reference: bool) -> Self {
        if reference {
            return Self::generate(count, 1, bits, |i, out| {
                out[0] = if i < source.rows() { source.row(i) } else { 0 };
            });
        }
        let stride = bits.div_ceil(8);
        let mut bytes = vec![0u8; (count * bits).div_ceil(8)];
        // Eight field elements occupy exactly `bits` bytes, even for the
        // 1-, 2- and 4-bit subfields. Each worker owns disjoint output bytes.
        bytes
            .par_chunks_mut(bits)
            .enumerate()
            .with_min_len(256)
            .for_each(|(group, dst)| {
                if group * 8 >= source.rows() {
                    return;
                }
                let values = decode_eight(source, group);
                if bits < 8 {
                    for (row, &value) in values.iter().enumerate().take(dst.len() * 8 / bits) {
                        let offset = row * bits;
                        dst[offset / 8] |= (value as u8) << (offset % 8);
                    }
                } else {
                    for (out, value) in dst.chunks_mut(stride).zip(values) {
                        out.copy_from_slice(&value.to_le_bytes()[..stride]);
                    }
                }
            });
        Self {
            count,
            degree: 1,
            bits,
            stride,
            bytes,
        }
    }

    fn generate<F>(count: usize, degree: usize, bits: usize, f: F) -> Self
    where
        F: Fn(usize, &mut [u128]) + Sync + Send,
    {
        Self::generate_prefix(count, count, degree, bits, f)
    }

    /// Materialize an active prefix while preserving the full zero-padded
    /// polynomial layout. Public geometry, never witness values, chooses it.
    fn generate_prefix<F>(count: usize, active: usize, degree: usize, bits: usize, f: F) -> Self
    where
        F: Fn(usize, &mut [u128]) + Sync + Send,
    {
        assert!(active <= count);
        let width = degree * bits;
        let stride = width.div_ceil(8);
        let mut bytes = vec![0u8; (count * width).div_ceil(8)];
        if width < 8 {
            let per_byte = 8 / width;
            bytes[..(active * width).div_ceil(8)]
                .par_iter_mut()
                .enumerate()
                .for_each_init(
                    || vec![0; degree],
                    |coeff, (block, dst)| {
                        let mut word = 0u8;
                        for r in 0..per_byte.min(active - block * per_byte) {
                            f(block * per_byte + r, coeff);
                            let value = coeff
                                .iter()
                                .enumerate()
                                .fold(0u8, |v, (j, &a)| v | ((a as u8) << (j * bits)));
                            word |= value << (r * width);
                        }
                        *dst = word;
                    },
                );
        } else {
            bytes[..active * stride]
                .par_chunks_mut(stride)
                .enumerate()
                .with_min_len(256)
                .for_each_init(
                    || vec![0; degree],
                    |coeff, (i, dst)| {
                        f(i, coeff);
                        dst.fill(0);
                        for (j, &a) in coeff.iter().enumerate() {
                            if bits < 8 {
                                dst[j * bits / 8] |= (a as u8) << ((j * bits) % 8);
                            } else {
                                let n = bits / 8;
                                dst[j * n..(j + 1) * n].copy_from_slice(&a.to_le_bytes()[..n]);
                            }
                        }
                    },
                );
        }
        Self {
            count,
            degree,
            bits,
            stride,
            bytes,
        }
    }
    #[inline]
    fn coefficient(&self, node: usize, j: usize) -> u128 {
        debug_assert!(node < self.count && j < self.degree);
        let bit = (node * self.degree + j) * self.bits;
        if self.bits < 8 {
            return ((self.bytes[bit / 8] >> (bit % 8)) & ((1u8 << self.bits) - 1)) as u128;
        }
        let mut raw = [0; 16];
        let n = self.bits / 8;
        raw[..n].copy_from_slice(&self.bytes[bit / 8..bit / 8 + n]);
        u128::from_le_bytes(raw)
    }
    #[inline]
    fn word(&self, node: usize) -> u128 {
        let width = self.degree * self.bits;
        assert!(width <= 128);
        if width < 8 {
            let bit = node * width;
            return ((self.bytes[bit / 8] >> (bit % 8)) & ((1u8 << width) - 1)) as u128;
        }
        let mut out = [0; 16];
        out[..self.stride]
            .copy_from_slice(&self.bytes[node * self.stride..(node + 1) * self.stride]);
        u128::from_le_bytes(out)
    }
}

fn prepare_tree(bits: &PackedBits, kappa: usize, domain: &Domain, reference: bool) -> PreparedTree {
    prepare_tree_impl(bits, kappa, domain, reference, true)
}

fn prepare_tree_impl(
    bits: &PackedBits,
    kappa: usize,
    domain: &Domain,
    reference: bool,
    cache_samples: bool,
) -> PreparedTree {
    let len = crate::padded_len(bits.rows()).unwrap();
    let mut layers = vec![Polys::leaves(bits, len, domain.base.base.bits, reference)];
    let mut samples = Vec::with_capacity(kappa.ilog2() as usize);
    for _ in 0..kappa.ilog2() {
        let prev = layers.last().unwrap();
        let next = prepare_parent_layer(prev, kappa, domain);
        if cache_samples {
            samples.push(prepare_parent_samples(prev, kappa, domain));
        }
        layers.push(next);
    }
    PreparedTree { layers, samples }
}

fn prepare_parent_layer(prev: &Polys, kappa: usize, domain: &Domain) -> Polys {
    prepare_parent_layer_prefix(prev, kappa, domain, prev.count)
}

/// Children are adjacent kappa-column rows. A partial first child activates
/// its columns; any part of the second child still fits that same prefix.
fn active_parent_nodes(active: usize, kappa: usize) -> usize {
    active / (2 * kappa) * kappa + (active % (2 * kappa)).min(kappa)
}

fn prepare_parent_layer_prefix(
    prev: &Polys,
    kappa: usize,
    domain: &Domain,
    active: usize,
) -> Polys {
    let d = prev.degree;
    let rows = prev.count / kappa / 2;
    Polys::generate_prefix(
        rows * kappa,
        active_parent_nodes(active, kappa),
        2 * d,
        prev.bits,
        |node, out| {
            let row = node / kappa;
            let column = node % kappa;
            let a = 2 * row * kappa + column;
            let b = a + kappa;
            if prev.bits == 32 && d <= 2 {
                let a0 = prev.coefficient(a, 0);
                let b0 = prev.coefficient(b, 0);
                let p0 = domain.base.multiply(a0, b0);
                out[0] = p0;
                if d == 1 {
                    out[1] = a0 ^ b0;
                } else {
                    let a1 = prev.coefficient(a, 1);
                    let b1 = prev.coefficient(b, 1);
                    let p1 = domain.base.multiply(a1, b1);
                    // Monic quadratics: three products replace four while
                    // preserving every coefficient, including padded X^2.
                    out[1] = domain.base.multiply(a0 ^ a1, b0 ^ b1) ^ p0 ^ p1;
                    out[2] = p1 ^ a0 ^ b0;
                    out[3] = a1 ^ b1;
                }
                return;
            }
            out.fill(0);
            for i in 0..d {
                let ai = prev.coefficient(a, i);
                out[i + d] ^= ai ^ prev.coefficient(b, i);
                for j in 0..d {
                    out[i + j] ^= domain.base.multiply(ai, prev.coefficient(b, j));
                }
            }
        },
    )
}

fn prepare_parent_samples(prev: &Polys, kappa: usize, domain: &Domain) -> Polys {
    prepare_parent_samples_prefix(prev, kappa, domain, prev.count)
}

fn prepare_parent_samples_prefix(
    prev: &Polys,
    kappa: usize,
    domain: &Domain,
    active: usize,
) -> Polys {
    let d = prev.degree;
    let rows = prev.count / kappa / 2;
    Polys::generate_prefix(
        rows * (kappa - 1),
        active.div_ceil(2 * kappa) * (kappa - 1),
        2 * d,
        domain.field.base.bits,
        |node, out| {
            let row = node / (kappa - 1);
            let sample = node % (kappa - 1);
            let mut a = [0u128; 64];
            let mut b = [0u128; 64];
            for j in 0..kappa {
                let weight = domain.off_weights[sample][j];
                for nu in 0..d {
                    a[nu] ^= domain.field.multiply(
                        weight,
                        domain.lift(prev.coefficient(2 * row * kappa + j, nu)),
                    );
                    b[nu] ^= domain.field.multiply(
                        weight,
                        domain.lift(prev.coefficient((2 * row + 1) * kappa + j, nu)),
                    );
                }
            }
            out.fill(0);
            for i in 0..d {
                out[i + d] ^= a[i] ^ b[i];
                for j in 0..d {
                    out[i + j] ^= domain.field.multiply(a[i], b[j]);
                }
            }
        },
    )
}

struct DenseTree {
    layers: Vec<Vec<F128>>,
    log_len: usize,
    recycle: bool,
}
const POOL_SLOT: usize = 2;

impl Drop for DenseTree {
    fn drop(&mut self) {
        for values in &mut self.layers {
            if self.recycle && !values.is_empty() {
                crate::tree_pool::give_f128(POOL_SLOT, std::mem::take(values));
            }
        }
    }
}
impl Tree for DenseTree {
    fn log_len(&self) -> usize {
        self.log_len
    }
    fn root(&self) -> F128 {
        self.layers.last().unwrap()[0]
    }
    fn layer(&self, level: usize) -> Cow<'_, [F128]> {
        Cow::Borrowed(&self.layers[level])
    }
    fn release_layer(&mut self, level: usize) {
        let values = std::mem::take(&mut self.layers[level]);
        if self.recycle {
            crate::tree_pool::give_f128(POOL_SLOT, values);
        }
    }
}

fn extension_tree(
    prepared: &PreparedTree,
    base: &FastField,
    z: F128,
    p: &mut Timings,
    recycle: bool,
) -> DenseTree {
    let start = Instant::now();
    let highest = prepared.layers.last().unwrap();
    let mut first = crate::tree_pool::take_f128(POOL_SLOT, highest.count);
    let mut projected = false;
    if highest.bits == 32 && highest.stride == 16 {
        let mut power = F128::ONE;
        let mut basis = Vec::with_capacity(128);
        for _ in 0..highest.degree {
            basis.extend(base.base.basis().iter().map(|&b| b * power));
            power *= z;
        }
        projected = projection_kernel::project_dense(
            &highest.bytes,
            highest.stride,
            &basis,
            power,
            &mut first,
        );
    }
    if !projected {
        let evaluator = PolynomialEvaluation::new(base.base, highest.degree * highest.bits, z);
        first
            .par_iter_mut()
            .enumerate()
            .with_min_len(1024)
            .for_each(|(i, dst)| {
                dst.write(evaluator.evaluate(highest.word(i)));
            });
    }
    // SAFETY: the parallel traversal initialized each logical output slot.
    let first = unsafe { crate::kernels::initialized_f128(first) };
    p.coefficient_eval_ms += elapsed(start);
    build_extension_tree(first, p, recycle)
}

fn build_extension_tree(first: Vec<F128>, p: &mut Timings, recycle: bool) -> DenseTree {
    let start = Instant::now();
    let log_len = first.len().ilog2() as usize;
    let mut layers = vec![first];
    while layers.last().unwrap().len() > 1 {
        let prev = layers.last().unwrap();
        let mut next = crate::tree_pool::take_f128(POOL_SLOT, prev.len() / 2);
        next.par_chunks_mut(4096)
            .enumerate()
            .for_each(|(block, dst)| crate::kernels::multiply_pairs(prev, block * 4096, dst));
        // SAFETY: all disjoint output chunks were initialized by multiply_pairs.
        layers.push(unsafe { crate::kernels::initialized_f128(next) });
    }
    p.extension_build_ms += elapsed(start);
    DenseTree {
        layers,
        log_len,
        recycle,
    }
}

/// Aggregate base/intermediate-field coefficients before evaluating at z.
fn aggregate(
    polys: &Polys,
    columns: usize,
    field: &FastField,
    point: &[F128],
    powers: &[F128],
) -> Vec<F128> {
    let rows = polys.count / columns;
    assert_eq!(rows, 1usize << point.len());
    let eq = SplitEqGhash::with_n_hi(point, point.len() / 2);
    let count = columns * polys.degree;
    let plan = aggregate_kernel::Plan::new(polys, columns, field, &eq.lo);
    let coefficients = (0..eq.hi.len())
        .into_par_iter()
        .map(|block| {
            plan.block(block * eq.lo.len())
                .into_iter()
                .map(|s| s * eq.hi[block])
                .collect::<Vec<_>>()
        })
        .reduce(
            || vec![F128::ZERO; count],
            |mut a, b| {
                for (x, y) in a.iter_mut().zip(b) {
                    *x += y;
                }
                a
            },
        );
    coefficients
        .chunks_exact(polys.degree)
        .map(|coeff| {
            coeff
                .iter()
                .zip(powers)
                .fold(powers[polys.degree], |sum, (&a, &z)| sum + a * z)
        })
        .collect()
}

fn gfni_k64_projection(input_bytes: usize) -> bool {
    if std::env::var_os("MIKROS_PERMUTATION_REFERENCE_PROJECTION").is_some_and(|value| value == "1")
    {
        return false;
    }
    // Amortize challenge-dependent matrix setup on large K64 inputs. The
    // projection kernel checks GFNI/AVX-512 support and retains its fallback.
    // Explicit 0/1 overrides remain available for differential measurements.
    std::env::var_os("MIKROS_PERMUTATION_GFNI_PROJECTION")
        .map_or(input_bytes >= 64 * 1024, |value| value == "1")
}

/// Fuse column contraction and polynomial evaluation into public byte-linear
/// tables. The online scan contains table lookups and XOR, with no per-node
/// extension multiplication. Each row's implicit leading coefficient is one.
fn packed_children(
    polys: &Polys,
    domain: &Domain,
    rho: &[F128],
    powers: &[F128],
) -> [Vec<F128>; 2] {
    let kappa = domain.kappa;
    let bases: Vec<Vec<F128>> = rho
        .iter()
        .map(|&weight| {
            let basis: Vec<_> = powers[..polys.degree]
                .iter()
                .flat_map(|&z| {
                    domain
                        .base
                        .base
                        .basis()
                        .iter()
                        .map(move |&b| b * z * weight)
                })
                .collect();
            basis
        })
        .collect();
    let rows = polys.count / kappa / 2;
    let mut a = crate::tree_pool::take_f128(POOL_SLOT, rows);
    let mut b = crate::tree_pool::take_f128(POOL_SLOT, rows);
    // K64/kappa=2 leaves have the same 16-byte physical row shape as
    // K32/kappa=4 leaves. The GFNI map depends on the supplied bit images,
    // not on the subfield used to represent those bits.
    let native = if (polys.bits == 32 && kappa == 4 && polys.degree <= 2)
        || (polys.bits == 64
            && kappa == 2
            && polys.degree == 1
            && gfni_k64_projection(polys.bytes.len()))
    {
        let basis: Vec<F128> = bases.iter().flatten().copied().collect();
        projection_kernel::project(
            &polys.bytes,
            kappa * polys.stride,
            &basis,
            powers[polys.degree],
            &mut a,
            &mut b,
        )
    } else {
        false
    };
    if !native {
        let tables: Vec<_> = bases.iter().map(|basis| linear_tables(basis)).collect();
        a.par_iter_mut()
            .zip(b.par_iter_mut())
            .enumerate()
            .with_min_len(1024)
            .for_each(|(row, (a, b))| {
                let mut av = powers[polys.degree];
                let mut bv = powers[polys.degree];
                for j in 0..kappa {
                    av += evaluate_linear(&tables[j], polys.word(2 * row * kappa + j));
                    bv += evaluate_linear(&tables[j], polys.word((2 * row + 1) * kappa + j));
                }
                a.write(av);
                b.write(bv);
            });
    }
    // SAFETY: native success or the fallback initializes every a/b slot.
    unsafe {
        [
            crate::kernels::initialized_f128(a),
            crate::kernels::initialized_f128(b),
        ]
    }
}

fn bind<C: Challenger>(statement: &CommittedStatement, c: &mut C) -> Result<(usize, usize), Error> {
    let dimensions = shape(statement)?;
    c.observe_label(b"mikros-packed-polynomial-permutation-lockstep-v1");
    c.observe_bytes(&(statement.rows as u64).to_le_bytes());
    c.observe_bytes(&(statement.width as u64).to_le_bytes());
    c.observe_bytes(&(dimensions.0 as u64).to_le_bytes());
    Ok(dimensions)
}
fn column_challenge<C: Challenger>(domain: &Domain, c: &mut C) -> F128 {
    loop {
        c.observe_label(b"mikros-packed-permutation-column-challenge-v1");
        let x = c.sample_f128();
        if !domain.is_node(x) {
            return x;
        }
    }
}
fn dot(a: &[F128], b: &[F128]) -> F128 {
    a.iter().zip(b).fold(F128::ZERO, |s, (&a, &b)| s + a * b)
}

pub fn prove<C: Challenger>(
    prepared: &Prepared,
    c: &mut C,
) -> Result<(Proof, [Claim; 2], Timings), Error> {
    prove_impl(PreparedStorage::Borrowed(prepared), c, false)
}

/// The same transcript as `prove`, consuming coefficient storage as soon as
/// it is no longer needed. Temporary trees and fold buffers bypass the pools.
pub fn prove_bounded<C: Challenger>(
    prepared: BoundedPrepared,
    c: &mut C,
) -> Result<(Proof, [Claim; 2], Timings), Error> {
    prove_impl(PreparedStorage::Owned(prepared.0), c, false)
}

/// Identical proof bytes and challenges, with a smaller peak for the 64-bit
/// tag field (kappa=2). Each side's highest coefficient layer is released after
/// its extension evaluations are written and before its upper tree is built.
/// After extension GKR, that layer is recomputed from the retained leaves only
/// for its column aggregate, then immediately dropped. This extra arithmetic
/// is online and is included in packed_aggregate_ms. Other widths retain the
/// ordinary bounded storage. K32 uses tiled coefficient/sample aggregation
/// without changing its tree layout or transcript.
pub fn prove_bounded_compact<C: Challenger>(
    prepared: BoundedPrepared,
    c: &mut C,
) -> Result<(Proof, [Claim; 2], Timings), Error> {
    prove_impl(PreparedStorage::Owned(prepared.0), c, true)
}

enum PreparedStorage<'a> {
    Borrowed(&'a Prepared),
    Owned(Prepared),
}
impl PreparedStorage<'_> {
    fn get(&self) -> &Prepared {
        match self {
            Self::Borrowed(p) => p,
            Self::Owned(p) => p,
        }
    }
    fn release_layer(&mut self, level: usize) {
        if let Self::Owned(p) = self {
            for tree in &mut p.trees {
                drop(std::mem::take(&mut tree.layers[level].bytes));
            }
        }
    }

    fn release_side_layer(&mut self, side: usize, level: usize) {
        if let Self::Owned(p) = self {
            drop(std::mem::take(&mut p.trees[side].layers[level].bytes));
        }
    }
}

/// K64's degree-two coefficient word and its extension evaluation both occupy
/// sixteen bytes. Their overlap is necessary only while evaluating this side,
/// never while building the rest of this side or the second extension tree.
fn extension_tree_compact(
    storage: &mut PreparedStorage<'_>,
    side: usize,
    z: F128,
    p: &mut Timings,
) -> DenseTree {
    let start = Instant::now();
    let prepared = storage.get();
    debug_assert_eq!(prepared.kappa, 2);
    let highest = &prepared.trees[side].layers[1];
    debug_assert_eq!((highest.bits, highest.degree, highest.stride), (64, 2, 16));
    let mut first = crate::tree_pool::take_f128(POOL_SLOT, highest.count);
    let projected = if gfni_k64_projection(highest.bytes.len()) {
        let base = prepared.domain.base.base.basis();
        let basis: Vec<_> = base
            .iter()
            .copied()
            .chain(base.iter().map(|&b| b * z))
            .collect();
        projection_kernel::project_dense(&highest.bytes, 16, &basis, z.square(), &mut first)
    } else {
        false
    };
    if !projected {
        let evaluator = PolynomialEvaluation::new(prepared.domain.base.base, 128, z);
        first
            .par_iter_mut()
            .enumerate()
            .with_min_len(1024)
            .for_each(|(i, out)| {
                out.write(evaluator.evaluate(highest.word(i)));
            });
    }
    // SAFETY: the disjoint traversal wrote every logical output element.
    let first = unsafe { crate::kernels::initialized_f128(first) };
    p.coefficient_eval_ms += elapsed(start);
    let start = Instant::now();
    storage.release_side_layer(side, 1);
    p.layer_release_ms += elapsed(start);
    build_extension_tree(first, p, false)
}

fn prove_impl<C: Challenger>(
    mut storage: PreparedStorage<'_>,
    c: &mut C,
    compact: bool,
) -> Result<(Proof, [Claim; 2], Timings), Error> {
    let bounded = matches!(&storage, PreparedStorage::Owned(_));
    let domain = Arc::clone(&storage.get().domain);
    let (kappa, log_len) = bind(&storage.get().statement, c)?;
    let s = kappa.ilog2() as usize;
    let z = c.sample_f128();
    let mut powers = Vec::with_capacity(kappa + 1);
    powers.push(F128::ONE);
    for i in 0..kappa {
        powers.push(powers[i] * z);
    }
    let mut timings = Timings::default();
    let stream32 = compact && bounded && kappa == 4 && domain.base.base.bits == 32;
    let compact = compact && bounded && kappa == 2;
    let (mut a, mut b) = if compact {
        // Keep this sequential: release the left coefficient layer before
        // allocating the right evaluation and upper extension layers.
        let a = extension_tree_compact(&mut storage, 0, z, &mut timings);
        let b = extension_tree_compact(&mut storage, 1, z, &mut timings);
        (a, b)
    } else {
        let [left, right] = storage.get().trees.each_ref();
        (
            extension_tree(left, domain.base, z, &mut timings, !bounded),
            extension_tree(right, domain.base, z, &mut timings, !bounded),
        )
    };
    let roots = [a.root(), b.root()];
    if roots[0] != roots[1] {
        return Err(Error::ProductMismatch);
    }
    c.observe_f128_slice(&roots);
    c.observe_label(b"packed-permutation-extension-tree-v1");
    let extension_gkr_start = Instant::now();
    let (extension, endpoints) = if bounded {
        crate::fast_sumcheck::prove_pair_bounded(&mut a, &mut b, c, &mut timings)
    } else {
        crate::fast_sumcheck::prove_pair(&mut a, &mut b, c, &mut timings, true, true)
    };
    drop(a);
    drop(b);
    timings.packed_extension_gkr_ms += elapsed(extension_gkr_start);
    debug_assert_eq!(endpoints[0].point, endpoints[1].point);
    debug_assert_eq!(endpoints[0].point.len(), log_len - s);
    let mut rho = build_eq(&endpoints[0].point[..s]);
    let mut point = endpoints[0].point[s..].to_vec();
    let mut values = endpoints.map(|q| q.value);
    let mut packed_layers = Vec::with_capacity(s);
    for level in (0..s).rev() {
        c.observe_label(b"packed-permutation-polynomial-layer-v1");
        c.observe_bytes(&(level as u64).to_le_bytes());
        let start = Instant::now();
        let reference_aggregate = std::env::var_os("MIKROS_PERMUTATION_REFERENCE_AGGREGATE")
            .is_some_and(|value| value == "1");
        let (columns, samples) = if stream32 && !reference_aggregate {
            let [(ca, sa), (cb, sb)] = storage.get().trees.each_ref().map(|tree| {
                aggregate_stream32::columns_and_samples(
                    &tree.layers[level],
                    Some(&tree.layers[level + 1]),
                    &domain,
                    &point,
                    &powers,
                )
            });
            ([ca, cb], [sa, sb])
        } else if compact && !reference_aggregate {
            // Recreate only a cache-sized coefficient tile and consume it
            // immediately. Both message components share the same leaf read.
            let [(ca, sa), (cb, sb)] = storage.get().trees.each_ref().map(|tree| {
                aggregate_stream64::columns_and_samples(
                    &tree.layers[level],
                    &domain,
                    &point,
                    &powers,
                )
            });
            ([ca, cb], [sa, sb])
        } else {
            let columns = storage.get().trees.each_ref().map(|tree| {
                if compact {
                    // Exactly the original coefficients, regenerated after both
                    // dense trees have been dropped. The map processes one side
                    // at a time, so these temporary parents never overlap.
                    debug_assert_eq!(level, 0);
                    debug_assert!(tree.layers[1].bytes.is_empty());
                    let parent = prepare_parent_layer(&tree.layers[0], kappa, &domain);
                    return aggregate(&parent, kappa, domain.base, &point, &powers);
                }
                aggregate(&tree.layers[level + 1], kappa, domain.base, &point, &powers)
            });
            let samples = storage.get().trees.each_ref().map(|tree| {
                if bounded {
                    // At most one side's off-domain coefficients is resident.
                    let samples = prepare_parent_samples(&tree.layers[level], kappa, &domain);
                    aggregate(&samples, kappa - 1, domain.field, &point, &powers)
                } else {
                    aggregate(
                        &tree.samples[level],
                        kappa - 1,
                        domain.field,
                        &point,
                        &powers,
                    )
                }
            });
            (columns, samples)
        };
        let quotients =
            std::array::from_fn::<_, 2, _>(|j| domain.quotient(&columns[j], &samples[j]));
        // Parent coefficients have now contributed their final message.
        storage.release_layer(level + 1);
        let aggregate_ms = elapsed(start);
        timings.messages_ms += aggregate_ms;
        timings.packed_aggregate_ms += aggregate_ms;
        for j in 0..2 {
            debug_assert_eq!(dot(&columns[j], &rho), values[j]);
            c.observe_f128_slice(&columns[j]);
            c.observe_f128_slice(&quotients[j]);
        }
        let x = column_challenge(&domain, c);
        rho = domain.packing(x);
        let psi = domain.vanishing(x);
        values = std::array::from_fn(|j| {
            dot(&columns[j], &rho) + psi * domain::evaluate_extension(&quotients[j], x)
        });
        let start = Instant::now();
        let children = storage
            .get()
            .trees
            .each_ref()
            .map(|tree| packed_children(&tree.layers[level], &domain, &rho, &powers));
        if level == 0 {
            storage.release_layer(0);
        }
        let projection_ms = elapsed(start);
        timings.layer_materialize_ms += projection_ms;
        timings.packed_projection_ms += projection_ms;
        let (layers, next_point, next_values) = if bounded {
            crate::fast_sumcheck::prove_weighted_pair_with_recycling(
                [&children[0][0], &children[1][0]],
                [&children[0][1], &children[1][1]],
                &point,
                values,
                c,
                &mut timings,
                false,
            )
        } else {
            crate::fast_sumcheck::prove_weighted_pair(
                [&children[0][0], &children[1][0]],
                [&children[0][1], &children[1][1]],
                &point,
                values,
                c,
                &mut timings,
            )
        };
        let release_start = Instant::now();
        for values in children.into_iter().flatten() {
            if !bounded {
                crate::tree_pool::give_f128(POOL_SLOT, values);
            }
        }
        timings.layer_release_ms += elapsed(release_start);
        point = next_point;
        values = next_values;
        let [ca, cb] = columns;
        let [qa, qb] = quotients;
        let [sa, sb] = layers;
        packed_layers.push([
            PackedLayerProof {
                columns: ca,
                quotient: qa,
                sumcheck: sa,
            },
            PackedLayerProof {
                columns: cb,
                quotient: qb,
                sumcheck: sb,
            },
        ]);
    }
    let claims = values.map(|v| Claim {
        row_point: point.clone(),
        column_weights: rho.clone(),
        value: v + z,
    });
    Ok((
        Proof {
            roots,
            extension,
            packed_layers,
        },
        claims,
        timings,
    ))
}

fn verify_weighted_pair<C: Challenger>(
    layers: [&LayerProof; 2],
    point: &[F128],
    mut values: [F128; 2],
    c: &mut C,
) -> Result<(Vec<F128>, [F128; 2]), Error> {
    if layers.iter().any(|p| p.rounds.len() != point.len()) {
        return Err(Error::ProofShape);
    }
    let mut q = Vec::with_capacity(point.len() + 1);
    for i in 0..point.len() {
        for layer in &layers {
            c.observe_f128_slice(&layer.rounds[i]);
        }
        let r = c.sample_f128();
        q.push(r);
        for j in 0..2 {
            let [g0, g2, g3] = layers[j].rounds[i];
            values[j] = g0 + r * (values[j] + g2 + g3 + r * (g2 + r * g3));
        }
    }
    let equality = point
        .iter()
        .zip(&q)
        .fold(F128::ONE, |v, (&x, &y)| v * (F128::ONE + x + y));
    for j in 0..2 {
        if values[j] != equality * layers[j].left * layers[j].right {
            return Err(Error::Sumcheck);
        }
    }
    for layer in &layers {
        c.observe_f128_slice(&[layer.left, layer.right]);
    }
    let mix = c.sample_f128();
    values = layers.map(|layer| layer.left + mix * (layer.left + layer.right));
    q.insert(0, mix);
    Ok((q, values))
}

/// Replay using public shape, proof and bound commitments only. Both returned
/// claims remain unauthenticated until the caller verifies their PCS openings.
pub fn verify<C: Challenger>(
    statement: &CommittedStatement,
    proof: &Proof,
    c: &mut C,
) -> Result<[Claim; 2], Error> {
    let (kappa, log_len) = bind(statement, c)?;
    let s = kappa.ilog2() as usize;
    let domain = domain(statement.width)?;
    if proof.packed_layers.len() != s
        || proof
            .packed_layers
            .iter()
            .flatten()
            .any(|p| p.columns.len() != kappa || p.quotient.len() != kappa - 1)
    {
        return Err(Error::ProofShape);
    }
    let z = c.sample_f128();
    if proof.roots[0] != proof.roots[1] {
        return Err(Error::ProductMismatch);
    }
    c.observe_f128_slice(&proof.roots);
    c.observe_label(b"packed-permutation-extension-tree-v1");
    let endpoints =
        crate::sumcheck::verify_pair(proof.extension.each_ref(), proof.roots, log_len - s, c)?;
    let mut rho = build_eq(&endpoints[0].point[..s]);
    let mut point = endpoints[0].point[s..].to_vec();
    let mut values = endpoints.map(|q| q.value);
    for (index, layers) in proof.packed_layers.iter().enumerate() {
        let level = s - 1 - index;
        c.observe_label(b"packed-permutation-polynomial-layer-v1");
        c.observe_bytes(&(level as u64).to_le_bytes());
        for j in 0..2 {
            if dot(&layers[j].columns, &rho) != values[j] {
                return Err(Error::Constraint);
            }
            c.observe_f128_slice(&layers[j].columns);
            c.observe_f128_slice(&layers[j].quotient);
        }
        let x = column_challenge(&domain, c);
        rho = domain.packing(x);
        let psi = domain.vanishing(x);
        values = std::array::from_fn(|j| {
            dot(&layers[j].columns, &rho) + psi * domain::evaluate_extension(&layers[j].quotient, x)
        });
        (point, values) = verify_weighted_pair(
            [&layers[0].sumcheck, &layers[1].sumcheck],
            &point,
            values,
            c,
        )?;
    }
    Ok(values.map(|v| Claim {
        row_point: point.clone(),
        column_weights: rho.clone(),
        value: v + z,
    }))
}

#[cfg(test)]
mod preparation_tests {
    use super::*;
    use crate::{claims::row_weight, permutation};
    use flock_field::test_rng::Rng;
    use flock_transcript::challenger::FsChallenger;

    fn values(rows: usize, width: usize, rng: &mut Rng) -> Vec<u128> {
        let mask = (1u128 << width) - 1;
        (0..rows).map(|_| (rng.next_u64() as u128) & mask).collect()
    }

    fn assert_polys_equal(a: &Polys, b: &Polys) {
        assert_eq!(
            (a.count, a.degree, a.bits, a.stride),
            (b.count, b.degree, b.bits, b.stride)
        );
        assert_eq!(a.bytes, b.bytes);
    }

    #[test]
    fn k32_monic_parent_products_match_independent_convolution() {
        let domain = Domain::new(FastField::for_width(32).unwrap());
        let mut rng = Rng::new(0x32_ca_7a);
        for degree in [1, 2] {
            for active in [0, 1, 7, 8, 17, 511, 512] {
                let coefficients: Vec<_> = (0..512 * degree)
                    .map(|i| {
                        if i / degree < active {
                            rng.next_u64() as u32 as u128
                        } else {
                            0
                        }
                    })
                    .collect();
                let children = Polys::generate(512, degree, 32, |i, out| {
                    out.copy_from_slice(&coefficients[i * degree..(i + 1) * degree]);
                });
                let actual = prepare_parent_layer_prefix(&children, 4, &domain, active);
                for node in 0..actual.count {
                    let a = 2 * (node / 4) * 4 + node % 4;
                    let b = a + 4;
                    let mut expected = vec![0u128; 2 * degree + 1];
                    for i in 0..=degree {
                        for j in 0..=degree {
                            let ai = if i == degree {
                                1
                            } else {
                                children.coefficient(a, i)
                            };
                            let bj = if j == degree {
                                1
                            } else {
                                children.coefficient(b, j)
                            };
                            expected[i + j] ^= domain.base.base.multiply(ai, bj);
                        }
                    }
                    assert_eq!(expected[2 * degree], 1);
                    for (i, &coefficient) in expected[..2 * degree].iter().enumerate() {
                        assert_eq!(
                            actual.coefficient(node, i),
                            coefficient,
                            "degree={degree}, active={active}, node={node}, i={i}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn bounded_preparation_preserves_proof_bytes_claims_and_transcript() {
        let mut rng = Rng::new(0xB0_0DED);
        for width in [8usize, 16, 32, 64] {
            let kappa = 128 / width;
            for rows in [kappa * kappa, kappa * kappa + 3, 1024] {
                let raw = values(rows, width, &mut rng);
                let mut permuted = raw.clone();
                permuted.reverse();
                let source = permutation::prepare(
                    PackedBits::from_rows(&raw, width).unwrap(),
                    PackedBits::from_rows(&permuted, width).unwrap(),
                )
                .unwrap();
                let cached = prepare(&source).unwrap();
                let bounded = prepare_bounded(&source).unwrap();
                let compact = prepare_bounded(&source).unwrap();
                assert_eq!(bounded.statement(), cached.statement());
                assert!(bounded.resident_bytes() < cached.resident_bytes());
                let mut cached_ch = FsChallenger::new(b"packed-bounded-lifetime");
                let mut bounded_ch = FsChallenger::new(b"packed-bounded-lifetime");
                let mut compact_ch = FsChallenger::new(b"packed-bounded-lifetime");
                let (expected, expected_claims, _) = prove(&cached, &mut cached_ch).unwrap();
                let (proof, claims, _) = prove_bounded(bounded, &mut bounded_ch).unwrap();
                let (compact_proof, compact_claims, _) =
                    prove_bounded_compact(compact, &mut compact_ch).unwrap();
                assert_eq!(
                    bincode::serialize(&proof).unwrap(),
                    bincode::serialize(&expected).unwrap()
                );
                assert_eq!(
                    bincode::serialize(&compact_proof).unwrap(),
                    bincode::serialize(&expected).unwrap()
                );
                assert_eq!(claims, expected_claims);
                assert_eq!(compact_claims, expected_claims);
                let mut verifier = FsChallenger::new(b"packed-bounded-lifetime");
                assert_eq!(
                    verify(source.statement(), &proof, &mut verifier).unwrap(),
                    claims
                );
                let next = bounded_ch.sample_f128();
                assert_eq!(next, cached_ch.sample_f128());
                assert_eq!(next, compact_ch.sample_f128());
                assert_eq!(next, verifier.sample_f128());
                for (bits, claim) in [source.left(), source.right()].into_iter().zip(&claims) {
                    let field = SmallField::for_width(width).unwrap();
                    let actual = (0..rows).fold(F128::ZERO, |sum, row| {
                        sum + field.embed(bits.row(row))
                            * claim.column_weights[row % kappa]
                            * row_weight(&claim.row_point, row / kappa)
                    });
                    assert_eq!(actual, claim.value);
                }
            }
        }
    }

    #[test]
    fn compact_k64_releases_each_parent_before_building_the_next_tree() {
        let mut rng = Rng::new(0xB0_64_0DED);
        // Width 33 exercises rounding into K64; 67 rows exercises zero padding.
        for (rows, width) in [(64, 64), (67, 33), (4096, 64)] {
            let raw = values(rows, width, &mut rng);
            let source = permutation::prepare(
                PackedBits::from_rows(&raw, width).unwrap(),
                PackedBits::from_rows(&raw.iter().copied().rev().collect::<Vec<_>>(), width)
                    .unwrap(),
            )
            .unwrap();
            let cached = prepare_bounded(&source).unwrap();
            let mut owned = PreparedStorage::Owned(prepare_bounded(&source).unwrap().0);
            let initial_bytes = owned.get().resident_bytes();
            let parent_bytes = owned.get().trees[0].layers[1].bytes.capacity();
            assert_eq!(initial_bytes, 4 * parent_bytes);
            let z = rng.f128();
            let mut timings = Timings::default();
            let a = extension_tree_compact(&mut owned, 0, z, &mut timings);
            assert!(owned.get().trees[0].layers[1].bytes.is_empty());
            assert_eq!(owned.get().resident_bytes(), initial_bytes - parent_bytes);
            assert!(!owned.get().trees[1].layers[1].bytes.is_empty());
            let b = extension_tree_compact(&mut owned, 1, z, &mut timings);
            assert_eq!(
                owned.get().resident_bytes(),
                initial_bytes - 2 * parent_bytes
            );
            let expected_a = extension_tree(
                &cached.0.trees[0],
                cached.0.domain.base,
                z,
                &mut timings,
                false,
            );
            let expected_b = extension_tree(
                &cached.0.trees[1],
                cached.0.domain.base,
                z,
                &mut timings,
                false,
            );
            assert_eq!(a.layers, expected_a.layers);
            assert_eq!(b.layers, expected_b.layers);
            drop((a, b, expected_a, expected_b));
            for side in 0..2 {
                let parent = prepare_parent_layer(
                    &owned.get().trees[side].layers[0],
                    2,
                    &owned.get().domain,
                );
                assert_polys_equal(&parent, &cached.0.trees[side].layers[1]);
            }
            let mut ordinary_ch = FsChallenger::new(b"packed-compact-k64-padding");
            let (ordinary, ordinary_claims, _) = prove_bounded(cached, &mut ordinary_ch).unwrap();
            let mut compact_ch = FsChallenger::new(b"packed-compact-k64-padding");
            let (compact, compact_claims, _) =
                prove_bounded_compact(prepare_bounded(&source).unwrap(), &mut compact_ch).unwrap();
            assert_eq!(
                bincode::serialize(&ordinary).unwrap(),
                bincode::serialize(&compact).unwrap()
            );
            assert_eq!(ordinary_claims, compact_claims);
            assert_eq!(ordinary_ch.sample_f128(), compact_ch.sample_f128());
            let mut verifier = FsChallenger::new(b"packed-compact-k64-padding");
            assert_eq!(
                verify(source.statement(), &compact, &mut verifier).unwrap(),
                compact_claims
            );
        }
    }

    #[test]
    fn active_prefix_subbyte_coefficients_match_bit_oracle() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        for bits in [1usize, 2, 4] {
            // Covers degree*bits below eight, at eight, and above eight.
            for degree in [1usize, 2, 4, 8] {
                for count in [1usize, 3, 7, 8, 9, 17] {
                    for active in 0..=count {
                        let mask = (1u128 << bits) - 1;
                        let coefficient =
                            |row: usize, j: usize| ((row * 5 + j * 3 + row / 2 + 1) as u128) & mask;
                        let calls: Vec<_> = (0..count).map(|_| AtomicUsize::new(0)).collect();
                        let polys =
                            Polys::generate_prefix(count, active, degree, bits, |i, out| {
                                assert!(i < active);
                                calls[i].fetch_add(1, Ordering::Relaxed);
                                for (j, value) in out.iter_mut().enumerate() {
                                    *value = coefficient(i, j);
                                }
                            });
                        let mut expected = vec![0u8; (count * degree * bits).div_ceil(8)];
                        for row in 0..count {
                            assert_eq!(
                                calls[row].load(Ordering::Relaxed),
                                usize::from(row < active)
                            );
                            for j in 0..degree {
                                let value = if row < active { coefficient(row, j) } else { 0 };
                                assert_eq!(polys.coefficient(row, j), value);
                                for bit in 0..bits {
                                    let position = (row * degree + j) * bits + bit;
                                    expected[position / 8] |=
                                        (((value >> bit) & 1) as u8) << (position % 8);
                                }
                            }
                        }
                        assert_eq!(
                            polys.bytes, expected,
                            "bits={bits}, degree={degree}, count={count}, active={active}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn eight_row_leaf_decode_preserves_field_width_partial_groups_and_zero_padding() {
        let mut rng = Rng::new(0x83A7_1391);
        for width in [1usize, 2, 4, 7, 8, 13, 16, 31, 32, 63, 64] {
            for rows in [1usize, 3, 8, 16, 19, 24, 129, 1 << 16] {
                let raw = values(rows, width, &mut rng);
                let source = PackedBits::from_rows(&raw, width).unwrap();
                let count = rows.next_power_of_two();
                let bits = width.next_power_of_two();
                let expected = Polys::leaves(&source, count, bits, true);
                let optimized = Polys::leaves(&source, count, bits, false);
                assert_polys_equal(&optimized, &expected);
                for row in 0..count {
                    assert_eq!(
                        optimized.coefficient(row, 0),
                        raw.get(row).copied().unwrap_or(0)
                    );
                }
            }
        }
    }

    #[test]
    fn decoded_preparation_preserves_all_coefficients_proof_bytes_and_source_claims() {
        let mut rng = Rng::new(0xCAB1_64A2);
        for width in [8usize, 16, 32, 64] {
            let kappa = 128 / width;
            let minimum = kappa * kappa;
            // Full input, byte-aligned input requiring power-of-two padding,
            // and an input whose column-major byte boundaries are unaligned.
            for rows in [minimum, minimum + 8, minimum + 3] {
                let raw = values(rows, width, &mut rng);
                let mut permuted = raw.clone();
                permuted.reverse();
                let source = permutation::prepare(
                    PackedBits::from_rows(&raw, width).unwrap(),
                    PackedBits::from_rows(&permuted, width).unwrap(),
                )
                .unwrap();
                let reference = prepare_impl(&source, true).unwrap();
                let optimized = prepare_impl(&source, false).unwrap();
                assert_eq!(reference.statement(), optimized.statement());
                assert_eq!(reference.kappa(), optimized.kappa());
                for (a, b) in reference.trees.iter().zip(&optimized.trees) {
                    assert_eq!(a.layers.len(), b.layers.len());
                    assert_eq!(a.samples.len(), b.samples.len());
                    for (a, b) in a
                        .layers
                        .iter()
                        .chain(&a.samples)
                        .zip(b.layers.iter().chain(&b.samples))
                    {
                        assert_polys_equal(a, b);
                    }
                }
                let mut rc = FsChallenger::new(b"packed-preparation-eight-rows");
                let mut pc = FsChallenger::new(b"packed-preparation-eight-rows");
                let (reference_proof, reference_claims, _) = prove(&reference, &mut rc).unwrap();
                let (proof, claims, _) = prove(&optimized, &mut pc).unwrap();
                assert_eq!(
                    bincode::serialize(&reference_proof).unwrap(),
                    bincode::serialize(&proof).unwrap()
                );
                assert_eq!(reference_claims, claims);
                let mut vc = FsChallenger::new(b"packed-preparation-eight-rows");
                assert_eq!(verify(source.statement(), &proof, &mut vc).unwrap(), claims);
                let next = pc.sample_f128();
                assert_eq!(next, rc.sample_f128());
                assert_eq!(next, vc.sample_f128());
                for (bits, claim) in [source.left(), source.right()].into_iter().zip(&claims) {
                    let field = SmallField::for_width(width).unwrap();
                    let actual = (0..rows).fold(F128::ZERO, |sum, row| {
                        // Original source bits and the fixed field embedding,
                        // independent of the optimized decoder and coefficients.
                        let value = (0..width).fold(F128::ZERO, |value, bit| {
                            if bits.bit(bit * rows + row) {
                                value + field.basis()[bit]
                            } else {
                                value
                            }
                        });
                        sum + value
                            * claim.column_weights[row % kappa]
                            * row_weight(&claim.row_point, row / kappa)
                    });
                    assert_eq!(actual, claim.value);
                }
            }
        }
    }
}
