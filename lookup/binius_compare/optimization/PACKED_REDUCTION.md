# Coefficient-domain column reduction

The existing compact node is a monic polynomial in K[Z], degree d=2^level,
stored without its leading coefficient. K is the actual subfield containing
the whole value-and-position tuple. Its embedding into E is unchanged.

For each compact child layer, use kappa=min(128/(d*log2|K|), remaining gate
count), rounded down to the allowed power of two. Group kappa consecutive
gates; their selector bits are the low coordinates of the gate point.
The row/column order is an explicit permutation of the manuscript layout.

For each of the two trees and left/right child, interpolate the kappa formal
polynomials A_j(Z) at H={0,...,kappa-1} inside K, obtaining A(X,Z).
Products and division by Psi(X)=product(X+h) take place in K[Z][X], before
evaluating Z=z. A word packs formal coefficients, never a new extension field.
Temporary polynomial vectors may contain several words.

Let l be the row eq weights, r the column eq weights, and lambda the already
sampled two-tree batching challenge. Send

  C_j = sum_i l_i [A_ij(z)B_ij(z) + lambda A'_ij(z)B'_ij(z)],
  H(X) = (sum_i l_i [A_i(X,z)B_i(X,z)
                    + lambda A'_i(X,z)B'_i(X,z)] - sum_j C_j L_j(X))/Psi(X).

Check the incoming claim equals dot(r,C). Absorb C and the kappa-1 quotient
coefficients before sampling x outside H. The shorter claim is
dot(L(x),C)+Psi(x)H(x), proved by the equality-factored batched inner product
over row coordinates only. Its four terminal values are unweighted witness
MLEs; eq factors were removed by the inner-product verifier.

Send four vectors V_child[j]=MLE_row(A_j(z),y), each of length kappa. Check
their L(x)-weighted sums against all four inner-product terminal values.
Absorb all vectors BEFORE sampling fresh ordinary Boolean column coordinates
eta. Combine with eq(eta), then sample the child selector. The resulting
ordinary child MLE point is [child_selector, eta..., row_point...]. This
continues the tree induction and finally reaches the original committed bits.
Public z is removed only at the leaves; both public tag components are still
authenticated by the existing verifier-side decomposition.

Shape-derived kappa, quotient length, inner-product rounds, all four terminal
vectors and every compact/dense layer tag must be checked by the verifier.
The transcript has its own version. Each packing reduction adds a univariate
identity check of degree at most 2*kappa-2, plus randomized column reduction;
the old protocol's soundness bound cannot simply be reused unchanged.

This implements the manuscript's quotient/Lagrange reduction using conventional
Boolean eq weights and a batched quadratic-core inner product. Performance is
an experimental outcome: symbolic preprocessing can outweigh skipped rounds.
