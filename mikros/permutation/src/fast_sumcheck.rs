//! Preserve the original cubic messages while factoring equality internally.
use crate::{
    product_tree::{CompactLayer, Tree},
    profile::{Timings, elapsed},
    sumcheck::{LayerProof, LeafClaim, TreeProof},
};
use flock_core::zerocheck::univariate_skip::SplitEqGhash;
use flock_field::{F128, F256Unreduced};
use flock_transcript::challenger::Challenger;
use rayon::prelude::*;
use std::{mem::swap, time::Instant};

#[derive(Clone, Copy)]
enum Input<'a> {
    Interleaved(&'a [F128]),
    Separate(&'a [F128], &'a [F128]),
    Compact(CompactLayer<'a>),
}
impl Input<'_> {
    fn len(self) -> usize {
        match self {
            Self::Interleaved(x) => x.len() / 2,
            Self::Separate(a, _) => a.len(),
            Self::Compact(x) => x.len() / 2,
        }
    }
    #[inline]
    fn at(self, i: usize) -> (F128, F128) {
        match self {
            Self::Interleaved(x) => (x[2 * i], x[2 * i + 1]),
            Self::Separate(a, b) => (a[i], b[i]),
            Self::Compact(x) => {
                let mut pair = [F128::ZERO; 2];
                x.read(2 * i, &mut pair);
                (pair[0], pair[1])
            }
        }
    }
}
fn eq(point: &[F128]) -> SplitEqGhash {
    SplitEqGhash::with_n_hi(point, point.len() / 2)
}
fn add(a: [F128; 3], b: [F128; 3]) -> [F128; 3] {
    std::array::from_fn(|i| a[i] + b[i])
}

#[derive(Clone, Copy)]
enum DenseInput<'a> {
    Interleaved(&'a [F128]),
    Separate(&'a [F128], &'a [F128]),
}
impl DenseInput<'_> {
    #[inline]
    fn at(self, i: usize) -> (F128, F128) {
        match self {
            Self::Interleaved(x) => (x[2 * i], x[2 * i + 1]),
            Self::Separate(a, b) => (a[i], b[i]),
        }
    }
}
#[inline(never)]
fn compact_block_message(src: CompactLayer<'_>, start: usize, weights: &[F128]) -> [F128; 3] {
    let mut values = [F128::ZERO; 1024];
    let mut result = [F128::ZERO; 3];
    for (block, w) in weights.chunks(256).enumerate() {
        let out = &mut values[..4 * w.len()];
        src.read(4 * (start + block * 256), out);
        result = add(
            result,
            dense_block_message(DenseInput::Interleaved(out), 0, w),
        );
    }
    result
}
fn block_message(input: Input<'_>, start: usize, weights: &[F128]) -> [F128; 3] {
    match input {
        Input::Interleaved(x) => dense_block_message(DenseInput::Interleaved(x), start, weights),
        Input::Separate(a, b) => dense_block_message(DenseInput::Separate(a, b), start, weights),
        Input::Compact(src) => compact_block_message(src, start, weights),
    }
}
fn dense_block_message(input: DenseInput<'_>, start: usize, weights: &[F128]) -> [F128; 3] {
    let mut sums = [F256Unreduced::ZERO; 3];
    #[allow(unused_mut)] // advanced by the cfg-gated four-lane kernel
    let mut cursor = 0;
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq"
    ))]
    {
        use flock_field::gf2_128::x86_64::{WideGhashX4, f128x4_loadu, f128x4_set, ghash_mul_x4};
        use std::arch::x86_64::_mm512_xor_si512;
        // SAFETY: all intrinsics are compile-time feature gated. The loop
        // reads four in-range pairs and four weights; no lanes are padded.
        unsafe {
            let mut accum = [WideGhashX4::zero(); 3];
            while cursor + 4 <= weights.len() {
                let lo: [(F128, F128); 4] =
                    std::array::from_fn(|j| input.at(2 * (start + cursor + j)));
                let hi: [(F128, F128); 4] =
                    std::array::from_fn(|j| input.at(2 * (start + cursor + j) + 1));
                let a0 = f128x4_set(lo[0].0, lo[1].0, lo[2].0, lo[3].0);
                let b0 = f128x4_set(lo[0].1, lo[1].1, lo[2].1, lo[3].1);
                let a1 = f128x4_set(hi[0].0, hi[1].0, hi[2].0, hi[3].0);
                let b1 = f128x4_set(hi[0].1, hi[1].1, hi[2].1, hi[3].1);
                let w = f128x4_loadu(weights.as_ptr().add(cursor));
                accum[0].mul_acc(ghash_mul_x4(a0, b0), w);
                accum[1].mul_acc(ghash_mul_x4(a1, b1), w);
                accum[2].mul_acc(
                    ghash_mul_x4(_mm512_xor_si512(a0, a1), _mm512_xor_si512(b0, b1)),
                    w,
                );
                cursor += 4;
            }
            for k in 0..3 {
                sums[k] ^= accum[k].fold();
            }
        }
    }
    for (j, &weight) in weights.iter().enumerate().skip(cursor) {
        let (a0, b0) = input.at(2 * (start + j));
        let (a1, b1) = input.at(2 * (start + j) + 1);
        let values = [a0 * b0, a1 * b1, (a0 + a1) * (b0 + b1)];
        for k in 0..3 {
            sums[k] ^= values[k].mul_unreduced(weight);
        }
    }
    sums.map(F256Unreduced::reduce)
}
fn message(input: Input<'_>, weights: &SplitEqGhash) -> [F128; 3] {
    assert_eq!(input.len() / 2, weights.lo.len() * weights.hi.len());
    let compute = |hi: usize| {
        block_message(input, hi * weights.lo.len(), &weights.lo).map(|x| x * weights.hi[hi])
    };
    if input.len() < 1 << 15 {
        (0..weights.hi.len())
            .map(compute)
            .fold([F128::ZERO; 3], add)
    } else {
        (0..weights.hi.len())
            .into_par_iter()
            .map(compute)
            .reduce(|| [F128::ZERO; 3], add)
    }
}

fn fold_block(input: Input<'_>, base: usize, a: &mut [F128], b: &mut [F128], r: F128) {
    match input {
        Input::Separate(x, y) => {
            flock_field::f128_slice::fold_pairs(x, base, a, r);
            flock_field::f128_slice::fold_pairs(y, base, b, r);
        }
        Input::Interleaved(src) => crate::kernels::fold_interleaved(src, base, a, b, r),
        Input::Compact(src) => {
            let mut values = [F128::ZERO; 1024];
            for (block, (a, b)) in a.chunks_mut(256).zip(b.chunks_mut(256)).enumerate() {
                let out = &mut values[..4 * a.len()];
                src.read(4 * (base + block * 256), out);
                crate::kernels::fold_interleaved(out, 0, a, b, r);
            }
        }
    }
}
fn fold_message(
    input: Input<'_>,
    a: &mut [F128],
    b: &mut [F128],
    r: F128,
    next: Option<&SplitEqGhash>,
) -> Option<[F128; 3]> {
    assert_eq!(input.len() / 2, a.len());
    assert_eq!(a.len(), b.len());
    if let Some(weights) = next {
        let chunk = 2 * weights.lo.len();
        assert_eq!(a.len(), chunk * weights.hi.len());
        let compute = |(hi, (a, b)): (usize, (&mut [F128], &mut [F128]))| {
            fold_block(input, hi * chunk, a, b, r);
            block_message(Input::Separate(a, b), 0, &weights.lo).map(|x| x * weights.hi[hi])
        };
        let result = if input.len() < 1 << 15 {
            a.chunks_mut(chunk)
                .zip(b.chunks_mut(chunk))
                .enumerate()
                .map(compute)
                .fold([F128::ZERO; 3], add)
        } else {
            a.par_chunks_mut(chunk)
                .zip(b.par_chunks_mut(chunk))
                .enumerate()
                .map(compute)
                .reduce(|| [F128::ZERO; 3], add)
        };
        Some(result)
    } else {
        fold_block(input, 0, a, b, r);
        None
    }
}

/// The same work partition and outer equality factors as `fold_message`, with
/// each block consuming folded values without rereading its output arrays.
fn fold_message_register(
    input: Input<'_>,
    a: &mut [F128],
    b: &mut [F128],
    r: F128,
    next: Option<&SplitEqGhash>,
) -> Option<[F128; 3]> {
    assert_eq!(input.len() / 2, a.len());
    assert_eq!(a.len(), b.len());
    let Some(weights) = next else {
        return fold_message(input, a, b, r, None);
    };
    let chunk = 2 * weights.lo.len();
    assert_eq!(a.len(), chunk * weights.hi.len());
    let compute = |(hi, (a, b)): (usize, (&mut [F128], &mut [F128]))| {
        fold_message_register_block(input, hi * chunk, a, b, r, &weights.lo)
            .map(|x| x * weights.hi[hi])
    };
    Some(if input.len() < 1 << 15 {
        a.chunks_mut(chunk)
            .zip(b.chunks_mut(chunk))
            .enumerate()
            .map(compute)
            .fold([F128::ZERO; 3], add)
    } else {
        a.par_chunks_mut(chunk)
            .zip(b.par_chunks_mut(chunk))
            .enumerate()
            .map(compute)
            .reduce(|| [F128::ZERO; 3], add)
    })
}

fn fold_message_register_block(
    input: Input<'_>,
    base: usize,
    a: &mut [F128],
    b: &mut [F128],
    r: F128,
    weights: &[F128],
) -> [F128; 3] {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len() % 2, 0);
    assert_eq!(a.len() / 2, weights.len());
    assert!(base <= input.len() / 2 && a.len() <= input.len() / 2 - base);
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "pclmulqdq",
        target_feature = "sse4.1"
    ))]
    // SAFETY: the feature gate covers the complete GHASH/fold backend. Slice
    // bounds and exclusive output borrows are preserved by native::block.
    unsafe {
        native::block(input, base, a, b, r, weights)
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
        fold_block(input, base, a, b, r);
        dense_block_message(DenseInput::Separate(a, b), 0, weights)
    }
}

#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512f",
    target_feature = "avx512bw",
    target_feature = "vpclmulqdq",
    target_feature = "pclmulqdq",
    target_feature = "sse4.1"
))]
mod native {
    use super::{F128, F256Unreduced, Input};
    use flock_field::gf2_128::x86_64::{WideGhashX4, f128x4_loadu, f128x4_set, ghash_mul_x4};
    use std::arch::x86_64::{
        __m512i, _mm512_permutex2var_epi64, _mm512_set_epi64, _mm512_shuffle_i32x4,
        _mm512_storeu_si512, _mm512_xor_si512,
    };

    struct Accumulator {
        wide: [WideGhashX4; 3],
        tail: [F256Unreduced; 3],
    }

    #[inline]
    #[target_feature(enable = "avx512f,avx512bw,vpclmulqdq,pclmulqdq,sse4.1")]
    unsafe fn separate(lo: __m512i, hi: __m512i) -> (__m512i, __m512i) {
        let even = _mm512_set_epi64(13, 12, 9, 8, 5, 4, 1, 0);
        let odd = _mm512_set_epi64(15, 14, 11, 10, 7, 6, 3, 2);
        (
            _mm512_permutex2var_epi64(lo, even, hi),
            _mm512_permutex2var_epi64(lo, odd, hi),
        )
    }

    /// Fold eight adjacent values of one source into four output lanes.
    #[inline]
    #[target_feature(enable = "avx512f,avx512bw,vpclmulqdq,pclmulqdq,sse4.1")]
    unsafe fn fold_eight(source: *const F128, r: __m512i) -> __m512i {
        // SAFETY: caller supplies eight readable F128 and the required ISA.
        unsafe {
            let (even, odd) = separate(f128x4_loadu(source), f128x4_loadu(source.add(4)));
            _mm512_xor_si512(even, ghash_mul_x4(r, _mm512_xor_si512(even, odd)))
        }
    }

    /// Fold [a0,b0,a1,b1,a2,b2,a3,b3] to [a0',b0',a1',b1'].
    #[inline]
    #[target_feature(enable = "avx512f,avx512bw,vpclmulqdq,pclmulqdq,sse4.1")]
    unsafe fn fold_interleaved_eight(source: *const F128, r: __m512i) -> __m512i {
        // SAFETY: caller supplies eight readable F128. Immediate shuffles
        // select whole 128-bit field elements, preserving their GHASH limbs.
        unsafe {
            let lo = f128x4_loadu(source);
            let hi = f128x4_loadu(source.add(4));
            let even = _mm512_shuffle_i32x4::<0x44>(lo, hi);
            let odd = _mm512_shuffle_i32x4::<0xee>(lo, hi);
            _mm512_xor_si512(even, ghash_mul_x4(r, _mm512_xor_si512(even, odd)))
        }
    }

    /// Consume an exact dense window; each weight covers four inputs of A
    /// and B and two outputs of each. INTERLEAVED is compile-time dispatch.
    #[inline]
    #[target_feature(enable = "avx512f,avx512bw,vpclmulqdq,pclmulqdq,sse4.1")]
    unsafe fn dense<const INTERLEAVED: bool>(
        source_a: &[F128],
        source_b: &[F128],
        a: &mut [F128],
        b: &mut [F128],
        challenge: F128,
        weights: &[F128],
        accum: &mut Accumulator,
    ) {
        debug_assert_eq!(a.len(), 2 * weights.len());
        debug_assert_eq!(a.len(), b.len());
        debug_assert_eq!(source_a.len(), a.len() * if INTERLEAVED { 4 } else { 2 });
        debug_assert!(INTERLEAVED || source_b.len() == 2 * b.len());
        // SAFETY: block constructs exact input/output windows. A vector step
        // reads four weights, sixteen values from each source, and writes eight
        // values of each output. Scalar tails write all remaining slots.
        unsafe {
            let r = f128x4_set(challenge, challenge, challenge, challenge);
            let [mut zero, mut one, mut infinity] = accum.wide;
            let mut cursor = 0;
            while cursor + 4 <= weights.len() {
                let offset = cursor * 4;
                let (al, ah, bl, bh) = if INTERLEAVED {
                    let source = source_a.as_ptr().add(offset * 2);
                    let (al, bl) = separate(
                        fold_interleaved_eight(source, r),
                        fold_interleaved_eight(source.add(8), r),
                    );
                    let (ah, bh) = separate(
                        fold_interleaved_eight(source.add(16), r),
                        fold_interleaved_eight(source.add(24), r),
                    );
                    (al, ah, bl, bh)
                } else {
                    (
                        fold_eight(source_a.as_ptr().add(offset), r),
                        fold_eight(source_a.as_ptr().add(offset + 8), r),
                        fold_eight(source_b.as_ptr().add(offset), r),
                        fold_eight(source_b.as_ptr().add(offset + 8), r),
                    )
                };
                let output = 2 * cursor;
                _mm512_storeu_si512(a.as_mut_ptr().add(output).cast(), al);
                _mm512_storeu_si512(a.as_mut_ptr().add(output + 4).cast(), ah);
                _mm512_storeu_si512(b.as_mut_ptr().add(output).cast(), bl);
                _mm512_storeu_si512(b.as_mut_ptr().add(output + 4).cast(), bh);
                let (a0, a1) = separate(al, ah);
                let (b0, b1) = separate(bl, bh);
                let weight = f128x4_loadu(weights.as_ptr().add(cursor));
                zero.mul_acc(ghash_mul_x4(a0, b0), weight);
                one.mul_acc(ghash_mul_x4(a1, b1), weight);
                infinity.mul_acc(
                    ghash_mul_x4(_mm512_xor_si512(a0, a1), _mm512_xor_si512(b0, b1)),
                    weight,
                );
                cursor += 4;
            }
            accum.wide = [zero, one, infinity];
            for (j, &weight) in weights.iter().enumerate().skip(cursor) {
                let (a0, a1, b0, b1) = if INTERLEAVED {
                    let x = &source_a[8 * j..8 * j + 8];
                    (
                        x[0] + challenge * (x[0] + x[2]),
                        x[4] + challenge * (x[4] + x[6]),
                        x[1] + challenge * (x[1] + x[3]),
                        x[5] + challenge * (x[5] + x[7]),
                    )
                } else {
                    let x = &source_a[4 * j..4 * j + 4];
                    let y = &source_b[4 * j..4 * j + 4];
                    (
                        x[0] + challenge * (x[0] + x[1]),
                        x[2] + challenge * (x[2] + x[3]),
                        y[0] + challenge * (y[0] + y[1]),
                        y[2] + challenge * (y[2] + y[3]),
                    )
                };
                a[2 * j] = a0;
                a[2 * j + 1] = a1;
                b[2 * j] = b0;
                b[2 * j + 1] = b1;
                for (sum, product) in
                    accum
                        .tail
                        .iter_mut()
                        .zip([a0 * b0, a1 * b1, (a0 + a1) * (b0 + b1)])
                {
                    *sum ^= product.mul_unreduced(weight);
                }
            }
        }
    }

    #[inline(never)]
    #[target_feature(enable = "avx512f,avx512bw,vpclmulqdq,pclmulqdq,sse4.1")]
    pub(super) unsafe fn block(
        input: Input<'_>,
        base: usize,
        a: &mut [F128],
        b: &mut [F128],
        r: F128,
        weights: &[F128],
    ) -> [F128; 3] {
        // SAFETY: the safe wrapper checked lengths. Exact subslices here also
        // enforce both source bounds. No output is read before it is written.
        unsafe {
            let mut accum = Accumulator {
                wide: [WideGhashX4::zero(); 3],
                tail: [F256Unreduced::ZERO; 3],
            };
            match input {
                Input::Separate(x, y) => dense::<false>(
                    &x[2 * base..2 * (base + a.len())],
                    &y[2 * base..2 * (base + b.len())],
                    a,
                    b,
                    r,
                    weights,
                    &mut accum,
                ),
                Input::Interleaved(x) => dense::<true>(
                    &x[4 * base..4 * (base + a.len())],
                    &[],
                    a,
                    b,
                    r,
                    weights,
                    &mut accum,
                ),
                Input::Compact(src) => {
                    let mut values = [F128::ZERO; 1024];
                    for (block, ((a, b), w)) in a
                        .chunks_mut(256)
                        .zip(b.chunks_mut(256))
                        .zip(weights.chunks(128))
                        .enumerate()
                    {
                        let decoded = &mut values[..4 * a.len()];
                        src.read(4 * (base + block * 256), decoded);
                        dense::<true>(decoded, &[], a, b, r, w, &mut accum);
                    }
                }
            }
            // Keep reduction outside both the dense loop and the compact
            // decode loop. The nine wide lanes may cross a decode call through
            // memory; inner-loop register residency needs assembly validation.
            std::array::from_fn(|k| (accum.tail[k] ^ accum.wide[k].fold()).reduce())
        }
    }
}

fn cubic(core: [F128; 3], parent: F128, prefix: F128) -> [F128; 3] {
    let [g0, g1, ginf] = core;
    [
        prefix * (F128::ONE + parent) * g0,
        prefix * (parent * ginf + g0 + g1),
        prefix * ginf,
    ]
}
fn evaluate([g0, g2, g3]: [F128; 3], claim: F128, t: F128) -> F128 {
    g0 + t * (claim + g2 + g3 + t * (g2 + t * g3))
}

pub(crate) fn prove<C: Challenger>(
    tree: &mut impl Tree,
    c: &mut C,
    p: &mut Timings,
    stream: bool,
) -> (TreeProof, LeafClaim) {
    let ([proof], [claim]) = prove_impl::<false, 1, C>([tree], c, p, stream, true);
    (proof, claim)
}

pub(crate) fn prove_register<C: Challenger>(
    tree: &mut impl Tree,
    c: &mut C,
    p: &mut Timings,
    stream: bool,
) -> (TreeProof, LeafClaim) {
    let ([proof], [claim]) = prove_impl::<true, 1, C>([tree], c, p, stream, true);
    (proof, claim)
}

/// Two independently checked reductions with a common point at every layer.
/// Both messages precede each shared challenge; the arithmetic kernels are
/// exactly those used by the sequential prover.
pub(crate) fn prove_pair<C: Challenger>(
    left: &mut impl Tree,
    right: &mut impl Tree,
    c: &mut C,
    p: &mut Timings,
    stream: bool,
    register: bool,
) -> ([TreeProof; 2], [LeafClaim; 2]) {
    if register {
        prove_impl::<true, 2, C>([left, right], c, p, stream, true)
    } else {
        prove_impl::<false, 2, C>([left, right], c, p, stream, true)
    }
}

/// Identical messages and folding kernels; release checked-out scratch rather
/// than retaining it after the tree phase of a consuming packed proof.
pub(crate) fn prove_pair_bounded<C: Challenger>(
    left: &mut impl Tree,
    right: &mut impl Tree,
    c: &mut C,
    p: &mut Timings,
) -> ([TreeProof; 2], [LeafClaim; 2]) {
    prove_impl::<true, 2, C>([left, right], c, p, true, false)
}

fn prove_impl<const REGISTER_FOLD: bool, const N: usize, C: Challenger>(
    mut trees: [&mut dyn Tree; N],
    c: &mut C,
    p: &mut Timings,
    stream: bool,
    recycle: bool,
) -> ([TreeProof; N], [LeafClaim; N]) {
    let log_len = trees[0].log_len();
    assert!(trees.iter().all(|tree| tree.log_len() == log_len));
    let mut point = Vec::new();
    let mut claims = std::array::from_fn::<_, N, _>(|j| trees[j].root());
    let mut layers = std::array::from_fn::<_, N, _>(|_| Vec::with_capacity(log_len));
    let capacity = if log_len > 1 { 1 << (log_len - 2) } else { 0 };
    // Reuse Flock's scratch pool. Only the prefixes written by fold_message
    // are ever read; untouched capacity is never inspected.
    let t = Instant::now();
    // Each tree owns its fold buffers. Compact layers remain borrowed; no
    // source, coefficient, or whole-tree copy is needed for synchronization.
    let mut cur = std::array::from_fn::<_, N, _>(|_| {
        std::array::from_fn::<_, 2, _>(|_| flock_core::scratch::take_f128(capacity))
    });
    let mut next = std::array::from_fn::<_, N, _>(|_| {
        std::array::from_fn::<_, 2, _>(|_| flock_core::scratch::take_f128(capacity))
    });
    p.scratch_setup_ms += elapsed(t);
    for level in (0..log_len).rev() {
        let t = Instant::now();
        let compact = std::array::from_fn::<_, N, _>(|j| {
            stream.then(|| trees[j].compact_layer(level)).flatten()
        });
        let children =
            std::array::from_fn::<_, N, _>(|j| compact[j].is_none().then(|| trees[j].layer(level)));
        let inputs = std::array::from_fn::<_, N, _>(|j| match compact[j] {
            Some(view) => Input::Compact(view),
            None => Input::Interleaved(children[j].as_ref().unwrap()),
        });
        p.layer_materialize_ms += elapsed(t);
        let k = point.len();
        let h = 1usize << k;
        let mut rounds = std::array::from_fn::<_, N, _>(|_| Vec::with_capacity(k));
        let mut q = Vec::with_capacity(k);
        let mut prefix = F128::ONE;
        let mut pending = if k > 0 {
            let t = Instant::now();
            let w = eq(&point[1..]);
            p.equality_ms += elapsed(t);
            let t = Instant::now();
            let v = inputs.map(|input| Some(message(input, &w)));
            p.messages_ms += elapsed(t);
            v
        } else {
            [None; N]
        };
        let mut len = 0;
        for i in 0..k {
            let messages = std::array::from_fn::<_, N, _>(|j| {
                cubic(pending[j].take().unwrap(), point[i], prefix)
            });
            for g in &messages {
                c.observe_f128_slice(g);
            }
            let r = c.sample_f128();
            for j in 0..N {
                claims[j] = evaluate(messages[j], claims[j], r);
                rounds[j].push(messages[j]);
            }
            q.push(r);
            prefix *= F128::ONE + point[i] + r;
            let t = Instant::now();
            let w = (i + 1 < k).then(|| eq(&point[i + 2..]));
            p.equality_ms += elapsed(t);
            let t = Instant::now();
            if i == 0 {
                len = h / 2;
                for j in 0..N {
                    let [a, b] = &mut cur[j];
                    pending[j] = if REGISTER_FOLD {
                        fold_message_register(
                            inputs[j],
                            &mut a[..len],
                            &mut b[..len],
                            r,
                            w.as_ref(),
                        )
                    } else {
                        fold_message(inputs[j], &mut a[..len], &mut b[..len], r, w.as_ref())
                    };
                }
            } else {
                let old_len = len;
                len /= 2;
                for j in 0..N {
                    let input = Input::Separate(&cur[j][0][..old_len], &cur[j][1][..old_len]);
                    let [a, b] = &mut next[j];
                    pending[j] = if REGISTER_FOLD {
                        fold_message_register(input, &mut a[..len], &mut b[..len], r, w.as_ref())
                    } else {
                        fold_message(input, &mut a[..len], &mut b[..len], r, w.as_ref())
                    };
                }
                swap(&mut cur, &mut next);
            }
            p.fold_and_message_ms += elapsed(t);
        }
        let boundaries = std::array::from_fn::<_, N, _>(|j| {
            if k == 0 {
                inputs[j].at(0)
            } else {
                (cur[j][0][0], cur[j][1][0])
            }
        });
        for j in 0..N {
            let (a, b) = boundaries[j];
            debug_assert_eq!(claims[j], prefix * a * b);
            c.observe_f128_slice(&[a, b]);
        }
        let rho = c.sample_f128();
        for j in 0..N {
            let (a, b) = boundaries[j];
            claims[j] = a + rho * (a + b);
        }
        point = vec![rho];
        point.extend(q);
        for (j, rounds) in rounds.into_iter().enumerate() {
            layers[j].push(LayerProof {
                rounds,
                left: boundaries[j].0,
                right: boundaries[j].1,
            });
        }
        let t = Instant::now();
        drop(children);
        for tree in &mut trees {
            tree.release_layer(level);
        }
        p.layer_release_ms += elapsed(t);
    }
    for buffer in cur.into_iter().chain(next).flatten() {
        if recycle {
            flock_core::scratch::give_f128(buffer);
        }
    }
    (
        layers.map(|layers| TreeProof { layers }),
        claims.map(|value| LeafClaim {
            point: point.clone(),
            value,
        }),
    )
}

/// One independently checked weighted-Hadamard layer on each of two sides.
/// This shares the existing native message/fold kernels and a synchronized
/// challenge sequence without changing the legacy whole-tree entry points.
pub(crate) fn prove_weighted_pair<C: Challenger>(
    a: [&[F128]; 2],
    b: [&[F128]; 2],
    point: &[F128],
    claims: [F128; 2],
    c: &mut C,
    p: &mut Timings,
) -> ([LayerProof; 2], Vec<F128>, [F128; 2]) {
    prove_weighted_pair_with_recycling(a, b, point, claims, c, p, true)
}

pub(crate) fn prove_weighted_pair_with_recycling<C: Challenger>(
    a: [&[F128]; 2],
    b: [&[F128]; 2],
    point: &[F128],
    mut claims: [F128; 2],
    c: &mut C,
    p: &mut Timings,
    recycle: bool,
) -> ([LayerProof; 2], Vec<F128>, [F128; 2]) {
    let h = 1usize << point.len();
    assert!(a.iter().chain(&b).all(|x| x.len() == h));
    let inputs = std::array::from_fn::<_, 2, _>(|j| Input::Separate(a[j], b[j]));
    let capacity = h / 2;
    let start = Instant::now();
    let mut cur = std::array::from_fn::<_, 2, _>(|_| {
        std::array::from_fn::<_, 2, _>(|_| flock_core::scratch::take_f128(capacity))
    });
    let mut next = std::array::from_fn::<_, 2, _>(|_| {
        std::array::from_fn::<_, 2, _>(|_| flock_core::scratch::take_f128(capacity))
    });
    p.scratch_setup_ms += elapsed(start);
    let mut rounds = [
        Vec::with_capacity(point.len()),
        Vec::with_capacity(point.len()),
    ];
    let mut q = Vec::with_capacity(point.len() + 1);
    let mut prefix = F128::ONE;
    let mut pending = if !point.is_empty() {
        let start = Instant::now();
        let weights = eq(&point[1..]);
        p.equality_ms += elapsed(start);
        let start = Instant::now();
        let messages = inputs.map(|x| Some(message(x, &weights)));
        p.messages_ms += elapsed(start);
        messages
    } else {
        [None; 2]
    };
    let mut len = 0;
    for i in 0..point.len() {
        let messages =
            std::array::from_fn::<_, 2, _>(|j| cubic(pending[j].take().unwrap(), point[i], prefix));
        for message in &messages {
            c.observe_f128_slice(message);
        }
        let r = c.sample_f128();
        q.push(r);
        for j in 0..2 {
            claims[j] = evaluate(messages[j], claims[j], r);
            rounds[j].push(messages[j]);
        }
        prefix *= F128::ONE + point[i] + r;
        let start = Instant::now();
        let weights = (i + 1 < point.len()).then(|| eq(&point[i + 2..]));
        p.equality_ms += elapsed(start);
        let start = Instant::now();
        if i == 0 {
            len = h / 2;
            for j in 0..2 {
                let [out_a, out_b] = &mut cur[j];
                pending[j] = fold_message_register(
                    inputs[j],
                    &mut out_a[..len],
                    &mut out_b[..len],
                    r,
                    weights.as_ref(),
                );
            }
        } else {
            let old = len;
            len /= 2;
            for j in 0..2 {
                let input = Input::Separate(&cur[j][0][..old], &cur[j][1][..old]);
                let [out_a, out_b] = &mut next[j];
                pending[j] = fold_message_register(
                    input,
                    &mut out_a[..len],
                    &mut out_b[..len],
                    r,
                    weights.as_ref(),
                );
            }
            swap(&mut cur, &mut next);
        }
        p.fold_and_message_ms += elapsed(start);
    }
    let boundary = std::array::from_fn::<_, 2, _>(|j| {
        if point.is_empty() {
            inputs[j].at(0)
        } else {
            (cur[j][0][0], cur[j][1][0])
        }
    });
    for j in 0..2 {
        debug_assert_eq!(claims[j], prefix * boundary[j].0 * boundary[j].1);
        c.observe_f128_slice(&[boundary[j].0, boundary[j].1]);
    }
    let rho = c.sample_f128();
    for j in 0..2 {
        claims[j] = boundary[j].0 + rho * (boundary[j].0 + boundary[j].1);
    }
    q.insert(0, rho);
    for buffer in cur.into_iter().chain(next).flatten() {
        if recycle {
            flock_core::scratch::give_f128(buffer);
        }
    }
    (
        rounds
            .into_iter()
            .enumerate()
            .map(|(j, rounds)| LayerProof {
                rounds,
                left: boundary[j].0,
                right: boundary[j].1,
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
        q,
        claims,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bits::PackedBits,
        product_tree::{Options, ProductTree},
        profile::Engine,
    };

    fn value(i: usize) -> F128 {
        let mut x = (i as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
        x ^= x >> 17;
        x ^= x << 13;
        F128::new(x, x.rotate_left(23) ^ 0xd1b5_4a32_d192_ed03)
    }

    fn check_register_block(input: Input<'_>, base: usize, count: usize, challenge: F128) {
        let weights: Vec<_> = (0..count)
            .map(|i| match i % 11 {
                0 => F128::ZERO,
                1 => F128::ONE,
                _ => value(i + 391),
            })
            .collect();
        let mut expected_a = Vec::with_capacity(2 * count);
        let mut expected_b = Vec::with_capacity(2 * count);
        for i in 0..2 * count {
            let (a0, b0) = input.at(2 * (base + i));
            let (a1, b1) = input.at(2 * (base + i) + 1);
            expected_a.push(a0 + challenge * (a0 + a1));
            expected_b.push(b0 + challenge * (b0 + b1));
        }
        let mut expected = [F128::ZERO; 3];
        for (i, &weight) in weights.iter().enumerate() {
            let [a0, a1] = [expected_a[2 * i], expected_a[2 * i + 1]];
            let [b0, b1] = [expected_b[2 * i], expected_b[2 * i + 1]];
            expected[0] += weight * (a0 * b0);
            expected[1] += weight * (a1 * b1);
            expected[2] += weight * ((a0 + a1) * (b0 + b1));
        }
        // Both outputs start one F128 into their allocations, exercising
        // unaligned ZMM stores and canaries on each side of the owned prefix.
        let canary = value(9987);
        let mut actual_a = vec![canary; 2 * count + 2];
        let mut actual_b = vec![canary; 2 * count + 2];
        let actual = fold_message_register_block(
            input,
            base,
            &mut actual_a[1..1 + 2 * count],
            &mut actual_b[1..1 + 2 * count],
            challenge,
            &weights,
        );
        assert_eq!(
            actual, expected,
            "weights={count},base={base},r={challenge:?}"
        );
        assert_eq!(&actual_a[1..1 + 2 * count], expected_a);
        assert_eq!(&actual_b[1..1 + 2 * count], expected_b);
        assert_eq!([actual_a[0], actual_a[2 * count + 1]], [canary; 2]);
        assert_eq!([actual_b[0], actual_b[2 * count + 1]], [canary; 2]);

        let mut old_a = vec![canary; 2 * count];
        let mut old_b = vec![canary; 2 * count];
        fold_block(input, base, &mut old_a, &mut old_b, challenge);
        assert_eq!(old_a, expected_a);
        assert_eq!(old_b, expected_b);
        assert_eq!(
            dense_block_message(DenseInput::Separate(&old_a, &old_b), 0, &weights),
            expected
        );
    }

    #[test]
    fn register_fold_dense_offsets_tails_and_special_challenges_match_scalar() {
        for count in [0, 1, 2, 3, 4, 5, 7, 8, 9, 31, 32, 33, 127, 128, 129] {
            for base in [0, 1, 3] {
                let len = 2 * base + 4 * count + 3;
                let x: Vec<_> = (0..len + 1).map(|i| value(i + 13)).collect();
                let y: Vec<_> = (0..len + 1).map(|i| value(i + 2741)).collect();
                let mut interleaved = vec![value(999)];
                for (&a, &b) in x[1..].iter().zip(&y[1..]) {
                    interleaved.extend([a, b]);
                }
                for challenge in [F128::ZERO, F128::ONE, value(813)] {
                    check_register_block(Input::Separate(&x[1..], &y[1..]), base, count, challenge);
                    check_register_block(
                        Input::Interleaved(&interleaved[1..]),
                        base,
                        count,
                        challenge,
                    );
                }
            }
        }
    }

    #[test]
    fn register_fold_compact_decode_boundaries_match_scalar() {
        // Include both K32 compact levels and the K64 leaf level; partial
        // original rows exercise the same zero padding used by the prover.
        for width in [17, 31, 32, 64] {
            let bits = PackedBits::from_fn_parallel(4101, width, |i| {
                let x = value(i + 917);
                (x.lo as u128) & ((1u128 << width) - 1)
            })
            .unwrap();
            let tree = ProductTree::from_bits(
                &bits,
                value(81),
                Options::default(),
                Engine::Fused,
                &mut Timings::default(),
            )
            .unwrap();
            for level in 0..tree.stats.switch_level {
                let compact = tree.compact_layer(level).unwrap();
                for count in [0, 1, 2, 3, 4, 7, 8, 127, 128, 129, 255] {
                    for challenge in [F128::ZERO, F128::ONE, value(1391)] {
                        check_register_block(Input::Compact(compact), 3, count, challenge);
                        // Read an exact suffix, including padded zero leaves.
                        let base = compact.len() / 4 - 2 * count;
                        check_register_block(Input::Compact(compact), base, count, challenge);
                    }
                }
            }
        }
    }

    #[test]
    fn register_fold_outer_eq_parallel_dispatch_and_final_round_match_baseline() {
        for log_weights in [0, 1, 3, 7, 12, 14] {
            let point: Vec<_> = (0..log_weights).map(|i| value(i + 185)).collect();
            let weights = eq(&point);
            let len = 4 * (1 << log_weights);
            let x: Vec<_> = (0..len).map(|i| value(i + 179)).collect();
            let y: Vec<_> = (0..len).map(|i| value(i + 587)).collect();
            let interleaved: Vec<_> = x.iter().zip(&y).flat_map(|(&a, &b)| [a, b]).collect();
            for input in [Input::Separate(&x, &y), Input::Interleaved(&interleaved)] {
                for next in [None, Some(&weights)] {
                    let mut expected_a = vec![F128::ZERO; len / 2];
                    let mut expected_b = vec![F128::ZERO; len / 2];
                    let expected =
                        fold_message(input, &mut expected_a, &mut expected_b, value(1949), next);
                    for threads in [1, 4] {
                        rayon::ThreadPoolBuilder::new()
                            .num_threads(threads)
                            .build()
                            .unwrap()
                            .install(|| {
                                let mut a = vec![F128::ONE; len / 2];
                                let mut b = vec![F128::ONE; len / 2];
                                assert_eq!(
                                    fold_message_register(input, &mut a, &mut b, value(1949), next),
                                    expected
                                );
                                assert_eq!(a, expected_a);
                                assert_eq!(b, expected_b);
                            });
                    }
                }
            }
        }
    }
}
