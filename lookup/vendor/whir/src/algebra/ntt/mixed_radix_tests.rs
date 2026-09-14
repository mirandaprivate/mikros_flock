//! Differential tests for explicitly opted-in low-adicity mixed radices.
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

// This prime and primitive generator were checked against every factor of
// p-1 = 2^6 * 3^2 * 5^3 * 11 * 17^2. It provides the same required 5/17
// smooth subgroup as F251^16 without depending on its unfinished Ark bridge.
#[derive(MontConfig)]
#[modulus = "228888001"]
#[generator = "7"]
struct RadixPrimeConfig;
type P = Fp64<MontBackend<RadixPrimeConfig, 1>>;
struct QuadraticConfig;
impl Fp2Config for QuadraticConfig {
    type Fp = P;
    const NONRESIDUE: P = MontFp!("7");
    const FROBENIUS_COEFF_FP2_C1: &'static [P] = &[MontFp!("1"), MontFp!("228888000")];
}
type E = Fp2<QuadraticConfig>;

#[derive(MontConfig)]
#[modulus = "65521"]
#[generator = "17"]
struct SmallPrimeConfig;
type S = Fp64<MontBackend<SmallPrimeConfig, 1>>;
struct SmallQuadraticConfig;
impl Fp2Config for SmallQuadraticConfig {
    type Fp = S;
    const NONRESIDUE: S = MontFp!("17");
    const FROBENIUS_COEFF_FP2_C1: &'static [S] = &[MontFp!("1"), MontFp!("65520")];
}
type SE = Fp2<SmallQuadraticConfig>;

fn base_engine() -> NttEngine<E> {
    let order = 20_808_000;
    let root = E::new(P::from(7u64).pow([11]), P::ZERO);
    NttEngine::new_with_supported_primes(order, root, &[2, 3, 5, 17])
}

fn extension_engine() -> NttEngine<SE> {
    // F65521^2 already contains the full factor 181^2. Derive a root, then
    // verify its exact order rather than fabricating FftField parameters.
    let order = 32 * 9 * 5 * 181 * 181;
    let exponent = (65_521u64.pow(2) - 1) / order as u64;
    let root = (1..128u64)
        .find_map(|c| {
            let root = SE::new(S::from(c), S::ONE).pow([exponent]);
            [2, 3, 5, 181]
                .iter()
                .all(|q| root.pow([(order / q) as u64]) != SE::ONE)
                .then_some(root)
        })
        .expect("deterministic root search must find the smooth subgroup");
    NttEngine::new_with_supported_primes(order, root, &[2, 3, 5, 181])
}

fn random_values<F: Field>(length: usize, seed: u64) -> Vec<F>
where
    Standard: Distribution<F>,
{
    let mut rng = StdRng::seed_from_u64(seed);
    (0..length).map(|_| rng.gen()).collect()
}

fn check_transform<F: Field>(engine: &NttEngine<F>, size: usize, seed: u64)
where
    Standard: Distribution<F>,
{
    for zero in [false, true] {
        let values = if zero {
            vec![F::ZERO; 2 * size]
        } else {
            random_values(2 * size, seed)
        };
        let mut actual = values.clone();
        engine.ntt_batch(&mut actual, size);
        let indices = if size <= 425 {
            (0..size).collect::<Vec<_>>()
        } else {
            vec![0, 1, size / 3, size - 1]
        };
        for (input, output) in values.chunks_exact(size).zip(actual.chunks_exact(size)) {
            for &index in &indices {
                assert_eq!(
                    output[index],
                    univariate_evaluate(input, engine.root(size).pow([index as u64])),
                    "size={size}, index={index}"
                );
            }
        }
        engine.intt_batch(&mut actual, size);
        assert_eq!(
            actual,
            values
                .iter()
                .map(|v| *v * F::from(size as u64))
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn mixed_dense_radices_five_seventeen_match_direct_evaluation() {
    let engine = base_engine();
    for size in [
        1, 2, 3, 5, 10, 15, 17, 25, 34, 75, 85, 125, 255, 289, 425, 1445, 7225,
    ] {
        check_transform(&engine, size, size as u64 + 1);
    }
}

#[test]
fn mixed_dense_181_and_181_squared_match_true_extension_evaluations() {
    let engine = extension_engine();
    assert!(prime_coordinate(engine.root(181)).is_none());
    assert!(prime_coordinate(engine.root(180)).is_some());
    for size in [5, 180, 181, 362, 543, 905, 32761] {
        check_transform(&engine, size, size as u64 + 11);
    }
}

fn check_rs<F: Field>(engine: &NttEngine<F>, shape: (usize, usize, usize, usize))
where
    Standard: Distribution<F>,
{
    let (message_len, interleaving, codeword_len, mask_len) = shape;
    let a = Buffer::from(random_values::<F>(message_len * interleaving, 172));
    let b = Buffer::from(random_values::<F>(message_len * interleaving, 291));
    let inputs = [&a, &b];
    let masks = Buffer::from(random_values::<F>(2 * interleaving * mask_len, 432));
    let actual = engine.interleaved_encode(
        Messages::new(&inputs, message_len, interleaving),
        &masks,
        codeword_len,
    );
    let points = engine.evaluation_points(
        message_len + mask_len,
        codeword_len,
        &(0..codeword_len).collect::<Vec<_>>(),
    );
    for (index, point) in points.into_iter().enumerate() {
        for column in 0..2 * interleaving {
            let start = (column % interleaving) * message_len;
            let message = &inputs[column / interleaving].to_slice()[start..start + message_len];
            let mask = &masks.to_slice()[column * mask_len..(column + 1) * mask_len];
            let expected = univariate_evaluate(message, point)
                + point.pow([message_len as u64]) * univariate_evaluate(mask, point);
            assert_eq!(
                actual.to_slice()[index * 2 * interleaving + column],
                expected,
                "shape={shape:?}, row={index}, column={column}"
            );
        }
    }
}

#[test]
fn mixed_radix_rs_coset_layout_masks_and_non_power_of_two_lengths() {
    let base = base_engine();
    for shape in [
        (7, 2, 17, 0),
        (17, 2, 85, 0),
        (31, 1, 85, 2),
        (68, 4, 170, 0),
        (97, 2, 289, 3),
        (201, 1, 425, 0),
    ] {
        check_rs(&base, shape);
    }
    let extension = extension_engine();
    for shape in [(90, 2, 181, 0), (181, 2, 905, 0), (182, 1, 362, 3)] {
        check_rs(&extension, shape);
    }
}

#[test]
fn mixed_radix_cache_snapshots_replace_without_historical_lcm() {
    let engine = base_engine();
    // Orders 125 and 289 are coprime: an LCM cache would need 36125 entries.
    let old = engine.roots_table(125);
    let next = engine.roots_table(289);
    assert_eq!(old.len(), 125);
    assert_eq!(next.len(), 289);
    assert_eq!(engine.roots.read().unwrap().len(), 289);
    assert_eq!(old[1], engine.root(125));
    assert_eq!(next[1], engine.root(289));
    let old_base = engine.base_roots_table(125).unwrap();
    let next_base = engine.base_roots_table(289).unwrap();
    assert_eq!(old_base.len(), 125);
    assert_eq!(next_base.len(), 289);
    assert_eq!(engine.base_roots.read().unwrap().len(), 289);
    assert_eq!(E::new(old_base[1], P::ZERO), engine.root(125));
    assert_eq!(E::new(next_base[1], P::ZERO), engine.root(289));
}

#[test]
fn mixed_radix_cache_replacement_is_safe_under_concurrent_orders() {
    let engine = base_engine();
    std::thread::scope(|scope| {
        for size in [125, 289, 85, 75, 255] {
            let engine = &engine;
            scope.spawn(move || {
                for repeat in 0..3 {
                    check_transform(engine, size, (size + repeat) as u64);
                }
            });
        }
    });
    assert!(engine.roots.read().unwrap().len() <= 289);
    assert!(engine.base_roots.read().unwrap().len() <= 289);
}

#[test]
fn mixed_radix_opt_in_preserves_default_gold_and_rejects_missing_roots() {
    let gold = NttEngine::<Field64_2>::new_from_fftfield();
    assert_eq!(gold.order, 3usize << 32);
    assert!(gold
        .divisors
        .iter()
        .all(|d| !d.is_multiple_of(5) && !d.is_multiple_of(17)));
    assert_eq!(
        gold.root(1 << 20),
        Field64_2::new(
            NttEngine::<Field64>::new_from_fftfield().root(1 << 20),
            Field64::ZERO
        )
    );
    let root = base_engine().root(85);
    assert!(std::panic::catch_unwind(|| NttEngine::new(85, root)).is_err());
    assert!(
        std::panic::catch_unwind(|| NttEngine::new_with_supported_primes(85, root, &[2, 3, 5]))
            .is_err()
    );
    assert!(
        std::panic::catch_unwind(|| NttEngine::new_with_supported_primes(
            85,
            E::ONE,
            &[2, 3, 5, 17]
        ))
        .is_err()
    );
    assert!(
        std::panic::catch_unwind(|| NttEngine::new_with_supported_primes(
            85,
            root,
            &[2, 3, 5, 17, 19]
        ))
        .is_err()
    );
    assert!(
        std::panic::catch_unwind(|| NttEngine::new_with_supported_primes(85, root, &[2, 5, 3, 17]))
            .is_err()
    );
}
