//! Batched GFNI evaluation of an arbitrary binary-linear coefficient map.
//! Challenge-dependent matrices are built inside `project`; source bytes keep
//! their original physical order. No transformed witness is cached offline.

use flock_field::F128;
use rayon::prelude::*;
use std::mem::MaybeUninit;

// [16-byte input half][output byte][input byte within that half].
type Matrices = [[[u64; 16]; 16]; 2];

fn matrices(basis: &[F128]) -> Matrices {
    let mut maps = [[[0u64; 16]; 16]; 2];
    for (input, images) in basis.chunks_exact(8).enumerate() {
        for (bit, &image) in images.iter().enumerate() {
            let bytes = (u128::from(image.lo) | (u128::from(image.hi) << 64)).to_le_bytes();
            for output in 0..16 {
                for output_bit in 0..8 {
                    // GFNI output bit i uses matrix byte 7-i, with input bit
                    // ordering unchanged. This matches native_coordinates.
                    maps[input / 16][output][input % 16] |=
                        u64::from((bytes[output] >> output_bit) & 1)
                            << (8 * (7 - output_bit) + bit);
                }
            }
        }
    }
    maps
}

fn available() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        std::is_x86_feature_detected!("gfni")
            && std::is_x86_feature_detected!("avx512f")
            && std::is_x86_feature_detected!("avx512bw")
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}

/// Initialize all even/odd outputs when the shape and native CPU are supported.
/// Otherwise return false without writing any output. `basis` is ordered by
/// physical input bit: column, polynomial coefficient, then coefficient bit.
pub(super) fn project(
    bytes: &[u8],
    input_bytes: usize,
    basis: &[F128],
    constant: F128,
    a: &mut [MaybeUninit<F128>],
    b: &mut [MaybeUninit<F128>],
) -> bool {
    if !matches!(input_bytes, 16 | 32)
        || a.len() != b.len()
        || basis.len() != input_bytes * 8
        || a.len()
            .checked_mul(2)
            .and_then(|n| n.checked_mul(input_bytes))
            != Some(bytes.len())
        || !available()
    {
        return false;
    }
    let maps = matrices(basis);
    #[cfg(target_arch = "x86_64")]
    a.par_chunks_mut(1024)
        .zip(b.par_chunks_mut(1024))
        .enumerate()
        .for_each(|(block, (a, b))| {
            let first = 2 * block * 1024 * input_bytes;
            let input = &bytes[first..first + 2 * a.len() * input_bytes];
            // SAFETY: available() checked all declared instructions. The input
            // slice covers exactly two rows per even/odd output pair, and each
            // chunk owns all its disjoint initialized output destinations.
            unsafe { native::chunk(input, input_bytes, basis, &maps, constant, a, b) };
        });
    true
}

/// Initialize one extension-field value per physical input row, retaining row
/// order. The matrices are built here from the current challenge-dependent
/// basis. Unsupported shapes or CPUs return false without writing any output.
pub(super) fn project_dense(
    bytes: &[u8],
    input_bytes: usize,
    basis: &[F128],
    constant: F128,
    out: &mut [MaybeUninit<F128>],
) -> bool {
    if !matches!(input_bytes, 16 | 32)
        || basis.len() != input_bytes * 8
        || out.len().checked_mul(input_bytes) != Some(bytes.len())
        || !available()
    {
        return false;
    }
    let maps = matrices(basis);
    #[cfg(target_arch = "x86_64")]
    out.par_chunks_mut(2048)
        .enumerate()
        .for_each(|(block, out)| {
            let first = block * 2048 * input_bytes;
            let input = &bytes[first..first + out.len() * input_bytes];
            // SAFETY: available() checked the required ISA. Each input row
            // maps to one output slot, and all parallel output chunks are
            // disjoint. chunk_dense initializes its full output, including tails.
            unsafe { native::chunk_dense(input, input_bytes, basis, &maps, constant, out) };
        });
    true
}

#[cfg(target_arch = "x86_64")]
mod native {
    use super::{F128, Matrices, MaybeUninit};
    use std::arch::x86_64::{
        __m512i, _mm512_gf2p8affine_epi64_epi8, _mm512_loadu_si512, _mm512_permutex2var_epi64,
        _mm512_set_epi64, _mm512_set1_epi64, _mm512_setzero_si512, _mm512_storeu_si512,
        _mm512_unpackhi_epi8, _mm512_unpackhi_epi16, _mm512_unpackhi_epi32, _mm512_unpackhi_epi64,
        _mm512_unpacklo_epi8, _mm512_unpacklo_epi16, _mm512_unpacklo_epi32, _mm512_unpacklo_epi64,
        _mm512_xor_si512,
    };

    /// Transpose the 16x16 byte matrix independently in each 128-bit lane.
    /// Applying this transpose twice recovers the exact original row ordering.
    #[inline]
    #[target_feature(enable = "avx512f,avx512bw")]
    unsafe fn transpose(mut rows: [__m512i; 16]) -> [__m512i; 16] {
        let mut next = [_mm512_setzero_si512(); 16];
        for i in (0..16).step_by(2) {
            next[i] = _mm512_unpacklo_epi8(rows[i], rows[i + 1]);
            next[i + 1] = _mm512_unpackhi_epi8(rows[i], rows[i + 1]);
        }
        rows = next;
        for group in (0..16).step_by(4) {
            for i in 0..2 {
                next[group + 2 * i] = _mm512_unpacklo_epi16(rows[group + i], rows[group + 2 + i]);
                next[group + 2 * i + 1] =
                    _mm512_unpackhi_epi16(rows[group + i], rows[group + 2 + i]);
            }
        }
        rows = next;
        for group in (0..16).step_by(8) {
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

    #[inline]
    #[target_feature(enable = "avx512f")]
    unsafe fn separate(lo: __m512i, hi: __m512i, odd: bool) -> __m512i {
        let indices = if odd {
            _mm512_set_epi64(15, 14, 11, 10, 7, 6, 3, 2)
        } else {
            _mm512_set_epi64(13, 12, 9, 8, 5, 4, 1, 0)
        };
        _mm512_permutex2var_epi64(lo, indices, hi)
    }

    /// Map a complete 64-row tile. DENSE stores all rows in physical order;
    /// otherwise a and b receive the 32 even and 32 odd rows, respectively.
    #[target_feature(enable = "gfni,avx512f,avx512bw")]
    unsafe fn tile<const DENSE: bool>(
        bytes: *const u8,
        input_bytes: usize,
        maps: &Matrices,
        constant: F128,
        a: *mut MaybeUninit<F128>,
        b: *mut MaybeUninit<F128>,
    ) {
        // SAFETY: caller supplies 64 complete 16/32-byte rows. DENSE requires
        // 64 writable outputs in a and never uses b; otherwise each pointer
        // has 32 writable slots. Loads read at most 2048 bytes, and stores
        // initialize exactly 1024 output bytes in total. No prior output is read.
        unsafe {
            let mut converted = [_mm512_setzero_si512(); 16];
            for half in 0..input_bytes / 16 {
                let rows: [__m512i; 16] = std::array::from_fn(|i| {
                    if input_bytes == 16 {
                        _mm512_loadu_si512(bytes.add(64 * i).cast())
                    } else {
                        // Two loads contain the two halves of four 32-byte
                        // rows. Separate equal halves into four 128-bit lanes.
                        let lo = _mm512_loadu_si512(bytes.add(128 * i).cast());
                        let hi = _mm512_loadu_si512(bytes.add(128 * i + 64).cast());
                        separate(lo, hi, half == 1)
                    }
                });
                let planes = transpose(rows);
                // Four independent result registers keep the 16 source planes
                // live without requiring 16 simultaneous result accumulators.
                for group in (0..16).step_by(4) {
                    let mut r0 = converted[group];
                    let mut r1 = converted[group + 1];
                    let mut r2 = converted[group + 2];
                    let mut r3 = converted[group + 3];
                    for input in 0..16 {
                        let value = planes[input];
                        r0 = _mm512_xor_si512(
                            r0,
                            _mm512_gf2p8affine_epi64_epi8::<0>(
                                value,
                                _mm512_set1_epi64(maps[half][group][input] as i64),
                            ),
                        );
                        r1 = _mm512_xor_si512(
                            r1,
                            _mm512_gf2p8affine_epi64_epi8::<0>(
                                value,
                                _mm512_set1_epi64(maps[half][group + 1][input] as i64),
                            ),
                        );
                        r2 = _mm512_xor_si512(
                            r2,
                            _mm512_gf2p8affine_epi64_epi8::<0>(
                                value,
                                _mm512_set1_epi64(maps[half][group + 2][input] as i64),
                            ),
                        );
                        r3 = _mm512_xor_si512(
                            r3,
                            _mm512_gf2p8affine_epi64_epi8::<0>(
                                value,
                                _mm512_set1_epi64(maps[half][group + 3][input] as i64),
                            ),
                        );
                    }
                    converted[group] = r0;
                    converted[group + 1] = r1;
                    converted[group + 2] = r2;
                    converted[group + 3] = r3;
                }
            }
            let rows = transpose(converted);
            let shift = _mm512_set_epi64(
                constant.hi as i64,
                constant.lo as i64,
                constant.hi as i64,
                constant.lo as i64,
                constant.hi as i64,
                constant.lo as i64,
                constant.hi as i64,
                constant.lo as i64,
            );
            if DENSE {
                for i in 0..16 {
                    _mm512_storeu_si512(a.add(4 * i).cast(), _mm512_xor_si512(rows[i], shift));
                }
            } else {
                for i in 0..8 {
                    let lo = _mm512_xor_si512(rows[2 * i], shift);
                    let hi = _mm512_xor_si512(rows[2 * i + 1], shift);
                    _mm512_storeu_si512(a.add(4 * i).cast(), separate(lo, hi, false));
                    _mm512_storeu_si512(b.add(4 * i).cast(), separate(lo, hi, true));
                }
            }
        }
    }

    #[target_feature(enable = "gfni,avx512f,avx512bw")]
    pub(super) unsafe fn chunk(
        bytes: &[u8],
        input_bytes: usize,
        basis: &[F128],
        maps: &Matrices,
        constant: F128,
        a: &mut [MaybeUninit<F128>],
        b: &mut [MaybeUninit<F128>],
    ) {
        let full = a.len() / 32 * 32;
        for pair in (0..full).step_by(32) {
            // SAFETY: one tile consumes 32 pairs within the complete prefixes.
            unsafe {
                tile::<false>(
                    bytes.as_ptr().add(2 * pair * input_bytes),
                    input_bytes,
                    maps,
                    constant,
                    a.as_mut_ptr().add(pair),
                    b.as_mut_ptr().add(pair),
                )
            };
        }
        // At most 62 input rows remain. This direct bitwise map is also valid
        // for all-zero matrices and constants and reads no uninitialized tail.
        for pair in full..a.len() {
            for (odd, destination) in [(0, &mut a[pair]), (1, &mut b[pair])] {
                let row =
                    &bytes[(2 * pair + odd) * input_bytes..(2 * pair + odd + 1) * input_bytes];
                let mut value = constant;
                for (byte, &raw) in row.iter().enumerate() {
                    let mut selected = raw;
                    while selected != 0 {
                        let bit = selected.trailing_zeros() as usize;
                        value += basis[byte * 8 + bit];
                        selected &= selected - 1;
                    }
                }
                destination.write(value);
            }
        }
    }

    #[target_feature(enable = "gfni,avx512f,avx512bw")]
    pub(super) unsafe fn chunk_dense(
        bytes: &[u8],
        input_bytes: usize,
        basis: &[F128],
        maps: &Matrices,
        constant: F128,
        out: &mut [MaybeUninit<F128>],
    ) {
        let full = out.len() / 64 * 64;
        for row in (0..full).step_by(64) {
            // SAFETY: this tile reads 64 complete rows and writes their 64
            // exclusive output slots. The DENSE instantiation never uses b.
            unsafe {
                tile::<true>(
                    bytes.as_ptr().add(row * input_bytes),
                    input_bytes,
                    maps,
                    constant,
                    out.as_mut_ptr().add(row),
                    std::ptr::null_mut(),
                )
            };
        }
        // At most 63 rows remain. A direct bitwise map avoids padding and
        // also covers all-zero maps, all-one inputs and arbitrary constants.
        for (row, destination) in out.iter_mut().enumerate().skip(full) {
            let input = &bytes[row * input_bytes..(row + 1) * input_bytes];
            let mut value = constant;
            for (byte, &raw) in input.iter().enumerate() {
                let mut selected = raw;
                while selected != 0 {
                    let bit = selected.trailing_zeros() as usize;
                    value += basis[byte * 8 + bit];
                    selected &= selected - 1;
                }
            }
            destination.write(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::linear_tables;

    fn basis(input_bytes: usize, salt: u64) -> Vec<F128> {
        (0..input_bytes * 8)
            .map(|i| {
                let mut x = (i as u64 + 17).wrapping_mul(0x9e3779b97f4a7c15) ^ salt;
                x ^= x >> 27;
                x = x.wrapping_mul(0x94d049bb133111eb);
                F128::new(x, x.rotate_left(19) ^ 0x8877665544332211)
            })
            .collect()
    }
    fn expected(bytes: &[u8], input_bytes: usize, basis: &[F128], constant: F128) -> Vec<F128> {
        let tables = linear_tables(basis);
        bytes
            .chunks_exact(input_bytes)
            .map(|row| {
                row.iter()
                    .zip(&tables)
                    .fold(constant, |v, (&x, t)| v + t[x as usize])
            })
            .collect()
    }
    fn check(bytes: &[u8], input_bytes: usize, basis: &[F128], constant: F128) {
        let rows = bytes.len() / input_bytes;
        assert_eq!(rows % 2, 0);
        let mut a = vec![MaybeUninit::new(F128::ONE); rows / 2];
        let mut b = vec![MaybeUninit::new(F128::ONE); rows / 2];
        assert!(project(bytes, input_bytes, basis, constant, &mut a, &mut b));
        let actual_a = unsafe { crate::kernels::initialized_f128(a) };
        let actual_b = unsafe { crate::kernels::initialized_f128(b) };
        let oracle = expected(bytes, input_bytes, basis, constant);
        for i in 0..rows / 2 {
            assert_eq!(actual_a[i], oracle[2 * i]);
            assert_eq!(actual_b[i], oracle[2 * i + 1]);
        }
    }

    fn check_dense(bytes: &[u8], input_bytes: usize, basis: &[F128], constant: F128) -> Vec<F128> {
        let rows = bytes.len() / input_bytes;
        let sentinel = F128::new(0x1234_abcd, 0x9876_5432);
        let mut guarded = vec![MaybeUninit::new(sentinel); rows + 2];
        assert!(project_dense(
            bytes,
            input_bytes,
            basis,
            constant,
            &mut guarded[1..rows + 1],
        ));
        // SAFETY: sentinels were initialized on creation, and successful
        // project_dense initialized every interior output slot.
        let guarded = unsafe { crate::kernels::initialized_f128(guarded) };
        assert_eq!(guarded[0], sentinel);
        assert_eq!(guarded[rows + 1], sentinel);
        let actual = &guarded[1..rows + 1];
        assert_eq!(actual, expected(bytes, input_bytes, basis, constant));
        actual.to_vec()
    }

    #[test]
    fn packed_dense_gfni_all_basis_bits_preserve_physical_row_order() {
        if !available() {
            return;
        }
        for input_bytes in [16, 32] {
            let basis = basis(input_bytes, 53);
            let bits = input_bytes * 8;
            let mut bytes = vec![0; bits * input_bytes];
            for row in 0..bits {
                let bit = (row * 73 + 19) % bits;
                bytes[row * input_bytes + bit / 8] = 1 << (bit % 8);
            }
            for constant in [F128::ZERO, F128::ONE, F128::new(19, 37)] {
                let actual = check_dense(&bytes, input_bytes, &basis, constant);
                for row in 0..bits {
                    assert_eq!(actual[row], constant + basis[(row * 73 + 19) % bits]);
                }
            }
        }
    }

    #[test]
    fn packed_dense_gfni_random_zero_one_tails_and_parallel_boundaries() {
        if !available() {
            return;
        }
        for input_bytes in [16, 32] {
            for rows in [
                0usize, 1, 2, 7, 31, 32, 33, 63, 64, 65, 127, 128, 129, 2047, 2048, 2049, 4097,
            ] {
                let bytes: Vec<_> = (0..rows * input_bytes)
                    .map(|i| {
                        let x = (i as u64 + 113).wrapping_mul(0xd6e8feb86659fd93);
                        (x ^ (x >> 37)) as u8
                    })
                    .collect();
                let general = basis(input_bytes, 71);
                for constant in [F128::ZERO, F128::ONE, F128::new(17, 23)] {
                    check_dense(&bytes, input_bytes, &general, constant);
                }
            }
            for raw in [0u8, 1, 255] {
                let bytes = vec![raw; 129 * input_bytes];
                for image in [F128::ZERO, F128::ONE] {
                    for constant in [F128::ZERO, F128::ONE] {
                        check_dense(&bytes, input_bytes, &vec![image; 8 * input_bytes], constant);
                    }
                }
            }
        }
    }

    #[test]
    fn packed_dense_degree_four_sf32_matches_legacy_polynomial_evaluation() {
        if !available() {
            return;
        }
        let field = crate::field::FastField::for_width(32).unwrap();
        for z in [F128::ZERO, F128::ONE, F128::new(0x2345_6789, 0xabcd_0123)] {
            let old = crate::field::PolynomialEvaluation::new(field.base, 128, z);
            let mut power = F128::ONE;
            let mut basis = Vec::with_capacity(128);
            for _ in 0..4 {
                basis.extend(field.base.basis().iter().map(|&b| b * power));
                power *= z;
            }
            for rows in [0usize, 1, 63, 64, 65, 2048, 2049] {
                let bytes: Vec<_> = (0..rows * 16)
                    .map(|i| {
                        let x = (i as u64 + 97).wrapping_mul(0x94d049bb133111eb);
                        (x ^ (x >> 29)) as u8
                    })
                    .collect();
                let actual = check_dense(&bytes, 16, &basis, power);
                let expected: Vec<_> = bytes
                    .chunks_exact(16)
                    .map(|row| old.evaluate(u128::from_le_bytes(row.try_into().unwrap())))
                    .collect();
                assert_eq!(actual, expected);
            }
        }
    }

    #[test]
    fn packed_dense_degree_two_sf64_matches_legacy_polynomial_evaluation() {
        if !available() {
            return;
        }
        let field = crate::field::FastField::for_width(64).unwrap();
        for threads in [1, 4] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap()
                .install(|| {
                    for z in [F128::ZERO, F128::ONE, F128::new(0x2345_6789, 0xabcd_0123)] {
                        let old = crate::field::PolynomialEvaluation::new(field.base, 128, z);
                        // The K64 parent is c0 + c1 Z + Z^2, with the
                        // two coefficients physically stored as adjacent u64s.
                        let basis: Vec<_> = field
                            .base
                            .basis()
                            .iter()
                            .copied()
                            .chain(field.base.basis().iter().map(|&b| b * z))
                            .collect();
                        for rows in [0usize, 1, 63, 64, 65, 2047, 2048, 2049] {
                            for pattern in [0u8, 1, 2] {
                                let bytes: Vec<_> = (0..rows * 16)
                                    .map(|i| match pattern {
                                        0 => 0,
                                        1 => 255,
                                        _ => {
                                            let x =
                                                (i as u64 + 97).wrapping_mul(0x94d049bb133111eb);
                                            (x ^ (x >> 29)) as u8
                                        }
                                    })
                                    .collect();
                                let actual = check_dense(&bytes, 16, &basis, z * z);
                                for (value, row) in actual.iter().zip(bytes.chunks_exact(16)) {
                                    let word = u128::from_le_bytes(row.try_into().unwrap());
                                    let c0 = u64::from_le_bytes(row[..8].try_into().unwrap());
                                    let c1 = u64::from_le_bytes(row[8..].try_into().unwrap());
                                    let scalar = field.embed(c0 as u128)
                                        + field.embed(c1 as u128) * z
                                        + z * z;
                                    assert_eq!(
                                        *value, scalar,
                                        "threads={threads}, rows={rows}, pattern={pattern}"
                                    );
                                    assert_eq!(*value, old.evaluate(word));
                                }
                            }
                        }
                    }
                });
        }
    }

    #[test]
    fn packed_projection_two_columns_sf64_matches_scalar_leaf_contraction() {
        if !available() {
            return;
        }
        let field = crate::field::FastField::for_width(64).unwrap();
        let domain = super::super::domain::Domain::new(field);
        assert_eq!(domain.kappa, 2);
        for threads in [1, 4] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap()
                .install(|| {
                    // x=0/1 select an individual column. These challenges are
                    // excluded by the protocol, but stress zero matrix halves.
                    for x in [F128::ZERO, F128::ONE, F128::new(19, 37)] {
                        let rho = domain.packing(x);
                        assert_eq!(rho[0] + rho[1], F128::ONE);
                        let basis: Vec<_> = rho.iter().flat_map(|&weight| {
                            field.base.basis().iter().map(move |&b| b * weight)
                        }).collect();
                        for z in [F128::ZERO, F128::ONE, F128::new(0x7654_3210, 0xfedc_ba98)] {
                            for pairs in [0usize, 1, 31, 32, 33, 1023, 1024, 1025] {
                                for pattern in [0u8, 1, 2] {
                                    let bytes: Vec<_> = (0..2 * pairs * 16).map(|i| match pattern {
                                        0 => 0,
                                        1 => 255,
                                        _ => {
                                            let x = (i as u64 + 113).wrapping_mul(0xd6e8feb86659fd93);
                                            (x ^ (x >> 37)) as u8
                                        }
                                    }).collect();
                                    let sentinel = F128::new(0x1234_abcd, 0x9876_5432);
                                    let mut a = vec![MaybeUninit::new(sentinel); pairs + 2];
                                    let mut b = vec![MaybeUninit::new(sentinel); pairs + 2];
                                    assert!(project(&bytes, 16, &basis, z, &mut a[1..pairs + 1], &mut b[1..pairs + 1]));
                                    // SAFETY: guard elements were initialized, and successful
                                    // dispatch initializes every interior even/odd output.
                                    let a = unsafe { crate::kernels::initialized_f128(a) };
                                    let b = unsafe { crate::kernels::initialized_f128(b) };
                                    assert_eq!((a[0], a[pairs + 1], b[0], b[pairs + 1]), (sentinel, sentinel, sentinel, sentinel));
                                    for (row_index, row) in bytes.chunks_exact(16).enumerate() {
                                        let t0 = u64::from_le_bytes(row[..8].try_into().unwrap());
                                        let t1 = u64::from_le_bytes(row[8..].try_into().unwrap());
                                        // Independent scalar oracle keeps the shift inside
                                        // each column, checking the implicit monic term too.
                                        let scalar = rho[0] * (field.embed(t0 as u128) + z)
                                            + rho[1] * (field.embed(t1 as u128) + z);
                                        let actual = if row_index & 1 == 0 { a[row_index / 2 + 1] } else { b[row_index / 2 + 1] };
                                        assert_eq!(actual, scalar, "threads={threads}, pairs={pairs}, row={row_index}, pattern={pattern}");
                                    }
                                }
                            }
                        }
                    }
                });
        }
    }

    #[test]
    fn packed_dense_unsupported_shapes_do_not_write_outputs() {
        let sentinel = F128::new(17, 23);
        let mut out = [MaybeUninit::new(sentinel); 4];
        for (bytes, input_bytes, basis_len) in [
            (vec![0u8; 96], 24usize, 192usize),
            (vec![0u8; 64], 16, 127),
            (vec![0u8; 63], 16, 128),
            (vec![0u8; 80], 16, 128),
            (vec![0u8; 127], 32, 256),
            (vec![], usize::MAX, 0),
        ] {
            assert!(!project_dense(
                &bytes,
                input_bytes,
                &vec![F128::ZERO; basis_len],
                F128::ZERO,
                &mut out,
            ));
            // SAFETY: outputs were initialized with sentinels and failed
            // dispatch must not have changed any of them.
            assert!(
                out.iter()
                    .all(|value| unsafe { value.assume_init() } == sentinel)
            );
        }
    }

    #[test]
    fn packed_projection_gfni_matrices_match_every_input_byte() {
        for input_bytes in [16, 32] {
            let basis = basis(input_bytes, 13);
            let maps = matrices(&basis);
            let tables = linear_tables(&basis);
            for input in 0..input_bytes {
                for x in 0..=255u8 {
                    let mut bytes = [0u8; 16];
                    for output in 0..16 {
                        let matrix = maps[input / 16][output][input % 16];
                        for bit in 0..8 {
                            let mask = (matrix >> (8 * (7 - bit))) as u8;
                            bytes[output] |= ((mask & x).count_ones() as u8 & 1) << bit;
                        }
                    }
                    let value = F128::new(
                        u64::from_le_bytes(bytes[..8].try_into().unwrap()),
                        u64::from_le_bytes(bytes[8..].try_into().unwrap()),
                    );
                    assert_eq!(value, tables[input][x as usize]);
                }
            }
        }
    }

    #[test]
    fn packed_projection_gfni_all_basis_bits_and_input_halves_match_scalar() {
        if !available() {
            return;
        }
        for input_bytes in [16, 32] {
            let basis = basis(input_bytes, 31);
            let mut bytes = vec![0u8; 2 * input_bytes * 8 * input_bytes];
            for bit in 0..input_bytes * 8 {
                bytes[(2 * bit) * input_bytes + bit / 8] = 1 << (bit % 8);
                let other = (bit + 73) % (input_bytes * 8);
                bytes[(2 * bit + 1) * input_bytes + other / 8] = 1 << (other % 8);
            }
            check(&bytes, input_bytes, &basis, F128::new(19, 37));
        }
    }

    #[test]
    fn packed_projection_gfni_random_zero_one_tails_and_parallel_boundaries() {
        if !available() {
            return;
        }
        for input_bytes in [16, 32] {
            for pairs in [
                0usize, 1, 7, 15, 16, 31, 32, 33, 63, 64, 65, 1023, 1024, 1025,
            ] {
                let bytes: Vec<_> = (0..2 * pairs * input_bytes)
                    .map(|i| {
                        let x = (i as u64 + 91).wrapping_mul(0xd6e8feb86659fd93);
                        (x ^ (x >> 37)) as u8
                    })
                    .collect();
                let general = basis(input_bytes, 43);
                for constant in [F128::ZERO, F128::ONE, F128::new(17, 23)] {
                    check(&bytes, input_bytes, &general, constant);
                }
            }
            let bytes = vec![255u8; 130 * input_bytes];
            check(
                &bytes,
                input_bytes,
                &vec![F128::ZERO; 8 * input_bytes],
                F128::ONE,
            );
            check(
                &bytes,
                input_bytes,
                &vec![F128::ONE; 8 * input_bytes],
                F128::ZERO,
            );
        }
    }

    #[test]
    fn packed_projection_unsupported_shapes_do_not_write_outputs() {
        let mut a = [MaybeUninit::new(F128::ONE); 1];
        let mut b = a;
        assert!(!project(
            &[0; 48],
            24,
            &[F128::ZERO; 192],
            F128::ZERO,
            &mut a,
            &mut b
        ));
        assert!(!project(
            &[0; 32],
            16,
            &[F128::ZERO; 127],
            F128::ZERO,
            &mut a,
            &mut b
        ));
        assert!(!project(
            &[0; 31],
            16,
            &[F128::ZERO; 128],
            F128::ZERO,
            &mut a,
            &mut b
        ));
        assert_eq!(unsafe { a[0].assume_init() }, F128::ONE);
        assert_eq!(unsafe { b[0].assume_init() }, F128::ONE);
    }
}
