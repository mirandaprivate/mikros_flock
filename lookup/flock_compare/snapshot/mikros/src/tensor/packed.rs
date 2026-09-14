//! Bit-sliced execution of the unchanged mixed-tensor certificate.
//!
//! Each transformed byte contains one bit from each of eight input rows.
//! A 256-entry subset table then folds arbitrary F128 row weights into H.
//! This table depends on the current weights and is built online for every
//! block. No fixed-challenge or subfield assumption is made.
//! The 64-row variant uses u64 masks and eight subset tables, combining their
//! results before writing H. The original eight-row path remains an ablation.

use flock_field::F128;
use flock_multilinear::{IndexOrder, eq_table};
use rayon::prelude::{IndexedParallelIterator, ParallelIterator, ParallelSlice};

use super::{COEFFICIENTS, MIN_ROWS_PER_TASK, ROW_BYTES};

const LANES: usize = 8;
const BLOCK_BYTES: usize = LANES * ROW_BYTES;

pub(super) fn aggregate8(a: &[u8], b: &[u8], weights: &[F128]) -> Vec<F128> {
    let rows_per_task = weights
        .len()
        .div_ceil(rayon::current_num_threads())
        .max(MIN_ROWS_PER_TASK)
        .div_ceil(LANES)
        * LANES;
    a.par_chunks(rows_per_task * ROW_BYTES)
        .zip(b.par_chunks(rows_per_task * ROW_BYTES))
        .zip(weights.par_chunks(rows_per_task))
        .map(|((a, b), weights)| aggregate_chunk(a, b, weights))
        .reduce_with(|mut left, right| {
            for (target, value) in left.iter_mut().zip(right) {
                *target += value;
            }
            left
        })
        .unwrap_or_else(|| vec![F128::ZERO; COEFFICIENTS])
}

const WIDE_LANES: usize = 64;
const WIDE_BLOCK_BYTES: usize = WIDE_LANES * ROW_BYTES;

pub(super) fn aggregate64(a: &[u8], b: &[u8], weights: &[F128]) -> Vec<F128> {
    aggregate64_simd(a, b, weights)
}

/// Original materialized transform, retained for within-build ablations.
pub(super) fn aggregate64_legacy(a: &[u8], b: &[u8], weights: &[F128]) -> Vec<F128> {
    aggregate64_with(a, b, weights, aggregate_chunk64)
}

/// Portable fused expansion and consumption, using arbitrary row weights.
pub(super) fn aggregate64_stream(a: &[u8], b: &[u8], weights: &[F128]) -> Vec<F128> {
    aggregate64_with(a, b, weights, aggregate_chunk64_stream)
}

/// GFNI represents the same subset-XOR linear map, without random table loads.
/// The fallback also works when the binary is built without native CPU flags.
pub(super) fn aggregate64_simd(a: &[u8], b: &[u8], weights: &[F128]) -> Vec<F128> {
    #[cfg(target_arch = "x86_64")]
    if x86_64::available() {
        return aggregate64_with(a, b, weights, x86_64::aggregate_chunk);
    }
    aggregate64_stream(a, b, weights)
}

/// Eight-coefficient GFNI prototype retained to isolate SIMD batching gains.
pub(super) fn aggregate64_simd_v1(a: &[u8], b: &[u8], weights: &[F128]) -> Vec<F128> {
    #[cfg(target_arch = "x86_64")]
    if x86_64::available() {
        return aggregate64_with(a, b, weights, x86_64::aggregate_chunk_v1);
    }
    aggregate64_stream(a, b, weights)
}

/// Factor the unchanged equality weights as L_outer * W_inner. The last
/// `block_log` row coordinates are the maximum inner block size; smaller
/// inputs use enough outer tasks to keep the current Rayon pool occupied.
/// Shared inner matrices are rebuilt from this proof's challenges every call.
pub(super) fn aggregate64_factored(
    a: &[u8],
    b: &[u8],
    row_point: &[F128],
    block_log: usize,
) -> Vec<F128> {
    let rows = 1usize
        .checked_shl(
            row_point
                .len()
                .try_into()
                .expect("too many row coordinates"),
        )
        .expect("too many row coordinates");
    assert_eq!(
        a.len(),
        rows.checked_mul(ROW_BYTES).expect("row table too large")
    );
    assert_eq!(a.len(), b.len());
    #[cfg(target_arch = "x86_64")]
    {
        let outer_log = (2 * rayon::current_num_threads())
            .next_power_of_two()
            .ilog2() as usize;
        let inner_log = block_log.min(row_point.len().saturating_sub(outer_log));
        if inner_log >= 6 && x86_64::available() {
            let split = row_point.len() - inner_log;
            let inner_weights = eq_table(&row_point[split..], F128::ONE, IndexOrder::HighToLow);
            let outer_weights = eq_table(&row_point[..split], F128::ONE, IndexOrder::HighToLow);
            let matrices = x86_64::prepare_factored(&inner_weights);
            let block_bytes = (1usize << inner_log) * ROW_BYTES;
            return a
                .par_chunks(block_bytes)
                .zip(b.par_chunks(block_bytes))
                .zip(outer_weights.par_chunks(1))
                .map(|((a, b), outer)| x86_64::aggregate_factored_chunk(a, b, &matrices, outer[0]))
                .reduce_with(|mut left, right| {
                    for (target, value) in left.iter_mut().zip(right) {
                        *target += value;
                    }
                    left
                })
                .unwrap_or_else(|| vec![F128::ZERO; COEFFICIENTS]);
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    let _ = block_log;
    let weights = eq_table(row_point, F128::ONE, IndexOrder::HighToLow);
    aggregate64_simd(a, b, &weights)
}

#[cfg(target_arch = "x86_64")]
mod x86_64;

fn aggregate64_with(
    a: &[u8],
    b: &[u8],
    weights: &[F128],
    chunk: fn(&[u8], &[u8], &[F128]) -> Vec<F128>,
) -> Vec<F128> {
    let rows_per_task = weights
        .len()
        .div_ceil(rayon::current_num_threads())
        .max(MIN_ROWS_PER_TASK)
        .div_ceil(WIDE_LANES)
        * WIDE_LANES;
    a.par_chunks(rows_per_task * ROW_BYTES)
        .zip(b.par_chunks(rows_per_task * ROW_BYTES))
        .zip(weights.par_chunks(rows_per_task))
        .map(|((a, b), weights)| chunk(a, b, weights))
        .reduce_with(|mut left, right| {
            for (target, value) in left.iter_mut().zip(right) {
                *target += value;
            }
            left
        })
        .unwrap_or_else(|| vec![F128::ZERO; COEFFICIENTS])
}

// Inspected LSB-first delta-swap transpose from flock-core/src/r1cs.rs.
// Its original helper is private; keep the same six-stage portable network.
// After transpose, output[column] bit row equals input[row] bit column.
#[inline]
fn transpose64(matrix: &mut [u64; WIDE_LANES]) {
    let mut distance = 32;
    let mut mask = 0x0000_0000_FFFF_FFFF;
    while distance != 0 {
        let mut index = 0;
        while index < WIDE_LANES {
            let swap = ((matrix[index] >> distance) ^ matrix[index + distance]) & mask;
            matrix[index] ^= swap << distance;
            matrix[index + distance] ^= swap;
            index = (index + distance + 1) & !distance;
        }
        distance >>= 1;
        mask ^= mask << distance;
    }
}

fn transpose_rows64(input: &[u8]) -> [u64; 128] {
    debug_assert!(input.len() <= WIDE_BLOCK_BYTES);
    debug_assert_eq!(input.len() % ROW_BYTES, 0);
    let mut columns = [0u64; 128];
    for (lane, row) in input.chunks_exact(ROW_BYTES).enumerate() {
        columns[lane] = u64::from_le_bytes(row[..8].try_into().unwrap());
        columns[WIDE_LANES + lane] = u64::from_le_bytes(row[8..].try_into().unwrap());
    }
    let (low, high) = columns.split_at_mut(WIDE_LANES);
    transpose64(low.try_into().unwrap());
    transpose64(high.try_into().unwrap());
    columns
}

#[inline(always)]
fn expand_four_words(input: &[u64; 4], output: &mut [u64; 9]) {
    let [a, b, c, d] = *input;
    let ac = a ^ c;
    let bd = b ^ d;
    *output = [a, b, a ^ b, c, d, c ^ d, ac, bd, ac ^ bd];
}

#[inline(always)]
fn expand_eight_words(input: &[u64; 8], output: &mut [u64; 27]) {
    let sum = [
        input[0] ^ input[4],
        input[1] ^ input[5],
        input[2] ^ input[6],
        input[3] ^ input[7],
    ];
    let (low, remainder) = output.split_at_mut(9);
    let (high, summed) = remainder.split_at_mut(9);
    expand_four_words(input[..4].try_into().unwrap(), low.try_into().unwrap());
    expand_four_words(input[4..].try_into().unwrap(), high.try_into().unwrap());
    expand_four_words(&sum, summed.try_into().unwrap());
}

macro_rules! word_tensor_level {
    ($name:ident, $child:ident, $input:expr, $output:expr) => {
        #[inline]
        fn $name(input: &[u64; $input], output: &mut [u64; $output]) {
            const HALF: usize = $input / 2;
            const THIRD: usize = $output / 3;
            let mut sum = [0u64; HALF];
            for index in 0..HALF {
                sum[index] = input[index] ^ input[HALF + index];
            }
            let (low, remainder) = output.split_at_mut(THIRD);
            let (high, summed) = remainder.split_at_mut(THIRD);
            $child(input[..HALF].try_into().unwrap(), low.try_into().unwrap());
            $child(input[HALF..].try_into().unwrap(), high.try_into().unwrap());
            $child(&sum, summed.try_into().unwrap());
        }
    };
}

word_tensor_level!(expand_16_words, expand_eight_words, 16, 81);
word_tensor_level!(expand_32_words, expand_16_words, 32, 243);
word_tensor_level!(expand_64_words, expand_32_words, 64, 729);
word_tensor_level!(expand_128_words, expand_64_words, 128, COEFFICIENTS);

fn transform_block64(input: &[u8], output: &mut [u64; COEFFICIENTS]) {
    expand_128_words(&transpose_rows64(input), output);
}

fn aggregate_chunk64(a: &[u8], b: &[u8], weights: &[F128]) -> Vec<F128> {
    let mut coefficients = vec![F128::ZERO; COEFFICIENTS];
    let mut left = [0u64; COEFFICIENTS];
    let mut right = [0u64; COEFFICIENTS];
    let mut tables = [[F128::ZERO; 256]; WIDE_LANES / LANES];
    for ((a, b), weights) in a
        .chunks(WIDE_BLOCK_BYTES)
        .zip(b.chunks(WIDE_BLOCK_BYTES))
        .zip(weights.chunks(WIDE_LANES))
    {
        transform_block64(a, &mut left);
        transform_block64(b, &mut right);
        for (stripe, table) in tables.iter_mut().enumerate() {
            let start = (stripe * LANES).min(weights.len());
            let end = (start + LANES).min(weights.len());
            subset_table(&weights[start..end], table);
        }
        for ((target, &left), &right) in coefficients.iter_mut().zip(&left).zip(&right) {
            let mask = (left & right).to_le_bytes();
            // Independent table loads, one contiguous H read/write for 64
            // rows. The eight-row baseline writes H eight separate times.
            let first = tables[0][mask[0] as usize] + tables[1][mask[1] as usize];
            let second = tables[2][mask[2] as usize] + tables[3][mask[3] as usize];
            let third = tables[4][mask[4] as usize] + tables[5][mask[5] as usize];
            let fourth = tables[6][mask[6] as usize] + tables[7][mask[7] as usize];
            *target += (first + second) + (third + fourth);
        }
    }
    coefficients
}

/// A leaf emits 27 real masks and five zero masks. The padding lets hardware
/// consumers process four complete groups of eight without scalar tails.
#[inline]
fn stream_eight(
    a: &[u64; 8],
    b: &[u64; 8],
    offset: usize,
    consume: &mut impl FnMut(usize, &[u64; 32]),
) {
    let mut left = [0; 27];
    let mut right = [0; 27];
    expand_eight_words(a, &mut left);
    expand_eight_words(b, &mut right);
    let mut masks = [0; 32];
    for index in 0..27 {
        masks[index] = left[index] & right[index];
    }
    consume(offset, &masks);
}

macro_rules! stream_level {
    ($name:ident, $child:ident, $input:expr, $output:expr) => {
        #[inline]
        fn $name(
            a: &[u64; $input],
            b: &[u64; $input],
            offset: usize,
            consume: &mut impl FnMut(usize, &[u64; 32]),
        ) {
            const HALF: usize = $input / 2;
            const THIRD: usize = $output / 3;
            // Preserve shared tensor recursion: each sum is computed once.
            // Only this recursion stack and a 27-coefficient leaf are live.
            let mut sum_a = [0; HALF];
            let mut sum_b = [0; HALF];
            for index in 0..HALF {
                sum_a[index] = a[index] ^ a[HALF + index];
                sum_b[index] = b[index] ^ b[HALF + index];
            }
            $child(
                a[..HALF].try_into().unwrap(),
                b[..HALF].try_into().unwrap(),
                offset,
                consume,
            );
            $child(
                a[HALF..].try_into().unwrap(),
                b[HALF..].try_into().unwrap(),
                offset + THIRD,
                consume,
            );
            $child(&sum_a, &sum_b, offset + 2 * THIRD, consume);
        }
    };
}

stream_level!(stream_16, stream_eight, 16, 81);
stream_level!(stream_32, stream_16, 32, 243);
stream_level!(stream_64, stream_32, 64, 729);
stream_level!(stream_128, stream_64, 128, COEFFICIENTS);

fn aggregate_chunk64_stream(a: &[u8], b: &[u8], weights: &[F128]) -> Vec<F128> {
    let mut coefficients = vec![F128::ZERO; COEFFICIENTS];
    let mut tables = [[F128::ZERO; 256]; WIDE_LANES / LANES];
    for ((a, b), weights) in a
        .chunks(WIDE_BLOCK_BYTES)
        .zip(b.chunks(WIDE_BLOCK_BYTES))
        .zip(weights.chunks(WIDE_LANES))
    {
        for (stripe, table) in tables.iter_mut().enumerate() {
            let start = (stripe * LANES).min(weights.len());
            subset_table(&weights[start..(start + LANES).min(weights.len())], table);
        }
        stream_128(
            &transpose_rows64(a),
            &transpose_rows64(b),
            0,
            &mut |offset, masks| {
                for (target, &mask) in coefficients[offset..offset + 27].iter_mut().zip(masks) {
                    let mask = mask.to_le_bytes();
                    let first = tables[0][mask[0] as usize] + tables[1][mask[1] as usize];
                    let second = tables[2][mask[2] as usize] + tables[3][mask[3] as usize];
                    let third = tables[4][mask[4] as usize] + tables[5][mask[5] as usize];
                    let fourth = tables[6][mask[6] as usize] + tables[7][mask[7] as usize];
                    *target += (first + second) + (third + fourth);
                }
            },
        );
    }
    coefficients
}

// Same inspected portable SWAR algorithm as flock_core::bits::transpose_8x8_bits.
// That helper is crate-private. Flock's public 64-byte transpose falls back to
// per-bit loops on non-AVX512 x86, so use this small portable primitive locally.
// Input bit 8*row+column becomes output bit 8*column+row.
#[inline(always)]
fn transpose8(mut value: u64) -> u64 {
    let swap = (value ^ (value >> 7)) & 0x00AA_00AA_00AA_00AA;
    value ^= swap ^ (swap << 7);
    let swap = (value ^ (value >> 14)) & 0x0000_CCCC_0000_CCCC;
    value ^= swap ^ (swap << 14);
    let swap = (value ^ (value >> 28)) & 0x0000_0000_F0F0_F0F0;
    value ^ swap ^ (swap << 28)
}

fn transpose_rows(input: &[u8]) -> [u8; 128] {
    debug_assert!(input.len() <= BLOCK_BYTES);
    debug_assert_eq!(input.len() % ROW_BYTES, 0);
    let mut output = [0u8; 128];
    for byte in 0..ROW_BYTES {
        let mut gathered = 0u64;
        for (lane, row) in input.chunks_exact(ROW_BYTES).enumerate() {
            gathered |= (row[byte] as u64) << (8 * lane);
        }
        output[8 * byte..8 * byte + 8].copy_from_slice(&transpose8(gathered).to_le_bytes());
    }
    output
}

// Two bytes a,b become three bytes a,b,a^b. Each byte is an eight-row mask.
#[inline(always)]
fn expand_pair(pair: u32) -> u64 {
    (pair & 0xffff) as u64 | ((((pair ^ (pair >> 8)) & 0xff) as u64) << 16)
}

// T^{tensor 2} on four mask bytes. The output has nine bytes, returned as
// its first eight bytes and the final byte, avoiding nine individual stores.
#[inline(always)]
fn expand_four(input: u32) -> (u64, u8) {
    let low = expand_pair(input);
    let high = expand_pair(input >> 16);
    let sum = expand_pair(input ^ (input >> 16));
    (low | (high << 24) | (sum << 48), (sum >> 16) as u8)
}

// The three low column variables execute with u32/u64 SWAR. No bit-by-bit
// transpose or per-row transform is needed after transpose_rows.
#[inline(always)]
fn expand_eight(input: &[u8; 8], output: &mut [u8; 27]) {
    let input = u64::from_le_bytes(*input);
    let low = input as u32;
    let high = (input >> 32) as u32;
    for (index, (word, tail)) in [expand_four(low), expand_four(high), expand_four(low ^ high)]
        .into_iter()
        .enumerate()
    {
        output[index * 9..index * 9 + 8].copy_from_slice(&word.to_le_bytes());
        output[index * 9 + 8] = tail;
    }
}

// Fixed dimensions allow the optimizer to remove slice checks and vectorize
// the contiguous u64 XORs without retaining a generic recursive hot path.
macro_rules! tensor_level {
    ($name:ident, $child:ident, $input:expr, $output:expr) => {
        #[inline]
        fn $name(input: &[u8; $input], output: &mut [u8; $output]) {
            const HALF: usize = $input / 2;
            const THIRD: usize = $output / 3;
            let mut sum = [0u8; HALF];
            for offset in (0..HALF).step_by(8) {
                let low = u64::from_le_bytes(input[offset..offset + 8].try_into().unwrap());
                let high =
                    u64::from_le_bytes(input[HALF + offset..HALF + offset + 8].try_into().unwrap());
                sum[offset..offset + 8].copy_from_slice(&(low ^ high).to_le_bytes());
            }
            let (low, remainder) = output.split_at_mut(THIRD);
            let (high, summed) = remainder.split_at_mut(THIRD);
            $child(input[..HALF].try_into().unwrap(), low.try_into().unwrap());
            $child(input[HALF..].try_into().unwrap(), high.try_into().unwrap());
            $child(&sum, summed.try_into().unwrap());
        }
    };
}

tensor_level!(expand_16, expand_eight, 16, 81);
tensor_level!(expand_32, expand_16, 32, 243);
tensor_level!(expand_64, expand_32, 64, 729);
tensor_level!(expand_128, expand_64, 128, COEFFICIENTS);

fn transform_block(input: &[u8], output: &mut [u8; COEFFICIENTS]) {
    expand_128(&transpose_rows(input), output);
}

fn subset_table(weights: &[F128], table: &mut [F128; 256]) {
    debug_assert!(weights.len() <= LANES);
    table[0] = F128::ZERO;
    let mut size = 1;
    for lane in 0..LANES {
        let weight = weights.get(lane).copied().unwrap_or(F128::ZERO);
        for mask in 0..size {
            table[size + mask] = table[mask] + weight;
        }
        size *= 2;
    }
}

fn aggregate_chunk(a: &[u8], b: &[u8], weights: &[F128]) -> Vec<F128> {
    let mut coefficients = vec![F128::ZERO; COEFFICIENTS];
    let mut left = [0u8; COEFFICIENTS];
    let mut right = [0u8; COEFFICIENTS];
    let mut table = [F128::ZERO; 256];
    for ((a, b), weights) in a
        .chunks(BLOCK_BYTES)
        .zip(b.chunks(BLOCK_BYTES))
        .zip(weights.chunks(LANES))
    {
        transform_block(a, &mut left);
        transform_block(b, &mut right);
        subset_table(weights, &mut table);
        // Contiguous H updates replace the baseline's per-set-bit scatter.
        for ((target, &left), &right) in coefficients.iter_mut().zip(&left).zip(&right) {
            *target += table[(left & right) as usize];
        }
    }
    coefficients
}

#[cfg(test)]
mod tests {
    use super::{
        BLOCK_BYTES, LANES, WIDE_BLOCK_BYTES, WIDE_LANES, aggregate64_factored, aggregate64_legacy,
        aggregate64_simd, aggregate64_simd_v1, aggregate64_stream, expand_eight, stream_128,
        subset_table, transform_block, transform_block64, transpose_rows, transpose_rows64,
        transpose8, transpose64,
    };
    use crate::tensor::{COEFFICIENTS, KAPPA, Kernel, ROW_BYTES, aggregate, transform_reference};
    use flock_field::{F128, test_rng::Rng};

    fn scalar_transpose(input: u64) -> u64 {
        let mut output = 0;
        for row in 0..8 {
            for column in 0..8 {
                output |= ((input >> (8 * row + column)) & 1) << (8 * column + row);
            }
        }
        output
    }

    #[test]
    fn swar_transpose_matches_every_basis_bit_and_random_matrices() {
        for bit in 0..64 {
            assert_eq!(transpose8(1 << bit), scalar_transpose(1 << bit));
        }
        let mut rng = Rng::new(0x7A4A5A05E);
        for _ in 0..256 {
            let input = rng.next_u64();
            assert_eq!(transpose8(input), scalar_transpose(input));
        }
    }

    #[test]
    fn wide_transpose_maps_every_matrix_basis_bit_and_random_words() {
        for row in 0..WIDE_LANES {
            for column in 0..WIDE_LANES {
                let mut input = [0u64; WIDE_LANES];
                input[row] = 1 << column;
                transpose64(&mut input);
                assert_eq!(input[column], 1 << row);
                assert_eq!(input.iter().filter(|&&word| word != 0).count(), 1);
            }
        }
        let mut rng = Rng::new(0x7A4A5A0564);
        for _ in 0..16 {
            let input: [u64; WIDE_LANES] = std::array::from_fn(|_| rng.next_u64());
            let mut actual = input;
            transpose64(&mut actual);
            for (column, &word) in actual.iter().enumerate() {
                let expected = input
                    .iter()
                    .enumerate()
                    .fold(0, |mask, (row, word)| mask | ((word >> column) & 1) << row);
                assert_eq!(word, expected);
            }
        }
    }

    #[test]
    fn swar_eight_column_expansion_matches_scalar_for_all_row_masks() {
        for mask in 0..256 {
            // Every one of eight rows independently exercises all Boolean
            // input assignments to the eight-column transform.
            let input: [u8; 8] =
                std::array::from_fn(|column| if mask >> column & 1 != 0 { 0xff } else { 0 });
            let scalar: Vec<u8> = input.iter().map(|byte| byte & 1).collect();
            let expected = transform_reference(&scalar);
            let mut actual = [0; 27];
            expand_eight(&input, &mut actual);
            for (actual, expected) in actual.into_iter().zip(expected) {
                assert_eq!(actual, if expected == 1 { 0xff } else { 0 });
            }
        }
        let mut rng = Rng::new(0x5A4A8);
        for _ in 0..32 {
            let mut input = [0; 8];
            rng.fill_bytes(&mut input);
            let mut actual = [0; 27];
            expand_eight(&input, &mut actual);
            for lane in 0..LANES {
                let scalar: Vec<u8> = input.iter().map(|byte| (byte >> lane) & 1).collect();
                for (&actual, expected) in actual.iter().zip(transform_reference(&scalar)) {
                    assert_eq!((actual >> lane) & 1, expected);
                }
            }
        }
    }

    #[test]
    fn packed_transform_keeps_all_columns_and_row_lanes_in_order() {
        for column in 0..KAPPA {
            let mut scalar = vec![0; KAPPA];
            scalar[column] = 1;
            let expected = transform_reference(&scalar);
            for lane in 0..LANES {
                let mut input = [0; BLOCK_BYTES];
                input[lane * ROW_BYTES + column / 8] = 1 << (column % 8);
                let planes = transpose_rows(&input);
                assert_eq!(planes[column], 1 << lane);
                assert_eq!(planes.iter().filter(|&&x| x != 0).count(), 1);
                let mut actual = [0; COEFFICIENTS];
                transform_block(&input, &mut actual);
                for (&actual, &expected) in actual.iter().zip(&expected) {
                    assert_eq!(actual, expected << lane);
                }
            }
        }
    }

    #[test]
    fn wide_transform_preserves_column_halves_and_every_u64_row_lane() {
        for lane in 0..WIDE_LANES {
            for column in [0, 7, 8, 63, 64, 119, 120, 127] {
                let mut input = [0; WIDE_BLOCK_BYTES];
                input[lane * ROW_BYTES + column / 8] = 1 << (column % 8);
                let planes = transpose_rows64(&input);
                assert_eq!(planes[column], 1u64 << lane);
                assert_eq!(planes.iter().filter(|&&word| word != 0).count(), 1);
                let mut scalar = vec![0; KAPPA];
                scalar[column] = 1;
                let expected = transform_reference(&scalar);
                let mut actual = [0; COEFFICIENTS];
                transform_block64(&input, &mut actual);
                for (&actual, &expected) in actual.iter().zip(&expected) {
                    assert_eq!(actual, (expected as u64) << lane);
                }
            }
        }
    }

    #[test]
    fn wide_transform_matches_eight_row_blocks_with_partial_final_lanes() {
        let mut rng = Rng::new(0xA1DE8);
        for rows in [0, 1, 7, 8, 9, 31, 32, 33, 63, 64] {
            let mut input = vec![0; rows * ROW_BYTES];
            rng.fill_bytes(&mut input);
            let mut actual = [0; COEFFICIENTS];
            transform_block64(&input, &mut actual);
            let mut expected = [0u64; COEFFICIENTS];
            for (stripe, chunk) in input.chunks(BLOCK_BYTES).enumerate() {
                let mut byte_masks = [0; COEFFICIENTS];
                transform_block(chunk, &mut byte_masks);
                for (target, byte) in expected.iter_mut().zip(byte_masks) {
                    *target |= (byte as u64) << (stripe * 8);
                }
            }
            assert_eq!(actual, expected, "rows={rows}");
        }
    }

    #[test]
    fn subset_table_uses_arbitrary_field_weights_and_zero_padded_lanes() {
        let mut rng = Rng::new(0x5AB5E7);
        for lanes in 0..=LANES {
            let weights = rng.f128_vec(lanes);
            let mut table = [F128::ONE; 256];
            subset_table(&weights, &mut table);
            for (mask, &actual) in table.iter().enumerate() {
                let expected =
                    weights
                        .iter()
                        .enumerate()
                        .fold(F128::ZERO, |sum, (lane, &weight)| {
                            if mask >> lane & 1 != 0 {
                                sum + weight
                            } else {
                                sum
                            }
                        });
                assert_eq!(actual, expected, "lanes={lanes}, mask={mask}");
            }
        }
    }

    #[test]
    fn packed_aggregation_matches_reference_at_block_and_chunk_boundaries() {
        let mut rng = Rng::new(0xA66A8E6A7E);
        for rows in [0, 1, 7, 8, 9, 63, 64, 65, 255, 256, 257, 511, 512, 513] {
            let mut a = vec![0; rows * ROW_BYTES];
            let mut b = vec![0; rows * ROW_BYTES];
            rng.fill_bytes(&mut a);
            rng.fill_bytes(&mut b);
            let weights = rng.f128_vec(rows);
            let expected = aggregate(&a, &b, &weights, Kernel::Reference);
            assert_eq!(
                aggregate(&a, &b, &weights, Kernel::Packed),
                expected,
                "rows={rows}"
            );
            assert_eq!(aggregate(&a, &b, &weights, Kernel::Packed8), expected);
            assert_eq!(aggregate(&a, &b, &weights, Kernel::Lookup), expected);
            assert_eq!(aggregate64_legacy(&a, &b, &weights), expected);
            assert_eq!(aggregate64_stream(&a, &b, &weights), expected);
            assert_eq!(aggregate64_simd(&a, &b, &weights), expected);
            assert_eq!(aggregate64_simd_v1(&a, &b, &weights), expected);
        }
    }

    #[test]
    fn streamed_masks_preserve_every_column_and_lane_with_zero_padding() {
        for column in 0..KAPPA {
            let mut scalar = vec![0; KAPPA];
            scalar[column] = 1;
            let expected = transform_reference(&scalar);
            for lane in 0..64 {
                let mut planes = [0; KAPPA];
                planes[column] = 1 << lane;
                let mut actual = vec![0; COEFFICIENTS];
                stream_128(&planes, &planes, 0, &mut |offset, masks| {
                    actual[offset..offset + 27].copy_from_slice(&masks[..27]);
                    assert!(masks[27..].iter().all(|&mask| mask == 0));
                });
                for (&actual, &expected) in actual.iter().zip(&expected) {
                    assert_eq!(actual, (expected as u64) << lane);
                }
            }
        }
    }

    #[test]
    fn streamed_masks_match_independent_materialized_random_tensors() {
        let mut rng = Rng::new(0x57_4EA4_ED);
        for rows in [0, 1, 7, 8, 9, 31, 32, 33, 63, 64] {
            let mut a = vec![0; rows * ROW_BYTES];
            let mut b = vec![0; rows * ROW_BYTES];
            rng.fill_bytes(&mut a);
            rng.fill_bytes(&mut b);
            let mut left = [0; COEFFICIENTS];
            let mut right = [0; COEFFICIENTS];
            transform_block64(&a, &mut left);
            transform_block64(&b, &mut right);
            stream_128(
                &transpose_rows64(&a),
                &transpose_rows64(&b),
                0,
                &mut |offset, masks| {
                    for index in 0..27 {
                        assert_eq!(masks[index], left[offset + index] & right[offset + index]);
                    }
                },
            );
        }
    }

    #[test]
    fn factored_equality_weights_match_full_weights_including_boolean_challenges() {
        let mut rng = Rng::new(0xFAC7_04ED);
        let row_log = 11;
        let mut a = vec![0; (1 << row_log) * ROW_BYTES];
        let mut b = a.clone();
        rng.fill_bytes(&mut a);
        rng.fill_bytes(&mut b);
        let random = rng.f128_vec(row_log);
        let mut partly_boolean = random.clone();
        partly_boolean[0] = F128::ZERO;
        partly_boolean[4] = F128::ONE;
        let pools: Vec<_> = [1, 3]
            .into_iter()
            .map(|threads| {
                rayon::ThreadPoolBuilder::new()
                    .num_threads(threads)
                    .build()
                    .unwrap()
            })
            .collect();
        for point in [
            random,
            partly_boolean,
            vec![F128::ZERO; row_log],
            vec![F128::ONE; row_log],
        ] {
            // Independent product for each row checks high-to-low prefix and
            // suffix ordering as well as the zero/one challenge edge cases.
            let weights: Vec<_> = (0..1 << row_log)
                .map(|row| {
                    point
                        .iter()
                        .enumerate()
                        .fold(F128::ONE, |weight, (axis, &r)| {
                            weight
                                * if row >> (row_log - axis - 1) & 1 == 0 {
                                    F128::ONE + r
                                } else {
                                    r
                                }
                        })
                })
                .collect();
            let expected = aggregate64_legacy(&a, &b, &weights);
            for pool in &pools {
                for block_log in [0, 6, 8, 16] {
                    let actual = pool.install(|| aggregate64_factored(&a, &b, &point, block_log));
                    assert_eq!(actual, expected, "block_log={block_log}");
                }
            }
        }
    }

    #[test]
    fn packed_aggregation_is_independent_of_parallel_partition() {
        let mut rng = Rng::new(0xAA4A11E1);
        let pools: Vec<_> = [1, 2, 3]
            .into_iter()
            .map(|threads| {
                rayon::ThreadPoolBuilder::new()
                    .num_threads(threads)
                    .build()
                    .unwrap()
            })
            .collect();
        for rows in [257, 511, 512, 513, 777] {
            let mut a = vec![0; rows * ROW_BYTES];
            let mut b = vec![0; rows * ROW_BYTES];
            rng.fill_bytes(&mut a);
            rng.fill_bytes(&mut b);
            let weights = rng.f128_vec(rows);
            let expected = aggregate(&a, &b, &weights, Kernel::Lookup);
            for pool in &pools {
                for kernel in [Kernel::Packed8, Kernel::Packed] {
                    let actual = pool.install(|| aggregate(&a, &b, &weights, kernel));
                    assert_eq!(actual, expected, "rows={rows}, kernel={kernel:?}");
                }
                assert_eq!(
                    pool.install(|| aggregate64_stream(&a, &b, &weights)),
                    expected
                );
                assert_eq!(
                    pool.install(|| aggregate64_simd(&a, &b, &weights)),
                    expected
                );
            }
        }
    }
}
