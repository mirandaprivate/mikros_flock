//! Deferred linear evaluations of the original Boolean oracles.
use crate::{Digest, Error, MAX_LOG_ROWS, bits::PackedBits};
use flock_field::F128;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Source {
    Left,
    Right,
    Values,
    Addresses,
    ReadCounters,
    Carry,
    FinalCounters,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OracleSpec {
    pub rows: usize,
    pub width: usize,
    pub digest: Digest,
}
impl OracleSpec {
    pub fn from_bits(bits: &PackedBits) -> Result<Self, Error> {
        bits.validate()?;
        Ok(Self {
            rows: bits.rows(),
            width: bits.width(),
            digest: bits.digest(),
        })
    }
    pub fn validate(&self) -> Result<(), Error> {
        if self.width > 128 || self.rows > 1usize << MAX_LOG_ROWS {
            return Err(Error::Statement);
        }
        Ok(())
    }
    pub fn check(&self, bits: &PackedBits) -> Result<(), Error> {
        if *self != Self::from_bits(bits)? {
            return Err(Error::Source);
        }
        Ok(())
    }
    pub(crate) fn check_shape(&self, bits: &PackedBits) -> Result<(), Error> {
        self.validate()?;
        bits.validate()?;
        if self.rows != bits.rows() || self.width != bits.width() {
            return Err(Error::Source);
        }
        Ok(())
    }
}

/// Point coordinates are low-to-high; matrix row factors are high-to-low,
/// matching Mikros. Offsets map an original oracle into a concatenated vector.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinearQuery {
    Rows {
        point: Vec<F128>,
        basis: Vec<F128>,
        row_offset: usize,
    },
    Matrix {
        column_weights: Vec<F128>,
        row_factors: Vec<[F128; 2]>,
        gate_offset: usize,
    },
}
pub fn row_weight(point: &[F128], row: usize) -> F128 {
    point.iter().enumerate().fold(F128::ONE, |w, (j, &r)| {
        w * if row >> j & 1 == 1 { r } else { F128::ONE + r }
    })
}
pub(crate) fn matrix_weight(columns: &[F128], factors: &[[F128; 2]], gate: usize) -> F128 {
    let row = gate / 128;
    factors
        .iter()
        .enumerate()
        .fold(columns[gate % 128], |w, (j, factor)| {
            w * factor[(row >> (factors.len() - 1 - j)) & 1]
        })
}
impl LinearQuery {
    /// Public coefficients in the committed, column-major bit-vector order.
    pub fn weights(&self, spec: &OracleSpec, length: usize) -> Result<Vec<F128>, Error> {
        spec.validate()?;
        let logical = spec.rows.checked_mul(spec.width).ok_or(Error::Length)?;
        if !length.is_power_of_two() || logical > length {
            return Err(Error::Length);
        }
        let mut out = vec![F128::ZERO; length];
        match self {
            Self::Rows {
                point,
                basis,
                row_offset,
            } => {
                if point.len() > MAX_LOG_ROWS
                    || basis.len() != spec.width
                    || row_offset.checked_add(spec.rows).ok_or(Error::Length)?
                        > 1usize << point.len()
                {
                    return Err(Error::ProofShape);
                }
                for b in 0..spec.width {
                    for row in 0..spec.rows {
                        out[b * spec.rows + row] = basis[b] * row_weight(point, row_offset + row);
                    }
                }
            }
            Self::Matrix {
                column_weights,
                row_factors,
                gate_offset,
            } => {
                if column_weights.len() != 128
                    || row_factors.len() > MAX_LOG_ROWS
                    || gate_offset.checked_add(logical).ok_or(Error::Length)?
                        > (1usize << row_factors.len()) * 128
                {
                    return Err(Error::ProofShape);
                }
                for (bit, v) in out[..logical].iter_mut().enumerate() {
                    *v = matrix_weight(column_weights, row_factors, gate_offset + bit);
                }
            }
        }
        Ok(out)
    }
    pub fn evaluate(&self, bits: &PackedBits) -> Result<F128, Error> {
        bits.validate()?;
        let mut result = F128::ZERO;
        match self {
            Self::Rows {
                point,
                basis,
                row_offset,
            } => {
                if point.len() > MAX_LOG_ROWS
                    || basis.len() != bits.width()
                    || row_offset.checked_add(bits.rows()).ok_or(Error::Length)?
                        > 1usize << point.len()
                {
                    return Err(Error::ProofShape);
                }
                for row in 0..bits.rows() {
                    let value = (0..bits.width()).fold(F128::ZERO, |v, b| {
                        if bits.bit(b * bits.rows() + row) {
                            v + basis[b]
                        } else {
                            v
                        }
                    });
                    result += row_weight(point, row_offset + row) * value;
                }
            }
            Self::Matrix {
                column_weights,
                row_factors,
                gate_offset,
            } => {
                if column_weights.len() != 128
                    || row_factors.len() > MAX_LOG_ROWS
                    || gate_offset
                        .checked_add(bits.bit_len())
                        .ok_or(Error::Length)?
                        > (1usize << row_factors.len()) * 128
                {
                    return Err(Error::ProofShape);
                }
                for bit in 0..bits.bit_len() {
                    if bits.bit(bit) {
                        result += matrix_weight(column_weights, row_factors, gate_offset + bit);
                    }
                }
            }
        }
        Ok(result)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OracleClaim {
    pub source: Source,
    pub query: LinearQuery,
    pub value: F128,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claims {
    pub oracles: Vec<(Source, OracleSpec)>,
    pub evaluations: Vec<OracleClaim>,
}
impl Claims {
    /// Evaluate all claims directly on their source oracles in linear time,
    /// including auxiliary witnesses.
    pub fn check(&self, sources: &[(Source, &PackedBits)]) -> Result<(), Error> {
        for (source, spec) in &self.oracles {
            let bits = sources
                .iter()
                .find(|(s, _)| s == source)
                .ok_or(Error::Source)?
                .1;
            spec.check(bits)?;
        }
        for claim in &self.evaluations {
            if !self.oracles.iter().any(|(s, _)| *s == claim.source) {
                return Err(Error::Source);
            }
            let bits = sources
                .iter()
                .find(|(s, _)| *s == claim.source)
                .ok_or(Error::Source)?
                .1;
            if claim.query.evaluate(bits)? != claim.value {
                return Err(Error::Source);
            }
        }
        Ok(())
    }
}
