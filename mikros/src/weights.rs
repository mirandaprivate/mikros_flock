//! Blocked construction of HighToLow equality and arbitrary pair tensors.
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

/// Small arbitrary-pair tensor in natural index order. The first factor is
/// the highest index bit, just as in `fill`. Each doubling is serial and uses
/// the one allocation; pairs need not sum to one or even be nonzero.
fn small_tensor_into(high_first: &[[F128; 2]], output: &mut Vec<F128>) {
    let length = 1usize
        .checked_shl(high_first.len().try_into().expect("tensor size"))
        .expect("tensor size");
    output.clear();
    output.resize(length, F128::ZERO);
    output[0] = F128::ONE;
    for (axis, &[a, b]) in high_first.iter().rev().enumerate() {
        let half = 1usize << axis;
        let (low, high) = output.split_at_mut(half);
        for (low, high) in low.iter_mut().zip(&mut high[..half]) {
            let value = *low;
            if value.is_zero() {
                *low = F128::ZERO;
                *high = F128::ZERO;
            } else {
                *low = if a.is_zero() {
                    F128::ZERO
                } else if a == F128::ONE {
                    value
                } else {
                    value * a
                };
                *high = if b.is_zero() {
                    F128::ZERO
                } else if b == F128::ONE {
                    value
                } else {
                    value * b
                };
            }
        }
    }
}

/// Fill `output[i] = product_j high_first[j][bit_(n-1-j)(i)]`.
/// This is valid for arbitrary F128 pairs, including [0,0] and [t,t]; it
/// performs no normalization or inversion. An empty tensor contains ONE.
/// Reuse only output capacity, rebuilding every challenge-dependent value.
pub(crate) fn fill_tensor(high_first: &[[F128; 2]], output: &mut Vec<F128>) {
    let length = 1usize
        .checked_shl(high_first.len().try_into().expect("tensor size"))
        .expect("tensor size");
    if high_first.len() <= 10 {
        small_tensor_into(high_first, output);
        return;
    }
    let split = high_first.len() - 10;
    let mut outer = Vec::new();
    let mut inner = Vec::new();
    small_tensor_into(&high_first[..split], &mut outer);
    small_tensor_into(&high_first[split..], &mut inner);
    output.clear();
    output.reserve(length);
    let uninit = &mut output.spare_capacity_mut()[..length];
    // One parallel output pass replaces a Rayon barrier and a new Vec at
    // each doubling level. The 10-bit inner factor stays cache resident.
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
    // SAFETY: every output block was fully initialized by the existing
    // scale kernel. All parallel tasks joined; panic leaves the Vec empty.
    unsafe { output.set_len(length) };
}

#[cfg(test)]
mod tests {
    use super::{fill, fill_tensor};
    use flock_field::{F128, test_rng::Rng};
    use flock_multilinear::{IndexOrder, eq_table};

    #[test]
    fn blocked_arbitrary_tensor_matches_scalar_and_functional_with_reuse() {
        let pools: Vec<_> = [1, 3]
            .into_iter()
            .map(|threads| {
                rayon::ThreadPoolBuilder::new()
                    .num_threads(threads)
                    .build()
                    .unwrap()
            })
            .collect();
        let mut buffers = [
            Vec::with_capacity((1 << 17) + 33),
            Vec::with_capacity((1 << 17) + 33),
        ];
        let addresses = buffers.each_ref().map(|v| v.as_ptr() as usize);
        let capacities = buffers.each_ref().map(|v| v.capacity());
        let mut rng = Rng::new(0xB10C_A2B1);
        for n in (0..=17).chain([9, 0, 16, 1]) {
            for shape in 0..5 {
                let mut pairs: Vec<_> = (0..n).map(|_| [rng.f128(), rng.f128()]).collect();
                if n != 0 {
                    match shape {
                        1 => pairs[n / 2] = [F128::ZERO; 2],
                        2 => pairs[n / 2] = [F128::new(31, 57); 2],
                        3 => pairs.iter_mut().enumerate().for_each(|(j, pair)| {
                            *pair = if j & 1 == 0 {
                                [F128::ZERO, F128::ONE]
                            } else {
                                [F128::ONE, F128::ZERO]
                            };
                        }),
                        4 => {
                            pairs[0] = [F128::ZERO, F128::ONE];
                            if n > 1 {
                                pairs[n - 1] = [F128::new(13, 17); 2];
                            }
                        }
                        _ => {}
                    }
                }
                // Independent direct index formula: no doubling/blocked
                // algorithm and no conversion to an ordinary MLE point.
                let expected: Vec<_> = (0..1usize << n)
                    .map(|i| {
                        pairs
                            .iter()
                            .enumerate()
                            .fold(F128::ONE, |product, (j, pair)| {
                                product * pair[(i >> (n - 1 - j)) & 1]
                            })
                    })
                    .collect();
                let low_first: Vec<_> = pairs.iter().rev().copied().collect();
                assert_eq!(
                    expected,
                    flock_core::pcs::functional::row_weights(&low_first)
                );
                for (index, pool) in pools.iter().enumerate() {
                    let out = &mut buffers[index];
                    out.clear();
                    out.resize((1usize << n) / 2 + 1, F128::new(0xD17D_17, 0xBAD0_CAFE));
                    pool.install(|| fill_tensor(&pairs, out));
                    assert_eq!(
                        *out,
                        expected,
                        "n={n} shape={shape} threads={}",
                        pool.current_num_threads()
                    );
                    assert_eq!(out.capacity(), capacities[index]);
                    assert_eq!(out.as_ptr() as usize, addresses[index]);
                }
            }
        }
    }

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
