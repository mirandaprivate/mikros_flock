//! Binary weighted Hadamard reduction from `latex/alg/alg-hadamard.tex`.
//!
//! The public statement is `value = l^T (A o B) r`, where `r` contains 128
//! arbitrary extension-field weights and `l` is a tensor of arbitrary pairs.
//! Both inputs contain consecutive 128-bit rows. Row factors are high-bit
//! first, exactly as in [`MatrixClaim`]. No normalization or inverse of a row
//! factor is used, including when a pair is zero or sums to zero.
//!
//! Bind the source commitments before entry and authenticate both returned
//! matrix claims. This reduction does not itself supply a commitment scheme.

use crate::{
    Error, MatrixClaim,
    inner_product::{self, InnerProductProof},
    layout::Shape,
    univariate::{self, KAPPA, Kernel, QUOTIENTS, domain::domain, field::field},
};
use flock_field::F128;
use flock_transcript::challenger::Challenger;
use rayon::prelude::{IndexedParallelIterator, ParallelIterator, ParallelSliceMut};
use serde::{Deserialize, Serialize};

/// The 128 weighted column values, the 127 monomial quotient coefficients,
/// and the length-`m` inner-product proof. The scalar claim is public input.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    pub column_claims: Vec<F128>,
    pub quotients: Vec<F128>,
    pub inner_product: InnerProductProof,
}

impl Proof {
    pub fn field_elements(&self) -> usize {
        self.column_claims.len() + self.quotients.len() + 2 * self.inner_product.rounds.len() + 2
    }
}

fn checked_shape(
    log_n: usize,
    row_factors: &[[F128; 2]],
    column_weights: &[F128],
) -> Result<Shape, Error> {
    let shape = Shape::new(log_n)?;
    if row_factors.len() != shape.row_log || column_weights.len() != KAPPA {
        return Err(Error::ProofShape("weighted Hadamard query"));
    }
    Ok(shape)
}

fn bind_statement<C: Challenger>(
    shape: Shape,
    row_factors: &[[F128; 2]],
    column_weights: &[F128],
    value: F128,
    challenger: &mut C,
) {
    challenger.observe_label(b"mikros-binary-weighted-hadamard-univariate-gf256-v1");
    challenger.observe_bytes(&(shape.log_n as u64).to_le_bytes());
    challenger.observe_bytes(&(KAPPA as u64).to_le_bytes());
    for pair in row_factors {
        challenger.observe_f128_slice(pair);
    }
    challenger.observe_f128_slice(column_weights);
    challenger.observe_f128(value);
}

fn observe_certificate<C: Challenger>(proof: &Proof, challenger: &mut C) {
    challenger.observe_label(b"mikros-weighted-univariate-column-and-quotient-v1");
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

fn project(columns: &[F128], weights: &[F128]) -> F128 {
    columns
        .iter()
        .zip(weights)
        .fold(F128::ZERO, |sum, (&column, &weight)| sum + column * weight)
}

fn small_weights(high_first: &[[F128; 2]]) -> Vec<F128> {
    // Reuse the native PCS's arbitrary-factor tensor construction, converting
    // only its public coordinate order. In particular [t,t] stays [t,t].
    let low_first: Vec<_> = high_first.iter().rev().copied().collect();
    flock_core::pcs::functional::row_weights(&low_first)
}

fn row_weights(high_first: &[[F128; 2]]) -> Vec<F128> {
    if high_first.len() <= 10 {
        return small_weights(high_first);
    }
    // A single large allocation avoids retaining a half-size previous tensor
    // during each doubling. At log_n=37 this output is 16 GiB; the two small
    // tensors need about 16 MiB and 16 KiB instead of a second 8 GiB buffer.
    let split = high_first.len() - 10;
    let outer = small_weights(&high_first[..split]);
    let inner = small_weights(&high_first[split..]);
    let len = outer.len() * inner.len();
    let mut output = Vec::new();
    let storage = inner_product::prepare_output(&mut output, len);
    storage
        .par_chunks_exact_mut(inner.len())
        .enumerate()
        .for_each(|(block, out)| {
            let factor = outer[block];
            for (destination, &value) in out.iter_mut().zip(&inner) {
                destination.write(value * factor);
            }
        });
    // SAFETY: every disjoint output chunk has been completely initialized,
    // and Rayon has joined all workers. On panic output still has length zero.
    unsafe { output.set_len(len) };
    output
}

#[cfg(test)]
thread_local! {
    static TILED_WEIGHTS_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
}

fn tiled_weights_enabled() -> bool {
    #[cfg(test)]
    if let Some(enabled) = TILED_WEIGHTS_OVERRIDE.get() {
        return enabled;
    }
    std::env::var_os("MIKROS_WH_TILED_WEIGHTS").is_some_and(|value| value == "1")
}

#[cfg(test)]
fn with_tiled_weights<T>(enabled: bool, run: impl FnOnce() -> T) -> T {
    struct Reset(Option<bool>);
    impl Drop for Reset {
        fn drop(&mut self) {
            TILED_WEIGHTS_OVERRIDE.set(self.0);
        }
    }
    let _reset = Reset(TILED_WEIGHTS_OVERRIDE.replace(Some(enabled)));
    run()
}

/// Keep a 1024-entry low tensor and a short high tensor. A consumer generates
/// only its current tile, so aggregate and packing never retain all row weights.
/// These are arbitrary factors, not normalized equality weights: no division,
/// inverse, or assumption that the two entries sum to one is valid here.
struct TiledRowWeights {
    outer: Vec<F128>,
    inner: Vec<F128>,
}

impl TiledRowWeights {
    fn new(high_first: &[[F128; 2]]) -> Self {
        let split = high_first.len().saturating_sub(10);
        Self {
            outer: small_weights(&high_first[..split]),
            inner: small_weights(&high_first[split..]),
        }
    }

    fn fill(&self, start: usize, out: &mut [F128]) {
        assert!(start <= self.outer.len() * self.inner.len());
        assert!(out.len() <= self.outer.len() * self.inner.len() - start);
        let mut written = 0;
        while written < out.len() {
            let row = start + written;
            let low = row % self.inner.len();
            let count = (self.inner.len() - low).min(out.len() - written);
            let source = &self.inner[low..low + count];
            let output = &mut out[written..written + count];
            let factor = self.outer[row / self.inner.len()];
            scale_tile(source, factor, output);
            written += count;
        }
    }
}

fn scale_tile(source: &[F128], factor: F128, output: &mut [F128]) {
    assert_eq!(source.len(), output.len());
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "pclmulqdq",
        target_feature = "sse4.1"
    ))]
    {
        use flock_field::gf2_128::x86_64::{f128x4_loadu, f128x4_set, ghash_mul_x4};
        // SAFETY: the compile-time gates match Flock's field kernel. Each
        // unaligned load/store covers four initialized, in-bounds entries.
        unsafe {
            let factor4 = f128x4_set(factor, factor, factor, factor);
            let bulk = source.len() / 4 * 4;
            for offset in (0..bulk).step_by(4) {
                core::arch::x86_64::_mm512_storeu_si512(
                    output.as_mut_ptr().add(offset).cast(),
                    ghash_mul_x4(f128x4_loadu(source.as_ptr().add(offset)), factor4),
                );
            }
            for (value, destination) in source[bulk..].iter().zip(&mut output[bulk..]) {
                *destination = *value * factor;
            }
        }
    }
    #[cfg(not(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "vpclmulqdq",
        target_feature = "pclmulqdq",
        target_feature = "sse4.1"
    )))]
    for (value, destination) in source.iter().zip(output) {
        *destination = *value * factor;
    }
}

fn terminal_claims(
    proof: &Proof,
    row_factors: &[[F128; 2]],
    rho: Vec<F128>,
    y: &[F128],
) -> [MatrixClaim; 2] {
    let chi: Vec<_> = y.iter().map(|&x| [F128::ONE + x, x]).collect();
    let weighted_chi = row_factors
        .iter()
        .zip(&chi)
        .map(|(l, chi)| [l[0] * chi[0], l[1] * chi[1]])
        .collect();
    [
        MatrixClaim {
            column_weights: rho.clone(),
            row_factors: chi,
            value: proof.inner_product.final_a,
        },
        MatrixClaim {
            column_weights: rho,
            row_factors: weighted_chi,
            value: proof.inner_product.final_b,
        },
    ]
}

/// Prove a fixed weighted scalar claim, without a third input matrix.
///
/// `log_n` counts input bits, so the matrix has `2^(log_n-7)` rows. All query
/// data and the scalar are bound before the certificate and its fresh column
/// challenge. `Kernel::PairAggregate` is the current native interpolation
/// kernel; coefficient/convolution kernels are not accepted by this API.
pub fn prove<C: Challenger>(
    a: &[u8],
    b: &[u8],
    log_n: usize,
    row_factors: &[[F128; 2]],
    column_weights: &[F128],
    value: F128,
    kernel: Kernel,
    challenger: &mut C,
) -> Result<(Proof, [MatrixClaim; 2]), Error> {
    prove_with_fold_buffers(
        a,
        b,
        log_n,
        row_factors,
        column_weights,
        value,
        kernel,
        [Vec::new(), Vec::new()],
        challenger,
    )
}

/// The identical proof, reusing two caller-owned allocations for the IP fold
/// outputs. Only capacity is reused: all previous field values are discarded
/// before writing. The buffers and every proof-local allocation are released
/// before this call returns, so this does not move work outside online proving.
pub fn prove_with_fold_buffers<C: Challenger>(
    a: &[u8],
    b: &[u8],
    log_n: usize,
    row_factors: &[[F128; 2]],
    column_weights: &[F128],
    value: F128,
    kernel: Kernel,
    fold_buffers: [Vec<F128>; 2],
    challenger: &mut C,
) -> Result<(Proof, [MatrixClaim; 2]), Error> {
    let shape = checked_shape(log_n, row_factors, column_weights)?;
    for (name, input) in [("a", a), ("b", b)] {
        if input.len() != shape.bytes {
            return Err(Error::InputLength {
                input: name,
                expected: shape.bytes,
                got: input.len(),
            });
        }
    }
    if kernel.is_coefficient() {
        return Err(Error::ProofShape("weighted Hadamard interpolation kernel"));
    }
    bind_statement(shape, row_factors, column_weights, value, challenger);
    let trace = std::env::var_os("E2E_PIOP_TRACE").is_some();
    let timer = || trace.then(std::time::Instant::now);
    let elapsed_ms = |start: Option<std::time::Instant>| {
        start.map_or(0.0, |start| start.elapsed().as_secs_f64() * 1000.0)
    };
    let start = timer();
    let tiled = tiled_weights_enabled().then(|| TiledRowWeights::new(row_factors));
    let weights = tiled.is_none().then(|| row_weights(row_factors));
    let weights_ms = elapsed_ms(start);
    let start = timer();
    let evaluations = if let Some(tiled) = &tiled {
        univariate::aggregate_tiled_weights(a, b, kernel, |start, out| tiled.fill(start, out))
    } else {
        univariate::aggregate(a, b, weights.as_deref().unwrap(), kernel)
    };
    let aggregate_ms = elapsed_ms(start);
    let start = timer();
    let column_claims: Vec<_> = evaluations[..KAPPA]
        .iter()
        .map(|coordinates| field().compose(coordinates))
        .collect();
    if project(&column_claims, column_weights) != value {
        return Err(Error::ColumnProjectionMismatch);
    }
    let mut proof = Proof {
        column_claims,
        quotients: domain().quotient(&evaluations),
        inner_product: InnerProductProof {
            rounds: Vec::new(),
            final_a: F128::ZERO,
            final_b: F128::ZERO,
        },
    };
    observe_certificate(&proof, challenger);
    let rho = domain().lagrange_at(sample_outside_h(challenger));
    let certificate_ms = elapsed_ms(start);
    let start = timer();
    let mut scratch = inner_product::ProverScratch::with_fold_buffers(fold_buffers);
    let initial_message = if let Some(tiled) = &tiled {
        crate::lookup::pack_pair_tiled_weights_with_wide_message_into(
            a,
            b,
            &rho,
            |start, out| tiled.fill(start, out),
            &mut scratch.a,
            &mut scratch.b,
        )
    } else {
        crate::lookup::pack_pair_weighted_with_wide_message_into(
            a,
            b,
            &rho,
            weights.as_deref().unwrap(),
            &mut scratch.a,
            &mut scratch.b,
        )
    };
    let pack_ms = elapsed_ms(start);
    let start = timer();
    // The weighted packed B now owns every use of l in the remaining IP.
    // Release the materialized tensor before allocating the IP fold buffers.
    drop(weights);
    let is_tiled = tiled.is_some();
    drop(tiled);
    let weights_release_ms = elapsed_ms(start);
    let start = timer();
    let (ip, y) = inner_product::prove_native(&mut scratch, Some(initial_message), challenger);
    proof.inner_product = ip;
    let claims = terminal_claims(&proof, row_factors, rho, &y);
    drop(scratch);
    if trace {
        eprintln!(
            "mikros_weighted_hadamard log_n={} tiled_weights={} weights_ms={:.6} aggregate_ms={:.6} certificate_ms={:.6} pack_ms={:.6} weights_release_ms={:.6} ip_with_release_ms={:.6}",
            log_n,
            is_tiled,
            weights_ms,
            aggregate_ms,
            certificate_ms,
            pack_ms,
            weights_release_ms,
            elapsed_ms(start),
        );
    }
    Ok((proof, claims))
}

/// Replay the weighted reduction and return its two deferred source queries.
/// The caller must authenticate both queries against the bound A/B sources.
pub fn verify<C: Challenger>(
    proof: &Proof,
    log_n: usize,
    row_factors: &[[F128; 2]],
    column_weights: &[F128],
    value: F128,
    challenger: &mut C,
) -> Result<[MatrixClaim; 2], Error> {
    let shape = checked_shape(log_n, row_factors, column_weights)?;
    if proof.column_claims.len() != KAPPA {
        return Err(Error::ProofShape("columns"));
    }
    if proof.quotients.len() != QUOTIENTS {
        return Err(Error::ProofShape("quotients"));
    }
    if proof.inner_product.rounds.len() != shape.row_log {
        return Err(Error::ProofShape("rounds"));
    }
    bind_statement(shape, row_factors, column_weights, value, challenger);
    if project(&proof.column_claims, column_weights) != value {
        return Err(Error::ColumnProjectionMismatch);
    }
    observe_certificate(proof, challenger);
    let x = sample_outside_h(challenger);
    let rho = domain().lagrange_at(x);
    let h_x = proof
        .quotients
        .iter()
        .rev()
        .fold(F128::ZERO, |sum, &coefficient| sum * x + coefficient);
    let ip_claim = project(&proof.column_claims, &rho) + domain().vanishing_at(x) * h_x;
    let y = inner_product::verify(&proof.inner_product, ip_claim, challenger)?;
    Ok(terminal_claims(proof, row_factors, rho, &y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_field::test_rng::Rng;
    use flock_transcript::challenger::FsChallenger;
    use std::collections::VecDeque;

    fn challenger() -> FsChallenger {
        FsChallenger::new(b"weighted-hadamard-tests-v1")
    }

    fn tensor_entry(row: usize, factors: &[[F128; 2]]) -> F128 {
        factors.iter().enumerate().fold(F128::ONE, |v, (i, p)| {
            v * p[(row >> (factors.len() - i - 1)) & 1]
        })
    }

    fn bit(input: &[u8], row: usize, column: usize) -> bool {
        input[16 * row + column / 8] >> (column % 8) & 1 == 1
    }

    // Direct original-bit oracle: no interpolation, packing table, tensor
    // construction helper, or sumcheck routine is shared with the prover.
    fn explicit_product(a: &[u8], b: &[u8], rows: &[[F128; 2]], cols: &[F128]) -> F128 {
        (0..a.len() / 16).fold(F128::ZERO, |sum, row| {
            let weight = tensor_entry(row, rows);
            sum + (0..KAPPA).fold(F128::ZERO, |value, column| {
                if bit(a, row, column) && bit(b, row, column) {
                    value + weight * cols[column]
                } else {
                    value
                }
            })
        })
    }

    fn explicit_source(input: &[u8], query: &MatrixClaim) -> F128 {
        (0..input.len() / 16).fold(F128::ZERO, |sum, row| {
            let weight = tensor_entry(row, &query.row_factors);
            sum + (0..KAPPA).fold(F128::ZERO, |value, column| {
                if bit(input, row, column) {
                    value + weight * query.column_weights[column]
                } else {
                    value
                }
            })
        })
    }

    fn input(rng: &mut Rng, log_n: usize) -> Vec<u8> {
        let mut bytes = vec![0; 1 << (log_n - 3)];
        rng.fill_bytes(&mut bytes);
        bytes
    }

    fn assert_sources(a: &[u8], b: &[u8], claims: &[MatrixClaim; 2]) {
        assert_eq!(explicit_source(a, &claims[0]), claims[0].value);
        assert_eq!(explicit_source(b, &claims[1]), claims[1].value);
        assert_eq!(claims[0].column_weights, claims[1].column_weights);
    }

    #[test]
    fn arbitrary_zero_and_degenerate_weights_match_direct_bit_oracles() {
        let mut rng = Rng::new(0xA81_7A12);
        for log_n in [7, 8, 10] {
            let a = input(&mut rng, log_n);
            let b = input(&mut rng, log_n);
            for case in 0..5 {
                let mut rows: Vec<_> = (0..log_n - 7).map(|_| [rng.f128(), rng.f128()]).collect();
                let mut columns = rng.f128_vec(KAPPA);
                if !rows.is_empty() {
                    match case {
                        1 => rows[0] = [F128::ZERO, F128::ZERO],
                        2 => rows[0] = [columns[0], columns[0]],
                        3 => rows[0] = [F128::ZERO, F128::ONE],
                        _ => {}
                    }
                }
                if case == 4 {
                    columns.fill(F128::ZERO);
                }
                let value = explicit_product(&a, &b, &rows, &columns);
                let mut prover = challenger();
                let (proof, claims) = prove(
                    &a,
                    &b,
                    log_n,
                    &rows,
                    &columns,
                    value,
                    Kernel::PairAggregate,
                    &mut prover,
                )
                .unwrap();
                assert_eq!(proof.field_elements(), 257 + 2 * (log_n - 7));
                let mut verifier = challenger();
                assert_eq!(
                    verify(&proof, log_n, &rows, &columns, value, &mut verifier).unwrap(),
                    claims
                );
                assert_eq!(prover.sample_f128(), verifier.sample_f128());
                assert_sources(&a, &b, &claims);
                for ((l, chi), weighted) in rows
                    .iter()
                    .zip(&claims[0].row_factors)
                    .zip(&claims[1].row_factors)
                {
                    assert_eq!(*weighted, [l[0] * chi[0], l[1] * chi[1]]);
                }
            }
        }
    }

    #[test]
    fn kernels_and_native_ip_preserve_the_same_proof() {
        let mut rng = Rng::new(0xFA17_E251);
        let log_n = 10;
        let a = input(&mut rng, log_n);
        let b = input(&mut rng, log_n);
        let rows: Vec<_> = (0..3).map(|_| [rng.f128(), rng.f128()]).collect();
        let columns = rng.f128_vec(KAPPA);
        let value = explicit_product(&a, &b, &rows, &columns);
        let run = |kernel| {
            prove(
                &a,
                &b,
                log_n,
                &rows,
                &columns,
                value,
                kernel,
                &mut challenger(),
            )
            .unwrap()
        };
        let (proof, claims) = run(Kernel::Lookup);
        assert_eq!(run(Kernel::PairAggregate), (proof.clone(), claims.clone()));
        let mut replay = challenger();
        bind_statement(
            Shape::new(log_n).unwrap(),
            &rows,
            &columns,
            value,
            &mut replay,
        );
        observe_certificate(&proof, &mut replay);
        let rho = domain().lagrange_at(sample_outside_h(&mut replay));
        let packed_a = crate::lookup::pack_rows(&a, &rho, crate::tensor::Kernel::Reference);
        let mut packed_b = crate::lookup::pack_rows(&b, &rho, crate::tensor::Kernel::Reference);
        for (i, v) in packed_b.iter_mut().enumerate() {
            *v *= tensor_entry(i, &rows);
        }
        let (reference_ip, _) = inner_product::prove_packed(packed_a, packed_b, &mut replay);
        assert_eq!(reference_ip, proof.inner_product);
        assert_sources(&a, &b, &claims);
    }

    #[test]
    fn tampered_messages_queries_and_external_scalar_are_rejected() {
        let mut rng = Rng::new(0xBA0_5CA1A);
        let log_n = 10;
        let a = input(&mut rng, log_n);
        let b = input(&mut rng, log_n);
        let rows: Vec<_> = (0..3).map(|_| [rng.f128(), rng.f128()]).collect();
        let columns = rng.f128_vec(KAPPA);
        let value = explicit_product(&a, &b, &rows, &columns);
        let (proof, claims) = prove(
            &a,
            &b,
            log_n,
            &rows,
            &columns,
            value,
            Kernel::PairAggregate,
            &mut challenger(),
        )
        .unwrap();
        let reject = |proof: &Proof| {
            assert!(verify(proof, log_n, &rows, &columns, value, &mut challenger()).is_err());
        };
        for kind in 0..8 {
            let mut bad = proof.clone();
            match kind {
                0 => bad.column_claims[4] += F128::ONE,
                1 => bad.quotients[3] += F128::ONE,
                2 => bad.inner_product.rounds[0][0] += F128::ONE,
                3 => bad.inner_product.final_a += F128::ONE,
                4 => bad.inner_product.final_b += F128::ONE,
                5 => {
                    bad.column_claims.pop();
                }
                6 => {
                    bad.quotients.push(F128::ZERO);
                }
                7 => {
                    bad.inner_product.rounds.pop();
                }
                _ => unreachable!(),
            }
            reject(&bad);
        }
        assert!(
            verify(
                &proof,
                log_n,
                &rows,
                &columns,
                value + F128::ONE,
                &mut challenger(),
            )
            .is_err()
        );
        assert!(
            prove(
                &a,
                &b,
                log_n,
                &rows,
                &columns,
                value + F128::ONE,
                Kernel::PairAggregate,
                &mut challenger(),
            )
            .is_err()
        );

        // Preserve the scalar column check exactly while changing its public
        // weights: rejection must therefore come from transcript binding.
        let mut changed_columns = columns.clone();
        changed_columns[0] += proof.column_claims[1];
        changed_columns[1] += proof.column_claims[0];
        assert_eq!(project(&proof.column_claims, &changed_columns), value);
        assert!(
            verify(
                &proof,
                log_n,
                &rows,
                &changed_columns,
                value,
                &mut challenger(),
            )
            .is_err()
        );
        let mut changed_rows = rows.clone();
        changed_rows[1][0] += F128::ONE;
        assert!(
            verify(
                &proof,
                log_n,
                &changed_rows,
                &columns,
                value,
                &mut challenger(),
            )
            .is_err()
        );

        // PIOP verification deliberately stops at source queries; the caller
        // must also reject a source changed after commitment.
        let mut changed_a = a.clone();
        changed_a[0] ^= 1;
        let mut changed_b = b.clone();
        changed_b[0] ^= 1;
        assert_ne!(explicit_source(&changed_a, &claims[0]), claims[0].value);
        assert_ne!(explicit_source(&changed_b, &claims[1]), claims[1].value);
    }

    #[derive(Clone)]
    struct Scripted {
        values: VecDeque<F128>,
        draws: usize,
    }

    impl Challenger for Scripted {
        fn observe_f128(&mut self, _value: F128) {}
        fn sample_f128(&mut self) -> F128 {
            self.draws += 1;
            self.values.pop_front().expect("scripted challenge")
        }
        fn fork_from_seed(&self, _seed: [F128; 2], _label: &'static [u8]) -> Self {
            self.clone()
        }
    }

    #[test]
    fn rejects_domain_nodes_and_preserves_zero_one_ip_challenges() {
        let mut rng = Rng::new(0xC0_1A00);
        let a = input(&mut rng, 9);
        let b = input(&mut rng, 9);
        let t = rng.f128();
        let rows = [[t, t], [F128::ZERO, rng.f128()]];
        let columns = rng.f128_vec(KAPPA);
        let value = explicit_product(&a, &b, &rows, &columns);
        let mut outside = rng.f128();
        while domain().contains(outside) {
            outside = rng.f128();
        }
        let script = Scripted {
            values: [F128::ZERO, F128::ONE, outside, F128::ONE, F128::ZERO].into(),
            draws: 0,
        };
        let mut prover = script.clone();
        let (proof, claims) = prove(
            &a,
            &b,
            9,
            &rows,
            &columns,
            value,
            Kernel::PairAggregate,
            &mut prover,
        )
        .unwrap();
        let mut verifier = script;
        assert_eq!(
            verify(&proof, 9, &rows, &columns, value, &mut verifier).unwrap(),
            claims
        );
        assert_eq!(prover.draws, 5);
        assert_eq!(verifier.draws, 5);
        assert_sources(&a, &b, &claims);
        assert_eq!(
            claims[0].row_factors,
            [[F128::ONE, F128::ZERO], [F128::ZERO, F128::ONE]]
        );
    }

    #[test]
    fn dimensions_are_checked_without_allocating_large_witnesses() {
        let columns = vec![F128::ONE; KAPPA];
        assert!(checked_shape(6, &[], &columns).is_err());
        assert!(checked_shape(8, &[], &columns).is_err());
        assert!(checked_shape(7, &[], &columns[..127]).is_err());
        assert!(
            prove(
                &[0; 15],
                &[0; 16],
                7,
                &[],
                &columns,
                F128::ZERO,
                Kernel::PairAggregate,
                &mut challenger(),
            )
            .is_err()
        );
        #[cfg(target_pointer_width = "64")]
        {
            let shape = checked_shape(37, &[[F128::ONE; 2]; 30], &columns).unwrap();
            assert_eq!(shape.rows, 1usize << 30);
            assert_eq!(shape.bytes, 1usize << 34);
        }
    }

    #[test]
    fn blocked_arbitrary_tensor_matches_indexed_factors() {
        let mut rng = Rng::new(0xB10C_A2B1);
        for n in [0, 1, 9, 10, 11, 15] {
            let mut rows: Vec<_> = (0..n).map(|_| [rng.f128(), rng.f128()]).collect();
            if n >= 2 {
                rows[1] = [rows[1][0]; 2];
            }
            let actual = row_weights(&rows);
            for (i, &value) in actual.iter().enumerate() {
                assert_eq!(value, tensor_entry(i, &rows));
            }
        }
    }

    #[test]
    fn tiled_weights_match_arbitrary_tensor_at_unaligned_boundaries() {
        let mut rng = Rng::new(0x71_1ED_0A11);
        for n in [0, 1, 9, 10, 11, 15] {
            let mut rows: Vec<_> = (0..n).map(|_| [rng.f128(), rng.f128()]).collect();
            if n >= 2 {
                rows[0] = [rows[0][0]; 2];
                rows[n - 1][0] = F128::ZERO;
            }
            let tiled = TiledRowWeights::new(&rows);
            let expected = row_weights(&rows);
            for start in [0, 1, 3, 1021, 1023, 1024, 2047] {
                if start > expected.len() {
                    continue;
                }
                for len in [0, 1, 3, 8, 127, 1024, 2049] {
                    let len = len.min(expected.len() - start);
                    let mut actual = vec![rng.f128(); len];
                    tiled.fill(start, &mut actual);
                    assert_eq!(actual, expected[start..start + len], "n={n}, start={start}");
                }
            }
        }
    }

    #[test]
    fn tiled_weights_preserve_proof_claims_and_next_challenge() {
        let mut rng = Rng::new(0x71_1ED_FA17);
        for log_n in [7, 8, 10, 18, 20] {
            let a = input(&mut rng, log_n);
            let b = input(&mut rng, log_n);
            let mut rows: Vec<_> = (0..log_n - 7).map(|_| [rng.f128(), rng.f128()]).collect();
            if rows.len() >= 2 {
                rows[0] = [rows[0][0]; 2];
                rows[1][0] = F128::ZERO;
            }
            let columns = rng.f128_vec(KAPPA);
            for zero_tensor in [false, true] {
                if zero_tensor {
                    if rows.is_empty() {
                        continue;
                    }
                    *rows.last_mut().unwrap() = [F128::ZERO; 2];
                }
                let value = explicit_product(&a, &b, &rows, &columns);
                let run = |enabled, kernel| {
                    with_tiled_weights(enabled, || {
                        let mut transcript = challenger();
                        let result = prove(
                            &a,
                            &b,
                            log_n,
                            &rows,
                            &columns,
                            value,
                            kernel,
                            &mut transcript,
                        )
                        .unwrap();
                        (result, transcript.sample_f128())
                    })
                };
                let expected = run(false, Kernel::PairAggregate);
                assert_eq!(run(true, Kernel::PairAggregate), expected);
                if log_n <= 10 {
                    assert_eq!(run(true, Kernel::Lookup), expected);
                    assert_eq!(run(true, Kernel::Reference), expected);
                }
                let (proof, claims) = &expected.0;
                assert_eq!(
                    verify(proof, log_n, &rows, &columns, value, &mut challenger()).unwrap(),
                    *claims
                );
            }
        }
    }

    #[test]
    fn reused_dirty_fold_buffers_preserve_proof_with_both_weight_paths() {
        let mut rng = Rng::new(0xB0FF_E212);
        for log_n in [7, 8, 10, 17] {
            let a = input(&mut rng, log_n);
            let b = input(&mut rng, log_n);
            let rows: Vec<_> = (0..log_n - 7).map(|_| [rng.f128(), rng.f128()]).collect();
            let columns = rng.f128_vec(KAPPA);
            let value = explicit_product(&a, &b, &rows, &columns);
            let run = |enabled, buffers| {
                with_tiled_weights(enabled, || {
                    let mut transcript = challenger();
                    let result = prove_with_fold_buffers(
                        &a,
                        &b,
                        log_n,
                        &rows,
                        &columns,
                        value,
                        Kernel::PairAggregate,
                        buffers,
                        &mut transcript,
                    )
                    .unwrap();
                    (result, transcript.sample_f128())
                })
            };
            let expected = run(false, [Vec::new(), Vec::new()]);
            let count = a.len() / 16;
            for len in [0, 1, count / 2, count * 2] {
                for enabled in [false, true] {
                    assert_eq!(
                        run(enabled, [rng.f128_vec(len), rng.f128_vec(len + 3)]),
                        expected
                    );
                }
            }
        }
    }
}
