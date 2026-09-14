//! Laurent packed IP for F128 inputs, used by the E2E linear reduction.
//!
//! This is an algebraic extension of the Laurent protocol, not the binary
//! carryless coefficient kernel. Both correction polynomials contain 127 F128
//! coefficients. Packing and coefficient construction use extension arithmetic.
//! The caller binds both input vectors (or their verified derivation from the
//! circuit and commitments) before entry and authenticates both terminal claims.
//! In E2E lincheck these inputs depend on Hadamard challenges, so correction
//! construction, packing and the IP tail are all online work.

use flock_field::{F128, F256Unreduced};
use flock_transcript::challenger::Challenger;
use rayon::prelude::{IndexedParallelIterator, ParallelIterator, ParallelSlice};
use serde::{Deserialize, Serialize};

use super::{COEFFICIENTS, Claims, KAPPA, powers};
use crate::{
    Error, MatrixClaim,
    inner_product::{self, InnerProductProof},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    pub h_plus: Vec<F128>,
    pub h_minus: Vec<F128>,
    pub inner_product: InnerProductProof,
}
impl Proof {
    pub fn field_elements(&self) -> usize {
        self.h_plus.len() + self.h_minus.len() + 2 * self.inner_product.rounds.len() + 2
    }
}

fn shape(log_n: usize) -> Result<usize, Error> {
    if !(7..usize::BITS as usize).contains(&log_n) {
        return Err(Error::InvalidLogN(log_n));
    }
    let len = 1usize << log_n;
    if len > isize::MAX as usize / std::mem::size_of::<F128>() {
        return Err(Error::InvalidLogN(log_n));
    }
    Ok(len)
}

fn bind<C: Challenger>(log_n: usize, claim: F128, ch: &mut C) {
    ch.observe_label(b"mikros-f128-packed-ip-laurent-v1");
    ch.observe_bytes(&(log_n as u64).to_le_bytes());
    ch.observe_bytes(&(KAPPA as u64).to_le_bytes());
    ch.observe_f128(claim);
}

fn observe<C: Challenger>(proof: &Proof, ch: &mut C) {
    ch.observe_label(b"mikros-f128-packed-ip-laurent-coefficients-v1");
    ch.observe_f128_slice(&proof.h_plus);
    ch.observe_f128_slice(&proof.h_minus);
}

fn coefficients(a: &[F128], b: &[F128]) -> Vec<F128> {
    let sums = a
        .par_chunks(KAPPA * 4)
        .zip(b.par_chunks(KAPPA * 4))
        .map(|(a, b)| {
            let mut sums = vec![F256Unreduced::ZERO; 2 * KAPPA - 1];
            for (a, b) in a.chunks_exact(KAPPA).zip(b.chunks_exact(KAPPA)) {
                for (left, &a) in a.iter().enumerate() {
                    for (right, &b) in b.iter().enumerate() {
                        sums[KAPPA - 1 + left - right] ^= a.mul_unreduced(b);
                    }
                }
            }
            sums
        })
        .reduce(
            || vec![F256Unreduced::ZERO; 2 * KAPPA - 1],
            |mut a, b| {
                for (a, b) in a.iter_mut().zip(b) {
                    *a ^= b;
                }
                a
            },
        );
    sums.into_iter().map(F256Unreduced::reduce).collect()
}

fn pack(input: &[F128], weights: &[F128]) -> Vec<F128> {
    input
        .par_chunks_exact(KAPPA)
        .map(|row| {
            row.iter()
                .zip(weights)
                .fold(F256Unreduced::ZERO, |sum, (&a, &b)| {
                    sum ^ a.mul_unreduced(b)
                })
                .reduce()
        })
        .collect()
}

fn terminal(proof: &Proof, x: F128, inverse: F128, y: &[F128]) -> Claims {
    let factors: Vec<_> = y.iter().map(|&x| [F128::ONE + x, x]).collect();
    Claims {
        a: MatrixClaim {
            column_weights: powers(x),
            row_factors: factors.clone(),
            value: proof.inner_product.final_a,
        },
        b: MatrixClaim {
            column_weights: powers(inverse),
            row_factors: factors,
            value: proof.inner_product.final_b,
        },
    }
}

pub fn prove<C: Challenger>(
    a: &[F128],
    b: &[F128],
    log_n: usize,
    claim: F128,
    ch: &mut C,
) -> Result<(Proof, Claims), Error> {
    let len = shape(log_n)?;
    for (name, input) in [("a", a), ("b", b)] {
        if input.len() != len {
            return Err(Error::InputLength {
                input: name,
                expected: len,
                got: input.len(),
            });
        }
    }
    let coefficients = coefficients(a, b);
    if coefficients[KAPPA - 1] != claim {
        return Err(Error::InnerProductMismatch);
    }
    let mut proof = Proof {
        h_plus: coefficients[KAPPA..].to_vec(),
        h_minus: coefficients[..KAPPA - 1].iter().rev().copied().collect(),
        inner_product: InnerProductProof {
            rounds: Vec::new(),
            final_a: F128::ZERO,
            final_b: F128::ZERO,
        },
    };
    bind(log_n, claim, ch);
    observe(&proof, ch);
    let x = super::sample_nonzero(ch);
    let inverse = x.inv();
    let mut scratch = inner_product::ProverScratch::default();
    scratch.a = pack(a, &powers(x));
    scratch.b = pack(b, &powers(inverse));
    let (ip, y) = inner_product::prove_native(&mut scratch, None, ch);
    proof.inner_product = ip;
    let claims = terminal(&proof, x, inverse, &y);
    Ok((proof, claims))
}

pub fn verify<C: Challenger>(
    proof: &Proof,
    log_n: usize,
    claim: F128,
    ch: &mut C,
) -> Result<Claims, Error> {
    shape(log_n)?;
    if proof.h_plus.len() != COEFFICIENTS
        || proof.h_minus.len() != COEFFICIENTS
        || proof.inner_product.rounds.len() != log_n - 7
    {
        return Err(Error::ProofShape("F128 Laurent messages"));
    }
    bind(log_n, claim, ch);
    observe(proof, ch);
    let x = super::sample_nonzero(ch);
    let inverse = x.inv();
    let eval = |polynomial: &[F128], point| {
        polynomial
            .iter()
            .rev()
            .fold(F128::ZERO, |a, &b| a * point + b)
    };
    let packed = claim + x * eval(&proof.h_plus, x) + inverse * eval(&proof.h_minus, inverse);
    let y = inner_product::verify(&proof.inner_product, packed, ch)?;
    Ok(terminal(proof, x, inverse, &y))
}

/// Evaluate a terminal functional on public F128 inputs (not bit-packed input).
/// For private inputs the caller must authenticate this functional with its PCS.
pub fn evaluate(input: &[F128], query: &MatrixClaim) -> Result<F128, Error> {
    if query.column_weights.len() != KAPPA
        || query.row_factors.len() >= usize::BITS as usize - 7
        || input.len() != KAPPA * (1usize << query.row_factors.len())
    {
        return Err(Error::ProofShape("F128 Laurent terminal"));
    }
    let mut sum = F256Unreduced::ZERO;
    for (row, values) in input.chunks_exact(KAPPA).enumerate() {
        let weight = query
            .row_factors
            .iter()
            .enumerate()
            .fold(F128::ONE, |w, (i, factor)| {
                w * factor[(row >> (query.row_factors.len() - 1 - i)) & 1]
            });
        let packed = values
            .iter()
            .zip(&query.column_weights)
            .fold(F256Unreduced::ZERO, |s, (&a, &b)| s ^ a.mul_unreduced(b))
            .reduce();
        sum ^= packed.mul_unreduced(weight);
    }
    Ok(sum.reduce())
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_field::test_rng::Rng;
    use flock_hash::HashKind;
    use flock_transcript::challenger::FsChallenger;

    #[test]
    fn f128_laurent_roundtrip_and_tamper() {
        let mut rng = Rng::new(0xE17_1A07);
        let ch = || FsChallenger::with_hash(b"f128-Laurent-test", HashKind::Blake3);
        for log_n in [7, 8, 10, 15] {
            let a = rng.f128_vec(1 << log_n);
            let b = rng.f128_vec(1 << log_n);
            let claim = a.iter().zip(&b).fold(F128::ZERO, |s, (&a, &b)| s + a * b);
            let mut p = ch();
            let (proof, claims) = prove(&a, &b, log_n, claim, &mut p).unwrap();
            let mut v = ch();
            assert_eq!(verify(&proof, log_n, claim, &mut v).unwrap(), claims);
            assert_eq!(p.sample_f128(), v.sample_f128());
            assert_eq!(evaluate(&a, &claims.a).unwrap(), claims.a.value);
            assert_eq!(evaluate(&b, &claims.b).unwrap(), claims.b.value);
            assert!(prove(&a, &b, log_n, claim + F128::ONE, &mut ch()).is_err());
            assert!(verify(&proof, log_n, claim + F128::ONE, &mut ch()).is_err());
            assert!(verify(&proof, log_n + 1, claim, &mut ch()).is_err());
            for which in 0..4 {
                let mut bad = proof.clone();
                match which {
                    0 => bad.h_plus[126] += F128::ONE,
                    1 => bad.h_minus[0] += F128::ONE,
                    2 => bad.inner_product.final_a += F128::ONE,
                    _ => bad.inner_product.final_b += F128::ONE,
                }
                assert!(verify(&bad, log_n, claim, &mut ch()).is_err());
            }
            let mut bad = proof.clone();
            bad.h_plus.pop();
            assert!(verify(&bad, log_n, claim, &mut ch()).is_err());
            let mut bad = proof;
            bad.h_minus.push(F128::ZERO);
            assert!(verify(&bad, log_n, claim, &mut ch()).is_err());
        }
    }
}
