# Completed v2 streaming experiment (2026-09-12)

User request: 再继续优化. This isolated permutation optimization and server
experiment are complete. Earlier monitoring instructions are superseded.
Do not restart the driver, rerun one-time adapters, or resume another task.

## Results

Final report: server_experiments/20260912T103800-permutation-v2/REPORT.md.
Raw samples, actual proof files, environment, logs and validation records are
in the same directory. Results archive and all 174 downloaded files passed
checksum validation. The summary validates 166 current proofs and 90 v1
 overlapping proof files. All 14 server cases succeeded without detected
interference from monitored benchmark/build processes. Driver PID3522044
completed at 2026-09-12 11:08:54 UTC.

N30 / 48 physical cores, median of five paired formal inputs:
- v1 compact: core 14.263 s, including conversion 16.628 s, peak 97.491 GiB.
- v2 compact: core 11.062 s, including conversion 13.065 s, peak 67.493 GiB.
- v1 full: core 11.214 s, including conversion 13.566 s, peak 97.488 GiB.
- v2 full: core 11.479 s, including conversion 13.453 s, peak 83.490 GiB.
- Flock: core 11.906 s, including conversion 13.124 s, peak 158.005 GiB.

Memory comes from separate cold processes. Source authentication is separately
timed. No PCS, commitments, openings or grinding. Full variant saves memory
but slightly regresses in core time. Compact and Flock total timings are close;
do not claim a decisive total-time win from this small difference.

## Scope and checks

Only streaming/ candidate and its support artifacts were modified. Previous
optimized/ Rust and snapshot/ were preserved and matched the frozen manifest.
No parent Hadamard edits, commits, subagents, or other server jobs were used.

Compact evaluation uses <=16 KiB blocks, tree layers are synchronously released
inside the timer, and leaf tables use actual logical bits. Original cubic GKR
proof bytes, claims, transcript and verifier are unchanged. See ENGINEERING.md.

Local native differential tests and Clippy passed. Server native AVX-512 and
VPCLMUL differential tests passed. Toolchain nightly-2026-03-21, native flags.

Source archive SHA256:
efec03193f5775e22899f92b171aa298d4443e66f7f0a583ab0ca764ea0299bc
Results archive SHA256:
a36fdc2ff534f3a2b84afd5c6df1251d742a7927784fa0be23e05191add6a9ee
Server binary SHA256:
0f7ebe2a24ddbcb808be0d2f8d78a03ff1e7a6a83bb1851ad727edc139df0931

No additional benchmarks are needed for this completed request. Do not rerun
prepare_streaming.py, adapt_entry.py, adapt_benchmark.py, specialize_dense.py,
prepare_driver.py, or source packaging into the existing destination.
