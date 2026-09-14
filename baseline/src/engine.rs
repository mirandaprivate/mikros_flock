//! Shared weighted-Hadamard engine, so equality construction is the only
//! prover difference between DaoThaler and DaoThalerGeneric.

use flock_field::{F128, F256Unreduced};
use flock_transcript::challenger::Challenger;
use rayon::prelude::*;

use crate::{Claims, Method, Proof, Result, Statement, bit, dao_thaler, evaluate_round};

const CHUNK: usize = 4096;
const PARALLEL_THRESHOLD: usize = 32768;

fn sum_chunks<const K: usize>(
    len: usize,
    scan: impl Fn(usize, usize) -> [F128; K] + Sync,
) -> [F128; K] {
    let add = |a: [F128; K], b: [F128; K]| std::array::from_fn(|i| a[i] + b[i]);
    let scan_chunk = |chunk| scan(chunk * CHUNK, len.min((chunk + 1) * CHUNK));
    if len >= PARALLEL_THRESHOLD && rayon::current_num_threads() > 1 {
        (0..len.div_ceil(CHUNK))
            .into_par_iter()
            .map(scan_chunk)
            .reduce(|| [F128::ZERO; K], add)
    } else {
        (0..len.div_ceil(CHUNK))
            .map(scan_chunk)
            .fold([F128::ZERO; K], add)
    }
}

fn project_c(c: &[u8], weights: &[F128], z: F128) -> F128 {
    let half = weights.len();
    let sums = sum_chunks(half, |start, end| {
        let mut sums = [F128::ZERO; 2];
        for i in start..end {
            if bit(c, i) {
                sums[0] += weights[i];
            }
            if bit(c, i + half) {
                sums[1] += weights[i];
            }
        }
        sums
    });
    sums[0] * (F128::ONE + z) + sums[1] * z
}

fn binary_coefficients(a: &[u8], b: &[u8], weights: &[F128]) -> [F128; 3] {
    let half = weights.len();
    sum_chunks(half, |start, end| {
        let mut sums = [F128::ZERO; 3];
        for i in start..end {
            let (a0, b0) = (bit(a, i), bit(b, i));
            let (da, db) = (a0 ^ bit(a, i + half), b0 ^ bit(b, i + half));
            // Products and differences stay in F2. No extension multiplication
            // occurs per pair until the first sumcheck challenge is sampled.
            for (k, nonzero) in [a0 & b0, (a0 & db) ^ (da & b0), da & db]
                .into_iter()
                .enumerate()
            {
                if nonzero {
                    sums[k] += weights[i];
                }
            }
        }
        sums
    })
}

fn extension_coefficients(values: &[[F128; 2]], weights: &[F128]) -> [F128; 3] {
    let half = weights.len();
    sum_chunks(half, |start, end| {
        let mut sums = [F256Unreduced::default(); 3];
        for i in start..end {
            let [a0, b0] = values[i];
            let [a1, b1] = values[i + half];
            let q0 = a0 * b0;
            let q2 = (a0 + a1) * (b0 + b1);
            let q1 = a1 * b1 + q0 + q2;
            for (k, value) in [q0, q1, q2].into_iter().enumerate() {
                sums[k] ^= value.mul_unreduced(weights[i]);
            }
        }
        sums.map(F256Unreduced::reduce)
    })
}

fn lift_and_fold(a: &[u8], b: &[u8], half: usize, r: F128) -> Vec<[F128; 2]> {
    // Each source has only N/2 extension elements after its first fold.
    let row = |i| {
        [a, b].map(|input| {
            let low = bit(input, i);
            let delta = low ^ bit(input, i + half);
            (if low { F128::ONE } else { F128::ZERO }) + if delta { r } else { F128::ZERO }
        })
    };
    if half >= PARALLEL_THRESHOLD && rayon::current_num_threads() > 1 {
        (0..half).into_par_iter().map(row).collect()
    } else {
        (0..half).map(row).collect()
    }
}

fn fold(values: &mut Vec<[F128; 2]>, r: F128) {
    let half = values.len() / 2;
    let (low, high) = values.split_at_mut(half);
    let pair = |(a, b): (&mut [F128; 2], &[F128; 2])| {
        for k in 0..2 {
            a[k] += (a[k] + b[k]) * r;
        }
    };
    if half >= PARALLEL_THRESHOLD && rayon::current_num_threads() > 1 {
        low.par_iter_mut().zip(high.par_iter()).for_each(pair);
    } else {
        low.iter_mut().zip(high.iter()).for_each(pair);
    }
    values.truncate(half);
}

fn remove_high_weight(weights: &mut Vec<F128>) {
    let half = weights.len() / 2;
    let (low, high) = weights.split_at_mut(half);
    // Summing both slices removes the variable even at z=0 or z=1. No
    // division, rejection sampling, or invert_or_zero convention is required.
    if half >= PARALLEL_THRESHOLD && rayon::current_num_threads() > 1 {
        low.par_iter_mut()
            .zip(high.par_iter())
            .for_each(|(a, b)| *a += *b);
    } else {
        low.iter_mut().zip(high.iter()).for_each(|(a, b)| *a += *b);
    }
    weights.truncate(half);
}

pub(crate) fn prove(
    statement: &Statement,
    a: &[u8],
    b: &[u8],
    c: &[u8],
    method: Method,
    z: Vec<F128>,
    ch: &mut impl Challenger,
) -> Result<(Proof, Claims)> {
    let n = statement.log_n;
    let mut weights = if n == 0 {
        Vec::new()
    } else if method == Method::DaoThaler {
        dao_thaler::equality_table(&z[..n - 1], (n - 1).min(7))?
    } else {
        dao_thaler::generic_eq_table(&z[..n - 1])?
    };
    let c_eval = if n == 0 {
        if bit(c, 0) { F128::ONE } else { F128::ZERO }
    } else {
        project_c(c, &weights, z[n - 1])
    };
    ch.observe_f128(c_eval);
    let mut values = if method == Method::GhashFullExtension {
        Some(
            (0..1usize << n)
                .map(|i| [a, b].map(|input| if bit(input, i) { F128::ONE } else { F128::ZERO }))
                .collect::<Vec<_>>(),
        )
    } else {
        None
    };
    let mut rounds = Vec::with_capacity(n);
    let mut point = vec![F128::ZERO; n];
    let mut claim = c_eval;
    let mut bound_eq = F128::ONE;
    for i in (0..n).rev() {
        let [q0, q1, q2] = match &values {
            Some(values) => extension_coefficients(values, &weights),
            None => binary_coefficients(a, b, &weights),
        };
        let e0 = F128::ONE + z[i];
        let message = [bound_eq * e0 * q0, bound_eq * (q1 + e0 * q2), bound_eq * q2];
        ch.observe_f128_slice(&message);
        let r = ch.sample_f128();
        claim = evaluate_round(message, claim, r);
        point[i] = r;
        rounds.push(message);
        match &mut values {
            Some(values) => fold(values, r),
            None => values = Some(lift_and_fold(a, b, 1usize << i, r)),
        }
        bound_eq *= e0 + r;
        if i > 0 {
            remove_high_weight(&mut weights);
        }
    }
    let [final_a, final_b] = values.map_or_else(
        || [a, b].map(|input| if bit(input, 0) { F128::ONE } else { F128::ZERO }),
        |values| values[0],
    );
    ch.observe_f128_slice(&[final_a, final_b]);
    let proof = Proof {
        c_eval,
        rounds,
        final_a,
        final_b,
    };
    let claims = Claims {
        ab_point: point,
        c_point: z,
        values: [final_a, final_b, c_eval],
    };
    Ok((proof, claims))
}
