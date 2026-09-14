//! Packed polynomial-tree permutation reduction from the revised manuscript.
//!
//! The public index convention is A[row, column] = source[kappa * row + column].
//! We pair adjacent rows, rather than manuscript upper/lower halves. These are
//! fixed index permutations; source values and their field embedding are intact.
//! All coefficient construction is offline. Every challenge-dependent table,
//! contraction, weighted aggregation, tree and proof message is computed online.

use crate::{
    Error,
    bits::PackedBits,
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
mod domain;
mod projection_kernel;
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
struct Polys {
    count: usize,
    degree: usize,
    bits: usize,
    stride: usize,
    bytes: Vec<u8>,
}

pub fn prepare(source: &PreparedPermutation) -> Result<Prepared, Error> {
    let (kappa, _) = shape(source.statement())?;
    let domain = domain(source.statement().width)?;
    let trees = [
        prepare_tree(source.left(), kappa, &domain),
        prepare_tree(source.right(), kappa, &domain),
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
    fn generate<F>(count: usize, degree: usize, bits: usize, f: F) -> Self
    where
        F: Fn(usize, &mut [u128]) + Sync + Send,
    {
        let width = degree * bits;
        let stride = width.div_ceil(8);
        let mut bytes = vec![0u8; (count * width).div_ceil(8)];
        if width < 8 {
            let per_byte = 8 / width;
            bytes.par_iter_mut().enumerate().for_each_init(
                || vec![0; degree],
                |coeff, (block, dst)| {
                    let mut word = 0u8;
                    for r in 0..per_byte.min(count - block * per_byte) {
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
            bytes
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

fn prepare_tree(bits: &PackedBits, kappa: usize, domain: &Domain) -> PreparedTree {
    let len = crate::padded_len(bits.rows()).unwrap();
    let mut layers = vec![Polys::generate(len, 1, domain.base.base.bits, |i, out| {
        out[0] = if i < bits.rows() { bits.row(i) } else { 0 };
    })];
    let mut samples = Vec::with_capacity(kappa.ilog2() as usize);
    for _ in 0..kappa.ilog2() {
        let prev = layers.last().unwrap();
        let d = prev.degree;
        let rows = prev.count / kappa / 2;
        let next = Polys::generate(rows * kappa, 2 * d, prev.bits, |node, out| {
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
        });
        let eta = Polys::generate(
            rows * (kappa - 1),
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
        );
        layers.push(next);
        samples.push(eta);
    }
    PreparedTree { layers, samples }
}

struct DenseTree {
    layers: Vec<Vec<F128>>,
    log_len: usize,
}
const POOL_SLOT: usize = 2;

impl Drop for DenseTree {
    fn drop(&mut self) {
        for values in &mut self.layers {
            if !values.is_empty() {
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
        crate::tree_pool::give_f128(POOL_SLOT, std::mem::take(&mut self.layers[level]));
    }
}

fn extension_tree(
    prepared: &PreparedTree,
    base: &FastField,
    z: F128,
    p: &mut Timings,
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
    DenseTree { layers, log_len }
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
    let (kappa, log_len) = bind(&prepared.statement, c)?;
    let s = kappa.ilog2() as usize;
    let z = c.sample_f128();
    let mut powers = Vec::with_capacity(kappa + 1);
    powers.push(F128::ONE);
    for i in 0..kappa {
        powers.push(powers[i] * z);
    }
    let mut timings = Timings::default();
    let [left, right] = prepared.trees.each_ref();
    let mut a = extension_tree(left, prepared.domain.base, z, &mut timings);
    let mut b = extension_tree(right, prepared.domain.base, z, &mut timings);
    let roots = [a.root(), b.root()];
    if roots[0] != roots[1] {
        return Err(Error::ProductMismatch);
    }
    c.observe_f128_slice(&roots);
    c.observe_label(b"packed-permutation-extension-tree-v1");
    let extension_gkr_start = Instant::now();
    let (extension, endpoints) =
        crate::fast_sumcheck::prove_pair(&mut a, &mut b, c, &mut timings, true, true);
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
        let columns = prepared.trees.each_ref().map(|tree| {
            aggregate(
                &tree.layers[level + 1],
                kappa,
                prepared.domain.base,
                &point,
                &powers,
            )
        });
        let samples = prepared.trees.each_ref().map(|tree| {
            aggregate(
                &tree.samples[level],
                kappa - 1,
                prepared.domain.field,
                &point,
                &powers,
            )
        });
        let quotients =
            std::array::from_fn::<_, 2, _>(|j| prepared.domain.quotient(&columns[j], &samples[j]));
        let aggregate_ms = elapsed(start);
        timings.messages_ms += aggregate_ms;
        timings.packed_aggregate_ms += aggregate_ms;
        for j in 0..2 {
            debug_assert_eq!(dot(&columns[j], &rho), values[j]);
            c.observe_f128_slice(&columns[j]);
            c.observe_f128_slice(&quotients[j]);
        }
        let x = column_challenge(&prepared.domain, c);
        rho = prepared.domain.packing(x);
        let psi = prepared.domain.vanishing(x);
        values = std::array::from_fn(|j| {
            dot(&columns[j], &rho) + psi * domain::evaluate_extension(&quotients[j], x)
        });
        let start = Instant::now();
        let children = prepared
            .trees
            .each_ref()
            .map(|tree| packed_children(&tree.layers[level], &prepared.domain, &rho, &powers));
        let projection_ms = elapsed(start);
        timings.layer_materialize_ms += projection_ms;
        timings.packed_projection_ms += projection_ms;
        let (layers, next_point, next_values) = crate::fast_sumcheck::prove_weighted_pair(
            [&children[0][0], &children[1][0]],
            [&children[0][1], &children[1][1]],
            &point,
            values,
            c,
            &mut timings,
        );
        let release_start = Instant::now();
        for values in children.into_iter().flatten() {
            crate::tree_pool::give_f128(POOL_SLOT, values);
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
