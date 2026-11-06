//! Byte-identical single-oracle RS encoding with bounded parallel initialization.
//!
//! The native bit-reversal and additive NTT are unchanged. At rate 1/2 the upstream
//! copy loop has only one whole-message task; here both copies use at most 64 KiB
//! per task. The allocated codeword has the same size as the native encoder.

use std::{mem::size_of, ptr};

use binius_field::{BinaryField, PackedField};
use binius_iop::fri::FRIParams;
use binius_iop_prover::fri::encode_interleaved;
use binius_math::{
    FieldBuffer, FieldSlice, FieldSliceMut, bit_reverse::bit_reverse_packed, ntt::AdditiveNTT,
};
use rayon::prelude::*;

const COPY_CHUNK_BYTES: usize = 64 * 1024;

/// Encode one unlifted oracle, preserving the native codeword exactly.
///
/// Unsupported shapes and sub-packed messages use the unchanged native encoder.
pub(crate) fn encode_interleaved_parallel<F, P, NTT>(
    params: &FRIParams<F>,
    oracle_index: usize,
    ntt: &NTT,
    message: FieldSlice<'_, P>,
) -> FieldBuffer<P>
where
    F: BinaryField,
    P: PackedField<Scalar = F>,
    NTT: AdditiveNTT<Field = F> + Sync,
{
    let spec = &params.input_oracles()[oracle_index];
    if params.input_oracles().len() != 1
        || spec.log_lift != 0
        || params.rs_code().log_inv_rate() != 1
        || message.log_len() < P::LOG_WIDTH
    {
        return encode_interleaved(params, oracle_index, ntt, message);
    }

    let rs = params.rs_code();
    let log_batch_size = spec.log_batch_size();
    assert_eq!(message.log_len(), rs.log_dim() + log_batch_size);
    assert_eq!(ntt.subspace(rs.log_len()), *rs.subspace());

    // Packed field elements are nonzero-sized Copy values. Both source and output
    // lengths are in packed elements; NTT dimensions below remain scalar lengths.
    let chunk_len = (COPY_CHUNK_BYTES / size_of::<P>()).max(1);
    let message_len = message.as_ref().len();
    let output_len = message_len
        .checked_mul(2)
        .expect("codeword length overflow");
    let mut output = Vec::<P>::with_capacity(output_len);

    output.spare_capacity_mut()[..message_len]
        .par_chunks_mut(chunk_len)
        .zip(message.as_ref().par_chunks(chunk_len))
        .for_each(|(dst, src)| {
            // SAFETY: src is an initialized slice in the distinct message
            // allocation. dst is disjoint spare capacity, exclusively owned by
            // this task, with exactly src.len() properly aligned P slots.
            unsafe {
                ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr().cast::<P>(), src.len());
            }
        });
    // SAFETY: the parallel iterator joined after initializing every element of
    // the prefix, including the last (possibly shorter) task. No suffix is exposed.
    unsafe { output.set_len(message_len) };

    bit_reverse_packed(FieldSliceMut::from_slice(
        message.log_len(),
        output.as_mut_slice(),
    ));

    // Keep a raw address, not an immutable reference to the Vec, while borrowing
    // its spare capacity mutably. The source is the initialized prefix and the
    // destination is the disjoint suffix; the allocation is never resized here.
    let source_addr = output.as_ptr() as usize;
    output.spare_capacity_mut()[..message_len]
        .par_chunks_mut(chunk_len)
        .enumerate()
        .for_each(|(chunk_index, dst)| {
            let offset = chunk_index * chunk_len;
            // SAFETY: offset + dst.len() <= message_len. Source reads remain in
            // the initialized immutable prefix; destination writes remain in the
            // uninitialized suffix of the same allocation. Tasks have disjoint
            // destinations, and no task mutates the prefix or Vec allocation.
            unsafe {
                let source = (source_addr as *const P).add(offset);
                ptr::copy_nonoverlapping(source, dst.as_mut_ptr().cast::<P>(), dst.len());
            }
        });
    // SAFETY: the complete suffix is initialized after the parallel join.
    unsafe { output.set_len(output_len) };

    let mut codeword = FieldBuffer::new(message.log_len() + 1, output);
    ntt.forward_transform(codeword.to_mut(), 1, log_batch_size);
    codeword
}

#[cfg(test)]
mod tests {
    use binius_field::{
        BinaryField128bGhash as B128, PackedBinaryGhash1x128b, PackedBinaryGhash4x128b,
    };
    use binius_math::{
        ReedSolomonCode,
        ntt::{NeighborsLastMultiThread, domain_context::GaoMateerPreExpanded},
    };

    use super::*;

    fn check_case<P: PackedField<Scalar = B128>>(
        log_message: usize,
        log_batch_size: usize,
        log_inv_rate: usize,
        pattern: usize,
    ) {
        let log_dim = log_message - log_batch_size;
        let domain = GaoMateerPreExpanded::<B128>::generate(log_dim + log_inv_rate);
        let shares = rayon::current_num_threads().next_power_of_two().ilog2() as usize;
        let ntt = NeighborsLastMultiThread::new(domain, shares);
        let rs = ReedSolomonCode::with_ntt_subspace(&ntt, log_dim, log_inv_rate);
        let params = FRIParams::new(rs, log_batch_size, Vec::new(), 3);
        let mut message = FieldBuffer::<P>::zeros(log_message);
        for index in 0..1usize << log_message {
            let bits = match pattern {
                0 => 0,
                1 => 1,
                _ => {
                    let x =
                        (index as u128 + 1).wrapping_mul(0xd6e8_feb8_6659_fd93_9e37_79b9_7f4a_7c15);
                    x ^ x.rotate_left(47) ^ (1u128 << (index % 128))
                }
            };
            message.set(index, B128::new(bits));
        }
        let before = message.clone();
        let expected = encode_interleaved(&params, 0, &ntt, message.to_ref());
        let actual = encode_interleaved_parallel(&params, 0, &ntt, message.to_ref());
        assert_eq!(actual.log_len(), expected.log_len());
        assert_eq!(
            actual.as_ref(),
            expected.as_ref(),
            "log_message={log_message}, batch={log_batch_size}, rate={log_inv_rate}, pattern={pattern}"
        );
        assert_eq!(
            message, before,
            "encoding must not mutate the cached message"
        );
    }

    #[test]
    fn parallel_encode_matches_native_one_and_four_threads() {
        for threads in [1, 4] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap()
                .install(|| {
                    // Include sub-packed fallback, exactly one packed word, and
                    // several 64 KiB copy chunks. Batch coordinates affect the
                    // NTT layout and must not be dropped by the replacement.
                    for (log_message, batch) in
                        [(1, 0), (2, 0), (3, 1), (5, 3), (10, 0), (14, 0), (14, 3)]
                    {
                        for pattern in 0..3 {
                            check_case::<PackedBinaryGhash1x128b>(log_message, batch, 1, pattern);
                            check_case::<PackedBinaryGhash4x128b>(log_message, batch, 1, pattern);
                        }
                    }
                    // Other rates remain delegated to the native implementation.
                    check_case::<PackedBinaryGhash4x128b>(10, 2, 2, 2);
                });
        }
    }
}
