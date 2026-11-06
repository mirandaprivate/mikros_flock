//! End-to-end polynomial commitment for packed short witnesses and RAAFold.
//!
//! The committed witness contains logical values of width 1, 2, 4, 8, 16,
//! 32, 64, or 128 bits packed into column-major `u128` words. Encoding the
//! top commitment uses only XOR. Opening arithmetic starts by unpacking each
//! logical value and embedding it into `GF(2^128)`; all subsequent folding,
//! recursive commitments, and generator reductions stay in that field.
//!
//! This implementation provides a complete non-hiding PCS proof. The opening
//! recursively folds the witness through the configured RAA code sequence
//! until the recursive witness has fewer than `2^17` field elements. It then
//! uses an unpadded rectangular Ligero-style tail whose height-to-width ratio
//! is the closest exact factorization to the security parameter.
//! Every non-terminal generator claim is reduced through two semantic
//! permutation lookups batched into one grand-product argument. The fixed
//! permutation-index columns form `H_j`. The claim-dependent pushforward and
//! prefix-product state matrices form two sequential `S_j` blocks. Setup `H_j`
//! and both dynamic blocks are encoded directly by the next code `C_{j-1}` and
//! authenticated on the same SwitchFold rows. No Reed--Solomon, FRI, or `T_j`
//! commitment is used in the generator reduction.

use ark_std::{rand::RngCore, vec, vec::Vec};
use binius_field::{Field, WideMul, util::FieldFn};
use binius_iop::channel::{
    Error as BiniusIopError, IOPVerifierChannel, OracleLinearRelation, OracleSpec,
};
use binius_iop_prover::channel::IOPProverChannel;
use binius_ip::channel::{Error as BiniusIpError, IPVerifierChannel};
use binius_ip_prover::channel::IPProverChannel;
use binius_math::{FieldBuffer, FieldSlice, multilinear::eq::eq_ind_partial_eval_scalars};
use blake2::Blake2s256;
use digest::Digest;
#[cfg(feature = "parallel")]
use rayon::prelude::*;
use std::time::Instant;

use crate::Error;

use crate::{
    raafold::{
        BinaryExtensionField, PackedBinaryExtensionField, RaaCode, RaaDirectInterleavedSetupShape,
        RaaDistanceProfile, RaaFold, RaaGeneratorReduction, RaaMatrixClaim, RaaScaledMatrixClaim,
        multilinear_inner_product_round,
    },
    raafold_short::{
        ShortRaaCode, ShortRaaFold, ShortRaaFoldCommitment, ShortRaaFoldCommitmentCache,
        ShortRaaMerklePath, ShortRaaPackedShape, ShortRaaPackedWitness, ShortRaaRowQuery,
    },
};

const DIGEST_BYTES: usize = 32;
const FIELD_BYTES: usize = 16;
const PROOF_MAGIC: &[u8; 8] = b"MRASPC01";
const TRANSCRIPT_DOMAIN: &[u8] = b"mikros/raafold-blaze/boolean-mle-opening/v1";
const DECIDE_TRANSCRIPT_DOMAIN: &[u8] = b"brakefold/raafold-short-pc/decide/v4";
const EXTENSION_LEAF_DOMAIN: &[u8] = b"brakefold/raafold-short-pc/extension-row/v1";
const INNER_DOMAIN: &[u8] = b"brakefold/raafold-short-pc/merkle-inner/v1";
const LIGERO_TAIL_DOMAIN: &[u8] = b"brakefold/raafold-short-pc/ligero-tail/v1";
const PARAMETER_DOMAIN: &[u8] = b"mikros/raafold-blaze/parameters/v1";
const GENERATOR_STATEMENT_DOMAIN: u128 = 0x7261_6166_6f6c_642d_7368_6f72_742d_7063;
/// Recursive vectors smaller than this many field elements use the Ligero tail.
pub const RAAFOLD_LIGERO_THRESHOLD: usize = 1 << 17;
const PARALLEL_FIELD_THRESHOLD: usize = 1 << 12;

type BinaryWideProduct = <BinaryExtensionField as WideMul>::Output;

/// Public metadata for one setup commitment to `C_{j-1}(H_j)`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShortRaaSetupLevel {
    shape: RaaDirectInterleavedSetupShape,
    root: [u8; DIGEST_BYTES],
}

impl ShortRaaSetupLevel {
    /// Returns the direct-interleaved shape committed at this level.
    pub fn shape(self) -> RaaDirectInterleavedSetupShape {
        self.shape
    }

    /// Returns the Merkle root committing to the encoded fixed `H_j` rows.
    pub fn root(self) -> [u8; DIGEST_BYTES] {
        self.root
    }
}

/// Public parameters for one fixed packed-witness shape.
#[derive(Clone, Debug)]
pub struct ShortRaaFoldParameters {
    sequence: ShortRaaFold<u128>,
    extension_sequence: RaaFold,
    n_witness: usize,
    packed_columns: usize,
    logical_element_bits: usize,
    security_bits: usize,
    terminal_level: usize,
    opening_tail: ShortRaaLigeroTailParameters,
    decide_tail: ShortRaaLigeroTailParameters,
    setup_levels: Vec<Option<ShortRaaSetupLevel>>,
    setup_digest: [u8; DIGEST_BYTES],
    parameter_digest: [u8; DIGEST_BYTES],
}

/// Public parameters for one rectangular Ligero-style RAA tail.
#[derive(Clone, Debug)]
pub struct ShortRaaLigeroTailParameters {
    witness_len: usize,
    height: usize,
    width: usize,
    code: RaaCode,
}

impl ShortRaaLigeroTailParameters {
    /// Returns the number of field elements in the terminal witness.
    pub fn witness_len(&self) -> usize {
        self.witness_len
    }

    /// Returns the unencoded matrix height.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the unencoded matrix width.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the actual dyadic height-to-width ratio.
    pub fn aspect_ratio(&self) -> usize {
        self.height / self.width
    }

    /// Returns the rate-1/4 RAA code used to encode every matrix column.
    pub fn code(&self) -> &RaaCode {
        &self.code
    }
}

/// Prover-only caches for opening the setup commitments to fixed `H_j`.
///
/// Verification intentionally does not accept this object. A verifier needs
/// only the setup shapes and roots stored in [`ShortRaaFoldParameters`].
#[derive(Debug)]
pub struct ShortRaaFoldProverSetup {
    setup_caches: Vec<Option<ExtensionCache>>,
    parameter_digest: [u8; DIGEST_BYTES],
}

impl ShortRaaFoldParameters {
    /// Returns the packed RAA code sequence.
    pub fn sequence(&self) -> &ShortRaaFold<u128> {
        &self.sequence
    }

    /// Returns the number of logical short witness elements.
    pub fn n_witness(&self) -> usize {
        self.n_witness
    }

    /// Returns the number of physical packed columns.
    pub fn packed_columns(&self) -> usize {
        self.packed_columns
    }

    /// Returns the logical witness element width.
    pub fn logical_element_bits(&self) -> usize {
        self.logical_element_bits
    }

    /// Returns the target soundness in bits.
    pub fn security_bits(&self) -> usize {
        self.security_bits
    }

    /// Returns the setup root committing to `H_j` under `C_{j-1}`.
    pub fn setup_root(&self, level: usize) -> Result<[u8; DIGEST_BYTES], Error> {
        Ok(self.setup_level(level)?.root)
    }

    /// Returns the direct-interleaved shape committed at one Decide level.
    pub fn setup_shape(&self, level: usize) -> Result<RaaDirectInterleavedSetupShape, Error> {
        Ok(self.setup_level(level)?.shape)
    }

    /// Returns the digest binding the complete public parameter set.
    pub fn parameter_digest(&self) -> [u8; DIGEST_BYTES] {
        self.parameter_digest
    }

    /// Returns the lowest RAA level processed before the Ligero tail.
    pub fn terminal_level(&self) -> usize {
        self.terminal_level
    }

    /// Returns the Ligero tail used by recursive opening.
    pub fn opening_tail(&self) -> &ShortRaaLigeroTailParameters {
        &self.opening_tail
    }

    /// Returns the Ligero tail used by recursive Decide.
    pub fn decide_tail(&self) -> &ShortRaaLigeroTailParameters {
        &self.decide_tail
    }

    /// Returns the public shape and root for one non-base setup level.
    pub fn setup_level(&self, level: usize) -> Result<&ShortRaaSetupLevel, Error> {
        self.setup_levels
            .get(level)
            .and_then(Option::as_ref)
            .ok_or(Error::EncodingError)
    }
}

impl ShortRaaFoldProverSetup {
    fn setup_cache(&self, level: usize) -> Result<&ExtensionCache, Error> {
        self.setup_caches
            .get(level)
            .and_then(Option::as_ref)
            .ok_or(Error::EncodingError)
    }
}

/// One authenticated extension-field row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortRaaExtensionRowQuery {
    /// Complete row across all extension-field columns.
    pub row: Vec<BinaryExtensionField>,
    /// BLAKE2s authentication path.
    pub path: ShortRaaMerklePath,
}

/// One recursive extension-field opening round.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortRaaExtensionRound {
    /// Commitment root for this recursive witness matrix.
    pub root: [u8; DIGEST_BYTES],
    /// Rows selected by the Fiat--Shamir transcript, in selection order.
    pub rows: Vec<ShortRaaExtensionRowQuery>,
}

/// One SwitchFold reduction for a deferred RAA generator claim.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortRaaGeneratorProof {
    /// RAA sequence level of the proved generator.
    pub level: usize,
    /// Roots of the sequentially committed dynamic `S_j` blocks.
    pub auxiliary_roots: Vec<[u8; DIGEST_BYTES]>,
}

/// One recursive Decide round authenticating setup and carried codeword rows.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortRaaDecideRound {
    /// Source level `j` whose fixed `H_j` claims are being opened.
    pub level: usize,
    /// Authenticated rows from the setup commitment to `C_{j-1}(H_j)`.
    pub setup_rows: Vec<ShortRaaExtensionRowQuery>,
    /// Authenticated rows from every dynamic `C_{j-1}(S_j)` commitment.
    pub auxiliary_rows: Vec<Vec<ShortRaaExtensionRowQuery>>,
    /// Authenticated rows from the optional carried witness commitment.
    pub carry_rows: Option<Vec<ShortRaaExtensionRowQuery>>,
    /// Root of the carried witness for the next Decide level.
    pub next_carry_root: Option<[u8; DIGEST_BYTES]>,
}

/// Recursive Decide proof for all generator claims left by the opening.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortRaaDecideProof {
    /// SwitchFold generator reductions in transcript order.
    pub generator_proofs: Vec<ShortRaaGeneratorProof>,
    /// Scalar responses for multi-evaluation and direct code-switch reductions.
    responses: Vec<BinaryExtensionField>,
    /// Rounds ordered from the largest code to the configured terminal level.
    pub rounds: Vec<ShortRaaDecideRound>,
    /// Ligero-style proof for the final carried vector.
    tail: ShortRaaLigeroTailProof,
}

impl ShortRaaDecideProof {
    /// Returns the number of RAA generator reductions in Decide.
    pub fn generator_proof_count(&self) -> usize {
        self.generator_proofs.len()
    }
}

/// Opening proof before fixed-generator claims are discharged.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortRaaOpeningProof {
    responses: Vec<BinaryExtensionField>,
    top_rows: Vec<ShortRaaRowQuery<u128>>,
    extension_rounds: Vec<ShortRaaExtensionRound>,
    tail: ShortRaaLigeroTailProof,
}

/// One rectangular Ligero-style terminal opening.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortRaaLigeroTailProof {
    /// Merkle root of the row commitment to the column-wise RAA encoding.
    pub root: [u8; DIGEST_BYTES],
    /// Queried encoded rows and their Merkle authentication paths.
    pub rows: Vec<ShortRaaExtensionRowQuery>,
}

/// Complete end-to-end proof: recursive opening plus SwitchFold generator reductions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortRaaFoldProof {
    /// Recursive packed-to-extension opening proof.
    pub opening: ShortRaaOpeningProof,
    /// Setup-committed recursive Decide proof.
    pub decide: ShortRaaDecideProof,
}

/// One generator claim deferred by the recursive opening verifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortRaaPendingGeneratorClaim {
    /// RAA sequence level.
    pub level: usize,
    /// Claim in Binius' equality-indicator MLE convention.
    pub claim: RaaMatrixClaim<BinaryExtensionField>,
}

/// Structured result returned by opening verification.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShortRaaOpeningReduction {
    /// Whether all opening, query, Merkle, and terminal checks passed.
    pub ok: bool,
    /// Non-base generator claims consumed by Decide.
    pub pending_generator_claims: Vec<ShortRaaPendingGeneratorClaim>,
}

/// End-to-end packed short-witness RAAFold polynomial commitment.
#[derive(Clone, Copy, Debug)]
pub struct ShortRaaFoldPC {
    n_witness: usize,
    packed_columns: usize,
    logical_element_bits: usize,
    code_switch_ratio: usize,
    security_bits: usize,
}

fn ligero_tail_dimensions(witness_len: usize, lambda: usize) -> Result<(usize, usize), Error> {
    if witness_len == 0 || !witness_len.is_power_of_two() || lambda == 0 {
        return Err(Error::EncodingError);
    }
    let log_witness = witness_len.ilog2() as usize;
    let target_log_ratio = (lambda as f64).log2();
    let row_log = ((log_witness + 1) / 2..=log_witness)
        .min_by(|&left, &right| {
            let left_log_ratio = (2 * left - log_witness) as f64;
            let right_log_ratio = (2 * right - log_witness) as f64;
            (left_log_ratio - target_log_ratio)
                .abs()
                .total_cmp(&(right_log_ratio - target_log_ratio).abs())
        })
        .ok_or(Error::EncodingError)?;
    let height = 1usize
        .checked_shl(row_log as u32)
        .ok_or(Error::EncodingError)?;
    let width = witness_len / height;
    if height.checked_mul(width) != Some(witness_len) || height < width {
        return Err(Error::EncodingError);
    }
    Ok((height, width))
}

fn sample_ligero_tail<R: RngCore>(
    witness_len: usize,
    lambda: usize,
    rng: &mut R,
) -> Result<ShortRaaLigeroTailParameters, Error> {
    let (height, width) = ligero_tail_dimensions(witness_len, lambda)?;
    let rate_inv = crate::raafold::RAAFOLD_RECURSIVE_RATE_INV;
    let code = RaaCode::sample(
        height,
        rate_inv,
        RaaDistanceProfile::blaze_assumed(rate_inv)?,
        rng,
    )?;
    Ok(ShortRaaLigeroTailParameters {
        witness_len,
        height,
        width,
        code,
    })
}

impl ShortRaaFoldPC {
    /// Creates a PCS for a fixed logical witness and physical packing shape.
    ///
    /// `packed_columns` is the top interleaving from the original witness into
    /// `C_d`. `code_switch_ratio` applies to every transition from `C_j` to
    /// `C_{j-1}`.
    pub fn new(
        n_witness: usize,
        packed_columns: usize,
        logical_element_bits: usize,
        code_switch_ratio: usize,
        security_bits: usize,
    ) -> Result<Self, Error> {
        if n_witness == 0
            || !n_witness.is_power_of_two()
            || packed_columns == 0
            || !packed_columns.is_power_of_two()
            || !matches!(logical_element_bits, 1 | 2 | 4 | 8 | 16 | 32 | 64 | 128)
            || code_switch_ratio <= 1
            || !code_switch_ratio.is_power_of_two()
            || security_bits == 0
        {
            return Err(Error::EncodingError);
        }
        let witness_bits = n_witness
            .checked_mul(logical_element_bits)
            .ok_or(Error::EncodingError)?;
        let packed_words = witness_bits / 128;
        if witness_bits % 128 != 0 || packed_words % packed_columns != 0 {
            return Err(Error::EncodingError);
        }
        let message_rows = packed_words / packed_columns;
        if !message_rows.is_power_of_two() || message_rows < code_switch_ratio {
            return Err(Error::EncodingError);
        }
        Ok(Self {
            n_witness,
            packed_columns,
            logical_element_bits,
            code_switch_ratio,
            security_bits,
        })
    }

    /// Samples the code sequence and builds public and prover-only setup data.
    pub fn setup<R: RngCore>(
        &self,
        rng: &mut R,
    ) -> Result<(ShortRaaFoldParameters, ShortRaaFoldProverSetup), Error> {
        let packed_words = self.n_witness * self.logical_element_bits / 128;
        let top_rows = packed_words / self.packed_columns;
        let mut descending = Vec::new();
        let mut rows = top_rows;
        loop {
            descending.push(rows);
            if rows < self.code_switch_ratio {
                break;
            }
            if rows % self.code_switch_ratio != 0 {
                return Err(Error::EncodingError);
            }
            rows /= self.code_switch_ratio;
        }
        descending.reverse();
        if descending.len() < 2 {
            return Err(Error::EncodingError);
        }
        let levels = descending.len();
        let codes = descending
            .into_iter()
            .enumerate()
            .map(|(level, message_len)| {
                let rate_inv = crate::raafold::raafold_rate_inv_for_level(level, levels);
                ShortRaaCode::sample(
                    message_len,
                    rate_inv,
                    RaaDistanceProfile::blaze_assumed(rate_inv)?,
                    rng,
                )
            })
            .collect::<Result<Vec<_>, Error>>()?;
        let sequence = ShortRaaFold::new(codes, self.code_switch_ratio)?;
        let extension_sequence = RaaFold::new(
            sequence
                .codes()
                .iter()
                .map(|code| code.extension_code().clone())
                .collect(),
            self.code_switch_ratio,
        )?;
        let terminal_level = (0..extension_sequence.levels())
            .rev()
            .find(|&level| {
                extension_sequence
                    .code(level)
                    .is_ok_and(|code| code.message_len() < RAAFOLD_LIGERO_THRESHOLD)
            })
            .ok_or(Error::EncodingError)?;
        let decide_level = terminal_level.checked_sub(1).ok_or(Error::EncodingError)?;
        let opening_tail = sample_ligero_tail(
            extension_sequence.code(terminal_level)?.message_len(),
            self.security_bits,
            rng,
        )?;
        let decide_tail = sample_ligero_tail(
            extension_sequence.code(decide_level)?.message_len(),
            self.security_bits,
            rng,
        )?;
        let (setup_levels, setup_caches) = build_setup_levels(&extension_sequence)?;
        let setup_digest = setup_commitment_digest(&setup_levels)?;
        let parameter_digest = parameter_digest(
            self,
            &sequence,
            terminal_level,
            &opening_tail,
            &decide_tail,
            &setup_digest,
        )?;
        Ok((
            ShortRaaFoldParameters {
                sequence,
                extension_sequence,
                n_witness: self.n_witness,
                packed_columns: self.packed_columns,
                logical_element_bits: self.logical_element_bits,
                security_bits: self.security_bits,
                terminal_level,
                opening_tail,
                decide_tail,
                setup_levels,
                setup_digest,
                parameter_digest,
            },
            ShortRaaFoldProverSetup {
                setup_caches,
                parameter_digest,
            },
        ))
    }

    /// Packs and commits logical values supplied in column-major order.
    pub fn commit_values(
        &self,
        pp: &ShortRaaFoldParameters,
        values: &[u128],
    ) -> Result<(ShortRaaFoldCommitment, ShortRaaFoldCommitmentCache<u128>), Error> {
        validate_parameters(self, pp)?;
        let shape = ShortRaaPackedShape::new(
            pp.sequence.top_code()?.message_len(),
            self.packed_columns,
            self.logical_element_bits,
        )?;
        if values.len() != self.n_witness {
            return Err(Error::EncodingError);
        }
        pp.sequence
            .commit_short_witness_owned(ShortRaaPackedWitness::from_column_major_values(
                shape, values,
            )?)
    }

    /// Commits a pre-packed witness after validating its logical shape.
    pub fn commit_packed(
        &self,
        pp: &ShortRaaFoldParameters,
        witness: ShortRaaPackedWitness,
    ) -> Result<(ShortRaaFoldCommitment, ShortRaaFoldCommitmentCache<u128>), Error> {
        validate_parameters(self, pp)?;
        let shape = witness.shape();
        if shape.logical_elements()? != self.n_witness
            || shape.packed_columns() != self.packed_columns
            || shape.element_bits() != self.logical_element_bits
        {
            return Err(Error::EncodingError);
        }
        pp.sequence.commit_short_witness_owned(witness)
    }

    /// Produces a complete Boolean-MLE opening and Decide proof.
    ///
    /// Coordinates are MSB-first over the flat logical values supplied to
    /// `commit_values`: the column coordinates precede the row coordinates.
    /// A Boolean point selects exactly one element, including at zero and one.
    pub fn open(
        &self,
        pp: &ShortRaaFoldParameters,
        prover_setup: &ShortRaaFoldProverSetup,
        commitment: &ShortRaaFoldCommitment,
        point: &[BinaryExtensionField],
        cache: ShortRaaFoldCommitmentCache<u128>,
    ) -> Result<(BinaryExtensionField, ShortRaaFoldProof), Error> {
        let profile_stages = std::env::var_os("RAAFOLD_PROFILE_STAGES").is_some();
        let total_start = Instant::now();
        let (value, opening, reduction) = self.open_with_reduction(pp, commitment, point, cache)?;
        let opening_time = total_start.elapsed();
        let decide_start = Instant::now();
        let decide = self.prove_decide(pp, prover_setup, commitment, &reduction)?;
        if profile_stages {
            eprintln!(
                "[RAAFOLD_PROFILE] open elements={} bits={} opening_s={:.9} decide_s={:.9} total_s={:.9}",
                self.n_witness,
                self.logical_element_bits,
                opening_time.as_secs_f64(),
                decide_start.elapsed().as_secs_f64(),
                total_start.elapsed().as_secs_f64(),
            );
        }
        Ok((value, ShortRaaFoldProof { opening, decide }))
    }

    /// Verifies the Boolean-MLE opening and all native RAA reductions.
    pub fn verify(
        &self,
        pp: &ShortRaaFoldParameters,
        commitment: &ShortRaaFoldCommitment,
        point: &[BinaryExtensionField],
        value: BinaryExtensionField,
        proof: &ShortRaaFoldProof,
    ) -> Result<bool, Error> {
        let reduction =
            self.verify_opening_reduction(pp, commitment, point, value, &proof.opening)?;
        if !reduction.ok {
            return Ok(false);
        }
        self.verify_decide(pp, commitment, &reduction, &proof.decide)
    }

    /// Produces the recursive opening and returns its deferred generator claims.
    pub fn open_with_reduction(
        &self,
        pp: &ShortRaaFoldParameters,
        commitment: &ShortRaaFoldCommitment,
        point: &[BinaryExtensionField],
        cache: ShortRaaFoldCommitmentCache<u128>,
    ) -> Result<
        (
            BinaryExtensionField,
            ShortRaaOpeningProof,
            ShortRaaOpeningReduction,
        ),
        Error,
    > {
        let profile_stages = std::env::var_os("RAAFOLD_PROFILE_STAGES").is_some();
        let opening_start = Instant::now();
        validate_parameters(self, pp)?;
        if point.len() != self.n_witness.ilog2() as usize
            || commitment.logical_elements() != self.n_witness
            || commitment.logical_element_bits() != self.logical_element_bits
            || commitment.columns() != self.packed_columns
            || commitment.generator_digest() != pp.sequence.generator_digest()
        {
            return Err(Error::InvalidNumberOfVariables);
        }
        let top_level = pp.sequence.levels() - 1;
        let top = pp.sequence.code(top_level)?;
        let shape = cache.short_witness_shape(commitment, top.message_len())?;
        let logical_columns = shape.logical_columns()?;
        let column_log = logical_columns.ilog2() as usize;
        if point.len() != column_log + top.message_len().ilog2() as usize {
            return Err(Error::InvalidNumberOfVariables);
        }
        let projection_start = Instant::now();
        let row_weights = equality_tensor(&point[column_log..]);
        let opening_vector =
            cache.message_row_projection(commitment, top.message_len(), &row_weights)?;
        let value = inner_product(&opening_vector, &equality_tensor(&point[..column_log]))?;
        let projection_time = projection_start.elapsed();

        let mut transcript = OpeningTranscriptWriter::new();
        absorb_opening_statement_writer(pp, commitment, point, value, &mut transcript);

        transcript.absorb_digest(b"round-root", &commitment.root());
        for opening in &opening_vector {
            transcript.push_response(*opening);
        }
        let top_freivalds = (0..logical_columns)
            .map(|_| transcript.challenge_non_singular(b"freivalds"))
            .collect::<Result<Vec<_>, _>>()?;
        let freivalds_value = inner_product(&opening_vector, &top_freivalds)?;
        let fold_start = Instant::now();
        let folded =
            cache.fold_message_to_extension(commitment, top.message_len(), &top_freivalds)?;
        let fold_time = fold_start.elapsed();
        let terminal_level = pp.terminal_level;
        let should_tail = folded.len() < RAAFOLD_LIGERO_THRESHOLD;
        if terminal_level > top_level || should_tail != (top_level == terminal_level) {
            return Err(Error::InvalidCommitment);
        }
        let prepare_start = Instant::now();
        let (next_binding, mut prepared) = if top_level == terminal_level {
            prepare_ligero_tail(&pp.opening_tail, &folded)?
        } else {
            let top_switch_ratio = pp
                .extension_sequence
                .switch_ratio_for_source_level(top_level)?;
            prepare_next_layer(
                &pp.extension_sequence,
                top_level.checked_sub(1),
                top_switch_ratio,
                &folded,
            )?
        };
        let prepare_time = prepare_start.elapsed();
        absorb_next_binding_writer(&mut transcript, next_binding);
        let rows_start = Instant::now();
        let selected = derive_selection_writer(
            top.extension_code(),
            pp.security_bits,
            commitment.encoded_rows(),
            &mut transcript,
        )?;
        let top_rows = cache.open_rows(&selected)?;
        let rows_time = rows_start.elapsed();
        // All queried rows and Merkle paths are now owned by the proof. The
        // full top witness/codeword cache is no longer needed by the reduction.
        let release_start = Instant::now();
        drop(cache);
        let release_time = release_start.elapsed();
        let sampled = top_rows
            .iter()
            .map(|query| {
                fold_packed_query_row(&query.row, self.logical_element_bits, &top_freivalds)
            })
            .collect::<Result<Vec<_>, Error>>()?;
        transcript.absorb_extensions(b"sampled-values", &sampled);
        let reduction_start = Instant::now();
        let top_reduction = prove_round_reduction(
            top.extension_code(),
            point,
            OpeningPointBasis::Equality,
            logical_columns,
            freivalds_value,
            &selected,
            &sampled,
            folded,
            &mut transcript,
        )?;
        if profile_stages {
            eprintln!(
                "[RAAFOLD_PROFILE] opening_top rows={} columns={} projection_s={:.9} fold_s={:.9} prepare_next_s={:.9} query_rows_s={:.9} cache_release_s={:.9} reduction_s={:.9}",
                top.message_len(),
                logical_columns,
                projection_time.as_secs_f64(),
                fold_time.as_secs_f64(),
                prepare_time.as_secs_f64(),
                rows_time.as_secs_f64(),
                release_time.as_secs_f64(),
                reduction_start.elapsed().as_secs_f64(),
            );
        }

        let mut pending = Vec::with_capacity(top_level - terminal_level + 1);
        if top_level == 0 {
            verify_base_generator(top.extension_code(), &top_reduction)?;
        } else {
            pending.push(top_reduction.pending_claim(top_level)?);
        }
        let mut next_value = top_reduction.next_value;
        let mut next_point = top_reduction.next_point;
        let mut rounds = Vec::with_capacity(top_level - terminal_level);

        for level in (terminal_level..top_level).rev() {
            let round_start = Instant::now();
            let code = pp.extension_sequence.code(level)?;
            let extension_columns = pp
                .extension_sequence
                .switch_ratio_for_source_level(level + 1)?;
            let round_cache = match prepared {
                PreparedNextLayer::Extension(cache) => cache,
                PreparedNextLayer::Ligero(_) => return Err(Error::InvalidCommitment),
            };
            if round_cache.message_entries.len()
                != code
                    .message_len()
                    .checked_mul(extension_columns)
                    .ok_or(Error::EncodingError)?
            {
                return Err(Error::EncodingError);
            }
            let (round, _next_folded, reduction, next_prepared) = prove_extension_round(
                &pp.extension_sequence,
                level,
                terminal_level,
                &pp.opening_tail,
                pp.security_bits,
                round_cache,
                &next_point,
                next_value,
                &mut transcript,
            )?;
            if level == 0 {
                verify_base_generator(code, &reduction)?;
            } else {
                pending.push(reduction.pending_claim(level)?);
            }
            rounds.push(round);
            next_value = reduction.next_value;
            next_point = reduction.next_point;
            prepared = next_prepared;
            if profile_stages {
                eprintln!(
                    "[RAAFOLD_PROFILE] opening_extension level={level} rows={} columns={extension_columns} total_s={:.9}",
                    code.message_len(),
                    round_start.elapsed().as_secs_f64(),
                );
            }
        }
        let tail_cache = match prepared {
            PreparedNextLayer::Ligero(cache) => cache,
            PreparedNextLayer::Extension(_) => return Err(Error::InvalidCommitment),
        };
        let tail_start = Instant::now();
        let tail = prove_ligero_tail(
            &pp.opening_tail,
            tail_cache,
            &next_point,
            next_value,
            pp.security_bits,
            &mut transcript,
        )?;
        if profile_stages {
            eprintln!(
                "[RAAFOLD_PROFILE] opening_tail rows={} columns={} tail_s={:.9} opening_total_s={:.9}",
                pp.opening_tail.height,
                pp.opening_tail.width,
                tail_start.elapsed().as_secs_f64(),
                opening_start.elapsed().as_secs_f64(),
            );
        }
        Ok((
            value,
            ShortRaaOpeningProof {
                responses: transcript.responses,
                top_rows,
                extension_rounds: rounds,
                tail,
            },
            ShortRaaOpeningReduction {
                ok: true,
                pending_generator_claims: pending,
            },
        ))
    }

    /// Verifies the recursive opening without yet consuming SwitchFold generator reductions.
    pub fn verify_opening_reduction(
        &self,
        pp: &ShortRaaFoldParameters,
        commitment: &ShortRaaFoldCommitment,
        point: &[BinaryExtensionField],
        value: BinaryExtensionField,
        proof: &ShortRaaOpeningProof,
    ) -> Result<ShortRaaOpeningReduction, Error> {
        let rejected = || ShortRaaOpeningReduction {
            ok: false,
            pending_generator_claims: Vec::new(),
        };
        if validate_parameters(self, pp).is_err()
            || point.len() != self.n_witness.ilog2() as usize
            || commitment.logical_elements() != self.n_witness
            || commitment.logical_element_bits() != self.logical_element_bits
            || commitment.columns() != self.packed_columns
        {
            return Ok(rejected());
        }
        let top_level = pp.sequence.levels() - 1;
        let terminal_level = pp.terminal_level;
        if terminal_level > top_level || proof.extension_rounds.len() != top_level - terminal_level
        {
            return Ok(rejected());
        }
        let top = pp.sequence.code(top_level)?;
        let logical_columns = commitment
            .columns()
            .checked_mul(128 / commitment.logical_element_bits())
            .ok_or(Error::EncodingError)?;
        let column_log = logical_columns.ilog2() as usize;
        if point.len() != column_log + top.message_len().ilog2() as usize {
            return Ok(rejected());
        }
        let mut transcript = OpeningTranscriptReader::new(&proof.responses);
        absorb_opening_statement_reader(pp, commitment, point, value, &mut transcript);
        transcript.absorb_digest(b"round-root", &commitment.root());
        let opening_vector = (0..logical_columns)
            .map(|_| transcript.read_response())
            .collect::<Result<Vec<_>, _>>()?;
        if inner_product(&opening_vector, &equality_tensor(&point[..column_log]))? != value {
            return Ok(rejected());
        }
        let top_freivalds = (0..logical_columns)
            .map(|_| transcript.challenge_non_singular(b"freivalds"))
            .collect::<Result<Vec<_>, _>>()?;
        let freivalds_value = inner_product(&opening_vector, &top_freivalds)?;
        let next_binding = if top_level == terminal_level {
            NextBinding::Ligero(proof.tail.root)
        } else {
            NextBinding::Extension(
                proof
                    .extension_rounds
                    .first()
                    .ok_or(Error::InvalidCommitment)?
                    .root,
            )
        };
        absorb_next_binding_reader(&mut transcript, next_binding);
        let selected = derive_selection_reader(
            top.extension_code(),
            pp.security_bits,
            commitment.encoded_rows(),
            &mut transcript,
        )?;
        if !pp.sequence.verify_top_packed_rows(
            commitment,
            self.logical_element_bits,
            &selected,
            &proof.top_rows,
        )? {
            return Ok(rejected());
        }
        let sampled = proof
            .top_rows
            .iter()
            .map(|query| {
                fold_packed_query_row(&query.row, self.logical_element_bits, &top_freivalds)
            })
            .collect::<Result<Vec<_>, Error>>()?;
        transcript.absorb_extensions(b"sampled-values", &sampled);
        let Some(top_reduction) = verify_round_reduction(
            top.extension_code(),
            point,
            OpeningPointBasis::Equality,
            logical_columns,
            freivalds_value,
            &selected,
            &sampled,
            &mut transcript,
        )?
        else {
            return Ok(rejected());
        };
        let mut pending = Vec::with_capacity(top_level - terminal_level + 1);
        if top_level == 0 {
            if verify_base_generator(top.extension_code(), &top_reduction).is_err() {
                return Ok(rejected());
            }
        } else {
            pending.push(top_reduction.pending_claim(top_level)?);
        }
        let mut expected_value = top_reduction.next_value;
        let mut expected_point = top_reduction.next_point;
        for (round_index, level) in (terminal_level..top_level).rev().enumerate() {
            let code = pp.extension_sequence.code(level)?;
            let round = &proof.extension_rounds[round_index];
            let next_binding = if round_index + 1 < proof.extension_rounds.len() {
                NextBinding::Extension(proof.extension_rounds[round_index + 1].root)
            } else {
                NextBinding::Ligero(proof.tail.root)
            };
            let Some(reduction) = verify_extension_round(
                &pp.extension_sequence,
                level,
                pp.security_bits,
                round,
                &expected_point,
                expected_value,
                next_binding,
                &mut transcript,
            )?
            else {
                return Ok(rejected());
            };
            if level == 0 {
                if verify_base_generator(code, &reduction).is_err() {
                    return Ok(rejected());
                }
            } else {
                pending.push(reduction.pending_claim(level)?);
            }
            expected_value = reduction.next_value;
            expected_point = reduction.next_point;
        }
        if !verify_ligero_tail(
            &pp.opening_tail,
            &proof.tail,
            &expected_point,
            expected_value,
            pp.security_bits,
            &mut transcript,
        )? {
            return Ok(rejected());
        }
        Ok(ShortRaaOpeningReduction {
            ok: transcript.is_exhausted(),
            pending_generator_claims: pending,
        })
    }
}

#[derive(Clone, Debug)]
struct ExtensionCache {
    message_entries: Vec<BinaryExtensionField>,
    encoded_entries: Vec<BinaryExtensionField>,
    n_rows: usize,
    n_cols: usize,
    tree: ExtensionMerkleTree,
}

#[derive(Clone, Debug)]
struct SwitchFoldAuxiliaryCache {
    logical_columns: usize,
    cache: ExtensionCache,
}

#[derive(Debug)]
struct SwitchFoldGeneratorProverOutput {
    proof: ShortRaaGeneratorProof,
    reduction: RaaGeneratorReduction<BinaryExtensionField>,
    auxiliary_claims: Vec<RaaMatrixClaim<BinaryExtensionField>>,
    auxiliary_caches: Vec<SwitchFoldAuxiliaryCache>,
}

#[derive(Debug)]
struct SwitchFoldGeneratorVerifierOutput {
    reduction: RaaGeneratorReduction<BinaryExtensionField>,
    auxiliary_claims: Vec<RaaMatrixClaim<BinaryExtensionField>>,
    auxiliary_columns: Vec<usize>,
}

#[derive(Clone, Copy)]
enum NextBinding {
    Extension([u8; DIGEST_BYTES]),
    Ligero([u8; DIGEST_BYTES]),
}

#[derive(Clone, Debug)]
enum PreparedNextLayer {
    Extension(ExtensionCache),
    Ligero(ExtensionCache),
}

#[derive(Clone, Debug)]
struct VerifiedRound {
    next_value: BinaryExtensionField,
    next_point: Vec<BinaryExtensionField>,
    generator_value: BinaryExtensionField,
    generator_left_point: Vec<BinaryExtensionField>,
    generator_right_point: Vec<BinaryExtensionField>,
}

#[derive(Clone, Debug)]
struct RaaEvalClaim {
    value: BinaryExtensionField,
    row_point: Vec<BinaryExtensionField>,
    col_weight: Vec<BinaryExtensionField>,
}

#[derive(Clone, Debug)]
struct RaaEvalWithPoint {
    value: BinaryExtensionField,
    point: Vec<BinaryExtensionField>,
}

#[derive(Clone, Debug)]
struct RaaPreparedDecideClaims {
    claims: Vec<RaaEvalClaim>,
    rows: usize,
    real_columns: usize,
    padded_columns: usize,
}

#[derive(Clone, Debug)]
struct RaaMultiEvalReduction {
    ok: bool,
    reduced: RaaEvalWithPoint,
}

#[derive(Clone, Debug)]
struct RaaMultiEvalProverResult {
    reduction: RaaMultiEvalReduction,
    folded_vector: Vec<BinaryExtensionField>,
}

#[derive(Clone, Debug)]
struct RaaDirectCodeSwitchResult {
    generator_claim: RaaMatrixClaim<BinaryExtensionField>,
    folded_witness_claim: RaaEvalWithPoint,
}

impl VerifiedRound {
    fn equality_generator_claim(&self) -> Result<RaaMatrixClaim<BinaryExtensionField>, Error> {
        Ok(RaaMatrixClaim {
            value: self.generator_value,
            point: (
                self.generator_left_point.iter().rev().copied().collect(),
                self.generator_right_point.iter().rev().copied().collect(),
            ),
        })
    }

    fn pending_claim(&self, level: usize) -> Result<ShortRaaPendingGeneratorClaim, Error> {
        Ok(ShortRaaPendingGeneratorClaim {
            level,
            claim: self.equality_generator_claim()?,
        })
    }
}

#[derive(Clone, Debug)]
struct LiteBulletReduction {
    a_value: BinaryExtensionField,
    a_point: Vec<BinaryExtensionField>,
    b_value: BinaryExtensionField,
    b_point: Vec<BinaryExtensionField>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OpeningPointBasis {
    Homogeneous,
    Equality,
}

fn prove_extension_round(
    sequence: &RaaFold,
    level: usize,
    terminal_level: usize,
    tail: &ShortRaaLigeroTailParameters,
    security_bits: usize,
    cache: ExtensionCache,
    point: &[BinaryExtensionField],
    value: BinaryExtensionField,
    transcript: &mut OpeningTranscriptWriter,
) -> Result<
    (
        ShortRaaExtensionRound,
        Vec<BinaryExtensionField>,
        VerifiedRound,
        PreparedNextLayer,
    ),
    Error,
> {
    let code = sequence.code(level)?;
    let extension_columns = sequence.switch_ratio_for_source_level(level + 1)?;
    if cache.n_rows != code.codeword_len()
        || cache.n_cols != extension_columns
        || cache.message_entries.len() != code.message_len() * extension_columns
        || cache.encoded_entries.len() != cache.n_rows * cache.n_cols
        || point.len() != extension_columns.ilog2() as usize + code.message_len().ilog2() as usize
    {
        return Err(Error::InvalidNumberOfVariables);
    }
    let column_log = extension_columns.ilog2() as usize;
    let opening_vector = systematic_row_projection(
        &cache.message_entries,
        code.message_len(),
        extension_columns,
        &equality_tensor(&point[column_log..]),
    )?;
    if inner_product(&opening_vector, &equality_tensor(&point[..column_log]))? != value {
        return Err(Error::InvalidCommitment);
    }
    transcript.absorb_digest(b"round-root", &cache.tree.root());
    for opening in &opening_vector {
        transcript.push_response(*opening);
    }
    let freivalds = (0..extension_columns)
        .map(|_| transcript.challenge_non_singular(b"freivalds"))
        .collect::<Result<Vec<_>, _>>()?;
    let freivalds_value = inner_product(&opening_vector, &freivalds)?;
    let folded = fold_message_matrix(
        &cache.message_entries,
        code.message_len(),
        extension_columns,
        &freivalds,
    )?;
    let (next_binding, prepared) = if level == terminal_level {
        prepare_ligero_tail(tail, &folded)?
    } else {
        let next_extension_columns = sequence.switch_ratio_for_source_level(level)?;
        prepare_next_layer(
            sequence,
            level.checked_sub(1),
            next_extension_columns,
            &folded,
        )?
    };
    absorb_next_binding_writer(transcript, next_binding);
    let selected = derive_selection_writer(code, security_bits, cache.n_rows, transcript)?;
    let rows = selected
        .iter()
        .map(|&row_index| {
            Ok(ShortRaaExtensionRowQuery {
                row: extension_row(
                    &cache.encoded_entries,
                    cache.n_rows,
                    cache.n_cols,
                    row_index,
                )?,
                path: cache.tree.path(row_index)?,
            })
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let sampled = rows
        .iter()
        .map(|query| inner_product(&query.row, &freivalds))
        .collect::<Result<Vec<_>, Error>>()?;
    transcript.absorb_extensions(b"sampled-values", &sampled);
    let reduction = prove_round_reduction(
        code,
        point,
        OpeningPointBasis::Equality,
        extension_columns,
        freivalds_value,
        &selected,
        &sampled,
        folded.clone(),
        transcript,
    )?;
    Ok((
        ShortRaaExtensionRound {
            root: cache.tree.root(),
            rows,
        },
        folded,
        reduction,
        prepared,
    ))
}

#[allow(clippy::too_many_arguments)]
fn verify_extension_round(
    sequence: &RaaFold,
    level: usize,
    security_bits: usize,
    round: &ShortRaaExtensionRound,
    point: &[BinaryExtensionField],
    value: BinaryExtensionField,
    next_binding: NextBinding,
    transcript: &mut OpeningTranscriptReader<'_>,
) -> Result<Option<VerifiedRound>, Error> {
    let code = sequence.code(level)?;
    let extension_columns = sequence.switch_ratio_for_source_level(level + 1)?;
    let column_log = extension_columns.ilog2() as usize;
    if point.len() != column_log + code.message_len().ilog2() as usize {
        return Ok(None);
    }
    transcript.absorb_digest(b"round-root", &round.root);
    let opening_vector = (0..extension_columns)
        .map(|_| transcript.read_response())
        .collect::<Result<Vec<_>, _>>()?;
    if inner_product(&opening_vector, &equality_tensor(&point[..column_log]))? != value {
        return Ok(None);
    }
    let freivalds = (0..extension_columns)
        .map(|_| transcript.challenge_non_singular(b"freivalds"))
        .collect::<Result<Vec<_>, _>>()?;
    let freivalds_value = inner_product(&opening_vector, &freivalds)?;
    absorb_next_binding_reader(transcript, next_binding);
    let selected = derive_selection_reader(code, security_bits, code.codeword_len(), transcript)?;
    if round.rows.len() != selected.len() {
        return Ok(None);
    }
    let mut sampled = Vec::with_capacity(selected.len());
    for (query, &row_index) in round.rows.iter().zip(&selected) {
        if query.row.len() != extension_columns
            || !verify_extension_path(
                hash_extension_row(&query.row),
                row_index,
                code.codeword_len(),
                &query.path,
                &round.root,
            )
        {
            return Ok(None);
        }
        sampled.push(inner_product(&query.row, &freivalds)?);
    }
    transcript.absorb_extensions(b"sampled-values", &sampled);
    verify_round_reduction(
        code,
        point,
        OpeningPointBasis::Equality,
        extension_columns,
        freivalds_value,
        &selected,
        &sampled,
        transcript,
    )
}

#[allow(clippy::too_many_arguments)]
fn prove_round_reduction(
    code: &RaaCode,
    point: &[BinaryExtensionField],
    point_basis: OpeningPointBasis,
    n_cols: usize,
    freivalds_value: BinaryExtensionField,
    selected_rows: &[usize],
    sampled_values: &[BinaryExtensionField],
    folded: Vec<BinaryExtensionField>,
    transcript: &mut OpeningTranscriptWriter,
) -> Result<VerifiedRound, Error> {
    let n_rows = code.message_len();
    let n_ext_rows = code.codeword_len();
    let column_log = n_cols.ilog2() as usize;
    if point.len() != column_log + n_rows.ilog2() as usize
        || folded.len() != n_rows
        || selected_rows.is_empty()
        || selected_rows.len() != sampled_values.len()
    {
        return Err(Error::InvalidNumberOfVariables);
    }
    let selection_log = selected_rows.len().next_power_of_two().ilog2() as usize;
    let selection_point = (0..selection_log)
        .map(|_| transcript.challenge_non_singular(b"selection-left"))
        .collect::<Result<Vec<_>, _>>()?;
    let selection_weights = homogeneous_tensor_prefix(&selection_point, selected_rows.len())?;
    let selected_value = inner_product(&selection_weights, sampled_values)?;
    let mut sparse_left = vec![BinaryExtensionField::ZERO; n_ext_rows];
    for (&row, &weight) in selected_rows.iter().zip(&selection_weights) {
        sparse_left[row] += weight;
    }
    let generator_left = code.generator_left_projection(&sparse_left)?;
    let rho = transcript.challenge_non_singular(b"opening-batch")?;
    let combined_value = selected_value + rho * freivalds_value;
    let row_weights = tensor_weights(&point[column_log..], point_basis);
    let combined_left = generator_left
        .into_iter()
        .zip(&row_weights)
        .map(|(selection, opening)| selection + rho * *opening)
        .collect::<Vec<_>>();
    let message = prove_litebullet(
        combined_value,
        combined_left,
        folded,
        transcript,
        b"message-litebullet",
    )?;
    let opening_left = inner_product(&row_weights, &equality_tensor(&message.a_point))?;
    let selection_left = message.a_value - rho * opening_left;
    let generator_right = code.generator_right_projection(&equality_tensor(&message.a_point))?;
    let generator = prove_sparse_litebullet(
        selection_left,
        n_ext_rows,
        selected_rows,
        &selection_weights,
        generator_right,
        transcript,
        b"generator-litebullet",
    )?;
    Ok(VerifiedRound {
        next_value: message.b_value,
        next_point: message.b_point,
        generator_value: generator.b_value,
        generator_left_point: generator.b_point,
        generator_right_point: message.a_point,
    })
}

fn verify_round_reduction(
    code: &RaaCode,
    point: &[BinaryExtensionField],
    point_basis: OpeningPointBasis,
    n_cols: usize,
    freivalds_value: BinaryExtensionField,
    selected_rows: &[usize],
    sampled_values: &[BinaryExtensionField],
    transcript: &mut OpeningTranscriptReader<'_>,
) -> Result<Option<VerifiedRound>, Error> {
    let n_rows = code.message_len();
    let n_ext_rows = code.codeword_len();
    let column_log = n_cols.ilog2() as usize;
    if point.len() != column_log + n_rows.ilog2() as usize
        || selected_rows.is_empty()
        || selected_rows.len() != sampled_values.len()
    {
        return Ok(None);
    }
    let selection_log = selected_rows.len().next_power_of_two().ilog2() as usize;
    let selection_point = (0..selection_log)
        .map(|_| transcript.challenge_non_singular(b"selection-left"))
        .collect::<Result<Vec<_>, _>>()?;
    let selection_weights = homogeneous_tensor_prefix(&selection_point, selected_rows.len())?;
    let selected_value = inner_product(&selection_weights, sampled_values)?;
    let rho = transcript.challenge_non_singular(b"opening-batch")?;
    let combined_value = selected_value + rho * freivalds_value;
    let Some(message) =
        verify_litebullet(combined_value, n_rows, transcript, b"message-litebullet")?
    else {
        return Ok(None);
    };
    let opening_left =
        tensor_equality_inner_product(&point[column_log..], point_basis, &message.a_point)?;
    let selection_left = message.a_value - rho * opening_left;
    let Some(generator) = verify_litebullet(
        selection_left,
        n_ext_rows,
        transcript,
        b"generator-litebullet",
    )?
    else {
        return Ok(None);
    };
    let expected_sparse = selected_rows
        .iter()
        .zip(&selection_weights)
        .map(|(&row, &weight)| weight * equality_tensor_weight_at(&generator.a_point, row))
        .sum();
    if generator.a_value != expected_sparse {
        return Ok(None);
    }
    Ok(Some(VerifiedRound {
        next_value: message.b_value,
        next_point: message.b_point,
        generator_value: generator.b_value,
        generator_left_point: generator.b_point,
        generator_right_point: message.a_point,
    }))
}

fn prove_litebullet(
    claimed_value: BinaryExtensionField,
    mut left: Vec<BinaryExtensionField>,
    mut right: Vec<BinaryExtensionField>,
    transcript: &mut OpeningTranscriptWriter,
    label: &[u8],
) -> Result<LiteBulletReduction, Error> {
    if left.is_empty()
        || !left.len().is_power_of_two()
        || left.len() != right.len()
        || inner_product(&left, &right)? != claimed_value
    {
        return Err(Error::InvalidCommitment);
    }
    transcript.absorb_bytes(b"litebullet-domain", label);
    let mut rhs = claimed_value;
    let mut a_point = Vec::with_capacity(left.len().ilog2() as usize);
    let mut b_point = Vec::with_capacity(left.len().ilog2() as usize);
    while left.len() > 1 {
        let half = left.len() / 2;
        let high_diagonal = inner_product(&left[half..], &right[half..])?;
        let cross_sum = inner_product(&left[..half], &right[half..])?
            + inner_product(&left[half..], &right[..half])?;
        transcript.push_response(high_diagonal);
        transcript.push_response(cross_sum);
        let challenge = transcript.challenge_non_singular(b"litebullet-fold")?;
        rhs = multilinear_inner_product_round(rhs, high_diagonal, cross_sum, challenge);
        let (left_lower, left_upper) = left.split_at_mut(half);
        let (right_lower, right_upper) = right.split_at_mut(half);
        fold_litebullet_pair(left_lower, left_upper, right_lower, right_upper, challenge);
        left.truncate(half);
        right.truncate(half);
        a_point.push(challenge);
        b_point.push(challenge);
    }
    let a_value = left[0];
    let b_value = right[0];
    transcript.push_response(a_value);
    transcript.push_response(b_value);
    if a_value * b_value != rhs {
        return Err(Error::InvalidCommitment);
    }
    Ok(LiteBulletReduction {
        a_value,
        a_point,
        b_value,
        b_point,
    })
}

fn prove_sparse_litebullet(
    claimed_value: BinaryExtensionField,
    length: usize,
    selected_indices: &[usize],
    selected_weights: &[BinaryExtensionField],
    right: Vec<BinaryExtensionField>,
    transcript: &mut OpeningTranscriptWriter,
    label: &[u8],
) -> Result<LiteBulletReduction, Error> {
    if length == 0
        || !length.is_power_of_two()
        || right.len() != length
        || selected_indices.len() != selected_weights.len()
        || selected_indices.iter().any(|&index| index >= length)
    {
        return Err(Error::InvalidNumberOfVariables);
    }
    let mut left = vec![BinaryExtensionField::ZERO; length];
    for (&index, &weight) in selected_indices.iter().zip(selected_weights) {
        left[index] += weight;
    }
    prove_litebullet(claimed_value, left, right, transcript, label)
}

fn verify_litebullet(
    claimed_value: BinaryExtensionField,
    length: usize,
    transcript: &mut OpeningTranscriptReader<'_>,
    label: &[u8],
) -> Result<Option<LiteBulletReduction>, Error> {
    if length == 0 || !length.is_power_of_two() {
        return Ok(None);
    }
    transcript.absorb_bytes(b"litebullet-domain", label);
    let mut rhs = claimed_value;
    let mut current = length;
    let mut a_point = Vec::with_capacity(length.ilog2() as usize);
    let mut b_point = Vec::with_capacity(length.ilog2() as usize);
    while current > 1 {
        let high_diagonal = transcript.read_response()?;
        let cross_sum = transcript.read_response()?;
        let challenge = transcript.challenge_non_singular(b"litebullet-fold")?;
        rhs = multilinear_inner_product_round(rhs, high_diagonal, cross_sum, challenge);
        a_point.push(challenge);
        b_point.push(challenge);
        current /= 2;
    }
    let a_value = transcript.read_response()?;
    let b_value = transcript.read_response()?;
    if a_value * b_value != rhs {
        return Ok(None);
    }
    Ok(Some(LiteBulletReduction {
        a_value,
        a_point,
        b_value,
        b_point,
    }))
}

struct OpeningTranscriptWriter {
    hasher: Blake2s256,
    challenge_counter: u64,
    responses: Vec<BinaryExtensionField>,
}

impl OpeningTranscriptWriter {
    fn new() -> Self {
        Self::with_domain(TRANSCRIPT_DOMAIN)
    }

    fn with_domain(domain: &[u8]) -> Self {
        let mut transcript = Self {
            hasher: Blake2s256::new(),
            challenge_counter: 0,
            responses: Vec::new(),
        };
        transcript.absorb_bytes(b"domain", domain);
        transcript
    }

    fn push_response(&mut self, value: BinaryExtensionField) {
        self.absorb_field(b"response", value);
        self.responses.push(value);
    }

    fn absorb_bytes(&mut self, label: &[u8], bytes: &[u8]) {
        absorb_framed(&mut self.hasher, label, bytes);
    }

    fn absorb_digest(&mut self, label: &[u8], digest: &[u8; DIGEST_BYTES]) {
        self.absorb_bytes(label, digest);
    }

    fn absorb_field(&mut self, label: &[u8], value: BinaryExtensionField) {
        self.absorb_bytes(label, &u128::from(value).to_le_bytes());
    }

    fn absorb_extensions(&mut self, label: &[u8], values: &[BinaryExtensionField]) {
        self.absorb_bytes(b"vector-label", label);
        self.absorb_bytes(b"vector-length", &(values.len() as u64).to_le_bytes());
        for &value in values {
            self.absorb_field(b"vector-element", value);
        }
    }

    fn challenge_digest(&mut self, label: &[u8]) -> [u8; DIGEST_BYTES] {
        let digest = derive_digest(&self.hasher, label, self.challenge_counter, 0);
        self.challenge_counter = self.challenge_counter.wrapping_add(1);
        self.absorb_bytes(b"digest-challenge-label", label);
        self.absorb_digest(b"digest-challenge", &digest);
        digest
    }

    fn challenge_non_singular(&mut self, label: &[u8]) -> Result<BinaryExtensionField, Error> {
        let counter = self.challenge_counter;
        for attempt in 0..u64::MAX {
            let digest = derive_digest(&self.hasher, label, counter, attempt);
            let value = BinaryExtensionField::from(u128::from_le_bytes(
                digest[..FIELD_BYTES]
                    .try_into()
                    .map_err(|_| Error::TranscriptError)?,
            ));
            if value != BinaryExtensionField::ZERO && value != BinaryExtensionField::ONE {
                self.challenge_counter = self.challenge_counter.wrapping_add(1);
                self.absorb_bytes(b"field-challenge-label", label);
                self.absorb_field(b"field-challenge", value);
                return Ok(value);
            }
        }
        Err(Error::TranscriptError)
    }
}

struct OpeningTranscriptReader<'a> {
    hasher: Blake2s256,
    challenge_counter: u64,
    responses: &'a [BinaryExtensionField],
    cursor: usize,
}

impl<'a> OpeningTranscriptReader<'a> {
    fn new(responses: &'a [BinaryExtensionField]) -> Self {
        Self::with_domain(responses, TRANSCRIPT_DOMAIN)
    }

    fn with_domain(responses: &'a [BinaryExtensionField], domain: &[u8]) -> Self {
        let mut transcript = Self {
            hasher: Blake2s256::new(),
            challenge_counter: 0,
            responses,
            cursor: 0,
        };
        transcript.absorb_bytes(b"domain", domain);
        transcript
    }

    fn read_response(&mut self) -> Result<BinaryExtensionField, Error> {
        let value = self
            .responses
            .get(self.cursor)
            .copied()
            .ok_or(Error::TranscriptError)?;
        self.cursor += 1;
        self.absorb_field(b"response", value);
        Ok(value)
    }

    fn absorb_bytes(&mut self, label: &[u8], bytes: &[u8]) {
        absorb_framed(&mut self.hasher, label, bytes);
    }

    fn absorb_digest(&mut self, label: &[u8], digest: &[u8; DIGEST_BYTES]) {
        self.absorb_bytes(label, digest);
    }

    fn absorb_field(&mut self, label: &[u8], value: BinaryExtensionField) {
        self.absorb_bytes(label, &u128::from(value).to_le_bytes());
    }

    fn absorb_extensions(&mut self, label: &[u8], values: &[BinaryExtensionField]) {
        self.absorb_bytes(b"vector-label", label);
        self.absorb_bytes(b"vector-length", &(values.len() as u64).to_le_bytes());
        for &value in values {
            self.absorb_field(b"vector-element", value);
        }
    }

    fn challenge_digest(&mut self, label: &[u8]) -> [u8; DIGEST_BYTES] {
        let digest = derive_digest(&self.hasher, label, self.challenge_counter, 0);
        self.challenge_counter = self.challenge_counter.wrapping_add(1);
        self.absorb_bytes(b"digest-challenge-label", label);
        self.absorb_digest(b"digest-challenge", &digest);
        digest
    }

    fn challenge_non_singular(&mut self, label: &[u8]) -> Result<BinaryExtensionField, Error> {
        let counter = self.challenge_counter;
        for attempt in 0..u64::MAX {
            let digest = derive_digest(&self.hasher, label, counter, attempt);
            let value = BinaryExtensionField::from(u128::from_le_bytes(
                digest[..FIELD_BYTES]
                    .try_into()
                    .map_err(|_| Error::TranscriptError)?,
            ));
            if value != BinaryExtensionField::ZERO && value != BinaryExtensionField::ONE {
                self.challenge_counter = self.challenge_counter.wrapping_add(1);
                self.absorb_bytes(b"field-challenge-label", label);
                self.absorb_field(b"field-challenge", value);
                return Ok(value);
            }
        }
        Err(Error::TranscriptError)
    }

    fn is_exhausted(&self) -> bool {
        self.cursor == self.responses.len()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SwitchFoldOracle {
    index: usize,
}

struct SwitchFoldGeneratorProverChannel<'a> {
    sequence: &'a RaaFold,
    level: usize,
    transcript: &'a mut OpeningTranscriptWriter,
    oracle_specs: Vec<OracleSpec>,
    next_oracle: usize,
    auxiliary_caches: Vec<SwitchFoldAuxiliaryCache>,
    auxiliary_claims: Vec<(usize, RaaMatrixClaim<BinaryExtensionField>)>,
    relation_error: Option<Error>,
}

impl<'a> SwitchFoldGeneratorProverChannel<'a> {
    fn new(
        sequence: &'a RaaFold,
        level: usize,
        oracle_specs: Vec<OracleSpec>,
        transcript: &'a mut OpeningTranscriptWriter,
    ) -> Self {
        Self {
            sequence,
            level,
            transcript,
            oracle_specs,
            next_oracle: 0,
            auxiliary_caches: Vec::new(),
            auxiliary_claims: Vec::new(),
            relation_error: None,
        }
    }

    fn finish(
        self,
    ) -> Result<
        (
            Vec<SwitchFoldAuxiliaryCache>,
            Vec<RaaMatrixClaim<BinaryExtensionField>>,
        ),
        Error,
    > {
        if let Some(error) = self.relation_error {
            return Err(error);
        }
        if self.next_oracle != self.oracle_specs.len()
            || self.auxiliary_caches.len() != self.oracle_specs.len()
            || self.auxiliary_claims.len() != self.oracle_specs.len()
        {
            return Err(Error::InvalidCommitment);
        }
        let mut claims = vec![None; self.oracle_specs.len()];
        for (index, claim) in self.auxiliary_claims {
            let slot = claims.get_mut(index).ok_or(Error::InvalidCommitment)?;
            if slot.replace(claim).is_some() {
                return Err(Error::InvalidCommitment);
            }
        }
        Ok((
            self.auxiliary_caches,
            claims
                .into_iter()
                .collect::<Option<Vec<_>>>()
                .ok_or(Error::InvalidCommitment)?,
        ))
    }
}

impl IPProverChannel<BinaryExtensionField> for SwitchFoldGeneratorProverChannel<'_> {
    fn send_one(&mut self, elem: BinaryExtensionField) {
        self.transcript.push_response(elem);
    }

    fn send_many(&mut self, elems: &[BinaryExtensionField]) {
        for &elem in elems {
            self.send_one(elem);
        }
    }

    fn observe_one(&mut self, value: BinaryExtensionField) {
        self.transcript
            .absorb_field(b"generator-observed-field", value);
    }

    fn observe_many(&mut self, values: &[BinaryExtensionField]) {
        self.transcript
            .absorb_extensions(b"generator-observed-fields", values);
    }

    fn sample(&mut self) -> BinaryExtensionField {
        self.transcript
            .challenge_non_singular(b"generator-channel-challenge")
            .expect("the hash transcript must yield a non-singular challenge")
    }
}

impl IOPProverChannel<PackedBinaryExtensionField> for SwitchFoldGeneratorProverChannel<'_> {
    type Oracle = SwitchFoldOracle;

    fn remaining_oracle_specs(&self) -> &[OracleSpec] {
        &self.oracle_specs[self.next_oracle..]
    }

    fn send_oracle(&mut self, buffer: FieldSlice<PackedBinaryExtensionField>) -> Self::Oracle {
        let index = self.next_oracle;
        let spec = self
            .oracle_specs
            .get(index)
            .expect("the RAA reduction must not send extra auxiliary blocks");
        assert!(!spec.is_zk, "direct SwitchFold commitments are non-hiding");
        assert_eq!(buffer.log_len(), spec.log_msg_len);
        let message_entries = buffer.iter_scalars().collect::<Vec<_>>();
        let auxiliary =
            commit_direct_interleaved_matrix(self.sequence, self.level, message_entries)
                .expect("a valid RAA auxiliary block must fit the direct lower-code commitment");
        absorb_auxiliary_commitment_writer(
            self.level,
            index,
            auxiliary.logical_columns,
            &auxiliary.cache,
            self.transcript,
        );
        self.auxiliary_caches.push(auxiliary);
        self.next_oracle += 1;
        SwitchFoldOracle { index }
    }

    fn prove_oracle_relations(
        &mut self,
        oracle_relations: impl IntoIterator<
            Item = (
                Self::Oracle,
                FieldBuffer<PackedBinaryExtensionField>,
                FieldBuffer<PackedBinaryExtensionField>,
                BinaryExtensionField,
            ),
        >,
    ) {
        for (oracle, message, transparent, claimed_value) in oracle_relations {
            if self.relation_error.is_some() {
                continue;
            }
            let Some(auxiliary) = self.auxiliary_caches.get(oracle.index) else {
                self.relation_error = Some(Error::InvalidCommitment);
                continue;
            };
            let message = message.take_data();
            if message != auxiliary.cache.message_entries {
                self.relation_error = Some(Error::InvalidCommitment);
                continue;
            }
            let transparent = transparent.take_data();
            match crate::raafold::prove_switchfold_oracle_relation(
                claimed_value,
                message,
                transparent,
                self,
            ) {
                Ok((oracle_claim, _transparent_claim)) => {
                    self.auxiliary_claims.push((oracle.index, oracle_claim));
                }
                Err(error) => self.relation_error = Some(error),
            }
        }
    }
}

struct SwitchFoldGeneratorVerifierChannel<'a, 'b> {
    sequence: &'a RaaFold,
    level: usize,
    transcript: &'a mut OpeningTranscriptReader<'b>,
    oracle_specs: Vec<OracleSpec>,
    roots: &'a [[u8; DIGEST_BYTES]],
    next_oracle: usize,
    auxiliary_claims: Vec<(usize, RaaMatrixClaim<BinaryExtensionField>)>,
}

impl<'a, 'b> SwitchFoldGeneratorVerifierChannel<'a, 'b> {
    fn new(
        sequence: &'a RaaFold,
        level: usize,
        oracle_specs: Vec<OracleSpec>,
        roots: &'a [[u8; DIGEST_BYTES]],
        transcript: &'a mut OpeningTranscriptReader<'b>,
    ) -> Self {
        Self {
            sequence,
            level,
            transcript,
            oracle_specs,
            roots,
            next_oracle: 0,
            auxiliary_claims: Vec::new(),
        }
    }

    fn finish(self) -> Result<Vec<RaaMatrixClaim<BinaryExtensionField>>, Error> {
        if self.next_oracle != self.oracle_specs.len()
            || self.roots.len() != self.oracle_specs.len()
            || self.auxiliary_claims.len() != self.oracle_specs.len()
        {
            return Err(Error::InvalidCommitment);
        }
        let mut claims = vec![None; self.oracle_specs.len()];
        for (index, claim) in self.auxiliary_claims {
            let slot = claims.get_mut(index).ok_or(Error::InvalidCommitment)?;
            if slot.replace(claim).is_some() {
                return Err(Error::InvalidCommitment);
            }
        }
        claims
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .ok_or(Error::InvalidCommitment)
    }
}

impl IPVerifierChannel<BinaryExtensionField> for SwitchFoldGeneratorVerifierChannel<'_, '_> {
    type Elem = BinaryExtensionField;

    fn recv_one(&mut self) -> Result<Self::Elem, BiniusIpError> {
        self.transcript
            .read_response()
            .map_err(|_| BiniusIpError::ProofEmpty)
    }

    fn sample(&mut self) -> Self::Elem {
        self.transcript
            .challenge_non_singular(b"generator-channel-challenge")
            .expect("the hash transcript must yield a non-singular challenge")
    }

    fn observe_one(&mut self, value: BinaryExtensionField) -> Self::Elem {
        self.transcript
            .absorb_field(b"generator-observed-field", value);
        value
    }

    fn observe_many(&mut self, values: &[BinaryExtensionField]) -> Vec<Self::Elem> {
        self.transcript
            .absorb_extensions(b"generator-observed-fields", values);
        values.to_vec()
    }

    fn assert_zero(&mut self, value: Self::Elem) -> Result<(), BiniusIpError> {
        if value == BinaryExtensionField::ZERO {
            Ok(())
        } else {
            Err(BiniusIpError::InvalidAssert)
        }
    }

    fn compute_public_value(
        &mut self,
        inputs: &[Self::Elem],
        function: impl FieldFn<BinaryExtensionField>,
    ) -> Self::Elem {
        function.call_native(inputs)
    }
}

impl IOPVerifierChannel<BinaryExtensionField> for SwitchFoldGeneratorVerifierChannel<'_, '_> {
    type Oracle = SwitchFoldOracle;

    fn remaining_oracle_specs(&self) -> &[OracleSpec] {
        &self.oracle_specs[self.next_oracle..]
    }

    fn recv_oracle(
        &mut self,
        log_msg_len: usize,
        _is_witness_dependent: bool,
    ) -> Result<Self::Oracle, BiniusIopError> {
        let index = self.next_oracle;
        let Some(spec) = self.oracle_specs.get(index) else {
            return Err(BiniusIopError::IPChannel(BiniusIpError::InvalidAssert));
        };
        if spec.is_zk || spec.log_msg_len != log_msg_len || self.roots.get(index).is_none() {
            return Err(BiniusIopError::IPChannel(BiniusIpError::InvalidAssert));
        }
        let logical_rows = self
            .sequence
            .direct_interleaved_level(self.level)
            .and_then(|direct| direct.logical_rows())
            .map_err(|_| BiniusIopError::IPChannel(BiniusIpError::InvalidAssert))?;
        let message_len = 1usize
            .checked_shl(log_msg_len as u32)
            .ok_or(BiniusIopError::IPChannel(BiniusIpError::InvalidAssert))?;
        if message_len % logical_rows != 0 {
            return Err(BiniusIopError::IPChannel(BiniusIpError::InvalidAssert));
        }
        let logical_columns = message_len / logical_rows;
        absorb_auxiliary_commitment_reader(
            self.sequence,
            self.level,
            index,
            logical_columns,
            self.roots[index],
            self.transcript,
        )
        .map_err(|_| BiniusIopError::IPChannel(BiniusIpError::InvalidAssert))?;
        self.next_oracle += 1;
        Ok(SwitchFoldOracle { index })
    }

    fn verify_oracle_relations(
        &mut self,
        oracle_relations: impl IntoIterator<Item = OracleLinearRelation<Self::Oracle, Self::Elem>>,
    ) -> Result<(), BiniusIopError> {
        for relation in oracle_relations {
            let index = relation.oracle.index;
            let log_msg_len = self
                .oracle_specs
                .get(index)
                .ok_or(BiniusIopError::IPChannel(BiniusIpError::InvalidAssert))?
                .log_msg_len;
            let (oracle_claim, transparent_claim) =
                crate::raafold::verify_switchfold_oracle_relation(
                    log_msg_len,
                    relation.claim,
                    self,
                )
                .map_err(|_| BiniusIopError::IPChannel(BiniusIpError::InvalidAssert))?;
            let transparent_point = transparent_claim.point.0.as_slice();
            if transparent_claim.value != (relation.transparent)(transparent_point) {
                return Err(BiniusIopError::IPChannel(BiniusIpError::InvalidAssert));
            }
            self.auxiliary_claims.push((index, oracle_claim));
        }
        Ok(())
    }
}

impl ShortRaaFoldPC {
    /// Proves all generator claims and recursively authenticates their fixed `H_j` claims.
    pub fn prove_decide(
        &self,
        pp: &ShortRaaFoldParameters,
        prover_setup: &ShortRaaFoldProverSetup,
        commitment: &ShortRaaFoldCommitment,
        opening: &ShortRaaOpeningReduction,
    ) -> Result<ShortRaaDecideProof, Error> {
        // [PROFILE] Opt-in wall-clock boundaries avoid affecting normal benchmark output.
        let profile_stages = std::env::var_os("RAAFOLD_PROFILE_STAGES").is_some();
        let decide_total_start = Instant::now();
        if !opening.ok
            || validate_parameters(self, pp).is_err()
            || validate_prover_setup(pp, prover_setup).is_err()
        {
            return Err(Error::InvalidCommitment);
        }
        validate_pending_order(pp, opening)?;
        let levels = pp.extension_sequence.levels();
        let terminal_level = pp.terminal_level;
        let mut transcript = OpeningTranscriptWriter::with_domain(DECIDE_TRANSCRIPT_DOMAIN);
        absorb_decide_statement_writer(pp, commitment, opening, &mut transcript);
        let mut generator_proofs = Vec::with_capacity(levels.saturating_mul(2));
        let mut rounds = Vec::with_capacity(levels.saturating_sub(terminal_level));
        let mut carried_generator = None;
        let mut carried_witness_claim = None;
        let mut carried_witness_cache: Option<ExtensionCache> = None;
        let mut tail = None;
        let mut generator_ordinal = 0usize;

        for level in (terminal_level..levels).rev() {
            let level_start = Instant::now();
            let code = pp.extension_sequence.code(level)?;
            let setup = pp.setup_level(level)?;
            let setup_cache = prover_setup.setup_cache(level)?;
            validate_setup_level(pp, level, setup)?;
            let mut claims = vec![opening_claim_at(opening, level)?];
            if let Some(claim) = carried_generator.take() {
                claims.push(claim);
            }
            let claim_count = claims.len();
            absorb_decide_round_context_writer(
                level,
                setup,
                carried_witness_cache.as_ref(),
                claim_count,
                &mut transcript,
            );

            let generator_start = Instant::now();
            let mut h_claims = Vec::with_capacity(claims.len());
            let mut auxiliary_claims = Vec::with_capacity(claims.len() * 2);
            let mut auxiliary_caches = Vec::with_capacity(claims.len() * 2);
            for claim in claims {
                let pending = ShortRaaPendingGeneratorClaim { level, claim };
                let output = prove_generator_claim(
                    pp,
                    commitment,
                    &pending,
                    generator_ordinal,
                    &mut transcript,
                )?;
                h_claims.extend(output.reduction.fixed_h_claims(code)?);
                auxiliary_claims.extend(output.auxiliary_claims);
                auxiliary_caches.extend(output.auxiliary_caches);
                generator_proofs.push(output.proof);
                generator_ordinal += 1;
            }
            let generator_time = generator_start.elapsed();

            let matrix_build_start = Instant::now();
            let auxiliary_columns = auxiliary_caches
                .iter()
                .map(|auxiliary| auxiliary.logical_columns)
                .collect::<Vec<_>>();
            let prepared = prepare_decide_claims(
                code,
                &h_claims,
                &auxiliary_claims,
                &auxiliary_columns,
                carried_witness_claim.as_ref(),
            )?;
            let matrix_entries = decide_matrix_entries(
                code,
                &auxiliary_caches,
                carried_witness_cache
                    .as_ref()
                    .map(|cache| cache.message_entries.as_slice()),
                prepared.real_columns,
                prepared.padded_columns,
            )?;
            let matrix_build_time = matrix_build_start.elapsed();
            let multi_eval_start = Instant::now();
            let multi_eval_result = prove_decide_multi_eval(
                &prepared.claims,
                matrix_entries,
                prepared.rows,
                prepared.real_columns,
                prepared.padded_columns,
                &mut transcript,
            )?;
            let multi_eval = multi_eval_result.reduction;
            let folded = multi_eval_result.folded_vector;
            if !multi_eval.ok {
                return Err(Error::InvalidCommitment);
            }
            let multi_eval_time = multi_eval_start.elapsed();
            let (column_point, row_point) = multi_eval
                .reduced
                .point
                .split_at(prepared.padded_columns.ilog2() as usize);
            if equality_mle_high_to_low(&folded, row_point) != Some(multi_eval.reduced.value) {
                return Err(Error::InvalidCommitment);
            }

            let authenticate_start = Instant::now();
            let selection = derive_direct_selection_writer(
                &pp.extension_sequence,
                level,
                pp.security_bits,
                &mut transcript,
            )?;
            let setup_rows = open_extension_rows(setup_cache, &selection)?;
            let auxiliary_rows = auxiliary_caches
                .iter()
                .map(|auxiliary| open_extension_rows(&auxiliary.cache, &selection))
                .collect::<Result<Vec<_>, Error>>()?;
            let carry_rows = match carried_witness_cache.as_ref() {
                Some(cache) => Some(open_extension_rows(cache, &selection)?),
                None => None,
            };
            let (iota, recursive_vector, base_point) = prove_direct_iota(
                pp,
                level,
                setup,
                &auxiliary_caches,
                carried_witness_cache.as_ref(),
                &setup_rows,
                &auxiliary_rows,
                carry_rows.as_deref(),
                &selection,
                &folded,
                column_point,
                row_point,
                prepared.real_columns,
            )?;
            transcript.absorb_extensions(b"decide-iota", &iota);
            let authenticate_time = authenticate_start.elapsed();

            let carry_commit_start = Instant::now();
            let (next_carry_root, next_carry, tail_cache, tail_root) = if level > terminal_level {
                let cache = commit_direct_carry(
                    &pp.extension_sequence,
                    level - 1,
                    recursive_vector.clone(),
                )?;
                (Some(cache.tree.root()), Some(cache), None, None)
            } else {
                let (binding, prepared_tail) =
                    prepare_ligero_tail(&pp.decide_tail, &recursive_vector)?;
                let root = match binding {
                    NextBinding::Ligero(root) => root,
                    NextBinding::Extension(_) => return Err(Error::InvalidCommitment),
                };
                let cache = match prepared_tail {
                    PreparedNextLayer::Ligero(cache) => cache,
                    PreparedNextLayer::Extension(_) => return Err(Error::InvalidCommitment),
                };
                (None, None, Some(cache), Some(root))
            };
            let carry_commit_time = carry_commit_start.elapsed();
            absorb_decide_next_writer(
                next_carry_root.as_ref(),
                tail_root.as_ref(),
                &mut transcript,
            )?;
            let code_switch_start = Instant::now();
            let switched = prove_direct_code_switch(
                pp.extension_sequence.code(level - 1)?,
                level - 1,
                &selection,
                &iota,
                &recursive_vector,
                multi_eval.reduced.value,
                base_point,
                &mut transcript,
            )?;
            let code_switch_time = code_switch_start.elapsed();

            if level > terminal_level {
                carried_generator = Some(switched.generator_claim);
                carried_witness_claim = Some(switched.folded_witness_claim);
                carried_witness_cache = next_carry;
            } else {
                verify_terminal_generator(pp.extension_sequence.code(level - 1)?, &switched)?;
                let tail_point = switched
                    .folded_witness_claim
                    .point
                    .iter()
                    .rev()
                    .copied()
                    .collect::<Vec<_>>();
                tail = Some(prove_ligero_tail(
                    &pp.decide_tail,
                    tail_cache.ok_or(Error::InvalidCommitment)?,
                    &tail_point,
                    switched.folded_witness_claim.value,
                    pp.security_bits,
                    &mut transcript,
                )?);
                carried_witness_cache = None;
                carried_witness_claim = None;
            }
            rounds.push(ShortRaaDecideRound {
                level,
                setup_rows,
                auxiliary_rows,
                carry_rows,
                next_carry_root,
            });
            if profile_stages {
                eprintln!(
                    "[RAAFOLD_PROFILE] decide_level={level} claims={claim_count} rows={} real_columns={} padded_columns={} generator_s={:.9} matrix_build_s={:.9} multi_eval_s={:.9} authenticate_iota_s={:.9} carry_commit_s={:.9} code_switch_s={:.9} level_total_s={:.9}",
                    prepared.rows,
                    prepared.real_columns,
                    prepared.padded_columns,
                    generator_time.as_secs_f64(),
                    matrix_build_time.as_secs_f64(),
                    multi_eval_time.as_secs_f64(),
                    authenticate_time.as_secs_f64(),
                    carry_commit_time.as_secs_f64(),
                    code_switch_time.as_secs_f64(),
                    level_start.elapsed().as_secs_f64(),
                );
            }
        }

        if carried_generator.is_some()
            || carried_witness_claim.is_some()
            || carried_witness_cache.is_some()
            || tail.is_none()
        {
            return Err(Error::InvalidCommitment);
        }
        if profile_stages {
            eprintln!(
                "[RAAFOLD_PROFILE] decide_total_s={:.9}",
                decide_total_start.elapsed().as_secs_f64()
            );
        }
        Ok(ShortRaaDecideProof {
            generator_proofs,
            responses: transcript.responses,
            rounds,
            tail: tail.ok_or(Error::InvalidCommitment)?,
        })
    }

    /// Verifies all SwitchFold generator reductions and the recursive Decide proof.
    pub fn verify_decide(
        &self,
        pp: &ShortRaaFoldParameters,
        commitment: &ShortRaaFoldCommitment,
        opening: &ShortRaaOpeningReduction,
        proof: &ShortRaaDecideProof,
    ) -> Result<bool, Error> {
        if !opening.ok
            || validate_parameters(self, pp).is_err()
            || validate_pending_order(pp, opening).is_err()
        {
            return Ok(false);
        }
        let levels = pp.extension_sequence.levels();
        let terminal_level = pp.terminal_level;
        if proof.rounds.len() != levels.saturating_sub(terminal_level) {
            return Ok(false);
        }
        let mut transcript =
            OpeningTranscriptReader::with_domain(&proof.responses, DECIDE_TRANSCRIPT_DOMAIN);
        absorb_decide_statement_reader(pp, commitment, opening, &mut transcript);
        let mut proof_cursor = 0usize;
        let mut generator_ordinal = 0usize;
        let mut carried_generator = None;
        let mut carried_witness_claim = None;
        let mut carried_witness_root = None;

        for (round_index, level) in (terminal_level..levels).rev().enumerate() {
            let round = &proof.rounds[round_index];
            if round.level != level {
                return Ok(false);
            }
            let code = pp.extension_sequence.code(level)?;
            let setup = pp.setup_level(level)?;
            validate_setup_level(pp, level, setup)?;
            let mut claims = vec![opening_claim_at(opening, level)?];
            if let Some(claim) = carried_generator.take() {
                claims.push(claim);
            }
            let proof_end = proof_cursor
                .checked_add(claims.len())
                .ok_or(Error::EncodingError)?;
            let Some(round_proofs) = proof.generator_proofs.get(proof_cursor..proof_end) else {
                return Ok(false);
            };
            absorb_decide_round_context_reader(
                level,
                setup,
                carried_witness_root.as_ref(),
                claims.len(),
                &mut transcript,
            );
            let mut h_claims = Vec::with_capacity(claims.len());
            let mut auxiliary_claims = Vec::with_capacity(claims.len() * 2);
            let mut auxiliary_columns = Vec::with_capacity(claims.len() * 2);
            for (claim, generator_proof) in claims.into_iter().zip(round_proofs) {
                let pending = ShortRaaPendingGeneratorClaim { level, claim };
                let Some(output) = verify_generator_claim(
                    pp,
                    commitment,
                    &pending,
                    generator_ordinal,
                    generator_proof,
                    &mut transcript,
                )?
                else {
                    return Ok(false);
                };
                h_claims.extend(output.reduction.fixed_h_claims(code)?);
                auxiliary_claims.extend(output.auxiliary_claims);
                auxiliary_columns.extend(output.auxiliary_columns);
                generator_ordinal += 1;
            }
            proof_cursor = proof_end;

            let prepared = prepare_decide_claims(
                code,
                &h_claims,
                &auxiliary_claims,
                &auxiliary_columns,
                carried_witness_claim.as_ref(),
            )?;
            let multi_eval = verify_decide_multi_eval(
                &prepared.claims,
                prepared.rows,
                prepared.real_columns,
                prepared.padded_columns,
                &mut transcript,
            )?;
            if !multi_eval.ok {
                return Ok(false);
            }
            let (column_point, row_point) = multi_eval
                .reduced
                .point
                .split_at(prepared.padded_columns.ilog2() as usize);
            let selection = derive_direct_selection_reader(
                &pp.extension_sequence,
                level,
                pp.security_bits,
                &mut transcript,
            )?;
            let carry_rows = round.carry_rows.as_deref();
            let auxiliary_roots = round_proofs
                .iter()
                .flat_map(|generator| generator.auxiliary_roots.iter().copied())
                .collect::<Vec<_>>();
            let Some((iota, base_point)) = verify_direct_iota(
                pp,
                level,
                setup,
                &auxiliary_roots,
                &auxiliary_columns,
                carried_witness_root.as_ref(),
                &round.setup_rows,
                &round.auxiliary_rows,
                carry_rows,
                &selection,
                column_point,
                row_point,
                prepared.real_columns,
            )?
            else {
                return Ok(false);
            };
            transcript.absorb_extensions(b"decide-iota", &iota);

            let tail_root = if level == terminal_level {
                Some(proof.tail.root)
            } else {
                None
            };
            match (level > terminal_level, round.next_carry_root) {
                (true, Some(_)) | (false, None) => {}
                _ => return Ok(false),
            }
            absorb_decide_next_reader(
                round.next_carry_root.as_ref(),
                tail_root.as_ref(),
                &mut transcript,
            )?;
            let Some(switched) = verify_direct_code_switch(
                pp.extension_sequence.code(level - 1)?,
                level - 1,
                &selection,
                &iota,
                multi_eval.reduced.value,
                base_point,
                &mut transcript,
            )?
            else {
                return Ok(false);
            };

            if level > terminal_level {
                carried_generator = Some(switched.generator_claim);
                carried_witness_claim = Some(switched.folded_witness_claim);
                carried_witness_root = round.next_carry_root;
            } else {
                if verify_terminal_generator(pp.extension_sequence.code(level - 1)?, &switched)
                    .is_err()
                    || !verify_ligero_tail(
                        &pp.decide_tail,
                        &proof.tail,
                        &switched
                            .folded_witness_claim
                            .point
                            .iter()
                            .rev()
                            .copied()
                            .collect::<Vec<_>>(),
                        switched.folded_witness_claim.value,
                        pp.security_bits,
                        &mut transcript,
                    )?
                {
                    return Ok(false);
                }
                carried_witness_claim = None;
                carried_witness_root = None;
            }
        }
        Ok(proof_cursor == proof.generator_proofs.len()
            && carried_generator.is_none()
            && carried_witness_claim.is_none()
            && carried_witness_root.is_none()
            && transcript.is_exhausted())
    }
}

fn generator_statement(
    pp: &ShortRaaFoldParameters,
    commitment: &ShortRaaFoldCommitment,
    pending: &ShortRaaPendingGeneratorClaim,
    ordinal: usize,
) -> Vec<BinaryExtensionField> {
    let mut statement =
        Vec::with_capacity(10 + pending.claim.point.0.len() + pending.claim.point.1.len());
    statement.push(BinaryExtensionField::from(GENERATOR_STATEMENT_DOMAIN));
    statement.push(BinaryExtensionField::from(pending.level as u128));
    statement.push(BinaryExtensionField::from(ordinal as u128));
    statement.push(pending.claim.value);
    statement.extend(digest_fields(&pp.parameter_digest));
    statement.extend(digest_fields(&commitment.root()));
    statement.push(BinaryExtensionField::from(
        pending.claim.point.0.len() as u128
    ));
    statement.extend_from_slice(&pending.claim.point.0);
    statement.push(BinaryExtensionField::from(
        pending.claim.point.1.len() as u128
    ));
    statement.extend_from_slice(&pending.claim.point.1);
    statement
}

fn prove_generator_claim(
    pp: &ShortRaaFoldParameters,
    commitment: &ShortRaaFoldCommitment,
    pending: &ShortRaaPendingGeneratorClaim,
    ordinal: usize,
    transcript: &mut OpeningTranscriptWriter,
) -> Result<SwitchFoldGeneratorProverOutput, Error> {
    let profile_stages = std::env::var_os("RAAFOLD_PROFILE_STAGES").is_some();
    let total_start = Instant::now();
    let code = pp.extension_sequence.code(pending.level)?;
    transcript.absorb_extensions(
        b"generator-statement",
        &generator_statement(pp, commitment, pending, ordinal),
    );
    let specs = code.permutation_oracle_specs().to_vec();
    let reduce_start = Instant::now();
    let mut channel = SwitchFoldGeneratorProverChannel::new(
        &pp.extension_sequence,
        pending.level,
        specs,
        transcript,
    );
    let reduction = code.reduce_prover(&pending.claim, &mut channel)?;
    let reduce_time = reduce_start.elapsed();
    let finish_start = Instant::now();
    let (auxiliary_caches, auxiliary_claims) = channel.finish()?;
    let finish_time = finish_start.elapsed();
    if !reduction.ok || reduction.grand_product_count() != 1 {
        return Err(Error::InvalidCommitment);
    }
    let auxiliary_roots = auxiliary_caches
        .iter()
        .map(|auxiliary| auxiliary.cache.tree.root())
        .collect();
    if profile_stages {
        eprintln!(
            "[RAAFOLD_PROFILE] generator_level={} ordinal={} switchfold_reduce_s={:.9} relation_finish_s={:.9} generator_total_s={:.9}",
            pending.level,
            ordinal,
            reduce_time.as_secs_f64(),
            finish_time.as_secs_f64(),
            total_start.elapsed().as_secs_f64(),
        );
    }
    Ok(SwitchFoldGeneratorProverOutput {
        proof: ShortRaaGeneratorProof {
            level: pending.level,
            auxiliary_roots,
        },
        reduction,
        auxiliary_claims,
        auxiliary_caches,
    })
}

fn verify_generator_claim(
    pp: &ShortRaaFoldParameters,
    commitment: &ShortRaaFoldCommitment,
    pending: &ShortRaaPendingGeneratorClaim,
    ordinal: usize,
    proof: &ShortRaaGeneratorProof,
    transcript: &mut OpeningTranscriptReader<'_>,
) -> Result<Option<SwitchFoldGeneratorVerifierOutput>, Error> {
    if proof.level != pending.level {
        return Ok(None);
    }
    let code = pp.extension_sequence.code(pending.level)?;
    let specs = code.permutation_oracle_specs().to_vec();
    if proof.auxiliary_roots.len() != specs.len() {
        return Ok(None);
    }
    transcript.absorb_extensions(
        b"generator-statement",
        &generator_statement(pp, commitment, pending, ordinal),
    );
    let mut channel = SwitchFoldGeneratorVerifierChannel::new(
        &pp.extension_sequence,
        pending.level,
        specs.clone(),
        &proof.auxiliary_roots,
        transcript,
    );
    let Ok(reduction) = code.verify_as_subprotocol(&pending.claim, &mut channel) else {
        return Ok(None);
    };
    let Ok(auxiliary_claims) = channel.finish() else {
        return Ok(None);
    };
    if !reduction.ok || reduction.grand_product_count() != 1 {
        return Ok(None);
    }
    let logical_rows = pp
        .extension_sequence
        .direct_interleaved_level(pending.level)?
        .logical_rows()?;
    let auxiliary_columns = specs
        .iter()
        .map(|spec| {
            1usize
                .checked_shl(spec.log_msg_len as u32)
                .filter(|message_len| *message_len % logical_rows == 0)
                .map(|message_len| message_len / logical_rows)
                .ok_or(Error::InvalidNumberOfVariables)
        })
        .collect::<Result<Vec<_>, Error>>()?;
    Ok(Some(SwitchFoldGeneratorVerifierOutput {
        reduction,
        auxiliary_claims,
        auxiliary_columns,
    }))
}

fn validate_pending_order(
    pp: &ShortRaaFoldParameters,
    opening: &ShortRaaOpeningReduction,
) -> Result<(), Error> {
    let expected = pp.sequence.levels().saturating_sub(pp.terminal_level);
    if opening.pending_generator_claims.len() != expected {
        return Err(Error::InvalidCommitment);
    }
    for (pending, level) in opening
        .pending_generator_claims
        .iter()
        .zip((pp.terminal_level..pp.sequence.levels()).rev())
    {
        if pending.level != level {
            return Err(Error::InvalidCommitment);
        }
    }
    Ok(())
}

fn opening_claim_at(
    opening: &ShortRaaOpeningReduction,
    level: usize,
) -> Result<RaaMatrixClaim<BinaryExtensionField>, Error> {
    opening
        .pending_generator_claims
        .iter()
        .find(|pending| pending.level == level)
        .map(|pending| pending.claim.clone())
        .ok_or(Error::InvalidCommitment)
}

fn validate_setup_level(
    pp: &ShortRaaFoldParameters,
    level: usize,
    setup: &ShortRaaSetupLevel,
) -> Result<(), Error> {
    let direct = pp.extension_sequence.direct_interleaved_level(level)?;
    let current = pp.extension_sequence.code(level)?;
    let h_layout = current.fixed_h_layout();
    let shape = setup.shape;
    if shape.level != level
        || shape.logical_rows != current.message_len()
        || shape.logical_rows != direct.logical_rows()?
        || shape.encoding_message_rows != direct.encoding_message_rows()
        || shape.encoded_rows != direct.encoded_rows()
        || shape.logical_h_columns != h_layout.columns
        || shape.physical_h_columns != direct.physical_columns(h_layout.columns)?
    {
        return Err(Error::InvalidCommitment);
    }
    Ok(())
}

fn validate_prover_setup_level(
    setup: &ShortRaaSetupLevel,
    cache: &ExtensionCache,
) -> Result<(), Error> {
    let expected_len = setup
        .shape
        .encoded_rows
        .checked_mul(setup.shape.physical_h_columns)
        .ok_or(Error::EncodingError)?;
    if !cache.message_entries.is_empty()
        || cache.n_rows != setup.shape.encoded_rows
        || cache.n_cols != setup.shape.physical_h_columns
        || cache.encoded_entries.len() != expected_len
        || cache.tree.root() != setup.root
    {
        return Err(Error::InvalidCommitment);
    }
    Ok(())
}

fn absorb_decide_statement_writer(
    pp: &ShortRaaFoldParameters,
    commitment: &ShortRaaFoldCommitment,
    opening: &ShortRaaOpeningReduction,
    transcript: &mut OpeningTranscriptWriter,
) {
    transcript.absorb_digest(b"parameters", &pp.parameter_digest);
    transcript.absorb_digest(b"setup-commitments", &pp.setup_digest);
    transcript.absorb_digest(b"witness-root", &commitment.root());
    transcript.absorb_bytes(
        b"pending-count",
        &(opening.pending_generator_claims.len() as u64).to_le_bytes(),
    );
    for pending in &opening.pending_generator_claims {
        absorb_pending_claim_writer(pending, transcript);
    }
}

fn absorb_decide_statement_reader(
    pp: &ShortRaaFoldParameters,
    commitment: &ShortRaaFoldCommitment,
    opening: &ShortRaaOpeningReduction,
    transcript: &mut OpeningTranscriptReader<'_>,
) {
    transcript.absorb_digest(b"parameters", &pp.parameter_digest);
    transcript.absorb_digest(b"setup-commitments", &pp.setup_digest);
    transcript.absorb_digest(b"witness-root", &commitment.root());
    transcript.absorb_bytes(
        b"pending-count",
        &(opening.pending_generator_claims.len() as u64).to_le_bytes(),
    );
    for pending in &opening.pending_generator_claims {
        absorb_pending_claim_reader(pending, transcript);
    }
}

fn absorb_pending_claim_writer(
    pending: &ShortRaaPendingGeneratorClaim,
    transcript: &mut OpeningTranscriptWriter,
) {
    transcript.absorb_bytes(b"pending-level", &(pending.level as u64).to_le_bytes());
    transcript.absorb_field(b"pending-value", pending.claim.value);
    transcript.absorb_extensions(b"pending-left", &pending.claim.point.0);
    transcript.absorb_extensions(b"pending-right", &pending.claim.point.1);
}

fn absorb_pending_claim_reader(
    pending: &ShortRaaPendingGeneratorClaim,
    transcript: &mut OpeningTranscriptReader<'_>,
) {
    transcript.absorb_bytes(b"pending-level", &(pending.level as u64).to_le_bytes());
    transcript.absorb_field(b"pending-value", pending.claim.value);
    transcript.absorb_extensions(b"pending-left", &pending.claim.point.0);
    transcript.absorb_extensions(b"pending-right", &pending.claim.point.1);
}

fn absorb_decide_round_context_writer(
    level: usize,
    setup: &ShortRaaSetupLevel,
    carry: Option<&ExtensionCache>,
    generator_count: usize,
    transcript: &mut OpeningTranscriptWriter,
) {
    transcript.absorb_bytes(b"round-level", &(level as u64).to_le_bytes());
    absorb_setup_shape_writer(setup.shape, transcript);
    transcript.absorb_digest(b"setup-root", &setup.root);
    absorb_carry_root_writer(carry.map(|cache| cache.tree.root()), transcript);
    transcript.absorb_bytes(b"generator-count", &(generator_count as u64).to_le_bytes());
}

fn absorb_decide_round_context_reader(
    level: usize,
    setup: &ShortRaaSetupLevel,
    carry_root: Option<&[u8; DIGEST_BYTES]>,
    generator_count: usize,
    transcript: &mut OpeningTranscriptReader<'_>,
) {
    transcript.absorb_bytes(b"round-level", &(level as u64).to_le_bytes());
    absorb_setup_shape_reader(setup.shape, transcript);
    transcript.absorb_digest(b"setup-root", &setup.root);
    absorb_carry_root_reader(carry_root.copied(), transcript);
    transcript.absorb_bytes(b"generator-count", &(generator_count as u64).to_le_bytes());
}

fn absorb_auxiliary_commitment_writer(
    level: usize,
    index: usize,
    logical_columns: usize,
    cache: &ExtensionCache,
    transcript: &mut OpeningTranscriptWriter,
) {
    transcript.absorb_bytes(b"auxiliary-domain", b"raafold/switchfold-auxiliary/v1");
    for (label, value) in [
        (b"auxiliary-level".as_slice(), level),
        (b"auxiliary-index".as_slice(), index),
        (b"auxiliary-logical-columns".as_slice(), logical_columns),
        (b"auxiliary-encoded-rows".as_slice(), cache.n_rows),
        (b"auxiliary-physical-columns".as_slice(), cache.n_cols),
    ] {
        transcript.absorb_bytes(label, &(value as u64).to_le_bytes());
    }
    transcript.absorb_digest(b"auxiliary-root", &cache.tree.root());
}

fn absorb_auxiliary_commitment_reader(
    sequence: &RaaFold,
    level: usize,
    index: usize,
    logical_columns: usize,
    root: [u8; DIGEST_BYTES],
    transcript: &mut OpeningTranscriptReader<'_>,
) -> Result<(), Error> {
    let direct = sequence.direct_interleaved_level(level)?;
    let encoded_rows = direct.encoded_rows();
    let physical_columns = direct.physical_columns(logical_columns)?;
    transcript.absorb_bytes(b"auxiliary-domain", b"raafold/switchfold-auxiliary/v1");
    for (label, value) in [
        (b"auxiliary-level".as_slice(), level),
        (b"auxiliary-index".as_slice(), index),
        (b"auxiliary-logical-columns".as_slice(), logical_columns),
        (b"auxiliary-encoded-rows".as_slice(), encoded_rows),
        (b"auxiliary-physical-columns".as_slice(), physical_columns),
    ] {
        transcript.absorb_bytes(label, &(value as u64).to_le_bytes());
    }
    transcript.absorb_digest(b"auxiliary-root", &root);
    Ok(())
}

fn absorb_setup_shape_writer(
    shape: RaaDirectInterleavedSetupShape,
    transcript: &mut OpeningTranscriptWriter,
) {
    for (label, value) in setup_shape_fields(shape) {
        transcript.absorb_bytes(label, &(value as u64).to_le_bytes());
    }
}

fn absorb_setup_shape_reader(
    shape: RaaDirectInterleavedSetupShape,
    transcript: &mut OpeningTranscriptReader<'_>,
) {
    for (label, value) in setup_shape_fields(shape) {
        transcript.absorb_bytes(label, &(value as u64).to_le_bytes());
    }
}

fn setup_shape_fields(shape: RaaDirectInterleavedSetupShape) -> [(&'static [u8], usize); 6] {
    [
        (b"setup-level", shape.level),
        (b"setup-logical-rows", shape.logical_rows),
        (b"setup-message-rows", shape.encoding_message_rows),
        (b"setup-encoded-rows", shape.encoded_rows),
        (b"setup-logical-columns", shape.logical_h_columns),
        (b"setup-physical-columns", shape.physical_h_columns),
    ]
}

fn absorb_carry_root_writer(
    root: Option<[u8; DIGEST_BYTES]>,
    transcript: &mut OpeningTranscriptWriter,
) {
    match root {
        Some(root) => {
            transcript.absorb_bytes(b"carry-present", &[1]);
            transcript.absorb_digest(b"carry-root", &root);
        }
        None => transcript.absorb_bytes(b"carry-present", &[0]),
    }
}

fn absorb_carry_root_reader(
    root: Option<[u8; DIGEST_BYTES]>,
    transcript: &mut OpeningTranscriptReader<'_>,
) {
    match root {
        Some(root) => {
            transcript.absorb_bytes(b"carry-present", &[1]);
            transcript.absorb_digest(b"carry-root", &root);
        }
        None => transcript.absorb_bytes(b"carry-present", &[0]),
    }
}

fn prepare_decide_claims(
    code: &RaaCode,
    h_claims: &[RaaScaledMatrixClaim<BinaryExtensionField>],
    auxiliary_claims: &[RaaMatrixClaim<BinaryExtensionField>],
    auxiliary_columns: &[usize],
    carried_witness: Option<&RaaEvalWithPoint>,
) -> Result<RaaPreparedDecideClaims, Error> {
    if h_claims.is_empty() || auxiliary_claims.len() != auxiliary_columns.len() {
        return Err(Error::InvalidNumberOfVariables);
    }
    let h_layout = code.fixed_h_layout();
    let rows = code.message_len();
    let row_log = rows.ilog2() as usize;
    let padded_h_log = h_layout.padded_columns.ilog2() as usize;
    let auxiliary_width = auxiliary_columns.iter().try_fold(0usize, |sum, &columns| {
        if columns == 0 || !columns.is_power_of_two() {
            return Err(Error::InvalidNumberOfVariables);
        }
        sum.checked_add(columns).ok_or(Error::EncodingError)
    })?;
    let real_columns = h_layout
        .columns
        .checked_add(auxiliary_width)
        .ok_or(Error::EncodingError)?
        .checked_add(usize::from(carried_witness.is_some()))
        .ok_or(Error::EncodingError)?;
    let padded_columns = real_columns
        .checked_next_power_of_two()
        .ok_or(Error::EncodingError)?;
    let mut claims = Vec::with_capacity(
        h_claims.len() + auxiliary_claims.len() + usize::from(carried_witness.is_some()),
    );
    for h_claim in h_claims {
        if h_claim.point.0.len() != row_log || h_claim.point.1.len() != padded_h_log {
            return Err(Error::InvalidNumberOfVariables);
        }
        let h_weights = eq_ind_partial_eval_scalars(&h_claim.point.1);
        let mut col_weight = vec![BinaryExtensionField::ZERO; real_columns];
        col_weight[..h_layout.columns]
            .iter_mut()
            .zip(&h_weights[..h_layout.columns])
            .for_each(|(target, weight)| *target = h_claim.scale * *weight);
        claims.push(RaaEvalClaim {
            value: h_claim.value,
            row_point: h_claim.point.0.clone(),
            col_weight,
        });
    }
    let mut column_offset = h_layout.columns;
    for (auxiliary_claim, &columns) in auxiliary_claims.iter().zip(auxiliary_columns) {
        let column_log = columns.ilog2() as usize;
        if !auxiliary_claim.point.0.is_empty()
            || auxiliary_claim.point.1.len() != row_log + column_log
        {
            return Err(Error::InvalidNumberOfVariables);
        }
        let (row_point, column_point) = auxiliary_claim.point.1.split_at(row_log);
        let column_weights = eq_ind_partial_eval_scalars(column_point);
        let mut col_weight = vec![BinaryExtensionField::ZERO; real_columns];
        let end = column_offset
            .checked_add(columns)
            .ok_or(Error::EncodingError)?;
        col_weight[column_offset..end].copy_from_slice(&column_weights);
        claims.push(RaaEvalClaim {
            value: auxiliary_claim.value,
            row_point: row_point.to_vec(),
            col_weight,
        });
        column_offset = end;
    }
    if column_offset != h_layout.columns + auxiliary_width {
        return Err(Error::EncodingError);
    }
    if let Some(witness) = carried_witness {
        if witness.point.len() != row_log {
            return Err(Error::InvalidNumberOfVariables);
        }
        let mut col_weight = vec![BinaryExtensionField::ZERO; real_columns];
        col_weight[real_columns - 1] = BinaryExtensionField::ONE;
        claims.push(RaaEvalClaim {
            value: witness.value,
            row_point: witness.point.clone(),
            col_weight,
        });
    }
    Ok(RaaPreparedDecideClaims {
        claims,
        rows,
        real_columns,
        padded_columns,
    })
}

fn decide_matrix_entries(
    code: &RaaCode,
    auxiliary_caches: &[SwitchFoldAuxiliaryCache],
    carried_witness: Option<&[BinaryExtensionField]>,
    real_columns: usize,
    padded_columns: usize,
) -> Result<Vec<BinaryExtensionField>, Error> {
    let rows = code.message_len();
    let h_columns = code.fixed_h_layout().columns;
    let mut expected_columns = h_columns;
    for auxiliary in auxiliary_caches {
        let expected_len = rows
            .checked_mul(auxiliary.logical_columns)
            .ok_or(Error::EncodingError)?;
        if auxiliary.logical_columns == 0 || auxiliary.cache.message_entries.len() != expected_len {
            return Err(Error::InvalidNumberOfVariables);
        }
        expected_columns = expected_columns
            .checked_add(auxiliary.logical_columns)
            .ok_or(Error::EncodingError)?;
    }
    if let Some(witness) = carried_witness {
        if witness.len() != rows {
            return Err(Error::InvalidNumberOfVariables);
        }
        expected_columns = expected_columns
            .checked_add(1)
            .ok_or(Error::EncodingError)?;
    }
    if real_columns != expected_columns
        || real_columns > padded_columns
        || !padded_columns.is_power_of_two()
    {
        return Err(Error::InvalidNumberOfVariables);
    }
    let matrix_len = rows
        .checked_mul(padded_columns)
        .ok_or(Error::EncodingError)?;
    // Every chunk is one final power-of-two row. Each logical block is written
    // directly to its destination columns and the unused suffix remains zero.
    let mut entries = vec![BinaryExtensionField::ZERO; matrix_len];
    code.write_fixed_h_row_major(&mut entries, padded_columns)?;
    let fill_dynamic_columns = |row: usize, target: &mut [BinaryExtensionField]| {
        let mut column_offset = h_columns;
        for auxiliary in auxiliary_caches {
            for column in 0..auxiliary.logical_columns {
                target[column_offset + column] =
                    auxiliary.cache.message_entries[column * rows + row];
            }
            column_offset += auxiliary.logical_columns;
        }
        if let Some(witness) = carried_witness {
            target[column_offset] = witness[row];
            column_offset += 1;
        }
        debug_assert_eq!(column_offset, real_columns);
    };
    #[cfg(feature = "parallel")]
    entries
        .par_chunks_mut(padded_columns)
        .enumerate()
        .for_each(|(row, target)| fill_dynamic_columns(row, target));
    #[cfg(not(feature = "parallel"))]
    entries
        .chunks_mut(padded_columns)
        .enumerate()
        .for_each(|(row, target)| fill_dynamic_columns(row, target));
    Ok(entries)
}

fn prove_decide_multi_eval(
    claims: &[RaaEvalClaim],
    matrix_entries: Vec<BinaryExtensionField>,
    rows: usize,
    real_columns: usize,
    padded_columns: usize,
    transcript: &mut OpeningTranscriptWriter,
) -> Result<RaaMultiEvalProverResult, Error> {
    validate_decide_multi_eval(
        claims,
        matrix_entries.len(),
        rows,
        real_columns,
        padded_columns,
    )?;
    absorb_multi_eval_statement_writer(claims, rows, real_columns, padded_columns, transcript);
    let rho = transcript.challenge_non_singular(b"decide-multi-eval-batch")?;
    let (rhs, powers) = combined_eval_target(claims, rho);
    let total_len = rows
        .checked_mul(padded_columns)
        .ok_or(Error::EncodingError)?;
    let mut right = vec![BinaryExtensionField::ZERO; total_len];
    let batched_claims = claims
        .iter()
        .zip(&powers)
        .map(|(claim, power)| {
            (
                claim
                    .col_weight
                    .iter()
                    .map(|column_weight| *power * *column_weight)
                    .collect::<Vec<_>>(),
                eq_ind_partial_eval_scalars(&claim.row_point),
            )
        })
        .collect::<Vec<_>>();
    let fill_weight = |row: usize, column: usize| {
        let wide = batched_claims.iter().fold(
            BinaryWideProduct::default(),
            |sum, (column_coefficients, row_weights)| {
                let coefficient = column_coefficients[column];
                if coefficient == BinaryExtensionField::ZERO {
                    sum
                } else {
                    sum + BinaryExtensionField::wide_mul(coefficient, row_weights[row])
                }
            },
        );
        BinaryExtensionField::reduce(wide)
    };
    #[cfg(feature = "parallel")]
    if total_len >= PARALLEL_FIELD_THRESHOLD && rayon::current_num_threads() > 1 {
        right
            .par_chunks_mut(padded_columns)
            .enumerate()
            .for_each(|(row, target)| {
                target[..real_columns]
                    .iter_mut()
                    .enumerate()
                    .for_each(|(column, value)| *value = fill_weight(row, column));
            });
    } else {
        right
            .chunks_mut(padded_columns)
            .enumerate()
            .for_each(|(row, target)| {
                target[..real_columns]
                    .iter_mut()
                    .enumerate()
                    .for_each(|(column, value)| *value = fill_weight(row, column));
            });
    }
    #[cfg(not(feature = "parallel"))]
    right
        .chunks_mut(padded_columns)
        .enumerate()
        .for_each(|(row, target)| {
            target[..real_columns]
                .iter_mut()
                .enumerate()
                .for_each(|(column, value)| *value = fill_weight(row, column));
        });
    let (reduction, folded_vector) = prove_row_major_matrix_litebullet(
        rhs,
        matrix_entries,
        right,
        rows,
        padded_columns,
        transcript,
        b"decide-multi-eval-litebullet",
    )?;
    let expected_weight = expected_multi_eval_weight(
        claims,
        &powers,
        real_columns,
        padded_columns,
        rows,
        &reduction.b_point,
    )?;
    Ok(RaaMultiEvalProverResult {
        reduction: RaaMultiEvalReduction {
            ok: reduction.b_value == expected_weight,
            reduced: RaaEvalWithPoint {
                value: reduction.a_value,
                point: reduction.a_point,
            },
        },
        folded_vector,
    })
}

/// Computes a round over complete row-major tiles. Wide scalar accumulators
/// remain local to a tile for tiny rows; wider rows use the SIMD dot kernel.
fn matrix_litebullet_round_tile(
    left: &[BinaryExtensionField],
    right: &[BinaryExtensionField],
    row_stride: usize,
    active_columns: usize,
) -> (BinaryExtensionField, BinaryExtensionField) {
    let half_columns = active_columns / 2;
    if half_columns >= 4 {
        left.chunks_exact(row_stride)
            .zip(right.chunks_exact(row_stride))
            .fold(
                (BinaryExtensionField::ZERO, BinaryExtensionField::ZERO),
                |(high_sum, cross_sum), (left_row, right_row)| {
                    let (left_low, left_high) = left_row[..active_columns].split_at(half_columns);
                    let (right_low, right_high) =
                        right_row[..active_columns].split_at(half_columns);
                    (
                        high_sum + BinaryExtensionField::dot_product(left_high, right_high),
                        cross_sum
                            + BinaryExtensionField::dot_product(left_low, right_high)
                            + BinaryExtensionField::dot_product(left_high, right_low),
                    )
                },
            )
    } else {
        // One or two lanes do not amortize three SIMD dispatches/reductions.
        let mut high_sum = BinaryWideProduct::default();
        let mut cross_sum = BinaryWideProduct::default();
        for (left_row, right_row) in left
            .chunks_exact(row_stride)
            .zip(right.chunks_exact(row_stride))
        {
            let (left_low, left_high) = left_row[..active_columns].split_at(half_columns);
            let (right_low, right_high) = right_row[..active_columns].split_at(half_columns);
            for index in 0..half_columns {
                high_sum += BinaryExtensionField::wide_mul(left_high[index], right_high[index]);
                cross_sum += BinaryExtensionField::wide_mul(left_low[index], right_high[index]);
                cross_sum += BinaryExtensionField::wide_mul(left_high[index], right_low[index]);
            }
        }
        (
            BinaryExtensionField::reduce(high_sum),
            BinaryExtensionField::reduce(cross_sum),
        )
    }
}

fn matrix_litebullet_round(
    left: &[BinaryExtensionField],
    right: &[BinaryExtensionField],
    rows: usize,
    row_stride: usize,
    active_columns: usize,
) -> (BinaryExtensionField, BinaryExtensionField) {
    let half_columns = active_columns / 2;
    let tile_rows = (PARALLEL_FIELD_THRESHOLD / half_columns).max(1);
    let tile_len = row_stride * tile_rows.min(rows);
    let round_tile = |(left, right): (&[BinaryExtensionField], &[BinaryExtensionField])| {
        matrix_litebullet_round_tile(left, right, row_stride, active_columns)
    };
    let combine = |(a, b), (c, d)| (a + c, b + d);
    #[cfg(feature = "parallel")]
    if rows * half_columns >= PARALLEL_FIELD_THRESHOLD && rayon::current_num_threads() > 1 {
        return left
            .par_chunks(tile_len)
            .zip(right.par_chunks(tile_len))
            .map(round_tile)
            .reduce(
                || (BinaryExtensionField::ZERO, BinaryExtensionField::ZERO),
                combine,
            );
    }
    left.chunks(tile_len)
        .zip(right.chunks(tile_len))
        .map(round_tile)
        .fold(
            (BinaryExtensionField::ZERO, BinaryExtensionField::ZERO),
            combine,
        )
}

/// Equality-basis fold: low + (low + high) * r = (1 + r) * low + r * high.
fn fold_equality_slice(
    low: &mut [BinaryExtensionField],
    high: &[BinaryExtensionField],
    challenge: BinaryExtensionField,
) {
    BinaryExtensionField::fold_equality_slice(low, high, challenge);
}

fn fold_litebullet_pair(
    left_low: &mut [BinaryExtensionField],
    left_high: &[BinaryExtensionField],
    right_low: &mut [BinaryExtensionField],
    right_high: &[BinaryExtensionField],
    challenge: BinaryExtensionField,
) {
    let fold_chunk = |left_low: &mut [BinaryExtensionField],
                      left_high: &[BinaryExtensionField],
                      right_low: &mut [BinaryExtensionField],
                      right_high: &[BinaryExtensionField]| {
        fold_equality_slice(left_low, left_high, challenge);
        fold_equality_slice(right_low, right_high, challenge);
    };
    #[cfg(feature = "parallel")]
    if left_low.len() >= PARALLEL_FIELD_THRESHOLD && rayon::current_num_threads() > 1 {
        left_low
            .par_chunks_mut(PARALLEL_FIELD_THRESHOLD)
            .zip(left_high.par_chunks(PARALLEL_FIELD_THRESHOLD))
            .zip(right_low.par_chunks_mut(PARALLEL_FIELD_THRESHOLD))
            .zip(right_high.par_chunks(PARALLEL_FIELD_THRESHOLD))
            .for_each(|(((left_low, left_high), right_low), right_high)| {
                fold_chunk(left_low, left_high, right_low, right_high);
            });
        return;
    }
    left_low
        .chunks_mut(PARALLEL_FIELD_THRESHOLD)
        .zip(left_high.chunks(PARALLEL_FIELD_THRESHOLD))
        .zip(right_low.chunks_mut(PARALLEL_FIELD_THRESHOLD))
        .zip(right_high.chunks(PARALLEL_FIELD_THRESHOLD))
        .for_each(|(((left_low, left_high), right_low), right_high)| {
            fold_chunk(left_low, left_high, right_low, right_high);
        });
}

fn prove_row_major_matrix_litebullet(
    claimed_value: BinaryExtensionField,
    mut left_matrix: Vec<BinaryExtensionField>,
    mut right_matrix: Vec<BinaryExtensionField>,
    rows: usize,
    row_stride: usize,
    transcript: &mut OpeningTranscriptWriter,
    label: &[u8],
) -> Result<(LiteBulletReduction, Vec<BinaryExtensionField>), Error> {
    let matrix_len = rows.checked_mul(row_stride).ok_or(Error::EncodingError)?;
    if rows == 0
        || !rows.is_power_of_two()
        || row_stride == 0
        || !row_stride.is_power_of_two()
        || left_matrix.len() != matrix_len
        || right_matrix.len() != matrix_len
        || inner_product(&left_matrix, &right_matrix)? != claimed_value
    {
        return Err(Error::InvalidCommitment);
    }

    transcript.absorb_bytes(b"litebullet-domain", label);
    let mut rhs = claimed_value;
    let mut a_point = Vec::with_capacity(matrix_len.ilog2() as usize);
    let mut b_point = Vec::with_capacity(matrix_len.ilog2() as usize);
    let mut active_columns = row_stride;
    while active_columns > 1 {
        let half_columns = active_columns / 2;
        let (high_diagonal, cross_sum) = matrix_litebullet_round(
            &left_matrix,
            &right_matrix,
            rows,
            row_stride,
            active_columns,
        );
        transcript.push_response(high_diagonal);
        transcript.push_response(cross_sum);
        let challenge = transcript.challenge_non_singular(b"litebullet-fold")?;
        rhs = multilinear_inner_product_round(rhs, high_diagonal, cross_sum, challenge);

        let fold_row = |left_row: &mut [BinaryExtensionField],
                        right_row: &mut [BinaryExtensionField]| {
            let (left_active, _) = left_row.split_at_mut(active_columns);
            let (right_active, _) = right_row.split_at_mut(active_columns);
            let (left_low, left_high) = left_active.split_at_mut(half_columns);
            let (right_low, right_high) = right_active.split_at_mut(half_columns);
            fold_equality_slice(left_low, left_high, challenge);
            fold_equality_slice(right_low, right_high, challenge);
        };
        #[cfg(feature = "parallel")]
        if rows * half_columns >= PARALLEL_FIELD_THRESHOLD && rayon::current_num_threads() > 1 {
            left_matrix
                .par_chunks_mut(row_stride)
                .zip(right_matrix.par_chunks_mut(row_stride))
                .for_each(|(left_row, right_row)| fold_row(left_row, right_row));
        } else {
            left_matrix
                .chunks_mut(row_stride)
                .zip(right_matrix.chunks_mut(row_stride))
                .for_each(|(left_row, right_row)| fold_row(left_row, right_row));
        }
        #[cfg(not(feature = "parallel"))]
        left_matrix
            .chunks_mut(row_stride)
            .zip(right_matrix.chunks_mut(row_stride))
            .for_each(|(left_row, right_row)| fold_row(left_row, right_row));
        active_columns = half_columns;
        a_point.push(challenge);
        b_point.push(challenge);
    }

    let folded_vector = left_matrix
        .chunks(row_stride)
        .map(|row| row[0])
        .collect::<Vec<_>>();
    let mut left = folded_vector.clone();
    let mut right = right_matrix
        .chunks(row_stride)
        .map(|row| row[0])
        .collect::<Vec<_>>();
    while left.len() > 1 {
        let half = left.len() / 2;
        let high_diagonal = inner_product(&left[half..], &right[half..])?;
        let cross_sum = inner_product(&left[..half], &right[half..])?
            + inner_product(&left[half..], &right[..half])?;
        transcript.push_response(high_diagonal);
        transcript.push_response(cross_sum);
        let challenge = transcript.challenge_non_singular(b"litebullet-fold")?;
        rhs = multilinear_inner_product_round(rhs, high_diagonal, cross_sum, challenge);
        let (left_low, left_high) = left.split_at_mut(half);
        let (right_low, right_high) = right.split_at_mut(half);
        fold_litebullet_pair(left_low, left_high, right_low, right_high, challenge);
        left.truncate(half);
        right.truncate(half);
        a_point.push(challenge);
        b_point.push(challenge);
    }

    let a_value = left[0];
    let b_value = right[0];
    transcript.push_response(a_value);
    transcript.push_response(b_value);
    if a_value * b_value != rhs {
        return Err(Error::InvalidCommitment);
    }
    Ok((
        LiteBulletReduction {
            a_value,
            a_point,
            b_value,
            b_point,
        },
        folded_vector,
    ))
}

fn verify_decide_multi_eval(
    claims: &[RaaEvalClaim],
    rows: usize,
    real_columns: usize,
    padded_columns: usize,
    transcript: &mut OpeningTranscriptReader<'_>,
) -> Result<RaaMultiEvalReduction, Error> {
    validate_decide_multi_eval(
        claims,
        rows.checked_mul(padded_columns)
            .ok_or(Error::EncodingError)?,
        rows,
        real_columns,
        padded_columns,
    )?;
    absorb_multi_eval_statement_reader(claims, rows, real_columns, padded_columns, transcript);
    let rho = transcript.challenge_non_singular(b"decide-multi-eval-batch")?;
    let (rhs, powers) = combined_eval_target(claims, rho);
    let total_len = rows
        .checked_mul(padded_columns)
        .ok_or(Error::EncodingError)?;
    let Some(reduction) =
        verify_litebullet(rhs, total_len, transcript, b"decide-multi-eval-litebullet")?
    else {
        return Ok(RaaMultiEvalReduction {
            ok: false,
            reduced: RaaEvalWithPoint {
                value: BinaryExtensionField::ZERO,
                point: Vec::new(),
            },
        });
    };
    let expected_weight = expected_multi_eval_weight(
        claims,
        &powers,
        real_columns,
        padded_columns,
        rows,
        &reduction.b_point,
    )?;
    Ok(RaaMultiEvalReduction {
        ok: reduction.b_value == expected_weight,
        reduced: RaaEvalWithPoint {
            value: reduction.a_value,
            point: reduction.a_point,
        },
    })
}

fn validate_decide_multi_eval(
    claims: &[RaaEvalClaim],
    matrix_len: usize,
    rows: usize,
    real_columns: usize,
    padded_columns: usize,
) -> Result<(), Error> {
    if claims.is_empty()
        || rows == 0
        || !rows.is_power_of_two()
        || real_columns == 0
        || real_columns > padded_columns
        || !padded_columns.is_power_of_two()
        || matrix_len
            != rows
                .checked_mul(padded_columns)
                .ok_or(Error::EncodingError)?
        || claims.iter().any(|claim| {
            claim.row_point.len() != rows.ilog2() as usize || claim.col_weight.len() != real_columns
        })
    {
        return Err(Error::InvalidNumberOfVariables);
    }
    Ok(())
}

fn absorb_multi_eval_statement_writer(
    claims: &[RaaEvalClaim],
    rows: usize,
    real_columns: usize,
    padded_columns: usize,
    transcript: &mut OpeningTranscriptWriter,
) {
    transcript.absorb_bytes(
        b"multi-eval-domain",
        b"raafold/direct-multi-eval-row-major/v2",
    );
    for (label, value) in [
        (b"multi-eval-rows".as_slice(), rows),
        (b"multi-eval-real-columns".as_slice(), real_columns),
        (b"multi-eval-padded-columns".as_slice(), padded_columns),
        (b"multi-eval-claims".as_slice(), claims.len()),
    ] {
        transcript.absorb_bytes(label, &(value as u64).to_le_bytes());
    }
    for claim in claims {
        transcript.absorb_field(b"multi-eval-value", claim.value);
        transcript.absorb_extensions(b"multi-eval-row-point", &claim.row_point);
        transcript.absorb_extensions(b"multi-eval-column-weight", &claim.col_weight);
    }
}

fn absorb_multi_eval_statement_reader(
    claims: &[RaaEvalClaim],
    rows: usize,
    real_columns: usize,
    padded_columns: usize,
    transcript: &mut OpeningTranscriptReader<'_>,
) {
    transcript.absorb_bytes(
        b"multi-eval-domain",
        b"raafold/direct-multi-eval-row-major/v2",
    );
    for (label, value) in [
        (b"multi-eval-rows".as_slice(), rows),
        (b"multi-eval-real-columns".as_slice(), real_columns),
        (b"multi-eval-padded-columns".as_slice(), padded_columns),
        (b"multi-eval-claims".as_slice(), claims.len()),
    ] {
        transcript.absorb_bytes(label, &(value as u64).to_le_bytes());
    }
    for claim in claims {
        transcript.absorb_field(b"multi-eval-value", claim.value);
        transcript.absorb_extensions(b"multi-eval-row-point", &claim.row_point);
        transcript.absorb_extensions(b"multi-eval-column-weight", &claim.col_weight);
    }
}

fn combined_eval_target(
    claims: &[RaaEvalClaim],
    rho: BinaryExtensionField,
) -> (BinaryExtensionField, Vec<BinaryExtensionField>) {
    let mut power = BinaryExtensionField::ONE;
    let mut target = BinaryExtensionField::ZERO;
    let mut powers = Vec::with_capacity(claims.len());
    for claim in claims {
        powers.push(power);
        target += power * claim.value;
        power *= rho;
    }
    (target, powers)
}

fn expected_multi_eval_weight(
    claims: &[RaaEvalClaim],
    powers: &[BinaryExtensionField],
    real_columns: usize,
    padded_columns: usize,
    rows: usize,
    point: &[BinaryExtensionField],
) -> Result<BinaryExtensionField, Error> {
    let column_log = padded_columns.ilog2() as usize;
    let row_log = rows.ilog2() as usize;
    if point.len() != column_log + row_log || claims.len() != powers.len() {
        return Err(Error::InvalidNumberOfVariables);
    }
    let (column_point, row_point) = point.split_at(column_log);
    let column_weights = equality_tensor(column_point);
    Ok(claims
        .iter()
        .zip(powers)
        .map(|(claim, power)| {
            let column_eval = claim
                .col_weight
                .iter()
                .zip(&column_weights[..real_columns])
                .map(|(weight, evaluation)| *weight * *evaluation)
                .sum::<BinaryExtensionField>();
            let row_eval = equality_points_inner_product(&claim.row_point, row_point);
            *power * column_eval * row_eval
        })
        .sum())
}

fn equality_points_inner_product(
    low_to_high_point: &[BinaryExtensionField],
    high_to_low_point: &[BinaryExtensionField],
) -> BinaryExtensionField {
    debug_assert_eq!(low_to_high_point.len(), high_to_low_point.len());
    low_to_high_point
        .iter()
        .enumerate()
        .map(|(bit, value)| {
            let other = high_to_low_point[high_to_low_point.len() - bit - 1];
            BinaryExtensionField::ONE + *value + other
        })
        .product()
}

fn derive_direct_selection_writer(
    sequence: &RaaFold,
    level: usize,
    security_bits: usize,
    transcript: &mut OpeningTranscriptWriter,
) -> Result<Vec<usize>, Error> {
    let direct = sequence.direct_interleaved_level(level)?;
    let query_count = sequence.direct_interleaved_num_queries(level, security_bits)?;
    sample_without_replacement(
        query_count,
        direct.encoded_rows(),
        transcript.challenge_digest(b"decide-direct-row-selection"),
    )
}

fn derive_direct_selection_reader(
    sequence: &RaaFold,
    level: usize,
    security_bits: usize,
    transcript: &mut OpeningTranscriptReader<'_>,
) -> Result<Vec<usize>, Error> {
    let direct = sequence.direct_interleaved_level(level)?;
    let query_count = sequence.direct_interleaved_num_queries(level, security_bits)?;
    sample_without_replacement(
        query_count,
        direct.encoded_rows(),
        transcript.challenge_digest(b"decide-direct-row-selection"),
    )
}

#[allow(clippy::too_many_arguments)]
fn prove_direct_iota(
    pp: &ShortRaaFoldParameters,
    level: usize,
    setup: &ShortRaaSetupLevel,
    auxiliary: &[SwitchFoldAuxiliaryCache],
    carry: Option<&ExtensionCache>,
    setup_rows: &[ShortRaaExtensionRowQuery],
    auxiliary_rows: &[Vec<ShortRaaExtensionRowQuery>],
    carry_rows: Option<&[ShortRaaExtensionRowQuery]>,
    selection: &[usize],
    folded: &[BinaryExtensionField],
    column_point: &[BinaryExtensionField],
    row_point: &[BinaryExtensionField],
    real_columns: usize,
) -> Result<
    (
        Vec<BinaryExtensionField>,
        Vec<BinaryExtensionField>,
        Vec<BinaryExtensionField>,
    ),
    Error,
> {
    let direct = pp.extension_sequence.direct_interleaved_level(level)?;
    let logical_rows = direct.logical_rows()?;
    let row_factor = direct.row_factor();
    if auxiliary.len() != auxiliary_rows.len() {
        return Err(Error::InvalidNumberOfVariables);
    }
    let auxiliary_logical_columns = auxiliary.iter().try_fold(0usize, |sum, block| {
        sum.checked_add(block.logical_columns)
            .ok_or(Error::EncodingError)
    })?;
    let expected_real_columns = setup
        .shape
        .logical_h_columns
        .checked_add(auxiliary_logical_columns)
        .and_then(|columns| columns.checked_add(usize::from(carry.is_some())))
        .ok_or(Error::EncodingError)?;
    let carry_present = carry.is_some();
    if setup.shape.level != level
        || setup.shape.logical_rows != logical_rows
        || setup.shape.encoded_rows != direct.encoded_rows()
        || setup.shape.physical_h_columns
            != direct.physical_columns(setup.shape.logical_h_columns)?
        || carry_rows.is_some() != carry_present
        || real_columns != expected_real_columns
        || folded.len() != logical_rows
        || row_point.len() != logical_rows.ilog2() as usize
        || column_point.len()
            != real_columns
                .checked_next_power_of_two()
                .ok_or(Error::EncodingError)?
                .ilog2() as usize
    {
        return Err(Error::InvalidNumberOfVariables);
    }
    let mut auxiliary_physical_columns = Vec::with_capacity(auxiliary.len());
    for block in auxiliary {
        let physical_columns = direct.physical_columns(block.logical_columns)?;
        let expected_message_len = logical_rows
            .checked_mul(block.logical_columns)
            .ok_or(Error::EncodingError)?;
        let expected_encoded_len = direct
            .encoded_rows()
            .checked_mul(physical_columns)
            .ok_or(Error::EncodingError)?;
        if block.logical_columns == 0
            || block.cache.message_entries.len() != expected_message_len
            || block.cache.n_rows != direct.encoded_rows()
            || block.cache.n_cols != physical_columns
            || block.cache.encoded_entries.len() != expected_encoded_len
        {
            return Err(Error::InvalidCommitment);
        }
        auxiliary_physical_columns.push(physical_columns);
    }
    if let Some(carry) = carry {
        let expected_len = direct
            .encoded_rows()
            .checked_mul(row_factor)
            .ok_or(Error::EncodingError)?;
        if carry.message_entries.len() != logical_rows
            || carry.n_rows != direct.encoded_rows()
            || carry.n_cols != row_factor
            || carry.encoded_entries.len() != expected_len
            || carry.tree.root() == [0u8; DIGEST_BYTES]
        {
            return Err(Error::InvalidCommitment);
        }
    }

    let physical_weights =
        direct_equality_column_weights(column_point, row_point, real_columns, row_factor)?;
    if physical_weights.len() != direct.physical_columns(real_columns)? {
        return Err(Error::InvalidNumberOfVariables);
    }
    let iota = compute_direct_iota(
        setup_rows,
        auxiliary_rows,
        carry_rows,
        selection,
        setup.shape.encoded_rows,
        setup.shape.physical_h_columns,
        &auxiliary_physical_columns,
        usize::from(carry_present) * row_factor,
        &physical_weights,
    )?;
    let (recursive_vector, base_point) = fold_direct_equality_witness(
        folded,
        direct.encoding_message_rows(),
        row_factor,
        row_point,
    )?;
    if equality_mle_high_to_low(&recursive_vector, &base_point)
        != equality_mle_high_to_low(folded, row_point)
    {
        return Err(Error::InvalidCommitment);
    }
    Ok((iota, recursive_vector, base_point))
}

#[allow(clippy::too_many_arguments)]
fn verify_direct_iota(
    pp: &ShortRaaFoldParameters,
    level: usize,
    setup: &ShortRaaSetupLevel,
    auxiliary_roots: &[[u8; DIGEST_BYTES]],
    auxiliary_logical_columns: &[usize],
    carry_root: Option<&[u8; DIGEST_BYTES]>,
    setup_rows: &[ShortRaaExtensionRowQuery],
    auxiliary_rows: &[Vec<ShortRaaExtensionRowQuery>],
    carry_rows: Option<&[ShortRaaExtensionRowQuery]>,
    selection: &[usize],
    column_point: &[BinaryExtensionField],
    row_point: &[BinaryExtensionField],
    real_columns: usize,
) -> Result<Option<(Vec<BinaryExtensionField>, Vec<BinaryExtensionField>)>, Error> {
    let direct = pp.extension_sequence.direct_interleaved_level(level)?;
    let logical_rows = direct.logical_rows()?;
    let row_factor = direct.row_factor();
    if auxiliary_roots.len() != auxiliary_logical_columns.len()
        || auxiliary_roots.len() != auxiliary_rows.len()
    {
        return Ok(None);
    }
    let auxiliary_width = auxiliary_logical_columns
        .iter()
        .try_fold(0usize, |sum, &columns| {
            if columns == 0 {
                return Err(Error::InvalidNumberOfVariables);
            }
            sum.checked_add(columns).ok_or(Error::EncodingError)
        })?;
    let carry_present = carry_root.is_some();
    let expected_real_columns = setup
        .shape
        .logical_h_columns
        .checked_add(auxiliary_width)
        .and_then(|columns| columns.checked_add(usize::from(carry_present)))
        .ok_or(Error::EncodingError)?;
    if setup.shape.level != level
        || setup.shape.logical_rows != logical_rows
        || setup.shape.encoded_rows != direct.encoded_rows()
        || setup.shape.physical_h_columns
            != direct.physical_columns(setup.shape.logical_h_columns)?
        || carry_rows.is_some() != carry_present
        || real_columns != expected_real_columns
        || row_point.len() != logical_rows.ilog2() as usize
        || column_point.len()
            != real_columns
                .checked_next_power_of_two()
                .ok_or(Error::EncodingError)?
                .ilog2() as usize
    {
        return Ok(None);
    }
    if !verify_direct_rows(
        setup_rows,
        selection,
        setup.shape.encoded_rows,
        setup.shape.physical_h_columns,
        &setup.root,
    ) {
        return Ok(None);
    }
    let mut auxiliary_physical_columns = Vec::with_capacity(auxiliary_roots.len());
    for ((root, &logical_columns), rows) in auxiliary_roots
        .iter()
        .zip(auxiliary_logical_columns)
        .zip(auxiliary_rows)
    {
        let physical_columns = direct.physical_columns(logical_columns)?;
        if !verify_direct_rows(
            rows,
            selection,
            direct.encoded_rows(),
            physical_columns,
            root,
        ) {
            return Ok(None);
        }
        auxiliary_physical_columns.push(physical_columns);
    }
    if let (Some(root), Some(rows)) = (carry_root, carry_rows) {
        if !verify_direct_rows(rows, selection, direct.encoded_rows(), row_factor, root) {
            return Ok(None);
        }
    }
    let physical_weights =
        direct_equality_column_weights(column_point, row_point, real_columns, row_factor)?;
    if physical_weights.len() != direct.physical_columns(real_columns)? {
        return Ok(None);
    }
    let iota = compute_direct_iota(
        setup_rows,
        auxiliary_rows,
        carry_rows,
        selection,
        setup.shape.encoded_rows,
        setup.shape.physical_h_columns,
        &auxiliary_physical_columns,
        usize::from(carry_present) * row_factor,
        &physical_weights,
    )?;
    let block_log = row_factor.ilog2() as usize;
    Ok(Some((iota, row_point[block_log..].to_vec())))
}

fn direct_equality_column_weights(
    column_point: &[BinaryExtensionField],
    row_point: &[BinaryExtensionField],
    logical_columns: usize,
    row_factor: usize,
) -> Result<Vec<BinaryExtensionField>, Error> {
    if logical_columns == 0 || row_factor <= 1 || !row_factor.is_power_of_two() {
        return Err(Error::InvalidNumberOfVariables);
    }
    let logical_weights = equality_tensor(column_point);
    let block_log = row_factor.ilog2() as usize;
    if logical_columns > logical_weights.len() || row_point.len() < block_log {
        return Err(Error::InvalidNumberOfVariables);
    }
    let block_weights = equality_tensor(&row_point[..block_log]);
    Ok(logical_weights[..logical_columns]
        .iter()
        .flat_map(|logical_weight| {
            block_weights
                .iter()
                .map(move |block_weight| *logical_weight * *block_weight)
        })
        .collect())
}

fn compute_direct_iota(
    setup_rows: &[ShortRaaExtensionRowQuery],
    auxiliary_rows: &[Vec<ShortRaaExtensionRowQuery>],
    carry_rows: Option<&[ShortRaaExtensionRowQuery]>,
    selection: &[usize],
    encoded_rows: usize,
    setup_columns: usize,
    auxiliary_columns: &[usize],
    carry_columns: usize,
    physical_weights: &[BinaryExtensionField],
) -> Result<Vec<BinaryExtensionField>, Error> {
    if selection.is_empty()
        || setup_rows.len() != selection.len()
        || auxiliary_rows.len() != auxiliary_columns.len()
        || selection.iter().any(|&row| row >= encoded_rows)
        || setup_rows
            .iter()
            .any(|query| query.row.len() != setup_columns)
        || carry_rows.is_some_and(|rows| rows.len() != selection.len())
    {
        return Err(Error::InvalidNumberOfVariables);
    }
    for (rows, &columns) in auxiliary_rows.iter().zip(auxiliary_columns) {
        if columns == 0
            || rows.len() != selection.len()
            || rows.iter().any(|query| query.row.len() != columns)
        {
            return Err(Error::InvalidNumberOfVariables);
        }
    }
    if carry_rows.is_some() != (carry_columns > 0)
        || carry_rows.is_some_and(|rows| rows.iter().any(|query| query.row.len() != carry_columns))
    {
        return Err(Error::InvalidNumberOfVariables);
    }
    let expected_columns = auxiliary_columns
        .iter()
        .try_fold(setup_columns, |sum, &columns| {
            sum.checked_add(columns).ok_or(Error::EncodingError)
        })?;
    let expected_columns = expected_columns
        .checked_add(carry_columns)
        .ok_or(Error::EncodingError)?;
    if physical_weights.len() != expected_columns {
        return Err(Error::InvalidNumberOfVariables);
    }
    setup_rows
        .iter()
        .enumerate()
        .map(|(query_index, setup_query)| {
            let mut value = setup_query
                .row
                .iter()
                .zip(&physical_weights[..setup_columns])
                .map(|(&value, &weight)| value * weight)
                .sum::<BinaryExtensionField>();
            let mut offset = setup_columns;
            for (rows, &columns) in auxiliary_rows.iter().zip(auxiliary_columns) {
                let end = offset.checked_add(columns).ok_or(Error::EncodingError)?;
                value += rows[query_index]
                    .row
                    .iter()
                    .zip(&physical_weights[offset..end])
                    .map(|(&entry, &weight)| entry * weight)
                    .sum::<BinaryExtensionField>();
                offset = end;
            }
            if let Some(rows) = carry_rows {
                let end = offset
                    .checked_add(carry_columns)
                    .ok_or(Error::EncodingError)?;
                value += rows[query_index]
                    .row
                    .iter()
                    .zip(&physical_weights[offset..end])
                    .map(|(&entry, &weight)| entry * weight)
                    .sum::<BinaryExtensionField>();
                offset = end;
            }
            if offset != physical_weights.len() {
                return Err(Error::InvalidNumberOfVariables);
            }
            Ok(value)
        })
        .collect()
}

fn verify_direct_rows(
    queries: &[ShortRaaExtensionRowQuery],
    selection: &[usize],
    encoded_rows: usize,
    columns: usize,
    root: &[u8; DIGEST_BYTES],
) -> bool {
    queries.len() == selection.len()
        && queries.iter().zip(selection).all(|(query, &row)| {
            query.row.len() == columns
                && verify_extension_path(
                    hash_extension_row(&query.row),
                    row,
                    encoded_rows,
                    &query.path,
                    root,
                )
        })
}

fn fold_direct_equality_witness(
    folded: &[BinaryExtensionField],
    target_rows: usize,
    row_factor: usize,
    row_point: &[BinaryExtensionField],
) -> Result<(Vec<BinaryExtensionField>, Vec<BinaryExtensionField>), Error> {
    let logical_rows = target_rows
        .checked_mul(row_factor)
        .ok_or(Error::EncodingError)?;
    let block_log = row_factor.ilog2() as usize;
    if target_rows == 0
        || !target_rows.is_power_of_two()
        || row_factor <= 1
        || !row_factor.is_power_of_two()
        || folded.len() != logical_rows
        || row_point.len() != logical_rows.ilog2() as usize
    {
        return Err(Error::InvalidNumberOfVariables);
    }
    let block_weights = equality_tensor(&row_point[..block_log]);
    let fold_row = |row: usize| {
        block_weights
            .iter()
            .enumerate()
            .map(|(block, weight)| folded[block * target_rows + row] * *weight)
            .sum()
    };
    #[cfg(feature = "parallel")]
    let recursive = (0..target_rows).into_par_iter().map(fold_row).collect();
    #[cfg(not(feature = "parallel"))]
    let recursive = (0..target_rows).map(fold_row).collect();
    Ok((recursive, row_point[block_log..].to_vec()))
}

fn absorb_decide_next_writer(
    carry_root: Option<&[u8; DIGEST_BYTES]>,
    tail_root: Option<&[u8; DIGEST_BYTES]>,
    transcript: &mut OpeningTranscriptWriter,
) -> Result<(), Error> {
    transcript.absorb_bytes(b"decide-next-domain", b"raafold/direct-next/v1");
    match (carry_root, tail_root) {
        (Some(root), None) => {
            transcript.absorb_bytes(b"decide-next-kind", b"carry");
            transcript.absorb_digest(b"decide-next-root", root);
        }
        (None, Some(root)) => {
            transcript.absorb_bytes(b"decide-next-kind", b"ligero-tail");
            transcript.absorb_digest(b"decide-next-root", root);
        }
        _ => return Err(Error::InvalidCommitment),
    }
    Ok(())
}

fn absorb_decide_next_reader(
    carry_root: Option<&[u8; DIGEST_BYTES]>,
    tail_root: Option<&[u8; DIGEST_BYTES]>,
    transcript: &mut OpeningTranscriptReader<'_>,
) -> Result<(), Error> {
    transcript.absorb_bytes(b"decide-next-domain", b"raafold/direct-next/v1");
    match (carry_root, tail_root) {
        (Some(root), None) => {
            transcript.absorb_bytes(b"decide-next-kind", b"carry");
            transcript.absorb_digest(b"decide-next-root", root);
        }
        (None, Some(root)) => {
            transcript.absorb_bytes(b"decide-next-kind", b"ligero-tail");
            transcript.absorb_digest(b"decide-next-root", root);
        }
        _ => return Err(Error::InvalidCommitment),
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn prove_direct_code_switch(
    target: &RaaCode,
    target_level: usize,
    selection: &[usize],
    iota: &[BinaryExtensionField],
    recursive_vector: &[BinaryExtensionField],
    merged_value: BinaryExtensionField,
    base_point: Vec<BinaryExtensionField>,
    transcript: &mut OpeningTranscriptWriter,
) -> Result<RaaDirectCodeSwitchResult, Error> {
    validate_direct_code_switch(target, selection, iota, &base_point)?;
    if recursive_vector.len() != target.message_len()
        || equality_mle_high_to_low(recursive_vector, &base_point) != Some(merged_value)
    {
        return Err(Error::InvalidCommitment);
    }
    absorb_direct_code_switch_writer(
        target,
        target_level,
        selection,
        iota,
        merged_value,
        &base_point,
        transcript,
    );

    let selection_log = selection.len().next_power_of_two().ilog2() as usize;
    let selection_point = (0..selection_log)
        .map(|_| transcript.challenge_non_singular(b"direct-code-switch-selection-left"))
        .collect::<Result<Vec<_>, _>>()?;
    let selection_weights = homogeneous_tensor_prefix(&selection_point, selection.len())?;
    let selected_value = inner_product(&selection_weights, iota)?;
    let mut sparse_left = vec![BinaryExtensionField::ZERO; target.codeword_len()];
    for (&row, &weight) in selection.iter().zip(&selection_weights) {
        sparse_left[row] += weight;
    }
    let generator_left = target.generator_left_projection(&sparse_left)?;
    let rho = transcript.challenge_non_singular(b"direct-code-switch-opening-batch")?;
    let combined_value = selected_value + rho * merged_value;
    let combined_left = generator_left
        .into_iter()
        .zip(equality_tensor(&base_point))
        .map(|(selected, opened)| selected + rho * opened)
        .collect::<Vec<_>>();
    let message = prove_litebullet(
        combined_value,
        combined_left,
        recursive_vector.to_vec(),
        transcript,
        b"decide-direct-message-litebullet",
    )?;
    let opening_left =
        tensor_equality_inner_product(&base_point, OpeningPointBasis::Equality, &message.a_point)?;
    let selection_left = message.a_value - rho * opening_left;
    let generator_right = target.generator_right_projection(&equality_tensor(&message.a_point))?;
    let generator = prove_sparse_litebullet(
        selection_left,
        target.codeword_len(),
        selection,
        &selection_weights,
        generator_right,
        transcript,
        b"decide-direct-generator-litebullet",
    )?;
    direct_code_switch_result(&message, &generator)
}

#[allow(clippy::too_many_arguments)]
fn verify_direct_code_switch(
    target: &RaaCode,
    target_level: usize,
    selection: &[usize],
    iota: &[BinaryExtensionField],
    merged_value: BinaryExtensionField,
    base_point: Vec<BinaryExtensionField>,
    transcript: &mut OpeningTranscriptReader<'_>,
) -> Result<Option<RaaDirectCodeSwitchResult>, Error> {
    if validate_direct_code_switch(target, selection, iota, &base_point).is_err() {
        return Ok(None);
    }
    absorb_direct_code_switch_reader(
        target,
        target_level,
        selection,
        iota,
        merged_value,
        &base_point,
        transcript,
    );
    let selection_log = selection.len().next_power_of_two().ilog2() as usize;
    let selection_point = (0..selection_log)
        .map(|_| transcript.challenge_non_singular(b"direct-code-switch-selection-left"))
        .collect::<Result<Vec<_>, _>>()?;
    let selection_weights = homogeneous_tensor_prefix(&selection_point, selection.len())?;
    let selected_value = inner_product(&selection_weights, iota)?;
    let rho = transcript.challenge_non_singular(b"direct-code-switch-opening-batch")?;
    let combined_value = selected_value + rho * merged_value;
    let Some(message) = verify_litebullet(
        combined_value,
        target.message_len(),
        transcript,
        b"decide-direct-message-litebullet",
    )?
    else {
        return Ok(None);
    };
    let opening_left =
        tensor_equality_inner_product(&base_point, OpeningPointBasis::Equality, &message.a_point)?;
    let selection_left = message.a_value - rho * opening_left;
    let Some(generator) = verify_litebullet(
        selection_left,
        target.codeword_len(),
        transcript,
        b"decide-direct-generator-litebullet",
    )?
    else {
        return Ok(None);
    };
    let expected_sparse = selection
        .iter()
        .zip(&selection_weights)
        .map(|(&row, &weight)| weight * equality_tensor_weight_at(&generator.a_point, row))
        .sum::<BinaryExtensionField>();
    if generator.a_value != expected_sparse {
        return Ok(None);
    }
    Ok(Some(direct_code_switch_result(&message, &generator)?))
}

fn validate_direct_code_switch(
    target: &RaaCode,
    selection: &[usize],
    iota: &[BinaryExtensionField],
    base_point: &[BinaryExtensionField],
) -> Result<(), Error> {
    if selection.is_empty()
        || selection.len() != iota.len()
        || selection.iter().any(|&row| row >= target.codeword_len())
        || base_point.len() != target.message_len().ilog2() as usize
    {
        return Err(Error::InvalidNumberOfVariables);
    }
    Ok(())
}

fn direct_code_switch_result(
    message: &LiteBulletReduction,
    generator: &LiteBulletReduction,
) -> Result<RaaDirectCodeSwitchResult, Error> {
    Ok(RaaDirectCodeSwitchResult {
        generator_claim: RaaMatrixClaim {
            value: generator.b_value,
            point: (
                generator.b_point.iter().rev().copied().collect(),
                message.a_point.iter().rev().copied().collect(),
            ),
        },
        folded_witness_claim: RaaEvalWithPoint {
            value: message.b_value,
            point: message.b_point.iter().rev().copied().collect(),
        },
    })
}

#[allow(clippy::too_many_arguments)]
fn absorb_direct_code_switch_writer(
    target: &RaaCode,
    target_level: usize,
    selection: &[usize],
    iota: &[BinaryExtensionField],
    merged_value: BinaryExtensionField,
    base_point: &[BinaryExtensionField],
    transcript: &mut OpeningTranscriptWriter,
) {
    transcript.absorb_bytes(
        b"direct-code-switch-domain",
        b"raafold/direct-code-switch/v1",
    );
    absorb_direct_code_switch_public_writer(
        target,
        target_level,
        selection,
        iota,
        merged_value,
        base_point,
        transcript,
    );
}

#[allow(clippy::too_many_arguments)]
fn absorb_direct_code_switch_reader(
    target: &RaaCode,
    target_level: usize,
    selection: &[usize],
    iota: &[BinaryExtensionField],
    merged_value: BinaryExtensionField,
    base_point: &[BinaryExtensionField],
    transcript: &mut OpeningTranscriptReader<'_>,
) {
    transcript.absorb_bytes(
        b"direct-code-switch-domain",
        b"raafold/direct-code-switch/v1",
    );
    transcript.absorb_bytes(
        b"direct-code-switch-target-level",
        &(target_level as u64).to_le_bytes(),
    );
    transcript.absorb_bytes(
        b"direct-code-switch-message-len",
        &(target.message_len() as u64).to_le_bytes(),
    );
    transcript.absorb_bytes(
        b"direct-code-switch-codeword-len",
        &(target.codeword_len() as u64).to_le_bytes(),
    );
    transcript.absorb_bytes(
        b"direct-code-switch-rate-inv",
        &(target.rate_inv() as u64).to_le_bytes(),
    );
    transcript.absorb_digest(b"direct-code-switch-generator", &target.generator_digest());
    absorb_direct_selection_reader(selection, transcript);
    transcript.absorb_extensions(b"direct-code-switch-iota", iota);
    transcript.absorb_field(b"direct-code-switch-merged-value", merged_value);
    transcript.absorb_extensions(b"direct-code-switch-base-point", base_point);
}

#[allow(clippy::too_many_arguments)]
fn absorb_direct_code_switch_public_writer(
    target: &RaaCode,
    target_level: usize,
    selection: &[usize],
    iota: &[BinaryExtensionField],
    merged_value: BinaryExtensionField,
    base_point: &[BinaryExtensionField],
    transcript: &mut OpeningTranscriptWriter,
) {
    transcript.absorb_bytes(
        b"direct-code-switch-target-level",
        &(target_level as u64).to_le_bytes(),
    );
    transcript.absorb_bytes(
        b"direct-code-switch-message-len",
        &(target.message_len() as u64).to_le_bytes(),
    );
    transcript.absorb_bytes(
        b"direct-code-switch-codeword-len",
        &(target.codeword_len() as u64).to_le_bytes(),
    );
    transcript.absorb_bytes(
        b"direct-code-switch-rate-inv",
        &(target.rate_inv() as u64).to_le_bytes(),
    );
    transcript.absorb_digest(b"direct-code-switch-generator", &target.generator_digest());
    absorb_direct_selection_writer(selection, transcript);
    transcript.absorb_extensions(b"direct-code-switch-iota", iota);
    transcript.absorb_field(b"direct-code-switch-merged-value", merged_value);
    transcript.absorb_extensions(b"direct-code-switch-base-point", base_point);
}

fn absorb_direct_selection_writer(selection: &[usize], transcript: &mut OpeningTranscriptWriter) {
    transcript.absorb_bytes(
        b"direct-code-switch-selection-len",
        &(selection.len() as u64).to_le_bytes(),
    );
    for &row in selection {
        transcript.absorb_bytes(
            b"direct-code-switch-selection-row",
            &(row as u64).to_le_bytes(),
        );
    }
}

fn absorb_direct_selection_reader(
    selection: &[usize],
    transcript: &mut OpeningTranscriptReader<'_>,
) {
    transcript.absorb_bytes(
        b"direct-code-switch-selection-len",
        &(selection.len() as u64).to_le_bytes(),
    );
    for &row in selection {
        transcript.absorb_bytes(
            b"direct-code-switch-selection-row",
            &(row as u64).to_le_bytes(),
        );
    }
}

fn verify_terminal_generator(
    code: &RaaCode,
    switched: &RaaDirectCodeSwitchResult,
) -> Result<(), Error> {
    if code.evaluate_generator_mle(
        &switched.generator_claim.point.0,
        &switched.generator_claim.point.1,
    )? != switched.generator_claim.value
    {
        return Err(Error::InvalidCommitment);
    }
    Ok(())
}

fn verify_base_generator(code: &RaaCode, round: &VerifiedRound) -> Result<(), Error> {
    let claim = round.equality_generator_claim()?;
    if code.evaluate_generator_mle(&claim.point.0, &claim.point.1)? != claim.value {
        return Err(Error::InvalidCommitment);
    }
    Ok(())
}

fn prepare_next_layer(
    sequence: &RaaFold,
    next_level: Option<usize>,
    extension_columns: usize,
    folded: &[BinaryExtensionField],
) -> Result<(NextBinding, PreparedNextLayer), Error> {
    let level = next_level.ok_or(Error::EncodingError)?;
    let code = sequence.code(level)?;
    if folded.len()
        != code
            .message_len()
            .checked_mul(extension_columns)
            .ok_or(Error::EncodingError)?
    {
        return Err(Error::EncodingError);
    }
    let cache =
        commit_extension_matrix(code, folded.to_vec(), code.message_len(), extension_columns)?;
    Ok((
        NextBinding::Extension(cache.tree.root()),
        PreparedNextLayer::Extension(cache),
    ))
}

fn prepare_ligero_tail(
    tail: &ShortRaaLigeroTailParameters,
    folded: &[BinaryExtensionField],
) -> Result<(NextBinding, PreparedNextLayer), Error> {
    if folded.len() != tail.witness_len {
        return Err(Error::EncodingError);
    }
    let cache = commit_extension_matrix(&tail.code, folded.to_vec(), tail.height, tail.width)?;
    Ok((
        NextBinding::Ligero(cache.tree.root()),
        PreparedNextLayer::Ligero(cache),
    ))
}

fn prove_ligero_tail(
    tail: &ShortRaaLigeroTailParameters,
    cache: ExtensionCache,
    point: &[BinaryExtensionField],
    value: BinaryExtensionField,
    security_bits: usize,
    transcript: &mut OpeningTranscriptWriter,
) -> Result<ShortRaaLigeroTailProof, Error> {
    validate_ligero_tail_cache(tail, &cache)?;
    let width_log = tail.width.ilog2() as usize;
    if point.len() != tail.witness_len.ilog2() as usize {
        return Err(Error::InvalidNumberOfVariables);
    }
    let column_weights = equality_tensor(&point[..width_log]);
    let projection = fold_message_matrix(
        &cache.message_entries,
        tail.height,
        tail.width,
        &column_weights,
    )?;
    if inner_product(&projection, &equality_tensor(&point[width_log..]))? != value {
        return Err(Error::InvalidCommitment);
    }
    absorb_ligero_tail_context_writer(tail, &cache.tree.root(), transcript);
    for &entry in &projection {
        transcript.push_response(entry);
    }
    let selection = derive_selection_writer(
        &tail.code,
        security_bits,
        tail.code.codeword_len(),
        transcript,
    )?;
    let rows = open_extension_rows(&cache, &selection)?;
    Ok(ShortRaaLigeroTailProof {
        root: cache.tree.root(),
        rows,
    })
}

fn verify_ligero_tail(
    tail: &ShortRaaLigeroTailParameters,
    proof: &ShortRaaLigeroTailProof,
    point: &[BinaryExtensionField],
    value: BinaryExtensionField,
    security_bits: usize,
    transcript: &mut OpeningTranscriptReader<'_>,
) -> Result<bool, Error> {
    let width_log = tail.width.ilog2() as usize;
    if point.len() != tail.witness_len.ilog2() as usize {
        return Ok(false);
    }
    absorb_ligero_tail_context_reader(tail, &proof.root, transcript);
    let projection = (0..tail.height)
        .map(|_| transcript.read_response())
        .collect::<Result<Vec<_>, _>>()?;
    if inner_product(&projection, &equality_tensor(&point[width_log..]))? != value {
        return Ok(false);
    }
    let selection = derive_selection_reader(
        &tail.code,
        security_bits,
        tail.code.codeword_len(),
        transcript,
    )?;
    if proof.rows.len() != selection.len() {
        return Ok(false);
    }
    let encoded_projection = tail.code.encode(&projection)?;
    let column_weights = equality_tensor(&point[..width_log]);
    for (query, &row_index) in proof.rows.iter().zip(&selection) {
        if query.row.len() != tail.width
            || !verify_extension_path(
                hash_extension_row(&query.row),
                row_index,
                tail.code.codeword_len(),
                &query.path,
                &proof.root,
            )
            || inner_product(&query.row, &column_weights)? != encoded_projection[row_index]
        {
            return Ok(false);
        }
    }
    Ok(true)
}

fn validate_ligero_tail_cache(
    tail: &ShortRaaLigeroTailParameters,
    cache: &ExtensionCache,
) -> Result<(), Error> {
    if cache.message_entries.len() != tail.witness_len
        || cache.encoded_entries.len()
            != tail
                .code
                .codeword_len()
                .checked_mul(tail.width)
                .ok_or(Error::EncodingError)?
        || cache.n_rows != tail.code.codeword_len()
        || cache.n_cols != tail.width
        || cache.tree.root() == [0u8; DIGEST_BYTES]
    {
        return Err(Error::InvalidCommitment);
    }
    Ok(())
}

fn absorb_ligero_tail_context_writer(
    tail: &ShortRaaLigeroTailParameters,
    root: &[u8; DIGEST_BYTES],
    transcript: &mut OpeningTranscriptWriter,
) {
    transcript.absorb_bytes(b"ligero-tail-domain", LIGERO_TAIL_DOMAIN);
    absorb_ligero_tail_public_writer(tail, root, transcript);
}

fn absorb_ligero_tail_context_reader(
    tail: &ShortRaaLigeroTailParameters,
    root: &[u8; DIGEST_BYTES],
    transcript: &mut OpeningTranscriptReader<'_>,
) {
    transcript.absorb_bytes(b"ligero-tail-domain", LIGERO_TAIL_DOMAIN);
    transcript.absorb_bytes(
        b"ligero-tail-witness-len",
        &(tail.witness_len as u64).to_le_bytes(),
    );
    transcript.absorb_bytes(b"ligero-tail-height", &(tail.height as u64).to_le_bytes());
    transcript.absorb_bytes(b"ligero-tail-width", &(tail.width as u64).to_le_bytes());
    transcript.absorb_bytes(
        b"ligero-tail-rate-inv",
        &(tail.code.rate_inv() as u64).to_le_bytes(),
    );
    transcript.absorb_digest(b"ligero-tail-generator", &tail.code.generator_digest());
    transcript.absorb_digest(b"ligero-tail-root", root);
}

fn absorb_ligero_tail_public_writer(
    tail: &ShortRaaLigeroTailParameters,
    root: &[u8; DIGEST_BYTES],
    transcript: &mut OpeningTranscriptWriter,
) {
    transcript.absorb_bytes(
        b"ligero-tail-witness-len",
        &(tail.witness_len as u64).to_le_bytes(),
    );
    transcript.absorb_bytes(b"ligero-tail-height", &(tail.height as u64).to_le_bytes());
    transcript.absorb_bytes(b"ligero-tail-width", &(tail.width as u64).to_le_bytes());
    transcript.absorb_bytes(
        b"ligero-tail-rate-inv",
        &(tail.code.rate_inv() as u64).to_le_bytes(),
    );
    transcript.absorb_digest(b"ligero-tail-generator", &tail.code.generator_digest());
    transcript.absorb_digest(b"ligero-tail-root", root);
}

fn commit_extension_matrix(
    code: &RaaCode,
    message_entries: Vec<BinaryExtensionField>,
    n_message_rows: usize,
    n_cols: usize,
) -> Result<ExtensionCache, Error> {
    if n_message_rows != code.message_len()
        || n_cols == 0
        || message_entries.len() != n_message_rows * n_cols
    {
        return Err(Error::EncodingError);
    }
    let n_rows = code.codeword_len();
    let mut encoded_entries = vec![BinaryExtensionField::ZERO; n_rows * n_cols];
    code.layout()
        .encode_interleaved_column_major_into_row_major(
            &message_entries,
            n_cols,
            &mut encoded_entries,
        )?;
    let tree = build_extension_tree(&encoded_entries, n_rows, n_cols)?;
    Ok(ExtensionCache {
        message_entries,
        encoded_entries,
        n_rows,
        n_cols,
        tree,
    })
}

fn build_setup_levels(
    sequence: &RaaFold,
) -> Result<(Vec<Option<ShortRaaSetupLevel>>, Vec<Option<ExtensionCache>>), Error> {
    let mut public_levels = core::iter::repeat_with(|| None)
        .take(sequence.levels())
        .collect::<Vec<_>>();
    let mut setup_caches = core::iter::repeat_with(|| None)
        .take(sequence.levels())
        .collect::<Vec<_>>();
    for level in 1..sequence.levels() {
        let fixed_h = sequence.encode_direct_interleaved_fixed_h(level)?;
        let shape = fixed_h.shape();
        let encoded_entries = fixed_h.into_encoded_entries();
        let tree = build_extension_tree(
            &encoded_entries,
            shape.encoded_rows,
            shape.physical_h_columns,
        )?;
        let root = tree.root();
        public_levels[level] = Some(ShortRaaSetupLevel { shape, root });
        setup_caches[level] = Some(ExtensionCache {
            message_entries: Vec::new(),
            encoded_entries,
            n_rows: shape.encoded_rows,
            n_cols: shape.physical_h_columns,
            tree,
        });
    }
    Ok((public_levels, setup_caches))
}

fn setup_commitment_digest(
    levels: &[Option<ShortRaaSetupLevel>],
) -> Result<[u8; DIGEST_BYTES], Error> {
    if levels.is_empty() || levels[0].is_some() {
        return Err(Error::EncodingError);
    }
    let mut hasher = Blake2s256::new();
    absorb_framed(
        &mut hasher,
        b"domain",
        b"brakefold/raafold-short-pc/setup-commitments/v1",
    );
    absorb_framed(&mut hasher, b"levels", &(levels.len() as u64).to_le_bytes());
    for (level, setup) in levels.iter().enumerate().skip(1) {
        let setup = setup.as_ref().ok_or(Error::EncodingError)?;
        if setup.shape.level != level {
            return Err(Error::EncodingError);
        }
        for (label, value) in [
            (b"level".as_slice(), setup.shape.level),
            (b"logical-rows".as_slice(), setup.shape.logical_rows),
            (
                b"encoding-message-rows".as_slice(),
                setup.shape.encoding_message_rows,
            ),
            (b"encoded-rows".as_slice(), setup.shape.encoded_rows),
            (
                b"logical-h-columns".as_slice(),
                setup.shape.logical_h_columns,
            ),
            (
                b"physical-h-columns".as_slice(),
                setup.shape.physical_h_columns,
            ),
        ] {
            absorb_framed(&mut hasher, label, &(value as u64).to_le_bytes());
        }
        absorb_framed(&mut hasher, b"root", &setup.root);
    }
    Ok(hasher.finalize().into())
}

fn commit_direct_carry(
    sequence: &RaaFold,
    source_level: usize,
    message_entries: Vec<BinaryExtensionField>,
) -> Result<ExtensionCache, Error> {
    let auxiliary = commit_direct_interleaved_matrix(sequence, source_level, message_entries)?;
    if auxiliary.logical_columns != 1 {
        return Err(Error::InvalidNumberOfVariables);
    }
    Ok(auxiliary.cache)
}

fn commit_direct_interleaved_matrix(
    sequence: &RaaFold,
    source_level: usize,
    message_entries: Vec<BinaryExtensionField>,
) -> Result<SwitchFoldAuxiliaryCache, Error> {
    let direct = sequence.direct_interleaved_level(source_level)?;
    let logical_rows = direct.logical_rows()?;
    if message_entries.is_empty() || message_entries.len() % logical_rows != 0 {
        return Err(Error::InvalidNumberOfVariables);
    }
    let logical_columns = message_entries.len() / logical_rows;
    let n_rows = direct.encoded_rows();
    let n_cols = direct.physical_columns(logical_columns)?;
    let encoded_len = n_rows.checked_mul(n_cols).ok_or(Error::EncodingError)?;
    let encode_start = Instant::now();
    let mut encoded_entries = vec![BinaryExtensionField::ZERO; encoded_len];
    direct.encode_column_major_into_row_major(
        &message_entries,
        logical_columns,
        &mut encoded_entries,
    )?;
    let encode_time = encode_start.elapsed();
    let merkle_start = Instant::now();
    let tree = build_extension_tree(&encoded_entries, n_rows, n_cols)?;
    if std::env::var_os("RAAFOLD_PROFILE_STAGES").is_some() {
        eprintln!(
            "[RAAFOLD_PROFILE] auxiliary_commit level={source_level} rows={n_rows} columns={n_cols} encoded_bytes={} encode_s={:.9} merkle_s={:.9}",
            encoded_len * FIELD_BYTES,
            encode_time.as_secs_f64(),
            merkle_start.elapsed().as_secs_f64(),
        );
    }
    Ok(SwitchFoldAuxiliaryCache {
        logical_columns,
        cache: ExtensionCache {
            message_entries,
            encoded_entries,
            n_rows,
            n_cols,
            tree,
        },
    })
}

fn open_extension_rows(
    cache: &ExtensionCache,
    selection: &[usize],
) -> Result<Vec<ShortRaaExtensionRowQuery>, Error> {
    selection
        .iter()
        .map(|&row| {
            Ok(ShortRaaExtensionRowQuery {
                row: extension_row(&cache.encoded_entries, cache.n_rows, cache.n_cols, row)?,
                path: cache.tree.path(row)?,
            })
        })
        .collect()
}

#[derive(Clone, Debug)]
struct ExtensionMerkleTree {
    levels: Vec<Vec<[u8; DIGEST_BYTES]>>,
}

impl ExtensionMerkleTree {
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
                .map(|pair| hash_inner(&pair[0], &pair[1]))
                .collect();
            #[cfg(not(feature = "parallel"))]
            let parents = children
                .chunks_exact(2)
                .map(|pair| hash_inner(&pair[0], &pair[1]))
                .collect();
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

fn build_extension_tree(
    entries: &[BinaryExtensionField],
    n_rows: usize,
    n_cols: usize,
) -> Result<ExtensionMerkleTree, Error> {
    if n_rows == 0 || !n_rows.is_power_of_two() || n_cols == 0 || entries.len() != n_rows * n_cols {
        return Err(Error::EncodingError);
    }
    let prefix = extension_leaf_prefix(n_cols);
    #[cfg(feature = "parallel")]
    let leaves = entries
        .par_chunks_exact(n_cols)
        .map(|row| hash_extension_row_from_prefix(prefix.clone(), row))
        .collect();
    #[cfg(not(feature = "parallel"))]
    let leaves = entries
        .chunks_exact(n_cols)
        .map(|row| hash_extension_row_from_prefix(prefix.clone(), row))
        .collect();
    ExtensionMerkleTree::from_leaves(leaves)
}

fn extension_leaf_prefix(n_cols: usize) -> Blake2s256 {
    let mut hasher = Blake2s256::new();
    absorb_framed(&mut hasher, b"domain", EXTENSION_LEAF_DOMAIN);
    absorb_framed(&mut hasher, b"width", &(n_cols as u64).to_le_bytes());
    hasher
}

fn hash_extension_row_from_prefix(
    mut hasher: Blake2s256,
    row: &[BinaryExtensionField],
) -> [u8; DIGEST_BYTES] {
    #[cfg(target_endian = "little")]
    {
        const { assert!(core::mem::size_of::<BinaryExtensionField>() == FIELD_BYTES) };
        // SAFETY: BinaryExtensionField is repr(transparent) over u128, with no
        // padding or uninitialized bytes. On little-endian targets this borrowed
        // byte view is exactly the concatenation of val().to_le_bytes(), in the
        // existing row order. The view neither outlives nor mutates the row.
        let bytes = unsafe {
            core::slice::from_raw_parts(row.as_ptr().cast::<u8>(), core::mem::size_of_val(row))
        };
        hasher.update(bytes);
    }
    #[cfg(not(target_endian = "little"))]
    for &value in row {
        hasher.update(u128::from(value).to_le_bytes());
    }
    hasher.finalize().into()
}

fn extension_row(
    entries: &[BinaryExtensionField],
    n_rows: usize,
    n_cols: usize,
    row: usize,
) -> Result<Vec<BinaryExtensionField>, Error> {
    if row >= n_rows || entries.len() != n_rows * n_cols {
        return Err(Error::EncodingError);
    }
    Ok(entries[row * n_cols..(row + 1) * n_cols].to_vec())
}

fn hash_extension_row(row: &[BinaryExtensionField]) -> [u8; DIGEST_BYTES] {
    hash_extension_row_from_prefix(extension_leaf_prefix(row.len()), row)
}

fn hash_inner(left: &[u8; DIGEST_BYTES], right: &[u8; DIGEST_BYTES]) -> [u8; DIGEST_BYTES] {
    let mut hasher = Blake2s256::new();
    absorb_framed(&mut hasher, b"domain", INNER_DOMAIN);
    hasher.update(left);
    hasher.update(right);
    hasher.finalize().into()
}

fn verify_extension_path(
    mut digest: [u8; DIGEST_BYTES],
    mut index: usize,
    leaf_count: usize,
    path: &ShortRaaMerklePath,
    root: &[u8; DIGEST_BYTES],
) -> bool {
    if leaf_count == 0
        || !leaf_count.is_power_of_two()
        || index >= leaf_count
        || path.siblings.len() != leaf_count.ilog2() as usize
    {
        return false;
    }
    for sibling in &path.siblings {
        digest = if index & 1 == 0 {
            hash_inner(&digest, sibling)
        } else {
            hash_inner(sibling, &digest)
        };
        index >>= 1;
    }
    digest == *root
}

fn fold_packed_query_row(
    packed_row: &[u128],
    element_bits: usize,
    weights: &[BinaryExtensionField],
) -> Result<BinaryExtensionField, Error> {
    if !matches!(element_bits, 1 | 2 | 4 | 8 | 16 | 32 | 64 | 128) {
        return Err(Error::EncodingError);
    }
    let values_per_word = 128 / element_bits;
    if weights.len() != packed_row.len() * values_per_word {
        return Err(Error::InvalidNumberOfVariables);
    }
    let mask = if element_bits == 128 {
        u128::MAX
    } else {
        (1u128 << element_bits) - 1
    };
    let wide = packed_row.iter().enumerate().fold(
        BinaryWideProduct::default(),
        |mut sum, (packed_column, &word)| {
            for lane in 0..values_per_word {
                let logical_column = packed_column * values_per_word + lane;
                let value = (word >> (lane * element_bits)) & mask;
                sum += BinaryExtensionField::wide_mul(
                    BinaryExtensionField::from(value),
                    weights[logical_column],
                );
            }
            sum
        },
    );
    Ok(BinaryExtensionField::reduce(wide))
}

fn systematic_row_projection(
    entries: &[BinaryExtensionField],
    n_rows: usize,
    n_cols: usize,
    row_weights: &[BinaryExtensionField],
) -> Result<Vec<BinaryExtensionField>, Error> {
    if entries.len() != n_rows * n_cols || row_weights.len() != n_rows {
        return Err(Error::InvalidNumberOfVariables);
    }
    let project = |column: usize| {
        BinaryExtensionField::reduce(serial_wide_inner_product(
            &entries[column * n_rows..(column + 1) * n_rows],
            row_weights,
        ))
    };
    #[cfg(feature = "parallel")]
    let result = (0..n_cols).into_par_iter().map(project).collect();
    #[cfg(not(feature = "parallel"))]
    let result = (0..n_cols).map(project).collect();
    Ok(result)
}

fn fold_message_matrix(
    entries: &[BinaryExtensionField],
    n_rows: usize,
    n_cols: usize,
    weights: &[BinaryExtensionField],
) -> Result<Vec<BinaryExtensionField>, Error> {
    if entries.len() != n_rows * n_cols || weights.len() != n_cols {
        return Err(Error::InvalidNumberOfVariables);
    }
    let fold = |row: usize| {
        let wide = (0..n_cols).fold(BinaryWideProduct::default(), |sum, column| {
            sum + BinaryExtensionField::wide_mul(entries[column * n_rows + row], weights[column])
        });
        BinaryExtensionField::reduce(wide)
    };
    #[cfg(feature = "parallel")]
    let result = (0..n_rows).into_par_iter().map(fold).collect();
    #[cfg(not(feature = "parallel"))]
    let result = (0..n_rows).map(fold).collect();
    Ok(result)
}

fn homogeneous_tensor(point: &[BinaryExtensionField]) -> Vec<BinaryExtensionField> {
    let mut weights = vec![BinaryExtensionField::ONE];
    for &challenge in point {
        let mut next = Vec::with_capacity(weights.len() * 2);
        for weight in weights {
            next.push(weight);
            next.push(weight * challenge);
        }
        weights = next;
    }
    weights
}

fn equality_tensor(point: &[BinaryExtensionField]) -> Vec<BinaryExtensionField> {
    let mut weights = vec![BinaryExtensionField::ONE];
    for &challenge in point {
        let mut next = Vec::with_capacity(weights.len() * 2);
        for weight in weights {
            next.push(weight * (BinaryExtensionField::ONE + challenge));
            next.push(weight * challenge);
        }
        weights = next;
    }
    weights
}

/// Computes a tensor inner product in logarithmic time without expanding rows.
fn tensor_equality_inner_product(
    left: &[BinaryExtensionField],
    basis: OpeningPointBasis,
    right: &[BinaryExtensionField],
) -> Result<BinaryExtensionField, Error> {
    if left.len() != right.len() {
        return Err(Error::InvalidNumberOfVariables);
    }
    Ok(left
        .iter()
        .zip(right)
        .map(|(&x, &y)| match basis {
            OpeningPointBasis::Homogeneous => BinaryExtensionField::ONE + y + x * y,
            OpeningPointBasis::Equality => BinaryExtensionField::ONE + x + y,
        })
        .product())
}

fn tensor_weights(
    point: &[BinaryExtensionField],
    basis: OpeningPointBasis,
) -> Vec<BinaryExtensionField> {
    match basis {
        OpeningPointBasis::Homogeneous => homogeneous_tensor(point),
        OpeningPointBasis::Equality => equality_tensor(point),
    }
}

fn homogeneous_tensor_prefix(
    point: &[BinaryExtensionField],
    prefix_len: usize,
) -> Result<Vec<BinaryExtensionField>, Error> {
    let full_len = 1usize
        .checked_shl(point.len() as u32)
        .ok_or(Error::InvalidNumberOfVariables)?;
    if prefix_len == 0 || prefix_len > full_len {
        return Err(Error::InvalidNumberOfVariables);
    }
    Ok((0..prefix_len)
        .map(|index| homogeneous_tensor_weight_at(point, index))
        .collect())
}

fn homogeneous_tensor_weight_at(
    point: &[BinaryExtensionField],
    position: usize,
) -> BinaryExtensionField {
    let mut result = BinaryExtensionField::ONE;
    for bit in 0..point.len() {
        if (position >> bit) & 1 == 1 {
            result *= point[point.len() - bit - 1];
        }
    }
    result
}

fn equality_tensor_weight_at(
    point: &[BinaryExtensionField],
    position: usize,
) -> BinaryExtensionField {
    let mut result = BinaryExtensionField::ONE;
    for bit in 0..point.len() {
        let coordinate = point[point.len() - bit - 1];
        result *= if (position >> bit) & 1 == 1 {
            coordinate
        } else {
            BinaryExtensionField::ONE + coordinate
        };
    }
    result
}

fn equality_mle_high_to_low(
    values: &[BinaryExtensionField],
    point: &[BinaryExtensionField],
) -> Option<BinaryExtensionField> {
    if values.is_empty() || values.len() != 1usize.checked_shl(point.len() as u32)? {
        return None;
    }
    inner_product(values, &equality_tensor(point)).ok()
}

fn inner_product(
    left: &[BinaryExtensionField],
    right: &[BinaryExtensionField],
) -> Result<BinaryExtensionField, Error> {
    if left.len() != right.len() {
        return Err(Error::InvalidNumberOfVariables);
    }
    #[cfg(feature = "parallel")]
    if left.len() >= PARALLEL_FIELD_THRESHOLD && rayon::current_num_threads() > 1 {
        return Ok(left
            .par_chunks(PARALLEL_FIELD_THRESHOLD)
            .zip(right.par_chunks(PARALLEL_FIELD_THRESHOLD))
            .map(|(left, right)| BinaryExtensionField::dot_product(left, right))
            .reduce(|| BinaryExtensionField::ZERO, |left, right| left + right));
    }
    Ok(BinaryExtensionField::dot_product(left, right))
}

fn serial_wide_inner_product(
    left: &[BinaryExtensionField],
    right: &[BinaryExtensionField],
) -> BinaryWideProduct {
    left.iter()
        .zip(right)
        .fold(BinaryWideProduct::default(), |sum, (&left, &right)| {
            sum + BinaryExtensionField::wide_mul(left, right)
        })
}

fn derive_selection_writer(
    code: &RaaCode,
    security_bits: usize,
    available_rows: usize,
    transcript: &mut OpeningTranscriptWriter,
) -> Result<Vec<usize>, Error> {
    let query_count = code.num_queries(security_bits, available_rows)?;
    sample_without_replacement(
        query_count,
        available_rows,
        transcript.challenge_digest(b"row-selection"),
    )
}

fn derive_selection_reader(
    code: &RaaCode,
    security_bits: usize,
    available_rows: usize,
    transcript: &mut OpeningTranscriptReader<'_>,
) -> Result<Vec<usize>, Error> {
    let query_count = code.num_queries(security_bits, available_rows)?;
    sample_without_replacement(
        query_count,
        available_rows,
        transcript.challenge_digest(b"row-selection"),
    )
}

fn sample_without_replacement(
    count: usize,
    population: usize,
    seed: [u8; DIGEST_BYTES],
) -> Result<Vec<usize>, Error> {
    if count == 0 || count > population {
        return Err(Error::InvalidNumberOfVariables);
    }
    if count == population {
        return Ok((0..population).collect());
    }
    use ark_std::collections::BTreeMap;
    let mut swaps = BTreeMap::new();
    let mut output = Vec::with_capacity(count);
    let mut counter = 0u64;
    for offset in 0..count {
        let remaining = population - offset;
        let sampled = sample_below(seed, &mut counter, remaining)?;
        let last = remaining - 1;
        let sampled_value = *swaps.get(&sampled).unwrap_or(&sampled);
        let last_value = *swaps.get(&last).unwrap_or(&last);
        swaps.insert(sampled, last_value);
        output.push(sampled_value);
    }
    Ok(output)
}

fn sample_below(seed: [u8; DIGEST_BYTES], counter: &mut u64, upper: usize) -> Result<usize, Error> {
    if upper == 0 {
        return Err(Error::InvalidNumberOfVariables);
    }
    let range = 1u128 << 64;
    let upper_u128 = upper as u128;
    let acceptance_limit = range - range % upper_u128;
    loop {
        let mut hasher = Blake2s256::new();
        hasher.update(b"raafold-short-pc-index/v1");
        hasher.update(seed);
        hasher.update(counter.to_le_bytes());
        *counter = counter.checked_add(1).ok_or(Error::TranscriptError)?;
        let digest = hasher.finalize();
        let candidate =
            u64::from_le_bytes(digest[..8].try_into().map_err(|_| Error::TranscriptError)?) as u128;
        if candidate < acceptance_limit {
            return Ok((candidate % upper_u128) as usize);
        }
    }
}

fn absorb_next_binding_writer(transcript: &mut OpeningTranscriptWriter, binding: NextBinding) {
    match binding {
        NextBinding::Extension(root) => {
            transcript.absorb_bytes(b"next-kind", b"extension");
            transcript.absorb_digest(b"next-root", &root);
        }
        NextBinding::Ligero(root) => {
            transcript.absorb_bytes(b"next-kind", b"ligero-tail");
            transcript.absorb_digest(b"next-root", &root);
        }
    }
}

fn absorb_next_binding_reader(transcript: &mut OpeningTranscriptReader<'_>, binding: NextBinding) {
    match binding {
        NextBinding::Extension(root) => {
            transcript.absorb_bytes(b"next-kind", b"extension");
            transcript.absorb_digest(b"next-root", &root);
        }
        NextBinding::Ligero(root) => {
            transcript.absorb_bytes(b"next-kind", b"ligero-tail");
            transcript.absorb_digest(b"next-root", &root);
        }
    }
}

fn absorb_opening_statement_writer(
    pp: &ShortRaaFoldParameters,
    commitment: &ShortRaaFoldCommitment,
    point: &[BinaryExtensionField],
    value: BinaryExtensionField,
    transcript: &mut OpeningTranscriptWriter,
) {
    transcript.absorb_digest(b"parameters", &pp.parameter_digest);
    absorb_commitment_writer(commitment, transcript);
    transcript.absorb_extensions(b"point", point);
    transcript.absorb_field(b"claimed-value", value);
}

fn absorb_opening_statement_reader(
    pp: &ShortRaaFoldParameters,
    commitment: &ShortRaaFoldCommitment,
    point: &[BinaryExtensionField],
    value: BinaryExtensionField,
    transcript: &mut OpeningTranscriptReader<'_>,
) {
    transcript.absorb_digest(b"parameters", &pp.parameter_digest);
    absorb_commitment_reader(commitment, transcript);
    transcript.absorb_extensions(b"point", point);
    transcript.absorb_field(b"claimed-value", value);
}

fn absorb_commitment_writer(
    commitment: &ShortRaaFoldCommitment,
    transcript: &mut OpeningTranscriptWriter,
) {
    transcript.absorb_digest(b"commitment-root", &commitment.root());
    for (label, value) in [
        (b"encoded-rows".as_slice(), commitment.encoded_rows()),
        (b"packed-columns".as_slice(), commitment.columns()),
        (b"word-bits".as_slice(), commitment.word_bits()),
        (
            b"logical-element-bits".as_slice(),
            commitment.logical_element_bits(),
        ),
        (
            b"logical-elements".as_slice(),
            commitment.logical_elements(),
        ),
    ] {
        transcript.absorb_bytes(label, &(value as u64).to_le_bytes());
    }
    transcript.absorb_digest(b"generator", &commitment.generator_digest());
}

fn absorb_commitment_reader(
    commitment: &ShortRaaFoldCommitment,
    transcript: &mut OpeningTranscriptReader<'_>,
) {
    transcript.absorb_digest(b"commitment-root", &commitment.root());
    for (label, value) in [
        (b"encoded-rows".as_slice(), commitment.encoded_rows()),
        (b"packed-columns".as_slice(), commitment.columns()),
        (b"word-bits".as_slice(), commitment.word_bits()),
        (
            b"logical-element-bits".as_slice(),
            commitment.logical_element_bits(),
        ),
        (
            b"logical-elements".as_slice(),
            commitment.logical_elements(),
        ),
    ] {
        transcript.absorb_bytes(label, &(value as u64).to_le_bytes());
    }
    transcript.absorb_digest(b"generator", &commitment.generator_digest());
}

fn parameter_digest(
    pc: &ShortRaaFoldPC,
    sequence: &ShortRaaFold<u128>,
    terminal_level: usize,
    opening_tail: &ShortRaaLigeroTailParameters,
    decide_tail: &ShortRaaLigeroTailParameters,
    setup_digest: &[u8; DIGEST_BYTES],
) -> Result<[u8; DIGEST_BYTES], Error> {
    let mut hasher = Blake2s256::new();
    absorb_framed(&mut hasher, b"domain", PARAMETER_DOMAIN);
    for (label, value) in [
        (b"witness".as_slice(), pc.n_witness),
        (b"packed-columns".as_slice(), pc.packed_columns),
        (b"element-bits".as_slice(), pc.logical_element_bits),
        (b"code-switch-ratio".as_slice(), pc.code_switch_ratio),
        (b"security-bits".as_slice(), pc.security_bits),
        (b"levels".as_slice(), sequence.levels()),
        (b"terminal-level".as_slice(), terminal_level),
    ] {
        absorb_framed(&mut hasher, label, &(value as u64).to_le_bytes());
    }
    absorb_framed(&mut hasher, b"generators", &sequence.generator_digest());
    absorb_ligero_tail_parameters(&mut hasher, b"opening-tail", opening_tail);
    absorb_ligero_tail_parameters(&mut hasher, b"decide-tail", decide_tail);
    absorb_framed(&mut hasher, b"setup-commitments", setup_digest);
    Ok(hasher.finalize().into())
}

fn absorb_ligero_tail_parameters(
    hasher: &mut Blake2s256,
    label: &[u8],
    tail: &ShortRaaLigeroTailParameters,
) {
    absorb_framed(hasher, b"tail-kind", label);
    for (field, value) in [
        (b"tail-witness".as_slice(), tail.witness_len),
        (b"tail-height".as_slice(), tail.height),
        (b"tail-width".as_slice(), tail.width),
        (b"tail-rate-inv".as_slice(), tail.code.rate_inv()),
    ] {
        absorb_framed(hasher, field, &(value as u64).to_le_bytes());
    }
    absorb_framed(hasher, b"tail-generator", &tail.code.generator_digest());
}

fn validate_ligero_tail(
    tail: &ShortRaaLigeroTailParameters,
    witness_len: usize,
    lambda: usize,
) -> Result<(), Error> {
    let (height, width) = ligero_tail_dimensions(witness_len, lambda)?;
    if tail.witness_len != witness_len
        || tail.height != height
        || tail.width != width
        || tail.height.checked_mul(tail.width) != Some(witness_len)
        || tail.code.message_len() != height
        || tail.code.rate_inv() != crate::raafold::RAAFOLD_RECURSIVE_RATE_INV
        || tail.code.distance_profile()
            != RaaDistanceProfile::blaze_assumed(crate::raafold::RAAFOLD_RECURSIVE_RATE_INV)?
    {
        return Err(Error::InvalidCommitment);
    }
    Ok(())
}

fn validate_parameters(pc: &ShortRaaFoldPC, pp: &ShortRaaFoldParameters) -> Result<(), Error> {
    let setup_digest = setup_commitment_digest(&pp.setup_levels)?;
    let terminal_level = (0..pp.extension_sequence.levels())
        .rev()
        .find(|&level| {
            pp.extension_sequence
                .code(level)
                .is_ok_and(|code| code.message_len() < RAAFOLD_LIGERO_THRESHOLD)
        })
        .ok_or(Error::InvalidCommitment)?;
    let decide_level = terminal_level
        .checked_sub(1)
        .ok_or(Error::InvalidCommitment)?;
    validate_ligero_tail(
        &pp.opening_tail,
        pp.extension_sequence.code(terminal_level)?.message_len(),
        pc.security_bits,
    )?;
    validate_ligero_tail(
        &pp.decide_tail,
        pp.extension_sequence.code(decide_level)?.message_len(),
        pc.security_bits,
    )?;
    if pp.n_witness != pc.n_witness
        || pp.packed_columns != pc.packed_columns
        || pp.logical_element_bits != pc.logical_element_bits
        || pp.security_bits != pc.security_bits
        || pp.sequence.switch_ratio() != pc.code_switch_ratio
        || pp.sequence.recursive_switch_ratio() != pc.code_switch_ratio
        || pp.extension_sequence.switch_ratio() != pc.code_switch_ratio
        || pp.extension_sequence.recursive_switch_ratio() != pc.code_switch_ratio
        || pp.extension_sequence.generator_digest() != pp.sequence.generator_digest()
        || pp.terminal_level != terminal_level
        || setup_digest != pp.setup_digest
        || parameter_digest(
            pc,
            &pp.sequence,
            terminal_level,
            &pp.opening_tail,
            &pp.decide_tail,
            &setup_digest,
        )? != pp.parameter_digest
    {
        return Err(Error::InvalidCommitment);
    }
    Ok(())
}

fn validate_prover_setup(
    pp: &ShortRaaFoldParameters,
    prover_setup: &ShortRaaFoldProverSetup,
) -> Result<(), Error> {
    if prover_setup.parameter_digest != pp.parameter_digest
        || prover_setup.setup_caches.len() != pp.setup_levels.len()
        || !matches!(prover_setup.setup_caches.first(), Some(None))
    {
        return Err(Error::InvalidCommitment);
    }
    for level in 1..pp.setup_levels.len() {
        validate_prover_setup_level(pp.setup_level(level)?, prover_setup.setup_cache(level)?)?;
    }
    Ok(())
}

fn digest_fields(digest: &[u8; DIGEST_BYTES]) -> [BinaryExtensionField; 2] {
    let low = u128::from_le_bytes(digest[..16].try_into().expect("fixed digest prefix"));
    let high = u128::from_le_bytes(digest[16..].try_into().expect("fixed digest suffix"));
    [
        BinaryExtensionField::from(low),
        BinaryExtensionField::from(high),
    ]
}

fn absorb_framed(hasher: &mut Blake2s256, label: &[u8], bytes: &[u8]) {
    hasher.update((label.len() as u64).to_le_bytes());
    hasher.update(label);
    hasher.update((bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
}

fn derive_digest(
    state: &Blake2s256,
    label: &[u8],
    counter: u64,
    attempt: u64,
) -> [u8; DIGEST_BYTES] {
    let mut hasher = state.clone();
    absorb_framed(&mut hasher, b"challenge-label", label);
    hasher.update(counter.to_le_bytes());
    hasher.update(attempt.to_le_bytes());
    hasher.finalize().into()
}

impl ShortRaaFoldProof {
    /// Serializes the complete proof with fixed-width fields and length prefixes.
    pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut output = Vec::new();
        output.extend_from_slice(PROOF_MAGIC);
        write_fields(&mut output, &self.opening.responses)?;
        write_len(&mut output, self.opening.top_rows.len())?;
        for query in &self.opening.top_rows {
            write_len(&mut output, query.row_index)?;
            write_len(&mut output, query.row.len())?;
            for &word in &query.row {
                output.extend_from_slice(&word.to_le_bytes());
            }
            write_path(&mut output, &query.path)?;
        }
        write_len(&mut output, self.opening.extension_rounds.len())?;
        for round in &self.opening.extension_rounds {
            output.extend_from_slice(&round.root);
            write_len(&mut output, round.rows.len())?;
            for query in &round.rows {
                write_fields(&mut output, &query.row)?;
                write_path(&mut output, &query.path)?;
            }
        }
        write_ligero_tail(&mut output, &self.opening.tail)?;
        write_len(&mut output, self.decide.generator_proofs.len())?;
        for proof in &self.decide.generator_proofs {
            write_len(&mut output, proof.level)?;
            write_len(&mut output, proof.auxiliary_roots.len())?;
            for root in &proof.auxiliary_roots {
                output.extend_from_slice(root);
            }
        }
        write_fields(&mut output, &self.decide.responses)?;
        write_len(&mut output, self.decide.rounds.len())?;
        for round in &self.decide.rounds {
            write_len(&mut output, round.level)?;
            write_extension_rows(&mut output, &round.setup_rows)?;
            write_len(&mut output, round.auxiliary_rows.len())?;
            for rows in &round.auxiliary_rows {
                write_extension_rows(&mut output, rows)?;
            }
            write_optional_extension_rows(&mut output, round.carry_rows.as_deref())?;
            write_optional_digest(&mut output, round.next_carry_root.as_ref());
        }
        write_ligero_tail(&mut output, &self.decide.tail)?;
        Ok(output)
    }

    /// Decodes a canonical proof and rejects trailing bytes or impossible lengths.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = ProofReader::new(bytes);
        if reader.take(PROOF_MAGIC.len())? != PROOF_MAGIC {
            return Err(Error::InvalidCommitment);
        }
        let responses = reader.read_fields()?;
        let top_count = reader.read_count(24)?;
        let mut top_rows = reserve_vec(top_count)?;
        for _ in 0..top_count {
            let row_index = reader.read_len()?;
            let row_len = reader.read_count(FIELD_BYTES)?;
            let mut row = reserve_vec(row_len)?;
            for _ in 0..row_len {
                row.push(u128::from_le_bytes(
                    reader
                        .take(16)?
                        .try_into()
                        .map_err(|_| Error::InvalidCommitment)?,
                ));
            }
            top_rows.push(ShortRaaRowQuery {
                row_index,
                row,
                path: reader.read_path()?,
            });
        }
        let round_count = reader.read_count(DIGEST_BYTES + 8)?;
        let mut extension_rounds = reserve_vec(round_count)?;
        for _ in 0..round_count {
            let root = reader
                .take(DIGEST_BYTES)?
                .try_into()
                .map_err(|_| Error::InvalidCommitment)?;
            let row_count = reader.read_count(16)?;
            let mut rows = reserve_vec(row_count)?;
            for _ in 0..row_count {
                rows.push(ShortRaaExtensionRowQuery {
                    row: reader.read_fields()?,
                    path: reader.read_path()?,
                });
            }
            extension_rounds.push(ShortRaaExtensionRound { root, rows });
        }
        let opening_tail = reader.read_ligero_tail()?;
        let generator_count = reader.read_count(16)?;
        let mut generator_proofs = reserve_vec(generator_count)?;
        for _ in 0..generator_count {
            let level = reader.read_len()?;
            let root_count = reader.read_count(DIGEST_BYTES)?;
            let mut auxiliary_roots = reserve_vec(root_count)?;
            for _ in 0..root_count {
                auxiliary_roots.push(
                    reader
                        .take(DIGEST_BYTES)?
                        .try_into()
                        .map_err(|_| Error::InvalidCommitment)?,
                );
            }
            generator_proofs.push(ShortRaaGeneratorProof {
                level,
                auxiliary_roots,
            });
        }
        let decide_responses = reader.read_fields()?;
        let decide_round_count = reader.read_count(26)?;
        let mut decide_rounds = reserve_vec(decide_round_count)?;
        for _ in 0..decide_round_count {
            let level = reader.read_len()?;
            let setup_rows = reader.read_extension_rows()?;
            let auxiliary_count = reader.read_count(8)?;
            let mut auxiliary_rows = reserve_vec(auxiliary_count)?;
            for _ in 0..auxiliary_count {
                auxiliary_rows.push(reader.read_extension_rows()?);
            }
            decide_rounds.push(ShortRaaDecideRound {
                level,
                setup_rows,
                auxiliary_rows,
                carry_rows: reader.read_optional_extension_rows()?,
                next_carry_root: reader.read_optional_digest()?,
            });
        }
        let decide_tail = reader.read_ligero_tail()?;
        if !reader.is_empty() {
            return Err(Error::InvalidCommitment);
        }
        Ok(Self {
            opening: ShortRaaOpeningProof {
                responses,
                top_rows,
                extension_rounds,
                tail: opening_tail,
            },
            decide: ShortRaaDecideProof {
                generator_proofs,
                responses: decide_responses,
                rounds: decide_rounds,
                tail: decide_tail,
            },
        })
    }

    /// Returns the canonical proof byte length.
    pub fn proof_size(&self) -> Result<usize, Error> {
        Ok(self.to_bytes()?.len())
    }
}

fn write_len(output: &mut Vec<u8>, value: usize) -> Result<(), Error> {
    output.extend_from_slice(
        &u64::try_from(value)
            .map_err(|_| Error::EncodingError)?
            .to_le_bytes(),
    );
    Ok(())
}

fn write_fields(output: &mut Vec<u8>, values: &[BinaryExtensionField]) -> Result<(), Error> {
    write_len(output, values.len())?;
    for &value in values {
        output.extend_from_slice(&u128::from(value).to_le_bytes());
    }
    Ok(())
}

fn write_path(output: &mut Vec<u8>, path: &ShortRaaMerklePath) -> Result<(), Error> {
    write_len(output, path.siblings.len())?;
    for sibling in &path.siblings {
        output.extend_from_slice(sibling);
    }
    Ok(())
}

fn write_extension_rows(
    output: &mut Vec<u8>,
    rows: &[ShortRaaExtensionRowQuery],
) -> Result<(), Error> {
    write_len(output, rows.len())?;
    for query in rows {
        write_fields(output, &query.row)?;
        write_path(output, &query.path)?;
    }
    Ok(())
}

fn write_ligero_tail(output: &mut Vec<u8>, tail: &ShortRaaLigeroTailProof) -> Result<(), Error> {
    output.extend_from_slice(&tail.root);
    write_extension_rows(output, &tail.rows)
}

fn write_optional_extension_rows(
    output: &mut Vec<u8>,
    rows: Option<&[ShortRaaExtensionRowQuery]>,
) -> Result<(), Error> {
    output.push(u8::from(rows.is_some()));
    if let Some(rows) = rows {
        write_extension_rows(output, rows)?;
    }
    Ok(())
}

fn write_optional_digest(output: &mut Vec<u8>, digest: Option<&[u8; DIGEST_BYTES]>) {
    output.push(u8::from(digest.is_some()));
    if let Some(digest) = digest {
        output.extend_from_slice(digest);
    }
}

struct ProofReader<'a> {
    bytes: &'a [u8],
    cursor: usize,
}

impl<'a> ProofReader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, cursor: 0 }
    }

    fn take(&mut self, length: usize) -> Result<&'a [u8], Error> {
        let end = self
            .cursor
            .checked_add(length)
            .filter(|&end| end <= self.bytes.len())
            .ok_or(Error::InvalidCommitment)?;
        let output = &self.bytes[self.cursor..end];
        self.cursor = end;
        Ok(output)
    }

    fn read_len(&mut self) -> Result<usize, Error> {
        let value = u64::from_le_bytes(
            self.take(8)?
                .try_into()
                .map_err(|_| Error::InvalidCommitment)?,
        );
        usize::try_from(value).map_err(|_| Error::InvalidCommitment)
    }

    /// Rejects impossible collection sizes before allocating their containers.
    /// Empty nested vectors still have an eight-byte length prefix; optional
    /// members contribute their tag byte to the enclosing entry's minimum.
    fn read_count(&mut self, min_entry_bytes: usize) -> Result<usize, Error> {
        let len = self.read_len()?;
        if min_entry_bytes == 0 || len > self.remaining() / min_entry_bytes {
            return Err(Error::InvalidCommitment);
        }
        Ok(len)
    }

    fn read_fields(&mut self) -> Result<Vec<BinaryExtensionField>, Error> {
        let len = self.read_count(FIELD_BYTES)?;
        let mut values = reserve_vec(len)?;
        for _ in 0..len {
            let value = u128::from_le_bytes(
                self.take(FIELD_BYTES)?
                    .try_into()
                    .map_err(|_| Error::InvalidCommitment)?,
            );
            values.push(BinaryExtensionField::from(value));
        }
        Ok(values)
    }

    fn read_path(&mut self) -> Result<ShortRaaMerklePath, Error> {
        let len = self.read_count(DIGEST_BYTES)?;
        let mut siblings = reserve_vec(len)?;
        for _ in 0..len {
            siblings.push(
                self.take(DIGEST_BYTES)?
                    .try_into()
                    .map_err(|_| Error::InvalidCommitment)?,
            );
        }
        Ok(ShortRaaMerklePath { siblings })
    }

    fn read_extension_rows(&mut self) -> Result<Vec<ShortRaaExtensionRowQuery>, Error> {
        let len = self.read_count(16)?;
        let mut rows = reserve_vec(len)?;
        for _ in 0..len {
            rows.push(ShortRaaExtensionRowQuery {
                row: self.read_fields()?,
                path: self.read_path()?,
            });
        }
        Ok(rows)
    }

    fn read_ligero_tail(&mut self) -> Result<ShortRaaLigeroTailProof, Error> {
        let root = self
            .take(DIGEST_BYTES)?
            .try_into()
            .map_err(|_| Error::InvalidCommitment)?;
        Ok(ShortRaaLigeroTailProof {
            root,
            rows: self.read_extension_rows()?,
        })
    }

    fn read_optional_extension_rows(
        &mut self,
    ) -> Result<Option<Vec<ShortRaaExtensionRowQuery>>, Error> {
        match self.take(1)?[0] {
            0 => Ok(None),
            1 => Ok(Some(self.read_extension_rows()?)),
            _ => Err(Error::InvalidCommitment),
        }
    }

    fn read_optional_digest(&mut self) -> Result<Option<[u8; DIGEST_BYTES]>, Error> {
        match self.take(1)?[0] {
            0 => Ok(None),
            1 => Ok(Some(
                self.take(DIGEST_BYTES)?
                    .try_into()
                    .map_err(|_| Error::InvalidCommitment)?,
            )),
            _ => Err(Error::InvalidCommitment),
        }
    }

    fn remaining(&self) -> usize {
        self.bytes.len() - self.cursor
    }

    fn is_empty(&self) -> bool {
        self.cursor == self.bytes.len()
    }
}

fn reserve_vec<T>(len: usize) -> Result<Vec<T>, Error> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(len)
        .map_err(|_| Error::InvalidCommitment)?;
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::rand::{SeedableRng, rngs::StdRng};

    fn field(value: u128) -> BinaryExtensionField {
        BinaryExtensionField::from(value)
    }

    fn scalar_inner_product(
        left: &[BinaryExtensionField],
        right: &[BinaryExtensionField],
    ) -> BinaryExtensionField {
        assert_eq!(left.len(), right.len());
        left.iter()
            .zip(right)
            .fold(BinaryExtensionField::ZERO, |sum, (&a, &b)| sum + a * b)
    }

    #[test]
    fn chunked_inner_product_matches_scalar_at_simd_and_parallel_boundaries() {
        for len in [0, 1, 2, 3, 4, 7, 16, 4095, 4096, 4097, 8193] {
            let left = dense_extension_rows(1, len);
            let right = left
                .iter()
                .enumerate()
                .map(|(i, &value)| value + field((i as u128 + 1) << 65))
                .collect::<Vec<_>>();
            assert_eq!(
                inner_product(&left, &right).unwrap(),
                scalar_inner_product(&left, &right),
                "length={len}"
            );
        }
        assert_eq!(
            inner_product(&[], &[BinaryExtensionField::ONE]),
            Err(Error::InvalidNumberOfVariables)
        );
    }

    #[test]
    fn bulk_equality_folding_matches_scalar_including_boolean_challenges() {
        for len in [0, 1, 2, 3, 4, 5, 31, 64, 257, 4097] {
            let low = dense_extension_rows(1, len);
            let high = low
                .iter()
                .rev()
                .map(|&value| value + BinaryExtensionField::ONE)
                .collect::<Vec<_>>();
            for challenge in [
                BinaryExtensionField::ZERO,
                BinaryExtensionField::ONE,
                field(0x1234_5678_9abc_def0),
            ] {
                let expected = low
                    .iter()
                    .zip(&high)
                    .map(|(&low, &high)| low + (low + high) * challenge)
                    .collect::<Vec<_>>();
                let mut actual = low.clone();
                fold_equality_slice(&mut actual, &high, challenge);
                assert_eq!(actual, expected, "length={len}, challenge={challenge:?}");
            }
        }
    }

    /// Scalar column-major execution of the same column-then-row transcript.
    /// It shares no message or folding kernels with the optimized matrix path.
    fn scalar_matrix_litebullet_reference(
        left: &[BinaryExtensionField],
        right: &[BinaryExtensionField],
        rows: usize,
        columns: usize,
        transcript: &mut OpeningTranscriptWriter,
        label: &[u8],
    ) -> (LiteBulletReduction, Vec<BinaryExtensionField>) {
        let column_major = |matrix: &[BinaryExtensionField]| {
            (0..columns)
                .flat_map(|column| (0..rows).map(move |row| matrix[row * columns + column]))
                .collect::<Vec<_>>()
        };
        let mut left = column_major(left);
        let mut right = column_major(right);
        let mut rhs = scalar_inner_product(&left, &right);
        let mut point = Vec::new();
        let mut folded_vector = if columns == 1 {
            left.clone()
        } else {
            Vec::new()
        };
        transcript.absorb_bytes(b"litebullet-domain", label);
        while left.len() > 1 {
            let half = left.len() / 2;
            let high = scalar_inner_product(&left[half..], &right[half..]);
            let cross = scalar_inner_product(&left[..half], &right[half..])
                + scalar_inner_product(&left[half..], &right[..half]);
            transcript.push_response(high);
            transcript.push_response(cross);
            let challenge = transcript
                .challenge_non_singular(b"litebullet-fold")
                .unwrap();
            let challenge_squared = challenge * challenge;
            rhs = rhs * (BinaryExtensionField::ONE + challenge_squared)
                + high
                + cross * (challenge + challenge_squared);
            for i in 0..half {
                left[i] = left[i] + (left[i] + left[i + half]) * challenge;
                right[i] = right[i] + (right[i] + right[i + half]) * challenge;
            }
            left.truncate(half);
            right.truncate(half);
            if left.len() == rows {
                folded_vector = left.clone();
            }
            point.push(challenge);
        }
        transcript.push_response(left[0]);
        transcript.push_response(right[0]);
        assert_eq!(left[0] * right[0], rhs);
        (
            LiteBulletReduction {
                a_value: left[0],
                a_point: point.clone(),
                b_value: right[0],
                b_point: point,
            },
            folded_vector,
        )
    }

    #[test]
    fn bulk_litebullet_preserves_scalar_transcript_bytes() {
        for len in [1, 2, 8, 8192, 16384] {
            let left = dense_extension_rows(len, 1);
            let right = left
                .iter()
                .enumerate()
                .map(|(i, &value)| value + field(((i as u128 + 11) << 83) | (i as u128 + 7)))
                .collect::<Vec<_>>();
            let claim = scalar_inner_product(&left, &right);
            let mut reference_transcript = OpeningTranscriptWriter::new();
            let (expected, _) = scalar_matrix_litebullet_reference(
                &left,
                &right,
                len,
                1,
                &mut reference_transcript,
                b"bulk-v5-regression",
            );
            let mut transcript = OpeningTranscriptWriter::new();
            let actual =
                prove_litebullet(claim, left, right, &mut transcript, b"bulk-v5-regression")
                    .unwrap();
            assert_eq!(actual.a_value, expected.a_value);
            assert_eq!(actual.b_value, expected.b_value);
            assert_eq!(actual.a_point, expected.a_point);
            assert_eq!(actual.b_point, expected.b_point);
            let response_bytes = |responses: &[BinaryExtensionField]| {
                responses
                    .iter()
                    .flat_map(|&value| u128::from(value).to_le_bytes())
                    .collect::<Vec<_>>()
            };
            assert_eq!(
                response_bytes(&transcript.responses),
                response_bytes(&reference_transcript.responses),
                "length={len}"
            );
            assert_eq!(
                transcript.challenge_digest(b"after-v5"),
                reference_transcript.challenge_digest(b"after-v5")
            );
        }
    }

    #[test]
    fn matrix_simd_litebullet_preserves_scalar_transcript_bytes() {
        for (rows, columns) in [
            (1, 1),
            (1, 128),
            (4, 2),
            (8, 8),
            (64, 128),
            (128, 128),
            (8192, 1),
            (8192, 4),
        ] {
            let left = dense_extension_rows(rows, columns);
            let right = left
                .iter()
                .enumerate()
                .map(|(i, &value)| value + field(((i as u128 + 5) << 71) | (i as u128 + 2)))
                .collect::<Vec<_>>();
            let claim = scalar_inner_product(&left, &right);
            let mut reference_transcript = OpeningTranscriptWriter::new();
            let (expected, expected_folded) = scalar_matrix_litebullet_reference(
                &left,
                &right,
                rows,
                columns,
                &mut reference_transcript,
                b"matrix-v4-regression",
            );
            let mut transcript = OpeningTranscriptWriter::new();
            let (actual, actual_folded) = prove_row_major_matrix_litebullet(
                claim,
                left,
                right,
                rows,
                columns,
                &mut transcript,
                b"matrix-v4-regression",
            )
            .unwrap();
            assert_eq!(
                actual_folded, expected_folded,
                "folded rows={rows}, columns={columns}"
            );
            assert_eq!(actual.a_value, expected.a_value);
            assert_eq!(actual.b_value, expected.b_value);
            assert_eq!(actual.a_point, expected.a_point);
            assert_eq!(actual.b_point, expected.b_point);
            let response_bytes = |responses: &[BinaryExtensionField]| {
                responses
                    .iter()
                    .flat_map(|&value| u128::from(value).to_le_bytes())
                    .collect::<Vec<_>>()
            };
            assert_eq!(
                response_bytes(&transcript.responses),
                response_bytes(&reference_transcript.responses),
                "response bytes rows={rows}, columns={columns}"
            );
            assert_eq!(
                transcript.challenge_digest(b"after-v4"),
                reference_transcript.challenge_digest(b"after-v4")
            );
        }
    }

    /// The pre-v3 leaf path, retained as an independent byte-order reference.
    fn extension_leaf_word_updates(row: &[BinaryExtensionField]) -> [u8; DIGEST_BYTES] {
        let mut hasher = Blake2s256::new();
        absorb_framed(&mut hasher, b"domain", EXTENSION_LEAF_DOMAIN);
        absorb_framed(&mut hasher, b"width", &(row.len() as u64).to_le_bytes());
        for &value in row {
            hasher.update(u128::from(value).to_le_bytes());
        }
        hasher.finalize().into()
    }

    fn dense_extension_rows(rows: usize, width: usize) -> Vec<BinaryExtensionField> {
        let mut rng = StdRng::seed_from_u64(0x726f_772d_6861_7368 ^ width as u64);
        (0..rows * width)
            .map(|i| {
                let random = rng.next_u64() as u128 | ((rng.next_u64() as u128) << 64);
                field(match i % 8 {
                    0 => 0,
                    1 => u128::MAX,
                    2 => 1u128 << 127,
                    3 => 1u128 << 64,
                    _ => random,
                })
            })
            .collect()
    }

    #[test]
    fn extension_leaf_bulk_bytes_match_word_updates() {
        for width in [1, 2, 3, 4, 8, 31, 32, 33, 64, 128, 512] {
            let entries = dense_extension_rows(17, width);
            let prefix = extension_leaf_prefix(width);
            for (row_index, row) in entries.chunks_exact(width).enumerate() {
                let expected = extension_leaf_word_updates(row);
                assert_eq!(
                    hash_extension_row(row),
                    expected,
                    "width={width} row={row_index}",
                );
                assert_eq!(
                    hash_extension_row_from_prefix(prefix.clone(), row),
                    expected,
                    "prefix width={width} row={row_index}",
                );
            }
        }
    }

    #[test]
    fn extension_bulk_tree_matches_word_update_roots_and_all_paths() {
        for n_rows in [1, 2, 16, 64] {
            for width in [1, 8, 128, 512] {
                let entries = dense_extension_rows(n_rows, width);
                let reference_leaves = entries
                    .chunks_exact(width)
                    .map(extension_leaf_word_updates)
                    .collect();
                let reference = ExtensionMerkleTree::from_leaves(reference_leaves).unwrap();
                let actual = build_extension_tree(&entries, n_rows, width).unwrap();
                assert_eq!(
                    actual.levels, reference.levels,
                    "rows={n_rows} width={width}",
                );
                for (index, row) in entries.chunks_exact(width).enumerate() {
                    let path = actual.path(index).unwrap();
                    assert_eq!(path, reference.path(index).unwrap());
                    assert!(verify_extension_path(
                        extension_leaf_word_updates(row),
                        index,
                        n_rows,
                        &path,
                        &actual.root(),
                    ));
                    assert!(verify_extension_path(
                        hash_extension_row(row),
                        index,
                        n_rows,
                        &path,
                        &reference.root(),
                    ));
                    let mut altered = row.to_vec();
                    altered[0] += field(1u128 << 127);
                    assert!(!verify_extension_path(
                        hash_extension_row(&altered),
                        index,
                        n_rows,
                        &path,
                        &reference.root(),
                    ));
                }
            }
        }
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn extension_bulk_tree_is_thread_count_independent() {
        let single = rayon::ThreadPoolBuilder::new()
            .num_threads(1)
            .build()
            .unwrap();
        let multiple = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        for width in [1, 8, 128, 512] {
            let entries = dense_extension_rows(64, width);
            let reference = single.install(|| build_extension_tree(&entries, 64, width).unwrap());
            let actual = multiple.install(|| build_extension_tree(&entries, 64, width).unwrap());
            assert_eq!(actual.levels, reference.levels, "width={width}");
        }
    }

    #[test]
    fn tensor_product_formulas_match_expansion_including_boolean_edges() {
        for log_len in 0..=9 {
            let x = (0..log_len)
                .map(|i| match i % 3 {
                    0 => BinaryExtensionField::ZERO,
                    1 => BinaryExtensionField::ONE,
                    _ => field((i * 17 + 3) as u128),
                })
                .collect::<Vec<_>>();
            let y = (0..log_len)
                .map(|i| field((i * 29 + 7) as u128))
                .collect::<Vec<_>>();
            for basis in [OpeningPointBasis::Homogeneous, OpeningPointBasis::Equality] {
                let expected =
                    inner_product(&tensor_weights(&x, basis), &equality_tensor(&y)).unwrap();
                assert_eq!(
                    tensor_equality_inner_product(&x, basis, &y).unwrap(),
                    expected
                );
            }
        }
        assert!(
            tensor_equality_inner_product(&[field(3)], OpeningPointBasis::Equality, &[]).is_err()
        );
    }

    #[test]
    fn boolean_mle_openings_match_every_corner_and_dense_point() {
        let n = 16usize;
        let pc = ShortRaaFoldPC::new(n, 2, 128, 2, 16).unwrap();
        let mut rng = StdRng::seed_from_u64(0x524141);
        let (pp, setup) = pc.setup(&mut rng).unwrap();
        let values = (0..n)
            .map(|i| ((i as u128 + 7) << 77) ^ (i as u128 * 31))
            .collect::<Vec<_>>();
        let mut points = (0..n)
            .map(|index| {
                (0..4)
                    .map(|bit| {
                        if index & (1 << (3 - bit)) == 0 {
                            BinaryExtensionField::ZERO
                        } else {
                            BinaryExtensionField::ONE
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        points.push(vec![field(3), field(9), field(27), field(81)]);
        for (index, point) in points.iter().enumerate() {
            let (commitment, cache) = pc.commit_values(&pp, &values).unwrap();
            let (value, proof) = pc.open(&pp, &setup, &commitment, point, cache).unwrap();
            let expected = inner_product(
                &values.iter().map(|&x| field(x)).collect::<Vec<_>>(),
                &equality_tensor(point),
            )
            .unwrap();
            assert_eq!(value, expected);
            if index < n {
                assert_eq!(value, field(values[index]));
            }
            assert!(pc.verify(&pp, &commitment, point, value, &proof).unwrap());
            assert!(
                !pc.verify(
                    &pp,
                    &commitment,
                    point,
                    value + BinaryExtensionField::ONE,
                    &proof
                )
                .unwrap_or(false)
            );
        }
    }

    #[test]
    #[ignore = "full recursive PCS test; run explicitly in release mode"]
    fn full_recursive_boolean_mle_opening() {
        let n = 1usize << 18;
        let pc = ShortRaaFoldPC::new(n, 2, 128, 4, 32).unwrap();
        let mut rng = StdRng::seed_from_u64(0x52414132);
        let (pp, setup) = pc.setup(&mut rng).unwrap();
        let values = (0..n)
            .map(|i| (i as u128 * 0x100010001) ^ ((i as u128 + 1) << 100))
            .collect::<Vec<_>>();
        let point = (0..18)
            .map(|i| field((i * 37 + 11) as u128))
            .collect::<Vec<_>>();
        let expected = inner_product(
            &values.iter().map(|&x| field(x)).collect::<Vec<_>>(),
            &equality_tensor(&point),
        )
        .unwrap();
        let shape = ShortRaaPackedShape::new(n / 2, 2, 128).unwrap();
        let witness = ShortRaaPackedWitness::from_packed_words(shape, values).unwrap();
        let (commitment, cache) = pc.commit_packed(&pp, witness).unwrap();
        let (value, proof) = pc.open(&pp, &setup, &commitment, &point, cache).unwrap();
        assert_eq!(value, expected);
        assert!(!proof.opening.extension_rounds.is_empty());
        assert!(proof.decide.rounds.len() >= 2);
        drop(setup);
        assert!(pc.verify(&pp, &commitment, &point, value, &proof).unwrap());
        let decoded = ShortRaaFoldProof::from_bytes(&proof.to_bytes().unwrap()).unwrap();
        assert!(
            pc.verify(&pp, &commitment, &point, value, &decoded)
                .unwrap()
        );
        let mut corrupted = decoded;
        corrupted.decide.responses[0] += BinaryExtensionField::ONE;
        assert!(
            !pc.verify(&pp, &commitment, &point, value, &corrupted)
                .unwrap_or(false)
        );
    }

    #[test]
    fn wide_inner_product_matches_eager_reduction() {
        for length in [0usize, 1, 3, 64, 4097] {
            let left = (0..length)
                .map(|index| field((index as u128).wrapping_mul(17).wrapping_add(3)))
                .collect::<Vec<_>>();
            let right = (0..length)
                .map(|index| field((index as u128).wrapping_mul(29).wrapping_add(11)))
                .collect::<Vec<_>>();
            let expected = left
                .iter()
                .zip(&right)
                .map(|(&left, &right)| left * right)
                .sum();
            assert_eq!(inner_product(&left, &right).unwrap(), expected);
        }
    }

    #[test]
    fn ligero_tail_uses_exact_unpadded_dyadic_rectangles() {
        assert_eq!(
            ligero_tail_dimensions(1 << 18, 100).unwrap(),
            (1 << 12, 1 << 6)
        );
        assert_eq!(
            ligero_tail_dimensions(1 << 19, 100).unwrap(),
            (1 << 13, 1 << 6)
        );
        for log_witness in 0..20 {
            let witness_len = 1usize << log_witness;
            let (height, width) = ligero_tail_dimensions(witness_len, 100).unwrap();
            assert_eq!(height * width, witness_len);
            assert!(height.is_power_of_two() && width.is_power_of_two());
            assert!(height >= width);
        }
    }

    fn end_to_end(bit_width: usize, seed: u64) {
        let n_witness = 1 << 10;
        let pc = ShortRaaFoldPC::new(n_witness, 2, bit_width, 4, 8).unwrap();
        let mut rng = StdRng::seed_from_u64(seed);
        let (pp, prover_setup) = pc.setup(&mut rng).unwrap();
        let mask = if bit_width == 128 {
            u128::MAX
        } else {
            (1u128 << bit_width) - 1
        };
        let values = (0..n_witness)
            .map(|index| ((index as u128).wrapping_mul(17).wrapping_add(5)) & mask)
            .collect::<Vec<_>>();
        let (commitment, cache) = pc.commit_values(&pp, &values).unwrap();
        let point = (0..n_witness.ilog2() as usize)
            .map(|index| field(index as u128 + 3))
            .collect::<Vec<_>>();
        let (value, opening, reduction) = pc
            .open_with_reduction(&pp, &commitment, &point, cache)
            .expect("recursive opening with Ligero tail must succeed");
        let decide = pc
            .prove_decide(&pp, &prover_setup, &commitment, &reduction)
            .expect("recursive Decide with Ligero tail must succeed");
        let proof = ShortRaaFoldProof { opening, decide };
        assert!(pc.verify(&pp, &commitment, &point, value, &proof).unwrap());
        let encoded = proof.to_bytes().unwrap();
        let decoded = ShortRaaFoldProof::from_bytes(&encoded).unwrap();
        assert_eq!(decoded, proof);
        assert!(
            pc.verify(&pp, &commitment, &point, value, &decoded)
                .unwrap()
        );
    }

    #[test]
    fn packed_widths_open_in_gf_2_128() {
        for (index, bit_width) in [1, 2, 4, 8, 16, 32, 64, 128].into_iter().enumerate() {
            end_to_end(bit_width, 901 + index as u64);
        }
    }

    #[test]
    fn setup_uses_blaze_rate_four_at_every_level() {
        let pc = ShortRaaFoldPC::new(1 << 10, 2, 128, 4, 8).unwrap();
        let mut rng = StdRng::seed_from_u64(919);
        let (pp, _) = pc.setup(&mut rng).unwrap();
        let rates = pp
            .sequence
            .codes()
            .iter()
            .map(ShortRaaCode::rate_inv)
            .collect::<Vec<_>>();
        assert!(rates.iter().all(|&rate| rate == 4));
        assert_eq!(pp.opening_tail().code().rate_inv(), 4);
        assert_eq!(pp.decide_tail().code().rate_inv(), 4);
    }

    #[test]
    fn setup_uses_128_top_columns_and_8_for_every_code_switch() {
        let pc = ShortRaaFoldPC::new(1 << 16, 128, 128, 8, 8).unwrap();
        let mut rng = StdRng::seed_from_u64(920);
        let (pp, _) = pc.setup(&mut rng).unwrap();
        let message_lengths = pp
            .sequence
            .codes()
            .iter()
            .map(ShortRaaCode::message_len)
            .collect::<Vec<_>>();
        assert_eq!(pp.packed_columns(), 128);
        assert_eq!(message_lengths, vec![1, 8, 64, 512]);
        for source_level in 1..pp.sequence.levels() {
            assert_eq!(
                pp.sequence
                    .switch_ratio_for_source_level(source_level)
                    .unwrap(),
                8
            );
            assert_eq!(
                pp.extension_sequence
                    .switch_ratio_for_source_level(source_level)
                    .unwrap(),
                8
            );
        }
        let top_level = pp.sequence.levels() - 1;
        let setup_shape = pp.setup_shape(top_level).unwrap();
        assert_eq!(setup_shape.logical_h_columns, 8);
        assert_eq!(setup_shape.physical_h_columns, 64);
    }

    #[test]
    fn top_decide_auxiliary_rows_use_the_code_switch_ratio() {
        let n_witness = 1 << 13;
        let pc = ShortRaaFoldPC::new(n_witness, 128, 128, 8, 8).unwrap();
        let mut rng = StdRng::seed_from_u64(921);
        let (pp, prover_setup) = pc.setup(&mut rng).unwrap();
        let values = (0..n_witness)
            .map(|index| (index * 31 + 7) as u128)
            .collect::<Vec<_>>();
        let (commitment, cache) = pc.commit_values(&pp, &values).unwrap();
        let point = (0..n_witness.ilog2() as usize)
            .map(|index| field(index as u128 + 3))
            .collect::<Vec<_>>();
        let (value, proof) = pc
            .open(&pp, &prover_setup, &commitment, &point, cache)
            .unwrap();

        assert_eq!(proof.decide.rounds.len(), 1);
        assert_eq!(proof.decide.rounds[0].auxiliary_rows.len(), 2);
        assert_eq!(proof.decide.rounds[0].auxiliary_rows[0][0].row.len(), 128);
        assert_eq!(proof.decide.rounds[0].auxiliary_rows[1][0].row.len(), 512);
        assert!(pc.verify(&pp, &commitment, &point, value, &proof).unwrap());
    }

    #[test]
    fn verifier_accepts_after_prover_setup_is_dropped() {
        let n_witness = 1 << 10;
        let pc = ShortRaaFoldPC::new(n_witness, 2, 8, 4, 8).unwrap();
        let mut rng = StdRng::seed_from_u64(947);
        let (pp, prover_setup) = pc.setup(&mut rng).unwrap();
        let values = (0..n_witness)
            .map(|index| ((index * 29 + 11) & 0xff) as u128)
            .collect::<Vec<_>>();
        let (commitment, cache) = pc.commit_values(&pp, &values).unwrap();
        let point = (0..n_witness.ilog2() as usize)
            .map(|index| field(index as u128 + 5))
            .collect::<Vec<_>>();
        let (value, proof) = pc
            .open(&pp, &prover_setup, &commitment, &point, cache)
            .unwrap();

        drop(prover_setup);

        assert!(pc.verify(&pp, &commitment, &point, value, &proof).unwrap());
    }

    #[test]
    fn tampering_is_rejected() {
        let pc = ShortRaaFoldPC::new(1 << 10, 2, 8, 4, 8).unwrap();
        let mut rng = StdRng::seed_from_u64(991);
        let (pp, prover_setup) = pc.setup(&mut rng).unwrap();
        let values = (0..1 << 10)
            .map(|index| (index & 0xff) as u128)
            .collect::<Vec<_>>();
        let (commitment, cache) = pc.commit_values(&pp, &values).unwrap();
        let point = (0..10).map(|index| field(index + 7)).collect::<Vec<_>>();
        let (value, proof) = pc
            .open(&pp, &prover_setup, &commitment, &point, cache)
            .unwrap();

        let mut changed_opening = proof.clone();
        changed_opening.opening.responses[0] += BinaryExtensionField::ONE;
        assert!(!matches!(
            pc.verify(&pp, &commitment, &point, value, &changed_opening),
            Ok(true)
        ));

        let mut changed_path = proof.clone();
        changed_path.opening.top_rows[0].path.siblings[0][0] ^= 1;
        assert!(!matches!(
            pc.verify(&pp, &commitment, &point, value, &changed_path),
            Ok(true)
        ));

        assert!(
            proof
                .decide
                .generator_proofs
                .iter()
                .all(|generator| generator.auxiliary_roots.len() == 2)
        );
        assert!(proof.opening.extension_rounds.is_empty());
        assert_eq!(proof.decide.rounds.len(), 1);
        assert_eq!(proof.decide.rounds[0].auxiliary_rows.len(), 2);
        assert!(!proof.opening.tail.rows.is_empty());
        assert!(!proof.decide.tail.rows.is_empty());

        let mut changed_opening_tail_row = proof.clone();
        changed_opening_tail_row.opening.tail.rows[0].row[0] += BinaryExtensionField::ONE;
        assert!(!matches!(
            pc.verify(&pp, &commitment, &point, value, &changed_opening_tail_row),
            Ok(true)
        ));

        let mut changed_decide_tail_path = proof.clone();
        changed_decide_tail_path.decide.tail.rows[0].path.siblings[0][0] ^= 1;
        assert!(!matches!(
            pc.verify(&pp, &commitment, &point, value, &changed_decide_tail_path),
            Ok(true)
        ));

        let mut changed_decide = proof.clone();
        changed_decide.decide.generator_proofs[0].auxiliary_roots[0][0] ^= 1;
        assert!(!matches!(
            pc.verify(&pp, &commitment, &point, value, &changed_decide),
            Ok(true)
        ));

        let mut changed_auxiliary_row = proof.clone();
        changed_auxiliary_row.decide.rounds[0].auxiliary_rows[0][0].row[0] +=
            BinaryExtensionField::ONE;
        assert!(!matches!(
            pc.verify(&pp, &commitment, &point, value, &changed_auxiliary_row),
            Ok(true)
        ));

        let mut changed_auxiliary_path = proof;
        changed_auxiliary_path.decide.rounds[0].auxiliary_rows[0][0]
            .path
            .siblings[0][0] ^= 1;
        assert!(!matches!(
            pc.verify(&pp, &commitment, &point, value, &changed_auxiliary_path),
            Ok(true)
        ));

        let (_, fresh_cache) = pc.commit_values(&pp, &values).unwrap();
        let (_, mut changed_setup_row) = pc
            .open(&pp, &prover_setup, &commitment, &point, fresh_cache)
            .unwrap();
        changed_setup_row.decide.rounds[0].setup_rows[0].row[0] += BinaryExtensionField::ONE;
        assert!(!matches!(
            pc.verify(&pp, &commitment, &point, value, &changed_setup_row),
            Ok(true)
        ));

        let (_, fresh_cache) = pc.commit_values(&pp, &values).unwrap();
        let (_, mut changed_setup_path) = pc
            .open(&pp, &prover_setup, &commitment, &point, fresh_cache)
            .unwrap();
        changed_setup_path.decide.rounds[0].setup_rows[0]
            .path
            .siblings[0][0] ^= 1;
        assert!(!matches!(
            pc.verify(&pp, &commitment, &point, value, &changed_setup_path),
            Ok(true)
        ));
    }

    #[test]
    fn decoder_rejects_impossible_counts_at_every_nested_level() {
        fn count(bytes: &mut Vec<u8>, offsets: &mut Vec<usize>) {
            offsets.push(bytes.len());
            write_len(bytes, 1).unwrap();
        }
        fn fields(bytes: &mut Vec<u8>, offsets: &mut Vec<usize>) {
            count(bytes, offsets);
            bytes.extend_from_slice(&[0u8; FIELD_BYTES]);
        }
        fn path(bytes: &mut Vec<u8>, offsets: &mut Vec<usize>) {
            count(bytes, offsets);
            bytes.extend_from_slice(&[0u8; DIGEST_BYTES]);
        }
        fn rows(bytes: &mut Vec<u8>, offsets: &mut Vec<usize>) {
            count(bytes, offsets);
            fields(bytes, offsets);
            path(bytes, offsets);
        }
        fn tail(bytes: &mut Vec<u8>, offsets: &mut Vec<usize>) {
            bytes.extend_from_slice(&[0u8; DIGEST_BYTES]);
            rows(bytes, offsets);
        }

        // A structurally canonical fixture populates every nested container,
        // including both optional branches. It is not a valid PCS statement.
        let mut bytes = PROOF_MAGIC.to_vec();
        let mut offsets = Vec::new();
        fields(&mut bytes, &mut offsets);
        count(&mut bytes, &mut offsets);
        write_len(&mut bytes, 0).unwrap();
        fields(&mut bytes, &mut offsets);
        path(&mut bytes, &mut offsets);
        count(&mut bytes, &mut offsets);
        bytes.extend_from_slice(&[0u8; DIGEST_BYTES]);
        rows(&mut bytes, &mut offsets);
        tail(&mut bytes, &mut offsets);
        count(&mut bytes, &mut offsets);
        write_len(&mut bytes, 0).unwrap();
        count(&mut bytes, &mut offsets);
        bytes.extend_from_slice(&[0u8; DIGEST_BYTES]);
        fields(&mut bytes, &mut offsets);
        count(&mut bytes, &mut offsets);
        write_len(&mut bytes, 0).unwrap();
        rows(&mut bytes, &mut offsets);
        count(&mut bytes, &mut offsets);
        rows(&mut bytes, &mut offsets);
        bytes.push(1);
        rows(&mut bytes, &mut offsets);
        bytes.push(1);
        bytes.extend_from_slice(&[0u8; DIGEST_BYTES]);
        tail(&mut bytes, &mut offsets);
        assert_eq!(
            ShortRaaFoldProof::from_bytes(&bytes)
                .unwrap()
                .to_bytes()
                .unwrap(),
            bytes
        );

        for offset in offsets {
            for invalid_count in [1_000_000u64, u64::MAX] {
                let mut malformed = bytes.clone();
                malformed[offset..offset + 8].copy_from_slice(&invalid_count.to_le_bytes());
                assert!(
                    ShortRaaFoldProof::from_bytes(&malformed).is_err(),
                    "count at byte {offset}"
                );
            }
        }
        for prefix_len in 0..bytes.len() {
            assert!(ShortRaaFoldProof::from_bytes(&bytes[..prefix_len]).is_err());
        }
        bytes.push(0);
        assert!(ShortRaaFoldProof::from_bytes(&bytes).is_err());
    }

    #[test]
    fn decoder_count_bounds_are_checked_before_reserving() {
        for minimum in [8usize, 16, 24, 26, 32, 40] {
            let mut short = 1u64.to_le_bytes().to_vec();
            short.resize(8 + minimum - 1, 0);
            assert!(ProofReader::new(&short).read_count(minimum).is_err());
            short.push(0);
            assert_eq!(ProofReader::new(&short).read_count(minimum).unwrap(), 1);
        }
        let mut short = PROOF_MAGIC.to_vec();
        short.extend_from_slice(&u64::MAX.to_le_bytes());
        assert!(ShortRaaFoldProof::from_bytes(&short).is_err());
    }

    #[test]
    fn decoder_rejects_trailing_and_truncated_bytes() {
        let proof = ShortRaaFoldProof {
            opening: ShortRaaOpeningProof {
                responses: vec![field(3)],
                top_rows: Vec::new(),
                extension_rounds: Vec::new(),
                tail: ShortRaaLigeroTailProof {
                    root: [1u8; DIGEST_BYTES],
                    rows: Vec::new(),
                },
            },
            decide: ShortRaaDecideProof {
                generator_proofs: Vec::new(),
                responses: Vec::new(),
                rounds: Vec::new(),
                tail: ShortRaaLigeroTailProof {
                    root: [2u8; DIGEST_BYTES],
                    rows: Vec::new(),
                },
            },
        };
        let bytes = proof.to_bytes().unwrap();
        let mut trailing = bytes.clone();
        trailing.push(0);
        assert!(ShortRaaFoldProof::from_bytes(&trailing).is_err());
        assert!(ShortRaaFoldProof::from_bytes(&bytes[..bytes.len() - 1]).is_err());
    }
}
