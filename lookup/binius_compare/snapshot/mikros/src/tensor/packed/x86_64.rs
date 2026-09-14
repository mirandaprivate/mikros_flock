//! GFNI evaluates arbitrary subset-XOR maps; no small-field weight assumption.
//!
//! The default path turns 64 coefficient masks into eight vectors of 64 stripe
//! bytes. GFNI applies one arbitrary 8-to-8 map per stripe and output field
//! byte; vector XOR combines all eight stripes. The original eight-coefficient
//! prototype is kept for ablation, including its horizontal stripe reduction.
//! Both use 1 KiB of affine matrices instead of 32 KiB subset-XOR tables.

use core::arch::x86_64::{
    __m512i, _mm_cvtsi128_si64, _mm_srli_si128, _mm_xor_si128, _mm256_castsi256_si128,
    _mm256_extracti128_si256, _mm256_xor_si256, _mm512_castsi512_si256, _mm512_extracti64x4_epi64,
    _mm512_gf2p8affine_epi64_epi8, _mm512_loadu_si512, _mm512_permutex2var_epi8,
    _mm512_permutex2var_epi64, _mm512_permutexvar_epi8, _mm512_set1_epi64, _mm512_setzero_si512,
    _mm512_storeu_si512, _mm512_xor_si512,
};

use flock_field::F128;
use rayon::prelude::{ParallelIterator, ParallelSlice};

#[cfg(all(
    target_feature = "avx512f",
    target_feature = "avx512bw",
    target_feature = "vpclmulqdq",
    target_feature = "pclmulqdq",
    target_feature = "sse4.1"
))]
use flock_field::gf2_128::x86_64::{f128x4_set, ghash_mul_x4};

use super::{
    COEFFICIENTS, WIDE_BLOCK_BYTES, WIDE_LANES, expand_32_words, stream_128, transpose_rows64,
    transpose8,
};

const PLANE_STRIDE: usize = COEFFICIENTS.next_multiple_of(64);
type Matrices = [[u64; 8]; 16];

pub(super) struct MatrixCache {
    matrices: Vec<Matrices>,
}

pub(super) fn available() -> bool {
    std::is_x86_feature_detected!("gfni")
        && std::is_x86_feature_detected!("avx512f")
        && std::is_x86_feature_detected!("avx512bw")
        && std::is_x86_feature_detected!("avx512vbmi")
}

const fn byte_transpose_indices() -> [u8; 64] {
    let mut indices = [0; 64];
    let mut index = 0;
    while index < 64 {
        indices[index] = ((index % 8) * 8 + index / 8) as u8;
        index += 1;
    }
    indices
}

const BYTE_TRANSPOSE: [u8; 64] = byte_transpose_indices();

const fn row_gather_indices(high: bool) -> [u8; 64] {
    let mut indices = [0; 64];
    let mut index = 0;
    while index < 64 {
        // GFNI matrix rows are numbered from the highest byte to the lowest.
        indices[index] = ((7 - index % 8) * 16 + index / 8 + if high { 8 } else { 0 }) as u8;
        index += 1;
    }
    indices
}

const GATHER_LOW: [u8; 64] = row_gather_indices(false);
const GATHER_HIGH: [u8; 64] = row_gather_indices(true);

const fn matrix_gather_indices() -> [u8; 64] {
    let mut indices = [0; 64];
    let mut index = 0;
    while index < 64 {
        indices[index] = ((7 - index % 8) * 8 + index / 8) as u8;
        index += 1;
    }
    indices
}

const MATRIX_GATHER: [u8; 64] = matrix_gather_indices();

/// Exchange one row-index bit and column-index bit in an 8x8 qword matrix.
const fn qword_transpose_indices(distance: usize, high: bool) -> [u64; 8] {
    let mut indices = [0; 8];
    let mut index = 0;
    while index < 8 {
        indices[index] = ((index & !distance)
            + if high { distance } else { 0 }
            + if index & distance != 0 { 8 } else { 0 }) as u64;
        index += 1;
    }
    indices
}

const QWORD_TRANSPOSE_LOW: [[u64; 8]; 3] = [
    qword_transpose_indices(1, false),
    qword_transpose_indices(2, false),
    qword_transpose_indices(4, false),
];
const QWORD_TRANSPOSE_HIGH: [[u64; 8]; 3] = [
    qword_transpose_indices(1, true),
    qword_transpose_indices(2, true),
    qword_transpose_indices(4, true),
];

fn affine_matrices(weights: &[F128]) -> Matrices {
    debug_assert!(weights.len() <= WIDE_LANES);
    let mut matrices = [[0; 8]; 16];
    for (byte, output) in matrices.iter_mut().enumerate() {
        for (stripe, matrix) in output.iter_mut().enumerate() {
            let mut rows = 0u64;
            for lane in 0..8 {
                let value = weights
                    .get(stripe * 8 + lane)
                    .copied()
                    .unwrap_or(F128::ZERO);
                let word = if byte < 8 { value.lo } else { value.hi };
                rows |= ((word >> ((byte % 8) * 8)) & 0xff) << (lane * 8);
            }
            // Input-bit columns become output-bit rows. Reverse the matrix
            // row bytes to match Intel GFNI's affine instruction convention.
            *matrix = transpose8(rows).swap_bytes();
        }
    }
    matrices
}

/// # Safety
/// Requires GFNI, AVX-512F/BW/VBMI. Loads cover eight complete 128-byte stripes;
/// partial blocks are handled by the portable transpose before any raw load.
#[target_feature(enable = "gfni,avx512f,avx512bw,avx512vbmi")]
unsafe fn transpose_rows(input: &[u8]) -> [u64; 128] {
    if input.len() != WIDE_BLOCK_BYTES {
        return transpose_rows64(input);
    }
    // SAFETY: target features are declared above; each load/store is within
    // fixed-size arrays or the checked 1024-byte input.
    unsafe {
        let low_index = _mm512_loadu_si512(GATHER_LOW.as_ptr().cast());
        let high_index = _mm512_loadu_si512(GATHER_HIGH.as_ptr().cast());
        let transpose_index = _mm512_loadu_si512(BYTE_TRANSPOSE.as_ptr().cast());
        let basis = _mm512_set1_epi64(0x8040_2010_0804_0201u64 as i64);
        let mut tiles = [[0u64; 8]; 16];
        for stripe in 0..8 {
            let input_low = _mm512_loadu_si512(input.as_ptr().add(stripe * 128).cast());
            let input_high = _mm512_loadu_si512(input.as_ptr().add(stripe * 128 + 64).cast());
            let low = _mm512_gf2p8affine_epi64_epi8::<0>(
                basis,
                _mm512_permutex2var_epi8(input_low, low_index, input_high),
            );
            let high = _mm512_gf2p8affine_epi64_epi8::<0>(
                basis,
                _mm512_permutex2var_epi8(input_low, high_index, input_high),
            );
            let mut low_words = [0u64; 8];
            let mut high_words = [0u64; 8];
            _mm512_storeu_si512(low_words.as_mut_ptr().cast(), low);
            _mm512_storeu_si512(high_words.as_mut_ptr().cast(), high);
            for byte in 0..8 {
                tiles[byte][stripe] = low_words[byte];
                tiles[byte + 8][stripe] = high_words[byte];
            }
        }
        let mut columns = [0; 128];
        for byte in 0..16 {
            let tile = _mm512_loadu_si512(tiles[byte].as_ptr().cast());
            let transposed = _mm512_permutexvar_epi8(transpose_index, tile);
            _mm512_storeu_si512(columns.as_mut_ptr().add(byte * 8).cast(), transposed);
        }
        columns
    }
}

#[inline]
#[target_feature(enable = "avx512f")]
unsafe fn xor_qwords(value: __m512i) -> u64 {
    let half = _mm256_xor_si256(
        _mm512_castsi512_si256(value),
        _mm512_extracti64x4_epi64::<1>(value),
    );
    let quarter = _mm_xor_si128(
        _mm256_castsi256_si128(half),
        _mm256_extracti128_si256::<1>(half),
    );
    _mm_cvtsi128_si64(_mm_xor_si128(quarter, _mm_srli_si128::<8>(quarter))) as u64
}

/// # Safety
/// Requires GFNI and AVX-512F/BW/VBMI. `offset` is a 27-element leaf start;
/// the five zero masks only XOR zero into the next leaf or padded plane tail.
#[inline]
#[target_feature(enable = "gfni,avx512f,avx512bw,avx512vbmi")]
unsafe fn consume(
    planes: &mut [[u8; PLANE_STRIDE]],
    matrices: &Matrices,
    offset: usize,
    masks: &[u64; 32],
) {
    debug_assert_eq!(planes.len(), 16);
    debug_assert!(offset + 32 <= PLANE_STRIDE);
    // SAFETY: four 64-byte mask loads and all 8-byte plane accesses stay in
    // checked fixed-size buffers. Matrix loads each cover exactly eight u64s.
    unsafe {
        let indices = _mm512_loadu_si512(BYTE_TRANSPOSE.as_ptr().cast());
        for group in 0..4 {
            let input = _mm512_loadu_si512(masks.as_ptr().add(group * 8).cast());
            let input = _mm512_permutexvar_epi8(indices, input);
            let start = offset + group * 8;
            for byte in 0..16 {
                let matrix = _mm512_loadu_si512(matrices[byte].as_ptr().cast());
                let contributions = _mm512_gf2p8affine_epi64_epi8::<0>(input, matrix);
                let value = xor_qwords(contributions);
                let old = u64::from_le_bytes(planes[byte][start..start + 8].try_into().unwrap());
                planes[byte][start..start + 8].copy_from_slice(&(old ^ value).to_le_bytes());
            }
        }
    }
}

pub(super) fn aggregate_chunk_v1(a: &[u8], b: &[u8], weights: &[F128]) -> Vec<F128> {
    // SAFETY: the sole production caller dispatches only after available().
    assert!(available());
    unsafe { aggregate_chunk_v1_unchecked(a, b, weights) }
}

#[target_feature(enable = "gfni,avx512f,avx512bw,avx512vbmi")]
unsafe fn aggregate_chunk_v1_unchecked(a: &[u8], b: &[u8], weights: &[F128]) -> Vec<F128> {
    let mut planes = vec![[0u8; PLANE_STRIDE]; 16];
    for ((a, b), weights) in a
        .chunks(WIDE_BLOCK_BYTES)
        .zip(b.chunks(WIDE_BLOCK_BYTES))
        .zip(weights.chunks(WIDE_LANES))
    {
        let matrices = affine_matrices(weights);
        // SAFETY: features are declared by this function, and transpose_rows
        // handles partial blocks. stream_128 emits only valid 27-element leaves.
        unsafe {
            stream_128(
                &transpose_rows(a),
                &transpose_rows(b),
                0,
                &mut |offset, masks| {
                    consume(&mut planes, &matrices, offset, masks);
                },
            );
        }
    }
    (0..COEFFICIENTS)
        .map(|index| {
            let lo = u64::from_le_bytes(std::array::from_fn(|byte| planes[byte][index]));
            let hi = u64::from_le_bytes(std::array::from_fn(|byte| planes[byte + 8][index]));
            F128 { lo, hi }
        })
        .collect()
}

/// # Safety
/// Same hardware requirements as transpose_rows. F128 has the repository's
/// repr(C, align(16)) layout with two little-endian u64 words on x86_64.
#[target_feature(enable = "gfni,avx512f,avx512bw,avx512vbmi")]
unsafe fn affine_matrices_hardware(weights: &[F128]) -> Matrices {
    if weights.len() != 64 {
        return affine_matrices(weights);
    }
    // SAFETY: the length check guarantees 64 initialized 16-byte F128 values;
    // byte aliasing reads are valid, and there is no padding in this layout.
    unsafe {
        let bytes = core::slice::from_raw_parts(weights.as_ptr().cast(), WIDE_BLOCK_BYTES);
        let columns = transpose_rows(bytes);
        let indices = _mm512_loadu_si512(MATRIX_GATHER.as_ptr().cast());
        let mut matrices = [[0; 8]; 16];
        for byte in 0..16 {
            let tile = _mm512_loadu_si512(columns.as_ptr().add(byte * 8).cast());
            _mm512_storeu_si512(
                matrices[byte].as_mut_ptr().cast(),
                _mm512_permutexvar_epi8(indices, tile),
            );
        }
        matrices
    }
}

/// Convert 64 coefficient masks into eight vectors of 64 row-stripe bytes.
/// The byte transpose uses the same VBMI map as Flock; the three qword stages
/// exchange row/column address bits, covering each 8x8 transpose exactly once.
#[inline]
#[target_feature(enable = "avx512f,avx512bw,avx512vbmi")]
unsafe fn mask_byte_planes(masks: &[u64; 64]) -> [__m512i; 8] {
    // SAFETY: every load is one complete fixed-size vector; permutation
    // indices are in 0..16 and only select initialized register elements.
    unsafe {
        let byte_indices = _mm512_loadu_si512(BYTE_TRANSPOSE.as_ptr().cast());
        let mut planes = [_mm512_setzero_si512(); 8];
        for group in 0..8 {
            let input = _mm512_loadu_si512(masks.as_ptr().add(group * 8).cast());
            planes[group] = _mm512_permutexvar_epi8(byte_indices, input);
        }
        for stage in 0..3 {
            let distance = 1 << stage;
            let low_indices = _mm512_loadu_si512(QWORD_TRANSPOSE_LOW[stage].as_ptr().cast());
            let high_indices = _mm512_loadu_si512(QWORD_TRANSPOSE_HIGH[stage].as_ptr().cast());
            for row in 0..8 {
                if row & distance == 0 {
                    let a = planes[row];
                    let b = planes[row + distance];
                    planes[row] = _mm512_permutex2var_epi64(a, low_indices, b);
                    planes[row + distance] = _mm512_permutex2var_epi64(a, high_indices, b);
                }
            }
        }
        planes
    }
}

#[inline]
#[target_feature(enable = "gfni,avx512f,avx512bw,avx512vbmi")]
unsafe fn consume64(
    planes: &mut [[u8; PLANE_STRIDE]],
    matrices: &Matrices,
    offset: usize,
    masks: &[u64; 64],
) {
    debug_assert_eq!(planes.len(), 16);
    debug_assert!(offset + 64 <= PLANE_STRIDE);
    // SAFETY: all input/output accesses are bounded full vectors. Each matrix
    // is broadcast to all 64 inputs; XOR remains binary-field addition.
    unsafe {
        let inputs = mask_byte_planes(masks);
        for byte in 0..16 {
            let mut sums = [_mm512_setzero_si512(); 4];
            for pair in 0..4 {
                let low = _mm512_gf2p8affine_epi64_epi8::<0>(
                    inputs[2 * pair],
                    _mm512_set1_epi64(matrices[byte][2 * pair] as i64),
                );
                let high = _mm512_gf2p8affine_epi64_epi8::<0>(
                    inputs[2 * pair + 1],
                    _mm512_set1_epi64(matrices[byte][2 * pair + 1] as i64),
                );
                sums[pair] = _mm512_xor_si512(low, high);
            }
            let sum = _mm512_xor_si512(
                _mm512_xor_si512(sums[0], sums[1]),
                _mm512_xor_si512(sums[2], sums[3]),
            );
            let target = planes[byte].as_mut_ptr().add(offset).cast();
            let old = _mm512_loadu_si512(target);
            _mm512_storeu_si512(target, _mm512_xor_si512(old, sum));
        }
    }
}

#[target_feature(enable = "gfni,avx512f,avx512bw,avx512vbmi")]
unsafe fn stream_wide_leaf(
    a: &[u64],
    b: &[u64],
    offset: usize,
    span: usize,
    planes: &mut [[u8; PLANE_STRIDE]],
    matrices: &Matrices,
) {
    // SAFETY: the top-level call has exactly 128 columns; recursive halves
    // are 64/32 columns, emitting nine non-overlapping 243-coefficient leaves.
    unsafe {
        if a.len() == 32 {
            let mut left = [0; 243];
            let mut right = [0; 243];
            expand_32_words(a.try_into().unwrap(), &mut left);
            expand_32_words(b.try_into().unwrap(), &mut right);
            let mut masks = [0; 256];
            for index in 0..243 {
                masks[index] = left[index] & right[index];
            }
            for group in 0..4 {
                consume64(
                    planes,
                    matrices,
                    offset + group * 64,
                    masks[group * 64..(group + 1) * 64].try_into().unwrap(),
                );
            }
        } else {
            let half = a.len() / 2;
            let mut sum_a = [0; 64];
            let mut sum_b = [0; 64];
            for index in 0..half {
                sum_a[index] = a[index] ^ a[index + half];
                sum_b[index] = b[index] ^ b[index + half];
            }
            let third = span / 3;
            stream_wide_leaf(&a[..half], &b[..half], offset, third, planes, matrices);
            stream_wide_leaf(
                &a[half..],
                &b[half..],
                offset + third,
                third,
                planes,
                matrices,
            );
            stream_wide_leaf(
                &sum_a[..half],
                &sum_b[..half],
                offset + 2 * third,
                third,
                planes,
                matrices,
            );
        }
    }
}

pub(super) fn aggregate_chunk(a: &[u8], b: &[u8], weights: &[F128]) -> Vec<F128> {
    assert!(available());
    // SAFETY: dispatch checks every target feature, as does the assertion.
    unsafe { aggregate_chunk_unchecked(a, b, weights) }
}

#[target_feature(enable = "gfni,avx512f,avx512bw,avx512vbmi")]
unsafe fn aggregate_chunk_unchecked(a: &[u8], b: &[u8], weights: &[F128]) -> Vec<F128> {
    let mut planes = vec![[0u8; PLANE_STRIDE]; 16];
    for ((a, b), weights) in a
        .chunks(WIDE_BLOCK_BYTES)
        .zip(b.chunks(WIDE_BLOCK_BYTES))
        .zip(weights.chunks(WIDE_LANES))
    {
        // SAFETY: same features as this function; partial blocks have a checked
        // portable transpose/matrix fallback and produce zero padded masks.
        unsafe {
            let matrices = affine_matrices_hardware(weights);
            stream_wide_leaf(
                &transpose_rows(a),
                &transpose_rows(b),
                0,
                COEFFICIENTS,
                &mut planes,
                &matrices,
            );
        }
    }
    (0..COEFFICIENTS)
        .map(|index| F128 {
            lo: u64::from_le_bytes(std::array::from_fn(|byte| planes[byte][index])),
            hi: u64::from_le_bytes(std::array::from_fn(|byte| planes[byte + 8][index])),
        })
        .collect()
}

pub(super) fn prepare_factored(inner_weights: &[F128]) -> MatrixCache {
    assert!(available());
    let matrices = inner_weights
        .par_chunks(WIDE_LANES)
        .map(|weights| {
            // SAFETY: feature dispatch above covers each parallel worker. All
            // workers of a process share the same OS-enabled instruction set.
            unsafe { affine_matrices_hardware(weights) }
        })
        .collect();
    MatrixCache { matrices }
}

pub(super) fn aggregate_factored_chunk(
    a: &[u8],
    b: &[u8],
    matrices: &MatrixCache,
    outer_weight: F128,
) -> Vec<F128> {
    assert!(available());
    // SAFETY: every required instruction feature is checked above.
    unsafe { aggregate_factored_chunk_unchecked(a, b, matrices, outer_weight) }
}

#[target_feature(enable = "gfni,avx512f,avx512bw,avx512vbmi")]
unsafe fn aggregate_factored_chunk_unchecked(
    a: &[u8],
    b: &[u8],
    matrices: &MatrixCache,
    outer_weight: F128,
) -> Vec<F128> {
    assert_eq!(a.len(), b.len());
    assert_eq!(matrices.matrices.len(), a.len().div_ceil(WIDE_BLOCK_BYTES));
    let mut planes = vec![[0u8; PLANE_STRIDE]; 16];
    for ((a, b), matrix) in a
        .chunks(WIDE_BLOCK_BYTES)
        .zip(b.chunks(WIDE_BLOCK_BYTES))
        .zip(&matrices.matrices)
    {
        // SAFETY: validated complete chunk/cache correspondence, and the
        // partial transpose fallback bounds every input load.
        unsafe {
            stream_wide_leaf(
                &transpose_rows(a),
                &transpose_rows(b),
                0,
                COEFFICIENTS,
                &mut planes,
                matrix,
            );
        }
    }
    let mut coefficients: Vec<F128> = (0..COEFFICIENTS)
        .map(|index| F128 {
            lo: u64::from_le_bytes(std::array::from_fn(|byte| planes[byte][index])),
            hi: u64::from_le_bytes(std::array::from_fn(|byte| planes[byte + 8][index])),
        })
        .collect();
    #[cfg(all(
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "pclmulqdq",
        target_feature = "sse4.1"
    ))]
    {
        // SAFETY: these kernels are exported by Flock only in native builds
        // that statically enable AVX-512F/BW, VPCLMULQDQ, PCLMULQDQ and SSE4.1.
        // Each chunk has four F128s.
        unsafe {
            let factor = f128x4_set(outer_weight, outer_weight, outer_weight, outer_weight);
            let mut chunks = coefficients.chunks_exact_mut(4);
            for chunk in &mut chunks {
                let pointer = chunk.as_mut_ptr().cast();
                let values = _mm512_loadu_si512(pointer);
                _mm512_storeu_si512(pointer, ghash_mul_x4(values, factor));
            }
            for value in chunks.into_remainder() {
                *value *= outer_weight;
            }
        }
    }
    #[cfg(not(all(
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "pclmulqdq",
        target_feature = "sse4.1"
    )))]
    for value in &mut coefficients {
        *value *= outer_weight;
    }
    coefficients
}

#[cfg(test)]
mod tests {
    use super::{
        PLANE_STRIDE, affine_matrices, affine_matrices_hardware, available, consume, consume64,
        mask_byte_planes, transpose_rows,
    };
    use crate::tensor::packed::{WIDE_BLOCK_BYTES, transpose_rows64};
    use core::arch::x86_64::_mm512_storeu_si512;
    use flock_field::{F128, test_rng::Rng};

    #[test]
    fn affine_matrix_encoding_matches_every_subset_of_arbitrary_weights() {
        let mut rng = Rng::new(0x6F_41_AFF1);
        for rows in [0, 1, 7, 8, 9, 31, 63, 64] {
            let weights = rng.f128_vec(rows);
            let matrices = affine_matrices(&weights);
            for stripe in 0..8 {
                for mask in 0..256u64 {
                    let expected = (0..8).filter(|&lane| mask >> lane & 1 != 0).fold(
                        F128::ZERO,
                        |sum, lane| {
                            sum + weights
                                .get(stripe * 8 + lane)
                                .copied()
                                .unwrap_or(F128::ZERO)
                        },
                    );
                    for (byte, stripe_matrices) in matrices.iter().enumerate() {
                        let matrix = stripe_matrices[stripe];
                        let actual = (0..8).fold(0u8, |value, bit| {
                            value
                                | ((((matrix >> ((7 - bit) * 8)) & mask).count_ones() as u8 & 1)
                                    << bit)
                        });
                        let expected_word = if byte < 8 { expected.lo } else { expected.hi };
                        assert_eq!(actual, (expected_word >> ((byte % 8) * 8)) as u8);
                    }
                }
            }
        }
    }

    #[test]
    fn hardware_gfni_transpose_matches_all_basis_bits_and_partial_rows() {
        if !available() {
            return;
        }
        let mut input = [0; WIDE_BLOCK_BYTES];
        for bit in 0..WIDE_BLOCK_BYTES * 8 {
            input[bit / 8] = 1 << (bit % 8);
            // SAFETY: this test checks every runtime feature before entry.
            let actual = unsafe { transpose_rows(&input) };
            assert_eq!(actual, transpose_rows64(&input), "bit={bit}");
            input[bit / 8] = 0;
        }
        let mut rng = Rng::new(0x6F_71_7A4A5);
        rng.fill_bytes(&mut input);
        for rows in 0..=64 {
            let input = &input[..rows * 16];
            assert_eq!(
                unsafe { transpose_rows(input) },
                transpose_rows64(input),
                "rows={rows}"
            );
        }
    }

    #[test]
    fn hardware_matrix_builder_and_mask_planes_match_scalar_layouts() {
        if !available() {
            return;
        }
        let mut rng = Rng::new(0x6F_71_647A);
        for rows in 0..=64 {
            let weights = rng.f128_vec(rows);
            // SAFETY: this test checks all required features before entry.
            assert_eq!(
                unsafe { affine_matrices_hardware(&weights) },
                affine_matrices(&weights)
            );
        }
        for _ in 0..16 {
            let masks: [u64; 64] = std::array::from_fn(|_| rng.next_u64());
            let planes = unsafe { mask_byte_planes(&masks) };
            for stripe in 0..8 {
                let mut actual = [0u8; 64];
                // SAFETY: available() includes AVX-512F and output is 64 bytes.
                unsafe { _mm512_storeu_si512(actual.as_mut_ptr().cast(), planes[stripe]) };
                for index in 0..64 {
                    assert_eq!(actual[index], (masks[index] >> (stripe * 8)) as u8);
                }
            }
        }
    }

    #[test]
    fn hardware_wide_consumer_matches_scalar_and_eight_coefficient_consumer() {
        if !available() {
            return;
        }
        let mut rng = Rng::new(0x6F_71_64C0);
        for rows in [0, 1, 7, 8, 9, 31, 63, 64] {
            let weights = rng.f128_vec(rows);
            let matrices = affine_matrices(&weights);
            for stripe in 0..8 {
                for base in (0..256).step_by(64) {
                    let masks: [u64; 64] =
                        std::array::from_fn(|index| ((base + index) as u64) << (stripe * 8));
                    let mut actual = vec![[0xa5; PLANE_STRIDE]; 16];
                    let mut old = actual.clone();
                    // SAFETY: each 64/32-element block is in bounds and runtime
                    // dispatch above checks every instruction feature.
                    unsafe {
                        consume64(&mut actual, &matrices, 1944, &masks);
                        consume(&mut old, &matrices, 1944, masks[..32].try_into().unwrap());
                        consume(&mut old, &matrices, 1976, masks[32..].try_into().unwrap());
                    }
                    assert_eq!(actual, old);
                    for (index, mask) in masks.into_iter().enumerate() {
                        let expected = weights
                            .iter()
                            .enumerate()
                            .filter(|(lane, _)| mask >> lane & 1 != 0)
                            .fold(F128::ZERO, |sum, (_, &weight)| sum + weight);
                        for byte in 0..16 {
                            let word = if byte < 8 { expected.lo } else { expected.hi };
                            assert_eq!(
                                actual[byte][1944 + index],
                                0xa5 ^ (word >> ((byte % 8) * 8)) as u8
                            );
                        }
                    }
                }
            }
            let masks: [u64; 64] = std::array::from_fn(|_| rng.next_u64());
            let mut actual = vec![[0u8; PLANE_STRIDE]; 16];
            let mut old = actual.clone();
            unsafe {
                consume64(&mut actual, &matrices, 0, &masks);
                consume(&mut old, &matrices, 0, masks[..32].try_into().unwrap());
                consume(&mut old, &matrices, 32, masks[32..].try_into().unwrap());
            }
            assert_eq!(actual, old);
        }
    }

    #[test]
    fn hardware_gfni_consumption_matches_all_masks_output_bits_and_padding() {
        if !available() {
            return;
        }
        let mut rng = Rng::new(0x6F_71_C05);
        for rows in [0, 1, 7, 8, 9, 63, 64] {
            let weights = rng.f128_vec(rows);
            let matrices = affine_matrices(&weights);
            for stripe in 0..8 {
                for base in (0..256).step_by(24) {
                    let mut masks = [0; 32];
                    for (index, mask) in masks[..27].iter_mut().enumerate() {
                        *mask = (((base + index) % 256) as u64) << (stripe * 8);
                    }
                    let mut planes = vec![[0xa5; PLANE_STRIDE]; 16];
                    // SAFETY: hardware feature check above, valid padded leaf.
                    unsafe { consume(&mut planes, &matrices, 2160, &masks) };
                    for (index, mask) in masks.into_iter().enumerate() {
                        let expected = weights
                            .iter()
                            .enumerate()
                            .filter(|(lane, _)| mask >> lane & 1 != 0)
                            .fold(F128::ZERO, |sum, (_, &weight)| sum + weight);
                        for byte in 0..16 {
                            let word = if byte < 8 { expected.lo } else { expected.hi };
                            assert_eq!(
                                planes[byte][2160 + index],
                                0xa5 ^ (word >> ((byte % 8) * 8)) as u8
                            );
                        }
                    }
                    assert!(
                        planes
                            .iter()
                            .all(|plane| plane[..2160].iter().all(|&byte| byte == 0xa5))
                    );
                }
            }
        }
    }
}
