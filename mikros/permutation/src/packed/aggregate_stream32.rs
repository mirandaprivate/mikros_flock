//! K32/kappa=4 aggregation with bounded coefficient tiles.
//!
//! Fixed off-domain interpolation is a byte-linear map. A tile reads each pair
//! of four-column child rows once, creates the three sample products, and may
//! create the parent columns in that same pass. Existing parent storage can be
//! reused instead. The highest nonmonic sample coefficient is reconstructed
//! after aggregation from the parent coefficients, so it is never materialized.

use super::{Domain, Polys, aggregate_kernel};
use crate::field::FastField;
use flock_core::zerocheck::univariate_skip::SplitEqGhash;
use flock_field::F128;
use rayon::prelude::*;

/// L_j(t) is fixed public field data. Since sum_j L_j(t)=1, interpolation is
/// x_0 + sum_{j=1}^3 L_j(t)*(x_j+x_0), requiring three byte maps, not four.
struct Interpolation {
    // [column 1..=3][byte 0..4][byte value][three off-domain points].
    tables: Vec<[[u32; 3]; 256]>,
}

impl Interpolation {
    fn new(domain: &Domain) -> Self {
        let mut tables = vec![[[0u32; 3]; 256]; 12];
        for column in 1..4 {
            for byte in 0..4 {
                let table = &mut tables[(column - 1) * 4 + byte];
                let basis: [[u32; 3]; 8] = std::array::from_fn(|bit| {
                    std::array::from_fn(|sample| {
                        domain.field.multiply(
                            domain.off_weights[sample][column],
                            1u128 << (8 * byte + bit),
                        ) as u32
                    })
                });
                for value in 1usize..256 {
                    let bit = value.trailing_zeros() as usize;
                    for sample in 0..3 {
                        table[value][sample] =
                            table[value & (value - 1)][sample] ^ basis[bit][sample];
                    }
                }
            }
        }
        Self { tables }
    }

    #[inline]
    fn pair(&self, a: &[u32; 4], b: &[u32; 4]) -> ([u32; 3], [u32; 3]) {
        let mut out_a = [a[0]; 3];
        let mut out_b = [b[0]; 3];
        for column in 1..4 {
            let da = (a[column] ^ a[0]).to_le_bytes();
            let db = (b[column] ^ b[0]).to_le_bytes();
            for byte in 0..4 {
                let table = &self.tables[(column - 1) * 4 + byte];
                let va = table[da[byte] as usize];
                let vb = table[db[byte] as usize];
                for sample in 0..3 {
                    out_a[sample] ^= va[sample];
                    out_b[sample] ^= vb[sample];
                }
            }
        }
        (out_a, out_b)
    }
}

/// Coefficients below the implicit leading one. Degree two uses three field
/// products via Karatsuba; additions and the fixed monic terms are XORs.
#[inline]
fn monic_product(a: [u32; 2], b: [u32; 2], degree: usize, field: &FastField) -> [u32; 4] {
    let p0 = field.multiply(a[0] as u128, b[0] as u128) as u32;
    if degree == 1 {
        return [p0, a[0] ^ b[0], 0, 0];
    }
    let p11 = field.multiply(a[1] as u128, b[1] as u128) as u32;
    let cross = field.multiply((a[0] ^ a[1]) as u128, (b[0] ^ b[1]) as u128) as u32;
    [p0, cross ^ p0 ^ p11, p11 ^ a[0] ^ b[0], a[1] ^ b[1]]
}

struct Scratch {
    parents: Option<Polys>,
    // These are coefficient arrays only, not monic sample polynomials: their
    // highest nonmonic term is omitted and recovered from aggregated parents.
    samples: Polys,
}

impl Scratch {
    fn new(rows: usize, degree: usize, generate_parents: bool) -> Self {
        let polys = |columns: usize, degree: usize| Polys {
            count: rows * columns,
            degree,
            bits: 32,
            stride: degree * 4,
            bytes: vec![0; rows * columns * degree * 4],
        };
        Self {
            parents: generate_parents.then(|| polys(4, 2 * degree)),
            samples: polys(3, 2 * degree - 1),
        }
    }

    fn fill(
        &mut self,
        prev: &Polys,
        start: usize,
        interpolation: &Interpolation,
        field: &FastField,
    ) {
        let degree = prev.degree;
        let rows = self.samples.count / 3;
        let row_bytes = 8 * prev.stride;
        let input = &prev.bytes[start * row_bytes..(start + rows) * row_bytes];
        for (row, input) in input.chunks_exact(row_bytes).enumerate() {
            let mut a = [[0u32; 2]; 4];
            let mut b = [[0u32; 2]; 4];
            for column in 0..4 {
                for nu in 0..degree {
                    let a_offset = (column * degree + nu) * 4;
                    let b_offset = ((4 + column) * degree + nu) * 4;
                    a[column][nu] =
                        u32::from_le_bytes(input[a_offset..a_offset + 4].try_into().unwrap());
                    b[column][nu] =
                        u32::from_le_bytes(input[b_offset..b_offset + 4].try_into().unwrap());
                }
            }
            if let Some(parents) = &mut self.parents {
                for column in 0..4 {
                    let product = monic_product(a[column], b[column], degree, field);
                    let begin = (row * 4 + column) * parents.stride;
                    for (out, coefficient) in parents.bytes[begin..begin + parents.stride]
                        .chunks_exact_mut(4)
                        .zip(product)
                    {
                        out.copy_from_slice(&coefficient.to_le_bytes());
                    }
                }
            }
            let mut mixed_a = [[0u32; 2]; 3];
            let mut mixed_b = [[0u32; 2]; 3];
            for nu in 0..degree {
                let (a, b) = interpolation.pair(&a.map(|x| x[nu]), &b.map(|x| x[nu]));
                for sample in 0..3 {
                    mixed_a[sample][nu] = a[sample];
                    mixed_b[sample][nu] = b[sample];
                }
            }
            for sample in 0..3 {
                let product = monic_product(mixed_a[sample], mixed_b[sample], degree, field);
                let begin = (row * 3 + sample) * self.samples.stride;
                for (out, coefficient) in self.samples.bytes[begin..begin + self.samples.stride]
                    .chunks_exact_mut(4)
                    .zip(product)
                {
                    out.copy_from_slice(&coefficient.to_le_bytes());
                }
            }
        }
    }
}

/// Exactly the columns and samples obtained by materializing the next layer
/// and all off-domain products, then calling `aggregate` on each. `parent`
/// optionally reuses that already-prepared next layer, avoiding recomputation.
///
/// No transcript calls, challenge restrictions, or witness-dependent pruning
/// occur here. Boolean challenge coordinates and zero-padded sources retain
/// the same equality weights as the materialized path.
pub(super) fn columns_and_samples(
    prev: &Polys,
    parent: Option<&Polys>,
    domain: &Domain,
    point: &[F128],
    powers: &[F128],
) -> (Vec<F128>, Vec<F128>) {
    assert_eq!((prev.bits, domain.kappa), (32, 4));
    assert!(matches!(prev.degree, 1 | 2));
    assert_eq!((domain.base.base.bits, domain.field.base.bits), (32, 32));
    assert_eq!(prev.stride, prev.degree * 4);
    assert_eq!(prev.bytes.len(), prev.count * prev.stride);
    assert_eq!(prev.count % 8, 0);
    assert_eq!(domain.off_weights.len(), 3);
    for weights in &domain.off_weights {
        assert_eq!(weights.len(), 4);
        assert_eq!(weights.iter().fold(0, |sum, &value| sum ^ value), 1);
    }
    let rows = prev.count / 8;
    assert!(rows.is_power_of_two());
    assert_eq!(point.len(), rows.ilog2() as usize);
    let degree = 2 * prev.degree;
    assert!(powers.len() > degree);
    if let Some(parent) = parent {
        assert_eq!(
            (parent.count, parent.degree, parent.bits),
            (rows * 4, degree, 32)
        );
        assert_eq!(parent.stride, degree * 4);
        assert_eq!(parent.bytes.len(), parent.count * parent.stride);
    }

    let eq = SplitEqGhash::with_n_hi(point, point.len() / 2);
    assert_eq!(eq.lo.len() * eq.hi.len(), rows);
    let interpolation = Interpolation::new(domain);
    let parent_plan = parent.map(|p| aggregate_kernel::Plan::new(p, 4, domain.base, &eq.lo));
    let parent_count = 4 * degree;
    let sample_count = 3 * (degree - 1);
    let jobs = rayon::current_num_threads().max(1).saturating_mul(4);
    let chunk_len = eq.hi.len().div_ceil(jobs).max(1);
    let coefficients = eq
        .hi
        .par_chunks(chunk_len)
        .enumerate()
        .map(|(chunk, hi_weights)| {
            let mut scratch = Scratch::new(eq.lo.len(), prev.degree, parent.is_none());
            let mut sums = vec![F128::ZERO; parent_count + sample_count];
            for (offset, &hi) in hi_weights.iter().enumerate() {
                let start = (chunk * chunk_len + offset) * eq.lo.len();
                scratch.fill(prev, start, &interpolation, domain.base);
                let columns = match &parent_plan {
                    Some(plan) => plan.block(start),
                    None => aggregate_kernel::Plan::new(
                        scratch.parents.as_ref().unwrap(),
                        4,
                        domain.base,
                        &eq.lo,
                    )
                    .block(0),
                };
                let samples =
                    aggregate_kernel::Plan::new(&scratch.samples, 3, domain.field, &eq.lo).block(0);
                for (sum, value) in sums.iter_mut().zip(columns.into_iter().chain(samples)) {
                    *sum += value * hi;
                }
            }
            sums
        })
        .reduce(
            || vec![F128::ZERO; parent_count + sample_count],
            |mut a, b| {
                for (a, b) in a.iter_mut().zip(b) {
                    *a += b;
                }
                a
            },
        );
    let columns = coefficients[..parent_count]
        .chunks_exact(degree)
        .map(|coefficients| {
            coefficients
                .iter()
                .zip(powers)
                .fold(powers[degree], |sum, (&coefficient, &power)| {
                    sum + coefficient * power
                })
        })
        .collect();
    let samples = coefficients[parent_count..]
        .chunks_exact(degree - 1)
        .enumerate()
        .map(|(sample, coefficients_below_top)| {
            // For monic A,B of degree d, coeff_{2d-1}(AB)=A_{d-1}+B_{d-1}.
            // Interpolation and the row-weighted aggregate are linear, so the
            // same coefficient at an off-domain point is a linear combination
            // of the four aggregated parent coefficients.
            let top = (0..4).fold(F128::ZERO, |sum, column| {
                sum + coefficients[column * degree + degree - 1]
                    * domain.field.embed(domain.off_weights[sample][column])
            });
            coefficients_below_top.iter().zip(powers).fold(
                powers[degree] + top * powers[degree - 1],
                |sum, (&coefficient, &power)| sum + coefficient * power,
            )
        })
        .collect();
    (columns, samples)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        claims::row_weight,
        packed::{aggregate, prepare_parent_layer, prepare_parent_samples},
    };
    use flock_field::test_rng::Rng;

    fn children(log_rows: usize, degree: usize, kind: usize) -> Polys {
        let count = (1usize << log_rows) * 8;
        let mut rng = Rng::new(0x3264_0189 ^ log_rows as u64 ^ degree as u64);
        let values: Vec<_> = (0..count)
            .map(|i| {
                std::array::from_fn::<_, 2, _>(|_| match kind {
                    1 => 0,
                    2 => u32::MAX as u128,
                    3 if i >= count.saturating_sub(5) => 0,
                    _ => rng.next_u64() as u32 as u128,
                })
            })
            .collect();
        Polys::generate(count, degree, 32, |i, out| {
            out.copy_from_slice(&values[i][..degree])
        })
    }

    fn powers(z: F128, degree: usize) -> Vec<F128> {
        let mut powers = vec![F128::ONE];
        for _ in 0..degree {
            powers.push(*powers.last().unwrap() * z);
        }
        powers
    }

    #[test]
    fn streamed_k32_matches_materialized_random_boolean_points_and_padding() {
        let domain = Domain::new(FastField::for_width(32).unwrap());
        let mut rng = Rng::new(0x3244_1197);
        for degree in [1, 2] {
            for log_rows in [0, 1, 6, 10] {
                for kind in 0..4 {
                    let prev = children(log_rows, degree, kind);
                    let parent = prepare_parent_layer(&prev, 4, &domain);
                    let samples = prepare_parent_samples(&prev, 4, &domain);
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
                        let powers = powers(z, 2 * degree);
                        let expected = (
                            aggregate(&parent, 4, domain.base, &point, &powers),
                            aggregate(&samples, 3, domain.field, &point, &powers),
                        );
                        for retained in [None, Some(&parent)] {
                            assert_eq!(
                                columns_and_samples(&prev, retained, &domain, &point, &powers),
                                expected,
                                "degree={degree},log_rows={log_rows},kind={kind},point_kind={point_kind},retained={}",
                                retained.is_some(),
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn streamed_k32_matches_independent_extension_field_products() {
        let domain = Domain::new(FastField::for_width(32).unwrap());
        let mut rng = Rng::new(0x3244_1297);
        for degree in [1, 2] {
            let prev = children(4, degree, 0);
            let point: Vec<_> = (0..4).map(|_| rng.f128()).collect();
            let z = rng.f128();
            let powers = powers(z, 2 * degree);
            let mut expected = [F128::ZERO; 7];
            for row in 0..prev.count / 8 {
                let weight = row_weight(&point, row);
                let input: [F128; 8] = std::array::from_fn(|column| {
                    (0..degree).fold(powers[degree], |value, nu| {
                        value
                            + domain.base.embed(prev.coefficient(row * 8 + column, nu)) * powers[nu]
                    })
                });
                for column in 0..4 {
                    expected[column] += weight * input[column] * input[4 + column];
                }
                for sample in 0..3 {
                    let a = (0..4).fold(F128::ZERO, |value, column| {
                        value
                            + input[column] * domain.field.embed(domain.off_weights[sample][column])
                    });
                    let b = (0..4).fold(F128::ZERO, |value, column| {
                        value
                            + input[4 + column]
                                * domain.field.embed(domain.off_weights[sample][column])
                    });
                    expected[4 + sample] += weight * a * b;
                }
            }
            assert_eq!(
                columns_and_samples(&prev, None, &domain, &point, &powers),
                (expected[..4].to_vec(), expected[4..].to_vec()),
            );
        }
    }

    #[test]
    fn streamed_k32_is_identical_across_worker_counts() {
        let domain = Domain::new(FastField::for_width(32).unwrap());
        let mut rng = Rng::new(0x3244_1397);
        for degree in [1, 2] {
            let prev = children(12, degree, 3);
            let parent = prepare_parent_layer(&prev, 4, &domain);
            let samples = prepare_parent_samples(&prev, 4, &domain);
            let point: Vec<_> = (0..12).map(|_| rng.f128()).collect();
            let powers = powers(rng.f128(), 2 * degree);
            let expected = (
                aggregate(&parent, 4, domain.base, &point, &powers),
                aggregate(&samples, 3, domain.field, &point, &powers),
            );
            for threads in [1, 4] {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(threads)
                    .build()
                    .unwrap();
                for retained in [None, Some(&parent)] {
                    assert_eq!(
                        pool.install(|| columns_and_samples(
                            &prev, retained, &domain, &point, &powers
                        )),
                        expected,
                        "degree={degree},threads={threads},retained={}",
                        retained.is_some(),
                    );
                }
            }
        }
    }
}
