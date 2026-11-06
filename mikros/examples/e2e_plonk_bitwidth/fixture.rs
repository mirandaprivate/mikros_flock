//! One connected MUL/ADD circuit over an actual binary subfield.
//!
//! Each raw PCS word contains 128 / b consecutive b-bit field coordinates.
//! These coordinates are not the GHASH encoding of the subfield element.

use flock_core::pcs::functional::TensorClaim;
use flock_field::F128;
use mikros_permutation::field::FastField;
#[cfg(test)]
use mikros_permutation::field::SmallField;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[path = "../e2e_common/near_full_circuit.rs"]
pub mod near_full_circuit;

#[derive(Default)]
pub struct Claims {
    pub witness: Vec<TensorClaim>,
    pub public: Vec<TensorClaim>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Circuit {
    pub log_gates: usize,
    pub value_bits: usize,
    pub seed: u64,
    pub io: u128,
    #[serde(default)]
    pub near_full: bool,
}

impl Circuit {
    pub fn new(log_gates: usize, value_bits: usize, seed: u64) -> Result<Self, String> {
        let mut c = Self {
            log_gates,
            value_bits,
            seed,
            io: 0,
            near_full: false,
        };
        c.validate()?;
        c.io = c.value(2, c.n() - 1);
        Ok(c)
    }

    pub fn new_near_full(log_gates: usize, value_bits: usize, seed: u64) -> Result<Self, String> {
        let mut c = Self {
            log_gates,
            value_bits,
            seed,
            io: 0,
            near_full: true,
        };
        c.validate()?;
        c.io = near_full_circuit::public_output(log_gates, seed) as u128;
        Ok(c)
    }

    pub fn circuit_name(&self) -> &'static str {
        if self.near_full {
            "near-full"
        } else {
            "paired"
        }
    }
    pub fn genuine_gate_count(&self) -> usize {
        if self.near_full {
            near_full_circuit::gate_count(self.log_gates)
        } else {
            self.n()
        }
    }
    pub fn independent_input_count(&self) -> usize {
        if self.near_full {
            near_full_circuit::INPUTS
        } else {
            self.n()
        }
    }
    pub fn r1cs_rows(&self) -> usize {
        if self.near_full {
            self.n()
        } else {
            4 * self.n()
        }
    }
    pub fn output_gate(&self) -> usize {
        self.genuine_gate_count() - 1
    }

    pub fn validate(&self) -> Result<(), String> {
        if !(14..=30).contains(&self.log_gates)
            || ![1, 2, 4, 8, 16, 32, 64, 128].contains(&self.value_bits)
            || (self.near_full && self.value_bits != 1)
        {
            return Err("PLONK bitwidth circuit dimensions".into());
        }
        if self.io > self.value_mask() || self.tag_bits() > 256 {
            return Err("PLONK bitwidth public output or tag dimensions".into());
        }
        Ok(())
    }

    pub fn n(&self) -> usize {
        1usize << self.log_gates
    }
    pub fn h(&self) -> usize {
        self.log_gates + 2
    }
    pub fn kappa(&self) -> usize {
        128 / self.value_bits
    }
    pub fn wire_words(&self) -> usize {
        self.n() / self.kappa()
    }
    pub fn witness_words(&self) -> usize {
        (4 * self.wire_words()).max(1 << 15)
    }
    pub fn copy_len(&self) -> usize {
        4 * self.n()
    }
    pub fn live_len(&self) -> usize {
        3 * self.n() + 1
    }
    pub fn tag_bits(&self) -> usize {
        (self.h() + self.value_bits).next_power_of_two()
    }
    pub fn copy_kappa(&self) -> usize {
        (128 / self.tag_bits()).max(1)
    }
    pub fn copy_field_bits(&self) -> usize {
        self.tag_bits().max(128)
    }
    pub fn copy_mode(&self) -> &'static str {
        match self.tag_bits() {
            1..=64 => "packed",
            128 => "unpacked_f128",
            256 => "unpacked_f256",
            _ => unreachable!("validated copy tag field"),
        }
    }
    pub fn label_digits(&self) -> usize {
        self.h().div_ceil(self.value_bits)
    }
    pub fn public_blocks(&self) -> usize {
        5 + 3 * self.label_digits()
    }
    pub fn public_words(&self) -> usize {
        (self.public_blocks().next_power_of_two() * self.wire_words()).max(1 << 15)
    }
    pub fn value_mask(&self) -> u128 {
        if self.value_bits == 128 {
            u128::MAX
        } else {
            (1u128 << self.value_bits) - 1
        }
    }

    pub fn selector(which: usize, gate: usize) -> bool {
        match which {
            0 | 1 => gate & 1 != 0,
            2 => gate & 1 == 0,
            3 => true,
            4 => false,
            _ => panic!("selector"),
        }
    }

    pub fn selector_at(&self, which: usize, gate: usize) -> bool {
        if self.near_full {
            near_full_circuit::selector(self.log_gates, which, gate)
        } else {
            Self::selector(which, gate)
        }
    }

    pub fn sigma(&self, index: usize) -> usize {
        if self.near_full {
            return near_full_circuit::sigma(self.log_gates, index);
        }
        let n = self.n();
        if index >= 3 * n {
            return if index == 3 * n { 3 * n - 1 } else { index };
        }
        let gate = index % n;
        match index / n {
            0 if gate & 1 == 0 => n + ((gate + n - 1) & (n - 1)),
            0 => 2 * n + gate - 1,
            1 if gate & 1 == 0 => index,
            1 => (gate + 1) & (n - 1),
            2 if gate & 1 == 0 => gate + 1,
            2 if gate == n - 1 => 3 * n,
            2 => index,
            _ => unreachable!(),
        }
    }

    fn input(&self, j: usize, lane: u64) -> u128 {
        let key = (j as u64).wrapping_mul(0xD6E8FEB86659FD93) ^ self.seed ^ lane;
        let low = mix(key) as u128;
        let value = if self.value_bits == 128 {
            low | ((mix(key ^ 0xA0761D6478BD642F) as u128) << 64)
        } else {
            low
        };
        value & self.value_mask()
    }

    /// Polynomial-basis coordinates in F_(2^b), not b independent executions.
    pub fn value(&self, wire: usize, gate: usize) -> u128 {
        if self.near_full {
            // Scalar reference only. witness() takes the linear packed route.
            return near_full_circuit::reference_wire_value(self.log_gates, self.seed, wire, gate)
                as u128;
        }
        let j = gate / 2;
        let a = self.input(j, 0);
        let b = self.input(j, 0x5A17DA7A);
        let product = FastField::for_width(self.value_bits)
            .unwrap()
            .multiply(a, b);
        if gate & 1 == 0 {
            match wire {
                0 => a,
                1 => b,
                2 => product,
                _ => panic!("wire"),
            }
        } else {
            let next = self.input((j + 1) & (self.n() / 2 - 1), 0);
            match wire {
                0 => product,
                1 => next,
                2 => product ^ next,
                _ => panic!("wire"),
            }
        }
    }

    /// Read one b-bit coordinate value from the canonical packed PCS input.
    pub fn packed_value(&self, words: &[F128], index: usize) -> u128 {
        let offset = (index % self.kappa()) * self.value_bits;
        (raw(words[index / self.kappa()]) >> offset) & self.value_mask()
    }

    pub fn witness(&self) -> Vec<F128> {
        if self.near_full {
            return near_full_circuit::plonk_witness(self.log_gates, self.seed);
        }
        let m = self.wire_words();
        let mut out = vec![F128::ZERO; self.witness_words()];
        out[..3 * m]
            .par_chunks_mut(m)
            .enumerate()
            .for_each(|(wire, block)| {
                block.par_iter_mut().enumerate().for_each(|(word, dst)| {
                    *dst = from_raw((0..self.kappa()).fold(0, |acc, column| {
                        acc | (self.value(wire, word * self.kappa() + column)
                            << (column * self.value_bits))
                    }));
                });
            });
        out
    }

    pub fn public(&self) -> Vec<F128> {
        let m = self.wire_words();
        let mut out = vec![F128::ZERO; self.public_words()];
        out[..self.public_blocks() * m]
            .par_chunks_mut(m)
            .enumerate()
            .for_each(|(block, dst)| {
                dst.par_iter_mut().enumerate().for_each(|(word, dst)| {
                    *dst = from_raw((0..self.kappa()).fold(0, |acc, column| {
                        let gate = word * self.kappa() + column;
                        let value = if block < 5 {
                            u128::from(self.selector_at(block, gate))
                        } else {
                            let block = block - 5;
                            let wire = block / self.label_digits();
                            let digit = block % self.label_digits();
                            ((self.sigma(wire * self.n() + gate) as u128)
                                >> (digit * self.value_bits))
                                & self.value_mask()
                        };
                        acc | (value << (column * self.value_bits))
                    }));
                });
            });
        out
    }

    pub fn digest(&self) -> [u8; 32] {
        let mut hash = blake3::Hasher::new();
        hash.update(b"connected-alternating-subfield-mul-add-plonk-v1");
        hash.update(&(self.log_gates as u64).to_le_bytes());
        hash.update(&(self.value_bits as u64).to_le_bytes());
        if self.near_full {
            hash.update(near_full_circuit::RECIPE_DOMAIN);
            hash.update(&(self.independent_input_count() as u64).to_le_bytes());
            hash.update(&(self.genuine_gate_count() as u64).to_le_bytes());
        }
        hash.update(&self.io.to_le_bytes());
        *hash.finalize().as_bytes()
    }
}

fn mix(mut x: u64) -> u64 {
    x ^= x >> 30;
    x = x.wrapping_mul(0xBF58476D1CE4E5B9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94D049BB133111EB);
    x ^ (x >> 31)
}

pub fn raw(x: F128) -> u128 {
    x.lo as u128 | ((x.hi as u128) << 64)
}
pub fn from_raw(x: u128) -> F128 {
    F128::new(x as u64, (x >> 64) as u64)
}
pub fn basis() -> Vec<F128> {
    (0..128).map(|bit| from_raw(1u128 << bit)).collect()
}
pub fn bytes(words: &[F128]) -> &[u8] {
    assert!(cfg!(target_endian = "little"));
    // SAFETY: F128 consists of two u64 words; this is its immutable byte view.
    unsafe { std::slice::from_raw_parts(words.as_ptr().cast(), std::mem::size_of_val(words)) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn near_full_public_padding_and_layout_binding() {
        let c = Circuit::new_near_full(14, 1, 97).unwrap();
        assert_eq!(c.genuine_gate_count(), c.n() - 257);
        assert_eq!(c.independent_input_count(), 256);
        assert_eq!(c.r1cs_rows(), c.n());
        assert_eq!(c.output_gate(), c.n() - 258);
        assert_eq!(c.circuit_name(), "near-full");
        let public = c.public();
        for gate in 0..c.n() {
            for selector in 0..5 {
                assert_eq!(
                    c.packed_value(&public, selector * c.n() + gate),
                    c.selector_at(selector, gate) as u128,
                );
                if gate >= c.genuine_gate_count() {
                    assert_eq!(c.selector_at(selector, gate), selector == 3);
                }
            }
            for wire in 0..3 {
                let label = (0..c.h()).fold(0usize, |acc, bit| {
                    acc | ((c.packed_value(&public, (5 + wire * c.h() + bit) * c.n() + gate)
                        as usize)
                        << bit)
                });
                assert_eq!(label, c.sigma(wire * c.n() + gate));
            }
        }
        let mut other_seed = c.clone();
        other_seed.seed ^= 1;
        assert_eq!(other_seed.public(), public);
        assert_eq!(other_seed.digest(), c.digest());
        let mut old_layout = c.clone();
        old_layout.near_full = false;
        assert_ne!(old_layout.digest(), c.digest());
        assert!(Circuit::new_near_full(14, 2, 97).is_err());
        assert!(Circuit::new_near_full(13, 1, 97).is_err());
    }

    #[test]
    fn genuine_subfield_gates_wiring_and_packed_public_digits() {
        for bits in [1, 2, 4, 8, 16, 32, 64, 128] {
            let c = Circuit::new(14, bits, 97).unwrap();
            let field = SmallField::for_width(bits).unwrap();
            let z = c.witness();
            let public = c.public();
            let mut seen = vec![false; c.copy_len()];
            for index in 0..c.copy_len() {
                let next = c.sigma(index);
                assert!(!seen[next]);
                seen[next] = true;
                let value = |index| {
                    if index == 3 * c.n() {
                        c.io
                    } else if index >= c.live_len() {
                        0
                    } else {
                        c.packed_value(&z, index)
                    }
                };
                assert_eq!(value(index), value(next));
            }
            for gate in 0..c.n() {
                let a = c.packed_value(&z, gate);
                let b = c.packed_value(&z, c.n() + gate);
                let out = c.packed_value(&z, 2 * c.n() + gate);
                assert_eq!(
                    out,
                    if gate & 1 == 0 {
                        field.multiply(a, b)
                    } else {
                        a ^ b
                    }
                );
                assert_eq!(
                    field.embed(field.multiply(a, b)),
                    field.embed(a) * field.embed(b)
                );
                for selector in 0..5 {
                    assert_eq!(
                        c.packed_value(&public, selector * c.n() + gate),
                        u128::from(Circuit::selector(selector, gate))
                    );
                }
                for wire in 0..3 {
                    let label = (0..c.label_digits()).fold(0u128, |acc, digit| {
                        acc | (c.packed_value(
                            &public,
                            (5 + wire * c.label_digits() + digit) * c.n() + gate,
                        ) << (digit * bits))
                    });
                    assert_eq!(label, c.sigma(wire * c.n() + gate) as u128);
                }
            }
            assert!(z[3 * c.wire_words()..].iter().all(|x| x.is_zero()));
            assert!(
                public[c.public_blocks() * c.wire_words()..]
                    .iter()
                    .all(|x| x.is_zero())
            );
            if bits > 1 {
                assert_ne!(field.multiply(2, 2), 2 & 2);
            }
            let high = 1u128 << (bits - 1);
            assert!((0..128).any(|j| c.input(j, 0) & high != 0));
            assert!((0..128).any(|j| c.input(j, 0) & high == 0));
        }
    }

    #[test]
    fn dimensions_and_transcript_distinguish_widths() {
        for bits in [0, 3, 31, 65, 129, 256] {
            assert!(Circuit::new(14, bits, 97).is_err());
        }
        assert!(Circuit::new(13, 1, 97).is_err());
        assert!(Circuit::new(31, 1, 97).is_err());
        let mut last = None;
        for bits in [1, 2, 4, 8, 16, 32, 64, 128] {
            let mut c = Circuit::new(30, bits, 97).unwrap();
            assert_eq!(c.tag_bits(), (32usize + bits).next_power_of_two());
            assert_eq!(c.copy_kappa(), (128 / c.tag_bits()).max(1));
            assert_eq!(c.wire_words(), (1usize << 30) * bits / 128);
            if let Some(previous) = last {
                assert_ne!(previous, c.digest());
            }
            last = Some(c.digest());
            if bits < 128 {
                c.io = 1u128 << bits;
                assert!(c.validate().is_err());
            } else {
                c.io = u128::MAX;
                assert!(c.validate().is_ok());
            }
        }
    }
}
