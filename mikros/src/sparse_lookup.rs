//! Binary sparse-matrix evaluation with indexed offline memory checking.
//!
//! This is the sparse-evaluation PIOP, not a polynomial commitment. Circuit
//! preprocessing fixes and authenticates every static column, including the
//! integer read/write/final timestamps and the binary value/padding column.
//! After x and y are known, the prover forms and commits the two gathered
//! equality tables BEFORE calling `prove`. The caller binds all commitments
//! and authenticates EVERY returned evaluation with its native PCS.
//!
//! Timestamps are u64 integers advanced during trusted preprocessing. Both
//! sides of each increment are committed independently; adding F128::ONE
//! would be an incorrect replacement for the integer increment. Verification
//! takes only the shape/digest, the query and the proof. It never scans the
//! matrix or a public equality table. It relies on the trusted preprocessing
//! statement for the address ranges, timestamp schedule and zero padding.
//!
//! A cubic sumcheck proves sum val[j]*read_row[j]*read_col[j]. Each gather is
//! checked by Init + Write = Read + Final using two complete product-tree
//! reductions. All points are LOW BIT FIRST. The two memory segments each
//! have D=max(table_len,read_len) slots; unused leaves are the public ONE.

use flock_core::r1cs::SparseBinaryMatrix;
use flock_field::{F128, F256Unreduced, f128_slice::fold_pairs};
use flock_transcript::challenger::Challenger;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::extension_product;

const MAX_LOG: usize = 30;
const CHUNK: usize = 4096;

#[cfg(test)]
thread_local! {
    static FUSION_OVERRIDE: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
    static CACHE_LEAVES_OVERRIDE: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
    static CACHE_BASE_REUSE_OVERRIDE: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
}

fn fusion_enabled() -> bool {
    #[cfg(test)]
    if let Some(value) = FUSION_OVERRIDE.get() {
        return value;
    }
    std::env::var_os("MIKROS_NO_SPARSE_LOOKUP_FUSION").is_none()
}

fn cache_leaves_enabled() -> bool {
    #[cfg(test)]
    if let Some(value) = CACHE_LEAVES_OVERRIDE.get() {
        return value;
    }
    std::env::var_os("MIKROS_SPARSE_CACHE_LEAVES").is_some_and(|value| value == "1")
}

fn cache_base_reuse_enabled() -> bool {
    #[cfg(test)]
    if let Some(value) = CACHE_BASE_REUSE_OVERRIDE.get() {
        return value;
    }
    !std::env::var_os("MIKROS_NO_SPARSE_CACHE_BASE_REUSE").is_some_and(|value| value == "1")
}

#[cfg(test)]
fn with_fusion<T>(enabled: bool, run: impl FnOnce() -> T) -> T {
    struct Restore(Option<bool>);
    impl Drop for Restore {
        fn drop(&mut self) {
            FUSION_OVERRIDE.set(self.0);
        }
    }
    let _restore = Restore(FUSION_OVERRIDE.replace(Some(enabled)));
    run()
}

#[cfg(test)]
fn with_cache_leaves<T>(enabled: bool, run: impl FnOnce() -> T) -> T {
    struct Restore(Option<bool>);
    impl Drop for Restore {
        fn drop(&mut self) {
            CACHE_LEAVES_OVERRIDE.set(self.0);
        }
    }
    let _restore = Restore(CACHE_LEAVES_OVERRIDE.replace(Some(enabled)));
    run()
}

#[cfg(test)]
fn with_cache_base_reuse<T>(enabled: bool, run: impl FnOnce() -> T) -> T {
    struct Restore(Option<bool>);
    impl Drop for Restore {
        fn drop(&mut self) {
            CACHE_BASE_REUSE_OVERRIDE.set(self.0);
        }
    }
    let _restore = Restore(CACHE_BASE_REUSE_OVERRIDE.replace(Some(enabled)));
    run()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OracleId {
    RowAddress,
    ColAddress,
    RowReadTime,
    RowWriteTime,
    ColReadTime,
    ColWriteTime,
    RowFinalTime,
    ColFinalTime,
    Value,
    ReadRow,
    ReadCol,
}

impl OracleId {
    pub const STATIC: [Self; 9] = [
        Self::RowAddress,
        Self::ColAddress,
        Self::RowReadTime,
        Self::RowWriteTime,
        Self::ColReadTime,
        Self::ColWriteTime,
        Self::RowFinalTime,
        Self::ColFinalTime,
        Self::Value,
    ];
}

/// Small verifier statement. The digest identifies the caller's trusted
/// preprocessed matrix commitment, including all nine static columns.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Statement {
    pub row_log: usize,
    pub col_log: usize,
    pub entry_log: usize,
    pub nonzeros: usize,
    pub digest: [u8; 32],
}

impl Statement {
    pub fn validate(&self) -> Result<(), Error> {
        for log in [self.row_log, self.col_log, self.entry_log] {
            checked_len(log)?;
        }
        if self.nonzeros > 1usize << self.entry_log
            || self.nonzeros.max(1).next_power_of_two() != 1usize << self.entry_log
        {
            return Err(Error::Shape);
        }
        Ok(())
    }

    pub fn column_log(&self, source: OracleId) -> usize {
        match source {
            OracleId::RowFinalTime => self.row_log,
            OracleId::ColFinalTime => self.col_log,
            _ => self.entry_log,
        }
    }

    /// Raw polynomial-basis bits needed by the native packed source carrier.
    pub fn column_bits(&self, source: OracleId) -> usize {
        match source {
            OracleId::RowAddress => self.row_log.max(1),
            OracleId::ColAddress => self.col_log.max(1),
            OracleId::Value => 1,
            OracleId::ReadRow | OracleId::ReadCol => 128,
            _ => self.entry_log + 1,
        }
    }
}

/// Offline prover data. A static column is never materialized as F128 here.
/// The caller may release this after its final online proof if desired.
#[derive(Clone, Debug)]
pub struct Prepared {
    pub row_log: usize,
    pub col_log: usize,
    pub entry_log: usize,
    pub nonzeros: usize,
    pub row: Vec<u64>,
    pub col: Vec<u64>,
    pub row_read_time: Vec<u64>,
    pub row_write_time: Vec<u64>,
    pub col_read_time: Vec<u64>,
    pub col_write_time: Vec<u64>,
    pub row_final_time: Vec<u64>,
    pub col_final_time: Vec<u64>,
    pub value: Vec<u64>,
}

impl Prepared {
    /// Canonicalize each binary matrix row by XORing its column postings.
    /// Repeated even-multiplicity entries are zero in GF(2), and never enter
    /// the committed COO or its timestamp log. A Rayon worker reuses its
    /// bitset across rows; the original circuit matrix remains unchanged.
    pub fn new(matrix: &SparseBinaryMatrix) -> Result<Self, Error> {
        Self::new_impl(matrix, false)
    }

    /// The same canonical matrix with zero-coefficient padding reads spread
    /// across valid row/column addresses instead of concentrated at zero.
    /// Record i uses (i % num_rows, i % num_cols). Every padding record still
    /// participates in both complete integer-timestamp memory checks.
    ///
    /// This is offline preprocessing and changes the static committed columns.
    /// The caller must commit this representation and use its gathers online;
    /// an existing preprocessing commitment cannot be reused. The statement's
    /// general column-width upper bounds and the online PIOP remain unchanged.
    pub fn new_balanced_padding(matrix: &SparseBinaryMatrix) -> Result<Self, Error> {
        Self::new_impl(matrix, true)
    }

    fn new_impl(matrix: &SparseBinaryMatrix, balanced_padding: bool) -> Result<Self, Error> {
        if !matrix.num_rows.is_power_of_two()
            || !matrix.num_cols.is_power_of_two()
            || matrix.rows.len() != matrix.num_rows
        {
            return Err(Error::Shape);
        }
        let row_log = matrix.num_rows.ilog2() as usize;
        let col_log = matrix.num_cols.ilog2() as usize;
        checked_len(row_log)?;
        checked_len(col_log)?;
        let canonical: Vec<Vec<u64>> = matrix
            .rows
            .par_iter()
            .map_init(
                || vec![0u64; matrix.num_cols.div_ceil(64)],
                |bits, postings| {
                    bits.fill(0);
                    for &col in postings {
                        if col >= matrix.num_cols {
                            return Err(Error::Address);
                        }
                        bits[col / 64] ^= 1u64 << (col % 64);
                    }
                    let count = bits.iter().map(|word| word.count_ones() as usize).sum();
                    let mut columns = Vec::with_capacity(count);
                    for (block, &word) in bits.iter().enumerate() {
                        let mut remaining = word;
                        while remaining != 0 {
                            let bit = remaining.trailing_zeros() as usize;
                            columns.push((block * 64 + bit) as u64);
                            remaining &= remaining - 1;
                        }
                    }
                    Ok(columns)
                },
            )
            .collect::<Result<_, Error>>()?;
        let nonzeros = canonical.iter().try_fold(0usize, |n, row| {
            n.checked_add(row.len()).ok_or(Error::Shape)
        })?;
        if nonzeros > 1usize << MAX_LOG {
            return Err(Error::Shape);
        }
        let entries = nonzeros.max(1).next_power_of_two();
        let mut row = Vec::with_capacity(entries);
        let mut col = Vec::with_capacity(entries);
        for (i, columns) in canonical.into_iter().enumerate() {
            for j in columns {
                row.push(i as u64);
                col.push(j);
            }
        }
        if balanced_padding {
            for i in nonzeros..entries {
                row.push((i % matrix.num_rows) as u64);
                col.push((i % matrix.num_cols) as u64);
            }
        } else {
            row.resize(entries, 0);
            col.resize(entries, 0);
        }
        let mut value = vec![0; entries];
        value[..nonzeros].fill(1);
        // Padding is a real read with value coefficient zero. Whether its
        // addresses are zero or balanced, its timestamps participate in the
        // same complete fixed memory log as the nonzero records.
        let (row_read_time, row_write_time, row_final_time) = timestamps(&row, matrix.num_rows)?;
        let (col_read_time, col_write_time, col_final_time) = timestamps(&col, matrix.num_cols)?;
        Ok(Self {
            row_log,
            col_log,
            entry_log: entries.ilog2() as usize,
            nonzeros,
            row,
            col,
            row_read_time,
            row_write_time,
            col_read_time,
            col_write_time,
            row_final_time,
            col_final_time,
            value,
        })
    }

    pub fn statement(&self, digest: [u8; 32]) -> Statement {
        Statement {
            row_log: self.row_log,
            col_log: self.col_log,
            entry_log: self.entry_log,
            nonzeros: self.nonzeros,
            digest,
        }
    }

    pub fn static_column(&self, source: OracleId) -> Option<&[u64]> {
        Some(match source {
            OracleId::RowAddress => &self.row,
            OracleId::ColAddress => &self.col,
            OracleId::RowReadTime => &self.row_read_time,
            OracleId::RowWriteTime => &self.row_write_time,
            OracleId::ColReadTime => &self.col_read_time,
            OracleId::ColWriteTime => &self.col_write_time,
            OracleId::RowFinalTime => &self.row_final_time,
            OracleId::ColFinalTime => &self.col_final_time,
            OracleId::Value => &self.value,
            OracleId::ReadRow | OracleId::ReadCol => return None,
        })
    }

    /// Challenge-dependent online work. Bind commitments to BOTH returned
    /// vectors before calling `prove`, including any shared batch carrier.
    pub fn gather(&self, x: &[F128], y: &[F128]) -> Result<ReadData, Error> {
        checked_len(self.row_log)?;
        checked_len(self.col_log)?;
        if x.len() != self.row_log || y.len() != self.col_log {
            return Err(Error::Shape);
        }
        let rows = eq_table(x);
        let cols = eq_table(y);
        if self.row.iter().any(|&i| i as usize >= rows.len())
            || self.col.iter().any(|&i| i as usize >= cols.len())
        {
            return Err(Error::Address);
        }
        Ok(ReadData {
            read_row: self.row.par_iter().map(|&i| rows[i as usize]).collect(),
            read_col: self.col.par_iter().map(|&i| cols[i as usize]).collect(),
        })
    }
}

fn timestamps(addresses: &[u64], len: usize) -> Result<(Vec<u64>, Vec<u64>, Vec<u64>), Error> {
    let mut final_time = vec![0u64; len];
    let mut read = Vec::with_capacity(addresses.len());
    let mut write = Vec::with_capacity(addresses.len());
    for &address in addresses {
        let counter = final_time.get_mut(address as usize).ok_or(Error::Address)?;
        read.push(*counter);
        *counter = counter.checked_add(1).ok_or(Error::Shape)?;
        write.push(*counter);
    }
    Ok((read, write, final_time))
}

#[derive(Clone, Debug)]
pub struct ReadData {
    pub read_row: Vec<F128>,
    pub read_col: Vec<F128>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evaluation {
    pub source: OracleId,
    pub point: Vec<F128>,
    pub value: F128,
}

pub type Claim = Evaluation;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SumcheckProof {
    /// [g(0), coefficient of X^2, coefficient of X^3].
    pub rounds: Vec<[F128; 3]>,
    /// Value, ReadRow, ReadCol at the final common point.
    pub terminal: [F128; 3],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryProof {
    pub product: extension_product::Proof,
    /// Left write: address/value/write-time; right read: address/value/read-
    /// time; right final: final-time. All seven values must be PCS-verified.
    pub evaluations: [F128; 7],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    pub matrix: SumcheckProof,
    pub row: MemoryProof,
    pub col: MemoryProof,
}

impl Proof {
    pub fn field_elements(&self) -> usize {
        3 * self.matrix.rounds.len()
            + 3
            + 14
            + self.row.product.field_elements()
            + self.col.product.field_elements()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Shape,
    Address,
    Sumcheck,
    Memory,
    Source,
    Product(extension_product::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sparse matrix lookup: {self:?}")
    }
}
impl std::error::Error for Error {}
impl From<extension_product::Error> for Error {
    fn from(value: extension_product::Error) -> Self {
        Self::Product(value)
    }
}

fn checked_len(log: usize) -> Result<usize, Error> {
    if log > MAX_LOG || log + 1 >= usize::BITS as usize {
        return Err(Error::Shape);
    }
    Ok(1usize << log)
}

fn raw(value: u64) -> F128 {
    F128::new(value, 0)
}

fn eq_table_reference(point: &[F128]) -> Vec<F128> {
    let mut table = vec![F128::ONE];
    for &r in point.iter().rev() {
        table = table
            .into_iter()
            .flat_map(|w| [w * (F128::ONE + r), w * r])
            .collect();
    }
    table
}

fn eq_table(point: &[F128]) -> Vec<F128> {
    if !fusion_enabled() {
        return eq_table_reference(point);
    }
    let mut table = vec![F128::ZERO; 1usize << point.len()];
    table[0] = F128::ONE;
    for (bit, &r) in point.iter().enumerate() {
        let half = 1usize << bit;
        let (lo, hi) = table[..2 * half].split_at_mut(half);
        let expand = |lo: &mut [F128], hi: &mut [F128]| {
            for (a, b) in lo.iter_mut().zip(hi) {
                *b = *a * r;
                *a += *b;
            }
        };
        if half < CHUNK {
            expand(lo, hi);
        } else {
            lo.par_chunks_mut(CHUNK)
                .zip(hi.par_chunks_mut(CHUNK))
                .for_each(|(lo, hi)| expand(lo, hi));
        }
    }
    table
}

fn dot_three_chunk(
    address: &[u64],
    values: &[F128],
    time: &[u64],
    weights: &[F128],
) -> [F256Unreduced; 3] {
    let mut result = [F256Unreduced::ZERO; 3];
    #[allow(unused_mut)]
    let mut start = 0;
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq",
        target_feature = "sse4.1"
    ))]
    {
        use flock_field::gf2_128::x86_64::{WideGhashX4, f128x4_loadu, f128x4_set};
        // SAFETY: target features are compile-time gated and each four-lane
        // load lies in the equal-length input chunks checked by the caller.
        unsafe {
            let mut accumulators = [WideGhashX4::zero(); 3];
            while start + 4 <= weights.len() {
                let w = f128x4_loadu(weights.as_ptr().add(start));
                let a = f128x4_set(
                    raw(address[start]),
                    raw(address[start + 1]),
                    raw(address[start + 2]),
                    raw(address[start + 3]),
                );
                let t = f128x4_set(
                    raw(time[start]),
                    raw(time[start + 1]),
                    raw(time[start + 2]),
                    raw(time[start + 3]),
                );
                accumulators[0].mul_acc(a, w);
                accumulators[1].mul_acc(f128x4_loadu(values.as_ptr().add(start)), w);
                accumulators[2].mul_acc(t, w);
                start += 4;
            }
            result = accumulators.map(|acc| acc.fold());
        }
    }
    for i in start..weights.len() {
        result[0] ^= raw(address[i]).mul_unreduced(weights[i]);
        result[1] ^= values[i].mul_unreduced(weights[i]);
        result[2] ^= raw(time[i]).mul_unreduced(weights[i]);
    }
    result
}

/// All three claims share a point. One equality-table allocation and weight
/// load serves address, gathered value and timestamp; reduction is deferred.
fn eval_three(address: &[u64], values: &[F128], time: &[u64], point: &[F128]) -> [F128; 3] {
    let weights = eq_table(point);
    assert_eq!(address.len(), weights.len());
    assert_eq!(values.len(), weights.len());
    assert_eq!(time.len(), weights.len());
    if weights.len() < CHUNK {
        return dot_three_chunk(address, values, time, &weights).map(F256Unreduced::reduce);
    }
    address
        .par_chunks(CHUNK)
        .zip(values.par_chunks(CHUNK))
        .zip(time.par_chunks(CHUNK))
        .zip(weights.par_chunks(CHUNK))
        .map(|(((a, v), t), w)| dot_three_chunk(a, v, t, w))
        .reduce(
            || [F256Unreduced::ZERO; 3],
            |a, b| std::array::from_fn(|i| a[i] ^ b[i]),
        )
        .map(F256Unreduced::reduce)
}

fn eval_raw_fast(values: &[u64], point: &[F128]) -> F128 {
    let weights = eq_table(point);
    assert_eq!(values.len(), weights.len());
    let dot = |values: &[u64], weights: &[F128]| {
        values
            .iter()
            .zip(weights)
            .fold(F256Unreduced::ZERO, |sum, (&v, &w)| {
                sum ^ raw(v).mul_unreduced(w)
            })
    };
    if values.len() < CHUNK {
        return dot(values, &weights).reduce();
    }
    values
        .par_chunks(CHUNK)
        .zip(weights.par_chunks(CHUNK))
        .map(|(v, w)| dot(v, w))
        .reduce(|| F256Unreduced::ZERO, |a, b| a ^ b)
        .reduce()
}

fn eval(values: &[F128], point: &[F128]) -> F128 {
    values
        .par_iter()
        .zip(eq_table(point).into_par_iter())
        .map(|(&value, weight)| value * weight)
        .reduce(|| F128::ZERO, |a, b| a + b)
}

fn eval_raw(values: &[u64], point: &[F128]) -> F128 {
    values
        .par_iter()
        .zip(eq_table(point).into_par_iter())
        .map(|(&value, weight)| raw(value) * weight)
        .reduce(|| F128::ZERO, |a, b| a + b)
}

fn bind<C: Challenger>(
    statement: &Statement,
    x: &[F128],
    y: &[F128],
    value: F128,
    ch: &mut C,
) -> Result<(), Error> {
    statement.validate()?;
    if x.len() != statement.row_log || y.len() != statement.col_log {
        return Err(Error::Shape);
    }
    ch.observe_label(b"mikros-binary-sparse-lookup-v1");
    for n in [
        statement.row_log,
        statement.col_log,
        statement.entry_log,
        statement.nonzeros,
    ] {
        ch.observe_bytes(&(n as u64).to_le_bytes());
    }
    ch.observe_bytes(&statement.digest);
    ch.observe_f128_slice(x);
    ch.observe_f128_slice(y);
    ch.observe_f128(value);
    Ok(())
}

fn message_reference(a: &[F128], b: &[F128], c: &[F128]) -> [F128; 3] {
    a.par_chunks_exact(2)
        .zip(b.par_chunks_exact(2))
        .zip(c.par_chunks_exact(2))
        .map(|((a, b), c)| {
            let da = a[0] + a[1];
            let db = b[0] + b[1];
            let dc = c[0] + c[1];
            [
                a[0] * b[0] * c[0],
                da * db * c[0] + da * b[0] * dc + a[0] * db * dc,
                da * db * dc,
            ]
        })
        .reduce(
            || [F128::ZERO; 3],
            |a, b| std::array::from_fn(|i| a[i] + b[i]),
        )
}

fn evaluate_message(g: [F128; 3], claim: F128, r: F128) -> F128 {
    g[0] + r * (claim + g[1] + g[2] + r * (g[1] + r * g[2]))
}

fn fold_reference(values: &[F128], r: F128) -> Vec<F128> {
    values
        .par_chunks_exact(2)
        .map(|v| v[0] + r * (v[0] + v[1]))
        .collect()
}

fn matrix_claims(point: &[F128], terminal: [F128; 3]) -> Vec<Evaluation> {
    [OracleId::Value, OracleId::ReadRow, OracleId::ReadCol]
        .into_iter()
        .zip(terminal)
        .map(|(source, value)| Evaluation {
            source,
            point: point.to_vec(),
            value,
        })
        .collect()
}

fn prove_matrix_reference<C: Challenger>(
    prepared: &Prepared,
    reads: &ReadData,
    mut claim: F128,
    ch: &mut C,
) -> Result<(SumcheckProof, Vec<Evaluation>), Error> {
    ch.observe_label(b"mikros-sparse-matrix-cubic-v1");
    let mut a: Vec<_> = prepared.value.iter().map(|&v| raw(v)).collect();
    let mut b = reads.read_row.clone();
    let mut c = reads.read_col.clone();
    let initial = a
        .par_iter()
        .zip(&b)
        .zip(&c)
        .map(|((&a, &b), &c)| a * b * c)
        .reduce(|| F128::ZERO, |a, b| a + b);
    if initial != claim {
        return Err(Error::Sumcheck);
    }
    let mut rounds = Vec::with_capacity(prepared.entry_log);
    let mut point = Vec::with_capacity(prepared.entry_log);
    while a.len() > 1 {
        let g = message_reference(&a, &b, &c);
        ch.observe_f128_slice(&g);
        let r = ch.sample_f128();
        claim = evaluate_message(g, claim, r);
        rounds.push(g);
        point.push(r);
        a = fold_reference(&a, r);
        b = fold_reference(&b, r);
        c = fold_reference(&c, r);
    }
    let terminal = [a[0], b[0], c[0]];
    if terminal[0] * terminal[1] * terminal[2] != claim {
        return Err(Error::Sumcheck);
    }
    ch.observe_f128_slice(&terminal);
    Ok((
        SumcheckProof { rounds, terminal },
        matrix_claims(&point, terminal),
    ))
}

/// Share the quadratic b*c coefficients across all cubic coefficients.
/// Outer products are XOR-accumulated before their one final reduction.
fn cubic_chunk(a: &[F128], b: &[F128], c: &[F128], initial: bool) -> [F256Unreduced; 4] {
    let mut out = [F256Unreduced::ZERO; 4];
    for ((a, b), c) in a
        .chunks_exact(2)
        .zip(b.chunks_exact(2))
        .zip(c.chunks_exact(2))
    {
        // The fixed Value column is a prefix of ones followed by zeros.
        // Folding preserves that shape except for one boundary element, so
        // almost all pairs take these exact constant-coefficient paths.
        // This uses actual folded values, not an assumption about the input.
        if a[0] == a[1] {
            if a[0].is_zero() {
                continue;
            }
            let db = b[0] + b[1];
            let dc = c[0] + c[1];
            if a[0] == F128::ONE {
                let g0 = b[0].mul_unreduced(c[0]);
                out[0] ^= g0;
                out[1] ^= db.mul_unreduced(dc);
                if initial {
                    out[3] ^= g0 ^ b[1].mul_unreduced(c[1]);
                }
            } else {
                let g0 = a[0].mul_unreduced(b[0] * c[0]);
                out[0] ^= g0;
                out[1] ^= a[0].mul_unreduced(db * dc);
                if initial {
                    out[3] ^= g0 ^ a[1].mul_unreduced(b[1] * c[1]);
                }
            }
            continue;
        }
        let da = a[0] + a[1];
        let db = b[0] + b[1];
        let dc = c[0] + c[1];
        let q0 = b[0] * c[0];
        let q2 = db * dc;
        let q1 = db * c[0] + b[0] * dc;
        let g0 = a[0].mul_unreduced(q0);
        out[0] ^= g0;
        out[1] ^= da.mul_unreduced(q1) ^ a[0].mul_unreduced(q2);
        out[2] ^= da.mul_unreduced(q2);
        if initial {
            // b1*c1=q0+q1+q2, so the input-consistency check needs no
            // additional inner field multiplication or full witness scan.
            out[3] ^= g0 ^ a[1].mul_unreduced(q0 + q1 + q2);
        }
    }
    out
}

fn sum_cubic(a: [F256Unreduced; 4], b: [F256Unreduced; 4]) -> [F256Unreduced; 4] {
    std::array::from_fn(|i| a[i] ^ b[i])
}

fn initial_cubic(a: &[F128], b: &[F128], c: &[F128]) -> [F128; 4] {
    if a.len() < CHUNK {
        return cubic_chunk(a, b, c, true).map(F256Unreduced::reduce);
    }
    a.par_chunks(CHUNK)
        .zip(b.par_chunks(CHUNK))
        .zip(c.par_chunks(CHUNK))
        .map(|((a, b), c)| cubic_chunk(a, b, c, true))
        .reduce(|| [F256Unreduced::ZERO; 4], sum_cubic)
        .map(F256Unreduced::reduce)
}

/// One parallel traversal folds all three sources using the existing SIMD
/// kernels and consumes each output chunk immediately for the next message.
fn fold_three_and_message(
    a: &[F128],
    b: &[F128],
    c: &[F128],
    next_a: &mut [F128],
    next_b: &mut [F128],
    next_c: &mut [F128],
    r: F128,
) -> [F128; 3] {
    let run = |base: usize, na: &mut [F128], nb: &mut [F128], nc: &mut [F128]| {
        fold_pairs(a, base, na, r);
        fold_pairs(b, base, nb, r);
        fold_pairs(c, base, nc, r);
        cubic_chunk(na, nb, nc, false)
    };
    let totals = if next_a.len() < CHUNK {
        run(0, next_a, next_b, next_c)
    } else {
        next_a
            .par_chunks_mut(CHUNK)
            .zip(next_b.par_chunks_mut(CHUNK))
            .zip(next_c.par_chunks_mut(CHUNK))
            .enumerate()
            .map(|(block, ((na, nb), nc))| run(block * CHUNK, na, nb, nc))
            .reduce(|| [F256Unreduced::ZERO; 4], sum_cubic)
    };
    std::array::from_fn(|i| totals[i].reduce())
}

fn prove_matrix<C: Challenger>(
    prepared: &Prepared,
    reads: &ReadData,
    mut claim: F128,
    ch: &mut C,
) -> Result<(SumcheckProof, Vec<Evaluation>), Error> {
    if !fusion_enabled() {
        return prove_matrix_reference(prepared, reads, claim, ch);
    }
    ch.observe_label(b"mikros-sparse-matrix-cubic-v1");
    let mut a: Vec<_> = prepared.value.iter().map(|&v| raw(v)).collect();
    let mut b = reads.read_row.clone();
    let mut c = reads.read_col.clone();
    let mut g = if a.len() == 1 {
        if a[0] * b[0] * c[0] != claim {
            return Err(Error::Sumcheck);
        }
        [F128::ZERO; 3]
    } else {
        let initial = initial_cubic(&a, &b, &c);
        if initial[3] != claim {
            return Err(Error::Sumcheck);
        }
        [initial[0], initial[1], initial[2]]
    };
    let mut next_a = vec![F128::ZERO; a.len() / 2];
    let mut next_b = vec![F128::ZERO; a.len() / 2];
    let mut next_c = vec![F128::ZERO; a.len() / 2];
    let mut rounds = Vec::with_capacity(prepared.entry_log);
    let mut point = Vec::with_capacity(prepared.entry_log);
    while a.len() > 1 {
        ch.observe_f128_slice(&g);
        let r = ch.sample_f128();
        claim = evaluate_message(g, claim, r);
        rounds.push(g);
        point.push(r);
        let half = a.len() / 2;
        next_a.truncate(half);
        next_b.truncate(half);
        next_c.truncate(half);
        g = fold_three_and_message(&a, &b, &c, &mut next_a, &mut next_b, &mut next_c, r);
        std::mem::swap(&mut a, &mut next_a);
        std::mem::swap(&mut b, &mut next_b);
        std::mem::swap(&mut c, &mut next_c);
    }
    let terminal = [a[0], b[0], c[0]];
    if terminal[0] * terminal[1] * terminal[2] != claim {
        return Err(Error::Sumcheck);
    }
    ch.observe_f128_slice(&terminal);
    Ok((
        SumcheckProof { rounds, terminal },
        matrix_claims(&point, terminal),
    ))
}

fn verify_matrix<C: Challenger>(
    statement: &Statement,
    proof: &SumcheckProof,
    mut claim: F128,
    ch: &mut C,
) -> Result<Vec<Evaluation>, Error> {
    if proof.rounds.len() != statement.entry_log {
        return Err(Error::Shape);
    }
    ch.observe_label(b"mikros-sparse-matrix-cubic-v1");
    let mut point = Vec::with_capacity(statement.entry_log);
    for &g in &proof.rounds {
        ch.observe_f128_slice(&g);
        let r = ch.sample_f128();
        point.push(r);
        claim = evaluate_message(g, claim, r);
    }
    if proof.terminal[0] * proof.terminal[1] * proof.terminal[2] != claim {
        return Err(Error::Sumcheck);
    }
    ch.observe_f128_slice(&proof.terminal);
    Ok(matrix_claims(&point, proof.terminal))
}

#[derive(Clone, Copy)]
enum Axis {
    Row,
    Col,
}

impl Axis {
    fn sources(self) -> [OracleId; 4] {
        match self {
            Self::Row => [
                OracleId::RowAddress,
                OracleId::ReadRow,
                OracleId::RowReadTime,
                OracleId::RowWriteTime,
            ],
            Self::Col => [
                OracleId::ColAddress,
                OracleId::ReadCol,
                OracleId::ColReadTime,
                OracleId::ColWriteTime,
            ],
        }
    }
    fn final_source(self) -> OracleId {
        match self {
            Self::Row => OracleId::RowFinalTime,
            Self::Col => OracleId::ColFinalTime,
        }
    }
    fn table_log(self, s: &Statement) -> usize {
        match self {
            Self::Row => s.row_log,
            Self::Col => s.col_log,
        }
    }
    fn values<'a>(self, data: &'a ReadData) -> &'a [F128] {
        match self {
            Self::Row => &data.read_row,
            Self::Col => &data.read_col,
        }
    }
}

#[derive(Clone, Copy)]
struct Fingerprint {
    shift: F128,
    address: F128,
    value: F128,
    time: F128,
}

impl Fingerprint {
    fn sample<C: Challenger>(axis: Axis, ch: &mut C) -> Self {
        ch.observe_label(b"mikros-sparse-indexed-memory-v1");
        ch.observe_bytes(&[match axis {
            Axis::Row => 0,
            Axis::Col => 1,
        }]);
        Self {
            shift: ch.sample_f128(),
            address: ch.sample_f128(),
            value: ch.sample_f128(),
            time: ch.sample_f128(),
        }
    }
    fn entry(self, address: u64, value: F128, time: u64) -> F128 {
        self.shift + self.address * raw(address) + self.value * value + self.time * raw(time)
    }
}

/// Per-axis, per-challenge online scratch. Only actual read/table leaves are
/// stored, not the possibly much larger ONE-padded product-tree domains.
/// The entries are [read, write] and [init, final], sharing their affine base.
/// This scratch must never survive the fingerprint challenge that created it.
struct FingerprintCache {
    reads: Vec<[F128; 2]>,
    table: Vec<[F128; 2]>,
}

impl FingerprintCache {
    fn new(
        f: Fingerprint,
        addresses: &[u64],
        values: &[F128],
        read_times: &[u64],
        write_times: &[u64],
        table: &[F128],
        final_times: &[u64],
    ) -> Self {
        assert_eq!(addresses.len(), values.len());
        assert_eq!(read_times.len(), values.len());
        assert_eq!(write_times.len(), values.len());
        assert_eq!(final_times.len(), table.len());
        let mut cached = Self {
            reads: vec![[F128::ZERO; 2]; values.len()],
            table: vec![[F128::ZERO; 2]; table.len()],
        };
        // This table depends on the current fingerprint challenge. Its init
        // entries also serve as checked bases for repeated addressed reads.
        cached
            .table
            .par_chunks_mut(CHUNK)
            .enumerate()
            .for_each(|(chunk, out)| {
                for (offset, pair) in out.iter_mut().enumerate() {
                    let i = chunk * CHUNK + offset;
                    let base = f.shift + f.address * raw(i as u64) + f.value * table[i];
                    *pair = [base, base + f.time * raw(final_times[i])];
                }
            });
        let reuse_base = cache_base_reuse_enabled();
        let cached_table = &cached.table;
        cached
            .reads
            .par_chunks_mut(CHUNK)
            .enumerate()
            .for_each(|(chunk, out)| {
                for (offset, pair) in out.iter_mut().enumerate() {
                    let i = chunk * CHUNK + offset;
                    // Never replace a supplied read by the honest gather.
                    // Reuse only after comparing the actual value. Incorrect
                    // reads and out-of-range addresses retain the old formula
                    // and therefore the same product proof/error transcript.
                    let cached_base = if reuse_base {
                        usize::try_from(addresses[i])
                            .ok()
                            .filter(|&address| table.get(address) == Some(&values[i]))
                            .map(|address| cached_table[address][0])
                    } else {
                        None
                    };
                    let base = cached_base.unwrap_or_else(|| {
                        f.shift + f.address * raw(addresses[i]) + f.value * values[i]
                    });
                    *pair = [
                        base + f.time * raw(read_times[i]),
                        base + f.time * raw(write_times[i]),
                    ];
                }
            });
        cached
    }

    #[inline]
    fn leaf(&self, side: usize, i: usize, d: usize) -> F128 {
        let half = i / d;
        let i = i % d;
        let (entries, coordinate) = match (side, half) {
            (0, 0) => (&self.table, 0),
            (0, 1) => (&self.reads, 1),
            (1, 0) => (&self.reads, 0),
            (1, 1) => (&self.table, 1),
            _ => return F128::ONE,
        };
        entries.get(i).map_or(F128::ONE, |pair| pair[coordinate])
    }
}

fn memory_queries(
    statement: &Statement,
    axis: Axis,
    leaves: &[extension_product::LeafClaim; 2],
    values: [F128; 7],
) -> Vec<Evaluation> {
    let [address, read, read_time, write_time] = axis.sources();
    let point_l = &leaves[0].point[..statement.entry_log];
    let point_r = &leaves[1].point[..statement.entry_log];
    let point_final = &leaves[1].point[..axis.table_log(statement)];
    [
        (address, point_l),
        (read, point_l),
        (write_time, point_l),
        (address, point_r),
        (read, point_r),
        (read_time, point_r),
        (axis.final_source(), point_final),
    ]
    .into_iter()
    .zip(values)
    .map(|((source, point), value)| Evaluation {
        source,
        point: point.to_vec(),
        value,
    })
    .collect()
}

fn prefix_factor(point: &[F128], log: usize) -> F128 {
    point[log..]
        .iter()
        .fold(F128::ONE, |v, &r| v * (F128::ONE + r))
}

fn identity_eval(point: &[F128]) -> F128 {
    point
        .iter()
        .enumerate()
        .fold(F128::ZERO, |v, (i, &r)| v + raw(1u64 << i) * r)
}

fn eq_eval(a: &[F128], b: &[F128]) -> F128 {
    a.iter()
        .zip(b)
        .fold(F128::ONE, |v, (&a, &b)| v * (F128::ONE + a + b))
}

fn check_memory(
    statement: &Statement,
    axis: Axis,
    table_point: &[F128],
    fingerprint: Fingerprint,
    leaves: &[extension_product::LeafClaim; 2],
    values: [F128; 7],
) -> Result<(), Error> {
    let table_log = axis.table_log(statement);
    let log_d = table_log.max(statement.entry_log);
    if leaves.iter().any(|leaf| leaf.point.len() != log_d + 1) {
        return Err(Error::Shape);
    }
    let f = fingerprint;
    for (side, leaf) in leaves.iter().enumerate() {
        let point = &leaf.point[..log_d];
        let selector = leaf.point[log_d];
        let table_active = prefix_factor(point, table_log);
        let read_active = prefix_factor(point, statement.entry_log);
        let table_base = f.shift
            + f.address * identity_eval(&point[..table_log])
            + f.value * eq_eval(table_point, &point[..table_log]);
        let (first, second) = if side == 0 {
            let init = F128::ONE + table_active * (F128::ONE + table_base);
            let write = F128::ONE
                + read_active
                    * (F128::ONE
                        + f.shift
                        + f.address * values[0]
                        + f.value * values[1]
                        + f.time * values[2]);
            (init, write)
        } else {
            let read = F128::ONE
                + read_active
                    * (F128::ONE
                        + f.shift
                        + f.address * values[3]
                        + f.value * values[4]
                        + f.time * values[5]);
            let final_state =
                F128::ONE + table_active * (F128::ONE + table_base + f.time * values[6]);
            (read, final_state)
        };
        if first + selector * (first + second) != leaf.value {
            return Err(Error::Memory);
        }
    }
    Ok(())
}

fn bind_memory_values<C: Challenger>(values: &[F128; 7], ch: &mut C) {
    ch.observe_label(b"mikros-sparse-memory-source-evaluations-v1");
    ch.observe_f128_slice(values);
}

fn prove_memory<C: Challenger>(
    prepared: &Prepared,
    statement: &Statement,
    reads: &ReadData,
    axis: Axis,
    table_point: &[F128],
    profiling: bool,
    ch: &mut C,
) -> Result<(MemoryProof, Vec<Evaluation>), Error> {
    let f = Fingerprint::sample(axis, ch);
    let [address, _, read_time, write_time] = axis.sources();
    let addresses = prepared.static_column(address).ok_or(Error::Source)?;
    let read_times = prepared.static_column(read_time).ok_or(Error::Source)?;
    let write_times = prepared.static_column(write_time).ok_or(Error::Source)?;
    let final_times = prepared
        .static_column(axis.final_source())
        .ok_or(Error::Source)?;
    let values = axis.values(reads);
    let table = eq_table(table_point);
    let log_d = axis.table_log(statement).max(statement.entry_log);
    let d = 1usize << log_d;
    let cache_enabled = cache_leaves_enabled();
    let cache_start = (profiling && cache_enabled).then(std::time::Instant::now);
    let cache = cache_enabled.then(|| {
        FingerprintCache::new(
            f,
            addresses,
            values,
            read_times,
            write_times,
            &table,
            final_times,
        )
    });
    let cache_build_ms = cache_start.map_or(0.0, |start| start.elapsed().as_secs_f64() * 1000.0);
    // Distinct closure types keep cache selection out of every hot leaf
    // callback. In particular, the uncached tree retains the pure formula
    // monomorphization from before fingerprint caching was introduced.
    let (product, leaves, tree_timings) = if let Some(cache) = &cache {
        extension_product::prove_pair(log_d + 1, |side, i| cache.leaf(side, i, d), ch)
    } else {
        extension_product::prove_pair(
            log_d + 1,
            |side, i| {
                let half = i / d;
                let i = i % d;
                match (side, half) {
                    (0, 0) if i < table.len() => f.entry(i as u64, table[i], 0),
                    (0, 1) if i < values.len() => f.entry(addresses[i], values[i], write_times[i]),
                    (1, 0) if i < values.len() => f.entry(addresses[i], values[i], read_times[i]),
                    (1, 1) if i < table.len() => f.entry(i as u64, table[i], final_times[i]),
                    _ => F128::ONE,
                }
            },
            ch,
        )
    }?;
    // Charge release to the online proof as well, and limit scratch lifetime
    // to one axis rather than retaining both axes through endpoint evaluation.
    let cache_release_start = (profiling && cache_enabled).then(std::time::Instant::now);
    drop(cache);
    let cache_release_ms =
        cache_release_start.map_or(0.0, |start| start.elapsed().as_secs_f64() * 1000.0);
    let endpoint_start = profiling.then(std::time::Instant::now);
    let evaluations = if fusion_enabled() {
        let left = eval_three(
            addresses,
            values,
            write_times,
            &leaves[0].point[..statement.entry_log],
        );
        let right = eval_three(
            addresses,
            values,
            read_times,
            &leaves[1].point[..statement.entry_log],
        );
        let final_value = eval_raw_fast(final_times, &leaves[1].point[..axis.table_log(statement)]);
        [
            left[0],
            left[1],
            left[2],
            right[0],
            right[1],
            right[2],
            final_value,
        ]
    } else {
        let templates = memory_queries(statement, axis, &leaves, [F128::ZERO; 7]);
        std::array::from_fn(|i| {
            let q = &templates[i];
            if let Some(source) = prepared.static_column(q.source) {
                eval_raw(source, &q.point)
            } else {
                eval(values, &q.point)
            }
        })
    };
    if let Some(start) = endpoint_start {
        eprintln!(
            "sparse_lookup_profile axis={} entries={} cached={} base_reuse={} cache_build_ms={:.3} cache_release_ms={:.3} tree_construction_ms={:.3} tree_root_ms={:.3} tree_reduction_ms={:.3} endpoint_eval_ms={:.3}",
            match axis {
                Axis::Row => "row",
                Axis::Col => "col",
            },
            values.len(),
            cache_enabled,
            cache_enabled && cache_base_reuse_enabled(),
            cache_build_ms,
            cache_release_ms,
            tree_timings.construction_ms,
            tree_timings.root_ms,
            tree_timings.reduction_ms,
            start.elapsed().as_secs_f64() * 1000.0,
        );
    }
    check_memory(statement, axis, table_point, f, &leaves, evaluations)?;
    bind_memory_values(&evaluations, ch);
    Ok((
        MemoryProof {
            product,
            evaluations,
        },
        memory_queries(statement, axis, &leaves, evaluations),
    ))
}

fn verify_memory<C: Challenger>(
    statement: &Statement,
    axis: Axis,
    table_point: &[F128],
    proof: &MemoryProof,
    ch: &mut C,
) -> Result<Vec<Evaluation>, Error> {
    let f = Fingerprint::sample(axis, ch);
    let log_d = axis.table_log(statement).max(statement.entry_log);
    let leaves = extension_product::verify_pair(log_d + 1, &proof.product, ch)?;
    check_memory(statement, axis, table_point, f, &leaves, proof.evaluations)?;
    bind_memory_values(&proof.evaluations, ch);
    Ok(memory_queries(statement, axis, &leaves, proof.evaluations))
}

/// PIOP only. All static sources and both online gathers must already be
/// committed and bound. Authentication of all 17 returned claims is required.
pub fn prove<C: Challenger>(
    prepared: &Prepared,
    statement: &Statement,
    x: &[F128],
    y: &[F128],
    matrix_value: F128,
    reads: &ReadData,
    ch: &mut C,
) -> Result<(Proof, Vec<Evaluation>), Error> {
    let profiling = std::env::var_os("MIKROS_SPARSE_PROFILE").is_some_and(|v| v == "1");
    let total_start = profiling.then(std::time::Instant::now);
    bind(statement, x, y, matrix_value, ch)?;
    if prepared.statement(statement.digest) != *statement
        || OracleId::STATIC.iter().any(|&source| {
            prepared
                .static_column(source)
                .is_none_or(|values| values.len() != 1usize << statement.column_log(source))
        })
        || reads.read_row.len() != 1usize << statement.entry_log
        || reads.read_col.len() != 1usize << statement.entry_log
    {
        return Err(Error::Shape);
    }
    let cubic_start = profiling.then(std::time::Instant::now);
    let (matrix, mut claims) = prove_matrix(prepared, reads, matrix_value, ch)?;
    let cubic_ms = cubic_start.map(|start| start.elapsed().as_secs_f64() * 1000.0);
    let (row, row_claims) = prove_memory(prepared, statement, reads, Axis::Row, x, profiling, ch)?;
    claims.extend(row_claims);
    let (col, col_claims) = prove_memory(prepared, statement, reads, Axis::Col, y, profiling, ch)?;
    claims.extend(col_claims);
    if let Some(start) = total_start {
        eprintln!(
            "sparse_lookup_profile entries={} fused={} cubic_ms={:.3} total_ms={:.3}",
            prepared.value.len(),
            fusion_enabled(),
            cubic_ms.unwrap(),
            start.elapsed().as_secs_f64() * 1000.0
        );
    }
    Ok((Proof { matrix, row, col }, claims))
}

/// Succinct PIOP verifier: no matrix, address arrays, counters, gathered values
/// or equality tables are inputs. Caller verifies the returned PCS claims.
pub fn verify<C: Challenger>(
    statement: &Statement,
    x: &[F128],
    y: &[F128],
    matrix_value: F128,
    proof: &Proof,
    ch: &mut C,
) -> Result<Vec<Evaluation>, Error> {
    bind(statement, x, y, matrix_value, ch)?;
    let mut claims = verify_matrix(statement, &proof.matrix, matrix_value, ch)?;
    claims.extend(verify_memory(statement, Axis::Row, x, &proof.row, ch)?);
    claims.extend(verify_memory(statement, Axis::Col, y, &proof.col, ch)?);
    Ok(claims)
}

/// Linear-time source oracle for tests. This is NOT a PCS verifier.
pub fn check_claims(
    prepared: &Prepared,
    reads: &ReadData,
    claims: &[Evaluation],
) -> Result<(), Error> {
    for claim in claims {
        let expected = checked_len(claim.point.len())?;
        let (len, actual) = if let Some(values) = prepared.static_column(claim.source) {
            if values.len() != expected {
                return Err(Error::Source);
            }
            (values.len(), eval_raw(values, &claim.point))
        } else {
            let values = match claim.source {
                OracleId::ReadRow => &reads.read_row,
                OracleId::ReadCol => &reads.read_col,
                _ => return Err(Error::Source),
            };
            if values.len() != expected {
                return Err(Error::Source);
            }
            (values.len(), eval(values, &claim.point))
        };
        if len != expected || actual != claim.value {
            return Err(Error::Source);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_transcript::challenger::FsChallenger;

    fn ch() -> FsChallenger {
        FsChallenger::new(b"sparse-lookup-test")
    }
    fn query(log: usize, seed: u64) -> Vec<F128> {
        (0..log)
            .map(|i| F128::new(seed + 13 * i as u64, seed + 29 * i as u64))
            .collect()
    }
    fn value(p: &Prepared, reads: &ReadData) -> F128 {
        p.value
            .iter()
            .zip(&reads.read_row)
            .zip(&reads.read_col)
            .fold(F128::ZERO, |v, ((&a, &b), &c)| v + raw(a) * b * c)
    }

    #[test]
    fn fused_kernels_match_reference_at_chunk_boundaries_and_degenerate_points() {
        use flock_field::test_rng::Rng;
        let mut rng = Rng::new(0x5A_25_E0);
        for log in [0, 1, 3, 11, 12, 13] {
            let point: Vec<_> = (0..log).map(|_| rng.f128()).collect();
            for point in [&point[..], &vec![F128::ZERO; log], &vec![F128::ONE; log]] {
                assert_eq!(
                    with_fusion(true, || eq_table(point)),
                    eq_table_reference(point)
                );
            }
            let mut a: Vec<_> = (0..1usize << log).map(|_| rng.f128()).collect();
            for (i, pair) in a.chunks_exact_mut(2).enumerate() {
                match i % 4 {
                    0 => pair[1] = pair[0],
                    1 => pair.fill(F128::ONE),
                    2 => pair.fill(F128::ZERO),
                    _ => {}
                }
            }
            let b: Vec<_> = (0..a.len()).map(|_| rng.f128()).collect();
            let c: Vec<_> = (0..a.len()).map(|_| rng.f128()).collect();
            let address: Vec<_> = a.iter().map(|x| x.lo).collect();
            let time: Vec<_> = c.iter().map(|x| x.hi).collect();
            let dots = with_fusion(true, || eval_three(&address, &b, &time, &point));
            let expected = with_fusion(false, || {
                [
                    eval_raw(&address, &point),
                    eval(&b, &point),
                    eval_raw(&time, &point),
                ]
            });
            assert_eq!(dots, expected);
            if a.len() == 1 {
                continue;
            }
            let got = initial_cubic(&a, &b, &c);
            assert_eq!(&got[..3], &message_reference(&a, &b, &c));
            assert_eq!(
                got[3],
                a.iter()
                    .zip(&b)
                    .zip(&c)
                    .fold(F128::ZERO, |v, ((&a, &b), &c)| v + a * b * c)
            );
            for r in [F128::ZERO, F128::ONE, rng.f128()] {
                let mut na = vec![F128::ZERO; a.len() / 2];
                let mut nb = na.clone();
                let mut nc = na.clone();
                let next = fold_three_and_message(&a, &b, &c, &mut na, &mut nb, &mut nc, r);
                assert_eq!(na, fold_reference(&a, r));
                assert_eq!(nb, fold_reference(&b, r));
                assert_eq!(nc, fold_reference(&c, r));
                assert_eq!(next, message_reference(&na, &nb, &nc));
            }
        }
    }

    #[test]
    fn fused_lookup_proof_bytes_and_transcript_match_reference() {
        for matrix in [
            SparseBinaryMatrix::new(1, 1, vec![vec![0]]),
            SparseBinaryMatrix::new(2, 4, vec![vec![], vec![]]),
            SparseBinaryMatrix::new(
                256,
                1024,
                (0..256)
                    .map(|i| {
                        if i % 7 == 0 {
                            vec![]
                        } else {
                            (0..31).map(|j| (i * 29 + j * 11) % 1024).collect()
                        }
                    })
                    .collect(),
            ),
        ] {
            let p = Prepared::new(&matrix).unwrap();
            let s = p.statement([23; 32]);
            let x = query(s.row_log, 331);
            let y = query(s.col_log, 7129);
            let reads = p.gather(&x, &y).unwrap();
            let v = value(&p, &reads);
            let run = |fused, cached, reuse| {
                with_cache_leaves(cached, || {
                    with_cache_base_reuse(reuse, || {
                        with_fusion(fused, || {
                            let mut challenger = ch();
                            let (proof, claims) =
                                prove(&p, &s, &x, &y, v, &reads, &mut challenger).unwrap();
                            (proof, claims, challenger.sample_f128())
                        })
                    })
                })
            };
            let old = run(false, false, false);
            for (fused, cached, reuse) in [
                (true, false, false),
                (false, true, false),
                (true, true, false),
                (false, true, true),
                (true, true, true),
            ] {
                let new = run(fused, cached, reuse);
                assert_eq!(
                    bincode::serialize(&old.0).unwrap(),
                    bincode::serialize(&new.0).unwrap()
                );
                assert_eq!(old.1, new.1);
                assert_eq!(old.2, new.2);
                assert_eq!(new.1.len(), 17);
                let mut vc = ch();
                assert_eq!(verify(&s, &x, &y, v, &new.0, &mut vc).unwrap(), new.1);
                assert_eq!(vc.sample_f128(), new.2);
                with_fusion(false, || check_claims(&p, &reads, &new.1)).unwrap();
            }
        }
    }

    #[test]
    fn cache_base_reuse_preserves_bad_read_and_bad_address_transcripts() {
        let matrix =
            SparseBinaryMatrix::new(4, 8, vec![vec![7, 1, 3], vec![1], vec![0, 7], vec![3]]);
        let original = Prepared::new(&matrix).unwrap();
        let s = original.statement([41; 32]);
        let x = query(s.row_log, 319);
        let y = query(s.col_log, 971);
        let original_reads = original.gather(&x, &y).unwrap();
        for mutation in 0..6 {
            let mut p = original.clone();
            let mut reads = original_reads.clone();
            match mutation {
                0 => reads.read_row[0] += F128::ONE,
                1 => reads.read_col[0] += F128::ONE,
                2 => p.row[0] = 1u64 << s.row_log,
                3 => p.col[0] = 1u64 << s.col_log,
                4 => p.row[0] = u64::MAX,
                5 => p.col[0] = u64::MAX,
                _ => unreachable!(),
            }
            // Let the cubic claim follow the supplied reads, so a rejected
            // read reaches memory checking rather than the initial sum check.
            let v = value(&p, &reads);
            let run = |cached, reuse| {
                with_cache_leaves(cached, || {
                    with_cache_base_reuse(reuse, || {
                        let mut challenger = ch();
                        let result = prove(&p, &s, &x, &y, v, &reads, &mut challenger);
                        (result, challenger.sample_f128())
                    })
                })
            };
            let reference = run(false, false);
            assert!(reference.0.is_err());
            assert_eq!(run(true, false), reference);
            assert_eq!(run(true, true), reference);
        }
    }

    #[test]
    fn fingerprint_cache_matches_each_challenge_and_one_padding() {
        use flock_field::test_rng::Rng;
        let mut rng = Rng::new(0xCAC4E);
        for (read_len, table_len) in [(1, 1), (1, 8), (16, 2), (8192, 4096)] {
            let addresses: Vec<_> = (0..read_len).map(|_| rng.f128().lo).collect();
            // Exercise integer carries and all raw bits without interpreting
            // timestamps as field successors.
            let read_times: Vec<_> = (0..read_len)
                .map(|i| [0, 1, 3, 7, 255, 65535, u64::MAX - 1, u64::MAX][i % 8])
                .collect();
            let write_times: Vec<_> = read_times.iter().map(|t| t.wrapping_add(1)).collect();
            let final_times: Vec<_> = (0..table_len).map(|_| rng.f128().hi).collect();
            let values: Vec<_> = (0..read_len).map(|_| rng.f128()).collect();
            let table: Vec<_> = (0..table_len).map(|_| rng.f128()).collect();
            let d = read_len.max(table_len);
            for _ in 0..3 {
                let f = Fingerprint {
                    shift: rng.f128(),
                    address: rng.f128(),
                    value: rng.f128(),
                    time: rng.f128(),
                };
                let cache = FingerprintCache::new(
                    f,
                    &addresses,
                    &values,
                    &read_times,
                    &write_times,
                    &table,
                    &final_times,
                );
                assert_eq!(cache.reads.len(), read_len);
                assert_eq!(cache.table.len(), table_len);
                for side in 0..2 {
                    for leaf in 0..2 * d {
                        let half = leaf / d;
                        let i = leaf % d;
                        let expected = match (side, half) {
                            (0, 0) if i < table_len => f.entry(i as u64, table[i], 0),
                            (0, 1) if i < read_len => {
                                f.entry(addresses[i], values[i], write_times[i])
                            }
                            (1, 0) if i < read_len => {
                                f.entry(addresses[i], values[i], read_times[i])
                            }
                            (1, 1) if i < table_len => f.entry(i as u64, table[i], final_times[i]),
                            _ => F128::ONE,
                        };
                        assert_eq!(cache.leaf(side, leaf, d), expected);
                    }
                }
            }
        }
    }

    #[test]
    #[ignore = "manual release timing of online fingerprint caching"]
    fn sparse_lookup_cache_leaves_benchmark() {
        let matrix = SparseBinaryMatrix::new(
            1024,
            4096,
            (0..1024)
                .map(|i| (0..47).map(|j| (i * 31 + j * 17) % 4096).collect())
                .collect(),
        );
        let p = Prepared::new(&matrix).unwrap();
        let s = p.statement([31; 32]);
        let x = query(s.row_log, 931);
        let y = query(s.col_log, 98131);
        let reads = p.gather(&x, &y).unwrap();
        let v = value(&p, &reads);
        let mut reference = None;
        for iteration in 0..4 {
            for cached in if iteration % 2 == 0 {
                [false, true]
            } else {
                [true, false]
            } {
                let start = std::time::Instant::now();
                let (proof, claims, next) = with_fusion(true, || {
                    with_cache_leaves(cached, || {
                        let mut challenger = ch();
                        let (proof, claims) =
                            prove(&p, &s, &x, &y, v, &reads, &mut challenger).unwrap();
                        (proof, claims, challenger.sample_f128())
                    })
                });
                let elapsed = start.elapsed().as_secs_f64() * 1000.0;
                let result = (bincode::serialize(&proof).unwrap(), claims, next);
                if let Some(ref expected) = reference {
                    assert_eq!(expected, &result);
                } else {
                    reference = Some(result);
                }
                eprintln!(
                    "sparse_lookup_cache threads={} entries={} iteration={iteration} cached={cached} ms={elapsed:.3}",
                    rayon::current_num_threads(),
                    p.value.len()
                );
            }
        }
    }

    #[test]
    #[ignore = "manual release timing of identical sparse PIOPs"]
    fn sparse_lookup_engineering_benchmark() {
        let matrix = SparseBinaryMatrix::new(
            1024,
            4096,
            (0..1024)
                .map(|i| (0..47).map(|j| (i * 31 + j * 17) % 4096).collect())
                .collect(),
        );
        let p = Prepared::new(&matrix).unwrap();
        let s = p.statement([31; 32]);
        let x = query(s.row_log, 931);
        let y = query(s.col_log, 98131);
        let reads = p.gather(&x, &y).unwrap();
        let v = value(&p, &reads);
        let mut reference = None;
        for iteration in 0..4 {
            for fused in if iteration % 2 == 0 {
                [false, true]
            } else {
                [true, false]
            } {
                let start = std::time::Instant::now();
                let proof = with_fusion(fused, || {
                    prove(&p, &s, &x, &y, v, &reads, &mut ch()).unwrap().0
                });
                let elapsed = start.elapsed().as_secs_f64() * 1000.0;
                let bytes = bincode::serialize(&proof).unwrap();
                if let Some(ref expected) = reference {
                    assert_eq!(expected, &bytes);
                } else {
                    reference = Some(bytes);
                }
                eprintln!(
                    "sparse_lookup_engineering threads={} entries={} iteration={iteration} fused={fused} ms={elapsed:.3}",
                    rayon::current_num_threads(),
                    p.value.len()
                );
            }
        }
    }

    #[test]
    fn balanced_padding_preserves_matrix_and_authenticates_all_memory_sources() {
        for matrix in [
            SparseBinaryMatrix::new(1, 1, vec![vec![]]),
            SparseBinaryMatrix::new(1, 1, vec![vec![0]]),
            SparseBinaryMatrix::new(2, 4, vec![vec![], vec![]]),
            SparseBinaryMatrix::new(1, 4, vec![vec![0, 1, 2]]),
            SparseBinaryMatrix::new(4, 8, vec![vec![0, 3, 7]; 4]),
            SparseBinaryMatrix::new(
                8,
                2,
                vec![
                    vec![0, 1],
                    vec![0, 1],
                    vec![1],
                    vec![0],
                    vec![1],
                    vec![0],
                    vec![1],
                    vec![],
                ],
            ),
            // 513 nonzeros create 511 real padding reads. Spreading them
            // shrinks both maximum timestamps from >255 to <=5.
            SparseBinaryMatrix::new(
                256,
                256,
                (0..256)
                    .map(|i| {
                        if i == 0 {
                            vec![0, 1, 2]
                        } else {
                            vec![(7 * i) % 256, (7 * i + 1) % 256]
                        }
                    })
                    .collect(),
            ),
        ] {
            let old = Prepared::new(&matrix).unwrap();
            let p = Prepared::new_balanced_padding(&matrix).unwrap();
            let s = p.statement([57; 32]);
            assert_eq!(s, old.statement(s.digest));
            assert_eq!(p.value, old.value);
            assert_eq!(&p.row[..p.nonzeros], &old.row[..p.nonzeros]);
            assert_eq!(&p.col[..p.nonzeros], &old.col[..p.nonzeros]);
            assert!(old.row[old.nonzeros..].iter().all(|&address| address == 0));
            assert!(old.col[old.nonzeros..].iter().all(|&address| address == 0));
            for i in p.nonzeros..p.value.len() {
                assert_eq!(p.value[i], 0);
                assert_eq!(p.row[i], (i % matrix.num_rows) as u64);
                assert_eq!(p.col[i], (i % matrix.num_cols) as u64);
            }
            for (addresses, read, write, final_times) in [
                (
                    &p.row,
                    &p.row_read_time,
                    &p.row_write_time,
                    &p.row_final_time,
                ),
                (
                    &p.col,
                    &p.col_read_time,
                    &p.col_write_time,
                    &p.col_final_time,
                ),
            ] {
                let mut counts = vec![0u64; final_times.len()];
                let mut padding_counts = counts.clone();
                for (i, &address) in addresses.iter().enumerate() {
                    let address = address as usize;
                    assert_eq!(read[i], counts[address]);
                    counts[address] += 1;
                    assert_eq!(write[i], counts[address]);
                    if i >= p.nonzeros {
                        padding_counts[address] += 1;
                    }
                }
                assert_eq!(&counts, final_times);
                assert_eq!(counts.iter().sum::<u64>(), p.value.len() as u64);
                assert!(
                    padding_counts.iter().max().unwrap() - padding_counts.iter().min().unwrap()
                        <= 1
                );
            }
            if p.nonzeros == 513 {
                assert!(old.row_final_time.iter().copied().max().unwrap() > 255);
                assert!(old.col_final_time.iter().copied().max().unwrap() > 255);
                assert!(p.row_final_time.iter().copied().max().unwrap() <= 5);
                assert!(p.col_final_time.iter().copied().max().unwrap() <= 5);
            }
            for kind in 0..3 {
                let x = match kind {
                    0 => query(s.row_log, 937),
                    1 => vec![F128::ZERO; s.row_log],
                    _ => vec![F128::ONE; s.row_log],
                };
                let y = match kind {
                    0 => query(s.col_log, 2791),
                    1 => vec![F128::ZERO; s.col_log],
                    _ => vec![F128::ONE; s.col_log],
                };
                let reads = p.gather(&x, &y).unwrap();
                let v = value(&p, &reads);
                assert_eq!(v, value(&old, &old.gather(&x, &y).unwrap()));
                let rows = eq_table_reference(&x);
                let cols = eq_table_reference(&y);
                let direct = matrix
                    .rows
                    .iter()
                    .enumerate()
                    .fold(F128::ZERO, |sum, (i, columns)| {
                        columns.iter().fold(sum, |sum, &j| sum + rows[i] * cols[j])
                    });
                assert_eq!(v, direct);
                let run = |cached| {
                    with_cache_leaves(cached, || {
                        let mut prover = ch();
                        let (proof, claims) =
                            prove(&p, &s, &x, &y, v, &reads, &mut prover).unwrap();
                        (proof, claims, prover.sample_f128())
                    })
                };
                let (proof, claims, next) = run(false);
                let cached = run(true);
                assert_eq!(
                    bincode::serialize(&proof).unwrap(),
                    bincode::serialize(&cached.0).unwrap()
                );
                assert_eq!(claims, cached.1);
                assert_eq!(next, cached.2);
                assert_eq!(claims.len(), 17);
                let mut verifier = ch();
                assert_eq!(
                    verify(&s, &x, &y, v, &proof, &mut verifier).unwrap(),
                    claims
                );
                assert_eq!(verifier.sample_f128(), next);
                check_claims(&p, &reads, &claims).unwrap();
                for i in 0..17 {
                    let mut bad = proof.clone();
                    if i < 3 {
                        bad.matrix.terminal[i] += F128::ONE;
                    } else if i < 10 {
                        bad.row.evaluations[i - 3] += F128::ONE;
                    } else {
                        bad.col.evaluations[i - 10] += F128::ONE;
                    }
                    assert!(
                        verify(&s, &x, &y, v, &bad, &mut ch())
                            .and_then(|claims| check_claims(&p, &reads, &claims))
                            .is_err()
                    );
                }
                if kind == 0 && p.nonzeros < p.value.len() {
                    // Padding is authenticated despite its zero coefficient.
                    let i = p.nonzeros;
                    let mut changed = p.clone();
                    changed.value[i] = 1;
                    assert!(check_claims(&changed, &reads, &claims).is_err());
                    for axis in 0..2 {
                        let mut changed = p.clone();
                        if axis == 0 {
                            changed.row_write_time[i] += 1;
                        } else {
                            changed.col_write_time[i] += 1;
                        }
                        assert!(check_claims(&changed, &reads, &claims).is_err());
                        let mut changed_reads = reads.clone();
                        if axis == 0 {
                            changed_reads.read_row[i] += F128::ONE;
                        } else {
                            changed_reads.read_col[i] += F128::ONE;
                        }
                        assert_eq!(value(&p, &changed_reads), v);
                        assert!(prove(&p, &s, &x, &y, v, &changed_reads, &mut ch()).is_err());
                    }
                }
            }
        }
    }

    #[test]
    fn sparse_lookup_roundtrip_rectangles_repeated_addresses_and_padding() {
        for matrix in [
            SparseBinaryMatrix::new(4, 8, vec![vec![7, 1, 3], vec![1], vec![0, 7], vec![3]]),
            SparseBinaryMatrix::new(
                8,
                2,
                vec![
                    vec![1],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                ],
            ),
            SparseBinaryMatrix::new(1, 1, vec![vec![0]]),
            SparseBinaryMatrix::new(2, 4, vec![vec![], vec![]]),
            SparseBinaryMatrix::new(2, 2, vec![vec![1, 1, 0], vec![0, 1, 0, 1]]),
        ] {
            let p = Prepared::new(&matrix).unwrap();
            let s = p.statement([7; 32]);
            let x = query(s.row_log, 5);
            let y = query(s.col_log, 91);
            let reads = p.gather(&x, &y).unwrap();
            let v = value(&p, &reads);
            let mut prover = ch();
            let (proof, claims) = prove(&p, &s, &x, &y, v, &reads, &mut prover).unwrap();
            let mut verifier = ch();
            let got = verify(&s, &x, &y, v, &proof, &mut verifier).unwrap();
            assert_eq!(claims.len(), 17);
            assert_eq!(got, claims);
            check_claims(&p, &reads, &got).unwrap();
            assert_eq!(prover.sample_f128(), verifier.sample_f128());
            let bytes = bincode::serialize(&proof).unwrap();
            let proof: Proof = bincode::deserialize(&bytes).unwrap();
            assert!(verify(&s, &x, &y, v, &proof, &mut ch()).is_ok());
            let count = p.row_final_time.iter().sum::<u64>();
            assert_eq!(count, 1u64 << s.entry_log);
            assert!(p.value[s.nonzeros..].iter().all(|&x| x == 0));
        }
    }

    #[test]
    fn sparse_lookup_rejects_source_changes_wrong_reads_and_forged_messages() {
        let matrix =
            SparseBinaryMatrix::new(4, 4, vec![vec![3, 1, 2], vec![3], vec![0], vec![2, 1]]);
        let p = Prepared::new(&matrix).unwrap();
        let s = p.statement([9; 32]);
        let x = query(s.row_log, 13);
        let y = query(s.col_log, 111);
        let reads = p.gather(&x, &y).unwrap();
        let v = value(&p, &reads);
        let (proof, claims) = prove(&p, &s, &x, &y, v, &reads, &mut ch()).unwrap();
        let reject = |candidate: &Proof| {
            verify(&s, &x, &y, v, candidate, &mut ch())
                .and_then(|c| check_claims(&p, &reads, &c))
                .is_err()
        };
        for i in 0..3 {
            let mut bad = proof.clone();
            bad.matrix.terminal[i] += F128::ONE;
            assert!(reject(&bad));
        }
        for axis in 0..2 {
            for i in 0..7 {
                let mut bad = proof.clone();
                if axis == 0 {
                    bad.row.evaluations[i] += F128::ONE;
                } else {
                    bad.col.evaluations[i] += F128::ONE;
                }
                assert!(reject(&bad));
            }
        }
        let mut bad = proof.clone();
        bad.matrix.rounds.pop();
        assert!(reject(&bad));
        let mut bad = proof.clone();
        bad.row.product.trees[0].layers.pop();
        assert!(reject(&bad));
        let mut changed = p.clone();
        changed.row[0] ^= 1;
        assert!(check_claims(&changed, &reads, &claims).is_err());
        let mut changed = p.clone();
        changed.col[0] ^= 1;
        assert!(check_claims(&changed, &reads, &claims).is_err());
        let mut changed = p.clone();
        changed.value[s.nonzeros] = 1;
        assert!(check_claims(&changed, &reads, &claims).is_err());
        let mut changed = reads.clone();
        changed.read_row[0] += F128::new(0x7123, 0x999);
        assert!(check_claims(&p, &changed, &claims).is_err());
        // Recompute the matrix claim so only the lookup can detect the forged gather.
        let forged_value = value(&p, &changed);
        assert!(prove(&p, &s, &x, &y, forged_value, &changed, &mut ch()).is_err());
        assert!(verify(&s, &x, &y, v + F128::ONE, &proof, &mut ch()).is_err());
        let mut changed_s = s.clone();
        changed_s.digest[0] ^= 1;
        assert!(verify(&changed_s, &x, &y, v, &proof, &mut ch()).is_err());
        let mut changed_s = s.clone();
        changed_s.entry_log = MAX_LOG + 1;
        assert!(verify(&changed_s, &x, &y, v, &proof, &mut ch()).is_err());
    }

    #[test]
    fn sparse_lookup_timestamp_increment_is_integer_not_field_xor() {
        let matrix = SparseBinaryMatrix::new(1, 4, vec![vec![0, 1, 2]]);
        let p = Prepared::new(&matrix).unwrap();
        assert_eq!(p.row_read_time, [0, 1, 2, 3]);
        assert_eq!(p.row_write_time, [1, 2, 3, 4]);
        assert_ne!(
            raw(p.row_read_time[1]) + F128::ONE,
            raw(p.row_write_time[1])
        );
        assert_eq!(p.row_final_time, [4]);
        assert_eq!(p.value, [1, 1, 1, 0]);
        assert!(Prepared::new(&SparseBinaryMatrix::new(2, 2, vec![vec![2], vec![]])).is_err());
        assert!(
            Prepared::new(&SparseBinaryMatrix::new(3, 2, vec![vec![], vec![], vec![]])).is_err()
        );
    }

    #[test]
    fn sparse_lookup_canonicalizes_binary_postings_before_padding_and_timestamps() {
        let postings = vec![
            vec![65, 1, 65, 2, 2, 1], // Everything cancels, across two bitset words.
            vec![127, 64, 127, 3, 3, 3, 64], // Only column 3 remains.
            vec![7, 1, 7, 1, 7, 0],   // Columns 0 and 7 remain, in canonical order.
            vec![66, 66],
        ];
        let matrix = SparseBinaryMatrix::new(4, 128, postings.clone());
        let mut reordered = postings;
        for row in &mut reordered {
            row.reverse();
        }
        let reordered = SparseBinaryMatrix::new(4, 128, reordered);
        let p = Prepared::new(&matrix).unwrap();
        let same = Prepared::new(&reordered).unwrap();
        assert_eq!(p.nonzeros, 3);
        assert_eq!(p.entry_log, 2);
        assert_eq!(p.row, [1, 2, 2, 0]);
        assert_eq!(p.col, [3, 0, 7, 0]);
        assert_eq!(p.value, [1, 1, 1, 0]);
        for source in OracleId::STATIC {
            assert_eq!(p.static_column(source), same.static_column(source));
        }
        let empty = Prepared::new(&SparseBinaryMatrix::new(
            2,
            2,
            vec![vec![0, 0, 1, 1], vec![1, 1]],
        ))
        .unwrap();
        assert_eq!(empty.nonzeros, 0);
        assert_eq!(empty.value, [0]);
        assert_eq!(empty.row_final_time, [1, 0]);

        // Independent original-posting matrix evaluation, retaining every XOR
        // contribution, must agree with the canonical COO at several points.
        for seed in [0, 3, 29, 4091] {
            let x = query(p.row_log, seed);
            let y = query(p.col_log, seed + 101);
            let eq_x = eq_table(&x);
            let eq_y = eq_table(&y);
            let expected = matrix
                .rows
                .iter()
                .enumerate()
                .fold(F128::ZERO, |sum, (r, cols)| {
                    cols.iter().fold(sum, |sum, &c| sum + eq_x[r] * eq_y[c])
                });
            let reads = p.gather(&x, &y).unwrap();
            assert_eq!(value(&p, &reads), expected);
            let s = p.statement([11; 32]);
            let (proof, _) = prove(&p, &s, &x, &y, expected, &reads, &mut ch()).unwrap();
            let claims = verify(&s, &x, &y, expected, &proof, &mut ch()).unwrap();
            check_claims(&p, &reads, &claims).unwrap();
        }
    }
}
