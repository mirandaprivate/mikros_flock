# Revised packed permutation

The `packed` module implements the polynomial-tree and packed-Hadamard
permutation reduction. The `permutation` module provides the legacy protocol,
which has a separate proof format and transcript domain.

## Layout and protocol

Let `b` be the actual subfield width selected by `SmallField::for_width`, and
`kappa = 128 / b`. The supported packed shape has `kappa >= 2` and a padded input
length at least `kappa^2`.

Storage uses `A[row, column] = input[kappa * row + column]`. Within each column,
adjacent rows are multiplied at each compact level. This is a fixed public index
permutation of the manuscript's column-major matrix and upper/lower row pairing;
the final source query is mapped back to the original input addresses.

There are `log2(kappa)` compact polynomial levels. Their highest level has
`N / kappa` entries, which are evaluated at the fingerprint challenge and become
the leaves of the ordinary extension-field product tree. The reverse proof then
uses one packed column reduction per compact level:

1. Aggregate the compact coefficients against the current row tensor and then
   evaluate the resulting scalar polynomials at the fingerprint challenge.
2. Fix the column claims and the degree-`kappa-2` quotient before sampling the
   column challenge outside the interpolation domain.
3. Contract the child columns, evaluate their polynomials, and prove the
   resulting weighted Hadamard claim by cubic sumcheck.
4. Bind both child values before the fresh row-mixing challenge.

Both grand-product trees advance in lockstep. Both sides' messages are bound
before every shared challenge. The terminal functional has a row MLE point and
`kappa` arbitrary column weights; it is not generally an ordinary MLE query.

For small base fields, interpolation uses a true embedded GF(256) intermediate
field. Coordinate conversion is checked through the existing extension-field
embedding; unrelated subfield coordinate systems are not cast to each other.

## Bit wiring and the PCS

Bit wiring encodes `(witness_bit, address)` as `witness_bit + 2 * address` in the
tuple field. At `N = 2^30` its width is 31 bits, so the actual field is GF(2^32)
and `kappa = 4`. This differs from a permutation of untagged binary entries,
whose base field is GF(2) and whose packing factor is 128.

`wiring::prepare_bits_packed` performs challenge-independent preparation.
`prove_bits_prepared_packed` and `verify_bits_packed` return a `BitTensorClaim`:
128 column weights, low-bit-first row factors, and an evaluation on the original
packed witness. The public id and sigma tags are removed online. The two encoded
source values are checked separately; their original-bit functionals must agree
before they become one claim.

The E2E adapter uses the existing tensor-functional interface and the same
production dense merged PCS, commitment, backend parameters, and single inner
opening as the previous comparison. It retains the original witness gather and
source authentication. Select the variant independently from the gather policy:

```text
cargo run --release -p mikros --example e2e_bit_compare -- \
  --log-leaves 14 --threads 64 --reps 6 --warmups 1 \
  --reduction lockstep --permutation both
```

This runs Flock, previous Mikros, and packed-permutation Mikros in rotating order.
Omitting `--permutation` now selects `packed`, comparing Flock with the optimized
new Mikros path. `--permutation legacy` explicitly selects the previous protocol;
the library's existing legacy APIs retain their original types and behavior.
At `log-leaves=14`, the E2E fixture has `2^30` witness bits and `2^26` wiring bits.
It must not be reported as an E2E proof of `2^30` permutation cells.

## Measurement boundaries

```text
cargo run --release -p mikros --example permutation_packed_compare -- \
  --log-n 30 --threads 64 --reps 8 --warmups 1
```

This standalone benchmark has exactly `2^30` bit-address permutation cells and
uses a public permutation moving bits across both words and bit coordinates.
Old and new variants receive the same witness and public permutation. Their
offline preparations are measured and reported separately. All challenge-bound
evaluation, coefficient aggregation, folding, and conversion to a source query
remain in prover time. Verification, proof serialization, and an independent
evaluation against the original bits are outside prover time.

The standalone benchmark is a PIOP comparison, with no PCS or grinding. Its
untimed source oracle must not be described as a succinct opening. The E2E
comparison includes commitment, online PIOPs and conversions, gathering, PCS
opening, and the configured transcript grinding.

For the paper's single-instance hash comparison, run
`cargo bench -p mikros --bench single_hash_compare` from the repository root.
Prepared coefficient storage includes off-domain polynomial products;
the standalone benchmark reports this storage separately from online proving.

The hardware implementation uses four-row VPCLMUL accumulators for coefficient
aggregation. For the GF(2^32), kappa-four compact layers, a GFNI affine transform
projects 64 input rows at once; its challenge-dependent matrices are built
online. Other shapes retain the scalar lookup implementation. Extension layers
and projected children reuse a separate bounded tree pool (slot 2), with the
same 32-GiB limit as the legacy Combined pool (slot 1). Reused storage is fully
overwritten; proof bytes and challenges are unchanged by either optimization.
The standalone benchmark reports pool occupancy and the batch's synchronous
scratch teardown separately from individual online proofs.

The default GF(2^32) coefficient aggregator now embeds contiguous coefficients
with GFNI into a bounded 16-KiB tile and immediately consumes that tile with
four-row VPCLMUL accumulators. It handles 6, 8, 12 and 16 coefficients per row.
Blocks below 64 rows and unsupported shapes or CPUs retain the original kernel.
The highest compact layer also uses GFNI polynomial evaluation, with its output
kept in the original physical row order. Challenge-dependent evaluation matrices
are built online; only the fixed public subfield embedding matrix is cached.

For ablation, `MIKROS_PACKED_AGGREGATE=baseline|gfni|tower|auto` selects the
coefficient kernel (`auto` is the default). The experimental `tower` path converts
the shared low equality weights online to four coordinates over GF(2^32), then
accumulates raw 32-bit polynomial products before reducing and recomposing them.
It retains exactly the same GHASH field and source coefficient basis. It was
comparable to GFNI in the native probes, so it remains opt-in. Both alternatives
use four VPCLMUL instructions per four rows per coefficient; they do not halve
the carryless multiplication count. Online weight conversion costs remain part
of proving time when comparing the kernels.

## Binary PLONK copy path

The single-instance binary PLONK circuit uses three wire columns and an IO
slot, padded to `4N` copy cells. At `N = 2^30`, a tagged cell needs 33 bits;
this path uses GF(2^64) coefficients and `kappa = 2`.

`prepare_bounded_from_fn` writes tagged rows directly into coefficient storage,
without first constructing and decoding bit-plane vectors. The compact K64
leaf aggregator generates parent columns and the off-domain product inside
bounded worker tiles, then immediately accumulates them. Terminal source
queries share row weights; the two witness queries use a joint packed scan.
Witness-dependent preparation, challenge-dependent tables and source query
evaluation all remain online. These changes preserve proof bytes and transcript
order.

With PCLMUL enabled at compile time, K64 multiplication uses reciprocal
polynomial reduction in the same field, replacing eight reduction-table reads
with two additional carryless products. The K64 leaf aggregator handles its
fixed multiplication by the basis element `x` with a shift and XOR. It aggregates
five coefficients and reconstructs the off-domain linear coefficient from the
two parent linear coefficients after weighted summation. This reduces worker
scratch from 48 to 40 bytes per row; the original materialized aggregate remains
available for differential checks.

Run the full PLONK proof, including its configured PCS, with:

```sh
RUSTFLAGS="-C target-cpu=native" cargo bench -p mikros --bench circuit_sat -- \
  --log-n 30 --threads 64 --system mikros --reps 1 --warmups 0
```

For stage diagnostics, invoke the worker directly; the comparison driver clears
diagnostic environment overrides:

```sh
MIKROS_PERMUTATION_PROFILE=1 RUSTFLAGS="-C target-cpu=native" \
  cargo bench -p mikros --bench circuit_sat -- \
  --worker --system mikros --log-gates 30 --threads 64 \
  --repetitions 1 --warmups 0 --seed 97
```

Direct workers accept `MIKROS_PERMUTATION_REFERENCE_PREPARATION=1`,
`MIKROS_PERMUTATION_REFERENCE_AGGREGATE=1` and
`MIKROS_PERMUTATION_REFERENCE_TERMINALS=1` for the corresponding original paths.
K64 projection selects GFNI automatically for inputs of at least 64 KiB on
supported CPUs. `MIKROS_PERMUTATION_GFNI_PROJECTION=0|1` disables or forces it;
`MIKROS_PERMUTATION_REFERENCE_PROJECTION=1` overrides it. The existing K32 GFNI
dispatch is unchanged. Native GFNI/AVX-512/VPCLMUL kernels require matching CPU
features and a native build; other CPUs retain the existing fallback paths.

For K32 tags (`kappa = 4`), the compact prover also consumes off-domain
coefficients in bounded tiles. Fixed interpolation weights use shared byte
tables; the already-prepared parent layer is reused. The highest non-leading
sample coefficient is reconstructed from the weighted parent coefficients,
leaving three or nine coefficients per sample row for the GFNI accumulator.
Monic quadratic products use three field multiplications through Karatsuba.
Native K32 multiplication and delayed product reduction use the reciprocal of
the existing modulus, with the original table reduction retained for portable
builds. These changes preserve the field representation, proof messages and
challenge order. `MIKROS_PERMUTATION_REFERENCE_AGGREGATE=1` retains the
materialized aggregate for differential measurements.

For the complete single-instance PLONK proof at `2^20` gates, including FRI:

```sh
RUSTFLAGS="-C target-cpu=native" cargo bench -p mikros --bench bitwidth_instances -- \
  --worker --case single --bits 8 --log-gates 20 --threads 1 \
  --repetitions 3 --warmups 1 --seed 97
```

The K32 core measurement includes online row preparation and permutation
proving, but excludes PLONK terminal projections and PCS:

```sh
RUSTFLAGS="-C target-cpu=native" MIKROS_ENGINEERING_INPUT=random \
  cargo test --release --locked -p mikros-permutation --test engineering_k32 \
  -- --ignored --nocapture --test-threads=1
```

It defaults to `2^22` tags, one worker, one warmup and three measurements;
`MIKROS_ENGINEERING_INPUT=copy` selects binary-value/position tags instead of
full-width pseudorandom tags. The test verifies every proof and reports proof,
terminal-claim and transcript identity alongside timings.
