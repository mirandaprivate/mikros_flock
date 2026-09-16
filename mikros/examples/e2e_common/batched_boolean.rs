//! Two groups of 128 SHA executions, reduced in F128 before paired PCS opening.
//!
//! A/B are group-major for weighted Hadamard. The committed Z carrier instead
//! interleaves the groups: `z[2 * constraint + group]`. Its bit columns are the
//! 128 instance lanes. The caller must bind the commitment and public statement
//! before entry, authenticate EVERY returned claim, and check public I/O and
//! the SHA constant pin separately. This module proves only the Boolean R1CS.

use super::{PreparedWitness, TreeFixture, TreeVerifier};
use flock_core::{
    challenger::Challenger,
    lincheck::LincheckCircuit,
    pcs::functional::{TensorClaim, row_weights},
};
use flock_field::F128;
use mikros::{MatrixClaim, packed_ip::extension, weighted_hadamard as wh};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

pub struct TransposedWitness {
    /// Interleaved PCS carrier: `z[2 * constraint + group]`.
    pub z: Vec<F128>,
    /// Group-major WH inputs: `[group * K + constraint]`.
    pub a: Vec<F128>,
    pub b: Vec<F128>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    pub target_values: [F128; 2],
    pub hadamard: wh::Proof,
    pub linchecks: [extension::Proof; 2],
}

pub struct Claims {
    /// Same constraint point for both F128 coordinates of the F256 opening.
    pub target_point: Vec<F128>,
    pub target_values: [F128; 2],
    /// Two bit-linear claims on the interleaved Z carrier.
    pub other: Vec<TensorClaim>,
}

fn dimensions(nu: usize, k_log: usize) -> Result<usize, String> {
    if nu != 8 || !(7..=24).contains(&k_log) {
        return Err("two-group SHA reduction requires exactly 256 instance slots".into());
    }
    Ok(1usize << k_log)
}

/// Offline bit transpose from FLOCK's batch-major layout. No challenge is used.
pub fn prepare(
    witness: &PreparedWitness,
    nu: usize,
    k_log: usize,
) -> Result<TransposedWitness, String> {
    let k = dimensions(nu, k_log)?;
    if [&witness.0, &witness.1, &witness.2]
        .iter()
        .any(|words| words.len() != 2 * k)
    {
        return Err("two-group SHA source witness length".into());
    }
    let transpose = |input: &[F128], interleaved: bool| {
        (0..2 * k)
            .into_par_iter()
            .map(|output| {
                let (group, constraint) = if interleaved {
                    (output & 1, output >> 1)
                } else {
                    (output / k, output % k)
                };
                let chunk = constraint / 128;
                let bit = constraint % 128;
                let mut out = F128::ZERO;
                for lane in 0..128 {
                    let word = input[chunk * 256 + group * 128 + lane];
                    let bit = if bit < 64 {
                        word.lo >> bit
                    } else {
                        word.hi >> (bit - 64)
                    } & 1;
                    if lane < 64 {
                        out.lo |= bit << lane;
                    } else {
                        out.hi |= bit << (lane - 64);
                    }
                }
                out
            })
            .collect::<Vec<_>>()
    };
    Ok(TransposedWitness {
        z: transpose(&witness.0, true),
        a: transpose(&witness.1, false),
        b: transpose(&witness.2, false),
    })
}

fn basis() -> Vec<F128> {
    (0..128)
        .map(|i| {
            if i < 64 {
                F128::new(1 << i, 0)
            } else {
                F128::new(0, 1 << (i - 64))
            }
        })
        .collect()
}

fn shape(view: &TreeVerifier<'_>) -> Result<usize, String> {
    let k = dimensions(view.nu, view.r1cs.k_log)?;
    if view.live_rows != 256
        || view.r1cs.csc_lincheck_circuit().n_cols() != k
        || !view.r1cs.c0_is_identity()
    {
        return Err("two-group SHA reduction requires 256 live executions".into());
    }
    Ok(k)
}

fn start<C: Challenger>(view: &TreeVerifier<'_>, ch: &mut C) -> ([F128; 2], Vec<F128>) {
    ch.observe_label(b"mikros-sha256-two-packed-groups-boolean-v1");
    ch.observe_bytes(&256u64.to_le_bytes());
    ch.observe_bytes(&(view.r1cs.k_log as u64).to_le_bytes());
    let groups = [ch.sample_f128(), ch.sample_f128()];
    let point = ch.sample_f128_vec(view.r1cs.k_log);
    (groups, point)
}

fn input_factors(groups: [F128; 2], point: &[F128]) -> Vec<[F128; 2]> {
    std::iter::once(groups)
        .chain(point.iter().rev().map(|&x| [F128::ONE + x, x]))
        .collect()
}

fn observe_target<C: Challenger>(values: [F128; 2], ch: &mut C) {
    ch.observe_label(b"mikros-sha256-two-group-targets-v1");
    ch.observe_f128_slice(&values);
}

fn bytes(words: &[F128]) -> &[u8] {
    assert!(cfg!(target_endian = "little"));
    // F128 has the repository's repr(C), lo-then-hi packed-bit representation.
    unsafe { std::slice::from_raw_parts(words.as_ptr().cast(), std::mem::size_of_val(words)) }
}

fn public_projection(
    circuit: &dyn LincheckCircuit,
    query: &MatrixClaim,
    left: bool,
) -> Result<Vec<F128>, String> {
    let log_k = circuit.n_cols().ilog2() as usize;
    if query.column_weights.len() != 128 || query.row_factors.len() != log_k + 1 {
        return Err("transposed lincheck source query shape".into());
    }
    let low_first: Vec<_> = query.row_factors[1..].iter().rev().copied().collect();
    let weights = row_weights(&low_first);
    let (a, b) = circuit.fold_split(&weights);
    Ok(if left { a } else { b })
}

fn byte_tables(columns: &[F128]) -> Vec<[F128; 256]> {
    columns
        .chunks_exact(8)
        .map(|weights| {
            let mut table = [F128::ZERO; 256];
            for mask in 1usize..256 {
                let bit = mask.trailing_zeros() as usize;
                table[mask] = table[mask & (mask - 1)] + weights[bit];
            }
            table
        })
        .collect()
}

fn word_dot(word: F128, tables: &[[F128; 256]]) -> F128 {
    let mut value = F128::ZERO;
    for (i, byte) in word
        .lo
        .to_le_bytes()
        .into_iter()
        .chain(word.hi.to_le_bytes())
        .enumerate()
    {
        value += tables[i][byte as usize];
    }
    value
}

fn private_projection(z: &[F128], query: &MatrixClaim) -> Vec<F128> {
    let tables = byte_tables(&query.column_weights);
    let groups = query.row_factors[0];
    z.par_chunks_exact(2)
        .map(|words| {
            groups[0] * word_dot(words[0], &tables) + groups[1] * word_dot(words[1], &tables)
        })
        .collect()
}

/// Monomial weights factor as product_i [1,t^(2^i)][bit_i]. They must not be
/// replaced by normalized MLE weights [1+t^(2^i),t^(2^i)].
fn monomial_factors(columns: &[F128]) -> Result<Vec<[F128; 2]>, String> {
    if columns.len() != 128 || columns[0] != F128::ONE {
        return Err("packed IP terminal is not a 128-term monomial vector".into());
    }
    let t = columns[1];
    let mut expected = F128::ONE;
    for &value in columns {
        if value != expected {
            return Err("packed IP terminal monomial mismatch".into());
        }
        expected *= t;
    }
    let mut power = t;
    Ok((0..7)
        .map(|_| {
            let pair = [F128::ONE, power];
            power = power.square();
            pair
        })
        .collect())
}

fn terminal_query(source: &MatrixClaim, terminal: &MatrixClaim) -> Result<TensorClaim, String> {
    let mut rows = vec![source.row_factors[0]];
    rows.extend(monomial_factors(&terminal.column_weights)?);
    rows.extend(terminal.row_factors.iter().rev().copied());
    if rows.len() != source.row_factors.len() {
        return Err("transposed lincheck terminal dimension mismatch".into());
    }
    Ok(TensorClaim {
        columns: source.column_weights.clone(),
        rows,
        value: terminal.value,
    })
}

fn lincheck_prove<C: Challenger>(
    circuit: &dyn LincheckCircuit,
    z: &[F128],
    sources: &[MatrixClaim; 2],
    ch: &mut C,
) -> Result<([extension::Proof; 2], Vec<TensorClaim>), String> {
    let mut proofs = Vec::with_capacity(2);
    let mut claims = Vec::with_capacity(2);
    for (side, source) in sources.iter().enumerate() {
        ch.observe_label(if side == 0 {
            b"transposed-A0"
        } else {
            b"transposed-B0"
        });
        let public = public_projection(circuit, source, side == 0)?;
        let private = private_projection(z, source);
        let (proof, terminal) = extension::prove(
            &public,
            &private,
            circuit.n_cols().ilog2() as usize,
            source.value,
            ch,
        )
        .map_err(|e| format!("two-group SHA lincheck: {e:?}"))?;
        claims.push(terminal_query(source, &terminal.b)?);
        proofs.push(proof);
    }
    Ok((
        proofs.try_into().map_err(|_| "two lincheck proofs")?,
        claims,
    ))
}

fn lincheck_verify<C: Challenger>(
    circuit: &dyn LincheckCircuit,
    sources: &[MatrixClaim; 2],
    proofs: &[extension::Proof; 2],
    ch: &mut C,
) -> Result<Vec<TensorClaim>, String> {
    let mut claims = Vec::with_capacity(2);
    for (side, (source, proof)) in sources.iter().zip(proofs).enumerate() {
        ch.observe_label(if side == 0 {
            b"transposed-A0"
        } else {
            b"transposed-B0"
        });
        let public = public_projection(circuit, source, side == 0)?;
        let terminal =
            extension::verify(proof, circuit.n_cols().ilog2() as usize, source.value, ch)
                .map_err(|e| format!("two-group SHA lincheck verify: {e:?}"))?;
        if extension::evaluate(&public, &terminal.a)
            .map_err(|e| format!("matrix evaluation: {e:?}"))?
            != terminal.a.value
        {
            return Err("two-group SHA public matrix terminal rejected".into());
        }
        claims.push(terminal_query(source, &terminal.b)?);
    }
    Ok(claims)
}

pub fn prove<C: Challenger>(
    fixture: &TreeFixture,
    transposed: &TransposedWitness,
    ch: &mut C,
) -> Result<(Proof, Claims), String> {
    let view = fixture.verifier();
    let k = shape(&view)?;
    if [&transposed.z, &transposed.a, &transposed.b]
        .iter()
        .any(|words| words.len() != 2 * k)
    {
        return Err("two-group SHA transposed input length".into());
    }
    let (groups, point) = start(&view, ch);
    let weights = row_weights(
        &point
            .iter()
            .map(|&x| [F128::ONE + x, x])
            .collect::<Vec<_>>(),
    );
    let values = transposed
        .z
        .par_chunks_exact(2)
        .zip(&weights)
        .map(|(words, &weight)| [words[0] * weight, words[1] * weight])
        .reduce(|| [F128::ZERO; 2], |a, b| [a[0] + b[0], a[1] + b[1]]);
    observe_target(values, ch);
    let value = groups[0] * values[0] + groups[1] * values[1];
    let (hadamard, sources) = wh::prove(
        bytes(&transposed.a),
        bytes(&transposed.b),
        view.r1cs.k_log + 8,
        &input_factors(groups, &point),
        &basis(),
        value,
        mikros::Kernel::PairAggregate,
        ch,
    )
    .map_err(|e| format!("two-group SHA weighted Hadamard: {e:?}"))?;
    let (linchecks, other) = lincheck_prove(
        view.r1cs.csc_lincheck_circuit(),
        &transposed.z,
        &sources,
        ch,
    )?;
    Ok((
        Proof {
            target_values: values,
            hadamard,
            linchecks,
        },
        Claims {
            target_point: point,
            target_values: values,
            other,
        },
    ))
}

pub fn verify<C: Challenger>(
    view: &TreeVerifier<'_>,
    proof: &Proof,
    ch: &mut C,
) -> Result<Claims, String> {
    shape(view)?;
    let (groups, point) = start(view, ch);
    observe_target(proof.target_values, ch);
    let value = groups[0] * proof.target_values[0] + groups[1] * proof.target_values[1];
    let sources = wh::verify(
        &proof.hadamard,
        view.r1cs.k_log + 8,
        &input_factors(groups, &point),
        &basis(),
        value,
        ch,
    )
    .map_err(|e| format!("two-group SHA weighted Hadamard verify: {e:?}"))?;
    let other = lincheck_verify(
        view.r1cs.csc_lincheck_circuit(),
        &sources,
        &proof.linchecks,
        ch,
    )?;
    Ok(Claims {
        target_point: point,
        target_values: proof.target_values,
        other,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_transcript::challenger::FsChallenger;

    fn evaluate(z: &[F128], query: &TensorClaim) -> F128 {
        let table = byte_tables(&query.columns);
        z.iter()
            .zip(row_weights(&query.rows))
            .fold(F128::ZERO, |sum, (&word, weight)| {
                sum + weight * word_dot(word, &table)
            })
    }

    struct ShiftCircuit(usize);
    impl LincheckCircuit for ShiftCircuit {
        fn n_cols(&self) -> usize {
            self.0
        }
        fn fold_alpha_batched(&self, alpha: F128, weights: &[F128]) -> Vec<F128> {
            let (a, b) = self.fold_split(weights);
            a.into_iter().zip(b).map(|(a, b)| alpha * a + b).collect()
        }
        fn fold_split(&self, weights: &[F128]) -> (Vec<F128>, Vec<F128>) {
            (
                (0..self.0)
                    .map(|i| weights[(i + self.0 - 1) % self.0])
                    .collect(),
                (0..self.0).map(|i| weights[i ^ 1]).collect(),
            )
        }
    }

    #[test]
    fn transposed_lincheck_authenticates_both_sources_and_replays_transcript() {
        let circuit = ShiftCircuit(256);
        let z: Vec<_> = (0..512)
            .map(|i| {
                F128::new(
                    (i as u64).wrapping_mul(0xa70123ab),
                    !(i as u64).wrapping_mul(0xb302349e),
                )
            })
            .collect();
        let sources: [MatrixClaim; 2] = std::array::from_fn(|side| {
            let columns: Vec<_> = (0..128)
                .map(|i| F128::new(i as u64 + 1, (i as u64 + 9) * 17))
                .collect();
            let mut factors: Vec<_> = (0..9)
                .map(|i| [F128::new(i + 1 + side as u64, 0), F128::new(i + 5, 3)])
                .collect();
            factors[2] = [F128::new(3, 2); 2];
            let query = TensorClaim {
                columns: columns.clone(),
                rows: std::iter::once(factors[0])
                    .chain(factors[1..].iter().rev().copied())
                    .collect(),
                value: F128::ZERO,
            };
            let mapped: Vec<_> = (0..512)
                .map(|i| {
                    let constraint = i / 2;
                    let source = if side == 0 {
                        (constraint + 1) % 256
                    } else {
                        constraint ^ 1
                    };
                    z[2 * source + (i & 1)]
                })
                .collect();
            MatrixClaim {
                column_weights: columns,
                row_factors: factors,
                value: evaluate(&mapped, &query),
            }
        });
        let mut pch = FsChallenger::new(b"transposed-lincheck-test");
        let (proofs, claims) = lincheck_prove(&circuit, &z, &sources, &mut pch).unwrap();
        for claim in &claims {
            assert_eq!(evaluate(&z, claim), claim.value);
        }
        let mut vch = FsChallenger::new(b"transposed-lincheck-test");
        let replay = lincheck_verify(&circuit, &sources, &proofs, &mut vch).unwrap();
        assert_eq!(replay, claims);
        assert_eq!(pch.sample_f128(), vch.sample_f128());
        for side in 0..2 {
            let mut tampered = proofs.clone();
            tampered[side].h_plus[7] += F128::ONE;
            let rejected = match lincheck_verify(
                &circuit,
                &sources,
                &tampered,
                &mut FsChallenger::new(b"transposed-lincheck-test"),
            ) {
                Err(_) => true,
                Ok(claims) => claims
                    .iter()
                    .any(|claim| evaluate(&z, claim) != claim.value),
            };
            assert!(rejected);
        }
    }

    #[test]
    fn transpose_matches_every_coordinate_and_both_group_layouts() {
        let k_log = 8;
        let k = 1 << k_log;
        let input: Vec<_> = (0..2 * k)
            .map(|i| {
                F128::new(
                    (i as u64).wrapping_mul(0x123456789abcdef),
                    !(i as u64).wrapping_mul(0xfedcba987654321),
                )
            })
            .collect();
        let t = prepare(
            &(input.clone(), input.clone(), input.clone(), vec![]),
            8,
            k_log,
        )
        .unwrap();
        for constraint in 0..k {
            for instance in 0..256 {
                let group = instance / 128;
                let lane = instance % 128;
                let source = input[(constraint / 128) * 256 + instance];
                let bit = constraint % 128;
                let expected = if bit < 64 {
                    source.lo >> bit
                } else {
                    source.hi >> (bit - 64)
                } & 1;
                for word in [
                    t.z[2 * constraint + group],
                    t.a[group * k + constraint],
                    t.b[group * k + constraint],
                ] {
                    assert_eq!(
                        if lane < 64 {
                            word.lo >> lane
                        } else {
                            word.hi >> (lane - 64)
                        } & 1,
                        expected
                    );
                }
            }
        }
        assert!(prepare(&(input.clone(), input.clone(), input, vec![]), 7, k_log).is_err());
    }

    #[test]
    fn monomial_tensor_preserves_zero_and_zero_sum_pairs() {
        for t in [F128::ZERO, F128::ONE, F128::new(5, 7)] {
            let mut power = F128::ONE;
            let columns: Vec<_> = (0..128)
                .map(|_| {
                    let out = power;
                    power *= t;
                    out
                })
                .collect();
            assert_eq!(row_weights(&monomial_factors(&columns).unwrap()), columns);
            let mut bad = columns;
            bad[63] += F128::ONE;
            assert!(monomial_factors(&bad).is_err());
        }
    }
}
