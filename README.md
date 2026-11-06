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
| `mikros/qafold/` | F65521 PCS with 8-to-1 packing and QAFold over F_(65521^8). |
| `mikros/fri/` | Default binary PLONK PCS: packed F128 MLE openings with additive FRI. |
| `mikros/raafold/` | Binary RAAFold PCS and GHASH/POLYVAL field adapter. |
| `mikros/whir/` | Optional Plonky3 binary WHIR PCS with GHASH/tower conversion. |
| `mikros/benches/plonk_fri.rs`, `plonk_raafold.rs`, `plonk_whir.rs` | Standalone PCS experiments using the shared binary PLONK driver. |
| `mikros/permutation/src/indexed_lookup.rs` | Indexed lookup with public addresses and multiplicative counters. |
| `mikros/benches/lookup_bitwidth.rs` | Lookup comparison across value bitwidths and witness sizes. |
| `mikros/examples/e2e_hash_compare.rs` | End-to-end SHA-256, Keccak, and BLAKE3 comparisons. |
| `mikros/examples/e2e_common/` | Shared circuit, witness, reduction, and PCS integration code. |
| `mikros/benches/e2e_*.rs`, `binary_hadamard.rs` | Multiple-instance hash and Hadamard `cargo bench` entry points. |
| `mikros/benches/multi_hash_fri_compare.rs` | Table 4: batches of 256 hashes with Mikros FRI versus native FLOCK. |
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
The hash and Hadamard experiments default to one thread pinned to one CPU and
native CPU kernels. The recorded server runs used an Intel Xeon Gold 6342 with GFNI,
AVX-512, and VPCLMULQDQ; the updated Table 4 runs use CPU 0.
Install Rust through rustup, a C/C++ build toolchain, Python 3.10 or newer, and GNU time.
Native CPU code generation is enabled in `.cargo/config.toml`.

```bash
rustup toolchain install nightly-2026-03-21 --profile minimal
export RUSTFLAGS="-C target-cpu=native"
```

## Experiments

Each experiment uses `cargo bench` to build and run its complete comparison.
For the hash, Hadamard and single-instance entries, defaults are one thread,
256 instances for the three `e2e_<hash>` entries, one instance with `2^25`
AND/XOR constraints for `single_instance`, and `N = 2^30` for Hadamard.
The `circuit_sat` and `single_instance` comparisons use additive FRI for Mikros and native Ligerito Fast
for FLOCK. For Table 4, pass `--pcs fri` to the three `e2e_<hash>` entries as
shown below; omitting it retains the earlier native-PCS comparison.
The `single_hash_fri_compare` entry compares one call of each hash with Mikros's
current additive-FRI backend against native FLOCK skip-7.
The `multi_hash_fri_compare` entry uses Mikros FRI for the 256-instance comparison
and refreshes the three root hash result logs for Table 4.
In the multiple-instance experiments, Mikros
packs instances in groups of 128.
Run the experiments sequentially. On Linux, a one-thread run selects the first
allowed CPU automatically. Append `-- --cpu 16` to select a specific CPU, and
keep its SMT sibling idle.

### 1. SHA-256 compression

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench e2e_sha256 -- --pcs fri
```

### 2. Keccak-f[1600] permutation

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench e2e_keccak -- --pcs fri
```

### 3. BLAKE3 compression

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench e2e_blake3 -- --pcs fri
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

### Single-instance hashes with FRI

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench single_hash_fri_compare -- \
  --threads 64 --reps 10 --warmups 1 --validate \
  --output target/paper-results/single-hash-fri-vs-flock.log
```

This runs one SHA-256 compression, one Keccak-f[1600] permutation, and one BLAKE3
compression per proof. Mikros uses binary PLONK, its packed permutation argument,
and two packed F128 FRI openings; FLOCK uses native public-matrix R1CS with skip-7
and Ligerito Fast. The default is 64 prover workers, one verifier worker, and ten
measured proofs after one warmup for each hash/system. Both arms include online
witness commitment, reductions, grinding, and all PCS openings; fixed public
preprocessing and outer proof serialization are excluded. Logs report the
median times, serialized proof bytes, whole-process peak RSS, and actual circuit
dimensions. The shared PLONK fixture retains its minimum 2^15 packed-word domain;
FLOCK likewise retains its native PCS capacity padding. Raw samples and optional
tamper checks are saved alongside the readable log. Existing root logs are preserved.

### Table 4: batches of 256 hashes with FRI

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench multi_hash_fri_compare -- \
  --threads 1 --reps 10 --warmups 1 --validate
```

This runs 256 independent SHA-256 compressions, Keccak-f[1600] permutations, or
BLAKE3 compressions per proof. Mikros retains its two groups of 128 packed
instances and public R1CS matrix checks, then authenticates the merged witness
opening with additive FRI over F2^128. FLOCK uses native Ligerito Fast and skip-7.
Both include online commitment, reductions, grinding, and complete PCS opening
and verification. The default is one prover thread, one verifier thread, and ten
measurements after one warmup per system/hash. FRI parameters and their query-phase
security scope are recorded in the raw metadata.

After all three comparisons pass, this entry updates `e2e_flock_sha256.log`,
`e2e_flock_keccak.log`, and `e2e_flock_blake3.log`. Previous logs and raw measurements
are retained under `target/paper-results/`; the combined report is
`target/paper-results/multiple-hash-fri-vs-flock.log`. To keep a diagnostic run
separate, append `--hash-output-dir target/paper-results/multiple-fri-smoke`.
The original `e2e_<hash>` commands remain available for native-PCS comparisons;
use a separate `--output` path to preserve the current Table 4 logs.

### Output and measurement

The hash, Hadamard and single-instance entries print an aligned table with prover time
(ms), verifier time (ms), serialized proof size (KiB), RAM (MiB), and sample count,
followed by the FLOCK/Mikros prover-time ratio. Hash and Hadamard reports are
saved to the corresponding root `.log` listed below. New single-instance FRI
reports are saved under `target/paper-results/`; no shell redirection is needed.
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
| `--pcs native\|fri` | `native` | Mikros hash PCS; use `fri` for the updated Table 4. FLOCK retains native Ligerito Fast. |
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
Mikros uses binary PLONK with its packed permutation argument and F128 additive FRI. FLOCK uses
native R1CS with **skip 7**, public CSC matrices, and a direct PCS claim for the
public output. Internal wire consistency is expressed by those public matrices.
FLOCK retains its native Ligerito Fast PCS. Both prove the same logical circuit
and public output, with different witness layouts, arithmetizations and PCS.
Here `N = 2^25` counts genuine AND/XOR gates. FLOCK's R1CS has `4N` rows:
the `N` gate equations, `N + 1` input/constant identity rows, and padding.
Mikros has `N` gate equations and a `4N`-position copy domain. The report lists
these sizes separately.

When updating a server checkout, sync both `mikros/` and
`crates/flock-core/src/lincheck.rs`. The latter includes the single-instance
(`n_outer = 1`) input-layout and folding support. The older byte-stripe path
requires at least eight outer instances and will reject this experiment,
regardless of the gate count.

The report is printed to stdout and saved to
`target/paper-results/single-fri-vs-flock-2-25.log`, with raw JSON and
per-process diagnostics alongside it. The historical native-PCS result remains
in `single_2_25.log`. The new report gives
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
and `--output` to change this experiment's settings. `--system mikros` or
`--system flock` runs one arm; the default runs both. For a small check:

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench single_instance -- \
  --log-n 14 --reps 1 --warmups 0 \
  --output target/paper-results/single-smoke.log
```

Single-instance hash experiments remain available through each individual
`e2e_<hash>` entry with `--instances 1 --output target/paper-results/<hash>-single.log`.
Those retain their hash-specific native FLOCK circuit pipeline.

For Table 5, use 256 independent private inputs and `2^30 - 257` actual gates:

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench single_instance -- \
  --circuit near-full --log-n 30 --threads 1 --cpu 0 --reps 1 --warmups 0 \
  --seed 97 --output target/paper-results/single_near_full_2_30.log
```

The shared circuit alternates AND and XOR gates, each consuming the previous
output and one of the 256 inputs. A public permutation changes the input order
in each block of 256 gates. FLOCK has exactly `2^30` R1CS rows: one constant,
256 input identity rows, and `2^30 - 257` gate rows. Mikros uses a `2^30`-row
PLONK domain with 257 padding rows and a `2^32`-position copy domain. Both
prove the same circuit and public output. The default `--circuit paired`
retains the earlier input layout. For a small check of the new layout, replace
`--log-n 30` with `--log-n 14` and choose a separate output path.

### 6. Bitwidth-aware indexed lookup

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench lookup_bitwidth -- \
  --log-n 12,16,20,24 --bits 1,2,4,8,16 --table-log 8 \
  --threads 64 --reps 6 --warmups 1 \
  --out target/paper-results/lookup-bitwidth
```

This compares the packed permutation reduction with a full-F128 Lasso-style
baseline on the same public access schedule. Public addresses, multiplicative
counters and field tables are prepared offline. Witness-dependent tuple encoding,
polynomial coefficients and proof generation remain in the prover timer.
The experiment measures PIOP reductions without PCS; every proof is replayed and
its source claims are checked against the original query and table values.

The report gives the value bitwidth, position-field width, encoded tuple width,
effective packing factor, prover/verifier time and serialized proof size.
Verifier time includes evaluation of the public arrays. Both methods share the
same F128 challenge field. Address and counter capacity can keep the encoded
width unchanged across several value bitwidths. The current packed parameters
support base-field widths 1, 2, 4, 8 and 16; unsupported field towers are rejected.

The entry prints each sample and a comparison table, and writes `lookup_bitwidth.log`, raw
samples and summaries under `--out`. Use a new output directory for each run.
These local results are excluded from Git and do not replace the five logs below.

### 7. Binary Circuit SAT with 2^30 gates

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench circuit_sat
```

This runs one connected binary AND/XOR circuit with exactly `2^30` gates,
using 64 workers and one measured proof per system without a warmup.
Mikros uses the single-instance PLONK construction, packed permutation and
packed F128 openings with additive FRI. FLOCK uses skip 7 with public CSC
matrices, `C = I`, and its native Ligerito Fast PCS. Both verify complete proofs. FLOCK has
`4N` padded R1CS rows, and Mikros has a `4N`-position copy domain.

The entry prints each stage and the prover time, verifier time, proof size,
peak RSS and component timings. Results are saved to
`target/paper-results/circuit-sat-fri-vs-flock-2-30.log`, with raw measurements alongside it.
Fixed circuit preprocessing is offline; witness-dependent preparation and
all PCS openings are online. FLOCK verifier time includes its public-matrix
evaluation, which is linear in the matrix size. This compares two complete
systems on the same circuit; it is not a comparison of PCS backends alone.
The implementation constructs compact CSC arrays directly to
avoid a temporary row-vector matrix at this scale. The full run requires a
large-memory machine; the bounded Mikros copy path alone uses hundreds of GiB.

Use `--log-n`, `--threads`, `--reps`, `--warmups`, `--system` and `--output` after `--`
to change the run. For a small complete-proof check:

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench circuit_sat -- \
  --log-n 14 --threads 4 --output target/paper-results/circuit-sat-smoke.log
```

### 8. Binary Circuit SAT with shared public matrices

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench circuit_sat_public_matrix
```

This compares Mikros and FLOCK on the same single-instance binary AND/XOR
circuit, native R1CS witness, public CSC matrices and `C = I`. Mikros uses
packing factor 128 and FLOCK uses skip 7. Both commit one witness vector and
authenticate three witness claims through the same native Ligerito Fast PCS.
The matrices are public and do not require separate PCS commitments.

The default is `2^30` genuine gates (`4N` padded R1CS rows), 64 workers,
one measured proof and no warmup. The report includes prover time, verifier
time, proof size, whole-process peak RSS, and prover/verifier phases. Public
matrix evaluation is included in both verifier times. Fixed matrix setup and
witness generation are offline; witness-dependent preparation and PCS work
are online. The comparable linear-reduction total includes challenge-dependent
matrix preparation and input conversion; its statistics are computed from
the per-sample sums. Results are printed and saved to
`target/paper-results/circuit-sat-public-matrix-2-30.log`, with raw measurements
alongside it. The existing PLONK comparison and root result logs are separate.

Use `--log-n`, `--threads`, `--reps`, `--warmups` and `--output` after `--`
to change the run. A small complete-proof check is:

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench circuit_sat_public_matrix -- \
  --log-n 14 --threads 4 --output target/paper-results/public-matrix-smoke.log
```

### 9. PLONK witness-field bitwidth

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench plonk_bitwidth
```

This runs complete single-instance Mikros PLONK proofs over `GF(2^b)` for
`b = 1, 2, 4, 8, 16, 32, 64, 128`. The bitwidth is the field degree. The default
circuit has `2^26` gates, with 64 workers, one measured proof per field and no warmup.
Each field runs in a separate process, using FLOCK's native Ligerito Fast PCS.
This entry measures Mikros across fields; it has no FLOCK prover arm.
The wiring and gate count are shared, while the field arithmetic changes with `b`.

The report lists witness packing `kappa = 128/b`, copy-tag width, reduction field and packing,
prover/verifier time, proof size, whole-process peak RSS and per-phase timings.
The full gate, permutation, packed-opening and PCS proofs are verified.
Timing starts from the canonical packed-bit witness input. Generating the
benchmark witness and preparing this input layout are excluded; cloning that
input, commitment, challenge-dependent packing, copy-tag construction and
PCS input conversion are included. Fixed public commitments are offline;
public PCS openings are online. Packed opening reduces all witness/public
claims to two native PCS terminal claims for every field.

The binary gate kernel is used at `b=1`; wider fields use the field reduction
path, with no gate compression (`kappa=1`) at `b=128`.
Copy tags use `D = next_power_of_two(log_n + 2 + b)`: tags below 128 bits use
packed permutation with `copy_kappa=128/D`. For 64-bit tags, evaluated layers
are released early and recomputed later, with recomputation counted online.
Tags of 128 or 256 bits use ordinary ProductGKR with `copy_kappa=1`, over
`GF(2^128)` or `GF(2^256)` respectively. The report identifies these paths as
`packed`, `unpacked_f128` and `unpacked_f256`.

PIOP grinding is reported separately, including the initial permutation
fingerprint grinding. `Online minus PIOP grinding` subtracts these measured
counters within each sample; it is a diagnostic, not the E2E prover time,
and still includes PCS grinding. The default one-sample result can vary with
the amount of grinding work.

Results are printed and saved under `target/paper-results/plonk-bitwidth-wide-2-26/`
as `plonk_bitwidth.log` and `plonk_bitwidth.json`, with individual process
records under `runs/`. This directory is separate from the earlier five-field
results. Use `--log-n`, `--bits`, `--threads`, `--reps`,
`--warmups` and `--out` after `--` to change the run. For a small check:

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench plonk_bitwidth -- \
  --log-n 14 --bits 1,2,4,8,16,32,64,128 --threads 4 \
  --out target/paper-results/plonk-bitwidth-wide-smoke
```

### 10. PLONK bitwidth and circuit-size sweep

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench plonk_bitwidth_sweep -- \
  --log-ns 20,22,24,26,28,30 --bits 1,2,4,8,16,32,64,128 \
  --threads 64 --reps 1 --warmups 0 \
  --out target/paper-results/plonk-bitwidth-sweep
```

This runs the complete Mikros PLONK experiment from Section 9 over the 48
combinations of circuit size and witness field. Each point uses one measured
proof in a separate process, with full gate, copy, packed-opening and native
Ligerito PCS verification. The entry records prover time, verifier time,
serialized proof size and whole-process peak RAM. The online/offline timing
boundary is the same as in Section 9.

The output directory contains `checkpoint.json`, `results.csv`, `summary.log`
and per-case records. Append `--resume` to continue the same sweep. Successful
measurements and cases already stopped by the live memory guard are validated
and reused; guarded cases are not automatically retried. Memory
estimates and available memory determine whether a case can run; use
`--memory-budget-gib` and `--reserve-gib` to set the memory limit and reserve.
Skipped or incomplete cases retain an explicit status and reason: memory
budget, unsupported PCS parameters, a resource-guard stop, worker error,
pending or running. They have no measured metric values.

Generate a four-panel overview and four individual metric plots:

```bash
python3 -m pip install matplotlib
python3 mikros/benches/plot_plonk_bitwidth_sweep.py \
  target/paper-results/plonk-bitwidth-sweep/checkpoint.json \
  --out target/paper-results/plonk-bitwidth-sweep/figures
```

Each plot has one line per field bitwidth and a logarithmic gate-count axis.
Prover seconds and RAM GiB use logarithmic axes; proof KiB and verifier
milliseconds use linear axes. Use `--prover-scale`, `--ram-scale`,
`--proof-scale` or `--verifier-scale` with `linear` or `log` to change them.
Unavailable points appear as gaps; values are never replaced by zero or
interpolated. Exact missing-case reasons remain in the checkpoint.

The script exports `plonk_bitwidth_sweep` and `plonk_bitwidth_{prover,ram,proof,verifier}`
in PNG, PDF and SVG. `figures_manifest.json` records the source JSON SHA-256,
sampling configuration, missing statuses and checksums of the figures.
Figure metadata contains the experiment scope and source checksum, without
machine names, user paths or timestamps. All sweep outputs stay under the
ignored `target/` directory.

## Optional binary PLONK with RAAFold

This entry proves one connected circuit with alternating AND and XOR gates,
including the copy/permutation argument and both packed PCS openings.
It uses the two-accumulator RAA code with repetition four over GF(2^128).

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench plonk_raafold -- \
  --log-n 30 --threads 64 --reps 1 --warmups 0
```

Use `--log-n 26` for a smaller run. The console table and `plonk_raafold.log`
are written under `target/paper-results/plonk-raafold-2-<log-n>/`, together with
raw measurements. Online time includes witness field conversion, commitment,
gate and copy proofs, packed-opening reduction, both RAAFold Opening/Decide
proofs, and serialization. Fixed public commitments and code setup are offline.
The reported query budget assumes relative distance 0.19; this experimental
profile does not certify finite-length code-sampling failure probabilities.

## Default binary PLONK with additive FRI

The `circuit_sat` and `single_instance` comparisons use this PCS for Mikros.
This standalone entry uses the same circuit, gate/copy proofs and packed-opening reduction
as the RAAFold entry. It authenticates the resulting 128-bit packed-word MLEs
with Binius64's BaseFold MLE-check and additive FRI over GF(2^128).

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench plonk_fri -- \
  --log-n 26 --threads 64 --reps 1 --warmups 0
```

The console table, `plonk_fri.log`, raw JSON and process records are saved under
`target/paper-results/plonk-fri-2-<log-n>/`. Use `--log-n 30` for the larger
circuit. Online time includes witness commitment, gate/copy proofs, packed
opening, both complete PCS openings and serialization. Fixed public commitment
and preprocessing are offline. The non-hiding FRI configuration uses rate 1/2
and 241 queries: the 100-bit query-phase target excludes folding and other
protocol errors. See [the PCS module](mikros/fri/README.md) for its interface.

## Optional binary PLONK with WHIR

This entry replaces the two packed-word PCS openings with Plonky3 0.8.0 binary
WHIR. It uses the same circuit, permutation proof and packed-opening reduction
as `plonk_fri`. Witness field conversion and both PCS openings are online;
the fixed public commitment is offline.

```bash
RUSTFLAGS="-C target-cpu=native" cargo +nightly-2026-03-21 bench --locked -p mikros --bench plonk_whir -- \
  --log-n 26 --threads 64 --reps 1 --warmups 0
```

Results are written to `target/paper-results/plonk-whir-2-<log-n>/plonk_whir.log`,
with raw samples and configuration alongside. Use the same size, worker count,
seed and sampling options with `plonk_fri` to compare the PCS choices.
The [WHIR module](mikros/whir/README.md) describes field conversion, security
parameters and standalone PCS tests. FRI remains the default for `circuit_sat`.

## Witness bitwidth: single and 256-instance proofs

This experiment compares one PLONK proof, including its copy/permutation
argument, with a Packed R1CS proof for 256 independent instances using public
matrices and C = I. Both use additive FRI over GF(2^128). Each instance has
2^20 gates by default; the batch therefore contains 2^28 gates. The plots
report total proof costs, without dividing the batch measurements by 256.

```bash
RUSTFLAGS="-C target-cpu=native" cargo +nightly-2026-03-21 bench --locked -p mikros --bench bitwidth_instances -- \
  --log-n 20 --bits 1,2,4,8,16,32,64,128 --threads 1 --reps 1 --warmups 0 \
  --out target/paper-results/bitwidth-instances-2-20
```

The defaults use one measured proof and no warmup for each case in a fresh
process. For witness field GF(2^b), the packing width is kappa = 128/b;
the b = 128 endpoint has kappa = 1 and no subfield packing. Four panels show
prover time, verifier time, proof size and RAM. RAM is GNU `time`'s whole-process
peak RSS, including setup and verification, rather than a prover-only reading.
The default resource guard permits up to 300 GiB per experiment process and
reserves 32 GiB of available host memory. Unavailable cases remain blank.

The output directory contains `checkpoint.json`, `results.csv`, `summary.log`
and PDF/PNG/SVG plots. Use `--resume` to reuse verified successful cases and
retry incomplete cases. Plotting requires Matplotlib; use `--no-plots` to
collect measurements without it and render later:

```bash
python3 -m pip install matplotlib
python3 mikros/benches/plot_bitwidth_instances.py \
  --input target/paper-results/bitwidth-instances-2-20/checkpoint.json \
  --out target/paper-results/bitwidth-instances-2-20/plots
```

## Recorded results

The logs below retain the paper results from one-thread runs pinned to
one CPU. The single-instance log uses the previous native PCS in both arms;
new FRI comparisons are written under `target/paper-results/`.
Earlier server hash results used 256 instances and three measurement
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
| Single-instance AND/XOR, 2^25 (historical native PCS) | Previous `single_instance` | [single_2_25.log](single_2_25.log) | 1 |

## Acknowledgments and license

This implementation builds on FLOCK. It also includes code derived from
[binius64](https://github.com/binius-zk/binius64) (Copyright 2025 The Binius
Developers and Irreducible, Inc.) and
[bolt-rs](https://github.com/bcc-research/bolt-rs) (Copyright 2026 Bain Capital
Crypto, LP and Ron Rothblum). See the source-file headers for attribution.

Licensed under [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
