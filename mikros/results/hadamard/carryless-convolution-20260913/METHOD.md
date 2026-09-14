# Carryless coefficient-convolution experiment

This experiment changes only the implementation of Mikros's binary univariate
Hadamard first message. The interpolation field is AES GF(256), kappa is 128,
and the extension field remains GHASH GF(2^128). Proof messages, challenge order,
terminal queries, and the remainder of the prover are unchanged.

## Kernels

- `pairreuse`: existing paired continuation-point interpolation and register
  aggregation, followed by interpolation/quotient recovery.
- `coeffnativereuse`: existing byte-to-monomial table, high triangular GFNI
  schoolbook product, and sparse quotient recovery.
- `coeffclmulreuse`: byte-to-bitplane table, four-row VPCLMUL Karatsuba high
  product, bitplane decoding, the same register weight consumer, and sparse
  quotient recovery.
- `coeffnibblereuse`: the same carryless product with an independent smaller
  nibble-to-bitplane table.
- `flock128`, when selected: the existing Flock degree-two zerocheck with seven
  skipped rounds, hence the same 128-bit row compression.

Each new coefficient input is eight 128-bit planes. Polynomial multiplication
along the eight AES coefficient bits uses 27 Karatsuba leaves over F2[X]. Each
leaf computes the upper half of an ordinary 128-by-128 binary product with
three CLMUL instructions. AVX-512 processes four rows at once: 81 wide CLMUL
instructions per four rows, before XOR recombination, AES reduction, layout
conversion, and weighting. This is not a count of all instructions or cycles.

High-half projection commutes with the additions and coefficient-basis
reduction used here. The algorithm never multiplies already truncated
X-polynomials. Degree 255 is mathematically zero and is checked by independent
basis and product tests.

The byte and nibble encoding tables occupy 512 KiB and 64 KiB respectively.
Their 128-byte entries are aligned to 64 bytes. Nibble encoding performs twice
as many table reads; a smaller footprint does not imply a faster kernel.
Sparse quotient data is initialized separately, so the nibble path does not
initialize an unused byte table. Carryless worker blocks use 2048 rows, matching
`pairreuse`; the preserved schoolbook experiment uses 512-row blocks. All share
64-row accumulation tiles. These are complete engineering configurations, not
an instruction-only multiplication ablation.

## Timing and validation boundaries

All witness-dependent encoding, convolution, decoding, weighting, packing,
folding, and messages are timed. Fixed public tables initialize inside the
first proof: cold/warmup timing includes their construction. Formal warm
timings use those prepared tables and independent reusable scratch capacity
per system. No measured time is subtracted.

Every proof is individually verified; prover/verifier claims and the next
challenge are compared. Within each `(dimension, run)`, serialized Mikros proof
digests must match across kernels. Source authentication defaults to a full
independent oracle. The explicit `--reuse-source-checks` option performs that
oracle for the first Mikros proof on each immutable input and permits only
exact full-Claims equality afterward. The cache also binds the original slice
addresses, lengths, and dimension. Flock always uses its own full source oracle.
This saves only untimed repeated validation; JSON records each check method
and its earlier anchor. The analyzer audits these records separately.

Native server tests must cover the hardware paths. Local WSL has a fallback
backend, so its success does not establish native-kernel correctness or speed.

Server runs use 64 threads and affinity `0-47,48-55,72-79`: 48 physical cores plus
16 SMT siblings, 32 logical CPUs on each NUMA node, on Xeon Gold 6342. Memory
placement uses Linux first touch. This is a shared server, not an exclusive
reservation. The pinned nightly toolchain and `-C target-cpu=native` are recorded
with complete source hashes and actual thread/backend metadata.

Ordinary timings rotate system order with a full number of rotations. Warmups,
independent phase profiles, and any fresh-process cold/RSS samples stay
separate. A profile certificate time is the per-sample sum of aggregation and
quotient recovery, summarized afterward; it is not the sum of two medians.

The benchmark is the direct packed-opening Hadamard PIOP. It does not include
PCS, column-to-tensor aggregation, or the E2E SNARK. Grinding is disabled and
there is no equal-security claim.
