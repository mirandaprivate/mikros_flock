//! Ordinary Product-GKR for 256-bit injective copy tags. The random shift is
//! uniform F256; all sumcheck/fold coordinates lie in F128. Consequently each
//! terminal coordinate is an F128-linear query on the original committed bits.
//! No tag compression, witness lookup, or direct-source check is a verifier step.

use super::super::{
    fixture::{Circuit, Claims, from_raw},
    queries::{evaluate, lift, pairs, tensor},
};
use super::{Weights, check_layout, identity_labels};
use flock_core::{
    challenger::Challenger, pcs::functional::TensorClaim, zerocheck::univariate_skip::SplitEqGhash,
};
use flock_field::{F128, F256};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Layer {
    /// Cubic coefficients (g0, g2, g3); g1 follows from the incoming claim.
    pub rounds: Vec<[F256; 3]>,
    pub left: F256,
    pub right: F256,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TreeProof {
    pub roots: [F256; 2],
    /// Root-to-leaf order, with independently verified equations for both sides.
    pub layers: Vec<[Layer; 2]>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proof {
    pub tree: TreeProof,
    pub wire_values: [[F256; 3]; 2],
    pub label_values: Vec<F256>,
}

pub struct Prepared {
    digest: [u8; 32],
    tags: [Vec<F256>; 2],
}

#[derive(Debug)]
struct Terminals {
    /// Low-to-high coordinates shared by the two independently checked trees.
    point: Vec<F128>,
    values: [F256; 2],
}

fn shape(c: &Circuit) -> Result<(), String> {
    c.validate()?;
    if c.tag_bits() != 256 || c.value_bits != 128 || c.kappa() != 1 {
        return Err("wide copy requires 128-bit values and 256-bit tags".into());
    }
    Ok(())
}

/// Quadratic-extension coordinates provide a bijective F2-linear encoding:
/// the low F128 coordinate contains the public position, the high contains the
/// complete 128-bit value. Multiplication uses the actual irreducible F256 field.
fn tag(label: usize, value: u128) -> F256 {
    F256::new(from_raw(label as u128), from_raw(value))
}

pub fn prepare(c: &Circuit, z: &[F128]) -> Result<Prepared, String> {
    shape(c)?;
    if z.len() != c.witness_words() {
        return Err("wide copy witness dimensions".into());
    }
    let tags = std::array::from_fn(|side| {
        (0..c.copy_len())
            .into_par_iter()
            .map(|i| {
                if i >= c.live_len() {
                    return F256::ZERO;
                }
                let label = if side == 0 { c.sigma(i) } else { i };
                let value = if i == 3 * c.n() {
                    c.io
                } else {
                    c.packed_value(z, i)
                };
                tag(label, value)
            })
            .collect()
    });
    Ok(Prepared {
        digest: c.digest(),
        tags,
    })
}

fn observe<C: Challenger>(ch: &mut C, values: &[F256]) {
    for &value in values {
        ch.observe_f256(value);
    }
}

fn bind<C: Challenger>(log_len: usize, ch: &mut C) -> F256 {
    ch.observe_label(b"mikros-plonk-injective-copy-f256-gkr-v1");
    ch.observe_bytes(&(log_len as u64).to_le_bytes());
    // One double-width event, including when wrapped by GrindingChallenger.
    ch.sample_f256()
}

fn tree(mut leaves: Vec<F256>, shift: F256) -> Vec<Vec<F256>> {
    leaves.par_iter_mut().for_each(|x| *x += shift);
    let mut layers = vec![leaves];
    while layers.last().unwrap().len() > 1 {
        let parent = layers
            .last()
            .unwrap()
            .par_chunks_exact(2)
            .map(|p| p[0] * p[1])
            .collect();
        layers.push(parent);
    }
    layers
}

#[derive(Clone, Copy)]
enum Input<'a> {
    Interleaved(&'a [F256]),
    Separate(&'a [F256], &'a [F256]),
}
impl Input<'_> {
    fn len(self) -> usize {
        match self {
            Self::Interleaved(x) => x.len() / 2,
            Self::Separate(a, _) => a.len(),
        }
    }
    fn at(self, i: usize) -> (F256, F256) {
        match self {
            Self::Interleaved(x) => (x[2 * i], x[2 * i + 1]),
            Self::Separate(a, b) => (a[i], b[i]),
        }
    }
}

/// The equality factor stays in F128. Split tables avoid an O(N) equality
/// vector; summing a block before its high-coordinate scaling also saves work.
fn message(input: Input<'_>, point: &[F128], prefix: F128) -> [F256; 3] {
    let p = point[0];
    let weights = SplitEqGhash::with_n_hi(&point[1..], (point.len() - 1) / 2);
    let sums = weights
        .hi
        .par_iter()
        .enumerate()
        .map(|(hi, &high)| {
            let mut out = [F256::ZERO; 3];
            for (lo, &low) in weights.lo.iter().enumerate() {
                let index = hi * weights.lo.len() + lo;
                let (a0, b0) = input.at(2 * index);
                let (a1, b1) = input.at(2 * index + 1);
                let v0 = a0 * b0;
                let v2 = (a0 + a1) * (b0 + b1);
                let v1 = a1 * b1 + v0 + v2;
                out[0] += v0 * low;
                out[1] += v1 * low;
                out[2] += v2 * low;
            }
            out.map(|value| value * high)
        })
        .reduce(
            || [F256::ZERO; 3],
            |a, b| std::array::from_fn(|i| a[i] + b[i]),
        );
    [
        (sums[0] * (F128::ONE + p)) * prefix,
        (sums[1] + sums[2] * (F128::ONE + p)) * prefix,
        sums[2] * prefix,
    ]
}

fn fold(input: Input<'_>, r: F128) -> [Vec<F256>; 2] {
    let n = input.len() / 2;
    let mut a = vec![F256::ZERO; n];
    let mut b = vec![F256::ZERO; n];
    a.par_iter_mut()
        .zip(b.par_iter_mut())
        .enumerate()
        .for_each(|(i, (a, b))| {
            let (a0, b0) = input.at(2 * i);
            let (a1, b1) = input.at(2 * i + 1);
            *a = a0 + (a0 + a1) * r;
            *b = b0 + (b0 + b1) * r;
        });
    [a, b]
}

fn round_value(g: [F256; 3], claim: F256, r: F128) -> F256 {
    let g1 = claim + g[1] + g[2];
    g[0] + (g1 + (g[1] + g[2] * r) * r) * r
}

fn prove_tree<C: Challenger>(
    tags: [Vec<F256>; 2],
    ch: &mut C,
) -> Result<(TreeProof, Terminals), String> {
    let len = tags[0].len();
    if len < 2 || !len.is_power_of_two() || tags[1].len() != len {
        return Err("wide copy tree dimensions".into());
    }
    let log_len = len.ilog2() as usize;
    let shift = bind(log_len, ch);
    // Consume tag buffers in-place. Each tree totals fewer than 2R F256 words.
    let mut trees = tags.map(|source| tree(source, shift));
    let roots = std::array::from_fn(|side| trees[side].last().unwrap()[0]);
    if roots[0] != roots[1] {
        return Err("wide copy product mismatch".into());
    }
    observe(ch, &roots);
    let mut claims = roots;
    let mut point: Vec<F128> = Vec::new();
    let mut layers = Vec::with_capacity(log_len);
    for level in (0..log_len).rev() {
        ch.observe_label(b"mikros-plonk-copy-f256-layer-v1");
        ch.observe_bytes(&(level as u64).to_le_bytes());
        let inputs = [
            Input::Interleaved(&trees[0][level]),
            Input::Interleaved(&trees[1][level]),
        ];
        let mut buffers: Option<[[Vec<F256>; 2]; 2]> = None;
        let mut rounds: [Vec<[F256; 3]>; 2] =
            std::array::from_fn(|_| Vec::with_capacity(point.len()));
        let mut q = Vec::with_capacity(point.len());
        let mut prefix = F128::ONE;
        for i in 0..point.len() {
            let current = match &buffers {
                None => inputs,
                Some(b) => [
                    Input::Separate(&b[0][0], &b[0][1]),
                    Input::Separate(&b[1][0], &b[1][1]),
                ],
            };
            let messages = current.map(|input| message(input, &point[i..], prefix));
            for m in &messages {
                observe(ch, m);
            }
            let r = ch.sample_f128();
            for side in 0..2 {
                claims[side] = round_value(messages[side], claims[side], r);
                rounds[side].push(messages[side]);
            }
            let next = current.map(|input| fold(input, r));
            buffers = Some(next);
            prefix *= F128::ONE + point[i] + r;
            q.push(r);
        }
        let boundaries = match &buffers {
            None => inputs.map(|input| input.at(0)),
            Some(b) => [(b[0][0][0], b[0][1][0]), (b[1][0][0], b[1][1][0])],
        };
        for side in 0..2 {
            let (left, right) = boundaries[side];
            if (left * right) * prefix != claims[side] {
                return Err("wide copy sumcheck invariant".into());
            }
            observe(ch, &[left, right]);
        }
        let r = ch.sample_f128();
        claims = boundaries.map(|(a, b)| a + (a + b) * r);
        let [a, b] = rounds;
        layers.push([
            Layer {
                rounds: a,
                left: boundaries[0].0,
                right: boundaries[0].1,
            },
            Layer {
                rounds: b,
                left: boundaries[1].0,
                right: boundaries[1].1,
            },
        ]);
        point = vec![r];
        point.extend(q);
        drop(buffers);
        // The current child layer has made its last contribution. Never pool
        // these large F256 buffers across layers or proofs.
        for t in &mut trees {
            drop(std::mem::take(&mut t[level]));
        }
    }
    Ok((
        TreeProof { roots, layers },
        Terminals {
            point,
            values: claims.map(|v| v + shift),
        },
    ))
}

fn verify_tree<C: Challenger>(
    log_len: usize,
    proof: &TreeProof,
    ch: &mut C,
) -> Result<Terminals, String> {
    if log_len == 0 || proof.layers.len() != log_len || proof.roots[0] != proof.roots[1] {
        return Err("wide copy tree proof dimensions or roots".into());
    }
    let shift = bind(log_len, ch);
    observe(ch, &proof.roots);
    let mut claims = proof.roots;
    let mut point = Vec::new();
    for (layer, pair) in proof.layers.iter().enumerate() {
        if pair.iter().any(|p| p.rounds.len() != layer) {
            return Err("wide copy round count".into());
        }
        ch.observe_label(b"mikros-plonk-copy-f256-layer-v1");
        ch.observe_bytes(&((log_len - 1 - layer) as u64).to_le_bytes());
        let mut q = Vec::with_capacity(layer);
        let mut prefix = F128::ONE;
        for i in 0..layer {
            for p in pair {
                observe(ch, &p.rounds[i]);
            }
            let r = ch.sample_f128();
            for side in 0..2 {
                claims[side] = round_value(pair[side].rounds[i], claims[side], r);
            }
            prefix *= F128::ONE + point[i] + r;
            q.push(r);
        }
        for side in 0..2 {
            let p = &pair[side];
            if (p.left * p.right) * prefix != claims[side] {
                return Err("wide copy cubic replay".into());
            }
            observe(ch, &[p.left, p.right]);
        }
        let r = ch.sample_f128();
        claims =
            std::array::from_fn(|side| pair[side].left + (pair[side].left + pair[side].right) * r);
        point = vec![r];
        point.extend(q);
    }
    Ok(Terminals {
        point,
        values: claims.map(|v| v + shift),
    })
}

fn weights(c: &Circuit, point: &[F128]) -> Result<Weights, String> {
    if point.len() != c.h() {
        return Err("wide copy terminal point".into());
    }
    let k = c.kappa().ilog2() as usize;
    Ok(Weights {
        columns: tensor(&point[..k]),
        rows: pairs(&point[k..]),
    })
}

fn basis(component: usize) -> Vec<F128> {
    (0..256)
        .map(|bit| {
            if bit / 128 == component {
                from_raw(1u128 << (bit % 128))
            } else {
                F128::ZERO
            }
        })
        .collect()
}

fn add_claim(target: &mut Vec<TensorClaim>, q: TensorClaim) -> Result<(), String> {
    // A zero component is publicly known, so it needs no PCS query. Reject a
    // nonzero claimed value explicitly rather than silently discarding it.
    if q.columns.iter().all(|x| *x == F128::ZERO) {
        if q.value != F128::ZERO {
            return Err("wide copy nonzero empty coordinate".into());
        }
    } else {
        target.push(q);
    }
    Ok(())
}

fn expand(
    c: &Circuit,
    log_z: usize,
    log_public: usize,
    proof: &Proof,
    terminals: &Terminals,
) -> Result<Claims, String> {
    if proof.label_values.len() != 3 * c.label_digits() {
        return Err("wide copy label count".into());
    }
    let weights = weights(c, &terminals.point)?;
    let mut claims = Claims::default();
    for component in 0..2 {
        let basis = basis(component);
        for side in 0..2 {
            let labels = if side == 0 {
                proof
                    .label_values
                    .iter()
                    .fold(F128::ZERO, |a, v| a + v.coordinates()[component])
            } else {
                identity_labels(c, &weights, &basis)
            };
            let wires = proof.wire_values[side]
                .iter()
                .fold(F128::ZERO, |a, v| a + v.coordinates()[component]);
            let io_index = 3 * c.n();
            let io_label = if side == 0 {
                c.sigma(io_index)
            } else {
                io_index
            };
            let io = tag(io_label, c.io).coordinates()[component] * weights.at(io_index);
            if labels + wires + io != terminals.values[side].coordinates()[component] {
                return Err(format!(
                    "wide copy coordinate {component} side {side} recombination"
                ));
            }
            for wire in 0..3 {
                let mut q = weights.wire_query(c, wire, &basis[128..]);
                q.value = proof.wire_values[side][wire].coordinates()[component];
                add_claim(&mut claims.witness, lift(q, wire, log_z))?;
            }
            if side == 0 {
                for wire in 0..3 {
                    for digit in 0..c.label_digits() {
                        let start = digit * c.value_bits;
                        let mut q =
                            weights.wire_query(c, wire, &basis[start..start + c.value_bits]);
                        q.value = proof.label_values[wire * c.label_digits() + digit].coordinates()
                            [component];
                        add_claim(
                            &mut claims.public,
                            lift(q, 5 + wire * c.label_digits() + digit, log_public),
                        )?;
                    }
                }
            }
        }
    }
    Ok(claims)
}

fn observe_values<C: Challenger>(proof: &Proof, ch: &mut C) {
    for values in &proof.wire_values {
        observe(ch, values);
    }
    observe(ch, &proof.label_values);
}

pub fn prove<C: Challenger>(
    c: &Circuit,
    z: &[F128],
    public: &[F128],
    prepared: Prepared,
    ch: &mut C,
) -> Result<(Proof, Claims), String> {
    shape(c)?;
    if z.len() != c.witness_words()
        || public.len() != c.public_words()
        || prepared.digest != c.digest()
        || prepared.tags.iter().any(|x| x.len() != c.copy_len())
    {
        return Err("wide copy prepared dimensions".into());
    }
    let (tree, terminals) = prove_tree(prepared.tags, ch)?;
    let weights = weights(c, &terminals.point)?;
    let mut proof = Proof {
        tree,
        wire_values: [[F256::ZERO; 3]; 2],
        label_values: vec![F256::ZERO; 3 * c.label_digits()],
    };
    let m = c.wire_words();
    for component in 0..2 {
        let basis = basis(component);
        for side in 0..2 {
            for wire in 0..3 {
                let q = weights.wire_query(c, wire, &basis[128..]);
                let value = if component == 0 {
                    F128::ZERO
                } else {
                    evaluate(&z[wire * m..(wire + 1) * m], &q)
                };
                let mut coordinates = proof.wire_values[side][wire].coordinates();
                coordinates[component] = value;
                proof.wire_values[side][wire] = F256::new(coordinates[0], coordinates[1]);
            }
            if side == 0 {
                for wire in 0..3 {
                    for digit in 0..c.label_digits() {
                        let start = digit * c.value_bits;
                        let q = weights.wire_query(c, wire, &basis[start..start + c.value_bits]);
                        let block = 5 + wire * c.label_digits() + digit;
                        let value = if component == 1 {
                            F128::ZERO
                        } else {
                            evaluate(&public[block * m..(block + 1) * m], &q)
                        };
                        let i = wire * c.label_digits() + digit;
                        let mut coordinates = proof.label_values[i].coordinates();
                        coordinates[component] = value;
                        proof.label_values[i] = F256::new(coordinates[0], coordinates[1]);
                    }
                }
            }
        }
    }
    observe_values(&proof, ch);
    let claims = expand(
        c,
        z.len().ilog2() as usize,
        public.len().ilog2() as usize,
        &proof,
        &terminals,
    )?;
    Ok((proof, claims))
}

pub fn verify<C: Challenger>(
    c: &Circuit,
    log_z: usize,
    log_public: usize,
    proof: &Proof,
    ch: &mut C,
) -> Result<Claims, String> {
    shape(c)?;
    check_layout(c, log_z, log_public)?;
    if proof.label_values.len() != 3 * c.label_digits() {
        return Err("wide copy label count".into());
    }
    let terminals = verify_tree(c.h(), &proof.tree, ch)?;
    observe_values(proof, ch);
    expand(c, log_z, log_public, proof, &terminals)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::challenger::FsChallenger;

    #[test]
    fn f256_tree_both_coordinates_source_oracle_and_tampering() {
        for log in [1, 2, 5, 8] {
            let source: Vec<_> = (0..1usize << log)
                .map(|i| {
                    F256::new(
                        F128::new(i as u64 * 31 + 7, i as u64 + 3),
                        F128::new(i as u64 * 29 + 5, 1u64 << 63),
                    )
                })
                .collect();
            let mut right = source.clone();
            right.reverse();
            let mut pc = FsChallenger::new(b"wide-copy-independent");
            let (proof, terminals) = prove_tree([source.clone(), right.clone()], &mut pc).unwrap();
            let mut vc = FsChallenger::new(b"wide-copy-independent");
            let got = verify_tree(log, &proof, &mut vc).unwrap();
            assert_eq!(got.point, terminals.point);
            assert_eq!(got.values, terminals.values);
            assert_eq!(pc.sample_f128(), vc.sample_f128());
            for (side, source) in [source, right].iter().enumerate() {
                let expected =
                    source
                        .iter()
                        .enumerate()
                        .fold(F256::ZERO, |sum, (index, &value)| {
                            let w = got
                                .point
                                .iter()
                                .enumerate()
                                .fold(F128::ONE, |w, (bit, &r)| {
                                    w * if index >> bit & 1 == 0 {
                                        F128::ONE + r
                                    } else {
                                        r
                                    }
                                });
                            sum + value * w
                        });
                assert_eq!(got.values[side], expected);
            }
            let mut bad = proof.clone();
            bad.layers.last_mut().unwrap()[0].left.c1 += F128::ONE;
            assert!(
                verify_tree(log, &bad, &mut FsChallenger::new(b"wide-copy-independent")).is_err()
            );
            let mut bad = proof;
            bad.layers.pop();
            assert!(
                verify_tree(log, &bad, &mut FsChallenger::new(b"wide-copy-independent")).is_err()
            );
        }
    }
}
