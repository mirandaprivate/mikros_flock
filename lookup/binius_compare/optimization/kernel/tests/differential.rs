use flock_field::F128;
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros_binius_kernel::{
    batched,
    bits::PackedBits,
    claims::Source,
    field::{FastField, SmallField},
    permutation::{self, Options, PreparedRows, Strategy},
    profile::Engine,
};
fn ch() -> FsChallenger {
    FsChallenger::with_hash(b"optimized-differential", HashKind::Blake3)
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
fn arithmetic_and_bit_transposition_against_frozen_reference() {
    for w in [1, 2, 4, 8, 16, 32, 64, 128] {
        let a = baseline::field::SmallField::for_width(w).unwrap();
        let b = FastField::for_width(w).unwrap();
        for i in 0..1000 {
            let x = word(i + 1) & mask(w);
            let y = word(i + 917) & mask(w);
            assert_eq!(a.multiply(x, y), b.multiply(x, y));
            assert_eq!(a.embed(x), b.embed(x));
        }
    }
    for n in [0, 1, 3, 8, 31, 1024] {
        for w in [0, 1, 8, 28, 38, 64, 127, 128] {
            let values: Vec<_> = (0..n).map(|i| word(i as u128 + 9) & mask(w)).collect();
            let old = baseline::bits::PackedBits::from_rows(&values, w).unwrap();
            let new = PackedBits::from_fn_parallel(n, w, |i| values[i]).unwrap();
            assert_eq!(old.bytes(), new.bytes());
            assert_eq!(old.digest(), new.digest_parallel());
        }
    }
}
#[test]
fn prepared_modes_are_byte_identical_to_frozen_piop() {
    for n in [0, 1, 3, 16, 257] {
        for w in [1, 8, 16, 28, 38, 64, 128] {
            let values: Vec<_> = (0..n).map(|i| word(i as u128 + 17) & mask(w)).collect();
            let mut other = values.clone();
            other.reverse();
            let a = baseline::bits::PackedBits::from_rows(&values, w).unwrap();
            let b = baseline::bits::PackedBits::from_rows(&other, w).unwrap();
            let st = baseline::permutation::Statement::new(&a, &b).unwrap();
            let mut old_ch = ch();
            let (old, claims, _) =
                baseline::permutation::prove(&st, &a, &b, Default::default(), &mut old_ch).unwrap();
            let expected = bincode::serialize(&old).unwrap();
            let end = old_ch.sample_f128();
            let rows = PreparedRows::new(values, other, w).unwrap();
            assert_eq!(
                bincode::serialize(rows.statement()).unwrap(),
                bincode::serialize(&st).unwrap()
            );
            for strategy in [Strategy::BitwidthAware, Strategy::FullExtension] {
                for engine in [Engine::Reference, Engine::LookupTree, Engine::Fused] {
                    let mut c = ch();
                    let (proof, got, _, _) = permutation::prove_prepared(
                        &rows,
                        Options {
                            strategy,
                            ..Default::default()
                        },
                        engine,
                        &mut c,
                    )
                    .unwrap();
                    assert_eq!(
                        bincode::serialize(&proof).unwrap(),
                        expected,
                        "n={n},w={w},{engine:?},{strategy:?}"
                    );
                    assert_eq!(
                        bincode::serialize(&got).unwrap(),
                        bincode::serialize(&claims).unwrap()
                    );
                    assert_eq!(c.sample_f128(), end);
                }
            }
        }
    }
}
#[test]
fn batched_all_claims_authenticated_and_tampering_rejected() {
    for n in [1, 3, 16, 259, 1 << 16] {
        for w in [8, 28, 38, 128] {
            let values: Vec<_> = (0..n).map(|i| word(i as u128 + 11) & mask(w)).collect();
            let mut other = values.clone();
            other.reverse();
            let a = PackedBits::from_fn_parallel(n, w, |i| values[i]).unwrap();
            let b = PackedBits::from_fn_parallel(n, w, |i| other[i]).unwrap();
            let rows = PreparedRows::new(values, other, w).unwrap();
            let mut c = ch();
            let (proof, claims, _, _) = batched::prove(&rows, Options::default(), &mut c).unwrap();
            let bytes = bincode::serialize(&proof).unwrap();
            let proof: batched::Proof = bincode::deserialize(&bytes).unwrap();
            let mut v = ch();
            let got = batched::verify(rows.statement(), &proof, &mut v).unwrap();
            assert_eq!(claims, got);
            assert_eq!(c.sample_f128(), v.sample_f128());
            claims
                .check_parallel(&[(Source::Left, &a), (Source::Right, &b)])
                .unwrap();
            let (full, full_claims, _, _) = batched::prove(
                &rows,
                Options {
                    strategy: Strategy::FullExtension,
                    ..Default::default()
                },
                &mut ch(),
            )
            .unwrap();
            assert_eq!(bincode::serialize(&full).unwrap(), bytes);
            assert_eq!(claims, full_claims);
            for index in 0..proof.layers.len() {
                for side in 0..4 {
                    let mut bad = proof.clone();
                    bad.layers[index].terminal[side] += F128::ONE;
                    assert!(batched::verify(rows.statement(), &bad, &mut ch()).is_err());
                }
                if !proof.layers[index].rounds.is_empty() {
                    for k in 0..2 {
                        let mut bad = proof.clone();
                        bad.layers[index].rounds[0][k] += F128::ONE;
                        assert!(batched::verify(rows.statement(), &bad, &mut ch()).is_err());
                    }
                }
            }
            let mut altered = a.clone();
            if n > 0 {
                let mut data = altered.bytes().to_vec();
                data[0] ^= 1;
                altered = PackedBits::new(n, w, data).unwrap();
                assert!(
                    claims
                        .check_parallel(&[(Source::Left, &altered), (Source::Right, &b)])
                        .is_err()
                );
            }
        }
    }
    assert!(PreparedRows::new(vec![256], vec![256], 8).is_err());
    assert_eq!(SmallField::for_width(28).unwrap().bits, 32);
}
