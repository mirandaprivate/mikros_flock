# Binary permutation v4: completed

User explicitly authorized the 394-file package and server destination.
Uploaded archive SHA256:
a611bfc25b7f1dafb788843c26f5b84e336d862adccc2667c458571598ddb437

Server root:
/home/mscong1/mikros-experiments/20260912T-native64/permutation-binary-v4/
Driver PID 3575030 and parent 3575029 finished. SSH session 66023 was closed.
No own experiment or transfer process remains active. Do not restart the run.
Completed 2026-09-12T14:20:15Z, 8 cases, 78 proof records, 8 native tests,
110 downloaded evidence files. No excluded attempts. Every source/build
hash and new/old Mikros proof identity check passed.

Local results: server_experiments/20260912T-binary-v4/
Entry point: RESULTS.md; full REPORT.md, SUMMARY.json and summary.csv are
in the result directory. Raw JSON, JSONL, actual proofs, environment,
commands, waits, source archive and hashed evidence archive are retained.
Results archive SHA256:
6461e1e5378211e418dc8203c9161b84164410ace1420be32a595d984abf77dd
Executable SHA256:
b3ebe1efce5b675de1f82992b73263b771efc9ea6281bc402dd3abfaaf9a3467

Main parameters: N30 items, 1-bit values plus 30-bit position labels, K32,
E128. 64 threads = 48 physical cores plus 16 SMT siblings, CPU affinity
0-47,48-55,72-79. Native GFNI/AVX-512/VPCLMUL enabled. Width8 only ran N20/N26
regression; no width8 N30 in this experiment. No Hadamard skip comparison.

Six-run warm medians (v3 / new / Flock):
core 9.722 / 8.620 / 12.060 s;
including conversion 11.489 / 10.277 / 13.174 s;
including verifier/full source 13.517 / 12.318 / 13.792 s.
Cold RSS 49.736 / 49.737 / 158.000 GiB.
Proof bytes 44,208 / 44,208 / 16,176.
New core improves 11.3% over v3, 28.5% over Flock; all six pairs win.
Single cold new core 10.884 s is slower than v3's 10.537 s; no cold-start
speedup claim. Construction mean 4.342 -> 2.492 s; layer release mean
1.891 -> 2.618 s offsets some gain. No hardware-counter attribution.

Candidate is an independent crate in binary/, previous dependency is
unchanged fused v3. K32 fusion retains the two required compact layers,
skips the transient quartic coefficient layer, preserves messages/claims/
transcript. Existing v1/v2/v3 and root code were not changed. Main protocol
crates were not switched to this experimental candidate. No PCS/grinding
or equal-security claim. All required work complete; no git commit or
memory writes. Relevant memory context: MEMORY.md 3-4, rollout
01a056d0-ce09-7a91-b8e9-9483540e2e93.
