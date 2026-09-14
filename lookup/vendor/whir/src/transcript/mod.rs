//! Wrapper around Spongefish to add out-of-band hint messages.
//!
//! We need these for the Merkle tree proofs as doing them in-transcript
//! would roughly double the verifier cost.

pub mod codecs;
mod mock_sponge;

#[cfg(debug_assertions)]
use std::any::type_name;
use std::fmt::Debug;

use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::rand::{rngs::StdRng, CryptoRng, RngCore};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256, Sha3_512};
use spongefish::StdHash;
pub use spongefish::{
    Codec, Decoding, DuplexSpongeInterface, Encoding, NargDeserialize, NargSerialize,
    VerificationError, VerificationResult,
};

#[cfg(test)]
pub use self::mock_sponge::MockSponge;

#[macro_export]
macro_rules! verify {
    ($cond:expr) => {
        #[allow(clippy::neg_cmp_op_on_partial_ord)]
        if !$cond {
            #[cfg(feature = "verifier_panics")]
            panic!("Verification failed: {}", stringify!($cond));

            #[cfg(not(feature = "verifier_panics"))]
            return Err(spongefish::VerificationError);
        };
    };
}

/// Marker trait for types that can be used as prover messages.
///
/// Like [`spongefish::Codec`], but without the [`Encoding<T>`] requirement.
pub trait ProverMessage<U = [u8]>: NargDeserialize + NargSerialize + Encoding<U>
where
    U: ?Sized,
{
}

#[derive(Clone, Copy, Debug)]
pub struct DomainSeparator<'a, I> {
    pub protocol_id: [u8; 64],
    pub session_id: [u8; 32],
    instance: &'a I,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Interaction {
    ProverMessage(String),
    VerifierMessage(String),
    Hint(String),
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub struct Proof {
    pub narg_string: Vec<u8>,
    pub hints: Vec<u8>,

    #[cfg(debug_assertions)]
    #[serde(skip)]
    pub pattern: Option<Vec<Interaction>>,
}

impl Proof {
    /// Reconstruct the actual wire proof. Debug interaction traces are local
    /// diagnostics, never authenticated proof data or a serialization format.
    pub fn from_raw_parts(narg_string: Vec<u8>, hints: Vec<u8>) -> Self {
        Self {
            narg_string,
            hints,
            #[cfg(debug_assertions)]
            pattern: None,
        }
    }
}

pub struct ProverState<H = StdHash, R = StdRng>
where
    H: DuplexSpongeInterface,
    R: RngCore + CryptoRng,
{
    inner: spongefish::ProverState<H, R>,
    hints: Vec<u8>,

    #[cfg(debug_assertions)]
    pattern: Vec<Interaction>,
}

pub struct VerifierState<'a, H = StdHash>
where
    H: DuplexSpongeInterface,
{
    inner: spongefish::VerifierState<'a, H>,
    /// Independently parsed NARG suffix used to reject noncanonical encodings
    /// before Spongefish normalizes and absorbs their decoded values.
    canonical_remaining: &'a [u8],
    hints: &'a [u8],

    #[cfg(debug_assertions)]
    pattern: Option<&'a [Interaction]>,
}

pub trait VerifierMessage {
    type U;

    fn verifier_message<T>(&mut self) -> T
    where
        T: Decoding<[Self::U]>;

    fn verifier_message_vec<T>(&mut self, count: usize) -> Vec<T>
    where
        T: Decoding<[Self::U]>,
    {
        (0..count).map(|_| self.verifier_message()).collect()
    }
}

impl DomainSeparator<'static, ()> {
    pub fn protocol<C: Serialize>(config: &C) -> Self {
        const INSTANCE: &() = &();
        let mut hash = Sha3_512::new();
        ciborium::into_writer(config, &mut hash).expect("Computing protocol hash failed");
        let protocol_id: [u8; 64] = hash.finalize().into();
        Self {
            protocol_id,
            session_id: [0; 32],
            instance: INSTANCE,
        }
    }

    #[must_use]
    pub fn session<S: Serialize>(self, session: &S) -> Self {
        let mut hash = Sha3_256::new();
        ciborium::into_writer(session, &mut hash).expect("Computing session hash failed");
        let session_id: [u8; 32] = hash.finalize().into();
        Self { session_id, ..self }
    }

    pub const fn instance<I>(self, instance: &I) -> DomainSeparator<'_, I> {
        DomainSeparator {
            protocol_id: self.protocol_id,
            session_id: self.session_id,
            instance,
        }
    }
}

impl<T, U> ProverMessage<U> for T
where
    T: NargSerialize + NargDeserialize + Encoding<U>,
    U: ?Sized,
{
}

impl<H> ProverState<H, StdRng>
where
    H: DuplexSpongeInterface,
{
    /// Construct a new prover state with a custom duplex hash function.
    ///
    /// **Note.** The `spongefish` API currently does not allow creating an
    /// instance with a non-standard random number generator.
    pub fn new<I>(ds: &DomainSeparator<'_, I>, duplex: H) -> Self
    where
        u8: Encoding<[H::U]>,
        I: Encoding<[H::U]>,
    {
        Self {
            inner: spongefish::DomainSeparator::new(ds.protocol_id)
                .session(ds.session_id)
                .instance(ds.instance)
                .to_prover(duplex),
            hints: Vec::new(),

            #[cfg(debug_assertions)]
            pattern: Vec::new(),
        }
    }
}

impl ProverState<StdHash, StdRng> {
    /// Construct a new prover state with the standard duplex hash function.
    pub fn new_std<I>(ds: &DomainSeparator<'_, I>) -> Self
    where
        I: Encoding<[u8]>,
    {
        Self::new(ds, StdHash::default())
    }
}

impl<H, R> ProverState<H, R>
where
    H: DuplexSpongeInterface,
    R: RngCore + CryptoRng,
{
    /// Access the prover's private transcript-bound RNG.
    pub fn rng(&mut self) -> &mut (impl RngCore + CryptoRng) {
        self.inner.rng()
    }

    #[cfg_attr(test, track_caller)]
    pub fn prover_message<T>(&mut self, message: &T)
    where
        T: Encoding<[H::U]> + NargSerialize + ?Sized,
    {
        #[cfg(debug_assertions)]
        self.push(Interaction::ProverMessage(type_name::<T>().to_owned()));
        self.inner.prover_message(message);
    }

    #[cfg_attr(test, track_caller)]
    pub fn prover_messages<T>(&mut self, messages: &[T])
    where
        T: Encoding<[H::U]> + NargSerialize,
    {
        for message in messages {
            self.prover_message(message);
        }
    }

    #[cfg_attr(test, track_caller)]
    pub fn prover_hint<T>(&mut self, hint: &T)
    where
        T: NargSerialize,
    {
        #[cfg(debug_assertions)]
        self.push(Interaction::Hint(type_name::<T>().to_owned()));
        hint.serialize_into_narg(&mut self.hints);
    }

    #[cfg_attr(test, track_caller)]
    pub fn prover_hint_ark<T>(&mut self, value: &T)
    where
        T: CanonicalSerialize + ?Sized,
    {
        #[cfg(debug_assertions)]
        self.push(Interaction::Hint(type_name::<T>().to_owned()));
        value
            .serialize_compressed(&mut self.hints)
            .expect("Failed to serialize hint");
    }

    pub fn proof(self) -> Proof {
        Proof {
            narg_string: self.inner.narg_string().to_owned(),
            hints: self.hints,

            #[cfg(debug_assertions)]
            pattern: Some(self.pattern),
        }
    }

    #[cfg(debug_assertions)]
    fn push(&mut self, interaction: Interaction) {
        self.pattern.push(interaction);
    }
}

impl<H, R> VerifierMessage for ProverState<H, R>
where
    H: DuplexSpongeInterface,
    R: RngCore + CryptoRng,
{
    type U = H::U;

    #[cfg_attr(test, track_caller)]
    fn verifier_message<T>(&mut self) -> T
    where
        T: Decoding<[H::U]>,
    {
        #[cfg(debug_assertions)]
        self.push(Interaction::VerifierMessage(type_name::<T>().to_owned()));
        self.inner.verifier_message()
    }
}

impl<'a, H> VerifierState<'a, H>
where
    H: DuplexSpongeInterface,
{
    pub fn new<'b, I>(ds: &DomainSeparator<'b, I>, proof: &'a Proof, duplex: H) -> Self
    where
        u8: Encoding<[H::U]>,
        I: Encoding<[H::U]>,
    {
        Self {
            inner: spongefish::DomainSeparator::new(ds.protocol_id)
                .session(ds.session_id)
                .instance(ds.instance)
                .to_verifier(duplex, &proof.narg_string),
            canonical_remaining: &proof.narg_string,
            hints: &proof.hints,
            #[cfg(debug_assertions)]
            pattern: proof.pattern.as_deref(),
        }
    }

    pub const fn as_spongefish(&mut self) -> &mut spongefish::VerifierState<'a, H> {
        &mut self.inner
    }

    #[cfg_attr(debug_assertions, track_caller)]
    pub fn check_eof(self) -> VerificationResult<()> {
        #[cfg(debug_assertions)]
        assert!(self.pattern.is_none_or(<[Interaction]>::is_empty));
        verify!(self.inner.check_eof().is_ok());
        verify!(self.canonical_remaining.is_empty());
        verify!(self.hints.is_empty());
        Ok(())
    }

    #[cfg_attr(test, track_caller)]
    pub fn prover_message<T>(&mut self) -> VerificationResult<T>
    where
        T: Encoding<[H::U]> + NargDeserialize + NargSerialize,
    {
        #[cfg(debug_assertions)]
        self.pop_pattern(&Interaction::ProverMessage(type_name::<T>().to_owned()));
        let encoded = self.canonical_remaining;
        let decoded = T::deserialize_from_narg(&mut self.canonical_remaining)?;
        let consumed = encoded.len() - self.canonical_remaining.len();
        let canonical = decoded.serialize_into_new_narg();
        verify!(canonical.as_ref() == &encoded[..consumed]);
        self.inner.prover_message()
    }

    #[cfg_attr(test, track_caller)]
    pub fn prover_messages_vec<T>(&mut self, len: usize) -> VerificationResult<Vec<T>>
    where
        T: Encoding<[H::U]> + NargDeserialize + NargSerialize,
    {
        (0..len).map(|_| self.prover_message()).collect()
    }

    #[cfg_attr(test, track_caller)]
    pub fn prover_hint<T>(&mut self) -> VerificationResult<T>
    where
        T: NargDeserialize,
    {
        #[cfg(debug_assertions)]
        self.pop_pattern(&Interaction::Hint(type_name::<T>().to_owned()));
        T::deserialize_from_narg(&mut self.hints)
    }

    #[cfg_attr(test, track_caller)]
    pub fn prover_hint_ark<T>(&mut self) -> VerificationResult<T>
    where
        T: CanonicalDeserialize,
    {
        #[cfg(debug_assertions)]
        self.pop_pattern(&Interaction::Hint(type_name::<T>().to_owned()));
        T::deserialize_compressed(&mut self.hints).map_err(|_| VerificationError)
    }

    /// Read an IRS field-row hint using a length fixed by the verifier's
    /// parameters and sampled query count, never by the untrusted proof.
    /// Arkworks' generic Vec decoder allocates its wire-declared length before
    /// reading elements, so it must not be used at this boundary.
    pub fn prover_hint_ark_vec_exact<T: ark_ff::Field>(
        &mut self,
        expected_len: usize,
    ) -> VerificationResult<Vec<T>> {
        #[cfg(debug_assertions)]
        self.pop_pattern(&Interaction::Hint(type_name::<Vec<T>>().to_owned()));
        verify!(self.hints.len() >= 8);
        let (length_bytes, body) = self.hints.split_at(8);
        let declared_u64 =
            u64::from_le_bytes(length_bytes.try_into().map_err(|_| VerificationError)?);
        let declared_len = usize::try_from(declared_u64).map_err(|_| VerificationError)?;
        verify!(declared_len == expected_len);
        let element_bytes = T::ZERO.compressed_size();
        verify!(element_bytes > 0);
        let encoded_len = expected_len
            .checked_mul(element_bytes)
            .ok_or(VerificationError)?;
        verify!(body.len() >= encoded_len);
        let (mut encoded, tail) = body.split_at(encoded_len);
        // Allocation only follows the exact external-length and byte-budget
        // checks above; a short malicious length prefix cannot request memory.
        let mut values = Vec::with_capacity(expected_len);
        for _ in 0..expected_len {
            values.push(T::deserialize_compressed(&mut encoded).map_err(|_| VerificationError)?);
        }
        verify!(encoded.is_empty());
        self.hints = tail;
        Ok(values)
    }

    #[cfg(debug_assertions)]
    #[track_caller]
    fn pop_pattern(&mut self, interaction: &Interaction) {
        let Some(pattern) = self.pattern else { return };
        assert!(!pattern.is_empty());
        let (expected, tail) = pattern.split_first().unwrap();
        assert_eq!(
            interaction, expected,
            "Transcript error: Expected interaction {expected:?} got {interaction:?}"
        );
        self.pattern = Some(tail);
    }
}

impl<'a> VerifierState<'a, StdHash> {
    /// Construct a new verifier state with the standard duplex hash function.
    pub fn new_std<'b, I>(ds: &DomainSeparator<'b, I>, proof: &'a Proof) -> Self
    where
        I: Encoding<[u8]>,
    {
        Self::new(ds, proof, StdHash::default())
    }
}

impl<H> VerifierMessage for VerifierState<'_, H>
where
    H: DuplexSpongeInterface,
{
    type U = H::U;

    #[cfg_attr(test, track_caller)]
    fn verifier_message<T>(&mut self) -> T
    where
        T: Decoding<[H::U]>,
    {
        #[cfg(debug_assertions)]
        self.pop_pattern(&Interaction::VerifierMessage(type_name::<T>().to_owned()));
        self.inner.verifier_message()
    }
}

#[cfg(all(test, not(feature = "verifier_panics")))]
mod mikros_security_tests {
    use ark_ff::{AdditiveGroup, Field};

    use super::*;
    use crate::algebra::fields::{Field64, Field64_2};
    use crate::transcript::codecs::{Empty, U64};

    fn transcript_domain(instance: &Empty) -> DomainSeparator<'_, Empty> {
        DomainSeparator::protocol(&"mikros-whir-canonical-wire-v1").instance(instance)
    }

    #[test]
    fn raw_transcript_round_trip_preserves_crypto_without_debug_pattern() {
        let instance = Empty;
        let ds = transcript_domain(&instance);
        let element = Field64_2::new(Field64::from(5_u64), Field64::from(9_u64));
        let mut prover = ProverState::new_std(&ds);
        prover.prover_message(&element);
        prover.prover_hint(&U64(19));
        let challenge: Field64_2 = prover.verifier_message();
        let generated = prover.proof();
        let serialized = serde_json::to_value(&generated).unwrap();
        assert!(serialized.get("pattern").is_none());
        let deserialized: Proof = serde_json::from_value(serialized).unwrap();
        let raw = Proof::from_raw_parts(deserialized.narg_string, deserialized.hints);
        let mut verifier = VerifierState::new_std(&ds, &raw);
        assert_eq!(verifier.prover_message::<Field64_2>().unwrap(), element);
        assert_eq!(verifier.prover_hint::<U64>().unwrap(), U64(19));
        assert_eq!(verifier.verifier_message::<Field64_2>(), challenge);
        verifier.check_eof().unwrap();
    }

    #[test]
    fn raw_transcript_rejects_noncanonical_extension_limb() {
        let instance = Empty;
        let ds = transcript_domain(&instance);
        let mut prover = ProverState::new_std(&ds);
        prover.prover_message(&Field64_2::ZERO);
        let generated = prover.proof();
        // The pinned Spongefish decoder reduces p to zero; the local wrapper
        // must reject the distinct wire representation before accepting it.
        for limb in 0..2 {
            let mut narg = generated.narg_string.clone();
            narg[limb * 8..limb * 8 + 8]
                .copy_from_slice(&18_446_744_069_414_584_321_u64.to_le_bytes());
            let raw = Proof::from_raw_parts(narg, generated.hints.clone());
            let mut verifier = VerifierState::new_std(&ds, &raw);
            assert!(verifier.prover_message::<Field64_2>().is_err());
        }
    }

    #[test]
    fn raw_transcript_rejects_truncation_and_both_trailing_channels() {
        let instance = Empty;
        let ds = transcript_domain(&instance);
        let mut prover = ProverState::new_std(&ds);
        prover.prover_message(&Field64_2::ONE);
        let generated = prover.proof();
        let mut truncated = generated.narg_string.clone();
        truncated.pop();
        let raw = Proof::from_raw_parts(truncated, Vec::new());
        assert!(VerifierState::new_std(&ds, &raw)
            .prover_message::<Field64_2>()
            .is_err());

        for trailing_hint in [false, true] {
            let mut raw = Proof::from_raw_parts(generated.narg_string.clone(), Vec::new());
            if trailing_hint {
                raw.hints.push(0);
            } else {
                raw.narg_string.push(0);
            }
            let mut verifier = VerifierState::new_std(&ds, &raw);
            assert_eq!(
                verifier.prover_message::<Field64_2>().unwrap(),
                Field64_2::ONE
            );
            assert!(verifier.check_eof().is_err());
        }
    }

    #[test]
    fn bounded_field_hint_rejects_lengths_before_allocation() {
        let instance = Empty;
        let ds = transcript_domain(&instance);
        let values = vec![Field64_2::ONE, Field64_2::ZERO];
        let mut prover = ProverState::new_std(&ds);
        prover.prover_hint_ark(&values);
        let generated = prover.proof();
        let raw = Proof::from_raw_parts(generated.narg_string.clone(), generated.hints.clone());
        let mut verifier = VerifierState::new_std(&ds, &raw);
        assert_eq!(
            verifier.prover_hint_ark_vec_exact::<Field64_2>(2).unwrap(),
            values
        );
        verifier.check_eof().unwrap();

        // Each case is rejected by the header comparison before allocating.
        for declared in [0_u64, 1, 3, 1_u64 << 40, u64::MAX] {
            let mut hints = generated.hints.clone();
            hints[..8].copy_from_slice(&declared.to_le_bytes());
            let raw = Proof::from_raw_parts(Vec::new(), hints);
            assert!(VerifierState::new_std(&ds, &raw)
                .prover_hint_ark_vec_exact::<Field64_2>(2)
                .is_err());
        }
        let mut truncated = generated.hints.clone();
        truncated.pop();
        let raw = Proof::from_raw_parts(Vec::new(), truncated);
        assert!(VerifierState::new_std(&ds, &raw)
            .prover_hint_ark_vec_exact::<Field64_2>(2)
            .is_err());

        // Even an invalid external length is rejected by checked multiplication
        // or the remaining-byte budget, never handed to Vec::with_capacity.
        let raw = Proof::from_raw_parts(Vec::new(), u64::MAX.to_le_bytes().to_vec());
        assert!(VerifierState::new_std(&ds, &raw)
            .prover_hint_ark_vec_exact::<Field64_2>(usize::MAX)
            .is_err());
    }
}
