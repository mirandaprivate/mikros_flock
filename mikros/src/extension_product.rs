//! Two independent extension-field product-tree reductions, as in the
//! multiple-instance construction in `latex/6-together.tex`.
//!
//! The caller binds the circuit and both source commitments, samples any
//! fingerprint challenges, and supplies an immutable leaf function. Leaves
//! are indexed in their original physical order. A parent of a length-2h
//! layer is `parent[i] = child[i] * child[h+i]` (high-bit pairing). Sumcheck
//! binds low bits first, and returned points use that same low-bit-first order.
//! The caller expands its affine leaf expression and authenticates both returned
//! leaf claims with the PCS.
//!
//! Only one internal tree is active. For log_len<=28 a per-proof pool may keep
//! consumed layers' capacities for reuse; it releases them before returning.
//! `MIKROS_TREE_LARGE_POOL=1` extends this exact-capacity policy through log 32.
//! The complete leaf layer is never
//! allocated: its first products and its final sumcheck are streamed from the
//! leaf function. The second tree is constructed after the first reduction;
//! its root must equal the already bound root. Sources must therefore be
//! deterministic and fixed throughout the call. All this work is online.
//! There is no division by equality weights and no implicit grinding policy.

use flock_core::zerocheck::univariate_skip::SplitEqGhash;
use flock_field::{F128, F256Unreduced};
use flock_transcript::challenger::Challenger;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    mem::{MaybeUninit, size_of},
    time::Instant,
};

const DOMAIN: &[u8] = b"mikros-extension-product-sequential-high-pair-v1";
const CHUNK: usize = 4096;

#[cfg(test)]
thread_local! {
    static TREE_FUSION_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
    static TREE_POOL_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
    static TREE_DENSE_SIMD_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
    static TREE_REGISTER_FUSION_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
}

fn tree_register_fusion_enabled() -> bool {
    #[cfg(test)]
    if let Some(enabled) = TREE_REGISTER_FUSION_OVERRIDE.get() {
        return enabled;
    }
    std::env::var_os("MIKROS_TREE_REGISTER_FUSION").is_some_and(|value| value == "1")
}

#[cfg(test)]
fn with_tree_register_fusion<T>(enabled: bool, run: impl FnOnce() -> T) -> T {
    struct Reset(Option<bool>);
    impl Drop for Reset {
        fn drop(&mut self) {
            TREE_REGISTER_FUSION_OVERRIDE.set(self.0);
        }
    }
    let _reset = Reset(TREE_REGISTER_FUSION_OVERRIDE.replace(Some(enabled)));
    run()
}

fn tree_dense_simd_enabled() -> bool {
    #[cfg(test)]
    if let Some(enabled) = TREE_DENSE_SIMD_OVERRIDE.get() {
        return enabled;
    }
    std::env::var_os("MIKROS_TREE_DENSE_SIMD").is_some_and(|value| value == "1")
}

#[cfg(test)]
fn with_tree_dense_simd<T>(enabled: bool, run: impl FnOnce() -> T) -> T {
    struct Reset(Option<bool>);
    impl Drop for Reset {
        fn drop(&mut self) {
            TREE_DENSE_SIMD_OVERRIDE.set(self.0);
        }
    }
    let _reset = Reset(TREE_DENSE_SIMD_OVERRIDE.replace(Some(enabled)));
    run()
}

fn tree_pool_enabled(log_len: usize) -> bool {
    // Large-tree reuse is an explicit experiment. At log L=32 the exact
    // capacity pool owns at most 96 GiB, and is released before native PCS.
    // Leave the established large-tree policy unchanged unless requested.
    let large = std::env::var_os("MIKROS_TREE_LARGE_POOL").is_some_and(|v| v == "1");
    if !tree_pool_size_allowed(log_len, large) {
        return false;
    }
    #[cfg(test)]
    if let Some(enabled) = TREE_POOL_OVERRIDE.get() {
        return enabled;
    }
    std::env::var_os("MIKROS_NO_TREE_POOL").is_none()
}

fn tree_pool_size_allowed(log_len: usize, large: bool) -> bool {
    log_len <= if large { 32 } else { 28 }
}

#[cfg(test)]
fn with_tree_pool_override<T>(enabled: bool, f: impl FnOnce() -> T) -> T {
    struct Reset(Option<bool>);
    impl Drop for Reset {
        fn drop(&mut self) {
            TREE_POOL_OVERRIDE.set(self.0);
        }
    }
    let _reset = Reset(TREE_POOL_OVERRIDE.replace(Some(enabled)));
    f()
}

fn tree_fusion_enabled() -> bool {
    #[cfg(test)]
    if let Some(enabled) = TREE_FUSION_OVERRIDE.get() {
        return enabled;
    }
    std::env::var_os("MIKROS_NO_TREE_FUSION").is_none()
}

#[cfg(test)]
fn with_tree_fusion_override<T>(enabled: bool, f: impl FnOnce() -> T) -> T {
    struct Reset(Option<bool>);
    impl Drop for Reset {
        fn drop(&mut self) {
            TREE_FUSION_OVERRIDE.set(self.0);
        }
    }
    let _reset = Reset(TREE_FUSION_OVERRIDE.replace(Some(enabled)));
    f()
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeafClaim {
    pub point: Vec<F128>,
    pub value: F128,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayerProof {
    /// Cubic coefficients [g(0), g_2, g_3]; g_1 follows from g(0)+g(1)=claim.
    pub rounds: Vec<[F128; 3]>,
    pub children: [F128; 2],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TreeProof {
    /// Root first: layer k contains exactly k sumcheck rounds.
    pub layers: Vec<LayerProof>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    pub root: F128,
    pub trees: [TreeProof; 2],
}

impl Proof {
    pub fn field_elements(&self) -> usize {
        1 + self
            .trees
            .iter()
            .flat_map(|tree| &tree.layers)
            .map(|layer| 3 * layer.rounds.len() + 2)
            .sum::<usize>()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Timings {
    pub construction_ms: f64,
    pub root_ms: f64,
    pub reduction_ms: f64,
    /// Included in reduction_ms; do not add it again.
    pub release_ms: f64,
    /// Maximum Vec<F128> capacity owned by this module, INCLUDING idle buffers
    /// in the per-proof reuse pool, excluding the
    /// caller's source, proof, small split equality tables and allocator state.
    /// For L>=2, exact-capacity pooling needs at most 3L/2-2 elements: one L/2
    /// buffer and two of each smaller power of two. Without pooling the bound
    /// is L-1. All retained buffers are dropped before prove_pair returns.
    /// This is not a process-RSS measurement; larger trees (log L>28) do not
    /// enable pooling automatically. The explicit large-pool experiment extends
    /// the same bound through log L=32 (96 GiB); the caller's leaf cache is extra.
    pub peak_field_buffer_bytes: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Length,
    Allocation,
    ProductMismatch,
    ProofShape,
    LayerCheck,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "extension product proof: {self:?}")
    }
}
impl std::error::Error for Error {}

fn length(log_len: usize) -> Result<usize, Error> {
    if log_len >= usize::BITS as usize {
        return Err(Error::Length);
    }
    let len = 1usize << log_len;
    if len
        .checked_mul(size_of::<F128>())
        .is_none_or(|n| n > isize::MAX as usize)
    {
        return Err(Error::Length);
    }
    Ok(len)
}

fn elapsed(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

fn bind<C: Challenger>(log_len: usize, c: &mut C) {
    c.observe_label(DOMAIN);
    c.observe_bytes(&(log_len as u64).to_le_bytes());
}

fn bind_tree<C: Challenger>(side: usize, c: &mut C) {
    c.observe_label(b"mikros-extension-product-tree-v1");
    c.observe_bytes(&(side as u64).to_le_bytes());
}

fn init_vec<F>(len: usize, buffers: &mut Buffers, fill: F) -> Result<Vec<F128>, Error>
where
    F: Fn(usize, &mut [MaybeUninit<F128>]) + Sync,
{
    let mut values = buffers.acquire(len)?;
    let out = &mut values.spare_capacity_mut()[..len];
    if len < CHUNK {
        fill(0, out);
    } else {
        out.par_chunks_mut(CHUNK)
            .enumerate()
            .for_each(|(block, dst)| fill(block * CHUNK, dst));
    }
    // SAFETY: each infallible fill writes every slot in its disjoint chunk.
    // If it panics, Vec still has length zero and never reads uninitialized data.
    unsafe { values.set_len(len) };
    Ok(values)
}

struct Buffers {
    live: usize,
    pooled: usize,
    peak: usize,
    reuse: bool,
    pool: Vec<Vec<F128>>,
}
impl Buffers {
    fn new(reuse: bool) -> Self {
        Self {
            live: 0,
            pooled: 0,
            peak: 0,
            reuse,
            pool: Vec::new(),
        }
    }
    fn acquire(&mut self, len: usize) -> Result<Vec<F128>, Error> {
        // Exact capacities keep the deterministic memory bound. In particular
        // never lend a large idle buffer to a tiny round and inflate its live
        // capacity. Every returned Vec has length zero and is fully rewritten.
        let values = if let Some(index) = self.pool.iter().position(|v| v.capacity() == len) {
            let values = self.pool.swap_remove(index);
            self.pooled -= values.capacity();
            debug_assert!(values.is_empty());
            values
        } else {
            let mut values = Vec::new();
            values
                .try_reserve_exact(len)
                .map_err(|_| Error::Allocation)?;
            values
        };
        self.live += values.capacity();
        self.peak = self.peak.max(self.live + self.pooled);
        Ok(values)
    }
    fn release(&mut self, mut values: Vec<F128>) {
        self.live -= values.capacity();
        if self.reuse {
            values.clear();
            self.pooled += values.capacity();
            self.pool.push(values);
        } else {
            drop(values);
        }
    }
    fn finish(&mut self) {
        debug_assert_eq!(self.live, 0);
        // This is an explicit end-of-proof release, not global allocator or
        // PCS retention. The caller charges its entire cost to online time.
        self.pool.clear();
        self.pooled = 0;
    }
}

enum Input<'a, F> {
    Dense(&'a [F128]),
    Leaves {
        read: &'a F,
        side: usize,
        len: usize,
    },
    Separate(&'a [F128], &'a [F128]),
}
impl<F: Fn(usize, usize) -> F128> Input<'_, F> {
    fn len(&self) -> usize {
        match self {
            Self::Dense(values) => values.len() / 2,
            Self::Leaves { len, .. } => len / 2,
            Self::Separate(a, b) => {
                debug_assert_eq!(a.len(), b.len());
                a.len()
            }
        }
    }
    #[inline]
    fn at(&self, i: usize) -> (F128, F128) {
        match self {
            Self::Dense(values) => (values[i], values[values.len() / 2 + i]),
            Self::Leaves { read, side, len } => (read(*side, i), read(*side, len / 2 + i)),
            Self::Separate(a, b) => (a[i], b[i]),
        }
    }

    /// Four high-bit child pairs. Dense slices use two contiguous vector loads;
    /// implicit leaves keep their original callback and physical index order.
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq",
        target_feature = "sse4.1"
    ))]
    #[inline]
    unsafe fn values4(
        &self,
        start: usize,
        contiguous: bool,
    ) -> (core::arch::x86_64::__m512i, core::arch::x86_64::__m512i) {
        use flock_field::gf2_128::x86_64::{f128x4_loadu, f128x4_set};
        debug_assert!(start + 4 <= self.len());
        // SAFETY: the caller provides four in-range child pairs. Dense and
        // separate slices are read-only; unaligned vector loads are supported.
        unsafe {
            if contiguous {
                match self {
                    Self::Dense(values) => {
                        return (
                            f128x4_loadu(values.as_ptr().add(start)),
                            f128x4_loadu(values.as_ptr().add(values.len() / 2 + start)),
                        );
                    }
                    Self::Separate(a, b) => {
                        return (
                            f128x4_loadu(a.as_ptr().add(start)),
                            f128x4_loadu(b.as_ptr().add(start)),
                        );
                    }
                    Self::Leaves { .. } => {}
                }
            }
            let v: [_; 4] = std::array::from_fn(|j| self.at(start + j));
            (
                f128x4_set(v[0].0, v[1].0, v[2].0, v[3].0),
                f128x4_set(v[0].1, v[1].1, v[2].1, v[3].1),
            )
        }
    }

    /// Four even/odd pairs for each child. Deinterleave in registers, keeping
    /// the low/high u64 limbs of every field element together.
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq",
        target_feature = "sse4.1"
    ))]
    #[inline]
    unsafe fn adjacent4(&self, start: usize, contiguous: bool) -> [core::arch::x86_64::__m512i; 4] {
        use core::arch::x86_64::{_mm512_permutex2var_epi64, _mm512_set_epi64};
        use flock_field::gf2_128::x86_64::f128x4_set;
        debug_assert!(start + 8 <= self.len());
        unsafe {
            if contiguous && !matches!(self, Self::Leaves { .. }) {
                let (al, bl) = self.values4(start, true);
                let (ah, bh) = self.values4(start + 4, true);
                let even = _mm512_set_epi64(13, 12, 9, 8, 5, 4, 1, 0);
                let odd = _mm512_set_epi64(15, 14, 11, 10, 7, 6, 3, 2);
                return [
                    _mm512_permutex2var_epi64(al, even, ah),
                    _mm512_permutex2var_epi64(bl, even, bh),
                    _mm512_permutex2var_epi64(al, odd, ah),
                    _mm512_permutex2var_epi64(bl, odd, bh),
                ];
            }
            let lo: [_; 4] = std::array::from_fn(|j| self.at(start + 2 * j));
            let hi: [_; 4] = std::array::from_fn(|j| self.at(start + 2 * j + 1));
            [
                f128x4_set(lo[0].0, lo[1].0, lo[2].0, lo[3].0),
                f128x4_set(lo[0].1, lo[1].1, lo[2].1, lo[3].1),
                f128x4_set(hi[0].0, hi[1].0, hi[2].0, hi[3].0),
                f128x4_set(hi[0].1, hi[1].1, hi[2].1, hi[3].1),
            ]
        }
    }
}

fn products<F: Fn(usize, usize) -> F128 + Sync>(
    input: &Input<'_, F>,
    buffers: &mut Buffers,
) -> Result<Vec<F128>, Error> {
    let _contiguous = tree_dense_simd_enabled();
    init_vec(input.len(), buffers, |start, dst| {
        #[allow(unused_mut)] // Advanced by the native four-lane path only.
        let mut i = 0;
        #[cfg(all(
            target_arch = "x86_64",
            target_feature = "avx512f",
            target_feature = "vpclmulqdq",
            target_feature = "sse4.1"
        ))]
        // SAFETY: the cfg gate covers the ISA; every iteration reads four
        // in-range pairs and initializes four consecutive output elements.
        unsafe {
            use flock_field::gf2_128::x86_64::ghash_mul_x4;
            use std::arch::x86_64::_mm512_storeu_si512;
            while i + 4 <= dst.len() {
                let (a, b) = input.values4(start + i, _contiguous);
                _mm512_storeu_si512(dst.as_mut_ptr().add(i).cast(), ghash_mul_x4(a, b));
                i += 4;
            }
        }
        for (offset, dst) in dst.iter_mut().enumerate().skip(i) {
            let (a, b) = input.at(start + offset);
            dst.write(a * b);
        }
    })
}

fn build<F: Fn(usize, usize) -> F128 + Sync>(
    len: usize,
    side: usize,
    read: &F,
    buffers: &mut Buffers,
) -> Result<Vec<Vec<F128>>, Error> {
    let mut layers = Vec::new();
    if len == 1 {
        return Ok(layers);
    }
    layers.push(products(&Input::Leaves { read, side, len }, buffers)?);
    while layers.last().unwrap().len() > 1 {
        let next = products::<F>(&Input::Dense(layers.last().unwrap()), buffers)?;
        layers.push(next);
    }
    Ok(layers)
}

fn core_block<F: Fn(usize, usize) -> F128>(
    input: &Input<'_, F>,
    start: usize,
    weights: &[F128],
    _contiguous: bool,
) -> [F128; 3] {
    let mut sums = [F256Unreduced::ZERO; 3];
    #[allow(unused_mut)] // Advanced by the native four-lane path only.
    let mut i = 0;
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq",
        target_feature = "sse4.1"
    ))]
    // SAFETY: all four input pairs and all four weights are in range. The
    // accumulator is the existing Flock GHASH four-lane unreduced product.
    unsafe {
        use flock_field::gf2_128::x86_64::{WideGhashX4, f128x4_loadu, ghash_mul_x4};
        use std::arch::x86_64::_mm512_xor_si512;
        let mut accum = [WideGhashX4::zero(); 3];
        while i + 4 <= weights.len() {
            let [a0, b0, a1, b1] = input.adjacent4(2 * (start + i), _contiguous);
            let w = f128x4_loadu(weights.as_ptr().add(i));
            accum[0].mul_acc(ghash_mul_x4(a0, b0), w);
            accum[1].mul_acc(ghash_mul_x4(a1, b1), w);
            accum[2].mul_acc(
                ghash_mul_x4(_mm512_xor_si512(a0, a1), _mm512_xor_si512(b0, b1)),
                w,
            );
            i += 4;
        }
        for k in 0..3 {
            sums[k] ^= accum[k].fold();
        }
    }
    for (offset, &weight) in weights.iter().enumerate().skip(i) {
        let (a0, b0) = input.at(2 * (start + offset));
        let (a1, b1) = input.at(2 * (start + offset) + 1);
        for (sum, value) in sums
            .iter_mut()
            .zip([a0 * b0, a1 * b1, (a0 + a1) * (b0 + b1)])
        {
            *sum ^= value.mul_unreduced(weight);
        }
    }
    sums.map(F256Unreduced::reduce)
}

fn message<F: Fn(usize, usize) -> F128 + Sync>(input: &Input<'_, F>, suffix: &[F128]) -> [F128; 3] {
    let eq = SplitEqGhash::with_n_hi(suffix, suffix.len() / 2);
    debug_assert_eq!(input.len() / 2, eq.lo.len() * eq.hi.len());
    let contiguous = tree_dense_simd_enabled();
    let block =
        |i: usize| core_block(input, i * eq.lo.len(), &eq.lo, contiguous).map(|v| v * eq.hi[i]);
    let add = |a: [F128; 3], b: [F128; 3]| std::array::from_fn(|i| a[i] + b[i]);
    if input.len() < CHUNK {
        (0..eq.hi.len()).map(block).fold([F128::ZERO; 3], add)
    } else {
        (0..eq.hi.len())
            .into_par_iter()
            .map(block)
            .reduce(|| [F128::ZERO; 3], add)
    }
}

/// Initialize one disjoint output tile using the original scalar/SIMD fold.
fn fold_block<F: Fn(usize, usize) -> F128 + Sync>(
    input: &Input<'_, F>,
    r: F128,
    start: usize,
    a: &mut [MaybeUninit<F128>],
    b: &mut [MaybeUninit<F128>],
    _contiguous: bool,
) {
    debug_assert_eq!(a.len(), b.len());
    #[allow(unused_mut)] // Advanced by the native four-lane path only.
    let mut i = 0;
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq",
        target_feature = "sse4.1"
    ))]
    // SAFETY: each vector iteration reads eight in-range pairs and writes
    // four distinct slots in each output. The ISA is compile-time gated.
    unsafe {
        use flock_field::gf2_128::x86_64::{f128x4_set, ghash_mul_x4};
        use std::arch::x86_64::{_mm512_storeu_si512, _mm512_xor_si512};
        let r = f128x4_set(r, r, r, r);
        while i + 4 <= a.len() {
            let [al, bl, ah, bh] = input.adjacent4(2 * (start + i), _contiguous);
            _mm512_storeu_si512(
                a.as_mut_ptr().add(i).cast(),
                _mm512_xor_si512(al, ghash_mul_x4(r, _mm512_xor_si512(al, ah))),
            );
            _mm512_storeu_si512(
                b.as_mut_ptr().add(i).cast(),
                _mm512_xor_si512(bl, ghash_mul_x4(r, _mm512_xor_si512(bl, bh))),
            );
            i += 4;
        }
    }
    for (offset, (a, b)) in a.iter_mut().zip(b).enumerate().skip(i) {
        let low = input.at(2 * (start + offset));
        let high = input.at(2 * (start + offset) + 1);
        a.write(low.0 + r * (low.0 + high.0));
        b.write(low.1 + r * (low.1 + high.1));
    }
}

fn fold<F: Fn(usize, usize) -> F128 + Sync>(
    input: &Input<'_, F>,
    r: F128,
    buffers: &mut Buffers,
) -> Result<[Vec<F128>; 2], Error> {
    let len = input.len() / 2;
    let mut a = buffers.acquire(len)?;
    let mut b = buffers.acquire(len)?;
    let contiguous = tree_dense_simd_enabled();
    let fill = |start: usize, a: &mut [MaybeUninit<F128>], b: &mut [MaybeUninit<F128>]| {
        fold_block(input, r, start, a, b, contiguous);
    };
    let ao = &mut a.spare_capacity_mut()[..len];
    let bo = &mut b.spare_capacity_mut()[..len];
    if len < CHUNK {
        fill(0, ao, bo);
    } else {
        ao.par_chunks_mut(CHUNK)
            .zip(bo.par_chunks_mut(CHUNK))
            .enumerate()
            .for_each(|(block, (a, b))| fill(block * CHUNK, a, b));
    }
    // SAFETY: fill initializes both output slices completely, including tails.
    unsafe {
        a.set_len(len);
        b.set_len(len);
    }
    Ok([a, b])
}

/// Fold a cache-sized output tile and consume it immediately for the next
/// round's unscaled message. The caller already knows that round's suffix
/// point. Its prefix/cubic conversion is still applied in transcript order.
/// No normalization, inverse or new challenge is used.
fn fold_and_message<F: Fn(usize, usize) -> F128 + Sync>(
    input: &Input<'_, F>,
    r: F128,
    suffix: &[F128],
    buffers: &mut Buffers,
) -> Result<([Vec<F128>; 2], [F128; 3]), Error> {
    fold_and_message_tiled::<F, { CHUNK / 2 }>(input, r, suffix, buffers)
}

fn fold_and_message_tiled<F: Fn(usize, usize) -> F128 + Sync, const PAIR_TILE: usize>(
    input: &Input<'_, F>,
    r: F128,
    suffix: &[F128],
    buffers: &mut Buffers,
) -> Result<([Vec<F128>; 2], [F128; 3]), Error> {
    let len = input.len() / 2;
    debug_assert!(len >= 2 && len.is_power_of_two());
    debug_assert!(PAIR_TILE.is_power_of_two());
    let eq = SplitEqGhash::with_n_hi(suffix, suffix.len() / 2);
    debug_assert_eq!(len / 2, eq.lo.len() * eq.hi.len());
    // Tile boundaries align with both adjacent output pairs and split-eq
    // low-weight blocks. Large low tables are consumed in several tiles.
    let tile = 2 * eq.lo.len().min(PAIR_TILE);
    let mut a = buffers.acquire(len)?;
    let mut b = buffers.acquire(len)?;
    let contiguous = tree_dense_simd_enabled();
    let _register_fusion = tree_register_fusion_enabled();
    let consume =
        |(block, (a, b)): (usize, (&mut [MaybeUninit<F128>], &mut [MaybeUninit<F128>]))| {
            let start = block * tile;
            let first_pair = start / 2;
            let low_offset = first_pair % eq.lo.len();
            let weights = &eq.lo[low_offset..low_offset + a.len() / 2];
            #[cfg(all(
                target_arch = "x86_64",
                target_feature = "avx512f",
                target_feature = "vpclmulqdq",
                target_feature = "sse4.1"
            ))]
            if _register_fusion {
                // SAFETY: the input has twice each output's length; this tile
                // and its complete weight slice are disjoint and in range.
                return unsafe { fold_message_registers(input, r, start, a, b, weights) }
                    .map(|value| value * eq.hi[first_pair / eq.lo.len()]);
            }
            fold_block(input, r, start, a, b, contiguous);
            // SAFETY: fold_block initialized every element of these two disjoint
            // tiles before this read. Vec lengths remain zero until all jobs join.
            let (a, b) = unsafe {
                (
                    std::slice::from_raw_parts(a.as_ptr().cast::<F128>(), a.len()),
                    std::slice::from_raw_parts(b.as_ptr().cast::<F128>(), b.len()),
                )
            };
            core_block::<F>(&Input::Separate(a, b), 0, weights, contiguous)
                .map(|value| value * eq.hi[first_pair / eq.lo.len()])
        };
    let add = |a: [F128; 3], b: [F128; 3]| std::array::from_fn(|i| a[i] + b[i]);
    let ao = &mut a.spare_capacity_mut()[..len];
    let bo = &mut b.spare_capacity_mut()[..len];
    let next = if len < CHUNK {
        ao.chunks_mut(tile)
            .zip(bo.chunks_mut(tile))
            .enumerate()
            .map(consume)
            .fold([F128::ZERO; 3], add)
    } else {
        ao.par_chunks_mut(tile)
            .zip(bo.par_chunks_mut(tile))
            .enumerate()
            .map(consume)
            .reduce(|| [F128::ZERO; 3], add)
    };
    // SAFETY: every output slot was initialized and all parallel reads have
    // joined. On panic both Vec lengths remain zero and no slot is dropped.
    unsafe {
        a.set_len(len);
        b.set_len(len);
    }
    Ok(([a, b], next))
}

/// Fold sixteen child pairs and consume their eight outputs in registers.
/// Stores are needed by the next challenge, but no output is loaded again to
/// construct the already-determined next message. All three raw sums retain
/// their original low-weight factor; the caller applies the high factor.
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512f",
    target_feature = "vpclmulqdq",
    target_feature = "sse4.1"
))]
unsafe fn fold_message_registers<F: Fn(usize, usize) -> F128 + Sync>(
    input: &Input<'_, F>,
    r: F128,
    start: usize,
    a: &mut [MaybeUninit<F128>],
    b: &mut [MaybeUninit<F128>],
    weights: &[F128],
) -> [F128; 3] {
    use core::arch::x86_64::{
        _mm512_permutex2var_epi64, _mm512_set_epi64, _mm512_storeu_si512, _mm512_xor_si512,
    };
    use flock_field::gf2_128::x86_64::{WideGhashX4, f128x4_loadu, f128x4_set, ghash_mul_x4};
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), weights.len() * 2);
    assert!(2 * (start + a.len()) <= input.len());
    unsafe {
        let challenge = f128x4_set(r, r, r, r);
        let even = _mm512_set_epi64(13, 12, 9, 8, 5, 4, 1, 0);
        let odd = _mm512_set_epi64(15, 14, 11, 10, 7, 6, 3, 2);
        let mut accum = [WideGhashX4::zero(); 3];
        let bulk = a.len() / 8 * 8;
        for i in (0..bulk).step_by(8) {
            let fold_four = |offset| {
                let [al, bl, ah, bh] = input.adjacent4(2 * (start + offset), true);
                (
                    _mm512_xor_si512(al, ghash_mul_x4(challenge, _mm512_xor_si512(al, ah))),
                    _mm512_xor_si512(bl, ghash_mul_x4(challenge, _mm512_xor_si512(bl, bh))),
                )
            };
            let (al, bl) = fold_four(i);
            let (ah, bh) = fold_four(i + 4);
            let a0 = _mm512_permutex2var_epi64(al, even, ah);
            let a1 = _mm512_permutex2var_epi64(al, odd, ah);
            let b0 = _mm512_permutex2var_epi64(bl, even, bh);
            let b1 = _mm512_permutex2var_epi64(bl, odd, bh);
            let w = f128x4_loadu(weights.as_ptr().add(i / 2));
            accum[0].mul_acc(ghash_mul_x4(a0, b0), w);
            accum[1].mul_acc(ghash_mul_x4(a1, b1), w);
            accum[2].mul_acc(
                ghash_mul_x4(_mm512_xor_si512(a0, a1), _mm512_xor_si512(b0, b1)),
                w,
            );
            _mm512_storeu_si512(a.as_mut_ptr().add(i).cast(), al);
            _mm512_storeu_si512(a.as_mut_ptr().add(i + 4).cast(), ah);
            _mm512_storeu_si512(b.as_mut_ptr().add(i).cast(), bl);
            _mm512_storeu_si512(b.as_mut_ptr().add(i + 4).cast(), bh);
        }
        // Tiny tiles and partial-vector tails retain the checked scalar fold.
        fold_block(input, r, start + bulk, &mut a[bulk..], &mut b[bulk..], true);
        let at = std::slice::from_raw_parts(a.as_ptr().add(bulk).cast::<F128>(), a.len() - bulk);
        let bt = std::slice::from_raw_parts(b.as_ptr().add(bulk).cast::<F128>(), b.len() - bulk);
        let tail = core_block::<F>(&Input::Separate(at, bt), 0, &weights[bulk / 2..], true);
        std::array::from_fn(|k| accum[k].fold().reduce() + tail[k])
    }
}

fn cubic([g0, g1, gi]: [F128; 3], parent: F128, prefix: F128) -> [F128; 3] {
    [
        prefix * (F128::ONE + parent) * g0,
        prefix * (parent * gi + g0 + g1),
        prefix * gi,
    ]
}
fn evaluate([g0, g2, g3]: [F128; 3], claim: F128, r: F128) -> F128 {
    g0 + r * (claim + g2 + g3 + r * (g2 + r * g3))
}

fn reduce<F: Fn(usize, usize) -> F128 + Sync, C: Challenger>(
    len: usize,
    side: usize,
    read: &F,
    mut layers: Vec<Vec<F128>>,
    root: F128,
    buffers: &mut Buffers,
    timings: &mut Timings,
    c: &mut C,
) -> Result<(TreeProof, LeafClaim), Error> {
    bind_tree(side, c);
    if let Some(root_layer) = layers.pop() {
        buffers.release(root_layer);
    }
    let log_len = len.ilog2() as usize;
    let mut point = Vec::new();
    let mut claim = root;
    let mut proof = TreeProof {
        layers: Vec::with_capacity(log_len),
    };
    let fuse = tree_fusion_enabled();
    for level in 0..log_len {
        let mut original = layers.pop();
        let mut current: Option<[Vec<F128>; 2]> = None;
        let mut rounds = Vec::with_capacity(level);
        let mut next_point = Vec::with_capacity(level + 1);
        let mut prefix = F128::ONE;
        let mut cached_message = None;
        for round in 0..level {
            let input = match &current {
                Some([a, b]) => Input::Separate(a, b),
                None => match &original {
                    Some(values) => Input::Dense(values),
                    None => Input::Leaves { read, side, len },
                },
            };
            let raw = cached_message
                .take()
                .unwrap_or_else(|| message(&input, &point[round + 1..]));
            let g = cubic(raw, point[round], prefix);
            c.observe_f128_slice(&g);
            let r = c.sample_f128();
            claim = evaluate(g, claim, r);
            rounds.push(g);
            next_point.push(r);
            prefix *= F128::ONE + point[round] + r;
            let next = if fuse && round + 1 < level {
                let (next, raw) = fold_and_message(&input, r, &point[round + 2..], buffers)?;
                cached_message = Some(raw);
                next
            } else {
                fold(&input, r, buffers)?
            };
            let release_start = Instant::now();
            if let Some(values) = original.take() {
                buffers.release(values);
            }
            if let Some(values) = current.take() {
                for v in values {
                    buffers.release(v);
                }
            }
            timings.release_ms += elapsed(release_start);
            current = Some(next);
        }
        let children = match &current {
            Some([a, b]) => [a[0], b[0]],
            None => match &original {
                Some(values) => [values[0], values[1]],
                None => [read(side, 0), read(side, 1)],
            },
        };
        if claim != prefix * children[0] * children[1] {
            return Err(Error::LayerCheck);
        }
        c.observe_f128_slice(&children);
        let r = c.sample_f128();
        claim = children[0] + r * (children[0] + children[1]);
        next_point.push(r);
        point = next_point;
        proof.layers.push(LayerProof { rounds, children });
        let release_start = Instant::now();
        if let Some(values) = original {
            buffers.release(values);
        }
        if let Some(values) = current {
            for v in values {
                buffers.release(v);
            }
        }
        timings.release_ms += elapsed(release_start);
    }
    debug_assert!(layers.is_empty());
    Ok((
        proof,
        LeafClaim {
            point,
            value: claim,
        },
    ))
}

/// Prove equality of the two length-2^log_len products. No source commitment
/// or source-value check is performed here: both returned claims are deferred.
pub fn prove_pair<C, F>(
    log_len: usize,
    leaves: F,
    c: &mut C,
) -> Result<(Proof, [LeafClaim; 2], Timings), Error>
where
    C: Challenger,
    F: Fn(usize, usize) -> F128 + Sync,
{
    let len = length(log_len)?;
    bind(log_len, c);
    let mut timings = Timings::default();
    let mut buffers = Buffers::new(tree_pool_enabled(log_len));
    let mut roots = Vec::with_capacity(2);
    let mut proofs = Vec::with_capacity(2);
    let mut claims = Vec::with_capacity(2);
    for side in 0..2 {
        let start = Instant::now();
        let layers = build(len, side, &leaves, &mut buffers)?;
        timings.construction_ms += elapsed(start);
        let start = Instant::now();
        let root = layers.last().map_or_else(|| leaves(side, 0), |v| v[0]);
        if side == 0 {
            c.observe_f128(root);
        } else if roots[0] != root {
            return Err(Error::ProductMismatch);
        }
        roots.push(root);
        timings.root_ms += elapsed(start);
        let start = Instant::now();
        let (proof, claim) = reduce(
            len,
            side,
            &leaves,
            layers,
            root,
            &mut buffers,
            &mut timings,
            c,
        )?;
        timings.reduction_ms += elapsed(start);
        proofs.push(proof);
        claims.push(claim);
        debug_assert_eq!(buffers.live, 0);
    }
    let release_start = Instant::now();
    buffers.finish();
    let final_release_ms = elapsed(release_start);
    timings.release_ms += final_release_ms;
    timings.reduction_ms += final_release_ms;
    timings.peak_field_buffer_bytes = buffers.peak * size_of::<F128>();
    Ok((
        Proof {
            root: roots[0],
            trees: proofs.try_into().unwrap(),
        },
        claims.try_into().unwrap(),
        timings,
    ))
}

/// Check every layer and return the two independently sampled ordinary MLE
/// queries. Acceptance is complete only after the caller authenticates them.
pub fn verify_pair<C: Challenger>(
    log_len: usize,
    proof: &Proof,
    c: &mut C,
) -> Result<[LeafClaim; 2], Error> {
    length(log_len)?;
    if proof.trees.iter().any(|tree| {
        tree.layers.len() != log_len
            || tree
                .layers
                .iter()
                .enumerate()
                .any(|(k, layer)| layer.rounds.len() != k)
    }) {
        return Err(Error::ProofShape);
    }
    bind(log_len, c);
    c.observe_f128(proof.root);
    let mut claims = Vec::with_capacity(2);
    for (side, tree) in proof.trees.iter().enumerate() {
        bind_tree(side, c);
        let mut point = Vec::new();
        let mut claim = proof.root;
        for (level, layer) in tree.layers.iter().enumerate() {
            let mut next_point = Vec::with_capacity(level + 1);
            let mut prefix = F128::ONE;
            for (i, &g) in layer.rounds.iter().enumerate() {
                c.observe_f128_slice(&g);
                let r = c.sample_f128();
                claim = evaluate(g, claim, r);
                next_point.push(r);
                prefix *= F128::ONE + point[i] + r;
            }
            if claim != prefix * layer.children[0] * layer.children[1] {
                return Err(Error::LayerCheck);
            }
            c.observe_f128_slice(&layer.children);
            let r = c.sample_f128();
            claim = layer.children[0] + r * (layer.children[0] + layer.children[1]);
            next_point.push(r);
            point = next_point;
        }
        claims.push(LeafClaim {
            point,
            value: claim,
        });
    }
    Ok(claims.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_transcript::challenger::FsChallenger;

    fn challenger() -> FsChallenger {
        FsChallenger::new(b"extension-product-independent-tests-v1")
    }
    fn source(log: usize) -> [Vec<F128>; 2] {
        let n = 1usize << log;
        let left: Vec<_> = (0..n)
            .map(|i| {
                let x = (i as u64 + 91).wrapping_mul(0x9e37_79b9_7f4a_7c15);
                F128::new(x, x.rotate_left(29) ^ 0x7398_ac21)
            })
            .collect();
        let right = (0..n).map(|i| left[(i * 5 + 3) & (n - 1)]).collect();
        [left, right]
    }
    // No product-tree, split-equality builder or folding code is used here.
    fn oracle(source: &[F128], claim: &LeafClaim) -> F128 {
        source.iter().enumerate().fold(F128::ZERO, |sum, (i, &v)| {
            let weight = claim
                .point
                .iter()
                .enumerate()
                .fold(F128::ONE, |w, (bit, &r)| {
                    w * if (i >> bit) & 1 == 0 {
                        F128::ONE + r
                    } else {
                        r
                    }
                });
            sum + v * weight
        })
    }
    fn authenticated(log: usize, proof: &Proof, sources: &[Vec<F128>; 2]) -> bool {
        let Ok(claims) = verify_pair(log, proof, &mut challenger()) else {
            return false;
        };
        claims
            .iter()
            .zip(sources)
            .all(|(q, s)| oracle(s, q) == q.value)
    }

    #[test]
    fn tree_contiguous_simd_matches_scalar_at_unaligned_pair_boundaries() {
        let data = source(14);
        for offset in [0usize, 1, 3] {
            for n in [4usize, 8, 12, 16, 20, 124, 128, 132, 4096, 4100] {
                let read = |side: usize, i: usize| data[side][offset + i];
                let inputs = [
                    Input::Dense(&data[0][offset..offset + 2 * n]),
                    Input::Separate(&data[0][offset..offset + n], &data[1][offset..offset + n]),
                    Input::Leaves {
                        read: &read,
                        side: 0,
                        len: 2 * n,
                    },
                ];
                let weights = &data[1][offset..offset + n / 2];
                for input in &inputs {
                    let expected = (0..n / 2).fold([F128::ZERO; 3], |mut sum, i| {
                        let (a0, b0) = input.at(2 * i);
                        let (a1, b1) = input.at(2 * i + 1);
                        for (dst, value) in
                            sum.iter_mut()
                                .zip([a0 * b0, a1 * b1, (a0 + a1) * (b0 + b1)])
                        {
                            *dst += value * weights[i];
                        }
                        sum
                    });
                    for contiguous in [false, true] {
                        assert_eq!(core_block(input, 0, weights, contiguous), expected);
                        for r in [F128::ZERO, F128::ONE, data[1][12]] {
                            let mut a = vec![MaybeUninit::uninit(); n / 2];
                            let mut b = a.clone();
                            fold_block(input, r, 0, &mut a, &mut b, contiguous);
                            for i in 0..n / 2 {
                                let low = input.at(2 * i);
                                let high = input.at(2 * i + 1);
                                // SAFETY: fold_block initialized every slot.
                                unsafe {
                                    assert_eq!(a[i].assume_init(), low.0 + r * (low.0 + high.0));
                                    assert_eq!(b[i].assume_init(), low.1 + r * (low.1 + high.1));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn tree_contiguous_simd_preserves_proof_claims_transcript_and_capacity() {
        for threads in [1, 3] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            pool.install(|| {
                for log in [0usize, 1, 2, 4, 7, 13] {
                    let sources = source(log);
                    for fusion in [false, true] {
                        let run = |contiguous| {
                            with_tree_dense_simd(contiguous, || {
                                with_tree_fusion_override(fusion, || {
                                    let mut pc = challenger();
                                    let (proof, claims, timing) =
                                        prove_pair(log, |side, i| sources[side][i], &mut pc)
                                            .unwrap();
                                    let mut vc = challenger();
                                    assert_eq!(verify_pair(log, &proof, &mut vc).unwrap(), claims);
                                    let next = pc.sample_f128();
                                    assert_eq!(next, vc.sample_f128());
                                    for side in 0..2 {
                                        assert_eq!(
                                            oracle(&sources[side], &claims[side]),
                                            claims[side].value
                                        );
                                    }
                                    (
                                        bincode::serialize(&proof).unwrap(),
                                        claims,
                                        next,
                                        timing.peak_field_buffer_bytes,
                                    )
                                })
                            })
                        };
                        let reference = run(false);
                        assert_eq!(reference, run(true));
                        with_tree_register_fusion(true, || {
                            assert_eq!(reference, run(false));
                            assert_eq!(reference, run(true));
                        });
                    }
                }
            });
        }
    }

    #[test]
    fn tree_fusion_dense_separate_and_leaf_tiles_match_original_folds_and_messages() {
        fn compare<F: Fn(usize, usize) -> F128 + Sync>(
            input: &Input<'_, F>,
            r: F128,
            suffix: &[F128],
        ) {
            let expected = fold(input, r, &mut Buffers::new(false)).unwrap();
            for i in 0..expected[0].len() {
                let (a0, b0) = input.at(2 * i);
                let (a1, b1) = input.at(2 * i + 1);
                assert_eq!(expected[0][i], a0 + r * (a0 + a1));
                assert_eq!(expected[1][i], b0 + r * (b0 + b1));
            }
            let expected_message =
                message::<F>(&Input::Separate(&expected[0], &expected[1]), suffix);
            let (actual, actual_message) =
                fold_and_message(input, r, suffix, &mut Buffers::new(false)).unwrap();
            assert_eq!(actual, expected);
            assert_eq!(actual_message, expected_message);
            // A smaller tile exercises partial low-equality blocks without
            // requiring the huge production dimension at which lo > CHUNK/2.
            let (small, small_message) =
                fold_and_message_tiled::<F, 2>(input, r, suffix, &mut Buffers::new(false)).unwrap();
            assert_eq!(small, expected);
            assert_eq!(small_message, expected_message);
            with_tree_register_fusion(true, || {
                let (actual, raw) =
                    fold_and_message(input, r, suffix, &mut Buffers::new(false)).unwrap();
                assert_eq!(actual, expected);
                assert_eq!(raw, expected_message);
                let (tiny, raw) =
                    fold_and_message_tiled::<F, 2>(input, r, suffix, &mut Buffers::new(false))
                        .unwrap();
                assert_eq!(tiny, expected);
                assert_eq!(raw, expected_message);
            });
        }
        for log in [3usize, 4, 5, 11, 12, 13, 14, 15] {
            let sources = source(log);
            let read = |side: usize, i: usize| sources[side][i];
            let leaves = Input::Leaves {
                read: &read,
                side: 0,
                len: sources[0].len(),
            };
            for r in [F128::ZERO, F128::ONE, F128::new(0xEDB1_6208, 0x8913_C674)] {
                let suffix: Vec<_> = (0..log - 3)
                    .map(|i| {
                        if r == F128::ZERO || r == F128::ONE {
                            r
                        } else {
                            sources[1][i]
                        }
                    })
                    .collect();
                compare(&leaves, r, &suffix);
                compare::<fn(usize, usize) -> F128>(&Input::Dense(&sources[0]), r, &suffix);
                compare::<fn(usize, usize) -> F128>(
                    &Input::Separate(
                        &sources[0][..sources[0].len() / 2],
                        &sources[0][sources[0].len() / 2..],
                    ),
                    r,
                    &suffix,
                );
            }
        }
    }

    #[test]
    fn tree_fusion_preserves_full_proof_claims_and_next_challenge() {
        for log in [0usize, 1, 2, 3, 7, 13, 14] {
            let mut sources = source(log);
            if log > 2 {
                // Exercise a zero product together with an implicit-one tail.
                let n = sources[0].len();
                sources[0][n * 3 / 4..].fill(F128::ONE);
                sources[0][2] = F128::ZERO;
                sources[1] = sources[0].iter().rev().copied().collect();
            }
            let run = |enabled| {
                with_tree_fusion_override(enabled, || {
                    let mut pc = challenger();
                    let (proof, claims, timing) =
                        prove_pair(log, |side, i| sources[side][i], &mut pc).unwrap();
                    let mut vc = challenger();
                    assert_eq!(verify_pair(log, &proof, &mut vc).unwrap(), claims);
                    let next = pc.sample_f128();
                    assert_eq!(next, vc.sample_f128());
                    for side in 0..2 {
                        assert_eq!(oracle(&sources[side], &claims[side]), claims[side].value);
                    }
                    (
                        bincode::serialize(&proof).unwrap(),
                        claims,
                        next,
                        timing.peak_field_buffer_bytes,
                    )
                })
            };
            assert_eq!(run(false), run(true));
        }
    }

    #[test]
    fn tree_fusion_zero_one_challenges_preserve_proof_and_observation_order() {
        for log in [0usize, 1, 2, 4, 7, 13] {
            let sources = source(log);
            for value in [F128::ZERO, F128::ONE] {
                let run = |enabled| {
                    with_tree_fusion_override(enabled, || {
                        let mut pc = Scripted {
                            value,
                            ..Default::default()
                        };
                        let (proof, claims, _) =
                            prove_pair(log, |side, i| sources[side][i], &mut pc).unwrap();
                        let mut vc = Scripted {
                            value,
                            ..Default::default()
                        };
                        assert_eq!(verify_pair(log, &proof, &mut vc).unwrap(), claims);
                        assert_eq!(pc.observations, vc.observations);
                        assert_eq!(pc.samples, vc.samples);
                        (
                            bincode::serialize(&proof).unwrap(),
                            claims,
                            pc.observations,
                            pc.samples,
                            pc.trees,
                        )
                    })
                };
                assert_eq!(run(false), run(true));
            }
        }
    }

    #[test]
    fn tree_pool_preserves_full_proof_claims_challenges_and_owned_capacity_bound() {
        for log in [0usize, 1, 2, 3, 7, 13, 14] {
            let sources = source(log);
            let mut baseline = None;
            for pool in [false, true] {
                for fusion in [false, true] {
                    let state = with_tree_pool_override(pool, || {
                        with_tree_fusion_override(fusion, || {
                            let mut pc = challenger();
                            let (proof, claims, timings) =
                                prove_pair(log, |side, i| sources[side][i], &mut pc).unwrap();
                            let n = 1usize << log;
                            let elements = if pool && n >= 2 { 3 * n / 2 - 2 } else { n - 1 };
                            assert_eq!(
                                timings.peak_field_buffer_bytes,
                                elements * size_of::<F128>()
                            );
                            assert!(timings.reduction_ms >= timings.release_ms);
                            let mut vc = challenger();
                            assert_eq!(verify_pair(log, &proof, &mut vc).unwrap(), claims);
                            let next = pc.sample_f128();
                            assert_eq!(next, vc.sample_f128());
                            for side in 0..2 {
                                assert_eq!(
                                    oracle(&sources[side], &claims[side]),
                                    claims[side].value
                                );
                            }
                            (bincode::serialize(&proof).unwrap(), claims, next)
                        })
                    });
                    if let Some(expected) = &baseline {
                        assert_eq!(&state, expected);
                    } else {
                        baseline = Some(state);
                    }
                }
            }
        }
    }

    #[test]
    fn tree_pool_zero_one_challenges_keep_schedule_under_all_fusion_modes() {
        let log = 9;
        let mut sources = source(log);
        // Zero roots and a reused zero/one-heavy tail must never expose stale
        // data from an earlier level or from the other independently read tree.
        let len = sources[0].len();
        sources[0][3 * len / 4..].fill(F128::ONE);
        sources[0][2] = F128::ZERO;
        sources[1] = sources[0].iter().rev().copied().collect();
        for value in [F128::ZERO, F128::ONE] {
            let mut baseline = None;
            for pool in [false, true] {
                for fusion in [false, true] {
                    let state = with_tree_pool_override(pool, || {
                        with_tree_fusion_override(fusion, || {
                            let mut pc = Scripted {
                                value,
                                ..Default::default()
                            };
                            let (proof, claims, _) =
                                prove_pair(log, |side, i| sources[side][i], &mut pc).unwrap();
                            let mut vc = Scripted {
                                value,
                                ..Default::default()
                            };
                            assert_eq!(verify_pair(log, &proof, &mut vc).unwrap(), claims);
                            assert_eq!(pc.observations, vc.observations);
                            assert_eq!(pc.samples, vc.samples);
                            for side in 0..2 {
                                assert_eq!(
                                    oracle(&sources[side], &claims[side]),
                                    claims[side].value
                                );
                            }
                            (
                                bincode::serialize(&proof).unwrap(),
                                claims,
                                pc.observations,
                                pc.samples,
                            )
                        })
                    });
                    if let Some(expected) = &baseline {
                        assert_eq!(&state, expected);
                    } else {
                        baseline = Some(state);
                    }
                }
            }
        }
    }

    #[test]
    fn tree_pool_exact_reuse_reinitializes_tails_and_drops_at_call_boundary() {
        for log in [0, 14, 26, 28, 29, 30, 32, 33, 63] {
            assert_eq!(tree_pool_size_allowed(log, false), log <= 28);
            assert_eq!(tree_pool_size_allowed(log, true), log <= 32);
        }
        with_tree_pool_override(true, || {
            assert!(tree_pool_enabled(28));
            let large = std::env::var_os("MIKROS_TREE_LARGE_POOL").is_some_and(|v| v == "1");
            assert_eq!(tree_pool_enabled(29), large);
            assert_eq!(tree_pool_enabled(32), large);
            assert!(!tree_pool_enabled(33));
        });
        with_tree_pool_override(false, || assert!(!tree_pool_enabled(28)));
        for len in [1usize, 4, CHUNK - 1, CHUNK, CHUNK + 1] {
            let mut buffers = Buffers::new(true);
            let original = init_vec(len, &mut buffers, |_, dst| {
                for value in dst {
                    value.write(F128::new(0xBAD, 0xDEAD));
                }
            })
            .unwrap();
            let original_ptr = original.as_ptr();
            let capacity = original.capacity();
            buffers.release(original);
            assert_eq!(buffers.live, 0);
            assert_eq!(buffers.pooled, capacity);
            // A failed allocation leaves existing pooled ownership intact.
            assert_eq!(buffers.acquire(usize::MAX).unwrap_err(), Error::Allocation);
            assert_eq!(buffers.live, 0);
            assert_eq!(buffers.pooled, capacity);
            let rewritten = init_vec(len, &mut buffers, |start, dst| {
                for (i, value) in dst.iter_mut().enumerate() {
                    value.write(F128::new((start + i) as u64, 0));
                }
            })
            .unwrap();
            assert_eq!(rewritten.as_ptr(), original_ptr);
            assert_eq!(buffers.pooled, 0);
            assert_eq!(buffers.live, capacity);
            for (i, &value) in rewritten.iter().enumerate() {
                assert_eq!(value, F128::new(i as u64, 0));
            }
            buffers.release(rewritten);
            buffers.finish();
            assert_eq!((buffers.live, buffers.pooled), (0, 0));
            assert!(buffers.pool.is_empty());
            assert_eq!(buffers.peak, capacity);
        }
    }

    #[test]
    fn extension_product_roundtrip_independent_sources_and_parallel_boundaries() {
        for log in [0usize, 1, 2, 3, 6, 9, 13] {
            let sources = source(log);
            let mut c = challenger();
            let (proof, claims, timings) = with_tree_pool_override(false, || {
                prove_pair(log, |side, i| sources[side][i], &mut c)
            })
            .unwrap();
            let expected_root = sources[0].iter().copied().fold(F128::ONE, |a, b| a * b);
            assert_eq!(proof.root, expected_root);
            let mut v = challenger();
            assert_eq!(verify_pair(log, &proof, &mut v).unwrap(), claims);
            assert_eq!(c.sample_f128(), v.sample_f128());
            assert!(authenticated(log, &proof, &sources));
            assert_eq!(
                proof.field_elements(),
                1 + 3 * log * log.saturating_sub(1) + 4 * log
            );
            // All internals total n-1 elements; the two half-sized fold
            // operands and their next output never exceed this for n>=4.
            assert!(timings.peak_field_buffer_bytes <= ((1usize << log) - 1) * size_of::<F128>());
            if log > 0 {
                assert_ne!(claims[0].point, claims[1].point);
            }
        }
    }

    #[derive(Default)]
    struct Scripted {
        value: F128,
        samples: usize,
        observations: Vec<usize>,
        trees: usize,
        last_values: Option<usize>,
    }
    impl Challenger for Scripted {
        fn fork_from_seed(&self, _: [F128; 2], _: &'static [u8]) -> Self {
            panic!("sequential product trees must not fork the challenger")
        }
        fn observe_label(&mut self, label: &[u8]) {
            if label == b"mikros-extension-product-tree-v1" {
                self.trees += 1;
            }
        }
        fn observe_f128(&mut self, _: F128) {
            self.last_values = Some(1);
        }
        fn observe_f128_slice(&mut self, values: &[F128]) {
            self.last_values = Some(values.len());
        }
        fn sample_f128(&mut self) -> F128 {
            // Every challenge follows one cubic message or a pair of child
            // values; even the root layer samples its own merge challenge.
            let n = self.last_values.take().unwrap();
            assert!(n == 2 || n == 3);
            self.observations.push(n);
            self.samples += 1;
            self.value
        }
    }

    #[test]
    fn extension_product_zero_one_challenges_preserve_layers_and_schedule() {
        for log in [0usize, 1, 2, 4, 7] {
            let sources = source(log);
            for value in [F128::ZERO, F128::ONE] {
                let mut c = Scripted {
                    value,
                    ..Default::default()
                };
                let (proof, claims, _) =
                    prove_pair(log, |side, i| sources[side][i], &mut c).unwrap();
                let mut v = Scripted {
                    value,
                    ..Default::default()
                };
                assert_eq!(verify_pair(log, &proof, &mut v).unwrap(), claims);
                assert_eq!(c.observations, v.observations);
                assert_eq!(c.samples, log * (log + 1));
                assert_eq!(c.trees, 2);
                let expected: Vec<_> = (0..2)
                    .flat_map(|_| {
                        (0..log).flat_map(|level| {
                            std::iter::repeat_n(3, level).chain(std::iter::once(2))
                        })
                    })
                    .collect();
                assert_eq!(c.observations, expected);
                for side in 0..2 {
                    assert_eq!(oracle(&sources[side], &claims[side]), claims[side].value);
                }
                if value == F128::ZERO {
                    for (level, layer) in proof.trees[0].layers.iter().enumerate() {
                        let modulus = 1 << (level + 1);
                        for child in 0..2 {
                            let expected = sources[0]
                                .iter()
                                .enumerate()
                                .filter(|(i, _)| i % modulus == child * (1 << level))
                                .fold(F128::ONE, |v, (_, &x)| v * x);
                            assert_eq!(layer.children[child], expected);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn extension_product_rejects_shapes_bad_products_and_tampered_messages() {
        assert_eq!(length(usize::BITS as usize), Err(Error::Length));
        assert_eq!(length(usize::MAX), Err(Error::Length));
        assert!(
            prove_pair(
                usize::MAX,
                |_, _| panic!("invalid dimension must not read source"),
                &mut challenger()
            )
            .is_err()
        );
        assert!(matches!(
            prove_pair(
                0,
                |side, _| if side == 0 { F128::ONE } else { F128::ZERO },
                &mut challenger()
            ),
            Err(Error::ProductMismatch)
        ));
        let log = 4;
        let sources = source(log);
        let (proof, _, _) = prove_pair(log, |side, i| sources[side][i], &mut challenger()).unwrap();
        let mut bad = proof.clone();
        bad.root += F128::ONE;
        assert!(!authenticated(log, &bad, &sources));
        for side in 0..2 {
            let mut bad = proof.clone();
            bad.trees[side].layers.pop();
            assert_eq!(
                verify_pair(log, &bad, &mut challenger()),
                Err(Error::ProofShape)
            );
            for level in 0..log {
                for child in 0..2 {
                    let mut bad = proof.clone();
                    bad.trees[side].layers[level].children[child] += F128::ONE;
                    assert!(!authenticated(log, &bad, &sources));
                }
                for round in 0..level {
                    for coefficient in 0..3 {
                        let mut bad = proof.clone();
                        bad.trees[side].layers[level].rounds[round][coefficient] += F128::ONE;
                        assert!(!authenticated(log, &bad, &sources));
                    }
                }
            }
            let mut bad = proof.clone();
            bad.trees[side].layers[0].rounds.push([F128::ZERO; 3]);
            assert_eq!(
                verify_pair(log, &bad, &mut challenger()),
                Err(Error::ProofShape)
            );
        }
        // A length-one reduction has no internal checks: authentication of the
        // returned leaf is essential, and must reject a changed claimed root.
        let sources = source(0);
        let (mut proof, _, _) =
            prove_pair(0, |side, i| sources[side][i], &mut challenger()).unwrap();
        proof.root += F128::ONE;
        assert!(!authenticated(0, &proof, &sources));
    }

    #[test]
    fn extension_product_threads_and_implicit_one_padding_match() {
        let log = 9;
        let mut sources = source(log);
        sources[0].fill(F128::ONE);
        for i in 0..3 * (1 << (log - 2)) + 7 {
            sources[0][i] = F128::new(i as u64 + 2, 3);
        }
        sources[1] = sources[0].clone();
        sources[1][..100].reverse();
        let run = |threads| {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap()
                .install(|| {
                    let (proof, claims, _) =
                        prove_pair(log, |side, i| sources[side][i], &mut challenger()).unwrap();
                    (proof, claims)
                })
        };
        let one = run(1);
        assert_eq!(one, run(4));
        assert!(authenticated(log, &one.0, &sources));
    }

    #[test]
    fn extension_product_zero_root_is_supported_without_normalization() {
        let log = 3;
        let mut sources = source(log);
        sources[0][2] = F128::ZERO;
        sources[1] = sources[0].iter().rev().copied().collect();
        let (proof, claims, _) =
            prove_pair(log, |side, i| sources[side][i], &mut challenger()).unwrap();
        assert_eq!(proof.root, F128::ZERO);
        assert!(authenticated(log, &proof, &sources));
        for side in 0..2 {
            assert_eq!(claims[side].value, oracle(&sources[side], &claims[side]));
        }
    }
}
