//! Eight binary rows at a time, using the original GF256 weight coordinates.
//!
//! GF2P8AFFINEQB computes output bit i from matrix byte 7-i (Intel SDM).
//! A subset byte selects rows; the affine map XORs precisely their weights.

use std::{
    arch::x86_64::{
        __m128i, __m512i, _mm_loadu_si128, _mm_unpackhi_epi8, _mm_unpackhi_epi16,
        _mm_unpackhi_epi32, _mm_unpacklo_epi8, _mm_unpacklo_epi16, _mm_unpacklo_epi32,
        _mm512_castsi128_si512, _mm512_gf2p8affine_epi64_epi8, _mm512_inserti32x4,
        _mm512_loadu_si512, _mm512_maskz_mov_epi8, _mm512_set_epi64, _mm512_set1_epi8,
        _mm512_set1_epi64, _mm512_shuffle_epi8, _mm512_storeu_si512, _mm512_xor_si512,
    },
    mem::MaybeUninit,
};

/// Transpose eight 16-byte rows, then transpose the bits of each 8x8 block.
/// Output byte 8*c+b contains bit b of input column c across the eight rows.
/// MATRIX reverses output bytes within each qword to directly encode a GFNI map.
///
/// # Safety
/// `input` provides 128 readable bytes. The declared ISA is available.
#[inline]
#[target_feature(enable = "gfni,avx512f,avx512bw")]
unsafe fn subset_columns<const MATRIX: bool>(input: *const u8) -> [__m512i; 2] {
    // SAFETY: all eight unaligned loads stay within the supplied 128 bytes.
    unsafe {
        let rows: [__m128i; 8] = std::array::from_fn(|i| _mm_loadu_si128(input.add(16 * i).cast()));
        let mut columns = [MaybeUninit::<__m128i>::uninit(); 8];
        for half in 0..2 {
            let pairs: [__m128i; 4] = std::array::from_fn(|i| {
                if half == 0 {
                    _mm_unpacklo_epi8(rows[2 * i], rows[2 * i + 1])
                } else {
                    _mm_unpackhi_epi8(rows[2 * i], rows[2 * i + 1])
                }
            });
            let low0 = _mm_unpacklo_epi16(pairs[0], pairs[1]);
            let high0 = _mm_unpackhi_epi16(pairs[0], pairs[1]);
            let low1 = _mm_unpacklo_epi16(pairs[2], pairs[3]);
            let high1 = _mm_unpackhi_epi16(pairs[2], pairs[3]);
            columns[4 * half].write(_mm_unpacklo_epi32(low0, low1));
            columns[4 * half + 1].write(_mm_unpackhi_epi32(low0, low1));
            columns[4 * half + 2].write(_mm_unpacklo_epi32(high0, high1));
            columns[4 * half + 3].write(_mm_unpackhi_epi32(high0, high1));
        }
        // Every 128-bit slot above is initialized before constructing vectors.
        let columns = columns.map(|column| column.assume_init());
        let powers = _mm512_set1_epi64(if MATRIX {
            0x0102_0408_1020_4080
        } else {
            0x8040_2010_0804_0201u64 as i64
        });
        let reverse = _mm512_set1_epi64(0x0001_0203_0405_0607);
        std::array::from_fn(|half| {
            let offset = 4 * half;
            let value = _mm512_castsi128_si512(columns[offset]);
            let value = _mm512_inserti32x4::<1>(value, columns[offset + 1]);
            let value = _mm512_inserti32x4::<2>(value, columns[offset + 2]);
            let value = _mm512_inserti32x4::<3>(value, columns[offset + 3]);
            // pshufb indices restart in each 128-bit lane: the second qword
            // must select bytes 15..8, not the first qword's bytes 7..0.
            let reverse = _mm512_xor_si512(
                reverse,
                _mm512_set_epi64(
                    0x0808_0808_0808_0808,
                    0,
                    0x0808_0808_0808_0808,
                    0,
                    0x0808_0808_0808_0808,
                    0,
                    0x0808_0808_0808_0808,
                    0,
                ),
            );
            _mm512_gf2p8affine_epi64_epi8::<0>(powers, _mm512_shuffle_epi8(value, reverse))
        })
    }
}

/// XOR the H samples only; continuation samples and the padding lane survive.
///
/// # Safety
/// Requires GFNI and AVX512F/BW, and matching coordinate/mask slice lengths.
#[inline(never)]
#[target_feature(enable = "gfni,avx512f,avx512bw")]
pub(super) unsafe fn consume_h_affine(
    sums: &mut [[u8; 256]; 16],
    coordinates: &[[u8; 16]],
    masks: &[[u64; 2]],
) {
    debug_assert_eq!(coordinates.len(), masks.len());
    let bulk = coordinates.len() / 8 * 8;
    // SAFETY: x86 is little endian; eight adjacent mask/coordinate rows each
    // occupy exactly 128 bytes. Output accesses are complete H sample vectors.
    unsafe {
        for start in (0..bulk).step_by(8) {
            let indices = subset_columns::<false>(masks.as_ptr().add(start).cast());
            let transposed = subset_columns::<true>(coordinates.as_ptr().add(start).cast());
            let mut matrices = MaybeUninit::<[u64; 16]>::uninit();
            let ptr = matrices.as_mut_ptr().cast::<u64>();
            _mm512_storeu_si512(ptr.cast(), transposed[0]);
            _mm512_storeu_si512(ptr.add(8).cast(), transposed[1]);
            let matrices = matrices.assume_init();
            for (sum, matrix) in sums.iter_mut().zip(matrices) {
                let matrix = _mm512_set1_epi64(matrix as i64);
                for (half, &index) in indices.iter().enumerate() {
                    let ptr = sum.as_mut_ptr().add(64 * half).cast();
                    let old = _mm512_loadu_si512(ptr);
                    let value = _mm512_gf2p8affine_epi64_epi8::<0>(index, matrix);
                    _mm512_storeu_si512(ptr, _mm512_xor_si512(old, value));
                }
            }
        }
        for (coordinate, mask) in coordinates[bulk..].iter().zip(&masks[bulk..]) {
            for (sum, &weight) in sums.iter_mut().zip(coordinate) {
                let scalar = _mm512_set1_epi8(weight as i8);
                for (half, &mask) in mask.iter().enumerate() {
                    let ptr = sum.as_mut_ptr().add(64 * half).cast();
                    let old = _mm512_loadu_si512(ptr);
                    _mm512_storeu_si512(
                        ptr,
                        _mm512_xor_si512(old, _mm512_maskz_mov_epi8(mask, scalar)),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_field::test_rng::Rng;

    #[test]
    fn affine_h_matches_bitwise_weights_all_subsets_tails_and_accumulation() {
        if !crate::univariate::has_gfni() {
            return;
        }
        let mut rng = Rng::new(0xA881_E008);
        for rows in [0, 1, 7, 8, 9, 15, 16, 63, 64, 65] {
            let mut coordinates = vec![[0u8; 16]; rows];
            let mut masks = vec![[0u64; 2]; rows];
            for coordinate in &mut coordinates {
                rng.fill_bytes(coordinate);
            }
            for mask in &mut masks {
                let mut bytes = [0; 16];
                rng.fill_bytes(&mut bytes);
                *mask = std::array::from_fn(|h| {
                    u64::from_le_bytes(bytes[8 * h..8 * h + 8].try_into().unwrap())
                });
            }
            check(&coordinates, &masks);
        }
        // Across two cases every subset byte occurs for every coordinate.
        for high in [0usize, 128] {
            let coordinates: Vec<[u8; 16]> = (0..8)
                .map(|row| {
                    std::array::from_fn(|coordinate| {
                        (1u8 << ((row + coordinate) % 8)).rotate_left((coordinate / 8) as u32)
                    })
                })
                .collect();
            let masks: Vec<[u64; 2]> = (0..8)
                .map(|row| {
                    std::array::from_fn(|half| {
                        (0..64).fold(0, |mask, bit| {
                            mask | ((((high + 64 * half + bit) >> row) & 1) as u64) << bit
                        })
                    })
                })
                .collect();
            check(&coordinates, &masks);
        }
    }

    fn check(coordinates: &[[u8; 16]], masks: &[[u64; 2]]) {
        let mut expected: [[u8; 256]; 16] =
            std::array::from_fn(|c| std::array::from_fn(|s| (c * 17 + s) as u8));
        let mut actual = expected;
        for (coordinate, mask) in coordinates.iter().zip(masks) {
            for (sum, &weight) in expected.iter_mut().zip(coordinate) {
                for (sample, out) in sum[..128].iter_mut().enumerate() {
                    if mask[sample / 64] >> (sample % 64) & 1 != 0 {
                        *out ^= weight;
                    }
                }
            }
        }
        // SAFETY: feature check above, matching slices; tests include tails.
        unsafe { consume_h_affine(&mut actual, coordinates, masks) };
        assert_eq!(actual, expected);
    }
}
