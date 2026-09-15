# Multiple-instance tree engineering, N26 and N30

This experiment keeps the protocol, proof messages, challenges, terminal claims,
commitments and native FLOCK PCS unchanged. Multiple means **128 independent
inputs to the same circuit, each with N gates**: N30 is 2^37 total binary gates.

Both systems use 64 Rayon/all-core threads, affinity `0-47,48-55,72-79`, native
AVX-512/GFNI/VPCLMUL, the system allocator, and identical native PCS parameters.
Online includes conversions, commit/recommit, PIOPs, openings and releases.
Offline fixture/static preprocessing is excluded; verification is separate.
Process MaxRSS includes offline work.

## Changes

All new controls default OFF and affect only Mikros's multiple product-tree.

- `--tree-dense-simd`: contiguous loads and register deinterleaving for dense
  products, messages and folds. Implicit leaf values keep the original callback.
- `--tree-register-fusion`: compute the next three raw message sums directly
  from folded output registers, before storing the outputs. It removes the
  output-tile reload. There is no earlier transcript observation or challenge.
- `--tree-large-pool`: extend exact-capacity per-proof reuse through log L=32.
  At N30, L=4N. The tree retains at most 3L/2-2 field elements, or 96 GiB.
  Witness/public arrays add 80 GiB and the online scaled-leaf cache adds 48 GiB:
  these main copy-stage arrays total at most 224 GiB. This is an allocation
  accounting bound, not an RSS measurement. All tree capacities are released
  before terminal evaluation and PCS. The 350 GiB preflight stays in force.

The existing baseline enables packed-direct PCS and W-prime first-write for
both systems, plus virtual M, tiled Hadamard weights, paired copy terminals and
gate-to-Hadamard scratch for Mikros. Earlier selector SIMD/gate-fusion/direct
W-prime SIMD controls remain OFF. `--modes multiple` lets the harness measure
only the two full 128-instance cases without changing their size or checks.

## Retained evidence

The v11 N30 baseline completed and verified all four cases. Single is 76.641 s
for Mikros versus 115.446 s for FLOCK. Multiple is 259.522 s versus 310.901 s.
These are single fresh-process observations, not statistical speedup estimates.
Mikros multiple spends 14.110 s in gate, 52.123 s in copy, 108.139 s in the two
PCS openings, 66.136 s in commitments/recommitments and 19.012 s in releases.

The v12 contiguous-load experiment has three fresh-process samples per system
and arm, alternating arm order. Mikros copy median is 2.699513 s OFF versus
2.734924 s ON: **no improvement**, so this flag is not recommended. The E2E
median is 18.420885 s OFF versus 18.223587 s ON; that reduction does not come
from copy and is not credited to the new tree kernel. Traced diagnostics are
retained separately and excluded from these medians.

The v13 register-fusion experiment also has three fresh-process samples per
arm. Copy median is 2.654596 s OFF versus 2.694703 s ON: **no improvement**.
E2E median is 17.821013 s OFF versus 17.389512 s ON; again the lower total
cannot be credited to copy. Both SIMD experiments remain OFF in the N30
candidate. The final N30 experiment isolates the large-tree pool under the
same v13 binary, with unchanged FLOCK as the control in both arms.

The root `e2e_n30_tree_optimization_compare_flock.log` contains the audited raw
samples and phase table. Original `e2e_single_compare_flock.log` and
`e2e_multiple_compare_flock.log` remain unchanged historical evidence.

Source snapshots, manifests, native test logs, raw case output and resource
records are under `server_experiments/20260914-plonk-e2e/v11`, `v12` and `v13`.
The read-only audit tool is `n30-analysis/report_tree.py`; its negative checks
reject altered raw output, incomplete runs and missing native ISA evidence.

## N30 large-pool result

The same v13 binary completed both arms with one fresh process per system and
arm. Only `MIKROS_TREE_LARGE_POOL=1` differs; dense SIMD and register fusion are
OFF. All four proofs verify. Within each system, full serialized proof, PIOP,
terminal-claim and commitment fingerprints, proof size and PCS parameters are
identical across the two arms and the v11 N30 baseline.

| Online phase (seconds) | Mikros pool OFF | Mikros pool ON | FLOCK control before | FLOCK control after |
| --- | ---: | ---: | ---: | ---: |
| Complete prover | 254.375 | 241.172 | 318.454 | 308.179 |
| Gate PIOP | 12.826 | 15.087 | 26.240 | 26.324 |
| Copy PIOP | 48.283 | 34.544 | 85.956 | 84.449 |
| Witness PCS opening | 81.767 | 81.407 | 95.289 | 91.459 |
| Public PCS opening | 24.118 | 25.365 | 21.942 | 20.941 |
| Witness commitment | 29.542 | 28.515 | 25.545 | 26.280 |
| Witness recommitment | 30.694 | 27.597 | 27.884 | 23.532 |
| Public recommitment | 9.190 | 9.071 | 9.402 | 9.434 |
| Outer online releases | 17.955 | 19.585 | 26.196 | 25.759 |
| Process MaxRSS (GiB) | 344.059 | 344.068 | 343.396 | 344.075 |

Tree-local allocation and release work is included in Copy PIOP; it is not
added again to the outer release row. Serialization, under 0.002 s in each
case, is included in the complete prover total.

Mikros copy drops **28.46% (13.739 s)**, or **1.398x** throughput. The pool
reuses exact-size consumed tree layers and fold buffers within the proof,
including the second tree, instead of repeatedly allocating/freeing them.
Every acquired buffer is fully rewritten; no witness or challenge-dependent
computation is moved offline. Retention is local to copy and ends before PCS,
so the measured overall peak remains about 344 GiB despite the larger
copy-stage capacity bound.

The Mikros process minor-page-fault count falls from 410,405,425 to
350,492,508, consistent with less first-touch allocation work. These are
whole-process counters, not copy-only counters. Both Mikros arms and the
ON-arm FLOCK control have zero major page faults and zero filesystem inputs.
The OFF-arm FLOCK control has 109,159 major faults and 3,835,552 filesystem
input blocks; its higher total may include I/O effects. This is a further
reason to avoid attributing the cross-arm E2E difference solely to the pool.

Complete prover time drops **5.19%**. The unchanged FLOCK control also drops
3.23%, so the total-time change cannot all be attributed to this optimization.
The within-arm FLOCK/Mikros ratio is **1.252x OFF** and **1.278x ON**. In the
ON arm, Mikros's copy is 2.445x faster than FLOCK's copy; the complete prover
is not 2x faster. These N30 figures are descriptive single observations;
repeat the arms before treating the percentages as stable estimates.

Use `--tree-large-pool` for this server's memory-guarded N30 multiple runs.
Keep it opt-in for other deployments and keep both new SIMD flags OFF.
At N26, log L=28 is already pooled by the baseline; the large-pool flag changes
the N30 allocation policy rather than introducing another N26 optimization.
The remaining time is concentrated in native PCS openings (106.773 s,
44.27%) and commitments/recommitments (65.183 s, 27.03%). Copy is now 14.32%
of the total. Further copy-only acceleration has limited E2E headroom under
the requirement to preserve the common native PCS and avoid shared FLOCK
optimizations.

The source snapshots and raw records are frozen. v14 changes only one test's
environment-sensitive assertion so the full suite can run with large pooling
already enabled. `v14/production_equivalence.json` verifies all other source
files and the complete production portion of `extension_product.rs` are
byte-identical to measured v13.

Final v14 validation with the large-pool flag already ON passed **234 native
tests, 0 failed, 1 ignored**, plus both full N14 multiple E2E proofs. The N14
proofs equal v13 byte-for-byte. The 384 current source files match the tested
v14 manifest. Local harness checks (11), report negative checks (5), and the
13 portable tree tests also passed. `v14/final_validation.json` records the
final audit; `n30-analysis/audit_final.py` reproduces it from retained evidence.

To repeat the selected experiment from the server source checkout, choose a
fresh log directory for each process pair:

```sh
python3 mikros/server_experiments/20260914-plonk-e2e/run_server.py \
  --logs repeat-n30-large-pool --levels 30 --modes multiple \
  --packed-direct --wp-uninit --gate-virtual-product --wh-tiled-weights \
  --copy-paired-terminals --gate-wh-scratch --tree-large-pool
```

For the reference arm, omit only `--tree-large-pool` and choose another fresh
log directory. The harness fixes 64 threads/native ISA, keeps the 350 GiB
preflight, builds/tests by default and verifies both complete native PCS proofs.
