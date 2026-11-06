# Binary WHIR packed-word PCS

This adapter authenticates Boolean multilinear evaluations of complete 128-bit
GHASH words using Plonky3's additive-domain WHIR. Dependencies are pinned to
Plonky3 0.8.0 (upstream revision
`a21e3ed42905040ad49c519e402f018381c12d0c`); no upstream source is modified.

The public interface uses FLOCK's `F128`. Plonky3's official polynomial-basis
isomorphism converts these words and challenges to its `BinaryField128` tower
field. The conversion preserves addition, multiplication and every packed bit;
raw representation casts would not. Commitment timing includes the conversion.
Batch conversion uses the upstream GFNI/AVX-512 path when compiled for a CPU
with these features. `PolyBasisNtt` supplies the additive FFT.

`Source::commit` retains the codeword and Merkle cache. `Source::open` consumes
that cache and opens the prescribed MLE point, with low-variable-first GHASH
coordinates converted to Plonky3's high-variable-first convention. The verifier
needs the key, commitment, point, value and proof, not a source table. The
adapter binds the statement and parameters to the transcript and checks the
returned evaluation against the existing PLONK terminal claim.

The configuration uses SHA-256, unique decoding, starting code rate 1/2,
folding factor four and a 100-bit per-term security target. The maximum PoW
budget is three bits; actual grinding and the upstream composed PCS error bound
are reported for each vector length. The target is not a claim of 100-bit
soundness for the full PLONK proof. This experimental PCS is non-hiding.

```sh
cargo +nightly-2026-03-21 test --release --locked -p mikros-whir
cargo +nightly-2026-03-21 test --release --locked -p mikros --example e2e_plonk_whir

RUSTFLAGS="-C target-cpu=native" cargo +nightly-2026-03-21 bench --locked -p mikros --bench plonk_whir -- \
  --log-n 26 --threads 64 --reps 1 --warmups 0
```

The PLONK entry shares gate/copy proofs, packing and accounting with `plonk_fri`.
Witness commitment, conversions, both PCS openings, serialization and buffer
release are online. Fixed public commitment and witness generation are offline.
Every measured proof is fully verified. Reports include prover phases,
verifier time, serialized proof size and whole-process maximum RSS.

For the isolated PCS, `--log-words` counts F128 words rather than binary gates:

```sh
RUSTFLAGS="-C target-cpu=native" cargo +nightly-2026-03-21 bench --locked -p mikros-whir --bench whir_pipeline -- \
  --log-words 20 --threads 4 --reps 3
```

Use the matching `mikros-fri --bench fri_pipeline` entry for the FRI arm.
Input generation and evaluation of the expected claim are outside both timers.

Upstream: [Plonky3](https://github.com/Plonky3/Plonky3),
[binary WHIR API](https://docs.rs/p3-binary-pcs/0.8.0/p3_binary_pcs/whir/).
