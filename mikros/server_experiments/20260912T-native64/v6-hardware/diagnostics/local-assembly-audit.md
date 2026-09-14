# Local audit of downloaded v5/v6 server assembly

Prepared locally on 2026-09-12. This file is an analyst note, **not a downloaded
server result**. No frozen source or server state was changed. The N30 timing
experiment was still running when this note was written.

## Evidence and limits

The executable identities come from `assembly_manifest.json`:

- v5 SHA256: `b08be2b84e6e1ce1b6f6b592b89a1b4fbacd3658e9cd1b3ba005918ec733a49f`
- v6 SHA256: `74b8a341c3c845d54f5f654ee66b57b4afff598c10a02479af2a485077001b28`

Locally computed SHA256 of the inspected text files:

- `v5-function-1.asm`: `df76f213d77b8aa1ff59d3af5f3f0000a05cbcfd16fa31e878de21026d2913a8`
- `v6-function-1.asm`: `d6ea56a2cc3ca4877afab9cf533810203fbbce286c7772307fb2f03bf7fd7a0f`
- `v6-function-2.asm`: `1ce58adb211f0a186ada4f0cd917b4013bfcb07179b3a84c547d06a09125eefc`
- `assembly_manifest.json`: `2333a9bc213b7d0c18f1661ce65f9de13c7001ebb6ac4056e3b1d4231f6f48e3`

The line references below are one-based lines of these exact text files.
Counts multiply assembly instructions by their established loop trip counts.
They describe memory-operand bytes and instructions, **not** measured cache
misses, DRAM bandwidth, retired hardware events, or elapsed-time attribution.
`perf-access.txt` records that hardware-counter access failed with
`perf_event_paranoid=4`.

## Accumulator residency and traffic

**v5:** `v5-function-1.asm:888-917`, addresses `64df60..64e05f`, updates two
coordinates per iteration. There are eight memory-source ZMM XORs and eight
ZMM stores; `rcx += 0x200` until `rcx == 0x1000` establishes eight iterations
per input row. Thus the actual accumulator instructions read 4096 bytes and
write 4096 bytes per input row. Four GFNI multiplications in this loop execute
eight times; the two AB products at lines 877-878 give 34 GFNI multiplications
per row in total. The two byte broadcasts per iteration read 16 coordinate
bytes per input row.

**v6:** `v6-function-2.asm:38-80`, addresses `667890..667992`, is the hot row
loop of `consume_four`. Its sixteen accumulators are `zmm0..zmm15`; temporary
values use `zmm16..zmm19`. There are **no stores, no stack operands, and no
accumulator reloads inside this loop**. Each input row in one call performs:

| Operation | Observed count |
|---|---:|
| Product loads | 2 x 64 bytes, lines 45-46 |
| H-mask loads | 2 x 8 bytes, lines 39-40 |
| Coordinate byte broadcasts | 4 x 1 byte, lines 38, 51, 60, 69 |
| GFNI products | 8 instructions |
| Masked moves for binary H samples | 8 instructions |
| Register-only accumulator XORs | 16 instructions |

The sixteen accumulator loads occur before the loop (lines 8-33), and the
sixteen stores occur after it (lines 81-96). The parent really calls this
function four times with coordinate offsets 0, 4, 8, 12:
`v6-function-1.asm:1083-1125`, calls at lines 1093, 1105, 1115, 1125.
The parent caps a tile at 64 rows (`:1142-1144`, `:1183-1190`). For a full tile:

| Selected traffic component | v5 bytes/input row | v6 bytes/input row |
|---|---:|---:|
| Accumulator reads + writes | 8192 | 128 |
| Product-buffer write + four consumer reads | 0 | 128 + 512 |
| Mask-buffer write + four consumer reads | 0 | 16 + 64 |
| Coordinate-buffer write + consumer reads | 32 | 32 |
| Sum of these selected components | 8224 | 880 |

This table excludes LDE-table reads, weight conversion internals, input reads,
loop/ABI bookkeeping, initialization, final transposition, and Rayon reduction.
The zero product/mask entries in v5 mean that these values stay in registers
between their production and the coordinate loop; they do not mean that v5
does not read its witness inputs. The coordinate row counts only the meaningful
16-byte materialization and the 16 byte broadcasts, excluding the adjacent
zero-initialization store in v5 at line 881.

For N=2^30 there are 2^23 input rows. Accumulator memory-operand volume alone
therefore changes from **64 GiB to 1 GiB**. Including the selected temporary
buffer components gives 64.25 GiB versus 6.875 GiB. These are instruction-level
logical byte volumes, not DRAM transfers or a predicted speedup. A partial tile
of T rows amortizes accumulator traffic as 8192/T bytes per row.

The arithmetic has not been reduced by tiling: four calls x eight GFNI products
per row plus two AB products still gives **34 GFNI multiplications per row**.
The improvement established by assembly is register reuse and loop organization.

## LDE fusion is present in machine code

The v6 tile producer computes the two continuations in registers in
`v6-function-1.asm:1751-1912`. The first continuation remains in `zmm0,zmm1`
while the second is generated into `zmm2,zmm3`. Both pass directly to the
two GFNI AB products at lines 1742 and 1744, which write only the resulting
128-byte product tile at lines 1743 and 1745. There are no A/B continuation
array stores in those two inlined LDE loops. The H masks are computed directly
with an XMM AND and one 16-byte store at lines 1756-1759.

Each LDE reads two 64-byte vectors for each of sixteen input bytes, so the
two LDEs read 4096 bytes from the 32 KiB continuation table per input row.
That count excludes permutation-constant and jump-table accesses visible in
the same loops. The extra continuation lane for point 255 stays independent
and is excluded from the final 255-sample output.

In v5, **do not count the displayed SSE fallback as the normal LDE path**:
`v5-function-1.asm:519-520` branches on ell=128 to lines 637-644. That branch
calls the separate AVX-512 continuation function through the GOT; the second
call appears at lines 781-788. Its callee body is not in the supplied dumps.
The parent does visibly initialize the 128-byte continuation buffer twice,
copy the first continuation at lines 654-657, and later reload both results
for multiplication at lines 873-878. This establishes elimination of those
specific intermediates in v6, but does not establish a complete v5 LDE traffic
count without the callee disassembly.

## Batched GFNI basis conversion is real, with remaining stack traffic

`v6-function-1.asm:1186-1190` distinguishes the batched-basis option and checks
for a complete 64-row batch. The full-batch path has the following machine code:

1. Sixteen 64-byte loads of GHASH values at lines 1195-1210.
2. A byte-plane transpose at lines 1211-1289: **64 `vpunpck*` instructions**.
3. Fixed matrix words loaded and broadcast into GFNI affine operations at
   lines 1319-1415. The affine immediate is zero. This is an actual binary
   basis map, not interpreting raw GHASH bytes as GF256 coordinates.
4. Sixteen converted byte planes are materialized by the output loop at
   lines 1315-1318.
5. Sixteen converted-plane loads at lines 1521-1536, another **64 `vpunpck*`**
   transpose instructions at lines 1537-1600, and sixteen final stores at
   lines 1601-1616, returning to consecutive 16-byte coordinate rows.

The matrix loop advances `rax` by 64 through 1024, so the sixteen affine
instruction sites in the text are repeated over sixteen output coordinates.
They are not sixteen affine instructions per batch. There are 256 matrix
slots with zero-matrix branches. Independently reconstructing the public
128-bit basis inverse from `PHI_8_TABLE` and eta_j=x^j gives fifteen nonzero
matrix blocks for every output coordinate, hence **240 executed affine
instructions per full 64-row batch**, or 3.75 per input row. This last exact
count combines observed control flow with the fixed public matrices; it is
not a sampled hardware-counter result. Matrix-word loads total 2048 bytes
per batch, versus 16384 bytes of scalar LUT loads for 64 rows. The old scalar
path has sixteen 128-bit LUT operands at `v5-function-1.asm:800-866`.

The batch conversion is **not entirely register-resident**. Its input planes
are stored at lines 1260-1290 before a call with a 1024-byte zeroing range
at lines 1291-1295 (source initialization of `converted`). Fifteen planes
are reloaded at lines 1296-1310. The remaining plane is reloaded at line 1412
for each of sixteen output coordinates; its matrix is nonzero in all sixteen
cases. Converted planes are also stored and subsequently reloaded before the
inverse transpose. This is a concrete future cleanup opportunity if the
basis variant benefits from further work; the frozen v6 results include it.

Neither the full basis routine nor the entire aggregation may be described
as having no memory traffic. The precise supported statement is that the
**four-coordinate accumulation hot loop has no accumulator spill/reload**.
Any speedup conclusion still requires the paired N30 benchmark results.
