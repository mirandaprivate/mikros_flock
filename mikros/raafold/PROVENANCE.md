# Source provenance

The four `src/raafold*.rs` modules are adapted from the local BrakeFold
repository, commit `007f8f919f6236b51a041a084285d8d3fee6d59d`, under its MIT license.
The original modules implement three accumulators. This port changes the
encoding and matching generator proof to Repeat4/P1/Acc/P2/Acc, uses rate 1/4
at every level, and domain-separates the modified protocol. The distance
0.19 is an explicit conditional assumption, not a certificate for any sampled
finite-length code. Query count is ceil(-lambda/log2(1-0.19/3)).

The Opening and recursive Decide APIs, authenticated fixed H and dynamic S
claims, row Merkle commitments, and bounded Ligero tail are retained.
Two verifier tensor inner products are evaluated with logarithmic product
formulas instead of allocating message-length vectors.
The external opening API uses the ordinary Boolean MLE in MSB-first coordinate
order, replacing the original homogeneous-coefficient evaluation convention.
The modified convention has a distinct transcript domain and proof magic.
The inherited four-slot grand-product layout currently contains two real and
two zero-padding permutation slots.

The inherited PCS and field tests are retained with shape expectations adjusted
to the two-accumulator rate-1/4 profile. RAAA-specific generator/packed-code
test modules are replaced by tests of the new encoding profile.

Binius64 dependencies are pinned to commit
`a41ff589ad23ed6e158e997c0ba85ede777fdce8`. Field multiplication uses the
RustCrypto POLYVAL representation; it is not interchangeable with GHASH raw
bits without an explicit isomorphism.

The direct POLYVAL CLMUL kernel is adapted from RustCrypto `polyval` 0.6.2,
`src/backend/clmul.rs` (MIT OR Apache-2.0). Four-lane VPCLMUL kernels apply the
same arithmetic independently per lane. Scalar and SIMD products, unreduced
accumulation, and bulk operations are checked against the original RustCrypto
implementation and an independent polynomial oracle. These engineering changes
preserve the field representation, transcript, commitments, and proof encoding.
