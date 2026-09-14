//! The binary protocol in appx-tech.tex:38-291 and appx-proof.tex:483-515.
//! The certificate is stored in lexicographic ternary order with Boolean
//! positions removed. The first digit 2 identifies its quotient h_j; this fixed
//! ordering enforces the allowed mixed basis and the total degree bound 2s.

use std::{error, fmt, time::Instant};

use flock_field::F128;
use flock_multilinear::{IndexOrder, eq_table};
use flock_transcript::challenger::Challenger;
use rayon::prelude::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};
use serde::{Deserialize, Serialize};

use crate::{
    inner_product::{self, InnerProductProof},
    layout::Shape,
    lookup::{pack_pair_weighted, pack_rows, project_rows},
    tensor::{self, COEFFICIENTS, KAPPA, Kernel, QUOTIENTS, S},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidLogN(usize),
    InputLength {
        input: &'static str,
        expected: usize,
        got: usize,
    },
    ProofShape(&'static str),
    ColumnProjectionMismatch,
    InnerProductMismatch,
    SourceMismatch(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mikros verification error: {self:?}")
    }
}
impl error::Error for Error {}

/// PIOP messages only. Authentication of the returned input evaluations is
/// deliberately delegated to the caller, like Flock's zerocheck API.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    pub projected_c: F128,
    pub column_claims: Vec<F128>,
    pub quotients: Vec<F128>,
    pub inner_product: InnerProductProof,
}

impl Proof {
    /// Payload excluding serialization lengths and all deferred PCS messages.
    pub fn field_elements(&self) -> usize {
        1 + self.column_claims.len()
            + self.quotients.len()
            + 2 * self.inner_product.rounds.len()
            + 2
    }
}

/// `value = row_tensor^T * input_matrix * column_weights`.
/// Row factors are in HighToLow order and need not sum to one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatrixClaim {
    pub column_weights: Vec<F128>,
    pub row_factors: Vec<[F128; 2]>,
    pub value: F128,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Claims {
    pub a: MatrixClaim,
    pub b: MatrixClaim,
    pub c: MatrixClaim,
}

/// Optional reusable allocation capacity. Every witness- and challenge-dependent
/// value is overwritten inside prove; this contains no reusable proof state.
#[derive(Default)]
pub struct ProverScratch {
    row_weights: Vec<F128>,
    inner_product: inner_product::ProverScratch,
}

/// Wall-clock milliseconds inside the actual prover, in execution order.
/// Profiling never changes messages or excludes setup from the total timing.
#[derive(Clone, Debug, Default, Serialize)]
pub struct ProverProfile {
    pub setup_ms: f64,
    pub tensor_aggregate_ms: f64,
    pub certificate_split_ms: f64,
    pub c_pack_project_ms: f64,
    pub transcript_challenge_ms: f64,
    /// Packed kernel includes B weighting here because packing and weighting
    /// are fused; its separate b_weight_ms is only the empty phase boundary.
    pub ab_pack_ms: f64,
    pub b_weight_ms: f64,
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
    challenger.observe_label(b"mikros-binary-hadamard-mixed-tensor-v1");
    challenger.observe_bytes(&(shape.log_n as u64).to_le_bytes());
    challenger.observe_bytes(&(KAPPA as u64).to_le_bytes());
    (
        challenger.sample_f128_vec(shape.row_log),
        challenger.sample_f128_vec(S),
    )
}

fn observe_certificate<C: Challenger>(proof: &Proof, challenger: &mut C) {
    challenger.observe_label(b"mikros-column-and-quotient-claims-v1");
    challenger.observe_f128(proof.projected_c);
    challenger.observe_f128_slice(&proof.column_claims);
    challenger.observe_f128_slice(&proof.quotients);
}

fn claims_from_points(
    proof: &Proof,
    row_point: &[F128],
    column_point: &[F128],
    x: &[F128],
    y: &[F128],
) -> Claims {
    let l = factors(row_point);
    let chi = factors(y);
    let rho = weights(x);
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

/// Reduce A o B = C for packed binary vectors, with kappa=128.
///
/// The caller binds the public statement/commitment roots to `challenger`
/// before entry. This function does not scan to reject a false relation:
/// a false input must fail verifier replay or source-claim authentication.
pub fn prove<C: Challenger>(
    a: &[u8],
    b: &[u8],
    c: &[u8],
    log_n: usize,
    kernel: Kernel,
    challenger: &mut C,
) -> Result<(Proof, Claims), Error> {
    prove_impl::<C, false>(
        a,
        b,
        c,
        log_n,
        kernel,
        challenger,
        &mut ProverScratch::default(),
    )
    .map(|(proof, claims, _)| (proof, claims))
}

/// Run the same prover with nine phase boundaries instrumented. Use `prove`
/// for primary end-to-end measurements; its clock calls compile away.
pub fn prove_profiled<C: Challenger>(
    a: &[u8],
    b: &[u8],
    c: &[u8],
    log_n: usize,
    kernel: Kernel,
    challenger: &mut C,
) -> Result<(Proof, Claims, ProverProfile), Error> {
    prove_impl::<C, true>(
        a,
        b,
        c,
        log_n,
        kernel,
        challenger,
        &mut ProverScratch::default(),
    )
}

/// Same messages and challenges as `prove`, retaining only scratch capacity.
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

/// Instrument the same scratch-reusing prover for a separate diagnostic run.
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
    let reference_l;
    let l = if kernel.is_native() {
        crate::weights::fill(&row_point, &mut scratch.row_weights);
        &scratch.row_weights
    } else {
        reference_l = weights(&row_point);
        &reference_l
    };
    let r = weights(&column_point);
    let setup_ms = checkpoint::<PROFILE>(&mut last);
    let h = if kernel == Kernel::PackedFactored {
        tensor::aggregate_factored(a, b, &row_point)
    } else {
        tensor::aggregate(a, b, l, kernel)
    };
    let tensor_aggregate_ms = checkpoint::<PROFILE>(&mut last);
    let mut column_claims = vec![F128::ZERO; KAPPA];
    let mut quotients = Vec::with_capacity(QUOTIENTS);
    for (index, &value) in h.iter().enumerate() {
        if let Some(column) = tensor::boolean_column(index) {
            column_claims[column] = value;
        } else {
            quotients.push(value);
        }
    }
    let certificate_split_ms = checkpoint::<PROFILE>(&mut last);
    let projected_c = if kernel.has_native_packing() {
        crate::lookup::project_rows_native(c, &r, l)
    } else if kernel.is_packed() {
        project_rows(c, &r, l)
    } else {
        let c_packed = pack_rows(c, &r, kernel);
        c_packed
            .iter()
            .zip(l)
            .fold(F128::ZERO, |sum, (&c, &l)| sum + c * l)
    };
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
    let x = challenger.sample_f128_vec(S);
    let rho = weights(&x);
    let transcript_challenge_ms = checkpoint::<PROFILE>(&mut last);
    let mut initial_message = None;
    let (packed_a, mut packed_b) = if kernel.has_native_packing() {
        initial_message = Some(crate::lookup::pack_pair_weighted_with_message_into(
            a,
            b,
            &rho,
            l,
            &mut scratch.inner_product.a,
            &mut scratch.inner_product.b,
        ));
        (Vec::new(), Vec::new())
    } else if kernel.is_packed() {
        pack_pair_weighted(a, b, &rho, l)
    } else {
        (pack_rows(a, &rho, kernel), pack_rows(b, &rho, kernel))
    };
    let ab_pack_ms = checkpoint::<PROFILE>(&mut last);
    if !kernel.is_packed() && packed_b.len() >= 4096 {
        packed_b
            .par_iter_mut()
            .zip(l.par_iter())
            .for_each(|(b, &l)| *b *= l);
    } else if !kernel.is_packed() {
        packed_b.iter_mut().zip(l).for_each(|(b, &l)| *b *= l);
    }
    let b_weight_ms = checkpoint::<PROFILE>(&mut last);
    let (ip, y) = if kernel.has_native_ip() {
        if !kernel.has_native_packing() {
            scratch.inner_product.a = packed_a;
            scratch.inner_product.b = packed_b;
        }
        inner_product::prove_native(&mut scratch.inner_product, initial_message, challenger)
    } else if kernel.is_packed() {
        inner_product::prove_packed(packed_a, packed_b, challenger)
    } else {
        inner_product::prove(packed_a, packed_b, challenger)
    };
    proof.inner_product = ip;
    let inner_product_ms = checkpoint::<PROFILE>(&mut last);
    let claims = claims_from_points(&proof, &row_point, &column_point, &x, &y);
    let terminal_claims_ms = checkpoint::<PROFILE>(&mut last);
    Ok((
        proof,
        claims,
        ProverProfile {
            setup_ms,
            tensor_aggregate_ms,
            certificate_split_ms,
            c_pack_project_ms,
            transcript_challenge_ms,
            ab_pack_ms,
            b_weight_ms,
            inner_product_ms,
            terminal_claims_ms,
        },
    ))
}

/// Check all PIOP messages and return the three claims the caller must open.
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
    let r = weights(&column_point);
    let projected = proof
        .column_claims
        .iter()
        .zip(&r)
        .fold(F128::ZERO, |sum, (&c, &r)| sum + c * r);
    if projected != proof.projected_c {
        return Err(Error::ColumnProjectionMismatch);
    }
    observe_certificate(proof, challenger);
    let x = challenger.sample_f128_vec(S);
    let mixed_weights = tensor::basis_weights(&x);
    let mut quotient = 0usize;
    let mut ip_claim = F128::ZERO;
    for (index, &weight) in mixed_weights.iter().enumerate().take(COEFFICIENTS) {
        let coefficient = if let Some(column) = tensor::boolean_column(index) {
            proof.column_claims[column]
        } else {
            let coefficient = proof.quotients[quotient];
            quotient += 1;
            coefficient
        };
        ip_claim += coefficient * weight;
    }
    let y = inner_product::verify(&proof.inner_product, ip_claim, challenger)?;
    Ok(claims_from_points(proof, &row_point, &column_point, &x, &y))
}

// This intentionally does not call the prover's equality tables or packing LUT.
fn source_value(input: &[u8], query: &MatrixClaim) -> F128 {
    let mut row_weights = vec![F128::ONE];
    for factor in &query.row_factors {
        let mut expanded = Vec::with_capacity(2 * row_weights.len());
        for &weight in &row_weights {
            expanded.push(weight * factor[0]);
            expanded.push(weight * factor[1]);
        }
        row_weights = expanded;
    }
    input
        .chunks_exact(16)
        .zip(row_weights)
        .fold(F128::ZERO, |sum, (row, weight)| {
            let mut packed = F128::ZERO;
            for column in 0..KAPPA {
                if (row[column / 8] >> (column % 8)) & 1 != 0 {
                    packed += query.column_weights[column];
                }
            }
            sum + packed * weight
        })
}

/// Linear-time independent original-input oracle. This authenticates deferred
/// claims only when the caller already has the original vectors. It is not PCS.
pub fn check_claims(
    a: &[u8],
    b: &[u8],
    c: &[u8],
    log_n: usize,
    claims: &Claims,
) -> Result<(), Error> {
    let shape = Shape::new(log_n)?;
    shape.check_inputs(a, b, c)?;
    for (name, input, query) in [
        ("a", a, &claims.a),
        ("b", b, &claims.b),
        ("c", c, &claims.c),
    ] {
        if query.column_weights.len() != KAPPA || query.row_factors.len() != shape.row_log {
            return Err(Error::ProofShape("source query"));
        }
        if source_value(input, query) != query.value {
            return Err(Error::SourceMismatch(name));
        }
    }
    Ok(())
}
