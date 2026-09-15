# Exact-gate PLONK E2E experiment

Single: one connected circuit with `N` genuine AND/XOR gates. Multiple: 128
independent inputs to the same circuit shape, **each with N gates**, `128*N`
total. The execution thread count is separately fixed at 64.

These files prepare and execute an experiment; they contain no measured result.
Source freeze, upload and execution are separate actions controlled by root.

## Prepare

`package_sources.py OUTPUT --list-only` inventories sources without freezing.
After integration is ready, run without `--list-only`; `OUTPUT` must be inside
this experiment directory. It writes a source tarball, its SHA-256 and an exact
per-file hash manifest, including new m36–39 native Fast PCS configurations.
It excludes history, credentials, unrelated projects and build artifacts.

## Server

Extract under the authorized
`/home/mscong1/mikros-experiments/20260912T-native64/v16-plonk-e2e-memory-20260914/source`.
Run from that snapshot:

```sh
python3 mikros/server_experiments/20260914-plonk-e2e/run_server.py --levels 14 --logs results-pilot14
```

`--dry-run` prints the plan without writing or executing. The default levels are
14, 20, 26, 30 in that order. Explicit `--levels 20`, `--levels 26`, or `--levels 30`
allows root to inspect earlier pilot evidence before progressing. Each selected
level runs four serial, separate processes with one repetition by default.

The build reuses the authorized v14 target cache, with native ISA flags and the
same nightly toolchain. No cargo clean or disk cleanup occurs. Only the authorized
`/home` tree is written; another user's `/data` directory is never used. The
driver checks available RAM, disk space, CPU idleness, selected hardware flags,
compiler flags, source hashes and binary hashes. It keeps subprocess output and
`/usr/bin/time -v` evidence even if a case panics or times out.

`--skip-build --reuse-evidence PATH` requires a same-source, same-binary build
receipt. `--skip-tests` is explicit and recorded; omitted tests are never reported
as passed. Result directories must be fresh and are not overwritten. Case timeouts
default to four hours; changing them does not create an unattended scheduler.

## Evidence and scope

`aggregate_results.py RESULTS` reconstructs a validated summary from actual raw
JSONL and statuses. It writes `summary.json`, `single_instance.log`,
`multiple_instance.log`, and hashes of evidence files. Root may place the two
instance logs at the repository root after retrieval. Missing or failed cases
stay explicitly incomplete and do not get ratios.

Comparisons require equal witness/public commitment hashes and equal witness/public
native FLOCK PCS parameters. Both online PCS openings, witness commitment and
recommit, public recommit, PIOPs, release and proof serialization are timed.
Initial public commitment and static input preparation are offline. Conversion
inside gate/copy PIOPs is included in those stage times; no separate conversion
timing is invented. One repetition describes a run and cannot establish statistical
significance. `/usr/bin/time` RSS covers the entire fresh process, including offline
setup and verification, not an isolated prover-only allocation peak.

At N30 the full multi-instance witness PCS buffer is 64 GiB, its rate-1/2 codeword
128 GiB, and its Merkle tree 8 GiB. Stage release/recommit is therefore necessary.
The default large-case RAM preflight requires 350 GiB available; this is a guard,
not a claim that the proof fits. Actual peak RSS and stage logs decide feasibility.

The v2 snapshot uses the native rectangular PCS geometry, split row equality
tables and bounded scratch lifetimes. Identical row queries reuse native
column-fold banks. All this work remains inside the online opening timer.
The earlier v1 one-column pilots are separate evidence and are not combined
with v2 timings.

## N26 optimization experiments

The N26 case uses one N-gate circuit in single mode and 128 separate N-gate
executions in multiple mode (2^33 total gates). Source vectors, native PCS
parameters and the release/recommit timing scope stay equal across systems.
`--trace` explicitly enables PCS, Ligerito and copy phase diagnostics; keep
these runs separate from the primary untraced timings. Use three fresh
invocations for independent-process samples.

`--disable-optimization NAME` is repeatable. Available controls are
`row-grouping`, `jit-fused`, `ordinary-eval`, `label-fold`, `paired-bitbanks`,
`leaf-cache`, `tree-fusion`, `sparse-bitbanks`, `constant-bitbanks`, and `tree-pool`. Environment overrides are
recorded in config and command evidence. Common PCS improvements apply to both
systems; leaf caching and extension-product tree fusion affect Mikros multiple.
Baseline and optimized proof hashes must match separately for each system.
Original N30 root reports are retained; N26 evidence uses separate directories.

The Mikros extension-product pool is local to one proof and defaults on only
for tree log length at most 28 (the N26 multiple case). Every retained buffer
is freed before that proof call returns, with final release charged to the
existing tree reduction and online copy timers. Capacity accounting includes
both checked-out and pooled buffers. Larger unmeasured trees keep the original
allocation policy. This is independent of the allocator experiment below.

`--allocator glibc-reuse` is a separate, N26-or-smaller experiment. It disables
the glibc dedicated-mmap threshold path and automatic arena trimming with
`MALLOC_MMAP_MAX_=0` and `MALLOC_TRIM_THRESHOLD_=-1`. Every source data drop,
PCS recommit and cleanup call remains inside its original timer; the allocator
may keep freed virtual/physical memory for subsequent allocations. Report its
RSS and compare both systems under the same policy. This policy is not enabled
by default and has not been validated at N30. Inherited allocator variables
are cleared so the selected policy is reproducible.

## Native packed-direct experiment

`--packed-direct` enables `FLOCK_PCS_PACKED_DIRECT=1` for both systems and both
prover and verifier. The default remains the previous RS route. Only complete
identity rectangles are eligible; after tensor adaptation, ordinary packed-word
MLE functionals use the existing native `PackedDirectClaim` interface. Other
functionals retain the native RS route. No commitment, original terminal claim,
or preceding PIOP is changed. The native PCS batching challenge stream and PCS
proof bytes can change, so this mode must be configured consistently on both
ends and compared separately from byte-preserving engineering ablations.

New logs record `piop_blake3` and `terminal_claims_blake3`, computed before the PCS
and charged to online serialization time. OFF/ON evidence must match these hashes,
commitments and PCS parameters separately for each system. Full proof hashes must
remain stable within each arm; the OFF arm must match the previous baseline.
Keep phase-traced runs separate and use the system allocator in primary timings.

The direct-only block-statistics scan also has a separate engineering control:
`--disable-optimization direct-block-parallel`. It compares the original
column-parallel implementation against row-chunk parallelism inside each column.
This control matters only with `--packed-direct`; it must preserve the complete
PCS proof bytes and transcript for that route. It does not change the default
route, which remains OFF for packed-direct until measured separately.

## Further byte-preserving kernels

`--direct-wp-simd` (requires `--packed-direct`) enables
`FLOCK_DIRECT_WP_SIMD=1`: factor the direct row tensor's high weight into a
segment coefficient and accumulate W-prime with four-lane native products.
`--gate-selector-simd` enables `MIKROS_GATE_SELECTOR_SIMD=1`: Mikros's multiple
gate message kernel checks the selector identities before using the residual
`q*(L+R+M)+(M+O)`. General selectors retain the original kernel; four original
wire folds, terminal claims and the Hadamard authentication remain present.

Both controls default OFF and are independent. Test reference, W-prime-only,
gate-only and both under the same packed-direct route, source, native binary,
system allocator and memory policy. Unlike changing the PCS route itself,
these controls must preserve the entire proof and transcript byte-for-byte.

`--gate-fusion` independently enables `MIKROS_GATE_FUSION=1` (default OFF).
It folds all four Mikros multiple gate wires into bounded output tiles and
consumes each tile for the next unscaled round message while it is still in
cache. Selectors are folded first; the next cubic message and challenge are
still bound in their original order. Compare a pure fusion arm to reference
with both arithmetic opt-ins OFF, before attributing any combined benefit.
Gate diagnostic traces separate virtual-product allocation, sumcheck,
virtual-product release and the weighted Hadamard proof; traces remain outside
the primary sample set.

`--wp-uninit` enables `FLOCK_WP_UNINIT=1` (default OFF) for both systems.
Native W-prime output is reserved without a full zero-initialization pass:
each worker writes all of its slots before that chunk can be read, and the
vector length is published only after every worker joins. It retains the
same RS/direct maps and release policy, with q-prime unchanged. Compare
reference, pure gate fusion, pure W-prime first-write, and both under one
frozen binary; keep the two earlier arithmetic opt-ins OFF in this study.
Complete proof and challenge equality is required for every arm.

## Mikros tiled input and scratch experiments

The next study keeps `--packed-direct --wp-uninit` ON for both systems and
leaves the earlier arithmetic and gate-fusion opt-ins OFF. New controls default
OFF and affect only Mikros:

- `--gate-virtual-product`: compute the initial bitwise-AND M in bounded tiles
  for the first gate message/fold. After that, fold M independently as usual.
- `--wh-tiled-weights`: construct arbitrary Hadamard row weights from two small
  tensors at existing aggregation/packing task boundaries. This also applies
  to single-mode weighted Hadamard calls; no factor is normalized or inverted.
- `--copy-paired-terminals`: evaluate two distinct multiple-copy terminal points
  in one scan per wire, preserving each point, scale and terminal claim.
- `--gate-wh-scratch`: requires `--gate-virtual-product`. Transfer ownership of
  two gate fold allocations into the Hadamard IP fold scratch. Reuse capacity
  only; initialize all output values and release all buffers before returning.

Keep native N14 pilots and traced N26 diagnostics separate from primary
fresh-process samples. Compare each new kernel locally before selecting a
combined arm; require full proof, PIOP, transcript, terminal, commitment and
PCS-parameter equality against the frozen reference. Online timing includes
the ownership transfer and all eventual releases; no memory pool survives a
proof call. FLOCK serves as an unchanged control for these four Mikros options.

## Multiple product-tree experiments

`--tree-dense-simd` enables `MIKROS_TREE_DENSE_SIMD=1`: contiguous AVX-512
loads and in-register even/odd separation in Mikros's extension product-tree.
Implicit leaf callbacks retain the existing physical indexing and computation.
The complete proof, claims and transcript must match the OFF arm.

`--tree-large-pool` enables `MIKROS_TREE_LARGE_POOL=1`: extend the existing
exact-capacity buffer pool from log L <= 28 through log L = 32. At N30 multiple,
L=4N and the pool owns at most 96 GiB; witness/public inputs and the online
48 GiB leaf cache are additional. All tree capacities are released before
terminal evaluation and native PCS. This flag defaults OFF; the system
allocator and the 350 GiB preflight requirement remain unchanged.

`--modes multiple` measures only the two 128-instance cases, preserving
`total_gates=128*2^log_gates` and the same comparison checks. Omission keeps
both single and multiple cases. Keep traced runs separate from primary runs,
and compare the two tree flags independently before selecting a combined arm.

`--tree-register-fusion` independently enables `MIKROS_TREE_REGISTER_FUSION=1`.
The native fused fold constructs the next three raw message sums directly
from output registers, then stores the outputs for the next challenge. It
does not reload the output tiles to compute that message. Implicit leaves and
dense slices use the same equations; portable kernels retain tile fusion.
Compare this arm independently of `--tree-dense-simd`, which only replaces
reads in the other product/message/fold kernels.
