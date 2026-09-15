# N26 Mikros tiling and scratch ablation

single: N26 gates; multiple: 128 circuits each with N26 gates (total 2^33 constraints)

Online commit/recommit, conversion, PIOPs, PCS, release and serialization included; offline excluded; verify separate; RSS includes the entire process

Median [min, max] of at least three fresh processes per arm/case; phase medians need not sum to total median

Descriptive samples, not a significance claim. Diagnostic trace runs are rejected. FLOCK is a no-op control for all new flags. Mikros single is a no-op control for virtual, paired and scratch; weights/all may also affect its weighted Hadamard calls. No-op control changes are variation; changes in unmodified Mikros phases may include indirect memory effects and must not automatically be credited to the target kernels.

All arms use PackedDirect + WP first-write. Earlier WP SIMD, selector SIMD and gate-fusion flags are OFF.
All current arms share source/binary, input commitments and native PCS. Same system/mode complete proofs, PIOPs, terminal claims, PCS parameters and counts agree across all arms.

| Arm | Additional opt-ins |
|---|---|
| reference | none |
| virtual | MIKROS_GATE_VIRTUAL_PRODUCT=1 |
| weights | MIKROS_WH_TILED_WEIGHTS=1 |
| paired | E2E_COPY_PAIRED_TERMINALS=1 |
| scratch | MIKROS_GATE_VIRTUAL_PRODUCT=1, MIKROS_GATE_WH_SCRATCH=1 |
| all | MIKROS_GATE_VIRTUAL_PRODUCT=1, MIKROS_WH_TILED_WEIGHTS=1, E2E_COPY_PAIRED_TERMINALS=1, MIKROS_GATE_WH_SCRATCH=1 |

Previous reference is separate evidence with WP first-write ON; only source/binary may differ. Missing new flags mean OFF, while all previous flags must be explicit and correct.

## single

F/M > 1 means Mikros is faster. Times are median [min, max] seconds.

| Stage | Arm | Mikros (s) | FLOCK (s) | M/F | F/M |
|---|---|---:|---:|---:|---:|
| Gate PIOP | reference | 0.0726 [0.0654, 0.0734] | 0.0166 [0.0166, 0.0169] | 4.376 | 0.229 |
| Gate PIOP | virtual | 0.0673 [0.0628, 0.0741] | 0.0166 [0.0164, 0.0173] | 4.045 | 0.247 |
| Gate PIOP | weights | 0.0657 [0.0602, 0.0744] | 0.0172 [0.0163, 0.0183] | 3.815 | 0.262 |
| Gate PIOP | paired | 0.0736 [0.0658, 0.0743] | 0.0165 [0.0162, 0.0168] | 4.474 | 0.224 |
| Gate PIOP | scratch | 0.0680 [0.0657, 0.0723] | 0.0168 [0.0166, 0.0193] | 4.046 | 0.247 |
| Gate PIOP | all | 0.0685 [0.0642, 0.0778] | 0.0171 [0.0168, 0.0171] | 4.006 | 0.250 |
| Gate PIOP | previous_reference | 0.0694 [0.0653, 0.0777] | 0.0178 [0.0174, 0.0197] | 3.895 | 0.257 |
| Copy PIOP | reference | 1.3317 [1.3088, 1.3470] | 6.6029 [6.5787, 6.6203] | 0.202 | 4.958 |
| Copy PIOP | virtual | 1.3099 [1.2789, 1.3138] | 6.5020 [6.3368, 6.6349] | 0.201 | 4.964 |
| Copy PIOP | weights | 1.3483 [1.2965, 1.3490] | 6.4994 [6.2665, 6.5173] | 0.207 | 4.820 |
| Copy PIOP | paired | 1.3006 [1.2719, 1.3060] | 6.3470 [6.3169, 6.5291] | 0.205 | 4.880 |
| Copy PIOP | scratch | 1.3058 [1.3001, 1.3285] | 6.5080 [6.4515, 6.6294] | 0.201 | 4.984 |
| Copy PIOP | all | 1.3055 [1.3043, 1.3204] | 6.5502 [6.4483, 6.7511] | 0.199 | 5.017 |
| Copy PIOP | previous_reference | 1.4375 [1.3828, 1.4474] | 6.6278 [6.5388, 6.7462] | 0.217 | 4.611 |
| Witness PCS opening | reference | 0.2232 [0.2152, 0.2259] | 0.2184 [0.2104, 0.2573] | 1.022 | 0.979 |
| Witness PCS opening | virtual | 0.2247 [0.2138, 0.2317] | 0.1862 [0.1748, 0.1997] | 1.206 | 0.829 |
| Witness PCS opening | weights | 0.2188 [0.2028, 0.2360] | 0.2144 [0.1790, 0.2268] | 1.021 | 0.980 |
| Witness PCS opening | paired | 0.2208 [0.2198, 0.2268] | 0.2077 [0.2039, 0.2135] | 1.063 | 0.941 |
| Witness PCS opening | scratch | 0.2198 [0.2172, 0.2444] | 0.1908 [0.1737, 0.2009] | 1.152 | 0.868 |
| Witness PCS opening | all | 0.2177 [0.2101, 0.2194] | 0.2081 [0.2073, 0.2138] | 1.046 | 0.956 |
| Witness PCS opening | previous_reference | 0.2158 [0.2073, 0.2259] | 0.1940 [0.1688, 0.2258] | 1.112 | 0.899 |
| Public PCS opening | reference | 1.1815 [1.1468, 1.1979] | 0.8620 [0.8528, 0.8831] | 1.371 | 0.730 |
| Public PCS opening | virtual | 1.1747 [1.1685, 1.1804] | 0.8987 [0.8759, 0.9097] | 1.307 | 0.765 |
| Public PCS opening | weights | 1.1530 [1.1406, 1.1789] | 0.9028 [0.8653, 0.9172] | 1.277 | 0.783 |
| Public PCS opening | paired | 1.1839 [1.1489, 1.1941] | 0.8945 [0.8566, 0.9012] | 1.323 | 0.756 |
| Public PCS opening | scratch | 1.1891 [1.1866, 1.1931] | 0.9179 [0.8834, 0.9187] | 1.295 | 0.772 |
| Public PCS opening | all | 1.1685 [1.1672, 1.1736] | 0.9220 [0.8516, 0.9271] | 1.267 | 0.789 |
| Public PCS opening | previous_reference | 1.1762 [1.1413, 1.2052] | 0.9156 [0.8711, 0.9198] | 1.285 | 0.778 |
| Commit and recommits | reference | 0.5682 [0.5645, 0.5832] | 0.5897 [0.5523, 0.6299] | 0.964 | 1.038 |
| Commit and recommits | virtual | 0.5832 [0.5829, 0.6655] | 0.6652 [0.4858, 0.6664] | 0.877 | 1.140 |
| Commit and recommits | weights | 0.5713 [0.5666, 0.6135] | 0.6113 [0.5775, 0.7208] | 0.934 | 1.070 |
| Commit and recommits | paired | 0.6363 [0.5744, 0.6366] | 0.6050 [0.5967, 0.6392] | 1.052 | 0.951 |
| Commit and recommits | scratch | 0.5777 [0.5693, 0.6022] | 0.6368 [0.5866, 0.6369] | 0.907 | 1.102 |
| Commit and recommits | all | 0.6121 [0.5715, 0.6744] | 0.6210 [0.5936, 0.7917] | 0.986 | 1.014 |
| Commit and recommits | previous_reference | 0.6256 [0.6117, 0.6688] | 0.7193 [0.7094, 0.7310] | 0.870 | 1.150 |
| Release | reference | 1.5804 [1.4799, 1.5950] | 0.8692 [0.8388, 0.8822] | 1.818 | 0.550 |
| Release | virtual | 1.5166 [1.5088, 1.5184] | 0.8837 [0.8575, 0.8862] | 1.716 | 0.583 |
| Release | weights | 1.5360 [1.5037, 1.5795] | 0.8623 [0.8452, 0.8874] | 1.781 | 0.561 |
| Release | paired | 1.5527 [1.5466, 1.5581] | 0.8874 [0.8480, 0.8906] | 1.750 | 0.572 |
| Release | scratch | 1.5861 [1.5542, 1.5932] | 0.8796 [0.8271, 0.8880] | 1.803 | 0.555 |
| Release | all | 1.5371 [1.5277, 1.5919] | 0.8603 [0.8569, 0.8783] | 1.787 | 0.560 |
| Release | previous_reference | 1.6740 [1.6384, 1.6855] | 0.9107 [0.8893, 0.9130] | 1.838 | 0.544 |
| Serialization | reference | 0.0009 [0.0008, 0.0009] | 0.0008 [0.0008, 0.0012] | 1.050 | 0.952 |
| Serialization | virtual | 0.0010 [0.0009, 0.0011] | 0.0008 [0.0007, 0.0011] | 1.315 | 0.761 |
| Serialization | weights | 0.0009 [0.0008, 0.0009] | 0.0008 [0.0008, 0.0011] | 1.136 | 0.880 |
| Serialization | paired | 0.0010 [0.0009, 0.0011] | 0.0008 [0.0008, 0.0008] | 1.319 | 0.758 |
| Serialization | scratch | 0.0009 [0.0009, 0.0009] | 0.0008 [0.0008, 0.0012] | 1.082 | 0.924 |
| Serialization | all | 0.0013 [0.0010, 0.0014] | 0.0008 [0.0008, 0.0010] | 1.682 | 0.594 |
| Serialization | previous_reference | 0.0009 [0.0007, 0.0011] | 0.0007 [0.0007, 0.0008] | 1.307 | 0.765 |
| Online total | reference | 4.9353 [4.8422, 4.9869] | 9.1640 [9.1531, 9.1848] | 0.539 | 1.857 |
| Online total | virtual | 4.8704 [4.8585, 4.9509] | 8.9723 [8.9561, 9.2886] | 0.543 | 1.842 |
| Online total | weights | 4.8948 [4.8620, 4.9416] | 9.1207 [8.8459, 9.1819] | 0.537 | 1.863 |
| Online total | paired | 4.9282 [4.8979, 4.9691] | 8.9661 [8.8881, 9.1359] | 0.550 | 1.819 |
| Online total | scratch | 4.9516 [4.9296, 4.9956] | 9.1020 [9.0810, 9.2027] | 0.544 | 1.838 |
| Online total | all | 4.9420 [4.9191, 4.9555] | 9.2085 [8.9760, 9.5514] | 0.537 | 1.863 |
| Online total | previous_reference | 5.1965 [5.1183, 5.2447] | 9.3977 [9.2769, 9.4639] | 0.553 | 1.808 |

Negative delta means less time; positive relative F/M gain improves Mikros's relative advantage.

| Stage | Arm | Mikros delta (s) | Mikros reduction | FLOCK delta (s) | FLOCK reduction | Relative F/M gain |
|---|---|---:|---:|---:|---:|---:|
| Gate PIOP | virtual | -0.0054 | +7.38% | +0.0000 | -0.19% | +8.18% |
| Gate PIOP | weights | -0.0070 | +9.62% | +0.0006 | -3.66% | +14.70% |
| Gate PIOP | paired | +0.0010 | -1.32% | -0.0001 | +0.88% | -2.18% |
| Gate PIOP | scratch | -0.0046 | +6.36% | +0.0002 | -1.28% | +8.16% |
| Gate PIOP | all | -0.0041 | +5.66% | +0.0005 | -3.06% | +9.24% |
| Copy PIOP | virtual | -0.0218 | +1.64% | -0.1009 | +1.53% | +0.11% |
| Copy PIOP | weights | +0.0166 | -1.25% | -0.1035 | +1.57% | -2.78% |
| Copy PIOP | paired | -0.0311 | +2.34% | -0.2559 | +3.88% | -1.57% |
| Copy PIOP | scratch | -0.0259 | +1.95% | -0.0949 | +1.44% | +0.52% |
| Copy PIOP | all | -0.0262 | +1.97% | -0.0527 | +0.80% | +1.19% |
| Witness PCS opening | virtual | +0.0014 | -0.64% | -0.0322 | +14.74% | -15.28% |
| Witness PCS opening | weights | -0.0044 | +1.97% | -0.0040 | +1.85% | +0.12% |
| Witness PCS opening | paired | -0.0024 | +1.09% | -0.0107 | +4.91% | -3.86% |
| Witness PCS opening | scratch | -0.0035 | +1.55% | -0.0276 | +12.64% | -11.26% |
| Witness PCS opening | all | -0.0055 | +2.47% | -0.0103 | +4.72% | -2.31% |
| Public PCS opening | virtual | -0.0068 | +0.58% | +0.0367 | -4.25% | +4.86% |
| Public PCS opening | weights | -0.0285 | +2.41% | +0.0408 | -4.73% | +7.31% |
| Public PCS opening | paired | +0.0024 | -0.20% | +0.0325 | -3.77% | +3.57% |
| Public PCS opening | scratch | +0.0076 | -0.65% | +0.0559 | -6.49% | +5.81% |
| Public PCS opening | all | -0.0130 | +1.10% | +0.0600 | -6.96% | +8.16% |
| Commit and recommits | virtual | +0.0150 | -2.64% | +0.0754 | -12.79% | +9.89% |
| Commit and recommits | weights | +0.0031 | -0.54% | +0.0216 | -3.66% | +3.11% |
| Commit and recommits | paired | +0.0681 | -11.99% | +0.0153 | -2.59% | -8.39% |
| Commit and recommits | scratch | +0.0095 | -1.66% | +0.0471 | -7.99% | +6.22% |
| Commit and recommits | all | +0.0439 | -7.73% | +0.0313 | -5.30% | -2.25% |
| Release | virtual | -0.0638 | +4.04% | +0.0144 | -1.66% | +5.94% |
| Release | weights | -0.0444 | +2.81% | -0.0069 | +0.79% | +2.07% |
| Release | paired | -0.0278 | +1.76% | +0.0181 | -2.09% | +3.91% |
| Release | scratch | +0.0057 | -0.36% | +0.0104 | -1.20% | +0.83% |
| Release | all | -0.0434 | +2.74% | -0.0089 | +1.03% | +1.76% |
| Serialization | virtual | +0.0002 | -22.98% | -0.0000 | +1.72% | -20.09% |
| Serialization | weights | +0.0000 | -2.11% | -0.0000 | +5.58% | -7.53% |
| Serialization | paired | +0.0002 | -22.77% | -0.0000 | +2.20% | -20.34% |
| Serialization | scratch | +0.0000 | -4.56% | +0.0000 | -1.53% | -2.90% |
| Serialization | all | +0.0005 | -55.94% | -0.0000 | +2.62% | -37.55% |
| Online total | virtual | -0.0649 | +1.32% | -0.1917 | +2.09% | -0.79% |
| Online total | weights | -0.0405 | +0.82% | -0.0433 | +0.47% | +0.35% |
| Online total | paired | -0.0071 | +0.14% | -0.1980 | +2.16% | -2.02% |
| Online total | scratch | +0.0163 | -0.33% | -0.0621 | +0.68% | -1.00% |
| Online total | all | +0.0067 | -0.13% | +0.0445 | -0.49% | +0.35% |

| System | Arm | Samples | Proof bytes | Logical W/P | Ring W/P | Verify ms [range] | Whole-process RSS GiB [range] |
|---|---|---:|---:|---:|---:|---:|---:|
| mikros | reference | 3 | 1296879 | 11/89 | 11/89 | 46.022 [46.021, 50.055] | 17.522 [17.516, 17.522] |
| mikros | virtual | 3 | 1296879 | 11/89 | 11/89 | 46.633 [46.398, 48.779] | 17.522 [17.520, 17.522] |
| mikros | weights | 3 | 1296879 | 11/89 | 11/89 | 45.905 [43.719, 58.815] | 17.520 [17.520, 17.522] |
| mikros | paired | 3 | 1296879 | 11/89 | 11/89 | 46.952 [46.262, 47.359] | 17.522 [17.520, 17.522] |
| mikros | scratch | 3 | 1296879 | 11/89 | 11/89 | 46.151 [44.899, 48.375] | 17.521 [17.521, 17.521] |
| mikros | all | 3 | 1296879 | 11/89 | 11/89 | 46.292 [45.431, 49.002] | 17.523 [17.523, 17.524] |
| mikros | previous_reference | 3 | 1296879 | 11/89 | 11/89 | 46.347 [44.282, 47.561] | 17.522 [17.518, 17.522] |
| flock | reference | 3 | 1215735 | 7/84 | 7/84 | 39.497 [38.661, 41.470] | 21.030 [21.027, 21.031] |
| flock | virtual | 3 | 1215735 | 7/84 | 7/84 | 40.272 [39.153, 41.224] | 21.029 [21.027, 21.032] |
| flock | weights | 3 | 1215735 | 7/84 | 7/84 | 39.644 [39.346, 43.410] | 21.029 [21.028, 21.031] |
| flock | paired | 3 | 1215735 | 7/84 | 7/84 | 41.438 [40.120, 42.123] | 21.029 [21.026, 21.033] |
| flock | scratch | 3 | 1215735 | 7/84 | 7/84 | 40.331 [39.819, 41.103] | 21.030 [21.030, 21.031] |
| flock | all | 3 | 1215735 | 7/84 | 7/84 | 40.942 [38.873, 44.381] | 21.028 [21.027, 21.028] |
| flock | previous_reference | 3 | 1215735 | 7/84 | 7/84 | 39.473 [38.155, 40.506] | 21.031 [21.024, 21.032] |

Raw primary phase timings, milliseconds (one row per fresh process):

| System | Arm | Directory | commit_ms | recommit_ms | public_recommit_ms | gates_ms | copy_ms | release_ms | witness_pcs_open_ms | public_pcs_open_ms | serialization_ms | prover_ms |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| mikros | reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-reference-n26-r1 | 15.832468 | 16.830656 | 531.837546 | 72.641467 | 1346.959228 | 1580.415590 | 223.220098 | 1197.937834 | 0.851646 | 4986.866486 |
| mikros | reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-reference-n26-r2 | 14.612899 | 15.136314 | 538.470781 | 65.370235 | 1308.805465 | 1595.049798 | 215.245448 | 1181.498956 | 0.836091 | 4935.321271 |
| mikros | reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-reference-n26-r3 | 16.302512 | 24.796428 | 542.090942 | 73.414231 | 1331.705016 | 1479.902828 | 225.948340 | 1146.786454 | 0.899084 | 4842.196327 |
| mikros | virtual | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-virtual-n26-r1 | 17.540714 | 21.279382 | 626.658545 | 67.279251 | 1309.901947 | 1518.358242 | 213.823300 | 1174.670015 | 1.088866 | 4950.889876 |
| mikros | virtual | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-virtual-n26-r2 | 15.862990 | 22.854106 | 544.170312 | 62.805044 | 1313.781987 | 1516.566125 | 224.658934 | 1168.481308 | 0.898476 | 4870.375246 |
| mikros | virtual | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-virtual-n26-r3 | 17.218346 | 22.794424 | 543.213927 | 74.080520 | 1278.935546 | 1508.815551 | 231.702432 | 1180.360739 | 1.047391 | 4858.450209 |
| mikros | weights | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-weights-n26-r1 | 14.999476 | 22.182165 | 534.089149 | 74.441937 | 1296.453993 | 1579.506902 | 218.826444 | 1153.044313 | 0.916926 | 4894.772667 |
| mikros | weights | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-weights-n26-r2 | 15.205057 | 16.511297 | 581.811760 | 60.163800 | 1348.964580 | 1536.043048 | 202.849060 | 1178.885325 | 0.869628 | 4941.603158 |
| mikros | weights | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-weights-n26-r3 | 16.634851 | 23.245169 | 526.733355 | 65.650659 | 1348.307232 | 1503.658584 | 235.973344 | 1140.619180 | 0.842127 | 4861.974800 |
| mikros | paired | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-paired-n26-r1 | 16.108798 | 21.211868 | 599.008199 | 73.603812 | 1300.576048 | 1546.566140 | 226.783438 | 1183.881620 | 1.045575 | 4969.097136 |
| mikros | paired | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-paired-n26-r2 | 15.683126 | 14.477287 | 606.488864 | 65.804145 | 1271.928482 | 1552.662389 | 220.782451 | 1148.883514 | 0.875658 | 4897.874991 |
| mikros | paired | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-paired-n26-r3 | 15.720666 | 20.912111 | 537.772217 | 74.307675 | 1306.032680 | 1558.074475 | 219.844636 | 1194.079890 | 1.132080 | 4928.234788 |
| mikros | scratch | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-scratch-n26-r1 | 16.236677 | 20.381168 | 541.060417 | 68.019794 | 1305.776234 | 1586.098655 | 219.756664 | 1193.094234 | 0.857021 | 4951.610396 |
| mikros | scratch | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-scratch-n26-r2 | 16.830043 | 21.257848 | 531.246805 | 72.343657 | 1328.518689 | 1593.174073 | 244.448753 | 1186.579214 | 0.890519 | 4995.608399 |
| mikros | scratch | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-scratch-n26-r3 | 15.936692 | 22.710602 | 563.516199 | 65.672783 | 1300.081459 | 1554.191602 | 217.200314 | 1189.131267 | 0.913189 | 4929.638962 |
| mikros | all | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-all-n26-r1 | 18.420400 | 16.524163 | 639.503585 | 64.152497 | 1304.250556 | 1527.693647 | 210.141896 | 1173.571838 | 0.957157 | 4955.499279 |
| mikros | all | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-all-n26-r2 | 15.677777 | 15.301426 | 581.157048 | 77.807144 | 1305.526045 | 1537.063929 | 217.707671 | 1167.236700 | 1.328096 | 4919.104386 |
| mikros | all | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-all-n26-r3 | 16.322534 | 24.223268 | 530.914357 | 68.530958 | 1320.377073 | 1591.944693 | 219.419356 | 1168.486711 | 1.428991 | 4941.971611 |
| mikros | previous_reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v10/evidence/wp_uninit-n26-r1 | 22.259503 | 17.799444 | 585.497275 | 69.393396 | 1447.429291 | 1685.460937 | 225.944846 | 1141.264586 | 1.115569 | 5196.531319 |
| mikros | previous_reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v10/evidence/wp_uninit-n26-r2 | 15.279321 | 14.248383 | 582.129873 | 65.281800 | 1382.786696 | 1673.993143 | 207.341798 | 1176.232761 | 0.718880 | 5118.302526 |
| mikros | previous_reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v10/evidence/wp_uninit-n26-r3 | 17.304820 | 16.191734 | 635.319346 | 77.711579 | 1437.483205 | 1638.403429 | 215.821679 | 1205.237292 | 0.885151 | 5244.711783 |
| flock | reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-reference-n26-r1 | 15.138411 | 23.831936 | 513.328517 | 16.569896 | 6620.287912 | 869.228435 | 210.435865 | 883.095339 | 0.810733 | 9153.103363 |
| flock | reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-reference-n26-r2 | 14.378273 | 31.389270 | 584.151230 | 16.599663 | 6578.671696 | 838.809243 | 257.349591 | 862.007774 | 1.158910 | 9184.806972 |
| flock | reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-reference-n26-r3 | 15.388729 | 28.522439 | 545.820396 | 16.902042 | 6602.917601 | 882.239708 | 218.428560 | 852.784413 | 0.780367 | 9164.047583 |
| flock | virtual | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-virtual-n26-r1 | 15.119611 | 44.455692 | 606.863824 | 17.261728 | 6634.918470 | 883.666759 | 186.240119 | 898.682459 | 1.068486 | 9288.574408 |
| flock | virtual | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-virtual-n26-r2 | 15.032067 | 45.497299 | 604.622768 | 16.350657 | 6336.777629 | 886.191096 | 174.808196 | 875.855306 | 0.674072 | 8956.149130 |
| flock | virtual | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-virtual-n26-r3 | 14.995904 | 26.375524 | 444.430354 | 16.631649 | 6502.008443 | 857.469631 | 199.674832 | 909.655400 | 0.796797 | 8972.325092 |
| flock | weights | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-weights-n26-r1 | 15.192326 | 24.120878 | 681.450904 | 18.345570 | 6499.368285 | 862.330139 | 214.390824 | 865.324702 | 1.100551 | 9181.897106 |
| flock | weights | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-weights-n26-r2 | 15.233961 | 24.044653 | 538.200498 | 17.207927 | 6266.538650 | 887.408337 | 178.952432 | 917.190648 | 0.763117 | 8845.918246 |
| flock | weights | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-weights-n26-r3 | 15.000874 | 28.555255 | 567.781413 | 16.286896 | 6517.298346 | 845.172447 | 226.792168 | 902.776514 | 0.765505 | 9120.729109 |
| flock | paired | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-paired-n26-r1 | 16.017179 | 31.233662 | 557.773559 | 16.787052 | 6347.007656 | 847.964960 | 213.510343 | 856.628211 | 0.842370 | 8888.146659 |
| flock | paired | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-paired-n26-r2 | 14.994868 | 53.309322 | 570.881712 | 16.453242 | 6316.946995 | 887.360145 | 203.914803 | 901.177107 | 0.780735 | 8966.075711 |
| flock | paired | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-paired-n26-r3 | 15.642054 | 29.686898 | 551.332779 | 16.230632 | 6529.055634 | 890.631978 | 207.694529 | 894.542244 | 0.792867 | 9135.897063 |
| flock | scratch | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-scratch-n26-r1 | 14.902301 | 25.683755 | 596.239934 | 19.338823 | 6451.537747 | 887.965002 | 200.856307 | 883.390057 | 0.801868 | 9081.018537 |
| flock | scratch | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-scratch-n26-r2 | 15.060336 | 14.995164 | 606.868702 | 16.568759 | 6629.393250 | 827.099981 | 173.683994 | 917.939247 | 0.823154 | 9202.709196 |
| flock | scratch | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-scratch-n26-r3 | 15.738815 | 26.412361 | 544.414670 | 16.812307 | 6508.016883 | 879.619338 | 190.827843 | 918.661155 | 1.182988 | 9101.974891 |
| flock | all | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-all-n26-r1 | 16.202193 | 25.260520 | 750.244848 | 17.107719 | 6751.061002 | 860.293691 | 208.112727 | 922.045227 | 0.767285 | 9551.387055 |
| flock | all | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-all-n26-r2 | 14.719384 | 28.295519 | 577.982049 | 16.795621 | 6550.209738 | 878.267148 | 213.816249 | 927.090226 | 1.014435 | 9208.534123 |
| flock | all | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-all-n26-r3 | 15.551182 | 24.976147 | 553.112527 | 17.128286 | 6448.309032 | 856.904348 | 207.335677 | 851.570644 | 0.789516 | 8975.960924 |
| flock | previous_reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v10/evidence/wp_uninit-n26-r1 | 16.461846 | 27.628760 | 686.949686 | 17.367168 | 6627.795784 | 910.749301 | 194.004987 | 915.630937 | 0.835540 | 9397.706851 |
| flock | previous_reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v10/evidence/wp_uninit-n26-r2 | 16.592861 | 26.319729 | 666.523541 | 17.814721 | 6538.772944 | 913.034741 | 225.833744 | 871.071896 | 0.671976 | 9276.939609 |
| flock | previous_reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v10/evidence/wp_uninit-n26-r3 | 18.047509 | 26.152331 | 675.078344 | 19.664002 | 6746.160015 | 889.269993 | 168.776260 | 919.771228 | 0.677046 | 9463.887115 |

## multiple

F/M > 1 means Mikros is faster. Times are median [min, max] seconds.

| Stage | Arm | Mikros (s) | FLOCK (s) | M/F | F/M |
|---|---|---:|---:|---:|---:|
| Gate PIOP | reference | 1.3441 [1.3279, 1.4900] | 1.7602 [1.7579, 1.7805] | 0.764 | 1.310 |
| Gate PIOP | virtual | 1.2441 [1.2227, 1.2652] | 1.8041 [1.7812, 1.8741] | 0.690 | 1.450 |
| Gate PIOP | weights | 1.2275 [1.2186, 1.2401] | 1.7745 [1.7675, 1.8269] | 0.692 | 1.446 |
| Gate PIOP | paired | 1.3274 [1.3062, 1.4932] | 1.7674 [1.7526, 1.8061] | 0.751 | 1.331 |
| Gate PIOP | scratch | 1.1181 [1.1123, 1.1214] | 1.7735 [1.7091, 1.7813] | 0.630 | 1.586 |
| Gate PIOP | all | 1.0217 [1.0014, 1.1935] | 1.7912 [1.7591, 1.8374] | 0.570 | 1.753 |
| Gate PIOP | previous_reference | 1.3618 [1.3006, 1.3650] | 1.8144 [1.8010, 1.8514] | 0.751 | 1.332 |
| Copy PIOP | reference | 2.6298 [2.6249, 2.7656] | 6.4133 [6.3358, 6.6476] | 0.410 | 2.439 |
| Copy PIOP | virtual | 2.7019 [2.6781, 2.8604] | 6.6137 [6.5842, 6.8288] | 0.409 | 2.448 |
| Copy PIOP | weights | 2.7703 [2.6179, 2.8230] | 6.5779 [6.5494, 6.6254] | 0.421 | 2.374 |
| Copy PIOP | paired | 2.6724 [2.4967, 2.7060] | 6.4782 [6.4524, 6.6505] | 0.413 | 2.424 |
| Copy PIOP | scratch | 2.6133 [2.6079, 2.7460] | 6.5208 [6.4224, 6.6600] | 0.401 | 2.495 |
| Copy PIOP | all | 2.6895 [2.5556, 2.7784] | 6.4774 [6.3084, 6.8959] | 0.415 | 2.408 |
| Copy PIOP | previous_reference | 2.6993 [2.6586, 2.7282] | 6.7082 [6.5178, 6.7455] | 0.402 | 2.485 |
| Witness PCS opening | reference | 5.1054 [4.6022, 5.2045] | 5.2264 [5.1904, 5.5382] | 0.977 | 1.024 |
| Witness PCS opening | virtual | 4.7135 [4.6014, 5.0243] | 5.1594 [5.1166, 5.1740] | 0.914 | 1.095 |
| Witness PCS opening | weights | 4.6374 [4.5171, 4.9567] | 5.6988 [5.2048, 5.7741] | 0.814 | 1.229 |
| Witness PCS opening | paired | 4.6938 [4.5550, 4.9431] | 5.2105 [5.1706, 5.5955] | 0.901 | 1.110 |
| Witness PCS opening | scratch | 4.6134 [4.6073, 5.0603] | 5.2960 [5.2928, 5.5348] | 0.871 | 1.148 |
| Witness PCS opening | all | 4.6295 [4.5742, 5.1702] | 5.1781 [5.1235, 5.6335] | 0.894 | 1.119 |
| Witness PCS opening | previous_reference | 4.7468 [4.7385, 4.8667] | 5.4739 [5.3460, 5.5895] | 0.867 | 1.153 |
| Public PCS opening | reference | 1.3348 [1.3031, 1.3600] | 0.9922 [0.9892, 0.9958] | 1.345 | 0.743 |
| Public PCS opening | virtual | 1.3076 [1.2546, 1.3169] | 1.0067 [0.9985, 1.0324] | 1.299 | 0.770 |
| Public PCS opening | weights | 1.3001 [1.2950, 1.3275] | 1.0233 [1.0162, 1.0496] | 1.270 | 0.787 |
| Public PCS opening | paired | 1.2794 [1.2692, 1.2843] | 1.0147 [1.0125, 1.0686] | 1.261 | 0.793 |
| Public PCS opening | scratch | 1.2869 [1.2673, 1.3026] | 1.0325 [1.0316, 1.0610] | 1.246 | 0.802 |
| Public PCS opening | all | 1.2885 [1.2487, 1.3941] | 1.0080 [1.0062, 1.0162] | 1.278 | 0.782 |
| Public PCS opening | previous_reference | 1.2908 [1.2778, 1.3003] | 1.0651 [1.0471, 1.1206] | 1.212 | 0.825 |
| Commit and recommits | reference | 4.4109 [4.1283, 4.7171] | 4.2352 [4.2059, 4.4803] | 1.041 | 0.960 |
| Commit and recommits | virtual | 4.2908 [4.2797, 4.2992] | 4.3945 [4.1893, 4.5465] | 0.976 | 1.024 |
| Commit and recommits | weights | 4.3032 [4.2796, 4.4514] | 4.3537 [4.2769, 4.7356] | 0.988 | 1.012 |
| Commit and recommits | paired | 4.3196 [4.2283, 4.5420] | 4.4249 [4.2247, 4.4927] | 0.976 | 1.024 |
| Commit and recommits | scratch | 4.3602 [4.3592, 4.4410] | 4.3478 [4.2049, 4.4637] | 1.003 | 0.997 |
| Commit and recommits | all | 4.3754 [4.2295, 4.6653] | 4.2533 [4.1782, 4.3000] | 1.029 | 0.972 |
| Commit and recommits | previous_reference | 4.4532 [4.1866, 4.4903] | 4.7129 [4.4975, 4.7300] | 0.945 | 1.058 |
| Release | reference | 2.7621 [2.7589, 2.8459] | 3.2087 [3.1766, 3.2992] | 0.861 | 1.162 |
| Release | virtual | 2.8415 [2.8236, 2.9489] | 3.2629 [3.1899, 3.3901] | 0.871 | 1.148 |
| Release | weights | 2.8385 [2.7871, 2.8859] | 3.2388 [3.1855, 3.3516] | 0.876 | 1.141 |
| Release | paired | 2.7914 [2.7011, 2.8602] | 3.2755 [3.2751, 3.3253] | 0.852 | 1.173 |
| Release | scratch | 2.8534 [2.7712, 2.8872] | 3.3057 [3.2227, 3.3177] | 0.863 | 1.158 |
| Release | all | 2.7576 [2.6821, 2.8014] | 3.2584 [3.1986, 3.2723] | 0.846 | 1.182 |
| Release | previous_reference | 3.0432 [2.9899, 3.1031] | 3.4864 [3.3428, 3.5267] | 0.873 | 1.146 |
| Serialization | reference | 0.0007 [0.0007, 0.0007] | 0.0006 [0.0006, 0.0009] | 1.106 | 0.904 |
| Serialization | virtual | 0.0007 [0.0006, 0.0011] | 0.0009 [0.0006, 0.0010] | 0.737 | 1.356 |
| Serialization | weights | 0.0009 [0.0007, 0.0010] | 0.0008 [0.0006, 0.0010] | 1.111 | 0.900 |
| Serialization | paired | 0.0007 [0.0006, 0.0008] | 0.0006 [0.0006, 0.0006] | 1.056 | 0.947 |
| Serialization | scratch | 0.0011 [0.0006, 0.0021] | 0.0007 [0.0006, 0.0022] | 1.589 | 0.629 |
| Serialization | all | 0.0009 [0.0007, 0.0012] | 0.0007 [0.0007, 0.0009] | 1.156 | 0.865 |
| Serialization | previous_reference | 0.0007 [0.0007, 0.0009] | 0.0007 [0.0006, 0.0008] | 1.025 | 0.976 |
| Online total | reference | 17.6922 [16.8649, 18.1611] | 22.0857 [22.0341, 22.1167] | 0.801 | 1.248 |
| Online total | virtual | 17.1967 [16.8989, 17.5823] | 22.3805 [21.9834, 22.5864] | 0.768 | 1.301 |
| Online total | weights | 17.2054 [16.7218, 17.5532] | 22.6387 [22.2730, 23.1222] | 0.760 | 1.316 |
| Online total | paired | 17.1712 [16.8447, 17.4563] | 22.4504 [21.9858, 22.5641] | 0.765 | 1.307 |
| Online total | scratch | 16.9946 [16.8582, 17.2810] | 22.2370 [22.2090, 22.5364] | 0.764 | 1.308 |
| Online total | all | 16.7468 [16.6515, 17.6620] | 22.3990 [21.6089, 22.4911] | 0.748 | 1.338 |
| Online total | previous_reference | 17.4972 [17.3766, 17.7304] | 23.1933 [22.6992, 23.4874] | 0.754 | 1.326 |

Negative delta means less time; positive relative F/M gain improves Mikros's relative advantage.

| Stage | Arm | Mikros delta (s) | Mikros reduction | FLOCK delta (s) | FLOCK reduction | Relative F/M gain |
|---|---|---:|---:|---:|---:|---:|
| Gate PIOP | virtual | -0.1000 | +7.44% | +0.0439 | -2.49% | +10.73% |
| Gate PIOP | weights | -0.1166 | +8.67% | +0.0143 | -0.81% | +10.39% |
| Gate PIOP | paired | -0.0167 | +1.24% | +0.0072 | -0.41% | +1.67% |
| Gate PIOP | scratch | -0.2260 | +16.81% | +0.0133 | -0.76% | +21.12% |
| Gate PIOP | all | -0.3223 | +23.98% | +0.0310 | -1.76% | +33.86% |
| Copy PIOP | virtual | +0.0722 | -2.75% | +0.2004 | -3.12% | +0.37% |
| Copy PIOP | weights | +0.1406 | -5.34% | +0.1647 | -2.57% | -2.64% |
| Copy PIOP | paired | +0.0426 | -1.62% | +0.0650 | -1.01% | -0.60% |
| Copy PIOP | scratch | -0.0164 | +0.62% | +0.1075 | -1.68% | +2.32% |
| Copy PIOP | all | +0.0598 | -2.27% | +0.0642 | -1.00% | -1.24% |
| Witness PCS opening | virtual | -0.3919 | +7.68% | -0.0671 | +1.28% | +6.92% |
| Witness PCS opening | weights | -0.4680 | +9.17% | +0.4724 | -9.04% | +20.04% |
| Witness PCS opening | paired | -0.4116 | +8.06% | -0.0160 | +0.31% | +8.44% |
| Witness PCS opening | scratch | -0.4920 | +9.64% | +0.0695 | -1.33% | +12.14% |
| Witness PCS opening | all | -0.4759 | +9.32% | -0.0484 | +0.93% | +9.26% |
| Public PCS opening | virtual | -0.0272 | +2.04% | +0.0146 | -1.47% | +3.58% |
| Public PCS opening | weights | -0.0347 | +2.60% | +0.0312 | -3.14% | +5.89% |
| Public PCS opening | paired | -0.0554 | +4.15% | +0.0225 | -2.27% | +6.70% |
| Public PCS opening | scratch | -0.0479 | +3.59% | +0.0404 | -4.07% | +7.94% |
| Public PCS opening | all | -0.0463 | +3.47% | +0.0158 | -1.59% | +5.24% |
| Commit and recommits | virtual | -0.1202 | +2.72% | +0.1593 | -3.76% | +6.67% |
| Commit and recommits | weights | -0.1077 | +2.44% | +0.1185 | -2.80% | +5.37% |
| Commit and recommits | paired | -0.0913 | +2.07% | +0.1896 | -4.48% | +6.69% |
| Commit and recommits | scratch | -0.0507 | +1.15% | +0.1125 | -2.66% | +3.85% |
| Commit and recommits | all | -0.0355 | +0.81% | +0.0181 | -0.43% | +1.24% |
| Release | virtual | +0.0795 | -2.88% | +0.0542 | -1.69% | -1.15% |
| Release | weights | +0.0765 | -2.77% | +0.0301 | -0.94% | -1.78% |
| Release | paired | +0.0294 | -1.06% | +0.0667 | -2.08% | +1.01% |
| Release | scratch | +0.0914 | -3.31% | +0.0969 | -3.02% | -0.28% |
| Release | all | -0.0044 | +0.16% | +0.0497 | -1.55% | +1.71% |
| Serialization | virtual | -0.0000 | +4.22% | +0.0003 | -43.66% | +49.99% |
| Serialization | weights | +0.0002 | -24.62% | +0.0002 | -24.06% | -0.45% |
| Serialization | paired | -0.0000 | +3.63% | +0.0000 | -0.95% | +4.75% |
| Serialization | scratch | +0.0004 | -51.11% | +0.0000 | -5.21% | -30.37% |
| Serialization | all | +0.0002 | -22.04% | +0.0001 | -16.79% | -4.30% |
| Online total | virtual | -0.4955 | +2.80% | +0.2948 | -1.33% | +4.25% |
| Online total | weights | -0.4868 | +2.75% | +0.5530 | -2.50% | +5.40% |
| Online total | paired | -0.5210 | +2.94% | +0.3647 | -1.65% | +4.74% |
| Online total | scratch | -0.6976 | +3.94% | +0.1513 | -0.69% | +4.82% |
| Online total | all | -0.9454 | +5.34% | +0.3134 | -1.42% | +7.14% |

| System | Arm | Samples | Proof bytes | Logical W/P | Ring W/P | Verify ms [range] | Whole-process RSS GiB [range] |
|---|---|---:|---:|---:|---:|---:|---:|
| mikros | reference | 3 | 1202103 | 11/8 | 2/8 | 16.611 [16.009, 16.638] | 26.274 [26.274, 26.275] |
| mikros | virtual | 3 | 1202103 | 11/8 | 2/8 | 16.193 [15.868, 20.102] | 26.275 [26.274, 26.279] |
| mikros | weights | 3 | 1202103 | 11/8 | 2/8 | 15.439 [15.114, 16.777] | 26.278 [26.274, 26.279] |
| mikros | paired | 3 | 1202103 | 11/8 | 2/8 | 17.510 [16.528, 17.570] | 26.276 [26.276, 26.278] |
| mikros | scratch | 3 | 1202103 | 11/8 | 2/8 | 16.290 [14.428, 18.191] | 26.277 [26.277, 26.280] |
| mikros | all | 3 | 1202103 | 11/8 | 2/8 | 16.705 [15.537, 16.925] | 26.278 [26.275, 26.279] |
| mikros | previous_reference | 3 | 1202103 | 11/8 | 2/8 | 17.115 [15.943, 18.680] | 26.276 [26.275, 26.278] |
| flock | reference | 3 | 1160639 | 7/3 | 4/3 | 17.007 [15.033, 17.926] | 26.310 [26.307, 26.311] |
| flock | virtual | 3 | 1160639 | 7/3 | 4/3 | 16.542 [15.520, 16.582] | 26.310 [26.308, 26.310] |
| flock | weights | 3 | 1160639 | 7/3 | 4/3 | 15.689 [14.793, 16.876] | 26.308 [26.306, 26.308] |
| flock | paired | 3 | 1160639 | 7/3 | 4/3 | 16.594 [15.088, 17.479] | 26.309 [26.309, 26.310] |
| flock | scratch | 3 | 1160639 | 7/3 | 4/3 | 15.522 [15.518, 16.246] | 26.309 [26.306, 26.310] |
| flock | all | 3 | 1160639 | 7/3 | 4/3 | 15.667 [14.639, 16.047] | 26.308 [26.308, 26.310] |
| flock | previous_reference | 3 | 1160639 | 7/3 | 4/3 | 15.131 [15.097, 16.873] | 26.308 [26.307, 26.308] |

Raw primary phase timings, milliseconds (one row per fresh process):

| System | Arm | Directory | commit_ms | recommit_ms | public_recommit_ms | gates_ms | copy_ms | release_ms | witness_pcs_open_ms | public_pcs_open_ms | serialization_ms | prover_ms |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| mikros | reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-reference-n26-r1 | 1681.434839 | 1895.815472 | 551.092905 | 1327.853199 | 2624.852186 | 2845.882969 | 4602.223644 | 1334.790787 | 0.697555 | 16864.933138 |
| mikros | reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-reference-n26-r2 | 2380.530807 | 1776.090834 | 560.451535 | 1489.965564 | 2629.754283 | 2758.876875 | 5204.536250 | 1359.975937 | 0.670424 | 18161.145793 |
| mikros | reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-reference-n26-r3 | 1845.521761 | 2058.531912 | 506.887855 | 1344.061583 | 2765.649312 | 2762.061454 | 5105.391680 | 1303.083163 | 0.726366 | 17692.190873 |
| mikros | virtual | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-virtual-n26-r1 | 1789.361648 | 1980.357413 | 510.012997 | 1244.096490 | 2701.941440 | 2948.865680 | 4713.537547 | 1307.586859 | 0.646298 | 17196.693666 |
| mikros | virtual | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-virtual-n26-r2 | 1827.875390 | 1878.114527 | 593.167311 | 1222.670896 | 2678.118684 | 2841.516352 | 4601.441343 | 1254.623830 | 1.055731 | 16898.893943 |
| mikros | virtual | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-virtual-n26-r3 | 1838.739375 | 1837.792113 | 614.240864 | 1265.228347 | 2860.438419 | 2823.645989 | 5024.339839 | 1316.869627 | 0.668091 | 17582.262389 |
| mikros | weights | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-weights-n26-r1 | 1845.906106 | 1867.228478 | 566.437106 | 1218.626793 | 2617.880575 | 2787.147664 | 4517.135873 | 1300.096222 | 1.045941 | 16721.781867 |
| mikros | weights | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-weights-n26-r2 | 1961.658130 | 1923.931767 | 565.803377 | 1240.129182 | 2770.305377 | 2838.520835 | 4956.742590 | 1294.974239 | 0.869319 | 17553.230206 |
| mikros | weights | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-weights-n26-r3 | 1871.048235 | 1896.806571 | 535.381044 | 1227.494773 | 2822.951319 | 2885.874219 | 4637.353033 | 1327.476959 | 0.700417 | 17205.364092 |
| mikros | paired | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-paired-n26-r1 | 1822.150420 | 1979.433925 | 518.011879 | 1306.171764 | 2706.012006 | 2860.193647 | 4693.803189 | 1284.271514 | 0.764682 | 17171.169542 |
| mikros | paired | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-paired-n26-r2 | 2270.168831 | 1706.819356 | 564.972959 | 1493.191132 | 2496.659338 | 2701.070446 | 4943.142995 | 1279.350272 | 0.672233 | 17456.324220 |
| mikros | paired | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-paired-n26-r3 | 1811.181663 | 1839.599599 | 577.534629 | 1327.391487 | 2672.391080 | 2791.425193 | 4555.032251 | 1269.237086 | 0.635535 | 16844.702509 |
| mikros | scratch | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-scratch-n26-r1 | 2029.895691 | 1874.706991 | 536.420305 | 1112.285484 | 2607.913025 | 2771.226313 | 5060.286504 | 1286.871881 | 1.054060 | 17280.977955 |
| mikros | scratch | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-scratch-n26-r2 | 1791.952518 | 1908.348988 | 658.884191 | 1121.391202 | 2613.329920 | 2853.417374 | 4607.326349 | 1302.625426 | 0.634025 | 16858.209002 |
| mikros | scratch | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-scratch-n26-r3 | 1811.413341 | 1960.105789 | 588.686277 | 1118.092635 | 2746.004680 | 2887.178575 | 4613.391197 | 1267.302847 | 2.099936 | 16994.586572 |
| mikros | all | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-all-n26-r1 | 2314.104034 | 1772.269823 | 578.966918 | 1193.513778 | 2555.555594 | 2682.091261 | 5170.236317 | 1394.109792 | 0.851285 | 17662.001381 |
| mikros | all | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-all-n26-r2 | 1785.350799 | 1861.569950 | 582.626071 | 1021.732227 | 2778.390569 | 2757.631718 | 4574.246138 | 1288.495493 | 1.163647 | 16651.505889 |
| mikros | all | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-all-n26-r3 | 1914.460708 | 1984.365439 | 476.581622 | 1001.354704 | 2689.523352 | 2801.413086 | 4629.468187 | 1248.688351 | 0.692718 | 16746.828638 |
| mikros | previous_reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v10/evidence/wp_uninit-n26-r1 | 2025.692031 | 1983.674431 | 443.801947 | 1364.970457 | 2658.599931 | 2989.929269 | 4738.490461 | 1290.806503 | 0.910302 | 17497.249972 |
| mikros | previous_reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v10/evidence/wp_uninit-n26-r2 | 1869.595935 | 2102.783176 | 517.915691 | 1300.608925 | 2728.176307 | 3043.240432 | 4866.671856 | 1300.311522 | 0.746313 | 17730.353670 |
| mikros | previous_reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v10/evidence/wp_uninit-n26-r3 | 1797.026103 | 1867.605673 | 521.947719 | 1361.782883 | 2699.276668 | 3103.144275 | 4746.779805 | 1277.841460 | 0.690590 | 17376.605740 |
| flock | reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-reference-n26-r1 | 1810.695357 | 1880.445335 | 544.086470 | 1757.854569 | 6335.775395 | 3176.642622 | 5538.184569 | 989.218917 | 0.906147 | 22034.099433 |
| flock | reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-reference-n26-r2 | 1986.160145 | 2060.578491 | 433.565369 | 1760.192479 | 6413.262928 | 3208.729345 | 5226.449502 | 995.792402 | 0.630618 | 22085.656427 |
| flock | reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-reference-n26-r3 | 1935.283338 | 1795.343942 | 475.278715 | 1780.465482 | 6647.610977 | 3299.189384 | 5190.412927 | 992.154167 | 0.615295 | 22116.651335 |
| flock | virtual | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-virtual-n26-r1 | 1938.462690 | 2059.536274 | 548.469413 | 1781.153448 | 6828.812117 | 3262.911924 | 5159.396636 | 1006.726496 | 0.639861 | 22586.394449 |
| flock | virtual | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-virtual-n26-r2 | 1738.371482 | 2044.572207 | 611.585404 | 1804.107934 | 6584.199032 | 3390.083872 | 5174.010610 | 1032.358550 | 0.905925 | 22380.480779 |
| flock | virtual | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-virtual-n26-r3 | 1810.085652 | 1839.005596 | 540.221470 | 1874.057401 | 6613.671555 | 3189.922194 | 5116.641784 | 998.514471 | 1.007690 | 21983.419633 |
| flock | weights | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-weights-n26-r1 | 1962.488254 | 1817.643940 | 573.547689 | 1767.499783 | 6577.921656 | 3351.588632 | 5204.813203 | 1016.176727 | 1.024477 | 22273.006377 |
| flock | weights | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-weights-n26-r2 | 1887.591440 | 1828.443443 | 560.836585 | 1774.535269 | 6625.370382 | 3238.831861 | 5698.839074 | 1023.315537 | 0.634872 | 22638.688934 |
| flock | weights | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-weights-n26-r3 | 2340.897295 | 1782.329437 | 612.398812 | 1826.935738 | 6549.367150 | 3185.505418 | 5774.086511 | 1049.566265 | 0.782368 | 23122.170692 |
| flock | paired | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-paired-n26-r1 | 1899.959264 | 1788.939066 | 535.794231 | 1767.356634 | 6478.229561 | 3275.457444 | 5170.595315 | 1068.584358 | 0.641242 | 21985.839442 |
| flock | paired | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-paired-n26-r2 | 2041.484310 | 1885.028207 | 498.339177 | 1752.586892 | 6452.414204 | 3325.274685 | 5595.512366 | 1012.490806 | 0.636603 | 22564.072618 |
| flock | paired | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-paired-n26-r3 | 2037.934208 | 1893.565378 | 561.242378 | 1806.054814 | 6650.514578 | 3275.057994 | 5210.468673 | 1014.651113 | 0.608502 | 22450.381281 |
| flock | scratch | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-scratch-n26-r1 | 1788.885517 | 1845.059668 | 570.962554 | 1709.079012 | 6659.952797 | 3305.676438 | 5295.983133 | 1032.525367 | 0.612042 | 22209.044716 |
| flock | scratch | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-scratch-n26-r2 | 1951.714415 | 1951.847355 | 560.118756 | 1773.484447 | 6422.375602 | 3222.691497 | 5292.765580 | 1061.043839 | 0.663489 | 22237.002229 |
| flock | scratch | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-scratch-n26-r3 | 1794.695358 | 2000.786256 | 552.287617 | 1781.285362 | 6520.776913 | 3317.650707 | 5534.784166 | 1031.570931 | 2.191254 | 22536.365605 |
| flock | all | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-all-n26-r1 | 1862.547510 | 1823.109995 | 567.687456 | 1759.085439 | 6477.449694 | 3258.435064 | 5633.500558 | 1016.195626 | 0.736506 | 22399.032466 |
| flock | all | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-all-n26-r2 | 1968.793276 | 1875.453760 | 455.754055 | 1837.358225 | 6895.899374 | 3272.307312 | 5178.089652 | 1006.227642 | 0.874303 | 22491.136116 |
| flock | all | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v11/evidence/primary-all-n26-r3 | 1689.901390 | 1824.122339 | 664.206620 | 1791.186166 | 6308.431673 | 3198.605198 | 5123.546949 | 1007.954121 | 0.655507 | 21608.909058 |
| flock | previous_reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v10/evidence/wp_uninit-n26-r1 | 2296.596916 | 1836.817087 | 579.510201 | 1851.384013 | 6708.184260 | 3526.748045 | 5345.963579 | 1047.068111 | 0.728177 | 23193.309018 |
| flock | previous_reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v10/evidence/wp_uninit-n26-r2 | 2270.624873 | 1903.700433 | 555.710380 | 1814.356094 | 6745.508941 | 3486.380083 | 5589.458580 | 1120.575252 | 0.818599 | 23487.439254 |
| flock | previous_reference | /mnt/c/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/v10/evidence/wp_uninit-n26-r3 | 2259.555582 | 1776.898602 | 461.074350 | 1801.028094 | 6517.795723 | 3342.814614 | 5473.914195 | 1065.106079 | 0.630543 | 22699.152761 |

comparison.json also retains all raw resource records, complete hashes, parameters, prior-reference differences and SHA-256 digests of read evidence files.
