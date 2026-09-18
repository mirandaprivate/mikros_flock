#![recursion_limit = "256"]
// SPDX-License-Identifier: MIT OR Apache-2.0
// Protocol wiring follows the original Binius EqInd sumcheck tests.
// Copyright 2024-2025 Irreducible Inc.
// Benchmark driver and independent source oracle: Mikros artifact contributors.

use binius_core::{
    fiat_shamir::{CanSample, HasherChallenger},
    protocols::sumcheck::{
        self, BatchSumcheckOutput, CompositeSumClaim, EqIndSumcheckClaim,
        eq_ind::{self, ClaimsSortingOrder},
        prove::eq_ind::EqIndSumcheckProverBuilder,
    },
    transcript::{ProverTranscript, VerifierTranscript},
};
use binius_field::{
    BinaryField1b, Field, PackedField,
    arch::OptimalUnderlier, as_packed_field::PackedType,
};
#[cfg(not(any(feature = "polyval", feature = "aes32", feature = "aes-bytesliced")))]
use binius_field::{BinaryField8b, BinaryField128b};
use binius_hal::make_portable_backend;
use binius_hash::groestl::Groestl256;
use binius_math::{
    ArithCircuit, CompositionPoly, EvaluationOrder, IsomorphicEvaluationDomainFactory,
    MLEEmbeddingAdapter, MultilinearExtension,
};
use rayon::prelude::*;
use serde_json::{Value, json};
use std::{env, fs, path::{Path, PathBuf}, time::{Instant, SystemTime, UNIX_EPOCH}};
mod engine;
mod profile;
use engine::{Composition, Options};
use profile::{NativeProfile, Profiled};

const REVISION: &str = "47675e19c86c0fb676f75437073a77af8e337938";
const SYSTEM: &str = "binius_switchover";
const DOMAIN_TAG: &[u8] = b"mikros-artifact/original-binius-eqind-hadamard/v1";
#[cfg(any(
    all(feature = "polyval", feature = "aes32"),
    all(feature = "polyval", feature = "aes-bytesliced"),
    all(feature = "aes32", feature = "aes-bytesliced"),
))]
compile_error!("select only one native field feature: polyval, aes32, or aes-bytesliced");

#[cfg(not(any(feature = "polyval", feature = "aes32", feature = "aes-bytesliced")))]
type F = BinaryField128b;
#[cfg(feature = "polyval")]
type F = binius_field::BinaryField128bPolyval;
#[cfg(any(feature = "aes32", feature = "aes-bytesliced"))]
type F = binius_field::AESTowerField128b;

#[cfg(not(any(feature = "polyval", feature = "aes32", feature = "aes-bytesliced")))]
type FDomain = BinaryField8b;
#[cfg(feature = "polyval")]
type FDomain = binius_field::BinaryField128bPolyval;
#[cfg(any(feature = "aes32", feature = "aes-bytesliced"))]
type FDomain = binius_field::AESTowerField8b;

// Fixture storage always remains ordinary compact bits in canonical byte order.
type P1 = PackedType<OptimalUnderlier, BinaryField1b>;
#[cfg(not(any(feature = "aes32", feature = "aes-bytesliced")))]
type P128 = PackedType<OptimalUnderlier, F>;
#[cfg(feature = "aes32")]
type P128 = binius_field::ByteSlicedAES32x128b;
#[cfg(feature = "aes-bytesliced")]
type P128 = PackedType<binius_field::arch::OptimalUnderlierByteSliced, F>;
type PBase = binius_field::PackedSubfield<P128, BinaryField1b>;
#[cfg(feature = "aes32")]
type POrdinal = PackedType<binius_field::arch::OptimalUnderlier256b, BinaryField1b>;
#[cfg(feature = "aes-bytesliced")]
type POrdinal = P1;
type EmbeddedInput = MLEEmbeddingAdapter<PBase, P128>;

const FIELD_VARIANT: &str = if cfg!(feature = "polyval") { "polyval" }
    else if cfg!(feature = "aes32") { "aes32" }
    else if cfg!(feature = "aes-bytesliced") { "aes-bytesliced" }
    else { "canonical" };
const BYTE_SLICED: bool = cfg!(any(feature = "aes32", feature = "aes-bytesliced"));
type Inputs = [Vec<P1>; 3];
type ProverTape = ProverTranscript<HasherChallenger<Groestl256>>;
type VerifierTape = VerifierTranscript<HasherChallenger<Groestl256>>;
type Claim = BatchSumcheckOutput<F>;

/// Exactly the binary Hadamard relation. In characteristic two, minus is plus.
#[derive(Debug, Clone, Copy)]
struct Hadamard;
impl<P: PackedField> CompositionPoly<P> for Hadamard {
    fn n_vars(&self) -> usize { 3 }
    fn degree(&self) -> usize { 2 }
    fn binary_tower_level(&self) -> usize { 0 }
    fn expression(&self) -> ArithCircuit<P::Scalar> {
        ArithCircuit::var(0) * ArithCircuit::var(1) + ArithCircuit::var(2)
    }
    fn evaluate(&self, query: &[P]) -> Result<P, binius_math::Error> {
        Ok(query[0] * query[1] + query[2])
    }
}

struct Config {
    log_n: usize, switchover: usize, runs: usize, threads: usize,
    output: Option<PathBuf>, rss: bool, tamper: bool, engine: Options,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Original Binius Hadamard benchmark failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let Some(config) = parse_args()? else { return Ok(()); };
    if cfg!(target_endian = "big") { return Err("fixture requires little-endian target".into()); }
    if env::var_os("BINIUS_DUMP_PROOF").is_some() {
        return Err("unset BINIUS_DUMP_PROOF for this experiment".into());
    }
    rayon::ThreadPoolBuilder::new().num_threads(config.threads).stack_size(8 << 20)
        .build_global().map_err(|e| e.to_string())?;
    let mut report = json!({
        "schema_version": 1,
        "metadata": {
            "system": SYSTEM,
            "upstream_repository": "https://github.com/IrreducibleOSS/binius",
            "upstream_revision": REVISION,
            "upstream_modified": false,
            "upstream_prover_modified": false,
            "protocol": "original Binius EqInd sumcheck with native transparent-to-folded switchover",
            "algorithm_label": "Binius small-field switchover",
            "sff_tensor_grid": false,
            "scope": "Complete binary Hadamard PIOP; no PCS. All three source claims independently checked outside timings.",
            "log_n": config.log_n,
            "n_binary_constraints": 1u64 << config.log_n,
            "switchover": config.switchover,
            "switchover_materialization_fold": config.switchover + 1,
            "materialization_after_round": config.switchover + 1,
            "switchover_semantics": "Native counter s materializes after challenge fold s+1; no rounds skipped. This engineering parameter is not part of the transcript statement.",
            "sumcheck_rounds": config.log_n,
            "skip_rounds": 0,
            "evaluation_order": format!("{:?}", config.engine.order),
            "field_backend": FIELD_VARIANT,
            "composition_backend": if config.engine.native_composition { "native" } else { "scalar" },
            "first_round_zero": config.engine.first_round_zero,
            "profile": config.engine.profile,
            "base_field_bits": 1,
            "challenge_field_bits": 128,
            "challenge_field": std::any::type_name::<F>(),
            "field_variant": FIELD_VARIANT,
            "cargo_field_feature": if FIELD_VARIANT == "canonical" { "default" } else { FIELD_VARIANT },
            "packed_field": std::any::type_name::<P128>(),
            "packed_extension_lanes": P128::WIDTH,
            "packed_extension_bytes": std::mem::size_of::<P128>(),
            "packed_prover_base": std::any::type_name::<PBase>(),
            "packed_input_storage": std::any::type_name::<P1>(),
            "domain_field": std::any::type_name::<FDomain>(),
            "transcript_field_encoding": "CanonicalTower (native Binius serialization and challenge sampling)",
            "packed_underlier_bits": std::mem::size_of::<P1>() * 8,
            "input_bytes": 3u64 << (config.log_n - 3),
            "grinding": "none in native generic sumcheck",
            "pcs": false,
            "source_check_reuse": false,
            "rss": config.rss,
            "tamper": config.tamper,
            "threads": config.threads,
            "rayon_actual_threads": rayon::current_num_threads(),
            "cpu": cpu_name(), "os": env::consts::OS, "arch": env::consts::ARCH,
            "compiled_target_features": compiled_features(),
            "harness_sources_blake3": harness_digest(),
            "rustflags": option_env!("MIKROS_BUILD_RUSTFLAGS"),
            "rustc": option_env!("MIKROS_BUILD_RUSTC"),
            "runs": config.runs,
            "mode": if config.rss { "fresh_process_rss_cold_proof" } else { "warm_timing" },
            "timing": "Prover includes F128 equality challenges, F2 MLE adapters and any native byte-sliced input transpose, native builder and equality-table expansion, and every batch_prove round. Verifier includes equality challenges, batch_verify, and exact terminal equality check. Input generation/digest, transcript statement initialization, tape finalization/replay comparison and independent source check are separate.",
            "memory": "VmHWM immediately after proof, before finalization, verifier or diagnostic input regeneration. Native equality tables and challenge-dependent witness materialization are included.",
            "ownership": "Compact ordinal F2 inputs move into native MLEs; byte-sliced builds instead transpose these compact bits online with the original native transpose_from. No N-element F128 pre-expansion. Independent source inputs regenerate after primary VmHWM capture.",
            "input_adaptation": if BYTE_SLICED { "native compact F2 transpose_from, allocation and MLE adapter construction; timed online" } else { "zero-copy native compact F2 MLE adapter construction; timed online" },
            "source_oracle": "Direct byte subset-sum and split equality tensor; independently evaluates all original A/B/C bits at the returned MLE point. No native fold or prover state reused.",
            "input_generator": "SplitMix64; seed=0x6D696B726F730001 ^ (log_n<<32) ^ run; all A then all B; C=A&B; canonical little-endian bytes",
            "unix_timestamp_seconds": SystemTime::now().duration_since(UNIX_EPOCH).map_err(|e| e.to_string())?.as_secs(),
        },
        "tamper_checks": [], "warmups": [], "samples": [], "summaries": []
    });
    if config.tamper {
        report["tamper_checks"] = Value::Array(tamper_checks_with(config.switchover, config.engine)?);
        write_if_requested(config.output.as_deref(), &report)?;
    }
    for index in 0..config.runs + usize::from(!config.rss) {
        let warmup = !config.rss && index == 0;
        let (stage, run, total) = if warmup { ("warmup", 1, 1) }
            else if config.rss { ("RSS", 1, config.runs) }
            else { ("measured", index, config.runs) };
        eprintln!("[hadamard] {SYSTEM} {stage} run {run}/{total} start (log_n={}, switchover={})",
            config.log_n, config.switchover);
        let item = sample_with(config.log_n, index, config.rss, config.switchover, config.engine)?;
        eprintln!("[hadamard] {SYSTEM} {stage} run {run}/{total} finished: prover={:.3} ms verifier={:.3} ms proof={} bytes (log_n={}, switchover={})",
            item["prover_ms"].as_f64().unwrap(), item["verifier_ms"].as_f64().unwrap(),
            item["proof_bytes"], config.log_n, config.switchover);
        report[if warmup { "warmups" } else { "samples" }].as_array_mut().unwrap().push(item);
        write_if_requested(config.output.as_deref(), &report)?;
    }
    let samples = report["samples"].as_array().unwrap();
    report["summaries"] = json!([{
        "system": SYSTEM, "log_n": config.log_n, "switchover": config.switchover,
        "n": samples.len(), "prover_median_ms": median(samples, "prover_ms"),
        "verifier_median_ms": median(samples, "verifier_ms"),
        "source_check_median_ms": median(samples, "input_claim_check_ms"),
    }]);
    write_if_requested(config.output.as_deref(), &report)?;
    if config.output.is_none() {
        println!("{}", serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?);
    }
    Ok(())
}

fn parse_args() -> Result<Option<Config>, String> {
    let mut config = Config { log_n: 30, switchover: 6, runs: 6, threads: 64,
        output: None, rss: false, tamper: false, engine: Options::default() };
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--rss" => config.rss = true,
            "--tamper" => config.tamper = true,
            "--profile" => config.engine.profile = true,
            "--help" | "-h" => {
                println!("binius-switchover-hadamard --log-n 30 --switchover 6 --runs 6 --threads 64 --output FILE [--rss] [--tamper] [--order low|high] [--composition native|scalar] [--first-round-zero yes|no] [--profile]");
                return Ok(None);
            }
            "--log-n" | "--switchover" | "--runs" | "--threads" | "--output" | "--order" | "--composition" | "--first-round-zero" => {
                let value = args.next().ok_or_else(|| format!("missing value for {arg}"))?;
                match arg.as_str() {
                    "--log-n" => config.log_n = value.parse::<usize>().map_err(|e| e.to_string())?,
                    "--switchover" => config.switchover = value.parse::<usize>().map_err(|e| e.to_string())?,
                    "--runs" => config.runs = value.parse::<usize>().map_err(|e| e.to_string())?,
                    "--threads" => config.threads = value.parse::<usize>().map_err(|e| e.to_string())?,
                    "--output" => config.output = Some(PathBuf::from(value)),
                    "--order" => config.engine.order = match value.as_str() {
                        "low" => EvaluationOrder::LowToHigh, "high" => EvaluationOrder::HighToLow,
                        _ => return Err("--order requires low or high".into()),
                    },
                    "--composition" => config.engine.native_composition = match value.as_str() {
                        "native" => true, "scalar" => false,
                        _ => return Err("--composition requires native or scalar".into()),
                    },
                    "--first-round-zero" => config.engine.first_round_zero = match value.as_str() {
                        "yes" => true, "no" => false,
                        _ => return Err("--first-round-zero requires yes or no".into()),
                    },
                    _ => unreachable!(),
                }
            }
            _ => return Err(format!("unknown option {arg}")),
        }
    }
    if !(10..=30).contains(&config.log_n) || config.switchover >= config.log_n
        || config.runs == 0 || config.threads == 0 {
        return Err("require 10 <= log-n <= 30, switchover < log-n, and positive runs/threads".into());
    }
    if config.rss && (config.runs != 1 || config.tamper) {
        return Err("cold --rss requires --runs 1 and cannot use --tamper".into());
    }
    if BYTE_SLICED && matches!(config.engine.order, EvaluationOrder::LowToHigh) {
        return Err("the pinned upstream byte-sliced F2 byte iterator is incompatible with low-to-high materialization; use --order high".into());
    }
    Ok(Some(config))
}

fn prover_tape(digest: &[u8; 32]) -> ProverTape {
    let mut tape = ProverTape::new();
    tape.set_debug(false);
    tape.observe().write_bytes(DOMAIN_TAG);
    tape.observe().write_bytes(digest);
    tape
}
fn verifier_tape(digest: &[u8; 32], bytes: Vec<u8>) -> VerifierTape {
    let mut tape = VerifierTape::new(bytes);
    tape.set_debug(false);
    tape.observe().write_bytes(DOMAIN_TAG);
    tape.observe().write_bytes(digest);
    tape
}
fn claims(log_n: usize) -> Result<Vec<EqIndSumcheckClaim<F, Hadamard>>, String> {
    Ok(vec![EqIndSumcheckClaim::new(log_n, 3,
        vec![CompositeSumClaim { sum: F::ZERO, composition: Hadamard }])
        .map_err(|e| format!("claim: {e:?}"))?])
}
fn bind_eq(log_n: usize, weights: &[F], output: Claim) -> Result<Claim, String> {
    let output = eq_ind::verify_sumcheck_outputs(ClaimsSortingOrder::DescendingVars,
        &claims(log_n)?, weights, output).map_err(|e| format!("terminal equality check: {e:?}"))?;
    if output.challenges.len() != log_n || output.multilinear_evals.len() != 1
        || output.multilinear_evals[0].len() != 3 {
        return Err("unexpected terminal claim shape".into());
    }
    Ok(output)
}
#[cfg(test)]
fn prove(log_n: usize, switchover: usize, inputs: Inputs, tape: &mut ProverTape) -> Result<Claim, String> {
    prove_with(log_n, switchover, inputs, tape, Options::default(), &mut NativeProfile::default())
}

fn embed_input(log_n: usize, input: Vec<P1>) -> Result<EmbeddedInput, String> {
    #[cfg(not(any(feature = "aes32", feature = "aes-bytesliced")))]
    let values = input;
    #[cfg(any(feature = "aes32", feature = "aes-bytesliced"))]
    let values: Vec<PBase> = {
        // These representations have different byte order. Use the upstream
        // transpose, including its allocation in the online prover timer.
        let ordinary: &[POrdinal] = bytemuck::cast_slice(&input);
        ordinary.par_chunks(16).map(|chunk| {
            let mut block = [POrdinal::default(); 16];
            block[..chunk.len()].copy_from_slice(chunk);
            PBase::transpose_from(&block)
        }).collect()
    };
    // Small tests may occupy a fraction of one byte-sliced packed value.
    MultilinearExtension::new(log_n, values)
        .map(MLEEmbeddingAdapter::<PBase, P128, _>::from)
        .map_err(|e| format!("F2 input adapter: {e:?}"))
}

fn prove_with(log_n: usize, switchover: usize, inputs: Inputs,
    tape: &mut ProverTape, options: Options, timings: &mut NativeProfile) -> Result<Claim, String> {
    if BYTE_SLICED && matches!(options.order, EvaluationOrder::LowToHigh) {
        return Err("unsupported upstream byte-sliced F2 low-to-high materialization".into());
    }
    let weights: Vec<F> = tape.sample_vec(log_n);
    let adaptation_start = Instant::now();
    let multilinears = inputs.into_iter().map(|input| embed_input(log_n, input))
        .collect::<Result<Vec<_>, _>>()?;
    timings.input_adaptation_ms = elapsed_ms(adaptation_start);
    let start = Instant::now();
    let backend = make_portable_backend();
    let domain = IsomorphicEvaluationDomainFactory::<FDomain>::default();
    let mut builder = EqIndSumcheckProverBuilder::with_switchover(multilinears, |_| switchover, &backend)
        .map_err(|e| format!("switchover builder: {e:?}"))?;
    // The Hadamard relation vanishes at every Boolean point, including the
    // half-cube with the first folded coordinate fixed to one.
    if options.first_round_zero { builder = builder.with_first_round_eval_1s(&[F::ZERO]); }
    let prover = builder.build::<FDomain, _>(options.order, &weights,
            [CompositeSumClaim { sum: F::ZERO, composition: Composition::new(options.native_composition) }], domain)
        .map_err(|e| format!("eq-ind builder: {e:?}"))?;
    timings.builder_ms = elapsed_ms(start);
    let start = Instant::now();
    let output = if options.profile {
        sumcheck::prove::batch_prove(vec![Profiled { inner: prover, timings }], tape)
    } else { sumcheck::prove::batch_prove(vec![prover], tape) }
    .map_err(|e| format!("native batch_prove: {e:?}"))?;
    timings.sumcheck_ms = elapsed_ms(start);
    bind_eq(log_n, &weights, output)
}
#[cfg(test)]
fn verify(log_n: usize, tape: &mut VerifierTape) -> Result<Claim, String> {
    verify_with(log_n, tape, Options::default())
}
fn verify_with(log_n: usize, tape: &mut VerifierTape, options: Options) -> Result<Claim, String> {
    let weights: Vec<F> = tape.sample_vec(log_n);
    let claims = claims(log_n)?;
    let regular = eq_ind::reduce_to_regular_sumchecks(&claims)
        .map_err(|e| format!("eq-ind verifier reduction: {e:?}"))?;
    let output = sumcheck::batch_verify(options.order, &regular, tape)
        .map_err(|e| format!("native batch_verify: {e:?}"))?;
    bind_eq(log_n, &weights, output)
}

#[cfg(test)]
fn sample(log_n: usize, run: usize, rss: bool, switchover: usize) -> Result<Value, String> {
    sample_with(log_n, run, rss, switchover, Options::default())
}
fn sample_with(log_n: usize, run: usize, rss: bool, switchover: usize, options: Options) -> Result<Value, String> {
    eprintln!("[hadamard] {SYSTEM}: input preparation started");
    let start = Instant::now();
    let (inputs, digest) = witness(log_n, run);
    let input_generation_ms = elapsed_ms(start);
    eprintln!("[hadamard] {SYSTEM}: input preparation finished");
    let mut prover = prover_tape(&digest);
    let before = rss.then(|| status_kib("VmRSS:")).flatten();
    let mut timings = NativeProfile::default();
    eprintln!("[hadamard] {SYSTEM}: proving started");
    let start = Instant::now();
    let claim = prove_with(log_n, switchover, inputs, &mut prover, options, &mut timings)?;
    let prover_ms = elapsed_ms(start);
    let peak = rss.then(|| status_kib("VmHWM:")).flatten();
    eprintln!("[hadamard] {SYSTEM}: proving finished ({prover_ms:.3} ms)");
    let next_prover: F = prover.sample();
    let start = Instant::now();
    let proof = prover.finalize();
    let proof_finalize_ms = elapsed_ms(start);
    let fields = 3 * log_n + 4;
    if proof.len() != fields * 16 { return Err(format!("native proof shape changed: {} != {}", proof.len(), fields * 16)); }
    eprintln!("[hadamard] {SYSTEM}: verification started");
    let start = Instant::now();
    let mut verifier = verifier_tape(&digest, proof.clone());
    let verifier_prepare_ms = elapsed_ms(start);
    let start = Instant::now();
    let checked = verify_with(log_n, &mut verifier, options)?;
    let verifier_ms = elapsed_ms(start);
    let next_verifier: F = verifier.sample();
    verifier.finalize().map_err(|e| format!("unread proof bytes: {e:?}"))?;
    if next_prover != next_verifier || claim.challenges != checked.challenges
        || claim.multilinear_evals != checked.multilinear_evals {
        return Err("Fiat-Shamir state or terminal claims mismatch".into());
    }
    eprintln!("[hadamard] {SYSTEM}: verification finished ({verifier_ms:.3} ms)");
    eprintln!("[hadamard] {SYSTEM}: independent source checks started (including input regeneration)");
    let start = Instant::now();
    let (source, source_digest) = witness(log_n, run);
    let input_regeneration_ms = elapsed_ms(start);
    if source_digest != digest { return Err("regenerated source digest mismatch".into()); }
    let start = Instant::now();
    check_source(&source, &checked)?;
    let input_claim_check_ms = elapsed_ms(start);
    eprintln!("[hadamard] {SYSTEM}: independent source checks finished");
    Ok(json!({
        "system": SYSTEM, "log_n": log_n, "run": run, "switchover": switchover,
        "field_variant": FIELD_VARIANT,
        "input_digest": hex(&digest), "prover_ms": prover_ms, "verifier_ms": verifier_ms,
        "input_claim_check_ms": input_claim_check_ms,
        "input_generation_ms": input_generation_ms, "input_regeneration_ms": input_regeneration_ms,
        "owned_input_copy_ms": 0.0, "input_conversion_ms": timings.input_adaptation_ms,
        "native_profile": if options.profile { timings.report() } else { Value::Null },
        "proof_finalize_ms": proof_finalize_ms, "verifier_prepare_ms": verifier_prepare_ms,
        "payload_field_elements": fields, "payload_field_bytes": fields * 16,
        "proof_bytes": proof.len(), "proof_digest": blake3::hash(&proof).to_hex().to_string(),
        "vm_rss_before_prove_kib": before, "vm_hwm_after_prove_kib": peak,
        "source_check_method": "independent_byte_lookup_and_split_eq_direct_oracle",
        "source_check_anchor": SYSTEM,
        "verifier_passed": true, "source_claims_verified": true, "transcript_matched": true,
        "verified": true, "source_checked": true, "transcript_match": true,
        "sumcheck_rounds": checked.challenges.len(), "next_challenge": format!("{next_verifier:?}"),
    }))
}

/// Independent source oracle: coordinates are least-significant-bit first.
/// The lowest 3 coordinates index bits of a byte; the rest index the bytes.
/// Only this diagnostic path uses subset tables; the measured prover is upstream.
fn source_values(inputs: &Inputs, point: &[F]) -> Result<[F; 3], String> {
    if point.len() < 3 || inputs.iter().any(|v| v.len() * P1::WIDTH != 1usize << point.len()) {
        return Err("source oracle input shape mismatch".into());
    }
    let columns: [&[u8]; 3] = std::array::from_fn(|i| bytemuck::cast_slice(&inputs[i]));
    let bit_weights = eq_weights(&point[..3]);
    let mut table = [F::ZERO; 256];
    for mask in 1usize..256 {
        table[mask] = table[mask & (mask - 1)] + bit_weights[mask.trailing_zeros() as usize];
    }
    let split = 3 + (point.len() - 3) / 2;
    let low = eq_weights(&point[3..split]);
    let high = eq_weights(&point[split..]);
    Ok(high.par_iter().enumerate().map(|(block, &high_weight)| {
        let mut sum = [F::ZERO; 3];
        let start = block * low.len();
        for (offset, &weight) in low.iter().enumerate() {
            for column in 0..3 { sum[column] += table[columns[column][start + offset] as usize] * weight; }
        }
        sum.map(|value| value * high_weight)
    }).reduce(|| [F::ZERO; 3], |a, b| std::array::from_fn(|i| a[i] + b[i])))
}
fn check_source(inputs: &Inputs, claim: &Claim) -> Result<(), String> {
    if claim.multilinear_evals.len() != 1 || claim.multilinear_evals[0].len() != 3 {
        return Err("source check requires exactly A/B/C claims".into());
    }
    if source_values(inputs, &claim.challenges)?.as_slice() != claim.multilinear_evals[0].as_slice() {
        return Err("independent A/B/C source oracle mismatch".into());
    }
    Ok(())
}
fn eq_weights(point: &[F]) -> Vec<F> {
    let mut weights = vec![F::ONE];
    for &r in point {
        let len = weights.len();
        weights.resize(2 * len, F::ZERO);
        for i in 0..len {
            let old = weights[i];
            weights[i] = old * (F::ONE - r);
            weights[i + len] = old * r;
        }
    }
    weights
}
struct SplitMix64(u64);
impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}
fn witness(log_n: usize, run: usize) -> (Inputs, [u8; 32]) {
    let mut rng = SplitMix64(0x6D69_6B72_6F73_0001 ^ ((log_n as u64) << 32) ^ run as u64);
    let len = 1usize << (log_n - P1::LOG_WIDTH);
    let mut inputs: Inputs = std::array::from_fn(|_| vec![P1::default(); len]);
    for column in &mut inputs[..2] {
        for chunk in bytemuck::cast_slice_mut::<P1, u8>(column).chunks_exact_mut(8) {
            chunk.copy_from_slice(&rng.next().to_le_bytes());
        }
    }
    let [a, b, c] = &mut inputs;
    for ((a, b), c) in bytemuck::cast_slice::<P1, u8>(a).iter()
        .zip(bytemuck::cast_slice::<P1, u8>(b))
        .zip(bytemuck::cast_slice_mut::<P1, u8>(c)) { *c = a & b; }
    let digest = input_digest(log_n, &inputs);
    (inputs, digest)
}
fn input_digest(log_n: usize, inputs: &Inputs) -> [u8; 32] {
    let mut hash = blake3::Hasher::new();
    hash.update(b"mikros-flock-input-v1");
    hash.update(&(log_n as u64).to_le_bytes());
    for input in inputs {
        let bytes = bytemuck::cast_slice::<P1, u8>(input);
        hash.update(&(bytes.len() as u64).to_le_bytes());
        hash.update(bytes);
    }
    *hash.finalize().as_bytes()
}

#[cfg(test)]
fn tamper_checks(switchover: usize) -> Result<Vec<Value>, String> {
    tamper_checks_with(switchover, Options::default())
}
fn tamper_checks_with(switchover: usize, options: Options) -> Result<Vec<Value>, String> {
    let log_n = 12.max(switchover + 1);
    if log_n > 16 { return Err("--tamper requires switchover <= 15".into()); }
    let (inputs, digest) = witness(log_n, 7);
    let mut tape = prover_tape(&digest);
    let honest = prove_with(log_n, switchover, inputs.clone(), &mut tape, options, &mut NativeProfile::default())?;
    let proof = tape.finalize();
    check_source(&inputs, &honest)?;
    let accept = |bytes: Vec<u8>, digest: &[u8; 32]| -> bool {
        let mut tape = verifier_tape(digest, bytes);
        let Ok(claim) = verify_with(log_n, &mut tape, options) else { return false; };
        tape.finalize().is_ok() && check_source(&inputs, &claim).is_ok()
    };
    if !accept(proof.clone(), &digest) { return Err("tamper suite honest control rejected".into()); }
    let mut cases = Vec::new();
    for (name, offset) in [
        ("first_round_message", 0), ("last_round_message", proof.len() - 4 * 16 - 1),
        ("terminal_a", proof.len() - 4 * 16), ("terminal_b", proof.len() - 3 * 16),
        ("terminal_c", proof.len() - 2 * 16), ("terminal_eq", proof.len() - 16),
    ] {
        let mut changed = proof.clone(); changed[offset] ^= 1;
        cases.push((name, !accept(changed, &digest)));
    }
    cases.push(("truncated", !accept(proof[..proof.len() - 1].to_vec(), &digest)));
    let mut trailing = proof.clone(); trailing.push(0);
    cases.push(("trailing_bytes", !accept(trailing, &digest)));
    let mut wrong_digest = digest; wrong_digest[0] ^= 1;
    cases.push(("wrong_input_digest", !accept(proof, &wrong_digest)));
    let mut wrong_c = inputs.clone();
    bytemuck::cast_slice_mut::<P1, u8>(&mut wrong_c[2])[0] ^= 1;
    cases.push(("false_c_source", check_source(&wrong_c, &honest).is_err()));
    for (name, rejected) in &cases {
        if !rejected { return Err(format!("tamper case accepted: {name}")); }
    }
    Ok(cases.into_iter().map(|(name, rejected)| json!({"name": name, "rejected": rejected})).collect())
}

fn elapsed_ms(start: Instant) -> f64 { start.elapsed().as_secs_f64() * 1000.0 }
fn hex(bytes: &[u8]) -> String { bytes.iter().map(|b| format!("{b:02x}")).collect() }
fn status_kib(key: &str) -> Option<u64> {
    fs::read_to_string("/proc/self/status").ok()?.lines().find_map(|line| {
        line.strip_prefix(key)?.split_whitespace().next()?.parse().ok()
    })
}
fn cpu_name() -> Option<String> {
    fs::read_to_string("/proc/cpuinfo").ok()?.lines().find_map(|line| {
        line.strip_prefix("model name").and_then(|line| line.split_once(':')).map(|(_, name)| name.trim().to_owned())
    })
}
fn compiled_features() -> Vec<&'static str> {
    [ ("gfni", cfg!(target_feature = "gfni")), ("avx512f", cfg!(target_feature = "avx512f")),
        ("avx512bw", cfg!(target_feature = "avx512bw")), ("avx512vbmi", cfg!(target_feature = "avx512vbmi")),
        ("vpclmulqdq", cfg!(target_feature = "vpclmulqdq")), ("pclmulqdq", cfg!(target_feature = "pclmulqdq")),
        ("avx2", cfg!(target_feature = "avx2")), ("sse4.1", cfg!(target_feature = "sse4.1")),
        ("neon", cfg!(target_feature = "neon")),
    ].into_iter().filter_map(|(name, on)| on.then_some(name)).collect()
}
fn harness_digest() -> String {
    let mut hash = blake3::Hasher::new();
    hash.update(include_bytes!("../Cargo.toml")); hash.update(include_bytes!("main.rs"));
    hash.update(include_bytes!("../build.rs"));
    hash.update(include_bytes!("engine.rs")); hash.update(include_bytes!("profile.rs"));
    hash.finalize().to_hex().to_string()
}
fn median(samples: &[Value], key: &str) -> f64 {
    let mut values: Vec<f64> = samples.iter().map(|sample| sample[key].as_f64().unwrap()).collect();
    values.sort_by(f64::total_cmp);
    let mid = values.len() / 2;
    if values.len() % 2 == 0 { (values[mid - 1] + values[mid]) / 2.0 } else { values[mid] }
}
fn write_if_requested(path: Option<&Path>, report: &Value) -> Result<(), String> {
    if let Some(path) = path {
        if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        fs::write(path, serde_json::to_string_pretty(report).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn native_options_preserve_messages() {
        let (inputs, digest) = witness(12, 3);
        for order in [EvaluationOrder::LowToHigh, EvaluationOrder::HighToLow] {
            // Pinned upstream ByteSliced F2 byte iteration overstates BYTES by
            // eight; low-order materialization invokes that unsafe iterator.
            // High-order folding uses its native generic path instead.
            if BYTE_SLICED && matches!(order, EvaluationOrder::LowToHigh) {
                let mut tape = prover_tape(&digest);
                assert!(prove_with(12, 6, inputs.clone(), &mut tape,
                    Options { order, ..Options::default() }, &mut NativeProfile::default()).is_err());
                continue;
            }
            let mut reference = None;
            for (native_composition, first_round_zero, profile) in [
                (false, false, false), (true, false, false),
                (true, true, false), (true, true, true),
            ] {
                let options = Options { order, native_composition, first_round_zero, profile };
                let mut tape = prover_tape(&digest);
                let claim = prove_with(12, 6, inputs.clone(), &mut tape, options, &mut NativeProfile::default()).unwrap();
                check_source(&inputs, &claim).unwrap();
                let proof = tape.finalize();
                let mut verifier = verifier_tape(&digest, proof.clone());
                let checked = verify_with(12, &mut verifier, options).unwrap();
                verifier.finalize().unwrap();
                check_source(&inputs, &checked).unwrap();
                assert_eq!(claim.challenges, checked.challenges);
                assert_eq!(claim.multilinear_evals, checked.multilinear_evals);
                if let Some(expected) = &reference { assert_eq!(&proof, expected); } else { reference = Some(proof); }
            }
            let options = Options { order, ..Options::default() };
            let mut false_inputs = inputs.clone();
            bytemuck::cast_slice_mut::<P1, u8>(&mut false_inputs[2])[0] ^= 1;
            let false_digest = input_digest(12, &false_inputs);
            let mut tape = prover_tape(&false_digest);
            if prove_with(12, 6, false_inputs.clone(), &mut tape, options, &mut NativeProfile::default()).is_ok() {
                let mut verifier = verifier_tape(&false_digest, tape.finalize());
                if let Ok(claim) = verify_with(12, &mut verifier, options) {
                    assert!(verifier.finalize().is_err() || check_source(&false_inputs, &claim).is_err());
                }
            }
        }
    }
    #[test]
    fn honest_native_proofs_and_complete_source_checks() {
        for log_n in [10, 12, 16] {
            let result = sample(log_n, 1, false, 6).unwrap();
            assert_eq!(result["verifier_passed"], true);
            assert_eq!(result["source_claims_verified"], true);
            assert_eq!(result["transcript_matched"], true);
            assert_eq!(result["proof_bytes"], (3 * log_n + 4) * 16);
        }
    }
    #[test]
    fn altered_messages_truncation_and_false_source_are_rejected() {
        let cases = tamper_checks(6).unwrap();
        assert_eq!(cases.len(), 10);
        assert!(cases.iter().all(|case| case["rejected"] == true));
    }
    #[test]
    fn switchover_changes_engineering_not_proof_messages() {
        let (inputs, digest) = witness(12, 3);
        let mut reference = None;
        for switchover in [0, 2, 4, 6] {
            let mut tape = prover_tape(&digest);
            let claim = prove(12, switchover, inputs.clone(), &mut tape).unwrap();
            check_source(&inputs, &claim).unwrap();
            let proof = tape.finalize();
            if let Some(expected) = &reference { assert_eq!(&proof, expected); } else { reference = Some(proof); }
        }
    }
    #[test]
    fn unsatisfied_hadamard_witness_cannot_verify_against_its_sources() {
        let (mut inputs, _) = witness(12, 21);
        bytemuck::cast_slice_mut::<P1, u8>(&mut inputs[2])[0] ^= 1;
        let digest = input_digest(12, &inputs);
        let mut prover = prover_tape(&digest);
        // An upstream prover error is a rejection; otherwise verify the actual
        // produced transcript and its terminal claims against the false witness.
        if prove(12, 6, inputs.clone(), &mut prover).is_err() { return; }
        let mut verifier = verifier_tape(&digest, prover.finalize());
        if let Ok(claim) = verify(12, &mut verifier) {
            assert!(verifier.finalize().is_err() || check_source(&inputs, &claim).is_err());
        }
    }
    #[test]
    fn split_eq_and_byte_lookup_match_dense_bit_oracle() {
        let (inputs, _) = witness(10, 11);
        let point: Vec<F> = (0..10).map(|i| F::new(0x1234567812345678u128 + i)).collect();
        let actual = source_values(&inputs, &point).unwrap();
        let mut expected = [F::ZERO; 3];
        for index in 0..1usize << point.len() {
            let weight: F = point.iter().enumerate().map(|(bit, &r)| {
                if index & (1 << bit) == 0 { F::ONE - r } else { r }
            }).product();
            for column in 0..3 {
                let bytes = bytemuck::cast_slice::<P1, u8>(&inputs[column]);
                if bytes[index / 8] & (1 << (index % 8)) != 0 { expected[column] += weight; }
            }
        }
        assert_eq!(actual, expected);
    }
    #[test]
    fn packed_inputs_match_byte_generator_and_canonical_digest() {
        for run in 0..3 {
            let log_n = 10;
            let (packed, digest) = witness(log_n, run);
            let mut rng = SplitMix64(0x6D69_6B72_6F73_0001 ^ ((log_n as u64) << 32) ^ run as u64);
            let mut bytes: [Vec<u8>; 3] = std::array::from_fn(|_| vec![0; 1 << (log_n - 3)]);
            for column in &mut bytes[..2] {
                for chunk in column.chunks_exact_mut(8) { chunk.copy_from_slice(&rng.next().to_le_bytes()); }
            }
            for i in 0..bytes[0].len() { bytes[2][i] = bytes[0][i] & bytes[1][i]; }
            let mut hash = blake3::Hasher::new();
            hash.update(b"mikros-flock-input-v1"); hash.update(&(log_n as u64).to_le_bytes());
            for column in 0..3 {
                hash.update(&(bytes[column].len() as u64).to_le_bytes()); hash.update(&bytes[column]);
                assert_eq!(bytemuck::cast_slice::<P1, u8>(&packed[column]), bytes[column]);
                for i in 0..1usize << log_n {
                    assert_eq!(packed[column][i / P1::WIDTH].get(i % P1::WIDTH),
                        if bytes[column][i / 8] & (1 << (i % 8)) == 0 { BinaryField1b::ZERO } else { BinaryField1b::ONE });
                }
            }
            assert_eq!(&digest, hash.finalize().as_bytes());
        }
    }
}
