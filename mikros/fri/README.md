# Additive FRI packed-word PCS

This crate authenticates ordinary Boolean multilinear evaluations of vectors of
GHASH `F128` words. It reuses Binius64's BaseFold MLE-check and additive FRI,
pinned to `a41ff589ad23ed6e158e997c0ba85ede777fdce8`.

`Source::commit` converts the input to native packed GHASH fields, performs the
rate-1/2 additive Reed–Solomon encoding, and caches its Merkle tree and codeword.
`Source::open` consumes that cache and the already-known claimed evaluation;
it does not recompute the initial commitment or rescan the source to compute
the claimed value. Points are in low-variable-first Boolean MLE order.
`verify` requires only a verifier key, fixed commitment, point, value and proof.
It runs the complete MLE-check and FRI query verification and requires exact
transcript exhaustion.

The transcript binds the protocol domain, pinned parameter digest, vector
length, external Merkle root, point and value. SHA-256 authenticates Merkle
trees and drives the native transcript. The protocol is non-hiding, matching
the existing benchmark scope. The native query calculator selects 241 queries
for rate 1/2 and a 100-bit **query-phase-only** target; this is not a claim of
100-bit soundness for the complete composed protocol.

The native optimized MLE-check prover interpolates using `(1-alpha)^-1`.
For a point containing `alpha=1`, this wrapper instead computes both actual
half evaluations and sends the same degree-one coefficient with the same
FRI schedule. The native verifier is unchanged. Tests compare these two
prover paths byte-for-byte off the boundary and test Boolean points.

```sh
cargo test -p mikros-fri --lib
cargo test -p mikros --example e2e_plonk_fri
```

Upstream repository: <https://github.com/binius-zk/binius64>.
The pinned source includes `LICENSE-MIT.txt` and `LICENSE-Apache-2.0.txt`.
No upstream source is vendored or modified by this adapter.

The default prover enables two byte-preserving optimizations: `parallel-encode`
initializes the two codeword halves in at most 64 KiB parallel copy tasks, and
`fixed-leaf-sha` hashes fixed 56–1024 byte leaves with precomputed SHA-256 padding
on x86-64; other architectures retain native hashing, including the AArch64
four-way SHA kernel.
The original bit reversal, additive NTT, inner-node compression, verifier,
query count and transcript are unchanged. Larger leaves retain native hashing.

Run the complete PCS benchmark (the size counts **F128 words**, not gates):

```sh
RUSTFLAGS="-C target-cpu=native" cargo bench -p mikros-fri --bench fri_pipeline -- \
  --log-words 26 --threads 64 --reps 3
```

It prints one `FRI_SAMPLE` JSON record per fully verified proof, including
commit/open/verify times, opening transcript bytes, commitment root and complete
proof digest. Inputs and evaluation generation are outside these timers; setup
is reported separately. Opening includes consuming and releasing the cache.
The opening byte count excludes the external commitment. To save stdout and
progress messages, append `2>&1 | tee fri_pipeline.log`.

Use `--no-default-features` before `--` for the native baseline. Enable individual
features with `--no-default-features --features parallel-encode` (or
`fixed-leaf-sha`) for ablation. The optional `single-mle` candidate removes two
short accumulator allocations per MLE leaf; it is not enabled by default.
`ADDITIVE_FRI_PROFILE_STAGES=1` emits diagnostic stage timings, including memory
release; keep it unset for formal timings. All variants preserve the same
commitment and proof bytes for a fixed statement.
