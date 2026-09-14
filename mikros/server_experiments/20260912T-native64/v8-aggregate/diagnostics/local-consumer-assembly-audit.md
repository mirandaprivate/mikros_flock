# Local v8 consumer assembly audit

This is **local analysis of downloaded server evidence**, not a server-generated artifact or a new measurement. It was written on 2026-09-12. No frozen source or downloaded assembly was edited and no server command was run for this audit. References use one-based lines in the neighboring assembly files, including objdump's continuation-byte lines.

`assembly_manifest.json:11-12` identifies the executable and its SHA-256: `c26172d4caa709fa3ee43a5b077757e56378702afe992c08d2b360daa9bc287f`. Locally checked assembly text hashes:

| File | SHA-256 |
|---|---|
| `function-4.asm` | `297d88555b7b431484464dbb2bced0a5d3c9baf9e8c06575096ffb54d4964316` |
| `function-7.asm` | `944341c7c24ebb7179f566a151c6fabf1b0a8bb71b7157049bf02fc263d90b79` |
| `function-9.asm` | `b19e5bed335220be5080d421fc47de6de74e123e7f848b4254a5967b7483c6a4` |

## Actual consumer placement and row loops

The eight-coordinate consumer bodies occur directly inside the aggregate closure in `function-4.asm`. The loops below do not call out to separate `consume_p_eight`, `consume_h_eight`, or `consume_half_eight` functions. The source's `#[inline(never)]` therefore cannot substitute for checking the generated call boundaries. `consume_h_affine` does retain a separate body at `0x67fca0` (`function-9.asm`); for example, `function-4.asm:1381` calls it before the continuation loops.

| Actual loop in `function-4.asm` | Work per row / loop traversal | Accumulator placement inside row loop |
|---|---|---|
| P8: lines 1443-1490, `0x66bdf0` | 8 coordinate broadcasts, two 64-byte product loads, 16 GFNI multiplies | Sixteen ZMM accumulator values; no stores and no accumulator reloads in the row loop |
| P8 second coordinate group: lines 1561-1608, `0x66c020` | Same work for coordinates 8-15 | Same |
| H8: lines 2150-2203, `0x66cb10` | 8 coordinate broadcasts, two 8-byte mask loads, 16 masked moves | Sixteen ZMM accumulator values; no stores in the row loop |
| Split P8: lines 2253-2300, `0x66cd50` | 8 broadcasts, two 64-byte product loads, 16 GFNI multiplies | Sixteen ZMM accumulator values; no stores in the row loop |
| Half8 first half: lines 4485-4538, `0x66f6a0` | 8 broadcasts, one 8-byte mask load, one 64-byte product load, 8 GFNI multiplies and 8 masked moves | Sixteen ZMM accumulator values; no stores in the row loop |
| Half8 second half: lines 4588-4641, `0x66f8e0` | Same work for the second sample half | Same |

The increments and backward branches at the end of each cited range identify actual row iterations. For example, P8 advances its product pointer by 128 bytes and coordinates by 16 bytes at lines 1487-1490. The P8 accumulators at that site are `zmm0` through `zmm15`; `zmm16` through `zmm19` hold broadcasts, products, and multiply temporaries. Accumulator stores at lines 1491-1521 and loads at lines 1523-1553 are **between coordinate-group loops**, not per-row spills. Additional P8 copies at lines 2762-2809 and 2880-2927 have the same 16-multiply, eight-broadcast, store-free row body.

H8 stores its results only after the row loop (`function-4.asm:2208-2223`). Half8 similarly writes sixteen vectors at lines 4543-4558 after the first-half loop, then loads the second-half accumulators at lines 4559-4574. Stack-relative product/mask addresses in these loops refer to the deliberately cached row tile, not spilled sums. The four-coordinate baseline remains register-resident in `function-7.asm:42-88`, with two product loads, two mask loads and four broadcasts per traversal.

Consequently the observed eight-coordinate schedules halve continuation-product input reads across all sixteen coordinates: four 128-byte reads per row in the baseline become two 128-byte reads. Mask input reads fall from 64 to 32 bytes per row, while coordinate broadcasts double from 16 to 32 for Split8/Half8. These are **logical operand counts in the consumption stage**. They exclude producing the cached tile, basis conversion, accumulator boundary traffic and the remainder of proving; they are not a DRAM bandwidth estimate.

## H affine: hoisting succeeds, but 21 of 32 sums use spill slots

The main H-affine loop is `function-9.asm:58-346`, from `0x67fde0` to the backward `jne` at `0x6803b9`. Its index advances by 128 input bytes at lines 344-346: **one iteration processes eight rows**. All 32 H accumulator vectors are loaded from the caller's `sums` buffer before this loop (lines 8-54), and written back after it (lines 347-393). There is no `sums`-base (`rdi`) load or store inside the eight-row loop. Thus the source-level description “reload/store all H sums after every eight rows” is not the actual machine behavior.

However, lifting all 32 sums leaves insufficient registers for the two transposes and affine maps. Within every eight-row iteration, 21 distinct H accumulator components occupy private stack slots at some point. The exact 64-byte stack operands in that loop are:

| Stack offsets from this function's `rsp` | Meaning | Loads and stores per eight-row iteration |
|---|---|---|
| `0`, `0x40`, `0x100`, `0x140`, `0x180`, `0x1c0`, `0x200`, `0x240` | Eight H sums kept in spill slots between iterations | One load and one store per slot |
| `0x280` through `0x540`, step `0x40` | Twelve H sums temporarily saved around transpose/register pressure | One load and one store per slot |
| `0x80` | Remaining spilled H sum, including a redundant-looking reload/store sequence in the emitted code | Two loads and two stores |
| `0xc0` | Temporary transpose vectors, reused for mask and coordinate data; **not an H sum** | Two loads and two stores |

The temporary H saves are visible at lines 58-82; the corresponding restores include lines 170-188, 269, 317-337. The eight persistent spill-slot updates include lines 210-217, 231, 240, 249, 253, 262 and 282. The `0x80` accesses are lines 82, 337, 340 and 342. The separate transpose slot is written/read at lines 125/196 and 206/271. Eleven H components survive the full loop in registers, although the compiler renames some registers.

This gives **22 loads plus 22 stores of 64 bytes for H spills per eight rows**, or 352 bytes per input row. The transpose slot adds two loads plus two stores, or another 32 bytes per row. These counts come from the actual single-backedge loop, not from multiplying every instruction in the function by the row count.

For a full 64-row tile, with no scalar tail:

| H-related logical operands | Bytes per 64-row tile | Bytes per row |
|---|---:|---:|
| Caller `sums` buffer: 32 vector loads + 32 vector stores, once per call | 4096 | 64 |
| Private H-sum spill setup/final reads: eight stores + eight loads | 1024 | 16 |
| Private H-sum spills inside eight iterations | 22528 | 352 |
| **H sums and their spills together** | **27648** | **432** |
| Separate transpose temporary slot | 2048 | 32 |

The corresponding four-coordinate baseline touches each H accumulator at the tile boundary without H-sum spills inside its row loop, giving 64 bytes per row for H sums on a 64-row tile. The affine path therefore has substantial remaining spill traffic even though its external `sums` accesses were hoisted. The originally estimated 512 bytes per row from repeatedly reading/writing the source H array is not the measured assembly count and should not be reported as such.

Each affine eight-row iteration executes 36 GFNI affine instructions: four for the two mask/coordinate bit transposes and 32 for the H contributions. The source's temporary 16-entry matrix array is not emitted as a simple 128-byte store/reload round trip: the generated code uses register broadcasts/permutations, alongside the separate `0xc0` transpose spill. The scalar tail starts at line 394 and is excluded from the full-tile arithmetic above.

## Scope and next opportunity

The assembly confirms that P8/H8/Half8 meet their register-accumulation design inside the row loops. H affine trades many row-wise masked operations for fewer affine operations per row, but suffers from making all 32 H accumulators live around the transposes. A future bounded experiment could limit that live set while reusing the already prepared subset bytes and coordinate matrices; its benefit is unmeasured. This audit changes no kernel and makes no timing claim. All byte counts are explicit architectural operands, not cache misses, cache-line transfers, or memory-controller traffic.
