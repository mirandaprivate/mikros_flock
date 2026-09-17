# FLINT extension arithmetic benchmark

Standalone C harness for FLINT's public `fq_nmod` API. This experiment does not
change the existing Rust field implementation or any Cargo workspace files.

| Base field | Prime | Extension degree | Defining polynomial | log2(field order) |
|---|---:|---:|---|---:|
| F241 | 241 | 16 | X^16 - 7 | 126.606229380 |
| F61441 | 61441 | 8 | X^8 - 11 | 127.255312614 |
| KoalaBear | 2130706433 | 4 | X^4 - 3 | 123.954738750 |
| Goldilocks | 18446744069414584321 | 2 | X^2 - 7 | 127.999999999 |

The 16/8/4/2 extension degrees follow the previous approximately-128-bit,
16-byte compact-representation convention. These are not fields of exactly
2^128 elements, nor claims of 128-bit protocol soundness. FLINT itself stores
each polynomial coefficient in a 64-bit word in this build.

The four binomials are irreducible. The harness checks primality and
irreducibility with FLINT. `verify_fields.py` independently checks the same
parameters with Python integer arithmetic and Rabin's polynomial criterion.
The binomials are fixed explicitly, without timing-based polynomial selection.

## Reproduce on this Ubuntu 22.04 x86_64 WSL environment

```sh
cd gruen/flint_bench
bash setup.sh
python3 run.py --sanitize --reps 1 --ms 10 --out results/sanitizer-reproduction
python3 run.py --reps 9 --ms 150 --cpu 2 --out results/reproduction
```

The recorded experiment also ran a second 9-repetition session with `--ms 300`
after observing timing spikes. `summarize.py OUTPUT_DIR RUN1_DIR RUN2_DIR` pools
all samples with equal weight per sample, without excluding slow observations.

Each output directory must be new. Runs are serial. Do not run a benchmark
concurrently with a compiler or another CPU-intensive job.

`setup.sh` downloads the official FLINT 3.6.0 release, extracts Ubuntu GMP/MPFR
development headers and the m4 executable locally, then builds FLINT with
`-O3 -march=native`, shared-library and CPU-specific assembly/FFT support.
The matching pre-existing system GMP/MPFR runtimes are linked through local
symlinks. Nothing is installed into system directories. Ubuntu package versions
and hashes are saved; this setup script is specific to the recorded environment.
The C harness is compiled with GCC, `-O3 -march=native -Wall -Wextra -Werror`.

## Measurement definition

- One FLINT thread, pinned to one logical CPU (default CPU 2).
- 256 independent random input pairs reused in fixed buffers. This is
  hot-buffer public-API throughput, not a streaming or dependency-chain benchmark.
- `add`, `sub`, `mul`, `square`, `scale` (extension times base scalar),
  `inverse`, and `fold` are measured separately.
- `fold` means one complete `a + r*(b-a)`, where `r` is a fixed extension
  challenge shared by the batch; it contains three FLINT API calls.
- Every output is distinct from its inputs. Scratch and output buffers are
  reserved for up to `2*degree-1` coefficients. This avoids output growth in
  timed loops but does not remove library-internal temporary work/allocations.
- Context construction, irreducibility checks, random-input generation,
  preallocation, calibration, warm-up, and output validation are untimed.
- Independent inputs use rejection sampling with a reproducible SplitMix64
  stream, seeded by `0x464c494e5442454e XOR p`. Inverse inputs are nonzero.
- Each field/operation is calibrated to approximately 150 ms per sample, then
  its operation count is held fixed for all 9 repetitions. Field and operation
  order rotate across repetitions. Each cell is warmed up again before timing.
- `CLOCK_MONOTONIC` measures wall time; thread CPU time is recorded separately.
  No estimated loop/call overhead is subtracted. The median of wall ns/op is
  the headline result. `Mops/s = 1000 / ns/op`.
- Dynamic FLINT calls are made directly from C, with no Python/Rust FFI in the
  timed loop. All output coefficients are consumed and checked after each cell.

## Validation and outputs

The C oracle performs schoolbook polynomial multiplication with ordinary
unsigned 128-bit integer products reduced modulo the base prime. It does not
call FLINT arithmetic. It checks add/sub/multiply/square/scale/fold and verifies
inverses by independently multiplying them back to one. Tests include zero,
one, maximal coefficients, the highest monomial, dense maximal coefficients,
and a generator product crossing the modulus degree. Zero is not inverted.
All result batches are checked after calibration and every measured sample.

The sanitizer run instruments the harness with AddressSanitizer and UBSan;
the separately built FLINT/GMP/MPFR libraries are not sanitizer-instrumented.
Sanitizer timings are not performance results.

Each completed run saves raw trials, summary CSV, field parameters, library
versions, compiler command, CPU/build details, hashes, validation results, and
a compact Markdown table. `completion.json` is written only after all checks
and expected sample counts pass. Cached dependencies and build products are
ignored by Git; benchmark source and measurement artifacts remain reviewable.

## Sources

- [FLINT releases](https://flintlib.org/downloads.html)
- [FLINT fq_nmod API](https://flintlib.org/doc/fq_nmod.html)
- [FLINT nmod_poly representation](https://flintlib.org/doc/nmod_poly.html)
- [Plonky3 KoalaBear parameters](https://github.com/Plonky3/Plonky3/blob/main/koala-bear/src/koala_bear.rs)

The downloaded FLINT source tree retains its original license and notices.
