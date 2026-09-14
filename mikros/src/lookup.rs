//! Binary chunk tables for tensor transforms and challenge-point binding.

use core::mem::MaybeUninit;
use flock_field::{F128, F256Unreduced};
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelIterator, ParallelIterator, ParallelSlice,
    ParallelSliceMut,
};

use crate::tensor::{KAPPA, Kernel};

const ROW_BYTES: usize = KAPPA / 8;

#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512f",
    target_feature = "avx512bw",
    target_feature = "vpclmulqdq",
    target_feature = "pclmulqdq",
    target_feature = "sse4.1"
))]
mod x86_64;

/// `T^{tensor 3}` maps eight input bits to 27 coefficient bits. This table
/// is witness- and challenge-independent, and occupies exactly one KiB.
pub(crate) static BYTE_TRANSFORM: [u32; 256] = make_byte_transform();

const fn make_byte_transform() -> [u32; 256] {
    let mut columns = [0u32; 8];
    let mut column = 0;
    while column < 8 {
        let mut coefficient = 0;
        while coefficient < 27 {
            let mut ternary = coefficient;
            let mut binary = column;
            let mut matches = true;
            let mut axis = 0;
            while axis < 3 {
                let digit = ternary % 3;
                if digit != 2 && digit != binary % 2 {
                    matches = false;
                }
                ternary /= 3;
                binary /= 2;
                axis += 1;
            }
            if matches {
                columns[column] |= 1 << coefficient;
            }
            coefficient += 1;
        }
        column += 1;
    }
    let mut table = [0u32; 256];
    let mut byte = 1usize;
    while byte < 256 {
        let bit = byte.trailing_zeros() as usize;
        table[byte] = table[byte & (byte - 1)] ^ columns[bit];
        byte += 1;
    }
    table
}

/// Bind each bit-packed 128-bit row to the supplied 128 column weights.
///
/// Lookup mode builds sixteen byte subset-XOR tables (64 KiB) for these
/// weights once per call, then performs sixteen lookups per row. Unlike the
/// fixed tensor table, these tables depend on the challenge; their creation
/// belongs in online packing time. There is no heap allocation per row.
pub fn pack_rows(packed: &[u8], weights: &[F128], kernel: Kernel) -> Vec<F128> {
    assert_eq!(weights.len(), KAPPA, "packing requires 128 column weights");
    assert_eq!(
        packed.len() % ROW_BYTES,
        0,
        "input must contain complete rows"
    );
    match kernel {
        Kernel::Reference => packed
            .chunks_exact(ROW_BYTES)
            .map(|row| {
                weights
                    .iter()
                    .enumerate()
                    .fold(F128::ZERO, |sum, (column, &weight)| {
                        if row[column / 8] >> (column % 8) & 1 != 0 {
                            sum + weight
                        } else {
                            sum
                        }
                    })
            })
            .collect(),
        Kernel::Lookup
        | Kernel::Packed
        | Kernel::Packed8
        | Kernel::PackedLegacy
        | Kernel::PackedStream
        | Kernel::PackedSimd
        | Kernel::PackedSimdV1
        | Kernel::PackedWide
        | Kernel::PackedFused
        | Kernel::PackedNative
        | Kernel::PackedFactored => {
            let mut tables = vec![[F128::ZERO; 256]; ROW_BYTES];
            for (byte, table) in tables.iter_mut().enumerate() {
                for value in 1usize..256 {
                    let bit = value.trailing_zeros() as usize;
                    table[value] = table[value ^ (1 << bit)] + weights[byte * 8 + bit];
                }
            }
            packed
                .par_chunks_exact(ROW_BYTES)
                .with_min_len(256)
                .map(|row| {
                    row.iter()
                        .zip(&tables)
                        .fold(F128::ZERO, |sum, (&byte, table)| sum + table[byte as usize])
                })
                .collect()
        }
    }
}

/// A challenge-dependent table shared by A and B. Its construction stays in
/// online proving time; unlike BYTE_TRANSFORM it cannot be cached globally.
pub(crate) struct BindingTable {
    data: Vec<[F128; 256]>,
}

impl BindingTable {
    pub(crate) fn new(weights: &[F128]) -> Self {
        assert_eq!(weights.len(), KAPPA);
        let mut data = vec![[F128::ZERO; 256]; ROW_BYTES];
        for (byte, table) in data.iter_mut().enumerate() {
            for bit in 0..8 {
                let half = 1 << bit;
                for mask in 0..half {
                    table[half + mask] = table[mask] + weights[byte * 8 + bit];
                }
            }
        }
        Self { data }
    }

    #[inline]
    pub(crate) fn fold(&self, row: &[u8]) -> F128 {
        let mut sums = [F128::ZERO; 4];
        for (bytes, tables) in row.chunks_exact(4).zip(self.data.chunks_exact(4)) {
            sums[0] += tables[0][bytes[0] as usize];
            sums[1] += tables[1][bytes[1] as usize];
            sums[2] += tables[2][bytes[2] as usize];
            sums[3] += tables[3][bytes[3] as usize];
        }
        sums[0] + sums[1] + sums[2] + sums[3]
    }
}

fn weight_native(values: &mut [F128], weights: &[F128]) {
    assert_eq!(values.len(), weights.len());
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "pclmulqdq",
        target_feature = "sse4.1"
    ))]
    {
        // SAFETY: feature gates match the kernel; it bounds all vector loads
        // and stores, with scalar handling of the last zero to three values.
        unsafe { x86_64::weight(values, weights) }
    }
    #[cfg(not(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "pclmulqdq",
        target_feature = "sse4.1"
    )))]
    for (value, &weight) in values.iter_mut().zip(weights) {
        *value *= weight;
    }
}

/// Bind adjacent rows, apply B's weight, and produce the first IP message in
/// one input pass. Only eight packed A/B rows are live before their output
/// write, avoiding a second complete scan of the two packed vectors.
pub(crate) fn pack_pair_weighted_with_message_into(
    a: &[u8],
    b: &[u8],
    column_weights: &[F128],
    row_weights: &[F128],
    a_out: &mut Vec<F128>,
    b_out: &mut Vec<F128>,
) -> [F128; 2] {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), row_weights.len() * ROW_BYTES);
    let table = BindingTable::new(column_weights);
    let len = row_weights.len();
    let a_storage = crate::inner_product::prepare_output(a_out, len);
    let b_storage = crate::inner_product::prepare_output(b_out, len);
    let bind_chunk = |start: usize,
                      a_out: &mut [MaybeUninit<F128>],
                      b_out: &mut [MaybeUninit<F128>]| {
        let mut sums = [F256Unreduced::ZERO; 2];
        for offset in (0..a_out.len()).step_by(8) {
            let take = (a_out.len() - offset).min(8);
            let mut a_values = [F128::ZERO; 8];
            let mut b_values = [F128::ZERO; 8];
            for i in 0..take {
                let input = (start + offset + i) * ROW_BYTES;
                a_values[i] = table.fold(&a[input..input + ROW_BYTES]);
                b_values[i] = table.fold(&b[input..input + ROW_BYTES]);
            }
            weight_native(
                &mut b_values[..take],
                &row_weights[start + offset..start + offset + take],
            );
            let message =
                crate::inner_product::native_message_chunk(&a_values[..take], &b_values[..take]);
            sums[0] ^= message[0];
            sums[1] ^= message[1];
            for i in 0..take {
                a_out[offset + i].write(a_values[i]);
                b_out[offset + i].write(b_values[i]);
            }
        }
        sums
    };
    // Every chunk starts on an even row, so adjacent-pair message summands
    // never cross a task boundary. A possible final odd row contributes none.
    let sums = if row_weights.len() < 4096 || rayon::current_num_threads() == 1 {
        bind_chunk(0, a_storage, b_storage)
    } else {
        a_storage
            .par_chunks_mut(1024)
            .zip(b_storage.par_chunks_mut(1024))
            .enumerate()
            .map(|(chunk, (a_out, b_out))| bind_chunk(chunk * 1024, a_out, b_out))
            .reduce(
                || [F256Unreduced::ZERO; 2],
                |a, b| [a[0] ^ b[0], a[1] ^ b[1]],
            )
    };
    // SAFETY: every element in both len-sized output slices is written by
    // exactly one bind_chunk task. Rayon has joined all tasks before returning.
    // On panic this block is never reached and both vectors remain empty.
    unsafe {
        a_out.set_len(len);
        b_out.set_len(len);
    }
    sums.map(F256Unreduced::reduce)
}

/// The same binding and first IP message as the original fused packing pass.
/// On supported native builds, each task retains its two four-lane product
/// accumulators across all 1024 rows and horizontally folds only at task end.
/// The original entry point remains the baseline and the portable fallback.
pub(crate) fn pack_pair_weighted_with_wide_message_into(
    a: &[u8],
    b: &[u8],
    column_weights: &[F128],
    row_weights: &[F128],
    a_out: &mut Vec<F128>,
    b_out: &mut Vec<F128>,
) -> [F128; 2] {
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "pclmulqdq",
        target_feature = "sse4.1"
    ))]
    {
        assert_eq!(a.len(), b.len());
        assert_eq!(a.len(), row_weights.len() * ROW_BYTES);
        let table = BindingTable::new(column_weights);
        let len = row_weights.len();
        let a_storage = crate::inner_product::prepare_output(a_out, len);
        let b_storage = crate::inner_product::prepare_output(b_out, len);
        let bind_chunk =
            |start: usize, a_out: &mut [MaybeUninit<F128>], b_out: &mut [MaybeUninit<F128>]| {
                let end = start + a_out.len();
                // SAFETY: compile-time gates match the native kernel. Every
                // input slice contains the complete rows corresponding to its
                // disjoint output slice; the kernel also checks these dimensions.
                unsafe {
                    x86_64::pack_pair_wide(
                        &table,
                        &a[start * ROW_BYTES..end * ROW_BYTES],
                        &b[start * ROW_BYTES..end * ROW_BYTES],
                        &row_weights[start..end],
                        a_out,
                        b_out,
                    )
                }
            };
        let sums = if len < 4096 || rayon::current_num_threads() == 1 {
            bind_chunk(0, a_storage, b_storage)
        } else {
            a_storage
                .par_chunks_mut(1024)
                .zip(b_storage.par_chunks_mut(1024))
                .enumerate()
                .map(|(chunk, (a_out, b_out))| bind_chunk(chunk * 1024, a_out, b_out))
                .reduce(
                    || [F256Unreduced::ZERO; 2],
                    |a, b| [a[0] ^ b[0], a[1] ^ b[1]],
                )
        };
        // SAFETY: every output element was initialized exactly once and all
        // tasks have joined. A panic before this commit leaves empty vectors.
        unsafe {
            a_out.set_len(len);
            b_out.set_len(len);
        }
        sums.map(F256Unreduced::reduce)
    }
    #[cfg(not(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "pclmulqdq",
        target_feature = "sse4.1"
    )))]
    pack_pair_weighted_with_message_into(a, b, column_weights, row_weights, a_out, b_out)
}

/// Four-lane deferred product accumulation for C's projection. The shared
/// binding table is still computed online from this proof's column challenge.
pub(crate) fn project_rows_native(c: &[u8], column_weights: &[F128], row_weights: &[F128]) -> F128 {
    assert_eq!(c.len(), row_weights.len() * ROW_BYTES);
    let table = BindingTable::new(column_weights);
    let project = |(bytes, weights): (&[u8], &[F128])| {
        #[cfg(all(
            target_arch = "x86_64",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "vpclmulqdq",
            target_feature = "pclmulqdq",
            target_feature = "sse4.1"
        ))]
        {
            // SAFETY: compile-time feature gates match the kernel contract.
            unsafe { x86_64::project(&table, bytes, weights) }
        }
        #[cfg(not(all(
            target_arch = "x86_64",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "vpclmulqdq",
            target_feature = "pclmulqdq",
            target_feature = "sse4.1"
        )))]
        {
            let mut sum = F256Unreduced::ZERO;
            for (row, &weight) in bytes.chunks_exact(ROW_BYTES).zip(weights) {
                sum ^= table.fold(row).mul_unreduced(weight);
            }
            sum
        }
    };
    let sum = if row_weights.len() < 4096 || rayon::current_num_threads() == 1 {
        project((c, row_weights))
    } else {
        c.par_chunks(1024 * ROW_BYTES)
            .zip(row_weights.par_chunks(1024))
            .map(project)
            .reduce(|| F256Unreduced::ZERO, |a, b| a ^ b)
    };
    sum.reduce()
}

/// Pack A/B together with one rho table and apply B's row weights on write.
pub(crate) fn pack_pair_weighted(
    a: &[u8],
    b: &[u8],
    column_weights: &[F128],
    row_weights: &[F128],
) -> (Vec<F128>, Vec<F128>) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), row_weights.len() * ROW_BYTES);
    let table = BindingTable::new(column_weights);
    let bind = |row: usize| {
        let offset = row * ROW_BYTES;
        (
            table.fold(&a[offset..offset + ROW_BYTES]),
            table.fold(&b[offset..offset + ROW_BYTES]) * row_weights[row],
        )
    };
    if row_weights.len() < 4096 || rayon::current_num_threads() == 1 {
        (0..row_weights.len()).map(bind).unzip()
    } else {
        (0..row_weights.len())
            .into_par_iter()
            .with_min_len(1024)
            .map(bind)
            .unzip()
    }
}

/// Fuse C packing with its projection; do not materialize a packed C vector.
/// Reuse Flock's unreduced field product accumulator, reducing once per chunk.
pub(crate) fn project_rows(c: &[u8], column_weights: &[F128], row_weights: &[F128]) -> F128 {
    assert_eq!(c.len(), row_weights.len() * ROW_BYTES);
    let table = BindingTable::new(column_weights);
    let project = |(bytes, weights): (&[u8], &[F128])| {
        let mut sum = F256Unreduced::ZERO;
        for (row, &weight) in bytes.chunks_exact(ROW_BYTES).zip(weights) {
            sum ^= table.fold(row).mul_unreduced(weight);
        }
        sum.reduce()
    };
    if row_weights.len() < 4096 || rayon::current_num_threads() == 1 {
        project((c, row_weights))
    } else {
        c.par_chunks(1024 * ROW_BYTES)
            .zip(row_weights.par_chunks(1024))
            .map(project)
            .reduce(|| F128::ZERO, |a, b| a + b)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ROW_BYTES, pack_pair_weighted, pack_pair_weighted_with_message_into,
        pack_pair_weighted_with_wide_message_into, pack_rows, project_rows, project_rows_native,
    };
    use crate::tensor::{KAPPA, Kernel};
    use flock_field::{F128, test_rng::Rng};

    #[test]
    fn all_byte_subsets_at_every_position_match_direct_sum() {
        let mut rng = Rng::new(0xB17B1AD);
        let weights = rng.f128_vec(KAPPA);
        let mut packed = vec![0u8; ROW_BYTES * ROW_BYTES * 256];
        for position in 0..ROW_BYTES {
            for byte in 0..256 {
                packed[(position * 256 + byte) * ROW_BYTES + position] = byte as u8;
            }
        }
        assert_eq!(
            pack_rows(&packed, &weights, Kernel::Lookup),
            pack_rows(&packed, &weights, Kernel::Reference)
        );
    }

    #[test]
    fn packing_random_rows_and_coordinate_basis_agrees() {
        let mut rng = Rng::new(0xC010AB4515);
        let mut packed = vec![0u8; 517 * ROW_BYTES];
        rng.fill_bytes(&mut packed);
        let weights = rng.f128_vec(KAPPA);
        assert_eq!(
            pack_rows(&packed, &weights, Kernel::Lookup),
            pack_rows(&packed, &weights, Kernel::Reference)
        );
        let basis: Vec<F128> = (0..KAPPA)
            .map(|column| {
                if column < 64 {
                    F128::new(1 << column, 0)
                } else {
                    F128::new(0, 1 << (column - 64))
                }
            })
            .collect();
        let expected: Vec<F128> = packed
            .chunks_exact(ROW_BYTES)
            .map(|row| {
                F128::new(
                    u64::from_le_bytes(row[..8].try_into().unwrap()),
                    u64::from_le_bytes(row[8..].try_into().unwrap()),
                )
            })
            .collect();
        assert_eq!(pack_rows(&packed, &basis, Kernel::Lookup), expected);
        assert!(pack_rows(&[], &weights, Kernel::Lookup).is_empty());
    }

    #[test]
    fn fused_binding_and_projection_match_independent_bit_sums() {
        let mut rng = Rng::new(0xF05ED);
        let mut native_a = Vec::new();
        let mut native_b = Vec::new();
        for rows in [0, 1, 2, 3, 7, 8, 9, 257, 4095, 4096, 4097, 2, 8192, 1] {
            let mut a = vec![0; rows * ROW_BYTES];
            let mut b = a.clone();
            rng.fill_bytes(&mut a);
            rng.fill_bytes(&mut b);
            let columns = rng.f128_vec(KAPPA);
            let mut weights = rng.f128_vec(rows);
            for weight in weights.iter_mut().step_by(7) {
                *weight = F128::ZERO;
            }
            let expected_a = pack_rows(&a, &columns, Kernel::Reference);
            let expected_b: Vec<_> = pack_rows(&b, &columns, Kernel::Reference)
                .into_iter()
                .zip(&weights)
                .map(|(value, &weight)| value * weight)
                .collect();
            let expected_projection = expected_b
                .iter()
                .fold(F128::ZERO, |sum, &value| sum + value);
            for threads in [1, 3] {
                rayon::ThreadPoolBuilder::new()
                    .num_threads(threads)
                    .build()
                    .unwrap()
                    .install(|| {
                        assert_eq!(
                            pack_pair_weighted(&a, &b, &columns, &weights),
                            (expected_a.clone(), expected_b.clone())
                        );
                        assert_eq!(project_rows(&b, &columns, &weights), expected_projection);
                        assert_eq!(
                            project_rows_native(&b, &columns, &weights),
                            expected_projection
                        );
                        let message = pack_pair_weighted_with_message_into(
                            &a,
                            &b,
                            &columns,
                            &weights,
                            &mut native_a,
                            &mut native_b,
                        );
                        assert_eq!(native_a, expected_a);
                        assert_eq!(native_b, expected_b);
                        let mut expected_message = [F128::ZERO; 2];
                        for (a, b) in expected_a.chunks_exact(2).zip(expected_b.chunks_exact(2)) {
                            expected_message[0] += a[0] * b[0];
                            expected_message[1] += (a[0] + a[1]) * (b[0] + b[1]);
                        }
                        assert_eq!(message, expected_message);
                    });
            }
        }
    }

    #[test]
    fn wide_packing_matches_direct_and_original_across_tails_threads_and_reuse() {
        let mut rng = Rng::new(0x8A11_1024_4097);
        let pools: Vec<_> = [1, 4]
            .into_iter()
            .map(|threads| {
                rayon::ThreadPoolBuilder::new()
                    .num_threads(threads)
                    .build()
                    .unwrap()
            })
            .collect();
        let mut wide_a = Vec::with_capacity(8192);
        let mut wide_b = Vec::with_capacity(8192);
        let capacity_a = wide_a.capacity();
        let capacity_b = wide_b.capacity();
        for rows in [0, 1, 2, 7, 8, 9, 1023, 1024, 1025, 4095, 4096, 4097, 2, 0] {
            // Offset the witnesses and weights, rather than relying on the
            // allocator to accidentally exercise unaligned vector loads.
            let mut a_storage = vec![0; rows * ROW_BYTES + 1];
            let mut b_storage = a_storage.clone();
            rng.fill_bytes(&mut a_storage);
            rng.fill_bytes(&mut b_storage);
            let a = &a_storage[1..];
            let b = &b_storage[1..];
            let columns = rng.f128_vec(KAPPA);
            let mut weight_storage = rng.f128_vec(rows + 1);
            let weights = &mut weight_storage[1..];
            for (index, weight) in weights.iter_mut().enumerate() {
                if index % 7 == 0 {
                    *weight = F128::ZERO;
                } else if index % 11 == 0 {
                    *weight = F128::ONE;
                }
            }
            let expected_a = pack_rows(a, &columns, Kernel::Reference);
            let expected_b: Vec<_> = pack_rows(b, &columns, Kernel::Reference)
                .into_iter()
                .zip(weights.iter())
                .map(|(value, &weight)| value * weight)
                .collect();
            let mut expected_message = [F128::ZERO; 2];
            for (a, b) in expected_a.chunks_exact(2).zip(expected_b.chunks_exact(2)) {
                expected_message[0] += a[0] * b[0];
                expected_message[1] += (a[0] + a[1]) * (b[0] + b[1]);
            }
            for pool in &pools {
                pool.install(|| {
                    let mut old_a = Vec::new();
                    let mut old_b = Vec::new();
                    let old_message = pack_pair_weighted_with_message_into(
                        a, b, &columns, weights, &mut old_a, &mut old_b,
                    );
                    let wide_message = pack_pair_weighted_with_wide_message_into(
                        a,
                        b,
                        &columns,
                        weights,
                        &mut wide_a,
                        &mut wide_b,
                    );
                    assert_eq!(wide_a, expected_a, "a, rows={rows}");
                    assert_eq!(wide_b, expected_b, "b, rows={rows}");
                    assert_eq!(wide_message, expected_message, "message, rows={rows}");
                    assert_eq!(wide_a, old_a);
                    assert_eq!(wide_b, old_b);
                    assert_eq!(wide_message, old_message);
                    assert_eq!(wide_a.capacity(), capacity_a);
                    assert_eq!(wide_b.capacity(), capacity_b);
                });
            }
        }
    }

    #[test]
    #[should_panic(expected = "complete rows")]
    fn packing_rejects_partial_rows() {
        pack_rows(&[0; ROW_BYTES - 1], &[F128::ZERO; KAPPA], Kernel::Lookup);
    }
}
