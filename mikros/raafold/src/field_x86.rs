// SPDX-License-Identifier: Apache-2.0 OR MIT
// Scalar Karatsuba multiplication and POLYVAL reduction adapted from
// RustCrypto polyval 0.6.2, src/backend/clmul.rs. The four-lane kernels apply
// the same arithmetic independently in each 128-bit VPCLMUL lane.

//! Internal kernels; callers must check the declared CPU features.

use core::arch::x86_64::*;

use super::PolyvalWideProduct;

#[inline]
#[target_feature(enable = "pclmulqdq")]
unsafe fn product_parts(a: __m128i, b: __m128i) -> (__m128i, __m128i, __m128i) {
    let lo = _mm_clmulepi64_si128::<0x00>(a, b);
    let hi = _mm_clmulepi64_si128::<0x11>(a, b);
    let ax = _mm_xor_si128(a, _mm_srli_si128::<8>(a));
    let bx = _mm_xor_si128(b, _mm_srli_si128::<8>(b));
    let mid = _mm_xor_si128(_mm_clmulepi64_si128::<0x00>(ax, bx), _mm_xor_si128(lo, hi));
    (lo, hi, mid)
}

#[inline]
#[target_feature(enable = "sse2")]
unsafe fn reduce_parts(lo: __m128i, hi: __m128i, mid: __m128i) -> __m128i {
    let v0 = lo;
    let mut v1 = _mm_xor_si128(_mm_srli_si128::<8>(lo), mid);
    let mut v2 = _mm_xor_si128(hi, _mm_srli_si128::<8>(mid));
    let mut v3 = _mm_srli_si128::<8>(hi);
    v2 = _mm_xor_si128(v2, v0);
    v2 = _mm_xor_si128(v2, _mm_srli_epi64::<1>(v0));
    v2 = _mm_xor_si128(v2, _mm_srli_epi64::<2>(v0));
    v2 = _mm_xor_si128(v2, _mm_srli_epi64::<7>(v0));
    v1 = _mm_xor_si128(v1, _mm_slli_epi64::<63>(v0));
    v1 = _mm_xor_si128(v1, _mm_slli_epi64::<62>(v0));
    v1 = _mm_xor_si128(v1, _mm_slli_epi64::<57>(v0));
    v3 = _mm_xor_si128(v3, v1);
    v3 = _mm_xor_si128(v3, _mm_srli_epi64::<1>(v1));
    v3 = _mm_xor_si128(v3, _mm_srli_epi64::<2>(v1));
    v3 = _mm_xor_si128(v3, _mm_srli_epi64::<7>(v1));
    v2 = _mm_xor_si128(v2, _mm_slli_epi64::<63>(v1));
    v2 = _mm_xor_si128(v2, _mm_slli_epi64::<62>(v1));
    v2 = _mm_xor_si128(v2, _mm_slli_epi64::<57>(v1));
    _mm_unpacklo_epi64(v2, v3)
}

#[inline]
#[target_feature(enable = "pclmulqdq")]
pub(super) unsafe fn multiply(a: u128, b: u128) -> u128 {
    let (lo, hi, mid) = product_parts(core::mem::transmute(a), core::mem::transmute(b));
    core::mem::transmute(reduce_parts(lo, hi, mid))
}

#[inline]
#[target_feature(enable = "pclmulqdq")]
pub(super) unsafe fn wide_multiply(a: u128, b: u128) -> PolyvalWideProduct {
    let (lo, hi, mid) = product_parts(core::mem::transmute(a), core::mem::transmute(b));
    let lo: [u64; 2] = core::mem::transmute(lo);
    let hi: [u64; 2] = core::mem::transmute(hi);
    let mid: [u64; 2] = core::mem::transmute(mid);
    PolyvalWideProduct([lo[0], lo[1] ^ mid[0], hi[0] ^ mid[1], hi[1]])
}

#[inline]
#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq")]
unsafe fn product_parts_x4(a: __m512i, b: __m512i) -> (__m512i, __m512i, __m512i) {
    let lo = _mm512_clmulepi64_epi128::<0x00>(a, b);
    let hi = _mm512_clmulepi64_epi128::<0x11>(a, b);
    let ax = _mm512_xor_si512(a, _mm512_bsrli_epi128::<8>(a));
    let bx = _mm512_xor_si512(b, _mm512_bsrli_epi128::<8>(b));
    let mid = _mm512_xor_si512(
        _mm512_clmulepi64_epi128::<0x00>(ax, bx),
        _mm512_xor_si512(lo, hi),
    );
    (lo, hi, mid)
}

#[inline]
#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq")]
unsafe fn multiply_x4(a: __m512i, b: __m512i) -> __m512i {
    let (v0, hi, mid) = product_parts_x4(a, b);
    let mut v1 = _mm512_xor_si512(_mm512_bsrli_epi128::<8>(v0), mid);
    let mut v2 = _mm512_xor_si512(hi, _mm512_bsrli_epi128::<8>(mid));
    let mut v3 = _mm512_bsrli_epi128::<8>(hi);
    v2 = _mm512_xor_si512(v2, v0);
    v2 = _mm512_xor_si512(v2, _mm512_srli_epi64::<1>(v0));
    v2 = _mm512_xor_si512(v2, _mm512_srli_epi64::<2>(v0));
    v2 = _mm512_xor_si512(v2, _mm512_srli_epi64::<7>(v0));
    v1 = _mm512_xor_si512(v1, _mm512_slli_epi64::<63>(v0));
    v1 = _mm512_xor_si512(v1, _mm512_slli_epi64::<62>(v0));
    v1 = _mm512_xor_si512(v1, _mm512_slli_epi64::<57>(v0));
    v3 = _mm512_xor_si512(v3, v1);
    v3 = _mm512_xor_si512(v3, _mm512_srli_epi64::<1>(v1));
    v3 = _mm512_xor_si512(v3, _mm512_srli_epi64::<2>(v1));
    v3 = _mm512_xor_si512(v3, _mm512_srli_epi64::<7>(v1));
    v2 = _mm512_xor_si512(v2, _mm512_slli_epi64::<63>(v1));
    v2 = _mm512_xor_si512(v2, _mm512_slli_epi64::<62>(v1));
    v2 = _mm512_xor_si512(v2, _mm512_slli_epi64::<57>(v1));
    _mm512_unpacklo_epi64(v2, v3)
}

#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq")]
pub(super) unsafe fn dot_x4(left: &[u128], right: &[u128]) -> PolyvalWideProduct {
    debug_assert_eq!(left.len(), right.len());
    debug_assert_eq!(left.len() % 4, 0);
    let mut lo = _mm512_setzero_si512();
    let mut hi = _mm512_setzero_si512();
    let mut mid = _mm512_setzero_si512();
    for i in (0..left.len()).step_by(4) {
        let a = _mm512_loadu_si512(left.as_ptr().add(i).cast());
        let b = _mm512_loadu_si512(right.as_ptr().add(i).cast());
        let (l, h, m) = product_parts_x4(a, b);
        lo = _mm512_xor_si512(lo, l);
        hi = _mm512_xor_si512(hi, h);
        mid = _mm512_xor_si512(mid, m);
    }
    let lo: [u64; 8] = core::mem::transmute(lo);
    let hi: [u64; 8] = core::mem::transmute(hi);
    let mid: [u64; 8] = core::mem::transmute(mid);
    let mut out = PolyvalWideProduct::default();
    for lane in 0..4 {
        let j = 2 * lane;
        out += PolyvalWideProduct([lo[j], lo[j + 1] ^ mid[j], hi[j] ^ mid[j + 1], hi[j + 1]]);
    }
    out
}

#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq")]
pub(super) unsafe fn multiply_assign_x4(values: &mut [u128], factor: u128) {
    debug_assert_eq!(values.len() % 4, 0);
    let factor = _mm512_set_epi64(
        (factor >> 64) as i64,
        factor as i64,
        (factor >> 64) as i64,
        factor as i64,
        (factor >> 64) as i64,
        factor as i64,
        (factor >> 64) as i64,
        factor as i64,
    );
    for block in values.chunks_exact_mut(4) {
        let value = _mm512_loadu_si512(block.as_ptr().cast());
        _mm512_storeu_si512(block.as_mut_ptr().cast(), multiply_x4(value, factor));
    }
}

#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq")]
pub(super) unsafe fn add_scaled_x4(dst: &mut [u128], src: &[u128], factor: u128) {
    debug_assert_eq!(dst.len(), src.len());
    debug_assert_eq!(dst.len() % 4, 0);
    let factor = _mm512_set_epi64(
        (factor >> 64) as i64,
        factor as i64,
        (factor >> 64) as i64,
        factor as i64,
        (factor >> 64) as i64,
        factor as i64,
        (factor >> 64) as i64,
        factor as i64,
    );
    for (dst, src) in dst.chunks_exact_mut(4).zip(src.chunks_exact(4)) {
        let a = _mm512_loadu_si512(src.as_ptr().cast());
        let old = _mm512_loadu_si512(dst.as_ptr().cast());
        _mm512_storeu_si512(
            dst.as_mut_ptr().cast(),
            _mm512_xor_si512(old, multiply_x4(a, factor)),
        );
    }
}

#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq")]
pub(super) unsafe fn fold_equality_x4(low: &mut [u128], high: &[u128], r: u128) {
    debug_assert_eq!(low.len(), high.len());
    debug_assert_eq!(low.len() % 4, 0);
    let r = _mm512_set_epi64(
        (r >> 64) as i64,
        r as i64,
        (r >> 64) as i64,
        r as i64,
        (r >> 64) as i64,
        r as i64,
        (r >> 64) as i64,
        r as i64,
    );
    for (low, high) in low.chunks_exact_mut(4).zip(high.chunks_exact(4)) {
        let old = _mm512_loadu_si512(low.as_ptr().cast());
        let high = _mm512_loadu_si512(high.as_ptr().cast());
        let delta = _mm512_xor_si512(old, high);
        _mm512_storeu_si512(
            low.as_mut_ptr().cast(),
            _mm512_xor_si512(old, multiply_x4(delta, r)),
        );
    }
}
