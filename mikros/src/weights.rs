//! Blocked construction of the same HighToLow equality tensor.
//! Small factors stay in cache; independent output blocks avoid a barrier at
//! every large doubling level. All random-point work remains inside prove.

use flock_field::F128;
use rayon::prelude::{IndexedParallelIterator, ParallelIterator, ParallelSliceMut};
use std::mem::MaybeUninit;

fn small(point: &[F128]) -> Vec<F128> {
    let mut out = vec![F128::ZERO; 1 << point.len()];
    out[0] = F128::ONE;
    for (axis, &coordinate) in point.iter().rev().enumerate() {
        let half = 1 << axis;
        let (low, high) = out.split_at_mut(half);
        for (low, high) in low.iter_mut().zip(&mut high[..half]) {
            let value = *low;
            *high = value * coordinate;
            *low = value + *high;
        }
    }
    out
}

fn scale(input: &[F128], factor: F128, out: &mut [MaybeUninit<F128>]) {
    assert_eq!(input.len(), out.len());
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "pclmulqdq",
        target_feature = "sse4.1"
    ))]
    {
        // SAFETY: cfg supplies the features; scale_x4 checks no extra bounds.
        unsafe { scale_x4(input, factor, out) };
    }
    #[cfg(not(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "pclmulqdq",
        target_feature = "sse4.1"
    )))]
    for (out, &input) in out.iter_mut().zip(input) {
        out.write(input * factor);
    }
}

#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512f",
    target_feature = "avx512bw",
    target_feature = "vpclmulqdq",
    target_feature = "pclmulqdq",
    target_feature = "sse4.1"
))]
#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq,pclmulqdq,sse4.1")]
unsafe fn scale_x4(input: &[F128], factor: F128, out: &mut [MaybeUninit<F128>]) {
    use core::arch::x86_64::_mm512_storeu_si512;
    use flock_field::gf2_128::x86_64::{f128x4_loadu, f128x4_set, ghash_mul_x4};
    // SAFETY: both slices have equal length; each vector accesses four F128.
    unsafe {
        let factors = f128x4_set(factor, factor, factor, factor);
        let full = input.len() / 4 * 4;
        for index in (0..full).step_by(4) {
            let product = ghash_mul_x4(f128x4_loadu(input.as_ptr().add(index)), factors);
            _mm512_storeu_si512(out.as_mut_ptr().add(index).cast(), product);
        }
        for index in full..input.len() {
            out[index].write(input[index] * factor);
        }
    }
}

pub(crate) fn fill(point: &[F128], output: &mut Vec<F128>) {
    let length = 1usize
        .checked_shl(point.len().try_into().expect("point size"))
        .expect("point size");
    if point.len() <= 10 {
        let values = small(point);
        output.clear();
        output.extend_from_slice(&values);
        return;
    }
    let split = point.len() - 10;
    let outer = small(&point[..split]);
    let inner = small(&point[split..]);
    output.clear();
    output.reserve(length);
    // Keep length zero until every block has been initialized. On panic the
    // allocation is still safely droppable; no uninitialized F128 is exposed.
    let uninit = &mut output.spare_capacity_mut()[..length];
    if rayon::current_num_threads() > 1 && length >= 32768 {
        uninit
            .par_chunks_mut(inner.len())
            .zip(outer)
            .for_each(|(out, factor)| scale(&inner, factor, out));
    } else {
        for (out, factor) in uninit.chunks_mut(inner.len()).zip(outer) {
            scale(&inner, factor, out);
        }
    }
    // SAFETY: outer.len()*inner.len()==length; scale writes every element of
    // each disjoint output block, and all Rayon tasks have joined successfully.
    unsafe { output.set_len(length) };
}

#[cfg(test)]
mod tests {
    use super::fill;
    use flock_field::{F128, test_rng::Rng};
    use flock_multilinear::{IndexOrder, eq_table};

    #[test]
    fn blocked_weights_match_original_across_lengths_points_and_reuse() {
        let mut rng = Rng::new(0xB10C_E0);
        let mut scratch = Vec::new();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(3)
            .build()
            .unwrap();
        for n in [0, 1, 7, 10, 11, 15, 17, 2, 0, 16] {
            for point in [rng.f128_vec(n), vec![F128::ZERO; n], vec![F128::ONE; n]] {
                pool.install(|| fill(&point, &mut scratch));
                assert_eq!(scratch, eq_table(&point, F128::ONE, IndexOrder::HighToLow));
            }
        }
    }
}
