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
    product_tree::{PreparedTreeCoefficients, ProductTree, Tree},
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

/// Shape of two already committed bit oracles. The caller binds the actual
/// commitment and public statement before entering this protocol.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommittedStatement {
    pub rows: usize,
    pub width: usize,
}

impl CommittedStatement {
    pub fn new(rows: usize, width: usize) -> Result<Self, Error> {
        let out = Self { rows, width };
        out.validate()?;
        Ok(out)
    }

    fn validate(&self) -> Result<(), Error> {
        padded_len(self.rows)?;
        SmallField::for_width(self.width)?;
        Ok(())
    }

    fn bind<C: Challenger>(&self, c: &mut C) -> Result<(), Error> {
        self.validate()?;
        c.observe_label(b"mikros-committed-bitwidth-permutation-v1");
        c.observe_bytes(&(self.rows as u64).to_le_bytes());
        c.observe_bytes(&(self.width as u64).to_le_bytes());
        Ok(())
    }
}

/// Offline validated inputs. Fields are private so the online proof cannot
/// accidentally use data of another shape. This is prover state, not a proof.
pub struct PreparedPermutation {
    statement: CommittedStatement,
    left: PackedBits,
    right: PackedBits,
}

impl PreparedPermutation {
    pub fn statement(&self) -> &CommittedStatement {
        &self.statement
    }

    pub fn left(&self) -> &PackedBits {
        &self.left
    }

    pub fn right(&self) -> &PackedBits {
        &self.right
    }
}

/// Validate and take ownership of source storage before online proving.
pub fn prepare(left: PackedBits, right: PackedBits) -> Result<PreparedPermutation, Error> {
    left.validate()?;
    right.validate()?;
    if left.rows() != right.rows() || left.width() != right.width() {
        return Err(Error::Statement);
    }
    let statement = CommittedStatement::new(left.rows(), left.width())?;
    Ok(PreparedPermutation {
        statement,
        left,
        right,
    })
}

/// Offline decoded source tuples and all challenge-independent compact monic
/// coefficients. This object owns its buffers and does not retain the original
/// PackedBits sources, so callers may release them after preparation.
pub struct PreparedCoefficients {
    statement: CommittedStatement,
    options: Options,
    left: PreparedTreeCoefficients,
    right: PreparedTreeCoefficients,
}

impl PreparedCoefficients {
    pub fn statement(&self) -> &CommittedStatement {
        &self.statement
    }

    pub fn options(&self) -> Options {
        self.options
    }

    /// Sum of coefficient-buffer and private offline leaf-table capacities for
    /// both trees, including the highest compact layer evaluated online. Shared
    /// global field tables and small metadata are excluded; source PackedBits
    /// are not retained by this object.
    pub fn resident_bytes(&self) -> usize {
        self.left.resident_bytes() + self.right.resident_bytes()
    }
}

pub fn prepare_tree_coefficients(
    source: &PreparedPermutation,
    options: Options,
) -> Result<PreparedCoefficients, Error> {
    options.validate()?;
    let left = PreparedTreeCoefficients::new(&source.left, options)?;
    let right = PreparedTreeCoefficients::new(&source.right, options)?;
    Ok(PreparedCoefficients {
        statement: source.statement,
        options,
        left,
        right,
    })
}

/// Two deferred source queries. They deliberately carry no pretend source
/// digest: their authentication is the caller's actual polynomial commitment.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommittedClaims {
    pub evaluations: [OracleClaim; 2],
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

#[derive(Clone, Copy, PartialEq, Eq)]
enum Schedule {
    Sequential,
    Lockstep,
}

impl Schedule {
    fn domain(self) -> &'static [u8] {
        match self {
            Self::Sequential => b"mikros-permutation-grand-product-v1",
            Self::Lockstep => b"mikros-permutation-grand-product-lockstep-v1",
        }
    }
}

fn prove_trees<C: Challenger>(
    a: &mut impl Tree,
    b: &mut impl Tree,
    z: F128,
    c: &mut C,
    engine: Engine,
    p: &mut Timings,
    schedule: Schedule,
) -> Result<(Proof, [LeafClaim; 2]), Error> {
    let roots = [a.root(), b.root()];
    if roots[0] != roots[1] {
        return Err(Error::ProductMismatch);
    }
    c.observe_f128_slice(&roots);
    if schedule == Schedule::Lockstep {
        let ([left, right], mut leaves) = if engine.fused_tree() || engine == Engine::Released {
            crate::fast_sumcheck::prove_pair(
                a,
                b,
                c,
                p,
                engine.fused_tree(),
                engine.register_fold(),
            )
        } else {
            crate::reference::sumcheck::prove_pair(a, b, c, p)
        };
        for leaf in &mut leaves {
            leaf.value += z;
        }
        return Ok((Proof { roots, left, right }, leaves));
    }
    c.observe_label(b"left-tree");
    let (left, mut ca) = if engine.register_fold() {
        crate::fast_sumcheck::prove_register(a, c, p, true)
    } else if engine.fused_tree() || engine == Engine::Released {
        crate::fast_sumcheck::prove(a, c, p, engine.fused_tree())
    } else {
        crate::reference::sumcheck::prove(a, c, p)
    };
    c.observe_label(b"right-tree");
    let (right, mut cb) = if engine.register_fold() {
        crate::fast_sumcheck::prove_register(b, c, p, true)
    } else if engine.fused_tree() || engine == Engine::Released {
        crate::fast_sumcheck::prove(b, c, p, engine.fused_tree())
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
    verify_encoded_impl(proof, rows, c, Schedule::Sequential)
}

fn verify_encoded_impl<C: Challenger>(
    proof: &Proof,
    rows: usize,
    c: &mut C,
    schedule: Schedule,
) -> Result<[LeafClaim; 2], Error> {
    let log_len = padded_len(rows)?.ilog2() as usize;
    c.observe_label(schedule.domain());
    let z = c.sample_f128();
    if proof.roots[0] != proof.roots[1] {
        return Err(Error::ProductMismatch);
    }
    c.observe_f128_slice(&proof.roots);
    if schedule == Schedule::Lockstep {
        let mut leaves =
            sumcheck::verify_pair([&proof.left, &proof.right], proof.roots, log_len, c)?;
        for leaf in &mut leaves {
            leaf.value += z;
        }
        return Ok(leaves);
    }
    c.observe_label(b"left-tree");
    let mut a = sumcheck::verify(&proof.left, proof.roots[0], log_len, c)?;
    c.observe_label(b"right-tree");
    let mut b = sumcheck::verify(&proof.right, proof.roots[1], log_len, c)?;
    a.value += z;
    b.value += z;
    Ok([a, b])
}
fn source_claims(statement: &Statement, leaves: [LeafClaim; 2]) -> Result<Claims, Error> {
    Ok(Claims {
        oracles: vec![
            (Source::Left, statement.left.clone()),
            (Source::Right, statement.right.clone()),
        ],
        evaluations: leaf_queries(statement.left.width, leaves)?.to_vec(),
    })
}

fn leaf_queries(width: usize, leaves: [LeafClaim; 2]) -> Result<[OracleClaim; 2], Error> {
    let basis = SmallField::for_width(width)?.basis()[..width].to_vec();
    Ok([Source::Left, Source::Right]
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
        .collect::<Vec<_>>()
        .try_into()
        .expect("exactly two leaves"))
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
    let (proof, leaves, stats, p) =
        prove_after_binding(left, right, options, engine, p, c, Schedule::Sequential)?;
    Ok((proof, source_claims(statement, leaves)?, stats, p))
}

/// Prove a permutation of already committed oracles. All online work remains
/// inside this call; only input validation/storage preparation is offline.
/// Both returned queries MUST be opened against the commitments bound by the
/// caller before this call. Merely verifying the GKR roots is insufficient.
pub fn prove_committed<C: Challenger>(
    prepared: &PreparedPermutation,
    options: Options,
    engine: Engine,
    c: &mut C,
) -> Result<(Proof, CommittedClaims, Stats, Timings), Error> {
    prove_committed_impl(prepared, options, engine, c, Schedule::Sequential)
}

/// Synchronized two-tree variant. Each tree keeps its own cubic equation and
/// proof messages, but all round and pairing challenges are shared. The two
/// returned source claims have the SAME point; their values need not be equal
/// for general source oracles. Both still require commitment authentication.
/// This uses a distinct versioned domain and must be replayed with the matching
/// lockstep API; as with all PIOP entry points, acceptance also requires PCS.
pub fn prove_committed_lockstep<C: Challenger>(
    prepared: &PreparedPermutation,
    options: Options,
    engine: Engine,
    c: &mut C,
) -> Result<(Proof, CommittedClaims, Stats, Timings), Error> {
    prove_committed_impl(prepared, options, engine, c, Schedule::Lockstep)
}

fn prove_committed_impl<C: Challenger>(
    prepared: &PreparedPermutation,
    options: Options,
    engine: Engine,
    c: &mut C,
    schedule: Schedule,
) -> Result<(Proof, CommittedClaims, Stats, Timings), Error> {
    options.validate()?;
    prepared.statement.bind(c)?;
    let (proof, leaves, stats, p) = prove_after_binding(
        &prepared.left,
        &prepared.right,
        options,
        engine,
        Timings::default(),
        c,
        schedule,
    )?;
    Ok((
        proof,
        CommittedClaims {
            evaluations: leaf_queries(prepared.statement.width, leaves)?,
        },
        stats,
        p,
    ))
}

/// Strict online entry: immutable source decoding and compact coefficients
/// were prepared offline. This call builds every z-dependent evaluation table,
/// evaluates the highest compact layer, constructs the extension tree, runs
/// GKR and derives both leaf queries. No source scan or coefficient copy occurs.
/// It uses exactly the same committed statement binding as prove_committed.
pub fn prove_prepared<C: Challenger>(
    prepared: &PreparedCoefficients,
    engine: Engine,
    c: &mut C,
) -> Result<(Proof, CommittedClaims, Stats, Timings), Error> {
    prove_prepared_impl(prepared, engine, c, Schedule::Sequential)
}

/// Strict online synchronized proof. Reuses the same immutable offline
/// coefficients as prove_prepared, with no source or coefficient copies.
/// The fused/compact/register kernels are shared with the sequential version.
pub fn prove_prepared_lockstep<C: Challenger>(
    prepared: &PreparedCoefficients,
    engine: Engine,
    c: &mut C,
) -> Result<(Proof, CommittedClaims, Stats, Timings), Error> {
    prove_prepared_impl(prepared, engine, c, Schedule::Lockstep)
}

fn prove_prepared_impl<C: Challenger>(
    prepared: &PreparedCoefficients,
    engine: Engine,
    c: &mut C,
    schedule: Schedule,
) -> Result<(Proof, CommittedClaims, Stats, Timings), Error> {
    prepared.statement.bind(c)?;
    c.observe_label(schedule.domain());
    let z = c.sample_f128();
    let mut p = Timings::default();
    let mut left = prepared.left.online(z, engine, &mut p);
    let mut right = prepared.right.online(z, engine, &mut p);
    let (proof, leaves) = prove_trees(&mut left, &mut right, z, c, engine, &mut p, schedule)?;
    let stats = Stats {
        left: left.stats,
        right: right.stats,
    };
    let claims = CommittedClaims {
        evaluations: leaf_queries(prepared.statement.width, leaves)?,
    };
    Ok((proof, claims, stats, p))
}

/// Replay the committed-oracle protocol. The returned queries are unauthenticated
/// until the outer verifier successfully checks their PCS openings.
pub fn verify_committed<C: Challenger>(
    statement: &CommittedStatement,
    proof: &Proof,
    c: &mut C,
) -> Result<CommittedClaims, Error> {
    verify_committed_impl(statement, proof, c, Schedule::Sequential)
}

/// Verify both synchronized tree equations independently. The caller must
/// authenticate both returned values, or reduce them using a verified public
/// relation such as the bit-wiring adapter before performing a PCS opening.
pub fn verify_committed_lockstep<C: Challenger>(
    statement: &CommittedStatement,
    proof: &Proof,
    c: &mut C,
) -> Result<CommittedClaims, Error> {
    verify_committed_impl(statement, proof, c, Schedule::Lockstep)
}

fn verify_committed_impl<C: Challenger>(
    statement: &CommittedStatement,
    proof: &Proof,
    c: &mut C,
    schedule: Schedule,
) -> Result<CommittedClaims, Error> {
    statement.bind(c)?;
    let leaves = verify_encoded_impl(proof, statement.rows, c, schedule)?;
    Ok(CommittedClaims {
        evaluations: leaf_queries(statement.width, leaves)?,
    })
}

fn prove_after_binding<C: Challenger>(
    left: &PackedBits,
    right: &PackedBits,
    options: Options,
    engine: Engine,
    mut p: Timings,
    c: &mut C,
    schedule: Schedule,
) -> Result<(Proof, [LeafClaim; 2], Stats, Timings), Error> {
    c.observe_label(schedule.domain());
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
        let (proof, leaves) = prove_trees(&mut a, &mut b, z, c, engine, &mut p, schedule)?;
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
        let (proof, leaves) = prove_trees(&mut a, &mut b, z, c, engine, &mut p, schedule)?;
        (
            proof,
            leaves,
            Stats {
                left: a.stats,
                right: b.stats,
            },
        )
    };
    Ok((proof, leaves, stats, p))
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
