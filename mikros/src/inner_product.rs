//! Degree-two sumcheck for a length-2^ell inner product, low bit first.
//! Each round sends g(0) and the X^2 coefficient. In characteristic two,
//! g(0)+g(1)=claim implies g(t)=g(0)+t*claim+(t^2+t)*g_infinity.

use core::mem::MaybeUninit;
use flock_field::{F128, F256Unreduced};
use flock_multilinear::fold_low;
use flock_transcript::challenger::Challenger;
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelIterator, ParallelIterator, ParallelSlice,
    ParallelSliceMut,
};
use serde::{Deserialize, Serialize};

use crate::hadamard::Error;

#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512f",
    target_feature = "avx512bw",
    target_feature = "vpclmulqdq",
    target_feature = "pclmulqdq",
    target_feature = "sse4.1"
))]
pub(crate) mod x86_64;

/// Caller-owned storage retained between independent proofs. Challenge values
/// and messages are recomputed on every call; only allocation capacity is reused.
#[derive(Default)]
pub(crate) struct ProverScratch {
    pub(crate) a: Vec<F128>,
    pub(crate) b: Vec<F128>,
    a_next: Vec<F128>,
    b_next: Vec<F128>,
}

#[cfg(test)]
std::thread_local! {
    static BATCH_PARALLEL_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
}

fn batch_parallel_enabled() -> bool {
    #[cfg(test)]
    if let Some(value) = BATCH_PARALLEL_OVERRIDE.with(|slot| slot.get()) {
        return value;
    }
    std::env::var_os("MIKROS_IP_BATCH_PARALLEL").is_some_and(|value| value == "1")
}

#[cfg(test)]
pub(crate) fn with_batch_parallel_override<T>(value: bool, run: impl FnOnce() -> T) -> T {
    struct Restore(Option<bool>);
    impl Drop for Restore {
        fn drop(&mut self) {
            BATCH_PARALLEL_OVERRIDE.with(|slot| slot.set(self.0));
        }
    }
    let _restore = Restore(BATCH_PARALLEL_OVERRIDE.with(|slot| slot.replace(Some(value))));
    run()
}

impl ProverScratch {
    /// Take ownership of earlier fold allocations. Only capacity is reused:
    /// prepare_output clears and initializes every destination before reading.
    pub(crate) fn with_fold_buffers([a_next, b_next]: [Vec<F128>; 2]) -> Self {
        Self {
            a: Vec::new(),
            b: Vec::new(),
            a_next,
            b_next,
        }
    }
}

/// Expose allocation capacity for the next output without touching its pages.
/// F128 is Copy, so clearing cannot run destructors. Until the caller has
/// written every requested value and commits the length, a panic leaves a
/// valid empty vector and any partially written field elements need no drop.
pub(crate) fn prepare_output(buffer: &mut Vec<F128>, len: usize) -> &mut [MaybeUninit<F128>] {
    buffer.clear();
    buffer.reserve(len);
    &mut buffer.spare_capacity_mut()[..len]
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InnerProductProof {
    pub rounds: Vec<[F128; 2]>,
    pub final_a: F128,
    pub final_b: F128,
}

fn round_message(a: &[F128], b: &[F128]) -> [F128; 2] {
    let sum_chunk = |(a, b): (&[F128], &[F128])| {
        let mut out = [F128::ZERO; 2];
        for (a, b) in a.chunks_exact(2).zip(b.chunks_exact(2)) {
            out[0] += a[0] * b[0];
            out[1] += (a[0] + a[1]) * (b[0] + b[1]);
        }
        out
    };
    if a.len() < 4096 {
        sum_chunk((a, b))
    } else {
        a.par_chunks(2048)
            .zip(b.par_chunks(2048))
            .map(sum_chunk)
            .reduce(|| [F128::ZERO; 2], |a, b| [a[0] + b[0], a[1] + b[1]])
    }
}

pub fn prove<C: Challenger>(
    mut a: Vec<F128>,
    mut b: Vec<F128>,
    challenger: &mut C,
) -> (InnerProductProof, Vec<F128>) {
    debug_assert_eq!(a.len(), b.len());
    debug_assert!(a.len().is_power_of_two());
    challenger.observe_label(b"mikros-inner-product-v1");
    let mut rounds = Vec::with_capacity(a.len().ilog2() as usize);
    let mut point = Vec::with_capacity(rounds.capacity());
    while a.len() > 1 {
        let message = round_message(&a, &b);
        challenger.observe_f128_slice(&message);
        let challenge = challenger.sample_f128();
        fold_low(&mut a, challenge);
        fold_low(&mut b, challenge);
        rounds.push(message);
        point.push(challenge);
    }
    challenger.observe_f128_slice(&[a[0], b[0]]);
    // Public matrix queries use HighToLow coordinates.
    point.reverse();
    (
        InnerProductProof {
            rounds,
            final_a: a[0],
            final_b: b[0],
        },
        point,
    )
}

// Chunk boundaries always preserve adjacent output pairs. A task writes two
// 64-KiB output chunks, and consumes four input values per message summand.
// Keep short tail rounds and a one-worker pool on the straight-line path:
// otherwise scheduling can dominate the handful of field operations per row.
const FOLD_OUTPUT_CHUNK: usize = 4096;
const PARALLEL_INPUT_THRESHOLD: usize = 32768;

fn use_parallel(input_len: usize) -> bool {
    input_len >= PARALLEL_INPUT_THRESHOLD && rayon::current_num_threads() > 1
}

fn add_unreduced(left: [F256Unreduced; 2], right: [F256Unreduced; 2]) -> [F256Unreduced; 2] {
    [left[0] ^ right[0], left[1] ^ right[1]]
}

/// Sum adjacent-pair products, using four independent VPCLMUL lanes when
/// enabled by the same native build flags as Flock. Odd tails are not pairs.
pub(crate) fn native_message_chunk(a: &[F128], b: &[F128]) -> [F256Unreduced; 2] {
    assert_eq!(a.len(), b.len());
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "pclmulqdq",
        target_feature = "sse4.1"
    ))]
    {
        // SAFETY: compile-time feature gates match the kernel contract, and
        // the kernel bounds every vector load before handling scalar tails.
        unsafe { x86_64::message_chunk(a, b) }
    }
    #[cfg(not(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "pclmulqdq",
        target_feature = "sse4.1"
    )))]
    {
        let mut out = [F256Unreduced::ZERO; 2];
        for (a, b) in a.chunks_exact(2).zip(b.chunks_exact(2)) {
            out[0] ^= a[0].mul_unreduced(b[0]);
            out[1] ^= (a[0] + a[1]).mul_unreduced(b[0] + b[1]);
        }
        out
    }
}

fn round_message_native(a: &[F128], b: &[F128]) -> [F128; 2] {
    let sums = if !use_parallel(a.len()) {
        native_message_chunk(a, b)
    } else {
        a.par_chunks(2 * FOLD_OUTPUT_CHUNK)
            .zip(b.par_chunks(2 * FOLD_OUTPUT_CHUNK))
            .map(|(a, b)| native_message_chunk(a, b))
            .reduce(|| [F256Unreduced::ZERO; 2], add_unreduced)
    };
    sums.map(F256Unreduced::reduce)
}

/// Compute both the original dot product and its first IP message in one
/// pass. The extra g(1) accumulator makes the self-check g(0)+g(1) available
/// without a second scan or recomputing g(0). No transcript action occurs here.
pub(crate) fn checked_initial_message_native(a: &[F128], b: &[F128]) -> (F128, Option<[F128; 2]>) {
    assert_eq!(a.len(), b.len());
    assert!(a.len().is_power_of_two());
    if a.len() == 1 {
        return (a[0] * b[0], None);
    }
    let sum_chunk = |(a, b): (&[F128], &[F128])| {
        #[cfg(all(
            target_arch = "x86_64",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "vpclmulqdq",
            target_feature = "pclmulqdq",
            target_feature = "sse4.1"
        ))]
        {
            // SAFETY: compile-time gates match the kernel; equal-length,
            // even-sized chunks keep every vector load and scalar tail valid.
            unsafe { x86_64::checked_message_chunk(a, b) }
        }
        #[cfg(not(all(
            target_arch = "x86_64",
            target_feature = "avx512f",
            target_feature = "avx512bw",
            target_feature = "vpclmulqdq",
            target_feature = "pclmulqdq",
            target_feature = "sse4.1"
        )))]
        {
            let mut sums = [F256Unreduced::ZERO; 3];
            for (a, b) in a.chunks_exact(2).zip(b.chunks_exact(2)) {
                sums[0] ^= a[0].mul_unreduced(b[0]);
                sums[1] ^= a[1].mul_unreduced(b[1]);
                sums[2] ^= (a[0] + a[1]).mul_unreduced(b[0] + b[1]);
            }
            sums
        }
    };
    let sums = if !use_parallel(a.len()) {
        sum_chunk((a, b))
    } else {
        a.par_chunks(2 * FOLD_OUTPUT_CHUNK)
            .zip(b.par_chunks(2 * FOLD_OUTPUT_CHUNK))
            .map(sum_chunk)
            .reduce(
                || [F256Unreduced::ZERO; 3],
                |a, b| [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2]],
            )
    };
    let [zero, one, infinity] = sums.map(F256Unreduced::reduce);
    (zero + one, Some([zero, infinity]))
}

/// Initial round message without reducing each separate product. Reduction
/// modulo the GHASH polynomial is XOR-linear, so the two total accumulators
/// give exactly the baseline's two sums.
fn round_message_packed(a: &[F128], b: &[F128]) -> [F128; 2] {
    let sum_chunk = |(a, b): (&[F128], &[F128])| {
        let mut out = [F256Unreduced::ZERO; 2];
        for (a, b) in a.chunks_exact(2).zip(b.chunks_exact(2)) {
            out[0] ^= a[0].mul_unreduced(b[0]);
            out[1] ^= (a[0] + a[1]).mul_unreduced(b[0] + b[1]);
        }
        out
    };
    let sums = if !use_parallel(a.len()) {
        sum_chunk((a, b))
    } else {
        a.par_chunks(2 * FOLD_OUTPUT_CHUNK)
            .zip(b.par_chunks(2 * FOLD_OUTPUT_CHUNK))
            .map(sum_chunk)
            .reduce(|| [F256Unreduced::ZERO; 2], add_unreduced)
    };
    sums.map(F256Unreduced::reduce)
}

/// Bind one challenge and accumulate the following round's message from the
/// freshly computed values. Each four inputs produce two outputs and one
/// quadratic-message summand. No transformed value is recomputed or reread
/// from a full intermediate vector to construct that message.
fn fold_chunk_and_message(
    a: &[F128],
    b: &[F128],
    a_out: &mut [F128],
    b_out: &mut [F128],
    challenge: F128,
) -> [F256Unreduced; 2] {
    debug_assert_eq!(a.len(), b.len());
    debug_assert_eq!(a.len(), 2 * a_out.len());
    debug_assert_eq!(a_out.len(), b_out.len());
    debug_assert_eq!(a.len() % 4, 0);
    let mut sums = [F256Unreduced::ZERO; 2];
    for (((a, b), a_out), b_out) in a
        .chunks_exact(4)
        .zip(b.chunks_exact(4))
        .zip(a_out.chunks_exact_mut(2))
        .zip(b_out.chunks_exact_mut(2))
    {
        // Same one-multiply interpolation used by the baseline's fold_low.
        let a0 = a[0] + challenge * (a[1] + a[0]);
        let a1 = a[2] + challenge * (a[3] + a[2]);
        let b0 = b[0] + challenge * (b[1] + b[0]);
        let b1 = b[2] + challenge * (b[3] + b[2]);
        a_out[0] = a0;
        a_out[1] = a1;
        b_out[0] = b0;
        b_out[1] = b1;
        sums[0] ^= a0.mul_unreduced(b0);
        sums[1] ^= (a0 + a1).mul_unreduced(b0 + b1);
    }
    sums
}

fn fold_and_message(
    a: &[F128],
    b: &[F128],
    a_out: &mut [F128],
    b_out: &mut [F128],
    challenge: F128,
) -> [F128; 2] {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), 2 * a_out.len());
    assert_eq!(a_out.len(), b_out.len());
    assert_eq!(a.len() % 4, 0);
    let sums = if !use_parallel(a.len()) {
        fold_chunk_and_message(a, b, a_out, b_out, challenge)
    } else {
        a_out
            .par_chunks_mut(FOLD_OUTPUT_CHUNK)
            .zip(b_out.par_chunks_mut(FOLD_OUTPUT_CHUNK))
            .enumerate()
            .map(|(chunk, (a_out, b_out))| {
                let start = 2 * chunk * FOLD_OUTPUT_CHUNK;
                let end = start + 2 * a_out.len();
                fold_chunk_and_message(&a[start..end], &b[start..end], a_out, b_out, challenge)
            })
            .reduce(|| [F256Unreduced::ZERO; 2], add_unreduced)
    };
    sums.map(F256Unreduced::reduce)
}

/// Transcript-identical engineering variant of [`prove`]. The first message
/// is computed directly; every subsequent message shares its input pass with
/// the preceding challenge's fold. Parallel workers own disjoint output
/// slices, and two half-size scratch buffers are reused across all rounds.
pub fn prove_packed<C: Challenger>(
    mut a: Vec<F128>,
    mut b: Vec<F128>,
    challenger: &mut C,
) -> (InnerProductProof, Vec<F128>) {
    assert_eq!(a.len(), b.len());
    assert!(a.len().is_power_of_two());
    challenger.observe_label(b"mikros-inner-product-v1");
    let mut rounds = Vec::with_capacity(a.len().ilog2() as usize);
    let mut point = Vec::with_capacity(rounds.capacity());
    if a.len() > 1 {
        let mut message = round_message_packed(&a, &b);
        // Safe initialized storage, allocated once. After each swap the old
        // input allocation becomes the destination for a strictly shorter
        // output, so every written slice stays within its initialized length.
        let scratch_len = if a.len() >= 4 { a.len() / 2 } else { 0 };
        let mut a_next = vec![F128::ZERO; scratch_len];
        let mut b_next = vec![F128::ZERO; scratch_len];
        loop {
            challenger.observe_f128_slice(&message);
            let challenge = challenger.sample_f128();
            rounds.push(message);
            point.push(challenge);
            if a.len() == 2 {
                a[0] = a[0] + challenge * (a[1] + a[0]);
                b[0] = b[0] + challenge * (b[1] + b[0]);
                break;
            }
            let half = a.len() / 2;
            message = fold_and_message(&a, &b, &mut a_next[..half], &mut b_next[..half], challenge);
            std::mem::swap(&mut a, &mut a_next);
            std::mem::swap(&mut b, &mut b_next);
            a.truncate(half);
            b.truncate(half);
        }
    }
    challenger.observe_f128_slice(&[a[0], b[0]]);
    point.reverse();
    (
        InnerProductProof {
            rounds,
            final_a: a[0],
            final_b: b[0],
        },
        point,
    )
}

/// Write every output value exactly once before it can become an input. This
/// scalar tail also serves the portable native path; the old ablation remains
/// unchanged and continues to use initialized output slices.
fn fold_chunk_uninit_scalar(
    a: &[F128],
    b: &[F128],
    a_out: &mut [MaybeUninit<F128>],
    b_out: &mut [MaybeUninit<F128>],
    challenge: F128,
) -> [F256Unreduced; 2] {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), 2 * a_out.len());
    assert_eq!(a_out.len(), b_out.len());
    assert_eq!(a.len() % 4, 0);
    let mut sums = [F256Unreduced::ZERO; 2];
    for (((a, b), a_out), b_out) in a
        .chunks_exact(4)
        .zip(b.chunks_exact(4))
        .zip(a_out.chunks_exact_mut(2))
        .zip(b_out.chunks_exact_mut(2))
    {
        let a0 = a[0] + challenge * (a[1] + a[0]);
        let a1 = a[2] + challenge * (a[3] + a[2]);
        let b0 = b[0] + challenge * (b[1] + b[0]);
        let b1 = b[2] + challenge * (b[3] + b[2]);
        a_out[0].write(a0);
        a_out[1].write(a1);
        b_out[0].write(b0);
        b_out[1].write(b1);
        sums[0] ^= a0.mul_unreduced(b0);
        sums[1] ^= (a0 + a1).mul_unreduced(b0 + b1);
    }
    sums
}

fn fold_chunk_native(
    a: &[F128],
    b: &[F128],
    a_out: &mut [MaybeUninit<F128>],
    b_out: &mut [MaybeUninit<F128>],
    challenge: F128,
) -> [F256Unreduced; 2] {
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "pclmulqdq",
        target_feature = "sse4.1"
    ))]
    {
        // SAFETY: features are statically enabled; all lengths are asserted
        // before the kernel, which bounds vector loads and stores by slices.
        unsafe { x86_64::fold_chunk(a, b, a_out, b_out, challenge) }
    }
    #[cfg(not(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "pclmulqdq",
        target_feature = "sse4.1"
    )))]
    fold_chunk_uninit_scalar(a, b, a_out, b_out, challenge)
}

fn fold_and_message_native(
    a: &[F128],
    b: &[F128],
    a_out: &mut Vec<F128>,
    b_out: &mut Vec<F128>,
    challenge: F128,
) -> [F128; 2] {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len() % 4, 0);
    let half = a.len() / 2;
    let a_storage = prepare_output(a_out, half);
    let b_storage = prepare_output(b_out, half);
    let sums = if !use_parallel(a.len()) {
        fold_chunk_native(a, b, a_storage, b_storage, challenge)
    } else {
        a_storage
            .par_chunks_mut(FOLD_OUTPUT_CHUNK)
            .zip(b_storage.par_chunks_mut(FOLD_OUTPUT_CHUNK))
            .enumerate()
            .map(|(chunk, (a_out, b_out))| {
                let start = 2 * chunk * FOLD_OUTPUT_CHUNK;
                let end = start + 2 * a_out.len();
                fold_chunk_native(&a[start..end], &b[start..end], a_out, b_out, challenge)
            })
            .reduce(|| [F256Unreduced::ZERO; 2], add_unreduced)
    };
    // SAFETY: the dimension assertions and disjoint even-sized chunks ensure
    // that the scalar/SIMD kernels initialized every output slot. All Rayon
    // work has joined. A panic skips this commit and leaves valid empty Vecs.
    unsafe {
        a_out.set_len(half);
        b_out.set_len(half);
    }
    sums.map(F256Unreduced::reduce)
}

/// Native field kernels with caller-owned scratch. `initial_message` may be
/// supplied by the fused packing pass. It must equal the ordinary first round
/// message for `scratch.a` and `scratch.b`; it is unused for one-element inputs.
/// No challenge is sampled early, and all observations match [`prove`].
pub(crate) fn prove_native<C: Challenger>(
    scratch: &mut ProverScratch,
    initial_message: Option<[F128; 2]>,
    challenger: &mut C,
) -> (InnerProductProof, Vec<F128>) {
    let ProverScratch {
        a,
        b,
        a_next,
        b_next,
    } = scratch;
    assert_eq!(a.len(), b.len());
    assert!(a.len().is_power_of_two());
    let mut len = a.len();
    challenger.observe_label(b"mikros-inner-product-v1");
    let mut rounds = Vec::with_capacity(len.ilog2() as usize);
    let mut point = Vec::with_capacity(rounds.capacity());
    if len > 1 {
        let mut message = initial_message.unwrap_or_else(|| round_message_native(a, b));
        loop {
            challenger.observe_f128_slice(&message);
            let challenge = challenger.sample_f128();
            rounds.push(message);
            point.push(challenge);
            if len == 2 {
                a[0] = a[0] + challenge * (a[1] + a[0]);
                b[0] = b[0] + challenge * (b[1] + b[0]);
                break;
            }
            let half = len / 2;
            message = fold_and_message_native(&a[..len], &b[..len], a_next, b_next, challenge);
            std::mem::swap(a, a_next);
            std::mem::swap(b, b_next);
            len = half;
        }
    }
    let final_a = a[0];
    let final_b = b[0];
    challenger.observe_f128_slice(&[final_a, final_b]);
    point.reverse();
    // Keep the largest allocations in the packing destinations for the next
    // proof. The final values above are already copied into the proof.
    if a_next.capacity() > a.capacity() {
        std::mem::swap(a, a_next);
    }
    if b_next.capacity() > b.capacity() {
        std::mem::swap(b, b_next);
    }
    (
        InnerProductProof {
            rounds,
            final_a,
            final_b,
        },
        point,
    )
}

/// Independent IP equations with a common terminal point. Every block's
/// message is fixed before each shared challenge; no equations are combined.
pub(crate) fn prove_native_batch<C: Challenger>(
    scratch: &mut [ProverScratch],
    initial_messages: &[Option<[F128; 2]>],
    challenger: &mut C,
) -> (Vec<InnerProductProof>, Vec<F128>) {
    assert!(!scratch.is_empty());
    assert_eq!(scratch.len(), initial_messages.len());
    let mut len = scratch[0].a.len();
    assert!(len.is_power_of_two());
    assert!(scratch.iter().all(|s| s.a.len() == len && s.b.len() == len));
    let parallel_batch =
        batch_parallel_enabled() && scratch.len() > 1 && rayon::current_num_threads() > 1;
    challenger.observe_label(b"mikros-inner-product-shared-point-v1");
    challenger.observe_bytes(&(scratch.len() as u64).to_le_bytes());
    challenger.observe_bytes(&(len.ilog2() as u64).to_le_bytes());
    let mut proofs: Vec<_> = scratch
        .iter()
        .map(|_| InnerProductProof {
            rounds: Vec::with_capacity(len.ilog2() as usize),
            final_a: F128::ZERO,
            final_b: F128::ZERO,
        })
        .collect();
    let mut point = Vec::with_capacity(len.ilog2() as usize);
    let mut messages: Vec<_> = scratch
        .iter()
        .zip(initial_messages)
        .map(|(s, initial)| {
            if len > 1 {
                initial.unwrap_or_else(|| round_message_native(&s.a, &s.b))
            } else {
                [F128::ZERO; 2]
            }
        })
        .collect();
    while len > 1 {
        for (proof, message) in proofs.iter_mut().zip(&messages) {
            challenger.observe_f128_slice(message);
            proof.rounds.push(*message);
        }
        let challenge = challenger.sample_f128();
        point.push(challenge);
        if parallel_batch && len >= 4096 {
            fold_batch_and_message_native(scratch, &mut messages, len, challenge);
        } else {
            for (s, message) in scratch.iter_mut().zip(&mut messages) {
                if len == 2 {
                    s.a[0] = s.a[0] + challenge * (s.a[1] + s.a[0]);
                    s.b[0] = s.b[0] + challenge * (s.b[1] + s.b[0]);
                } else {
                    *message = fold_and_message_native(
                        &s.a[..len],
                        &s.b[..len],
                        &mut s.a_next,
                        &mut s.b_next,
                        challenge,
                    );
                    std::mem::swap(&mut s.a, &mut s.a_next);
                    std::mem::swap(&mut s.b, &mut s.b_next);
                }
            }
        }
        len /= 2;
    }
    for (proof, s) in proofs.iter_mut().zip(scratch) {
        proof.final_a = s.a[0];
        proof.final_b = s.b[0];
        challenger.observe_f128_slice(&[proof.final_a, proof.final_b]);
        if s.a_next.capacity() > s.a.capacity() {
            std::mem::swap(&mut s.a, &mut s.a_next);
        }
        if s.b_next.capacity() > s.b.capacity() {
            std::mem::swap(&mut s.b, &mut s.b_next);
        }
    }
    point.reverse();
    (proofs, point)
}

/// All groups' disjoint output tiles share one parallel queue and barrier.
/// The transcript remains outside this function. Small tails retain the
/// original scalar scheduling; the tile size bounds task count on large inputs.
fn fold_batch_and_message_native(
    scratch: &mut [ProverScratch],
    messages: &mut [[F128; 2]],
    len: usize,
    challenge: F128,
) {
    assert_eq!(scratch.len(), messages.len());
    assert_eq!(len % 4, 0);
    let half = len / 2;
    let tasks_per_group = (4 * rayon::current_num_threads()).div_ceil(scratch.len());
    let tile = half.div_ceil(tasks_per_group).max(512).next_power_of_two();
    let mut jobs = Vec::new();
    for (group, s) in scratch.iter_mut().enumerate() {
        let a_out = prepare_output(&mut s.a_next, half);
        let b_out = prepare_output(&mut s.b_next, half);
        for (chunk, (a_out, b_out)) in a_out
            .chunks_mut(tile)
            .zip(b_out.chunks_mut(tile))
            .enumerate()
        {
            let start = chunk * tile * 2;
            let end = start + a_out.len() * 2;
            jobs.push((group, &s.a[start..end], &s.b[start..end], a_out, b_out));
        }
    }
    let partials: Vec<_> = jobs
        .into_par_iter()
        .map(|(group, a, b, a_out, b_out)| {
            (group, fold_chunk_native(a, b, a_out, b_out, challenge))
        })
        .collect();
    let mut sums = vec![[F256Unreduced::ZERO; 2]; scratch.len()];
    for (group, partial) in partials {
        sums[group] = add_unreduced(sums[group], partial);
    }
    for ((s, message), sum) in scratch.iter_mut().zip(messages).zip(sums) {
        // SAFETY: every half-sized output was split into disjoint complete
        // tiles. All jobs initialized every element and joined before commit;
        // a panic above leaves the vectors empty and safely droppable.
        unsafe {
            s.a_next.set_len(half);
            s.b_next.set_len(half);
        }
        *message = sum.map(F256Unreduced::reduce);
        std::mem::swap(&mut s.a, &mut s.a_next);
        std::mem::swap(&mut s.b, &mut s.b_next);
    }
}

pub(crate) fn verify_batch<C: Challenger>(
    proofs: &[InnerProductProof],
    claims: &[F128],
    challenger: &mut C,
) -> Result<Vec<F128>, Error> {
    if proofs.is_empty() || proofs.len() != claims.len() {
        return Err(Error::ProofShape("shared IP batch"));
    }
    let rounds = proofs[0].rounds.len();
    if proofs.iter().any(|proof| proof.rounds.len() != rounds) {
        return Err(Error::ProofShape("shared IP rounds"));
    }
    challenger.observe_label(b"mikros-inner-product-shared-point-v1");
    challenger.observe_bytes(&(proofs.len() as u64).to_le_bytes());
    challenger.observe_bytes(&(rounds as u64).to_le_bytes());
    let mut claims = claims.to_vec();
    let mut point = Vec::with_capacity(rounds);
    for round in 0..rounds {
        for proof in proofs {
            challenger.observe_f128_slice(&proof.rounds[round]);
        }
        let challenge = challenger.sample_f128();
        point.push(challenge);
        for (proof, claim) in proofs.iter().zip(&mut claims) {
            let [at_zero, at_infinity] = proof.rounds[round];
            *claim = at_zero + challenge * *claim + (challenge.square() + challenge) * at_infinity;
        }
    }
    for (proof, claim) in proofs.iter().zip(claims) {
        challenger.observe_f128_slice(&[proof.final_a, proof.final_b]);
        if claim != proof.final_a * proof.final_b {
            return Err(Error::InnerProductMismatch);
        }
    }
    point.reverse();
    Ok(point)
}

pub fn verify<C: Challenger>(
    proof: &InnerProductProof,
    mut claim: F128,
    challenger: &mut C,
) -> Result<Vec<F128>, Error> {
    challenger.observe_label(b"mikros-inner-product-v1");
    let mut point = Vec::with_capacity(proof.rounds.len());
    for &[at_zero, at_infinity] in &proof.rounds {
        challenger.observe_f128_slice(&[at_zero, at_infinity]);
        let challenge = challenger.sample_f128();
        claim = at_zero + challenge * claim + (challenge.square() + challenge) * at_infinity;
        point.push(challenge);
    }
    challenger.observe_f128_slice(&[proof.final_a, proof.final_b]);
    if claim != proof.final_a * proof.final_b {
        return Err(Error::InnerProductMismatch);
    }
    point.reverse();
    Ok(point)
}

#[cfg(test)]
mod batch_parallel_tests {
    use super::*;
    use flock_transcript::challenger::FsChallenger;

    #[test]
    fn batch_parallel_geometric_zero_one_matches_legacy() {
        for threads in [1, 3] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            pool.install(|| {
                for (log_n, count, pattern) in [
                    (0, 2, 0),
                    (1, 2, 1),
                    (10, 2, 2),
                    (12, 2, 2),
                    (12, 3, 2),
                    (15, 2, 2),
                ] {
                    let inputs: Vec<_> = (0..count)
                        .map(|group| {
                            let mut a = Vec::with_capacity(1 << log_n);
                            let mut b = Vec::with_capacity(1 << log_n);
                            let (mut left, mut right) = (F128::ONE, F128::ONE);
                            let left_ratio = F128::new(3 + group as u64, 1);
                            let right_ratio = F128::new(7 + group as u64, 2);
                            for _ in 0..1 << log_n {
                                a.push(match pattern {
                                    0 => F128::ZERO,
                                    1 => F128::ONE,
                                    _ => left,
                                });
                                b.push(match pattern {
                                    0 => F128::ZERO,
                                    1 => F128::ONE,
                                    _ => right,
                                });
                                left *= left_ratio;
                                right *= right_ratio;
                            }
                            (a, b)
                        })
                        .collect();
                    let claims: Vec<_> = inputs
                        .iter()
                        .map(|(a, b)| {
                            a.iter()
                                .zip(b)
                                .fold(F256Unreduced::ZERO, |sum, (&a, &b)| {
                                    sum ^ a.mul_unreduced(b)
                                })
                                .reduce()
                        })
                        .collect();
                    let run = |parallel, supplied| {
                        with_batch_parallel_override(parallel, || {
                            let mut scratch: Vec<_> = inputs
                                .iter()
                                .map(|(a, b)| ProverScratch {
                                    a: a.clone(),
                                    b: b.clone(),
                                    ..ProverScratch::default()
                                })
                                .collect();
                            let initial: Vec<_> = inputs
                                .iter()
                                .map(|(a, b)| {
                                    if supplied && a.len() > 1 {
                                        Some(native_message_chunk(a, b).map(F256Unreduced::reduce))
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                            let mut pc = FsChallenger::new(b"batch-parallel-IP-equivalence");
                            let (proofs, point) =
                                prove_native_batch(&mut scratch, &initial, &mut pc);
                            let mut vc = FsChallenger::new(b"batch-parallel-IP-equivalence");
                            assert_eq!(verify_batch(&proofs, &claims, &mut vc).unwrap(), point);
                            let future = pc.sample_f128_vec(4);
                            assert_eq!(future, vc.sample_f128_vec(4));
                            (bincode::serialize(&proofs).unwrap(), point, future)
                        })
                    };
                    let expected = run(false, false);
                    assert_eq!(
                        run(true, false),
                        expected,
                        "threads={threads}, log_n={log_n}, count={count}"
                    );
                    assert_eq!(
                        run(true, true),
                        expected,
                        "supplied message: threads={threads}, log_n={log_n}, count={count}"
                    );
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ProverScratch, checked_initial_message_native, fold_and_message, fold_and_message_native,
        native_message_chunk, prepare_output, prove, prove_native, prove_packed, round_message,
        verify,
    };
    use flock_field::{F128, test_rng::Rng};
    use flock_multilinear::fold_low;
    use flock_transcript::challenger::{Challenger, FsChallenger};

    #[test]
    fn checked_initial_message_matches_dot_and_scalar_message() {
        for threads in [1, 3] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            pool.install(|| {
                let mut rng = Rng::new(0xF05E_1F05);
                for n in [1, 2, 4, 8, 128, 8192, 16384, 32768, 65536] {
                    for pattern in 0..3 {
                        let mut a = rng.f128_vec(n);
                        let mut b = rng.f128_vec(n);
                        if pattern == 1 {
                            a.fill(F128::ZERO);
                        } else if pattern == 2 {
                            a.fill(F128::ONE);
                            b.fill(F128::ZERO);
                            b[n - 1] = F128::generator();
                        }
                        let expected = a.iter().zip(&b).fold(F128::ZERO, |s, (&a, &b)| s + a * b);
                        let (actual, message) = checked_initial_message_native(&a, &b);
                        assert_eq!(
                            actual, expected,
                            "dot: threads={threads}, n={n}, pattern={pattern}"
                        );
                        assert_eq!(
                            message,
                            (n > 1).then(|| round_message(&a, &b)),
                            "message: threads={threads}, n={n}, pattern={pattern}"
                        );
                    }
                }
            });
        }
    }

    #[test]
    fn fused_fold_and_message_matches_direct_work_at_chunk_boundaries() {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(3)
            .build()
            .unwrap();
        pool.install(|| {
            let mut rng = Rng::new(0xF05E_1F01);
            // Non-power-of-two lengths exercise partial final chunks in this
            // private kernel; real protocol vectors are powers of two.
            for n in [4, 8, 8188, 8192, 8196, 32764, 32768, 32772, 65540] {
                let a = rng.f128_vec(n);
                let b = rng.f128_vec(n);
                for challenge in [F128::ZERO, F128::ONE, rng.f128()] {
                    let direct_fold = |input: &[F128]| -> Vec<F128> {
                        input
                            .chunks_exact(2)
                            .map(|pair| pair[0] * (F128::ONE + challenge) + pair[1] * challenge)
                            .collect()
                    };
                    let expected_a = direct_fold(&a);
                    let expected_b = direct_fold(&b);
                    let mut actual_a = vec![F128::ZERO; n / 2];
                    let mut actual_b = vec![F128::ZERO; n / 2];
                    let message = fold_and_message(&a, &b, &mut actual_a, &mut actual_b, challenge);
                    assert_eq!(actual_a, expected_a, "a, input length {n}");
                    assert_eq!(actual_b, expected_b, "b, input length {n}");
                    assert_eq!(
                        message,
                        round_message(&expected_a, &expected_b),
                        "message, input length {n}"
                    );
                    let native =
                        fold_and_message_native(&a, &b, &mut actual_a, &mut actual_b, challenge);
                    assert_eq!(actual_a, expected_a, "native a, input length {n}");
                    assert_eq!(actual_b, expected_b, "native b, input length {n}");
                    assert_eq!(native, message, "native message, input length {n}");
                }
            }
        });
    }

    #[test]
    fn packed_proof_and_transcript_match_baseline_across_lengths_and_threads() {
        for threads in [1, 2, 6] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            pool.install(|| {
                let mut rng = Rng::new(0xF05E_1F02);
                let mut scratch = ProverScratch::default();
                for n in [1, 2, 4, 32, 1024, 4096, 16384, 32768, 65536, 1, 2, 32768, 4] {
                    for pattern in 0..3 {
                        let mut a = rng.f128_vec(n);
                        let mut b = rng.f128_vec(n);
                        if pattern == 1 {
                            a.fill(F128::ZERO);
                        } else if pattern == 2 {
                            a.fill(F128::ONE);
                            b.fill(F128::ZERO);
                            b[n - 1] = F128::generator();
                        }
                        let claim = a
                            .iter()
                            .zip(&b)
                            .fold(F128::ZERO, |sum, (&a, &b)| sum + a * b);
                        let domain = b"mikros-packed-inner-product-differential-v1";
                        let mut baseline_ch = FsChallenger::new(domain);
                        let baseline = prove(a.clone(), b.clone(), &mut baseline_ch);
                        let mut packed_ch = FsChallenger::new(domain);
                        let packed = prove_packed(a.clone(), b.clone(), &mut packed_ch);
                        assert_eq!(
                            packed, baseline,
                            "threads {threads}, length {n}, pattern {pattern}"
                        );
                        assert_eq!(packed_ch.sample_f128(), baseline_ch.sample_f128());
                        let initial = native_message_chunk(&a, &b).map(|sum| sum.reduce());
                        for supplied in [None, Some(initial)] {
                            scratch.a.clone_from(&a);
                            scratch.b.clone_from(&b);
                            let capacity_a = scratch.a.capacity();
                            let capacity_b = scratch.b.capacity();
                            let mut native_ch = FsChallenger::new(domain);
                            let native = prove_native(&mut scratch, supplied, &mut native_ch);
                            assert_eq!(native, baseline);
                            let mut expected_ch = FsChallenger::new(domain);
                            let _ = prove_packed(a.clone(), b.clone(), &mut expected_ch);
                            assert_eq!(native_ch.sample_f128(), expected_ch.sample_f128());
                            assert!(scratch.a.capacity() >= capacity_a);
                            assert!(scratch.b.capacity() >= capacity_b);
                        }
                        let point =
                            verify(&packed.0, claim, &mut FsChallenger::new(domain)).unwrap();
                        assert_eq!(point, packed.1);

                        // Independently fold the original vectors using the
                        // returned HighToLow coordinates, including zero rounds.
                        for &coordinate in point.iter().rev() {
                            fold_low(&mut a, coordinate);
                            fold_low(&mut b, coordinate);
                        }
                        assert_eq!(a[0], packed.0.final_a);
                        assert_eq!(b[0], packed.0.final_b);
                    }
                }
            });
        }
    }

    #[test]
    fn native_pair_messages_match_scalar_with_partial_vector_tails() {
        let mut rng = Rng::new(0xF05E_1F03);
        for n in [0, 1, 2, 3, 4, 6, 7, 8, 9, 10, 14, 16, 18, 1023, 1024, 1025] {
            let a = rng.f128_vec(n);
            let b = rng.f128_vec(n);
            assert_eq!(
                native_message_chunk(&a, &b).map(|sum| sum.reduce()),
                round_message(&a, &b)
            );
        }
    }

    #[test]
    fn uninitialized_parallel_output_remains_empty_after_panic_and_can_be_reused() {
        use rayon::prelude::{IndexedParallelIterator, ParallelIterator, ParallelSliceMut};
        use std::panic::{AssertUnwindSafe, catch_unwind};
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(3)
            .build()
            .unwrap();
        let mut output = vec![F128::ONE; 17];
        let mut other = Vec::new();
        let outcome = catch_unwind(AssertUnwindSafe(|| {
            pool.install(|| {
                let storage = prepare_output(&mut output, 8192);
                storage
                    .par_chunks_mut(1024)
                    .enumerate()
                    .for_each(|(chunk, values)| {
                        for value in values.iter_mut().take(37) {
                            value.write(F128::generator());
                        }
                        assert_ne!(chunk, 3, "injected failure after partial output writes");
                    });
            });
        }));
        assert!(outcome.is_err());
        assert!(output.is_empty());
        assert!(output.capacity() >= 8192);
        let mut rng = Rng::new(0xF05E_1F04);
        // Retry the actual fold path after interrupted initialization, then
        // shrink and grow both output buffers across parallel/tail boundaries.
        for n in [4, 32772, 8, 65540, 4] {
            let a = rng.f128_vec(n);
            let b = rng.f128_vec(n);
            let r = rng.f128();
            let mut expected_a = vec![F128::ZERO; n / 2];
            let mut expected_b = expected_a.clone();
            let expected = fold_and_message(&a, &b, &mut expected_a, &mut expected_b, r);
            let capacity_a = output.capacity();
            let capacity_b = other.capacity();
            let actual =
                pool.install(|| fold_and_message_native(&a, &b, &mut output, &mut other, r));
            assert_eq!(actual, expected);
            assert_eq!(output, expected_a);
            assert_eq!(other, expected_b);
            assert!(output.capacity() >= capacity_a);
            assert!(other.capacity() >= capacity_b);
        }
    }
}
