# Local validation before server measurement

Validated in WSL Ubuntu using nightly-2026-03-21 and
`RUSTFLAGS='-C target-cpu=native'`:

- `cargo fmt --package mikros-permutation-binary`: completed.
- `cargo test --release --locked --offline -j 4 -- --test-threads=1`:
  8 tests passed (1 library, 1 comparison driver, 6 differential tests).
- `cargo clippy --release --locked --offline -j 4 --all-targets --no-deps -- -D warnings`:
  passed. The frozen field dependency reports pre-existing unused AVX-512
  imports on the local CPU; its sources were not changed.
- `cargo build --release --locked --offline -j 4 --bin compare`: passed.
- `compare --log-n 20 --width 1 --threads 6 --systems previous,optimized,flock
  --runs 6 --warmups 1 --out results/local-w1-n20-v4.json`: completed;
  all 21 proof records verified, including every terminal source claim.
  New and independently compiled previous Mikros proof digests matched
  for every input. Saved raw proofs accompany the JSON and JSONL.

The local CPU lacks the server's AVX-512/VPCLMUL path; native server tests are
required before accepting server measurements. Local timings are not combined
with server measurements. This is a same-message PIOP optimization, without PCS.

The existing v3 frozen source manifest was checked against the current tree.
All old Rust/build inputs matched. Only the old README and THEORY were updated
after that experiment. The README is included for Rust's documentation
`include_str!`, with its updated hash recorded separately; old protocol Rust
and Cargo inputs are checked against the frozen manifest.
