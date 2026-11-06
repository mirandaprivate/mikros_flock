//! Online preparation directly from physical source rows.
//!
//! This writes the existing little-endian coefficient layout without an
//! intermediate column-major PackedBits allocation or a subsequent SWAR decode.
//! The caller must charge the closure and all of this preparation online.

use rayon::prelude::*;

use super::{
    BoundedPrepared, CommittedStatement, Error, Polys, Prepared, PreparedTree, domain,
    prepare_parent_layer, shape,
};

const CHUNK_BYTES: usize = 64 * 1024;

/// Prepare both source polynomials, evaluating each real row exactly once.
///
/// `row(i)` supplies the two unsigned logical-width values at position `i`.
/// It is not called for padding. Out-of-range values fail instead of silently
/// truncating into the enclosing power-of-two subfield.
pub fn prepare_bounded_from_fn<F>(
    statement: CommittedStatement,
    row: F,
) -> Result<BoundedPrepared, Error>
where
    F: Fn(usize) -> [u128; 2] + Sync + Send,
{
    let (kappa, _) = shape(&statement)?;
    let domain = domain(statement.width)?;
    let count = crate::padded_len(statement.rows)?;
    let bits = domain.base.base.bits;
    let byte_len = count.checked_mul(bits).ok_or(Error::Length)?.div_ceil(8);
    // Match Polys' zeroed allocation: on supporting allocators this can use
    // lazy zero pages. reserve + resize would serially touch the entire source
    // before the parallel row writer, including tens of GiB for K64.
    let mut left = vec![0u8; byte_len];
    let mut right = vec![0u8; byte_len];

    left.par_chunks_mut(CHUNK_BYTES)
        .zip(right.par_chunks_mut(CHUNK_BYTES))
        .enumerate()
        .try_for_each(|(block, (left, right))| {
            if bits < 8 {
                let per_byte = 8 / bits;
                let first = block * CHUNK_BYTES * per_byte;
                for (byte, (left, right)) in left.iter_mut().zip(right).enumerate() {
                    for lane in 0..per_byte {
                        let index = first + byte * per_byte + lane;
                        if index >= statement.rows {
                            break;
                        }
                        let values = row(index);
                        if values.iter().any(|&value| value >> statement.width != 0) {
                            return Err(Error::OutOfRange);
                        }
                        *left |= (values[0] as u8) << (lane * bits);
                        *right |= (values[1] as u8) << (lane * bits);
                    }
                }
            } else {
                let stride = bits / 8;
                let first = block * (CHUNK_BYTES / stride);
                for (within, (left, right)) in left
                    .chunks_exact_mut(stride)
                    .zip(right.chunks_exact_mut(stride))
                    .enumerate()
                {
                    let index = first + within;
                    if index >= statement.rows {
                        break;
                    }
                    let values = row(index);
                    // shape() restricts this protocol to fields at most K64,
                    // so the shift is always defined, including width zero.
                    if values.iter().any(|&value| value >> statement.width != 0) {
                        return Err(Error::OutOfRange);
                    }
                    left.copy_from_slice(&values[0].to_le_bytes()[..stride]);
                    right.copy_from_slice(&values[1].to_le_bytes()[..stride]);
                }
            }
            Ok(())
        })?;

    // The leaves have the same zero padding, degree, and physical layout as
    // Polys::leaves. Reuse the original parent construction without caching
    // off-domain samples, exactly as prepare_bounded does.
    let trees = [left, right].map(|bytes| {
        let mut layers = vec![Polys {
            count,
            degree: 1,
            bits,
            stride: bits.div_ceil(8),
            bytes,
        }];
        for _ in 0..kappa.ilog2() {
            let parent = prepare_parent_layer(layers.last().unwrap(), kappa, &domain);
            layers.push(parent);
        }
        PreparedTree {
            layers,
            samples: Vec::new(),
        }
    });
    Ok(BoundedPrepared(Prepared {
        statement,
        kappa,
        domain,
        trees,
    }))
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use flock_field::F128;
    use flock_transcript::challenger::{Challenger, FsChallenger};

    use super::*;
    use crate::{bits::PackedBits, claims::row_weight, field::SmallField, permutation};

    fn values(rows: usize, width: usize) -> Vec<u128> {
        let mask = (1u128 << width) - 1;
        (0..rows)
            .map(|index| {
                let x = (index as u64 + 17).wrapping_mul(0x9e3779b97f4a7c15);
                u128::from(x ^ x.rotate_left(29)) & mask
            })
            .collect()
    }

    fn legacy(left: &[u128], right: &[u128], width: usize) -> BoundedPrepared {
        let source = permutation::prepare(
            PackedBits::from_rows(left, width).unwrap(),
            PackedBits::from_rows(right, width).unwrap(),
        )
        .unwrap();
        super::super::prepare_bounded(&source).unwrap()
    }

    fn assert_identical(actual: &BoundedPrepared, expected: &BoundedPrepared) {
        assert_eq!(actual.statement(), expected.statement());
        assert_eq!(actual.0.kappa, expected.0.kappa);
        assert!(actual.resident_bytes() <= expected.resident_bytes());
        for (actual, expected) in actual.0.trees.iter().zip(&expected.0.trees) {
            assert!(actual.samples.is_empty());
            assert!(expected.samples.is_empty());
            assert_eq!(actual.layers.len(), expected.layers.len());
            for (actual, expected) in actual.layers.iter().zip(&expected.layers) {
                assert_eq!(
                    (actual.count, actual.degree, actual.bits, actual.stride),
                    (
                        expected.count,
                        expected.degree,
                        expected.bits,
                        expected.stride
                    )
                );
                assert_eq!(actual.bytes, expected.bytes);
            }
        }
    }

    #[test]
    fn direct_rows_match_all_prepared_coefficients_and_call_once() {
        for width in [0usize, 1, 2, 3, 4, 7, 8, 9, 16, 17, 32, 33, 64] {
            let bits = width.max(1).next_power_of_two();
            let kappa = 128 / bits;
            for rows in [kappa * kappa, kappa * kappa + 3] {
                let left = values(rows, width);
                let right: Vec<_> = left.iter().copied().rev().collect();
                let calls: Vec<_> = (0..rows).map(|_| AtomicUsize::new(0)).collect();
                let actual = prepare_bounded_from_fn(CommittedStatement { rows, width }, |index| {
                    assert!(index < rows, "closure must never see padded rows");
                    calls[index].fetch_add(1, Ordering::Relaxed);
                    [left[index], right[index]]
                })
                .unwrap();
                let expected = legacy(&left, &right, width);
                assert_identical(&actual, &expected);
                assert!(calls.iter().all(|count| count.load(Ordering::Relaxed) == 1));
                for tree in &actual.0.trees {
                    for index in rows..tree.layers[0].count {
                        assert_eq!(tree.layers[0].coefficient(index, 0), 0);
                    }
                }
            }
        }
    }

    #[test]
    fn direct_rows_cross_chunks_preserve_proofs_terminals_and_transcript() {
        for threads in [1, 4] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap()
                .install(|| {
                    // K64's 64 KiB task contains 8192 tags; the last case crosses
                    // that boundary and has a non-power-of-two source length.
                    for (rows, width) in [(259, 8), (67, 16), (67, 33), (64, 64), (8195, 64)] {
                        let left = values(rows, width);
                        let right: Vec<_> = left.iter().copied().rev().collect();
                        let statement = CommittedStatement { rows, width };
                        let actual =
                            prepare_bounded_from_fn(statement, |i| [left[i], right[i]]).unwrap();
                        let expected = legacy(&left, &right, width);
                        assert_identical(&actual, &expected);
                        let mut actual_ch = FsChallenger::new(b"direct-row-preparation");
                        let mut expected_ch = FsChallenger::new(b"direct-row-preparation");
                        let (actual, claims, _) =
                            super::super::prove_bounded_compact(actual, &mut actual_ch).unwrap();
                        let (expected, expected_claims, _) =
                            super::super::prove_bounded_compact(expected, &mut expected_ch)
                                .unwrap();
                        assert_eq!(
                            bincode::serialize(&actual).unwrap(),
                            bincode::serialize(&expected).unwrap()
                        );
                        assert_eq!(claims, expected_claims);
                        let mut verifier = FsChallenger::new(b"direct-row-preparation");
                        assert_eq!(
                            super::super::verify(&statement, &actual, &mut verifier).unwrap(),
                            claims
                        );
                        let next = actual_ch.sample_f128();
                        assert_eq!(next, expected_ch.sample_f128());
                        assert_eq!(next, verifier.sample_f128());
                        let field = SmallField::for_width(width).unwrap();
                        let kappa = 128 / field.bits;
                        for (raw, claim) in [&left, &right].into_iter().zip(&claims) {
                            let direct = raw.iter().enumerate().fold(F128::ZERO, |sum, (i, &v)| {
                                sum + field.embed(v)
                                    * claim.column_weights[i % kappa]
                                    * row_weight(&claim.row_point, i / kappa)
                            });
                            assert_eq!(direct, claim.value);
                        }
                    }
                });
        }
    }

    #[test]
    fn direct_rows_reject_invalid_shapes_and_logical_width_overflow() {
        for statement in [
            CommittedStatement { rows: 0, width: 64 },
            CommittedStatement { rows: 1, width: 64 },
            CommittedStatement {
                rows: 64,
                width: 129,
            },
            CommittedStatement {
                rows: 64,
                width: 65,
            },
            CommittedStatement {
                rows: (1usize << crate::MAX_LOG_ROWS) + 1,
                width: 64,
            },
        ] {
            assert!(
                prepare_bounded_from_fn(statement, |_| panic!("invalid shape called source"))
                    .is_err()
            );
        }
        for width in [0usize, 1, 3, 7, 9, 17, 33, 64] {
            let kappa = 128 / width.max(1).next_power_of_two();
            let rows = (kappa * kappa).max(8195);
            for side in 0..2 {
                let result = prepare_bounded_from_fn(CommittedStatement { rows, width }, |index| {
                    let mut values = [0; 2];
                    if index == rows - 1 {
                        values[side] = 1u128 << width;
                    }
                    values
                });
                assert!(
                    matches!(result, Err(Error::OutOfRange)),
                    "width={width}, side={side}"
                );
            }
        }
    }
}
