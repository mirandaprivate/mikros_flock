# Local analysis of the v8 paired continuation machine code

This is a local analyst note, written after downloading the server's existing
disassembly. It is not a server-generated measurement. No source, executable,
or running experiment was changed during this audit. N30 results are outside
this note's scope.

## Artifact identity

The inspected executable is the native server `univariate_vs_flock` binary
recorded by `assembly_manifest.json`. Its SHA-256 agrees with
`aggregate_build_artifact.json`:

```text
executable       c26172d4caa709fa3ee43a5b077757e56378702afe992c08d2b360daa9bc287f
assembly_manifest c47e5ad69f8864b9445822487ce6471ec8e5602cdf4953ad8b1c3dd963b094d9
function-4.asm   297d88555b7b431484464dbb2bced0a5d3c9baf9e8c06575096ffb54d4964316
function-8.asm   92084258d633d73d583a131ef85837980999a97dbc32935ad282e8c83c2d075e
source_manifest  e70a7c7079ce1661e0d7bdfe2cbf722a9863fd173cfda7c069ec0f5d1b6ed2ed
source.tar.gz    df582dd7396ad9478106b1383e3b9c6c71f13483406d812b802c9dfa0d1f253f
```

The manifest and function hashes above were recomputed locally. The executable
hash is read from the two downloaded records; the executable itself was not
downloaded or rehashed locally. The build artifact records 80 passed tests,
`+nightly-2026-03-21`, rustc `1.96.0-nightly ac7f9ec7d`, and LLVM 22.1.0.

All line numbers below refer to the downloaded `.asm` files, which include
instruction-byte continuation lines. They are not source line numbers.

## Same-executable comparison

The old producer remains in `function-8.asm`, `tiled::aggregate_block`, starting
at `0x67d4c0`. Its first continuation has the following control flow:

| Operation | Address | Assembly line |
|---|---|---:|
| Initialize public chunk index to 1 | `0x67e65f` | 975 |
| Load chunk byte and calculate LUT row | `0x67e6a9` | 989 |
| Calculate `(chunk >> 1) & 3` | `0x67e6b4` | 992 |
| Test chunk parity | `0x67e6cb` | 998 |
| Even/odd indirect dispatch | `0x67e6d8` / `0x67e6f7` | 1002 / 1009 |
| Advance chunk and compare against 16 | `0x67e783` / `0x67e78a` | 1035 / 1037 |
| Loop back to next chunk | `0x67e78e` | 1038 |

The second continuation repeats this structure: chunk initialization at
`0x67e7c7` (line 1051), loop entry `0x67e811` (1065), indirect dispatch at
`0x67e841` / `0x67e867` (1078 / 1086), and back edge at `0x67e8fe` (1115).
Thus each A/B row pair still executes 30 public-index permutation dispatches,
each containing a parity decision and an indirect jump. These decisions are
about a fixed schedule, not about a challenge or secret-dependent branch.

The Pair candidate is inlined into `function-4.asm`, the aggregate worker
closure. Its row producer occupies `0x66e410` through `0x66e896`
(lines 3505–3739; end-exclusive address `0x66e89c`). The compiler emitted a
straight-line body containing all 16 chunks from both rows. It has exactly one
branch, `jne 0x66e410` at the end, after advancing to the next whole row pair.
There are no calls, indirect jumps, parity tests, or chunk-loop back edges
inside this producer. The top-level kernel dispatch and tile/row iteration
still exist elsewhere; this is not a claim that the full aggregate is
branchless.

The Joint candidate contains another copy of the same producer schedule at
`0x66d120`–`0x66d59b` (function-4 lines 2461–2695). It is distinguished from
Pair by the following call to `consume_h_affine` at `0x66d5c8` (line 2706).
Pair instead continues into the original four-coordinate mask/multiply
consumer operations, beginning with the row loop at `0x66ea10` (line 3822).

## Actual instructions and memory operands

Counting decoded instructions in the complete Pair row-producer interval gives
186 instructions, including its H-mask construction, two GF256 products, tile
stores, and row-loop maintenance. This is a static count of a straight-line
iteration, not a hardware instruction-counter measurement. The same count
and opcode breakdown apply to the Joint copy.

There are exactly 64 `ZMMWORD PTR [rax+...]` reads from the continuation table:

| Instruction carrying the 64-byte table read | Count |
|---|---:|
| `vmovdqu64` | 16 |
| `vpermq` | 32 |
| `vpshufd` | 8 |
| `vpternlogd` | 4 |
| `vpxord` | 4 |
| Total | 64 |

These are 4096 logical bytes per A/B row pair, from the same 32 KiB table.
Memory-operand folding removes separate load instructions at many sites; it
does not remove those table reads, nor establish a reduction in DRAM traffic.

The fixed schedule also enables 28 `vpternlogd ... 0x96` instructions and four
ordinary XOR instructions to accumulate the continuations. Each ternary
operation is XOR of three operands. This represents the same 60 binary XORs
performed by the old four-accumulator, 15-chunk update, using 32 machine
instructions. The new producer also has 32 `vpermq`, eight `vpshufd`, and 16
`vshufi64x2` instructions. Its two permutation-control vectors are loaded once
before the row loop at `0x66e3c5` and `0x66e3cf` (lines 3489 and 3491).

Only `zmm0` through `zmm10` occur in the paired producer. Its entire stack
access list consists of three output stores:

| Address | Assembly line | Purpose |
|---|---:|---|
| `0x66e428` | 3509 | 16-byte H bit mask |
| `0x66e87f` | 3733 | First 64-byte continuation product |
| `0x66e887` | 3735 | Second 64-byte continuation product |

There are no stack loads, continuation-vector spills, initialized temporary
continuation arrays, or clearing calls inside this row producer. The mask and
product stores are the existing tile outputs required by the later consumer.
The old producer also avoids spilling its continuation ZMM values; its extra
stack operations here are scalar row bookkeeping and a saved first-byte
source, for example function-8 lines 958/963 and the read at line 1046. The
new result should therefore not be described as eliminating old vector spills.
Worker scratch initialization and basis-conversion work remain outside this
interval.

## Timing attribution boundary

The frozen source uses the original `consume_four` in Pair mode, and the
snapshot retains its `#[inline(never)]` annotation. Nevertheless, the actual
machine code differs at this boundary: the old worker calls the standalone
function at `0x67d280` four times per tile (function-8 lines 123, 137, 151,
165), while Pair's four-coordinate operations are present directly in the
worker closure, including loops at `0x66ea10` and `0x66ec90`. The source
algorithm and coordinate width remain the same, but code placement and call
structure are not identical. This audit makes no claim about which compiler
pass produced that difference.

The N26 screening files show Pair winning 13 of the 14 matched main runs.
In the separate seven-run diagnostic profile, mean small-field aggregation
time is 5.881214 ms for `basisreuse` and 2.907861 ms for `pairreuse`; respective
medians are 6.024096 and 2.907424 ms. These measurements support the complete
Pair engineering candidate. They do not isolate the contribution of branch
removal versus XOR fusion, scheduling, or the changed consumer call boundary.
The preflight records `perf_event_paranoid=4`; no branch-miss or cache-counter
attribution is supplied here.

The profile and main files used for these limited contextual checks have
SHA-256 `02089045490d6c1acd1b89a35a19926e1595245045a174e077f5712058ebfec0`
and `bf1b297e6204b0d3e05aba57408cacb186bf33535c1236b1b9738bcf5e7639e3`,
respectively. They describe N26 screening, not the independent N30 run.
