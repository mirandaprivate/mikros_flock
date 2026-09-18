//! Updated Section 4/5 PLONK E2E comparison; native FLOCK PCS for both systems.
#![recursion_limit = "256"]
#![allow(dead_code)]
#[path = "e2e_plonk/copy.rs"]
mod copy;
#[path = "e2e_plonk/fixture.rs"]
mod fixture;
#[path = "e2e_plonk/flock_r1cs.rs"]
mod flock_r1cs;
#[path = "e2e_plonk/gates.rs"]
mod gates;
#[path = "e2e_common/grinding.rs"]
mod grinding;
#[path = "e2e_plonk/queries.rs"]
mod queries;

use fixture::{Circuit, Mode};
use flock_core::{
    challenger::{Challenger, FsChallenger},
    pcs::{self, Commitment, PcsParams, ProverData, merged_tensor},
    product_gkr::BatchedGrinding,
    schedule::Registry,
    scratch,
};
use flock_field::F128;
use grinding::GrindingChallenger;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    env,
    io::{self, Write},
    time::Instant,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum System {
    Mikros,
    Flock,
}
impl System {
    fn name(self) -> &'static str {
        if self == Self::Mikros {
            "mikros"
        } else {
            "flock"
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
enum GateProof {
    Single(gates::SingleProof),
    Multiple(gates::MultipleProof),
    Flock(gates::FlockProof),
}
#[derive(Clone, Debug, Serialize, Deserialize)]
enum CopyProof {
    Single(copy::SingleProof),
    Multiple(copy::MultipleProof),
    Flock(copy::FlockProof),
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Proof {
    system: System,
    witness: Commitment,
    gate: GateProof,
    copy: CopyProof,
    gate_nonces: Vec<u64>,
    copy_nonces: Vec<u64>,
    opening: Option<mikros::packed_opening::Proof>,
    opening_nonces: Vec<u64>,
    witness_pcs: merged_tensor::Proof,
    public_pcs: merged_tensor::Proof,
}
#[derive(Default, Serialize)]
struct Times {
    commit_ms: f64,
    recommit_ms: f64,
    public_recommit_ms: f64,
    gates_ms: f64,
    copy_ms: f64,
    copy_prepare_ms: f64,
    packed_opening_ms: f64,
    release_ms: f64,
    witness_pcs_open_ms: f64,
    public_pcs_open_ms: f64,
    serialization_ms: f64,
    prover_ms: f64,
    witness_claims: usize,
    public_claims: usize,
    witness_ring_claims: usize,
    public_ring_claims: usize,
    native_pcs_claims: usize,
}
fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}
fn clear_scratch() {
    scratch::clear();
    mikros_permutation::tree_pool::clear_all();
}
fn event(stage: &str) {
    eprintln!("stage={stage}");
    io::stderr().flush().unwrap();
}
fn bind(c: &Circuit, system: System, w: &Commitment, p: &Commitment) -> FsChallenger {
    let mut ch = FsChallenger::new(b"mikros-flock-plonk-section4-5-native-pcs-e2e-v1");
    ch.observe_bytes(&c.digest());
    ch.observe_label(system.name().as_bytes());
    ch.observe_bytes(&bincode::serialize(w).unwrap());
    ch.observe_bytes(&bincode::serialize(p).unwrap());
    ch
}
fn merge(mut a: gates::Claims, b: gates::Claims) -> gates::Claims {
    a.witness.extend(b.witness);
    a.public.extend(b.public);
    a
}

/// Diagnostic evidence only: this never enters the Fiat-Shamir transcript.
/// Keep the PIOP and ordered source claims separate from the PCS proof so
/// native opening routes can be compared even when their proof bytes differ.
fn evidence_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> Result<String, String> {
    let encoded = bincode::serialize(value).map_err(|e| e.to_string())?;
    let mut hash = blake3::Hasher::new_derive_key(domain);
    hash.update(&encoded);
    Ok(hash.finalize().to_hex().to_string())
}

struct Setup {
    circuit: Circuit,
    system: System,
    witness: Vec<F128>,
    public: Vec<F128>,
    witness_registry: Registry,
    public_registry: Registry,
    witness_params: PcsParams,
    public_params: PcsParams,
    public_commitment: Commitment,
    public_data: ProverData,
    sigma: Option<Vec<usize>>,
    offline_ms: f64,
    bounded_copy: bool,
}

impl Setup {
    fn new(c: Circuit, system: System, bounded_copy: bool) -> Result<Self, String> {
        event("offline_witness_public_tables");
        let offline = Instant::now();
        let z = c.witness();
        let public = c.public();
        let log_z = z.len().ilog2() as usize;
        let log_pub = public.len().ilog2() as usize;
        let wr = fixture::registry(log_z, c.wire_words().ilog2() as usize);
        let pr = fixture::registry(
            log_pub,
            if c.mode == Mode::Single {
                c.wire_words().ilog2() as usize
            } else {
                c.log_gates
            },
        );
        let wu = fixture::union(&wr);
        let pu = fixture::union(&pr);
        let wp = fixture::parameters(&wu);
        let pp = fixture::parameters(&pu);
        wp.ligerito_prover_config()
            .map_err(|e| format!("witness PCS configuration: {e:?}"))?;
        pp.ligerito_prover_config()
            .map_err(|e| format!("public PCS configuration: {e:?}"))?;
        event("offline_public_commitment");
        let (public_cm, pd) = pcs::commit(&public, &pp);
        clear_scratch();
        event("offline_public_wiring");
        let sigma = if system == System::Flock {
            Some(c.sigma_table())
        } else {
            None
        };
        let offline_ms = ms(offline);
        clear_scratch();
        Ok(Self {
            circuit: c,
            system,
            witness: z,
            public,
            witness_registry: wr,
            public_registry: pr,
            witness_params: wp,
            public_params: pp,
            public_commitment: public_cm,
            public_data: pd,
            sigma,
            offline_ms,
            bounded_copy,
        })
    }
}

enum PreparedCopy {
    Cached(mikros_permutation::packed::Prepared),
    Bounded(mikros_permutation::packed::BoundedPrepared),
}

fn peak_rss_kib() -> Option<u64> {
    std::fs::read_to_string("/proc/self/status")
        .ok()?
        .lines()
        .find_map(|line| {
            line.strip_prefix("VmHWM:")?
                .split_whitespace()
                .next()?
                .parse()
                .ok()
        })
}

fn run(s: &Setup) -> Result<serde_json::Value, String> {
    let c = &s.circuit;
    let system = s.system;
    // The proof consumes its input. Prepare that owned input before online
    // timing, as for the other benchmark fixtures' witness generation.
    let mut z = s.witness.clone();
    let public = &s.public;
    let log_pub = public.len().ilog2() as usize;
    let wu = fixture::union(&s.witness_registry);
    let pu = fixture::union(&s.public_registry);
    let wp = &s.witness_params;
    let pp = &s.public_params;
    let public_cm = &s.public_commitment;
    event("online_witness_commitment");
    let begin = Instant::now();
    let mut times = Times::default();
    let t = Instant::now();
    let (witness_cm, wd) = pcs::commit(&z, wp);
    times.commit_ms = ms(t);
    let mut parent = bind(c, system, &witness_cm, public_cm);
    let gc = parent.fork(b"gates");
    let cc = parent.fork(b"copy");
    event("online_gate_piop");
    let t = Instant::now();
    let mut g = GrindingChallenger::prover(gc);
    let (gate, gclaims) = match (system, c.mode) {
        (System::Flock, _) => {
            gates::flock_prove(c, &z, &mut g).map(|(p, q)| (GateProof::Flock(p), q))?
        }
        (System::Mikros, Mode::Single) => {
            gates::single_prove(c, &z, public, &mut g).map(|(p, q)| (GateProof::Single(p), q))?
        }
        (System::Mikros, Mode::Multiple) => gates::multiple_prove(c, &z, log_pub, &mut g)
            .map(|(p, q)| (GateProof::Multiple(p), q))?,
    };
    let (gc, gate_nonces) = g.finish()?;
    times.gates_ms = ms(t);
    let t = Instant::now();
    clear_scratch();
    times.release_ms += ms(t);
    event("online_copy_prepare");
    let t = Instant::now();
    // Both conversions read current private witness bits. Public wiring and
    // its commitment are cached in Setup, but these preparations are online.
    let mut packed = if system == System::Mikros && c.mode == Mode::Single {
        Some(if s.bounded_copy {
            PreparedCopy::Bounded(copy::prepare_single_bounded(c, &z)?)
        } else {
            PreparedCopy::Cached(copy::prepare_single(c, &z)?)
        })
    } else {
        None
    };
    let single_values = if system == System::Flock && c.mode == Mode::Single {
        Some(copy::prepare_flock_single(c, &z)?)
    } else {
        None
    };
    times.copy_prepare_ms = ms(t);
    event("online_copy_piop");
    let t = Instant::now();
    let (copy, cclaims, cc, copy_nonces) = if system == System::Flock {
        let mut cc = cc;
        let (p, q) = copy::flock_prove(
            c,
            &mut z,
            &public,
            single_values.as_deref(),
            s.sigma.as_deref().unwrap(),
            BatchedGrinding::per_challenge_128(),
            &mut cc,
        )?;
        (CopyProof::Flock(p), q, cc, vec![])
    } else {
        let mut cc = if c.mode == Mode::Single {
            GrindingChallenger::prover(cc)
                .packed_permutation(c.log_gates + 2, 128 / (c.h() + 1).next_power_of_two())
        } else {
            GrindingChallenger::prover(cc).permutation(c.log_gates + 2)
        };
        let (p, q) = if c.mode == Mode::Single {
            let result = match packed.take().unwrap() {
                PreparedCopy::Cached(prepared) => {
                    copy::single_prove(c, &z, public, &prepared, &mut cc)
                }
                PreparedCopy::Bounded(prepared) => {
                    copy::single_prove_bounded(c, &z, public, prepared, &mut cc)
                }
            };
            result.map(|(p, q)| (CopyProof::Single(p), q))?
        } else {
            copy::multiple_prove(c, &z, &public, &mut cc)
                .map(|(p, q)| (CopyProof::Multiple(p), q))?
        };
        let (cc, nonces) = cc.finish()?;
        (p, q, cc, nonces)
    };
    times.copy_ms = ms(t);
    let t = Instant::now();
    drop(packed);
    drop(single_values);
    clear_scratch();
    times.release_ms += ms(t);
    let claims = merge(gclaims, cclaims);
    times.witness_claims = claims.witness.len();
    times.public_claims = claims.public.len();
    if claims.public.is_empty() {
        return Err("public PCS queries must not be omitted".into());
    }
    parent.merge_child(gc);
    parent.merge_child(cc);
    let t = Instant::now();
    let piop_blake3 = evidence_hash(
        "mikros-flock-e2e-piop-evidence-v1",
        &(&gate, &gate_nonces, &copy, &copy_nonces),
    )?;
    let terminal_claims_blake3 = evidence_hash(
        "mikros-flock-e2e-terminal-claims-evidence-v1",
        &(&claims.witness, &claims.public),
    )?;
    times.serialization_ms += ms(t);
    event("online_opening_reduction");
    let t = Instant::now();
    let (opening, opening_nonces, terminals, mut parent) = if system == System::Mikros {
        let mut och = GrindingChallenger::prover(parent);
        let (proof, terminal) = mikros::packed_opening::prove(
            &[(&z, &claims.witness), (public, &claims.public)],
            &mut och,
        )?;
        let (parent, nonces) = och.finish()?;
        (Some(proof), nonces, Some(terminal), parent)
    } else {
        (None, vec![], None, parent)
    };
    times.packed_opening_ms = ms(t);
    let witness_queries = terminals
        .as_ref()
        .map_or(claims.witness.as_slice(), |q| &q[..1]);
    let public_queries = terminals
        .as_ref()
        .map_or(claims.public.as_slice(), |q| &q[1..]);
    times.native_pcs_claims = witness_queries.len() + public_queries.len();
    event("online_witness_pcs_open");
    let t = Instant::now();
    let witness_pcs =
        merged_tensor::open_identity(z, &wd, &witness_cm, witness_queries, &wu, &mut parent)
            .map_err(|e| format!("witness native PCS open: {e:?}"))?;
    times.witness_pcs_open_ms = ms(t);
    let t = Instant::now();
    drop(wd);
    clear_scratch();
    times.release_ms += ms(t);
    event("online_public_pcs_open");
    let t = Instant::now();
    // Native PCS consumes its source, so the required public copy is online.
    let public_pcs = merged_tensor::open_identity(
        public.clone(),
        &s.public_data,
        public_cm,
        public_queries,
        &pu,
        &mut parent,
    )
    .map_err(|e| format!("public native PCS open: {e:?}"))?;
    times.public_pcs_open_ms = ms(t);
    let t = Instant::now();
    clear_scratch();
    times.release_ms += ms(t);
    times.witness_ring_claims = witness_pcs
        .merged
        .as_ref()
        .map_or(0, |p| p.ring_switches.len());
    times.public_ring_claims = public_pcs
        .merged
        .as_ref()
        .map_or(0, |p| p.ring_switches.len());
    let proof = Proof {
        system,
        witness: witness_cm,
        gate,
        copy,
        gate_nonces,
        copy_nonces,
        opening,
        opening_nonces,
        witness_pcs,
        public_pcs,
    };
    let t = Instant::now();
    let encoded = bincode::serialize(&proof).map_err(|e| e.to_string())?;
    times.serialization_ms += ms(t);
    times.prover_ms = ms(begin);
    let peak_rss_kib = peak_rss_kib();
    event("verify_full_proof");
    let t = Instant::now();
    let decoded: Proof = bincode::deserialize(&encoded).map_err(|e| e.to_string())?;
    let (mut v, verify_times) = verify_with_timings(c, system, public_cm, &decoded)?;
    let verify_ms = ms(t);
    let transcript_match = parent.sample_f128() == v.sample_f128();
    if !transcript_match {
        return Err("prover/verifier transcript mismatch".into());
    }
    // The public opening authenticates both selectors and sigma digits. Charge
    // that indivisible proof once, to the permutation row; the joint packed
    // opening reduction and the witness PCS stay in the shared row.
    let gate_bytes =
        bincode::serialized_size(&(&proof.gate, &proof.gate_nonces)).map_err(|e| e.to_string())?;
    let copy_bytes =
        bincode::serialized_size(&(&proof.copy, &proof.copy_nonces, &proof.public_pcs))
            .map_err(|e| e.to_string())?;
    let shared_bytes = (encoded.len() as u64)
        .checked_sub(gate_bytes + copy_bytes)
        .ok_or("component proof sizes exceed total")?;
    let copy_prover_ms = times.copy_prepare_ms + times.copy_ms + times.public_pcs_open_ms;
    let copy_verifier_ms = verify_times.copy_ms + verify_times.public_pcs_ms;
    let shared_prover_ms = times.prover_ms - times.gates_ms - copy_prover_ms;
    let shared_verifier_ms = verify_ms - verify_times.gate_ms - copy_verifier_ms;
    if shared_prover_ms < 0.0 || shared_verifier_ms < 0.0 {
        return Err("component timing exceeds end-to-end measurement".into());
    }
    let constraint_breakdown = json!({
        "gate": {"prover_ms":times.gates_ms,"verifier_ms":verify_times.gate_ms,"proof_bytes":gate_bytes},
        "copy": {"prover_ms":copy_prover_ms,"verifier_ms":copy_verifier_ms,"proof_bytes":copy_bytes},
        "shared": {"prover_ms":shared_prover_ms,"verifier_ms":shared_verifier_ms,"proof_bytes":shared_bytes}
    });
    Ok(
        json!({"type":"sample","mode":c.mode.name(),"system":system.name(),"log_gates":c.log_gates,"gates_per_instance":c.n(),"instances":c.mode.instances(),"total_gates":c.n() as u64*c.mode.instances() as u64,"threads":rayon::current_num_threads(),"seed":c.seed,"offline_ms":s.offline_ms,"times":times,"constraint_breakdown":constraint_breakdown,"verify_ms":verify_ms,"verified":true,"transcript_match":true,"pcs_config_match":true,"proof_bytes":encoded.len(),"proof_blake3":blake3::hash(&encoded).to_hex().to_string(),"piop_blake3":piop_blake3,"terminal_claims_blake3":terminal_claims_blake3,"witness_commitment_blake3":blake3::hash(&bincode::serialize(&proof.witness).unwrap()).to_hex().to_string(),"public_commitment_blake3":blake3::hash(&bincode::serialize(public_cm).unwrap()).to_hex().to_string(),"pcs_path":"FLOCK native commit + merged_tensor/open_batch_merged + Ligerito Fast","witness_pcs_params":wp,"public_pcs_params":pp,"native_pcs_identity_transport":true,"memory_policy":"retain online witness PCS data and offline public PCS data; witness-dependent copy preparation and required public opening copy included online","peak_rss_kib":peak_rss_kib,"rss_scope":"process VmHWM immediately after proving, before verification; includes cached offline data and prior repetitions","piop_grinding":"gate:8 bits/event; copy:degree-dependent initial fingerprint and native/per-event tails; no aggregate 128-bit theorem asserted","circuit":"connected alternating AND/XOR; one public final output; same wiring, 128 independent inputs in multiple"}),
    )
}

/// Public verifier only: no witness, source generator, sigma array or PCS data.
fn verify(
    c: &Circuit,
    system: System,
    public_cm: &Commitment,
    p: &Proof,
) -> Result<FsChallenger, String> {
    verify_with_timings(c, system, public_cm, p).map(|(ch, _)| ch)
}

#[derive(Default)]
struct VerificationTimes {
    gate_ms: f64,
    copy_ms: f64,
    public_pcs_ms: f64,
}

fn verify_with_timings(
    c: &Circuit,
    system: System,
    public_cm: &Commitment,
    p: &Proof,
) -> Result<(FsChallenger, VerificationTimes), String> {
    if p.system != system {
        return Err("wrong proof system".into());
    }
    let log_z = (4 * c.wire_words()).max(1 << 15).ilog2() as usize;
    let log_pub = c.public_words().ilog2() as usize;
    let wr = fixture::registry(log_z, c.wire_words().ilog2() as usize);
    let pr = fixture::registry(
        log_pub,
        if c.mode == Mode::Single {
            c.wire_words().ilog2() as usize
        } else {
            c.log_gates
        },
    );
    let wu = fixture::union(&wr);
    let pu = fixture::union(&pr);
    if p.witness.params != fixture::parameters(&wu) || public_cm.params != fixture::parameters(&pu)
    {
        return Err("native PCS parameter mismatch".into());
    }
    let mut parent = bind(c, system, &p.witness, public_cm);
    let gc = parent.fork(b"gates");
    let cc = parent.fork(b"copy");
    let mut times = VerificationTimes::default();
    let timer = Instant::now();
    let mut g = GrindingChallenger::verifier(gc, &p.gate_nonces);
    let gclaims = match (&p.gate, system, c.mode) {
        (GateProof::Flock(p), System::Flock, _) => gates::flock_verify(c, log_z, p, &mut g)?,
        (GateProof::Single(p), System::Mikros, Mode::Single) => {
            gates::single_verify(c, log_z, log_pub, p, &mut g)?
        }
        (GateProof::Multiple(p), System::Mikros, Mode::Multiple) => {
            gates::multiple_verify(c, log_z, log_pub, p, &mut g)?
        }
        _ => return Err("gate proof mode".into()),
    };
    let (gc, _) = g.finish()?;
    times.gate_ms = ms(timer);
    let timer = Instant::now();
    let (cclaims, cc) = if let (CopyProof::Flock(fp), System::Flock) = (&p.copy, system) {
        let mut cc = cc;
        let q = copy::flock_verify(
            c,
            log_z,
            log_pub,
            fp,
            BatchedGrinding::per_challenge_128(),
            &mut cc,
        )?;
        (q, cc)
    } else {
        let mut cc = if c.mode == Mode::Single {
            GrindingChallenger::verifier(cc, &p.copy_nonces)
                .packed_permutation(c.log_gates + 2, 128 / (c.h() + 1).next_power_of_two())
        } else {
            GrindingChallenger::verifier(cc, &p.copy_nonces).permutation(c.log_gates + 2)
        };
        let q = match (&p.copy, system, c.mode) {
            (CopyProof::Single(p), System::Mikros, Mode::Single) => {
                copy::single_verify(c, log_z, log_pub, p, &mut cc)?
            }
            (CopyProof::Multiple(p), System::Mikros, Mode::Multiple) => {
                copy::multiple_verify(c, log_z, log_pub, p, &mut cc)?
            }
            _ => return Err("copy proof mode".into()),
        };
        let (cc, _) = cc.finish()?;
        (q, cc)
    };
    if system == System::Flock && !p.copy_nonces.is_empty() {
        return Err("extraneous Flock copy nonces".into());
    }
    times.copy_ms = ms(timer);
    let claims = merge(gclaims, cclaims);
    parent.merge_child(gc);
    parent.merge_child(cc);
    let (terminals, mut parent) = if system == System::Mikros {
        let opening = p.opening.as_ref().ok_or("missing Mikros packed opening")?;
        let mut och = GrindingChallenger::verifier(parent, &p.opening_nonces);
        let terminal = mikros::packed_opening::verify(
            &[log_z, log_pub],
            &[&claims.witness, &claims.public],
            opening,
            &mut och,
        )?;
        let (parent, _) = och.finish()?;
        (Some(terminal), parent)
    } else {
        if p.opening.is_some() || !p.opening_nonces.is_empty() {
            return Err("extraneous FLOCK packed opening".into());
        }
        (None, parent)
    };
    let witness_queries = terminals
        .as_ref()
        .map_or(claims.witness.as_slice(), |q| &q[..1]);
    let public_queries = terminals
        .as_ref()
        .map_or(claims.public.as_slice(), |q| &q[1..]);
    merged_tensor::verify(
        &p.witness,
        witness_queries,
        &p.witness_pcs,
        &wu,
        &mut parent,
    )
    .map_err(|e| format!("witness PCS verify: {e:?}"))?;
    let timer = Instant::now();
    merged_tensor::verify(public_cm, public_queries, &p.public_pcs, &pu, &mut parent)
        .map_err(|e| format!("public PCS verify: {e:?}"))?;
    times.public_pcs_ms = ms(timer);
    Ok((parent, times))
}
fn main() {
    run_with_args(env::args().skip(1).collect());
}

/// Shared worker entry for the Cargo benchmark wrapper and standalone example.
pub fn run_with_args(args: Vec<String>) {
    assert!(
        env::var_os("FLOCK_NO_GRIND").is_none(),
        "FLOCK_NO_GRIND is not supported"
    );
    let arg = |key: &str, default: &str| {
        args.iter()
            .position(|x| x == key)
            .map(|i| args.get(i + 1).expect("option value").clone())
            .unwrap_or_else(|| default.into())
    };
    let mode = match arg("--mode", "single").as_str() {
        "single" => Mode::Single,
        "multiple" => Mode::Multiple,
        x => panic!("invalid mode {x}"),
    };
    let system = match arg("--system", "mikros").as_str() {
        "mikros" => System::Mikros,
        "flock" => System::Flock,
        x => panic!("invalid system {x}"),
    };
    let log_gates = arg("--log-gates", "25").parse().unwrap();
    let threads = arg("--threads", "1").parse().unwrap();
    let repetitions: usize = arg("--repetitions", "1").parse().unwrap();
    let warmups: usize = arg("--warmups", "0").parse().unwrap();
    let bounded_copy = match arg("--copy-storage", "bounded").as_str() {
        "bounded" => true,
        "cached" => false,
        x => panic!("invalid copy storage {x}"),
    };
    let seed = arg("--seed", "97").parse().unwrap();
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .stack_size(8 << 20)
        .build_global()
        .unwrap();
    assert!(
        flock_parallel::init_all_core_pool(threads),
        "all-core pool already initialized"
    );
    let all_core_threads = flock_parallel::all_core_pool().current_num_threads();
    assert_eq!(all_core_threads, threads);
    let circuit = Circuit::new(log_gates, mode, seed);
    if system == System::Flock && mode == Mode::Single {
        if let Err(error) = flock_r1cs::run(&circuit, repetitions, warmups) {
            eprintln!("native FLOCK R1CS benchmark failed: {error}");
            std::process::exit(1);
        }
        return;
    }
    println!(
        "{}",
        json!({"type":"metadata","mode":mode.name(),"system":system.name(),"log_gates":log_gates,"instances":mode.instances(),"threads":threads,"all_core_threads":all_core_threads,"seed":seed,"repetitions":repetitions,"warmups":warmups,"fixture_digest":circuit.digest().iter().map(|b|format!("{b:02x}")).collect::<String>(),"baseline_skip":7,"public_io":[circuit.io.lo,circuit.io.hi],"arithmetization":"binary PLONK","matrix_authentication":"preprocessed public PCS","permutation_proof":true,"genuine_gate_count":circuit.n(),"copy_storage":if system == System::Flock { "native grouped bounded layers" } else if bounded_copy { "consume coefficients; defer samples; release tree and fold buffers" } else { "cached coefficient reference" },"pcs_backend":"FLOCK native Ligerito Fast","pcs_packed_direct":env::var_os("FLOCK_PCS_PACKED_DIRECT").is_some_and(|v|v=="1"),"mikros_packed_opening":system==System::Mikros,"witness_words":(4*circuit.wire_words()).max(1<<15),"public_words":circuit.public_words(),"component_scope":"Permutation argument includes witness-dependent copy preparation, permutation PIOP and entire public PCS opening (sigma plus shared selectors); joint packed-opening reduction and witness PCS counted once in Shared PCS/other","timing_scope":"online witness commitment, gates, witness-dependent copy preparation, copy reduction, opening reduction and PCS openings; cached public commitment and wiring offline; serialization included","target_features":{"gfni":cfg!(target_feature="gfni"),"avx512f":cfg!(target_feature="avx512f"),"avx512bw":cfg!(target_feature="avx512bw"),"vpclmulqdq":cfg!(target_feature="vpclmulqdq")}})
    );
    io::stdout().flush().unwrap();
    let setup = match Setup::new(circuit, system, bounded_copy) {
        Ok(s) => s,
        Err(error) => {
            println!(
                "{}",
                json!({"type":"error","system":system.name(),"stage":"setup","error":error})
            );
            std::process::exit(1);
        }
    };
    for run_index in 0..warmups + repetitions {
        let warmup = run_index < warmups;
        let repetition = if warmup {
            run_index
        } else {
            run_index - warmups
        };
        let count = if warmup { warmups } else { repetitions };
        let kind = if warmup { "warmup" } else { "sample" };
        eprintln!(
            "[single constraints] {} {kind} {}/{} start (2^{log_gates} gates)",
            system.name(),
            repetition + 1,
            count
        );
        match run(&setup) {
            Ok(mut row) => {
                row["repetition"] = json!(repetition);
                row["warmup"] = json!(warmup);
                eprintln!(
                    "[single constraints] {} {kind} {}/{} finished: prover={:.3} ms verifier={:.3} ms proof={} bytes",
                    system.name(),
                    repetition + 1,
                    count,
                    row["times"]["prover_ms"].as_f64().unwrap(),
                    row["verify_ms"].as_f64().unwrap(),
                    row["proof_bytes"]
                );
                if system == System::Mikros && mode == Mode::Single {
                    for (key, label) in [
                        ("gate", "Gate constraint"),
                        ("copy", "Permutation argument"),
                        ("shared", "Shared PCS/other"),
                    ] {
                        let part = &row["constraint_breakdown"][key];
                        eprintln!(
                            "[single constraints] {label}: prover={:.3} ms verifier={:.3} ms proof={} bytes",
                            part["prover_ms"].as_f64().unwrap(),
                            part["verifier_ms"].as_f64().unwrap(),
                            part["proof_bytes"]
                        );
                    }
                }
                println!("{row}");
                io::stdout().flush().unwrap();
            }
            Err(error) => {
                println!(
                    "{}",
                    json!({"type":"error","mode":mode.name(),"system":system.name(),"repetition":repetition,"error":error})
                );
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod evidence_tests {
    use super::*;
    use flock_core::pcs::functional::TensorClaim;
    use flock_field::F128;

    #[test]
    fn piop_evidence_survives_serialization_and_detects_proof_or_nonce_changes() {
        // Minimal typed payloads test diagnostic serialization, not validity.
        let gate = GateProof::Single(gates::SingleProof {
            scalars: [F128::ONE; 5],
            hadamards: vec![],
        });
        let copy = CopyProof::Multiple(copy::MultipleProof {
            product: mikros::extension_product::Proof {
                root: F128::ONE,
                trees: std::array::from_fn(|_| mikros::extension_product::TreeProof {
                    layers: vec![],
                }),
            },
            wire_values: [[F128::ONE; 3]; 2],
            label_values: [F128::ZERO; 3],
        });
        let payload = (gate, vec![3u64], copy, vec![7u64]);
        let encoded = bincode::serialize(&payload).unwrap();
        let replay: (GateProof, Vec<u64>, CopyProof, Vec<u64>) =
            bincode::deserialize(&encoded).unwrap();
        let domain = "mikros-flock-e2e-piop-evidence-v1";
        let expected = evidence_hash(domain, &payload).unwrap();
        assert_eq!(evidence_hash(domain, &replay).unwrap(), expected);
        assert_eq!(bincode::serialize(&payload).unwrap(), encoded);
        let mut changed = replay.clone();
        changed.1[0] += 1;
        assert_ne!(evidence_hash(domain, &changed).unwrap(), expected);
        let mut changed = replay;
        let CopyProof::Multiple(p) = &mut changed.2 else {
            unreachable!();
        };
        p.wire_values[0][0] += F128::ONE;
        assert_ne!(evidence_hash(domain, &changed).unwrap(), expected);
    }

    #[test]
    fn terminal_evidence_binds_functional_and_source_group_order() {
        let witness = vec![TensorClaim {
            columns: fixture::basis(),
            rows: vec![[F128::ONE, F128::ZERO]],
            value: F128::ONE,
        }];
        let public = vec![TensorClaim {
            columns: queries::selector_column(2),
            rows: queries::pairs(&[F128::new(7, 0)]),
            value: F128::ZERO,
        }];
        let domain = "mikros-flock-e2e-terminal-claims-evidence-v1";
        let expected = evidence_hash(domain, &(&witness, &public)).unwrap();
        assert_eq!(
            evidence_hash(domain, &(witness.clone(), public.clone())).unwrap(),
            expected
        );
        assert_ne!(
            evidence_hash(domain, &(&public, &witness)).unwrap(),
            expected
        );
        for field in 0..3 {
            let mut changed = witness.clone();
            match field {
                0 => changed[0].columns[0] += F128::ONE,
                1 => changed[0].rows[0][0] += F128::ONE,
                _ => changed[0].value += F128::ONE,
            }
            assert_ne!(
                evidence_hash(domain, &(&changed, &public)).unwrap(),
                expected
            );
        }
    }
}
