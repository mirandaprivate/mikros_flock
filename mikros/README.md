# Mikros implementation

The crate implements packed Hadamard, inner-product, and multiple-instance
reductions over binary fields. Build instructions and experiment entry points
are in the [repository README](../README.md).

## Structure

| Path | Contents |
|---|---|
| `src/hadamard.rs` | Binary Hadamard reduction with packing factor 128. |
| `src/univariate/` | GF(256) interpolation, lookup tables, and SIMD aggregation. |
| `src/packed_ip.rs` | Packed inner-product reduction. |
| `src/multiple_gate.rs`, `src/multiple_image.rs` | Multiple-instance gate and matrix reductions. |
| `src/sparse_lookup.rs`, `src/packed_sources.rs` | Sparse-matrix lookup and packed commitment transport. |
| `permutation/` | Packed permutation and bit-wiring reductions. |
| `fri/` | Default binary PLONK PCS: packed F128 BaseFold MLE-check and additive FRI. |
| `whir/` | Optional Plonky3 binary WHIR PCS over full F128 packed words. |
| `examples/e2e_common/` | Circuit fixtures, shared PLONK driver and PCS integration. |
| `benches/circuit_sat.rs`, `single_instance.rs` | Mikros PLONK + FRI versus FLOCK public-CSC R1CS + native PCS. |
| `examples/e2e_hash_compare.rs` | SHA-256, Keccak, and BLAKE3 comparisons. |
| `benches/univariate_vs_flock.rs` | Binary Hadamard comparison. |

## Hadamard API

Inputs use consecutive groups of 128 bits as matrix rows. Interpolation takes
place over GF(256), with challenges in GF(2^128). `Kernel::PairAggregate` selects
the optimized aggregation path; hardware dispatch uses GFNI and AVX-512 when
available and falls back to lookup tables otherwise.

| API | Result |
|---|---|
| `prove` | Proof and three terminal matrix claims. |
| `prove_with_scratch` | The same reduction with reusable scratch capacity. |
| `verify` | Transcript verification and terminal claims for PCS authentication. |
| `check_claims` | Linear-time evaluation of the claims on the source inputs. |

Bind the public statement and input commitments to the challenger before the
reduction, then authenticate all terminal claims with the PCS. The end-to-end
binary PLONK examples compose these steps with packed F128 additive FRI.
The FLOCK baseline and the hash/multiple-instance experiments use native
Ligerito PCS. See [the FRI module](fri/README.md) for the opening interface.

## Tests

```bash
cargo +nightly-2026-03-21 test --release --locked -p mikros -p mikros-permutation
```

The tests cover reference and optimized kernels, transcript replay, source
evaluations, and rejection of altered proofs.
