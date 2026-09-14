//! Formal K[Z] coefficient packing with a Hadamard quotient identity in X.
//! See ../../PACKED_REDUCTION.md for ordering, layout and authentication.
use crate::{
    Error,
    batched::{self, InnerProof, Input, Scratch},
    claims::Claims,
    field::{FastField, PolynomialEvaluation},
    permutation::{PreparedRows, Statement, Stats, source_claims},
    product_tree::{Options, ProductTree, Tree},
    profile::{Engine, Timings, elapsed},
    sumcheck::LeafClaim,
};
use flock_core::zerocheck::univariate_skip::SplitEqGhash;
use flock_field::{F128, F256Unreduced};
use flock_multilinear::{IndexOrder, eq_table};
use flock_transcript::challenger::Challenger;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ColumnProof {
    pub columns: Vec<F128>,
    pub quotient: Vec<F128>,
    pub inner: InnerProof,
    pub terminal_columns: [Vec<F128>; 4],
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Layer {
    Dense(InnerProof),
    Packed(ColumnProof),
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proof {
    pub roots: [F128; 2],
    pub layers: Vec<Layer>,
}

/// Derived from coefficient capacity and the actual tuple subfield. H must
/// contain distinct K elements, including when this API is used on tiny K.
fn kappa(width: usize, level: usize, gate_log: usize) -> usize {
    let k = width.max(1).next_power_of_two();
    let bits = (1usize << level).saturating_mul(k);
    if bits > 64 {
        return 1;
    }
    let cardinality = if k < 8 { 1usize << k } else { usize::MAX };
    (128 / bits).min(1usize << gate_log).min(cardinality)
}

struct Domain {
    field: &'static FastField,
    k: usize,
    lagrange: [[u128; 16]; 16], // [column j][coefficient of X]
    psi: Vec<u128>,
}
impl Domain {
    fn new(field: &'static FastField, k: usize) -> Self {
        assert!(k.is_power_of_two() && (2..=16).contains(&k));
        let mul = |a, b| field.multiply(a, b);
        let mut lagrange = [[0u128; 16]; 16];
        let mut psi = vec![1u128];
        for j in 0..k {
            let mut next = vec![0; psi.len() + 1];
            for (i, &c) in psi.iter().enumerate() {
                next[i] ^= mul(c, j as u128);
                next[i + 1] ^= c;
            }
            psi = next;
        }
        for j in 0..k {
            let mut polynomial = vec![1u128];
            let mut denominator = 1;
            for h in 0..k {
                if j != h {
                    denominator = mul(denominator, (j ^ h) as u128);
                    let mut next = vec![0; polynomial.len() + 1];
                    for (i, &c) in polynomial.iter().enumerate() {
                        next[i] ^= mul(c, h as u128);
                        next[i + 1] ^= c;
                    }
                    polynomial = next;
                }
            }
            let mut inverse = 1;
            let mut base = denominator;
            let mut power = (1u128 << field.base.bits) - 2;
            while power != 0 {
                if power & 1 != 0 {
                    inverse = mul(inverse, base);
                }
                base = mul(base, base);
                power >>= 1;
            }
            for (i, &c) in polynomial.iter().enumerate() {
                lagrange[j][i] = mul(c, inverse);
            }
        }
        Self {
            field,
            k,
            lagrange,
            psi,
        }
    }
    fn at(&self, x: F128) -> Vec<F128> {
        (0..self.k)
            .map(|j| {
                self.lagrange[j][..self.k]
                    .iter()
                    .rev()
                    .fold(F128::ZERO, |v, &c| v * x + self.field.embed(c))
            })
            .collect()
    }
    fn vanishing(&self, x: F128) -> F128 {
        self.psi
            .iter()
            .rev()
            .fold(F128::ZERO, |v, &c| v * x + self.field.embed(c))
    }
    fn challenge<C: Challenger>(&self, c: &mut C) -> Result<F128, Error> {
        for _ in 0..16 {
            let x = c.sample_f128();
            if !(0..self.k).any(|j| x == self.field.embed(j as u128)) {
                return Ok(x);
            }
        }
        Err(Error::Transcript)
    }
    #[inline]
    fn scale_word(&self, word: u128, scalar: u128, degree: usize) -> u128 {
        if scalar == 0 {
            return 0;
        }
        if scalar == 1 {
            return word;
        }
        let k = self.field.base.bits;
        let mask = self.field.base.mask();
        let mut out = 0;
        for i in 0..degree {
            out ^= self.field.multiply((word >> (i * k)) & mask, scalar) << (i * k);
        }
        out
    }
    fn product(&self, a: u128, b: u128, d: usize, monic: bool) -> u128 {
        let k = self.field.base.bits;
        let mask = self.field.base.mask();
        let mut out = if monic { (a ^ b) << (d * k) } else { 0 };
        for s in 0..2 * d - 1 {
            let mut value = 0;
            if (16..=64).contains(&k) {
                let mut accumulator = 0;
                for i in 0..d {
                    if s >= i && s - i < d {
                        accumulator ^= self
                            .field
                            .multiply_unreduced((a >> (i * k)) & mask, (b >> ((s - i) * k)) & mask);
                    }
                }
                value = self.field.reduce(accumulator);
            } else {
                for i in 0..d {
                    if s >= i && s - i < d {
                        value ^= self
                            .field
                            .multiply((a >> (i * k)) & mask, (b >> ((s - i) * k)) & mask);
                    }
                }
            }
            out ^= value << (s * k);
        }
        out
    }
    fn interpolate(&self, words: &[u128; 16], d: usize) -> [u128; 16] {
        let mut out = [0; 16];
        for j in 0..self.k {
            for (s, v) in out[..self.k].iter_mut().enumerate() {
                *v ^= self.scale_word(words[j], self.lagrange[j][s], d);
            }
        }
        out
    }
    fn quotient(&self, a: &[u128; 16], b: &[u128; 16], d: usize) -> [u128; 16] {
        let mut product = [0; 32];
        // Only high X coefficients are needed for monic long division. In
        // these terms neither factor has X degree zero, so both Z polynomials
        // are non-monic (their common leading Z^d cancels under interpolation).
        for (s, v) in product
            .iter_mut()
            .enumerate()
            .take(2 * self.k - 1)
            .skip(self.k)
        {
            for i in 1..self.k {
                if s >= i && s - i < self.k {
                    *v ^= self.product(a[i], b[s - i], d, false);
                }
            }
        }
        let mut quotient = [0; 16];
        for s in (self.k..2 * self.k - 1).rev() {
            let value = product[s];
            quotient[s - self.k] = value;
            for j in 0..self.k {
                if s - self.k + j >= self.k {
                    product[s - self.k + j] ^= self.scale_word(value, self.psi[j], 2 * d);
                }
            }
        }
        quotient
    }
}

fn words(a: &ProductTree, b: &ProductTree, level: usize, row: usize, k: usize) -> [[u128; 16]; 4] {
    let mut out = [[0; 16]; 4];
    for j in 0..k {
        let gate = row * k + j;
        out[0][j] = a.coefficient_word(level, 2 * gate);
        out[1][j] = a.coefficient_word(level, 2 * gate + 1);
        out[2][j] = b.coefficient_word(level, 2 * gate);
        out[3][j] = b.coefficient_word(level, 2 * gate + 1);
    }
    out
}
fn vector_add(mut a: Vec<F128>, b: Vec<F128>) -> Vec<F128> {
    for (x, y) in a.iter_mut().zip(b) {
        *x += y;
    }
    a
}
fn weighted_coefficients(
    a: &ProductTree,
    b: &ProductTree,
    level: usize,
    point: &[F128],
    domain: &Domain,
    z: F128,
    lambda: F128,
) -> Vec<F128> {
    let k = domain.k;
    let d = 1usize << level;
    let count = 2 * k - 1;
    let eq = SplitEqGhash::with_n_hi(point, point.len() / 2);
    let evaluator = PolynomialEvaluation::new(domain.field.base, 2 * d * domain.field.base.bits, z);
    let calculate = |hi: usize| {
        let mut sum = vec![F256Unreduced::ZERO; 2 * count];
        for (lo, &weight) in eq.lo.iter().enumerate() {
            let row = hi * eq.lo.len() + lo;
            let w = words(a, b, level, row, k);
            for tree in 0..2 {
                let left = &w[2 * tree];
                let right = &w[2 * tree + 1];
                for j in 0..k {
                    let value = evaluator.evaluate(domain.product(left[j], right[j], d, true));
                    sum[tree * count + j] ^= value.mul_unreduced(weight);
                }
                let ga = domain.interpolate(left, d);
                let gb = domain.interpolate(right, d);
                let h = domain.quotient(&ga, &gb, d);
                for (j, &word) in h[..k - 1].iter().enumerate() {
                    sum[tree * count + k + j] ^= evaluator.linear(word).mul_unreduced(weight);
                }
            }
        }
        (0..count)
            .map(|j| (sum[j].reduce() + lambda * sum[count + j].reduce()) * eq.hi[hi])
            .collect()
    };
    if (1usize << point.len()) < 1 << 12 {
        (0..eq.hi.len())
            .map(calculate)
            .fold(vec![F128::ZERO; count], vector_add)
    } else {
        (0..eq.hi.len())
            .into_par_iter()
            .map(calculate)
            .reduce(|| vec![F128::ZERO; count], vector_add)
    }
}
fn projections(
    a: &ProductTree,
    b: &ProductTree,
    level: usize,
    rows: usize,
    domain: &Domain,
    z: F128,
    rho: &[F128],
) -> [Vec<F128>; 4] {
    let evaluator = PolynomialEvaluation::new(
        domain.field.base,
        (1usize << level) * domain.field.base.bits,
        z,
    );
    let maps: Vec<_> = rho.iter().map(|&r| evaluator.scaled(r)).collect();
    let mut out: [Vec<F128>; 4] = std::array::from_fn(|_| vec![F128::ZERO; rows]);
    let [a_out, b_out, c_out, d_out] = &mut out;
    a_out
        .par_iter_mut()
        .zip(b_out.par_iter_mut())
        .zip(c_out.par_iter_mut())
        .zip(d_out.par_iter_mut())
        .enumerate()
        .with_min_len(512)
        .for_each(|(row, (((a_out, b_out), c_out), d_out))| {
            let w = words(a, b, level, row, domain.k);
            let result: [F128; 4] = std::array::from_fn(|s| {
                (0..domain.k).fold(F128::ZERO, |v, j| v + maps[j].evaluate(w[s][j]))
            });
            *a_out = result[0];
            *b_out = result[1];
            *c_out = result[2];
            *d_out = result[3];
        });
    out
}
fn terminal_columns(
    a: &ProductTree,
    b: &ProductTree,
    level: usize,
    point: &[F128],
    domain: &Domain,
    z: F128,
) -> [Vec<F128>; 4] {
    let eq = SplitEqGhash::with_n_hi(point, point.len() / 2);
    let k = domain.k;
    let evaluator = PolynomialEvaluation::new(
        domain.field.base,
        (1usize << level) * domain.field.base.bits,
        z,
    );
    let calculate = |hi: usize| {
        let mut sum = vec![F256Unreduced::ZERO; 4 * k];
        for (lo, &weight) in eq.lo.iter().enumerate() {
            let w = words(a, b, level, hi * eq.lo.len() + lo, k);
            for s in 0..4 {
                for j in 0..k {
                    sum[s * k + j] ^= evaluator.evaluate(w[s][j]).mul_unreduced(weight);
                }
            }
        }
        sum.into_iter().map(|x| x.reduce() * eq.hi[hi]).collect()
    };
    let values = if (1usize << point.len()) < 1 << 12 {
        (0..eq.hi.len())
            .map(calculate)
            .fold(vec![F128::ZERO; 4 * k], vector_add)
    } else {
        (0..eq.hi.len())
            .into_par_iter()
            .map(calculate)
            .reduce(|| vec![F128::ZERO; 4 * k], vector_add)
    };
    std::array::from_fn(|s| values[s * k..(s + 1) * k].to_vec())
}
fn dot(a: &[F128], b: &[F128]) -> F128 {
    a.iter().zip(b).fold(F128::ZERO, |v, (&x, &y)| v + x * y)
}
fn bind_columns<C: Challenger>(
    proof: &ColumnProof,
    domain: &Domain,
    rho: &[F128],
    c: &mut C,
) -> Result<(Vec<F128>, [F128; 4]), Error> {
    for s in 0..4 {
        if proof.terminal_columns[s].len() != domain.k
            || dot(&proof.terminal_columns[s], rho) != proof.inner.terminal[s]
        {
            return Err(Error::Constraint);
        }
    }
    c.observe_label(b"packed-terminal-columns");
    for v in &proof.terminal_columns {
        c.observe_f128_slice(v);
    }
    let eta = c.sample_f128_vec(domain.k.ilog2() as usize);
    let weights = eq_table(&eta, F128::ONE, IndexOrder::LowToHigh);
    let values = std::array::from_fn(|s| dot(&proof.terminal_columns[s], &weights));
    Ok((eta, values))
}
fn shorter_claim<C: Challenger>(
    proof: &ColumnProof,
    point: &[F128],
    claim: F128,
    domain: &Domain,
    ch: &mut C,
) -> Result<(Vec<F128>, F128), Error> {
    let k = domain.k;
    if proof.columns.len() != k || proof.quotient.len() != k - 1 {
        return Err(Error::ProofShape);
    }
    let weights = eq_table(
        &point[..k.ilog2() as usize],
        F128::ONE,
        IndexOrder::LowToHigh,
    );
    if dot(&weights, &proof.columns) != claim {
        return Err(Error::Constraint);
    }
    ch.observe_f128_slice(&proof.columns);
    ch.observe_f128_slice(&proof.quotient);
    let x = domain.challenge(ch)?;
    let rho = domain.at(x);
    let h = proof
        .quotient
        .iter()
        .rev()
        .fold(F128::ZERO, |v, &a| v * x + a);
    Ok((
        rho.clone(),
        dot(&proof.columns, &rho) + domain.vanishing(x) * h,
    ))
}

pub fn prove<C: Challenger>(
    rows: &PreparedRows,
    ch: &mut C,
) -> Result<(Proof, Claims, Stats, Timings), Error> {
    let mut p = Timings::default();
    rows.statement.bind(ch)?;
    ch.observe_label(b"mikros-packed-coefficient-gkr-v3");
    let z = ch.sample_f128();
    let width = rows.statement.left.width;
    let field = FastField::for_width(width)?;
    let a = ProductTree::from_rows(
        &rows.values[0],
        width,
        z,
        Options::default(),
        Engine::Fused,
        &mut p,
    )?;
    let b = ProductTree::from_rows(
        &rows.values[1],
        width,
        z,
        Options::default(),
        Engine::Fused,
        &mut p,
    )?;
    let roots = [a.root(), b.root()];
    if roots[0] != roots[1] {
        return Err(Error::ProductMismatch);
    }
    ch.observe_f128_slice(&roots);
    let mut values = roots;
    let mut point = Vec::new();
    let mut layers = Vec::new();
    let mut scratch = Scratch::new(if a.log_len > 1 {
        1usize << (a.log_len - 2)
    } else {
        0
    });
    for level in (0..a.log_len).rev() {
        ch.observe_bytes(&(level as u64).to_le_bytes());
        let lambda = ch.sample_f128();
        let claim = values[0] + lambda * values[1];
        let k = kappa(width, level, point.len());
        let (layer, q, terminal) = if k > 1 {
            ch.observe_label(b"packed-column-layer");
            ch.observe_bytes(&(k as u64).to_le_bytes());
            let domain = Domain::new(field, k);
            let skip = k.ilog2() as usize;
            let t = Instant::now();
            let coeffs = weighted_coefficients(&a, &b, level, &point[skip..], &domain, z, lambda);
            p.packed_quotient_ms += elapsed(t);
            let mut proof = ColumnProof {
                columns: coeffs[..k].to_vec(),
                quotient: coeffs[k..].to_vec(),
                inner: InnerProof {
                    rounds: Vec::new(),
                    terminal: [F128::ZERO; 4],
                },
                terminal_columns: std::array::from_fn(|_| Vec::new()),
            };
            let (rho, short) = shorter_claim(&proof, &point, claim, &domain, ch)?;
            let t = Instant::now();
            let proj = projections(
                &a,
                &b,
                level,
                1usize << (point.len() - skip),
                &domain,
                z,
                &rho,
            );
            p.packed_projection_ms += elapsed(t);
            let (inner, q) = batched::prove_inner(
                Input::Separate(proj.each_ref().map(|v| v.as_slice())),
                &point[skip..],
                short,
                lambda,
                &mut scratch,
                ch,
                &mut p,
            );
            proof.inner = inner;
            let t = Instant::now();
            proof.terminal_columns = terminal_columns(&a, &b, level, &q, &domain, z);
            p.packed_columns_ms += elapsed(t);
            let (mut eta, terminal) = bind_columns(&proof, &domain, &rho, ch)?;
            eta.extend(q);
            p.packed_layers += 1;
            p.skipped_rounds += skip;
            (Layer::Packed(proof), eta, terminal)
        } else {
            ch.observe_label(b"dense-row-layer");
            let t = Instant::now();
            let ac = Tree::layer(&a, level);
            let bc = Tree::layer(&b, level);
            p.layer_materialize_ms += elapsed(t);
            let (inner, q) = batched::prove_inner(
                Input::Interleaved([&ac, &bc]),
                &point,
                claim,
                lambda,
                &mut scratch,
                ch,
                &mut p,
            );
            let terminal = inner.terminal;
            (Layer::Dense(inner), q, terminal)
        };
        let r = ch.sample_f128();
        values = [
            terminal[0] + r * (terminal[0] + terminal[1]),
            terminal[2] + r * (terminal[2] + terminal[3]),
        ];
        point = vec![r];
        point.extend(q);
        layers.push(layer);
    }
    let claims = source_claims(
        &rows.statement,
        std::array::from_fn(|s| LeafClaim {
            point: point.clone(),
            value: values[s] + z,
        }),
    )?;
    Ok((
        Proof { roots, layers },
        claims,
        Stats {
            left: a.stats,
            right: b.stats,
        },
        p,
    ))
}
pub fn verify<C: Challenger>(
    statement: &Statement,
    proof: &Proof,
    ch: &mut C,
) -> Result<Claims, Error> {
    statement.bind(ch)?;
    let log = crate::padded_len(statement.left.rows)?.ilog2() as usize;
    if proof.layers.len() != log || proof.roots[0] != proof.roots[1] {
        return Err(Error::ProofShape);
    }
    ch.observe_label(b"mikros-packed-coefficient-gkr-v3");
    let z = ch.sample_f128();
    ch.observe_f128_slice(&proof.roots);
    let field = FastField::for_width(statement.left.width)?;
    let mut values = proof.roots;
    let mut point = Vec::new();
    for (level, layer) in (0..log).rev().zip(&proof.layers) {
        ch.observe_bytes(&(level as u64).to_le_bytes());
        let lambda = ch.sample_f128();
        let claim = values[0] + lambda * values[1];
        let k = kappa(statement.left.width, level, point.len());
        let (q, terminal) = match layer {
            Layer::Packed(proof) if k > 1 => {
                ch.observe_label(b"packed-column-layer");
                ch.observe_bytes(&(k as u64).to_le_bytes());
                let domain = Domain::new(field, k);
                let (rho, short) = shorter_claim(proof, &point, claim, &domain, ch)?;
                let q = batched::verify_inner(
                    &proof.inner,
                    &point[k.ilog2() as usize..],
                    short,
                    lambda,
                    ch,
                )?;
                let (mut eta, terminal) = bind_columns(proof, &domain, &rho, ch)?;
                eta.extend(q);
                (eta, terminal)
            }
            Layer::Dense(proof) if k == 1 => {
                ch.observe_label(b"dense-row-layer");
                (
                    batched::verify_inner(proof, &point, claim, lambda, ch)?,
                    proof.terminal,
                )
            }
            _ => return Err(Error::ProofShape),
        };
        let r = ch.sample_f128();
        values = [
            terminal[0] + r * (terminal[0] + terminal[1]),
            terminal[2] + r * (terminal[2] + terminal[3]),
        ];
        point = vec![r];
        point.extend(q);
    }
    source_claims(
        statement,
        std::array::from_fn(|s| LeafClaim {
            point: point.clone(),
            value: values[s] + z,
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn formal_quotient_identity_at_independent_x_z() {
        for width in [1, 2, 4, 8, 16, 32, 64] {
            let f = FastField::for_width(width).unwrap();
            for level in 0..7 {
                let k = kappa(width, level, 8);
                if k < 2 {
                    continue;
                }
                let d = 1usize << level;
                let dom = Domain::new(f, k);
                let mask = if d * width == 128 {
                    u128::MAX
                } else {
                    (1u128 << (d * width)) - 1
                };
                let a: [u128; 16] = std::array::from_fn(|i| {
                    ((i as u128 * 912749123 + 89) ^ ((i as u128) << 32)) & mask
                });
                let b: [u128; 16] = std::array::from_fn(|i| {
                    ((i as u128 * 71244931 + 171) ^ ((i as u128) << 21)) & mask
                });
                let ga = dom.interpolate(&a, d);
                let gb = dom.interpolate(&b, d);
                let h = dom.quotient(&ga, &gb, d);
                for x in [F128::new(0xdeadbeef, 1), F128::new(317, 991)] {
                    let rho = dom.at(x);
                    assert_eq!(
                        rho.iter().copied().fold(F128::ZERO, |s, v| s + v),
                        F128::ONE
                    );
                    let z = F128::new(0xbadc0ffe, 137);
                    let ev = PolynomialEvaluation::new(f.base, d * width, z);
                    let prod = PolynomialEvaluation::new(f.base, 2 * d * width, z);
                    let av: (Vec<_>, Vec<_>) = (
                        (0..k).map(|j| ev.evaluate(a[j])).collect(),
                        (0..k).map(|j| ev.evaluate(b[j])).collect(),
                    );
                    let lhs = dot(&av.0, &rho) * dot(&av.1, &rho);
                    let c: Vec<_> = (0..k)
                        .map(|j| prod.evaluate(dom.product(a[j], b[j], d, true)))
                        .collect();
                    let hx = h[..k - 1]
                        .iter()
                        .rev()
                        .fold(F128::ZERO, |s, &word| s * x + prod.linear(word));
                    assert_eq!(
                        lhs,
                        dot(&c, &rho) + dom.vanishing(x) * hx,
                        "K={width},d={d},kappa={k}"
                    );
                }
            }
        }
    }
}
