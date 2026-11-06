//! Authenticate the existing two canonical packed-word MLEs with WHIR.

use flock_core::pcs::functional::TensorClaim;
use flock_field::F128;

pub use mikros_whir::{Cache, Commitment, VerifierKey};
pub const BACKEND_NAME: &str = "WHIR F128 equality";
pub const DISPLAY_NAME: &str = "WHIR";
pub const TRANSCRIPT_DOMAIN: &[u8] = b"Mikros-PLONK-WHIR-F128-v1";
pub const WITNESS_PROOF_LABEL: &[u8] = b"whir-witness-opening";
pub const PUBLIC_PROOF_LABEL: &[u8] = b"whir-public-opening";
pub const OFFLINE_SETUP_STAGE: &str = "offline_whir_setup";
pub const DEFAULT_LOG_GATES: usize = 26;
pub const DISTANCE_CERTIFIED: Option<bool> = None;

pub fn profile() -> serde_json::Value {
    mikros_whir::profile()
}

pub struct Source(mikros_whir::Source);

impl Source {
    pub fn new(words: usize, seed: u64) -> Result<Self, String> {
        Ok(Self(mikros_whir::Source::new(words, seed)?))
    }

    pub fn verifier_key(&self) -> &VerifierKey {
        self.0.verifier_key()
    }

    pub fn commit(&self, words: &[F128]) -> Result<(Commitment, Cache), String> {
        self.0.commit(words)
    }

    pub fn open(
        &self,
        commitment: &Commitment,
        claim: &TensorClaim,
        cache: Cache,
    ) -> Result<Vec<u8>, String> {
        let point = checked_point(claim, self.verifier_key().words())?;
        self.0.open(commitment, &point, claim.value, cache)
    }
}

fn checked_point(claim: &TensorClaim, words: usize) -> Result<Vec<F128>, String> {
    if !words.is_power_of_two()
        || claim.rows.len() != words.ilog2() as usize
        || claim.columns.len() != 128
    {
        return Err("WHIR terminal dimensions".into());
    }
    for (bit, &column) in claim.columns.iter().enumerate() {
        let expected = if bit < 64 {
            F128::new(1u64 << bit, 0)
        } else {
            F128::new(0, 1u64 << (bit - 64))
        };
        if column != expected {
            return Err("WHIR terminal must use the canonical GHASH word basis".into());
        }
    }
    claim
        .rows
        .iter()
        .map(|&[a, b]| {
            if a + b != F128::ONE {
                Err("WHIR terminal must be an equality MLE".into())
            } else {
                // Both TensorClaim and the adapter use low-variable-first coordinates.
                Ok(b)
            }
        })
        .collect()
}

pub fn verify(
    key: &VerifierKey,
    commitment: &Commitment,
    claim: &TensorClaim,
    encoded: &[u8],
) -> Result<(), String> {
    let point = checked_point(claim, key.words())?;
    mikros_whir::verify(key, commitment, &point, claim.value, encoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn claim(point: &[F128], value: F128) -> TensorClaim {
        TensorClaim {
            rows: point.iter().map(|&r| [F128::ONE + r, r]).collect(),
            columns: (0..128)
                .map(|bit| {
                    if bit < 64 {
                        F128::new(1 << bit, 0)
                    } else {
                        F128::new(0, 1 << (bit - 64))
                    }
                })
                .collect(),
            value,
        }
    }

    #[test]
    fn canonical_bit_functional_and_lsb_order() {
        let point = [F128::new(7, 9), F128::ONE, F128::ZERO, F128::new(11, 13)];
        let c = claim(&point, F128::ONE);
        assert_eq!(checked_point(&c, 16).unwrap(), point);
        let mut bad = c.clone();
        bad.columns[91] += F128::ONE;
        assert!(checked_point(&bad, 16).is_err());
        let mut bad = c.clone();
        bad.rows[0][0] += F128::ONE;
        assert!(checked_point(&bad, 16).is_err());
        assert!(checked_point(&c, 32).is_err());
    }

    #[test]
    fn full_packed_word_opening_binds_all_bits() {
        let words: Vec<_> = (0..32)
            .map(|i| F128::new((i + 1) * 391, (i + 7) * 977))
            .collect();
        let point: Vec<_> = (0..5).map(|i| F128::new(i + 2, i + 11)).collect();
        let value = words
            .iter()
            .enumerate()
            .map(|(i, &word)| {
                point.iter().enumerate().fold(word, |acc, (j, &r)| {
                    acc * if i >> j & 1 == 0 { F128::ONE + r } else { r }
                })
            })
            .fold(F128::ZERO, |a, b| a + b);
        let source = Source::new(words.len(), 97).unwrap();
        let (cm, cache) = source.commit(&words).unwrap();
        let c = claim(&point, value);
        let proof = source.open(&cm, &c, cache).unwrap();
        verify(source.verifier_key(), &cm, &c, &proof).unwrap();
        let mut bad = c.clone();
        bad.value += F128::new(0, 1 << 63);
        assert!(verify(source.verifier_key(), &cm, &bad, &proof).is_err());
        let mut bad = c.clone();
        bad.columns[127] = F128::ZERO;
        assert!(verify(source.verifier_key(), &cm, &bad, &proof).is_err());
    }
}
