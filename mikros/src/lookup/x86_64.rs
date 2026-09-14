//! Field operations reuse Flock's native four-lane VPCLMUL kernels.

use core::{
    arch::x86_64::{
        __m512i, _mm512_permutex2var_epi64, _mm512_set_epi64, _mm512_storeu_si512, _mm512_xor_si512,
    },
    mem::MaybeUninit,
};
use flock_field::{
    F128, F256Unreduced,
    gf2_128::x86_64::{WideGhashX4, f128x4_loadu, f128x4_set, ghash_mul_x4},
};

use super::{BindingTable, ROW_BYTES};

/// Separate the four adjacent F128 pairs without splitting either u64 limb.
/// This is the same permutation used by the unchanged IP message kernel.
#[inline]
#[target_feature(enable = "avx512f")]
unsafe fn separate_pairs(lo: __m512i, hi: __m512i) -> (__m512i, __m512i) {
    let even = _mm512_set_epi64(13, 12, 9, 8, 5, 4, 1, 0);
    let odd = _mm512_set_epi64(15, 14, 11, 10, 7, 6, 3, 2);
    (
        _mm512_permutex2var_epi64(lo, even, hi),
        _mm512_permutex2var_epi64(lo, odd, hi),
    )
}

/// Bind both witnesses, weight B, and accumulate the first message without
/// horizontally folding the wide accumulators after every eight-row group.
///
/// # Safety
/// Requires AVX-512F/BW, VPCLMULQDQ, PCLMULQDQ, and SSE4.1.
#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq,pclmulqdq,sse4.1")]
pub(super) unsafe fn pack_pair_wide(
    table: &BindingTable,
    a: &[u8],
    b: &[u8],
    weights: &[F128],
    a_out: &mut [MaybeUninit<F128>],
    b_out: &mut [MaybeUninit<F128>],
) -> [F256Unreduced; 2] {
    assert_eq!(a.len(), weights.len() * ROW_BYTES);
    assert_eq!(b.len(), a.len());
    assert_eq!(a_out.len(), weights.len());
    assert_eq!(b_out.len(), weights.len());
    // SAFETY: each bulk iteration consumes eight complete rows, reads eight
    // weights, and initializes eight distinct outputs in each slice. Unaligned
    // loads/stores only require F128's usual alignment. All shorter tails use
    // scalar accesses and initialize the final odd row without pairing it.
    unsafe {
        let mut zero = WideGhashX4::zero();
        let mut infinity = WideGhashX4::zero();
        let bulk = weights.len() / 8 * 8;
        for offset in (0..bulk).step_by(8) {
            let a_values: [F128; 8] = std::array::from_fn(|i| {
                let start = (offset + i) * ROW_BYTES;
                table.fold(&a[start..start + ROW_BYTES])
            });
            let b_values: [F128; 8] = std::array::from_fn(|i| {
                let start = (offset + i) * ROW_BYTES;
                table.fold(&b[start..start + ROW_BYTES])
            });
            let al = f128x4_loadu(a_values.as_ptr());
            let ah = f128x4_loadu(a_values.as_ptr().add(4));
            let bl = ghash_mul_x4(
                f128x4_loadu(b_values.as_ptr()),
                f128x4_loadu(weights.as_ptr().add(offset)),
            );
            let bh = ghash_mul_x4(
                f128x4_loadu(b_values.as_ptr().add(4)),
                f128x4_loadu(weights.as_ptr().add(offset + 4)),
            );
            let (a0, a1) = separate_pairs(al, ah);
            let (b0, b1) = separate_pairs(bl, bh);
            zero.mul_acc(a0, b0);
            infinity.mul_acc(_mm512_xor_si512(a0, a1), _mm512_xor_si512(b0, b1));
            _mm512_storeu_si512(a_out.as_mut_ptr().add(offset).cast(), al);
            _mm512_storeu_si512(a_out.as_mut_ptr().add(offset + 4).cast(), ah);
            _mm512_storeu_si512(b_out.as_mut_ptr().add(offset).cast(), bl);
            _mm512_storeu_si512(b_out.as_mut_ptr().add(offset + 4).cast(), bh);
        }
        let mut sums = [zero.fold(), infinity.fold()];
        let mut previous = None;
        for row in bulk..weights.len() {
            let start = row * ROW_BYTES;
            let a = table.fold(&a[start..start + ROW_BYTES]);
            let b = table.fold(&b[start..start + ROW_BYTES]) * weights[row];
            a_out[row].write(a);
            b_out[row].write(b);
            if let Some((a0, b0)) = previous.take() {
                sums[0] ^= F128::mul_unreduced(a0, b0);
                sums[1] ^= (a0 + a).mul_unreduced(b0 + b);
            } else {
                previous = Some((a, b));
            }
        }
        sums
    }
}

/// # Safety
/// Requires AVX-512F/BW, VPCLMULQDQ, PCLMULQDQ, and SSE4.1.
#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq,pclmulqdq,sse4.1")]
pub(super) unsafe fn weight(values: &mut [F128], weights: &[F128]) {
    assert_eq!(values.len(), weights.len());
    // SAFETY: every vector load/store handles exactly four in-bounds F128
    // elements. Values and weights cannot overlap by the safe caller's borrows.
    unsafe {
        let bulk = values.len() / 4 * 4;
        for offset in (0..bulk).step_by(4) {
            let value = f128x4_loadu(values.as_ptr().add(offset));
            let weight = f128x4_loadu(weights.as_ptr().add(offset));
            _mm512_storeu_si512(
                values.as_mut_ptr().add(offset).cast(),
                ghash_mul_x4(value, weight),
            );
        }
        for (value, &weight) in values[bulk..].iter_mut().zip(&weights[bulk..]) {
            *value *= weight;
        }
    }
}

/// # Safety
/// Requires AVX-512F/BW, VPCLMULQDQ, PCLMULQDQ, and SSE4.1.
#[target_feature(enable = "avx512f,avx512bw,vpclmulqdq,pclmulqdq,sse4.1")]
pub(super) unsafe fn project(
    table: &BindingTable,
    bytes: &[u8],
    weights: &[F128],
) -> F256Unreduced {
    assert_eq!(bytes.len(), weights.len() * ROW_BYTES);
    // SAFETY: four scalar folds below read four complete in-bounds rows;
    // weight vector loads are bounded by bulk; scalar code handles the tail.
    unsafe {
        let mut sum = WideGhashX4::zero();
        let bulk = weights.len() / 4 * 4;
        for row in (0..bulk).step_by(4) {
            let offset = row * ROW_BYTES;
            let values = f128x4_set(
                table.fold(&bytes[offset..offset + ROW_BYTES]),
                table.fold(&bytes[offset + ROW_BYTES..offset + 2 * ROW_BYTES]),
                table.fold(&bytes[offset + 2 * ROW_BYTES..offset + 3 * ROW_BYTES]),
                table.fold(&bytes[offset + 3 * ROW_BYTES..offset + 4 * ROW_BYTES]),
            );
            sum.mul_acc(values, f128x4_loadu(weights.as_ptr().add(row)));
        }
        let mut sum = sum.fold();
        for (row, &weight) in bytes[bulk * ROW_BYTES..]
            .chunks_exact(ROW_BYTES)
            .zip(&weights[bulk..])
        {
            sum ^= table.fold(row).mul_unreduced(weight);
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use core::mem::MaybeUninit;

    use flock_field::{F128, test_rng::Rng};

    use super::{BindingTable, ROW_BYTES, pack_pair_wide};
    use crate::{lookup::pack_pair_weighted_with_message_into, tensor::KAPPA};

    #[test]
    fn wide_packing_unaligned_outputs_preserve_canaries_and_odd_tails() {
        let mut rng = Rng::new(0xA119_5E17);
        let marker = F128::new(0x5a17_9e3d, 0xc4b8_7201);
        for rows in [0, 1, 7, 8, 9, 17, 1025] {
            let mut a_storage = vec![0; rows * ROW_BYTES + 1];
            let mut b_storage = a_storage.clone();
            rng.fill_bytes(&mut a_storage);
            rng.fill_bytes(&mut b_storage);
            let a = &a_storage[1..];
            let b = &b_storage[1..];
            let columns = rng.f128_vec(KAPPA);
            let weights = rng.f128_vec(rows + 1);
            let weights = &weights[1..];
            let table = BindingTable::new(&columns);
            let mut expected_a = Vec::new();
            let mut expected_b = Vec::new();
            let expected_message = pack_pair_weighted_with_message_into(
                a,
                b,
                &columns,
                weights,
                &mut expected_a,
                &mut expected_b,
            );
            let mut actual_a = vec![MaybeUninit::new(marker); rows + 8];
            let mut actual_b = vec![MaybeUninit::new(marker); rows + 8];
            let prefix_a = (1..=4)
                .find(|&i| !(actual_a.as_ptr() as usize + i * 16).is_multiple_of(64))
                .unwrap();
            let prefix_b = (1..=4)
                .find(|&i| !(actual_b.as_ptr() as usize + i * 16).is_multiple_of(64))
                .unwrap();
            // SAFETY: this module's cfg enables every feature and the output
            // sub-slices are disjoint, correctly sized, and intentionally not
            // aligned to a 64-byte vector. All canaries start initialized.
            let actual_message = unsafe {
                pack_pair_wide(
                    &table,
                    a,
                    b,
                    weights,
                    &mut actual_a[prefix_a..prefix_a + rows],
                    &mut actual_b[prefix_b..prefix_b + rows],
                )
            }
            .map(|value| value.reduce());
            assert_eq!(actual_message, expected_message, "rows={rows}");
            for (buffer, prefix, expected) in [
                (&actual_a, prefix_a, &expected_a),
                (&actual_b, prefix_b, &expected_b),
            ] {
                for (i, value) in buffer.iter().enumerate() {
                    let expected = if (prefix..prefix + rows).contains(&i) {
                        expected[i - prefix]
                    } else {
                        marker
                    };
                    // SAFETY: every element started as the initialized marker;
                    // the kernel overwrote exactly its designated outputs.
                    assert_eq!(
                        unsafe { value.assume_init() },
                        expected,
                        "rows={rows}, i={i}"
                    );
                }
            }
        }
    }
}
