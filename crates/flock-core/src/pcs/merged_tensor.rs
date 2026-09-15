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
//! `FLOCK_PCS_PACKED_DIRECT=1` optionally routes scalar multiples of canonical
//! packed-word columns directly through the same native merged PCS. Both
//! parties must use the same experimental setting: the PCS challenge stream
//! changes, while the original intake and serialized proof layout do not.

use std::ops::Range;

use serde::{Deserialize, Serialize};

use super::{
    Commitment, DirectEqInd, LOG_PACKING, MergedOpenProof, PackedDirectClaim, PackedDirectClaimRef,
    PcsOpenError, ProverData,
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

#[cfg(test)]
std::thread_local! {
    static PACKED_DIRECT_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
}

fn packed_direct_enabled() -> bool {
    #[cfg(test)]
    if let Some(enabled) = PACKED_DIRECT_OVERRIDE.get() {
        return enabled;
    }
    std::env::var_os("FLOCK_PCS_PACKED_DIRECT").is_some_and(|value| value == "1")
}

#[cfg(test)]
fn with_packed_direct_override<T>(enabled: bool, f: impl FnOnce() -> T) -> T {
    struct Restore(Option<bool>);
    impl Drop for Restore {
        fn drop(&mut self) {
            PACKED_DIRECT_OVERRIDE.set(self.0);
        }
    }
    let _restore = Restore(PACKED_DIRECT_OVERRIDE.replace(Some(enabled)));
    f()
}

/// In this geometry the committed word index and the virtual packed-word
/// index coincide, including every bit in each word. Identity compaction by
/// itself is insufficient: it can still contain partially useful words.
fn full_identity_rectangle(union: &UnionInstance<'_>) -> bool {
    let types = union.registry().types();
    types.len() == 1
        && types[0].k_log >= LOG_PACKING
        && types[0].useful_bits == 1 << types[0].k_log
        && union.counts() == [1usize << union.n_log()]
        && union
            .jagged_heights()
            .iter()
            .all(|&h| h == 1u64 << union.n_log())
        && union.compaction_is_identity()
        && union.dense_words() == union.packed_len()
}

/// Return the inverse of the nonzero scalar multiplying all canonical word
/// basis columns. A zero claimed value is not a reason to skip an opening.
fn packed_column_inverse(columns: &[F128]) -> Option<F128> {
    if columns.len() != 128 || columns[0].is_zero() {
        return None;
    }
    let scale = columns[0];
    columns
        .iter()
        .enumerate()
        .all(|(j, &column)| {
            let beta = if j < 64 {
                F128::new(1u64 << j, 0)
            } else {
                F128::new(0, 1u64 << (j - 64))
            };
            column == scale * beta
        })
        .then(|| scale.inv())
}

struct Routes {
    ring_switch: Vec<usize>,
    /// Original component index and inverse column scale, in original order.
    direct: Vec<(usize, F128)>,
}

fn routes(adapted: &Adapted, union: &UnionInstance<'_>) -> Routes {
    let allowed = packed_direct_enabled() && full_identity_rectangle(union);
    let mut result = Routes {
        ring_switch: Vec::new(),
        direct: Vec::new(),
    };
    for (index, component) in adapted.components.iter().enumerate() {
        if let Some(inverse) = allowed
            .then(|| packed_column_inverse(&component.columns))
            .flatten()
        {
            result.direct.push((index, inverse));
        } else {
            result.ring_switch.push(index);
        }
    }
    result
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
        if query.columns.iter().all(|x| x.is_zero()) || query.rows.contains(&[F128::ZERO; 2]) {
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
    open_impl(q, Some(padded), data, cm, claims, union, ch)
}

/// Open a full-domain rectangular union directly from its owned witness.
///
/// This exposes the native merged opener's identity alias: `q` is also the
/// padded witness, so no second witness or compaction buffer is needed.
/// Every row and every bit of its one table type must belong to the union
/// support. Multiple word columns use the native rectangular transport.
/// The caller must bind its circuit and commitment before the PIOP,
/// exactly as for [`open`]. For the same union, messages and challenges
/// match the existing borrowed-witness entry.
pub fn open_identity<C: Challenger>(
    q: Vec<F128>,
    data: &ProverData,
    cm: &Commitment,
    claims: &[TensorClaim],
    union: &UnionInstance<'_>,
    ch: &mut C,
) -> Result<Proof, PcsOpenError> {
    if !full_identity_rectangle(union) || q.len() != union.packed_len() {
        return Err(PcsOpenError::Assist);
    }
    open_impl(q, None, data, cm, claims, union, ch)
}

fn open_impl<C: Challenger>(
    q: Vec<F128>,
    padded: Option<&[F128]>,
    data: &ProverData,
    cm: &Commitment,
    claims: &[TensorClaim],
    union: &UnionInstance<'_>,
    ch: &mut C,
) -> Result<Proof, PcsOpenError> {
    check_commitment(cm, union)?;
    let padded_view = padded.unwrap_or(&q);
    if q.len() != union.committed_words() || padded_view.len() != union.packed_len() {
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
                let s_hat = fold_component(padded_view, &adapted.components[i], &padding);
                values[i] = claim_check(&adapted.components[i].columns, &s_hat);
                precomputed[i] = Some(s_hat);
            }
        }
    }
    bind(claims, &adapted, &values, ch)?;
    let routes = routes(&adapted, union);
    if std::env::var_os("PCS_TRACE").is_some() {
        eprintln!(
            "PCS merged_tensor intake: claims={} components={} ring_switch={} packed_direct={}",
            claims.len(),
            adapted.components.len(),
            routes.ring_switch.len(),
            routes.direct.len(),
        );
    }
    let merged = if adapted.components.is_empty() {
        None
    } else {
        let xs: Vec<_> = routes
            .ring_switch
            .iter()
            .map(|&i| adapted.components[i].x_outer.as_slice())
            .collect();
        let pre: Vec<_> = routes
            .ring_switch
            .iter()
            .map(|&i| precomputed[i].as_deref())
            .collect();
        let direct: Vec<_> = routes
            .direct
            .iter()
            .map(|&(i, inverse)| {
                // The dummy is only a legacy ring-switch adapter. Full
                // identity rectangles use these remaining coordinates in
                // exactly the same low-bit-first word order as committed q.
                let point = adapted.components[i].x_outer[1..].to_vec();
                PackedDirectClaim {
                    eq_ind: DirectEqInd::EqPoint(point.clone()),
                    point,
                    value: values[i] * inverse,
                }
            })
            .collect();
        let config = cm
            .params
            .ligerito_prover_config()
            .map_err(|_| PcsOpenError::Ligerito)?;
        Some(open_batch_merged(
            q,
            padded,
            data,
            cm,
            &xs,
            &pre,
            &[],
            &direct,
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
    // Rebuild the partition from the bound original queries and public union,
    // never from an unbound prover-controlled routing tag.
    let routes = routes(&adapted, union);
    let xs: Vec<_> = routes
        .ring_switch
        .iter()
        .map(|&i| adapted.components[i].x_outer.as_slice())
        .collect();
    let columns: Vec<_> = routes
        .ring_switch
        .iter()
        .map(|&i| adapted.components[i].columns.as_slice())
        .collect();
    let values: Vec<_> = routes
        .ring_switch
        .iter()
        .map(|&i| proof.component_values[i])
        .collect();
    let direct: Vec<_> = routes
        .direct
        .iter()
        .map(|&(i, inverse)| PackedDirectClaimRef {
            point: &adapted.components[i].x_outer[1..],
            value: proof.component_values[i] * inverse,
        })
        .collect();
    let config = cm
        .params
        .ligerito_verifier_config()
        .map_err(|_| PcsOpenError::Ligerito)?;
    verify_batch_merged_weighted(
        cm,
        &values,
        &columns,
        &xs,
        &direct,
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
    use super::{
        adapt, bind, fold_component, full_identity_rectangle, leading_weights, open, open_identity,
        packed_column_inverse, routes, verify, with_packed_direct_override,
    };
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

    fn packed_columns(scale: F128) -> Vec<F128> {
        (0..128)
            .map(|j| {
                scale
                    * if j < 64 {
                        F128::new(1 << j, 0)
                    } else {
                        F128::new(0, 1 << (j - 64))
                    }
            })
            .collect()
    }

    fn set_claim_value(claim: &mut TensorClaim, packed: &[F128]) {
        claim.value = claim_check(
            &claim.columns,
            &fold_1b_rows_naive(packed, &row_weights(&claim.rows)),
        );
    }

    #[test]
    fn packed_direct_scaled_columns_and_zero_sum_match_word_source() {
        let mut rng = Rng::new(0xD1_2EC7);
        for scale in [F128::ONE, F128::new(0, 1), rng.f128()] {
            let columns = packed_columns(scale);
            assert_eq!(packed_column_inverse(&columns), Some(scale.inv()));
            for j in 0..128 {
                let mut wrong = columns.clone();
                wrong[j] += F128::ONE;
                assert_eq!(packed_column_inverse(&wrong), None, "column {j}");
            }
        }
        assert_eq!(packed_column_inverse(&[]), None);
        assert_eq!(packed_column_inverse(&vec![F128::ZERO; 128]), None);
        let mut zero_first = packed_columns(F128::ONE);
        zero_first[0] = F128::ZERO;
        assert_eq!(packed_column_inverse(&zero_first), None);

        for row_log in 0..=7 {
            let packed = rng.f128_vec(1 << row_log);
            for bad_count in 0..=row_log {
                let mut claim = TensorClaim {
                    columns: packed_columns(rng.f128()),
                    rows: (0..row_log).map(|_| [rng.f128(), rng.f128()]).collect(),
                    value: F128::ZERO,
                };
                for pair in claim.rows.iter_mut().take(bad_count) {
                    *pair = [F128::new(7, 0); 2];
                }
                set_claim_value(&mut claim, &packed);
                let adapted = adapt(&[claim.clone()], row_log + 7).unwrap();
                let values: Vec<_> = adapted
                    .components
                    .iter()
                    .map(|component| {
                        let inverse = packed_column_inverse(&component.columns).unwrap();
                        let component_value = claim_check(
                            &component.columns,
                            &fold_component(&packed, component, &PaddingSpec::dense(row_log + 7)),
                        );
                        // Independent word-domain fold, with no bit expansion
                        // or interpolation weights reused from the adapter.
                        let mut words = packed.clone();
                        for &r in &component.x_outer[1..] {
                            words = words
                                .chunks_exact(2)
                                .map(|pair| pair[0] + r * (pair[0] + pair[1]))
                                .collect();
                        }
                        assert_eq!(component_value * inverse, words[0]);
                        component_value
                    })
                    .collect();
                bind(
                    &[claim],
                    &adapted,
                    &values,
                    &mut FsChallenger::new(b"packed-scale"),
                )
                .unwrap();
            }
        }
        // Zero functionals have no components; a genuine opening whose
        // evaluation is zero retains its route and must still be proved.
        let mut zero = TensorClaim {
            columns: packed_columns(F128::ONE),
            rows: vec![[F128::ZERO; 2]; 15],
            value: F128::ZERO,
        };
        assert!(adapt(&[zero.clone()], 22).unwrap().components.is_empty());
        zero.rows.fill([F128::ONE, F128::ZERO]);
        let registry = container_registry(7, 128, 15);
        let union = crate::union::UnionInstance::new(&registry, vec![32768]);
        let adapted = adapt(&[zero], 22).unwrap();
        with_packed_direct_override(true, || {
            let route = routes(&adapted, &union);
            assert!(route.ring_switch.is_empty());
            assert_eq!(route.direct.len(), 1);
        });
        with_packed_direct_override(false, || {
            assert_eq!(routes(&adapted, &union).ring_switch, [0]);
        });
    }

    #[test]
    fn packed_direct_native_mixed_rectangles_tamper_and_legacy_bytes() {
        use pcs::ligerito::LigeritoProfile;
        for col_log in [0, 2, 7] {
            let registry = container_registry(7 + col_log, 128 << col_log, 15 - col_log);
            let union = crate::union::UnionInstance::new(&registry, vec![1 << (15 - col_log)]);
            assert!(full_identity_rectangle(&union));
            let params = pcs::PcsParams {
                m: 22,
                log_inv_rate: 1,
                log_batch_size: 6,
                profile: LigeritoProfile::Fast,
                num_lanes: union.commit_lanes(6),
                merkle_hash: Default::default(),
            };
            let mut rng = Rng::new(0xD1_2EC8 + col_log as u64);
            let mut q = rng.f128_vec(union.packed_len());
            q[0] = F128::ZERO;
            let (cm, data) = pcs::commit(&q, &params);
            let mut claims: Vec<_> = (0..4)
                .map(|i| TensorClaim {
                    columns: if i == 1 {
                        rng.f128_vec(128)
                    } else {
                        packed_columns(rng.f128())
                    },
                    rows: (0..15)
                        .map(|_| {
                            let r = rng.f128();
                            // Non-normalized pair scale must also be removed
                            // from the value of the direct packed-word claim.
                            let scale = rng.f128();
                            [scale * (F128::ONE + r), scale * r]
                        })
                        .collect(),
                    value: F128::ZERO,
                })
                .collect();
            // The native Frobenius grouping must also handle a shared point
            // across the ring-switch and direct partitions.
            claims[1].rows = claims[0].rows.clone();
            // Same physical row point with different word-column weights.
            let mut same_row = claims[0].clone();
            same_row.columns = packed_columns(F128::ONE);
            if col_log > 0 {
                let r = rng.f128();
                same_row.rows[14] = [F128::ONE + r, r];
            }
            claims.push(same_row);
            claims[2].rows[0] = [F128::new(3, 0); 2];
            claims[2].rows[1] = [F128::new(5, 0); 2];
            // A nonzero functional with a zero value still gets authenticated.
            claims[3].rows.fill([F128::ONE, F128::ZERO]);
            let mut zero = claims[0].clone();
            zero.rows[0] = [F128::ZERO; 2];
            claims.push(zero);
            for claim in &mut claims {
                set_claim_value(claim, &q);
            }
            assert!(claims[3].value.is_zero());
            let adapted = adapt(&claims, union.m_total()).unwrap();
            with_packed_direct_override(true, || {
                let route = routes(&adapted, &union);
                assert_eq!(route.ring_switch, [1]);
                assert_eq!(route.direct.len(), 6);
            });
            let domain = b"packed-direct-native-mixed";
            let mut pch = FsChallenger::new(domain);
            let proof = with_packed_direct_override(true, || {
                open_identity(q.clone(), &data, &cm, &claims, &union, &mut pch).unwrap()
            });
            assert_eq!(proof.merged.as_ref().unwrap().ring_switches.len(), 1);
            let after = pch.sample_f128();
            let mut vch = FsChallenger::new(domain);
            with_packed_direct_override(true, || verify(&cm, &claims, &proof, &union, &mut vch))
                .unwrap();
            assert_eq!(after, vch.sample_f128());
            // The opposite configuration must reject cleanly, before consuming
            // the differently shaped native ring-switch proof list.
            assert!(
                with_packed_direct_override(false, || {
                    verify(&cm, &claims, &proof, &union, &mut FsChallenger::new(domain))
                })
                .is_err()
            );

            let reject = |claims: &[TensorClaim], proof: &super::Proof| {
                assert!(
                    with_packed_direct_override(true, || {
                        verify(&cm, claims, proof, &union, &mut FsChallenger::new(domain))
                    })
                    .is_err()
                );
            };
            let mut bad = proof.clone();
            // Preserve a zero-sum query's sum: the native PCS must check each
            // rescaled component rather than only the bound recombination.
            bad.component_values[2] += F128::ONE;
            bad.component_values[3] += F128::ONE;
            reject(&claims, &bad);
            let mut bad_claims = claims.clone();
            let mut bad = proof.clone();
            bad_claims[0].value += F128::ONE;
            bad.component_values[0] += F128::ONE;
            reject(&bad_claims, &bad);
            let mut bad_claims = claims.clone();
            bad_claims[0].columns[127] += F128::ONE;
            reject(&bad_claims, &proof);
            let mut bad_claims = claims.clone();
            for column in &mut bad_claims[0].columns {
                *column *= F128::new(7, 0);
            }
            reject(&bad_claims, &proof);
            let mut bad = proof.clone();
            bad.merged.as_mut().unwrap().q_eval += F128::ONE;
            reject(&claims, &bad);
            let mut bad = proof.clone();
            bad.merged.as_mut().unwrap().ring_switches[0].s_hat_v[0] += F128::ONE;
            reject(&claims, &bad);

            if col_log == 2 {
                let mut borrowed_ch = FsChallenger::new(domain);
                let borrowed = with_packed_direct_override(true, || {
                    open(q.clone(), &q, &data, &cm, &claims, &union, &mut borrowed_ch).unwrap()
                });
                assert_eq!(
                    bincode::serialize(&proof).unwrap(),
                    bincode::serialize(&borrowed).unwrap()
                );
                assert_eq!(after, borrowed_ch.sample_f128());
                let mut old_ch = FsChallenger::new(domain);
                let old = with_packed_direct_override(false, || {
                    open_identity(q.clone(), &data, &cm, &claims, &union, &mut old_ch).unwrap()
                });
                assert_eq!(old.component_values, proof.component_values);
                assert_eq!(old.merged.as_ref().unwrap().ring_switches.len(), 7);
                assert_ne!(
                    bincode::serialize(&old).unwrap(),
                    bincode::serialize(&proof).unwrap()
                );
                assert!(
                    with_packed_direct_override(true, || {
                        verify(&cm, &claims, &old, &union, &mut FsChallenger::new(domain))
                    })
                    .is_err()
                );
                let mut old_vch = FsChallenger::new(domain);
                with_packed_direct_override(false, || {
                    verify(&cm, &claims, &old, &union, &mut old_vch)
                })
                .unwrap();
                let old_after = old_ch.sample_f128();
                assert_eq!(old_after, old_vch.sample_f128());
                // Reconstruct the pre-routing native call explicitly. OFF
                // retains the complete old proof bytes and next challenge.
                let xs: Vec<_> = adapted
                    .components
                    .iter()
                    .map(|c| c.x_outer.as_slice())
                    .collect();
                let mut legacy_ch = FsChallenger::new(domain);
                bind(&claims, &adapted, &old.component_values, &mut legacy_ch).unwrap();
                let legacy = pcs::open_batch_merged(
                    q.clone(),
                    None,
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
                    &mut legacy_ch,
                );
                let legacy = super::Proof {
                    component_values: old.component_values.clone(),
                    merged: Some(legacy),
                };
                assert_eq!(
                    bincode::serialize(&old).unwrap(),
                    bincode::serialize(&legacy).unwrap()
                );
                assert_eq!(old_after, legacy_ch.sample_f128());
                // All-direct batches exercise native n_rs=0, including a
                // genuine zero evaluation and multiple distinct row groups.
                let direct_claims = vec![claims[0].clone(), claims[3].clone(), claims[4].clone()];
                let mut direct_ch = FsChallenger::new(domain);
                let direct = with_packed_direct_override(true, || {
                    open_identity(q, &data, &cm, &direct_claims, &union, &mut direct_ch).unwrap()
                });
                assert!(direct.merged.as_ref().unwrap().ring_switches.is_empty());
                let mut direct_vch = FsChallenger::new(domain);
                with_packed_direct_override(true, || {
                    verify(&cm, &direct_claims, &direct, &union, &mut direct_vch)
                })
                .unwrap();
                assert_eq!(direct_ch.sample_f128(), direct_vch.sample_f128());
            }
        }
    }

    #[test]
    fn packed_direct_partial_and_jagged_geometry_falls_back() {
        use pcs::ligerito::{LigeritoProfile, embedded_initial_k_or_default};
        // Both a partial-useful-bit identity compaction and a nonidentity
        // jagged compaction must retain the old bit-domain route.
        for (useful_bits, count) in [(1023, 4096), (1024, 4095)] {
            let registry = container_registry(10, useful_bits, 12);
            let union = crate::union::UnionInstance::new(&registry, vec![count]);
            assert!(!full_identity_rectangle(&union));
            assert_eq!(union.compaction_is_identity(), count == 4096);
            let mut rng = Rng::new(0xD1_2EC9);
            let mut packed = rng.f128_vec(union.packed_len());
            for row in 0..4096 {
                if useful_bits == 1023 {
                    packed[7 * 4096 + row].hi &= !(1 << 63);
                }
                if row >= count {
                    for col in 0..8 {
                        packed[col * 4096 + row] = F128::ZERO;
                    }
                }
            }
            let q = union.compact_witness(&packed);
            let k = embedded_initial_k_or_default(union.dense_m(), LigeritoProfile::Fast);
            let params = pcs::PcsParams {
                m: union.dense_m(),
                log_inv_rate: 1,
                log_batch_size: k,
                profile: LigeritoProfile::Fast,
                num_lanes: union.commit_lanes(k),
                merkle_hash: Default::default(),
            };
            let (cm, data) = if params.num_lanes.is_some() {
                pcs::commit_lane_major(&q, &params)
            } else {
                pcs::commit(&q, &params)
            };
            let mut claims = vec![TensorClaim {
                columns: packed_columns(rng.f128()),
                rows: (0..15)
                    .map(|_| {
                        let r = rng.f128();
                        [F128::ONE + r, r]
                    })
                    .collect(),
                value: F128::ZERO,
            }];
            set_claim_value(&mut claims[0], &packed);
            with_packed_direct_override(true, || {
                let route = routes(&adapt(&claims, union.m_total()).unwrap(), &union);
                assert_eq!(route.ring_switch, [0]);
                assert!(route.direct.is_empty());
            });
            let mut old_ch = FsChallenger::new(b"packed-direct-fallback");
            let old = with_packed_direct_override(false, || {
                open(q.clone(), &packed, &data, &cm, &claims, &union, &mut old_ch).unwrap()
            });
            let mut new_ch = FsChallenger::new(b"packed-direct-fallback");
            let new = with_packed_direct_override(true, || {
                open(q, &packed, &data, &cm, &claims, &union, &mut new_ch).unwrap()
            });
            assert_eq!(
                bincode::serialize(&old).unwrap(),
                bincode::serialize(&new).unwrap()
            );
            let after = old_ch.sample_f128();
            assert_eq!(after, new_ch.sample_f128());
            let mut vch = FsChallenger::new(b"packed-direct-fallback");
            with_packed_direct_override(true, || verify(&cm, &claims, &new, &union, &mut vch))
                .unwrap();
            assert_eq!(after, vch.sample_f128());
        }
    }

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

    fn container_registry(
        k_log: usize,
        useful_bits: usize,
        nu: usize,
    ) -> crate::schedule::Registry {
        use crate::{
            r1cs::SparseBinaryMatrix,
            schedule::{Registry, TableClass, TableType},
        };
        Registry::new(
            vec![TableType {
                k_log,
                useful_bits,
                a_0: SparseBinaryMatrix::new(0, 0, vec![]),
                b_0: SparseBinaryMatrix::new(0, 0, vec![]),
                c_0: SparseBinaryMatrix::new(0, 0, vec![]),
                const_pin: None,
                class: TableClass::Boolean,
                io_schema: vec![],
            }],
            nu,
        )
    }

    #[test]
    fn identity_open_rejects_wrong_length_and_partial_rectangle_support() {
        use pcs::ligerito::LigeritoProfile;
        let cm = pcs::Commitment {
            cap: vec![[0; 32]; 128],
            params: pcs::PcsParams {
                m: 22,
                log_inv_rate: 1,
                log_batch_size: 6,
                profile: LigeritoProfile::Fast,
                num_lanes: None,
                merkle_hash: Default::default(),
            },
        };
        let data = pcs::ProverData {
            codeword: vec![],
            merkle_tree: vec![],
        };
        let zero = TensorClaim {
            columns: vec![F128::ZERO; 128],
            rows: vec![[F128::ONE; 2]; 15],
            value: F128::ZERO,
        };
        for (k_log, useful, nu, count, words) in [
            (7, 128, 15, 32768, 32767),
            (7, 128, 15, 32767, 32768),
            (7, 127, 15, 32768, 32768),
            (8, 255, 14, 16384, 32768),
            (9, 512, 13, 8191, 32768),
        ] {
            let registry = container_registry(k_log, useful, nu);
            let union = crate::union::UnionInstance::new(&registry, vec![count]);
            assert!(
                open_identity(
                    vec![F128::ZERO; words],
                    &data,
                    &cm,
                    &[zero.clone()],
                    &union,
                    &mut FsChallenger::new(b"identity-invalid-shape"),
                )
                .is_err()
            );
        }
        // Full two-, four- and 128-column rectangles are the same owned
        // identity transport. A zero functional requires no backend data.
        for col_log in [0, 1, 2, 7] {
            let registry = container_registry(7 + col_log, 128 << col_log, 15 - col_log);
            let union = crate::union::UnionInstance::new(&registry, vec![1 << (15 - col_log)]);
            assert!(
                open_identity(
                    vec![F128::ZERO; 32768],
                    &data,
                    &cm,
                    &[zero.clone()],
                    &union,
                    &mut FsChallenger::new(b"identity-full-rectangle"),
                )
                .unwrap()
                .merged
                .is_none()
            );
        }
    }

    #[test]
    #[ignore = "full embedded m22 identity alias and Ligerito differential"]
    fn full_merged_identity_matches_borrowed_proof_and_transcript() {
        use pcs::ligerito::LigeritoProfile;
        for col_log in [0, 2, 7] {
            let registry = container_registry(7 + col_log, 128 << col_log, 15 - col_log);
            let union = crate::union::UnionInstance::new(&registry, vec![1 << (15 - col_log)]);
            assert!(union.compaction_is_identity());
            assert_eq!(
                union.jagged_heights(),
                vec![1 << (15 - col_log); 1 << col_log]
            );
            let params = pcs::PcsParams {
                m: 22,
                log_inv_rate: 1,
                log_batch_size: 6,
                profile: LigeritoProfile::Fast,
                num_lanes: union.commit_lanes(6),
                merkle_hash: Default::default(),
            };
            let mut rng = Rng::new(0x51AD_132);
            let q = rng.f128_vec(union.packed_len());
            let (cm, data) = pcs::commit(&q, &params);
            let mut claims: Vec<_> = (0..2)
                .map(|_| TensorClaim {
                    columns: rng.f128_vec(128),
                    rows: (0..15)
                        .map(|_| {
                            let r = rng.f128();
                            [F128::ONE + r, r]
                        })
                        .collect(),
                    value: F128::ZERO,
                })
                .collect();
            // Both the common normalized path and pre-move zero-sum folds must
            // give the same messages as the existing borrowed-witness entry.
            claims[1].rows[4] = [F128::ONE; 2];
            // Different bit weights and column point, identical row point:
            // the optimized rectangular transport must reuse its bit-bank
            // without combining or dropping either original claim.
            let mut repeated_row = claims[0].clone();
            repeated_row.columns = rng.f128_vec(128);
            if col_log > 0 {
                let r = rng.f128();
                repeated_row.rows[14] = [F128::ONE + r, r];
            }
            claims.push(repeated_row);
            for c in &mut claims {
                c.value = claim_check(&c.columns, &fold_1b_rows_naive(&q, &row_weights(&c.rows)));
            }
            let mut old_ch = FsChallenger::new(b"merged-identity-differential");
            let old = pcs::with_merged_split_rows_override(false, || {
                open(q.clone(), &q, &data, &cm, &claims, &union, &mut old_ch).unwrap()
            });
            let mut new_ch = FsChallenger::new(b"merged-identity-differential");
            let new = pcs::with_merged_split_rows_override(true, || {
                open_identity(q, &data, &cm, &claims, &union, &mut new_ch).unwrap()
            });
            assert_eq!(new.component_values.len(), 4);
            assert_eq!(
                bincode::serialize(&new).unwrap(),
                bincode::serialize(&old).unwrap()
            );
            let after = new_ch.sample_f128();
            assert_eq!(after, old_ch.sample_f128());
            let mut vch = FsChallenger::new(b"merged-identity-differential");
            verify(&cm, &claims, &new, &union, &mut vch).unwrap();
            assert_eq!(after, vch.sample_f128());
            let mut bad = new;
            bad.component_values[0] += F128::ONE;
            assert!(
                verify(
                    &cm,
                    &claims,
                    &bad,
                    &union,
                    &mut FsChallenger::new(b"merged-identity-differential")
                )
                .is_err()
            );
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
