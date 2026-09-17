# Original Binius Hadamard benchmark

This standalone crate measures the original Binius `EqIndSumcheckProverBuilder::with_switchover` on `eq(r,x) * (A(x)B(x) - C(x))`. The witness uses compact binary inputs and 128-bit challenges. It uses the unmodified upstream implementation at revision `47675e19c86c0fb676f75437073a77af8e337938`.

The switchover parameter is an engineering threshold: `--switchover s` materializes the folded extension-field witness after challenge fold `s+1`. It does not skip protocol rounds. This entry does not use univariate skipping or a precomputed `3^t` evaluation grid.

```sh
python setup.py
RUSTFLAGS="-C target-cpu=native" cargo +nightly-2026-03-21 build --release
RUSTFLAGS="-C target-cpu=native" cargo +nightly-2026-03-21 test --release
./target/release/binius-switchover-hadamard --log-n 30 --switchover 6 --runs 6 --threads 64 --output timing.json
./target/release/binius-switchover-hadamard --log-n 30 --switchover 6 --runs 1 --threads 64 --rss --output rss.json
./target/release/binius-switchover-hadamard --log-n 16 --switchover 6 --runs 2 --threads 64 --tamper --output smoke.json
```

The timed prover includes challenge generation, the native constructor, equality-table expansion, all folding and sumcheck rounds. Input generation and its canonical digest are separate. The native verifier checks the equality terminal claim; all three input claims are additionally checked against an independent direct source oracle outside the prover and verifier timings. These are PIOP measurements without a PCS.

Normal timing runs perform seed 0 as a warmup, followed by the requested formal runs. RSS uses one cold proof in a fresh process. The output records proof bytes, transcript agreement, source checks, timing scope, compiled CPU features and the upstream revision. The common fixture matches the Mikros/FLOCK SplitMix64 generator byte for byte.

The original native equality expansion uses `2^(log_n-1)` extension-field elements, regardless of the switchover threshold. At `log_n=30` that one table is 8 GiB; folding and witness materialization require additional memory. Use a server with sufficient RAM for this native implementation.

## Native engineering configurations

| Cargo feature | Field packing on the recorded AVX-512 server | Supported `--order` |
|---|---|---|
| Default | Canonical tower, four lanes | `low`, `high` |
| `polyval` | Polyval, four lanes with native carryless multiplication | `low`, `high` |
| `aes32` | Official benchmark's AES byte-sliced 32 lanes | `high` |
| `aes-bytesliced` | Native optimal AES byte-sliced 64 lanes | `high` |

Select one feature per build. Byte-sliced input transposition and allocation run
inside the prover timer. The pinned upstream F2 byte iterator overstates the
byte-sliced storage length; the harness rejects its unsafe LowToHigh path.
HighToLow uses the upstream generic folding path.

`--composition native|scalar` selects the original `ArithCircuitPoly` batch
implementation or the earlier packed-value callback. Both use SIMD field
values; `scalar` names the callback interface. `--first-round-zero yes|no`
controls the native builder's known-zero advice. The defaults are `native` and
`yes`. Tests require identical proof bytes across these options under a fixed
field and order. `--profile` records separate per-round execute/fold timings.

The recorded experiment screens the six supported field/order configurations
at N24 and N30, then measures the winner in separate timing, profiling and
cold-memory processes. Results, source hashes and the preceding canonical-tower
measurements are retained in [the root result log](../../compare_hadamard_2_30.log).
The paper command in [the root README](../../README.md) uses `--features aes32`
with `--order high --composition native --first-round-zero yes`.
