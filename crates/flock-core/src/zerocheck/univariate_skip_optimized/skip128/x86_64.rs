//! Two-register GFNI / AVX-512 kernels for the dense skip7 round one.

use core::arch::x86_64::{
    _mm512_gf2p8affine_epi64_epi8, _mm512_gf2p8mul_epi8, _mm512_loadu_si512,
    _mm512_permutex2var_epi8, _mm512_set1_epi8, _mm512_set1_epi64, _mm512_setzero_si512,
    _mm512_storeu_si512, _mm512_xor_si512,
};

use crate::ntt::InvNttTableByteSingleGf8;

/// # Safety
/// Requires GFNI and AVX-512F/BW. The table must have 128 lanes and sixteen
/// packed input bytes, and both witnesses must contain `base..base+128`.
#[allow(dead_code)] // exercised by runtime-gated tests on non-native builds
#[inline]
#[target_feature(enable = "gfni,avx512f,avx512bw")]
pub(super) unsafe fn shift_reduce(
    a: &[u8],
    b: &[u8],
    table: &InvNttTableByteSingleGf8,
    base: usize,
    low: &mut [u8; 64],
    high: &mut [u8; 64],
) {
    debug_assert!(base + 128 <= a.len() && base + 128 <= b.len());
    debug_assert_eq!((table.ell, table.n_chunks), (128, 16));
    // SAFETY: every row access is sixteen readable bytes in the validated
    // 128-byte medium block. The LDE returns exactly two register halves.
    unsafe {
        let mut acc = [_mm512_setzero_si512(); 2];
        for small in 0..8 {
            let offset = base + small * 16;
            let a = table.apply_x86_avx512_128_register_unchecked(a.as_ptr().add(offset));
            let b = table.apply_x86_avx512_128_register_unchecked(b.as_ptr().add(offset));
            let weight = _mm512_set1_epi8((1u8 << small) as i8);
            for half in 0..2 {
                let product = _mm512_gf2p8mul_epi8(a[half], b[half]);
                let product = if small == 0 {
                    product
                } else {
                    _mm512_gf2p8mul_epi8(product, weight)
                };
                acc[half] = _mm512_xor_si512(acc[half], product);
            }
        }
        _mm512_storeu_si512(low.as_mut_ptr().cast(), acc[0]);
        _mm512_storeu_si512(high.as_mut_ptr().cast(), acc[1]);
    }
}

const fn gather_indices(high: bool) -> [u8; 64] {
    let mut indices = [0u8; 64];
    let mut index = 0;
    while index < 64 {
        // GFNI's affine matrix rows run from the high byte to the low byte.
        indices[index] = ((7 - index % 8) * 16 + index / 8 + if high { 8 } else { 0 }) as u8;
        index += 1;
    }
    indices
}
const LOW: [u8; 64] = gather_indices(false);
const HIGH: [u8; 64] = gather_indices(true);

/// Transpose eight 128-bit C rows to 128 bytes of small-coordinate masks.
///
/// # Safety
/// Requires GFNI and AVX-512F/BW/VBMI. Fixed-size arrays bound both input loads
/// and output stores. No field multiplication or LDE is required for C here.
#[allow(dead_code)] // independent hardware test also covers portable builds
#[inline]
#[target_feature(enable = "gfni,avx512f,avx512bw,avx512vbmi")]
pub(super) unsafe fn transpose_c(input: &[u8; 128], low: &mut [u8; 64], high: &mut [u8; 64]) {
    // SAFETY: the two loads/stores exactly cover their fixed-size buffers.
    unsafe {
        let a = _mm512_loadu_si512(input.as_ptr().cast());
        let b = _mm512_loadu_si512(input.as_ptr().add(64).cast());
        let low_index = _mm512_loadu_si512(LOW.as_ptr().cast());
        let high_index = _mm512_loadu_si512(HIGH.as_ptr().cast());
        let basis = _mm512_set1_epi64(0x8040_2010_0804_0201u64 as i64);
        let first =
            _mm512_gf2p8affine_epi64_epi8::<0>(basis, _mm512_permutex2var_epi8(a, low_index, b));
        let second =
            _mm512_gf2p8affine_epi64_epi8::<0>(basis, _mm512_permutex2var_epi8(a, high_index, b));
        _mm512_storeu_si512(low.as_mut_ptr().cast(), first);
        _mm512_storeu_si512(high.as_mut_ptr().cast(), second);
    }
}
