use super::*;
use p3_field::PrimeCharacteristicRing;
use p3_multilinear_util::poly::Poly;

fn word(bits: u128) -> F128 {
    F128::new(bits as u64, (bits >> 64) as u64)
}

fn random_words(length: usize) -> Vec<F128> {
    let mut state = 0xf3a8_110c_6671_cc0b_94a1_7738_0039_2cfdu128;
    (0..length)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            word(state)
        })
        .collect()
}

/// Independent GHASH Boolean-MLE oracle: bind the least significant table
/// index bit first, without using any upstream layout, NTT, or field mapping.
fn evaluate(values: &[F128], point_lsb: &[F128]) -> F128 {
    assert_eq!(values.len(), 1 << point_lsb.len());
    let mut work = values.to_vec();
    for &r in point_lsb {
        for i in 0..work.len() / 2 {
            work[i] = work[2 * i] + (work[2 * i] + work[2 * i + 1]) * r;
        }
        work.truncate(work.len() / 2);
    }
    work[0]
}

fn valid_proof(log_words: usize) -> (VerifierKey, Commitment, Vec<F128>, F128, Vec<u8>) {
    let values = random_words(1 << log_words);
    let point = random_words(log_words);
    let value = evaluate(&values, &point);
    let source = Source::new(values.len(), 97).unwrap();
    let (cm, cache) = source.commit(&values).unwrap();
    let bytes = source.open(&cm, &point, value, cache).unwrap();
    (source.verifier_key().clone(), cm, point, value, bytes)
}

#[test]
fn complete_prescribed_opening_matches_independent_ghash_mle() {
    for log_words in [1, 2, 3, 4, 6, 10] {
        let (key, cm, point, value, bytes) = valid_proof(log_words);
        verify(&key, &cm, &point, value, &bytes).unwrap();
        assert_eq!(encode(&decode(&bytes).unwrap()).unwrap(), bytes);
    }
}

#[test]
fn basis_vectors_and_boolean_points_fix_the_index_bit_order() {
    let log_words = 4;
    let words = 1 << log_words;
    for index in 0..words {
        let mut values = vec![F128::ZERO; words];
        values[index] = word((1u128 << 127) | 0x87);
        let poly = Poly::new(convert_words(&values));
        for query in 0..words {
            let point: Vec<_> = (0..log_words)
                .map(|bit| {
                    if query >> bit & 1 == 0 {
                        F128::ZERO
                    } else {
                        F128::ONE
                    }
                })
                .collect();
            let actual = from_native(poly.eval_base(&native_point(&point)));
            assert_eq!(
                actual,
                evaluate(&values, &point),
                "index={index}, query={query}"
            );
        }
    }
}

#[test]
fn complete_zero_boolean_and_high_bit_queries() {
    let words = 16;
    for (values, point) in [
        (vec![F128::ZERO; words], vec![F128::ZERO; 4]),
        (random_words(words), vec![F128::ZERO; 4]),
        (random_words(words), vec![F128::ONE; 4]),
        (
            random_words(words),
            vec![word(1 << 127), F128::ONE, F128::ZERO, word((1 << 126) | 1)],
        ),
    ] {
        let value = evaluate(&values, &point);
        let source = Source::new(words, 0).unwrap();
        let (cm, cache) = source.commit(&values).unwrap();
        let bytes = source.open(&cm, &point, value, cache).unwrap();
        verify(&source.verifier_key(), &cm, &point, value, &bytes).unwrap();
    }
}

#[test]
fn wrong_claim_commitment_shape_and_profile_are_rejected() {
    let (key, cm, mut point, value, bytes) = valid_proof(6);
    assert!(verify(&key, &cm, &point, value + F128::ONE, &bytes).is_err());
    point[2] += F128::ONE;
    assert!(verify(&key, &cm, &point, value, &bytes).is_err());
    point[2] += F128::ONE;
    let mut changed = cm.clone();
    changed.root[0] ^= 1;
    assert!(verify(&key, &changed, &point, value, &bytes).is_err());
    changed = cm.clone();
    changed.words *= 2;
    assert!(verify(&key, &changed, &point, value, &bytes).is_err());
    changed = cm.clone();
    changed.parameter_digest[0] ^= 1;
    assert!(verify(&key, &changed, &point, value, &bytes).is_err());
    assert!(verify(&VerifierKey::new(128).unwrap(), &cm, &point, value, &bytes).is_err());
    assert!(verify(&key, &cm, &point[..point.len() - 1], value, &bytes).is_err());
}

#[test]
fn native_terminal_and_opening_shape_are_verified() {
    let (key, cm, point, value, bytes) = valid_proof(6);
    let mut wire = decode(&bytes).unwrap();
    wire.proof.whir.final_poly.as_mut().unwrap().as_mut_slice()[0] += E::ONE;
    assert!(verify(&key, &cm, &point, value, &encode(&wire).unwrap()).is_err());
    let mut wire = decode(&bytes).unwrap();
    wire.proof.evals.clear();
    assert!(verify(&key, &cm, &point, value, &encode(&wire).unwrap()).is_err());
    let mut wire = decode(&bytes).unwrap();
    wire.proof.evals[0] = OpeningBatch::new(vec![to_native(value), E::ZERO], vec![]);
    assert!(verify(&key, &cm, &point, value, &encode(&wire).unwrap()).is_err());
}

#[test]
fn malformed_lengths_truncation_and_trailing_bytes_are_rejected() {
    let (key, cm, point, value, bytes) = valid_proof(4);
    for length in [0, 1, 7, 8, 15, 16, 17, bytes.len() / 2, bytes.len() - 1] {
        assert!(verify(&key, &cm, &point, value, &bytes[..length]).is_err());
    }
    let mut changed = bytes.clone();
    changed.push(0);
    assert!(verify(&key, &cm, &point, value, &changed).is_err());
    // Matching outer length still does not permit an unused native tail.
    let payload_len = (changed.len() - 16) as u64;
    changed[8..16].copy_from_slice(&payload_len.to_le_bytes());
    assert!(verify(&key, &cm, &point, value, &changed).is_err());
    let mut changed = bytes.clone();
    changed[8..16].copy_from_slice(&u64::MAX.to_le_bytes());
    assert!(verify(&key, &cm, &point, value, &changed).is_err());
    let mut changed = bytes.clone();
    changed[0] ^= 1;
    assert!(verify(&key, &cm, &point, value, &changed).is_err());
    // Bogus native container length with only a tiny physical payload.
    let mut malformed = MAGIC.to_vec();
    malformed.extend_from_slice(&20u64.to_le_bytes());
    malformed.extend_from_slice(&[255; 20]);
    assert!(verify(&key, &cm, &point, value, &malformed).is_err());
}

#[test]
fn source_shape_cache_and_claim_are_checked() {
    let values = random_words(16);
    let point = random_words(4);
    let value = evaluate(&values, &point);
    let source = Source::new(values.len(), 0).unwrap();
    assert!(source.commit(&values[..8]).is_err());
    let (cm, cache) = source.commit(&values).unwrap();
    let mut changed = cm.clone();
    changed.root[0] ^= 1;
    assert!(source.open(&changed, &point, value, cache).is_err());
    let (cm, cache) = source.commit(&values).unwrap();
    assert!(source.open(&cm, &point, value + F128::ONE, cache).is_err());
}

#[test]
fn deterministic_seed_independent_setup_and_proof() {
    let values = random_words(16);
    let point = random_words(4);
    let value = evaluate(&values, &point);
    let mut proofs = Vec::new();
    for seed in [0, 97] {
        let source = Source::new(values.len(), seed).unwrap();
        let (cm, cache) = source.commit(&values).unwrap();
        let bytes = source.open(&cm, &point, value, cache).unwrap();
        proofs.push((cm, bytes));
    }
    assert_eq!(proofs[0], proofs[1]);
}

#[test]
fn parameter_derivation_records_actual_security_and_grinding_without_allocating_source() {
    for log_words in [4, 21, 25, 26, 30] {
        let key = VerifierKey::new(1 << log_words)
            .unwrap_or_else(|e| panic!("log_words={log_words}: {e}"));
        let metadata = key.parameter_profile();
        assert_eq!(metadata["words"], 1usize << log_words);
        assert!(
            metadata["prescribed_opening_security_bits"]
                .as_f64()
                .unwrap()
                .is_finite()
        );
        assert!(metadata["actual_max_pow_bits"].as_u64().unwrap() <= MAX_POW_BITS as u64);
        if log_words <= 26 {
            assert_eq!(metadata["actual_max_pow_bits"], 0);
        }
        assert_eq!(
            key.parameter_digest(),
            VerifierKey::new(1 << log_words).unwrap().parameter_digest()
        );
    }
    for words in [0, 1, 3, (1usize << 30) + 1, 1usize << 31] {
        assert!(VerifierKey::new(words).is_err());
    }
}
