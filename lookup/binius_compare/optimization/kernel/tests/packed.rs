use flock_field::F128;
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros_binius_kernel::{
    bits::PackedBits,
    claims::Source,
    packed::{self, Layer},
    permutation::PreparedRows,
};
fn ch() -> FsChallenger {
    FsChallenger::with_hash(b"packed-golden", HashKind::Blake3)
}

#[test]
fn packed_tree_reaches_both_original_oracles_for_every_subfield() {
    for n in [1, 3, 16, 257] {
        for w in [1, 2, 4, 8, 16, 28, 38, 64, 128] {
            let mask = if w == 128 {
                u128::MAX
            } else {
                (1u128 << w) - 1
            };
            let a: Vec<_> = (0..n)
                .map(|i| ((i as u128 * 0x821acd871fe98) ^ ((i as u128 * 3) << 40)) & mask)
                .collect();
            let mut b = a.clone();
            b.reverse();
            let left = PackedBits::from_fn_parallel(n, w, |i| a[i]).unwrap();
            let right = PackedBits::from_fn_parallel(n, w, |i| b[i]).unwrap();
            let rows = PreparedRows::new(a, b, w).unwrap();
            let mut c = ch();
            let (proof, claims, _, _) =
                packed::prove(&rows, &mut c).unwrap_or_else(|e| panic!("n={n},w={w}: {e}"));
            let bytes = bincode::serialize(&proof).unwrap();
            let proof: packed::Proof = bincode::deserialize(&bytes).unwrap();
            let mut v = ch();
            let verified = packed::verify(rows.statement(), &proof, &mut v).unwrap();
            assert_eq!(claims, verified);
            assert_eq!(c.sample_f128(), v.sample_f128());
            claims
                .check_parallel(&[(Source::Left, &left), (Source::Right, &right)])
                .unwrap();
            for i in 0..proof.layers.len() {
                if let Layer::Packed(layer) = &proof.layers[i] {
                    for j in 0..layer.columns.len() {
                        let mut bad = proof.clone();
                        if let Layer::Packed(p) = &mut bad.layers[i] {
                            p.columns[j] += F128::ONE;
                        }
                        assert!(packed::verify(rows.statement(), &bad, &mut ch()).is_err());
                    }
                    for j in 0..layer.quotient.len() {
                        let mut bad = proof.clone();
                        if let Layer::Packed(p) = &mut bad.layers[i] {
                            p.quotient[j] += F128::ONE;
                        }
                        assert!(packed::verify(rows.statement(), &bad, &mut ch()).is_err());
                    }
                    for s in 0..4 {
                        for j in 0..layer.columns.len() {
                            let mut bad = proof.clone();
                            if let Layer::Packed(p) = &mut bad.layers[i] {
                                p.terminal_columns[s][j] += F128::ONE;
                            }
                            assert!(packed::verify(rows.statement(), &bad, &mut ch()).is_err());
                        }
                    }
                    let mut bad = proof.clone();
                    if let Layer::Packed(p) = &mut bad.layers[i] {
                        p.quotient.pop();
                    }
                    assert!(packed::verify(rows.statement(), &bad, &mut ch()).is_err());
                    let mut bad = proof.clone();
                    bad.layers[i] = Layer::Dense(layer.inner.clone());
                    assert!(packed::verify(rows.statement(), &bad, &mut ch()).is_err());
                }
            }
        }
    }
}
