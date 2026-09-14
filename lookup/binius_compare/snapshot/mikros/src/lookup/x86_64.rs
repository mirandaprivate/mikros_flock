//! Field operations reuse Flock's native four-lane VPCLMUL kernels.

use core::arch::x86_64::_mm512_storeu_si512;
use flock_field::{
    F128, F256Unreduced,
    gf2_128::x86_64::{WideGhashX4, f128x4_loadu, f128x4_set, ghash_mul_x4},
};

use super::{BindingTable, ROW_BYTES};

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
