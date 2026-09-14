# Challenge degrees of this implementation

These are bounds for the actual independent cubic product-tree protocol in this
crate. They are not a claim that an unground F128 protocol has overall 128-bit
soundness, or that adding a fixed number of PoW bits establishes such a claim.

Let `N = padded_len(rows)` and `mu = log2(N)`. The committed protocol samples:

1. One fingerprint challenge `z`. For unequal source multisets, the difference
   of their two monic degree-N products is nonzero and has degree at most N-1.
   See `latex/4-block.tex`, the argument following
   `eq:packed-permutation-fingerprint`. The compact coefficients are an exact
   way to compute these products; they do not decrease this degree bound.
2. For each of two independent trees, layers `ell = 0 .. mu-1` sample `ell`
   sumcheck challenges of degree at most three, followed by one linear merge
   challenge. See `latex/appx-proof.tex`, “Product-tree soundness”, and the
   matching messages/challenges in `src/fast_sumcheck.rs` and `src/sumcheck.rs`.

The challenge-event order is therefore `[N-1]`, then, twice,
`[1], [3,1], [3,3,1], ...`. The event count is `1 + mu^2 + mu`.
There is no extra batching challenge between the two trees. Oracle-shape, sigma
digest and domain-separation bindings do not sample challenges.

Without grinding, a direct interactive union bound, before terminal openings, is

```
[(N - 1) + 3 * mu * (mu - 1) + 2 * mu] / 2^128.
```

At N=2^30 the fingerprint term alone is about 2^-98. For the special copy
relation with identical values on both sides, the Z^(N-1) coefficient also
cancels, but using N-1 is a safe common bound and avoids relying on additional
instance structure.

The inherited standalone protocol and this crate's unwrapped APIs have no PoW
messages. A wrapping challenger can implement a matched prover/verifier nonce
schedule, but it must cover the actual event degrees. Under the repository's
computational PoW work-factor convention, `grinding_bits_for_degree(d)` uses
`floor(log2(d)) + 1` for nonzero d: the per-event values are mu for fingerprint
(power-of-two N>1), two for cubic rounds, and one for linear merges. This makes
each event smaller than 2^-128 in that convention, not their union. A common
additional union margin must include all gate, wiring, transport and PCS events,
and the PCS profile/security assumptions must be accounted for separately.

Authentication obligations remain essential: both leaf queries must be opened
against the commitment bound before z, and public sigma must be fixed by the
verifier's statement. The cached sigma evaluation here is computed directly from
that public table, not accepted from the prover.
