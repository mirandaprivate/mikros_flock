//! Explicit-table indexed Lasso: Init ⊎ Write = Read ⊎ Final.
//! Integer counters are bit decomposed. Their carry equations are proved with
//! Mikros binary Hadamard, and every terminal claim targets a source bit oracle.
use crate::{
    Digest, Error,
    bits::{PackedBits, bits_for},
    claims::{Claims, LinearQuery, OracleClaim, OracleSpec, Source, matrix_weight, row_weight},
    field::{SmallField, from_raw, raw},
    padded_len,
    permutation::{self, Options},
    sumcheck::LeafClaim,
};
use flock_field::F128;
use flock_transcript::challenger::Challenger;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Statement {
    pub table: PackedBits,
    pub values: OracleSpec,
    pub addresses: OracleSpec,
}
impl Statement {
    pub fn new(
        table: PackedBits,
        values: &PackedBits,
        addresses: &PackedBits,
    ) -> Result<Self, Error> {
        let out = Self {
            table,
            values: OracleSpec::from_bits(values)?,
            addresses: OracleSpec::from_bits(addresses)?,
        };
        out.validate()?;
        Ok(out)
    }
    pub fn counter_bits(&self) -> usize {
        bits_for(self.values.rows)
    }
    pub fn tuple_bits(&self) -> usize {
        self.values.width + self.addresses.width + self.counter_bits()
    }
    pub fn validate(&self) -> Result<(), Error> {
        self.table.validate()?;
        self.values.validate()?;
        self.addresses.validate()?;
        if self.table.rows() == 0
            || self.values.rows != self.addresses.rows
            || self.values.width != self.table.width()
            || self.addresses.width != bits_for(self.table.rows() - 1)
        {
            return Err(Error::Statement);
        }
        padded_len(
            self.table
                .rows()
                .checked_add(self.values.rows)
                .ok_or(Error::Length)?,
        )?;
        padded_len(
            self.values
                .rows
                .checked_mul(self.counter_bits())
                .ok_or(Error::Length)?,
        )?;
        Ok(())
    }
    fn bind<C: Challenger>(&self, c: &mut C) -> Result<(), Error> {
        self.validate()?;
        c.observe_label(b"mikros-bitwidth-indexed-lasso-v1");
        for x in [
            self.table.rows(),
            self.values.rows,
            self.values.width,
            self.addresses.width,
            self.counter_bits(),
        ] {
            c.observe_bytes(&(x as u64).to_le_bytes());
        }
        c.observe_bytes(&self.table.digest());
        c.observe_bytes(&self.values.digest);
        c.observe_bytes(&self.addresses.digest);
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Auxiliary {
    pub read_counters: PackedBits,
    /// Interior h_1,...,h_(c-1); h_0=1 and h_c=0 are public constants.
    pub carry: PackedBits,
    pub final_counters: PackedBits,
}
impl Auxiliary {
    pub fn build(
        statement: &Statement,
        values: &PackedBits,
        addresses: &PackedBits,
    ) -> Result<Self, Error> {
        statement.validate()?;
        statement.values.check(values)?;
        statement.addresses.check(addresses)?;
        let n = values.rows();
        let m = statement.table.rows();
        let cw = statement.counter_bits();
        let mut counters = vec![0u128; m];
        let mut reads = Vec::with_capacity(n);
        let mut carries = Vec::with_capacity(n);
        for i in 0..n {
            let a = addresses.row(i) as usize;
            if a >= m {
                return Err(Error::OutOfRange);
            }
            if values.row(i) != statement.table.row(a) {
                return Err(Error::Source);
            }
            let counter = counters[a];
            reads.push(counter);
            let mut carry = 1u128;
            let mut interior = 0;
            for j in 0..cw.saturating_sub(1) {
                carry &= (counter >> j) & 1;
                interior |= carry << j;
            }
            carries.push(interior);
            counters[a] += 1;
        }
        Ok(Self {
            read_counters: PackedBits::from_rows(&reads, cw)?,
            carry: PackedBits::from_rows(&carries, cw.saturating_sub(1))?,
            final_counters: PackedBits::from_rows(&counters, cw)?,
        })
    }
    fn digests(&self) -> [Digest; 3] {
        [
            self.read_counters.digest(),
            self.carry.digest(),
            self.final_counters.digest(),
        ]
    }
    pub fn sources<'a>(
        &'a self,
        values: &'a PackedBits,
        addresses: &'a PackedBits,
    ) -> [(Source, &'a PackedBits); 5] {
        [
            (Source::Values, values),
            (Source::Addresses, addresses),
            (Source::ReadCounters, &self.read_counters),
            (Source::Carry, &self.carry),
            (Source::FinalCounters, &self.final_counters),
        ]
    }
    fn validate(&self, s: &Statement) -> Result<(), Error> {
        for (bits, rows, width) in [
            (&self.read_counters, s.values.rows, s.counter_bits()),
            (
                &self.carry,
                s.values.rows,
                s.counter_bits().saturating_sub(1),
            ),
            (&self.final_counters, s.table.rows(), s.counter_bits()),
        ] {
            bits.validate()?;
            if bits.rows() != rows || bits.width() != width {
                return Err(Error::Statement);
            }
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    pub auxiliary_digests: [Digest; 3],
    pub permutation: permutation::Proof,
    /// Left: value,address,read counter,carry. Right: value,address,read counter,final counter.
    pub parts: [F128; 8],
    pub increments: Option<mikros::Proof>,
}
impl Proof {
    pub fn field_elements(&self) -> usize {
        self.permutation.field_elements()
            + 8
            + self.increments.as_ref().map_or(0, |p| p.field_elements())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    pub tuple_bits: usize,
    pub randomized_tuple: bool,
    pub aggregation_challenges: usize,
    pub trees: permutation::Stats,
}
#[derive(Clone, Debug)]
pub struct ProverOutput {
    pub proof: Proof,
    pub claims: Claims,
    pub auxiliary: Auxiliary,
    pub stats: Stats,
}

struct Encoding {
    basis: Vec<F128>,
    widths: [usize; 3],
    randomized: bool,
}
impl Encoding {
    fn new<C: Challenger>(s: &Statement, digests: &[Digest; 3], c: &mut C) -> Self {
        c.observe_label(b"lasso-auxiliary-oracles");
        for digest in digests {
            c.observe_bytes(digest);
        }
        c.observe_label(b"lasso-tuple-encoding");
        let width = s.tuple_bits();
        let randomized = width > 128;
        let basis = if randomized {
            // Independent E challenges, fixed only after all constituent oracles.
            // No attempt to instantiate K of degree greater than E.
            let eta = c.sample_f128_vec(width.div_ceil(128));
            (0..width)
                .map(|j| eta[j / 128] * from_raw(1u128 << (j % 128)))
                .collect()
        } else {
            SmallField::for_width(width).unwrap().basis()[..width].to_vec()
        };
        Self {
            basis,
            widths: [s.addresses.width, s.values.width, s.counter_bits()],
            randomized,
        }
    }
    fn encode(&self, address: u128, value: u128, counter: u128) -> u128 {
        let mut offset = 0;
        let mut coordinates = 0;
        let mut extension = F128::ZERO;
        for (v, width) in [address, value, counter].into_iter().zip(self.widths) {
            if self.randomized {
                for bit in 0..width {
                    if v >> bit & 1 != 0 {
                        extension += self.basis[offset + bit];
                    }
                }
            } else if width != 0 {
                coordinates ^= v << offset;
            }
            offset += width;
        }
        if self.randomized {
            raw(extension)
        } else {
            coordinates
        }
    }
    fn value(&self, address: u128, value: u128, counter: u128) -> F128 {
        let coordinates = self.encode(address, value, counter);
        if self.randomized {
            from_raw(coordinates)
        } else {
            SmallField::for_width(self.basis.len())
                .unwrap()
                .embed(coordinates)
        }
    }
    fn field_width(&self) -> usize {
        if self.randomized {
            128
        } else {
            self.basis.len()
        }
    }
}
pub(crate) fn oracles(s: &Statement, digests: &[Digest; 3]) -> Vec<(Source, OracleSpec)> {
    vec![
        (Source::Values, s.values.clone()),
        (Source::Addresses, s.addresses.clone()),
        (
            Source::ReadCounters,
            OracleSpec {
                rows: s.values.rows,
                width: s.counter_bits(),
                digest: digests[0],
            },
        ),
        (
            Source::Carry,
            OracleSpec {
                rows: s.values.rows,
                width: s.counter_bits().saturating_sub(1),
                digest: digests[1],
            },
        ),
        (
            Source::FinalCounters,
            OracleSpec {
                rows: s.table.rows(),
                width: s.counter_bits(),
                digest: digests[2],
            },
        ),
    ]
}
fn queries(s: &Statement, e: &Encoding, leaves: &[LeafClaim; 2], digests: &[Digest; 3]) -> Claims {
    let wa = s.addresses.width;
    let wv = s.values.width;
    let a = &e.basis[..wa];
    let v = &e.basis[wa..wa + wv];
    let c = &e.basis[wa + wv..];
    let carry = c.get(1..).unwrap_or(&[]);
    let mut evaluations = Vec::new();
    for (side, source, basis, offset) in [
        (0, Source::Values, v, s.table.rows()),
        (0, Source::Addresses, a, s.table.rows()),
        (0, Source::ReadCounters, c, s.table.rows()),
        (0, Source::Carry, carry, s.table.rows()),
        (1, Source::Values, v, 0),
        (1, Source::Addresses, a, 0),
        (1, Source::ReadCounters, c, 0),
        (1, Source::FinalCounters, c, s.values.rows),
    ] {
        evaluations.push(OracleClaim {
            source,
            query: LinearQuery::Rows {
                point: leaves[side].point.clone(),
                basis: basis.to_vec(),
                row_offset: offset,
            },
            value: F128::ZERO,
        });
    }
    Claims {
        oracles: oracles(s, digests),
        evaluations,
    }
}
fn check_parts(
    s: &Statement,
    e: &Encoding,
    leaves: &[LeafClaim; 2],
    parts: &[F128; 8],
) -> Result<(), Error> {
    let mut public = [F128::ZERO; 2];
    for j in 0..s.table.rows() {
        let base = e.value(j as u128, s.table.row(j), 0);
        public[0] += row_weight(&leaves[0].point, j) * base;
        public[1] += row_weight(&leaves[1].point, s.values.rows + j) * base;
    }
    if s.counter_bits() != 0 {
        let one = e.basis[s.addresses.width + s.values.width];
        for i in 0..s.values.rows {
            public[0] += row_weight(&leaves[0].point, s.table.rows() + i) * one;
        }
    }
    for side in 0..2 {
        if parts[side * 4..side * 4 + 4]
            .iter()
            .fold(public[side], |x, &y| x + y)
            != leaves[side].value
        {
            return Err(Error::Constraint);
        }
    }
    Ok(())
}
fn increment_inputs(
    s: &Statement,
    aux: &Auxiliary,
) -> Result<Option<(usize, [Vec<u8>; 3])>, Error> {
    let n = s.values.rows;
    let cw = s.counter_bits();
    if n * cw == 0 {
        return Ok(None);
    }
    let len = padded_len(n * cw)?.max(128);
    let mut inputs = [vec![0u8; len / 8], vec![0u8; len / 8], vec![0u8; len / 8]];
    for j in 0..cw {
        for i in 0..n {
            let g = j * n + i;
            let h = j == 0 || aux.carry.bit((j - 1) * n + i);
            let bit = aux.read_counters.bit(g);
            let next = j + 1 < cw && aux.carry.bit(j * n + i);
            if (h && bit) != next {
                return Err(Error::Constraint);
            }
            for (input, b) in inputs.iter_mut().zip([h, bit, next]) {
                if b {
                    input[g / 8] |= 1 << (g % 8);
                }
            }
        }
    }
    Ok(Some((len.ilog2() as usize, inputs)))
}
fn append_increment_claims(s: &Statement, claims: &mut Claims, h: mikros::Claims) {
    let n = s.values.rows;
    let known = (0..n).fold(F128::ZERO, |acc, g| {
        acc + matrix_weight(&h.a.column_weights, &h.a.row_factors, g)
    });
    for (source, matrix, offset, constant) in [
        (Source::Carry, h.a, n, known),
        (Source::ReadCounters, h.b, 0, F128::ZERO),
        (Source::Carry, h.c, 0, F128::ZERO),
    ] {
        claims.evaluations.push(OracleClaim {
            source,
            query: LinearQuery::Matrix {
                column_weights: matrix.column_weights,
                row_factors: matrix.row_factors,
                gate_offset: offset,
            },
            value: matrix.value + constant,
        });
    }
}
pub fn prove<C: Challenger>(
    s: &Statement,
    values: &PackedBits,
    addresses: &PackedBits,
    options: Options,
    c: &mut C,
) -> Result<ProverOutput, Error> {
    let aux = Auxiliary::build(s, values, addresses)?;
    prove_with_auxiliary(s, values, addresses, aux, options, c)
}
/// Explicit auxiliary entry point supports composing externally constructed,
/// committed counter oracles. The prover still checks every increment equation.
pub fn prove_with_auxiliary<C: Challenger>(
    s: &Statement,
    values: &PackedBits,
    addresses: &PackedBits,
    auxiliary: Auxiliary,
    options: Options,
    c: &mut C,
) -> Result<ProverOutput, Error> {
    s.validate()?;
    s.values.check(values)?;
    s.addresses.check(addresses)?;
    let digests = auxiliary.digests();
    prove_prebound(s, values, addresses, auxiliary, digests, options, c)
}
/// Internal composition entry point: all five roots must already commit to
/// the exact input/auxiliary bits before any tuple or product challenge.
pub(crate) fn prove_prebound<C: Challenger>(
    s: &Statement,
    values: &PackedBits,
    addresses: &PackedBits,
    auxiliary: Auxiliary,
    digests: [Digest; 3],
    options: Options,
    c: &mut C,
) -> Result<ProverOutput, Error> {
    s.validate()?;
    s.values.check_shape(values)?;
    s.addresses.check_shape(addresses)?;
    auxiliary.validate(s)?;
    options.validate()?;
    let inputs = increment_inputs(s, &auxiliary)?;
    s.bind(c)?;
    let encoding = Encoding::new(s, &digests, c);
    let n = values.rows();
    let m = s.table.rows();
    let mut left = Vec::with_capacity(n + m);
    let mut right = Vec::with_capacity(n + m);
    for j in 0..m {
        left.push(encoding.encode(j as u128, s.table.row(j), 0));
    }
    for i in 0..n {
        let a = addresses.row(i);
        let v = values.row(i);
        let read = auxiliary.read_counters.row(i);
        let write = read ^ ((auxiliary.carry.row(i) << 1) | 1);
        left.push(encoding.encode(a, v, write));
        right.push(encoding.encode(a, v, read));
    }
    for j in 0..m {
        right.push(encoding.encode(j as u128, s.table.row(j), auxiliary.final_counters.row(j)));
    }
    let (permutation, leaves, trees) =
        permutation::prove_encoded(&left, &right, encoding.field_width(), options, c)?;
    let mut claims = queries(s, &encoding, &leaves, &digests);
    let sources = auxiliary.sources(values, addresses);
    let mut parts = [F128::ZERO; 8];
    for (claim, part) in claims.evaluations.iter_mut().zip(&mut parts) {
        let bits = sources
            .iter()
            .find(|(source, _)| *source == claim.source)
            .unwrap()
            .1;
        *part = claim.query.evaluate(bits)?;
        claim.value = *part;
    }
    check_parts(s, &encoding, &leaves, &parts)?;
    c.observe_label(b"lasso-source-parts");
    c.observe_f128_slice(&parts);
    let increments = if let Some((log_n, [a, b, d])) = inputs {
        let (proof, h) = mikros::prove(&a, &b, &d, log_n, mikros::Kernel::Lookup, c)
            .map_err(|_| Error::Constraint)?;
        append_increment_claims(s, &mut claims, h);
        Some(proof)
    } else {
        None
    };
    Ok(ProverOutput {
        proof: Proof {
            auxiliary_digests: digests,
            permutation,
            parts,
            increments,
        },
        claims,
        auxiliary,
        stats: Stats {
            tuple_bits: s.tuple_bits(),
            randomized_tuple: encoding.randomized,
            aggregation_challenges: if encoding.randomized {
                s.tuple_bits().div_ceil(128)
            } else {
                0
            },
            trees,
        },
    })
}
pub fn verify<C: Challenger>(s: &Statement, proof: &Proof, c: &mut C) -> Result<Claims, Error> {
    s.bind(c)?;
    let encoding = Encoding::new(s, &proof.auxiliary_digests, c);
    let count = s.values.rows * s.counter_bits();
    if proof.increments.is_some() != (count != 0) {
        return Err(Error::ProofShape);
    }
    let leaves =
        permutation::verify_encoded(&proof.permutation, s.table.rows() + s.values.rows, c)?;
    check_parts(s, &encoding, &leaves, &proof.parts)?;
    let mut claims = queries(s, &encoding, &leaves, &proof.auxiliary_digests);
    for (claim, value) in claims.evaluations.iter_mut().zip(proof.parts) {
        claim.value = value;
    }
    c.observe_label(b"lasso-source-parts");
    c.observe_f128_slice(&proof.parts);
    if let Some(proof) = &proof.increments {
        let log_n = padded_len(count)?.max(128).ilog2() as usize;
        let h = mikros::verify(proof, log_n, c).map_err(|_| Error::Constraint)?;
        append_increment_claims(s, &mut claims, h);
    }
    Ok(claims)
}
