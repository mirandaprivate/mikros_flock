//! Independent native hash calls with an explicit public-I/O address map.
//!
//! The SHA-256 branch reuses the previous fixture verbatim. The I/O map is
//! indexed by witness word columns, not by an assumed I/O prefix or schema
//! order: native BLAKE3 stores its output words between its input regions.

use std::array::from_fn;

use flock_core::{
    circuit::{Cell, Circuit},
    lincheck::LincheckCircuit,
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
use flock_hash::blake3_compress;
use flock_prover::r1cs_hashes::{blake3, keccak, sha2};

use crate::e2e_common::{FIXTURE_SEED, PreparedWitness, TreeFixture, TreeVerifier};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HashKind {
    Sha256,
    Keccak,
    Blake3,
}

impl HashKind {
    pub fn parse(value: &str) -> Result<Self, String> {
        match value {
            "sha256" | "sha-256" => Ok(Self::Sha256),
            "keccak" | "keccak-f1600" => Ok(Self::Keccak),
            "blake3" => Ok(Self::Blake3),
            _ => Err(format!("unsupported hash kind {value:?}")),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sha256 => "sha256",
            Self::Keccak => "keccak",
            Self::Blake3 => "blake3",
        }
    }

    pub fn unit(self) -> &'static str {
        match self {
            Self::Sha256 => "SHA-256 compression with standard IV; no message padding",
            Self::Keccak => "Keccak-f[1600] permutation; 24 rounds; no sponge padding",
            Self::Blake3 => {
                "BLAKE3 compression; standard IV, counter=0, block_len=64, flags=11; full 512-bit output"
            }
        }
    }

    /// Variable message/state input bits, excluding public IV/parameters.
    pub fn input_bits(self) -> usize {
        match self {
            Self::Sha256 | Self::Blake3 => 512,
            Self::Keccak => 1600,
        }
    }

    pub fn output_bits(self) -> usize {
        match self {
            Self::Sha256 => 256,
            Self::Keccak => 1600,
            Self::Blake3 => 512,
        }
    }
}

/// A public word's exact location in the native batch-major witness.
/// Its word address is `(word_col << nu) + row`. `public_index` is an
/// index in `HashFixture::public`, not a circuit cell-slot number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PublicWordBinding {
    pub word_col: usize,
    pub row: usize,
    pub public_index: usize,
}

enum Records {
    Sha256(Vec<([u32; 8], [u32; 16])>),
    Keccak(Vec<keccak::Lanes>),
    Blake3(Vec<blake3::Compression>),
}

pub struct HashFixture {
    pub kind: HashKind,
    pub nu: usize,
    pub registry: Registry,
    pub r1cs: BlockR1cs,
    pub circuit: Circuit,
    pub public: Vec<F128>,
    /// All public witness columns, in ascending `word_col` order.
    pub io_word_columns: Vec<usize>,
    /// Every public witness word, including shared IV and parameter words.
    pub public_bindings: Vec<PublicWordBinding>,
    records: Records,
}

impl HashFixture {
    pub fn new(kind: HashKind, count: usize) -> Self {
        assert!((1..=1 << 19).contains(&count));
        match kind {
            HashKind::Sha256 => Self::sha256(count),
            HashKind::Blake3 => Self::blake3(count),
            HashKind::Keccak => Self::keccak(count),
        }
    }

    fn sha256(count: usize) -> Self {
        let old = TreeFixture::independent_compressions(count);
        let schema = old.registry.types()[0].io_schema.clone();
        let public_bindings = (0..count)
            .flat_map(|row| {
                (0..8).map(move |word_col| PublicWordBinding {
                    word_col,
                    row,
                    public_index: match word_col {
                        0 | 1 => word_col,
                        2 | 3 => 2 + 4 * count + 2 * row + word_col - 2,
                        _ => 2 + 4 * row + word_col - 4,
                    },
                })
            })
            .collect();
        let fixture = Self {
            kind: HashKind::Sha256,
            nu: old.nu,
            registry: old.registry,
            r1cs: old.r1cs,
            circuit: old.circuit,
            public: old.public,
            io_word_columns: sorted_columns(&schema),
            public_bindings,
            records: Records::Sha256(old.compressions),
        };
        fixture.check_public_map();
        fixture
    }

    fn blake3(count: usize) -> Self {
        // k_log=14 plus nu>=8 reaches the smallest native PCS shape m22.
        let nu = (count.next_power_of_two().ilog2() as usize).max(8);
        let mut rng = Rng::new(FIXTURE_SEED);
        const FLAGS: u32 = (1 << 0) | (1 << 1) | (1 << 3);
        let records: Vec<blake3::Compression> = (0..count)
            .map(|_| (blake3::BLAKE3_IV, from_fn(|_| rng.next_u32()), 0, 64, FLAGS))
            .collect();
        let mut public = pack_u32_words(&blake3::BLAKE3_IV);
        public.extend(pack_u32_words(&[0, 0, 64, FLAGS]));
        for (_, message, _, _, _) in &records {
            public.extend(pack_u32_words(message));
        }
        for (cv, message, counter, block_len, flags) in &records {
            public.extend(pack_u32_words(&blake3_compress(
                cv, message, *counter, *block_len, *flags,
            )));
        }
        let schema = blake3::io_schema();
        let mut public_bindings = Vec::with_capacity(count * schema.len());
        // Use native schema positions for circuit cells and native witness
        // columns for the Mikros public opening. They are different orders.
        let mut classes = vec![Vec::new(); public.len()];
        for (index, class) in classes.iter_mut().enumerate() {
            class.push(public_cell(index, nu, schema.len()));
        }
        for row in 0..count {
            for (slot, io) in schema.iter().enumerate() {
                let public_index = match io.word_col {
                    0 | 1 => io.word_col,
                    2 | 3 => 3 + 4 * count + 4 * row + io.word_col - 2,
                    4..=7 => 3 + 4 * row + io.word_col - 4,
                    8 => 2,
                    9 | 10 => 3 + 4 * count + 4 * row + 2 + io.word_col - 9,
                    _ => panic!("unexpected native BLAKE3 I/O column"),
                };
                classes[public_index].push(Cell::new(slot, row));
                public_bindings.push(PublicWordBinding {
                    word_col: io.word_col,
                    row,
                    public_index,
                });
            }
        }
        let r1cs = blake3::build_block_r1cs(nu);
        assert!(r1cs.c0_is_identity());
        let registry = Registry::new(
            vec![TableType::from_block_r1cs(&r1cs).with_io_schema(schema.clone())],
            nu,
        );
        let circuit = Circuit::new(&registry, vec![count], public.len(), classes)
            .expect("independent BLAKE3 compression wiring must be valid");
        let _ = r1cs.csc_lincheck_circuit();
        let fixture = Self {
            kind: HashKind::Blake3,
            nu,
            registry,
            r1cs,
            circuit,
            public,
            io_word_columns: sorted_columns(&schema),
            public_bindings,
            records: Records::Blake3(records),
        };
        fixture.check_public_map();
        fixture
    }

    fn keccak(count: usize) -> Self {
        let nu = (count.next_power_of_two().ilog2() as usize).max(7);
        let mut rng = Rng::new(FIXTURE_SEED);
        let records: Vec<keccak::Lanes> = (0..count).map(|_| from_fn(|_| rng.next_u64())).collect();
        let mut public = Vec::with_capacity(26 * count);
        for lanes in &records {
            public.extend(pack_lanes(lanes));
        }
        for lanes in &records {
            let mut output = *lanes;
            keccak::keccak_f1600(&mut output);
            public.extend(pack_lanes(&output));
        }
        let schema = keccak::io_schema();
        let mut public_bindings = Vec::with_capacity(count * schema.len());
        let mut wires = Vec::with_capacity(count * schema.len());
        for row in 0..count {
            for (slot, io) in schema.iter().enumerate() {
                let public_index = match io.word_col {
                    0..=12 => 13 * row + io.word_col,
                    16..=28 => 13 * count + 13 * row + io.word_col - 16,
                    _ => panic!("unexpected native Keccak I/O column"),
                };
                public_bindings.push(PublicWordBinding {
                    word_col: io.word_col,
                    row,
                    public_index,
                });
                wires.push(vec![
                    public_cell(public_index, nu, schema.len()),
                    Cell::new(slot, row),
                ]);
            }
        }
        let r1cs = keccak::build_block_r1cs(nu);
        assert!(r1cs.c0_is_identity());
        let registry = Registry::new(
            vec![TableType::from_block_r1cs(&r1cs).with_io_schema(schema.clone())],
            nu,
        );
        let circuit = Circuit::new(&registry, vec![count], public.len(), wires)
            .expect("independent Keccak permutation wiring must be valid");
        // This compact native relation has implicit A/B matrices. Its shared
        // public transpose walker is mandatory; do not construct a CSC cache
        // from the descriptor's empty matrix stubs.
        let fixture = Self {
            kind: HashKind::Keccak,
            nu,
            registry,
            r1cs,
            circuit,
            public,
            io_word_columns: sorted_columns(&schema),
            public_bindings,
            records: Records::Keccak(records),
        };
        fixture.check_public_map();
        fixture
    }

    pub fn count(&self) -> usize {
        match &self.records {
            Records::Sha256(records) => records.len(),
            Records::Keccak(records) => records.len(),
            Records::Blake3(records) => records.len(),
        }
    }

    pub fn fixture_name(&self) -> &'static str {
        match self.kind {
            HashKind::Sha256 => "sha256-independent-compressions",
            HashKind::Keccak => "keccak-independent-permutations",
            HashKind::Blake3 => "blake3-independent-compressions",
        }
    }

    pub fn verifier(&self) -> TreeVerifier<'_> {
        TreeVerifier {
            nu: self.nu,
            r1cs: &self.r1cs,
            registry: &self.registry,
            circuit: &self.circuit,
            public: &self.public,
            live_rows: self.count(),
        }
    }

    pub fn union(&self) -> UnionInstance<'_> {
        UnionInstance::new(&self.registry, vec![self.count()])
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
        match &self.records {
            Records::Sha256(records) => {
                sha2::generate_witness_batch_major_partial(records, self.nu)
            }
            Records::Keccak(records) => {
                let states: Vec<_> = records.iter().map(keccak::lanes_to_state).collect();
                keccak::generate_witness_batch_major_partial(&states, self.nu)
            }
            Records::Blake3(records) => {
                blake3::generate_witness_batch_major_partial(records, self.nu)
            }
        }
    }

    pub fn lincheck_circuit(&self) -> &dyn LincheckCircuit {
        match self.kind {
            HashKind::Keccak => &keccak::KeccakLincheckCircuit,
            _ => self.r1cs.csc_lincheck_circuit(),
        }
    }

    pub fn matrix_authentication(&self) -> &'static str {
        match self.kind {
            HashKind::Keccak => {
                "public native compact A/B matrix walker; C=I; complete matrix check; restored FLOCK encoder"
            }
            _ => "public native A/B CSC; C=I; complete matrix check",
        }
    }

    /// Keep the established SHA-256 input/statement receipt byte-for-byte.
    pub fn fixture_digest_blake3(&self) -> String {
        let bytes = match &self.records {
            Records::Sha256(records) => bincode::serialize(&(records, &self.public)).unwrap(),
            Records::Keccak(records) => bincode::serialize(&(records, &self.public)).unwrap(),
            Records::Blake3(records) => bincode::serialize(&(records, &self.public)).unwrap(),
        };
        ::blake3::hash(&bytes).to_hex().to_string()
    }

    fn check_public_map(&self) {
        let mut seen = vec![false; self.io_word_columns.len() * self.count()];
        for binding in &self.public_bindings {
            assert!(binding.row < self.count());
            assert!(binding.public_index < self.public.len());
            let column = self
                .io_word_columns
                .binary_search(&binding.word_col)
                .expect("bound column is in native I/O schema");
            let address = column * self.count() + binding.row;
            assert!(!seen[address], "duplicate public witness word");
            seen[address] = true;
        }
        assert!(
            seen.into_iter().all(|bound| bound),
            "all native I/O words must be public"
        );
    }
}

fn sorted_columns(schema: &[IoWord]) -> Vec<usize> {
    let mut columns: Vec<_> = schema.iter().map(|word| word.word_col).collect();
    columns.sort_unstable();
    assert!(columns.windows(2).all(|pair| pair[0] != pair[1]));
    columns
}

fn public_cell(word: usize, nu: usize, gate_slots: usize) -> Cell {
    Cell::new(gate_slots + (word >> nu), word & ((1 << nu) - 1))
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

fn pack_lanes(lanes: &keccak::Lanes) -> Vec<F128> {
    // State lanes occupy the low 1600 bits; the unused high half of the
    // final 128-bit word is constrained zero by the native relation.
    lanes
        .chunks(2)
        .map(|pair| F128::new(pair[0], pair.get(1).copied().unwrap_or(0)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_witness_and_public(fixture: &HashFixture) {
        let (z, a, b, _) = fixture.prepare_witness();
        assert!(fixture.r1cs.c0_is_identity());
        for ((z, a), b) in z.iter().zip(&a).zip(&b) {
            assert_eq!(z.lo, a.lo & b.lo);
            assert_eq!(z.hi, a.hi & b.hi);
        }
        for binding in &fixture.public_bindings {
            assert_eq!(
                z[(binding.word_col << fixture.nu) + binding.row],
                fixture.public[binding.public_index]
            );
        }
        let bit = crate::e2e_common::bit_wiring::BitWiringFixture::new(
            &fixture.circuit,
            &z,
            &fixture.public,
        )
        .unwrap();
        assert!(
            bit.sigma
                .iter()
                .enumerate()
                .all(|(i, &j)| bit.flock_values[i] == bit.flock_values[j])
        );
    }

    #[test]
    fn sha256_reuses_the_historical_fixture_exactly() {
        let old = TreeFixture::independent_compressions(256);
        let fixture = HashFixture::new(HashKind::Sha256, 256);
        assert_eq!(fixture.public, old.public);
        assert_eq!(fixture.circuit.digest(), old.circuit.digest());
        assert_eq!(fixture.registry.digest(), old.registry.digest());
        assert_eq!(fixture.prepare_witness(), old.prepare_witness());
        check_witness_and_public(&fixture);
    }

    #[test]
    fn blake3_binds_inputs_params_and_both_output_halves() {
        let fixture = HashFixture::new(HashKind::Blake3, 256);
        assert_eq!(fixture.io_word_columns, (0..11).collect::<Vec<_>>());
        assert_eq!(fixture.public_bindings.len(), 11 * 256);
        assert_eq!(fixture.public.len(), 3 + 8 * 256);
        let Records::Blake3(records) = &fixture.records else {
            unreachable!()
        };
        for (row, (_, message, counter, block_len, flags)) in records.iter().enumerate() {
            assert_eq!((*counter, *block_len, *flags), (0, 64, 11));
            let message_bytes: Vec<_> =
                message.iter().flat_map(|word| word.to_le_bytes()).collect();
            let mut bytes = [0u8; 64];
            ::blake3::Hasher::new()
                .update(&message_bytes)
                .finalize_xof()
                .fill(&mut bytes);
            let expected: Vec<u32> = bytes
                .chunks_exact(4)
                .map(|word| u32::from_le_bytes(word.try_into().unwrap()))
                .collect();
            assert_eq!(
                &fixture.public[3 + 4 * 256 + 4 * row..3 + 4 * 256 + 4 * row + 4],
                pack_u32_words(&expected)
            );
        }
        check_witness_and_public(&fixture);
    }

    #[test]
    fn keccak_binds_all_1600_input_and_output_bits_across_layout_holes() {
        let fixture = HashFixture::new(HashKind::Keccak, 256);
        assert_eq!(
            fixture.io_word_columns,
            (0..13).chain(16..29).collect::<Vec<_>>()
        );
        assert_eq!(fixture.public_bindings.len(), 26 * 256);
        assert_eq!(fixture.public.len(), 26 * 256);
        assert_eq!(
            fixture.lincheck_circuit().const_pin_col(),
            Some(keccak::Z_CONST)
        );
        let Records::Keccak(records) = &fixture.records else {
            unreachable!()
        };
        for (row, lanes) in records.iter().enumerate() {
            let mut state = keccak::lanes_to_state(lanes);
            keccak::keccak_f(&mut state);
            let output = pack_lanes(&keccak::state_to_lanes(&state));
            assert_eq!(
                &fixture.public[13 * 256 + 13 * row..13 * 256 + 13 * (row + 1)],
                output
            );
            assert_eq!(fixture.public[13 * row + 12].hi, 0);
            assert_eq!(output[12].hi, 0);
        }
        check_witness_and_public(&fixture);
    }
}
