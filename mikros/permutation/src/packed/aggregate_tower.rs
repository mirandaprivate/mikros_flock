//! Mixed K32/E aggregation in actual K32 coordinates of each shared weight.
//!
//! The coefficient stream remains in its existing SmallField32 polynomial
//! basis. Only the challenge-dependent low equality weights are converted.
//! Four unreduced 32x32 products per row are accumulated independently, then
//! reduced and composed back into the unchanged GHASH field at block end.

use crate::field::{FastField, from_raw, raw};
use flock_field::F128;
use std::sync::OnceLock;

struct Tower {
    field: &'static FastField,
    powers: [F128; 4],
    decode: Box<[[u128; 256]]>,
}

impl Tower {
    fn new() -> Option<Self> {
        let field = FastField::for_width(32).ok()?;
        // x has degree 128 in the GHASH representation, hence degree four
        // over K32. Check the full binary rank, with a bounded fallback search
        // so a representation change cannot silently produce wrong coordinates.
        for candidate in 2u128..258 {
            let alpha = from_raw(candidate);
            let powers = [F128::ONE, alpha, alpha.square(), alpha.square() * alpha];
            let columns = std::array::from_fn::<_, 128, _>(|bit| {
                raw(powers[bit / 32] * field.base.basis()[bit % 32])
            });
            if let Some(inverse) = invert_binary_basis(&columns) {
                let mut decode = vec![[0u128; 256]; 16].into_boxed_slice();
                for byte in 0..16 {
                    for value in 1usize..256 {
                        let bit = value.trailing_zeros() as usize;
                        decode[byte][value] =
                            decode[byte][value & (value - 1)] ^ inverse[8 * byte + bit];
                    }
                }
                return Some(Self {
                    field,
                    powers,
                    decode,
                });
            }
        }
        None
    }

    fn decompose(&self, weight: F128) -> [u32; 4] {
        let bytes = raw(weight).to_le_bytes();
        let mut coordinates = 0u128;
        for (byte, &value) in bytes.iter().enumerate() {
            coordinates ^= self.decode[byte][value as usize];
        }
        std::array::from_fn(|j| (coordinates >> (32 * j)) as u32)
    }

    fn compose(&self, coordinates: [u32; 4]) -> F128 {
        coordinates
            .into_iter()
            .enumerate()
            .fold(F128::ZERO, |value, (j, coordinate)| {
                value + self.field.embed(coordinate as u128) * self.powers[j]
            })
    }

    fn reduce(&self, products: [u64; 4]) -> F128 {
        self.compose(products.map(|p| self.field.reduce_product32(p)))
    }
}

fn invert_binary_basis(columns: &[u128; 128]) -> Option<[u128; 128]> {
    let mut rows = [0u128; 128];
    let mut inverse_rows = [0u128; 128];
    for row in 0..128 {
        inverse_rows[row] = 1u128 << row;
        for (column, &value) in columns.iter().enumerate() {
            rows[row] |= ((value >> row) & 1) << column;
        }
    }
    for column in 0..128 {
        let pivot = (column..128).find(|&row| rows[row] & (1u128 << column) != 0)?;
        rows.swap(column, pivot);
        inverse_rows.swap(column, pivot);
        let pivot_row = rows[column];
        let pivot_inverse = inverse_rows[column];
        for row in 0..128 {
            if row != column && rows[row] & (1u128 << column) != 0 {
                rows[row] ^= pivot_row;
                inverse_rows[row] ^= pivot_inverse;
            }
        }
    }
    Some(std::array::from_fn(|column| {
        inverse_rows
            .iter()
            .enumerate()
            .fold(0, |value, (row, &bits)| {
                value | (((bits >> column) & 1) << row)
            })
    }))
}

fn tower() -> Option<&'static Tower> {
    static TOWER: OnceLock<Option<Tower>> = OnceLock::new();
    TOWER.get_or_init(Tower::new).as_ref()
}

/// Each 128-bit lane contains one row's coordinate pair, zero-extended to
/// two qwords. Pair zero holds K-coordinates 0/1; pair one holds 2/3.
#[repr(C, align(64))]
struct FourWeights {
    pairs: [[u64; 8]; 2],
}

pub(super) struct Weights {
    tower: &'static Tower,
    full: Vec<FourWeights>,
    tail: Vec<[u32; 4]>,
    len: usize,
}

/// Online conversion. Call once per equality low table, then share immutably
/// across all high blocks using that table. No challenge-dependent global cache.
pub(super) fn prepare(weights: &[F128]) -> Option<Weights> {
    let tower = tower()?;
    let mut full = Vec::with_capacity(weights.len() / 4);
    let mut chunks = weights.chunks_exact(4);
    for chunk in &mut chunks {
        let coordinates: [[u32; 4]; 4] = std::array::from_fn(|row| tower.decompose(chunk[row]));
        full.push(FourWeights {
            pairs: std::array::from_fn(|pair| {
                std::array::from_fn(|slot| coordinates[slot / 2][2 * pair + slot % 2] as u64)
            }),
        });
    }
    let tail = chunks
        .remainder()
        .iter()
        .map(|&w| tower.decompose(w))
        .collect();
    Some(Weights {
        tower,
        full,
        tail,
        len: weights.len(),
    })
}

#[inline]
fn coefficient(bytes: &[u8], coefficients_per_row: usize, row: usize, column: usize) -> u32 {
    let offset = (row * coefficients_per_row + column) * 4;
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}

#[inline]
fn polynomial_product(a: u32, mut b: u32) -> u64 {
    let mut product = 0;
    while b != 0 {
        product ^= (a as u64) << b.trailing_zeros();
        b &= b - 1;
    }
    product
}

/// Aggregate one exact high block of row-major, little-endian K32 coefficients.
/// Returns GHASH values, matching the original embedding-and-multiply kernel.
pub(super) fn block(bytes: &[u8], coefficients_per_row: usize, weights: &Weights) -> Vec<F128> {
    assert_eq!(bytes.len(), weights.len * coefficients_per_row * 4);
    let mut products = vec![[0u64; 4]; coefficients_per_row];
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "sse4.1"
    ))]
    {
        // SAFETY: native code is compile-time feature gated; exact byte length
        // was checked above, and only complete four-row groups enter the kernel.
        unsafe { native::aggregate(bytes, coefficients_per_row, &weights.full, &mut products) };
    }
    #[cfg(not(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "sse4.1"
    )))]
    for (group, weight) in weights.full.iter().enumerate() {
        for row in 0..4 {
            for (column, sum) in products.iter_mut().enumerate() {
                let a = coefficient(bytes, coefficients_per_row, group * 4 + row, column);
                for coordinate in 0..4 {
                    *sum.get_mut(coordinate).unwrap() ^= polynomial_product(
                        a,
                        weight.pairs[coordinate / 2][2 * row + coordinate % 2] as u32,
                    );
                }
            }
        }
    }
    for (row, coordinates) in weights.tail.iter().enumerate() {
        for (column, sum) in products.iter_mut().enumerate() {
            let a = coefficient(
                bytes,
                coefficients_per_row,
                weights.full.len() * 4 + row,
                column,
            );
            for j in 0..4 {
                sum[j] ^= polynomial_product(a, coordinates[j]);
            }
        }
    }
    products
        .into_iter()
        .map(|p| weights.tower.reduce(p))
        .collect()
}

#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512f",
    target_feature = "avx512bw",
    target_feature = "vpclmulqdq",
    target_feature = "sse4.1"
))]
mod native {
    use super::{FourWeights, coefficient};
    use std::arch::x86_64::*;

    #[inline]
    #[target_feature(enable = "avx512f,sse4.1")]
    unsafe fn fold(v: __m512i) -> [u64; 2] {
        let a = _mm_xor_si128(
            _mm512_extracti32x4_epi32::<0>(v),
            _mm512_extracti32x4_epi32::<1>(v),
        );
        let b = _mm_xor_si128(
            _mm512_extracti32x4_epi32::<2>(v),
            _mm512_extracti32x4_epi32::<3>(v),
        );
        let sum = _mm_xor_si128(a, b);
        [
            _mm_extract_epi64::<0>(sum) as u64,
            _mm_extract_epi64::<1>(sum) as u64,
        ]
    }

    /// Four rows times four coordinates require sixteen 32x32 products:
    /// exactly four VPCLMUL instructions, not two. Two independent results in
    /// each lane occupy disjoint qwords only AFTER multiplying and shifting.
    #[target_feature(enable = "avx512f,avx512bw,vpclmulqdq,sse4.1")]
    unsafe fn tile<const C: usize>(
        bytes: &[u8],
        width: usize,
        first: usize,
        weights: &[FourWeights],
    ) -> [[u64; 4]; C] {
        let zero = _mm512_setzero_si512();
        let mut sums = [[zero; 2]; C];
        for (group, weight) in weights.iter().enumerate() {
            // SAFETY: each pair contains eight readable u64s; loadu makes no
            // stronger alignment assumption about the caller's allocation.
            let w01 = unsafe { _mm512_loadu_si512(weight.pairs[0].as_ptr().cast()) };
            let w23 = unsafe { _mm512_loadu_si512(weight.pairs[1].as_ptr().cast()) };
            for column in 0..C {
                let values: [u64; 4] = std::array::from_fn(|row| {
                    coefficient(bytes, width, group * 4 + row, first + column) as u64
                });
                let a = _mm512_set_epi64(
                    values[3] as i64,
                    values[3] as i64,
                    values[2] as i64,
                    values[2] as i64,
                    values[1] as i64,
                    values[1] as i64,
                    values[0] as i64,
                    values[0] as i64,
                );
                let lo01 = _mm512_clmulepi64_epi128::<0x00>(a, w01);
                let hi01 = _mm512_bslli_epi128::<8>(_mm512_clmulepi64_epi128::<0x11>(a, w01));
                let lo23 = _mm512_clmulepi64_epi128::<0x00>(a, w23);
                let hi23 = _mm512_bslli_epi128::<8>(_mm512_clmulepi64_epi128::<0x11>(a, w23));
                sums[column][0] = _mm512_xor_si512(sums[column][0], _mm512_xor_si512(lo01, hi01));
                sums[column][1] = _mm512_xor_si512(sums[column][1], _mm512_xor_si512(lo23, hi23));
            }
        }
        std::array::from_fn(|column| {
            // SAFETY: tile declares every feature required by fold.
            let lo = unsafe { fold(sums[column][0]) };
            let hi = unsafe { fold(sums[column][1]) };
            [lo[0], lo[1], hi[0], hi[1]]
        })
    }

    #[target_feature(enable = "avx512f,avx512bw,vpclmulqdq,sse4.1")]
    pub(super) unsafe fn aggregate(
        bytes: &[u8],
        width: usize,
        weights: &[FourWeights],
        out: &mut [[u64; 4]],
    ) {
        let mut first = 0;
        while first + 4 <= width {
            // SAFETY: exact whole-block dimensions were checked by block();
            // this tile accesses four in-range coefficient positions per row.
            out[first..first + 4]
                .copy_from_slice(&unsafe { tile::<4>(bytes, width, first, weights) });
            first += 4;
        }
        match width - first {
            0 => {}
            1 => out[first..].copy_from_slice(&unsafe { tile::<1>(bytes, width, first, weights) }),
            2 => out[first..].copy_from_slice(&unsafe { tile::<2>(bytes, width, first, weights) }),
            3 => out[first..].copy_from_slice(&unsafe { tile::<3>(bytes, width, first, weights) }),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn next(state: &mut u128) -> u128 {
        *state ^= *state << 13;
        *state ^= *state >> 17;
        *state ^= *state << 43;
        *state
    }

    #[test]
    fn tower32_weight_decomposition_preserves_every_binary_and_tower_basis_bit() {
        let t = tower().expect("actual degree-four K32 tower");
        for bit in 0..128 {
            let value = from_raw(1u128 << bit);
            assert_eq!(t.compose(t.decompose(value)), value);
            let encoded = t.powers[bit / 32] * t.field.base.basis()[bit % 32];
            let mut expected = [0u32; 4];
            expected[bit / 32] = 1 << (bit % 32);
            assert_eq!(t.decompose(encoded), expected);
        }
        assert!(invert_binary_basis(&[0; 128]).is_none());
    }

    #[test]
    fn tower32_mixed_products_equal_the_original_ghash_field() {
        let t = tower().unwrap();
        let mut state = 0xaf31_5728_e802_aaa5_8721_c031_0762_8437u128;
        for i in 0..384 {
            let a = match i {
                0 => 0,
                1 => 1,
                2 => u32::MAX,
                _ => next(&mut state) as u32,
            };
            let weight = from_raw(next(&mut state));
            let actual = t.reduce(t.decompose(weight).map(|b| polynomial_product(a, b)));
            assert_eq!(actual, t.field.embed(a as u128) * weight);
        }
    }

    #[test]
    fn tower32_blocks_preserve_distinct_rows_coefficients_and_all_tail_lengths() {
        let field = FastField::for_width(32).unwrap();
        let mut state = 0xaba7_cd33_281e_990c_1453_2945_000a_770du128;
        for rows in [0usize, 1, 2, 3, 4, 5, 7, 8, 17, 33] {
            let original: Vec<F128> = (0..rows)
                .map(|i| match i % 9 {
                    0 => F128::ZERO,
                    1 => F128::ONE,
                    _ => from_raw(next(&mut state)),
                })
                .collect();
            let weights = prepare(&original).unwrap();
            for width in [1usize, 3, 4, 6, 8, 12, 16] {
                // Offset by one byte to exercise input reads without alignment.
                let mut bytes = vec![0xa5u8; 1 + rows * width * 4];
                for chunk in bytes[1..].chunks_mut(4) {
                    chunk.copy_from_slice(&(next(&mut state) as u32).to_le_bytes());
                }
                let input = &bytes[1..];
                let expected: Vec<F128> = (0..width)
                    .map(|column| {
                        original
                            .iter()
                            .enumerate()
                            .fold(F128::ZERO, |sum, (row, &weight)| {
                                sum + field.embed(coefficient(input, width, row, column) as u128)
                                    * weight
                            })
                    })
                    .collect();
                assert_eq!(
                    block(input, width, &weights),
                    expected,
                    "rows={rows},width={width}"
                );
                assert_eq!(bytes[0], 0xa5);
            }
        }
    }
}
