//! Candidate A: batch the fixed GF(2^32) embedding with GFNI before a native
//! unreduced dot product. Only a 64-row, at most 16-KiB local tile is expanded.

#[cfg(any(
    test,
    all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq"
    )
))]
use crate::field::FastField;
use flock_field::F128;
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512f",
    target_feature = "vpclmulqdq"
))]
use std::sync::OnceLock;

#[cfg(any(
    test,
    all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq"
    )
))]
type Matrices = [[u64; 4]; 16];

#[cfg(any(
    test,
    all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq"
    )
))]
fn make_matrices(basis: &[F128]) -> Matrices {
    let mut maps = [[0u64; 4]; 16];
    for input in 0..4 {
        for bit in 0..8 {
            let v = basis[input * 8 + bit];
            let bytes = (u128::from(v.lo) | (u128::from(v.hi) << 64)).to_le_bytes();
            for output in 0..16 {
                for output_bit in 0..8 {
                    maps[output][input] |= u64::from((bytes[output] >> output_bit) & 1)
                        << (8 * (7 - output_bit) + bit);
                }
            }
        }
    }
    maps
}

fn available() -> bool {
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq"
    ))]
    {
        std::is_x86_feature_detected!("gfni") && std::is_x86_feature_detected!("avx512bw")
    }
    #[cfg(not(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq"
    )))]
    {
        false
    }
}

/// `bytes` contains rows of little-endian u32 coefficients, without padding.
/// Return one weighted extension-field sum per coefficient. Unsupported shapes
/// or instruction sets return None so the existing implementation remains usable.
pub(super) fn block(
    bytes: &[u8],
    coefficients_per_row: usize,
    basis: &[F128],
    weights: &[F128],
) -> Option<Vec<F128>> {
    if !matches!(coefficients_per_row, 6 | 8 | 12 | 16)
        || basis.len() != 32
        || weights
            .len()
            .checked_mul(coefficients_per_row)
            .and_then(|n| n.checked_mul(4))
            != Some(bytes.len())
        || !available()
    {
        return None;
    }
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq"
    ))]
    {
        // This is fixed public field data, cached across all equality blocks.
        // A caller-supplied test basis is kept separate from the canonical cache.
        static MAPS: OnceLock<Matrices> = OnceLock::new();
        let canonical = FastField::for_width(32).unwrap().base.basis();
        let local;
        let maps = if basis == canonical {
            MAPS.get_or_init(|| make_matrices(canonical))
        } else {
            local = make_matrices(basis);
            &local
        };
        // SAFETY: build-time and runtime checks cover every declared feature;
        // shape checks provide exactly C u32 inputs per row and weight.
        Some(unsafe {
            match coefficients_per_row {
                6 => native::block::<6>(bytes, basis, maps, weights),
                8 => native::block::<8>(bytes, basis, maps, weights),
                12 => native::block::<12>(bytes, basis, maps, weights),
                16 => native::block::<16>(bytes, basis, maps, weights),
                _ => unreachable!(),
            }
        })
    }
    #[cfg(not(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq"
    )))]
    {
        None
    }
}

#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512f",
    target_feature = "vpclmulqdq"
))]
mod native {
    use super::{F128, Matrices};
    use flock_field::gf2_128::x86_64::{WideGhashX4, f128x4_set};
    use std::{arch::x86_64::*, mem::MaybeUninit};

    const fn shuffle(byte: u8) -> [u8; 64] {
        let mut indices = [0x80; 64];
        let mut lane = 0;
        while lane < 4 {
            let mut row = 0;
            while row < 4 {
                indices[lane * 16 + row] = byte + 4 * row as u8;
                row += 1;
            }
            lane += 1;
        }
        indices
    }
    const SHUFFLES: [[u8; 64]; 4] = [shuffle(0), shuffle(1), shuffle(2), shuffle(3)];

    #[inline]
    #[target_feature(enable = "avx512f,avx512bw")]
    unsafe fn transpose(mut rows: [__m512i; 16]) -> [__m512i; 16] {
        let mut next = [_mm512_setzero_si512(); 16];
        for i in (0..16).step_by(2) {
            next[i] = _mm512_unpacklo_epi8(rows[i], rows[i + 1]);
            next[i + 1] = _mm512_unpackhi_epi8(rows[i], rows[i + 1]);
        }
        rows = next;
        for group in (0..16).step_by(4) {
            for i in 0..2 {
                next[group + 2 * i] = _mm512_unpacklo_epi16(rows[group + i], rows[group + 2 + i]);
                next[group + 2 * i + 1] =
                    _mm512_unpackhi_epi16(rows[group + i], rows[group + 2 + i]);
            }
        }
        rows = next;
        for group in (0..16).step_by(8) {
            for i in 0..4 {
                next[group + 2 * i] = _mm512_unpacklo_epi32(rows[group + i], rows[group + 4 + i]);
                next[group + 2 * i + 1] =
                    _mm512_unpackhi_epi32(rows[group + i], rows[group + 4 + i]);
            }
        }
        rows = next;
        for i in 0..8 {
            next[2 * i] = _mm512_unpacklo_epi64(rows[i], rows[8 + i]);
            next[2 * i + 1] = _mm512_unpackhi_epi64(rows[i], rows[8 + i]);
        }
        next
    }

    /// Exactly 64 consecutive u32 coefficients become 64 consecutive F128s.
    #[target_feature(enable = "gfni,avx512f,avx512bw")]
    unsafe fn embed64(bytes: *const u8, maps: &Matrices, output: *mut MaybeUninit<F128>) {
        // SAFETY: caller supplies 256 readable bytes and 1024 writable bytes.
        // All four byte planes are fully initialized before GFNI reads them.
        unsafe {
            let source: [__m512i; 4] =
                std::array::from_fn(|i| _mm512_loadu_si512(bytes.add(64 * i).cast()));
            let planes: [__m512i; 4] = std::array::from_fn(|byte| {
                let mask = _mm512_loadu_si512(SHUFFLES[byte].as_ptr().cast());
                // First dword in each 128-bit lane contains four selected
                // bytes. Compress those four dwords to a complete 16-row lane.
                let chunks: [__m128i; 4] = std::array::from_fn(|i| {
                    let selected = _mm512_shuffle_epi8(source[i], mask);
                    _mm512_castsi512_si128(_mm512_maskz_compress_epi32(0x1111, selected))
                });
                let v = _mm512_castsi128_si512(chunks[0]);
                let v = _mm512_inserti32x4::<1>(v, chunks[1]);
                let v = _mm512_inserti32x4::<2>(v, chunks[2]);
                _mm512_inserti32x4::<3>(v, chunks[3])
            });
            let converted: [__m512i; 16] = std::array::from_fn(|out| {
                let mut value = _mm512_setzero_si512();
                for input in 0..4 {
                    let mapped = _mm512_gf2p8affine_epi64_epi8::<0>(
                        planes[input],
                        _mm512_set1_epi64(maps[out][input] as i64),
                    );
                    value = _mm512_xor_si512(value, mapped);
                }
                value
            });
            let rows = transpose(converted);
            // Here the byte planes used natural row order, so each 128-bit
            // lane after the transpose belongs to a different 16-row group.
            for (i, &row) in rows.iter().enumerate() {
                let mut four = MaybeUninit::<[F128; 4]>::uninit();
                _mm512_storeu_si512(four.as_mut_ptr().cast(), row);
                for (lane, value) in four.assume_init().into_iter().enumerate() {
                    output.add(16 * lane + i).write(MaybeUninit::new(value));
                }
            }
        }
    }

    #[inline]
    fn scalar_embed(mut value: u32, basis: &[F128]) -> F128 {
        let mut result = F128::ZERO;
        while value != 0 {
            result += basis[value.trailing_zeros() as usize];
            value &= value - 1;
        }
        result
    }

    #[target_feature(enable = "gfni,avx512f,avx512bw,vpclmulqdq,sse4.1")]
    pub(super) unsafe fn block<const C: usize>(
        bytes: &[u8],
        basis: &[F128],
        maps: &Matrices,
        weights: &[F128],
    ) -> Vec<F128> {
        // SAFETY: caller validates C and exact source size. Each 64-row tile
        // writes all 64*C local values before any are read by the dot product.
        unsafe {
            let mut sums = [WideGhashX4::zero(); C];
            let mut tile = MaybeUninit::<[F128; 1024]>::uninit();
            let full = weights.len() / 64 * 64;
            for first in (0..full).step_by(64) {
                for group in 0..C {
                    embed64(
                        bytes.as_ptr().add((first * C + 64 * group) * 4),
                        maps,
                        tile.as_mut_ptr()
                            .cast::<MaybeUninit<F128>>()
                            .add(64 * group),
                    );
                }
                let values = std::slice::from_raw_parts(tile.as_ptr().cast::<F128>(), 64 * C);
                for row in (0..64).step_by(4) {
                    let w = f128x4_set(
                        weights[first + row],
                        weights[first + row + 1],
                        weights[first + row + 2],
                        weights[first + row + 3],
                    );
                    for coefficient in 0..C {
                        let v = f128x4_set(
                            values[row * C + coefficient],
                            values[(row + 1) * C + coefficient],
                            values[(row + 2) * C + coefficient],
                            values[(row + 3) * C + coefficient],
                        );
                        sums[coefficient].mul_acc(v, w);
                    }
                }
            }
            let mut sums = sums.map(|sum| sum.fold());
            for row in full..weights.len() {
                for coefficient in 0..C {
                    let start = (row * C + coefficient) * 4;
                    let value = u32::from_le_bytes(bytes[start..start + 4].try_into().unwrap());
                    if value != 0 {
                        sums[coefficient] ^= scalar_embed(value, basis).mul_unreduced(weights[row]);
                    }
                }
            }
            sums.into_iter().map(|sum| sum.reduce()).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_field::F256Unreduced;

    fn oracle(bytes: &[u8], count: usize, basis: &[F128], weights: &[F128]) -> Vec<F128> {
        let tables = crate::field::linear_tables(basis);
        let mut sums = vec![F256Unreduced::ZERO; count];
        for (row, &weight) in weights.iter().enumerate() {
            for column in 0..count {
                let input = &bytes[(row * count + column) * 4..(row * count + column + 1) * 4];
                let value = input
                    .iter()
                    .zip(&tables)
                    .fold(F128::ZERO, |s, (&x, t)| s + t[x as usize]);
                sums[column] ^= value.mul_unreduced(weight);
            }
        }
        sums.into_iter().map(F256Unreduced::reduce).collect()
    }

    #[test]
    fn packed_aggregate_gfni_public_matrices_match_every_basis_bit() {
        let basis = FastField::for_width(32).unwrap().base.basis();
        let maps = make_matrices(basis);
        for bit in 0..32 {
            let mut bytes = [0u8; 16];
            for out in 0..16 {
                for output_bit in 0..8 {
                    bytes[out] |= (((maps[out][bit / 8] >> (8 * (7 - output_bit) + bit % 8)) & 1)
                        as u8)
                        << output_bit;
                }
            }
            assert_eq!(
                F128::new(
                    u64::from_le_bytes(bytes[..8].try_into().unwrap()),
                    u64::from_le_bytes(bytes[8..].try_into().unwrap())
                ),
                basis[bit]
            );
        }
    }

    #[test]
    fn packed_aggregate_gfni_every_input_bit_and_row_lane_matches_original() {
        if !available() {
            return;
        }
        let basis = FastField::for_width(32).unwrap().base.basis();
        for count in [6, 8, 12, 16] {
            // Each one-hot row weight selects every position in both 64-row
            // tiles. Coefficient bits vary with row and column across all 32 bits.
            let mut bytes = vec![0u8; 128 * count * 4];
            for row in 0..128 {
                for column in 0..count {
                    bytes[(row * count + column) * 4..(row * count + column + 1) * 4]
                        .copy_from_slice(&(1u32 << ((row + 13 * column) % 32)).to_le_bytes());
                }
            }
            for selected in 0..128 {
                let mut weights = vec![F128::ZERO; 128];
                weights[selected] = F128::ONE;
                assert_eq!(
                    block(&bytes, count, basis, &weights).unwrap(),
                    oracle(&bytes, count, basis, &weights)
                );
            }
        }
    }

    #[test]
    fn packed_aggregate_gfni_random_tails_zero_weights_and_byte_offsets_match_original() {
        if !available() {
            return;
        }
        let basis = FastField::for_width(32).unwrap().base.basis();
        for count in [6, 8, 12, 16] {
            for rows in [0usize, 1, 3, 4, 7, 31, 63, 64, 65, 127, 128, 129, 257] {
                let weights: Vec<_> = (0..rows)
                    .map(|i| {
                        if i % 7 == 0 {
                            F128::ZERO
                        } else {
                            F128::new((i as u64 + 17) * 0x77153c1, (i as u64 + 79) * 0x431c752)
                        }
                    })
                    .collect();
                for offset in [0usize, 1, 3, 5, 15] {
                    let input: Vec<u8> = (0..rows * count * 4 + offset)
                        .map(|i| {
                            let v = (i as u64 + 73).wrapping_mul(0x9e3779b97f4a7c15);
                            (v ^ (v >> 39)) as u8
                        })
                        .collect();
                    let bytes = &input[offset..];
                    assert_eq!(
                        block(bytes, count, basis, &weights).unwrap(),
                        oracle(bytes, count, basis, &weights)
                    );
                    assert_eq!(
                        block(bytes, count, basis, &vec![F128::ZERO; rows]).unwrap(),
                        vec![F128::ZERO; count]
                    );
                }
            }
        }
    }

    #[test]
    fn packed_aggregate_gfni_rejects_incompatible_shapes() {
        let basis = FastField::for_width(32).unwrap().base.basis();
        assert!(block(&[0; 12], 3, basis, &[F128::ONE]).is_none());
        assert!(block(&[0; 24], 6, &basis[..31], &[F128::ONE]).is_none());
        assert!(block(&[0; 23], 6, basis, &[F128::ONE]).is_none());
    }
}
