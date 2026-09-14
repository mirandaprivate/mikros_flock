# Local audit of v7 basis-conversion assembly

Prepared locally on 2026-09-12 from the downloaded server disassembly. This is
an analyst note, **not a downloaded server result or a timing measurement**.
No source snapshot or server state was changed. The N30 experiment was still
running when this note was written.

## Evidence

`assembly_manifest.json` identifies executable SHA256:

`3da890dca453354ffff72a039c9a3715b05b08f7b4a0dd7a2b65e12a80c572ea`

Locally computed input SHA256 values:

- `function-7.asm` (2915 lines):
  `6ff91c06789d675c39fe677e27df79cc7c9fe9b721b76bdd4e9b237e083e4c56`
- `assembly_manifest.json`:
  `d58b830a40f83513ab5be175cf6fea5f533c2035445dbfe6addcddc4213de442`

All following line references are one-based lines in this exact
`function-7.asm`. The same executable contains both the retained batch
implementation and the new directly constructed-plane implementation. This
permits comparison without relying on different binary layouts between v6
and v7. The earlier v6 findings are recorded in
`../../v6-hardware/diagnostics/local-assembly-audit.md`.

## The two branches

The conversion selector is stored at `[rsp+0x44]`: lines 989-995 establish
values 2, 1, and 0. These correspond to the source's Register, Batch, and
Lookup modes. Lines 1199-1209 inspect that selector:

- Batch, selector 1: jump at `658e8f` to `659080` (line 1306).
- Register, selector 2: jump at `658e9b` to `6595f0` (line 1536).
- Lookup, selector 0: the scalar path begins after that test.

Both batch entries check for 64 rows. The retained batch path branches to
`659cbc` for a shorter input (lines 1306-1307); the new path branches to
`65a121` (lines 1536-1537). Both destinations handle remaining rows through
the scalar coordinate tables.

## The targeted clearing call and input-plane spills are removed

The old batch path still exhibits the v6 pattern:

1. Sixteen input loads at lines 1308-1323.
2. Exactly 64 `vpunpck*` instructions at lines 1324-1403. Interleaved with
   their final stage are sixteen 64-byte input-plane stores to the stack,
   including the final store to `[rsp+0x400]` at line 1403.
3. Lines 1404-1408 request `memset(rsp+0x440, 0, 0x400)` and call the named
   `memset@GLIBC_2.2.5` target at `65930b`.
4. Fifteen input planes are reloaded at lines 1409-1423. The last input
   plane is instead reloaded at `6595ad`, line 1525, inside the coordinate
   loop whenever its matrix block is nonzero.

The new Register path has sixteen input loads at lines 1538-1553 and exactly
64 transpose instructions at lines 1554-1617. The final transpose results
are assigned directly to `zmm0` through `zmm15` at lines 1602-1617.
They remain the input-plane registers for all affine operations at
lines 1626-1727. There is **no function call, no input-plane spill store,
and no input-plane reload** between those input loads and completion of
the affine loop. In particular, there is no converted-array `memset` on
this path.

This confirms the intended machine-code change from constructing completed
planes with `array::from_fn`. It does not establish an elapsed-time gain.

## Remaining storage and unchanged arithmetic

The converted output is still materialized on the stack in both paths:

| Per full 64-row conversion | Retained batch | Register candidate |
|---|---|---|
| Input-plane spill stores | 16 x 64 bytes, lines 1373-1403 | None in the inspected path |
| Converted-array clearing | Requested 1024-byte `memset`, line 1408 | Removed |
| Input-plane reloads | 15 x 64 bytes before loop; another 64-byte load for each nonzero last-column block | None in the inspected path |
| Completed-plane stores | One 64-byte store x 16 loop iterations, line 1428 | One 64-byte store x 16 loop iterations, line 1622 |
| Completed-plane reloads | 16 x 64 bytes, lines 1728-1743 | 16 x 64 bytes, lines 1920-1935 |
| Final coordinate-row writes | 16 x 64 bytes, lines 1808-1823 | 16 x 64 bytes, lines 2000-2015 |

The output loops advance `rax` by 64 until 1024 (lines 1428-1431 and
1622-1625), establishing sixteen iterations. The new loop's only vector
stack store is its completed output plane at line 1622; its input planes
are held in registers. There is no newly introduced input-plane spill.
The second transpose still uses 64 `vpunpck*` instructions in each branch
(lines 1744-1807 and 1936-1999).

Each branch has sixteen static GFNI-affine instruction sites inside that
sixteen-iteration coordinate loop, with the same zero-matrix branches.
Thus the 32 affine sites reported for this parent function belong to two
mutually exclusive implementations. They are not 32 operations executed
per batch. The fixed matrix computation and its zero-block skipping have
not been reduced by this change.

Other clearing remains: lines 1002-1014 initialize the worker's sums,
coordinate tile, and masks. These calls are before the tile loop and are
not the per-conversion clearing removed above. The candidate therefore
must not be described as a wholly stack-free or initialization-free
aggregation kernel.

These counts describe the displayed vector-memory operands and the requested
`memset` range. They are not cache misses, DRAM traffic, or hardware-counter
measurements. `preflight.json` records `perf_event_paranoid=4`; the paired
benchmark must determine whether this verified code change improves runtime.
