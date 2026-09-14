# Mikros permutation + Binius: ordered optimization

Suite: ablation. Unit: ms, measured medians. Same raw bit commitments and PCS policy.
Reference is the instrumented original path; Flock/reference retains the original adapter.
Flock/bridge uses the same optimized bit conversion, byte embedding and column evaluator as Mikros.

| log N | bits | threads | system / mode | prepare | commit | PIOP | bridge | PCS open | total | verify | proof KiB |
|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 20 | 8 | 1 | flock/bridge | 44.902 | 16.962 | 153.668 | 2.719 | 24.186 | 245.148 | 41.305 | 641.56 |
| 20 | 8 | 1 | flock/reference | 114.624 | 16.519 | 157.868 | 123.868 | 25.652 | 438.256 | 43.430 | 641.56 |
| 20 | 8 | 1 | mikros/batched | 61.208 | 16.474 | 211.661 | 166.303 | 25.206 | 481.159 | 53.704 | 641.60 |
| 20 | 8 | 1 | mikros/bridge | 61.618 | 16.699 | 209.640 | 19.031 | 24.071 | 333.523 | 25.141 | 641.60 |
| 20 | 8 | 1 | mikros/direct | 63.115 | 16.497 | 615.343 | 166.952 | 25.485 | 887.815 | 53.141 | 653.64 |
| 20 | 8 | 1 | mikros/full_extension | 398.627 | 16.430 | 443.353 | 168.878 | 25.625 | 1051.832 | 53.600 | 653.64 |
| 20 | 8 | 1 | mikros/full_optimized | 60.574 | 16.473 | 131.625 | 18.778 | 23.994 | 249.577 | 26.299 | 641.60 |
| 20 | 8 | 1 | mikros/fused | 62.476 | 16.518 | 222.522 | 166.338 | 25.849 | 499.081 | 53.130 | 653.64 |
| 20 | 8 | 1 | mikros/lookup | 61.041 | 16.404 | 296.914 | 165.431 | 25.569 | 563.763 | 53.953 | 653.64 |
| 20 | 8 | 1 | mikros/packed | 60.622 | 16.466 | 442.214 | 18.932 | 24.091 | 557.522 | 26.006 | 642.21 |
| 20 | 8 | 1 | mikros/reference | 402.323 | 16.594 | 772.979 | 169.147 | 25.508 | 1383.217 | 53.142 | 653.64 |
| 20 | 8 | 6 | flock/bridge | 27.966 | 15.974 | 60.848 | 1.109 | 27.526 | 133.619 | 36.227 | 641.56 |
| 20 | 8 | 6 | flock/reference | 116.728 | 15.965 | 59.776 | 101.890 | 31.679 | 321.059 | 35.660 | 641.56 |
| 20 | 8 | 6 | mikros/batched | 39.414 | 16.010 | 73.895 | 138.393 | 29.541 | 295.450 | 48.037 | 641.60 |
| 20 | 8 | 6 | mikros/bridge | 39.512 | 16.157 | 71.565 | 5.150 | 27.952 | 161.888 | 17.843 | 641.60 |
| 20 | 8 | 6 | mikros/direct | 38.304 | 16.068 | 604.582 | 138.298 | 33.266 | 834.829 | 49.046 | 653.64 |
| 20 | 8 | 6 | mikros/full_extension | 394.215 | 16.712 | 424.896 | 133.370 | 29.406 | 1002.844 | 47.025 | 653.64 |
| 20 | 8 | 6 | mikros/full_optimized | 40.303 | 16.316 | 52.069 | 5.310 | 27.516 | 143.439 | 18.607 | 641.60 |
| 20 | 8 | 6 | mikros/fused | 39.778 | 16.409 | 75.228 | 133.592 | 30.846 | 297.243 | 47.047 | 653.64 |
| 20 | 8 | 6 | mikros/lookup | 40.058 | 15.864 | 210.970 | 134.841 | 30.530 | 432.014 | 46.402 | 653.64 |
| 20 | 8 | 6 | mikros/packed | 39.631 | 15.749 | 122.266 | 5.310 | 26.828 | 210.422 | 18.367 | 642.21 |
| 20 | 8 | 6 | mikros/reference | 394.032 | 17.192 | 764.737 | 135.267 | 33.221 | 1330.355 | 47.381 | 653.64 |

## PIOP phase attribution

Phase medians are independently calculated and need not sum to the total median.
Fused message/fold timing is a single combined phase, not separately estimated.

| log N | bits | threads | mode | checks | decode | compact tree | coefficient eval | extension tree | materialize | eq | messages | fold | fused fold/message | packed quotient | packed projection | packed columns | skipped rounds |
|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 20 | 8 | 1 | batched | 0.000 | 7.638 | 52.159 | 7.274 | 3.950 | 35.896 | 0.226 | 27.786 | 0.000 | 74.117 | 0.000 | 0.000 | 0.000 | 0.000 |
| 20 | 8 | 1 | bridge | 0.000 | 7.488 | 52.835 | 7.319 | 3.881 | 35.891 | 0.226 | 27.983 | 0.000 | 73.517 | 0.000 | 0.000 | 0.000 | 0.000 |
| 20 | 8 | 1 | direct | 0.002 | 5.148 | 188.147 | 72.236 | 3.670 | 158.104 | 31.586 | 93.095 | 50.849 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| 20 | 8 | 1 | full_extension | 1.879 | 169.807 | 0.000 | 55.582 | 15.062 | 18.081 | 32.042 | 93.879 | 50.967 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| 20 | 8 | 1 | full_optimized | 0.000 | 0.000 | 0.000 | 11.601 | 16.285 | 0.003 | 0.225 | 27.844 | 0.000 | 75.006 | 0.000 | 0.000 | 0.000 | 0.000 |
| 20 | 8 | 1 | fused | 0.002 | 7.706 | 55.428 | 7.565 | 3.969 | 35.929 | 0.442 | 33.266 | 0.000 | 76.939 | 0.000 | 0.000 | 0.000 | 0.000 |
| 20 | 8 | 1 | lookup | 0.002 | 7.441 | 54.246 | 7.272 | 3.896 | 47.755 | 31.295 | 93.684 | 51.874 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| 20 | 8 | 1 | packed | 0.000 | 7.605 | 53.011 | 7.444 | 4.420 | 0.002 | 0.192 | 15.102 | 0.000 | 40.520 | 220.729 | 34.668 | 53.177 | 3.000 |
| 20 | 8 | 1 | reference | 1.826 | 175.563 | 189.958 | 72.728 | 3.734 | 158.205 | 32.060 | 94.164 | 51.238 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| 20 | 8 | 6 | batched | 0.000 | 2.964 | 14.863 | 2.824 | 4.371 | 10.069 | 0.232 | 7.892 | 0.000 | 30.256 | 0.000 | 0.000 | 0.000 | 0.000 |
| 20 | 8 | 6 | bridge | 0.000 | 2.930 | 14.735 | 2.932 | 4.768 | 9.285 | 0.234 | 7.968 | 0.000 | 28.492 | 0.000 | 0.000 | 0.000 | 0.000 |
| 20 | 8 | 6 | direct | 0.002 | 4.937 | 184.799 | 72.985 | 3.804 | 164.384 | 21.075 | 96.060 | 52.088 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| 20 | 8 | 6 | full_extension | 1.874 | 162.411 | 0.000 | 53.263 | 15.256 | 19.522 | 22.372 | 94.576 | 51.292 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| 20 | 8 | 6 | full_optimized | 0.000 | 0.000 | 0.000 | 5.454 | 10.470 | 0.003 | 0.233 | 8.041 | 0.000 | 28.328 | 0.000 | 0.000 | 0.000 | 0.000 |
| 20 | 8 | 6 | fused | 0.002 | 3.270 | 15.000 | 2.776 | 4.612 | 9.519 | 0.494 | 10.536 | 0.000 | 29.988 | 0.000 | 0.000 | 0.000 | 0.000 |
| 20 | 8 | 6 | lookup | 0.002 | 2.951 | 14.979 | 3.016 | 4.413 | 22.257 | 17.665 | 95.080 | 51.734 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |
| 20 | 8 | 6 | packed | 0.000 | 2.600 | 14.352 | 2.915 | 4.638 | 0.002 | 0.199 | 4.788 | 0.000 | 19.017 | 52.461 | 9.905 | 12.096 | 3.000 |
| 20 | 8 | 6 | reference | 1.859 | 163.623 | 184.261 | 72.143 | 3.776 | 160.204 | 23.408 | 94.531 | 52.192 | 0.000 | 0.000 | 0.000 | 0.000 | 0.000 |

Validated 264 serialized proofs (242 measured), including all terminal PCS openings.
Checked same-input statements/parameters, same-protocol proof bytes across modes/threads, and balanced cyclic order in formal suites.
Excluded and preserved 2 interfered attempts; no timing-based exclusions.
Fast100/diagnostic PCS parameters; no ZK, no claim of equal complete 128-bit soundness or complete Flock R1CS throughput.
Packed changes the protocol and adds quotient/column identity checks. See PACKED_REDUCTION.md.
This is a shared WSL host; raw samples, ranges, compiler/CPU and source hashes are retained.
