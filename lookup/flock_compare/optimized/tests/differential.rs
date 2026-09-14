use flock_field::F128;
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros_permutation_optimized::{
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
    for n in [0, 1, 3, 16, 257] {
        for w in [1, 8, 28, 38, 64, 128] {
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
