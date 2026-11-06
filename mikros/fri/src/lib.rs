//! Complete, non-hiding multilinear PCS over the GHASH binary field.
//!
//! The protocol and verifier come from the pinned Binius64 BaseFold implementation. The
//! commitment cache retains the original RS codeword and Merkle tree; opening does not
//! recommit the message. The query target below applies only to the FRI query phase.

use binius_field::{BinaryField128bGhash as B128, Field, PackedField, arch::OptimalPackedB128};
use binius_hash::{StdDigest, StdHashSuite};
use binius_iop::{
    basefold::verify_mlecheck_basefold,
    channel::OracleSpec,
    fri::{FRIParams, calculate_n_test_queries},
    merkle_channel::{TranscriptMerkleCommitment, VerifierMerkleTranscriptChannel},
    merkle_tree::{BinaryMerkleTreeScheme, Commitment as MerkleCommitment},
};
#[cfg(any(test, not(feature = "single-mle")))]
use binius_iop_prover::basefold::prove_mlecheck_basefold;
#[cfg(not(feature = "parallel-encode"))]
use binius_iop_prover::fri::encode_interleaved;
use binius_iop_prover::{
    fri::FRIFoldProver,
    merkle_channel::{MerkleIPProverChannel, ProverMerkleTranscriptChannel},
};
use binius_math::{
    FieldBuffer,
    multilinear::{evaluate::evaluate, fold::fold_highest_var_inplace},
    ntt::{
        NeighborsLastMultiThread,
        domain_context::{GaoMateerOnTheFly, GaoMateerPreExpanded},
    },
};
use binius_transcript::{
    Options, ProverTranscript, VerifierTranscript, fiat_shamir::HasherChallenger,
};
use flock_field::F128;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "parallel-encode")]
mod encode;
#[cfg(all(feature = "fixed-leaf-sha", any(test, target_arch = "x86_64")))]
mod fixed_sha256;
#[cfg(feature = "single-mle")]
mod mle;
mod profiling;

// Keep the native four-way SHA kernel on AArch64; this specialization is measured on x86.
#[cfg(all(feature = "fixed-leaf-sha", target_arch = "x86_64"))]
type ProverHashSuite = fixed_sha256::FixedSha256HashSuite;
#[cfg(not(all(feature = "fixed-leaf-sha", target_arch = "x86_64")))]
type ProverHashSuite = StdHashSuite;

pub const BINIUS_REVISION: &str = "a41ff589ad23ed6e158e997c0ba85ede777fdce8";
pub const LOG_INV_RATE: usize = 1;
pub const QUERY_SECURITY_BITS: usize = 100;
const DOMAIN: &[u8] = b"mikros-packed-ghash-basefold-additive-fri-v1";
type Packed = OptimalPackedB128;
type Challenger = HasherChallenger<StdDigest>;
type Ntt = NeighborsLastMultiThread<GaoMateerPreExpanded<B128>>;
type NativeCommitment = <ProverMerkleTranscriptChannel<
    ProverTranscript<Challenger>,
    Challenger,
    B128,
    ProverHashSuite,
> as MerkleIPProverChannel<B128>>::Commitment;

/// Both implementations use the polynomial basis modulo x^128 + x^7 + x^2 + x + 1.
#[inline]
pub fn to_binius(value: F128) -> B128 {
    B128::new(u128::from(value.lo) | (u128::from(value.hi) << 64))
}

#[inline]
pub fn from_binius(value: B128) -> F128 {
    let bits = u128::from(value);
    F128::new(bits as u64, (bits >> 64) as u64)
}

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

/// The verifier retains only public parameters, never a source vector or prover cache.
#[derive(Debug, Clone)]
pub struct VerifierKey {
    words: usize,
    params: FRIParams<B128>,
    parameter_digest: [u8; 32],
}

impl VerifierKey {
    pub fn new(words: usize) -> Result<Self, String> {
        // The native channel samples query indices as u32. Restrict dimensions before
        // constructing parameters or allocating any prover-owned data.
        if !words.is_power_of_two() || words < 2 || words.ilog2() > 31 {
            return Err("FRI words must be a power of two in [2, 2^31]".into());
        }
        let log_words = words.ilog2() as usize;
        let domain = GaoMateerOnTheFly::<B128>::generate(log_words + LOG_INV_RATE);
        let (params, _) = FRIParams::optimal_for_batch(
            &domain,
            &BinaryMerkleTreeScheme::<B128, StdHashSuite>::new(),
            &[OracleSpec::new(log_words)],
            LOG_INV_RATE,
            calculate_n_test_queries(QUERY_SECURITY_BITS, LOG_INV_RATE),
        );
        let mut key = Self {
            words,
            params,
            parameter_digest: [0; 32],
        };
        key.parameter_digest = *blake3::hash(&key.parameter_bytes()).as_bytes();
        Ok(key)
    }

    pub fn words(&self) -> usize {
        self.words
    }
    pub fn parameter_digest(&self) -> [u8; 32] {
        self.parameter_digest
    }

    pub fn parameter_profile(&self) -> serde_json::Value {
        serde_json::json!({
            "words": self.words,
            "log_words": self.words.ilog2(),
            "log_batch_size": self.params.log_batch_size(),
            "rs_log_dimension": self.params.rs_code().log_dim(),
            "terminal_log_dimension": self.params.n_final_challenges(),
            "fold_arities": self.params.fold_arities(),
            "native_queries": self.params.n_test_queries(),
            "log_inv_rate": LOG_INV_RATE,
        })
    }

    fn parameter_bytes(&self) -> Vec<u8> {
        let mut bytes = DOMAIN.to_vec();
        bytes.extend_from_slice(BINIUS_REVISION.as_bytes());
        bytes.extend_from_slice(b"GHASH-polynomial-basis;Gao-Mateer;SHA256;non-hiding;Boolean-LSB");
        for value in [
            self.words,
            LOG_INV_RATE,
            QUERY_SECURITY_BITS,
            self.params.n_test_queries(),
            self.params.rs_code().log_dim(),
            self.params.log_batch_size(),
            self.params.n_final_challenges(),
            self.params.fold_arities().len(),
        ] {
            bytes.extend_from_slice(&(value as u64).to_le_bytes());
        }
        for &arity in self.params.fold_arities() {
            bytes.extend_from_slice(&(arity as u64).to_le_bytes());
        }
        for &basis in self.params.rs_code().subspace().basis() {
            bytes.extend_from_slice(&u128::from(basis).to_le_bytes());
        }
        bytes
    }

    fn check(&self, commitment: &Commitment, point: &[F128]) -> Result<(), String> {
        if commitment.words != self.words || commitment.parameter_digest != self.parameter_digest {
            return Err("FRI commitment parameters do not match the verifier key".into());
        }
        if point.len() != self.words.ilog2() as usize {
            return Err("FRI opening point has the wrong dimension".into());
        }
        Ok(())
    }
}

/// Source-dependent memory is consumed exactly once by `Source::open`.
pub struct Cache {
    commitment: Commitment,
    message: FieldBuffer<Packed>,
    codeword: FieldBuffer<Packed>,
    tree: NativeCommitment,
}

pub struct Source {
    key: VerifierKey,
    ntt: Ntt,
}

impl Source {
    /// The RS code is deterministic and has no trusted setup or seed-dependent generator.
    pub fn new(words: usize, _seed: u64) -> Result<Self, String> {
        let key = VerifierKey::new(words)?;
        let domain = GaoMateerPreExpanded::generate(key.params.rs_code().log_len());
        let shares = rayon::current_num_threads().next_power_of_two().ilog2() as usize;
        Ok(Self {
            key,
            ntt: NeighborsLastMultiThread::new(domain, shares),
        })
    }

    pub fn verifier_key(&self) -> &VerifierKey {
        &self.key
    }

    pub fn commit(&self, words: &[F128]) -> Result<(Commitment, Cache), String> {
        if words.len() != self.key.words {
            return Err("FRI commitment source length mismatch".into());
        }
        let mut profile = profiling::Stages::new("commit", self.key.words);
        // Construct packed native field words directly, without a second scalar-sized copy.
        let packed = words
            .par_chunks(Packed::WIDTH)
            .map(|chunk| Packed::from_scalars(chunk.iter().copied().map(to_binius)))
            .collect::<Vec<_>>();
        let message = FieldBuffer::new(words.len().ilog2() as usize, packed);
        profile.mark("convert");
        #[cfg(not(feature = "parallel-encode"))]
        let codeword = encode_interleaved(&self.key.params, 0, &self.ntt, message.to_ref());
        #[cfg(feature = "parallel-encode")]
        let codeword =
            encode::encode_interleaved_parallel(&self.key.params, 0, &self.ntt, message.to_ref());
        profile.mark("encode");
        let mut transcript = prover_transcript();
        let mut channel =
            ProverMerkleTranscriptChannel::<_, Challenger, B128, ProverHashSuite>::new(
                &mut transcript,
            );
        let tree = channel
            .send_merkle_commitment(codeword.to_ref(), 1 << self.key.params.log_batch_size());
        drop(channel);
        profile.mark("merkle");
        profile.emit();
        // A native non-hiding SHA-256 commitment serializes exactly its 32-byte root.
        let root: [u8; 32] = transcript
            .finalize()
            .try_into()
            .map_err(|_| "unexpected native FRI root serialization".to_owned())?;
        let commitment = Commitment {
            root,
            words: words.len(),
            parameter_digest: self.key.parameter_digest,
        };
        Ok((
            commitment.clone(),
            Cache {
                commitment,
                message,
                codeword,
                tree,
            },
        ))
    }

    /// `point` is low-variable-first, with Boolean factors (1 + r, r).
    /// The caller supplies the already reduced value. No extra source evaluation is performed.
    pub fn open(
        &self,
        commitment: &Commitment,
        point: &[F128],
        value: F128,
        cache: Cache,
    ) -> Result<Vec<u8>, String> {
        self.key.check(commitment, point)?;
        if cache.commitment != *commitment {
            return Err("FRI opening cache belongs to a different commitment".into());
        }
        let native_point: Vec<_> = point.iter().copied().map(to_binius).collect();
        let mut transcript = prover_transcript();
        transcript
            .observe()
            .write_bytes(&statement(&self.key, commitment, point, value));
        let mut channel =
            ProverMerkleTranscriptChannel::<_, Challenger, B128, ProverHashSuite>::new(
                &mut transcript,
            );
        let folder = FRIFoldProver::new(&self.key.params, &self.ntt, cache.codeword, cache.tree);
        if native_point.contains(&B128::ONE) {
            // The pinned optimized prover divides by 1-alpha, so it cannot recover R(0)
            // when alpha=1. Evaluate both halves in this rare boundary case, retaining
            // exactly the native degree-one message, challenge and FRI schedule.
            prove_boundary(cache.message, &native_point, folder, &mut channel);
        } else if profiling::enabled() {
            profiling::prove_profiled(
                cache.message,
                &native_point,
                to_binius(value),
                &self.key.params,
                folder,
                &mut channel,
            );
        } else {
            #[cfg(feature = "single-mle")]
            mle::prove_specialized(
                cache.message,
                &native_point,
                to_binius(value),
                folder,
                &mut channel,
            );
            #[cfg(not(feature = "single-mle"))]
            prove_mlecheck_basefold(
                cache.message,
                &native_point,
                to_binius(value),
                None,
                &[],
                folder,
                &mut channel,
            );
        }
        drop(channel);
        Ok(transcript.finalize())
    }
}

fn prover_transcript() -> ProverTranscript<Challenger> {
    ProverTranscript::with_opts(
        Challenger::default(),
        Options {
            debug_assertions: false,
        },
    )
}

fn statement(key: &VerifierKey, commitment: &Commitment, point: &[F128], value: F128) -> Vec<u8> {
    let mut bytes = DOMAIN.to_vec();
    bytes.extend_from_slice(&key.parameter_digest);
    bytes.extend_from_slice(&(key.words as u64).to_le_bytes());
    bytes.extend_from_slice(&commitment.root);
    bytes.extend_from_slice(&(point.len() as u64).to_le_bytes());
    for x in point.iter().copied().chain(std::iter::once(value)) {
        bytes.extend_from_slice(&x.lo.to_le_bytes());
        bytes.extend_from_slice(&x.hi.to_le_bytes());
    }
    bytes
}

fn prove_boundary<Channel: MerkleIPProverChannel<B128>>(
    mut message: FieldBuffer<Packed>,
    point: &[B128],
    mut folder: FRIFoldProver<'_, B128, Packed, Ntt, Channel::Commitment>,
    channel: &mut Channel,
) {
    for remaining in (1..=point.len()).rev() {
        let (low, high) = message.split_half_ref();
        let slope =
            evaluate(&low, &point[..remaining - 1]) + evaluate(&high, &point[..remaining - 1]);
        channel.send_one(slope);
        folder.execute_fold_round(channel);
        let challenge = channel.sample();
        fold_highest_var_inplace(&mut message, challenge);
        folder.receive_challenge(challenge);
    }
    folder.execute_fold_round(channel);
    folder.finish_proof(channel);
}

pub fn verify(
    key: &VerifierKey,
    commitment: &Commitment,
    point: &[F128],
    value: F128,
    proof: &[u8],
) -> Result<(), String> {
    key.check(commitment, point)?;
    let native_point: Vec<_> = point.iter().copied().map(to_binius).collect();
    let mut transcript = VerifierTranscript::with_opts(
        Challenger::default(),
        proof.to_vec(),
        Options {
            debug_assertions: false,
        },
    );
    transcript
        .observe()
        .write_bytes(&statement(key, commitment, point, value));
    let mut channel =
        VerifierMerkleTranscriptChannel::<_, Challenger, B128, StdHashSuite>::new(&mut transcript);
    // The fixed external root is already observed in the statement. It is not read from
    // prover-controlled proof bytes and the verifier has no source/cache access.
    let native_commitment = TranscriptMerkleCommitment {
        commitment: MerkleCommitment {
            root: commitment.root.into(),
            depth: key.params.rs_code().log_len(),
        },
        leaf_size: 1 << key.params.log_batch_size(),
    };
    verify_mlecheck_basefold(
        &key.params,
        &[native_commitment],
        to_binius(value),
        &native_point,
        None,
        &[],
        &mut channel,
    )
    .map_err(|e| format!("FRI complete MLE verification: {e}"))?;
    drop(channel);
    transcript
        .finalize()
        .map_err(|e| format!("FRI transcript exhaustion: {e}"))
}

pub fn profile() -> serde_json::Value {
    serde_json::json!({
        "protocol": "BaseFold MLE-check + additive FRI",
        "binius64_revision": BINIUS_REVISION,
        "rate": 0.5, "log_inv_rate": LOG_INV_RATE,
        "query_security_bits": QUERY_SECURITY_BITS,
        "native_queries": calculate_n_test_queries(QUERY_SECURITY_BITS, LOG_INV_RATE),
        "query_security_scope": "FRI query phase only; excludes folding and other protocol errors",
        "field_bits": 128, "packed_word_bits": 128, "zero_knowledge": false,
        "point_order": "low-to-high Boolean MLE", "hash": "SHA-256",
        "native_packing_width": Packed::WIDTH,
        "ntt": "Gao-Mateer pre-expanded, neighbors-last multi-thread",
        "parallel_encoding_copies": cfg!(feature = "parallel-encode"),
        "fixed_length_sha256_leaves": cfg!(all(feature = "fixed-leaf-sha", target_arch = "x86_64")),
        "single_column_mle": cfg!(feature = "single-mle"),
    })
}

#[cfg(test)]
mod tests;
