//! Two groups of 128 native hash executions, with all public I/O authenticated.
//! Public columns are covered exactly by aligned binary intervals; private
//! columns between or after them are never treated as public zeroes.
#[path = "../e2e_plonk/fixture.rs"]
pub mod container;
#[path = "grinding.rs"]
mod grinding;

use crate::e2e_common::{TreeVerifier, batched_boolean::TransposedWitness, column_boolean};
use crate::hash_fixture::{HashKind, PublicWordBinding};
use flock_core::{
    challenger::{Challenger, FsChallenger},
    lincheck::LincheckCircuit,
    pcs::{self, Commitment, PcsParams, merged_tensor, paired},
    schedule::Registry,
    zerocheck::univariate_skip::build_eq,
};
use flock_field::{F128, F256};
use grinding::GrindingChallenger;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::BTreeMap, time::Instant};

#[derive(Clone)]
pub struct PublicBlock {
    pub first_bit: usize,
    pub log_bits: usize,
    pub values: Vec<F256>,
}
pub struct PublicIo {
    pub blocks: Vec<PublicBlock>,
    pub public_bit_count: usize,
}

/// Construct from public inputs and wiring metadata only, never the witness.
pub fn public_io(
    view: &TreeVerifier<'_>,
    columns: &[usize],
    bindings: &[PublicWordBinding],
) -> Result<PublicIo, String> {
    if view.registry.types().len() != 1 {
        return Err("public I/O expects one native hash type".into());
    }
    let mut schema_columns: Vec<_> = view.registry.types()[0]
        .io_schema
        .iter()
        .map(|w| w.word_col)
        .collect();
    schema_columns.sort_unstable();
    if columns != schema_columns {
        return Err("public columns must cover the whole native I/O schema".into());
    }
    if view.live_rows != 256 || columns.is_empty() || columns.windows(2).any(|c| c[0] >= c[1]) {
        return Err("public I/O requires sorted unique columns and 256 rows".into());
    }
    let mut addresses: BTreeMap<usize, Vec<Option<usize>>> =
        columns.iter().map(|&c| (c, vec![None; 256])).collect();
    for b in bindings {
        if b.row >= 256 || b.public_index >= view.public.len() {
            return Err("public binding out of range".into());
        }
        let entry = addresses
            .get_mut(&b.word_col)
            .ok_or("unknown public word column")?;
        if entry[b.row].replace(b.public_index).is_some() {
            return Err("duplicate public binding".into());
        }
    }
    let mut values = BTreeMap::new();
    for (&word, address) in &addresses {
        if word >= (1usize << view.r1cs.k_log) / 128 || address.iter().any(Option::is_none) {
            return Err("incomplete public binding".into());
        }
        let bits: Vec<F256> = (0..128)
            .into_par_iter()
            .map(|bit| {
                let groups: [F128; 2] = std::array::from_fn(|group| {
                    let mut value = F128::ZERO;
                    for lane in 0..128 {
                        let input = view.public[address[group * 128 + lane].unwrap()];
                        let b = if bit < 64 {
                            input.lo >> bit
                        } else {
                            input.hi >> (bit - 64)
                        } & 1;
                        if lane < 64 {
                            value.lo |= b << lane;
                        } else {
                            value.hi |= b << (lane - 64);
                        }
                    }
                    value
                });
                F256::new(groups[0], groups[1])
            })
            .collect();
        values.insert(word, bits);
    }
    let mut blocks = Vec::new();
    let mut i = 0;
    while i < columns.len() {
        let start_word = columns[i];
        let mut j = i + 1;
        while j < columns.len() && columns[j] == columns[j - 1] + 1 {
            j += 1;
        }
        let mut start = start_word * 128;
        let end = (columns[j - 1] + 1) * 128;
        while start < end {
            let remaining = (end - start).ilog2() as usize;
            let log_bits = if start == 0 {
                remaining
            } else {
                remaining.min(start.trailing_zeros() as usize)
            };
            let count = 1usize << log_bits;
            let data = (start..start + count)
                .map(|bit| values[&(bit / 128)][bit % 128])
                .collect();
            blocks.push(PublicBlock {
                first_bit: start,
                log_bits,
                values: data,
            });
            start += count;
        }
        i = j;
    }
    Ok(PublicIo {
        blocks,
        public_bit_count: columns.len() * 128,
    })
}

pub fn public_claims<C: Challenger>(
    view: &TreeVerifier<'_>,
    io: &PublicIo,
    ch: &mut C,
) -> Vec<paired::Claim> {
    ch.observe_label(b"hash-exact-public-column-cover-v1");
    ch.observe_bytes(&(io.blocks.len() as u64).to_le_bytes());
    let mut claims = Vec::with_capacity(io.blocks.len() + 1);
    for block in &io.blocks {
        ch.observe_bytes(&(block.first_bit as u64).to_le_bytes());
        ch.observe_bytes(&(block.log_bits as u64).to_le_bytes());
        let mut point = ch.sample_f128_vec(block.log_bits);
        let eq = build_eq(&point);
        let value = block
            .values
            .par_iter()
            .zip(&eq)
            .map(|(&v, &e)| v * e)
            .reduce(|| F256::ZERO, |a, b| a + b);
        point.extend((block.log_bits..view.r1cs.k_log).map(|bit| {
            if block.first_bit >> bit & 1 == 1 {
                F128::ONE
            } else {
                F128::ZERO
            }
        }));
        claims.push(paired::Claim::at_point(&point, value));
    }
    let pin = view
        .r1cs
        .const_pin
        .expect("native binary hash constant pin");
    let point: Vec<_> = (0..view.r1cs.k_log)
        .map(|bit| {
            if pin >> bit & 1 == 1 {
                F128::ONE
            } else {
                F128::ZERO
            }
        })
        .collect();
    let ones = F128::new(u64::MAX, u64::MAX);
    claims.push(paired::Claim::at_point(&point, F256::new(ones, ones)));
    claims
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Proof {
    pub commitment: Commitment,
    pub boolean: column_boolean::Proof,
    pub nonces: Vec<u64>,
    pub pairing_nonce: u64,
    pub pcs: merged_tensor::Proof,
}

fn bind(kind: HashKind, view: &TreeVerifier<'_>, cm: &Commitment) -> FsChallenger {
    let mut ch = FsChallenger::new(b"mikros-native-hash-paired-public-r1cs-v1");
    ch.observe_label(kind.as_str().as_bytes());
    ch.observe_bytes(&view.circuit.digest());
    ch.observe_bytes(&view.r1cs.statement_digest());
    ch.observe_f128_slice(view.public);
    ch.observe_bytes(&(view.live_rows as u64).to_le_bytes());
    ch.observe_bytes(&bincode::serialize(cm).unwrap());
    ch
}
fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}

pub fn prove(
    kind: HashKind,
    view: &TreeVerifier<'_>,
    matrix: &dyn LincheckCircuit,
    witness: &TransposedWitness,
    owned_z: Vec<F128>,
    io: &PublicIo,
    params: &PcsParams,
    registry: &Registry,
) -> Result<(Proof, serde_json::Value, FsChallenger), String> {
    let union = container::union(registry);
    let begin = Instant::now();
    let t = Instant::now();
    let (commitment, data) = pcs::commit(&owned_z, params);
    let commit_ms = ms(t);
    let mut ch = GrindingChallenger::prover(bind(kind, view, &commitment));
    let t = Instant::now();
    let (boolean, claims) = column_boolean::prove_with_matrix(view, matrix, witness, &mut ch)?;
    let reduction_f128_ms = ms(t);
    let t = Instant::now();
    let public = public_claims(view, io, &mut ch);
    let public_io_ms = ms(t);
    let (mut ch, nonces) = ch.finish()?;
    let t = Instant::now();
    let (pairing_nonce, mut queries) =
        paired::reduce_prover(&public, params.opening_grinding(), &mut ch)
            .map_err(|e| format!("pair reduction: {e:?}"))?;
    queries.extend(claims.other);
    let pair_reduction_ms = ms(t);
    let source_claims = queries.len();
    let t = Instant::now();
    let pcs = merged_tensor::open_identity(owned_z, &data, &commitment, &queries, &union, &mut ch)
        .map_err(|e| format!("native PCS: {e:?}"))?;
    let pcs_open_ms = ms(t);
    let ring_claims = pcs.merged.as_ref().map_or(0, |p| p.ring_switches.len());
    let timings = json!({"commit_ms":commit_ms,"reduction_f128_ms":reduction_f128_ms,
        "public_io_ms":public_io_ms,"pair_reduction_ms":pair_reduction_ms,
        "pcs_open_ms":pcs_open_ms,"prover_ms":ms(begin),"source_claims":source_claims,
        "ring_claims":ring_claims,"inner_openings":1,"public_io_blocks":io.blocks.len()});
    Ok((
        Proof {
            commitment,
            boolean,
            nonces,
            pairing_nonce,
            pcs,
        },
        timings,
        ch,
    ))
}

pub fn verify(
    kind: HashKind,
    view: &TreeVerifier<'_>,
    matrix: &dyn LincheckCircuit,
    io: &PublicIo,
    params: &PcsParams,
    registry: &Registry,
    proof: &Proof,
) -> Result<FsChallenger, String> {
    if proof.commitment.params != *params {
        return Err("PCS parameters".into());
    }
    let union = container::union(registry);
    let mut ch = GrindingChallenger::verifier(bind(kind, view, &proof.commitment), &proof.nonces);
    let claims = column_boolean::verify_with_matrix(view, matrix, &proof.boolean, &mut ch)?;
    let public = public_claims(view, io, &mut ch);
    let (mut ch, _) = ch.finish()?;
    let mut queries = paired::reduce_verify(
        &public,
        proof.pairing_nonce,
        params.opening_grinding(),
        &mut ch,
    )
    .map_err(|e| format!("pair verification: {e:?}"))?;
    queries.extend(claims.other);
    merged_tensor::verify(&proof.commitment, &queries, &proof.pcs, &union, &mut ch)
        .map_err(|e| format!("native PCS verification: {e:?}"))?;
    Ok(ch)
}

pub fn tamper(
    kind: HashKind,
    view: &TreeVerifier<'_>,
    matrix: &dyn LincheckCircuit,
    io: &PublicIo,
    params: &PcsParams,
    registry: &Registry,
    proof: &Proof,
) -> Result<Vec<String>, String> {
    let mut cases = Vec::new();
    let mut reject = |p: &Proof, name: String| -> Result<(), String> {
        if verify(kind, view, matrix, io, params, registry, p).is_ok() {
            return Err(format!("accepted Mikros mutation {name}"));
        }
        cases.push(name);
        Ok(())
    };
    for group in 0..2 {
        for field in 0..4 {
            let mut bad = proof.clone();
            let h = &mut bad.boolean.hadamards[group];
            match field {
                0 => h.quotients[0] += F128::ONE,
                1 => h.inner_product.final_a += F128::ONE,
                2 => h.inner_product.final_b += F128::ONE,
                _ => h.projected_c += F128::ONE,
            }
            reject(&bad, format!("hadamard_{group}_{field}"))?;
        }
    }
    for side in 0..2 {
        for field in 0..5 {
            let mut bad = proof.clone();
            let p = &mut bad.boolean.linchecks[side];
            match field {
                0 => p.columns[0][7] += F128::ONE,
                1 => p.ip.final_a += F128::ONE,
                2 => p.ip.final_b += F128::ONE,
                3 => p.ip.rounds[0][0] += F128::ONE,
                _ => {
                    p.columns.pop();
                }
            }
            reject(&bad, format!("image_{side}_{field}"))?;
        }
    }
    for field in 0..7 {
        let mut bad = proof.clone();
        let name = match field {
            0 => {
                bad.boolean.hadamards.swap(0, 1);
                "group_swap"
            }
            1 => {
                bad.boolean.hadamards.pop();
                "group_shape"
            }
            2 => {
                bad.boolean.linchecks.pop();
                "image_shape"
            }
            3 => {
                bad.commitment.cap[0][0] ^= 1;
                "commitment"
            }
            4 => {
                bad.nonces.pop();
                "grinding_shape"
            }
            5 => {
                bad.pcs.merged.as_mut().unwrap().q_eval += F128::ONE;
                "pcs_evaluation"
            }
            _ => {
                bad.pairing_nonce ^= 1;
                "pair_nonce"
            }
        };
        reject(&bad, name.into())?;
    }
    Ok(cases)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash_fixture::HashFixture;

    #[test]
    fn exact_public_cover_and_claims_match_native_transposed_witness() {
        for kind in [HashKind::Sha256, HashKind::Blake3, HashKind::Keccak] {
            let f = HashFixture::new(kind, 256);
            let view = f.verifier();
            let io = public_io(&view, &f.io_word_columns, &f.public_bindings).unwrap();
            let w = crate::e2e_common::batched_boolean::prepare(
                &f.prepare_witness(),
                f.nu,
                f.r1cs.k_log,
            )
            .unwrap();
            let covered: Vec<_> = io
                .blocks
                .iter()
                .flat_map(|b| b.first_bit..b.first_bit + b.values.len())
                .collect();
            let expected: Vec<_> = f
                .io_word_columns
                .iter()
                .flat_map(|w| w * 128..(w + 1) * 128)
                .collect();
            assert_eq!(covered, expected);
            for block in &io.blocks {
                assert_eq!(block.first_bit % block.values.len(), 0);
                for (i, value) in block.values.iter().enumerate() {
                    let bit = block.first_bit + i;
                    assert_eq!(*value, F256::new(w.z[bit * 2], w.z[bit * 2 + 1]));
                }
            }
            let mut ch = FsChallenger::new(b"public-cover-oracle");
            for claim in public_claims(&view, &io, &mut ch) {
                let weights = flock_core::pcs::functional::row_weights(&claim.rows);
                let result =
                    w.z.chunks_exact(2)
                        .zip(weights)
                        .fold(F256::ZERO, |sum, (v, q)| sum + F256::new(v[0], v[1]) * q);
                assert_eq!(result, claim.value);
            }
        }
    }
}
