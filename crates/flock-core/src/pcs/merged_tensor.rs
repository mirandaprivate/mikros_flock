//! Arbitrary bit-linear tensors through the production merged PCS transport.
//!
//! Each nonzero row pair `[a,b]` with `s=a+b != 0` becomes the ordinary
//! equality point `b/s`; its scale `s` moves into the 128 column weights.
//! A nonzero zero-sum pair cannot be normalized. If there are `d` such pairs,
//! set all their coordinates to a common variable T. The requested functional
//! is the degree-d coefficient of this polynomial, recovered from d+1 fixed
//! evaluations. This handles `[a,a]` without exponential corner expansion.
//!
//! All component values are bound and their sums checked before opening every
//! component through the existing merged sumcheck, Frobenius assist and inner
//! EqPoint Ligerito path. The leading dummy coordinate is only the legacy
//! prover's point-shape adapter: the verifier checks all 128 column weights.
//! Callers must first bind the commitment, public circuit and original PIOP.
//! Normalization and any challenge-dependent component folds are online work.

use std::ops::Range;

use serde::{Deserialize, Serialize};

use super::{
    Commitment, MergedOpenProof, PcsOpenError, ProverData,
    functional::TensorClaim,
    open_batch_merged,
    ring_switch::{
        build_eq_split, claim_check, fold_1b_rows_naive, fold_1b_rows_split, split_n_lo,
    },
    verify_batch_merged_weighted,
};
use crate::{challenger::Challenger, field::F128, union::UnionInstance, zerocheck::PaddingSpec};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    /// In original-query order, then fixed interpolation-node order. A normal
    /// query contributes one value, a d-zero-sum query d+1, a zero query none.
    pub component_values: Vec<F128>,
    /// None exactly when all original functionals are identically zero.
    pub merged: Option<MergedOpenProof>,
}

struct Component {
    columns: Vec<F128>,
    // Ordinary, low-bit-first row coordinates after the legacy dummy zero.
    x_outer: Vec<F128>,
}

struct Adapted {
    components: Vec<Component>,
    groups: Vec<Range<usize>>,
}

/// Lagrange leading-coefficient weights for the fixed distinct field nodes.
fn leading_weights(nodes: &[F128]) -> Vec<F128> {
    nodes
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            nodes
                .iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .fold(F128::ONE, |product, (_, &y)| product * (x + y))
                .inv()
        })
        .collect()
}

fn adapt(claims: &[TensorClaim], m: usize) -> Result<Adapted, PcsOpenError> {
    if !(7..usize::BITS as usize).contains(&m)
        || claims.is_empty()
        || claims
            .iter()
            .any(|q| q.columns.len() != 128 || q.rows.len() != m - 7)
    {
        return Err(PcsOpenError::Assist);
    }
    let mut components = Vec::new();
    let mut groups = Vec::with_capacity(claims.len());
    for query in claims {
        let start = components.len();
        if query.columns.iter().all(|x| x.is_zero())
            || query.rows.contains(&[F128::ZERO; 2])
        {
            if !query.value.is_zero() {
                return Err(PcsOpenError::Assist);
            }
            groups.push(start..start);
            continue;
        }
        let mut point = vec![F128::ZERO; query.rows.len() + 1];
        let mut scale = F128::ONE;
        let mut bad = Vec::new();
        for (i, &[a, b]) in query.rows.iter().enumerate() {
            let s = a + b;
            if s.is_zero() {
                // The [0,0] case was checked above; a=b is nonzero here.
                scale *= a;
                bad.push(i + 1);
            } else {
                scale *= s;
                point[i + 1] = if s == F128::ONE { b } else { b * s.inv() };
            }
        }
        let nodes: Vec<_> = (0..=bad.len()).map(|i| F128::new(i as u64, 0)).collect();
        for (&node, lambda) in nodes.iter().zip(leading_weights(&nodes)) {
            let mut x_outer = point.clone();
            for &i in &bad {
                x_outer[i] = node;
            }
            let scale = scale * lambda;
            components.push(Component {
                columns: query.columns.iter().map(|&c| scale * c).collect(),
                x_outer,
            });
        }
        groups.push(start..components.len());
    }
    Ok(Adapted { components, groups })
}

fn bind<C: Challenger>(
    claims: &[TensorClaim],
    adapted: &Adapted,
    values: &[F128],
    ch: &mut C,
) -> Result<(), PcsOpenError> {
    if values.len() != adapted.components.len() {
        return Err(PcsOpenError::Assist);
    }
    for (query, group) in claims.iter().zip(&adapted.groups) {
        if values[group.clone()].iter().fold(F128::ZERO, |a, &b| a + b) != query.value {
            return Err(PcsOpenError::Assist);
        }
    }
    ch.observe_label(b"flock-merged-tensor-intake-v1");
    ch.observe_bytes(&(claims.len() as u64).to_le_bytes());
    for (query, group) in claims.iter().zip(&adapted.groups) {
        ch.observe_bytes(&(query.rows.len() as u64).to_le_bytes());
        ch.observe_f128_slice(&query.columns);
        for pair in &query.rows {
            ch.observe_f128_slice(pair);
        }
        ch.observe_f128(query.value);
        ch.observe_bytes(&(group.len() as u64).to_le_bytes());
    }
    // Components and their order are deterministic functions of these bound
    // queries. Values are genuine prover messages, all fixed before batching.
    ch.observe_f128_slice(values);
    Ok(())
}

fn check_commitment(cm: &Commitment, union: &UnionInstance<'_>) -> Result<(), PcsOpenError> {
    if cm.params.m != union.dense_m() || cm.params.log_inv_rate != cm.params.profile.log_inv_rate()
    {
        return Err(PcsOpenError::Ligerito);
    }
    let config = cm
        .params
        .ligerito_verifier_config()
        .map_err(|_| PcsOpenError::Ligerito)?;
    if cm.params.log_batch_size != config.initial_k
        || cm.params.num_lanes != union.commit_lanes(config.initial_k)
        || cm.cap.len() != 1usize << config.level_zero_cap_depth()
    {
        return Err(PcsOpenError::Ligerito);
    }
    Ok(())
}

fn fold_component(padded: &[F128], component: &Component, padding: &PaddingSpec) -> Vec<F128> {
    let rows = &component.x_outer[1..];
    if rows.len() >= 4 {
        let (lo, hi) = build_eq_split(rows, split_n_lo(rows.len()));
        fold_1b_rows_split(padded, &lo, &hi, padding)
    } else {
        let (weights, _) = build_eq_split(rows, rows.len());
        fold_1b_rows_naive(padded, &weights)
    }
}

/// Open on the same compacted union stack and padded virtual buffer as the
/// production Flock prover. Ordinary queries use its batched/sparse fold.
/// `padded` must be canonically zero outside the public union support, as
/// required by `UnionInstance::compact_witness`; this entry does not accept
/// arbitrary dirty padding from an unchecked/pooled witness source.
pub fn open<C: Challenger>(
    q: Vec<F128>,
    padded: &[F128],
    data: &ProverData,
    cm: &Commitment,
    claims: &[TensorClaim],
    union: &UnionInstance<'_>,
    ch: &mut C,
) -> Result<Proof, PcsOpenError> {
    check_commitment(cm, union)?;
    if q.len() != union.committed_words() || padded.len() != union.packed_len() {
        return Err(PcsOpenError::Assist);
    }
    let adapted = adapt(claims, union.m_total())?;
    let padding = union.padding_spec();
    let mut values = vec![F128::ZERO; adapted.components.len()];
    let mut precomputed = vec![None; adapted.components.len()];
    for (query, group) in claims.iter().zip(&adapted.groups) {
        if group.len() == 1 {
            values[group.start] = query.value;
        } else {
            for i in group.clone() {
                let s_hat = fold_component(padded, &adapted.components[i], &padding);
                values[i] = claim_check(&adapted.components[i].columns, &s_hat);
                precomputed[i] = Some(s_hat);
            }
        }
    }
    bind(claims, &adapted, &values, ch)?;
    let merged = if adapted.components.is_empty() {
        None
    } else {
        let xs: Vec<_> = adapted
            .components
            .iter()
            .map(|c| c.x_outer.as_slice())
            .collect();
        let pre: Vec<_> = precomputed.iter().map(Option::as_deref).collect();
        let config = cm
            .params
            .ligerito_prover_config()
            .map_err(|_| PcsOpenError::Ligerito)?;
        Some(open_batch_merged(
            q,
            Some(padded),
            data,
            cm,
            &xs,
            &pre,
            &[],
            &[],
            &padding,
            &union.jagged_heights(),
            union.n_log(),
            &config,
            cm.params.opening_grinding(),
            ch,
        ))
    };
    Ok(Proof {
        component_values: values,
        merged,
    })
}

/// Recompute the complete intake from verified PIOP claims, check component
/// recombination, then use the production weighted128 merged verifier.
/// The caller additionally checks its public profile/hash parameter policy.
pub fn verify<C: Challenger>(
    cm: &Commitment,
    claims: &[TensorClaim],
    proof: &Proof,
    union: &UnionInstance<'_>,
    ch: &mut C,
) -> Result<(), PcsOpenError> {
    check_commitment(cm, union)?;
    let adapted = adapt(claims, union.m_total())?;
    bind(claims, &adapted, &proof.component_values, ch)?;
    if adapted.components.is_empty() {
        return if proof.merged.is_none() {
            Ok(())
        } else {
            Err(PcsOpenError::Assist)
        };
    }
    let merged = proof.merged.as_ref().ok_or(PcsOpenError::Assist)?;
    let xs: Vec<_> = adapted
        .components
        .iter()
        .map(|c| c.x_outer.as_slice())
        .collect();
    let columns: Vec<_> = adapted
        .components
        .iter()
        .map(|c| c.columns.as_slice())
        .collect();
    let config = cm
        .params
        .ligerito_verifier_config()
        .map_err(|_| PcsOpenError::Ligerito)?;
    verify_batch_merged_weighted(
        cm,
        &proof.component_values,
        &columns,
        &xs,
        &[],
        &union.jagged_heights(),
        union.n_log(),
        merged,
        &config,
        cm.params.opening_grinding(),
        ch,
    )
}

#[cfg(test)]
mod tests {
    use super::{adapt, bind, fold_component, leading_weights, open, verify};
    use crate::{
        challenger::{Challenger, FsChallenger},
        field::F128,
        pcs::{
            self,
            functional::{TensorClaim, row_weights},
            ring_switch::{claim_check, fold_1b_rows_naive},
        },
        test_rng::Rng,
        zerocheck::PaddingSpec,
    };

    #[test]
    fn arbitrary_pairs_and_zero_sum_diagonal_match_source() {
        let mut rng = Rng::new(0x51AD_130);
        for row_log in 0..=7 {
            let packed = rng.f128_vec(1 << row_log);
            for bad_count in 0..=row_log {
                let mut rows: Vec<_> = (0..row_log).map(|_| [rng.f128(), rng.f128()]).collect();
                for pair in rows.iter_mut().take(bad_count) {
                    let a = rng.f128() + F128::ONE;
                    *pair = [a, a];
                }
                let mut query = TensorClaim {
                    columns: rng.f128_vec(128),
                    rows,
                    value: F128::ZERO,
                };
                query.value = claim_check(
                    &query.columns,
                    &fold_1b_rows_naive(&packed, &row_weights(&query.rows)),
                );
                let adapted = adapt(&[query.clone()], 7 + row_log).unwrap();
                assert_eq!(adapted.components.len(), bad_count + 1);
                let values: Vec<_> = adapted
                    .components
                    .iter()
                    .map(|c| {
                        assert_eq!(c.x_outer[0], F128::ZERO);
                        claim_check(
                            &c.columns,
                            &fold_component(&packed, c, &PaddingSpec::dense(7 + row_log)),
                        )
                    })
                    .collect();
                bind(
                    &[query.clone()],
                    &adapted,
                    &values,
                    &mut FsChallenger::new(b"diagonal"),
                )
                .unwrap();
                let mut wrong = values.clone();
                wrong[0] += F128::ONE;
                assert!(
                    bind(
                        &[query],
                        &adapted,
                        &wrong,
                        &mut FsChallenger::new(b"diagonal")
                    )
                    .is_err()
                );
            }
        }
        let packed = rng.f128_vec(128);
        let mut sparse = TensorClaim {
            columns: rng.f128_vec(128),
            rows: vec![[F128::ONE, F128::ZERO]; 7],
            value: F128::ZERO,
        };
        sparse.rows[1] = [F128::ZERO, F128::new(9, 0)];
        sparse.rows[4] = [F128::new(3, 0), F128::new(7, 0)];
        sparse.value = claim_check(
            &sparse.columns,
            &fold_1b_rows_naive(&packed, &row_weights(&sparse.rows)),
        );
        let adapted = adapt(&[sparse.clone()], 14).unwrap();
        let value = claim_check(
            &adapted.components[0].columns,
            &fold_component(&packed, &adapted.components[0], &PaddingSpec::dense(14)),
        );
        assert_eq!(value, sparse.value);
        // A maximal valid query does not cause exponential expansion.
        let query = TensorClaim {
            columns: vec![F128::ONE; 128],
            rows: vec![[F128::ONE; 2]; 28],
            value: F128::ZERO,
        };
        assert_eq!(adapt(&[query], 35).unwrap().components.len(), 29);
    }

    #[test]
    fn zero_functional_and_component_shapes_are_explicit() {
        let mut q = TensorClaim {
            columns: vec![F128::ONE; 128],
            rows: vec![[F128::ZERO; 2]],
            value: F128::ZERO,
        };
        assert!(adapt(&[q.clone()], 8).unwrap().components.is_empty());
        q.value = F128::ONE;
        assert!(adapt(&[q.clone()], 8).is_err());
        q.value = F128::ZERO;
        q.rows = vec![[F128::ONE; 2]];
        q.columns.fill(F128::ZERO);
        assert!(adapt(&[q.clone()], 8).unwrap().components.is_empty());
        q.columns.pop();
        assert!(adapt(&[q], 8).is_err());
        assert!(adapt(&[], 8).is_err());
        let nodes: Vec<_> = (0..8).map(|i| F128::new(i, 0)).collect();
        let weights = leading_weights(&nodes);
        for degree in 0..8 {
            let value = nodes.iter().zip(&weights).fold(F128::ZERO, |s, (&x, &w)| {
                s + w * (0..degree).fold(F128::ONE, |a, _| a * x)
            });
            assert_eq!(value, if degree == 7 { F128::ONE } else { F128::ZERO });
        }
    }

    #[test]
    #[ignore = "full embedded m22 merged transport and Ligerito"]
    fn full_merged_normalized_zero_sum_and_tamper() {
        use crate::{
            r1cs::SparseBinaryMatrix,
            schedule::{Registry, TableClass, TableType},
            union::UnionInstance,
        };
        use pcs::ligerito::{LigeritoProfile, embedded_initial_k_or_default};
        let ty = TableType {
            k_log: 10,
            useful_bits: 1024,
            a_0: SparseBinaryMatrix::new(0, 0, vec![]),
            b_0: SparseBinaryMatrix::new(0, 0, vec![]),
            c_0: SparseBinaryMatrix::new(0, 0, vec![]),
            const_pin: None,
            class: TableClass::Boolean,
            io_schema: vec![],
        };
        let registry = Registry::new(vec![ty], 12);
        let union = UnionInstance::new(&registry, vec![4095]);
        let m = union.dense_m();
        let k = embedded_initial_k_or_default(m, LigeritoProfile::Fast);
        let params = pcs::PcsParams {
            m,
            log_inv_rate: 1,
            log_batch_size: k,
            profile: LigeritoProfile::Fast,
            num_lanes: union.commit_lanes(k),
            merkle_hash: Default::default(),
        };
        let mut rng = Rng::new(0x51AD_131);
        let mut packed = rng.f128_vec(union.packed_len());
        for c in 0..8 {
            packed[c * 4096 + 4095] = F128::ZERO;
        }
        let q = union.compact_witness(&packed);
        let (cm, data) = if params.num_lanes.is_some() {
            pcs::commit_lane_major(&q, &params)
        } else {
            pcs::commit(&q, &params)
        };
        let mut claims: Vec<_> = (0..3)
            .map(|_| TensorClaim {
                columns: rng.f128_vec(128),
                rows: (0..15).map(|_| [rng.f128(), rng.f128()]).collect(),
                value: F128::ZERO,
            })
            .collect();
        // A legacy skip64-times-one-bit functional is an exact weighted128
        // special case. The adapter does not alter its ordinary row point.
        claims[0].columns = pcs::ring_switch::build_claim_weights(rng.f128(), rng.f128());
        claims[0].rows = (0..15)
            .map(|_| {
                let r = rng.f128();
                [F128::ONE + r, r]
            })
            .collect();
        claims[1].rows[2] = [F128::ONE; 2];
        claims[2].rows[12..].fill([F128::ONE, F128::ZERO]);
        for c in &mut claims {
            c.value = claim_check(
                &c.columns,
                &fold_1b_rows_naive(&packed, &row_weights(&c.rows)),
            );
        }
        let mut pch = FsChallenger::new(b"merged-tensor-full");
        let proof = open(q.clone(), &packed, &data, &cm, &claims, &union, &mut pch).unwrap();
        assert_eq!(proof.component_values.len(), 4);
        let mut vch = FsChallenger::new(b"merged-tensor-full");
        verify(&cm, &claims, &proof, &union, &mut vch).unwrap();
        assert_eq!(pch.sample_f128(), vch.sample_f128());
        // Exact production-prover reuse: under the same bound intake, even
        // the zero-sum precomputed folds give the same complete merged proof
        // as the existing batched prover's ordinary non-precomputed route.
        let adapted = adapt(&claims, union.m_total()).unwrap();
        let xs: Vec<_> = adapted
            .components
            .iter()
            .map(|c| c.x_outer.as_slice())
            .collect();
        let mut direct_ch = FsChallenger::new(b"merged-tensor-full");
        bind(&claims, &adapted, &proof.component_values, &mut direct_ch).unwrap();
        let direct = pcs::open_batch_merged(
            q.clone(),
            Some(&packed),
            &data,
            &cm,
            &xs,
            &[],
            &[],
            &[],
            &union.padding_spec(),
            &union.jagged_heights(),
            union.n_log(),
            &params.ligerito_prover_config().unwrap(),
            params.opening_grinding(),
            &mut direct_ch,
        );
        assert_eq!(proof.merged.as_ref().unwrap(), &direct);
        let reject = |p: &super::Proof| {
            assert!(
                verify(
                    &cm,
                    &claims,
                    p,
                    &union,
                    &mut FsChallenger::new(b"merged-tensor-full")
                )
                .is_err()
            )
        };
        let mut bad = proof.clone();
        bad.component_values[1] += F128::ONE;
        reject(&bad);
        // Preserve the original zero-sum query's recombination. The individual
        // component openings, not the intake's sum check, must reject this.
        let mut bad = proof.clone();
        bad.component_values[1] += F128::ONE;
        bad.component_values[2] += F128::ONE;
        assert_eq!(
            bad.component_values[1] + bad.component_values[2],
            claims[1].value
        );
        reject(&bad);
        let mut bad = proof.clone();
        bad.component_values.pop();
        reject(&bad);
        for i in 0..4 {
            let mut bad = proof.clone();
            bad.merged.as_mut().unwrap().ring_switches[i].s_hat_v[0] += F128::ONE;
            reject(&bad);
        }
        let mut bad = proof.clone();
        bad.merged.as_mut().unwrap().q_eval += F128::ONE;
        reject(&bad);
        let mut bad = proof.clone();
        bad.merged.as_mut().unwrap().merged_rounds[0].0 += F128::ONE;
        reject(&bad);
        let mut bad = proof.clone();
        bad.merged.as_mut().unwrap().inner.ligerito.initial_cap[0][0] ^= 1;
        reject(&bad);
        let mut bad = proof.clone();
        bad.merged = None;
        reject(&bad);
        let zero = TensorClaim {
            columns: vec![F128::ZERO; 128],
            rows: vec![[F128::ONE; 2]; 15],
            value: F128::ZERO,
        };
        let mut zch = FsChallenger::new(b"merged-tensor-zero");
        let zero_proof = open(q, &packed, &data, &cm, &[zero.clone()], &union, &mut zch).unwrap();
        assert!(zero_proof.merged.is_none() && zero_proof.component_values.is_empty());
        let mut zvch = FsChallenger::new(b"merged-tensor-zero");
        verify(&cm, &[zero.clone()], &zero_proof, &union, &mut zvch).unwrap();
        assert_eq!(zch.sample_f128(), zvch.sample_f128());
        let mut bad = zero_proof.clone();
        bad.merged = proof.merged;
        assert!(
            verify(
                &cm,
                &[zero.clone()],
                &bad,
                &union,
                &mut FsChallenger::new(b"merged-tensor-zero")
            )
            .is_err()
        );
        let mut wrong_zero = zero;
        wrong_zero.value = F128::ONE;
        assert!(
            verify(
                &cm,
                &[wrong_zero],
                &zero_proof,
                &union,
                &mut FsChallenger::new(b"merged-tensor-zero")
            )
            .is_err()
        );
    }
}
