use flock_field::{F128, test_rng::Rng};
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use hadamard_baselines::{
    Claims, Method, Proof, Statement, check_claims, dao_thaler, evaluate_packed, prove, verify,
};

// Frozen copy of the independent existing Flock opening oracle.
#[path = "../support/flock_oracle.rs"]
mod flock_oracle;

fn challenger() -> FsChallenger {
    FsChallenger::with_hash(b"baseline-tests/public-input/v1", HashKind::Blake3)
}

fn fixture(log_n: usize, seed: u64) -> (Statement, [Vec<u8>; 3]) {
    let n = 1usize << log_n;
    let mut rng = Rng::new(seed);
    let mut a = vec![0; n.div_ceil(8)];
    let mut b = a.clone();
    rng.fill_bytes(&mut a);
    rng.fill_bytes(&mut b);
    if n < 8 {
        a[0] &= ((1u16 << n) - 1) as u8;
        b[0] &= ((1u16 << n) - 1) as u8;
    }
    let c = a.iter().zip(&b).map(|(a, b)| a & b).collect::<Vec<_>>();
    (
        Statement::from_inputs(log_n, &a, &b, &c).unwrap(),
        [a, b, c],
    )
}

fn roundtrip(s: &Statement, inputs: &[Vec<u8>; 3], method: Method) -> (Proof, Claims) {
    let [a, b, c] = inputs;
    let mut pc = challenger();
    let (proof, claims) = prove(s, a, b, c, method, &mut pc).unwrap();
    let mut vc = challenger();
    let output = verify(s, method, &proof, &mut vc).unwrap();
    assert_eq!(output, claims);
    assert_eq!(pc.sample_f128(), vc.sample_f128());
    check_claims(s, a, b, c, &output).unwrap();
    let encoded = bincode::serialize(&proof).unwrap();
    assert_eq!(encoded.len(), 16 * proof.field_elements() + 8);
    assert_eq!(bincode::deserialize::<Proof>(&encoded).unwrap(), proof);
    (proof, claims)
}

#[test]
fn optimized_paths_match_their_generic_controls_at_boundaries() {
    for n in [0, 1, 2, 3, 4, 6, 7, 8, 10, 13] {
        for seed in [1, 29, 1337] {
            let (s, inputs) = fixture(n, seed);
            let sff = roundtrip(&s, &inputs, Method::GhashSmallFieldFirst);
            assert_eq!(sff, roundtrip(&s, &inputs, Method::GhashFullExtension));
            let dao = roundtrip(&s, &inputs, Method::DaoThaler);
            assert_eq!(dao, roundtrip(&s, &inputs, Method::DaoThalerGeneric));
        }
    }
}

#[test]
fn packed_first_round_covers_all_local_bit_patterns() {
    // N=2 exhausts all four A/B bits, independently of extension challenges.
    for a in 0u8..4 {
        for b in 0u8..4 {
            let inputs = [vec![a], vec![b], vec![a & b]];
            let s = Statement::from_inputs(1, &inputs[0], &inputs[1], &inputs[2]).unwrap();
            assert_eq!(
                roundtrip(&s, &inputs, Method::GhashSmallFieldFirst),
                roundtrip(&s, &inputs, Method::GhashFullExtension)
            );
            roundtrip(&s, &inputs, Method::DaoThaler);
        }
    }
    for pattern in [0, 255, 0x55, 0xaa, 0x80, 1] {
        let inputs = [vec![pattern; 32], vec![0xaa; 32], vec![pattern & 0xaa; 32]];
        let s = Statement::from_inputs(8, &inputs[0], &inputs[1], &inputs[2]).unwrap();
        for method in [Method::GhashSmallFieldFirst, Method::DaoThaler] {
            roundtrip(&s, &inputs, method);
        }
    }
}

#[test]
fn parallel_paths_are_transcript_and_message_identical() {
    let (s, inputs) = fixture(17, 913);
    let single = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap();
    let multi = rayon::ThreadPoolBuilder::new()
        .num_threads(3)
        .build()
        .unwrap();
    for method in [
        Method::GhashSmallFieldFirst,
        Method::DaoThaler,
        Method::DaoThalerGeneric,
    ] {
        let expected = single.install(|| roundtrip(&s, &inputs, method));
        assert_eq!(expected, multi.install(|| roundtrip(&s, &inputs, method)));
    }
}

#[test]
fn invalid_relations_and_changed_statements_are_rejected() {
    for method in [Method::GhashSmallFieldFirst, Method::DaoThaler] {
        let (s, mut inputs) = fixture(8, 617);
        let (proof, _) = roundtrip(&s, &inputs, method);
        let mut changed = s.clone();
        changed.input_digest[0] ^= 1;
        assert!(verify(&changed, method, &proof, &mut challenger()).is_err());
        for bit in [0, 1, 7, 63, 64, 127, 128, 255] {
            inputs[2][bit / 8] ^= 1 << (bit % 8);
            let [a, b, c] = &inputs;
            let false_s = Statement::from_inputs(8, a, b, c).unwrap();
            let (bad, _) = prove(&false_s, a, b, c, method, &mut challenger()).unwrap();
            let result = verify(&false_s, method, &bad, &mut challenger());
            assert!(result.is_err() || check_claims(&false_s, a, b, c, &result.unwrap()).is_err());
            inputs[2][bit / 8] ^= 1 << (bit % 8);
        }
        let wrong_method = if method.fixed() {
            Method::GhashSmallFieldFirst
        } else {
            Method::DaoThaler
        };
        assert!(verify(&s, wrong_method, &proof, &mut challenger()).is_err());
    }
}

#[test]
fn all_message_classes_and_malformed_shapes_are_checked() {
    let (s, inputs) = fixture(8, 333);
    for method in [Method::GhashSmallFieldFirst, Method::DaoThaler] {
        let (proof, _) = roundtrip(&s, &inputs, method);
        for round in 0..s.log_n {
            for coefficient in 0..3 {
                let mut bad = proof.clone();
                bad.rounds[round][coefficient] += F128::ONE;
                assert!(verify(&s, method, &bad, &mut challenger()).is_err());
            }
        }
        for field in 0..3 {
            let mut bad = proof.clone();
            match field {
                0 => bad.c_eval += F128::ONE,
                1 => bad.final_a += F128::ONE,
                _ => bad.final_b += F128::ONE,
            }
            assert!(verify(&s, method, &bad, &mut challenger()).is_err());
        }
        let mut bad = proof.clone();
        bad.rounds.pop();
        assert!(verify(&s, method, &bad, &mut challenger()).is_err());
        bad.rounds.push([F128::ZERO; 3]);
        bad.rounds.push([F128::ZERO; 3]);
        assert!(verify(&s, method, &bad, &mut challenger()).is_err());
    }
    assert!(Statement::from_inputs(31, &[], &[], &[]).is_err());
    assert!(Statement::from_inputs(0, &[2], &[0], &[0]).is_err());
    assert!(
        prove(
            &s,
            &inputs[0][1..],
            &inputs[1],
            &inputs[2],
            Method::DaoThaler,
            &mut challenger()
        )
        .is_err()
    );
}

#[test]
fn all_three_source_openings_require_authentication() {
    let (s, inputs) = fixture(8, 69);
    for method in [Method::GhashSmallFieldFirst, Method::DaoThaler] {
        let (proof, claims) = roundtrip(&s, &inputs, method);
        for column in 0..3 {
            let mut bad = claims.clone();
            bad.values[column] += F128::ONE;
            assert!(check_claims(&s, &inputs[0], &inputs[1], &inputs[2], &bad).is_err());
            let mut changed_inputs = inputs.clone();
            changed_inputs[column][0] ^= 1;
            assert!(
                check_claims(
                    &s,
                    &changed_inputs[0],
                    &changed_inputs[1],
                    &changed_inputs[2],
                    &claims
                )
                .is_err()
            );
        }
        let mut bad = claims.clone();
        bad.ab_point.pop();
        assert!(check_claims(&s, &inputs[0], &inputs[1], &inputs[2], &bad).is_err());
        let mut forged = proof;
        assert!(!forged.final_a.is_zero() && !forged.final_b.is_zero());
        let factor = F128::generator();
        forged.final_a *= factor;
        forged.final_b *= factor.inv();
        let unauthenticated = verify(&s, method, &forged, &mut challenger()).unwrap();
        assert!(check_claims(&s, &inputs[0], &inputs[1], &inputs[2], &unauthenticated).is_err());
        assert_ne!(claims.ab_point, claims.c_point);
        let mut wrong_c_point = claims.clone();
        wrong_c_point.c_point = claims.ab_point;
        assert!(check_claims(&s, &inputs[0], &inputs[1], &inputs[2], &wrong_c_point).is_err());
    }
}

struct ConstantChallenger(F128);
impl Challenger for ConstantChallenger {
    fn observe_f128(&mut self, _: F128) {}
    fn sample_f128(&mut self) -> F128 {
        self.0
    }
    fn fork_from_seed(&self, _: [F128; 2], _: &'static [u8]) -> Self {
        Self(self.0)
    }
}

#[test]
fn zero_and_one_challenges_need_no_division_or_rejection() {
    let (s, inputs) = fixture(10, 931);
    for r in [F128::ZERO, F128::ONE] {
        for method in [Method::GhashSmallFieldFirst, Method::DaoThaler] {
            let [a, b, c] = &inputs;
            let (proof, claims) = prove(&s, a, b, c, method, &mut ConstantChallenger(r)).unwrap();
            assert_eq!(
                claims,
                verify(&s, method, &proof, &mut ConstantChallenger(r)).unwrap()
            );
            check_claims(&s, a, b, c, &claims).unwrap();
        }
    }
}

#[test]
fn same_binary_relation_is_accepted_by_flock_and_both_baselines() {
    let (s, inputs) = fixture(13, 425);
    let [a, b, c] = &inputs;
    let mut pc = challenger();
    pc.observe_bytes(&s.input_digest);
    let (proof, claim) = flock_core::zerocheck::prove_packed(a, b, c, s.log_n, &mut pc);
    let mut vc = challenger();
    vc.observe_bytes(&s.input_digest);
    let verified = flock_core::zerocheck::verify(s.log_n, &proof, &mut vc).unwrap();
    assert_eq!(verified, claim);
    flock_oracle::check_claims(a, b, c, s.log_n, &verified).unwrap();
    for method in [Method::GhashSmallFieldFirst, Method::DaoThaler] {
        roundtrip(&s, &inputs, method);
    }
}

#[test]
fn canonical_embedding_and_algorithm3_match_every_basis_vector() {
    let z = dao_thaler::generators();
    assert_eq!(z[0].square(), z[0] + F128::ONE);
    for i in 1..7 {
        assert_eq!(z[i].square(), z[i] * z[i - 1] + F128::ONE);
    }
    let mut rng = Rng::new(908);
    let mut values = (0..128)
        .map(|i| F128::new((1u128 << i) as u64, ((1u128 << i) >> 64) as u64))
        .collect::<Vec<_>>();
    values.extend(rng.f128_vec(128));
    for value in values {
        let canonical = dao_thaler::to_canonical(value);
        assert_eq!(dao_thaler::from_canonical(canonical), value);
        for i in 0..7 {
            assert_eq!(
                dao_thaler::from_canonical(dao_thaler::algorithm3_word(canonical, i)),
                value * z[i]
            );
        }
    }
}

fn rank(values: &[F128]) -> usize {
    let mut pivots = [0u128; 128];
    let mut rank = 0;
    for v in values {
        let mut row = u128::from(v.lo) | u128::from(v.hi) << 64;
        while row != 0 {
            let i = row.ilog2() as usize;
            if pivots[i] == 0 {
                pivots[i] = row;
                rank += 1;
                break;
            }
            row ^= pivots[i];
        }
    }
    rank
}

#[test]
fn algorithm2_tables_have_correct_order_and_fixed_basis_rank() {
    let mut rng = Rng::new(773);
    for n in 0usize..=12 {
        for k in 0..=n.min(7) {
            let mut point = dao_thaler::generators()[..k].to_vec();
            point.extend(rng.f128_vec(n - k));
            let table = dao_thaler::equality_table(&point, k).unwrap();
            assert_eq!(table, dao_thaler::generic_eq_table(&point).unwrap());
            for (index, &weight) in table.iter().enumerate() {
                let direct = point.iter().enumerate().fold(F128::ONE, |w, (i, &r)| {
                    w * if index & (1 << i) == 0 {
                        F128::ONE + r
                    } else {
                        r
                    }
                });
                assert_eq!(weight, direct);
            }
        }
    }
    for k in 0..=7 {
        assert_eq!(
            rank(&dao_thaler::equality_table(&dao_thaler::generators()[..k], k).unwrap()),
            1 << k
        );
    }
    assert_eq!(
        dao_thaler::operation_counts(20, 7).unwrap(),
        (8191, 1040384)
    );
    assert!(dao_thaler::equality_table(&[F128::ONE], 1).is_err());
    assert!(dao_thaler::equality_table(&[], 1).is_err());
    assert!(dao_thaler::operation_counts(3, 4).is_err());
}

#[test]
fn packed_opening_oracle_matches_direct_multilinear_evaluation() {
    for n in [0, 3, 7, 10] {
        let (_, inputs) = fixture(n, 666);
        let mut rng = Rng::new(42);
        let point = rng.f128_vec(n);
        let expected = (0..1usize << n)
            .filter(|&i| inputs[0][i / 8] & (1 << (i % 8)) != 0)
            .map(|j| {
                point.iter().enumerate().fold(F128::ONE, |w, (i, &r)| {
                    w * if j & (1 << i) == 0 { F128::ONE + r } else { r }
                })
            })
            .fold(F128::ZERO, |a, b| a + b);
        assert_eq!(evaluate_packed(&inputs[0], &point).unwrap(), expected);
    }
}
