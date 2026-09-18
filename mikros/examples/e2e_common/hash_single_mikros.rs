//! One live native hash instance, packing 128 gate bits in each F128 word.
//! The remaining BatchMajor instance slots are zero padding required by the
//! existing PCS geometry, not additional hash executions. Every native I/O
//! word is public, so exact word openings discharge all circuit wiring classes.
#[path = "grinding.rs"]
mod grinding;

use crate::e2e_common::{PreparedWitness, TreeVerifier};
use crate::hash_fixture::{HashKind, PublicWordBinding};
use flock_core::{
    challenger::{Challenger, FsChallenger},
    circuit::CellSlot,
    lincheck::LincheckCircuit,
    pcs::{
        self, Commitment, PcsParams,
        functional::TensorClaim,
        ligerito::{LigeritoProfile, embedded_initial_k_or_default},
        merged_tensor,
    },
    zerocheck::univariate_skip::build_eq,
};
use flock_field::F128;
use grinding::GrindingChallenger;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::BTreeMap, time::Instant};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicBlock {
    pub first_word: usize,
    pub log_words: usize,
    pub values: Vec<F128>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicIo {
    pub blocks: Vec<PublicBlock>,
}

/// Derive gate-word values from verifier-known wiring and public data only.
/// This path deliberately requires every live gate cell to belong to a class
/// containing a public cell; it cannot silently omit a private copy constraint.
fn public_words(view: &TreeVerifier<'_>) -> Result<BTreeMap<usize, F128>, String> {
    if view.live_rows != 1
        || view.registry.types().len() != 1
        || view.nu != view.circuit.cells().nu()
        || !view.circuit.check_public(view.public)
        || !view.circuit.check_instance(&view.union())
    {
        return Err("single hash public circuit shape".into());
    }
    let slots = view.circuit.cells().slots();
    let mask = (1usize << view.nu) - 1;
    let mut values = BTreeMap::new();
    for class in view.circuit.wires() {
        let mut public_value = None;
        let mut gate_words = Vec::new();
        for &cell in class {
            let row = cell & mask;
            match slots[cell >> view.nu] {
                CellSlot::Public { s } => {
                    let value = *view
                        .public
                        .get((s << view.nu) + row)
                        .ok_or("public wiring address out of range")?;
                    if public_value.is_some_and(|old| old != value) {
                        return Err("unequal public words in one wiring class".into());
                    }
                    public_value = Some(value);
                }
                CellSlot::Gate { ty: 0, word } if row == 0 => gate_words.push(word.word_col),
                _ => {
                    return Err(
                        "single hash wiring contains a non-live gate or padding cell".into(),
                    );
                }
            }
        }
        if !gate_words.is_empty() {
            let value = public_value.ok_or("single hash has a private wiring class")?;
            for word in gate_words {
                if values.insert(word, value).is_some() {
                    return Err("duplicate gate word in public wiring".into());
                }
            }
        }
    }
    let mut columns: Vec<_> = view.registry.types()[0]
        .io_schema
        .iter()
        .map(|w| w.word_col)
        .collect();
    columns.sort_unstable();
    if columns.is_empty()
        || columns.windows(2).any(|w| w[0] == w[1])
        || values.keys().copied().collect::<Vec<_>>() != columns
    {
        return Err("every native single-instance I/O word must be publicly wired".into());
    }
    Ok(values)
}

fn blocks(values: &BTreeMap<usize, F128>) -> PublicIo {
    let columns: Vec<_> = values.keys().copied().collect();
    let mut blocks = Vec::new();
    let mut i = 0;
    while i < columns.len() {
        let mut j = i + 1;
        while j < columns.len() && columns[j] == columns[j - 1] + 1 {
            j += 1;
        }
        let mut start = columns[i];
        let end = columns[j - 1] + 1;
        while start < end {
            let remaining = (end - start).ilog2() as usize;
            let log_words = if start == 0 {
                remaining
            } else {
                remaining.min(start.trailing_zeros() as usize)
            };
            let len = 1usize << log_words;
            blocks.push(PublicBlock {
                first_word: start,
                log_words,
                values: (start..start + len).map(|word| values[&word]).collect(),
            });
            start += len;
        }
        i = j;
    }
    PublicIo { blocks }
}

pub fn public_io(
    view: &TreeVerifier<'_>,
    columns: &[usize],
    bindings: &[PublicWordBinding],
) -> Result<PublicIo, String> {
    let values = public_words(view)?;
    if columns != values.keys().copied().collect::<Vec<_>>() || bindings.len() != values.len() {
        return Err("single public I/O must cover the complete sorted native schema".into());
    }
    let mut seen = BTreeMap::new();
    for binding in bindings {
        if binding.row != 0
            || binding.public_index >= view.public.len()
            || values.get(&binding.word_col) != Some(&view.public[binding.public_index])
            || seen.insert(binding.word_col, ()).is_some()
        {
            return Err("single public I/O binding differs from public circuit wiring".into());
        }
    }
    Ok(blocks(&values))
}

fn shape(
    view: &TreeVerifier<'_>,
    matrix: &dyn LincheckCircuit,
    io: &PublicIo,
    params: &PcsParams,
) -> Result<(), String> {
    if io != &blocks(&public_words(view)?)
        || !view.r1cs.c0_is_identity()
        || view.r1cs.m != view.r1cs.k_log + view.nu
        || matrix.n_cols() != 1usize << view.r1cs.k_log
        || matrix.const_pin_col() != view.r1cs.const_pin
    {
        return Err("single native R1CS, public I/O or matrix shape".into());
    }
    let union = view.union();
    let profile = LigeritoProfile::Fast;
    let k = embedded_initial_k_or_default(union.dense_m(), profile);
    let expected = PcsParams {
        m: union.dense_m(),
        log_inv_rate: profile.log_inv_rate(),
        log_batch_size: k,
        profile,
        num_lanes: union.commit_lanes(k),
        merkle_hash: Default::default(),
    };
    if *params != expected {
        return Err("single hash native PCS parameters differ".into());
    }
    Ok(())
}

fn canonical_columns() -> Vec<F128> {
    (0..128)
        .map(|i| {
            if i < 64 {
                F128::new(1u64 << i, 0)
            } else {
                F128::new(0, 1u64 << (i - 64))
            }
        })
        .collect()
}

pub fn public_claims<C: Challenger>(
    view: &TreeVerifier<'_>,
    io: &PublicIo,
    ch: &mut C,
) -> Vec<TensorClaim> {
    ch.observe_label(b"mikros-single-hash-public-word-cover-v1");
    ch.observe_bytes(&(io.blocks.len() as u64).to_le_bytes());
    io.blocks
        .iter()
        .map(|block| {
            ch.observe_bytes(&(block.first_word as u64).to_le_bytes());
            ch.observe_bytes(&(block.log_words as u64).to_le_bytes());
            ch.observe_f128_slice(&block.values);
            let point = ch.sample_f128_vec(block.log_words);
            let eq = build_eq(&point);
            let value = block
                .values
                .iter()
                .zip(eq)
                .fold(F128::ZERO, |s, (&v, e)| s + v * e);
            // Natural BatchMajor word address: (gate_word << nu) + instance.
            let mut rows = vec![[F128::ONE, F128::ZERO]; view.nu];
            rows.extend(point.iter().map(|&x| [F128::ONE + x, x]));
            rows.extend((block.log_words..view.r1cs.k_log - 7).map(|bit| {
                if block.first_word >> bit & 1 == 0 {
                    [F128::ONE, F128::ZERO]
                } else {
                    [F128::ZERO, F128::ONE]
                }
            }));
            TensorClaim {
                columns: canonical_columns(),
                rows,
                value,
            }
        })
        .collect()
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Proof {
    pub commitment: Commitment,
    pub hadamard: mikros::hadamard::Proof,
    pub lincheck: mikros::e2e_lincheck::Proof,
    pub nonces: Vec<u64>,
    pub pcs: merged_tensor::Proof,
}

fn bind(kind: HashKind, view: &TreeVerifier<'_>, cm: &Commitment) -> FsChallenger {
    let mut ch = FsChallenger::new(b"mikros-single-native-hash-gate-packed-v1");
    ch.observe_label(kind.as_str().as_bytes());
    ch.observe_bytes(&view.r1cs.statement_digest());
    view.union()
        .bind_statement_circuit(&mut ch, cm, &view.circuit.digest(), view.public);
    ch
}
fn matrix_query(claim: mikros::MatrixClaim) -> TensorClaim {
    TensorClaim {
        columns: claim.column_weights,
        rows: claim.row_factors.into_iter().rev().collect(),
        value: claim.value,
    }
}
fn bytes(words: &[F128]) -> &[u8] {
    assert!(cfg!(target_endian = "little"));
    // Same repr(C), lo-then-hi packed-bit representation as the native prover.
    unsafe { std::slice::from_raw_parts(words.as_ptr().cast(), std::mem::size_of_val(words)) }
}
fn ms(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

/// `owned_z` is union.compact_witness(&prepared.0), prepared by the caller
/// inside its online timer; `prepared` remains the full virtual BatchMajor Z/A/B.
pub fn prove(
    kind: HashKind,
    view: &TreeVerifier<'_>,
    matrix: &dyn LincheckCircuit,
    prepared: &PreparedWitness,
    owned_z: Vec<F128>,
    io: &PublicIo,
    params: &PcsParams,
) -> Result<(Proof, serde_json::Value, FsChallenger), String> {
    let begin = Instant::now();
    shape(view, matrix, io, params)?;
    let union = view.union();
    if [&prepared.0, &prepared.1, &prepared.2]
        .iter()
        .any(|w| w.len() != union.packed_len())
        || owned_z.len() != union.committed_words()
    {
        return Err("single native witness shape".into());
    }
    let t = Instant::now();
    let (commitment, data) = if params.num_lanes.is_some() {
        pcs::commit_lane_major(&owned_z, params)
    } else {
        pcs::commit(&owned_z, params)
    };
    let commit_ms = ms(t);
    let mut ch = GrindingChallenger::prover(bind(kind, view, &commitment));
    let t = Instant::now();
    let (hadamard, terminal) = mikros::prove(
        bytes(&prepared.1),
        bytes(&prepared.2),
        bytes(&prepared.0),
        view.r1cs.m,
        mikros::Kernel::PairAggregate,
        &mut ch,
    )
    .map_err(|e| format!("single Hadamard: {e:?}"))?;
    let hadamard_ms = ms(t);
    let t = Instant::now();
    let (lincheck, mut claims) = mikros::e2e_lincheck::prove(
        matrix,
        &prepared.0,
        view.nu,
        1,
        &terminal.a,
        &terminal.b,
        &mut ch,
    )
    .map_err(|e| format!("single lincheck: {e:?}"))?;
    claims.push(matrix_query(terminal.c));
    let lincheck_ms = ms(t);
    let t = Instant::now();
    claims.extend(public_claims(view, io, &mut ch));
    let public_io_ms = ms(t);
    let (mut ch, nonces) = ch.finish()?;
    let source_claims = claims.len();
    let t = Instant::now();
    let pcs = merged_tensor::open(
        owned_z,
        &prepared.0,
        &data,
        &commitment,
        &claims,
        &union,
        &mut ch,
    )
    .map_err(|e| format!("single native PCS: {e:?}"))?;
    let pcs_open_ms = ms(t);
    let ring_claims = pcs.merged.as_ref().map_or(0, |p| p.ring_switches.len());
    let timings = json!({"commit_ms":commit_ms,"reduction_f128_ms":hadamard_ms+lincheck_ms,
        "hadamard_ms":hadamard_ms,"lincheck_ms":lincheck_ms,"public_io_ms":public_io_ms,
        "pair_reduction_ms":0.0,"pcs_open_ms":pcs_open_ms,"prover_ms":ms(begin),
        "source_claims":source_claims,"ring_claims":ring_claims,"inner_openings":usize::from(pcs.merged.is_some()),
        "public_io_blocks":io.blocks.len()});
    Ok((
        Proof {
            commitment,
            hadamard,
            lincheck,
            nonces,
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
    proof: &Proof,
) -> Result<FsChallenger, String> {
    shape(view, matrix, io, params)?;
    if proof.commitment.params != *params {
        return Err("single commitment parameters".into());
    }
    let mut ch = GrindingChallenger::verifier(bind(kind, view, &proof.commitment), &proof.nonces);
    let terminal = mikros::verify(&proof.hadamard, view.r1cs.m, &mut ch)
        .map_err(|e| format!("single Hadamard verification: {e:?}"))?;
    let mut claims = mikros::e2e_lincheck::verify(
        matrix,
        view.nu,
        1,
        &terminal.a,
        &terminal.b,
        &proof.lincheck,
        &mut ch,
    )
    .map_err(|e| format!("single lincheck verification: {e:?}"))?;
    claims.push(matrix_query(terminal.c));
    claims.extend(public_claims(view, io, &mut ch));
    let (mut ch, _) = ch.finish()?;
    merged_tensor::verify(
        &proof.commitment,
        &claims,
        &proof.pcs,
        &view.union(),
        &mut ch,
    )
    .map_err(|e| format!("single native PCS verification: {e:?}"))?;
    Ok(ch)
}

pub fn tamper(
    kind: HashKind,
    view: &TreeVerifier<'_>,
    matrix: &dyn LincheckCircuit,
    io: &PublicIo,
    params: &PcsParams,
    proof: &Proof,
) -> Result<Vec<String>, String> {
    let mut cases = Vec::new();
    let mut reject = |bad: &Proof, name: String| -> Result<(), String> {
        if verify(kind, view, matrix, io, params, bad).is_ok() {
            return Err(format!("accepted single mutation {name}"));
        }
        cases.push(name);
        Ok(())
    };
    for field in 0..8 {
        let mut bad = proof.clone();
        match field {
            0 => bad.hadamard.projected_c += F128::ONE,
            1 => bad.hadamard.column_claims[0] += F128::ONE,
            2 => bad.hadamard.quotients[0] += F128::ONE,
            3 => bad.hadamard.inner_product.rounds[0][0] += F128::ONE,
            4 => bad.hadamard.inner_product.final_a += F128::ONE,
            5 => bad.hadamard.inner_product.final_b += F128::ONE,
            6 => {
                bad.hadamard.column_claims.pop();
            }
            _ => {
                bad.hadamard.quotients.pop();
            }
        }
        reject(&bad, format!("single_hadamard_{field}"))?;
    }
    for side in 0..2 {
        for field in 0..6 {
            let mut bad = proof.clone();
            let p = if side == 0 {
                &mut bad.lincheck.a
            } else {
                &mut bad.lincheck.b
            };
            match field {
                0 => p.h_plus[0] += F128::ONE,
                1 => p.h_minus[0] += F128::ONE,
                2 => p.inner_product.rounds[0][0] += F128::ONE,
                3 => p.inner_product.final_a += F128::ONE,
                4 => p.inner_product.final_b += F128::ONE,
                _ => {
                    p.h_plus.pop();
                }
            }
            reject(&bad, format!("single_lincheck_{side}_{field}"))?;
        }
    }
    for field in 0..5 {
        let mut bad = proof.clone();
        let name = match field {
            0 => {
                bad.commitment.cap[0][0] ^= 1;
                "single_commitment"
            }
            1 => {
                bad.nonces.pop();
                "single_grinding_shape"
            }
            2 => {
                bad.pcs.merged.as_mut().ok_or("missing PCS proof")?.q_eval += F128::ONE;
                "single_pcs_evaluation"
            }
            3 => {
                bad.pcs.component_values[0] += F128::ONE;
                "single_pcs_component"
            }
            _ => {
                bad.pcs.merged = None;
                "single_pcs_missing"
            }
        };
        reject(&bad, name.into())?;
    }
    for remove in [false, true] {
        let mut bad = io.clone();
        if remove {
            bad.blocks.pop();
        } else {
            bad.blocks[0].values[0] += F128::ONE;
        }
        if verify(kind, view, matrix, &bad, params, proof).is_ok() {
            return Err("accepted single public I/O mutation".into());
        }
        cases.push(
            if remove {
                "single_public_coverage"
            } else {
                "single_public_value"
            }
            .into(),
        );
    }
    Ok(cases)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash_fixture::HashFixture;
    #[test]
    fn single_public_cover_matches_native_gate_packed_witness() {
        for kind in [HashKind::Sha256, HashKind::Keccak, HashKind::Blake3] {
            let f = HashFixture::new(kind, 1);
            let view = f.verifier();
            let prepared = f.prepare_witness();
            let io = public_io(&view, &f.io_word_columns, &f.public_bindings).unwrap();
            let covered: Vec<_> = io
                .blocks
                .iter()
                .flat_map(|b| b.first_word..b.first_word + b.values.len())
                .collect();
            assert_eq!(covered, f.io_word_columns);
            for block in &io.blocks {
                assert_eq!(block.values.len(), 1usize << block.log_words);
                assert_eq!(block.first_word % block.values.len(), 0);
                for (i, &value) in block.values.iter().enumerate() {
                    assert_eq!(value, prepared.0[(block.first_word + i) << view.nu]);
                }
            }
            // Every non-live instance slot is zero, including its constant pin.
            for words in [&prepared.0, &prepared.1, &prepared.2] {
                assert!(
                    words
                        .chunks_exact(1usize << view.nu)
                        .all(|w| w[1..].iter().all(|x| x.is_zero()))
                );
            }
            let mut ch = FsChallenger::new(b"single-public-cover-oracle");
            for claim in public_claims(&view, &io, &mut ch) {
                assert_eq!(claim.rows.len(), view.r1cs.m - 7);
                let rows = flock_core::pcs::functional::row_weights(&claim.rows);
                let value = prepared
                    .0
                    .iter()
                    .zip(rows)
                    .fold(F128::ZERO, |s, (&word, w)| s + word * w);
                assert_eq!(value, claim.value);
            }
            let mut omitted = f.public_bindings.clone();
            omitted.pop();
            assert!(public_io(&view, &f.io_word_columns, &omitted).is_err());
        }
    }
}
