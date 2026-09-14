# Default packed-permutation selection

2026-09-14: after the user's request to adopt the optimized version, the E2E
executable now defaults to `--permutation packed --reduction lockstep`.
`--system mikros` therefore selects `mikros_packed`; explicit `legacy` and `both`
remain available. Frozen server snapshots and their results are unchanged.

The release executable was rebuilt locally with nightly-2026-03-21, offline and
locked. A four-thread, log-leaves=3 run without `--permutation` verified the packed
proof and rejected all 30 packed tamper cases. A separate explicit
`--permutation legacy --reduction cube` run also verified. Commands and full
outputs are recorded alongside this file. These are correctness smoke checks,
not additional server performance measurements.
