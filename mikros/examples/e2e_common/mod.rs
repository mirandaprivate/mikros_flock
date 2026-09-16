//! Shared, deterministic SHA-256 tree fixture for end-to-end comparisons.
//!
//! The circuit and its word schema follow flock-prover/tests/circuit_wiring.rs.
//! Leaves are public 512-bit messages; every internal gate compresses its two
//! children's outputs, and the public root is checked against native SHA-256.

#[allow(dead_code)]
pub mod batched_boolean;
pub mod column_boolean;
#[allow(dead_code)]
pub mod bit_wiring;
#[allow(dead_code)]
pub mod flock_boolean;

use std::array::from_fn;

use flock_core::{
    circuit::{Cell, Circuit},
    pcs::{
        PcsParams,
        ligerito::{LigeritoProfile, embedded_initial_k_or_default},
    },
    r1cs::BlockR1cs,
    schedule::{IoWord, Registry, TableType},
    test_rng::Rng,
    union::UnionInstance,
};
use flock_field::F128;
use flock_prover::r1cs_hashes::sha2::{
    SHA256_IV, build_block_r1cs, generate_witness_batch_major_partial, sha256_compress,
};

pub const DOMAIN: &[u8] = b"mikros-flock-e2e-sha256-tree-v1";
pub const FIXTURE_SEED: u64 = 0x5a20_454e_4432_454e;
pub type PreparedWitness = (Vec<F128>, Vec<F128>, Vec<F128>, Vec<u8>);

pub struct TreeFixture {
    pub independent: bool,
    pub log_leaves: usize,
    pub nu: usize,
    pub registry: Registry,
    pub r1cs: BlockR1cs,
    pub circuit: Circuit,
    pub compressions: Vec<([u32; 8], [u32; 16])>,
    pub public: Vec<F128>,
    pub root: [u32; 8],
}

/// Public verifier view: deliberately excludes compressions and witness buffers.
pub struct TreeVerifier<'a> {
    pub nu: usize,
    pub r1cs: &'a BlockR1cs,
    pub registry: &'a Registry,
    pub circuit: &'a Circuit,
    pub public: &'a [F128],
    pub live_rows: usize,
}

impl TreeVerifier<'_> {
    pub fn union(&self) -> UnionInstance<'_> {
        UnionInstance::new(self.registry, vec![self.live_rows])
    }
}

impl TreeFixture {
    pub fn fixture_name(&self) -> &'static str {
        if self.independent {
            "sha256-independent-compressions"
        } else {
            "sha256-binary-tree"
        }
    }

    /// Exactly `count` independent compression calls, each with the standard
    /// IV and a public 512-bit message. Every 256-bit output is public and
    /// linked to its own compression row; there are no tree parent calls.
    /// These are unpadded compression blocks, not complete SHA-256 hashes.
    pub fn independent_compressions(count: usize) -> Self {
        assert!((1..=1 << 19).contains(&count));
        let nu = (count.next_power_of_two().ilog2() as usize).max(7);
        let mut rng = Rng::new(FIXTURE_SEED);
        let compressions: Vec<_> = (0..count)
            .map(|_| (SHA256_IV, from_fn(|_| rng.next_u32())))
            .collect();
        let outputs: Vec<_> = compressions
            .iter()
            .map(|(h, m)| sha256_compress(h, m))
            .collect();
        let mut public = pack_u32_words(&SHA256_IV);
        for (_, message) in &compressions {
            public.extend(pack_u32_words(message));
        }
        for output in &outputs {
            public.extend(pack_u32_words(output));
        }
        let mut wires = Vec::new();
        for slot in 0..2 {
            let mut class = vec![public_cell(slot, nu)];
            class.extend((0..count).map(|row| Cell::new(slot, row)));
            wires.push(class);
        }
        for row in 0..count {
            for word in 0..4 {
                wires.push(vec![
                    public_cell(2 + 4 * row + word, nu),
                    Cell::new(2 + word, row),
                ]);
            }
            for word in 0..2 {
                wires.push(vec![
                    Cell::new(6 + word, row),
                    public_cell(2 + 4 * count + 2 * row + word, nu),
                ]);
            }
        }
        let r1cs = build_block_r1cs(nu);
        let registry = Registry::new(
            vec![TableType::from_block_r1cs(&r1cs).with_io_schema(vec![
                IoWord::input(0),
                IoWord::input(1),
                IoWord::input(4),
                IoWord::input(5),
                IoWord::input(6),
                IoWord::input(7),
                IoWord::output(2),
                IoWord::output(3),
            ])],
            nu,
        );
        let circuit = Circuit::new(&registry, vec![count], public.len(), wires)
            .expect("independent SHA256 compression wiring must be valid");
        let _ = r1cs.csc_lincheck_circuit();
        Self {
            independent: true,
            log_leaves: 0,
            nu,
            registry,
            r1cs,
            circuit,
            compressions,
            public,
            root: outputs[count - 1],
        }
    }

    pub fn verifier(&self) -> TreeVerifier<'_> {
        TreeVerifier {
            nu: self.nu,
            r1cs: &self.r1cs,
            registry: &self.registry,
            circuit: &self.circuit,
            public: &self.public,
            live_rows: self.compressions.len(),
        }
    }
    pub fn new(log_leaves: usize) -> Self {
        // m = SHA256 K_LOG(15) + nu. Seven row bits reach the smallest
        // published PCS configuration, m22, even for a tiny tree.
        assert!(log_leaves <= 19, "the registered PCS shapes end at m35");
        let nu = (log_leaves + 1).max(7);
        let leaves = 1usize << log_leaves;
        let n_gates = 2 * leaves - 1;
        let mut rng = Rng::new(FIXTURE_SEED);
        let mut compressions = Vec::with_capacity(n_gates);
        let mut outputs = Vec::with_capacity(n_gates);
        let mut public = pack_u32_words(&SHA256_IV);
        for _ in 0..leaves {
            let message = from_fn(|_| rng.next_u32());
            public.extend(pack_u32_words(&message));
            outputs.push(sha256_compress(&SHA256_IV, &message));
            compressions.push((SHA256_IV, message));
        }
        let mut children = Vec::with_capacity(leaves - 1);
        let (mut start, mut count) = (0, leaves);
        while count > 1 {
            for i in 0..count / 2 {
                let (left, right) = (start + 2 * i, start + 2 * i + 1);
                let mut message = [0; 16];
                message[..8].copy_from_slice(&outputs[left]);
                message[8..].copy_from_slice(&outputs[right]);
                outputs.push(sha256_compress(&SHA256_IV, &message));
                compressions.push((SHA256_IV, message));
                children.push((left, right));
            }
            start += count;
            count /= 2;
        }
        let root = outputs[n_gates - 1];
        public.extend(pack_u32_words(&root));

        let mut wires = Vec::new();
        for (word, slot) in [(0, 0), (1, 1)] {
            let mut class = vec![public_cell(word, nu)];
            class.extend((0..n_gates).map(|row| Cell::new(slot, row)));
            wires.push(class);
        }
        for i in 0..leaves {
            for word in 0..4 {
                wires.push(vec![
                    public_cell(2 + 4 * i + word, nu),
                    Cell::new(2 + word, i),
                ]);
            }
        }
        for (parent, &(left, right)) in children.iter().enumerate() {
            let row = leaves + parent;
            for (source, output, input) in
                [(left, 6, 2), (left, 7, 3), (right, 6, 4), (right, 7, 5)]
            {
                wires.push(vec![Cell::new(output, source), Cell::new(input, row)]);
            }
        }
        for word in 0..2 {
            wires.push(vec![
                Cell::new(6 + word, n_gates - 1),
                public_cell(2 + 4 * leaves + word, nu),
            ]);
        }
        let r1cs = build_block_r1cs(nu);
        let schema = vec![
            IoWord::input(0),
            IoWord::input(1),
            IoWord::input(4),
            IoWord::input(5),
            IoWord::input(6),
            IoWord::input(7),
            IoWord::output(2),
            IoWord::output(3),
        ];
        let registry = Registry::new(
            vec![TableType::from_block_r1cs(&r1cs).with_io_schema(schema)],
            nu,
        );
        let circuit = Circuit::new(&registry, vec![n_gates], public.len(), wires)
            .expect("SHA256 tree wiring must form a valid circuit");
        // Build the fixed CSC cache offline, before any measured proof.
        let _ = r1cs.csc_lincheck_circuit();
        assert_eq!(&public[2 + 4 * leaves..], pack_u32_words(&root));
        Self {
            independent: false,
            log_leaves,
            nu,
            registry,
            r1cs,
            circuit,
            compressions,
            public,
            root,
        }
    }

    pub fn union(&self) -> UnionInstance<'_> {
        UnionInstance::new(&self.registry, vec![self.compressions.len()])
    }

    pub fn pcs_params(&self, profile: LigeritoProfile) -> PcsParams {
        let union = self.union();
        let m = union.dense_m();
        let k = embedded_initial_k_or_default(m, profile);
        PcsParams {
            m,
            log_inv_rate: profile.log_inv_rate(),
            log_batch_size: k,
            profile,
            num_lanes: union.commit_lanes(k),
            merkle_hash: Default::default(),
        }
    }

    pub fn prepare_witness(&self) -> PreparedWitness {
        generate_witness_batch_major_partial(&self.compressions, self.nu)
    }
}

fn public_cell(word: usize, nu: usize) -> Cell {
    Cell::new(8 + (word >> nu), word & ((1 << nu) - 1))
}

fn pack_u32_words(words: &[u32]) -> Vec<F128> {
    assert!(words.len().is_multiple_of(4));
    words
        .chunks_exact(4)
        .map(|w| {
            F128::new(
                u64::from(w[0]) | (u64::from(w[1]) << 32),
                u64::from(w[2]) | (u64::from(w[3]) << 32),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn independent_128_compressions_bind_every_input_and_output() {
        let fixture = TreeFixture::independent_compressions(128);
        assert!(fixture.independent);
        assert_eq!(fixture.fixture_name(), "sha256-independent-compressions");
        assert_eq!(fixture.compressions.len(), 128);
        assert_eq!(fixture.circuit.counts(), &[128]);
        assert_eq!(fixture.nu, 7);
        assert_eq!(fixture.public.len(), 2 + 4 * 128 + 2 * 128);
        for (row, (h, m)) in fixture.compressions.iter().enumerate() {
            assert_eq!(*h, SHA256_IV);
            assert_eq!(
                &fixture.public[2 + 4 * row..2 + 4 * row + 4],
                pack_u32_words(m)
            );
            let start = 2 + 4 * 128 + 2 * row;
            assert_eq!(
                &fixture.public[start..start + 2],
                pack_u32_words(&sha256_compress(h, m))
            );
        }
        let witness = fixture.prepare_witness();
        let bit = bit_wiring::BitWiringFixture::new(&fixture.circuit, &witness.0, &fixture.public)
            .expect("valid independent compression bit wiring");
        assert!(
            bit.sigma
                .iter()
                .enumerate()
                .all(|(i, &j)| bit.flock_values[i] == bit.flock_values[j])
        );
        for row in [0, 64, 127] {
            let mut public = fixture.public.clone();
            public[2 + 4 * 128 + 2 * row] += F128::ONE;
            let bad =
                bit_wiring::BitWiringFixture::new(&fixture.circuit, &witness.0, &public).unwrap();
            assert!(
                bad.sigma
                    .iter()
                    .enumerate()
                    .any(|(i, &j)| bad.flock_values[i] != bad.flock_values[j]),
                "output {row} must participate in copy constraints"
            );
        }
    }
}
