//! WHIR authentication of public linear forms over GHASH F128.
//! Adapted from ../mikros/mikros/whir/src/pcs.rs, retaining commitment replay,
//! final-form verification, canonical parsing and both transcript EOF checks.
use crate::{Digest, Error, whir_field::Ghash, whir_rs};
use flock_field::F128;
use serde::{Deserialize, Serialize};
use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    sync::Arc,
};
use whir::{
    algebra::{
        embedding::Identity,
        linear_form::{Covector, LinearForm},
    },
    bits::Bits,
    buffer::Buffer,
    hash,
    parameters::ProtocolParameters,
    protocols::{
        params::DecodingRegime,
        whir::{Config, Witness},
    },
    transcript::{
        DomainSeparator, Proof as BackendProof, ProverState, VerifierState, codecs::Empty,
    },
};

const MAGIC: &[u8; 8] = b"LWHIR001";
pub const MAX_PROOF_BYTES: usize = 64 * 1024 * 1024;
pub const MAX_CLAIMS: usize = 32;
pub const MAX_NUM_VARS: usize = 20;
type Backend = Config<Identity<Ghash>>;
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parameters {
    pub num_vars: usize,
    pub security_bits: usize,
    pub log_inv_rate: usize,
    pub folding_factor: usize,
}
impl Parameters {
    pub fn diagnostic(num_vars: usize) -> Self {
        Self {
            num_vars,
            security_bits: 96,
            log_inv_rate: 1,
            folding_factor: 4.min(num_vars),
        }
    }
    fn validate(&self) -> Result<(), Error> {
        if !(1..=MAX_NUM_VARS).contains(&self.num_vars)
            || !(1..=96).contains(&self.security_bits)
            || self.log_inv_rate != 1
            || !(1..=4.min(self.num_vars)).contains(&self.folding_factor)
        {
            return Err(Error::Capacity);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Commitment {
    pub root: Digest,
    pub parameters: Parameters,
}
pub struct ProverData {
    commitment: Commitment,
    values: Vec<F128>,
    buffer: Buffer<Ghash>,
    witness: Witness<Ghash, Identity<Ghash>>,
    commit_proof: BackendProof,
}
impl ProverData {
    pub fn commitment(&self) -> &Commitment {
        &self.commitment
    }
    pub fn values(&self) -> &[F128] {
        &self.values
    }
}
#[derive(Clone, Debug)]
pub struct Pcs {
    parameters: Parameters,
    backend: Arc<Backend>,
    profile: Digest,
}
impl Pcs {
    pub fn new(parameters: Parameters) -> Result<Self, Error> {
        parameters.validate()?;
        whir_rs::register();
        let backend = catch_unwind(AssertUnwindSafe(|| {
            Backend::new(
                1 << parameters.num_vars,
                &ProtocolParameters {
                    decoding_regime: DecodingRegime::Unique,
                    starting_log_inv_rate: parameters.log_inv_rate,
                    initial_folding_factor: parameters.folding_factor,
                    folding_factor: parameters.folding_factor,
                    security_level: parameters.security_bits,
                    pow_bits: 0,
                    batch_size: 1,
                    hash_id: hash::BLAKE3,
                },
            )
        }))
        .map_err(|_| Error::Capacity)?;
        if !backend.check_max_pow_bits(Bits::new(0.))
            || backend.initial_skip_pow.difficulty() > Bits::new(0.)
            || backend.security_level(1, MAX_CLAIMS) + 1e-9 < parameters.security_bits as f64
        {
            return Err(Error::Capacity);
        }
        let mut h = blake3::Hasher::new();
        h.update(b"lookup-WHIR-GHASH-x128+x7+x2+x+1-additive-raw-index-monomial-RS-v1");
        h.update(&DomainSeparator::protocol(&backend).protocol_id);
        let profile = *h.finalize().as_bytes();
        Ok(Self {
            parameters,
            backend: Arc::new(backend),
            profile,
        })
    }
    pub fn parameters(&self) -> &Parameters {
        &self.parameters
    }
    pub fn diagnostic_security_bits(&self, claims: usize) -> Result<f64, Error> {
        if !(1..=MAX_CLAIMS).contains(&claims) {
            return Err(Error::ProofShape);
        }
        Ok(self.backend.security_level(1, claims))
    }
    pub fn commit(&self, values: Vec<F128>) -> Result<ProverData, Error> {
        if values.len() != 1usize << self.parameters.num_vars {
            return Err(Error::Length);
        }
        let buffer = Buffer::from(
            values
                .iter()
                .copied()
                .map(Ghash::from_flock)
                .collect::<Vec<_>>(),
        );
        let ds = DomainSeparator::protocol(self.backend.as_ref())
            .session(&("lookup-whir-commit-v1", self.profile))
            .instance(&Empty);
        let mut state = ProverState::new_std(&ds);
        let witness = self.backend.commit(&mut state, &[&buffer]);
        let commit_proof = state.proof();
        let commitment = Commitment {
            root: self.commitment_id(&commit_proof),
            parameters: self.parameters.clone(),
        };
        Ok(ProverData {
            commitment,
            values,
            buffer,
            witness,
            commit_proof,
        })
    }
    /// Coefficients of each linear form in flat table order. These are public
    /// query weights, not witness values or values supplied by the proof.
    pub fn open(
        &self,
        data: &ProverData,
        weights: &[Vec<F128>],
        values: &[F128],
        context: &[u8],
    ) -> Result<Vec<u8>, Error> {
        self.check_queries(weights, values)?;
        if data.commitment.parameters != self.parameters {
            return Err(Error::Statement);
        }
        for (w, &value) in weights.iter().zip(values) {
            if w.iter()
                .zip(&data.values)
                .fold(F128::ZERO, |s, (&a, &b)| s + a * b)
                != value
            {
                return Err(Error::Source);
            }
        }
        let session = self.session(&data.commitment, weights, values, context);
        let ds = DomainSeparator::protocol(self.backend.as_ref())
            .session(&("lookup-whir-open-v1", session))
            .instance(&Empty);
        let mut state = ProverState::new_std(&ds);
        let _ = self.backend.prove(
            &mut state,
            &[&data.buffer],
            vec![&data.witness],
            forms(weights),
            Buffer::from(
                values
                    .iter()
                    .copied()
                    .map(Ghash::from_flock)
                    .collect::<Vec<_>>(),
            ),
        );
        serialize(&data.commit_proof, &state.proof())
    }
    /// Verifier accepts no witness, original source tables or prover cache.
    pub fn verify(
        &self,
        commitment: &Commitment,
        weights: &[Vec<F128>],
        values: &[F128],
        proof: &[u8],
        context: &[u8],
    ) -> Result<(), Error> {
        self.check_queries(weights, values)?;
        if commitment.parameters != self.parameters {
            return Err(Error::Statement);
        }
        if proof.len() > MAX_PROOF_BYTES {
            return Err(Error::Length);
        }
        catch_unwind(AssertUnwindSafe(|| {
            self.verify_inner(commitment, weights, values, proof, context)
        }))
        .unwrap_or(Err(Error::Pcs))
    }
    fn verify_inner(
        &self,
        commitment: &Commitment,
        weights: &[Vec<F128>],
        values: &[F128],
        proof: &[u8],
        context: &[u8],
    ) -> Result<(), Error> {
        let (commit, opening) = deserialize(proof)?;
        if self.commitment_id(&commit) != commitment.root {
            return Err(Error::Pcs);
        }
        let ds = DomainSeparator::protocol(self.backend.as_ref())
            .session(&("lookup-whir-commit-v1", self.profile))
            .instance(&Empty);
        let mut state = VerifierState::new_std(&ds, &commit);
        let authenticated = self
            .backend
            .receive_commitment(&mut state)
            .map_err(|_| Error::Pcs)?;
        state.check_eof().map_err(|_| Error::Pcs)?;
        let session = self.session(commitment, weights, values, context);
        let ds = DomainSeparator::protocol(self.backend.as_ref())
            .session(&("lookup-whir-open-v1", session))
            .instance(&Empty);
        let mut state = VerifierState::new_std(&ds, &opening);
        let values: Vec<_> = values.iter().copied().map(Ghash::from_flock).collect();
        let final_claim = self
            .backend
            .verify(&mut state, &[&authenticated], &values)
            .map_err(|_| Error::Pcs)?;
        let forms = forms(weights);
        final_claim
            .verify(forms.iter().map(|f| f.as_ref()))
            .map_err(|_| Error::Pcs)?;
        state.check_eof().map_err(|_| Error::Pcs)
    }
    fn check_queries(&self, weights: &[Vec<F128>], values: &[F128]) -> Result<(), Error> {
        if !(1..=MAX_CLAIMS).contains(&weights.len())
            || weights.len() != values.len()
            || weights
                .iter()
                .any(|w| w.len() != 1usize << self.parameters.num_vars)
        {
            return Err(Error::ProofShape);
        }
        Ok(())
    }
    fn commitment_id(&self, proof: &BackendProof) -> Digest {
        let mut h = blake3::Hasher::new();
        h.update(b"lookup-whir-commitment-id-v1");
        h.update(&self.profile);
        hash_bytes(&mut h, &proof.narg_string);
        hash_bytes(&mut h, &proof.hints);
        *h.finalize().as_bytes()
    }
    fn session(
        &self,
        commitment: &Commitment,
        weights: &[Vec<F128>],
        values: &[F128],
        context: &[u8],
    ) -> Digest {
        let mut h = blake3::Hasher::new();
        h.update(b"lookup-whir-ordered-linear-openings-v1");
        h.update(&self.profile);
        h.update(&commitment.root);
        hash_bytes(&mut h, context);
        h.update(&(weights.len() as u64).to_le_bytes());
        for (w, &v) in weights.iter().zip(values) {
            h.update(&(w.len() as u64).to_le_bytes());
            for &x in w {
                h.update(&Ghash::from_flock(x).bytes());
            }
            h.update(&Ghash::from_flock(v).bytes());
        }
        *h.finalize().as_bytes()
    }
}
fn forms(weights: &[Vec<F128>]) -> Vec<Box<dyn LinearForm<Ghash>>> {
    weights
        .iter()
        .map(|w| {
            Box::new(Covector::new(
                w.iter().copied().map(Ghash::from_flock).collect(),
            )) as Box<dyn LinearForm<Ghash>>
        })
        .collect()
}
fn hash_bytes(h: &mut blake3::Hasher, bytes: &[u8]) {
    h.update(&(bytes.len() as u64).to_le_bytes());
    h.update(bytes);
}
fn serialize(commit: &BackendProof, opening: &BackendProof) -> Result<Vec<u8>, Error> {
    let parts = [
        &commit.narg_string,
        &commit.hints,
        &opening.narg_string,
        &opening.hints,
    ];
    let len = parts
        .iter()
        .try_fold(40usize, |n, p| n.checked_add(p.len()))
        .ok_or(Error::Length)?;
    if len > MAX_PROOF_BYTES {
        return Err(Error::Length);
    }
    let mut bytes = Vec::with_capacity(len);
    bytes.extend_from_slice(MAGIC);
    for part in parts {
        bytes.extend_from_slice(&(part.len() as u64).to_le_bytes());
        bytes.extend_from_slice(part);
    }
    Ok(bytes)
}
fn deserialize(mut bytes: &[u8]) -> Result<(BackendProof, BackendProof), Error> {
    if bytes.len() > MAX_PROOF_BYTES || bytes.get(..8) != Some(MAGIC) {
        return Err(Error::Pcs);
    }
    bytes = &bytes[8..];
    let mut parts = Vec::with_capacity(4);
    for _ in 0..4 {
        let head: [u8; 8] = bytes
            .get(..8)
            .ok_or(Error::Pcs)?
            .try_into()
            .map_err(|_| Error::Pcs)?;
        bytes = &bytes[8..];
        let len = usize::try_from(u64::from_le_bytes(head)).map_err(|_| Error::Length)?;
        // Bounds before allocation: never trust a prover-supplied vector length.
        let part = bytes.get(..len).ok_or(Error::Pcs)?.to_vec();
        bytes = &bytes[len..];
        parts.push(part);
    }
    if !bytes.is_empty() {
        return Err(Error::Pcs);
    }
    let mut parts = parts.into_iter();
    Ok((
        BackendProof::from_raw_parts(parts.next().unwrap(), parts.next().unwrap()),
        BackendProof::from_raw_parts(parts.next().unwrap(), parts.next().unwrap()),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn both_transcripts_reject_trailing_narg_and_hints() {
        let pcs = Pcs::new(Parameters::diagnostic(3)).unwrap();
        let data = pcs.commit(vec![F128::ZERO; 8]).unwrap();
        let weights = vec![vec![F128::ONE; 8]];
        let proof = pcs
            .open(&data, &weights, &[F128::ZERO], b"strict-wire")
            .unwrap();
        pcs.verify(
            data.commitment(),
            &weights,
            &[F128::ZERO],
            &proof,
            b"strict-wire",
        )
        .unwrap();
        for stream in 0..4 {
            let (mut commit, mut opening) = deserialize(&proof).unwrap();
            match stream {
                0 => commit.narg_string.push(0),
                1 => commit.hints.push(0),
                2 => opening.narg_string.push(0),
                _ => opening.hints.push(0),
            }
            let mut commitment = data.commitment().clone();
            // Recompute ID for the malformed commitment-only stream, so the
            // test reaches transcript parsing rather than just hash mismatch.
            commitment.root = pcs.commitment_id(&commit);
            let corrupted = serialize(&commit, &opening).unwrap();
            assert!(
                pcs.verify(
                    &commitment,
                    &weights,
                    &[F128::ZERO],
                    &corrupted,
                    b"strict-wire"
                )
                .is_err(),
                "stream {stream}"
            );
        }
        let mut oversized = proof.clone();
        oversized[8..16].copy_from_slice(&u64::MAX.to_le_bytes());
        assert!(
            pcs.verify(
                data.commitment(),
                &weights,
                &[F128::ZERO],
                &oversized,
                b"strict-wire"
            )
            .is_err()
        );
        assert!(
            pcs.verify(data.commitment(), &[], &[], &proof, b"strict-wire")
                .is_err()
        );
    }
}
