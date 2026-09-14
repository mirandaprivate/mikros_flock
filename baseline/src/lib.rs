//! Binary three-oracle `A * B - C = 0` reductions.
//!
//! [`plonky3`] directly calls the sibling Plonky3 checkout's small-field-first
//! prover and verifier. Its native binary tower fields and transcript are kept.
//! Dao--Thaler specializes the low seven equality coordinates to a canonical
//! tower basis and constructs their weights using Algorithms 2 and 3.
//!
//! The root API retains the GHASH Dao--Thaler adapter and explicitly named
//! GHASH controls. The degree-two relation yields a cubic weighted sumcheck.
//! Verification returns three unauthenticated source evaluations. Bind actual
//! commitments before calling the API; [`Statement::from_inputs`] and
//! [`check_claims`] supply a public-input testing alternative only.

pub mod dao_thaler;
mod engine;
pub mod plonky3;

use flock_field::F128;
use flock_transcript::challenger::Challenger;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, String>;

/// The generic/full-extension methods are differential controls, with the
/// same transcript as their optimized counterpart.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    GhashSmallFieldFirst,
    GhashFullExtension,
    DaoThaler,
    DaoThalerGeneric,
}

impl Method {
    pub fn fixed(self) -> bool {
        matches!(self, Self::DaoThaler | Self::DaoThalerGeneric)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Statement {
    /// Input index bit i is coordinate i; folds proceed high to low.
    pub log_n: usize,
    /// Caller-bound source identifier, not a polynomial commitment.
    pub input_digest: [u8; 32],
}

impl Statement {
    pub fn validate(&self) -> Result<usize> {
        if self.log_n > 30 || self.log_n >= usize::BITS as usize {
            return Err("expected 0 <= log_n <= 30 and a representable input size".into());
        }
        Ok(1usize << self.log_n)
    }

    pub fn from_inputs(log_n: usize, a: &[u8], b: &[u8], c: &[u8]) -> Result<Self> {
        let mut statement = Self {
            log_n,
            input_digest: [0; 32],
        };
        validate_inputs(&statement, a, b, c)?;
        let mut hash = blake3::Hasher::new();
        hash.update(b"hadamard-baselines/public-binary-inputs/v1");
        hash.update(&(log_n as u64).to_le_bytes());
        for input in [a, b, c] {
            hash.update(&(input.len() as u64).to_le_bytes());
            hash.update(input);
        }
        statement.input_digest = *hash.finalize().as_bytes();
        Ok(statement)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    /// C evaluated at the initial equality point.
    pub c_eval: F128,
    /// Cubic coefficients [constant, quadratic, cubic]. The linear coefficient
    /// is recovered as claim + quadratic + cubic in characteristic two.
    pub rounds: Vec<[F128; 3]>,
    pub final_a: F128,
    pub final_b: F128,
}

impl Proof {
    pub fn field_elements(&self) -> usize {
        3 * self.rounds.len() + 3
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claims {
    /// A and B at the terminal sumcheck point, in LSB-first coordinate order.
    pub ab_point: Vec<F128>,
    /// C at the initial (possibly specialized) equality point.
    pub c_point: Vec<F128>,
    pub values: [F128; 3],
}

pub fn prove(
    statement: &Statement,
    a: &[u8],
    b: &[u8],
    c: &[u8],
    method: Method,
    challenger: &mut impl Challenger,
) -> Result<(Proof, Claims)> {
    validate_inputs(statement, a, b, c)?;
    let z = start(statement, method, challenger)?;
    engine::prove(statement, a, b, c, method, z, challenger)
}

pub fn verify(
    statement: &Statement,
    method: Method,
    proof: &Proof,
    challenger: &mut impl Challenger,
) -> Result<Claims> {
    statement.validate()?;
    if proof.rounds.len() != statement.log_n {
        return Err("incorrect number of sumcheck rounds".into());
    }
    let z = start(statement, method, challenger)?;
    challenger.observe_f128(proof.c_eval);
    let mut claim = proof.c_eval;
    let mut point = vec![F128::ZERO; statement.log_n];
    let mut bound_eq = F128::ONE;
    for (round, &message) in proof.rounds.iter().enumerate() {
        challenger.observe_f128_slice(&message);
        let r = challenger.sample_f128();
        let i = statement.log_n - 1 - round;
        point[i] = r;
        claim = evaluate_round(message, claim, r);
        bound_eq *= F128::ONE + z[i] + r;
    }
    challenger.observe_f128_slice(&[proof.final_a, proof.final_b]);
    if claim != bound_eq * proof.final_a * proof.final_b {
        return Err("sumcheck terminal product check failed".into());
    }
    Ok(Claims {
        ab_point: point,
        c_point: z,
        values: [proof.final_a, proof.final_b, proof.c_eval],
    })
}

fn start(statement: &Statement, method: Method, ch: &mut impl Challenger) -> Result<Vec<F128>> {
    statement.validate()?;
    ch.observe_label(b"hadamard-baselines/binary-degree-two/v1");
    ch.observe_bytes(&(statement.log_n as u64).to_le_bytes());
    ch.observe_bytes(&statement.input_digest);
    ch.observe_bytes(&[u8::from(method.fixed())]);
    let fixed = if method.fixed() {
        statement.log_n.min(7)
    } else {
        0
    };
    Ok((0..statement.log_n)
        .map(|i| {
            if i < fixed {
                dao_thaler::generators()[i]
            } else {
                ch.sample_f128()
            }
        })
        .collect())
}

fn evaluate_round([c0, c2, c3]: [F128; 3], claim: F128, r: F128) -> F128 {
    ((c3 * r + c2) * r + claim + c2 + c3) * r + c0
}

fn validate_inputs(statement: &Statement, a: &[u8], b: &[u8], c: &[u8]) -> Result<usize> {
    let n = statement.validate()?;
    if [a.len(), b.len(), c.len()] != [n.div_ceil(8); 3] {
        return Err("packed source lengths do not match the statement".into());
    }
    if n < 8 && (a[0] | b[0] | c[0]) >> n != 0 {
        return Err("unused high bits must be zero".into());
    }
    Ok(n)
}

#[inline]
pub(crate) fn bit(input: &[u8], i: usize) -> bool {
    input[i / 8] & (1 << (i % 8)) != 0
}

/// Direct input authentication for tests/public inputs, outside prover and
/// verifier timings. This does not use the prover's equality table or folds.
pub fn check_claims(
    statement: &Statement,
    a: &[u8],
    b: &[u8],
    c: &[u8],
    claims: &Claims,
) -> Result<()> {
    validate_inputs(statement, a, b, c)?;
    if claims.ab_point.len() != statement.log_n || claims.c_point.len() != statement.log_n {
        return Err("incorrect source opening dimension".into());
    }
    if Statement::from_inputs(statement.log_n, a, b, c)? != *statement {
        return Err("source digest does not match the statement".into());
    }
    for (index, (input, point)) in [
        (a, &claims.ab_point),
        (b, &claims.ab_point),
        (c, &claims.c_point),
    ]
    .into_iter()
    .enumerate()
    {
        if evaluate_packed(input, point)? != claims.values[index] {
            return Err(format!(
                "source {} opening disagrees with the original input",
                ["A", "B", "C"][index]
            ));
        }
    }
    Ok(())
}

/// Independent bounded-memory multilinear evaluation; accepts LSB-first bits.
pub fn evaluate_packed(input: &[u8], point: &[F128]) -> Result<F128> {
    if point.len() > 30 || input.len() != (1usize << point.len()).div_ceil(8) {
        return Err("invalid packed multilinear evaluation shape".into());
    }
    let low_bits = point.len().min(8);
    let width = 1usize << low_bits;
    let low: Vec<_> = (0..width)
        .map(|j| {
            point[..low_bits]
                .iter()
                .enumerate()
                .fold(F128::ONE, |w, (i, &r)| {
                    w * if j & (1 << i) == 0 { F128::ONE + r } else { r }
                })
        })
        .collect();
    let blocks = 1usize << (point.len() - low_bits);
    let block = |j: usize| {
        let high = point[low_bits..]
            .iter()
            .enumerate()
            .fold(F128::ONE, |w, (i, &r)| {
                w * if j & (1 << i) == 0 { F128::ONE + r } else { r }
            });
        let mut sum = F128::ZERO;
        for (k, &w) in low.iter().enumerate() {
            if bit(input, j * width + k) {
                sum += w;
            }
        }
        sum * high
    };
    Ok(if blocks >= 256 && rayon::current_num_threads() > 1 {
        (0..blocks)
            .into_par_iter()
            .map(block)
            .reduce(|| F128::ZERO, |a, b| a + b)
    } else {
        (0..blocks).map(block).fold(F128::ZERO, |a, b| a + b)
    })
}
