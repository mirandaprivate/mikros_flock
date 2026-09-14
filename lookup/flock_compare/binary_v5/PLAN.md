# Permutation iteration v5

Run only inside the already authorized server directory
`/home/mscong1/mikros-experiments/20260912T-native64/permutation-binary-v4/iteration-v5`.
Use the independent `source/binary_v5` crate, frozen binary v4 as `previous`,
and snapshot Flock. Do not alter or overwrite the v4 source/results.

All work uses 64 threads and `taskset -c 0-47,48-55,72-79`: 48 physical cores,
16 SMT siblings, 32 logical CPUs on each NUMA node. Linux default first-touch;
no explicit NUMA binding or CPU exclusivity. Set
`CARGO_TARGET_DIR=$RUN_ROOT/source/target`, nightly2026-03-21 and
`RUSTFLAGS=-C target-cpu=native`. Locked fetch precedes offline release tests
and build. Every source file and the executable are hash-checked.

Only width1 is tested in this iteration. At N30 its value/index tuple is
31 bits and uses K32. `register` changes register-resident fold+next-message;
`reuse` adds bounded tree-layer storage; `combined` applies both. The two
reuse slots are independent: slot0=reuse, slot1=combined, each at most 32 GiB
of allocation capacity, at most 32 buffers, and a 1 MiB minimum retained buffer.
The old/new Mikros and Flock still share the existing snapshot F128 scratch
pool. Do not describe previous as wholly unpooled or the three systems as
having fully isolated scratch pools.

The public default prover remains Engine::Fused. Candidates require their
explicit Engine selectors; this experiment does not silently promote a new
default. After measuring, document the speed/retention/cleanup tradeoff.

## Fixed plan and selection

1. N20 smoke: previous/register/reuse/combined/flock, one proof each, no warmup.
2. N26 screening: previous/register/reuse/combined, one warmup then eight
   measured inputs each. Repeat Williams orders ABDC/BCAD/CDBA/DACB twice.
   Positions and immediate predecessors are balanced; not all24 permutations.
3. Among register/reuse/combined, select the smallest measured mean core time;
   an exact tie uses the candidate name. Persist the input report SHA, all
   candidate means, executable SHA and UTC in selection.json before N30 starts.
   Do not replace the selection based on subsequent favorable N30 samples.
4. N30 main: previous/selected/flock, one warmup then six measured inputs each,
   covering all six three-system permutations in the frozen order policy.
5. Independent cold processes for each of those three systems: runs1/warmups0,
   seed1. Independent warm processes: runs1/warmups1, seed0. The measured input
   matches main run1 in both modes. Warm VmHWM includes the warmup lifetime.

This yields9 cases and71 accepted proof records when no attempt is excluded:
5 smoke+36 screening+21 main+3 cold+6 warm. The verifier uses the recorded plan
and actual counts. Whole affected attempts are retained and retried at most3
times after detected same-user benchmark/build interference. Before N30 require
at least220 GiB available, including any finite ancestor cgroup remainder.
Three quiet checks precede commands; the wait defaults to1800 seconds, bounded
to15–7200 seconds. Never stop another job.

## Timings, retention and cleanup

Every proof replays verifier, compares claims/transcripts and performs full
source authentication. Original proof bytes must match across all Mikros
variants; Flock has its own proof format. This is permutation PIOP without
PCS, grinding, Hadamard skip alignment or an equal-security claim.

Core includes each synchronous pool return/rejection; total adds native input
conversion. Public input generation/digest and final verifier/source checks
remain separate. Capture both tree pools after each proof, keeping their raw
stats, exact retained capacity and reuse counters. All mathematical values
are overwritten even on allocation hits.

After all inputs and proof objects are dropped, save both pool stats and RSS,
then synchronously clear the tree pools and shared F128 pool. Report each clear
time, total cleanup, memory before/after, batch wall before/including cleanup.
Cleanup is outside per-proof core/total and is not free. Batch wall includes
input generation, verification, source scans, proof output and warmups; it is
not proving-only and must not replace proof timers or be mixed with medians.

Report main mean/median/range and paired comparisons, register/previous,
reuse/previous, combined/register and combined/reuse, plus chosen/Flock N30.
Do not combine different N or previous binaries' timing samples. Separately
report single-process cold/warm RSS, retained capacity and batch teardown.
Retain all regressions, cold single-sample reversals, and source-check overhead.
Do not infer cache, NUMA or allocator causes without corresponding measurements.

## Download and replay

Archive all results/ files including proof directories and excluded attempts,
plus root/selection.json. results_manifest.json maps root-relative paths to
SHA256 (including selection.json, excluding the manifest itself). After safely
downloading and checking the archive and every member, write
download_validation.json with archive_sha256 and verified_files. Preserve the
original source_manifest.json, package.json and source.tar.gz.
Run the frozen binary_v5/summarize_experiment.py --root <downloaded-root>.
