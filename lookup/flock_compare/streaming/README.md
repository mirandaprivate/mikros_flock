# Streaming Mikros permutation candidate

Independent successor to ../optimized. It preserves the original cubic proof,
claims, and transcript. The previous candidate is compiled separately as the
`previous` dependency and compared on identical paired input.

Completed N=2^30 server results and validation are in
[REPORT.md](server_experiments/20260912T103800-permutation-v2/REPORT.md).
See [ENGINEERING.md](ENGINEERING.md) for the implementation and tradeoffs.

At 48 physical cores, compact prover time including native input conversion
fell from 16.628 s to 13.065 s (five-run medians), and separate cold-process
peak memory fell from 97.49 GiB to 67.49 GiB. Flock measured 13.124 s and
158.01 GiB under the same experiment. These timings exclude the separately
reported source authentication. The full-extension variant saves memory but
does not improve core prover time.

Implemented changes:
- Compact layers are evaluated in at most 16 KiB blocks inside the first GKR
  message/fold scans, without a full expanded temporary array.
- Used tree layers are released synchronously and their release time is counted.
- Leaf lookup tables cover actual logical bits: 38-bit leaves in K64 use five
  byte tables instead of eight. Higher tree products retain full-width tables.
- `release` isolates early release with the old full materialization; `optimized`
  adds streaming, and `full` applies early release to full-extension trees.
- `previous` and `previous_full` run the unmodified earlier candidate.

All modes replay the verifier and authenticate every original source claim.
This remains a permutation PIOP comparison without PCS or grinding.
Validation checked 166 current benchmark proofs and matched 90 overlapping
proofs from the prior server experiment. Native differential tests and Clippy
passed. Previous Rust sources and the frozen baseline were preserved.

```sh
cargo +nightly-2026-03-21 test --release --locked -- --test-threads=1
cargo +nightly-2026-03-21 build --release --locked --bin compare
target/release/compare --log-n 20 --threads 6 --runs 6 --out results/NEW.json
```

On the recorded 48-physical-core server, reproduce the main timing comparison
with a new output path (peak memory needs separate single-system processes):

```sh
taskset -c 0-47 target/release/compare --log-n 30 --threads 48 --systems previous,previous_full,optimized,full,flock --runs 5 --warmups 1 --out results/NEW-n30.json
```
