#![cfg(feature = "whir")]
use flock_field::F128;
use mikros_lookup::{
    authenticated::*,
    bits::PackedBits,
    permutation::{Options, Strategy},
};
fn bits(v: &[u128], w: usize) -> PackedBits {
    PackedBits::from_rows(v, w).unwrap()
}
#[test]
fn permutation_whir_roundtrips_and_rejects_changes() {
    for (rows, width) in [
        (vec![], 0),
        (vec![0], 0),
        (vec![0, 1, 1, 0, 1], 1),
        (vec![5, 1, 5, 3, 0], 3),
        (vec![u128::MAX, 0, 1, 1 << 127], 128),
    ] {
        let a = bits(&rows, width);
        let mut reversed = rows.clone();
        reversed.reverse();
        let b = bits(&reversed, width);
        let (s, p, _) = prove_permutation(&a, &b, Options::default(), b"permutation-app").unwrap();
        // Serialized public statement + proof suffice: no witness/cache passed.
        let wire = bincode::serialize(&(s.clone(), p.clone())).unwrap();
        let (s, p): (PermutationStatement, PermutationProof) = bincode::deserialize(&wire).unwrap();
        verify_permutation(&s, &p, b"permutation-app").unwrap();
        assert_eq!(p.sources.len(), 2);
        assert!(verify_permutation(&s, &p, b"other-app").is_err());
        let mut changed = s.clone();
        changed.commitments[0].root[0] ^= 1;
        assert!(verify_permutation(&changed, &p, b"permutation-app").is_err());
        let mut downgraded = s.clone();
        downgraded.commitments[0].parameters.security_bits = 1;
        assert!(verify_permutation(&downgraded, &p, b"permutation-app").is_err());
        let mut wrong = p.clone();
        wrong.sources.pop();
        assert!(verify_permutation(&s, &wrong, b"permutation-app").is_err());
        for source in 0..2 {
            let mut wrong = p.clone();
            wrong.sources[source].opening.push(0);
            assert!(verify_permutation(&s, &wrong, b"permutation-app").is_err());
            let mut wrong = p.clone();
            wrong.sources[source].booleanity.left += F128::ONE;
            assert!(verify_permutation(&s, &wrong, b"permutation-app").is_err());
        }
    }
    assert!(
        prove_permutation(
            &bits(&[0, 1], 1),
            &bits(&[1, 1], 1),
            Options::default(),
            b"x"
        )
        .is_err()
    );
}
#[test]
fn lasso_whir_exact_and_randomized_tuples() {
    for width in [3, 125, 126, 128] {
        // N=3, cw=2, aw=1: totals 6,128,129,131 respectively.
        let high = 1u128 << (width - 1);
        let table = bits(&[high, high + 1], width);
        let values = bits(&[high + 1, high, high + 1], width);
        let addresses = bits(&[1, 0, 1], 1);
        let (s, p, stats) =
            prove_lasso(table, &values, &addresses, Options::default(), b"lasso-app").unwrap();
        assert_eq!(stats.randomized_tuple, width + 3 > 128);
        assert_eq!(p.sources.len(), 5);
        let wire = bincode::serialize(&(s.clone(), p.clone())).unwrap();
        let (s, p): (LassoStatement, LassoProof) = bincode::deserialize(&wire).unwrap();
        verify_lasso(&s, &p, b"lasso-app").unwrap();
        assert!(verify_lasso(&s, &p, b"wrong").is_err());
        for source in 0..5 {
            let mut wrong = p.clone();
            wrong.sources[source].opening.push(0);
            assert!(
                verify_lasso(&s, &wrong, b"lasso-app").is_err(),
                "source {source}"
            );
        }
        let mut wrong = p.clone();
        wrong.auxiliary_commitments[0].root[0] ^= 1;
        assert!(verify_lasso(&s, &wrong, b"lasso-app").is_err());
        let mut wrong = p.clone();
        wrong.piop.auxiliary_digests[1][0] ^= 1;
        assert!(verify_lasso(&s, &wrong, b"lasso-app").is_err());
        let mut wrong = p.clone();
        wrong.sources.swap(0, 1);
        assert!(verify_lasso(&s, &wrong, b"lasso-app").is_err());
        let mut wrong = p.clone();
        wrong.piop.parts[0] += F128::ONE;
        assert!(verify_lasso(&s, &wrong, b"lasso-app").is_err());
        let mut wrong = s.clone();
        wrong.piop.table = bits(&[high, high], width);
        assert!(verify_lasso(&wrong, &p, b"lasso-app").is_err());
    }
}
#[test]
fn lasso_whir_empty_singleton_and_repeated_table_values() {
    for (table, values, addresses) in [
        (bits(&[0], 0), bits(&[], 0), bits(&[], 0)),
        (bits(&[1], 1), bits(&[1], 1), bits(&[0], 0)),
        (
            bits(&[3, 3, 3], 2),
            bits(&[3, 3, 3, 3, 3], 2),
            bits(&[2, 0, 1, 2, 2], 2),
        ),
    ] {
        let (s, p, _) =
            prove_lasso(table, &values, &addresses, Options::default(), b"edge").unwrap();
        verify_lasso(&s, &p, b"edge").unwrap();
        let mut wrong = p.clone();
        wrong.sources.push(wrong.sources[0].clone());
        assert!(verify_lasso(&s, &wrong, b"edge").is_err());
    }
    assert!(
        prove_lasso(
            bits(&[1, 2], 2),
            &bits(&[3], 2),
            &bits(&[0], 1),
            Options::default(),
            b"bad"
        )
        .is_err()
    );
}
#[test]
fn compact_and_full_extension_share_whir_verifier() {
    let a = bits(&[0, 7, 1, 0, 2, 3, 7, 1], 3);
    let b = bits(&[1, 7, 3, 2, 0, 1, 7, 0], 3);
    let (s, p, _) = prove_permutation(&a, &b, Options::default(), b"same").unwrap();
    let full = Options {
        strategy: Strategy::FullExtension,
        ..Default::default()
    };
    let (s2, p2, _) = prove_permutation(&a, &b, full, b"same").unwrap();
    assert_eq!(s, s2);
    assert_eq!(p, p2);
    verify_permutation(&s, &p2, b"same").unwrap();
}

#[test]
fn zero_width_does_not_bypass_capacity_guards() {
    let huge = PackedBits::new((1 << 20) + 1, 0, vec![]).unwrap();
    let empty = bits(&[], 0);
    assert!(prove_permutation(&huge, &huge, Options::default(), b"cap").is_err());
    let addresses = bits(&[], 21);
    assert!(prove_lasso(huge, &empty, &addresses, Options::default(), b"cap").is_err());
}
