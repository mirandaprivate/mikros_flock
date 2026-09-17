# F128 WHIR integration

The optional `whir` feature authenticates permutation and Lasso source claims.
The `authenticated` API takes the public statement and proof; source witnesses
remain on the prover side.

| Path | Contents |
|---|---|
| `src/whir_field.rs` | arkworks field interface for FLOCK's GHASH F128. |
| `src/whir_rs.rs` | Additive-domain Reed–Solomon evaluation. |
| `src/whir_pcs.rs` | Commitment, linear opening, and verification. |
| `src/authenticated.rs` | PIOP, Booleanity, padding, and PCS composition. |
| `vendor/whir/` | Vendored WHIR implementation and licenses. |

Each source bit occupies one F128 element. Source commitments, shapes, PCS
parameters, and application context are bound before the reduction challenges.
Booleanity and padding checks share the authenticated source vectors. Opening
queries are derived from the verified reduction transcript.

The configured WHIR profile uses rate 1/2, BLAKE3, unique decoding, a 96-bit
backend target, and zero PoW bits. The composition is non-zero-knowledge; its
total error also includes the outer reduction. Sources are limited to `2^20`
elements and openings to 32 linear claims. Public covectors are constructed
and evaluated explicitly in linear time.

From this directory:

```bash
cargo +nightly-2026-03-21 test --locked -p mikros-lookup
cargo +nightly-2026-03-21 run --release --locked -p mikros-lookup \
  --example whir_lookup -- 64 8
```

The example arguments are `rows value_bits`. Output includes proof size,
prover time, verifier time, and the verification result. Prover time includes
commitment, auxiliary witness construction, reduction, Booleanity, and openings.
Input generation is separate. The paper's four main comparisons use FLOCK's
native PCS and are documented in the repository README.

The vendor source manifest records imported file hashes and local changes.
The adapter exposes RS message coefficients and handles standard sumcheck
in characteristic two; zero-knowledge construction rejects characteristic two.
