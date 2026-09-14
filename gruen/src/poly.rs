//! Natural-order radix-2 transforms and base-domain Lagrange evaluation.
use crate::field::{Extension, Field, Prime};

pub fn eval<F: Field>(p: &[F], x: F) -> F {
    p.iter().rev().fold(F::ZERO, |acc, &c| acc * x + c)
}

pub struct Domain<B: Prime> {
    pub log: usize,
    pub nodes: Vec<B>,
    pub shift: B,
    pub vanishing_inverse: B,
    twiddles: Vec<Vec<B>>,
    inverse_twiddles: Vec<Vec<B>>,
    size_inverse: B,
    shift_powers: Vec<B>,
    inverse_shift_powers: Vec<B>,
}

impl<B: Prime> Domain<B> {
    pub fn new(log: usize) -> Result<Self, String> {
        // Bound allocations separately from the field's theoretical two-adicity.
        if log > 16 {
            return Err("experiment skip cap is 16 rounds".into());
        }
        let root = B::root(log).ok_or("skip exceeds base-field two-adicity")?;
        let size = 1usize << log;
        let shift = (2..1000)
            .map(B::from_u64)
            .find(|&s| s.pow(size as u128) != B::ONE)
            .ok_or("no disjoint multiplicative coset")?;
        let powers = |x: B, n: usize| {
            let mut v = Vec::with_capacity(n);
            let mut p = B::ONE;
            for _ in 0..n {
                v.push(p);
                p = p * x;
            }
            v
        };
        let twiddles = |x: B| {
            (1..=log)
                .map(|j| powers(x.pow((size >> j) as u128), 1 << (j - 1)))
                .collect()
        };
        Ok(Self {
            log,
            nodes: powers(root, size),
            shift,
            vanishing_inverse: (shift.pow(size as u128) - B::ONE).inverse().unwrap(),
            twiddles: twiddles(root),
            inverse_twiddles: twiddles(root.inverse().unwrap()),
            size_inverse: B::from_u64(size as u64).inverse().unwrap(),
            shift_powers: powers(shift, size),
            inverse_shift_powers: powers(shift.inverse().unwrap(), size),
        })
    }

    pub fn fft<E: Extension<B>>(&self, data: &mut [E], inverse: bool) {
        assert_eq!(data.len(), self.nodes.len());
        if self.log == 0 {
            return;
        }
        for i in 0..data.len() {
            let j = i.reverse_bits() >> (usize::BITS as usize - self.log);
            if i < j {
                data.swap(i, j);
            }
        }
        let twiddles = if inverse {
            &self.inverse_twiddles
        } else {
            &self.twiddles
        };
        for (stage, tw) in twiddles.iter().enumerate() {
            let half = 1 << stage;
            for block in data.chunks_exact_mut(2 * half) {
                for j in 0..half {
                    let u = block[j];
                    let v = block[j + half].scale(tw[j]);
                    block[j] = u + v;
                    block[j + half] = u - v;
                }
            }
        }
        if inverse {
            for x in data {
                *x = x.scale(self.size_inverse);
            }
        }
    }

    pub fn coset_lde(&self, values: &mut [B])
    where
        B: Extension<B>,
    {
        self.fft(values, true);
        for (v, &s) in values.iter_mut().zip(&self.shift_powers) {
            *v = *v * s;
        }
        self.fft(values, false);
    }

    pub fn coset_interpolate<E: Extension<B>>(&self, values: &mut [E]) {
        self.fft(values, true);
        for (v, &s) in values.iter_mut().zip(&self.inverse_shift_powers) {
            *v = v.scale(s);
        }
    }

    pub fn lagrange<E: Extension<B>>(&self, x: E) -> Vec<E> {
        // A challenge on the subgroup is mathematically valid; avoid 0/0.
        if let Some(i) = self.nodes.iter().position(|&h| E::lift(h) == x) {
            let mut v = vec![E::ZERO; self.nodes.len()];
            v[i] = E::ONE;
            return v;
        }
        let diffs: Vec<_> = self.nodes.iter().map(|&h| x - E::lift(h)).collect();
        let inv = batch_inverse(&diffs);
        let scale = (x.pow(self.nodes.len() as u128) - E::ONE).scale(self.size_inverse);
        self.nodes
            .iter()
            .zip(inv)
            .map(|(&h, inv)| (scale * inv).scale(h))
            .collect()
    }
}

pub fn batch_inverse<F: Field>(values: &[F]) -> Vec<F> {
    let mut out = Vec::with_capacity(values.len());
    let mut product = F::ONE;
    for &v in values {
        assert_ne!(v, F::ZERO);
        out.push(product);
        product = product * v;
    }
    let mut inv = product.inverse().unwrap();
    for i in (0..values.len()).rev() {
        out[i] = out[i] * inv;
        inv = inv * values[i];
    }
    out
}

/// `z[0]` indexes the least significant Boolean coordinate.
pub fn eq_table<F: Field>(z: &[F]) -> Vec<F> {
    let mut out = vec![F::ONE];
    for &x in z.iter().rev() {
        let mut next = Vec::with_capacity(out.len() * 2);
        for v in out {
            let vx = v * x;
            next.push(v - vx);
            next.push(vx);
        }
        out = next;
    }
    out
}

pub fn eq<F: Field>(x: F, y: F) -> F {
    (F::ONE - x) * (F::ONE - y) + x * y
}

/// Select the FFT path when available; otherwise keep the requested block size
/// using distinct base-field nodes. Never silently reduce the number of rounds.
pub enum SkipDomain<B: Prime> {
    Radix2(Domain<B>),
    Lagrange(LagrangeDomain<B>),
}

impl<B: Prime + Extension<B>> SkipDomain<B> {
    pub fn new(log: usize) -> Result<Self, String> {
        validate_skip_domain::<B>(log)?;
        if log <= B::TWO_ADICITY {
            Ok(Self::Radix2(Domain::new(log)?))
        } else {
            Ok(Self::Lagrange(LagrangeDomain::new(1 << log)?))
        }
    }
    pub fn nodes(&self) -> &[B] {
        match self {
            Self::Radix2(d) => &d.nodes,
            Self::Lagrange(d) => &d.nodes,
        }
    }
    pub fn label(&self) -> &'static str {
        match self {
            Self::Radix2(_) => "radix2_coset",
            Self::Lagrange(_) => "general_lagrange",
        }
    }
    pub fn evaluate(&self, values: &mut [B], scratch: &mut [B]) {
        match self {
            Self::Radix2(d) => d.coset_lde(values),
            Self::Lagrange(d) => {
                for (out, row) in scratch.iter_mut().zip(&d.evaluation_matrix) {
                    *out = row
                        .iter()
                        .zip(values.iter())
                        .fold(B::ZERO, |acc, (&w, &v)| acc + w * v);
                }
                values.copy_from_slice(scratch);
            }
        }
    }
    pub fn inverse_vanishing(&self, index: usize) -> B {
        match self {
            Self::Radix2(d) => d.vanishing_inverse,
            Self::Lagrange(d) => d.vanishing_inverses[index],
        }
    }
    pub fn interpolate<E: Extension<B>>(&self, values: &mut [E]) {
        match self {
            Self::Radix2(d) => d.coset_interpolate(values),
            Self::Lagrange(d) => {
                let mut coefficients = vec![E::ZERO; values.len()];
                for (&v, basis) in values.iter().zip(&d.quotient_basis) {
                    for (out, &coefficient) in coefficients.iter_mut().zip(basis) {
                        *out = *out + v.scale(coefficient);
                    }
                }
                values.copy_from_slice(&coefficients);
            }
        }
    }
    pub fn lagrange<E: Extension<B>>(&self, x: E) -> Vec<E> {
        match self {
            Self::Radix2(d) => d.lagrange(x),
            Self::Lagrange(d) => d
                .input_basis
                .iter()
                .map(|p| p.iter().rev().fold(E::ZERO, |acc, &c| acc * x + E::lift(c)))
                .collect(),
        }
    }
}

pub fn validate_skip_domain<B: Prime>(log: usize) -> Result<(), String> {
    if log > 16 {
        return Err("experiment skip cap is 16 rounds".into());
    }
    if log > B::TWO_ADICITY {
        // Dense interpolation is quadratic in the block size. Current kappa
        // profiles require only 16 and 4 nodes; cap other callers explicitly.
        if log > 8 {
            return Err("general Lagrange interpolation cap is 256 nodes".into());
        }
        if (2u128 << log) > B::MODULUS as u128 {
            return Err("general interpolation requires 2*block_size distinct base nodes".into());
        }
    }
    Ok(())
}

/// Verifier-side vanishing polynomial, independent of interpolation matrices.
pub fn skip_vanishing<B: Prime, E: Extension<B>>(log: usize, x: E) -> E {
    if log <= B::TWO_ADICITY {
        x.pow(1 << log) - E::ONE
    } else {
        (0..1usize << log).fold(E::ONE, |acc, j| acc * (x - E::from_u64(j as u64)))
    }
}

/// General-node fallback: H={0,...,B-1}, S={B,...,2B-1}.
/// Computes the same quotient reduction with Psi_H instead of X^B-1.
pub struct LagrangeDomain<B: Prime> {
    pub nodes: Vec<B>,
    input_basis: Vec<Vec<B>>,
    evaluation_matrix: Vec<Vec<B>>,
    quotient_basis: Vec<Vec<B>>,
    vanishing_inverses: Vec<B>,
}

impl<B: Prime> LagrangeDomain<B> {
    pub fn new(size: usize) -> Result<Self, String> {
        if size == 0 || size > 256 || size as u128 * 2 > B::MODULUS as u128 {
            return Err("invalid general interpolation size".into());
        }
        let nodes: Vec<_> = (0..size).map(|i| B::from_u64(i as u64)).collect();
        let samples: Vec<_> = (size..2 * size).map(|i| B::from_u64(i as u64)).collect();
        let input_basis = lagrange_basis(&nodes);
        let evaluation_matrix = samples
            .iter()
            .map(|&s| input_basis.iter().map(|p| eval(p, s)).collect())
            .collect();
        let vanishing_values: Vec<_> = samples
            .iter()
            .map(|&s| nodes.iter().fold(B::ONE, |p, &h| p * (s - h)))
            .collect();
        Ok(Self {
            nodes,
            input_basis,
            evaluation_matrix,
            quotient_basis: lagrange_basis(&samples),
            vanishing_inverses: batch_inverse(&vanishing_values),
        })
    }
}

/// Coefficients of each cardinal polynomial, computed once per domain setup.
fn lagrange_basis<B: Prime>(nodes: &[B]) -> Vec<Vec<B>> {
    nodes
        .iter()
        .enumerate()
        .map(|(i, &node)| {
            let mut polynomial = vec![B::ONE];
            let mut denominator = B::ONE;
            for (j, &other) in nodes.iter().enumerate() {
                if i == j {
                    continue;
                }
                let mut next = vec![B::ZERO; polynomial.len() + 1];
                for (degree, &c) in polynomial.iter().enumerate() {
                    next[degree] = next[degree] - other * c;
                    next[degree + 1] = next[degree + 1] + c;
                }
                polynomial = next;
                denominator = denominator * (node - other);
            }
            let inv = denominator.inverse().unwrap();
            for c in &mut polynomial {
                *c = *c * inv;
            }
            polynomial
        })
        .collect()
}
