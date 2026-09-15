use super::fixture::{basis, from_raw};
use flock_core::{
    pcs::{
        functional::{TensorClaim, row_weights},
        ring_switch::{claim_check, fold_1b_rows_naive, fold_1b_rows_split},
    },
    zerocheck::PaddingSpec,
};
use flock_field::{F128, F256Unreduced};
use rayon::prelude::*;

/// The same terminal-evaluation choices are used by Mikros and FLOCK.
/// These affect only prover work, never the returned query or transcript.
#[derive(Clone, Copy)]
pub struct CopyEvaluation {
    pub ordinary_reference: bool,
    pub label_reference: bool,
}

impl CopyEvaluation {
    pub fn from_env() -> Self {
        let choice = |key: &str, fast: &str| match std::env::var(key) {
            Ok(value) if value == "reference" => true,
            Ok(value) if value == fast => false,
            Err(std::env::VarError::NotPresent) => false,
            value => panic!("invalid {key}: {value:?}; expected {fast} or reference"),
        };
        Self {
            ordinary_reference: choice("E2E_ORDINARY_EVAL", "dot"),
            label_reference: choice("E2E_PUBLIC_LABEL_FOLD", "shared"),
        }
    }
}

pub fn pairs(point: &[F128]) -> Vec<[F128; 2]> {
    point.iter().map(|&r| [F128::ONE + r, r]).collect()
}
pub fn tensor(point: &[F128]) -> Vec<F128> {
    row_weights(&pairs(point))
}
pub fn matrix(q: mikros::MatrixClaim) -> TensorClaim {
    TensorClaim {
        columns: q.column_weights,
        rows: q.row_factors.into_iter().rev().collect(),
        value: q.value,
    }
}
pub fn ordinary(point: &[F128], value: F128) -> TensorClaim {
    TensorClaim {
        columns: basis(),
        rows: pairs(point),
        value,
    }
}
/// Restrict a block, retaining its external scale on the column functional.
pub fn lift(mut q: TensorClaim, block: usize, log_total_words: usize) -> TensorClaim {
    let old = q.rows.len();
    assert!(old <= log_total_words);
    assert!(block < 1usize << (log_total_words - old));
    q.rows.extend((0..log_total_words - old).map(|i| {
        if block & (1 << i) == 0 {
            [F128::ONE, F128::ZERO]
        } else {
            [F128::ZERO, F128::ONE]
        }
    }));
    q
}
pub fn scale(mut q: TensorClaim, a: F128) -> TensorClaim {
    for x in &mut q.columns {
        *x *= a;
    }
    q.value *= a;
    q
}
/// The 128 bit-column evaluations for one common row functional. Several
/// column queries may reuse this result without rescanning the packed words.
pub fn fold_columns(words: &[F128], rows: &[[F128; 2]]) -> Vec<F128> {
    assert_eq!(words.len(), 1usize << rows.len());
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

pub fn evaluate(words: &[F128], q: &TensorClaim) -> F128 {
    assert_eq!(q.columns.len(), 128);
    claim_check(&q.columns, &fold_columns(words, &q.rows))
}

/// Exact F128 dot product. The four-lane accumulator delays reduction until
/// the end of a block; scalar tails use the same unreduced representation.
fn dot_block(words: &[F128], weights: &[F128]) -> F128 {
    assert_eq!(words.len(), weights.len());
    let mut sum = F256Unreduced::ZERO;
    #[allow(unused_mut)]
    let mut i = 0;
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq",
        target_feature = "sse4.1"
    ))]
    // SAFETY: the compile-time gate covers every required ISA feature. Each
    // iteration loads four in-bounds values from each possibly unaligned slice.
    unsafe {
        use flock_field::gf2_128::x86_64::{WideGhashX4, f128x4_loadu};
        let mut wide = WideGhashX4::zero();
        while i + 4 <= words.len() {
            wide.mul_acc(
                f128x4_loadu(words.as_ptr().add(i)),
                f128x4_loadu(weights.as_ptr().add(i)),
            );
            i += 4;
        }
        sum ^= wide.fold();
    }
    for (&word, &weight) in words[i..].iter().zip(&weights[i..]) {
        sum ^= word.mul_unreduced(weight);
    }
    sum.reduce()
}

/// Evaluate two independent dots while loading each witness word once. The
/// accumulators and weights remain separate even when the two points coincide.
fn paired_dot_block(words: &[F128], weights: [&[F128]; 2]) -> [F128; 2] {
    assert!(weights.iter().all(|weight| weight.len() == words.len()));
    let mut sums = [F256Unreduced::ZERO; 2];
    #[allow(unused_mut)]
    let mut i = 0;
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq",
        target_feature = "sse4.1"
    ))]
    // SAFETY: the ISA is compile-time gated. Every load reads four in-bounds
    // elements; all inputs may be unaligned. Only the witness load is shared.
    unsafe {
        use flock_field::gf2_128::x86_64::{WideGhashX4, f128x4_loadu};
        let mut wide = [WideGhashX4::zero(), WideGhashX4::zero()];
        while i + 4 <= words.len() {
            let word = f128x4_loadu(words.as_ptr().add(i));
            for side in 0..2 {
                wide[side].mul_acc(word, f128x4_loadu(weights[side].as_ptr().add(i)));
            }
            i += 4;
        }
        for (sum, accumulator) in sums.iter_mut().zip(wide) {
            *sum ^= accumulator.fold();
        }
    }
    for (index, &word) in words.iter().enumerate().skip(i) {
        for side in 0..2 {
            sums[side] ^= word.mul_unreduced(weights[side][index]);
        }
    }
    sums.map(F256Unreduced::reduce)
}

/// Small equality tensors for two independent ordinary MLE points. The copy
/// prover reuses these tables for all three wires; no full row table is stored.
pub struct PairedOrdinaryEvaluation {
    lo: [Vec<F128>; 2],
    hi: [Vec<F128>; 2],
    len: usize,
}

impl PairedOrdinaryEvaluation {
    pub fn new(points: [&[F128]; 2]) -> Self {
        assert_eq!(points[0].len(), points[1].len());
        let split = points[0].len().div_ceil(2);
        Self {
            lo: points.map(|point| row_weights(&pairs(&point[..split]))),
            hi: points.map(|point| row_weights(&pairs(&point[split..]))),
            len: 1usize << points[0].len(),
        }
    }

    pub fn evaluate(&self, words: &[F128], scales: [F128; 2]) -> [F128; 2] {
        assert_eq!(words.len(), self.len);
        let block = |(i, words): (usize, &[F128])| {
            let sums = paired_dot_block(words, [&self.lo[0], &self.lo[1]]);
            std::array::from_fn(|side| sums[side] * self.hi[side][i])
        };
        let add = |a: [F128; 2], b: [F128; 2]| [a[0] + b[0], a[1] + b[1]];
        let sums = if words.len() >= 1 << 15 {
            words
                .par_chunks(self.lo[0].len())
                .enumerate()
                .map(block)
                .reduce(|| [F128::ZERO; 2], add)
        } else {
            words
                .chunks(self.lo[0].len())
                .enumerate()
                .map(block)
                .fold([F128::ZERO; 2], add)
        };
        // Retain both block-prefix scales, including zero, without dividing.
        std::array::from_fn(|side| scales[side] * sums[side])
    }
}

/// When columns are the fixed bit basis, linearity identifies the bit-matrix
/// query with an ordinary F128 MLE. Keep the block-prefix scale even if zero;
/// no normalization or division is performed. Equality weights remain online.
pub fn evaluate_ordinary(
    words: &[F128],
    point: &[F128],
    scale_value: F128,
    reference: bool,
) -> F128 {
    assert_eq!(words.len(), 1usize << point.len());
    if reference {
        return evaluate(words, &scale(ordinary(point, F128::ZERO), scale_value));
    }
    let row_pairs = pairs(point);
    let split = point.len().div_ceil(2);
    let lo = row_weights(&row_pairs[..split]);
    let hi = row_weights(&row_pairs[split..]);
    let block = |(i, values): (usize, &[F128])| dot_block(values, &lo) * hi[i];
    let value = if words.len() >= 1 << 15 {
        words
            .par_chunks(lo.len())
            .enumerate()
            .map(block)
            .reduce(|| F128::ZERO, |a, b| a + b)
    } else {
        words
            .chunks(lo.len())
            .enumerate()
            .map(block)
            .fold(F128::ZERO, |a, b| a + b)
    };
    scale_value * value
}
pub fn eval_with_value(words: &[F128], mut q: TensorClaim) -> TensorClaim {
    q.value = evaluate(words, &q);
    q
}
pub fn selector_column(which: usize) -> Vec<F128> {
    (0..128)
        .map(|j| if j == which { F128::ONE } else { F128::ZERO })
        .collect()
}
pub fn label_columns(start: usize, h: usize) -> Vec<F128> {
    (0..128)
        .map(|j| {
            if (start..start + h).contains(&j) {
                from_raw(1u128 << (j - start))
            } else {
                F128::ZERO
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_field::test_rng::Rng;

    #[test]
    fn ordinary_dot_matches_original_bits_and_independent_mle() {
        let mut rng = Rng::new(0x0AD1_AA26);
        for log in [0usize, 1, 3, 4, 7, 15] {
            let words = rng.f128_vec(1usize << log);
            for point in [
                vec![F128::ZERO; log],
                vec![F128::ONE; log],
                rng.f128_vec(log),
            ] {
                let expected = words
                    .iter()
                    .enumerate()
                    .fold(F128::ZERO, |sum, (i, &word)| {
                        let weight = point.iter().enumerate().fold(F128::ONE, |value, (j, &r)| {
                            value * if (i >> j) & 1 == 0 { F128::ONE + r } else { r }
                        });
                        sum + weight * word
                    });
                for scale in [F128::ZERO, F128::ONE, F128::new(97, 23)] {
                    assert_eq!(
                        evaluate_ordinary(&words, &point, scale, false),
                        scale * expected
                    );
                    assert_eq!(
                        evaluate_ordinary(&words, &point, scale, true),
                        scale * expected
                    );
                }
            }
        }
    }

    #[test]
    fn native_dot_matches_scalar_with_offsets_tails_and_each_source_bit() {
        let mut rng = Rng::new(0xD074_128);
        let words = rng.f128_vec(80);
        let weights = rng.f128_vec(80);
        for start in [0usize, 1, 3] {
            for len in [0usize, 1, 3, 4, 5, 15, 16, 17, 63, 64, 65] {
                let (a, b) = (&words[start..start + len], &weights[start..start + len]);
                let expected = a.iter().zip(b).fold(F128::ZERO, |v, (&a, &b)| v + a * b);
                assert_eq!(dot_block(a, b), expected);
            }
        }
        for bit in 0..128 {
            let mut a = vec![F128::ZERO; 65];
            a[bit % 65] = from_raw(1u128 << bit);
            assert_eq!(
                dot_block(&a, &weights[..65]),
                a[bit % 65] * weights[bit % 65]
            );
        }
    }

    #[test]
    fn paired_dot_matches_scalar_with_offsets_tails_and_each_source_bit() {
        let mut rng = Rng::new(0xD074_2A18);
        let words = rng.f128_vec(80);
        let weights = [rng.f128_vec(80), rng.f128_vec(80)];
        for start in [0usize, 1, 3] {
            for len in [0usize, 1, 3, 4, 5, 15, 16, 17, 63, 64, 65] {
                let words = &words[start..start + len];
                for same in [false, true] {
                    let pair = [
                        &weights[0][start..start + len],
                        &weights[usize::from(!same)][start..start + len],
                    ];
                    let expected = pair.map(|weights| {
                        words
                            .iter()
                            .zip(weights)
                            .fold(F128::ZERO, |v, (&a, &b)| v + a * b)
                    });
                    assert_eq!(paired_dot_block(words, pair), expected);
                }
            }
        }
        for bit in 0..128 {
            let mut words = vec![F128::ZERO; 65];
            words[bit % 65] = from_raw(1u128 << bit);
            assert_eq!(
                paired_dot_block(&words, [&weights[0][..65], &weights[1][..65]]),
                weights
                    .each_ref()
                    .map(|weights| words[bit % 65] * weights[bit % 65])
            );
        }
    }

    #[test]
    fn paired_ordinary_matches_independent_points_and_arbitrary_scales() {
        let mut rng = Rng::new(0x0AD2_26AA);
        for log in [0usize, 1, 3, 4, 7, 15] {
            let words = rng.f128_vec(1usize << log);
            for points in [
                [vec![F128::ZERO; log], vec![F128::ONE; log]],
                [vec![F128::ONE; log], vec![F128::ONE; log]],
                [rng.f128_vec(log), rng.f128_vec(log)],
            ] {
                let evaluation =
                    PairedOrdinaryEvaluation::new(points.each_ref().map(Vec::as_slice));
                let expected = points.each_ref().map(|point| {
                    words
                        .iter()
                        .enumerate()
                        .fold(F128::ZERO, |sum, (i, &word)| {
                            let weight = point.iter().enumerate().fold(F128::ONE, |v, (j, &r)| {
                                v * if i >> j & 1 == 0 { F128::ONE + r } else { r }
                            });
                            sum + word * weight
                        })
                });
                for scales in [
                    [F128::ZERO, F128::ONE],
                    [F128::ONE, F128::ZERO],
                    [F128::ONE; 2],
                    [rng.f128(), rng.f128()],
                ] {
                    let actual = evaluation.evaluate(&words, scales);
                    for side in 0..2 {
                        assert_eq!(actual[side], expected[side] * scales[side]);
                        assert_eq!(
                            actual[side],
                            evaluate_ordinary(&words, &points[side], scales[side], true)
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn lifting_keeps_block_and_scale() {
        let words = (0..1024)
            .map(|i| F128::new(i as u64, 0))
            .collect::<Vec<_>>();
        let q = ordinary(&[F128::new(3, 0); 7], F128::ZERO);
        let q = eval_with_value(&words[384..512], q);
        let scaled = scale(q, F128::new(17, 0));
        let lifted = lift(scaled, 3, 10);
        assert_eq!(evaluate(&words, &lifted), lifted.value);
    }
}
