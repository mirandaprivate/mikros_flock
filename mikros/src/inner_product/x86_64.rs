//! Four-lane field kernels using Flock's unchanged GHASH implementation.

use core::arch::x86_64::{
    __m512i, _mm512_permutex2var_epi64, _mm512_set_epi64, _mm512_storeu_si512, _mm512_xor_si512,
};
use core::mem::MaybeUninit;
use flock_field::{
    F128, F256Unreduced,
    gf2_128::x86_64::{WideGhashX4, f128x4_loadu, f128x4_set, ghash_mul_x4},
};

/// Input registers contain eight contiguous F128 values. Return the four
/// even/odd values respectively, preserving each F128's pair of u64 limbs.
#[inline]
#[target_feature(enable = "avx512f")]
unsafe fn separate_pairs(lo: __m512i, hi: __m512i) -> (__m512i, __m512i) {
    let even = _mm512_set_epi64(13, 12, 9, 8, 5, 4, 1, 0);
    let odd = _mm512_set_epi64(15, 14, 11, 10, 7, 6, 3, 2);
    (
        _mm512_permutex2var_epi64(lo, even, hi),
        _mm512_permutex2var_epi64(lo, odd, hi),
    )
}

/// # Safety
/// Requires AVX-512F/BW, VPCLMULQDQ, PCLMULQDQ, and SSE4.1.
#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq,pclmulqdq,sse4.1")]
pub(super) unsafe fn message_chunk(a: &[F128], b: &[F128]) -> [F256Unreduced; 2] {
    assert_eq!(a.len(), b.len());
    // SAFETY: vector loads below cover eight elements of equal-length slices;
    // scalar chunks handle all remaining adjacent pairs.
    unsafe {
        let mut zero = WideGhashX4::zero();
        let mut infinity = WideGhashX4::zero();
        let bulk = a.len() / 8 * 8;
        for offset in (0..bulk).step_by(8) {
            let (a0, a1) = separate_pairs(
                f128x4_loadu(a.as_ptr().add(offset)),
                f128x4_loadu(a.as_ptr().add(offset + 4)),
            );
            let (b0, b1) = separate_pairs(
                f128x4_loadu(b.as_ptr().add(offset)),
                f128x4_loadu(b.as_ptr().add(offset + 4)),
            );
            zero.mul_acc(a0, b0);
            infinity.mul_acc(_mm512_xor_si512(a0, a1), _mm512_xor_si512(b0, b1));
        }
        let mut sums = [zero.fold(), infinity.fold()];
        for (a, b) in a[bulk..].chunks_exact(2).zip(b[bulk..].chunks_exact(2)) {
            sums[0] ^= a[0].mul_unreduced(b[0]);
            sums[1] ^= (a[0] + a[1]).mul_unreduced(b[0] + b[1]);
        }
        sums
    }
}

/// First message plus g(1), allowing the caller to check the full inner
/// product g(0)+g(1) without a separate scalar dot-product pass.
///
/// # Safety
/// Requires AVX-512F/BW, VPCLMULQDQ, PCLMULQDQ, and SSE4.1.
#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq,pclmulqdq,sse4.1")]
pub(super) unsafe fn checked_message_chunk(a: &[F128], b: &[F128]) -> [F256Unreduced; 3] {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len() % 2, 0);
    // SAFETY: each vector step reads eight elements within both input slices;
    // the even-sized tail is handled by bounded scalar adjacent pairs.
    unsafe {
        let mut zero = WideGhashX4::zero();
        let mut one = WideGhashX4::zero();
        let mut infinity = WideGhashX4::zero();
        let bulk = a.len() / 8 * 8;
        for offset in (0..bulk).step_by(8) {
            let (a0, a1) = separate_pairs(
                f128x4_loadu(a.as_ptr().add(offset)),
                f128x4_loadu(a.as_ptr().add(offset + 4)),
            );
            let (b0, b1) = separate_pairs(
                f128x4_loadu(b.as_ptr().add(offset)),
                f128x4_loadu(b.as_ptr().add(offset + 4)),
            );
            zero.mul_acc(a0, b0);
            one.mul_acc(a1, b1);
            infinity.mul_acc(_mm512_xor_si512(a0, a1), _mm512_xor_si512(b0, b1));
        }
        let mut sums = [zero.fold(), one.fold(), infinity.fold()];
        for (a, b) in a[bulk..].chunks_exact(2).zip(b[bulk..].chunks_exact(2)) {
            sums[0] ^= a[0].mul_unreduced(b[0]);
            sums[1] ^= a[1].mul_unreduced(b[1]);
            sums[2] ^= (a[0] + a[1]).mul_unreduced(b[0] + b[1]);
        }
        sums
    }
}

#[inline]
#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq,pclmulqdq,sse4.1")]
unsafe fn fold_eight(input: *const F128, challenge: __m512i) -> __m512i {
    // SAFETY: caller guarantees eight readable elements and required features.
    unsafe {
        let (even, odd) = separate_pairs(f128x4_loadu(input), f128x4_loadu(input.add(4)));
        _mm512_xor_si512(even, ghash_mul_x4(_mm512_xor_si512(even, odd), challenge))
    }
}

/// Fold sixteen inputs to eight outputs, and accumulate the next message
/// directly from those output registers before their single write to memory.
///
/// # Safety
/// Requires AVX-512F/BW, VPCLMULQDQ, PCLMULQDQ, and SSE4.1.
#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq,pclmulqdq,sse4.1")]
pub(super) unsafe fn fold_chunk(
    a: &[F128],
    b: &[F128],
    a_out: &mut [MaybeUninit<F128>],
    b_out: &mut [MaybeUninit<F128>],
    challenge: F128,
) -> [F256Unreduced; 2] {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), 2 * a_out.len());
    assert_eq!(a_out.len(), b_out.len());
    assert_eq!(a.len() % 4, 0);
    // SAFETY: every iteration reads sixteen inputs and writes eight outputs,
    // bounded by bulk. Separate mutable slices prevent input/output aliasing.
    unsafe {
        let r = f128x4_set(challenge, challenge, challenge, challenge);
        let mut zero = WideGhashX4::zero();
        let mut infinity = WideGhashX4::zero();
        let bulk = a.len() / 16 * 16;
        for offset in (0..bulk).step_by(16) {
            let al = fold_eight(a.as_ptr().add(offset), r);
            let ah = fold_eight(a.as_ptr().add(offset + 8), r);
            let bl = fold_eight(b.as_ptr().add(offset), r);
            let bh = fold_eight(b.as_ptr().add(offset + 8), r);
            let (a0, a1) = separate_pairs(al, ah);
            let (b0, b1) = separate_pairs(bl, bh);
            zero.mul_acc(a0, b0);
            infinity.mul_acc(_mm512_xor_si512(a0, a1), _mm512_xor_si512(b0, b1));
            _mm512_storeu_si512(a_out.as_mut_ptr().add(offset / 2).cast(), al);
            _mm512_storeu_si512(a_out.as_mut_ptr().add(offset / 2 + 4).cast(), ah);
            _mm512_storeu_si512(b_out.as_mut_ptr().add(offset / 2).cast(), bl);
            _mm512_storeu_si512(b_out.as_mut_ptr().add(offset / 2 + 4).cast(), bh);
        }
        let tail = super::fold_chunk_uninit_scalar(
            &a[bulk..],
            &b[bulk..],
            &mut a_out[bulk / 2..],
            &mut b_out[bulk / 2..],
            challenge,
        );
        [zero.fold() ^ tail[0], infinity.fold() ^ tail[1]]
    }
}
