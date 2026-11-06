//! Complete single-instance PLONK proofs over actual F_{2^b} witness fields.
#![recursion_limit = "256"]
#![allow(dead_code)]
#[path = "e2e_plonk_bitwidth/copy.rs"]
mod copy;
#[path = "e2e_plonk_bitwidth/fixture.rs"]
mod fixture;
#[path = "e2e_plonk_bitwidth/gates.rs"]
mod gates;
#[path = "e2e_common/grinding.rs"]
mod grinding;
#[path = "e2e_plonk_bitwidth/hadamard.rs"]
mod hadamard;
#[path = "e2e_plonk/fixture.rs"]
mod old_fixture;
#[path = "e2e_plonk/queries.rs"]
mod queries;

use fixture::{Circuit, Claims};
use flock_core::{
    challenger::{Challenger, FsChallenger},
    pcs::{self, Commitment, ProverData, merged_tensor},
    schedule::Registry,
};
use flock_field::F128;
use grinding::GrindingChallenger;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    io::{self, Write},
    time::Instant,
};

type Result<T> = std::result::Result<T, String>;
#[derive(Serialize, Deserialize)]
struct Proof {
    witness: Commitment,
    gate: gates::Proof,
    copy: copy::Proof,
    gate_nonces: Vec<u64>,
    copy_nonces: Vec<u64>,
    opening: mikros::packed_opening::Proof,
    opening_nonces: Vec<u64>,
    witness_pcs: merged_tensor::Proof,
    public_pcs: merged_tensor::Proof,
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
    native_pcs_claims: usize,
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
fn bind(c: &Circuit, w: &Commitment, p: &Commitment) -> FsChallenger {
    let mut ch = FsChallenger::new(b"mikros-plonk-field-bitwidth-native-pcs-v1");
    ch.observe_bytes(&c.digest());
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
    witness_registry: Registry,
    public_registry: Registry,
    public_cm: Commitment,
    public_data: ProverData,
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
        let wr = old_fixture::registry(
            witness.len().ilog2() as usize,
            c.wire_words().ilog2() as usize,
        );
        let pr = old_fixture::registry(
            public.len().ilog2() as usize,
            c.wire_words().ilog2() as usize,
        );
        let wp = old_fixture::parameters(&old_fixture::union(&wr));
        let pp = old_fixture::parameters(&old_fixture::union(&pr));
        wp.ligerito_prover_config()
            .map_err(|e| format!("witness PCS config: {e:?}"))?;
        pp.ligerito_prover_config()
            .map_err(|e| format!("public PCS config: {e:?}"))?;
        event("offline_public_commitment");
        let (public_cm, public_data) = pcs::commit(&public, &pp);
        clear_scratch();
        Ok(Self {
            circuit: c,
            witness,
            public,
            witness_registry: wr,
            public_registry: pr,
            public_cm,
            public_data,
            offline_ms: ms(start),
        })
    }
}

fn prove(s: &Setup) -> Result<(Proof, FsChallenger, Times, GrindingTimes)> {
    let start = Instant::now();
    let c = &s.circuit;
    let mut times = Times::default();
    let mut grinding = GrindingTimes::default();
    let wu = old_fixture::union(&s.witness_registry);
    let pu = old_fixture::union(&s.public_registry);
    event("online_witness_commitment");
    let t = Instant::now();
    // The PCS consumes its source; duplicating the witness remains online.
    let z = s.witness.clone();
    let (witness, wd) = pcs::commit(&z, &old_fixture::parameters(&wu));
    times.commit_ms = ms(t);
    let mut parent = bind(c, &witness, &s.public_cm);
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
    times.native_pcs_claims = terminals.len();
    if terminals.len() != 2 {
        return Err("expected one PCS terminal per source group".into());
    }
    event("online_witness_pcs_open");
    let t = Instant::now();
    let witness_pcs =
        merged_tensor::open_identity(z, &wd, &witness, &terminals[..1], &wu, &mut parent)
            .map_err(|e| format!("witness PCS open: {e:?}"))?;
    times.witness_pcs_open_ms = ms(t);
    let t = Instant::now();
    drop(wd);
    clear_scratch();
    times.release_ms += ms(t);
    event("online_public_pcs_open");
    let t = Instant::now();
    let public_pcs = merged_tensor::open_identity(
        s.public.clone(),
        &s.public_data,
        &s.public_cm,
        &terminals[1..],
        &pu,
        &mut parent,
    )
    .map_err(|e| format!("public PCS open: {e:?}"))?;
    times.public_pcs_open_ms = ms(t);
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

fn verify(c: &Circuit, public_cm: &Commitment, p: &Proof) -> Result<(FsChallenger, VerifyTimes)> {
    // No witness or mutable prover state is passed to this verifier.
    c.validate()?;
    let log_z = (4 * c.wire_words()).max(1 << 15).ilog2() as usize;
    let log_public = c.public_words().ilog2() as usize;
    let wr = old_fixture::registry(log_z, c.wire_words().ilog2() as usize);
    let pr = old_fixture::registry(log_public, c.wire_words().ilog2() as usize);
    let wu = old_fixture::union(&wr);
    let pu = old_fixture::union(&pr);
    if p.witness.params != old_fixture::parameters(&wu)
        || public_cm.params != old_fixture::parameters(&pu)
    {
        return Err("PCS configuration mismatch".into());
    }
    let mut parent = bind(c, &p.witness, public_cm);
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
    merged_tensor::verify(
        &p.witness,
        &terminals[..1],
        &p.witness_pcs,
        &wu,
        &mut parent,
    )
    .map_err(|e| format!("witness PCS replay: {e:?}"))?;
    times.witness_pcs_ms = ms(t);
    let t = Instant::now();
    merged_tensor::verify(public_cm, &terminals[1..], &p.public_pcs, &pu, &mut parent)
        .map_err(|e| format!("public PCS replay: {e:?}"))?;
    times.public_pcs_ms = ms(t);
    Ok((parent, times))
}

fn emit(v: Value) -> Result<()> {
    println!("{v}");
    io::stdout().flush().map_err(|e| e.to_string())
}
pub fn run_with_args(args: Vec<String>) -> Result<()> {
    let mut log = 26;
    let mut bits = 1;
    let mut threads = 64;
    let mut reps = 1usize;
    let mut warmups = 0usize;
    let mut seed = 97u64;
    let mut args = args.iter();
    while let Some(a) = args.next() {
        let v = args
            .next()
            .ok_or_else(|| format!("missing value for {a}"))?;
        match a.as_str() {
            "--log-gates" => log = v.parse().map_err(|_| "invalid log gates")?,
            "--bits" => bits = v.parse().map_err(|_| "invalid bitwidth")?,
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
    let c = Circuit::new(log, bits, seed)?;
    let setup = Setup::new(c)?;
    let c = &setup.circuit;
    emit(
        json!({"type":"metadata","system":"mikros","mode":"single","arithmetization":"PLONK",
        "value_bits":bits,"base_field_bits":bits,"field_semantics":"GF(2^b); ADD is field addition, MUL is field multiplication",
        "kappa":c.kappa(),"tag_bits":c.tag_bits(),"copy_kappa":c.copy_kappa(),"log_gates":log,
        "copy_field_bits":c.copy_field_bits(),"copy_mode":c.copy_mode(),
        "instances":1,"gates_per_instance":c.n(),"total_gates":c.n(),"threads":threads,
        "all_core_threads":flock_parallel::all_core_pool().current_num_threads(),"seed":seed,
        "repetitions":reps,"warmups":warmups,"pcs_backend":"FLOCK native Ligerito Fast",
        "witness_words":setup.witness.len(),"public_words":setup.public.len(),
        "witness_pcs_params":old_fixture::parameters(&old_fixture::union(&setup.witness_registry)),
        "public_pcs_params":setup.public_cm.params,"fixture_digest":blake3::Hash::from(c.digest()).to_hex().to_string(),
        "target_features":{"gfni":cfg!(target_feature="gfni"),"avx512f":cfg!(target_feature="avx512f"),"avx512bw":cfg!(target_feature="avx512bw"),"vpclmulqdq":cfg!(target_feature="vpclmulqdq")},
        "copy_storage":if c.tag_bits()<=64 {"bounded_compact"} else {"bounded_dense"},"witness_bits_unpadded":3*c.n()*bits,
        "piop_grinding":"8-bit gate/opening; degree-dependent fingerprint and 2-bit cubic tail; packed-column filters only for packed copy"}),
    )?;
    for run in 0..warmups + reps {
        let warmup = run < warmups;
        let repetition = if warmup { run } else { run - warmups };
        eprintln!(
            "[PLONK bitwidth] b={bits} {} {}/{} start (2^{log} gates)",
            if warmup { "warmup" } else { "sample" },
            repetition + 1,
            if warmup { warmups } else { reps }
        );
        let (proof, mut pc, mut times, grinding) = prove(&setup)?;
        let t = Instant::now();
        let encoded = bincode::serialize(&proof).map_err(|e| e.to_string())?;
        times.serialization_ms = ms(t);
        times.prover_ms += times.serialization_ms;
        let rss = peak_rss_kib();
        event("verify_full_proof");
        let t = Instant::now();
        let decoded: Proof = bincode::deserialize(&encoded).map_err(|e| e.to_string())?;
        let (mut vc, verify_times) = verify(c, &setup.public_cm, &decoded)?;
        let verify_ms = ms(t);
        if pc.sample_f128_vec(4) != vc.sample_f128_vec(4) {
            return Err("complete proof transcript mismatch".into());
        }
        eprintln!(
            "[PLONK bitwidth] b={bits} {} {}/{} finished: prover={:.3} ms verifier={verify_ms:.3} ms proof={} bytes",
            if warmup { "warmup" } else { "sample" },
            repetition + 1,
            if warmup { warmups } else { reps },
            times.prover_ms,
            encoded.len()
        );
        emit(
            json!({"type":"sample","value_bits":bits,"repetition":repetition,"warmup":warmup,
            "verified":true,"transcript_match":true,"proof_bytes":encoded.len(),"verify_ms":verify_ms,
            "peak_rss_kib":rss,"offline_ms":setup.offline_ms,"times":times,"verifier_times":verify_times,
            "piop_grinding_ms":grinding,"proof_blake3":blake3::hash(&encoded).to_hex().to_string(),
            "witness_pcs_params":proof.witness.params,"public_pcs_params":setup.public_cm.params}),
        )?;
    }
    Ok(())
}
fn main() {
    if let Err(e) = run_with_args(std::env::args().skip(1).collect()) {
        eprintln!("PLONK bitwidth failed: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn complete_field_proofs_and_attack_rejection() {
        for bits in [1, 2, 4, 8, 16, 32, 64, 128] {
            let s = Setup::new(Circuit::new(14, bits, 97).unwrap()).unwrap();
            let (p, mut pc, _, _) = prove(&s).unwrap();
            let encoded = bincode::serialize(&p).unwrap();
            let (mut vc, _) = verify(&s.circuit, &s.public_cm, &p).unwrap();
            assert_eq!(pc.sample_f128_vec(4), vc.sample_f128_vec(4));
            let fresh = || bincode::deserialize::<Proof>(&encoded).unwrap();
            let mut changed = s.circuit.clone();
            changed.io ^= 1;
            assert!(verify(&changed, &s.public_cm, &p).is_err());
            let mut changed = s.circuit.clone();
            changed.value_bits = if bits == 1 { 2 } else { 1 };
            assert!(verify(&changed, &s.public_cm, &p).is_err());
            let mut bad = fresh();
            bad.gate.scalars[0] += F128::ONE;
            assert!(verify(&s.circuit, &s.public_cm, &bad).is_err());
            let mut bad = fresh();
            bad.opening.columns[0][0] += F128::ONE;
            assert!(verify(&s.circuit, &s.public_cm, &bad).is_err());
            let mut bad = fresh();
            bad.gate_nonces[0] ^= 1;
            assert!(verify(&s.circuit, &s.public_cm, &bad).is_err());
            let mut bad = fresh();
            bad.witness.params.m += 1;
            assert!(verify(&s.circuit, &s.public_cm, &bad).is_err());
            let other = Setup::new(Circuit::new(14, bits, 101).unwrap()).unwrap();
            let (different, _, _, _) = prove(&other).unwrap();
            let mut bad = fresh();
            bad.witness_pcs = different.witness_pcs;
            assert!(verify(&s.circuit, &s.public_cm, &bad).is_err());
        }
    }
}
