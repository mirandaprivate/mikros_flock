//! Univariate Hadamard proof from `latex/4-block.tex` (2026-09-12).
//!
//! This stops at the section's direct packed-opening interface. The additional
//! column-to-tensor aggregation in alg-hadamard and PCS are not included.
//! Inputs are packed bits; residual degree <=254, quotient degree <=126.

use crate::{
    inner_product::{self, InnerProductProof},
    layout::Shape,
    univariate::{self, KAPPA, Kernel, QUOTIENTS, domain::domain, field::field},
};
use flock_field::F128;
use flock_multilinear::{IndexOrder, eq_table};
use flock_transcript::challenger::Challenger;
use serde::{Deserialize, Serialize};
use std::time::Instant;

// Query representation and independent source oracle are protocol-independent.
pub use crate::mixed_tensor::{Claims, Error, MatrixClaim, check_claims};

/// PIOP messages excluding deferred authentication. Quotient coefficients are
/// in ascending monomial order, h[0] through h[126].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    pub projected_c: F128,
    pub column_claims: Vec<F128>,
    pub quotients: Vec<F128>,
    pub inner_product: InnerProductProof,
}
impl Proof {
    pub fn field_elements(&self) -> usize {
        1 + self.column_claims.len()
            + self.quotients.len()
            + 2 * self.inner_product.rounds.len()
            + 2
    }
}

/// Reusable capacity only; every challenge-dependent value is overwritten.
#[derive(Default)]
pub struct ProverScratch {
    row_weights: Vec<F128>,
    inner_product: inner_product::ProverScratch,
}
#[derive(Clone, Debug, Default, Serialize)]
pub struct ProverProfile {
    pub setup_ms: f64,
    pub small_field_aggregate_ms: f64,
    pub interpolate_quotient_ms: f64,
    pub c_pack_project_ms: f64,
    pub transcript_challenge_ms: f64,
    pub ab_pack_ms: f64,
    pub inner_product_ms: f64,
    pub terminal_claims_ms: f64,
}

fn checkpoint<const PROFILE: bool>(last: &mut Option<Instant>) -> f64 {
    if PROFILE {
        let now = Instant::now();
        let elapsed = now.duration_since(last.expect("profile clock initialized"));
        *last = Some(now);
        elapsed.as_secs_f64() * 1000.0
    } else {
        0.0
    }
}
fn weights(point: &[F128]) -> Vec<F128> {
    eq_table(point, F128::ONE, IndexOrder::HighToLow)
}
fn factors(point: &[F128]) -> Vec<[F128; 2]> {
    point.iter().map(|&x| [F128::ONE + x, x]).collect()
}
fn start<C: Challenger>(shape: Shape, challenger: &mut C) -> (Vec<F128>, Vec<F128>) {
    challenger.observe_label(b"mikros-binary-hadamard-univariate-gf256-v1");
    challenger.observe_bytes(&(shape.log_n as u64).to_le_bytes());
    challenger.observe_bytes(&(KAPPA as u64).to_le_bytes());
    (
        challenger.sample_f128_vec(shape.row_log),
        challenger.sample_f128_vec(7),
    )
}
fn observe_certificate<C: Challenger>(proof: &Proof, challenger: &mut C) {
    challenger.observe_label(b"mikros-univariate-column-and-quotient-v1");
    challenger.observe_f128(proof.projected_c);
    challenger.observe_f128_slice(&proof.column_claims);
    challenger.observe_f128_slice(&proof.quotients);
}
fn sample_outside_h<C: Challenger>(challenger: &mut C) -> F128 {
    loop {
        let x = challenger.sample_f128();
        if !domain().contains(x) {
            return x;
        }
    }
}
fn claims_from_points(
    proof: &Proof,
    row_point: &[F128],
    column_point: &[F128],
    rho: Vec<F128>,
    y: &[F128],
) -> Claims {
    let l = factors(row_point);
    let chi = factors(y);
    let weighted_chi = l
        .iter()
        .zip(&chi)
        .map(|(l, y)| [l[0] * y[0], l[1] * y[1]])
        .collect();
    Claims {
        a: MatrixClaim {
            column_weights: rho.clone(),
            row_factors: chi,
            value: proof.inner_product.final_a,
        },
        b: MatrixClaim {
            column_weights: rho,
            row_factors: weighted_chi,
            value: proof.inner_product.final_b,
        },
        c: MatrixClaim {
            column_weights: weights(column_point),
            row_factors: l,
            value: proof.projected_c,
        },
    }
}

/// Reduce A o B = C. Bind the statement/commitment roots before entry and
/// authenticate all three returned matrix claims after verifier replay.
pub fn prove<C: Challenger>(
    a: &[u8],
    b: &[u8],
    c: &[u8],
    log_n: usize,
    kernel: Kernel,
    challenger: &mut C,
) -> Result<(Proof, Claims), Error> {
    prove_with_scratch(
        a,
        b,
        c,
        log_n,
        kernel,
        challenger,
        &mut ProverScratch::default(),
    )
}
pub fn prove_with_scratch<C: Challenger>(
    a: &[u8],
    b: &[u8],
    c: &[u8],
    log_n: usize,
    kernel: Kernel,
    challenger: &mut C,
    scratch: &mut ProverScratch,
) -> Result<(Proof, Claims), Error> {
    prove_impl::<C, false>(a, b, c, log_n, kernel, challenger, scratch)
        .map(|(proof, claims, _)| (proof, claims))
}
pub fn prove_profiled<C: Challenger>(
    a: &[u8],
    b: &[u8],
    c: &[u8],
    log_n: usize,
    kernel: Kernel,
    challenger: &mut C,
) -> Result<(Proof, Claims, ProverProfile), Error> {
    prove_profiled_with_scratch(
        a,
        b,
        c,
        log_n,
        kernel,
        challenger,
        &mut ProverScratch::default(),
    )
}
pub fn prove_profiled_with_scratch<C: Challenger>(
    a: &[u8],
    b: &[u8],
    c: &[u8],
    log_n: usize,
    kernel: Kernel,
    challenger: &mut C,
    scratch: &mut ProverScratch,
) -> Result<(Proof, Claims, ProverProfile), Error> {
    prove_impl::<C, true>(a, b, c, log_n, kernel, challenger, scratch)
}
fn prove_impl<C: Challenger, const PROFILE: bool>(
    a: &[u8],
    b: &[u8],
    c: &[u8],
    log_n: usize,
    kernel: Kernel,
    challenger: &mut C,
    scratch: &mut ProverScratch,
) -> Result<(Proof, Claims, ProverProfile), Error> {
    let mut last = if PROFILE { Some(Instant::now()) } else { None };
    let shape = Shape::new(log_n)?;
    shape.check_inputs(a, b, c)?;
    let (row_point, column_point) = start(shape, challenger);
    crate::weights::fill(&row_point, &mut scratch.row_weights);
    let l = &scratch.row_weights;
    let r = weights(&column_point);
    // Include public table construction in the cold proof's setup timing.
    let domain = domain();
    let coefficients = kernel.is_coefficient();
    if coefficients {
        univariate::coefficient::prepare_kernel(kernel);
    }
    let setup_ms = checkpoint::<PROFILE>(&mut last);
    // Both representations start with the same 128 column claims. The last
    // 127 slots contain either continuation samples or high product coefficients.
    let evaluations = if coefficients {
        univariate::coefficient::aggregate_with_kernel(a, b, l, kernel)
    } else {
        univariate::aggregate(a, b, l, kernel)
    };
    let small_field_aggregate_ms = checkpoint::<PROFILE>(&mut last);
    let column_claims = evaluations[..KAPPA]
        .iter()
        .map(|x| field().compose(x))
        .collect();
    let quotients = if coefficients {
        univariate::coefficient::quotient(&evaluations)
    } else {
        domain.quotient(&evaluations)
    };
    let interpolate_quotient_ms = checkpoint::<PROFILE>(&mut last);
    let projected_c = crate::lookup::project_rows_native(c, &r, l);
    let c_pack_project_ms = checkpoint::<PROFILE>(&mut last);
    let mut proof = Proof {
        projected_c,
        column_claims,
        quotients,
        inner_product: InnerProductProof {
            rounds: Vec::new(),
            final_a: F128::ZERO,
            final_b: F128::ZERO,
        },
    };
    observe_certificate(&proof, challenger);
    let x = sample_outside_h(challenger);
    let rho = domain.lagrange_at(x);
    let transcript_challenge_ms = checkpoint::<PROFILE>(&mut last);
    let pack_with_message = if kernel == Kernel::RegisterPacked {
        crate::lookup::pack_pair_weighted_with_wide_message_into
    } else {
        crate::lookup::pack_pair_weighted_with_message_into
    };
    let initial_message = pack_with_message(
        a,
        b,
        &rho,
        l,
        &mut scratch.inner_product.a,
        &mut scratch.inner_product.b,
    );
    let ab_pack_ms = checkpoint::<PROFILE>(&mut last);
    let (ip, y) = inner_product::prove_native(
        &mut scratch.inner_product,
        Some(initial_message),
        challenger,
    );
    proof.inner_product = ip;
    let inner_product_ms = checkpoint::<PROFILE>(&mut last);
    let claims = claims_from_points(&proof, &row_point, &column_point, rho, &y);
    let terminal_claims_ms = checkpoint::<PROFILE>(&mut last);
    Ok((
        proof,
        claims,
        ProverProfile {
            setup_ms,
            small_field_aggregate_ms,
            interpolate_quotient_ms,
            c_pack_project_ms,
            transcript_challenge_ms,
            ab_pack_ms,
            inner_product_ms,
            terminal_claims_ms,
        },
    ))
}

pub fn verify<C: Challenger>(
    proof: &Proof,
    log_n: usize,
    challenger: &mut C,
) -> Result<Claims, Error> {
    let shape = Shape::new(log_n)?;
    if proof.column_claims.len() != KAPPA {
        return Err(Error::ProofShape("columns"));
    }
    if proof.quotients.len() != QUOTIENTS {
        return Err(Error::ProofShape("quotients"));
    }
    if proof.inner_product.rounds.len() != shape.row_log {
        return Err(Error::ProofShape("rounds"));
    }
    let (row_point, column_point) = start(shape, challenger);
    let projected = proof
        .column_claims
        .iter()
        .zip(weights(&column_point))
        .fold(F128::ZERO, |sum, (&c, r)| sum + c * r);
    if projected != proof.projected_c {
        return Err(Error::ColumnProjectionMismatch);
    }
    observe_certificate(proof, challenger);
    let x = sample_outside_h(challenger);
    let rho = domain().lagrange_at(x);
    let c_x = proof
        .column_claims
        .iter()
        .zip(&rho)
        .fold(F128::ZERO, |sum, (&c, &rho)| sum + c * rho);
    let h_x = proof
        .quotients
        .iter()
        .rev()
        .fold(F128::ZERO, |value, &coefficient| value * x + coefficient);
    let ip_claim = c_x + domain().vanishing_at(x) * h_x;
    let y = inner_product::verify(&proof.inner_product, ip_claim, challenger)?;
    Ok(claims_from_points(
        proof,
        &row_point,
        &column_point,
        rho,
        &y,
    ))
}
