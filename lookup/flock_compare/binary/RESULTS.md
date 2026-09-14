# Binary permutation v4: server results

Completed on the dual Xeon Gold 6342 server: N = 2^30 permutation items,
1-bit values, 30-bit public position tags, K = GF(2^32), E = GF(2^128).
All systems used 64 threads on `0-47,48-55,72-79` and the same native build,
with GFNI, AVX-512 and VPCLMUL enabled. This is 48 physical cores plus
16 SMT siblings on a shared server.

Times below are medians of six measurements after one warmup. Peak RSS is
measured separately in one cold process per system and includes common inputs.

| Metric | Previous Mikros v3 | K32 fusion v4 | Flock batched Product-GKR |
|---|---:|---:|---:|
| Prover core, seconds | 9.722 | 8.620 | 12.060 |
| Prover including native conversion, seconds | 11.489 | 10.277 | 13.174 |
| Including verifier and full source authentication, seconds | 13.517 | 12.318 | 13.792 |
| Peak RSS, GiB | 49.736 | 49.737 | 158.000 |
| Actual proof bytes | 44,208 | 44,208 | 16,176 |

The new core median is 11.3% below the previous implementation and 28.5%
below Flock. Including native input conversion, it is 22.0% below Flock.
All six paired inputs favor v4 against both baselines for core and total
prover time. The memory advantage over Flock predates this optimization;
the new fusion did not reduce the observed process peak relative to v3.

The changed construction stage combines decoding, retained K32 leaves,
retained quadratic coefficients, and direct extension-layer production.
Its mean falls from 4.342 to 2.492 seconds, a reduction of 42.6%. The GKR
messages/folding mean remains about 3.0 seconds. Synchronous layer release
increases from 1.891 to 2.618 seconds, offsetting part of the construction
gain. Hardware counters were not measured, so the release-time difference
is not attributed to a specific cache, NUMA or allocator mechanism.

Cold-start behavior is separate: the single cold v4 prover core took
10.884 seconds, compared with v3's 10.537 seconds. It therefore does not
establish a cold-start speedup. The width8/N26 regression uses the unchanged
K64 path and does not establish an additional optimization benefit.

All 8 server tests and 78 benchmark proof records passed. The new and old
Mikros proofs, claims and challenges remain equivalent. Every timed record
replays the verifier and authenticates every source claim. All 110 downloaded
evidence files and 394 source/build inputs passed SHA256 verification, with
no excluded attempts. The driver finished and the SSH session was closed.

This comparison proves the same public relation `f[sigma[i]] = g[i]` but
does not include PCS, commitments, openings or grinding, and does not claim
equal security parameters. Mikros still has two separate cubic GKR chains;
Flock's batched proof is smaller. Source authentication is a full direct
input check, not PCS verification. This optimization is available in the
independent `mikros-permutation-binary` crate; historical experiments and
the original protocol crates are preserved.

[Complete report](server_experiments/20260912T-binary-v4/REPORT.md),
[machine-readable statistics](server_experiments/20260912T-binary-v4/SUMMARY.json),
[raw main measurements](server_experiments/20260912T-binary-v4/results/main-w1-n30-64t-attempt1.json),
[local validation](LOCAL_VALIDATION.md).
