//! Public interpolation data with verified, actual intermediate-field embeddings.
use crate::field::{FastField, raw};
use flock_field::F128;

pub(super) struct Domain {
    pub base: &'static FastField,
    pub field: &'static FastField,
    pub kappa: usize,
    pub lagrange: Vec<Vec<u128>>,
    pub off_weights: Vec<Vec<u128>>,
    pub psi: Vec<u128>,
    pub quotient_map: Vec<Vec<u128>>,
    base_map: Option<Vec<u128>>,
}

fn mul_poly(a: &[u128], b: &[u128], field: &FastField) -> Vec<u128> {
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out[i + j] ^= field.multiply(x, y);
        }
    }
    out
}
fn inverse(value: u128, field: &FastField) -> u128 {
    assert_ne!(value, 0);
    let mut exponent = (1u128 << field.base.bits) - 2;
    let mut x = value;
    let mut out = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            out = field.multiply(out, x);
        }
        x = field.multiply(x, x);
        exponent >>= 1;
    }
    out
}
fn basis(nodes: &[u128], field: &FastField) -> Vec<Vec<u128>> {
    nodes
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            let mut p = vec![1];
            let mut d = 1;
            for (j, &y) in nodes.iter().enumerate() {
                if i != j {
                    p = mul_poly(&p, &[y, 1], field);
                    d = field.multiply(d, x ^ y);
                }
            }
            let inv = inverse(d, field);
            p.into_iter().map(|v| field.multiply(v, inv)).collect()
        })
        .collect()
}
pub(super) fn evaluate(p: &[u128], x: u128, field: &FastField) -> u128 {
    p.iter().rev().fold(0, |v, &c| field.multiply(v, x) ^ c)
}
pub(super) fn evaluate_extension(p: &[F128], x: F128) -> F128 {
    p.iter().rev().fold(F128::ZERO, |v, &c| v * x + c)
}
impl Domain {
    pub fn new(base: &'static FastField) -> Self {
        let kappa = 128 / base.base.bits;
        // GF256 supplies at least 2*kappa-1 nodes for the binary/small cases.
        // For larger base fields no coordinate change is made.
        let field = FastField::for_width(base.base.bits.max(8)).unwrap();
        let base_map = if base.base.bits == field.base.bits {
            None
        } else {
            let embedded: std::collections::HashMap<_, _> =
                (0..256u128).map(|x| (raw(field.embed(x)), x)).collect();
            Some(
                (0..1u128 << base.base.bits)
                    .map(|x| {
                        *embedded
                            .get(&raw(base.embed(x)))
                            .expect("actual subfield inclusion")
                    })
                    .collect(),
            )
        };
        let roots: Vec<_> = (0..kappa as u128).collect();
        let off: Vec<_> = (kappa as u128..(2 * kappa - 1) as u128).collect();
        let lagrange = basis(&roots, field);
        let off_weights = off
            .iter()
            .map(|&x| lagrange.iter().map(|p| evaluate(p, x, field)).collect())
            .collect();
        let psi = roots
            .iter()
            .fold(vec![1], |p, &x| mul_poly(&p, &[x, 1], field));
        // q(off_j)=f(off_j)/Psi(off_j), degree(q)<=kappa-2.
        let quotient_map = basis(&off, field)
            .into_iter()
            .zip(&off)
            .map(|(p, &x)| {
                let inv = inverse(evaluate(&psi, x, field), field);
                p.into_iter().map(|v| field.multiply(v, inv)).collect()
            })
            .collect();
        Self {
            base,
            field,
            kappa,
            lagrange,
            off_weights,
            psi,
            quotient_map,
            base_map,
        }
    }
    pub fn lift(&self, x: u128) -> u128 {
        self.base_map.as_ref().map_or(x, |map| map[x as usize])
    }
    pub fn packing(&self, x: F128) -> Vec<F128> {
        self.lagrange
            .iter()
            .map(|p| {
                p.iter()
                    .rev()
                    .fold(F128::ZERO, |v, &a| v * x + self.field.embed(a))
            })
            .collect()
    }
    pub fn vanishing(&self, x: F128) -> F128 {
        self.psi
            .iter()
            .rev()
            .fold(F128::ZERO, |v, &a| v * x + self.field.embed(a))
    }
    pub fn is_node(&self, x: F128) -> bool {
        (0..self.kappa).any(|i| self.field.embed(i as u128) == x)
    }
    pub fn quotient(&self, columns: &[F128], samples: &[F128]) -> Vec<F128> {
        let mut q = vec![F128::ZERO; self.kappa - 1];
        for j in 0..self.kappa - 1 {
            let gc = columns
                .iter()
                .zip(&self.off_weights[j])
                .fold(F128::ZERO, |s, (&c, &w)| s + c * self.field.embed(w));
            let f = samples[j] + gc;
            for (out, &a) in q.iter_mut().zip(&self.quotient_map[j]) {
                *out += f * self.field.embed(a);
            }
        }
        q
    }
}
