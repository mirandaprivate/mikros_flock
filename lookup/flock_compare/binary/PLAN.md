# Binary permutation server comparison

Run only inside `/home/mscong1/mikros-experiments/20260912T-native64/permutation-binary-v4`
or a fresh directory with that name plus a suffix. The source package contains
`source/binary`, its frozen fused v3 predecessor and snapshot Flock dependencies.
No historical source or results are overwritten.

Primary values are binary (`--width 1`). At N30 the permutation tuple has
1 value bit plus 30 index bits, so its coefficient field is K32. The candidate
fuses the applicable K32 construction. `previous` is fused v3; `optimized` is
this candidate; `flock` is snapshot `product_gkr::prove_batched`.
This is a permutation PIOP comparison, without PCS/grinding or Hadamard skip
alignment. It makes no equal-security or full-SNARK claim.

All children use `taskset -c 0-47,48-55,72-79`: 64 logical CPUs covering 48
physical cores and 16 SMT siblings, with 32 logical CPUs on each NUMA node.
Both prover pools request 64 threads. Linux default first-touch remains in
effect; the host is shared and there is no CPU exclusivity.

```bash
python3 source/binary/server_driver.py --root "$RUN_ROOT" --widths 1
```

For every selected width, run smoke N20 (one warmup, one measured input), then
screening N26 (one warmup, six measured inputs), with all three systems. Width1
also runs N30 (one warmup, six measured inputs) followed by one independent
N30 cold process per system (one proof, zero warmups). All six permutations of
the three-system order occur once in each six-input main/screening case.
Cold seed1 matches main run1 for independent input/proof-identity verification.

`--widths 1,8` adds width8 smoke/N26 regression only. Add `--width8-n30` only
when the additional width8 N30 main/RSS work is authorized. At N30 width8 has
38-bit tuples and K64; never pool its samples or conclusions with width1.
The full configured case list is recorded in environment.json. Default width1
has 6 cases and 51 proof records; width8 smoke/screening adds 27 records, while
its optional N30 main/RSS adds 24 more. Actual accepted counts are validated
from the recorded plan and samples, not assumed by the report.

The driver waits for three quiet checks before commands and each case. N30
requires at least 220 GiB available RAM, also accounting for finite ancestor
cgroup limits. It watches same-user cargo/rustc/linker/benchmark processes,
records interference, and retries the complete affected case at most three
times. `--max-idle-seconds` defaults to 1800, bounded to 15–7200. It never
kills another job; failed waits and every excluded attempt remain recorded.

Build sequence: locked Cargo fetch, then native release tests and build using
`--offline`, nightly2026-03-21 and `-C target-cpu=native`. Tests must pass before
the benchmark. Source hashes are checked before and after; executable SHA256
is recorded and checked before every case and after completion.

Every proof includes verifier replay, claims/transcript equality and full
source authentication before its successful record. The driver checks saved
proof bytes, same-input pairing, JSON/JSONL agreement, execution order and RSS.
Native input conversion is in `prover_total_ms`; core includes synchronous
cleanup; public input generation/digest and final source authentication are
outside those timings and reported separately. Do not replace source checks
with sampled checks or describe them as PCS verification.

Download complete logs, JSON/JSONL and actual proof files with SHA256 manifests.
Run `summarize_experiment.py --root <downloaded-directory>` only after complete
status. Keep width/N separate and include median, mean, range, paired ratios,
win counts, cold RSS, verifier/source time and actual proof sizes. Retain all
negative results and excluded attempts. Phase comparisons group fused work
with the old decode/build/evaluate sum; no hardware-counter causes are inferred.
