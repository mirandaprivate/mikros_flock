use flock_field::F128;
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros_lookup::{
    Error,
    bits::PackedBits,
    claims::Source,
    field::{SmallField, from_raw},
    lasso,
    permutation::{self, Options, Strategy},
};

fn transcript() -> FsChallenger {
    FsChallenger::with_hash(b"lookup-tests", HashKind::Blake3)
}
fn full() -> Options {
    Options {
        strategy: Strategy::FullExtension,
        ..Default::default()
    }
}
fn bits(values: &[u128], width: usize) -> PackedBits {
    PackedBits::from_rows(values, width).unwrap()
}
fn random(state: &mut u128) -> u128 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

#[test]
fn actual_subfields_preserve_multiplication_and_frobenius() {
    let mut seed = 0x87398123_u128;
    for k in [1, 2, 4, 8, 16, 32, 64, 128] {
        let field = SmallField::for_width(k).unwrap();
        assert_eq!(field.bits, k);
        for _ in 0..40 {
            let a = random(&mut seed) & field.mask();
            let b = random(&mut seed) & field.mask();
            assert_eq!(
                field.embed(field.multiply(a, b)),
                field.embed(a) * field.embed(b),
                "K degree {k}"
            );
            assert_eq!(field.embed(a ^ b), field.embed(a) + field.embed(b));
            let mut x = field.embed(a);
            for _ in 0..k {
                x = x.square();
            }
            assert_eq!(x, field.embed(a));
        }
    }
    assert_eq!(SmallField::for_width(3).unwrap().bits, 4);
    assert!(matches!(SmallField::for_width(129), Err(Error::Capacity)));
}

#[test]
fn canonical_column_major_storage() {
    for width in [0, 1, 3, 7, 8, 31, 64, 127, 128] {
        let mask = if width == 128 {
            u128::MAX
        } else {
            (1u128 << width) - 1
        };
        let values = [0, 1 & mask, 17 & mask, mask, 3 & mask];
        let packed = bits(&values, width);
        for (i, v) in values.into_iter().enumerate() {
            assert_eq!(packed.row(i), v);
        }
        assert_eq!(
            bincode::deserialize::<PackedBits>(&bincode::serialize(&packed).unwrap()).unwrap(),
            packed
        );
    }
    assert!(PackedBits::new(1, 1, vec![0x81]).is_err());
    assert!(PackedBits::new(8, 1, vec![]).is_err());
    assert!(PackedBits::from_rows(&[1], 0).is_err());
    assert!(PackedBits::from_rows(&[4], 2).is_err());
}

fn permutation_case(values: &[u128], width: usize) {
    let left = bits(values, width);
    let mut shuffled = values.to_vec();
    shuffled.reverse();
    let right = bits(&shuffled, width);
    let s = permutation::Statement::new(&left, &right).unwrap();
    let mut p = transcript();
    let (proof, claims, stats) =
        permutation::prove(&s, &left, &right, Options::default(), &mut p).unwrap();
    let mut v = transcript();
    let verified = permutation::verify(&s, &proof, &mut v).unwrap();
    assert_eq!(claims, verified);
    assert_eq!(p.sample_f128(), v.sample_f128());
    verified
        .check(&[(Source::Left, &left), (Source::Right, &right)])
        .unwrap();
    let (reference, reference_claims, _) =
        permutation::prove(&s, &left, &right, full(), &mut transcript()).unwrap();
    assert_eq!(proof, reference);
    assert_eq!(claims, reference_claims);
    assert!(stats.left.max_coefficient_bits <= 128);
    assert!(stats.left.subfield_bits <= 128);
    for cap in [1, 7, 8, 16, 63, 64, 127, 128] {
        let options = Options {
            coefficient_capacity_bits: cap,
            ..Default::default()
        };
        let (alternate, alt_claims, st) =
            permutation::prove(&s, &left, &right, options, &mut transcript()).unwrap();
        assert_eq!(proof, alternate, "width {width}, cap {cap}");
        assert_eq!(claims, alt_claims);
        assert!(st.left.max_coefficient_bits <= cap);
    }
}
#[test]
fn permutation_packing_matches_extension_at_all_boundaries() {
    let mut seed = 13;
    for width in [0, 1, 2, 3, 4, 8, 16, 31, 64, 127, 128] {
        let mask = if width == 128 {
            u128::MAX
        } else {
            (1u128 << width) - 1
        };
        for n in [0, 1, 2, 3, 9, 17] {
            let data: Vec<_> = (0..n).map(|_| random(&mut seed) & mask).collect();
            permutation_case(&data, width);
        }
    }
    permutation_case(&(0..257).map(|i| (i % 2) as u128).collect::<Vec<_>>(), 1);
}
#[test]
fn permutation_rejects_wrong_multiplicity_and_shape() {
    let left = bits(&[1, 1, 2], 2);
    let right = bits(&[1, 2, 2], 2);
    let s = permutation::Statement::new(&left, &right).unwrap();
    assert!(permutation::prove(&s, &left, &right, Options::default(), &mut transcript()).is_err());
    assert!(permutation::Statement::new(&left, &bits(&[1], 2)).is_err());
    assert!(permutation::Statement::new(&left, &bits(&[1, 1, 2], 3)).is_err());
    let s = permutation::Statement::new(&left, &left).unwrap();
    assert!(
        permutation::prove(
            &s,
            &left,
            &left,
            Options {
                coefficient_capacity_bits: 129,
                ..Default::default()
            },
            &mut transcript()
        )
        .is_err()
    );
}
#[test]
fn every_permutation_message_is_bound() {
    let left = bits(&[0, 1, 2, 3, 3, 1, 1, 2], 2);
    let s = permutation::Statement::new(&left, &left).unwrap();
    let (proof, _, _) =
        permutation::prove(&s, &left, &left, Options::default(), &mut transcript()).unwrap();
    let rejects = |candidate: &permutation::Proof| {
        permutation::verify(&s, candidate, &mut transcript())
            .and_then(|c| c.check(&[(Source::Left, &left), (Source::Right, &left)]))
            .is_err()
    };
    for side in 0..2 {
        let mut candidate = proof.clone();
        candidate.roots[side] += F128::ONE;
        assert!(rejects(&candidate));
        for layer in 0..proof.left.layers.len() {
            for terminal in 0..2 {
                let mut candidate = proof.clone();
                let tree = if side == 0 {
                    &mut candidate.left
                } else {
                    &mut candidate.right
                };
                if terminal == 0 {
                    tree.layers[layer].left += F128::ONE;
                } else {
                    tree.layers[layer].right += F128::ONE;
                }
                assert!(rejects(&candidate));
            }
            for round in 0..layer {
                for coefficient in 0..3 {
                    let mut candidate = proof.clone();
                    let tree = if side == 0 {
                        &mut candidate.left
                    } else {
                        &mut candidate.right
                    };
                    tree.layers[layer].rounds[round][coefficient] += F128::ONE;
                    assert!(rejects(&candidate));
                }
            }
        }
    }
    let mut candidate = proof.clone();
    candidate.left.layers.pop();
    assert!(rejects(&candidate));
    let mut candidate = proof.clone();
    candidate.right.layers[0].rounds.push([F128::ZERO; 3]);
    assert!(rejects(&candidate));
    let mut wrong = s.clone();
    wrong.left.digest[0] ^= 1;
    assert!(
        permutation::verify(&wrong, &proof, &mut transcript())
            .and_then(|c| c.check(&[(Source::Left, &left), (Source::Right, &left)]))
            .is_err()
    );
}

fn lasso_case(table: &[u128], width: usize, addresses: &[u128]) -> lasso::ProverOutput {
    let t = bits(table, width);
    let values = bits(
        &addresses
            .iter()
            .map(|&a| table[a as usize])
            .collect::<Vec<_>>(),
        width,
    );
    let a = bits(addresses, mikros_lookup::bits::bits_for(table.len() - 1));
    let s = lasso::Statement::new(t, &values, &a).unwrap();
    let mut p = transcript();
    let result = lasso::prove(&s, &values, &a, Options::default(), &mut p).unwrap();
    let mut v = transcript();
    let claims = lasso::verify(&s, &result.proof, &mut v).unwrap();
    assert_eq!(claims, result.claims);
    assert_eq!(p.sample_f128(), v.sample_f128());
    claims
        .check(&result.auxiliary.sources(&values, &a))
        .unwrap();
    let reference = lasso::prove(&s, &values, &a, full(), &mut transcript()).unwrap();
    assert_eq!(result.proof, reference.proof);
    assert_eq!(result.claims, reference.claims);
    let encoded = bincode::serialize(&result.proof).unwrap();
    let decoded: lasso::Proof = bincode::deserialize(&encoded).unwrap();
    assert_eq!(decoded, result.proof);
    result
}
#[test]
fn lasso_duplicates_unused_addresses_and_empty_queries() {
    lasso_case(&[5, 5, 7, 0, 5], 3, &[0, 1, 0, 4, 2, 4, 4, 0, 1]);
    lasso_case(&[0], 0, &[0, 0, 0, 0, 0, 0, 0, 0, 0]);
    lasso_case(&[7], 3, &[0]);
    lasso_case(&[5, 5, 7], 3, &[]);
    lasso_case(&[0], 0, &[]);
    lasso_case(&[3, 2], 2, &[1; 33]);
}
#[test]
fn lasso_exact_127_128_and_randomized_129_bits() {
    for width in [124, 125, 126, 127, 128] {
        let top = 1u128 << (width - 1);
        let result = lasso_case(&[top, top + 1], width, &[1, 0, 1]);
        // address=1 bit, counter=2 bits.
        assert_eq!(result.stats.tuple_bits, width + 3);
        assert_eq!(result.stats.randomized_tuple, width + 3 > 128);
        assert_eq!(
            result.stats.aggregation_challenges,
            if width + 3 > 128 { 2 } else { 0 }
        );
        assert!(result.stats.trees.left.subfield_bits <= 128);
    }
}
#[test]
fn lasso_rejects_invalid_lookup_and_addresses() {
    let t = bits(&[2, 3, 4], 3);
    let a = bits(&[0, 1], 2);
    let bad = bits(&[2, 4], 3);
    let s = lasso::Statement::new(t.clone(), &bad, &a).unwrap();
    assert!(lasso::prove(&s, &bad, &a, Options::default(), &mut transcript()).is_err());
    let a = bits(&[3], 2);
    let v = bits(&[4], 3);
    let s = lasso::Statement::new(t, &v, &a).unwrap();
    assert!(lasso::prove(&s, &v, &a, Options::default(), &mut transcript()).is_err());
    assert!(lasso::Statement::new(bits(&[], 3), &bits(&[], 3), &bits(&[], 0)).is_err());
}
#[test]
fn lasso_rejects_forged_counters_carries_and_final_state() {
    let t = bits(&[2, 2], 2);
    let a = bits(&[0, 0, 0], 1);
    let v = bits(&[2, 2, 2], 2);
    let s = lasso::Statement::new(t, &v, &a).unwrap();
    let aux = lasso::Auxiliary::build(&s, &v, &a).unwrap();
    for which in 0..4 {
        let mut forged = aux.clone();
        match which {
            0 => forged.read_counters = bits(&[0, 1, 3], 2), // top carry would wrap
            1 => forged.read_counters = bits(&[0, 0, 2], 2), // old timestamp
            2 => forged.carry = bits(&[1, 1, 0], 1),
            _ => forged.final_counters = bits(&[2, 1], 2),
        }
        assert!(
            lasso::prove_with_auxiliary(&s, &v, &a, forged, Options::default(), &mut transcript())
                .is_err()
        );
    }
}
#[test]
fn lasso_all_parts_and_auxiliary_claims_are_authenticated() {
    let t = bits(&[9, 8, 7], 4);
    let a = bits(&[1, 0, 1, 1, 2], 2);
    let v = bits(&[8, 9, 8, 8, 7], 4);
    let s = lasso::Statement::new(t, &v, &a).unwrap();
    let out = lasso::prove(&s, &v, &a, Options::default(), &mut transcript()).unwrap();
    assert_eq!(out.claims.evaluations.len(), 11);
    let rejects = |candidate: &lasso::Proof| {
        lasso::verify(&s, candidate, &mut transcript())
            .and_then(|c| c.check(&out.auxiliary.sources(&v, &a)))
            .is_err()
    };
    for i in 0..8 {
        let mut p = out.proof.clone();
        p.parts[i] += F128::ONE;
        assert!(rejects(&p));
    }
    // Preserve the sum of parts to exercise source authentication.
    let mut p = out.proof.clone();
    p.parts[0] += F128::ONE;
    p.parts[1] += F128::ONE;
    assert!(rejects(&p));
    for i in 0..3 {
        let mut p = out.proof.clone();
        p.auxiliary_digests[i][0] ^= 1;
        assert!(rejects(&p));
    }
    let mut p = out.proof.clone();
    p.increments = None;
    assert!(rejects(&p));
    let mut p = out.proof.clone();
    p.increments.as_mut().unwrap().projected_c += F128::ONE;
    assert!(rejects(&p));
    let mut p = out.proof.clone();
    p.increments.as_mut().unwrap().inner_product.final_a += F128::ONE;
    assert!(rejects(&p));
    for i in 0..out.claims.evaluations.len() {
        let mut claims = out.claims.clone();
        claims.evaluations[i].value += from_raw(17);
        assert!(claims.check(&out.auxiliary.sources(&v, &a)).is_err());
    }
}

#[derive(Clone)]
struct ZeroChallenges;
impl Challenger for ZeroChallenges {
    fn observe_f128(&mut self, _: F128) {}
    fn sample_f128(&mut self) -> F128 {
        F128::ZERO
    }
    fn fork_from_seed(&self, _: [F128; 2], _: &'static [u8]) -> Self {
        Self
    }
}
#[test]
fn zero_fingerprint_factors_do_not_require_inversion_or_resampling() {
    let left = bits(&[0, 1, 0], 1);
    let right = bits(&[1, 0, 0], 1);
    let s = permutation::Statement::new(&left, &right).unwrap();
    let (p, c, _) =
        permutation::prove(&s, &left, &right, Options::default(), &mut ZeroChallenges).unwrap();
    assert_eq!(p.roots, [F128::ZERO; 2]);
    assert_eq!(c, permutation::verify(&s, &p, &mut ZeroChallenges).unwrap());
    c.check(&[(Source::Left, &left), (Source::Right, &right)])
        .unwrap();
}

#[test]
fn thread_counts_preserve_proof_and_transcript() {
    let values: Vec<_> = (0..8193).map(|i| (i % 2) as u128).collect();
    let left = bits(&values, 1);
    let right = bits(&values.iter().copied().rev().collect::<Vec<_>>(), 1);
    let s = permutation::Statement::new(&left, &right).unwrap();
    let mut reference = None;
    for threads in [1, 3] {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();
        for strategy in [Strategy::BitwidthAware, Strategy::FullExtension] {
            let result = pool.install(|| {
                let mut p = transcript();
                let (proof, claims, _) = permutation::prove(
                    &s,
                    &left,
                    &right,
                    Options {
                        strategy,
                        ..Default::default()
                    },
                    &mut p,
                )
                .unwrap();
                let mut v = transcript();
                let verified = permutation::verify(&s, &proof, &mut v).unwrap();
                assert_eq!(claims, verified);
                claims
                    .check(&[(Source::Left, &left), (Source::Right, &right)])
                    .unwrap();
                let next = p.sample_f128();
                assert_eq!(next, v.sample_f128());
                (proof, claims, next)
            });
            if let Some(expected) = &reference {
                assert_eq!(&result, expected);
            } else {
                reference = Some(result);
            }
        }
    }
}
