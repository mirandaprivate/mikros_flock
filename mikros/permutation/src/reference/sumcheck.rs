//! GKR reduction for a multiplication tree. Each round has degree three:
//! eq(parent_point, x) * left(x) * right(x).
use crate::sumcheck::{LayerProof, LeafClaim, TreeProof};
use crate::{
    product_tree::Tree,
    profile::{Timings, elapsed},
};
use flock_field::F128;
use flock_multilinear::{IndexOrder, eq_table, fold_low};
use flock_transcript::challenger::Challenger;
use std::time::Instant;

fn evaluate([g0, g2, g3]: [F128; 3], claim: F128, t: F128) -> F128 {
    g0 + t * (claim + g2 + g3 + t * (g2 + t * g3))
}
pub(crate) fn prove<C: Challenger>(
    tree: &impl Tree,
    c: &mut C,
    p: &mut Timings,
) -> (TreeProof, LeafClaim) {
    let ([proof], [claim]) = prove_impl([tree], c, p);
    (proof, claim)
}

pub(crate) fn prove_pair<C: Challenger>(
    left: &impl Tree,
    right: &impl Tree,
    c: &mut C,
    p: &mut Timings,
) -> ([TreeProof; 2], [LeafClaim; 2]) {
    prove_impl([left, right], c, p)
}

fn prove_impl<const N: usize, C: Challenger>(
    trees: [&dyn Tree; N],
    c: &mut C,
    p: &mut Timings,
) -> ([TreeProof; N], [LeafClaim; N]) {
    let log_len = trees[0].log_len();
    assert!(trees.iter().all(|tree| tree.log_len() == log_len));
    let mut point = Vec::new();
    let mut claims = trees.map(|tree| tree.root());
    let mut layers = std::array::from_fn::<_, N, _>(|_| Vec::with_capacity(log_len));
    for level in (0..log_len).rev() {
        let phase = Instant::now();
        let children = trees.map(|tree| tree.layer(level));
        let mut a = std::array::from_fn::<_, N, _>(|j| {
            children[j]
                .chunks_exact(2)
                .map(|p| p[0])
                .collect::<Vec<_>>()
        });
        let mut b = std::array::from_fn::<_, N, _>(|j| {
            children[j]
                .chunks_exact(2)
                .map(|p| p[1])
                .collect::<Vec<_>>()
        });
        p.layer_materialize_ms += elapsed(phase);
        let phase = Instant::now();
        let mut e = eq_table(&point, F128::ONE, IndexOrder::LowToHigh);
        p.equality_ms += elapsed(phase);
        let mut rounds = std::array::from_fn::<_, N, _>(|_| Vec::with_capacity(point.len()));
        let mut q = Vec::new();
        for _ in 0..point.len() {
            let phase = Instant::now();
            let mut messages = [[F128::ZERO; 3]; N];
            for j in 0..N {
                for i in 0..a[j].len() / 2 {
                    let a0 = a[j][2 * i];
                    let da = a0 + a[j][2 * i + 1];
                    let b0 = b[j][2 * i];
                    let db = b0 + b[j][2 * i + 1];
                    let e0 = e[2 * i];
                    let de = e0 + e[2 * i + 1];
                    let ab0 = a0 * b0;
                    let ab1 = a0 * db + da * b0;
                    let ab2 = da * db;
                    messages[j][0] += e0 * ab0;
                    messages[j][1] += e0 * ab2 + de * ab1;
                    messages[j][2] += de * ab2;
                }
            }
            p.messages_ms += elapsed(phase);
            for g in &messages {
                c.observe_f128_slice(g);
            }
            let t = c.sample_f128();
            for j in 0..N {
                claims[j] = evaluate(messages[j], claims[j], t);
                rounds[j].push(messages[j]);
            }
            q.push(t);
            let phase = Instant::now();
            for j in 0..N {
                fold_low(&mut a[j], t);
                fold_low(&mut b[j], t);
            }
            fold_low(&mut e, t);
            p.folds_ms += elapsed(phase);
        }
        for j in 0..N {
            debug_assert_eq!(claims[j], e[0] * a[j][0] * b[j][0]);
            c.observe_f128_slice(&[a[j][0], b[j][0]]);
        }
        let rho = c.sample_f128();
        for j in 0..N {
            claims[j] = a[j][0] + rho * (a[j][0] + b[j][0]);
        }
        point = vec![rho];
        point.extend(q);
        for (j, rounds) in rounds.into_iter().enumerate() {
            layers[j].push(LayerProof {
                rounds,
                left: a[j][0],
                right: b[j][0],
            });
        }
    }
    (
        layers.map(|layers| TreeProof { layers }),
        claims.map(|value| LeafClaim {
            point: point.clone(),
            value,
        }),
    )
}
