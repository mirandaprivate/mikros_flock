# E2E bit-wiring experiment state

CURRENT TURN (v3 COMPLETE): User authorized making Mikros reductions
similar to Flock. Main example defaults to --reduction lockstep, with legacy
and cube alternatives for same-version ablations. Shared cube gather audits
the eight first-word values with an explicit PoW2 3-coordinate challenge and
returns one normal TensorClaim. New Mikros lockstep APIs reuse existing kernels,
bind both messages before each shared challenge, and independently verify both
trees; bit wiring subtracts each public tag then checks common point AND value.
Expected PCS input counts: legacy F11/M20, cube F4/M6, lockstep F4/M5.
Boolean A/B different factors and constant pin remain authenticated. No PCS
backend or parameters changed. Existing v2 snapshots/results remain immutable.
Local permutation 27 tests and E2E helpers 9 tests passed; all three E2E modes
passed full proofs/tamper replay. Targeted Clippy passed. Frozen v3 contains
350 files / 1713558 archive bytes, SHA256:
a12caa90ee14b59abbd32f75bd1cf0cbd9de88872b67cc45f96f268b9d156011.
Uploaded under the prior scope to native64/e2e-bit-v1/v3, checksum verified,
independent copy of v2 build cache (no shared writable old artifacts).
Server 64-thread driver completed for m22/m26/m30 and largest legacy/cube
ablations. PID3654330, UTC2026-09-13T08:40:37Z--08:42:43Z. First PID3654165
failed at build because copied v2 Cargo cache plus archive mtime0 missed the
new permutation API. Updated mtimes of 11 manifest-different source files
(contents unchanged), then rebuilt and reran all stages. Initial failure logs
are preserved in results-build-cache-failure/ and cache-recovery.json.
64 native test executions passed, 58 complete proofs verified (39 formal,
13 warmup, 6 validation), 144 tamper rejections across all three modes.
All full PCS configs/commitments match production, summaries independently
recomputed and equal. Main m22/m26/m30 medians Flock:
133.204197/319.238549/1530.044513 ms; Mikros:
174.092243/239.187277/811.590212 ms. Maximum M/F=0.5304356867 (time -46.9564%).
Largest same-v3 Mikros legacy/cube/lockstep=1138.782/864.598/811.590 ms;
PCS=577.921/281.361/267.208 ms. Different mode domains change challenges/PoW,
so these are whole-mode measurements, not fixed-transcript kernel timings.
Production word-wiring largest=549.886 ms, separate from bit comparison.
115 downloaded evidence files verified, archive56848 bytes / SHA256:
e28d6a4a2c546bbb942cf2804a1593be70621e7bb1b95746d9c78713f1eba16a.
Report: results/e2e/server-lockstep64-20260913/RESULTS.md, regenerate analyze.py.
All jobs exited; server shell15299 closed. No credentials saved to artifacts.

PREVIOUS (v2 COMPLETE): User requests the same production Flock PCS for true
E2E replacement. Both main arms now target union dense compaction/commit_lanes,
commit_lane_major, production open_batch_merged with generalized weighted128
RS scalar checking, unchanged merged sumcheck/Frobenius/single EqPoint Ligerito.
Agent e2e_flock_audit owns shared core PCS changes; e2e_mikros_audit owns new
pcs/merged_tensor.rs adapter; root owns main/driver/docs; permutation audit
reviewed the new adapter. Legacy v1 results/snapshots stay immutable. V2 native
server run completed with 51 passing targeted tests, Flock23/Mikros33 rejected
E2E tamper cases, 38 fully verified actual proofs. All 3 paths share matching
PCS parameters, embedded config/query/grinding lists and commitment digest.
Every online query conversion remains timed. Results and verified 87-file
evidence: results/e2e/server-pcs-aligned64-20260913/RESULTS.md (analysis.py can
regenerate). Common m22/m26/m30 Flock medians: 277.494604/452.370605/1635.948855 ms;
Mikros: 343.954613/498.490073/1166.604319 ms. M/F:1.23950/1.10195/0.713106.
Maximum-size Mikros has 28.6894% lower prover time under the shared bit setup.
Production word-wiring remains a separate reference (m30:557.966 ms), same
PCS but different PIOP/claims and full API assembly timing. Its verifier core
retains default 1T pool, unlike common global64 verifier; do not directly label
their verifier columns a matched-thread comparison. Server PID3596058 exited
successfully; SSH closed after retrieval. Evidence SHA256:
77f47016a4625ebc40101a9a132fc31cefc24e8db653517dea1831d28b41b4a4.

The following records describe the completed v1 baseline and its history.

User request: replace Flock Hadamard, packed IP and permutation by Mikros,
describe wiring at bit granularity, measure complete E2E prover time. Offline
conversion is excluded; online conversion is included. Server runs and iterative
source uploads under the previously authorized `20260912T-native64` directory
remain authorized. Never write credentials to artifacts.

Implemented in this task (not yet a completed server comparison):

- `examples/e2e_bit_compare.rs`: same SHA256 tree, bit sigma, full padded Z
  commitment and Flock functional/Ligerito PCS for both arms, kappa128/skip7.
- `examples/e2e_flock_baseline.rs`: separate production Flock dense/word baseline.
- `examples/e2e_common`: shared circuit, public-only verifier view, Flock
  ZC128/LC128, bit permutation/gather, explicit PoW challenge adapter.
- `src/packed_ip.rs` and `src/packed_ip/extension.rs`: binary and F128 Laurent
  IP respectively. F128 version is needed for the actual lincheck inputs.
- `src/e2e_lincheck.rs`: public matrix transpose, two Laurent IPs, constant pin.
- New root workspace member `mikros/permutation`: root Flock types, arbitrary
  bit sigma, strictly offline tuple/coefficient preparation and online proof.
- `crates/flock-core/src/pcs/functional.rs`: arbitrary 128 column weights and
  nonnormalized tensor rows, authenticated by original Flock Ligerito backend.
- `product_gkr::prove_batched_prepared_sigma_with_grinding`: move fixed sigma
  encoding offline while preserving original Flock proof bytes.

Validation completed before final source freeze:

- Mikros 58 library and 6 packed-IP integration tests, targeted Clippy passed.
- Root permutation 18 release tests and Clippy passed; prepared vs original
  proof/claims/next challenge identical across broad shapes and engine choices.
- Four Flock bit/helper/skip128 tests passed.
- Functional tensor-vs-dense and full m22 PCS+tamper tests passed.
- Both complete m22/4-thread E2E arms passed full verifier and all ring-claim
  tamper checks. Final public-output/PCS-backend negative checks added and are
  being rerun; exec session 46463 at the time this file was first written.

Server experiment driver `server_experiments/run_e2e_64.py` will use CPUs
`0-47,48-55,72-79`, native target, 64-thread pools, m22/m26/m30 via log-leaves
6/10/14. It retains all raw logs and failures, verifies source manifest before
and after, runs native validation before timings. Packaging script is
`server_experiments/package_e2e.py`. At first write no source uploaded or server
build/job started; read-only server connection confirmed swiftlab, 96 logical
CPUs, 377 GiB total / 366 GiB available RAM.

Final local validation is complete: both E2E arms fully verify; Flock rejects
18 negative cases and Mikros rejects 28. Results:
`mikros/results/e2e/common-bit-m22-4t-validation.jsonl`. Root E2E example Clippy
passes. The final immutable source bundle is `server_experiments/20260913T-e2e-bit-v1/release/`:
348 files, 1,690,713 archive bytes, SHA256
`f22f9d9316e4e9480fdffa29d2255ef84393d4ac74a2ded188fb5d06f174c21c`.
Earlier local `v1/` bundle was never uploaded and is superseded by `release/`.

Remote directory has been created:
`/home/mscong1/mikros-experiments/20260912T-native64/e2e-bit-v1/v1/`.
SCP upload was rejected by automatic approval review on 2026-09-13: it requires
explicit authorization for these newly added 348 E2E source files at this exact
destination despite earlier same-parent experiment authorization. Nothing was
uploaded and no server build/job started. A concise async authorization question
is pending; do not bypass the rejection or retry until the user approves this
scope. Unaffected local validation may continue. Once approved, upload the
existing immutable archive and checksum; verify checksum, safely extract under
the new remote directory, and launch `run_e2e_64.py` there.

While waiting for that authorization, the complete m26 case (2047 SHA gates,
4 local threads, one warmup and one measured proof per system) also passed both
full verifiers and deterministic transcript/proof checks. Its log is
`mikros/results/e2e/common-bit-m26-4t-validation.jsonl`; this is local validation,
not the requested 64-thread server comparison. No running local or remote job
remains at this pause. All agents' implementation subtasks finished.

Keep limits explicit: common full-padded PCS is not production dense/jagged
optimization; production reference remains separately labeled. No ZK masking
was added. The verifier has no witness input but scans public sigma (and SHA
fixture's public leaf messages grow with batch size). Do not claim an arbitrary
circuit polylog verifier. PoW is described per event/work factor, not as a proof
of total 128-bit security. Flock and Mikros fingerprint PoW both use log2(bit N);
Boolean challenges use PoW8; Flock tail native policy, Mikros cubic tail PoW2;
PCS uses original embedded Fast and opening policy.

Do not alter frozen prior binary_v5 or old experiment snapshots. The earlier no-commit preference was superseded by the user's explicit request on 2026-09-14 to commit the current results and synchronize with mirandaprivate/mikros_flock.

2026-09-13: User explicitly authorized the frozen 348-file upload. Upload completed and all archive/source SHA256 hashes verified on swiftlab. The checksum file uses CRLF; GNU sha256sum -c treated CR as part of filename, so the exact hash was verified with Python instead (passed). Server cgroup memory.max and memory.high both max. Native64 runner launched in the authorized e2e-bit-v1/v1 directory, results-e2e logs, log-leaves 6,10,14, one warmup + three formal repetitions per arm. No timing conclusions yet.

2026-09-13 COMPLETE: Server driver PID 3590981 finished 00:59:03--01:01:36 UTC+8. All build/test/validation and six performance processes exited zero. 44 native tests; Flock18/Mikros28 negative cases rejected. Downloaded 79 evidence files plus their manifest; archive SHA256 1b52be8c205e76264f52e1ac12c2d93e5f185a5f6935107ae1197e2bcbdabfc7. Evidence under mikros/results/e2e/server-native64-20260913/. Both source and executable hashes verified before/after. Common m22/m26/m30 Flock medians 136.270564/519.167687/3721.268363 ms; Mikros 161.324849/664.669553/4748.426768 ms. Production reference 81.244890/152.032020/572.617120 ms with separate layout/timing boundary. Independent audit agrees. analysis.py recomputes RESULTS.md and analysis.json from immutable evidence. New documentation/analysis only after the frozen release; no server job remains.

V2 2026-09-13: Local release build, merged_tensor3 and weighted4 tests passed; m22 full bit E2E+Flock23/Mikros33 negatives passed. All three local paths share exact config/query/grinding data and commitment digest. Clippy --no-deps passed with -A unknown-lints for a pre-existing lint name; local nonAVX512 dependency has a pre-existing unused import warning. Frozen 349-file v2 archive 1,700,562 bytes SHA256 28189bdd8c780c8a44f4ba152d0d63d52e27f745aae82d7739a10f9732ccb5cb uploaded and checked at /home/mscong1/mikros-experiments/20260912T-native64/e2e-bit-v1/v2/. Native64 driver started, results-e2e. No new server numbers yet.

2026-09-14 COMPLETE: Optimized packed permutation is the default E2E path (packed + lockstep); explicit legacy/both remain available. Native 64-thread final results and immutable v3/v4 evidence are documented in results/permutation-packed-20260914/OPTIMIZED.md. Default selection and legacy/cube compatibility were verified locally after promotion; outputs are in default-validation/. User requested committing current results and synchronizing with mirandaprivate/mikros_flock.
