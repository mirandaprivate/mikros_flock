//! Public binary-matrix image checks for packed elements of K=GF(2^b).
//!
//! Each group transmits kappa=128/b extension-valued K-column evaluations.
//! A K-linear coordinate projection authenticates these messages with one
//! public CSC fold and one IP. Raw b-bit storage is mapped by the fixed F2-linear
//! map Psi into an E/K basis before the IP. Its endpoint is therefore a
//! bit-linear PCS query, not an ordinary MLE of the unconverted raw words.
//! Bind the circuit and commitments first, and authenticate the returned claim.

use crate::{
    MatrixClaim,
    inner_product::{self, InnerProductProof, ProverScratch},
    multiple_image::{self, PointClaim},
};
use flock_core::{
    lincheck::LincheckCircuit,
    pcs::{
        functional::row_weights,
        ring_switch::{build_fold_byte_table, fold_1b_rows_naive},
    },
};
use flock_field::{F128, F256Unreduced};
use flock_transcript::challenger::Challenger;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

fn raw(value: F128) -> u128 {
    u128::from(value.lo) | (u128::from(value.hi) << 64)
}
fn from_raw(value: u128) -> F128 {
    F128::new(value as u64, (value >> 64) as u64)
}

#[inline]
fn map_word(value: F128, table: &[F128]) -> F128 {
    value
        .lo
        .to_le_bytes()
        .into_iter()
        .chain(value.hi.to_le_bytes())
        .enumerate()
        .fold(F128::ZERO, |sum, (i, byte)| {
            sum + table[256 * i + usize::from(byte)]
        })
}

/// Witness-independent coordinate maps. Construct this during public setup.
#[derive(Clone)]
pub struct FieldPacking {
    bits: usize,
    subfield_basis: Vec<F128>,
    gamma: Vec<F128>,
    columns: Vec<F128>,
    inverse_columns: Vec<u128>,
    packing_table: Vec<F128>,
}

impl FieldPacking {
    /// `subfield_basis[h]` is the existing embedding of the polynomial-basis
    /// K coordinate 2^h. This validates the exact degree and all 128 map columns.
    pub fn new(bits: usize, subfield_basis: &[F128]) -> Result<Self, String> {
        if ![1, 2, 4, 8, 16, 32, 64, 128].contains(&bits)
            || subfield_basis.len() != bits
            || subfield_basis[0] != F128::ONE
        {
            return Err("field image subfield basis shape".into());
        }
        if bits > 1 {
            let generator = subfield_basis[1];
            let mut power = F128::ONE;
            for &element in subfield_basis {
                if element != power {
                    return Err("field image requires the polynomial subfield basis".into());
                }
                power *= generator;
            }
            let mut frobenius = generator;
            for i in 1..=bits {
                frobenius = frobenius.square();
                if i == bits / 2 && frobenius == generator {
                    return Err("field image subfield degree too small".into());
                }
            }
            if frobenius != generator {
                return Err("field image generator is outside the claimed subfield".into());
            }
        }
        // x generates E/F2, hence also E/K; its first kappa powers form an E/K basis.
        let gamma: Vec<_> = (0..128 / bits).map(|j| from_raw(1u128 << j)).collect();
        let columns: Vec<_> = gamma
            .iter()
            .flat_map(|&g| subfield_basis.iter().map(move |&b| g * b))
            .collect();
        let inverse_columns = invert_binary_columns(&columns)?;
        let packing_table = build_fold_byte_table(&columns);
        Ok(Self {
            bits,
            subfield_basis: subfield_basis.to_vec(),
            gamma,
            columns,
            inverse_columns,
            packing_table,
        })
    }

    pub fn value_bits(&self) -> usize {
        self.bits
    }
    pub fn kappa(&self) -> usize {
        self.gamma.len()
    }
    /// Raw-word bit coefficients for the IP endpoint's PCS query.
    pub fn columns(&self) -> &[F128] {
        &self.columns
    }

    /// Apply Psi to one raw packed word, without changing its K coordinates.
    pub fn pack_word(&self, word: F128) -> F128 {
        if self.bits == 1 {
            word
        } else {
            map_word(word, &self.packing_table)
        }
    }

    fn field_columns(&self, words: &[F128], weights: &[F128]) -> Vec<F128> {
        // Reuse the SIMD bit transpose internally; only kappa K columns are sent.
        fold_1b_rows_naive(words, weights)
            .chunks_exact(self.bits)
            .map(|coordinates| dot(coordinates, &self.subfield_basis))
            .collect()
    }

    /// Phi(sum_j gamma_j a_j)=sum_j eta_j a_j, a_j in K. The inverse
    /// binary basis computes the same coordinates as the relative trace-dual basis.
    fn projection_table(&self, eta: &[F128]) -> Vec<F128> {
        debug_assert_eq!(eta.len(), self.kappa());
        let mapped: Vec<_> = eta
            .iter()
            .flat_map(|&g| self.subfield_basis.iter().map(move |&b| g * b))
            .collect();
        let images: Vec<_> = self
            .inverse_columns
            .iter()
            .map(|&coordinates| {
                let mut coordinates = coordinates;
                let mut value = F128::ZERO;
                while coordinates != 0 {
                    value += mapped[coordinates.trailing_zeros() as usize];
                    coordinates &= coordinates - 1;
                }
                value
            })
            .collect();
        build_fold_byte_table(&images)
    }

    fn projected_claim(&self, columns: &[F128], projection: &[F128]) -> F128 {
        columns
            .iter()
            .zip(&self.gamma)
            .fold(F256Unreduced::ZERO, |sum, (&column, &gamma)| {
                sum ^ gamma.mul_unreduced(map_word(column, projection))
            })
            .reduce()
    }
}

fn invert_binary_columns(columns: &[F128]) -> Result<Vec<u128>, String> {
    let mut rows: Vec<u128> = (0..128)
        .map(|r| {
            columns
                .iter()
                .enumerate()
                .fold(0, |row, (c, &value)| row | (((raw(value) >> r) & 1) << c))
        })
        .collect();
    let mut inverse: Vec<u128> = (0..128).map(|i| 1u128 << i).collect();
    for column in 0..128 {
        let pivot = (column..128)
            .find(|&r| (rows[r] >> column) & 1 != 0)
            .ok_or("field image packing basis has rank below 128")?;
        rows.swap(column, pivot);
        inverse.swap(column, pivot);
        let pivot_row = rows[column];
        let pivot_inverse = inverse[column];
        for row in 0..128 {
            if row != column && (rows[row] >> column) & 1 != 0 {
                rows[row] ^= pivot_row;
                inverse[row] ^= pivot_inverse;
            }
        }
    }
    Ok((0..128)
        .map(|column| {
            inverse.iter().enumerate().fold(0, |out, (row, &value)| {
                out | (((value >> column) & 1) << row)
            })
        })
        .collect())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicBatchProof {
    /// Exactly kappa actual K-column evaluations per group.
    pub columns: Vec<Vec<F128>>,
    pub ip: InnerProductProof,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublicBatchClaims {
    /// Low-first MLE of sum_g weights[g] * Psi(raw_witness[g]).
    pub witness: PointClaim,
    pub weights: Vec<F128>,
    /// The 128 F2-linear coefficients of Psi; authenticate against raw storage.
    pub columns: Vec<F128>,
}

fn dot(a: &[F128], b: &[F128]) -> F128 {
    debug_assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b)
        .fold(F256Unreduced::ZERO, |sum, (&a, &b)| {
            sum ^ a.mul_unreduced(b)
        })
        .reduce()
}

fn shape(packing: &FieldPacking, sources: &[MatrixClaim]) -> Result<(usize, usize), String> {
    let first = sources.first().ok_or("empty field image batch")?;
    let log_n = first.row_factors.len();
    if log_n >= usize::BITS as usize
        || sources.iter().any(|q| {
            q.row_factors != first.row_factors || q.column_weights.len() != packing.kappa()
        })
    {
        return Err("field image requires matching row tensors and kappa columns".into());
    }
    let n = 1usize << log_n;
    if n > isize::MAX as usize / size_of::<F128>() {
        return Err("field image dimension overflow".into());
    }
    Ok((log_n, n))
}

fn tensor_high(factors: &[[F128; 2]]) -> Vec<F128> {
    let mut values = Vec::new();
    crate::weights::fill_tensor(factors, &mut values);
    values
}

fn transcript<C: Challenger>(
    packing: &FieldPacking,
    left: bool,
    sources: &[MatrixClaim],
    columns: &[Vec<F128>],
    ch: &mut C,
) -> Result<(Vec<F128>, Vec<F128>, F128), String> {
    if columns.len() != sources.len() || columns.iter().any(|u| u.len() != packing.kappa()) {
        return Err("field image batch column message shape".into());
    }
    ch.observe_label(b"mikros-multiple-field-image-public-csc-v1");
    ch.observe_bytes(&(packing.bits as u64).to_le_bytes());
    ch.observe_f128_slice(&packing.subfield_basis);
    ch.observe_bytes(&[u8::from(left)]);
    ch.observe_bytes(&(sources.len() as u64).to_le_bytes());
    ch.observe_bytes(&(sources[0].row_factors.len() as u64).to_le_bytes());
    for (source, columns) in sources.iter().zip(columns) {
        ch.observe_f128_slice(&source.column_weights);
        for pair in &source.row_factors {
            ch.observe_f128_slice(pair);
        }
        ch.observe_f128(source.value);
        if dot(columns, &source.column_weights) != source.value {
            return Err("field image column scalar check".into());
        }
        ch.observe_f128_slice(columns);
    }
    // Every original group message is fixed before either random reduction.
    ch.observe_label(b"mikros-field-image-group-weights-v1");
    let weights = ch.sample_f128_vec(sources.len());
    ch.observe_label(b"mikros-field-image-K-linear-projection-v1");
    let eta = ch.sample_f128_vec(packing.kappa());
    let projection = packing.projection_table(&eta);
    let scalar = columns
        .iter()
        .zip(&weights)
        .fold(F128::ZERO, |sum, (columns, &z)| {
            // Phi is K-linear, not E-linear: project before multiplying by z.
            sum + z * packing.projected_claim(columns, &projection)
        });
    Ok((weights, projection, scalar))
}

fn public_marginal(
    circuit: &dyn LincheckCircuit,
    left: bool,
    xi: &mut [F128],
    projection: &[F128],
) -> Result<Vec<F128>, String> {
    xi.par_iter_mut()
        .for_each(|value| *value = map_word(*value, projection));
    let rhs = circuit.fold_side(xi, left);
    if rhs.len() != xi.len() {
        return Err("field image public matrix output shape".into());
    }
    Ok(rhs)
}

/// Public A/B must be binary matrices. Every group is one raw word per matrix
/// coordinate, containing kappa independent K elements. The returned endpoint
/// still requires packed-opening/PCS authentication against those original words.
pub fn prove_public_batch<C: Challenger>(
    circuit: &dyn LincheckCircuit,
    left: bool,
    packing: &FieldPacking,
    witnesses: &[&[F128]],
    images: &[&[F128]],
    sources: &[MatrixClaim],
    ch: &mut C,
) -> Result<(PublicBatchProof, PublicBatchClaims), String> {
    let (_, n) = shape(packing, sources)?;
    if circuit.n_cols() != n
        || witnesses.len() != sources.len()
        || images.len() != sources.len()
        || witnesses.iter().chain(images).any(|w| w.len() != n)
    {
        return Err("field image public batch input shape".into());
    }
    if packing.bits == 1 {
        let (proof, claims) =
            multiple_image::prove_public_batch(circuit, left, witnesses, images, sources, ch)?;
        return Ok((
            PublicBatchProof {
                columns: proof.columns,
                ip: proof.ip,
            },
            PublicBatchClaims {
                witness: claims.witness,
                weights: claims.weights,
                columns: packing.columns.clone(),
            },
        ));
    }
    let mut xi = tensor_high(&sources[0].row_factors);
    let columns: Vec<_> = images
        .iter()
        .map(|image| packing.field_columns(image, &xi))
        .collect();
    let (weights, projection, scalar) = transcript(packing, left, sources, &columns, ch)?;
    let rhs = public_marginal(circuit, left, &mut xi, &projection)?;
    let mut scratch = ProverScratch::with_fold_buffers([xi, Vec::new()]);
    scratch.a = (0..n)
        .into_par_iter()
        .map(|i| {
            witnesses
                .iter()
                .zip(&weights)
                .fold(F256Unreduced::ZERO, |sum, (w, &z)| {
                    sum ^ z.mul_unreduced(packing.pack_word(w[i]))
                })
                .reduce()
        })
        .collect();
    scratch.b = rhs;
    let (actual, initial) = inner_product::checked_initial_message_native(&scratch.a, &scratch.b);
    if actual != scalar {
        return Err("field image public projected identity".into());
    }
    ch.observe_label(b"mikros-field-image-public-batch-ip-v1");
    let (ip, mut point) = inner_product::prove_native(&mut scratch, initial, ch);
    point.reverse();
    let claims = PublicBatchClaims {
        witness: PointClaim {
            point,
            value: ip.final_a,
        },
        weights,
        columns: packing.columns.clone(),
    };
    Ok((PublicBatchProof { columns, ip }, claims))
}

pub fn verify_public_batch<C: Challenger>(
    circuit: &dyn LincheckCircuit,
    left: bool,
    packing: &FieldPacking,
    sources: &[MatrixClaim],
    proof: &PublicBatchProof,
    ch: &mut C,
) -> Result<PublicBatchClaims, String> {
    let (log_n, n) = shape(packing, sources)?;
    if circuit.n_cols() != n || proof.ip.rounds.len() != log_n {
        return Err("field image public proof shape".into());
    }
    if packing.bits == 1 {
        let legacy = multiple_image::PublicBatchProof {
            columns: proof.columns.clone(),
            ip: proof.ip.clone(),
        };
        let claims = multiple_image::verify_public_batch(circuit, left, sources, &legacy, ch)?;
        return Ok(PublicBatchClaims {
            witness: claims.witness,
            weights: claims.weights,
            columns: packing.columns.clone(),
        });
    }
    let (weights, projection, scalar) = transcript(packing, left, sources, &proof.columns, ch)?;
    ch.observe_label(b"mikros-field-image-public-batch-ip-v1");
    let mut point = inner_product::verify(&proof.ip, scalar, ch)
        .map_err(|e| format!("field image IP: {e:?}"))?;
    point.reverse();
    let mut xi = tensor_high(&sources[0].row_factors);
    let rhs = public_marginal(circuit, left, &mut xi, &projection)?;
    let factors: Vec<_> = point.iter().map(|&r| [F128::ONE + r, r]).collect();
    if dot(&row_weights(&factors), &rhs) != proof.ip.final_b {
        return Err("field image public matrix terminal".into());
    }
    Ok(PublicBatchClaims {
        witness: PointClaim {
            point,
            value: proof.ip.final_a,
        },
        weights,
        columns: packing.columns.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::{lincheck::CscCircuit, r1cs::SparseBinaryMatrix};
    use flock_field::test_rng::Rng;
    use flock_transcript::challenger::FsChallenger;
    use mikros_permutation::field::SmallField;

    fn plan(bits: usize) -> FieldPacking {
        FieldPacking::new(bits, SmallField::for_width(bits).unwrap().basis()).unwrap()
    }

    fn dense_matrices(n: usize) -> (CscCircuit, Vec<Vec<bool>>, Vec<Vec<bool>>) {
        let a: Vec<Vec<bool>> = (0..n)
            .map(|r| (0..n).map(|c| c == r || c == r + 1).collect())
            .collect();
        let b: Vec<Vec<bool>> = (0..n)
            .map(|r| (0..n).map(|c| c + 1 == r).collect())
            .collect();
        let sparse = |matrix: &[Vec<bool>]| {
            SparseBinaryMatrix::new(
                n,
                n,
                matrix
                    .iter()
                    .map(|row| {
                        row.iter()
                            .enumerate()
                            .filter_map(|(c, &bit)| bit.then_some(c))
                            .collect()
                    })
                    .collect(),
            )
        };
        (CscCircuit::from_matrices(&sparse(&a), &sparse(&b)), a, b)
    }

    fn coordinate(word: F128, bits: usize, j: usize) -> u128 {
        (raw(word) >> (j * bits))
            & if bits == 128 {
                u128::MAX
            } else {
                (1u128 << bits) - 1
            }
    }

    fn direct_pack(word: F128, bits: usize) -> F128 {
        let field = SmallField::for_width(bits).unwrap();
        (0..128 / bits).fold(F128::ZERO, |sum, j| {
            sum + from_raw(1u128 << j) * field.embed(coordinate(word, bits, j))
        })
    }

    // Deliberately independent dense loops: no production transpose, row-tensor
    // helper, coordinate map or CSC fold contributes to this reference claim.
    fn source(
        matrix: &[Vec<bool>],
        witness: &[F128],
        bits: usize,
        factors: &[[F128; 2]],
        rho: Vec<F128>,
    ) -> (Vec<F128>, MatrixClaim, Vec<F128>) {
        let field = SmallField::for_width(bits).unwrap();
        let image: Vec<_> = matrix
            .iter()
            .map(|row| {
                row.iter()
                    .zip(witness)
                    .filter(|(bit, _)| **bit)
                    .fold(F128::ZERO, |sum, (_, &w)| sum + w)
            })
            .collect();
        let mut columns = vec![F128::ZERO; 128 / bits];
        for (i, &word) in image.iter().enumerate() {
            let weight = factors
                .iter()
                .enumerate()
                .fold(F128::ONE, |w, (axis, pair)| {
                    w * pair[(i >> (factors.len() - axis - 1)) & 1]
                });
            for (j, column) in columns.iter_mut().enumerate() {
                *column += weight * field.embed(coordinate(word, bits, j));
            }
        }
        let value = columns
            .iter()
            .zip(&rho)
            .fold(F128::ZERO, |sum, (&v, &w)| sum + v * w);
        (
            image,
            MatrixClaim {
                row_factors: factors.to_vec(),
                column_weights: rho,
                value,
            },
            columns,
        )
    }

    fn direct_mle(words: &[F128], bits: usize, point: &[F128]) -> F128 {
        words
            .iter()
            .enumerate()
            .fold(F128::ZERO, |sum, (i, &word)| {
                let weight = point.iter().enumerate().fold(F128::ONE, |w, (axis, &r)| {
                    w * if i >> axis & 1 == 0 { F128::ONE + r } else { r }
                });
                sum + weight * direct_pack(word, bits)
            })
    }

    #[test]
    fn coordinate_maps_are_invertible_and_projection_is_k_linear() {
        let mut rng = Rng::new(0xF1E1_D001);
        for bits in [1, 2, 4, 8, 16, 32, 64, 128] {
            let packing = plan(bits);
            let eta = rng.f128_vec(packing.kappa());
            let projection = packing.projection_table(&eta);
            for (j, &gamma) in packing.gamma.iter().enumerate() {
                assert_eq!(map_word(gamma, &projection), eta[j]);
            }
            for _ in 0..32 {
                let word = rng.f128_vec(1)[0];
                assert_eq!(packing.pack_word(word), direct_pack(word, bits));
                let coordinates = packing
                    .inverse_columns
                    .iter()
                    .enumerate()
                    .filter(|(bit, _)| raw(word) >> *bit & 1 != 0)
                    .fold(0u128, |sum, (_, &column)| sum ^ column);
                assert_eq!(packing.pack_word(from_raw(coordinates)), word);
                let value = SmallField::for_width(bits).unwrap().embed(coordinate(
                    rng.f128_vec(1)[0],
                    bits,
                    0,
                ));
                assert_eq!(
                    map_word(word * value, &projection),
                    map_word(word, &projection) * value
                );
            }
            if bits > 1 {
                let mut bad = packing.subfield_basis.clone();
                bad[1] = F128::ONE;
                assert!(FieldPacking::new(bits, &bad).is_err());
            }
        }
        assert!(FieldPacking::new(3, &[F128::ONE; 3]).is_err());
        assert!(FieldPacking::new(1, &[F128::ZERO]).is_err());
    }

    #[test]
    fn actual_field_columns_and_raw_pcs_queries_match_dense_reference() {
        let mut rng = Rng::new(0xF1E1_D002);
        for bits in [1, 2, 4, 8, 16, 32, 64, 128] {
            let packing = plan(bits);
            for log in [0, 3, 6] {
                let n = 1 << log;
                let (matrix, a, b) = dense_matrices(n);
                for (left, dense) in [(true, &a), (false, &b)] {
                    for groups in [1, 2, 3] {
                        let mut factors: Vec<_> = (0..log)
                            .map(|_| [rng.f128_vec(1)[0], rng.f128_vec(1)[0]])
                            .collect();
                        if log > 0 && groups == 2 {
                            factors[0] = [F128::ZERO; 2];
                        }
                        if log > 0 && groups == 3 {
                            factors[0] = [F128::ONE; 2];
                        }
                        let witnesses: Vec<_> = (0..groups).map(|_| rng.f128_vec(n)).collect();
                        let triples: Vec<_> = witnesses
                            .iter()
                            .map(|w| {
                                source(dense, w, bits, &factors, rng.f128_vec(packing.kappa()))
                            })
                            .collect();
                        let sources: Vec<_> = triples.iter().map(|t| t.1.clone()).collect();
                        let ws: Vec<_> = witnesses.iter().map(Vec::as_slice).collect();
                        let ims: Vec<_> = triples.iter().map(|t| t.0.as_slice()).collect();
                        let mut pc = FsChallenger::new(b"field-image-dense");
                        let (proof, claims) = prove_public_batch(
                            &matrix, left, &packing, &ws, &ims, &sources, &mut pc,
                        )
                        .unwrap();
                        assert_eq!(
                            proof.columns,
                            triples.iter().map(|t| t.2.clone()).collect::<Vec<_>>()
                        );
                        assert!(proof.columns.iter().all(|v| v.len() == 128 / bits));
                        let expected = witnesses
                            .iter()
                            .zip(&claims.weights)
                            .fold(F128::ZERO, |sum, (w, &z)| {
                                sum + z * direct_mle(w, bits, &claims.witness.point)
                            });
                        assert_eq!(claims.witness.value, expected);
                        assert_eq!(claims.columns, packing.columns());
                        let encoded = bincode::serialize(&proof).unwrap();
                        let decoded = bincode::deserialize(&encoded).unwrap();
                        let mut vc = FsChallenger::new(b"field-image-dense");
                        assert_eq!(
                            verify_public_batch(
                                &matrix, left, &packing, &sources, &decoded, &mut vc
                            )
                            .unwrap(),
                            claims
                        );
                        assert_eq!(pc.sample_f128_vec(4), vc.sample_f128_vec(4));
                    }
                }
            }
        }
    }

    #[test]
    fn binary_delegation_preserves_existing_messages_and_challenges() {
        let mut rng = Rng::new(0xF1E1_D003);
        let packing = plan(1);
        let (matrix, a, _) = dense_matrices(16);
        let factors: Vec<_> = (0..4)
            .map(|_| [rng.f128_vec(1)[0], rng.f128_vec(1)[0]])
            .collect();
        let witnesses: Vec<_> = (0..2).map(|_| rng.f128_vec(16)).collect();
        let triples: Vec<_> = witnesses
            .iter()
            .map(|w| source(&a, w, 1, &factors, rng.f128_vec(128)))
            .collect();
        let ws: Vec<_> = witnesses.iter().map(Vec::as_slice).collect();
        let ims: Vec<_> = triples.iter().map(|t| t.0.as_slice()).collect();
        let sources: Vec<_> = triples.iter().map(|t| t.1.clone()).collect();
        let mut old = FsChallenger::new(b"field-image-binary");
        let mut new = FsChallenger::new(b"field-image-binary");
        let (old_proof, old_claims) =
            multiple_image::prove_public_batch(&matrix, true, &ws, &ims, &sources, &mut old)
                .unwrap();
        let (proof, claims) =
            prove_public_batch(&matrix, true, &packing, &ws, &ims, &sources, &mut new).unwrap();
        assert_eq!(
            bincode::serialize(&old_proof).unwrap(),
            bincode::serialize(&proof).unwrap()
        );
        assert_eq!(old_claims.witness, claims.witness);
        assert_eq!(old_claims.weights, claims.weights);
        assert_eq!(old.sample_f128_vec(4), new.sample_f128_vec(4));
    }

    #[test]
    fn malformed_statements_sources_messages_and_terminals_are_rejected() {
        let mut rng = Rng::new(0xF1E1_D004);
        for bits in [2, 8, 32, 128] {
            let packing = plan(bits);
            let (matrix, a, _) = dense_matrices(16);
            let factors: Vec<_> = (0..4)
                .map(|_| [rng.f128_vec(1)[0], rng.f128_vec(1)[0]])
                .collect();
            let witnesses: Vec<_> = (0..3).map(|_| rng.f128_vec(16)).collect();
            let triples: Vec<_> = witnesses
                .iter()
                .map(|w| source(&a, w, bits, &factors, rng.f128_vec(packing.kappa())))
                .collect();
            let ws: Vec<_> = witnesses.iter().map(Vec::as_slice).collect();
            let ims: Vec<_> = triples.iter().map(|t| t.0.as_slice()).collect();
            let sources: Vec<_> = triples.iter().map(|t| t.1.clone()).collect();
            let ch = || FsChallenger::new(b"field-image-tamper");
            let (proof, _) =
                prove_public_batch(&matrix, true, &packing, &ws, &ims, &sources, &mut ch())
                    .unwrap();
            for mutation in 0..6 {
                let mut bad = proof.clone();
                match mutation {
                    0 => bad.columns[0][0] += F128::ONE,
                    1 => {
                        bad.columns[0].pop();
                    }
                    2 => bad.ip.final_a += F128::ONE,
                    3 => bad.ip.final_b += F128::ONE,
                    4 => bad.ip.rounds[0][0] += F128::ONE,
                    _ => {
                        bad.columns.pop();
                    }
                }
                assert!(
                    verify_public_batch(&matrix, true, &packing, &sources, &bad, &mut ch())
                        .is_err()
                );
            }
            assert!(
                verify_public_batch(&matrix, false, &packing, &sources, &proof, &mut ch()).is_err()
            );
            let mut wrong = witnesses.clone();
            wrong[0][0] += F128::ONE;
            let wrong_ws: Vec<_> = wrong.iter().map(Vec::as_slice).collect();
            assert!(
                prove_public_batch(
                    &matrix,
                    true,
                    &packing,
                    &wrong_ws,
                    &ims,
                    &sources,
                    &mut ch()
                )
                .is_err()
            );
            let mut bad_sources = sources.clone();
            bad_sources[0].value += F128::ONE;
            assert!(
                verify_public_batch(&matrix, true, &packing, &bad_sources, &proof, &mut ch())
                    .is_err()
            );
            bad_sources = sources.clone();
            bad_sources[1].row_factors[0][0] += F128::ONE;
            assert!(
                verify_public_batch(&matrix, true, &packing, &bad_sources, &proof, &mut ch())
                    .is_err()
            );
            assert!(
                prove_public_batch(&matrix, true, &packing, &ws[..2], &ims, &sources, &mut ch())
                    .is_err()
            );
            assert!(verify_public_batch(&matrix, true, &packing, &[], &proof, &mut ch()).is_err());
        }
    }
}
