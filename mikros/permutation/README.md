# Mikros permutation using the root Flock field

The E2E entry point now defaults to the revised packed permutation with fused
GFNI aggregation. See [PACKED.md](PACKED.md) for the current implementation and
[server results](../results/permutation-packed-20260914/OPTIMIZED.md). The APIs
described below remain available for explicit legacy comparisons.

This crate ports the optimized `lookup/flock_compare/binary_v5` permutation
implementation without modifying its frozen experiment sources. Dependencies
use the root Flock field and transcript types, so PCS composition does not
cross snapshot type boundaries.

`permutation::{Statement, prove, prove_profiled, verify}` preserves the standalone
digest-bound protocol. Its returned source claims still require a PCS.

For composition, `permutation::prepare` performs offline shape checks and takes
ownership of the two packed bit oracles. `prove_committed` and `verify_committed`
bind a distinct protocol label and the oracle shape, without rescanning the
source to hash it. **The caller must bind the actual PCS commitment and public
statement before either call, and authenticate both returned linear queries.**
This API does not claim that a digest or a full-source checker is a PCS.

`wiring::WordWiringStatement::new` validates and hashes a public word permutation
offline. `wiring::prepare` expands every packed `F128` word into all 128 bits,
using `row = word * 128 + bit` and `sigma_bit(row) = sigma(word) * 128 + bit`.
The online `wiring::{prove, verify}` subtract public index-tag evaluations from
both GKR leaf claims and return two ordinary bit-MLE claims on the original
packed witness. These claims must be discharged against its commitment.

For wiring already specified at bit granularity, use
`wiring::BitWiringStatement::new(&sigma)`, then
`wiring::prepare_bits(&packed_words, &statement)` offline. Any permutation of bit
addresses is supported, including moves across bit positions within a word.
`wiring::{prove_bits, verify_bits}` returns the same two `BitMleClaim` obligations;
it does not force word-preserving wiring. Bit address `i` is coordinate `i % 128`
of `packed_words[i / 128]`. Nonzero unused bits in sub-word inputs are rejected.
The public sigma tags are pre-encoded offline (16 bytes per bit address, in
addition to the `usize` sigma vector). Their challenge-dependent evaluation stays
online and is included in `Timings::source_reduction_ms`.

The current PIOP limit is 2^30 bit cells, hence 2^23 original packed words.
No high bits are truncated. The word permutation must preserve witness values;
permutations of unequal words correctly reject. Fixed dummy/public cells and
the mapping from circuit cells to the committed witness remain the caller's
responsibility. In particular, the generic adapter is not a replacement for
the circuit's public-cell recombination checks.

For a strict offline/online boundary, call
`permutation::prepare_tree_coefficients(&sources, options)` or
`wiring::prepare_bit_coefficients(&prepared_wiring, options)` offline. These
objects own the decoded tuples and every challenge-independent monic coefficient
layer, including the highest compact layer. The original source tuple buffers
can then be dropped. The static leaf embedding lookup is also prepared once and
shared without copying across online proofs. `resident_bytes()` reports the
coefficient-buffer and private leaf-table capacities (excluding shared global
field tables, small metadata and the public sigma cache). For K32 and capacity
128, the two coefficient trees occupy approximately 24 bytes per permutation
bit cell, plus their small leaf tables.

Use `permutation::prove_prepared` or `wiring::prove_bits_prepared` for timed online
proving. These entries borrow the immutable coefficients without cloning them.
All z-dependent tables, coefficient evaluation, extension product layers, GKR,
and both query reductions remain inside the online call. The old
`prove_committed`/`prove_bits` convenience entries still include source decoding
and static coefficient generation and should not be labeled strict online.
Both routes have identical proof messages and challenges under the same binding.

The direct public sigma evaluation is linear-time verifier work. This adapter
does not silently replace it with an unauthenticated prover-supplied scalar;
a deployment requiring sublinear verification of a fixed large circuit must
add an authenticated public-table evaluation mechanism.
