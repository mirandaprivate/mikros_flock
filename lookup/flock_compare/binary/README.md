# Binary permutation: K32 fused construction

This independent candidate compares 1-bit values under the same public relation
`f[sigma[i]] = g[i]` against the frozen Flock batched Product-GKR. It uses the
unchanged fused v3 crate as `previous`; the original crates and prior experiments
remain available for reproduction.

At N = 2^30, an input consists of one value bit and 30 position bits. The
injective encoding therefore occupies 31 bits in K = GF(2^32), inside the common
E = GF(2^128). This is different from the earlier 8-bit-value experiment, which
used K64. The Flock Product-GKR entry point has no sumcheck skip parameter.

The candidate retains compact K32 leaves and the 64-bit quadratic coefficient
layer. It fuses decoding, their construction, and evaluation, then multiplies
evaluated quadratics to produce the first extension layer directly. This avoids
materializing the 128-bit quartic coefficient layer. Arithmetic, proof messages,
challenge order, source claims and verifier are unchanged. K64 retains the v3
path; other widths/capacities retain their existing fallback.

`compare --systems previous,optimized,flock --width 1` measures v3, this
candidate and Flock. The server driver uses 64 threads on 48 physical cores plus
16 SMT siblings, with native ISA flags and identical CPU affinity for all three.
Six measured inputs cover all six system orders; one warmup is recorded. Every
proof is replayed and every terminal source claim is authenticated. Preparation,
proving, verifier replay and direct source authentication are reported separately.
Independent one-system processes measure peak RSS. All results are permutation
PIOP measurements and exclude PCS, commitments, openings and grinding.

```sh
export RUSTFLAGS='-C target-cpu=native'
cargo +nightly-2026-03-21 test --release --locked -- --test-threads=1
cargo +nightly-2026-03-21 build --release --locked --bin compare
target/release/compare --log-n 20 --width 1 --threads 6 \
  --systems previous,optimized,flock --runs 6 --warmups 1 --out results/NEW.json
```

The candidate is experimental until its differential tests and independently
measured server results have been checked. See PLAN.md for the measurement plan.
