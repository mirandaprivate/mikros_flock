//! Matched kappa=128 PIOP benchmark, independent source checks outside timing.
#[path = "support/source_checks.rs"]
mod source_checks;
use flock_core::zerocheck;
use flock_field::test_rng::Rng;
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros::{Kernel, ProverScratch};
use serde_json::{Value, json};
use std::{
    env, fs,
    path::PathBuf,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

fn main() {
    if let Err(error) = run() {
        eprintln!("benchmark failed: {error}");
        std::process::exit(1);
    }
}
fn run() -> Result<(), String> {
    let mut logs = vec![16, 20];
    let mut systems = vec!["pairreuse".to_string(), "flock128".to_string()];
    let mut runs = 6usize;
    let mut threads = 1usize;
    let mut output = None;
    let mut profile = false;
    let mut rss = false;
    let mut reuse_source_checks = false;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--bench" => (),
            "--profile" => profile = true,
            "--rss" => rss = true,
            "--reuse-source-checks" => reuse_source_checks = true,
            "--help" | "-h" => {
                println!(
                    "univariate_vs_flock --logs 16,20 --runs 6 --threads 64 --systems reference,lookup,native,reuse,tiled,tiledreuse,basis,basisreuse,register,registerreuse,registerpacked,registerpackedreuse,split,splitreuse,half,halfreuse,pair,pairreuse,affine,affinereuse,joint,jointreuse,coefflookup,coefflookupreuse,coeffnative,coeffnativereuse,coeffclmul,coeffclmulreuse,coeffnibble,coeffnibblereuse,flock128 --output FILE [--profile] [--rss] [--reuse-source-checks]"
                );
                return Ok(());
            }
            "--logs" | "--systems" | "--runs" | "--threads" | "--output" => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("missing value for {arg}"))?;
                match arg.as_str() {
                    "--logs" => {
                        logs = value
                            .split(',')
                            .map(|x| x.parse::<usize>().map_err(|e| e.to_string()))
                            .collect::<Result<_, _>>()?
                    }
                    "--systems" => systems = value.split(',').map(str::to_string).collect(),
                    "--runs" => runs = value.parse::<usize>().map_err(|e| e.to_string())?,
                    "--threads" => threads = value.parse::<usize>().map_err(|e| e.to_string())?,
                    "--output" => {
                        let path = PathBuf::from(value);
                        output = Some(if path.is_absolute() {
                            path
                        } else {
                            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                                .parent()
                                .unwrap()
                                .join(path)
                        });
                    }
                    _ => unreachable!(),
                }
            }
            _ => return Err(format!("unknown option {arg}")),
        }
    }
    if runs == 0 || threads == 0 || logs.is_empty() || systems.is_empty() {
        return Err("nonempty logs/systems and positive runs/threads required".into());
    }
    for (index, system) in systems.iter().enumerate() {
        if ![
            "reference",
            "lookup",
            "native",
            "reuse",
            "tiled",
            "tiledreuse",
            "basis",
            "basisreuse",
            "register",
            "registerreuse",
            "registerpacked",
            "registerpackedreuse",
            "split",
            "splitreuse",
            "half",
            "halfreuse",
            "pair",
            "pairreuse",
            "affine",
            "affinereuse",
            "joint",
            "jointreuse",
            "coefflookup",
            "coefflookupreuse",
            "coeffnative",
            "coeffnativereuse",
            "coeffclmul",
            "coeffclmulreuse",
            "coeffnibble",
            "coeffnibblereuse",
            "flock128",
        ]
        .contains(&system.as_str())
            || systems[..index].contains(system)
        {
            return Err(format!("unknown or duplicate system {system}"));
        }
    }
    for &log in &logs {
        if !(7..=30).contains(&log) || (systems.iter().any(|s| s == "flock128") && log < 14) {
            return Err(format!("unsupported log dimension {log}"));
        }
    }
    if rss && (systems.len() != 1 || logs.len() != 1 || runs != 1) {
        return Err("--rss requires one system, one dimension, --runs 1 in a fresh process".into());
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .stack_size(8 << 20)
        .build_global()
        .map_err(|e| e.to_string())?;
    if !flock_parallel::init_all_core_pool(threads) {
        return Err("Flock pool already initialized".into());
    }
    let mut report = json!({
        "schema_version": 1,
        "metadata": {
            "protocol": "mikros-univariate-gf256-v1", "latex": "latex/4-block.tex",
            "scope": "Direct packed-opening PIOP only; no extra column-to-tensor aggregation or PCS; all source claims independently checked outside timings",
            "kappa": 128, "intermediate_field_bits": 8, "extension_coordinates": 16,
            "flock_skip_rounds": 7, "grinding": "disabled; no equal-security claim",
            "threads": threads, "rayon_actual_threads": rayon::current_num_threads(),
            "os": env::consts::OS, "arch": env::consts::ARCH,
            "cpu": cpu_name(), "compiled_target_features": compiled_features(),
            "build_configuration_blake3": source_hash(&[
                ("Cargo.toml", include_bytes!("../../Cargo.toml")),
                ("Cargo.lock", include_bytes!("../../Cargo.lock")),
                (".cargo/config.toml", include_bytes!("../../.cargo/config.toml")),
            ]),
            "native_backend": mikros::univariate::native_backend(), "rustc": option_env!("MIKROS_BUILD_RUSTC"),
            "carryless_backend": mikros::univariate::carryless_backend(),
            "mikros_sources_blake3": source_hash(include!(concat!(env!("OUT_DIR"), "/mikros_sources.rs"))),
            "flock_sources_blake3": source_hash(include!(concat!(env!("OUT_DIR"), "/flock_sources.rs"))),
            "unix_timestamp_seconds": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            "systems": systems, "logs": logs, "runs": runs, "profile": profile,
            "mode": if rss { "fresh_process_rss_cold_proof" } else if profile { "paired_profile_diagnostic" } else { "paired_warm_timing" },
            "position_balanced": runs.is_multiple_of(systems.len()),
            "kernel_scope": "reference/lookup/native/tiled/basis/register/split/half/pair/affine/joint select the first aggregation only; native/reuse preserve the v5 kernel; basis/basisreuse retain v6 batched coordinate conversion; register/registerreuse avoid its intermediate zeroing; registerpacked/registerpackedreuse also defer IP message horizontal folds within each packing task; split/splitreuse separate H and continuation accumulation in eight-coordinate loops; half/halfreuse group eight coordinates by sample half; pair/pairreuse unroll paired continuation interpolation with the original consume loop; affine/affinereuse replace split H masks with eight-row GFNI affine maps; joint/jointreuse combine paired interpolation and affine H; split/half/pair/affine/joint retain the v6 Batch conversion and original packing; every mode uses the same mathematical GF256 basis, C projection, weights and IP protocol with hardware fallbacks",
            "scratch": "reuse/tiledreuse/basisreuse/registerreuse/registerpackedreuse/splitreuse/halfreuse/pairreuse/affinereuse/jointreuse retain independent per-system allocation capacity, reset for each dimension; other Mikros modes allocate and deallocate per proof; all dynamic values rebuilt; Flock uses its production pool",
            "setup": "fixed tables initialized inside first proof and included in warmup/cold timing; no costs subtracted",
            "carryless_scope": "coeffclmul/coeffnibble encode directly to binary planes using 512 KiB byte or 64 KiB nibble tables; four-row VPCLMUL Karatsuba high convolution, AES reduction, decode and original register consumer; same 2048-row outer blocks as PairAggregate; all witness conversions online; safe original native/lookup fallback; corresponding reuse modes retain independent scratch capacity",
            "source_check_reuse": reuse_source_checks,
            "source_check_policy": "Every proof is verified and its transcript checked; each (dimension,run) has new immutable inputs and a new cache. First Mikros claims use the full source oracle; optional --reuse-source-checks permits later kernels only when their verifier-returned Claims equal the previously checked Claims exactly. Records identify full_oracle versus exact_claim_cache and the anchor system. Flock always uses its full oracle. All checks and comparisons are outside prover timing.",
            "coefficient_scope": "coefflookup/coeffnative use monomial row encoding and upper schoolbook products; coeffnative uses GFNI/AVX-512 when available; column claims and monomial quotient bytes are identical to existing kernels; interpolate_quotient_ms measures sparse division for these modes; witness-to-coefficient conversion and products are online and timed; only input-independent lookup tables persist; coefflookupreuse/coeffnativereuse retain independent per-system allocation capacity",
            "timing": "input generation/digest, challenger creation, serialization and source claim checks excluded; verifier is reduction replay only",
            "memory": "VmHWM read immediately after prove, before verify and source checks; includes input and runtime",
        },
        "warmups": [], "samples": [], "summaries": []
    });
    for &log in &logs {
        // Keep each system's retained capacity independent, including during
        // rotated ordering; no candidate inherits another candidate's buffers.
        let mut scratches: Vec<_> = systems.iter().map(|_| ProverScratch::default()).collect();
        for run in 0..runs + usize::from(!rss) {
            let warmup = !rss && run == 0;
            let (a, b, c, digest) = witness(log, run);
            let mut source_cache = source_checks::SourceCheckCache::new(&a, &b, &c, log);
            let mut identity = None;
            for order in 0..systems.len() {
                let system_index = (run + order) % systems.len();
                let system = &systems[system_index];
                let sample = sample(
                    system,
                    log,
                    run,
                    order,
                    &a,
                    &b,
                    &c,
                    &digest,
                    profile,
                    rss,
                    &mut scratches[system_index],
                    reuse_source_checks,
                    &mut source_cache,
                )?;
                if system != "flock128" {
                    let proof_digest = sample["proof_digest"].as_str().unwrap();
                    if let Some(expected) = &identity {
                        if expected != proof_digest {
                            return Err("Mikros kernels changed proof bytes".into());
                        }
                    } else {
                        identity = Some(proof_digest.to_owned());
                    }
                }
                println!(
                    "n={log} run={run} {system}: {:.3} ms, {} bytes{}",
                    sample["prover_ms"].as_f64().unwrap(),
                    sample["payload_field_bytes"],
                    if warmup { " (warmup)" } else { "" }
                );
                report[if warmup { "warmups" } else { "samples" }]
                    .as_array_mut()
                    .unwrap()
                    .push(sample);
                if let Some(path) = &output {
                    write_report(path, &report)?;
                }
            }
        }
    }
    let samples = report["samples"].as_array().unwrap();
    let mut summaries = Vec::new();
    for &log in &logs {
        for system in &systems {
            let selected: Vec<_> = samples
                .iter()
                .filter(|s| s["log_n"] == log && s["system"] == *system)
                .collect();
            let mut summary = json!({"log_n": log, "system": system, "runs": selected.len(), "payload_field_bytes": selected[0]["payload_field_bytes"], "bincode_bytes": selected[0]["bincode_bytes"]});
            for metric in ["prover_ms", "verifier_ms", "input_claim_check_ms"] {
                let mut values: Vec<_> = selected
                    .iter()
                    .map(|s| s[metric].as_f64().unwrap())
                    .collect();
                values.sort_by(f64::total_cmp);
                let len = values.len();
                let median = if len % 2 == 0 {
                    (values[len / 2 - 1] + values[len / 2]) / 2.0
                } else {
                    values[len / 2]
                };
                summary[metric] = json!({"median": median, "min": values[0], "max": values[len-1]});
            }
            summaries.push(summary);
        }
    }
    report["summaries"] = json!(summaries);
    if let Some(path) = &output {
        write_report(path, &report)?;
    }
    Ok(())
}

fn challenger(system: &str, digest: &[u8; 32]) -> FsChallenger {
    let label: &[u8] = if system == "flock128" {
        b"flock-zerocheck-skip7-bench-v1"
    } else {
        b"mikros-univariate-bench-v1"
    };
    let mut ch = FsChallenger::with_hash(label, HashKind::Blake3);
    ch.observe_bytes(digest);
    ch
}

#[allow(clippy::too_many_arguments)]
fn sample(
    system: &str,
    log: usize,
    run: usize,
    order: usize,
    a: &[u8],
    b: &[u8],
    c: &[u8],
    digest: &[u8; 32],
    profile: bool,
    rss: bool,
    scratch: &mut ProverScratch,
    reuse_source_checks: bool,
    source_cache: &mut source_checks::SourceCheckCache<'_>,
) -> Result<Value, String> {
    let mut prover = challenger(system, digest);
    let before = rss.then(|| status_kib("VmRSS:")).flatten();
    let start = Instant::now();
    let mut phases = Value::Null;
    let mut source_check_method = "full_oracle";
    let mut source_check_anchor = system.to_owned();
    let (prove_ms, verify_ms, check_ms, peak, fields, encoded) = if system == "flock128" {
        let (proof, claims) = zerocheck::prove_packed_skip7(a, b, c, log, &mut prover);
        let prove_ms = ms(start);
        let peak = rss.then(|| status_kib("VmHWM:")).flatten();
        let mut verifier = challenger(system, digest);
        let start = Instant::now();
        let checked =
            zerocheck::verify_skip7(log, &proof, &mut verifier).map_err(|e| format!("{e:?}"))?;
        let verify_ms = ms(start);
        if checked != claims || prover.sample_f128() != verifier.sample_f128() {
            return Err("Flock claim/transcript mismatch".into());
        }
        let start = Instant::now();
        mikros::flock_baseline::check_claims_with_skip(a, b, c, log, 7, &checked)?;
        let check_ms = ms(start);
        let fields =
            proof.round1_ab.len() + proof.round1_c.len() + 2 * proof.multilinear_rounds.len() + 3;
        if fields != 259 + 2 * (log - 7) {
            return Err("Flock payload shape changed".into());
        }
        (
            prove_ms,
            verify_ms,
            check_ms,
            peak,
            fields,
            bincode::serialize(&proof).map_err(|e| e.to_string())?,
        )
    } else {
        let kernel = match system {
            "reference" => Kernel::Reference,
            "lookup" => Kernel::Lookup,
            "native" | "reuse" => Kernel::Native,
            "tiled" | "tiledreuse" => Kernel::Tiled,
            "basis" | "basisreuse" => Kernel::TiledBasis,
            "register" | "registerreuse" => Kernel::RegisterBasis,
            "registerpacked" | "registerpackedreuse" => Kernel::RegisterPacked,
            "split" | "splitreuse" => Kernel::SplitAggregate,
            "half" | "halfreuse" => Kernel::HalfAggregate,
            "pair" | "pairreuse" => Kernel::PairAggregate,
            "affine" | "affinereuse" => Kernel::AffineAggregate,
            "joint" | "jointreuse" => Kernel::JointAggregate,
            "coefflookup" | "coefflookupreuse" => Kernel::CoefficientLookup,
            "coeffnative" | "coeffnativereuse" => Kernel::CoefficientNative,
            "coeffclmul" | "coeffclmulreuse" => Kernel::CoefficientCarryless,
            "coeffnibble" | "coeffnibblereuse" => Kernel::CoefficientCarrylessNibble,
            _ => unreachable!("validated Mikros system"),
        };
        let mut fresh = ProverScratch::default();
        let scratch = if matches!(
            system,
            "reuse"
                | "tiledreuse"
                | "basisreuse"
                | "registerreuse"
                | "registerpackedreuse"
                | "splitreuse"
                | "halfreuse"
                | "pairreuse"
                | "affinereuse"
                | "jointreuse"
                | "coefflookupreuse"
                | "coeffnativereuse"
                | "coeffclmulreuse"
                | "coeffnibblereuse"
        ) {
            scratch
        } else {
            &mut fresh
        };
        let (proof, claims) = if profile {
            let (p, c, stats) = mikros::hadamard::prove_profiled_with_scratch(
                a,
                b,
                c,
                log,
                kernel,
                &mut prover,
                scratch,
            )
            .map_err(|e| e.to_string())?;
            phases = serde_json::to_value(stats).unwrap();
            (p, c)
        } else {
            mikros::prove_with_scratch(a, b, c, log, kernel, &mut prover, scratch)
                .map_err(|e| e.to_string())?
        };
        // Fresh modes include deallocation, just like public prove(). Reuse
        // retains only its independent caller-owned buffers across proofs.
        drop(fresh);
        let prove_ms = ms(start);
        let peak = rss.then(|| status_kib("VmHWM:")).flatten();
        let mut verifier = challenger(system, digest);
        let start = Instant::now();
        let checked = mikros::verify(&proof, log, &mut verifier).map_err(|e| e.to_string())?;
        let verify_ms = ms(start);
        if checked != claims || prover.sample_f128() != verifier.sample_f128() {
            return Err("Mikros claim/transcript mismatch".into());
        }
        let start = Instant::now();
        let source_check =
            source_cache.check(a, b, c, log, &checked, system, reuse_source_checks)?;
        let check_ms = ms(start);
        source_check_method = source_check.method;
        source_check_anchor = source_check.anchor;
        let fields = proof.field_elements();
        if fields != 258 + 2 * (log - 7) {
            return Err("Mikros payload shape changed".into());
        }
        (
            prove_ms,
            verify_ms,
            check_ms,
            peak,
            fields,
            bincode::serialize(&proof).map_err(|e| e.to_string())?,
        )
    };
    Ok(json!({"system":system,"log_n":log,"run":run,"order":order,
        "input_digest":digest.iter().map(|b|format!("{b:02x}")).collect::<String>(),
        "prover_ms":prove_ms,"verifier_ms":verify_ms,"input_claim_check_ms":check_ms,
        "payload_field_elements":fields,"payload_field_bytes":fields*16,
        "bincode_bytes":encoded.len(),"proof_digest":blake3::hash(&encoded).to_hex().to_string(),
        "vm_rss_before_prove_kib":before,"vm_hwm_after_prove_kib":peak,"mikros_phases":phases,
        "source_check_method":source_check_method,"source_check_anchor":source_check_anchor,
        "verifier_passed":true,"transcript_matched":true,"source_claims_verified":true}))
}
fn cpu_name() -> Option<String> {
    fs::read_to_string("/proc/cpuinfo")
        .ok()?
        .lines()
        .find_map(|line| {
            line.strip_prefix("model name")
                .and_then(|s| s.split_once(':'))
                .map(|(_, s)| s.trim().to_owned())
        })
}
fn compiled_features() -> Vec<&'static str> {
    [
        ("gfni", cfg!(target_feature = "gfni")),
        ("avx512f", cfg!(target_feature = "avx512f")),
        ("avx512bw", cfg!(target_feature = "avx512bw")),
        ("avx512vbmi", cfg!(target_feature = "avx512vbmi")),
        ("vpclmulqdq", cfg!(target_feature = "vpclmulqdq")),
        ("pclmulqdq", cfg!(target_feature = "pclmulqdq")),
        ("avx2", cfg!(target_feature = "avx2")),
        ("sse4.1", cfg!(target_feature = "sse4.1")),
        ("neon", cfg!(target_feature = "neon")),
    ]
    .into_iter()
    .filter_map(|(name, on)| on.then_some(name))
    .collect()
}
fn source_hash(sources: &[(&str, &[u8])]) -> String {
    let mut hash = blake3::Hasher::new();
    for (name, source) in sources {
        hash.update(&(name.len() as u64).to_le_bytes());
        hash.update(name.as_bytes());
        hash.update(&(source.len() as u64).to_le_bytes());
        hash.update(source);
    }
    hash.finalize().to_hex().to_string()
}
fn witness(log: usize, run: usize) -> (Vec<u8>, Vec<u8>, Vec<u8>, [u8; 32]) {
    let mut rng = Rng::new(0x6D69_6B72_6F73_0001 ^ ((log as u64) << 32) ^ run as u64);
    let mut a = vec![0; 1usize << (log - 3)];
    let mut b = a.clone();
    rng.fill_bytes(&mut a);
    rng.fill_bytes(&mut b);
    let c: Vec<_> = a.iter().zip(&b).map(|(a, b)| a & b).collect();
    let mut hash = blake3::Hasher::new();
    hash.update(b"mikros-flock-input-v1");
    hash.update(&(log as u64).to_le_bytes());
    for input in [&a, &b, &c] {
        hash.update(&(input.len() as u64).to_le_bytes());
        hash.update(input);
    }
    (a, b, c, *hash.finalize().as_bytes())
}
fn ms(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}
fn status_kib(key: &str) -> Option<u64> {
    fs::read_to_string("/proc/self/status")
        .ok()?
        .lines()
        .find_map(|l| l.strip_prefix(key)?.split_whitespace().next()?.parse().ok())
}
fn write_report(path: &PathBuf, report: &Value) -> Result<(), String> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(path, serde_json::to_string_pretty(report).unwrap()).map_err(|e| e.to_string())
}
