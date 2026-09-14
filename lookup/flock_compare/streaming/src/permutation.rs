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
    product_tree::{ProductTree, Tree},
    profile::{Engine, Timings, elapsed},
    sumcheck::{self, LeafClaim},
};
use flock_field::F128;
use flock_transcript::challenger::Challenger;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Statement {
    pub left: OracleSpec,
    pub right: OracleSpec,
}
impl Statement {
    pub fn new_parallel(left: &PackedBits, right: &PackedBits) -> Result<Self, Error> {
        left.validate()?;
        right.validate()?;
        let result = Self {
            left: OracleSpec {
                rows: left.rows(),
                width: left.width(),
                digest: left.digest_parallel(),
            },
            right: OracleSpec {
                rows: right.rows(),
                width: right.width(),
                digest: right.digest_parallel(),
            },
        };
        result.validate()?;
        Ok(result)
    }
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

fn prove_trees<C: Challenger>(
    a: &mut impl Tree,
    b: &mut impl Tree,
    z: F128,
    c: &mut C,
    engine: Engine,
    p: &mut Timings,
) -> Result<(Proof, [LeafClaim; 2]), Error> {
    let roots = [a.root(), b.root()];
    if roots[0] != roots[1] {
        return Err(Error::ProductMismatch);
    }
    c.observe_f128_slice(&roots);
    c.observe_label(b"left-tree");
    let (left, mut ca) = if matches!(engine, Engine::Fused | Engine::Released) {
        crate::fast_sumcheck::prove(a, c, p, engine == Engine::Fused)
    } else {
        crate::reference::sumcheck::prove(a, c, p)
    };
    c.observe_label(b"right-tree");
    let (right, mut cb) = if matches!(engine, Engine::Fused | Engine::Released) {
        crate::fast_sumcheck::prove(b, c, p, engine == Engine::Fused)
    } else {
        crate::reference::sumcheck::prove(b, c, p)
    };
    ca.value += z;
    cb.value += z;
    Ok((Proof { roots, left, right }, [ca, cb]))
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
    prove_profiled(statement, left, right, options, Engine::Fused, c).map(|(a, b, c, _)| (a, b, c))
}
pub fn prove_profiled<C: Challenger>(
    statement: &Statement,
    left: &PackedBits,
    right: &PackedBits,
    options: Options,
    engine: Engine,
    c: &mut C,
) -> Result<(Proof, Claims, Stats, Timings), Error> {
    let mut p = Timings::default();
    let t = Instant::now();
    if engine == Engine::Reference {
        statement.left.check(left)?;
        statement.right.check(right)?;
    } else {
        check_source_parallel(&statement.left, left)?;
        check_source_parallel(&statement.right, right)?;
    }
    statement.bind(c)?;
    options.validate()?;
    p.source_checks_ms = elapsed(t);
    c.observe_label(b"mikros-permutation-grand-product-v1");
    let z = c.sample_f128();
    let (proof, leaves, stats) = if engine == Engine::Reference {
        let t = Instant::now();
        let va: Vec<_> = (0..left.rows()).map(|i| left.row(i)).collect();
        let vb: Vec<_> = (0..right.rows()).map(|i| right.row(i)).collect();
        p.decode_ms += elapsed(t);
        let mut a = crate::reference::product_tree::ProductTree::new(
            &va,
            left.width(),
            z,
            options,
            &mut p,
        )?;
        let mut b = crate::reference::product_tree::ProductTree::new(
            &vb,
            right.width(),
            z,
            options,
            &mut p,
        )?;
        let (proof, leaves) = prove_trees(&mut a, &mut b, z, c, engine, &mut p)?;
        (
            proof,
            leaves,
            Stats {
                left: a.stats,
                right: b.stats,
            },
        )
    } else {
        let mut a = ProductTree::from_bits(left, z, options, engine, &mut p)?;
        let mut b = ProductTree::from_bits(right, z, options, engine, &mut p)?;
        let (proof, leaves) = prove_trees(&mut a, &mut b, z, c, engine, &mut p)?;
        (
            proof,
            leaves,
            Stats {
                left: a.stats,
                right: b.stats,
            },
        )
    };
    Ok((proof, source_claims(statement, leaves)?, stats, p))
}
pub fn check_source_parallel(spec: &OracleSpec, bits: &PackedBits) -> Result<(), Error> {
    spec.validate()?;
    bits.validate()?;
    if spec.rows != bits.rows()
        || spec.width != bits.width()
        || spec.digest != bits.digest_parallel()
    {
        return Err(Error::Source);
    }
    Ok(())
}
pub fn verify<C: Challenger>(
    statement: &Statement,
    proof: &Proof,
    c: &mut C,
) -> Result<Claims, Error> {
    statement.bind(c)?;
    source_claims(statement, verify_encoded(proof, statement.left.rows, c)?)
}
