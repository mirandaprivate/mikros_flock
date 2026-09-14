//! GF(256) interpolation and coordinate aggregation for the univariate proof.
//!
//! H = {0,...,127}, S = {0,...,254}, with bytes interpreted in Flock's AES
//! GF(256) representation. E remains the existing GHASH GF(2^128).

pub(crate) mod coefficient;
pub(crate) mod domain;
pub(crate) mod field;

use flock_field::F128;
use rayon::prelude::{IndexedParallelIterator, ParallelIterator, ParallelSlice};

#[cfg(test)]
mod tests {
    use super::{Kernel, aggregate, has_gfni};
    use flock_field::test_rng::Rng;

    #[test]
    fn hardware_register_tiles_match_lookup_at_partial_tiles_and_workers() {
        if !has_gfni() {
            return;
        }
        let mut rng = Rng::new(0x716E_D001);
        for rows in [1, 7, 63, 64, 65, 127, 2047, 2048, 2049] {
            let mut a = vec![0; rows * 16];
            let mut b = a.clone();
            rng.fill_bytes(&mut a);
            rng.fill_bytes(&mut b);
            let weights = rng.f128_vec(rows);
            let expected = aggregate(&a, &b, &weights, Kernel::Lookup);
            for kernel in [
                Kernel::Native,
                Kernel::Tiled,
                Kernel::TiledBasis,
                Kernel::RegisterBasis,
                Kernel::RegisterPacked,
                Kernel::SplitAggregate,
                Kernel::HalfAggregate,
                Kernel::PairAggregate,
                Kernel::AffineAggregate,
                Kernel::JointAggregate,
            ] {
                assert_eq!(
                    aggregate(&a, &b, &weights, kernel),
                    expected,
                    "rows={rows}, kernel={kernel:?}"
                );
            }
        }
    }
}

pub const KAPPA: usize = 128;
pub const SAMPLES: usize = 255;
pub const QUOTIENTS: usize = 127;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kernel {
    /// Independent accumulation using full extension-field multiplication.
    Reference,
    /// Byte-block interpolation and an explicit 64 KiB GF(256) product table.
    Lookup,
    /// Flock's compressed interpolation table and GFNI/AVX-512 when available.
    /// Falls back to table multiplication without changing the protocol.
    Native,
    /// Cache 64 rows and keep four coordinates' accumulators in registers.
    Tiled,
    /// Tiled aggregation plus a batched GFNI field-basis conversion.
    TiledBasis,
    /// Tiled aggregation with a GFNI conversion avoiding intermediate zeroing.
    RegisterBasis,
    /// RegisterBasis aggregation and deferred wide IP-message folding in packing.
    RegisterPacked,
    /// Batch conversion with separate eight-coordinate H and continuation loops.
    SplitAggregate,
    /// Batch conversion with eight-coordinate loops over each sample half.
    HalfAggregate,
    /// Batch conversion and fixed-index, paired continuation interpolation.
    PairAggregate,
    /// Split aggregation with eight-row GFNI affine accumulation of H.
    AffineAggregate,
    /// Combine fixed paired interpolation with the affine H consumer.
    JointAggregate,
    /// Experimental monomial high-product convolution using the GF(256) table.
    CoefficientLookup,
    /// The same coefficient construction with GFNI/AVX-512 schoolbook products.
    CoefficientNative,
    /// Four-row carryless high convolution, with a byte-to-bitplanes table.
    CoefficientCarryless,
    /// Carryless convolution with the smaller nibble-to-bitplanes table.
    CoefficientCarrylessNibble,
}

impl Kernel {
    pub(crate) fn is_coefficient(self) -> bool {
        matches!(
            self,
            Self::CoefficientLookup
                | Self::CoefficientNative
                | Self::CoefficientCarryless
                | Self::CoefficientCarrylessNibble
        )
    }
}

/// Backend for the experimental bitplane convolution, including safe fallbacks.
pub fn carryless_backend() -> &'static str {
    #[cfg(target_arch = "x86_64")]
    if has_gfni() && std::is_x86_feature_detected!("vpclmulqdq") {
        return "vpclmul-avx512";
    }
    native_backend()
}

/// Which backend the current host can execute (independent of proof data).
pub fn native_backend() -> &'static str {
    if has_gfni() {
        "gfni-avx512"
    } else {
        "lookup-fallback"
    }
}

fn has_gfni() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        std::is_x86_feature_detected!("gfni")
            && std::is_x86_feature_detected!("avx512f")
            && std::is_x86_feature_detected!("avx512bw")
    }
    #[cfg(not(target_arch = "x86_64"))]
    false
}

/// Compute U = L^T ((AP) o (BP)); no expanded witness matrix is allocated.
/// Each worker keeps O(kappa) coordinates, independent of its row count.
pub(crate) fn aggregate(a: &[u8], b: &[u8], weights: &[F128], kernel: Kernel) -> Vec<[u8; 16]> {
    assert!(
        !kernel.is_coefficient(),
        "coefficient kernels produce high coefficients, not point evaluations"
    );
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), weights.len() * 16);
    // Build public tables before parallel workers request them.
    let _ = domain::domain();
    let _ = field::field();
    let block_rows = 2048;
    a.par_chunks(block_rows * 16)
        .zip(b.par_chunks(block_rows * 16))
        .zip(weights.par_chunks(block_rows))
        .map(|((a, b), l)| {
            if kernel == Kernel::Reference {
                reference_block(a, b, l)
            } else {
                #[cfg(target_arch = "x86_64")]
                if matches!(
                    kernel,
                    Kernel::Native
                        | Kernel::Tiled
                        | Kernel::TiledBasis
                        | Kernel::RegisterBasis
                        | Kernel::RegisterPacked
                        | Kernel::SplitAggregate
                        | Kernel::HalfAggregate
                        | Kernel::PairAggregate
                        | Kernel::AffineAggregate
                        | Kernel::JointAggregate
                ) && has_gfni()
                {
                    // SAFETY: all required ISA features checked above. Blocks
                    // contain complete rows and corresponding weights.
                    return unsafe {
                        match kernel {
                            Kernel::Native => native_block(a, b, l),
                            Kernel::Tiled => {
                                tiled::aggregate_block(a, b, l, tiled::BasisConversion::Lookup)
                            }
                            Kernel::TiledBasis => {
                                tiled::aggregate_block(a, b, l, tiled::BasisConversion::Batch)
                            }
                            Kernel::RegisterBasis | Kernel::RegisterPacked => {
                                tiled::aggregate_block(a, b, l, tiled::BasisConversion::Register)
                            }
                            Kernel::SplitAggregate => tiled::aggregate_block_split8(
                                a,
                                b,
                                l,
                                tiled::BasisConversion::Batch,
                            ),
                            Kernel::HalfAggregate => {
                                tiled::aggregate_block_half8(a, b, l, tiled::BasisConversion::Batch)
                            }
                            Kernel::PairAggregate => {
                                tiled::aggregate_block_pair(a, b, l, tiled::BasisConversion::Batch)
                            }
                            Kernel::AffineAggregate => tiled::aggregate_block_affine(
                                a,
                                b,
                                l,
                                tiled::BasisConversion::Batch,
                            ),
                            Kernel::JointAggregate => {
                                tiled::aggregate_block_joint(a, b, l, tiled::BasisConversion::Batch)
                            }
                            _ => unreachable!(),
                        }
                    };
                }
                lookup_block(a, b, l, kernel != Kernel::Lookup)
            }
        })
        .reduce(
            || vec![[0; 16]; SAMPLES],
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

#[cfg(target_arch = "x86_64")]
mod tiled;

fn reference_block(a: &[u8], b: &[u8], l: &[F128]) -> Vec<[u8; 16]> {
    let field = field::field();
    let domain = domain::domain();
    let mut sums = vec![F128::ZERO; SAMPLES];
    for ((a, b), &weight) in a.chunks_exact(16).zip(b.chunks_exact(16)).zip(l) {
        let left = domain.eval_row(a.try_into().unwrap());
        let right = domain.eval_row(b.try_into().unwrap());
        for ((sum, left), right) in sums.iter_mut().zip(left).zip(right) {
            *sum += weight * field.embed(left) * field.embed(right);
        }
    }
    sums.into_iter().map(|x| field.decompose(x)).collect()
}

fn lookup_block(a: &[u8], b: &[u8], l: &[F128], native_lde: bool) -> Vec<[u8; 16]> {
    let field = field::field();
    let domain = domain::domain();
    let mut sums = vec![0u128; SAMPLES];
    let mut products = [0u128; 256];
    for ((a, b), &weight) in a.chunks_exact(16).zip(b.chunks_exact(16)).zip(l) {
        let evaluate = |row| {
            if native_lde {
                domain.eval_row_native(row)
            } else {
                domain.eval_row(row)
            }
        };
        let left = evaluate(a.try_into().unwrap());
        let right = evaluate(b.try_into().unwrap());
        let coordinates = field.decompose(weight);
        // Tabulate the K-linear map z -> z*l once for this row. It uses eight
        // table products per coordinate; remaining entries need only XORs.
        let basis: [u128; 8] = std::array::from_fn(|bit| {
            u128::from_le_bytes(coordinates.map(|x| field.mul(x, 1 << bit)))
        });
        for value in 1usize..256 {
            products[value] = products[value & (value - 1)];
            products[value] ^= basis[value.trailing_zeros() as usize];
        }
        for ((sum, x), y) in sums.iter_mut().zip(left).zip(right) {
            *sum ^= products[field.mul(x, y) as usize];
        }
    }
    sums.into_iter().map(u128::to_le_bytes).collect()
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "gfni,avx512f,avx512bw")]
unsafe fn native_block(a: &[u8], b: &[u8], l: &[F128]) -> Vec<[u8; 16]> {
    use std::arch::x86_64::{
        __m512i, _mm512_gf2p8mul_epi8, _mm512_loadu_si512, _mm512_maskz_mov_epi8, _mm512_set1_epi8,
        _mm512_storeu_si512, _mm512_xor_si512,
    };
    let field = field::field();
    let domain = domain::domain();
    // Coordinate-major so each GFNI instruction multiplies 64 samples by
    // one coordinate of l. The final padding lane is never interpolated.
    let mut sums = [[0u8; 256]; 16];
    for ((a, b), &weight) in a.chunks_exact(16).zip(b.chunks_exact(16)).zip(l) {
        let mut left = [0; 256];
        let mut right = [0; 256];
        left[..SAMPLES].copy_from_slice(&domain.eval_row_native(a.try_into().unwrap()));
        right[..SAMPLES].copy_from_slice(&domain.eval_row_native(b.try_into().unwrap()));
        let coordinates = field.decompose(weight);
        let masks: [u64; 2] = std::array::from_fn(|half| {
            let offset = half * 8;
            u64::from_le_bytes(a[offset..offset + 8].try_into().unwrap())
                & u64::from_le_bytes(b[offset..offset + 8].try_into().unwrap())
        });
        // SAFETY: each load/store stays within a 256-byte initialized array,
        // with offsets 0,64,128,192. Required features are function attributes.
        unsafe {
            let products: [__m512i; 2] = std::array::from_fn(|chunk| {
                _mm512_gf2p8mul_epi8(
                    _mm512_loadu_si512(left.as_ptr().add(128 + 64 * chunk).cast()),
                    _mm512_loadu_si512(right.as_ptr().add(128 + 64 * chunk).cast()),
                )
            });
            for (sum, coordinate) in sums.iter_mut().zip(coordinates) {
                let scalar = _mm512_set1_epi8(coordinate as i8);
                // H samples are bits. Masked XOR replaces their GF256
                // products, retaining exactly the same column claims.
                for (half, mask) in masks.iter().enumerate() {
                    let ptr = sum.as_mut_ptr().add(64 * half).cast();
                    let old = _mm512_loadu_si512(ptr);
                    let value = _mm512_maskz_mov_epi8(*mask, scalar);
                    _mm512_storeu_si512(ptr, _mm512_xor_si512(old, value));
                }
                for (chunk, product) in products.iter().enumerate() {
                    let ptr = sum.as_mut_ptr().add(128 + 64 * chunk).cast();
                    let old = _mm512_loadu_si512(ptr);
                    let value = _mm512_gf2p8mul_epi8(*product, scalar);
                    _mm512_storeu_si512(ptr, _mm512_xor_si512(old, value));
                }
            }
        }
    }
    (0..SAMPLES)
        .map(|sample| std::array::from_fn(|coordinate| sums[coordinate][sample]))
        .collect()
}
