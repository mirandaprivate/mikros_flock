//! RAAFold authentication of Mikros's two canonical packed-word MLEs.

use ark_std::rand::{SeedableRng, rngs::StdRng};
use flock_core::pcs::functional::TensorClaim;
use flock_field::F128;
use mikros_raafold::{
    ghash,
    raafold_short::{
        ShortRaaFoldCommitment, ShortRaaFoldCommitmentCache, ShortRaaPackedShape,
        ShortRaaPackedWitness,
    },
    raafold_short_pc::{
        ShortRaaFoldPC, ShortRaaFoldParameters, ShortRaaFoldProof, ShortRaaFoldProverSetup,
    },
};

pub type Commitment = ShortRaaFoldCommitment;
pub type Cache = ShortRaaFoldCommitmentCache<u128>;
pub const COLUMNS: usize = 128;
pub const SWITCH_RATIO: usize = 8;
pub const QUERY_SECURITY_BITS: usize = 100;
pub const BACKEND_NAME: &str = "RAAFold RAA-4 equality";
pub const DISPLAY_NAME: &str = "RAAFold";
pub const TRANSCRIPT_DOMAIN: &[u8] = b"mikros-binary-plonk-raafold-equality-v1";
pub const WITNESS_PROOF_LABEL: &[u8] = b"raafold-witness-proof";
pub const PUBLIC_PROOF_LABEL: &[u8] = b"raafold-public-proof";
pub const OFFLINE_SETUP_STAGE: &str = "offline_raafold_setup";
pub const DEFAULT_LOG_GATES: usize = 30;
pub const DISTANCE_CERTIFIED: Option<bool> = Some(false);

pub fn profile() -> serde_json::Value {
    serde_json::json!({"accumulators":2,"repetition":4,"rate":0.25,"distance":0.19,
        "distance_status":"assumed; finite-length setup failure not certified",
        "query_security_bits":QUERY_SECURITY_BITS,"packed_word_bits":128,
        "columns":COLUMNS,"switch_ratio":SWITCH_RATIO})
}

pub struct VerifierKey {
    pc: ShortRaaFoldPC,
    params: ShortRaaFoldParameters,
}

impl VerifierKey {
    pub fn words(&self) -> usize {
        self.params.n_witness()
    }

    pub fn parameter_digest(&self) -> [u8; 32] {
        self.params.parameter_digest()
    }

    pub fn parameter_profile(&self) -> serde_json::Value {
        serde_json::json!({"words":self.words(),"logical_element_bits":128,
            "columns":COLUMNS,"switch_ratio":SWITCH_RATIO,
            "query_security_bits":QUERY_SECURITY_BITS,"profile":profile()})
    }
}

pub struct Source {
    key: VerifierKey,
    prover_setup: ShortRaaFoldProverSetup,
}

impl Source {
    pub fn new(words: usize, seed: u64) -> Result<Self, String> {
        let pc = ShortRaaFoldPC::new(words, COLUMNS, 128, SWITCH_RATIO, QUERY_SECURITY_BITS)
            .map_err(|e| format!("RAAFold parameters: {e:?}"))?;
        let mut rng = StdRng::seed_from_u64(seed);
        let (params, prover_setup) = pc
            .setup(&mut rng)
            .map_err(|e| format!("RAAFold setup: {e:?}"))?;
        Ok(Self {
            key: VerifierKey { pc, params },
            prover_setup,
        })
    }

    pub fn verifier_key(&self) -> &VerifierKey {
        &self.key
    }

    pub fn commit(&self, words: &[F128]) -> Result<(Commitment, Cache), String> {
        // Conversion is part of commit timing. Ownership avoids another full copy.
        let mapped = ghash::to_packed_words(words);
        let shape = ShortRaaPackedShape::new(words.len() / COLUMNS, COLUMNS, 128)
            .map_err(|e| format!("RAAFold packed shape: {e:?}"))?;
        let packed = ShortRaaPackedWitness::from_packed_words(shape, mapped)
            .map_err(|e| format!("RAAFold packed witness: {e:?}"))?;
        self.key
            .pc
            .commit_packed(&self.key.params, packed)
            .map_err(|e| format!("RAAFold commit: {e:?}"))
    }

    pub fn open(
        &self,
        cm: &Commitment,
        claim: &TensorClaim,
        cache: Cache,
    ) -> Result<Vec<u8>, String> {
        let point = checked_point(claim, cm.logical_elements())?;
        let (value, proof) = self
            .key
            .pc
            .open(&self.key.params, &self.prover_setup, cm, &point, cache)
            .map_err(|e| format!("RAAFold opening+Decide: {e:?}"))?;
        if value != ghash::to_polyval(claim.value) {
            return Err("RAAFold terminal value differs from the packed-opening claim".into());
        }
        proof
            .to_bytes()
            .map_err(|e| format!("RAAFold proof encoding: {e:?}"))
    }
}

fn checked_point(
    claim: &TensorClaim,
    words: usize,
) -> Result<Vec<mikros_raafold::raafold::BinaryExtensionField>, String> {
    if !words.is_power_of_two()
        || claim.rows.len() != words.ilog2() as usize
        || claim.columns.len() != 128
    {
        return Err("RAAFold terminal dimensions".into());
    }
    for (bit, &column) in claim.columns.iter().enumerate() {
        let expected = if bit < 64 {
            F128::new(1u64 << bit, 0)
        } else {
            F128::new(0, 1u64 << (bit - 64))
        };
        if column != expected {
            return Err("RAAFold terminal must use the canonical GHASH word basis".into());
        }
    }
    let mut low_point = Vec::with_capacity(claim.rows.len());
    for &[a, b] in &claim.rows {
        if a + b != F128::ONE {
            return Err("RAAFold terminal must be an equality MLE".into());
        }
        low_point.push(b);
    }
    Ok(ghash::boolean_point_lsb_to_msb(&low_point))
}

pub fn verify(
    key: &VerifierKey,
    cm: &Commitment,
    claim: &TensorClaim,
    encoded: &[u8],
) -> Result<(), String> {
    let params = &key.params;
    if cm.logical_elements() != params.n_witness()
        || cm.logical_element_bits() != 128
        || cm.columns() != COLUMNS
        || cm.word_bits() != 128
        || cm.generator_digest() != params.sequence().generator_digest()
    {
        return Err("RAAFold commitment configuration mismatch".into());
    }
    let point = checked_point(claim, params.n_witness())?;
    let proof = ShortRaaFoldProof::from_bytes(encoded)
        .map_err(|e| format!("RAAFold proof decoding: {e:?}"))?;
    if !key
        .pc
        .verify(params, cm, &point, ghash::to_polyval(claim.value), &proof)
        .map_err(|e| format!("RAAFold verification: {e:?}"))?
    {
        return Err("RAAFold complete opening verification rejected".into());
    }
    Ok(())
}
