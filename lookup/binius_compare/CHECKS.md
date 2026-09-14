# Validation performed

- `cargo +nightly-2026-03-21 fmt -p mikros-binius-permutation -- --check`: passed; final formatting followed the one Clippy correction.
- `cargo +nightly-2026-03-21 clippy --release --offline -j 2 --all-targets --no-deps -- -D warnings`: passed for the adapter library and benchmark.
- `cargo +nightly-2026-03-21 test --release --offline -j 2 -- --test-threads=1`: all three test groups passed, including backend-message mutations and a column mutation preserving the PIOP linear claim.
- `cargo +nightly-2026-03-21 build --release --offline -j 2`: passed.
- All 308 copied source files matched the snapshot manifest SHA-256 values on inspection. Base revision: `b684b1258e4b1f202bec24afd660ace851b09e5e`.

The unmodified dependency snapshot emits existing unused AVX-512 import and generated lookup feature-check warnings on this AVX2 host. They were not silently changed or treated as adapter lint failures.

An initial small-shape test exposed an unsupported zero-initial-fold path in the upstream diagnostic PCS. The adapter now requires `log_n >= 15`, ensuring a positive initial folding parameter. All reported benchmark shapes meet that condition. A proposed mutation of Flock's compatibility `s_sigma_eval` field was replaced with a mutation of its authenticated `f_eval`: the sigma-aware verifier intentionally recomputes sigma's value from the public mapping.

`results/pilot-20260912/` is a four-proof pilot, not part of the final statistics. `results/local-20260912-r1/` is an interrupted exploratory run: its million-row batch overlapped a concurrent Rust build, and the runner stopped. The finalized runner retains and retries entire build-interfered batches; exclusions do not depend on observed duration. Only the accepted complete batches listed in the final run's `execution.json` enter its summary.

Measurements use actual serialized/deserialized proofs. Every accepted sample checks both source openings and prover/verifier transcript equality, plus identical input and commitment identifiers across the paired systems. The runner records source and executable hashes and checks they remain unchanged during measurement.
