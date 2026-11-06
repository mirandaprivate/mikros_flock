# Binary RAAFold PCS

This crate provides a non-hiding binary-field PCS with authenticated SwitchFold
generator reductions. Encoding is `Repeat4 -> P1 -> Acc -> P2 -> Acc` at every
level. The default distance `0.19` is an explicit assumption for the sampled
code, with query count `ceil(-lambda / log2(1 - 0.19 / 3))`; it is not a
finite-length distance certificate. The query budgets are 1060 at lambda=100
and 1357 at lambda=128, capped by the available number of rows.

On x86-64, field arithmetic uses direct PCLMUL multiplication and four-lane
AVX-512/VPCLMUL dot products and scaled additions when available. Other CPUs
retain portable fallbacks. Projection and folding operate on contiguous tiles;
large scans and power tables are parallelized without changing proof bytes.
Merkle leaves hash contiguous field bytes in one update on little-endian hosts;
the hash function, row framing, authentication paths, and roots are unchanged.
Equality folds use a single multiplication per element, encoding alternates
between two buffers without a final matrix copy, and relation proofs consume
owned field buffers without cloning their contents.

`raafold_short_pc::ShortRaaFoldPC` exposes `setup`, `commit_values`,
`commit_packed`, `open`, and `verify`. A complete proof includes Opening and
recursive Decide; `ShortRaaFoldProof::{to_bytes,from_bytes}` encode the proof.
Commitments support serde serialization. The verifier uses only public
parameters, the commitment, point, claimed value, and proof.

`open` and `verify` use the ordinary Boolean MLE of the flat logical input
array with **MSB-first** coordinates. Boolean points select the corresponding
array entry. Values are interpreted as raw POLYVAL field representations.
Mikros GHASH elements require the explicit field conversion in `ghash` and
its low-first opening points must be reversed after conversion.

For an owned vector of full-width raw field elements, avoid a copy by using
`ShortRaaPackedWitness::from_packed_words` and `commit_packed`; the shape is
`ShortRaaPackedShape::new(n / columns, columns, 128)`.

Run the unit tests from the repository root:

```sh
RAYON_NUM_THREADS=4 cargo test -p mikros-raafold --lib -- --test-threads=1
RAYON_NUM_THREADS=4 cargo test -p mikros-raafold --lib --release \
  full_recursive_boolean_mle_opening -- --ignored --test-threads=1
```

The second test crosses the recursive-tail threshold and requires more memory
than the ordinary suite. See `PROVENANCE.md` for source and protocol changes.

For deterministic complete-PCS comparisons, including proof digests:

```sh
RUSTFLAGS="-C target-cpu=native" cargo run --profile bench \
  -p mikros-raafold --example opt_compare -- \
  --log-words 22 --threads 64 --reps 3 --seed 97
```

Set `RAAFOLD_PROFILE_STAGES=1` for diagnostic stage timings; unset it for formal
timing runs. Compare commitment roots, values, and full-proof digests across
implementations before comparing their timings.

For arithmetic-only comparisons against the original POLYVAL implementation:

```sh
RUSTFLAGS="-C target-cpu=native" cargo run --profile bench \
  -p mikros-raafold --example field_compare -- \
  --log-elements 22 --reps 5 --threads 1,64 --chunk 4096
```

Both examples use the bench profile to match the PLONK experiment's LTO and
code-generation settings. Arithmetic timings exclude allocation and output
validation; complete-PCS and PLONK timings include their full proof routines.
