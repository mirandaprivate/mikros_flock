//! Compatibility and adversarial checks for the synchronized transcript.
use crate::{
    Error,
    bits::PackedBits,
    claims::{LinearQuery, Source, row_weight},
    permutation::{self, CommittedClaims, Options, PreparedPermutation, Strategy},
    profile::Engine,
    wiring::{self, BitWiringStatement},
};
use flock_field::F128;
use flock_transcript::challenger::{Challenger, FsChallenger};

fn ch() -> FsChallenger {
    let mut c = FsChallenger::new(b"mikros-lockstep-test-v1");
    c.observe_label(b"outer-commitment-and-public-statement");
    c.observe_bytes(&[0x52; 32]);
    c
}

fn sources(n: usize, width: usize) -> PreparedPermutation {
    let mask = u128::MAX >> (128 - width);
    let values: Vec<_> = (0..n)
        .map(|i| {
            let x = (i as u128 + 29).wrapping_mul(0xb9d0_c325_27ff_f47a_d53b_41aa_4732_771d);
            (x ^ (x >> 43) ^ (x << 31)) & mask
        })
        .collect();
    permutation::prepare(
        PackedBits::from_fn_parallel(n, width, |i| values[i]).unwrap(),
        PackedBits::from_fn_parallel(n, width, |i| values[n - 1 - i]).unwrap(),
    )
    .unwrap()
}

fn authenticate(source: &PreparedPermutation, claims: &CommittedClaims) {
    for (claim, bits) in claims
        .evaluations
        .iter()
        .zip([source.left(), source.right()])
    {
        assert_eq!(claim.query.evaluate(bits).unwrap(), claim.value);
    }
    let points = claims
        .evaluations
        .each_ref()
        .map(|claim| match &claim.query {
            LinearQuery::Rows { point, .. } => point,
            _ => panic!("expected source row queries"),
        });
    assert_eq!(points[0], points[1]);
}

#[test]
fn lockstep_prepared_full_reference_and_optimized_agree() {
    // Empty input, no GKR layer, log1, non-power-of-two lengths, byte tails,
    // and field-width transitions all exercise the same statement semantics.
    for (n, width) in [
        (0, 1),
        (1, 31),
        (2, 9),
        (3, 33),
        (9, 17),
        (257, 31),
        (65, 128),
    ] {
        let source = sources(n, width);
        let mut baseline = ch();
        let (expected_proof, expected_claims, _, _) = permutation::prove_committed_lockstep(
            &source,
            Options::default(),
            Engine::Reference,
            &mut baseline,
        )
        .unwrap();
        let expected_next = baseline.sample_f128();
        authenticate(&source, &expected_claims);
        for strategy in [Strategy::BitwidthAware, Strategy::FullExtension] {
            let options = Options {
                strategy,
                ..Options::default()
            };
            let prepared = permutation::prepare_tree_coefficients(&source, options).unwrap();
            for engine in [
                Engine::Reference,
                Engine::Fused,
                Engine::Registers,
                Engine::Combined,
            ] {
                let mut full = ch();
                let (proof, claims, _, _) =
                    permutation::prove_committed_lockstep(&source, options, engine, &mut full)
                        .unwrap();
                assert_eq!(proof, expected_proof, "n={n}, width={width}, {engine:?}");
                assert_eq!(claims, expected_claims);
                assert_eq!(full.sample_f128(), expected_next);
                let mut online = ch();
                let (proof, claims, _, _) =
                    permutation::prove_prepared_lockstep(&prepared, engine, &mut online).unwrap();
                assert_eq!(proof, expected_proof);
                assert_eq!(claims, expected_claims);
                assert_eq!(online.sample_f128(), expected_next);
                let mut verifier = ch();
                assert_eq!(
                    permutation::verify_committed_lockstep(
                        source.statement(),
                        &proof,
                        &mut verifier,
                    )
                    .unwrap(),
                    expected_claims
                );
                assert_eq!(verifier.sample_f128(), expected_next);
            }
        }
    }
}

#[test]
fn lockstep_parallel_compact_register_path_matches_reference() {
    let source = sources((1 << 15) + 17, 9);
    let mut baseline = ch();
    let (expected, claims, _, _) = permutation::prove_committed_lockstep(
        &source,
        Options::default(),
        Engine::Reference,
        &mut baseline,
    )
    .unwrap();
    let next = baseline.sample_f128();
    let prepared = permutation::prepare_tree_coefficients(&source, Options::default()).unwrap();
    for engine in [Engine::Fused, Engine::Combined] {
        let mut prover = ch();
        let (proof, actual, _, _) =
            permutation::prove_prepared_lockstep(&prepared, engine, &mut prover).unwrap();
        assert_eq!(proof, expected);
        assert_eq!(actual, claims);
        assert_eq!(prover.sample_f128(), next);
    }
}

#[test]
fn lockstep_checks_each_tree_and_proof_shape_independently() {
    let source = sources(17, 9);
    let (proof, _, _, _) = permutation::prove_committed_lockstep(
        &source,
        Options::default(),
        Engine::Fused,
        &mut ch(),
    )
    .unwrap();
    let reject = |bad: &permutation::Proof| {
        assert!(
            permutation::verify_committed_lockstep(source.statement(), bad, &mut ch()).is_err()
        );
    };
    for side in 0..2 {
        for mutation in 0..7 {
            let mut bad = proof.clone();
            let tree = if side == 0 {
                &mut bad.left
            } else {
                &mut bad.right
            };
            match mutation {
                0 => tree.layers[1].rounds[0][0] += F128::ONE,
                1 => tree.layers.last_mut().unwrap().left += F128::ONE,
                2 => tree.layers.last_mut().unwrap().right += F128::ONE,
                3 => {
                    tree.layers.pop();
                }
                4 => tree.layers.push(tree.layers.last().unwrap().clone()),
                5 => {
                    tree.layers[1].rounds.pop();
                }
                _ => tree.layers[1].rounds.push([F128::ZERO; 3]),
            }
            reject(&bad);
        }
        let mut bad = proof.clone();
        bad.roots[side] += F128::ONE;
        reject(&bad);
    }
    // Equal-and-opposite errors (addition is XOR) must not cancel across trees.
    let mut bad = proof.clone();
    bad.left.layers[1].rounds[0][0] += F128::ONE;
    bad.right.layers[1].rounds[0][0] += F128::ONE;
    reject(&bad);
    let mut bad = proof.clone();
    std::mem::swap(&mut bad.left, &mut bad.right);
    reject(&bad);
}

struct Record {
    inner: FsChallenger,
    pending: Vec<usize>,
    barriers: Vec<Vec<usize>>,
}
impl Record {
    fn new() -> Self {
        Self {
            inner: ch(),
            pending: vec![],
            barriers: vec![],
        }
    }
}
impl Challenger for Record {
    fn fork_from_seed(&self, _: [F128; 2], _: &'static [u8]) -> Self {
        panic!("the sequential lockstep test transcript must not fork")
    }
    fn observe_label(&mut self, x: &[u8]) {
        self.inner.observe_label(x);
    }
    fn observe_bytes(&mut self, x: &[u8]) {
        self.inner.observe_bytes(x);
    }
    fn observe_f128(&mut self, x: F128) {
        self.pending.push(1);
        self.inner.observe_f128(x);
    }
    fn observe_f128_slice(&mut self, x: &[F128]) {
        self.pending.push(x.len());
        self.inner.observe_f128_slice(x);
    }
    fn sample_f128(&mut self) -> F128 {
        self.barriers.push(std::mem::take(&mut self.pending));
        self.inner.sample_f128()
    }
}

#[test]
fn lockstep_binds_both_messages_before_every_shared_challenge() {
    let source = sources(8, 4);
    let mut prover = Record::new();
    let (proof, claims, _, _) = permutation::prove_committed_lockstep(
        &source,
        Options::default(),
        Engine::Combined,
        &mut prover,
    )
    .unwrap();
    assert_eq!(
        prover.barriers,
        vec![
            vec![],        // z
            vec![2, 2, 2], // roots and both pairs, then layer-0 rho
            vec![3, 3],
            vec![2, 2], // layer 1
            vec![3, 3],
            vec![3, 3],
            vec![2, 2], // layer 2
        ]
    );
    let mut verifier = Record::new();
    assert_eq!(
        permutation::verify_committed_lockstep(source.statement(), &proof, &mut verifier,).unwrap(),
        claims
    );
    assert_eq!(prover.barriers, verifier.barriers);
    assert_eq!(prover.sample_f128(), verifier.sample_f128());
}

#[test]
fn sequential_challenge_barriers_preserve_the_legacy_schedule() {
    let source = sources(8, 4);
    let mut prover = Record::new();
    let (proof, claims, _, _) =
        permutation::prove_committed(&source, Options::default(), Engine::Combined, &mut prover)
            .unwrap();
    assert_eq!(
        prover.barriers,
        vec![
            vec![],     // z
            vec![2, 2], // roots and the left tree's top pair
            vec![3],
            vec![2],
            vec![3],
            vec![3],
            vec![2], // rest of left tree
            vec![2],
            vec![3],
            vec![2],
            vec![3],
            vec![3],
            vec![2], // entire right tree
        ]
    );
    // Replay the old outer domain and tree order explicitly, without using
    // the public permutation verifier's new schedule dispatcher.
    let mut legacy = ch();
    legacy.observe_label(b"mikros-committed-bitwidth-permutation-v1");
    legacy.observe_bytes(&(source.statement().rows as u64).to_le_bytes());
    legacy.observe_bytes(&(source.statement().width as u64).to_le_bytes());
    legacy.observe_label(b"mikros-permutation-grand-product-v1");
    let z = legacy.sample_f128();
    legacy.observe_f128_slice(&proof.roots);
    legacy.observe_label(b"left-tree");
    let left = crate::sumcheck::verify(&proof.left, proof.roots[0], 3, &mut legacy).unwrap();
    legacy.observe_label(b"right-tree");
    let right = crate::sumcheck::verify(&proof.right, proof.roots[1], 3, &mut legacy).unwrap();
    for (leaf, claim) in [left, right].into_iter().zip(&claims.evaluations) {
        assert_eq!(leaf.value + z, claim.value);
    }
    assert_eq!(prover.sample_f128(), legacy.sample_f128());
}

struct Constant(F128);
impl Challenger for Constant {
    fn fork_from_seed(&self, _: [F128; 2], _: &'static [u8]) -> Self {
        panic!("the constant-challenge test transcript must not fork")
    }
    fn observe_f128(&mut self, _: F128) {}
    fn sample_f128(&mut self) -> F128 {
        self.0
    }
}

#[test]
fn lockstep_zero_and_one_challenges_and_zero_values_are_defined() {
    for value in [F128::ZERO, F128::ONE] {
        for n in [0, 1, 2, 5, 33] {
            let source = permutation::prepare(
                PackedBits::from_fn_parallel(n, 9, |_| 0).unwrap(),
                PackedBits::from_fn_parallel(n, 9, |_| 0).unwrap(),
            )
            .unwrap();
            let (proof, claims, _, _) = permutation::prove_committed_lockstep(
                &source,
                Options::default(),
                Engine::Combined,
                &mut Constant(value),
            )
            .unwrap();
            assert_eq!(
                permutation::verify_committed_lockstep(
                    source.statement(),
                    &proof,
                    &mut Constant(value),
                )
                .unwrap(),
                claims
            );
            authenticate(&source, &claims);
        }
    }
}

fn bit_fixture(log: usize) -> (Vec<F128>, BitWiringStatement) {
    let n = 1usize << log;
    // Permute bits within their bit-value class, including across words and
    // high bit coordinates. The witness need not be word-preserving.
    let bits: Vec<_> = (0..n).map(|i| (i ^ (i >> 3) ^ (i >> 7)) & 1 != 0).collect();
    let mut sigma: Vec<_> = (0..n).collect();
    for class in [false, true] {
        let group: Vec<_> = (0..n).filter(|&i| bits[i] == class).collect();
        for (j, &i) in group.iter().enumerate() {
            sigma[i] = group[(j + 1) % group.len()];
        }
    }
    let mut packed = vec![F128::ZERO; n.div_ceil(128)];
    for (i, &bit) in bits.iter().enumerate() {
        if bit {
            if i & 127 < 64 {
                packed[i >> 7].lo |= 1 << (i & 63);
            } else {
                packed[i >> 7].hi |= 1 << (i & 63);
            }
        }
    }
    (packed, BitWiringStatement::new(&sigma).unwrap())
}

fn bit_eval(packed: &[F128], point: &[F128]) -> F128 {
    (0..1usize << point.len()).fold(F128::ZERO, |sum, i| {
        let word = packed[i >> 7];
        let raw = (word.lo as u128) | ((word.hi as u128) << 64);
        if raw >> (i & 127) & 1 == 1 {
            sum + row_weight(point, i)
        } else {
            sum
        }
    })
}

#[test]
fn lockstep_bit_wiring_returns_one_authenticated_mle_after_distinct_tags() {
    for log in [0, 1, 3, 7, 9] {
        let (packed, statement) = bit_fixture(log);
        let source = wiring::prepare_bits(&packed, &statement).unwrap();
        let prepared = wiring::prepare_bit_coefficients(&source, Options::default()).unwrap();
        let mut baseline = ch();
        let (expected, claim, _, _) = wiring::prove_bits_lockstep(
            &statement,
            &source,
            Options::default(),
            Engine::Reference,
            &mut baseline,
        )
        .unwrap();
        assert_eq!(claim.value, bit_eval(&packed, &claim.point));
        let next = baseline.sample_f128();
        for engine in [Engine::Fused, Engine::Combined] {
            let mut prover = ch();
            let (proof, actual, _, _) =
                wiring::prove_bits_prepared_lockstep(&statement, &prepared, engine, &mut prover)
                    .unwrap();
            assert_eq!(proof, expected);
            assert_eq!(actual, claim);
            assert_eq!(prover.sample_f128(), next);
            let mut verifier = ch();
            assert_eq!(
                wiring::verify_bits_lockstep(&statement, &proof, &mut verifier).unwrap(),
                claim
            );
            assert_eq!(verifier.sample_f128(), next);
        }
        if log >= 3 {
            let wrong = BitWiringStatement::new(&(0..1usize << log).collect::<Vec<_>>()).unwrap();
            assert!(wiring::verify_bits_lockstep(&wrong, &expected, &mut ch()).is_err());
            assert!(
                wiring::prove_bits_prepared_lockstep(
                    &wrong,
                    &prepared,
                    Engine::Combined,
                    &mut ch(),
                )
                .is_err()
            );
        }
    }
}

#[test]
fn lockstep_bit_adapter_rejects_different_points_values_and_source_tags() {
    let (packed, statement) = bit_fixture(8);
    let source = wiring::prepare_bits(&packed, &statement).unwrap();
    let (_, claims, _, _) = permutation::prove_committed_lockstep(
        source.permutation(),
        Options::default(),
        Engine::Fused,
        &mut ch(),
    )
    .unwrap();
    let reduced = wiring::reduce_bit_claims(&statement, &claims).unwrap();
    assert_eq!(reduced[0], reduced[1]);
    assert_ne!(claims.evaluations[0].value, claims.evaluations[1].value);
    assert_eq!(
        wiring::reduce_bit_claims_lockstep(&statement, &claims).unwrap(),
        reduced[0]
    );

    for side in 0..2 {
        let mut bad = claims.clone();
        bad.evaluations[side].value += F128::ONE;
        assert_eq!(
            wiring::reduce_bit_claims_lockstep(&statement, &bad),
            Err(Error::Constraint)
        );
        let mut bad = claims.clone();
        if let LinearQuery::Rows { point, .. } = &mut bad.evaluations[side].query {
            point[0] += F128::ONE;
        }
        let wrong_point = wiring::reduce_bit_claims(&statement, &bad).unwrap();
        // Force the reduced VALUES equal while retaining the wrong point;
        // equality of values alone is insufficient to merge source openings.
        bad.evaluations[side].value += wrong_point[side].value + reduced[0].value;
        let wrong_point = wiring::reduce_bit_claims(&statement, &bad).unwrap();
        assert_eq!(wrong_point[0].value, wrong_point[1].value);
        assert_eq!(
            wiring::reduce_bit_claims_lockstep(&statement, &bad),
            Err(Error::ProofShape)
        );
    }
    let mut bad = claims.clone();
    bad.evaluations.swap(0, 1);
    assert_eq!(
        wiring::reduce_bit_claims_lockstep(&statement, &bad),
        Err(Error::ProofShape)
    );
    let mut bad = claims;
    bad.evaluations[1].source = Source::Left;
    assert_eq!(
        wiring::reduce_bit_claims_lockstep(&statement, &bad),
        Err(Error::ProofShape)
    );
}

#[test]
fn lockstep_domain_is_distinct_and_sequential_api_remains_available() {
    let source = sources(17, 9);
    let mut sequential = ch();
    let (old, claims, _, _) =
        permutation::prove_committed(&source, Options::default(), Engine::Fused, &mut sequential)
            .unwrap();
    let mut verifier = ch();
    assert_eq!(
        permutation::verify_committed(source.statement(), &old, &mut verifier).unwrap(),
        claims
    );
    assert_eq!(sequential.sample_f128(), verifier.sample_f128());
    let (new, _, _, _) = permutation::prove_committed_lockstep(
        &source,
        Options::default(),
        Engine::Fused,
        &mut ch(),
    )
    .unwrap();
    assert_ne!(old, new);
    assert!(permutation::verify_committed_lockstep(source.statement(), &old, &mut ch()).is_err());
    assert!(permutation::verify_committed(source.statement(), &new, &mut ch()).is_err());
}
