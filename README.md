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
| `mikros/benches/single_instance.rs`, `single_hash_compare.rs` | Single-instance AND/XOR comparison, with the original entry name retained as an alias. |
| `mikros/benches/e2e_single_constraints.rs` | Single-instance comparison on the connected AND/XOR circuit. |
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
256 instances for the three `e2e_<hash>` entries, one instance with `2^25`
AND/XOR constraints for `single_instance`, and `N = 2^30` for Hadamard. The E2E comparisons use
FLOCK's native Ligerito Fast PCS. In the multiple-instance experiments, Mikros
packs instances in groups of 128.
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
JSON sidecars and process diagnostics are local outputs excluded from Git;
the repository retains only the five result logs listed below. Published logs
use repository-relative paths. Source checksum manifests remain versioned.
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

Pass experiment options after `--`. The options below apply to the four entries
above; the single-circuit entry's options are described in Section 5.

| Option | Default | Purpose |
|---|---|---|
| `--threads N` | `1` | Number of worker threads. |
| `--cpu N` | First allowed CPU | Select a CPU for a one-thread run. |
| `--instances N` | `e2e_<hash>`: `256` | Hash E2E instance count (`1` or `256`). |
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

### 5. Single instance with 2^25 binary constraints

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench single_instance
```

The existing `single_hash_compare` and `e2e_single_constraints` bench names are
aliases for this experiment. The default is one connected AND/XOR circuit,
one CPU, and **one measured proof per system, without a warmup**.
Mikros uses binary PLONK with its packed permutation argument. FLOCK uses
native R1CS with **skip 7**, public CSC matrices, and a direct PCS claim for the
public output. Internal wire consistency is expressed by those public matrices.
Both use FLOCK's native Ligerito Fast PCS; their witness layouts differ.
Here `N = 2^25` counts genuine AND/XOR gates. FLOCK's R1CS has `4N` rows:
the `N` gate equations, `N + 1` input/constant identity rows, and padding.
Mikros has `N` gate equations and a `4N`-position copy domain. The report lists
these sizes separately.

When updating a server checkout, sync both `mikros/` and
`crates/flock-core/src/lincheck.rs`. The latter includes the single-instance
(`n_outer = 1`) input-layout and folding support. The older byte-stripe path
requires at least eight outer instances and will reject this experiment,
regardless of the gate count.

The report is printed to stdout and saved to `single_2_25.log`, with
raw measurements in the ignored `single_2_25.json`. Per-process
diagnostics are under `target/paper-results/e2e-single-2-25/`. The report gives
prover time, verifier time, proof size, RAM, and individual prover phases.
Its Mikros **Permutation argument** row includes online copy preparation,
permutation proving, and the entire public PCS opening and verification.
That public opening authenticates both sigma and selector data and is counted
once in this row. **Shared PCS/other** includes the combined packed-opening
reduction, witness commitment/opening, and remaining online work. Per-sample
component times and proof sizes sum to the E2E totals.

Fixed circuit/public commitment preparation is offline; witness-dependent
copy preparation is online. The verifier checks the complete proof and PCS.
RAM is the GNU time whole-process peak, including setup and verification.
Progress reports show each run and its measurements. Previous results are
archived before replacement. Use `--cpu`, `--log-n`, `--reps`, `--warmups`,
and `--output` to change this experiment's settings. For a small check:

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench single_instance -- \
  --log-n 14 --reps 1 --warmups 0 \
  --output target/paper-results/single-smoke.log
```

Single-instance hash experiments remain available through each individual
`e2e_<hash>` entry with `--instances 1 --output target/paper-results/<hash>-single.log`.
Those retain their hash-specific native FLOCK circuit pipeline.

## Recorded results

The logs below are the default destinations for one-thread runs pinned to
one CPU. Earlier server hash results used 256 instances and three measurement
batches (18 measured proofs per system); new hash runs use 10 samples per system.
The earlier three-system Hadamard comparison used six measured proofs per system
and three separate cold-process RAM measurements; new runs use the 10/10/1
sampling described above. Each report retains its own configuration. Earlier
logs may use the historical JSON format; rerunning a command publishes the
readable report and local JSON sidecar described above.

| Experiment | Cargo bench target | Default log / recorded results | Threads |
|---|---|---|---|
| SHA-256 | `e2e_sha256` | [e2e_flock_sha256.log](e2e_flock_sha256.log) | 1 |
| Keccak | `e2e_keccak` | [e2e_flock_keccak.log](e2e_flock_keccak.log) | 1 |
| BLAKE3 | `e2e_blake3` | [e2e_flock_blake3.log](e2e_flock_blake3.log) | 1 |
| Binary Hadamard, 2^30 | `binary_hadamard` | [compare_hadamard_2_30.log](compare_hadamard_2_30.log) | 1 |
| Single-instance AND/XOR, 2^25 | `single_instance` (`single_hash_compare` alias) | [single_2_25.log](single_2_25.log) | 1 |

## Acknowledgments and license

This implementation builds on FLOCK. It also includes code derived from
[binius64](https://github.com/binius-zk/binius64) (Copyright 2025 The Binius
Developers and Irreducible, Inc.) and
[bolt-rs](https://github.com/bcc-research/bolt-rs) (Copyright 2026 Bain Capital
Crypto, LP and Ron Rothblum). See the source-file headers for attribution.

Licensed under [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
