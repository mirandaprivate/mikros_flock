//! Bounded K64/kappa=2 coefficient aggregation without whole parent/sample arrays.
//!
//! Each equality-low block is generated from its four-leaf rows once. Its two
//! parent columns and the off-domain sample's constant coefficient remain in
//! reusable worker-local storage and use the existing mixed-field dot-product
//! kernel. Its linear coefficient follows from the parent linear coefficients.
//! The high/low equality convention and implicit monic term match `aggregate`.

use super::{Domain, Polys, aggregate_kernel};
use flock_core::zerocheck::univariate_skip::SplitEqGhash;
use flock_field::F128;
use rayon::prelude::*;

struct Scratch {
    parents: Polys,
    // Only this coefficient is passed to the dot-product kernel; the sample's
    // implicit monic term and linear coefficient are reconstructed separately.
    sample_constants: Polys,
}

impl Scratch {
    fn new(rows: usize) -> Self {
        let polys = |columns: usize, degree: usize| Polys {
            count: rows * columns,
            degree,
            bits: 64,
            stride: degree * 8,
            bytes: vec![0; rows * columns * degree * 8],
        };
        Self {
            parents: polys(2, 2),
            sample_constants: polys(1, 1),
        }
    }

    fn fill(&mut self, leaves: &Polys, start: usize, domain: &Domain) {
        let rows = self.sample_constants.count;
        let source = &leaves.bytes[start * 32..(start + rows) * 32];
        // Domain::new uses nodes {0,1} and the off-domain node 2, hence
        // L1(2)=2. Multiplication by this polynomial-basis element is one
        // shift and modulus correction, rather than a general field product.
        debug_assert_eq!(domain.off_weights[0][1], 2);
        for ((input, parents), sample) in source
            .chunks_exact(32)
            .zip(self.parents.bytes.chunks_exact_mut(32))
            .zip(self.sample_constants.bytes.chunks_exact_mut(8))
        {
            let word = |i: usize| {
                u64::from_le_bytes(input[i * 8..(i + 1) * 8].try_into().unwrap()) as u128
            };
            let [a0, a1, b0, b1] = [word(0), word(1), word(2), word(3)];
            let coefficients = [
                domain.base.multiply(a0, b0),
                a0 ^ b0,
                domain.base.multiply(a1, b1),
                a1 ^ b1,
            ];
            for (out, coefficient) in parents.chunks_exact_mut(8).zip(coefficients) {
                out.copy_from_slice(&(coefficient as u64).to_le_bytes());
            }
            let a = a0 ^ domain.field.multiply_by_x(a0 ^ a1);
            let b = b0 ^ domain.field.multiply_by_x(b0 ^ b1);
            sample.copy_from_slice(&(domain.field.multiply(a, b) as u64).to_le_bytes());
        }
    }
}

/// Return the same `(columns, samples)` as materializing K64's degree-two
/// parents and off-domain samples and calling `aggregate` on each.
///
/// `point` uses low-bit-first Boolean coordinates. All row weights, including
/// zero weights at Boolean challenge coordinates, remain intact. No transcript
/// calls, global caches, or source-dependent shortcuts occur here.
pub(super) fn columns_and_samples(
    leaves: &Polys,
    domain: &Domain,
    point: &[F128],
    powers: &[F128],
) -> (Vec<F128>, Vec<F128>) {
    assert_eq!((leaves.bits, leaves.degree, leaves.stride), (64, 1, 8));
    assert_eq!(domain.kappa, 2);
    assert_eq!((domain.base.base.bits, domain.field.base.bits), (64, 64));
    assert_eq!(domain.off_weights.len(), 1);
    assert_eq!(domain.off_weights[0].len(), 2);
    assert_eq!(domain.off_weights[0][0] ^ domain.off_weights[0][1], 1);
    assert_eq!(domain.off_weights[0][1], 2);
    assert_eq!(leaves.count % 4, 0);
    assert_eq!(leaves.bytes.len(), leaves.count * 8);
    let rows = leaves.count / 4;
    assert!(rows.is_power_of_two());
    assert_eq!(point.len(), rows.ilog2() as usize);
    assert!(powers.len() >= 3);

    let eq = SplitEqGhash::with_n_hi(point, point.len() / 2);
    assert_eq!(eq.lo.len() * eq.hi.len(), rows);
    // A bounded number of jobs amortizes allocations over several high blocks.
    // At N=2^30, each scratch is 40*2^15 bytes (1.25 MiB), with one live scratch
    // per active worker. No scratch size grows linearly with the full source.
    let jobs = rayon::current_num_threads().max(1).saturating_mul(4);
    let chunk_len = eq.hi.len().div_ceil(jobs).max(1);
    let coefficients = eq
        .hi
        .par_chunks(chunk_len)
        .enumerate()
        .map(|(chunk, hi_weights)| {
            let mut scratch = Scratch::new(eq.lo.len());
            let mut sums = [F128::ZERO; 5];
            for (offset, &hi) in hi_weights.iter().enumerate() {
                let start = (chunk * chunk_len + offset) * eq.lo.len();
                scratch.fill(leaves, start, domain);
                let columns = aggregate_kernel::block(&scratch.parents, 2, domain.base, 0, &eq.lo);
                let samples =
                    aggregate_kernel::block(&scratch.sample_constants, 1, domain.field, 0, &eq.lo);
                for (sum, value) in sums.iter_mut().zip(columns.into_iter().chain(samples)) {
                    *sum += value * hi;
                }
            }
            sums
        })
        .reduce(
            || [F128::ZERO; 5],
            |mut a, b| {
                for (a, b) in a.iter_mut().zip(b) {
                    *a += b;
                }
                a
            },
        );
    let evaluate =
        |i: usize| powers[2] + coefficients[i] * powers[0] + coefficients[i + 1] * powers[1];
    // With u0=a0+b0 and u1=a1+b1, the sample's linear coefficient is
    // u0 + 2*(u0+u1). Embedding preserves this identity, including after the
    // common extension-field row weights. Reconstruct it after aggregation
    // instead of embedding and accumulating a sixth coefficient in every row.
    let sample_linear =
        coefficients[1] + domain.field.embed(2) * (coefficients[1] + coefficients[3]);
    let sample = powers[2] + coefficients[4] * powers[0] + sample_linear * powers[1];
    (vec![evaluate(0), evaluate(2)], vec![sample])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bits::PackedBits,
        claims::row_weight,
        field::FastField,
        packed::{aggregate, prepare_parent_layer, prepare_parent_samples},
        permutation,
    };
    use flock_field::test_rng::Rng;
    use flock_transcript::challenger::{Challenger, FsChallenger};

    fn leaves(log_rows: usize, kind: usize) -> Polys {
        let count = 1usize << (log_rows + 2);
        let mut rng = Rng::new(0x6432_7189 ^ log_rows as u64);
        let values: Vec<u128> = (0..count)
            .map(|i| match kind {
                0 => rng.next_u64() as u128,
                1 => 0,
                2 => u64::MAX as u128,
                // Binary copy tags near the top of a 2^32-cell copy domain.
                4 => {
                    ((1u128 << 32) - count as u128 + i as u128)
                        | ((rng.next_u64() as u128 & 1) << 63)
                }
                // Exercise partial live groups and the zero-padded suffix.
                _ if i < count.saturating_sub(3) => rng.next_u64() as u128,
                _ => 0,
            })
            .collect();
        Polys::generate(count, 1, 64, |i, out| out[0] = values[i])
    }

    fn materialized(
        leaves: &Polys,
        domain: &Domain,
        point: &[F128],
        powers: &[F128],
    ) -> (Vec<F128>, Vec<F128>) {
        let parents = prepare_parent_layer(leaves, 2, domain);
        let samples = prepare_parent_samples(leaves, 2, domain);
        (
            aggregate(&parents, 2, domain.base, point, powers),
            aggregate(&samples, 1, domain.field, point, powers),
        )
    }

    #[test]
    fn streamed_k64_matches_materialized_random_boolean_points_and_padding() {
        let domain = Domain::new(FastField::for_width(64).unwrap());
        let mut rng = Rng::new(0x6432_8197);
        for log_rows in [0, 1, 2, 3, 6, 10, 14] {
            for kind in 0..5 {
                let leaves = leaves(log_rows, kind);
                for point_kind in 0..4 {
                    let point: Vec<_> = (0..log_rows)
                        .map(|i| match point_kind {
                            0 => rng.f128(),
                            1 => F128::ZERO,
                            2 => F128::ONE,
                            _ if i % 2 == 0 => F128::ZERO,
                            _ => F128::ONE,
                        })
                        .collect();
                    let z = match point_kind {
                        1 => F128::ZERO,
                        2 => F128::ONE,
                        _ => rng.f128(),
                    };
                    let powers = [F128::ONE, z, z.square()];
                    assert_eq!(
                        columns_and_samples(&leaves, &domain, &point, &powers),
                        materialized(&leaves, &domain, &point, &powers),
                        "log_rows={log_rows}, kind={kind}, point_kind={point_kind}"
                    );
                }
            }
        }
    }

    #[test]
    fn streamed_k64_matches_independent_extension_field_products() {
        let domain = Domain::new(FastField::for_width(64).unwrap());
        let leaves = leaves(5, 0);
        let mut rng = Rng::new(0x6432_9197);
        let point: Vec<_> = (0..5).map(|_| rng.f128()).collect();
        let z = rng.f128();
        let mut expected = [F128::ZERO; 3];
        let off: Vec<_> = domain.off_weights[0]
            .iter()
            .map(|&x| domain.field.base.embed(x))
            .collect();
        for row in 0..leaves.count / 4 {
            let weight = row_weight(&point, row);
            let input: [F128; 4] =
                std::array::from_fn(|j| domain.base.base.embed(leaves.coefficient(4 * row + j, 0)));
            expected[0] += weight * (input[0] + z) * (input[2] + z);
            expected[1] += weight * (input[1] + z) * (input[3] + z);
            expected[2] += weight
                * (off[0] * input[0] + off[1] * input[1] + z)
                * (off[0] * input[2] + off[1] * input[3] + z);
        }
        assert_eq!(
            columns_and_samples(&leaves, &domain, &point, &[F128::ONE, z, z.square()]),
            (expected[..2].to_vec(), expected[2..].to_vec())
        );
    }

    #[test]
    fn streamed_k64_is_identical_across_worker_counts() {
        let domain = Domain::new(FastField::for_width(64).unwrap());
        let leaves = leaves(12, 3);
        let mut rng = Rng::new(0x6432_a197);
        let point: Vec<_> = (0..12).map(|_| rng.f128()).collect();
        let z = rng.f128();
        let powers = [F128::ONE, z, z.square()];
        let expected = materialized(&leaves, &domain, &point, &powers);
        for threads in [1, 4] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            assert_eq!(
                pool.install(|| columns_and_samples(&leaves, &domain, &point, &powers)),
                expected,
                "threads={threads}"
            );
        }
    }

    #[test]
    fn five_coefficient_stream_preserves_full_proof_bytes_claims_and_transcript() {
        use crate::packed::{prepare_bounded, prove_bounded, prove_bounded_compact, verify};

        let mut rng = Rng::new(0x6432_b197);
        for (rows, width, sparse) in [(256, 64, false), (259, 64, true), (259, 33, true)] {
            let values: Vec<_> = (0..rows)
                .map(|i| {
                    if !sparse {
                        rng.next_u64() as u128
                    } else {
                        // Sparse binary-plus-position tags, including the high
                        // label bit used by the 2^32-cell copy argument.
                        let label = (1u128 << 32) - rows as u128 + i as u128;
                        let bit = rng.next_u64() as u128 & 1;
                        if width == 64 {
                            label | (bit << 63)
                        } else {
                            (label << 1) | bit
                        }
                    }
                })
                .collect();
            let reversed: Vec<_> = values.iter().copied().rev().collect();
            let source = permutation::prepare(
                PackedBits::from_rows(&values, width).unwrap(),
                PackedBits::from_rows(&reversed, width).unwrap(),
            )
            .unwrap();
            // The ordinary bounded path retains the original materialized
            // six-coefficient aggregate as an independent protocol reference.
            let mut reference_ch = FsChallenger::new(b"k64-five-coefficient-stream");
            let (reference, reference_claims, _) =
                prove_bounded(prepare_bounded(&source).unwrap(), &mut reference_ch).unwrap();
            let mut streamed_ch = FsChallenger::new(b"k64-five-coefficient-stream");
            let (streamed, streamed_claims, _) =
                prove_bounded_compact(prepare_bounded(&source).unwrap(), &mut streamed_ch).unwrap();
            assert_eq!(
                bincode::serialize(&streamed).unwrap(),
                bincode::serialize(&reference).unwrap(),
                "rows={rows}, width={width}"
            );
            assert_eq!(streamed_claims, reference_claims);
            let mut verifier_ch = FsChallenger::new(b"k64-five-coefficient-stream");
            assert_eq!(
                verify(source.statement(), &streamed, &mut verifier_ch).unwrap(),
                streamed_claims
            );
            let next = reference_ch.sample_f128();
            assert_eq!(streamed_ch.sample_f128(), next);
            assert_eq!(verifier_ch.sample_f128(), next);
        }
    }
}
