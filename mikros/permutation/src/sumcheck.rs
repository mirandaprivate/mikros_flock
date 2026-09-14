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
    let [claim] = verify_impl([proof], [root], log_len, c)?;
    Ok(claim)
}

/// Verify both constraints separately; no linear combination can cancel errors
/// across the two trees. Messages and child evaluations are fixed for BOTH
/// trees before the common round or pairing challenge is sampled.
pub(crate) fn verify_pair<C: Challenger>(
    proofs: [&TreeProof; 2],
    roots: [F128; 2],
    log_len: usize,
    c: &mut C,
) -> Result<[LeafClaim; 2], Error> {
    verify_impl(proofs, roots, log_len, c)
}

fn verify_impl<const N: usize, C: Challenger>(
    proofs: [&TreeProof; N],
    mut claims: [F128; N],
    log_len: usize,
    c: &mut C,
) -> Result<[LeafClaim; N], Error> {
    if proofs.iter().any(|proof| proof.layers.len() != log_len) {
        return Err(Error::ProofShape);
    }
    let mut point = Vec::new();
    for level in 0..log_len {
        let layers = proofs.map(|proof| &proof.layers[level]);
        if layers.iter().any(|layer| layer.rounds.len() != level) {
            return Err(Error::ProofShape);
        }
        let mut q = Vec::new();
        for round in 0..level {
            for layer in &layers {
                c.observe_f128_slice(&layer.rounds[round]);
            }
            let t = c.sample_f128();
            for j in 0..N {
                claims[j] = evaluate(layers[j].rounds[round], claims[j], t);
            }
            q.push(t);
        }
        let eq = point
            .iter()
            .zip(&q)
            .fold(F128::ONE, |acc, (&r, &s)| acc * (F128::ONE + r + s));
        for j in 0..N {
            let layer = layers[j];
            if claims[j] != eq * layer.left * layer.right {
                return Err(Error::Sumcheck);
            }
        }
        for layer in &layers {
            c.observe_f128_slice(&[layer.left, layer.right]);
        }
        let rho = c.sample_f128();
        for j in 0..N {
            let layer = layers[j];
            claims[j] = layer.left + rho * (layer.left + layer.right);
        }
        point = vec![rho];
        point.extend(q);
    }
    Ok(claims.map(|value| LeafClaim {
        point: point.clone(),
        value,
    }))
}
