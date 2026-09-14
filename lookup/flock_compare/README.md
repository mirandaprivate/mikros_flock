# Mikros / Flock permutation comparison

Completed local run: `results/local-20260912-permutation-r3/REPORT.md` and
`summary.csv`. All 237 accepted proofs passed complete source checks. The two
earlier pilot directories are retained as excluded runs with recorded build
interference; they are not included in the reported medians.

This experiment proves the same public relation `f[sigma[i]] = g[i]` using:

- Mikros `Strategy::BitwidthAware` (compact coefficient product tree).
- Mikros `Strategy::FullExtension` (control with the same messages).
- Flock's existing `product_gkr::prove_batched` with all rows live.

Mikros's existing API proves multiset equality. The adapter encodes `(f[i], i)`
and `(g[i], sigma[i])` as injective bit strings, with the value in the low bits.
Thus its input width is `value_bits + log_n`. Flock uses its native randomized
value/position fingerprints. Comparing untagged Mikros multisets to Flock's
tagged relation would omit the public permutation constraint.

All three use the same raw inputs per run, deterministic public permutations,
F128 backend, BLAKE3 Fiat-Shamir challenger, and a common statement digest.
Mikros retains its own internal statement checks and hashing. The verifier
replays each proof, checks transcript synchronization, and authenticates every
terminal source claim by evaluating the source arrays. Flock's public sigma
evaluation is checked, including an additional run of its sigma-aware verifier.
These are PIOP measurements with full source checks outside the prover timer;
there are no PCS commitments/openings, zero-knowledge wrappers, grinding, or
claims of identical security parameters.

`snapshot/manifest.json` records frozen protocol source hashes. Only the
snapshot lookup crate's module declarations and manifest are generated to
exclude unrelated, concurrently developed Lasso and PCS modules. Protocol
implementation files are copied unchanged. This independent workspace and
target directory do not change the original protocol crates.

## Timing and validation

- `prepare_ms`: build native representations from the common raw integers.
- `prover_core_ms`: actual public prover call, including its internal checks,
  hashing, allocation, representation conversion, and proof construction.
- `prover_total_ms`: preparation plus the prover call.
- `verifier_core_ms`: PIOP replay returning deferred evaluation claims.
- `source_check_ms`: authenticate all returned source evaluations.
- `sigma_aware_verifier_ms`: Flock's additional verifier with public sigma.

Common raw data generation, statement digest, and challenger prefix are outside
timing. Proof serialization is outside timing. Message sizes are bincode bytes
without the public statement, sigma, commitments, or openings.

Each size gets one saved warmup and six measured runs, covering all six orders
of the three systems. No slow samples are discarded. Single-thread tests cover
`log_n = 12,16,20`, value widths `1,8,32`; six-thread tests cover `log_n = 16,20`
and width `8`. Linux affinity uses one logical CPU per reported physical core.
The driver waits for competing builds to finish and records any build
interference during measurement. Each size/thread configuration is a separate
process with its own warmup. If a build starts during a case, that entire attempt
is retained but excluded, and the whole case is retried after the build ends
(up to five attempts). Exclusion never depends on a measured duration.
Windows/WSL is still a shared host.

Current Mikros's tree construction and sumcheck/fold loops are scalar. Its
shared equality-table routine uses Rayon on large tables. Flock uses its
production parallel paths. Compact Mikros reconstructs full-extension values
for two separate degree-three GKR chains; this is not the manuscript's full
packed coefficient reduction implementation.

Unit tests cover honest proofs, changed values, incorrect public mappings,
tampered messages, and the public sigma source claim. Every timed sample is
fully checked. `summarize.py` independently recomputes statistics, checks the
paired input digests, compact/full proof identity, cross-thread proof identity,
balanced run orders, and exact message counts.

## Reproduce in WSL/Linux

Run from this directory. Use the existing snapshot for the recorded experiment;
`prepare_snapshot.py` creates a new snapshot only when the directory is absent.

```sh
cargo +nightly-2026-03-21 fmt -p permutation-flock-compare -- --check
cargo +nightly-2026-03-21 clippy --release --offline -j 1 --all-targets --no-deps -- -D warnings
cargo +nightly-2026-03-21 test --release --offline -j 1 -- --test-threads=1
cargo +nightly-2026-03-21 build --release --offline -j 1
python3 run_experiment.py --name NEW_RUN_ID
python3 summarize.py results/NEW_RUN_ID
```

The runner requires six available physical cores and Linux `taskset`/`lscpu`.
Results include JSON samples, per-sample JSONL recovery logs, commands, source
and executable hashes, environment metadata, a Chinese report, and CSV.
