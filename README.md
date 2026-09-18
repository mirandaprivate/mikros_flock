# Mikros

Supplementary code for the **Mikros** paper, including the Mikros implementation
and experiments comparing it with **FLOCK (skip = 7)**.
The Hadamard experiment also compares the original Binius sumcheck with delayed
extension-field folding (`switchover`).

## Repository structure

| Path | Contents |
|---|---|
| `mikros/src/` | Packed Hadamard, inner-product, and multiple-instance reductions. |
| `mikros/permutation/` | Packed permutation proof. |
| `mikros/examples/e2e_hash_compare.rs` | End-to-end SHA-256, Keccak, and BLAKE3 comparisons. |
| `mikros/examples/e2e_common/` | Shared circuit, witness, reduction, and PCS integration code. |
| `mikros/benches/e2e_*.rs`, `binary_hadamard.rs` | Multiple-instance hash and Hadamard `cargo bench` entry points. |
| `mikros/benches/single_hash_compare.rs` | Single-instance comparison across all three hashes. |
| `mikros/benches/paper_bench.py` | Process orchestration, measurement checks, and console/log reports. |
| `mikros/benches/univariate_vs_flock.rs` | Binary Hadamard measurement worker. |
| `mikros/binius_switchover_hadamard/` | Original Binius Hadamard comparison with a pinned upstream version. |
| `crates/flock-*/` | FLOCK implementation, Ligerito PCS, and shared field and parallel kernels. |

## Build

**CPU requirement for reported performance:** Mikros relies on **GFNI and
AVX-512 (`avx512f`, `avx512bw`)** for efficient extension-field operations.
GFNI accelerates GF(256) coordinate multiplication and basis conversion;
vectorized GF(2^128) multiplication additionally uses **VPCLMULQDQ**.
**On personal computers without these CPU features, Mikros uses slower fallback
kernels and is unlikely to achieve the efficiency reported for the server.**
Enabling `-C target-cpu=native` does not provide instructions missing from the CPU.

Run the following commands from the repository root in a Linux Bash shell.
The paper experiments use one thread pinned to one CPU and native CPU
kernels. The recorded server runs used an Intel Xeon Gold 6342 with GFNI,
AVX-512, and VPCLMULQDQ; CPU 16 was used for the one-thread runs.
Install Rust through rustup, a C/C++ build toolchain, Python 3.10 or newer, and GNU time.
Native CPU code generation is enabled in `.cargo/config.toml`.

```bash
rustup toolchain install nightly-2026-03-21 --profile minimal
export RUSTFLAGS="-C target-cpu=native"
```

## Experiments

Each experiment uses `cargo bench` to build and run its complete comparison.
Defaults are one thread,
256 instances for the `e2e_*` entries, one instance for `single_hash_compare`,
and `N = 2^30` for Hadamard. The E2E comparisons use
FLOCK's native Ligerito Fast PCS; Mikros packs instances in groups of 128.
Run the experiments sequentially. On Linux, a one-thread run selects the first
allowed CPU automatically. Append `-- --cpu 16` to select a specific CPU, and
keep its SMT sibling idle.

### 1. SHA-256 compression

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench e2e_sha256
```

### 2. Keccak-f[1600] permutation

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench e2e_keccak
```

### 3. BLAKE3 compression

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench e2e_blake3
```

### 4. Binary Hadamard with 2^30 constraints

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench binary_hadamard
```

This compares Mikros's `pairreuse` kernel, FLOCK skip-7, and original Binius SFF.
The pinned Binius source is fetched if needed and built in a separate workspace
before measurement. A prebuilt worker can be supplied with `--binius-bin PATH`;
`--without-binius` explicitly selects a two-system comparison.
Binius uses native AES byte-sliced arithmetic and high-to-low evaluation.
Its `switchover = 6` performs seven small-field folds before materializing the
extension-field witness; all 30 sumcheck message rounds remain.

### Output and measurement

Each entry point prints an aligned table to standard output with prover time
(ms), verifier time (ms), serialized proof size (KiB), RAM (MiB), and sample count,
followed by the FLOCK/Mikros prover-time ratio. The same text is saved to the
corresponding root `.log` listed below; no shell redirection is needed.
Metadata and all samples are saved in the adjacent `.json` with the same stem
(for example, `e2e_flock_sha256.log` and `e2e_flock_sha256.json`).
Per-process output and diagnostics are retained under `target/paper-results/`.
While workers run, standard error shows the system, current run, and phase.
Every completed run reports its prover time, verifier time, and serialized
proof size; warmup and RAM-only runs are labelled separately. These lines are
also retained in the per-process diagnostics, with all samples in the JSON.
During a silent phase, an elapsed-time update appears every 15 seconds.
Results are published only after all measurements and checks succeed; previous
output files are copied into the new run directory.

Each hash E2E comparison uses one fresh process per system, with one excluded
warmup followed by **10 measured proofs each for Mikros and FLOCK**. The default
is 10 samples per system in total (`--reps 10 --batches 1`).
Prover time includes online conversion,
commitment, reductions, grinding, and PCS opening; offline preparation is
excluded. Verifier time covers full verification. RAM is the maximum
whole-process GNU time peak across the measurement processes (one by default).

Hadamard runs **10 measured proofs each for Mikros and FLOCK**, after one excluded
warmup each, plus three separate RAM processes per system. **Binius runs once in
total**, without a warmup; that cold proof supplies both its timing and RAM.
The report labels these sampling differences. Hadamard measures the PIOP
reduction and verifier replay, without commitments or PCS. Independent
input-claim checks are outside the timers. RAM is measured immediately after
proving, before verification and source checks: the median of three fresh runs
for Mikros/FLOCK, and the single reading for Binius. Every measured proof is
verified.

Pass experiment options after `--`:

| Option | Default | Purpose |
|---|---|---|
| `--threads N` | `1` | Number of worker threads. |
| `--cpu N` | First allowed CPU | Select a CPU for a one-thread run. |
| `--instances N` | `e2e_*`: `256`; single suite: `1` | E2E instance count; the single suite requires `1`. |
| `--wiring MODE` | `packed` | Mikros single-instance copy argument: `packed` or `public`. |
| `--log-n N` | `30` | Hadamard size `2^N`, with `14 <= N <= 30`. |
| `--reps N` | `10` | Mikros/FLOCK measured proofs per process; Binius always runs once. |
| `--batches N` | `1` | E2E measurement processes per system; total samples = `reps * batches`. |
| `--rss-runs N` | `3` | Separate Mikros/FLOCK Hadamard RAM measurements; Binius reuses its sole proof. |
| `--output PATH.log` | Root log below | Set the report path and matching JSON sidecar. |
| `--validate` | Off | Run additional E2E tamper checks outside measurements. |

For a short check without replacing the paper logs:

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench e2e_sha256 -- \
  --reps 1 --batches 1 --output target/paper-results/sha256-smoke.log
cargo +nightly-2026-03-21 bench --locked -p mikros --bench binary_hadamard -- \
  --log-n 14 --reps 1 --rss-runs 1 --output target/paper-results/hadamard-smoke.log
```

### 5. Single-instance hashes

Run all three hashes with one command:

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench single_hash_compare
```

The suite runs SHA-256, Keccak, and BLAKE3 sequentially. For each hash, Mikros and
FLOCK each run **one excluded warmup followed by 10 measured samples**
(`--reps 10 --batches 1`): 60 measured proofs across the full suite.
Defaults are one instance, one thread, Mikros binary PLONK with its original **packed permutation
argument**, and FLOCK
native R1CS with **skip 7**. Both use FLOCK's native Ligerito Fast PCS.
Witness-dependent permutation preparation and proving are included in online
time. Fixed circuit and permutation preprocessing are offline.

The combined report is printed to stdout and saved to root
`single_hash_compare.log`, with metadata and samples in `single_hash_compare.json`.
It reports prover time, verifier time, proof size, and RAM for both systems,
plus a separate Mikros table with **Gate constraint**, **Permutation argument**,
and **Shared PCS/other** rows. The permutation row reports its prover time,
verifier time, and serialized proof size, including online preparation and
grinding. Shared commitments and openings are counted once in the shared row.
Progress and per-run component measurements appear on stderr. Existing reports
are archived before replacement.

Sampling, CPU selection, validation, and output options work as above.
For a short run with separate tamper checks:

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench single_hash_compare -- \
  --reps 1 --batches 1 --validate \
  --output target/paper-results/single-smoke.log
```

To run one hash, use its individual `e2e_*` entry with `--instances 1`;
it also defaults to the packed permutation argument. Previously recorded reports retain their
own configuration until rerun, including [single_hash_compare.log](single_hash_compare.log)
and [single_hash_compare_1thread.log](single_hash_compare_1thread.log).

## Recorded results

The logs below are the default destinations for one-thread runs pinned to
one CPU. Earlier server hash results used 256 instances and three measurement
batches (18 measured proofs per system); new hash runs use 10 samples per system.
The earlier three-system Hadamard comparison used six measured proofs per system
and three separate cold-process RAM measurements; new runs use the 10/10/1
sampling described above. Each report retains its own configuration. Earlier
logs may use the historical JSON format; rerunning a command publishes the
readable report and JSON sidecar described above. The earlier 64-thread logs,
including the earlier 18-proof Mikros/FLOCK Hadamard comparison, are archived in
[previous64](mikros/server_experiments/20260918-paper-single-thread/previous64/).

| Experiment | Cargo bench target | Default log / recorded results | Threads |
|---|---|---|---|
| SHA-256 | `e2e_sha256` | [e2e_flock_sha256.log](e2e_flock_sha256.log) | 1 |
| Keccak | `e2e_keccak` | [e2e_flock_keccak.log](e2e_flock_keccak.log) | 1 |
| BLAKE3 | `e2e_blake3` | [e2e_flock_blake3.log](e2e_flock_blake3.log) | 1 |
| Binary Hadamard, 2^30 | `binary_hadamard` | [compare_hadamard_2_30.log](compare_hadamard_2_30.log) | 1 |
| Single-instance SHA-256, Keccak, BLAKE3 | `single_hash_compare` | [single_hash_compare.log](single_hash_compare.log) | 1 |

## Acknowledgments and license

This implementation builds on FLOCK. It also includes code derived from
[binius64](https://github.com/binius-zk/binius64) (Copyright 2025 The Binius
Developers and Irreducible, Inc.) and
[bolt-rs](https://github.com/bcc-research/bolt-rs) (Copyright 2026 Bain Capital
Crypto, LP and Ron Rothblum). See the source-file headers for attribution.

Licensed under [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
