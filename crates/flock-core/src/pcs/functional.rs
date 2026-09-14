//! Open a bit-linear functional with arbitrary 128 column weights.
//!
//! This extends the ring-switch frontend without changing the commitment or
//! Ligerito backend. In particular, row factors need not sum to one. This is
//! necessary for a Hadamard reduction's product of two equality weights.
//! Callers derive queries from their verified PIOP, and bind the commitment and
//! public circuit before starting that PIOP. The opening also binds each query.

use super::{
    BatchOpeningProofLigerito, Commitment, OpeningGrinding, PcsError, ProverData,
    ligerito::{
        extension::recursive_verifier_with_basis_succinct,
        recursive_prover_with_basis_precomputed_round0_lanes,
    },
    ring_switch::{
        RingSwitchError, RingSwitchProof, claim_check, fold_1b_rows_1way_method_of_four_russians,
        fold_b128_elems, tensor_algebra_transpose,
    },
    tensor_algebra::TensorAlgebra256,
};
use crate::zerocheck::univariate_skip::build_eq;
use crate::{
    challenger::Challenger,
    field::{F128, F256},
};
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
    ParallelSlice, ParallelSliceMut,
};
use serde::{Deserialize, Serialize};

/// `sum_{i,j} bits[128*i+j] * columns[j] * product_k rows[k][bit_k(i)]`.
/// `rows` is **low-bit first**, including non-normalized or zero pairs.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TensorClaim {
    pub columns: Vec<F128>,
    pub rows: Vec<[F128; 2]>,
    pub value: F128,
}

fn malformed() -> PcsError {
    PcsError::RingSwitch(RingSwitchError::MalformedProof)
}

fn check_shape(commitment: &Commitment, claims: &[TensorClaim]) -> Result<usize, PcsError> {
    let m = commitment.params.m;
    if !(7..usize::BITS as usize).contains(&m)
        || claims.is_empty()
        || claims
            .iter()
            .any(|c| c.columns.len() != 128 || c.rows.len() != m - 7)
    {
        return Err(malformed());
    }
    Ok(m - 7)
}

fn bind_queries<C: Challenger>(claims: &[TensorClaim], ch: &mut C) {
    ch.observe_label(b"flock-pcs-tensor-functional-v1");
    ch.observe_bytes(&(claims.len() as u64).to_le_bytes());
    for claim in claims {
        ch.observe_bytes(&(claim.rows.len() as u64).to_le_bytes());
        ch.observe_f128_slice(&claim.columns);
        for pair in &claim.rows {
            ch.observe_f128_slice(pair);
        }
        ch.observe_f128(claim.value);
    }
}

/// Materialize the tensor in natural packed-word index order.
pub fn row_weights(rows: &[[F128; 2]]) -> Vec<F128> {
    // Grow in reverse coordinate order so adjacent outputs bind the low bit.
    // This costs O(N) multiplications rather than O(N log N), with no inverse.
    let mut weights = vec![F128::ONE];
    for &[a, b] in rows.iter().rev() {
        let mut next = vec![F128::ZERO; weights.len() * 2];
        next.par_chunks_exact_mut(2)
            .zip(weights.par_iter())
            .for_each(|(out, &w)| {
                if !w.is_zero() {
                    out[0] = if a == F128::ONE { w } else { a * w };
                    out[1] = if b == F128::ONE { w } else { b * w };
                }
            });
        weights = next;
    }
    weights
}

fn ring_target(s_hat_v: &[F128], eq: &[F128]) -> F128 {
    claim_check(&tensor_algebra_transpose(s_hat_v), eq)
}

/// Evaluate the ring-switched row tensor at all remaining Boolean suffixes.
/// Only public query factors and transcript challenges are read by this code.
fn eval_residual(rows: &[[F128; 2]], eq: &[F128], fixed: &[F256], suffix_log: usize) -> Vec<F256> {
    assert_eq!(rows.len(), fixed.len() + suffix_log);
    let mut prefix = TensorAlgebra256::one();
    for (&[a, b], &q) in rows.iter().zip(fixed) {
        let mut next = prefix.clone().scale_vertical_base(a);
        next += &prefix
            .scale_vertical_base(a + b)
            .scale_horizontal_extension(q);
        prefix = next;
    }
    (0..1usize << suffix_log)
        .map(|i| {
            let scale = rows[fixed.len()..]
                .iter()
                .enumerate()
                .fold(F128::ONE, |acc, (k, pair)| acc * pair[(i >> k) & 1]);
            prefix.clone().scale_vertical_base(scale).fold(eq)
        })
        .collect()
}

/// Authenticate all tensor claims against the existing Flock commitment.
/// Challenge-dependent folds, basis construction and lane conversion happen
/// inside this call and are online work. Input is the natural packed bit order.
pub fn open<C: Challenger>(
    packed: Vec<F128>,
    data: &ProverData,
    commitment: &Commitment,
    claims: &[TensorClaim],
    grinding: OpeningGrinding,
    ch: &mut C,
) -> Result<BatchOpeningProofLigerito, PcsError> {
    let log_n = check_shape(commitment, claims)?;
    if packed.len() != 1usize << log_n {
        return Err(malformed());
    }
    let config = commitment
        .params
        .ligerito_prover_config()
        .map_err(|_| PcsError::Ligerito)?;
    if config.initial_k != commitment.params.log_batch_size {
        return Err(malformed());
    }
    bind_queries(claims, ch);
    let mut ring_switches = Vec::with_capacity(claims.len());
    let mut ring_outputs = Vec::with_capacity(claims.len());
    for claim in claims {
        let weights = row_weights(&claim.rows);
        let s_hat_v = fold_1b_rows_1way_method_of_four_russians(&packed, &weights);
        if claim_check(&claim.columns, &s_hat_v) != claim.value {
            return Err(PcsError::RingSwitch(RingSwitchError::ClaimMismatch));
        }
        ch.observe_label(b"flock-ring-switch-functional-v1");
        ch.observe_f128_slice(&s_hat_v);
        let (grinding_nonce, point) = if grinding.ring_switch_bits == 0 {
            (0, ch.sample_f128_vec(7))
        } else {
            ch.grind_pow_and_sample_f128_vec(grinding.ring_switch_bits, 7)
        };
        let eq = build_eq(&point);
        let target = ring_target(&s_hat_v, &eq);
        ring_outputs.push((weights, eq, target));
        ring_switches.push(RingSwitchProof {
            s_hat_v,
            grinding_nonce,
        });
    }
    let bits = grinding.claim_batch_bits_for(claims.len());
    let (batching_nonces, gammas) = if bits == 0 {
        (Vec::new(), ch.sample_f128_vec(claims.len()))
    } else {
        let (nonce, gammas) = ch.grind_pow_and_sample_f128_vec(bits, claims.len());
        (vec![nonce], gammas)
    };
    let mut basis = vec![F128::ZERO; packed.len()];
    let mut target = F128::ZERO;
    for ((weights, eq, value), &gamma) in ring_outputs.into_iter().zip(&gammas) {
        // Folding is F2-linear. Scaling the output coefficients fuses gamma
        // into the lookup and avoids a field multiplication per packed word.
        let scaled: Vec<_> = eq.iter().map(|&x| gamma * x).collect();
        let contribution = fold_b128_elems(&weights, &scaled);
        basis
            .par_iter_mut()
            .zip(contribution.par_iter())
            .for_each(|(x, y)| *x += *y);
        target += gamma * value;
    }
    let lane_major = commitment.params.num_ntts() < 1usize << config.initial_k;
    let (packed, basis) = if lane_major {
        (
            super::commit::lane_grid_from_lane_major(&packed, config.initial_k),
            super::commit::lane_grid_from_lane_major(&basis, config.initial_k),
        )
    } else {
        (packed, basis)
    };
    let round0 = packed
        .par_chunks_exact(2)
        .zip(basis.par_chunks_exact(2))
        .map(|(a, b)| (a[0] * b[0], (a[0] + a[1]) * (b[0] + b[1])))
        .reduce(|| (F128::ZERO, F128::ZERO), |(a, b), (c, d)| (a + c, b + d));
    let ligerito = recursive_prover_with_basis_precomputed_round0_lanes(
        &config,
        packed,
        basis,
        target,
        &data.codeword,
        &data.merkle_tree,
        commitment.params.num_ntts(),
        lane_major,
        round0,
        None,
        None,
        None,
        None,
        ch,
    );
    Ok(BatchOpeningProofLigerito {
        ring_switches,
        batching_nonces,
        ligerito,
    })
}

/// Verify with no witness or materialized row tensor. Uses the normal embedded
/// Ligerito config and the commitment's Merkle cap. The enclosing verifier must
/// compare commitment parameters with its public circuit parameters.
pub fn verify<C: Challenger>(
    commitment: &Commitment,
    claims: &[TensorClaim],
    proof: &BatchOpeningProofLigerito,
    grinding: OpeningGrinding,
    ch: &mut C,
) -> Result<(), PcsError> {
    let log_n = check_shape(commitment, claims)?;
    let config = commitment
        .params
        .ligerito_verifier_config()
        .map_err(|_| PcsError::Ligerito)?;
    if config.initial_k != commitment.params.log_batch_size
        || proof.ring_switches.len() != claims.len()
        || proof.ring_switches.iter().any(|p| p.s_hat_v.len() != 128)
    {
        return Err(malformed());
    }
    let bits = grinding.claim_batch_bits_for(claims.len());
    if proof.batching_nonces.len() != usize::from(bits != 0) {
        return Err(malformed());
    }
    bind_queries(claims, ch);
    let mut outputs = Vec::with_capacity(claims.len());
    for (claim, proof) in claims.iter().zip(&proof.ring_switches) {
        ch.observe_label(b"flock-ring-switch-functional-v1");
        ch.observe_f128_slice(&proof.s_hat_v);
        if claim_check(&claim.columns, &proof.s_hat_v) != claim.value {
            return Err(PcsError::RingSwitch(RingSwitchError::ClaimMismatch));
        }
        let point = if grinding.ring_switch_bits != 0 {
            ch.verify_pow_and_sample_f128_vec(proof.grinding_nonce, grinding.ring_switch_bits, 7)
                .ok_or(PcsError::RingSwitch(RingSwitchError::InvalidGrinding))?
        } else {
            if proof.grinding_nonce != 0 {
                return Err(PcsError::RingSwitch(RingSwitchError::InvalidGrinding));
            }
            ch.sample_f128_vec(7)
        };
        let eq = build_eq(&point);
        outputs.push((ring_target(&proof.s_hat_v, &eq), eq));
    }
    let gammas = if bits == 0 {
        ch.sample_f128_vec(claims.len())
    } else {
        ch.verify_pow_and_sample_f128_vec(proof.batching_nonces[0], bits, claims.len())
            .ok_or(PcsError::RingSwitch(RingSwitchError::InvalidGrinding))?
    };
    let target = outputs
        .iter()
        .zip(&gammas)
        .fold(F128::ZERO, |acc, ((v, _), &g)| acc + *v * g);
    let lane_major = commitment.params.num_ntts() < 1usize << config.initial_k;
    let rows: Vec<_> = claims
        .iter()
        .map(|c| {
            let mut r = c.rows.clone();
            if lane_major {
                r.rotate_left(log_n - config.initial_k);
            }
            r
        })
        .collect();
    let evaluator = |fixed: &[F256], suffix_log: usize| {
        let mut values = vec![F256::ZERO; 1usize << suffix_log];
        for ((r, (_, eq)), &gamma) in rows.iter().zip(&outputs).zip(&gammas) {
            for (dst, value) in values
                .iter_mut()
                .zip(eval_residual(r, eq, fixed, suffix_log))
            {
                *dst += value * gamma;
            }
        }
        values
    };
    if recursive_verifier_with_basis_succinct(
        &config,
        &proof.ligerito,
        log_n,
        target,
        &commitment.cap,
        commitment.params.num_ntts(),
        evaluator,
        ch,
    ) {
        Ok(())
    } else {
        Err(PcsError::Ligerito)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenger::{FsChallenger, RandomChallenger};

    #[test]
    fn arbitrary_pair_tensor_matches_dense_ring_switch() {
        let mut ch = RandomChallenger::new(0xE2E_130);
        for log_n in 0..=8 {
            let mut rows: Vec<_> = (0..log_n)
                .map(|_| [ch.sample_f128(), ch.sample_f128()])
                .collect();
            // Explicitly include zero and non-normalized factors.
            if log_n > 1 {
                rows[1][0] = F128::ZERO;
            }
            let eq: Vec<_> = (0..128).map(|_| ch.sample_f128()).collect();
            let dense = fold_b128_elems(&row_weights(&rows), &eq);
            for prefix_len in 0..=log_n {
                let fixed: Vec<_> = (0..prefix_len)
                    .map(|_| F256::new(ch.sample_f128(), ch.sample_f128()))
                    .collect();
                let actual = eval_residual(&rows, &eq, &fixed, log_n - prefix_len);
                let mut expected: Vec<F256> = dense.iter().map(|&x| F256::from(x)).collect();
                for &r in &fixed {
                    expected = expected
                        .chunks_exact(2)
                        .map(|v| v[0] + (v[0] + v[1]) * r)
                        .collect();
                }
                assert_eq!(actual, expected);
            }
        }
    }

    #[test]
    #[ignore = "full embedded m22 Ligerito proof"]
    fn tensor_opening_full_pcs_and_tamper() {
        use super::super::ligerito::{LigeritoProfile, embedded_initial_k_or_default};
        let m = 22;
        let params = super::super::PcsParams {
            m,
            log_inv_rate: 1,
            log_batch_size: embedded_initial_k_or_default(m, LigeritoProfile::Fast),
            profile: LigeritoProfile::Fast,
            num_lanes: None,
            merkle_hash: Default::default(),
        };
        let mut rng = RandomChallenger::new(0xE2E_131);
        let packed: Vec<_> = (0..1usize << (m - 7)).map(|_| rng.sample_f128()).collect();
        let (commitment, data) = super::super::commit(&packed, &params);
        let mut claims: Vec<_> = (0..2)
            .map(|_| TensorClaim {
                columns: (0..128).map(|_| rng.sample_f128()).collect(),
                rows: (0..m - 7)
                    .map(|_| [rng.sample_f128(), rng.sample_f128()])
                    .collect(),
                value: F128::ZERO,
            })
            .collect();
        for claim in &mut claims {
            claim.value = claim_check(
                &claim.columns,
                &fold_1b_rows_1way_method_of_four_russians(&packed, &row_weights(&claim.rows)),
            );
        }
        let mut pch = FsChallenger::new(b"functional-full-test");
        let proof = open(
            packed,
            &data,
            &commitment,
            &claims,
            params.opening_grinding(),
            &mut pch,
        )
        .unwrap();
        let check =
            |commitment: &Commitment, claims: &[TensorClaim], proof: &BatchOpeningProofLigerito| {
                verify(
                    commitment,
                    claims,
                    proof,
                    params.opening_grinding(),
                    &mut FsChallenger::new(b"functional-full-test"),
                )
            };
        assert!(check(&commitment, &claims, &proof).is_ok());
        let mut vch = FsChallenger::new(b"functional-full-test");
        verify(
            &commitment,
            &claims,
            &proof,
            params.opening_grinding(),
            &mut vch,
        )
        .unwrap();
        assert_eq!(pch.sample_f128(), vch.sample_f128());
        for i in 0..claims.len() {
            let mut bad = claims.clone();
            bad[i].value += F128::ONE;
            assert!(check(&commitment, &bad, &proof).is_err());
            let mut bad = proof.clone();
            bad.ring_switches[i].s_hat_v[127] += F128::ONE;
            assert!(check(&commitment, &claims, &bad).is_err());
        }
        let mut bad = claims.clone();
        bad[0].rows[2][1] += F128::ONE;
        assert!(check(&commitment, &bad, &proof).is_err());
        let mut bad = commitment.clone();
        bad.cap[0][0] ^= 1;
        assert!(check(&bad, &claims, &proof).is_err());
    }
}
