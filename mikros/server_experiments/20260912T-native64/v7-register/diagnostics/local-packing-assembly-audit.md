# Local analysis of v7 packing assembly

This file is a **local, read-only analysis of downloaded server evidence**, written on 2026-09-12. It is not a server-generated artifact or a new measurement. No source file or downloaded assembly was modified, and no server command was run for this audit. Line references below are one-based lines in the neighboring `.asm` files.

The recorded executable SHA-256 is `3da890dca453354ffff72a039c9a3715b05b08f7b4a0dd7a2b65e12a80c572ea` (`assembly_manifest.json:2-3`). This is the executable hash, not a source/archive hash. Local SHA-256 checks of the inspected text files give:

| File | SHA-256 |
|---|---|
| `function-13.asm` | `ceb894f5338c1135fda5789471c17002ca871ce49b5a85fb9681d0a259d4178b` |
| `function-14.asm` | `744bc8948c8fc5bb86b91fe93d7e275ea6b96047f817af903aebd15ef684a7ed` |
| `function-15.asm` | `9679fceffaca0621b3a594e12a1c7311170573d098c9021cd40d5d90007e26da` |
| `function-16.asm` | `0be1ece46b35a0de6b45a9726bd8d062c3fa71ba37564494e3fa0a52c7e8327a` |

## The intended persistence is present, with two accumulator spills

The new closure is `function-14.asm`, starting at `0x6650d0`. Its six wide components are initialized once at lines 62-67. At the multiply-accumulate site, the zero-message components are `zmm0`, `zmm4`, `zmm3`; the infinity-message components are `zmm2`, `zmm1`, `zmm27` (lines 110-125). These are the two three-component `WideGhashX4` values in the source, and they accumulate across the eight-row iterations rather than being reset inside each iteration.

Control flow confirms the placement: the row index advances by eight at lines 77-79; lines 134-135 exit the bulk loop only when its counter reaches zero. Otherwise execution continues through the packing body and returns via lines 826-827 to the weighting/accumulation site at line 74. The six horizontal lane folds occur **after that loop**, at lines 922-953 (`0x666468` through `0x66652e`). The short scalar tail follows them. The wrapper retains the same 1024-row parallel chunk size: `function-15.asm:84-106` and `function-16.asm:84-106` both construct chunks with `0x400` elements. Thus a full parallel task performs 128 eight-row iterations and one final fold of each wide accumulator.

It would be incorrect to call the entire new packing loop spill-free. During the unrolled A lookup expansion, the zero-message low/high components are explicitly saved with two 64-byte stores at `function-14.asm:691-692` (`[rsp+0x110]`, `[rsp+0x150]`) and restored at lines 793-794. This happens once per full eight-row group on the normal 16-byte-row path: **128 bytes stored plus 128 bytes loaded per group**, or 32 bytes of logical stack operands per row. The other four wide components survive in registers; some move between registers at lines 684-686 and 790-792. These spills preserve unreduced sums; they do not reintroduce per-group horizontal folds.

## The observed output and temporary stores

In the new bulk loop, B weighting is performed in ZMM registers at `function-14.asm:81-105`; the resulting weighted B registers feed the message arithmetic at lines 108-125. Four 64-byte output stores at lines 127-131 write packed A and weighted B, totaling **256 bytes per eight rows**, exactly the required two arrays. There is no intervening store/reload of weighted B and no call in the normal bulk path. Pair separation has been lowered to four `vshufi64x2` instructions (lines 106-109), rather than general variable permutations.

The compiler did not eliminate every packing temporary. The unweighted B values are written one row at a time to `[rsp+index+0x190]` at line 823, repeated eight times by lines 824-827, then read as two ZMM values at lines 74-75. A packing also has partial-result spills at lines 602-603 and 679-681, consumed at lines 778-782. These are separate from the two wide-sum spills above. Both old and new closures allocate `0x218` bytes of stack in their prologue (line 8 in each file); the new source-level arrays therefore must not be described as completely absent from the generated code.

For comparison, the old closure `function-13.asm` calls `inner_product::native_message_chunk` at line 254 **inside each eight-row iteration** (the loop advances by eight at lines 44-50 and returns at line 404). Its weighted B vectors are explicitly stored back to stack at lines 102 and 119 before that call. Reduced-width chunk-result values are spilled at lines 120-121 and reloaded/XORed with the callee's results at lines 255-258. Output is copied with individual 16-byte row stores in lines 270-403. The new code therefore removes the old per-group call and weighted-B stack round trip and emits wider output stores, in addition to moving the horizontal folding to the task boundary.

The old callee itself is not among the four assembly files audited here. The call frequency and caller-side traffic are direct assembly observations; the callee's precise dynamic instruction or memory count is not claimed. Likewise, the 621-versus-1300 text-line count measures neither executed work nor latency: both dumps contain bounds checks, branches, and tail paths.

## Interpretation boundary

The machine code supports the specific optimization claim: persistent unreduced message accumulation, one horizontal fold per parallel chunk, and direct use/output of weighted B. It also identifies a remaining implementation cost: the A lookup expansion creates enough register pressure to spill two accumulator components. All byte figures above count explicit logical operands in the cited path, not cache misses, L1 transactions, or DRAM traffic. This audit does not infer a speedup from assembly or extrapolate the N26 screening result to N30; timings and proof-equivalence results remain separate benchmark evidence.
