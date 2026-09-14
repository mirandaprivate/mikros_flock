//! Direct calls into `../../mikros/mikros/src/baseline.rs`, using that checkout's
//! field arithmetic, factored equality weights, folds and native sumcheck.
//! Packed F2 sources are expanded to BinaryField8 inside [`prove`]. A GF(2)
//! working field cannot provide the four distinct native interpolation nodes.
//! This wraps the two-input weighted product reduction with the C(z) opening
//! required for three-input Hadamard zerocheck. No PCS is provided.

use mikros_experiments::baseline::{self as native, BaselineProof, NativeSumcheck};
use mikros_experiments::common::WitnessSide;
use p3_binary_field::{BinaryChallenger, BinaryField8, BinaryField128, TowerLevel};
use p3_challenger::{CanObserve, FieldChallenger, HashChallenger};
use p3_field::PrimeCharacteristicRing;
use p3_multilinear_util::{point::Point, poly::Poly};
use p3_sumcheck::generic_degree::GenericDegreeProof;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Result, Statement, bit, validate_inputs};

pub type Base = BinaryField8;
pub type Extension = BinaryField128;
pub type Challenger = BinaryChallenger<Base, HashChallenger<u8, p3_blake3::Blake3, 32>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Method {
    SmallFieldFirst,
    FullExtension,
}

/// Native proof fields, unchanged; claimed_sum is the C(z) opening.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proof {
    pub sumcheck: GenericDegreeProof<Base, Extension>,
    pub final_a: Extension,
    pub final_b: Extension,
}

impl Proof {
    pub fn field_elements(&self) -> usize {
        self.sumcheck
            .round_polys
            .iter()
            .map(Vec::len)
            .sum::<usize>()
            + 3
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claims {
    /// Native Plonky3 order: coordinate zero is the highest input-index bit.
    pub ab_point: Vec<Extension>,
    pub c_point: Vec<Extension>,
    pub values: [Extension; 3],
}

pub fn challenger(domain: &[u8]) -> Challenger {
    Challenger::from_hasher(domain.to_vec(), p3_blake3::Blake3)
}

fn start(statement: &Statement, ch: &mut Challenger) -> Result<Vec<Extension>> {
    statement.validate()?;
    for byte in b"hadamard-baselines/plonky3-binary-degree-two/v1"
        .iter()
        .copied()
        .chain((statement.log_n as u64).to_le_bytes())
        .chain(statement.input_digest)
    {
        ch.observe(Base::from_repr(byte));
    }
    // Storage schedule does not change the transcript, including the control.
    Ok((0..statement.log_n)
        .map(|_| ch.sample_algebra_element())
        .collect())
}

fn unpack(input: &[u8], n: usize) -> Vec<Base> {
    let element = |i| Base::from_bool(bit(input, i));
    if n >= 4096 {
        (0..n).into_par_iter().map(element).collect()
    } else {
        (0..n).map(element).collect()
    }
}

pub fn prove(
    statement: &Statement,
    a: &[u8],
    b: &[u8],
    c: &[u8],
    method: Method,
    ch: &mut Challenger,
) -> Result<Proof> {
    let n = validate_inputs(statement, a, b, c)?;
    let z = start(statement, ch)?;
    // C evaluation also uses the actual Plonky3 implementation. Release its
    // byte-field buffer before expanding A/B and entering the native prover.
    let c_eval = Poly::new(unpack(c, n)).eval_base(&Point::new(z.clone()));
    let a = unpack(a, n);
    let b = unpack(b, n);
    let proof = match method {
        Method::SmallFieldFirst => native::prove_hadamard(&a, &b, &z, c_eval, ch)?,
        Method::FullExtension => native::prove_hadamard_full_extension(&a, &b, &z, c_eval, ch)?,
    };
    let NativeSumcheck::Cubic(sumcheck) = proof.sumcheck else {
        return Err("native Hadamard prover returned a non-cubic proof".into());
    };
    Ok(Proof {
        sumcheck,
        final_a: proof.final_a,
        final_b: proof.final_b,
    })
}

pub fn verify(statement: &Statement, proof: &Proof, ch: &mut Challenger) -> Result<Claims> {
    let n = statement.validate()?;
    // Validate before cloning the small transcript into the upstream type.
    if proof.sumcheck.round_polys.len() != statement.log_n
        || proof.sumcheck.round_polys.iter().any(|p| p.len() != 3)
        || !proof.sumcheck.pow_witnesses.is_empty()
    {
        return Err("invalid native cubic proof shape or unexpected PoW".into());
    }
    let z = start(statement, ch)?;
    let native_proof = BaselineProof {
        sumcheck: NativeSumcheck::Cubic(proof.sumcheck.clone()),
        final_a: proof.final_a,
        final_b: proof.final_b,
    };
    let claims = native::verify_hadamard(&native_proof, n, &z, proof.sumcheck.claimed_sum, ch)?;
    if claims.len() != 2
        || claims[0].side != WitnessSide::A
        || claims[1].side != WitnessSide::B
        || claims[0].factors != claims[1].factors
        || claims[0].factors.len() != statement.log_n
        || claims[0]
            .factors
            .iter()
            .any(|p| p[0] + p[1] != Extension::ONE)
    {
        return Err("unexpected native Hadamard terminal claims".into());
    }
    Ok(Claims {
        ab_point: claims[0].factors.iter().map(|p| p[1]).collect(),
        c_point: z,
        values: [claims[0].value, claims[1].value, proof.sumcheck.claimed_sum],
    })
}

/// Independent bit-source oracle, outside the timed native reduction. Uses
/// direct tensor products in bounded tiles; no native folds or equality tables.
pub fn evaluate_packed(input: &[u8], point: &[Extension]) -> Result<Extension> {
    if point.len() > 30 || input.len() != (1usize << point.len()).div_ceil(8) {
        return Err("invalid native packed evaluation shape".into());
    }
    let low_bits = point.len().min(8);
    let split = point.len() - low_bits;
    let weight = |j: usize, coords: &[Extension]| {
        coords
            .iter()
            .rev()
            .enumerate()
            .fold(Extension::ONE, |w, (i, &r)| {
                w * if j & (1 << i) == 0 {
                    Extension::ONE - r
                } else {
                    r
                }
            })
    };
    let low: Vec<_> = (0..1usize << low_bits)
        .map(|j| weight(j, &point[split..]))
        .collect();
    let blocks = 1usize << split;
    let block = |j| {
        let mut sum = Extension::ZERO;
        for (k, &w) in low.iter().enumerate() {
            if bit(input, j * low.len() + k) {
                sum += w;
            }
        }
        sum * weight(j, &point[..split])
    };
    Ok(if blocks >= 256 {
        (0..blocks).into_par_iter().map(block).sum()
    } else {
        (0..blocks).map(block).sum()
    })
}

pub fn check_claims(
    statement: &Statement,
    a: &[u8],
    b: &[u8],
    c: &[u8],
    claims: &Claims,
) -> Result<()> {
    validate_inputs(statement, a, b, c)?;
    if claims.ab_point.len() != statement.log_n || claims.c_point.len() != statement.log_n {
        return Err("incorrect native source opening dimension".into());
    }
    if Statement::from_inputs(statement.log_n, a, b, c)? != *statement {
        return Err("source digest does not match the statement".into());
    }
    for (i, (input, point)) in [
        (a, &claims.ab_point),
        (b, &claims.ab_point),
        (c, &claims.c_point),
    ]
    .into_iter()
    .enumerate()
    {
        if evaluate_packed(input, point)? != claims.values[i] {
            return Err(format!(
                "native source {} opening disagrees with original input",
                ["A", "B", "C"][i]
            ));
        }
    }
    Ok(())
}
