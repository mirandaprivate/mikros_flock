//! Full Flock circuit proof of a SHA-256 binary tree, with honest timing labels.
//!
//! cargo run --release -p mikros --example e2e_flock_baseline --
//!     --log-leaves 3 --threads 4 --reps 3 --warmups 1 --profile fast
//!
//! The existing public prover owns assembly, compaction, commitment, all PIOPs,
//! and the PCS opening. Its total is reported together, not as pure online time.
//! No source-witness check substitutes for the complete proof verifier.

mod e2e_common;

use std::{
    env,
    error::Error,
    io::{self, Write},
    time::Instant,
};

use e2e_common::{DOMAIN, FIXTURE_SEED, TreeFixture};
use flock_core::{
    challenger::{Challenger, FsChallenger},
    pcs::ligerito::{LigeritoProfile, embedded_security_config},
    verifier::verify_ligerito_union_circuit,
};
use flock_prover::prover::{UnionSlotProverInput, prove_fast_ligerito_union_circuit};
use serde_json::json;

struct Options {
    log_leaves: usize,
    threads: usize,
    reps: usize,
    warmups: usize,
    profile: LigeritoProfile,
}

fn options() -> Result<Options, String> {
    let mut options = Options {
        log_leaves: 3,
        threads: 1,
        reps: 3,
        warmups: 1,
        profile: LigeritoProfile::Fast,
    };
    let mut args = env::args().skip(1);
    while let Some(flag) = args.next() {
        if flag == "--help" {
            println!(
                "--log-leaves N --threads N --reps N --warmups N --profile fast|fast100|slim|slim100|secure"
            );
            std::process::exit(0);
        }
        let value = args
            .next()
            .ok_or_else(|| format!("missing value for {flag}"))?;
        match flag.as_str() {
            "--log-leaves" => {
                options.log_leaves = value.parse().map_err(|_| "invalid --log-leaves")?
            }
            "--threads" => options.threads = value.parse().map_err(|_| "invalid --threads")?,
            "--reps" => options.reps = value.parse().map_err(|_| "invalid --reps")?,
            "--warmups" => options.warmups = value.parse().map_err(|_| "invalid --warmups")?,
            "--profile" => {
                options.profile = match value.as_str() {
                    "fast" => LigeritoProfile::Fast,
                    "fast100" => LigeritoProfile::Fast100,
                    "slim" => LigeritoProfile::Slim,
                    "slim100" => LigeritoProfile::Slim100,
                    "secure" => LigeritoProfile::Secure,
                    _ => return Err("invalid --profile".into()),
                }
            }
            _ => return Err(format!("unknown argument {flag}")),
        }
    }
    if options.threads == 0 || options.reps == 0 || options.log_leaves > 19 {
        return Err("threads/reps must be positive; log-leaves must be at most 19".into());
    }
    Ok(options)
}

fn main() -> Result<(), Box<dyn Error>> {
    let options = options().map_err(io::Error::other)?;
    if env::var_os("FLOCK_NO_GRIND").is_some() {
        return Err("FLOCK_NO_GRIND is incompatible with verified full proofs".into());
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(options.threads)
        .stack_size(8 << 20)
        .build_global()?;
    if !flock_parallel::init_all_core_pool(options.threads) {
        return Err(
            "all-core pool was already initialized; cannot ensure requested threads".into(),
        );
    }
    let t = Instant::now();
    let fixture = TreeFixture::new(options.log_leaves);
    let view = fixture.verifier();
    let union = view.union();
    let pcs = fixture.pcs_params(options.profile);
    let prover_config = pcs.ligerito_prover_config().map_err(io::Error::other)?;
    let _ = pcs.ligerito_verifier_config().map_err(io::Error::other)?;
    let offline_fixture_ms = t.elapsed().as_secs_f64() * 1e3;
    let t = Instant::now();
    let prepared = fixture.prepare_witness();
    let offline_witness_ms = t.elapsed().as_secs_f64() * 1e3;
    let prepared_bytes =
        (prepared.0.len() + prepared.1.len() + prepared.2.len()) * 16 + prepared.3.len();
    let fixture_digest = blake3::hash(&bincode::serialize(&(
        &fixture.compressions,
        &fixture.public,
    ))?)
    .to_hex()
    .to_string();
    let config_digest = blake3::hash(
        embedded_security_config(pcs.m, pcs.profile)
            .ok_or("missing registered PCS config")?
            .as_bytes(),
    )
    .to_hex()
    .to_string();
    let mut output = io::BufWriter::new(io::stdout().lock());
    writeln!(
        output,
        "{}",
        json!({
            "record": "metadata", "schema_version": 1, "system": "flock-circuit-e2e",
            "fixture": "sha256-binary-tree", "seed": FIXTURE_SEED, "fixture_digest_blake3": fixture_digest,
            "circuit_digest_blake3": hex(&fixture.circuit.digest()), "log_leaves": fixture.log_leaves,
            "leaves": 1usize << fixture.log_leaves, "gate_count": fixture.compressions.len(),
            "row_log": view.nu, "padded_m": union.m_total(), "dense_m": union.dense_m(),
            "cell_log": fixture.circuit.cells().mu(), "public_words": fixture.public.len(),
            "native_root_u32": fixture.root, "threads": rayon::current_num_threads(),
            "all_core_threads": flock_parallel::all_core_pool().current_num_threads(),
            "reps": options.reps, "warmups": options.warmups, "pcs_params": pcs,
            "pcs_config_blake3": config_digest, "pcs_queries": prover_config.queries,
            "pcs_query_grinding_bits": prover_config.grinding_bits,
            "pcs_fold_grinding_bits": prover_config.fold_grinding_bits,
            "offline_fixture_ms": offline_fixture_ms, "offline_witness_ms": offline_witness_ms,
            "offline_prepared_bytes": prepared_bytes,
            "timing_boundary": "prepare clone excluded; prove includes union assembly, compaction, commitment, Hadamard, lincheck, wiring, and merged PCS; no pure-online claim",
            "verification": "complete non-deferred verifier; no witness input; matrix and sigma obligations discharged",
            "proof_encoding": "bincode serialize((Commitment, R1csProofCircuitMerged)); full bundle including cap and PCS",
            "zero_knowledge_claim": false, "architecture": std::env::consts::ARCH,
            "target_features": features(),
        })
    )?;
    output.flush()?;
    let lc = view.r1cs.csc_lincheck_circuit();
    let mut first_digest = None;
    for iteration in 0..options.warmups + options.reps {
        let t = Instant::now();
        let witness = prepared.clone();
        let offline_clone_ms = t.elapsed().as_secs_f64() * 1e3;
        let slot = UnionSlotProverInput::new(witness, lc);
        let mut prover_ch = FsChallenger::new(DOMAIN);
        let t = Instant::now();
        let (proof, commitment, _) = prove_fast_ligerito_union_circuit(
            &union,
            view.circuit,
            view.public,
            &pcs,
            vec![slot],
            vec![],
            &mut prover_ch,
        );
        let prove_with_commit_ms = t.elapsed().as_secs_f64() * 1e3;
        let commitment_digest = blake3::hash(&bincode::serialize(&commitment)?)
            .to_hex()
            .to_string();
        let mut verifier_ch = FsChallenger::new(DOMAIN);
        let t = Instant::now();
        verify_ligerito_union_circuit(
            &union,
            view.circuit,
            view.public,
            &[lc],
            &commitment,
            &proof,
            &pcs,
            &mut verifier_ch,
        )
        .map_err(|e| io::Error::other(format!("complete verification failed: {e:?}")))?;
        let verify_ms = t.elapsed().as_secs_f64() * 1e3;
        if prover_ch.sample_f128() != verifier_ch.sample_f128() {
            return Err("prover/verifier transcript did not end at the same state".into());
        }
        let t = Instant::now();
        let encoded = bincode::serialize(&(&commitment, &proof))?;
        let serialize_ms = t.elapsed().as_secs_f64() * 1e3;
        let proof_digest = blake3::hash(&encoded).to_hex().to_string();
        if let Some(expected) = &first_digest {
            if expected != &proof_digest {
                return Err("same input produced different proof bytes".into());
            }
        } else {
            first_digest = Some(proof_digest.clone());
        }
        writeln!(
            output,
            "{}",
            json!({
                "record": "sample", "system": "flock-circuit-e2e", "iteration": iteration,
                "warmup": iteration < options.warmups, "offline_clone_ms": offline_clone_ms,
                "prove_with_commit_ms": prove_with_commit_ms, "verify_ms": verify_ms,
                "serialize_ms": serialize_ms, "proof_bytes": encoded.len(), "proof_blake3": proof_digest,
                "commitment_blake3": commitment_digest,
                "verified": true, "transcript_match": true,
            })
        )?;
        output.flush()?;
    }
    Ok(())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn features() -> Vec<&'static str> {
    let mut output = vec![];
    for (name, enabled) in [
        ("aes", cfg!(target_feature = "aes")),
        ("pclmulqdq", cfg!(target_feature = "pclmulqdq")),
        ("avx2", cfg!(target_feature = "avx2")),
        ("avx512f", cfg!(target_feature = "avx512f")),
        ("avx512bw", cfg!(target_feature = "avx512bw")),
        ("gfni", cfg!(target_feature = "gfni")),
        ("vpclmulqdq", cfg!(target_feature = "vpclmulqdq")),
    ] {
        if enabled {
            output.push(name);
        }
    }
    output
}
