//! Matched native hash workloads: one call or 256 independent calls.
//! Run each hash/system in a separate process to measure peak RSS with GNU time.
#![recursion_limit = "256"]
#![allow(dead_code)]
#[path = "e2e_common/mod.rs"]
pub(crate) mod e2e_common;
#[path = "e2e_common/hash_fixture.rs"]
pub(crate) mod hash_fixture;
#[path = "e2e_common/hash_mikros.rs"]
pub(crate) mod hash_mikros;
#[path = "e2e_common/hash_single_plonk.rs"]
pub(crate) mod hash_single_plonk;

use e2e_common::{PreparedWitness, TreeVerifier, batched_boolean};
use flock_core::{
    challenger::{Challenger, FsChallenger},
    lincheck::LincheckCircuit,
    pcs::{Commitment, PcsParams, ligerito::LigeritoProfile},
    proof::R1csProofCircuitMerged,
    schedule::IoDirection,
    verifier::verify_ligerito_union_circuit_with_skip,
};
use flock_field::F128;
use flock_prover::prover::{UnionSlotProverInput, prove_fast_ligerito_union_circuit_with_skip};
use hash_fixture::{HashFixture, HashKind};
use serde_json::json;
use std::{
    env,
    error::Error,
    io::{self, Write},
    time::Instant,
};

struct Options {
    kind: HashKind,
    system: String,
    instances: usize,
    skip: usize,
    threads: usize,
    reps: usize,
    warmups: usize,
    tamper: bool,
}
fn options(args: Vec<String>) -> Result<Options, String> {
    let mut o = Options {
        kind: HashKind::Sha256,
        system: "mikros".into(),
        instances: 256,
        skip: 7,
        threads: 64,
        reps: 6,
        warmups: 1,
        tamper: false,
    };
    let mut args = args.into_iter();
    while let Some(flag) = args.next() {
        if flag == "--tamper" {
            o.tamper = true;
            continue;
        }
        if flag == "--help" {
            println!(
                "--hash sha256|keccak|blake3 --system mikros|flock --instances 1|256 --skip 7 --threads N --reps N --warmups N --tamper"
            );
            std::process::exit(0);
        }
        let v = args
            .next()
            .ok_or_else(|| format!("missing value: {flag}"))?;
        match flag.as_str() {
            "--hash" => o.kind = HashKind::parse(&v)?,
            "--system" => o.system = v,
            "--instances" => o.instances = v.parse().map_err(|_| "invalid instances")?,
            "--skip" => o.skip = v.parse().map_err(|_| "invalid skip")?,
            "--threads" => o.threads = v.parse().map_err(|_| "invalid threads")?,
            "--reps" => o.reps = v.parse().map_err(|_| "invalid reps")?,
            "--warmups" => o.warmups = v.parse().map_err(|_| "invalid warmups")?,
            _ => return Err(format!("unknown argument {flag}")),
        }
    }
    if !matches!(o.instances, 1 | 256)
        || o.skip != 7
        || o.threads == 0
        || o.reps == 0
        || !matches!(o.system.as_str(), "mikros" | "flock")
    {
        return Err(
            "requires 1 or 256 instances, skip7, positive threads/reps, and mikros|flock".into(),
        );
    }
    Ok(o)
}
fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}
fn digest<T: serde::Serialize>(v: &T) -> String {
    blake3::hash(&bincode::serialize(v).unwrap())
        .to_hex()
        .to_string()
}
fn hex(v: &[u8]) -> String {
    v.iter().map(|b| format!("{b:02x}")).collect()
}
fn features() -> Vec<&'static str> {
    [
        ("avx2", cfg!(target_feature = "avx2")),
        ("avx512f", cfg!(target_feature = "avx512f")),
        ("avx512bw", cfg!(target_feature = "avx512bw")),
        ("gfni", cfg!(target_feature = "gfni")),
        ("vpclmulqdq", cfg!(target_feature = "vpclmulqdq")),
    ]
    .into_iter()
    .filter_map(|(n, b)| b.then_some(n))
    .collect()
}
fn flock_ch(kind: HashKind) -> FsChallenger {
    if kind == HashKind::Sha256 {
        return FsChallenger::new(e2e_common::DOMAIN);
    }
    let mut ch = FsChallenger::new(b"flock-native-three-hash-e2e-v1");
    ch.observe_label(kind.as_str().as_bytes());
    ch
}

/// Deliberately wrong public matrix, without changing the bound statement or
/// the witness. This exercises matrix checking instead of just FS rebinding.
struct WrongMatrix<'a> {
    original: &'a dyn LincheckCircuit,
    zero_left: bool,
}
impl LincheckCircuit for WrongMatrix<'_> {
    fn n_cols(&self) -> usize {
        self.original.n_cols()
    }
    fn const_pin_col(&self) -> Option<usize> {
        self.original.const_pin_col()
    }
    fn fold_alpha_batched(&self, alpha: F128, point: &[F128]) -> Vec<F128> {
        let (a, b) = self.fold_split(point);
        a.iter().zip(b).map(|(&a, b)| alpha * a + b).collect()
    }
    fn fold_split(&self, point: &[F128]) -> (Vec<F128>, Vec<F128>) {
        let (mut a, mut b) = self.original.fold_split(point);
        if self.zero_left {
            a.fill(F128::ZERO);
        } else {
            b.fill(F128::ZERO);
        }
        (a, b)
    }
    fn fold_side(&self, point: &[F128], left: bool) -> Vec<F128> {
        if left == self.zero_left {
            vec![F128::ZERO; self.n_cols()]
        } else {
            self.original.fold_side(point, left)
        }
    }
}

fn flock_verify(
    f: &HashFixture,
    public: &[F128],
    matrix: &dyn LincheckCircuit,
    cm: &Commitment,
    p: &R1csProofCircuitMerged,
    skip: usize,
    params: &PcsParams,
) -> Result<FsChallenger, String> {
    let union = f.union();
    let mut ch = flock_ch(f.kind);
    verify_ligerito_union_circuit_with_skip(
        &union,
        &f.circuit,
        public,
        &[matrix],
        cm,
        p,
        skip,
        params,
        &mut ch,
    )
    .map_err(|e| format!("complete FLOCK verify: {e:?}"))?;
    Ok(ch)
}

fn native_tamper(
    f: &HashFixture,
    params: &PcsParams,
    cm: &Commitment,
    p: &R1csProofCircuitMerged,
) -> Result<Vec<String>, String> {
    let matrix = f.lincheck_circuit();
    let mut cases = Vec::new();
    for field in 0..18 {
        let mut bad = p.clone();
        let b = bad.boolean.as_mut().ok_or("missing native boolean proof")?;
        let name = match field {
            0 => {
                b.zerocheck.round1_ab[0] += F128::ONE;
                "zc_round1_ab"
            }
            1 => {
                b.zerocheck.round1_c[0] += F128::ONE;
                "zc_round1_c"
            }
            2 => {
                b.zerocheck.multilinear_rounds[0].0 += F128::ONE;
                "zc_tail"
            }
            3 => {
                b.zerocheck.final_a_eval += F128::ONE;
                "zc_a"
            }
            4 => {
                b.zerocheck.final_b_eval += F128::ONE;
                "zc_b"
            }
            5 => {
                b.zerocheck.final_c_eval += F128::ONE;
                "zc_c"
            }
            6 => {
                b.lincheck.rounds[0].0 += F128::ONE;
                "lc_round"
            }
            7 => {
                b.lincheck.z_partial[0] += F128::ONE;
                "lc_partial"
            }
            8 => {
                b.lincheck.rounds.last_mut().unwrap().1 += F128::ONE;
                "lc_terminal"
            }
            9 => {
                b.lincheck.z_partial.pop();
                "lc_shape"
            }
            10 => {
                bad.wiring.gather[0] += F128::ONE;
                "wiring_gather"
            }
            11 => {
                bad.pcs_open.q_eval += F128::ONE;
                "pcs_evaluation"
            }
            12 => {
                bad.pcs_open.merged_rounds[0].0 += F128::ONE;
                "pcs_round"
            }
            13 => {
                bad.pcs_open.ring_switches[0].s_hat_v[0] += F128::ONE;
                "pcs_column"
            }
            14 => {
                bad.pcs_open.ring_switches.pop();
                "pcs_claims"
            }
            15 => {
                b.zerocheck.round1_ab.pop();
                "zc_shape"
            }
            16 => {
                b.zerocheck.grinding_nonces.pop();
                "zc_nonce_shape"
            }
            _ => {
                b.lincheck.grinding_nonces.pop();
                "lc_nonce_shape"
            }
        };
        if flock_verify(f, &f.public, matrix, cm, &bad, 7, params).is_ok() {
            return Err(format!("accepted FLOCK {name}"));
        }
        cases.push(name.into());
    }
    for skip in [6, 8] {
        if flock_verify(f, &f.public, matrix, cm, p, skip, params).is_ok() {
            return Err("accepted wrong skip".into());
        }
        cases.push(format!("wrong_skip_{skip}"));
    }
    let mut bad = cm.clone();
    bad.cap[0][0] ^= 1;
    if flock_verify(f, &f.public, matrix, &bad, p, 7, params).is_ok() {
        return Err("accepted bad commitment".into());
    }
    cases.push("commitment".into());
    for left in [true, false] {
        let wrong = WrongMatrix {
            original: matrix,
            zero_left: left,
        };
        if flock_verify(f, &f.public, &wrong, cm, p, 7, params).is_ok() {
            return Err("accepted wrong public matrix".into());
        }
        cases.push(format!("public_matrix_{}", if left { "a" } else { "b" }));
    }
    Ok(cases)
}

/// Every I/O word, including parameters, tested on both packed-group boundaries.
fn public_mutations(f: &HashFixture) -> Vec<(String, Vec<F128>)> {
    let schema = &f.registry.types()[0].io_schema;
    let mut out = Vec::new();
    for row in [0, 127, 128, 255].into_iter().filter(|&r| r < f.count()) {
        for word in schema {
            let b = f
                .public_bindings
                .iter()
                .find(|b| b.row == row && b.word_col == word.word_col)
                .unwrap();
            let mut public = f.public.clone();
            public[b.public_index] += F128::ONE;
            let direction = if word.dir == IoDirection::In {
                "input"
            } else {
                "output"
            };
            out.push((
                format!("public_{direction}_row{row}_word{}", word.word_col),
                public,
            ));
        }
    }
    out
}

fn main() -> Result<(), Box<dyn Error>> {
    run_with_args(env::args().skip(1).collect())
}

pub(crate) fn run_with_args(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let o = options(args).map_err(io::Error::other)?;
    if env::var_os("FLOCK_NO_GRIND").is_some() {
        return Err("FLOCK_NO_GRIND is forbidden for full verified proofs".into());
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(o.threads)
        .stack_size(8 << 20)
        .build_global()?;
    if !flock_parallel::init_all_core_pool(o.threads) {
        return Err("all-core pool already initialized".into());
    }
    // The native FLOCK verifier uses a single-thread pool. Match that policy
    // for both single-instance systems without changing the multiple runs.
    let verifier_pool = if o.instances == 1 {
        Some(
            rayon::ThreadPoolBuilder::new()
                .num_threads(1)
                .stack_size(8 << 20)
                .build()?,
        )
    } else {
        None
    };
    let t = Instant::now();
    let f = HashFixture::new(o.kind, o.instances);
    let view = f.verifier();
    let matrix = f.lincheck_circuit();
    let offline_fixture_ms = ms(t);
    let t = Instant::now();
    let prepared: PreparedWitness = f.prepare_witness();
    let offline_witness_ms = ms(t);
    let prepared_digest = digest(&prepared);
    let mut transpose_ms = 0.0;
    let mut public_setup_ms = 0.0;
    let transposed = if o.system == "mikros" && o.instances == 256 {
        let t = Instant::now();
        let w = batched_boolean::prepare(&prepared, f.nu, f.r1cs.k_log)?;
        transpose_ms = ms(t);
        Some(w)
    } else {
        None
    };
    let io = if o.system == "mikros" && o.instances == 256 {
        let t = Instant::now();
        let io = hash_mikros::public_io(&view, &f.io_word_columns, &f.public_bindings)?;
        public_setup_ms = ms(t);
        Some(io)
    } else {
        None
    };
    let single_plonk = if o.system == "mikros" && o.instances == 1 {
        let t = Instant::now();
        let setup = hash_single_plonk::Setup::new(&f, &prepared)?;
        public_setup_ms = ms(t);
        Some(setup)
    } else {
        None
    };
    let carrier = transposed.as_ref().map(|w| {
        let log = w.z.len().ilog2() as usize;
        hash_mikros::container::registry(log, log)
    });
    let params = if let Some(s) = &single_plonk {
        s.witness_params.clone()
    } else if let Some(r) = &carrier {
        let mut p = hash_mikros::container::parameters(&hash_mikros::container::union(r));
        p.num_lanes = None;
        p
    } else {
        f.pcs_params(LigeritoProfile::Fast)
    };
    let cfg = params.ligerito_prover_config()?;
    let program_id = match o.kind {
        HashKind::Sha256 => "native-sha256-compression-csc-v1",
        HashKind::Blake3 => "native-blake3-compression-csc-v1",
        HashKind::Keccak => "native-keccak-f1600-compact-walker-c2d0c248-v1",
    };
    let mut out = io::BufWriter::new(io::stdout().lock());
    writeln!(
        out,
        "{}",
        json!({"record":"metadata","schema_version":1,"hash":o.kind.as_str(),"system":o.system,
            "instances":o.instances,"unit":o.kind.unit(),"input_bits":o.kind.input_bits(),"output_bits":o.kind.output_bits(),
            "threads":rayon::current_num_threads(),"all_core_threads":flock_parallel::all_core_pool().current_num_threads(),
            "verifier_threads":if o.instances==1 {Some(1)}else{None},
            "reps":o.reps,"warmups":o.warmups,"baseline_skip":o.skip,"univariate_skip":o.skip,"mikros_kappa":128,
            "instance_groups":if o.instances==256 {Some(2)} else {None},
            "instance_mode":if o.instances==1 {"single"}else{"multiple"},
            "mikros_packing_axis":if o.instances==1 {"gate_bits"}else{"instances"},
            "native_instance_capacity":1usize<<f.nu,"r1cs_m":f.r1cs.m,
            "live_constraint_bits":o.instances*f.r1cs.useful_bits,
            "padding_policy":if single_plonk.is_some(){"one PLONK hash execution; selectors zero on padded gates, padding wires anchored by copy permutation; native PCS minimum domain zero padding"}else{"one active row per requested call; inactive native PCS capacity is zero-filled"},
            "r1cs_k_log":f.r1cs.k_log,"r1cs_c_identity":f.r1cs.c0_is_identity(),
            "native_k_skip":f.r1cs.k_skip,"matrix_authentication":if single_plonk.is_some(){"PLONK selectors and sigma digit tables authenticated by preprocessed public PCS commitment"}else{f.matrix_authentication()},
            "matrix_representation":if o.kind==HashKind::Keccak{"native_compact_walker"}else{"public_csc"},
            "program_id":program_id,
            "matrix_digest_scope":if o.kind==HashKind::Keccak{"R1CS descriptor only: A/B semantics are the trusted native Keccak walker bound by hash kind/program; full source SHA256 in experiment manifest"}else{"explicit public A/B/C matrices and native R1CS descriptor"},
            "pcs_params":params,"pcs_queries":cfg.queries,"pcs_query_grinding_bits":cfg.grinding_bits,
            "pcs_fold_grinding_bits":cfg.fold_grinding_bits,"target_features":features(),
            "packed_direct":env::var("FLOCK_PCS_PACKED_DIRECT").unwrap_or_default(),
            "zero_knowledge_claim":false,"verification":"complete non-deferred verifier; no witness input; matrix and sigma obligations discharged",
            "fixture_digest_blake3":f.fixture_digest_blake3(),"circuit_digest_blake3":hex(&f.circuit.digest()),
            "public_digest_blake3":digest(&f.public),
            "r1cs_digest_blake3":hex(&f.r1cs.statement_digest()),"prepared_witness_blake3":prepared_digest,
            "offline_fixture_ms":offline_fixture_ms,"offline_witness_ms":offline_witness_ms,
            "offline_transpose_ms":transpose_ms,"offline_public_setup_ms":public_setup_ms,
            "offline_copy_setup_ms":single_plonk.as_ref().map_or(0.0,|s|s.offline_copy_setup_ms),
            "public_word_columns":f.io_word_columns,"public_words":f.public.len(),
            "public_io_blocks":io.as_ref().map(|io|io.blocks.iter().map(|b|json!({"first_bit":b.first_bit,"log_bits":b.log_bits})).collect::<Vec<_>>()),
            "single_plonk":single_plonk.as_ref().map(|s|s.metadata()),
            "arithmetization":if single_plonk.is_some(){"binary PLONK"}else{"native R1CS"},
            "permutation_proof":if single_plonk.is_some(){"Mikros packed intermediate-field permutation"}else if o.system=="flock"{"native FLOCK Product-GKR"}else{"public word cover"},
            "timing_scope":"prepared input clone excluded; online conversion, commitment, reduction, configured grinding, public I/O/wiring and native PCS included; verifier separate",
            "ram_scope":"external GNU time peak RSS of whole process including offline setup, warmup, prover and verifier",
            "proof_encoding":if o.system=="mikros"{"bincode full Mikros proof including commitment and PCS"}else{"bincode (Commitment, R1csProofCircuitMerged)"}
        })
    )?;
    out.flush()?;
    let mut first = None;
    for iteration in 0..o.warmups + o.reps {
        let (stage, run, total) = if iteration < o.warmups {
            ("warmup", iteration + 1, o.warmups)
        } else {
            ("measured", iteration - o.warmups + 1, o.reps)
        };
        eprintln!(
            "[{}] {} {stage} run {run}/{total} start (instances={})",
            o.kind.as_str(),
            o.system,
            o.instances
        );
        let (encoded, cm_digest, prover_ms, verify_ms, timings, cases) = if let Some(w) =
            &transposed
        {
            let registry = carrier.as_ref().unwrap();
            let io = io.as_ref().unwrap();
            let owned = w.z.clone();
            let t = Instant::now();
            let (proof, timings, mut pch) =
                hash_mikros::prove(o.kind, &view, matrix, w, owned, io, &params, registry)?;
            let prover_ms = ms(t);
            let t = Instant::now();
            let mut vch =
                hash_mikros::verify(o.kind, &view, matrix, io, &params, registry, &proof)?;
            let verify_ms = ms(t);
            if pch.sample_f128() != vch.sample_f128() {
                return Err("Mikros FS mismatch".into());
            }
            let mut cases = Vec::new();
            if o.tamper && iteration == 0 {
                cases = hash_mikros::tamper(o.kind, &view, matrix, io, &params, registry, &proof)?;
                for left in [true, false] {
                    let wrong = WrongMatrix {
                        original: matrix,
                        zero_left: left,
                    };
                    if hash_mikros::verify(o.kind, &view, &wrong, io, &params, registry, &proof)
                        .is_ok()
                    {
                        return Err("Mikros accepted wrong public matrix".into());
                    }
                    cases.push(format!("public_matrix_{}", if left { "a" } else { "b" }));
                }
                for (name, public) in public_mutations(&f) {
                    let bad = TreeVerifier {
                        public: &public,
                        ..view
                    };
                    let bio = hash_mikros::public_io(&bad, &f.io_word_columns, &f.public_bindings)?;
                    if hash_mikros::verify(o.kind, &bad, matrix, &bio, &params, registry, &proof)
                        .is_ok()
                    {
                        return Err(format!("Mikros accepted {name}").into());
                    }
                    cases.push(name);
                }
                // Alter public expected values without changing the transcript's
                // bound statement: every interval must reach the actual PCS.
                for i in 0..io.blocks.len() {
                    let mut blocks = io.blocks.clone();
                    blocks[i].values[0] += flock_field::F256::ONE;
                    let bad = hash_mikros::PublicIo {
                        blocks,
                        public_bit_count: io.public_bit_count,
                    };
                    if hash_mikros::verify(o.kind, &view, matrix, &bad, &params, registry, &proof)
                        .is_ok()
                    {
                        return Err("Mikros accepted public interval forgery".into());
                    }
                    cases.push(format!("public_interval_{i}"));
                }
            }
            (
                bincode::serialize(&proof)?,
                digest(&proof.commitment),
                prover_ms,
                verify_ms,
                timings,
                cases,
            )
        } else if let Some(s) = &single_plonk {
            let owned = s.witness.clone();
            let t = Instant::now();
            let (proof, mut timings, mut pch) = hash_single_plonk::prove(o.kind, s, owned)?;
            let prover_ms = ms(t);
            let t = Instant::now();
            let (mut vch, verification) = verifier_pool.as_ref().unwrap().install(|| {
                hash_single_plonk::verify_with_timings(
                    o.kind,
                    &s.circuit,
                    &s.public_commitment,
                    &proof,
                )
            })?;
            let verify_ms = ms(t);
            if pch.sample_f128() != vch.sample_f128() {
                return Err("single Mikros FS mismatch".into());
            }
            let mut cases = Vec::new();
            if o.tamper && iteration == 0 {
                cases = hash_single_plonk::tamper(o.kind, s, &proof)?;
            }
            // Component accounting is outside both online timers. Shared PCS
            // authenticates gate and copy claims together and is counted once.
            let encoded = bincode::serialize(&proof)?;
            let [gate_bytes, copy_bytes, shared_bytes] = hash_single_plonk::proof_sizes(&proof)?;
            if gate_bytes + copy_bytes + shared_bytes != encoded.len() as u64 {
                return Err("single PLONK proof component sizes do not sum to full proof".into());
            }
            let gate_prover_ms = timings["gates_ms"].as_f64().unwrap();
            let copy_prover_ms =
                timings["copy_prepare_ms"].as_f64().unwrap() + timings["copy_ms"].as_f64().unwrap();
            let shared_prover_ms = prover_ms - gate_prover_ms - copy_prover_ms;
            let shared_verifier_ms = verify_ms - verification.gate_ms - verification.copy_ms;
            if shared_prover_ms < 0.0 || shared_verifier_ms < 0.0 {
                return Err("single PLONK constraint timers exceed E2E time".into());
            }
            timings["constraint_breakdown"] = json!({
                "gate": {"prover_ms":gate_prover_ms,"verifier_ms":verification.gate_ms,"proof_bytes":gate_bytes},
                "copy": {"prover_ms":copy_prover_ms,"verifier_ms":verification.copy_ms,"proof_bytes":copy_bytes},
                "shared": {"prover_ms":shared_prover_ms,"verifier_ms":shared_verifier_ms,"proof_bytes":shared_bytes}
            });
            (
                encoded,
                digest(&proof.commitment),
                prover_ms,
                verify_ms,
                timings,
                cases,
            )
        } else {
            let union = f.union();
            let owned = prepared.clone();
            let slot = UnionSlotProverInput::new(owned, matrix);
            let mut pch = flock_ch(o.kind);
            let t = Instant::now();
            let (p, cm, _) = prove_fast_ligerito_union_circuit_with_skip(
                &union,
                &f.circuit,
                &f.public,
                7,
                &params,
                vec![slot],
                vec![],
                &mut pch,
            );
            let prover_ms = ms(t);
            let t = Instant::now();
            let mut vch = if let Some(pool) = &verifier_pool {
                pool.install(|| flock_verify(&f, &f.public, matrix, &cm, &p, 7, &params))?
            } else {
                flock_verify(&f, &f.public, matrix, &cm, &p, 7, &params)?
            };
            let verify_ms = ms(t);
            if pch.sample_f128() != vch.sample_f128() {
                return Err("FLOCK FS mismatch".into());
            }
            let mut cases = Vec::new();
            if o.tamper && iteration == 0 {
                cases = native_tamper(&f, &params, &cm, &p)?;
                for (name, public) in public_mutations(&f) {
                    if flock_verify(&f, &public, matrix, &cm, &p, 7, &params).is_ok() {
                        return Err(format!("FLOCK accepted {name}").into());
                    }
                    cases.push(name);
                }
            }
            (
                bincode::serialize(&(&cm, &p))?,
                digest(&cm),
                prover_ms,
                verify_ms,
                json!({}),
                cases,
            )
        };
        let hash = blake3::hash(&encoded).to_hex().to_string();
        if first.replace(hash.clone()).is_some_and(|h| h != hash) {
            return Err("unstable proof bytes".into());
        }
        eprintln!(
            "[{}] {} {stage} run {run}/{total} finished: prover={prover_ms:.3} ms verifier={verify_ms:.3} ms proof={} bytes (instances={})",
            o.kind.as_str(),
            o.system,
            encoded.len(),
            o.instances
        );
        if let Some(parts) = timings.get("constraint_breakdown") {
            for (key, label) in [
                ("gate", "gate constraint"),
                ("copy", "copy constraint"),
                ("shared", "shared PCS/other"),
            ] {
                eprintln!(
                    "  {label}: prover={:.3} ms verifier={:.3} ms proof={} bytes",
                    parts[key]["prover_ms"].as_f64().unwrap(),
                    parts[key]["verifier_ms"].as_f64().unwrap(),
                    parts[key]["proof_bytes"]
                );
            }
        }
        writeln!(
            out,
            "{}",
            json!({"record":"sample","hash":o.kind.as_str(),"system":o.system,"iteration":iteration,
            "warmup":iteration<o.warmups,"prover_ms":prover_ms,"verify_ms":verify_ms,"timings":timings,
            "proof_bytes":encoded.len(),"proof_blake3":hash,"commitment_blake3":cm_digest,
            "verified":true,"transcript_match":true})
        )?;
        out.flush()?;
        if !cases.is_empty() {
            writeln!(
                out,
                "{}",
                json!({"record":"negative_tests","hash":o.kind.as_str(),"system":o.system,
            "rejected":cases.len(),"cases":cases})
            )?;
            out.flush()?;
        }
    }
    Ok(())
}
