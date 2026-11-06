//! Two groups of 128 native hash executions with a complete additive-FRI PCS.
//!
//! The packed R1CS reductions and exact public-I/O cover are shared with the
//! native-PCS experiment. Their arbitrary bit-linear functionals pass through
//! Mikros's packed-opening frontend before the one canonical F128 MLE is opened.
//! Public matrices are verifier-known CSC (or the same native circuit walker),
//! C is the identity, and there is no additional PLONK permutation argument.

#[path = "grinding.rs"]
mod grinding;
#[path = "../e2e_plonk_fri/pcs.rs"]
mod pcs;

use super::{
    e2e_common::{TreeVerifier, batched_boolean::TransposedWitness, column_boolean},
    hash_fixture::HashKind,
    hash_mikros::{PublicIo, public_claims},
};
use flock_core::{
    challenger::{Challenger, FsChallenger},
    lincheck::LincheckCircuit,
    pcs::{OpeningGrinding, paired},
};
use flock_field::F128;
use grinding::GrindingChallenger;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::time::Instant;

pub use pcs::{Commitment, VerifierKey};
pub const BACKEND_NAME: &str = pcs::BACKEND_NAME;
const DOMAIN: &[u8] = b"mikros-native-hash-packed-r1cs-additive-fri-v1";

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

/// Contains only reusable, witness-independent FRI parameters. Every proof
/// commits its current witness online and consumes that proof's own cache.
pub struct Setup {
    pub witness_vk: VerifierKey,
    source: pcs::Source,
}

impl Setup {
    pub fn new(words: usize) -> Result<Self, String> {
        let source = pcs::Source::new(words, 0x4d554c5449504c45)?;
        let witness_vk = source.verifier_key().clone();
        Ok(Self { witness_vk, source })
    }

    pub fn metadata(&self) -> Value {
        json!({
            "arithmetization":"binary packed R1CS", "pcs_backend":BACKEND_NAME,
            "pcs_profile":pcs::profile(), "instances":256,
            "packed_groups":2,"instance_packing_width":128,
            "gate_hadamard_invocations":2,"permutation_proof":false,
            "matrix_authentication":"public CSC/native walker, C=I",
            "public_io_authentication":"exact public column cover and constant pin; paired claims then packed opening",
            "witness_words":self.witness_vk.words(),
            "witness_pcs_params":self.witness_vk.parameter_profile(),
            "witness_parameter_digest":blake3::Hash::from(self.witness_vk.parameter_digest()).to_hex().to_string(),
            "pcs_claims":1,"pcs_complete":true,"zero_knowledge_claim":false,
            "piop_grinding_bits":grinding::PIOP_GRIND_BITS,
            "pairing_grinding_bits":OpeningGrinding::per_challenge_128().claim_batch_bits,
            "packed_opening_grinding_bits":grinding::PIOP_GRIND_BITS,
            "pcs_data_policy":"online witness commit; consume its cached codeword and Merkle tree in one FRI opening; no recommit",
            "padding_policy":"unchanged two-group native R1CS carrier; no PLONK or extra PCS padding"
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Proof {
    pub commitment: Commitment,
    pub boolean: column_boolean::Proof,
    pub nonces: Vec<u64>,
    pub pairing_nonce: u64,
    pub opening: mikros::packed_opening::Proof,
    pub opening_nonces: Vec<u64>,
    pub witness_pcs: Vec<u8>,
}

fn shape(view: &TreeVerifier<'_>, key: &VerifierKey) -> Result<(), String> {
    if view.nu != 8 || view.live_rows != 256 || !(7..=24).contains(&view.r1cs.k_log) {
        return Err("multiple FRI requires 256 independent executions".into());
    }
    if key.words() != 2 * (1usize << view.r1cs.k_log) {
        return Err("multiple FRI public statement/PCS dimensions".into());
    }
    Ok(())
}

fn bind(
    kind: HashKind,
    view: &TreeVerifier<'_>,
    cm: &Commitment,
    key: &VerifierKey,
) -> FsChallenger {
    let mut ch = FsChallenger::new(DOMAIN);
    ch.observe_label(kind.as_str().as_bytes());
    ch.observe_bytes(&view.circuit.digest());
    ch.observe_bytes(&view.r1cs.statement_digest());
    ch.observe_f128_slice(view.public);
    ch.observe_bytes(&(view.live_rows as u64).to_le_bytes());
    ch.observe_bytes(&key.parameter_digest());
    ch.observe_bytes(&bincode::serialize(cm).expect("commitment encoding"));
    ch
}

pub fn prove(
    kind: HashKind,
    view: &TreeVerifier<'_>,
    matrix: &dyn LincheckCircuit,
    witness: &TransposedWitness,
    owned_z: Vec<F128>,
    io: &PublicIo,
    setup: &Setup,
) -> Result<(Proof, Value, FsChallenger), String> {
    let begin = Instant::now();
    shape(view, &setup.witness_vk)?;
    if owned_z.len() != setup.witness_vk.words() {
        return Err("multiple FRI witness length".into());
    }
    event("online_witness_fri_commitment");
    let t = Instant::now();
    let (commitment, data) = setup.source.commit(&owned_z)?;
    let commit_ms = ms(t);
    let mut ch = GrindingChallenger::prover(bind(kind, view, &commitment, &setup.witness_vk));
    event("online_packed_r1cs_piop");
    let t = Instant::now();
    let (boolean, claims) = column_boolean::prove_with_matrix(view, matrix, witness, &mut ch)?;
    let reduction_f128_ms = ms(t);
    let t = Instant::now();
    let public = public_claims(view, io, &mut ch);
    let public_io_ms = ms(t);
    let piop_grinding_ms = ch.grinding_ms();
    let (mut ch, nonces) = ch.finish()?;
    let t = Instant::now();
    // Match the original Fast PCS's six-bit filter on the pairing challenge.
    // This is a transport policy, independent of the FRI query-phase target.
    let (pairing_nonce, mut queries) =
        paired::reduce_prover(&public, OpeningGrinding::per_challenge_128(), &mut ch)
            .map_err(|e| format!("pair reduction: {e:?}"))?;
    queries.extend(claims.other);
    let pair_reduction_ms = ms(t);
    let source_claims = queries.len();
    let piop_blake3 = digest(&(&boolean, &nonces, pairing_nonce));
    let terminal_claims_blake3 = digest(&queries);
    event("online_packed_opening_reduction");
    let t = Instant::now();
    let mut och = GrindingChallenger::prover(ch);
    let (opening, terminal) = mikros::packed_opening::prove(&[(&owned_z, &queries)], &mut och)?;
    let opening_grinding_ms = och.grinding_ms();
    let (mut ch, opening_nonces) = och.finish()?;
    let packed_opening_ms = ms(t);
    if terminal.len() != 1 {
        return Err("multiple FRI expected one canonical PCS terminal".into());
    }
    event("online_witness_fri_open");
    let t = Instant::now();
    let witness_pcs = setup.source.open(&commitment, &terminal[0], data)?;
    ch.observe_label(pcs::WITNESS_PROOF_LABEL);
    ch.observe_bytes(&witness_pcs);
    let witness_pcs_open_ms = ms(t);
    let witness_pcs_proof_bytes = witness_pcs.len();
    drop(owned_z);
    let times = json!({
        "commit_ms":commit_ms,"reduction_f128_ms":reduction_f128_ms,
        "public_io_ms":public_io_ms,"pair_reduction_ms":pair_reduction_ms,
        "packed_opening_ms":packed_opening_ms,"witness_pcs_open_ms":witness_pcs_open_ms,
        "pcs_open_ms":packed_opening_ms+witness_pcs_open_ms,
        "piop_grinding_ms":piop_grinding_ms,"opening_grinding_ms":opening_grinding_ms,
        "source_claims":source_claims,"witness_claims":source_claims,
        "public_io_blocks":io.blocks.len(),"pcs_claims":1,"inner_openings":1,
        "witness_pcs_proof_bytes":witness_pcs_proof_bytes,"recommit_ms":0.0,
        "piop_blake3":piop_blake3,"terminal_claims_blake3":terminal_claims_blake3,
        "pcs_complete":true,"prover_ms":ms(begin)
    });
    Ok((
        Proof {
            commitment,
            boolean,
            nonces,
            pairing_nonce,
            opening,
            opening_nonces,
            witness_pcs,
        },
        times,
        ch,
    ))
}

#[derive(Default)]
pub struct VerificationTimings {
    pub reduction_ms: f64,
    pub public_io_ms: f64,
    pub pairing_ms: f64,
    pub opening_ms: f64,
    pub witness_pcs_ms: f64,
}

/// Only the public statement/matrices, public I/O and fixed FRI parameters are
/// supplied to verification. It never receives the witness or prover cache.
pub fn verify_with_timings(
    kind: HashKind,
    view: &TreeVerifier<'_>,
    matrix: &dyn LincheckCircuit,
    io: &PublicIo,
    key: &VerifierKey,
    proof: &Proof,
) -> Result<(FsChallenger, VerificationTimings), String> {
    shape(view, key)?;
    if proof.commitment.words() != key.words() {
        return Err("multiple FRI commitment dimensions".into());
    }
    let mut times = VerificationTimings::default();
    let mut ch =
        GrindingChallenger::verifier(bind(kind, view, &proof.commitment, key), &proof.nonces);
    let t = Instant::now();
    let claims = column_boolean::verify_with_matrix(view, matrix, &proof.boolean, &mut ch)?;
    times.reduction_ms = ms(t);
    let t = Instant::now();
    let public = public_claims(view, io, &mut ch);
    let (mut ch, _) = ch.finish()?;
    times.public_io_ms = ms(t);
    let t = Instant::now();
    let mut queries = paired::reduce_verify(
        &public,
        proof.pairing_nonce,
        OpeningGrinding::per_challenge_128(),
        &mut ch,
    )
    .map_err(|e| format!("pair verification: {e:?}"))?;
    queries.extend(claims.other);
    times.pairing_ms = ms(t);
    let t = Instant::now();
    let mut och = GrindingChallenger::verifier(ch, &proof.opening_nonces);
    let terminal = mikros::packed_opening::verify(
        &[key.words().ilog2() as usize],
        &[&queries],
        &proof.opening,
        &mut och,
    )?;
    let (mut ch, _) = och.finish()?;
    times.opening_ms = ms(t);
    if terminal.len() != 1 {
        return Err("multiple FRI expected one canonical PCS terminal".into());
    }
    let t = Instant::now();
    pcs::verify(key, &proof.commitment, &terminal[0], &proof.witness_pcs)?;
    ch.observe_label(pcs::WITNESS_PROOF_LABEL);
    ch.observe_bytes(&proof.witness_pcs);
    times.witness_pcs_ms = ms(t);
    Ok((ch, times))
}

pub fn verify(
    kind: HashKind,
    view: &TreeVerifier<'_>,
    matrix: &dyn LincheckCircuit,
    io: &PublicIo,
    key: &VerifierKey,
    proof: &Proof,
) -> Result<FsChallenger, String> {
    verify_with_timings(kind, view, matrix, io, key, proof).map(|(ch, _)| ch)
}

pub fn tamper(
    kind: HashKind,
    view: &TreeVerifier<'_>,
    matrix: &dyn LincheckCircuit,
    io: &PublicIo,
    key: &VerifierKey,
    proof: &Proof,
) -> Result<Vec<String>, String> {
    let mut cases = Vec::new();
    let mut reject = |p: &Proof, name: String| -> Result<(), String> {
        if verify(kind, view, matrix, io, key, p).is_ok() {
            return Err(format!("multiple FRI accepted {name}"));
        }
        cases.push(name);
        Ok(())
    };
    for group in 0..2 {
        for field in 0..4 {
            let mut bad = proof.clone();
            let h = &mut bad.boolean.hadamards[group];
            match field {
                0 => h.quotients[0] += F128::ONE,
                1 => h.inner_product.final_a += F128::ONE,
                2 => h.inner_product.final_b += F128::ONE,
                _ => h.projected_c += F128::ONE,
            }
            reject(&bad, format!("hadamard_{group}_{field}"))?;
        }
    }
    for side in 0..2 {
        for field in 0..5 {
            let mut bad = proof.clone();
            let p = &mut bad.boolean.linchecks[side];
            match field {
                0 => p.columns[0][7] += F128::ONE,
                1 => p.ip.final_a += F128::ONE,
                2 => p.ip.final_b += F128::ONE,
                3 => p.ip.rounds[0][0] += F128::ONE,
                _ => {
                    p.columns.pop();
                }
            }
            reject(&bad, format!("image_{side}_{field}"))?;
        }
    }
    for field in 0..15 {
        let mut bad = proof.clone();
        let name = match field {
            0 => {
                bad.boolean.hadamards.swap(0, 1);
                "group_swap"
            }
            1 => {
                bad.boolean.hadamards.pop();
                "group_shape"
            }
            2 => {
                bad.boolean.linchecks.pop();
                "image_shape"
            }
            3 => {
                let mut encoded = bincode::serialize(&bad.commitment).map_err(|e| e.to_string())?;
                encoded[0] ^= 1;
                bad.commitment = bincode::deserialize(&encoded).map_err(|e| e.to_string())?;
                "commitment_root"
            }
            4 => {
                bad.nonces.pop();
                "piop_grinding_shape"
            }
            5 => {
                bad.pairing_nonce ^= 1;
                "pair_nonce"
            }
            6 => {
                bad.opening.columns[0][0] += F128::ONE;
                "opening_column"
            }
            7 => {
                bad.opening.ips[0].final_a += F128::ONE;
                "opening_terminal"
            }
            8 => {
                bad.opening.columns.pop();
                "opening_claim_shape"
            }
            9 => {
                bad.opening.ips.clear();
                "missing_opening_group"
            }
            10 => {
                bad.opening_nonces.pop();
                "opening_grinding_shape"
            }
            11 => {
                bad.witness_pcs[0] ^= 1;
                "fri_tamper"
            }
            12 => {
                bad.witness_pcs.clear();
                "missing_fri"
            }
            13 => {
                bad.witness_pcs.pop();
                "truncated_fri"
            }
            _ => {
                bad.witness_pcs.push(0);
                "trailing_fri"
            }
        };
        reject(&bad, name.into())?;
    }
    let wrong_kind = if kind == HashKind::Sha256 {
        HashKind::Blake3
    } else {
        HashKind::Sha256
    };
    if verify(wrong_kind, view, matrix, io, key, proof).is_ok() {
        return Err("multiple FRI accepted wrong hash kind".into());
    }
    cases.push("hash_kind".into());
    Ok(cases)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash_fixture::PublicWordBinding;
    use flock_core::{
        circuit::{Cell, Circuit},
        r1cs::{BlockR1cs, SparseBinaryMatrix, WitnessLayout},
        schedule::{IoWord, Registry, TableType},
    };
    use std::sync::OnceLock;

    #[test]
    fn multiple_hash_fri_complete_proof_and_tamper_checks() {
        // Smallest supported base matrix: 128 identity constraints, with two
        // packed groups. x*x=x holds bitwise; bit zero is the constant-one pin.
        let identity = SparseBinaryMatrix::new(128, 128, (0..128).map(|i| vec![i]).collect());
        let r1cs = BlockR1cs {
            m: 15,
            k_log: 7,
            k_skip: 7,
            useful_bits: 128,
            a_0: identity.clone(),
            b_0: identity.clone(),
            c_0: identity,
            layout: WitnessLayout::BatchMajor,
            const_pin: Some(0),
            digest_cache: OnceLock::new(),
            csc_cache: OnceLock::new(),
        };
        let registry = Registry::new(
            vec![TableType::from_block_r1cs(&r1cs).with_io_schema(vec![IoWord::input(0)])],
            8,
        );
        let public: Vec<_> = (0..256)
            .map(|i| F128::new((i * 977 + 1) | 1, i * 391 + 27))
            .collect();
        let wires = (0..256)
            .map(|i| vec![Cell::new(0, i), Cell::new(1, i)])
            .collect();
        let circuit = Circuit::new(&registry, vec![256], public.len(), wires).unwrap();
        let view = TreeVerifier {
            nu: 8,
            r1cs: &r1cs,
            registry: &registry,
            circuit: &circuit,
            public: &public,
            live_rows: 256,
        };
        let bindings: Vec<_> = (0..256)
            .map(|i| PublicWordBinding {
                word_col: 0,
                row: i,
                public_index: i,
            })
            .collect();
        let io = super::super::hash_mikros::public_io(&view, &[0], &bindings).unwrap();
        let z: Vec<_> = io.blocks[0]
            .values
            .iter()
            .flat_map(|v| [v.c0, v.c1])
            .collect();
        let grouped: Vec<_> = (0..2)
            .flat_map(|g| z.chunks_exact(2).map(move |p| p[g]))
            .collect();
        let witness = TransposedWitness {
            z: z.clone(),
            a: grouped.clone(),
            b: grouped,
        };
        let setup = Setup::new(z.len()).unwrap();
        let matrix = r1cs.csc_lincheck_circuit();
        let (proof, times, mut prover) =
            prove(HashKind::Sha256, &view, matrix, &witness, z, &io, &setup).unwrap();
        let encoded = bincode::serialize(&proof).unwrap();
        let decoded: Proof = bincode::deserialize(&encoded).unwrap();
        let mut verifier = verify(
            HashKind::Sha256,
            &view,
            matrix,
            &io,
            &setup.witness_vk,
            &decoded,
        )
        .unwrap();
        assert_eq!(prover.sample_f128_vec(4), verifier.sample_f128_vec(4));
        assert_eq!(times["pcs_claims"], 1);
        assert_eq!(times["source_claims"], 6);
        assert_eq!(times["recommit_ms"], 0.0);
        assert_eq!(
            tamper(
                HashKind::Sha256,
                &view,
                matrix,
                &io,
                &setup.witness_vk,
                &proof
            )
            .unwrap()
            .len(),
            34
        );
        let wrong_key = VerifierKey::new(512).unwrap();
        assert!(verify(HashKind::Sha256, &view, matrix, &io, &wrong_key, &proof).is_err());
        let mut changed = public.clone();
        changed[129] += F128::new(0, 1 << 63);
        let changed_view = TreeVerifier {
            public: &changed,
            ..view
        };
        let changed_io =
            super::super::hash_mikros::public_io(&changed_view, &[0], &bindings).unwrap();
        assert!(
            verify(
                HashKind::Sha256,
                &changed_view,
                matrix,
                &changed_io,
                &setup.witness_vk,
                &proof
            )
            .is_err()
        );
    }
}
