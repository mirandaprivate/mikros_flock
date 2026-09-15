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
| Gate PIOP | 0.0712 | 0.0175 | 0.0875 | 0.0169 | 0.246 | 0.193 |
| Copy PIOP | 1.3993 | 6.5085 | 1.4422 | 6.5803 | 4.651 | 4.563 |
| Witness PCS opening | 0.2313 | 0.2067 | 0.2424 | 0.2299 | 0.894 | 0.948 |
| Public PCS opening | 1.1682 | 0.9177 | 1.1851 | 0.9160 | 0.786 | 0.773 |
| Commit and recommits | 0.6939 | 0.7843 | 0.5846 | 0.6091 | 1.130 | 1.042 |
| Release | 1.6738 | 0.9320 | 1.6736 | 0.9522 | 0.557 | 0.569 |
| Serialization | 0.0009 | 0.0007 | 0.0010 | 0.0009 | 0.771 | 0.837 |
| Online total | 5.2657 | 9.2343 | 5.2218 | 9.2833 | 1.754 | 1.778 |

Relative F/M ratio change: +1.38%.

| System | Time reduction | OFF/ON speedup | RSS OFF/ON median (GiB) | Proof bytes OFF/ON |
|---|---:|---:|---:|---:|
| mikros | +0.83% | 1.008 | 17.520 / 17.520 | 1296879 / 1296879 |
| flock | -0.53% | 0.995 | 21.029 / 21.028 | 1215735 / 1215735 |

| System | Logical witness/public | Ring claims OFF witness/public | Ring claims ON witness/public | Verify OFF/ON (ms) |
|---|---:|---:|---:|---:|
| mikros | 11 / 89 | 11 / 89 | 11 / 89 | 46.141 / 45.571 |
| flock | 7 / 84 | 7 / 84 | 7 / 84 | 39.769 / 39.099 |

### Engineering change within the ON route

Previous/current ON have identical full proofs and protocol inputs; source and binary may differ.

| Stage (s) | Previous ON Mikros | Previous ON FLOCK | Current ON Mikros | Current ON FLOCK |
|---|---:|---:|---:|---:|
| Gate PIOP | 0.0722 | 0.0171 | 0.0875 | 0.0169 |
| Copy PIOP | 1.4008 | 6.6910 | 1.4422 | 6.5803 |
| Witness PCS opening | 0.2327 | 0.2075 | 0.2424 | 0.2299 |
| Public PCS opening | 1.1718 | 0.9128 | 1.1851 | 0.9160 |
| Commit and recommits | 0.6085 | 0.6443 | 0.5846 | 0.6091 |
| Release | 1.6264 | 0.9116 | 1.6736 | 0.9522 |
| Serialization | 0.0009 | 0.0007 | 0.0010 | 0.0009 |
| Online total | 5.1185 | 9.2675 | 5.2218 | 9.2833 |

| System | Previous/current ON speedup | Time reduction | RSS previous/current ON median (GiB) | Proof bytes previous/current ON | Verify previous/current ON (ms) |
|---|---:|---:|---:|---:|---:|
| mikros | 0.980 | -2.02% | 17.520 / 17.520 | 1296879 / 1296879 | 45.310 / 45.571 |
| flock | 0.998 | -0.17% | 21.030 / 21.028 | 1215735 / 1215735 | 39.273 / 39.099 |

Engineering change in F/M ratio: -1.81%.

mikros off online seconds, one per process: [5.297223, 5.265654, 5.143547].
mikros on online seconds, one per process: [5.221783, 5.184779, 5.276056].
mikros baseline online seconds, one per process: [5.150012, 5.247331, 5.20758].
mikros previous_on online seconds, one per process: [5.118452, 5.085132, 5.211386].
flock off online seconds, one per process: [9.19497, 9.23429, 9.429808].
flock on online seconds, one per process: [9.283303, 9.341047, 9.204774].
flock baseline online seconds, one per process: [9.522699, 9.455977, 9.284187].
flock previous_on online seconds, one per process: [9.631568, 9.221626, 9.2675].

## multiple

| Stage (s) | OFF Mikros | OFF FLOCK | ON Mikros | ON FLOCK | F/M OFF | F/M ON |
|---|---:|---:|---:|---:|---:|---:|
| Gate PIOP | 1.5131 | 1.7300 | 1.3386 | 1.7444 | 1.143 | 1.303 |
| Copy PIOP | 2.8587 | 6.6835 | 2.7165 | 6.4546 | 2.338 | 2.376 |
| Witness PCS opening | 7.0520 | 6.8044 | 5.6148 | 6.0150 | 0.965 | 1.071 |
| Public PCS opening | 1.3577 | 1.0436 | 1.2983 | 1.0341 | 0.769 | 0.796 |
| Commit and recommits | 4.8806 | 4.4045 | 4.3132 | 4.3179 | 0.902 | 1.001 |
| Release | 2.9973 | 3.5548 | 2.9935 | 3.4604 | 1.186 | 1.156 |
| Serialization | 0.0007 | 0.0007 | 0.0008 | 0.0006 | 0.983 | 0.791 |
| Online total | 20.6707 | 24.1170 | 18.2209 | 23.1648 | 1.167 | 1.271 |

Relative F/M ratio change: +8.97%.

| System | Time reduction | OFF/ON speedup | RSS OFF/ON median (GiB) | Proof bytes OFF/ON |
|---|---:|---:|---:|---:|
| mikros | +11.85% | 1.134 | 26.285 / 26.278 | 1239135 / 1202103 |
| flock | +3.95% | 1.041 | 26.311 / 26.307 | 1172983 / 1160639 |

| System | Logical witness/public | Ring claims OFF witness/public | Ring claims ON witness/public | Verify OFF/ON (ms) |
|---|---:|---:|---:|---:|
| mikros | 11 / 8 | 11 / 8 | 2 / 8 | 18.428 / 16.423 |
| flock | 7 / 3 | 7 / 3 | 4 / 3 | 16.914 / 16.013 |

### Engineering change within the ON route

Previous/current ON have identical full proofs and protocol inputs; source and binary may differ.

| Stage (s) | Previous ON Mikros | Previous ON FLOCK | Current ON Mikros | Current ON FLOCK |
|---|---:|---:|---:|---:|
| Gate PIOP | 1.3309 | 1.8023 | 1.3386 | 1.7444 |
| Copy PIOP | 2.7150 | 6.7124 | 2.7165 | 6.4546 |
| Witness PCS opening | 8.3789 | 7.0588 | 5.6148 | 6.0150 |
| Public PCS opening | 1.2959 | 1.0360 | 1.2983 | 1.0341 |
| Commit and recommits | 4.2536 | 4.4280 | 4.3132 | 4.3179 |
| Release | 3.0181 | 3.4949 | 2.9935 | 3.4604 |
| Serialization | 0.0011 | 0.0006 | 0.0008 | 0.0006 |
| Online total | 21.2154 | 24.5170 | 18.2209 | 23.1648 |

| System | Previous/current ON speedup | Time reduction | RSS previous/current ON median (GiB) | Proof bytes previous/current ON | Verify previous/current ON (ms) |
|---|---:|---:|---:|---:|---:|
| mikros | 1.164 | +14.12% | 26.276 / 26.278 | 1202103 / 1202103 | 15.914 / 16.423 |
| flock | 1.058 | +5.52% | 26.309 / 26.307 | 1160639 / 1160639 | 15.673 / 16.013 |

Engineering change in F/M ratio: +10.01%.

mikros off online seconds, one per process: [20.670727, 19.720679, 20.906391].
mikros on online seconds, one per process: [19.258085, 18.051204, 18.220877].
mikros baseline online seconds, one per process: [21.211236, 19.923004, 20.037872].
mikros previous_on online seconds, one per process: [21.215447, 21.499356, 20.651637].
flock off online seconds, one per process: [24.748054, 24.11703, 23.753875].
flock on online seconds, one per process: [23.164762, 23.467278, 22.951596].
flock baseline online seconds, one per process: [24.424474, 24.043425, 24.039764].
flock previous_on online seconds, one per process: [24.507939, 24.521575, 24.516963].

Every system/mode OFF full proof and commitment also matches the supplied v6 baseline.

Positive ratio change means this routing optimization improves Mikros's relative E2E advantage; negative change means FLOCK benefits more proportionally.

Full per-sample timings, RSS/resource records, proof hashes and input-file SHA-256 digests are in comparison.json.
