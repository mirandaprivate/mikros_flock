# Optimized Mikros permutation PIOP

Completed server experiment: [N30 report and phase analysis](server_experiments/20260912T091817-permutation-v1/REPORT.md).
The report includes 24/48 physical-core runs, six paired measured inputs per
system, cold-process RSS, and the N26 ablation. All 180 local/server benchmark
proofs passed validation; the original proof format is preserved.

See [ENGINEERING.md](ENGINEERING.md) for calculation order, the unchanged cubic
message reconstruction, and why compact storage currently does not lower the
N30 peak RSS. Recompute the tables with:

```sh
python3 summarize_experiment.py --root server_experiments/20260912T091817-permutation-v1
```

This is an isolated, API-compatible candidate. The frozen baseline, previous
results, and concurrently developed lookup/PCS modules remain unchanged.

The public `permutation::prove`/`verify` and `Options` interfaces are preserved.
Every optimized mode produces exactly the reference proof bytes, claims, and
next transcript challenge. The verifier's arithmetic is unchanged. This is an
engineering optimization of the existing two-chain cubic GKR, not the complete
packed coefficient-domain reduction in the manuscript.

Changes:

- 8x8 SWAR bit transpose for parallel packed input construction and decoding.
- Same-modulus PCLMUL multiplication with byte reduction tables for K=16/32/64.
- F2-linear byte tables evaluate monic coefficient nodes at the challenge;
  per-node Horner multiplications become lookup/XOR operations.
- Parallel compact/extension tree construction; extension layers are borrowed
  by GKR, without an extra full-layer clone.
- Split equality weights reconstruct exactly the old cubic messages. Fold and
  next-message calculation share a pass; Flock's scratch pool is reused.
- Four-lane AVX-512/VPCLMUL tree, message and fold kernels on supporting CPUs,
  with a scalar/PCLMUL fallback and no change to field representation.
- A split-equality, parallel full-source checker authenticates every returned
  oracle and claim. It is not a PCS verifier.

`prove_profiled` exposes Reference, ParallelTree, LookupTree and Fused engines.
Reference retains the original arithmetic and allocations with phase timers;
all engines are checked against the separately compiled frozen baseline.
The `full` benchmark system applies the same optimized GKR to a full-extension
tree, to distinguish storage packing from arithmetic optimization.

The `compare` binary proves `f[sigma[i]]=g[i]` for all systems, with 8-bit values
and explicit position tags for Mikros. It uses a reproducible four-round
Feistel public permutation and hashes all actual f/g/sigma data before the
transcript. Input generation and common hashing are outside timing; native
representation conversion, prover core, verifier and complete source checks
are recorded separately. Results also save the actual proof bytes. For three
systems, six runs cover all six execution orders. For six ablations, six cyclic
orders balance every position. Initial subfield/table caches are in warmups.

Local ablations: `results/local-ablation-1t.json` and
`results/local-ablation-6t.json`, N=2^20 with six measured paired inputs.
Server driver: N26 ablation (same K=64 as N30); N30 with 24 and 48 physical
cores, six samples plus warmup per system, and isolated cold processes for RSS.
The driver waits for competing benchmarks/builds and records any interference.

```sh
cargo +nightly-2026-03-21 test --release --locked -- --test-threads=1
cargo +nightly-2026-03-21 clippy --release --locked --all-targets --no-deps -- -D warnings
cargo +nightly-2026-03-21 build --release --locked --bin compare
target/release/compare --log-n 20 --threads 6 --runs 6 --out results/NEW.json
```

Root verification alone is insufficient: the benchmark replays the verifier,
checks transcript synchronization, and authenticates all original source claims,
including Flock's sigma evaluation. PCS and grinding are excluded; no claim of
equal complete-SNARK security parameters is made. VmHWM in multi-system runs is
cumulative; use the separate cold-process runs for memory comparisons.
