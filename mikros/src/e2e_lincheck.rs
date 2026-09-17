//! Linear reduction for a single repeated binary R1CS block in BatchMajor order.
//!
//! The natural packed-word address is `constraint_chunk * 2^n_log + instance`.
//! Each Hadamard matrix query separates into constraint weights and instance
//! factors. Public A0/B0 transposes push the former to the witness columns; two
//! Laurent IPs over F128 reduce the resulting claims to the committed Z oracle.
//! This uses [`crate::packed_ip::extension`], not the binary coefficient kernel.
//!
//! The caller must bind the public circuit, counts, statement and full padded Z
//! commitment before Hadamard/lincheck, and authenticate every returned claim.
//! `LincheckCircuit` is public verifier data. The enclosing circuit proof handles
//! copy constraints and public I/O.

use flock_core::{
    lincheck::{LincheckCircuit, eq_prefix_sum},
    pcs::{functional::TensorClaim, ring_switch::fold_1b_rows_naive},
};
use flock_field::F128;
use flock_transcript::challenger::Challenger;
use rayon::prelude::{ParallelIterator, ParallelSlice};
use serde::{Deserialize, Serialize};

use crate::{Error, MatrixClaim, packed_ip::extension};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    pub a: extension::Proof,
    pub b: extension::Proof,
}

fn dimensions(
    circuit: &dyn LincheckCircuit,
    n_log: usize,
    live_rows: usize,
    a: &MatrixClaim,
    b: &MatrixClaim,
) -> Result<(usize, usize), Error> {
    let k = circuit.n_cols();
    if !k.is_power_of_two() || k < 128 || n_log >= usize::BITS as usize {
        return Err(Error::ProofShape("lincheck dimensions"));
    }
    let k_log = k.ilog2() as usize;
    if k_log
        .checked_add(n_log)
        .is_none_or(|m| m >= usize::BITS as usize)
        || live_rows > 1usize << n_log
        || circuit.const_pin_col().is_some_and(|column| column >= k)
        || [a, b]
            .iter()
            .any(|q| q.column_weights.len() != 128 || q.row_factors.len() != k_log - 7 + n_log)
    {
        return Err(Error::ProofShape("lincheck query"));
    }
    Ok((k_log, 1usize << n_log))
}

fn bind<C: Challenger>(
    circuit: &dyn LincheckCircuit,
    n_log: usize,
    live_rows: usize,
    a: &MatrixClaim,
    b: &MatrixClaim,
    ch: &mut C,
) {
    ch.observe_label(b"mikros-single-block-batch-major-lincheck-v1");
    ch.observe_bytes(&(circuit.n_cols() as u64).to_le_bytes());
    ch.observe_bytes(&(n_log as u64).to_le_bytes());
    ch.observe_bytes(&(live_rows as u64).to_le_bytes());
    let pin = circuit.const_pin_col();
    ch.observe_bytes(&[u8::from(pin.is_some())]);
    ch.observe_bytes(&(pin.unwrap_or(0) as u64).to_le_bytes());
    for query in [a, b] {
        ch.observe_f128_slice(&query.column_weights);
        for pair in &query.row_factors {
            ch.observe_f128_slice(pair);
        }
        ch.observe_f128(query.value);
    }
}

fn tensor_high(factors: &[[F128; 2]]) -> Vec<F128> {
    let mut weights = vec![F128::ONE];
    for &[a, b] in factors {
        weights = weights.into_iter().flat_map(|w| [w * a, w * b]).collect();
    }
    weights
}

fn push_query(
    circuit: &dyn LincheckCircuit,
    query: &MatrixClaim,
    k_log: usize,
    a: bool,
) -> Vec<F128> {
    let chunks = tensor_high(&query.row_factors[..k_log - 7]);
    let inner: Vec<_> = chunks
        .iter()
        .flat_map(|&chunk| {
            query
                .column_weights
                .iter()
                .map(move |&column| chunk * column)
        })
        .collect();
    // The public interface accepts an arbitrary vector despite its eq_inner
    // name. Its operation is a linear matrix transpose, with no MLE assumption.
    let (left, right) = circuit.fold_split(&inner);
    if a { left } else { right }
}

fn partial_fold(z: &[F128], instance_factors: &[[F128; 2]]) -> Vec<F128> {
    let weights = tensor_high(instance_factors);
    z.par_chunks_exact(weights.len())
        .flat_map_iter(|chunk| fold_1b_rows_naive(chunk, &weights))
        .collect()
}

fn z_query(terminal: MatrixClaim, instance_factors: &[[F128; 2]]) -> TensorClaim {
    // PCS factors are low-bit first: instance variables precede chunk variables
    // in the BatchMajor packed-word address. Both sources here are high-first.
    let rows = instance_factors
        .iter()
        .rev()
        .copied()
        .chain(terminal.row_factors.iter().rev().copied())
        .collect();
    TensorClaim {
        columns: terminal.column_weights,
        rows,
        value: terminal.value,
    }
}

fn pin_claim<C: Challenger>(
    circuit: &dyn LincheckCircuit,
    k_log: usize,
    n_log: usize,
    live_rows: usize,
    ch: &mut C,
) -> Option<TensorClaim> {
    circuit.const_pin_col().map(|column| {
        ch.observe_label(b"mikros-single-block-live-constant-pin-v1");
        let point = ch.sample_f128_vec(n_log);
        let mut columns = vec![F128::ZERO; 128];
        columns[column & 127] = F128::ONE;
        let mut rows: Vec<_> = point.iter().map(|&x| [F128::ONE + x, x]).collect();
        rows.extend((0..k_log - 7).map(|i| {
            if (column >> (7 + i)) & 1 == 0 {
                [F128::ONE, F128::ZERO]
            } else {
                [F128::ZERO, F128::ONE]
            }
        }));
        TensorClaim {
            columns,
            rows,
            value: eq_prefix_sum(&point, live_rows),
        }
    })
}

/// Reduce both verified Hadamard source queries to the same committed Z.
/// Returned order: A-derived query, B-derived query, optional constant pin.
pub fn prove<C: Challenger>(
    circuit: &dyn LincheckCircuit,
    z_packed: &[F128],
    n_log: usize,
    live_rows: usize,
    a: &MatrixClaim,
    b: &MatrixClaim,
    ch: &mut C,
) -> Result<(Proof, Vec<TensorClaim>), Error> {
    let (k_log, instances) = dimensions(circuit, n_log, live_rows, a, b)?;
    let expected = (circuit.n_cols() / 128) * instances;
    if z_packed.len() != expected {
        return Err(Error::InputLength {
            input: "z_packed",
            expected,
            got: z_packed.len(),
        });
    }
    bind(circuit, n_log, live_rows, a, b, ch);
    let mut proofs = Vec::with_capacity(2);
    let mut queries = Vec::with_capacity(3);
    for (is_a, query) in [(true, a), (false, b)] {
        ch.observe_label(if is_a { b"A0" } else { b"B0" });
        let public = push_query(circuit, query, k_log, is_a);
        let factors = &query.row_factors[k_log - 7..];
        let private = partial_fold(z_packed, factors);
        let (proof, claims) = extension::prove(&public, &private, k_log, query.value, ch)?;
        queries.push(z_query(claims.b, factors));
        proofs.push(proof);
    }
    queries.extend(pin_claim(circuit, k_log, n_log, live_rows, ch));
    let mut proofs = proofs.into_iter();
    Ok((
        Proof {
            a: proofs.next().expect("two proofs"),
            b: proofs.next().expect("two proofs"),
        },
        queries,
    ))
}

/// Verifier input includes only public matrices, public shapes and verified
/// upstream queries. All returned Z queries must be passed to the PCS verifier.
pub fn verify<C: Challenger>(
    circuit: &dyn LincheckCircuit,
    n_log: usize,
    live_rows: usize,
    a: &MatrixClaim,
    b: &MatrixClaim,
    proof: &Proof,
    ch: &mut C,
) -> Result<Vec<TensorClaim>, Error> {
    let (k_log, _) = dimensions(circuit, n_log, live_rows, a, b)?;
    bind(circuit, n_log, live_rows, a, b, ch);
    let mut queries = Vec::with_capacity(3);
    for (is_a, query, proof) in [(true, a, &proof.a), (false, b, &proof.b)] {
        ch.observe_label(if is_a { b"A0" } else { b"B0" });
        let public = push_query(circuit, query, k_log, is_a);
        let claims = extension::verify(proof, k_log, query.value, ch)?;
        if extension::evaluate(&public, &claims.a)? != claims.a.value {
            return Err(Error::SourceMismatch("public matrix"));
        }
        queries.push(z_query(claims.b, &query.row_factors[k_log - 7..]));
    }
    queries.extend(pin_claim(circuit, k_log, n_log, live_rows, ch));
    Ok(queries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::{lincheck::CscCircuit, r1cs::SparseBinaryMatrix};
    use flock_field::test_rng::Rng;
    use flock_hash::HashKind;
    use flock_transcript::challenger::FsChallenger;

    fn bit(z: &[F128], position: usize) -> bool {
        let word = z[position / 128];
        let bits = u128::from(word.lo) | (u128::from(word.hi) << 64);
        bits >> (position & 127) & 1 == 1
    }
    fn set(z: &mut [F128], position: usize) {
        if position & 127 < 64 {
            z[position / 128].lo |= 1 << (position & 63);
        } else {
            z[position / 128].hi |= 1 << (position & 63);
        }
    }
    fn evaluate(z: &[F128], query: &TensorClaim) -> F128 {
        let mut total = F128::ZERO;
        for word in 0..z.len() {
            let weight = query
                .rows
                .iter()
                .enumerate()
                .fold(F128::ONE, |w, (i, pair)| w * pair[(word >> i) & 1]);
            for column in 0..128 {
                if bit(z, word * 128 + column) {
                    total += weight * query.columns[column];
                }
            }
        }
        total
    }
    fn apply(matrix: &SparseBinaryMatrix, z: &[F128], n_log: usize) -> Vec<F128> {
        let mut output = vec![F128::ZERO; z.len()];
        let instances = 1 << n_log;
        for row in 0..matrix.num_rows {
            for instance in 0..instances {
                let value = matrix.rows[row].iter().fold(false, |v, &col| {
                    v ^ bit(z, ((col / 128) * instances + instance) * 128 + col % 128)
                });
                if value {
                    set(
                        &mut output,
                        ((row / 128) * instances + instance) * 128 + row % 128,
                    );
                }
            }
        }
        output
    }
    fn matrix_query(values: &[F128], rng: &mut Rng) -> MatrixClaim {
        let columns = rng.f128_vec(128);
        let mut rows: Vec<_> = (0..values.len().ilog2())
            .map(|_| [rng.f128(), rng.f128()])
            .collect();
        // Vanishing normalizer: preserving pairs must work without division.
        rows[0][1] = rows[0][0];
        let value = evaluate(
            values,
            &TensorClaim {
                columns: columns.clone(),
                rows: rows.clone(),
                value: F128::ZERO,
            },
        );
        rows.reverse();
        MatrixClaim {
            column_weights: columns,
            row_factors: rows,
            value,
        }
    }

    #[test]
    fn e2e_lincheck_two_different_general_tensors_match_batch_major_sources() {
        for k in [128, 512] {
            let (n_log, live, pin) = (3, 5, if k == 128 { 3 } else { 259 });
            let a = SparseBinaryMatrix::new(k, k, (0..k).map(|i| vec![i, (i + 13) % k]).collect());
            let b = SparseBinaryMatrix::new(
                k,
                k,
                (0..k)
                    .map(|i| vec![(i * 3) % k, (i + 5) % k, (i + 65) % k])
                    .collect(),
            );
            let circuit = CscCircuit::from_matrices(&a, &b).with_const_pin(Some(pin));
            let mut rng = Rng::new(0xE2E_C0DE);
            let mut z = vec![F128::ZERO; (k / 128) << n_log];
            for column in 0..k {
                for instance in 0..live {
                    if column == pin || rng.next_u64() & 1 == 1 {
                        set(
                            &mut z,
                            (((column / 128) << n_log) + instance) * 128 + column % 128,
                        );
                    }
                }
            }
            let qa = matrix_query(&apply(&a, &z, n_log), &mut rng);
            let qb = matrix_query(&apply(&b, &z, n_log), &mut rng);
            assert_ne!(qa.row_factors, qb.row_factors);
            let ch = || FsChallenger::with_hash(b"Mikros-e2e-lincheck-test", HashKind::Blake3);
            let mut p = ch();
            let (proof, claims) = prove(&circuit, &z, n_log, live, &qa, &qb, &mut p).unwrap();
            let mut v = ch();
            assert_eq!(
                verify(&circuit, n_log, live, &qa, &qb, &proof, &mut v).unwrap(),
                claims
            );
            assert_eq!(p.sample_f128(), v.sample_f128());
            assert_eq!(claims.len(), 3);
            for claim in &claims {
                assert_eq!(evaluate(&z, claim), claim.value);
            }
            let mut bad = proof.clone();
            bad.a.h_plus[0] += F128::ONE;
            assert!(verify(&circuit, n_log, live, &qa, &qb, &bad, &mut ch()).is_err());
            assert!(verify(&circuit, n_log, live + 1, &qa, &qb, &proof, &mut ch()).is_err());
            assert!(verify(&circuit, n_log + 1, live, &qa, &qb, &proof, &mut ch()).is_err());
            assert!(
                prove(
                    &circuit,
                    &z[..z.len() - 1],
                    n_log,
                    live,
                    &qa,
                    &qb,
                    &mut ch()
                )
                .is_err()
            );
            let mut bad = qa.clone();
            bad.value += F128::ONE;
            assert!(verify(&circuit, n_log, live, &bad, &qb, &proof, &mut ch()).is_err());
            assert!(prove(&circuit, &z, n_log, live, &bad, &qb, &mut ch()).is_err());
            // The PIOP defers the pin to PCS; changing the actual constant bit
            // must fail that returned query even when no witness is in verifier.
            let mut bad_z = z;
            bad_z[(pin / 128) << n_log].lo ^= 1 << (pin & 63);
            assert_ne!(evaluate(&bad_z, &claims[2]), claims[2].value);
        }
    }
}
