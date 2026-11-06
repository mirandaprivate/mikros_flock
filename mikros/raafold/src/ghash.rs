//! Exact field and coordinate adapter between Mikros and RAAFold.
//!
//! Mikros uses ordinary polynomial multiplication modulo
//! `g(X) = X^128 + X^7 + X^2 + X + 1`, with coefficient `X^i` in bit `i`.
//! RAAFold's RustCrypto field uses POLYVAL's `dot` multiplication modulo
//! `p(Y) = Y^128 + Y^127 + Y^126 + Y^121 + 1`:
//! `u dot v = u*v*Y^-128`. In particular its raw multiplicative identity is
//! `Y^128 mod p`, not the integer bit pattern one.
//!
//! Since `p` is the reciprocal polynomial of `g`, the exact isomorphism is
//! `phi(a) = Y^128 * a(Y^-1) = Y * reverse_bits_128(a) mod p`.
//! It is a fixed bit reversal followed by one shift and conditional XOR.
//! Copying the raw 128 bits unchanged would not preserve multiplication.
//!
//! Packed witness conversion treats each original Mikros `F128` word as one
//! logical RAAFold element of width **128**. It preserves vector order and
//! the column-major layout. It does not reinterpret an original packed word
//! as 128 independently opened one-bit logical elements.
//!
//! Point conversion only maps the field and reverses LSB-first coordinates
//! to MSB-first coordinates. It is for the RAAFold **equality-basis** opening
//! entry. The original homogeneous `(1, x)` entry is a different contract;
//! merely changing the field and coordinate order does not convert its basis.

use crate::raafold_field::RustCryptoPolyvalField;
use flock_field::F128;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// The low 128 coefficients of the POLYVAL modulus, also POLYVAL `dot`'s one.
const POLYVAL_REDUCTION: u128 = 0xc200_0000_0000_0000_0000_0000_0000_0001;
/// The correction after division by `Y`, including the recovered high bit.
const POLYVAL_DIVIDE_Y: u128 = 0xe100_0000_0000_0000_0000_0000_0000_0000;

/// Maps GHASH polynomial coefficient bits to RustCrypto POLYVAL `dot` bits.
#[inline]
pub const fn ghash_bits_to_polyval_bits(value: u128) -> u128 {
    let reversed = value.reverse_bits();
    (reversed << 1) ^ (POLYVAL_REDUCTION & 0u128.wrapping_sub(reversed >> 127))
}

/// Inverts [`ghash_bits_to_polyval_bits`] for every 128-bit pattern.
#[inline]
pub const fn polyval_bits_to_ghash_bits(value: u128) -> u128 {
    let divided = (value >> 1) ^ (POLYVAL_DIVIDE_Y & 0u128.wrapping_sub(value & 1));
    divided.reverse_bits()
}

/// Applies the field isomorphism to a Mikros value, challenge, or claim.
#[inline]
pub fn to_polyval(value: F128) -> RustCryptoPolyvalField {
    let bits = u128::from(value.lo) | (u128::from(value.hi) << 64);
    RustCryptoPolyvalField::new(ghash_bits_to_polyval_bits(bits))
}

/// Applies the inverse field isomorphism to a RAAFold value or challenge.
#[inline]
pub fn from_polyval(value: RustCryptoPolyvalField) -> F128 {
    let bits = polyval_bits_to_ghash_bits(value.val());
    F128::new(bits as u64, (bits >> 64) as u64)
}

/// Converts original Mikros field words to RAAFold's logical-width-128 input.
///
/// The returned array retains the exact input vector order. In particular,
/// no row/column transpose and no bit-lane expansion is performed.
pub fn to_packed_words(values: &[F128]) -> Vec<u128> {
    #[cfg(feature = "parallel")]
    if values.len() >= 4096 {
        return values.par_iter().map(|&value| to_polyval(value).val()).collect();
    }
    values.iter().map(|&value| to_polyval(value).val()).collect()
}

/// Converts GHASH coefficient words in place, avoiding a second full vector.
///
/// The input must contain the raw coefficient bits of one original Mikros
/// `F128` per word. After this call the words are ready for the RAAFold packed
/// witness API with `logical_element_bits = 128`.
pub fn convert_packed_words_in_place(words: &mut [u128]) {
    #[cfg(feature = "parallel")]
    if words.len() >= 4096 {
        words.par_iter_mut().for_each(|word| *word = ghash_bits_to_polyval_bits(*word));
        return;
    }
    for word in words {
        *word = ghash_bits_to_polyval_bits(*word);
    }
}

/// Maps a Boolean MLE point from Mikros LSB-first to RAAFold MSB-first order.
///
/// Both sides use factors `(1 + r, r)`; this function requires RAAFold's
/// equality-basis opening. Coordinates zero and one are handled exactly and
/// need no inversion, resampling, or special-case normalization.
pub fn boolean_point_lsb_to_msb(point: &[F128]) -> Vec<RustCryptoPolyvalField> {
    point.iter().rev().copied().map(to_polyval).collect()
}

/// Inverts [`boolean_point_lsb_to_msb`] without changing the equality basis.
pub fn boolean_point_msb_to_lsb(point: &[RustCryptoPolyvalField]) -> Vec<F128> {
    point.iter().rev().copied().map(from_polyval).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use binius_field::{Field, arithmetic_traits::InvertOrZero};

    fn from_bits(bits: u128) -> F128 {
        F128::new(bits as u64, (bits >> 64) as u64)
    }

    fn next(state: &mut u128) -> u128 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *state
    }

    fn equality_weight_msb(point: &[RustCryptoPolyvalField], index: usize) -> RustCryptoPolyvalField {
        point.iter().enumerate().fold(RustCryptoPolyvalField::ONE, |product, (i, &r)| {
            product * if (index >> (point.len() - 1 - i)) & 1 == 0 {
                RustCryptoPolyvalField::ONE + r
            } else {
                r
            }
        })
    }

    // Independent evaluation using adjacent-pair folds instead of the direct
    // tensor products used by the POLYVAL oracle below.
    fn ghash_mle_lsb(values: &[F128], point: &[F128]) -> F128 {
        assert_eq!(values.len(), 1usize << point.len());
        let mut folded = values.to_vec();
        for &r in point {
            for i in 0..folded.len() / 2 {
                folded[i] = folded[2 * i] * (F128::ONE + r) + folded[2 * i + 1] * r;
            }
            folded.truncate(folded.len() / 2);
        }
        folded[0]
    }

    #[test]
    fn maps_zero_one_generator_and_all_basis_vectors() {
        assert_eq!(to_polyval(F128::ZERO), RustCryptoPolyvalField::ZERO);
        assert_eq!(to_polyval(F128::ONE), RustCryptoPolyvalField::ONE);
        assert_eq!(to_polyval(F128::generator()).val(), 1u128 << 127);
        assert_eq!(from_polyval(RustCryptoPolyvalField::ONE), F128::ONE);
        assert_ne!(to_polyval(F128::ONE).val(), 1);
        for bit in 0..128 {
            let raw = 1u128 << bit;
            assert_eq!(polyval_bits_to_ghash_bits(ghash_bits_to_polyval_bits(raw)), raw);
            assert_eq!(ghash_bits_to_polyval_bits(polyval_bits_to_ghash_bits(raw)), raw);
            assert_eq!(from_polyval(to_polyval(from_bits(raw))), from_bits(raw));
        }
    }

    #[test]
    fn multiplication_matches_on_every_pair_of_binary_basis_vectors() {
        // Since conversion and both field multiplications are F2-linear in
        // each operand, this covers the complete multiplication table basis.
        for left in 0..128 {
            let a = from_bits(1u128 << left);
            for right in 0..128 {
                let b = from_bits(1u128 << right);
                assert_eq!(to_polyval(a * b), to_polyval(a) * to_polyval(b), "basis {left}, {right}");
            }
        }
    }

    #[test]
    fn random_addition_multiplication_and_roundtrip_match() {
        let mut state = 0x0123_4567_89ab_cdef_fedc_ba98_7654_3210;
        for _ in 0..2048 {
            let a = from_bits(next(&mut state));
            let b = from_bits(next(&mut state));
            assert_eq!(to_polyval(a + b), to_polyval(a) + to_polyval(b));
            assert_eq!(to_polyval(a * b), to_polyval(a) * to_polyval(b));
            assert_eq!(from_polyval(to_polyval(a)), a);
            let arbitrary_polyval = RustCryptoPolyvalField::new(next(&mut state));
            assert_eq!(to_polyval(from_polyval(arbitrary_polyval)), arbitrary_polyval);
        }
    }

    #[test]
    fn nonzero_inverses_are_preserved() {
        let mut state = 0xdead_beef_1234_5678_fedc_ba98_1357_2468;
        for _ in 0..32 {
            let a = from_bits(next(&mut state));
            assert_ne!(a, F128::ZERO);
            let inverse = to_polyval(a).invert_or_zero();
            assert_eq!(to_polyval(a.inv()), inverse);
            assert_eq!(to_polyval(a) * inverse, RustCryptoPolyvalField::ONE);
            assert_eq!(a * from_polyval(inverse), F128::ONE);
        }
    }

    #[test]
    fn packed_word_conversion_preserves_order_and_matches_in_place() {
        let mut state = 0xa15e_cafe_9876_5432_10fe_dcba_7654_3210;
        // Covers the parallel conversion threshold when that feature is on.
        let raw: Vec<_> = (0..8193).map(|_| next(&mut state)).collect();
        let source: Vec<_> = raw.iter().copied().map(from_bits).collect();
        let mapped = to_packed_words(&source);
        let mut in_place = raw.clone();
        convert_packed_words_in_place(&mut in_place);
        assert_eq!(mapped, in_place);
        for (original, converted) in raw.into_iter().zip(mapped) {
            assert_eq!(polyval_bits_to_ghash_bits(converted), original);
        }
    }

    #[test]
    fn equality_opening_matches_original_words_and_column_major_layout() {
        let mut state = 0x5eed_51ab_ef01_2345_fedc_ba98_7610_4321;
        for row_log in 0..=4 {
            for column_log in 0..=3 {
                let log = row_log + column_log;
                let values: Vec<_> = (0..1usize << log).map(|_| from_bits(next(&mut state))).collect();
                for pinned in [false, true] {
                    let lsb: Vec<_> = (0..log).map(|i| {
                        if pinned && i % 3 == 0 { F128::ZERO }
                        else if pinned && i % 3 == 1 { F128::ONE }
                        else { from_bits(next(&mut state)) }
                    }).collect();
                    let point = boolean_point_lsb_to_msb(&lsb);
                    assert_eq!(boolean_point_msb_to_lsb(&point), lsb);
                    let expected = to_polyval(ghash_mle_lsb(&values, &lsb));
                    let converted = to_packed_words(&values);
                    let direct = converted.iter().enumerate().fold(RustCryptoPolyvalField::ZERO, |sum, (index, &word)| {
                        sum + RustCryptoPolyvalField::new(word) * equality_weight_msb(&point, index)
                    });
                    assert_eq!(direct, expected);
                    // RAAFold splits MSB coordinates into columns then rows.
                    // A flat original vector is already column-major for this
                    // split, so no data permutation is needed at width 128.
                    let rows = 1usize << row_log;
                    let by_column = (0..1usize << column_log).fold(RustCryptoPolyvalField::ZERO, |sum, column| {
                        let row_projection = (0..rows).fold(RustCryptoPolyvalField::ZERO, |partial, row| {
                            partial + RustCryptoPolyvalField::new(converted[column * rows + row])
                                * equality_weight_msb(&point[column_log..], row)
                        });
                        sum + row_projection * equality_weight_msb(&point[..column_log], column)
                    });
                    assert_eq!(by_column, expected);
                }
            }
        }
    }
}
