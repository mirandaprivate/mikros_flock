//! Shared binary-field PLONK driver with a backend-specific packed MLE PCS adapter.
#![allow(dead_code)]
#[path = "../e2e_plonk_bitwidth/copy.rs"]
mod copy;
#[path = "../e2e_plonk_bitwidth/fixture.rs"]
mod fixture;
#[path = "../e2e_plonk_bitwidth/gates.rs"]
mod gates;
#[path = "grinding.rs"]
mod grinding;
#[path = "../e2e_plonk_bitwidth/hadamard.rs"]
mod hadamard;
#[path = "../e2e_plonk/fixture.rs"]
mod old_fixture;
#[path = "../e2e_plonk/queries.rs"]
mod queries;

use super::pcs as backend;
use backend::{Commitment, VerifierKey};
use bincode::Options;
use fixture::{Circuit, Claims};
use flock_core::challenger::{Challenger, FsChallenger};
use flock_field::F128;
use grinding::GrindingChallenger;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    io::{self, Write},
    time::Instant,
};

type Result<T> = std::result::Result<T, String>;

fn decode_proof(bytes: &[u8]) -> Result<Proof> {
    if bytes.len() > 512 * 1024 * 1024 {
        return Err("PLONK proof exceeds the 512 MiB decoding limit".into());
    }
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(bytes.len() as u64)
        .reject_trailing_bytes()
        .deserialize(bytes)
        .map_err(|e| format!("PLONK proof decoding: {e}"))
}
#[derive(Serialize, Deserialize)]
struct Proof {
    witness: Commitment,
    gate: gates::Proof,
    copy: copy::Proof,
    gate_nonces: Vec<u64>,
    copy_nonces: Vec<u64>,
    opening: mikros::packed_opening::Proof,
    opening_nonces: Vec<u64>,
    witness_pcs: Vec<u8>,
    public_pcs: Vec<u8>,
}
#[derive(Default, Serialize)]
struct Times {
    commit_ms: f64,
    gates_ms: f64,
    copy_prepare_ms: f64,
    copy_ms: f64,
    packed_opening_ms: f64,
    witness_pcs_open_ms: f64,
    public_pcs_open_ms: f64,
    serialization_ms: f64,
    release_ms: f64,
    prover_ms: f64,
    witness_claims: usize,
    public_claims: usize,
    pcs_claims: usize,
}
#[derive(Default, Serialize)]
struct VerifyTimes {
    gate_ms: f64,
    copy_ms: f64,
    opening_ms: f64,
    witness_pcs_ms: f64,
    public_pcs_ms: f64,
}
#[derive(Default, Serialize)]
struct GrindingTimes {
    gate_ms: f64,
    copy_ms: f64,
    copy_initial_ms: f64,
    opening_ms: f64,
    total_ms: f64,
}
fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}
fn event(name: &str) {
    eprintln!("stage={name}");
    io::stderr().flush().unwrap();
}
fn clear_scratch() {
    flock_core::scratch::clear();
    mikros_permutation::tree_pool::clear_all();
}
fn peak_rss_kib() -> usize {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines().find_map(|l| {
                l.strip_prefix("VmHWM:")
                    .and_then(|v| v.split_whitespace().next()?.parse().ok())
            })
        })
        .unwrap_or(0)
}
fn merge(mut a: Claims, b: Claims) -> Claims {
    a.witness.extend(b.witness);
    a.public.extend(b.public);
    a
}
fn bind(
    c: &Circuit,
    w: &Commitment,
    p: &Commitment,
    wp: &VerifierKey,
    pp: &VerifierKey,
) -> FsChallenger {
    let mut ch = FsChallenger::new(backend::TRANSCRIPT_DOMAIN);
    ch.observe_bytes(&c.digest());
    ch.observe_bytes(&wp.parameter_digest());
    ch.observe_bytes(&pp.parameter_digest());
    ch.observe_bytes(&bincode::serialize(w).unwrap());
    ch.observe_bytes(&bincode::serialize(p).unwrap());
    ch
}

fn copy_challenger(c: &Circuit, challenger: GrindingChallenger) -> GrindingChallenger {
    if c.tag_bits() <= 64 {
        challenger.packed_permutation(c.h(), c.copy_kappa())
    } else {
        // Ordinary Product-GKR has no packed-column challenges. Keep the
        // existing degree-dependent fingerprint and cubic-tail work policy.
        // F256 fingerprints use one double-width challenge-vector event.
        challenger.permutation(c.h())
    }
}

struct Setup {
    circuit: Circuit,
    witness: Vec<F128>,
    public: Vec<F128>,
    witness_source: backend::Source,
    public_source: backend::Source,
    public_cm: Commitment,
    public_data: Option<backend::Cache>,
    offline_ms: f64,
}
impl Setup {
    fn new(c: Circuit) -> Result<Self> {
        let start = Instant::now();
        event("offline_public_tables");
        hadamard::warmup(c.value_bits)?;
        copy::warmup(&c)?;
        event("offline_witness_generation");
        let witness = c.witness();
        let public = c.public();
        event(backend::OFFLINE_SETUP_STAGE);
        // Reproducible public code sampling, independent of witness values.
        let witness_source =
            backend::Source::new(witness.len(), 0x5749544e455353 ^ c.log_gates as u64)?;
        let public_source =
            backend::Source::new(public.len(), 0x5055424c4943 ^ c.log_gates as u64)?;
        event("offline_public_commitment");
        let (public_cm, public_data) = public_source.commit(&public)?;
        clear_scratch();
        Ok(Self {
            circuit: c,
            witness,
            public,
            witness_source,
            public_source,
            public_cm,
            public_data: Some(public_data),
            offline_ms: ms(start),
        })
    }

    fn prepare_public_cache(&mut self) -> Result<()> {
        if self.public_data.is_none() {
            event("offline_rebuild_public_cache");
            let (cm, cache) = self.public_source.commit(&self.public)?;
            if cm != self.public_cm {
                return Err("fixed public commitment changed".into());
            }
            self.public_data = Some(cache);
        }
        Ok(())
    }
}

fn prove(s: &mut Setup) -> Result<(Proof, FsChallenger, Times, GrindingTimes)> {
    let start = Instant::now();
    let c = &s.circuit;
    let mut times = Times::default();
    let mut grinding = GrindingTimes::default();
    let public_data = s.public_data.take().ok_or("missing offline public cache")?;
    event("online_witness_commitment");
    let t = Instant::now();
    let z = &s.witness;
    let (witness, wd) = s.witness_source.commit(z)?;
    times.commit_ms = ms(t);
    let mut parent = bind(
        c,
        &witness,
        &s.public_cm,
        s.witness_source.verifier_key(),
        s.public_source.verifier_key(),
    );
    let gc = parent.fork(b"gates");
    let cc = parent.fork(b"copy");
    event("online_gate_piop");
    let t = Instant::now();
    let mut g = GrindingChallenger::prover(gc);
    let (gate, gclaims) = gates::prove(c, &z, &s.public, &mut g)?;
    grinding.gate_ms = g.grinding_ms();
    let (gc, gate_nonces) = g.finish()?;
    times.gates_ms = ms(t);
    let t = Instant::now();
    clear_scratch();
    times.release_ms += ms(t);
    event("online_copy_prepare");
    let t = Instant::now();
    let prepared = copy::prepare(c, &z)?;
    times.copy_prepare_ms = ms(t);
    event("online_copy_piop");
    let t = Instant::now();
    let mut cc = copy_challenger(c, GrindingChallenger::prover(cc));
    let (copy, cclaims) = copy::prove(c, &z, &s.public, prepared, &mut cc)?;
    grinding.copy_ms = cc.grinding_ms();
    grinding.copy_initial_ms = cc.initial_fingerprint_grinding_ms();
    let (cc, copy_nonces) = cc.finish()?;
    times.copy_ms = ms(t);
    let t = Instant::now();
    clear_scratch();
    times.release_ms += ms(t);
    let claims = merge(gclaims, cclaims);
    times.witness_claims = claims.witness.len();
    times.public_claims = claims.public.len();
    parent.merge_child(gc);
    parent.merge_child(cc);
    event("online_opening_reduction");
    let t = Instant::now();
    let mut och = GrindingChallenger::prover(parent);
    let (opening, terminals) = mikros::packed_opening::prove(
        &[(&z, &claims.witness), (&s.public, &claims.public)],
        &mut och,
    )?;
    grinding.opening_ms = och.grinding_ms();
    grinding.total_ms = grinding.gate_ms + grinding.copy_ms + grinding.opening_ms;
    let (mut parent, opening_nonces) = och.finish()?;
    times.packed_opening_ms = ms(t);
    times.pcs_claims = terminals.len();
    if terminals.len() != 2 {
        return Err("expected one PCS terminal per source group".into());
    }
    event("online_witness_pcs_open");
    let t = Instant::now();
    let witness_pcs = s.witness_source.open(&witness, &terminals[0], wd)?;
    times.witness_pcs_open_ms = ms(t);
    parent.observe_label(backend::WITNESS_PROOF_LABEL);
    parent.observe_bytes(&witness_pcs);
    let t = Instant::now();
    clear_scratch();
    times.release_ms += ms(t);
    event("online_public_pcs_open");
    let t = Instant::now();
    let public_pcs = s
        .public_source
        .open(&s.public_cm, &terminals[1], public_data)?;
    times.public_pcs_open_ms = ms(t);
    parent.observe_label(backend::PUBLIC_PROOF_LABEL);
    parent.observe_bytes(&public_pcs);
    let t = Instant::now();
    clear_scratch();
    times.release_ms += ms(t);
    let p = Proof {
        witness,
        gate,
        copy,
        gate_nonces,
        copy_nonces,
        opening,
        opening_nonces,
        witness_pcs,
        public_pcs,
    };
    times.prover_ms = ms(start);
    Ok((p, parent, times, grinding))
}

fn verify(
    c: &Circuit,
    public_cm: &Commitment,
    witness_key: &VerifierKey,
    public_key: &VerifierKey,
    p: &Proof,
) -> Result<(FsChallenger, VerifyTimes)> {
    // No witness or mutable prover state is passed to this verifier.
    c.validate()?;
    let log_z = (4 * c.wire_words()).max(1 << 15).ilog2() as usize;
    let log_public = c.public_words().ilog2() as usize;
    if witness_key.words() != 1usize << log_z || public_key.words() != 1usize << log_public {
        return Err("PCS verification key dimensions".into());
    }
    let mut parent = bind(c, &p.witness, public_cm, witness_key, public_key);
    let gc = parent.fork(b"gates");
    let cc = parent.fork(b"copy");
    let mut times = VerifyTimes::default();
    let t = Instant::now();
    let mut g = GrindingChallenger::verifier(gc, &p.gate_nonces);
    let gclaims = gates::verify(c, log_z, log_public, &p.gate, &mut g)?;
    let (gc, _) = g.finish()?;
    times.gate_ms = ms(t);
    let t = Instant::now();
    let mut cc = copy_challenger(c, GrindingChallenger::verifier(cc, &p.copy_nonces));
    let cclaims = copy::verify(c, log_z, log_public, &p.copy, &mut cc)?;
    let (cc, _) = cc.finish()?;
    times.copy_ms = ms(t);
    let claims = merge(gclaims, cclaims);
    parent.merge_child(gc);
    parent.merge_child(cc);
    let t = Instant::now();
    let mut och = GrindingChallenger::verifier(parent, &p.opening_nonces);
    let terminals = mikros::packed_opening::verify(
        &[log_z, log_public],
        &[&claims.witness, &claims.public],
        &p.opening,
        &mut och,
    )?;
    let (mut parent, _) = och.finish()?;
    times.opening_ms = ms(t);
    if terminals.len() != 2 {
        return Err("PCS terminal shape".into());
    }
    let t = Instant::now();
    backend::verify(witness_key, &p.witness, &terminals[0], &p.witness_pcs)?;
    parent.observe_label(backend::WITNESS_PROOF_LABEL);
    parent.observe_bytes(&p.witness_pcs);
    times.witness_pcs_ms = ms(t);
    let t = Instant::now();
    backend::verify(public_key, public_cm, &terminals[1], &p.public_pcs)?;
    parent.observe_label(backend::PUBLIC_PROOF_LABEL);
    parent.observe_bytes(&p.public_pcs);
    times.public_pcs_ms = ms(t);
    Ok((parent, times))
}

fn emit(v: Value) -> Result<()> {
    println!("{v}");
    io::stdout().flush().map_err(|e| e.to_string())
}

// This is reporting metadata only. Each protocol keeps its original statement
// digest and transcript domain; the equality is covered by fixture tests below.
fn comparison_fixture(c: &Circuit) -> Result<old_fixture::Circuit> {
    if c.value_bits != 1 {
        return Err("binary comparison requires one-bit witness values".into());
    }
    // The shared near-full statement is already available after offline witness
    // generation. Construct only its public descriptor here, without replaying
    // the sequential circuit a second time.
    let old = if c.near_full {
        old_fixture::Circuit {
            log_gates: c.log_gates,
            mode: old_fixture::Mode::Single,
            seed: c.seed,
            io: F128::new(c.io as u64, (c.io >> 64) as u64),
            near_full: true,
        }
    } else {
        old_fixture::Circuit::new(c.log_gates, old_fixture::Mode::Single, c.seed)
    };
    if old.io != F128::new(c.io as u64, (c.io >> 64) as u64) {
        return Err("binary comparison public output mismatch".into());
    }
    Ok(old)
}

fn constraint_breakdown(
    proof: &Proof,
    proof_bytes: usize,
    times: &Times,
    verifier: &VerifyTimes,
    verify_ms: f64,
) -> Result<Value> {
    let gate_bytes = bincode::serialized_size(&(&proof.gate, &proof.gate_nonces))
        .map_err(|e| e.to_string())? as usize;
    let copy_bytes = bincode::serialized_size(&(&proof.copy, &proof.copy_nonces, &proof.public_pcs))
        .map_err(|e| e.to_string())? as usize;
    let shared_bytes = proof_bytes
        .checked_sub(gate_bytes + copy_bytes)
        .ok_or("component proof sizes exceed complete proof")?;
    let gate_ms = times.gates_ms;
    let copy_ms = times.copy_prepare_ms + times.copy_ms + times.public_pcs_open_ms;
    let copy_verify_ms = verifier.copy_ms + verifier.public_pcs_ms;
    let shared_ms = times.prover_ms - gate_ms - copy_ms;
    let shared_verify_ms = verify_ms - verifier.gate_ms - copy_verify_ms;
    if shared_ms < 0.0 || shared_verify_ms < 0.0 || shared_bytes == 0 {
        return Err("invalid complete proof component accounting".into());
    }
    Ok(json!({
        "gate":{"prover_ms":gate_ms,"verifier_ms":verifier.gate_ms,"proof_bytes":gate_bytes},
        "copy":{"prover_ms":copy_ms,"verifier_ms":copy_verify_ms,"proof_bytes":copy_bytes},
        "shared":{"prover_ms":shared_ms,"verifier_ms":shared_verify_ms,"proof_bytes":shared_bytes}
    }))
}

pub fn run_with_args(args: Vec<String>) -> Result<()> {
    let mut log = backend::DEFAULT_LOG_GATES;
    let mut bits = 1;
    let mut threads = 64;
    let mut reps = 1usize;
    let mut warmups = 0usize;
    let mut seed = 97u64;
    let mut near_full = false;
    let mut args = args.iter();
    while let Some(a) = args.next() {
        let v = args
            .next()
            .ok_or_else(|| format!("missing value for {a}"))?;
        match a.as_str() {
            "--circuit" => {
                near_full = match v.as_str() {
                    "paired" => false,
                    "near-full" => true,
                    _ => return Err(format!("unknown circuit {v}")),
                }
            }
            "--bits" => bits = v.parse().map_err(|_| "invalid witness bitwidth")?,
            "--log-gates" => log = v.parse().map_err(|_| "invalid log gates")?,
            "--threads" => threads = v.parse().map_err(|_| "invalid threads")?,
            "--repetitions" => reps = v.parse().map_err(|_| "invalid repetitions")?,
            "--warmups" => warmups = v.parse().map_err(|_| "invalid warmups")?,
            "--seed" => seed = v.parse().map_err(|_| "invalid seed")?,
            _ => return Err(format!("unknown argument {a}")),
        }
    }
    if threads == 0 || threads > 256 || reps == 0 || reps > 100 || warmups > 10 {
        return Err("invalid sampling configuration".into());
    }
    if std::env::var_os("FLOCK_NO_GRIND").is_some() {
        return Err("FLOCK_NO_GRIND is unsupported".into());
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .stack_size(8 << 20)
        .build_global()
        .map_err(|e| e.to_string())?;
    if !flock_parallel::init_all_core_pool(threads) {
        return Err("all-core pool already initialized".into());
    }
    let c = if near_full {
        Circuit::new_near_full(log, bits, seed)?
    } else {
        Circuit::new(log, bits, seed)?
    };
    let mut setup = Setup::new(c)?;
    let c = setup.circuit.clone();
    let comparison = if bits == 1 {
        Some(comparison_fixture(&c)?)
    } else {
        None
    };
    let mut metadata = json!({"type":"metadata","system":"mikros","mode":"single","arithmetization":"PLONK",
        "value_bits":bits,"base_field_bits":bits,"field_semantics":"GF(2^b); ADD is field addition, MUL is field multiplication",
        "kappa":c.kappa(),"tag_bits":c.tag_bits(),"copy_kappa":c.copy_kappa(),"log_gates":log,
        "copy_field_bits":c.copy_field_bits(),"copy_mode":c.copy_mode(),
        "instances":1,"gates_per_instance":c.genuine_gate_count(),"total_gates":c.genuine_gate_count(),"threads":threads,
        "all_core_threads":flock_parallel::all_core_pool().current_num_threads(),"seed":seed,
        "repetitions":reps,"warmups":warmups,"pcs_backend":backend::BACKEND_NAME,
        "witness_words":setup.witness.len(),"public_words":setup.public.len(),
        "pcs_profile":backend::profile(),
        "witness_pcs_parameters":setup.witness_source.verifier_key().parameter_profile(),
        "public_pcs_parameters":setup.public_source.verifier_key().parameter_profile(),
        "fixture_digest":blake3::Hash::from(c.digest()).to_hex().to_string(),
        "public_io":[c.io as u64,(c.io >> 64) as u64],"genuine_gate_count":c.genuine_gate_count(),
        "circuit_layout":c.circuit_name(),"private_input_values":c.independent_input_count(),
        "private_input_bits":c.independent_input_count()*bits,
        "plonk_domain_size":c.n(),"copy_domain_size":c.copy_len(),
        "matrix_authentication":"preprocessed public PCS","permutation_proof":true,
        "component_scope":"Permutation argument includes online copy preparation, permutation proof and entire public PCS opening; combined packed-opening reduction and witness PCS counted once in Shared PCS/other",
        "target_features":{"gfni":cfg!(target_feature="gfni"),"avx512f":cfg!(target_feature="avx512f"),"avx512bw":cfg!(target_feature="avx512bw"),"vpclmulqdq":cfg!(target_feature="vpclmulqdq")},
        "copy_storage":if c.tag_bits()<=64 {"bounded_compact"} else {"bounded_dense"},"witness_bits_unpadded":3*c.n()*bits,
        "piop_grinding":"8-bit gate/opening; degree-dependent fingerprint and 2-bit cubic tail; packed-column filters only for packed copy"});
    // Only the binary circuit has the existing native-FLOCK comparison
    // descriptor. Wider fields retain their own statement and public output.
    if let Some(comparison) = comparison {
        metadata["comparison_fixture_digest"] =
            json!(blake3::Hash::from(comparison.digest()).to_hex().to_string());
    }
    emit(metadata)?;
    for run in 0..warmups + reps {
        let warmup = run < warmups;
        let repetition = if warmup { run } else { run - warmups };
        eprintln!(
            "[PLONK {}] b={bits} {} {}/{} start ({} genuine gates; 2^{log} padded rows)",
            backend::DISPLAY_NAME,
            if warmup { "warmup" } else { "sample" },
            repetition + 1,
            if warmup { warmups } else { reps },
            c.genuine_gate_count()
        );
        setup.prepare_public_cache()?;
        let (proof, mut pc, mut times, grinding) = prove(&mut setup)?;
        let t = Instant::now();
        let encoded = bincode::serialize(&proof).map_err(|e| e.to_string())?;
        times.serialization_ms = ms(t);
        times.prover_ms += times.serialization_ms;
        let rss = peak_rss_kib();
        event("verify_full_proof");
        let t = Instant::now();
        let decoded = decode_proof(&encoded)?;
        let (mut vc, verify_times) = verify(
            &c,
            &setup.public_cm,
            setup.witness_source.verifier_key(),
            setup.public_source.verifier_key(),
            &decoded,
        )?;
        let verify_ms = ms(t);
        if pc.sample_f128_vec(4) != vc.sample_f128_vec(4) {
            return Err("complete proof transcript mismatch".into());
        }
        let breakdown =
            constraint_breakdown(&proof, encoded.len(), &times, &verify_times, verify_ms)?;
        eprintln!(
            "[PLONK {}] b={bits} {} {}/{} finished: prover={:.3} ms verifier={verify_ms:.3} ms proof={} bytes",
            backend::DISPLAY_NAME,
            if warmup { "warmup" } else { "sample" },
            repetition + 1,
            if warmup { warmups } else { reps },
            times.prover_ms,
            encoded.len()
        );
        let mut sample = json!({"type":"sample","value_bits":bits,"repetition":repetition,"warmup":warmup,
            "circuit_layout":c.circuit_name(),"genuine_gate_count":c.genuine_gate_count(),
            "gates_per_instance":c.genuine_gate_count(),"total_gates":c.genuine_gate_count(),
            "private_input_values":c.independent_input_count(),"private_input_bits":c.independent_input_count()*bits,
            "plonk_domain_size":c.n(),"copy_domain_size":c.copy_len(),
            "verified":true,"transcript_match":true,"proof_bytes":encoded.len(),"verify_ms":verify_ms,
            "peak_rss_kib":rss,"offline_ms":setup.offline_ms,"times":times,"verifier_times":verify_times,
            "piop_grinding_ms":grinding,"proof_blake3":blake3::hash(&encoded).to_hex().to_string(),
            "pcs_complete":true,"constraint_breakdown":breakdown,
            "witness_pcs_proof_bytes":proof.witness_pcs.len(),"public_pcs_proof_bytes":proof.public_pcs.len()});
        if let Some(certified) = backend::DISTANCE_CERTIFIED {
            sample["distance_certified"] = json!(certified);
        }
        emit(sample)?;
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn binary_comparison_fixture_matches_values_wiring_and_public_output() {
        for seed in [0, 97, u64::MAX] {
            let c = Circuit::new(14, 1, seed).unwrap();
            let old = comparison_fixture(&c).unwrap();
            assert_eq!(c.witness(), old.witness());
            for index in 0..4 * c.n() {
                assert_eq!(c.sigma(index), old.sigma(index));
            }
            for gate in 0..c.n() {
                for wire in 0..3 {
                    assert_eq!(c.value(wire, gate), (old.word(wire, gate).lo & 1) as u128);
                }
                for selector in 0..5 {
                    assert_eq!(
                        Circuit::selector(selector, gate),
                        old_fixture::Circuit::selector(selector, gate)
                    );
                }
            }
        }
        assert!(comparison_fixture(&Circuit::new(14, 2, 97).unwrap()).is_err());
        let mut bad = Circuit::new(14, 1, 97).unwrap();
        bad.io ^= 1;
        assert!(comparison_fixture(&bad).is_err());
    }
    fn check(s: &Setup, c: &Circuit, p: &Proof) -> Result<(FsChallenger, VerifyTimes)> {
        verify(
            c,
            &s.public_cm,
            s.witness_source.verifier_key(),
            s.public_source.verifier_key(),
            p,
        )
    }
    #[test]
    fn near_full_binary_plonk_complete_proof_and_boundary_tampering() {
        let mut s = Setup::new(Circuit::new_near_full(14, 1, 97).unwrap()).unwrap();
        let c = &s.circuit;
        assert_eq!(c.genuine_gate_count(), (1 << 14) - 257);
        assert_eq!(c.independent_input_count(), 256);
        assert_eq!(c.r1cs_rows(), c.n());
        assert_eq!(c.copy_len(), 4 * c.n());
        let descriptor = comparison_fixture(c).unwrap();
        assert!(descriptor.near_full);
        assert_eq!(descriptor.io, F128::new(c.io as u64, 0));
        assert_ne!(
            descriptor.digest(),
            old_fixture::Circuit::new(14, old_fixture::Mode::Single, 97).digest()
        );

        // The final real gate and the first padding row are distinct. Both
        // constrain their output wire, and qC is authenticated for both rows.
        let flip = |words: &mut [F128], index: usize| {
            words[index / 128] += fixture::from_raw(1u128 << (index % 128));
        };
        for gate in [c.output_gate(), c.genuine_gate_count()] {
            let mut witness = s.witness.clone();
            flip(&mut witness, 2 * c.n() + gate);
            assert!(
                gates::prove(
                    c,
                    &witness,
                    &s.public,
                    &mut FsChallenger::new(b"near-full-invalid-output-test")
                )
                .is_err()
            );
            let mut public = s.public.clone();
            flip(&mut public, 4 * c.n() + gate);
            assert!(
                gates::prove(
                    c,
                    &s.witness,
                    &public,
                    &mut FsChallenger::new(b"near-full-invalid-selector-test")
                )
                .is_err()
            );
        }

        let (proof, mut pc, _, _) = prove(&mut s).unwrap();
        let encoded = bincode::serialize(&proof).unwrap();
        let fresh = || decode_proof(&encoded).unwrap();
        let (mut vc, _) = check(&s, &s.circuit, &fresh()).unwrap();
        assert_eq!(pc.sample_f128_vec(4), vc.sample_f128_vec(4));
        let mut wrong_output = s.circuit.clone();
        wrong_output.io ^= 1;
        assert!(check(&s, &wrong_output, &proof).is_err());
        let mut wrong_layout = s.circuit.clone();
        wrong_layout.near_full = false;
        assert!(check(&s, &wrong_layout, &proof).is_err());
        let mut bad = fresh();
        bad.opening.columns[0][0] += F128::ONE;
        assert!(check(&s, &s.circuit, &bad).is_err());
        for group in 0..2 {
            let mut bad = fresh();
            let bytes = if group == 0 {
                &mut bad.witness_pcs
            } else {
                &mut bad.public_pcs
            };
            let last = bytes.len() - 1;
            bytes[last] ^= 1;
            assert!(check(&s, &s.circuit, &bad).is_err());
        }
    }
    #[test]
    fn binary_plonk_complete_proof_and_tampering() {
        let mut s = Setup::new(Circuit::new(14, 1, 97).unwrap()).unwrap();
        let (proof, mut pc, times, _) = prove(&mut s).unwrap();
        let encoded = bincode::serialize(&proof).unwrap();
        let decoded = decode_proof(&encoded).unwrap();
        let start = Instant::now();
        let (mut vc, verifier_times) = check(&s, &s.circuit, &decoded).unwrap();
        let verify_ms = ms(start);
        let parts = constraint_breakdown(&proof, encoded.len(), &times, &verifier_times, verify_ms)
            .unwrap();
        assert_eq!(
            ["gate", "copy", "shared"]
                .iter()
                .map(|key| parts[key]["proof_bytes"].as_u64().unwrap())
                .sum::<u64>(),
            encoded.len() as u64
        );
        assert_eq!(
            parts["copy"]["prover_ms"],
            json!(times.copy_prepare_ms + times.copy_ms + times.public_pcs_open_ms)
        );
        assert_eq!(pc.sample_f128_vec(4), vc.sample_f128_vec(4));
        let fresh = || decode_proof(&encoded).unwrap();
        let mut trailing = encoded.clone();
        trailing.push(0);
        assert!(decode_proof(&trailing).is_err());
        assert!(decode_proof(&encoded[..encoded.len() - 1]).is_err());
        let mut oversized = encoded.clone();
        let length_offset = encoded.len() - proof.public_pcs.len() - 8;
        oversized[length_offset..length_offset + 8].copy_from_slice(&u64::MAX.to_le_bytes());
        assert!(decode_proof(&oversized).is_err());
        let mut circuit = s.circuit.clone();
        circuit.io ^= 1;
        assert!(check(&s, &circuit, &proof).is_err());
        let mut bad = fresh();
        bad.gate.scalars[0] += F128::ONE;
        assert!(check(&s, &s.circuit, &bad).is_err());
        let mut bad = fresh();
        bad.opening.columns[0][0] += F128::ONE;
        assert!(check(&s, &s.circuit, &bad).is_err());
        let mut bad = fresh();
        bad.gate_nonces[0] ^= 1;
        assert!(check(&s, &s.circuit, &bad).is_err());
        for group in 0..2 {
            let mut bad = fresh();
            let bytes = if group == 0 {
                &mut bad.witness_pcs
            } else {
                &mut bad.public_pcs
            };
            bytes.pop();
            assert!(check(&s, &s.circuit, &bad).is_err());
            let mut bad = fresh();
            let bytes = if group == 0 {
                &mut bad.witness_pcs
            } else {
                &mut bad.public_pcs
            };
            bytes.push(0);
            assert!(check(&s, &s.circuit, &bad).is_err());
            let mut bad = fresh();
            let bytes = if group == 0 {
                &mut bad.witness_pcs
            } else {
                &mut bad.public_pcs
            };
            let last = bytes.len() - 1;
            bytes[last] ^= 1;
            assert!(check(&s, &s.circuit, &bad).is_err());
        }
        let mut other = Setup::new(Circuit::new(14, 1, 101).unwrap()).unwrap();
        let (other_proof, _, _, _) = prove(&mut other).unwrap();
        let mut bad = fresh();
        bad.witness_pcs = other_proof.witness_pcs;
        assert!(check(&s, &s.circuit, &bad).is_err());
        let mut bad = fresh();
        bad.witness = other_proof.witness;
        assert!(check(&s, &s.circuit, &bad).is_err());
    }

    #[test]
    fn all_field_widths_complete_proof_and_tampering() {
        for bits in [1, 2, 4, 8, 16, 32, 64, 128] {
            let c = Circuit::new(14, bits, 97).unwrap();
            let mut s = Setup::new(c.clone()).unwrap();
            let (proof, mut pc, times, _) =
                prove(&mut s).unwrap_or_else(|error| panic!("b={bits}: {error}"));
            assert_eq!(times.witness_claims, 11, "b={bits}");
            assert_eq!(times.public_claims, 5 + 3 * c.label_digits(), "b={bits}");
            assert_eq!(times.pcs_claims, 2, "b={bits}");
            assert_eq!(s.witness_source.verifier_key().words(), c.witness_words());
            assert_eq!(s.public_source.verifier_key().words(), c.public_words());
            match &proof.copy {
                copy::Proof::Packed(_) => assert!(c.tag_bits() <= 64),
                copy::Proof::Native128(_) => assert_eq!(c.tag_bits(), 128),
                copy::Proof::Wide256(_) => {
                    assert_eq!(bits, 128);
                    assert_eq!(c.tag_bits(), 256);
                }
            }
            let encoded = bincode::serialize(&proof).unwrap();
            let fresh = || decode_proof(&encoded).unwrap();
            let (mut vc, _) =
                check(&s, &c, &fresh()).unwrap_or_else(|error| panic!("b={bits}: {error}"));
            assert_eq!(pc.sample_f128_vec(4), vc.sample_f128_vec(4), "b={bits}");

            // Public outputs bind every value bit, including the top bit of
            // F128; no scalar comparison to the binary fixture is involved.
            let mut wrong_output = c.clone();
            wrong_output.io ^= 1u128 << (bits - 1);
            assert!(check(&s, &wrong_output, &proof).is_err(), "b={bits}: IO");

            let mut bad = fresh();
            bad.gate.scalars[0] += F128::new(0, 1 << 63);
            assert!(check(&s, &c, &bad).is_err(), "b={bits}: gate");

            let mut bad = fresh();
            match &mut bad.copy {
                copy::Proof::Packed(p) => p.wire_values[0][0] += F128::new(0, 1 << 63),
                copy::Proof::Native128(p) => p.wire_values[0][0] += F128::new(0, 1 << 63),
                copy::Proof::Wide256(p) => {
                    // The high F256 coordinate contains the witness value.
                    // This checks that its top bit reaches the F128 PCS claims.
                    p.wire_values[0][0] +=
                        flock_field::F256::new(F128::ZERO, F128::new(0, 1 << 63));
                }
            }
            assert!(check(&s, &c, &bad).is_err(), "b={bits}: copy");

            let mut bad = fresh();
            bad.opening.columns[0][127] += F128::ONE;
            assert!(check(&s, &c, &bad).is_err(), "b={bits}: packed opening");

            for group in 0..2 {
                let mut bad = fresh();
                let bytes = if group == 0 {
                    &mut bad.witness_pcs
                } else {
                    &mut bad.public_pcs
                };
                assert!(!bytes.is_empty());
                let last = bytes.len() - 1;
                bytes[last] ^= 1;
                assert!(check(&s, &c, &bad).is_err(), "b={bits}: PCS group {group}");
            }
            assert_eq!(Circuit::new_near_full(14, bits, 97).is_ok(), bits == 1);
        }
    }
}
