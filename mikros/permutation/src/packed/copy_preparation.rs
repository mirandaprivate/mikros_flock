//! Fixed public position geometry for a binary copy proof.
//!
//! The plan contains labels and their public linear interpolations only. Every
//! witness bit, public-I/O value, polynomial product and sample product is
//! rebuilt by `prepare`, which belongs in online prover time.

use super::{
    CommittedStatement, Domain, Error, Polys, Prepared, PreparedTree, active_parent_nodes, domain,
    prepare_parent_layer_prefix, prepare_parent_samples_prefix, shape,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Instant};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct CopyPreparationTimings {
    pub leaf_ms: f64,
    pub layers_ms: f64,
    pub samples_ms: f64,
    pub total_ms: f64,
}

struct FirstSamples {
    /// Each kappa-row block's label interpolation at every off-domain point.
    /// These are degree-zero values, stored in the same compact field format.
    labels: [Polys; 2],
    /// table[sample][mask] = theta * sum_j lambda_sample[j] * mask[j].
    /// The table is independent of which mask the live witness will select.
    value_tables: Vec<Vec<u128>>,
}

/// Public setup for sources `label[side][i] + theta * value(i)`, where theta
/// is the highest coordinate in a d-bit intermediate field. Both sides use
/// the same live value at each position, as in the PLONK copy reduction.
/// No witness values, public-I/O values or challenges are retained here.
pub struct CopyPreparationPlan {
    statement: CommittedStatement,
    active_rows: usize,
    kappa: usize,
    domain: Arc<Domain>,
    labels: [Polys; 2],
    first_samples: Option<FirstSamples>,
}

fn elapsed(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

fn source_value_masks<F>(count: usize, active_rows: usize, value: F) -> Vec<u8>
where
    F: Fn(usize) -> bool + Sync + Send,
{
    let mut values = vec![0u8; count.div_ceil(8)];
    values[..active_rows.div_ceil(8)]
        .par_iter_mut()
        .enumerate()
        .with_min_len(256)
        .for_each(|(group, dst)| {
            *dst = (0..8).fold(0, |mask, row| {
                let index = group * 8 + row;
                mask | (u8::from(index < active_rows && value(index)) << row)
            });
        });
    values
}

fn inject_leaf_values(label: &Polys, active_rows: usize, values: &[u8]) -> Polys {
    let bits = label.bits;
    let mut leaf = label.clone();
    let active_bytes = (active_rows * bits).div_ceil(8);
    // Include the complete final eight-row group where it exists;
    // its inactive mask bits are zero and cannot change labels.
    let scan_bytes = active_rows
        .div_ceil(8)
        .saturating_mul(bits)
        .min(leaf.bytes.len());
    debug_assert!(scan_bytes >= active_bytes);
    leaf.bytes[..scan_bytes]
        .par_chunks_mut(bits)
        .enumerate()
        .with_min_len(256)
        .for_each(|(group, dst)| {
            let mask = values[group];
            if bits < 8 {
                for row in 0..8.min(dst.len() * 8 / bits) {
                    let position = row * bits + bits - 1;
                    dst[position / 8] |= ((mask >> row) & 1) << (position % 8);
                }
            } else {
                for (row, out) in dst.chunks_mut(bits / 8).enumerate() {
                    out[bits / 8 - 1] |= ((mask >> row) & 1) << 7;
                }
            }
        });
    leaf
}

impl CopyPreparationPlan {
    /// `labels` contains exactly statement.rows public labels on each side.
    /// Positions at and beyond `active_rows` are fixed zero source entries;
    /// both labels must be zero there. Their value closure is never called.
    /// Power-of-two source padding is automatic.
    pub fn new(
        statement: CommittedStatement,
        labels: [Vec<u128>; 2],
        active_rows: usize,
    ) -> Result<Self, Error> {
        let (kappa, log_len) = shape(&statement)?;
        let domain = domain(statement.width)?;
        let bits = domain.base.base.bits;
        if statement.width != bits || bits > 64 {
            return Err(Error::Width);
        }
        let theta = 1u128 << (bits - 1);
        if labels.iter().any(|label| label.len() != statement.rows) {
            return Err(Error::Length);
        }
        if active_rows > statement.rows {
            return Err(Error::Length);
        }
        if labels
            .iter()
            .any(|label| label[active_rows..].iter().any(|&value| value != 0))
        {
            return Err(Error::Padding);
        }
        if labels.iter().flatten().any(|&label| label >= theta) {
            return Err(Error::OutOfRange);
        }
        let len = 1usize << log_len;
        len.checked_mul(bits).ok_or(Error::Length)?;
        let labels = labels.map(|values| {
            Polys::generate_prefix(len, active_rows, 1, bits, |i, out| {
                out[0] = values.get(i).copied().unwrap_or(0);
            })
        });
        // The common d=32,kappa=4 case needs only 3*16 shared table entries.
        // Avoid exponential tables for the smaller subfields; their ordinary
        // sample construction remains exactly the existing implementation.
        let first_samples = if kappa <= 8 {
            let mixed = labels.each_ref().map(|label| {
                Polys::generate_prefix(
                    (len / kappa) * (kappa - 1),
                    active_rows.div_ceil(kappa) * (kappa - 1),
                    1,
                    domain.field.base.bits,
                    |node, out| {
                        let block = node / (kappa - 1);
                        let sample = node % (kappa - 1);
                        out[0] = (0..kappa).fold(0, |sum, j| {
                            sum ^ domain.field.multiply(
                                domain.off_weights[sample][j],
                                domain.lift(label.coefficient(block * kappa + j, 0)),
                            )
                        });
                    },
                )
            });
            let value_tables = domain
                .off_weights
                .iter()
                .map(|weights| {
                    let contributions: Vec<_> = weights
                        .iter()
                        .map(|&weight| domain.field.multiply(weight, domain.lift(theta)))
                        .collect();
                    let mut table = vec![0; 1usize << kappa];
                    for bit in 0..kappa {
                        let half = 1usize << bit;
                        for mask in 0..half {
                            table[half + mask] = table[mask] ^ contributions[bit];
                        }
                    }
                    table
                })
                .collect();
            Some(FirstSamples {
                labels: mixed,
                value_tables,
            })
        } else {
            None
        };
        Ok(Self {
            statement,
            active_rows,
            kappa,
            domain,
            labels,
            first_samples,
        })
    }

    pub fn statement(&self) -> &CommittedStatement {
        &self.statement
    }

    /// Owned public coefficient/table buffer capacities. Shared global domain
    /// and field tables, small metadata and online buffers are excluded.
    pub fn resident_bytes(&self) -> usize {
        self.labels
            .iter()
            .map(|p| p.bytes.capacity())
            .sum::<usize>()
            + self.first_samples.as_ref().map_or(0, |first| {
                first
                    .labels
                    .iter()
                    .map(|p| p.bytes.capacity())
                    .sum::<usize>()
                    + first
                        .value_tables
                        .iter()
                        .map(|t| t.capacity() * std::mem::size_of::<u128>())
                        .sum::<usize>()
            })
    }

    /// Supply the current proof's bits, including its current public-I/O
    /// values. Each source position is read exactly once; no result is cached
    /// in the plan. All returned timings measure work performed in this call.
    pub fn prepare<F>(&self, value: F) -> Result<(Prepared, CopyPreparationTimings), Error>
    where
        F: Fn(usize) -> bool + Sync + Send,
    {
        self.prepare_impl(value, true)
    }

    fn prepare_impl<F>(
        &self,
        value: F,
        use_public_samples: bool,
    ) -> Result<(Prepared, CopyPreparationTimings), Error>
    where
        F: Fn(usize) -> bool + Sync + Send,
    {
        let begin = Instant::now();
        let leaf_start = Instant::now();
        let count = self.labels[0].count;
        // One temporary bitmask stream is shared by both sources and first
        // sample construction. It is recomputed from the current input.
        let values = source_value_masks(count, self.active_rows, value);
        let leaves = self
            .labels
            .each_ref()
            .map(|label| inject_leaf_values(label, self.active_rows, &values));
        let mut timings = CopyPreparationTimings {
            leaf_ms: elapsed(leaf_start),
            ..Default::default()
        };
        let trees = leaves
            .into_iter()
            .enumerate()
            .map(|(side, leaf)| {
                let mut layers = vec![leaf];
                let mut active = self.active_rows;
                let mut samples = Vec::with_capacity(self.kappa.ilog2() as usize);
                for level in 0..self.kappa.ilog2() {
                    let previous = layers.last().unwrap();
                    let start = Instant::now();
                    let next =
                        prepare_parent_layer_prefix(previous, self.kappa, &self.domain, active);
                    timings.layers_ms += elapsed(start);
                    let start = Instant::now();
                    let sample = match self
                        .first_samples
                        .as_ref()
                        .filter(|_| level == 0 && use_public_samples)
                    {
                        Some(first) => self.first_sample_layer(first, side, &values),
                        None => prepare_parent_samples_prefix(
                            previous,
                            self.kappa,
                            &self.domain,
                            active,
                        ),
                    };
                    timings.samples_ms += elapsed(start);
                    layers.push(next);
                    samples.push(sample);
                    active = active_parent_nodes(active, self.kappa);
                }
                PreparedTree { layers, samples }
            })
            .collect::<Vec<_>>();
        let trees: [PreparedTree; 2] = trees.try_into().ok().unwrap();
        drop(values);
        timings.total_ms = elapsed(begin);
        Ok((
            Prepared {
                statement: self.statement,
                kappa: self.kappa,
                domain: self.domain.clone(),
                trees,
            },
            timings,
        ))
    }

    fn first_sample_layer(&self, first: &FirstSamples, side: usize, values: &[u8]) -> Polys {
        let kappa = self.kappa;
        debug_assert!(kappa <= 8 && kappa.is_power_of_two());
        let parent_rows = self.labels[side].count / kappa / 2;
        let value_mask = |block: usize| {
            let start = block * kappa;
            ((values[start / 8] >> (start % 8)) as usize) & ((1usize << kappa) - 1)
        };
        Polys::generate_prefix(
            parent_rows * (kappa - 1),
            self.active_rows.div_ceil(2 * kappa) * (kappa - 1),
            2,
            self.domain.field.base.bits,
            |node, out| {
                let row = node / (kappa - 1);
                let sample = node % (kappa - 1);
                let table = &first.value_tables[sample];
                let a = first.labels[side].coefficient((2 * row) * (kappa - 1) + sample, 0)
                    ^ table[value_mask(2 * row)];
                let b = first.labels[side].coefficient((2 * row + 1) * (kappa - 1) + sample, 0)
                    ^ table[value_mask(2 * row + 1)];
                out[0] = self.domain.field.multiply(a, b);
                out[1] = a ^ b;
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{bits::PackedBits, claims::row_weight, field::SmallField, packed, permutation};
    use flock_field::{F128, test_rng::Rng};
    use flock_transcript::challenger::{Challenger, FsChallenger};

    #[test]
    fn subbyte_leaf_injection_preserves_labels_and_reads_only_live_values() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        // Exercise the production leaf helpers without constructing the
        // large kappa=128 tree required by a complete one-bit-field proof.
        for bits in [1usize, 2, 4, 8] {
            for count in [1usize, 3, 7, 8, 9, 17] {
                for active in 0..=count {
                    let theta = 1u128 << (bits - 1);
                    let labels: Vec<_> = (0..count)
                        .map(|i| {
                            if i < active {
                                (i as u128 * 5 + 3) % theta
                            } else {
                                0
                            }
                        })
                        .collect();
                    let label = Polys::generate_prefix(count, active, 1, bits, |i, out| {
                        out[0] = labels[i];
                    });
                    let original = label.bytes.clone();
                    for pattern in 0..3 {
                        let live_value = |i: usize| match pattern {
                            0 => false,
                            1 => true,
                            _ => i % 3 == 1,
                        };
                        let calls: Vec<_> = (0..count).map(|_| AtomicUsize::new(0)).collect();
                        let masks = source_value_masks(count, active, |i| {
                            assert!(i < active);
                            calls[i].fetch_add(1, Ordering::Relaxed);
                            live_value(i)
                        });
                        let mut expected_masks = vec![0u8; count.div_ceil(8)];
                        for i in 0..active {
                            expected_masks[i / 8] |= u8::from(live_value(i)) << (i % 8);
                        }
                        assert_eq!(masks, expected_masks);
                        for (i, calls) in calls.iter().enumerate() {
                            assert_eq!(calls.load(Ordering::Relaxed), usize::from(i < active));
                        }
                        let leaf = inject_leaf_values(&label, active, &masks);
                        // An independent bit-by-bit oracle covers the final
                        // partial byte and all unused high/padding bits too.
                        let mut expected = vec![0u8; (count * bits).div_ceil(8)];
                        for i in 0..count {
                            let value = labels[i]
                                + if i < active && live_value(i) {
                                    theta
                                } else {
                                    0
                                };
                            for bit in 0..bits {
                                let position = i * bits + bit;
                                expected[position / 8] |=
                                    (((value >> bit) & 1) as u8) << (position % 8);
                            }
                            assert_eq!(leaf.coefficient(i, 0), value);
                        }
                        assert_eq!(leaf.bytes, expected, "bits={bits}, active={active}");
                        assert_eq!(label.bytes, original);
                    }
                }
            }
        }
    }

    fn assert_same_prepared(a: &Prepared, b: &Prepared) {
        assert_eq!(a.statement(), b.statement());
        assert_eq!(a.kappa(), b.kappa());
        for (a, b) in a.trees.iter().zip(&b.trees) {
            assert_eq!(a.layers.len(), b.layers.len());
            assert_eq!(a.samples.len(), b.samples.len());
            for (a, b) in a
                .layers
                .iter()
                .chain(&a.samples)
                .zip(b.layers.iter().chain(&b.samples))
            {
                assert_eq!(
                    (a.count, a.degree, a.bits, a.stride),
                    (b.count, b.degree, b.bits, b.stride)
                );
                assert_eq!(a.bytes, b.bytes);
            }
        }
    }

    #[test]
    fn plan_reuse_preserves_all_coefficients_proof_and_live_io() {
        let mut rng = Rng::new(0xC093_9E12);
        for width in [8usize, 16, 32, 64] {
            let kappa = 128 / width;
            // Duplicate labels fit width=8's reserved highest bit. Reverse
            // their order for a nontrivial public geometry; symmetric value
            // bits preserve the resulting two source multisets.
            for rows in [kappa * kappa, kappa * kappa + 3] {
                let live = rows - 3;
                let mask = (1u128 << (width - 1)) - 1;
                let identity: Vec<_> = (0..rows)
                    .map(|i| if i < live { (i / 2) as u128 & mask } else { 0 })
                    .collect();
                // Use identical public label multisets, differing only by an
                // explicit rotation; derive compatible value groups below.
                let mut permuted = identity.clone();
                permuted[..live].reverse();
                let labels = [permuted, identity];
                let statement = CommittedStatement::new(rows, width).unwrap();
                let plan = CopyPreparationPlan::new(statement, labels.clone(), live).unwrap();
                assert_eq!(plan.statement(), &statement);
                assert!(plan.resident_bytes() > 0);
                for iteration in 0..2 {
                    // Symmetric bits preserve the reversed-label multiset;
                    // iteration two changes witness and final public-I/O bits.
                    let mut values: Vec<_> = (0..rows).map(|_| false).collect();
                    for i in 0..live.div_ceil(2) {
                        let bit = if i == 0 || i + 1 == live.div_ceil(2) {
                            iteration != 0
                        } else {
                            rng.bit()
                        };
                        values[i] = bit;
                        values[live - 1 - i] = bit;
                    }
                    let theta = 1u128 << (width - 1);
                    let source = permutation::prepare(
                        PackedBits::from_fn_parallel(rows, width, |i| {
                            labels[0][i] | (u128::from(values[i]) * theta)
                        })
                        .unwrap(),
                        PackedBits::from_fn_parallel(rows, width, |i| {
                            labels[1][i] | (u128::from(values[i]) * theta)
                        })
                        .unwrap(),
                    )
                    .unwrap();
                    let reference = packed::prepare_impl(&source, false).unwrap();
                    let (direct, _) = plan.prepare_impl(|i| values[i], false).unwrap();
                    let (prepared, timings) = plan
                        .prepare(|i| {
                            assert!(i < live);
                            values[i]
                        })
                        .unwrap();
                    assert!(timings.total_ms >= timings.leaf_ms);
                    assert_same_prepared(&reference, &direct);
                    assert_same_prepared(&reference, &prepared);
                    let mut rc = FsChallenger::new(b"public-copy-preparation");
                    let mut pc = FsChallenger::new(b"public-copy-preparation");
                    let (rp, rq, _) = packed::prove(&reference, &mut rc).unwrap();
                    let (proof, claims, _) = packed::prove(&prepared, &mut pc).unwrap();
                    assert_eq!(
                        bincode::serialize(&rp).unwrap(),
                        bincode::serialize(&proof).unwrap()
                    );
                    assert_eq!(rq, claims);
                    let mut vc = FsChallenger::new(b"public-copy-preparation");
                    assert_eq!(packed::verify(&statement, &proof, &mut vc).unwrap(), claims);
                    let next = pc.sample_f128();
                    assert_eq!(next, rc.sample_f128());
                    assert_eq!(next, vc.sample_f128());
                    for (side, claim) in claims.iter().enumerate() {
                        let field = SmallField::for_width(width).unwrap();
                        let expected = (0..rows).fold(F128::ZERO, |sum, i| {
                            let raw = labels[side][i] | (u128::from(values[i]) * theta);
                            sum + field.embed(raw)
                                * claim.column_weights[i % kappa]
                                * row_weight(&claim.row_point, i / kappa)
                        });
                        assert_eq!(expected, claim.value);
                    }
                }
            }
        }
    }

    #[test]
    fn dead_suffix_skips_only_complete_zero_coefficient_regions() {
        let mut rng = Rng::new(0xC093_9E18);
        for width in [8usize, 16, 32, 64] {
            let kappa = 128 / width;
            let rows = (kappa * kappa).max(32);
            for active in [
                0,
                1,
                kappa - 1,
                kappa,
                kappa + 1,
                2 * kappa - 1,
                2 * kappa,
                (2 * kappa + 1).min(rows),
                rows - 1,
                rows,
            ] {
                let theta = 1u128 << (width - 1);
                let labels: Vec<_> = (0..rows)
                    .map(|i| if i < active { i as u128 % theta } else { 0 })
                    .collect();
                let values: Vec<_> = (0..rows).map(|i| i < active && rng.bit()).collect();
                let statement = CommittedStatement::new(rows, width).unwrap();
                let plan =
                    CopyPreparationPlan::new(statement, [labels.clone(), labels.clone()], active)
                        .unwrap();
                let bits = PackedBits::from_fn_parallel(rows, width, |i| {
                    labels[i] | (u128::from(values[i]) * theta)
                })
                .unwrap();
                let source = permutation::prepare(bits.clone(), bits).unwrap();
                let reference = packed::prepare_impl(&source, false).unwrap();
                let calls = std::sync::atomic::AtomicUsize::new(0);
                let (prepared, _) = plan
                    .prepare(|i| {
                        assert!(i < active);
                        calls.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        values[i]
                    })
                    .unwrap();
                assert_eq!(calls.load(std::sync::atomic::Ordering::Relaxed), active);
                assert_same_prepared(&reference, &prepared);
            }
        }
    }

    #[test]
    fn public_plan_rejects_value_coordinates_and_malformed_geometry() {
        let statement = CommittedStatement::new(16, 32).unwrap();
        assert!(CopyPreparationPlan::new(statement, [vec![0; 15], vec![0; 16]], 16).is_err());
        assert!(
            CopyPreparationPlan::new(statement, [vec![1u128 << 31; 16], vec![0; 16]], 16).is_err()
        );
        assert!(
            CopyPreparationPlan::new(statement, [vec![1u128 << 32; 16], vec![0; 16]], 16).is_err()
        );
        assert!(CopyPreparationPlan::new(statement, [vec![0; 16], vec![0; 16]], 17).is_err());
        assert!(CopyPreparationPlan::new(statement, [vec![0; 16], vec![1; 16]], 15).is_err());
        assert!(
            CopyPreparationPlan::new(
                CommittedStatement::new(16, 31).unwrap(),
                [vec![0; 16], vec![0; 16]],
                16
            )
            .is_err()
        );
    }
}
