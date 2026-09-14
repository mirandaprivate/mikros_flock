use flock_field::F128;
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros_permutation_binary::{
    bits::PackedBits,
    claims::Source,
    field::{FastField, SmallField},
    permutation::{self, Options, Strategy},
    profile::Engine,
};
fn ch() -> FsChallenger {
    FsChallenger::with_hash(b"permutation-engine-golden", HashKind::Blake3)
}
fn word(mut x: u128) -> u128 {
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 43;
    x.wrapping_mul(0x9e3779b97f4a7c15)
}
fn mask(w: usize) -> u128 {
    if w == 128 {
        u128::MAX
    } else {
        (1u128 << w) - 1
    }
}

#[test]
fn pclmul_and_embedding_match_every_reference_subfield() {
    for w in [1, 2, 4, 8, 16, 32, 64, 128] {
        let a = SmallField::for_width(w).unwrap();
        let b = FastField::for_width(w).unwrap();
        for i in 0..500 {
            let x = word(i) & mask(w);
            let y = word(i + 978) & mask(w);
            assert_eq!(a.embed(x), b.embed(x), "embed K={w}");
            assert_eq!(a.multiply(x, y), b.multiply(x, y), "multiply K={w}");
        }
    }
}
#[test]
fn transposed_bytes_and_parallel_digest_match_reference() {
    for n in [0, 1, 3, 8, 16, 31, 64, 1024] {
        for w in [0, 1, 4, 8, 13, 28, 38, 64, 127, 128] {
            let values: Vec<_> = (0..n).map(|i| word(i as u128 + 17) & mask(w)).collect();
            let reference = PackedBits::from_rows(&values, w).unwrap();
            let fast = PackedBits::from_fn_parallel(n, w, |i| values[i]).unwrap();
            assert_eq!(reference, fast, "n={n} w={w}");
            assert_eq!(reference.digest(), fast.digest_parallel());
        }
    }
}
#[test]
fn every_engine_and_tree_strategy_has_identical_proof_and_claims() {
    for n in [0, 1, 2, 3, 4, 16, 257] {
        for w in [1, 8, 28, 33, 38, 63, 64, 128] {
            let values: Vec<_> = (0..n).map(|i| word(i as u128 + 19) & mask(w)).collect();
            let mut other = values.clone();
            other.reverse();
            let left = PackedBits::from_fn_parallel(n, w, |i| values[i]).unwrap();
            let right = PackedBits::from_fn_parallel(n, w, |i| other[i]).unwrap();
            let statement = permutation::Statement::new_parallel(&left, &right).unwrap();
            let old_left = baseline::bits::PackedBits::new(n, w, left.bytes().to_vec()).unwrap();
            let old_right = baseline::bits::PackedBits::new(n, w, right.bytes().to_vec()).unwrap();
            let old_statement =
                baseline::permutation::Statement::new(&old_left, &old_right).unwrap();
            let mut old_ch = ch();
            let (old_proof, old_claims, _) = baseline::permutation::prove(
                &old_statement,
                &old_left,
                &old_right,
                Default::default(),
                &mut old_ch,
            )
            .unwrap();
            let bytes = bincode::serialize(&old_proof).unwrap();
            let claim_bytes = bincode::serialize(&old_claims).unwrap();
            let next = old_ch.sample_f128();
            for engine in [
                Engine::Reference,
                Engine::ParallelTree,
                Engine::LookupTree,
                Engine::Released,
                Engine::Fused,
            ] {
                for strategy in [Strategy::BitwidthAware, Strategy::FullExtension] {
                    let mut prover = ch();
                    let (proof, claims, _, _) = permutation::prove_profiled(
                        &statement,
                        &left,
                        &right,
                        Options {
                            strategy,
                            ..Default::default()
                        },
                        engine,
                        &mut prover,
                    )
                    .unwrap();
                    assert_eq!(
                        bincode::serialize(&proof).unwrap(),
                        bytes,
                        "n={n},w={w},engine={engine:?},strategy={strategy:?}"
                    );
                    assert_eq!(bincode::serialize(&claims).unwrap(), claim_bytes);
                    assert_eq!(prover.sample_f128(), next);
                    let verified = permutation::verify(&statement, &proof, &mut ch()).unwrap();
                    assert_eq!(claims, verified);
                    claims
                        .check(&[(Source::Left, &left), (Source::Right, &right)])
                        .unwrap();
                    claims
                        .check_parallel(&[(Source::Left, &left), (Source::Right, &right)])
                        .unwrap();
                }
            }
        }
    }
}

#[test]
fn binary_fused_capacity_boundaries_match_separately_compiled_v3() {
    for n in [2, 3, 4, 7, 8, 9, 16, 17, 65] {
        for width in [17, 27, 31, 32, 33, 38, 64, 65] {
            let a = PackedBits::from_fn_parallel(n, width, |i| word(i as u128 + 43) & mask(width))
                .unwrap();
            let b = PackedBits::from_fn_parallel(n, width, |i| {
                word((n - 1 - i) as u128 + 43) & mask(width)
            })
            .unwrap();
            let st = permutation::Statement::new_parallel(&a, &b).unwrap();
            let va = previous::bits::PackedBits::new(n, width, a.bytes().to_vec()).unwrap();
            let vb = previous::bits::PackedBits::new(n, width, b.bytes().to_vec()).unwrap();
            let vst = previous::permutation::Statement::new_parallel(&va, &vb).unwrap();
            for cap in [1, 63, 64, 127, 128] {
                let mut c = ch();
                let (proof, claims, stats, _) = permutation::prove_profiled(
                    &st,
                    &a,
                    &b,
                    Options {
                        coefficient_capacity_bits: cap,
                        ..Default::default()
                    },
                    Engine::Fused,
                    &mut c,
                )
                .unwrap();
                if (17..=32).contains(&width) && cap == 128 && n >= 3 {
                    for tree in [&stats.left, &stats.right] {
                        assert_eq!(tree.switch_level, 2);
                        assert_eq!(tree.max_coefficient_bits, 64);
                        assert_eq!(tree.compact_bytes, n.next_power_of_two() * 8);
                    }
                }
                let mut vc = ch();
                let (vproof, vclaims, _, _) = previous::permutation::prove_profiled(
                    &vst,
                    &va,
                    &vb,
                    previous::permutation::Options {
                        coefficient_capacity_bits: cap,
                        ..Default::default()
                    },
                    previous::profile::Engine::Fused,
                    &mut vc,
                )
                .unwrap();
                assert_eq!(
                    bincode::serialize(&proof).unwrap(),
                    bincode::serialize(&vproof).unwrap(),
                    "n={n}, width={width}, cap={cap}"
                );
                assert_eq!(
                    bincode::serialize(&claims).unwrap(),
                    bincode::serialize(&vclaims).unwrap()
                );
                assert_eq!(c.sample_f128(), vc.sample_f128());
                let verified = permutation::verify(&st, &proof, &mut ch()).unwrap();
                assert_eq!(claims, verified);
                claims
                    .check_parallel(&[(Source::Left, &a), (Source::Right, &b)])
                    .unwrap();
            }
        }
    }
}

#[test]
fn binary_fused_parallel_k32_matches_v3_proof_claims_and_transcript() {
    // A partial final decode block plus enough work for both the fused tree
    // construction and the sumcheck's parallel scheduling threshold.
    let n = (1usize << 16) + 3;
    let width = 31;
    let values: Vec<_> = (0..n)
        .map(|i| word(i as u128 + 917) & mask(width))
        .collect();
    let mut reverse = values.clone();
    reverse.reverse();
    let a = PackedBits::from_rows(&values, width).unwrap();
    let b = PackedBits::from_rows(&reverse, width).unwrap();
    let st = permutation::Statement::new_parallel(&a, &b).unwrap();
    let va = previous::bits::PackedBits::new(n, width, a.bytes().to_vec()).unwrap();
    let vb = previous::bits::PackedBits::new(n, width, b.bytes().to_vec()).unwrap();
    let vst = previous::permutation::Statement::new_parallel(&va, &vb).unwrap();
    let mut old_ch = ch();
    let (old_proof, old_claims, _) =
        previous::permutation::prove(&vst, &va, &vb, Default::default(), &mut old_ch).unwrap();
    let proof_bytes = bincode::serialize(&old_proof).unwrap();
    let claim_bytes = bincode::serialize(&old_claims).unwrap();
    let next = old_ch.sample_f128();
    for threads in [1, 4] {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap()
            .install(|| {
                let mut c = ch();
                let (proof, claims, _) =
                    permutation::prove(&st, &a, &b, Default::default(), &mut c).unwrap();
                assert_eq!(bincode::serialize(&proof).unwrap(), proof_bytes);
                assert_eq!(bincode::serialize(&claims).unwrap(), claim_bytes);
                assert_eq!(c.sample_f128(), next);
                assert_eq!(permutation::verify(&st, &proof, &mut ch()).unwrap(), claims);
                claims
                    .check_parallel(&[(Source::Left, &a), (Source::Right, &b)])
                    .unwrap();
            });
    }
}
#[test]
fn parallel_rounds_and_source_authentication_reject_tampering() {
    let n = 1 << 16;
    let w = 38;
    let left = PackedBits::from_fn_parallel(n, w, |i| word(i as u128) & mask(w)).unwrap();
    let right =
        PackedBits::from_fn_parallel(n, w, |i| word((n - 1 - i) as u128) & mask(w)).unwrap();
    let statement = permutation::Statement::new_parallel(&left, &right).unwrap();
    let (mut proof, claims, _) =
        permutation::prove(&statement, &left, &right, Options::default(), &mut ch()).unwrap();
    let (old, old_claims, _, _) = permutation::prove_profiled(
        &statement,
        &left,
        &right,
        Options::default(),
        Engine::Reference,
        &mut ch(),
    )
    .unwrap();
    assert_eq!(proof, old);
    assert_eq!(claims, old_claims);
    claims
        .check_parallel(&[(Source::Left, &left), (Source::Right, &right)])
        .unwrap();
    let mut bad = claims.clone();
    bad.evaluations[0].value += F128::ONE;
    assert!(
        bad.check_parallel(&[(Source::Left, &left), (Source::Right, &right)])
            .is_err()
    );
    let altered =
        PackedBits::from_fn_parallel(n, w, |i| (word(i as u128) & mask(w)) ^ u128::from(i == 7))
            .unwrap();
    assert!(
        claims
            .check_parallel(&[(Source::Left, &altered), (Source::Right, &right)])
            .is_err()
    );
    proof.left.layers[1].rounds[0][0] += F128::ONE;
    assert!(permutation::verify(&statement, &proof, &mut ch()).is_err());
}
