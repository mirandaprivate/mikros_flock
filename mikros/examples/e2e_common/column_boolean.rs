//! Packed R1CS: independent Hadamards, shared row queries, column projection.
//! Public CSC checks the matrix endpoints directly. All witness endpoints,
//! including C=I, are authenticated by the original native PCS commitment.
use super::{TreeFixture, TreeVerifier, batched_boolean::TransposedWitness};
use flock_core::{challenger::Challenger, lincheck::LincheckCircuit, pcs::functional::TensorClaim};
use flock_field::F128;
use mikros::{MatrixClaim, hadamard, multiple_image};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proof {
    pub hadamards: Vec<hadamard::Proof>,
    pub linchecks: Vec<multiple_image::PublicBatchProof>,
}
pub struct Claims {
    pub other: Vec<TensorClaim>,
}
fn shape(view: &TreeVerifier<'_>, circuit: &dyn LincheckCircuit) -> Result<usize, String> {
    if view.nu != 8 || view.live_rows != 256 || !(7..=24).contains(&view.r1cs.k_log) {
        return Err("column R1CS requires 256 live executions".into());
    }
    let n = 1 << view.r1cs.k_log;
    if !view.r1cs.c0_is_identity()
        || circuit.n_cols() != n
        || circuit.const_pin_col() != view.r1cs.const_pin
    {
        return Err("column R1CS requires matching public matrix, constant pin and C=I".into());
    }
    Ok(n)
}
fn basis() -> Vec<F128> {
    (0..128)
        .map(|j| {
            if j < 64 {
                F128::new(1u64 << j, 0)
            } else {
                F128::new(0, 1u64 << (j - 64))
            }
        })
        .collect()
}
fn start<C: Challenger>(log_n: usize, ch: &mut C) {
    ch.observe_label(b"mikros-sha256-packed-r1cs-column-projection-v1");
    ch.observe_bytes(&(log_n as u64).to_le_bytes());
    ch.observe_bytes(&256u64.to_le_bytes());
}
fn witness_claim(claim: multiple_image::PublicBatchClaims) -> Result<TensorClaim, String> {
    // The group factors may sum to zero; no normalization is allowed.
    let weights: [F128; 2] = claim
        .weights
        .try_into()
        .map_err(|_| "column R1CS block weights")?;
    let mut rows = vec![weights];
    rows.extend(claim.witness.point.iter().map(|&r| [F128::ONE + r, r]));
    Ok(TensorClaim {
        columns: basis(),
        rows,
        value: claim.witness.value,
    })
}
fn identity_claim(group: usize, query: &MatrixClaim) -> TensorClaim {
    let mut rows = vec![if group == 0 {
        [F128::ONE, F128::ZERO]
    } else {
        [F128::ZERO, F128::ONE]
    }];
    rows.extend(query.row_factors.iter().rev().copied());
    TensorClaim {
        columns: query.column_weights.clone(),
        rows,
        value: query.value,
    }
}
fn bytes(words: &[F128]) -> &[u8] {
    assert!(cfg!(target_endian = "little"));
    // F128 is repr(C), lo then hi, representing binary instance coordinates.
    unsafe { std::slice::from_raw_parts(words.as_ptr().cast(), std::mem::size_of_val(words)) }
}
fn deinterleave(words: &[F128], serial: bool) -> [Vec<F128>; 2] {
    assert_eq!(words.len() % 2, 0);
    if serial {
        let mut even = Vec::with_capacity(words.len() / 2);
        let mut odd = Vec::with_capacity(words.len() / 2);
        for pair in words.chunks_exact(2) {
            even.push(pair[0]);
            odd.push(pair[1]);
        }
        [even, odd]
    } else {
        std::array::from_fn(|group| words.par_chunks_exact(2).map(|w| w[group]).collect())
    }
}
pub fn prove<C: Challenger>(
    fixture: &TreeFixture,
    witness: &TransposedWitness,
    ch: &mut C,
) -> Result<(Proof, Claims), String> {
    let view = fixture.verifier();
    prove_with_matrix(&view, view.r1cs.csc_lincheck_circuit(), witness, ch)
}

/// The matrix is public verifier configuration, shared with the FLOCK prover.
/// CSC and an equivalent native circuit walker use the same image protocol.
pub fn prove_with_matrix<C: Challenger>(
    view: &TreeVerifier<'_>,
    circuit: &dyn LincheckCircuit,
    witness: &TransposedWitness,
    ch: &mut C,
) -> Result<(Proof, Claims), String> {
    let profile = std::env::var("MIKROS_PROFILE").is_ok_and(|v| v == "1");
    let timer = profile.then(std::time::Instant::now);
    let elapsed = || timer.map_or(0.0, |t| t.elapsed().as_secs_f64() * 1000.0);
    let n = shape(view, circuit)?;
    if [&witness.z, &witness.a, &witness.b]
        .iter()
        .any(|v| v.len() != 2 * n)
    {
        return Err("column R1CS transposed witness shape".into());
    }
    start(view.r1cs.k_log, ch);
    // Temporary deinterleaving is included in online time, unlike the initial
    // challenge-independent conversion common to the compared modes.
    // The measured small SHA carrier benefits from one sequential pass.
    // Larger carriers retain parallel copying unless explicitly overridden.
    let serial_deinterleave = std::env::var_os("MIKROS_SERIAL_DEINTERLEAVE")
        .map_or(witness.z.len() <= 1 << 17, |value| value == "1");
    let packed = deinterleave(&witness.z, serial_deinterleave);
    let deinterleave_end = elapsed();
    let a = [bytes(&witness.a[..n]), bytes(&witness.a[n..])];
    let b = [bytes(&witness.b[..n]), bytes(&witness.b[n..])];
    let c = [bytes(&packed[0]), bytes(&packed[1])];
    let (hadamards, sources) = hadamard::prove_batch_shared_rows(
        &a,
        &b,
        &c,
        view.r1cs.k_log + 7,
        mikros::Kernel::PairAggregate,
        ch,
    )
    .map_err(|e| format!("column R1CS Hadamard: {e:?}"))?;
    let hadamard_end = elapsed();
    let mut linchecks = Vec::with_capacity(2);
    let mut other = Vec::with_capacity(4);
    let witnesses = [packed[0].as_slice(), packed[1].as_slice()];
    for side in 0..2 {
        ch.observe_label(b"mikros-column-r1cs-linear-image-batch-v1");
        ch.observe_bytes(&(side as u64).to_le_bytes());
        let image = if side == 0 { &witness.a } else { &witness.b };
        let queries: Vec<_> = sources
            .iter()
            .map(|q| if side == 0 { q.a.clone() } else { q.b.clone() })
            .collect();
        let (proof, claim) = multiple_image::prove_public_batch(
            circuit,
            side == 0,
            &witnesses,
            &[&image[..n], &image[n..]],
            &queries,
            ch,
        )?;
        linchecks.push(proof);
        other.push(witness_claim(claim)?);
    }
    let images_end = elapsed();
    other.extend(
        sources
            .iter()
            .enumerate()
            .map(|(group, q)| identity_claim(group, &q.c)),
    );
    if profile {
        eprintln!(
            "{}",
            serde_json::json!({"record":"mikros_column_r1cs_profile",
            "serial_deinterleave":serial_deinterleave,
            "deinterleave_ms":deinterleave_end, "hadamard_ms":hadamard_end-deinterleave_end,
            "images_ms":images_end-hadamard_end, "claim_map_ms":elapsed()-images_end,
            "total_ms":elapsed()})
        );
    }
    Ok((
        Proof {
            hadamards,
            linchecks,
        },
        Claims { other },
    ))
}
pub fn verify<C: Challenger>(
    view: &TreeVerifier<'_>,
    proof: &Proof,
    ch: &mut C,
) -> Result<Claims, String> {
    verify_with_matrix(view, view.r1cs.csc_lincheck_circuit(), proof, ch)
}

pub fn verify_with_matrix<C: Challenger>(
    view: &TreeVerifier<'_>,
    circuit: &dyn LincheckCircuit,
    proof: &Proof,
    ch: &mut C,
) -> Result<Claims, String> {
    shape(view, circuit)?;
    if proof.hadamards.len() != 2 || proof.linchecks.len() != 2 {
        return Err("column R1CS proof block count".into());
    }
    start(view.r1cs.k_log, ch);
    let sources = hadamard::verify_batch_shared_rows(&proof.hadamards, view.r1cs.k_log + 7, ch)
        .map_err(|e| format!("column R1CS Hadamard verify: {e:?}"))?;
    let mut other = Vec::with_capacity(4);
    for side in 0..2 {
        ch.observe_label(b"mikros-column-r1cs-linear-image-batch-v1");
        ch.observe_bytes(&(side as u64).to_le_bytes());
        let queries: Vec<_> = sources
            .iter()
            .map(|q| if side == 0 { q.a.clone() } else { q.b.clone() })
            .collect();
        let claim = multiple_image::verify_public_batch(
            circuit,
            side == 0,
            &queries,
            &proof.linchecks[side],
            ch,
        )?;
        other.push(witness_claim(claim)?);
    }
    other.extend(
        sources
            .iter()
            .enumerate()
            .map(|(group, q)| identity_claim(group, &q.c)),
    );
    Ok(Claims { other })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serial_deinterleave_matches_parallel_and_direct_indices() {
        for n in [0, 1, 2, 31, 32, 4096, 32768] {
            let words: Vec<_> = (0..2 * n)
                .map(|i| F128::new(i as u64, !(i as u64)))
                .collect();
            let expected: [Vec<F128>; 2] =
                std::array::from_fn(|group| (0..n).map(|i| words[2 * i + group]).collect());
            assert_eq!(deinterleave(&words, false), expected);
            assert_eq!(deinterleave(&words, true), expected);
        }
    }
}
