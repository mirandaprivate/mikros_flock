# Parallel Mersenne-31 / KoalaBear / Goldilocks comparison

This standalone workspace compares four Hadamard reductions over a shared scalar
field backend: Gruen, Gruen with skipped rounds, Mikros reference interpolation,
and Mikros optimized interpolation. The packing factor equals the challenge
extension degree, and Gruen skips `log2(kappa)` rounds in each field:

| Base field | Challenge extension | Mikros kappa | Gruen skip | Gruen skip domain |
| --- | --- | ---: | ---: | --- |
| Mersenne-31 | `i^2 = -1`, `u^2 = 2 + i` | 4 | 2 | General Lagrange nodes |
| KoalaBear | `X^4 - 3` | 4 | 2 | Radix-2 coset |
| Goldilocks | `X^2 - 7` | 2 | 1 | Radix-2 coset |

All three challenge fields occupy 16 bytes per element. Mersenne-31 has
two-adicity one, so its skip-two path uses a general Lagrange adaptation of the
quotient reduction, with nodes `0,1,2,3` and extra points `4,5,6,7`.

Run from the repository root on Linux:

```bash
RUSTFLAGS="-C target-cpu=native" cargo +nightly-2026-03-21 test \
  --manifest-path gruen/server64/Cargo.toml --release --locked
RUSTFLAGS="-C target-cpu=native" cargo +nightly-2026-03-21 bench \
  --manifest-path gruen/server64/Cargo.toml --locked --bench compare -- \
  --n 26,30 --threads 64 --reps 3 --warmups 1 \
  --fields Mersenne31,KoalaBear,Goldilocks --systems 0,1,3 \
  --out target/paper-results/gruen-parallel
```

The bench reports progress and per-sample prover time, verifier time, and proof
element bytes. The output directory contains `raw.csv`, `summary.csv`, proof
coordinates, and `completion.json`; use a new output directory for each run.
To retain console progress as well, append `2>&1 | tee gruen-compare.log` to the
command. System numbers select Gruen (0), Gruen with skipped rounds (1), Mikros
reference (2), and Mikros optimized (3).

Gruen skips one round for Goldilocks and two for KoalaBear and Mersenne-31, matching the Mikros
packing factors. Three repetitions rotate all three selected systems through
each execution position; each system also has one excluded warmup. For CPU
pinning, prefix the command with `taskset -c CPU_LIST` using CPUs on your machine.

For a witness-size sweep with the same packing factor for both systems in each
field (Mersenne-31 and KoalaBear: 4, Goldilocks: 2), select systems 1 and 3:

```bash
RUSTFLAGS="-C target-cpu=native" cargo +nightly-2026-03-21 bench \
  --manifest-path gruen/server64/Cargo.toml --locked --bench compare -- \
  --n 16,18,20,22,24,26,28,30 --threads 64 --reps 6 --warmups 1 \
  --fields Mersenne31,KoalaBear,Goldilocks --systems 1,3 \
  --out target/paper-results/gruen-size-sweep
```

Here `--n` gives the logarithm of the number of Hadamard rows. Each row stores
three base-field elements, so the input arrays occupy `24 * 2^n` bytes over
Goldilocks or `12 * 2^n` bytes over Mersenne-31 and KoalaBear; these are not peak RAM readings.
Six repetitions alternate the execution order of the two systems equally.

For a smaller local run, use `--n 20,22,24,26 --threads 4` with the same field
and system selections. Large runs require additional memory for extension-field
tables and folding buffers beyond the input arrays. Choose the worker count for
the machine running the experiment; `server64` is only the workspace name.

Mikros optimized uses finite-difference extrapolation, 256-row tensor-factored
column aggregation, and fused fold/next-message computation with reusable
buffers. Differential tests check every proof coordinate against the frozen
serial reference. In the phase CSV, `round_fold_ms` includes the fused next
message and scratch allocation/disposal; compare `rounds_ms + round_fold_ms`
when examining the tail across implementations.

Prover time includes public setup, allocation, packing, round messages, folding,
column consistency, and cleanup. Input generation and direct source evaluation
are timed separately. Proof element bytes count raw field elements, without
serialization or PCS. Both systems use the same scalar modular-arithmetic
backend; this is not a benchmark of the native SIMD Plonky3 implementation.
These PIOP experiments use a deterministic challenge tape and direct terminal
checks. The binary end-to-end experiments are described in the root README.

`reference/` contains the scalar reference used for differential tests;
`reference_manifest.json` records its source hashes.
