//! Non-hiding packed GHASH MLE openings using Plonky3's binary WHIR.
//!
//! Input words and query coordinates are transported through the official field
//! isomorphism. The encoder is the additive-domain `PolyBasisNtt`, not an
//! odd-characteristic FFT and not a reinterpretation of GHASH bit coordinates.

mod conversion;

use conversion::{convert_words, from_native, to_native};
use flock_field::F128;
use p3_binary_field::{BinaryChallenger, BinaryField128};
use p3_binary_pcs::whir::{BooleanWhirDomain, recommended_cap_height};
use p3_challenger::HashChallenger;
use p3_commit::{Mmcs, MultilinearPcs};
use p3_matrix::dense::RowMajorMatrix;
use p3_merkle_tree::MerkleTreeMmcs;
use p3_multilinear_util::point::Point;
use p3_sha256::Sha256;
use p3_sumcheck::layout::{Layout, PrefixProver, Table};
use p3_sumcheck::{
    OpeningBatch, OpeningEvals, OpeningProtocol, PrescribedPointPcs, TableShape, TableSpec,
};
use p3_symmetric::{CompressionFunctionFromHasher, SerializingHasher};
use p3_whir::pcs::{WhirProverData, proof::PcsProof};
use p3_whir::{FoldingFactor, ProtocolParameters, SecurityAssumption, WhirConfig, WhirProver};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    sync::Arc,
};

pub const SECURITY_BITS: usize = 100;
pub const LOG_INV_RATE: usize = 1;
pub const MAX_LOG_WORDS: usize = 30;
pub const MAX_POW_BITS: usize = 3;
pub const MAX_PROOF_BYTES: usize = 64 * 1024 * 1024;
pub const UPSTREAM_VERSION: &str = "0.8.0";
const DOMAIN: &[u8] = b"mikros-packed-ghash-p3-binary-whir-v1";
const MAGIC: &[u8; 8] = b"MPWHIR01";
type E = BinaryField128;
type LeafHash = SerializingHasher<Sha256>;
type Compress = CompressionFunctionFromHasher<Sha256, 2, 32>;
type Merkle = MerkleTreeMmcs<E, u8, LeafHash, Compress, 2, 32>;
type Challenger = BinaryChallenger<E, HashChallenger<u8, Sha256, 32>>;
type NativeLayout = PrefixProver<E, E>;
type Backend = WhirProver<E, E, BooleanWhirDomain, Merkle, Challenger, NativeLayout>;
type Config = WhirConfig<E, E, Challenger>;
type NativeCommitment = <Merkle as Mmcs<E>>::Commitment;
type NativeData = WhirProverData<E, E, Merkle, NativeLayout>;
type NativeProof = PcsProof<E, E, Merkle>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Commitment {
    root: [u8; 32],
    words: usize,
    parameter_digest: [u8; 32],
}

impl Commitment {
    pub fn root(&self) -> [u8; 32] {
        self.root
    }
    pub fn words(&self) -> usize {
        self.words
    }
}

/// Public configuration only: no source vector or encoded table.
#[derive(Clone)]
pub struct VerifierKey {
    words: usize,
    backend: Arc<Backend>,
    parameter_digest: [u8; 32],
    cap_height: usize,
}

impl std::fmt::Debug for VerifierKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VerifierKey")
            .field("words", &self.words)
            .field("parameter_digest", &self.parameter_digest)
            .finish()
    }
}

impl VerifierKey {
    pub fn new(words: usize) -> Result<Self, String> {
        if words < 2 || !words.is_power_of_two() || words.trailing_zeros() as usize > MAX_LOG_WORDS
        {
            return Err("WHIR word count must be a power of two in [2, 2^30]".into());
        }
        let log_words = words.trailing_zeros() as usize;
        let domain = BooleanWhirDomain::default();
        // Prefer zero grinding. Large shapes may require up to three bits to
        // retain the requested per-term target; never silently lower it.
        let mut result = None;
        let mut errors = Vec::new();
        for pow_bits in 0..=MAX_POW_BITS {
            let parameters = ProtocolParameters {
                security_level: SECURITY_BITS,
                pow_bits,
                round_log_inv_rates: vec![],
                folding_factor: FoldingFactor::Constant(4.min(log_words)),
                soundness_type: SecurityAssumption::UniqueDecoding,
                starting_log_inv_rate: LOG_INV_RATE,
            };
            match Config::new_with_domain(log_words, parameters, &domain) {
                Ok(config) => {
                    result = Some(config);
                    break;
                }
                Err(error) => errors.push(format!("pow={pow_bits}: {error:?}")),
            }
        }
        let config = result
            .ok_or_else(|| format!("WHIR parameter derivation failed: {}", errors.join("; ")))?;
        if !config.check_pow_bits() || config.max_pow_bits() > MAX_POW_BITS {
            return Err("WHIR derived grinding exceeds explicit budget".into());
        }
        config
            .validate_initial_claims(1 + config.commitment_ood_samples())
            .map_err(|e| format!("WHIR claim budget: {e:?}"))?;
        let cap_height = recommended_cap_height(&config);
        let mmcs = Merkle::new(LeafHash::new(Sha256), Compress::new(Sha256), cap_height);
        let backend = Arc::new(Backend::new(config, domain, mmcs));
        let mut key = Self {
            words,
            backend,
            parameter_digest: [0; 32],
            cap_height,
        };
        let parameters =
            serde_json::to_vec(&(profile(), key.parameter_profile())).map_err(|e| e.to_string())?;
        let mut hash = blake3::Hasher::new();
        hash.update(DOMAIN);
        hash.update(&(parameters.len() as u64).to_le_bytes());
        hash.update(&parameters);
        key.parameter_digest = *hash.finalize().as_bytes();
        Ok(key)
    }

    pub fn words(&self) -> usize {
        self.words
    }
    pub fn parameter_digest(&self) -> [u8; 32] {
        self.parameter_digest
    }
    pub fn parameter_profile(&self) -> Value {
        let config = &self.backend;
        let rounds: Vec<_> = config
            .round_parameters()
            .iter()
            .map(|r| {
                json!({
                    "num_variables":r.num_variables, "folding_factor":r.folding_factor,
                    "log_inv_rate":r.log_inv_rate, "queries":r.num_queries,
                    "ood_samples":r.ood_samples, "query_pow_bits":r.pow_bits,
                    "folding_pow_bits":r.folding_pow_bits,
                    "domain_size":r.domain_size, "log_folded_domain_size":r.log_folded_domain_size,
                })
            })
            .collect();
        let security = config.prescribed_security(&protocol(self.words));
        json!({
            "words":self.words, "log_words":self.words.trailing_zeros(),
            "folding_schedule":config.folding_schedule(), "cap_height":self.cap_height,
            "selected_pow_budget":config.params().pow_bits, "actual_max_pow_bits":config.max_pow_bits(),
            "initial_ood_samples":config.commitment_ood_samples(),
            "initial_folding_pow_bits":config.starting_folding_pow_bits(),
            "rounds":rounds, "terminal_queries":config.terminal().num_queries,
            "terminal_pow_bits":config.terminal().pow_bits,
            "final_sumcheck_rounds":config.final_sumcheck_rounds(),
            "final_folding_pow_bits":config.final_folding_pow_bits(),
            "prescribed_opening_security_bits":security.as_ref().map(|s| s.error().bits()),
            "log2_max_candidates":security.as_ref().map(|s| s.log2_max_candidates),
        })
    }

    fn check(&self, cm: &Commitment, point: &[F128]) -> Result<(), String> {
        if cm.words != self.words || cm.parameter_digest != self.parameter_digest {
            return Err("WHIR commitment shape/profile mismatch".into());
        }
        if point.len() != self.words.trailing_zeros() as usize {
            return Err("WHIR opening point dimension mismatch".into());
        }
        Ok(())
    }
}

pub struct Source {
    key: VerifierKey,
}
pub struct Cache {
    commitment: Commitment,
    native_commitment: NativeCommitment,
    data: NativeData,
}

impl Source {
    pub fn new(words: usize, _seed: u64) -> Result<Self, String> {
        Ok(Self {
            key: VerifierKey::new(words)?,
        })
    }
    pub fn verifier_key(&self) -> &VerifierKey {
        &self.key
    }

    pub fn commit(&self, values: &[F128]) -> Result<(Commitment, Cache), String> {
        if values.len() != self.key.words {
            return Err("WHIR source length mismatch".into());
        }
        // A Table matrix stores one complete multilinear polynomial per row.
        let table = Table::new(RowMajorMatrix::new(convert_words(values), values.len()));
        let witness =
            NativeLayout::new_witness(vec![table], self.key.backend.folding_schedule()[0]);
        let mut challenger = Challenger::from_hasher(self.key.parameter_digest.to_vec(), Sha256);
        let (native_commitment, data) = self
            .key
            .backend
            .commit(witness, &mut challenger)
            .map_err(|e| format!("WHIR commit: {e:?}"))?;
        let commitment = Commitment {
            root: commitment_id(&self.key, &native_commitment)?,
            words: self.key.words,
            parameter_digest: self.key.parameter_digest,
        };
        Ok((
            commitment.clone(),
            Cache {
                commitment,
                native_commitment,
                data,
            },
        ))
    }

    pub fn open(
        &self,
        cm: &Commitment,
        point_lsb: &[F128],
        value: F128,
        cache: Cache,
    ) -> Result<Vec<u8>, String> {
        self.key.check(cm, point_lsb)?;
        if cache.commitment != *cm {
            return Err("WHIR commitment/cache mismatch".into());
        }
        let mut challenger = opening_challenger(&self.key, cm, point_lsb, value);
        // PrescribedPointPcs does not bind the commitment itself. Bind exactly
        // once here and in verify, before all native WHIR challenges.
        self.key
            .backend
            .observe_commitment(&cache.native_commitment, &mut challenger);
        let proof = self
            .key
            .backend
            .open_at(
                cache.data,
                &protocol(self.key.words),
                &[native_point(point_lsb)],
                &mut challenger,
            )
            .map_err(|e| format!("WHIR opening: {e:?}"))?;
        check_evals(&proof.evals, value)?;
        encode(&WireProof {
            commitment: cache.native_commitment,
            proof,
        })
    }
}

pub fn verify(
    key: &VerifierKey,
    cm: &Commitment,
    point_lsb: &[F128],
    value: F128,
    bytes: &[u8],
) -> Result<(), String> {
    key.check(cm, point_lsb)?;
    if bytes.len() > MAX_PROOF_BYTES {
        return Err("WHIR proof exceeds byte limit".into());
    }
    catch_unwind(AssertUnwindSafe(|| {
        let wire = decode(bytes)?;
        if commitment_id(key, &wire.commitment)? != cm.root {
            return Err("WHIR native commitment mismatch".into());
        }
        check_evals(&wire.proof.evals, value)?;
        let mut challenger = opening_challenger(key, cm, point_lsb, value);
        key.backend
            .observe_commitment(&wire.commitment, &mut challenger);
        let evals = key
            .backend
            .verify_at(
                &wire.commitment,
                &wire.proof,
                &protocol(key.words),
                &[native_point(point_lsb)],
                &mut challenger,
            )
            .map_err(|e| format!("WHIR verification: {e:?}"))?;
        check_evals(&evals, value)
    }))
    .map_err(|_| "malformed WHIR proof rejected".to_string())?
}

fn check_evals(evals: &[OpeningEvals<E>], value: F128) -> Result<(), String> {
    if evals.len() != 1
        || evals[0].current().len() != 1
        || !evals[0].next().is_empty()
        || from_native(evals[0].current()[0]) != value
    {
        return Err("WHIR returned opening does not match the claimed value".into());
    }
    Ok(())
}

fn protocol(words: usize) -> OpeningProtocol {
    OpeningProtocol::new(vec![TableSpec::new(
        TableShape::new(words.trailing_zeros() as usize, 1),
        vec![OpeningBatch::new(vec![0], vec![])],
    )])
}

fn native_point(point_lsb: &[F128]) -> Point<E> {
    Point::new(point_lsb.iter().rev().copied().map(to_native).collect())
}

fn opening_challenger(
    key: &VerifierKey,
    cm: &Commitment,
    point: &[F128],
    value: F128,
) -> Challenger {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(DOMAIN);
    bytes.extend_from_slice(&key.parameter_digest);
    bytes.extend_from_slice(&(key.words as u64).to_le_bytes());
    bytes.extend_from_slice(&cm.root);
    bytes.extend_from_slice(&(point.len() as u64).to_le_bytes());
    for v in point.iter().chain(std::iter::once(&value)) {
        bytes.extend_from_slice(&v.lo.to_le_bytes());
        bytes.extend_from_slice(&v.hi.to_le_bytes());
    }
    Challenger::from_hasher(bytes, Sha256)
}

fn commitment_id(key: &VerifierKey, commitment: &NativeCommitment) -> Result<[u8; 32], String> {
    let cap = postcard::to_allocvec(commitment).map_err(|e| e.to_string())?;
    let mut hash = blake3::Hasher::new();
    hash.update(DOMAIN);
    hash.update(b"commitment-cap");
    hash.update(&key.parameter_digest);
    hash.update(&(key.words as u64).to_le_bytes());
    hash.update(&(cap.len() as u64).to_le_bytes());
    hash.update(&cap);
    Ok(*hash.finalize().as_bytes())
}

#[derive(Serialize, Deserialize)]
struct WireProof {
    commitment: NativeCommitment,
    proof: NativeProof,
}

fn encode(wire: &WireProof) -> Result<Vec<u8>, String> {
    let payload = postcard::to_allocvec(wire).map_err(|e| e.to_string())?;
    if payload.len() > MAX_PROOF_BYTES - 16 {
        return Err("WHIR proof exceeds byte limit".into());
    }
    let mut bytes = Vec::with_capacity(payload.len() + 16);
    bytes.extend_from_slice(MAGIC);
    bytes.extend_from_slice(&(payload.len() as u64).to_le_bytes());
    bytes.extend(payload);
    Ok(bytes)
}

fn decode(bytes: &[u8]) -> Result<WireProof, String> {
    if bytes.len() < 16 || bytes.len() > MAX_PROOF_BYTES || &bytes[..8] != MAGIC {
        return Err("invalid WHIR proof header".into());
    }
    let length = u64::from_le_bytes(bytes[8..16].try_into().unwrap());
    if length != (bytes.len() - 16) as u64 {
        return Err("WHIR proof length/trailing data mismatch".into());
    }
    let (wire, tail): (WireProof, _) =
        postcard::take_from_bytes(&bytes[16..]).map_err(|e| format!("WHIR proof decode: {e}"))?;
    if !tail.is_empty() || postcard::to_allocvec(&wire).map_err(|e| e.to_string())? != bytes[16..] {
        return Err("non-canonical WHIR proof encoding".into());
    }
    Ok(wire)
}

pub fn profile() -> Value {
    json!({
        "protocol":"Plonky3 binary WHIR prescribed Boolean MLE", "upstream_version":UPSTREAM_VERSION,
        "rate":0.5, "log_inv_rate":LOG_INV_RATE, "security_target_per_term_bits":SECURITY_BITS,
        "security_scope":"upstream per-term target; composed prescribed-opening estimate is reported separately; excludes outer PLONK and hash/Fiat-Shamir assumptions",
        "decoding":"unique", "maximum_allowed_pow_bits":MAX_POW_BITS, "pow_policy":"smallest accepted budget in 0..=3, with all actual slots reported per source",
        "field_bits":128, "code_field_bits":128, "packed_word_bits":128, "zero_knowledge":false,
        "field_conversion":"official GHASH polynomial-basis to BinaryField128 tower isomorphism",
        "point_order":"low-to-high Boolean MLE; reversed for native MSB-first point",
        "hash":"SHA-256", "commitment_id_hash":"BLAKE3",
        "encoder":"BooleanWhirDomain / PolyBasisNtt additive NTT",
        "commitment":"parameter-bound digest of native Merkle cap; cap included in opening bytes",
    })
}

#[cfg(test)]
mod tests;
