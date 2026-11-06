//! Fixed-length SHA-256 leaf hashing with the native Binius commitment format.
//!
//! Only the prover's leaf implementation changes. Leaf digests, inner-node
//! compression and verifier hashing remain exactly those of `StdHashSuite`.

use std::mem::MaybeUninit;

use binius_hash::{
    ParallelDigest, ParallelSha256Digest, StdHashSuite, binary_merkle_tree::HashSuite,
};
use binius_utils::{
    FixedSizeSerializeBytes, SerializeBytes,
    rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator},
};
use sha2::{Sha256, block_api::compress256, digest::Output};

// The measured FRI input leaves contain 16 or 32 field words (256 or 512
// bytes). Bound per-task storage; larger termination leaves use the native
// streaming implementation instead of allocating a leaf-sized buffer.
const MAX_FIXED_LEAF_BYTES: usize = 1024;
const MAX_BLOCKS: usize = (MAX_FIXED_LEAF_BYTES + 9).div_ceil(64);
const SHA256_IV: [u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

#[derive(Debug, Clone, Default)]
pub struct FixedSha256HashSuite;

impl HashSuite for FixedSha256HashSuite {
    type LeafHash = <StdHashSuite as HashSuite>::LeafHash;
    type Compression = <StdHashSuite as HashSuite>::Compression;
    type ParCompression = <StdHashSuite as HashSuite>::ParCompression;
    type ParLeafHash = FixedSha256Digest;
}

#[derive(Debug, Clone, Default)]
pub struct FixedSha256Digest;

impl ParallelDigest for FixedSha256Digest {
    type Digest = Sha256;

    fn new() -> Self {
        Self
    }

    fn digest<I: IntoIterator<Item: SerializeBytes>>(
        &self,
        source: impl IndexedParallelIterator<Item = I>,
        out: &mut [MaybeUninit<Output<Sha256>>],
    ) {
        ParallelSha256Digest::new().digest(source, out);
    }

    fn digest_with_const_len<I: IntoIterator<Item: FixedSizeSerializeBytes>>(
        &self,
        n_items_per_input: usize,
        source: impl IndexedParallelIterator<Item = I>,
        out: &mut [MaybeUninit<Output<Sha256>>],
    ) {
        assert_eq!(
            source.len(),
            out.len(),
            "leaf count must match digest count"
        );
        let leaf_len = n_items_per_input
            .checked_mul(<I::Item as FixedSizeSerializeBytes>::BYTE_SIZE)
            .expect("fixed leaf byte length overflow");
        // Native already specializes a one-block padded leaf. Keep that path,
        // and retain its streaming implementation for large leaves.
        if !(56..=MAX_FIXED_LEAF_BYTES).contains(&leaf_len) {
            ParallelSha256Digest::new().digest_with_const_len(n_items_per_input, source, out);
            return;
        }

        let n_blocks = (leaf_len + 9).div_ceil(64);
        let mut template = [[0u8; 64]; MAX_BLOCKS];
        let bytes = template.as_flattened_mut();
        bytes[leaf_len] = 0x80;
        bytes[n_blocks * 64 - 8..n_blocks * 64]
            .copy_from_slice(&((leaf_len as u64) * 8).to_be_bytes());

        source
            .zip(out.par_iter_mut())
            .for_each_with(template, |blocks, (items, output)| {
                // Reuse a small task-local buffer. Each leaf overwrites only
                // its message prefix; the fixed SHA padding stays intact.
                let mut cursor = &mut blocks.as_flattened_mut()[..leaf_len];
                let mut count = 0;
                for item in items {
                    item.serialize(&mut cursor)
                        .expect("precondition: fixed-size items serialize without error");
                    count += 1;
                }
                assert_eq!(count, n_items_per_input, "fixed leaf item count mismatch");
                assert!(cursor.is_empty(), "fixed leaf serialized length mismatch");

                let mut state = SHA256_IV;
                compress256(&mut state, &blocks[..n_blocks]);
                let mut digest = Output::<Sha256>::default();
                for (bytes, word) in digest.chunks_exact_mut(4).zip(state) {
                    bytes.copy_from_slice(&word.to_be_bytes());
                }
                output.write(digest);
            });
    }
}

#[cfg(test)]
mod tests {
    use binius_field::BinaryField128bGhash;
    use binius_utils::rayon::iter::IntoParallelRefIterator;
    use sha2::digest::Digest;

    use super::*;

    fn leaf_digests<T: FixedSizeSerializeBytes + Copy + Send + Sync>(
        leaves: &[Vec<T>],
        leaf_items: usize,
        fixed: bool,
    ) -> Vec<Output<Sha256>> {
        let mut out: Vec<_> = (0..leaves.len()).map(|_| MaybeUninit::uninit()).collect();
        if fixed {
            FixedSha256Digest.digest_with_const_len(
                leaf_items,
                leaves.par_iter().map(|leaf| leaf.iter().copied()),
                &mut out,
            );
        } else {
            ParallelSha256Digest.digest_with_const_len(
                leaf_items,
                leaves.par_iter().map(|leaf| leaf.iter().copied()),
                &mut out,
            );
        }
        out.into_iter()
            // SAFETY: the two digest implementations initialize every output
            // slot for the equal-length input/output slices above.
            .map(|value| unsafe { value.assume_init() })
            .collect()
    }

    #[test]
    fn fixed_leaves_match_native_and_serial_sha_at_padding_boundaries() {
        for leaf_len in [
            0, 1, 16, 32, 48, 55, 56, 63, 64, 65, 119, 120, 127, 128, 255, 256, 511, 512, 513,
            1023, 1024, 1025, 2048,
        ] {
            for leaf_count in [0, 1, 3, 4, 5, 17, 65] {
                let leaves: Vec<Vec<u8>> = (0..leaf_count)
                    .map(|leaf| {
                        (0..leaf_len)
                            .map(|byte| ((byte * 73 + leaf * 131 + (byte >> 3)) & 255) as u8)
                            .collect()
                    })
                    .collect();
                let got = leaf_digests(&leaves, leaf_len, true);
                assert_eq!(
                    got,
                    leaf_digests(&leaves, leaf_len, false),
                    "length {leaf_len}, count {leaf_count}"
                );
                let expected: Vec<_> = leaves.iter().map(Sha256::digest).collect();
                assert_eq!(
                    got, expected,
                    "serial length {leaf_len}, count {leaf_count}"
                );
            }
        }
    }

    #[test]
    fn ghash_leaf_serialization_matches_native_for_actual_fri_lengths() {
        for items in [1, 2, 3, 4, 8, 16, 32, 64, 128] {
            let leaves: Vec<Vec<BinaryField128bGhash>> = (0..19)
                .map(|leaf| {
                    (0..items)
                        .map(|word| {
                            BinaryField128bGhash::new(
                                ((leaf as u128 + 1) << 109)
                                    ^ ((word as u128 + 9) << 67)
                                    ^ (leaf as u128 * 913 + word as u128 * 773 + 1),
                            )
                        })
                        .collect()
                })
                .collect();
            assert_eq!(
                leaf_digests(&leaves, items, true),
                leaf_digests(&leaves, items, false)
            );
        }
    }

    #[test]
    fn variable_length_interface_uses_native_streaming_hash() {
        let leaves: Vec<Vec<u8>> = (0..31).map(|leaf| vec![leaf as u8; leaf * 77]).collect();
        let mut out: Vec<_> = (0..leaves.len()).map(|_| MaybeUninit::uninit()).collect();
        FixedSha256Digest.digest(leaves.par_iter().map(|leaf| leaf.iter().copied()), &mut out);
        for (leaf, got) in leaves.iter().zip(out) {
            // SAFETY: the digest operation initialized every output slot.
            assert_eq!(unsafe { got.assume_init() }, Sha256::digest(leaf));
        }
    }
}
