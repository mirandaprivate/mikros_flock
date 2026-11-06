//! Arithmetic-only ablation on identical dense POLYVAL-coordinate inputs.
//! No PCS, hashing, allocation or fixture generation is timed. Materialized
//! multiplication and dot products have different output traffic and should
//! be compared within their respective groups.

use std::{env, fmt::Write, hint::black_box, io::Write as _, time::Instant};

use ark_std::rand::{RngCore, SeedableRng, rngs::StdRng};
use binius_field::{Field, WideMul};
use blake2::Blake2s256;
use digest::Digest;
use mikros_raafold::raafold_field::{PolyvalWideProduct, RustCryptoPolyvalField as F};
use polyval::{
    Block, Key, Polyval,
    universal_hash::{KeyInit, UniversalHash},
};
#[cfg(feature = "parallel")]
use rayon::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Options {
    log_elements: usize,
    reps: usize,
    threads: Vec<usize>,
    chunk: usize,
    seed: u64,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            log_elements: 22,
            reps: 5,
            threads: vec![1, 64],
            chunk: 4096,
            seed: 97,
        }
    }
}

fn parse(args: impl IntoIterator<Item = String>) -> Result<Option<Options>, String> {
    let mut options = Options::default();
    let mut args = args.into_iter();
    while let Some(flag) = args.next() {
        if flag == "--help" || flag == "-h" {
            println!(
                "field_compare [--log-elements N] [--reps R] [--threads 1,64] [--chunk C] [--seed S]\nDefaults: N=22, R=5, threads=1,64, chunk=4096, seed=97.\nOne excluded warmup per kernel; measured kernel order rotates per repetition.\nSame dense F128 inputs; every materialized output is checked element by element."
            );
            return Ok(None);
        }
        let value = args
            .next()
            .ok_or_else(|| format!("missing value for {flag}"))?;
        match flag.as_str() {
            "--log-elements" => {
                options.log_elements = value.parse().map_err(|_| "invalid log-elements")?
            }
            "--reps" => options.reps = value.parse().map_err(|_| "invalid reps")?,
            "--threads" => {
                options.threads = value
                    .split(',')
                    .map(|x| x.parse().map_err(|_| "invalid threads"))
                    .collect::<Result<_, _>>()?
            }
            "--chunk" => options.chunk = value.parse().map_err(|_| "invalid chunk")?,
            "--seed" => options.seed = value.parse().map_err(|_| "invalid seed")?,
            _ => return Err(format!("unknown option {flag}")),
        }
    }
    if options.log_elements >= usize::BITS as usize
        || (1usize << options.log_elements)
            .checked_mul(core::mem::size_of::<F>())
            .is_none()
    {
        return Err("element vector size overflows usize".into());
    }
    if options.reps == 0
        || options.chunk == 0
        || options.threads.is_empty()
        || options.threads.contains(&0)
    {
        return Err("reps, chunk and thread counts must be positive".into());
    }
    #[cfg(not(feature = "parallel"))]
    if options.threads.iter().any(|&x| x != 1) {
        return Err("this build requires --threads 1 (parallel feature disabled)".into());
    }
    Ok(Some(options))
}

#[derive(Clone, Copy, Debug)]
enum Kernel {
    OldMul,
    NewMul,
    OldScalarDot,
    NewScalarDot,
    WideDot,
    SimdDot,
}

impl Kernel {
    const ALL: [Self; 6] = [
        Self::OldMul,
        Self::NewMul,
        Self::OldScalarDot,
        Self::NewScalarDot,
        Self::WideDot,
        Self::SimdDot,
    ];
    fn name(self) -> &'static str {
        match self {
            Self::OldMul => "old_polyval_mul",
            Self::NewMul => "new_direct_mul",
            Self::OldScalarDot => "old_scalar_dot",
            Self::NewScalarDot => "new_scalar_dot",
            Self::WideDot => "new_wide_dot",
            Self::SimdDot => "new_simd_dot",
        }
    }
    fn materializes(self) -> bool {
        matches!(self, Self::OldMul | Self::NewMul)
    }
}

/// Exactly the former field multiplication implementation, with a fresh
/// RustCrypto context per independent field product.
#[inline]
fn old_multiply(a: F, b: F) -> F {
    let key = Key::from(a.val().to_le_bytes());
    let block = Block::from(b.val().to_le_bytes());
    let mut hash = Polyval::new(&key);
    hash.update(&[block]);
    F::new(u128::from_le_bytes(hash.finalize().into()))
}

#[inline]
fn scalar_dot(left: &[F], right: &[F], old: bool) -> F {
    if old {
        left.iter()
            .zip(right)
            .fold(F::ZERO, |sum, (&a, &b)| sum + old_multiply(a, b))
    } else {
        left.iter()
            .zip(right)
            .fold(F::ZERO, |sum, (&a, &b)| sum + a * b)
    }
}

#[inline]
fn wide_dot(left: &[F], right: &[F]) -> F {
    F::reduce(
        left.iter()
            .zip(right)
            .fold(PolyvalWideProduct::default(), |sum, (&a, &b)| {
                sum + F::wide_mul(a, b)
            }),
    )
}

fn chunk_dot(kernel: Kernel, left: &[F], right: &[F]) -> F {
    match kernel {
        Kernel::OldScalarDot => scalar_dot(left, right, true),
        Kernel::NewScalarDot => scalar_dot(left, right, false),
        Kernel::WideDot => wide_dot(left, right),
        Kernel::SimdDot => F::dot_product(left, right),
        _ => unreachable!(),
    }
}

fn materialize(
    kernel: Kernel,
    left: &[F],
    right: &[F],
    out: &mut [F],
    threads: usize,
    chunk: usize,
) {
    let fill = |out: &mut [F], left: &[F], right: &[F]| {
        if matches!(kernel, Kernel::OldMul) {
            for ((dst, &a), &b) in out.iter_mut().zip(left).zip(right) {
                *dst = old_multiply(a, b);
            }
        } else {
            for ((dst, &a), &b) in out.iter_mut().zip(left).zip(right) {
                *dst = a * b;
            }
        }
    };
    #[cfg(feature = "parallel")]
    if threads > 1 {
        out.par_chunks_mut(chunk)
            .zip(left.par_chunks(chunk))
            .zip(right.par_chunks(chunk))
            .for_each(|((out, left), right)| fill(out, left, right));
        return;
    }
    let _ = (threads, chunk);
    fill(out, left, right);
}

fn evaluate_dot(kernel: Kernel, left: &[F], right: &[F], threads: usize, chunk: usize) -> F {
    #[cfg(feature = "parallel")]
    if threads > 1 {
        return left
            .par_chunks(chunk)
            .zip(right.par_chunks(chunk))
            .map(|(a, b)| chunk_dot(kernel, a, b))
            .reduce(|| F::ZERO, |a, b| a + b);
    }
    let _ = (threads, chunk);
    chunk_dot(kernel, left, right)
}

fn measure(
    kernel: Kernel,
    left: &[F],
    right: &[F],
    out: &mut [F],
    reference: &[F],
    threads: usize,
    chunk: usize,
) -> Result<(f64, F), String> {
    let left = black_box(left);
    let right = black_box(right);
    let start = Instant::now();
    let dot = if kernel.materializes() {
        materialize(kernel, left, right, out, threads, chunk);
        // Observed before ending the timed region, not just during the later
        // correctness pass. This prevents dead-store elimination.
        black_box(&*out);
        F::ZERO
    } else {
        black_box(evaluate_dot(kernel, left, right, threads, chunk))
    };
    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
    let checksum = if kernel.materializes() {
        if &*out != reference {
            return Err(format!("{} materialized output mismatch", kernel.name()));
        }
        out.iter().copied().sum()
    } else {
        dot
    };
    Ok((elapsed_ms, checksum))
}

fn median(values: &[f64]) -> f64 {
    let mut values = values.to_vec();
    values.sort_by(f64::total_cmp);
    let n = values.len();
    if n % 2 == 1 {
        values[n / 2]
    } else {
        (values[n / 2 - 1] + values[n / 2]) * 0.5
    }
}

fn benchmark(
    options: &Options,
    threads: usize,
    left: &[F],
    right: &[F],
    reference: &[F],
) -> Result<(), String> {
    let expected: F = reference.iter().copied().sum();
    let mut out = vec![F::ZERO; left.len()];
    let mut measurements: [Vec<f64>; 6] =
        core::array::from_fn(|_| Vec::with_capacity(options.reps));
    eprintln!("[field_compare] threads={threads}: one excluded warmup for each kernel");
    for kernel in Kernel::ALL {
        let (_, value) = measure(
            kernel,
            left,
            right,
            &mut out,
            reference,
            threads,
            options.chunk,
        )?;
        if value != expected {
            return Err(format!("{} warmup checksum mismatch", kernel.name()));
        }
    }
    for sample in 0..options.reps {
        for offset in 0..Kernel::ALL.len() {
            let index = (sample + offset) % Kernel::ALL.len();
            let kernel = Kernel::ALL[index];
            let (ms, value) = measure(
                kernel,
                left,
                right,
                &mut out,
                reference,
                threads,
                options.chunk,
            )?;
            if value != expected {
                return Err(format!("{} sample checksum mismatch", kernel.name()));
            }
            measurements[index].push(ms);
            println!(
                "sample,{},{},{},{},{},{:.6},{:.6},{:032x},true",
                kernel.name(),
                threads,
                sample + 1,
                left.len(),
                options.chunk,
                ms,
                left.len() as f64 / (ms * 1000.0),
                value.val()
            );
            std::io::stdout().flush().map_err(|e| e.to_string())?;
        }
    }
    for (kernel, times) in Kernel::ALL.into_iter().zip(&measurements) {
        let ms = median(times);
        println!(
            "median,{},{},0,{},{},{:.6},{:.6},{:032x},true",
            kernel.name(),
            threads,
            left.len(),
            options.chunk,
            ms,
            left.len() as f64 / (ms * 1000.0),
            expected.val()
        );
    }
    Ok(())
}

fn run(options: Options) -> Result<(), String> {
    let n = 1usize << options.log_elements;
    let mut rng = StdRng::seed_from_u64(options.seed);
    let mut hash = Blake2s256::new();
    let mut make_input = || {
        (0..n)
            .map(|_| {
                let raw = u128::from(rng.next_u64()) | (u128::from(rng.next_u64()) << 64);
                hash.update(raw.to_le_bytes());
                F::new(raw)
            })
            .collect::<Vec<_>>()
    };
    let left = make_input();
    let right = make_input();
    let mut input_digest = String::new();
    for byte in hash.finalize() {
        write!(&mut input_digest, "{byte:02x}").unwrap();
    }
    eprintln!(
        "[field_compare] preparing untimed full-output oracle via former RustCrypto implementation"
    );
    let reference: Vec<_> = left
        .iter()
        .zip(&right)
        .map(|(&a, &b)| old_multiply(a, b))
        .collect();
    #[cfg(target_arch = "x86_64")]
    let cpu_features = format!(
        "pclmulqdq:{};avx512f:{};avx512bw:{};vpclmulqdq:{}",
        std::is_x86_feature_detected!("pclmulqdq"),
        std::is_x86_feature_detected!("avx512f"),
        std::is_x86_feature_detected!("avx512bw"),
        std::is_x86_feature_detected!("vpclmulqdq")
    );
    #[cfg(not(target_arch = "x86_64"))]
    let cpu_features = String::from("non-x86_64:portable-dispatch");
    println!(
        "# field_compare_v1 elements=2^{} reps={} warmups=1 seed={} chunk={} field_bytes={} wide_bytes={} cpu={} input_blake2s256={}",
        options.log_elements,
        options.reps,
        options.seed,
        options.chunk,
        core::mem::size_of::<F>(),
        core::mem::size_of::<PolyvalWideProduct>(),
        cpu_features,
        input_digest
    );
    println!(
        "# mul: writes preallocated output; dot: reduced scalar result. Allocation, RNG, oracle and checks excluded. Warmups excluded; order rotates per sample. No CPU affinity is imposed by this example."
    );
    println!(
        "# scalar labels describe the source loop; normal compiler auto-vectorization remains enabled. simd_dot uses the explicit runtime-dispatched field kernel."
    );
    println!(
        "record,kernel,threads,sample,elements,chunk,time_ms,million_products_per_second,checksum,verified"
    );
    for &threads in &options.threads {
        #[cfg(feature = "parallel")]
        {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .map_err(|e| e.to_string())?;
            pool.install(|| benchmark(&options, threads, &left, &right, &reference))?;
        }
        #[cfg(not(feature = "parallel"))]
        benchmark(&options, threads, &left, &right, &reference)?;
    }
    Ok(())
}

fn main() {
    let result = parse(env::args().skip(1)).and_then(|options| match options {
        Some(options) => run(options),
        None => Ok(()),
    });
    if let Err(error) = result {
        eprintln!("field_compare failed: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernels_agree_with_materialized_original_products_including_tails() {
        let mut rng = StdRng::seed_from_u64(97);
        for n in [0, 1, 3, 4, 5, 31, 32, 33, 4097] {
            let left: Vec<_> = (0..n)
                .map(|_| F::new(u128::from(rng.next_u64()) | (u128::from(rng.next_u64()) << 64)))
                .collect();
            let right: Vec<_> = (0..n)
                .map(|_| F::new(u128::from(rng.next_u64()) | (u128::from(rng.next_u64()) << 64)))
                .collect();
            let reference: Vec<_> = left
                .iter()
                .zip(&right)
                .map(|(&a, &b)| old_multiply(a, b))
                .collect();
            let expected: F = reference.iter().copied().sum();
            for kernel in Kernel::ALL {
                let mut out = vec![F::ZERO; n];
                assert_eq!(
                    measure(kernel, &left, &right, &mut out, &reference, 1, 13)
                        .unwrap()
                        .1,
                    expected
                );
                #[cfg(feature = "parallel")]
                rayon::ThreadPoolBuilder::new()
                    .num_threads(4)
                    .build()
                    .unwrap()
                    .install(|| {
                        assert_eq!(
                            measure(kernel, &left, &right, &mut out, &reference, 4, 13)
                                .unwrap()
                                .1,
                            expected
                        );
                    });
            }
        }
    }

    #[test]
    fn cli_rejects_zero_counts_overflow_and_unknown_options() {
        for args in [
            vec!["--reps", "0"],
            vec!["--threads", "0"],
            vec!["--chunk", "0"],
            vec!["--bogus", "2"],
            vec!["--threads"],
        ] {
            assert!(parse(args.into_iter().map(str::to_owned)).is_err());
        }
        assert!(parse(["--log-elements".to_owned(), usize::BITS.to_string()]).is_err());
        assert_eq!(
            parse(["--threads".to_owned(), "1".to_owned()])
                .unwrap()
                .unwrap()
                .threads,
            [1]
        );
        assert_eq!(median(&[3.0, 1.0, 2.0]), 2.0);
        assert_eq!(median(&[1.0, 3.0]), 2.0);
    }
}
