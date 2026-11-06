//! Indexed lookup reduction on one fixed public address schedule, with and
//! without bitwidth-aware packing. This benchmark does not instantiate a PCS.
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros_permutation::{
    bits::PackedBits,
    indexed_lookup::{PublicSchedule, Statement, Strategy, prove, verify_profiled},
};
use serde::Serialize;
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    time::Instant,
};

type Result<T> = std::result::Result<T, String>;
const LABELS: [&str; 2] = ["Mikros packed", "Full-extension F128"];
const IDS: [&str; 2] = ["packed", "full_extension"];
const SEED: u64 = 0x4d49_4b52_4f53_1025;

#[derive(Serialize)]
struct Options {
    log_ns: Vec<usize>,
    bits: Vec<usize>,
    table_log: usize,
    threads: usize,
    reps: usize,
    warmups: usize,
    out: PathBuf,
}
fn list(value: &str) -> Result<Vec<usize>> {
    let values = value
        .split(',')
        .map(|s| {
            s.parse::<usize>()
                .map_err(|_| format!("invalid list: {value}"))
        })
        .collect::<Result<Vec<_>>>()?;
    if values.is_empty() || values.iter().collect::<BTreeSet<_>>().len() != values.len() {
        return Err("lists must be nonempty and contain distinct values".into());
    }
    Ok(values)
}
fn options() -> Result<Options> {
    let mut o = Options {
        log_ns: vec![12, 16, 20],
        bits: vec![1, 2, 4, 8, 16],
        table_log: 8,
        threads: 64,
        reps: 6,
        warmups: 1,
        out: PathBuf::from("target/paper-results/lookup-bitwidth"),
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--bench" {
            continue;
        }
        if arg == "--help" || arg == "-h" {
            println!(
                "cargo bench -p mikros --bench lookup_bitwidth -- --log-n 12,16,20 --bits 1,2,4,8,16 --table-log 8 --threads 64 --reps 6 --warmups 1 --out NEW_DIRECTORY\n\nN=2^log-n lookup queries, M=2^table-log table entries. The table values and public addresses use one deterministic seed. Each bitwidth masks the same table values. Sequential alternating methods; all proofs verified. Output directory must be new or empty. No PCS and no zero knowledge."
            );
            std::process::exit(0);
        }
        let value = args
            .next()
            .ok_or_else(|| format!("missing value for {arg}"))?;
        match arg.as_str() {
            "--log-n" => o.log_ns = list(&value)?,
            "--bits" => o.bits = list(&value)?,
            "--table-log" => o.table_log = value.parse().map_err(|_| "invalid table-log")?,
            "--threads" => o.threads = value.parse().map_err(|_| "invalid threads")?,
            "--reps" => o.reps = value.parse().map_err(|_| "invalid reps")?,
            "--warmups" => o.warmups = value.parse().map_err(|_| "invalid warmups")?,
            "--out" => o.out = value.into(),
            _ => return Err(format!("unknown option {arg}")),
        }
    }
    if o.log_ns.iter().any(|&n| !(1..=30).contains(&n))
        || o.bits.iter().any(|b| ![1, 2, 4, 8, 16].contains(b))
        || o.table_log > 24
        || !(1..=256).contains(&o.threads)
        || !(1..=100).contains(&o.reps)
        || o.warmups > 10
    {
        return Err("require log-n=1..30, bits in {1,2,4,8,16}, table-log=0..24, threads=1..256, reps=1..100, warmups=0..10 (the protocol also validates the combined tuple width)".into());
    }
    Ok(o)
}

struct Log(File);
impl Log {
    fn line(&mut self, message: impl AsRef<str>) -> Result<()> {
        let message = message.as_ref();
        println!("{message}");
        writeln!(self.0, "{message}").map_err(|e| e.to_string())?;
        self.0.flush().map_err(|e| e.to_string())
    }
}
fn ms(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}
fn mix(mut n: u64) -> u64 {
    n = (n ^ (n >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    n = (n ^ (n >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    n ^ (n >> 31)
}
fn table_word(index: usize) -> u128 {
    let n = index as u64;
    mix(n ^ SEED) as u128 | ((mix(n ^ SEED.rotate_left(29)) as u128) << 64)
}
fn address(index: usize, table_rows: usize) -> usize {
    (mix(index as u64 ^ SEED.rotate_right(13)) as usize) & (table_rows - 1)
}
fn median(mut xs: Vec<f64>) -> f64 {
    xs.sort_by(f64::total_cmp);
    let n = xs.len();
    if n.is_multiple_of(2) {
        (xs[n / 2 - 1] + xs[n / 2]) / 2.0
    } else {
        xs[n / 2]
    }
}
fn phases(value: impl Serialize) -> Result<BTreeMap<String, f64>> {
    let v = serde_json::to_value(value).map_err(|e| e.to_string())?;
    let object = v.as_object().ok_or("timings must serialize as an object")?;
    object
        .iter()
        .map(|(k, v)| {
            v.as_f64()
                .filter(|x| x.is_finite() && *x >= 0.0)
                .map(|x| (k.clone(), x))
                .ok_or_else(|| format!("invalid timing field: {k}"))
        })
        .collect()
}
fn write_json(path: impl AsRef<Path>, value: impl Serialize) -> Result<()> {
    fs::write(
        path,
        serde_json::to_vec_pretty(&value).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}

#[derive(Clone, Serialize)]
struct Sample {
    log_n: usize,
    query_rows: usize,
    table_rows: usize,
    value_bits: usize,
    position_bits: usize,
    tuple_bits: usize,
    packing_width: usize,
    segment_len: usize,
    encoded_rows: usize,
    d: usize,
    method: &'static str,
    threads: usize,
    sample: usize,
    order: usize,
    warmup: bool,
    prover_ms: f64,
    verifier_ms: f64,
    source_check_ms: f64,
    serialization_ms: f64,
    proof_bytes: usize,
    proof_field_elements: usize,
    coefficient_bytes: Option<usize>,
    tree_storage_bytes: Option<usize>,
    input_bytes: usize,
    public_schedule_bytes: usize,
    address_generation_ms: f64,
    public_plan_ms: f64,
    input_generation_ms: f64,
    statement_hash_ms: f64,
    prover_phases_ms: BTreeMap<String, f64>,
    verifier_phases_ms: BTreeMap<String, f64>,
    proof_blake3: String,
    verified: bool,
    transcript_match: bool,
    source_claims_checked: bool,
    deterministic_proof_match: bool,
}

fn run(o: &Options, log: &mut Log) -> Result<()> {
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(o.threads)
        .build()
        .map_err(|e| e.to_string())?;
    write_json(
        o.out.join("metadata.json"),
        json!({
            "options": o, "seed": SEED, "worker_count": pool.current_num_threads(),
            "architecture": std::env::consts::ARCH,
            "os": std::env::consts::OS,
            "rustflags_runtime": std::env::var("RUSTFLAGS").ok(),
            "scope": "indexed lookup PIOP with public address schedule; no PCS; no zero knowledge",
            "baseline": "Lasso-style full F128 on the same public schedule; not a native BabySpartan measurement",
            "sampling": "same input and transcript per method; alternate order; warmups excluded from medians",
            "prover_timer": "all prove() work, including source checks and witness tuple/coefficient preparation",
            "verifier_timer": "verify_profiled(), including linear public-schedule evaluation; excludes direct source-oracle checks",
            "excluded": "public schedule setup, input generation, Statement bit-oracle hashing, serialization, direct source-oracle checks",
            "storage": "logical input/coefficient/tree/schedule storage; not measured peak RSS",
            "scratch_policy": "method-specific pools retained across repetitions; all pools released after each bitwidth group"
        }),
    )?;
    log.line(format!(
        "Indexed lookup | {} workers | {} measured samples + {} excluded warmups | table M=2^{}",
        o.threads, o.reps, o.warmups, o.table_log
    ))?;
    log.line("Status: RUNNING. A completion.json is written only after every proof and source claim passes.")?;
    let mut samples = Vec::new();
    let mut setups = Vec::new();
    let mut raw = File::create(o.out.join("raw.jsonl")).map_err(|e| e.to_string())?;
    let sweep_start = Instant::now();
    for &log_n in &o.log_ns {
        let n = 1usize << log_n;
        let table_rows = 1usize << o.table_log;
        let start = Instant::now();
        let addresses: Vec<_> = (0..n).map(|i| address(i, table_rows)).collect();
        let address_generation_ms = ms(start);
        for &bits in &o.bits {
            log.line(format!("[lookup] setup N=2^{log_n}, value_bits={bits}"))?;
            let start = Instant::now();
            let plan = pool
                .install(|| PublicSchedule::new(table_rows, &addresses, bits))
                .map_err(|e| e.to_string())?;
            let public_plan_ms = ms(start);
            let start = Instant::now();
            let mask = if bits == 128 {
                u128::MAX
            } else {
                (1u128 << bits) - 1
            };
            let (values, table) = pool.install(|| -> Result<_> {
                let table =
                    PackedBits::from_fn_parallel(table_rows, bits, |i| table_word(i) & mask)
                        .map_err(|e| e.to_string())?;
                let values =
                    PackedBits::from_fn_parallel(n, bits, |i| table_word(addresses[i]) & mask)
                        .map_err(|e| e.to_string())?;
                Ok((values, table))
            })?;
            let input_generation_ms = ms(start);
            let start = Instant::now();
            let statement = pool
                .install(|| Statement::new(&plan, &values, &table))
                .map_err(|e| e.to_string())?;
            let statement_hash_ms = ms(start);
            log.line(format!(
                "[lookup] value_bits={} position_bits={} tuple_bits={} kappa'={} segment={} encoded_rows={} d={}; public_plan={public_plan_ms:.3} ms, inputs={input_generation_ms:.3} ms, statement_hash={statement_hash_ms:.3} ms",
                plan.value_bits(), plan.position_bits(), plan.tuple_bits(), plan.packing_width(),
                plan.segment_len(), plan.encoded_rows(), plan.d()
            ))?;
            let mut expected_hash: [Option<String>; 2] = [None, None];
            for round in 0..o.warmups + o.reps {
                let warmup = round < o.warmups;
                let sample = if warmup {
                    round + 1
                } else {
                    round - o.warmups + 1
                };
                let phase = if warmup { "warmup" } else { "sample" };
                let total = if warmup { o.warmups } else { o.reps };
                // Warmup and measured sequences each start with the packed method.
                let sequence = sample - 1;
                let order = if sequence.is_multiple_of(2) {
                    [0, 1]
                } else {
                    [1, 0]
                };
                for (order_index, method) in order.into_iter().enumerate() {
                    log.line(format!(
                        "[lookup] N=2^{log_n} b={bits} {} {phase} {sample}/{total} start",
                        LABELS[method]
                    ))?;
                    let strategy = if method == 0 {
                        Strategy::Packed
                    } else {
                        Strategy::FullExtension
                    };
                    let mut prover = FsChallenger::new(b"mikros-lookup-bitwidth-bench-v1");
                    let start = Instant::now();
                    let output = pool
                        .install(|| {
                            prove(&plan, &statement, &values, &table, strategy, &mut prover)
                        })
                        .map_err(|e| e.to_string())?;
                    let prover_ms = ms(start);
                    let mut verifier = FsChallenger::new(b"mikros-lookup-bitwidth-bench-v1");
                    let start = Instant::now();
                    let (claims, verify_timings) = pool
                        .install(|| {
                            verify_profiled(&plan, &statement, &output.proof, &mut verifier)
                        })
                        .map_err(|e| e.to_string())?;
                    let verifier_ms = ms(start);
                    if claims != output.claims || prover.sample_f128() != verifier.sample_f128() {
                        return Err(format!(
                            "claims/transcript mismatch at N=2^{log_n}, b={bits}, {}",
                            LABELS[method]
                        ));
                    }
                    let start = Instant::now();
                    pool.install(|| claims.check(&values, &table, &plan))
                        .map_err(|e| e.to_string())?;
                    let source_check_ms = ms(start);
                    let start = Instant::now();
                    let encoded = bincode::serialize(&output.proof).map_err(|e| e.to_string())?;
                    let serialization_ms = ms(start);
                    let proof_blake3 = blake3::hash(&encoded).to_hex().to_string();
                    if expected_hash[method]
                        .as_ref()
                        .is_some_and(|h| h != &proof_blake3)
                    {
                        return Err(format!(
                            "nondeterministic proof at N=2^{log_n}, b={bits}, {}",
                            LABELS[method]
                        ));
                    }
                    expected_hash[method] = Some(proof_blake3.clone());
                    let row = Sample {
                        log_n,
                        query_rows: n,
                        table_rows,
                        value_bits: bits,
                        position_bits: plan.position_bits(),
                        tuple_bits: plan.tuple_bits(),
                        packing_width: if method == 0 { plan.packing_width() } else { 1 },
                        segment_len: plan.segment_len(),
                        encoded_rows: plan.encoded_rows(),
                        d: plan.d(),
                        method: IDS[method],
                        threads: o.threads,
                        sample,
                        order: order_index,
                        warmup,
                        prover_ms,
                        verifier_ms,
                        source_check_ms,
                        serialization_ms,
                        proof_bytes: encoded.len(),
                        proof_field_elements: output.proof.field_elements(),
                        coefficient_bytes: (method == 0).then_some(output.coefficient_bytes),
                        tree_storage_bytes: (method == 1).then_some(output.tree_storage_bytes),
                        input_bytes: values.bytes().len() + table.bytes().len(),
                        public_schedule_bytes: plan.resident_bytes(),
                        address_generation_ms,
                        public_plan_ms,
                        input_generation_ms,
                        statement_hash_ms,
                        prover_phases_ms: phases(&output.timings)?,
                        verifier_phases_ms: phases(&verify_timings)?,
                        proof_blake3,
                        verified: true,
                        transcript_match: true,
                        source_claims_checked: true,
                        deterministic_proof_match: true,
                    };
                    serde_json::to_writer(&mut raw, &row).map_err(|e| e.to_string())?;
                    writeln!(raw).map_err(|e| e.to_string())?;
                    raw.flush().map_err(|e| e.to_string())?;
                    log.line(format!(
                        "[lookup] N=2^{log_n} b={bits} {} {phase} {sample}/{total} finished: prover={prover_ms:.3} ms verifier={verifier_ms:.3} ms proof={} bytes; source_check={source_check_ms:.3} ms; verified=true",
                        LABELS[method], row.proof_bytes
                    ))?;
                    samples.push(row);
                }
            }
            let start = Instant::now();
            mikros_permutation::tree_pool::clear_all();
            let scratch_teardown_ms = ms(start);
            setups.push(json!({
                "log_n": log_n, "value_bits": bits,
                "address_generation_ms": address_generation_ms,
                "public_plan_ms": public_plan_ms,
                "input_generation_ms": input_generation_ms,
                "statement_hash_ms": statement_hash_ms,
                "scratch_teardown_ms": scratch_teardown_ms
            }));
            write_json(o.out.join("setup.json"), &setups)?;
            log.line(format!("[lookup] N=2^{log_n} b={bits} group complete; scratch teardown={scratch_teardown_ms:.3} ms (excluded)"))?;
        }
    }
    let measured: Vec<_> = samples.iter().filter(|s| !s.warmup).collect();
    let expected = o.log_ns.len() * o.bits.len() * 2 * o.reps;
    if measured.len() != expected
        || samples.len() != o.log_ns.len() * o.bits.len() * 2 * (o.reps + o.warmups)
    {
        return Err("incomplete sample set".into());
    }
    write_json(o.out.join("raw.json"), &samples)?;
    csv(&o.out.join("raw.csv"), &samples)?;
    let summary = summarize(&samples);
    write_json(o.out.join("summary.json"), &summary)?;
    summary_csv(&o.out.join("summary.csv"), &summary)?;
    report(log, &summary)?;
    log.line(format!("Status: COMPLETE; {} measured proofs, {} including warmups; every proof, source claim, transcript, and deterministic repeat checked.", measured.len(), samples.len()))?;
    write_json(
        o.out.join("completion.json"),
        json!({
            "status": "complete", "measured_proofs": measured.len(),
            "verified_including_warmups": samples.len(), "all_verified": true,
            "all_source_claims_checked": true, "all_transcripts_match": true,
            "all_deterministic_repeats_match": true, "wall_seconds": sweep_start.elapsed().as_secs_f64()
        }),
    )
}

fn csv(path: &Path, samples: &[Sample]) -> Result<()> {
    let mut file = File::create(path).map_err(|e| e.to_string())?;
    let scalar_keys: Vec<String> = serde_json::to_value(&samples[0])
        .map_err(|e| e.to_string())?
        .as_object()
        .ok_or("sample must be an object")?
        .iter()
        .filter(|(_, value)| !value.is_object())
        .map(|(key, _)| key.clone())
        .collect();
    let phase_keys: BTreeSet<_> = samples
        .iter()
        .flat_map(|s| {
            s.prover_phases_ms
                .keys()
                .map(|k| format!("prover.{k}"))
                .chain(s.verifier_phases_ms.keys().map(|k| format!("verifier.{k}")))
        })
        .collect();
    let keys: Vec<_> = scalar_keys.into_iter().chain(phase_keys).collect();
    writeln!(file, "{}", keys.join(",")).map_err(|e| e.to_string())?;
    for s in samples {
        let object = serde_json::to_value(s).map_err(|e| e.to_string())?;
        let cells: Vec<_> = keys
            .iter()
            .map(|key| {
                let value = if let Some(k) = key.strip_prefix("prover.") {
                    json!(s.prover_phases_ms[k])
                } else if let Some(k) = key.strip_prefix("verifier.") {
                    json!(s.verifier_phases_ms[k])
                } else {
                    object[key].clone()
                };
                match value {
                    Value::String(s) => s,
                    other => other.to_string(),
                }
            })
            .collect();
        writeln!(file, "{}", cells.join(",")).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[derive(Serialize)]
struct Summary {
    log_n: usize,
    value_bits: usize,
    position_bits: usize,
    tuple_bits: usize,
    packing_width: usize,
    method: &'static str,
    samples: usize,
    prover_ms: f64,
    verifier_ms: f64,
    proof_bytes: usize,
    coefficient_bytes: Option<usize>,
    tree_storage_bytes: Option<usize>,
    input_bytes: usize,
    prover_phases_ms: BTreeMap<String, f64>,
    verifier_phases_ms: BTreeMap<String, f64>,
}
fn summarize(samples: &[Sample]) -> Vec<Summary> {
    let groups: BTreeSet<_> = samples
        .iter()
        .map(|s| (s.log_n, s.value_bits, s.method))
        .collect();
    groups
        .into_iter()
        .map(|(log_n, value_bits, method)| {
            let rows: Vec<_> = samples
                .iter()
                .filter(|s| {
                    !s.warmup
                        && s.log_n == log_n
                        && s.value_bits == value_bits
                        && s.method == method
                })
                .collect();
            let first = rows[0];
            let aggregate = |verify: bool| -> BTreeMap<String, f64> {
                let phases = if verify {
                    &first.verifier_phases_ms
                } else {
                    &first.prover_phases_ms
                };
                phases
                    .keys()
                    .map(|key| {
                        (
                            key.clone(),
                            median(
                                rows.iter()
                                    .map(|s| {
                                        if verify {
                                            s.verifier_phases_ms[key]
                                        } else {
                                            s.prover_phases_ms[key]
                                        }
                                    })
                                    .collect(),
                            ),
                        )
                    })
                    .collect()
            };
            Summary {
                log_n,
                value_bits,
                position_bits: first.position_bits,
                tuple_bits: first.tuple_bits,
                packing_width: first.packing_width,
                method,
                samples: rows.len(),
                prover_ms: median(rows.iter().map(|s| s.prover_ms).collect()),
                verifier_ms: median(rows.iter().map(|s| s.verifier_ms).collect()),
                proof_bytes: first.proof_bytes,
                coefficient_bytes: first.coefficient_bytes,
                tree_storage_bytes: first.tree_storage_bytes,
                input_bytes: first.input_bytes,
                prover_phases_ms: aggregate(false),
                verifier_phases_ms: aggregate(true),
            }
        })
        .collect()
}
fn summary_csv(path: &Path, summaries: &[Summary]) -> Result<()> {
    let mut file = File::create(path).map_err(|e| e.to_string())?;
    writeln!(file, "log_n,value_bits,position_bits,tuple_bits,kappa,method,samples,prover_ms,verifier_ms,proof_bytes,input_bytes,coefficient_bytes,tree_storage_bytes").map_err(|e| e.to_string())?;
    for s in summaries {
        writeln!(
            file,
            "{},{},{},{},{},{},{},{:.9},{:.9},{},{},{},{}",
            s.log_n,
            s.value_bits,
            s.position_bits,
            s.tuple_bits,
            s.packing_width,
            s.method,
            s.samples,
            s.prover_ms,
            s.verifier_ms,
            s.proof_bytes,
            s.input_bytes,
            s.coefficient_bytes
                .map_or_else(String::new, |v| v.to_string()),
            s.tree_storage_bytes
                .map_or_else(String::new, |v| v.to_string())
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
fn report(log: &mut Log, rows: &[Summary]) -> Result<()> {
    log.line("\nIndexed lookup comparison (medians; milliseconds; serialized proof bytes)")?;
    log.line(format!(
        "{:>6} {:>4} {:>5} {:>5} {:>6} {:>12} {:>12} {:>9} {:>12} {:>12}",
        "log N",
        "bits",
        "pos",
        "tuple",
        "kappa'",
        "Packed ms",
        "Full ms",
        "Full/Pack",
        "Packed B",
        "Full B"
    ))?;
    for packed in rows.iter().filter(|s| s.method == IDS[0]) {
        let full = rows
            .iter()
            .find(|s| {
                s.log_n == packed.log_n && s.value_bits == packed.value_bits && s.method == IDS[1]
            })
            .ok_or("missing method in summary")?;
        log.line(format!(
            "{:>6} {:>4} {:>5} {:>5} {:>6} {:>12.3} {:>12.3} {:>8.3}x {:>12} {:>12}",
            packed.log_n,
            packed.value_bits,
            packed.position_bits,
            packed.tuple_bits,
            packed.packing_width,
            packed.prover_ms,
            full.prover_ms,
            full.prover_ms / packed.prover_ms,
            packed.proof_bytes,
            full.proof_bytes
        ))?;
    }
    log.line(
        "\nVerifier and logical storage (MiB = 2^20 bytes; these are not RAM/RSS measurements)",
    )?;
    log.line(format!(
        "{:>6} {:>4} {:<19} {:>13} {:>11} {:>12} {:>11}",
        "log N", "bits", "method", "Verifier ms", "Input MiB", "Coeff MiB", "Tree MiB"
    ))?;
    for row in rows {
        let mib = |value: Option<usize>| {
            value.map_or_else(|| "N/A".into(), |v| format!("{:.3}", v as f64 / 1048576.0))
        };
        log.line(format!(
            "{:>6} {:>4} {:<19} {:>13.3} {:>11.3} {:>12} {:>11}",
            row.log_n,
            row.value_bits,
            row.method,
            row.verifier_ms,
            row.input_bytes as f64 / 1048576.0,
            mib(row.coefficient_bytes),
            mib(row.tree_storage_bytes)
        ))?;
    }
    log.line("\nProver includes witness tuple conversion, coefficient preparation, internal source checks, and the reduction. Public schedule setup, input generation, Statement source hashing, serialization, and direct source-oracle checks are excluded and recorded separately in raw data.")?;
    log.line("Verifier includes public-schedule linear evaluation. Direct source claims were checked outside this timer; no PCS is instantiated. Proof bytes count bincode PIOP messages. This is not a zero-knowledge implementation.")?;
    log.line("Full-extension F128 is a Lasso-style baseline on the identical public schedule, not a native BabySpartan measurement. Position/counter widths can dominate tuple width, so increasing value bitwidth need not change kappa'. Phase medians are reported in summary.json; medians of phases need not sum to the median total.")?;
    log.line("Storage N/A means that backend does not expose the corresponding statistic, not zero allocation. Coefficient and tree statistics cover different objects and must not be compared as peak memory. Method-specific scratch pools are retained across repetitions and released after each bitwidth group.")
}

fn main() {
    let result = (|| -> Result<()> {
        let o = options()?;
        if o.out.exists()
            && fs::read_dir(&o.out)
                .map_err(|e| e.to_string())?
                .next()
                .is_some()
        {
            return Err(format!(
                "output directory is not empty: {}; choose a new --out directory",
                o.out.display()
            ));
        }
        fs::create_dir_all(&o.out).map_err(|e| e.to_string())?;
        let mut log =
            Log(File::create(o.out.join("lookup_bitwidth.log")).map_err(|e| e.to_string())?);
        let result = run(&o, &mut log);
        if let Err(error) = &result {
            let _ = log.line(format!("Status: FAILED; {error}"));
            let _ = write_json(
                o.out.join("failure.json"),
                json!({"status":"failed","error":error}),
            );
        }
        result
    })();
    if let Err(error) = result {
        eprintln!("lookup-bitwidth comparison failed: {error}");
        std::process::exit(1);
    }
}
