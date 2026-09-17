//! Univariate Hadamard reduction with terminal packed-opening claims.
//!
//! Inputs are packed bits. The residual has degree at most 254 and its quotient
//! has degree at most 126. The caller authenticates the returned claims with PCS.

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

#[derive(Clone, Copy, Debug)]
struct SharedBatchTuning {
    wide_pack: bool,
    group_tiles: bool,
    tile_rows: usize,
}

impl Default for SharedBatchTuning {
    fn default() -> Self {
        Self {
            wide_pack: false,
            group_tiles: false,
            tile_rows: 1024,
        }
    }
}

#[cfg(test)]
std::thread_local! {
    static SHARED_BATCH_OVERRIDE: std::cell::Cell<Option<SharedBatchTuning>> = const {
        std::cell::Cell::new(None)
    };
}

fn shared_batch_tuning() -> Result<SharedBatchTuning, Error> {
    #[cfg(test)]
    if let Some(tuning) = SHARED_BATCH_OVERRIDE.with(|slot| slot.get()) {
        return Ok(tuning);
    }
    let enabled = |name| std::env::var_os(name).is_some_and(|value| value == "1");
    let mut tuning = SharedBatchTuning {
        wide_pack: enabled("MIKROS_HADAMARD_WIDE_PACK"),
        group_tiles: std::env::var_os("MIKROS_HADAMARD_GROUP_TILES")
            .is_none_or(|value| value == "1"),
        tile_rows: 2048,
    };
    // The tile option does not change or reject the legacy scheduling path.
    if tuning.group_tiles {
        if let Ok(value) = std::env::var("MIKROS_HADAMARD_TILE_ROWS") {
            tuning.tile_rows = value
                .parse()
                .map_err(|_| Error::ProofShape("Hadamard tile rows must be 512, 1024, or 2048"))?;
        }
        if !matches!(tuning.tile_rows, 512 | 1024 | 2048) {
            return Err(Error::ProofShape(
                "Hadamard tile rows must be 512, 1024, or 2048",
            ));
        }
    }
    Ok(tuning)
}

#[cfg(test)]
fn with_shared_batch_override<T>(tuning: SharedBatchTuning, run: impl FnOnce() -> T) -> T {
    struct Restore(Option<SharedBatchTuning>);
    impl Drop for Restore {
        fn drop(&mut self) {
            SHARED_BATCH_OVERRIDE.with(|slot| slot.set(self.0));
        }
    }
    let _restore = Restore(SHARED_BATCH_OVERRIDE.with(|slot| slot.replace(Some(tuning))));
    run()
}

fn batch_clock<const PROFILE: bool>() -> Option<Instant> {
    if PROFILE { Some(Instant::now()) } else { None }
}

fn batch_elapsed<const PROFILE: bool>(start: Option<Instant>) -> f64 {
    if PROFILE {
        start.expect("batch profile clock").elapsed().as_secs_f64() * 1000.0
    } else {
        0.0
    }
}

#[derive(Default, Serialize)]
struct SharedBlockProfile {
    // Joint aggregation has one total time, not an invented per-block split.
    aggregate_ms: Option<f64>,
    c_project_ms: f64,
    quotient_ms: f64,
    packing_ms: f64,
}

fn start_batch<C: Challenger>(
    shape: Shape,
    count: usize,
    ch: &mut C,
) -> (Vec<F128>, Vec<Vec<F128>>) {
    ch.observe_label(b"mikros-hadamard-shared-rows-independent-columns-v1");
    ch.observe_bytes(&(shape.log_n as u64).to_le_bytes());
    ch.observe_bytes(&(KAPPA as u64).to_le_bytes());
    ch.observe_bytes(&(count as u64).to_le_bytes());
    let row = ch.sample_f128_vec(shape.row_log);
    let columns = (0..count).map(|_| ch.sample_f128_vec(7)).collect();
    (row, columns)
}

/// Separate Hadamard constraints, jointly scheduled to share the row tensor
/// and the IP terminal point. Column MLE and outside-domain challenges remain
/// independent. Bind every source commitment before entry, then authenticate
/// every returned source claim. This transcript differs from `prove`.
pub fn prove_batch_shared_rows<C: Challenger>(
    a: &[&[u8]],
    b: &[&[u8]],
    c: &[&[u8]],
    log_n: usize,
    kernel: Kernel,
    ch: &mut C,
) -> Result<(Vec<Proof>, Vec<Claims>), Error> {
    let tuning = shared_batch_tuning()?;
    if std::env::var_os("MIKROS_PROFILE").is_some_and(|value| value == "1") {
        prove_batch_shared_rows_impl::<true, C>(a, b, c, log_n, kernel, tuning, ch)
    } else {
        prove_batch_shared_rows_impl::<false, C>(a, b, c, log_n, kernel, tuning, ch)
    }
}

fn prove_batch_shared_rows_impl<const PROFILE: bool, C: Challenger>(
    a: &[&[u8]],
    b: &[&[u8]],
    c: &[&[u8]],
    log_n: usize,
    kernel: Kernel,
    tuning: SharedBatchTuning,
    ch: &mut C,
) -> Result<(Vec<Proof>, Vec<Claims>), Error> {
    let started = batch_clock::<PROFILE>();
    let shape = Shape::new(log_n)?;
    if a.is_empty() || a.len() != b.len() || a.len() != c.len() {
        return Err(Error::ProofShape("Hadamard batch"));
    }
    for i in 0..a.len() {
        shape.check_inputs(a[i], b[i], c[i])?;
    }
    let (row, columns) = start_batch(shape, a.len(), ch);
    let mut l = Vec::new();
    crate::weights::fill(&row, &mut l);
    let domain = domain();
    if kernel.is_coefficient() {
        univariate::coefficient::prepare_kernel(kernel);
    }
    let group_tiles = tuning.group_tiles && !kernel.is_coefficient();
    let wide_pack = tuning.wide_pack || kernel == Kernel::RegisterPacked;
    let mut profiles: Vec<_> = (0..a.len())
        .map(|_| SharedBlockProfile::default())
        .collect();
    let mut aggregate_ms = 0.0;
    let mut grouped_evaluations = if group_tiles {
        let begin = batch_clock::<PROFILE>();
        let values = univariate::aggregate_groups(a, b, &l, kernel, tuning.tile_rows);
        aggregate_ms = batch_elapsed::<PROFILE>(begin);
        Some(values)
    } else {
        None
    };
    let mut proofs = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        let evaluations = if let Some(values) = &mut grouped_evaluations {
            std::mem::take(&mut values[i])
        } else {
            let begin = batch_clock::<PROFILE>();
            let values = if kernel.is_coefficient() {
                univariate::coefficient::aggregate_with_kernel(a[i], b[i], &l, kernel)
            } else {
                univariate::aggregate(a[i], b[i], &l, kernel)
            };
            let elapsed = batch_elapsed::<PROFILE>(begin);
            profiles[i].aggregate_ms = Some(elapsed);
            aggregate_ms += elapsed;
            values
        };
        let begin = batch_clock::<PROFILE>();
        let projected_c = crate::lookup::project_rows_native(c[i], &weights(&columns[i]), &l);
        profiles[i].c_project_ms = batch_elapsed::<PROFILE>(begin);
        let begin = batch_clock::<PROFILE>();
        let column_claims = evaluations[..KAPPA]
            .iter()
            .map(|x| field().compose(x))
            .collect();
        let quotients = if kernel.is_coefficient() {
            univariate::coefficient::quotient(&evaluations)
        } else {
            domain.quotient(&evaluations)
        };
        profiles[i].quotient_ms = batch_elapsed::<PROFILE>(begin);
        let proof = Proof {
            projected_c,
            column_claims,
            quotients,
            inner_product: InnerProductProof {
                rounds: Vec::new(),
                final_a: F128::ZERO,
                final_b: F128::ZERO,
            },
        };
        observe_certificate(&proof, ch);
        proofs.push(proof);
    }
    // All certificates are fixed before any outside-domain challenge.
    let rhos: Vec<_> = (0..a.len())
        .map(|_| domain.lagrange_at(sample_outside_h(ch)))
        .collect();
    let mut scratch: Vec<_> = (0..a.len())
        .map(|_| inner_product::ProverScratch::default())
        .collect();
    let pack = if wide_pack {
        crate::lookup::pack_pair_weighted_with_wide_message_into
    } else {
        crate::lookup::pack_pair_weighted_with_message_into
    };
    let mut messages = Vec::with_capacity(a.len());
    for i in 0..a.len() {
        let begin = batch_clock::<PROFILE>();
        let s = &mut scratch[i];
        messages.push(Some(pack(a[i], b[i], &rhos[i], &l, &mut s.a, &mut s.b)));
        profiles[i].packing_ms = batch_elapsed::<PROFILE>(begin);
    }
    let begin = batch_clock::<PROFILE>();
    let (ips, y) = inner_product::prove_native_batch(&mut scratch, &messages, ch);
    let shared_ip_ms = batch_elapsed::<PROFILE>(begin);
    let mut claims = Vec::with_capacity(a.len());
    for (i, (ip, rho)) in ips.into_iter().zip(rhos).enumerate() {
        proofs[i].inner_product = ip;
        claims.push(claims_from_points(&proofs[i], &row, &columns[i], rho, &y));
    }
    if PROFILE {
        eprintln!(
            "{}",
            serde_json::json!({
                "record": "mikros_hadamard_shared_batch_profile",
                "blocks": a.len(), "rows_per_block": l.len(),
                "kernel": format!("{kernel:?}"), "wide_pack": wide_pack,
                "group_tiles": group_tiles,
                "tile_rows": if group_tiles { tuning.tile_rows } else { 2048 },
                "aggregate_ms": aggregate_ms,
                "c_project_ms": profiles.iter().map(|p| p.c_project_ms).sum::<f64>(),
                "quotient_ms": profiles.iter().map(|p| p.quotient_ms).sum::<f64>(),
                "packing_ms": profiles.iter().map(|p| p.packing_ms).sum::<f64>(),
                "shared_ip_ms": shared_ip_ms,
                "total_ms": batch_elapsed::<PROFILE>(started),
                "per_block": profiles,
            })
        );
    }
    Ok((proofs, claims))
}

pub fn verify_batch_shared_rows<C: Challenger>(
    proofs: &[Proof],
    log_n: usize,
    ch: &mut C,
) -> Result<Vec<Claims>, Error> {
    let shape = Shape::new(log_n)?;
    if proofs.is_empty()
        || proofs.iter().any(|p| {
            p.column_claims.len() != KAPPA
                || p.quotients.len() != QUOTIENTS
                || p.inner_product.rounds.len() != shape.row_log
        })
    {
        return Err(Error::ProofShape("Hadamard batch"));
    }
    let (row, columns) = start_batch(shape, proofs.len(), ch);
    for (proof, column) in proofs.iter().zip(&columns) {
        let projected = proof
            .column_claims
            .iter()
            .zip(weights(column))
            .fold(F128::ZERO, |sum, (&v, weight)| sum + v * weight);
        if projected != proof.projected_c {
            return Err(Error::ColumnProjectionMismatch);
        }
        observe_certificate(proof, ch);
    }
    let mut rhos = Vec::with_capacity(proofs.len());
    let mut ip_claims = Vec::with_capacity(proofs.len());
    for proof in proofs {
        let x = sample_outside_h(ch);
        let rho = domain().lagrange_at(x);
        let c_x = proof
            .column_claims
            .iter()
            .zip(&rho)
            .fold(F128::ZERO, |sum, (&v, &r)| sum + v * r);
        let h_x = proof
            .quotients
            .iter()
            .rev()
            .fold(F128::ZERO, |sum, &v| sum * x + v);
        ip_claims.push(c_x + domain().vanishing_at(x) * h_x);
        rhos.push(rho);
    }
    let ips: Vec<_> = proofs
        .iter()
        .map(|proof| proof.inner_product.clone())
        .collect();
    let y = inner_product::verify_batch(&ips, &ip_claims, ch)?;
    Ok(proofs
        .iter()
        .zip(columns)
        .zip(rhos)
        .map(|((proof, column), rho)| claims_from_points(proof, &row, &column, rho, &y))
        .collect())
}

#[cfg(test)]
mod shared_batch_tests {
    use super::*;
    use flock_field::test_rng::Rng;
    use flock_transcript::challenger::FsChallenger;

    fn inputs(log_n: usize, count: usize) -> Vec<(Vec<u8>, Vec<u8>, Vec<u8>)> {
        let mut rng = Rng::new(0xBA7C_AA01);
        (0..count)
            .map(|_| {
                let mut a = vec![0; 1 << (log_n - 3)];
                let mut b = a.clone();
                rng.fill_bytes(&mut a);
                rng.fill_bytes(&mut b);
                let c = a.iter().zip(&b).map(|(&a, &b)| a & b).collect();
                (a, b, c)
            })
            .collect()
    }

    #[test]
    fn independent_oracles_shared_rows_distinct_columns_and_transcript() {
        for (log_n, count) in [(7, 1), (10, 2), (12, 3)] {
            let data = inputs(log_n, count);
            let a: Vec<_> = data.iter().map(|x| x.0.as_slice()).collect();
            let b: Vec<_> = data.iter().map(|x| x.1.as_slice()).collect();
            let c: Vec<_> = data.iter().map(|x| x.2.as_slice()).collect();
            let mut pc = FsChallenger::new(b"shared-Hadamard-test");
            let (proofs, claims) =
                prove_batch_shared_rows(&a, &b, &c, log_n, Kernel::PairAggregate, &mut pc).unwrap();
            let mut vc = FsChallenger::new(b"shared-Hadamard-test");
            assert_eq!(
                verify_batch_shared_rows(&proofs, log_n, &mut vc).unwrap(),
                claims
            );
            assert_eq!(pc.sample_f128(), vc.sample_f128());
            for (i, q) in claims.iter().enumerate() {
                check_claims(a[i], b[i], c[i], log_n, q).unwrap();
                assert_eq!(q.a.row_factors, claims[0].a.row_factors);
                assert_eq!(q.b.row_factors, claims[0].b.row_factors);
                assert_eq!(q.c.row_factors, claims[0].c.row_factors);
                if i > 0 {
                    assert_ne!(q.a.column_weights, claims[0].a.column_weights);
                    assert_ne!(q.c.column_weights, claims[0].c.column_weights);
                }
            }
        }
    }

    #[test]
    fn shared_batch_opt_in_preserves_proof_claims_and_transcript() {
        // Full 0/1 inputs, geometric bit patterns, and random data cover empty
        // tails through several group tiles and the parallel-IP threshold.
        for threads in [1, 3] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            pool.install(|| {
                for (log_n, pattern) in [(7, 0), (14, 1), (17, 2), (19, 3)] {
                    let mut data = inputs(log_n, 2);
                    for (group, (a, b, c)) in data.iter_mut().enumerate() {
                        for (index, (a, b)) in a.iter_mut().zip(b.iter_mut()).enumerate() {
                            match pattern {
                                0 => { *a = 0; *b = 0; }
                                1 => { *a = u8::MAX; *b = u8::MAX; }
                                2 => {
                                    *a = 1u8.rotate_left(((index + group) % 8) as u32);
                                    *b = 3u8.rotate_left(((2 * index + group) % 8) as u32);
                                }
                                _ => {}
                            }
                        }
                        *c = a.iter().zip(b.iter()).map(|(&a, &b)| a & b).collect();
                    }
                    let a: Vec<_> = data.iter().map(|x| x.0.as_slice()).collect();
                    let b: Vec<_> = data.iter().map(|x| x.1.as_slice()).collect();
                    let c: Vec<_> = data.iter().map(|x| x.2.as_slice()).collect();
                    let run = |tuning, parallel| {
                        with_shared_batch_override(tuning, || {
                            inner_product::with_batch_parallel_override(parallel, || {
                                let mut pc = FsChallenger::new(b"shared-batch-opt-in-equivalence");
                                let (proofs, claims) = prove_batch_shared_rows(
                                    &a, &b, &c, log_n, Kernel::PairAggregate, &mut pc,
                                ).unwrap();
                                let mut vc = FsChallenger::new(b"shared-batch-opt-in-equivalence");
                                assert_eq!(verify_batch_shared_rows(&proofs, log_n, &mut vc).unwrap(), claims);
                                let future = pc.sample_f128_vec(4);
                                assert_eq!(future, vc.sample_f128_vec(4));
                                (bincode::serialize(&proofs).unwrap(), claims, future)
                            })
                        })
                    };
                    let baseline = run(SharedBatchTuning::default(), false);
                    for (tuning, parallel) in [
                        (SharedBatchTuning { wide_pack: true, ..SharedBatchTuning::default() }, false),
                        (SharedBatchTuning { group_tiles: true, tile_rows: 512, ..SharedBatchTuning::default() }, false),
                        (SharedBatchTuning { group_tiles: true, tile_rows: 1024, ..SharedBatchTuning::default() }, false),
                        (SharedBatchTuning { group_tiles: true, tile_rows: 2048, ..SharedBatchTuning::default() }, false),
                        (SharedBatchTuning { wide_pack: true, group_tiles: true, tile_rows: 512 }, true),
                    ] {
                        assert_eq!(run(tuning, parallel), baseline,
                            "threads={threads}, log_n={log_n}, pattern={pattern}, tuning={tuning:?}, parallel={parallel}");
                    }
                    for (i, claims) in baseline.1.iter().enumerate() {
                        check_claims(a[i], b[i], c[i], log_n, claims).unwrap();
                    }
                }
            });
        }
    }

    #[test]
    fn shared_batch_swaps_messages_endpoints_and_shapes_rejected() {
        let log_n = 11;
        let data = inputs(log_n, 2);
        let a: Vec<_> = data.iter().map(|x| x.0.as_slice()).collect();
        let b: Vec<_> = data.iter().map(|x| x.1.as_slice()).collect();
        let c: Vec<_> = data.iter().map(|x| x.2.as_slice()).collect();
        let (proofs, _) = prove_batch_shared_rows(
            &a,
            &b,
            &c,
            log_n,
            Kernel::PairAggregate,
            &mut FsChallenger::new(b"shared-Hadamard-tamper"),
        )
        .unwrap();
        for kind in 0..7 {
            let mut bad = proofs.clone();
            match kind {
                0 => bad.swap(0, 1),
                1 => bad[1].column_claims[0] += F128::ONE,
                2 => bad[0].quotients[1] += F128::ONE,
                3 => bad[1].inner_product.rounds[0][0] += F128::ONE,
                4 => bad[0].inner_product.final_a += F128::ONE,
                5 => bad[1].inner_product.final_b += F128::ONE,
                _ => {
                    bad[0].inner_product.rounds.pop();
                }
            }
            assert!(
                verify_batch_shared_rows(
                    &bad,
                    log_n,
                    &mut FsChallenger::new(b"shared-Hadamard-tamper")
                )
                .is_err(),
                "kind={kind}"
            );
        }
        assert!(verify_batch_shared_rows(&[], log_n, &mut FsChallenger::new(b"empty")).is_err());
    }
}
