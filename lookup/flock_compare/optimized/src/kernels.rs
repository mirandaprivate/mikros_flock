use flock_field::F128;
use std::mem::{ManuallyDrop, MaybeUninit};

pub(crate) fn uninit_f128(len: usize) -> Vec<MaybeUninit<F128>> {
    let mut values = Vec::with_capacity(len);
    values.resize_with(len, MaybeUninit::uninit);
    values
}
/// Caller must initialize every element; capacity beyond len may be uninitialized.
pub(crate) unsafe fn initialized_f128(values: Vec<MaybeUninit<F128>>) -> Vec<F128> {
    let mut values = ManuallyDrop::new(values);
    // SAFETY: guaranteed by caller; MaybeUninit<F128> and F128 have identical
    // layout. ManuallyDrop transfers this allocation exactly once.
    unsafe { Vec::from_raw_parts(values.as_mut_ptr().cast(), values.len(), values.capacity()) }
}
pub(crate) fn fold_interleaved(src: &[F128], base: usize, a: &mut [F128], b: &mut [F128], r: F128) {
    assert_eq!(a.len(), b.len());
    assert!(4 * (base + a.len()) <= src.len());
    let mut i = 0;
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq"
    ))]
    {
        use flock_field::gf2_128::x86_64::{f128x4_set, ghash_mul_x4};
        use std::arch::x86_64::{_mm512_storeu_si512, _mm512_xor_si512};
        // SAFETY: cfg enables intrinsics, loop bounds cover sixteen readable
        // source elements and four writable elements in each output.
        unsafe {
            let rr = f128x4_set(r, r, r, r);
            while i + 4 <= a.len() {
                let j = 4 * (base + i);
                let a0 = f128x4_set(src[j], src[j + 4], src[j + 8], src[j + 12]);
                let a1 = f128x4_set(src[j + 2], src[j + 6], src[j + 10], src[j + 14]);
                let b0 = f128x4_set(src[j + 1], src[j + 5], src[j + 9], src[j + 13]);
                let b1 = f128x4_set(src[j + 3], src[j + 7], src[j + 11], src[j + 15]);
                let av = _mm512_xor_si512(a0, ghash_mul_x4(rr, _mm512_xor_si512(a0, a1)));
                let bv = _mm512_xor_si512(b0, ghash_mul_x4(rr, _mm512_xor_si512(b0, b1)));
                _mm512_storeu_si512(a.as_mut_ptr().add(i).cast(), av);
                _mm512_storeu_si512(b.as_mut_ptr().add(i).cast(), bv);
                i += 4;
            }
        }
    }
    while i < a.len() {
        let j = 4 * (base + i);
        a[i] = src[j] + r * (src[j] + src[j + 2]);
        b[i] = src[j + 1] + r * (src[j + 1] + src[j + 3]);
        i += 1;
    }
}
pub(crate) fn multiply_pairs(src: &[F128], base: usize, dst: &mut [MaybeUninit<F128>]) {
    assert!(2 * (base + dst.len()) <= src.len());
    let mut i = 0;
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq"
    ))]
    {
        use flock_field::gf2_128::x86_64::{f128x4_set, ghash_mul_x4};
        use std::arch::x86_64::_mm512_storeu_si512;
        while i + 4 <= dst.len() {
            let j = 2 * (base + i);
            // SAFETY: feature gate enables the intrinsics; all eight input
            // elements and all four destination elements are in bounds.
            unsafe {
                let a = f128x4_set(src[j], src[j + 2], src[j + 4], src[j + 6]);
                let b = f128x4_set(src[j + 1], src[j + 3], src[j + 5], src[j + 7]);
                _mm512_storeu_si512(dst.as_mut_ptr().add(i).cast(), ghash_mul_x4(a, b));
            }
            i += 4;
        }
    }
    while i < dst.len() {
        let j = 2 * (base + i);
        dst[i].write(src[j] * src[j + 1]);
        i += 1;
    }
}
