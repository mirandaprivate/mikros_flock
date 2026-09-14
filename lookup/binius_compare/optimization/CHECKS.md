# Validation record

The final local release suite contains 13 test functions, each covering multiple
shapes, modes, or adversarial edits. `checks-default.log` records their execution
after selecting the new default; `clippy-default.log` and `build-default.log`
record the final checks/build. Earlier validation logs are retained too.

- 1,000 independent arithmetic comparisons per actual subfield (degrees
  1,2,4,8,16,32,64,128), against the separately compiled frozen implementation.
- Block transpose bytes and parallel digest match canonical scalar encodings,
  including empty, non-byte-aligned and 128-bit inputs.
- Direct, lookup and fused engines, compact and full trees, match frozen PIOP
  proof bytes, all claims and transcript tails for multiple padded shapes.
- An end-to-end golden test compares all five legacy modes with the original
  separately compiled Binius adapter, including complete PCS proof bytes.
- Every retained tree layer equals independently multiplied leaf blocks.
- Batched GKR matches direct authentication of both original sources, including
  a parallel-sized shape. Compact and full trees have identical batched proofs.
- The quadratic-core recurrence handles eq coordinates zero and one, as well
  as zero/one/random batching challenges, without division by zero.
- Formal K[Z][X] quotient identities are checked at independent extension-field
  x,z points for every compact subfield/degree configuration.
- Packed GKR reaches both original input oracles, including padding. Tests
  alter every column claim, quotient coefficient and terminal column entry,
  truncate quotient shape, and substitute dense/packed layer types.
- Scalar and subset-sum bit-column evaluators agree over multiple sizes/widths.
- All end-to-end modes authenticate f and g with the real Binius/Ligerito PCS.
  Rejection tests change contexts, source commitments, public sigma, PIOP roots,
  column values (including edits preserving their PIOP weighted sum), padding,
  ring-switch messages, Ligerito messages, and source-opening counts.
- Formal experiments deserialize/replay every timed proof and compare exact
  proof bytes within each protocol across modes and one/six-thread execution.

The imported AVX-512 paths are not exercised on this i7-8700K. Local tests and
measurements exercise scalar/PCLMUL code. Tests do not constitute a formal
cryptographic security proof; new batching/packing error terms are described
separately from the inherited PCS parameter policy.

Earlier intermediate logs are retained: a benchmark option needed an explicit
usize, two test expressions needed cast/shift parentheses, and Clippy requested
one precedence clarification. These were corrected before final validation.
The first pilot invocation lacked Rust in its non-login PATH and stopped before
measurement; `pilot-20260912-v2` uses the login shell and is the completed pilot.

An initial `cargo fmt --all` attempted to inspect the intentionally incomplete
frozen lookup module tree and stopped at the omitted WHIR test module. Subsequent
formatting targets only the two new packages. The source audit confirmed all
308 frozen copied files remained byte-identical; no baseline restoration was
needed. `audit_baseline.py` repeats this check before/after each formal run.
The final audit additionally covers both generated snapshot files, for 310
matching hashes. Formal suites validated 684 accepted proofs; the cross-suite
check and source archives are recorded in results/validation-all.json.
