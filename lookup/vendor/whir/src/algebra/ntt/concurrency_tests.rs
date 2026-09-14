//! Cold-start regression tests inside the same Rayon pool that builds caches.
//!
//! OS-thread callers are insufficient: a lock owner can run nested Rayon work,
//! and work-stealing can re-enter another transform waiting for that owner's
//! cache or OnceLock. Run these with an external test-process timeout when
//! checking an unfixed implementation; no detached timeout thread is used.

use super::*;
use crate::algebra::{
    fields::{Field64, Field64_2},
    univariate_evaluate,
};
use ark_ff::{Fp2, Fp2Config, Fp64, MontBackend, MontConfig, MontFp};
use ark_std::rand::{
    distributions::{Distribution, Standard},
    rngs::StdRng,
    Rng, SeedableRng,
};
use std::sync::Barrier;

#[derive(MontConfig)]
#[modulus = "65521"]
#[generator = "17"]
struct ConcurrentPrimeConfig;
type P = Fp64<MontBackend<ConcurrentPrimeConfig, 1>>;
struct ConcurrentQuadraticConfig;
impl Fp2Config for ConcurrentQuadraticConfig {
    type Fp = P;
    const NONRESIDUE: P = MontFp!("17");
    const FROBENIUS_COEFF_FP2_C1: &'static [P] = &[MontFp!("1"), MontFp!("65520")];
}
type E = Fp2<ConcurrentQuadraticConfig>;

fn mixed_engine() -> NttEngine<E> {
    let order = 32 * 9 * 5 * 181 * 181;
    let exponent = (65_521u64.pow(2) - 1) / order as u64;
    let root = (1..128u64)
        .find_map(|c| {
            let candidate = E::new(P::from(c), P::ONE).pow([exponent]);
            [2, 3, 5, 181]
                .iter()
                .all(|prime| candidate.pow([(order / prime) as u64]) != E::ONE)
                .then_some(candidate)
        })
        .expect("real quadratic extension contains the tested root order");
    NttEngine::new_with_supported_primes(order, root, &[2, 3, 5, 181])
}

fn samples<F: Field>(length: usize, seed: u64) -> Vec<F>
where
    Standard: Distribution<F>,
{
    let mut rng = StdRng::seed_from_u64(seed);
    (0..length).map(|_| rng.gen()).collect()
}

fn check_sampled_rows<F: Field>(engine: &NttEngine<F>, size: usize, input: &[F], output: &[F]) {
    assert_eq!(input.len(), output.len());
    for row in [0, input.len() / size - 1] {
        let source = &input[row * size..(row + 1) * size];
        let actual = &output[row * size..(row + 1) * size];
        for index in [0, 1, size / 2, size - 1] {
            assert_eq!(
                actual[index],
                univariate_evaluate(source, engine.root(size).pow([index as u64]))
            );
        }
    }
}

#[test]
fn rayon_cold_base_root_cache_same_and_distinct_orders() {
    for threads in [1, 2, 4] {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();
        for cold_round in 0..12 {
            let order = 3usize * (1usize << 16);
            let root = Field64::GENERATOR.pow([(18_446_744_069_414_584_321u64 - 1) / order as u64]);
            let engine = NttEngine::new(order, Field64_2::new(root, Field64::ZERO));
            let barrier = Barrier::new(threads);
            pool.install(|| {
                rayon::broadcast(|worker| {
                    let sizes = if cold_round % 2 == 0 {
                        [8192; 4]
                    } else {
                        [8192, 12288, 6144, 4096]
                    };
                    let size = sizes[worker.index()];
                    let input =
                        samples::<Field64_2>(size, (cold_round * 7 + worker.index()) as u64);
                    let mut output = input.clone();
                    // Broadcast guarantees exactly one callback per worker;
                    // unlike a barrier in an arbitrary par_iter, this has no
                    // unscheduled callbacks exceeding the pool's capacity.
                    barrier.wait();
                    engine.ntt(&mut output);
                    check_sampled_rows(&engine, size, &input, &output);
                });
            });
        }
    }
}

#[test]
fn rayon_cold_extension_root_cache_mixed_orders() {
    for threads in [1, 2, 4] {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();
        for cold_round in 0..12 {
            let engine = mixed_engine();
            let barrier = Barrier::new(threads);
            pool.install(|| {
                rayon::broadcast(|worker| {
                    // 32 does not divide p-1. Each requested root is truly in E;
                    // no 181 factor is used, so this isolates roots from Rader.
                    let size = [288, 160, 1440, 96][worker.index()];
                    assert!(prime_coordinate(engine.root(size)).is_none());
                    let input = samples::<E>(size * 5, (cold_round * 7 + worker.index()) as u64);
                    let mut output = input.clone();
                    barrier.wait();
                    engine.ntt_batch(&mut output, size);
                    check_sampled_rows(&engine, size, &input, &output);
                });
            });
        }
    }
}

#[test]
fn rayon_cold_rader181_first_use_and_nested_batches() {
    for threads in [1, 2, 4] {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();
        for cold_round in 0..12 {
            let engine = mixed_engine();
            assert!(engine.rader181.get().is_none());
            let barrier = Barrier::new(threads);
            pool.install(|| {
                rayon::broadcast(|worker| {
                    // 65 rows cross both the local 32-row scratch batch and the
                    // Rayon dispatcher threshold before first plan publication.
                    let input = samples::<E>(181 * 65, (cold_round * 7 + worker.index()) as u64);
                    let mut output = input.clone();
                    barrier.wait();
                    engine.ntt_batch(&mut output, 181);
                    check_sampled_rows(&engine, 181, &input, &output);
                });
            });
            assert!(engine.rader181.get().is_some_and(|plan| plan.is_some()));
            let plan = engine.rader181_plan().unwrap();
            assert!(std::ptr::eq(plan, engine.rader181_plan().unwrap()));
        }
    }
}

#[test]
fn rayon_cold_rader181_n20_selector_shape() {
    // Recorded F65521 N20 regular PCS: 8192 coefficients per message,
    // interleaving 16, codeword 17376, two 8688-point cosets. Reproduce its
    // exact dispatcher shape; the test field is a real quadratic extension
    // of the same characteristic, not the production degree-eight bridge.
    const CODEWORD: usize = 17_376;
    const SIZE: usize = 8_688;
    const LENGTH: usize = 278_016;
    for threads in [1, 2, 4, 64] {
        eprintln!("cold N20 Rader selector shape: {threads} workers, start");
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();
        let engine = mixed_engine();
        assert_eq!(sqrt_factor_from_divisors(SIZE, &engine.divisors), 48);
        let input = samples::<E>(LENGTH, 1000 + threads as u64);
        let mut output = input.clone();
        pool.install(|| {
            // Real interleaved_encode prepares the codeword-order twiddles
            // before entering ntt_batch. Leave the Rader plan completely cold.
            let _roots = engine.roots_table(CODEWORD);
            assert!(engine.rader181.get().is_none());
            engine.ntt_batch(&mut output, SIZE);
        });
        check_sampled_rows(&engine, SIZE, &input, &output);
        assert!(engine.rader181.get().is_some_and(|plan| plan.is_some()));
        eprintln!("cold N20 Rader selector shape: {threads} workers, completed");
    }
}

#[test]
fn rayon_cold_large_extension_root_tables_publish_compatible_snapshots() {
    // Direct cache requests isolate large full-E root generation from Rader.
    // Every order exceeds the 16-byte field's parallel-generation threshold.
    for threads in [2, 4] {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();
        let engine = mixed_engine();
        let barrier = Barrier::new(threads);
        pool.install(|| {
            rayon::broadcast(|worker| {
                let order = [260_640, 52_128, 28_960, 17_376][worker.index()];
                assert!(order > crate::utils::workload_size::<E>());
                assert!(prime_coordinate(engine.root(order)).is_none());
                barrier.wait();
                let snapshot = engine.roots_table(order);
                assert!(snapshot.len().is_multiple_of(order));
                // Another caller may replace the shared cache; this snapshot
                // must remain valid with its own compatible order and stride.
                let step = snapshot.len() / order;
                for index in [0, 1, order / 2, order - 1] {
                    assert_eq!(
                        snapshot[index * step],
                        engine.root(order).pow([index as u64])
                    );
                }
            });
        });
        assert!(engine.rader181.get().is_none());
    }
}
