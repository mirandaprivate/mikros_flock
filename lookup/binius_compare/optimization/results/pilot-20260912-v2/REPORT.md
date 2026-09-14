# Mikros permutation + Binius: ordered optimization

Suite: pilot. Unit: ms, measured medians. Same raw bit commitments and PCS policy.
Reference is the instrumented original path; Flock/reference retains the original adapter.
Flock/bridge uses the same optimized bit conversion, byte embedding and column evaluator as Mikros.

| log N | bits | threads | system / mode | prepare | commit | PIOP | bridge | PCS open | total | verify | proof KiB |
|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 16 | 8 | 1 | flock/bridge | 2.131 | 0.838 | 10.487 | 0.268 | 2.818 | 16.584 | 4.538 | 353.81 |
| 16 | 8 | 1 | flock/reference | 6.294 | 0.875 | 11.963 | 7.542 | 2.905 | 29.618 | 4.748 | 353.81 |
| 16 | 8 | 1 | mikros/batched | 2.846 | 0.862 | 14.497 | 9.512 | 3.193 | 30.953 | 5.353 | 353.85 |
| 16 | 8 | 1 | mikros/bridge | 2.875 | 0.892 | 14.013 | 1.108 | 3.091 | 22.023 | 4.385 | 353.85 |
| 16 | 8 | 1 | mikros/direct | 4.135 | 0.877 | 39.483 | 10.715 | 3.310 | 58.667 | 5.332 | 361.48 |
| 16 | 8 | 1 | mikros/full_extension | 24.380 | 1.226 | 24.701 | 9.861 | 3.230 | 63.452 | 5.954 | 361.48 |
| 16 | 8 | 1 | mikros/full_optimized | 4.233 | 0.851 | 11.297 | 1.141 | 3.101 | 20.661 | 4.535 | 353.85 |
| 16 | 8 | 1 | mikros/fused | 3.161 | 0.861 | 15.841 | 9.687 | 3.169 | 32.764 | 5.529 | 361.48 |
| 16 | 8 | 1 | mikros/lookup | 3.122 | 0.872 | 20.355 | 11.258 | 3.373 | 39.102 | 5.328 | 361.48 |
| 16 | 8 | 1 | mikros/packed | 3.011 | 0.925 | 27.446 | 1.054 | 2.442 | 34.927 | 4.105 | 354.44 |
| 16 | 8 | 1 | mikros/reference | 20.648 | 0.908 | 48.421 | 9.704 | 2.860 | 82.590 | 5.668 | 361.48 |

## PIOP phase attribution

Phase medians are independently calculated and need not sum to the total median.
Fused message/fold timing is a single combined phase, not separately estimated.

| log N | bits | threads | mode | checks | decode | compact tree | coefficient eval | extension tree | materialize | eq | messages | fold | fused fold/message | packed quotient | packed projection | packed columns | skipped rounds |
|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 16 | 8 | 1 | batched | 0.000 | 0.483 | 3.257 | 0.508 | 0.270 | 3.148 | 0.093 | 1.729 | 0.000 | 4.684 | 0.000 | 0.000 | 0.000 | 0.000 |
| 16 | 8 | 1 | bridge | 0.000 | 0.469 | 3.275 | 0.632 | 0.278 | 2.625 | 0.059 | 1.738 | 0.000 | 4.620 | 0.000 | 0.000 | 0.000 | 0.000 |
| 16 | 8 | 1 | direct | 0.002 | 0.274 | 10.752 | 4.615 | 0.291 | 10.201 | 2.981 | 6.412 | 3.265 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| 16 | 8 | 1 | full_extension | 0.100 | 5.836 | 0.000 | 3.463 | 0.917 | 1.765 | 2.742 | 5.809 | 3.424 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| 16 | 8 | 1 | full_optimized | 0.000 | 0.000 | 0.000 | 2.128 | 1.294 | 0.001 | 0.067 | 2.062 | 0.000 | 5.365 | 0.000 | 0.000 | 0.000 | 0.000 |
| 16 | 8 | 1 | fused | 0.002 | 0.489 | 3.401 | 0.526 | 0.275 | 2.969 | 0.134 | 2.379 | 0.000 | 5.336 | 0.000 | 0.000 | 0.000 | 0.000 |
| 16 | 8 | 1 | lookup | 0.001 | 0.470 | 3.266 | 0.494 | 0.270 | 3.580 | 2.830 | 5.788 | 3.217 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| 16 | 8 | 1 | packed | 0.000 | 0.469 | 3.269 | 0.495 | 0.291 | 0.001 | 0.049 | 0.942 | 0.000 | 2.357 | 13.444 | 2.523 | 3.323 | 3.000 |
| 16 | 8 | 1 | reference | 0.100 | 7.905 | 10.670 | 4.472 | 0.242 | 11.287 | 2.741 | 6.541 | 3.265 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |

Validated 22 serialized proofs (11 measured), including all terminal PCS openings.
Checked same-input statements/parameters, same-protocol proof bytes across modes/threads, and balanced cyclic order in formal suites.
Excluded and preserved 0 interfered attempts; no timing-based exclusions.
Fast100/diagnostic PCS parameters; no ZK, no claim of equal complete 128-bit soundness or complete Flock R1CS throughput.
Packed changes the protocol and adds quotient/column identity checks. See PACKED_REDUCTION.md.
This is a shared WSL host; raw samples, ranges, compiler/CPU and source hashes are retained.
