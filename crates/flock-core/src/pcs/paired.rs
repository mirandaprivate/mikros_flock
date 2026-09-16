//! Paired F128 functionals through the unchanged native Flock PCS.
//!
//! A logical word `W[i] = A[i] + u B[i]` in F256 is carried as adjacent
//! F128 words `[A[i], B[i]]`. This is a change of representation, not a
//! reduction in committed bytes: N F256 words still occupy 2N F128 words.
//! The coordinate selector is the least-significant packed-word index bit.
//!
//! Both claimed coordinates and their shared functional are bound before a
//! fresh F128 challenge `lambda` is sampled. Their authentication reduces to
//! the single native functional `a + lambda*b`, with coordinate row weights
//! `[1, lambda]`. The ordinary merged transport handles even `lambda = 1`
//! without dividing by zero. No F256 entry-point or Ligerito change is used.
//! Callers must bind the commitment, circuit and PIOP before this reduction,
//! then authenticate every returned claim (along with any ordinary claims)
//! using the existing merged PCS. Challenge-dependent work is online work.

use serde::{Deserialize, Serialize};

use crate::{
    challenger::Challenger,
    field::{F128, F256},
    pcs::{OpeningGrinding, PcsOpenError, functional::TensorClaim},
};

/// The same bit-linear functional applied to each coordinate of paired words.
///
/// `value.c0` authenticates A and `value.c1` authenticates B. Columns are the
/// 128 bit weights within each F128 word. Rows exclude the new coordinate bit
/// and are low-bit first, just as in [`TensorClaim`]. Arbitrary nonnormalized
/// row pairs are supported; both coordinates must use exactly these weights.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claim {
    pub columns: Vec<F128>,
    pub rows: Vec<[F128; 2]>,
    pub value: F256,
}

impl Claim {
    /// A paired packed-word MLE evaluation `W(point) = value`.
    pub fn at_point(point: &[F128], value: F256) -> Self {
        Self {
            columns: (0..128)
                .map(|j| {
                    if j < 64 {
                        F128::new(1u64 << j, 0)
                    } else {
                        F128::new(0, 1u64 << (j - 64))
                    }
                })
                .collect(),
            rows: point.iter().map(|&r| [F128::ONE + r, r]).collect(),
            value,
        }
    }
}

fn bind<C: Challenger>(claims: &[Claim], ch: &mut C) -> Result<(), PcsOpenError> {
    let Some(first) = claims.first() else {
        return Err(PcsOpenError::Assist);
    };
    if claims
        .iter()
        .any(|claim| claim.columns.len() != 128 || claim.rows.len() != first.rows.len())
    {
        return Err(PcsOpenError::Assist);
    }
    ch.observe_label(b"flock-pcs-paired-f128-functional-v1");
    ch.observe_bytes(&(claims.len() as u64).to_le_bytes());
    ch.observe_bytes(&(first.rows.len() as u64).to_le_bytes());
    for claim in claims {
        ch.observe_f128_slice(&claim.columns);
        for pair in &claim.rows {
            ch.observe_f128_slice(pair);
        }
        ch.observe_f256(claim.value);
    }
    Ok(())
}

fn reduce(claims: &[Claim], lambda: F128) -> Vec<TensorClaim> {
    claims
        .iter()
        .map(|claim| {
            let mut rows = Vec::with_capacity(claim.rows.len() + 1);
            rows.push([F128::ONE, lambda]);
            rows.extend_from_slice(&claim.rows);
            TensorClaim {
                columns: claim.columns.clone(),
                rows,
                value: claim.value.c0 + lambda * claim.value.c1,
            }
        })
        .collect()
}

/// Bind paired claims and return `(pairing_nonce, native_claims)`.
///
/// Serialize the nonce with the enclosing proof, and pass every native claim
/// to the ordinary PCS together with other claims on the same carrier. Even
/// one paired claim batches two coordinates, so claim-batch grinding applies.
pub fn reduce_prover<C: Challenger>(
    claims: &[Claim],
    grinding: OpeningGrinding,
    ch: &mut C,
) -> Result<(u64, Vec<TensorClaim>), PcsOpenError> {
    bind(claims, ch)?;
    let (nonce, lambda) = if grinding.claim_batch_bits == 0 {
        (0, ch.sample_f128())
    } else {
        let (nonce, values) = ch.grind_pow_and_sample_f128_vec(grinding.claim_batch_bits, 1);
        (nonce, values[0])
    };
    Ok((nonce, reduce(claims, lambda)))
}

/// Replay the paired-claim reduction before verifying the returned native
/// claims with the existing merged PCS. This function alone is not a proof.
pub fn reduce_verify<C: Challenger>(
    claims: &[Claim],
    pairing_nonce: u64,
    grinding: OpeningGrinding,
    ch: &mut C,
) -> Result<Vec<TensorClaim>, PcsOpenError> {
    bind(claims, ch)?;
    let lambda = if grinding.claim_batch_bits == 0 {
        if pairing_nonce != 0 {
            return Err(PcsOpenError::Assist);
        }
        ch.sample_f128()
    } else {
        ch.verify_pow_and_sample_f128_vec(pairing_nonce, grinding.claim_batch_bits, 1)
            .ok_or(PcsOpenError::Assist)?[0]
    };
    Ok(reduce(claims, lambda))
}

#[cfg(test)]
mod tests {
    use super::{Claim, reduce, reduce_prover, reduce_verify};
    use crate::{
        challenger::{Challenger, FsChallenger},
        field::{F128, F256},
        pcs::{
            self, OpeningGrinding, functional::row_weights, ligerito::LigeritoProfile,
            merged_tensor,
        },
        r1cs::SparseBinaryMatrix,
        schedule::{Registry, TableClass, TableType},
        test_rng::Rng,
        union::UnionInstance,
    };

    fn evaluate(words: &[F128], rows: &[[F128; 2]]) -> F128 {
        words
            .iter()
            .zip(row_weights(rows))
            .fold(F128::ZERO, |sum, (&word, weight)| sum + word * weight)
    }

    #[test]
    fn paired_reduction_preserves_both_coordinates_and_degenerate_challenge() {
        let mut rng = Rng::new(0xFA12);
        let a = rng.f128_vec(16);
        let b = rng.f128_vec(16);
        let mut claim = Claim::at_point(&rng.f128_vec(4), F256::ZERO);
        claim.value = F256::new(evaluate(&a, &claim.rows), evaluate(&b, &claim.rows));
        let carrier: Vec<_> = a.iter().zip(&b).flat_map(|(&a, &b)| [a, b]).collect();
        for lambda in [F128::ZERO, F128::ONE, rng.f128()] {
            let native = reduce(&[claim.clone()], lambda);
            assert_eq!(native.len(), 1);
            assert_eq!(evaluate(&carrier, &native[0].rows), native[0].value);
        }
        let mut pch = FsChallenger::new(b"paired-reduction");
        let (nonce, native) =
            reduce_prover(&[claim.clone()], OpeningGrinding::disabled(), &mut pch).unwrap();
        let mut vch = FsChallenger::new(b"paired-reduction");
        assert_eq!(
            native,
            reduce_verify(
                &[claim.clone()],
                nonce,
                OpeningGrinding::disabled(),
                &mut vch
            )
            .unwrap()
        );
        assert_eq!(pch.sample_f128(), vch.sample_f128());
        assert!(
            reduce_verify(
                &[claim],
                1,
                OpeningGrinding::disabled(),
                &mut FsChallenger::new(b"paired-reduction")
            )
            .is_err()
        );
    }

    #[test]
    fn paired_claims_native_pcs_roundtrip_and_tamper_rejection() {
        let registry = Registry::new(
            vec![TableType {
                k_log: 7,
                useful_bits: 128,
                a_0: SparseBinaryMatrix::new(0, 0, vec![]),
                b_0: SparseBinaryMatrix::new(0, 0, vec![]),
                c_0: SparseBinaryMatrix::new(0, 0, vec![]),
                const_pin: None,
                class: TableClass::Boolean,
                io_schema: vec![],
            }],
            15,
        );
        let union = UnionInstance::new(&registry, vec![1 << 15]);
        let params = pcs::PcsParams {
            m: 22,
            log_inv_rate: 1,
            log_batch_size: 6,
            profile: LigeritoProfile::Fast,
            num_lanes: union.commit_lanes(6),
            merkle_hash: Default::default(),
        };
        let mut rng = Rng::new(0xFA13);
        let a = rng.f128_vec(1 << 14);
        let b = rng.f128_vec(1 << 14);
        let carrier: Vec<_> = a.iter().zip(&b).flat_map(|(&a, &b)| [a, b]).collect();
        let claims: Vec<_> = (0..2)
            .map(|_| {
                let mut claim = Claim::at_point(&rng.f128_vec(14), F256::ZERO);
                claim.value = F256::new(evaluate(&a, &claim.rows), evaluate(&b, &claim.rows));
                claim
            })
            .collect();
        let (cm, data) = pcs::commit(&carrier, &params);
        let init = |cm: &pcs::Commitment| {
            let mut ch = FsChallenger::new(b"paired-native-pcs");
            ch.observe_bytes(&bincode::serialize(cm).unwrap());
            ch
        };
        let mut pch = init(&cm);
        let (nonce, native) = reduce_prover(&claims, params.opening_grinding(), &mut pch).unwrap();
        let proof =
            merged_tensor::open_identity(carrier.clone(), &data, &cm, &native, &union, &mut pch)
                .unwrap();
        let check = |cm: &pcs::Commitment, claims: &[Claim], proof: &merged_tensor::Proof| {
            let mut ch = init(cm);
            let native = reduce_verify(claims, nonce, params.opening_grinding(), &mut ch)?;
            merged_tensor::verify(cm, &native, proof, &union, &mut ch)?;
            Ok::<_, pcs::PcsOpenError>(ch.sample_f128())
        };
        assert_eq!(pch.sample_f128(), check(&cm, &claims, &proof).unwrap());
        for index in 0..claims.len() {
            for coordinate in 0..2 {
                let mut bad = claims.clone();
                if coordinate == 0 {
                    bad[index].value.c0 += F128::ONE;
                } else {
                    bad[index].value.c1 += F128::ONE;
                }
                assert!(check(&cm, &bad, &proof).is_err());
            }
        }
        let mut bad = claims.clone();
        bad[0].value = F256::new(bad[0].value.c1, bad[0].value.c0);
        assert!(check(&cm, &bad, &proof).is_err());
        let mut bad = claims.clone();
        bad[0].rows[0][0] += F128::ONE;
        assert!(check(&cm, &bad, &proof).is_err());
        let mut swapped = carrier;
        for pair in swapped.chunks_exact_mut(2) {
            pair.swap(0, 1);
        }
        let (swapped_cm, _) = pcs::commit(&swapped, &params);
        assert!(check(&swapped_cm, &claims, &proof).is_err());
        let mut bad_proof = proof.clone();
        bad_proof.merged.as_mut().unwrap().q_eval += F128::ONE;
        assert!(check(&cm, &claims, &bad_proof).is_err());
    }
}
