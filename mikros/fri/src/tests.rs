use super::*;

fn random(state: &mut u128) -> F128 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    F128::new(*state as u64, (*state >> 64) as u64)
}

fn fixture(log_words: usize) -> (Vec<F128>, Vec<F128>) {
    let mut state = 0x25ae_8764_b9c3_2168_d475_8b10_a87f_3251;
    let words = (0..1 << log_words).map(|_| random(&mut state)).collect();
    let point = (0..log_words).map(|_| random(&mut state)).collect();
    (words, point)
}

/// Independent direct Boolean Lagrange oracle in the existing FLOCK field.
fn direct(words: &[F128], point: &[F128]) -> F128 {
    words
        .iter()
        .enumerate()
        .map(|(i, &word)| {
            point.iter().enumerate().fold(word, |acc, (bit, &r)| {
                acc * if i >> bit & 1 == 0 { F128::ONE + r } else { r }
            })
        })
        .fold(F128::ZERO, |a, b| a + b)
}

#[test]
fn ghash_isomorphism_all_basis_and_dense_arithmetic() {
    assert_eq!(to_binius(F128::ZERO), B128::ZERO);
    assert_eq!(to_binius(F128::ONE), B128::ONE);
    for bit in 0..128 {
        let f = F128::new((1u128 << bit) as u64, ((1u128 << bit) >> 64) as u64);
        assert_eq!(u128::from(to_binius(f)), 1u128 << bit);
        assert_eq!(from_binius(to_binius(f)), f);
    }
    let mut state = 0xc642_7391_53bd_109e_ab76_f15c_8774_31ad;
    for _ in 0..4096 {
        let a = random(&mut state);
        let b = random(&mut state);
        assert_eq!(from_binius(to_binius(a)), a);
        assert_eq!(from_binius(to_binius(a) + to_binius(b)), a + b);
        assert_eq!(from_binius(to_binius(a) * to_binius(b)), a * b);
    }
}

fn roundtrip(log_words: usize, threads: usize) -> (Commitment, Vec<u8>) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap()
        .install(|| {
            let (words, point) = fixture(log_words);
            let source = Source::new(words.len(), 97).unwrap();
            let (cm, cache) = source.commit(&words).unwrap();
            let value = direct(&words, &point);
            let proof = source.open(&cm, &point, value, cache).unwrap();
            // A fresh key demonstrates verification needs no prover Source or message access.
            let key = VerifierKey::new(words.len()).unwrap();
            verify(&key, &cm, &point, value, &proof).unwrap();
            assert_eq!(
                bincode::deserialize::<Commitment>(&bincode::serialize(&cm).unwrap()).unwrap(),
                cm
            );
            (cm, proof)
        })
}

#[test]
fn complete_native_mle_fri_one_and_four_workers() {
    for log_words in [1, 5, 10, 13] {
        assert_eq!(roundtrip(log_words, 1), roundtrip(log_words, 4));
    }
}

#[test]
fn wrong_statement_and_malformed_proofs_reject() {
    let (words, point) = fixture(10);
    let source = Source::new(words.len(), 97).unwrap();
    let (cm, cache) = source.commit(&words).unwrap();
    let value = direct(&words, &point);
    let proof = source.open(&cm, &point, value, cache).unwrap();
    let key = source.verifier_key();
    assert!(verify(key, &cm, &point, value + F128::ONE, &proof).is_err());
    let mut wrong_point = point.clone();
    wrong_point[3] += F128::ONE;
    assert!(verify(key, &cm, &wrong_point, value, &proof).is_err());
    let mut wrong_root = cm.clone();
    wrong_root.root[0] ^= 1;
    assert!(verify(key, &wrong_root, &point, value, &proof).is_err());
    let mut wrong_params = cm.clone();
    wrong_params.parameter_digest[4] ^= 1;
    assert!(verify(key, &wrong_params, &point, value, &proof).is_err());
    let mut wrong_shape = cm.clone();
    wrong_shape.words *= 2;
    assert!(verify(key, &wrong_shape, &point, value, &proof).is_err());
    assert!(
        verify(
            &VerifierKey::new(words.len() * 2).unwrap(),
            &cm,
            &point,
            value,
            &proof
        )
        .is_err()
    );
    assert!(verify(key, &cm, &point[..point.len() - 1], value, &proof).is_err());
    for cut in [0, 1, 16, proof.len() / 2, proof.len() - 1] {
        assert!(verify(key, &cm, &point, value, &proof[..cut]).is_err());
    }
    for i in [0, 15, proof.len() / 2, proof.len() - 1] {
        let mut changed = proof.clone();
        changed[i] ^= 1;
        assert!(
            verify(key, &cm, &point, value, &changed).is_err(),
            "tamper {i}"
        );
    }
    let mut trailing = proof.clone();
    trailing.push(0);
    assert!(verify(key, &cm, &point, value, &trailing).is_err());
}

#[test]
fn boolean_boundary_points_are_complete_and_value_bound() {
    let (words, random_point) = fixture(6);
    let source = Source::new(words.len(), 97).unwrap();
    let mut points = vec![
        vec![F128::ZERO; 6],
        vec![F128::ONE; 6],
        (0..6)
            .map(|i| if i % 2 == 0 { F128::ONE } else { F128::ZERO })
            .collect(),
    ];
    for index in [0, 3, 5] {
        let mut point = random_point.clone();
        point[index] = F128::ONE;
        points.push(point);
    }
    for point in points {
        let (cm, cache) = source.commit(&words).unwrap();
        let value = direct(&words, &point);
        let proof = source.open(&cm, &point, value, cache).unwrap();
        verify(source.verifier_key(), &cm, &point, value, &proof).unwrap();
        assert!(
            verify(
                source.verifier_key(),
                &cm,
                &point,
                value + F128::ONE,
                &proof
            )
            .is_err()
        );
        // A malicious prover knows the false value before Fiat-Shamir challenges are drawn.
        let (_, wrong_cache) = source.commit(&words).unwrap();
        let bad = source
            .open(&cm, &point, value + F128::ONE, wrong_cache)
            .unwrap();
        assert!(verify(source.verifier_key(), &cm, &point, value + F128::ONE, &bad).is_err());
    }
}

#[test]
fn boundary_schedule_matches_native_proof_off_boundary() {
    let (words, point) = fixture(8);
    let source = Source::new(words.len(), 97).unwrap();
    let (cm, cache) = source.commit(&words).unwrap();
    let value = direct(&words, &point);
    let native = source.open(&cm, &point, value, cache).unwrap();
    let (_, cache) = source.commit(&words).unwrap();
    let mut transcript = prover_transcript();
    transcript
        .observe()
        .write_bytes(&statement(&source.key, &cm, &point, value));
    let mut channel =
        ProverMerkleTranscriptChannel::<_, Challenger, B128, StdHashSuite>::new(&mut transcript);
    let folder = FRIFoldProver::new(&source.key.params, &source.ntt, cache.codeword, cache.tree);
    prove_boundary(
        cache.message,
        &point.into_iter().map(to_binius).collect::<Vec<_>>(),
        folder,
        &mut channel,
    );
    drop(channel);
    assert_eq!(native, transcript.finalize());
}

#[test]
fn profiled_schedule_matches_native_proof() {
    let (words, point) = fixture(10);
    let source = Source::new(words.len(), 97).unwrap();
    let (cm, cache) = source.commit(&words).unwrap();
    let value = direct(&words, &point);
    let mut reference = prover_transcript();
    reference
        .observe()
        .write_bytes(&statement(&source.key, &cm, &point, value));
    let mut native_channel =
        ProverMerkleTranscriptChannel::<_, Challenger, B128, ProverHashSuite>::new(&mut reference);
    let folder = FRIFoldProver::new(&source.key.params, &source.ntt, cache.codeword, cache.tree);
    prove_mlecheck_basefold(
        cache.message,
        &point.iter().copied().map(to_binius).collect::<Vec<_>>(),
        to_binius(value),
        None,
        &[],
        folder,
        &mut native_channel,
    );
    drop(native_channel);
    let native = reference.finalize();
    let (_, cache) = source.commit(&words).unwrap();
    let mut transcript = prover_transcript();
    transcript
        .observe()
        .write_bytes(&statement(&source.key, &cm, &point, value));
    let mut channel =
        ProverMerkleTranscriptChannel::<_, Challenger, B128, ProverHashSuite>::new(&mut transcript);
    let folder = FRIFoldProver::new(&source.key.params, &source.ntt, cache.codeword, cache.tree);
    profiling::prove_profiled(
        cache.message,
        &point.into_iter().map(to_binius).collect::<Vec<_>>(),
        to_binius(value),
        &source.key.params,
        folder,
        &mut channel,
    );
    drop(channel);
    assert_eq!(native, transcript.finalize());
}

#[test]
fn cache_binding_and_dimensions_are_checked() {
    for words in [0, 1, 3, 9, usize::MAX] {
        assert!(VerifierKey::new(words).is_err());
    }
    let (words, point) = fixture(6);
    let source = Source::new(words.len(), 97).unwrap();
    assert!(source.commit(&words[..words.len() - 1]).is_err());
    let (cm, _) = source.commit(&words).unwrap();
    let mut changed = words.clone();
    changed[0] += F128::ONE;
    let (_, bad_cache) = source.commit(&changed).unwrap();
    assert!(
        source
            .open(&cm, &point, direct(&words, &point), bad_cache)
            .is_err()
    );
    assert_eq!(
        calculate_n_test_queries(QUERY_SECURITY_BITS, LOG_INV_RATE),
        241
    );
    assert_eq!(
        source.verifier_key().parameter_profile()["native_queries"],
        241
    );
    assert_eq!(
        source.verifier_key().parameter_digest(),
        VerifierKey::new(words.len()).unwrap().parameter_digest()
    );
}
