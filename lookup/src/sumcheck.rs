//! GKR reduction for a multiplication tree. Each round has degree three:
//! eq(parent_point, x) * left(x) * right(x).
use crate::{Error, product_tree::ProductTree};
use flock_field::F128;
use flock_multilinear::{IndexOrder, eq_table, fold_low};
use flock_transcript::challenger::Challenger;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayerProof {
    pub rounds: Vec<[F128; 3]>,
    pub left: F128,
    pub right: F128,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TreeProof {
    pub layers: Vec<LayerProof>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LeafClaim {
    pub point: Vec<F128>,
    pub value: F128,
}
fn evaluate([g0, g2, g3]: [F128; 3], claim: F128, t: F128) -> F128 {
    g0 + t * (claim + g2 + g3 + t * (g2 + t * g3))
}
/// Shared degree-three reduction, also used to authenticate Booleanity.
/// Terminal values are absorbed here; the caller supplies any next challenge.
pub(crate) fn prove_product<C: Challenger>(
    point: &[F128],
    mut a: Vec<F128>,
    mut b: Vec<F128>,
    mut claim: F128,
    c: &mut C,
) -> (LayerProof, Vec<F128>) {
    assert_eq!(a.len(), 1usize << point.len());
    assert_eq!(a.len(), b.len());
    let mut e = eq_table(point, F128::ONE, IndexOrder::LowToHigh);
    let mut rounds = Vec::new();
    let mut q = Vec::new();
    for _ in 0..point.len() {
        let mut g = [F128::ZERO; 3];
        for i in 0..a.len() / 2 {
            let a0 = a[2 * i];
            let da = a0 + a[2 * i + 1];
            let b0 = b[2 * i];
            let db = b0 + b[2 * i + 1];
            let e0 = e[2 * i];
            let de = e0 + e[2 * i + 1];
            let ab0 = a0 * b0;
            let ab1 = a0 * db + da * b0;
            let ab2 = da * db;
            g[0] += e0 * ab0;
            g[1] += e0 * ab2 + de * ab1;
            g[2] += de * ab2;
        }
        c.observe_f128_slice(&g);
        let t = c.sample_f128();
        claim = evaluate(g, claim, t);
        rounds.push(g);
        q.push(t);
        fold_low(&mut a, t);
        fold_low(&mut b, t);
        fold_low(&mut e, t);
    }
    debug_assert_eq!(claim, e[0] * a[0] * b[0]);
    c.observe_f128_slice(&[a[0], b[0]]);
    (
        LayerProof {
            rounds,
            left: a[0],
            right: b[0],
        },
        q,
    )
}
pub(crate) fn verify_product<C: Challenger>(
    proof: &LayerProof,
    point: &[F128],
    mut claim: F128,
    c: &mut C,
) -> Result<Vec<F128>, Error> {
    if proof.rounds.len() != point.len() {
        return Err(Error::ProofShape);
    }
    let mut q = Vec::new();
    for &g in &proof.rounds {
        c.observe_f128_slice(&g);
        let t = c.sample_f128();
        claim = evaluate(g, claim, t);
        q.push(t);
    }
    let eq = point
        .iter()
        .zip(&q)
        .fold(F128::ONE, |acc, (&r, &s)| acc * (F128::ONE + r + s));
    if claim != eq * proof.left * proof.right {
        return Err(Error::Sumcheck);
    }
    c.observe_f128_slice(&[proof.left, proof.right]);
    Ok(q)
}
pub(crate) fn prove<C: Challenger>(tree: &ProductTree, c: &mut C) -> (TreeProof, LeafClaim) {
    let mut point = Vec::new();
    let mut claim = tree.root();
    let mut layers = Vec::new();
    for level in (0..tree.log_len).rev() {
        let children = tree.layer(level);
        let a: Vec<_> = children.chunks_exact(2).map(|p| p[0]).collect();
        let b: Vec<_> = children.chunks_exact(2).map(|p| p[1]).collect();
        let (layer, q) = prove_product(&point, a, b, claim, c);
        let rho = c.sample_f128();
        claim = layer.left + rho * (layer.left + layer.right);
        point = vec![rho];
        point.extend(q);
        layers.push(layer);
    }
    (
        TreeProof { layers },
        LeafClaim {
            point,
            value: claim,
        },
    )
}
pub(crate) fn verify<C: Challenger>(
    proof: &TreeProof,
    root: F128,
    log_len: usize,
    c: &mut C,
) -> Result<LeafClaim, Error> {
    if proof.layers.len() != log_len {
        return Err(Error::ProofShape);
    }
    let mut point = Vec::new();
    let mut claim = root;
    for (level, layer) in proof.layers.iter().enumerate() {
        if layer.rounds.len() != level {
            return Err(Error::ProofShape);
        }
        let q = verify_product(layer, &point, claim, c)?;
        let rho = c.sample_f128();
        claim = layer.left + rho * (layer.left + layer.right);
        point = vec![rho];
        point.extend(q);
    }
    Ok(LeafClaim {
        point,
        value: claim,
    })
}
