//! Optional, type-indexed CPU arithmetic kernels.
//!
//! Kernels operate on the existing scalar field representation. They must
//! implement exactly the same arithmetic as the scalar fallback; registration
//! does not change the field, embedding, transcript, or wire encoding. Resolve
//! an Arc once per operation and release the registry lock before doing work.

use std::{
    fmt::Debug,
    sync::{Arc, LazyLock},
};

use crate::type_map::{Family, TypeMap};

/// Same-field batch arithmetic, optionally implemented with genuine SIMD.
///
/// Paired slices have equal lengths. Empty slices are valid; dot products and
/// both sumcheck coefficients are then zero. `sumcheck_pair` returns
/// `(sum a0*b0, sum (a1-a0)*(b1-b0))`. `fold` writes
/// `low[i] += weight * (high[i] - low[i])`. No method may resize a slice or
/// change its element order. Implementations must handle scalar lane tails.
///
/// The trait itself has no `F: Field` bound so it can be a TypeMap family;
/// concrete implementations and all arithmetic callers enforce that bound.
pub trait BatchArithmetic<F>: Debug + Send + Sync {
    fn dot(&self, a: &[F], b: &[F]) -> F;
    fn sumcheck_pair(&self, a0: &[F], a1: &[F], b0: &[F], b1: &[F]) -> (F, F);
    fn fold(&self, low: &mut [F], high: &[F], weight: F);
    fn scale(&self, values: &mut [F], weight: F);
    fn axpy(&self, acc: &mut [F], weight: F, v: &[F]);
    fn mul_assign(&self, values: &mut [F], rhs: &[F]);

    /// Accumulate `sum_j coefficients[j] * points[j]^i` into `acc[i]`.
    /// Both coefficient/point slices have equal length. Returning false must
    /// leave acc unchanged, so the original mixed/scalar kernel can run.
    fn geometric_accumulate(&self, _acc: &mut [F], _coefficients: &[F], _points: &[F]) -> bool {
        false
    }

    /// Optional fixed-radix batch NTT using the original engine's powers and
    /// eight small-root constants. False must leave values unchanged.
    fn small_ntt(
        &self,
        _values: &mut [F],
        _size: usize,
        _roots: &[F],
        _small_roots: &[F; 8],
    ) -> bool {
        false
    }

    /// Optional row-major twiddle application with the original root powers.
    /// False must leave values unchanged, including when a shape is unsupported.
    fn twiddles(&self, _values: &mut [F], _roots: &[F], _rows: usize, _cols: usize) -> bool {
        false
    }
}

#[derive(Default)]
pub struct BatchFamily;

impl Family for BatchFamily {
    type Dyn<F: 'static> = dyn BatchArithmetic<F>;
}

/// Optional kernels keyed by the complete scalar field type, not storage size.
pub static BATCH: LazyLock<TypeMap<BatchFamily>> = LazyLock::new(TypeMap::new);

/// Obtain an immutable snapshot. No registry lock remains held on return.
pub fn get<F: 'static>() -> Option<Arc<dyn BatchArithmetic<F>>> {
    BATCH.get::<F>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algebra::{
        self,
        embedding::{Basefield, Compose, Embedding, Frobenius, Identity},
        sumcheck,
    };
    use ark_ff::{AdditiveGroup, Field, Fp2, Fp2Config, Fp64, MontBackend, MontConfig, MontFp};
    use std::sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Mutex,
    };

    // Private field types prevent registration from affecting any existing
    // Field64/Field64_2 test or application profile in another test thread.
    #[derive(MontConfig)]
    #[modulus = "97"]
    #[generator = "5"]
    struct BatchPrimeConfig;
    type P = Fp64<MontBackend<BatchPrimeConfig, 1>>;
    struct BatchQuadraticConfig;
    impl Fp2Config for BatchQuadraticConfig {
        type Fp = P;
        const NONRESIDUE: P = MontFp!("5");
        const FROBENIUS_COEFF_FP2_C1: &'static [P] = &[MontFp!("1"), MontFp!("96")];
    }
    type E = Fp2<BatchQuadraticConfig>;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[derive(Debug, Default)]
    struct CountingKernel {
        calls: AtomicUsize,
        dot_calls: AtomicUsize,
        remove_on_dot: AtomicBool,
        handle_geometric: AtomicBool,
    }

    impl CountingKernel {
        fn hit(&self) {
            self.calls.fetch_add(1, Ordering::Relaxed);
        }
        fn count(&self) -> usize {
            self.calls.load(Ordering::Relaxed)
        }
    }

    impl BatchArithmetic<E> for CountingKernel {
        fn dot(&self, a: &[E], b: &[E]) -> E {
            assert_eq!(a.len(), b.len());
            self.hit();
            self.dot_calls.fetch_add(1, Ordering::Relaxed);
            if self.remove_on_dot.swap(false, Ordering::Relaxed) {
                // Exercises lock-free callback execution and snapshot lifetime.
                BATCH.remove::<E>();
            }
            a.iter().zip(b).map(|(&a, &b)| a * b).sum()
        }
        fn sumcheck_pair(&self, a0: &[E], a1: &[E], b0: &[E], b1: &[E]) -> (E, E) {
            assert_eq!(a0.len(), a1.len());
            assert_eq!(a0.len(), b0.len());
            assert_eq!(a0.len(), b1.len());
            self.hit();
            a0.iter()
                .zip(a1)
                .zip(b0.iter().zip(b1))
                .fold((E::ZERO, E::ZERO), |(c0, c2), ((&a0, &a1), (&b0, &b1))| {
                    (c0 + a0 * b0, c2 + (a1 - a0) * (b1 - b0))
                })
        }
        fn fold(&self, low: &mut [E], high: &[E], weight: E) {
            assert_eq!(low.len(), high.len());
            self.hit();
            for (low, &high) in low.iter_mut().zip(high) {
                *low += (high - *low) * weight;
            }
        }
        fn scale(&self, values: &mut [E], weight: E) {
            self.hit();
            for value in values {
                *value *= weight;
            }
        }
        fn axpy(&self, acc: &mut [E], weight: E, v: &[E]) {
            assert_eq!(acc.len(), v.len());
            self.hit();
            for (acc, &v) in acc.iter_mut().zip(v) {
                *acc += weight * v;
            }
        }
        fn mul_assign(&self, values: &mut [E], rhs: &[E]) {
            assert_eq!(values.len(), rhs.len());
            self.hit();
            for (value, &rhs) in values.iter_mut().zip(rhs) {
                *value *= rhs;
            }
        }
        fn geometric_accumulate(&self, acc: &mut [E], coefficients: &[E], points: &[E]) -> bool {
            if !self.handle_geometric.load(Ordering::Relaxed) {
                return false;
            }
            assert_eq!(coefficients.len(), points.len());
            self.hit();
            for (i, acc) in acc.iter_mut().enumerate() {
                *acc += coefficients
                    .iter()
                    .zip(points)
                    .map(|(&c, &p)| c * p.pow([i as u64]))
                    .sum::<E>();
            }
            true
        }
    }

    struct Registration;
    impl Drop for Registration {
        fn drop(&mut self) {
            BATCH.remove::<E>();
        }
    }
    fn register() -> (Arc<CountingKernel>, Registration) {
        assert!(get::<E>().is_none());
        let kernel = Arc::new(CountingKernel::default());
        BATCH.insert::<E>(kernel.clone());
        (kernel, Registration)
    }
    fn values(n: usize, seed: u64) -> Vec<E> {
        (0..n)
            .map(|i| {
                E::new(
                    P::from(seed + i as u64 * 7),
                    P::from(seed + i as u64 * 11 + 1),
                )
            })
            .collect()
    }
    fn reference_sumcheck(a: &[E], b: &[E]) -> (E, E) {
        let len = a.len().min(b.len());
        if len == 0 {
            return (E::ZERO, E::ZERO);
        }
        if len == 1 {
            return (a[0] * b[0], E::ZERO);
        }
        let padded = len.next_power_of_two();
        let mut a = a[..len].to_vec();
        let mut b = b[..len].to_vec();
        a.resize(padded, E::ZERO);
        b.resize(padded, E::ZERO);
        let half = padded / 2;
        (0..half).fold((E::ZERO, E::ZERO), |(c0, c2), i| {
            (
                c0 + a[i] * b[i],
                c2 + (a[half + i] - a[i]) * (b[half + i] - b[i]),
            )
        })
    }
    fn reference_fold(values: &[E], weight: E) -> Vec<E> {
        if values.len() <= 1 {
            return values.to_vec();
        }
        let mut values = values.to_vec();
        values.resize(values.len().next_power_of_two(), E::ZERO);
        let half = values.len() / 2;
        (0..half)
            .map(|i| values[i] + weight * (values[half + i] - values[i]))
            .collect()
    }

    #[test]
    fn batch_dispatch_preserves_shapes_padding_and_fallback() {
        let _lock = TEST_LOCK.lock().unwrap();
        let a = values(33, 2);
        let b = values(33, 17);
        assert!(get::<E>().is_none());
        assert_eq!(
            algebra::dot(&a, &b),
            a.iter().zip(&b).map(|(&x, &y)| x * y).sum()
        );
        assert_eq!(
            sumcheck::compute_sumcheck_polynomial(&a, &b),
            reference_sumcheck(&a, &b)
        );
        let (kernel, _registration) = register();
        for n in [0, 1, 2, 3, 7, 8, 9, 31, 32, 33, 63, 64, 65, 129] {
            let a = values(n, 13);
            let b = values(n, 29);
            assert_eq!(
                algebra::dot(&a, &b),
                a.iter().zip(&b).map(|(&x, &y)| x * y).sum()
            );
            for other in [n, n.saturating_sub(1), n + 3] {
                let c = values(other, 31);
                assert_eq!(
                    sumcheck::compute_sumcheck_polynomial(&a, &c),
                    reference_sumcheck(&a, &c)
                );
            }
            for weight in [E::ZERO, E::ONE, -E::ONE, E::new(P::from(8), P::from(9))] {
                let mut folded = a.clone();
                sumcheck::fold(&mut folded, weight);
                assert_eq!(folded, reference_fold(&a, weight));
                let mut scaled = a.clone();
                algebra::scalar_mul(&mut scaled, weight);
                assert_eq!(scaled, a.iter().map(|&a| a * weight).collect::<Vec<_>>());
                let mut acc = a.clone();
                algebra::scalar_mul_add(&mut acc, weight, &b);
                assert_eq!(
                    acc,
                    a.iter()
                        .zip(&b)
                        .map(|(&a, &b)| a + weight * b)
                        .collect::<Vec<_>>()
                );
            }
            let mut product = a.clone();
            get::<E>().unwrap().mul_assign(&mut product, &b);
            assert_eq!(
                product,
                a.iter().zip(&b).map(|(&a, &b)| a * b).collect::<Vec<_>>()
            );
        }
        assert!(kernel.count() > 0);
        let count = kernel.count();
        assert!(std::panic::catch_unwind(|| algebra::dot(&a, &b[..32])).is_err());
        assert_eq!(
            kernel.count(),
            count,
            "invalid shape must fail before entering the hook"
        );
    }

    #[test]
    fn batch_dispatch_only_identity_not_frobenius_or_basefield() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (kernel, _registration) = register();
        let a = values(73, 2);
        let b = values(73, 19);
        let identity = Identity::<E>::new();
        let frobenius = Frobenius::<E>::new(1);
        assert_ne!(frobenius.map(b[0]), b[0]);
        let expected: E = a.iter().zip(&b).map(|(&a, &b)| a * b).sum();
        assert_eq!(algebra::mixed_dot(&identity, &a, &b), expected);
        let mut identity_acc = a.clone();
        algebra::mixed_scalar_mul_add(&identity, &mut identity_acc, E::ONE, &b);
        let count = kernel.count();
        assert!(count > 0);
        let expected_frob: E = a.iter().zip(&b).map(|(&a, &b)| a * frobenius.map(b)).sum();
        assert_eq!(algebra::mixed_dot(&frobenius, &a, &b), expected_frob);
        let mut acc = a.clone();
        let weight = a[0];
        algebra::mixed_scalar_mul_add(&frobenius, &mut acc, weight, &b);
        assert_eq!(
            acc,
            a.iter()
                .zip(&b)
                .map(|(&a, &b)| a + weight * frobenius.map(b))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            kernel.count(),
            count,
            "same-field nonidentity must not use the registered kernel"
        );
        let composed = Compose::new(identity, frobenius.clone());
        assert_eq!(algebra::mixed_dot(&composed, &a, &b), expected_frob);
        let base = Basefield::<E>::new();
        let primes: Vec<P> = (0..a.len()).map(|i| P::from(i as u64 + 1)).collect();
        assert_eq!(
            algebra::mixed_dot(&base, &a, &primes),
            a.iter()
                .zip(&primes)
                .map(|(&a, &b)| a * E::from_base_prime_field(b))
                .sum()
        );
        assert_eq!(kernel.count(), count);
    }

    #[test]
    fn batch_dispatch_retains_snapshot_without_holding_registry_lock() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (kernel, _registration) = register();
        kernel.remove_on_dot.store(true, Ordering::Relaxed);
        let chunk = crate::utils::workload_size::<E>().max(1);
        let a = values(5 * chunk + 3, 7);
        let b = values(a.len(), 11);
        let expected: E = a.iter().zip(&b).map(|(&a, &b)| a * b).sum();
        #[cfg(feature = "parallel")]
        let actual = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap()
            .install(|| algebra::dot(&a, &b));
        #[cfg(not(feature = "parallel"))]
        let actual = algebra::dot(&a, &b);
        assert_eq!(actual, expected);
        assert!(
            get::<E>().is_none(),
            "callback removed its registration without deadlocking"
        );
        #[cfg(feature = "parallel")]
        assert_eq!(
            kernel.dot_calls.load(Ordering::Relaxed),
            a.len().div_ceil(chunk)
        );
        #[cfg(not(feature = "parallel"))]
        assert_eq!(kernel.dot_calls.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn batch_geometric_optional_hook_preserves_nonzero_accumulator_and_fallback() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (kernel, _registration) = register();
        let coefficients = values(4, 23);
        for points in [
            vec![E::ZERO, E::ONE, E::from(3), E::from(17)],
            values(4, 61),
        ] {
            for n in [0, 1, 3, 7, 8, 9, 65] {
                let initial = values(n, 2);
                let expected: Vec<E> = initial
                    .iter()
                    .enumerate()
                    .map(|(i, &v)| {
                        v + coefficients
                            .iter()
                            .zip(&points)
                            .map(|(&c, &p)| c * p.pow([i as u64]))
                            .sum::<E>()
                    })
                    .collect();
                for enabled in [false, true] {
                    kernel.handle_geometric.store(enabled, Ordering::Relaxed);
                    let before = kernel.count();
                    let mut actual = initial.clone();
                    algebra::geometric_accumulate(&mut actual, coefficients.clone(), &points);
                    assert_eq!(actual, expected);
                    assert_eq!(kernel.count() - before, usize::from(enabled));
                }
            }
        }
        // The old public helper silently zipped unequal argument slices. Do not
        // change that boundary or offer a malformed shape to a batch kernel.
        let mut actual = values(5, 7);
        let mut expected = actual.clone();
        let points = values(2, 3);
        let before = kernel.count();
        for (i, value) in expected.iter_mut().enumerate() {
            *value += coefficients
                .iter()
                .zip(&points)
                .map(|(&c, &p)| c * p.pow([i as u64]))
                .sum::<E>();
        }
        algebra::geometric_accumulate(&mut actual, coefficients, &points);
        assert_eq!(actual, expected);
        assert_eq!(kernel.count(), before);
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn batch_parallel_sumcheck_rounds_match_one_worker_and_scalar_reference() {
        let _lock = TEST_LOCK.lock().unwrap();
        let (kernel, _registration) = register();
        for len in [32768, 32771] {
            let a = values(len, 2);
            let b = values(len, 7);
            let weights = [E::ZERO, E::ONE, -E::ONE, E::new(P::from(5), P::from(8))];
            let run = || {
                let mut a = a.clone();
                let mut b = b.clone();
                let mut rounds = Vec::new();
                for weight in weights {
                    let expected_a = reference_fold(&a, weight);
                    let expected_b = reference_fold(&b, weight);
                    let coefficients =
                        sumcheck::fold_and_compute_polynomial(&mut a, &mut b, weight);
                    assert_eq!(a, expected_a);
                    assert_eq!(b, expected_b);
                    assert_eq!(coefficients, reference_sumcheck(&a, &b));
                    rounds.push(coefficients);
                }
                (a, b, rounds)
            };
            let single = rayon::ThreadPoolBuilder::new()
                .num_threads(1)
                .build()
                .unwrap()
                .install(run);
            let parallel = rayon::ThreadPoolBuilder::new()
                .num_threads(4)
                .build()
                .unwrap()
                .install(run);
            assert_eq!(single, parallel);
        }
        assert!(kernel.count() > 0);
    }
}
