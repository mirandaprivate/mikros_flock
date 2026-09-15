# N26 native PackedDirect OFF/ON comparison

single: N26 gates; multiple: 128 independent circuits each with N26 gates

Online commit/recommit, conversion, PIOPs, PCS and release included; offline excluded

Medians of independent fresh processes; phase medians need not sum to total median

Descriptive samples and ranges, not a statistical significance claim

Current OFF/ON use the same frozen source and binary; only FLOCK_PCS_PACKED_DIRECT differs.
PIOP, terminal claims, commitments and PCS parameters match. Full proofs may change across routes.

## single

| Stage (s) | OFF Mikros | OFF FLOCK | ON Mikros | ON FLOCK | F/M OFF | F/M ON |
|---|---:|---:|---:|---:|---:|---:|
| Gate PIOP | 0.0816 | 0.0171 | 0.0722 | 0.0171 | 0.210 | 0.236 |
| Copy PIOP | 1.4365 | 6.6353 | 1.4008 | 6.6910 | 4.619 | 4.777 |
| Witness PCS opening | 0.2389 | 0.2169 | 0.2327 | 0.2075 | 0.908 | 0.891 |
| Public PCS opening | 1.1946 | 0.9096 | 1.1718 | 0.9128 | 0.761 | 0.779 |
| Commit and recommits | 0.6017 | 0.6108 | 0.6085 | 0.6443 | 1.015 | 1.059 |
| Release | 1.6584 | 0.9177 | 1.6264 | 0.9116 | 0.553 | 0.561 |
| Serialization | 0.0010 | 0.0010 | 0.0009 | 0.0007 | 0.983 | 0.763 |
| Online total | 5.1379 | 9.3044 | 5.1185 | 9.2675 | 1.811 | 1.811 |

Relative F/M ratio change: -0.02%.

| System | Time reduction | OFF/ON speedup | RSS OFF/ON median (GiB) | Proof bytes OFF/ON |
|---|---:|---:|---:|---:|
| mikros | +0.38% | 1.004 | 17.520 / 17.520 | 1296879 / 1296879 |
| flock | +0.40% | 1.004 | 21.031 / 21.030 | 1215735 / 1215735 |

| System | Logical witness/public | Ring claims OFF witness/public | Ring claims ON witness/public | Verify OFF/ON (ms) |
|---|---:|---:|---:|---:|
| mikros | 11 / 89 | 11 / 89 | 11 / 89 | 46.725 / 45.310 |
| flock | 7 / 84 | 7 / 84 | 7 / 84 | 38.655 / 39.273 |

mikros off online seconds, one per process: [5.224975, 5.093888, 5.137921].
mikros on online seconds, one per process: [5.118452, 5.085132, 5.211386].
mikros baseline online seconds, one per process: [5.150012, 5.247331, 5.20758].
flock off online seconds, one per process: [9.304391, 9.207625, 9.619242].
flock on online seconds, one per process: [9.631568, 9.221626, 9.2675].
flock baseline online seconds, one per process: [9.522699, 9.455977, 9.284187].

## multiple

| Stage (s) | OFF Mikros | OFF FLOCK | ON Mikros | ON FLOCK | F/M OFF | F/M ON |
|---|---:|---:|---:|---:|---:|---:|
| Gate PIOP | 1.4101 | 1.8362 | 1.3309 | 1.8023 | 1.302 | 1.354 |
| Copy PIOP | 2.7286 | 6.4847 | 2.7150 | 6.7124 | 2.377 | 2.472 |
| Witness PCS opening | 7.1582 | 6.6663 | 8.3789 | 7.0588 | 0.931 | 0.842 |
| Public PCS opening | 1.3287 | 1.0343 | 1.2959 | 1.0360 | 0.778 | 0.799 |
| Commit and recommits | 4.2844 | 4.3825 | 4.2536 | 4.4280 | 1.023 | 1.041 |
| Release | 3.0492 | 3.5491 | 3.0181 | 3.4949 | 1.164 | 1.158 |
| Serialization | 0.0008 | 0.0011 | 0.0011 | 0.0006 | 1.403 | 0.542 |
| Online total | 19.9443 | 24.1655 | 21.2154 | 24.5170 | 1.212 | 1.156 |

Relative F/M ratio change: -4.62%.

| System | Time reduction | OFF/ON speedup | RSS OFF/ON median (GiB) | Proof bytes OFF/ON |
|---|---:|---:|---:|---:|
| mikros | -6.37% | 0.940 | 26.285 / 26.276 | 1239135 / 1202103 |
| flock | -1.45% | 0.986 | 26.308 / 26.309 | 1172983 / 1160639 |

| System | Logical witness/public | Ring claims OFF witness/public | Ring claims ON witness/public | Verify OFF/ON (ms) |
|---|---:|---:|---:|---:|
| mikros | 11 / 8 | 11 / 8 | 2 / 8 | 18.885 / 15.914 |
| flock | 7 / 3 | 7 / 3 | 4 / 3 | 16.972 / 15.673 |

mikros off online seconds, one per process: [19.944304, 19.678464, 20.9061].
mikros on online seconds, one per process: [21.215447, 21.499356, 20.651637].
mikros baseline online seconds, one per process: [21.211236, 19.923004, 20.037872].
flock off online seconds, one per process: [24.762065, 24.165537, 23.711724].
flock on online seconds, one per process: [24.507939, 24.521575, 24.516963].
flock baseline online seconds, one per process: [24.424474, 24.043425, 24.039764].

Every system/mode OFF full proof and commitment also matches the supplied v6 baseline.

Positive ratio change means this routing optimization improves Mikros's relative E2E advantage; negative change means FLOCK benefits more proportionally.

Full per-sample timings, RSS/resource records, proof hashes and input-file SHA-256 digests are in comparison.json.
