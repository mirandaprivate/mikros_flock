//! Equal-length multiset proof. Verifier output must be authenticated against
//! both original bit oracles; merely accepting the product roots is insufficient.
pub use crate::product_tree::{Options, Strategy, TreeStats};
pub use crate::sumcheck::{LayerProof, TreeProof};
use crate::{
    Error,
    bits::PackedBits,
    claims::{Claims, LinearQuery, OracleClaim, OracleSpec, Source},
    field::SmallField,
    padded_len,
    product_tree::ProductTree,
    sumcheck::{self, LeafClaim},
};
use flock_field::F128;
use flock_transcript::challenger::Challenger;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Statement {
    pub left: OracleSpec,
    pub right: OracleSpec,
}
impl Statement {
    pub fn new(left: &PackedBits, right: &PackedBits) -> Result<Self, Error> {
        let result = Self {
            left: OracleSpec::from_bits(left)?,
            right: OracleSpec::from_bits(right)?,
        };
        result.validate()?;
        Ok(result)
    }
    fn validate(&self) -> Result<(), Error> {
        self.left.validate()?;
        self.right.validate()?;
        if self.left.rows != self.right.rows || self.left.width != self.right.width {
            return Err(Error::Statement);
        }
        Ok(())
    }
    fn bind<C: Challenger>(&self, c: &mut C) -> Result<(), Error> {
        self.validate()?;
        c.observe_label(b"mikros-bitwidth-permutation-v1");
        c.observe_bytes(&(self.left.rows as u64).to_le_bytes());
        c.observe_bytes(&(self.left.width as u64).to_le_bytes());
        c.observe_bytes(&self.left.digest);
        c.observe_bytes(&self.right.digest);
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    pub roots: [F128; 2],
    pub left: TreeProof,
    pub right: TreeProof,
}
impl Proof {
    pub fn field_elements(&self) -> usize {
        2 + [&self.left, &self.right]
            .iter()
            .flat_map(|p| &p.layers)
            .map(|l| l.rounds.len() * 3 + 2)
            .sum::<usize>()
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stats {
    pub left: TreeStats,
    pub right: TreeStats,
}

pub(crate) fn prove_encoded<C: Challenger>(
    left: &[u128],
    right: &[u128],
    width: usize,
    options: Options,
    c: &mut C,
) -> Result<(Proof, [LeafClaim; 2], Stats), Error> {
    if left.len() != right.len() {
        return Err(Error::Length);
    }
    options.validate()?;
    c.observe_label(b"mikros-permutation-grand-product-v1");
    let z = c.sample_f128();
    let a = ProductTree::new(left, width, z, options)?;
    let b = ProductTree::new(right, width, z, options)?;
    let roots = [a.root(), b.root()];
    if roots[0] != roots[1] {
        return Err(Error::ProductMismatch);
    }
    c.observe_f128_slice(&roots);
    c.observe_label(b"left-tree");
    let (left, mut claim_a) = sumcheck::prove(&a, c);
    c.observe_label(b"right-tree");
    let (right, mut claim_b) = sumcheck::prove(&b, c);
    // Every padded factor is z + d, with d=0 in padding; eq weights sum to one.
    claim_a.value += z;
    claim_b.value += z;
    Ok((
        Proof { roots, left, right },
        [claim_a, claim_b],
        Stats {
            left: a.stats,
            right: b.stats,
        },
    ))
}
pub(crate) fn verify_encoded<C: Challenger>(
    proof: &Proof,
    rows: usize,
    c: &mut C,
) -> Result<[LeafClaim; 2], Error> {
    let log_len = padded_len(rows)?.ilog2() as usize;
    c.observe_label(b"mikros-permutation-grand-product-v1");
    let z = c.sample_f128();
    if proof.roots[0] != proof.roots[1] {
        return Err(Error::ProductMismatch);
    }
    c.observe_f128_slice(&proof.roots);
    c.observe_label(b"left-tree");
    let mut a = sumcheck::verify(&proof.left, proof.roots[0], log_len, c)?;
    c.observe_label(b"right-tree");
    let mut b = sumcheck::verify(&proof.right, proof.roots[1], log_len, c)?;
    a.value += z;
    b.value += z;
    Ok([a, b])
}
fn source_claims(statement: &Statement, leaves: [LeafClaim; 2]) -> Result<Claims, Error> {
    let basis =
        SmallField::for_width(statement.left.width)?.basis()[..statement.left.width].to_vec();
    Ok(Claims {
        oracles: vec![
            (Source::Left, statement.left.clone()),
            (Source::Right, statement.right.clone()),
        ],
        evaluations: [Source::Left, Source::Right]
            .into_iter()
            .zip(leaves)
            .map(|(source, leaf)| OracleClaim {
                source,
                query: LinearQuery::Rows {
                    point: leaf.point,
                    basis: basis.clone(),
                    row_offset: 0,
                },
                value: leaf.value,
            })
            .collect(),
    })
}
pub fn prove<C: Challenger>(
    statement: &Statement,
    left: &PackedBits,
    right: &PackedBits,
    options: Options,
    c: &mut C,
) -> Result<(Proof, Claims, Stats), Error> {
    statement.left.check(left)?;
    statement.right.check(right)?;
    statement.bind(c)?;
    let a: Vec<_> = (0..left.rows()).map(|i| left.row(i)).collect();
    let b: Vec<_> = (0..right.rows()).map(|i| right.row(i)).collect();
    let (proof, leaves, stats) = prove_encoded(&a, &b, left.width(), options, c)?;
    Ok((proof, source_claims(statement, leaves)?, stats))
}
pub fn verify<C: Challenger>(
    statement: &Statement,
    proof: &Proof,
    c: &mut C,
) -> Result<Claims, Error> {
    statement.bind(c)?;
    source_claims(statement, verify_encoded(proof, statement.left.rows, c)?)
}
