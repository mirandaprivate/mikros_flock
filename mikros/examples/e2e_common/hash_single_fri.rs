//! One native hash execution compiled to binary PLONK, with complete additive-FRI
//! authentication of the packed witness and public tables. Only the fixed public
//! labels and their commitment are prepared offline; all witness-dependent copy
//! preparation and both PCS openings are online. The consumed public commitment
//! cache is replenished offline before each proof and its root checked unchanged.

#[path = "grinding.rs"]
mod grinding;
#[path = "../e2e_plonk_fri/pcs.rs"]
mod pcs;

use super::{
    e2e_common::PreparedWitness,
    hash_fixture::{HashFixture, HashKind},
    hash_single_plonk::{copy, fixture, gates},
};
use flock_core::challenger::{Challenger, FsChallenger};
use flock_field::F128;
use grinding::GrindingChallenger;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::time::Instant;

pub use pcs::{Commitment, VerifierKey};
pub const BACKEND_NAME: &str = pcs::BACKEND_NAME;
const DOMAIN: &[u8] = b"mikros-single-native-hash-plonk-additive-fri-v1";

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}
fn digest<T: Serialize>(value: &T) -> String {
    blake3::hash(&bincode::serialize(value).expect("serialize receipt"))
        .to_hex()
        .to_string()
}
fn event(stage: &str) {
    eprintln!("stage={stage}");
}

pub struct Setup {
    pub circuit: fixture::Circuit,
    pub witness: Vec<F128>,
    pub public: Vec<F128>,
    pub public_commitment: Commitment,
    pub witness_vk: VerifierKey,
    pub public_vk: VerifierKey,
    pub offline_copy_setup_ms: f64,
    witness_source: pcs::Source,
    public_source: pcs::Source,
    public_data: Option<pcs::Cache>,
    copy_preprocessing: copy::Preprocessing,
}

impl Setup {
    pub fn new(f: &HashFixture, prepared: &PreparedWitness) -> Result<Self, String> {
        let (circuit, witness) = fixture::compile(f, prepared)?;
        Self::from_compiled(circuit, witness)
    }

    fn from_compiled(circuit: fixture::Circuit, witness: Vec<F128>) -> Result<Self, String> {
        circuit.validate_assignment(&witness)?;
        let public = circuit.public();
        let witness_source = pcs::Source::new(witness.len(), 0x5749544e455353)?;
        let public_source = pcs::Source::new(public.len(), 0x5055424c4943)?;
        let witness_vk = witness_source.verifier_key().clone();
        let public_vk = public_source.verifier_key().clone();
        event("offline_public_fri_commitment");
        let (public_commitment, public_data) = public_source.commit(&public)?;
        let t = Instant::now();
        let copy_preprocessing = copy::preprocess(&circuit)?;
        let offline_copy_setup_ms = ms(t);
        Ok(Self {
            circuit,
            witness,
            public,
            public_commitment,
            witness_vk,
            public_vk,
            offline_copy_setup_ms,
            witness_source,
            public_source,
            public_data: Some(public_data),
            copy_preprocessing,
        })
    }

    /// Replenish only fixed public preprocessing, outside the online timer.
    /// Source-dependent witness commitment and copy preparation are never cached.
    pub fn prepare_public_cache(&mut self) -> Result<(), String> {
        if self.public_data.is_none() {
            event("offline_rebuild_public_fri_cache");
            let (commitment, cache) = self.public_source.commit(&self.public)?;
            if commitment != self.public_commitment {
                return Err("fixed public FRI commitment changed".into());
            }
            self.public_data = Some(cache);
        }
        Ok(())
    }

    pub fn metadata(&self) -> Value {
        json!({
            "arithmetization":"binary PLONK", "pcs_backend":BACKEND_NAME,
            "pcs_profile":pcs::profile(), "log_gates":self.circuit.log_gates,
            "actual_gates":self.circuit.gate_count,"native_constraint_bits":self.circuit.native_bits,
            "padded_gates":self.circuit.n(),"public_io_bits":self.circuit.io.len(),
            "copy_positions":self.circuit.live_len(),"copy_padded_positions":self.circuit.copy_len(),
            "copy_base_field_bits":(self.circuit.h()+1).next_power_of_two(),
            "copy_packing_width":128/(self.circuit.h()+1).next_power_of_two(),
            "gate_hadamard_invocations":5,"permutation_proof":true,
            "copy_argument":"packed permutation",
            "sigma_authentication":"preprocessed public FRI PCS",
            "witness_words":self.witness.len(),"public_table_words":self.public.len(),
            "witness_pcs_params":self.witness_vk.parameter_profile(),
            "public_pcs_params":self.public_vk.parameter_profile(),
            "witness_parameter_digest":blake3::Hash::from(self.witness_vk.parameter_digest()).to_hex().to_string(),
            "public_parameter_digest":blake3::Hash::from(self.public_vk.parameter_digest()).to_hex().to_string(),
            "public_commitment_blake3":digest(&self.public_commitment),
            "plonk_statement_blake3":blake3::Hash::from(self.circuit.digest()).to_hex().to_string(),
            "copy_preprocessing_bytes":self.copy_preprocessing.resident_bytes(),
            "copy_preparation_scope":"fixed label interpolation offline; current witness/public I/O and coefficient layers online",
            "pcs_data_policy":"consume fixed public cache online; replenish offline before each proof and check commitment unchanged; no online recommit",
            "padding_policy":"shared hash PLONK geometry and native PCS minimum 2^15-word padding retained; actual_gates counts genuine gates",
            "pcs_claims":2,"pcs_complete":true,"zero_knowledge_claim":false
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Proof {
    pub commitment: Commitment,
    pub gate: gates::SingleProof,
    pub copy: copy::SingleProof,
    pub gate_nonces: Vec<u64>,
    pub copy_nonces: Vec<u64>,
    pub opening: mikros::packed_opening::Proof,
    pub opening_nonces: Vec<u64>,
    pub witness_pcs: Vec<u8>,
    pub public_pcs: Vec<u8>,
}

fn bind(
    kind: HashKind,
    c: &fixture::Circuit,
    witness: &Commitment,
    public: &Commitment,
    witness_key: &VerifierKey,
    public_key: &VerifierKey,
) -> FsChallenger {
    let mut ch = FsChallenger::new(DOMAIN);
    ch.observe_label(kind.as_str().as_bytes());
    ch.observe_bytes(&c.digest());
    ch.observe_bytes(&witness_key.parameter_digest());
    ch.observe_bytes(&public_key.parameter_digest());
    ch.observe_bytes(&bincode::serialize(witness).expect("commitment encoding"));
    ch.observe_bytes(&bincode::serialize(public).expect("commitment encoding"));
    ch
}

pub fn prove(
    kind: HashKind,
    s: &mut Setup,
    owned: Vec<F128>,
) -> Result<(Proof, Value, FsChallenger), String> {
    let begin = Instant::now();
    if owned.len() != s.circuit.witness_words() {
        return Err("single FRI witness shape".into());
    }
    let public_data = s
        .public_data
        .take()
        .ok_or("missing offline public FRI cache")?;
    event("online_witness_fri_commitment");
    let t = Instant::now();
    let (commitment, data) = s.witness_source.commit(&owned)?;
    let commit_ms = ms(t);
    let mut parent = bind(
        kind,
        &s.circuit,
        &commitment,
        &s.public_commitment,
        &s.witness_vk,
        &s.public_vk,
    );
    let gc = parent.fork(b"gates");
    let cc = parent.fork(b"copy");
    event("online_gate_piop");
    let t = Instant::now();
    let mut g = GrindingChallenger::prover(gc);
    let (gate, mut claims) = gates::single_prove(&s.circuit, &owned, &s.public, &mut g)?;
    let gate_grinding_ms = g.grinding_ms();
    let (gc, gate_nonces) = g.finish()?;
    let gates_ms = ms(t);
    event("online_copy_prepare");
    let t = Instant::now();
    let (prepared, preparation) = copy::prepare_online(&s.circuit, &owned, &s.copy_preprocessing)?;
    let copy_prepare_ms = ms(t);
    event("online_copy_piop");
    let t = Instant::now();
    let mut cc = GrindingChallenger::prover(cc)
        .packed_permutation(s.circuit.h(), 128 / (s.circuit.h() + 1).next_power_of_two());
    let (copy, cclaims) = copy::single_prove(&s.circuit, &owned, &s.public, &prepared, &mut cc)?;
    let copy_grinding_ms = cc.grinding_ms();
    let copy_initial_grinding_ms = cc.initial_fingerprint_grinding_ms();
    let (cc, copy_nonces) = cc.finish()?;
    drop(prepared);
    let copy_ms = ms(t);
    claims.witness.extend(cclaims.witness);
    claims.public.extend(cclaims.public);
    parent.merge_child(gc);
    parent.merge_child(cc);
    let piop_blake3 = digest(&(&gate, &copy, &gate_nonces, &copy_nonces));
    let terminal_claims_blake3 = digest(&(&claims.witness, &claims.public));
    event("online_opening_reduction");
    let t = Instant::now();
    let mut och = GrindingChallenger::prover(parent);
    let (opening, terminal) = mikros::packed_opening::prove(
        &[(&owned, &claims.witness), (&s.public, &claims.public)],
        &mut och,
    )?;
    let opening_grinding_ms = och.grinding_ms();
    let (mut parent, opening_nonces) = och.finish()?;
    let packed_opening_ms = ms(t);
    if terminal.len() != 2 {
        return Err("single FRI expected exactly two canonical PCS terminals".into());
    }
    event("online_witness_fri_open");
    let t = Instant::now();
    let witness_pcs = s.witness_source.open(&commitment, &terminal[0], data)?;
    let witness_pcs_open_ms = ms(t);
    parent.observe_label(pcs::WITNESS_PROOF_LABEL);
    parent.observe_bytes(&witness_pcs);
    event("online_public_fri_open");
    let t = Instant::now();
    let public_pcs = s
        .public_source
        .open(&s.public_commitment, &terminal[1], public_data)?;
    let public_pcs_open_ms = ms(t);
    parent.observe_label(pcs::PUBLIC_PROOF_LABEL);
    parent.observe_bytes(&public_pcs);
    let witness_pcs_proof_bytes = witness_pcs.len();
    let public_pcs_proof_bytes = public_pcs.len();
    drop(owned);
    let times = json!({
        "commit_ms":commit_ms,"gates_ms":gates_ms,"copy_prepare_ms":copy_prepare_ms,"copy_ms":copy_ms,
        "copy_input_ms":preparation.input_ms,"copy_layers_ms":preparation.layers_ms,"copy_samples_ms":preparation.samples_ms,
        "gate_grinding_ms":gate_grinding_ms,"copy_grinding_ms":copy_grinding_ms,
        "copy_initial_grinding_ms":copy_initial_grinding_ms,"opening_grinding_ms":opening_grinding_ms,
        "packed_opening_ms":packed_opening_ms,"witness_pcs_open_ms":witness_pcs_open_ms,
        "public_pcs_open_ms":public_pcs_open_ms,
        "pcs_open_ms":witness_pcs_open_ms+public_pcs_open_ms,"recommit_ms":0.0,
        "pcs_claims":2,"witness_claims":claims.witness.len(),"public_claims":claims.public.len(),
        "witness_pcs_proof_bytes":witness_pcs_proof_bytes,"public_pcs_proof_bytes":public_pcs_proof_bytes,
        "piop_blake3":piop_blake3,"terminal_claims_blake3":terminal_claims_blake3,
        "pcs_complete":true,"prover_ms":ms(begin)
    });
    Ok((
        Proof {
            commitment,
            gate,
            copy,
            gate_nonces,
            copy_nonces,
            opening,
            opening_nonces,
            witness_pcs,
            public_pcs,
        },
        times,
        parent,
    ))
}

#[derive(Default)]
pub struct VerificationTimings {
    pub gate_ms: f64,
    pub copy_ms: f64,
    pub opening_ms: f64,
    pub witness_pcs_ms: f64,
    pub public_pcs_ms: f64,
}

/// The verifier receives only the public descriptor, fixed commitment/keys, and
/// proof. It never receives the source witness, public-table vectors, or caches.
pub fn verify_with_timings(
    kind: HashKind,
    c: &fixture::Circuit,
    public_cm: &Commitment,
    witness_key: &VerifierKey,
    public_key: &VerifierKey,
    p: &Proof,
) -> Result<(FsChallenger, VerificationTimings), String> {
    if !(7..=30).contains(&c.log_gates)
        || c.io.len() > c.n()
        || witness_key.words() != c.witness_words()
        || public_key.words() != c.public_words()
        || p.commitment.words() != witness_key.words()
        || public_cm.words() != public_key.words()
    {
        return Err("single FRI public statement/PCS dimensions".into());
    }
    let log_z = witness_key.words().ilog2() as usize;
    let log_p = public_key.words().ilog2() as usize;
    let mut parent = bind(kind, c, &p.commitment, public_cm, witness_key, public_key);
    let gc = parent.fork(b"gates");
    let cc = parent.fork(b"copy");
    let mut times = VerificationTimings::default();
    let t = Instant::now();
    let mut g = GrindingChallenger::verifier(gc, &p.gate_nonces);
    let mut claims = gates::single_verify(c, log_z, log_p, &p.gate, &mut g)?;
    let (gc, _) = g.finish()?;
    times.gate_ms = ms(t);
    let t = Instant::now();
    let mut cc = GrindingChallenger::verifier(cc, &p.copy_nonces)
        .packed_permutation(c.h(), 128 / (c.h() + 1).next_power_of_two());
    let cclaims = copy::single_verify(c, log_z, log_p, &p.copy, &mut cc)?;
    let (cc, _) = cc.finish()?;
    times.copy_ms = ms(t);
    claims.witness.extend(cclaims.witness);
    claims.public.extend(cclaims.public);
    parent.merge_child(gc);
    parent.merge_child(cc);
    let t = Instant::now();
    let mut och = GrindingChallenger::verifier(parent, &p.opening_nonces);
    let terminal = mikros::packed_opening::verify(
        &[log_z, log_p],
        &[&claims.witness, &claims.public],
        &p.opening,
        &mut och,
    )?;
    let (mut parent, _) = och.finish()?;
    times.opening_ms = ms(t);
    if terminal.len() != 2 {
        return Err("single FRI expected exactly two canonical PCS terminals".into());
    }
    let t = Instant::now();
    pcs::verify(witness_key, &p.commitment, &terminal[0], &p.witness_pcs)?;
    parent.observe_label(pcs::WITNESS_PROOF_LABEL);
    parent.observe_bytes(&p.witness_pcs);
    times.witness_pcs_ms = ms(t);
    let t = Instant::now();
    pcs::verify(public_key, public_cm, &terminal[1], &p.public_pcs)?;
    parent.observe_label(pcs::PUBLIC_PROOF_LABEL);
    parent.observe_bytes(&p.public_pcs);
    times.public_pcs_ms = ms(t);
    Ok((parent, times))
}

pub fn proof_sizes(p: &Proof) -> Result<[u64; 3], bincode::Error> {
    Ok([
        bincode::serialized_size(&(&p.gate, &p.gate_nonces))?,
        // Public-table authentication is grouped with the permutation argument
        // by the experiment's accounting convention (selectors also share it).
        bincode::serialized_size(&(&p.copy, &p.copy_nonces, &p.public_pcs))?,
        bincode::serialized_size(&(&p.commitment, &p.opening, &p.opening_nonces, &p.witness_pcs))?,
    ])
}

pub fn tamper(kind: HashKind, s: &Setup, p: &Proof) -> Result<Vec<String>, String> {
    let verify = |c: &fixture::Circuit, cm: &Commitment, p: &Proof| {
        verify_with_timings(kind, c, cm, &s.witness_vk, &s.public_vk, p)
    };
    let mut cases = Vec::new();
    for i in 0..18 {
        let mut bad = p.clone();
        let name = match i {
            0 => {
                bad.gate.scalars[0] += F128::ONE;
                "gate_scalar"
            }
            1 => {
                bad.gate.hadamards.pop();
                "missing_gate_reduction"
            }
            2 => {
                bad.copy.permutation.roots[0] += F128::ONE;
                "permutation_root"
            }
            3 => {
                bad.copy.wire_values[0][0] += F128::ONE;
                "permutation_wire_terminal"
            }
            4 => {
                bad.copy.label_values[0] += F128::ONE;
                "permutation_position_terminal"
            }
            5 => {
                bad.witness_pcs[0] ^= 1;
                "witness_fri_tamper"
            }
            6 => {
                bad.public_pcs[0] ^= 1;
                "public_fri_tamper"
            }
            7 => {
                bad.witness_pcs.clear();
                "missing_witness_fri"
            }
            8 => {
                bad.public_pcs.clear();
                "missing_public_fri"
            }
            9 => {
                bad.copy_nonces.pop();
                "copy_grinding_shape"
            }
            10 => {
                bad.gate_nonces.pop();
                "gate_grinding_shape"
            }
            11 => {
                bad.opening.columns[0][0] += F128::ONE;
                "opening_column"
            }
            12 => {
                bad.opening.ips[0].final_a += F128::ONE;
                "opening_ip_terminal"
            }
            13 => {
                bad.opening.ips.pop();
                "missing_opening_group"
            }
            14 => {
                bad.opening_nonces.pop();
                "opening_grinding_shape"
            }
            15 => {
                bad.witness_pcs.pop();
                "truncated_witness_fri"
            }
            16 => {
                bad.witness_pcs.push(0);
                "trailing_witness_fri"
            }
            _ => {
                bad.public_pcs.push(0);
                "trailing_public_fri"
            }
        };
        if verify(&s.circuit, &s.public_commitment, &bad).is_ok() {
            return Err(format!("single FRI accepted {name}"));
        }
        cases.push(name.into());
    }
    for i in (0..s.circuit.io.len()).filter(|&i| i % 128 == 0 || i + 1 == s.circuit.io.len()) {
        let mut c = s.circuit.clone();
        c.io[i] ^= true;
        if verify(&c, &s.public_commitment, p).is_ok() {
            return Err(format!("single FRI accepted changed public bit {i}"));
        }
        cases.push(format!("public_bit_{i}"));
    }
    let wrong_kind = if kind == HashKind::Sha256 {
        HashKind::Blake3
    } else {
        HashKind::Sha256
    };
    if verify_with_timings(
        wrong_kind,
        &s.circuit,
        &s.public_commitment,
        &s.witness_vk,
        &s.public_vk,
        p,
    )
    .is_ok()
    {
        return Err("single FRI accepted wrong hash kind".into());
    }
    cases.push("hash_kind".into());
    // Commitment encoding starts with the 32-byte Merkle root; keep dimensions
    // and parameter digest intact to exercise actual root authentication.
    let mut encoded = bincode::serialize(&s.public_commitment).map_err(|e| e.to_string())?;
    encoded[0] ^= 1;
    let wrong_root: Commitment = bincode::deserialize(&encoded).map_err(|e| e.to_string())?;
    if verify(&s.circuit, &wrong_root, p).is_ok() {
        return Err("single FRI accepted changed public commitment".into());
    }
    cases.push("public_commitment_root".into());
    Ok(cases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_hash_fri_complete_proof_and_tamper_checks() {
        let (circuit, witness) = fixture::tiny_fixture();
        let mut s = Setup::from_compiled(circuit, witness).unwrap();
        let owned = s.witness.clone();
        let (proof, times, mut prover) = prove(HashKind::Sha256, &mut s, owned).unwrap();
        let encoded = bincode::serialize(&proof).unwrap();
        let decoded: Proof = bincode::deserialize(&encoded).unwrap();
        let (mut verifier, _) = verify_with_timings(
            HashKind::Sha256,
            &s.circuit,
            &s.public_commitment,
            &s.witness_vk,
            &s.public_vk,
            &decoded,
        )
        .unwrap();
        assert_eq!(prover.sample_f128_vec(4), verifier.sample_f128_vec(4));
        assert_eq!(
            proof_sizes(&proof).unwrap().into_iter().sum::<u64>(),
            encoded.len() as u64
        );
        assert_eq!(times["pcs_claims"], 2);
        assert_eq!(times["recommit_ms"], 0.0);
        assert!(tamper(HashKind::Sha256, &s, &proof).unwrap().len() >= 22);
        let wrong_key = VerifierKey::new(2 * s.witness_vk.words()).unwrap();
        assert!(
            verify_with_timings(
                HashKind::Sha256,
                &s.circuit,
                &s.public_commitment,
                &wrong_key,
                &s.public_vk,
                &proof
            )
            .is_err()
        );
        let wrong_public_key = VerifierKey::new(2 * s.public_vk.words()).unwrap();
        assert!(
            verify_with_timings(
                HashKind::Sha256,
                &s.circuit,
                &s.public_commitment,
                &s.witness_vk,
                &wrong_public_key,
                &proof
            )
            .is_err()
        );
        assert!(s.public_data.is_none());
        let fixed = s.public_commitment.clone();
        s.prepare_public_cache().unwrap();
        assert_eq!(s.public_commitment, fixed);
        let owned = s.witness.clone();
        let (again, _, _) = prove(HashKind::Sha256, &mut s, owned).unwrap();
        assert_eq!(bincode::serialize(&again).unwrap(), encoded);
    }
}
