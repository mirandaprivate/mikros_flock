//! The field isomorphism from FLOCK's GHASH coordinates to the WHIR tower field.
//!
//! Both fields have 128 bits, but their bit representations are different.
//! In particular, constructing a tower element directly from a GHASH word is
//! incorrect. The upstream polynomial basis is exactly FLOCK's basis modulo
//! `X^128 + X^7 + X^2 + X + 1`.

use flock_field::F128;
use p3_binary_field::{BinaryField128, TowerLevel, poly_basis};
use p3_field::PrimeCharacteristicRing;

/// A conversion task holds at most 64 KiB of polynomial/tower coordinate scratch.
/// The multiple of 64 also lets the upstream GFNI kernel process complete blocks.
const CHUNK_WORDS: usize = 4096;

#[inline]
fn polynomial_bits(value: F128) -> u128 {
    value.lo as u128 | ((value.hi as u128) << 64)
}

#[inline]
pub(crate) fn to_native(value: F128) -> BinaryField128 {
    poly_basis::to_tower(polynomial_bits(value))
}

#[inline]
pub(crate) fn from_native(value: BinaryField128) -> F128 {
    let bits = poly_basis::from_tower(value);
    F128::new(bits as u64, (bits >> 64) as u64)
}

fn convert_chunk(scratch: &mut Vec<u128>, source: &[F128], output: &mut [BinaryField128]) {
    debug_assert_eq!(source.len(), output.len());
    debug_assert!(source.len() <= CHUNK_WORDS);
    scratch.clear();
    scratch.extend(source.iter().copied().map(polynomial_bits));
    // This changes the coordinates, not the represented field elements. It
    // includes the upstream scalar tail path on lengths not divisible by 64.
    poly_basis::to_tower_slice(scratch);
    for (out, &tower_bits) in output.iter_mut().zip(scratch.iter()) {
        *out = BinaryField128::from_repr(tower_bits);
    }
}

/// Convert the complete packed-word table without a second full-length buffer.
///
/// The conversion preserves word order. The original raw 128 witness bits are
/// first interpreted as one GHASH field element, then carried through the field
/// isomorphism; this is not 128 separate subfield embeddings.
///
/// With `parallel`, each active Rayon task reuses at most 64 KiB of scratch.
/// SIMD availability is determined by the upstream crate's compilation target.
pub(crate) fn convert_words(words: &[F128]) -> Vec<BinaryField128> {
    let mut output = vec![BinaryField128::ZERO; words.len()];
    #[cfg(feature = "parallel")]
    {
        use rayon::prelude::*;
        words
            .par_chunks(CHUNK_WORDS)
            .zip(output.par_chunks_mut(CHUNK_WORDS))
            .for_each_init(
                || Vec::with_capacity(CHUNK_WORDS),
                |scratch, (source, destination)| convert_chunk(scratch, source, destination),
            );
    }
    #[cfg(not(feature = "parallel"))]
    {
        let mut scratch = Vec::with_capacity(CHUNK_WORDS);
        for (source, destination) in words
            .chunks(CHUNK_WORDS)
            .zip(output.chunks_mut(CHUNK_WORDS))
        {
            convert_chunk(&mut scratch, source, destination);
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use p3_field::Field;

    fn raw(bits: u128) -> F128 {
        F128::new(bits as u64, (bits >> 64) as u64)
    }

    fn next(seed: &mut u128) -> u128 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed
    }

    #[test]
    fn constants_all_basis_elements_and_modulus_crossing() {
        assert_eq!(to_native(F128::ZERO), BinaryField128::ZERO);
        assert_eq!(to_native(F128::ONE), BinaryField128::ONE);
        assert_eq!(from_native(BinaryField128::ZERO), F128::ZERO);
        assert_eq!(from_native(BinaryField128::ONE), F128::ONE);
        for bit in 0..128 {
            let value = raw(1u128 << bit);
            assert_eq!(from_native(to_native(value)), value, "bit {bit}");
        }
        let crossing = to_native(raw(1u128 << 127)) * to_native(raw(2));
        assert_eq!(from_native(crossing), raw(0x87));
        // A representation-only reinterpretation must not silently replace the
        // conversion: the tower's bit one is not the polynomial indeterminate.
        assert_ne!(to_native(raw(2)), BinaryField128::from_repr(2));
    }

    #[test]
    fn random_products_squares_and_inverses_match_flock() {
        let mut seed = 0xa712_d09e_123f_ecc1_b97a_62d3_4319_2341;
        for _ in 0..1024 {
            let a = raw(next(&mut seed));
            let b = raw(next(&mut seed));
            let native_a = to_native(a);
            let native_b = to_native(b);
            assert_eq!(from_native(native_a), a);
            assert_eq!(from_native(native_a + native_b), a + b);
            assert_eq!(from_native(native_a * native_b), a * b);
            assert_eq!(from_native(native_a.square()), a.square());
            if a != F128::ZERO {
                let inverse = native_a.try_inverse().unwrap();
                assert_eq!(a * from_native(inverse), F128::ONE);
                assert_eq!(to_native(from_native(inverse)), inverse);
            }
        }
        assert_eq!(BinaryField128::ZERO.try_inverse(), None);
    }

    fn check_batch_lengths() {
        let mut seed = 0x6827_6149_afdb_98e1_6b43_2155_9999_7777;
        for len in [
            0,
            1,
            2,
            63,
            64,
            65,
            127,
            128,
            129,
            CHUNK_WORDS - 1,
            CHUNK_WORDS,
            CHUNK_WORDS + 1,
            3 * CHUNK_WORDS + 65,
        ] {
            let mut words: Vec<_> = (0..len).map(|_| raw(next(&mut seed))).collect();
            for (slot, bits) in words
                .iter_mut()
                .zip([0, 1, 1u128 << 127, u128::MAX, 1u128 << 64])
            {
                *slot = raw(bits);
            }
            let converted = convert_words(&words);
            assert_eq!(converted.len(), words.len());
            for (&actual, &expected) in converted.iter().zip(&words) {
                assert_eq!(actual, to_native(expected), "len {len}");
                assert_eq!(from_native(actual), expected, "len {len}");
            }
        }
    }

    #[test]
    fn batch_conversion_matches_scalar_with_all_tails() {
        check_batch_lengths();
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn batch_conversion_matches_with_one_and_four_workers() {
        for workers in [1, 4] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap()
                .install(check_batch_lengths);
        }
    }
}
