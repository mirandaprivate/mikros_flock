//! Binary Laurent packed inner product from `latex/alg/alg-ip.tex`.
//!
//! Each consecutive 128 input bits is one matrix row, matching [`crate::layout`].
//! The two correction polynomials have 127 binary coefficients each. They are
//! bound before sampling a nonzero extension-field point. The terminal queries
//! retain the actual monomial and reciprocal column weights: these weights are
//! not ordinary MLE weights at the powers of the sampled point.
//!
//! Bind the application statement and both input commitments into the challenger
//! before entry. [`verify`] returns two deferred matrix claims; both must be
//! authenticated by the caller. [`check_claims`] evaluates them directly on the
//! inputs for tests. Zero-knowledge masking and grinding are application choices.

use flock_field::{F128, F256Unreduced};
use flock_transcript::challenger::Challenger;
use rayon::prelude::{IndexedParallelIterator, ParallelIterator, ParallelSlice, ParallelSliceMut};
use serde::{Deserialize, Serialize};

pub use crate::{Error, MatrixClaim};
pub mod extension;
use crate::{
    inner_product::{self, InnerProductProof},
    layout::Shape,
    lookup::BindingTable,
};

pub const KAPPA: usize = 128;
pub const COEFFICIENTS: usize = KAPPA - 1;
const ROW_BYTES: usize = KAPPA / 8;
const COEFFICIENT_LABEL: &[u8] = b"mikros-binary-packed-ip-laurent-coefficients-v1";

/// Binary coefficients in ascending degree order, packed low bit first.
/// Bit 127 of each array is canonical zero padding and is rejected if set.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    pub h_plus: [u8; ROW_BYTES],
    pub h_minus: [u8; ROW_BYTES],
    pub inner_product: InnerProductProof,
}

impl Proof {
    /// Extension-field messages only; the 254 binary coefficients are separate.
    pub fn field_elements(&self) -> usize {
        2 * self.inner_product.rounds.len() + 2
    }

    /// Logical payload rounded to bytes, excluding serializer lengths and PCS.
    pub fn payload_bytes(&self) -> usize {
        2 * ROW_BYTES + 16 * self.field_elements()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Claims {
    pub a: MatrixClaim,
    pub b: MatrixClaim,
}

/// Retains allocation capacity only; all input and challenge values are replaced
/// on every proof. Challenge-dependent packing tables are rebuilt online.
#[derive(Default)]
pub struct ProverScratch {
    inner_product: inner_product::ProverScratch,
}

fn check_inputs(a: &[u8], b: &[u8], log_n: usize) -> Result<Shape, Error> {
    let shape = Shape::new(log_n)?;
    for (name, input) in [("a", a), ("b", b)] {
        if input.len() != shape.bytes {
            return Err(Error::InputLength {
                input: name,
                expected: shape.bytes,
                got: input.len(),
            });
        }
    }
    Ok(shape)
}

fn check_binary_claim(claim: F128) -> Result<(), Error> {
    if claim != F128::ZERO && claim != F128::ONE {
        return Err(Error::ProofShape("binary inner-product claim"));
    }
    Ok(())
}

fn bind<C: Challenger>(log_n: usize, claim: F128, challenger: &mut C) {
    challenger.observe_label(b"mikros-binary-packed-ip-laurent-v1");
    challenger.observe_bytes(&(log_n as u64).to_le_bytes());
    challenger.observe_bytes(&(KAPPA as u64).to_le_bytes());
    challenger.observe_f128(claim);
}

fn observe_coefficients<C: Challenger>(proof: &Proof, challenger: &mut C) {
    challenger.observe_label(COEFFICIENT_LABEL);
    challenger.observe_bytes(&proof.h_plus);
    challenger.observe_bytes(&proof.h_minus);
}

fn sample_nonzero<C: Challenger>(challenger: &mut C) -> F128 {
    loop {
        let point = challenger.sample_f128();
        if !point.is_zero() {
            return point;
        }
    }
}

/// Product coefficient j equals the Laurent coefficient j - 127. Interpreting
/// rows as F128 here is only a route to the existing carryless multiply kernel:
/// the 256-bit product is never reduced modulo the GHASH polynomial.
fn coefficients(a: &[u8], b: &[u8]) -> F256Unreduced {
    let sum_chunk = |(a, b): (&[u8], &[u8])| {
        let mut sum = F256Unreduced::ZERO;
        for (a, b) in a.chunks_exact(ROW_BYTES).zip(b.chunks_exact(ROW_BYTES)) {
            let a = u128::from_le_bytes(a.try_into().expect("complete row"));
            let b = u128::from_le_bytes(b.try_into().expect("complete row")).reverse_bits();
            sum ^= F128::new(a as u64, (a >> 64) as u64)
                .mul_unreduced(F128::new(b as u64, (b >> 64) as u64));
        }
        sum
    };
    if a.len() < 65536 || rayon::current_num_threads() == 1 {
        sum_chunk((a, b))
    } else {
        a.par_chunks(16384)
            .zip(b.par_chunks(16384))
            .map(sum_chunk)
            .reduce(|| F256Unreduced::ZERO, |a, b| a ^ b)
    }
}

fn coefficient(product: F256Unreduced, index: usize) -> u8 {
    let words = [product.r0, product.r1, product.r2, product.r3];
    ((words[index / 64] >> (index % 64)) & 1) as u8
}

fn powers(point: F128) -> Vec<F128> {
    let mut weights = vec![F128::ONE; KAPPA];
    for column in 1..KAPPA {
        weights[column] = weights[column - 1] * point;
    }
    weights
}

fn claims(proof: &Proof, point: F128, inverse: F128, row_point: &[F128]) -> Claims {
    let row_factors: Vec<_> = row_point.iter().map(|&x| [F128::ONE + x, x]).collect();
    Claims {
        a: MatrixClaim {
            column_weights: powers(point),
            row_factors: row_factors.clone(),
            value: proof.inner_product.final_a,
        },
        b: MatrixClaim {
            column_weights: powers(inverse),
            row_factors,
            value: proof.inner_product.final_b,
        },
    }
}

fn pack_into(input: &[u8], weights: &[F128], output: &mut Vec<F128>) {
    let table = BindingTable::new(weights);
    let rows = input.len() / ROW_BYTES;
    let uninitialized = inner_product::prepare_output(output, rows);
    if input.len() < 65536 || rayon::current_num_threads() == 1 {
        for (row, value) in input.chunks_exact(ROW_BYTES).zip(uninitialized) {
            value.write(table.fold(row));
        }
    } else {
        input
            .par_chunks(16384)
            .zip(uninitialized.par_chunks_mut(1024))
            .for_each(|(input, output)| {
                for (row, value) in input.chunks_exact(ROW_BYTES).zip(output) {
                    value.write(table.fold(row));
                }
            });
    }
    // SAFETY: each disjoint output slot was initialized once above, all Rayon
    // tasks have joined, and Shape guarantees a whole number of complete rows.
    unsafe { output.set_len(rows) };
}

/// Prove `<a,b> = claim` over GF(2), with `claim` canonically zero or one.
/// The returned source claims still require commitment authentication.
pub fn prove<C: Challenger>(
    a: &[u8],
    b: &[u8],
    log_n: usize,
    claim: F128,
    challenger: &mut C,
) -> Result<(Proof, Claims), Error> {
    prove_with_scratch(
        a,
        b,
        log_n,
        claim,
        challenger,
        &mut ProverScratch::default(),
    )
}

pub fn prove_with_scratch<C: Challenger>(
    a: &[u8],
    b: &[u8],
    log_n: usize,
    claim: F128,
    challenger: &mut C,
    scratch: &mut ProverScratch,
) -> Result<(Proof, Claims), Error> {
    check_inputs(a, b, log_n)?;
    check_binary_claim(claim)?;
    let product = coefficients(a, b);
    if F128::new(coefficient(product, KAPPA - 1) as u64, 0) != claim {
        return Err(Error::InnerProductMismatch);
    }
    let mut proof = Proof {
        h_plus: [0; ROW_BYTES],
        h_minus: [0; ROW_BYTES],
        inner_product: InnerProductProof {
            rounds: Vec::new(),
            final_a: F128::ZERO,
            final_b: F128::ZERO,
        },
    };
    for degree in 0..COEFFICIENTS {
        proof.h_plus[degree / 8] |= coefficient(product, KAPPA + degree) << (degree % 8);
        proof.h_minus[degree / 8] |= coefficient(product, KAPPA - 2 - degree) << (degree % 8);
    }
    bind(log_n, claim, challenger);
    observe_coefficients(&proof, challenger);
    let point = sample_nonzero(challenger);
    let inverse = point.inv();
    pack_into(a, &powers(point), &mut scratch.inner_product.a);
    pack_into(b, &powers(inverse), &mut scratch.inner_product.b);
    let (inner_product, row_point) =
        inner_product::prove_native(&mut scratch.inner_product, None, challenger);
    proof.inner_product = inner_product;
    let claims = claims(&proof, point, inverse, &row_point);
    Ok((proof, claims))
}

fn eval_correction(coefficients: &[u8; ROW_BYTES], point: F128) -> F128 {
    (0..COEFFICIENTS).rev().fold(F128::ZERO, |value, degree| {
        value * point + F128::new(((coefficients[degree / 8] >> (degree % 8)) & 1) as u64, 0)
    })
}

/// Replay the reduction, returning exactly two deferred source queries. The
/// verifier does not accept input vectors or an untrusted prover-supplied point.
pub fn verify<C: Challenger>(
    proof: &Proof,
    log_n: usize,
    claim: F128,
    challenger: &mut C,
) -> Result<Claims, Error> {
    let shape = Shape::new(log_n)?;
    check_binary_claim(claim)?;
    if proof.h_plus[ROW_BYTES - 1] & 0x80 != 0 || proof.h_minus[ROW_BYTES - 1] & 0x80 != 0 {
        return Err(Error::ProofShape("Laurent coefficient padding"));
    }
    if proof.inner_product.rounds.len() != shape.row_log {
        return Err(Error::ProofShape("rounds"));
    }
    bind(log_n, claim, challenger);
    observe_coefficients(proof, challenger);
    let point = sample_nonzero(challenger);
    let inverse = point.inv();
    let packed_claim = claim
        + point * eval_correction(&proof.h_plus, point)
        + inverse * eval_correction(&proof.h_minus, inverse);
    let row_point = inner_product::verify(&proof.inner_product, packed_claim, challenger)?;
    Ok(claims(proof, point, inverse, &row_point))
}

/// Linear-time source evaluation for tests.
pub fn check_claims(a: &[u8], b: &[u8], log_n: usize, claims: &Claims) -> Result<(), Error> {
    let shape = check_inputs(a, b, log_n)?;
    for (name, input, query) in [("a", a, &claims.a), ("b", b, &claims.b)] {
        if query.column_weights.len() != KAPPA || query.row_factors.len() != shape.row_log {
            return Err(Error::ProofShape("source query"));
        }
        let mut value = F128::ZERO;
        for (row, bytes) in input.chunks_exact(ROW_BYTES).enumerate() {
            let row_weight = query.row_factors.iter().enumerate().fold(
                F128::ONE,
                |weight, (coordinate, factor)| {
                    weight * factor[(row >> (shape.row_log - 1 - coordinate)) & 1]
                },
            );
            let row_value = (0..KAPPA).fold(F128::ZERO, |value, column| {
                if bytes[column / 8] >> (column % 8) & 1 == 1 {
                    value + query.column_weights[column]
                } else {
                    value
                }
            });
            value += row_weight * row_value;
        }
        if value != query.value {
            return Err(Error::SourceMismatch(name));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carryless_coefficients_match_all_column_pairs_without_reduction() {
        use flock_field::test_rng::Rng;
        let mut rng = Rng::new(0x1A0_7E17);
        for rows in [1, 2, 9] {
            let mut a = vec![0; ROW_BYTES * rows];
            let mut b = a.clone();
            rng.fill_bytes(&mut a);
            rng.fill_bytes(&mut b);
            let actual = coefficients(&a, &b);
            let mut expected = [0u8; 255];
            for (a, b) in a.chunks_exact(ROW_BYTES).zip(b.chunks_exact(ROW_BYTES)) {
                for left in 0..KAPPA {
                    for right in 0..KAPPA {
                        expected[127 + left - right] ^=
                            (a[left / 8] >> (left % 8)) & (b[right / 8] >> (right % 8)) & 1;
                    }
                }
            }
            for (index, &expected) in expected.iter().enumerate() {
                assert_eq!(
                    coefficient(actual, index),
                    expected,
                    "rows={rows}, index={index}"
                );
            }
            assert_eq!(coefficient(actual, 255), 0);
        }
        // The extreme products exercise coefficients outside F128's degree
        // range; accidental GHASH reduction would destroy these positions.
        for (left, right, index) in [(0, 127, 0), (127, 0, 254), (63, 63, 127)] {
            let mut a = [0; ROW_BYTES];
            let mut b = [0; ROW_BYTES];
            a[left / 8] = 1 << (left % 8);
            b[right / 8] = 1 << (right % 8);
            let actual = coefficients(&a, &b);
            for degree in 0..256 {
                assert_eq!(coefficient(actual, degree), u8::from(degree == index));
            }
        }
    }
}
