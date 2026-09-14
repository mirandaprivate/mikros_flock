//! Paired PIOP-reduction benchmark. Run one instance at a time.
//! Input generation/digests and independent opening checks are untimed.

use std::{
    env, fs,
    hint::black_box,
    path::{Path, PathBuf},
    process::Command,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

use flock_core::zerocheck;
use flock_field::test_rng::Rng;
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros::{Kernel, flock_baseline};
use serde::Serialize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
enum System {
    Reference,
    Lookup,
    Packed,
    Packed8,
    Legacy,
    Stream,
    Simd,
    SimdV1,
    Wide,
    Fused,
    Native,
    Reuse,
    Factored,
    FactoredReuse,
    Flock,
}

impl System {
    fn name(self) -> &'static str {
        match self {
            Self::Reference => "reference",
            Self::Lookup => "lookup",
            Self::Packed => "packed",
            Self::Packed8 => "packed8",
            Self::Legacy => "legacy",
            Self::Stream => "stream",
            Self::Simd => "simd",
            Self::SimdV1 => "simdv1",
            Self::Wide => "wide",
            Self::Fused => "fused",
            Self::Native => "native",
            Self::Reuse => "reuse",
            Self::Factored => "factored",
            Self::FactoredReuse => "factoredreuse",
            Self::Flock => "flock",
        }
    }

    fn challenger(self, digest: &[u8; 32]) -> FsChallenger {
        let domain: &[u8] = match self {
            Self::Flock => b"flock-zerocheck-bench-v1",
            _ => b"mikros-hadamard-bench-v1",
        };
        let mut challenger = FsChallenger::with_hash(domain, HashKind::Blake3);
        challenger.observe_bytes(digest);
        challenger
    }

    fn kernel(self) -> Kernel {
        match self {
            Self::Reference => Kernel::Reference,
            Self::Lookup => Kernel::Lookup,
            Self::Packed => Kernel::Packed,
            Self::Packed8 => Kernel::Packed8,
            Self::Legacy => Kernel::PackedLegacy,
            Self::Stream => Kernel::PackedStream,
            Self::Simd => Kernel::PackedSimd,
            Self::SimdV1 => Kernel::PackedSimdV1,
            Self::Wide => Kernel::PackedWide,
            Self::Fused => Kernel::PackedFused,
            Self::Native | Self::Reuse => Kernel::PackedNative,
            Self::Factored | Self::FactoredReuse => Kernel::PackedFactored,
            Self::Flock => unreachable!(),
        }
    }
}

struct Options {
    logs: Vec<usize>,
    runs: usize,
    threads: usize,
    systems: Vec<System>,
    output: Option<PathBuf>,
    rss: bool,
    profile: bool,
}

#[derive(Serialize)]
struct Sample {
    log_n: usize,
    system: System,
    run: usize,
    order: usize,
    input_digest: String,
    prover_ms: f64,
    verifier_ms: f64,
    input_claim_check_ms: f64,
    payload_field_elements: usize,
    payload_field_bytes: usize,
    bincode_bytes: u64,
    proof_digest: String,
    vm_rss_before_prove_kib: Option<u64>,
    vm_hwm_after_prove_kib: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mikros_phases: Option<mikros::hadamard::ProverProfile>,
}

#[derive(Serialize)]
struct Timing {
    median: f64,
    min: f64,
    max: f64,
}

#[derive(Serialize)]
struct Summary {
    log_n: usize,
    constraints: usize,
    system: System,
    runs: usize,
    prover_ms: Timing,
    verifier_ms: Timing,
    input_claim_check_ms: Timing,
    payload_field_elements: usize,
    payload_field_bytes: usize,
    bincode_bytes: u64,
}

#[derive(Serialize)]
struct Report {
    schema_version: u32,
    metadata: serde_json::Value,
    summaries: Vec<Summary>,
    warmups: Vec<Sample>,
    samples: Vec<Sample>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("benchmark failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let Some(options) = parse_options()? else {
        return Ok(());
    };
    rayon::ThreadPoolBuilder::new()
        .num_threads(options.threads)
        .stack_size(8 << 20)
        .build_global()
        .map_err(|e| format!("cannot initialize global thread pool: {e}"))?;
    if !flock_parallel::init_all_core_pool(options.threads) {
        return Err("all-core thread pool was initialized before benchmark configuration".into());
    }

    let mut report = Report {
        schema_version: 1,
        metadata: metadata(&options),
        summaries: Vec::new(),
        warmups: Vec::new(),
        samples: Vec::new(),
    };
    println!(
        "PIOP reductions, BLAKE3 transcript, no grinding, {} thread(s)",
        options.threads
    );
    println!("Generation/digest and original-input claim checks excluded from prove/verify time.");
    if options.rss {
        println!(
            "RSS mode: one cold proof in this process, no warmup; VmHWM read before verification/input checks."
        );
    }

    for &log_n in &options.logs {
        // Only the explicitly named reuse variant persists capacity between
        // different proofs. The warmup is the first allocation, retained in JSON.
        let mut scratch = mikros::ProverScratch::default();
        let mut factored_scratch = mikros::ProverScratch::default();
        if !options.rss {
            let (a, b, c, digest) = witness(log_n, 0);
            for (order, &system) in options.systems.iter().enumerate() {
                let sample = run_one(
                    system,
                    log_n,
                    0,
                    order,
                    &a,
                    &b,
                    &c,
                    &digest,
                    false,
                    options.profile,
                    if system == System::FactoredReuse {
                        &mut factored_scratch
                    } else {
                        &mut scratch
                    },
                )?;
                println!(
                    "  warmup n=2^{log_n} {:9}: {:.3} ms",
                    system.name(),
                    sample.prover_ms
                );
                report.warmups.push(sample);
            }
        }
        for run in 1..=options.runs {
            let (a, b, c, digest) = witness(log_n, run);
            // Alternate and rotate systems to reduce consistent cache/thermal
            // position bias, preserving identical inputs within every pair.
            let mut order = options.systems.clone();
            let count = order.len();
            order.rotate_left(((run - 1) / 2) % count);
            if run % 2 == 0 {
                order.reverse();
            }
            for (position, system) in order.into_iter().enumerate() {
                let sample = run_one(
                    system,
                    log_n,
                    run,
                    position,
                    &a,
                    &b,
                    &c,
                    &digest,
                    options.rss,
                    options.profile,
                    if system == System::FactoredReuse {
                        &mut factored_scratch
                    } else {
                        &mut scratch
                    },
                )?;
                println!(
                    "  n=2^{log_n} run={run} {:9}: prove {:.3} ms, verify {:.3} ms, {} bytes",
                    system.name(),
                    sample.prover_ms,
                    sample.verifier_ms,
                    sample.bincode_bytes
                );
                report.samples.push(sample);
            }
        }
        for &system in &options.systems {
            let selected: Vec<&Sample> = report
                .samples
                .iter()
                .filter(|s| s.log_n == log_n && s.system == system)
                .collect();
            let first = selected[0];
            let summary = Summary {
                log_n,
                constraints: 1usize << log_n,
                system,
                runs: selected.len(),
                prover_ms: timing(selected.iter().map(|s| s.prover_ms).collect()),
                verifier_ms: timing(selected.iter().map(|s| s.verifier_ms).collect()),
                input_claim_check_ms: timing(
                    selected.iter().map(|s| s.input_claim_check_ms).collect(),
                ),
                payload_field_elements: first.payload_field_elements,
                payload_field_bytes: first.payload_field_bytes,
                bincode_bytes: first.bincode_bytes,
            };
            println!(
                "  median n=2^{log_n} {:9}: {:.3} ms prove [{:.3}, {:.3}], {:.3} ms verify",
                system.name(),
                summary.prover_ms.median,
                summary.prover_ms.min,
                summary.prover_ms.max,
                summary.verifier_ms.median
            );
            report.summaries.push(summary);
        }
        // Preserve completed dimensions if a later larger benchmark fails.
        if let Some(path) = &options.output {
            write_report(path, &report)?;
        }
    }
    if let Some(path) = &options.output {
        println!("JSON: {}", path.display());
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
        );
    }
    Ok(())
}

fn run_one(
    system: System,
    log_n: usize,
    run: usize,
    order: usize,
    a: &[u8],
    b: &[u8],
    c: &[u8],
    digest: &[u8; 32],
    rss: bool,
    profile: bool,
    scratch: &mut mikros::ProverScratch,
) -> Result<Sample, String> {
    let mut prover = system.challenger(digest);
    let rss_before = if rss {
        linux_status_kib("VmRSS:")
    } else {
        None
    };
    let mut mikros_phases = None;
    let (prover_ms, verifier_ms, input_claim_check_ms, fields, bincode_bytes, proof_digest, peak) =
        match system {
            system if system != System::Flock => {
                let kernel = system.kernel();
                let start = Instant::now();
                let reuses_capacity = matches!(system, System::Reuse | System::FactoredReuse);
                let result = if reuses_capacity && profile {
                    mikros::hadamard::prove_profiled_with_scratch(
                        black_box(a),
                        black_box(b),
                        black_box(c),
                        log_n,
                        kernel,
                        &mut prover,
                        scratch,
                    )
                    .map(|(proof, claims, phases)| {
                        mikros_phases = Some(phases);
                        (proof, claims)
                    })
                } else if reuses_capacity {
                    mikros::prove_with_scratch(
                        black_box(a),
                        black_box(b),
                        black_box(c),
                        log_n,
                        kernel,
                        &mut prover,
                        scratch,
                    )
                } else if profile {
                    mikros::hadamard::prove_profiled(
                        black_box(a),
                        black_box(b),
                        black_box(c),
                        log_n,
                        kernel,
                        &mut prover,
                    )
                    .map(|(proof, claims, phases)| {
                        mikros_phases = Some(phases);
                        (proof, claims)
                    })
                } else {
                    mikros::prove(
                        black_box(a),
                        black_box(b),
                        black_box(c),
                        log_n,
                        kernel,
                        &mut prover,
                    )
                };
                let prover_ms = millis(start);
                let peak = if rss {
                    linux_status_kib("VmHWM:")
                } else {
                    None
                };
                let (proof, claims) =
                    result.map_err(|e| format!("{} prove: {e:?}", system.name()))?;
                let mut verifier = system.challenger(digest);
                let start = Instant::now();
                let checked = mikros::verify(black_box(&proof), log_n, &mut verifier);
                let verifier_ms = millis(start);
                let checked = checked.map_err(|e| format!("{} verify: {e:?}", system.name()))?;
                if claims != checked {
                    return Err(format!("{} prover/verifier claims differ", system.name()));
                }
                if prover.sample_f128() != verifier.sample_f128() {
                    return Err(format!("{} transcript states differ", system.name()));
                }
                let start = Instant::now();
                mikros::check_claims(a, b, c, log_n, &checked)
                    .map_err(|e| format!("{} original-input check: {e:?}", system.name()))?;
                let input_claim_check_ms = millis(start);
                let (bytes, proof_digest) = proof_encoding(&proof)?;
                (
                    prover_ms,
                    verifier_ms,
                    input_claim_check_ms,
                    proof.field_elements(),
                    bytes,
                    proof_digest,
                    peak,
                )
            }
            _ => {
                let start = Instant::now();
                let (proof, claims) = zerocheck::prove_packed(
                    black_box(a),
                    black_box(b),
                    black_box(c),
                    log_n,
                    &mut prover,
                );
                let prover_ms = millis(start);
                let peak = if rss {
                    linux_status_kib("VmHWM:")
                } else {
                    None
                };
                let mut verifier = system.challenger(digest);
                let start = Instant::now();
                let checked = zerocheck::verify(log_n, black_box(&proof), &mut verifier);
                let verifier_ms = millis(start);
                let checked = checked.map_err(|e| format!("Flock verify: {e:?}"))?;
                if claims != checked {
                    return Err("Flock prover/verifier claims differ".into());
                }
                if prover.sample_f128() != verifier.sample_f128() {
                    return Err("Flock transcript states differ".into());
                }
                let start = Instant::now();
                flock_baseline::check_claims(a, b, c, log_n, &checked)?;
                let input_claim_check_ms = millis(start);
                let fields = proof.round1_ab.len()
                    + proof.round1_c.len()
                    + 2 * proof.multilinear_rounds.len()
                    + 3;
                let (bytes, proof_digest) = proof_encoding(&proof)?;
                (
                    prover_ms,
                    verifier_ms,
                    input_claim_check_ms,
                    fields,
                    bytes,
                    proof_digest,
                    peak,
                )
            }
        };
    Ok(Sample {
        log_n,
        system,
        run,
        order,
        input_digest: hex(digest),
        prover_ms,
        verifier_ms,
        input_claim_check_ms,
        payload_field_elements: fields,
        payload_field_bytes: fields * 16,
        bincode_bytes,
        proof_digest,
        vm_rss_before_prove_kib: rss_before,
        vm_hwm_after_prove_kib: peak,
        mikros_phases,
    })
}

fn proof_encoding<T: Serialize>(proof: &T) -> Result<(u64, String), String> {
    let encoded = bincode::serialize(proof).map_err(|e| e.to_string())?;
    Ok((
        encoded.len() as u64,
        blake3::hash(&encoded).to_hex().to_string(),
    ))
}

fn witness(log_n: usize, run: usize) -> (Vec<u8>, Vec<u8>, Vec<u8>, [u8; 32]) {
    let mut rng = Rng::new(0x6D69_6B72_6F73_0001 ^ ((log_n as u64) << 32) ^ run as u64);
    let mut a = vec![0; (1usize << log_n) / 8];
    let mut b = a.clone();
    rng.fill_bytes(&mut a);
    rng.fill_bytes(&mut b);
    let c: Vec<u8> = a.iter().zip(&b).map(|(x, y)| x & y).collect();
    let mut hash = blake3::Hasher::new();
    hash.update(b"mikros-flock-input-v1");
    hash.update(&(log_n as u64).to_le_bytes());
    for column in [&a, &b, &c] {
        hash.update(&(column.len() as u64).to_le_bytes());
        hash.update(column);
    }
    (a, b, c, *hash.finalize().as_bytes())
}

fn millis(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

fn timing(mut values: Vec<f64>) -> Timing {
    values.sort_by(f64::total_cmp);
    let n = values.len();
    let median = if n.is_multiple_of(2) {
        (values[n / 2 - 1] + values[n / 2]) / 2.0
    } else {
        values[n / 2]
    };
    Timing {
        median,
        min: values[0],
        max: values[n - 1],
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn linux_status_kib(key: &str) -> Option<u64> {
    if !cfg!(target_os = "linux") {
        return None;
    }
    fs::read_to_string("/proc/self/status")
        .ok()?
        .lines()
        .find_map(|line| {
            let value = line.strip_prefix(key)?.split_whitespace().next()?;
            value.parse().ok()
        })
}

fn command_output(program: &str, args: &[&str], directory: &Path) -> Option<String> {
    let output = Command::new(program)
        .args(args)
        .current_dir(directory)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn metadata(options: &Options) -> serde_json::Value {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let git_status = command_output("git", &["status", "--porcelain"], root);
    // Embed sources at build time: a dirty Git revision alone cannot identify
    // which engineering candidate produced an archived measurement.
    let mut source_hash = blake3::Hasher::new();
    for (name, source) in [
        ("Cargo.toml", include_str!("../Cargo.toml")),
        ("hadamard.rs", include_str!("../src/hadamard.rs")),
        ("tensor.rs", include_str!("../src/tensor.rs")),
        ("tensor/packed.rs", include_str!("../src/tensor/packed.rs")),
        (
            "tensor/packed/x86_64.rs",
            include_str!("../src/tensor/packed/x86_64.rs"),
        ),
        ("lookup.rs", include_str!("../src/lookup.rs")),
        ("lookup/x86_64.rs", include_str!("../src/lookup/x86_64.rs")),
        ("inner_product.rs", include_str!("../src/inner_product.rs")),
        (
            "inner_product/x86_64.rs",
            include_str!("../src/inner_product/x86_64.rs"),
        ),
        ("weights.rs", include_str!("../src/weights.rs")),
        ("layout.rs", include_str!("../src/layout.rs")),
        (
            "flock_baseline.rs",
            include_str!("../src/flock_baseline.rs"),
        ),
        ("benchmark.rs", include_str!("hadamard_vs_flock.rs")),
    ] {
        source_hash.update(&(name.len() as u64).to_le_bytes());
        source_hash.update(name.as_bytes());
        source_hash.update(&(source.len() as u64).to_le_bytes());
        source_hash.update(source.as_bytes());
    }
    let cpu = fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|text| {
            text.lines().find_map(|line| {
                line.strip_prefix("model name")
                    .and_then(|s| s.split_once(':'))
                    .map(|(_, s)| s.trim().to_owned())
            })
        })
        .or_else(|| env::var("PROCESSOR_IDENTIFIER").ok());
    let features: Vec<&str> = [
        ("aes", cfg!(target_feature = "aes")),
        ("pclmulqdq", cfg!(target_feature = "pclmulqdq")),
        ("sse4.1", cfg!(target_feature = "sse4.1")),
        ("gfni", cfg!(target_feature = "gfni")),
        ("avx2", cfg!(target_feature = "avx2")),
        ("avx512f", cfg!(target_feature = "avx512f")),
        ("avx512bw", cfg!(target_feature = "avx512bw")),
        ("avx512vbmi", cfg!(target_feature = "avx512vbmi")),
        ("vpclmulqdq", cfg!(target_feature = "vpclmulqdq")),
        ("neon", cfg!(target_feature = "neon")),
    ]
    .into_iter()
    .filter_map(|(name, enabled)| enabled.then_some(name))
    .collect();
    // Record only inspected performance knobs; never enumerate arbitrary
    // environment variables, which may contain credentials.
    let tuning_environment: serde_json::Map<String, serde_json::Value> = [
        "FLOCK_SPARSE_GATE",
        "FLOCK_ZC_TIMING",
        "FLOCK_POOL_TRACE",
        "RAYON_NUM_THREADS",
        "RUSTFLAGS",
    ]
    .into_iter()
    .filter_map(|key| {
        env::var(key)
            .ok()
            .map(|value| (key.to_owned(), serde_json::Value::String(value)))
    })
    .collect();
    serde_json::json!({
        "unix_timestamp_seconds": SystemTime::now().duration_since(UNIX_EPOCH).ok().map(|d| d.as_secs()),
        "os": env::consts::OS, "arch": env::consts::ARCH, "cpu": cpu,
        "rustc": option_env!("MIKROS_BUILD_RUSTC"),
        "git_revision": command_output("git", &["rev-parse", "HEAD"], root),
        "git_dirty": git_status.as_ref().map(|status| !status.is_empty()),
        "git_status": git_status, "compiled_target_features": features,
        "repository_default_rustflags": "-C target-cpu=native", "tuning_environment": tuning_environment,
        "threads": options.threads, "rayon_actual_threads": rayon::current_num_threads(),
        "mikros_phase_profiling": options.profile,
        "mikros_sources_blake3": source_hash.finalize().to_hex().to_string(),
        "mikros_kernel_variants": "reference: scalar oracle; lookup: original per-row LUT; packed8: eight-row blocks; packed: selected packed blocks; both packed variants use fused binding and fused IP tail",
        "native_ablation_variants": "legacy: old64; stream: streamed tensor; simdv1: GFNI horizontal reduction; simd: GFNI byte planes; wide: plus VPCLMUL IP; fused: plus packing/initial-message and C projection; native: plus blocked row weights; reuse: native with persistent allocation capacity; factored: native with tensor-factored row aggregation; factoredreuse: factored with persistent allocation capacity",
        "scratch_policy": "reuse and factoredreuse each retain independent buffers across proofs; warmup stores first allocation; all witness/challenge-dependent contents rebuilt within prove; every other Mikros variant allocates fresh scratch per proof",
        "proof_digest": "BLAKE3 of bincode proof outside all timings; identical Mikros kernels must have identical digests on paired input",
        "logs": options.logs, "runs": options.runs, "systems": options.systems,
        "mode": if options.rss { "fresh_process_rss_cold_proof" } else { "paired_warm_timing" },
        "warmups_per_system_per_dimension": if options.rss { 0 } else { 1 },
        "transcript_hash": "blake3", "grinding": "disabled for both; no equal-128-bit-security claim",
        "scope": "PIOP reductions only; no PCS commitments or openings; every returned claim checked directly against original input outside timings",
        "input": "deterministic packed uniform a,b; c=a&b; same per-run triple and observed BLAKE3 input digest for all systems",
        "timing": "input generation, digest, challenger initialization, serialization, direct input claim checks excluded; all per-proof work inside prove included",
        "measured_setup": {"isolated_table_setup_ms": null, "policy": "warmup samples include first-proof setup; no setup costs subtracted from prover timing"},
        "memory": "Linux-only process VmHWM after prove and before verify/input checks; includes inputs, runtime and tables; requires --rss with one system/dimension in a fresh process; unavailable elsewhere"
    })
}

fn write_report(path: &Path, report: &Report) -> Result<(), String> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    let json = serde_json::to_string_pretty(report).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| format!("write {}: {e}", path.display()))
}

fn parse_options() -> Result<Option<Options>, String> {
    let mut options = Options {
        logs: vec![13, 16, 18, 20, 22, 24],
        runs: 5,
        threads: 1,
        systems: vec![
            System::Reference,
            System::Lookup,
            System::Packed,
            System::Flock,
        ],
        output: None,
        rss: false,
        profile: false,
    };
    let mut args = env::args().skip(1);
    while let Some(argument) = args.next() {
        match argument.as_str() {
            "--help" | "-h" => {
                println!(
                    "hadamard_vs_flock [--logs 13,16,20] [--runs 5] [--threads 1] [--systems reference,lookup,packed,packed8,legacy,stream,simdv1,simd,wide,fused,native,reuse,factored,factoredreuse,flock] [--output results.json] [--rss] [--profile]\n--rss requires one system/dimension; executes one cold proof without warmup.\n--profile records Mikros phases inside the actual prover; Flock phases use FLOCK_ZC_TIMING=1 on stderr."
                );
                return Ok(None);
            }
            "--bench" => {}
            "--rss" => options.rss = true,
            "--profile" => options.profile = true,
            "--logs" | "--runs" | "--threads" | "--systems" | "--output" => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("missing value for {argument}"))?;
                match argument.as_str() {
                    "--logs" => {
                        options.logs = value
                            .split(',')
                            .map(|v| v.parse().map_err(|_| format!("invalid dimension {v:?}")))
                            .collect::<Result<_, _>>()?
                    }
                    "--runs" => {
                        options.runs = value.parse().map_err(|_| "invalid --runs".to_owned())?
                    }
                    "--threads" => {
                        options.threads =
                            value.parse().map_err(|_| "invalid --threads".to_owned())?
                    }
                    "--systems" => {
                        options.systems = value
                            .split(',')
                            .map(|v| match v {
                                "reference" => Ok(System::Reference),
                                "lookup" => Ok(System::Lookup),
                                "packed" => Ok(System::Packed),
                                "packed8" => Ok(System::Packed8),
                                "legacy" => Ok(System::Legacy),
                                "stream" => Ok(System::Stream),
                                "simd" => Ok(System::Simd),
                                "simdv1" => Ok(System::SimdV1),
                                "wide" => Ok(System::Wide),
                                "fused" => Ok(System::Fused),
                                "native" => Ok(System::Native),
                                "reuse" => Ok(System::Reuse),
                                "factored" => Ok(System::Factored),
                                "factoredreuse" => Ok(System::FactoredReuse),
                                "flock" => Ok(System::Flock),
                                _ => Err(format!("unknown system {v:?}")),
                            })
                            .collect::<Result<_, _>>()?
                    }
                    "--output" => options.output = Some(value.into()),
                    _ => unreachable!(),
                }
            }
            _ => return Err(format!("unknown argument {argument:?}; use --help")),
        }
    }
    if options.runs == 0
        || options.threads == 0
        || options.logs.is_empty()
        || options.systems.is_empty()
    {
        return Err("runs, threads, dimensions and systems must be nonempty/nonzero".into());
    }
    for (i, &log_n) in options.logs.iter().enumerate() {
        if log_n < 7
            || log_n >= usize::BITS as usize
            || (options.systems.contains(&System::Flock) && log_n < 13)
        {
            return Err(format!(
                "unsupported benchmark dimension {log_n}; Flock requires at least 13"
            ));
        }
        if options.logs[..i].contains(&log_n) {
            return Err("duplicate dimensions".into());
        }
    }
    for (i, system) in options.systems.iter().enumerate() {
        if options.systems[..i].contains(system) {
            return Err("duplicate systems".into());
        }
    }
    if options.rss {
        if options.logs.len() != 1 || options.systems.len() != 1 {
            return Err("--rss needs exactly one --logs dimension and one --systems entry in a fresh process".into());
        }
        options.runs = 1;
    }
    // Cargo starts a benchmark in its package directory, while invoking the
    // executable directly keeps the caller's cwd. Make documented output paths
    // deterministic in both cases: relative paths are repository-relative.
    if let Some(path) = &mut options.output
        && path.is_relative()
    {
        *path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("workspace package has a parent")
            .join(&*path);
    }
    Ok(Some(options))
}
