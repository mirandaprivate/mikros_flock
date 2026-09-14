//! Permutation and indexed Lasso with WHIR authentication of every bit oracle.
//! Verifiers take public statements and proofs; they never read private tables.
//! The caller must use the expected public statement and application context.
use crate::{
    Error,
    bits::PackedBits,
    claims::{Claims, OracleSpec, Source},
    lasso,
    permutation::{self, Options},
    sumcheck::{self, LayerProof},
    whir_pcs::{Commitment, MAX_NUM_VARS, Parameters, Pcs, ProverData},
};
use flock_field::F128;
use flock_hash::HashKind;
use flock_multilinear::{IndexOrder, eq_table};
use flock_transcript::challenger::{Challenger, FsChallenger};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceProof {
    pub booleanity: LayerProof,
    pub opening: Vec<u8>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermutationStatement {
    pub piop: permutation::Statement,
    pub commitments: [Commitment; 2],
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermutationProof {
    pub piop: permutation::Proof,
    pub sources: Vec<SourceProof>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LassoStatement {
    pub piop: lasso::Statement,
    pub commitments: [Commitment; 2],
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LassoProof {
    pub piop: lasso::Proof,
    pub auxiliary_commitments: [Commitment; 3],
    pub sources: Vec<SourceProof>,
}

/// Commit before drawing any PIOP challenge. Bit order is column-major, with
/// canonical zero padding to a power of two (minimum two field elements).
fn commit(bits: &PackedBits) -> Result<(OracleSpec, Pcs, ProverData), Error> {
    let mut spec = OracleSpec::from_bits(bits)?;
    let parameters = parameters(&spec)?;
    let length = 1usize << parameters.num_vars;
    let values = (0..length)
        .map(|i| {
            if i < bits.bit_len() && bits.bit(i) {
                F128::ONE
            } else {
                F128::ZERO
            }
        })
        .collect();
    let pcs = Pcs::new(parameters)?;
    let data = pcs.commit(values)?;
    spec.digest = data.commitment().root;
    Ok((spec, pcs, data))
}
fn parameters(spec: &OracleSpec) -> Result<Parameters, Error> {
    spec.validate()?;
    let length = spec
        .rows
        .checked_mul(spec.width)
        .ok_or(Error::Length)?
        .max(2);
    if length > 1usize << MAX_NUM_VARS || spec.rows > 1usize << MAX_NUM_VARS {
        return Err(Error::Capacity);
    }
    Ok(Parameters::diagnostic(
        length.next_power_of_two().ilog2() as usize
    ))
}
fn begin(
    protocol: &[u8],
    context: &[u8],
    oracles: &[(Source, OracleSpec)],
    commitments: &[Commitment],
) -> Result<FsChallenger, Error> {
    if oracles.len() != commitments.len() {
        return Err(Error::ProofShape);
    }
    let mut c = FsChallenger::with_hash(b"mikros-lookup-whir-composition-v1", HashKind::Blake3);
    c.observe_label(protocol);
    c.observe_bytes(&(context.len() as u64).to_le_bytes());
    c.observe_bytes(context);
    c.observe_bytes(&(oracles.len() as u64).to_le_bytes());
    for ((source, spec), commitment) in oracles.iter().zip(commitments) {
        // The verifier fixes its security/profile policy; the prover cannot
        // downgrade it by replacing parameters in an otherwise valid proof.
        if commitment.parameters != parameters(spec)? || spec.digest != commitment.root {
            return Err(Error::Statement);
        }
        c.observe_bytes(&[source_tag(*source)]);
        c.observe_bytes(&(spec.rows as u64).to_le_bytes());
        c.observe_bytes(&(spec.width as u64).to_le_bytes());
        c.observe_bytes(&commitment.root);
        for value in [
            commitment.parameters.num_vars,
            commitment.parameters.security_bits,
            commitment.parameters.log_inv_rate,
            commitment.parameters.folding_factor,
        ] {
            c.observe_bytes(&(value as u64).to_le_bytes());
        }
    }
    Ok(c)
}
fn source_tag(s: Source) -> u8 {
    match s {
        Source::Left => 0,
        Source::Right => 1,
        Source::Values => 2,
        Source::Addresses => 3,
        Source::ReadCounters => 4,
        Source::Carry => 5,
        Source::FinalCounters => 6,
    }
}
fn validate_claims(claims: &Claims, oracles: &[(Source, OracleSpec)]) -> Result<(), Error> {
    if claims.oracles != oracles
        || claims
            .evaluations
            .iter()
            .any(|claim| !oracles.iter().any(|(s, _)| *s == claim.source))
    {
        return Err(Error::Source);
    }
    Ok(())
}
fn queries(
    claims: &Claims,
    source: Source,
    spec: &OracleSpec,
    r: &[F128],
    q: &[F128],
    value: F128,
) -> Result<(Vec<Vec<F128>>, Vec<F128>), Error> {
    let length = 1usize << r.len();
    let mut weights = Vec::new();
    let mut values = Vec::new();
    for claim in claims
        .evaluations
        .iter()
        .filter(|claim| claim.source == source)
    {
        weights.push(claim.query.weights(spec, length)?);
        values.push(claim.value);
    }
    // f(q), used by Booleanity's terminal f(q)*(f(q)+1) check.
    weights.push(eq_table(q, F128::ONE, IndexOrder::LowToHigh));
    values.push(value);
    let logical = spec.rows.checked_mul(spec.width).ok_or(Error::Length)?;
    if logical < length {
        // Authenticate the implicit zero tail of the original bit oracle.
        let mut tail = eq_table(r, F128::ONE, IndexOrder::LowToHigh);
        tail[..logical].fill(F128::ZERO);
        weights.push(tail);
        values.push(F128::ZERO);
    }
    Ok((weights, values))
}
fn opening_context(c: &mut FsChallenger) -> Vec<u8> {
    c.observe_label(b"source-whir-opening");
    c.sample_f128_vec(2)
        .iter()
        .flat_map(|&x| crate::field::raw(x).to_le_bytes())
        .collect()
}
fn absorb_opening(c: &mut FsChallenger, proof: &[u8]) {
    c.observe_bytes(blake3::hash(proof).as_bytes());
}
fn prove_sources(
    claims: &Claims,
    oracles: &[(Source, OracleSpec)],
    data: &[(Pcs, ProverData)],
    c: &mut FsChallenger,
) -> Result<Vec<SourceProof>, Error> {
    validate_claims(claims, oracles)?;
    if data.len() != oracles.len() {
        return Err(Error::ProofShape);
    }
    let mut proofs = Vec::new();
    for ((source, spec), (pcs, data)) in oracles.iter().zip(data) {
        if spec.digest != data.commitment().root {
            return Err(Error::Source);
        }
        c.observe_label(b"source-booleanity");
        c.observe_bytes(&[source_tag(*source)]);
        let r = c.sample_f128_vec(pcs.parameters().num_vars);
        let a = data.values().to_vec();
        if a.iter().any(|&x| x != F128::ZERO && x != F128::ONE) {
            return Err(Error::Constraint);
        }
        let b = a.iter().map(|&x| x + F128::ONE).collect();
        let (booleanity, q) = sumcheck::prove_product(&r, a, b, F128::ZERO, c);
        let (weights, values) = queries(claims, *source, spec, &r, &q, booleanity.left)?;
        let context = opening_context(c);
        let opening = pcs.open(data, &weights, &values, &context)?;
        absorb_opening(c, &opening);
        proofs.push(SourceProof {
            booleanity,
            opening,
        });
    }
    Ok(proofs)
}
fn verify_sources(
    claims: &Claims,
    oracles: &[(Source, OracleSpec)],
    commitments: &[Commitment],
    proofs: &[SourceProof],
    c: &mut FsChallenger,
) -> Result<(), Error> {
    validate_claims(claims, oracles)?;
    if proofs.len() != oracles.len() || commitments.len() != oracles.len() {
        return Err(Error::ProofShape);
    }
    for (((source, spec), commitment), proof) in oracles.iter().zip(commitments).zip(proofs) {
        let pcs = Pcs::new(parameters(spec)?)?;
        if spec.digest != commitment.root || commitment.parameters != *pcs.parameters() {
            return Err(Error::Statement);
        }
        c.observe_label(b"source-booleanity");
        c.observe_bytes(&[source_tag(*source)]);
        let r = c.sample_f128_vec(pcs.parameters().num_vars);
        let q = sumcheck::verify_product(&proof.booleanity, &r, F128::ZERO, c)?;
        if proof.booleanity.left + proof.booleanity.right != F128::ONE {
            return Err(Error::Constraint);
        }
        let (weights, values) = queries(claims, *source, spec, &r, &q, proof.booleanity.left)?;
        let context = opening_context(c);
        pcs.verify(commitment, &weights, &values, &proof.opening, &context)?;
        absorb_opening(c, &proof.opening);
    }
    Ok(())
}

pub fn prove_permutation(
    left: &PackedBits,
    right: &PackedBits,
    options: Options,
    context: &[u8],
) -> Result<(PermutationStatement, PermutationProof, permutation::Stats), Error> {
    // Validate the relation's public dimensions before doing commitment work.
    permutation::Statement::new(left, right)?;
    let (left_spec, left_pcs, left_data) = commit(left)?;
    let (right_spec, right_pcs, right_data) = commit(right)?;
    let statement = PermutationStatement {
        piop: permutation::Statement {
            left: left_spec,
            right: right_spec,
        },
        commitments: [
            left_data.commitment().clone(),
            right_data.commitment().clone(),
        ],
    };
    let oracles = vec![
        (Source::Left, statement.piop.left.clone()),
        (Source::Right, statement.piop.right.clone()),
    ];
    let mut c = begin(b"permutation", context, &oracles, &statement.commitments)?;
    let (piop, claims, stats) =
        permutation::prove_prebound(&statement.piop, left, right, options, &mut c)?;
    let sources = prove_sources(
        &claims,
        &oracles,
        &[(left_pcs, left_data), (right_pcs, right_data)],
        &mut c,
    )?;
    Ok((statement, PermutationProof { piop, sources }, stats))
}
pub fn verify_permutation(
    statement: &PermutationStatement,
    proof: &PermutationProof,
    context: &[u8],
) -> Result<(), Error> {
    let oracles = vec![
        (Source::Left, statement.piop.left.clone()),
        (Source::Right, statement.piop.right.clone()),
    ];
    let mut c = begin(b"permutation", context, &oracles, &statement.commitments)?;
    let claims = permutation::verify(&statement.piop, &proof.piop, &mut c)?;
    verify_sources(
        &claims,
        &oracles,
        &statement.commitments,
        &proof.sources,
        &mut c,
    )
}
pub fn prove_lasso(
    table: PackedBits,
    values: &PackedBits,
    addresses: &PackedBits,
    options: Options,
    context: &[u8],
) -> Result<(LassoStatement, LassoProof, lasso::Stats), Error> {
    let raw = lasso::Statement::new(table, values, addresses)?;
    // Cap all shapes before allocating auxiliary counters, including width=0.
    for (_, spec) in lasso::oracles(&raw, &[[0; 32]; 3]) {
        parameters(&spec)?;
    }
    let auxiliary = lasso::Auxiliary::build(&raw, values, addresses)?;
    let mut data = Vec::new();
    let mut specs = Vec::new();
    let mut commitments = Vec::new();
    for (source, bits) in auxiliary.sources(values, addresses) {
        let (spec, pcs, witness) = commit(bits)?;
        commitments.push(witness.commitment().clone());
        specs.push((source, spec));
        data.push((pcs, witness));
    }
    let statement = LassoStatement {
        piop: lasso::Statement {
            table: raw.table,
            values: specs[0].1.clone(),
            addresses: specs[1].1.clone(),
        },
        commitments: [commitments[0].clone(), commitments[1].clone()],
    };
    let mut c = begin(b"lasso", context, &specs, &commitments)?;
    let roots = [
        commitments[2].root,
        commitments[3].root,
        commitments[4].root,
    ];
    let output = lasso::prove_prebound(
        &statement.piop,
        values,
        addresses,
        auxiliary,
        roots,
        options,
        &mut c,
    )?;
    let sources = prove_sources(&output.claims, &specs, &data, &mut c)?;
    let proof = LassoProof {
        piop: output.proof,
        auxiliary_commitments: [
            commitments[2].clone(),
            commitments[3].clone(),
            commitments[4].clone(),
        ],
        sources,
    };
    Ok((statement, proof, output.stats))
}
pub fn verify_lasso(
    statement: &LassoStatement,
    proof: &LassoProof,
    context: &[u8],
) -> Result<(), Error> {
    statement.piop.validate()?;
    let mut commitments = statement.commitments.to_vec();
    commitments.extend_from_slice(&proof.auxiliary_commitments);
    let roots = proof.auxiliary_commitments.clone().map(|c| c.root);
    if roots != proof.piop.auxiliary_digests {
        return Err(Error::Statement);
    }
    let oracles = lasso::oracles(&statement.piop, &roots);
    let mut c = begin(b"lasso", context, &oracles, &commitments)?;
    let claims = lasso::verify(&statement.piop, &proof.piop, &mut c)?;
    verify_sources(&claims, &oracles, &commitments, &proof.sources, &mut c)
}

#[cfg(test)]
mod tests {
    use super::*;
    /// A malicious prover can commit to arbitrary E entries. Supply a valid
    /// WHIR proof of their actual evaluations, but Booleanity/zero-tail claims
    /// from the alleged bit oracle. Verification must reject the discrepancy.
    #[test]
    fn verifier_rejects_nonboolean_sources_and_nonzero_padding() {
        for padding_attack in [false, true] {
            let mut spec = OracleSpec {
                rows: 3,
                width: 1,
                digest: [0; 32],
            };
            let pcs = Pcs::new(parameters(&spec).unwrap()).unwrap();
            let vector = if padding_attack {
                vec![F128::ZERO, F128::ONE, F128::ZERO, F128::ONE]
            } else {
                vec![
                    crate::field::from_raw(7),
                    F128::ZERO,
                    F128::ZERO,
                    F128::ZERO,
                ]
            };
            let data = pcs.commit(vector.clone()).unwrap();
            spec.digest = data.commitment().root;
            let oracles = vec![(Source::Left, spec.clone())];
            let commitments = vec![data.commitment().clone()];
            let claims = Claims {
                oracles: oracles.clone(),
                evaluations: vec![],
            };
            let mut c = begin(b"malicious-source", b"test", &oracles, &commitments).unwrap();
            c.observe_label(b"source-booleanity");
            c.observe_bytes(&[source_tag(Source::Left)]);
            let r = c.sample_f128_vec(2);
            let alleged = if padding_attack {
                vector.clone()
            } else {
                vec![F128::ZERO; 4]
            };
            let b = alleged.iter().map(|&v| v + F128::ONE).collect();
            let (booleanity, q) = sumcheck::prove_product(&r, alleged, b, F128::ZERO, &mut c);
            let (weights, claimed) =
                queries(&claims, Source::Left, &spec, &r, &q, booleanity.left).unwrap();
            let actual = weights
                .iter()
                .map(|w| {
                    w.iter()
                        .zip(&vector)
                        .fold(F128::ZERO, |s, (&w, &v)| s + w * v)
                })
                .collect::<Vec<_>>();
            assert_ne!(actual, claimed);
            let context = opening_context(&mut c);
            let opening = pcs.open(&data, &weights, &actual, &context).unwrap();
            // Demonstrate that the supplied WHIR opening is individually valid.
            pcs.verify(&commitments[0], &weights, &actual, &opening, &context)
                .unwrap();
            let proof = SourceProof {
                booleanity,
                opening,
            };
            let mut verifier = begin(b"malicious-source", b"test", &oracles, &commitments).unwrap();
            assert!(
                verify_sources(&claims, &oracles, &commitments, &[proof], &mut verifier).is_err()
            );
        }
    }
}
