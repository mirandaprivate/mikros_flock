//! Four independent high-half GF(256)[X] convolutions using VPCLMULQDQ.
//!
//! `rows[r][side][p]` is a bit plane: bit j is AES coefficient bit p of
//! X^j in that row's input polynomial. Each output plane's bit j instead
//! describes X^(128+j). Bit127 (degree255) is always zero, since both inputs
//! have degree at most127. These are ordinary polynomial bits, not GHASH's
//! extension-field coordinates or a reduced GF(2^128) product.
//!
//! Karatsuba acts on the eight coefficient-bit planes over the ring F2[X].
//! Its 27 base products each compute only the upper128 bits, in four lanes.
//! AES reduction is linear and commutes with this high-half projection.
//! There is no carry from the discarded low128 bits into the retained half.

use std::arch::x86_64::{
    __m512i, _mm512_bsrli_epi128, _mm512_clmulepi64_epi128, _mm512_setzero_si512, _mm512_xor_si512,
};

const PLANES: usize = 8;
const ROWS: usize = 4;

/// High halves of four independent GF(256)[X] products in AES bit-plane form.
/// Each side has128 coefficients; no byte transposition is performed here.
///
/// # Safety
/// Requires AVX-512F, AVX-512BW and VPCLMULQDQ. The caller must perform runtime
/// dispatch or otherwise establish those features. Full fixed-size references
/// make every input row/lane valid; callers may zero unused tail rows.
#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq")]
pub(super) unsafe fn high_products(rows: &[[[u128; PLANES]; 2]; ROWS]) -> [[u128; PLANES]; ROWS] {
    // SAFETY: all helpers have the same declared feature contract; pack/unpack
    // preserve four128-bit lanes and never read beyond these complete arrays.
    unsafe {
        let left =
            std::array::from_fn(|plane| pack(std::array::from_fn(|row| rows[row][0][plane])));
        let right =
            std::array::from_fn(|plane| pack(std::array::from_fn(|row| rows[row][1][plane])));
        let reduced = reduce_aes(mul8(left, right));
        let lanes: [[u128; ROWS]; PLANES] = reduced.map(|plane| unpack(plane));
        std::array::from_fn(|row| std::array::from_fn(|plane| lanes[plane][row]))
    }
}

#[inline]
fn pack(lanes: [u128; ROWS]) -> __m512i {
    // SAFETY: both types contain512 initialized bits with no invalid bit
    // patterns. On x86, little-endian u128 bits are the CLMUL polynomial bits;
    // the first array element occupies the low128-bit SIMD lane.
    unsafe { std::mem::transmute(lanes) }
}

#[inline]
fn unpack(lanes: __m512i) -> [u128; ROWS] {
    // SAFETY: same size/layout mapping as pack; every u128 bit pattern is valid.
    unsafe { std::mem::transmute(lanes) }
}

/// H(A*B) = A_hi*B_hi + high64(A_lo*B_hi + A_hi*B_lo).
/// The cross-term high64 values enter only the low64 of the returned high128.
/// A_lo*B_lo has degree <=126 and cannot contribute to H.
#[inline]
#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq")]
unsafe fn binary_high(left: __m512i, right: __m512i) -> __m512i {
    let diagonal = _mm512_clmulepi64_epi128::<0x11>(left, right);
    let cross = _mm512_xor_si512(
        _mm512_clmulepi64_epi128::<0x01>(left, right),
        _mm512_clmulepi64_epi128::<0x10>(left, right),
    );
    _mm512_xor_si512(diagonal, _mm512_bsrli_epi128::<8>(cross))
}

#[inline]
#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq")]
unsafe fn mul2(left: [__m512i; 2], right: [__m512i; 2]) -> [__m512i; 3] {
    // SAFETY: this function declares binary_high's required features.
    unsafe {
        let low = binary_high(left[0], right[0]);
        let high = binary_high(left[1], right[1]);
        let sum = binary_high(
            _mm512_xor_si512(left[0], left[1]),
            _mm512_xor_si512(right[0], right[1]),
        );
        [
            low,
            _mm512_xor_si512(sum, _mm512_xor_si512(low, high)),
            high,
        ]
    }
}

#[inline]
#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq")]
unsafe fn mul4(left: [__m512i; 4], right: [__m512i; 4]) -> [__m512i; 7] {
    // SAFETY: each recursive subproduct has the same feature contract.
    unsafe {
        let low = mul2([left[0], left[1]], [right[0], right[1]]);
        let high = mul2([left[2], left[3]], [right[2], right[3]]);
        let sum = mul2(
            std::array::from_fn(|i| _mm512_xor_si512(left[i], left[2 + i])),
            std::array::from_fn(|i| _mm512_xor_si512(right[i], right[2 + i])),
        );
        let cross: [__m512i; 3] =
            std::array::from_fn(|i| _mm512_xor_si512(sum[i], _mm512_xor_si512(low[i], high[i])));
        [
            low[0],
            low[1],
            _mm512_xor_si512(low[2], cross[0]),
            cross[1],
            _mm512_xor_si512(high[0], cross[2]),
            high[1],
            high[2],
        ]
    }
}

#[inline]
#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq")]
unsafe fn mul8(left: [__m512i; PLANES], right: [__m512i; PLANES]) -> [__m512i; 15] {
    // SAFETY: recursive calls have the same feature contract. Karatsuba's
    // additions commute with binary_high, which is an F2-bilinear projection;
    // no multiplication of previously truncated intermediate values occurs.
    unsafe {
        let low = mul4(
            std::array::from_fn(|i| left[i]),
            std::array::from_fn(|i| right[i]),
        );
        let high = mul4(
            std::array::from_fn(|i| left[4 + i]),
            std::array::from_fn(|i| right[4 + i]),
        );
        let sum = mul4(
            std::array::from_fn(|i| _mm512_xor_si512(left[i], left[4 + i])),
            std::array::from_fn(|i| _mm512_xor_si512(right[i], right[4 + i])),
        );
        let mut output = [_mm512_setzero_si512(); 15];
        for i in 0..7 {
            let cross = _mm512_xor_si512(sum[i], _mm512_xor_si512(low[i], high[i]));
            output[i] = _mm512_xor_si512(output[i], low[i]);
            output[4 + i] = _mm512_xor_si512(output[4 + i], cross);
            output[8 + i] = _mm512_xor_si512(output[8 + i], high[i]);
        }
        output
    }
}

/// Reduce coefficient-bit powers z^8 = z^4+z^3+z+1 in the AES representation.
/// Descending order matters: reducing z^14 initially produces z^9 and z^10.
#[inline]
#[target_feature(enable = "avx512f")]
unsafe fn reduce_aes(mut product: [__m512i; 15]) -> [__m512i; PLANES] {
    for degree in (PLANES..15).rev() {
        let high = product[degree];
        for term in [0, 1, 3, 4] {
            let target = degree - PLANES + term;
            product[target] = _mm512_xor_si512(product[target], high);
        }
    }
    std::array::from_fn(|plane| product[plane])
}

/// A64-plane-product oracle for the same hardware high-half primitive. Kept
/// independent of the Karatsuba assembly; not selected by production dispatch.
#[cfg(test)]
#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq")]
unsafe fn schoolbook_products(rows: &[[[u128; PLANES]; 2]; ROWS]) -> [[u128; PLANES]; ROWS] {
    // SAFETY: every called helper's feature requirements are declared here.
    unsafe {
        let mut product = [_mm512_setzero_si512(); 15];
        for i in 0..PLANES {
            let left = pack(std::array::from_fn(|row| rows[row][0][i]));
            for j in 0..PLANES {
                let right = pack(std::array::from_fn(|row| rows[row][1][j]));
                product[i + j] = _mm512_xor_si512(product[i + j], binary_high(left, right));
            }
        }
        let lanes: [[u128; ROWS]; PLANES] = reduce_aes(product).map(|plane| unpack(plane));
        std::array::from_fn(|row| std::array::from_fn(|plane| lanes[plane][row]))
    }
}

#[cfg(test)]
mod tests {
    use flock_field::{F8, test_rng::Rng};

    use super::{PLANES, ROWS, binary_high, high_products, pack, schoolbook_products, unpack};

    fn available() -> bool {
        std::is_x86_feature_detected!("avx512f")
            && std::is_x86_feature_detected!("avx512bw")
            && std::is_x86_feature_detected!("vpclmulqdq")
    }

    // Independent bit-by-bit polynomial multiplication. The two u128 words
    // are the unreduced degree0..127 and degree128..255 parts respectively.
    fn binary_product_oracle(left: u128, right: u128) -> [u128; 2] {
        let mut full = [0; 2];
        for bit in 0..128 {
            if (right >> bit) & 1 != 0 {
                full[0] ^= left << bit;
                if bit != 0 {
                    full[1] ^= left >> (128 - bit);
                }
            }
        }
        full
    }

    fn planes(bytes: &[u8; 128]) -> [u128; PLANES] {
        std::array::from_fn(|plane| {
            bytes.iter().enumerate().fold(0, |bits, (degree, &byte)| {
                bits | (u128::from((byte >> plane) & 1) << degree)
            })
        })
    }

    fn bytes(planes: &[u128; PLANES]) -> [u8; 128] {
        std::array::from_fn(|degree| {
            planes.iter().enumerate().fold(0, |byte, (plane, bits)| {
                byte | (((bits >> degree) & 1) as u8) << plane
            })
        })
    }

    fn high_f8_oracle(left: &[u8; 128], right: &[u8; 128]) -> [u8; 128] {
        let mut full = [F8::ZERO; 256];
        for (i, &a) in left.iter().enumerate() {
            for (j, &b) in right.iter().enumerate() {
                full[i + j] += F8(a) * F8(b);
            }
        }
        std::array::from_fn(|i| full[128 + i].0)
    }

    #[test]
    fn independent_binary_oracle_has_correct_high_half_boundaries() {
        for (i, j) in [
            (0, 127),
            (1, 127),
            (63, 63),
            (63, 65),
            (64, 64),
            (65, 63),
            (127, 127),
        ] {
            let mut expected = [0; 2];
            expected[(i + j) / 128] = 1u128 << ((i + j) % 128);
            assert_eq!(binary_product_oracle(1u128 << i, 1u128 << j), expected);
        }
    }

    #[test]
    fn vpclmul_cross_high64_terms_match_independent_full_binary_products() {
        if !available() {
            return;
        }
        let mut cases = vec![
            (0, 0),
            (u128::MAX, u128::MAX),
            (u128::MAX, 1),
            (1, u128::MAX),
        ];
        for i in [0, 1, 62, 63, 64, 65, 126, 127] {
            for j in [0, 1, 62, 63, 64, 65, 126, 127] {
                cases.push((1u128 << i, 1u128 << j));
            }
        }
        let mut rng = Rng::new(0x434C_4D55_4C31);
        for _ in 0..128 {
            let a = u128::from(rng.next_u64()) | (u128::from(rng.next_u64()) << 64);
            let b = u128::from(rng.next_u64()) | (u128::from(rng.next_u64()) << 64);
            cases.push((a, b));
        }
        for group in cases.chunks(ROWS) {
            let left = std::array::from_fn(|r| group.get(r).map_or(0, |p| p.0));
            let right = std::array::from_fn(|r| group.get(r).map_or(0, |p| p.1));
            // SAFETY: the runtime check establishes every required feature.
            let actual = unsafe { unpack(binary_high(pack(left), pack(right))) };
            for row in 0..ROWS {
                assert_eq!(actual[row], binary_product_oracle(left[row], right[row])[1]);
                assert_eq!(actual[row] >> 127, 0);
            }
        }
    }

    #[test]
    fn four_lane_karatsuba_matches_unrestricted_f8_schoolbook() {
        if !available() {
            return;
        }
        let mut rng = Rng::new(0x434C_4D55_4C32);
        for _ in 0..16 {
            let mut inputs = [[[0; 128]; 2]; ROWS];
            for row in &mut inputs {
                for side in row {
                    rng.fill_bytes(side);
                }
            }
            let input_planes = inputs.map(|row| row.map(|side| planes(&side)));
            // SAFETY: runtime dispatch was checked above; all four lanes are complete.
            let (actual, schoolbook) = unsafe {
                (
                    high_products(&input_planes),
                    schoolbook_products(&input_planes),
                )
            };
            assert_eq!(actual, schoolbook);
            for row in 0..ROWS {
                assert_eq!(
                    bytes(&actual[row]),
                    high_f8_oracle(&inputs[row][0], &inputs[row][1])
                );
                assert!(actual[row].iter().all(|plane| plane >> 127 == 0));
            }
        }
    }

    #[test]
    fn every_aes_basis_product_and_monomial_boundary_is_correct() {
        if !available() {
            return;
        }
        for i in [0, 1, 62, 63, 64, 65, 126, 127] {
            for j in [0, 1, 62, 63, 64, 65, 126, 127] {
                for group in (0..PLANES * PLANES).step_by(ROWS) {
                    let mut inputs = [[[0u128; PLANES]; 2]; ROWS];
                    let mut expected = [[0u128; PLANES]; ROWS];
                    for row in 0..ROWS {
                        let a_bit = (group + row) / PLANES;
                        let b_bit = (group + row) % PLANES;
                        inputs[row][0][a_bit] = 1u128 << i;
                        inputs[row][1][b_bit] = 1u128 << j;
                        if i + j >= 128 {
                            let value = (F8(1 << a_bit) * F8(1 << b_bit)).0;
                            for bit in 0..PLANES {
                                expected[row][bit] =
                                    u128::from((value >> bit) & 1) << (i + j - 128);
                            }
                        }
                    }
                    // SAFETY: every feature is checked, and all inputs are initialized.
                    assert_eq!(unsafe { high_products(&inputs) }, expected);
                }
            }
        }
    }

    #[test]
    fn padded_tail_rows_remain_independent_and_zero() {
        if !available() {
            return;
        }
        let mut rng = Rng::new(0x434C_4D55_4C33);
        for count in 0..=ROWS {
            let mut inputs = [[[0u8; 128]; 2]; ROWS];
            for row in &mut inputs[..count] {
                for side in row {
                    rng.fill_bytes(side);
                }
            }
            let mut input_planes = inputs.map(|row| row.map(|side| planes(&side)));
            // SAFETY: all ISA features were checked, including the partial-tail case.
            let expected = unsafe { high_products(&input_planes) };
            for row in 0..ROWS {
                assert_eq!(
                    bytes(&expected[row]),
                    high_f8_oracle(&inputs[row][0], &inputs[row][1])
                );
            }
            assert!(expected[count..].iter().all(|row| *row == [0; PLANES]));
            if count != 0 {
                input_planes[0][0] = [0; PLANES];
                // SAFETY: the same checked features and complete four-lane arrays.
                let changed = unsafe { high_products(&input_planes) };
                assert_eq!(changed[0], [0; PLANES]);
                assert_eq!(&changed[1..], &expected[1..]);
            }
        }
    }
}
