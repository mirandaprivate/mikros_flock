//! Binary RAA encoding and authenticated SwitchFold generator reductions.
//!
//! The code is Repeat4 -> P1 -> Accumulate -> P2 -> Accumulate, matching
//! the two-accumulator Blaze code. Every recursive level has rate 1/4.
//! The distance profile delta=0.19 is explicitly conditional: it is not a
//! certificate for the sampled finite-length permutation instance.
//! Both permutation leaves are authenticated by a batched grand product;
//! fixed H and dynamic S claims must be discharged by the enclosing PCS.
//! The production backend is adapted from BrakeFold; see PROVENANCE.md.

use core::fmt::Debug;
use std::{sync::Arc, time::Instant};

use crate::Error;
use ark_std::{rand::RngCore, vec, vec::Vec};
use binius_field::{Field, field::FieldOps};
use binius_iop::channel::{IOPVerifierChannel, OracleLinearRelation, OracleSpec};
use binius_iop_prover::channel::IOPProverChannel;
use binius_ip::channel::IPVerifierChannel;
use binius_ip_prover::channel::IPProverChannel;
use binius_math::FieldBuffer;
use binius_math::multilinear::eq::{eq_ind, eq_ind_partial_eval_scalars};
use blake2::Blake2s256;
use digest::Digest;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::queries_for_relative_distance;
use crate::raafold_field::RustCryptoPolyvalField;

/// The challenge field used by the RAAFold reduction.
pub type BinaryExtensionField = RustCryptoPolyvalField;

/// Packed Binius representation used by the native permutation proof.
pub type PackedBinaryExtensionField = RustCryptoPolyvalField;

#[cfg(feature = "parallel")]
const PARALLEL_FIELD_THRESHOLD: usize = 1 << 12;

/// Domain separation for the two-accumulator generator.
pub const RAA_DOMAIN_LABEL: &[u8] = b"brakefold/raa-two-accumulator/v2";

#[cfg(feature = "parallel")]
const PARALLEL_SCAN_MIN_LEN: usize = 1 << 15;
/// Number of accumulator/permutation layers in every RAA code.
pub const RAA_ACCUMULATION_LAYERS: usize = 2;

const RAA_PERMUTATION_COUNT: usize = RAA_ACCUMULATION_LAYERS;
const RAA_FACTOR_COUNT: usize = 5;
const RAA_PADDED_PERMUTATION_COUNT: usize = 4;
const RAA_PRODUCT_SIDE_COUNT: usize = 2;
const RAA_SHIFTED_VIEW_COUNT: usize = 2;
const RAA_PRODUCT_CHAIN_COUNT: usize = RAA_PERMUTATION_COUNT * RAA_PRODUCT_SIDE_COUNT;
const RAA_PERMUTATION_ORACLE_COUNT: usize = 2;
const RAA_DIRECT_INTERLEAVED_DOMAIN_LABEL: &[u8] = b"brakefold/raa-direct-interleaved-sequence/v3";
#[derive(Clone, Debug, Eq, PartialEq)]
struct RaaHadamardReduction<E = BinaryExtensionField> {
    left: RaaMatrixClaim<E>,
    right: RaaMatrixClaim<E>,
}

/// Repetition factor at the top level (rate `1/4`).
pub const RAAFOLD_TOP_RATE_INV: usize = 4;

/// Repetition factor at every recursive level (rate `1/4`).
pub const RAAFOLD_RECURSIVE_RATE_INV: usize = 4;

/// Number of extension-field columns in the top witness commitment.
pub const RAAFOLD_TOP_INTERLEAVING: usize = 128;

/// Message-length reduction used by every recursive RAAFold code switch.
pub const RAAFOLD_RECURSIVE_SWITCH_RATIO: usize = 8;

pub(crate) const fn raafold_rate_inv_for_level(_level: usize, _levels: usize) -> usize {
    RAAFOLD_RECURSIVE_RATE_INV
}

/// Assumed relative distance of the rate-1/4 two-accumulator Blaze code.
/// This is conditional on the sampled code meeting that distance.
pub const BLAZE_ASSUMED_RELATIVE_DISTANCE: f64 = 0.19;

/// Additive operations required by RAA encoding.
///
/// This deliberately does not require multiplication: packed short-bitwidth
/// lanes are characteristic-two vector-space elements, not scalar fields.
pub trait RaaAdditiveSymbol: Copy + Default + Eq + Send + Sync + Debug + 'static {
    /// Additive identity.
    fn zero() -> Self;

    /// Characteristic-two addition.
    fn add(self, rhs: Self) -> Self;
}

impl RaaAdditiveSymbol for BinaryExtensionField {
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    #[inline]
    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}

/// Provenance attached to an RAA distance parameter.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RaaDistanceProvenance {
    /// An experimental or otherwise unproved setup assumption.
    Assumed,
    /// A setup-specific certificate with a failure-probability bound.
    Certified {
        /// Base-two logarithm of the setup failure probability.
        log2_failure_probability: f64,
    },
}

/// Code distance and the effective distance used for proximity queries.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RaaDistanceProfile {
    /// Relative distance asserted for the sampled RAA generator.
    code_relative_distance: f64,
    /// Relative distance after protocol-level degradation.
    query_relative_distance: f64,
    /// Source of the distance assertion.
    provenance: RaaDistanceProvenance,
}

impl RaaDistanceProfile {
    /// Returns Blaze's explicitly assumed rate-1/4 distance profile.
    /// This does not certify the finite-length sampled code.
    pub fn blaze_assumed(rate_inv: usize) -> Result<Self, Error> {
        if rate_inv != 4 {
            return Err(Error::EncodingError);
        }
        Self::assumed(
            BLAZE_ASSUMED_RELATIVE_DISTANCE,
            BLAZE_ASSUMED_RELATIVE_DISTANCE,
        )
    }

    /// Builds an explicitly assumed distance profile.
    pub fn assumed(
        code_relative_distance: f64,
        query_relative_distance: f64,
    ) -> Result<Self, Error> {
        Self::new(
            code_relative_distance,
            query_relative_distance,
            RaaDistanceProvenance::Assumed,
        )
    }

    /// Builds a certified distance profile.
    pub fn certified(
        code_relative_distance: f64,
        query_relative_distance: f64,
        log2_failure_probability: f64,
    ) -> Result<Self, Error> {
        if !log2_failure_probability.is_finite() || log2_failure_probability >= 0.0 {
            return Err(Error::EncodingError);
        }
        Self::new(
            code_relative_distance,
            query_relative_distance,
            RaaDistanceProvenance::Certified {
                log2_failure_probability,
            },
        )
    }

    fn new(
        code_relative_distance: f64,
        query_relative_distance: f64,
        provenance: RaaDistanceProvenance,
    ) -> Result<Self, Error> {
        if !valid_distance(code_relative_distance)
            || !valid_distance(query_relative_distance)
            || query_relative_distance > code_relative_distance
        {
            return Err(Error::EncodingError);
        }
        Ok(Self {
            code_relative_distance,
            query_relative_distance,
            provenance,
        })
    }

    /// Returns the relative distance asserted for the sampled RAA generator.
    pub fn code_relative_distance(self) -> f64 {
        self.code_relative_distance
    }

    /// Returns the effective relative distance used for proximity queries.
    pub fn query_relative_distance(self) -> f64 {
        self.query_relative_distance
    }

    /// Returns the provenance of the distance assertion.
    pub fn provenance(self) -> RaaDistanceProvenance {
        self.provenance
    }
}

/// A permutation with the convention `output[i] = input[sources[i]]`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaaPermutation {
    sources: Vec<u32>,
    inverse_sources: Vec<u32>,
}

impl RaaPermutation {
    /// Builds and validates a permutation.
    pub fn from_sources(sources: Vec<u32>) -> Result<Self, Error> {
        if sources.is_empty() {
            return Err(Error::EncodingError);
        }
        let mut seen = vec![false; sources.len()];
        for source in &sources {
            let source = *source as usize;
            if source >= sources.len() || seen[source] {
                return Err(Error::EncodingError);
            }
            seen[source] = true;
        }
        let mut inverse_sources = vec![0u32; sources.len()];
        for (row, &source) in sources.iter().enumerate() {
            inverse_sources[source as usize] = row as u32;
        }
        Ok(Self {
            sources,
            inverse_sources,
        })
    }

    /// Returns the identity permutation of the requested length.
    pub fn identity(length: usize) -> Result<Self, Error> {
        validate_permutation_length(length)?;
        Self::from_sources((0..length).map(|index| index as u32).collect())
    }

    /// Samples a uniformly shuffled permutation with Fisher--Yates.
    pub fn sample<R: RngCore>(length: usize, rng: &mut R) -> Result<Self, Error> {
        validate_permutation_length(length)?;
        let mut sources = (0..length).map(|index| index as u32).collect::<Vec<_>>();
        for upper in (1..length).rev() {
            sources.swap(upper, sample_below(rng, upper + 1)?);
        }
        Self::from_sources(sources)
    }

    /// Returns the permutation length.
    pub fn len(&self) -> usize {
        self.sources.len()
    }

    /// Returns whether this permutation is empty.
    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }

    /// Returns the source-index table.
    pub fn sources(&self) -> &[u32] {
        &self.sources
    }

    fn inverse_sources(&self) -> &[u32] {
        &self.inverse_sources
    }
}

/// Shared setup for extension-field and packed-lane RAA encoders.
#[derive(Clone, Debug)]
pub struct RaaLayout {
    message_len: usize,
    rate_inv: usize,
    permutations: [RaaPermutation; RAA_PERMUTATION_COUNT],
    distance: RaaDistanceProfile,
    generator_digest: [u8; 32],
}

impl RaaLayout {
    /// Samples both setup permutations.
    pub fn sample<R: RngCore>(
        message_len: usize,
        rate_inv: usize,
        distance: RaaDistanceProfile,
        rng: &mut R,
    ) -> Result<Self, Error> {
        let codeword_len = validate_layout_shape(message_len, rate_inv)?;
        let permutations = [
            RaaPermutation::sample(codeword_len, rng)?,
            RaaPermutation::sample(codeword_len, rng)?,
        ];
        Self::from_permutations(message_len, rate_inv, distance, permutations)
    }

    /// Builds a setup from two explicit permutations.
    pub fn from_permutations(
        message_len: usize,
        rate_inv: usize,
        distance: RaaDistanceProfile,
        permutations: [RaaPermutation; RAA_PERMUTATION_COUNT],
    ) -> Result<Self, Error> {
        let codeword_len = validate_layout_shape(message_len, rate_inv)?;
        if permutations
            .iter()
            .any(|permutation| permutation.len() != codeword_len)
        {
            return Err(Error::EncodingError);
        }
        let generator_digest = generator_digest(message_len, rate_inv, &permutations)?;
        Ok(Self {
            message_len,
            rate_inv,
            permutations,
            distance,
            generator_digest,
        })
    }

    /// Returns the message length.
    pub fn message_len(&self) -> usize {
        self.message_len
    }

    /// Returns the inverse rate.
    pub fn rate_inv(&self) -> usize {
        self.rate_inv
    }

    /// Returns the codeword length.
    pub fn codeword_len(&self) -> usize {
        self.message_len * self.rate_inv
    }

    /// Returns the setup distance profile.
    pub fn distance_profile(&self) -> RaaDistanceProfile {
        self.distance
    }

    /// Returns a digest binding the RAA domain, shape, and permutations.
    pub fn generator_digest(&self) -> [u8; 32] {
        self.generator_digest
    }

    /// Returns one of the two setup permutations.
    pub fn permutation(&self, round: usize) -> Result<&RaaPermutation, Error> {
        self.permutations
            .get(round.checked_sub(1).ok_or(Error::EncodingError)?)
            .ok_or(Error::EncodingError)
    }

    pub(crate) fn encode_additive<S: RaaAdditiveSymbol>(
        &self,
        message: &[S],
    ) -> Result<Vec<S>, Error> {
        self.encode_additive_with_mode(message, true)
    }

    pub(crate) fn encode_additive_serial<S: RaaAdditiveSymbol>(
        &self,
        message: &[S],
    ) -> Result<Vec<S>, Error> {
        self.encode_additive_with_mode(message, false)
    }

    /// Encodes a column-major interleaved message into row-major codeword rows.
    ///
    /// Permutations move complete interleaved rows. Accumulation uses a
    /// parallel scan over 64 row groups and applies every XOR offset to a
    /// complete row. Matrices with fewer than 64 rows use the direct serial
    /// scan.
    pub(crate) fn encode_interleaved_column_major_into_row_major<S: RaaAdditiveSymbol>(
        &self,
        messages: &[S],
        columns: usize,
        output: &mut [S],
    ) -> Result<(), Error> {
        let message_entries = self
            .message_len
            .checked_mul(columns)
            .ok_or(Error::EncodingError)?;
        let encoded_entries = self
            .codeword_len()
            .checked_mul(columns)
            .ok_or(Error::EncodingError)?;
        if columns == 0 || messages.len() != message_entries || output.len() != encoded_entries {
            return Err(Error::EncodingError);
        }

        let mut scratch = vec![S::zero(); encoded_entries];
        if columns >= 32 && message_entries >= 32 * 1024 {
            // Output is not live until P2. Borrow its first message-sized
            // prefix for a blocked transpose, then read each P1 source as a
            // contiguous row instead of gathering one item from every column.
            // This needs no additional matrix allocation. P2 later overwrites
            // the whole output, after all staging reads have finished.
            let staged = &mut output[..message_entries];
            transpose_interleaved_message_into_row_major(
                messages,
                self.message_len,
                columns,
                staged,
            );
            finish_fused_staged_interleaved_encoding(
                message_entries,
                columns,
                self.rate_inv,
                self.permutations[0].sources(),
                self.permutations[1].sources(),
                output,
                &mut scratch,
            );
        } else {
            fill_interleaved_repeated_permutation(
                messages,
                self.message_len,
                columns,
                self.rate_inv,
                self.permutations[0].sources(),
                &mut scratch,
            );
            self.finish_interleaved_row_major_encoding(columns, output, &mut scratch);
        }
        Ok(())
    }

    /// Encodes generated message rows directly into row-major codeword rows.
    pub(crate) fn encode_interleaved_generated_into_row_major<S, F>(
        &self,
        columns: usize,
        output: &mut [S],
        value_at: F,
    ) -> Result<(), Error>
    where
        S: RaaAdditiveSymbol,
        F: Fn(usize, usize) -> S + Sync,
    {
        let encoded_entries = self
            .codeword_len()
            .checked_mul(columns)
            .ok_or(Error::EncodingError)?;
        if columns == 0 || output.len() != encoded_entries {
            return Err(Error::EncodingError);
        }

        let mut scratch = vec![S::zero(); encoded_entries];
        fill_generated_interleaved_repeated_permutation(
            self.message_len,
            columns,
            self.rate_inv,
            self.permutations[0].sources(),
            &mut scratch,
            value_at,
        );
        self.finish_interleaved_row_major_encoding(columns, output, &mut scratch);
        Ok(())
    }

    fn finish_interleaved_row_major_encoding<S: RaaAdditiveSymbol>(
        &self,
        columns: usize,
        output: &mut [S],
        scratch: &mut [S],
    ) {
        // P1(Repeat(message)) is already in scratch. End the second scan in the
        // caller's output buffer, avoiding a full codeword copy after encoding.
        prefix_accumulate_interleaved_rows(scratch, columns);
        apply_interleaved_permutation(scratch, columns, self.permutations[1].sources(), output);
        prefix_accumulate_interleaved_rows(output, columns);
    }

    fn encode_additive_with_mode<S: RaaAdditiveSymbol>(
        &self,
        message: &[S],
        allow_parallel_scan: bool,
    ) -> Result<Vec<S>, Error> {
        let mut output = vec![S::zero(); self.codeword_len()];
        let mut scratch = vec![S::zero(); self.codeword_len()];
        self.encode_additive_into_with_scratch(
            message,
            &mut output,
            &mut scratch,
            allow_parallel_scan,
        )?;
        Ok(output)
    }

    pub(crate) fn encode_additive_into_with_scratch<S: RaaAdditiveSymbol>(
        &self,
        message: &[S],
        output: &mut [S],
        scratch: &mut [S],
        allow_parallel_scan: bool,
    ) -> Result<(), Error> {
        if message.len() != self.message_len
            || output.len() != self.codeword_len()
            || scratch.len() != self.codeword_len()
        {
            return Err(Error::EncodingError);
        }
        fill_repeated_permutation(
            message,
            self.rate_inv,
            self.permutations[0].sources(),
            output,
            allow_parallel_scan,
        );
        prefix_accumulate(output, allow_parallel_scan);
        apply_permutation(
            output,
            self.permutations[1].sources(),
            scratch,
            allow_parallel_scan,
        );
        prefix_accumulate(scratch, allow_parallel_scan);
        output.copy_from_slice(scratch);
        Ok(())
    }
}

/// Two-accumulator RAA over `GF(2^128)`.
#[derive(Clone, Debug)]
pub struct RaaCode {
    layout: Arc<RaaLayout>,
}

impl RaaCode {
    /// Samples a two-accumulator RAA setup.
    pub fn sample<R: RngCore>(
        message_len: usize,
        rate_inv: usize,
        distance: RaaDistanceProfile,
        rng: &mut R,
    ) -> Result<Self, Error> {
        Ok(Self {
            layout: Arc::new(RaaLayout::sample(message_len, rate_inv, distance, rng)?),
        })
    }

    /// Builds a code from explicit setup permutations.
    pub fn from_permutations(
        message_len: usize,
        rate_inv: usize,
        distance: RaaDistanceProfile,
        permutations: [RaaPermutation; RAA_PERMUTATION_COUNT],
    ) -> Result<Self, Error> {
        Ok(Self {
            layout: Arc::new(RaaLayout::from_permutations(
                message_len,
                rate_inv,
                distance,
                permutations,
            )?),
        })
    }

    /// Returns the shared generator layout.
    pub fn layout(&self) -> &RaaLayout {
        &self.layout
    }

    /// Returns the message length.
    pub fn message_len(&self) -> usize {
        self.layout.message_len()
    }

    /// Returns the inverse rate.
    pub fn rate_inv(&self) -> usize {
        self.layout.rate_inv()
    }

    /// Returns the codeword length.
    pub fn codeword_len(&self) -> usize {
        self.layout.codeword_len()
    }

    /// Returns the setup distance profile.
    pub fn distance_profile(&self) -> RaaDistanceProfile {
        self.layout.distance_profile()
    }

    /// Returns the setup generator digest.
    pub fn generator_digest(&self) -> [u8; 32] {
        self.layout.generator_digest()
    }

    /// Returns the public setup elements absorbed before generator reduction.
    ///
    /// The digest binds the protocol version, dimensions, and both
    /// permutations. The dimensions are repeated explicitly to keep transcript
    /// inspection straightforward.
    pub fn transcript_setup_elements(&self) -> [BinaryExtensionField; 4] {
        let digest = self.generator_digest();
        let mut low = [0u8; 16];
        let mut high = [0u8; 16];
        low.copy_from_slice(&digest[..16]);
        high.copy_from_slice(&digest[16..]);
        [
            BinaryExtensionField::from(self.message_len() as u128),
            BinaryExtensionField::from(self.rate_inv() as u128),
            BinaryExtensionField::from(u128::from_le_bytes(low)),
            BinaryExtensionField::from(u128::from_le_bytes(high)),
        ]
    }

    /// Encodes one extension-field message with two accumulations.
    pub fn encode(
        &self,
        message: &[BinaryExtensionField],
    ) -> Result<Vec<BinaryExtensionField>, Error> {
        self.layout.encode_additive(message)
    }

    /// Encodes one extension-field message into a caller-owned codeword.
    ///
    /// The implementation allocates only the permutation scratch vector. This
    /// is the preferred path when many independently encoded columns share one
    /// preallocated commitment matrix.
    pub fn encode_into(
        &self,
        message: &[BinaryExtensionField],
        output: &mut [BinaryExtensionField],
    ) -> Result<(), Error> {
        let mut scratch = vec![BinaryExtensionField::ZERO; self.codeword_len()];
        self.layout
            .encode_additive_into_with_scratch(message, output, &mut scratch, true)
    }

    /// Encodes independent columns, parallelizing across columns.
    pub fn encode_columns(
        &self,
        messages: &[Vec<BinaryExtensionField>],
    ) -> Result<Vec<Vec<BinaryExtensionField>>, Error> {
        if messages
            .iter()
            .any(|message| message.len() != self.message_len())
        {
            return Err(Error::EncodingError);
        }
        if messages.len() == 1 {
            return Ok(vec![self.encode(&messages[0])?]);
        }
        #[cfg(feature = "parallel")]
        let encoded = messages
            .par_iter()
            .map(|message| self.layout.encode_additive_serial(message))
            .collect::<Result<Vec<_>, _>>()?;
        #[cfg(not(feature = "parallel"))]
        let encoded = messages
            .iter()
            .map(|message| self.layout.encode_additive_serial(message))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(encoded)
    }

    /// Computes `G r` without materializing `G`.
    pub fn generator_right_projection(
        &self,
        right: &[BinaryExtensionField],
    ) -> Result<Vec<BinaryExtensionField>, Error> {
        self.encode(right)
    }

    /// Computes `l^T G` by transposing the five factors.
    pub fn generator_left_projection(
        &self,
        left: &[BinaryExtensionField],
    ) -> Result<Vec<BinaryExtensionField>, Error> {
        if left.len() != self.codeword_len() {
            return Err(Error::EncodingError);
        }
        apply_subchain_left(self.layout(), 0, RAA_FACTOR_COUNT - 1, left.to_vec())
    }

    /// Evaluates the generator MLE in Binius' equality-indicator basis.
    pub fn evaluate_generator_mle(
        &self,
        row_point: &[BinaryExtensionField],
        column_point: &[BinaryExtensionField],
    ) -> Result<BinaryExtensionField, Error> {
        validate_generator_claim_shape(self.layout(), row_point, column_point)?;
        let row_weights = eq_ind_partial_eval_scalars(row_point);
        let column_weights = eq_ind_partial_eval_scalars(column_point);
        let encoded = self.encode(&column_weights)?;
        Ok(inner_product(&row_weights, &encoded))
    }

    /// Returns the proximity-query count from the explicit query distance.
    pub fn num_queries(&self, security_bits: usize, available_rows: usize) -> Result<usize, Error> {
        if available_rows == 0 {
            return Err(Error::EncodingError);
        }
        Ok(queries_for_relative_distance(
            security_bits,
            self.distance_profile().query_relative_distance(),
        )?
        .min(available_rows))
    }

    /// Returns the pushforward and prefix-product oracle specifications.
    ///
    /// The second oracle is precisely the dynamic `S_j`: two direct prefix-
    /// product records, one per permutation. Every record contains the two
    /// sides of the permutation product identity and the two shifted views
    /// `(1 || prefixvec)` and `(prefixvec || pi)` of each side. The two real
    /// permutation records are padded to four.
    /// The pushforward oracle is a dynamic auxiliary block that binds the
    /// permutation leaf claim. SwitchFold commits it before sampling the
    /// grand-product challenges, then commits the prefix states afterward.
    /// Both blocks are opened recursively with the next RAA code.
    pub fn permutation_oracle_specs(&self) -> [OracleSpec; RAA_PERMUTATION_ORACLE_COUNT] {
        let log_padded_len = self.codeword_len().ilog2() as usize + 2;
        [
            OracleSpec::new(log_padded_len),
            OracleSpec::new(log_padded_len + 2),
        ]
    }

    /// Returns one setup column containing `omega^{P_i(j)}` in `GF(2^128)`.
    ///
    /// The multiplicative generator `omega` is fixed by the field.  Its first
    /// `codeword_len` powers are distinct, so the grand product binds both the
    /// value and its exact permutation position.
    pub fn permutation_index_column(
        &self,
        round: usize,
    ) -> Result<Vec<BinaryExtensionField>, Error> {
        let powers = permutation_index_powers(self.codeword_len())?;
        Ok(self
            .layout
            .permutation(round)?
            .sources()
            .iter()
            .map(|source| powers[*source as usize])
            .collect())
    }

    /// Returns the logical fixed-matrix shape used by recursive Decide.
    ///
    /// Every permutation-index vector has `rate_inv * message_len` entries.
    /// Reshaping it column-major yields `rate_inv` columns of `message_len`
    /// rows. Two real rounds and two implicit zero rounds retain the source
    /// grand-product protocol's four-slot layout.
    pub fn fixed_h_layout(&self) -> RaaFixedHLayout {
        RaaFixedHLayout {
            rows: self.message_len(),
            columns: RAA_PERMUTATION_COUNT * self.rate_inv(),
            padded_columns: RAA_PADDED_PERMUTATION_COUNT * self.rate_inv(),
            columns_per_permutation: self.rate_inv(),
        }
    }

    /// Materializes one logical column of the fixed `H_j` matrix.
    ///
    /// Columns are ordered by permutation round and then by the contiguous
    /// `message_len`-entry block within that permutation-index vector.
    pub fn fixed_h_column(&self, column: usize) -> Result<Vec<BinaryExtensionField>, Error> {
        let layout = self.fixed_h_layout();
        let powers = permutation_index_powers(self.codeword_len())?;
        let mut output = vec![BinaryExtensionField::ZERO; layout.rows];
        self.fixed_h_block_into_with_powers(column, 0, &mut output, &powers)?;
        Ok(output)
    }

    /// Materializes the real, unpadded columns of the fixed `H_j` matrix.
    pub fn fixed_h_columns(&self) -> Result<Vec<Vec<BinaryExtensionField>>, Error> {
        let layout = self.fixed_h_layout();
        let powers = permutation_index_powers(self.codeword_len())?;
        #[cfg(feature = "parallel")]
        let columns = (0..layout.columns)
            .into_par_iter()
            .map(|column| {
                let mut output = vec![BinaryExtensionField::ZERO; layout.rows];
                self.fixed_h_block_into_with_powers(column, 0, &mut output, &powers)?;
                Ok(output)
            })
            .collect::<Result<Vec<_>, _>>()?;
        #[cfg(not(feature = "parallel"))]
        let columns = (0..layout.columns)
            .map(|column| {
                let mut output = vec![BinaryExtensionField::ZERO; layout.rows];
                self.fixed_h_block_into_with_powers(column, 0, &mut output, &powers)?;
                Ok(output)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(columns)
    }

    /// Writes the real fixed `H_j` columns directly into a row-major matrix.
    pub(crate) fn write_fixed_h_row_major(
        &self,
        output: &mut [BinaryExtensionField],
        row_stride: usize,
    ) -> Result<(), Error> {
        let layout = self.fixed_h_layout();
        let expected_len = layout
            .rows
            .checked_mul(row_stride)
            .ok_or(Error::EncodingError)?;
        if row_stride < layout.columns || output.len() != expected_len {
            return Err(Error::EncodingError);
        }
        let powers = permutation_index_powers(self.codeword_len())?;
        let permutation_sources = [
            self.layout.permutation(1)?.sources(),
            self.layout.permutation(2)?.sources(),
        ];
        let fill_row = |row: usize, target: &mut [BinaryExtensionField]| {
            for (column, value) in target[..layout.columns].iter_mut().enumerate() {
                let round = column / layout.columns_per_permutation;
                let rate_column = column % layout.columns_per_permutation;
                let source_offset = rate_column * layout.rows + row;
                *value = powers[permutation_sources[round][source_offset] as usize];
            }
        };
        #[cfg(feature = "parallel")]
        output
            .par_chunks_mut(row_stride)
            .enumerate()
            .for_each(|(row, target)| fill_row(row, target));
        #[cfg(not(feature = "parallel"))]
        output
            .chunks_mut(row_stride)
            .enumerate()
            .for_each(|(row, target)| fill_row(row, target));
        Ok(())
    }

    fn fixed_h_block_into_with_powers(
        &self,
        column: usize,
        row_start: usize,
        output: &mut [BinaryExtensionField],
        powers: &[BinaryExtensionField],
    ) -> Result<(), Error> {
        let layout = self.fixed_h_layout();
        if column >= layout.columns
            || powers.len() != self.codeword_len()
            || output.is_empty()
            || row_start
                .checked_add(output.len())
                .filter(|end| *end <= layout.rows)
                .is_none()
        {
            return Err(Error::EncodingError);
        }
        let round = column / layout.columns_per_permutation + 1;
        let rate_column = column % layout.columns_per_permutation;
        let start = rate_column
            .checked_mul(layout.rows)
            .and_then(|offset| offset.checked_add(row_start))
            .ok_or(Error::EncodingError)?;
        let end = start
            .checked_add(output.len())
            .ok_or(Error::EncodingError)?;
        let sources = self
            .layout
            .permutation(round)?
            .sources()
            .get(start..end)
            .ok_or(Error::EncodingError)?;
        output
            .iter_mut()
            .zip(sources)
            .for_each(|(target, source)| *target = powers[*source as usize]);
        Ok(())
    }

    /// Proves the generator reduction and one batched permutation grand product.
    ///
    /// The next two channel oracle slots must match
    /// [`permutation_oracle_specs`](Self::permutation_oracle_specs). They bind
    /// the two pushforward vectors and the padded shifted-prefix state oracle,
    /// respectively. The returned fixed-index claim must be opened
    /// against the setup commitment to `H_j`.
    pub fn reduce_prover<Channel>(
        &self,
        claim: &RaaMatrixClaim<BinaryExtensionField>,
        channel: &mut Channel,
    ) -> Result<RaaGeneratorReduction<BinaryExtensionField>, Error>
    where
        Channel: IOPProverChannel<PackedBinaryExtensionField>,
    {
        // [PROFILE] These boundaries are enabled only for explicit benchmark runs.
        let profile_stages = std::env::var_os("RAAFOLD_PROFILE_STAGES").is_some();
        let total_start = Instant::now();
        validate_generator_claim_dimensions(self.layout(), claim)?;
        validate_permutation_oracle_specs(channel.remaining_oracle_specs(), self.codeword_len())?;
        channel.observe_many(&self.transcript_setup_elements());
        let mut leaf_claims = Vec::with_capacity(RAA_FACTOR_COUNT);
        let factor_chain_start = Instant::now();
        prove_factor_chain(
            self.layout(),
            claim,
            0,
            RAA_FACTOR_COUNT - 1,
            channel,
            &mut leaf_claims,
        )?;
        let factor_chain_time = factor_chain_start.elapsed();
        let leaf_start = Instant::now();
        let leaves = assemble_leaves(leaf_claims)?;
        validate_transparent_leaves(self.layout(), &leaves)?;
        let leaf_time = leaf_start.elapsed();

        let grand_product_start = Instant::now();
        let permutation_grand_product =
            prove_permutation_grand_product(self.layout(), &leaves.permutations, channel)?;
        let grand_product_time = grand_product_start.elapsed();
        if profile_stages {
            eprintln!(
                "[RAAFOLD_PROFILE] generator_reduce_n={} factor_chain_s={:.9} leaf_checks_s={:.9} grand_product_s={:.9} reduce_total_s={:.9}",
                self.codeword_len(),
                factor_chain_time.as_secs_f64(),
                leaf_time.as_secs_f64(),
                grand_product_time.as_secs_f64(),
                total_start.elapsed().as_secs_f64(),
            );
        }
        Ok(RaaGeneratorReduction {
            ok: true,
            leaves,
            permutation_grand_product,
        })
    }

    /// Verifies the generator reduction with logarithmic transparent checks.
    ///
    /// The caller must already have absorbed the root claim and fixed-index
    /// commitments. The returned `H_j` claim is checked against the direct
    /// `C_{j-1}` setup commitment by the enclosing Decide protocol.
    pub fn verify_as_subprotocol<Channel>(
        &self,
        claim: &RaaMatrixClaim<BinaryExtensionField>,
        channel: &mut Channel,
    ) -> Result<RaaGeneratorReduction<BinaryExtensionField>, Error>
    where
        Channel: IOPVerifierChannel<BinaryExtensionField, Elem = BinaryExtensionField>,
    {
        validate_generator_claim_dimensions(self.layout(), claim)?;
        validate_permutation_oracle_specs(channel.remaining_oracle_specs(), self.codeword_len())?;
        let _ = channel.observe_many(&self.transcript_setup_elements());
        let mut leaf_claims = Vec::with_capacity(RAA_FACTOR_COUNT);
        verify_factor_chain(
            self.layout(),
            claim,
            0,
            RAA_FACTOR_COUNT - 1,
            channel,
            &mut leaf_claims,
        )?;
        let leaves = assemble_leaves(leaf_claims)?;
        verify_transparent_leaves(self.layout(), &leaves, channel)?;

        let permutation_grand_product =
            verify_permutation_grand_product(self.layout(), &leaves.permutations, channel)?;
        Ok(RaaGeneratorReduction {
            ok: true,
            leaves,
            permutation_grand_product,
        })
    }
}

/// An MLE claim on a matrix in the Binius equality-indicator basis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaaMatrixClaim<E = BinaryExtensionField> {
    /// Claimed matrix MLE value.
    pub value: E,
    /// Row and column evaluation points.
    pub point: (Vec<E>, Vec<E>),
}

/// A matrix MLE claim whose value includes a public multiplicative scale.
///
/// This projective form avoids dividing by a random branch weight when a
/// submatrix is extracted from a larger reduction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaaScaledMatrixClaim<E = BinaryExtensionField> {
    /// Claimed value, equal to `scale * matrix_mle(point)`.
    pub value: E,
    /// Public scale multiplying the matrix evaluation.
    pub scale: E,
    /// Row and column evaluation points.
    pub point: (Vec<E>, Vec<E>),
}

/// Logical fixed-matrix shape for the two setup permutation columns.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RaaFixedHLayout {
    /// Number of rows, equal to the RAA message length `k_j`.
    pub rows: usize,
    /// Number of real columns, equal to `2 * rate_inv`.
    pub columns: usize,
    /// Power-of-two folding width, equal to `4 * rate_inv`.
    pub padded_columns: usize,
    /// Columns contributed by each permutation-index vector.
    pub columns_per_permutation: usize,
}

/// Five leaf claims obtained from `A P2 A P1 R`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaaGeneratorLeaves<E = BinaryExtensionField> {
    /// Accumulator claims ordered by encoding round `A1, A2`.
    pub accumulators: [RaaMatrixClaim<E>; RAA_PERMUTATION_COUNT],
    /// Permutation claims ordered by encoding round `P1, P2`.
    pub permutations: [RaaMatrixClaim<E>; RAA_PERMUTATION_COUNT],
    /// Claim on the adjacent repetition matrix.
    pub repeat: RaaMatrixClaim<E>,
}

/// Reduced output of the batched permutation grand product.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaaPermutationGrandProductReduction<E = BinaryExtensionField> {
    /// The single batched MLE claim on the padded `H_j` index matrix.
    pub fixed_h_claim: RaaScaledMatrixClaim<E>,
}

/// Complete RAA generator reduction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaaGeneratorReduction<E = BinaryExtensionField> {
    /// Whether all prover-side local checks passed.
    pub ok: bool,
    /// Five factor claims produced by projected LiteBullet reductions.
    pub leaves: RaaGeneratorLeaves<E>,
    /// One grand-product reduction batching both permutations.
    pub permutation_grand_product: RaaPermutationGrandProductReduction<E>,
}

impl<E> RaaGeneratorReduction<E> {
    /// Returns the number of batched permutation grand products.
    pub const fn grand_product_count(&self) -> usize {
        1
    }

    /// Returns the single batched claim on the padded fixed `H_j` matrix.
    pub fn fixed_h_claims(&self, code: &RaaCode) -> Result<[RaaScaledMatrixClaim<E>; 1], Error>
    where
        E: Clone,
    {
        let claim = &self.permutation_grand_product.fixed_h_claim;
        let layout = code.fixed_h_layout();
        if claim.point.0.len() != layout.rows.ilog2() as usize
            || claim.point.1.len() != layout.padded_columns.ilog2() as usize
        {
            return Err(Error::InvalidNumberOfVariables);
        }
        Ok([claim.clone()])
    }
}

/// One direct code switch from level `j` to the preceding code `C_{j-1}`.
///
/// A logical `k_j`-entry column is split into `switch_ratio` contiguous
/// `k_{j-1}`-entry blocks. Each block is encoded independently by `C_{j-1}`.
/// There are no random-foldable diagonals and no `T` columns.
#[derive(Clone, Debug)]
pub struct RaaDirectInterleavedLevel {
    source_level: usize,
    target_level: usize,
    row_factor: usize,
    target: RaaCode,
}

impl RaaDirectInterleavedLevel {
    /// Returns the level whose logical columns are being switched.
    pub fn source_level(&self) -> usize {
        self.source_level
    }

    /// Returns the preceding level used to encode every physical column.
    pub fn target_level(&self) -> usize {
        self.target_level
    }

    /// Returns the number of contiguous blocks per logical column.
    pub fn row_factor(&self) -> usize {
        self.row_factor
    }

    /// Returns the number of rows in one logical source column.
    pub fn logical_rows(&self) -> Result<usize, Error> {
        self.target
            .message_len()
            .checked_mul(self.row_factor)
            .ok_or(Error::EncodingError)
    }

    /// Returns the message rows consumed by each lower-code encoding.
    pub fn encoding_message_rows(&self) -> usize {
        self.target.message_len()
    }

    /// Returns the encoded rows in each physical commitment column.
    pub fn encoded_rows(&self) -> usize {
        self.target.codeword_len()
    }

    /// Returns the lower RAA code used by this direct switch.
    pub fn target_code(&self) -> &RaaCode {
        &self.target
    }

    /// Returns the number of physical columns produced from logical columns.
    pub fn physical_columns(&self, logical_columns: usize) -> Result<usize, Error> {
        if logical_columns == 0 {
            return Err(Error::EncodingError);
        }
        logical_columns
            .checked_mul(self.row_factor)
            .ok_or(Error::EncodingError)
    }

    /// Encodes one logical source column as consecutive lower-code codewords.
    pub fn encode(
        &self,
        message: &[BinaryExtensionField],
    ) -> Result<Vec<BinaryExtensionField>, Error> {
        let output_len = self
            .encoded_rows()
            .checked_mul(self.row_factor)
            .ok_or(Error::EncodingError)?;
        let mut output = vec![BinaryExtensionField::ZERO; output_len];
        self.encode_into(message, &mut output)?;
        Ok(output)
    }

    /// Encodes one logical source column into a caller-owned allocation.
    pub fn encode_into(
        &self,
        message: &[BinaryExtensionField],
        output: &mut [BinaryExtensionField],
    ) -> Result<(), Error> {
        let message_rows = self.encoding_message_rows();
        let encoded_rows = self.encoded_rows();
        if message.len() != self.logical_rows()?
            || output.len()
                != encoded_rows
                    .checked_mul(self.row_factor)
                    .ok_or(Error::EncodingError)?
        {
            return Err(Error::EncodingError);
        }
        #[cfg(feature = "parallel")]
        output
            .par_chunks_mut(encoded_rows)
            .zip(message.par_chunks(message_rows))
            .try_for_each_init(
                || vec![BinaryExtensionField::ZERO; encoded_rows],
                |scratch, (encoded, block)| {
                    self.target
                        .layout
                        .encode_additive_into_with_scratch(block, encoded, scratch, false)
                },
            )?;
        #[cfg(not(feature = "parallel"))]
        {
            let mut scratch = vec![BinaryExtensionField::ZERO; encoded_rows];
            output
                .chunks_mut(encoded_rows)
                .zip(message.chunks(message_rows))
                .try_for_each(|(encoded, block)| {
                    self.target.layout.encode_additive_into_with_scratch(
                        block,
                        encoded,
                        &mut scratch,
                        false,
                    )
                })?;
        }
        Ok(())
    }

    /// Encodes column-major logical columns in `(logical_column, block)` order.
    pub fn encode_columns(
        &self,
        columns: &[Vec<BinaryExtensionField>],
    ) -> Result<Vec<BinaryExtensionField>, Error> {
        let logical_rows = self.logical_rows()?;
        if columns.is_empty() || columns.iter().any(|column| column.len() != logical_rows) {
            return Err(Error::EncodingError);
        }
        let encoded_rows = self.encoded_rows();
        let message_rows = self.encoding_message_rows();
        let physical_columns = self.physical_columns(columns.len())?;
        let output_len = encoded_rows
            .checked_mul(physical_columns)
            .ok_or(Error::EncodingError)?;
        let mut output = vec![BinaryExtensionField::ZERO; output_len];
        #[cfg(feature = "parallel")]
        output
            .par_chunks_mut(encoded_rows)
            .enumerate()
            .try_for_each_init(
                || vec![BinaryExtensionField::ZERO; encoded_rows],
                |scratch, (physical_column, encoded)| {
                    let logical_column = physical_column / self.row_factor;
                    let block = physical_column % self.row_factor;
                    let start = block
                        .checked_mul(message_rows)
                        .ok_or(Error::EncodingError)?;
                    let end = start
                        .checked_add(message_rows)
                        .ok_or(Error::EncodingError)?;
                    self.target.layout.encode_additive_into_with_scratch(
                        &columns[logical_column][start..end],
                        encoded,
                        scratch,
                        false,
                    )
                },
            )?;
        #[cfg(not(feature = "parallel"))]
        {
            let mut scratch = vec![BinaryExtensionField::ZERO; encoded_rows];
            output.chunks_mut(encoded_rows).enumerate().try_for_each(
                |(physical_column, encoded)| {
                    let logical_column = physical_column / self.row_factor;
                    let block = physical_column % self.row_factor;
                    let start = block
                        .checked_mul(message_rows)
                        .ok_or(Error::EncodingError)?;
                    let end = start
                        .checked_add(message_rows)
                        .ok_or(Error::EncodingError)?;
                    self.target.layout.encode_additive_into_with_scratch(
                        &columns[logical_column][start..end],
                        encoded,
                        &mut scratch,
                        false,
                    )
                },
            )?;
        }
        Ok(output)
    }

    /// Encodes an already column-major logical matrix without cloning columns.
    pub fn encode_column_major(
        &self,
        entries: &[BinaryExtensionField],
        logical_columns: usize,
    ) -> Result<Vec<BinaryExtensionField>, Error> {
        let logical_rows = self.logical_rows()?;
        if logical_columns == 0
            || entries.len()
                != logical_rows
                    .checked_mul(logical_columns)
                    .ok_or(Error::EncodingError)?
        {
            return Err(Error::EncodingError);
        }
        let encoded_rows = self.encoded_rows();
        let message_rows = self.encoding_message_rows();
        let physical_columns = self.physical_columns(logical_columns)?;
        let mut output = vec![
            BinaryExtensionField::ZERO;
            encoded_rows
                .checked_mul(physical_columns)
                .ok_or(Error::EncodingError)?
        ];
        let encode_physical =
            |physical_column: usize,
             encoded: &mut [BinaryExtensionField],
             scratch: &mut Vec<BinaryExtensionField>| {
                let logical_column = physical_column / self.row_factor;
                let block = physical_column % self.row_factor;
                let logical_offset = logical_column
                    .checked_mul(logical_rows)
                    .ok_or(Error::EncodingError)?;
                let block_offset = block
                    .checked_mul(message_rows)
                    .ok_or(Error::EncodingError)?;
                let start = logical_offset
                    .checked_add(block_offset)
                    .ok_or(Error::EncodingError)?;
                let end = start
                    .checked_add(message_rows)
                    .ok_or(Error::EncodingError)?;
                self.target.layout.encode_additive_into_with_scratch(
                    &entries[start..end],
                    encoded,
                    scratch,
                    false,
                )
            };
        #[cfg(feature = "parallel")]
        output
            .par_chunks_mut(encoded_rows)
            .enumerate()
            .try_for_each_init(
                || vec![BinaryExtensionField::ZERO; encoded_rows],
                |scratch, (physical_column, encoded)| {
                    encode_physical(physical_column, encoded, scratch)
                },
            )?;
        #[cfg(not(feature = "parallel"))]
        {
            let mut scratch = vec![BinaryExtensionField::ZERO; encoded_rows];
            output.chunks_mut(encoded_rows).enumerate().try_for_each(
                |(physical_column, encoded)| {
                    encode_physical(physical_column, encoded, &mut scratch)
                },
            )?;
        }
        Ok(output)
    }

    /// Encodes column-major logical entries directly into row-major codeword rows.
    pub fn encode_column_major_into_row_major(
        &self,
        entries: &[BinaryExtensionField],
        logical_columns: usize,
        output: &mut [BinaryExtensionField],
    ) -> Result<(), Error> {
        let logical_rows = self.logical_rows()?;
        let physical_columns = self.physical_columns(logical_columns)?;
        let expected_message_entries = logical_rows
            .checked_mul(logical_columns)
            .ok_or(Error::EncodingError)?;
        let expected_encoded_entries = self
            .encoded_rows()
            .checked_mul(physical_columns)
            .ok_or(Error::EncodingError)?;
        if entries.len() != expected_message_entries || output.len() != expected_encoded_entries {
            return Err(Error::EncodingError);
        }
        self.target
            .layout
            .encode_interleaved_column_major_into_row_major(entries, physical_columns, output)
    }

    /// Projects a vector through the transpose of the direct block code.
    pub fn left_projection(
        &self,
        left: &[BinaryExtensionField],
    ) -> Result<Vec<BinaryExtensionField>, Error> {
        let encoded_rows = self.encoded_rows();
        let message_rows = self.encoding_message_rows();
        if left.len()
            != encoded_rows
                .checked_mul(self.row_factor)
                .ok_or(Error::EncodingError)?
        {
            return Err(Error::EncodingError);
        }
        let mut projected = vec![BinaryExtensionField::ZERO; self.logical_rows()?];
        #[cfg(feature = "parallel")]
        projected
            .par_chunks_mut(message_rows)
            .zip(left.par_chunks(encoded_rows))
            .try_for_each(|(target, block)| {
                let block_projection = self.target.generator_left_projection(block)?;
                target.copy_from_slice(&block_projection);
                Ok::<(), Error>(())
            })?;
        #[cfg(not(feature = "parallel"))]
        projected
            .chunks_mut(message_rows)
            .zip(left.chunks(encoded_rows))
            .try_for_each(|(target, block)| {
                let block_projection = self.target.generator_left_projection(block)?;
                target.copy_from_slice(&block_projection);
                Ok::<(), Error>(())
            })?;
        Ok(projected)
    }

    /// Folds the block coordinate of one logical column at an MLE point.
    ///
    /// Equality-indicator coordinates are low-bit first, so the prefix selects
    /// a row inside a `k_{j-1}` block and the suffix selects the block.
    pub fn fold_logical_column(
        &self,
        entries: &[BinaryExtensionField],
        logical_row_point: &[BinaryExtensionField],
    ) -> Result<Vec<BinaryExtensionField>, Error> {
        let logical_rows = self.logical_rows()?;
        let message_rows = self.encoding_message_rows();
        let base_log = message_rows.ilog2() as usize;
        if entries.len() != logical_rows || logical_row_point.len() != logical_rows.ilog2() as usize
        {
            return Err(Error::InvalidNumberOfVariables);
        }
        let block_weights = eq_ind_partial_eval_scalars(&logical_row_point[base_log..]);
        if block_weights.len() != self.row_factor {
            return Err(Error::InvalidNumberOfVariables);
        }
        let fold_row = |row: usize| {
            block_weights
                .iter()
                .enumerate()
                .map(|(block, weight)| entries[block * message_rows + row] * *weight)
                .sum()
        };
        #[cfg(feature = "parallel")]
        let folded = (0..message_rows).into_par_iter().map(fold_row).collect();
        #[cfg(not(feature = "parallel"))]
        let folded = (0..message_rows).map(fold_row).collect();
        Ok(folded)
    }

    /// Expands logical column weights to `(logical_column, block)` weights.
    pub fn physical_column_weights(
        &self,
        padded_logical_weights: &[BinaryExtensionField],
        logical_columns: usize,
        logical_row_point: &[BinaryExtensionField],
    ) -> Result<Vec<BinaryExtensionField>, Error> {
        let logical_rows = self.logical_rows()?;
        let base_log = self.encoding_message_rows().ilog2() as usize;
        if logical_columns == 0
            || padded_logical_weights.is_empty()
            || !padded_logical_weights.len().is_power_of_two()
            || logical_columns
                .checked_next_power_of_two()
                .filter(|padded| *padded == padded_logical_weights.len())
                .is_none()
            || logical_row_point.len() != logical_rows.ilog2() as usize
        {
            return Err(Error::InvalidNumberOfVariables);
        }
        let block_weights = eq_ind_partial_eval_scalars(&logical_row_point[base_log..]);
        Ok(padded_logical_weights[..logical_columns]
            .iter()
            .flat_map(|logical_weight| {
                block_weights
                    .iter()
                    .map(move |block_weight| *logical_weight * *block_weight)
            })
            .collect())
    }

    /// Returns the target code's direct proximity-query count with no RF loss.
    ///
    /// Direct interleaving applies no protocol-level distance degradation, so
    /// this intentionally uses the lower code's asserted code distance rather
    /// than its separately configured top-level query distance.
    pub fn num_queries(&self, security_bits: usize) -> Result<usize, Error> {
        Ok(queries_for_relative_distance(
            security_bits,
            self.target.distance_profile().code_relative_distance(),
        )?
        .min(self.encoded_rows()))
    }
}

/// Shape of the setup encoding that commits fixed `H_j` with `C_{j-1}`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RaaDirectInterleavedSetupShape {
    /// Current generator level `j`.
    pub level: usize,
    /// Logical rows in `H_j`, equal to `k_j`.
    pub logical_rows: usize,
    /// Message rows consumed by `C_{j-1}`.
    pub encoding_message_rows: usize,
    /// Encoded rows produced by `C_{j-1}`.
    pub encoded_rows: usize,
    /// Logical fixed columns, equal to `2 * rate_inv`.
    pub logical_h_columns: usize,
    /// Physical committed columns, equal to `2 * rate_inv * switch_ratio`.
    pub physical_h_columns: usize,
}

/// Row-major setup codeword ready for an outer Merkle commitment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RaaDirectInterleavedFixedH {
    shape: RaaDirectInterleavedSetupShape,
    encoded_entries: Vec<BinaryExtensionField>,
}

impl RaaDirectInterleavedFixedH {
    /// Returns the logical and physical dimensions for the commitment layer.
    pub fn shape(&self) -> RaaDirectInterleavedSetupShape {
        self.shape
    }

    /// Returns the complete row-major lower-code encoding.
    pub fn encoded_entries(&self) -> &[BinaryExtensionField] {
        &self.encoded_entries
    }

    /// Moves the complete row-major encoding out of this setup object.
    pub fn into_encoded_entries(self) -> Vec<BinaryExtensionField> {
        self.encoded_entries
    }

    /// Returns one row for hashing by the enclosing commitment scheme.
    pub fn encoded_row(&self, row: usize) -> Result<Vec<BinaryExtensionField>, Error> {
        let expected_len = self
            .shape
            .encoded_rows
            .checked_mul(self.shape.physical_h_columns)
            .ok_or(Error::EncodingError)?;
        if row >= self.shape.encoded_rows || self.encoded_entries.len() != expected_len {
            return Err(Error::EncodingError);
        }
        let start = row
            .checked_mul(self.shape.physical_h_columns)
            .ok_or(Error::EncodingError)?;
        let end = start
            .checked_add(self.shape.physical_h_columns)
            .ok_or(Error::EncodingError)?;
        self.encoded_entries
            .get(start..end)
            .map(<[BinaryExtensionField]>::to_vec)
            .ok_or(Error::EncodingError)
    }
}

/// A geometric RAA sequence with rate `1/4` at every level.
#[derive(Clone, Debug)]
pub struct RaaFold {
    codes: Vec<RaaCode>,
    top_switch_ratio: usize,
    recursive_switch_ratio: usize,
    generator_digest: [u8; 32],
}

impl RaaFold {
    /// Builds a sequence ordered from smallest to largest message length.
    ///
    /// At least two levels are required; every level uses repetition four.
    /// Callers selecting the conditional Blaze experiment profile should construct
    /// each profile with [`RaaDistanceProfile::blaze_assumed`].
    pub fn new(codes: Vec<RaaCode>, switch_ratio: usize) -> Result<Self, Error> {
        Self::new_with_switch_ratios(codes, switch_ratio, switch_ratio)
    }

    /// Builds a sequence with separate highest and recursive switch ratios.
    ///
    /// The transition from the largest code uses `top_switch_ratio`; every
    /// remaining transition uses `recursive_switch_ratio`.
    pub fn new_with_switch_ratios(
        codes: Vec<RaaCode>,
        top_switch_ratio: usize,
        recursive_switch_ratio: usize,
    ) -> Result<Self, Error> {
        let levels = codes.len();
        if levels < 2
            || codes
                .iter()
                .enumerate()
                .any(|(level, code)| code.rate_inv() != raafold_rate_inv_for_level(level, levels))
            || top_switch_ratio <= 1
            || !top_switch_ratio.is_power_of_two()
            || recursive_switch_ratio <= 1
            || !recursive_switch_ratio.is_power_of_two()
        {
            return Err(Error::EncodingError);
        }
        for (target_level, pair) in codes.windows(2).enumerate() {
            let source_level = target_level + 1;
            let switch_ratio = if source_level + 1 == levels {
                top_switch_ratio
            } else {
                recursive_switch_ratio
            };
            if pair[0]
                .message_len()
                .checked_mul(switch_ratio)
                .ok_or(Error::EncodingError)?
                != pair[1].message_len()
            {
                return Err(Error::EncodingError);
            }
        }
        let generator_digest =
            raa_sequence_generator_digest(&codes, top_switch_ratio, recursive_switch_ratio)?;
        Ok(Self {
            codes,
            top_switch_ratio,
            recursive_switch_ratio,
            generator_digest,
        })
    }

    /// Returns one recursive code level.
    pub fn code(&self, level: usize) -> Result<&RaaCode, Error> {
        self.codes.get(level).ok_or(Error::EncodingError)
    }

    /// Returns the number of recursive levels.
    pub fn levels(&self) -> usize {
        self.codes.len()
    }

    /// Returns all RAA codes in ascending message-length order.
    pub fn codes(&self) -> &[RaaCode] {
        &self.codes
    }

    /// Returns the largest RAA code in the sequence.
    pub fn top_code(&self) -> Result<&RaaCode, Error> {
        self.codes.last().ok_or(Error::EncodingError)
    }

    /// Returns the switch ratio used by the highest transition.
    pub fn switch_ratio(&self) -> usize {
        self.top_switch_ratio
    }

    /// Returns the switch ratio used after the highest transition.
    pub fn recursive_switch_ratio(&self) -> usize {
        self.recursive_switch_ratio
    }

    /// Returns the switch ratio from `source_level` to the preceding level.
    pub fn switch_ratio_for_source_level(&self, source_level: usize) -> Result<usize, Error> {
        if source_level == 0 || source_level >= self.codes.len() {
            return Err(Error::EncodingError);
        }
        Ok(if source_level + 1 == self.codes.len() {
            self.top_switch_ratio
        } else {
            self.recursive_switch_ratio
        })
    }

    /// Returns the number of RF layers used by direct interleaving.
    pub const fn proximity_rf_layers(&self) -> usize {
        0
    }

    /// Returns the digest binding the direct mode, geometry, and every code.
    pub fn generator_digest(&self) -> [u8; 32] {
        self.generator_digest
    }

    /// Returns public sequence setup elements for the outer transcript.
    ///
    /// The enclosing PCS must absorb these elements after its setup
    /// commitments and before deriving any opening challenge.
    pub fn transcript_setup_elements(&self) -> [BinaryExtensionField; 4] {
        let mut low = [0u8; 16];
        let mut high = [0u8; 16];
        low.copy_from_slice(&self.generator_digest[..16]);
        high.copy_from_slice(&self.generator_digest[16..]);
        [
            BinaryExtensionField::from(self.codes.len() as u128),
            BinaryExtensionField::from(self.top_switch_ratio as u128),
            BinaryExtensionField::from(u128::from_le_bytes(low)),
            BinaryExtensionField::from(u128::from_le_bytes(high)),
        ]
    }

    /// Returns the direct switch from `source_level` to `C_{source_level-1}`.
    pub fn direct_interleaved_level(
        &self,
        source_level: usize,
    ) -> Result<RaaDirectInterleavedLevel, Error> {
        let target_level = source_level.checked_sub(1).ok_or(Error::EncodingError)?;
        let target = self.code(target_level)?;
        let source = self.code(source_level)?;
        let switch_ratio = self.switch_ratio_for_source_level(source_level)?;
        if source.message_len()
            != target
                .message_len()
                .checked_mul(switch_ratio)
                .ok_or(Error::EncodingError)?
        {
            return Err(Error::EncodingError);
        }
        Ok(RaaDirectInterleavedLevel {
            source_level,
            target_level,
            row_factor: switch_ratio,
            target: target.clone(),
        })
    }

    /// Encodes fixed `H_j` directly with `C_{j-1}` and no `T` columns.
    ///
    /// The fixed matrix is streamed from permutation source indices one block
    /// at a time. It is never materialized as a padded `k_j x 4r` matrix.
    pub fn encode_direct_interleaved_fixed_h(
        &self,
        level: usize,
    ) -> Result<RaaDirectInterleavedFixedH, Error> {
        let direct = self.direct_interleaved_level(level)?;
        let current = self.code(level)?;
        let h_layout = current.fixed_h_layout();
        let message_rows = direct.encoding_message_rows();
        let encoded_rows = direct.encoded_rows();
        if h_layout.rows != direct.logical_rows()? {
            return Err(Error::EncodingError);
        }
        let physical_h_columns = direct.physical_columns(h_layout.columns)?;
        let index_powers = permutation_index_powers(current.codeword_len())?;
        let encoded_len = encoded_rows
            .checked_mul(physical_h_columns)
            .ok_or(Error::EncodingError)?;
        let mut encoded_entries = vec![BinaryExtensionField::ZERO; encoded_len];
        let permutation_sources = [
            current.layout.permutation(1)?.sources(),
            current.layout.permutation(2)?.sources(),
        ];
        direct
            .target
            .layout
            .encode_interleaved_generated_into_row_major(
                physical_h_columns,
                &mut encoded_entries,
                |message_row, physical_column| {
                    let logical_column = physical_column / direct.row_factor;
                    let block = physical_column % direct.row_factor;
                    let logical_row = block * message_rows + message_row;
                    let round = logical_column / h_layout.columns_per_permutation;
                    let rate_column = logical_column % h_layout.columns_per_permutation;
                    let source_offset = rate_column * h_layout.rows + logical_row;
                    index_powers[permutation_sources[round][source_offset] as usize]
                },
            )?;

        Ok(RaaDirectInterleavedFixedH {
            shape: RaaDirectInterleavedSetupShape {
                level,
                logical_rows: h_layout.rows,
                encoding_message_rows: message_rows,
                encoded_rows,
                logical_h_columns: h_layout.columns,
                physical_h_columns,
            },
            encoded_entries,
        })
    }

    /// Returns the query count for an ordinary codeword opening at one level.
    ///
    /// Recursive setup openings must instead call
    /// [`direct_interleaved_num_queries`](Self::direct_interleaved_num_queries)
    /// so the lower code and its undegraded distance are selected.
    pub fn opening_num_queries(
        &self,
        level: usize,
        security_bits: usize,
        available_rows: usize,
    ) -> Result<usize, Error> {
        self.code(level)?.num_queries(security_bits, available_rows)
    }

    /// Returns setup-query count for level `j` from the lower code `C_{j-1}`.
    pub fn direct_interleaved_num_queries(
        &self,
        source_level: usize,
        security_bits: usize,
    ) -> Result<usize, Error> {
        self.direct_interleaved_level(source_level)?
            .num_queries(security_bits)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RaaFactor {
    Accumulator(usize),
    Permutation(usize),
    Repeat,
}

const RAA_FACTORS: [RaaFactor; RAA_FACTOR_COUNT] = [
    RaaFactor::Accumulator(2),
    RaaFactor::Permutation(2),
    RaaFactor::Accumulator(1),
    RaaFactor::Permutation(1),
    RaaFactor::Repeat,
];

fn prove_factor_chain<Channel>(
    layout: &RaaLayout,
    claim: &RaaMatrixClaim<BinaryExtensionField>,
    start: usize,
    end: usize,
    channel: &mut Channel,
    leaves: &mut Vec<(RaaFactor, RaaMatrixClaim<BinaryExtensionField>)>,
) -> Result<(), Error>
where
    Channel: IPProverChannel<BinaryExtensionField>,
{
    if start == end {
        leaves.push((RAA_FACTORS[start], claim.clone()));
        return Ok(());
    }
    let split = (start + end) / 2;
    let row_weights = eq_ind_partial_eval_scalars(&claim.point.0);
    let column_weights = eq_ind_partial_eval_scalars(&claim.point.1);
    let left_values = apply_subchain_left(layout, start, split, row_weights)?;
    let right_values = apply_subchain_right(layout, split + 1, end, column_weights)?;
    let (left_reduction, right_reduction) =
        prove_projected_litebullet(claim.value, left_values, right_values, channel)?;
    let left_claim = RaaMatrixClaim {
        value: left_reduction.value,
        point: (claim.point.0.clone(), left_reduction.point.1),
    };
    let right_claim = RaaMatrixClaim {
        value: right_reduction.value,
        point: (right_reduction.point.0, claim.point.1.clone()),
    };
    prove_factor_chain(layout, &left_claim, start, split, channel, leaves)?;
    prove_factor_chain(layout, &right_claim, split + 1, end, channel, leaves)
}

fn verify_factor_chain<Channel>(
    layout: &RaaLayout,
    claim: &RaaMatrixClaim<Channel::Elem>,
    start: usize,
    end: usize,
    channel: &mut Channel,
    leaves: &mut Vec<(RaaFactor, RaaMatrixClaim<Channel::Elem>)>,
) -> Result<(), Error>
where
    Channel: IPVerifierChannel<BinaryExtensionField, Elem = BinaryExtensionField>,
{
    if start == end {
        leaves.push((RAA_FACTORS[start], claim.clone()));
        return Ok(());
    }
    let split = (start + end) / 2;
    let log_intermediate_len = layout.codeword_len().ilog2() as usize;
    let (left_reduction, right_reduction) =
        verify_projected_litebullet(log_intermediate_len, claim.value.clone(), channel)?;
    let left_claim = RaaMatrixClaim {
        value: left_reduction.value,
        point: (claim.point.0.clone(), left_reduction.point.1),
    };
    let right_claim = RaaMatrixClaim {
        value: right_reduction.value,
        point: (right_reduction.point.0, claim.point.1.clone()),
    };
    verify_factor_chain(layout, &left_claim, start, split, channel, leaves)?;
    verify_factor_chain(layout, &right_claim, split + 1, end, channel, leaves)
}

/// Compute both round messages in one scheduled pass over L2-sized tiles.
/// All three dot products in a tile reuse the same four input slices.
fn projected_litebullet_round(
    left_low: &[BinaryExtensionField],
    left_high: &[BinaryExtensionField],
    right_low: &[BinaryExtensionField],
    right_high: &[BinaryExtensionField],
) -> (BinaryExtensionField, BinaryExtensionField) {
    const CHUNK_LEN: usize = 1 << 12;
    let round_chunk = |left_low: &[BinaryExtensionField],
                       left_high: &[BinaryExtensionField],
                       right_low: &[BinaryExtensionField],
                       right_high: &[BinaryExtensionField]| {
        (
            BinaryExtensionField::dot_product(left_high, right_high),
            BinaryExtensionField::dot_product(left_low, right_high)
                + BinaryExtensionField::dot_product(left_high, right_low),
        )
    };
    let combine = |(a, b), (c, d)| (a + c, b + d);
    #[cfg(feature = "parallel")]
    if left_low.len() >= PARALLEL_FIELD_THRESHOLD && rayon::current_num_threads() > 1 {
        return left_low
            .par_chunks(CHUNK_LEN)
            .zip(left_high.par_chunks(CHUNK_LEN))
            .zip(right_low.par_chunks(CHUNK_LEN))
            .zip(right_high.par_chunks(CHUNK_LEN))
            .map(|(((left_low, left_high), right_low), right_high)| {
                round_chunk(left_low, left_high, right_low, right_high)
            })
            .reduce(
                || (BinaryExtensionField::ZERO, BinaryExtensionField::ZERO),
                combine,
            );
    }
    left_low
        .chunks(CHUNK_LEN)
        .zip(left_high.chunks(CHUNK_LEN))
        .zip(right_low.chunks(CHUNK_LEN))
        .zip(right_high.chunks(CHUNK_LEN))
        .map(|(((left_low, left_high), right_low), right_high)| {
            round_chunk(left_low, left_high, right_low, right_high)
        })
        .fold(
            (BinaryExtensionField::ZERO, BinaryExtensionField::ZERO),
            combine,
        )
}

fn fold_projected_litebullet_pair(
    left_low: &mut [BinaryExtensionField],
    left_high: &[BinaryExtensionField],
    right_low: &mut [BinaryExtensionField],
    right_high: &[BinaryExtensionField],
    challenge: BinaryExtensionField,
) {
    const CHUNK_LEN: usize = 1 << 12;
    let fold_chunk = |left_low: &mut [BinaryExtensionField],
                      left_high: &[BinaryExtensionField],
                      right_low: &mut [BinaryExtensionField],
                      right_high: &[BinaryExtensionField]| {
        BinaryExtensionField::fold_equality_slice(left_low, left_high, challenge);
        BinaryExtensionField::fold_equality_slice(right_low, right_high, challenge);
    };
    #[cfg(feature = "parallel")]
    if left_low.len() >= PARALLEL_FIELD_THRESHOLD && rayon::current_num_threads() > 1 {
        left_low
            .par_chunks_mut(CHUNK_LEN)
            .zip(left_high.par_chunks(CHUNK_LEN))
            .zip(right_low.par_chunks_mut(CHUNK_LEN))
            .zip(right_high.par_chunks(CHUNK_LEN))
            .for_each(|(((left_low, left_high), right_low), right_high)| {
                fold_chunk(left_low, left_high, right_low, right_high);
            });
        return;
    }
    left_low
        .chunks_mut(CHUNK_LEN)
        .zip(left_high.chunks(CHUNK_LEN))
        .zip(right_low.chunks_mut(CHUNK_LEN))
        .zip(right_high.chunks(CHUNK_LEN))
        .for_each(|(((left_low, left_high), right_low), right_high)| {
            fold_chunk(left_low, left_high, right_low, right_high);
        });
}

fn prove_projected_litebullet<Channel>(
    claimed_value: BinaryExtensionField,
    mut left: Vec<BinaryExtensionField>,
    mut right: Vec<BinaryExtensionField>,
    channel: &mut Channel,
) -> Result<
    (
        RaaMatrixClaim<BinaryExtensionField>,
        RaaMatrixClaim<BinaryExtensionField>,
    ),
    Error,
>
where
    Channel: IPProverChannel<BinaryExtensionField>,
{
    if left.is_empty()
        || !left.len().is_power_of_two()
        || left.len() != right.len()
        || inner_product(&left, &right) != claimed_value
    {
        return Err(Error::InvalidCommitment);
    }
    let mut rhs = claimed_value;
    let mut challenges = Vec::with_capacity(left.len().ilog2() as usize);
    while left.len() > 1 {
        let half = left.len() / 2;
        let (high_diagonal, cross_sum) = projected_litebullet_round(
            &left[..half],
            &left[half..],
            &right[..half],
            &right[half..],
        );
        channel.send_many(&[high_diagonal, cross_sum]);
        let challenge = channel.sample();
        rhs = multilinear_inner_product_round(rhs, high_diagonal, cross_sum, challenge);
        let (left_low, left_high) = left.split_at_mut(half);
        let (right_low, right_high) = right.split_at_mut(half);
        fold_projected_litebullet_pair(left_low, left_high, right_low, right_high, challenge);
        left.truncate(half);
        right.truncate(half);
        challenges.push(challenge);
    }
    let left_value = left[0];
    let right_value = right[0];
    if left_value * right_value != rhs {
        return Err(Error::InvalidCommitment);
    }
    channel.send_many(&[left_value, right_value]);
    Ok(projected_litebullet_from_raw(
        left_value,
        right_value,
        &challenges,
    ))
}

/// Reduces one SwitchFold auxiliary-oracle inner product to MLE claims.
///
/// The first returned claim opens the dynamic oracle. The second opens the
/// public transparent weight vector and is checked directly by the verifier.
pub(crate) fn prove_switchfold_oracle_relation<Channel>(
    claimed_value: BinaryExtensionField,
    oracle: Vec<BinaryExtensionField>,
    transparent: Vec<BinaryExtensionField>,
    channel: &mut Channel,
) -> Result<
    (
        RaaMatrixClaim<BinaryExtensionField>,
        RaaMatrixClaim<BinaryExtensionField>,
    ),
    Error,
>
where
    Channel: IPProverChannel<BinaryExtensionField>,
{
    prove_projected_litebullet(claimed_value, oracle, transparent, channel)
}

fn verify_projected_litebullet<Channel>(
    num_variables: usize,
    mut claimed_value: BinaryExtensionField,
    channel: &mut Channel,
) -> Result<
    (
        RaaMatrixClaim<BinaryExtensionField>,
        RaaMatrixClaim<BinaryExtensionField>,
    ),
    Error,
>
where
    Channel: IPVerifierChannel<BinaryExtensionField, Elem = BinaryExtensionField>,
{
    let mut challenges = Vec::with_capacity(num_variables);
    for _ in 0..num_variables {
        let [high_diagonal, cross_sum] = channel
            .recv_array::<2>()
            .map_err(|_| Error::TranscriptError)?;
        let challenge = channel.sample();
        claimed_value =
            multilinear_inner_product_round(claimed_value, high_diagonal, cross_sum, challenge);
        challenges.push(challenge);
    }
    let [left_value, right_value] = channel
        .recv_array::<2>()
        .map_err(|_| Error::TranscriptError)?;
    channel
        .assert_zero(left_value * right_value - claimed_value)
        .map_err(|_| Error::InvalidCommitment)?;
    Ok(projected_litebullet_from_raw(
        left_value,
        right_value,
        &challenges,
    ))
}

/// Replays a SwitchFold auxiliary-oracle inner-product reduction.
pub(crate) fn verify_switchfold_oracle_relation<Channel>(
    num_variables: usize,
    claimed_value: BinaryExtensionField,
    channel: &mut Channel,
) -> Result<
    (
        RaaMatrixClaim<BinaryExtensionField>,
        RaaMatrixClaim<BinaryExtensionField>,
    ),
    Error,
>
where
    Channel: IPVerifierChannel<BinaryExtensionField, Elem = BinaryExtensionField>,
{
    verify_projected_litebullet(num_variables, claimed_value, channel)
}

fn projected_litebullet_from_raw(
    left_value: BinaryExtensionField,
    right_value: BinaryExtensionField,
    challenges: &[BinaryExtensionField],
) -> (
    RaaMatrixClaim<BinaryExtensionField>,
    RaaMatrixClaim<BinaryExtensionField>,
) {
    let point = challenges.iter().rev().copied().collect::<Vec<_>>();
    (
        RaaMatrixClaim {
            value: left_value,
            point: (Vec::new(), point.clone()),
        },
        RaaMatrixClaim {
            value: right_value,
            point: (point, Vec::new()),
        },
    )
}

#[inline]
pub(crate) fn multilinear_fold_pair(
    low: BinaryExtensionField,
    high: BinaryExtensionField,
    challenge: BinaryExtensionField,
) -> BinaryExtensionField {
    low + (low + high) * challenge
}

#[inline]
pub(crate) fn multilinear_inner_product_round(
    claim: BinaryExtensionField,
    high_diagonal: BinaryExtensionField,
    cross_sum: BinaryExtensionField,
    challenge: BinaryExtensionField,
) -> BinaryExtensionField {
    let challenge_squared = challenge * challenge;
    claim * (BinaryExtensionField::ONE + challenge_squared)
        + high_diagonal
        + cross_sum * (challenge + challenge_squared)
}

fn prove_hadamard_matrix<Channel>(
    claimed_value: BinaryExtensionField,
    output_point: &(Vec<BinaryExtensionField>, Vec<BinaryExtensionField>),
    rows: usize,
    columns: usize,
    left: &mut [BinaryExtensionField],
    right: &mut [BinaryExtensionField],
    channel: &mut Channel,
) -> Result<RaaHadamardReduction<BinaryExtensionField>, Error>
where
    Channel: IPProverChannel<BinaryExtensionField>,
{
    validate_hadamard_shape(output_point, rows, columns, left, right)?;
    let row_weights = eq_ind_partial_eval_scalars(&output_point.0);
    let column_weights = eq_ind_partial_eval_scalars(&output_point.1);
    if hadamard_output_eval(left, right, rows, columns, &row_weights, &column_weights)
        != claimed_value
    {
        return Err(Error::InvalidCommitment);
    }

    let mut rhs = claimed_value;
    let mut equality_prefix = BinaryExtensionField::ONE;
    let mut column_challenges = Vec::with_capacity(columns.ilog2() as usize);
    let mut current_columns = columns;
    while current_columns > 1 {
        let half_columns = current_columns / 2;
        let coordinate = output_point.1[half_columns.ilog2() as usize];
        let remaining_column_weights =
            eq_ind_partial_eval_scalars(&output_point.1[..half_columns.ilog2() as usize]);
        let coefficients = hadamard_column_sumcheck_coefficients(
            left,
            right,
            rows,
            half_columns,
            coordinate,
            equality_prefix,
            &row_weights,
            &remaining_column_weights,
        );
        channel.send_many(&coefficients);
        let challenge = channel.sample();
        rhs = cubic_sumcheck_round(rhs, coefficients, challenge);
        equality_prefix *= BinaryExtensionField::ONE + coordinate + challenge;
        fold_hadamard_columns(left, rows, half_columns, challenge);
        fold_hadamard_columns(right, rows, half_columns, challenge);
        column_challenges.push(challenge);
        current_columns = half_columns;
    }

    let mut row_challenges = Vec::with_capacity(rows.ilog2() as usize);
    let mut current_rows = rows;
    while current_rows > 1 {
        let half_rows = current_rows / 2;
        let coordinate = output_point.0[half_rows.ilog2() as usize];
        let remaining_row_weights =
            eq_ind_partial_eval_scalars(&output_point.0[..half_rows.ilog2() as usize]);
        let coefficients = hadamard_row_sumcheck_coefficients(
            left,
            right,
            half_rows,
            coordinate,
            equality_prefix,
            &remaining_row_weights,
        );
        channel.send_many(&coefficients);
        let challenge = channel.sample();
        rhs = cubic_sumcheck_round(rhs, coefficients, challenge);
        equality_prefix *= BinaryExtensionField::ONE + coordinate + challenge;
        fold_hadamard_rows(left, half_rows, current_rows, challenge);
        fold_hadamard_rows(right, half_rows, current_rows, challenge);
        row_challenges.push(challenge);
        current_rows = half_rows;
    }

    let left_value = left[0];
    let right_value = right[0];
    let row_point = row_challenges.iter().rev().copied().collect::<Vec<_>>();
    let column_point = column_challenges.iter().rev().copied().collect::<Vec<_>>();
    let expected = eq_ind(&output_point.0, &row_point)
        * eq_ind(&output_point.1, &column_point)
        * left_value
        * right_value;
    if expected != rhs {
        return Err(Error::InvalidCommitment);
    }
    channel.send_many(&[left_value, right_value]);
    Ok(hadamard_reduction_from_raw(
        left_value,
        right_value,
        &row_challenges,
        &column_challenges,
    ))
}

fn verify_hadamard_matrix<Channel>(
    mut claimed_value: BinaryExtensionField,
    output_point: &(Vec<BinaryExtensionField>, Vec<BinaryExtensionField>),
    rows: usize,
    columns: usize,
    channel: &mut Channel,
) -> Result<RaaHadamardReduction<BinaryExtensionField>, Error>
where
    Channel: IPVerifierChannel<BinaryExtensionField, Elem = BinaryExtensionField>,
{
    validate_hadamard_point_shape(output_point, rows, columns)?;
    let mut column_challenges = Vec::with_capacity(columns.ilog2() as usize);
    let mut current_columns = columns;
    while current_columns > 1 {
        let coefficients = channel
            .recv_array::<3>()
            .map_err(|_| Error::TranscriptError)?;
        let challenge = channel.sample();
        claimed_value = cubic_sumcheck_round(claimed_value, coefficients, challenge);
        column_challenges.push(challenge);
        current_columns /= 2;
    }

    let mut row_challenges = Vec::with_capacity(rows.ilog2() as usize);
    let mut current_rows = rows;
    while current_rows > 1 {
        let coefficients = channel
            .recv_array::<3>()
            .map_err(|_| Error::TranscriptError)?;
        let challenge = channel.sample();
        claimed_value = cubic_sumcheck_round(claimed_value, coefficients, challenge);
        row_challenges.push(challenge);
        current_rows /= 2;
    }
    let [left_value, right_value] = channel
        .recv_array::<2>()
        .map_err(|_| Error::TranscriptError)?;
    let row_point = row_challenges.iter().rev().copied().collect::<Vec<_>>();
    let column_point = column_challenges.iter().rev().copied().collect::<Vec<_>>();
    let expected = eq_ind(&output_point.0, &row_point)
        * eq_ind(&output_point.1, &column_point)
        * left_value
        * right_value;
    channel
        .assert_zero(expected - claimed_value)
        .map_err(|_| Error::InvalidCommitment)?;
    Ok(hadamard_reduction_from_raw(
        left_value,
        right_value,
        &row_challenges,
        &column_challenges,
    ))
}

fn hadamard_reduction_from_raw(
    left_value: BinaryExtensionField,
    right_value: BinaryExtensionField,
    row_challenges: &[BinaryExtensionField],
    column_challenges: &[BinaryExtensionField],
) -> RaaHadamardReduction<BinaryExtensionField> {
    let row_point = row_challenges.iter().rev().copied().collect::<Vec<_>>();
    let column_point = column_challenges.iter().rev().copied().collect::<Vec<_>>();
    RaaHadamardReduction {
        left: RaaMatrixClaim {
            value: left_value,
            point: (row_point.clone(), column_point.clone()),
        },
        right: RaaMatrixClaim {
            value: right_value,
            point: (row_point, column_point),
        },
    }
}

fn validate_hadamard_shape(
    output_point: &(Vec<BinaryExtensionField>, Vec<BinaryExtensionField>),
    rows: usize,
    columns: usize,
    left: &[BinaryExtensionField],
    right: &[BinaryExtensionField],
) -> Result<(), Error> {
    validate_hadamard_point_shape(output_point, rows, columns)?;
    let entries = rows.checked_mul(columns).ok_or(Error::EncodingError)?;
    if left.len() != entries || right.len() != entries {
        return Err(Error::InvalidNumberOfVariables);
    }
    Ok(())
}

fn validate_hadamard_point_shape(
    output_point: &(Vec<BinaryExtensionField>, Vec<BinaryExtensionField>),
    rows: usize,
    columns: usize,
) -> Result<(), Error> {
    if rows == 0
        || columns == 0
        || !rows.is_power_of_two()
        || !columns.is_power_of_two()
        || output_point.0.len() != rows.ilog2() as usize
        || output_point.1.len() != columns.ilog2() as usize
    {
        return Err(Error::InvalidNumberOfVariables);
    }
    Ok(())
}

fn hadamard_output_eval(
    left: &[BinaryExtensionField],
    right: &[BinaryExtensionField],
    rows: usize,
    columns: usize,
    row_weights: &[BinaryExtensionField],
    column_weights: &[BinaryExtensionField],
) -> BinaryExtensionField {
    debug_assert_eq!(left.len(), rows * columns);
    debug_assert_eq!(right.len(), rows * columns);
    #[cfg(feature = "parallel")]
    {
        (0..rows * columns)
            .into_par_iter()
            .map(|index| {
                let row = index % rows;
                let column = index / rows;
                left[index] * right[index] * row_weights[row] * column_weights[column]
            })
            .sum()
    }
    #[cfg(not(feature = "parallel"))]
    {
        (0..rows * columns)
            .map(|index| {
                let row = index % rows;
                let column = index / rows;
                left[index] * right[index] * row_weights[row] * column_weights[column]
            })
            .sum()
    }
}

fn column_major_matrix_eval(
    matrix: &[BinaryExtensionField],
    rows: usize,
    columns: usize,
    row_weights: &[BinaryExtensionField],
    column_weights: &[BinaryExtensionField],
) -> BinaryExtensionField {
    debug_assert_eq!(matrix.len(), rows * columns);
    #[cfg(feature = "parallel")]
    {
        matrix
            .par_iter()
            .enumerate()
            .map(|(index, value)| *value * row_weights[index % rows] * column_weights[index / rows])
            .sum()
    }
    #[cfg(not(feature = "parallel"))]
    {
        matrix
            .iter()
            .enumerate()
            .map(|(index, value)| *value * row_weights[index % rows] * column_weights[index / rows])
            .sum()
    }
}

fn hadamard_column_sumcheck_coefficients(
    left: &[BinaryExtensionField],
    right: &[BinaryExtensionField],
    rows: usize,
    half_columns: usize,
    output_coordinate: BinaryExtensionField,
    prefix_weight: BinaryExtensionField,
    row_weights: &[BinaryExtensionField],
    remaining_column_weights: &[BinaryExtensionField],
) -> [BinaryExtensionField; 3] {
    debug_assert_eq!(row_weights.len(), rows);
    debug_assert_eq!(remaining_column_weights.len(), half_columns);
    let coefficients = |index: usize| {
        let column = index / rows;
        let row = index % rows;
        let low = column * rows + row;
        let high = (half_columns + column) * rows + row;
        hadamard_cubic_coefficients(
            left[low],
            left[high],
            right[low],
            right[high],
            output_coordinate,
            prefix_weight * row_weights[row] * remaining_column_weights[column],
        )
    };
    #[cfg(feature = "parallel")]
    {
        (0..half_columns * rows)
            .into_par_iter()
            .map(coefficients)
            .reduce(
                || [BinaryExtensionField::ZERO; 3],
                |left, right| core::array::from_fn(|index| left[index] + right[index]),
            )
    }
    #[cfg(not(feature = "parallel"))]
    {
        (0..half_columns * rows)
            .map(coefficients)
            .fold([BinaryExtensionField::ZERO; 3], |sum, value| {
                core::array::from_fn(|index| sum[index] + value[index])
            })
    }
}

fn fold_hadamard_columns(
    matrix: &mut [BinaryExtensionField],
    rows: usize,
    half_columns: usize,
    challenge: BinaryExtensionField,
) {
    let (low, high) = matrix.split_at_mut(half_columns * rows);
    #[cfg(feature = "parallel")]
    low.par_iter_mut()
        .zip(high[..half_columns * rows].par_iter())
        .for_each(|(low, high)| *low = multilinear_fold_pair(*low, *high, challenge));
    #[cfg(not(feature = "parallel"))]
    low.iter_mut()
        .zip(high[..half_columns * rows].iter())
        .for_each(|(low, high)| *low = multilinear_fold_pair(*low, *high, challenge));
}

fn hadamard_row_sumcheck_coefficients(
    left: &[BinaryExtensionField],
    right: &[BinaryExtensionField],
    half_rows: usize,
    output_coordinate: BinaryExtensionField,
    prefix_weight: BinaryExtensionField,
    remaining_row_weights: &[BinaryExtensionField],
) -> [BinaryExtensionField; 3] {
    debug_assert_eq!(remaining_row_weights.len(), half_rows);
    let coefficients = |row: usize| {
        hadamard_cubic_coefficients(
            left[row],
            left[half_rows + row],
            right[row],
            right[half_rows + row],
            output_coordinate,
            prefix_weight * remaining_row_weights[row],
        )
    };
    #[cfg(feature = "parallel")]
    {
        (0..half_rows).into_par_iter().map(coefficients).reduce(
            || [BinaryExtensionField::ZERO; 3],
            |left, right| core::array::from_fn(|index| left[index] + right[index]),
        )
    }
    #[cfg(not(feature = "parallel"))]
    {
        (0..half_rows)
            .map(coefficients)
            .fold([BinaryExtensionField::ZERO; 3], |sum, value| {
                core::array::from_fn(|index| sum[index] + value[index])
            })
    }
}

fn fold_hadamard_rows(
    matrix: &mut [BinaryExtensionField],
    half_rows: usize,
    current_rows: usize,
    challenge: BinaryExtensionField,
) {
    let (low, high) = matrix[..current_rows].split_at_mut(half_rows);
    #[cfg(feature = "parallel")]
    low.par_iter_mut()
        .zip(high.par_iter())
        .for_each(|(low, high)| *low = multilinear_fold_pair(*low, *high, challenge));
    #[cfg(not(feature = "parallel"))]
    low.iter_mut()
        .zip(high.iter())
        .for_each(|(low, high)| *low = multilinear_fold_pair(*low, *high, challenge));
}

fn hadamard_cubic_coefficients(
    left_low: BinaryExtensionField,
    left_high: BinaryExtensionField,
    right_low: BinaryExtensionField,
    right_high: BinaryExtensionField,
    output_coordinate: BinaryExtensionField,
    remaining_weight: BinaryExtensionField,
) -> [BinaryExtensionField; 3] {
    let left_delta = left_low + left_high;
    let right_delta = right_low + right_high;
    let constant_product = left_low * right_low;
    let linear_product = left_delta * right_low + left_low * right_delta;
    let quadratic_product = left_delta * right_delta;
    let equality_constant = BinaryExtensionField::ONE + output_coordinate;
    [
        remaining_weight * equality_constant * constant_product,
        remaining_weight * (constant_product + equality_constant * linear_product),
        remaining_weight * (linear_product + equality_constant * quadratic_product),
    ]
}

#[inline]
fn cubic_sumcheck_round(
    claim: BinaryExtensionField,
    [constant, linear, quadratic]: [BinaryExtensionField; 3],
    challenge: BinaryExtensionField,
) -> BinaryExtensionField {
    let cubic = claim + linear + quadratic;
    constant + challenge * (linear + challenge * (quadratic + challenge * cubic))
}

fn apply_subchain_right(
    layout: &RaaLayout,
    start: usize,
    end: usize,
    values: Vec<BinaryExtensionField>,
) -> Result<Vec<BinaryExtensionField>, Error> {
    let mut current = values;
    for factor in RAA_FACTORS[start..=end].iter().rev() {
        current = apply_factor_right(layout, *factor, current)?;
    }
    Ok(current)
}

fn apply_subchain_left(
    layout: &RaaLayout,
    start: usize,
    end: usize,
    values: Vec<BinaryExtensionField>,
) -> Result<Vec<BinaryExtensionField>, Error> {
    let mut current = values;
    for factor in &RAA_FACTORS[start..=end] {
        current = apply_factor_left(layout, *factor, current)?;
    }
    Ok(current)
}

fn apply_factor_right(
    layout: &RaaLayout,
    factor: RaaFactor,
    mut input: Vec<BinaryExtensionField>,
) -> Result<Vec<BinaryExtensionField>, Error> {
    match factor {
        RaaFactor::Accumulator(_) => {
            if input.len() != layout.codeword_len() {
                return Err(Error::EncodingError);
            }
            prefix_accumulate(&mut input, true);
            Ok(input)
        }
        RaaFactor::Permutation(round) => {
            if input.len() != layout.codeword_len() {
                return Err(Error::EncodingError);
            }
            let mut output = vec![BinaryExtensionField::ZERO; input.len()];
            apply_permutation(
                &input,
                layout.permutation(round)?.sources(),
                &mut output,
                true,
            );
            Ok(output)
        }
        RaaFactor::Repeat => {
            if input.len() != layout.message_len() {
                return Err(Error::EncodingError);
            }
            Ok(input
                .iter()
                .flat_map(|value| core::iter::repeat_n(*value, layout.rate_inv()))
                .collect())
        }
    }
}

fn apply_factor_left(
    layout: &RaaLayout,
    factor: RaaFactor,
    mut input: Vec<BinaryExtensionField>,
) -> Result<Vec<BinaryExtensionField>, Error> {
    if input.len() != layout.codeword_len() {
        return Err(Error::EncodingError);
    }
    match factor {
        RaaFactor::Accumulator(_) => {
            suffix_accumulate(&mut input);
            Ok(input)
        }
        RaaFactor::Permutation(round) => {
            let mut output = vec![BinaryExtensionField::ZERO; input.len()];
            apply_permutation(
                &input,
                layout.permutation(round)?.inverse_sources(),
                &mut output,
                true,
            );
            Ok(output)
        }
        RaaFactor::Repeat => Ok(input
            .chunks_exact(layout.rate_inv())
            .map(|chunk| {
                chunk
                    .iter()
                    .copied()
                    .fold(BinaryExtensionField::ZERO, |sum, value| sum + value)
            })
            .collect()),
    }
}

fn assemble_leaves<E>(
    claims: Vec<(RaaFactor, RaaMatrixClaim<E>)>,
) -> Result<RaaGeneratorLeaves<E>, Error> {
    if claims.len() != RAA_FACTOR_COUNT {
        return Err(Error::InvalidCommitment);
    }
    let mut accumulators = [None, None];
    let mut permutations = [None, None];
    let mut repeat = None;
    for (factor, claim) in claims {
        match factor {
            RaaFactor::Accumulator(round) => accumulators[round - 1] = Some(claim),
            RaaFactor::Permutation(round) => permutations[round - 1] = Some(claim),
            RaaFactor::Repeat => repeat = Some(claim),
        }
    }
    let [Some(accumulator_1), Some(accumulator_2)] = accumulators else {
        return Err(Error::InvalidCommitment);
    };
    let [Some(permutation_1), Some(permutation_2)] = permutations else {
        return Err(Error::InvalidCommitment);
    };
    Ok(RaaGeneratorLeaves {
        accumulators: [accumulator_1, accumulator_2],
        permutations: [permutation_1, permutation_2],
        repeat: repeat.ok_or(Error::InvalidCommitment)?,
    })
}

fn validate_transparent_leaves(
    layout: &RaaLayout,
    leaves: &RaaGeneratorLeaves<BinaryExtensionField>,
) -> Result<(), Error> {
    for claim in &leaves.accumulators {
        if claim.value != accumulator_mle(&claim.point.0, &claim.point.1)? {
            return Err(Error::InvalidCommitment);
        }
    }
    if leaves.repeat.value
        != repeat_mle(
            layout.rate_inv(),
            &leaves.repeat.point.0,
            &leaves.repeat.point.1,
        )?
    {
        return Err(Error::InvalidCommitment);
    }
    Ok(())
}

fn verify_transparent_leaves<Channel>(
    layout: &RaaLayout,
    leaves: &RaaGeneratorLeaves<Channel::Elem>,
    channel: &mut Channel,
) -> Result<(), Error>
where
    Channel: IPVerifierChannel<BinaryExtensionField>,
{
    for claim in &leaves.accumulators {
        let expected = accumulator_mle(&claim.point.0, &claim.point.1)?;
        channel
            .assert_zero(claim.value.clone() - expected)
            .map_err(|_| Error::InvalidCommitment)?;
    }
    let repeat_expected = repeat_mle(
        layout.rate_inv(),
        &leaves.repeat.point.0,
        &leaves.repeat.point.1,
    )?;
    channel
        .assert_zero(leaves.repeat.value.clone() - repeat_expected)
        .map_err(|_| Error::InvalidCommitment)
}

fn owned_scalar_field_buffer(
    values: Vec<BinaryExtensionField>,
) -> Result<FieldBuffer<PackedBinaryExtensionField>, Error> {
    if !values.len().is_power_of_two() {
        return Err(Error::InvalidNumberOfVariables);
    }
    let log_len = values
        .len()
        .checked_ilog2()
        .ok_or(Error::InvalidNumberOfVariables)? as usize;
    // The packed alias is width one: transfer the existing allocation without
    // repacking, changing coordinates, or dropping any live scalar lanes.
    Ok(FieldBuffer::new(log_len, values))
}

fn prove_permutation_grand_product<Channel>(
    layout: &RaaLayout,
    claims: &[RaaMatrixClaim<BinaryExtensionField>; RAA_PERMUTATION_COUNT],
    channel: &mut Channel,
) -> Result<RaaPermutationGrandProductReduction<BinaryExtensionField>, Error>
where
    Channel: IOPProverChannel<PackedBinaryExtensionField>,
{
    let profile_stages = std::env::var_os("RAAFOLD_PROFILE_STAGES").is_some();
    let total_start = Instant::now();
    for claim in claims {
        validate_square_leaf(layout, claim)?;
    }
    let n = layout.codeword_len();
    let log_n = n.ilog2() as usize;
    let padded_len = n
        .checked_mul(RAA_PADDED_PERMUTATION_COUNT)
        .ok_or(Error::EncodingError)?;
    let chain_len = padded_len
        .checked_mul(RAA_PRODUCT_SIDE_COUNT)
        .ok_or(Error::EncodingError)?;
    let state_len = chain_len
        .checked_mul(RAA_SHIFTED_VIEW_COUNT)
        .ok_or(Error::EncodingError)?;
    let pushforward_build_start = Instant::now();
    let index_powers = permutation_index_powers(n)?;

    let mut pushforward = vec![BinaryExtensionField::ZERO; padded_len];
    let mut left_weights = Vec::with_capacity(RAA_PERMUTATION_COUNT);
    for (round_index, claim) in claims.iter().enumerate() {
        let weights = eq_ind_partial_eval_scalars(&claim.point.0);
        let permutation = layout.permutation(round_index + 1)?;
        for (row, source) in permutation.sources().iter().enumerate() {
            pushforward[round_index * n + *source as usize] = weights[row];
        }
        let right_weights = eq_ind_partial_eval_scalars(&claim.point.1);
        if inner_product(
            &pushforward[round_index * n..(round_index + 1) * n],
            &right_weights,
        ) != claim.value
        {
            return Err(Error::InvalidCommitment);
        }
        left_weights.push(weights);
    }
    let pushforward_buffer = FieldBuffer::from_values(&pushforward);
    let pushforward_build_time = pushforward_build_start.elapsed();
    let pushforward_commit_start = Instant::now();
    let pushforward_oracle = channel.send_oracle(pushforward_buffer.to_ref());
    let pushforward_commit_time = pushforward_commit_start.elapsed();

    let alpha = channel.sample();
    let beta = channel.sample();
    let state_build_start = Instant::now();
    let mut states = vec![BinaryExtensionField::ZERO; state_len];
    let mut factors = vec![BinaryExtensionField::ZERO; chain_len];
    let mut endpoints = [BinaryExtensionField::ZERO; RAA_PERMUTATION_COUNT];
    {
        let (left_factors, right_factors) = factors.split_at_mut(padded_len);
        let (inclusive_states, exclusive_states) = states.split_at_mut(chain_len);
        let (left_inclusive_states, right_inclusive_states) =
            inclusive_states.split_at_mut(padded_len);
        let (left_exclusive_states, right_exclusive_states) =
            exclusive_states.split_at_mut(padded_len);
        for round_index in 0..RAA_PERMUTATION_COUNT {
            let permutation = layout.permutation(round_index + 1)?;
            let round_offset = round_index * n;
            let factor_at = |row: usize| {
                let source = permutation.sources()[row] as usize;
                let left_factor =
                    alpha + index_powers[source] + beta * left_weights[round_index][row];
                let right_factor =
                    alpha + index_powers[row] + beta * pushforward[round_offset + row];
                (left_factor, right_factor)
            };
            let left_round_factors = &mut left_factors[round_offset..round_offset + n];
            let right_round_factors = &mut right_factors[round_offset..round_offset + n];
            #[cfg(feature = "parallel")]
            left_round_factors
                .par_iter_mut()
                .zip(right_round_factors.par_iter_mut())
                .enumerate()
                .for_each(|(row, (left_target, right_target))| {
                    let (left_factor, right_factor) = factor_at(row);
                    *left_target = left_factor;
                    *right_target = right_factor;
                });
            #[cfg(not(feature = "parallel"))]
            for (row, (left_target, right_target)) in left_round_factors
                .iter_mut()
                .zip(right_round_factors.iter_mut())
                .enumerate()
            {
                let (left_factor, right_factor) = factor_at(row);
                *left_target = left_factor;
                *right_target = right_factor;
            }

            let left_endpoint = fill_direct_product_prefixes(
                left_round_factors,
                &mut left_inclusive_states[round_offset..round_offset + n],
                &mut left_exclusive_states[round_offset..round_offset + n],
            )?;
            let right_endpoint = fill_direct_product_prefixes(
                right_round_factors,
                &mut right_inclusive_states[round_offset..round_offset + n],
                &mut right_exclusive_states[round_offset..round_offset + n],
            )?;
            if left_endpoint != right_endpoint {
                return Err(Error::InvalidCommitment);
            }
            endpoints[round_index] = left_endpoint;
        }
    }
    let state_buffer = FieldBuffer::from_values(&states);
    let state_build_time = state_build_start.elapsed();
    let state_commit_start = Instant::now();
    let state_oracle = channel.send_oracle(state_buffer.to_ref());
    let state_commit_time = state_commit_start.elapsed();
    let hadamard_prepare_start = Instant::now();
    channel.send_many(&endpoints);
    let shift_challenge = channel.sample();
    let shift_batch = channel.sample();

    let hadamard_coordinates = channel.sample_many(log_n + 3);
    let hadamard_point = (
        hadamard_coordinates[..log_n].to_vec(),
        hadamard_coordinates[log_n..].to_vec(),
    );
    let (inclusive_states, exclusive_states) = states.split_at_mut(chain_len);
    let row_weights = eq_ind_partial_eval_scalars(&hadamard_point.0);
    let column_weights = eq_ind_partial_eval_scalars(&hadamard_point.1);
    let inclusive_eval = column_major_matrix_eval(
        inclusive_states,
        n,
        RAA_PADDED_PERMUTATION_COUNT * RAA_PRODUCT_SIDE_COUNT,
        &row_weights,
        &column_weights,
    );
    channel.send_one(inclusive_eval);
    let hadamard_prepare_time = hadamard_prepare_start.elapsed();
    let hadamard_start = Instant::now();
    let hadamard = prove_hadamard_matrix(
        inclusive_eval,
        &hadamard_point,
        n,
        RAA_PADDED_PERMUTATION_COUNT * RAA_PRODUCT_SIDE_COUNT,
        exclusive_states,
        &mut factors,
        channel,
    )?;
    let hadamard_time = hadamard_start.elapsed();

    let relation_start = Instant::now();
    let pushforward_eval_start = Instant::now();
    let mut pushforward_point = hadamard.right.point.0.clone();
    pushforward_point.extend_from_slice(&hadamard.right.point.1[..2]);
    let pushforward_eval = inner_product(
        &pushforward,
        &eq_ind_partial_eval_scalars(&pushforward_point),
    );
    channel.send_one(pushforward_eval);
    let fixed_h_claim = derive_fixed_h_claim(
        layout,
        claims,
        alpha,
        beta,
        &hadamard.right,
        pushforward_eval,
    )?;
    let pushforward_eval_time = pushforward_eval_start.elapsed();

    let pushforward_relation_start = Instant::now();
    let push_batch = channel.sample();
    let (pushforward_transparent, pushforward_claim) =
        build_pushforward_relation(claims, &pushforward_point, pushforward_eval, push_batch, n)?;
    if inner_product(&pushforward, &pushforward_transparent) != pushforward_claim {
        return Err(Error::InvalidCommitment);
    }
    let pushforward_relation_time = pushforward_relation_start.elapsed();

    let state_relation_start = Instant::now();
    let state_batch = channel.sample();
    let (state_transparent, state_claim) = build_state_relation(
        &hadamard_point,
        inclusive_eval,
        &hadamard.left,
        state_batch,
        shift_challenge,
        shift_batch,
        &endpoints,
        n,
    )?;
    let state_relation_time = state_relation_start.elapsed();
    let queue_start = Instant::now();
    channel.prove_oracle_relations([
        (
            pushforward_oracle,
            pushforward_buffer,
            owned_scalar_field_buffer(pushforward_transparent)?,
            pushforward_claim,
        ),
        (
            state_oracle,
            state_buffer,
            owned_scalar_field_buffer(state_transparent)?,
            state_claim,
        ),
    ]);
    let queue_time = queue_start.elapsed();
    let relation_time = relation_start.elapsed();
    if profile_stages {
        eprintln!(
            "[RAAFOLD_PROFILE] grand_product_n={n} pushforward_build_s={:.9} pushforward_oracle_commit_s={:.9} factor_prefix_state_s={:.9} state_oracle_commit_s={:.9} hadamard_prepare_s={:.9} hadamard_s={:.9} pushforward_eval_s={:.9} pushforward_relation_s={:.9} state_relation_s={:.9} relation_queue_s={:.9} relations_s={:.9} grand_product_total_s={:.9}",
            pushforward_build_time.as_secs_f64(),
            pushforward_commit_time.as_secs_f64(),
            state_build_time.as_secs_f64(),
            state_commit_time.as_secs_f64(),
            hadamard_prepare_time.as_secs_f64(),
            hadamard_time.as_secs_f64(),
            pushforward_eval_time.as_secs_f64(),
            pushforward_relation_time.as_secs_f64(),
            state_relation_time.as_secs_f64(),
            queue_time.as_secs_f64(),
            relation_time.as_secs_f64(),
            total_start.elapsed().as_secs_f64(),
        );
    }
    Ok(RaaPermutationGrandProductReduction { fixed_h_claim })
}

fn fill_direct_product_prefixes(
    factors: &[BinaryExtensionField],
    current_prefixes: &mut [BinaryExtensionField],
    previous_prefixes: &mut [BinaryExtensionField],
) -> Result<BinaryExtensionField, Error> {
    let len = factors.len();
    if len == 0 || current_prefixes.len() != len || previous_prefixes.len() != len {
        return Err(Error::InvalidNumberOfVariables);
    }

    #[cfg(feature = "parallel")]
    if len >= PARALLEL_SCAN_MIN_LEN && rayon::current_num_threads() > 1 {
        let threads = rayon::current_num_threads();
        let chunk_len = len.div_ceil(threads).max(PARALLEL_SCAN_MIN_LEN);
        let totals = current_prefixes
            .par_chunks_mut(chunk_len)
            .zip(factors.par_chunks(chunk_len))
            .map(|(prefix_chunk, factor_chunk)| {
                let mut running = BinaryExtensionField::ONE;
                for (prefix, factor) in prefix_chunk.iter_mut().zip(factor_chunk) {
                    running *= *factor;
                    *prefix = running;
                }
                running
            })
            .collect::<Vec<_>>();
        let mut running = BinaryExtensionField::ONE;
        let offsets = totals
            .iter()
            .map(|total| {
                let offset = running;
                running *= *total;
                offset
            })
            .collect::<Vec<_>>();
        current_prefixes
            .par_chunks_mut(chunk_len)
            .zip(offsets.into_par_iter())
            .for_each(|(prefix_chunk, offset)| {
                if offset != BinaryExtensionField::ONE {
                    prefix_chunk.iter_mut().for_each(|value| *value *= offset);
                }
            });
        previous_prefixes[0] = BinaryExtensionField::ONE;
        previous_prefixes[1..]
            .par_iter_mut()
            .zip(current_prefixes[..len - 1].par_iter())
            .for_each(|(previous, current)| *previous = *current);
        return Ok(running);
    }

    let mut running = BinaryExtensionField::ONE;
    for ((factor, current), previous) in factors.iter().zip(current_prefixes).zip(previous_prefixes)
    {
        *previous = running;
        running *= *factor;
        *current = running;
    }
    Ok(running)
}

fn verify_permutation_grand_product<Channel>(
    layout: &RaaLayout,
    claims: &[RaaMatrixClaim<BinaryExtensionField>; RAA_PERMUTATION_COUNT],
    channel: &mut Channel,
) -> Result<RaaPermutationGrandProductReduction<BinaryExtensionField>, Error>
where
    Channel: IOPVerifierChannel<BinaryExtensionField, Elem = BinaryExtensionField>,
{
    for claim in claims {
        validate_square_leaf_dimensions(layout, claim)?;
    }
    let n = layout.codeword_len();
    let log_n = n.ilog2() as usize;
    let padded_log = log_n + 2;
    let chain_log = padded_log + 1;
    let state_log = chain_log + 1;
    let pushforward_oracle = channel
        .recv_oracle(padded_log, true)
        .map_err(|_| Error::TranscriptError)?;
    let alpha = channel.sample();
    let beta = channel.sample();
    let state_oracle = channel
        .recv_oracle(state_log, true)
        .map_err(|_| Error::TranscriptError)?;
    let endpoints = channel
        .recv_array::<RAA_PERMUTATION_COUNT>()
        .map_err(|_| Error::TranscriptError)?;
    let shift_challenge = channel.sample();
    let shift_batch = channel.sample();
    let hadamard_coordinates = channel.sample_many(chain_log);
    let hadamard_point = (
        hadamard_coordinates[..log_n].to_vec(),
        hadamard_coordinates[log_n..].to_vec(),
    );
    let inclusive_eval = channel.recv_one().map_err(|_| Error::TranscriptError)?;
    let hadamard = verify_hadamard_matrix(
        inclusive_eval,
        &hadamard_point,
        n,
        RAA_PADDED_PERMUTATION_COUNT * RAA_PRODUCT_SIDE_COUNT,
        channel,
    )?;
    let mut pushforward_point = hadamard.right.point.0.clone();
    pushforward_point.extend_from_slice(&hadamard.right.point.1[..2]);
    let pushforward_eval = channel.recv_one().map_err(|_| Error::TranscriptError)?;
    let fixed_h_claim = derive_fixed_h_claim(
        layout,
        claims,
        alpha,
        beta,
        &hadamard.right,
        pushforward_eval,
    )?;

    let push_batch = channel.sample();
    let push_points = pushforward_relation_points(claims, &pushforward_point, n)?;
    let push_coefficients = batching_powers(push_batch, push_points.len());
    let pushforward_claim = pushforward_eval
        + claims
            .iter()
            .zip(push_coefficients.iter().skip(1))
            .map(|(claim, coefficient)| *coefficient * claim.value)
            .sum::<BinaryExtensionField>();

    let state_batch = channel.sample();
    let shift_claim = shift_consistency_claim(shift_challenge, shift_batch, &endpoints, n)?;
    let state_coefficients = batching_powers(state_batch, 3);
    let state_claim = inclusive_eval
        + state_coefficients[1] * hadamard.left.value
        + state_coefficients[2] * shift_claim;
    let mut inclusive_point = hadamard_point.0.clone();
    inclusive_point.extend_from_slice(&hadamard_point.1);
    inclusive_point.push(BinaryExtensionField::ZERO);
    let mut exclusive_point = hadamard.left.point.0.clone();
    exclusive_point.extend_from_slice(&hadamard.left.point.1);
    exclusive_point.push(BinaryExtensionField::ONE);
    channel
        .verify_oracle_relations([
            OracleLinearRelation {
                oracle: pushforward_oracle,
                transparent: Box::new(move |challenge| {
                    push_points
                        .iter()
                        .zip(&push_coefficients)
                        .map(|(point, coefficient)| *coefficient * eq_ind(point, challenge))
                        .sum()
                }),
                claim: pushforward_claim,
            },
            OracleLinearRelation {
                oracle: state_oracle,
                transparent: Box::new(move |challenge| {
                    eq_ind(&inclusive_point, challenge)
                        + state_coefficients[1] * eq_ind(&exclusive_point, challenge)
                        + state_coefficients[2]
                            * shift_consistency_transparent_mle(
                                challenge,
                                shift_challenge,
                                shift_batch,
                                n,
                            )
                }),
                claim: state_claim,
            },
        ])
        .map_err(|_| Error::InvalidCommitment)?;
    Ok(RaaPermutationGrandProductReduction { fixed_h_claim })
}

fn permutation_index_powers(length: usize) -> Result<Vec<BinaryExtensionField>, Error> {
    if length == 0 || !length.is_power_of_two() {
        return Err(Error::EncodingError);
    }
    Ok(geometric_power_values_from(
        BinaryExtensionField::MULTIPLICATIVE_GENERATOR,
        length,
        0,
    ))
}

fn derive_fixed_h_claim(
    layout: &RaaLayout,
    claims: &[RaaMatrixClaim<BinaryExtensionField>; RAA_PERMUTATION_COUNT],
    alpha: BinaryExtensionField,
    beta: BinaryExtensionField,
    factor_claim: &RaaMatrixClaim<BinaryExtensionField>,
    pushforward_eval: BinaryExtensionField,
) -> Result<RaaScaledMatrixClaim<BinaryExtensionField>, Error> {
    let n = layout.codeword_len();
    let log_n = n.ilog2() as usize;
    if factor_claim.point.0.len() != log_n || factor_claim.point.1.len() != 3 {
        return Err(Error::InvalidNumberOfVariables);
    }
    let index_point = &factor_claim.point.0;
    let round_point = &factor_claim.point.1[..2];
    let right_branch = factor_claim.point.1[2];
    let left_branch = BinaryExtensionField::ONE - right_branch;
    let round_weights = eq_ind_partial_eval_scalars(round_point);
    let real_round_weight = round_weights[..RAA_PERMUTATION_COUNT]
        .iter()
        .copied()
        .sum::<BinaryExtensionField>();
    let identity_tags = real_round_weight * multiplicative_index_mle(index_point);
    let left_eval = claims
        .iter()
        .zip(&round_weights)
        .map(|(claim, round_weight)| *round_weight * eq_ind(&claim.point.0, index_point))
        .sum::<BinaryExtensionField>();
    let known = left_branch * (alpha * real_round_weight + beta * left_eval)
        + right_branch * (alpha * real_round_weight + identity_tags + beta * pushforward_eval);
    let fixed_h_value = factor_claim.value - known;
    let row_log = layout.message_len().ilog2() as usize;
    let mut column_point = index_point[row_log..].to_vec();
    column_point.extend_from_slice(round_point);
    Ok(RaaScaledMatrixClaim {
        value: fixed_h_value,
        scale: left_branch,
        point: (index_point[..row_log].to_vec(), column_point),
    })
}

fn multiplicative_index_mle(point: &[BinaryExtensionField]) -> BinaryExtensionField {
    let mut result = BinaryExtensionField::ONE;
    let mut power = BinaryExtensionField::MULTIPLICATIVE_GENERATOR;
    for &coordinate in point {
        result *= BinaryExtensionField::ONE - coordinate + coordinate * power;
        power = power * power;
    }
    result
}

fn build_pushforward_relation(
    claims: &[RaaMatrixClaim<BinaryExtensionField>; RAA_PERMUTATION_COUNT],
    effective_point: &[BinaryExtensionField],
    pushforward_eval: BinaryExtensionField,
    batch: BinaryExtensionField,
    n: usize,
) -> Result<(Vec<BinaryExtensionField>, BinaryExtensionField), Error> {
    let points = pushforward_relation_points(claims, effective_point, n)?;
    let coefficients = batching_powers(batch, points.len());
    let mut transparent = vec![BinaryExtensionField::ZERO; n * RAA_PADDED_PERMUTATION_COUNT];
    for (point, coefficient) in points.iter().zip(&coefficients) {
        let weights = eq_ind_partial_eval_scalars(point);
        add_scaled_values(&mut transparent, &weights, *coefficient);
    }
    let claim = pushforward_eval
        + claims
            .iter()
            .zip(coefficients.iter().skip(1))
            .map(|(claim, coefficient)| *coefficient * claim.value)
            .sum::<BinaryExtensionField>();
    Ok((transparent, claim))
}

fn pushforward_relation_points(
    claims: &[RaaMatrixClaim<BinaryExtensionField>; RAA_PERMUTATION_COUNT],
    effective_point: &[BinaryExtensionField],
    n: usize,
) -> Result<Vec<Vec<BinaryExtensionField>>, Error> {
    let log_n = n.ilog2() as usize;
    if effective_point.len() != log_n + 2 {
        return Err(Error::InvalidNumberOfVariables);
    }
    let mut points = Vec::with_capacity(RAA_PERMUTATION_COUNT + 1);
    points.push(effective_point.to_vec());
    for (round, claim) in claims.iter().enumerate() {
        if claim.point.1.len() != log_n {
            return Err(Error::InvalidNumberOfVariables);
        }
        let mut point = claim.point.1.clone();
        point.extend(boolean_point(round, 2));
        points.push(point);
    }
    Ok(points)
}

#[allow(clippy::too_many_arguments)]
fn build_state_relation(
    hadamard_output_point: &(Vec<BinaryExtensionField>, Vec<BinaryExtensionField>),
    inclusive_eval: BinaryExtensionField,
    exclusive_claim: &RaaMatrixClaim<BinaryExtensionField>,
    state_batch: BinaryExtensionField,
    shift_challenge: BinaryExtensionField,
    shift_batch: BinaryExtensionField,
    endpoints: &[BinaryExtensionField; RAA_PERMUTATION_COUNT],
    n: usize,
) -> Result<(Vec<BinaryExtensionField>, BinaryExtensionField), Error> {
    let log_n = n.ilog2() as usize;
    if hadamard_output_point.0.len() != log_n
        || hadamard_output_point.1.len() != 3
        || exclusive_claim.point.0.len() != log_n
        || exclusive_claim.point.1.len() != 3
    {
        return Err(Error::InvalidNumberOfVariables);
    }
    let state_coefficients = batching_powers(state_batch, 3);
    let coefficients = batching_powers(shift_batch, RAA_PRODUCT_CHAIN_COUNT);
    let padded_len = n * RAA_PADDED_PERMUTATION_COUNT;
    let chain_len = padded_len * RAA_PRODUCT_SIDE_COUNT;
    let mut inclusive_point = hadamard_output_point.0.clone();
    inclusive_point.extend_from_slice(&hadamard_output_point.1);
    inclusive_point.push(BinaryExtensionField::ZERO);
    let mut exclusive_point = exclusive_claim.point.0.clone();
    exclusive_point.extend_from_slice(&exclusive_claim.point.1);
    exclusive_point.push(BinaryExtensionField::ONE);
    let mut transparent = eq_ind_partial_eval_scalars(&inclusive_point);
    add_scaled_values(
        &mut transparent,
        &eq_ind_partial_eval_scalars(&exclusive_point),
        state_coefficients[1],
    );
    let powers = geometric_power_values(shift_challenge, n);
    for product_side in 0..RAA_PRODUCT_SIDE_COUNT {
        for round in 0..RAA_PERMUTATION_COUNT {
            let coefficient =
                state_coefficients[2] * coefficients[round * RAA_PRODUCT_SIDE_COUNT + product_side];
            let offset = product_side * padded_len + round * n;
            let inclusive_scale = coefficient * shift_challenge;
            let exclusive_scale = coefficient;
            #[cfg(feature = "parallel")]
            {
                transparent[offset..offset + n]
                    .par_iter_mut()
                    .zip(powers.par_iter())
                    .for_each(|(target, power)| *target += inclusive_scale * *power);
                transparent[chain_len + offset..chain_len + offset + n]
                    .par_iter_mut()
                    .zip(powers.par_iter())
                    .for_each(|(target, power)| *target += exclusive_scale * *power);
            }
            #[cfg(not(feature = "parallel"))]
            {
                transparent[offset..offset + n]
                    .iter_mut()
                    .zip(powers.iter())
                    .for_each(|(target, power)| *target += inclusive_scale * *power);
                transparent[chain_len + offset..chain_len + offset + n]
                    .iter_mut()
                    .zip(powers.iter())
                    .for_each(|(target, power)| *target += exclusive_scale * *power);
            }
        }
    }
    let shift_claim = shift_consistency_claim(shift_challenge, shift_batch, endpoints, n)?;
    let claim = inclusive_eval
        + state_coefficients[1] * exclusive_claim.value
        + state_coefficients[2] * shift_claim;
    Ok((transparent, claim))
}

fn shift_consistency_claim(
    shift_challenge: BinaryExtensionField,
    shift_batch: BinaryExtensionField,
    endpoints: &[BinaryExtensionField; RAA_PERMUTATION_COUNT],
    n: usize,
) -> Result<BinaryExtensionField, Error> {
    let terminal_exponent = n.checked_add(1).ok_or(Error::EncodingError)?;
    let terminal_power = field_power(shift_challenge, terminal_exponent);
    let coefficients = batching_powers(shift_batch, RAA_PRODUCT_CHAIN_COUNT);
    Ok((0..RAA_PERMUTATION_COUNT)
        .flat_map(|round| {
            (0..RAA_PRODUCT_SIDE_COUNT).map(move |product_side| (round, product_side))
        })
        .map(|(round, product_side)| {
            coefficients[round * RAA_PRODUCT_SIDE_COUNT + product_side]
                * (shift_challenge + endpoints[round] * terminal_power)
        })
        .sum())
}

fn shift_consistency_transparent_mle(
    challenge: &[BinaryExtensionField],
    shift_challenge: BinaryExtensionField,
    shift_batch: BinaryExtensionField,
    n: usize,
) -> BinaryExtensionField {
    let log_n = n.ilog2() as usize;
    if challenge.len() != log_n + 4 {
        return BinaryExtensionField::ZERO;
    }
    let index_point = &challenge[..log_n];
    let round_point = &challenge[log_n..log_n + 2];
    let product_side_point = challenge[log_n + 2];
    let shifted_view_point = challenge[log_n + 3];
    let round_weights = eq_ind_partial_eval_scalars(round_point);
    let product_side_weights = [
        BinaryExtensionField::ONE - product_side_point,
        product_side_point,
    ];
    let coefficients = batching_powers(shift_batch, RAA_PRODUCT_CHAIN_COUNT);
    let chain_eval = (0..RAA_PERMUTATION_COUNT)
        .flat_map(|round| {
            (0..RAA_PRODUCT_SIDE_COUNT).map(move |product_side| (round, product_side))
        })
        .map(|(round, product_side)| {
            round_weights[round]
                * product_side_weights[product_side]
                * coefficients[round * RAA_PRODUCT_SIDE_COUNT + product_side]
        })
        .sum::<BinaryExtensionField>();
    let shifted_view_eval =
        (BinaryExtensionField::ONE - shifted_view_point) * shift_challenge + shifted_view_point;
    geometric_power_mle(index_point, shift_challenge) * chain_eval * shifted_view_eval
}

fn geometric_power_mle(
    point: &[BinaryExtensionField],
    shift_challenge: BinaryExtensionField,
) -> BinaryExtensionField {
    let mut result = shift_challenge;
    let mut bit_power = shift_challenge;
    for coordinate in point {
        result *= BinaryExtensionField::ONE - *coordinate + *coordinate * bit_power;
        bit_power *= bit_power;
    }
    result
}

fn geometric_power_values(
    shift_challenge: BinaryExtensionField,
    len: usize,
) -> Vec<BinaryExtensionField> {
    geometric_power_values_from(shift_challenge, len, 1)
}

fn geometric_power_values_from(
    shift_challenge: BinaryExtensionField,
    len: usize,
    first_exponent: usize,
) -> Vec<BinaryExtensionField> {
    let mut values = vec![BinaryExtensionField::ZERO; len];
    #[cfg(feature = "parallel")]
    if len >= PARALLEL_SCAN_MIN_LEN && rayon::current_num_threads() > 1 {
        let threads = rayon::current_num_threads();
        let chunk_len = len.div_ceil(threads).max(PARALLEL_SCAN_MIN_LEN);
        values
            .par_chunks_mut(chunk_len)
            .enumerate()
            .for_each(|(chunk_index, chunk)| {
                let start = chunk_index * chunk_len;
                let mut current = field_power(shift_challenge, start + first_exponent);
                for value in chunk {
                    *value = current;
                    current *= shift_challenge;
                }
            });
        return values;
    }
    let mut current = field_power(shift_challenge, first_exponent);
    for value in &mut values {
        *value = current;
        current *= shift_challenge;
    }
    values
}

fn field_power(mut base: BinaryExtensionField, mut exponent: usize) -> BinaryExtensionField {
    let mut result = BinaryExtensionField::ONE;
    while exponent != 0 {
        if exponent & 1 == 1 {
            result *= base;
        }
        exponent >>= 1;
        if exponent != 0 {
            base *= base;
        }
    }
    result
}

fn batching_powers(challenge: BinaryExtensionField, count: usize) -> Vec<BinaryExtensionField> {
    let mut current = BinaryExtensionField::ONE;
    (0..count)
        .map(|_| {
            let value = current;
            current *= challenge;
            value
        })
        .collect()
}

fn boolean_point(value: usize, bits: usize) -> Vec<BinaryExtensionField> {
    (0..bits)
        .map(|bit| {
            if (value >> bit) & 1 == 0 {
                BinaryExtensionField::ZERO
            } else {
                BinaryExtensionField::ONE
            }
        })
        .collect()
}

/// Evaluates the accumulation matrix MLE in `O(log n)` field operations.
pub fn accumulator_mle<E: FieldOps>(row_point: &[E], column_point: &[E]) -> Result<E, Error> {
    if row_point.len() != column_point.len() {
        return Err(Error::InvalidNumberOfVariables);
    }
    let all_one_prefix_mles = all_one_matrix_prefix_mles(row_point, column_point);
    Ok(accumulator_mle_from_prefixes(
        row_point,
        column_point,
        &all_one_prefix_mles,
        row_point.len(),
    ))
}

fn all_one_matrix_prefix_mles<E: FieldOps>(row_point: &[E], column_point: &[E]) -> Vec<E> {
    let mut values = Vec::with_capacity(row_point.len() + 1);
    let mut current = E::one();
    values.push(current.clone());
    for (row, column) in row_point.iter().zip(column_point) {
        let row_sum = (E::one() - row.clone()) + row.clone();
        let column_sum = (E::one() - column.clone()) + column.clone();
        current *= row_sum * column_sum;
        values.push(current.clone());
    }
    values
}

fn accumulator_mle_from_prefixes<E: FieldOps>(
    row_point: &[E],
    column_point: &[E],
    all_one_prefix_mles: &[E],
    level: usize,
) -> E {
    if level == 0 {
        return E::one();
    }
    let row_high = row_point[level - 1].clone();
    let column_high = column_point[level - 1].clone();
    let row_low_weight = E::one() - row_high.clone();
    let column_low_weight = E::one() - column_high.clone();
    let diagonal_weight =
        row_low_weight * column_low_weight.clone() + row_high.clone() * column_high;
    let lower_left_all_one_weight = row_high * column_low_weight;
    diagonal_weight
        * accumulator_mle_from_prefixes(row_point, column_point, all_one_prefix_mles, level - 1)
        + lower_left_all_one_weight * all_one_prefix_mles[level - 1].clone()
}

/// Evaluates the adjacent repetition matrix MLE in `O(log n)` operations.
pub fn repeat_mle<E: FieldOps>(
    rate_inv: usize,
    row_point: &[E],
    column_point: &[E],
) -> Result<E, Error> {
    if rate_inv == 0 || !rate_inv.is_power_of_two() {
        return Err(Error::EncodingError);
    }
    let log_rate = rate_inv.ilog2() as usize;
    if row_point.len() != column_point.len() + log_rate {
        return Err(Error::InvalidNumberOfVariables);
    }
    Ok(eq_ind(&row_point[log_rate..], column_point))
}

fn validate_generator_claim_dimensions<E>(
    layout: &RaaLayout,
    claim: &RaaMatrixClaim<E>,
) -> Result<(), Error> {
    validate_generator_claim_shape(layout, &claim.point.0, &claim.point.1)
}

fn validate_generator_claim_shape<E>(
    layout: &RaaLayout,
    row_point: &[E],
    column_point: &[E],
) -> Result<(), Error> {
    if row_point.len() != layout.codeword_len().ilog2() as usize
        || column_point.len() != layout.message_len().ilog2() as usize
    {
        return Err(Error::InvalidNumberOfVariables);
    }
    Ok(())
}

fn validate_square_leaf(
    layout: &RaaLayout,
    claim: &RaaMatrixClaim<BinaryExtensionField>,
) -> Result<(), Error> {
    validate_square_leaf_dimensions(layout, claim)
}

fn validate_square_leaf_dimensions<E>(
    layout: &RaaLayout,
    claim: &RaaMatrixClaim<E>,
) -> Result<(), Error> {
    let log_len = layout.codeword_len().ilog2() as usize;
    if claim.point.0.len() != log_len || claim.point.1.len() != log_len {
        return Err(Error::InvalidNumberOfVariables);
    }
    Ok(())
}

fn validate_permutation_oracle_specs(
    specs: &[OracleSpec],
    codeword_len: usize,
) -> Result<(), Error> {
    let pushforward = OracleSpec::new(codeword_len.ilog2() as usize + 2);
    let states = OracleSpec::new(codeword_len.ilog2() as usize + 4);
    if specs.len() < RAA_PERMUTATION_ORACLE_COUNT || specs[0] != pushforward || specs[1] != states {
        return Err(Error::InvalidCommitment);
    }
    Ok(())
}

fn validate_layout_shape(message_len: usize, rate_inv: usize) -> Result<usize, Error> {
    if message_len == 0
        || !message_len.is_power_of_two()
        || rate_inv <= 1
        || !rate_inv.is_power_of_two()
    {
        return Err(Error::EncodingError);
    }
    let codeword_len = message_len
        .checked_mul(rate_inv)
        .ok_or(Error::EncodingError)?;
    validate_permutation_length(codeword_len)?;
    Ok(codeword_len)
}

fn validate_permutation_length(length: usize) -> Result<(), Error> {
    if length == 0 || length > (u32::MAX as usize).saturating_add(1) {
        Err(Error::EncodingError)
    } else {
        Ok(())
    }
}

fn valid_distance(distance: f64) -> bool {
    distance.is_finite() && distance > 0.0 && distance < 1.0
}

fn sample_below<R: RngCore>(rng: &mut R, upper: usize) -> Result<usize, Error> {
    if upper == 0 || upper > u64::MAX as usize {
        return Err(Error::EncodingError);
    }
    let upper = upper as u64;
    let rejection_threshold = upper.wrapping_neg() % upper;
    loop {
        let sample = rng.next_u64();
        if sample >= rejection_threshold {
            return Ok((sample % upper) as usize);
        }
    }
}

fn generator_digest(
    message_len: usize,
    rate_inv: usize,
    permutations: &[RaaPermutation; RAA_PERMUTATION_COUNT],
) -> Result<[u8; 32], Error> {
    let message_len = u64::try_from(message_len).map_err(|_| Error::EncodingError)?;
    let rate_inv = u64::try_from(rate_inv).map_err(|_| Error::EncodingError)?;
    let mut hasher = Blake2s256::new();
    hasher.update(RAA_DOMAIN_LABEL);
    hasher.update(message_len.to_le_bytes());
    hasher.update(rate_inv.to_le_bytes());
    for (round, permutation) in permutations.iter().enumerate() {
        hasher.update((round as u64 + 1).to_le_bytes());
        for source in permutation.sources() {
            hasher.update(source.to_le_bytes());
        }
    }
    Ok(hasher.finalize().into())
}

fn raa_sequence_generator_digest(
    codes: &[RaaCode],
    top_switch_ratio: usize,
    recursive_switch_ratio: usize,
) -> Result<[u8; 32], Error> {
    let mut hasher = Blake2s256::new();
    hasher.update(RAA_DIRECT_INTERLEAVED_DOMAIN_LABEL);
    hasher.update(
        u64::try_from(codes.len())
            .map_err(|_| Error::EncodingError)?
            .to_le_bytes(),
    );
    hasher.update(
        u64::try_from(top_switch_ratio)
            .map_err(|_| Error::EncodingError)?
            .to_le_bytes(),
    );
    hasher.update(
        u64::try_from(recursive_switch_ratio)
            .map_err(|_| Error::EncodingError)?
            .to_le_bytes(),
    );
    for (level, code) in codes.iter().enumerate() {
        hasher.update(
            u64::try_from(level)
                .map_err(|_| Error::EncodingError)?
                .to_le_bytes(),
        );
        hasher.update(code.generator_digest());
        let distance = code.distance_profile();
        hasher.update(distance.code_relative_distance().to_bits().to_le_bytes());
        hasher.update(distance.query_relative_distance().to_bits().to_le_bytes());
        match distance.provenance() {
            RaaDistanceProvenance::Assumed => hasher.update([0]),
            RaaDistanceProvenance::Certified {
                log2_failure_probability,
            } => {
                hasher.update([1]);
                hasher.update(log2_failure_probability.to_bits().to_le_bytes());
            }
        }
    }
    Ok(hasher.finalize().into())
}

fn fill_repeated_permutation<S: RaaAdditiveSymbol>(
    message: &[S],
    rate_inv: usize,
    permutation: &[u32],
    output: &mut [S],
    allow_parallel: bool,
) {
    debug_assert_eq!(output.len(), permutation.len());
    #[cfg(not(feature = "parallel"))]
    let _ = allow_parallel;
    #[cfg(feature = "parallel")]
    if allow_parallel {
        output
            .par_iter_mut()
            .zip(permutation.par_iter())
            .for_each(|(target, source)| *target = message[*source as usize / rate_inv]);
        return;
    }
    output
        .iter_mut()
        .zip(permutation.iter())
        .for_each(|(target, source)| *target = message[*source as usize / rate_inv]);
}

fn transpose_interleaved_message_into_row_major<S: RaaAdditiveSymbol>(
    input: &[S],
    rows: usize,
    columns: usize,
    output: &mut [S],
) {
    const TILE: usize = 32;
    debug_assert!(columns > 0);
    debug_assert_eq!(input.len(), rows * columns);
    debug_assert_eq!(output.len(), input.len());
    if rows == 0 {
        return;
    }
    let chunk_rows = rows.min(TILE);
    // Both memory-facing copies are contiguous. The small temporary avoids
    // repeatedly writing one scalar at a large row stride (and conflicting
    // L1 cache sets) while loading a contiguous input column segment.
    let transpose_rows = |block_index: usize, target: &mut [S]| {
        let first_row = block_index * chunk_rows;
        let block_rows = target.len() / columns;
        let mut tile = [S::zero(); TILE * TILE];
        for first_column in (0..columns).step_by(TILE) {
            let block_columns = (columns - first_column).min(TILE);
            for column in 0..block_columns {
                let start = (first_column + column) * rows + first_row;
                for (row, &value) in input[start..start + block_rows].iter().enumerate() {
                    tile[row * TILE + column] = value;
                }
            }
            for (row, output_row) in target.chunks_mut(columns).enumerate() {
                output_row[first_column..first_column + block_columns]
                    .copy_from_slice(&tile[row * TILE..row * TILE + block_columns]);
            }
        }
    };
    #[cfg(feature = "parallel")]
    output
        .par_chunks_mut(chunk_rows * columns)
        .enumerate()
        .for_each(|(index, target)| transpose_rows(index, target));
    #[cfg(not(feature = "parallel"))]
    output
        .chunks_mut(chunk_rows * columns)
        .enumerate()
        .for_each(|(index, target)| transpose_rows(index, target));
}

fn finish_fused_staged_interleaved_encoding<S: RaaAdditiveSymbol>(
    message_entries: usize,
    columns: usize,
    rate_inv: usize,
    first_permutation: &[u32],
    second_permutation: &[u32],
    output: &mut [S],
    scratch: &mut [S],
) {
    debug_assert!(columns > 0 && rate_inv > 0);
    debug_assert_eq!(first_permutation.len(), second_permutation.len());
    debug_assert_eq!(output.len(), first_permutation.len() * columns);
    debug_assert_eq!(scratch.len(), output.len());
    debug_assert!(message_entries <= output.len());
    let rows = first_permutation.len();
    if rows == 0 {
        return;
    }
    let chunk_rows = interleaved_fused_chunk_rows(rows);
    let chunk_entries = chunk_rows * columns;

    // First scan: consume the row-major staging prefix before any output
    // overwrite. Scratch contains only each chunk's inclusive local prefix.
    // The chunk totals become the omitted global offsets after the barrier.
    let mut first_offsets = {
        let staged = &output[..message_entries];
        let fill_chunk = |chunk_index: usize, target: &mut [S]| {
            let first_row = chunk_index * chunk_rows;
            let mut running = vec![S::zero(); columns];
            for (row_index, target_row) in target.chunks_mut(columns).enumerate() {
                let source_row = first_permutation[first_row + row_index] as usize / rate_inv;
                let source = &staged[source_row * columns..(source_row + 1) * columns];
                for ((target, sum), &value) in target_row.iter_mut().zip(&mut running).zip(source) {
                    *sum = sum.add(value);
                    *target = *sum;
                }
            }
            running
        };
        #[cfg(feature = "parallel")]
        let totals = scratch
            .par_chunks_mut(chunk_entries)
            .enumerate()
            .map(|(index, target)| fill_chunk(index, target))
            .collect::<Vec<_>>();
        #[cfg(not(feature = "parallel"))]
        let totals = scratch
            .chunks_mut(chunk_entries)
            .enumerate()
            .map(|(index, target)| fill_chunk(index, target))
            .collect::<Vec<_>>();
        totals
    };
    interleaved_chunk_totals_to_offsets(&mut first_offsets, columns);

    // P2 reads a source chunk's local prefix and applies that chunk's offset
    // on demand. Fuse the second local prefix into the output write, avoiding
    // both a complete scratch offset pass and a separate output scan.
    let fill_chunk = |chunk_index: usize, target: &mut [S]| {
        let first_row = chunk_index * chunk_rows;
        let mut running = vec![S::zero(); columns];
        for (row_index, target_row) in target.chunks_mut(columns).enumerate() {
            let source_row = second_permutation[first_row + row_index] as usize;
            let source = &scratch[source_row * columns..(source_row + 1) * columns];
            let offset = &first_offsets[source_row / chunk_rows];
            for (((target, sum), &value), &offset) in target_row
                .iter_mut()
                .zip(&mut running)
                .zip(source)
                .zip(offset)
            {
                *sum = sum.add(value).add(offset);
                *target = *sum;
            }
        }
        running
    };
    #[cfg(feature = "parallel")]
    let mut second_offsets = output
        .par_chunks_mut(chunk_entries)
        .enumerate()
        .map(|(index, target)| fill_chunk(index, target))
        .collect::<Vec<_>>();
    #[cfg(not(feature = "parallel"))]
    let mut second_offsets = output
        .chunks_mut(chunk_entries)
        .enumerate()
        .map(|(index, target)| fill_chunk(index, target))
        .collect::<Vec<_>>();
    interleaved_chunk_totals_to_offsets(&mut second_offsets, columns);

    // Only the final global offset pass remains. Chunk zero's offset is zero.
    let add_offset = |target: &mut [S], offset: &[S]| {
        for row in target.chunks_mut(columns) {
            for (value, &offset) in row.iter_mut().zip(offset) {
                *value = value.add(offset);
            }
        }
    };
    #[cfg(feature = "parallel")]
    output
        .par_chunks_mut(chunk_entries)
        .zip(second_offsets.par_iter())
        .skip(1)
        .for_each(|(target, offset)| add_offset(target, offset));
    #[cfg(not(feature = "parallel"))]
    output
        .chunks_mut(chunk_entries)
        .zip(&second_offsets)
        .skip(1)
        .for_each(|(target, offset)| add_offset(target, offset));
}

fn interleaved_fused_chunk_rows(rows: usize) -> usize {
    #[cfg(feature = "parallel")]
    if rows >= 64 && rayon::current_num_threads() > 1 {
        return rows.div_ceil(64);
    }
    rows
}

fn interleaved_chunk_totals_to_offsets<S: RaaAdditiveSymbol>(
    totals: &mut [Vec<S>],
    columns: usize,
) {
    let mut running = vec![S::zero(); columns];
    for total in totals {
        for (value, running) in total.iter_mut().zip(&mut running) {
            let chunk_total = *value;
            *value = *running;
            *running = running.add(chunk_total);
        }
    }
}

#[cfg(test)]
fn fill_row_major_interleaved_repeated_permutation<S: RaaAdditiveSymbol>(
    messages: &[S],
    columns: usize,
    rate_inv: usize,
    permutation: &[u32],
    output: &mut [S],
) {
    debug_assert_eq!(output.len(), permutation.len() * columns);
    let fill_row = |target: &mut [S], source: &u32| {
        let start = (*source as usize / rate_inv) * columns;
        target.copy_from_slice(&messages[start..start + columns]);
    };
    #[cfg(feature = "parallel")]
    output
        .par_chunks_mut(columns)
        .zip(permutation.par_iter())
        .for_each(|(target, source)| fill_row(target, source));
    #[cfg(not(feature = "parallel"))]
    output
        .chunks_mut(columns)
        .zip(permutation.iter())
        .for_each(|(target, source)| fill_row(target, source));
}

fn fill_interleaved_repeated_permutation<S: RaaAdditiveSymbol>(
    messages: &[S],
    message_rows: usize,
    columns: usize,
    rate_inv: usize,
    permutation: &[u32],
    output: &mut [S],
) {
    debug_assert_eq!(messages.len(), message_rows * columns);
    debug_assert_eq!(output.len(), permutation.len() * columns);
    let fill_row = |target: &mut [S], source: &u32| {
        let source_row = *source as usize / rate_inv;
        for (column, value) in target.iter_mut().enumerate() {
            *value = messages[column * message_rows + source_row];
        }
    };
    #[cfg(feature = "parallel")]
    output
        .par_chunks_mut(columns)
        .zip(permutation.par_iter())
        .for_each(|(target, source)| fill_row(target, source));
    #[cfg(not(feature = "parallel"))]
    output
        .chunks_mut(columns)
        .zip(permutation.iter())
        .for_each(|(target, source)| fill_row(target, source));
}

fn fill_generated_interleaved_repeated_permutation<S, F>(
    message_rows: usize,
    columns: usize,
    rate_inv: usize,
    permutation: &[u32],
    output: &mut [S],
    value_at: F,
) where
    S: RaaAdditiveSymbol,
    F: Fn(usize, usize) -> S + Sync,
{
    debug_assert_eq!(output.len(), permutation.len() * columns);
    let fill_row = |target: &mut [S], source: &u32| {
        let source_row = *source as usize / rate_inv;
        debug_assert!(source_row < message_rows);
        target
            .iter_mut()
            .enumerate()
            .for_each(|(column, value)| *value = value_at(source_row, column));
    };
    #[cfg(feature = "parallel")]
    output
        .par_chunks_mut(columns)
        .zip(permutation.par_iter())
        .for_each(|(target, source)| fill_row(target, source));
    #[cfg(not(feature = "parallel"))]
    output
        .chunks_mut(columns)
        .zip(permutation.iter())
        .for_each(|(target, source)| fill_row(target, source));
}

fn apply_permutation<S: RaaAdditiveSymbol>(
    input: &[S],
    permutation: &[u32],
    output: &mut [S],
    allow_parallel: bool,
) {
    debug_assert_eq!(input.len(), permutation.len());
    debug_assert_eq!(output.len(), permutation.len());
    #[cfg(not(feature = "parallel"))]
    let _ = allow_parallel;
    #[cfg(feature = "parallel")]
    if allow_parallel {
        output
            .par_iter_mut()
            .zip(permutation.par_iter())
            .for_each(|(target, source)| *target = input[*source as usize]);
        return;
    }
    output
        .iter_mut()
        .zip(permutation.iter())
        .for_each(|(target, source)| *target = input[*source as usize]);
}

fn apply_interleaved_permutation<S: RaaAdditiveSymbol>(
    input: &[S],
    columns: usize,
    permutation: &[u32],
    output: &mut [S],
) {
    debug_assert_eq!(input.len(), permutation.len() * columns);
    debug_assert_eq!(output.len(), permutation.len() * columns);
    let apply_row = |target: &mut [S], source: &u32| {
        let start = *source as usize * columns;
        target.copy_from_slice(&input[start..start + columns]);
    };
    #[cfg(feature = "parallel")]
    output
        .par_chunks_mut(columns)
        .zip(permutation.par_iter())
        .for_each(|(target, source)| apply_row(target, source));
    #[cfg(not(feature = "parallel"))]
    output
        .chunks_mut(columns)
        .zip(permutation.iter())
        .for_each(|(target, source)| apply_row(target, source));
}

fn prefix_accumulate<S: RaaAdditiveSymbol>(values: &mut [S], allow_parallel: bool) {
    if values.is_empty() {
        return;
    }
    #[cfg(not(feature = "parallel"))]
    let _ = allow_parallel;
    #[cfg(feature = "parallel")]
    if allow_parallel && values.len() >= PARALLEL_SCAN_MIN_LEN {
        let threads = rayon::current_num_threads().max(1);
        let chunk_len = values.len().div_ceil(threads).max(PARALLEL_SCAN_MIN_LEN);
        let totals = values
            .par_chunks_mut(chunk_len)
            .map(|chunk| {
                prefix_accumulate_serial(chunk);
                *chunk.last().expect("RAA scan chunks are non-empty")
            })
            .collect::<Vec<_>>();
        let mut running = S::zero();
        let offsets = totals
            .iter()
            .map(|total| {
                let offset = running;
                running = running.add(*total);
                offset
            })
            .collect::<Vec<_>>();
        values
            .par_chunks_mut(chunk_len)
            .zip(offsets.into_par_iter())
            .for_each(|(chunk, offset)| {
                if offset != S::zero() {
                    chunk
                        .iter_mut()
                        .for_each(|value| *value = offset.add(*value));
                }
            });
        return;
    }
    prefix_accumulate_serial(values);
}

fn prefix_accumulate_serial<S: RaaAdditiveSymbol>(values: &mut [S]) {
    let mut running = S::zero();
    for value in values {
        running = running.add(*value);
        *value = running;
    }
}

fn prefix_accumulate_interleaved_rows<S: RaaAdditiveSymbol>(values: &mut [S], columns: usize) {
    const ROW_GROUPS: usize = 64;

    debug_assert!(columns > 0 && values.len() % columns == 0);
    let rows = values.len() / columns;
    if rows == 0 {
        return;
    }
    #[cfg(feature = "parallel")]
    if rows >= ROW_GROUPS && rayon::current_num_threads() > 1 {
        let chunk_rows = rows.div_ceil(ROW_GROUPS);
        let chunk_entries = chunk_rows * columns;
        let totals = values
            .par_chunks_mut(chunk_entries)
            .map(|chunk| {
                prefix_accumulate_interleaved_rows_serial(chunk, columns);
                chunk[chunk.len() - columns..].to_vec()
            })
            .collect::<Vec<_>>();

        let mut running = vec![S::zero(); columns];
        let offsets = totals
            .iter()
            .map(|total| {
                let offset = running.clone();
                running
                    .iter_mut()
                    .zip(total)
                    .for_each(|(sum, value)| *sum = sum.add(*value));
                offset
            })
            .collect::<Vec<_>>();
        values
            .par_chunks_mut(chunk_entries)
            .zip(offsets.par_iter())
            .for_each(|(chunk, offset)| {
                for row in chunk.chunks_mut(columns) {
                    row.iter_mut()
                        .zip(offset)
                        .for_each(|(value, offset)| *value = offset.add(*value));
                }
            });
        return;
    }
    prefix_accumulate_interleaved_rows_serial(values, columns);
}

fn prefix_accumulate_interleaved_rows_serial<S: RaaAdditiveSymbol>(
    values: &mut [S],
    columns: usize,
) {
    let mut running = vec![S::zero(); columns];
    for row in values.chunks_mut(columns) {
        running.iter_mut().zip(row).for_each(|(sum, value)| {
            *sum = sum.add(*value);
            *value = *sum;
        });
    }
}

fn suffix_accumulate(values: &mut [BinaryExtensionField]) {
    #[cfg(feature = "parallel")]
    if values.len() >= PARALLEL_SCAN_MIN_LEN && rayon::current_num_threads() > 1 {
        let chunk_len = values
            .len()
            .div_ceil(rayon::current_num_threads())
            .max(PARALLEL_SCAN_MIN_LEN);
        let mut offsets = values
            .par_chunks_mut(chunk_len)
            .map(|chunk| {
                suffix_accumulate_serial(chunk);
                chunk[0]
            })
            .collect::<Vec<_>>();
        let mut running = BinaryExtensionField::ZERO;
        for offset in offsets.iter_mut().rev() {
            let total = *offset;
            *offset = running;
            running += total;
        }
        values
            .par_chunks_mut(chunk_len)
            .zip(offsets.par_iter())
            .for_each(|(chunk, &offset)| {
                if offset != BinaryExtensionField::ZERO {
                    chunk.iter_mut().for_each(|value| *value += offset);
                }
            });
        return;
    }
    suffix_accumulate_serial(values);
}

fn suffix_accumulate_serial(values: &mut [BinaryExtensionField]) {
    let mut running = BinaryExtensionField::ZERO;
    for value in values.iter_mut().rev() {
        running += *value;
        *value = running;
    }
}

fn inner_product(
    left: &[BinaryExtensionField],
    right: &[BinaryExtensionField],
) -> BinaryExtensionField {
    assert_eq!(left.len(), right.len());
    #[cfg(feature = "parallel")]
    if left.len() >= PARALLEL_FIELD_THRESHOLD && rayon::current_num_threads() > 1 {
        return left
            .par_chunks(PARALLEL_FIELD_THRESHOLD)
            .zip(right.par_chunks(PARALLEL_FIELD_THRESHOLD))
            .map(|(left, right)| BinaryExtensionField::dot_product(left, right))
            .reduce(|| BinaryExtensionField::ZERO, |left, right| left + right);
    }
    BinaryExtensionField::dot_product(left, right)
}

fn add_scaled_values(
    target: &mut [BinaryExtensionField],
    source: &[BinaryExtensionField],
    coefficient: BinaryExtensionField,
) {
    assert_eq!(target.len(), source.len());
    #[cfg(feature = "parallel")]
    if target.len() >= PARALLEL_FIELD_THRESHOLD && rayon::current_num_threads() > 1 {
        target
            .par_chunks_mut(PARALLEL_FIELD_THRESHOLD)
            .zip(source.par_chunks(PARALLEL_FIELD_THRESHOLD))
            .for_each(|(target, source)| {
                BinaryExtensionField::add_scaled_slice(target, source, coefficient)
            });
        return;
    }
    BinaryExtensionField::add_scaled_slice(target, source, coefficient);
}

#[cfg(test)]
mod projected_litebullet_tests {
    use super::*;
    use ark_std::rand::{SeedableRng, rngs::StdRng};

    fn values(len: usize, seed: u64) -> Vec<BinaryExtensionField> {
        let mut rng = StdRng::seed_from_u64(seed);
        (0..len)
            .map(|i| match i % 13 {
                0 => BinaryExtensionField::ZERO,
                1 => BinaryExtensionField::ONE,
                _ => BinaryExtensionField::from(
                    u128::from(rng.next_u64()) | (u128::from(rng.next_u64()) << 64),
                ),
            })
            .collect()
    }

    fn scalar_dot(
        left: &[BinaryExtensionField],
        right: &[BinaryExtensionField],
    ) -> BinaryExtensionField {
        assert_eq!(left.len(), right.len());
        left.iter()
            .zip(right)
            .fold(BinaryExtensionField::ZERO, |sum, (&left, &right)| {
                sum + left * right
            })
    }

    #[test]
    fn projected_round_and_bulk_pair_match_scalar_at_chunk_boundaries() {
        for len in [0, 1, 2, 3, 4, 7, 4095, 4096, 4097, 8193] {
            let left_low = values(len, 1);
            let left_high = values(len, 2);
            let right_low = values(len, 3);
            let right_high = values(len, 4);
            assert_eq!(
                projected_litebullet_round(&left_low, &left_high, &right_low, &right_high),
                (
                    scalar_dot(&left_high, &right_high),
                    scalar_dot(&left_low, &right_high) + scalar_dot(&left_high, &right_low)
                ),
                "round length={len}",
            );
            for challenge in [
                BinaryExtensionField::ZERO,
                BinaryExtensionField::ONE,
                BinaryExtensionField::from(0x1234_5678_9abc_def0u128),
            ] {
                let scalar_fold = |low: &[BinaryExtensionField], high: &[BinaryExtensionField]| {
                    low.iter()
                        .zip(high)
                        .map(|(&low, &high)| low + (low + high) * challenge)
                        .collect::<Vec<_>>()
                };
                let mut actual_left = left_low.clone();
                let mut actual_right = right_low.clone();
                fold_projected_litebullet_pair(
                    &mut actual_left,
                    &left_high,
                    &mut actual_right,
                    &right_high,
                    challenge,
                );
                assert_eq!(
                    actual_left,
                    scalar_fold(&left_low, &left_high),
                    "left length={len}"
                );
                assert_eq!(
                    actual_right,
                    scalar_fold(&right_low, &right_high),
                    "right length={len}"
                );
            }
        }
    }

    struct RecordingChannel {
        hasher: Blake2s256,
        responses: Vec<BinaryExtensionField>,
        challenges: Vec<BinaryExtensionField>,
        boolean_prefix: bool,
    }

    impl RecordingChannel {
        fn new(boolean_prefix: bool) -> Self {
            let mut hasher = Blake2s256::new();
            hasher.update(b"raafold/projected-litebullet-differential-test");
            Self {
                hasher,
                responses: Vec::new(),
                challenges: Vec::new(),
                boolean_prefix,
            }
        }

        fn response_bytes(&self) -> Vec<u8> {
            self.responses
                .iter()
                .flat_map(|&value| u128::from(value).to_le_bytes())
                .collect()
        }
    }

    impl IPProverChannel<BinaryExtensionField> for RecordingChannel {
        fn send_one(&mut self, value: BinaryExtensionField) {
            self.hasher.update([0]);
            self.hasher.update(u128::from(value).to_le_bytes());
            self.responses.push(value);
        }

        fn send_many(&mut self, values: &[BinaryExtensionField]) {
            for &value in values {
                self.send_one(value);
            }
        }

        fn observe_one(&mut self, value: BinaryExtensionField) {
            self.hasher.update([1]);
            self.hasher.update(u128::from(value).to_le_bytes());
        }

        fn observe_many(&mut self, values: &[BinaryExtensionField]) {
            for &value in values {
                self.observe_one(value);
            }
        }

        fn sample(&mut self) -> BinaryExtensionField {
            let digest = self.hasher.clone().finalize();
            let challenge = if self.boolean_prefix && self.challenges.len() < 2 {
                if self.challenges.is_empty() {
                    BinaryExtensionField::ZERO
                } else {
                    BinaryExtensionField::ONE
                }
            } else {
                BinaryExtensionField::from(u128::from_le_bytes(digest[..16].try_into().unwrap()))
            };
            self.hasher.update([2]);
            self.hasher.update(u128::from(challenge).to_le_bytes());
            self.challenges.push(challenge);
            challenge
        }
    }

    fn scalar_projected_reference(
        mut left: Vec<BinaryExtensionField>,
        mut right: Vec<BinaryExtensionField>,
        channel: &mut RecordingChannel,
    ) -> (
        RaaMatrixClaim<BinaryExtensionField>,
        RaaMatrixClaim<BinaryExtensionField>,
    ) {
        let mut rhs = scalar_dot(&left, &right);
        let mut point = Vec::new();
        while left.len() > 1 {
            let half = left.len() / 2;
            let high = scalar_dot(&left[half..], &right[half..]);
            let cross = scalar_dot(&left[..half], &right[half..])
                + scalar_dot(&left[half..], &right[..half]);
            channel.send_many(&[high, cross]);
            let challenge = channel.sample();
            let squared = challenge * challenge;
            rhs =
                rhs * (BinaryExtensionField::ONE + squared) + high + cross * (challenge + squared);
            for i in 0..half {
                left[i] = left[i] + (left[i] + left[i + half]) * challenge;
                right[i] = right[i] + (right[i] + right[i + half]) * challenge;
            }
            left.truncate(half);
            right.truncate(half);
            point.push(challenge);
        }
        assert_eq!(left[0] * right[0], rhs);
        channel.send_many(&[left[0], right[0]]);
        point.reverse();
        (
            RaaMatrixClaim {
                value: left[0],
                point: (Vec::new(), point.clone()),
            },
            RaaMatrixClaim {
                value: right[0],
                point: (point, Vec::new()),
            },
        )
    }

    #[test]
    fn projected_bulk_litebullet_preserves_scalar_transcript_bytes_and_claims() {
        let check = || {
            for len in [1, 2, 4, 8, 64, 8192, 16384] {
                let left = values(len, 0x7261_612d_6c65_6674);
                let right = values(len, 0x7261_612d_7269_6768);
                for boolean_prefix in [false, true] {
                    let mut reference = RecordingChannel::new(boolean_prefix);
                    let expected =
                        scalar_projected_reference(left.clone(), right.clone(), &mut reference);
                    let mut channel = RecordingChannel::new(boolean_prefix);
                    let actual = prove_projected_litebullet(
                        scalar_dot(&left, &right),
                        left.clone(),
                        right.clone(),
                        &mut channel,
                    )
                    .unwrap();
                    assert_eq!(actual, expected, "claims length={len}");
                    assert_eq!(
                        channel.response_bytes(),
                        reference.response_bytes(),
                        "response bytes length={len}"
                    );
                    assert_eq!(
                        channel.challenges, reference.challenges,
                        "challenges length={len}"
                    );
                    assert_eq!(
                        channel.sample(),
                        reference.sample(),
                        "continuation length={len}"
                    );
                }
            }
        };
        #[cfg(feature = "parallel")]
        for workers in [1, 4] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap()
                .install(check);
        }
        #[cfg(not(feature = "parallel"))]
        check();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::rand::{SeedableRng, rngs::StdRng};

    #[test]
    fn owned_scalar_buffers_preserve_storage_and_reject_invalid_lengths() {
        for len in [0, 3, 5, 6, 9, 1023] {
            assert!(matches!(
                owned_scalar_field_buffer(vec![BinaryExtensionField::ZERO; len]),
                Err(Error::InvalidNumberOfVariables),
            ));
        }
        for len in [1, 2, 4, 16, 1024] {
            let mut values = Vec::with_capacity(len + 7);
            values.extend((0..len).map(|index| {
                BinaryExtensionField::from((1u128 << 127) ^ ((index as u128) << 65) ^ index as u128)
            }));
            let expected = values.clone();
            let pointer = values.as_ptr();
            let capacity = values.capacity();
            let buffer = owned_scalar_field_buffer(values).unwrap();
            assert_eq!(buffer.log_len(), len.ilog2() as usize);
            assert_eq!(buffer.iter_scalars().collect::<Vec<_>>(), expected);
            let recovered = buffer.take_data();
            assert_eq!(recovered.as_ptr(), pointer);
            assert_eq!(recovered.capacity(), capacity);
            assert_eq!(recovered, expected);
        }
    }

    fn independent_column_encoding<S: RaaAdditiveSymbol>(
        layout: &RaaLayout,
        messages: &[S],
        columns: usize,
    ) -> Vec<S> {
        let mut result = vec![S::zero(); layout.codeword_len() * columns];
        for column in 0..columns {
            let message =
                &messages[column * layout.message_len()..(column + 1) * layout.message_len()];
            let mut encoded = message
                .iter()
                .flat_map(|&value| core::iter::repeat_n(value, layout.rate_inv()))
                .collect::<Vec<_>>();
            // Independent original Repeat -> P1 -> Accumulate -> P2 ->
            // Accumulate, one column at a time, without interleaved helpers.
            for round in 1..=2 {
                let mut next = layout
                    .permutation(round)
                    .unwrap()
                    .sources()
                    .iter()
                    .map(|&source| encoded[source as usize])
                    .collect::<Vec<_>>();
                let mut running = S::zero();
                for value in &mut next {
                    running = running.add(*value);
                    *value = running;
                }
                encoded = next;
            }
            for (row, value) in encoded.into_iter().enumerate() {
                result[row * columns + column] = value;
            }
        }
        result
    }

    fn check_interleaved_pingpong<S: RaaAdditiveSymbol + From<u128>>() {
        let check = || {
            let mut rng = StdRng::seed_from_u64(0x7069_6e67_706f_6e67);
            // Encoded row counts 4, 32, 64, 128 and 1024 cover both sides of
            // the interleaved encoder's 64-row parallel-scan boundary.
            for message_rows in [1, 8, 16, 32, 256] {
                let layout = RaaLayout::sample(
                    message_rows,
                    4,
                    RaaDistanceProfile::blaze_assumed(4).unwrap(),
                    &mut rng,
                )
                .unwrap();
                for columns in [1, 2, 8, 128] {
                    let messages = (0..message_rows * columns)
                        .map(|index| {
                            let random = rng.next_u64() as u128 | ((rng.next_u64() as u128) << 64);
                            S::from(match index % 11 {
                                0 => 0,
                                1 => u128::MAX,
                                2 => 1u128 << 127,
                                3 => 1u128 << 64,
                                _ => random,
                            })
                        })
                        .collect::<Vec<_>>();
                    let expected = independent_column_encoding(&layout, &messages, columns);
                    // Nonzero destination contents catch any dependence on the
                    // old output buffer now that initial fill targets scratch.
                    let mut output = vec![S::from(u128::MAX); expected.len()];
                    layout
                        .encode_interleaved_column_major_into_row_major(
                            &messages,
                            columns,
                            &mut output,
                        )
                        .unwrap();
                    assert_eq!(
                        output, expected,
                        "column-major rows={message_rows} columns={columns}",
                    );
                    output.fill(S::from(u128::MAX));
                    layout
                        .encode_interleaved_generated_into_row_major(
                            columns,
                            &mut output,
                            |row, column| messages[column * message_rows + row],
                        )
                        .unwrap();
                    assert_eq!(
                        output, expected,
                        "generated rows={message_rows} columns={columns}",
                    );
                }
            }
        };
        #[cfg(feature = "parallel")]
        for threads in [1, 4] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap()
                .install(check);
        }
        #[cfg(not(feature = "parallel"))]
        check();
    }

    #[test]
    fn interleaved_pingpong_field_matches_independent_columns() {
        check_interleaved_pingpong::<BinaryExtensionField>();
    }

    #[test]
    fn interleaved_pingpong_packed_words_match_independent_columns() {
        check_interleaved_pingpong::<u128>();
    }

    fn check_interleaved_staged_transpose<S: RaaAdditiveSymbol + From<u128>>() {
        let check = || {
            // Rectangular tails exercise both tile dimensions independently
            // of the power-of-two shapes accepted by RaaLayout.
            for rows in [0, 1, 31, 32, 33, 63, 64, 65] {
                for columns in [1, 2, 8, 31, 32, 33, 128, 512] {
                    let input = (0..rows * columns)
                        .map(|index| {
                            S::from((1u128 << 127) ^ ((index as u128) << 65) ^ index as u128)
                        })
                        .collect::<Vec<_>>();
                    let mut output = vec![S::from(u128::MAX); input.len()];
                    transpose_interleaved_message_into_row_major(
                        &input,
                        rows,
                        columns,
                        &mut output,
                    );
                    for row in 0..rows {
                        for column in 0..columns {
                            assert_eq!(output[row * columns + column], input[column * rows + row]);
                        }
                    }
                }
            }

            let mut rng = StdRng::seed_from_u64(0x7374_6167_6564_7261);
            // Last three configurations select the staged path; the others
            // cover both dispatch thresholds and the narrow-column fallback.
            for (rows, columns, staged) in [
                (64, 128, false),
                (1024, 31, false),
                (4096, 8, false),
                (1024, 32, true),
                (256, 128, true),
                (64, 512, true),
            ] {
                assert_eq!(columns >= 32 && rows * columns >= 32 * 1024, staged);
                let layout = RaaLayout::sample(
                    rows,
                    4,
                    RaaDistanceProfile::blaze_assumed(4).unwrap(),
                    &mut rng,
                )
                .unwrap();
                let input = (0..rows * columns)
                    .map(|index| {
                        S::from(match index % 7 {
                            0 => 0,
                            1 => u128::MAX,
                            2 => 1u128 << 127,
                            _ => rng.next_u64() as u128 | ((rng.next_u64() as u128) << 64),
                        })
                    })
                    .collect::<Vec<_>>();
                let expected = independent_column_encoding(&layout, &input, columns);
                let mut actual = vec![S::from(u128::MAX); expected.len()];
                layout
                    .encode_interleaved_column_major_into_row_major(&input, columns, &mut actual)
                    .unwrap();
                assert_eq!(
                    actual, expected,
                    "staged={staged} rows={rows} columns={columns}"
                );
            }
        };
        #[cfg(feature = "parallel")]
        for workers in [1, 4] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap()
                .install(check);
        }
        #[cfg(not(feature = "parallel"))]
        check();
    }

    #[test]
    fn interleaved_staged_transpose_field_matches_independent_columns() {
        check_interleaved_staged_transpose::<BinaryExtensionField>();
    }

    #[test]
    fn interleaved_staged_transpose_packed_words_match_independent_columns() {
        check_interleaved_staged_transpose::<u128>();
    }

    fn check_fused_interleaved_chunk_boundaries<S: RaaAdditiveSymbol + From<u128>>() {
        let check = || {
            let mut rng = StdRng::seed_from_u64(0x6675_7365_645f_7261);
            // Rate one allows exact 63/64/65 and 127/128/129 row boundaries;
            // rate four additionally checks the actual Repeat4 convention.
            for message_rows in [0, 1, 15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129] {
                for rate in [1, 4] {
                    let rows = message_rows * rate;
                    let mut permutations = [
                        (0..rows as u32).collect::<Vec<_>>(),
                        (0..rows as u32).collect::<Vec<_>>(),
                    ];
                    for permutation in &mut permutations {
                        for i in (1..permutation.len()).rev() {
                            let j = rng.next_u64() as usize % (i + 1);
                            permutation.swap(i, j);
                        }
                    }
                    for columns in [1, 2, 8, 31, 32, 33, 128] {
                        let message = (0..message_rows * columns)
                            .map(|index| {
                                S::from(match index % 7 {
                                    0 => 0,
                                    1 => u128::MAX,
                                    2 => 1u128 << 127,
                                    _ => rng.next_u64() as u128 | ((rng.next_u64() as u128) << 64),
                                })
                            })
                            .collect::<Vec<_>>();
                        let mut expected = vec![S::zero(); rows * columns];
                        for column in 0..columns {
                            let mut encoded = (0..message_rows)
                                .flat_map(|row| {
                                    core::iter::repeat_n(message[row * columns + column], rate)
                                })
                                .collect::<Vec<_>>();
                            for permutation in &permutations {
                                let mut next = permutation
                                    .iter()
                                    .map(|&i| encoded[i as usize])
                                    .collect::<Vec<_>>();
                                let mut sum = S::zero();
                                for value in &mut next {
                                    sum = sum.add(*value);
                                    *value = sum;
                                }
                                encoded = next;
                            }
                            for (row, value) in encoded.into_iter().enumerate() {
                                expected[row * columns + column] = value;
                            }
                        }
                        let mut actual = vec![S::from(u128::MAX); rows * columns];
                        actual[..message.len()].copy_from_slice(&message);
                        let mut scratch = vec![S::from(u128::MAX); actual.len()];
                        finish_fused_staged_interleaved_encoding(
                            message.len(),
                            columns,
                            rate,
                            &permutations[0],
                            &permutations[1],
                            &mut actual,
                            &mut scratch,
                        );
                        assert_eq!(
                            actual, expected,
                            "fused rows={rows} columns={columns} rate={rate}"
                        );
                    }
                }
            }
        };
        #[cfg(feature = "parallel")]
        for workers in [1, 4] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap()
                .install(check);
        }
        #[cfg(not(feature = "parallel"))]
        check();
    }

    #[test]
    fn fused_interleaved_field_matches_independent_chunk_boundaries() {
        check_fused_interleaved_chunk_boundaries::<BinaryExtensionField>();
    }

    #[test]
    fn fused_interleaved_packed_words_match_independent_chunk_boundaries() {
        check_fused_interleaved_chunk_boundaries::<u128>();
    }

    #[cfg(feature = "parallel")]
    #[test]
    #[ignore = "64-worker encoding ablation: 2 GiB output, about 6.5 GiB peak RAM, Linux /proc required"]
    fn encoding_layout_ablation() {
        const MESSAGE_ROWS: usize = 65_536;
        const COLUMNS: usize = 512;
        const WORKERS: usize = 64;
        const REPS: usize = 3;
        const NAMES: [&str; 3] = ["v6_gather_pingpong", "v7_transpose_pingpong", "v8_fused"];

        fn minor_faults() -> u64 {
            let stat = std::fs::read_to_string("/proc/self/stat")
                .expect("Linux /proc/self/stat is required");
            // The comm field can itself contain spaces or parentheses. Field
            // ten (minflt) is index seven after the final ') ' delimiter.
            // https://man7.org/linux/man-pages/man5/proc_pid_stat.5.html
            stat.rsplit_once(") ")
                .expect("proc stat comm delimiter")
                .1
                .split_whitespace()
                .nth(7)
                .expect("proc stat minflt")
                .parse()
                .expect("numeric proc stat minflt")
        }

        fn encode_variant(
            layout: &RaaLayout,
            message: &[BinaryExtensionField],
            output: &mut [BinaryExtensionField],
            variant: usize,
        ) {
            if variant == 2 {
                layout
                    .encode_interleaved_column_major_into_row_major(message, COLUMNS, output)
                    .unwrap();
                return;
            }
            // This allocation, its initialization, and its eventual release
            // are inside the measured encoder, matching the production API.
            let mut scratch = vec![BinaryExtensionField::ZERO; output.len()];
            if variant == 0 {
                fill_interleaved_repeated_permutation(
                    message,
                    MESSAGE_ROWS,
                    COLUMNS,
                    layout.rate_inv(),
                    layout.permutation(1).unwrap().sources(),
                    &mut scratch,
                );
            } else {
                let staged = &mut output[..message.len()];
                transpose_interleaved_message_into_row_major(
                    message,
                    MESSAGE_ROWS,
                    COLUMNS,
                    staged,
                );
                fill_row_major_interleaved_repeated_permutation(
                    staged,
                    COLUMNS,
                    layout.rate_inv(),
                    layout.permutation(1).unwrap().sources(),
                    &mut scratch,
                );
            }
            layout.finish_interleaved_row_major_encoding(COLUMNS, output, &mut scratch);
        }

        rayon::ThreadPoolBuilder::new().num_threads(WORKERS).build().unwrap().install(|| {
            let mut rng = StdRng::seed_from_u64(97);
            let layout = RaaLayout::sample(MESSAGE_ROWS, 4, RaaDistanceProfile::blaze_assumed(4).unwrap(), &mut rng).unwrap();
            let message = (0..MESSAGE_ROWS * COLUMNS).map(|_| {
                BinaryExtensionField::from(rng.next_u64() as u128 | ((rng.next_u64() as u128) << 64))
            }).collect::<Vec<_>>();
            let encoded_entries = layout.codeword_len() * COLUMNS;
            let mut expected = vec![BinaryExtensionField::ZERO; encoded_entries];
            let mut output = vec![BinaryExtensionField::ZERO; encoded_entries];
            // calloc alone need not touch physical pages. The opaque mutable
            // slice forces a real fill before reference/warmup or timing.
            let faults_before_touch = minor_faults();
            std::hint::black_box(expected.as_mut_slice()).fill(BinaryExtensionField::ZERO);
            std::hint::black_box(output.as_mut_slice()).fill(BinaryExtensionField::ZERO);
            let faults_after_touch = minor_faults();
            // One untimed v6 reference establishes the complete expected
            // codeword. All nine measured outputs are compared byte-for-byte.
            encode_variant(&layout, &message, &mut expected, 0);
            println!("# encoding_layout_ablation rows={MESSAGE_ROWS} columns={COLUMNS} encoded_bytes={} threads={WORKERS} reps={REPS} seed=97 output_pretouch_minor_faults={}",
                encoded_entries * core::mem::size_of::<BinaryExtensionField>(), faults_after_touch - faults_before_touch);
            println!("# Scratch allocation/init/drop timed; input/setup/output allocation and pretouch, proc reads, reference and equality checks excluded. One untimed v6 reference; no per-variant warmup. Order rotates by repetition.");
            println!("record,variant,threads,repetition,position,time_ms,minor_fault_delta,verified");
            let mut times = [[0.0f64; REPS]; 3];
            for repetition in 0..REPS {
                for position in 0..3 {
                    let variant = (repetition + position) % 3;
                    let faults_before = minor_faults();
                    let start = Instant::now();
                    encode_variant(&layout, &message, &mut output, variant);
                    let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
                    let faults_after = minor_faults();
                    assert!(output == expected, "encoder output differs for {} repetition {}", NAMES[variant], repetition + 1);
                    times[variant][repetition] = elapsed_ms;
                    println!("sample,{},{WORKERS},{},{},{elapsed_ms:.6},{},true", NAMES[variant], repetition + 1, position + 1, faults_after - faults_before);
                }
            }
            for (variant, samples) in times.iter_mut().enumerate() {
                samples.sort_by(f64::total_cmp);
                println!("median,{},{WORKERS},0,0,{:.6},,true", NAMES[variant], samples[REPS / 2]);
            }
        });
    }

    #[test]
    fn optimized_scan_powers_and_dot_match_serial_at_chunk_boundaries() {
        let check = || {
            for len in [0, 1, 17, 4097, 32767, 32768, 32769, 131073] {
                let values = (0..len)
                    .map(|i| BinaryExtensionField::new(((i as u128 + 3) << 79) ^ (i as u128 * 97)))
                    .collect::<Vec<_>>();
                let mut expected = values.clone();
                suffix_accumulate_serial(&mut expected);
                let mut actual = values.clone();
                suffix_accumulate(&mut actual);
                assert_eq!(actual, expected, "suffix length {len}");
                for base in [
                    BinaryExtensionField::ZERO,
                    BinaryExtensionField::ONE,
                    BinaryExtensionField::MULTIPLICATIVE_GENERATOR,
                ] {
                    for first in [0, 1] {
                        let actual = geometric_power_values_from(base, len, first);
                        let mut power = field_power(base, first);
                        for value in actual {
                            assert_eq!(value, power);
                            power *= base;
                        }
                    }
                }
                let factor = BinaryExtensionField::new(0xdeadbeef123456789);
                let source = values.iter().rev().copied().collect::<Vec<_>>();
                let mut expected = values.clone();
                for (target, value) in expected.iter_mut().zip(&source) {
                    *target += factor * *value;
                }
                let mut actual = values.clone();
                add_scaled_values(&mut actual, &source, factor);
                assert_eq!(actual, expected);
                let expected = values
                    .iter()
                    .zip(&source)
                    .fold(BinaryExtensionField::ZERO, |sum, (&a, &b)| sum + a * b);
                assert_eq!(inner_product(&values, &source), expected);
            }
        };
        #[cfg(feature = "parallel")]
        for threads in [1, 4] {
            rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap()
                .install(check);
        }
        #[cfg(not(feature = "parallel"))]
        check();
    }

    #[test]
    fn repeat_four_two_accumulators_match_independent_encoding() {
        let mut rng = StdRng::seed_from_u64(0x42524141);
        for message_len in [1usize, 4, 32, 256] {
            let code = RaaCode::sample(
                message_len,
                4,
                RaaDistanceProfile::blaze_assumed(4).unwrap(),
                &mut rng,
            )
            .unwrap();
            let message = (0..message_len)
                .map(|i| BinaryExtensionField::from((i as u128 + 3) << 70 ^ (i as u128 * 79)))
                .collect::<Vec<_>>();
            let mut expected = message.iter().flat_map(|&x| [x; 4]).collect::<Vec<_>>();
            for round in 1..=2 {
                expected = code
                    .layout()
                    .permutation(round)
                    .unwrap()
                    .sources()
                    .iter()
                    .map(|&source| expected[source as usize])
                    .collect();
                let mut sum = BinaryExtensionField::ZERO;
                for x in &mut expected {
                    sum += *x;
                    *x = sum;
                }
            }
            assert_eq!(code.encode(&message).unwrap(), expected);
            assert_eq!(
                code.layout().encode_additive_serial(&message).unwrap(),
                expected
            );
            let mut row_major = vec![BinaryExtensionField::ZERO; expected.len() * 2];
            let columns = [message.clone(), message.clone()].concat();
            code.layout()
                .encode_interleaved_column_major_into_row_major(&columns, 2, &mut row_major)
                .unwrap();
            for (row, expected) in row_major.chunks_exact(2).zip(expected) {
                assert_eq!(row, &[expected, expected]);
            }
        }
    }
}
