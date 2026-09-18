//! Single hash PLONK from Section 6: five weighted gate reductions, packed
//! copy permutation, and separate native PCS openings for witness/public data.
#[path = "../e2e_single_plonk/copy.rs"]
pub mod copy;
#[path = "../e2e_single_plonk/fixture.rs"]
pub mod fixture;
#[path = "../e2e_single_plonk/gates.rs"]
pub mod gates;
#[path = "grinding.rs"]
mod grinding;
#[path = "../e2e_plonk/queries.rs"]
mod queries;

use crate::{
    e2e_common::PreparedWitness,
    hash_fixture::{HashFixture, HashKind},
};
use flock_core::{
    challenger::{Challenger, FsChallenger},
    pcs::{self, Commitment, PcsParams, ProverData, merged_tensor},
    schedule::Registry,
};
use flock_field::F128;
use grinding::GrindingChallenger;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::time::Instant;

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}
fn digest<T: Serialize>(x: &T) -> String {
    blake3::hash(&bincode::serialize(x).unwrap())
        .to_hex()
        .to_string()
}

pub struct Setup {
    pub circuit: fixture::Circuit,
    pub witness: Vec<F128>,
    pub public: Vec<F128>,
    pub witness_registry: Registry,
    pub public_registry: Registry,
    pub witness_params: PcsParams,
    pub public_params: PcsParams,
    pub public_commitment: Commitment,
    pub offline_copy_setup_ms: f64,
    public_data: ProverData,
    copy_preprocessing: copy::Preprocessing,
}

impl Setup {
    pub fn new(f: &HashFixture, prepared: &PreparedWitness) -> Result<Self, String> {
        let (circuit, witness) = fixture::compile(f, prepared)?;
        let public = circuit.public();
        let block_log = circuit.wire_words().ilog2() as usize;
        let witness_registry = fixture::registry(witness.len().ilog2() as usize, block_log);
        let public_registry = fixture::registry(public.len().ilog2() as usize, block_log);
        let witness_params = fixture::parameters(&fixture::union(&witness_registry));
        let public_params = fixture::parameters(&fixture::union(&public_registry));
        witness_params
            .ligerito_prover_config()
            .map_err(|e| format!("witness PCS config: {e:?}"))?;
        public_params
            .ligerito_prover_config()
            .map_err(|e| format!("public PCS config: {e:?}"))?;
        let (public_commitment, public_data) = pcs::commit(&public, &public_params);
        let start = Instant::now();
        let copy_preprocessing = copy::preprocess(&circuit)?;
        let offline_copy_setup_ms = ms(start);
        Ok(Self {
            circuit,
            witness,
            public,
            witness_registry,
            public_registry,
            witness_params,
            public_params,
            public_commitment,
            offline_copy_setup_ms,
            public_data,
            copy_preprocessing,
        })
    }
    pub fn metadata(&self) -> Value {
        json!({"arithmetization":"binary PLONK", "log_gates":self.circuit.log_gates,
            "actual_gates":self.circuit.gate_count,"native_constraint_bits":self.circuit.native_bits,
            "padded_gates":self.circuit.n(),"public_io_bits":self.circuit.io.len(),
            "copy_positions":self.circuit.live_len(),"copy_padded_positions":self.circuit.copy_len(),
            "copy_base_field_bits":(self.circuit.h()+1).next_power_of_two(),
            "copy_packing_width":128/(self.circuit.h()+1).next_power_of_two(),
            "gate_hadamard_invocations":5,"permutation_proof":true,
            "witness_words":self.witness.len(),"public_table_words":self.public.len(),
            "witness_pcs_params":self.witness_params,"public_pcs_params":self.public_params,
            "public_commitment_blake3":digest(&self.public_commitment),
            "plonk_statement_blake3":self.circuit.digest().iter().map(|x|format!("{x:02x}")).collect::<String>(),
            "scalar_projection":if std::env::var_os("MIKROS_SINGLE_GATE_REFERENCE").is_some_and(|v|v=="1"){ "materialized reference" }else{ "fused subset-XOR" },
            "opening_projection":if std::env::var_os("MIKROS_OPENING_REFERENCE").is_some_and(|v|v=="1"){ "dense reference" }else{ "restricted source blocks and fused native IP" },
            "pcs_data_policy":if std::env::var_os("MIKROS_SINGLE_PCS_REFERENCE").is_some_and(|v|v=="1"){ "reference online recommit" }else{ "retain witness data; reuse preprocessed public data" },
            "copy_preprocessing_bytes":self.copy_preprocessing.resident_bytes(),
            "copy_preparation_kernel":if std::env::var_os("MIKROS_SINGLE_COPY_REFERENCE").is_some_and(|v|v=="1"){ "v6 bit-sliced source and SWAR decode" }else{ "direct rows and preprocessed public label interpolation" },
            "copy_preparation_scope":"fixed circuit label interpolation offline; current witness and public I/O bits and their coefficient layers online"})
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
    pub witness_pcs: merged_tensor::Proof,
    pub public_pcs: merged_tensor::Proof,
}

fn bind(
    kind: HashKind,
    c: &fixture::Circuit,
    cm: &Commitment,
    public_cm: &Commitment,
) -> FsChallenger {
    let mut ch = FsChallenger::new(b"mikros-single-native-hash-plonk-v1");
    ch.observe_label(kind.as_str().as_bytes());
    ch.observe_bytes(&c.digest());
    ch.observe_bytes(&bincode::serialize(cm).unwrap());
    ch.observe_bytes(&bincode::serialize(public_cm).unwrap());
    ch
}

pub fn prove(
    kind: HashKind,
    s: &Setup,
    owned: Vec<F128>,
) -> Result<(Proof, Value, FsChallenger), String> {
    let begin = Instant::now();
    if owned.len() != s.witness.len() {
        return Err("PLONK witness shape".into());
    }
    let t = Instant::now();
    let (commitment, data) = pcs::commit(&owned, &s.witness_params);
    let commit_ms = ms(t);
    let mut parent = bind(kind, &s.circuit, &commitment, &s.public_commitment);
    let gc = parent.fork(b"gates");
    let cc = parent.fork(b"copy");
    let t = Instant::now();
    let mut g = GrindingChallenger::prover(gc);
    let (gate, mut claims) = gates::single_prove(&s.circuit, &owned, &s.public, &mut g)?;
    let gate_grinding_ms = g.grinding_ms();
    let (gc, gate_nonces) = g.finish()?;
    let gates_ms = ms(t);
    let t = Instant::now();
    let (prepared, copy_preparation) =
        copy::prepare_online(&s.circuit, &owned, &s.copy_preprocessing)?;
    let copy_prepare_ms = ms(t);
    let t = Instant::now();
    let mut cch = GrindingChallenger::prover(cc)
        .packed_permutation(s.circuit.h(), 128 / (s.circuit.h() + 1).next_power_of_two());
    let (copy, cclaims) = copy::single_prove(&s.circuit, &owned, &s.public, &prepared, &mut cch)?;
    let copy_grinding_ms = cch.grinding_ms();
    let copy_initial_grinding_ms = cch.initial_fingerprint_grinding_ms();
    let (cc, copy_nonces) = cch.finish()?;
    drop(prepared);
    let copy_ms = ms(t);
    claims.witness.extend(cclaims.witness);
    claims.public.extend(cclaims.public);
    parent.merge_child(gc);
    parent.merge_child(cc);
    let piop_blake3 = digest(&(&gate, &copy, &gate_nonces, &copy_nonces));
    let terminal_claims_blake3 = digest(&(&claims.witness, &claims.public));
    let t = Instant::now();
    let mut och = GrindingChallenger::prover(parent);
    let (opening, terminal) = mikros::packed_opening::prove(
        &[(&owned, &claims.witness), (&s.public, &claims.public)],
        &mut och,
    )?;
    let opening_grinding_ms = och.grinding_ms();
    let (mut parent, opening_nonces) = och.finish()?;
    let packed_opening_ms = ms(t);
    let t = Instant::now();
    let reference = std::env::var_os("MIKROS_SINGLE_PCS_REFERENCE").is_some_and(|v| v == "1");
    let repeated_w = reference.then(|| pcs::commit(&owned, &s.witness_params));
    let repeated_p = reference.then(|| pcs::commit(&s.public, &s.public_params));
    if repeated_w
        .as_ref()
        .is_some_and(|(cm, _)| digest(cm) != digest(&commitment))
        || repeated_p
            .as_ref()
            .is_some_and(|(cm, _)| digest(cm) != digest(&s.public_commitment))
    {
        return Err("recommit mismatch".into());
    }
    let recommit_ms = ms(t);
    let t = Instant::now();
    let witness_pcs = merged_tensor::open_identity(
        owned,
        repeated_w.as_ref().map_or(&data, |(_, d)| d),
        &commitment,
        &terminal[..1],
        &fixture::union(&s.witness_registry),
        &mut parent,
    )
    .map_err(|e| format!("PLONK witness PCS: {e:?}"))?;
    let witness_pcs_ms = ms(t);
    let t = Instant::now();
    // The native PCS consumes its input: this required online copy is timed.
    let public_pcs = merged_tensor::open_identity(
        s.public.clone(),
        repeated_p.as_ref().map_or(&s.public_data, |(_, d)| d),
        &s.public_commitment,
        &terminal[1..],
        &fixture::union(&s.public_registry),
        &mut parent,
    )
    .map_err(|e| format!("PLONK public PCS: {e:?}"))?;
    let public_pcs_ms = ms(t);
    let times = json!({"commit_ms":commit_ms,"gates_ms":gates_ms,"copy_prepare_ms":copy_prepare_ms,"copy_ms":copy_ms,"recommit_ms":recommit_ms,
        "copy_input_ms":copy_preparation.input_ms,"copy_layers_ms":copy_preparation.layers_ms,"copy_samples_ms":copy_preparation.samples_ms,
        "gate_grinding_ms":gate_grinding_ms,"copy_grinding_ms":copy_grinding_ms,"copy_initial_grinding_ms":copy_initial_grinding_ms,"opening_grinding_ms":opening_grinding_ms,
        "packed_opening_ms":packed_opening_ms,"witness_pcs_open_ms":witness_pcs_ms,"public_pcs_open_ms":public_pcs_ms,"pcs_open_ms":witness_pcs_ms+public_pcs_ms,
        "native_pcs_claims":2,
        "witness_claims":claims.witness.len(),"public_claims":claims.public.len(),
        "witness_ring_claims":witness_pcs.merged.as_ref().map_or(0,|p|p.ring_switches.len()),
        "public_ring_claims":public_pcs.merged.as_ref().map_or(0,|p|p.ring_switches.len()),
        "piop_blake3":piop_blake3,"terminal_claims_blake3":terminal_claims_blake3,"prover_ms":ms(begin)});
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

/// Takes only the public circuit, fixed preprocessing commitment and proof.
pub fn verify(
    kind: HashKind,
    c: &fixture::Circuit,
    public_cm: &Commitment,
    p: &Proof,
) -> Result<FsChallenger, String> {
    let log_z = c.witness_words().ilog2() as usize;
    let log_p = c.public_words().ilog2() as usize;
    let block_log = c.wire_words().ilog2() as usize;
    let wr = fixture::registry(log_z, block_log);
    let pr = fixture::registry(log_p, block_log);
    let wu = fixture::union(&wr);
    let pu = fixture::union(&pr);
    if p.commitment.params != fixture::parameters(&wu)
        || public_cm.params != fixture::parameters(&pu)
    {
        return Err("PLONK PCS params".into());
    }
    let mut parent = bind(kind, c, &p.commitment, public_cm);
    let gc = parent.fork(b"gates");
    let cc = parent.fork(b"copy");
    let mut g = GrindingChallenger::verifier(gc, &p.gate_nonces);
    let mut claims = gates::single_verify(c, log_z, log_p, &p.gate, &mut g)?;
    let (gc, _) = g.finish()?;
    let mut cch = GrindingChallenger::verifier(cc, &p.copy_nonces)
        .packed_permutation(c.h(), 128 / (c.h() + 1).next_power_of_two());
    let cclaims = copy::single_verify(c, log_z, log_p, &p.copy, &mut cch)?;
    let (cc, _) = cch.finish()?;
    claims.witness.extend(cclaims.witness);
    claims.public.extend(cclaims.public);
    parent.merge_child(gc);
    parent.merge_child(cc);
    let mut och = GrindingChallenger::verifier(parent, &p.opening_nonces);
    let terminal = mikros::packed_opening::verify(
        &[log_z, log_p],
        &[&claims.witness, &claims.public],
        &p.opening,
        &mut och,
    )?;
    let (mut parent, _) = och.finish()?;
    merged_tensor::verify(
        &p.commitment,
        &terminal[..1],
        &p.witness_pcs,
        &wu,
        &mut parent,
    )
    .map_err(|e| format!("PLONK witness PCS replay: {e:?}"))?;
    merged_tensor::verify(public_cm, &terminal[1..], &p.public_pcs, &pu, &mut parent)
        .map_err(|e| format!("PLONK public PCS replay: {e:?}"))?;
    Ok(parent)
}

pub fn tamper(kind: HashKind, s: &Setup, p: &Proof) -> Result<Vec<String>, String> {
    let mut cases = Vec::new();
    for k in 0..15 {
        let mut bad = p.clone();
        let name = match k {
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
                bad.witness_pcs.component_values[0] += F128::ONE;
                "witness_pcs_value"
            }
            6 => {
                bad.public_pcs.component_values[0] += F128::ONE;
                "public_pcs_value"
            }
            7 => {
                bad.witness_pcs.merged = None;
                "missing_witness_pcs"
            }
            8 => {
                bad.public_pcs.merged = None;
                "missing_public_pcs"
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
            _ => {
                bad.opening_nonces.pop();
                "opening_grinding_shape"
            }
        };
        if verify(kind, &s.circuit, &s.public_commitment, &bad).is_ok() {
            return Err(format!("accepted {name}"));
        }
        cases.push(name.to_owned());
    }
    for i in 0..s.circuit.io.len() {
        // Cover every 128-bit native public word; constituent bit constraints
        // are independently checked by the compiler's source-oracle tests.
        if i % 128 != 0 && i + 1 != s.circuit.io.len() {
            continue;
        }
        let mut bad = s.circuit.clone();
        bad.io[i] = !bad.io[i];
        if verify(kind, &bad, &s.public_commitment, p).is_ok() {
            return Err("accepted changed public I/O".into());
        }
        cases.push(format!("public_bit_{i}"));
    }
    Ok(cases)
}
