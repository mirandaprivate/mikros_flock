//! Optional consumer schedules for the same coordinate/sample sums.
//!
//! Each eight-coordinate row consumer has sixteen ZMM accumulator values.
//! Split8 separates the two H vectors from the two continuation vectors;
//! Half8 pairs H and continuation
//! for one half. Both consume eight coordinates, reducing product/mask rescans
//! at the cost of broadcasting the coordinates twice. This is a scheduling
//! tradeoff to measure, not an assumed speedup. The original four-coordinate
//! implementation remains intact in the parent module as an ablation baseline.
//! Mode 2 changes only the continuation producer, retaining consume_four;
//! mode 3 replaces only Split8's H consumer with an affine GFNI consumer.
//! Mode 4 combines the pair producer and affine H with continuation8.

use super::{BasisConversion, ROWS, SAMPLES, domain, field};
use flock_field::F128;
use std::{
    arch::x86_64::{
        __m512i, _mm512_gf2p8mul_epi8, _mm512_loadu_si512, _mm512_maskz_mov_epi8, _mm512_set1_epi8,
        _mm512_storeu_si512, _mm512_xor_si512,
    },
    mem::MaybeUninit,
};

/// # Safety
/// Requires GFNI, AVX512F/BW, with a.len() == b.len() == l.len()*16.
#[target_feature(enable = "gfni,avx512f,avx512bw")]
pub(super) unsafe fn aggregate_block<const MODE: u8>(
    a: &[u8],
    b: &[u8],
    l: &[F128],
    conversion: BasisConversion,
) -> Vec<[u8; 16]> {
    debug_assert_eq!(a.len(), b.len());
    debug_assert_eq!(a.len(), l.len() * 16);
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
            // SAFETY: both witnesses contain count complete rows, and this
            // function enables the required ISA. Each product is initialized.
            unsafe {
                let [left, right] = if matches!(MODE, 2 | 4) {
                    domain.continuation_pair_registers(a.try_into().unwrap(), b.try_into().unwrap())
                } else {
                    [
                        domain.continuation_registers(a.try_into().unwrap()),
                        domain.continuation_registers(b.try_into().unwrap()),
                    ]
                };
                products[row].write([
                    _mm512_gf2p8mul_epi8(left[0], right[0]),
                    _mm512_gf2p8mul_epi8(left[1], right[1]),
                ]);
            }
        }
        // SAFETY: only the count initialized rows are exposed, including the
        // last partial tile. No consumer can observe the unused suffix.
        let products =
            unsafe { std::slice::from_raw_parts(products.as_ptr().cast::<[__m512i; 2]>(), count) };
        if matches!(MODE, 3 | 4) {
            // SAFETY: both initialized arrays have exactly count valid rows.
            unsafe {
                super::affine_h::consume_h_affine(
                    &mut sums,
                    &coordinates[..count],
                    &masks[..count],
                );
            }
        }
        for offset in [0, 8] {
            // SAFETY: offset+8<=16; every input has the same initialized count.
            unsafe {
                match MODE {
                    0 => {
                        consume_h_eight(&mut sums, &coordinates[..count], &masks[..count], offset);
                        consume_p_eight(&mut sums, products, &coordinates[..count], offset);
                    }
                    1 => {
                        consume_half_eight::<0>(
                            &mut sums,
                            products,
                            &coordinates[..count],
                            &masks[..count],
                            offset,
                        );
                        consume_half_eight::<1>(
                            &mut sums,
                            products,
                            &coordinates[..count],
                            &masks[..count],
                            offset,
                        );
                    }
                    2 => {
                        for offset in [offset, offset + 4] {
                            super::consume_four(
                                &mut sums,
                                products,
                                &coordinates[..count],
                                &masks[..count],
                                offset,
                            );
                        }
                    }
                    3 | 4 => consume_p_eight(&mut sums, products, &coordinates[..count], offset),
                    _ => unreachable!("unknown optional aggregation schedule"),
                }
            }
        }
    }
    // Sample 255 is the speculative final continuation lane, excluded from
    // the certificate exactly as in the original aggregation kernel.
    (0..SAMPLES)
        .map(|sample| std::array::from_fn(|coordinate| sums[coordinate][sample]))
        .collect()
}

/// # Safety
/// Requires the declared ISA, offset in {0,8}, and matching input lengths.
#[inline(never)]
#[target_feature(enable = "gfni,avx512f,avx512bw")]
unsafe fn consume_h_eight(
    sums: &mut [[u8; 256]; 16],
    coordinates: &[[u8; 16]],
    masks: &[[u64; 2]],
    offset: usize,
) {
    // SAFETY: each selected coordinate owns two full 64-byte H vectors.
    unsafe {
        let mut h0: [__m512i; 8] =
            std::array::from_fn(|c| _mm512_loadu_si512(sums[offset + c].as_ptr().cast()));
        let mut h1: [__m512i; 8] =
            std::array::from_fn(|c| _mm512_loadu_si512(sums[offset + c].as_ptr().add(64).cast()));
        for (coordinates, mask) in coordinates.iter().zip(masks) {
            let coordinates = coordinates.as_ptr().add(offset);
            macro_rules! update {
                ($c:literal) => {{
                    let scalar = _mm512_set1_epi8(*coordinates.add($c) as i8);
                    h0[$c] = _mm512_xor_si512(h0[$c], _mm512_maskz_mov_epi8(mask[0], scalar));
                    h1[$c] = _mm512_xor_si512(h1[$c], _mm512_maskz_mov_epi8(mask[1], scalar));
                }};
            }
            update!(0);
            update!(1);
            update!(2);
            update!(3);
            update!(4);
            update!(5);
            update!(6);
            update!(7);
        }
        for c in 0..8 {
            _mm512_storeu_si512(sums[offset + c].as_mut_ptr().cast(), h0[c]);
            _mm512_storeu_si512(sums[offset + c].as_mut_ptr().add(64).cast(), h1[c]);
        }
    }
}

/// # Safety
/// Requires the declared ISA, offset in {0,8}, and matching input lengths.
#[inline(never)]
#[target_feature(enable = "gfni,avx512f,avx512bw")]
unsafe fn consume_p_eight(
    sums: &mut [[u8; 256]; 16],
    products: &[[__m512i; 2]],
    coordinates: &[[u8; 16]],
    offset: usize,
) {
    // SAFETY: each coordinate owns two full 64-byte continuation vectors.
    unsafe {
        let mut p0: [__m512i; 8] =
            std::array::from_fn(|c| _mm512_loadu_si512(sums[offset + c].as_ptr().add(128).cast()));
        let mut p1: [__m512i; 8] =
            std::array::from_fn(|c| _mm512_loadu_si512(sums[offset + c].as_ptr().add(192).cast()));
        for (product, coordinates) in products.iter().zip(coordinates) {
            let coordinates = coordinates.as_ptr().add(offset);
            macro_rules! update {
                ($c:literal) => {{
                    let scalar = _mm512_set1_epi8(*coordinates.add($c) as i8);
                    p0[$c] = _mm512_xor_si512(p0[$c], _mm512_gf2p8mul_epi8(product[0], scalar));
                    p1[$c] = _mm512_xor_si512(p1[$c], _mm512_gf2p8mul_epi8(product[1], scalar));
                }};
            }
            update!(0);
            update!(1);
            update!(2);
            update!(3);
            update!(4);
            update!(5);
            update!(6);
            update!(7);
        }
        for c in 0..8 {
            _mm512_storeu_si512(sums[offset + c].as_mut_ptr().add(128).cast(), p0[c]);
            _mm512_storeu_si512(sums[offset + c].as_mut_ptr().add(192).cast(), p1[c]);
        }
    }
}

/// # Safety
/// Requires the declared ISA, HALF in {0,1}, offset in {0,8}, equal lengths.
#[inline(never)]
#[target_feature(enable = "gfni,avx512f,avx512bw")]
unsafe fn consume_half_eight<const HALF: usize>(
    sums: &mut [[u8; 256]; 16],
    products: &[[__m512i; 2]],
    coordinates: &[[u8; 16]],
    masks: &[[u64; 2]],
    offset: usize,
) {
    // SAFETY: HALF is fixed to 0 or 1 by the only callers; all vector ranges
    // stay inside the selected coordinate. The eight scalar reads are valid.
    unsafe {
        let mut h: [__m512i; 8] = std::array::from_fn(|c| {
            _mm512_loadu_si512(sums[offset + c].as_ptr().add(HALF * 64).cast())
        });
        let mut p: [__m512i; 8] = std::array::from_fn(|c| {
            _mm512_loadu_si512(sums[offset + c].as_ptr().add(128 + HALF * 64).cast())
        });
        for ((product, coordinates), mask) in products.iter().zip(coordinates).zip(masks) {
            let coordinates = coordinates.as_ptr().add(offset);
            macro_rules! update {
                ($c:literal) => {{
                    let scalar = _mm512_set1_epi8(*coordinates.add($c) as i8);
                    h[$c] = _mm512_xor_si512(h[$c], _mm512_maskz_mov_epi8(mask[HALF], scalar));
                    p[$c] = _mm512_xor_si512(p[$c], _mm512_gf2p8mul_epi8(product[HALF], scalar));
                }};
            }
            update!(0);
            update!(1);
            update!(2);
            update!(3);
            update!(4);
            update!(5);
            update!(6);
            update!(7);
        }
        for c in 0..8 {
            _mm512_storeu_si512(sums[offset + c].as_mut_ptr().add(HALF * 64).cast(), h[c]);
            _mm512_storeu_si512(
                sums[offset + c].as_mut_ptr().add(128 + HALF * 64).cast(),
                p[c],
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BasisConversion, aggregate_block, consume_h_eight, consume_half_eight, consume_p_eight,
        field,
    };
    use flock_field::{F128, test_rng::Rng};
    use std::arch::x86_64::{__m512i, _mm512_loadu_si512};

    #[test]
    fn aggregate_eight_consumers_match_scalar_all_lanes_and_partial_tiles() {
        if !super::super::super::has_gfni() {
            return;
        }
        let mut rng = Rng::new(0x8C00_255A_256B);
        for rows in [0, 1, 7, 63, 64] {
            let bytes: Vec<[[u8; 64]; 2]> = (0..rows)
                .map(|row| {
                    std::array::from_fn(|half| {
                        std::array::from_fn(|lane| (row * 17 + half * 64 + lane) as u8)
                    })
                })
                .collect();
            // SAFETY: each input array contains two full vectors; ISA checked.
            let products: Vec<[__m512i; 2]> = bytes
                .iter()
                .map(|row| unsafe { row.map(|half| _mm512_loadu_si512(half.as_ptr().cast())) })
                .collect();
            let coordinates: Vec<[u8; 16]> = (0..rows)
                .map(|row| std::array::from_fn(|c| (row * 4 + c) as u8))
                .collect();
            let masks: Vec<[u64; 2]> = (0..rows)
                .map(|row| match row % 4 {
                    0 => [0, u64::MAX],
                    1 => [1 << (row % 64), 1 << (63 - row % 64)],
                    _ => [rng.next_u64(), rng.next_u64()],
                })
                .collect();
            let mut initial = [[0u8; 256]; 16];
            for coordinate in &mut initial {
                rng.fill_bytes(coordinate);
            }
            let mut expected = initial;
            for row in 0..rows {
                for c in 0..16 {
                    for half in 0..2 {
                        for lane in 0..64 {
                            if masks[row][half] >> lane & 1 != 0 {
                                expected[c][half * 64 + lane] ^= coordinates[row][c];
                            }
                            expected[c][128 + half * 64 + lane] ^=
                                field().mul(coordinates[row][c], bytes[row][half][lane]);
                        }
                    }
                }
            }
            let mut split = initial;
            let mut half = initial;
            for offset in [0, 8] {
                // SAFETY: valid offsets/ISA and equally sized input arrays.
                unsafe {
                    consume_h_eight(&mut split, &coordinates, &masks, offset);
                    consume_p_eight(&mut split, &products, &coordinates, offset);
                    consume_half_eight::<0>(&mut half, &products, &coordinates, &masks, offset);
                    consume_half_eight::<1>(&mut half, &products, &coordinates, &masks, offset);
                }
            }
            assert_eq!(split, expected, "split8, rows={rows}");
            assert_eq!(half, expected, "half8, rows={rows}");
        }
    }

    #[test]
    fn aggregate_eight_schedules_match_reference_across_row_boundaries() {
        if !super::super::super::has_gfni() {
            return;
        }
        let mut rng = Rng::new(0x8A66_0064_2049);
        for rows in [0, 1, 7, 8, 63, 64, 65, 127, 128, 129, 2047, 2048, 2049] {
            // Offset both witnesses by a byte so native loads cannot depend on
            // witness alignment; weights include nonuniform zero/one values.
            let mut a = vec![0; rows * 16 + 1];
            let mut b = a.clone();
            rng.fill_bytes(&mut a);
            rng.fill_bytes(&mut b);
            let mut weights = rng.f128_vec(rows);
            for (row, weight) in weights.iter_mut().enumerate() {
                if row % 17 == 16 {
                    *weight = F128::ZERO;
                } else if row % 19 == 18 {
                    *weight = F128::ONE;
                }
            }
            let a = &a[1..];
            let b = &b[1..];
            let expected = super::super::super::reference_block(a, b, &weights);
            for conversion in [
                BasisConversion::Lookup,
                BasisConversion::Batch,
                BasisConversion::Register,
            ] {
                // SAFETY: ISA checked and complete rows with matching weights.
                unsafe {
                    assert_eq!(
                        aggregate_block::<0>(a, b, &weights, conversion),
                        expected,
                        "split8, rows={rows}"
                    );
                    assert_eq!(
                        aggregate_block::<1>(a, b, &weights, conversion),
                        expected,
                        "half8, rows={rows}"
                    );
                    assert_eq!(
                        aggregate_block::<2>(a, b, &weights, conversion),
                        expected,
                        "pair continuation, rows={rows}"
                    );
                    assert_eq!(
                        aggregate_block::<3>(a, b, &weights, conversion),
                        expected,
                        "affine H, rows={rows}"
                    );
                    assert_eq!(
                        super::super::aggregate_block_joint(a, b, &weights, conversion),
                        expected,
                        "pair continuation + affine H, rows={rows}"
                    );
                    assert_eq!(
                        super::super::aggregate_block(a, b, &weights, conversion),
                        expected,
                        "four-coordinate baseline, rows={rows}"
                    );
                }
            }
        }
    }
}
