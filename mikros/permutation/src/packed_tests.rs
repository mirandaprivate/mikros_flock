//! Independent source-oracle and adversarial tests for the revised reduction.
use crate::{
    Error,
    bits::PackedBits,
    claims::row_weight,
    field::SmallField,
    packed::{self, Claim, Proof},
    permutation::{self, CommittedStatement, PreparedPermutation},
};
use flock_field::F128;
use flock_transcript::challenger::{Challenger, FsChallenger};

fn ch() -> FsChallenger {
    let mut c = FsChallenger::new(b"packed-permutation-independent-test-v1");
    c.observe_label(b"bound-outer-source-commitments");
    c.observe_bytes(&[0x71; 32]);
    c
}
fn source(n: usize, width: usize) -> PreparedPermutation {
    let mask = (1u128 << width) - 1;
    let values: Vec<_> = (0..n)
        .map(|i| {
            let x = (i as u128 + 37).wrapping_mul(0xaafd_be95_123c_7854_bc36_7367);
            (x ^ (x >> 21) ^ (x << 13)) & mask
        })
        .collect();
    permutation::prepare(
        PackedBits::from_fn_parallel(n, width, |i| values[i]).unwrap(),
        PackedBits::from_fn_parallel(n, width, |i| values[n - 1 - i]).unwrap(),
    )
    .unwrap()
}
fn authenticate(bits: &PackedBits, claim: &Claim) -> F128 {
    let kappa = claim.column_weights.len();
    let field = SmallField::for_width(bits.width()).unwrap();
    let mut actual = F128::ZERO;
    // Read the original bit oracle and reconstruct its fixed embedding without
    // using prepared polynomial coefficients, aggregation or encoding tables.
    for i in 0..bits.rows() {
        let value = (0..bits.width()).fold(F128::ZERO, |v, bit| {
            if bits.bit(bit * bits.rows() + i) {
                v + field.basis()[bit]
            } else {
                v
            }
        });
        actual += value * claim.column_weights[i % kappa] * row_weight(&claim.row_point, i / kappa);
    }
    assert_eq!(actual, claim.value);
    assert_eq!(
        claim
            .column_weights
            .iter()
            .copied()
            .fold(F128::ZERO, |a, b| a + b),
        F128::ONE
    );
    actual
}
fn check(source: &PreparedPermutation) -> Proof {
    let prepared = packed::prepare(source).unwrap();
    let mut prover = ch();
    let mut z_source = ch();
    z_source.observe_label(b"mikros-packed-polynomial-permutation-lockstep-v1");
    z_source.observe_bytes(&(source.statement().rows as u64).to_le_bytes());
    z_source.observe_bytes(&(source.statement().width as u64).to_le_bytes());
    z_source.observe_bytes(&(prepared.kappa() as u64).to_le_bytes());
    let z = z_source.sample_f128();
    let (proof, claims, _) = packed::prove(&prepared, &mut prover).unwrap();
    let mut verifier = ch();
    assert_eq!(
        packed::verify(source.statement(), &proof, &mut verifier).unwrap(),
        claims
    );
    assert_eq!(prover.sample_f128(), verifier.sample_f128());
    assert_eq!(claims[0].row_point, claims[1].row_point);
    assert_eq!(claims[0].column_weights, claims[1].column_weights);
    for (j, bits) in [source.left(), source.right()].into_iter().enumerate() {
        authenticate(bits, &claims[j]);
        let field = SmallField::for_width(bits.width()).unwrap();
        let root = (0..crate::padded_len(bits.rows()).unwrap()).fold(F128::ONE, |p, i| {
            p * (z + if i < bits.rows() {
                field.embed(bits.row(i))
            } else {
                F128::ZERO
            })
        });
        assert_eq!(root, proof.roots[j]);
    }
    proof
}

#[test]
fn packed_polynomial_roundtrip_all_subfields_and_intermediate_embeddings() {
    for width in [1usize, 2, 4, 8, 16, 31, 64] {
        let kappa = 128 / width.next_power_of_two();
        check(&source(kappa * kappa, width));
    }
}

#[test]
fn packed_reused_tree_and_projection_buffers_overwrite_different_challenges() {
    // Above the pool's one-MiB threshold for both extension and child arrays.
    let src = source(1 << 19, 31);
    let prepared = packed::prepare(&src).unwrap();
    for nonce in [19u64, 73] {
        let mut prover = ch();
        prover.observe_bytes(&nonce.to_le_bytes());
        let (proof, claims, _) = packed::prove(&prepared, &mut prover).unwrap();
        let mut verifier = ch();
        verifier.observe_bytes(&nonce.to_le_bytes());
        assert_eq!(
            packed::verify(src.statement(), &proof, &mut verifier).unwrap(),
            claims
        );
        assert_eq!(prover.sample_f128(), verifier.sample_f128());
        authenticate(src.left(), &claims[0]);
        authenticate(src.right(), &claims[1]);
    }
}

#[test]
fn packed_polynomial_non_power_of_two_padding_and_shape_rejection() {
    check(&source(19, 31));
    for (n, width) in [(0, 31), (8, 31), (1024, 1), (16, 128)] {
        let bits = PackedBits::from_fn_parallel(n, width, |_| 0).unwrap();
        let src = permutation::prepare(bits.clone(), bits).unwrap();
        assert!(packed::prepare(&src).is_err());
    }
}

#[test]
fn packed_polynomial_tamper_roots_columns_quotient_rounds_and_boundaries() {
    let src = source(256, 31);
    let good = check(&src);
    let reject = |proof: &Proof| {
        let result = packed::verify(src.statement(), proof, &mut ch());
        // Acceptance is conditional on the source claims. A forged endpoint
        // must never pass the independent source oracle even if a degenerate
        // algebraic event allows the local PIOP equation to pass.
        if let Ok(claims) = result {
            let mismatches = claims
                .iter()
                .zip([src.left(), src.right()])
                .filter(|(claim, bits)| {
                    let field = SmallField::for_width(bits.width()).unwrap();
                    let actual = (0..bits.rows()).fold(F128::ZERO, |v, i| {
                        v + field.embed(bits.row(i))
                            * claim.column_weights[i % claim.column_weights.len()]
                            * row_weight(&claim.row_point, i / claim.column_weights.len())
                    });
                    actual != claim.value
                })
                .count();
            assert_ne!(
                mismatches, 0,
                "tampered proof and authenticated endpoints both accepted"
            );
        }
    };
    let mut bad = good.clone();
    bad.roots[0] += F128::ONE;
    reject(&bad);
    for side in 0..2 {
        let mut bad = good.clone();
        bad.extension[side].layers[1].rounds[0][0] += F128::ONE;
        reject(&bad);
        let mut bad = good.clone();
        bad.extension[side].layers[0].left += F128::ONE;
        reject(&bad);
        for level in 0..good.packed_layers.len() {
            let mut bad = good.clone();
            bad.packed_layers[level][side].columns[0] += F128::ONE;
            reject(&bad);
            let mut bad = good.clone();
            bad.packed_layers[level][side].quotient[0] += F128::ONE;
            reject(&bad);
            let mut bad = good.clone();
            bad.packed_layers[level][side].sumcheck.rounds[0][2] += F128::ONE;
            reject(&bad);
            let mut bad = good.clone();
            bad.packed_layers[level][side].sumcheck.left += F128::ONE;
            reject(&bad);
            let mut bad = good.clone();
            bad.packed_layers[level][side].sumcheck.right += F128::ONE;
            reject(&bad);
        }
    }
    let mut bad = good.clone();
    bad.packed_layers.pop();
    assert!(packed::verify(src.statement(), &bad, &mut ch()).is_err());
    let mut bad = good.clone();
    bad.packed_layers[0][0].columns.push(F128::ZERO);
    assert_eq!(
        packed::verify(src.statement(), &bad, &mut ch()),
        Err(Error::ProofShape)
    );
    let mut bad = good;
    bad.packed_layers[0][0].quotient.push(F128::ZERO);
    assert_eq!(
        packed::verify(src.statement(), &bad, &mut ch()),
        Err(Error::ProofShape)
    );
}

#[test]
fn packed_polynomial_thread_count_preserves_proof_claims_and_transcript() {
    let src = source(1 << 12, 31);
    let prepared = packed::prepare(&src).unwrap();
    let run = |threads| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap()
            .install(|| {
                let mut c = ch();
                let (proof, claims, _) = packed::prove(&prepared, &mut c).unwrap();
                (proof, claims, c.sample_f128())
            })
    };
    assert_eq!(run(1), run(4));
}

struct ForcedNodes {
    inner: FsChallenger,
    pending: bool,
    forced: usize,
    samples: usize,
}
impl ForcedNodes {
    fn new() -> Self {
        Self {
            inner: ch(),
            pending: false,
            forced: 0,
            samples: 0,
        }
    }
}
impl Challenger for ForcedNodes {
    fn observe_label(&mut self, label: &[u8]) {
        self.inner.observe_label(label);
        self.pending = label == b"mikros-packed-permutation-column-challenge-v1";
    }
    fn observe_bytes(&mut self, bytes: &[u8]) {
        self.inner.observe_bytes(bytes)
    }
    fn observe_f128(&mut self, x: F128) {
        self.inner.observe_f128(x)
    }
    fn observe_f128_slice(&mut self, x: &[F128]) {
        self.inner.observe_f128_slice(x)
    }
    fn sample_f128(&mut self) -> F128 {
        let normal = self.inner.sample_f128();
        self.samples += 1;
        if self.samples == 1 {
            return F128::ZERO;
        }
        if self.pending && self.forced < 2 {
            let x = if self.forced == 0 {
                F128::ZERO
            } else {
                F128::ONE
            };
            self.forced += 1;
            self.pending = false;
            return x;
        }
        self.pending = false;
        normal
    }
    fn sample_f128_vec(&mut self, n: usize) -> Vec<F128> {
        (0..n).map(|_| self.sample_f128()).collect()
    }
    fn hash_kind(&self) -> flock_hash::HashKind {
        self.inner.hash_kind()
    }
    fn fork_from_seed(&self, _: [F128; 2], _: &'static [u8]) -> Self {
        panic!("unused")
    }
}

#[test]
fn packed_polynomial_zero_shift_and_rejected_column_nodes_replay() {
    let src = source(64, 31);
    let prepared = packed::prepare(&src).unwrap();
    let mut p = ForcedNodes::new();
    let (proof, claims, _) = packed::prove(&prepared, &mut p).unwrap();
    let mut v = ForcedNodes::new();
    assert_eq!(
        packed::verify(src.statement(), &proof, &mut v).unwrap(),
        claims
    );
    assert_eq!(p.forced, 2);
    assert_eq!(v.forced, 2);
    assert_eq!(p.samples, v.samples);
    assert_eq!(p.inner.sample_f128(), v.inner.sample_f128());
    for (claim, bits) in claims.iter().zip([src.left(), src.right()]) {
        authenticate(bits, claim);
    }
}

#[test]
fn packed_polynomial_statement_and_multiset_mismatch_rejected() {
    let src = source(64, 31);
    let proof = check(&src);
    assert!(
        packed::verify(
            &CommittedStatement::new(128, 31).unwrap(),
            &proof,
            &mut ch()
        )
        .is_err()
    );
    assert!(packed::verify(&CommittedStatement::new(64, 30).unwrap(), &proof, &mut ch()).is_err());
    let a = PackedBits::from_fn_parallel(64, 31, |i| i as u128).unwrap();
    let b = PackedBits::from_fn_parallel(64, 31, |i| if i == 0 { 101 } else { i as u128 }).unwrap();
    let bad = permutation::prepare(a, b).unwrap();
    assert!(matches!(
        packed::prove(&packed::prepare(&bad).unwrap(), &mut ch()),
        Err(Error::ProductMismatch)
    ));
}
