//! Produce challenge indices from a transcript.

use crate::transcript::{Decoding, VerifierMessage};

/// Generate a set of indices for challenges.
pub fn challenge_indices<T>(
    transcript: &mut T,
    num_leaves: usize,
    count: usize,
    deduplicate: bool,
) -> Vec<usize>
where
    T: VerifierMessage,
    u8: Decoding<[T::U]>,
{
    if count == 0 {
        return Vec::new();
    }
    assert!(num_leaves > 0, "Cannot sample an empty code domain");
    if num_leaves == 1 {
        // `size_bytes` would be zero, making `chunks_exact` panic.
        return if deduplicate { vec![0] } else { vec![0; count] };
    }

    if !num_leaves.is_power_of_two() {
        // Mixed-radix RS codewords need not have power-of-two length. Draw
        // ceil(log2(n)) uniform bits and reject candidates >= n. Every accepted
        // index has exactly the same number of byte preimages; reduction modulo
        // n without rejection would bias queries. Acceptance probability > 1/2.
        // Keep the legacy power-of-two branch byte-for-byte unchanged below.
        let bits = usize::BITS - (num_leaves - 1).leading_zeros();
        let bytes = (bits as usize).div_ceil(8);
        let mask = usize::MAX >> (usize::BITS - bits);
        let mut indices = Vec::with_capacity(count);
        while indices.len() < count {
            let mut candidate = 0usize;
            for _ in 0..bytes {
                let byte: u8 = transcript.verifier_message();
                candidate = (candidate << 8) | byte as usize;
            }
            candidate &= mask;
            if candidate < num_leaves {
                indices.push(candidate);
            }
        }
        if deduplicate {
            indices.sort_unstable();
            indices.dedup();
        }
        return indices;
    }

    // Calculate the required bytes of entropy
    // TODO: Round total to bytes, instead of per index.
    let size_bytes = (num_leaves.ilog2() as usize).div_ceil(8);

    // Get required entropy bits.
    let entropy: Vec<u8> = (0..count * size_bytes)
        .map(|_| transcript.verifier_message())
        .collect();

    // Convert bytes into indices
    let mut indices = entropy
        .chunks_exact(size_bytes)
        .map(|chunk| chunk.iter().fold(0usize, |acc, &b| (acc << 8) | b as usize) % num_leaves)
        .collect::<Vec<usize>>();

    // Sort and deduplicate indices if requested
    if deduplicate {
        indices.sort_unstable();
        indices.dedup();
    }
    indices
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transcript::{codecs::Empty, DomainSeparator, MockSponge, ProverState};

    #[test]
    fn mixed_radix_rejects_outside_domain_and_keeps_requested_count() {
        let ds = DomainSeparator::protocol(&module_path!()).session(&"mixed rejection").instance(&Empty);
        let mut transcript = ProverState::new(&ds, MockSponge {
            absorb: None, squeeze: &[255, 181, 180, 0, 180, 254, 1],
        });
        assert_eq!(challenge_indices(&mut transcript, 181, 4, false), vec![180, 0, 180, 1]);
        let mut transcript = ProverState::new(&ds, MockSponge {
            absorb: None, squeeze: &[255, 181, 180, 0, 180, 254, 1],
        });
        assert_eq!(challenge_indices(&mut transcript, 181, 4, true), vec![0, 1, 180]);
    }

    #[test]
    fn mixed_radix_sampler_has_equal_preimages_exhaustively() {
        for n in [3usize, 5, 17, 181, 251, 362, 905, 65521] {
            let bits = usize::BITS - (n - 1).leading_zeros();
            let bytes = (bits as usize).div_ceil(8);
            let mask = usize::MAX >> (usize::BITS - bits);
            let mut entropy = Vec::new();
            let mut expected = Vec::new();
            for candidate in 0..(1usize << (8 * bytes)) {
                for offset in (0..bytes).rev() { entropy.push((candidate >> (8 * offset)) as u8); }
                if candidate & mask < n { expected.push(candidate & mask); }
            }
            let ds = DomainSeparator::protocol(&module_path!()).session(&"mixed exhaustive").instance(&Empty);
            let mut transcript = ProverState::new(&ds, MockSponge { absorb: None, squeeze: &entropy });
            let actual = challenge_indices(&mut transcript, n, expected.len(), false);
            assert_eq!(actual, expected);
            let mut histogram = vec![0; n];
            for index in actual { histogram[index] += 1; }
            assert!(histogram.iter().all(|&count| count == 1usize << (8 * bytes - bits as usize)));
        }
    }

    #[test]
    fn mixed_radix_rejection_preserves_the_exact_next_challenge() {
        // Both fixtures accept four draws, including a duplicate, after three
        // rejections. The second case also exercises masked high bits and
        // big-endian two-byte candidates. The final three bytes are sentinels.
        let cases: &[(usize, &[u8], usize, &[usize])] = &[
            (181, &[255, 181, 180, 0, 180, 254, 1, 0xa5, 0x5a, 0x42], 7,
                &[180, 0, 180, 1]),
            (362, &[0xff, 0xff, 0x01, 0x6a, 0x01, 0x69, 0xfe, 0x00,
                0x01, 0x69, 0x01, 0xfe, 0xfe, 0x01, 0xa5, 0x5a, 0x42], 14,
                &[361, 0, 361, 1]),
        ];
        let ds = DomainSeparator::protocol(&module_path!())
            .session(&"mixed rejection next challenge")
            .instance(&Empty);
        for &(num_leaves, bytes, consumed, accepted) in cases {
            for deduplicate in [false, true] {
                let mut actual = ProverState::new(&ds, MockSponge {
                    absorb: None, squeeze: bytes,
                });
                let mut reference = ProverState::new(&ds, MockSponge {
                    absorb: None, squeeze: bytes,
                });
                let mut expected = accepted.to_vec();
                if deduplicate {
                    expected.sort_unstable();
                    expected.dedup();
                }
                assert_eq!(challenge_indices(&mut actual, num_leaves, 4, deduplicate), expected);
                // Independently advance by the hand-counted byte prefix, not
                // by running another copy of the rejection algorithm.
                for _ in 0..consumed {
                    let _: u8 = reference.verifier_message();
                }
                let next: [u8; 3] = std::array::from_fn(|_| actual.verifier_message());
                let expected_next: [u8; 3] = std::array::from_fn(|_| reference.verifier_message());
                assert_eq!(next, [0xa5, 0x5a, 0x42]);
                assert_eq!(next, expected_next);
            }
        }
    }

    #[test]
    fn power_of_two_matches_legacy_indices_and_following_challenges() {
        // Frozen original power-of-two algorithm, independent of the new
        // rejection branch. n=1 is excluded: the old chunks_exact(0) path was
        // undefined, rather than a legacy byte sequence to preserve.
        fn legacy<T>(transcript: &mut T, n: usize, count: usize, deduplicate: bool) -> Vec<usize>
        where
            T: VerifierMessage,
            u8: Decoding<[T::U]>,
        {
            if count == 0 {
                return Vec::new();
            }
            assert!(n.is_power_of_two() && n > 1);
            let size_bytes = (n.ilog2() as usize).div_ceil(8);
            let entropy: Vec<u8> = (0..count * size_bytes)
                .map(|_| transcript.verifier_message()).collect();
            let mut indices: Vec<usize> = entropy.chunks_exact(size_bytes)
                .map(|chunk| chunk.iter().fold(0usize, |acc, &b| (acc << 8) | b as usize) % n)
                .collect();
            if deduplicate {
                indices.sort_unstable();
                indices.dedup();
            }
            indices
        }
        let bytes: Vec<u8> = (0..512).map(|i| (i * 73 + 19) as u8).collect();
        let ds = DomainSeparator::protocol(&module_path!())
            .session(&"legacy power2 next challenges")
            .instance(&Empty);
        for num_leaves in [2usize, 128, 256, 512, 8192, 65536, 1 << 20,
            1usize << (usize::BITS - 1)]
        {
            for count in [0usize, 1, 2, 11, 31] {
                for deduplicate in [false, true] {
                    let mut actual = ProverState::new(&ds, MockSponge {
                        absorb: None, squeeze: &bytes,
                    });
                    let mut reference = ProverState::new(&ds, MockSponge {
                        absorb: None, squeeze: &bytes,
                    });
                    assert_eq!(challenge_indices(&mut actual, num_leaves, count, deduplicate),
                        legacy(&mut reference, num_leaves, count, deduplicate));
                    let next: [u8; 8] = std::array::from_fn(|_| actual.verifier_message());
                    let expected: [u8; 8] = std::array::from_fn(|_| reference.verifier_message());
                    let offset = count * (num_leaves.ilog2() as usize).div_ceil(8);
                    assert_eq!(next, expected);
                    assert_eq!(next.as_slice(), &bytes[offset..offset + 8]);
                }
            }
        }
    }

    #[test]
    fn test_challenge_stir_queries_single_byte_indices() {
        let num_leaves = 1 << 7;
        let num_queries = 5;

        let ds = DomainSeparator::protocol(&module_path!())
            .session(&format!("Test at {}:{}", file!(), line!()))
            .instance(&Empty);
        // Mock transcript with fixed bytes (ensuring reproducibility)
        let sponge = MockSponge {
            absorb: None, // Anything is fine
            squeeze: &[
                0x01, 0x23, 0x45, 0x67, 0x89, // Query 1
                0xAB, 0xCD, 0xEF, 0x12, 0x34, // Query 2
                0x56, 0x78, 0x9A, 0xBC, 0xDE, // Query 3
                0xF0, 0x11, 0x22, 0x33, 0x44, // Query 4
                0x55, 0x66, 0x77, 0x88, 0x99, // Query 5
            ],
        };
        let mut prover_state = ProverState::new(&ds, sponge);

        let result = challenge_indices(&mut prover_state, num_leaves, num_queries, true);

        // Manually computed expected indices
        let index_0 = 0x01 % num_leaves;
        let index_1 = 0x23 % num_leaves;
        let index_2 = 0x45 % num_leaves;
        let index_3 = 0x67 % num_leaves;
        let index_4 = 0x89 % num_leaves;

        let mut expected_indices = vec![index_0, index_1, index_2, index_3, index_4];
        expected_indices.sort_unstable();
        expected_indices.dedup();

        assert_eq!(
            result, expected_indices,
            "Mismatch in computed indices for domain_size_bytes = 1"
        );
    }

    #[test]
    fn test_challenge_stir_queries_two_byte_indices() {
        let num_leaves = 1 << 13;
        let num_queries = 5;

        let ds = DomainSeparator::protocol(&module_path!())
            .session(&format!("Test at {}:{}", file!(), line!()))
            .instance(&Empty);
        // Expected `folded_domain_size = 65536 / 8 = 8192`
        let sponge = MockSponge {
            absorb: None,
            squeeze: &[
                0x01, 0x23, 0x45, 0x67, 0x89, // Query 1
                0xAB, 0xCD, 0xEF, 0x12, 0x34, // Query 2
                0x56, 0x78, 0x9A, 0xBC, 0xDE, // Query 3
                0xF0, 0x11, 0x22, 0x33, 0x44, // Query 4
                0x55, 0x66, 0x77, 0x88, 0x99, // Query 5
            ],
        };
        let mut prover_state = ProverState::new(&ds, sponge);

        let result = challenge_indices(&mut prover_state, num_leaves, num_queries, true);

        // Manually computed expected indices using two bytes per index
        let index_0 = ((0x01 << 8) | 0x23) % num_leaves;
        let index_1 = ((0x45 << 8) | 0x67) % num_leaves;
        let index_2 = ((0x89 << 8) | 0xAB) % num_leaves;
        let index_3 = ((0xCD << 8) | 0xEF) % num_leaves;
        let index_4 = ((0x12 << 8) | 0x34) % num_leaves;

        let mut expected_indices = vec![index_0, index_1, index_2, index_3, index_4];
        expected_indices.sort_unstable();

        assert_eq!(
            result, expected_indices,
            "Mismatch in computed indices for domain_size_bytes = 2"
        );
    }

    #[test]
    fn test_challenge_stir_queries_three_byte_indices() {
        let num_leaves = 1 << 20;
        let num_queries = 4;

        // Expected `folded_domain_size = 2^24 / 16 = 2^20 = 1,048,576`
        let ds = DomainSeparator::protocol(&module_path!())
            .session(&format!("Test at {}:{}", file!(), line!()))
            .instance(&Empty);
        let sponge = MockSponge {
            absorb: None,
            squeeze: &[
                0x12, 0x34, 0x56, // Query 1
                0x78, 0x9A, 0xBC, // Query 2
                0xDE, 0xF0, 0x11, // Query 3
                0x22, 0x33, 0x44, // Query 4
            ],
        };
        let mut prover_state = ProverState::new(&ds, sponge);

        let result = challenge_indices(&mut prover_state, num_leaves, num_queries, true);

        // Manually computed expected indices using three bytes per index
        let index_0 = ((0x12 << 16) | (0x34 << 8) | 0x56) % num_leaves;
        let index_1 = ((0x78 << 16) | (0x9A << 8) | 0xBC) % num_leaves;
        let index_2 = ((0xDE << 16) | (0xF0 << 8) | 0x11) % num_leaves;
        let index_3 = ((0x22 << 16) | (0x33 << 8) | 0x44) % num_leaves;

        let mut expected_indices = vec![index_0, index_1, index_2, index_3];
        expected_indices.sort_unstable();

        assert_eq!(
            result, expected_indices,
            "Mismatch in computed indices for domain_size_bytes = 3"
        );
    }

    #[test]
    fn test_challenge_stir_queries_duplicate_indices() {
        // Case where the function should deduplicate indices
        let num_leaves = 128;
        let num_queries = 5;

        // Mock narg_string where some indices will collide
        let ds = DomainSeparator::protocol(&module_path!())
            .session(&format!("Test at {}:{}", file!(), line!()))
            .instance(&Empty);
        let mut prover_state = ProverState::new(
            &ds,
            MockSponge {
                absorb: None,
                squeeze: &[
                    0x20, 0x40, 0x20, 0x60, 0x40, // Duplicate indices 0x20 and 0x40
                ],
            },
        );

        let result = challenge_indices(&mut prover_state, num_leaves, num_queries, true);

        // Manually computed expected indices, ensuring duplicates are removed
        let mut expected_indices = vec![0x20 % num_leaves, 0x40 % num_leaves, 0x60 % num_leaves];
        expected_indices.sort_unstable();

        assert_eq!(
            result, expected_indices,
            "Mismatch in computed indices for deduplication test"
        );
    }
}
