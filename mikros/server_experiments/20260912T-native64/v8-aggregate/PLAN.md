# v8 first-aggregation engineering experiment

Only the first aggregation changes. The v6 Batch field-basis conversion,
original A/B packing, C projection, IP, transcript, proof format, and GF256
domains remain identical. Existing kernels and historical snapshots stay intact.

Candidates in one executable:

| system | first-aggregation schedule |
|---|---|
| basisreuse | unchanged v6 four-coordinate consumer |
| splitreuse | separate H and continuation, eight coordinates per loop |
| halfreuse | H/continuation halves, eight coordinates per loop |
| pairreuse | fixed-index paired interpolation, original consumer |
| affinereuse | split consumer with eight-row GFNI affine H accumulation |
| jointreuse | paired interpolation plus affine H and continuation8 |
| flock128 | unchanged Flock skip-7, kappa=128 |

Native server: 64 threads, fixed CPU affinity `0-47,48-55,72-79`, shared
48-physical-core / 96-logical-CPU host, Linux default first-touch. No NUMA
policy changes, privileged profiling, or exclusive-CPU assumptions.

Screen N26: 14 main measurements per system, rotated positions balanced twice;
separate seven-run profile group, no profile samples pooled with main timing.
Then select the candidate and measure N30 against basisreuse and Flock128
using the same frozen executable: nine main runs, separate three-run profile,
and independent one-proof RSS processes. If multiple candidates remain
materially different, record any revised comparison before starting that group.

Correctness gates: native tests including all new kernels, basis and subset
coverage, partial tiles, nonzero sums, input alignment, threading, proof bytes,
claims, and transcript. Every experiment proof receives independent original
input checks outside proving time. Retain all results including regressions.

Report main medians, means, ranges, per-input paired ratios and win counts.
Report aggregate stage times and ratios separately from the independent
profile process. A lower instruction count is not a measured speedup.
Affinereuse deliberately trades fewer mask operations for more H sum access;
only actual machine code and timing can decide whether this trade is useful.

The affine matrix byte order follows Intel SDM GF2P8AFFINEQB:
[Intel instruction reference](https://cdrdv2-public.intel.com/835757/325383-sdm-vol-2abcd.pdf).
The scalar bitwise oracle and full-proof checks independently validate the
implementation's use of that convention.

## Selection recorded before N30

N26 complete: pairreuse and jointreuse are both materially faster than the
basisreuse baseline in this batch, but their mean/median ordering differs.
The independent N30 group therefore retains both finalists: basisreuse,
pairreuse, jointreuse, flock128. Eight main runs give two balanced rotations;
the separate profile process has four runs per system, and each system has
one independent RSS process. This decision and its server UTC precede launch
and are preserved in selection.json. No source or executable changed.
