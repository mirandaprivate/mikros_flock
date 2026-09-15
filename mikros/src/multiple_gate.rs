//! Gate sumcheck for packing 128 binary circuit executions across columns.
//!
//! The four wire vectors are `[L,R,M,O]`, with virtual `M` supplied by the
//! caller. The five binary selectors are `[L,R,M,O,C]`. This proves the
//! equality-weighted packed gate residual, and returns their nine evaluations
//! at one common, low-bit-first point. Authenticate the selectors and L/R/O
//! through the PCS and reduce M through the weighted Hadamard proof.
//!
//! Public selectors may repeat with any power-of-two period up to 128. This
//! representation is exact: selector state never expands to the full gate
//! count. The source commitment must bind the corresponding repeated columns.

use crate::{Error, inner_product};
use core::mem::MaybeUninit;
use flock_core::zerocheck::univariate_skip::SplitEqGhash;
use flock_field::{F128, F256Unreduced};
use flock_transcript::challenger::Challenger;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

/// One packed-bit period for each selector, in `[L,R,M,O,C]` order.
/// `period_log=0` is a constant selector; unused bits in its final byte must
/// be zero. The full source repeats this period until there are `2^log_n` gates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepeatedSelectors {
    pub period_log: usize,
    pub packed: [Vec<u8>; 5],
}

/// A cubic round transmits `[g(0), coefficient(X^2), coefficient(X^3)]`.
/// Its linear coefficient follows from the running claim `g(0)+g(1)`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    pub rounds: Vec<[F128; 3]>,
    pub selectors: [F128; 5],
    pub wires: [F128; 4],
}

impl Proof {
    pub fn field_elements(&self) -> usize {
        3 * self.rounds.len() + 9
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Claims {
    /// Common MLE point, in low-bit-first order.
    pub point: Vec<F128>,
    /// `[L,R,M,O,C]` selector evaluations, all requiring authentication.
    pub selectors: [F128; 5],
    /// `[L,R,M,O]` evaluations. M is the next weighted Hadamard scalar claim.
    pub wires: [F128; 4],
}

fn checked_len(log_n: usize, period_log: usize) -> Result<usize, Error> {
    if log_n >= usize::BITS as usize {
        return Err(Error::InvalidLogN(log_n));
    }
    let n = 1usize << log_n;
    if n > isize::MAX as usize / size_of::<F128>() {
        return Err(Error::InvalidLogN(log_n));
    }
    if period_log > 7 || period_log > log_n {
        return Err(Error::ProofShape("multiple gate selector period"));
    }
    Ok(n)
}

impl RepeatedSelectors {
    fn initial_state(&self, log_n: usize) -> Result<[Vec<F128>; 5], Error> {
        checked_len(log_n, self.period_log)?;
        let period = 1usize << self.period_log;
        let bytes = period.div_ceil(8);
        for input in &self.packed {
            if input.len() != bytes {
                return Err(Error::InputLength {
                    input: "selector period",
                    expected: bytes,
                    got: input.len(),
                });
            }
            if period < 8 && input[0] >> period != 0 {
                return Err(Error::ProofShape("noncanonical selector period"));
            }
        }
        Ok(std::array::from_fn(|j| {
            (0..period)
                .map(|i| {
                    if self.packed[j][i / 8] >> (i % 8) & 1 == 0 {
                        F128::ZERO
                    } else {
                        F128::ONE
                    }
                })
                .collect()
        }))
    }
}

fn start<C: Challenger>(
    log_n: usize,
    period_log: usize,
    basis_sum: F128,
    challenger: &mut C,
) -> Vec<F128> {
    challenger.observe_label(b"mikros-binary-multiple-instance-gates-v1");
    challenger.observe_bytes(&(log_n as u64).to_le_bytes());
    challenger.observe_bytes(&(period_log as u64).to_le_bytes());
    challenger.observe_bytes(&128u64.to_le_bytes());
    challenger.observe_f128(basis_sum);
    challenger.sample_f128_vec(log_n)
}

fn observe_terminals<C: Challenger>(proof: &Proof, challenger: &mut C) {
    challenger.observe_label(b"mikros-multiple-instance-gate-terminals-v1");
    challenger.observe_f128_slice(&proof.selectors);
    challenger.observe_f128_slice(&proof.wires);
}

fn residual(selectors: &[F128; 5], wires: &[F128; 4], basis_sum: F128) -> F128 {
    (0..4).fold(basis_sum * selectors[4], |sum, j| {
        sum + selectors[j] * wires[j]
    })
}

fn cubic([r0, r1, rinf]: [F128; 3], x: F128, prefix: F128) -> [F128; 3] {
    [
        prefix * (F128::ONE + x) * r0,
        prefix * (x * rinf + r0 + r1),
        prefix * rinf,
    ]
}

fn evaluate([g0, g2, g3]: [F128; 3], claim: F128, r: F128) -> F128 {
    g0 + r * (claim + g2 + g3 + r * (g2 + r * g3))
}

fn add(a: [F128; 3], b: [F128; 3]) -> [F128; 3] {
    std::array::from_fn(|j| a[j] + b[j])
}

#[cfg(test)]
thread_local! {
    static SELECTOR_SIMD_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
    static GATE_FUSION_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
}

fn gate_fusion_enabled() -> bool {
    #[cfg(test)]
    if let Some(enabled) = GATE_FUSION_OVERRIDE.get() {
        return enabled;
    }
    std::env::var_os("MIKROS_GATE_FUSION").is_some_and(|value| value == "1")
}

#[cfg(test)]
fn with_gate_fusion<T>(enabled: bool, run: impl FnOnce() -> T) -> T {
    struct Reset(Option<bool>);
    impl Drop for Reset {
        fn drop(&mut self) {
            GATE_FUSION_OVERRIDE.set(self.0);
        }
    }
    let _reset = Reset(GATE_FUSION_OVERRIDE.replace(Some(enabled)));
    run()
}

fn selector_simd_enabled() -> bool {
    #[cfg(test)]
    if let Some(enabled) = SELECTOR_SIMD_OVERRIDE.get() {
        return enabled;
    }
    std::env::var_os("MIKROS_GATE_SELECTOR_SIMD").is_some_and(|value| value == "1")
}

#[cfg(test)]
fn with_selector_simd<T>(enabled: bool, run: impl FnOnce() -> T) -> T {
    struct Reset(Option<bool>);
    impl Drop for Reset {
        fn drop(&mut self) {
            SELECTOR_SIMD_OVERRIDE.set(self.0);
        }
    }
    let _reset = Reset(SELECTOR_SIMD_OVERRIDE.replace(Some(enabled)));
    run()
}

/// These identities hold for the AND/XOR selector pattern and survive its
/// folds. Check the complete current period; no identity of the wires, in
/// particular M=L&R, is assumed by this arithmetic rearrangement.
fn selector_residual_matches(selectors: &[Vec<F128>; 5]) -> bool {
    let len = selectors[0].len();
    len != 0
        && selectors.iter().all(|column| column.len() == len)
        && (0..len).all(|i| {
            selectors[1][i] == selectors[0][i]
                && selectors[2][i] == F128::ONE + selectors[0][i]
                && selectors[3][i] == F128::ONE
                && selectors[4][i] == F128::ZERO
        })
}

/// Weighted q*(L+R+M)+(M+O), with U=L+R+M and V=M+O. Equal
/// coefficient vectors can use one multiplication of U+V instead of two.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SelectorProduct {
    Zero,
    U,
    V,
    Sum,
    Separate,
}

impl SelectorProduct {
    fn new(u: &[F128], v: &[F128]) -> Self {
        match (u.is_empty(), v.is_empty()) {
            (true, true) => Self::Zero,
            (false, true) => Self::U,
            (true, false) => Self::V,
            (false, false) if u == v => Self::Sum,
            (false, false) => Self::Separate,
        }
    }
}

/// The repeated selector depends only on low index bits. Put all of them in
/// eq.lo, and multiply each selector coefficient by that shared weight once
/// per round. High blocks reuse these small arrays, with no N-sized eq table.
struct RoundWeights {
    eq: SplitEqGhash,
    coefficients: [[Vec<F128>; 3]; 4],
    constant: [F128; 3],
    selector_products: Option<[SelectorProduct; 3]>,
}

impl RoundWeights {
    fn new(point: &[F128], selectors: &[Vec<F128>; 5], basis_sum: F128) -> Self {
        let period = selectors[0].len();
        let period_log = period.ilog2() as usize;
        let lo_log = point.len().div_ceil(2).max(period_log.saturating_sub(1));
        let eq = SplitEqGhash::with_n_hi(point, point.len() - lo_log);
        let mask = period - 1;
        let coefficients = std::array::from_fn(|j| {
            let mut result: [Vec<F128>; 3] =
                std::array::from_fn(|_| Vec::with_capacity(eq.lo.len()));
            for (i, &weight) in eq.lo.iter().enumerate() {
                let q0 = selectors[j][(2 * i) & mask];
                let q1 = selectors[j][(2 * i + 1) & mask];
                let a = weight * q0;
                let b = weight * q1;
                result[0].push(a);
                result[1].push(b);
                result[2].push(a + b);
            }
            for coefficient in &mut result {
                if coefficient.iter().all(|x| x.is_zero()) {
                    coefficient.clear();
                }
            }
            result
        });
        let mut constant = [F128::ZERO; 3];
        for (i, &weight) in eq.lo.iter().enumerate() {
            constant[0] += weight * selectors[4][(2 * i) & mask];
            constant[1] += weight * selectors[4][(2 * i + 1) & mask];
        }
        constant[0] *= basis_sum;
        constant[1] *= basis_sum;
        // Resolve the opt-in and inspect the selector period before entering
        // Rayon: the selected kernel is then shared by every worker.
        let selector_products = (selector_simd_enabled() && selector_residual_matches(selectors))
            .then(|| {
                std::array::from_fn(|k| {
                    SelectorProduct::new(&coefficients[0][k], &coefficients[3][k])
                })
            });
        Self {
            eq,
            coefficients,
            constant,
            selector_products,
        }
    }
}

fn message_block(
    wires: [&[F128]; 4],
    base: usize,
    weights: &[[Vec<F128>; 3]; 4],
    count: usize,
) -> [F128; 3] {
    message_block_range(wires, base, weights, 0, count)
}

fn message_block_range(
    wires: [&[F128]; 4],
    base: usize,
    weights: &[[Vec<F128>; 3]; 4],
    weight_offset: usize,
    count: usize,
) -> [F128; 3] {
    let mut accum = [F256Unreduced::ZERO; 3];
    #[allow(unused_mut)]
    let mut cursor = 0;
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq",
        target_feature = "sse4.1"
    ))]
    {
        use core::arch::x86_64::_mm512_xor_si512;
        use flock_field::gf2_128::x86_64::{WideGhashX4, f128x4_loadu, f128x4_set};
        // SAFETY: native features are compile-time gated. Each iteration has
        // four complete adjacent pairs and four in-bounds coefficient values.
        unsafe {
            let mut wide = [WideGhashX4::zero(); 3];
            let full = count / 4 * 4;
            for j in 0..4 {
                if weights[j].iter().all(Vec::is_empty) {
                    continue;
                }
                for i in (0..full).step_by(4) {
                    let input = &wires[j][2 * (base + i)..];
                    let even = f128x4_set(input[0], input[2], input[4], input[6]);
                    let odd = f128x4_set(input[1], input[3], input[5], input[7]);
                    let values = [even, odd, _mm512_xor_si512(even, odd)];
                    for k in 0..3 {
                        if !weights[j][k].is_empty() {
                            wide[k].mul_acc(
                                values[k],
                                f128x4_loadu(weights[j][k].as_ptr().add(weight_offset + i)),
                            );
                        }
                    }
                }
            }
            for k in 0..3 {
                accum[k] ^= wide[k].fold();
            }
            cursor = full;
        }
    }
    for j in 0..4 {
        for i in cursor..count {
            let a = wires[j][2 * (base + i)];
            let b = wires[j][2 * (base + i) + 1];
            for (k, value) in [a, b, a + b].into_iter().enumerate() {
                if !weights[j][k].is_empty() {
                    accum[k] ^= value.mul_unreduced(weights[j][k][weight_offset + i]);
                }
            }
        }
    }
    accum.map(F256Unreduced::reduce)
}

fn message_block_selector_simd(
    wires: [&[F128]; 4],
    base: usize,
    weights: &[[Vec<F128>; 3]; 4],
    count: usize,
    products: [SelectorProduct; 3],
) -> [F128; 3] {
    message_block_selector_simd_range(wires, base, weights, 0, count, products)
}

fn message_block_selector_simd_range(
    wires: [&[F128]; 4],
    base: usize,
    weights: &[[Vec<F128>; 3]; 4],
    weight_offset: usize,
    count: usize,
    products: [SelectorProduct; 3],
) -> [F128; 3] {
    let mut accum = [F256Unreduced::ZERO; 3];
    #[allow(unused_mut)]
    let mut cursor = 0;
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq",
        target_feature = "sse4.1"
    ))]
    {
        use core::arch::x86_64::_mm512_xor_si512;
        use flock_field::gf2_128::x86_64::{WideGhashX4, f128x4_loadu, f128x4_set};
        // SAFETY: native features are compile-time gated. Every iteration
        // reads four complete pairs from each wire. The selected product
        // accesses only nonempty coefficient arrays, each covering count.
        unsafe {
            let mut wide = [WideGhashX4::zero(); 3];
            let full = count / 4 * 4;
            for i in (0..full).step_by(4) {
                let pairs: [_; 4] = std::array::from_fn(|j| {
                    let input = &wires[j][2 * (base + i)..];
                    [
                        f128x4_set(input[0], input[2], input[4], input[6]),
                        f128x4_set(input[1], input[3], input[5], input[7]),
                    ]
                });
                let u0 = _mm512_xor_si512(_mm512_xor_si512(pairs[0][0], pairs[1][0]), pairs[2][0]);
                let u1 = _mm512_xor_si512(_mm512_xor_si512(pairs[0][1], pairs[1][1]), pairs[2][1]);
                let v0 = _mm512_xor_si512(pairs[2][0], pairs[3][0]);
                let v1 = _mm512_xor_si512(pairs[2][1], pairs[3][1]);
                let u = [u0, u1, _mm512_xor_si512(u0, u1)];
                let v = [v0, v1, _mm512_xor_si512(v0, v1)];
                for k in 0..3 {
                    match products[k] {
                        SelectorProduct::Zero => {}
                        SelectorProduct::U => {
                            wide[k].mul_acc(
                                u[k],
                                f128x4_loadu(weights[0][k].as_ptr().add(weight_offset + i)),
                            );
                        }
                        SelectorProduct::V => {
                            wide[k].mul_acc(
                                v[k],
                                f128x4_loadu(weights[3][k].as_ptr().add(weight_offset + i)),
                            );
                        }
                        SelectorProduct::Sum => {
                            wide[k].mul_acc(
                                _mm512_xor_si512(u[k], v[k]),
                                f128x4_loadu(weights[0][k].as_ptr().add(weight_offset + i)),
                            );
                        }
                        SelectorProduct::Separate => {
                            wide[k].mul_acc(
                                u[k],
                                f128x4_loadu(weights[0][k].as_ptr().add(weight_offset + i)),
                            );
                            wide[k].mul_acc(
                                v[k],
                                f128x4_loadu(weights[3][k].as_ptr().add(weight_offset + i)),
                            );
                        }
                    }
                }
            }
            for k in 0..3 {
                accum[k] ^= wide[k].fold();
            }
            cursor = full;
        }
    }
    for i in cursor..count {
        let even: [F128; 4] = std::array::from_fn(|j| wires[j][2 * (base + i)]);
        let odd: [F128; 4] = std::array::from_fn(|j| wires[j][2 * (base + i) + 1]);
        let u0 = even[0] + even[1] + even[2];
        let u1 = odd[0] + odd[1] + odd[2];
        let v0 = even[2] + even[3];
        let v1 = odd[2] + odd[3];
        let u = [u0, u1, u0 + u1];
        let v = [v0, v1, v0 + v1];
        for k in 0..3 {
            match products[k] {
                SelectorProduct::Zero => {}
                SelectorProduct::U => {
                    accum[k] ^= u[k].mul_unreduced(weights[0][k][weight_offset + i])
                }
                SelectorProduct::V => {
                    accum[k] ^= v[k].mul_unreduced(weights[3][k][weight_offset + i])
                }
                SelectorProduct::Sum => {
                    accum[k] ^= (u[k] + v[k]).mul_unreduced(weights[0][k][weight_offset + i]);
                }
                SelectorProduct::Separate => {
                    accum[k] ^= u[k].mul_unreduced(weights[0][k][weight_offset + i]);
                    accum[k] ^= v[k].mul_unreduced(weights[3][k][weight_offset + i]);
                }
            }
        }
    }
    accum.map(F256Unreduced::reduce)
}

fn round_message(
    wires: [&[F128]; 4],
    point: &[F128],
    selectors: &[Vec<F128>; 5],
    basis_sum: F128,
) -> [F128; 3] {
    round_message_source(wires, point, selectors, basis_sum, false)
}

/// The initial virtual-product path materializes M only in a bounded tile.
/// After the first challenge, M is an ordinary folded extension-field wire;
/// it must never be replaced by AND of the already-folded L and R.
fn round_message_source(
    wires: [&[F128]; 4],
    point: &[F128],
    selectors: &[Vec<F128>; 5],
    basis_sum: F128,
    virtual_product: bool,
) -> [F128; 3] {
    let weights = RoundWeights::new(point, selectors, basis_sum);
    assert_eq!(
        wires[0].len() / 2,
        weights.eq.lo.len() * weights.eq.hi.len()
    );
    let block = |hi: usize| {
        let base = hi * weights.eq.lo.len();
        let count = weights.eq.lo.len();
        let message = if virtual_product {
            message_virtual_product_block(wires, base, &weights, count)
        } else if let Some(products) = weights.selector_products {
            message_block_selector_simd(wires, base, &weights.coefficients, count, products)
        } else {
            message_block(wires, base, &weights.coefficients, count)
        };
        message.map(|v| v * weights.eq.hi[hi])
    };
    let sums = if wires[0].len() >= 1 << 15 && rayon::current_num_threads() > 1 {
        (0..weights.eq.hi.len())
            .into_par_iter()
            .map(block)
            .reduce(|| [F128::ZERO; 3], add)
    } else {
        (0..weights.eq.hi.len())
            .map(block)
            .fold([F128::ZERO; 3], add)
    };
    // The high equality tensor sums to one. The public constant residual is
    // linear in the current selector variable, so its quadratic part is zero.
    add(sums, weights.constant)
}

const FOLD_TILE: usize = 1024;

#[inline]
fn packed_product(a: F128, b: F128) -> F128 {
    F128::new(a.lo & b.lo, a.hi & b.hi)
}

fn fill_virtual_product(a: &[F128], b: &[F128], output: &mut [F128]) {
    assert_eq!(a.len(), output.len());
    assert_eq!(b.len(), output.len());
    for ((value, &a), &b) in output.iter_mut().zip(a).zip(b) {
        *value = packed_product(a, b);
    }
}

fn message_virtual_product_block(
    wires: [&[F128]; 4],
    base: usize,
    weights: &RoundWeights,
    count: usize,
) -> [F128; 3] {
    let mut product = [F128::ZERO; FOLD_TILE];
    let mut sum = [F128::ZERO; 3];
    for offset in (0..count).step_by(FOLD_TILE / 2) {
        let pairs = (count - offset).min(FOLD_TILE / 2);
        let range = 2 * (base + offset)..2 * (base + offset + pairs);
        fill_virtual_product(
            &wires[0][range.clone()],
            &wires[1][range.clone()],
            &mut product[..2 * pairs],
        );
        let tile = [
            &wires[0][range.clone()],
            &wires[1][range.clone()],
            &product[..2 * pairs],
            &wires[3][range],
        ];
        let message = if let Some(products) = weights.selector_products {
            message_block_selector_simd_range(
                tile,
                0,
                &weights.coefficients,
                offset,
                pairs,
                products,
            )
        } else {
            message_block_range(tile, 0, &weights.coefficients, offset, pairs)
        };
        sum = add(sum, message);
    }
    sum
}

fn fold_virtual_product_tile(
    a: &[F128],
    b: &[F128],
    challenge: F128,
    start: usize,
    destination: &mut [MaybeUninit<F128>],
) {
    let mut product = [F128::ZERO; 2 * FOLD_TILE];
    let source = &mut product[..2 * destination.len()];
    let range = 2 * start..2 * (start + destination.len());
    fill_virtual_product(&a[range.clone()], &b[range], source);
    fold_tile(source, challenge, 0, destination);
}

fn fold_virtual_product_into(a: &[F128], b: &[F128], challenge: F128, output: &mut Vec<F128>) {
    let len = a.len() / 2;
    let uninit = inner_product::prepare_output(output, len);
    let block = |(i, destination): (usize, &mut [MaybeUninit<F128>])| {
        fold_virtual_product_tile(a, b, challenge, i * FOLD_TILE, destination);
    };
    if a.len() >= 1 << 15 && rayon::current_num_threads() > 1 {
        uninit.par_chunks_mut(FOLD_TILE).enumerate().for_each(block);
    } else {
        uninit.chunks_mut(FOLD_TILE).enumerate().for_each(block);
    }
    // SAFETY: all output slots were initialized by joined, disjoint workers.
    unsafe { output.set_len(len) };
}

fn fold_tile(
    source: &[F128],
    challenge: F128,
    start: usize,
    destination: &mut [MaybeUninit<F128>],
) {
    // Reuse Flock's native pair-fold kernel on a bounded initialized tile.
    // This avoids a whole-array zeroing pass and keeps intermediate writes
    // in cache, without exposing uninitialized values as an F128 slice.
    let mut tile = [F128::ZERO; FOLD_TILE];
    flock_field::f128_slice::fold_pairs(source, start, &mut tile[..destination.len()], challenge);
    // SAFETY: source tile is initialized; destination has the same length,
    // is disjoint from tile, and this worker exclusively owns every slot.
    unsafe {
        std::ptr::copy_nonoverlapping(
            tile.as_ptr(),
            destination.as_mut_ptr().cast::<F128>(),
            destination.len(),
        );
    }
}

fn fold_into(source: &[F128], challenge: F128, output: &mut Vec<F128>) {
    let len = source.len() / 2;
    let uninit = inner_product::prepare_output(output, len);
    let block = |(i, destination): (usize, &mut [MaybeUninit<F128>])| {
        fold_tile(source, challenge, i * FOLD_TILE, destination);
    };
    if source.len() >= 1 << 15 && rayon::current_num_threads() > 1 {
        uninit.par_chunks_mut(FOLD_TILE).enumerate().for_each(block);
    } else {
        uninit.chunks_mut(FOLD_TILE).enumerate().for_each(block);
    }
    // SAFETY: all output slots were initialized by joined, disjoint workers.
    unsafe { output.set_len(len) };
}

/// Fold all four original wires and immediately consume the initialized
/// output tiles for the next unscaled message. Its public suffix and folded
/// selector state are already known; no challenge or message is observed here.
fn fold_and_message(
    source: [&[F128]; 4],
    challenge: F128,
    suffix: &[F128],
    selectors: &[Vec<F128>; 5],
    basis_sum: F128,
    output: &mut [Vec<F128>; 4],
) -> [F128; 3] {
    fold_and_message_tiled::<{ FOLD_TILE / 2 }>(
        source, challenge, suffix, selectors, basis_sum, output,
    )
}

fn fold_and_message_tiled<const PAIR_TILE: usize>(
    source: [&[F128]; 4],
    challenge: F128,
    suffix: &[F128],
    selectors: &[Vec<F128>; 5],
    basis_sum: F128,
    output: &mut [Vec<F128>; 4],
) -> [F128; 3] {
    fold_and_message_source::<PAIR_TILE>(
        source, challenge, suffix, selectors, basis_sum, output, false,
    )
}

fn fold_and_message_source<const PAIR_TILE: usize>(
    source: [&[F128]; 4],
    challenge: F128,
    suffix: &[F128],
    selectors: &[Vec<F128>; 5],
    basis_sum: F128,
    output: &mut [Vec<F128>; 4],
    virtual_product: bool,
) -> [F128; 3] {
    let len = source[0].len() / 2;
    assert!(PAIR_TILE.is_power_of_two() && PAIR_TILE <= FOLD_TILE / 2);
    assert!(len >= 2 && len.is_power_of_two());
    assert!(
        source
            .iter()
            .enumerate()
            .all(|(j, wire)| { (virtual_product && j == 2) || wire.len() == 2 * len })
    );
    let weights = RoundWeights::new(suffix, selectors, basis_sum);
    assert_eq!(len / 2, weights.eq.lo.len() * weights.eq.hi.len());
    // Both lengths are powers of two: a tile never crosses a low-table
    // boundary, and every output pair is consumed by exactly one worker.
    let tile = 2 * weights.eq.lo.len().min(PAIR_TILE);
    let consume = |(block, destinations): (usize, [&mut [MaybeUninit<F128>]; 4])| {
        let start = block * tile;
        for j in 0..4 {
            if virtual_product && j == 2 {
                fold_virtual_product_tile(source[0], source[1], challenge, start, destinations[j]);
            } else {
                fold_tile(source[j], challenge, start, destinations[j]);
            }
        }
        // SAFETY: fold_tile initialized all four disjoint destination tiles
        // before these shared reads. No worker can access another's tile;
        // output Vec lengths remain zero until every worker joins.
        let folded: [&[F128]; 4] = destinations.each_ref().map(|destination| unsafe {
            std::slice::from_raw_parts(destination.as_ptr().cast::<F128>(), destination.len())
        });
        let first_pair = start / 2;
        let weight_offset = first_pair % weights.eq.lo.len();
        let count = folded[0].len() / 2;
        let message = if let Some(products) = weights.selector_products {
            message_block_selector_simd_range(
                folded,
                0,
                &weights.coefficients,
                weight_offset,
                count,
                products,
            )
        } else {
            message_block_range(folded, 0, &weights.coefficients, weight_offset, count)
        };
        message.map(|value| value * weights.eq.hi[first_pair / weights.eq.lo.len()])
    };
    let [a, b, m, o] = output
        .each_mut()
        .map(|wire| inner_product::prepare_output(wire, len));
    let sums = if source[0].len() >= 1 << 15 && rayon::current_num_threads() > 1 {
        a.par_chunks_mut(tile)
            .zip(b.par_chunks_mut(tile))
            .zip(m.par_chunks_mut(tile))
            .zip(o.par_chunks_mut(tile))
            .enumerate()
            .map(|(block, (((a, b), m), o))| consume((block, [a, b, m, o])))
            .reduce(|| [F128::ZERO; 3], add)
    } else {
        a.chunks_mut(tile)
            .zip(b.chunks_mut(tile))
            .zip(m.chunks_mut(tile))
            .zip(o.chunks_mut(tile))
            .enumerate()
            .map(|(block, (((a, b), m), o))| consume((block, [a, b, m, o])))
            .fold([F128::ZERO; 3], add)
    };
    // SAFETY: every output slot was initialized, then read only within its
    // owning tile, and all workers have joined. A panic leaves lengths zero.
    for wire in output {
        unsafe { wire.set_len(len) };
    }
    // As in round_message, the public constant has no quadratic term, and
    // the high equality tensor sums to one even for zero/one suffix entries.
    add(sums, weights.constant)
}

/// Prove gates for 128 executions. Here `log_n` counts gates PER execution,
/// and each element of a wire packs its 128 binary values in the fixed basis.
/// `basis_sum` is the sum of these 128 basis elements and is public input.
/// Source commitments and public I/O must be bound before calling this entry.
pub fn prove<C: Challenger>(
    wires: [&[F128]; 4],
    selectors: &RepeatedSelectors,
    log_n: usize,
    basis_sum: F128,
    challenger: &mut C,
) -> Result<(Proof, Claims), Error> {
    prove_source(wires, selectors, log_n, basis_sum, challenger, false, None)
}

/// Prove gates with the virtual wire M generated as the bitwise AND of the
/// initial packed L and R. The supplied wires are `[L,R,O]`. This emits exactly
/// the same proof as `prove([L,R,L & R,O], ...)`, without an N-element M array.
///
/// This entry is for the fixed binary basis in which packed coordinates are
/// the word bits. It preserves M's terminal claim: callers must still perform
/// the weighted Hadamard reduction and authenticate the resulting sources.
/// Later rounds fold M as an independent extension-field wire.
pub fn prove_virtual_product<C: Challenger>(
    wires: [&[F128]; 3],
    selectors: &RepeatedSelectors,
    log_n: usize,
    basis_sum: F128,
    challenger: &mut C,
) -> Result<(Proof, Claims), Error> {
    prove_source(
        [wires[0], wires[1], &[], wires[2]],
        selectors,
        log_n,
        basis_sum,
        challenger,
        true,
        None,
    )
}

/// As `prove_virtual_product`, retaining two now-dead fold buffers for the
/// subsequent weighted Hadamard IP. No witness values in these buffers are
/// meaningful after this call; the consumer must overwrite them before use.
/// Buffer ownership is explicit, so release remains in the caller's online
/// proof lifecycle instead of a persistent pool.
pub fn prove_virtual_product_with_fold_buffers<C: Challenger>(
    wires: [&[F128]; 3],
    selectors: &RepeatedSelectors,
    log_n: usize,
    basis_sum: F128,
    challenger: &mut C,
) -> Result<(Proof, Claims, [Vec<F128>; 2]), Error> {
    let mut fold_buffers = std::array::from_fn(|_| Vec::new());
    let (proof, claims) = prove_source(
        [wires[0], wires[1], &[], wires[2]],
        selectors,
        log_n,
        basis_sum,
        challenger,
        true,
        Some(&mut fold_buffers),
    )?;
    Ok((proof, claims, fold_buffers))
}

fn prove_source<C: Challenger>(
    wires: [&[F128]; 4],
    selectors: &RepeatedSelectors,
    log_n: usize,
    basis_sum: F128,
    challenger: &mut C,
    virtual_product: bool,
    fold_buffers: Option<&mut [Vec<F128>; 2]>,
) -> Result<(Proof, Claims), Error> {
    let n = checked_len(log_n, selectors.period_log)?;
    for (j, wire) in wires.into_iter().enumerate() {
        if virtual_product && j == 2 {
            continue;
        }
        if wire.len() != n {
            return Err(Error::InputLength {
                input: "packed gate wire",
                expected: n,
                got: wire.len(),
            });
        }
    }
    let mut selector_state = selectors.initial_state(log_n)?;
    let x = start(log_n, selectors.period_log, basis_sum, challenger);
    let mut point = Vec::with_capacity(log_n);
    let mut rounds = Vec::with_capacity(log_n);
    let mut claim = F128::ZERO;
    let mut prefix = F128::ONE;
    let mut state: [Vec<F128>; 4] = std::array::from_fn(|_| Vec::new());
    let mut next: [Vec<F128>; 4] = std::array::from_fn(|_| Vec::new());
    let fusion = gate_fusion_enabled();
    let trace = std::env::var_os("E2E_PIOP_TRACE").is_some();
    let mut phase_ms = [0.0; 3]; // standalone message, standalone fold, fused fold+message
    let mut cached_core = None;
    for i in 0..log_n {
        let current: [&[F128]; 4] = if i == 0 {
            wires
        } else {
            std::array::from_fn(|j| state[j].as_slice())
        };
        let core = if let Some(core) = cached_core.take() {
            core
        } else {
            let timer = trace.then(std::time::Instant::now);
            let core = if virtual_product && i == 0 {
                round_message_source(current, &x[i + 1..], &selector_state, basis_sum, true)
            } else {
                round_message(current, &x[i + 1..], &selector_state, basis_sum)
            };
            if let Some(timer) = timer {
                phase_ms[0] += timer.elapsed().as_secs_f64() * 1000.0;
            }
            core
        };
        if prefix * ((F128::ONE + x[i]) * core[0] + x[i] * core[1]) != claim {
            return Err(Error::InnerProductMismatch);
        }
        let message = cubic(core, x[i], prefix);
        challenger.observe_f128_slice(&message);
        let r = challenger.sample_f128();
        claim = evaluate(message, claim, r);
        prefix *= F128::ONE + x[i] + r;
        if fusion && i + 1 < log_n {
            let timer = trace.then(std::time::Instant::now);
            for selector in &mut selector_state {
                if selector.len() > 1 {
                    flock_multilinear::fold_low(selector, r);
                }
            }
            cached_core = Some(if virtual_product && i == 0 {
                fold_and_message_source::<{ FOLD_TILE / 2 }>(
                    current,
                    r,
                    &x[i + 2..],
                    &selector_state,
                    basis_sum,
                    &mut next,
                    true,
                )
            } else {
                fold_and_message(
                    current,
                    r,
                    &x[i + 2..],
                    &selector_state,
                    basis_sum,
                    &mut next,
                )
            });
            if let Some(timer) = timer {
                phase_ms[2] += timer.elapsed().as_secs_f64() * 1000.0;
            }
        } else {
            let timer = trace.then(std::time::Instant::now);
            for j in 0..4 {
                if virtual_product && i == 0 && j == 2 {
                    fold_virtual_product_into(current[0], current[1], r, &mut next[j]);
                } else {
                    fold_into(current[j], r, &mut next[j]);
                }
            }
            for selector in &mut selector_state {
                if selector.len() > 1 {
                    flock_multilinear::fold_low(selector, r);
                }
            }
            if let Some(timer) = timer {
                phase_ms[1] += timer.elapsed().as_secs_f64() * 1000.0;
            }
        }
        std::mem::swap(&mut state, &mut next);
        rounds.push(message);
        point.push(r);
    }
    let proof = Proof {
        rounds,
        selectors: std::array::from_fn(|j| selector_state[j][0]),
        wires: std::array::from_fn(|j| {
            if log_n != 0 {
                state[j][0]
            } else if virtual_product && j == 2 {
                packed_product(wires[0][0], wires[1][0])
            } else {
                wires[j][0]
            }
        }),
    };
    observe_terminals(&proof, challenger);
    if claim != prefix * residual(&proof.selectors, &proof.wires, basis_sum) {
        return Err(Error::InnerProductMismatch);
    }
    let claims = Claims {
        point,
        selectors: proof.selectors,
        wires: proof.wires,
    };
    if let Some(buffers) = fold_buffers {
        for j in 0..2 {
            // State and next swap each round, so either one can own the
            // original N/2 allocation. No buffer is handed off on an error.
            let source = if state[j].capacity() >= next[j].capacity() {
                &mut state[j]
            } else {
                &mut next[j]
            };
            buffers[j] = std::mem::take(source);
            buffers[j].clear();
        }
    }
    if trace {
        eprintln!(
            "mikros_multiple_gate log_n={log_n} fusion={fusion} virtual_product={virtual_product} selector_simd={} message_ms={:.6} fold_ms={:.6} fused_fold_message_ms={:.6}",
            selector_simd_enabled(),
            phase_ms[0],
            phase_ms[1],
            phase_ms[2],
        );
    }
    Ok((proof, claims))
}

/// Verify only the sumcheck and terminal gate relation. No original selector
/// or witness vector is accepted here: the caller must authenticate the claims
/// through its PCS and the virtual-product Hadamard reduction.
pub fn verify<C: Challenger>(
    proof: &Proof,
    log_n: usize,
    period_log: usize,
    basis_sum: F128,
    challenger: &mut C,
) -> Result<Claims, Error> {
    checked_len(log_n, period_log)?;
    if proof.rounds.len() != log_n {
        return Err(Error::ProofShape("multiple gate rounds"));
    }
    let x = start(log_n, period_log, basis_sum, challenger);
    let mut point = Vec::with_capacity(log_n);
    let mut claim = F128::ZERO;
    let mut equality = F128::ONE;
    for (i, &message) in proof.rounds.iter().enumerate() {
        challenger.observe_f128_slice(&message);
        let r = challenger.sample_f128();
        claim = evaluate(message, claim, r);
        equality *= F128::ONE + x[i] + r;
        point.push(r);
    }
    observe_terminals(proof, challenger);
    if claim != equality * residual(&proof.selectors, &proof.wires, basis_sum) {
        return Err(Error::InnerProductMismatch);
    }
    Ok(Claims {
        point,
        selectors: proof.selectors,
        wires: proof.wires,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_field::test_rng::Rng;
    use flock_transcript::challenger::FsChallenger;

    fn challenger() -> FsChallenger {
        FsChallenger::new(b"multiple-gate-tests-v1")
    }

    fn eq_index(point: &[F128], index: usize) -> F128 {
        point.iter().enumerate().fold(F128::ONE, |v, (i, &r)| {
            v * if index >> i & 1 == 0 {
                F128::ONE + r
            } else {
                r
            }
        })
    }

    fn source_value(source: &[F128], point: &[F128]) -> F128 {
        source
            .iter()
            .enumerate()
            .fold(F128::ZERO, |v, (i, &a)| v + a * eq_index(point, i))
    }

    fn selector_bit(selectors: &RepeatedSelectors, j: usize, gate: usize) -> F128 {
        let i = gate & ((1 << selectors.period_log) - 1);
        if selectors.packed[j][i / 8] >> (i % 8) & 1 == 0 {
            F128::ZERO
        } else {
            F128::ONE
        }
    }

    fn fixture(
        log_n: usize,
        period_log: usize,
        rng: &mut Rng,
    ) -> (RepeatedSelectors, [Vec<F128>; 4], F128) {
        let period = 1usize << period_log;
        let mut selectors = RepeatedSelectors {
            period_log,
            packed: std::array::from_fn(|_| vec![0; period.div_ceil(8)]),
        };
        for i in 0..period {
            for j in 0..5 {
                if j == 3 || rng.bit() {
                    selectors.packed[j][i / 8] |= 1 << (i % 8);
                }
            }
        }
        let n = 1usize << log_n;
        let l = rng.f128_vec(n);
        let r = rng.f128_vec(n);
        let m: Vec<_> = l
            .iter()
            .zip(&r)
            .map(|(a, b)| F128::new(a.lo & b.lo, a.hi & b.hi))
            .collect();
        let basis_sum = F128::new(u64::MAX, u64::MAX);
        let o = (0..n)
            .map(|i| {
                selector_bit(&selectors, 0, i) * l[i]
                    + selector_bit(&selectors, 1, i) * r[i]
                    + selector_bit(&selectors, 2, i) * m[i]
                    + selector_bit(&selectors, 4, i) * basis_sum
            })
            .collect();
        (selectors, [l, r, m, o], basis_sum)
    }

    fn assert_sources(selectors: &RepeatedSelectors, wires: &[Vec<F128>; 4], claims: &Claims) {
        for j in 0..4 {
            assert_eq!(source_value(&wires[j], &claims.point), claims.wires[j]);
        }
        for j in 0..5 {
            let value = (0..wires[0].len()).fold(F128::ZERO, |v, i| {
                v + selector_bit(selectors, j, i) * eq_index(&claims.point, i)
            });
            assert_eq!(value, claims.selectors[j]);
        }
    }

    fn selector_simd_state(q: Vec<F128>) -> [Vec<F128>; 5] {
        let len = q.len();
        [
            q.clone(),
            q.clone(),
            q.iter().map(|&v| F128::ONE + v).collect(),
            vec![F128::ONE; len],
            vec![F128::ZERO; len],
        ]
    }

    fn selector_simd_fixture(
        log_n: usize,
        q: &[bool],
        rng: &mut Rng,
    ) -> (RepeatedSelectors, [Vec<F128>; 4], F128) {
        let period_log = q.len().ilog2() as usize;
        assert_eq!(q.len(), 1 << period_log);
        let (mut selectors, mut wires, basis_sum) = fixture(log_n, period_log, rng);
        selectors.packed.iter_mut().for_each(|p| p.fill(0));
        for (i, &q) in q.iter().enumerate() {
            for (j, bit) in [q, q, !q, true, false].into_iter().enumerate() {
                if bit {
                    selectors.packed[j][i / 8] |= 1 << (i % 8);
                }
            }
        }
        for i in 0..wires[0].len() {
            wires[3][i] = if q[i & (q.len() - 1)] {
                wires[0][i] + wires[1][i]
            } else {
                wires[2][i]
            };
        }
        (selectors, wires, basis_sum)
    }

    #[test]
    fn virtual_product_tiles_match_materialized_first_message_and_fold() {
        let mut rng = Rng::new(0xA11D_710E);
        for (log_n, period_log) in [(1, 0), (7, 7), (10, 3), (13, 7), (15, 1)] {
            let (selectors, mut wires, basis_sum) = fixture(log_n, period_log, &mut rng);
            // Deliberately violate the gate relation: the first-round kernel
            // must compute the true residual, rather than assuming it is zero.
            wires[3][0] += F128::ONE;
            let state = selectors.initial_state(log_n).unwrap();
            let point = rng.f128_vec(log_n - 1);
            for simd in [false, true] {
                with_selector_simd(simd, || {
                    assert_eq!(
                        round_message(
                            wires.each_ref().map(Vec::as_slice),
                            &point,
                            &state,
                            basis_sum
                        ),
                        round_message_source(
                            [&wires[0], &wires[1], &[], &wires[3]],
                            &point,
                            &state,
                            basis_sum,
                            true,
                        )
                    );
                });
            }
            for r in [F128::ZERO, F128::ONE, rng.f128()] {
                let mut expected = Vec::new();
                fold_into(&wires[2], r, &mut expected);
                let mut actual = Vec::new();
                fold_virtual_product_into(&wires[0], &wires[1], r, &mut actual);
                assert_eq!(actual, expected);
            }
        }
        // A non-full final tile also needs every uninitialized slot written.
        let a = rng.f128_vec(2 * (FOLD_TILE + 3));
        let b = rng.f128_vec(a.len());
        let m: Vec<_> = a
            .iter()
            .zip(&b)
            .map(|(&a, &b)| packed_product(a, b))
            .collect();
        let r = rng.f128();
        let mut expected = Vec::new();
        fold_into(&m, r, &mut expected);
        let mut actual = Vec::new();
        fold_virtual_product_into(&a, &b, r, &mut actual);
        assert_eq!(actual, expected);
    }

    #[test]
    fn virtual_product_preserves_proof_claims_and_challenges_across_gate_kernels() {
        let mut rng = Rng::new(0xA11D_F011);
        for (log_n, period_log) in [(0, 0), (1, 0), (1, 1), (7, 7), (10, 3), (15, 1)] {
            let q: Vec<_> = (0..1 << period_log).map(|i| i & 1 != 0).collect();
            for (selectors, wires, basis_sum) in [
                fixture(log_n, period_log, &mut rng),
                selector_simd_fixture(log_n, &q, &mut rng),
            ] {
                let mut scripts = vec![vec![]];
                if log_n <= 10 {
                    scripts.extend([
                        vec![F128::ZERO; 2 * log_n],
                        vec![F128::ONE; 2 * log_n],
                        (0..2 * log_n)
                            .map(|i| if i & 1 == 0 { F128::ZERO } else { F128::ONE })
                            .collect(),
                    ]);
                }
                for script in scripts {
                    let expected = with_gate_fusion(false, || {
                        with_selector_simd(false, || {
                            let mut ch = GateFusionChallenger::new(&script);
                            let (p, c) = prove(
                                wires.each_ref().map(Vec::as_slice),
                                &selectors,
                                log_n,
                                basis_sum,
                                &mut ch,
                            )
                            .unwrap();
                            (bincode::serialize(&p).unwrap(), c, ch.sample_f128())
                        })
                    });
                    for fusion in [false, true] {
                        for simd in [false, true] {
                            with_gate_fusion(fusion, || {
                                with_selector_simd(simd, || {
                                    let mut prover = GateFusionChallenger::new(&script);
                                    let (proof, claims) = prove_virtual_product(
                                        [&wires[0], &wires[1], &wires[3]],
                                        &selectors,
                                        log_n,
                                        basis_sum,
                                        &mut prover,
                                    )
                                    .unwrap();
                                    let mut verifier = GateFusionChallenger::new(&script);
                                    assert_eq!(
                                        verify(&proof, log_n, period_log, basis_sum, &mut verifier)
                                            .unwrap(),
                                        claims
                                    );
                                    if log_n <= 10 {
                                        assert_sources(&selectors, &wires, &claims);
                                    }
                                    let next = prover.sample_f128();
                                    assert_eq!(next, verifier.sample_f128());
                                    assert_eq!(
                                        (bincode::serialize(&proof).unwrap(), claims, next),
                                        expected,
                                        "log_n={log_n}, period_log={period_log}, fusion={fusion}, simd={simd}"
                                    );
                                })
                            });
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn virtual_product_rejects_bad_output_and_tampered_messages_or_terminals() {
        let mut rng = Rng::new(0xA11D_BAD0);
        let (selectors, wires, basis_sum) = selector_simd_fixture(8, &[false, true], &mut rng);
        for fusion in [false, true] {
            for simd in [false, true] {
                with_gate_fusion(fusion, || {
                    with_selector_simd(simd, || {
                        for bad_row in [0, 1] {
                            let mut output = wires[3].clone();
                            output[bad_row] += F128::ONE;
                            assert!(
                                prove_virtual_product(
                                    [&wires[0], &wires[1], &output],
                                    &selectors,
                                    8,
                                    basis_sum,
                                    &mut challenger()
                                )
                                .is_err()
                            );
                        }
                        let (proof, _) = prove_virtual_product(
                            [&wires[0], &wires[1], &wires[3]],
                            &selectors,
                            8,
                            basis_sum,
                            &mut challenger(),
                        )
                        .unwrap();
                        for j in 0..4 {
                            let mut bad = proof.clone();
                            bad.wires[j] += F128::ONE;
                            assert!(verify(&bad, 8, 1, basis_sum, &mut challenger()).is_err());
                        }
                        for j in 0..3 {
                            let mut bad = proof.clone();
                            bad.rounds[0][j] += F128::ONE;
                            assert!(verify(&bad, 8, 1, basis_sum, &mut challenger()).is_err());
                        }
                    })
                });
            }
        }
    }

    #[test]
    fn virtual_product_validates_all_supplied_wires_before_transcript() {
        let mut rng = Rng::new(0xA11D_1E00);
        let (selectors, wires, basis_sum) = fixture(7, 7, &mut rng);
        for j in 0..3 {
            let mut inputs = [&wires[0][..], &wires[1][..], &wires[3][..]];
            inputs[j] = &inputs[j][..127];
            let mut ch = challenger();
            assert!(matches!(
                prove_virtual_product(inputs, &selectors, 7, basis_sum, &mut ch),
                Err(Error::InputLength { .. })
            ));
            assert_eq!(ch.sample_f128(), challenger().sample_f128());
        }
        let inputs = [&wires[0][..], &wires[1][..], &wires[3][..]];
        let mut bad = selectors;
        bad.packed[2].pop();
        let mut ch = challenger();
        assert!(prove_virtual_product(inputs, &bad, 7, basis_sum, &mut ch).is_err());
        assert_eq!(ch.sample_f128(), challenger().sample_f128());
    }

    #[test]
    fn virtual_product_fold_buffer_handoff_preserves_proof_and_retains_largest_capacity() {
        let mut rng = Rng::new(0xA11D_B0FF);
        // Both parities matter: after an odd/even round count, the N/2
        // allocation lives in opposite sides of the state/next pair.
        for log_n in [0, 1, 2, 7, 8, 15] {
            let (selectors, wires, basis_sum) = fixture(log_n, log_n.min(3), &mut rng);
            for fusion in [false, true] {
                for simd in [false, true] {
                    with_gate_fusion(fusion, || {
                        with_selector_simd(simd, || {
                            let inputs = [&wires[0][..], &wires[1][..], &wires[3][..]];
                            let mut original = challenger();
                            let expected = prove_virtual_product(
                                inputs,
                                &selectors,
                                log_n,
                                basis_sum,
                                &mut original,
                            )
                            .unwrap();
                            let mut retained = challenger();
                            let (proof, claims, buffers) = prove_virtual_product_with_fold_buffers(
                                inputs,
                                &selectors,
                                log_n,
                                basis_sum,
                                &mut retained,
                            )
                            .unwrap();
                            assert_eq!((proof, claims), expected);
                            assert_eq!(retained.sample_f128(), original.sample_f128());
                            for buffer in buffers {
                                assert!(buffer.is_empty());
                                assert!(buffer.capacity() >= (1usize << log_n) / 2);
                            }
                        })
                    });
                }
            }
        }
    }

    #[test]
    fn gate_fusion_folded_arrays_and_next_message_match_reference_for_all_tiles() {
        let mut rng = Rng::new(0x6A7E_F051);
        for log_n in [2usize, 3, 4, 8, 12, 15] {
            let n = 1 << log_n;
            let source: [Vec<F128>; 4] = std::array::from_fn(|_| rng.f128_vec(n));
            let period = 1 << (log_n - 1).min(7);
            let general = std::array::from_fn(|_| rng.f128_vec(period));
            let matched = selector_simd_state(rng.f128_vec(period));
            for selectors in [general, matched] {
                for challenge in [F128::ZERO, F128::ONE, rng.f128()] {
                    let expected: [Vec<F128>; 4] = std::array::from_fn(|j| {
                        source[j]
                            .chunks_exact(2)
                            .map(|p| p[0] + challenge * (p[0] + p[1]))
                            .collect()
                    });
                    for suffix in [
                        rng.f128_vec(log_n - 2),
                        vec![F128::ZERO; log_n - 2],
                        vec![F128::ONE; log_n - 2],
                    ] {
                        let basis_sum = rng.f128();
                        let expected_message = with_selector_simd(false, || {
                            round_message(
                                expected.each_ref().map(Vec::as_slice),
                                &suffix,
                                &selectors,
                                basis_sum,
                            )
                        });
                        for simd in [false, true] {
                            with_selector_simd(simd, || {
                                let mut output = std::array::from_fn(|_| vec![F128::ONE; n + 3]);
                                let actual = fold_and_message(
                                    source.each_ref().map(Vec::as_slice),
                                    challenge,
                                    &suffix,
                                    &selectors,
                                    basis_sum,
                                    &mut output,
                                );
                                assert_eq!(actual, expected_message, "log_n={log_n}, simd={simd}");
                                assert_eq!(output, expected);
                                // Tiny tiles force nonzero low-table offsets and
                                // scalar tails; size four also exercises full
                                // four-lane messages on the native build.
                                if log_n <= 12 {
                                    let tiny = fold_and_message_tiled::<1>(
                                        source.each_ref().map(Vec::as_slice),
                                        challenge,
                                        &suffix,
                                        &selectors,
                                        basis_sum,
                                        &mut output,
                                    );
                                    assert_eq!(tiny, expected_message);
                                    assert_eq!(output, expected);
                                    let vector = fold_and_message_tiled::<4>(
                                        source.each_ref().map(Vec::as_slice),
                                        challenge,
                                        &suffix,
                                        &selectors,
                                        basis_sum,
                                        &mut output,
                                    );
                                    assert_eq!(vector, expected_message);
                                    assert_eq!(output, expected);
                                }
                            });
                        }
                    }
                }
            }
        }
        with_gate_fusion(false, || {
            assert!(!gate_fusion_enabled());
            with_gate_fusion(true, || assert!(gate_fusion_enabled()));
            assert!(!gate_fusion_enabled());
        });
    }

    /// Force boundary challenges while still absorbing every event into a
    /// real FS transcript. Once the script ends, its next sample therefore
    /// checks the full observed-message and squeeze order across variants.
    struct GateFusionChallenger {
        inner: FsChallenger,
        forced: std::collections::VecDeque<F128>,
    }
    impl GateFusionChallenger {
        fn new(forced: &[F128]) -> Self {
            Self {
                inner: challenger(),
                forced: forced.iter().copied().collect(),
            }
        }
    }
    impl Challenger for GateFusionChallenger {
        fn observe_label(&mut self, label: &[u8]) {
            self.inner.observe_label(label);
        }
        fn observe_bytes(&mut self, bytes: &[u8]) {
            self.inner.observe_bytes(bytes);
        }
        fn observe_f128(&mut self, value: F128) {
            self.inner.observe_f128(value);
        }
        fn observe_f128_slice(&mut self, values: &[F128]) {
            self.inner.observe_f128_slice(values);
        }
        fn sample_f128(&mut self) -> F128 {
            let sample = self.inner.sample_f128();
            self.forced.pop_front().unwrap_or(sample)
        }
        fn sample_f128_vec(&mut self, n: usize) -> Vec<F128> {
            self.inner
                .sample_f128_vec(n)
                .into_iter()
                .map(|sample| self.forced.pop_front().unwrap_or(sample))
                .collect()
        }
        fn fork_from_seed(&self, seed: [F128; 2], label: &'static [u8]) -> Self {
            Self {
                inner: self.inner.fork_from_seed(seed, label),
                forced: Default::default(),
            }
        }
    }

    #[test]
    fn gate_fusion_four_flag_combinations_preserve_proof_and_next_challenge() {
        let mut rng = Rng::new(0x6A7E_F052);
        for (log_n, period_log) in [
            (0, 0),
            (1, 0),
            (1, 1),
            (2, 1),
            (3, 3),
            (7, 7),
            (10, 3),
            (15, 1),
        ] {
            let arbitrary = fixture(log_n, period_log, &mut rng);
            let q: Vec<_> = (0..1 << period_log).map(|i| i & 1 != 0).collect();
            let matched = selector_simd_fixture(log_n, &q, &mut rng);
            for (selectors, wires, basis_sum) in [arbitrary, matched] {
                let scripts = [
                    vec![], // Fully Fiat-Shamir; no forced challenge.
                    vec![F128::ZERO; 2 * log_n],
                    vec![F128::ONE; 2 * log_n],
                    (0..2 * log_n)
                        .map(|i| if i & 1 == 0 { F128::ZERO } else { F128::ONE })
                        .collect(),
                ];
                for script in scripts {
                    let mut baseline = None;
                    for (fusion, simd) in
                        [(false, false), (false, true), (true, false), (true, true)]
                    {
                        let result = with_gate_fusion(fusion, || {
                            with_selector_simd(simd, || {
                                let mut prover = GateFusionChallenger::new(&script);
                                let (proof, claims) = prove(
                                    wires.each_ref().map(Vec::as_slice),
                                    &selectors,
                                    log_n,
                                    basis_sum,
                                    &mut prover,
                                )
                                .unwrap();
                                assert!(prover.forced.is_empty());
                                let mut verifier = GateFusionChallenger::new(&script);
                                assert_eq!(
                                    verify(&proof, log_n, period_log, basis_sum, &mut verifier)
                                        .unwrap(),
                                    claims
                                );
                                let next = prover.sample_f128();
                                assert_eq!(next, verifier.sample_f128());
                                if log_n <= 10 {
                                    assert_sources(&selectors, &wires, &claims);
                                }
                                (bincode::serialize(&proof).unwrap(), claims, next)
                            })
                        });
                        if let Some(expected) = &baseline {
                            assert_eq!(
                                &result, expected,
                                "log_n={log_n}, period_log={period_log}, fusion={fusion}, simd={simd}"
                            );
                        } else {
                            baseline = Some(result);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn gate_fusion_preserves_invalid_wire_and_virtual_product_rejection() {
        // Keep the existing independent gate/transcript mutation checks and
        // the real weighted-Hadamard fake-M check active in both fusion arms.
        for fusion in [false, true] {
            with_gate_fusion(fusion, || {
                selector_simd_fake_virtual_product_is_still_authenticated();
                for simd in [false, true] {
                    with_selector_simd(simd, bad_gate_messages_and_terminal_values_are_rejected);
                }
            });
        }
    }

    #[test]
    fn selector_simd_rounds_and_unaligned_tails_match_scalar_oracle() {
        let mut rng = Rng::new(0x51EC_70A1);
        let n = 512;
        // Arbitrary M and O are deliberate: this kernel must not use witness
        // satisfaction, including at the first un-folded selector pattern.
        let wires: [Vec<F128>; 4] = std::array::from_fn(|_| rng.f128_vec(n));
        let random = rng.f128();
        let patterns = [
            vec![F128::ZERO],
            vec![F128::ONE],
            vec![random],
            vec![F128::ZERO, F128::ONE],
            vec![F128::ONE, F128::ZERO],
            vec![random, random],
            rng.f128_vec(8),
            rng.f128_vec(128),
        ];
        for q in patterns {
            let selectors = selector_simd_state(q);
            let period = selectors[0].len();
            for point in [rng.f128_vec(8), vec![F128::ZERO; 8], vec![F128::ONE; 8]] {
                let basis_sum = rng.f128();
                let weights =
                    with_selector_simd(true, || RoundWeights::new(&point, &selectors, basis_sum));
                let products = weights.selector_products.unwrap();
                for count in [0, 1, 2, 3, 4, 5, 6, 7, 8, 11, 16] {
                    for base in [0, 3] {
                        assert_eq!(
                            message_block_selector_simd(
                                wires.each_ref().map(Vec::as_slice),
                                base,
                                &weights.coefficients,
                                count,
                                products,
                            ),
                            message_block(
                                wires.each_ref().map(Vec::as_slice),
                                base,
                                &weights.coefficients,
                                count,
                            ),
                            "period={period}, count={count}, base={base}"
                        );
                    }
                }
                let mut expected = [F128::ZERO; 3];
                for i in 0..n / 2 {
                    let q0: [F128; 5] = std::array::from_fn(|j| selectors[j][2 * i & (period - 1)]);
                    let q1: [F128; 5] =
                        std::array::from_fn(|j| selectors[j][(2 * i + 1) & (period - 1)]);
                    let w0 = std::array::from_fn(|j| wires[j][2 * i]);
                    let w1 = std::array::from_fn(|j| wires[j][2 * i + 1]);
                    let weight = eq_index(&point, i);
                    expected[0] += weight * residual(&q0, &w0, basis_sum);
                    expected[1] += weight * residual(&q1, &w1, basis_sum);
                    expected[2] += weight
                        * (0..4).fold(F128::ZERO, |v, j| v + (q0[j] + q1[j]) * (w0[j] + w1[j]));
                }
                for enabled in [false, true] {
                    assert_eq!(
                        with_selector_simd(enabled, || {
                            round_message(
                                wires.each_ref().map(Vec::as_slice),
                                &point,
                                &selectors,
                                basis_sum,
                            )
                        }),
                        expected
                    );
                }
            }
        }
    }

    #[test]
    fn selector_simd_checks_every_selector_entry_and_falls_back() {
        let mut rng = Rng::new(0x51EC_FA11);
        let selectors = selector_simd_state(rng.f128_vec(128));
        assert!(selector_residual_matches(&selectors));
        let wires: [Vec<F128>; 4] = std::array::from_fn(|_| rng.f128_vec(256));
        let point = rng.f128_vec(7);
        for j in 0..5 {
            for i in [0, 127] {
                let mut bad = selectors.clone();
                bad[j][i] += F128::ONE;
                assert!(!selector_residual_matches(&bad), "j={j}, i={i}");
                assert!(
                    with_selector_simd(true, || RoundWeights::new(&point, &bad, F128::ONE))
                        .selector_products
                        .is_none()
                );
                let messages = [false, true].map(|enabled| {
                    with_selector_simd(enabled, || {
                        round_message(wires.each_ref().map(Vec::as_slice), &point, &bad, F128::ONE)
                    })
                });
                assert_eq!(messages[0], messages[1]);
            }
            let mut short = selectors.clone();
            short[j].pop();
            assert!(!selector_residual_matches(&short));
        }
        assert!(!selector_residual_matches(&std::array::from_fn(|_| {
            Vec::new()
        })));
        with_selector_simd(false, || {
            assert!(!selector_simd_enabled());
            with_selector_simd(true, || assert!(selector_simd_enabled()));
            assert!(!selector_simd_enabled());
            assert!(
                RoundWeights::new(&point, &selectors, F128::ONE)
                    .selector_products
                    .is_none()
            );
        });
    }

    #[test]
    fn selector_simd_selects_three_and_four_product_patterns() {
        let mut rng = Rng::new(0x51EC_C057);
        let point = rng.f128_vec(8);
        let patterns = [
            (
                vec![F128::ZERO, F128::ONE],
                [SelectorProduct::V, SelectorProduct::Sum, SelectorProduct::U],
            ),
            (
                vec![F128::ONE, F128::ZERO],
                [SelectorProduct::Sum, SelectorProduct::V, SelectorProduct::U],
            ),
            (
                vec![F128::ZERO],
                [
                    SelectorProduct::V,
                    SelectorProduct::V,
                    SelectorProduct::Zero,
                ],
            ),
            (
                vec![F128::ONE],
                [
                    SelectorProduct::Sum,
                    SelectorProduct::Sum,
                    SelectorProduct::Zero,
                ],
            ),
            (
                vec![rng.f128()],
                [
                    SelectorProduct::Separate,
                    SelectorProduct::Separate,
                    SelectorProduct::Zero,
                ],
            ),
        ];
        for (q, expected) in patterns {
            let actual = with_selector_simd(true, || {
                RoundWeights::new(&point, &selector_simd_state(q), F128::ONE).selector_products
            });
            assert_eq!(actual, Some(expected));
        }
    }

    #[test]
    fn selector_simd_complete_proof_bytes_claims_and_next_challenge_are_identical() {
        let mut rng = Rng::new(0x51EC_9A00);
        let cases = [
            (0, vec![false]),
            (1, vec![true]),
            (1, vec![false, true]),
            (1, vec![true, false]),
            (7, vec![false; 8]),
            (7, vec![true; 8]),
            (10, (0..128).map(|_| rng.bit()).collect()),
            (15, vec![false, true]),
        ];
        let mut fixtures: Vec<_> = cases
            .into_iter()
            .map(|(log_n, q)| (log_n, selector_simd_fixture(log_n, &q, &mut rng)))
            .collect();
        // Unmatched public selectors must preserve the whole proof, too.
        let (mut selectors, mut wires, basis_sum) = fixture(10, 7, &mut rng);
        selectors.packed[4][0] |= 1;
        for i in 0..wires[0].len() {
            wires[3][i] = selector_bit(&selectors, 0, i) * wires[0][i]
                + selector_bit(&selectors, 1, i) * wires[1][i]
                + selector_bit(&selectors, 2, i) * wires[2][i]
                + selector_bit(&selectors, 4, i) * basis_sum;
        }
        fixtures.push((10, (selectors, wires, basis_sum)));
        for (log_n, (selectors, wires, basis_sum)) in fixtures {
            let results = [false, true].map(|enabled| {
                with_selector_simd(enabled, || {
                    let mut prover = challenger();
                    let (proof, claims) = prove(
                        wires.each_ref().map(Vec::as_slice),
                        &selectors,
                        log_n,
                        basis_sum,
                        &mut prover,
                    )
                    .unwrap();
                    let mut verifier = challenger();
                    assert_eq!(
                        verify(
                            &proof,
                            log_n,
                            selectors.period_log,
                            basis_sum,
                            &mut verifier
                        )
                        .unwrap(),
                        claims
                    );
                    let next = prover.sample_f128();
                    assert_eq!(next, verifier.sample_f128());
                    if log_n <= 10 {
                        assert_sources(&selectors, &wires, &claims);
                    }
                    (bincode::serialize(&proof).unwrap(), claims, next)
                })
            });
            assert_eq!(
                results[0], results[1],
                "log_n={log_n}, period={}",
                selectors.period_log
            );
        }
    }

    #[test]
    fn selector_simd_fake_virtual_product_is_still_authenticated() {
        let mut rng = Rng::new(0x51EC_BAD0);
        let (selectors, original, basis_sum) = selector_simd_fixture(8, &[false, true], &mut rng);
        for enabled in [false, true] {
            with_selector_simd(enabled, || {
                let mut wires = original.clone();
                wires[2][0] += F128::ONE; // AND row: gate relation already fails.
                assert!(
                    prove(
                        wires.each_ref().map(Vec::as_slice),
                        &selectors,
                        8,
                        basis_sum,
                        &mut challenger()
                    )
                    .is_err()
                );

                let mut wires = original.clone();
                wires[2][1] += F128::ONE; // XOR row: M still needs Hadamard authentication.
                let mut prover = challenger();
                let (proof, claims) = prove(
                    wires.each_ref().map(Vec::as_slice),
                    &selectors,
                    8,
                    basis_sum,
                    &mut prover,
                )
                .unwrap();
                assert_eq!(source_value(&wires[2], &claims.point), claims.wires[2]);
                assert_ne!(source_value(&original[2], &claims.point), claims.wires[2]);
                let mut bad_terminal = proof.clone();
                bad_terminal.wires[2] += F128::ONE;
                assert!(verify(&bad_terminal, 8, 1, basis_sum, &mut challenger()).is_err());
                let bytes = |input: &[F128]| -> Vec<u8> {
                    input
                        .iter()
                        .flat_map(|v| v.lo.to_le_bytes().into_iter().chain(v.hi.to_le_bytes()))
                        .collect()
                };
                let row_factors: Vec<_> = claims
                    .point
                    .iter()
                    .rev()
                    .map(|&r| [F128::ONE + r, r])
                    .collect();
                let column_weights: Vec<_> = (0..128)
                    .map(|i| {
                        if i < 64 {
                            F128::new(1u64 << i, 0)
                        } else {
                            F128::new(0, 1u64 << (i - 64))
                        }
                    })
                    .collect();
                assert!(matches!(
                    crate::weighted_hadamard::prove(
                        &bytes(&wires[0]),
                        &bytes(&wires[1]),
                        15,
                        &row_factors,
                        &column_weights,
                        claims.wires[2],
                        crate::Kernel::PairAggregate,
                        &mut prover,
                    ),
                    Err(Error::ColumnProjectionMismatch)
                ));
            });
        }
    }

    #[test]
    fn arbitrary_repeated_binary_selectors_match_all_nine_source_oracles() {
        let mut rng = Rng::new(0x5E1_EC702);
        for (log_n, period_log) in [(0, 0), (1, 0), (1, 1), (7, 7), (10, 1), (10, 7)] {
            let (selectors, wires, basis_sum) = fixture(log_n, period_log, &mut rng);
            let mut prover = challenger();
            let (proof, claims) = prove(
                wires.each_ref().map(Vec::as_slice),
                &selectors,
                log_n,
                basis_sum,
                &mut prover,
            )
            .unwrap();
            assert_eq!(proof.field_elements(), 3 * log_n + 9);
            let mut verifier = challenger();
            assert_eq!(
                verify(&proof, log_n, period_log, basis_sum, &mut verifier).unwrap(),
                claims
            );
            assert_eq!(prover.sample_f128(), verifier.sample_f128());
            assert_sources(&selectors, &wires, &claims);
        }
    }

    #[test]
    fn first_cubic_message_matches_direct_polynomial_values() {
        let mut rng = Rng::new(0xC0B1_Ca11);
        let (selectors, wires, basis_sum) = fixture(7, 7, &mut rng);
        let (proof, _) = prove(
            wires.each_ref().map(Vec::as_slice),
            &selectors,
            7,
            basis_sum,
            &mut challenger(),
        )
        .unwrap();
        let x = start(7, 7, basis_sum, &mut challenger());
        for t in [F128::ZERO, F128::ONE, rng.f128(), rng.f128()] {
            let mut expected = F128::ZERO;
            for i in 0..wires[0].len() / 2 {
                let q: [F128; 5] = std::array::from_fn(|j| {
                    let q0 = selector_bit(&selectors, j, 2 * i);
                    let q1 = selector_bit(&selectors, j, 2 * i + 1);
                    q0 + t * (q0 + q1)
                });
                let w: [F128; 4] = std::array::from_fn(|j| {
                    wires[j][2 * i] + t * (wires[j][2 * i] + wires[j][2 * i + 1])
                });
                expected +=
                    eq_index(&x[1..], i) * (F128::ONE + x[0] + t) * residual(&q, &w, basis_sum);
            }
            assert_eq!(evaluate(proof.rounds[0], F128::ZERO, t), expected);
        }
    }

    #[test]
    fn round_kernel_matches_scalar_for_general_folded_selectors_and_tails() {
        let mut rng = Rng::new(0xB10C_9E70);
        for log_n in [1usize, 2, 3, 7, 10, 15] {
            let n = 1usize << log_n;
            let wires: [Vec<F128>; 4] = std::array::from_fn(|_| rng.f128_vec(n));
            let period = 1usize << log_n.min(7);
            let mut selectors: [Vec<F128>; 5] = std::array::from_fn(|_| rng.f128_vec(period));
            selectors[0].fill(F128::ZERO);
            selectors[1].fill(F128::ONE);
            let point = rng.f128_vec(log_n - 1);
            let basis_sum = rng.f128();
            let mut expected = [F128::ZERO; 3];
            for i in 0..n / 2 {
                let q0: [F128; 5] = std::array::from_fn(|j| selectors[j][(2 * i) & (period - 1)]);
                let q1: [F128; 5] =
                    std::array::from_fn(|j| selectors[j][(2 * i + 1) & (period - 1)]);
                let w0 = std::array::from_fn(|j| wires[j][2 * i]);
                let w1 = std::array::from_fn(|j| wires[j][2 * i + 1]);
                let weight = eq_index(&point, i);
                expected[0] += weight * residual(&q0, &w0, basis_sum);
                expected[1] += weight * residual(&q1, &w1, basis_sum);
                expected[2] +=
                    weight * (0..4).fold(F128::ZERO, |v, j| v + (q0[j] + q1[j]) * (w0[j] + w1[j]));
            }
            let actual = round_message(
                wires.each_ref().map(Vec::as_slice),
                &point,
                &selectors,
                basis_sum,
            );
            assert_eq!(actual, expected, "log_n={log_n}");
        }
    }

    #[test]
    fn bad_gate_messages_and_terminal_values_are_rejected() {
        let mut rng = Rng::new(0xBAD_9A7E);
        let (mut selectors, mut wires, basis_sum) = fixture(8, 1, &mut rng);
        selectors.packed = [vec![2], vec![2], vec![1], vec![3], vec![0]];
        for i in 0..wires[0].len() {
            wires[3][i] = if i & 1 == 0 {
                wires[2][i]
            } else {
                wires[0][i] + wires[1][i]
            };
        }
        let (proof, claims) = prove(
            wires.each_ref().map(Vec::as_slice),
            &selectors,
            8,
            basis_sum,
            &mut challenger(),
        )
        .unwrap();
        for kind in 0..13 {
            let mut bad = proof.clone();
            match kind {
                0..=2 => bad.rounds[0][kind] += F128::ONE,
                3..=7 => bad.selectors[kind - 3] += F128::ONE,
                8..=11 => bad.wires[kind - 8] += F128::ONE,
                12 => {
                    bad.rounds.pop();
                }
                _ => unreachable!(),
            }
            assert!(
                verify(&bad, 8, 1, basis_sum, &mut challenger()).is_err(),
                "kind={kind}"
            );
        }
        assert!(verify(&proof, 8, 2, basis_sum, &mut challenger()).is_err());
        assert!(verify(&proof, 8, 1, basis_sum + F128::ONE, &mut challenger()).is_err());
        wires[3][0] += F128::ONE;
        assert!(
            prove(
                wires.each_ref().map(Vec::as_slice),
                &selectors,
                8,
                basis_sum,
                &mut challenger()
            )
            .is_err()
        );
        assert_ne!(source_value(&wires[3], &claims.point), claims.wires[3]);
    }

    #[test]
    fn dimensions_period_padding_and_native_fold_are_checked() {
        assert!(checked_len(0, 1).is_err());
        assert!(checked_len(10, 8).is_err());
        assert!(checked_len(usize::BITS as usize, 0).is_err());
        #[cfg(target_pointer_width = "64")]
        assert_eq!(checked_len(30, 7).unwrap(), 1usize << 30);
        let selectors = RepeatedSelectors {
            period_log: 1,
            packed: std::array::from_fn(|_| vec![4]),
        };
        assert!(selectors.initial_state(7).is_err());
        let mut rng = Rng::new(0xF01D_93);
        for n in [2, 4, 6, 2046, 2048, 2050, 32768, 32770] {
            let source = rng.f128_vec(n);
            for r in [F128::ZERO, F128::ONE, rng.f128()] {
                let mut output = vec![F128::ONE; n + 3];
                fold_into(&source, r, &mut output);
                let expected: Vec<_> = source
                    .chunks_exact(2)
                    .map(|p| p[0] + r * (p[0] + p[1]))
                    .collect();
                assert_eq!(output, expected);
            }
        }
    }
}
