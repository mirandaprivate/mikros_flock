# Multiple-instance R1CS with packed sparse matrix authentication

The main 256-SHA example `e2e_sha256_paired` now uses public CSC and the
native C=I encoding, as requested for the FLOCK comparison. The equivalent
explicit entry is `e2e_sha256_direct_lincheck`. The default `--reduction column`
follows the updated Packed R1CS column projection: two separate Hadamards
share row challenges, then A/B each fix both column messages before the fresh
block/coordinate projections and one ordinary F128 IP. Public CSC checks each
matrix endpoint directly, replacing the manuscript's second IP and SPARK.
Both C=I queries retain their own column weights and reach the same witness
PCS. This mode has six paired or eight separate source claims, with one native
PCS opening; explicit F256 pairing covers public I/O and the constant pin.
Use `--reduction legacy` to reproduce the earlier weighted-residual and packed
extension-IP reduction. See the root column-projection comparison report;
the earlier public-CSC report records the frozen legacy snapshot.

The engineering defaults preserve the complete column-mode proof and transcript:
single-side public CSC, blocked arbitrary-pair row tensors, GFNI quotient
interpolation (portable fallback), joint Hadamard aggregation with 2048-row
tasks, and single-column PCS low-bit support plus unit-table reuse. Wide
packing and parallel batch IP remain optional. The small SHA carrier is
deinterleaved in one sequential pass, still inside online time; carriers above
131072 F128 words retain the parallel default.

Set each switch explicitly to `0` to recover the engineering reference:
`MIKROS_PUBLIC_CSC_SINGLE`, `MIKROS_IMAGE_BLOCKED_WEIGHTS`,
`MIKROS_HADAMARD_GFNI_QUOTIENT`, `MIKROS_HADAMARD_GROUP_TILES`,
`MIKROS_PCS_LOW_BIT_SUPPORT`, `MIKROS_PCS_REUSE_UNIT_TABLE`, and
`MIKROS_SERIAL_DEINTERLEAVE`. `MIKROS_HADAMARD_WIDE_PACK` and
`MIKROS_IP_BATCH_PARALLEL` default to `0`; set them to `1` for ablation.
`MIKROS_HADAMARD_TILE_ROWS=512|1024|2048` applies only to joint aggregation.
`MIKROS_PROFILE=1` prints nested phase timings on stderr; disable it for formal
timing. The native-64 runner `run_engineering.py` records every switch, verifies
frozen proof hashes for both systems, and keeps profiling, ablations and final
comparisons in separate directories. See the root engineering comparison report.

The separate `e2e_sha256_sparse_shared` example retains the lookup construction
described below. It follows the R1CS
construction in the active subsection of `latex/6-together.tex`, including
lookup authentication of all three sparse matrices. Its timings must be distinguished from the public-CSC comparison, which
authenticates public matrices directly and does not include this lookup protocol. The original independent-block sparse
composition is retained as `e2e_sha256_sparse_independent`.

For the two blocks of 128 instances:

1. Run separate binary packed Hadamard relations for the three virtual images
   `S_L W`, `S_R W`, `S_O W`. Their row challenges and IP folding challenges
   are shared; column challenges remain independent. Bind both blocks'
   round messages before sampling the common challenge. No separate image
   commitments are used and products never mix instances.
2. For each matrix, fix both blocks' 128 column messages, check each source
   claim, then sample the random linear-combination weights and fresh
   coordinate projection. Project each message separately before combining:
   bit unpacking is only F2-linear and cannot be interchanged with an F128
   weighted combination.
3. Use two F128 inner-product reductions per matrix to obtain a packed-assignment
   opening and a matrix MLE claim. The public coordinate projection terminal
   is evaluated by Frobenius tensor recursion, without scanning the matrix.
4. Authenticate the matrix MLE with the sparse lookup below. This includes
   `S_O` even when it is the identity matrix.

The shared row queries allow each same-matrix pair of linear image claims
to be combined according to the manuscript's larger-batch construction.
There are three matrix-evaluation proofs and six indexed memory arguments,
instead of six and twelve in the retained independent-block composition.
Static matrix commitments are reused across proofs.

`sparse_lookup::Prepared` canonically enumerates the nonzero coordinates of
each binary matrix, cancelling repeated positions over GF(2). Its static
columns are `row`, `col`, `value`, four read/write timestamp columns, and two
final timestamp columns. Time increments are computed as integers during
public preprocessing. They are not additions of `F128::ONE`.

After the row/column evaluation points are fixed, the prover forms two
F128 equality-table gather vectors and commits them before any lookup
compression challenge. A cubic sumcheck proves

```text
matrix_value = sum(value[i] * read_row[i] * read_col[i]).
```

For each gather, two product trees prove the indexed memory multiset identity
`Init union Write = Read union Final`. Tree terminals authenticate the fixed
addresses and timestamps and the online read values. Equality-table and
identity-address terminals are evaluated by tensor formulas. Every returned
claim must be passed to PCS; `sparse_lookup::check_claims` is only a test oracle.

`packed_sources::Layout` packs small values into native F128 words: eight
16-bit positions, four 32-bit timestamps, or 128 binary coefficients per word.
Widths round up to powers of two for tensor-compatible placement. It converts
logical numeric-column MLEs into bit-linear native PCS queries, including
the in-word weights and the column block prefixes.

The complete example authenticates three groups through the existing FLOCK
Ligerito Fast backend:

- Witness: two interleaved F128 vectors viewed as one logical F256 vector.
  The two coordinates are fixed before the pairing challenge. This does not
  halve stored bytes.
- Static circuit data: one packed commitment constructed offline and reused.
- Online reads: one commitment created after the matrix query points are
  known, with its full construction and opening cost included in prover time.

The shared composition has 33 static and 18 read evaluation claims. Witness
authentication has five logical F256 claims, or ten separate F128 claims in
the comparison mode. These remain three native PCS openings in either mode;
logical pairing does not combine independent PCS commitments.

The verifier's key contains dimensions, commitments, layouts and public I/O.
It does not contain the sparse matrices or the gathered vectors. All online
reductions, commitments, queries and openings are verified; no direct matrix
evaluation substitutes for a lookup proof.

The Mikros SHA encoding retains 48 message-schedule words, 64 T1 words and
64 additional E/A state words in 5632 formerly unused witness slots. It uses
31102 of 32768 slots. Their new R1CS rows certify their definitions. It keeps
the 32768-row dimension and original public I/O and constant-pin positions.
Native FLOCK retains its original inlined encoding. These encode the same
SHA compression statement but have different matrix commitments; benchmark
reports must distinguish this representation change from PCS packing gains.
The original materialized sparse encoding has 777,672 nonzero coefficients
(A: 535,556; B: 209,348; C: 32,768), compared with 47,399,406 in native FLOCK's
inlined representation. The current compact encoding additionally stores
actual carries and Ch/Maj outputs in the existing auxiliary slots. For example, it certifies
`(x+c)(y+c) = carry_out+c`, moving the correction into C. C is consequently
nonidentity and still receives a complete lookup proof. Public I/O and pin
positions remain fixed; internal witness values and the matrix digest change.
Set `MIKROS_SHA_LEGACY_CARRIES=1` to select the original materialized encoding.
`MIKROS_SHA_CARRY_ONLY=1` keeps actual carries but restores product-slot Ch/Maj.
The default compact nonzero counts are A=157,970, B=63,955 and C=54,917;
lookup domains are respectively 2^18, 2^16 and 2^16.
These are representation-size changes, not measured timing ratios; see the
root online comparison report for complete measurements.

Online implementation switches:

- `MIKROS_NO_IMAGE_FUSION=1`: separate IP self-check/initial-message scans.
- `MIKROS_NO_SPARSE_LOOKUP_FUSION=1`: reference sparse sumcheck/endpoint kernels.
- `MIKROS_SPARSE_CACHE_LEAVES=1`: build fingerprint caches after challenges;
  allocation, construction and release are included in online time.
- `MIKROS_NO_SPARSE_CACHE_BASE_REUSE=1`: disable checked base reuse inside that cache.
- `FLOCK_NO_SPARSE_PREFIX_WEIGHTS=1`: build full-domain PCS weight tables.
- `FLOCK_NO_ZERO_WEIGHT_BLOCK_SKIP=1`: process zero-weight ring-switch blocks.
- `FLOCK_NO_SPARSE_SCALED_EQ=1`: independently construct and scale support weights.

The optimized kernels retain reference paths with proof/transcript equivalence
tests. Changes to batch scheduling or circuit representation produce different
proofs and are validated by complete verification and negative tests instead.

Entry points:

- `src/multiple_image.rs`: column projection, two IPs, public tensor recursion.
- `src/sparse_lookup.rs`: deferred sparse-evaluation and indexed memory PIOP.
- `src/packed_sources.rs`: compact column layout and native PCS authentication.
- `examples/e2e_common/sparse_sha_shared.rs`: current 256-instance composition.
- `examples/e2e_common/sparse_sha.rs`: retained independent-block composition.

The construction uses the memory-checking approach of
[SPARK in Spartan](https://iacr.org/archive/crypto2020/12171304/12171304.pdf),
with separately committed integer read/write timestamps for characteristic
two. Fixed metadata commitments assume correct public circuit preprocessing.
The example is not zero knowledge. The native PCS security profile alone
does not establish a strict 128-bit soundness theorem for this new composite
PIOP; its lookup fingerprint and sumcheck errors must also be counted.
