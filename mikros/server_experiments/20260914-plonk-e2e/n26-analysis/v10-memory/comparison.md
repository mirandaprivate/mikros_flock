# N26 four-arm memory-traffic ablation

single: N26 gates; multiple: 128 circuits each with N26 gates

Online commit/recommit, conversion, PIOPs, PCS and release included; offline excluded

Median [min, max] of at least three fresh processes per arm/case; phase medians need not sum to the total median

Descriptive samples, not a significance claim; trace runs are rejected rather than counted

PackedDirect is ON throughout. FLOCK_DIRECT_WP_SIMD and MIKROS_GATE_SELECTOR_SIMD are OFF. WP uninitialized output applies equally to Mikros and FLOCK; relative benefit is assessed from measured ratios.
Current arms share one frozen source/binary and produce identical complete proof hashes/bytes, PIOPs, claims, commitments, PCS parameters and logical/ring counts.

| Arm | Extra opt-ins |
|---|---|
| reference | none |
| fusion | MIKROS_GATE_FUSION=1 |
| wp_uninit | FLOCK_WP_UNINIT=1 |
| both | MIKROS_GATE_FUSION=1, FLOCK_WP_UNINIT=1 |

Previous reference is separate evidence: only its source/binary may differ; full proofs and all other audited inputs match current reference.

## single

M/F < 1 and F/M > 1 mean Mikros is faster. Times are median [min, max] seconds.

| Stage | Arm | Mikros (s) | FLOCK (s) | M/F | F/M |
|---|---|---:|---:|---:|---:|
| Gate PIOP | reference | 0.0799 [0.0672, 0.1047] | 0.0173 [0.0164, 0.0181] | 4.609 | 0.217 |
| Gate PIOP | fusion | 0.0711 [0.0679, 0.0772] | 0.0175 [0.0169, 0.0194] | 4.060 | 0.246 |
| Gate PIOP | wp_uninit | 0.0694 [0.0653, 0.0777] | 0.0178 [0.0174, 0.0197] | 3.895 | 0.257 |
| Gate PIOP | both | 0.0893 [0.0861, 0.0964] | 0.0168 [0.0165, 0.0172] | 5.297 | 0.189 |
| Gate PIOP | previous_reference | 0.0644 [0.0637, 0.0815] | 0.0167 [0.0167, 0.0176] | 3.857 | 0.259 |
| Copy PIOP | reference | 1.4293 [1.3885, 1.4354] | 6.4877 [6.4560, 6.8480] | 0.220 | 4.539 |
| Copy PIOP | fusion | 1.4102 [1.3366, 1.4292] | 6.6339 [6.5380, 6.8298] | 0.213 | 4.704 |
| Copy PIOP | wp_uninit | 1.4375 [1.3828, 1.4474] | 6.6278 [6.5388, 6.7462] | 0.217 | 4.611 |
| Copy PIOP | both | 1.4424 [1.4332, 1.4446] | 6.5879 [6.5093, 6.5950] | 0.219 | 4.567 |
| Copy PIOP | previous_reference | 1.4219 [1.4103, 1.4368] | 6.6151 [6.3425, 6.6171] | 0.215 | 4.652 |
| Witness PCS opening | reference | 0.2401 [0.2391, 0.2541] | 0.2713 [0.2136, 0.2748] | 0.885 | 1.130 |
| Witness PCS opening | fusion | 0.2420 [0.2275, 0.2564] | 0.2086 [0.1954, 0.2310] | 1.160 | 0.862 |
| Witness PCS opening | wp_uninit | 0.2158 [0.2073, 0.2259] | 0.1940 [0.1688, 0.2258] | 1.112 | 0.899 |
| Witness PCS opening | both | 0.2120 [0.2091, 0.2252] | 0.1640 [0.1629, 0.1685] | 1.292 | 0.774 |
| Witness PCS opening | previous_reference | 0.2785 [0.2643, 0.2801] | 0.2304 [0.2189, 0.2526] | 1.209 | 0.827 |
| Public PCS opening | reference | 1.2137 [1.1956, 1.2265] | 0.9081 [0.8895, 0.9150] | 1.336 | 0.748 |
| Public PCS opening | fusion | 1.1839 [1.1784, 1.2268] | 0.8733 [0.8616, 0.8747] | 1.356 | 0.738 |
| Public PCS opening | wp_uninit | 1.1762 [1.1413, 1.2052] | 0.9156 [0.8711, 0.9198] | 1.285 | 0.778 |
| Public PCS opening | both | 1.1502 [1.1494, 1.1740] | 0.9687 [0.9253, 0.9766] | 1.187 | 0.842 |
| Public PCS opening | previous_reference | 1.1981 [1.1276, 1.2036] | 0.8821 [0.8691, 0.8823] | 1.358 | 0.736 |
| Commit and recommits | reference | 0.6197 [0.5662, 0.6565] | 0.7475 [0.5948, 0.7855] | 0.829 | 1.206 |
| Commit and recommits | fusion | 0.6039 [0.5675, 0.6454] | 0.6407 [0.6158, 0.6673] | 0.943 | 1.061 |
| Commit and recommits | wp_uninit | 0.6256 [0.6117, 0.6688] | 0.7193 [0.7094, 0.7310] | 0.870 | 1.150 |
| Commit and recommits | both | 0.6439 [0.6155, 0.6942] | 0.5288 [0.5218, 0.6189] | 1.218 | 0.821 |
| Commit and recommits | previous_reference | 0.6108 [0.6026, 0.6273] | 0.6188 [0.5939, 0.6811] | 0.987 | 1.013 |
| Release | reference | 1.5934 [1.5904, 1.6661] | 0.9171 [0.9065, 0.9273] | 1.737 | 0.576 |
| Release | fusion | 1.6692 [1.6412, 1.6722] | 0.8969 [0.8864, 0.9110] | 1.861 | 0.537 |
| Release | wp_uninit | 1.6740 [1.6384, 1.6855] | 0.9107 [0.8893, 0.9130] | 1.838 | 0.544 |
| Release | both | 1.6351 [1.6113, 1.6390] | 0.9022 [0.8983, 0.9220] | 1.812 | 0.552 |
| Release | previous_reference | 1.5969 [1.5921, 1.6332] | 0.9228 [0.9186, 0.9586] | 1.731 | 0.578 |
| Serialization | reference | 0.0011 [0.0010, 0.0013] | 0.0007 [0.0007, 0.0008] | 1.527 | 0.655 |
| Serialization | fusion | 0.0009 [0.0009, 0.0009] | 0.0012 [0.0008, 0.0015] | 0.765 | 1.306 |
| Serialization | wp_uninit | 0.0009 [0.0007, 0.0011] | 0.0007 [0.0007, 0.0008] | 1.307 | 0.765 |
| Serialization | both | 0.0009 [0.0007, 0.0009] | 0.0007 [0.0007, 0.0007] | 1.358 | 0.737 |
| Serialization | previous_reference | 0.0011 [0.0010, 0.0012] | 0.0008 [0.0008, 0.0008] | 1.375 | 0.728 |
| Online total | reference | 5.1964 [5.1701, 5.2042] | 9.3641 [9.3025, 9.5310] | 0.555 | 1.802 |
| Online total | fusion | 5.1790 [5.0539, 5.2776] | 9.2727 [9.1554, 9.4946] | 0.559 | 1.790 |
| Online total | wp_uninit | 5.1965 [5.1183, 5.2447] | 9.3977 [9.2769, 9.4639] | 0.553 | 1.808 |
| Online total | both | 5.1755 [5.1433, 5.2355] | 9.1719 [9.1037, 9.2281] | 0.564 | 1.772 |
| Online total | previous_reference | 5.1819 [5.0893, 5.2271] | 9.3204 [8.9981, 9.3400] | 0.556 | 1.799 |

Positive time reduction means faster; negative delta means less time. Positive relative F/M gain improves Mikros's relative advantage.

| Stage | Arm | Mikros delta (s) | Mikros reduction | M reference/arm | FLOCK delta (s) | FLOCK reduction | F reference/arm | Relative F/M gain |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Gate PIOP | fusion | -0.0088 | +11.00% | 1.124 | +0.0002 | -1.04% | 0.990 | +13.54% |
| Gate PIOP | wp_uninit | -0.0105 | +13.10% | 1.151 | +0.0005 | -2.83% | 0.973 | +18.33% |
| Gate PIOP | both | +0.0094 | -11.78% | 0.895 | -0.0005 | +2.74% | 1.028 | -12.99% |
| Copy PIOP | fusion | -0.0191 | +1.34% | 1.014 | +0.1462 | -2.25% | 0.978 | +3.64% |
| Copy PIOP | wp_uninit | +0.0082 | -0.58% | 0.994 | +0.1400 | -2.16% | 0.979 | +1.57% |
| Copy PIOP | both | +0.0131 | -0.92% | 0.991 | +0.1001 | -1.54% | 0.985 | +0.62% |
| Witness PCS opening | fusion | +0.0020 | -0.82% | 0.992 | -0.0627 | +23.12% | 1.301 | -23.74% |
| Witness PCS opening | wp_uninit | -0.0242 | +10.10% | 1.112 | -0.0773 | +28.50% | 1.399 | -20.47% |
| Witness PCS opening | both | -0.0281 | +11.70% | 1.132 | -0.1073 | +39.56% | 1.654 | -31.55% |
| Public PCS opening | fusion | -0.0297 | +2.45% | 1.025 | -0.0348 | +3.83% | 1.040 | -1.41% |
| Public PCS opening | wp_uninit | -0.0374 | +3.08% | 1.032 | +0.0075 | -0.83% | 0.992 | +4.04% |
| Public PCS opening | both | -0.0635 | +5.23% | 1.055 | +0.0606 | -6.67% | 0.937 | +12.56% |
| Commit and recommits | fusion | -0.0158 | +2.55% | 1.026 | -0.1068 | +14.29% | 1.167 | -12.04% |
| Commit and recommits | wp_uninit | +0.0058 | -0.94% | 0.991 | -0.0282 | +3.77% | 1.039 | -4.67% |
| Commit and recommits | both | +0.0242 | -3.91% | 0.962 | -0.2187 | +29.26% | 1.414 | -31.92% |
| Release | fusion | +0.0757 | -4.75% | 0.955 | -0.0202 | +2.20% | 1.023 | -6.64% |
| Release | wp_uninit | +0.0806 | -5.06% | 0.952 | -0.0063 | +0.69% | 1.007 | -5.47% |
| Release | both | +0.0416 | -2.61% | 0.975 | -0.0149 | +1.63% | 1.017 | -4.13% |
| Serialization | fusion | -0.0002 | +21.25% | 1.270 | +0.0004 | -57.10% | 0.637 | +99.49% |
| Serialization | wp_uninit | -0.0003 | +22.09% | 1.283 | -0.0001 | +9.00% | 1.099 | +16.79% |
| Serialization | both | -0.0002 | +20.34% | 1.255 | -0.0001 | +10.42% | 1.116 | +12.46% |
| Online total | fusion | -0.0174 | +0.33% | 1.003 | -0.0914 | +0.98% | 1.010 | -0.64% |
| Online total | wp_uninit | +0.0002 | -0.00% | 1.000 | +0.0336 | -0.36% | 0.996 | +0.36% |
| Online total | both | -0.0208 | +0.40% | 1.004 | -0.1922 | +2.05% | 1.021 | -1.66% |

| System | Arm | Samples | Proof bytes | Logical W/P | Ring W/P | Verify median [range] ms | RSS median [range] GiB |
|---|---|---:|---:|---:|---:|---:|---:|
| mikros | reference | 3 | 1296879 | 11/89 | 11/89 | 46.857 [46.735, 47.810] | 17.521 [17.520, 17.522] |
| mikros | fusion | 3 | 1296879 | 11/89 | 11/89 | 46.014 [43.826, 46.784] | 17.522 [17.520, 17.524] |
| mikros | wp_uninit | 3 | 1296879 | 11/89 | 11/89 | 46.347 [44.282, 47.561] | 17.522 [17.518, 17.522] |
| mikros | both | 3 | 1296879 | 11/89 | 11/89 | 46.880 [44.338, 47.601] | 17.521 [17.520, 17.522] |
| mikros | previous_reference | 3 | 1296879 | 11/89 | 11/89 | 45.611 [45.043, 46.610] | 17.521 [17.518, 17.521] |
| flock | reference | 3 | 1215735 | 7/84 | 7/84 | 39.832 [39.663, 40.965] | 21.027 [21.026, 21.030] |
| flock | fusion | 3 | 1215735 | 7/84 | 7/84 | 39.327 [39.238, 41.349] | 21.029 [21.029, 21.030] |
| flock | wp_uninit | 3 | 1215735 | 7/84 | 7/84 | 39.473 [38.155, 40.506] | 21.031 [21.024, 21.032] |
| flock | both | 3 | 1215735 | 7/84 | 7/84 | 41.958 [41.099, 42.655] | 21.030 [21.028, 21.031] |
| flock | previous_reference | 3 | 1215735 | 7/84 | 7/84 | 40.982 [39.108, 41.995] | 21.030 [21.029, 21.030] |

mikros reference raw online milliseconds, one per process: [5204.238955, 5170.1292570000005, 5196.376817].
mikros fusion raw online milliseconds, one per process: [5277.55888, 5053.8875259999995, 5179.0196399999995].
mikros wp_uninit raw online milliseconds, one per process: [5196.531319, 5118.3025259999995, 5244.711783].
mikros both raw online milliseconds, one per process: [5235.509233, 5143.304763, 5175.538218].
mikros previous_reference raw online milliseconds, one per process: [5089.311719, 5181.852035000001, 5227.06854].
flock reference raw online milliseconds, one per process: [9364.059336, 9531.023032000001, 9302.459404000001].
flock fusion raw online milliseconds, one per process: [9272.693997999999, 9155.404055, 9494.623608].
flock wp_uninit raw online milliseconds, one per process: [9397.706851, 9276.939609, 9463.887115].
flock both raw online milliseconds, one per process: [9103.683962, 9171.898215000001, 9228.105813999999].
flock previous_reference raw online milliseconds, one per process: [9340.020558999999, 8998.093139, 9320.41936].

## multiple

M/F < 1 and F/M > 1 mean Mikros is faster. Times are median [min, max] seconds.

| Stage | Arm | Mikros (s) | FLOCK (s) | M/F | F/M |
|---|---|---:|---:|---:|---:|
| Gate PIOP | reference | 1.4673 [1.3520, 1.5385] | 1.8491 [1.7732, 1.8681] | 0.794 | 1.260 |
| Gate PIOP | fusion | 1.2746 [1.2445, 1.4713] | 1.8026 [1.6774, 1.8254] | 0.707 | 1.414 |
| Gate PIOP | wp_uninit | 1.3618 [1.3006, 1.3650] | 1.8144 [1.8010, 1.8514] | 0.751 | 1.332 |
| Gate PIOP | both | 1.4948 [1.4487, 1.5513] | 1.8098 [1.8085, 1.8378] | 0.826 | 1.211 |
| Gate PIOP | previous_reference | 1.3258 [1.3170, 1.5216] | 1.7367 [1.7217, 1.8257] | 0.763 | 1.310 |
| Copy PIOP | reference | 2.9035 [2.6966, 2.9186] | 6.7274 [6.4510, 6.8413] | 0.432 | 2.317 |
| Copy PIOP | fusion | 2.7050 [2.7015, 2.9507] | 6.7311 [6.6196, 6.7810] | 0.402 | 2.488 |
| Copy PIOP | wp_uninit | 2.6993 [2.6586, 2.7282] | 6.7082 [6.5178, 6.7455] | 0.402 | 2.485 |
| Copy PIOP | both | 2.9372 [2.9082, 2.9505] | 6.6018 [6.4915, 6.6274] | 0.445 | 2.248 |
| Copy PIOP | previous_reference | 2.7112 [2.7018, 2.9484] | 6.6273 [6.6015, 6.6910] | 0.409 | 2.444 |
| Witness PCS opening | reference | 5.7081 [5.6256, 6.0984] | 6.1160 [5.9879, 6.5156] | 0.933 | 1.071 |
| Witness PCS opening | fusion | 5.6429 [5.4872, 5.7466] | 6.1321 [5.9498, 6.4453] | 0.920 | 1.087 |
| Witness PCS opening | wp_uninit | 4.7468 [4.7385, 4.8667] | 5.4739 [5.3460, 5.5895] | 0.867 | 1.153 |
| Witness PCS opening | both | 5.0811 [4.8906, 5.0853] | 5.4651 [5.2420, 5.6417] | 0.930 | 1.076 |
| Witness PCS opening | previous_reference | 5.6742 [5.6621, 5.7493] | 6.1188 [6.0731, 6.2712] | 0.927 | 1.078 |
| Public PCS opening | reference | 1.3395 [1.2596, 1.3844] | 1.1094 [1.0368, 1.1258] | 1.207 | 0.828 |
| Public PCS opening | fusion | 1.3369 [1.2971, 1.3481] | 1.0983 [1.0209, 1.1018] | 1.217 | 0.822 |
| Public PCS opening | wp_uninit | 1.2908 [1.2778, 1.3003] | 1.0651 [1.0471, 1.1206] | 1.212 | 0.825 |
| Public PCS opening | both | 1.3387 [1.3135, 1.3904] | 1.1087 [1.0593, 1.1282] | 1.207 | 0.828 |
| Public PCS opening | previous_reference | 1.2986 [1.2756, 1.4309] | 1.0724 [1.0086, 1.0986] | 1.211 | 0.826 |
| Commit and recommits | reference | 4.8272 [4.2504, 4.9660] | 4.7079 [4.3919, 4.9388] | 1.025 | 0.975 |
| Commit and recommits | fusion | 4.3713 [4.1907, 4.6845] | 4.2788 [4.0968, 4.8929] | 1.022 | 0.979 |
| Commit and recommits | wp_uninit | 4.4532 [4.1866, 4.4903] | 4.7129 [4.4975, 4.7300] | 0.945 | 1.058 |
| Commit and recommits | both | 4.7552 [4.6883, 4.7822] | 4.6832 [4.6488, 4.7804] | 1.015 | 0.985 |
| Commit and recommits | previous_reference | 4.4692 [4.4120, 4.8132] | 4.8330 [4.3166, 4.9401] | 0.925 | 1.081 |
| Release | reference | 3.0261 [2.9124, 3.0508] | 3.4617 [3.3792, 3.5966] | 0.874 | 1.144 |
| Release | fusion | 2.9903 [2.9777, 3.0611] | 3.3709 [3.3173, 3.5490] | 0.887 | 1.127 |
| Release | wp_uninit | 3.0432 [2.9899, 3.1031] | 3.4864 [3.3428, 3.5267] | 0.873 | 1.146 |
| Release | both | 2.9708 [2.8210, 3.0588] | 3.4158 [3.3353, 3.4997] | 0.870 | 1.150 |
| Release | previous_reference | 2.9523 [2.8566, 3.0607] | 3.4279 [3.4263, 3.4547] | 0.861 | 1.161 |
| Serialization | reference | 0.0009 [0.0008, 0.0010] | 0.0007 [0.0006, 0.0009] | 1.193 | 0.838 |
| Serialization | fusion | 0.0007 [0.0006, 0.0007] | 0.0009 [0.0006, 0.0009] | 0.764 | 1.309 |
| Serialization | wp_uninit | 0.0007 [0.0007, 0.0009] | 0.0007 [0.0006, 0.0008] | 1.025 | 0.976 |
| Serialization | both | 0.0009 [0.0008, 0.0012] | 0.0008 [0.0006, 0.0010] | 1.213 | 0.824 |
| Serialization | previous_reference | 0.0007 [0.0007, 0.0008] | 0.0007 [0.0006, 0.0007] | 1.049 | 0.953 |
| Online total | reference | 19.5677 [18.0976, 19.6631] | 23.9915 [23.2386, 24.6507] | 0.816 | 1.226 |
| Online total | fusion | 18.3093 [17.9121, 19.2634] | 23.4815 [22.8481, 24.3645] | 0.780 | 1.282 |
| Online total | wp_uninit | 17.4972 [17.3766, 17.7304] | 23.1933 [22.6992, 23.4874] | 0.754 | 1.326 |
| Online total | both | 18.5045 [18.2930, 18.6730] | 23.0507 [22.9903, 23.1472] | 0.803 | 1.246 |
| Online total | previous_reference | 18.5286 [18.3341, 19.3210] | 23.8434 [23.1486, 24.2562] | 0.777 | 1.287 |

Positive time reduction means faster; negative delta means less time. Positive relative F/M gain improves Mikros's relative advantage.

| Stage | Arm | Mikros delta (s) | Mikros reduction | M reference/arm | FLOCK delta (s) | FLOCK reduction | F reference/arm | Relative F/M gain |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Gate PIOP | fusion | -0.1927 | +13.13% | 1.151 | -0.0465 | +2.51% | 1.026 | +12.23% |
| Gate PIOP | wp_uninit | -0.1055 | +7.19% | 1.077 | -0.0347 | +1.88% | 1.019 | +5.72% |
| Gate PIOP | both | +0.0275 | -1.88% | 0.982 | -0.0393 | +2.12% | 1.022 | -3.93% |
| Copy PIOP | fusion | -0.1985 | +6.84% | 1.073 | +0.0037 | -0.06% | 0.999 | +7.40% |
| Copy PIOP | wp_uninit | -0.2042 | +7.03% | 1.076 | -0.0192 | +0.28% | 1.003 | +7.26% |
| Copy PIOP | both | +0.0337 | -1.16% | 0.989 | -0.1255 | +1.87% | 1.019 | -2.99% |
| Witness PCS opening | fusion | -0.0652 | +1.14% | 1.012 | +0.0161 | -0.26% | 0.997 | +1.42% |
| Witness PCS opening | wp_uninit | -0.9613 | +16.84% | 1.203 | -0.6420 | +10.50% | 1.117 | +7.63% |
| Witness PCS opening | both | -0.6270 | +10.98% | 1.123 | -0.6509 | +10.64% | 1.119 | +0.38% |
| Public PCS opening | fusion | -0.0027 | +0.20% | 1.002 | -0.0111 | +1.00% | 1.010 | -0.80% |
| Public PCS opening | wp_uninit | -0.0487 | +3.64% | 1.038 | -0.0443 | +3.99% | 1.042 | -0.36% |
| Public PCS opening | both | -0.0009 | +0.07% | 1.001 | -0.0007 | +0.06% | 1.001 | +0.00% |
| Commit and recommits | fusion | -0.4558 | +9.44% | 1.104 | -0.4291 | +9.11% | 1.100 | +0.36% |
| Commit and recommits | wp_uninit | -0.3740 | +7.75% | 1.084 | +0.0050 | -0.11% | 0.999 | +8.51% |
| Commit and recommits | both | -0.0720 | +1.49% | 1.015 | -0.0248 | +0.53% | 1.005 | +0.98% |
| Release | fusion | -0.0358 | +1.18% | 1.012 | -0.0908 | +2.62% | 1.027 | -1.46% |
| Release | wp_uninit | +0.0172 | -0.57% | 0.994 | +0.0247 | -0.71% | 0.993 | +0.15% |
| Release | both | -0.0553 | +1.83% | 1.019 | -0.0459 | +1.33% | 1.013 | +0.51% |
| Serialization | fusion | -0.0002 | +22.41% | 1.289 | +0.0002 | -21.23% | 0.825 | +56.24% |
| Serialization | wp_uninit | -0.0001 | +14.08% | 1.164 | +0.0000 | -0.03% | 1.000 | +16.42% |
| Serialization | both | +0.0001 | -6.66% | 0.938 | +0.0000 | -4.91% | 0.953 | -1.64% |
| Online total | fusion | -1.2584 | +6.43% | 1.069 | -0.5100 | +2.13% | 1.022 | +4.60% |
| Online total | wp_uninit | -2.0704 | +10.58% | 1.118 | -0.7982 | +3.33% | 1.034 | +8.11% |
| Online total | both | -1.0631 | +5.43% | 1.057 | -0.9408 | +3.92% | 1.041 | +1.60% |

| System | Arm | Samples | Proof bytes | Logical W/P | Ring W/P | Verify median [range] ms | RSS median [range] GiB |
|---|---|---:|---:|---:|---:|---:|---:|
| mikros | reference | 3 | 1202103 | 11/8 | 2/8 | 16.488 [15.328, 17.047] | 26.276 [26.273, 26.279] |
| mikros | fusion | 3 | 1202103 | 11/8 | 2/8 | 16.525 [15.733, 19.632] | 26.276 [26.274, 26.278] |
| mikros | wp_uninit | 3 | 1202103 | 11/8 | 2/8 | 17.115 [15.943, 18.680] | 26.276 [26.275, 26.278] |
| mikros | both | 3 | 1202103 | 11/8 | 2/8 | 17.214 [17.142, 17.304] | 26.277 [26.276, 26.277] |
| mikros | previous_reference | 3 | 1202103 | 11/8 | 2/8 | 15.507 [15.429, 16.305] | 26.278 [26.275, 26.279] |
| flock | reference | 3 | 1160639 | 7/3 | 4/3 | 15.566 [15.391, 18.216] | 26.307 [26.306, 26.309] |
| flock | fusion | 3 | 1160639 | 7/3 | 4/3 | 15.907 [15.339, 16.517] | 26.309 [26.309, 26.311] |
| flock | wp_uninit | 3 | 1160639 | 7/3 | 4/3 | 15.131 [15.097, 16.873] | 26.308 [26.307, 26.308] |
| flock | both | 3 | 1160639 | 7/3 | 4/3 | 17.236 [14.779, 17.319] | 26.307 [26.305, 26.309] |
| flock | previous_reference | 3 | 1160639 | 7/3 | 4/3 | 15.947 [14.991, 16.599] | 26.308 [26.305, 26.310] |

mikros reference raw online milliseconds, one per process: [19567.693791, 18097.595907, 19663.129763].
mikros fusion raw online milliseconds, one per process: [18309.30256, 17912.142003, 19263.350679].
mikros wp_uninit raw online milliseconds, one per process: [17497.249971999998, 17730.35367, 17376.60574].
mikros both raw online milliseconds, one per process: [18672.987564, 18293.024756000003, 18504.548465].
mikros previous_reference raw online milliseconds, one per process: [18334.091053, 18528.581506, 19320.972839].
flock reference raw online milliseconds, one per process: [24650.69674, 23238.596996999997, 23991.487921].
flock fusion raw online milliseconds, one per process: [24364.460431, 22848.114204, 23481.501776].
flock wp_uninit raw online milliseconds, one per process: [23193.309018, 23487.439254, 22699.152761].
flock both raw online milliseconds, one per process: [23147.208522, 22990.288049, 23050.737297].
flock previous_reference raw online milliseconds, one per process: [23843.443166, 23148.557818, 24256.201751].

comparison.json retains all raw phase milliseconds, proof/PIOP/claim/commitment hashes, parameters, resource records, previous-reference changes and SHA-256 digests of evidence files.
