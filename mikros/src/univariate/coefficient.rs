//! Experimental coefficient-form aggregation for binary univariate Hadamard.
//!
//! The interpolation field, nodes, GHASH embedding and quotient polynomial are
//! exactly those of the existing evaluation-form path. The 255-entry output is
//! deliberately different: entries 0..128 are weighted column claims on H,
//! while entries 128..255 are weighted product coefficients of degrees 128..254.
//! Only [`quotient`] understands the latter representation.
//!
//! Binary interpolation is linear. A shared Four-Russians table caches the sum
//! of eight monomial Lagrange coefficient vectors for every source byte. Its
//! 16 * 256 * 128 bytes (512 KiB) replace per-bit GF(2)-GF(256) multiplication
//! by sixteen table reads and XORs per row. Addition needs no field table.
//! The existing 64 KiB multiplication table and field-basis maps are reused.
//!
//! Schoolbook multiplication computes only the high triangular half: at most
//! 1+...+127 = 8128 GF(256) products per row, versus 16384 for a full product.
//! This suffices because the interpolated column polynomial has degree <128.
//! Sparse monic division needs no low product coefficients or interpolation.
//! This is a separate experiment; it does not change the default proof path.

use std::sync::OnceLock;

use flock_field::F128;
use rayon::prelude::{IndexedParallelIterator, ParallelIterator, ParallelSlice};

use super::{KAPPA, QUOTIENTS, SAMPLES, domain, field::field};

#[cfg(target_arch = "x86_64")]
mod carryless;
mod encoding;

const COORDINATES: usize = 16;
const BLOCK_ROWS: usize = 512;

struct Tables {
    row_coefficients: Box<[[u8; KAPPA]]>,
}

// Quotient data is independent of the row-encoding tables, so the nibble
// experiment does not need to initialize the old 512 KiB byte table.
fn psi_terms() -> &'static [(usize, u8)] {
    static TERMS: OnceLock<Vec<(usize, u8)>> = OnceLock::new();
    TERMS.get_or_init(|| {
        let nodes: Vec<_> = (0..KAPPA).map(|node| node as u8).collect();
        let psi = domain::vanishing_polynomial(&nodes);
        assert_eq!(psi.len(), KAPPA + 1);
        assert_eq!(psi[KAPPA], 1);
        psi[..KAPPA]
            .iter()
            .copied()
            .enumerate()
            .filter(|&(_, coefficient)| coefficient != 0)
            .collect()
    })
}

fn tables() -> &'static Tables {
    static TABLES: OnceLock<Tables> = OnceLock::new();
    TABLES.get_or_init(|| {
        let nodes: Vec<_> = (0..KAPPA).map(|node| node as u8).collect();
        let (basis, _) = domain::lagrange_basis(&nodes);
        let mut row_coefficients = vec![[0; KAPPA]; COORDINATES * 256].into_boxed_slice();
        for chunk in 0..COORDINATES {
            for byte in 1usize..256 {
                let current = chunk * 256 + byte;
                let previous = chunk * 256 + (byte & (byte - 1));
                let bit_basis = &basis[chunk * 8 + byte.trailing_zeros() as usize];
                for (degree, &coefficient) in bit_basis.iter().enumerate() {
                    row_coefficients[current][degree] =
                        row_coefficients[previous][degree] ^ coefficient;
                }
            }
        }
        Tables { row_coefficients }
    })
}

/// Initialize reusable, public coefficient and intermediate-field tables.
/// Call before the online timer when measuring an offline-prepared experiment.
pub(crate) fn prepare() {
    let _ = field();
    let _ = tables();
    let _ = psi_terms();
}

pub(crate) fn prepare_kernel(kernel: super::Kernel) {
    use super::Kernel;
    if matches!(
        kernel,
        Kernel::CoefficientCarryless | Kernel::CoefficientCarrylessNibble
    ) && super::carryless_backend() == "vpclmul-avx512"
    {
        let _ = field();
        let _ = psi_terms();
        if kernel == Kernel::CoefficientCarryless {
            encoding::prepare_mode(encoding::Encoding::Byte);
        } else {
            encoding::prepare();
        }
    } else {
        prepare();
    }
}

impl Tables {
    fn coefficients(&self, row: &[u8; COORDINATES]) -> [u8; KAPPA] {
        let mut output = [0; KAPPA];
        for (chunk, &byte) in row.iter().enumerate() {
            let table = &self.row_coefficients[chunk * 256 + usize::from(byte)];
            for (value, &coefficient) in output.iter_mut().zip(table) {
                *value ^= coefficient;
            }
        }
        output
    }
}

/// Return H's 128 weighted column claims followed by 127 weighted high product
/// coefficients, all in the existing sixteen GF(256) coordinates of GHASH.
/// `native=true` selects GFNI/AVX-512 only after checking all required features;
/// unsupported hosts execute the identical table-schoolbook calculation.
///
/// Inputs contain consecutive 16-byte binary rows and one F128 weight per row.
/// Workers retain a fixed-size accumulator and tiles of at most 64 converted weights.
#[cfg(test)]
pub(crate) fn aggregate(
    a: &[u8],
    b: &[u8],
    weights: &[F128],
    native: bool,
) -> Vec<[u8; COORDINATES]> {
    aggregate_with_kernel(
        a,
        b,
        weights,
        if native {
            super::Kernel::CoefficientNative
        } else {
            super::Kernel::CoefficientLookup
        },
    )
}

pub(crate) fn aggregate_with_kernel(
    a: &[u8],
    b: &[u8],
    weights: &[F128],
    kernel: super::Kernel,
) -> Vec<[u8; COORDINATES]> {
    use super::Kernel;
    assert!(kernel.is_coefficient());
    assert_eq!(a.len(), b.len(), "coefficient input lengths differ");
    assert_eq!(
        a.len(),
        weights.len() * COORDINATES,
        "incomplete binary rows"
    );
    prepare_kernel(kernel);
    let is_carryless = matches!(
        kernel,
        Kernel::CoefficientCarryless | Kernel::CoefficientCarrylessNibble
    );
    // Match PairAggregate's work partition for the new algorithms.
    let block_rows = if is_carryless { 2048 } else { BLOCK_ROWS };
    a.par_chunks(block_rows * COORDINATES)
        .zip(b.par_chunks(block_rows * COORDINATES))
        .zip(weights.par_chunks(block_rows))
        .map(|((a, b), weights)| {
            #[cfg(target_arch = "x86_64")]
            if kernel != Kernel::CoefficientLookup && super::has_gfni() {
                if is_carryless && std::is_x86_feature_detected!("vpclmulqdq") {
                    // SAFETY: features are checked and chunks contain complete rows.
                    return unsafe {
                        carryless_block(
                            a,
                            b,
                            weights,
                            if kernel == Kernel::CoefficientCarryless {
                                encoding::Encoding::Byte
                            } else {
                                encoding::Encoding::Nibble
                            },
                        )
                    };
                }
                // SAFETY: runtime dispatch checks GFNI and AVX-512F/BW. The
                // length checks and equal chunking supply only complete rows.
                return unsafe { native_block(a, b, weights) };
            }
            lookup_block(a, b, weights)
        })
        .reduce(
            || vec![[0; COORDINATES]; SAMPLES],
            |mut left, right| {
                for (left, right) in left.iter_mut().zip(right) {
                    for (x, y) in left.iter_mut().zip(right) {
                        *x ^= y;
                    }
                }
                left
            },
        )
}

/// Divide the coefficient-form residual by the monic vanishing polynomial.
/// The caller must supply this module's 255-entry representation, not samples.
///
/// The degree <128 column polynomial changes only the discarded remainder, so
/// high coefficients alone determine the quotient. This routine constructs h;
/// it does not check the claimed columns or prove that the remainder is zero.
/// The original protocol must still bind/check those columns and the identity.
pub(crate) fn quotient(aggregate: &[[u8; COORDINATES]]) -> Vec<F128> {
    assert_eq!(
        aggregate.len(),
        SAMPLES,
        "wrong coefficient aggregate length"
    );
    let f = field();
    let mut high = aggregate[KAPPA..].to_vec();
    // Descending monic long division. A processed slot can retain its quotient
    // coefficient: every nonleading divisor term only updates lower slots.
    for degree in (0..QUOTIENTS).rev() {
        let coefficient = high[degree];
        for &(psi_degree, psi_coefficient) in psi_terms() {
            if degree + psi_degree >= KAPPA {
                let target = &mut high[degree + psi_degree - KAPPA];
                for (output, value) in target.iter_mut().zip(coefficient) {
                    *output ^= f.mul(value, psi_coefficient);
                }
            }
        }
    }
    high.iter()
        .map(|coefficient| f.compose(coefficient))
        .collect()
}

fn high_product(left: &[u8; KAPPA], right: &[u8; KAPPA]) -> [u8; QUOTIENTS] {
    let f = field();
    let mut high = [0; QUOTIENTS];
    for (degree, &a) in left.iter().enumerate().skip(1) {
        if a != 0 {
            for (output, &b) in high[..degree].iter_mut().zip(&right[KAPPA - degree..]) {
                *output ^= f.mul(a, b);
            }
        }
    }
    high
}

fn lookup_block(a: &[u8], b: &[u8], weights: &[F128]) -> Vec<[u8; COORDINATES]> {
    let f = field();
    let t = tables();
    let mut sums = vec![0u128; SAMPLES];
    let mut weighted = [0u128; 256];
    for ((a, b), &weight) in a
        .chunks_exact(COORDINATES)
        .zip(b.chunks_exact(COORDINATES))
        .zip(weights)
    {
        let a: &[u8; COORDINATES] = a.try_into().expect("complete binary row");
        let b: &[u8; COORDINATES] = b.try_into().expect("complete binary row");
        let left = t.coefficients(a);
        let right = t.coefficients(b);
        let high = high_product(&left, &right);
        let coordinates = f.decompose(weight);
        let weight_bits = u128::from_le_bytes(coordinates);
        let mut mask = u128::from_le_bytes(*a) & u128::from_le_bytes(*b);
        while mask != 0 {
            sums[mask.trailing_zeros() as usize] ^= weight_bits;
            mask &= mask - 1;
        }
        // This row's K-linear map c -> c*l uses only 8*16 table products;
        // the remaining 255 subset entries require XORs, not F128 products.
        let basis: [u128; 8] = std::array::from_fn(|bit| {
            u128::from_le_bytes(coordinates.map(|coordinate| f.mul(coordinate, 1 << bit)))
        });
        for value in 1usize..256 {
            weighted[value] =
                weighted[value & (value - 1)] ^ basis[value.trailing_zeros() as usize];
        }
        for (sum, value) in sums[KAPPA..].iter_mut().zip(high) {
            *sum ^= weighted[usize::from(value)];
        }
    }
    sums.into_iter().map(u128::to_le_bytes).collect()
}

#[cfg(target_arch = "x86_64")]
#[inline]
#[target_feature(enable = "avx512f")]
unsafe fn coefficient_pair_native(
    t: &Tables,
    a: &[u8; COORDINATES],
    b: &[u8; COORDINATES],
) -> ([u8; KAPPA], [u8; KAPPA + 64]) {
    use std::arch::x86_64::{
        _mm512_loadu_si512, _mm512_setzero_si512, _mm512_storeu_si512, _mm512_xor_si512,
    };

    let mut left = [0; KAPPA];
    // The extra 64 right coefficients are identically zero. They make every
    // shifted 64-byte schoolbook read in high_product_native stay in bounds.
    let mut right = [0; KAPPA + 64];
    let mut a0 = _mm512_setzero_si512();
    let mut a1 = a0;
    let mut b0 = a0;
    let mut b1 = a0;
    // SAFETY: feature declared above; each table has exactly 128 bytes, and
    // the two stores per output cover only initialized bytes 0..128.
    unsafe {
        for chunk in 0..COORDINATES {
            let ap = t.row_coefficients[chunk * 256 + usize::from(a[chunk])].as_ptr();
            let bp = t.row_coefficients[chunk * 256 + usize::from(b[chunk])].as_ptr();
            a0 = _mm512_xor_si512(a0, _mm512_loadu_si512(ap.cast()));
            b0 = _mm512_xor_si512(b0, _mm512_loadu_si512(bp.cast()));
            a1 = _mm512_xor_si512(a1, _mm512_loadu_si512(ap.add(64).cast()));
            b1 = _mm512_xor_si512(b1, _mm512_loadu_si512(bp.add(64).cast()));
        }
        _mm512_storeu_si512(left.as_mut_ptr().cast(), a0);
        _mm512_storeu_si512(left.as_mut_ptr().add(64).cast(), a1);
        _mm512_storeu_si512(right.as_mut_ptr().cast(), b0);
        _mm512_storeu_si512(right.as_mut_ptr().add(64).cast(), b1);
    }
    (left, right)
}

#[cfg(target_arch = "x86_64")]
#[inline]
#[target_feature(enable = "gfni,avx512f,avx512bw")]
unsafe fn high_product_native(
    left: &[u8; KAPPA],
    right: &[u8; KAPPA + 64],
) -> [std::arch::x86_64::__m512i; 2] {
    use std::arch::x86_64::{
        _mm512_gf2p8mul_epi8, _mm512_loadu_si512, _mm512_set1_epi8, _mm512_setzero_si512,
        _mm512_xor_si512,
    };

    let mut high0 = _mm512_setzero_si512();
    let mut high1 = high0;
    // SAFETY: right[128..192] is zero padding, supplied by the only production
    // caller above. For i=1..127, the first read starts at 128-i in 1..127;
    // for i>64 the second starts at 192-i in 65..127. Both end before byte192.
    // Excess lanes multiply that padding and contribute zero, including the
    // reserved degree255 lane. No low-degree product is ever computed.
    unsafe {
        for (degree, &a) in left.iter().enumerate().skip(1) {
            if a != 0 {
                let scalar = _mm512_set1_epi8(a as i8);
                let first = _mm512_loadu_si512(right.as_ptr().add(KAPPA - degree).cast());
                high0 = _mm512_xor_si512(high0, _mm512_gf2p8mul_epi8(scalar, first));
                if degree > 64 {
                    let second = _mm512_loadu_si512(right.as_ptr().add(KAPPA + 64 - degree).cast());
                    high1 = _mm512_xor_si512(high1, _mm512_gf2p8mul_epi8(scalar, second));
                }
            }
        }
    }
    [high0, high1]
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "gfni,avx512f,avx512bw")]
unsafe fn native_block(a: &[u8], b: &[u8], weights: &[F128]) -> Vec<[u8; COORDINATES]> {
    use std::{arch::x86_64::__m512i, mem::MaybeUninit};

    const TILE_ROWS: usize = 64;
    let t = tables();
    let f = field();
    let mut coordinates = [[0; COORDINATES]; TILE_ROWS];
    let mut masks = [[0u64; 2]; TILE_ROWS];
    let mut products = [MaybeUninit::<[__m512i; 2]>::uninit(); TILE_ROWS];
    let mut sums = [[0u8; 256]; COORDINATES];
    for ((a, b), weights) in a
        .chunks(TILE_ROWS * COORDINATES)
        .zip(b.chunks(TILE_ROWS * COORDINATES))
        .zip(weights.chunks(TILE_ROWS))
    {
        let count = weights.len();
        f.decompose_block(weights, &mut coordinates[..count]);
        for (row, (a, b)) in a
            .chunks_exact(COORDINATES)
            .zip(b.chunks_exact(COORDINATES))
            .enumerate()
        {
            let a: &[u8; COORDINATES] = a.try_into().expect("complete binary row");
            let b: &[u8; COORDINATES] = b.try_into().expect("complete binary row");
            masks[row] = std::array::from_fn(|half| {
                let offset = half * 8;
                u64::from_le_bytes(a[offset..offset + 8].try_into().expect("complete half row"))
                    & u64::from_le_bytes(
                        b[offset..offset + 8].try_into().expect("complete half row"),
                    )
            });
            // SAFETY: all required features are declared; both rows are
            // complete and pair conversion supplies the convolution padding.
            unsafe {
                let (left, right) = coefficient_pair_native(t, a, b);
                products[row].write(high_product_native(&left, &right));
            }
        }
        // SAFETY: exactly count complete rows initialized both product vectors
        // above. MaybeUninit preserves its payload's layout and alignment.
        let products =
            unsafe { std::slice::from_raw_parts(products.as_ptr().cast::<[__m512i; 2]>(), count) };
        for coordinate in (0..COORDINATES).step_by(4) {
            // SAFETY: all tile lengths match, features are declared above, and
            // coordinate is 0,4,8,12. Reusing PairAggregate's exact register
            // consumer keeps weighted accumulation equal across experiments;
            // its two arbitrary product vectors can hold coefficients instead
            // of continuation samples without changing any field operations.
            unsafe {
                super::tiled::consume_four(
                    &mut sums,
                    products,
                    &coordinates[..count],
                    &masks[..count],
                    coordinate,
                )
            };
        }
    }
    // Padding degree255 was kept zero and is excluded from the certificate.
    (0..SAMPLES)
        .map(|sample| std::array::from_fn(|coordinate| sums[coordinate][sample]))
        .collect()
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "gfni,avx512f,avx512bw,vpclmulqdq")]
unsafe fn carryless_block(
    a: &[u8],
    b: &[u8],
    weights: &[F128],
    mode: encoding::Encoding,
) -> Vec<[u8; COORDINATES]> {
    use std::{
        arch::x86_64::{__m512i, _mm512_loadu_si512},
        mem::MaybeUninit,
    };

    const TILE_ROWS: usize = 64;
    let f = field();
    let mut coordinates = [[0; COORDINATES]; TILE_ROWS];
    let mut masks = [[0u64; 2]; TILE_ROWS];
    let mut products = [MaybeUninit::<[__m512i; 2]>::uninit(); TILE_ROWS];
    let mut sums = [[0u8; 256]; COORDINATES];
    for ((a, b), weights) in a
        .chunks(TILE_ROWS * COORDINATES)
        .zip(b.chunks(TILE_ROWS * COORDINATES))
        .zip(weights.chunks(TILE_ROWS))
    {
        let count = weights.len();
        f.decompose_block(weights, &mut coordinates[..count]);
        for (row, (a, b)) in a
            .chunks_exact(COORDINATES)
            .zip(b.chunks_exact(COORDINATES))
            .enumerate()
        {
            masks[row] = std::array::from_fn(|half| {
                let offset = half * 8;
                u64::from_le_bytes(a[offset..offset + 8].try_into().unwrap())
                    & u64::from_le_bytes(b[offset..offset + 8].try_into().unwrap())
            });
        }
        for first in (0..count).step_by(4) {
            let rows = (count - first).min(4);
            let offset = first * COORDINATES;
            let planes = if rows == 4 {
                encoding::encode_four(
                    a[offset..offset + 64].try_into().unwrap(),
                    b[offset..offset + 64].try_into().unwrap(),
                    mode,
                )
            } else {
                let mut padded = [[[0u128; 8]; 2]; 4];
                for (row, pair) in padded[..rows].iter_mut().enumerate() {
                    let at = offset + row * COORDINATES;
                    *pair = encoding::encode_pair(
                        a[at..at + 16].try_into().unwrap(),
                        b[at..at + 16].try_into().unwrap(),
                        mode,
                    );
                }
                padded
            };
            // SAFETY: features are declared and all four rows, including any
            // tail padding, contain eight initialized planes for both inputs.
            let high = unsafe { carryless::high_products(&planes) };
            for (row, planes) in high[..rows].iter().enumerate() {
                let bytes = encoding::to_bytes(planes);
                debug_assert_eq!(bytes[127], 0, "degree 255 cannot occur");
                // SAFETY: two 64-byte loads cover the initialized output exactly.
                products[first + row].write(unsafe {
                    [
                        _mm512_loadu_si512(bytes.as_ptr().cast()),
                        _mm512_loadu_si512(bytes.as_ptr().add(64).cast()),
                    ]
                });
            }
        }
        // SAFETY: exactly count rows were initialized above; no padded row is
        // exposed to the shared weighted consumer.
        let products =
            unsafe { std::slice::from_raw_parts(products.as_ptr().cast::<[__m512i; 2]>(), count) };
        for coordinate in (0..COORDINATES).step_by(4) {
            // SAFETY: checked features, complete matching slices, valid coordinate.
            unsafe {
                super::tiled::consume_four(
                    &mut sums,
                    products,
                    &coordinates[..count],
                    &masks[..count],
                    coordinate,
                )
            };
        }
    }
    (0..SAMPLES)
        .map(|sample| std::array::from_fn(|coordinate| sums[coordinate][sample]))
        .collect()
}

#[cfg(test)]
mod tests {
    use flock_field::{F8, F128, test_rng::Rng};

    use super::{
        BLOCK_ROWS, COORDINATES, KAPPA, QUOTIENTS, SAMPLES, aggregate, high_product, quotient,
        tables,
    };
    use crate::univariate::{Kernel, aggregate as sample_aggregate, domain::domain, field::field};

    fn check_against_samples(a: &[u8], b: &[u8], weights: &[F128]) {
        let samples = sample_aggregate(a, b, weights, Kernel::Lookup);
        let expected_quotient = domain().quotient(&samples);
        let scalar = aggregate(a, b, weights, false);
        assert_eq!(&scalar[..KAPPA], &samples[..KAPPA]);
        assert_eq!(quotient(&scalar), expected_quotient);
        assert_eq!(aggregate(a, b, weights, true), scalar);
        for kernel in [
            Kernel::CoefficientCarryless,
            Kernel::CoefficientCarrylessNibble,
        ] {
            assert_eq!(
                super::aggregate_with_kernel(a, b, weights, kernel),
                scalar,
                "coefficient aggregate differs for {kernel:?}"
            );
        }
        let paired = sample_aggregate(a, b, weights, Kernel::PairAggregate);
        assert_eq!(paired, samples);
        assert_eq!(domain().quotient(&paired), quotient(&scalar));
    }

    #[test]
    fn monomial_byte_tables_match_existing_domain_on_every_sample() {
        let mut rng = Rng::new(0x434F_4546_4631);
        let mut rows = vec![[0; COORDINATES], [255; COORDINATES]];
        for bit in 0..KAPPA {
            let mut row = [0; COORDINATES];
            row[bit / 8] = 1 << (bit % 8);
            rows.push(row);
        }
        for _ in 0..4 {
            let mut row = [0; COORDINATES];
            rng.fill_bytes(&mut row);
            rows.push(row);
        }
        for row in rows {
            let coefficients = tables().coefficients(&row);
            let expected = domain().eval_row(&row);
            for (point, sample) in expected.into_iter().enumerate() {
                let actual = coefficients
                    .iter()
                    .rev()
                    .fold(F8::ZERO, |value, &coefficient| {
                        value * F8(point as u8) + F8(coefficient)
                    });
                assert_eq!(actual.0, sample);
            }
        }
        let mut one = [0; KAPPA];
        one[0] = 1;
        assert_eq!(tables().coefficients(&[255; COORDINATES]), one);
    }

    #[test]
    fn triangular_schoolbook_matches_full_product_and_native_boundaries() {
        let mut rng = Rng::new(0x434F_4546_4632);
        let mut cases = vec![([0; KAPPA], [0; KAPPA])];
        for i in [0, 1, 63, 64, 65, 126, 127] {
            for j in [0, 1, 63, 64, 65, 126, 127] {
                let mut left = [0; KAPPA];
                let mut right = [0; KAPPA];
                left[i] = 197;
                right[j] = 231;
                cases.push((left, right));
            }
        }
        for _ in 0..8 {
            let mut left = [0; KAPPA];
            let mut right = [0; KAPPA];
            rng.fill_bytes(&mut left);
            rng.fill_bytes(&mut right);
            cases.push((left, right));
        }
        for (left, right) in cases {
            let mut full = [F8::ZERO; SAMPLES];
            for (i, &a) in left.iter().enumerate() {
                for (j, &b) in right.iter().enumerate() {
                    full[i + j] += F8(a) * F8(b);
                }
            }
            let high = high_product(&left, &right);
            assert_eq!(
                high.as_slice(),
                &full[KAPPA..].iter().map(|x| x.0).collect::<Vec<_>>()
            );
            #[cfg(target_arch = "x86_64")]
            if crate::univariate::has_gfni() {
                let mut padded = [0; KAPPA + 64];
                padded[..KAPPA].copy_from_slice(&right);
                // SAFETY: dispatch checks the required features. The output
                // is two vectors (128 initialized bytes), with identical size
                // to the byte array; right's required suffix padding is zero.
                let actual: [u8; KAPPA] =
                    unsafe { std::mem::transmute(super::high_product_native(&left, &padded)) };
                assert_eq!(&actual[..QUOTIENTS], &high);
                assert_eq!(actual[QUOTIENTS], 0, "degree255 must remain padding");
            }
        }
    }

    #[test]
    fn weighted_aggregate_matches_lookup_and_pair_quotients_at_worker_boundaries() {
        let mut rng = Rng::new(0x434F_4546_4633);
        for rows in [
            0,
            1,
            7,
            63,
            64,
            65,
            127,
            BLOCK_ROWS - 1,
            BLOCK_ROWS,
            BLOCK_ROWS + 1,
            2049,
        ] {
            let mut a = vec![0; rows * COORDINATES];
            let mut b = a.clone();
            rng.fill_bytes(&mut a);
            rng.fill_bytes(&mut b);
            check_against_samples(&a, &b, &rng.f128_vec(rows));
        }
    }

    #[test]
    fn zero_one_basis_and_square_rows_have_expected_high_coefficients() {
        let weight = F128::new(0x3456_789A_BCDE_F012, 0x1122_3344_5566_7788);
        for row in [[0; COORDINATES], [255; COORDINATES], [0xA5; COORDINATES]] {
            for one_side in [[0; COORDINATES], [255; COORDINATES]] {
                let output = aggregate(&row, &one_side, &[weight], false);
                assert!(output[KAPPA..].iter().all(|x| *x == [0; COORDINATES]));
                assert_eq!(quotient(&output), vec![F128::ZERO; QUOTIENTS]);
                check_against_samples(&row, &one_side, &[weight]);
            }
        }
        for bit in [0, 7, 8, 63, 64, 119, 127] {
            let mut row = [0; COORDINATES];
            row[bit / 8] = 1 << (bit % 8);
            let output = aggregate(&row, &row, &[weight], false);
            for (index, claim) in output[..KAPPA].iter().enumerate() {
                assert_eq!(
                    *claim,
                    if index == bit {
                        field().decompose(weight)
                    } else {
                        [0; COORDINATES]
                    }
                );
            }
            // In characteristic two, a polynomial square has no odd powers.
            assert!(
                output[KAPPA..]
                    .iter()
                    .enumerate()
                    .all(|(i, x)| i % 2 == 0 || *x == [0; COORDINATES])
            );
            check_against_samples(&row, &row, &[weight]);
        }
        check_against_samples(&[255; COORDINATES], &[255; COORDINATES], &[F128::ZERO]);
    }

    #[test]
    fn valid_binary_residual_has_zero_remainder_in_full_f8_division() {
        let mut rng = Rng::new(0x434F_4546_4635);
        let nodes: Vec<_> = (0..KAPPA).map(|i| i as u8).collect();
        let psi = crate::univariate::domain::vanishing_polynomial(&nodes);
        for _ in 0..4 {
            let mut a = [0; COORDINATES];
            let mut b = [0; COORDINATES];
            rng.fill_bytes(&mut a);
            rng.fill_bytes(&mut b);
            let c = std::array::from_fn(|i| a[i] & b[i]);
            let left = tables().coefficients(&a);
            let right = tables().coefficients(&b);
            let column = tables().coefficients(&c);
            let mut residual = [F8::ZERO; SAMPLES];
            for (i, &a) in left.iter().enumerate() {
                for (j, &b) in right.iter().enumerate() {
                    residual[i + j] += F8(a) * F8(b);
                }
            }
            for (output, value) in residual.iter_mut().zip(column) {
                *output += F8(value);
            }
            let weight = rng.f128();
            let mut expected = vec![F128::ZERO; QUOTIENTS];
            for degree in (KAPPA..SAMPLES).rev() {
                let coefficient = residual[degree];
                expected[degree - KAPPA] = weight * field().embed(coefficient.0);
                for (i, &divisor) in psi.iter().enumerate() {
                    residual[degree - KAPPA + i] += coefficient * F8(divisor);
                }
            }
            assert!(residual.iter().all(|value| *value == F8::ZERO));
            assert_eq!(quotient(&aggregate(&a, &b, &[weight], false)), expected);
            assert_eq!(quotient(&aggregate(&a, &b, &[weight], true)), expected);
        }
    }

    #[test]
    fn sparse_quotient_matches_full_extension_long_division() {
        let mut rng = Rng::new(0x434F_4546_4634);
        let nodes: Vec<_> = (0..KAPPA).map(|i| i as u8).collect();
        let psi = crate::univariate::domain::vanishing_polynomial(&nodes);
        let f = field();
        for _ in 0..4 {
            let high = rng.f128_vec(QUOTIENTS);
            let mut encoded = vec![[0; COORDINATES]; SAMPLES];
            for (slot, value) in encoded[KAPPA..].iter_mut().zip(&high) {
                *slot = f.decompose(*value);
            }
            let mut residual = rng.f128_vec(KAPPA);
            residual.extend(high);
            let mut expected = vec![F128::ZERO; QUOTIENTS];
            for degree in (KAPPA..SAMPLES).rev() {
                let coefficient = residual[degree];
                expected[degree - KAPPA] = coefficient;
                for (i, &divisor) in psi.iter().enumerate() {
                    residual[degree - KAPPA + i] += coefficient * f.embed(divisor);
                }
            }
            assert_eq!(quotient(&encoded), expected);
            // Changing the low column data cannot change monic division's
            // quotient; the protocol, not this helper, checks the remainder.
            for slot in &mut encoded[..KAPPA] {
                rng.fill_bytes(slot);
            }
            assert_eq!(quotient(&encoded), expected);
        }
    }

    #[test]
    #[should_panic(expected = "incomplete binary rows")]
    fn incomplete_rows_are_rejected() {
        aggregate(&[0; 15], &[0; 15], &[F128::ONE], false);
    }

    #[test]
    #[should_panic(expected = "wrong coefficient aggregate length")]
    fn truncated_quotient_input_is_rejected() {
        quotient(&[[0; COORDINATES]; SAMPLES - 1]);
    }
}
