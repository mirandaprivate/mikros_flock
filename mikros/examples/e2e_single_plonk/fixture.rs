//! Compile one native hash relation to binary three-wire PLONK.
//!
//! The public descriptor contains no witness. Compilation and witness conversion
//! are offline. XOR sharing and constant simplifications depend exclusively on
//! the public relation, never on witness values. SHA256/BLAKE3 use direct binary
//! compression circuits, and Keccak uses its factored forward recurrence. The
//! optional MIKROS_PLONK_COMPILER_REFERENCE=1 route expands SHA/BLAKE sparse
//! R1CS rows for auditing; it is deliberately not the default benchmark path.

use std::collections::HashMap;

use crate::{
    e2e_common::PreparedWitness,
    hash_fixture::{HashFixture, HashKind},
};
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
use flock_prover::r1cs_hashes::{blake3 as blake3_native, keccak, sha2};
use mikros_permutation::bits::PackedBits;
use serde::{Deserialize, Serialize};

/// Public PLONK statement. Selectors are packed in little-endian bit order.
/// A gate satisfies qL*a + qR*b + qM*a*b + qO*c + qC = 0 over F2.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Circuit {
    pub log_gates: usize,
    pub io: Vec<bool>,
    pub gate_count: usize,
    pub native_bits: usize,
    pub hash_kind: String,
    pub selectors: [Vec<u8>; 5],
    /// Cycles over three wire columns followed by individual public bits.
    /// All unused addresses in the fourth column are fixed points.
    pub sigma: Vec<usize>,
    /// Cached offline. Rebuild after deliberately changing public structure.
    pub structure_digest: [u8; 32],
}

impl Circuit {
    pub fn n(&self) -> usize {
        1usize << self.log_gates
    }
    pub fn wire_words(&self) -> usize {
        self.n() / 128
    }
    pub fn h(&self) -> usize {
        self.log_gates + 2
    }
    pub fn copy_len(&self) -> usize {
        4 * self.n()
    }
    pub fn live_len(&self) -> usize {
        3 * self.n() + self.io.len()
    }
    pub fn witness_words(&self) -> usize {
        (4 * self.wire_words()).max(1 << 15)
    }
    pub fn public_words(&self) -> usize {
        ((5 + 3 * self.h()).next_power_of_two() * self.wire_words()).max(1 << 15)
    }
    pub fn gate_public_words(&self) -> usize {
        (8 * self.wire_words()).max(1 << 15)
    }
    /// Public-wiring mode authenticates selectors only; sigma is evaluated
    /// directly from the public circuit by the verifier.
    pub fn gate_public(&self) -> Vec<F128> {
        let m = self.wire_words();
        let mut q = vec![F128::ZERO; self.gate_public_words()];
        for (k, selector) in self.selectors.iter().enumerate() {
            bytes_mut(&mut q[k * m..(k + 1) * m]).copy_from_slice(selector);
        }
        q
    }
    pub fn sigma(&self, i: usize) -> usize {
        self.sigma[i]
    }
    pub fn selector(&self, which: usize, gate: usize) -> bool {
        self.selectors[which][gate / 8] >> (gate % 8) & 1 != 0
    }
    pub fn public(&self) -> Vec<F128> {
        let m = self.wire_words();
        let mut q = vec![F128::ZERO; self.public_words()];
        for (k, selector) in self.selectors.iter().enumerate() {
            bytes_mut(&mut q[k * m..(k + 1) * m]).copy_from_slice(selector);
        }
        for wire in 0..3 {
            let packed = PackedBits::from_fn_parallel(self.n(), self.h(), |g| {
                self.sigma(wire * self.n() + g) as u128
            })
            .expect("valid fixed-width sigma");
            let start = (5 + wire * self.h()) * m;
            bytes_mut(&mut q[start..start + self.h() * m]).copy_from_slice(packed.bytes());
        }
        q
    }
    pub fn digest(&self) -> [u8; 32] {
        let mut h = blake3::Hasher::new();
        h.update(b"native-single-hash-binary-plonk-compiler-v1");
        h.update(&self.structure_digest);
        h.update(&(self.io.len() as u64).to_le_bytes());
        let packed: Vec<u8> = self
            .io
            .chunks(8)
            .map(|chunk| {
                chunk
                    .iter()
                    .enumerate()
                    .fold(0, |v, (j, &b)| v | ((b as u8) << j))
            })
            .collect();
        h.update(&packed);
        *h.finalize().as_bytes()
    }
    /// Offline public preprocessing. I/O values are deliberately excluded here
    /// and rebound on each digest() call, including during I/O tamper tests.
    pub fn rebuild_structure_digest(&mut self) {
        let public = (
            &self.hash_kind,
            self.log_gates,
            self.gate_count,
            self.native_bits,
            &self.selectors,
            &self.sigma,
        );
        self.structure_digest =
            *blake3::hash(&bincode::serialize(&public).expect("serialize public structure"))
                .as_bytes();
    }
    pub fn single_copy_source(&self, z: &[F128], side: usize) -> PackedBits {
        assert!(side < 2);
        assert_eq!(z.len(), self.witness_words());
        let d = (self.h() + 1).next_power_of_two();
        PackedBits::from_fn_parallel(self.copy_len(), d, |i| {
            if i >= self.live_len() {
                return 0;
            }
            let label = if side == 0 { self.sigma(i) } else { i };
            let value = if i >= 3 * self.n() {
                self.io[i - 3 * self.n()]
            } else {
                bit(z, i)
            };
            (label as u128) | ((value as u128) << (d - 1))
        })
        .expect("valid copy source shape")
    }

    /// Independent cleartext constraint check, used only in offline preparation
    /// and tests. The cryptographic verifier does not call this witness reader.
    pub fn validate_assignment(&self, z: &[F128]) -> Result<(), String> {
        if !(7..=30).contains(&self.log_gates)
            || z.len() != self.witness_words()
            || self.io.len() > self.n()
            || self.sigma.len() != self.copy_len()
            || self.selectors.iter().any(|s| s.len() != self.n() / 8)
        {
            return Err("PLONK assignment shape".into());
        }
        let n = self.n();
        for g in 0..n {
            let [a, b, c] = std::array::from_fn(|w| bit(z, w * n + g));
            let q = |k| self.selector(k, g);
            if (q(0) & a) ^ (q(1) & b) ^ (q(2) & a & b) ^ (q(3) & c) ^ q(4) {
                return Err(format!("PLONK gate {g}"));
            }
        }
        let value = |i| {
            if i < 3 * n {
                bit(z, i)
            } else {
                self.io[i - 3 * n]
            }
        };
        let mut seen = vec![false; self.copy_len()];
        for (i, &s) in self.sigma.iter().enumerate() {
            if s >= self.copy_len() || seen[s] {
                return Err("copy sigma is not a permutation".into());
            }
            seen[s] = true;
            if i < self.live_len() {
                if s >= self.live_len() || value(i) != value(s) {
                    return Err(format!("copy edge {i}->{s}"));
                }
            } else if s != i {
                return Err("dead copy address is not fixed".into());
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct Gate {
    wires: [usize; 3],
    selectors: u8,
}

struct Builder {
    values: Vec<bool>,
    zero: usize,
    one: usize,
    gates: Vec<Gate>,
    xor_nodes: HashMap<(usize, usize), usize>,
    and_nodes: HashMap<(usize, usize), usize>,
}

impl Builder {
    fn new(mut values: Vec<bool>, one: usize) -> Result<Self, String> {
        if values.get(one) != Some(&true) {
            return Err("native constant-one wire is not one".into());
        }
        let zero = values.len();
        values.push(false);
        let mut b = Self {
            values,
            zero,
            one,
            gates: Vec::new(),
            xor_nodes: HashMap::new(),
            and_nodes: HashMap::new(),
        };
        // A genuine zero constraint anchors all unused/padding wire slots.
        b.gates.push(Gate {
            wires: [zero; 3],
            selectors: 1 << 3,
        });
        Ok(b)
    }
    fn xor(&mut self, a: usize, b: usize) -> usize {
        if a == b {
            return self.zero;
        }
        if a == self.zero {
            return b;
        }
        if b == self.zero {
            return a;
        }
        let key = if a < b { (a, b) } else { (b, a) };
        if let Some(&out) = self.xor_nodes.get(&key) {
            return out;
        }
        let out = self.values.len();
        self.values.push(self.values[a] ^ self.values[b]);
        let gate = if a == self.one || b == self.one {
            // Public constant pin permits qL*a + qO*c + qC = 0.
            Gate {
                wires: [if a == self.one { b } else { a }, self.zero, out],
                selectors: 0b11001,
            }
        } else {
            Gate {
                wires: [a, b, out],
                selectors: 0b01011,
            }
        };
        self.gates.push(gate);
        self.xor_nodes.insert(key, out);
        out
    }
    fn xor_all(&mut self, ids: &[usize]) -> usize {
        ids.iter().fold(self.zero, |sum, &id| self.xor(sum, id))
    }
    fn and(&mut self, a: usize, b: usize) -> usize {
        if a == self.zero || b == self.zero {
            return self.zero;
        }
        if a == self.one || a == b {
            return b;
        }
        if b == self.one {
            return a;
        }
        let key = if a < b { (a, b) } else { (b, a) };
        if let Some(&out) = self.and_nodes.get(&key) {
            return out;
        }
        let out = self.values.len();
        self.values.push(self.values[a] & self.values[b]);
        self.gates.push(Gate {
            wires: [a, b, out],
            selectors: 0b01100,
        });
        self.and_nodes.insert(key, out);
        out
    }
    fn or(&mut self, a: usize, b: usize) -> usize {
        if a == self.one || b == self.one {
            return self.one;
        }
        if a == self.zero || a == b {
            return b;
        }
        if b == self.zero {
            return a;
        }
        let out = self.values.len();
        self.values.push(self.values[a] | self.values[b]);
        // a OR b = a + b + a*b over F2, in one allowed PLONK gate.
        self.gates.push(Gate {
            wires: [a, b, out],
            selectors: 0b01111,
        });
        out
    }
    fn majority(&mut self, a: usize, b: usize, c: usize) -> usize {
        if a == b || a == c {
            return a;
        }
        if b == c {
            return b;
        }
        if a == self.zero {
            return self.and(b, c);
        }
        if b == self.zero {
            return self.and(a, c);
        }
        if c == self.zero {
            return self.and(a, b);
        }
        if a == self.one {
            return self.or(b, c);
        }
        if b == self.one {
            return self.or(a, c);
        }
        if c == self.one {
            return self.or(a, b);
        }
        // maj(a,b,c) = a + (a+b)*(a+c); the first XOR is shared
        // with the corresponding sum bit in a ripple adder.
        let ab = self.xor(a, b);
        let ac = self.xor(a, c);
        let p = self.and(ab, ac);
        self.xor(a, p)
    }
    /// Canonical binary-trie decomposition of a sorted GF2 support. Repeated
    /// subintervals in the native dense linear forms share actual XOR gates.
    /// Unlike a value-based simplifier, this is identical for every witness.
    fn support_sorted(&mut self, ids: &[usize]) -> usize {
        match ids.len() {
            0 => self.zero,
            1 => ids[0],
            _ => {
                let bit = (ids[0] ^ ids[ids.len() - 1]).ilog2();
                let split = ids.partition_point(|id| id & (1usize << bit) == 0);
                debug_assert!(split > 0 && split < ids.len());
                let a = self.support_sorted(&ids[..split]);
                let b = self.support_sorted(&ids[split..]);
                self.xor(a, b)
            }
        }
    }
    fn support(&mut self, ids: &[usize]) -> usize {
        if ids.windows(2).all(|w| w[0] < w[1]) {
            return self.support_sorted(ids);
        }
        let mut canonical = ids.to_vec();
        canonical.sort_unstable();
        let mut reduced = Vec::with_capacity(canonical.len());
        for id in canonical {
            if reduced.last() == Some(&id) {
                reduced.pop();
            } else {
                reduced.push(id);
            }
        }
        self.support_sorted(&reduced)
    }
    fn equal(&mut self, a: usize, out: usize) -> Result<(), String> {
        if self.values[a] != self.values[out] {
            return Err(format!("native linear relation at wire {out}"));
        }
        if a != out {
            self.gates.push(Gate {
                wires: [a, self.zero, out],
                selectors: 0b01001,
            });
        }
        Ok(())
    }
    fn product(&mut self, a: usize, b: usize, out: usize) -> Result<(), String> {
        if self.values[a] & self.values[b] != self.values[out] {
            return Err(format!("native product relation at wire {out}"));
        }
        if a == self.zero || b == self.zero {
            self.gates.push(Gate {
                wires: [self.zero, self.zero, out],
                selectors: 0b01000,
            });
        } else if a == self.one || a == b {
            self.equal(b, out)?;
        } else if b == self.one {
            self.equal(a, out)?;
        } else {
            self.gates.push(Gate {
                wires: [a, b, out],
                selectors: 0b01100,
            });
        }
        Ok(())
    }
    #[cfg(test)]
    fn finish(
        self,
        public: &[(usize, bool)],
        native_bits: usize,
        kind: &str,
    ) -> Result<(Circuit, Vec<F128>), String> {
        self.finish_with_minimum(public, native_bits, kind, 128)
    }
    fn finish_with_minimum(
        mut self,
        public: &[(usize, bool)],
        native_bits: usize,
        kind: &str,
        minimum_gates: usize,
    ) -> Result<(Circuit, Vec<F128>), String> {
        // Even a structurally unused public input must have an explicit copy
        // endpoint in the witness; a tautological equality supplies it.
        let mut used = vec![false; self.values.len()];
        for gate in &self.gates {
            for &w in &gate.wires {
                used[w] = true;
            }
        }
        for &(id, value) in public {
            if self.values.get(id) != Some(&value) {
                return Err(format!("native public I/O mismatch at wire {id}"));
            }
            if !used[id] {
                self.gates.push(Gate {
                    wires: [id, self.zero, id],
                    selectors: 0b01001,
                });
                used[id] = true;
            }
        }
        let gate_count = self.gates.len();
        let n = gate_count
            .max(public.len())
            .max(minimum_gates)
            .max(128)
            .next_power_of_two();
        if n > 1usize << 30 {
            return Err("compiled circuit exceeds supported gate count".into());
        }
        let mut c = Circuit {
            log_gates: n.ilog2() as usize,
            io: public.iter().map(|(_, v)| *v).collect(),
            gate_count,
            native_bits,
            hash_kind: kind.into(),
            selectors: std::array::from_fn(|_| vec![0; n / 8]),
            sigma: (0..4 * n).collect(),
            structure_digest: [0; 32],
        };
        let mut z = vec![F128::ZERO; c.witness_words()];
        // first/last form cycles without storing a second 3N occurrence table.
        let mut first = vec![usize::MAX; self.values.len()];
        let mut last = first.clone();
        let mut append = |id: usize, address: usize, sigma: &mut [usize]| {
            if first[id] == usize::MAX {
                first[id] = address;
            } else {
                sigma[last[id]] = address;
            }
            last[id] = address;
        };
        for g in 0..n {
            let gate = self.gates.get(g).copied().unwrap_or(Gate {
                wires: [self.zero; 3],
                selectors: 0,
            });
            for k in 0..5 {
                c.selectors[k][g / 8] |= ((gate.selectors >> k) & 1) << (g % 8);
            }
            for (w, &id) in gate.wires.iter().enumerate() {
                let address = w * n + g;
                if self.values[id] {
                    set_bit(&mut z, address);
                }
                append(id, address, &mut c.sigma);
            }
        }
        for (j, &(id, _)) in public.iter().enumerate() {
            append(id, 3 * n + j, &mut c.sigma);
        }
        drop(append);
        for (id, &start) in first.iter().enumerate() {
            if start != usize::MAX {
                c.sigma[last[id]] = start;
            }
        }
        c.rebuild_structure_digest();
        c.validate_assignment(&z)?;
        Ok((c, z))
    }
}

fn native_bits(words: &[F128], nu: usize, k: usize) -> Result<Vec<bool>, String> {
    let expected = (k / 128) << nu;
    if words.len() < expected {
        return Err("short native packed witness".into());
    }
    Ok((0..k)
        .map(|i| bit(words, ((i / 128) << nu) * 128 + i % 128))
        .collect())
}

/// Offline compilation from exactly the native fixture and its row-zero witness.
/// Public IV, parameters, input, output, I/O padding, and the constant-one wire
/// all become explicit public endpoints of the nontrivial copy permutation.
pub fn compile(
    f: &HashFixture,
    prepared: &PreparedWitness,
) -> Result<(Circuit, Vec<F128>), String> {
    if f.count() != 1 || !f.r1cs.c0_is_identity() {
        return Err("single native C=I relation required".into());
    }
    let k = f.r1cs.k();
    let values = native_bits(&prepared.0, f.nu, k)?;
    let a_values = native_bits(&prepared.1, f.nu, k)?;
    let b_values = native_bits(&prepared.2, f.nu, k)?;
    if (0..k).any(|i| (a_values[i] & b_values[i]) != values[i]) {
        return Err("native A*B=Z check failed".into());
    }
    let one = f.r1cs.const_pin.ok_or("native constant pin required")?;
    let mut b = Builder::new(values, one)?;
    let reference = std::env::var_os("MIKROS_PLONK_COMPILER_REFERENCE").is_some_and(|v| v == "1");
    if f.kind == HashKind::Keccak {
        compile_keccak(&mut b, &a_values, &b_values)?;
    } else if reference {
        if f.r1cs.a_0.rows.len() != k || f.r1cs.b_0.rows.len() != k {
            return Err("native sparse matrix shape".into());
        }
        for i in 0..k {
            let a = b.support(&f.r1cs.a_0.rows[i]);
            let right = b.support(&f.r1cs.b_0.rows[i]);
            if b.values[a] != a_values[i] || b.values[right] != b_values[i] {
                return Err(format!("native matrix/witness mismatch at row {i}"));
            }
            b.product(a, right, i)?;
        }
    } else {
        match f.kind {
            HashKind::Sha256 => compile_sha256(&mut b)?,
            HashKind::Blake3 => compile_blake3(&mut b)?,
            HashKind::Keccak => unreachable!(),
        }
    }
    let mut public = Vec::with_capacity(f.public_bindings.len() * 128 + 1);
    for binding in &f.public_bindings {
        if binding.row != 0 || binding.public_index >= f.public.len() || binding.word_col >= k / 128
        {
            return Err("single public binding shape".into());
        }
        for j in 0..128 {
            public.push((
                binding.word_col * 128 + j,
                bit(&f.public[binding.public_index..=binding.public_index], j),
            ));
        }
    }
    public.push((one, true));
    // Section 4 single-instance theorem: m=N/kappa >= max(kappa,q,n_io).
    // Here kappa=128, q=2, and every explicitly copied public bit is an I/O.
    let minimum_gates = 128 * public.len().max(128);
    b.finish_with_minimum(&public, k, f.kind.as_str(), minimum_gates)
}

type Word = [usize; 32];

fn word(base: usize) -> Word {
    std::array::from_fn(|i| base + i)
}
fn constant_word(b: &Builder, value: u32) -> Word {
    std::array::from_fn(|i| if value >> i & 1 == 0 { b.zero } else { b.one })
}
fn rotr(a: Word, shift: usize) -> Word {
    std::array::from_fn(|i| a[(i + shift) % 32])
}
fn shr(b: &Builder, a: Word, shift: usize) -> Word {
    std::array::from_fn(|i| if i + shift < 32 { a[i + shift] } else { b.zero })
}
fn xor_word(b: &mut Builder, a: Word, c: Word) -> Word {
    std::array::from_fn(|i| b.xor(a[i], c[i]))
}
fn xor_three(b: &mut Builder, a: Word, c: Word, d: Word) -> Word {
    let ac = xor_word(b, a, c);
    xor_word(b, ac, d)
}
/// Exact binary addition mod 2^32. Every generated sum/carry is constrained.
/// The discarded carry beyond bit 31 is never allocated.
fn add_word(b: &mut Builder, a: Word, c: Word) -> Word {
    let mut carry = b.zero;
    std::array::from_fn(|i| {
        let ac = b.xor(a[i], c[i]);
        let sum = b.xor(ac, carry);
        if i < 31 {
            carry = b.majority(a[i], c[i], carry);
        }
        sum
    })
}
fn add_words(b: &mut Builder, words: &[Word]) -> Word {
    words[1..]
        .iter()
        .fold(words[0], |sum, &w| add_word(b, sum, w))
}
fn equal_word(b: &mut Builder, value: Word, output: Word) -> Result<(), String> {
    for i in 0..32 {
        b.equal(value[i], output[i])?;
    }
    Ok(())
}

/// FIPS SHA-256 compression with arbitrary public chaining value and message.
/// Only input IDs and fixed algorithm constants feed newly generated wires;
/// the eliminated native R1CS intermediate witness is deliberately not read.
fn compile_sha256(b: &mut Builder) -> Result<(), String> {
    let initial: [Word; 8] = std::array::from_fn(|i| word(sha2::h_bit(i, 0)));
    let mut schedule: Vec<Word> = (0..16).map(|i| word(sha2::m_bit(i, 0))).collect();
    for t in 16..64 {
        let x = schedule[t - 15];
        let y = schedule[t - 2];
        let sx = shr(b, x, 3);
        let sy = shr(b, y, 10);
        let s0 = xor_three(b, rotr(x, 7), rotr(x, 18), sx);
        let s1 = xor_three(b, rotr(y, 17), rotr(y, 19), sy);
        let next = add_words(b, &[s1, schedule[t - 7], s0, schedule[t - 16]]);
        schedule.push(next);
    }
    let mut state = initial;
    for r in 0..64 {
        let [a, x, c, d, e, f, g, h] = state;
        let s1 = xor_three(b, rotr(e, 6), rotr(e, 11), rotr(e, 25));
        let ch = std::array::from_fn(|i| {
            let fg = b.xor(f[i], g[i]);
            let p = b.and(e[i], fg);
            b.xor(g[i], p)
        });
        let round_constant = constant_word(b, sha2::SHA256_K[r]);
        let t1 = add_words(b, &[h, s1, ch, round_constant, schedule[r]]);
        let s0 = xor_three(b, rotr(a, 2), rotr(a, 13), rotr(a, 22));
        let maj = std::array::from_fn(|i| b.majority(a[i], x[i], c[i]));
        let t2 = add_word(b, s0, maj);
        let a_next = add_word(b, t1, t2);
        let e_next = add_word(b, d, t1);
        state = [a_next, a, x, c, e_next, e, f, g];
    }
    for i in 0..8 {
        let result = add_word(b, initial[i], state[i]);
        equal_word(b, result, word(sha2::h_out_bit(i, 0)))?;
    }
    Ok(())
}

/// Native BLAKE3 compression, including all public counter/length/flag bits and
/// both output halves. Algorithmic IV[0..4] is fixed by BLAKE3 itself; the
/// chaining value and compression parameters remain genuine circuit inputs.
fn compile_blake3(b: &mut Builder) -> Result<(), String> {
    let cv: [Word; 8] = std::array::from_fn(|i| word(blake3_native::CV_BASE + 32 * i));
    let mut message: [Word; 16] = std::array::from_fn(|i| word(blake3_native::M_BASE + 32 * i));
    let mut state: [Word; 16] = std::array::from_fn(|i| match i {
        0..=7 => cv[i],
        8..=11 => constant_word(b, blake3_native::BLAKE3_IV[i - 8]),
        _ => word(blake3_native::T_LO_BASE + (i - 12) * 32),
    });
    for r in 0..blake3_native::N_ROUNDS {
        for g in 0..blake3_native::N_G_PER_ROUND {
            let [a, x, c, d] = blake3_native::G_LANES[g];
            let [mx, my] = blake3_native::G_MSG_IDX[g];
            state[a] = add_words(b, &[state[a], state[x], message[mx]]);
            state[d] = rotr(xor_word(b, state[d], state[a]), 16);
            state[c] = add_word(b, state[c], state[d]);
            state[x] = rotr(xor_word(b, state[x], state[c]), 12);
            state[a] = add_words(b, &[state[a], state[x], message[my]]);
            state[d] = rotr(xor_word(b, state[d], state[a]), 8);
            state[c] = add_word(b, state[c], state[d]);
            state[x] = rotr(xor_word(b, state[x], state[c]), 7);
        }
        if r + 1 < blake3_native::N_ROUNDS {
            message = std::array::from_fn(|i| message[blake3_native::MSG_PERMUTATION[i]]);
        }
    }
    for i in 0..8 {
        let low = xor_word(b, state[i], state[i + 8]);
        let high = xor_word(b, state[i + 8], cv[i]);
        equal_word(b, low, word(blake3_native::OUT_LO_BASE + 32 * i))?;
        equal_word(b, high, word(blake3_native::OUT_HI_BASE + 32 * i))?;
    }
    Ok(())
}

/// Exactly the compact native Keccak relation, with the implicit linear forms
/// factored into a public XOR DAG. The source preimage API determines all bit
/// ordering and rotation conventions; no second Keccak rotation table is kept.
fn compile_keccak(b: &mut Builder, a_values: &[bool], b_values: &[bool]) -> Result<(), String> {
    let mut covered = vec![false; keccak::K];
    covered[keccak::Z_CONST] = true;
    if !a_values[keccak::Z_CONST] || !b_values[keccak::Z_CONST] {
        return Err("Keccak constant matrix row".into());
    }
    let mut state: Vec<_> = (0..keccak::STATE_BITS)
        .map(|j| {
            let id = keccak::z_pos_state(0, j);
            covered[id] = true;
            id
        })
        .collect();
    for &id in &state {
        if a_values[id] != b.values[id] || !b_values[id] {
            return Err(format!("Keccak input matrix row {id}"));
        }
    }
    for r in 0..keccak::N_ROUNDS {
        let mut phi = vec![b.zero; keccak::STATE_BITS];
        for z in 0..64 {
            for y in 0..5 {
                for x in 0..5 {
                    let pre = keccak::theta_rho_pi_preimage(x, y, z);
                    let left: Vec<_> = pre[1..6].iter().map(|&i| state[i]).collect();
                    let right: Vec<_> = pre[6..11].iter().map(|&i| state[i]).collect();
                    let left = b.xor_all(&left);
                    let right = b.xor_all(&right);
                    let delta = b.xor(left, right);
                    phi[keccak::state_idx(x, y, z)] = b.xor(state[pre[0]], delta);
                }
            }
        }
        let mut next = vec![b.zero; keccak::STATE_BITS];
        for z in 0..64 {
            for y in 0..5 {
                for x in 0..5 {
                    let j = keccak::state_idx(x, y, z);
                    let id = keccak::z_pos_t(r, j);
                    covered[id] = true;
                    let a = b.xor(b.one, phi[keccak::state_idx((x + 1) % 5, y, z)]);
                    let right = phi[keccak::state_idx((x + 2) % 5, y, z)];
                    if b.values[a] != a_values[id] || b.values[right] != b_values[id] {
                        return Err(format!("Keccak implicit matrix row {id}"));
                    }
                    b.product(a, right, id)?;
                    next[j] = b.xor(phi[j], id);
                    if x == 0 && y == 0 && (keccak::ROUND_CONSTANTS[r] >> z) & 1 != 0 {
                        next[j] = b.xor(next[j], b.one);
                    }
                }
            }
        }
        state = next;
    }
    for (j, &value) in state.iter().enumerate() {
        let id = keccak::z_pos_state(24, j);
        covered[id] = true;
        if b.values[value] != a_values[id] || !b_values[id] {
            return Err(format!("Keccak output matrix row {id}"));
        }
        b.equal(value, id)?;
    }
    for (id, used) in covered.into_iter().enumerate() {
        if !used {
            if a_values[id] || b_values[id] {
                return Err(format!("Keccak padding matrix row {id}"));
            }
            b.product(b.zero, b.zero, id)?;
        }
    }
    Ok(())
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
fn set_bit(z: &mut [F128], i: usize) {
    if i % 128 < 64 {
        z[i / 128].lo |= 1u64 << (i % 128);
    } else {
        z[i / 128].hi |= 1u64 << (i % 64);
    }
}
pub fn bytes(z: &[F128]) -> &[u8] {
    assert!(cfg!(target_endian = "little"));
    // F128 is the repository's repr(C) two-u64 packed-bit storage.
    unsafe { std::slice::from_raw_parts(z.as_ptr().cast(), std::mem::size_of_val(z)) }
}
pub fn bytes_mut(z: &mut [F128]) -> &mut [u8] {
    assert!(cfg!(target_endian = "little"));
    unsafe { std::slice::from_raw_parts_mut(z.as_mut_ptr().cast(), std::mem::size_of_val(z)) }
}
pub fn registry(log_words: usize, block_log_words: usize) -> Registry {
    assert!(log_words >= 15 && block_log_words <= log_words);
    let k_log = 7 + log_words - block_log_words;
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
        block_log_words,
    )
}
pub fn union(registry: &Registry) -> UnionInstance<'_> {
    UnionInstance::new(registry, vec![1usize << registry.nu()])
}
pub fn parameters(u: &UnionInstance<'_>) -> PcsParams {
    let m = u.dense_m();
    let profile = LigeritoProfile::Fast;
    let k = embedded_initial_k_or_default(m, profile);
    PcsParams {
        m,
        log_inv_rate: profile.log_inv_rate(),
        log_batch_size: k,
        profile,
        num_lanes: u.commit_lanes(k),
        merkle_hash: Default::default(),
    }
}

#[cfg(test)]
pub fn tiny_fixture() -> (Circuit, Vec<F128>) {
    let mut b = Builder::new(vec![true, false, true], 2).unwrap();
    let sum = b.xor(0, 1);
    let inverse = b.xor(sum, b.one);
    let product = b.values.len();
    b.values.push(false);
    b.product(0, 1, product).unwrap();
    b.finish(
        &[
            (0, true),
            (1, false),
            (sum, true),
            (product, false),
            (2, true),
            (inverse, false),
        ],
        3,
        "tiny",
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_plonk_tiny_public_tables_and_copy_cycles() {
        let (c, z) = tiny_fixture();
        c.validate_assignment(&z).unwrap();
        let public = c.public();
        assert_eq!(c.n(), 128);
        assert!(c.sigma.iter().enumerate().any(|(i, &s)| i != s));
        for g in 0..c.n() {
            for k in 0..5 {
                assert_eq!(bit(&public, k * c.n() + g), c.selector(k, g));
            }
            for w in 0..3 {
                for j in 0..c.h() {
                    assert_eq!(
                        bit(&public, (5 + w * c.h() + j) * c.n() + g),
                        c.sigma(w * c.n() + g) & (1 << j) != 0
                    );
                }
            }
        }
        for j in 0..c.io.len() {
            let mut bad = c.clone();
            bad.io[j] ^= true;
            assert_ne!(bad.digest(), c.digest(), "current public bit is bound");
            assert!(bad.validate_assignment(&z).is_err(), "public bit {j}");
        }
        for i in [0, c.n(), 2 * c.n()] {
            let mut bad = z.clone();
            bad[i / 128] += from_raw(1u128 << (i % 128));
            assert!(c.validate_assignment(&bad).is_err());
        }
        let mut bad = c.clone();
        bad.sigma[0] = bad.sigma[1];
        assert!(bad.validate_assignment(&z).is_err());
    }

    #[test]
    fn single_plonk_support_sharing_is_witness_independent() {
        let mut a = Builder::new(vec![false, true, false, true, true], 4).unwrap();
        let mut b = Builder::new(vec![true, false, true, false, true], 4).unwrap();
        for support in [&[0, 1, 2, 3][..], &[0, 1], &[0, 1, 0, 2, 2, 3], &[]] {
            let x = a.support(support);
            let y = b.support(support);
            assert_eq!(x, y);
            assert_eq!(
                a.values[x],
                support.iter().fold(false, |v, &i| v ^ a.values[i])
            );
            assert_eq!(
                b.values[y],
                support.iter().fold(false, |v, &i| v ^ b.values[i])
            );
        }
        assert_eq!(
            a.gates
                .iter()
                .map(|g| (g.wires, g.selectors))
                .collect::<Vec<_>>(),
            b.gates
                .iter()
                .map(|g| (g.wires, g.selectors))
                .collect::<Vec<_>>()
        );
    }

    fn assert_builder_gates(b: &Builder) {
        for (g, gate) in b.gates.iter().enumerate() {
            let [a, x, c] = gate.wires.map(|i| b.values[i]);
            let q = |i: usize| ((gate.selectors >> i) & 1u8) != 0u8;
            assert!(
                !((q(0) & a) ^ (q(1) & x) ^ (q(2) & a & x) ^ (q(3) & c) ^ q(4)),
                "gate {g}"
            );
        }
    }
    fn put_word(values: &mut [bool], base: usize, x: u32) {
        for i in 0..32 {
            values[base + i] = x >> i & 1 != 0;
        }
    }
    fn read_word(b: &Builder, ids: Word) -> u32 {
        ids.iter()
            .enumerate()
            .fold(0, |x, (i, &id)| x | ((b.values[id] as u32) << i))
    }
    fn random_word(seed: &mut u64) -> u32 {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        *seed as u32
    }

    #[test]
    fn single_plonk_direct_add_and_majority_match_integer_oracles() {
        let mut seed = 0x8471_5353_0000_3210;
        let mut inputs = vec![
            (0, 0),
            (u32::MAX, 1),
            (u32::MAX, u32::MAX),
            (0x7fff_ffff, 1),
        ];
        for _ in 0..48 {
            inputs.push((random_word(&mut seed), random_word(&mut seed)));
        }
        for (a, x) in inputs {
            let mut values = vec![false; 65];
            put_word(&mut values, 0, a);
            put_word(&mut values, 32, x);
            values[64] = true;
            let mut b = Builder::new(values, 64).unwrap();
            let result = add_word(&mut b, word(0), word(32));
            assert_eq!(read_word(&b, result), a.wrapping_add(x));
            let fixed = constant_word(&b, x);
            let result = add_word(&mut b, word(0), fixed);
            assert_eq!(read_word(&b, result), a.wrapping_add(x));
            assert_builder_gates(&b);
        }
        for a in [false, true] {
            for x in [false, true] {
                for c in [false, true] {
                    let mut b = Builder::new(vec![a, x, c, true], 3).unwrap();
                    let out = b.majority(0, 1, 2);
                    assert_eq!(b.values[out], (a & x) ^ (a & c) ^ (x & c));
                    assert_builder_gates(&b);
                }
            }
        }
    }

    #[test]
    fn single_plonk_direct_hashes_match_native_oracles_on_independent_inputs() {
        let mut seed = 0x2184_5782_5263_9019;
        for case in 0..3 {
            let message: [u32; 16] = std::array::from_fn(|_| match case {
                0 => 0,
                1 => u32::MAX,
                _ => random_word(&mut seed),
            });
            let cv: [u32; 8] = std::array::from_fn(|_| random_word(&mut seed));
            let sha_output = sha2::sha256_compress(&cv, &message);
            let mut sha_values = vec![false; sha2::K];
            sha_values[sha2::Z_CONST_POS] = true;
            for i in 0..8 {
                put_word(&mut sha_values, sha2::h_bit(i, 0), cv[i]);
                put_word(&mut sha_values, sha2::h_out_bit(i, 0), sha_output[i]);
            }
            for i in 0..16 {
                put_word(&mut sha_values, sha2::m_bit(i, 0), message[i]);
            }
            // All eliminated native carry/choice/etc. slots are zero here.
            // A successful circuit must derive its own intermediates solely
            // from inputs, not accidentally read the old R1CS witness.
            let mut b = Builder::new(sha_values.clone(), sha2::Z_CONST_POS).unwrap();
            compile_sha256(&mut b).unwrap();
            assert_builder_gates(&b);
            assert!(b.gates.len() < 200_000);
            sha_values[sha2::h_out_bit(7, 31)] ^= true;
            let mut bad = Builder::new(sha_values, sha2::Z_CONST_POS).unwrap();
            assert!(compile_sha256(&mut bad).is_err());

            let params: [u32; 4] = std::array::from_fn(|_| random_word(&mut seed));
            let counter = u64::from(params[0]) | (u64::from(params[1]) << 32);
            let output = flock_hash::blake3_compress(&cv, &message, counter, params[2], params[3]);
            let mut values = vec![false; blake3_native::K];
            values[blake3_native::Z_CONST_POS] = true;
            for i in 0..8 {
                put_word(&mut values, blake3_native::CV_BASE + 32 * i, cv[i]);
                put_word(&mut values, blake3_native::OUT_LO_BASE + 32 * i, output[i]);
                put_word(
                    &mut values,
                    blake3_native::OUT_HI_BASE + 32 * i,
                    output[8 + i],
                );
            }
            for i in 0..16 {
                put_word(&mut values, blake3_native::M_BASE + 32 * i, message[i]);
            }
            for i in 0..4 {
                put_word(&mut values, blake3_native::T_LO_BASE + 32 * i, params[i]);
            }
            let mut b = Builder::new(values.clone(), blake3_native::Z_CONST_POS).unwrap();
            compile_blake3(&mut b).unwrap();
            assert_builder_gates(&b);
            assert!(b.gates.len() < 100_000);
            values[blake3_native::OUT_HI_BASE + 255] ^= true;
            let mut bad = Builder::new(values, blake3_native::Z_CONST_POS).unwrap();
            assert!(compile_blake3(&mut bad).is_err());
        }
    }

    #[test]
    fn single_plonk_three_native_hashes_compile_and_bind_every_public_bit() {
        for kind in [HashKind::Sha256, HashKind::Keccak, HashKind::Blake3] {
            let f = HashFixture::new(kind, 1);
            let prepared = f.prepare_witness();
            let (c, z) = compile(&f, &prepared).unwrap();
            c.validate_assignment(&z).unwrap();
            assert_eq!(c.io.len(), 128 * f.public_bindings.len() + 1);
            assert!(c.gate_count <= c.n());
            assert!(c.n() / 128 >= c.io.len().max(128));
            assert!(c.sigma.iter().enumerate().any(|(i, &s)| i != s));
            for j in 0..c.io.len() {
                let address = 3 * c.n() + j;
                // Every public bit has a cycle edge to a witness occurrence.
                assert_ne!(c.sigma(address), address);
                assert!(c.sigma(address) < 3 * c.n());
                assert_eq!(bit(&z, c.sigma(address)), c.io[j]);
            }
            let mut bad = c.clone();
            bad.io[0] ^= true;
            assert!(bad.validate_assignment(&z).is_err());
            let mut wrong_witness = z.clone();
            wrong_witness[0] += F128::ONE;
            assert!(c.validate_assignment(&wrong_witness).is_err());
            eprintln!(
                "single_plonk_fixture hash={} native_bits={} gates={} padded_gates={} io_bits={} witness_words={} public_words={}",
                kind.as_str(),
                c.native_bits,
                c.gate_count,
                c.n(),
                c.io.len(),
                c.witness_words(),
                c.public_words()
            );
        }
    }
}
