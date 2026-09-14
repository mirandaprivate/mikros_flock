# Fused first product layer for Mikros permutation

Independent v3 candidate; `previous` compiles the unchanged streaming v2 crate.

Completed results: [REPORT.md](server_experiments/20260912T120220-permutation-v3/REPORT.md).
Theory and cost derivations: [THEORY.md](THEORY.md).

At N=2^30 / 48 physical cores, six-run median core time fell from 11.263 s
to 10.069 s; including input conversion, from 13.442 s to 12.292 s. Peak
memory stayed near 67.49 GiB. Flock measured 13.262 s including conversion
and 158.01 GiB. Including verifier replay and direct source authentication,
v3 measured 14.736 s versus Flock's 13.860 s; this still excludes PCS.

96 selected benchmark proof files passed validation, with 124 overlapping v2
proof files matched byte-for-byte. All ten accepted server cases completed;
one earlier 24-core attempt was excluded for competing build processes.

For K64 inputs and a coefficient capacity of 128 bits, fuse bit decoding,
compact leaf storage, leaf evaluation and first-layer pair multiplication.
This keeps the same source oracles, cubic proof bytes, transcript and verifier.
Other widths, capacities and the full-extension strategy retain the v2 path.

Benchmarks include native input conversion and synchronous cleanup. Source
authentication is separately measured. This is a permutation PIOP, without PCS,
commitments, openings or grinding. See THEORY.md for protocol comparison.

`compare --systems previous,optimized,flock` selects v2 compact, v3 fused,
and the frozen Flock batched Product-GKR, respectively. `previous_full` and
`full` remain available as full-extension controls; the fusion does not apply
to those paths.

Validation and a local comparison (requires a Linux host with sufficient RAM):

```sh
export RUSTFLAGS='-C target-cpu=native'
cargo +nightly-2026-03-21 test --release --locked -- --test-threads=1
cargo +nightly-2026-03-21 build --release --locked --bin compare
target/release/compare --log-n 26 --threads 6 --systems previous,optimized,flock --runs 6 --warmups 1 --out results/NEW.json
```

N26 exercises 34 logical bits in K64. The usual N20 / 8-bit-value comparison
uses K32, so it exercises the unchanged fallback rather than the new fusion.
Use fresh output paths; proof files are saved next to each JSON report.

Server experiment artifacts are under
`server_experiments/20260912T120220-permutation-v3/`.
