//! Two Mikros coefficient trees, one equality-factored GKR chain.
//! This does not use Flock's value/position fingerprint or change Mikros leaves.
use crate::{
    Error,
    claims::Claims,
    permutation::{PreparedRows, Statement, Stats, source_claims},
    product_tree::{Options, ProductTree, Tree},
    profile::{Engine, Timings, elapsed},
    sumcheck::LeafClaim,
};
use flock_core::zerocheck::univariate_skip::SplitEqGhash;
use flock_field::{F128, F256Unreduced};
use flock_transcript::challenger::Challenger;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{mem::swap, time::Instant};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InnerProof {
    /// q(1), q(infinity), except parent=1 uses q(0), q(infinity).
    pub rounds: Vec<[F128; 2]>,
    pub terminal: [F128; 4],
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proof {
    pub roots: [F128; 2],
    pub layers: Vec<InnerProof>,
}

#[derive(Clone, Copy)]
pub(crate) enum Input<'a> {
    Interleaved([&'a [F128]; 2]),
    Separate([&'a [F128]; 4]),
}
impl Input<'_> {
    fn len(self) -> usize {
        match self {
            Self::Interleaved(a) => a[0].len() / 2,
            Self::Separate(a) => a[0].len(),
        }
    }
    #[inline]
    fn at(self, i: usize) -> [F128; 4] {
        match self {
            Self::Interleaved([a, b]) => [a[2 * i], a[2 * i + 1], b[2 * i], b[2 * i + 1]],
            Self::Separate(a) => a.map(|x| x[i]),
        }
    }
}

pub(crate) struct Scratch {
    cur: [Vec<F128>; 4],
    next: [Vec<F128>; 4],
}
impl Scratch {
    pub(crate) fn new(capacity: usize) -> Self {
        Self {
            cur: std::array::from_fn(|_| flock_core::scratch::take_f128(capacity)),
            next: std::array::from_fn(|_| flock_core::scratch::take_f128(capacity)),
        }
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        for v in self.cur.iter_mut().chain(self.next.iter_mut()) {
            flock_core::scratch::give_f128(std::mem::take(v));
        }
    }
}
fn split(point: &[F128]) -> SplitEqGhash {
    SplitEqGhash::with_n_hi(point, point.len() / 2)
}
fn add(a: [F128; 2], b: [F128; 2]) -> [F128; 2] {
    [a[0] + b[0], a[1] + b[1]]
}
fn block_message(
    input: Input<'_>,
    start: usize,
    weights: &[F128],
    lambda: F128,
    corner: usize,
) -> [F128; 2] {
    let mut sums = [F256Unreduced::ZERO; 4];
    for (j, &weight) in weights.iter().enumerate() {
        let a = input.at(2 * (start + j));
        let b = input.at(2 * (start + j) + 1);
        let e = if corner == 0 { a } else { b };
        let d: [F128; 4] = std::array::from_fn(|i| a[i] + b[i]);
        let v = [e[0] * e[1], e[2] * e[3], d[0] * d[1], d[2] * d[3]];
        for k in 0..4 {
            sums[k] ^= v[k].mul_unreduced(weight);
        }
    }
    let [a, b, c, d] = sums.map(F256Unreduced::reduce);
    [a + lambda * b, c + lambda * d]
}
fn message(input: Input<'_>, weights: &SplitEqGhash, lambda: F128, corner: usize) -> [F128; 2] {
    let calculate = |hi: usize| {
        block_message(input, hi * weights.lo.len(), &weights.lo, lambda, corner)
            .map(|x| x * weights.hi[hi])
    };
    if input.len() < 1 << 15 {
        (0..weights.hi.len())
            .map(calculate)
            .fold([F128::ZERO; 2], add)
    } else {
        (0..weights.hi.len())
            .into_par_iter()
            .map(calculate)
            .reduce(|| [F128::ZERO; 2], add)
    }
}
fn fold_block(input: Input<'_>, start: usize, out: [&mut [F128]; 4], r: F128) {
    match input {
        Input::Separate(src) => {
            for (s, d) in src.into_iter().zip(out) {
                flock_field::f128_slice::fold_pairs(s, start, d, r);
            }
        }
        Input::Interleaved(src) => {
            let [a, b, c, d] = out;
            crate::kernels::fold_interleaved(src[0], start, a, b, r);
            crate::kernels::fold_interleaved(src[1], start, c, d, r);
        }
    }
}
fn fold_message(
    input: Input<'_>,
    out: [&mut [F128]; 4],
    r: F128,
    next: Option<&SplitEqGhash>,
    lambda: F128,
    corner: usize,
) -> Option<[F128; 2]> {
    if let Some(w) = next {
        let chunk = 2 * w.lo.len();
        let [a, b, c, d] = out;
        let calculate = |(hi, (((a, b), c), d)): (
            usize,
            (((&mut [F128], &mut [F128]), &mut [F128]), &mut [F128]),
        )| {
            fold_block(input, hi * chunk, [a, b, c, d], r);
            block_message(Input::Separate([a, b, c, d]), 0, &w.lo, lambda, corner)
                .map(|x| x * w.hi[hi])
        };
        let sums = if input.len() < 1 << 15 {
            a.chunks_mut(chunk)
                .zip(b.chunks_mut(chunk))
                .zip(c.chunks_mut(chunk))
                .zip(d.chunks_mut(chunk))
                .enumerate()
                .map(calculate)
                .fold([F128::ZERO; 2], add)
        } else {
            a.par_chunks_mut(chunk)
                .zip(b.par_chunks_mut(chunk))
                .zip(c.par_chunks_mut(chunk))
                .zip(d.par_chunks_mut(chunk))
                .enumerate()
                .map(calculate)
                .reduce(|| [F128::ZERO; 2], add)
        };
        Some(sums)
    } else {
        fold_block(input, 0, out, r);
        None
    }
}

/// Normalize the remaining claim by removing the current eq factor.
/// Parent=1 has a deterministic alternate convention, so no division by zero
/// or challenge resampling is required, even for adversarial zero endpoints.
fn advance(claim: F128, parent: F128, msg: [F128; 2], r: F128) -> F128 {
    let [corner, inf] = msg;
    let (q0, q1) = if parent == F128::ONE {
        (corner, claim)
    } else {
        (
            (claim + parent * corner) * (F128::ONE + parent).inv(),
            corner,
        )
    };
    q0 + r * (q0 + q1 + inf + r * inf)
}
pub(crate) fn prove_inner<C: Challenger>(
    input: Input<'_>,
    point: &[F128],
    mut claim: F128,
    lambda: F128,
    scratch: &mut Scratch,
    ch: &mut C,
    p: &mut Timings,
) -> (InnerProof, Vec<F128>) {
    assert_eq!(input.len(), 1usize << point.len());
    let mut rounds = Vec::with_capacity(point.len());
    let mut q = Vec::with_capacity(point.len());
    let mut pending = if !point.is_empty() {
        let t = Instant::now();
        let w = split(&point[1..]);
        p.equality_ms += elapsed(t);
        let t = Instant::now();
        let v = message(input, &w, lambda, usize::from(point[0] != F128::ONE));
        p.messages_ms += elapsed(t);
        Some(v)
    } else {
        None
    };
    let mut len = input.len();
    for (i, &parent) in point.iter().enumerate() {
        let msg = pending.take().unwrap();
        ch.observe_f128_slice(&msg);
        let r = ch.sample_f128();
        claim = advance(claim, parent, msg, r);
        rounds.push(msg);
        q.push(r);
        let t = Instant::now();
        let w = (i + 1 < point.len()).then(|| split(&point[i + 2..]));
        p.equality_ms += elapsed(t);
        let corner = usize::from(point.get(i + 1).copied() != Some(F128::ONE));
        let t = Instant::now();
        len /= 2;
        if i == 0 {
            pending = fold_message(
                input,
                scratch.cur.each_mut().map(|v| &mut v[..len]),
                r,
                w.as_ref(),
                lambda,
                corner,
            );
        } else {
            pending = fold_message(
                Input::Separate(scratch.cur.each_ref().map(|v| &v[..2 * len])),
                scratch.next.each_mut().map(|v| &mut v[..len]),
                r,
                w.as_ref(),
                lambda,
                corner,
            );
            swap(&mut scratch.cur, &mut scratch.next);
        }
        p.fold_and_message_ms += elapsed(t);
    }
    let terminal = if point.is_empty() {
        input.at(0)
    } else {
        scratch.cur.each_ref().map(|v| v[0])
    };
    assert_eq!(
        claim,
        terminal[0] * terminal[1] + lambda * terminal[2] * terminal[3]
    );
    ch.observe_f128_slice(&terminal);
    (InnerProof { rounds, terminal }, q)
}
pub(crate) fn verify_inner<C: Challenger>(
    proof: &InnerProof,
    point: &[F128],
    mut claim: F128,
    lambda: F128,
    ch: &mut C,
) -> Result<Vec<F128>, Error> {
    if proof.rounds.len() != point.len() {
        return Err(Error::ProofShape);
    }
    let mut q = Vec::with_capacity(point.len());
    for (&parent, &msg) in point.iter().zip(&proof.rounds) {
        ch.observe_f128_slice(&msg);
        let r = ch.sample_f128();
        claim = advance(claim, parent, msg, r);
        q.push(r);
    }
    let t = proof.terminal;
    if claim != t[0] * t[1] + lambda * t[2] * t[3] {
        return Err(Error::Sumcheck);
    }
    ch.observe_f128_slice(&t);
    Ok(q)
}

pub fn prove<C: Challenger>(
    rows: &PreparedRows,
    options: Options,
    ch: &mut C,
) -> Result<(Proof, Claims, Stats, Timings), Error> {
    let mut p = Timings::default();
    rows.statement.bind(ch)?;
    options.validate()?;
    ch.observe_label(b"mikros-batched-coefficient-gkr-v2");
    let z = ch.sample_f128();
    let a = ProductTree::from_rows(
        &rows.values[0],
        rows.statement.left.width,
        z,
        options,
        Engine::Fused,
        &mut p,
    )?;
    let b = ProductTree::from_rows(
        &rows.values[1],
        rows.statement.right.width,
        z,
        options,
        Engine::Fused,
        &mut p,
    )?;
    let roots = [a.root(), b.root()];
    if roots[0] != roots[1] {
        return Err(Error::ProductMismatch);
    }
    ch.observe_f128_slice(&roots);
    let mut claims = roots;
    let mut point = Vec::new();
    let mut layers = Vec::new();
    let mut scratch = Scratch::new(if a.log_len > 1 {
        1usize << (a.log_len - 2)
    } else {
        0
    });
    for level in (0..a.log_len).rev() {
        ch.observe_bytes(&(level as u64).to_le_bytes());
        let lambda = ch.sample_f128();
        let t = Instant::now();
        let ac = Tree::layer(&a, level);
        let bc = Tree::layer(&b, level);
        p.layer_materialize_ms += elapsed(t);
        let (layer, q) = prove_inner(
            Input::Interleaved([&ac, &bc]),
            &point,
            claims[0] + lambda * claims[1],
            lambda,
            &mut scratch,
            ch,
            &mut p,
        );
        let r = ch.sample_f128();
        let t = layer.terminal;
        claims = [t[0] + r * (t[0] + t[1]), t[2] + r * (t[2] + t[3])];
        point = vec![r];
        point.extend(q);
        layers.push(layer);
    }
    let leaves = std::array::from_fn(|i| LeafClaim {
        point: point.clone(),
        value: claims[i] + z,
    });
    Ok((
        Proof { roots, layers },
        source_claims(&rows.statement, leaves)?,
        Stats {
            left: a.stats,
            right: b.stats,
        },
        p,
    ))
}
pub fn verify<C: Challenger>(
    statement: &Statement,
    proof: &Proof,
    ch: &mut C,
) -> Result<Claims, Error> {
    statement.bind(ch)?;
    let log = crate::padded_len(statement.left.rows)?.ilog2() as usize;
    if proof.layers.len() != log || proof.roots[0] != proof.roots[1] {
        return Err(Error::ProofShape);
    }
    ch.observe_label(b"mikros-batched-coefficient-gkr-v2");
    let z = ch.sample_f128();
    ch.observe_f128_slice(&proof.roots);
    let mut values = proof.roots;
    let mut point = Vec::new();
    for (level, layer) in (0..log).rev().zip(&proof.layers) {
        ch.observe_bytes(&(level as u64).to_le_bytes());
        let lambda = ch.sample_f128();
        let q = verify_inner(layer, &point, values[0] + lambda * values[1], lambda, ch)?;
        let r = ch.sample_f128();
        let t = layer.terminal;
        values = [t[0] + r * (t[0] + t[1]), t[2] + r * (t[2] + t[3])];
        point = vec![r];
        point.extend(q);
    }
    source_claims(
        statement,
        std::array::from_fn(|i| LeafClaim {
            point: point.clone(),
            value: values[i] + z,
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_hash::HashKind;
    use flock_multilinear::{IndexOrder, eq_table};
    use flock_transcript::challenger::FsChallenger;
    #[test]
    fn zero_one_eq_coordinates_and_zero_batch_challenge() {
        for point in [
            vec![F128::ZERO; 3],
            vec![F128::ONE; 3],
            vec![F128::ZERO, F128::ONE, F128::new(123, 456)],
        ] {
            let inputs: [Vec<F128>; 4] = std::array::from_fn(|s| {
                (0..8)
                    .map(|i| F128::new((i * 13 + s * 9) as u64, 1))
                    .collect()
            });
            for lambda in [F128::ZERO, F128::ONE, F128::new(321, 987)] {
                let weights = eq_table(&point, F128::ONE, IndexOrder::LowToHigh);
                let claim = (0..8).fold(F128::ZERO, |v, i| {
                    v + weights[i]
                        * (inputs[0][i] * inputs[1][i] + lambda * inputs[2][i] * inputs[3][i])
                });
                let mut c = FsChallenger::with_hash(b"degenerate", HashKind::Blake3);
                let (proof, q) = prove_inner(
                    Input::Separate(inputs.each_ref().map(|v| v.as_slice())),
                    &point,
                    claim,
                    lambda,
                    &mut Scratch::new(4),
                    &mut c,
                    &mut Timings::default(),
                );
                let mut v = FsChallenger::with_hash(b"degenerate", HashKind::Blake3);
                assert_eq!(
                    verify_inner(&proof, &point, claim, lambda, &mut v).unwrap(),
                    q
                );
                assert_eq!(c.sample_f128(), v.sample_f128());
            }
        }
    }
}
