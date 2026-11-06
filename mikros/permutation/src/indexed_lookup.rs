//! Public-address indexed lookup from Appendix `Indexed Lookup via Packed
//! Permutation`. The access schedule and multiplicative counters are public
//! preprocessing. Every witness-dependent encoding and polynomial coefficient
//! is constructed inside `prove`. Both terminal equations refer to the original
//! query/table bit oracles; their PCS authentication is the caller's obligation.
use crate::{
    Digest, Error,
    bits::PackedBits,
    claims::{LinearQuery, OracleSpec},
    field::{FastField, SmallField, raw},
    packed, permutation,
    profile::{Engine, Timings as KernelTimings},
};
use flock_field::F128;
use flock_multilinear::{IndexOrder, eq_table};
use flock_transcript::challenger::Challenger;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}
fn power(mut x: F128, mut n: u128) -> F128 {
    let mut y = F128::ONE;
    while n != 0 {
        if n & 1 != 0 {
            y *= x;
        }
        x = x.square();
        n >>= 1;
    }
    y
}
fn frobenius(mut x: F128, n: usize) -> F128 {
    for _ in 0..n {
        x = x.square();
    }
    x
}

/// Exact prime divisors of 2^b-1 for every supported position field. Testing
/// all these divisors certifies full multiplicative order, not just degree.
fn prime_divisors(bits: usize) -> &'static [u128] {
    match bits {
        1 => &[],
        2 => &[3],
        4 => &[3, 5],
        8 => &[3, 5, 17],
        16 => &[3, 5, 17, 257],
        32 => &[3, 5, 17, 257, 65537],
        _ => unreachable!("position field is at most 32 bits"),
    }
}
fn primitive(field: &SmallField) -> Result<(u128, F128), Error> {
    let order = (1u128 << field.bits) - 1;
    for coordinate in 1..=order {
        let x = field.embed(coordinate);
        if power(x, order) == F128::ONE
            && prime_divisors(field.bits)
                .iter()
                .all(|p| power(x, order / p) != F128::ONE)
        {
            return Ok((coordinate, x));
        }
    }
    Err(Error::Constraint)
}

/// Inverse of a real subfield embedding, obtained by F2 Gaussian elimination.
/// F128 polynomial bits are never interpreted as smaller-field coordinates.
struct Coordinates {
    pivots: [(u128, u128); 128],
}
impl Coordinates {
    fn new(field: &SmallField) -> Self {
        let mut out = Self {
            pivots: [(0, 0); 128],
        };
        for (i, &basis) in field.basis().iter().enumerate() {
            let mut x = raw(basis);
            let mut coordinate = 1u128 << i;
            while x != 0 {
                let p = 127 - x.leading_zeros() as usize;
                if out.pivots[p].0 == 0 {
                    out.pivots[p] = (x, coordinate);
                    break;
                }
                x ^= out.pivots[p].0;
                coordinate ^= out.pivots[p].1;
            }
            assert_ne!(x, 0, "subfield embedding basis is independent");
        }
        out
    }
    fn encode(&self, value: F128) -> Result<u128, Error> {
        let mut x = raw(value);
        let mut coordinate = 0;
        while x != 0 {
            let p = 127 - x.leading_zeros() as usize;
            if self.pivots[p].0 == 0 {
                return Err(Error::Constraint);
            }
            x ^= self.pivots[p].0;
            coordinate ^= self.pivots[p].1;
        }
        Ok(coordinate)
    }
}

/// Private fields guarantee the multiplicative counter schedule was checked
/// during construction. This is public preprocessing, never a proof witness.
pub struct PublicSchedule {
    table_rows: usize,
    addresses: Vec<usize>,
    value_bits: usize,
    d: usize,
    position_bits: usize,
    tuple_bits: usize,
    segment_len: usize,
    digest: Digest,
    value_coordinates: Vec<u128>,
    value_basis: Vec<F128>,
    public_left: Vec<u128>,
    public_right: Vec<u128>,
    generator: F128,
    generator_order: u128,
}
impl PublicSchedule {
    pub fn new(table_rows: usize, addresses: &[usize], value_bits: usize) -> Result<Self, Error> {
        if ![1, 2, 4, 8, 16].contains(&value_bits) {
            return Err(Error::Width);
        }
        if table_rows == 0 || table_rows > 1usize << 30 || addresses.len() > 1usize << 30 {
            return Err(Error::Length);
        }
        if addresses.iter().any(|&a| a >= table_rows) {
            return Err(Error::OutOfRange);
        }
        let address_bits = if table_rows == 1 {
            0
        } else {
            (table_rows - 1).ilog2() as usize + 1
        };
        let mut d = (address_bits.div_ceil(value_bits) + 1).next_power_of_two();
        while value_bits * d <= 32 && ((1u128 << (value_bits * d)) - 1) <= addresses.len() as u128 {
            d *= 2;
        }
        let position_bits = value_bits.checked_mul(d).ok_or(Error::Capacity)?;
        if position_bits > 32 || !(128 / value_bits).is_multiple_of(4 * d) {
            return Err(Error::Capacity);
        }
        let tuple_bits = 2 * position_bits;
        let packing_width = 128 / tuple_bits;
        // Pad the query and table independently to the same power of two.
        // Additional identical zero padding also covers the packed shape's
        // minimum length requirement on very small fixtures.
        let segment_len = table_rows
            .max(addresses.len())
            .max(packing_width * packing_width / 2)
            .max(1)
            .next_power_of_two();
        let encoded_rows = segment_len.checked_mul(2).ok_or(Error::Length)?;
        permutation::CommittedStatement::new(encoded_rows, tuple_bits)?;
        let base = SmallField::for_width(value_bits)?;
        let position = FastField::for_width(position_bits)?;
        let tuple = FastField::for_width(tuple_bits)?;
        let coordinates = Coordinates::new(tuple.base);
        let beta = if d == 1 {
            F128::ONE
        } else {
            position.base.basis()[1]
        };
        // 1,beta,...,beta^(d-1) is a basis over the true embedded base field.
        let theta = power(beta, (d - 1) as u128);
        let vartheta = tuple.base.basis()[1];
        if frobenius(vartheta, position_bits) == vartheta {
            return Err(Error::Constraint);
        }
        let value_basis: Vec<_> = base.basis().iter().map(|&v| theta * v).collect();
        let value_coordinates = value_basis
            .iter()
            .map(|&v| coordinates.encode(v))
            .collect::<Result<Vec<_>, _>>()?;
        let labels: Vec<F128> = (0..table_rows)
            .map(|j| {
                let mut x = j as u128;
                let mut beta_power = F128::ONE;
                let mut label = F128::ZERO;
                for _ in 0..d - 1 {
                    label += beta_power * base.embed(x & base.mask());
                    x >>= value_bits;
                    beta_power *= beta;
                }
                debug_assert_eq!(x, 0);
                label
            })
            .collect();
        let (generator_coordinate, generator) = primitive(position.base)?;
        let generator_order = (1u128 << position_bits) - 1;
        if generator_order <= addresses.len() as u128 {
            return Err(Error::Capacity);
        }
        let mut counters = vec![1u128; table_rows];
        let mut public_left = vec![0; encoded_rows];
        let mut public_right = vec![0; encoded_rows];
        for (i, &a) in addresses.iter().enumerate() {
            let read = position.embed(counters[a]);
            counters[a] = position.multiply(counters[a], generator_coordinate);
            let write = position.embed(counters[a]);
            public_left[i] = coordinates.encode(labels[a] + vartheta * read)?;
            public_right[segment_len + i] = coordinates.encode(labels[a] + vartheta * write)?;
        }
        for j in 0..table_rows {
            public_left[segment_len + j] =
                coordinates.encode(labels[j] + vartheta * position.embed(counters[j]))?;
            public_right[j] = coordinates.encode(labels[j] + vartheta)?;
        }
        let mut hash = blake3::Hasher::new();
        hash.update(b"mikros-public-indexed-lookup-schedule-v1");
        for x in [
            table_rows,
            addresses.len(),
            value_bits,
            d,
            tuple_bits,
            segment_len,
        ] {
            hash.update(&(x as u64).to_le_bytes());
        }
        for &a in addresses {
            hash.update(&(a as u64).to_le_bytes());
        }
        hash.update(&raw(generator).to_le_bytes());
        Ok(Self {
            table_rows,
            addresses: addresses.to_vec(),
            value_bits,
            d,
            position_bits,
            tuple_bits,
            segment_len,
            digest: *hash.finalize().as_bytes(),
            value_coordinates,
            value_basis,
            public_left,
            public_right,
            generator,
            generator_order,
        })
    }
    pub fn query_rows(&self) -> usize {
        self.addresses.len()
    }
    pub fn table_rows(&self) -> usize {
        self.table_rows
    }
    pub fn value_bits(&self) -> usize {
        self.value_bits
    }
    pub fn d(&self) -> usize {
        self.d
    }
    pub fn position_bits(&self) -> usize {
        self.position_bits
    }
    pub fn tuple_bits(&self) -> usize {
        self.tuple_bits
    }
    pub fn packing_width(&self) -> usize {
        128 / self.tuple_bits
    }
    pub fn segment_len(&self) -> usize {
        self.segment_len
    }
    pub fn encoded_rows(&self) -> usize {
        2 * self.segment_len
    }
    pub fn generator(&self) -> F128 {
        self.generator
    }
    pub fn generator_order(&self) -> u128 {
        self.generator_order
    }
    pub fn addresses(&self) -> &[usize] {
        &self.addresses
    }
    /// Coefficients of the original value bits in the terminal F128 equations.
    pub fn value_basis(&self) -> &[F128] {
        &self.value_basis
    }
    pub fn resident_bytes(&self) -> usize {
        (self.public_left.len() + self.public_right.len() + self.value_coordinates.len()) * 16
            + self.value_basis.len() * 16
            + self.addresses.len() * std::mem::size_of::<usize>()
    }
    fn shape(&self) -> Result<permutation::CommittedStatement, Error> {
        permutation::CommittedStatement::new(self.encoded_rows(), self.tuple_bits)
    }
    fn check_shape(&self, values: &PackedBits, table: &PackedBits) -> Result<(), Error> {
        values.validate()?;
        table.validate()?;
        if values.rows() != self.query_rows()
            || table.rows() != self.table_rows
            || values.width() != self.value_bits
            || table.width() != self.value_bits
        {
            return Err(Error::Statement);
        }
        Ok(())
    }
    fn encoded_value(&self, mut value: u128) -> u128 {
        let mut out = 0;
        while value != 0 {
            let bit = value.trailing_zeros() as usize;
            out ^= self.value_coordinates[bit];
            value &= value - 1;
        }
        out
    }
    fn source_value(&self, mut value: u128) -> F128 {
        let mut out = F128::ZERO;
        while value != 0 {
            let bit = value.trailing_zeros() as usize;
            out += self.value_basis[bit];
            value &= value - 1;
        }
        out
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Statement {
    pub schedule_digest: Digest,
    pub values: OracleSpec,
    pub table: OracleSpec,
}
impl Statement {
    pub fn new(
        plan: &PublicSchedule,
        values: &PackedBits,
        table: &PackedBits,
    ) -> Result<Self, Error> {
        plan.check_shape(values, table)?;
        Ok(Self {
            schedule_digest: plan.digest,
            values: OracleSpec {
                rows: values.rows(),
                width: values.width(),
                digest: values.digest_parallel(),
            },
            table: OracleSpec {
                rows: table.rows(),
                width: table.width(),
                digest: table.digest_parallel(),
            },
        })
    }
    fn bind<C: Challenger>(&self, plan: &PublicSchedule, c: &mut C) -> Result<(), Error> {
        if self.schedule_digest != plan.digest
            || self.values.rows != plan.query_rows()
            || self.table.rows != plan.table_rows
            || self.values.width != plan.value_bits
            || self.table.width != plan.value_bits
        {
            return Err(Error::Statement);
        }
        c.observe_label(b"mikros-public-indexed-lookup-v1");
        c.observe_bytes(&self.schedule_digest);
        c.observe_bytes(&self.values.digest);
        c.observe_bytes(&self.table.digest);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Strategy {
    Packed,
    FullExtension,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Proof {
    Packed(packed::Proof),
    FullExtension(permutation::Proof),
}
impl Proof {
    pub fn field_elements(&self) -> usize {
        match self {
            Self::Packed(p) => p.field_elements(),
            Self::FullExtension(p) => p.field_elements(),
        }
    }
}
/// A tensor functional on the encoded row order. With one column it is an
/// ordinary MLE; packed proofs retain their arbitrary column weights.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Functional {
    pub row_point: Vec<F128>,
    pub columns: Vec<F128>,
}
impl Functional {
    fn sum<F: Fn(usize) -> F128 + Sync>(&self, length: usize, value: F) -> Result<F128, Error> {
        if self.columns.is_empty()
            || !self.columns.len().is_power_of_two()
            || self.row_point.len() >= usize::BITS as usize
            || (1usize << self.row_point.len()).checked_mul(self.columns.len()) != Some(length)
        {
            return Err(Error::ProofShape);
        }
        let cut = self.row_point.len() / 2;
        let low = eq_table(&self.row_point[..cut], F128::ONE, IndexOrder::LowToHigh);
        let high = eq_table(&self.row_point[cut..], F128::ONE, IndexOrder::LowToHigh);
        let width = self.columns.len();
        Ok((0..high.len())
            .into_par_iter()
            .map(|hi| {
                let mut acc = F128::ZERO;
                for (lo, &weight) in low.iter().enumerate() {
                    let row = hi * low.len() + lo;
                    let mut inner = F128::ZERO;
                    for (column, &column_weight) in self.columns.iter().enumerate() {
                        inner += column_weight * value(row * width + column);
                    }
                    acc += weight * inner;
                }
                high[hi] * acc
            })
            .reduce(|| F128::ZERO, |a, b| a + b))
    }
}
/// Each side is one linear equation over BOTH original sources. The public
/// tags are already subtracted. Authenticate both equations against the bound
/// query/table commitments; neither encoded tuple oracle is trusted.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceClaim {
    pub functional: Functional,
    pub value: F128,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claims {
    pub statement: Statement,
    pub sides: [SourceClaim; 2],
}
impl Claims {
    /// Independent linear-time source oracle for tests/benchmarks, not a PCS.
    pub fn check(
        &self,
        values: &PackedBits,
        table: &PackedBits,
        plan: &PublicSchedule,
    ) -> Result<(), Error> {
        if Statement::new(plan, values, table)? != self.statement {
            return Err(Error::Source);
        }
        for side in 0..2 {
            let actual = self.sides[side].functional.sum(plan.encoded_rows(), |i| {
                let (bits, row) = if (i < plan.segment_len) == (side == 0) {
                    (values, i % plan.segment_len)
                } else {
                    (table, i % plan.segment_len)
                };
                if row < bits.rows() {
                    plan.source_value(bits.row(row))
                } else {
                    F128::ZERO
                }
            })?;
            if actual != self.sides[side].value {
                return Err(Error::Source);
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Timings {
    pub source_checks_ms: f64,
    pub tuple_encoding_ms: f64,
    pub coefficient_build_ms: f64,
    pub reduction_ms: f64,
    pub public_reduction_ms: f64,
    pub total_ms: f64,
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct VerifyTimings {
    pub replay_ms: f64,
    pub public_reduction_ms: f64,
    pub total_ms: f64,
}
pub struct Output {
    pub proof: Proof,
    pub claims: Claims,
    pub timings: Timings,
    pub coefficient_bytes: usize,
    pub tree_storage_bytes: usize,
    pub kernel_timings: KernelTimings,
}
fn packed_leaves(leaves: [packed::Claim; 2]) -> [SourceClaim; 2] {
    leaves.map(|leaf| SourceClaim {
        functional: Functional {
            row_point: leaf.row_point,
            columns: leaf.column_weights,
        },
        value: leaf.value,
    })
}
fn ordinary_leaves(leaves: permutation::CommittedClaims) -> Result<[SourceClaim; 2], Error> {
    let mut out = Vec::new();
    for leaf in leaves.evaluations {
        let LinearQuery::Rows {
            point,
            row_offset: 0,
            ..
        } = leaf.query
        else {
            return Err(Error::ProofShape);
        };
        out.push(SourceClaim {
            functional: Functional {
                row_point: point,
                columns: vec![F128::ONE],
            },
            value: leaf.value,
        });
    }
    out.try_into().map_err(|_| Error::ProofShape)
}
fn reduce_public(
    plan: &PublicSchedule,
    statement: &Statement,
    mut sides: [SourceClaim; 2],
) -> Result<Claims, Error> {
    let field = FastField::for_width(plan.tuple_bits)?;
    for (side, public) in sides
        .iter_mut()
        .zip([&plan.public_left, &plan.public_right])
    {
        side.value += side
            .functional
            .sum(plan.encoded_rows(), |i| field.embed(public[i]))?;
    }
    Ok(Claims {
        statement: statement.clone(),
        sides,
    })
}

pub fn prove<C: Challenger>(
    plan: &PublicSchedule,
    statement: &Statement,
    values: &PackedBits,
    table: &PackedBits,
    strategy: Strategy,
    c: &mut C,
) -> Result<Output, Error> {
    let total = Instant::now();
    let mut timings = Timings::default();
    let t = Instant::now();
    if Statement::new(plan, values, table)? != *statement {
        return Err(Error::Source);
    }
    statement.bind(plan, c)?;
    timings.source_checks_ms = ms(t);
    let t = Instant::now();
    let encode = |side: usize, public: &[u128]| {
        PackedBits::from_fn_parallel(plan.encoded_rows(), plan.tuple_bits, |i| {
            let (bits, row) = if (i < plan.segment_len) == (side == 0) {
                (values, i % plan.segment_len)
            } else {
                (table, i % plan.segment_len)
            };
            public[i]
                ^ if row < bits.rows() {
                    plan.encoded_value(bits.row(row))
                } else {
                    0
                }
        })
    };
    let source = permutation::prepare(
        encode(0, &plan.public_left)?,
        encode(1, &plan.public_right)?,
    )?;
    timings.tuple_encoding_ms = ms(t);
    let (proof, leaves, coefficient_bytes, tree_storage_bytes, kernel_timings) = match strategy {
        Strategy::Packed => {
            let t = Instant::now();
            let prepared = packed::prepare(&source)?;
            let bytes = prepared.resident_bytes();
            timings.coefficient_build_ms = ms(t);
            let t = Instant::now();
            let (proof, leaves, kernel) = packed::prove(&prepared, c)?;
            // Witness-dependent layers and off-domain samples are rebuilt for
            // every proof. Include their release in the online reduction time.
            drop(prepared);
            timings.reduction_ms = ms(t);
            (
                Proof::Packed(proof),
                packed_leaves(leaves),
                bytes,
                0,
                kernel,
            )
        }
        Strategy::FullExtension => {
            let t = Instant::now();
            let (proof, leaves, stats, kernel) = permutation::prove_committed_lockstep(
                &source,
                permutation::Options {
                    strategy: permutation::Strategy::FullExtension,
                    ..Default::default()
                },
                Engine::Combined,
                c,
            )?;
            timings.reduction_ms = ms(t);
            let bytes = stats.left.compact_bytes
                + stats.left.extension_bytes
                + stats.right.compact_bytes
                + stats.right.extension_bytes;
            (
                Proof::FullExtension(proof),
                ordinary_leaves(leaves)?,
                0,
                bytes,
                kernel,
            )
        }
    };
    drop(source);
    let t = Instant::now();
    let claims = reduce_public(plan, statement, leaves)?;
    timings.public_reduction_ms = ms(t);
    timings.total_ms = ms(total);
    Ok(Output {
        proof,
        claims,
        timings,
        coefficient_bytes,
        tree_storage_bytes,
        kernel_timings,
    })
}
pub fn verify_profiled<C: Challenger>(
    plan: &PublicSchedule,
    statement: &Statement,
    proof: &Proof,
    c: &mut C,
) -> Result<(Claims, VerifyTimings), Error> {
    let total = Instant::now();
    let t = Instant::now();
    statement.bind(plan, c)?;
    let shape = plan.shape()?;
    let leaves = match proof {
        Proof::Packed(proof) => packed_leaves(packed::verify(&shape, proof, c)?),
        Proof::FullExtension(proof) => {
            ordinary_leaves(permutation::verify_committed_lockstep(&shape, proof, c)?)?
        }
    };
    let replay_ms = ms(t);
    let t = Instant::now();
    let claims = reduce_public(plan, statement, leaves)?;
    Ok((
        claims,
        VerifyTimings {
            replay_ms,
            public_reduction_ms: ms(t),
            total_ms: ms(total),
        },
    ))
}
pub fn verify<C: Challenger>(
    plan: &PublicSchedule,
    statement: &Statement,
    proof: &Proof,
    c: &mut C,
) -> Result<Claims, Error> {
    verify_profiled(plan, statement, proof, c).map(|(claims, _)| claims)
}

#[cfg(test)]
mod field_tests {
    use super::*;
    #[test]
    fn true_subfield_coordinates_and_generator_orders() {
        for b in [1, 2, 4, 8, 16, 32, 64] {
            let field = SmallField::for_width(b).unwrap();
            let inverse = Coordinates::new(field);
            for x in [0, 1, 3, 0x1234_5678_9abc_def0u128 & field.mask()] {
                let x = x & field.mask();
                assert_eq!(inverse.encode(field.embed(x)).unwrap(), x);
            }
            if b <= 32 {
                let (_, generator) = primitive(field).unwrap();
                let order = (1u128 << b) - 1;
                assert_eq!(power(generator, order), F128::ONE);
                for &p in prime_divisors(b) {
                    assert_ne!(power(generator, order / p), F128::ONE);
                }
            }
        }
    }
    #[test]
    fn nested_embeddings_and_value_position_counter_encoding_are_injective() {
        for b in [1usize, 2, 4, 8, 16] {
            let plan = PublicSchedule::new(3, &[0, 2, 0, 1, 2], b).unwrap();
            let base = SmallField::for_width(b).unwrap();
            let position = SmallField::for_width(plan.position_bits).unwrap();
            let tuple = SmallField::for_width(plan.tuple_bits).unwrap();
            let position_coordinates = Coordinates::new(position);
            let tuple_coordinates = Coordinates::new(tuple);
            for x in 0..16u128.min(1u128 << b) {
                for y in 0..16u128.min(1u128 << b) {
                    let px = position_coordinates.encode(base.embed(x)).unwrap();
                    let py = position_coordinates.encode(base.embed(y)).unwrap();
                    assert_eq!(
                        position.embed(position.multiply(px, py)),
                        base.embed(base.multiply(x, y))
                    );
                }
                assert_eq!(tuple.embed(plan.encoded_value(x)), plan.source_value(x));
                assert_eq!(
                    tuple_coordinates.encode(plan.source_value(x)).unwrap(),
                    plan.encoded_value(x)
                );
            }
            let mut encodings = std::collections::HashSet::new();
            // Every table position has a distinct public (position, initial
            // counter) tag; adding a value in its reserved Fq coordinate never
            // collides with a different position/value pair.
            for j in 0..3 {
                for x in 0..16u128.min(1u128 << b) {
                    assert!(encodings.insert(plan.public_right[j] ^ plan.encoded_value(x)));
                }
            }
            for i in plan.query_rows()..plan.segment_len {
                assert_eq!(plan.public_left[i], 0);
            }
            for i in plan.table_rows..plan.segment_len {
                assert_eq!(plan.public_right[i], 0);
            }
        }
    }
}
