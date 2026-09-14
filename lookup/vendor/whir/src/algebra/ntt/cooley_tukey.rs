//! Number-theoretic transforms (NTTs) over fields with high two-adicity.
//!
//! Implements the √N Cooley-Tukey six-step algorithm to achieve parallelism with good locality.
//! A global cache is used for twiddle factors.

use std::sync::{Arc, OnceLock, RwLock};

use ark_ff::{AdditiveGroup, FftField, Field};
#[cfg(feature = "tracing")]
use tracing::instrument;
#[cfg(feature = "parallel")]
use {crate::utils::workload_size, rayon::prelude::*, std::cmp::max};

use super::{transpose, utils::sqrt_factor_from_divisors, Messages, ReedSolomon};
#[cfg(not(feature = "rs_in_order"))]
use crate::algebra::ntt::transpose::transpose_permute;
use crate::{
    algebra::batch::{self, BatchArithmetic},
    algebra::ntt::utils::divisors,
    buffer::{Buffer, BufferOps},
    utils::{chunks_exact_or_empty, zip_strict},
};

// Supported primes
const PRIMES: [usize; 2] = [2, 3];
// Explicitly opted-in dense reference radices for low-two-adicity fields.
// Do not widen PRIMES: legacy FFT-field constructors keep their exact domains.
const EXTENDED_PRIMES: [usize; 5] = [2, 3, 5, 17, 181];

const RADER_PRIME: usize = 181;
const RADER_LENGTH: usize = RADER_PRIME - 1;
// Bound temporary extension-field storage, while amortizing the small NTT's
// transpose allocations across 32 transforms rather than allocating per row.
const RADER_BATCH_ROWS: usize = 32;
const BATCH_RADICES: [usize; 7] = [2, 3, 4, 5, 8, 16, 17];

#[derive(Clone, Copy)]
struct BatchContext<'a, F: Field> {
    kernel: Option<&'a dyn BatchArithmetic<F>>,
    // Only present for the full-E root path. The existing compact prime-root
    // cache is not expanded to a second large E table just to call a hook.
    full_roots: Option<&'a [F]>,
}

/// Fixed-root Rader plan. The independent convolution engine only sees order
/// 180, so it cannot evict the outer transform's (potentially huge) root cache.
#[derive(Debug)]
struct Rader181<F: Field> {
    convolution: NttEngine<F>,
    input_indices: [usize; RADER_LENGTH],
    output_indices: [usize; RADER_LENGTH],
    normalized_kernel_spectrum: Vec<F>,
}

/// Engine for computing NTTs over arbitrary fields.
/// Assumes the field has large two-adicity.
#[derive(Debug)]
pub struct NttEngine<F: Field> {
    order: usize,         // order of omega_orger
    divisors: Vec<usize>, // divisors of the order.
    omega_order: F,       // primitive order'th root.

    // Roots of small order (zero if unavailable). The naming convention is that omega_foo has order foo.
    half_omega_3_1_plus_2: F, // ½(ω₃ + ω₃²)
    half_omega_3_1_min_2: F,  // ½(ω₃ - ω₃²)
    omega_4_1: F,
    omega_8_1: F,
    omega_8_3: F,
    omega_16_1: F,
    omega_16_3: F,
    omega_16_9: F,

    // Immutable snapshots keep concurrent transforms valid when a different
    // mixed-radix order replaces the cache. Never grow to historical LCMs.
    roots: RwLock<Arc<Vec<F>>>,
    // A requested subgroup can be in the prime field even when omega_order is
    // not. Keep a separate cache so a larger extension subgroup cannot disable
    // or poison the smaller prime-field twiddle path.
    base_roots: RwLock<Arc<Vec<F::BasePrimeField>>>,
    base_small_roots: Option<[F::BasePrimeField; 8]>,
    // Immutable and published once for this engine's actual 181st root.
    // Concurrent cold callers may build duplicate small plans without holding
    // an initialization lock across nested Rayon work.
    rader181: OnceLock<Option<Box<Rader181<F>>>>,
    // At most 55 actual root powers, built lazily only for a registered kernel.
    // There is no large root-cache allocation or historical LCM growth here.
    batch_small_roots: OnceLock<[Vec<F>; 7]>,
    // An engine-local flag reaches all Rayon workers, unlike a thread-local
    // reference-mode switch. It is absent from production builds.
    #[cfg(test)]
    force_dense_181: bool,
}

impl<F: FftField> NttEngine<F> {
    /// Construct a new engine from the field's `FftField` trait.
    pub fn new_from_fftfield() -> Self {
        let (mut omega, mut order) = if let (Some(mut omega), Some(b), Some(k)) = (
            F::LARGE_SUBGROUP_ROOT_OF_UNITY,
            F::SMALL_SUBGROUP_BASE,
            F::SMALL_SUBGROUP_BASE_ADICITY,
        ) {
            // Extract supported subgroup q from small group b.
            let mut order = 1;
            let mut remaining = (b as usize).checked_pow(k).expect("Small group too large.");
            for p in PRIMES {
                while remaining.is_multiple_of(p) {
                    order *= p;
                    remaining /= p;
                }
            }
            omega = omega.pow([remaining as u64]);
            (omega, order)
        } else {
            (F::TWO_ADIC_ROOT_OF_UNITY, 1)
        };
        let twos = F::TWO_ADICITY.min(order.leading_zeros()) as usize;
        for _ in 0..(F::TWO_ADICITY as usize - twos) {
            omega.square_in_place();
        }
        order <<= twos;
        Self::new(order, omega)
    }
}

/// Creates a new NttEngine. `omega_order` must be a primitive root of unity of even order `omega`.
impl<F: Field> NttEngine<F> {
    pub fn new(order: usize, omega_order: F) -> Self {
        Self::new_with_supported_primes(order, omega_order, &PRIMES)
    }

    /// Construct an engine with explicitly selected radices. The default
    /// constructor remains restricted to 2 and 3. Radices 5 and 17 use dense
    /// prime DFT kernels. Radix 181 uses Rader's length-180 convolution when
    /// that subgroup is available in the prime field, and otherwise stays dense.
    ///
    /// Panics if the whitelist is not strictly sorted, contains an unsupported
    /// radix, misses an order factor, or the root does not have exactly `order`.
    pub fn new_with_supported_primes(
        order: usize,
        omega_order: F,
        supported_primes: &[usize],
    ) -> Self {
        assert!(order > 0, "root order must be positive");
        assert!(
            supported_primes.windows(2).all(|w| w[0] < w[1]),
            "supported primes must be sorted and unique"
        );
        assert!(
            supported_primes.iter().all(|p| EXTENDED_PRIMES.contains(p)),
            "unsupported prime radix"
        );
        let order_divisors = divisors(order, supported_primes);
        // Make sure `omega_order` is a primitive root of unity.
        assert_eq!(omega_order.pow([order as u64]), F::ONE);
        for &prime in supported_primes {
            if order.is_multiple_of(prime) {
                assert_ne!(omega_order.pow([(order / prime) as u64]), F::ONE);
            }
        }

        let mut res = Self {
            order,
            divisors: order_divisors,
            omega_order,
            half_omega_3_1_plus_2: F::ZERO,
            half_omega_3_1_min_2: F::ZERO,
            omega_4_1: F::ZERO,
            omega_8_1: F::ZERO,
            omega_8_3: F::ZERO,
            omega_16_1: F::ZERO,
            omega_16_3: F::ZERO,
            omega_16_9: F::ZERO,
            roots: RwLock::new(Arc::new(Vec::new())),
            base_roots: RwLock::new(Arc::new(Vec::new())),
            base_small_roots: None,
            rader181: OnceLock::new(),
            batch_small_roots: OnceLock::new(),
            #[cfg(test)]
            force_dense_181: false,
        };
        if order.is_multiple_of(3) {
            let omega_3_1 = res.root(3);
            let omega_3_2 = omega_3_1 * omega_3_1;
            // Note: char F cannot be 2 and so division by 2 works, because primitive roots of unity with even order exist.
            res.half_omega_3_1_min_2 = (omega_3_1 - omega_3_2) / F::from(2u64);
            res.half_omega_3_1_plus_2 = (omega_3_1 + omega_3_2) / F::from(2u64);
        }
        if order.is_multiple_of(4) {
            res.omega_4_1 = res.root(4);
        }
        if order.is_multiple_of(8) {
            res.omega_8_1 = res.root(8);
            res.omega_8_3 = res.omega_8_1.pow([3]);
        }
        if order.is_multiple_of(16) {
            res.omega_16_1 = res.root(16);
            res.omega_16_3 = res.omega_16_1.pow([3]);
            res.omega_16_9 = res.omega_16_1.pow([9]);
        }
        let small_roots = res.small_roots();
        res.base_small_roots = small_roots
            .into_iter()
            .map(prime_coordinate)
            .collect::<Option<Vec<_>>>()
            .and_then(|roots| roots.try_into().ok());
        res
    }

    pub fn ntt(&self, values: &mut [F]) {
        self.ntt_batch(values, values.len());
    }

    pub fn ntt_batch(&self, values: &mut [F], size: usize) {
        let kernel = (!generic_ntt_forced()).then(batch::get::<F>).flatten();
        self.ntt_batch_with_kernel(values, size, kernel.as_deref());
    }

    fn ntt_batch_with_kernel(
        &self,
        values: &mut [F],
        size: usize,
        kernel: Option<&dyn BatchArithmetic<F>>,
    ) {
        assert!(values.len().is_multiple_of(size));
        if size <= 1 {
            return;
        }
        if kernel.is_some() {
            let _ = self.batch_root_tables();
        }
        // Prepare the small shared plan before splitting one transform into
        // many radix-181 worker tasks. This reduces duplicate cold builds;
        // correctness does not depend on this prefetch or cache warmth.
        if size.is_multiple_of(RADER_PRIME) {
            let _ = self.rader181_plan();
        }
        if !generic_ntt_forced() {
            if let Some(small_roots) = &self.base_small_roots {
                if let Some(roots) = self.base_roots_table(size) {
                    self.ntt_dispatch(
                        values,
                        &roots,
                        size,
                        small_roots,
                        |value, root| value.mul_by_base_prime_field(&root),
                        BatchContext {
                            kernel,
                            full_roots: None,
                        },
                    );
                    return;
                }
            }
        }
        self.ntt_batch_generic_with_kernel(values, size, kernel);
    }

    fn batch_root_tables(&self) -> &[Vec<F>; 7] {
        if let Some(tables) = self.batch_small_roots.get() {
            return tables;
        }
        // Tiny, scalar construction outside OnceLock's initialization lock.
        // Never execute a nested NTT/Rayon task while publishing shared state.
        let candidate = BATCH_RADICES.map(|size| {
            if !self.order.is_multiple_of(size) {
                return Vec::new();
            }
            let root = self.root(size);
            let mut value = F::ONE;
            (0..size)
                .map(|_| {
                    let out = value;
                    value *= root;
                    out
                })
                .collect()
        });
        let _ = self.batch_small_roots.set(candidate);
        self.batch_small_roots
            .get()
            .expect("small root tables published")
    }

    fn small_roots(&self) -> [F; 8] {
        [
            self.half_omega_3_1_plus_2,
            self.half_omega_3_1_min_2,
            self.omega_4_1,
            self.omega_8_1,
            self.omega_8_3,
            self.omega_16_1,
            self.omega_16_3,
            self.omega_16_9,
        ]
    }

    // Retain the original arithmetic as the true-extension fallback and a
    // differential-test entry point. Input values always remain elements of F.
    fn ntt_batch_generic(&self, values: &mut [F], size: usize) {
        self.ntt_batch_generic_with_kernel(values, size, None);
    }

    fn ntt_batch_generic_with_kernel(
        &self,
        values: &mut [F],
        size: usize,
        kernel: Option<&dyn BatchArithmetic<F>>,
    ) {
        if size <= 1 {
            return;
        }
        let roots = self.roots_table(size);
        self.ntt_dispatch(
            values,
            &roots,
            size,
            &self.small_roots(),
            |value, root| value * root,
            BatchContext {
                kernel,
                full_roots: Some(&roots),
            },
        );
    }

    fn rader181_plan(&self) -> Option<&Rader181<F>> {
        #[cfg(test)]
        if self.force_dense_181 {
            return None;
        }
        if let Some(plan) = self.rader181.get() {
            return plan.as_deref();
        }
        // Never invoke the nested convolution NTT in get_or_init's closure:
        // work-stealing may re-enter another caller waiting for the same cell.
        let candidate = self.build_rader181_plan();
        // set only publishes an already-complete value; no Rayon work occurs
        // under its short internal initialization lock. A losing candidate is
        // dropped here, outside that lock.
        let _ = self.rader181.set(candidate);
        self.rader181
            .get()
            .expect("Rader plan was published")
            .as_deref()
    }

    fn build_rader181_plan(&self) -> Option<Box<Rader181<F>>> {
        if !self.order.is_multiple_of(RADER_PRIME) || !self.order.is_multiple_of(RADER_LENGTH) {
            return None;
        }
        let root180 = self.root(RADER_LENGTH);
        // Restrict this optimization to the intended inexpensive convolution
        // domain. The dense kernel handles all other fields.
        prime_coordinate(root180)?;
        let convolution = Self::new_with_supported_primes(RADER_LENGTH, root180, &[2, 3, 5]);

        // 2 is a generator of (Z/181Z)*. Gather a_j = x_{2^j};
        // b_j = omega^(2^-j); scatter c_m + x_0 to index 2^-m.
        // Then (a*b)_m = sum_j x_{2^j} omega^(2^(j-m)).
        let mut input_indices = [0; RADER_LENGTH];
        let mut index = 1;
        for slot in &mut input_indices {
            *slot = index;
            index = index * 2 % RADER_PRIME;
        }
        let output_indices =
            std::array::from_fn(|i| input_indices[(RADER_LENGTH - i) % RADER_LENGTH]);
        let omega = self.root(RADER_PRIME);
        let mut omega_powers = [F::ONE; RADER_PRIME];
        for i in 1..RADER_PRIME {
            omega_powers[i] = omega_powers[i - 1] * omega;
        }
        let mut normalized_kernel_spectrum = output_indices
            .iter()
            .map(|&i| omega_powers[i])
            .collect::<Vec<_>>();
        // This fixed plan is also built by the scalar reference path. Do not
        // re-resolve the global batch registry here: an inherited `None` must
        // remain hook-free even for a cold plan. The ordinary prime-field
        // twiddle path is still available in this 180-point transform.
        convolution.ntt_batch_with_kernel(
            &mut normalized_kernel_spectrum,
            RADER_LENGTH,
            None,
        );
        // intt is deliberately unnormalized. Apply 1/180 once to the cached
        // kernel, not to every row after its inverse transform.
        let inverse_length = F::from(RADER_LENGTH as u64)
            .inverse()
            .expect("a primitive 180th root requires invertible 180");
        for value in &mut normalized_kernel_spectrum {
            *value *= inverse_length;
        }
        Some(Box::new(Rader181 {
            convolution,
            input_indices,
            output_indices,
            normalized_kernel_spectrum,
        }))
    }

    fn ntt_rader181(values: &mut [F], plan: &Rader181<F>, kernel: Option<&dyn BatchArithmetic<F>>) {
        debug_assert_eq!(values.len() % RADER_PRIME, 0);
        let rows_per_batch = (values.len() / RADER_PRIME).min(RADER_BATCH_ROWS);
        let mut scratch = vec![F::ZERO; rows_per_batch * RADER_LENGTH];
        for batch in values.chunks_mut(RADER_BATCH_ROWS * RADER_PRIME) {
            let scratch = &mut scratch[..batch.len() / RADER_PRIME * RADER_LENGTH];
            for (row, gathered) in batch
                .chunks_exact(RADER_PRIME)
                .zip(scratch.chunks_exact_mut(RADER_LENGTH))
            {
                for (value, &index) in gathered.iter_mut().zip(&plan.input_indices) {
                    *value = row[index];
                }
            }
            plan.convolution
                .ntt_batch_with_kernel(scratch, RADER_LENGTH, kernel);
            for row in scratch.chunks_exact_mut(RADER_LENGTH) {
                if let Some(kernel) = kernel {
                    kernel.mul_assign(row, &plan.normalized_kernel_spectrum);
                } else {
                    for (value, &coefficient) in
                        row.iter_mut().zip(&plan.normalized_kernel_spectrum)
                    {
                        *value *= coefficient;
                    }
                }
                // This is exactly intt_batch's index reversal. Do it in the
                // bounded local batch to avoid spawning nested Rayon work for
                // an individual 180-element reversal.
                row[1..].reverse();
            }
            plan.convolution
                .ntt_batch_with_kernel(scratch, RADER_LENGTH, kernel);
            for (row, convolved) in batch
                .chunks_exact_mut(RADER_PRIME)
                .zip(scratch.chunks_exact(RADER_LENGTH))
            {
                let zero = row[0];
                row[0] = row.iter().copied().sum();
                for (&value, &index) in convolved.iter().zip(&plan.output_indices) {
                    row[index] = zero + value;
                }
            }
        }
    }

    /// Inverse NTT. Does not apply 1/n scaling factor.
    pub fn intt(&self, values: &mut [F]) {
        if values.len() <= 1 {
            return;
        }
        values[1..].reverse();
        self.ntt(values);
    }

    /// Inverse batch NTT. Does not apply 1/n scaling factor.
    pub fn intt_batch(&self, values: &mut [F], size: usize) {
        assert!(values.len().is_multiple_of(size));
        if size <= 1 {
            return;
        }

        #[cfg(not(feature = "parallel"))]
        values.chunks_exact_mut(size).for_each(|values| {
            values[1..].reverse();
        });

        #[cfg(feature = "parallel")]
        values.par_chunks_exact_mut(size).for_each(|values| {
            values[1..].reverse();
        });

        self.ntt_batch(values, size);
    }

    pub fn checked_root(&self, order: usize) -> Option<F> {
        if order == 0 {
            return Some(F::ONE);
        }
        self.order
            .is_multiple_of(order)
            .then(|| self.omega_order.pow([(self.order / order) as u64]))
    }

    pub fn root(&self, order: usize) -> F {
        self.checked_root(order)
            .expect("Subgroup of requested order does not exist.")
    }

    /// Returns a cached table of roots of unity of the given order.
    fn roots_table(&self, order: usize) -> Arc<Vec<F>> {
        assert!(
            self.order.is_multiple_of(order),
            "No subgroup of order {order}."
        );

        {
            let roots = self.roots.read().unwrap();
            if !roots.is_empty() && roots.len().is_multiple_of(order) {
                return Arc::clone(&roots);
            }
        }
        // No shared cache lock is held during potentially nested Rayon work.
        // Concurrent misses may temporarily duplicate requested-order tables;
        // do not introduce a blocking single-flight gate in this call path.
        let candidate = Arc::new(build_root_powers(self.root(order), order));
        let mut roots = self.roots.write().unwrap();
        if !roots.is_empty() && roots.len().is_multiple_of(order) {
            return Arc::clone(&roots);
        }
        // Retain one requested order, never the LCM of historical requests.
        // Active transforms keep their immutable Arc snapshot. Release the
        // lock before potentially freeing a large replaced table.
        let previous = std::mem::replace(&mut *roots, Arc::clone(&candidate));
        drop(roots);
        drop(previous);
        candidate
    }

    fn base_roots_table(&self, order: usize) -> Option<Arc<Vec<F::BasePrimeField>>> {
        assert!(self.order.is_multiple_of(order), "Unsupported root order");
        {
            let roots = self.base_roots.read().unwrap();
            if !roots.is_empty() && roots.len().is_multiple_of(order) {
                return Some(Arc::clone(&roots));
            }
        }
        // Test the requested root, not the engine's largest-order generator.
        // An incompatible subgroup returns before allocating its root table.
        let root = prime_coordinate(self.root(order))?;
        let candidate = Arc::new(build_root_powers(root, order));
        let mut roots = self.base_roots.write().unwrap();
        if !roots.is_empty() && roots.len().is_multiple_of(order) {
            return Some(Arc::clone(&roots));
        }
        let previous = std::mem::replace(&mut *roots, Arc::clone(&candidate));
        drop(roots);
        drop(previous);
        Some(candidate)
    }

    /// Compute NTTs in place by splititng into two factors.
    /// Recurses using the sqrt(N) Cooley-Tukey Six step NTT algorithm.
    fn ntt_recurse<T: Field, M: Fn(F, T) -> F + Copy + Send + Sync>(
        &self,
        values: &mut [F],
        roots: &[T],
        size: usize,
        small_roots: &[T; 8],
        multiply: M,
        batch: BatchContext<'_, F>,
    ) {
        debug_assert_eq!(values.len() % size, 0);
        let n1 = sqrt_factor_from_divisors(size, &self.divisors);
        assert!(n1 > 1, "prime transform requires a supported radix kernel");
        let n2 = size / n1;

        transpose(values, n1, n2);
        self.ntt_dispatch(values, roots, n1, small_roots, multiply, batch);
        transpose(values, n2, n1);
        // TODO: When (n1, n2) are coprime we can use the
        // Good-Thomas NTT algorithm and avoid the twiddle loop.
        if !batch
            .kernel
            .zip(batch.full_roots)
            .is_some_and(|(kernel, full_roots)| kernel.twiddles(values, full_roots, n1, n2))
        {
            apply_twiddles_mixed(values, roots, n1, n2, multiply);
        }
        self.ntt_dispatch(values, roots, n2, small_roots, multiply, batch);
        transpose(values, n1, n2);
    }

    fn ntt_dispatch<T: Field, M: Fn(F, T) -> F + Copy + Send + Sync>(
        &self,
        values: &mut [F],
        roots: &[T],
        size: usize,
        small_roots: &[T; 8],
        multiply: M,
        batch: BatchContext<'_, F>,
    ) {
        debug_assert_eq!(values.len() % size, 0);
        debug_assert_eq!(roots.len() % size, 0);
        #[cfg(feature = "parallel")]
        if values.len() > workload_size::<F>() && values.len() != size {
            // Multiple NTTs, compute in parallel.
            // Work size is largest multiple of `size` smaller than `WORKLOAD_SIZE`.
            let workload_size = size * max(1, workload_size::<F>() / size);
            return values.par_chunks_mut(workload_size).for_each(|values| {
                self.ntt_dispatch(values, roots, size, small_roots, multiply, batch);
            });
        }
        if let Some(kernel) = batch.kernel {
            if let Some(index) = BATCH_RADICES.iter().position(|&radix| radix == size) {
                if kernel.small_ntt(
                    values,
                    size,
                    &self.batch_root_tables()[index],
                    &self.small_roots(),
                ) {
                    return;
                }
            }
        }
        match size {
            0 | 1 => {}
            2 => {
                for v in values.chunks_exact_mut(2) {
                    (v[0], v[1]) = (v[0] + v[1], v[0] - v[1]);
                }
            }
            3 => {
                for v in values.chunks_exact_mut(3) {
                    // Rader NTT to reduce 3 to 2.
                    let v0 = v[0];
                    (v[1], v[2]) = (v[1] + v[2], v[1] - v[2]);
                    v[0] += v[1];
                    v[1] = multiply(v[1], small_roots[0]); // ½(ω₃ + ω₃²)
                    v[2] = multiply(v[2], small_roots[1]); // ½(ω₃ - ω₃²)
                    v[1] += v0;
                    (v[1], v[2]) = (v[1] + v[2], v[1] - v[2]);
                }
            }
            4 => {
                for v in values.chunks_exact_mut(4) {
                    (v[0], v[2]) = (v[0] + v[2], v[0] - v[2]);
                    (v[1], v[3]) = (v[1] + v[3], v[1] - v[3]);
                    v[3] = multiply(v[3], small_roots[2]);
                    (v[0], v[1]) = (v[0] + v[1], v[0] - v[1]);
                    (v[2], v[3]) = (v[2] + v[3], v[2] - v[3]);
                    (v[1], v[2]) = (v[2], v[1]);
                }
            }
            8 => {
                for v in values.chunks_exact_mut(8) {
                    // Cooley-Tukey with v as 2x4 matrix.
                    (v[0], v[4]) = (v[0] + v[4], v[0] - v[4]);
                    (v[1], v[5]) = (v[1] + v[5], v[1] - v[5]);
                    (v[2], v[6]) = (v[2] + v[6], v[2] - v[6]);
                    (v[3], v[7]) = (v[3] + v[7], v[3] - v[7]);
                    v[5] = multiply(v[5], small_roots[3]);
                    v[6] = multiply(v[6], small_roots[2]); // == omega_8_2
                    v[7] = multiply(v[7], small_roots[4]);
                    (v[0], v[2]) = (v[0] + v[2], v[0] - v[2]);
                    (v[1], v[3]) = (v[1] + v[3], v[1] - v[3]);
                    v[3] = multiply(v[3], small_roots[2]);
                    (v[0], v[1]) = (v[0] + v[1], v[0] - v[1]);
                    (v[2], v[3]) = (v[2] + v[3], v[2] - v[3]);
                    (v[4], v[6]) = (v[4] + v[6], v[4] - v[6]);
                    (v[5], v[7]) = (v[5] + v[7], v[5] - v[7]);
                    v[7] = multiply(v[7], small_roots[2]);
                    (v[4], v[5]) = (v[4] + v[5], v[4] - v[5]);
                    (v[6], v[7]) = (v[6] + v[7], v[6] - v[7]);
                    (v[1], v[4]) = (v[4], v[1]);
                    (v[3], v[6]) = (v[6], v[3]);
                }
            }
            16 => {
                for v in values.chunks_exact_mut(16) {
                    // Cooley-Tukey with v as 4x4 matrix.
                    for i in 0..4 {
                        let v = &mut v[i..];
                        (v[0], v[8]) = (v[0] + v[8], v[0] - v[8]);
                        (v[4], v[12]) = (v[4] + v[12], v[4] - v[12]);
                        v[12] = multiply(v[12], small_roots[2]);
                        (v[0], v[4]) = (v[0] + v[4], v[0] - v[4]);
                        (v[8], v[12]) = (v[8] + v[12], v[8] - v[12]);
                        (v[4], v[8]) = (v[8], v[4]);
                    }
                    v[5] = multiply(v[5], small_roots[5]);
                    v[6] = multiply(v[6], small_roots[3]);
                    v[7] = multiply(v[7], small_roots[6]);
                    v[9] = multiply(v[9], small_roots[3]);
                    v[10] = multiply(v[10], small_roots[2]);
                    v[11] = multiply(v[11], small_roots[4]);
                    v[13] = multiply(v[13], small_roots[6]);
                    v[14] = multiply(v[14], small_roots[4]);
                    v[15] = multiply(v[15], small_roots[7]);
                    for i in 0..4 {
                        let v = &mut v[i * 4..];
                        (v[0], v[2]) = (v[0] + v[2], v[0] - v[2]);
                        (v[1], v[3]) = (v[1] + v[3], v[1] - v[3]);
                        v[3] = multiply(v[3], small_roots[2]);
                        (v[0], v[1]) = (v[0] + v[1], v[0] - v[1]);
                        (v[2], v[3]) = (v[2] + v[3], v[2] - v[3]);
                        (v[1], v[2]) = (v[2], v[1]);
                    }
                    (v[1], v[4]) = (v[4], v[1]);
                    (v[2], v[8]) = (v[8], v[2]);
                    (v[3], v[12]) = (v[12], v[3]);
                    (v[6], v[9]) = (v[9], v[6]);
                    (v[7], v[13]) = (v[13], v[7]);
                    (v[11], v[14]) = (v[14], v[11]);
                }
            }
            5 | 17 | 181 => {
                if size == RADER_PRIME {
                    if let Some(plan) = self.rader181_plan() {
                        return Self::ntt_rader181(values, plan, batch.kernel);
                    }
                }
                // Dense prime DFT, with bounded scratch reused across the
                // batch. Keep 181 as the exact reference and as a fallback
                // when no suitable length-180 prime-field convolution exists.
                let mut output = vec![F::ZERO; size];
                let stride = roots.len() / size;
                for row in values.chunks_exact_mut(size) {
                    output[0] = row.iter().copied().sum();
                    for (frequency, result) in output.iter_mut().enumerate().skip(1) {
                        let step = frequency * stride;
                        let mut index = step;
                        let mut sum = row[0];
                        for &value in row.iter().skip(1) {
                            sum += multiply(value, roots[index]);
                            index += step;
                            if index >= roots.len() {
                                index -= roots.len();
                            }
                        }
                        *result = sum;
                    }
                    row.copy_from_slice(&output);
                }
            }
            size => self.ntt_recurse(values, roots, size, small_roots, multiply, batch),
        }
    }
}

impl<F: Field> ReedSolomon<F> for NttEngine<F> {
    fn next_order(&self, size: usize) -> Option<usize> {
        match self.divisors.binary_search(&size) {
            Ok(index) | Err(index) => self.divisors.get(index).copied(),
        }
    }

    fn generator(&self, codeword_length: usize) -> F {
        self.omega_order
            .pow([(self.order / codeword_length) as u64])
    }

    fn evaluation_points(
        &self,
        masked_message_length: usize,
        codeword_length: usize,
        indices: &[usize],
    ) -> Vec<F> {
        assert!(masked_message_length <= codeword_length);
        assert!(self.order.is_multiple_of(codeword_length));
        let mut result = Vec::new();
        let generator = self.generator(codeword_length);

        // Coset transformation
        let mut coset_size = self.next_order(masked_message_length).unwrap();
        while !codeword_length.is_multiple_of(coset_size) {
            coset_size = self.next_order(coset_size + 1).unwrap();
        }
        #[cfg(not(feature = "rs_in_order"))]
        let num_cosets = codeword_length / coset_size;

        for &index in indices {
            assert!(index < codeword_length);

            #[cfg(not(feature = "rs_in_order"))]
            let index = transpose_permute(index, num_cosets, coset_size);
            result.push(generator.pow([index as u64]));
        }
        result
    }

    #[cfg_attr(feature = "tracing", instrument(skip(self, messages, masks), fields(
        num_messages = messages.vectors.len() * messages.interleaving_depth,
        message_len = messages.message_length,
        codeword_length = codeword_length,
        mask_len = masks.len().checked_div(messages.vectors.len() * messages.interleaving_depth)

    )))]
    fn interleaved_encode(
        &self,
        messages: Messages<'_, F>,
        masks: &Buffer<F>,
        codeword_length: usize,
    ) -> Buffer<F> {
        let vectors = messages
            .vectors
            .iter()
            .map(|v| v.to_slice())
            .collect::<Vec<_>>();
        let messages = vectors
            .iter()
            .flat_map(|v| {
                chunks_exact_or_empty(v, messages.message_length, messages.interleaving_depth)
            })
            .collect::<Vec<_>>();
        assert!(self.order.is_multiple_of(codeword_length));
        if messages.is_empty() {
            assert!(masks.is_empty());
            return Buffer::from(Vec::new());
        }
        let num_messages = messages.len();
        let message_len = messages[0].len();
        assert!(messages.iter().all(|m| m.len() == message_len));
        assert!(masks.len().is_multiple_of(num_messages));
        let mask_length = masks.len() / num_messages;
        let masked_message_length = message_len + mask_length;
        assert!(masked_message_length <= codeword_length);

        // Coset-NTT: instead of doing one codeword-length NTT on mostly zeros,
        // do `num_cosets` many `coset_size`-point NTTs on twisted coefficient
        // vectors. For coset `c`, we evaluate on points
        //
        //     ω_N^{c + j * num_cosets} = ω_N^c · (ω_N^{num_cosets})^j
        //
        // so the coefficient of X^i must be multiplied by (ω_N^c)^i.
        //
        // You can also see this as applying a first round of Cooley-Tukey with
        // N = coset_size × num_cosets, and solving it directly by observing that
        // only the first coset is non-zero.
        let mut coset_size = self.next_order(masked_message_length).unwrap();
        while !codeword_length.is_multiple_of(coset_size) {
            coset_size = self.next_order(coset_size + 1).unwrap();
        }
        let num_cosets = codeword_length / coset_size;
        let coset_padding = coset_size - masked_message_length;

        // Lay out twisted coefficients in contiguous coset blocks of length
        // `coset_size`, zero-padding each block as needed.
        let mut result = Vec::with_capacity(num_messages * codeword_length);
        for (message, mask) in zip_strict(
            messages,
            chunks_exact_or_empty(masks.to_slice(), mask_length, num_messages),
        ) {
            // FFT[a 0 0 0] = [a a a a], so just replicate input in coset dimension.
            for _ in 0..num_cosets {
                result.extend_from_slice(message);
                result.extend_from_slice(mask);
                result.resize(result.len() + coset_padding, F::ZERO);
            }
        }
        assert_eq!(result.len(), num_messages * codeword_length);

        // NTT each coset block, then transpose each codeword block from
        // coset-major `(num_cosets × coset_size)` layout into standard codeword
        // order `(coset_size × num_cosets)`, where global index is
        // `c + j * num_cosets`.
        let kernel = (!generic_ntt_forced()).then(batch::get::<F>).flatten();
        if let Some(roots) = (!generic_ntt_forced())
            .then(|| self.base_roots_table(codeword_length))
            .flatten()
        {
            apply_twiddles_mixed(
                &mut result,
                roots.as_slice(),
                num_cosets,
                coset_size,
                |value, root| value.mul_by_base_prime_field(&root),
            );
        } else {
            let roots = self.roots_table(codeword_length);
            if !kernel
                .as_ref()
                .is_some_and(|kernel| kernel.twiddles(&mut result, &roots, num_cosets, coset_size))
            {
                apply_twiddles_mixed(
                    &mut result,
                    &roots,
                    num_cosets,
                    coset_size,
                    |value, root| value * root,
                );
            }
        }
        self.ntt_batch_with_kernel(&mut result, coset_size, kernel.as_deref());

        #[cfg(feature = "rs_in_order")]
        transpose(&mut result, num_cosets, coset_size);

        // Transpose to row-major order with vectors stacked horizontally.
        transpose(&mut result, num_messages, codeword_length);
        Buffer::from(result)
    }
}

/// Construct a complete powers table without any shared-cache lock held.
/// Small tables (including Rader's 180-point convolution roots) stay serial,
/// avoiding nested task overhead; larger tables retain parallel generation.
fn build_root_powers<T: Field>(root: T, size: usize) -> Vec<T> {
    let mut table = Vec::with_capacity(size);
    #[cfg(feature = "parallel")]
    if size > workload_size::<T>() {
        table.par_extend((0..size).into_par_iter().map_with(T::ZERO, |value, i| {
            if *value == T::ZERO {
                *value = root.pow([i as u64]);
            } else {
                *value *= root;
            }
            *value
        }));
        return table;
    }
    let mut value = T::ONE;
    for _ in 0..size {
        table.push(value);
        value *= root;
    }
    table
}

/// Applies twiddle factors to a slice of field elements in-place.
///
/// This is part of the six-step Cooley-Tukey NTT algorithm,
/// where after transposing and partially transforming a 2D matrix,
/// we multiply each non-zero row and column entry by a scalar "twiddle factor"
/// derived from powers of a root of unity.
///
/// Given:
/// - `values`: a flattened set of NTT matrices, each with shape `[rows × cols]`
/// - `roots`: the root-of-unity table (should have length divisible by `rows * cols`)
/// - `rows`, `cols`: the dimensions of each matrix
///
/// This function mutates `values` in-place, applying twiddle factors like so:
///
/// ```text
/// values[i][j] *= roots[(i * step + j * step) % roots.len()]
/// ```
///
/// More specifically:
/// - The first row and column are left untouched (twiddle factor is 1).
/// - For each row `i > 0`, each element `values[i][j > 0]` is multiplied by a twiddle factor.
/// - The factor is taken as `roots[index]`, where:
///   - `index` starts at `step = (i * roots.len()) / (rows * cols)`
///   - `index` increments by `step` for each column.
///
/// ### Parallelism
/// - If `parallel` is enabled and `values.len()` exceeds a threshold:
///   - Large matrices are split into workloads and processed in parallel.
///   - If a single matrix is present, its rows are parallelized directly.
///
/// ### Panics
/// - If `values.len() % (rows * cols) != 0`
/// - If `roots.len()` is not divisible by `rows * cols`
///
/// ### Example
/// Suppose you have a `2×4` matrix:
/// ```text
/// [ a0 a1 a2 a3 ]
/// [ b0 b1 b2 b3 ]
/// ```
/// and `roots = [r0, r1, ..., rN]`, then the transformed matrix becomes:
/// ```text
/// [ a0  a1       a2       a3       ]
/// [ b0  b1*rX1   b2*rX2   b3*rX3   ]
/// ```
/// where `rX1`, `rX2`, etc., are powers of root-of-unity determined by row/col indices.
pub fn apply_twiddles<F: Field>(values: &mut [F], roots: &[F], rows: usize, cols: usize) {
    if !generic_ntt_forced() {
        if batch::get::<F>().is_some_and(|kernel| kernel.twiddles(values, roots, rows, cols)) {
            return;
        }
        let base_roots: Option<Vec<_>> = roots.iter().copied().map(prime_coordinate).collect();
        if let Some(base_roots) = base_roots {
            return apply_twiddles_mixed(values, &base_roots, rows, cols, |value, root| {
                value.mul_by_base_prime_field(&root)
            });
        }
    }
    apply_twiddles_mixed(values, roots, rows, cols, |value, root| value * root);
}

fn prime_coordinate<F: Field>(value: F) -> Option<F::BasePrimeField> {
    let mut coefficients = value.to_base_prime_field_elements();
    let first = coefficients.next()?;
    coefficients
        .all(|coefficient| coefficient == F::BasePrimeField::ZERO)
        .then_some(first)
}

fn generic_ntt_forced() -> bool {
    #[cfg(test)]
    {
        FORCE_GENERIC_NTT.with(std::cell::Cell::get)
    }
    #[cfg(not(test))]
    {
        false
    }
}

#[cfg(test)]
thread_local! {
    // Only entry points consult this flag; recursive parallel kernels receive
    // their arithmetic statically. Other concurrently running tests are isolated.
    static FORCE_GENERIC_NTT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn apply_twiddles_mixed<F: Field, T: Field, M: Fn(F, T) -> F + Copy + Send + Sync>(
    values: &mut [F],
    roots: &[T],
    rows: usize,
    cols: usize,
    multiply: M,
) {
    let size = rows * cols;
    debug_assert_eq!(values.len() % size, 0);
    let step = roots.len() / size;

    #[cfg(feature = "parallel")]
    {
        if values.len() > workload_size::<F>() {
            if values.len() == size {
                // Only one matrix → parallelize rows directly
                values
                    .par_chunks_exact_mut(cols)
                    .enumerate()
                    .skip(1)
                    .for_each(|(i, row)| {
                        let step = (i * step) % roots.len();
                        let mut index = step;
                        for value in row.iter_mut().skip(1) {
                            index %= roots.len();
                            *value = multiply(*value, roots[index]);
                            index += step;
                        }
                    });
                return;
            }
            // Multiple matrices → chunk and recurse
            let workload_size = size * max(1, workload_size::<F>() / size);
            values
                .par_chunks_mut(workload_size)
                .for_each(|chunk| apply_twiddles_mixed(chunk, roots, rows, cols, multiply));
            return;
        }
    }

    // Fallback (non-parallel or small workload)
    for matrix in values.chunks_exact_mut(size) {
        for (i, row) in matrix.chunks_exact_mut(cols).enumerate().skip(1) {
            let step = (i * step) % roots.len();
            let mut index = step;
            for value in row.iter_mut().skip(1) {
                index %= roots.len();
                *value = multiply(*value, roots[index]);
                index += step;
            }
        }
    }
}

#[cfg(test)]
#[path = "mixed_radix_tests.rs"]
mod mixed_radix_tests;

#[cfg(test)]
#[path = "rader181_tests.rs"]
mod rader181_tests;

#[cfg(all(test, feature = "parallel"))]
#[path = "concurrency_tests.rs"]
mod concurrency_tests;

#[cfg(test)]
mod optional_batch_tests {
    use super::*;
    use ark_ff::{Fp2, Fp2Config, Fp64, MontBackend, MontConfig, MontFp};
    use std::sync::atomic::{AtomicUsize, Ordering};

    // Private types isolate global registry use from parallel tests.
    // These two tests deliberately share that private field and registration.
    static REGISTRY_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    #[derive(MontConfig)]
    #[modulus = "65521"]
    #[generator = "17"]
    struct BatchPrimeConfig;
    type P = Fp64<MontBackend<BatchPrimeConfig, 1>>;
    struct BatchQuadraticConfig;
    impl Fp2Config for BatchQuadraticConfig {
        type Fp = P;
        const NONRESIDUE: P = MontFp!("17");
        const FROBENIUS_COEFF_FP2_C1: &'static [P] = &[MontFp!("1"), MontFp!("65520")];
    }
    type E = Fp2<BatchQuadraticConfig>;

    #[derive(Debug, Default)]
    struct Recording {
        small: AtomicUsize,
        twiddle: AtomicUsize,
        pointwise: AtomicUsize,
    }
    impl BatchArithmetic<E> for Recording {
        fn dot(&self, a: &[E], b: &[E]) -> E {
            a.iter().zip(b).map(|(&a, &b)| a * b).sum()
        }
        fn sumcheck_pair(&self, a0: &[E], a1: &[E], b0: &[E], b1: &[E]) -> (E, E) {
            (
                self.dot(a0, b0),
                (0..a0.len())
                    .map(|i| (a1[i] - a0[i]) * (b1[i] - b0[i]))
                    .sum(),
            )
        }
        fn fold(&self, a: &mut [E], b: &[E], r: E) {
            for (a, &b) in a.iter_mut().zip(b) {
                *a += r * (b - *a);
            }
        }
        fn scale(&self, a: &mut [E], r: E) {
            for a in a {
                *a *= r;
            }
        }
        fn axpy(&self, a: &mut [E], r: E, b: &[E]) {
            for (a, &b) in a.iter_mut().zip(b) {
                *a += r * b;
            }
        }
        fn mul_assign(&self, a: &mut [E], b: &[E]) {
            self.pointwise.fetch_add(1, Ordering::Relaxed);
            for (a, &b) in a.iter_mut().zip(b) {
                *a *= b;
            }
        }
        fn small_ntt(&self, v: &mut [E], size: usize, roots: &[E], _: &[E; 8]) -> bool {
            self.small.fetch_add(1, Ordering::Relaxed);
            if size == 3 {
                return false;
            } // Exercise unchanged scalar fallback.
            assert_eq!(roots.len(), size);
            assert_eq!(roots[0], E::ONE);
            let mut out = vec![E::ZERO; size];
            for row in v.chunks_exact_mut(size) {
                for (j, out) in out.iter_mut().enumerate() {
                    *out = row
                        .iter()
                        .enumerate()
                        .map(|(i, &x)| x * roots[i * j % size])
                        .sum();
                }
                row.copy_from_slice(&out);
            }
            true
        }
        fn twiddles(&self, v: &mut [E], roots: &[E], rows: usize, cols: usize) -> bool {
            self.twiddle.fetch_add(1, Ordering::Relaxed);
            apply_twiddles_mixed(v, roots, rows, cols, |a, b| a * b);
            true
        }
    }
    fn engine() -> NttEngine<E> {
        let order = 32 * 9 * 5 * 181 * 181;
        let exponent = (65_521u64.pow(2) - 1) / order as u64;
        let root = (1..128u64)
            .find_map(|c| {
                let r = E::new(P::from(c), P::ONE).pow([exponent]);
                [2, 3, 5, 181]
                    .iter()
                    .all(|q| r.pow([(order / q) as u64]) != E::ONE)
                    .then_some(r)
            })
            .unwrap();
        NttEngine::new_with_supported_primes(order, root, &[2, 3, 5, 181])
    }
    fn samples(length: usize) -> Vec<E> {
        (0..length)
            .map(|i| {
                if i % 13 == 0 {
                    E::ZERO
                } else {
                    E::new(P::from((i * 31 + 9) as u64), P::from((i * i + 17) as u64))
                }
            })
            .collect()
    }
    #[test]
    fn optional_ntt_hooks_preserve_mixed_rader_inverse_and_reference() {
        let _registry_lock = REGISTRY_TEST_LOCK.lock().unwrap();
        let engine = engine();
        let kernel = Arc::new(Recording::default());
        batch::BATCH.insert::<E>(kernel.clone());
        for size in [2, 3, 4, 5, 8, 16, 90, 180, 181, 362, 543, 724] {
            let input = samples(size * 3);
            let mut actual = input.clone();
            let mut expected = input.clone();
            engine.ntt_batch_generic(&mut expected, size);
            engine.ntt_batch(&mut actual, size);
            assert_eq!(actual, expected, "size={size}");
            engine.intt_batch(&mut actual, size);
            let scaled: Vec<_> = input.iter().map(|&x| x * E::from(size as u64)).collect();
            assert_eq!(actual, scaled, "unnormalized inverse size={size}");
        }
        assert!(kernel.small.load(Ordering::Relaxed) > 0);
        assert!(kernel.twiddle.load(Ordering::Relaxed) > 0);
        assert!(kernel.pointwise.load(Ordering::Relaxed) > 0);
        let before = [
            kernel.small.load(Ordering::Relaxed),
            kernel.twiddle.load(Ordering::Relaxed),
            kernel.pointwise.load(Ordering::Relaxed),
        ];
        struct Reset(bool);
        impl Drop for Reset {
            fn drop(&mut self) {
                FORCE_GENERIC_NTT.with(|x| x.set(self.0));
            }
        }
        let _reset = Reset(FORCE_GENERIC_NTT.with(|x| x.replace(true)));
        let mut v = samples(181 * 4);
        engine.ntt_batch(&mut v, 181);
        assert_eq!(
            before,
            [
                kernel.small.load(Ordering::Relaxed),
                kernel.twiddle.load(Ordering::Relaxed),
                kernel.pointwise.load(Ordering::Relaxed)
            ]
        );
    }

    #[test]
    fn cold_rader_reference_plan_does_not_resolve_batch_hooks() {
        let _registry_lock = REGISTRY_TEST_LOCK.lock().unwrap();
        let kernel = Arc::new(Recording::default());
        batch::BATCH.insert::<E>(kernel.clone());

        struct Reset(bool);
        impl Drop for Reset {
            fn drop(&mut self) {
                FORCE_GENERIC_NTT.with(|flag| flag.set(self.0));
            }
        }
        for forced_entry in [false, true] {
            let cold_engine = engine();
            assert!(cold_engine.rader181.get().is_none());
            let input = samples(RADER_PRIME * 4);
            let mut actual = input.clone();
            if forced_entry {
                let _reset = Reset(FORCE_GENERIC_NTT.with(|flag| flag.replace(true)));
                cold_engine.ntt_batch(&mut actual, RADER_PRIME);
            } else {
                cold_engine.ntt_batch_generic(&mut actual, RADER_PRIME);
            }

            let plan = cold_engine.rader181.get().unwrap().as_ref().unwrap();
            assert!(plan.convolution.batch_small_roots.get().is_none());
            assert_eq!(kernel.small.load(Ordering::Relaxed), 0);
            assert_eq!(kernel.twiddle.load(Ordering::Relaxed), 0);
            assert_eq!(kernel.pointwise.load(Ordering::Relaxed), 0);

            let mut dense_engine = engine();
            dense_engine.force_dense_181 = true;
            let mut expected = input;
            dense_engine.ntt_batch_generic(&mut expected, RADER_PRIME);
            assert_eq!(actual, expected, "forced entry={forced_entry}");
        }
    }
}

#[cfg(test)]
mod base_twiddle_tests {
    use super::*;
    use crate::{
        algebra::{
            embedding::Identity,
            fields::{Field64, Field64_2},
            linear_form::{Evaluate, LinearForm, MultilinearExtension},
        },
        hash,
        parameters::ProtocolParameters,
        protocols::{params::DecodingRegime, whir::Config},
        transcript::{codecs::Empty, DomainSeparator, Proof, ProverState, VerifierState},
    };
    use ark_ff::{Fp2, Fp2Config, Fp64, MontBackend, MontConfig, MontFp};
    use ark_std::rand::{rngs::StdRng, Rng, SeedableRng};

    type E = Field64_2;

    fn with_generic<T>(run: impl FnOnce() -> T) -> T {
        struct Reset(bool);
        impl Drop for Reset {
            fn drop(&mut self) {
                FORCE_GENERIC_NTT.with(|flag| flag.set(self.0));
            }
        }
        let _reset = Reset(FORCE_GENERIC_NTT.with(|flag| flag.replace(true)));
        run()
    }

    fn samples(length: usize, seed: u64) -> Vec<E> {
        let mut rng = StdRng::seed_from_u64(seed);
        (0..length).map(|_| rng.gen()).collect()
    }

    fn mixed_order_engine() -> NttEngine<E> {
        let order = 3 << 16;
        let root = NttEngine::<Field64>::new_from_fftfield().root(order);
        NttEngine::new(order, E::new(root, Field64::ZERO))
    }

    #[test]
    fn base_twiddle_ntt_intt_matches_generic_for_full_extension_inputs() {
        let engine = mixed_order_engine();
        for size in [1, 2, 3, 4, 8, 16, 32, 48, 64, 512, 2048] {
            for zero in [false, true] {
                let input = if zero {
                    vec![E::ZERO; size * 3]
                } else {
                    samples(size * 3, size as u64)
                };
                let mut actual = input.clone();
                let mut expected = input.clone();
                engine.ntt_batch(&mut actual, size);
                engine.ntt_batch_generic(&mut expected, size);
                assert_eq!(actual, expected, "NTT size={size}");
                engine.intt_batch(&mut actual, size);
                with_generic(|| engine.intt_batch(&mut expected, size));
                assert_eq!(actual, expected, "INTT size={size}");
                assert_eq!(
                    actual,
                    input
                        .into_iter()
                        .map(|x| x * E::from(size as u64))
                        .collect::<Vec<_>>()
                );
            }
        }
    }

    #[test]
    fn requested_base_subgroups_do_not_depend_on_extension_engine_generator() {
        // Ark 0.5 exposes the base field's FFT parameters for Fp2, even though
        // Gold2 mathematically admits a larger two-power subgroup.
        let default_engine = NttEngine::<E>::new_from_fftfield();
        assert_eq!(E::TWO_ADICITY, 32);
        assert_eq!(default_engine.omega_order.c1, Field64::ZERO);
        assert_eq!(
            prime_coordinate(default_engine.omega_order),
            Some(default_engine.omega_order.c0)
        );

        // In Gold2, u^2 = 7 and 7 generates Gold*. Removing the odd factor
        // 2^32 - 1 from u's order 2(p - 1) gives a genuine order-2^33 root.
        // Check its order explicitly rather than relying on FftField metadata.
        let order = 1usize << 33;
        let root = E::new(Field64::ZERO, Field64::ONE).pow([u64::from(u32::MAX)]);
        assert_eq!(root.pow([order as u64]), E::ONE);
        assert_eq!(root.pow([(order / 2) as u64]), -E::ONE);
        let engine = NttEngine::<E>::new(order, root);
        assert_ne!(engine.omega_order.c1, Field64::ZERO);
        assert!(prime_coordinate(engine.omega_order).is_none());
        // This must return without trying to allocate a 2^33-entry root table.
        assert!(engine.base_roots_table(engine.order).is_none());
        for size in [16, 1024, 64, 4096, 128] {
            let mut values = samples(size, size as u64 + 29);
            let mut expected = values.clone();
            engine.ntt_batch(&mut values, size);
            engine.ntt_batch_generic(&mut expected, size);
            assert_eq!(values, expected);
            let cache = engine.base_roots_table(size).unwrap();
            assert!(cache.len().is_multiple_of(size));
            let stride = cache.len() / size;
            assert_eq!(E::new(cache[stride], Field64::ZERO), engine.root(size));
        }
    }

    #[test]
    fn base_twiddle_cache_growth_is_safe_under_concurrent_reuse() {
        let engine = NttEngine::<E>::new_from_fftfield();
        std::thread::scope(|scope| {
            for size in [32, 512, 128, 4096, 1024, 64] {
                let engine = &engine;
                scope.spawn(move || {
                    for repeat in 0..3 {
                        let mut actual = samples(size, (size + repeat) as u64);
                        let mut expected = actual.clone();
                        engine.ntt_batch(&mut actual, size);
                        engine.ntt_batch_generic(&mut expected, size);
                        assert_eq!(actual, expected);
                    }
                });
            }
        });
    }

    #[test]
    fn base_twiddle_interleaved_encoding_matches_generic_bytes() {
        let engine = mixed_order_engine();
        for (message_length, interleaving, codeword_length, mask_length) in [
            (1, 1, 2, 0),
            (3, 2, 8, 1),
            (16, 4, 32, 0),
            (32, 16, 128, 0),
            (128, 4, 512, 3),
        ] {
            for zero in [false, true] {
                let length = message_length * interleaving;
                let first = Buffer::from(if zero {
                    vec![E::ZERO; length]
                } else {
                    samples(length, 101)
                });
                let second = Buffer::from(samples(length, 213));
                let refs = [&first, &second];
                let messages = Messages::new(&refs, message_length, interleaving);
                let masks = Buffer::from(samples(mask_length * interleaving * refs.len(), 313));
                let actual = engine.interleaved_encode(messages, &masks, codeword_length);
                let expected =
                    with_generic(|| engine.interleaved_encode(messages, &masks, codeword_length));
                assert_eq!(actual, expected);
            }
        }
    }

    #[derive(MontConfig)]
    #[modulus = "17"]
    #[generator = "3"]
    struct F17Config;
    type F17 = Fp64<MontBackend<F17Config, 1>>;
    struct E17Config;
    impl Fp2Config for E17Config {
        type Fp = F17;
        const NONRESIDUE: F17 = MontFp!("3");
        const FROBENIUS_COEFF_FP2_C1: &'static [F17] = &[MontFp!("1"), MontFp!("16")];
    }
    type E17 = Fp2<E17Config>;

    #[test]
    fn actual_extension_roots_keep_generic_ntt_and_twiddle_fallback() {
        // u^2=3, with 3 of order 16 in F17, gives u order 32 in E17.
        // This exercises a genuinely extension-valued NTT without huge tables.
        let engine = NttEngine::<E17>::new(32, E17::new(F17::ZERO, F17::ONE));
        assert!(engine.base_roots_table(32).is_none());
        let input = (0..32)
            .map(|i| E17::new(F17::from(i as u64), F17::from((3 * i + 1) as u64)))
            .collect::<Vec<_>>();
        let mut actual = input.clone();
        engine.ntt(&mut actual);
        let expected = (0..32)
            .map(|i| {
                let root = engine.root(32).pow([i as u64]);
                input
                    .iter()
                    .enumerate()
                    .map(|(j, value)| *value * root.pow([j as u64]))
                    .sum::<E17>()
            })
            .collect::<Vec<_>>();
        assert_eq!(actual, expected);
        let mut smaller = input[..8].to_vec();
        let mut reference = smaller.clone();
        engine.ntt(&mut smaller);
        engine.ntt_batch_generic(&mut reference, 8);
        assert_eq!(smaller, reference);
        assert!(engine.base_roots_table(8).is_some());
        let roots = (0..32)
            .map(|i| engine.root(32).pow([i]))
            .collect::<Vec<_>>();
        let mut actual = input.clone();
        let mut reference = input;
        apply_twiddles(&mut actual, &roots, 4, 8);
        apply_twiddles_mixed(&mut reference, &roots, 4, 8, |a, b| a * b);
        assert_eq!(actual, reference);
    }

    fn complete_whir_proof() -> Proof {
        let config = Config::<Identity<E>>::new(
            1024,
            &ProtocolParameters {
                decoding_regime: DecodingRegime::Unique,
                starting_log_inv_rate: 1,
                initial_folding_factor: 2,
                folding_factor: 2,
                security_level: 32,
                pow_bits: 0,
                batch_size: 1,
                hash_id: hash::BLAKE3,
            },
        );
        let table = samples(1024, 719);
        let forms = [
            MultilinearExtension::new(samples(10, 29)),
            MultilinearExtension::new(samples(10, 79)),
        ];
        let values = forms
            .iter()
            .map(|l| l.evaluate(&Identity::new(), &table))
            .collect::<Vec<_>>();
        let ds = DomainSeparator::protocol(&config)
            .session(&"ntt-twiddle-numeric-regression")
            .instance(&Empty);
        let mut prover = ProverState::new_std(&ds);
        let table = Buffer::from(table);
        let witness = config.commit(&mut prover, &[&table]);
        let owned = forms
            .iter()
            .cloned()
            .map(|f| Box::new(f) as Box<dyn LinearForm<E>>)
            .collect();
        let _pending = config.prove(
            &mut prover,
            &[&table],
            vec![&witness],
            owned,
            Buffer::from(values.clone()),
        );
        let proof = prover.proof();
        let mut verifier = VerifierState::new_std(&ds, &proof);
        let commitment = config.receive_commitment(&mut verifier).unwrap();
        let claim = config
            .verify(&mut verifier, &[&commitment], &values)
            .unwrap();
        claim
            .verify(forms.iter().map(|f| f as &dyn LinearForm<E>))
            .unwrap();
        verifier.check_eof().unwrap();
        proof
    }

    #[test]
    fn full_whir_proof_bytes_match_generic_ntt_reference() {
        let reference = with_generic(complete_whir_proof);
        let actual = complete_whir_proof();
        assert_eq!(actual.narg_string, reference.narg_string);
        assert_eq!(actual.hints, reference.hints);
    }
}

#[cfg(test)]
#[allow(clippy::significant_drop_tightening)]
mod tests {
    use ark_ff::{AdditiveGroup as _, BigInteger, PrimeField};

    use super::*;
    use crate::algebra::fields::Field64;

    #[test]
    fn test_new_from_fftfield_basic() {
        // Ensure that an engine is created correctly from FFT field properties
        let engine = NttEngine::<Field64>::new_from_fftfield();

        // Verify that the order of the engine is correctly set
        assert_eq!(engine.order, 3 << 32);

        // Verify that the root of unity is correctly initialized
        assert_eq!(
            engine.omega_order,
            Field64::GENERATOR.pow([(18_446_744_069_414_584_320 / engine.order) as u64])
        );
    }

    #[test]
    fn test_root_computation() {
        let engine = NttEngine::<Field64>::new_from_fftfield();

        // Ensure the root exponentiates correctly
        assert_eq!(engine.root(8).pow([8]), Field64::ONE);
        assert_eq!(engine.root(4).pow([4]), Field64::ONE);
        assert_eq!(engine.root(2).pow([2]), Field64::ONE);

        // Ensure it's not a lower-order root
        assert_ne!(engine.root(8).pow([4]), Field64::ONE);
        assert_ne!(engine.root(4).pow([2]), Field64::ONE);
    }

    #[test]
    fn test_root_of_unity_multiplication() {
        let engine = NttEngine::<Field64>::new_from_fftfield();

        let root = engine.root(16);

        // Multiply root by itself repeatedly and verify expected outcomes
        assert_eq!(root.pow([2]), engine.root(8));
        assert_eq!(root.pow([4]), engine.root(4));
        assert_eq!(root.pow([8]), engine.root(2));
    }

    #[test]
    fn test_root_of_unity_inversion() {
        let engine = NttEngine::<Field64>::new_from_fftfield();
        let root = engine.root(16);

        // The inverse of ω is ω^{-1}, computed as ω^(p-2) in Field64.
        let p: u64 = u64::from_be_bytes(Field64::MODULUS.to_bytes_be().try_into().unwrap());
        let inverse_root = root.pow([p - 2]);
        assert_eq!(root * inverse_root, Field64::ONE);
    }

    #[test]
    fn test_precomputed_small_roots() {
        let engine = NttEngine::<Field64>::new_from_fftfield();

        // Check that precomputed values are correctly initialized
        assert_eq!(engine.omega_8_1.pow([8]), Field64::ONE);
        assert_eq!(engine.omega_8_3.pow([8]), Field64::ONE);
        assert_eq!(engine.omega_16_1.pow([16]), Field64::ONE);
        assert_eq!(engine.omega_16_3.pow([16]), Field64::ONE);
        assert_eq!(engine.omega_16_9.pow([16]), Field64::ONE);
    }

    #[test]
    fn test_consistency_across_multiple_instances() {
        let engine1 = NttEngine::<Field64>::new_from_fftfield();
        let engine2 = NttEngine::<Field64>::new_from_fftfield();

        // Ensure that multiple instances yield the same results
        assert_eq!(engine1.root(8), engine2.root(8));
        assert_eq!(engine1.root(4), engine2.root(4));
        assert_eq!(engine1.root(2), engine2.root(2));
    }

    #[test]
    fn test_roots_table_basic() {
        let engine = NttEngine::<Field64>::new_from_fftfield();
        let roots_4 = engine.roots_table(4);

        // Check hardcoded expected values (ω^i)
        assert_eq!(roots_4[0], Field::ONE);
        assert_eq!(roots_4[1], engine.root(4));
        assert_eq!(roots_4[2], engine.root(4).pow([2]));
        assert_eq!(roots_4[3], engine.root(4).pow([3]));
    }

    #[test]
    fn test_roots_table_minimal_order() {
        let engine = NttEngine::<Field64>::new_from_fftfield();

        let roots_2 = engine.roots_table(2);

        // Must contain only ω^0 and ω^1
        assert_eq!(roots_2.len(), 2);
        assert_eq!(roots_2[0], Field64::ONE);
        assert_eq!(roots_2[1], engine.root(2));
    }

    #[test]
    fn test_roots_table_progression() {
        let engine = NttEngine::<Field64>::new_from_fftfield();

        let roots_4 = engine.roots_table(4);

        // Ensure the sequence follows expected powers of the root of unity
        for i in 0..4 {
            assert_eq!(roots_4[i], engine.root(4).pow([i as u64]));
        }
    }

    #[test]
    fn test_roots_table_cached_results() {
        let engine = NttEngine::<Field64>::new_from_fftfield();

        let first_access = engine.roots_table(4);
        let second_access = engine.roots_table(4);

        // The memory location should be the same, meaning it's cached
        assert!(std::ptr::eq(first_access.as_ptr(), second_access.as_ptr()));
    }

    #[test]
    fn test_roots_table_recompute_factor_order() {
        let engine = NttEngine::<Field64>::new_from_fftfield();

        let roots_4 = engine.roots_table(4);
        let roots_2 = engine.roots_table(2);

        // Ensure first two elements of roots_4 match the first two elements of roots_2
        assert_eq!(&roots_4[..2], &roots_2[..2]);
    }

    #[test]
    fn test_apply_twiddles_basic() {
        let engine = NttEngine::<Field64>::new_from_fftfield();

        let mut values = vec![
            Field64::from(1),
            Field64::from(2),
            Field64::from(3),
            Field64::from(4),
            Field64::from(5),
            Field64::from(6),
            Field64::from(7),
            Field64::from(8),
        ];

        // Mock roots
        let r1 = Field64::from(33);
        let roots = vec![r1];

        // Ensure the root of unity is correct
        assert_eq!(engine.root(4).pow([4]), Field64::ONE);

        apply_twiddles(&mut values, &roots, 2, 4);

        // The first row should remain unchanged
        assert_eq!(values[0], Field64::from(1));
        assert_eq!(values[1], Field64::from(2));
        assert_eq!(values[2], Field64::from(3));
        assert_eq!(values[3], Field64::from(4));

        // The second row should be multiplied by the correct twiddle factors
        assert_eq!(values[4], Field64::from(5)); // No change for first column
        assert_eq!(values[5], Field64::from(6) * r1);
        assert_eq!(values[6], Field64::from(7) * r1);
        assert_eq!(values[7], Field64::from(8) * r1);
    }

    #[test]
    fn test_apply_twiddles_single_row() {
        let mut values = vec![Field64::from(1), Field64::from(2)];

        // Mock roots
        let r1 = Field64::from(12);
        let roots = vec![r1];

        apply_twiddles(&mut values, &roots, 1, 2);

        // Everything should remain unchanged
        assert_eq!(values[0], Field64::from(1));
        assert_eq!(values[1], Field64::from(2));
    }

    #[test]
    fn test_apply_twiddles_varying_rows() {
        let mut values = vec![
            Field64::from(1),
            Field64::from(2),
            Field64::from(3),
            Field64::from(4),
            Field64::from(5),
            Field64::from(6),
            Field64::from(7),
            Field64::from(8),
            Field64::from(9),
        ];

        // Mock roots
        let roots = (2..100).map(Field64::from).collect::<Vec<_>>();

        apply_twiddles(&mut values, &roots, 3, 3);

        // First row remains unchanged
        assert_eq!(values[0], Field64::from(1));
        assert_eq!(values[1], Field64::from(2));
        assert_eq!(values[2], Field64::from(3));

        // Second row multiplied by twiddle factors
        assert_eq!(values[3], Field64::from(4));
        assert_eq!(values[4], Field64::from(5) * roots[10]);
        assert_eq!(values[5], Field64::from(6) * roots[20]);

        // Third row multiplied by twiddle factors
        assert_eq!(values[6], Field64::from(7));
        assert_eq!(values[7], Field64::from(8) * roots[20]);
        assert_eq!(values[8], Field64::from(9) * roots[40]);
    }

    #[test]
    fn test_apply_twiddles_large_table() {
        let rows = 320;
        let cols = 320;
        let size = rows * cols;

        let mut values: Vec<Field64> = (0..size as u64).map(Field64::from).collect();

        // Generate a large set of twiddle factors
        let roots: Vec<Field64> = (0..(size * 2) as u64).map(Field64::from).collect();

        apply_twiddles(&mut values, &roots, rows, cols);

        // Verify the first row remains unchanged
        for (i, &col) in values.iter().enumerate().take(cols) {
            assert_eq!(col, Field64::from(i as u64));
        }

        // Verify the first column remains unchanged
        for row in 1..rows {
            let index = row * cols;
            assert_eq!(
                values[index],
                Field64::from(index as u64),
                "Mismatch in first column at row={row}"
            );
        }

        // Verify that other rows have been modified using the twiddle factors
        for row in 1..rows {
            let mut idx = row * 2;
            for col in 1..cols {
                let index = row * cols + col;
                let expected = Field64::from(index as u64) * roots[idx];
                assert_eq!(values[index], expected, "Mismatch at row={row}, col={col}");
                idx += 2 * row;
            }
        }
    }

    #[test]
    fn test_ntt_batch_size_2() {
        let engine = NttEngine::<Field64>::new_from_fftfield();

        // Input values: f(x) = [1, 2]
        let f0 = Field64::from(1);
        let f1 = Field64::from(2);
        let mut values = vec![f0, f1];

        // Compute the expected NTT manually:
        //
        //   F(0)  =  f0 + f1
        //   F(1)  =  f0 - f1
        //
        // ω is the 2nd root of unity: ω² = 1.

        let expected_f0 = f0 + f1;
        let expected_f1 = f0 - f1;

        let expected_values = vec![expected_f0, expected_f1];

        engine.ntt_batch(&mut values, 2);

        assert_eq!(values, expected_values);
    }

    #[test]
    fn test_ntt_batch_size_4() {
        let engine = NttEngine::<Field64>::new_from_fftfield();

        // Input values: f(x) = [1, 2, 3, 4]
        let f0 = Field64::from(1);
        let f1 = Field64::from(2);
        let f2 = Field64::from(3);
        let f3 = Field64::from(4);
        let mut values = vec![f0, f1, f2, f3];

        // Compute the expected NTT manually:
        //
        //   F(0)  =  f0 + f1 + f2 + f3
        //   F(1)  =  f0 + f1 * ω + f2 * ω² + f3 * ω³
        //   F(2)  =  f0 + f1 * ω² + f2 * ω⁴ + f3 * ω⁶
        //   F(3)  =  f0 + f1 * ω³ + f2 * ω⁶ + f3 * ω⁹
        //
        // ω is the 4th root of unity: ω⁴ = 1, ω² = -1.

        let omega = engine.omega_4_1;
        let omega1 = omega; // ω
        let omega2 = omega * omega; // ω² = -1
        let omega3 = omega * omega2; // ω³ = -ω
        let omega4 = omega * omega3; // ω⁴ = 1

        let expected_f0 = f0 + f1 + f2 + f3;
        let expected_f1 = f0 + f1 * omega1 + f2 * omega2 + f3 * omega3;
        let expected_f2 = f0 + f1 * omega2 + f2 * omega4 + f3 * omega2;
        let expected_f3 = f0 + f1 * omega3 + f2 * omega2 + f3 * omega1;

        let expected_values = vec![expected_f0, expected_f1, expected_f2, expected_f3];

        engine.ntt_batch(&mut values, 4);

        assert_eq!(values, expected_values);
    }

    #[test]
    fn test_ntt_batch_size_8() {
        let engine = NttEngine::<Field64>::new_from_fftfield();

        // Input values: f(x) = [1, 2, 3, 4, 5, 6, 7, 8]
        let f0 = Field64::from(1);
        let f1 = Field64::from(2);
        let f2 = Field64::from(3);
        let f3 = Field64::from(4);
        let f4 = Field64::from(5);
        let f5 = Field64::from(6);
        let f6 = Field64::from(7);
        let f7 = Field64::from(8);
        let mut values = vec![f0, f1, f2, f3, f4, f5, f6, f7];

        // Compute the expected NTT manually:
        //
        //   F(k) = ∑ f_j * ω^(j*k)  for k ∈ {0, ..., 7}
        //
        // ω is the 8th root of unity: ω⁸ = 1.

        let omega = engine.omega_8_1; // ω
        let omega1 = omega; // ω
        let omega2 = omega * omega; // ω²
        let omega3 = omega * omega2; // ω³
        let omega4 = omega * omega3; // ω⁴
        let omega5 = omega * omega4; // ω⁵
        let omega6 = omega * omega5; // ω⁶
        let omega7 = omega * omega6; // ω⁷

        let expected_f0 = f0 + f1 + f2 + f3 + f4 + f5 + f6 + f7;
        let expected_f1 = f0
            + f1 * omega1
            + f2 * omega2
            + f3 * omega3
            + f4 * omega4
            + f5 * omega5
            + f6 * omega6
            + f7 * omega7;
        let expected_f2 = f0
            + f1 * omega2
            + f2 * omega4
            + f3 * omega6
            + f4 * Field64::ONE
            + f5 * omega2
            + f6 * omega4
            + f7 * omega6;
        let expected_f3 = f0
            + f1 * omega3
            + f2 * omega6
            + f3 * omega1
            + f4 * omega4
            + f5 * omega7
            + f6 * omega2
            + f7 * omega5;
        let expected_f4 = f0
            + f1 * omega4
            + f2 * Field64::ONE
            + f3 * omega4
            + f4 * Field64::ONE
            + f5 * omega4
            + f6 * Field64::ONE
            + f7 * omega4;
        let expected_f5 = f0
            + f1 * omega5
            + f2 * omega2
            + f3 * omega7
            + f4 * omega4
            + f5 * omega1
            + f6 * omega6
            + f7 * omega3;
        let expected_f6 = f0
            + f1 * omega6
            + f2 * omega4
            + f3 * omega2
            + f4 * Field64::ONE
            + f5 * omega6
            + f6 * omega4
            + f7 * omega2;
        let expected_f7 = f0
            + f1 * omega7
            + f2 * omega6
            + f3 * omega5
            + f4 * omega4
            + f5 * omega3
            + f6 * omega2
            + f7 * omega1;

        let expected_values = vec![
            expected_f0,
            expected_f1,
            expected_f2,
            expected_f3,
            expected_f4,
            expected_f5,
            expected_f6,
            expected_f7,
        ];

        engine.ntt_batch(&mut values, 8);

        assert_eq!(values, expected_values);
    }

    #[test]
    fn test_ntt_batch_size_16() {
        let engine = NttEngine::<Field64>::new_from_fftfield();

        // Input values: f(x) = [1, 2, ..., 16]
        let values: Vec<_> = (1..=16).map(Field64::from).collect();
        let mut values_ntt = values.clone();

        // Compute the expected NTT manually:
        //
        //   F(k) = ∑ f_j * ω^(j*k)  for k ∈ {0, ..., 15}
        //
        // ω is the 16th root of unity: ω¹⁶ = 1.

        let omega = engine.omega_16_1; // ω
        let mut expected_values = vec![Field64::ZERO; 16];
        for (k, expected_value) in expected_values.iter_mut().enumerate().take(16) {
            let omega_k = omega.pow([k as u64]);
            *expected_value = values
                .iter()
                .enumerate()
                .map(|(j, &f_j)| f_j * omega_k.pow([j as u64]))
                .sum();
        }

        engine.ntt_batch(&mut values_ntt, 16);

        assert_eq!(values_ntt, expected_values);
    }

    #[test]
    fn test_ntt_batch_size_32() {
        let engine = NttEngine::<Field64>::new_from_fftfield();

        // Input values: f(x) = [1, 2, ..., 32]
        let values: Vec<_> = (1..=32).map(Field64::from).collect();
        let mut values_ntt = values.clone();

        // Compute the expected NTT manually:
        //
        //   F(k) = ∑ f_j * ω^(j*k)  for k ∈ {0, ..., 31}
        //
        // ω is the 32nd root of unity: ω³² = 1.

        let omega = engine.root(32);
        let mut expected_values = vec![Field64::ZERO; 32];
        for (k, expected_value) in expected_values.iter_mut().enumerate().take(32) {
            let omega_k = omega.pow([k as u64]);
            *expected_value = values
                .iter()
                .enumerate()
                .map(|(j, &f_j)| f_j * omega_k.pow([j as u64]))
                .sum();
        }

        engine.ntt_batch(&mut values_ntt, 32);

        assert_eq!(values_ntt, expected_values);
    }
}
