# Parallel Goldilocks / KoalaBear comparison

This standalone workspace compares four Hadamard reductions over a shared scalar
field backend: Gruen, Gruen with skipped rounds, Mikros reference interpolation,
and Mikros optimized interpolation. Goldilocks uses the extension `X^2 - 7`
with packing factor 2; KoalaBear uses `X^4 - 3` with packing factor 4.

Run from the repository root on Linux:

```bash
RUSTFLAGS="-C target-cpu=native" cargo +nightly-2026-03-21 test \
  --manifest-path gruen/server64/Cargo.toml --release --locked
RUSTFLAGS="-C target-cpu=native" cargo +nightly-2026-03-21 run \
  --manifest-path gruen/server64/Cargo.toml --release --locked --bin compare -- \
  --n 20,30 --threads 64 --reps 8 --warmups 1 \
  --fields Goldilocks,KoalaBear --systems 0,1,2,3 \
  --out target/paper-results/gruen-parallel
```

Prover time includes public setup, allocation, packing, round messages, and
folding. Input generation and direct source evaluation are timed separately.
These PIOP experiments use a deterministic challenge tape and direct terminal
checks. The binary end-to-end experiments are described in the root README.

`reference/` contains the scalar reference used for differential tests;
`reference_manifest.json` records its source hashes.
