//! Regression suite for the historical mixed-tensor protocol.
use flock_field::{F128, test_rng::Rng};
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros::legacy as mikros;
use mikros::{Claims, Error, Kernel, Proof, check_claims, prove, verify};

fn witness(log_n: usize, seed: u64) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut rng = Rng::new(seed);
    let mut a = vec![0; 1 << (log_n - 3)];
    let mut b = a.clone();
    rng.fill_bytes(&mut a);
    rng.fill_bytes(&mut b);
    let c = a.iter().zip(&b).map(|(a, b)| a & b).collect();
    (a, b, c)
}

fn challenger(seed: u64) -> FsChallenger {
    let mut ch = FsChallenger::with_hash(b"mikros-integration-test-v1", HashKind::Blake3);
    ch.observe_bytes(&seed.to_le_bytes());
    ch
}

fn valid(
    a: &[u8],
    b: &[u8],
    c: &[u8],
    log_n: usize,
    proof: &Proof,
    seed: u64,
) -> Result<Claims, Error> {
    let claims = verify(proof, log_n, &mut challenger(seed))?;
    check_claims(a, b, c, log_n, &claims)?;
    Ok(claims)
}

#[test]
fn reference_and_lookup_are_message_identical_and_source_verified() {
    for log_n in [7, 8, 10, 13, 16] {
        let (a, b, c) = witness(log_n, log_n as u64 + 90210);
        let (reference, expected) =
            prove(&a, &b, &c, log_n, Kernel::Reference, &mut challenger(17)).unwrap();
        let (lookup, actual) =
            prove(&a, &b, &c, log_n, Kernel::Lookup, &mut challenger(17)).unwrap();
        assert_eq!(reference, lookup);
        assert_eq!(expected, actual);
        let (packed, packed_claims) =
            prove(&a, &b, &c, log_n, Kernel::Packed, &mut challenger(17)).unwrap();
        assert_eq!(packed, lookup);
        assert_eq!(packed_claims, actual);
        let (packed8, claims8) =
            prove(&a, &b, &c, log_n, Kernel::Packed8, &mut challenger(17)).unwrap();
        assert_eq!(packed8, packed);
        assert_eq!(claims8, packed_claims);
        assert_eq!(valid(&a, &b, &c, log_n, &lookup, 17).unwrap(), actual);
        assert_eq!(lookup.field_elements(), 2190 + 2 * (log_n - 7));
        let bytes = bincode::serialize(&lookup).unwrap();
        let decoded: Proof = bincode::deserialize(&bytes).unwrap();
        assert_eq!(decoded, lookup);
        assert_eq!(bytes.len(), 16 * lookup.field_elements() + 24);
    }
}

#[test]
fn profiling_preserves_proof_claims_and_transcript() {
    for kernel in [Kernel::Lookup, Kernel::Packed, Kernel::Packed8] {
        for log_n in [7, 13, 16] {
            let (a, b, c) = witness(log_n, 0xFA5E);
            let mut normal_ch = challenger(71);
            let mut profiled_ch = challenger(71);
            let expected = prove(&a, &b, &c, log_n, kernel, &mut normal_ch).unwrap();
            let (proof, claims, phases) =
                mikros::hadamard::prove_profiled(&a, &b, &c, log_n, kernel, &mut profiled_ch)
                    .unwrap();
            assert_eq!(expected, (proof.clone(), claims.clone()));
            assert_eq!(normal_ch.sample_f128(), profiled_ch.sample_f128());
            assert_eq!(valid(&a, &b, &c, log_n, &proof, 71).unwrap(), claims);
            assert!(phases.tensor_aggregate_ms >= 0.0);
        }
    }
}

#[test]
fn structured_and_boundary_bit_inputs_roundtrip() {
    let log_n = 10;
    let bytes = 1 << (log_n - 3);
    for (left, right) in [(0, 0), (0, 255), (255, 255), (0x55, 0xaa), (0x55, 0xff)] {
        let (a, b, c) = (
            vec![left; bytes],
            vec![right; bytes],
            vec![left & right; bytes],
        );
        let (proof, _) = prove(&a, &b, &c, log_n, Kernel::Lookup, &mut challenger(19)).unwrap();
        valid(&a, &b, &c, log_n, &proof, 19).unwrap();
    }
    for bit in [0, 7, 8, 63, 64, 127, 128, 255, 512, 1023] {
        let mut a = vec![0; bytes];
        a[bit / 8] = 1 << (bit % 8);
        let b = vec![255; bytes];
        let (proof, _) = prove(&a, &b, &a, log_n, Kernel::Lookup, &mut challenger(23)).unwrap();
        valid(&a, &b, &a, log_n, &proof, 23).unwrap();
    }
}

#[test]
fn false_c_is_rejected_even_when_prover_constructs_a_proof() {
    let (a, b, mut c) = witness(13, 0xBAD_C);
    for bit in [0, 7, 8, 63, 64, 127, 128, 8191] {
        c[bit / 8] ^= 1 << (bit % 8);
        for kernel in [Kernel::Reference, Kernel::Lookup, Kernel::Packed] {
            let (proof, _) = prove(&a, &b, &c, 13, kernel, &mut challenger(29)).unwrap();
            assert!(valid(&a, &b, &c, 13, &proof, 29).is_err());
        }
        c[bit / 8] ^= 1 << (bit % 8);
    }
}

#[test]
fn every_message_class_is_bound_to_the_transcript() {
    let (a, b, c) = witness(13, 45);
    let (proof, _) = prove(&a, &b, &c, 13, Kernel::Lookup, &mut challenger(31)).unwrap();
    let mut changed = proof.clone();
    changed.projected_c += F128::ONE;
    assert!(valid(&a, &b, &c, 13, &changed, 31).is_err());
    for index in [0, 1, 63, 64, 127] {
        let mut changed = proof.clone();
        changed.column_claims[index] += F128::ONE;
        assert!(valid(&a, &b, &c, 13, &changed, 31).is_err());
    }
    for index in [0, 1, 64, 128, 729, 1215, 1539, 1755, 1899, 1995, 2058] {
        let mut changed = proof.clone();
        changed.quotients[index] += F128::ONE;
        assert!(valid(&a, &b, &c, 13, &changed, 31).is_err());
    }
    for round in 0..proof.inner_product.rounds.len() {
        for coefficient in 0..2 {
            let mut changed = proof.clone();
            changed.inner_product.rounds[round][coefficient] += F128::ONE;
            assert!(valid(&a, &b, &c, 13, &changed, 31).is_err());
        }
    }
    let mut changed = proof.clone();
    changed.inner_product.final_a += F128::ONE;
    assert!(valid(&a, &b, &c, 13, &changed, 31).is_err());
    changed = proof.clone();
    changed.inner_product.final_b += F128::ONE;
    assert!(valid(&a, &b, &c, 13, &changed, 31).is_err());
    assert!(valid(&a, &b, &c, 13, &proof, 32).is_err());
    assert!(verify(&proof, 13, &mut FsChallenger::new(b"wrong-protocol-domain")).is_err());
}

#[test]
fn product_preserving_tampering_requires_and_fails_source_authentication() {
    let (a, b, c) = witness(13, 46);
    let (mut proof, _) = prove(&a, &b, &c, 13, Kernel::Lookup, &mut challenger(37)).unwrap();
    let scale = F128::generator();
    assert!(!proof.inner_product.final_a.is_zero());
    assert!(!proof.inner_product.final_b.is_zero());
    proof.inner_product.final_a *= scale;
    proof.inner_product.final_b *= scale.inv();
    let claims = verify(&proof, 13, &mut challenger(37)).unwrap();
    assert!(check_claims(&a, &b, &c, 13, &claims).is_err());
}

#[test]
fn source_authentication_checks_all_three_original_columns() {
    let (mut a, mut b, mut c) = witness(10, 47);
    let (proof, _) = prove(&a, &b, &c, 10, Kernel::Lookup, &mut challenger(41)).unwrap();
    let claims = verify(&proof, 10, &mut challenger(41)).unwrap();
    for bit in [0, 7, 8, 63, 64, 127, 128, 1023] {
        a[bit / 8] ^= 1 << (bit % 8);
        assert_eq!(
            check_claims(&a, &b, &c, 10, &claims),
            Err(Error::SourceMismatch("a"))
        );
        a[bit / 8] ^= 1 << (bit % 8);
        b[bit / 8] ^= 1 << (bit % 8);
        assert_eq!(
            check_claims(&a, &b, &c, 10, &claims),
            Err(Error::SourceMismatch("b"))
        );
        b[bit / 8] ^= 1 << (bit % 8);
        c[bit / 8] ^= 1 << (bit % 8);
        assert_eq!(
            check_claims(&a, &b, &c, 10, &claims),
            Err(Error::SourceMismatch("c"))
        );
        c[bit / 8] ^= 1 << (bit % 8);
    }
}

#[test]
fn weighted_b_query_is_not_an_mle_at_coordinatewise_products() {
    let (a, b, c) = witness(10, 48);
    let (proof, _) = prove(&a, &b, &c, 10, Kernel::Lookup, &mut challenger(43)).unwrap();
    let mut claims = verify(&proof, 10, &mut challenger(43)).unwrap();
    check_claims(&a, &b, &c, 10, &claims).unwrap();
    for factor in &mut claims.b.row_factors {
        assert_ne!(factor[0] + factor[1], F128::ONE);
        factor[0] = F128::ONE + factor[1];
    }
    assert_eq!(
        check_claims(&a, &b, &c, 10, &claims),
        Err(Error::SourceMismatch("b"))
    );
}

#[test]
fn malformed_shapes_and_dimensions_return_errors() {
    let (a, b, c) = witness(10, 49);
    for log_n in [0, 6, usize::BITS as usize, usize::MAX] {
        assert!(prove(&a, &b, &c, log_n, Kernel::Lookup, &mut challenger(47)).is_err());
    }
    assert!(
        prove(
            &a[..a.len() - 1],
            &b,
            &c,
            10,
            Kernel::Lookup,
            &mut challenger(47)
        )
        .is_err()
    );
    let (proof, mut claims) = prove(&a, &b, &c, 10, Kernel::Lookup, &mut challenger(47)).unwrap();
    for extra in [false, true] {
        for field in 0..3 {
            let mut changed = proof.clone();
            match field {
                0 => {
                    if extra {
                        changed.column_claims.push(F128::ZERO);
                    } else {
                        changed.column_claims.pop();
                    }
                }
                1 => {
                    if extra {
                        changed.quotients.push(F128::ZERO);
                    } else {
                        changed.quotients.pop();
                    }
                }
                _ => {
                    if extra {
                        changed.inner_product.rounds.push([F128::ZERO; 2]);
                    } else {
                        changed.inner_product.rounds.pop();
                    }
                }
            }
            assert!(matches!(
                verify(&changed, 10, &mut challenger(47)),
                Err(Error::ProofShape(_))
            ));
        }
    }
    assert!(verify(&proof, 11, &mut challenger(47)).is_err());
    claims.b.column_weights.pop();
    assert!(matches!(
        check_claims(&a, &b, &c, 10, &claims),
        Err(Error::ProofShape(_))
    ));
}

#[test]
fn lookup_proofs_are_independent_of_thread_count() {
    let (a, b, c) = witness(16, 50);
    let run = |threads| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap()
            .install(|| prove(&a, &b, &c, 16, Kernel::Lookup, &mut challenger(53)).unwrap())
    };
    assert_eq!(run(1), run(4));
}

#[test]
fn packed_proofs_match_lookup_on_structured_and_parallel_domains() {
    for log_n in [7, 10, 16, 18, 20] {
        let (mut a, mut b, mut c) = witness(log_n, 0xBACED);
        // Include all-zero, all-one and cross-row/byte boundary patterns.
        for mode in 0..4 {
            if mode == 1 {
                a.fill(0);
                b.fill(255);
            }
            if mode == 2 {
                a.fill(255);
                b.fill(255);
            }
            if mode == 3 {
                a.fill(0);
                b.fill(0);
                for bit in [0, 7, 8, 63, 64, 127, 128, 1023] {
                    if bit < (1 << log_n) {
                        a[bit / 8] |= 1 << (bit % 8);
                    }
                }
                b.copy_from_slice(&a);
            }
            for ((c, &a), &b) in c.iter_mut().zip(&a).zip(&b) {
                *c = a & b;
            }
            let expected = prove(&a, &b, &c, log_n, Kernel::Lookup, &mut challenger(79)).unwrap();
            for threads in [1, 3] {
                rayon::ThreadPoolBuilder::new()
                    .num_threads(threads)
                    .build()
                    .unwrap()
                    .install(|| {
                        let mut ch = challenger(79);
                        let actual = prove(&a, &b, &c, log_n, Kernel::Packed, &mut ch).unwrap();
                        assert_eq!(actual, expected);
                        let mut verifier = challenger(79);
                        let checked = verify(&actual.0, log_n, &mut verifier).unwrap();
                        assert_eq!(ch.sample_f128(), verifier.sample_f128());
                        check_claims(&a, &b, &c, log_n, &checked).unwrap();
                    });
            }
        }
    }
}

#[test]
fn all_native_ablations_and_reused_scratch_preserve_protocol_across_new_witnesses() {
    for threads in [1, 3] {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap()
            .install(|| {
                let mut scratch = mikros::ProverScratch::default();
                // Shrinking and growing between proofs catches stale initialized
                // slices and challenge-dependent state accidentally retained in pools.
                for (run, log_n) in [7, 13, 18, 8, 19, 7].into_iter().enumerate() {
                    let seed = 0x5120 + run as u64;
                    let (a, b, c) = witness(log_n, seed);
                    let mut baseline_ch = challenger(seed);
                    let expected =
                        prove(&a, &b, &c, log_n, Kernel::PackedLegacy, &mut baseline_ch).unwrap();
                    let challenge_after = baseline_ch.sample_f128();
                    let expected_bytes = bincode::serialize(&expected.0).unwrap();
                    assert_eq!(
                        valid(&a, &b, &c, log_n, &expected.0, seed).unwrap(),
                        expected.1
                    );
                    for kernel in [
                        Kernel::PackedStream,
                        Kernel::PackedSimdV1,
                        Kernel::PackedSimd,
                        Kernel::PackedWide,
                        Kernel::PackedFused,
                        Kernel::PackedNative,
                        Kernel::PackedFactored,
                        Kernel::Packed,
                    ] {
                        let mut ch = challenger(seed);
                        let actual = prove(&a, &b, &c, log_n, kernel, &mut ch).unwrap();
                        assert_eq!(
                            actual, expected,
                            "kernel={kernel:?}, log_n={log_n}, threads={threads}"
                        );
                        assert_eq!(bincode::serialize(&actual.0).unwrap(), expected_bytes);
                        assert_eq!(ch.sample_f128(), challenge_after);
                    }
                    let mut ch = challenger(seed);
                    let (proof, claims, _) = mikros::hadamard::prove_profiled_with_scratch(
                        &a,
                        &b,
                        &c,
                        log_n,
                        Kernel::PackedNative,
                        &mut ch,
                        &mut scratch,
                    )
                    .unwrap();
                    assert_eq!((proof, claims), expected);
                    assert_eq!(ch.sample_f128(), challenge_after);
                    let mut ch = challenger(seed);
                    let actual = mikros::prove_with_scratch(
                        &a,
                        &b,
                        &c,
                        log_n,
                        Kernel::PackedNative,
                        &mut ch,
                        &mut scratch,
                    )
                    .unwrap();
                    assert_eq!(actual, expected);
                    assert_eq!(ch.sample_f128(), challenge_after);
                }
            });
    }
}
