# Binary permutation v5: register folding and bounded tree reuse

This independent experimental crate preserves binary v4 as the separately
compiled `previous` dependency. All candidates retain its K32 fused construction,
proof messages, challenge order and terminal claims. The default
`permutation::prove` retains v4 behavior; candidates are explicit engines in
`prove_profiled` until their tradeoffs have been measured.

- `Registers` / `register`: compute the next GKR message directly from folded
  values in AVX-512 registers; retain the output for the subsequent challenge.
  The scalar fallback preserves the existing fold-and-scan implementation.
- `Recycled` / `reuse`: reuse initialized tree-layer allocations between proofs.
  A dedicated pool retains buffers of at least 1 MiB, at most 32 buffers and
  at most 32 GiB of capacity. Every element is overwritten before use. Small
  bit-packed layers retain fresh zero-filled allocation.
- `Combined` / `combined`: both changes, with a separate tree pool so the two
  reuse candidates cannot warm each other's tree buffers.

Old/new Mikros and Flock already share the existing Flock GKR scratch pool.
The new tree pools add explicit retained memory and affect cold versus repeated
proving differently. `tree_pool::stats(slot)` exposes both pools; slot 0 belongs
 to Recycled, slot 1 to Combined. `tree_pool::clear_all()` releases both pools
synchronously. The benchmark reports this cleanup and shared scratch cleanup
outside the per-proof timer. These costs must be included when judging a
one-shot invocation; warm gains do not establish cold gains.

The relation is `f[sigma[i]] = g[i]`. N counts permutation items. At N=2^30,
one-bit values plus 30 position bits fit injectively in K32 inside F128.
Flock uses the frozen batched Product-GKR entry point, which has no skip-round
parameter. These are PIOP measurements: every proof is replayed and every
source claim checked against original inputs, but PCS, commitments, openings
and grinding are excluded. This does not establish full-system security parity.

```sh
export RUSTFLAGS='-C target-cpu=native'
cargo +nightly-2026-03-21 test --release --locked -- --test-threads=1
cargo +nightly-2026-03-21 build --release --locked --bin compare
target/release/compare --log-n 20 --width 1 --threads 4 \
  --systems previous,register,reuse,combined,flock --runs 1 --warmups 1 \
  --out results/NEW.json
```

See PLAN.md for the predeclared 64-thread ablation and independent N30 check.

