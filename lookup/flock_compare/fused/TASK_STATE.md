# V3 fused permutation optimization: completed 2026-09-12

User requested further Mikros permutation optimization and theory comparison
with Flock. Both implementation and experiments are complete. This was a side
conversation; no subagents or other tasks/jobs were used or modified.

Independent candidate in fused/, using streaming v2 as the previous dependency.
All v1/v2 Rust and frozen snapshot files matched their earlier hashes during
packaging. Runtime Rust remained unchanged during the server experiment.
Only this candidate and its support artifacts were added. No commits.

See THEORY.md for protocol/field-width/soundness/message/memory derivations.
Report and raw samples:
server_experiments/20260912T120220-permutation-v3/REPORT.md

Completed at 2026-09-12T12:32:35Z. All ten accepted server cases succeeded.
One main-n30-24t attempt was excluded for competing cargo/rustc processes;
its artifacts were retained, and the complete group was rerun after idle.
The 48-core group had no detected monitored interference. Shared host.
Own driver PID3539263 finished. Own SSH session84891 was closed successfully.
No local/server processes for this task remain active. Do not restart them.

Validation: native local differential tests and Clippy passed; native server
AVX512/VPCLMUL tests passed. All 137 downloaded result files passed hashes.
96 selected proof files validated; 124 overlapping v2 files matched by actual
SHA256/input/proof digest. 32 distinct statement/protocol pairs. Source claims
fully authenticated; unchanged cubic proof/transcript/verifier. No PCS.

Six-run median N30 / 48 physical cores:
- v2: core11.262695s, including conversion13.442464s, direct source2.464589s,
  RSS67.491GiB; pipeline incl verifier/source15.903687s.
- v3: core10.068783s, including conversion12.291605s, direct source2.447483s,
  RSS67.492GiB; pipeline incl verifier/source14.735645s.
- Flock: core12.055737s, including conversion13.261631s, direct source0.595541s,
  RSS158.008GiB; pipeline incl verifier/source13.860243s.
24-core v2/v3 core14.094479/11.992997s; including conversion17.423077/15.348173s.

Fusion stage48t: v2 4.552s -> v3 2.647s; synchronous layer release2.496s ->
3.081s partly offsets savings. Peak memory unchanged. v3 uses first-layer
F128 products instead of K64 coefficient products; this is a SIMD/lookup/
array-traffic optimization, not a reduction in EE multiplication count.

Source package: 375 whitelist files, 1,705,812 bytes.
SHA256 22af1c546f87ff9be28c36a0f18a2bb1fbb1622cb59d570a4491d91f42cf0971
Results archive: 3,100,824 bytes.
SHA256 f2aa5599d72d61f961564c86e1d365dacedee0578c2f0e1a0a2fda44ffd9191d
Binary SHA256 202cb93a8df9ea5f14b2da1713b988a7b8fb72c74856ff6646fd7aa03aba9df5

Future requests: preserve this completed source/results snapshot; use a new
candidate for changed protocols. Current theory highlights possible batched
quadratic messages, input conversion/source check optimization, and full
manuscript coefficient-domain reduction as distinct follow-up work.
Do not rerun package_source.py into the completed experiment directory.
No more checks or benchmarks are necessary for this completed request.
