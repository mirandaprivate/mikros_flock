//! Scalar port of the sibling Mikros weighted-Hadamard quotient + quadratic
//! sumcheck + TWO column-consistency reductions, composed with a C opening.
//! It shares Gruen's trace, field arithmetic and deterministic verifier tape.
//! This is a PIOP experiment, not the sibling's SIMD implementation or a PCS.
use crate::{
    Challenges, Trace,
    field::{Extension, Field, Prime},
    poly::{batch_inverse, eq_table, eval},
};
use rayon::prelude::*;
use std::mem::MaybeUninit;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interpolation {
    Reference,
    Optimized,
}
impl Interpolation {
    pub fn label(self) -> &'static str {
        match self {
            Self::Reference => "mikros_lagrange_reference",
            Self::Optimized => "mikros_lagrange_optimized",
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Proof<E: Field> {
    pub n: usize,
    pub k: usize,
    pub column_products: Vec<E>,
    pub quotient: Vec<E>,
    pub c_evaluation: E,
    /// Quadratic coefficients q0,q2; q1 is recovered from the current claim.
    pub rounds: Vec<[E; 2]>,
    pub terminal: [E; 2],
    pub columns_a: Vec<E>,
    pub columns_b: Vec<E>,
}
impl<E: Field> Proof<E> {
    pub fn field_elements(&self) -> usize {
        self.column_products.len()
            + self.quotient.len()
            + 1
            + 2 * self.rounds.len()
            + 2
            + self.columns_a.len()
            + self.columns_b.len()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Timings {
    pub setup: Duration,
    pub helpers: Duration,
    pub packing_weights: Duration,
    pub packing: Duration,
    /// Reference: every message scan and claim update. Optimized: the first
    /// message scan and all claim updates; later messages are fused into folds.
    pub rounds: Duration,
    /// Optimized includes the next message, scratch allocation/reuse and final
    /// scratch disposal. The fused work is counted here exactly once.
    pub round_fold: Duration,
    pub terminal: Duration,
    pub total: Duration,
}

/// Public, witness-independent interpolation data, reconstructed inside each
/// timed prover call. No unreported cache/preprocessing advantage.
pub struct Setup<B: Prime> {
    pub nodes: Vec<B>,
    pub basis: Vec<Vec<B>>,
    pub vanishing: Vec<B>,
    pub extra_weights: Vec<Vec<B>>,
    pub extra_vanishing_inverse: Vec<B>,
    pub quotient_basis: Vec<Vec<B>>,
    denominator_inverse: Vec<B>,
}
impl<B: Prime> Setup<B> {
    pub fn new(kappa: usize, mode: Interpolation) -> Result<Self, String> {
        if !kappa.is_power_of_two()
            || !(2..=16).contains(&kappa)
            || (2 * kappa) as u128 > B::MODULUS as u128
        {
            return Err("Mikros requires 2<=kappa<=16 and distinct prime-field nodes".into());
        }
        let nodes = (1..=kappa)
            .map(|i| B::from_u64(i as u64))
            .collect::<Vec<_>>();
        let extra = (kappa + 1..2 * kappa)
            .map(|i| B::from_u64(i as u64))
            .collect::<Vec<_>>();
        let basis = cardinal_basis(&nodes, mode);
        let vanishing = vanishing(&nodes);
        let denominators = nodes
            .iter()
            .enumerate()
            .map(|(j, &x)| {
                nodes
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != j)
                    .fold(B::ONE, |p, (_, &h)| p * (x - h))
            })
            .collect::<Vec<_>>();
        let denominator_inverse = batch_inverse(&denominators);
        let extra_weights = extra
            .iter()
            .map(|&x| basis.iter().map(|p| eval(p, x)).collect())
            .collect();
        let extra_vanishing_inverse = batch_inverse(
            &extra
                .iter()
                .map(|&x| eval(&vanishing, x))
                .collect::<Vec<_>>(),
        );
        let quotient_basis = cardinal_basis(&extra, mode);
        Ok(Self {
            nodes,
            basis,
            vanishing,
            extra_weights,
            extra_vanishing_inverse,
            quotient_basis,
            denominator_inverse,
        })
    }
    pub fn lagrange<E: Extension<B>>(&self, x: E, mode: Interpolation) -> Vec<E> {
        if mode == Interpolation::Reference {
            return self.basis.iter().map(|p| eval_base(p, x)).collect();
        }
        if self.nodes.len() == 2 {
            return vec![E::from_u64(2) - x, x - E::ONE];
        }
        if self.nodes.len() == 4 {
            let d: [E; 4] = std::array::from_fn(|i| x - E::lift(self.nodes[i]));
            let first = d[0] * d[1];
            let last = d[2] * d[3];
            return [d[1] * last, d[0] * last, d[3] * first, d[2] * first]
                .into_iter()
                .zip(&self.denominator_inverse)
                .map(|(v, &w)| v.scale(w))
                .collect();
        }
        // Prefix/suffix cardinal products: O(kappa) extension multiplies,
        // no extension inverse and no exceptional-node division by zero.
        let k = self.nodes.len();
        let mut prefix = vec![E::ONE; k];
        for i in 1..k {
            prefix[i] = prefix[i - 1] * (x - E::lift(self.nodes[i - 1]));
        }
        let mut suffix = E::ONE;
        for i in (0..k).rev() {
            prefix[i] = (prefix[i] * suffix).scale(self.denominator_inverse[i]);
            suffix = suffix * (x - E::lift(self.nodes[i]));
        }
        prefix
    }
}
fn eval_base<B: Prime, E: Extension<B>>(poly: &[B], x: E) -> E {
    poly.iter().rev().fold(E::ZERO, |v, &a| v * x + E::lift(a))
}
pub fn vanishing<B: Prime>(nodes: &[B]) -> Vec<B> {
    let mut p = vec![B::ONE];
    for &h in nodes {
        p.push(B::ZERO);
        for i in (1..p.len()).rev() {
            p[i] = p[i - 1] - p[i] * h;
        }
        p[0] = -p[0] * h;
    }
    p
}
pub fn cardinal_basis<B: Prime>(nodes: &[B], mode: Interpolation) -> Vec<Vec<B>> {
    if nodes.is_empty() {
        return Vec::new();
    }
    if mode == Interpolation::Reference {
        return nodes
            .iter()
            .enumerate()
            .map(|(j, &h)| {
                let others = nodes
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &x)| (i != j).then_some(x))
                    .collect::<Vec<_>>();
                let inv = others
                    .iter()
                    .fold(B::ONE, |v, &x| v * (h - x))
                    .inverse()
                    .unwrap();
                vanishing(&others).into_iter().map(|c| c * inv).collect()
            })
            .collect();
    }
    // One vanishing polynomial, synthetic divisions and ONE batch inversion:
    // O(kappa^2) base work instead of rebuilding kappa degree-kappa products.
    let p = vanishing(nodes);
    let n = nodes.len();
    let mut basis = Vec::with_capacity(n);
    let mut den = Vec::with_capacity(n);
    for &h in nodes {
        let mut q = vec![B::ZERO; n];
        q[n - 1] = p[n];
        for j in (1..n).rev() {
            q[j - 1] = p[j] + h * q[j];
        }
        den.push(eval(&q, h));
        basis.push(q);
    }
    for (q, inv) in basis.iter_mut().zip(batch_inverse(&den)) {
        for c in q {
            *c = *c * inv;
        }
    }
    basis
}

/// Evaluate the interpolant on 1,...,k at k+1,...,2k-1. Backward finite
/// differences require only base additions/subtractions, no dense dot products.
pub fn extrapolate<B: Prime>(values: &[B], out: &mut [B]) {
    let k = values.len();
    assert!((1..=16).contains(&k));
    assert_eq!(out.len(), k - 1);
    let mut scratch = [B::ZERO; 16];
    let mut differences = [B::ZERO; 16];
    scratch[..k].copy_from_slice(values);
    differences[0] = values[k - 1];
    for order in 1..k {
        for i in 0..k - order {
            scratch[i] = scratch[i + 1] - scratch[i];
        }
        differences[order] = scratch[k - order - 1];
    }
    for value in out {
        for j in (0..k - 1).rev() {
            differences[j] = differences[j] + differences[j + 1];
        }
        *value = differences[0];
    }
}
fn dot<F: Field>(a: &[F], b: &[F]) -> F {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).fold(F::ZERO, |s, (&a, &b)| s + a * b)
}
fn shape<B: Prime, E: Extension<B>>(
    trace: &Trace<B>,
    tape: &Challenges<E>,
) -> Result<(usize, usize), String> {
    let n = trace.n()?;
    let k = crate::kappa_skip_rounds::<B, E>()?;
    if n > 30 || n < k || !(2..=16).contains(&E::DEGREE) {
        return Err("Mikros trace/extension shape".into());
    }
    tape.check(n)?;
    Ok((n, k))
}

/// Helpers use A*B and independently read C. The latter must never be replaced
/// with A*B merely because benchmark fixtures are normally valid witnesses.
pub fn helpers<B: Prime, E: Extension<B>>(
    trace: &Trace<B>,
    weights: &[E],
    setup: &Setup<B>,
    mode: Interpolation,
) -> (Vec<E>, Vec<E>, Vec<E>) {
    let k = setup.nodes.len();
    assert_eq!(trace.rows.len(), k * weights.len());
    trace
        .rows
        .par_chunks(k * 4096)
        .zip(weights.par_chunks(4096))
        .map(|(rows, ws)| helpers_rows(rows, ws, setup, mode))
        .reduce(
            || (vec![E::ZERO; k], vec![E::ZERO; k - 1], vec![E::ZERO; k]),
            |mut a, b| {
                for (x, y) in a.0.iter_mut().zip(b.0) {
                    *x = *x + y;
                }
                for (x, y) in a.1.iter_mut().zip(b.1) {
                    *x = *x + y;
                }
                for (x, y) in a.2.iter_mut().zip(b.2) {
                    *x = *x + y;
                }
                a
            },
        )
}
fn helpers_rows<B: Prime, E: Extension<B>>(
    rows: &[[B; 3]],
    weights: &[E],
    setup: &Setup<B>,
    mode: Interpolation,
) -> (Vec<E>, Vec<E>, Vec<E>) {
    let k = setup.nodes.len();
    assert_eq!(rows.len(), k * weights.len());
    let mut columns = vec![E::ZERO; k];
    let mut c_columns = vec![E::ZERO; k];
    let mut extra = vec![E::ZERO; k - 1];
    let mut a = [B::ZERO; 16];
    let mut b = [B::ZERO; 16];
    let mut ax = [B::ZERO; 16];
    let mut bx = [B::ZERO; 16];
    for (rows, &w) in rows.chunks_exact(k).zip(weights) {
        for j in 0..k {
            a[j] = rows[j][0];
            b[j] = rows[j][1];
            columns[j] = columns[j] + w.scale(a[j] * b[j]);
            c_columns[j] = c_columns[j] + w.scale(rows[j][2]);
        }
        if mode == Interpolation::Optimized && k == 2 {
            // Exact constant quotient for consecutive nodes; avoid evaluating
            // and subtracting two interpolants to obtain the same difference.
            extra[0] = extra[0] + w.scale((a[1] - a[0]) * (b[1] - b[0]));
            continue;
        }
        if mode == Interpolation::Optimized {
            extrapolate(&a[..k], &mut ax[..k - 1]);
            extrapolate(&b[..k], &mut bx[..k - 1]);
        } else {
            for j in 0..k - 1 {
                ax[j] = dot(&a[..k], &setup.extra_weights[j]);
                bx[j] = dot(&b[..k], &setup.extra_weights[j]);
            }
        }
        for j in 0..k - 1 {
            extra[j] = extra[j] + w.scale(ax[j] * bx[j]);
        }
    }
    if mode == Interpolation::Optimized && k == 2 {
        return (columns, extra, c_columns);
    }
    let mut quotient = vec![E::ZERO; k - 1];
    for (j, &extra_value) in extra.iter().enumerate() {
        let interpolated = columns
            .iter()
            .zip(&setup.extra_weights[j])
            .fold(E::ZERO, |s, (&v, &w)| s + v.scale(w));
        let q = (extra_value - interpolated).scale(setup.extra_vanishing_inverse[j]);
        for (out, &c) in quotient.iter_mut().zip(&setup.quotient_basis[j]) {
            *out = *out + q.scale(c);
        }
    }
    (columns, quotient, c_columns)
}
fn pack<B: Prime, E: Extension<B>>(
    row: &[[B; 3]],
    column: usize,
    rho: &[E],
    mode: Interpolation,
) -> E {
    if mode == Interpolation::Reference {
        return row
            .iter()
            .zip(rho)
            .fold(E::ZERO, |s, (v, &w)| s + w.scale(v[column]));
    }
    // Sum of Lagrange weights is exactly one, even for node challenges.
    let base = row[0][column];
    row.iter()
        .zip(rho)
        .skip(1)
        .fold(E::lift(base), |s, (v, &w)| s + w.scale(v[column] - base))
}

const COLUMN_BLOCK_LOG: usize = 8;

/// Factor the two column-consistency weights across 256-row blocks. For row
/// index (h,l), u(h,l)=u_hi(h)*u_lo(l) and z(h,l)=z_hi(h)*z_lo(l).
/// Accumulate with the shared low weights, then multiply each column sum by
/// its high weight. This avoids an extension product and a folding-table entry
/// for every row; the messages and the point at which challenges are used stay
/// unchanged. In particular, u*z is a product of equality weights, not eq(u*z).
fn terminal_columns_blocked<B: Prime, E: Extension<B>>(
    trace: &Trace<B>,
    tape: &Challenges<E>,
    k: usize,
) -> (Vec<E>, Vec<E>) {
    let kappa = E::DEGREE;
    let block_rows = 1 << COLUMN_BLOCK_LOG;
    let (r_lo, r_hi) = tape.rounds[k..].split_at(COLUMN_BLOCK_LOG);
    let (z_lo, z_hi) = tape.z[k..].split_at(COLUMN_BLOCK_LOG);
    let low_a = eq_table(r_lo);
    let mut low_b = eq_table(z_lo);
    for (v, &u) in low_b.iter_mut().zip(&low_a) {
        *v = *v * u;
    }
    let high_a = eq_table(r_hi);
    let mut high_b = eq_table(z_hi);
    for (v, &u) in high_b.iter_mut().zip(&high_a) {
        *v = *v * u;
    }
    let (columns_a, columns_b) = trace
        .rows
        .par_chunks_exact(kappa * block_rows)
        .zip(high_a.par_iter())
        .zip(high_b.par_iter())
        .with_min_len(16)
        .fold(
            || ([E::ZERO; 16], [E::ZERO; 16]),
            |mut total, ((block, &u_hi), &v_hi)| {
                let mut a = [E::ZERO; 16];
                let mut b = [E::ZERO; 16];
                for ((row, &u), &v) in block.chunks_exact(kappa).zip(&low_a).zip(&low_b) {
                    for j in 0..kappa {
                        a[j] = a[j] + u.scale(row[j][0]);
                        b[j] = b[j] + v.scale(row[j][1]);
                    }
                }
                for j in 0..kappa {
                    total.0[j] = total.0[j] + u_hi * a[j];
                    total.1[j] = total.1[j] + v_hi * b[j];
                }
                total
            },
        )
        .reduce(
            || ([E::ZERO; 16], [E::ZERO; 16]),
            |mut a, b| {
                for j in 0..kappa {
                    a.0[j] = a.0[j] + b.0[j];
                    a.1[j] = a.1[j] + b.1[j];
                }
                a
            },
        );
    (columns_a[..kappa].to_vec(), columns_b[..kappa].to_vec())
}

const FOLD_OUTPUT_CHUNK: usize = 4096;
const PARALLEL_INPUT_THRESHOLD: usize = 32768;

fn use_parallel(len: usize) -> bool {
    len >= PARALLEL_INPUT_THRESHOLD && rayon::current_num_threads() > 1
}

fn initial_message<E: Field>(a: &[E], b: &[E]) -> [E; 2] {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len() % 2, 0);
    let chunk = |(aa, bb): (&[E], &[E])| {
        let mut out = [E::ZERO; 2];
        for (ap, bp) in aa.chunks_exact(2).zip(bb.chunks_exact(2)) {
            out[0] = out[0] + ap[0] * bp[0];
            out[1] = out[1] + (ap[1] - ap[0]) * (bp[1] - bp[0]);
        }
        out
    };
    if use_parallel(a.len()) {
        a.par_chunks(2 * FOLD_OUTPUT_CHUNK)
            .zip(b.par_chunks(2 * FOLD_OUTPUT_CHUNK))
            .map(chunk)
            .reduce(|| [E::ZERO; 2], |a, b| [a[0] + b[0], a[1] + b[1]])
    } else {
        chunk((a, b))
    }
}

/// As in the binary IP kernel, expose spare capacity without zero-filling.
/// Field is Copy: a panic leaves an empty, valid buffer with nothing to drop.
fn prepare_output<E: Field>(buffer: &mut Vec<E>, len: usize) -> &mut [MaybeUninit<E>] {
    buffer.clear();
    buffer.reserve(len);
    &mut buffer.spare_capacity_mut()[..len]
}

fn fold_message_chunk<E: Field>(
    a: &[E],
    b: &[E],
    a_out: &mut [MaybeUninit<E>],
    b_out: &mut [MaybeUninit<E>],
    r: E,
) -> [E; 2] {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), 2 * a_out.len());
    assert_eq!(a_out.len(), b_out.len());
    assert_eq!(a.len() % 4, 0);
    let mut sums = [E::ZERO; 2];
    for (((a, b), a_out), b_out) in a
        .chunks_exact(4)
        .zip(b.chunks_exact(4))
        .zip(a_out.chunks_exact_mut(2))
        .zip(b_out.chunks_exact_mut(2))
    {
        let a0 = a[0] + r * (a[1] - a[0]);
        let a1 = a[2] + r * (a[3] - a[2]);
        let b0 = b[0] + r * (b[1] - b[0]);
        let b1 = b[2] + r * (b[3] - b[2]);
        a_out[0].write(a0);
        a_out[1].write(a1);
        b_out[0].write(b0);
        b_out[1].write(b1);
        sums[0] = sums[0] + a0 * b0;
        sums[1] = sums[1] + (a1 - a0) * (b1 - b0);
    }
    sums
}

/// Consume the current round challenge, fold, and immediately accumulate the
/// following message. No later challenge is read while producing that message.
fn fold_and_next_message<E: Field>(
    a: &[E],
    b: &[E],
    a_out: &mut Vec<E>,
    b_out: &mut Vec<E>,
    r: E,
) -> Option<[E; 2]> {
    assert_eq!(a.len(), b.len());
    assert!(a.len().is_power_of_two() && a.len() >= 2);
    let half = a.len() / 2;
    let a_storage = prepare_output(a_out, half);
    let b_storage = prepare_output(b_out, half);
    let next = if half == 1 {
        a_storage[0].write(a[0] + r * (a[1] - a[0]));
        b_storage[0].write(b[0] + r * (b[1] - b[0]));
        None
    } else if use_parallel(a.len()) {
        Some(
            a_storage
                .par_chunks_mut(FOLD_OUTPUT_CHUNK)
                .zip(b_storage.par_chunks_mut(FOLD_OUTPUT_CHUNK))
                .enumerate()
                .map(|(chunk, (a_out, b_out))| {
                    let start = 2 * chunk * FOLD_OUTPUT_CHUNK;
                    let end = start + 2 * a_out.len();
                    fold_message_chunk(&a[start..end], &b[start..end], a_out, b_out, r)
                })
                .reduce(|| [E::ZERO; 2], |a, b| [a[0] + b[0], a[1] + b[1]]),
        )
    } else {
        Some(fold_message_chunk(a, b, a_storage, b_storage, r))
    };
    // SAFETY: the one-output case writes both slots explicitly. Otherwise the
    // asserted power-of-two shape and disjoint even-sized output chunks cover
    // every slot. All workers joined successfully before committing lengths;
    // any panic skips this commit and leaves valid empty vectors.
    unsafe {
        a_out.set_len(half);
        b_out.set_len(half);
    }
    next
}

pub fn prove<B: Prime + Extension<B>, E: Extension<B>>(
    trace: &Trace<B>,
    tape: &Challenges<E>,
    mode: Interpolation,
) -> Result<(Proof<E>, Timings), String> {
    let start = Instant::now();
    let (n, k) = shape(trace, tape)?;
    let kappa = E::DEGREE;
    let mut timings = Timings::default();
    let t = Instant::now();
    let setup = Setup::<B>::new(kappa, mode)?;
    let row_weights = eq_table(&tape.z[k..]);
    let column_weights = eq_table(&tape.z[..k]);
    timings.setup = t.elapsed();
    let t = Instant::now();
    let (column_products, quotient, c_columns) = helpers(trace, &row_weights, &setup, mode);
    let c_evaluation = dot(&c_columns, &column_weights);
    if dot(&column_products, &column_weights) != c_evaluation {
        return Err("Mikros weighted A*B differs from C".into());
    }
    timings.helpers = t.elapsed();
    let t = Instant::now();
    let rho = setup.lagrange(tape.skip, mode);
    timings.packing_weights = t.elapsed();
    let t = Instant::now();
    let (mut a, mut b): (Vec<E>, Vec<E>) = trace
        .rows
        .par_chunks_exact(kappa)
        .zip(row_weights.par_iter())
        .with_min_len(4096)
        .map(|(row, &weight)| (pack(row, 0, &rho, mode), pack(row, 1, &rho, mode) * weight))
        .unzip();
    timings.packing = t.elapsed();
    let mut claim = dot(&column_products, &rho)
        + eval_base(&setup.vanishing, tape.skip) * eval(&quotient, tape.skip);
    let mut rounds = Vec::with_capacity(n - k);
    let terminal = if mode == Interpolation::Optimized {
        let t = Instant::now();
        let mut message = (n > k).then(|| initial_message(&a, &b));
        timings.rounds += t.elapsed();
        let mut a_next = Vec::new();
        let mut b_next = Vec::new();
        for &r in &tape.rounds[k..] {
            let t = Instant::now();
            let [q0, q2] = message.expect("each nonterminal fold produces the next message");
            let q1 = claim - q0 - q0 - q2;
            claim = q0 + r * (q1 + r * q2);
            rounds.push([q0, q2]);
            timings.rounds += t.elapsed();
            let t = Instant::now();
            message = fold_and_next_message(&a, &b, &mut a_next, &mut b_next, r);
            std::mem::swap(&mut a, &mut a_next);
            std::mem::swap(&mut b, &mut b_next);
            timings.round_fold += t.elapsed();
        }
        let terminal = [a[0], b[0]];
        let t = Instant::now();
        // Reused buffers retain their earlier capacity. Dispose of them inside
        // the measured fold scope, before terminal processing and total timing.
        drop((a, b, a_next, b_next));
        timings.round_fold += t.elapsed();
        terminal
    } else {
        for &r in &tape.rounds[k..] {
            let t = Instant::now();
            let [q0, q2] = a
                .par_chunks(8192)
                .zip(b.par_chunks(8192))
                .map(|(aa, bb)| {
                    let mut out = [E::ZERO; 2];
                    for (ap, bp) in aa.chunks_exact(2).zip(bb.chunks_exact(2)) {
                        out[0] = out[0] + ap[0] * bp[0];
                        out[1] = out[1] + (ap[1] - ap[0]) * (bp[1] - bp[0]);
                    }
                    out
                })
                .reduce(|| [E::ZERO; 2], |a, b| [a[0] + b[0], a[1] + b[1]]);
            let q1 = claim - q0 - q0 - q2;
            claim = q0 + r * (q1 + r * q2);
            rounds.push([q0, q2]);
            timings.rounds += t.elapsed();
            let t = Instant::now();
            let fold = |v: &[E]| {
                v.par_chunks_exact(2)
                    .with_min_len(4096)
                    .map(|p| p[0] + r * (p[1] - p[0]))
                    .collect::<Vec<E>>()
            };
            (a, b) = rayon::join(|| fold(&a), || fold(&b));
            timings.round_fold += t.elapsed();
        }
        [a[0], b[0]]
    };
    if claim != terminal[0] * terminal[1] {
        return Err("Mikros internal quadratic claim mismatch".into());
    }
    let t = Instant::now();
    let (columns_a, columns_b) = if mode == Interpolation::Optimized && n - k > COLUMN_BLOCK_LOG {
        terminal_columns_blocked(trace, tape, k)
    } else {
        let folding = eq_table(&tape.rounds[k..]);
        trace
            .rows
            .par_chunks(kappa * 4096)
            .zip(folding.par_chunks(4096))
            .zip(row_weights.par_chunks(4096))
            .map(|((block, us), zs)| {
                let mut columns_a = vec![E::ZERO; kappa];
                let mut columns_b = vec![E::ZERO; kappa];
                for ((row, &u), &original) in block.chunks_exact(kappa).zip(us).zip(zs) {
                    let v = u * original;
                    for j in 0..kappa {
                        columns_a[j] = columns_a[j] + u.scale(row[j][0]);
                        columns_b[j] = columns_b[j] + v.scale(row[j][1]);
                    }
                }
                (columns_a, columns_b)
            })
            .reduce(
                || (vec![E::ZERO; kappa], vec![E::ZERO; kappa]),
                |mut a, b| {
                    for (x, y) in a.0.iter_mut().zip(b.0) {
                        *x = *x + y;
                    }
                    for (x, y) in a.1.iter_mut().zip(b.1) {
                        *x = *x + y;
                    }
                    a
                },
            )
    };
    timings.terminal = t.elapsed();
    let proof = Proof {
        n,
        k,
        column_products,
        quotient,
        c_evaluation,
        rounds,
        terminal,
        columns_a,
        columns_b,
    };
    timings.total = start.elapsed();
    Ok((proof, timings))
}

/// Source oracle evaluation is deliberately separate from timed proving and
/// reduction verification. B's factors are PRODUCTS of equality factors,
/// not equality factors evaluated at a product of coordinates.
pub fn oracle_evaluations<B: Prime + Extension<B>, E: Extension<B>>(
    trace: &Trace<B>,
    tape: &Challenges<E>,
) -> Result<[E; 3], String> {
    let (_, k) = shape(trace, tape)?;
    let columns = eq_table(&tape.rounds[..k]);
    let low_z = eq_table(&tape.z[..k]);
    let high = eq_table(&tape.rounds[k..]);
    let z = eq_table(&tape.z[k..]);
    let out = trace
        .rows
        .par_chunks(4096)
        .enumerate()
        .map(|(chunk_index, chunk)| {
            let mut out = [E::ZERO; 3];
            for (offset, row) in chunk.iter().enumerate() {
                let i = chunk_index * 4096 + offset;
                let col = i % E::DEGREE;
                let j = i / E::DEGREE;
                let a = columns[col] * high[j];
                let b = a * z[j];
                let c = low_z[col] * z[j];
                out[0] = out[0] + a.scale(row[0]);
                out[1] = out[1] + b.scale(row[1]);
                out[2] = out[2] + c.scale(row[2]);
            }
            out
        })
        .reduce(|| [E::ZERO; 3], |a, b| std::array::from_fn(|i| a[i] + b[i]));
    Ok(out)
}
pub fn verify<B: Prime, E: Extension<B>>(
    proof: &Proof<E>,
    tape: &Challenges<E>,
    n: usize,
    trusted: [E; 3],
) -> Result<(), String> {
    tape.check(n)?;
    let kappa = E::DEGREE;
    if !(2..=16).contains(&kappa) || !kappa.is_power_of_two() {
        return Err("Mikros extension degree".into());
    }
    let k = kappa.ilog2() as usize;
    if n > 30
        || n < k
        || proof.n != n
        || proof.k != k
        || proof.column_products.len() != kappa
        || proof.quotient.len() != kappa - 1
        || proof.rounds.len() != n - k
        || proof.columns_a.len() != kappa
        || proof.columns_b.len() != kappa
    {
        return Err("Mikros malformed proof".into());
    }
    let setup = Setup::<B>::new(kappa, Interpolation::Optimized)?;
    if proof.c_evaluation != trusted[2]
        || dot(&proof.column_products, &eq_table(&tape.z[..k])) != proof.c_evaluation
    {
        return Err("Mikros public C/column claim mismatch".into());
    }
    let rho = setup.lagrange(tape.skip, Interpolation::Optimized);
    let mut claim = dot(&proof.column_products, &rho)
        + eval_base(&setup.vanishing, tape.skip) * eval(&proof.quotient, tape.skip);
    for (&[q0, q2], &r) in proof.rounds.iter().zip(&tape.rounds[k..]) {
        let q1 = claim - q0 - q0 - q2;
        claim = q0 + r * (q1 + r * q2);
    }
    if claim != proof.terminal[0] * proof.terminal[1]
        || dot(&proof.columns_a, &rho) != proof.terminal[0]
        || dot(&proof.columns_b, &rho) != proof.terminal[1]
    {
        return Err("Mikros quadratic/column consistency failed".into());
    }
    // Fresh post-column-message challenges in an actual interactive execution.
    let aggregation = eq_table(&tape.rounds[..k]);
    if dot(&proof.columns_a, &aggregation) != trusted[0]
        || dot(&proof.columns_b, &aggregation) != trusted[1]
    {
        return Err("Mikros source oracle mismatch".into());
    }
    Ok(())
}
