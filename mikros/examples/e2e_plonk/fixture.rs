//! A connected, regular AND/XOR circuit with exactly N genuine gate equations.
//! Gate 2j is a_j AND b_j; gate 2j+1 is its output XOR a_(j+1 mod N/2).
//! Shared a inputs connect consecutive gadgets. The last output is public.
//! Random input generation, packing, selectors and wiring are all offline.

use flock_core::{
    pcs::{
        PcsParams,
        ligerito::{LigeritoProfile, embedded_initial_k_or_default},
    },
    r1cs::SparseBinaryMatrix,
    schedule::{Registry, TableClass, TableType},
    union::UnionInstance,
};
use flock_field::F128;
use mikros_permutation::bits::PackedBits;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Single,
    Multiple,
}
impl Mode {
    pub fn name(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Multiple => "multiple",
        }
    }
    pub fn instances(self) -> usize {
        if self == Self::Single { 1 } else { 128 }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Circuit {
    pub log_gates: usize,
    pub mode: Mode,
    pub seed: u64,
    pub io: F128,
}
impl Circuit {
    pub fn new(log_gates: usize, mode: Mode, seed: u64) -> Self {
        assert!((14..=30).contains(&log_gates));
        let mut c = Self {
            log_gates,
            mode,
            seed,
            io: F128::ZERO,
        };
        let out = c.word(2, c.n() - 1);
        c.io = if mode == Mode::Single {
            F128::new(out.lo & 1, 0)
        } else {
            out
        };
        c
    }
    pub fn n(&self) -> usize {
        1usize << self.log_gates
    }
    pub fn wire_words(&self) -> usize {
        self.n() * self.mode.instances() / 128
    }
    pub fn h(&self) -> usize {
        self.log_gates + 2
    }
    pub fn copy_len(&self) -> usize {
        4 * self.n()
    }
    pub fn live_len(&self) -> usize {
        3 * self.n() + 1
    }
    pub fn public_blocks(&self) -> usize {
        if self.mode == Mode::Single {
            5 + 3 * self.h()
        } else {
            1
        }
    }
    pub fn public_words(&self) -> usize {
        (if self.mode == Mode::Single {
            self.public_blocks().next_power_of_two() * self.wire_words()
        } else {
            self.n()
        })
        .max(1 << 15)
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
    pub fn sigma(&self, index: usize) -> usize {
        let n = self.n();
        if index >= 3 * n {
            return if index == 3 * n { 3 * n - 1 } else { index };
        }
        let g = index % n;
        match index / n {
            0 if g & 1 == 0 => n + ((g + n - 1) & (n - 1)),
            0 => 2 * n + g - 1,
            1 if g & 1 == 0 => index,
            1 => (g + 1) & (n - 1),
            2 if g & 1 == 0 => g + 1,
            2 if g == n - 1 => 3 * n,
            2 => index,
            _ => unreachable!(),
        }
    }
    fn input(&self, j: usize, lane: u64) -> F128 {
        let k = (j as u64).wrapping_mul(0xD6E8FEB86659FD93) ^ self.seed ^ lane;
        F128::new(mix(k), mix(k ^ 0x9E3779B97F4A7C15))
    }
    /// All 128 independent instance bits at one physical gate position.
    pub fn word(&self, wire: usize, g: usize) -> F128 {
        let j = g / 2;
        let a = self.input(j, 0);
        let b = self.input(j, 0x5A17DA7A);
        let c = F128::new(a.lo & b.lo, a.hi & b.hi);
        if g & 1 == 0 {
            match wire {
                0 => a,
                1 => b,
                2 => c,
                _ => panic!("wire"),
            }
        } else {
            let next = self.input((j + 1) & (self.n() / 2 - 1), 0);
            match wire {
                0 => c,
                1 => next,
                2 => c + next,
                _ => panic!("wire"),
            }
        }
    }
    pub fn digest(&self) -> [u8; 32] {
        let mut h = blake3::Hasher::new();
        h.update(b"connected-alternating-and-xor-plonk-v1");
        h.update(&(self.log_gates as u64).to_le_bytes());
        h.update(&(self.mode.instances() as u64).to_le_bytes());
        // Seed identifies reproducible input generation, not a verifier oracle.
        h.update(&self.io.lo.to_le_bytes());
        h.update(&self.io.hi.to_le_bytes());
        *h.finalize().as_bytes()
    }
    pub fn witness(&self) -> Vec<F128> {
        let m = self.wire_words();
        let mut z = vec![F128::ZERO; (4 * m).max(1 << 15)];
        z[..3 * m]
            .par_chunks_mut(m)
            .enumerate()
            .for_each(|(wire, out)| {
                out.par_iter_mut().enumerate().for_each(|(i, v)| {
                    if self.mode == Mode::Multiple {
                        *v = self.word(wire, i);
                    } else {
                        let mut lo = 0;
                        let mut hi = 0;
                        for b in 0..128 {
                            let x = self.word(wire, i * 128 + b).lo & 1;
                            if b < 64 {
                                lo |= x << b;
                            } else {
                                hi |= x << (b - 64);
                            }
                        }
                        *v = F128::new(lo, hi);
                    }
                });
            });
        z
    }
    pub fn public(&self) -> Vec<F128> {
        let mut q = vec![F128::ZERO; self.public_words()];
        if self.mode == Mode::Multiple {
            q[..self.n()].par_iter_mut().enumerate().for_each(|(g, v)| {
                let mut bits = 0u128;
                for k in 0..5 {
                    bits |= (Self::selector(k, g) as u128) << k;
                }
                for wire in 0..3 {
                    bits |= (self.sigma(wire * self.n() + g) as u128) << (5 + wire * self.h());
                }
                *v = from_raw(bits);
            });
        } else {
            let m = self.wire_words();
            for selector in 0..5 {
                let byte = (0..8).fold(0u8, |v, b| v | ((Self::selector(selector, b) as u8) << b));
                let word = u64::from_le_bytes([byte; 8]);
                q[selector * m..(selector + 1) * m]
                    .par_iter_mut()
                    .for_each(|v| *v = F128::new(word, word));
            }
            for wire in 0..3 {
                let bits = PackedBits::from_fn_parallel(self.n(), self.h(), |g| {
                    self.sigma(wire * self.n() + g) as u128
                })
                .unwrap();
                let begin = (5 + wire * self.h()) * m;
                bytes_mut(&mut q[begin..begin + self.h() * m]).copy_from_slice(bits.bytes());
            }
        }
        q
    }
    pub fn sigma_table(&self) -> Vec<usize> {
        (0..self.copy_len())
            .into_par_iter()
            .map(|i| self.sigma(i))
            .collect()
    }
    pub fn single_copy_source(&self, z: &[F128], side: usize) -> PackedBits {
        assert_eq!(self.mode, Mode::Single);
        let d = (self.h() + 1).next_power_of_two();
        PackedBits::from_fn_parallel(self.copy_len(), d, |i| {
            if i >= self.live_len() {
                return 0;
            }
            let label = if side == 0 { self.sigma(i) } else { i };
            let bit = if i == 3 * self.n() {
                self.io.lo & 1
            } else {
                bit(z, i) as u64
            };
            (label as u128) | ((bit as u128) << (d - 1))
        })
        .unwrap()
    }
}
fn mix(mut x: u64) -> u64 {
    x ^= x >> 30;
    x = x.wrapping_mul(0xBF58476D1CE4E5B9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94D049BB133111EB);
    x ^ (x >> 31)
}
pub fn from_raw(x: u128) -> F128 {
    F128::new(x as u64, (x >> 64) as u64)
}
pub fn basis() -> Vec<F128> {
    (0..128).map(|b| from_raw(1u128 << b)).collect()
}
pub fn bit(z: &[F128], i: usize) -> bool {
    let w = z[i / 128];
    if i % 128 < 64 {
        (w.lo >> (i % 128)) & 1 != 0
    } else {
        (w.hi >> (i % 64)) & 1 != 0
    }
}
pub fn bytes(z: &[F128]) -> &[u8] {
    assert!(cfg!(target_endian = "little"));
    unsafe { std::slice::from_raw_parts(z.as_ptr().cast(), std::mem::size_of_val(z)) }
}
pub fn bytes_mut(z: &mut [F128]) -> &mut [u8] {
    assert!(cfg!(target_endian = "little"));
    unsafe { std::slice::from_raw_parts_mut(z.as_mut_ptr().cast(), std::mem::size_of_val(z)) }
}

/// Full native rectangular container, with gate rows low and source blocks high.
/// The commitment's packed-word order and bytes are unchanged.
pub fn registry(log_words: usize, log_rows: usize) -> Registry {
    assert!(log_words >= 15);
    assert!(log_rows <= log_words);
    let k_log = 7 + log_words - log_rows;
    Registry::new(
        vec![TableType {
            k_log,
            useful_bits: 1usize << k_log,
            a_0: SparseBinaryMatrix::new(0, 0, vec![]),
            b_0: SparseBinaryMatrix::new(0, 0, vec![]),
            c_0: SparseBinaryMatrix::new(0, 0, vec![]),
            const_pin: None,
            class: TableClass::Boolean,
            io_schema: vec![],
        }],
        log_rows,
    )
}
pub fn union(registry: &Registry) -> UnionInstance<'_> {
    UnionInstance::new(registry, vec![1usize << registry.nu()])
}
pub fn parameters(u: &UnionInstance<'_>) -> PcsParams {
    let m = u.dense_m();
    let profile = LigeritoProfile::Fast;
    PcsParams {
        m,
        log_inv_rate: profile.log_inv_rate(),
        log_batch_size: embedded_initial_k_or_default(m, profile),
        profile,
        num_lanes: u.commit_lanes(embedded_initial_k_or_default(m, profile)),
        merkle_hash: Default::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_gates_wiring_and_public_tables() {
        for mode in [Mode::Single, Mode::Multiple] {
            let c = Circuit::new(14, mode, 12);
            let z = c.witness();
            let q = c.public();
            assert_eq!(
                c.n() * mode.instances(),
                if mode == Mode::Single {
                    1 << 14
                } else {
                    1 << 21
                }
            );
            for i in 0..c.copy_len() {
                assert_eq!(c.sigma(c.sigma(i)), i);
            }
            for g in 0..c.n() {
                let [a, b, o] = std::array::from_fn(|w| c.word(w, g));
                assert_eq!(
                    o,
                    if g & 1 == 0 {
                        F128::new(a.lo & b.lo, a.hi & b.hi)
                    } else {
                        a + b
                    }
                );
                for s in 0..5 {
                    let index = if mode == Mode::Single {
                        s * c.n() + g
                    } else {
                        128 * g + s
                    };
                    assert_eq!(bit(&q, index), Circuit::selector(s, g));
                }
                for w in 0..3 {
                    for b in 0..c.h() {
                        let i = if mode == Mode::Single {
                            (5 + w * c.h() + b) * c.n() + g
                        } else {
                            128 * g + 5 + w * c.h() + b
                        };
                        assert_eq!(bit(&q, i), c.sigma(w * c.n() + g) & (1 << b) != 0);
                    }
                }
            }
            let val = |i: usize| {
                if i == 3 * c.n() {
                    c.io
                } else if mode == Mode::Single {
                    F128::new(bit(&z, i) as u64, 0)
                } else {
                    z[i]
                }
            };
            for i in 0..c.live_len() {
                assert_eq!(val(i), val(c.sigma(i)));
            }
        }
    }
}
