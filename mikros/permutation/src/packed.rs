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
mod aggregate_tower;
mod copy_preparation;
mod domain;
mod projection_kernel;
pub use copy_preparation::{CopyPreparationPlan, CopyPreparationTimings};
use domain::Domain;

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
            out.fill(0);
            let row = node / kappa;
            let column = node % kappa;
            let a = 2 * row * kappa + column;
            let b = a + kappa;
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
    DenseTree { layers, log_len, recycle }
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
    let native = if polys.bits == 32 && kappa == 4 && polys.degree <= 2 {
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
    prove_impl(PreparedStorage::Borrowed(prepared), c)
}

/// The same transcript as `prove`, consuming coefficient storage as soon as
/// it is no longer needed. Temporary trees and fold buffers bypass the pools.
pub fn prove_bounded<C: Challenger>(
    prepared: BoundedPrepared,
    c: &mut C,
) -> Result<(Proof, [Claim; 2], Timings), Error> {
    prove_impl(PreparedStorage::Owned(prepared.0), c)
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
}

fn prove_impl<C: Challenger>(
    mut storage: PreparedStorage<'_>,
    c: &mut C,
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
    let [left, right] = storage.get().trees.each_ref();
    let mut a = extension_tree(left, domain.base, z, &mut timings, !bounded);
    let mut b = extension_tree(right, domain.base, z, &mut timings, !bounded);
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
        let columns = storage.get().trees.each_ref().map(|tree| {
            aggregate(
                &tree.layers[level + 1],
                kappa,
                domain.base,
                &point,
                &powers,
            )
        });
        let samples = storage.get().trees.each_ref().map(|tree| {
            if bounded {
                // At most one side's off-domain coefficients is resident.
                let samples = prepare_parent_samples(&tree.layers[level], kappa, &domain);
                aggregate(&samples, kappa - 1, domain.field, &point, &powers)
            } else {
                aggregate(&tree.samples[level], kappa - 1, domain.field, &point, &powers)
            }
        });
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
        let children = storage.get()
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
                &point, values, c, &mut timings, false,
            )
        } else {
            crate::fast_sumcheck::prove_weighted_pair(
                [&children[0][0], &children[1][0]],
                [&children[0][1], &children[1][1]],
                &point, values, c, &mut timings,
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
                ).unwrap();
                let cached = prepare(&source).unwrap();
                let bounded = prepare_bounded(&source).unwrap();
                assert_eq!(bounded.statement(), cached.statement());
                assert!(bounded.resident_bytes() < cached.resident_bytes());
                let mut cached_ch = FsChallenger::new(b"packed-bounded-lifetime");
                let mut bounded_ch = FsChallenger::new(b"packed-bounded-lifetime");
                let (expected, expected_claims, _) = prove(&cached, &mut cached_ch).unwrap();
                let (proof, claims, _) = prove_bounded(bounded, &mut bounded_ch).unwrap();
                assert_eq!(bincode::serialize(&proof).unwrap(), bincode::serialize(&expected).unwrap());
                assert_eq!(claims, expected_claims);
                let mut verifier = FsChallenger::new(b"packed-bounded-lifetime");
                assert_eq!(verify(source.statement(), &proof, &mut verifier).unwrap(), claims);
                let next = bounded_ch.sample_f128();
                assert_eq!(next, cached_ch.sample_f128());
                assert_eq!(next, verifier.sample_f128());
                for (bits, claim) in [source.left(), source.right()].into_iter().zip(&claims) {
                    let field = SmallField::for_width(width).unwrap();
                    let actual = (0..rows).fold(F128::ZERO, |sum, row| {
                        sum + field.embed(bits.row(row)) * claim.column_weights[row % kappa]
                            * row_weight(&claim.row_point, row / kappa)
                    });
                    assert_eq!(actual, claim.value);
                }
            }
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
