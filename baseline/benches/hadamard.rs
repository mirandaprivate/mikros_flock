//! Paired, verified PIOP timings. Inputs/serialization/oracle checks are untimed.
use std::{env, fs, path::PathBuf, time::Instant};

use flock_field::test_rng::Rng;
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use hadamard_baselines::{Method, Statement, check_claims, plonky3, prove, verify};
use p3_binary_field::TowerLevel;
use p3_challenger::{CanObserve, FieldChallenger};
use p3_field::{Field, PackedValue};
use serde_json::json;

#[path = "../support/flock_oracle.rs"]
mod flock_oracle;

fn main() {
    if let Err(e) = run() {
        eprintln!("benchmark failed: {e}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut log_n = 16usize;
    let mut runs = 3usize;
    let mut threads = 1usize;
    let mut output = None::<PathBuf>;
    let mut systems = vec![
        "small_field_first".to_owned(),
        "dao_thaler".to_owned(),
        "flock".to_owned(),
    ];
    let mut args = env::args().skip(1);
    while let Some(key) = args.next() {
        // Cargo appends this marker even for harness=false benchmarks.
        if key == "--bench" {
            continue;
        }
        if key == "--help" {
            println!(
                "hadamard --log-n 16 --runs 3 --threads 1 --systems small_field_first,dao_thaler,flock --output result.json\nOptional controls: small_field_first_full_extension,dao_thaler_generic,ghash_small_field_first,ghash_full_extension"
            );
            return Ok(());
        }
        let value = args
            .next()
            .ok_or_else(|| format!("missing value for {key}"))?;
        match key.as_str() {
            "--log-n" => log_n = value.parse().map_err(|_| "invalid log-n")?,
            "--runs" => runs = value.parse().map_err(|_| "invalid runs")?,
            "--threads" => threads = value.parse().map_err(|_| "invalid threads")?,
            "--systems" => systems = value.split(',').map(str::to_owned).collect(),
            "--output" => output = Some(value.into()),
            _ => return Err(format!("unknown option {key}")),
        }
    }
    if runs == 0 || threads == 0 || log_n > 30 || systems.is_empty() {
        return Err("nonzero runs/threads/systems and log-n <= 30 required".into());
    }
    for (i, system) in systems.iter().enumerate() {
        if systems[..i].contains(system) {
            return Err("duplicate system".into());
        }
        if system != "flock" && native_method(system).is_none() {
            method(system)?;
        }
        if system == "flock" && log_n < 13 {
            return Err("Flock requires log-n >= 13".into());
        }
    }
    for key in ["FLOCK_ZC_TIMING", "FLOCK_POOL_TRACE", "FLOCK_SPARSE_GATE"] {
        if env::var_os(key).is_some() {
            return Err(format!("unset {key} for an ordinary comparison"));
        }
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()
        .map_err(|e| e.to_string())?;
    let mut samples = Vec::new();
    let mut warmups = Vec::new();
    for run in 0..=runs {
        let n = 1usize << log_n;
        let mut rng = Rng::new(0x6d69_6b72_6f73_0001 ^ ((log_n as u64) << 32) ^ run as u64);
        let mut a = vec![0; n.div_ceil(8)];
        let mut b = a.clone();
        rng.fill_bytes(&mut a);
        rng.fill_bytes(&mut b);
        if n < 8 {
            a[0] &= ((1u16 << n) - 1) as u8;
            b[0] &= ((1u16 << n) - 1) as u8;
        }
        let c: Vec<u8> = a.iter().zip(&b).map(|(a, b)| a & b).collect();
        let statement = Statement::from_inputs(log_n, &a, &b, &c)?;
        let mut order = systems.clone();
        if run > 0 {
            let len = order.len();
            order.rotate_left(((run - 1) / 2) % len);
            if run % 2 == 0 {
                order.reverse();
            }
        }
        for (position, name) in order.iter().enumerate() {
            let ch = || {
                FsChallenger::with_hash(b"hadamard-baselines/paired-benchmark/v1", HashKind::Blake3)
            };
            let mut pc = ch();
            let mut vc = ch();
            // Source ID binding is outside prover time for all systems.
            pc.observe_bytes(&statement.input_digest);
            vc.observe_bytes(&statement.input_digest);
            let (prover_ms, verifier_ms, check_ms, bytes, digest) =
                if let Some(method) = native_method(name) {
                    let mut npc = plonky3::challenger(b"hadamard-baselines/paired-benchmark/v1");
                    let mut nvc = npc.clone();
                    for byte in statement.input_digest {
                        npc.observe(plonky3::Base::from_repr(byte));
                        nvc.observe(plonky3::Base::from_repr(byte));
                    }
                    let start = Instant::now();
                    let proof = plonky3::prove(&statement, &a, &b, &c, method, &mut npc)?;
                    let prover_ms = start.elapsed().as_secs_f64() * 1000.0;
                    let start = Instant::now();
                    let claims = plonky3::verify(&statement, &proof, &mut nvc)?;
                    let verifier_ms = start.elapsed().as_secs_f64() * 1000.0;
                    if npc.sample_algebra_element::<plonky3::Extension>()
                        != nvc.sample_algebra_element::<plonky3::Extension>()
                    {
                        return Err("native Plonky3 transcript replay mismatch".into());
                    }
                    let start = Instant::now();
                    plonky3::check_claims(&statement, &a, &b, &c, &claims)?;
                    let check_ms = start.elapsed().as_secs_f64() * 1000.0;
                    let encoded = bincode::serialize(&proof).map_err(|e| e.to_string())?;
                    (
                        prover_ms,
                        verifier_ms,
                        check_ms,
                        encoded.len(),
                        blake3::hash(&encoded).to_hex().to_string(),
                    )
                } else if name == "flock" {
                    let start = Instant::now();
                    let (proof, claims) =
                        flock_core::zerocheck::prove_packed(&a, &b, &c, log_n, &mut pc);
                    let prover_ms = start.elapsed().as_secs_f64() * 1000.0;
                    let start = Instant::now();
                    let verified = flock_core::zerocheck::verify(log_n, &proof, &mut vc)
                        .map_err(|e| format!("Flock verifier: {e:?}"))?;
                    let verifier_ms = start.elapsed().as_secs_f64() * 1000.0;
                    if claims != verified {
                        return Err("Flock claims disagree".into());
                    }
                    let start = Instant::now();
                    flock_oracle::check_claims(&a, &b, &c, log_n, &verified)?;
                    let check_ms = start.elapsed().as_secs_f64() * 1000.0;
                    let encoded = bincode::serialize(&proof).map_err(|e| e.to_string())?;
                    (
                        prover_ms,
                        verifier_ms,
                        check_ms,
                        encoded.len(),
                        blake3::hash(&encoded).to_hex().to_string(),
                    )
                } else {
                    let method = method(name)?;
                    let start = Instant::now();
                    let (proof, claims) = prove(&statement, &a, &b, &c, method, &mut pc)?;
                    let prover_ms = start.elapsed().as_secs_f64() * 1000.0;
                    let start = Instant::now();
                    let verified = verify(&statement, method, &proof, &mut vc)?;
                    let verifier_ms = start.elapsed().as_secs_f64() * 1000.0;
                    if claims != verified {
                        return Err("baseline claims disagree".into());
                    }
                    let start = Instant::now();
                    check_claims(&statement, &a, &b, &c, &verified)?;
                    let check_ms = start.elapsed().as_secs_f64() * 1000.0;
                    let encoded = bincode::serialize(&proof).map_err(|e| e.to_string())?;
                    (
                        prover_ms,
                        verifier_ms,
                        check_ms,
                        encoded.len(),
                        blake3::hash(&encoded).to_hex().to_string(),
                    )
                };
            if pc.sample_f128() != vc.sample_f128() {
                return Err("transcript replay mismatch".into());
            }
            eprintln!(
                "run={run} {name}: prove {prover_ms:.3} ms, verify {verifier_ms:.3} ms, {bytes} bytes"
            );
            let sample = json!({"run":run,"order":position,"system":name,"input_digest":statement.input_digest,
                "prover_ms":prover_ms,"verifier_ms":verifier_ms,"input_claim_check_ms":check_ms,
                "bincode_bytes":bytes,"proof_digest":digest});
            if run == 0 {
                warmups.push(sample);
            } else {
                samples.push(sample);
            }
        }
    }
    let summaries: Vec<_> = systems.iter().map(|name| {
        let rows: Vec<_> = samples.iter().filter(|s| s["system"] == *name).collect();
        let stat = |key: &str| {
            let mut data: Vec<_> = rows.iter().map(|s| s[key].as_f64().unwrap()).collect();
            data.sort_by(f64::total_cmp);
            let median = (data[(data.len()-1)/2] + data[data.len()/2]) / 2.0;
            json!({"median":median,"min":data[0],"max":data[data.len()-1]})
        };
        json!({"system":name,"prover_ms":stat("prover_ms"),"verifier_ms":stat("verifier_ms"),
            "input_claim_check_ms":stat("input_claim_check_ms"),"bincode_bytes":rows[0]["bincode_bytes"]})
    }).collect();
    let mut source_hash = blake3::Hasher::new();
    for source in [
        include_str!("../src/lib.rs"),
        include_str!("../src/engine.rs"),
        include_str!("../src/dao_thaler.rs"),
        include_str!("../src/plonky3.rs"),
        include_str!("../support/flock_oracle.rs"),
        include_str!("hadamard.rs"),
        include_str!("../Cargo.toml"),
        include_str!("../Cargo.lock"),
        include_str!("../build.rs"),
        include_str!("../reference_sources.json"),
        include_str!("../../../mikros/mikros/src/baseline.rs"),
    ] {
        source_hash.update(&(source.len() as u64).to_le_bytes());
        source_hash.update(source.as_bytes());
    }
    let report = json!({"schema_version":2,"metadata":{
        "log_n":log_n,"runs":runs,"threads":threads,"actual_threads":rayon::current_num_threads(),"systems":systems,
        "unix_timestamp_seconds":std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).ok().map(|d|d.as_secs()),
        "rustc":option_env!("BASELINES_BUILD_RUSTC"),"sources_blake3":source_hash.finalize().to_hex().to_string(),
        "scope":"PIOP only; no PCS/grinding/equal-security claim; full original-input checks retained outside timings",
        "input":"identical packed uniform F2 A/B, C=A&B in each round",
        "field":"small_field_first: native Plonky3 BinaryField8/BinaryField128; Dao-Thaler and Flock: GHASH F128",
        "small_field_first_implementation":"direct path dependency: ../mikros/mikros/src/baseline.rs::prove_hadamard and verify_hadamard",
        "native_base_packing_width":<plonky3::Base as Field>::Packing::WIDTH,
        "transcript":"BLAKE3; native BinaryChallenger/HashChallenger for Plonky3, FsChallenger for GHASH methods; distinct encodings",
        "rustflags":option_env!("BASELINES_BUILD_RUSTFLAGS"),
        "cpu":fs::read_to_string("/proc/cpuinfo").ok().and_then(|s| s.lines().find(|l| l.starts_with("model name")).map(str::to_owned)),
        "compiled_features":{"avx2":cfg!(target_feature="avx2"),"avx512f":cfg!(target_feature="avx512f"),"gfni":cfg!(target_feature="gfni"),"pclmulqdq":cfg!(target_feature="pclmulqdq"),"vpclmulqdq":cfg!(target_feature="vpclmulqdq")},
        "setup":"one warmup per system; fixed tower map setup stays in first-use proof; witness-dependent work is online"
    },"warmups":warmups,"samples":samples,"summaries":summaries});
    let encoded = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?;
    if let Some(path) = output {
        let path = if path.is_absolute() {
            path
        } else {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path)
        };
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(path, encoded).map_err(|e| e.to_string())?;
    } else {
        println!("{encoded}");
    }
    Ok(())
}

fn method(name: &str) -> Result<Method, String> {
    match name {
        "ghash_small_field_first" => Ok(Method::GhashSmallFieldFirst),
        "ghash_full_extension" => Ok(Method::GhashFullExtension),
        "dao_thaler" => Ok(Method::DaoThaler),
        "dao_thaler_generic" => Ok(Method::DaoThalerGeneric),
        _ => Err(format!("unknown system {name}")),
    }
}

fn native_method(name: &str) -> Option<plonky3::Method> {
    match name {
        "small_field_first" => Some(plonky3::Method::SmallFieldFirst),
        "small_field_first_full_extension" => Some(plonky3::Method::FullExtension),
        _ => None,
    }
}
