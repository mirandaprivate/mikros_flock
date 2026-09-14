//! Word copy constraints reduced to two bit-MLE queries on the original packed
//! witness. Every one of the word's 128 coordinates participates.
//!
//! The public permutation is validated and bound here. The caller MUST bind the
//! real witness commitment first and discharge both returned claims using its
//! PCS. Public/dummy circuit-cell recombination is an outer-layer obligation.

use crate::{
    Digest, Error, MAX_LOG_ROWS,
    bits::PackedBits,
    claims::{LinearQuery, Source},
    field::{SmallField, evaluate_linear, linear_tables},
    permutation::{self, CommittedClaims, CommittedStatement, Options, Proof, Stats},
    profile::{Engine, Timings, elapsed},
};
use flock_core::zerocheck::univariate_skip::SplitEqGhash;
use flock_field::{F128, F256Unreduced};
use flock_transcript::challenger::Challenger;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Validated verifier-known permutation. Construct once during offline circuit
/// preparation. No deserializer bypasses the permutation and size checks.
#[derive(Clone, Debug)]
pub struct WordWiringStatement {
    sigma: Vec<usize>,
    digest: Digest,
    shape: CommittedStatement,
}

impl WordWiringStatement {
    pub fn new(sigma: &[usize]) -> Result<Self, Error> {
        let words = sigma.len();
        if words == 0 || !words.is_power_of_two() || words > (1usize << (MAX_LOG_ROWS - 7)) {
            return Err(Error::Length);
        }
        let mut seen = vec![false; words];
        for &image in sigma {
            if image >= words || std::mem::replace(&mut seen[image], true) {
                return Err(Error::Statement);
            }
        }
        let rows = words * 128;
        let shape = CommittedStatement::new(rows, 1 + rows.ilog2() as usize)?;
        let mut hash = blake3::Hasher::new();
        hash.update(b"mikros-public-word-permutation-v1");
        hash.update(&(words as u64).to_le_bytes());
        for &image in sigma {
            hash.update(&(image as u64).to_le_bytes());
        }
        Ok(Self {
            sigma: sigma.to_vec(),
            digest: *hash.finalize().as_bytes(),
            shape,
        })
    }

    pub fn words(&self) -> usize {
        self.sigma.len()
    }

    pub fn sigma(&self) -> &[usize] {
        &self.sigma
    }

    pub fn digest(&self) -> Digest {
        self.digest
    }

    pub fn shape(&self) -> &CommittedStatement {
        &self.shape
    }

    /// Low seven coordinates identify the bit within the word. A public word
    /// permutation preserves those coordinates exactly.
    pub fn bit_sigma(&self, row: usize) -> usize {
        assert!(row < self.shape.rows);
        self.sigma[row >> 7] * 128 + (row & 127)
    }

    fn bind<C: Challenger>(&self, c: &mut C) {
        c.observe_label(b"mikros-committed-word-wiring-v1");
        c.observe_bytes(&(self.words() as u64).to_le_bytes());
        c.observe_bytes(&self.digest);
    }
}

/// Private offline prover storage tied to one checked public permutation.
pub struct PreparedWiring {
    permutation: permutation::PreparedPermutation,
    sigma_digest: Digest,
}

impl PreparedWiring {
    pub fn permutation(&self) -> &permutation::PreparedPermutation {
        &self.permutation
    }
}

/// Strict offline wiring state. Source tuples are decoded and every compact
/// coefficient layer is cached. Original PackedBits may be dropped afterwards.
pub struct PreparedWiringCoefficients {
    permutation: permutation::PreparedCoefficients,
    sigma_digest: Digest,
}

impl PreparedWiringCoefficients {
    pub fn permutation(&self) -> &permutation::PreparedCoefficients {
        &self.permutation
    }

    /// Retained coefficient and private leaf-table capacities, excluding the
    /// public statement's sigma/cache and separately retained witness/sources.
    pub fn resident_bytes(&self) -> usize {
        self.permutation.resident_bytes()
    }
}

/// Offline tree preparation for either bit-defined or word-defined wiring.
pub fn prepare_bit_coefficients(
    source: &PreparedWiring,
    options: Options,
) -> Result<PreparedWiringCoefficients, Error> {
    Ok(PreparedWiringCoefficients {
        permutation: permutation::prepare_tree_coefficients(&source.permutation, options)?,
        sigma_digest: source.sigma_digest,
    })
}

/// An ordinary bit-MLE query: point coordinates are low-to-high, and bit
/// `128 * word + bit` is polynomial-basis coordinate `bit` of that F128 word.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BitMleClaim {
    pub point: Vec<F128>,
    pub value: F128,
}

#[inline]
fn word_bit(word: F128, bit: usize) -> u128 {
    if bit < 64 {
        ((word.lo >> bit) & 1) as u128
    } else {
        ((word.hi >> (bit - 64)) & 1) as u128
    }
}

/// Offline expansion. Both sides use the same original word vector; only the
/// public position tag differs. This proves w[sigma[i]] = w[i].
pub fn prepare(words: &[F128], statement: &WordWiringStatement) -> Result<PreparedWiring, Error> {
    if words.len() != statement.words() {
        return Err(Error::Length);
    }
    let shape = statement.shape;
    let left = PackedBits::from_fn_parallel(shape.rows, shape.width, |row| {
        word_bit(words[row >> 7], row & 127) | ((row as u128) << 1)
    })?;
    let right = PackedBits::from_fn_parallel(shape.rows, shape.width, |row| {
        word_bit(words[row >> 7], row & 127) | ((statement.bit_sigma(row) as u128) << 1)
    })?;
    Ok(PreparedWiring {
        permutation: permutation::prepare(left, right)?,
        sigma_digest: statement.digest,
    })
}

/// The identity tag has a closed-form MLE. For sigma, split off the seven
/// preserved bit coordinates and evaluate only the public word permutation.
/// This is a public-table computation, not witness/source authentication.
fn tag_eval(statement: &WordWiringStatement, point: &[F128], permuted: bool) -> F128 {
    let basis = SmallField::for_width(statement.shape.width)
        .expect("validated shape")
        .basis();
    if !permuted {
        return point
            .iter()
            .zip(&basis[1..])
            .fold(F128::ZERO, |a, (&r, &b)| a + r * b);
    }
    let low = point[..7]
        .iter()
        .zip(&basis[1..8])
        .fold(F128::ZERO, |a, (&r, &b)| a + r * b);
    let word_point = &point[7..];
    if word_point.is_empty() {
        return low;
    }
    let tables = linear_tables(&basis[8..8 + word_point.len()]);
    let eq = SplitEqGhash::with_n_hi(word_point, word_point.len() / 2);
    let high = (0..eq.hi.len())
        .into_par_iter()
        .map(|hi| {
            let base = hi * eq.lo.len();
            let mut sum = F256Unreduced::ZERO;
            for (lo, &weight) in eq.lo.iter().enumerate() {
                let value = evaluate_linear(&tables, statement.sigma[base + lo] as u128);
                sum ^= value.mul_unreduced(weight);
            }
            sum.reduce() * eq.hi[hi]
        })
        .reduce(|| F128::ZERO, |a, b| a + b);
    low + high
}

/// Reduce the two complete source queries by verifier-known position tags.
/// Reject unexpected shape, basis, offset, source order, or point length.
pub fn reduce_claims(
    statement: &WordWiringStatement,
    claims: &CommittedClaims,
) -> Result<[BitMleClaim; 2], Error> {
    let shape = statement.shape;
    let expected = &SmallField::for_width(shape.width)?.basis()[..shape.width];
    let log_rows = shape.rows.ilog2() as usize;
    let mut out = Vec::with_capacity(2);
    for (i, (claim, source)) in claims
        .evaluations
        .iter()
        .zip([Source::Left, Source::Right])
        .enumerate()
    {
        let LinearQuery::Rows {
            point,
            basis,
            row_offset,
        } = &claim.query
        else {
            return Err(Error::ProofShape);
        };
        if claim.source != source
            || *row_offset != 0
            || point.len() != log_rows
            || basis.as_slice() != expected
            || basis[0] != F128::ONE
        {
            return Err(Error::ProofShape);
        }
        out.push(BitMleClaim {
            point: point.clone(),
            value: claim.value + tag_eval(statement, point, i == 1),
        });
    }
    Ok(out.try_into().expect("exactly two leaves"))
}

/// Online permutation proof and reduction to two PCS obligations. Bind the
/// original witness commitment before calling; authenticate both obligations.
pub fn prove<C: Challenger>(
    statement: &WordWiringStatement,
    prepared: &PreparedWiring,
    options: Options,
    engine: Engine,
    c: &mut C,
) -> Result<(Proof, [BitMleClaim; 2], Stats, Timings), Error> {
    if prepared.sigma_digest != statement.digest
        || prepared.permutation.statement() != &statement.shape
    {
        return Err(Error::Statement);
    }
    statement.bind(c);
    let (proof, claims, stats, mut timings) =
        permutation::prove_committed(&prepared.permutation, options, engine, c)?;
    let start = Instant::now();
    let reduced = reduce_claims(statement, &claims)?;
    timings.source_reduction_ms = elapsed(start);
    Ok((proof, reduced, stats, timings))
}

/// Replay using only the public permutation and proof. Successful return still
/// requires PCS verification of both returned bit-MLE claims.
pub fn verify<C: Challenger>(
    statement: &WordWiringStatement,
    proof: &Proof,
    c: &mut C,
) -> Result<[BitMleClaim; 2], Error> {
    statement.bind(c);
    let claims = permutation::verify_committed(&statement.shape, proof, c)?;
    reduce_claims(statement, &claims)
}

/// Word-preserving wiring with an exact offline/online coefficient boundary.
pub fn prove_prepared<C: Challenger>(
    statement: &WordWiringStatement,
    prepared: &PreparedWiringCoefficients,
    engine: Engine,
    c: &mut C,
) -> Result<(Proof, [BitMleClaim; 2], Stats, Timings), Error> {
    if prepared.sigma_digest != statement.digest
        || prepared.permutation.statement() != &statement.shape
    {
        return Err(Error::Statement);
    }
    statement.bind(c);
    let (proof, claims, stats, mut timings) =
        permutation::prove_prepared(&prepared.permutation, engine, c)?;
    let start = Instant::now();
    let reduced = reduce_claims(statement, &claims)?;
    timings.source_reduction_ms = elapsed(start);
    Ok((proof, reduced, stats, timings))
}

/// A public permutation defined directly on bit addresses. Unlike
/// `WordWiringStatement`, images may move a bit to any coordinate of any word.
/// Validation, digesting, and the field encoding of every sigma tag are offline.
#[derive(Clone, Debug)]
pub struct BitWiringStatement {
    sigma: Vec<usize>,
    sigma_tags: Vec<F128>,
    digest: Digest,
    shape: CommittedStatement,
}

impl BitWiringStatement {
    pub fn new(sigma: &[usize]) -> Result<Self, Error> {
        let bits = sigma.len();
        if bits == 0 || !bits.is_power_of_two() || bits > 1usize << MAX_LOG_ROWS {
            return Err(Error::Length);
        }
        let mut seen = vec![false; bits];
        for &image in sigma {
            if image >= bits || std::mem::replace(&mut seen[image], true) {
                return Err(Error::Statement);
            }
        }
        let shape = CommittedStatement::new(bits, 1 + bits.ilog2() as usize)?;
        let basis = SmallField::for_width(shape.width)?.basis();
        let tables = linear_tables(&basis[1..shape.width]);
        let sigma_tags = sigma
            .par_iter()
            .map(|&image| evaluate_linear(&tables, image as u128))
            .collect();
        let mut hash = blake3::Hasher::new();
        hash.update(b"mikros-public-bit-permutation-v1");
        hash.update(&(bits as u64).to_le_bytes());
        for &image in sigma {
            hash.update(&(image as u64).to_le_bytes());
        }
        Ok(Self {
            sigma: sigma.to_vec(),
            sigma_tags,
            digest: *hash.finalize().as_bytes(),
            shape,
        })
    }

    pub fn bits(&self) -> usize {
        self.shape.rows
    }

    pub fn sigma(&self) -> &[usize] {
        &self.sigma
    }

    /// The challenge-independent field encodings of public sigma tags. This
    /// offline cache costs 16 bytes per bit address, in addition to sigma.
    pub fn encoded_sigma_tags(&self) -> &[F128] {
        &self.sigma_tags
    }

    pub fn digest(&self) -> Digest {
        self.digest
    }

    pub fn shape(&self) -> &CommittedStatement {
        &self.shape
    }

    fn bind<C: Challenger>(&self, c: &mut C) {
        c.observe_label(b"mikros-committed-bit-wiring-v1");
        c.observe_bytes(&(self.bits() as u64).to_le_bytes());
        c.observe_bytes(&self.digest);
    }
}

/// Offline conversion of an ordinary packed bit witness to the two tagged
/// column-major source oracles. The bit at address i is coordinate i % 128 of
/// packed[i / 128]. Unused padding bits in a sub-word witness must be zero.
pub fn prepare_bits(
    packed: &[F128],
    statement: &BitWiringStatement,
) -> Result<PreparedWiring, Error> {
    let shape = statement.shape;
    if packed.len() != shape.rows.div_ceil(128) {
        return Err(Error::Length);
    }
    if shape.rows < 128 {
        let raw = (packed[0].lo as u128) | ((packed[0].hi as u128) << 64);
        if raw >> shape.rows != 0 {
            return Err(Error::Padding);
        }
    }
    let left = PackedBits::from_fn_parallel(shape.rows, shape.width, |row| {
        word_bit(packed[row >> 7], row & 127) | ((row as u128) << 1)
    })?;
    let right = PackedBits::from_fn_parallel(shape.rows, shape.width, |row| {
        word_bit(packed[row >> 7], row & 127) | ((statement.sigma[row] as u128) << 1)
    })?;
    Ok(PreparedWiring {
        permutation: permutation::prepare(left, right)?,
        sigma_digest: statement.digest,
    })
}

fn bit_tag_eval(statement: &BitWiringStatement, point: &[F128], permuted: bool) -> F128 {
    if !permuted {
        let basis = SmallField::for_width(statement.shape.width)
            .expect("validated shape")
            .basis();
        return point
            .iter()
            .zip(&basis[1..])
            .fold(F128::ZERO, |a, (&r, &b)| a + r * b);
    }
    if point.is_empty() {
        return statement.sigma_tags[0];
    }
    let eq = SplitEqGhash::with_n_hi(point, point.len() / 2);
    (0..eq.hi.len())
        .into_par_iter()
        .map(|hi| {
            let base = hi * eq.lo.len();
            let mut sum = F256Unreduced::ZERO;
            for (lo, &weight) in eq.lo.iter().enumerate() {
                sum ^= statement.sigma_tags[base + lo].mul_unreduced(weight);
            }
            sum.reduce() * eq.hi[hi]
        })
        .reduce(|| F128::ZERO, |a, b| a + b)
}

/// Convert each verified leaf to a query on the original committed bit witness.
/// Sigma's field encodings are cached offline; their challenge-dependent MLE
/// is calculated here and is online work on both prover and verifier.
pub fn reduce_bit_claims(
    statement: &BitWiringStatement,
    claims: &CommittedClaims,
) -> Result<[BitMleClaim; 2], Error> {
    let shape = statement.shape;
    let expected = &SmallField::for_width(shape.width)?.basis()[..shape.width];
    let log_rows = shape.rows.ilog2() as usize;
    let mut out = Vec::with_capacity(2);
    for (i, (claim, source)) in claims
        .evaluations
        .iter()
        .zip([Source::Left, Source::Right])
        .enumerate()
    {
        let LinearQuery::Rows {
            point,
            basis,
            row_offset,
        } = &claim.query
        else {
            return Err(Error::ProofShape);
        };
        if claim.source != source
            || *row_offset != 0
            || point.len() != log_rows
            || basis.as_slice() != expected
            || basis[0] != F128::ONE
        {
            return Err(Error::ProofShape);
        }
        out.push(BitMleClaim {
            point: point.clone(),
            value: claim.value + bit_tag_eval(statement, point, i == 1),
        });
    }
    Ok(out.try_into().expect("exactly two leaves"))
}

/// Online proof for a directly specified bit wiring. Bind the actual commitment
/// first and route BOTH outputs to the PCS; no full-source check is substituted.
pub fn prove_bits<C: Challenger>(
    statement: &BitWiringStatement,
    prepared: &PreparedWiring,
    options: Options,
    engine: Engine,
    c: &mut C,
) -> Result<(Proof, [BitMleClaim; 2], Stats, Timings), Error> {
    if prepared.sigma_digest != statement.digest
        || prepared.permutation.statement() != &statement.shape
    {
        return Err(Error::Statement);
    }
    statement.bind(c);
    let (proof, claims, stats, mut timings) =
        permutation::prove_committed(&prepared.permutation, options, engine, c)?;
    let start = Instant::now();
    let reduced = reduce_bit_claims(statement, &claims)?;
    timings.source_reduction_ms = elapsed(start);
    Ok((proof, reduced, stats, timings))
}

/// Replay the bit wiring against verifier-known sigma and return two pending
/// PCS obligations on the original packed witness.
pub fn verify_bits<C: Challenger>(
    statement: &BitWiringStatement,
    proof: &Proof,
    c: &mut C,
) -> Result<[BitMleClaim; 2], Error> {
    statement.bind(c);
    let claims = permutation::verify_committed(&statement.shape, proof, c)?;
    reduce_bit_claims(statement, &claims)
}

/// Strict online proof of bit-defined wiring. Offline source decoding and
/// coefficient construction are excluded by the prepared API boundary, not by
/// subtracting phase measurements. Public sigma evaluation remains online.
pub fn prove_bits_prepared<C: Challenger>(
    statement: &BitWiringStatement,
    prepared: &PreparedWiringCoefficients,
    engine: Engine,
    c: &mut C,
) -> Result<(Proof, [BitMleClaim; 2], Stats, Timings), Error> {
    if prepared.sigma_digest != statement.digest
        || prepared.permutation.statement() != &statement.shape
    {
        return Err(Error::Statement);
    }
    statement.bind(c);
    let (proof, claims, stats, mut timings) =
        permutation::prove_prepared(&prepared.permutation, engine, c)?;
    let start = Instant::now();
    let reduced = reduce_bit_claims(statement, &claims)?;
    timings.source_reduction_ms = elapsed(start);
    Ok((proof, reduced, stats, timings))
}

/// Close the public-tag reduction for a synchronized copy-constraint proof.
/// Encoded source values generally differ: only the original bit-MLE values,
/// after their respective id/sigma tags have been removed, must coincide.
/// The surviving claim still needs authentication against the original PCS.
pub fn reduce_bit_claims_lockstep(
    statement: &BitWiringStatement,
    claims: &CommittedClaims,
) -> Result<BitMleClaim, Error> {
    let [left, right] = reduce_bit_claims(statement, claims)?;
    if left.point != right.point {
        return Err(Error::ProofShape);
    }
    if left.value != right.value {
        return Err(Error::Constraint);
    }
    Ok(left)
}

/// Synchronized bit wiring with online coefficient construction. Both tree
/// equations are verified before their tagged leaves reduce to one bit-MLE
/// claim. The actual original-witness commitment must already be bound.
pub fn prove_bits_lockstep<C: Challenger>(
    statement: &BitWiringStatement,
    prepared: &PreparedWiring,
    options: Options,
    engine: Engine,
    c: &mut C,
) -> Result<(Proof, BitMleClaim, Stats, Timings), Error> {
    if prepared.sigma_digest != statement.digest
        || prepared.permutation.statement() != &statement.shape
    {
        return Err(Error::Statement);
    }
    statement.bind(c);
    let (proof, claims, stats, mut timings) =
        permutation::prove_committed_lockstep(&prepared.permutation, options, engine, c)?;
    let start = Instant::now();
    let reduced = reduce_bit_claims_lockstep(statement, &claims)?;
    timings.source_reduction_ms = elapsed(start);
    Ok((proof, reduced, stats, timings))
}

/// Strict online synchronized bit-wiring entry. Offline coefficients and tag
/// encodings are reused; z-dependent evaluation, tree/GKR work and the public
/// sigma MLE remain online. Only one original-witness PCS claim is returned.
pub fn prove_bits_prepared_lockstep<C: Challenger>(
    statement: &BitWiringStatement,
    prepared: &PreparedWiringCoefficients,
    engine: Engine,
    c: &mut C,
) -> Result<(Proof, BitMleClaim, Stats, Timings), Error> {
    if prepared.sigma_digest != statement.digest
        || prepared.permutation.statement() != &statement.shape
    {
        return Err(Error::Statement);
    }
    statement.bind(c);
    let (proof, claims, stats, mut timings) =
        permutation::prove_prepared_lockstep(&prepared.permutation, engine, c)?;
    let start = Instant::now();
    let reduced = reduce_bit_claims_lockstep(statement, &claims)?;
    timings.source_reduction_ms = elapsed(start);
    Ok((proof, reduced, stats, timings))
}

/// Replay both synchronized tree constraints, remove the independently bound
/// public tags and require both source evaluations to coincide. There is no
/// witness parameter: success is conditional on authenticating the returned
/// claim with the outer PCS verifier.
pub fn verify_bits_lockstep<C: Challenger>(
    statement: &BitWiringStatement,
    proof: &Proof,
    c: &mut C,
) -> Result<BitMleClaim, Error> {
    statement.bind(c);
    let claims = permutation::verify_committed_lockstep(&statement.shape, proof, c)?;
    reduce_bit_claims_lockstep(statement, &claims)
}

/// A functional on the original packed witness. Columns index the 128 bits
/// within a word; row factors are low-address-bit first. For fewer than 128
/// witness bits, unused columns have zero weight.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BitTensorClaim {
    pub column_weights: Vec<F128>,
    pub row_factors: Vec<[F128; 2]>,
    pub value: F128,
}

/// Challenge-independent preparation for the packed column-reduction protocol.
pub struct PreparedPackedWiring {
    permutation: crate::packed::Prepared,
    sigma_digest: Digest,
}

impl PreparedPackedWiring {
    pub fn resident_bytes(&self) -> usize {
        self.permutation.resident_bytes()
    }

    pub fn kappa(&self) -> usize {
        self.permutation.kappa()
    }
}

pub fn prepare_bits_packed(
    packed: &[F128],
    statement: &BitWiringStatement,
) -> Result<PreparedPackedWiring, Error> {
    let source = prepare_bits(packed, statement)?;
    Ok(PreparedPackedWiring {
        permutation: crate::packed::prepare(source.permutation())?,
        sigma_digest: statement.digest,
    })
}

/// Packing factor of the tuple field, distinct from the witness's 128-bit word.
pub fn bit_packed_kappa(statement: &BitWiringStatement) -> Result<usize, Error> {
    Ok(128 / SmallField::for_width(statement.shape.width)?.bits)
}

fn packed_tag_eval(
    statement: &BitWiringStatement,
    claim: &crate::packed::Claim,
    permuted: bool,
) -> F128 {
    let kappa = claim.column_weights.len();
    let low_bits = kappa.ilog2() as usize;
    if !permuted {
        let basis = SmallField::for_width(statement.shape.width)
            .expect("validated packed wiring shape")
            .basis();
        let low =
            claim
                .column_weights
                .iter()
                .enumerate()
                .fold(F128::ZERO, |sum, (column, &weight)| {
                    let tag = (0..low_bits).fold(F128::ZERO, |tag, bit| {
                        if column & (1 << bit) == 0 {
                            tag
                        } else {
                            tag + basis[1 + bit]
                        }
                    });
                    sum + weight * tag
                });
        return claim
            .row_point
            .iter()
            .enumerate()
            .fold(low, |sum, (bit, &r)| sum + r * basis[1 + low_bits + bit]);
    }
    let eq = SplitEqGhash::with_n_hi(&claim.row_point, (claim.row_point.len() + low_bits) / 2);
    // These low-address weights are shared by every high block. Folding rho
    // into this small table avoids an extra field product and reduction for
    // every source row during the full sigma scan.
    let low_weights = eq
        .lo
        .iter()
        .flat_map(|&weight| claim.column_weights.iter().map(move |&rho| weight * rho))
        .collect::<Vec<_>>();
    (0..eq.hi.len())
        .into_par_iter()
        .map(|hi| {
            let mut sum = F256Unreduced::ZERO;
            let base = hi * low_weights.len();
            for (lo, &weight) in low_weights.iter().enumerate() {
                sum ^= statement.sigma_tags[base + lo].mul_unreduced(weight);
            }
            sum.reduce() * eq.hi[hi]
        })
        .reduce(|| F128::ZERO, |a, b| a + b)
}

/// Remove both independently evaluated public tags, then authenticate their
/// common bit functional. No witness is available to this verifier adapter.
pub fn reduce_bit_claims_packed(
    statement: &BitWiringStatement,
    claims: &[crate::packed::Claim; 2],
) -> Result<BitTensorClaim, Error> {
    let kappa = bit_packed_kappa(statement)?;
    let log_bits = statement.bits().ilog2() as usize;
    if kappa < 2 || statement.bits() < kappa {
        return Err(Error::ProofShape);
    }
    let column_log = kappa.ilog2() as usize;
    for claim in claims {
        if claim.column_weights.len() != kappa
            || claim.row_point.len() != log_bits - column_log
            || claim
                .column_weights
                .iter()
                .copied()
                .fold(F128::ZERO, |a, b| a + b)
                != F128::ONE
        {
            return Err(Error::ProofShape);
        }
    }
    if claims[0].column_weights != claims[1].column_weights
        || claims[0].row_point != claims[1].row_point
    {
        return Err(Error::ProofShape);
    }
    let left = claims[0].value + packed_tag_eval(statement, &claims[0], false);
    let right = claims[1].value + packed_tag_eval(statement, &claims[1], true);
    if left != right {
        return Err(Error::Constraint);
    }
    let mut columns = claims[0].column_weights.clone();
    let low_rows = (7usize.saturating_sub(column_log)).min(claims[0].row_point.len());
    for &r in &claims[0].row_point[..low_rows] {
        let width = columns.len();
        columns.resize(width * 2, F128::ZERO);
        for i in 0..width {
            let value = columns[i];
            columns[i] = value * (F128::ONE + r);
            columns[width + i] = value * r;
        }
    }
    columns.resize(128, F128::ZERO);
    Ok(BitTensorClaim {
        column_weights: columns,
        row_factors: claims[0].row_point[low_rows..]
            .iter()
            .map(|&r| [F128::ONE + r, r])
            .collect(),
        value: left,
    })
}

pub fn prove_bits_prepared_packed<C: Challenger>(
    statement: &BitWiringStatement,
    prepared: &PreparedPackedWiring,
    c: &mut C,
) -> Result<(crate::packed::Proof, BitTensorClaim, Timings), Error> {
    if prepared.sigma_digest != statement.digest
        || prepared.permutation.statement() != &statement.shape
    {
        return Err(Error::Statement);
    }
    statement.bind(c);
    let (proof, claims, mut timings) = crate::packed::prove(&prepared.permutation, c)?;
    let start = Instant::now();
    let claim = reduce_bit_claims_packed(statement, &claims)?;
    timings.source_reduction_ms = elapsed(start);
    Ok((proof, claim, timings))
}

pub fn verify_bits_packed<C: Challenger>(
    statement: &BitWiringStatement,
    proof: &crate::packed::Proof,
    c: &mut C,
) -> Result<BitTensorClaim, Error> {
    statement.bind(c);
    let claims = crate::packed::verify(&statement.shape, proof, c)?;
    reduce_bit_claims_packed(statement, &claims)
}
