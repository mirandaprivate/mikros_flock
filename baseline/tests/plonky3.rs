use flock_field::test_rng::Rng;
use hadamard_baselines::{
    Statement,
    plonky3::{self, Base, Claims, Extension, Method, Proof},
};
use p3_binary_field::TowerLevel;
use p3_challenger::FieldChallenger;
use p3_field::PrimeCharacteristicRing;

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
    let c: Vec<_> = a.iter().zip(&b).map(|(a, b)| a & b).collect();
    (
        Statement::from_inputs(log_n, &a, &b, &c).unwrap(),
        [a, b, c],
    )
}

fn challenger() -> plonky3::Challenger {
    plonky3::challenger(b"native-baseline-test/v1")
}

fn roundtrip(s: &Statement, inputs: &[Vec<u8>; 3], method: Method) -> (Vec<u8>, Claims) {
    let [a, b, c] = inputs;
    let mut pc = challenger();
    let proof = plonky3::prove(s, a, b, c, method, &mut pc).unwrap();
    let bytes = bincode::serialize(&proof).unwrap();
    assert_eq!(bytes.len(), 56 * s.log_n + 64);
    assert_eq!(proof.field_elements(), 3 * s.log_n + 3);
    let decoded: Proof = bincode::deserialize(&bytes).unwrap();
    let mut vc = challenger();
    let claims = plonky3::verify(s, &decoded, &mut vc).unwrap();
    assert_eq!(
        pc.sample_algebra_element::<Extension>(),
        vc.sample_algebra_element::<Extension>()
    );
    plonky3::check_claims(s, a, b, c, &claims).unwrap();
    (bytes, claims)
}

#[test]
fn actual_native_small_field_and_full_extension_have_identical_proofs() {
    for n in [0, 1, 2, 3, 4, 7, 8, 10, 13] {
        for seed in [1, 29, 1337] {
            let (s, inputs) = fixture(n, seed);
            assert_eq!(
                roundtrip(&s, &inputs, Method::SmallFieldFirst),
                roundtrip(&s, &inputs, Method::FullExtension)
            );
        }
    }
    for a in 0u8..4 {
        for b in 0u8..4 {
            let inputs = [vec![a], vec![b], vec![a & b]];
            let s = Statement::from_inputs(1, &inputs[0], &inputs[1], &inputs[2]).unwrap();
            assert_eq!(
                roundtrip(&s, &inputs, Method::SmallFieldFirst),
                roundtrip(&s, &inputs, Method::FullExtension)
            );
        }
    }
}

#[test]
fn native_parallel_folds_and_factored_weights_replay_identically() {
    let (s, inputs) = fixture(17, 913);
    let single = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap();
    let multi = rayon::ThreadPoolBuilder::new()
        .num_threads(3)
        .build()
        .unwrap();
    let expected = single.install(|| roundtrip(&s, &inputs, Method::SmallFieldFirst));
    assert_eq!(
        expected,
        multi.install(|| roundtrip(&s, &inputs, Method::SmallFieldFirst))
    );
    assert_eq!(
        expected,
        multi.install(|| roundtrip(&s, &inputs, Method::FullExtension))
    );
}

#[test]
fn native_proof_rejects_tampering_and_source_oracle_authenticates_all_three_inputs() {
    let (s, inputs) = fixture(8, 617);
    let [a, b, c] = &inputs;
    let proof = plonky3::prove(&s, a, b, c, Method::SmallFieldFirst, &mut challenger()).unwrap();
    let claims = plonky3::verify(&s, &proof, &mut challenger()).unwrap();
    let mut changed = s.clone();
    changed.input_digest[0] ^= 1;
    assert!(plonky3::verify(&changed, &proof, &mut challenger()).is_err());
    for round in 0..s.log_n {
        for node in 0..3 {
            let mut bad = proof.clone();
            bad.sumcheck.round_polys[round][node] += Extension::ONE;
            assert!(plonky3::verify(&s, &bad, &mut challenger()).is_err());
        }
    }
    for field in 0..3 {
        let mut bad = proof.clone();
        match field {
            0 => bad.final_a += Extension::ONE,
            1 => bad.final_b += Extension::ONE,
            _ => bad.sumcheck.claimed_sum += Extension::ONE,
        }
        assert!(plonky3::verify(&s, &bad, &mut challenger()).is_err());
        let mut bad_claims = claims.clone();
        bad_claims.values[field] += Extension::ONE;
        assert!(plonky3::check_claims(&s, a, b, c, &bad_claims).is_err());
        let mut bad_inputs = inputs.clone();
        bad_inputs[field][0] ^= 1;
        // Rebinding the public-input digest cannot authenticate a forged opening.
        let bad_s =
            Statement::from_inputs(8, &bad_inputs[0], &bad_inputs[1], &bad_inputs[2]).unwrap();
        assert!(
            plonky3::check_claims(
                &bad_s,
                &bad_inputs[0],
                &bad_inputs[1],
                &bad_inputs[2],
                &claims
            )
            .is_err()
        );
    }
    for bad_shape in 0..3 {
        let mut bad = proof.clone();
        match bad_shape {
            0 => {
                bad.sumcheck.round_polys.pop();
            }
            1 => {
                bad.sumcheck.round_polys[0].pop();
            }
            _ => bad.sumcheck.pow_witnesses.push(Base::ONE),
        }
        assert!(plonky3::verify(&s, &bad, &mut challenger()).is_err());
    }
    for index in [0, 1, 7, 63, 127, 128, 255] {
        let mut bad_c = c.clone();
        bad_c[index / 8] ^= 1 << (index % 8);
        let bad_s = Statement::from_inputs(8, a, b, &bad_c).unwrap();
        let bad = plonky3::prove(
            &bad_s,
            a,
            b,
            &bad_c,
            Method::SmallFieldFirst,
            &mut challenger(),
        )
        .unwrap();
        assert!(plonky3::verify(&bad_s, &bad, &mut challenger()).is_err());
    }
    assert!(plonky3::prove(&s, &[], b, c, Method::SmallFieldFirst, &mut challenger()).is_err());
    let mut wrong = claims.clone();
    wrong.c_point.pop();
    assert!(plonky3::check_claims(&s, a, b, c, &wrong).is_err());
}

#[test]
fn independent_native_oracle_uses_high_to_low_coordinates_and_agrees_with_ghash_isomorphism() {
    let (_, inputs) = fixture(8, 1337);
    for index in 0usize..256 {
        let point: Vec<_> = (0..8)
            .rev()
            .map(|i| Extension::from_bool(index & (1 << i) != 0))
            .collect();
        assert_eq!(
            plonky3::evaluate_packed(&inputs[0], &point).unwrap(),
            Extension::from_bool(inputs[0][index / 8] & (1 << (index % 8)) != 0)
        );
    }
    let mut ch = challenger();
    for n in [0, 1, 7, 8, 9, 13] {
        let (_, inputs) = fixture(n, 1337);
        let point: Vec<Extension> = (0..n).map(|_| ch.sample_algebra_element()).collect();
        let ghash_point: Vec<_> = point
            .iter()
            .rev()
            .map(|x| hadamard_baselines::dao_thaler::from_canonical(x.to_repr()))
            .collect();
        for input in inputs {
            let value = plonky3::evaluate_packed(&input, &point).unwrap();
            assert_eq!(
                hadamard_baselines::dao_thaler::from_canonical(value.to_repr()),
                hadamard_baselines::evaluate_packed(&input, &ghash_point).unwrap()
            );
        }
    }
}
