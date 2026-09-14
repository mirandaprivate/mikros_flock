//! Intermediate-field arithmetic for `latex/4-block.tex`.
//!
//! `K = GF(256)` uses Flock's AES representation and its `PHI_8_TABLE`
//! embedding into the GHASH field `E = GF(2^128)`. An element of `E` is
//! represented over `K` in the basis `eta_j = x^j`, for `0 <= j < 16`.
//! These coordinates are **not** the sixteen bytes of a GHASH element.
//!
//! Initialization checks that the 128 binary vectors
//! `eta_j * embed(1 << t)` form a basis. Gaussian elimination constructs the
//! inverse change of basis; byte lookup tables then perform both conversions
//! using sixteen lookups and XORs. Together with the actual 256-by-256
//! multiplication table, these reusable tables occupy about 192 KiB.

use std::sync::OnceLock;

use flock_field::{F8, F128, PHI_8_TABLE};

const COORDINATES: usize = 16;
const BINARY_DIMENSION: usize = 128;
const FIELD_SIZE: usize = 256;

type ByteTables = Box<[[u128; FIELD_SIZE]]>;

/// Shared arithmetic and basis maps for the intermediate field `GF(256)`.
pub(crate) struct SmallField {
    multiplication: Box<[u8]>,
    inverse: [u8; FIELD_SIZE],
    to_coordinates: ByteTables,
    to_ghash: ByteTables,
    #[cfg(target_arch = "x86_64")]
    coordinate_matrices: [[u64; COORDINATES]; COORDINATES],
}

/// Returns the process-wide, lazily initialized intermediate-field tables.
pub(crate) fn field() -> &'static SmallField {
    static FIELD: OnceLock<SmallField> = OnceLock::new();
    FIELD.get_or_init(SmallField::new)
}

impl SmallField {
    fn new() -> Self {
        let mut multiplication = vec![0; FIELD_SIZE * FIELD_SIZE].into_boxed_slice();
        let mut inverse = [0; FIELD_SIZE];
        for a in 0..FIELD_SIZE {
            inverse[a] = F8(a as u8).inv().0;
            for b in 0..FIELD_SIZE {
                multiplication[a * FIELD_SIZE + b] = (F8(a as u8) * F8(b as u8)).0;
            }
        }

        // Column 8*j+t maps bit t of K-coordinate j to GHASH binary bits.
        // Powers x^j with j < 16 need no reduction in the GHASH polynomial.
        let mut columns = [0u128; BINARY_DIMENSION];
        for j in 0..COORDINATES {
            let eta = F128::new(1u64 << j, 0);
            for t in 0..8 {
                columns[8 * j + t] = ghash_bits(eta * PHI_8_TABLE[1 << t]);
            }
        }
        let inverse_columns = invert_binary_basis(&columns);
        let to_coordinates = byte_tables(&inverse_columns);

        Self {
            multiplication,
            inverse,
            #[cfg(target_arch = "x86_64")]
            coordinate_matrices: coordinate_matrices(&to_coordinates),
            to_coordinates,
            to_ghash: byte_tables(&columns),
        }
    }

    /// Multiplies two AES-representation `GF(256)` elements by table lookup.
    #[inline]
    pub(crate) fn mul(&self, a: u8, b: u8) -> u8 {
        self.multiplication[usize::from(a) * FIELD_SIZE + usize::from(b)]
    }

    /// Returns the multiplicative inverse of a nonzero element.
    ///
    /// As in `F8::inv`, zero maps to zero. Callers performing division must
    /// independently ensure their denominator is nonzero.
    #[inline]
    pub(crate) fn inv(&self, a: u8) -> u8 {
        self.inverse[usize::from(a)]
    }

    /// Embeds an AES-representation `GF(256)` element into GHASH.
    #[inline]
    pub(crate) fn embed(&self, a: u8) -> F128 {
        PHI_8_TABLE[usize::from(a)]
    }

    /// Finds the unique coordinates `a_j` such that `value = sum_j embed(a_j) x^j`.
    #[inline]
    pub(crate) fn decompose(&self, value: F128) -> [u8; COORDINATES] {
        apply_byte_tables(&self.to_coordinates, &ghash_bits(value).to_le_bytes()).to_le_bytes()
    }

    /// Converts many GHASH values to the same K coordinates as [`Self::decompose`].
    /// Full batches of 64 use fixed GFNI affine maps when available; incomplete
    /// batches and other machines retain the scalar lookup implementation.
    pub(crate) fn decompose_block(&self, input: &[F128], output: &mut [[u8; COORDINATES]]) {
        assert_eq!(
            input.len(),
            output.len(),
            "coordinate output length mismatch"
        );
        #[cfg(target_arch = "x86_64")]
        let processed = if native_coordinates_available() {
            for (values, coordinates) in input.chunks_exact(64).zip(output.chunks_exact_mut(64)) {
                // SAFETY: runtime checks cover every required feature, both
                // chunks contain exactly 64 rows, and output is exclusively borrowed.
                unsafe {
                    native_coordinates::decompose64(&self.coordinate_matrices, values, coordinates)
                };
            }
            input.len() / 64 * 64
        } else {
            0
        };
        #[cfg(not(target_arch = "x86_64"))]
        let processed = 0;
        for (&value, coordinates) in input[processed..].iter().zip(&mut output[processed..]) {
            *coordinates = self.decompose(value);
        }
    }

    /// Candidate with the same mapping and batch boundaries as
    /// [`Self::decompose_block`], constructing converted planes directly instead
    /// of zero-initializing their complete array. The original batch path is
    /// retained for measurement of this initialization difference.
    pub(crate) fn decompose_block_register(
        &self,
        input: &[F128],
        output: &mut [[u8; COORDINATES]],
    ) {
        assert_eq!(
            input.len(),
            output.len(),
            "coordinate output length mismatch"
        );
        #[cfg(target_arch = "x86_64")]
        let processed = if native_coordinates_available() {
            for (values, coordinates) in input.chunks_exact(64).zip(output.chunks_exact_mut(64)) {
                // SAFETY: the dispatch checks GFNI and AVX512F/BW. Both
                // chunks contain exactly 64 rows; output is exclusively
                // borrowed and all vector stores remain inside this chunk.
                unsafe {
                    native_coordinates::decompose64_register(
                        &self.coordinate_matrices,
                        values,
                        coordinates,
                    )
                };
            }
            input.len() / 64 * 64
        } else {
            0
        };
        #[cfg(not(target_arch = "x86_64"))]
        let processed = 0;
        for (&value, coordinates) in input[processed..].iter().zip(&mut output[processed..]) {
            *coordinates = self.decompose(value);
        }
    }

    /// Computes `sum_j embed(coordinates[j]) x^j` in the GHASH field.
    #[inline]
    pub(crate) fn compose(&self, coordinates: &[u8; COORDINATES]) -> F128 {
        from_ghash_bits(apply_byte_tables(&self.to_ghash, coordinates))
    }
}

#[cfg(target_arch = "x86_64")]
fn native_coordinates_available() -> bool {
    std::is_x86_feature_detected!("gfni")
        && std::is_x86_feature_detected!("avx512f")
        && std::is_x86_feature_detected!("avx512bw")
}

/// Each block maps one source byte to one output coordinate byte. GFNI encodes
/// output bit r's input mask in byte 7-r, rather than byte r, of the matrix.
#[cfg(target_arch = "x86_64")]
fn coordinate_matrices(tables: &[[u128; FIELD_SIZE]]) -> [[u64; COORDINATES]; COORDINATES] {
    std::array::from_fn(|output| {
        std::array::from_fn(|input| {
            let mut matrix = 0;
            for bit in 0..8 {
                let image = (tables[input][1 << bit] >> (8 * output)) as u8;
                for output_bit in 0..8 {
                    matrix |= u64::from((image >> output_bit) & 1) << (8 * (7 - output_bit) + bit);
                }
            }
            matrix
        })
    })
}

#[cfg(target_arch = "x86_64")]
mod native_coordinates {
    use std::arch::x86_64::{
        __m512i, _mm512_gf2p8affine_epi64_epi8, _mm512_loadu_si512, _mm512_set1_epi64,
        _mm512_setzero_si512, _mm512_storeu_si512, _mm512_unpackhi_epi8, _mm512_unpackhi_epi16,
        _mm512_unpackhi_epi32, _mm512_unpackhi_epi64, _mm512_unpacklo_epi8, _mm512_unpacklo_epi16,
        _mm512_unpacklo_epi32, _mm512_unpacklo_epi64, _mm512_xor_si512,
    };

    use super::{COORDINATES, F128};

    // Independently transpose each of the four 128-bit lanes. The byte planes
    // enumerate rows [0,4,...,60,1,5,...,61,2,...,62,3,...,63]. Every map uses
    // that same order; transposing back recovers consecutive 16-byte rows.
    #[inline]
    #[target_feature(enable = "avx512f,avx512bw")]
    unsafe fn transpose(mut rows: [__m512i; COORDINATES]) -> [__m512i; COORDINATES] {
        let mut next = [_mm512_setzero_si512(); COORDINATES];
        for i in (0..COORDINATES).step_by(2) {
            next[i] = _mm512_unpacklo_epi8(rows[i], rows[i + 1]);
            next[i + 1] = _mm512_unpackhi_epi8(rows[i], rows[i + 1]);
        }
        rows = next;
        for group in (0..COORDINATES).step_by(4) {
            for i in 0..2 {
                next[group + 2 * i] = _mm512_unpacklo_epi16(rows[group + i], rows[group + 2 + i]);
                next[group + 2 * i + 1] =
                    _mm512_unpackhi_epi16(rows[group + i], rows[group + 2 + i]);
            }
        }
        rows = next;
        for group in (0..COORDINATES).step_by(8) {
            for i in 0..4 {
                next[group + 2 * i] = _mm512_unpacklo_epi32(rows[group + i], rows[group + 4 + i]);
                next[group + 2 * i + 1] =
                    _mm512_unpackhi_epi32(rows[group + i], rows[group + 4 + i]);
            }
        }
        rows = next;
        for i in 0..8 {
            next[2 * i] = _mm512_unpacklo_epi64(rows[i], rows[8 + i]);
            next[2 * i + 1] = _mm512_unpackhi_epi64(rows[i], rows[8 + i]);
        }
        next
    }

    #[target_feature(enable = "gfni,avx512f,avx512bw")]
    pub(super) unsafe fn decompose64(
        matrices: &[[u64; COORDINATES]; COORDINATES],
        input: &[F128],
        output: &mut [[u8; COORDINATES]],
    ) {
        debug_assert_eq!(input.len(), 64);
        debug_assert_eq!(output.len(), 64);
        // SAFETY: 64 F128 inputs and 64 byte-array outputs are each 1024
        // initialized bytes. The sixteen unaligned accesses cover exactly
        // those buffers. GFNI uses only the checked function target features.
        unsafe {
            let rows =
                std::array::from_fn(|i| _mm512_loadu_si512(input.as_ptr().add(4 * i).cast()));
            let planes = transpose(rows);
            let mut converted = [_mm512_setzero_si512(); COORDINATES];
            for (result, matrix_row) in converted.iter_mut().zip(matrices) {
                let mut sum = _mm512_setzero_si512();
                for (&plane, &matrix) in planes.iter().zip(matrix_row) {
                    if matrix != 0 {
                        let mapped = _mm512_gf2p8affine_epi64_epi8::<0>(
                            plane,
                            _mm512_set1_epi64(matrix as i64),
                        );
                        sum = _mm512_xor_si512(sum, mapped);
                    }
                }
                *result = sum;
            }
            let rows = transpose(converted);
            for (i, row) in rows.into_iter().enumerate() {
                _mm512_storeu_si512(output.as_mut_ptr().add(4 * i).cast(), row);
            }
        }
    }

    /// # Safety
    /// Requires GFNI and AVX512F/BW, exactly 64 initialized F128 input rows,
    /// and an exclusively borrowed output slice containing exactly 64 rows.
    #[target_feature(enable = "gfni,avx512f,avx512bw")]
    pub(super) unsafe fn decompose64_register(
        matrices: &[[u64; COORDINATES]; COORDINATES],
        input: &[F128],
        output: &mut [[u8; COORDINATES]],
    ) {
        debug_assert_eq!(input.len(), 64);
        debug_assert_eq!(output.len(), 64);
        // SAFETY: each buffer contains 1024 initialized bytes; the sixteen
        // unaligned vector accesses cover exactly those buffers. from_fn
        // constructs all sixteen converted planes before transpose reads
        // them. Each accumulator starts at zero, including a hypothetical
        // all-zero matrix row, so no uninitialized value is ever observed.
        unsafe {
            let rows =
                std::array::from_fn(|i| _mm512_loadu_si512(input.as_ptr().add(4 * i).cast()));
            let planes = transpose(rows);
            let converted: [__m512i; COORDINATES] = std::array::from_fn(|coordinate| {
                let mut sum = _mm512_setzero_si512();
                for (&plane, &matrix) in planes.iter().zip(&matrices[coordinate]) {
                    if matrix != 0 {
                        let mapped = _mm512_gf2p8affine_epi64_epi8::<0>(
                            plane,
                            _mm512_set1_epi64(matrix as i64),
                        );
                        sum = _mm512_xor_si512(sum, mapped);
                    }
                }
                sum
            });
            let rows = transpose(converted);
            for (i, row) in rows.into_iter().enumerate() {
                _mm512_storeu_si512(output.as_mut_ptr().add(4 * i).cast(), row);
            }
        }
    }
}

#[inline]
fn ghash_bits(value: F128) -> u128 {
    u128::from(value.lo) | (u128::from(value.hi) << 64)
}

#[inline]
fn from_ghash_bits(value: u128) -> F128 {
    F128::new(value as u64, (value >> 64) as u64)
}

/// Inverts a binary matrix specified by columns, returning inverse columns.
fn invert_binary_basis(columns: &[u128; BINARY_DIMENSION]) -> [u128; BINARY_DIMENSION] {
    let mut rows = [0u128; BINARY_DIMENSION];
    let mut inverse_rows = [0u128; BINARY_DIMENSION];
    for r in 0..BINARY_DIMENSION {
        inverse_rows[r] = 1u128 << r;
        for (c, column) in columns.iter().enumerate() {
            rows[r] |= ((column >> r) & 1) << c;
        }
    }

    // Reduce [M | I] to [I | M^-1], checking full rank even in release builds.
    for c in 0..BINARY_DIMENSION {
        let pivot = (c..BINARY_DIMENSION)
            .find(|&r| (rows[r] >> c) & 1 != 0)
            .expect("GHASH powers x^0..x^15 must form a full-rank GF(256) basis");
        rows.swap(c, pivot);
        inverse_rows.swap(c, pivot);
        let pivot_row = rows[c];
        let pivot_inverse = inverse_rows[c];
        for r in 0..BINARY_DIMENSION {
            if r != c && (rows[r] >> c) & 1 != 0 {
                rows[r] ^= pivot_row;
                inverse_rows[r] ^= pivot_inverse;
            }
        }
    }

    let mut inverse_columns = [0u128; BINARY_DIMENSION];
    for (c, column) in inverse_columns.iter_mut().enumerate() {
        for (r, row) in inverse_rows.iter().enumerate() {
            *column |= ((row >> c) & 1) << r;
        }
    }
    inverse_columns
}

fn byte_tables(columns: &[u128; BINARY_DIMENSION]) -> ByteTables {
    // Allocate each 64 KiB map on the heap, avoiding a large stack temporary.
    let mut tables = vec![[0u128; FIELD_SIZE]; COORDINATES].into_boxed_slice();
    for j in 0..COORDINATES {
        for byte in 1usize..FIELD_SIZE {
            let bit = byte.trailing_zeros() as usize;
            tables[j][byte] = tables[j][byte & (byte - 1)] ^ columns[8 * j + bit];
        }
    }
    tables
}

#[inline]
fn apply_byte_tables(tables: &[[u128; FIELD_SIZE]], input: &[u8; COORDINATES]) -> u128 {
    let mut output = 0;
    for j in 0..COORDINATES {
        output ^= tables[j][usize::from(input[j])];
    }
    output
}

#[cfg(test)]
mod tests {
    use flock_field::{F8, F128, test_rng::Rng};

    use super::{COORDINATES, field, from_ghash_bits};

    fn compose_direct(coordinates: &[u8; COORDINATES]) -> F128 {
        coordinates
            .iter()
            .enumerate()
            .fold(F128::ZERO, |sum, (j, &coordinate)| {
                sum + field().embed(coordinate) * F128::new(1u64 << j, 0)
            })
    }

    #[test]
    fn small_field_all_products_match_f8() {
        let k = field();
        assert_eq!(k.multiplication.len(), 65536);
        for a in 0u16..256 {
            for b in 0u16..256 {
                assert_eq!(k.mul(a as u8, b as u8), (F8(a as u8) * F8(b as u8)).0);
            }
        }
        assert_eq!(k.inv(0), 0);
        for a in 1u16..256 {
            assert_eq!(k.inv(a as u8), F8(a as u8).inv().0);
            assert_eq!(k.mul(a as u8, k.inv(a as u8)), 1);
        }
        assert!(std::ptr::eq(k, field()));
    }

    #[test]
    fn small_field_binary_bases_roundtrip() {
        let k = field();
        assert_eq!(k.decompose(F128::ZERO), [0; COORDINATES]);
        assert_eq!(k.compose(&[0; COORDINATES]), F128::ZERO);
        for bit in 0..128 {
            let value = from_ghash_bits(1u128 << bit);
            let coordinates = k.decompose(value);
            assert_eq!(k.compose(&coordinates), value, "GHASH basis bit {bit}");
            assert_eq!(compose_direct(&coordinates), value, "GHASH basis bit {bit}");

            let mut coordinates = [0; COORDINATES];
            coordinates[bit / 8] = 1 << (bit % 8);
            let value = k.compose(&coordinates);
            assert_eq!(value, compose_direct(&coordinates), "K basis bit {bit}");
            assert_eq!(k.decompose(value), coordinates, "K basis bit {bit}");
        }
    }

    #[test]
    fn small_field_coordinates_use_embedding_not_raw_ghash_bytes() {
        let k = field();
        for a in 0u16..256 {
            let mut coordinates = [0; COORDINATES];
            coordinates[0] = a as u8;
            assert_eq!(k.decompose(k.embed(a as u8)), coordinates);
            assert_eq!(k.compose(&coordinates), k.embed(a as u8));
        }
        let mut x_coordinates = [0; COORDINATES];
        x_coordinates[1] = 1;
        assert_eq!(k.decompose(F128::new(2, 0)), x_coordinates);
        assert_ne!(k.embed(2), F128::new(2, 0));
    }

    #[test]
    fn small_field_random_roundtrip() {
        let k = field();
        let mut rng = Rng::new(0x5a17_8fbc_23e9_6140);
        for _ in 0..256 {
            let value = rng.f128();
            let coordinates = k.decompose(value);
            assert_eq!(k.compose(&coordinates), value);
            assert_eq!(compose_direct(&coordinates), value);

            let mut coordinates = [0; COORDINATES];
            rng.fill_bytes(&mut coordinates);
            let value = k.compose(&coordinates);
            assert_eq!(k.decompose(value), coordinates);
            assert_eq!(value, compose_direct(&coordinates));
        }
    }

    #[test]
    fn small_field_coordinate_scaling_matches_extension_multiplication() {
        let k = field();
        let mut rng = Rng::new(0x29f8_c1a4_708e_365b);
        for _ in 0..32 {
            let mut coordinates = [0; COORDINATES];
            rng.fill_bytes(&mut coordinates);
            let value = k.compose(&coordinates);
            for scalar in 0u16..256 {
                let scalar = scalar as u8;
                let scaled = coordinates.map(|coordinate| k.mul(coordinate, scalar));
                assert_eq!(k.compose(&scaled), k.embed(scalar) * value);
                assert_eq!(k.decompose(k.embed(scalar) * value), scaled);
            }
        }
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn coordinate_affine_matrices_match_every_binary_basis_vector() {
        let k = field();
        for bit in 0..128 {
            let source_byte = bit / 8;
            let source_value = 1u8 << (bit % 8);
            let mut actual = [0; COORDINATES];
            for (output, row) in actual.iter_mut().zip(&k.coordinate_matrices) {
                let matrix = row[source_byte];
                for output_bit in 0..8 {
                    let mask = (matrix >> (8 * (7 - output_bit))) as u8;
                    *output |= (((mask & source_value).count_ones() & 1) as u8) << output_bit;
                }
            }
            assert_eq!(
                actual,
                k.decompose(from_ghash_bits(1u128 << bit)),
                "bit {bit}"
            );
        }
    }

    #[test]
    fn coordinate_batch_matches_both_binary_bases() {
        let k = field();
        let mut input = Vec::with_capacity(256);
        for bit in 0..128 {
            input.push(from_ghash_bits(1u128 << bit));
        }
        for bit in 0..128 {
            let mut coordinates = [0; COORDINATES];
            coordinates[bit / 8] = 1 << (bit % 8);
            input.push(compose_direct(&coordinates));
        }
        let mut actual = vec![[0; COORDINATES]; input.len()];
        k.decompose_block(&input, &mut actual);
        for (i, (&value, coordinates)) in input.iter().zip(actual).enumerate() {
            assert_eq!(coordinates, k.decompose(value), "basis index {i}");
            assert_eq!(compose_direct(&coordinates), value, "basis index {i}");
        }
    }

    #[test]
    fn coordinate_batch_matches_random_rows_tails_and_unaligned_buffers() {
        let k = field();
        let mut rng = Rng::new(0x64ba_7c48_a551_8e16);
        for length in [
            0, 1, 7, 8, 15, 16, 31, 32, 63, 64, 65, 127, 128, 129, 255, 256, 257, 1024,
        ] {
            let mut input = rng.f128_vec(length + 2);
            if length >= 3 {
                input[1] = F128::ZERO;
                input[2] = F128::ONE;
                input[3] = from_ghash_bits(u128::MAX);
            }
            let mut output = vec![[0xA5; COORDINATES]; length + 2];
            // Start the native input at a 16-byte offset and the output at a
            // 16-byte offset; neither pointer needs 64-byte alignment.
            k.decompose_block(&input[1..1 + length], &mut output[1..1 + length]);
            assert_eq!(output[0], [0xA5; COORDINATES]);
            assert_eq!(output[length + 1], [0xA5; COORDINATES]);
            for (i, (&value, actual)) in input[1..1 + length]
                .iter()
                .zip(&output[1..1 + length])
                .enumerate()
            {
                assert_eq!(*actual, k.decompose(value), "length {length}, row {i}");
            }
        }
    }

    #[test]
    #[should_panic(expected = "coordinate output length mismatch")]
    fn coordinate_batch_rejects_mismatched_output_length() {
        field().decompose_block(&[F128::ZERO], &mut []);
    }

    #[test]
    fn register_coordinate_batch_matches_scalar_and_original_batch() {
        let k = field();
        let check = |input: &[F128]| {
            // Guard both ends and use a sixteen-byte offset: the native
            // routine may not assume that either slice is ZMM-aligned.
            let mut actual = vec![[0xA5; COORDINATES]; input.len() + 2];
            let mut baseline = vec![[0; COORDINATES]; input.len()];
            k.decompose_block_register(input, &mut actual[1..1 + input.len()]);
            k.decompose_block(input, &mut baseline);
            assert_eq!(actual[0], [0xA5; COORDINATES]);
            assert_eq!(actual[input.len() + 1], [0xA5; COORDINATES]);
            assert_eq!(&actual[1..1 + input.len()], baseline.as_slice());
            for (&value, coordinates) in input.iter().zip(&baseline) {
                assert_eq!(*coordinates, k.decompose(value));
                assert_eq!(compose_direct(coordinates), value);
            }
        };
        let mut basis = Vec::with_capacity(256);
        for bit in 0..128 {
            basis.push(from_ghash_bits(1u128 << bit));
        }
        for bit in 0..128 {
            let mut coordinates = [0; COORDINATES];
            coordinates[bit / 8] = 1 << (bit % 8);
            basis.push(compose_direct(&coordinates));
        }
        check(&basis);
        let mut rng = Rng::new(0x6410_2716_D150_10F0);
        for length in [
            0, 1, 15, 31, 63, 64, 65, 127, 128, 129, 1024, 2047, 2048, 2049,
        ] {
            let mut input = rng.f128_vec(length + 2);
            if length >= 3 {
                input[1] = F128::ZERO;
                input[2] = F128::ONE;
                input[3] = from_ghash_bits(u128::MAX);
            }
            check(&input[1..1 + length]);
        }
    }

    #[test]
    #[should_panic(expected = "coordinate output length mismatch")]
    fn register_coordinate_batch_rejects_mismatched_output_length() {
        field().decompose_block_register(&[F128::ZERO], &mut []);
    }
}
