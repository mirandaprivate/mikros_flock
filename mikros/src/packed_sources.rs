//! Packed commitment transport for sparse-matrix value and position columns.
//!
//! A width-w entry occupies w bits, so one native F128 carrier contains 128/w
//! adjacent entries. A claim evaluates the multilinear extension of the raw
//! numeric F128 embeddings, rather than a Boolean extension of individual bits.
//! The public layout converts that claim to the existing native FLOCK tensor
//! functional without reading any committed values on the verifier side.
//!
//! The enclosing lookup protocol binds the public layout and commitment before
//! reduction and authenticates the resulting queries. Public matrix commitments
//! are stored in the preprocessing verification key.

use flock_core::{
    challenger::Challenger,
    pcs::{
        self, Commitment, PcsParams, ProverData,
        functional::TensorClaim,
        ligerito::{LigeritoProfile, embedded_initial_k_or_default},
        merged_tensor,
    },
    r1cs::SparseBinaryMatrix,
    schedule::{Registry, TableClass, TableType},
    union::UnionInstance,
};
use flock_field::F128;
use serde::{Deserialize, Serialize};

/// Unpacked offline input. Every value's raw representation must fit `width`.
#[derive(Clone, Debug)]
pub struct Column {
    pub width: usize,
    pub values: Vec<F128>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ColumnSpec {
    pub len: usize,
    pub width: usize,
    pub word_offset: usize,
    pub word_len: usize,
}

/// An MLE query of one raw-value column; coordinates are low-bit first.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claim {
    pub column: usize,
    pub point: Vec<F128>,
    pub value: F128,
}

/// Public geometry. Each column occupies an aligned power-of-two word block.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Layout {
    specs: Vec<ColumnSpec>,
    words: usize,
}

fn raw(value: F128) -> u128 {
    value.lo as u128 | ((value.hi as u128) << 64)
}

fn from_raw(value: u128) -> F128 {
    F128::new(value as u64, (value >> 64) as u64)
}

fn basis(bit: usize) -> F128 {
    from_raw(1u128 << bit)
}

fn eq_pair(coordinate: F128) -> [F128; 2] {
    [F128::ONE + coordinate, coordinate]
}

impl Layout {
    /// `specs` contains `(logical_length, entry_width_bits)` for each column.
    /// Lengths and widths must be powers of two; widths lie in 1..=128.
    pub fn new(specs: &[(usize, usize)]) -> Result<Self, String> {
        if specs.is_empty() {
            return Err("packed source layout needs at least one column".into());
        }
        let mut result = Vec::with_capacity(specs.len());
        let mut end = 0usize;
        for &(len, width) in specs {
            if !len.is_power_of_two() || !width.is_power_of_two() || width > 128 {
                return Err(
                    "packed source length/width must be powers of two, width <= 128".into(),
                );
            }
            let word_len = (len / (128 / width)).max(1);
            let word_offset = end
                .checked_add(word_len - 1)
                .map(|x| x & !(word_len - 1))
                .ok_or("packed source alignment overflow")?;
            end = word_offset
                .checked_add(word_len)
                .ok_or("packed source length overflow")?;
            result.push(ColumnSpec {
                len,
                width,
                word_offset,
                word_len,
            });
        }
        let words = end
            .max(1 << 15)
            .checked_next_power_of_two()
            .ok_or("packed source padded length overflow")?;
        // Both native PCS and its bit-domain shapes use machine-size shifts.
        if words.ilog2() as usize + 7 >= usize::BITS as usize {
            return Err("packed source bit domain exceeds address space".into());
        }
        Ok(Self {
            specs: result,
            words,
        })
    }

    pub fn specs(&self) -> &[ColumnSpec] {
        &self.specs
    }

    pub fn word_len(&self) -> usize {
        self.words
    }

    pub fn log_words(&self) -> usize {
        self.words.ilog2() as usize
    }

    /// Offline bit packing. Padding and alignment gaps are canonically zero.
    pub fn pack(&self, columns: &[Column]) -> Result<Vec<F128>, String> {
        if columns.len() != self.specs.len() {
            return Err("packed source column count mismatch".into());
        }
        let mut packed = vec![F128::ZERO; self.words];
        for (column, spec) in columns.iter().zip(&self.specs) {
            if column.width != spec.width || column.values.len() != spec.len {
                return Err("packed source column shape mismatch".into());
            }
            let entries_per_word = 128 / spec.width;
            for (index, &value) in column.values.iter().enumerate() {
                let value = raw(value);
                if spec.width != 128 && (value >> spec.width) != 0 {
                    return Err("packed source value exceeds entry width".into());
                }
                let word = &mut packed[spec.word_offset + index / entries_per_word];
                let shift = (index % entries_per_word) * spec.width;
                *word = from_raw(raw(*word) | (value << shift));
            }
        }
        Ok(packed)
    }

    /// Convert a numeric-entry MLE to the native bit-linear functional.
    /// The conversion uses only the public geometry, query, and claimed value.
    pub fn tensor_claim(&self, claim: &Claim) -> Result<TensorClaim, String> {
        let spec = self
            .specs
            .get(claim.column)
            .ok_or("packed source column index")?;
        let log_len = spec.len.ilog2() as usize;
        if claim.point.len() != log_len {
            return Err("packed source query dimension mismatch".into());
        }
        let log_entries_per_word = (128 / spec.width).ilog2() as usize;
        let local_vars = log_len.min(log_entries_per_word);
        let local_eq: Vec<_> = (0..1usize << local_vars)
            .map(|index| {
                claim.point[..local_vars]
                    .iter()
                    .enumerate()
                    .fold(F128::ONE, |product, (bit, &coordinate)| {
                        product * eq_pair(coordinate)[(index >> bit) & 1]
                    })
            })
            .collect();
        let columns = (0..128)
            .map(|bit| {
                let entry = bit / spec.width;
                local_eq
                    .get(entry)
                    .map_or(F128::ZERO, |&weight| weight * basis(bit % spec.width))
            })
            .collect();
        let mut rows: Vec<_> = claim.point[local_vars..]
            .iter()
            .copied()
            .map(eq_pair)
            .collect();
        let log_block = spec.word_len.ilog2() as usize;
        debug_assert_eq!(rows.len(), log_block);
        for bit in log_block..self.log_words() {
            rows.push(if (spec.word_offset >> bit) & 1 == 0 {
                [F128::ONE, F128::ZERO]
            } else {
                [F128::ZERO, F128::ONE]
            });
        }
        Ok(TensorClaim {
            columns,
            rows,
            value: claim.value,
        })
    }

    pub fn tensor_claims(&self, claims: &[Claim]) -> Result<Vec<TensorClaim>, String> {
        claims
            .iter()
            .map(|claim| self.tensor_claim(claim))
            .collect()
    }

    /// Full rectangular native PCS geometry, with no compaction or lane omission.
    pub fn registry(&self) -> Registry {
        Registry::new(
            vec![TableType {
                k_log: 7,
                useful_bits: 128,
                a_0: SparseBinaryMatrix::new(0, 0, vec![]),
                b_0: SparseBinaryMatrix::new(0, 0, vec![]),
                c_0: SparseBinaryMatrix::new(0, 0, vec![]),
                const_pin: None,
                class: TableClass::Boolean,
                io_schema: vec![],
            }],
            self.log_words(),
        )
    }

    pub fn parameters(&self) -> PcsParams {
        let m = self.log_words() + 7;
        let profile = LigeritoProfile::Fast;
        PcsParams {
            m,
            log_inv_rate: profile.log_inv_rate(),
            log_batch_size: embedded_initial_k_or_default(m, profile),
            profile,
            num_lanes: None,
            merkle_hash: Default::default(),
        }
    }

    pub fn commit(&self, packed: &[F128]) -> Result<(Commitment, ProverData), String> {
        if packed.len() != self.words {
            return Err("packed source carrier length mismatch".into());
        }
        let params = self.parameters();
        params.ligerito_prover_config()?;
        Ok(pcs::commit(packed, &params))
    }

    fn check_commitment(&self, commitment: &Commitment) -> Result<(), String> {
        if commitment.params != self.parameters() {
            return Err("packed source commitment parameters mismatch".into());
        }
        Ok(())
    }

    /// Native FLOCK merged opening; all challenge-dependent conversion is online.
    pub fn open<C: Challenger>(
        &self,
        packed: Vec<F128>,
        data: &ProverData,
        commitment: &Commitment,
        claims: &[Claim],
        ch: &mut C,
    ) -> Result<merged_tensor::Proof, String> {
        self.check_commitment(commitment)?;
        if packed.len() != self.words || claims.is_empty() {
            return Err("packed source opening shape mismatch".into());
        }
        let claims = self.tensor_claims(claims)?;
        let registry = self.registry();
        let union = UnionInstance::new(&registry, vec![self.words]);
        merged_tensor::open_identity(packed, data, commitment, &claims, &union, ch)
            .map_err(|error| format!("packed source native opening: {error:?}"))
    }

    /// Authenticate every query without scanning any sparse column values.
    pub fn verify<C: Challenger>(
        &self,
        commitment: &Commitment,
        claims: &[Claim],
        proof: &merged_tensor::Proof,
        ch: &mut C,
    ) -> Result<(), String> {
        self.check_commitment(commitment)?;
        if claims.is_empty() {
            return Err("packed source opening requires claims".into());
        }
        let claims = self.tensor_claims(claims)?;
        let registry = self.registry();
        let union = UnionInstance::new(&registry, vec![self.words]);
        merged_tensor::verify(commitment, &claims, proof, &union, ch)
            .map_err(|error| format!("packed source native verification: {error:?}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::{challenger::FsChallenger, pcs::functional::row_weights};

    fn evaluate(values: &[F128], point: &[F128]) -> F128 {
        let mut work = values.to_vec();
        for &coordinate in point {
            for index in 0..work.len() / 2 {
                work[index] =
                    work[2 * index] + coordinate * (work[2 * index] + work[2 * index + 1]);
            }
            work.truncate(work.len() / 2);
        }
        work[0]
    }

    fn evaluate_tensor(packed: &[F128], claim: &TensorClaim) -> F128 {
        packed
            .iter()
            .zip(row_weights(&claim.rows))
            .fold(F128::ZERO, |sum, (&word, weight)| {
                if weight.is_zero() {
                    return sum;
                }
                let value = (0..128)
                    .filter(|&bit| (raw(word) >> bit) & 1 != 0)
                    .fold(F128::ZERO, |sum, bit| sum + claim.columns[bit]);
                sum + weight * value
            })
    }

    fn fixture() -> (Layout, Vec<Column>) {
        let specs = [(256, 1), (64, 16), (4, 32), (16, 128), (2, 16), (32, 32)];
        let columns = specs
            .iter()
            .enumerate()
            .map(|(column, &(len, width))| Column {
                width,
                values: (0..len)
                    .map(|i| {
                        if width == 128 {
                            F128::new(
                                (i as u64).wrapping_mul(0xD6E8FEB86659FD93),
                                17 + column as u64 + i as u64,
                            )
                        } else {
                            from_raw((i as u128 * 997 + column as u128) & ((1u128 << width) - 1))
                        }
                    })
                    .collect(),
            })
            .collect();
        (Layout::new(&specs).unwrap(), columns)
    }

    #[test]
    fn packed_widths_offsets_and_numeric_mles_match() {
        let (layout, columns) = fixture();
        let packed = layout.pack(&columns).unwrap();
        assert_eq!(layout.word_len(), 1 << 15);
        assert_eq!(
            layout
                .specs
                .iter()
                .map(|s| s.word_offset)
                .collect::<Vec<_>>(),
            [0, 8, 16, 32, 48, 56]
        );
        for (column, source) in columns.iter().enumerate() {
            for kind in 0..3 {
                let point: Vec<_> = (0..source.values.len().ilog2())
                    .map(|i| match kind {
                        0 => F128::ZERO,
                        1 => F128::ONE,
                        _ => F128::new(0x1234 + 31 * i as u64, 17 + i as u64),
                    })
                    .collect();
                let claim = Claim {
                    column,
                    value: evaluate(&source.values, &point),
                    point,
                };
                let native = layout.tensor_claim(&claim).unwrap();
                assert_eq!(
                    evaluate_tensor(&packed, &native),
                    claim.value,
                    "column {column}, point kind {kind}"
                );
            }
        }
        // Exact byte packing of adjacent 16-bit values, including a partial word.
        let spec = &layout.specs[1];
        let expected = columns[1].values[..8]
            .iter()
            .enumerate()
            .fold(0, |x, (i, &v)| x | (raw(v) << (16 * i)));
        assert_eq!(raw(packed[spec.word_offset]), expected);
        assert_eq!(raw(packed[48]) >> 32, 0);
        assert_eq!(packed[3], F128::ZERO);
    }

    #[test]
    fn malformed_columns_and_queries_are_rejected() {
        assert!(Layout::new(&[]).is_err());
        for specs in [[(0, 1)], [(3, 1)], [(4, 0)], [(4, 3)], [(4, 256)]] {
            assert!(Layout::new(&specs).is_err());
        }
        let (layout, mut columns) = fixture();
        columns[0].values[0] = F128::new(2, 0);
        assert!(layout.pack(&columns).is_err());
        assert!(layout.pack(&columns[..1]).is_err());
        assert!(
            layout
                .tensor_claim(&Claim {
                    column: 99,
                    point: vec![],
                    value: F128::ZERO
                })
                .is_err()
        );
        assert!(
            layout
                .tensor_claim(&Claim {
                    column: 0,
                    point: vec![],
                    value: F128::ZERO
                })
                .is_err()
        );
    }

    #[test]
    fn packed_sparse_columns_native_pcs_roundtrip_and_tamper_rejection() {
        let (layout, columns) = fixture();
        let packed = layout.pack(&columns).unwrap();
        let claims: Vec<_> = columns
            .iter()
            .enumerate()
            .map(|(column, source)| {
                let point: Vec<_> = (0..source.values.len().ilog2())
                    .map(|i| F128::new(71 + i as u64, 131 + 13 * i as u64))
                    .collect();
                Claim {
                    column,
                    value: evaluate(&source.values, &point),
                    point,
                }
            })
            .collect();
        let (commitment, data) = layout.commit(&packed).unwrap();
        let start = |layout: &Layout, commitment: &Commitment| {
            let mut ch = FsChallenger::new(b"packed-sparse-columns-native-pcs-test");
            ch.observe_bytes(&bincode::serialize(layout).unwrap());
            ch.observe_bytes(&bincode::serialize(commitment).unwrap());
            ch
        };
        let mut pch = start(&layout, &commitment);
        let proof = layout
            .open(packed, &data, &commitment, &claims, &mut pch)
            .unwrap();
        let check = |claims: &[Claim], proof: &merged_tensor::Proof| {
            let mut ch = start(&layout, &commitment);
            layout.verify(&commitment, claims, proof, &mut ch)?;
            Ok::<_, String>(ch.sample_f128())
        };
        assert_eq!(pch.sample_f128(), check(&claims, &proof).unwrap());
        for column in 0..claims.len() {
            let mut bad = claims.clone();
            bad[column].value += F128::ONE;
            assert!(check(&bad, &proof).is_err());
        }
        let mut bad = claims.clone();
        // Columns 2 and 4 have different lengths; adjust the query so rejection
        // comes from authentication, rather than only the dimension check.
        bad[2] = Claim {
            column: 4,
            point: claims[4].point.clone(),
            value: claims[2].value,
        };
        assert!(check(&bad, &proof).is_err());
        let mut bad = claims.clone();
        bad[0].point[0] += F128::ONE;
        assert!(check(&bad, &proof).is_err());
        let mut bad_proof = proof.clone();
        bad_proof.merged.as_mut().unwrap().q_eval += F128::ONE;
        assert!(check(&claims, &bad_proof).is_err());
        let mut bad_commitment = commitment.clone();
        bad_commitment.params.log_inv_rate += 1;
        assert!(
            layout
                .verify(
                    &bad_commitment,
                    &claims,
                    &proof,
                    &mut start(&layout, &bad_commitment)
                )
                .is_err()
        );
    }
}
