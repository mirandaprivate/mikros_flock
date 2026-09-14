//! Degree-two Hadamard zerocheck experiments, specialized from han0110's pinned
//! univariate-skip implementation. See README.md for protocol and timing scope.
pub mod field;
pub mod mikros;
pub mod poly;

use field::{Extension, Field, Prime, Rng};
use poly::{Domain, SkipDomain, eq, eq_table, eval, skip_vanishing, validate_skip_domain};
use rayon::prelude::*;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Trace<B: Prime> {
    pub rows: Vec<[B; 3]>,
}
impl<B: Prime> Trace<B> {
    pub fn random(n: usize, seed: u64) -> Self {
        assert!(n <= 30, "experiment input cap is 2^30 rows");
        let mut rng = Rng(seed);
        let rows = (0..1usize << n)
            .map(|_| {
                let a = B::random(&mut rng);
                let b = B::random(&mut rng);
                [a, b, a * b]
            })
            .collect();
        Self { rows }
    }
    pub fn n(&self) -> Result<usize, String> {
        if !self.rows.len().is_power_of_two() {
            return Err("trace length must be a positive power of two".into());
        }
        Ok(self.rows.len().ilog2() as usize)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Method {
    Regular,
    Gruen,
    Skip(usize),
}
impl Method {
    pub fn skip(self) -> usize {
        if let Self::Skip(k) = self { k } else { 0 }
    }
    pub fn label(self) -> String {
        match self {
            Self::Regular => "regular_3eval".into(),
            Self::Gruen => "gruen_2eval".into(),
            Self::Skip(k) => format!("gruen_skip_{k}"),
        }
    }
}

/// Verifier-supplied randomness, materialized up front only for reproducible tests.
/// A real interactive integration must reveal each r after its round message.
#[derive(Clone, Debug)]
pub struct Challenges<E: Field> {
    pub z: Vec<E>,
    pub rounds: Vec<E>,
    pub skip: E,
}
impl<E: Field> Challenges<E> {
    pub fn deterministic(n: usize, seed: u64) -> Self {
        let mut rng = Rng(seed);
        Self {
            z: (0..n).map(|_| E::random(&mut rng)).collect(),
            rounds: (0..n).map(|_| E::random(&mut rng)).collect(),
            skip: E::random(&mut rng),
        }
    }
    fn check(&self, n: usize) -> Result<(), String> {
        if self.z.len() != n || self.rounds.len() != n {
            return Err("challenge dimension mismatch".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Proof<E: Field> {
    pub n: usize,
    pub skip_rounds: usize,
    /// Coefficients of Q(X); exact bound deg Q <= B-2 for a*b-c.
    pub quotient: Vec<E>,
    /// Coefficients `[q0,q2,q3]`; q1 is recovered from q(0)+q(1)=claim.
    pub rounds: Vec<[E; 3]>,
    pub terminal: [E; 3],
}
impl<E: Field> Proof<E> {
    pub fn field_elements(&self) -> usize {
        self.quotient.len() + 3 * self.rounds.len() + 3
    }
}

#[derive(Clone, Debug, Default)]
pub struct Timings {
    pub setup: Duration,
    pub quotient: Duration,
    pub initial_fold: Duration,
    pub rounds: Duration,
    pub round_fold: Duration,
    pub total: Duration,
}

fn validate<B: Prime>(n: usize, k: usize) -> Result<(), String> {
    if n > 30 {
        return Err("experiment input cap is 2^30 rows".into());
    }
    if k > n {
        return Err("invalid skip: require k <= n".into());
    }
    validate_skip_domain::<B>(k)
}

/// Kappa is the challenge extension degree. Combining kappa rows skips
/// log2(kappa) Boolean rounds; no two-adicity clamping or timing-based selection.
pub fn kappa_skip_rounds<B: Prime, E: Extension<B>>() -> Result<usize, String> {
    if !E::DEGREE.is_power_of_two() {
        return Err("kappa must be a nonzero power of two".into());
    }
    let k = E::DEGREE.ilog2() as usize;
    validate_skip_domain::<B>(k)?;
    Ok(k)
}

/// Aggregate the quotient on a disjoint base-field coset, then interpolate.
/// The three trace polynomials have degree < B, so Q has degree <= B-2.
pub fn skip_quotient<B: Prime + Extension<B>, E: Extension<B>>(
    trace: &Trace<B>,
    domain: &Domain<B>,
    weights: &[E],
) -> Vec<E> {
    let b = domain.nodes.len();
    assert_eq!(trace.rows.len(), b * weights.len());
    let mut acc = trace
        .rows
        .par_chunks(b * 4096)
        .zip(weights.par_chunks(4096))
        .map(|(chunk, ws)| {
            let mut acc = vec![E::ZERO; b];
            let mut cols = [vec![B::ZERO; b], vec![B::ZERO; b], vec![B::ZERO; b]];
            for (rows, &weight) in chunk.chunks_exact(b).zip(ws) {
                for (col, data) in cols.iter_mut().enumerate() {
                    for (v, row) in data.iter_mut().zip(rows) {
                        *v = row[col];
                    }
                    domain.coset_lde(data);
                }
                for i in 0..b {
                    let value = (cols[0][i] * cols[1][i] - cols[2][i]) * domain.vanishing_inverse;
                    acc[i] = acc[i] + weight.scale(value);
                }
            }
            acc
        })
        .reduce(
            || vec![E::ZERO; b],
            |mut a, b| {
                for (x, y) in a.iter_mut().zip(b) {
                    *x = *x + y;
                }
                a
            },
        );
    domain.coset_interpolate(&mut acc);
    // Sending only B-1 coefficients enforces the exact degree bound.
    acc.truncate(b - 1);
    acc
}

/// Quotient reduction on the selected base-field domain, including the
/// general-node fallback required by kappa-sized F251 and Mersenne31 blocks.
pub fn skip_quotient_on_domain<B: Prime + Extension<B>, E: Extension<B>>(
    trace: &Trace<B>,
    domain: &SkipDomain<B>,
    weights: &[E],
) -> Vec<E> {
    if let SkipDomain::Radix2(d) = domain {
        return skip_quotient(trace, d, weights);
    }
    let b = domain.nodes().len();
    assert_eq!(trace.rows.len(), b * weights.len());
    let mut acc = trace
        .rows
        .par_chunks(b * 4096)
        .zip(weights.par_chunks(4096))
        .map(|(chunk, ws)| {
            let mut acc = vec![E::ZERO; b];
            let mut cols = [vec![B::ZERO; b], vec![B::ZERO; b], vec![B::ZERO; b]];
            let mut scratch = vec![B::ZERO; b];
            for (rows, &weight) in chunk.chunks_exact(b).zip(ws) {
                for (col, data) in cols.iter_mut().enumerate() {
                    for (v, row) in data.iter_mut().zip(rows) {
                        *v = row[col];
                    }
                    domain.evaluate(data, &mut scratch);
                }
                for i in 0..b {
                    let value =
                        (cols[0][i] * cols[1][i] - cols[2][i]) * domain.inverse_vanishing(i);
                    acc[i] = acc[i] + weight.scale(value);
                }
            }
            acc
        })
        .reduce(
            || vec![E::ZERO; b],
            |mut a, b| {
                for (x, y) in a.iter_mut().zip(b) {
                    *x = *x + y;
                }
                a
            },
        );
    domain.interpolate(&mut acc);
    acc.truncate(b - 1);
    acc
}

fn initial_fold<B: Prime, E: Extension<B>>(trace: &Trace<B>, weights: &[E]) -> Vec<[E; 3]> {
    trace
        .rows
        .par_chunks_exact(weights.len())
        .with_min_len(4096)
        .map(|rows| {
            let mut out = [E::ZERO; 3];
            for (row, &w) in rows.iter().zip(weights) {
                for col in 0..3 {
                    out[col] = out[col] + w.scale(row[col]);
                }
            }
            out
        })
        .collect()
}

fn fold_extension<E: Field>(rows: &mut Vec<[E; 3]>, r: E) {
    // Distinct output avoids races between parallel readers and writers.
    // Allocation and old-buffer destruction stay inside prover timing.
    *rows = rows
        .par_chunks_exact(2)
        .with_min_len(4096)
        .map(|pair| std::array::from_fn(|j| pair[0][j] + (pair[1][j] - pair[0][j]) * r))
        .collect();
}

/// Evaluate the constraint at 0,1,2 (ordinary) or 1,2 (Gruen recovery).
fn extra_evals<V: Field, E: Field>(
    rows: &[[V; 3]],
    weights: &[E],
    recover: bool,
    scale: impl Fn(E, V) -> E + Sync,
) -> [E; 3] {
    rows.par_chunks(8192)
        .zip(weights.par_chunks(4096))
        .map(|(chunk, ws)| {
            let mut out = [E::ZERO; 3];
            for (pair, &w) in chunk.chunks_exact(2).zip(ws) {
                let lo = pair[0];
                let hi = pair[1];
                if !recover {
                    out[0] = out[0] + scale(w, lo[0] * lo[1] - lo[2]);
                }
                out[1] = out[1] + scale(w, hi[0] * hi[1] - hi[2]);
                let at2: [V; 3] = std::array::from_fn(|j| hi[j] + hi[j] - lo[j]);
                out[2] = out[2] + scale(w, at2[0] * at2[1] - at2[2]);
            }
            out
        })
        .reduce(|| [E::ZERO; 3], |a, b| std::array::from_fn(|i| a[i] + b[i]))
}

fn round_poly<B: Prime, E: Extension<B>>(mut h: [E; 3], claim: E, z: E, recover: bool) -> [E; 3] {
    if recover {
        h[0] = (claim - z * h[1]) * (E::ONE - z).inverse().unwrap();
    }
    let c2 = (h[2] - h[1] - h[1] + h[0]).scale(B::from_u64(2).inverse().unwrap());
    [h[0], h[1] - h[0] - c2, c2]
}

fn compress<E: Field>(h: [E; 3], z: E, prefix: E) -> [E; 3] {
    let e0 = (E::ONE - z) * prefix;
    let e1 = (z + z - E::ONE) * prefix;
    [h[0] * e0, h[1] * e1 + h[2] * e0, h[2] * e1]
}

/// End-to-end reduction prover. Timed scope includes equality tables, transforms,
/// trace folding, allocations and all round messages; excludes witness creation.
pub fn prove<B: Prime + Extension<B>, E: Extension<B>>(
    trace: &Trace<B>,
    tape: &Challenges<E>,
    method: Method,
) -> Result<(Proof<E>, Timings), String> {
    let start = Instant::now();
    let n = trace.n()?;
    let k = method.skip();
    validate::<B>(n, k)?;
    tape.check(n)?;
    let mut timings = Timings::default();
    let setup = Instant::now();
    // Each suffix is smaller by two; total table generation is O(2^(n-k)).
    let weights: Vec<_> = (k..n).map(|i| eq_table(&tape.z[i + 1..])).collect();
    let mut claim = E::ZERO;
    let mut quotient = Vec::new();
    let mut rows = Vec::new();
    timings.setup += setup.elapsed();
    if k > 0 {
        let setup = Instant::now();
        let domain = SkipDomain::<B>::new(k)?;
        let skip_weights = eq_table(&tape.z[k..]);
        timings.setup += setup.elapsed();
        let t = Instant::now();
        quotient = skip_quotient_on_domain(trace, &domain, &skip_weights);
        claim = eval(&quotient, tape.skip) * skip_vanishing::<B, E>(k, tape.skip);
        timings.quotient += t.elapsed();
        let t = Instant::now();
        rows = initial_fold(trace, &domain.lagrange(tape.skip));
        timings.initial_fold += t.elapsed();
    }
    let mut messages = Vec::with_capacity(n - k);
    let mut prefix = E::ONE;
    for i in k..n {
        let t = Instant::now();
        let z = tape.z[i];
        // z=1 is a supported challenge; explicitly compute h(0) in that case.
        let recover = method != Method::Regular && z != E::ONE;
        let values = if i == 0 && k == 0 {
            extra_evals(&trace.rows, &weights[i - k], recover, E::scale)
        } else {
            extra_evals(&rows, &weights[i - k], recover, |w, v| w * v)
        };
        let h = round_poly::<B, E>(values, claim, z, recover);
        messages.push(compress(h, z, prefix));
        claim = eval(&h, tape.rounds[i]);
        prefix = prefix * eq(z, tape.rounds[i]);
        timings.rounds += t.elapsed();
        let t = Instant::now();
        if i == 0 && k == 0 {
            rows = initial_fold(trace, &[E::ONE - tape.rounds[i], tape.rounds[i]]);
        } else {
            fold_extension(&mut rows, tape.rounds[i]);
        }
        timings.round_fold += t.elapsed();
    }
    let terminal = if n == 0 {
        trace.rows[0].map(E::lift)
    } else {
        rows[0]
    };
    let proof = Proof {
        n,
        skip_rounds: k,
        quotient,
        rounds: messages,
        terminal,
    };
    timings.total = start.elapsed();
    Ok((proof, timings))
}

/// Dense trusted-oracle evaluation, independent of prover folding. This stands in
/// for the mixed Lagrange/multilinear opening that a PCS integration must supply.
pub fn oracle_evaluations<B: Prime + Extension<B>, E: Extension<B>>(
    trace: &Trace<B>,
    tape: &Challenges<E>,
    k: usize,
) -> Result<[E; 3], String> {
    let n = trace.n()?;
    validate::<B>(n, k)?;
    tape.check(n)?;
    let low = if k == 0 {
        vec![E::ONE]
    } else {
        SkipDomain::<B>::new(k)?.lagrange(tape.skip)
    };
    let high = eq_table(&tape.rounds[k..]);
    let out = trace
        .rows
        .par_chunks(low.len() * 4096)
        .zip(high.par_chunks(4096))
        .map(|(block, ws)| {
            let mut out = [E::ZERO; 3];
            for (chunk, &wh) in block.chunks_exact(low.len()).zip(ws) {
                for (row, &wl) in chunk.iter().zip(&low) {
                    let w = wh * wl;
                    for j in 0..3 {
                        out[j] = out[j] + w.scale(row[j]);
                    }
                }
            }
            out
        })
        .reduce(|| [E::ZERO; 3], |a, b| std::array::from_fn(|i| a[i] + b[i]));
    Ok(out)
}

/// Check all polynomial messages and a terminal evaluation authenticated by an
/// external oracle. n and k are expected statement parameters, not prover choices.
pub fn verify<B: Prime, E: Extension<B>>(
    proof: &Proof<E>,
    tape: &Challenges<E>,
    n: usize,
    k: usize,
    trusted: [E; 3],
) -> Result<(), String> {
    tape.check(n)?;
    validate::<B>(n, k)?;
    if k > n || k > 16 || n > 30 || proof.n != n || proof.skip_rounds != k {
        return Err("proof parameter mismatch".into());
    }
    if proof.rounds.len() != n - k || proof.quotient.len() != if k == 0 { 0 } else { (1 << k) - 1 }
    {
        return Err("invalid proof polynomial lengths".into());
    }
    if proof.terminal != trusted {
        return Err("terminal oracle mismatch".into());
    }
    let mut claim = if k == 0 {
        E::ZERO
    } else {
        eval(&proof.quotient, tape.skip) * skip_vanishing::<B, E>(k, tape.skip)
    };
    let mut prefix = E::ONE;
    for (i, &[q0, q2, q3]) in (k..n).zip(&proof.rounds) {
        let q1 = claim - q0 - q0 - q2 - q3;
        claim = eval(&[q0, q1, q2, q3], tape.rounds[i]);
        prefix = prefix * eq(tape.z[i], tape.rounds[i]);
    }
    if claim != prefix * (trusted[0] * trusted[1] - trusted[2]) {
        return Err("terminal zerocheck identity failed".into());
    }
    Ok(())
}
