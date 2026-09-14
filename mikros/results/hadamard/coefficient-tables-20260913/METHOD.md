# GF(256) coefficient-table Hadamard comparison

The user-provided subsection proposes cached GF(q^t) addition/multiplication
tables, a monomial Lagrange coefficient matrix, and per-row schoolbook
polynomial products. This experiment compares that construction with the
existing evaluation-domain Mikros Hadamard prover. It does not change the
protocol, proof messages, field, challenge sequence, or PCS boundary.

## Fixed parameters and arithmetic

For binary kappa = 128, the proposed rule first tries t = 7, but
128 < 7 ceil(128/7) = 133. The next t = 8 works, so K = GF(256),
E = GF(2^128), and [E:K] = 16. These are already the implemented fields.
The GF(256) multiplication table has 256^2 one-byte entries (64 KiB).
Addition is byte XOR and does not need an addition table. The E/K coordinates
use the existing field-basis conversion, not raw bytes of the GHASH value.

The existing PairAggregate computes 128 column products directly as bit AND
and 127 effective continuation products in GF(256). Its hardware path
computes 128 continuation lanes and discards the point 255. It reconstructs
the quotient once after all rows have been aggregated.

Dense coefficient schoolbook multiplication takes 128^2 = 16,384 GF(256)
products per row. The experimental implementation improves on this baseline:
it computes only the 127 high coefficients, requiring at most
sum(i, i=1..127) = 8,128 scalar products per row. This is 64 times the 127
effective continuation products, but is not a prediction of the full-prover
runtime ratio. Zero coefficients, SIMD, encoding, weighting, and the shared
IP tail affect actual time.

The counts 127 versus 8128 compare only the product stage, not all first-round
work. Naively mapping both binary input rows to kappa-1 continuation points
has 2*kappa*(kappa-1) scalar factors, while mapping them to kappa monomial
coefficients has 2*kappa^2 factors. In both cases each factor multiplies a
binary input bit by a fixed GF(256) constant, so it is a selection/XOR and
admits input-byte tables. These are not arbitrary GF(256) multiplications.
After that encoding, full schoolbook convolution adds kappa^2 arbitrary
GF(256) multiplications and (kappa-1)^2 additions; approximately 2*kappa^2
counts multiplication and addition together, not multiplications alone.
Both complete constructions can have O(N*kappa) arithmetic complexity while
having substantially different costs in the actual binary implementation.

## Why the high half is sufficient

Let P(X) = sum_i l_i g_A,i(X) g_B,i(X), with degree at most 254, and let
Psi(X) be the monic degree-128 vanishing polynomial of H. Division gives
P = Q Psi + R with deg R < 128. On H, R(gamma_j) = P(gamma_j) = c_j;
therefore R = g_C by uniqueness of interpolation and Q is exactly the
original h. Monic long division determines Q from coefficients 128..254
alone. Terms below degree 128 cannot affect any later quotient coefficient.

The experimental aggregate stores the same 128 column claims followed by
the 127 weighted high product coefficients. It applies sparse monic division
directly; it does not convert coefficients back to point evaluations.
This H is an additive binary subspace. In the AES GF(256) representation,
Psi has nonzero coefficients only at degrees 1, 2, 4, 8, 16, 32, 64, 128.

## Experimental variants and timing

- `pairreuse`: the existing optimized evaluation-domain kernel.
- `coefflookupreuse`: high schoolbook products using the existing GF(256)
  multiplication table.
- `coeffnativereuse`: the same high-product construction using GFNI/AVX-512
  when available, with a portable fallback.
- `flock128`: existing Flock degree-two zerocheck with seven skipped rounds,
  so its initial compression is also 128.

The coefficient variants also use a 512 KiB input-byte-to-coefficients XOR
table instead of constructing every row with 128 separate bit selections.
This is input-independent preprocessing and is retained across proofs.
The native coefficient variant reuses PairAggregate's exact 64-row,
four-coordinate register consumer and batched field-basis conversion.
Its outer Rayon blocks contain 512 rows, versus 2048 for PairAggregate;
small inputs can therefore use different numbers of workers. This is a
comparison of the implemented paths, not an isolated instruction benchmark.
Witness-dependent row encoding, row products, random-weight aggregation,
and the full shared tail remain inside the prover timer. They do not cache
expanded witness polynomials or products between proofs. All candidates
start with the same packed binary inputs and use independent scratch capacity.

Public table initialization is included in each process's first proof.
Steady-state results exclude warmups; independent fresh-process records show
the cold behavior. No offline timing is subtracted from an online operation.

If fixed-witness product preprocessing is allowed in a different application,
one could cache the 127 high product coefficients per row before the online
challenge. At N = 2^30 this is 1016 MiB, plus 128 MiB for H product bit masks
(the original inputs are still needed for challenge-dependent packing).
The existing path can equally precompute its 127 continuation products and
the same masks, with the same storage. Such a comparison must give both
paths this preprocessing opportunity and report preprocessing time, storage,
and reuse count. It is distinct from caching public multiplication tables.

The existing profile field `interpolate_quotient_ms` denotes quotient
reconstruction for every variant: interpolation for PairAggregate, sparse
division for the coefficient kernels. Compare the per-sample sum
`small_field_aggregate_ms + interpolate_quotient_ms` as certificate generation.
Do not sum separately computed medians or combine stages from one process
with total times from another process.

This is a direct packed-opening PIOP comparison, with independent source
claim checks outside the timer. It excludes PCS and disables grinding, as
the pre-existing Hadamard benchmark does. It is not an end-to-end SNARK or
an equal-security comparison with Flock. Mikros kernels must have identical
proof bytes, returned claims, and next transcript challenges.
