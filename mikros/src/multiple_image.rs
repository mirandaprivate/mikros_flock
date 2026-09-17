//! Authenticate one virtual binary linear image through column projection.
//!
//! Implements `latex/6-together.tex`: fix the 128 column evaluations before
//! sampling a fresh coordinate projection. The committed sparse API runs two
//! ordinary IPs without exposing the sparse matrix to its verifier; its matrix
//! MLE MUST reach the caller's sparse/lookup proof. The public CSC API runs one
//! IP and recomputes its public terminal through `LincheckCircuit`.
//! Every returned witness MLE must reach the original packed PCS commitment.
//! Bind both commitments and the enclosing statement before entering here.

use crate::{
    MatrixClaim,
    inner_product::{self, InnerProductProof, ProverScratch},
};
use flock_core::{
    lincheck::LincheckCircuit,
    pcs::{
        functional::row_weights,
        ring_switch::{build_fold_byte_table, fold_1b_rows_naive, linearized_coefficients},
    },
    r1cs::SparseBinaryMatrix,
};
use flock_field::{F128, F256Unreduced};
use flock_transcript::challenger::Challenger;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

const WIDTH: usize = 128;

#[cfg(test)]
thread_local! {
    static PUBLIC_CSC_SINGLE_OVERRIDE: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
    static IMAGE_BLOCKED_WEIGHTS_OVERRIDE: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) };
}

fn image_blocked_weights() -> bool {
    #[cfg(test)]
    if let Some(value) = IMAGE_BLOCKED_WEIGHTS_OVERRIDE.with(|v| v.get()) {
        return value;
    }
    std::env::var_os("MIKROS_IMAGE_BLOCKED_WEIGHTS").is_none_or(|v| v == "1")
}

fn public_csc_single() -> bool {
    #[cfg(test)]
    if let Some(value) = PUBLIC_CSC_SINGLE_OVERRIDE.with(|v| v.get()) {
        return value;
    }
    std::env::var_os("MIKROS_PUBLIC_CSC_SINGLE").is_none_or(|v| v == "1")
}

fn profile_elapsed(start: Option<std::time::Instant>) -> f64 {
    start.map_or(0.0, |t| t.elapsed().as_secs_f64() * 1000.0)
}

struct PublicMarginal {
    selected: Vec<F128>,
    unused: Vec<F128>,
    coordinate_ms: f64,
    csc_ms: f64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PointClaim {
    /// Low-bit-first ordinary MLE point.
    pub point: Vec<F128>,
    pub value: F128,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixEvaluation {
    /// Both points are low-bit first: eq(row)^T S eq(column).
    pub row_point: Vec<F128>,
    pub column_point: Vec<F128>,
    pub value: F128,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claims {
    pub witness: PointClaim,
    pub matrix: MatrixEvaluation,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    /// u = U^T beta. Each element exposes one column's 128 base coordinates.
    pub columns: Vec<F128>,
    pub ip1: InnerProductProof,
    pub ip2: InnerProductProof,
}

/// Column projection with a public circuit. The second IP of the committed
/// sparse API is replaced by the verifier's explicit public CSC evaluation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicProof {
    pub columns: Vec<F128>,
    pub ip: InnerProductProof,
}

/// Independent column messages, followed by one IP for the random linear
/// combination of witness blocks under the same public matrix and row tensor.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicBatchProof {
    pub columns: Vec<Vec<F128>>,
    pub ip: InnerProductProof,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicBatchClaims {
    /// Ordinary LOW FIRST MLE of sum_j weights[j] * witness[j].
    pub witness: PointClaim,
    pub weights: Vec<F128>,
}

fn checked_shape(log_n: usize, source: &MatrixClaim) -> Result<usize, String> {
    if log_n >= usize::BITS as usize
        || source.row_factors.len() != log_n
        || source.column_weights.len() != WIDTH
    {
        return Err("multiple image query shape".into());
    }
    let n = 1usize << log_n;
    if n > isize::MAX as usize / size_of::<F128>() {
        return Err("multiple image dimension overflow".into());
    }
    Ok(n)
}

fn basis() -> Vec<F128> {
    (0..WIDTH)
        .map(|i| {
            if i < 64 {
                F128::new(1u64 << i, 0)
            } else {
                F128::new(0, 1u64 << (i - 64))
            }
        })
        .collect()
}

fn dot(a: &[F128], b: &[F128]) -> F128 {
    debug_assert_eq!(a.len(), b.len());
    // Column/projection checks contain exactly 128 terms. Scheduling one
    // Rayon chunk costs more than this small straight-line accumulation.
    if a.len() <= WIDTH {
        return a
            .iter()
            .zip(b)
            .fold(F256Unreduced::ZERO, |sum, (&a, &b)| {
                sum ^ a.mul_unreduced(b)
            })
            .reduce();
    }
    a.par_chunks(4096)
        .zip(b.par_chunks(4096))
        .map(|(a, b)| {
            a.iter().zip(b).fold(F256Unreduced::ZERO, |sum, (&a, &b)| {
                sum ^ a.mul_unreduced(b)
            })
        })
        .reduce(|| F256Unreduced::ZERO, |a, b| a ^ b)
        .reduce()
}

fn bind<C: Challenger>(log_n: usize, source: &MatrixClaim, ch: &mut C) {
    ch.observe_label(b"mikros-multiple-image-column-projection-v1");
    ch.observe_bytes(&(log_n as u64).to_le_bytes());
    ch.observe_bytes(&(WIDTH as u64).to_le_bytes());
    ch.observe_f128_slice(&source.column_weights);
    for pair in &source.row_factors {
        ch.observe_f128_slice(pair);
    }
    ch.observe_f128(source.value);
}

fn column_message<C: Challenger>(columns: &[F128], ch: &mut C) -> Vec<F128> {
    ch.observe_label(b"mikros-multiple-image-columns-v1");
    ch.observe_f128_slice(columns);
    // No projection challenge is sampled before ALL column messages are fixed.
    ch.sample_f128_vec(WIDTH)
}

fn projected_claim(columns: &[F128], projection: &[F128]) -> F128 {
    // Input word j carries U_{i,j} in bit i. Folding with beta_j packs U's
    // rows, exactly U beta, rather than reusing the transposed column packing.
    let packed_rows = fold_1b_rows_naive(columns, &basis());
    dot(projection, &packed_rows)
}

fn project_coordinate(value: F128, tables: &[F128]) -> F128 {
    value
        .lo
        .to_le_bytes()
        .into_iter()
        .chain(value.hi.to_le_bytes())
        .enumerate()
        .fold(F128::ZERO, |sum, (i, byte)| {
            sum + tables[i * 256 + byte as usize]
        })
}

fn tensor_high(factors: &[[F128; 2]]) -> Vec<F128> {
    if image_blocked_weights() {
        let mut out = Vec::new();
        crate::weights::fill_tensor(factors, &mut out);
        out
    } else {
        row_weights(&factors.iter().rev().copied().collect::<Vec<_>>())
    }
}

fn eq_low(point: &[F128]) -> Vec<F128> {
    row_weights(
        &point
            .iter()
            .map(|&x| [F128::ONE + x, x])
            .collect::<Vec<_>>(),
    )
}

/// Evaluate eq(point)^T Xi projection without materializing Xi or a length-N
/// tensor. The coordinate map Phi(t)=sum_j projection_j bit_j(t) is F2-linear:
/// Phi(t)=sum_h coeff_h t^(2^h). Frobenius preserves products, so each term's
/// sum over Boolean indices factors into log(N) independent two-entry sums.
/// No normalizer is inverted, including [a,a], zero pairs and 0/1 points.
fn evaluate_public_query(source: &MatrixClaim, projection: &[F128], point: &[F128]) -> F128 {
    debug_assert_eq!(source.row_factors.len(), point.len());
    let coefficients = linearized_coefficients(&build_fold_byte_table(projection));
    let mut powers: Vec<_> = source.row_factors.iter().rev().copied().collect();
    let mut result = F128::ZERO;
    for coefficient in coefficients {
        let product = powers
            .iter()
            .zip(point)
            .fold(F128::ONE, |product, (pair, &x)| {
                product * ((F128::ONE + x) * pair[0] + x * pair[1])
            });
        result += coefficient * product;
        for pair in &mut powers {
            pair[0] = pair[0].square();
            pair[1] = pair[1].square();
        }
    }
    result
}

fn sparse_apply(matrix: &SparseBinaryMatrix, values: &[F128]) -> Vec<F128> {
    matrix
        .rows
        .par_iter()
        .map(|columns| {
            columns
                .iter()
                .fold(F128::ZERO, |sum, &column| sum + values[column])
        })
        .collect()
}

fn sparse_apply_into(matrix: &SparseBinaryMatrix, values: &[F128], output: &mut Vec<F128>) {
    // After IP1, its largest A allocation is retained in scratch.a. Every
    // element is replaced here; no previous challenge-dependent value is used.
    output.resize(matrix.num_rows, F128::ZERO);
    output
        .par_iter_mut()
        .zip(matrix.rows.par_iter())
        .for_each(|(out, columns)| {
            *out = columns
                .iter()
                .fold(F128::ZERO, |sum, &column| sum + values[column]);
        });
}

fn sparse_transpose_apply(matrix: &SparseBinaryMatrix, values: &[F128]) -> Vec<F128> {
    let mut result = vec![F128::ZERO; matrix.num_cols];
    for (columns, &value) in matrix.rows.iter().zip(values) {
        for &column in columns {
            result[column] += value;
        }
    }
    result
}

fn native_ip<C: Challenger>(
    a: Vec<F128>,
    b: Vec<F128>,
    ch: &mut C,
) -> (InnerProductProof, Vec<F128>) {
    let mut scratch = ProverScratch::default();
    scratch.a = a;
    scratch.b = b;
    native_ip_in_scratch(&mut scratch, None, ch)
}

fn native_ip_in_scratch<C: Challenger>(
    scratch: &mut ProverScratch,
    initial_message: Option<[F128; 2]>,
    ch: &mut C,
) -> (InnerProductProof, Vec<F128>) {
    let (proof, mut high_point) = inner_product::prove_native(scratch, initial_message, ch);
    high_point.reverse();
    (proof, high_point)
}

fn bind_public<C: Challenger>(log_n: usize, left: bool, source: &MatrixClaim, ch: &mut C) {
    ch.observe_label(b"mikros-multiple-image-public-csc-v1");
    ch.observe_bytes(&[u8::from(left)]);
    bind(log_n, source, ch);
}

/// Xi is base-field coordinate extraction, not multiplication by the random
/// projection in F128. Reuse its allocation after sending the column message.
fn public_marginals(
    circuit: &dyn LincheckCircuit,
    xi: &mut [F128],
    projection: &[F128],
    left: bool,
    profile: bool,
) -> Result<PublicMarginal, String> {
    let start = profile.then(std::time::Instant::now);
    let table = build_fold_byte_table(projection);
    xi.par_iter_mut()
        .for_each(|x| *x = project_coordinate(*x, &table));
    let coordinate_ms = profile_elapsed(start);
    let start = profile.then(std::time::Instant::now);
    let (selected, unused) = if public_csc_single() {
        let selected = circuit.fold_side(xi, left);
        if selected.len() != xi.len() {
            return Err("multiple image public circuit output shape".into());
        }
        (selected, Vec::new())
    } else {
        let (a, b) = circuit.fold_split(xi);
        if a.len() != xi.len() || b.len() != xi.len() {
            return Err("multiple image public circuit output shape".into());
        }
        if left { (a, b) } else { (b, a) }
    };
    Ok(PublicMarginal {
        selected,
        unused,
        coordinate_ms,
        csc_ms: profile_elapsed(start),
    })
}

/// Prove one 128-instance block's image under public A (`left=true`) or B.
/// The caller binds the public circuit and packed witness commitment first,
/// and must authenticate the returned ordinary witness MLE through that PCS.
/// C=I needs no image reduction; callers route its source to the witness PCS.
pub fn prove_public<C: Challenger>(
    circuit: &dyn LincheckCircuit,
    left: bool,
    witness: &[F128],
    image: &[F128],
    source: &MatrixClaim,
    ch: &mut C,
) -> Result<(PublicProof, PointClaim), String> {
    let log_n = source.row_factors.len();
    let n = checked_shape(log_n, source)?;
    if circuit.n_cols() != n || witness.len() != n || image.len() != n {
        return Err("multiple image public circuit/input shape".into());
    }
    bind_public(log_n, left, source, ch);
    let mut xi = tensor_high(&source.row_factors);
    let columns = fold_1b_rows_naive(image, &xi);
    if dot(&columns, &source.column_weights) != source.value {
        return Err("multiple image public column scalar check".into());
    }
    let projection = column_message(&columns, ch);
    let scalar = projected_claim(&columns, &projection);
    let PublicMarginal {
        selected: rhs,
        unused,
        ..
    } = public_marginals(circuit, &mut xi, &projection, left, false)?;
    // Only allocation capacity is reused: prove_native rewrites its outputs
    // after the next challenge. No challenge-dependent value survives a proof.
    let mut scratch = ProverScratch::with_fold_buffers([xi, unused]);
    scratch.a.extend_from_slice(witness);
    scratch.b = rhs;
    let (actual, initial_message) =
        inner_product::checked_initial_message_native(&scratch.a, &scratch.b);
    if actual != scalar {
        return Err("multiple image public projected identity".into());
    }
    ch.observe_label(b"mikros-multiple-image-public-ip-v1");
    let (ip, point) = native_ip_in_scratch(&mut scratch, initial_message, ch);
    let claim = PointClaim {
        point,
        value: ip.final_a,
    };
    Ok((PublicProof { columns, ip }, claim))
}

/// Verify the projection and IP, then evaluate its public terminal using the
/// caller's bound circuit. Verification scans the public circuit and returns
/// a witness claim for PCS authentication.
pub fn verify_public<C: Challenger>(
    circuit: &dyn LincheckCircuit,
    left: bool,
    source: &MatrixClaim,
    proof: &PublicProof,
    ch: &mut C,
) -> Result<PointClaim, String> {
    let log_n = source.row_factors.len();
    let n = checked_shape(log_n, source)?;
    if circuit.n_cols() != n || proof.columns.len() != WIDTH || proof.ip.rounds.len() != log_n {
        return Err("multiple image public proof shape".into());
    }
    bind_public(log_n, left, source, ch);
    if dot(&proof.columns, &source.column_weights) != source.value {
        return Err("multiple image public column scalar check".into());
    }
    let projection = column_message(&proof.columns, ch);
    let scalar = projected_claim(&proof.columns, &projection);
    ch.observe_label(b"mikros-multiple-image-public-ip-v1");
    let mut point = inner_product::verify(&proof.ip, scalar, ch)
        .map_err(|e| format!("multiple image public IP: {e:?}"))?;
    point.reverse();
    let mut xi = tensor_high(&source.row_factors);
    let rhs = public_marginals(circuit, &mut xi, &projection, left, false)?.selected;
    if dot(&eq_low(&point), &rhs) != proof.ip.final_b {
        return Err("multiple image public terminal".into());
    }
    Ok(PointClaim {
        point,
        value: proof.ip.final_a,
    })
}

fn claims(proof: &Proof, y: Vec<F128>, x: Vec<F128>) -> Claims {
    Claims {
        witness: PointClaim {
            point: y.clone(),
            value: proof.ip1.final_a,
        },
        matrix: MatrixEvaluation {
            row_point: x,
            column_point: y,
            value: proof.ip2.final_a,
        },
    }
}

pub fn prove<C: Challenger>(
    matrix: &SparseBinaryMatrix,
    witness: &[F128],
    image: &[F128],
    source: &MatrixClaim,
    ch: &mut C,
) -> Result<(Proof, Claims), String> {
    let log_n = source.row_factors.len();
    let n = checked_shape(log_n, source)?;
    if witness.len() != n
        || image.len() != n
        || matrix.num_rows != n
        || matrix.num_cols != n
        || matrix.rows.len() != n
        || matrix.rows.iter().flatten().any(|&column| column >= n)
    {
        return Err("multiple image sparse matrix/input shape".into());
    }
    bind(log_n, source, ch);
    let xi = tensor_high(&source.row_factors);
    let columns = fold_1b_rows_naive(image, &xi);
    if dot(&columns, &source.column_weights) != source.value {
        return Err("multiple image column scalar check".into());
    }
    let projection = column_message(&columns, ch);
    let scalar = projected_claim(&columns, &projection);
    let table = build_fold_byte_table(&projection);
    let xi_projection: Vec<_> = xi
        .par_iter()
        .map(|&x| project_coordinate(x, &table))
        .collect();
    let rhs = sparse_transpose_apply(matrix, &xi_projection);
    if dot(witness, &rhs) != scalar {
        return Err("multiple image projected identity".into());
    }
    ch.observe_label(b"mikros-multiple-image-first-ip-v1");
    let (ip1, y) = native_ip(witness.to_vec(), rhs, ch);
    let image_eq_y = sparse_apply(matrix, &eq_low(&y));
    if dot(&image_eq_y, &xi_projection) != ip1.final_b {
        return Err("multiple image second IP identity".into());
    }
    ch.observe_label(b"mikros-multiple-image-second-ip-v1");
    let (ip2, x) = native_ip(image_eq_y, xi_projection, ch);
    let proof = Proof { columns, ip1, ip2 };
    debug_assert_eq!(proof.ip1.rounds.len(), log_n);
    debug_assert_eq!(proof.ip2.rounds.len(), log_n);
    let claims = claims(&proof, y, x);
    Ok((proof, claims))
}

pub fn verify<C: Challenger>(
    log_n: usize,
    source: &MatrixClaim,
    proof: &Proof,
    ch: &mut C,
) -> Result<Claims, String> {
    checked_shape(log_n, source)?;
    if proof.columns.len() != WIDTH
        || proof.ip1.rounds.len() != log_n
        || proof.ip2.rounds.len() != log_n
    {
        return Err("multiple image proof shape".into());
    }
    bind(log_n, source, ch);
    if dot(&proof.columns, &source.column_weights) != source.value {
        return Err("multiple image column scalar check".into());
    }
    let projection = column_message(&proof.columns, ch);
    let scalar = projected_claim(&proof.columns, &projection);
    ch.observe_label(b"mikros-multiple-image-first-ip-v1");
    let mut y = inner_product::verify(&proof.ip1, scalar, ch)
        .map_err(|e| format!("multiple image first IP: {e:?}"))?;
    y.reverse();
    ch.observe_label(b"mikros-multiple-image-second-ip-v1");
    let mut x = inner_product::verify(&proof.ip2, proof.ip1.final_b, ch)
        .map_err(|e| format!("multiple image second IP: {e:?}"))?;
    x.reverse();
    if evaluate_public_query(source, &projection, &x) != proof.ip2.final_b {
        return Err("multiple image public tensor recursion".into());
    }
    Ok(claims(proof, y, x))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchProof {
    /// One original binary coordinate matrix per block; never combine these
    /// packed columns by F128 multiplication before coordinate extraction.
    pub columns: Vec<Vec<F128>>,
    pub ip1: InnerProductProof,
    pub ip2: InnerProductProof,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchClaims {
    /// Opening of sum_j weights[j] * witness[j], at a LOW FIRST point.
    pub witness: PointClaim,
    pub matrix: MatrixEvaluation,
    pub weights: Vec<F128>,
}

fn batch_shape(log_n: usize, sources: &[MatrixClaim]) -> Result<usize, String> {
    let first = sources.first().ok_or("empty image batch")?;
    let n = checked_shape(log_n, first)?;
    for source in sources {
        checked_shape(log_n, source)?;
        if source.row_factors != first.row_factors {
            return Err("image batch requires identical row factors".into());
        }
    }
    Ok(n)
}

fn batch_transcript<C: Challenger>(
    log_n: usize,
    sources: &[MatrixClaim],
    columns: &[Vec<F128>],
    ch: &mut C,
) -> Result<(Vec<F128>, Vec<F128>, F128), String> {
    if columns.len() != sources.len() || columns.iter().any(|u| u.len() != WIDTH) {
        return Err("image batch column shape".into());
    }
    ch.observe_label(b"mikros-multiple-image-shared-query-batch-v1");
    ch.observe_bytes(&(sources.len() as u64).to_le_bytes());
    for (source, u) in sources.iter().zip(columns) {
        bind(log_n, source, ch);
        if dot(u, &source.column_weights) != source.value {
            return Err("image batch column scalar check".into());
        }
        ch.observe_label(b"mikros-image-batch-fixed-columns-v1");
        ch.observe_f128_slice(u);
    }
    // All U_j are fixed before both independent random reductions.
    ch.observe_label(b"mikros-image-batch-weights-v1");
    let weights = ch.sample_f128_vec(sources.len());
    ch.observe_label(b"mikros-image-batch-coordinate-projection-v1");
    let projection = ch.sample_f128_vec(WIDTH);
    let scalar = columns
        .iter()
        .zip(&weights)
        .fold(F128::ZERO, |sum, (u, &z)| {
            // Coordinate extraction is only F2-linear. Apply it to each U_j first.
            sum + z * projected_claim(u, &projection)
        });
    Ok((weights, projection, scalar))
}

fn batch_claims(proof: &BatchProof, y: Vec<F128>, x: Vec<F128>, weights: Vec<F128>) -> BatchClaims {
    BatchClaims {
        witness: PointClaim {
            point: y.clone(),
            value: proof.ip1.final_a,
        },
        matrix: MatrixEvaluation {
            row_point: x,
            column_point: y,
            value: proof.ip2.final_a,
        },
        weights,
    }
}

/// Reduce independent 128-instance blocks with one public CSC and one row
/// tensor. All original binary column messages precede the block weights and
/// coordinate projection. The caller binds all packed witness commitments and
/// authenticates the returned weighted ordinary MLE through their native PCS.
pub fn prove_public_batch<C: Challenger>(
    circuit: &dyn LincheckCircuit,
    left: bool,
    witnesses: &[&[F128]],
    images: &[&[F128]],
    sources: &[MatrixClaim],
    ch: &mut C,
) -> Result<(PublicBatchProof, PublicBatchClaims), String> {
    let profile = std::env::var("MIKROS_PROFILE").is_ok_and(|v| v == "1");
    let start = profile.then(std::time::Instant::now);
    let log_n = sources
        .first()
        .ok_or("empty image batch")?
        .row_factors
        .len();
    let n = batch_shape(log_n, sources)?;
    if circuit.n_cols() != n
        || witnesses.len() != sources.len()
        || images.len() != sources.len()
        || witnesses.iter().chain(images).any(|w| w.len() != n)
    {
        return Err("multiple image public batch input shape".into());
    }
    let stage = profile.then(std::time::Instant::now);
    let mut xi = tensor_high(&sources[0].row_factors);
    let row_weights_ms = profile_elapsed(stage);
    let stage = profile.then(std::time::Instant::now);
    let columns: Vec<_> = images
        .iter()
        .map(|image| fold_1b_rows_naive(image, &xi))
        .collect();
    let columns_ms = profile_elapsed(stage);
    let stage = profile.then(std::time::Instant::now);
    ch.observe_label(b"mikros-multiple-image-public-csc-batch-v1");
    ch.observe_bytes(&[u8::from(left)]);
    let (weights, projection, scalar) = batch_transcript(log_n, sources, &columns, ch)?;
    let transcript_projection_ms = profile_elapsed(stage);
    let PublicMarginal {
        selected: rhs,
        unused,
        coordinate_ms,
        csc_ms,
    } = public_marginals(circuit, &mut xi, &projection, left, profile)?;
    let stage = profile.then(std::time::Instant::now);
    let mut scratch = ProverScratch::with_fold_buffers([xi, unused]);
    scratch.a = (0..n)
        .into_par_iter()
        .map(|i| {
            witnesses
                .iter()
                .zip(&weights)
                .fold(F128::ZERO, |sum, (w, &z)| sum + z * w[i])
        })
        .collect();
    scratch.b = rhs;
    let combine_ms = profile_elapsed(stage);
    let stage = profile.then(std::time::Instant::now);
    let (actual, initial_message) =
        inner_product::checked_initial_message_native(&scratch.a, &scratch.b);
    if actual != scalar {
        return Err("multiple image public batch projected identity".into());
    }
    let first_message_ms = profile_elapsed(stage);
    let stage = profile.then(std::time::Instant::now);
    ch.observe_label(b"mikros-multiple-image-public-batch-ip-v1");
    let (ip, point) = native_ip_in_scratch(&mut scratch, initial_message, ch);
    let ip_ms = profile_elapsed(stage);
    let claims = PublicBatchClaims {
        witness: PointClaim {
            point,
            value: ip.final_a,
        },
        weights,
    };
    if profile {
        eprintln!(
            "{}",
            serde_json::json!({
                "record": "mikros_public_image_batch_profile", "side": if left { "A" } else { "B" },
                "n": n, "blocks": sources.len(), "single_csc": public_csc_single(),
                "row_weights_ms": row_weights_ms, "columns_ms": columns_ms,
                "transcript_projection_ms": transcript_projection_ms,
                "coordinate_map_ms": coordinate_ms, "csc_ms": csc_ms,
                "combine_ms": combine_ms, "first_message_ms": first_message_ms,
                "ip_ms": ip_ms, "total_ms": profile_elapsed(start)
            })
        );
    }
    Ok((PublicBatchProof { columns, ip }, claims))
}

/// Verify a shared-row public image batch. This explicitly evaluates the
/// public matrix terminal, then returns the weighted packed witness PCS claim.
pub fn verify_public_batch<C: Challenger>(
    circuit: &dyn LincheckCircuit,
    left: bool,
    sources: &[MatrixClaim],
    proof: &PublicBatchProof,
    ch: &mut C,
) -> Result<PublicBatchClaims, String> {
    let log_n = sources
        .first()
        .ok_or("empty image batch")?
        .row_factors
        .len();
    let n = batch_shape(log_n, sources)?;
    if circuit.n_cols() != n || proof.ip.rounds.len() != log_n {
        return Err("multiple image public batch proof shape".into());
    }
    ch.observe_label(b"mikros-multiple-image-public-csc-batch-v1");
    ch.observe_bytes(&[u8::from(left)]);
    let (weights, projection, scalar) = batch_transcript(log_n, sources, &proof.columns, ch)?;
    ch.observe_label(b"mikros-multiple-image-public-batch-ip-v1");
    let mut point = inner_product::verify(&proof.ip, scalar, ch)
        .map_err(|e| format!("multiple image public batch IP: {e:?}"))?;
    point.reverse();
    let mut xi = tensor_high(&sources[0].row_factors);
    let rhs = public_marginals(circuit, &mut xi, &projection, left, false)?.selected;
    if dot(&eq_low(&point), &rhs) != proof.ip.final_b {
        return Err("multiple image public batch terminal".into());
    }
    Ok(PublicBatchClaims {
        witness: PointClaim {
            point,
            value: proof.ip.final_a,
        },
        weights,
    })
}

/// Authenticate several independent image claims sharing one row tensor and
/// one binary matrix. Bind every witness/circuit commitment before entry.
/// Caller authenticates the returned matrix claim and weighted witness claim.
pub fn prove_batch<C: Challenger>(
    matrix: &SparseBinaryMatrix,
    witnesses: &[&[F128]],
    images: &[&[F128]],
    sources: &[MatrixClaim],
    ch: &mut C,
) -> Result<(BatchProof, BatchClaims), String> {
    // This switch restores the separate self-check/message scans and scratch
    // allocations for reproducible ablations. It never changes the transcript.
    let fusion = !std::env::var_os("MIKROS_NO_IMAGE_FUSION").is_some_and(|value| value == "1");
    prove_batch_with_fusion(matrix, witnesses, images, sources, fusion, ch)
}

fn prove_batch_with_fusion<C: Challenger>(
    matrix: &SparseBinaryMatrix,
    witnesses: &[&[F128]],
    images: &[&[F128]],
    sources: &[MatrixClaim],
    fusion: bool,
    ch: &mut C,
) -> Result<(BatchProof, BatchClaims), String> {
    let log_n = sources
        .first()
        .ok_or("empty image batch")?
        .row_factors
        .len();
    let n = batch_shape(log_n, sources)?;
    if witnesses.len() != sources.len()
        || images.len() != sources.len()
        || witnesses.iter().chain(images).any(|w| w.len() != n)
        || matrix.num_rows != n
        || matrix.num_cols != n
        || matrix.rows.len() != n
        || matrix.rows.iter().flatten().any(|&column| column >= n)
    {
        return Err("image batch input shape".into());
    }
    let xi = tensor_high(&sources[0].row_factors);
    let columns: Vec<_> = images
        .iter()
        .map(|image| fold_1b_rows_naive(image, &xi))
        .collect();
    let (weights, projection, scalar) = batch_transcript(log_n, sources, &columns, ch)?;
    let table = build_fold_byte_table(&projection);
    let xi_projection: Vec<_> = xi
        .par_iter()
        .map(|&x| project_coordinate(x, &table))
        .collect();
    let combined: Vec<_> = (0..n)
        .into_par_iter()
        .map(|i| {
            witnesses
                .iter()
                .zip(&weights)
                .fold(F128::ZERO, |sum, (w, &z)| sum + z * w[i])
        })
        .collect();
    let rhs = sparse_transpose_apply(matrix, &xi_projection);
    let (ip1, ip2, y, x) = if fusion {
        let mut scratch = ProverScratch::default();
        scratch.a = combined;
        scratch.b = rhs;
        let (actual, message) =
            inner_product::checked_initial_message_native(&scratch.a, &scratch.b);
        if actual != scalar {
            return Err("image batch projected identity".into());
        }
        ch.observe_label(b"mikros-multiple-image-first-ip-v1");
        let (ip1, y) = native_ip_in_scratch(&mut scratch, message, ch);
        sparse_apply_into(matrix, &eq_low(&y), &mut scratch.a);
        scratch.b = xi_projection;
        let (actual, message) =
            inner_product::checked_initial_message_native(&scratch.a, &scratch.b);
        if actual != ip1.final_b {
            return Err("image batch second IP identity".into());
        }
        ch.observe_label(b"mikros-multiple-image-second-ip-v1");
        let (ip2, x) = native_ip_in_scratch(&mut scratch, message, ch);
        (ip1, ip2, y, x)
    } else {
        if dot(&combined, &rhs) != scalar {
            return Err("image batch projected identity".into());
        }
        ch.observe_label(b"mikros-multiple-image-first-ip-v1");
        let (ip1, y) = native_ip(combined, rhs, ch);
        let image_eq_y = sparse_apply(matrix, &eq_low(&y));
        if dot(&image_eq_y, &xi_projection) != ip1.final_b {
            return Err("image batch second IP identity".into());
        }
        ch.observe_label(b"mikros-multiple-image-second-ip-v1");
        let (ip2, x) = native_ip(image_eq_y, xi_projection, ch);
        (ip1, ip2, y, x)
    };
    let proof = BatchProof { columns, ip1, ip2 };
    let claims = batch_claims(&proof, y, x, weights);
    Ok((proof, claims))
}

pub fn verify_batch<C: Challenger>(
    log_n: usize,
    sources: &[MatrixClaim],
    proof: &BatchProof,
    ch: &mut C,
) -> Result<BatchClaims, String> {
    batch_shape(log_n, sources)?;
    if proof.ip1.rounds.len() != log_n || proof.ip2.rounds.len() != log_n {
        return Err("image batch IP shape".into());
    }
    let (weights, projection, scalar) = batch_transcript(log_n, sources, &proof.columns, ch)?;
    ch.observe_label(b"mikros-multiple-image-first-ip-v1");
    let mut y = inner_product::verify(&proof.ip1, scalar, ch)
        .map_err(|e| format!("image batch first IP: {e:?}"))?;
    y.reverse();
    ch.observe_label(b"mikros-multiple-image-second-ip-v1");
    let mut x = inner_product::verify(&proof.ip2, proof.ip1.final_b, ch)
        .map_err(|e| format!("image batch second IP: {e:?}"))?;
    x.reverse();
    if evaluate_public_query(&sources[0], &projection, &x) != proof.ip2.final_b {
        return Err("image batch public tensor recursion".into());
    }
    Ok(batch_claims(proof, y, x, weights))
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::lincheck::CscCircuit;
    use flock_field::test_rng::Rng;
    use flock_transcript::challenger::FsChallenger;

    fn with_single_csc<T>(enabled: bool, f: impl FnOnce() -> T) -> T {
        struct Reset(Option<bool>);
        impl Drop for Reset {
            fn drop(&mut self) {
                PUBLIC_CSC_SINGLE_OVERRIDE.with(|v| v.set(self.0));
            }
        }
        let _reset = Reset(PUBLIC_CSC_SINGLE_OVERRIDE.with(|v| v.replace(Some(enabled))));
        f()
    }

    fn with_blocked_weights<T>(enabled: bool, f: impl FnOnce() -> T) -> T {
        struct Reset(Option<bool>);
        impl Drop for Reset {
            fn drop(&mut self) {
                IMAGE_BLOCKED_WEIGHTS_OVERRIDE.with(|v| v.set(self.0));
            }
        }
        let _reset = Reset(IMAGE_BLOCKED_WEIGHTS_OVERRIDE.with(|v| v.replace(Some(enabled))));
        f()
    }

    #[test]
    fn public_csc_single_side_preserves_proof_and_transcript() {
        let mut rng = Rng::new(0xC5C0_0201);
        for log_n in [0, 1, 5, 9] {
            let (circuit, a, b) = public_dense_matrices(log_n);
            for zero in [false, true] {
                let factors: Vec<_> = (0..log_n)
                    .map(|_| {
                        if zero {
                            [F128::ZERO; 2]
                        } else {
                            [rng.f128(), rng.f128()]
                        }
                    })
                    .collect();
                for (left, matrix) in [(true, &a), (false, &b)] {
                    let ws: Vec<_> = (0..2).map(|_| rng.f128_vec(1 << log_n)).collect();
                    let triples: Vec<_> = ws
                        .iter()
                        .map(|w| public_dense_source(matrix, w, &factors, rng.f128_vec(WIDTH)))
                        .collect();
                    let witnesses: Vec<_> = ws.iter().map(Vec::as_slice).collect();
                    let images: Vec<_> = triples.iter().map(|x| x.0.as_slice()).collect();
                    let sources: Vec<_> = triples.iter().map(|x| x.1.clone()).collect();
                    let run = |single| {
                        with_single_csc(single, || {
                            let mut ch = FsChallenger::new(b"single-csc-equivalence");
                            let result = prove_public_batch(
                                &circuit, left, &witnesses, &images, &sources, &mut ch,
                            )
                            .unwrap();
                            let end = ch.sample_f128_vec(8);
                            let mut vc = FsChallenger::new(b"single-csc-equivalence");
                            assert_eq!(
                                verify_public_batch(&circuit, left, &sources, &result.0, &mut vc)
                                    .unwrap(),
                                result.1
                            );
                            assert_eq!(end, vc.sample_f128_vec(8));
                            (result, end)
                        })
                    };
                    let reference = with_blocked_weights(false, || run(false));
                    assert_eq!(with_blocked_weights(false, || run(true)), reference);
                    assert_eq!(with_blocked_weights(true, || run(false)), reference);
                    assert_eq!(with_blocked_weights(true, || run(true)), reference);
                }
            }
        }
    }

    fn public_dense_matrices(log_n: usize) -> (CscCircuit, Vec<Vec<bool>>, Vec<Vec<bool>>) {
        let n = 1 << log_n;
        let a: Vec<Vec<bool>> = (0..n)
            .map(|r| (0..n).map(|c| c == r || c == r + 1).collect())
            .collect();
        let b: Vec<Vec<bool>> = (0..n)
            .map(|r| (0..n).map(|c| c + 1 == r).collect())
            .collect();
        let sparse = |dense: &[Vec<bool>]| {
            SparseBinaryMatrix::new(
                n,
                n,
                dense
                    .iter()
                    .map(|row| {
                        row.iter()
                            .enumerate()
                            .filter_map(|(c, &v)| v.then_some(c))
                            .collect()
                    })
                    .collect(),
            )
        };
        (CscCircuit::from_matrices(&sparse(&a), &sparse(&b)), a, b)
    }

    // Independent row/bit loops: do not use sparse_apply, tensor_high or
    // fold_1b_rows_naive to derive the expected public statement or columns.
    fn public_dense_source(
        matrix: &[Vec<bool>],
        witness: &[F128],
        factors: &[[F128; 2]],
        rho: Vec<F128>,
    ) -> (Vec<F128>, MatrixClaim, Vec<F128>) {
        let image: Vec<_> = matrix
            .iter()
            .map(|row| {
                row.iter()
                    .zip(witness)
                    .filter(|(bit, _)| **bit)
                    .fold(F128::ZERO, |s, (_, &w)| s + w)
            })
            .collect();
        let mut columns = vec![F128::ZERO; WIDTH];
        for (i, &word) in image.iter().enumerate() {
            let xi = factors.iter().enumerate().fold(F128::ONE, |v, (j, pair)| {
                v * pair[(i >> (factors.len() - 1 - j)) & 1]
            });
            for (bit, column) in columns.iter_mut().enumerate() {
                let on = if bit < 64 {
                    (word.lo >> bit) & 1
                } else {
                    (word.hi >> (bit - 64)) & 1
                };
                if on != 0 {
                    *column += xi;
                }
            }
        }
        let value = columns
            .iter()
            .zip(&rho)
            .fold(F128::ZERO, |s, (&a, &b)| s + a * b);
        (
            image,
            MatrixClaim {
                row_factors: factors.to_vec(),
                column_weights: rho,
                value,
            },
            columns,
        )
    }

    fn public_direct_mle(values: &[F128], point: &[F128]) -> F128 {
        values
            .iter()
            .enumerate()
            .fold(F128::ZERO, |sum, (i, &value)| {
                let weight = point.iter().enumerate().fold(F128::ONE, |w, (j, &r)| {
                    w * if (i >> j) & 1 == 1 { r } else { F128::ONE + r }
                });
                sum + value * weight
            })
    }

    #[test]
    fn public_csc_dense_reference_single_and_batch() {
        let mut rng = Rng::new(0xC5C0_0101);
        for log_n in [0, 1, 3, 8, 9] {
            let (circuit, a, b) = public_dense_matrices(log_n);
            for mode in 0..4 {
                let mut factors: Vec<_> = (0..log_n).map(|_| [rng.f128(), rng.f128()]).collect();
                if log_n > 0 {
                    match mode {
                        1 => factors[0] = [rng.f128(); 2],
                        2 => factors[0] = [F128::ZERO; 2],
                        3 => factors[0] = [F128::ZERO, F128::ONE],
                        _ => {}
                    }
                }
                for (left, matrix) in [(true, &a), (false, &b)] {
                    let witnesses: Vec<_> = (0..2).map(|_| rng.f128_vec(1 << log_n)).collect();
                    let triples: Vec<_> = witnesses
                        .iter()
                        .map(|w| public_dense_source(matrix, w, &factors, rng.f128_vec(WIDTH)))
                        .collect();
                    for (witness, (image, source, columns)) in witnesses.iter().zip(&triples) {
                        let mut pc = FsChallenger::new(b"public-csc-dense-single");
                        let (proof, claim) =
                            prove_public(&circuit, left, witness, image, source, &mut pc).unwrap();
                        assert_eq!(&proof.columns, columns);
                        assert_eq!(proof.ip.rounds.len(), log_n);
                        assert_eq!(public_direct_mle(witness, &claim.point), claim.value);
                        let mut vc = FsChallenger::new(b"public-csc-dense-single");
                        assert_eq!(
                            verify_public(&circuit, left, source, &proof, &mut vc).unwrap(),
                            claim
                        );
                        assert_eq!(pc.sample_f128_vec(8), vc.sample_f128_vec(8));
                    }
                    let ws: Vec<_> = witnesses.iter().map(Vec::as_slice).collect();
                    let ims: Vec<_> = triples.iter().map(|t| t.0.as_slice()).collect();
                    let sources: Vec<_> = triples.iter().map(|t| t.1.clone()).collect();
                    let mut pc = FsChallenger::new(b"public-csc-dense-batch");
                    let (proof, claims) =
                        prove_public_batch(&circuit, left, &ws, &ims, &sources, &mut pc).unwrap();
                    assert_eq!(
                        proof.columns,
                        triples.iter().map(|t| t.2.clone()).collect::<Vec<_>>()
                    );
                    let expected = witnesses
                        .iter()
                        .zip(&claims.weights)
                        .fold(F128::ZERO, |s, (w, &z)| {
                            s + z * public_direct_mle(w, &claims.witness.point)
                        });
                    assert_eq!(claims.witness.value, expected);
                    let mut vc = FsChallenger::new(b"public-csc-dense-batch");
                    assert_eq!(
                        verify_public_batch(&circuit, left, &sources, &proof, &mut vc).unwrap(),
                        claims
                    );
                    assert_eq!(pc.sample_f128_vec(8), vc.sample_f128_vec(8));
                    let bytes = bincode::serialize(&proof).unwrap();
                    assert_eq!(
                        bincode::deserialize::<PublicBatchProof>(&bytes).unwrap(),
                        proof
                    );
                }
            }
        }
    }

    #[test]
    fn public_csc_single_rejects_columns_endpoints_rounds_side_and_witness() {
        let (circuit, a, _) = public_dense_matrices(3);
        let mut rng = Rng::new(0xC5C0_0202);
        let witness = rng.f128_vec(8);
        let factors: Vec<_> = (0..3).map(|_| [rng.f128(), rng.f128()]).collect();
        let (image, source, _) = public_dense_source(&a, &witness, &factors, rng.f128_vec(WIDTH));
        let domain = b"public-csc-single-tamper";
        let (proof, _) = prove_public(
            &circuit,
            true,
            &witness,
            &image,
            &source,
            &mut FsChallenger::new(domain),
        )
        .unwrap();
        for kind in 0..8 {
            let mut bad = proof.clone();
            match kind {
                0 => bad.columns[17] += F128::ONE,
                1 => bad.ip.final_a += F128::ONE,
                2 => bad.ip.final_b += F128::ONE,
                3 => bad.ip.rounds[1][0] += F128::ONE,
                4 => {
                    bad.ip.rounds.pop();
                }
                5 => bad.ip.rounds.push([F128::ZERO; 2]),
                6 => {
                    bad.columns.pop();
                }
                _ => bad.columns.push(F128::ZERO),
            }
            assert!(
                verify_public(
                    &circuit,
                    true,
                    &source,
                    &bad,
                    &mut FsChallenger::new(domain)
                )
                .is_err(),
                "kind={kind}"
            );
        }
        assert!(
            verify_public(
                &circuit,
                false,
                &source,
                &proof,
                &mut FsChallenger::new(domain)
            )
            .is_err()
        );
        for kind in 0..3 {
            let mut bad = source.clone();
            match kind {
                0 => bad.value += F128::ONE,
                1 => bad.row_factors[0][0] += F128::ONE,
                _ => bad.column_weights[0] += F128::ONE,
            }
            assert!(
                verify_public(&circuit, true, &bad, &proof, &mut FsChallenger::new(domain))
                    .is_err()
            );
        }
        let mut wrong_witness = witness.clone();
        wrong_witness[0] += F128::ONE;
        assert!(
            prove_public(
                &circuit,
                true,
                &wrong_witness,
                &image,
                &source,
                &mut FsChallenger::new(domain)
            )
            .is_err()
        );
        let mut wrong_image = image;
        wrong_image[0] += F128::ONE;
        assert!(
            prove_public(
                &circuit,
                true,
                &witness,
                &wrong_image,
                &source,
                &mut FsChallenger::new(domain)
            )
            .is_err()
        );
    }

    #[test]
    fn public_csc_batch_rejects_columns_sources_rounds_side_and_witness() {
        let (circuit, a, b) = public_dense_matrices(3);
        let mut rng = Rng::new(0xC5C0_0303);
        let factors: Vec<_> = (0..3).map(|_| [rng.f128(), rng.f128()]).collect();
        let witnesses: Vec<_> = (0..2).map(|_| rng.f128_vec(8)).collect();
        let triples: Vec<_> = witnesses
            .iter()
            .map(|w| public_dense_source(&a, w, &factors, rng.f128_vec(WIDTH)))
            .collect();
        let ws: Vec<_> = witnesses.iter().map(Vec::as_slice).collect();
        let ims: Vec<_> = triples.iter().map(|t| t.0.as_slice()).collect();
        let sources: Vec<_> = triples.iter().map(|t| t.1.clone()).collect();
        let domain = b"public-csc-batch-tamper";
        let (proof, _) = prove_public_batch(
            &circuit,
            true,
            &ws,
            &ims,
            &sources,
            &mut FsChallenger::new(domain),
        )
        .unwrap();
        for kind in 0..9 {
            let mut bad = proof.clone();
            match kind {
                0 => bad.columns[1][3] += F128::ONE,
                1 => bad.columns.swap(0, 1),
                2 => bad.ip.final_a += F128::ONE,
                3 => bad.ip.final_b += F128::ONE,
                4 => bad.ip.rounds[0][1] += F128::ONE,
                5 => {
                    bad.ip.rounds.pop();
                }
                6 => bad.ip.rounds.push([F128::ZERO; 2]),
                7 => {
                    bad.columns[0].pop();
                }
                _ => {
                    bad.columns.pop();
                }
            }
            assert!(
                verify_public_batch(
                    &circuit,
                    true,
                    &sources,
                    &bad,
                    &mut FsChallenger::new(domain)
                )
                .is_err(),
                "kind={kind}"
            );
        }
        assert!(
            verify_public_batch(
                &circuit,
                false,
                &sources,
                &proof,
                &mut FsChallenger::new(domain)
            )
            .is_err()
        );
        let to_sparse = |m: &[Vec<bool>]| {
            SparseBinaryMatrix::new(
                8,
                8,
                m.iter()
                    .map(|row| {
                        row.iter()
                            .enumerate()
                            .filter_map(|(i, &v)| v.then_some(i))
                            .collect()
                    })
                    .collect(),
            )
        };
        let swapped = CscCircuit::from_matrices(&to_sparse(&b), &to_sparse(&a));
        assert!(
            verify_public_batch(
                &swapped,
                true,
                &sources,
                &proof,
                &mut FsChallenger::new(domain)
            )
            .is_err()
        );
        for kind in 0..4 {
            let mut bad = sources.clone();
            match kind {
                0 => bad[1].value += F128::ONE,
                1 => bad[1].row_factors[0][0] += F128::ONE,
                2 => bad[1].column_weights[3] += F128::ONE,
                _ => bad.swap(0, 1),
            }
            assert!(
                verify_public_batch(&circuit, true, &bad, &proof, &mut FsChallenger::new(domain))
                    .is_err()
            );
        }
        let mut wrong = witnesses.clone();
        wrong[1][0] += F128::ONE;
        let wrong_ws: Vec<_> = wrong.iter().map(Vec::as_slice).collect();
        assert!(
            prove_public_batch(
                &circuit,
                true,
                &wrong_ws,
                &ims,
                &sources,
                &mut FsChallenger::new(domain)
            )
            .is_err()
        );
    }

    #[test]
    fn public_csc_rejects_malformed_inputs_and_public_outputs() {
        let (circuit, a, _) = public_dense_matrices(1);
        let witness = [F128::ONE, F128::new(37, 91)];
        let (image, source, _) =
            public_dense_source(&a, &witness, &[[F128::ONE, F128::new(21, 41)]], basis());
        let domain = b"public-csc-shape";
        let (proof, _) = prove_public(
            &circuit,
            true,
            &witness,
            &image,
            &source,
            &mut FsChallenger::new(domain),
        )
        .unwrap();
        let (batch, _) = prove_public_batch(
            &circuit,
            true,
            &[&witness],
            &[&image],
            &[source.clone()],
            &mut FsChallenger::new(domain),
        )
        .unwrap();
        for kind in 0..4 {
            let mut bad = source.clone();
            match kind {
                0 => {
                    bad.column_weights.pop();
                }
                1 => {
                    bad.row_factors.pop();
                }
                2 => bad.row_factors.push([F128::ONE; 2]),
                _ => bad.row_factors = vec![[F128::ONE; 2]; usize::BITS as usize],
            }
            assert!(
                prove_public(
                    &circuit,
                    true,
                    &witness,
                    &image,
                    &bad,
                    &mut FsChallenger::new(domain)
                )
                .is_err()
            );
            assert!(
                verify_public(&circuit, true, &bad, &proof, &mut FsChallenger::new(domain))
                    .is_err()
            );
            assert!(
                verify_public_batch(
                    &circuit,
                    true,
                    &[bad],
                    &batch,
                    &mut FsChallenger::new(domain)
                )
                .is_err()
            );
        }
        assert!(
            prove_public(
                &circuit,
                true,
                &witness[..1],
                &image,
                &source,
                &mut FsChallenger::new(domain)
            )
            .is_err()
        );
        assert!(
            prove_public(
                &circuit,
                true,
                &witness,
                &image[..1],
                &source,
                &mut FsChallenger::new(domain)
            )
            .is_err()
        );
        assert!(
            prove_public_batch(
                &circuit,
                true,
                &[],
                &[],
                &[],
                &mut FsChallenger::new(domain)
            )
            .is_err()
        );
        assert!(
            verify_public_batch(&circuit, true, &[], &batch, &mut FsChallenger::new(domain))
                .is_err()
        );
        assert!(
            prove_public_batch(
                &circuit,
                true,
                &[&witness],
                &[],
                &[source.clone()],
                &mut FsChallenger::new(domain)
            )
            .is_err()
        );
        assert!(
            prove_public_batch(
                &circuit,
                true,
                &[&witness[..1]],
                &[&image],
                &[source.clone()],
                &mut FsChallenger::new(domain)
            )
            .is_err()
        );
        struct BadCircuit {
            n: usize,
            malformed: bool,
        }
        impl LincheckCircuit for BadCircuit {
            fn n_cols(&self) -> usize {
                self.n
            }
            fn fold_alpha_batched(&self, _: F128, _: &[F128]) -> Vec<F128> {
                panic!("must use fold_split")
            }
            fn fold_split(&self, _: &[F128]) -> (Vec<F128>, Vec<F128>) {
                assert!(self.malformed, "shape must fail before matrix evaluation");
                (vec![F128::ZERO; 1], vec![F128::ZERO; 2])
            }
        }
        for bad in [
            BadCircuit {
                n: 0,
                malformed: false,
            },
            BadCircuit {
                n: 3,
                malformed: false,
            },
            BadCircuit {
                n: 2,
                malformed: true,
            },
        ] {
            assert!(
                prove_public(
                    &bad,
                    true,
                    &witness,
                    &image,
                    &source,
                    &mut FsChallenger::new(domain)
                )
                .is_err()
            );
            assert!(
                verify_public(&bad, true, &source, &proof, &mut FsChallenger::new(domain)).is_err()
            );
            assert!(
                prove_public_batch(
                    &bad,
                    true,
                    &[&witness],
                    &[&image],
                    &[source.clone()],
                    &mut FsChallenger::new(domain)
                )
                .is_err()
            );
            assert!(
                verify_public_batch(
                    &bad,
                    true,
                    &[source.clone()],
                    &batch,
                    &mut FsChallenger::new(domain)
                )
                .is_err()
            );
        }
    }

    #[test]
    fn public_csc_zero_and_cancelling_batch_weights() {
        struct FixedWeights {
            inner: FsChallenger,
            weights: Option<Vec<F128>>,
        }
        impl Challenger for FixedWeights {
            fn fork_from_seed(&self, seed: [F128; 2], label: &'static [u8]) -> Self {
                Self {
                    inner: self.inner.fork_from_seed(seed, label),
                    weights: None,
                }
            }
            fn observe_label(&mut self, label: &[u8]) {
                self.inner.observe_label(label);
            }
            fn observe_bytes(&mut self, bytes: &[u8]) {
                self.inner.observe_bytes(bytes);
            }
            fn observe_f128(&mut self, value: F128) {
                self.inner.observe_f128(value);
            }
            fn sample_f128(&mut self) -> F128 {
                self.inner.sample_f128()
            }
            fn sample_f128_vec(&mut self, n: usize) -> Vec<F128> {
                if let Some(weights) = self.weights.take() {
                    assert_eq!(weights.len(), n);
                    weights
                } else {
                    self.inner.sample_f128_vec(n)
                }
            }
        }
        let (circuit, a, _) = public_dense_matrices(3);
        let mut rng = Rng::new(0xC5C0_0505);
        let witness = rng.f128_vec(8);
        let factors: Vec<_> = (0..3).map(|_| [rng.f128(); 2]).collect();
        let triples: Vec<_> = (0..2)
            .map(|_| public_dense_source(&a, &witness, &factors, rng.f128_vec(WIDTH)))
            .collect();
        let ims: Vec<_> = triples.iter().map(|t| t.0.as_slice()).collect();
        let sources: Vec<_> = triples.iter().map(|t| t.1.clone()).collect();
        let z = rng.f128();
        for weights in [vec![F128::ZERO; 2], vec![z; 2], vec![z, F128::ZERO]] {
            let make_ch = || FixedWeights {
                inner: FsChallenger::new(b"public-csc-zero-weights"),
                weights: Some(weights.clone()),
            };
            let mut pc = make_ch();
            let (proof, claims) = prove_public_batch(
                &circuit,
                true,
                &[&witness, &witness],
                &ims,
                &sources,
                &mut pc,
            )
            .unwrap();
            assert_eq!(claims.weights, weights);
            assert_eq!(
                claims.witness.value,
                (weights[0] + weights[1]) * public_direct_mle(&witness, &claims.witness.point)
            );
            let mut vc = make_ch();
            assert_eq!(
                verify_public_batch(&circuit, true, &sources, &proof, &mut vc).unwrap(),
                claims
            );
            assert_eq!(pc.sample_f128(), vc.sample_f128());
        }
    }

    fn fixture(log_n: usize, seed: u64) -> (SparseBinaryMatrix, Vec<F128>, Vec<F128>, MatrixClaim) {
        let n = 1 << log_n;
        let matrix = SparseBinaryMatrix::new(
            n,
            n,
            (0..n)
                .map(|i| {
                    if n == 1 {
                        vec![0]
                    } else if i % 3 == 0 {
                        vec![]
                    } else {
                        vec![(i + 1) % n, (i * 3 + 2) % n]
                    }
                })
                .collect(),
        );
        let mut rng = Rng::new(seed);
        let witness = rng.f128_vec(n);
        let image = sparse_apply(&matrix, &witness);
        let mut row_factors: Vec<_> = (0..log_n).map(|_| [rng.f128(), rng.f128()]).collect();
        if log_n > 1 {
            row_factors[0] = [rng.f128(); 2];
        }
        let column_weights = rng.f128_vec(WIDTH);
        let columns = fold_1b_rows_naive(&image, &tensor_high(&row_factors));
        let value = dot(&columns, &column_weights);
        (
            matrix,
            witness,
            image,
            MatrixClaim {
                row_factors,
                column_weights,
                value,
            },
        )
    }

    fn check_sources(matrix: &SparseBinaryMatrix, witness: &[F128], c: &Claims) {
        assert_eq!(dot(witness, &eq_low(&c.witness.point)), c.witness.value);
        assert_eq!(
            dot(
                &eq_low(&c.matrix.row_point),
                &sparse_apply(matrix, &eq_low(&c.matrix.column_point))
            ),
            c.matrix.value
        );
        assert_eq!(c.witness.point, c.matrix.column_point);
    }

    #[test]
    fn nonidentity_roundtrip_all_endpoints_and_low_first_points() {
        for log_n in [0, 1, 3, 7, 9] {
            let (matrix, witness, image, source) = fixture(log_n, 0xA900 + log_n as u64);
            let mut pc = FsChallenger::new(b"multiple-image-roundtrip");
            let (proof, c) = prove(&matrix, &witness, &image, &source, &mut pc).unwrap();
            check_sources(&matrix, &witness, &c);
            let mut vc = FsChallenger::new(b"multiple-image-roundtrip");
            assert_eq!(verify(log_n, &source, &proof, &mut vc).unwrap(), c);
            assert_eq!(pc.sample_f128(), vc.sample_f128());
        }
    }

    #[test]
    fn public_frobenius_recursion_matches_direct_coordinate_projection() {
        let mut rng = Rng::new(0xA901);
        for log_n in [0, 1, 4, 9] {
            let projection = rng.f128_vec(WIDTH);
            let mut source = MatrixClaim {
                row_factors: (0..log_n).map(|_| [rng.f128(), rng.f128()]).collect(),
                column_weights: vec![F128::ZERO; WIDTH],
                value: F128::ZERO,
            };
            if log_n > 1 {
                source.row_factors[0] = [F128::ONE; 2];
                source.row_factors[1] = [F128::ZERO, F128::ONE];
            }
            for mode in 0..3 {
                let point = if mode == 0 {
                    vec![F128::ZERO; log_n]
                } else if mode == 1 {
                    vec![F128::ONE; log_n]
                } else {
                    rng.f128_vec(log_n)
                };
                let table = build_fold_byte_table(&projection);
                let direct: Vec<_> = tensor_high(&source.row_factors)
                    .into_iter()
                    .map(|v| project_coordinate(v, &table))
                    .collect();
                assert_eq!(
                    evaluate_public_query(&source, &projection, &point),
                    dot(&eq_low(&point), &direct)
                );
            }
            if log_n > 0 {
                source.row_factors[log_n - 1] = [F128::ZERO; 2];
                assert_eq!(
                    evaluate_public_query(&source, &projection, &rng.f128_vec(log_n)),
                    F128::ZERO
                );
            }
        }
    }

    #[test]
    fn columns_rounds_endpoints_and_source_tampering_rejected() {
        let log_n = 4;
        let (matrix, witness, image, source) = fixture(log_n, 0xA902);
        let (proof, _) = prove(
            &matrix,
            &witness,
            &image,
            &source,
            &mut FsChallenger::new(b"multiple-image-tamper"),
        )
        .unwrap();
        for kind in 0..9 {
            let mut p = proof.clone();
            match kind {
                0 => p.columns[17] += F128::ONE,
                1 => p.ip1.rounds[1][0] += F128::ONE,
                2 => p.ip2.rounds[2][1] += F128::ONE,
                3 => p.ip1.final_a += F128::ONE,
                4 => p.ip1.final_b += F128::ONE,
                5 => p.ip2.final_a += F128::ONE,
                6 => p.ip2.final_b += F128::ONE,
                7 => {
                    p.ip1.rounds.pop();
                }
                _ => {
                    p.columns.pop();
                }
            }
            assert!(
                verify(
                    log_n,
                    &source,
                    &p,
                    &mut FsChallenger::new(b"multiple-image-tamper")
                )
                .is_err(),
                "kind={kind}"
            );
        }
        let mut wrong = source.clone();
        wrong.row_factors.reverse();
        assert!(
            verify(
                log_n,
                &wrong,
                &proof,
                &mut FsChallenger::new(b"multiple-image-tamper")
            )
            .is_err()
        );
        let mut wrong = source.clone();
        wrong.value += F128::ONE;
        assert!(
            prove(
                &matrix,
                &witness,
                &image,
                &wrong,
                &mut FsChallenger::new(b"multiple-image-tamper")
            )
            .is_err()
        );
        let mut wrong = image;
        wrong[0] += F128::ONE;
        assert!(
            prove(
                &matrix,
                &witness,
                &wrong,
                &source,
                &mut FsChallenger::new(b"multiple-image-tamper")
            )
            .is_err()
        );
    }

    fn batch_fixture(
        log_n: usize,
        count: usize,
    ) -> (
        SparseBinaryMatrix,
        Vec<Vec<F128>>,
        Vec<Vec<F128>>,
        Vec<MatrixClaim>,
    ) {
        let (matrix, _, _, first) = fixture(log_n, 0xBABA_0100);
        let mut witnesses = Vec::new();
        let mut images = Vec::new();
        let mut sources = Vec::new();
        for i in 0..count {
            let (_, witness, image, mut source) = fixture(log_n, 0xBABA_0200 + i as u64);
            source.row_factors = first.row_factors.clone();
            source.value = dot(
                &fold_1b_rows_naive(&image, &tensor_high(&source.row_factors)),
                &source.column_weights,
            );
            witnesses.push(witness);
            images.push(image);
            sources.push(source);
        }
        (matrix, witnesses, images, sources)
    }

    #[test]
    fn batch_projection_and_terminals_match_independent_sources() {
        for (log_n, count) in [(0, 2), (4, 1), (4, 2), (5, 3)] {
            let (matrix, witnesses, images, sources) = batch_fixture(log_n, count);
            let ws: Vec<_> = witnesses.iter().map(Vec::as_slice).collect();
            let ims: Vec<_> = images.iter().map(Vec::as_slice).collect();
            let mut pc = FsChallenger::new(b"batch-image-test");
            let (proof, c) = prove_batch(&matrix, &ws, &ims, &sources, &mut pc).unwrap();
            let mut vc = FsChallenger::new(b"batch-image-test");
            assert_eq!(verify_batch(log_n, &sources, &proof, &mut vc).unwrap(), c);
            assert_eq!(pc.sample_f128(), vc.sample_f128());
            let combined: Vec<_> = (0..1 << log_n)
                .map(|i| {
                    witnesses
                        .iter()
                        .zip(&c.weights)
                        .fold(F128::ZERO, |s, (w, &z)| s + w[i] * z)
                })
                .collect();
            check_sources(
                &matrix,
                &combined,
                &Claims {
                    witness: c.witness,
                    matrix: c.matrix,
                },
            );
        }
        // Demonstrate why F128-weighted packed columns cannot be transposed
        // after combination: the coordinate transpose is only F2-linear.
        let mut rng = Rng::new(0xBA7C_00F2);
        let left = rng.f128_vec(WIDTH);
        let right = rng.f128_vec(WIDTH);
        let r = rng.f128_vec(WIDTH);
        let z = [rng.f128(), rng.f128()];
        let wrong: Vec<_> = left
            .iter()
            .zip(&right)
            .map(|(&a, &b)| z[0] * a + z[1] * b)
            .collect();
        assert_ne!(
            projected_claim(&wrong, &r),
            z[0] * projected_claim(&left, &r) + z[1] * projected_claim(&right, &r)
        );
    }

    #[test]
    fn batch_fusion_preserves_bytes_claims_and_transcript() {
        for threads in [1, 3] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            pool.install(|| {
                for (log_n, count) in [(0, 2), (1, 2), (2, 3), (7, 1), (9, 3), (15, 2)] {
                    let (matrix, witnesses, images, sources) = batch_fixture(log_n, count);
                    let ws: Vec<_> = witnesses.iter().map(Vec::as_slice).collect();
                    let ims: Vec<_> = images.iter().map(Vec::as_slice).collect();
                    let domain = b"batch-image-fusion-equivalence";
                    let mut reference_ch = FsChallenger::new(domain);
                    let reference = prove_batch_with_fusion(
                        &matrix,
                        &ws,
                        &ims,
                        &sources,
                        false,
                        &mut reference_ch,
                    )
                    .unwrap();
                    let mut fused_ch = FsChallenger::new(domain);
                    let fused =
                        prove_batch_with_fusion(&matrix, &ws, &ims, &sources, true, &mut fused_ch)
                            .unwrap();
                    assert_eq!(
                        bincode::serialize(&fused.0).unwrap(),
                        bincode::serialize(&reference.0).unwrap(),
                        "proof bytes: threads={threads}, log_n={log_n}, count={count}"
                    );
                    assert_eq!(fused.1, reference.1);
                    let mut verifier_ch = FsChallenger::new(domain);
                    assert_eq!(
                        verify_batch(log_n, &sources, &fused.0, &mut verifier_ch).unwrap(),
                        fused.1
                    );
                    let next = fused_ch.sample_f128_vec(8);
                    assert_eq!(next, reference_ch.sample_f128_vec(8));
                    assert_eq!(next, verifier_ch.sample_f128_vec(8));
                }
            });
        }
    }

    #[test]
    fn batch_fusion_preserves_projected_identity_selfcheck() {
        let (matrix, mut witnesses, images, sources) = batch_fixture(4, 2);
        witnesses[0][0] += F128::ONE;
        let ws: Vec<_> = witnesses.iter().map(Vec::as_slice).collect();
        let ims: Vec<_> = images.iter().map(Vec::as_slice).collect();
        let mut reference_ch = FsChallenger::new(b"batch-image-fusion-selfcheck");
        let mut fused_ch = FsChallenger::new(b"batch-image-fusion-selfcheck");
        let reference =
            prove_batch_with_fusion(&matrix, &ws, &ims, &sources, false, &mut reference_ch);
        let fused = prove_batch_with_fusion(&matrix, &ws, &ims, &sources, true, &mut fused_ch);
        assert_eq!(reference, Err("image batch projected identity".into()));
        assert_eq!(fused, reference);
        assert_eq!(fused_ch.sample_f128_vec(8), reference_ch.sample_f128_vec(8));
    }

    #[test]
    fn batch_columns_endpoints_swaps_and_different_queries_rejected() {
        let log_n = 4;
        let (matrix, witnesses, images, sources) = batch_fixture(log_n, 2);
        let ws: Vec<_> = witnesses.iter().map(Vec::as_slice).collect();
        let ims: Vec<_> = images.iter().map(Vec::as_slice).collect();
        let (proof, _) = prove_batch(
            &matrix,
            &ws,
            &ims,
            &sources,
            &mut FsChallenger::new(b"batch-image-tamper"),
        )
        .unwrap();
        for kind in 0..8 {
            let mut bad = proof.clone();
            match kind {
                0 => bad.columns.swap(0, 1),
                1 => bad.columns[1][0] += F128::ONE,
                2 => bad.ip1.final_a += F128::ONE,
                3 => bad.ip1.final_b += F128::ONE,
                4 => bad.ip2.final_a += F128::ONE,
                5 => bad.ip2.final_b += F128::ONE,
                6 => {
                    bad.ip2.rounds.pop();
                }
                _ => {
                    bad.columns.pop();
                }
            }
            assert!(
                verify_batch(
                    log_n,
                    &sources,
                    &bad,
                    &mut FsChallenger::new(b"batch-image-tamper")
                )
                .is_err(),
                "kind={kind}"
            );
        }
        let mut wrong = sources.clone();
        wrong[1].row_factors[0][0] += F128::ONE;
        assert!(
            verify_batch(
                log_n,
                &wrong,
                &proof,
                &mut FsChallenger::new(b"batch-image-tamper")
            )
            .is_err()
        );
        assert!(
            prove_batch(
                &matrix,
                &ws,
                &ims,
                &wrong,
                &mut FsChallenger::new(b"batch-image-tamper")
            )
            .is_err()
        );
        let swapped = [sources[1].clone(), sources[0].clone()];
        assert!(
            verify_batch(
                log_n,
                &swapped,
                &proof,
                &mut FsChallenger::new(b"batch-image-tamper")
            )
            .is_err()
        );
    }
}
