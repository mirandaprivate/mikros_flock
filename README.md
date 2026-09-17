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
| `mikros/benches/univariate_vs_flock.rs` | Binary Hadamard comparison. |
| `mikros/binius_switchover_hadamard/` | Original Binius Hadamard comparison with a pinned upstream version. |
| `crates/flock-*/` | FLOCK implementation, Ligerito PCS, and shared field and parallel kernels. |

## Build

Run the following commands from the repository root in a Linux Bash shell.
The experiments use 64 threads and native CPU kernels; the recorded server runs
used an Intel Xeon Gold 6342 with GFNI, AVX-512, and VPCLMULQDQ.
Install Rust through rustup, a C/C++ build toolchain, and GNU time.

```bash
rustup toolchain install nightly-2026-03-21 --profile minimal
export RUSTFLAGS="-C target-cpu=native"
export RAYON_NUM_THREADS=64
export FLOCK_PCS_PACKED_DIRECT=1 FLOCK_WP_UNINIT=1
export CARGO_TARGET_DIR="$PWD/target"
mkdir -p target/paper-results
cargo +nightly-2026-03-21 build --release --locked -p mikros --example e2e_hash_compare
```

## Experiments

The timing commands below use one warmup and six measured proofs.
The three end-to-end experiments each prove 256 independent instances, using
FLOCK's native Ligerito Fast PCS. Mikros packs instances in groups of 128.
Run the experiments sequentially.

### 1. SHA-256 compression

```bash
for system in mikros flock; do
  /usr/bin/time -v -o "target/paper-results/sha256-${system}.rss.txt" \
    target/release/examples/e2e_hash_compare \
    --hash sha256 --system "$system" --instances 256 --skip 7 \
    --threads 64 --reps 6 --warmups 1 \
    > "target/paper-results/sha256-${system}.jsonl"
done
```

### 2. Keccak-f[1600] permutation

```bash
for system in mikros flock; do
  /usr/bin/time -v -o "target/paper-results/keccak-${system}.rss.txt" \
    target/release/examples/e2e_hash_compare \
    --hash keccak --system "$system" --instances 256 --skip 7 \
    --threads 64 --reps 6 --warmups 1 \
    > "target/paper-results/keccak-${system}.jsonl"
done
```

### 3. BLAKE3 compression

```bash
for system in mikros flock; do
  /usr/bin/time -v -o "target/paper-results/blake3-${system}.rss.txt" \
    target/release/examples/e2e_hash_compare \
    --hash blake3 --system "$system" --instances 256 --skip 7 \
    --threads 64 --reps 6 --warmups 1 \
    > "target/paper-results/blake3-${system}.jsonl"
done
```

The JSONL output records `prover_ms`, `verify_ms`, and `proof_bytes` for each
proof. GNU time records whole-process peak RAM in the corresponding `.rss.txt`
file. Prover time includes online conversion, commitment, reduction, and PCS
opening; offline witness preparation is reported separately. Every proof is
verified.

### 4. Binary Hadamard with 2^30 constraints

```bash
cargo +nightly-2026-03-21 bench --locked -p mikros --bench univariate_vs_flock -- \
  --logs 30 --runs 6 --threads 64 --systems pairreuse,flock128 \
  --output target/paper-results/hadamard_2_30.json
```

`pairreuse` selects Mikros's optimized univariate kernel with scratch reuse;
`flock128` selects FLOCK skip = 7. This experiment measures the Hadamard PIOP.
It records prover time, verifier replay time, and proof size, with input-claim
checks timed separately. To measure RAM in a fresh process for each system:

```bash
for system in pairreuse flock128; do
  cargo +nightly-2026-03-21 bench --locked -p mikros --bench univariate_vs_flock -- \
    --logs 30 --runs 1 --threads 64 --systems "$system" --rss \
    --output "target/paper-results/hadamard_2_30-${system}-rss.json"
done
```

Run the original Binius baseline on the same binary inputs:

```bash
python3 mikros/binius_switchover_hadamard/setup.py
cargo +nightly-2026-03-21 run --release --locked \
  --manifest-path mikros/binius_switchover_hadamard/Cargo.toml --features aes32 -- \
  --log-n 30 --switchover 6 --runs 6 --threads 64 \
  --order high --composition native --first-round-zero yes \
  --output target/paper-results/hadamard_2_30-binius.json
```

Binius uses binary witness storage, native AES byte-sliced field arithmetic,
and its equality-weighted sumcheck. The configuration above was selected by
native composition benchmarks at 2^24 and field/order benchmarks at 2^30.
`--switchover s` materializes the extension-field witness after round `s + 1`.
The comparison uses seven early rounds (`s = 6`), reducing the witness to `N/128`.
All 30 sumcheck message rounds remain in the original Binius protocol.
The timer includes equality-weight construction and all proof rounds. Use
`--runs 1 --rss` for a fresh-process RAM measurement.

## Recorded results

The hash results use three measurement batches (18 measured proofs per system).
The three-system Hadamard comparison uses six measured proofs per system and
three separate cold-process RAM measurements on the 64-thread server. The log
also retains the earlier 18-proof Mikros/FLOCK comparison. Each log contains
the configuration, samples, and summary statistics.

| Experiment | Results |
|---|---|
| SHA-256 | [e2e_flock_sha256.log](e2e_flock_sha256.log) |
| Keccak | [e2e_flock_keccak.log](e2e_flock_keccak.log) |
| BLAKE3 | [e2e_flock_blake3.log](e2e_flock_blake3.log) |
| Binary Hadamard, 2^30 | [compare_hadamard_2_30.log](compare_hadamard_2_30.log) |

## Acknowledgments and license

This implementation builds on FLOCK. It also includes code derived from
[binius64](https://github.com/binius-zk/binius64) (Copyright 2025 The Binius
Developers and Irreducible, Inc.) and
[bolt-rs](https://github.com/bcc-research/bolt-rs) (Copyright 2026 Bain Capital
Crypto, LP and Ron Rothblum). See the source-file headers for attribution.

Licensed under [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
