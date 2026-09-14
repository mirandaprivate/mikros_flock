//! Four-row accumulation with the same unreduced products as the scalar oracle.
use super::Polys;
use crate::field::FastField;
use flock_field::{F128, F256Unreduced};

#[derive(Clone, Copy)]
pub(super) enum Kernel {
    Baseline,
    Gfni,
    Tower,
}

impl Kernel {
    #[cfg(test)]
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Baseline => "baseline",
            Self::Gfni => "gfni",
            Self::Tower => "tower",
        }
    }
}

/// The weight conversion is prepared once online and shared by all hi blocks.
pub(super) struct Plan<'a> {
    polys: &'a Polys,
    columns: usize,
    field: &'a FastField,
    weights: &'a [F128],
    kernel: Kernel,
    tower: Option<super::aggregate_tower::Weights>,
}

impl<'a> Plan<'a> {
    pub(super) fn new(
        polys: &'a Polys,
        columns: usize,
        field: &'a FastField,
        weights: &'a [F128],
    ) -> Self {
        let kernel = match std::env::var("MIKROS_PACKED_AGGREGATE").as_deref() {
            Ok("gfni") => Kernel::Gfni,
            Ok("tower") => Kernel::Tower,
            Ok("baseline") => Kernel::Baseline,
            Ok("auto") | Err(_) => {
                // The native probe covers all four K32 coefficient shapes.
                // Keep tiny equality blocks on the original implementation;
                // GFNI's scalar bit-linear tail is not useful below one tile.
                if weights.len() >= 64 {
                    Kernel::Gfni
                } else {
                    Kernel::Baseline
                }
            }
            Ok(value) => panic!("unsupported MIKROS_PACKED_AGGREGATE={value}"),
        };
        Self::with_kernel(polys, columns, field, weights, kernel)
    }

    pub(super) fn with_kernel(
        polys: &'a Polys,
        columns: usize,
        field: &'a FastField,
        weights: &'a [F128],
        kernel: Kernel,
    ) -> Self {
        let tower = if matches!(kernel, Kernel::Tower) && polys.bits == 32 && field.base.bits == 32
        {
            super::aggregate_tower::prepare(weights)
        } else {
            None
        };
        Self {
            polys,
            columns,
            field,
            weights,
            kernel,
            tower,
        }
    }

    pub(super) fn block(&self, start: usize) -> Vec<F128> {
        assert!((start + self.weights.len()) * self.columns <= self.polys.count);
        if self.polys.bits == 32 && self.field.base.bits == 32 {
            let count = self.columns * self.polys.degree;
            let first = start * count * 4;
            let bytes = &self.polys.bytes[first..first + self.weights.len() * count * 4];
            match self.kernel {
                Kernel::Gfni => {
                    if let Some(result) = super::aggregate_gfni::block(
                        bytes,
                        count,
                        self.field.base.basis(),
                        self.weights,
                    ) {
                        return result;
                    }
                }
                Kernel::Tower => {
                    if let Some(weights) = &self.tower {
                        return super::aggregate_tower::block(bytes, count, weights);
                    }
                }
                Kernel::Baseline => {}
            }
        }
        block(self.polys, self.columns, self.field, start, self.weights)
    }
}

pub(super) fn block(
    polys: &Polys,
    columns: usize,
    field: &FastField,
    start: usize,
    weights: &[F128],
) -> Vec<F128> {
    assert!((start + weights.len()) * columns <= polys.count);
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq"
    ))]
    if weights.len() >= 4 {
        // SAFETY: the build enables the native instruction set and all source
        // accesses are bounded by the assertion above and the four-row loop.
        return unsafe { native(polys, columns, field, start, weights) };
    }
    scalar(polys, columns, field, start, weights)
}

fn scalar(
    polys: &Polys,
    columns: usize,
    field: &FastField,
    start: usize,
    weights: &[F128],
) -> Vec<F128> {
    let mut sums = vec![F256Unreduced::ZERO; columns * polys.degree];
    for (i, &weight) in weights.iter().enumerate() {
        for column in 0..columns {
            for nu in 0..polys.degree {
                let value = polys.coefficient((start + i) * columns + column, nu);
                if value != 0 {
                    sums[column * polys.degree + nu] ^= field.embed(value).mul_unreduced(weight);
                }
            }
        }
    }
    sums.into_iter().map(F256Unreduced::reduce).collect()
}

#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512f",
    target_feature = "vpclmulqdq"
))]
#[target_feature(enable = "avx512f,vpclmulqdq,sse4.1")]
unsafe fn native(
    polys: &Polys,
    columns: usize,
    field: &FastField,
    start: usize,
    weights: &[F128],
) -> Vec<F128> {
    use flock_field::gf2_128::x86_64::{WideGhashX4, f128x4_set};
    // SAFETY: the caller enables these features; the accumulator has no memory
    // accesses and is copied as an initialized SIMD value.
    let mut sums = vec![unsafe { WideGhashX4::zero() }; columns * polys.degree];
    let full = weights.len() / 4 * 4;
    for i in (0..full).step_by(4) {
        // SAFETY: i+4<=weights.len(), and f128x4_set only assembles values.
        let weight =
            unsafe { f128x4_set(weights[i], weights[i + 1], weights[i + 2], weights[i + 3]) };
        for column in 0..columns {
            for nu in 0..polys.degree {
                let values = std::array::from_fn::<_, 4, _>(|lane| {
                    field.embed(polys.coefficient((start + i + lane) * columns + column, nu))
                });
                // SAFETY: all four initialized values are assembled in the
                // same lane order as weights and used by the native accumulator.
                unsafe {
                    let value = f128x4_set(values[0], values[1], values[2], values[3]);
                    sums[column * polys.degree + nu].mul_acc(value, weight);
                }
            }
        }
    }
    let mut sums: Vec<_> = sums
        .into_iter()
        .map(|sum| {
            // SAFETY: this function carries avx512f+sse4.1. fold XORs the lanes
            // before reduction; it does not introduce a different field operation.
            unsafe { sum.fold() }
        })
        .collect();
    for i in full..weights.len() {
        for column in 0..columns {
            for nu in 0..polys.degree {
                let value = polys.coefficient((start + i) * columns + column, nu);
                if value != 0 {
                    sums[column * polys.degree + nu] ^=
                        field.embed(value).mul_unreduced(weights[i]);
                }
            }
        }
    }
    sums.into_iter().map(F256Unreduced::reduce).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::zerocheck::univariate_skip::SplitEqGhash;

    fn compare(native_path: bool) {
        for bits in [1, 8, 32] {
            let field = FastField::for_width(bits).unwrap();
            for dimension in [0usize, 1, 2, 3, 8, 13] {
                let point: Vec<_> = (0..dimension)
                    .map(|i| {
                        F128::new((i as u64 + 31) * 0xabc123, ((i as u64 + 79) * 0xfa71) << 13)
                    })
                    .collect();
                let eq = SplitEqGhash::with_n_hi(&point, dimension / 2);
                let rows = eq.lo.len() + 9;
                let columns = 3;
                let polys = Polys::generate(rows * columns, 4, bits, |node, out| {
                    for (j, x) in out.iter_mut().enumerate() {
                        *x = ((node as u128 * 0x771_53c1)
                            ^ (j as u128 * 0x434_9873)
                            ^ (node as u128 >> 2))
                            & field.base.mask();
                    }
                });
                for start in [0usize, 1, 5] {
                    for count in [0usize, 1, 2, 3, 4, 5, 7, 9, eq.lo.len()] {
                        let n = count.min(eq.lo.len());
                        let weights = &eq.lo[..n];
                        let expected = scalar(&polys, columns, field, start, weights);
                        let actual = if native_path {
                            #[cfg(all(
                                target_arch = "x86_64",
                                target_feature = "avx512f",
                                target_feature = "vpclmulqdq"
                            ))]
                            // SAFETY: the test is compiled only for the native
                            // instruction set and generated rows cover all tails.
                            {
                                unsafe { native(&polys, columns, field, start, weights) }
                            }
                            #[cfg(not(all(
                                target_arch = "x86_64",
                                target_feature = "avx512f",
                                target_feature = "vpclmulqdq"
                            )))]
                            {
                                unreachable!("native test is feature-gated")
                            }
                        } else {
                            block(&polys, columns, field, start, weights)
                        };
                        assert_eq!(
                            actual, expected,
                            "bits={bits},dim={dimension},start={start},n={n}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn packed_aggregate_dispatch_matches_scalar_fields_tails_and_point_boundaries() {
        compare(false);
    }

    #[test]
    fn packed_aggregate_plans_match_scalar_at_nonzero_starts_and_native_tails() {
        let weights: Vec<_> = (0..65)
            .map(|i| match i {
                0 => F128::ONE,
                1 => F128::ZERO,
                _ => F128::new(
                    (i as u64 + 31).wrapping_mul(0x9e37_79b9_7f4a_7c15),
                    (i as u64 + 79).wrapping_mul(0xbf58_476d_1ce4_e5b9),
                ),
            })
            .collect();
        for bits in [1, 8, 32] {
            let field = FastField::for_width(bits).unwrap();
            for (columns, degree) in [(3, 2), (4, 2), (3, 4), (4, 4)] {
                // The largest subrange ends exactly at the final source row.
                let polys = Polys::generate(70 * columns, degree, bits, |node, out| {
                    for (coefficient, value) in out.iter_mut().enumerate() {
                        let mut x = (node as u64 + 17).wrapping_mul(0x9e37_79b9_7f4a_7c15)
                            ^ (coefficient as u64 + 3).rotate_left(11);
                        x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
                        x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
                        *value = ((x ^ (x >> 31)) as u128) & field.base.mask();
                    }
                });
                for start in [1, 5] {
                    // 65 covers one GFNI tile plus its scalar tail; 1 also
                    // exercises a tower block containing only a partial group.
                    for length in [0, 1, 4, 65] {
                        let weights = &weights[..length];
                        let expected = scalar(&polys, columns, field, start, weights);
                        for kernel in [Kernel::Baseline, Kernel::Gfni, Kernel::Tower] {
                            let plan = Plan::with_kernel(&polys, columns, field, weights, kernel);
                            assert_eq!(
                                plan.block(start),
                                expected,
                                "kernel={},bits={bits},columns={columns},degree={degree},start={start},length={length}",
                                kernel.name(),
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq"
    ))]
    fn packed_aggregate_native_matches_scalar_fields_tails_and_point_boundaries() {
        compare(true);
    }
}

#[cfg(test)]
#[path = "aggregate_bench.rs"]
mod bench;
