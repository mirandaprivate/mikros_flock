//! An opt-in measurement, separate from correctness tests and full PIOP timings.
use super::{F128, FastField, Kernel, Plan, Polys};
use flock_core::zerocheck::univariate_skip::SplitEqGhash;
use rayon::prelude::*;
use std::{hint::black_box, time::Instant};

fn milliseconds(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

fn aggregate(
    polys: &Polys,
    columns: usize,
    field: &FastField,
    eq: &SplitEqGhash,
    kernel: Kernel,
) -> Vec<F128> {
    // This includes all challenge-dependent tower conversion in each measured
    // call. Only the public, fixed field-coordinate matrices can be cached.
    let plan = Plan::with_kernel(polys, columns, field, &eq.lo, kernel);
    (0..eq.hi.len())
        .into_par_iter()
        .map(|hi| {
            plan.block(hi * eq.lo.len())
                .into_iter()
                .map(|x| x * eq.hi[hi])
                .collect::<Vec<_>>()
        })
        .reduce(
            || vec![F128::ZERO; columns * polys.degree],
            |mut a, b| {
                for (x, y) in a.iter_mut().zip(b) {
                    *x += y;
                }
                a
            },
        )
}

#[test]
#[ignore = "explicit native 64-thread coefficient aggregation benchmark"]
fn packed_aggregate_kernel_benchmark() {
    let option = |key: &str, default| {
        std::env::var(key)
            .ok()
            .map(|s| s.parse::<usize>().unwrap())
            .unwrap_or(default)
    };
    let log_rows = option("MIKROS_AGG_BENCH_LOG_ROWS", 21);
    let threads = option("MIKROS_AGG_BENCH_THREADS", 64);
    let reps = option("MIKROS_AGG_BENCH_REPS", 6);
    assert!((13..=25).contains(&log_rows) && threads > 0 && reps > 0);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap();
    let native = cfg!(all(
        target_arch = "x86_64",
        target_feature = "gfni",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "sse4.1"
    ));
    pool.install(|| {
        println!("\n{{\"record\":\"aggregate_metadata\",\"log_rows\":{log_rows},\"threads\":{threads},\"reps\":{reps},\"low_weight_log\":13,\"online_weight_conversion_included\":true,\"native_kernels_compiled\":{native}}}");
        let field = FastField::for_width(32).unwrap();
        let point = (0..log_rows).map(|i| F128::new(0x731_4567 * (i as u64 + 5), 0x879a_ab31 * (i as u64 + 19))).collect::<Vec<_>>();
        let eq = SplitEqGhash::with_n_hi(&point, log_rows - 13);
        let variants = [Kernel::Baseline, Kernel::Gfni, Kernel::Tower];
        for (columns, degree) in [(4,4), (3,4), (4,2), (3,2)] {
            let polys = Polys::generate((1usize << log_rows) * columns, degree, 32, |node, dst| {
                for (coefficient, out) in dst.iter_mut().enumerate() {
                    let mut x = (node as u64 + 17).wrapping_mul(0x9e3779b97f4a7c15) ^ coefficient as u64;
                    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
                    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
                    *out = ((x ^ (x >> 31)) & 0xffff_ffff) as u128;
                }
            });
            let expected = aggregate(&polys, columns, field, &eq, Kernel::Baseline);
            for iteration in 0..=reps {
                for slot in 0..3 {
                    let kernel = variants[(iteration + slot) % 3];
                    let start = Instant::now();
                    let result = aggregate(&polys, columns, field, &eq, kernel);
                    let elapsed = milliseconds(start);
                    assert_eq!(result, expected, "coefficient sums changed under {}", kernel.name());
                    black_box(result);
                    println!("{{\"record\":\"aggregate_sample\",\"kernel\":\"{}\",\"columns\":{columns},\"degree\":{degree},\"iteration\":{iteration},\"slot\":{slot},\"warmup\":{},\"milliseconds\":{elapsed},\"verified\":true}}",kernel.name(),iteration==0);
                }
            }
        }
    });
}
