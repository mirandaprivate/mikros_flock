//! Register-tiled GF256 aggregation, with exactly the v5 polynomial samples.

use super::{SAMPLES, domain::domain, field::field};
use flock_field::F128;
use std::{
    arch::x86_64::{
        __m512i, _mm512_gf2p8mul_epi8, _mm512_loadu_si512, _mm512_maskz_mov_epi8, _mm512_set1_epi8,
        _mm512_storeu_si512, _mm512_xor_si512,
    },
    mem::MaybeUninit,
};

const ROWS: usize = 64;

mod affine_h;
mod eight;

#[derive(Clone, Copy)]
pub(super) enum BasisConversion {
    Lookup,
    Batch,
    Register,
}

/// Split H-bit and continuation accumulation into eight-coordinate consumers.
///
/// # Safety
/// Requires GFNI, AVX512F/BW and equally sized complete input rows/weights.
#[target_feature(enable = "gfni,avx512f,avx512bw")]
pub(super) unsafe fn aggregate_block_split8(
    a: &[u8],
    b: &[u8],
    l: &[F128],
    conversion: BasisConversion,
) -> Vec<[u8; 16]> {
    // SAFETY: this entry has the same ISA and input contract as its callee.
    unsafe { eight::aggregate_block::<0>(a, b, l, conversion) }
}

/// Accumulate one H/continuation half at a time over eight coordinates.
///
/// # Safety
/// Requires GFNI, AVX512F/BW and equally sized complete input rows/weights.
#[target_feature(enable = "gfni,avx512f,avx512bw")]
pub(super) unsafe fn aggregate_block_half8(
    a: &[u8],
    b: &[u8],
    l: &[F128],
    conversion: BasisConversion,
) -> Vec<[u8; 16]> {
    // SAFETY: this entry has the same ISA and input contract as its callee.
    unsafe { eight::aggregate_block::<1>(a, b, l, conversion) }
}

/// Generate the two witnesses' continuations together; keep consume_four.
///
/// # Safety
/// Requires GFNI, AVX512F/BW and equally sized complete input rows/weights.
#[target_feature(enable = "gfni,avx512f,avx512bw")]
pub(super) unsafe fn aggregate_block_pair(
    a: &[u8],
    b: &[u8],
    l: &[F128],
    conversion: BasisConversion,
) -> Vec<[u8; 16]> {
    // SAFETY: this entry has the same ISA and input contract as its callee.
    unsafe { eight::aggregate_block::<2>(a, b, l, conversion) }
}

/// Replace split8's H consumer with GFNI affine maps; keep continuation8.
///
/// # Safety
/// Requires GFNI, AVX512F/BW and equally sized complete input rows/weights.
#[target_feature(enable = "gfni,avx512f,avx512bw")]
pub(super) unsafe fn aggregate_block_affine(
    a: &[u8],
    b: &[u8],
    l: &[F128],
    conversion: BasisConversion,
) -> Vec<[u8; 16]> {
    // SAFETY: this entry has the same ISA and input contract as its callee.
    unsafe { eight::aggregate_block::<3>(a, b, l, conversion) }
}

/// Combine pair continuation production with affine H and continuation8.
///
/// # Safety
/// Requires GFNI, AVX512F/BW and equally sized complete input rows/weights.
#[target_feature(enable = "gfni,avx512f,avx512bw")]
pub(super) unsafe fn aggregate_block_joint(
    a: &[u8],
    b: &[u8],
    l: &[F128],
    conversion: BasisConversion,
) -> Vec<[u8; 16]> {
    // SAFETY: this entry has the same ISA and input contract as its callee.
    unsafe { eight::aggregate_block::<4>(a, b, l, conversion) }
}

/// # Safety
/// Requires GFNI, AVX512F/BW and equally sized complete input rows/weights.
#[target_feature(enable = "gfni,avx512f,avx512bw")]
pub(super) unsafe fn aggregate_block(
    a: &[u8],
    b: &[u8],
    l: &[F128],
    conversion: BasisConversion,
) -> Vec<[u8; 16]> {
    let domain = domain();
    let field = field();
    let mut sums = [[0u8; 256]; 16];
    let mut coordinates = [[0u8; 16]; ROWS];
    let mut masks = [[0u64; 2]; ROWS];
    let mut products = [MaybeUninit::<[__m512i; 2]>::uninit(); ROWS];
    for ((a, b), weights) in a
        .chunks(ROWS * 16)
        .zip(b.chunks(ROWS * 16))
        .zip(l.chunks(ROWS))
    {
        let count = weights.len();
        match conversion {
            BasisConversion::Batch => {
                field.decompose_block(weights, &mut coordinates[..count]);
            }
            BasisConversion::Register => {
                field.decompose_block_register(weights, &mut coordinates[..count]);
            }
            BasisConversion::Lookup => {
                for (out, &weight) in coordinates.iter_mut().zip(weights) {
                    *out = field.decompose(weight);
                }
            }
        }
        for (row, (a, b)) in a.chunks_exact(16).zip(b.chunks_exact(16)).enumerate() {
            masks[row] = std::array::from_fn(|half| {
                let offset = half * 8;
                u64::from_le_bytes(a[offset..offset + 8].try_into().unwrap())
                    & u64::from_le_bytes(b[offset..offset + 8].try_into().unwrap())
            });
            // SAFETY: complete 16-byte rows; the function enables the ISA.
            // Each row gets both initialized products before consumption.
            unsafe {
                let left = domain.continuation_registers(a.try_into().unwrap());
                let right = domain.continuation_registers(b.try_into().unwrap());
                products[row].write([
                    _mm512_gf2p8mul_epi8(left[0], right[0]),
                    _mm512_gf2p8mul_epi8(left[1], right[1]),
                ]);
            }
        }
        // SAFETY: exactly count complete rows above initialized both vector
        // products; MaybeUninit has the same alignment/layout as its payload.
        let products =
            unsafe { std::slice::from_raw_parts(products.as_ptr().cast::<[__m512i; 2]>(), count) };
        for coordinate in (0..16).step_by(4) {
            // SAFETY: matching initialized tile sizes and coordinate+4<=16.
            unsafe {
                consume_four(
                    &mut sums,
                    products,
                    &coordinates[..count],
                    &masks[..count],
                    coordinate,
                )
            };
        }
    }
    // The last continuation lane is point255, not part of our domain. It may
    // be computed speculatively; only samples0..254 enter the certificate.
    (0..SAMPLES)
        .map(|sample| std::array::from_fn(|coordinate| sums[coordinate][sample]))
        .collect()
}

// Keep the four-coordinate tile boundary visible to the register allocator.
// The row loop has 16 accumulator ZMMs, plus products/scalar/temporary registers.
#[inline(never)]
#[target_feature(enable = "gfni,avx512f,avx512bw")]
pub(super) unsafe fn consume_four(
    sums: &mut [[u8; 256]; 16],
    products: &[[__m512i; 2]],
    coordinates: &[[u8; 16]],
    masks: &[[u64; 2]],
    offset: usize,
) {
    // SAFETY: offsets0,64,128,192 are full vectors inside each coordinate.
    // Caller supplies offset in {0,4,8,12} and matching tile lengths.
    unsafe {
        let mut h0: [__m512i; 4] =
            std::array::from_fn(|c| _mm512_loadu_si512(sums[offset + c].as_ptr().cast()));
        let mut h1: [__m512i; 4] =
            std::array::from_fn(|c| _mm512_loadu_si512(sums[offset + c].as_ptr().add(64).cast()));
        let mut p0: [__m512i; 4] =
            std::array::from_fn(|c| _mm512_loadu_si512(sums[offset + c].as_ptr().add(128).cast()));
        let mut p1: [__m512i; 4] =
            std::array::from_fn(|c| _mm512_loadu_si512(sums[offset + c].as_ptr().add(192).cast()));
        for ((product, coordinates), mask) in products.iter().zip(coordinates).zip(masks) {
            macro_rules! update {
                ($c:literal) => {{
                    let scalar = _mm512_set1_epi8(coordinates[offset + $c] as i8);
                    h0[$c] = _mm512_xor_si512(h0[$c], _mm512_maskz_mov_epi8(mask[0], scalar));
                    h1[$c] = _mm512_xor_si512(h1[$c], _mm512_maskz_mov_epi8(mask[1], scalar));
                    p0[$c] = _mm512_xor_si512(p0[$c], _mm512_gf2p8mul_epi8(product[0], scalar));
                    p1[$c] = _mm512_xor_si512(p1[$c], _mm512_gf2p8mul_epi8(product[1], scalar));
                }};
            }
            update!(0);
            update!(1);
            update!(2);
            update!(3);
        }
        for c in 0..4 {
            _mm512_storeu_si512(sums[offset + c].as_mut_ptr().cast(), h0[c]);
            _mm512_storeu_si512(sums[offset + c].as_mut_ptr().add(64).cast(), h1[c]);
            _mm512_storeu_si512(sums[offset + c].as_mut_ptr().add(128).cast(), p0[c]);
            _mm512_storeu_si512(sums[offset + c].as_mut_ptr().add(192).cast(), p1[c]);
        }
    }
}
