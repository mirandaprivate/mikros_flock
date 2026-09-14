use flock_field::F128;
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros_permutation::{
    Error,
    bits::PackedBits,
    claims::{LinearQuery, Source, row_weight},
    field::{SmallField, from_raw},
    permutation::{self, Options, Strategy},
    profile::Engine,
    wiring::{self, BitWiringStatement, WordWiringStatement},
};

fn ch() -> FsChallenger {
    let mut ch = FsChallenger::with_hash(
        b"mikros-permutation-root-composition-test",
        HashKind::Blake3,
    );
    ch.observe_label(b"outer-statement-and-pcs-commitment");
    ch.observe_bytes(&[0x42; 32]);
    ch
}

fn random_word(mut state: u128) -> u128 {
    state ^= state << 13;
    state ^= state >> 17;
    state ^= state << 43;
    state.wrapping_mul(0x9e37_79b9_7f4a_7c15_3723_0950_ab39_2739)
}

fn mask(width: usize) -> u128 {
    if width == 128 {
        u128::MAX
    } else {
        (1u128 << width) - 1
    }
}

fn source_pair(n: usize, width: usize, seed: u128) -> (PackedBits, PackedBits) {
    let values: Vec<_> = (0..n)
        .map(|i| random_word(seed + i as u128) & mask(width))
        .collect();
    (
        PackedBits::from_fn_parallel(n, width, |i| values[i]).unwrap(),
        PackedBits::from_fn_parallel(n, width, |i| values[n - 1 - i]).unwrap(),
    )
}

fn bit_eval(packed: &[F128], point: &[F128]) -> F128 {
    (0..1usize << point.len()).fold(F128::ZERO, |sum, i| {
        let word = packed[i >> 7];
        let b = i & 127;
        let bit = if b < 64 {
            word.lo >> b & 1
        } else {
            word.hi >> (b - 64) & 1
        };
        if bit == 0 {
            sum
        } else {
            sum + row_weight(point, i)
        }
    })
}

fn pack(bits: &[bool]) -> Vec<F128> {
    let mut words = vec![F128::ZERO; bits.len().div_ceil(128)];
    for (i, &bit) in bits.iter().enumerate() {
        if bit {
            let b = i & 127;
            if b < 64 {
                words[i >> 7].lo |= 1 << b;
            } else {
                words[i >> 7].hi |= 1 << (b - 64);
            }
        }
    }
    words
}

#[test]
fn standalone_binding_preserves_reference_proof_bytes_and_challenges() {
    for n in [1, 8, 257] {
        for width in [1, 8, 31, 33, 64, 128] {
            let (left, right) = source_pair(n, width, 79);
            let statement = permutation::Statement::new(&left, &right).unwrap();
            let mut reference_ch = ch();
            let (reference, claims, _, _) = permutation::prove_profiled(
                &statement,
                &left,
                &right,
                Options::default(),
                Engine::Reference,
                &mut reference_ch,
            )
            .unwrap();
            let bytes = bincode::serialize(&reference).unwrap();
            let next = reference_ch.sample_f128();
            for engine in [
                Engine::Fused,
                Engine::Registers,
                Engine::Recycled,
                Engine::Combined,
            ] {
                let mut prover = ch();
                let (proof, actual, _, _) = permutation::prove_profiled(
                    &statement,
                    &left,
                    &right,
                    Options::default(),
                    engine,
                    &mut prover,
                )
                .unwrap();
                assert_eq!(bincode::serialize(&proof).unwrap(), bytes);
                assert_eq!(claims, actual);
                assert_eq!(prover.sample_f128(), next);
                assert_eq!(
                    permutation::verify(&statement, &proof, &mut ch()).unwrap(),
                    claims
                );
                claims
                    .check(&[(Source::Left, &left), (Source::Right, &right)])
                    .unwrap();
            }
        }
    }
}

#[test]
fn committed_binding_matches_reference_and_authenticates_both_queries() {
    for seed in [15, 851, 29] {
        for width in [1, 17, 31, 38, 128] {
            let (left, right) = source_pair(128, width, seed);
            let prepared = permutation::prepare(left, right).unwrap();
            let mut reference_ch = ch();
            let (reference, expected, _, _) = permutation::prove_committed(
                &prepared,
                Options::default(),
                Engine::Reference,
                &mut reference_ch,
            )
            .unwrap();
            let bytes = bincode::serialize(&reference).unwrap();
            let next = reference_ch.sample_f128();
            for engine in [Engine::Fused, Engine::Recycled, Engine::Combined] {
                for strategy in [Strategy::BitwidthAware, Strategy::FullExtension] {
                    let mut prover = ch();
                    let (proof, claims, _, _) = permutation::prove_committed(
                        &prepared,
                        Options {
                            strategy,
                            ..Default::default()
                        },
                        engine,
                        &mut prover,
                    )
                    .unwrap();
                    assert_eq!(bincode::serialize(&proof).unwrap(), bytes);
                    assert_eq!(claims, expected);
                    assert_eq!(prover.sample_f128(), next);
                    let mut verifier = ch();
                    assert_eq!(
                        permutation::verify_committed(prepared.statement(), &proof, &mut verifier)
                            .unwrap(),
                        claims
                    );
                    assert_eq!(verifier.sample_f128(), next);
                    for (claim, bits) in claims
                        .evaluations
                        .iter()
                        .zip([prepared.left(), prepared.right()])
                    {
                        assert_eq!(claim.query.evaluate(bits).unwrap(), claim.value);
                    }
                }
            }
        }
    }
}

#[test]
fn word_wiring_preserves_all_128_bits_and_returns_original_bit_mle() {
    let statement = WordWiringStatement::new(&[1, 0, 3, 2]).unwrap();
    for seed in [19u128, 9751, 491] {
        let a = from_raw(random_word(seed));
        let b = from_raw(random_word(seed + 1));
        assert_ne!(a.hi | b.hi, 0);
        let words = [a, a, b, b];
        let prepared = wiring::prepare(&words, &statement).unwrap();
        for i in 0..512 {
            assert_eq!(
                statement.bit_sigma(i),
                statement.sigma()[i >> 7] * 128 + (i & 127)
            );
            let row = prepared.permutation().left().row(i);
            let raw = (words[i >> 7].lo as u128) | ((words[i >> 7].hi as u128) << 64);
            assert_eq!(row & 1, raw >> (i & 127) & 1);
            assert_eq!(row >> 1, i as u128);
        }
        let mut prover = ch();
        let (proof, claims, _, _) = wiring::prove(
            &statement,
            &prepared,
            Options::default(),
            Engine::Recycled,
            &mut prover,
        )
        .unwrap();
        let mut verifier = ch();
        assert_eq!(
            wiring::verify(&statement, &proof, &mut verifier).unwrap(),
            claims
        );
        assert_eq!(prover.sample_f128(), verifier.sample_f128());
        for claim in &claims {
            assert_eq!(claim.value, bit_eval(&words, &claim.point));
        }
    }
}

#[test]
fn differences_in_high_word_bits_cannot_satisfy_copy_constraints() {
    let statement = WordWiringStatement::new(&[1, 0]).unwrap();
    for bit in [0, 63, 64, 99, 127] {
        let words = [F128::ZERO, from_raw(1u128 << bit)];
        let prepared = wiring::prepare(&words, &statement).unwrap();
        assert_eq!(
            wiring::prove(
                &statement,
                &prepared,
                Options::default(),
                Engine::Fused,
                &mut ch()
            )
            .unwrap_err(),
            Error::ProductMismatch
        );
    }
}

#[test]
fn bit_wiring_supports_non_word_preserving_public_permutations() {
    for log_bits in [0, 3, 7, 8, 10] {
        let n = 1usize << log_bits;
        let bits: Vec<_> = (0..n)
            .map(|i| random_word(i as u128 + 17) & 1 == 1)
            .collect();
        let mut sigma: Vec<_> = (0..n).collect();
        for value in [false, true] {
            let positions: Vec<_> = (0..n).filter(|&i| bits[i] == value).collect();
            for (j, &i) in positions.iter().enumerate() {
                sigma[i] = positions[(j + 1) % positions.len()];
            }
        }
        if n >= 128 {
            assert!(sigma.iter().enumerate().any(|(i, &s)| i % 128 != s % 128));
        }
        let statement = BitWiringStatement::new(&sigma).unwrap();
        let field = SmallField::for_width(statement.shape().width).unwrap();
        for (&tag, &image) in statement.encoded_sigma_tags().iter().zip(&sigma) {
            assert_eq!(tag, field.embed((image as u128) << 1));
        }
        let packed = pack(&bits);
        let prepared = wiring::prepare_bits(&packed, &statement).unwrap();
        let mut reference = ch();
        let (expected, expected_claims, _, _) = wiring::prove_bits(
            &statement,
            &prepared,
            Options::default(),
            Engine::Reference,
            &mut reference,
        )
        .unwrap();
        let next = reference.sample_f128();
        for engine in [Engine::Fused, Engine::Recycled, Engine::Combined] {
            let mut prover = ch();
            let (proof, claims, _, timings) = wiring::prove_bits(
                &statement,
                &prepared,
                Options::default(),
                engine,
                &mut prover,
            )
            .unwrap();
            assert_eq!(
                bincode::serialize(&proof).unwrap(),
                bincode::serialize(&expected).unwrap()
            );
            assert_eq!(claims, expected_claims);
            assert_eq!(prover.sample_f128(), next);
            assert_eq!(timings.source_checks_ms, 0.0);
            let mut verifier = ch();
            assert_eq!(
                wiring::verify_bits(&statement, &proof, &mut verifier).unwrap(),
                claims
            );
            assert_eq!(verifier.sample_f128(), next);
            for claim in &claims {
                assert_eq!(claim.value, bit_eval(&packed, &claim.point));
            }
        }
    }
}

#[test]
fn bit_wiring_binds_public_sigma_and_outer_commitment() {
    let sigma: Vec<_> = (0..256).map(|i| (i + 19) % 256).collect();
    let statement = BitWiringStatement::new(&sigma).unwrap();
    let packed = [F128::ZERO; 2];
    let prepared = wiring::prepare_bits(&packed, &statement).unwrap();
    let (proof, _, _, _) = wiring::prove_bits(
        &statement,
        &prepared,
        Options::default(),
        Engine::Fused,
        &mut ch(),
    )
    .unwrap();
    let other = BitWiringStatement::new(&(0..256).collect::<Vec<_>>()).unwrap();
    assert!(wiring::verify_bits(&other, &proof, &mut ch()).is_err());
    assert!(
        wiring::prove_bits(
            &other,
            &prepared,
            Options::default(),
            Engine::Fused,
            &mut ch()
        )
        .is_err()
    );
    let mut wrong_context = ch();
    wrong_context.observe_bytes(&[0x99; 32]);
    assert!(wiring::verify_bits(&statement, &proof, &mut wrong_context).is_err());
    let mut bad = proof;
    bad.left.layers[0].left += F128::ONE;
    assert!(wiring::verify_bits(&statement, &bad, &mut ch()).is_err());
}

#[test]
fn bit_wiring_rejects_changed_witness_and_invalid_public_shape() {
    let statement =
        BitWiringStatement::new(&(0..256).map(|i| (i + 1) % 256).collect::<Vec<_>>()).unwrap();
    let packed = [F128::new(0, 1 << 63), F128::ZERO];
    let prepared = wiring::prepare_bits(&packed, &statement).unwrap();
    assert_eq!(
        wiring::prove_bits(
            &statement,
            &prepared,
            Options::default(),
            Engine::Fused,
            &mut ch()
        )
        .unwrap_err(),
        Error::ProductMismatch
    );
    for sigma in [vec![], vec![0, 0], vec![1, 2], vec![0, 1, 2]] {
        assert!(BitWiringStatement::new(&sigma).is_err());
        assert!(WordWiringStatement::new(&sigma).is_err());
    }
    let small = BitWiringStatement::new(&[0, 1]).unwrap();
    assert!(wiring::prepare_bits(&[F128::new(0, 1)], &small).is_err());
    assert!(wiring::prepare_bits(&[], &small).is_err());
}

#[test]
fn query_adapter_rejects_unexpected_basis_offsets_and_source_order() {
    let statement = BitWiringStatement::new(&(0..128).collect::<Vec<_>>()).unwrap();
    let prepared = wiring::prepare_bits(&[F128::new(19, 58)], &statement).unwrap();
    let (_, claims, _, _) = permutation::prove_committed(
        prepared.permutation(),
        Options::default(),
        Engine::Fused,
        &mut ch(),
    )
    .unwrap();
    assert!(wiring::reduce_bit_claims(&statement, &claims).is_ok());
    let mut changed = claims.clone();
    let LinearQuery::Rows { row_offset, .. } = &mut changed.evaluations[0].query else {
        unreachable!()
    };
    *row_offset = 1;
    assert!(wiring::reduce_bit_claims(&statement, &changed).is_err());
    let mut changed = claims.clone();
    let LinearQuery::Rows { basis, .. } = &mut changed.evaluations[1].query else {
        unreachable!()
    };
    basis[0] += F128::ONE;
    assert!(wiring::reduce_bit_claims(&statement, &changed).is_err());
    let mut changed = claims;
    changed.evaluations.swap(0, 1);
    assert!(wiring::reduce_bit_claims(&statement, &changed).is_err());
}

#[test]
fn offline_coefficients_match_original_online_generation_at_capacity_boundaries() {
    for n in [0, 1, 2, 3, 8, 33, 257] {
        for width in [0, 1, 8, 31, 33, 64, 128] {
            let (left, right) = source_pair(n, width, 297);
            let source = permutation::prepare(left, right).unwrap();
            for capacity in [1, 64, 128] {
                for strategy in [Strategy::BitwidthAware, Strategy::FullExtension] {
                    let options = Options {
                        strategy,
                        coefficient_capacity_bits: capacity,
                    };
                    let prepared =
                        permutation::prepare_tree_coefficients(&source, options).unwrap();
                    assert_eq!(prepared.statement(), source.statement());
                    assert_eq!(prepared.options().coefficient_capacity_bits, capacity);
                    assert!(prepared.resident_bytes() > 0);
                    let mut old = ch();
                    let (expected, expected_claims, _, _) =
                        permutation::prove_committed(&source, options, Engine::Fused, &mut old)
                            .unwrap();
                    let bytes = bincode::serialize(&expected).unwrap();
                    let next = old.sample_f128();
                    for engine in [Engine::Reference, Engine::Recycled, Engine::Combined] {
                        let mut prover = ch();
                        let (proof, claims, _, timings) =
                            permutation::prove_prepared(&prepared, engine, &mut prover).unwrap();
                        assert_eq!(
                            bincode::serialize(&proof).unwrap(),
                            bytes,
                            "n={n}, width={width}, capacity={capacity}, strategy={strategy:?}, engine={engine:?}"
                        );
                        assert_eq!(claims, expected_claims);
                        assert_eq!(prover.sample_f128(), next);
                        assert_eq!(timings.decode_ms, 0.0);
                        assert_eq!(timings.compact_build_ms, 0.0);
                        assert_eq!(timings.fused_leaf_build_ms, 0.0);
                        assert_eq!(timings.source_checks_ms, 0.0);
                        assert_eq!(
                            permutation::verify_committed(prepared.statement(), &proof, &mut ch())
                                .unwrap(),
                            claims
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn prepared_bit_and_word_wiring_reuse_offline_coefficients_across_challenges() {
    let bits: Vec<_> = (0..256).map(|i| i % 5 == 0).collect();
    let packed = pack(&bits);
    let mut sigma: Vec<_> = (0..256).collect();
    for value in [false, true] {
        let positions: Vec<_> = (0..256).filter(|&i| bits[i] == value).collect();
        for (j, &i) in positions.iter().enumerate() {
            sigma[i] = positions[(j + 1) % positions.len()];
        }
    }
    let statement = BitWiringStatement::new(&sigma).unwrap();
    let source = wiring::prepare_bits(&packed, &statement).unwrap();
    let prepared = wiring::prepare_bit_coefficients(&source, Options::default()).unwrap();
    assert!(prepared.resident_bytes() > 0);
    for nonce in [0u8, 37, 195] {
        let mut original = ch();
        original.observe_bytes(&[nonce]);
        let mut candidate = ch();
        candidate.observe_bytes(&[nonce]);
        let (reference, expected, _, _) = wiring::prove_bits(
            &statement,
            &source,
            Options::default(),
            Engine::Recycled,
            &mut original,
        )
        .unwrap();
        let (proof, claims, _, _) =
            wiring::prove_bits_prepared(&statement, &prepared, Engine::Recycled, &mut candidate)
                .unwrap();
        assert_eq!(
            bincode::serialize(&proof).unwrap(),
            bincode::serialize(&reference).unwrap()
        );
        assert_eq!(claims, expected);
        assert_eq!(original.sample_f128(), candidate.sample_f128());
        for claim in &claims {
            assert_eq!(claim.value, bit_eval(&packed, &claim.point));
        }
    }
    drop(source);
    let (proof, claims, _, _) =
        wiring::prove_bits_prepared(&statement, &prepared, Engine::Recycled, &mut ch()).unwrap();
    assert_eq!(
        wiring::verify_bits(&statement, &proof, &mut ch()).unwrap(),
        claims
    );

    let words = [F128::new(773, 0x91832742_1298ff00); 2];
    let statement = WordWiringStatement::new(&[1, 0]).unwrap();
    let source = wiring::prepare(&words, &statement).unwrap();
    let prepared = wiring::prepare_bit_coefficients(&source, Options::default()).unwrap();
    let (old, expected, _, _) = wiring::prove(
        &statement,
        &source,
        Options::default(),
        Engine::Recycled,
        &mut ch(),
    )
    .unwrap();
    drop(source);
    let (proof, claims, _, _) =
        wiring::prove_prepared(&statement, &prepared, Engine::Recycled, &mut ch()).unwrap();
    assert_eq!(
        bincode::serialize(&proof).unwrap(),
        bincode::serialize(&old).unwrap()
    );
    assert_eq!(claims, expected);
    assert_eq!(
        wiring::verify(&statement, &proof, &mut ch()).unwrap(),
        claims
    );
}
