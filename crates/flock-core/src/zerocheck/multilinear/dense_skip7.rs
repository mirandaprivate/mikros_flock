//! Dense 128-bit univariate-skip folds with the existing round-2 message.
//! Skip-6, sparse, compact, and lookahead paths remain in the parent module.

use std::mem::MaybeUninit;

use rayon::prelude::{IndexedParallelIterator, ParallelIterator, ParallelSliceMut};

use super::{F128, F256Unreduced, SplitEqGhash, UniSkipFoldTable, take_f128};

const ROW_BYTES: usize = 16;

/// Dense counterpart of the parent kernel, with the same split-equality
/// weighting and process-global F128 allocation pool. Every worker initializes
/// its actual outputs; no serial full-vector zero fill is needed.
pub(super) fn fold(
    a_packed: &[u8],
    b_packed: &[u8],
    m: usize,
    table: &UniSkipFoldTable,
    mlv_challenges: &[F128],
) -> (Vec<F128>, Vec<F128>, F128, F128) {
    assert!(m > 7 && m < usize::BITS as usize);
    assert_eq!(table.n_chunks, ROW_BYTES);
    assert_eq!(table.data.len(), ROW_BYTES * 256);
    let n_out = 1usize << (m - 7);
    let input_bytes = n_out.checked_mul(ROW_BYTES).expect("skip-7 input size");
    assert_eq!(a_packed.len(), input_bytes);
    assert_eq!(b_packed.len(), input_bytes);
    assert_eq!(mlv_challenges.len(), m - 7);

    let eq = SplitEqGhash::new(&mlv_challenges[1..]);
    let pairs_per_chunk = eq.lo.len();
    let output_chunk = 2 * pairs_per_chunk;
    assert_eq!(output_chunk * eq.hi.len(), n_out);
    let mut a_folded = take_f128(n_out);
    let mut b_folded = take_f128(n_out);
    // Pool contents are not inputs. Keep Vec lengths zero until all workers
    // have written their outputs; F128 is Copy, so partial writes need no drop.
    a_folded.clear();
    b_folded.clear();
    let a_storage = &mut a_folded.spare_capacity_mut()[..n_out];
    let b_storage = &mut b_folded.spare_capacity_mut()[..n_out];
    let (sum_one, sum_infinity) = a_storage
        .par_chunks_mut(output_chunk)
        .zip(b_storage.par_chunks_mut(output_chunk))
        .enumerate()
        .map(|(high, (a_out, b_out))| {
            let start = high * output_chunk * ROW_BYTES;
            let end = start + output_chunk * ROW_BYTES;
            let (one, infinity) = fold_chunk(
                &a_packed[start..end],
                &b_packed[start..end],
                table,
                &eq.lo,
                a_out,
                b_out,
            );
            (eq.hi[high] * one.reduce(), eq.hi[high] * infinity.reduce())
        })
        .reduce(|| (F128::ZERO, F128::ZERO), |(a, b), (c, d)| (a + c, b + d));
    // SAFETY: each disjoint output chunk was completely initialized by its
    // scalar or SIMD kernel, including every tail. Rayon joined all workers.
    // A panic skips this commit and leaves two valid empty vectors.
    unsafe {
        a_folded.set_len(n_out);
        b_folded.set_len(n_out);
    }
    (
        a_folded,
        b_folded,
        mlv_challenges[0] * sum_one,
        sum_infinity,
    )
}

fn fold_chunk_scalar(
    a: &[u8],
    b: &[u8],
    table: &UniSkipFoldTable,
    eq: &[F128],
    a_out: &mut [MaybeUninit<F128>],
    b_out: &mut [MaybeUninit<F128>],
) -> (F256Unreduced, F256Unreduced) {
    let mut one = F256Unreduced::ZERO;
    let mut infinity = F256Unreduced::ZERO;
    for (pair, &weight) in eq.iter().enumerate() {
        let start = pair * 2 * ROW_BYTES;
        let a0 = table.fold_one_row(&a[start..start + ROW_BYTES]);
        let a1 = table.fold_one_row(&a[start + ROW_BYTES..start + 2 * ROW_BYTES]);
        let b0 = table.fold_one_row(&b[start..start + ROW_BYTES]);
        let b1 = table.fold_one_row(&b[start + ROW_BYTES..start + 2 * ROW_BYTES]);
        a_out[2 * pair].write(a0);
        a_out[2 * pair + 1].write(a1);
        b_out[2 * pair].write(b0);
        b_out[2 * pair + 1].write(b1);
        one ^= weight.mul_unreduced(a1 * b1);
        infinity ^= weight.mul_unreduced((a0 + a1) * (b0 + b1));
    }
    (one, infinity)
}

fn fold_chunk(
    a: &[u8],
    b: &[u8],
    table: &UniSkipFoldTable,
    eq: &[F128],
    a_out: &mut [MaybeUninit<F128>],
    b_out: &mut [MaybeUninit<F128>],
) -> (F256Unreduced, F256Unreduced) {
    assert_eq!(table.n_chunks, ROW_BYTES);
    assert_eq!(table.data.len(), ROW_BYTES * 256);
    assert_eq!(a.len(), eq.len() * 2 * ROW_BYTES);
    assert_eq!(a.len(), b.len());
    assert_eq!(a_out.len(), 2 * eq.len());
    assert_eq!(a_out.len(), b_out.len());
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "pclmulqdq",
        target_feature = "sse4.1"
    ))]
    {
        // SAFETY: compile-time gates supply every required feature. Shape
        // assertions bound the 16-byte rows, full tables, outputs and eq loads.
        unsafe { fold_chunk_x86(a, b, table, eq, a_out, b_out) }
    }
    #[cfg(not(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "pclmulqdq",
        target_feature = "sse4.1"
    )))]
    fold_chunk_scalar(a, b, table, eq, a_out, b_out)
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
unsafe fn fold_chunk_x86(
    a: &[u8],
    b: &[u8],
    table: &UniSkipFoldTable,
    eq: &[F128],
    a_out: &mut [MaybeUninit<F128>],
    b_out: &mut [MaybeUninit<F128>],
) -> (F256Unreduced, F256Unreduced) {
    use super::kernels::x86_64::fold_round2_pair_x86_unchecked_8;
    use crate::field::gf2_128::x86_64::{WideGhashX4, f128x4_loadu, f128x4_set, ghash_mul_x4};

    // SAFETY: the safe dispatcher checked all shapes. Each four-pair batch
    // reads eight complete 16-byte rows and initializes eight output slots.
    unsafe {
        let mut one = WideGhashX4::zero();
        let mut infinity = WideGhashX4::zero();
        let bulk = eq.len() / 4 * 4;
        for pair in (0..bulk).step_by(4) {
            let mut a0 = [F128::ZERO; 4];
            let mut a1 = [F128::ZERO; 4];
            let mut b0 = [F128::ZERO; 4];
            let mut b1 = [F128::ZERO; 4];
            for lane in 0..4 {
                let start = (pair + lane) * 2 * ROW_BYTES;
                // Reuse the unchanged 64-bit row kernel twice. The second
                // pass reads the upper eight bytes with tables 8..16.
                let low = fold_round2_pair_x86_unchecked_8(
                    table.data.as_ptr(),
                    a.as_ptr().add(start),
                    a.as_ptr().add(start + ROW_BYTES),
                    b.as_ptr().add(start),
                    b.as_ptr().add(start + ROW_BYTES),
                );
                let high = fold_round2_pair_x86_unchecked_8(
                    table.data.as_ptr().add(8 * 256),
                    a.as_ptr().add(start + 8),
                    a.as_ptr().add(start + ROW_BYTES + 8),
                    b.as_ptr().add(start + 8),
                    b.as_ptr().add(start + ROW_BYTES + 8),
                );
                a0[lane] = low[0] + high[0];
                a1[lane] = low[1] + high[1];
                b0[lane] = low[2] + high[2];
                b1[lane] = low[3] + high[3];
                a_out[2 * (pair + lane)].write(a0[lane]);
                a_out[2 * (pair + lane) + 1].write(a1[lane]);
                b_out[2 * (pair + lane)].write(b0[lane]);
                b_out[2 * (pair + lane) + 1].write(b1[lane]);
            }
            let g_one = ghash_mul_x4(f128x4_loadu(a1.as_ptr()), f128x4_loadu(b1.as_ptr()));
            let a_sum = f128x4_set(a0[0] + a1[0], a0[1] + a1[1], a0[2] + a1[2], a0[3] + a1[3]);
            let b_sum = f128x4_set(b0[0] + b1[0], b0[1] + b1[1], b0[2] + b1[2], b0[3] + b1[3]);
            let g_infinity = ghash_mul_x4(a_sum, b_sum);
            let weights = f128x4_loadu(eq.as_ptr().add(pair));
            one.mul_acc(weights, g_one);
            infinity.mul_acc(weights, g_infinity);
        }
        let (one_tail, infinity_tail) = fold_chunk_scalar(
            &a[bulk * 2 * ROW_BYTES..],
            &b[bulk * 2 * ROW_BYTES..],
            table,
            &eq[bulk..],
            &mut a_out[bulk * 2..],
            &mut b_out[bulk * 2..],
        );
        (one.fold() ^ one_tail, infinity.fold() ^ infinity_tail)
    }
}

#[cfg(test)]
mod tests {
    use super::{F128, MaybeUninit, ROW_BYTES, UniSkipFoldTable, fold_chunk};
    use crate::{
        field::PHI_8_TABLE,
        test_rng::Rng,
        zerocheck::{
            PaddingSpec,
            multilinear::{
                pack_bits, uni_skip_fold_and_round_pair_naive,
                uni_skip_fold_and_round_pair_optimized_packed,
                uni_skip_fold_and_round_pair_optimized_packed_padded,
            },
        },
    };

    #[test]
    fn skip7_dense_folds_and_messages_match_naive_128_bit_reference() {
        let mut rng = Rng::new(0x5717_1280);
        let pools: Vec<_> = [1, 3]
            .into_iter()
            .map(|threads| {
                rayon::ThreadPoolBuilder::new()
                    .num_threads(threads)
                    .build()
                    .unwrap()
            })
            .collect();
        // m=8 exercises two folded outputs; m>=17 reaches four-pair SIMD
        // batches under SplitEqGhash's seven-bit outer split.
        for m in [8, 9, 10, 16, 17, 18] {
            let a = rng.bits(1 << m);
            let b = rng.bits(1 << m);
            let a_packed = pack_bits(&a);
            let b_packed = pack_bits(&b);
            for z in [PHI_8_TABLE[0], PHI_8_TABLE[127], rng.f128()] {
                let table = UniSkipFoldTable::new(7, z);
                assert_eq!(table.n_chunks, 16);
                for challenges in [
                    rng.f128_vec(m - 7),
                    vec![F128::ZERO; m - 7],
                    vec![F128::ONE; m - 7],
                ] {
                    let expected = uni_skip_fold_and_round_pair_naive(&a, &b, m, 7, z, &challenges);
                    for pool in &pools {
                        let actual = pool.install(|| {
                            uni_skip_fold_and_round_pair_optimized_packed(
                                &a_packed,
                                &b_packed,
                                m,
                                7,
                                &table,
                                &challenges,
                            )
                        });
                        assert_eq!(actual, expected, "m={m}, z={z:?}");
                    }
                }
            }
        }
    }

    #[test]
    fn skip7_dense_chunk_vector_boundaries_match_arbitrary_table_sums() {
        let mut rng = Rng::new(0x5717_BA7C);
        let table = UniSkipFoldTable {
            n_chunks: ROW_BYTES,
            data: rng.f128_vec(ROW_BYTES * 256),
        };
        for pairs in [0, 1, 2, 3, 4, 5, 7, 8, 9, 127, 128, 129] {
            let mut a = vec![0u8; pairs * 2 * ROW_BYTES];
            let mut b = a.clone();
            rng.fill_bytes(&mut a);
            rng.fill_bytes(&mut b);
            let eq = rng.f128_vec(pairs);
            let direct = |bytes: &[u8]| {
                bytes
                    .chunks_exact(ROW_BYTES)
                    .map(|row| {
                        row.iter()
                            .enumerate()
                            .fold(F128::ZERO, |sum, (chunk, &byte)| {
                                sum + table.data[chunk * 256 + byte as usize]
                            })
                    })
                    .collect::<Vec<_>>()
            };
            let expected_a = direct(&a);
            let expected_b = direct(&b);
            let mut expected = (F128::ZERO, F128::ZERO);
            for pair in 0..pairs {
                let i = 2 * pair;
                expected.0 += eq[pair] * expected_a[i + 1] * expected_b[i + 1];
                expected.1 += eq[pair]
                    * (expected_a[i] + expected_a[i + 1])
                    * (expected_b[i] + expected_b[i + 1]);
            }
            let mut actual_a = vec![MaybeUninit::uninit(); 2 * pairs];
            let mut actual_b = actual_a.clone();
            let actual = fold_chunk(&a, &b, &table, &eq, &mut actual_a, &mut actual_b);
            assert_eq!((actual.0.reduce(), actual.1.reduce()), expected);
            // SAFETY: fold_chunk's checked shape covers every output slot,
            // including zero-length vectors and the one-to-three-pair tails.
            let actual_a: Vec<_> = actual_a
                .into_iter()
                .map(|v| unsafe { v.assume_init() })
                .collect();
            let actual_b: Vec<_> = actual_b
                .into_iter()
                .map(|v| unsafe { v.assume_init() })
                .collect();
            assert_eq!(actual_a, expected_a, "pairs={pairs}");
            assert_eq!(actual_b, expected_b, "pairs={pairs}");
        }
    }

    #[test]
    #[should_panic(expected = "skip-7 optimized fold requires dense padding")]
    fn skip7_dense_entry_rejects_sparse_padding() {
        let table = UniSkipFoldTable::new(7, F128::ZERO);
        uni_skip_fold_and_round_pair_optimized_packed_padded(
            &[0; 32],
            &[0; 32],
            8,
            7,
            &table,
            &[F128::ONE],
            &PaddingSpec::uniform(8, 128, 1),
        );
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn skip7_dense_entry_rejects_truncated_public_table() {
        let mut table = UniSkipFoldTable::new(7, F128::ZERO);
        table.data.pop();
        uni_skip_fold_and_round_pair_optimized_packed(
            &[0; 32],
            &[0; 32],
            8,
            7,
            &table,
            &[F128::ONE],
        );
    }
}
