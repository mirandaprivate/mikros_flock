//! Packed short-bitwidth encoding for the two-accumulator RAA code.
//!
//! The committed witness consists of logical `b`-bit values for
//! `b in {1, 2, 4, 8, 16, 32, 64, 128}`. Several values are stored in one
//! `u128` word. Repeat, permutation, and prefix accumulation use only XOR, so
//! this short representation can be encoded without expanding every value.
//! Opening challenges, MLEs, and generator claims nevertheless live in
//! Binius' 128-bit binary extension field. The first opening round unpacks and
//! embeds each short value into that field before any multiplication.
//! Generator-matrix MLE claims delegate to [`RaaCode`]; the two permutation
//! relations are batched into one prefix-product/Hadamard reduction.
//! This module also provides a row-oriented BLAKE2s commitment for packed
//! codewords. Each word is absorbed at its native byte width; the commitment
//! envelope additionally binds the logical element width represented inside
//! each word. Fixed permutation-index columns and generator-MLE reductions remain in the
//! 128-bit extension field. A complete recursive opening or Decide protocol
//! remains the responsibility of the enclosing PCS.

use core::marker::PhantomData;

use ark_std::{rand::RngCore, vec, vec::Vec};
use binius_field::Field;
use binius_iop::channel::{IOPVerifierChannel, OracleSpec};
use binius_iop_prover::channel::IOPProverChannel;
use blake2::Blake2s256;
use digest::Digest;
#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::Error;

use crate::queries_for_relative_distance;
use crate::raafold::{
    raafold_rate_inv_for_level, BinaryExtensionField, PackedBinaryExtensionField,
    RaaAdditiveSymbol, RaaCode, RaaDistanceProfile, RaaFold as ExtensionRaaFold,
    RaaGeneratorReduction, RaaMatrixClaim, RaaPermutation,
};
const DIGEST_BYTES: usize = 32;
const SHORT_RAA_LEAF_DOMAIN: &[u8] = b"brakefold/raafold-short/packed-row/v1";
const SHORT_RAA_INNER_DOMAIN: &[u8] = b"brakefold/raafold-short/merkle-inner/v1";
const SHORT_RAA_ROOT_DOMAIN: &[u8] = b"brakefold/raafold-short/root-envelope/v2";

mod sealed {
    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for u128 {}
}

/// A machine word containing independent binary RAA lanes.
pub trait RaaPackedWord: RaaAdditiveSymbol + sealed::Sealed {
    /// Number of binary lanes in the word.
    const BITS: usize;

    /// Builds a packed word from the low bits of a `u128`.
    fn from_u128(value: u128) -> Self;

    /// Returns the packed lanes in the low bits of a `u128`.
    fn to_u128(self) -> u128;
}

macro_rules! impl_raa_packed_word {
    ($($word:ty),+ $(,)?) => {
        $(
            impl RaaAdditiveSymbol for $word {
                #[inline]
                fn zero() -> Self {
                    0
                }

                #[inline]
                fn add(self, rhs: Self) -> Self {
                    self ^ rhs
                }
            }

            impl RaaPackedWord for $word {
                const BITS: usize = <$word>::BITS as usize;

                #[inline]
                fn from_u128(value: u128) -> Self {
                    value as $word
                }

                #[inline]
                fn to_u128(self) -> u128 {
                    self as u128
                }
            }
        )+
    };
}

impl_raa_packed_word!(u8, u16, u32, u64, u128);

/// A BLAKE2s Merkle commitment to one packed RAA codeword matrix.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShortRaaFoldCommitment {
    root: [u8; DIGEST_BYTES],
    encoded_rows: usize,
    columns: usize,
    word_bits: usize,
    logical_element_bits: usize,
    logical_elements: usize,
    generator_digest: [u8; DIGEST_BYTES],
}

impl ShortRaaFoldCommitment {
    /// Returns the row-oriented Merkle root.
    pub fn root(self) -> [u8; DIGEST_BYTES] {
        self.root
    }

    /// Returns the number of encoded rows committed in each column.
    pub fn encoded_rows(self) -> usize {
        self.encoded_rows
    }

    /// Returns the number of packed columns in every authenticated row.
    pub fn columns(self) -> usize {
        self.columns
    }

    /// Returns the number of independent binary lanes in each packed word.
    pub fn word_bits(self) -> usize {
        self.word_bits
    }

    /// Returns the bit width of each logical witness element packed into a word.
    pub fn logical_element_bits(self) -> usize {
        self.logical_element_bits
    }

    /// Returns the number of logical short witness elements.
    pub fn logical_elements(self) -> usize {
        self.logical_elements
    }

    /// Returns the digest of the RAA generator bound into every leaf.
    pub fn generator_digest(self) -> [u8; DIGEST_BYTES] {
        self.generator_digest
    }
}

/// One BLAKE2s authentication path for a packed RAA row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortRaaMerklePath {
    /// Sibling digests ordered from the leaf level to the root.
    pub siblings: Vec<[u8; DIGEST_BYTES]>,
}

/// One authenticated row of packed RAA codeword symbols.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortRaaRowQuery<W: RaaPackedWord> {
    /// Row index in the encoded matrix.
    pub row_index: usize,
    /// Complete packed row across all committed columns.
    pub row: Vec<W>,
    /// Merkle authentication path for this row.
    pub path: ShortRaaMerklePath,
}

/// Prover state retained after committing one packed RAA codeword matrix.
pub struct ShortRaaFoldCommitmentCache<W: RaaPackedWord> {
    message_entries: Vec<W>,
    encoded_entries: Vec<W>,
    encoded_rows: usize,
    columns: usize,
    tree: ShortRaaMerkleTree,
}

/// Column-major shape of a witness whose short values are packed into `u128` words.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShortRaaPackedShape {
    message_rows: usize,
    packed_columns: usize,
    element_bits: usize,
}

impl ShortRaaPackedShape {
    /// Creates a valid packed witness shape.
    pub fn new(
        message_rows: usize,
        packed_columns: usize,
        element_bits: usize,
    ) -> Result<Self, Error> {
        if message_rows == 0
            || !message_rows.is_power_of_two()
            || packed_columns == 0
            || !packed_columns.is_power_of_two()
            || !valid_short_element_bits(element_bits)
        {
            return Err(Error::EncodingError);
        }
        let shape = Self {
            message_rows,
            packed_columns,
            element_bits,
        };
        shape.logical_elements()?;
        Ok(shape)
    }

    /// Returns the number of message rows.
    pub fn message_rows(self) -> usize {
        self.message_rows
    }

    /// Returns the number of physical packed columns.
    pub fn packed_columns(self) -> usize {
        self.packed_columns
    }

    /// Returns the bit width of one logical witness element.
    pub fn element_bits(self) -> usize {
        self.element_bits
    }

    /// Returns the number of short values stored in one physical word.
    pub fn values_per_word(self) -> usize {
        u128::BITS as usize / self.element_bits
    }

    /// Returns the number of logical short-value columns.
    pub fn logical_columns(self) -> Result<usize, Error> {
        self.packed_columns
            .checked_mul(self.values_per_word())
            .ok_or(Error::EncodingError)
    }

    /// Returns the total number of logical witness elements.
    pub fn logical_elements(self) -> Result<usize, Error> {
        self.message_rows
            .checked_mul(self.logical_columns()?)
            .ok_or(Error::EncodingError)
    }

    fn packed_words(self) -> Result<usize, Error> {
        self.message_rows
            .checked_mul(self.packed_columns)
            .ok_or(Error::EncodingError)
    }
}

/// An owned short-value witness packed in column-major matrix order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortRaaPackedWitness {
    shape: ShortRaaPackedShape,
    words: Vec<u128>,
}

impl ShortRaaPackedWitness {
    /// Validates an already packed witness.
    pub fn from_packed_words(shape: ShortRaaPackedShape, words: Vec<u128>) -> Result<Self, Error> {
        if words.len() != shape.packed_words()? {
            return Err(Error::EncodingError);
        }
        Ok(Self { shape, words })
    }

    /// Packs logical values supplied in column-major order.
    pub fn from_column_major_values(
        shape: ShortRaaPackedShape,
        values: &[u128],
    ) -> Result<Self, Error> {
        if values.len() != shape.logical_elements()? {
            return Err(Error::EncodingError);
        }
        let mut words = vec![0u128; shape.packed_words()?];
        let values_per_word = shape.values_per_word();
        let mask = short_value_mask(shape.element_bits);
        for packed_column in 0..shape.packed_columns {
            for row in 0..shape.message_rows {
                let mut word = 0u128;
                for lane in 0..values_per_word {
                    let logical_column = packed_column * values_per_word + lane;
                    let value = values[logical_column * shape.message_rows + row];
                    if value & !mask != 0 {
                        return Err(Error::EncodingError);
                    }
                    word |= value << (lane * shape.element_bits);
                }
                words[packed_column * shape.message_rows + row] = word;
            }
        }
        Ok(Self { shape, words })
    }

    /// Returns the authenticated logical shape.
    pub fn shape(&self) -> ShortRaaPackedShape {
        self.shape
    }

    /// Returns the packed physical words.
    pub fn packed_words(&self) -> &[u128] {
        &self.words
    }

    /// Returns one logical short value by row and logical column.
    pub fn value(&self, row: usize, logical_column: usize) -> Result<u128, Error> {
        packed_short_value(&self.words, self.shape, row, logical_column)
    }

    fn into_words(self) -> Vec<u128> {
        self.words
    }
}

impl<W: RaaPackedWord> ShortRaaFoldCommitmentCache<W> {
    /// Opens distinct selected rows in caller-provided order.
    pub fn open_rows(&self, row_indices: &[usize]) -> Result<Vec<ShortRaaRowQuery<W>>, Error> {
        if row_indices.is_empty()
            || row_indices.iter().enumerate().any(|(position, row)| {
                *row >= self.encoded_rows || row_indices[..position].contains(row)
            })
        {
            return Err(Error::EncodingError);
        }
        row_indices
            .iter()
            .map(|row_index| {
                Ok(ShortRaaRowQuery {
                    row_index: *row_index,
                    row: packed_encoded_row(
                        &self.encoded_entries,
                        self.encoded_rows,
                        self.columns,
                        *row_index,
                    )?,
                    path: self.tree.path(*row_index)?,
                })
            })
            .collect()
    }
}

#[derive(Clone)]
struct ShortRaaMerkleTree {
    levels: Vec<Vec<[u8; DIGEST_BYTES]>>,
}

impl ShortRaaMerkleTree {
    fn from_leaves(leaves: Vec<[u8; DIGEST_BYTES]>) -> Result<Self, Error> {
        if leaves.is_empty() || !leaves.len().is_power_of_two() {
            return Err(Error::EncodingError);
        }
        let mut levels = vec![leaves];
        while levels.last().map(Vec::len).unwrap_or_default() > 1 {
            let children = levels.last().ok_or(Error::HashingError)?;
            #[cfg(feature = "parallel")]
            let parents = children
                .par_chunks_exact(2)
                .map(|pair| hash_short_raa_inner(&pair[0], &pair[1]))
                .collect::<Vec<_>>();
            #[cfg(not(feature = "parallel"))]
            let parents = children
                .chunks_exact(2)
                .map(|pair| hash_short_raa_inner(&pair[0], &pair[1]))
                .collect::<Vec<_>>();
            levels.push(parents);
        }
        Ok(Self { levels })
    }

    fn root(&self) -> [u8; DIGEST_BYTES] {
        self.levels
            .last()
            .and_then(|level| level.first())
            .copied()
            .unwrap_or([0u8; DIGEST_BYTES])
    }

    fn path(&self, mut index: usize) -> Result<ShortRaaMerklePath, Error> {
        if index >= self.levels.first().map(Vec::len).unwrap_or_default() {
            return Err(Error::InvalidCommitment);
        }
        let mut siblings = Vec::with_capacity(self.levels.len().saturating_sub(1));
        for level in self.levels.iter().take(self.levels.len().saturating_sub(1)) {
            siblings.push(level[index ^ 1]);
            index >>= 1;
        }
        Ok(ShortRaaMerklePath { siblings })
    }
}

/// A packed-lane view of one [`RaaCode`] setup.
#[derive(Clone, Debug)]
pub struct ShortRaaCode<W: RaaPackedWord> {
    extension_code: RaaCode,
    marker: PhantomData<W>,
}

impl<W: RaaPackedWord> ShortRaaCode<W> {
    /// Samples two permutations and shares them with the extension-field code.
    pub fn sample<R: RngCore>(
        message_len: usize,
        rate_inv: usize,
        distance: RaaDistanceProfile,
        rng: &mut R,
    ) -> Result<Self, Error> {
        Ok(Self::from_extension_code(RaaCode::sample(
            message_len,
            rate_inv,
            distance,
            rng,
        )?))
    }

    /// Builds a packed code from two explicit source-index permutations.
    pub fn from_permutations(
        message_len: usize,
        rate_inv: usize,
        distance: RaaDistanceProfile,
        permutations: [RaaPermutation; 2],
    ) -> Result<Self, Error> {
        Ok(Self::from_extension_code(RaaCode::from_permutations(
            message_len,
            rate_inv,
            distance,
            permutations,
        )?))
    }

    /// Reuses an existing extension-field setup without copying its permutations.
    pub fn from_extension_code(extension_code: RaaCode) -> Self {
        Self {
            extension_code,
            marker: PhantomData,
        }
    }

    /// Returns the canonical extension-field code for this setup.
    pub fn extension_code(&self) -> &RaaCode {
        &self.extension_code
    }

    /// Returns the message length in packed words.
    pub fn message_len(&self) -> usize {
        self.extension_code.message_len()
    }

    /// Returns the inverse code rate.
    pub fn rate_inv(&self) -> usize {
        self.extension_code.rate_inv()
    }

    /// Returns the codeword length in packed words.
    pub fn codeword_len(&self) -> usize {
        self.extension_code.codeword_len()
    }

    /// Returns the number of independent binary lanes in one symbol.
    pub const fn packed_lanes(&self) -> usize {
        W::BITS
    }

    /// Returns the explicit distance profile shared with the extension code.
    pub fn distance_profile(&self) -> RaaDistanceProfile {
        self.extension_code.distance_profile()
    }

    /// Returns the digest binding dimensions and both permutations.
    pub fn generator_digest(&self) -> [u8; 32] {
        self.extension_code.generator_digest()
    }

    /// Encodes all packed binary lanes with the RAA pipeline.
    pub fn encode(&self, message: &[W]) -> Result<Vec<W>, Error> {
        self.extension_code.layout().encode_additive(message)
    }

    /// Encodes independent packed columns, parallelizing across columns.
    pub fn encode_columns(&self, messages: &[Vec<W>]) -> Result<Vec<Vec<W>>, Error> {
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
            .map(|message| self.extension_code.layout().encode_additive_serial(message))
            .collect::<Result<Vec<_>, _>>()?;
        #[cfg(not(feature = "parallel"))]
        let encoded = messages
            .iter()
            .map(|message| self.extension_code.layout().encode_additive_serial(message))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(encoded)
    }

    /// Encodes and commits an owned column-major packed witness.
    ///
    /// The encoded matrix is stored row-major. Every permutation moves a full
    /// interleaved row, and every accumulator XORs full rows in parallel row
    /// chunks.
    pub fn commit_owned(
        &self,
        witness: Vec<W>,
        columns: usize,
    ) -> Result<(ShortRaaFoldCommitment, ShortRaaFoldCommitmentCache<W>), Error> {
        self.commit_packed_owned(witness, columns, W::BITS)
    }

    /// Encodes and commits packed words with an explicit logical element width.
    ///
    /// The RAA encoder operates independently on every binary lane. The
    /// `logical_element_bits` value records how adjacent lanes are grouped by
    /// the caller, so the same packed bytes cannot be reopened under a
    /// different short-bitwidth interpretation.
    pub fn commit_packed_owned(
        &self,
        witness: Vec<W>,
        columns: usize,
        logical_element_bits: usize,
    ) -> Result<(ShortRaaFoldCommitment, ShortRaaFoldCommitmentCache<W>), Error> {
        self.commit_packed_owned_with_binding(
            witness,
            columns,
            logical_element_bits,
            self.generator_digest(),
        )
    }

    fn commit_packed_owned_with_binding(
        &self,
        witness: Vec<W>,
        columns: usize,
        logical_element_bits: usize,
        binding_digest: [u8; DIGEST_BYTES],
    ) -> Result<(ShortRaaFoldCommitment, ShortRaaFoldCommitmentCache<W>), Error> {
        let message_rows = self.message_len();
        let encoded_rows = self.codeword_len();
        if !valid_logical_element_bits::<W>(logical_element_bits)
            || columns == 0
            || witness.len()
                != message_rows
                    .checked_mul(columns)
                    .ok_or(Error::EncodingError)?
        {
            return Err(Error::EncodingError);
        }
        let encoded_len = encoded_rows
            .checked_mul(columns)
            .ok_or(Error::EncodingError)?;
        let logical_elements = message_rows
            .checked_mul(columns)
            .and_then(|words| words.checked_mul(W::BITS / logical_element_bits))
            .ok_or(Error::EncodingError)?;
        let mut encoded_entries = vec![W::zero(); encoded_len];
        self.extension_code
            .layout()
            .encode_interleaved_column_major_into_row_major(
                &witness,
                columns,
                &mut encoded_entries,
            )?;
        let tree = build_short_raa_tree::<W>(&encoded_entries, encoded_rows, columns)?;
        let root = hash_short_raa_root::<W>(
            &tree.root(),
            encoded_rows,
            columns,
            logical_element_bits,
            logical_elements,
            &binding_digest,
        )?;
        let commitment = ShortRaaFoldCommitment {
            root,
            encoded_rows,
            columns,
            word_bits: W::BITS,
            logical_element_bits,
            logical_elements,
            generator_digest: binding_digest,
        };
        Ok((
            commitment,
            ShortRaaFoldCommitmentCache {
                message_entries: witness,
                encoded_entries,
                encoded_rows,
                columns,
                tree,
            },
        ))
    }

    /// Verifies authenticated packed rows at verifier-selected indices.
    pub fn verify_rows(
        &self,
        commitment: &ShortRaaFoldCommitment,
        expected_indices: &[usize],
        queries: &[ShortRaaRowQuery<W>],
    ) -> Result<bool, Error> {
        self.verify_packed_rows(
            commitment,
            W::BITS,
            &self.generator_digest(),
            expected_indices,
            queries,
        )
    }

    /// Verifies rows under an explicit packed-element interpretation.
    pub fn verify_packed_rows(
        &self,
        commitment: &ShortRaaFoldCommitment,
        logical_element_bits: usize,
        binding_digest: &[u8; DIGEST_BYTES],
        expected_indices: &[usize],
        queries: &[ShortRaaRowQuery<W>],
    ) -> Result<bool, Error> {
        if commitment.encoded_rows != self.codeword_len()
            || commitment.columns == 0
            || commitment.word_bits != W::BITS
            || commitment.logical_element_bits != logical_element_bits
            || !valid_logical_element_bits::<W>(logical_element_bits)
            || commitment.logical_elements
                != self
                    .message_len()
                    .checked_mul(commitment.columns)
                    .and_then(|words| words.checked_mul(W::BITS / commitment.logical_element_bits))
                    .ok_or(Error::EncodingError)?
            || commitment.generator_digest != *binding_digest
            || expected_indices.is_empty()
            || expected_indices.len() != queries.len()
        {
            return Ok(false);
        }
        for (position, (expected_index, query)) in expected_indices.iter().zip(queries).enumerate()
        {
            if *expected_index != query.row_index
                || query.row_index >= commitment.encoded_rows
                || query.row.len() != commitment.columns
                || expected_indices[..position].contains(expected_index)
            {
                return Ok(false);
            }
            let leaf = hash_short_raa_row::<W>(&query.row, commitment.columns)?;
            let Some(tree_root) = short_raa_merkle_root_from_path(
                leaf,
                query.row_index,
                commitment.encoded_rows,
                &query.path,
            ) else {
                return Ok(false);
            };
            let root = hash_short_raa_root::<W>(
                &tree_root,
                commitment.encoded_rows,
                commitment.columns,
                commitment.logical_element_bits,
                commitment.logical_elements,
                &commitment.generator_digest,
            )?;
            if root != commitment.root {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Computes `G r` independently in every packed binary lane.
    pub fn generator_right_projection(&self, right: &[W]) -> Result<Vec<W>, Error> {
        self.encode(right)
    }

    /// Computes `l^T G` independently in every packed binary lane.
    pub fn generator_left_projection(&self, left: &[W]) -> Result<Vec<W>, Error> {
        if left.len() != self.codeword_len() {
            return Err(Error::EncodingError);
        }
        let mut current = left.to_vec();
        let mut scratch = vec![W::zero(); left.len()];

        for round in (1..=3).rev() {
            suffix_accumulate(&mut current);
            apply_permutation_transpose(
                &current,
                self.extension_code.layout().permutation(round)?.sources(),
                &mut scratch,
            );
            core::mem::swap(&mut current, &mut scratch);
        }

        Ok(current
            .chunks_exact(self.rate_inv())
            .map(|chunk| {
                chunk
                    .iter()
                    .copied()
                    .fold(W::zero(), |sum, value| sum.add(value))
            })
            .collect())
    }

    /// Evaluates the shared generator MLE over Binius' extension field.
    pub fn evaluate_generator_mle(
        &self,
        left_point: &[BinaryExtensionField],
        right_point: &[BinaryExtensionField],
    ) -> Result<BinaryExtensionField, Error> {
        self.extension_code
            .evaluate_generator_mle(left_point, right_point)
    }

    /// Delegates the generator reduction to the extension-field code.
    ///
    /// The result has one grand product batching the two setup permutations.
    pub fn reduce_prover<Channel>(
        &self,
        claim: &RaaMatrixClaim<BinaryExtensionField>,
        channel: &mut Channel,
    ) -> Result<RaaGeneratorReduction<BinaryExtensionField>, Error>
    where
        Channel: IOPProverChannel<PackedBinaryExtensionField>,
    {
        self.extension_code.reduce_prover(claim, channel)
    }

    /// Verifies the shared generator reduction as an atomic subprotocol.
    pub fn verify_as_subprotocol<Channel>(
        &self,
        claim: &RaaMatrixClaim<BinaryExtensionField>,
        channel: &mut Channel,
    ) -> Result<RaaGeneratorReduction<BinaryExtensionField>, Error>
    where
        Channel: IOPVerifierChannel<BinaryExtensionField, Elem = BinaryExtensionField>,
    {
        self.extension_code.verify_as_subprotocol(claim, channel)
    }

    /// Returns the pushforward and prefix-product oracle specifications.
    pub fn permutation_oracle_specs(&self) -> [OracleSpec; 2] {
        self.extension_code.permutation_oracle_specs()
    }

    /// Returns one setup-committed `omega^{P_i(j)}` index column.
    pub fn permutation_index_column(
        &self,
        round: usize,
    ) -> Result<Vec<BinaryExtensionField>, Error> {
        self.extension_code.permutation_index_column(round)
    }

    /// Returns the proximity-query count from the shared distance profile.
    pub fn num_queries(&self, security_bits: usize, available_rows: usize) -> Result<usize, Error> {
        self.extension_code
            .num_queries(security_bits, available_rows)
    }
}

/// A packed RAA sequence with rate `1/4` at every level.
#[derive(Clone, Debug)]
pub struct ShortRaaFold<W: RaaPackedWord> {
    codes: Vec<ShortRaaCode<W>>,
    top_switch_ratio: usize,
    recursive_switch_ratio: usize,
    generator_digest: [u8; DIGEST_BYTES],
}

impl<W: RaaPackedWord> ShortRaaFold<W> {
    /// Builds a sequence ordered from the smallest to the largest message.
    ///
    /// At least two levels are required; every level uses repetition four.
    pub fn new(codes: Vec<ShortRaaCode<W>>, switch_ratio: usize) -> Result<Self, Error> {
        Self::new_with_switch_ratios(codes, switch_ratio, switch_ratio)
    }

    /// Builds a sequence with separate highest and recursive switch ratios.
    pub fn new_with_switch_ratios(
        codes: Vec<ShortRaaCode<W>>,
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
        let extension_fold = ExtensionRaaFold::new_with_switch_ratios(
            codes
                .iter()
                .map(|code| code.extension_code().clone())
                .collect(),
            top_switch_ratio,
            recursive_switch_ratio,
        )?;
        Ok(Self {
            codes,
            top_switch_ratio,
            recursive_switch_ratio,
            generator_digest: extension_fold.generator_digest(),
        })
    }

    /// Returns one recursive packed-code level.
    pub fn code(&self, level: usize) -> Result<&ShortRaaCode<W>, Error> {
        self.codes.get(level).ok_or(Error::EncodingError)
    }

    /// Returns the number of recursive levels.
    pub fn levels(&self) -> usize {
        self.codes.len()
    }

    /// Returns all packed RAA codes in ascending message-length order.
    pub fn codes(&self) -> &[ShortRaaCode<W>] {
        &self.codes
    }

    /// Returns the largest packed RAA code in the sequence.
    pub fn top_code(&self) -> Result<&ShortRaaCode<W>, Error> {
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

    /// Returns the digest binding geometry, distances, and every RAA generator.
    ///
    /// Top-level commitments bind this sequence digest, the physical word
    /// width, and the logical element width in one authenticated envelope.
    pub fn generator_digest(&self) -> [u8; DIGEST_BYTES] {
        self.generator_digest
    }

    /// Encodes and commits an owned top-level packed witness matrix.
    pub fn commit_top_owned(
        &self,
        witness: Vec<W>,
        columns: usize,
    ) -> Result<(ShortRaaFoldCommitment, ShortRaaFoldCommitmentCache<W>), Error> {
        self.commit_top_packed_owned(witness, columns, W::BITS)
    }

    /// Commits a packed top-level witness and binds its logical bit width.
    pub fn commit_top_packed_owned(
        &self,
        witness: Vec<W>,
        columns: usize,
        logical_element_bits: usize,
    ) -> Result<(ShortRaaFoldCommitment, ShortRaaFoldCommitmentCache<W>), Error> {
        self.top_code()?.commit_packed_owned_with_binding(
            witness,
            columns,
            logical_element_bits,
            self.generator_digest,
        )
    }

    /// Verifies top-level authenticated rows against this sequence.
    pub fn verify_top_rows(
        &self,
        commitment: &ShortRaaFoldCommitment,
        expected_indices: &[usize],
        queries: &[ShortRaaRowQuery<W>],
    ) -> Result<bool, Error> {
        self.verify_top_packed_rows(commitment, W::BITS, expected_indices, queries)
    }

    /// Verifies top-level rows under the expected logical bit width.
    pub fn verify_top_packed_rows(
        &self,
        commitment: &ShortRaaFoldCommitment,
        logical_element_bits: usize,
        expected_indices: &[usize],
        queries: &[ShortRaaRowQuery<W>],
    ) -> Result<bool, Error> {
        self.top_code()?.verify_packed_rows(
            commitment,
            logical_element_bits,
            &self.generator_digest,
            expected_indices,
            queries,
        )
    }

    /// Returns the proximity-query count for an ordinary opening at one level.
    pub fn opening_num_queries(
        &self,
        level: usize,
        security_bits: usize,
        available_rows: usize,
    ) -> Result<usize, Error> {
        self.code(level)?.num_queries(security_bits, available_rows)
    }

    /// Returns the setup-query count for level `j` from lower `C_{j-1}`.
    ///
    /// Direct interleaving has no RF distance loss, so this uses the lower
    /// code's raw distance and caps the count by its encoded row count.
    pub fn direct_interleaved_num_queries(
        &self,
        source_level: usize,
        security_bits: usize,
    ) -> Result<usize, Error> {
        let target_level = source_level.checked_sub(1).ok_or(Error::EncodingError)?;
        let target = self.code(target_level)?;
        Ok(queries_for_relative_distance(
            security_bits,
            target.distance_profile().code_relative_distance(),
        )?
        .min(target.codeword_len()))
    }
}

impl ShortRaaFold<u128> {
    /// Commits logical short values while retaining only their packed witness form.
    pub fn commit_short_witness_owned(
        &self,
        witness: ShortRaaPackedWitness,
    ) -> Result<(ShortRaaFoldCommitment, ShortRaaFoldCommitmentCache<u128>), Error> {
        let shape = witness.shape();
        if shape.message_rows() != self.top_code()?.message_len() {
            return Err(Error::EncodingError);
        }
        self.commit_top_packed_owned(
            witness.into_words(),
            shape.packed_columns(),
            shape.element_bits(),
        )
    }
}

impl ShortRaaFoldCommitmentCache<u128> {
    /// Returns the logical short-witness shape bound by this cache and commitment.
    pub fn short_witness_shape(
        &self,
        commitment: &ShortRaaFoldCommitment,
        message_rows: usize,
    ) -> Result<ShortRaaPackedShape, Error> {
        if commitment.word_bits != u128::BITS as usize
            || commitment.encoded_rows != self.encoded_rows
            || commitment.columns != self.columns
            || self.encoded_entries.len()
                != self
                    .encoded_rows
                    .checked_mul(self.columns)
                    .ok_or(Error::EncodingError)?
            || self.message_entries.len()
                != message_rows
                    .checked_mul(self.columns)
                    .ok_or(Error::EncodingError)?
        {
            return Err(Error::InvalidCommitment);
        }
        let shape =
            ShortRaaPackedShape::new(message_rows, self.columns, commitment.logical_element_bits)?;
        if shape.logical_elements()? != commitment.logical_elements {
            return Err(Error::InvalidCommitment);
        }
        let expected_root = hash_short_raa_root::<u128>(
            &self.tree.root(),
            commitment.encoded_rows,
            commitment.columns,
            commitment.logical_element_bits,
            commitment.logical_elements,
            &commitment.generator_digest,
        )?;
        if expected_root != commitment.root {
            return Err(Error::InvalidCommitment);
        }
        Ok(shape)
    }

    /// Projects the packed message rows at a `GF(2^128)` MLE row point.
    ///
    /// The returned vector has one extension-field value per logical short
    /// witness column. Every short value is embedded into `GF(2^128)` before
    /// it is multiplied by a row weight.
    pub fn message_row_projection(
        &self,
        commitment: &ShortRaaFoldCommitment,
        message_rows: usize,
        row_weights: &[BinaryExtensionField],
    ) -> Result<Vec<BinaryExtensionField>, Error> {
        let shape = self.short_witness_shape(commitment, message_rows)?;
        if row_weights.len() != message_rows {
            return Err(Error::InvalidNumberOfVariables);
        }
        let logical_columns = shape.logical_columns()?;
        if shape.element_bits() == 128 {
            // Full-width words already contain one raw field element. Stream
            // complete columns without repeated lane extraction or checks.
            let project =
                |column: &[u128]| BinaryExtensionField::dot_product_bits(column, row_weights);
            #[cfg(feature = "parallel")]
            let projected = self
                .message_entries
                .par_chunks_exact(message_rows)
                .map(project)
                .collect();
            #[cfg(not(feature = "parallel"))]
            let projected = self
                .message_entries
                .chunks_exact(message_rows)
                .map(project)
                .collect();
            return Ok(projected);
        }
        let project_column = |logical_column: usize| {
            (0..message_rows)
                .map(|row| {
                    let value =
                        packed_short_value(&self.message_entries, shape, row, logical_column)?;
                    Ok(BinaryExtensionField::from(value) * row_weights[row])
                })
                .try_fold(BinaryExtensionField::ZERO, |sum, term| {
                    term.map(|term| sum + term)
                })
        };
        #[cfg(feature = "parallel")]
        let projected = (0..logical_columns)
            .into_par_iter()
            .map(project_column)
            .collect::<Result<Vec<_>, Error>>()?;
        #[cfg(not(feature = "parallel"))]
        let projected = (0..logical_columns)
            .map(project_column)
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(projected)
    }

    /// Folds all packed message columns with `GF(2^128)` coefficients.
    ///
    /// This is the base-to-extension seam used by the first recursive round.
    /// No expanded `message_rows x logical_columns` field matrix is allocated.
    pub fn fold_message_to_extension(
        &self,
        commitment: &ShortRaaFoldCommitment,
        message_rows: usize,
        column_weights: &[BinaryExtensionField],
    ) -> Result<Vec<BinaryExtensionField>, Error> {
        let shape = self.short_witness_shape(commitment, message_rows)?;
        if column_weights.len() != shape.logical_columns()? {
            return Err(Error::InvalidNumberOfVariables);
        }
        if shape.element_bits() == 128 {
            // Parallelize output tiles, then stream a contiguous slice of
            // every source column. The output tile stays in the L1 cache.
            const TILE_ROWS: usize = 256;
            let mut folded = vec![BinaryExtensionField::ZERO; message_rows];
            let fold_tile = |tile_index: usize, output: &mut [BinaryExtensionField]| {
                let row_start = tile_index * TILE_ROWS;
                for (column, &weight) in column_weights.iter().enumerate() {
                    let start = column * message_rows + row_start;
                    let end = start + output.len();
                    BinaryExtensionField::add_scaled_bits_slice(
                        output,
                        &self.message_entries[start..end],
                        weight,
                    );
                }
            };
            #[cfg(feature = "parallel")]
            folded
                .par_chunks_mut(TILE_ROWS)
                .enumerate()
                .for_each(|(index, output)| fold_tile(index, output));
            #[cfg(not(feature = "parallel"))]
            folded
                .chunks_mut(TILE_ROWS)
                .enumerate()
                .for_each(|(index, output)| fold_tile(index, output));
            return Ok(folded);
        }
        let fold_row = |row: usize| {
            column_weights
                .iter()
                .enumerate()
                .map(|(logical_column, weight)| {
                    let value =
                        packed_short_value(&self.message_entries, shape, row, logical_column)?;
                    Ok(BinaryExtensionField::from(value) * *weight)
                })
                .try_fold(BinaryExtensionField::ZERO, |sum, term| {
                    term.map(|term| sum + term)
                })
        };
        #[cfg(feature = "parallel")]
        let folded = (0..message_rows)
            .into_par_iter()
            .map(fold_row)
            .collect::<Result<Vec<_>, Error>>()?;
        #[cfg(not(feature = "parallel"))]
        let folded = (0..message_rows)
            .map(fold_row)
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(folded)
    }

    /// Folds encoded short rows with extension-field coefficients for row queries.
    pub fn fold_codeword_to_extension(
        &self,
        commitment: &ShortRaaFoldCommitment,
        message_rows: usize,
        column_weights: &[BinaryExtensionField],
    ) -> Result<Vec<BinaryExtensionField>, Error> {
        let shape = self.short_witness_shape(commitment, message_rows)?;
        if column_weights.len() != shape.logical_columns()? {
            return Err(Error::InvalidNumberOfVariables);
        }
        let encoded_shape =
            ShortRaaPackedShape::new(self.encoded_rows, self.columns, shape.element_bits())?;
        let fold_row = |row: usize| {
            column_weights
                .iter()
                .enumerate()
                .map(|(logical_column, weight)| {
                    let value = packed_short_value_row_major(
                        &self.encoded_entries,
                        encoded_shape,
                        row,
                        logical_column,
                    )?;
                    Ok(BinaryExtensionField::from(value) * *weight)
                })
                .try_fold(BinaryExtensionField::ZERO, |sum, term| {
                    term.map(|term| sum + term)
                })
        };
        #[cfg(feature = "parallel")]
        let folded = (0..self.encoded_rows)
            .into_par_iter()
            .map(fold_row)
            .collect::<Result<Vec<_>, Error>>()?;
        #[cfg(not(feature = "parallel"))]
        let folded = (0..self.encoded_rows)
            .map(fold_row)
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(folded)
    }
}

fn suffix_accumulate<W: RaaPackedWord>(values: &mut [W]) {
    let mut running = W::zero();
    for value in values.iter_mut().rev() {
        running = running.add(*value);
        *value = running;
    }
}

fn apply_permutation_transpose<W: RaaPackedWord>(
    input: &[W],
    permutation: &[u32],
    output: &mut [W],
) {
    debug_assert_eq!(input.len(), permutation.len());
    debug_assert_eq!(output.len(), permutation.len());
    output.fill(W::zero());
    for (value, source) in input.iter().zip(permutation) {
        output[*source as usize] = *value;
    }
}

fn build_short_raa_tree<W: RaaPackedWord>(
    encoded_entries: &[W],
    encoded_rows: usize,
    columns: usize,
) -> Result<ShortRaaMerkleTree, Error> {
    if encoded_rows == 0
        || !encoded_rows.is_power_of_two()
        || columns == 0
        || encoded_entries.len()
            != encoded_rows
                .checked_mul(columns)
                .ok_or(Error::EncodingError)?
    {
        return Err(Error::EncodingError);
    }
    #[cfg(feature = "parallel")]
    let leaves = (0..encoded_rows)
        .into_par_iter()
        .map(|row| hash_short_raa_encoded_row::<W>(encoded_entries, encoded_rows, columns, row))
        .collect::<Result<Vec<_>, Error>>()?;
    #[cfg(not(feature = "parallel"))]
    let leaves = (0..encoded_rows)
        .map(|row| hash_short_raa_encoded_row::<W>(encoded_entries, encoded_rows, columns, row))
        .collect::<Result<Vec<_>, Error>>()?;
    ShortRaaMerkleTree::from_leaves(leaves)
}

fn hash_short_raa_encoded_row<W: RaaPackedWord>(
    encoded_entries: &[W],
    encoded_rows: usize,
    columns: usize,
    row: usize,
) -> Result<[u8; DIGEST_BYTES], Error> {
    if row >= encoded_rows
        || encoded_entries.len()
            != encoded_rows
                .checked_mul(columns)
                .ok_or(Error::EncodingError)?
    {
        return Err(Error::EncodingError);
    }
    let mut hasher = Blake2s256::new();
    absorb_short_raa_frame(&mut hasher, b"domain", SHORT_RAA_LEAF_DOMAIN);
    for &word in &encoded_entries[row * columns..(row + 1) * columns] {
        update_short_raa_word::<W>(&mut hasher, word);
    }
    Ok(hasher.finalize().into())
}

fn hash_short_raa_row<W: RaaPackedWord>(
    row: &[W],
    columns: usize,
) -> Result<[u8; DIGEST_BYTES], Error> {
    if row.len() != columns {
        return Err(Error::EncodingError);
    }
    let mut hasher = Blake2s256::new();
    absorb_short_raa_frame(&mut hasher, b"domain", SHORT_RAA_LEAF_DOMAIN);
    row.iter()
        .copied()
        .for_each(|word| update_short_raa_word::<W>(&mut hasher, word));
    Ok(hasher.finalize().into())
}

fn hash_short_raa_root<W: RaaPackedWord>(
    tree_root: &[u8; DIGEST_BYTES],
    encoded_rows: usize,
    columns: usize,
    logical_element_bits: usize,
    logical_elements: usize,
    generator_digest: &[u8; DIGEST_BYTES],
) -> Result<[u8; DIGEST_BYTES], Error> {
    let mut hasher = Blake2s256::new();
    absorb_short_raa_frame(&mut hasher, b"domain", SHORT_RAA_ROOT_DOMAIN);
    absorb_short_raa_frame(&mut hasher, b"generator", generator_digest);
    absorb_short_raa_usize(&mut hasher, b"word-bits", W::BITS)?;
    absorb_short_raa_usize(&mut hasher, b"logical-element-bits", logical_element_bits)?;
    absorb_short_raa_usize(&mut hasher, b"logical-elements", logical_elements)?;
    absorb_short_raa_usize(&mut hasher, b"encoded-rows", encoded_rows)?;
    absorb_short_raa_usize(&mut hasher, b"columns", columns)?;
    absorb_short_raa_frame(&mut hasher, b"tree-root", tree_root);
    Ok(hasher.finalize().into())
}

fn valid_logical_element_bits<W: RaaPackedWord>(logical_element_bits: usize) -> bool {
    logical_element_bits > 0
        && logical_element_bits <= W::BITS
        && logical_element_bits.is_power_of_two()
        && W::BITS % logical_element_bits == 0
}

fn valid_short_element_bits(element_bits: usize) -> bool {
    matches!(element_bits, 1 | 2 | 4 | 8 | 16 | 32 | 64 | 128)
}

fn short_value_mask(element_bits: usize) -> u128 {
    if element_bits == u128::BITS as usize {
        u128::MAX
    } else {
        (1u128 << element_bits) - 1
    }
}

fn packed_short_value(
    words: &[u128],
    shape: ShortRaaPackedShape,
    row: usize,
    logical_column: usize,
) -> Result<u128, Error> {
    let logical_columns = shape.logical_columns()?;
    if row >= shape.message_rows || logical_column >= logical_columns {
        return Err(Error::EncodingError);
    }
    let values_per_word = shape.values_per_word();
    let packed_column = logical_column / values_per_word;
    let lane = logical_column % values_per_word;
    let word = *words
        .get(packed_column * shape.message_rows + row)
        .ok_or(Error::EncodingError)?;
    Ok((word >> (lane * shape.element_bits)) & short_value_mask(shape.element_bits))
}

fn packed_short_value_row_major(
    words: &[u128],
    shape: ShortRaaPackedShape,
    row: usize,
    logical_column: usize,
) -> Result<u128, Error> {
    let logical_columns = shape.logical_columns()?;
    if row >= shape.message_rows || logical_column >= logical_columns {
        return Err(Error::EncodingError);
    }
    let values_per_word = shape.values_per_word();
    let packed_column = logical_column / values_per_word;
    let lane = logical_column % values_per_word;
    let word = *words
        .get(row * shape.packed_columns + packed_column)
        .ok_or(Error::EncodingError)?;
    Ok((word >> (lane * shape.element_bits)) & short_value_mask(shape.element_bits))
}

fn update_short_raa_word<W: RaaPackedWord>(hasher: &mut Blake2s256, word: W) {
    let bytes = word.to_u128().to_le_bytes();
    hasher.update(&bytes[..W::BITS / 8]);
}

fn packed_encoded_row<W: RaaPackedWord>(
    encoded_entries: &[W],
    encoded_rows: usize,
    columns: usize,
    row: usize,
) -> Result<Vec<W>, Error> {
    if row >= encoded_rows
        || encoded_entries.len()
            != encoded_rows
                .checked_mul(columns)
                .ok_or(Error::EncodingError)?
    {
        return Err(Error::EncodingError);
    }
    Ok(encoded_entries[row * columns..(row + 1) * columns].to_vec())
}

fn hash_short_raa_inner(
    left: &[u8; DIGEST_BYTES],
    right: &[u8; DIGEST_BYTES],
) -> [u8; DIGEST_BYTES] {
    let mut hasher = Blake2s256::new();
    absorb_short_raa_frame(&mut hasher, b"domain", SHORT_RAA_INNER_DOMAIN);
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().into()
}

fn short_raa_merkle_root_from_path(
    mut digest: [u8; DIGEST_BYTES],
    mut index: usize,
    leaf_count: usize,
    path: &ShortRaaMerklePath,
) -> Option<[u8; DIGEST_BYTES]> {
    if leaf_count == 0
        || !leaf_count.is_power_of_two()
        || index >= leaf_count
        || path.siblings.len() != leaf_count.ilog2() as usize
    {
        return None;
    }
    for sibling in &path.siblings {
        digest = if index & 1 == 0 {
            hash_short_raa_inner(&digest, sibling)
        } else {
            hash_short_raa_inner(sibling, &digest)
        };
        index >>= 1;
    }
    Some(digest)
}

fn absorb_short_raa_frame(hasher: &mut Blake2s256, label: &[u8], bytes: &[u8]) {
    hasher.update((label.len() as u64).to_le_bytes());
    hasher.update(label);
    hasher.update((bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
}

fn absorb_short_raa_usize(
    hasher: &mut Blake2s256,
    label: &[u8],
    value: usize,
) -> Result<(), Error> {
    let value = u64::try_from(value).map_err(|_| Error::EncodingError)?;
    absorb_short_raa_frame(hasher, label, &value.to_le_bytes());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::rand::{rngs::StdRng, SeedableRng};

    fn random_field(rng: &mut StdRng, index: usize) -> BinaryExtensionField {
        match index % 11 {
            0 => BinaryExtensionField::ZERO,
            1 => BinaryExtensionField::ONE,
            _ => BinaryExtensionField::from(
                u128::from(rng.next_u64()) | (u128::from(rng.next_u64()) << 64),
            ),
        }
    }

    #[test]
    fn full_width_projection_and_tiled_fold_match_scalar_field_arithmetic() {
        let mut rng = StdRng::seed_from_u64(0x7261_612d_666f_6c64);
        // Cover scalar/SIMD tails and both sides of the 256-row tile boundary.
        for rows in [1, 2, 4, 8, 128, 256, 512] {
            for columns in [1, 4, 128] {
                let code = ShortRaaCode::<u128>::sample(
                    rows,
                    4,
                    RaaDistanceProfile::blaze_assumed(4).unwrap(),
                    &mut rng,
                )
                .unwrap();
                let values = (0..rows * columns)
                    .map(|i| random_field(&mut rng, i))
                    .collect::<Vec<_>>();
                let row_weights = (0..rows)
                    .map(|i| random_field(&mut rng, i))
                    .collect::<Vec<_>>();
                let column_weights = (0..columns)
                    .map(|i| random_field(&mut rng, i + 1))
                    .collect::<Vec<_>>();
                let (commitment, cache) = code
                    .commit_owned(values.iter().map(|value| value.val()).collect(), columns)
                    .unwrap();

                let expected_projection = values
                    .chunks_exact(rows)
                    .map(|column| {
                        column
                            .iter()
                            .zip(&row_weights)
                            .fold(BinaryExtensionField::ZERO, |acc, (&value, &weight)| {
                                acc + value * weight
                            })
                    })
                    .collect::<Vec<_>>();
                let expected_fold = (0..rows)
                    .map(|row| {
                        (0..columns).fold(BinaryExtensionField::ZERO, |acc, column| {
                            acc + values[column * rows + row] * column_weights[column]
                        })
                    })
                    .collect::<Vec<_>>();
                assert_eq!(
                    cache
                        .message_row_projection(&commitment, rows, &row_weights)
                        .unwrap(),
                    expected_projection,
                    "projection rows={rows}, columns={columns}",
                );
                assert_eq!(
                    cache
                        .fold_message_to_extension(&commitment, rows, &column_weights)
                        .unwrap(),
                    expected_fold,
                    "fold rows={rows}, columns={columns}",
                );
                // The two contractions commute and preserve the same claim.
                let projected_claim = expected_projection
                    .iter()
                    .zip(&column_weights)
                    .fold(BinaryExtensionField::ZERO, |acc, (&value, &weight)| {
                        acc + value * weight
                    });
                let folded_claim = expected_fold
                    .iter()
                    .zip(&row_weights)
                    .fold(BinaryExtensionField::ZERO, |acc, (&value, &weight)| {
                        acc + value * weight
                    });
                assert_eq!(projected_claim, folded_claim);
            }
        }
    }

    #[test]
    fn full_width_fast_paths_reject_mismatched_dimensions() {
        let mut rng = StdRng::seed_from_u64(0x6469_6d65_6e73_696f);
        let code = ShortRaaCode::<u128>::sample(
            8,
            4,
            RaaDistanceProfile::blaze_assumed(4).unwrap(),
            &mut rng,
        )
        .unwrap();
        let (commitment, cache) = code.commit_owned(vec![0; 32], 4).unwrap();
        assert!(cache
            .message_row_projection(&commitment, 8, &[BinaryExtensionField::ONE; 7])
            .is_err());
        assert!(cache
            .fold_message_to_extension(&commitment, 8, &[BinaryExtensionField::ONE; 3])
            .is_err());
        assert!(cache
            .message_row_projection(&commitment, 4, &[BinaryExtensionField::ONE; 4])
            .is_err());
        assert!(cache
            .fold_message_to_extension(&commitment, 4, &[BinaryExtensionField::ONE; 4])
            .is_err());
    }
}
