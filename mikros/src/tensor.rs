//! The binary mixed-tensor certificate from `latex/mikros/appx-tech.tex`.
//!
//! Input bit `j` is column `j` of a 128-bit row. Tensor coordinates are
//! lexicographic: the first coordinate is the most significant binary or
//! ternary digit. We retain the basis `(1-X, X, X^2-X)` throughout.

use flock_field::F128;
use rayon::prelude::{IndexedParallelIterator, ParallelIterator, ParallelSlice};

use crate::lookup::BYTE_TRANSFORM;

mod packed;

pub const KAPPA: usize = 128;
pub const S: usize = 7;
pub const COEFFICIENTS: usize = 2187;
pub const QUOTIENTS: usize = 2059;

const ROW_BYTES: usize = KAPPA / 8;
const INNER_COEFFICIENTS: usize = 27;
const OUTER_COEFFICIENTS: usize = 81;
const MIN_ROWS_PER_TASK: usize = 256;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kernel {
    Reference,
    Lookup,
    /// Sixty-four-row bit-sliced transform and online subset-XOR aggregation.
    Packed,
    /// Eight-row predecessor retained for within-executable ablation.
    Packed8,
    /// Original 64-row implementation retained for protocol-identical ablation.
    PackedLegacy,
    /// Streamed tensor expansion with portable subset tables.
    PackedStream,
    /// Native tensor aggregation; legacy packing and IP tail.
    PackedSimd,
    /// First GFNI implementation with eight-coefficient horizontal reductions.
    PackedSimdV1,
    /// Native tensor and wide IP folding, without packing/message fusion.
    PackedWide,
    /// Native tensor, packing/message fusion and projection, old row setup.
    PackedFused,
    /// Native aggregation, packing/initial message, and IP tail.
    PackedNative,
    /// Share inner GFNI matrices using the unchanged equality-point factors.
    PackedFactored,
}

impl Kernel {
    pub const fn is_packed(self) -> bool {
        !matches!(self, Self::Reference | Self::Lookup)
    }

    pub const fn is_native(self) -> bool {
        matches!(
            self,
            Self::Packed | Self::PackedNative | Self::PackedFactored
        )
    }

    pub const fn has_native_ip(self) -> bool {
        matches!(
            self,
            Self::Packed
                | Self::PackedNative
                | Self::PackedFactored
                | Self::PackedWide
                | Self::PackedFused
        )
    }

    pub const fn has_native_packing(self) -> bool {
        matches!(
            self,
            Self::Packed | Self::PackedNative | Self::PackedFactored | Self::PackedFused
        )
    }
}

/// Return the Boolean column represented by a ternary coefficient index.
/// Indices containing a `2` belong to a quotient, not a column claim.
pub fn boolean_column(mut ternary_index: usize) -> Option<usize> {
    if ternary_index >= COEFFICIENTS {
        return None;
    }
    let mut column = 0;
    for axis in 0..S {
        let digit = ternary_index % 3;
        if digit == 2 {
            return None;
        }
        column |= digit << axis;
        ternary_index /= 3;
    }
    Some(column)
}

/// Evaluate the full lexicographic mixed basis at a seven-coordinate point.
pub fn basis_weights(point: &[F128]) -> Vec<F128> {
    assert_eq!(
        point.len(),
        S,
        "mixed-basis point must have seven coordinates"
    );
    let mut weights = vec![F128::ONE];
    for &x in point {
        let factors = [F128::ONE + x, x, x.square() + x];
        let mut next = Vec::with_capacity(weights.len() * 3);
        for value in weights {
            next.extend(factors.map(|factor| value * factor));
        }
        weights = next;
    }
    weights
}

/// Compute `H = (U_A AND U_B)^T row_weights`, with `U = A (T^{tensor 7})^T`.
///
/// The returned 2187 field elements include both the 128 Boolean column
/// claims and the 2059 quotient coefficients. Neither kernel materializes
/// the transformed matrix. Lookup workers each retain just one accumulator.
pub fn aggregate(a: &[u8], b: &[u8], row_weights: &[F128], kernel: Kernel) -> Vec<F128> {
    assert_eq!(a.len(), b.len(), "Hadamard inputs must have equal length");
    assert_eq!(a.len() % ROW_BYTES, 0, "input must contain complete rows");
    assert_eq!(
        a.len() / ROW_BYTES,
        row_weights.len(),
        "one weight per row is required"
    );
    match kernel {
        Kernel::Reference => aggregate_reference(a, b, row_weights),
        Kernel::Packed => packed::aggregate64(a, b, row_weights),
        Kernel::Packed8 => packed::aggregate8(a, b, row_weights),
        Kernel::PackedLegacy => packed::aggregate64_legacy(a, b, row_weights),
        Kernel::PackedStream => packed::aggregate64_stream(a, b, row_weights),
        Kernel::PackedSimdV1 => packed::aggregate64_simd_v1(a, b, row_weights),
        Kernel::PackedSimd
        | Kernel::PackedWide
        | Kernel::PackedFused
        | Kernel::PackedNative
        | Kernel::PackedFactored => packed::aggregate64_simd(a, b, row_weights),
        Kernel::Lookup => {
            // At most one task per Rayon worker, apart from rounding. Avoid
            // allocating a 35 KiB coefficient vector for every tiny chunk.
            let rows_per_task = row_weights
                .len()
                .div_ceil(rayon::current_num_threads())
                .max(MIN_ROWS_PER_TASK);
            a.par_chunks(rows_per_task * ROW_BYTES)
                .zip(b.par_chunks(rows_per_task * ROW_BYTES))
                .zip(row_weights.par_chunks(rows_per_task))
                .map(|((a, b), weights)| aggregate_lookup_chunk(a, b, weights))
                .reduce_with(|mut left, right| {
                    for (target, value) in left.iter_mut().zip(right) {
                        *target += value;
                    }
                    left
                })
                .unwrap_or_else(|| vec![F128::ZERO; COEFFICIENTS])
        }
    }
}

/// Preserve the HighToLow row point while sharing inner affine matrices.
pub(crate) fn aggregate_factored(a: &[u8], b: &[u8], row_point: &[F128]) -> Vec<F128> {
    packed::aggregate64_factored(a, b, row_point, 16)
}

/// Deliberately simple scalar recursion, independent of the byte table.
fn transform_reference(input: &[u8]) -> Vec<u8> {
    assert!(input.len().is_power_of_two());
    if input.len() == 1 {
        return vec![input[0]];
    }
    let half = input.len() / 2;
    let sum: Vec<u8> = input[..half]
        .iter()
        .zip(&input[half..])
        .map(|(left, right)| left ^ right)
        .collect();
    let mut output = transform_reference(&input[..half]);
    output.extend(transform_reference(&input[half..]));
    output.extend(transform_reference(&sum));
    output
}

fn aggregate_reference(a: &[u8], b: &[u8], weights: &[F128]) -> Vec<F128> {
    let mut coefficients = vec![F128::ZERO; COEFFICIENTS];
    for ((a, b), &weight) in a
        .chunks_exact(ROW_BYTES)
        .zip(b.chunks_exact(ROW_BYTES))
        .zip(weights)
    {
        let unpack = |row: &[u8]| -> Vec<u8> {
            (0..KAPPA)
                .map(|column| (row[column / 8] >> (column % 8)) & 1)
                .collect()
        };
        let a = transform_reference(&unpack(a));
        let b = transform_reference(&unpack(b));
        for ((target, left), right) in coefficients.iter_mut().zip(a).zip(b) {
            if left & right != 0 {
                *target += weight;
            }
        }
    }
    coefficients
}

/// Apply the four outer tensor factors to blocks holding 27 inner bits.
/// The recursion uses 65 u32 XORs for its sixteen-word input, with no heap.
fn transform_outer(input: &[u32], output: &mut [u32]) {
    if input.len() == 1 {
        output[0] = input[0];
        return;
    }
    let half = input.len() / 2;
    let mut sums = [0u32; ROW_BYTES / 2];
    for index in 0..half {
        sums[index] = input[index] ^ input[index + half];
    }
    let third = output.len() / 3;
    let (low, rest) = output.split_at_mut(third);
    let (high, sum) = rest.split_at_mut(third);
    transform_outer(&input[..half], low);
    transform_outer(&input[half..], high);
    transform_outer(&sums[..half], sum);
}

fn transform_lookup(row: &[u8]) -> [u32; OUTER_COEFFICIENTS] {
    debug_assert_eq!(row.len(), ROW_BYTES);
    let mut inner = [0u32; ROW_BYTES];
    for (target, &byte) in inner.iter_mut().zip(row) {
        *target = BYTE_TRANSFORM[byte as usize];
    }
    let mut output = [0u32; OUTER_COEFFICIENTS];
    transform_outer(&inner, &mut output);
    output
}

fn aggregate_lookup_chunk(a: &[u8], b: &[u8], weights: &[F128]) -> Vec<F128> {
    let mut coefficients = vec![F128::ZERO; COEFFICIENTS];
    for ((a, b), &weight) in a
        .chunks_exact(ROW_BYTES)
        .zip(b.chunks_exact(ROW_BYTES))
        .zip(weights)
    {
        let a = transform_lookup(a);
        let b = transform_lookup(b);
        for (outer, (&left, &right)) in a.iter().zip(&b).enumerate() {
            let mut product = left & right;
            while product != 0 {
                let inner = product.trailing_zeros() as usize;
                coefficients[outer * INNER_COEFFICIENTS + inner] += weight;
                product &= product - 1;
            }
        }
    }
    coefficients
}

#[cfg(test)]
mod tests {
    use super::{
        BYTE_TRANSFORM, COEFFICIENTS, KAPPA, Kernel, QUOTIENTS, ROW_BYTES, S, aggregate,
        basis_weights, boolean_column, transform_lookup, transform_reference,
    };
    use flock_field::{F128, test_rng::Rng};

    #[test]
    fn every_byte_lookup_matches_independent_scalar_transform() {
        assert_eq!(std::mem::size_of_val(&BYTE_TRANSFORM), 1024);
        for byte in 0..256 {
            let input: Vec<u8> = (0..8).map(|bit| ((byte >> bit) & 1) as u8).collect();
            let expected = transform_reference(&input);
            let actual = BYTE_TRANSFORM[byte];
            assert_eq!(actual >> 27, 0);
            for (index, bit) in expected.into_iter().enumerate() {
                assert_eq!(
                    ((actual >> index) & 1) as u8,
                    bit,
                    "byte {byte}, coefficient {index}"
                );
            }
        }
    }

    #[test]
    fn full_transform_preserves_lexicographic_coordinates() {
        let mut rng = Rng::new(0x7E450001);
        // Basis vectors detect column swaps; random rows exercise XOR paths.
        for sample in 0..KAPPA + 8 {
            let mut row = [0u8; ROW_BYTES];
            if sample < KAPPA {
                row[sample / 8] = 1 << (sample % 8);
            } else {
                rng.fill_bytes(&mut row);
            }
            let bits: Vec<u8> = (0..KAPPA)
                .map(|column| (row[column / 8] >> (column % 8)) & 1)
                .collect();
            let expected = transform_reference(&bits);
            let actual = transform_lookup(&row);
            for index in 0..COEFFICIENTS {
                assert_eq!(
                    ((actual[index / 27] >> (index % 27)) & 1) as u8,
                    expected[index]
                );
            }
        }
    }

    #[test]
    fn column_and_quotient_slots_partition_the_tensor() {
        let mut seen = [false; KAPPA];
        let mut quotient_sizes = [0; S];
        for index in 0..COEFFICIENTS {
            if let Some(column) = boolean_column(index) {
                assert!(!seen[column]);
                seen[column] = true;
                let ternary = (0..S).fold(0, |value, axis| {
                    value * 3 + ((column >> (S - 1 - axis)) & 1)
                });
                assert_eq!(index, ternary);
            } else {
                let mut place = 3usize.pow((S - 1) as u32);
                for size in &mut quotient_sizes {
                    if (index / place) % 3 == 2 {
                        *size += 1;
                        break;
                    }
                    place /= 3;
                }
            }
        }
        assert!(seen.into_iter().all(|seen| seen));
        assert_eq!(quotient_sizes, [729, 486, 324, 216, 144, 96, 64]);
        assert_eq!(quotient_sizes.iter().sum::<usize>(), QUOTIENTS);
        assert_eq!(boolean_column(COEFFICIENTS), None);
    }

    #[test]
    fn aggregate_lookup_matches_reference_and_column_products() {
        let mut rng = Rng::new(0xA667E6A7);
        for rows in [0, 1, 3, 257] {
            let mut a = vec![0u8; rows * ROW_BYTES];
            let mut b = vec![0u8; rows * ROW_BYTES];
            rng.fill_bytes(&mut a);
            rng.fill_bytes(&mut b);
            let weights = rng.f128_vec(rows);
            let actual = aggregate(&a, &b, &weights, Kernel::Lookup);
            assert_eq!(actual, aggregate(&a, &b, &weights, Kernel::Reference));
            for (index, &value) in actual.iter().enumerate() {
                if let Some(column) = boolean_column(index) {
                    let expected = (0..rows).fold(F128::ZERO, |sum, row| {
                        if (a[row * ROW_BYTES + column / 8] & b[row * ROW_BYTES + column / 8])
                            >> (column % 8)
                            & 1
                            != 0
                        {
                            sum + weights[row]
                        } else {
                            sum
                        }
                    });
                    assert_eq!(value, expected);
                }
            }
        }
    }

    #[test]
    fn mixed_basis_certificate_matches_direct_multilinear_products() {
        let mut rng = Rng::new(0xCE471F1CA7E);
        let rows = 3;
        let mut a = vec![0u8; rows * ROW_BYTES];
        let mut b = vec![0u8; rows * ROW_BYTES];
        rng.fill_bytes(&mut a);
        rng.fill_bytes(&mut b);
        let weights = rng.f128_vec(rows);
        let coefficients = aggregate(&a, &b, &weights, Kernel::Lookup);
        for sample in 0..4 {
            let point: Vec<F128> = (0..S)
                .map(|axis| {
                    if sample == 0 {
                        if axis % 2 == 0 { F128::ZERO } else { F128::ONE }
                    } else {
                        rng.f128()
                    }
                })
                .collect();
            let chi = basis_weights(&point);
            let actual = coefficients
                .iter()
                .zip(chi)
                .fold(F128::ZERO, |sum, (&coefficient, basis)| {
                    sum + coefficient * basis
                });
            // Evaluate each multilinear row directly from its Boolean values;
            // this oracle does not use the transform or mixed-basis builder.
            let eval = |row: &[u8]| {
                (0..KAPPA).fold(F128::ZERO, |sum, column| {
                    if row[column / 8] >> (column % 8) & 1 == 0 {
                        return sum;
                    }
                    let term = point
                        .iter()
                        .enumerate()
                        .fold(F128::ONE, |product, (axis, &x)| {
                            product
                                * if column >> (S - 1 - axis) & 1 == 0 {
                                    F128::ONE + x
                                } else {
                                    x
                                }
                        });
                    sum + term
                })
            };
            let expected = (0..rows).fold(F128::ZERO, |sum, row| {
                sum + weights[row]
                    * eval(&a[row * ROW_BYTES..(row + 1) * ROW_BYTES])
                    * eval(&b[row * ROW_BYTES..(row + 1) * ROW_BYTES])
            });
            assert_eq!(actual, expected);
        }
    }

    #[test]
    #[should_panic(expected = "one weight per row")]
    fn aggregate_rejects_missing_row_weights() {
        aggregate(&[0; ROW_BYTES], &[0; ROW_BYTES], &[], Kernel::Lookup);
    }
}
