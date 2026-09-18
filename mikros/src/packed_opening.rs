//! Section 5 packed opening frontend for the native FLOCK PCS.
//!
//! Every query sends its 128 column evaluations before any fresh, independent
//! 128-coordinate projection is sampled. Projected claims in each committed
//! group share one ordinary F128 inner-product reduction. Its one remaining
//! canonical-basis MLE claim must be authenticated by the underlying PCS.
//! Callers must bind the commitments before entering this protocol.

use flock_core::{
    challenger::Challenger,
    pcs::{
        functional::{TensorClaim, row_weights},
        ring_switch::{fold_1b_rows_naive, fold_1b_rows_split},
        tensor_algebra::TensorAlgebra,
    },
    zerocheck::PaddingSpec,
};
use flock_field::{F128, F256Unreduced};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{InnerProductProof, inner_product, lookup::BindingTable};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    /// Flattened in group order, then in that group's query order.
    pub columns: Vec<Vec<F128>>,
    pub ips: Vec<InnerProductProof>,
}

fn basis() -> Vec<F128> {
    (0..128)
        .map(|bit| {
            if bit < 64 {
                F128::new(1u64 << bit, 0)
            } else {
                F128::new(0, 1u64 << (bit - 64))
            }
        })
        .collect()
}

fn dot(a: &[F128], b: &[F128]) -> F128 {
    debug_assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b)
        .fold(F256Unreduced::ZERO, |acc, (&a, &b)| {
            acc ^ a.mul_unreduced(b)
        })
        .reduce()
}

fn check_shape(logs: &[usize], claims: &[&[TensorClaim]]) -> Result<usize, String> {
    if logs.is_empty()
        || logs.len() != claims.len()
        || logs.iter().any(|&log| log >= usize::BITS as usize)
    {
        return Err("packed opening group shape".into());
    }
    let mut count = 0usize;
    for (&log, group) in logs.iter().zip(claims) {
        if group.is_empty()
            || group
                .iter()
                .any(|q| q.columns.len() != 128 || q.rows.len() != log)
        {
            return Err("packed opening query shape".into());
        }
        count = count
            .checked_add(group.len())
            .ok_or("packed opening query count overflow")?;
    }
    Ok(count)
}

fn bind_queries<C: Challenger>(logs: &[usize], claims: &[&[TensorClaim]], ch: &mut C) {
    ch.observe_label(b"mikros-packed-opening-v1");
    ch.observe_bytes(&(logs.len() as u64).to_le_bytes());
    for (&log, group) in logs.iter().zip(claims) {
        ch.observe_bytes(&(log as u64).to_le_bytes());
        ch.observe_bytes(&(group.len() as u64).to_le_bytes());
        for q in *group {
            ch.observe_f128_slice(&q.columns);
            for pair in &q.rows {
                ch.observe_f128_slice(pair);
            }
            ch.observe_f128(q.value);
        }
    }
}

fn reference_enabled() -> Result<bool, String> {
    match std::env::var("MIKROS_OPENING_REFERENCE") {
        Ok(value) if value == "1" => Ok(true),
        Ok(value) if value == "0" => Ok(false),
        Err(std::env::VarError::NotPresent) => Ok(false),
        _ => Err("MIKROS_OPENING_REFERENCE must be 0 or 1".into()),
    }
}

/// Strip only exact coordinate restrictions, never a nontrivial field scale:
/// the later bit-coordinate projection is F2-linear, not F128-linear.
fn active_block(rows: &[[F128; 2]], reference: bool) -> (usize, usize) {
    let mut active = rows.len();
    let mut offset = 0usize;
    if !reference {
        while active != 0 {
            let pair = rows[active - 1];
            let bit = if pair == [F128::ONE, F128::ZERO] {
                0
            } else if pair == [F128::ZERO, F128::ONE] {
                1
            } else {
                break;
            };
            active -= 1;
            offset |= bit << active;
        }
    }
    (active, offset)
}

fn columns(words: &[F128], rows: &[[F128; 2]], reference: bool) -> Vec<F128> {
    let (active, offset) = active_block(rows, reference);
    let words = &words[offset..offset + (1usize << active)];
    let rows = &rows[..active];
    if rows.len() < 4 {
        fold_1b_rows_naive(words, &row_weights(rows))
    } else {
        let split = rows.len().div_ceil(2).max(4).min(rows.len());
        fold_1b_rows_split(
            words,
            &row_weights(&rows[..split]),
            &row_weights(&rows[split..]),
            &PaddingSpec::dense(rows.len() + 7),
        )
    }
}

fn projected_target(columns: &[F128], projection: &[F128]) -> F128 {
    // columns[j] packs column j of U. Transposing packs the rows of U,
    // so their dot with r is exactly r^T U beta.
    let rows = TensorAlgebra {
        elems: columns.to_vec(),
    }
    .transpose();
    dot(&rows.elems, projection)
}

fn projected_weights(
    rows: &[[F128; 2]],
    projection: &[F128],
    reference: bool,
) -> (usize, Vec<F128>) {
    let (active, offset) = active_block(rows, reference);
    let weights = row_weights(&rows[..active]);
    let table = BindingTable::new(projection);
    let project = |weight: &F128| {
        let mut bytes = [0u8; 16];
        bytes[..8].copy_from_slice(&weight.lo.to_le_bytes());
        bytes[8..].copy_from_slice(&weight.hi.to_le_bytes());
        table.fold(&bytes)
    };
    let projected = if weights.len() >= 32768 {
        weights.par_iter().map(project).collect()
    } else {
        weights.iter().map(project).collect()
    };
    (offset, projected)
}

/// Exact public MLE evaluation from the manuscript's small-matrix recursion.
/// TensorAlgebra's horizontal side stores Y and its vertical side stores Xi.
fn public_evaluation(
    rows: &[[F128; 2]],
    projection: &[F128],
    point: &[F128],
    reference: bool,
) -> F128 {
    debug_assert_eq!(rows.len(), point.len());
    let (active, offset) = active_block(rows, reference);
    let mut w = TensorAlgebra::one();
    for (&[a, b], &y) in rows[..active].iter().zip(point) {
        let mut next = w.clone().scale_vertical(a);
        next += &w.scale_vertical(a + b).scale_horizontal(y);
        w = next;
    }
    // The zero extension belongs to Y's MLE, outside the F2-linear
    // projection. Do not discard or normalize this prefix factor.
    let prefix = point
        .iter()
        .enumerate()
        .skip(active)
        .fold(F128::ONE, |acc, (bit, &y)| {
            acc * if offset >> bit & 1 == 0 {
                F128::ONE + y
            } else {
                y
            }
        });
    prefix * w.fold_vertical(projection)
}

fn terminal(mut high_point: Vec<F128>, value: F128) -> TensorClaim {
    high_point.reverse();
    TensorClaim {
        columns: basis(),
        rows: high_point.into_iter().map(|y| [F128::ONE + y, y]).collect(),
        value,
    }
}

/// All groups' commitments must already be bound. Returned terminal claims
/// are in group order and each refers to that group's whole packed vector.
pub fn prove<C: Challenger>(
    groups: &[(&[F128], &[TensorClaim])],
    ch: &mut C,
) -> Result<(Proof, Vec<TensorClaim>), String> {
    prove_impl(groups, reference_enabled()?, ch)
}

fn prove_impl<C: Challenger>(
    groups: &[(&[F128], &[TensorClaim])],
    reference: bool,
    ch: &mut C,
) -> Result<(Proof, Vec<TensorClaim>), String> {
    if groups
        .iter()
        .any(|(words, _)| !words.len().is_power_of_two())
    {
        return Err("packed opening source length".into());
    }
    let logs: Vec<_> = groups
        .iter()
        .map(|(words, _)| words.len().ilog2() as usize)
        .collect();
    let claims: Vec<_> = groups.iter().map(|(_, claims)| *claims).collect();
    let count = check_shape(&logs, &claims)?;
    bind_queries(&logs, &claims, ch);
    let jobs: Vec<_> = groups
        .iter()
        .flat_map(|(words, queries)| queries.iter().map(move |q| (*words, q)))
        .collect();
    let messages: Vec<_> = jobs
        .par_iter()
        .map(|(words, q)| columns(words, &q.rows, reference))
        .collect();
    // No projection is sampled until every query's complete column message
    // has been fixed. Independent full vectors prevent cross-query cancellation.
    for ((_, q), message) in jobs.iter().zip(&messages) {
        if dot(&q.columns, message) != q.value {
            return Err("packed opening column scalar mismatch".into());
        }
        ch.observe_f128_slice(message);
    }
    let projections: Vec<_> = (0..count).map(|_| ch.sample_f128_vec(128)).collect();
    let mut ips = Vec::with_capacity(groups.len());
    let mut terminals = Vec::with_capacity(groups.len());
    let mut first = 0;
    for (group, &(words, queries)) in groups.iter().enumerate() {
        let last = first + queries.len();
        let mut combined = vec![F128::ZERO; words.len()];
        let projected: Vec<_> = queries
            .par_iter()
            .zip(&projections[first..last])
            .map(|(q, r)| projected_weights(&q.rows, r, reference))
            .collect();
        for (offset, weights) in projected {
            for (out, value) in combined[offset..offset + weights.len()]
                .iter_mut()
                .zip(weights)
            {
                *out += value;
            }
        }
        let target = messages[first..last]
            .iter()
            .zip(&projections[first..last])
            .fold(F128::ZERO, |acc, (message, r)| {
                acc + projected_target(message, r)
            });
        // This catches an internal transpose/projection mistake before a
        // proof can leave the prover. The native initial pass is reused by IP.
        let (actual, initial) = inner_product::checked_initial_message_native(words, &combined);
        if actual != target {
            return Err("packed opening projected target mismatch".into());
        }
        ch.observe_label(b"mikros-packed-opening-group-v1");
        ch.observe_bytes(&(group as u64).to_le_bytes());
        let (proof, point) = if reference {
            inner_product::prove_packed(words.to_vec(), combined, ch)
        } else {
            let mut scratch = inner_product::ProverScratch::default();
            scratch.a = words.to_vec();
            scratch.b = combined;
            inner_product::prove_native(&mut scratch, initial, ch)
        };
        terminals.push(terminal(point, proof.final_a));
        ips.push(proof);
        first = last;
    }
    Ok((
        Proof {
            columns: messages,
            ips,
        },
        terminals,
    ))
}

pub fn verify<C: Challenger>(
    group_logs: &[usize],
    claims: &[&[TensorClaim]],
    proof: &Proof,
    ch: &mut C,
) -> Result<Vec<TensorClaim>, String> {
    let count = check_shape(group_logs, claims)?;
    if proof.columns.len() != count
        || proof.columns.iter().any(|column| column.len() != 128)
        || proof.ips.len() != group_logs.len()
        || proof
            .ips
            .iter()
            .zip(group_logs)
            .any(|(ip, &log)| ip.rounds.len() != log)
    {
        return Err("packed opening proof shape".into());
    }
    bind_queries(group_logs, claims, ch);
    for (q, message) in claims
        .iter()
        .flat_map(|group| group.iter())
        .zip(&proof.columns)
    {
        if dot(&q.columns, message) != q.value {
            return Err("packed opening column scalar mismatch".into());
        }
        ch.observe_f128_slice(message);
    }
    let projections: Vec<_> = (0..count).map(|_| ch.sample_f128_vec(128)).collect();
    let mut first = 0;
    let mut terminals = Vec::with_capacity(group_logs.len());
    for (group, queries) in claims.iter().enumerate() {
        let last = first + queries.len();
        let target = proof.columns[first..last]
            .iter()
            .zip(&projections[first..last])
            .fold(F128::ZERO, |acc, (message, r)| {
                acc + projected_target(message, r)
            });
        ch.observe_label(b"mikros-packed-opening-group-v1");
        ch.observe_bytes(&(group as u64).to_le_bytes());
        let point = inner_product::verify(&proof.ips[group], target, ch)
            .map_err(|e| format!("packed opening inner product: {e:?}"))?;
        let low_point: Vec<_> = point.iter().rev().copied().collect();
        let public_value = queries
            .iter()
            .zip(&projections[first..last])
            .fold(F128::ZERO, |acc, (q, r)| {
                acc + public_evaluation(&q.rows, r, &low_point, false)
            });
        if public_value != proof.ips[group].final_b {
            return Err("packed opening public terminal mismatch".into());
        }
        terminals.push(terminal(point, proof.ips[group].final_a));
        first = last;
    }
    Ok(terminals)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::challenger::FsChallenger;
    use flock_field::test_rng::Rng;

    fn coordinate_dot(value: F128, weights: &[F128]) -> F128 {
        weights
            .iter()
            .enumerate()
            .fold(F128::ZERO, |sum, (bit, &weight)| {
                let set = if bit < 64 {
                    value.lo >> bit & 1
                } else {
                    value.hi >> (bit - 64) & 1
                };
                if set == 0 { sum } else { sum + weight }
            })
    }

    fn oracle(words: &[F128], q: &TensorClaim) -> F128 {
        words
            .iter()
            .enumerate()
            .fold(F128::ZERO, |acc, (row, &word)| {
                let weight = q
                    .rows
                    .iter()
                    .enumerate()
                    .fold(F128::ONE, |acc, (bit, pair)| acc * pair[row >> bit & 1]);
                acc + weight * coordinate_dot(word, &q.columns)
            })
    }

    fn queries(words: &[F128], rng: &mut Rng) -> Vec<TensorClaim> {
        let log = words.len().ilog2() as usize;
        (0..6)
            .map(|index| {
                let mut rows: Vec<_> = (0..log).map(|_| [rng.f128(), rng.f128()]).collect();
                if log > 0 {
                    rows[log - 1] = match index {
                        0 => [F128::ONE, F128::ZERO],
                        1 => [F128::ZERO, F128::ONE],
                        2 => [F128::ZERO; 2],
                        3 => [F128::new(17, 9); 2],
                        4 => [F128::ZERO, F128::new(23, 42)],
                        _ => rows[log - 1],
                    };
                }
                if log > 1 && index < 2 {
                    rows[log - 2] = [F128::ZERO, F128::ONE];
                }
                let mut q = TensorClaim {
                    columns: rng.f128_vec(128),
                    rows,
                    value: F128::ZERO,
                };
                q.value = oracle(words, &q);
                q
            })
            .collect()
    }

    #[test]
    fn public_recurrence_matches_independent_coordinate_oracle() {
        let mut rng = Rng::new(0x0F3A_1387);
        for log in [0usize, 1, 3, 6] {
            let words = rng.f128_vec(1 << log);
            for q in queries(&words, &mut rng) {
                let projection = rng.f128_vec(128);
                for point in [
                    vec![F128::ZERO; log],
                    vec![F128::ONE; log],
                    rng.f128_vec(log),
                ] {
                    let expected = (0..1usize << log).fold(F128::ZERO, |sum, index| {
                        let xi = q
                            .rows
                            .iter()
                            .enumerate()
                            .fold(F128::ONE, |p, (bit, pair)| p * pair[index >> bit & 1]);
                        let y = point.iter().enumerate().fold(F128::ONE, |p, (bit, &y)| {
                            p * if index >> bit & 1 == 0 {
                                F128::ONE + y
                            } else {
                                y
                            }
                        });
                        sum + y * coordinate_dot(xi, &projection)
                    });
                    assert_eq!(
                        public_evaluation(&q.rows, &projection, &point, true),
                        expected
                    );
                    assert_eq!(
                        public_evaluation(&q.rows, &projection, &point, false),
                        expected
                    );
                }
            }
        }
    }

    #[test]
    fn batched_opening_preserves_reference_proof_and_authenticates_full_oracles() {
        let mut rng = Rng::new(0xE231_A901);
        for logs in [[0usize, 2], [5, 8]] {
            let words: Vec<_> = logs.iter().map(|&log| rng.f128_vec(1 << log)).collect();
            let queries: Vec<_> = words.iter().map(|words| queries(words, &mut rng)).collect();
            let groups: Vec<_> = words
                .iter()
                .zip(&queries)
                .map(|(words, queries)| (words.as_slice(), queries.as_slice()))
                .collect();
            let claims: Vec<_> = queries.iter().map(Vec::as_slice).collect();
            let mut rc = FsChallenger::new(b"packed-opening-test");
            let mut pc = FsChallenger::new(b"packed-opening-test");
            let (reference, rt) = prove_impl(&groups, true, &mut rc).unwrap();
            let (proof, terminal) = prove_impl(&groups, false, &mut pc).unwrap();
            assert_eq!(proof, reference);
            assert_eq!(terminal, rt);
            let mut vc = FsChallenger::new(b"packed-opening-test");
            assert_eq!(verify(&logs, &claims, &proof, &mut vc).unwrap(), terminal);
            for (words, q) in words.iter().zip(&terminal) {
                assert_eq!(oracle(words, q), q.value);
            }
            let next = pc.sample_f128();
            assert_eq!(next, rc.sample_f128());
            assert_eq!(next, vc.sample_f128());

            let rejects = |bad: &Proof| {
                assert!(
                    verify(
                        &logs,
                        &claims,
                        bad,
                        &mut FsChallenger::new(b"packed-opening-test")
                    )
                    .is_err()
                )
            };
            let mut bad = proof.clone();
            bad.columns[0][0] += F128::ONE;
            rejects(&bad);
            // Preserve the advertised scalar by compensating in another
            // column: the independent projection and IP must still reject.
            let mut bad = proof.clone();
            let q = &queries[0][0];
            bad.columns[0][0] += q.columns[1];
            bad.columns[0][1] += q.columns[0];
            rejects(&bad);
            let mut bad = proof.clone();
            bad.ips[0].final_a += F128::ONE;
            rejects(&bad);
            let mut bad = proof.clone();
            bad.ips[1].rounds[0][0] += F128::ONE;
            rejects(&bad);
            let mut bad = proof.clone();
            bad.columns[0].pop();
            rejects(&bad);
            let mut bad = proof.clone();
            bad.ips.pop();
            rejects(&bad);
            let mut bad = proof;
            bad.ips[0].rounds.push([F128::ZERO; 2]);
            rejects(&bad);
        }
    }

    #[test]
    fn malformed_groups_and_false_scalar_are_rejected() {
        let mut rng = Rng::new(0xA031);
        let words = rng.f128_vec(8);
        let mut claims = queries(&words, &mut rng);
        claims[0].value += F128::ONE;
        assert!(prove_impl(&[(&words, &claims)], false, &mut FsChallenger::new(b"bad")).is_err());
        claims[0].value += F128::ONE;
        claims[0].columns.pop();
        assert!(prove_impl(&[(&words, &claims)], false, &mut FsChallenger::new(b"bad")).is_err());
        assert!(prove_impl(&[], false, &mut FsChallenger::new(b"bad")).is_err());
        assert!(
            prove_impl(
                &[(&words[..7], &claims)],
                false,
                &mut FsChallenger::new(b"bad")
            )
            .is_err()
        );
    }
}
