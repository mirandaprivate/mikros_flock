//! The active multiple-instance R1CS construction in latex/6-together.tex.
//! All three virtual matrix images close through committed sparse lookups.
#![allow(dead_code)]
#[path = "../e2e_plonk/fixture.rs"]
mod container;
mod grinding;

use crate::e2e_common::{TreeFixture, TreeVerifier};
use flock_core::{
    challenger::{Challenger, FsChallenger},
    pcs::{self, Commitment, PcsParams, functional::TensorClaim, merged_tensor, paired},
    r1cs::SparseBinaryMatrix,
    zerocheck::univariate_skip::build_eq,
};
use flock_field::{F128, F256, F256Unreduced};
use flock_prover::r1cs_hashes::sha2::{
    build_sparse_block_r1cs, build_sparse_block_witness,
    build_sparse_compact_block_r1cs, build_sparse_compact_block_witness,
};
use grinding::GrindingChallenger;
use mikros::{multiple_image as image, packed_sources as packed, sparse_lookup as sparse};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    env,
    io::{self, Write},
    time::Instant,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Opening {
    Paired,
    Separate,
}
impl Opening {
    fn name(self) -> &'static str {
        match self {
            Self::Paired => "paired",
            Self::Separate => "separate",
        }
    }
}

/// Only preprocessed commitments, dimensions and public I/O. No sparse matrix,
/// addresses, timestamps, witness or online gather vectors reach verify().
#[derive(Clone)]
struct VerificationKey {
    log_n: usize,
    pin: usize,
    public_io: Vec<F256>,
    circuit_digest: Vec<u8>,
    fixture_digest: String,
    static_layout: packed::Layout,
    read_layout: packed::Layout,
    static_commitment: Commitment,
    matrices: Vec<sparse::Statement>,
    witness_params: PcsParams,
}

struct Prepared {
    vk: VerificationKey,
    matrices: [SparseBinaryMatrix; 3],
    sparse: Vec<sparse::Prepared>,
    static_words: Vec<F128>,
    static_data: pcs::ProverData,
    static_column_value_bits: Vec<usize>,
    /// Interleaved F128 coordinates of logical F256 words.
    witness: Vec<F128>,
    groups: Vec<Vec<F128>>,
    images: Vec<Vec<F128>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Proof {
    opening: Opening,
    commitment: Commitment,
    read_commitment: Commitment,
    hadamard: Vec<mikros::Proof>,
    images: Vec<image::BatchProof>,
    lookups: Vec<sparse::Proof>,
    /// Both coordinates at each of the three combined IP witness terminals.
    witness_values: Vec<F256>,
    nonces: Vec<u64>,
    pairing_nonce: Option<u64>,
    witness_pcs: merged_tensor::Proof,
    static_pcs: merged_tensor::Proof,
    read_pcs: merged_tensor::Proof,
}

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}
fn fail(e: impl std::fmt::Debug) -> String {
    format!("{e:?}")
}
fn raw(x: u64) -> F128 {
    F128::new(x, 0)
}
fn bytes(words: &[F128]) -> &[u8] {
    assert!(cfg!(target_endian = "little"));
    // F128's established repr(C), lo then hi, is the binary oracle layout.
    unsafe { std::slice::from_raw_parts(words.as_ptr().cast(), std::mem::size_of_val(words)) }
}
fn evaluate(values: &[F128], point: &[F128]) -> F128 {
    let weights = build_eq(point);
    assert_eq!(values.len(), weights.len());
    values
        .par_iter()
        .zip(weights)
        .map(|(&v, w)| v * w)
        .reduce(|| F128::ZERO, |a, b| a + b)
}

/// Reuse each equality weight and defer field reduction for both coordinates.
fn evaluate_pair(groups: &[Vec<F128>], point: &[F128]) -> F256 {
    assert_eq!(groups.len(), 2);
    let weights = build_eq(point);
    assert_eq!(groups[0].len(), weights.len());
    assert_eq!(groups[1].len(), weights.len());
    let sum = weights
        .par_chunks(2048)
        .zip(groups[0].par_chunks(2048))
        .zip(groups[1].par_chunks(2048))
        .map(|((w, a), b)| {
            let mut sums = [F256Unreduced::ZERO; 2];
            for ((&w, &a), &b) in w.iter().zip(a).zip(b) {
                sums[0] ^= a.mul_unreduced(w);
                sums[1] ^= b.mul_unreduced(w);
            }
            sums
        })
        .reduce(
            || [F256Unreduced::ZERO; 2],
            |a, b| [a[0] ^ b[0], a[1] ^ b[1]],
        );
    F256::new(sum[0].reduce(), sum[1].reduce())
}
fn image_words(matrix: &SparseBinaryMatrix, witness: &[F128]) -> Vec<F128> {
    matrix
        .rows
        .par_iter()
        .map(|row| row.iter().fold(F128::ZERO, |v, &j| v + witness[j]))
        .collect()
}

fn public_io(vk: &TreeVerifier<'_>) -> Vec<F256> {
    assert_eq!(vk.live_rows, 256);
    assert_eq!(vk.public.len(), 1538);
    (0..1024)
        .into_par_iter()
        .map(|c| {
            let (word, bit) = (c / 128, c % 128);
            let values: [F128; 2] = std::array::from_fn(|group| {
                let mut bits = 0u128;
                for lane in 0..128 {
                    let instance = group * 128 + lane;
                    let address = match word {
                        0 | 1 => word,
                        2 | 3 => 2 + 4 * 256 + 2 * instance + word - 2,
                        _ => 2 + 4 * instance + word - 4,
                    };
                    let v = vk.public[address];
                    bits |= (((u128::from(v.hi) << 64) | u128::from(v.lo)) >> bit & 1) << lane;
                }
                F128::new(bits as u64, (bits >> 64) as u64)
            });
            F256::new(values[0], values[1])
        })
        .collect()
}

fn static_columns_for(
    prepared: &[sparse::Prepared],
    narrow: bool,
) -> Result<(Vec<packed::Column>, Vec<usize>), String> {
    let mut columns = Vec::new();
    let mut value_bits = Vec::new();
    for p in prepared {
        let statement = p.statement([0; 32]);
        for id in sparse::OracleId::STATIC {
            let values = p.static_column(id).ok_or("missing static column")?;
            let max_value = values.iter().copied().max().unwrap_or(0);
            let bits = (64 - max_value.leading_zeros() as usize).max(1);
            let width = if narrow { bits } else { statement.column_bits(id) }
                .max(1)
                .next_power_of_two();
            columns.push(packed::Column {
                width,
                values: values.iter().map(|&x| raw(x)).collect(),
            });
            value_bits.push(bits);
        }
    }
    Ok((columns, value_bits))
}

fn prepare(f: &TreeFixture) -> Result<Prepared, String> {
    // The compact encoding stores actual carry bits and authenticates the
    // corresponding non-identity C matrix through the same complete lookup.
    let compact = env::var_os("MIKROS_SHA_LEGACY_CARRIES").is_none();
    let r1cs = if compact {
        build_sparse_compact_block_r1cs(f.nu)
    } else {
        build_sparse_block_r1cs(f.nu)
    };
    let bits: Vec<_> = f
        .compressions
        .par_iter()
        .map(|(h, m)| {
            if compact {
                build_sparse_compact_block_witness(h, m)
            } else {
                build_sparse_block_witness(h, m)
            }
        })
        .collect();
    let groups: Vec<Vec<_>> = (0..2)
        .map(|g| {
            (0..1usize << r1cs.k_log)
                .into_par_iter()
                .map(|position| {
                    let mut word = 0u128;
                    for lane in 0..128 {
                        word |= u128::from(bits[128 * g + lane][position]) << lane;
                    }
                    F128::new(word as u64, (word >> 64) as u64)
                })
                .collect()
        })
        .collect();
    let witness: Vec<_> = (0..groups[0].len())
        .flat_map(|i| [groups[0][i], groups[1][i]])
        .collect();
    let matrices = [&r1cs.a_0, &r1cs.b_0, &r1cs.c_0];
    let sparse: Vec<_> = matrices
        .iter()
        .map(|s| {
            if env::var_os("MIKROS_SPARSE_ZERO_PADDING_ADDRESS").is_some() {
                sparse::Prepared::new(s)
            } else {
                sparse::Prepared::new_balanced_padding(s)
            }
            .map_err(fail)
        })
        .collect::<Result<_, _>>()?;
    let (static_columns, static_column_value_bits) = static_columns_for(
        &sparse,
        env::var_os("MIKROS_SPARSE_WIDE_METADATA").is_none(),
    )?;
    let static_specs: Vec<_> = static_columns
        .iter()
        .map(|c| (c.values.len(), c.width))
        .collect();
    let static_layout = packed::Layout::new(&static_specs)?;
    let static_words = static_layout.pack(&static_columns)?;
    let (static_commitment, static_data) = pcs::commit(&static_words, &static_layout.parameters());
    let digest = *blake3::hash(&bincode::serialize(&static_commitment).map_err(fail)?).as_bytes();
    let statements: Vec<_> = sparse.iter().map(|s| s.statement(digest)).collect();
    let read_specs: Vec<_> = (0..3)
        .flat_map(|i| [(1usize << statements[i % 3].entry_log, 128); 2])
        .collect();
    let read_layout = packed::Layout::new(&read_specs)?;
    let images: Vec<_> = (0..6)
        .map(|i| image_words(matrices[i % 3], &groups[i / 3]))
        .collect();
    let registry = container::registry(f.r1cs.k_log + 1, f.r1cs.k_log + 1);
    let mut witness_params = container::parameters(&container::union(&registry));
    witness_params.num_lanes = None;
    let vk = VerificationKey {
        log_n: f.r1cs.k_log,
        pin: f.r1cs.const_pin.ok_or("SHA pin")?,
        public_io: public_io(&f.verifier()),
        circuit_digest: r1cs.statement_digest().to_vec(),
        fixture_digest: blake3::hash(
            &bincode::serialize(&(&f.compressions, &f.public)).map_err(fail)?,
        )
        .to_hex()
        .to_string(),
        static_layout,
        read_layout,
        static_commitment,
        matrices: statements,
        witness_params,
    };
    Ok(Prepared {
        vk,
        matrices: [r1cs.a_0, r1cs.b_0, r1cs.c_0],
        sparse,
        static_words,
        static_data,
        static_column_value_bits,
        witness,
        groups,
        images,
    })
}

fn bind(vk: &VerificationKey, commitment: &Commitment) -> Result<FsChallenger, String> {
    let mut ch =
        FsChallenger::new(b"mikros-256-sha-r1cs-shared-rows-batched-images-spark-native-v1");
    ch.observe_bytes(
        &bincode::serialize(&(
            256usize,
            vk.log_n,
            vk.pin,
            &vk.circuit_digest,
            &vk.public_io,
            &vk.static_layout,
            &vk.read_layout,
            &vk.static_commitment,
            &vk.matrices,
            commitment,
        ))
        .map_err(fail)?,
    );
    Ok(ch)
}
fn bind_image<C: Challenger>(index: usize, ch: &mut C) {
    ch.observe_label(b"multiple-image-group-then-L-R-O");
    ch.observe_bytes(&(index as u64).to_le_bytes());
}
fn bind_lookup<C: Challenger>(index: usize, ch: &mut C) {
    ch.observe_label(b"multiple-sparse-evaluation-group-then-L-R-O");
    ch.observe_bytes(&(index as u64).to_le_bytes());
}

fn public_claims<C: Challenger>(vk: &VerificationKey, ch: &mut C) -> Vec<paired::Claim> {
    ch.observe_label(b"sha256-all-public-io-and-pin-v1");
    let mut point = ch.sample_f128_vec(10);
    let weights = build_eq(&point);
    let value = vk
        .public_io
        .par_iter()
        .zip(weights)
        .map(|(&v, w)| v * w)
        .reduce(|| F256::ZERO, |a, b| a + b);
    point.resize(vk.log_n, F128::ZERO);
    let pin: Vec<_> = (0..vk.log_n)
        .map(|i| {
            if vk.pin >> i & 1 == 1 {
                F128::ONE
            } else {
                F128::ZERO
            }
        })
        .collect();
    let ones = F128::new(u64::MAX, u64::MAX);
    vec![
        paired::Claim::at_point(&point, value),
        paired::Claim::at_point(&pin, F256::new(ones, ones)),
    ]
}
fn paired_queries<C: Challenger>(
    vk: &VerificationKey,
    claims: &[image::BatchClaims],
    values: &[F256],
    ch: &mut C,
) -> Result<Vec<paired::Claim>, String> {
    if claims.len() != 3 || values.len() != 3 {
        return Err("three combined matrix witness terminals required".into());
    }
    ch.observe_label(b"fix-both-witness-coordinates-before-pairing-v1");
    let mut out = Vec::new();
    for (claim, &value) in claims.iter().zip(values) {
        if claim.weights.len() != 2
            || claim
                .weights
                .iter()
                .zip(value.coordinates())
                .fold(F128::ZERO, |sum, (&z, v)| sum + z * v)
                != claim.witness.value
        {
            return Err("combined witness coordinates differ from IP terminal".into());
        }
        ch.observe_f256(value);
        out.push(paired::Claim::at_point(&claim.witness.point, value));
    }
    out.extend(public_claims(vk, ch));
    Ok(out)
}
fn opening_claims<C: Challenger>(
    opening: Opening,
    claims: &[paired::Claim],
    nonce: Option<u64>,
    proving: bool,
    params: &PcsParams,
    ch: &mut C,
) -> Result<(Option<u64>, Vec<TensorClaim>), String> {
    ch.observe_label(b"opening-mode-after-complete-F128-sparse-reduction-v1");
    ch.observe_label(opening.name().as_bytes());
    match opening {
        Opening::Paired if proving => {
            let (n, q) =
                paired::reduce_prover(claims, params.opening_grinding(), ch).map_err(fail)?;
            Ok((Some(n), q))
        }
        Opening::Paired => {
            let q = paired::reduce_verify(
                claims,
                nonce.ok_or("missing pair nonce")?,
                params.opening_grinding(),
                ch,
            )
            .map_err(fail)?;
            Ok((nonce, q))
        }
        Opening::Separate => {
            if nonce.is_some() {
                return Err("unexpected pairing nonce".into());
            }
            let mut out = Vec::new();
            for claim in claims {
                for group in 0..2 {
                    let mut rows = vec![if group == 0 {
                        [F128::ONE, F128::ZERO]
                    } else {
                        [F128::ZERO, F128::ONE]
                    }];
                    rows.extend_from_slice(&claim.rows);
                    out.push(TensorClaim {
                        columns: claim.columns.clone(),
                        rows,
                        value: claim.value.coordinates()[group],
                    });
                }
            }
            Ok((None, out))
        }
    }
}
fn distribute(
    index: usize,
    claims: Vec<sparse::Evaluation>,
    static_claims: &mut Vec<packed::Claim>,
    read_claims: &mut Vec<packed::Claim>,
) -> Result<(), String> {
    for c in claims {
        let dynamic = match c.source {
            sparse::OracleId::ReadRow => Some(0),
            sparse::OracleId::ReadCol => Some(1),
            _ => None,
        };
        let (target, column) = if let Some(axis) = dynamic {
            (&mut *read_claims, 2 * index + axis)
        } else {
            let field = sparse::OracleId::STATIC
                .iter()
                .position(|&id| id == c.source)
                .ok_or("unknown sparse source")?;
            (
                &mut *static_claims,
                (index % 3) * sparse::OracleId::STATIC.len() + field,
            )
        };
        target.push(packed::Claim {
            column,
            point: c.point,
            value: c.value,
        });
    }
    Ok(())
}

fn prove(
    p: &Prepared,
    owned_witness: Vec<F128>,
    opening: Opening,
) -> Result<(Proof, serde_json::Value, FsChallenger), String> {
    let vk = &p.vk;
    let t = Instant::now();
    let (commitment, witness_data) = pcs::commit(&owned_witness, &vk.witness_params);
    let witness_commit_ms = ms(t);
    let mut ch = GrindingChallenger::prover(bind(vk, &commitment)?);
    let t = Instant::now();
    let (hadamard, block_claims) = mikros::hadamard::prove_batch_shared_rows(
        &[bytes(&p.images[0]), bytes(&p.images[3])],
        &[bytes(&p.images[1]), bytes(&p.images[4])],
        &[bytes(&p.images[2]), bytes(&p.images[5])],
        vk.log_n + 7,
        mikros::Kernel::PairAggregate,
        &mut ch,
    )
    .map_err(fail)?;
    let sources: Vec<_> = block_claims
        .into_iter()
        .flat_map(|claims| [claims.a, claims.b, claims.c])
        .collect();
    let hadamard_ms = ms(t);
    let t = Instant::now();
    let matrices = [&p.matrices[0], &p.matrices[1], &p.matrices[2]];
    let mut images = Vec::new();
    let mut image_claims = Vec::new();
    for i in 0..3 {
        bind_image(i, &mut ch);
        let (proof, claims) = image::prove_batch(
            matrices[i],
            &[&p.groups[0], &p.groups[1]],
            &[&p.images[i], &p.images[i + 3]],
            &[sources[i].clone(), sources[i + 3].clone()],
            &mut ch,
        )?;
        images.push(proof);
        image_claims.push(claims);
    }
    let image_reduction_ms = ms(t);
    let t = Instant::now();
    let reads: Vec<_> = image_claims
        .iter()
        .enumerate()
        .map(|(i, c)| {
            p.sparse[i % 3]
                .gather(&c.matrix.row_point, &c.matrix.column_point)
                .map_err(fail)
        })
        .collect::<Result<_, _>>()?;
    // Read columns already contain complete native words. Copy them directly
    // into the aligned carrier instead of cloning and repacking each element.
    let mut read_words = vec![F128::ZERO; vk.read_layout.word_len()];
    for (index, values) in reads
        .iter()
        .flat_map(|r| [&r.read_row, &r.read_col])
        .enumerate()
    {
        let spec = &vk.read_layout.specs()[index];
        if spec.width != 128 || spec.len != values.len() || spec.word_len != values.len() {
            return Err("full-word read packing layout mismatch".into());
        }
        read_words[spec.word_offset..spec.word_offset + values.len()].copy_from_slice(values);
    }
    let (read_commitment, read_data) = pcs::commit(&read_words, &vk.read_layout.parameters());
    ch.observe_label(b"commit-all-query-dependent-SPARK-reads-before-lookup-challenges");
    ch.observe_bytes(&bincode::serialize(&read_commitment).map_err(fail)?);
    let read_prepare_commit_ms = ms(t);
    let t = Instant::now();
    let mut lookups = Vec::new();
    let mut static_claims = Vec::new();
    let mut read_claims = Vec::new();
    for i in 0..3 {
        bind_lookup(i, &mut ch);
        let m = &image_claims[i].matrix;
        let (proof, claims) = sparse::prove(
            &p.sparse[i % 3],
            &vk.matrices[i % 3],
            &m.row_point,
            &m.column_point,
            m.value,
            &reads[i],
            &mut ch,
        )
        .map_err(fail)?;
        lookups.push(proof);
        distribute(i, claims, &mut static_claims, &mut read_claims)?;
    }
    let lookup_reduction_ms = ms(t);
    let t = Instant::now();
    let witness_values: Vec<_> = image_claims
        .iter()
        .map(|c| evaluate_pair(&p.groups, &c.witness.point))
        .collect();
    let queries = paired_queries(vk, &image_claims, &witness_values, &mut ch)?;
    let public_and_pair_values_ms = ms(t);
    let piop_grinding_ms = ch.grinding_ms();
    let (mut ch, nonces) = ch.finish()?;
    let t = Instant::now();
    let (pairing_nonce, witness_claims) =
        opening_claims(opening, &queries, None, true, &vk.witness_params, &mut ch)?;
    let pair_reduction_ms = ms(t);
    let t = Instant::now();
    let registry = container::registry(vk.log_n + 1, vk.log_n + 1);
    let witness_pcs = merged_tensor::open_identity(
        owned_witness,
        &witness_data,
        &commitment,
        &witness_claims,
        &container::union(&registry),
        &mut ch,
    )
    .map_err(fail)?;
    let witness_open_ms = ms(t);
    let t = Instant::now();
    let static_pcs = vk.static_layout.open(
        p.static_words.clone(),
        &p.static_data,
        &vk.static_commitment,
        &static_claims,
        &mut ch,
    )?;
    let static_open_ms = ms(t);
    let t = Instant::now();
    let read_pcs = vk.read_layout.open(
        read_words,
        &read_data,
        &read_commitment,
        &read_claims,
        &mut ch,
    )?;
    let read_open_ms = ms(t);
    let times = json!({"witness_commit_ms":witness_commit_ms,"hadamard_ms":hadamard_ms,"image_reduction_ms":image_reduction_ms,
        "piop_grinding_ms":piop_grinding_ms,
        "read_prepare_commit_ms":read_prepare_commit_ms,"lookup_reduction_ms":lookup_reduction_ms,
        "public_and_pair_values_ms":public_and_pair_values_ms,"pair_reduction_ms":pair_reduction_ms,
        "witness_open_ms":witness_open_ms,"static_open_ms":static_open_ms,"read_open_ms":read_open_ms,
        "hadamard_proofs":2,"matrix_evaluation_proofs":3,"lookup_arguments":6,
        "witness_claims":witness_claims.len(),"static_claims":static_claims.len(),"read_claims":read_claims.len(),"inner_openings":3});
    Ok((
        Proof {
            opening,
            commitment,
            read_commitment,
            hadamard,
            images,
            lookups,
            witness_values,
            nonces,
            pairing_nonce,
            witness_pcs,
            static_pcs,
            read_pcs,
        },
        times,
        ch,
    ))
}

fn verify(vk: &VerificationKey, proof: &Proof, opening: Opening) -> Result<FsChallenger, String> {
    if proof.opening != opening
        || proof.commitment.params != vk.witness_params
        || proof.read_commitment.params != vk.read_layout.parameters()
        || proof.hadamard.len() != 2
        || proof.images.len() != 3
        || proof.lookups.len() != 3
    {
        return Err("invalid complete sparse R1CS proof shape".into());
    }
    let mut ch = GrindingChallenger::verifier(bind(vk, &proof.commitment)?, &proof.nonces);
    let sources: Vec<_> =
        mikros::hadamard::verify_batch_shared_rows(&proof.hadamard, vk.log_n + 7, &mut ch)
            .map_err(fail)?
            .into_iter()
            .flat_map(|q| [q.a, q.b, q.c])
            .collect();
    let mut image_claims = Vec::new();
    for i in 0..3 {
        bind_image(i, &mut ch);
        image_claims.push(image::verify_batch(
            vk.log_n,
            &[sources[i].clone(), sources[i + 3].clone()],
            &proof.images[i],
            &mut ch,
        )?);
    }
    ch.observe_label(b"commit-all-query-dependent-SPARK-reads-before-lookup-challenges");
    ch.observe_bytes(&bincode::serialize(&proof.read_commitment).map_err(fail)?);
    let mut static_claims = Vec::new();
    let mut read_claims = Vec::new();
    for i in 0..3 {
        bind_lookup(i, &mut ch);
        let m = &image_claims[i].matrix;
        let claims = sparse::verify(
            &vk.matrices[i % 3],
            &m.row_point,
            &m.column_point,
            m.value,
            &proof.lookups[i],
            &mut ch,
        )
        .map_err(fail)?;
        distribute(i, claims, &mut static_claims, &mut read_claims)?;
    }
    let queries = paired_queries(vk, &image_claims, &proof.witness_values, &mut ch)?;
    let (mut ch, _) = ch.finish()?;
    let (_, witness_claims) = opening_claims(
        opening,
        &queries,
        proof.pairing_nonce,
        false,
        &vk.witness_params,
        &mut ch,
    )?;
    let registry = container::registry(vk.log_n + 1, vk.log_n + 1);
    merged_tensor::verify(
        &proof.commitment,
        &witness_claims,
        &proof.witness_pcs,
        &container::union(&registry),
        &mut ch,
    )
    .map_err(fail)?;
    vk.static_layout.verify(
        &vk.static_commitment,
        &static_claims,
        &proof.static_pcs,
        &mut ch,
    )?;
    vk.read_layout.verify(
        &proof.read_commitment,
        &read_claims,
        &proof.read_pcs,
        &mut ch,
    )?;
    Ok(ch)
}

fn tamper(vk: &VerificationKey, proof: &Proof) -> Result<usize, String> {
    let reject = |p: &Proof| {
        if verify(vk, p, proof.opening).is_err() {
            Ok(())
        } else {
            Err("tampered complete proof accepted".to_owned())
        }
    };
    let mut count = 0;
    for i in 0..3 {
        let mut bad = proof.clone();
        for group in 0..2 {
            bad.images[i].columns[group][0] += F128::ONE;
            reject(&bad)?;
            count += 1;
            bad = proof.clone();
        }
        let mut bad = proof.clone();
        bad.images[i].ip2.final_a += F128::ONE;
        reject(&bad)?;
        count += 1;
    }
    for coordinate in 0..2 {
        let mut bad = proof.clone();
        if coordinate == 0 {
            bad.witness_values[0].c0 += F128::ONE
        } else {
            bad.witness_values[0].c1 += F128::ONE
        };
        reject(&bad)?;
        count += 1;
    }
    let mut bad = proof.clone();
    bad.commitment.cap[0][0] ^= 1;
    reject(&bad)?;
    count += 1;
    let mut bad = proof.clone();
    bad.read_commitment.cap[0][0] ^= 1;
    reject(&bad)?;
    count += 1;
    let mut bad = proof.clone();
    bad.nonces.pop();
    reject(&bad)?;
    count += 1;
    for source in 0..3 {
        let mut bad = proof.clone();
        let pcs = match source {
            0 => &mut bad.witness_pcs,
            1 => &mut bad.static_pcs,
            _ => &mut bad.read_pcs,
        };
        pcs.merged.as_mut().ok_or("missing inner opening")?.q_eval += F128::ONE;
        reject(&bad)?;
        count += 1;
    }
    if let Some(n) = proof.pairing_nonce {
        let mut bad = proof.clone();
        bad.pairing_nonce = Some(n ^ 1);
        reject(&bad)?;
        count += 1;
    }
    for index in 0..3 {
        let mut bad = proof.clone();
        bad.lookups.swap(index, (index + 1) % 3);
        reject(&bad)?;
        count += 1;
    }
    for group in 0..2 {
        let mut bad = proof.clone();
        bad.hadamard[group].inner_product.final_a += F128::ONE;
        reject(&bad)?;
        count += 1;
    }
    for field in 0..4 {
        let mut bad_vk = vk.clone();
        match field {
            0 => bad_vk.public_io[0].c0 += F128::ONE,
            1 => bad_vk.public_io[512].c1 += F128::ONE,
            2 => bad_vk.pin ^= 1,
            _ => bad_vk.circuit_digest[0] ^= 1,
        }
        if verify(&bad_vk, proof, proof.opening).is_ok() {
            return Err("changed public statement accepted".into());
        }
        count += 1;
    }
    Ok(count)
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut threads, mut reps, mut warmups, mut check) = (64, 6, 1, false);
    let mut modes = vec![Opening::Paired];
    let mut args = env::args().skip(1);
    while let Some(flag) = args.next() {
        if flag == "--tamper" {
            check = true;
            continue;
        }
        let value = args.next().ok_or("missing argument")?;
        match flag.as_str() {
            "--threads" => threads = value.parse()?,
            "--reps" => reps = value.parse()?,
            "--warmups" => warmups = value.parse()?,
            "--opening" => {
                modes = match value.as_str() {
                    "paired" => vec![Opening::Paired],
                    "separate" => vec![Opening::Separate],
                    "both" => vec![Opening::Separate, Opening::Paired],
                    _ => return Err("unknown opening".into()),
                }
            }
            _ => return Err(format!("unknown argument {flag}").into()),
        }
    }
    if threads == 0 || reps == 0 || env::var_os("FLOCK_NO_GRIND").is_some() {
        return Err("invalid configuration".into());
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .stack_size(8 << 20)
        .build_global()?;
    if !flock_parallel::init_all_core_pool(threads) {
        return Err("all-core pool already initialized".into());
    }
    let t = Instant::now();
    let f = TreeFixture::independent_compressions(256);
    let p = prepare(&f)?;
    let offline_ms = ms(t);
    let mut out = io::BufWriter::new(io::stdout().lock());
    writeln!(
        out,
        "{}",
        json!({"record":"metadata","schema_version":3,"fixture":"sha256-independent-compressions","sha_compressions":256,
        "batch_reduction":"shared Hadamard row challenges; per-block column challenges; column-first random combination of linear images",
        "threads":threads,"all_core_threads":flock_parallel::all_core_pool().current_num_threads(),"reduction_field_bits":128,"opening_logical_field_bits":256,
        "fixture_digest_blake3":p.vk.fixture_digest,"circuit_digest_blake3":p.vk.circuit_digest.iter().map(|b|format!("{b:02x}")).collect::<String>(),
        "native_flock_circuit_digest_blake3":f.circuit.digest().iter().map(|b|format!("{b:02x}")).collect::<String>(),
        "sha_r1cs_encoding":if env::var_os("MIKROS_SHA_LEGACY_CARRIES").is_none() {
            if env::var_os("MIKROS_SHA_CARRY_ONLY").is_some_and(|v| v == "1") {
                "Mikros materializes schedule/T1/E/A and stores actual carry bits; C authenticates carry corrections; 31102 useful slots in 32768 rows, original public I/O and pin; native FLOCK unchanged"
            } else {
                "Mikros materializes schedule/T1/E/A and stores actual carry/Ch/Maj values; nonidentity C authenticates affine corrections; 31102 useful slots in 32768 rows, original public I/O and pin; native FLOCK unchanged"
            }
        } else {
            "Mikros materializes schedule/T1/E/A with legacy auxiliary carries and identity C; 31102 useful slots in 32768 rows, original public I/O and pin; native FLOCK unchanged"
        },
        "pcs_params":p.vk.witness_params,"static_pcs_params":p.vk.static_layout.parameters(),"read_pcs_params":p.vk.read_layout.parameters(),
        "static_columns":p.vk.static_layout.specs(),"read_columns":p.vk.read_layout.specs(),"nonzeros":p.vk.matrices.iter().map(|s|s.nonzeros).collect::<Vec<_>>(),
        "static_column_value_bits":p.static_column_value_bits,
        "static_layout_policy":{"balanced_padding":env::var_os("MIKROS_SPARSE_ZERO_PADDING_ADDRESS").is_none(),"narrow_values":env::var_os("MIKROS_SPARSE_WIDE_METADATA").is_none()},
        "static_commitment_bytes":p.static_words.len()*16,"witness_commitment_bytes":p.witness.len()*16,"read_commitment_bytes":p.vk.read_layout.word_len()*16,
        "offline_circuit_witness_sparse_commit_ms":offline_ms,"target_features":features(),"matrix_authentication":"packed sparse SPARK memory-checking lookup; all L/R/O; verifier receives no matrices",
        "timing":"offline circuit, fixed sparse metadata commitment, witness/image preparation and input clone excluded; all query-dependent reads, read commitment, F128 reductions, lookup proofs and three native PCS openings included",
        "comparison":"native FLOCK retains its native circuit representation and prover API; same SHA public statement and Ligerito Fast backend, differing layouts/PIOPs",
        "zero_knowledge":false})
    )?;
    out.flush()?;
    let mut hashes = std::collections::BTreeMap::new();
    let mut common_piop = None;
    for iteration in 0..warmups + reps {
        let mut order = modes.clone();
        if iteration % 2 == 1 {
            order.reverse()
        }
        for opening in order {
            let owned = p.witness.clone();
            let t = Instant::now();
            let (proof, mut timings, mut ch) = prove(&p, owned, opening)?;
            timings["prover_ms"] = json!(ms(t));
            let t = Instant::now();
            let mut vch = verify(&p.vk, &proof, opening)?;
            let verify_ms = ms(t);
            if ch.sample_f128() != vch.sample_f128() {
                return Err("transcript mismatch".into());
            }
            let encoded = bincode::serialize(&proof)?;
            let hash = blake3::hash(&encoded).to_hex().to_string();
            if hashes
                .insert(opening.name(), hash.clone())
                .is_some_and(|x| x != hash)
            {
                return Err("unstable proof hash".into());
            }
            let piop = blake3::hash(&bincode::serialize(&(
                &proof.hadamard,
                &proof.images,
                &proof.lookups,
                &proof.read_commitment,
                &proof.witness_values,
                &proof.nonces,
            ))?)
            .to_hex()
            .to_string();
            if common_piop.as_ref().is_some_and(|x| x != &piop) {
                return Err("opening changed F128 reductions".into());
            }
            common_piop = Some(piop.clone());
            writeln!(
                out,
                "{}",
                json!({"record":"sample","system":"mikros","opening":opening.name(),"iteration":iteration,"warmup":iteration<warmups,
                "timings":timings,"verify_ms":verify_ms,"proof_bytes":encoded.len(),"proof_blake3":hash,"piop_blake3":piop,
                "commitment_blake3":blake3::hash(&bincode::serialize(&proof.commitment)?).to_hex().to_string(),"verified":true,"transcript_match":true})
            )?;
            out.flush()?;
            if check && iteration == 0 {
                let rejected = tamper(&p.vk, &proof)?;
                writeln!(
                    out,
                    "{}",
                    json!({"record":"negative_tests","opening":opening.name(),"rejected":rejected})
                )?;
                out.flush()?;
            }
        }
    }
    Ok(())
}
fn features() -> Vec<&'static str> {
    [
        ("avx512f", cfg!(target_feature = "avx512f")),
        ("avx512bw", cfg!(target_feature = "avx512bw")),
        ("gfni", cfg!(target_feature = "gfni")),
        ("vpclmulqdq", cfg!(target_feature = "vpclmulqdq")),
    ]
    .into_iter()
    .filter_map(|(s, b)| b.then_some(s))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::e2e_common::batched_boolean;
    #[test]
    fn paired_evaluation_matches_independent_coordinate_queries() {
        for log_n in [0, 1, 5, 12, 15] {
            let n = 1 << log_n;
            let groups: Vec<Vec<F128>> = (0..2)
                .map(|g| {
                    (0..n)
                        .map(|i| {
                            F128::new(
                                (i as u64).wrapping_mul(0x9876123400000051 + g),
                                (i as u64).wrapping_mul(0x19982340000617 + g) ^ 0xAC,
                            )
                        })
                        .collect()
                })
                .collect();
            for point in [
                vec![F128::ZERO; log_n],
                vec![F128::ONE; log_n],
                (0..log_n)
                    .map(|i| F128::new(7 + i as u64, 19 * i as u64))
                    .collect(),
            ] {
                assert_eq!(
                    evaluate_pair(&groups, &point),
                    F256::new(evaluate(&groups[0], &point), evaluate(&groups[1], &point))
                );
            }
        }
    }
    #[test]
    fn all_256_public_inputs_outputs_and_pin_match_packed_witness() {
        let f = TreeFixture::independent_compressions(256);
        let packed = batched_boolean::prepare(&f.prepare_witness(), f.nu, f.r1cs.k_log).unwrap();
        for (i, value) in public_io(&f.verifier()).iter().enumerate() {
            assert_eq!(*value, F256::new(packed.z[2 * i], packed.z[2 * i + 1]));
        }
        let pin = f.r1cs.const_pin.unwrap();
        let ones = F128::new(u64::MAX, u64::MAX);
        assert_eq!([packed.z[2 * pin], packed.z[2 * pin + 1]], [ones; 2]);
    }
}
