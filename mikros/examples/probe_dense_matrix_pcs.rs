//! Native PCS probe for dense bit commitments to fixed SHA sparse matrices.
//! This does not prove a SHA computation and does not replace an E2E result.

use flock_core::{
    challenger::{Challenger, FsChallenger},
    pcs::Commitment,
    r1cs::SparseBinaryMatrix,
    zerocheck::univariate_skip::build_eq,
};
use flock_field::F128;
use flock_prover::r1cs_hashes::sha2::build_sparse_block_r1cs;
use mikros::packed_sources::{Claim, Layout};
use rayon::prelude::*;
use serde_json::json;
use std::{
    env,
    io::{self, Write},
    time::Instant,
};

const LOG_N: usize = 15;

fn ms(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

/// Direct row-major bit packing. Repeated binary entries cancel by XOR.
fn fill_matrix_words(matrix: &SparseBinaryMatrix, words: &mut [F128]) {
    assert!(matrix.num_cols.is_power_of_two() && matrix.num_cols >= 128);
    assert_eq!(matrix.rows.len(), matrix.num_rows);
    let row_words = matrix.num_cols / 128;
    assert_eq!(words.len(), matrix.num_rows * row_words);
    words
        .par_chunks_mut(row_words)
        .zip(matrix.rows.par_iter())
        .for_each(|(out, columns)| {
            out.fill(F128::ZERO);
            for &column in columns {
                assert!(column < matrix.num_cols);
                let bit = column % 128;
                let word = &mut out[column / 128];
                if bit < 64 {
                    word.lo ^= 1u64 << bit;
                } else {
                    word.hi ^= 1u64 << (bit - 64);
                }
            }
        });
}

fn matrix_evaluate(matrix: &SparseBinaryMatrix, row: &[F128], column: &[F128]) -> F128 {
    let row_eq = build_eq(row);
    let col_eq = build_eq(column);
    assert_eq!(row_eq.len(), matrix.num_rows);
    assert_eq!(col_eq.len(), matrix.num_cols);
    matrix
        .rows
        .par_iter()
        .zip(row_eq)
        .map(|(columns, weight)| {
            weight * columns.iter().fold(F128::ZERO, |sum, &j| sum + col_eq[j])
        })
        .reduce(|| F128::ZERO, |a, b| a + b)
}

fn start(layout: &Layout, commitment: &Commitment, matrices: usize) -> FsChallenger {
    let mut ch = FsChallenger::new(b"mikros-dense-sha-matrix-native-pcs-probe-v1");
    ch.observe_bytes(&bincode::serialize(&(LOG_N, matrices, layout, commitment)).unwrap());
    ch
}

fn points(ch: &mut FsChallenger, matrices: usize) -> Vec<Vec<F128>> {
    // Natural packed-bit index is row*N+column, so the low 15 coordinates
    // are the column point and the next 15 coordinates are the row point.
    (0..2 * matrices)
        .map(|_| ch.sample_f128_vec(2 * LOG_N))
        .collect()
}

fn features() -> Vec<&'static str> {
    [
        ("avx512f", cfg!(target_feature = "avx512f")),
        ("avx512bw", cfg!(target_feature = "avx512bw")),
        ("gfni", cfg!(target_feature = "gfni")),
        ("vpclmulqdq", cfg!(target_feature = "vpclmulqdq")),
    ]
    .into_iter()
    .filter_map(|(name, enabled)| enabled.then_some(name))
    .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut threads, mut reps, mut warmups, mut matrix_count) = (64usize, 3usize, 1usize, 3usize);
    let mut args = env::args().skip(1);
    while let Some(flag) = args.next() {
        let value = args.next().ok_or("missing option value")?;
        match flag.as_str() {
            "--threads" => threads = value.parse()?,
            "--reps" => reps = value.parse()?,
            "--warmups" => warmups = value.parse()?,
            "--matrices" => matrix_count = value.parse()?,
            _ => return Err(format!("unknown option {flag}").into()),
        }
    }
    if threads == 0 || reps == 0 || !(2..=3).contains(&matrix_count) {
        return Err("require positive threads/reps and --matrices 2 or 3".into());
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .stack_size(8 << 20)
        .build_global()?;
    if !flock_parallel::init_all_core_pool(threads) {
        return Err("all-core pool already initialized".into());
    }
    let offline = Instant::now();
    let build = Instant::now();
    let r1cs = build_sparse_block_r1cs(8);
    let matrix_build_ms = ms(build);
    assert_eq!(r1cs.k_log, LOG_N);
    let matrices = [&r1cs.a_0, &r1cs.b_0, &r1cs.c_0];
    let matrices = &matrices[..matrix_count];
    assert!(
        matrices
            .iter()
            .all(|m| m.num_rows == 1 << LOG_N && m.num_cols == 1 << LOG_N)
    );
    let nnz: Vec<_> = matrices
        .iter()
        .map(|m| m.rows.iter().map(Vec::len).sum::<usize>())
        .collect();
    let layout = Layout::new(&vec![(1usize << (2 * LOG_N), 1); matrix_count])?;
    let pack = Instant::now();
    let mut carrier = vec![F128::ZERO; layout.word_len()];
    for (matrix, spec) in matrices.iter().zip(layout.specs()) {
        fill_matrix_words(
            matrix,
            &mut carrier[spec.word_offset..spec.word_offset + spec.word_len],
        );
    }
    let matrix_pack_ms = ms(pack);
    let commit = Instant::now();
    let (commitment, data) = layout.commit(&carrier)?;
    let matrix_commit_ms = ms(commit);
    let offline_ms = ms(offline);
    let mut out = io::BufWriter::new(io::stdout().lock());
    let commitment_hash = blake3::hash(&bincode::serialize(&commitment)?)
        .to_hex()
        .to_string();
    writeln!(
        out,
        "{}",
        json!({
            "record":"metadata", "schema_version":1, "probe":"dense-binary-sha-matrix-native-pcs",
            "scope":"PCS microbenchmark only; not an E2E SHA proof",
            "matrices":matrix_count,"matrix_names":&["A","B","C"][..matrix_count],
            "queries_per_matrix":2,"claims":2*matrix_count,"row_log":LOG_N,"column_log":LOG_N,
            "binary_entries_per_matrix":1usize<<(2*LOG_N),"nonzero_entries":nnz,
            "carrier_input_bytes":carrier.len()*16,"native_codeword_bytes":data.codeword.len()*16,
            "serialized_commitment_bytes":bincode::serialized_size(&commitment)?,
            "pcs_params":layout.parameters(),"columns":layout.specs(),"commitment_blake3":commitment_hash,
            "threads":threads,"all_core_threads":flock_parallel::all_core_pool().current_num_threads(),
            "target_features":features(),"reps":reps,"warmups":warmups,
            "offline_ms":offline_ms,"matrix_build_ms":matrix_build_ms,"matrix_pack_ms":matrix_pack_ms,"matrix_commit_ms":matrix_commit_ms,
            "timing":"fixed matrix construction, bit packing and commitment offline; query generation, sparse matrix evaluation, carrier clone and native opening online; verifier receives no matrices",
            "environment":env::vars().filter(|(k,_)|k.starts_with("FLOCK_")||k.starts_with("MIKROS_")||k=="RUSTFLAGS"||k=="RAYON_NUM_THREADS").collect::<std::collections::BTreeMap<_,_>>()
        })
    )?;
    out.flush()?;
    let mut proof_hash = None;
    for iteration in 0..warmups + reps {
        let total = Instant::now();
        let query = Instant::now();
        let mut ch = start(&layout, &commitment, matrix_count);
        let query_points = points(&mut ch, matrix_count);
        let claims: Vec<_> = query_points
            .into_iter()
            .enumerate()
            .map(|(i, point)| {
                let value =
                    matrix_evaluate(matrices[i % matrix_count], &point[LOG_N..], &point[..LOG_N]);
                Claim {
                    column: i % matrix_count,
                    point,
                    value,
                }
            })
            .collect();
        let matrix_evaluation_ms = ms(query);
        let copy = Instant::now();
        let owned = carrier.clone();
        let source_clone_ms = ms(copy);
        let opening = Instant::now();
        let proof = layout.open(owned, &data, &commitment, &claims, &mut ch)?;
        let opening_ms = ms(opening);
        let prover_ms = ms(total);
        let verification = Instant::now();
        let mut vch = start(&layout, &commitment, matrix_count);
        let expected_points = points(&mut vch, matrix_count);
        if !claims
            .iter()
            .zip(&expected_points)
            .all(|(c, p)| &c.point == p)
        {
            return Err("query replay differs".into());
        }
        layout.verify(&commitment, &claims, &proof, &mut vch)?;
        let verify_ms = ms(verification);
        if ch.sample_f128() != vch.sample_f128() {
            return Err("transcript mismatch".into());
        }
        // Commitment is fixed preprocessing data; these bytes are the actual
        // standalone opening bundle including the six claimed MLE values.
        let encoded = bincode::serialize(&(&claims, &proof))?;
        let hash = blake3::hash(&encoded).to_hex().to_string();
        if proof_hash.as_ref().is_some_and(|old| old != &hash) {
            return Err("proof bytes changed between repetitions".into());
        }
        proof_hash = Some(hash.clone());
        writeln!(
            out,
            "{}",
            json!({"record":"sample","iteration":iteration,"warmup":iteration<warmups,
            "matrix_evaluation_ms":matrix_evaluation_ms,"source_clone_ms":source_clone_ms,"opening_ms":opening_ms,"prover_ms":prover_ms,
            "verify_ms":verify_ms,"opening_proof_bytes":encoded.len(),"proof_blake3":hash,
            "commitment_blake3":commitment_hash,"verified":true,"transcript_match":true})
        )?;
        out.flush()?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn packed_matrix_xor_and_point_order() {
        let matrix =
            SparseBinaryMatrix::new(2, 256, vec![vec![0, 127, 128, 255, 128], vec![1, 129]]);
        let mut packed = vec![F128::ZERO; 4];
        fill_matrix_words(&matrix, &mut packed);
        assert_eq!(
            packed,
            [
                F128::new(1, 1 << 63),
                F128::new(0, 1 << 63),
                F128::new(2, 0),
                F128::new(2, 0)
            ]
        );
        let column: Vec<_> = (0..8).map(|i| F128::new(9 + i, 1)).collect();
        let row = [F128::new(91, 13)];
        let point: Vec<_> = column.iter().chain(&row).copied().collect();
        let eq = build_eq(&point);
        let dense = (0..512).fold(F128::ZERO, |sum, i| {
            let w = packed[i / 128];
            let bit = if i % 128 < 64 {
                (w.lo >> (i % 64)) & 1
            } else {
                (w.hi >> (i % 64)) & 1
            };
            if bit == 1 { sum + eq[i] } else { sum }
        });
        assert_eq!(matrix_evaluate(&matrix, &row, &column), dense);
    }
}
