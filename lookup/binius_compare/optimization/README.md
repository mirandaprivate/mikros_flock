# Mikros permutation + Binius: ordered optimization

Measured results and recommended mode: [RESULTS.md](RESULTS.md).

This standalone workspace implements and measures the five optimization steps
in `PLAN.md`. It keeps the original permutation relation `f[sigma[i]]=g[i]`,
original value-bit commitments, Flock Binius ring-switching/Ligerito PCS policy,
and authentication of both terminal source claims. The verifier never receives
the private f/g arrays. The baseline snapshot and live workspaces are unchanged.

## Modes

| Mode | Input/tree | GKR | Column adapter |
|---|---|---|---|
| `reference` | Original bit conversion and compact tree, instrumented | Original two cubic chains | Original scalar |
| `full_extension` | Original conversion, full F128 tree | Original two cubic chains | Original scalar |
| `direct` | Checked integer rows, block bit transpose | Original two cubic chains | Original scalar |
| `lookup` | Direct rows, PCLMUL, delayed coefficient reduction, byte evaluation tables | Original two cubic chains | Original scalar |
| `fused` | Same optimized compact tree | Split-eq and fused fold/message, original cubic messages | Original scalar |
| `batched` | Same optimized compact tree | Two trees in one quadratic-core chain | Original scalar |
| `bridge` | Same optimized compact tree | Batched | Four-bit subset-sum tables |
| `full_optimized` | Direct rows, byte embedding, full F128 tree | Batched | Same optimized adapter |
| `packed` | Optimized compact coefficient tree | Batched with coefficient-domain quotient reductions | Same optimized adapter |

`reference` through `fused` produce the same complete proof bytes as the frozen
baseline. `batched`, `bridge` and `full_optimized` produce identical proof bytes
to one another. The packed protocol has a separate version and additional
quotient/column checks. Implementations of each protocol are deterministic
across thread counts; reference, batched and packed protocols need not have the
same transcript or security error bound.

Flock is always measured with its untouched native product-GKR. Its `reference`
arm retains the original adapter; its `bridge` arm gets the same block transpose,
byte embedding, and bit-column evaluator as optimized Mikros. This separates
common adapter gains from differences between permutation PIOPs.

## API

```rust,ignore
use mikros_binius_optimized::{prove_with_mode, verify, Mode, System};
let out = prove_with_mode(&f, &g, &sigma, 8, System::Mikros,
    b"my-application", Mode::FullOptimized)?;
verify(&out.statement, &out.proof, System::Mikros, b"my-application")?;
```

`Statement`, `Proof`, `prove`, `verify`, and `parameters` mirror the original
PCS adapter. Applications must supply their expected public statement/context.
`prove` defaults to `FullOptimized`, selected by the completed one/six-core
N=2^20 ablation. This prioritizes speed; `Bridge` retains compact tree storage,
and `Packed` enables the quotient reduction. The default still proves Mikros's
exact tagged tuple relation and uses the new Mikros batched GKR.
`prove_with_mode` exposes ablations; mode selection is a prover implementation
choice. The proof variant and its domain-separated transcript select the
verifier protocol. `verify_with_mode` only selects equivalent scalar/fast public
tag arithmetic for measurement, not a weaker verification policy.

Prepared input fields are private. Their constructor validates ranges and
computes the same canonical bit-oracle identifiers once. Tree construction
then borrows the checked integer rows, avoiding bit-to-integer decoding and
duplicate source hashing. The public tag remains in the exact tuple relation;
it is removed from terminal claims using the actual public sigma.

## Packed reduction

See `PACKED_REDUCTION.md` and `kernel/src/packed.rs`. For compact child degree d
and actual subfield degree k, the available column packing is `128/(d*k)`, capped
by the number of gates and distinct K interpolation nodes. A layer skips
`log2(kappa)` row sumcheck rounds; it sends a quotient identity and authenticates
all four terminal column vectors before randomizing their column coordinates.
Formal K[Z] products use the same modulus/embedding and delayed reduction.
No packed coefficient word is interpreted as an invented larger field.

Packing can reduce inner-product work while increasing preprocessing work.
The experiment retains that mode regardless of whether it is fastest.

## Reproduce

Run inside WSL from this directory, with the Rust toolchain in PATH (for example
through `bash -lc`). Use package-specific formatting: the frozen snapshot
deliberately omits unrelated WHIR modules.

```sh
cargo +nightly-2026-03-21 fmt -p mikros-binius-optimized -p mikros-binius-kernel -- --check
cargo +nightly-2026-03-21 clippy --release --offline -j 2 --workspace --all-targets --no-deps -- -D warnings
cargo +nightly-2026-03-21 test --release --offline -j 2 --workspace -- --test-threads=1
cargo +nightly-2026-03-21 build --release --offline -j 2
python3 run_experiment.py --suite ablation --name NEW_ABLATION
python3 summarize.py results/NEW_ABLATION
python3 run_experiment.py --suite scaling --name NEW_SCALING
python3 summarize.py results/NEW_SCALING
python3 audit_baseline.py
```

The full ablation uses 11 arms and 11 measured cyclic orders, plus one warmup
per arm, at N=2^20, 8 bits, one/six physical cores. The scaling suite uses six
arms and six measured cyclic orders across N=2^16,2^18,2^20 and widths 1,8,32,
plus the six-core N=2^20/8-bit case. All runs are sequential. Detected competing
builds/benchmarks invalidate an entire retained batch; no slow samples are
discarded. Each proof is serialized, deserialized, verified through both PCS
openings, and checked for transcript synchronization before recording success.

Results include phase timers, min/max totals, source/executable hashes, CPU,
toolchain, query parameters, execution logs and validation reports. The formal
suites retain the exact measured optimization files in `sources.zip`. Phase
medians do not necessarily add to the median total. Fine timers add some
instrumentation overhead, so the baseline is identified as instrumented.

The inherited PCS uses Fast100 or explicit small-shape diagnostic settings,
without ZK. New batching and packing protocols have their own algebraic error
terms. These are component benchmarks, not a claim of equal complete 128-bit
soundness or complete Flock R1CS performance.

Imported local kernels and their hashes are recorded in `IMPORTS.json`; the
original optimized workspace was not edited. Hardware-specific AVX-512 code
has a scalar/PCLMUL fallback; the local i7-8700K exercises the latter.
