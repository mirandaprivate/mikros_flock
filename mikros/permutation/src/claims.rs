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
    /// Check every source and claim directly in linear time, using split
    /// equality tables for row queries.
    pub fn check_parallel(&self, sources: &[(Source, &PackedBits)]) -> Result<(), Error> {
        for (source, spec) in &self.oracles {
            let bits = sources
                .iter()
                .find(|(s, _)| s == source)
                .ok_or(Error::Source)?
                .1;
            crate::permutation::check_source_parallel(spec, bits)?;
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
            let actual = match &claim.query {
                LinearQuery::Rows {
                    point,
                    basis,
                    row_offset: 0,
                } if point.len() >= 6
                    && point.len() <= MAX_LOG_ROWS
                    && basis.len() == bits.width()
                    && bits.rows() <= 1usize << point.len() =>
                {
                    use flock_core::zerocheck::univariate_skip::SplitEqGhash;
                    use flock_field::F256Unreduced;
                    use rayon::prelude::*;
                    let eq = SplitEqGhash::with_n_hi(point, point.len() / 2);
                    let tables = crate::field::linear_tables(basis);
                    let blocks = bits.rows().div_ceil(eq.lo.len());
                    (0..blocks)
                        .into_par_iter()
                        .map(|hi| {
                            let start = hi * eq.lo.len();
                            let end = (start + eq.lo.len()).min(bits.rows());
                            let mut sum = F256Unreduced::ZERO;
                            for first in (start..end).step_by(8) {
                                let values = crate::bits::decode_eight(bits, first / 8);
                                for r in 0..8.min(end - first) {
                                    let value = crate::field::evaluate_linear(&tables, values[r]);
                                    sum ^= value.mul_unreduced(eq.lo[first + r - start]);
                                }
                            }
                            sum.reduce() * eq.hi[hi]
                        })
                        .reduce(|| F128::ZERO, |a, b| a + b)
                }
                _ => claim.query.evaluate(bits)?,
            };
            if actual != claim.value {
                return Err(Error::Source);
            }
        }
        Ok(())
    }
    /// Linear-time reference authentication, NOT a PCS verifier. All returned
    /// oracles and claims must be authenticated, including auxiliary witnesses.
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
