# Section 4/5 PLONK end-to-end comparison

The new executable is `e2e_plonk_compare`. The old SHA benchmark counts padded
witness bits and is not the requested exact-gate experiment.

## Workload and statement

`--log-gates 30 --mode single` proves one circuit with exactly 1,073,741,824
binary gate equations. `--mode multiple` proves **128 executions of that same
circuit**, each with 1,073,741,824 gates: 137,438,953,472 binary constraints in
one batch. It does not divide the gate count among executions.

The synthetic circuit alternates AND and XOR. Gate 2j computes `a_j AND b_j`;
gate 2j+1 XORs that output with `a_(j+1 mod N/2)`. Shared inputs link the
successive gadgets. The final output is public. This is a regular mixed-gate
circuit, not SHA-256. Corresponding instance bits are independent deterministic
test inputs, packed into one F128 per gate in multiple mode. Both systems use
identical wire values, public output, sigma, commitments and PCS parameters.

There are `3N+1` live copy positions, padded to `4N`. At N=2^30, the single
Mikros tuple field is GF(2^64) and its copy packing width is 2. Gate packing
width and FLOCK zerocheck skip width are both 128.

## Reduction and PCS

Mikros single follows the current five weighted Hadamard construction, including
the nested authentication of the virtual product. Multiple uses the cubic
packed gate sumcheck, one binary weighted Hadamard, and two independently
challenged extension product trees. Every terminal, including block-prefix
weights, is passed to the PCS. Zero or zero-sum factors are never divided away.

FLOCK uses its original skip-128 binary zerocheck and native grouped Product-GKR.
For this fixed local gate pattern, the linear conversion is
`A=L+odd*R`, `B=even*R+odd`, `C=O`; its exact tensor expansion authenticates
the original wire columns. FLOCK's two copy trees share a terminal point.
Consumed native GKR layers are released through the new bounded-memory entry;
its proof bytes and transcript are identical to the original entry.

Both systems use FLOCK's native commitment, production merged opening,
ring-switch and Ligerito Fast backend. `merged_tensor::open_identity` exposes
the existing native ownership path without copying the entire witness. For a
fixed geometry, its proof is byte-identical to the borrowed wrapper. Added m36--39 Fast profiles
come from the unchanged native security derivation and validation. In the
target single experiment witness/public m are 32/37; in multiple they are 39/37.
The full power-of-two commitment geometry is shared, without lane compaction.
The current driver uses gate rows as the low coordinates and source blocks as
the high coordinates. At N30 the witness has four packed-word columns; the
single public table has 128 columns and the multiple public table one column.
This activates FLOCK's native rectangular opening transport. Earlier v1 pilots
used a one-column registry and are retained as separate evidence, not mixed
with the current geometry's performance measurements.
The preprocessed public commitment is trusted verifier-key data; the online
verifier never reads witness values, sigma arrays or prover preparation.

At N=2^30 the expected witness/public tensor claim counts are respectively
11/101 for Mikros single, 7/96 for FLOCK single, 11/8 for Mikros multiple, and
7/3 for FLOCK multiple. These different protocol outputs use the same PCS
interface and backend; no unauthenticated claims are omitted to match counts.

## Timing and memory

Offline time includes input generation/packing, static public-table commitment,
Mikros single tuple coefficients, and FLOCK single bit-to-field expansion plus
static sigma indexing. It is recorded separately. Online time includes witness
commitment, gate work (including virtual products and FLOCK's local linear
combination), all challenges and dependent conversions, copy proof, query
expansion, both native PCS openings, cleanup and proof serialization.

The two systems use the same memory policy: discard initial PCS data, release
PIOP scratch/preparation, then recompute each commitment's opening data.
**Both witness recommit and public recommit are charged online.** The recomputed
commitment must exactly match its originally bound value. These measurements
are therefore not timings with unlimited preprocessed PCS data resident.
Native FLOCK scratch and Mikros's separate 32-GiB tree pool are both cleared.
Large rectangular-transport row weights use the exact product of two smaller equality tables,
and unused native scratch is released before entering the inner PCS. These
changes preserve the native proof messages and transcript for a fixed geometry.
Full identity rectangles compute column bit banks with FLOCK's split fold,
reuse each exact row point, and reuse the same banks for ring switching and
rectangular transport. Bank construction remains online.
The one-column multiple public table still uses the original general transport;
its separate memory use is included in the process RSS measurement.
For m36 and above, the inner PCS also releases idle scratch before the last
level-zero fold and before committing the next recursive level. This prevents
large consumed F256 buffers from surviving into the next allocation phase.
Each server case is a fresh process, run serially at 64 Rayon workers and
64 FLOCK workers with the same fixed CPU affinity and native ISA flags.

The native PCS uses its strict Fast configuration. PIOP grinding is the
explicit common gate policy and degree-dependent copy policy recorded in each
sample. This experiment does not assert a new aggregate 128-bit security
theorem for the binary specialization or for the complete composition.

## Validation and evidence

The new core tests cover source-oracle agreement, non-normalized/degenerate
weights, challenge ordering, tampering, streamed product trees and thread
determinism. E2E component tests also check the public I/O, padding, exact
gate/wiring counts, all terminal queries, and a fake virtual product on an XOR
row. Four local N14 complete proofs were verified with matching commitments
and PCS configurations; those local timings are not server measurements.

The server harness under `server_experiments/20260914-plonk-e2e` freezes an
allowlisted source manifest, validates the native build and proofs, records
raw JSONL/time-v/status evidence, and aggregates only completed verified runs.
Target reports are the repository-root `e2e_single_compare_flock.log` and
`e2e_multiple_compare_flock.log`. Missing or failed cases must remain explicit;
smaller pilots must never be relabeled as the N30 targets.

The v2 server snapshot is
`v16-plonk-e2e-memory-20260914/source`, with source archive SHA-256
`c36ae7ac24ff0150a1bd9a768ce3f1d7ccf6d6cc0b2dfa9c88ade81623b3fdd9`.
Its N14, N20 and N26 pilots all passed complete verification and matched
commitments/PCS parameters across systems. At N26 the observed single prover
times were 7.218 s (Mikros) and 10.825 s (FLOCK); the multiple batch times were
27.870 s and 31.348 s. These are one-repetition pilots, not N30 results.
The largest pilot process RSS was 26.327 GiB. N26's multiple witness m35 still
uses the ordinary small-domain scratch policy; its RSS must not be linearly
extrapolated without accounting for the m36-and-above release boundaries.

## Completed N30 server results

All four complete proofs passed their public verifiers, transcript checks,
commitment equality checks and native PCS parameter checks. The executed v2
source and binary hashes were verified before and after both pilot and target
runs. Both thread pools had 64 workers with GFNI, AVX-512F/BW and VPCLMUL enabled.

| Online prover, seconds | Mikros | FLOCK | Observed time reduction |
| --- | ---: | ---: | ---: |
| One N30 circuit | 122.763 | 133.767 | 8.226% |
| 128 N30 circuits, entire batch | 431.974 | 449.069 | 3.807% |

Single offline times were 146.951 s / 27.862 s (Mikros / FLOCK). They are
excluded by the requested static-conversion policy, so the table is not a
comparison of total wall time including preprocessing. Multiple's offline
times are also retained in the raw samples. Online includes all dynamic
conversions, both recommits, both openings, release and serialization.

Single's main saving is copy proof time, 42.559 s versus 81.871 s. Its public
PCS opening costs more, 48.439 s versus 31.557 s, and memory release costs
20.006 s versus 8.722 s. In multiple mode, gate/copy times are 16.571/80.842 s
for Mikros and 25.640/90.917 s for FLOCK; witness PCS times are nearly equal,
201.652 s versus 202.057 s. Mikros's public opening remains slower,
48.593 s versus 39.392 s. These component timings explain only this observed
run, not an asymptotic or statistically established advantage.

The time-v maximum RSS values are 327.844/336.313 GiB for single and
344.057/344.072 GiB for multiple (Mikros/FLOCK). These counters cover the
whole process, including offline setup and verification. Single had
41,600/1,090 major page faults; multiple had 0/1. A zero time-v `Swaps`
counter is not evidence of a swap-free execution; live paging observations
are retained separately. One repetition per case is descriptive evidence,
and the modest differences need repeated runs before claiming stability.

The requested root logs are byte-for-byte copies of the validated target
reports. Full raw JSONL, time-v, command/configuration, test and hash receipts
are retained under
`server_experiments/20260914-plonk-e2e/v2/evidence/`.
The downloaded evidence archive SHA-256 is
`adccea1f06bb8f9905483ae3aac432b33679d51164d207095faeb125d8f82c43`.
