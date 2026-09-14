//! Exact differential tests for the optional Rader-181 convolution.
use super::*;
use crate::{
    algebra::{
        embedding::Identity,
        linear_form::{Evaluate, LinearForm, MultilinearExtension},
        ntt::NTT,
        univariate_evaluate,
    },
    hash,
    parameters::ProtocolParameters,
    protocols::{params::DecodingRegime, whir::Config},
    transcript::{codecs::Empty, DomainSeparator, Proof, ProverState, VerifierState},
};
use ark_ff::{Fp2, Fp2Config, Fp64, MontBackend, MontConfig, MontFp};
use ark_std::rand::{rngs::StdRng, Rng, SeedableRng};

// A distinct Rust type from mixed_radix_tests keeps global WHIR NTT
// registration in the proof test isolated from concurrently running tests.
#[derive(MontConfig)]
#[modulus = "65521"]
#[generator = "17"]
struct RaderPrimeConfig;
type P = Fp64<MontBackend<RaderPrimeConfig, 1>>;
struct RaderQuadraticConfig;
impl Fp2Config for RaderQuadraticConfig {
    type Fp = P;
    const NONRESIDUE: P = MontFp!("17");
    const FROBENIUS_COEFF_FP2_C1: &'static [P] = &[MontFp!("1"), MontFp!("65520")];
}
type E = Fp2<RaderQuadraticConfig>;

fn engine() -> NttEngine<E> {
    let order = 32 * 9 * 5 * 181 * 181;
    let exponent = (65_521u64.pow(2) - 1) / order as u64;
    let root = (1..128u64)
        .find_map(|c| {
            let root = E::new(P::from(c), P::ONE).pow([exponent]);
            [2, 3, 5, 181]
                .iter()
                .all(|q| root.pow([(order / q) as u64]) != E::ONE)
                .then_some(root)
        })
        .expect("actual quadratic field contains the required smooth subgroup");
    NttEngine::new_with_supported_primes(order, root, &[2, 3, 5, 181])
}

fn dense_engine(fast: &NttEngine<E>) -> NttEngine<E> {
    let mut dense = NttEngine::new_with_supported_primes(
        fast.order,
        fast.omega_order,
        &[2, 3, 5, 181],
    );
    dense.force_dense_181 = true;
    dense
}

fn samples(length: usize, seed: u64) -> Vec<E> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..length).map(|_| rng.gen()).collect()
}

#[test]
fn rader181_primitive_matches_dense_direct_and_inverse_with_reused_cache() {
    let fast = engine();
    assert!(prime_coordinate(fast.root(180)).is_some());
    assert!(prime_coordinate(fast.root(181)).is_none());
    let dense = dense_engine(&fast);
    let mut vectors = vec![vec![E::ZERO; 181], vec![E::ONE; 181]];
    for index in [0, 1, 2, 90, 180] {
        let mut basis = vec![E::ZERO; 181];
        basis[index] = E::new(P::from(3u64), P::from(7u64));
        vectors.push(basis);
    }
    // Cross both the bounded scratch-batch boundary and, on normal targets,
    // the Rayon workload boundary. The dense flag belongs to the engine.
    vectors.push(samples(34 * 181, 12));
    for values in vectors {
        let mut actual = values.clone();
        let mut reference = values.clone();
        fast.ntt_batch(&mut actual, 181);
        dense.ntt_batch(&mut reference, 181);
        assert_eq!(actual, reference);
        for (row, transformed) in values.chunks_exact(181).zip(actual.chunks_exact(181)) {
            for index in [0, 1, 90, 180] {
                assert_eq!(transformed[index], univariate_evaluate(row, fast.root(181).pow([index as u64])));
            }
        }
        fast.intt_batch(&mut actual, 181);
        assert_eq!(actual, values.iter().map(|&v| v * E::from(181u64)).collect::<Vec<_>>());
    }
    let plan = fast.rader181_plan().unwrap();
    assert!(std::ptr::eq(plan, fast.rader181_plan().unwrap()));
    let mut indices = plan.input_indices;
    indices.sort_unstable();
    assert_eq!(indices.as_slice(), &(1..181).collect::<Vec<_>>());
    assert!(dense.rader181.get().is_none(), "forced dense must not initialize the optimized plan");
    assert_eq!(plan.convolution.base_roots.read().unwrap().len(), 180);
}

#[test]
fn rader181_square_and_alternate_root_match_dense() {
    let fast = engine();
    let dense = dense_engine(&fast);
    for size in [362, 543, 905, 181 * 181] {
        let values = samples(size, size as u64 + 8);
        let mut actual = values.clone();
        let mut reference = values.clone();
        fast.ntt(&mut actual);
        dense.ntt(&mut reference);
        assert_eq!(actual, reference, "size={size}");
        fast.intt(&mut actual);
        assert_eq!(actual, values.iter().map(|&v| v * E::from(size as u64)).collect::<Vec<_>>());
    }
    let alternate = NttEngine::new_with_supported_primes(
        fast.order, fast.omega_order.pow([7]), &[2, 3, 5, 181],
    );
    let mut actual = samples(181, 991);
    let mut reference = actual.clone();
    alternate.ntt(&mut actual);
    dense_engine(&alternate).ntt(&mut reference);
    assert_eq!(actual, reference);
    assert_ne!(fast.rader181_plan().unwrap().normalized_kernel_spectrum,
        alternate.rader181_plan().unwrap().normalized_kernel_spectrum);
}

#[test]
fn rader181_missing_convolution_domain_stays_dense() {
    let outer = engine();
    let limited = NttEngine::new_with_supported_primes(181, outer.root(181), &[181]);
    let values = samples(181, 27);
    let mut actual = values.clone();
    limited.ntt(&mut actual);
    assert!(limited.rader181_plan().is_none());
    for (index, value) in actual.iter().enumerate() {
        assert_eq!(*value, univariate_evaluate(&values, limited.root(181).pow([index as u64])));
    }
}

#[test]
fn rader181_extension_only_convolution_root_stays_dense() {
    // Here p + 1 = 180 * 181, so both roots exist in the quadratic field,
    // but no primitive 180th root lies in the prime field.
    #[derive(MontConfig)]
    #[modulus = "32579"]
    #[generator = "6"]
    struct NonBasePrimeConfig;
    type Q = Fp64<MontBackend<NonBasePrimeConfig, 1>>;
    struct NonBaseQuadraticConfig;
    impl Fp2Config for NonBaseQuadraticConfig {
        type Fp = Q;
        const NONRESIDUE: Q = MontFp!("2");
        const FROBENIUS_COEFF_FP2_C1: &'static [Q] = &[MontFp!("1"), MontFp!("32578")];
    }
    type Q2 = Fp2<NonBaseQuadraticConfig>;
    let order = 180 * 181;
    let root = (1..128u64).find_map(|i| {
        let root = Q2::new(Q::from(i), Q::ONE).pow([32578]);
        [2, 3, 5, 181].iter().all(|q| root.pow([(order / q) as u64]) != Q2::ONE).then_some(root)
    }).unwrap();
    let engine = NttEngine::new_with_supported_primes(order, root, &[2, 3, 5, 181]);
    assert!(prime_coordinate(engine.root(180)).is_none());
    assert!(engine.rader181_plan().is_none());
    let values = (0..181u64).map(|i| Q2::new(Q::from(i), Q::from(i + 1))).collect::<Vec<_>>();
    let mut actual = values.clone();
    engine.ntt(&mut actual);
    for (index, value) in actual.iter().enumerate() {
        assert_eq!(*value, univariate_evaluate(&values, engine.root(181).pow([index as u64])));
    }
}

#[test]
fn rader181_rs_masks_cosets_and_layout_match_dense() {
    let fast = engine();
    let dense = dense_engine(&fast);
    for (message, interleaving, codeword, mask_len) in [
        (90, 2, 181, 0), (181, 2, 905, 0), (182, 1, 362, 3),
    ] {
        let a = Buffer::from(samples(message * interleaving, 211));
        let b = Buffer::from(samples(message * interleaving, 219));
        let inputs = [&a, &b];
        let masks = Buffer::from(samples(2 * interleaving * mask_len, 301));
        let actual = fast.interleaved_encode(Messages::new(&inputs, message, interleaving), &masks, codeword);
        let reference = dense.interleaved_encode(Messages::new(&inputs, message, interleaving), &masks, codeword);
        assert_eq!(actual.to_slice(), reference.to_slice());
        let points = fast.evaluation_points(message + mask_len, codeword, &(0..codeword).collect::<Vec<_>>());
        for (row, point) in points.iter().enumerate() {
            for column in 0..2 * interleaving {
                let start = (column % interleaving) * message;
                let coefficients = &inputs[column / interleaving].to_slice()[start..start + message];
                let mask = &masks.to_slice()[column * mask_len..(column + 1) * mask_len];
                assert_eq!(actual.to_slice()[row * 2 * interleaving + column],
                    univariate_evaluate(coefficients, *point)
                    + point.pow([message as u64]) * univariate_evaluate(mask, *point));
            }
        }
    }
    assert!(fast.rader181.get().unwrap().is_some());
    assert!(dense.rader181.get().is_none());
}

#[test]
fn rader181_plan_is_shared_under_concurrent_first_use() {
    let fast = engine();
    let expected = {
        let mut values = samples(181, 72);
        dense_engine(&fast).ntt(&mut values);
        values
    };
    std::thread::scope(|scope| {
        for _ in 0..8 {
            let fast = &fast;
            let expected = &expected;
            scope.spawn(move || {
                for _ in 0..3 {
                    let mut values = samples(181, 72);
                    fast.ntt(&mut values);
                    assert_eq!(&values, expected);
                    assert!(std::ptr::eq(fast.rader181_plan().unwrap(), fast.rader181_plan().unwrap()));
                }
            });
        }
    });
    assert_eq!(fast.rader181_plan().unwrap().convolution.base_roots.read().unwrap().len(), 180);
}

fn complete_proof() -> Proof {
    let config = Config::<Identity<E>>::new(1024, &ProtocolParameters {
        decoding_regime: DecodingRegime::Unique,
        starting_log_inv_rate: 1,
        initial_folding_factor: 2,
        folding_factor: 2,
        // Correctness-only small-field test; not a benchmark/security profile.
        security_level: 8,
        pow_bits: 0,
        batch_size: 1,
        hash_id: hash::BLAKE3,
    });
    let table = samples(1024, 173);
    let forms = [
        MultilinearExtension::new(samples(10, 29)),
        MultilinearExtension::new(samples(10, 79)),
    ];
    let values = forms.iter().map(|f| f.evaluate(&Identity::new(), &table)).collect::<Vec<_>>();
    let ds = DomainSeparator::protocol(&config).session(&"rader181-exact-byte-test").instance(&Empty);
    let mut prover = ProverState::new_std(&ds);
    let table = Buffer::from(table);
    let witness = config.commit(&mut prover, &[&table]);
    let owned = forms.iter().cloned().map(|f| Box::new(f) as Box<dyn LinearForm<E>>).collect();
    let _pending = config.prove(&mut prover, &[&table], vec![&witness], owned, Buffer::from(values.clone()));
    let proof = prover.proof();
    let mut verifier = VerifierState::new_std(&ds, &proof);
    let commitment = config.receive_commitment(&mut verifier).unwrap();
    let final_claim = config.verify(&mut verifier, &[&commitment], &values).unwrap();
    final_claim.verify(forms.iter().map(|f| f as &dyn LinearForm<E>)).unwrap();
    verifier.check_eof().unwrap();
    proof
}

#[test]
fn rader181_complete_whir_proof_bytes_match_forced_dense() {
    struct Restore(Option<Arc<dyn ReedSolomon<E>>>);
    impl Drop for Restore {
        fn drop(&mut self) {
            NTT.remove::<E>();
            if let Some(old) = self.0.take() { NTT.insert::<E>(old); }
        }
    }
    let _restore = Restore(NTT.get::<E>());
    let fast = Arc::new(engine());
    let dense = Arc::new(dense_engine(&fast));
    NTT.insert::<E>(fast.clone());
    let actual = complete_proof();
    assert!(fast.rader181.get().is_some_and(|p| p.is_some()), "full proof must actually use radix 181");
    NTT.insert::<E>(dense.clone());
    let reference = complete_proof();
    assert!(dense.rader181.get().is_none(), "all worker transforms must obey forced-dense mode");
    assert_eq!(actual.narg_string, reference.narg_string);
    assert_eq!(actual.hints, reference.hints);
}
