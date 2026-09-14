use flock_field::F128;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros_permutation::{field::SmallField, packed, wiring};

fn challenger() -> FsChallenger {
    let mut c = FsChallenger::new(b"packed-wiring-composition-test-v1");
    c.observe_label(b"actual-outer-commitment-placeholder-for-source-oracle-tests");
    c
}

fn bit(words: &[F128], address: usize) -> bool {
    let w = words[address >> 7];
    if address & 127 < 64 {
        (w.lo >> (address & 63)) & 1 != 0
    } else {
        (w.hi >> (address & 63)) & 1 != 0
    }
}

fn fixture(log: usize) -> (Vec<F128>, wiring::BitWiringStatement) {
    let n = 1usize << log;
    let mut words = vec![F128::ZERO; n.div_ceil(128)];
    for i in 0..n {
        if (i / 2).count_ones() & 1 != 0 {
            if i & 127 < 64 {
                words[i >> 7].lo |= 1 << (i & 63);
            } else {
                words[i >> 7].hi |= 1 << (i & 63);
            }
        }
    }
    // Bit-level swaps preserve each pair's value while moving word coordinates.
    let statement =
        wiring::BitWiringStatement::new(&(0..n).map(|i| i ^ 1).collect::<Vec<_>>()).unwrap();
    (words, statement)
}

fn tensor_value(words: &[F128], claim: &wiring::BitTensorClaim) -> F128 {
    (0..words.len()).fold(F128::ZERO, |sum, row| {
        let weight = claim
            .row_factors
            .iter()
            .enumerate()
            .fold(F128::ONE, |w, (b, pair)| w * pair[(row >> b) & 1]);
        let value = (0..128)
            .filter(|&column| bit(words, row * 128 + column))
            .fold(F128::ZERO, |v, column| v + claim.column_weights[column]);
        sum + weight * value
    })
}

fn synthetic_claims(words: &[F128], statement: &wiring::BitWiringStatement) -> [packed::Claim; 2] {
    let kappa = wiring::bit_packed_kappa(statement).unwrap();
    let row_point = (0..statement.bits().ilog2() as usize - kappa.ilog2() as usize)
        .map(|i| F128::new((19 + 3 * i) as u64, (7 + i) as u64))
        .collect::<Vec<_>>();
    let mut column_weights = (0..kappa - 1)
        .map(|i| F128::new((31 + i * 7) as u64, i as u64))
        .collect::<Vec<_>>();
    column_weights.push(column_weights.iter().fold(F128::ONE, |a, &b| a + b));
    let basis = SmallField::for_width(statement.shape().width)
        .unwrap()
        .basis();
    std::array::from_fn(|side| {
        let value = (0..statement.bits()).fold(F128::ZERO, |sum, address| {
            let row = address / kappa;
            let weight =
                row_point
                    .iter()
                    .enumerate()
                    .fold(column_weights[address % kappa], |w, (b, &r)| {
                        w * if row & (1 << b) == 0 {
                            F128::ONE + r
                        } else {
                            r
                        }
                    });
            let tag = if side == 0 {
                address
            } else {
                statement.sigma()[address]
            };
            let encoded = (0..statement.bits().ilog2() as usize).fold(
                if bit(words, address) {
                    F128::ONE
                } else {
                    F128::ZERO
                },
                |v, b| {
                    if tag & (1 << b) == 0 {
                        v
                    } else {
                        v + basis[b + 1]
                    }
                },
            );
            sum + weight * encoded
        });
        packed::Claim {
            row_point: row_point.clone(),
            column_weights: column_weights.clone(),
            value,
        }
    })
}

#[test]
fn packed_bit_adapter_matches_original_bit_functional_with_padding_and_arbitrary_columns() {
    for log in [6, 7, 8, 10] {
        let (words, statement) = fixture(log);
        let claims = synthetic_claims(&words, &statement);
        let actual = wiring::reduce_bit_claims_packed(&statement, &claims).unwrap();
        assert_eq!(actual.column_weights.len(), 128);
        assert_eq!(actual.row_factors.len(), log.saturating_sub(7));
        assert_eq!(actual.value, tensor_value(&words, &actual));
        if log < 7 {
            assert!(
                actual.column_weights[1usize << log..]
                    .iter()
                    .all(|x| *x == F128::ZERO)
            );
        }
    }
}

#[test]
fn packed_bit_adapter_rejects_different_or_malformed_queries_and_values() {
    let (words, statement) = fixture(10);
    let claims = synthetic_claims(&words, &statement);
    let mut bad = claims.clone();
    bad[1].value += F128::ONE;
    assert!(wiring::reduce_bit_claims_packed(&statement, &bad).is_err());
    let mut bad = claims.clone();
    bad[1].row_point[0] += F128::ONE;
    assert!(wiring::reduce_bit_claims_packed(&statement, &bad).is_err());
    let mut bad = claims.clone();
    bad[1].column_weights.swap(0, 1);
    assert!(wiring::reduce_bit_claims_packed(&statement, &bad).is_err());
    let mut bad = claims.clone();
    bad[0].column_weights.pop();
    assert!(wiring::reduce_bit_claims_packed(&statement, &bad).is_err());
    let mut bad = claims.clone();
    bad[0].row_point.pop();
    assert!(wiring::reduce_bit_claims_packed(&statement, &bad).is_err());
    let mut bad = claims;
    for claim in &mut bad {
        claim.column_weights[0] += F128::ONE;
    }
    assert!(wiring::reduce_bit_claims_packed(&statement, &bad).is_err());
}

#[test]
fn packed_bit_wiring_roundtrip_authenticates_the_original_witness_and_binds_sigma() {
    for log in [8, 10] {
        let (words, statement) = fixture(log);
        let prepared = wiring::prepare_bits_packed(&words, &statement).unwrap();
        for repetition in 0..2 {
            let mut prover = challenger();
            prover.observe_f128(F128::new(repetition, 0));
            let (proof, claim, _) =
                wiring::prove_bits_prepared_packed(&statement, &prepared, &mut prover).unwrap();
            let mut verifier = challenger();
            verifier.observe_f128(F128::new(repetition, 0));
            let verified = wiring::verify_bits_packed(&statement, &proof, &mut verifier).unwrap();
            assert_eq!(claim, verified);
            assert_eq!(claim.value, tensor_value(&words, &claim));
            assert_eq!(prover.sample_f128(), verifier.sample_f128());
            let wrong = wiring::BitWiringStatement::new(&(0..statement.bits()).collect::<Vec<_>>())
                .unwrap();
            assert!(
                wiring::prove_bits_prepared_packed(&wrong, &prepared, &mut challenger()).is_err()
            );
            let mut wrong_verifier = challenger();
            wrong_verifier.observe_f128(F128::new(repetition, 0));
            assert!(wiring::verify_bits_packed(&wrong, &proof, &mut wrong_verifier).is_err());
        }
        let mut wrong_words = words;
        wrong_words[0].lo ^= 1;
        let invalid = wiring::prepare_bits_packed(&wrong_words, &statement).unwrap();
        assert!(
            wiring::prove_bits_prepared_packed(&statement, &invalid, &mut challenger()).is_err()
        );
    }
}
