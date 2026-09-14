//! GKR reduction for a multiplication tree. Each round has degree three:
//! eq(parent_point, x) * left(x) * right(x).
use crate::Error;
use flock_field::F128;
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
        let mut q = Vec::new();
        for &g in &layer.rounds {
            c.observe_f128_slice(&g);
            let t = c.sample_f128();
            claim = evaluate(g, claim, t);
            q.push(t);
        }
        let eq = point
            .iter()
            .zip(&q)
            .fold(F128::ONE, |acc, (&r, &s)| acc * (F128::ONE + r + s));
        if claim != eq * layer.left * layer.right {
            return Err(Error::Sumcheck);
        }
        c.observe_f128_slice(&[layer.left, layer.right]);
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
