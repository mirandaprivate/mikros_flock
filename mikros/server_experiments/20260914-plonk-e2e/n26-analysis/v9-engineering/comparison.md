# N26 PackedDirect ON engineering comparison

single: N26 gates; multiple: 128 circuits each with N26 gates

Online commit/recommit, conversion, PIOPs, PCS and release included; offline excluded

Medians and min/max ranges of at least three independent fresh processes per arm/case; phase medians need not sum to the total median

Descriptive samples and ranges, not a statistical significance claim; trace runs are excluded by rejection

Four arms use one frozen source/binary, seed and native PCS configuration. Complete proof hashes/bytes, PIOPs, terminal claims, commitments and logical/ring counts match for each system/mode.

| Arm | Extra opt-ins (PackedDirect remains ON) |
|---|---|
| reference | none |
| wp | FLOCK_DIRECT_WP_SIMD=1 |
| gate | MIKROS_GATE_SELECTOR_SIMD=1 |
| both | FLOCK_DIRECT_WP_SIMD=1, MIKROS_GATE_SELECTOR_SIMD=1 |

Previous ON is separately identified: source/binary may differ; all other audited inputs, full proofs and counts must match reference.

## single

Times below are median [min, max] seconds. M/F < 1 means Mikros is faster; F/M > 1 means Mikros is faster.

| Stage | Arm | Mikros (s) | FLOCK (s) | M/F | F/M |
|---|---|---:|---:|---:|---:|
| Gate PIOP | reference | 0.0644 [0.0637, 0.0815] | 0.0167 [0.0167, 0.0176] | 3.857 | 0.259 |
| Gate PIOP | wp | 0.0732 [0.0672, 0.0981] | 0.0165 [0.0164, 0.0175] | 4.427 | 0.226 |
| Gate PIOP | gate | 0.0715 [0.0662, 0.0884] | 0.0166 [0.0161, 0.0172] | 4.320 | 0.231 |
| Gate PIOP | both | 0.0688 [0.0683, 0.0741] | 0.0164 [0.0164, 0.0170] | 4.181 | 0.239 |
| Gate PIOP | previous_on | 0.0875 [0.0723, 0.0975] | 0.0169 [0.0164, 0.0169] | 5.189 | 0.193 |
| Copy PIOP | reference | 1.4219 [1.4103, 1.4368] | 6.6151 [6.3425, 6.6171] | 0.215 | 4.652 |
| Copy PIOP | wp | 1.4211 [1.4037, 1.4760] | 6.7087 [6.6062, 6.9207] | 0.212 | 4.721 |
| Copy PIOP | gate | 1.4206 [1.3925, 1.4213] | 6.6191 [6.4754, 6.9228] | 0.215 | 4.659 |
| Copy PIOP | both | 1.4029 [1.3912, 1.5289] | 6.8077 [6.6669, 6.9385] | 0.206 | 4.853 |
| Copy PIOP | previous_on | 1.4422 [1.4339, 1.4734] | 6.5803 [6.4457, 6.6417] | 0.219 | 4.563 |
| Witness PCS opening | reference | 0.2785 [0.2643, 0.2801] | 0.2304 [0.2189, 0.2526] | 1.209 | 0.827 |
| Witness PCS opening | wp | 0.2355 [0.2338, 0.2781] | 0.2213 [0.1896, 0.2273] | 1.064 | 0.940 |
| Witness PCS opening | gate | 0.2518 [0.2302, 0.2575] | 0.2252 [0.1726, 0.2413] | 1.118 | 0.894 |
| Witness PCS opening | both | 0.2401 [0.2400, 0.2523] | 0.2126 [0.1830, 0.2675] | 1.129 | 0.885 |
| Witness PCS opening | previous_on | 0.2424 [0.2322, 0.2584] | 0.2299 [0.2280, 0.2612] | 1.055 | 0.948 |
| Public PCS opening | reference | 1.1981 [1.1276, 1.2036] | 0.8821 [0.8691, 0.8823] | 1.358 | 0.736 |
| Public PCS opening | wp | 1.1579 [1.1573, 1.1909] | 0.8861 [0.8741, 0.9368] | 1.307 | 0.765 |
| Public PCS opening | gate | 1.1983 [1.1731, 1.2334] | 0.9043 [0.8808, 0.9396] | 1.325 | 0.755 |
| Public PCS opening | both | 1.1660 [1.1607, 1.1976] | 0.8746 [0.8651, 0.9063] | 1.333 | 0.750 |
| Public PCS opening | previous_on | 1.1851 [1.1611, 1.2213] | 0.9160 [0.8999, 0.9241] | 1.294 | 0.773 |
| Commit and recommits | reference | 0.6108 [0.6026, 0.6273] | 0.6188 [0.5939, 0.6811] | 0.987 | 1.013 |
| Commit and recommits | wp | 0.5903 [0.5796, 0.5984] | 0.5913 [0.5544, 0.6703] | 0.998 | 1.002 |
| Commit and recommits | gate | 0.7122 [0.5976, 0.7539] | 0.5905 [0.4958, 0.6196] | 1.206 | 0.829 |
| Commit and recommits | both | 0.6467 [0.5820, 0.6916] | 0.5997 [0.5808, 0.6267] | 1.078 | 0.927 |
| Commit and recommits | previous_on | 0.5846 [0.5647, 0.6328] | 0.6091 [0.5851, 0.6200] | 0.960 | 1.042 |
| Release | reference | 1.5969 [1.5921, 1.6332] | 0.9228 [0.9186, 0.9586] | 1.731 | 0.578 |
| Release | wp | 1.6445 [1.5798, 1.6742] | 0.9009 [0.9004, 0.9193] | 1.825 | 0.548 |
| Release | gate | 1.6738 [1.6078, 1.6757] | 0.9138 [0.9028, 0.9628] | 1.832 | 0.546 |
| Release | both | 1.6089 [1.5744, 1.6225] | 0.9150 [0.9026, 0.9404] | 1.758 | 0.569 |
| Release | previous_on | 1.6736 [1.6278, 1.6876] | 0.9522 [0.9219, 0.9605] | 1.758 | 0.569 |
| Serialization | reference | 0.0011 [0.0010, 0.0012] | 0.0008 [0.0008, 0.0008] | 1.375 | 0.728 |
| Serialization | wp | 0.0012 [0.0009, 0.0013] | 0.0007 [0.0006, 0.0009] | 1.753 | 0.571 |
| Serialization | gate | 0.0009 [0.0009, 0.0009] | 0.0011 [0.0007, 0.0011] | 0.854 | 1.171 |
| Serialization | both | 0.0011 [0.0009, 0.0011] | 0.0009 [0.0008, 0.0012] | 1.286 | 0.778 |
| Serialization | previous_on | 0.0010 [0.0009, 0.0011] | 0.0009 [0.0008, 0.0009] | 1.195 | 0.837 |
| Online total | reference | 5.1819 [5.0893, 5.2271] | 9.3204 [8.9981, 9.3400] | 0.556 | 1.799 |
| Online total | wp | 5.1485 [5.1250, 5.1904] | 9.4559 [9.1809, 9.5243] | 0.544 | 1.837 |
| Online total | gate | 5.2707 [5.2093, 5.3495] | 9.2707 [8.9446, 9.7048] | 0.569 | 1.759 |
| Online total | both | 5.1647 [5.1472, 5.2094] | 9.3991 [9.2761, 9.6658] | 0.549 | 1.820 |
| Online total | previous_on | 5.2218 [5.1848, 5.2761] | 9.2833 [9.2048, 9.3410] | 0.562 | 1.778 |

### Changes from reference

Positive time reduction means faster; negative delta means less time. Positive F/M change improves Mikros's relative advantage.

| Stage | Arm | Mikros delta (s) | Mikros reduction | Reference/Mikros speedup | FLOCK delta (s) | FLOCK reduction | Reference/FLOCK speedup | F/M change |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Gate PIOP | wp | +0.0088 | -13.74% | 0.879 | -0.0002 | +0.92% | 1.009 | -12.89% |
| Gate PIOP | gate | +0.0071 | -11.10% | 0.900 | -0.0001 | +0.82% | 1.008 | -10.73% |
| Gate PIOP | both | +0.0044 | -6.78% | 0.936 | -0.0002 | +1.50% | 1.015 | -7.75% |
| Copy PIOP | wp | -0.0008 | +0.06% | 1.001 | +0.0936 | -1.41% | 0.986 | +1.47% |
| Copy PIOP | gate | -0.0013 | +0.09% | 1.001 | +0.0039 | -0.06% | 0.999 | +0.15% |
| Copy PIOP | both | -0.0190 | +1.34% | 1.014 | +0.1925 | -2.91% | 0.972 | +4.31% |
| Witness PCS opening | wp | -0.0430 | +15.44% | 1.183 | -0.0092 | +3.97% | 1.041 | +13.56% |
| Witness PCS opening | gate | -0.0267 | +9.59% | 1.106 | -0.0052 | +2.27% | 1.023 | +8.10% |
| Witness PCS opening | both | -0.0384 | +13.78% | 1.160 | -0.0178 | +7.72% | 1.084 | +7.03% |
| Public PCS opening | wp | -0.0402 | +3.36% | 1.035 | +0.0039 | -0.45% | 0.996 | +3.94% |
| Public PCS opening | gate | +0.0002 | -0.02% | 1.000 | +0.0222 | -2.52% | 0.975 | +2.50% |
| Public PCS opening | both | -0.0321 | +2.68% | 1.028 | -0.0076 | +0.86% | 1.009 | +1.88% |
| Commit and recommits | wp | -0.0206 | +3.37% | 1.035 | -0.0275 | +4.44% | 1.046 | -1.11% |
| Commit and recommits | gate | +0.1014 | -16.59% | 0.858 | -0.0283 | +4.57% | 1.048 | -18.15% |
| Commit and recommits | both | +0.0359 | -5.87% | 0.945 | -0.0191 | +3.08% | 1.032 | -8.46% |
| Release | wp | +0.0476 | -2.98% | 0.971 | -0.0219 | +2.37% | 1.024 | -5.20% |
| Release | gate | +0.0769 | -4.81% | 0.954 | -0.0090 | +0.97% | 1.010 | -5.52% |
| Release | both | +0.0121 | -0.76% | 0.992 | -0.0078 | +0.84% | 1.008 | -1.58% |
| Serialization | wp | +0.0001 | -5.25% | 0.950 | -0.0001 | +17.45% | 1.211 | -21.57% |
| Serialization | gate | -0.0002 | +19.89% | 1.248 | +0.0002 | -28.89% | 0.776 | +60.90% |
| Serialization | both | +0.0000 | -2.35% | 0.977 | +0.0001 | -9.40% | 0.914 | +6.89% |
| Online total | wp | -0.0333 | +0.64% | 1.006 | +0.1355 | -1.45% | 0.986 | +2.11% |
| Online total | gate | +0.0889 | -1.71% | 0.983 | -0.0497 | +0.53% | 1.005 | -2.21% |
| Online total | both | -0.0171 | +0.33% | 1.003 | +0.0786 | -0.84% | 0.992 | +1.18% |

| System | Arm | Samples | Proof bytes | Logical W/P | Ring W/P | Verify median [range] ms | RSS median [range] GiB |
|---|---|---:|---:|---:|---:|---:|---:|
| mikros | reference | 3 | 1296879 | 11/89 | 11/89 | 45.611 [45.043, 46.610] | 17.521 [17.518, 17.521] |
| mikros | wp | 3 | 1296879 | 11/89 | 11/89 | 46.043 [45.176, 46.283] | 17.520 [17.520, 17.521] |
| mikros | gate | 3 | 1296879 | 11/89 | 11/89 | 45.826 [45.141, 48.139] | 17.519 [17.519, 17.522] |
| mikros | both | 3 | 1296879 | 11/89 | 11/89 | 46.591 [45.997, 46.599] | 17.521 [17.518, 17.523] |
| mikros | previous_on | 3 | 1296879 | 11/89 | 11/89 | 45.571 [45.083, 46.124] | 17.520 [17.518, 17.523] |
| flock | reference | 3 | 1215735 | 7/84 | 7/84 | 40.982 [39.108, 41.995] | 21.030 [21.029, 21.030] |
| flock | wp | 3 | 1215735 | 7/84 | 7/84 | 38.471 [38.066, 38.510] | 21.031 [21.028, 21.032] |
| flock | gate | 3 | 1215735 | 7/84 | 7/84 | 39.244 [38.533, 39.757] | 21.030 [21.027, 21.030] |
| flock | both | 3 | 1215735 | 7/84 | 7/84 | 40.842 [38.800, 42.451] | 21.028 [21.028, 21.030] |
| flock | previous_on | 3 | 1215735 | 7/84 | 7/84 | 39.099 [39.037, 39.352] | 21.028 [21.028, 21.029] |

mikros reference online seconds, one per process: [5.089312, 5.181852, 5.227069].
mikros wp online seconds, one per process: [5.148527, 5.190425, 5.124992].
mikros gate online seconds, one per process: [5.209346, 5.349471, 5.270713].
mikros both online seconds, one per process: [5.147169, 5.209371, 5.164712].
mikros previous_on online seconds, one per process: [5.221783, 5.184779, 5.276056].
flock reference online seconds, one per process: [9.340021, 8.998093, 9.320419].
flock wp online seconds, one per process: [9.180912, 9.524252, 9.455938].
flock gate online seconds, one per process: [9.270694, 8.944646, 9.704788].
flock both online seconds, one per process: [9.399055, 9.276085, 9.665787].
flock previous_on online seconds, one per process: [9.283303, 9.341047, 9.204774].

## multiple

Times below are median [min, max] seconds. M/F < 1 means Mikros is faster; F/M > 1 means Mikros is faster.

| Stage | Arm | Mikros (s) | FLOCK (s) | M/F | F/M |
|---|---|---:|---:|---:|---:|
| Gate PIOP | reference | 1.3258 [1.3170, 1.5216] | 1.7367 [1.7217, 1.8257] | 0.763 | 1.310 |
| Gate PIOP | wp | 1.5404 [1.3656, 1.5861] | 1.7715 [1.7426, 1.8069] | 0.870 | 1.150 |
| Gate PIOP | gate | 1.5445 [1.5359, 1.5446] | 1.7871 [1.7253, 1.7966] | 0.864 | 1.157 |
| Gate PIOP | both | 1.3217 [1.3096, 1.5556] | 1.7478 [1.7208, 1.8223] | 0.756 | 1.322 |
| Gate PIOP | previous_on | 1.3386 [1.3341, 1.5336] | 1.7444 [1.7268, 1.9143] | 0.767 | 1.303 |
| Copy PIOP | reference | 2.7112 [2.7018, 2.9484] | 6.6273 [6.6015, 6.6910] | 0.409 | 2.444 |
| Copy PIOP | wp | 2.8110 [2.6543, 2.9333] | 6.6254 [6.6229, 6.7909] | 0.424 | 2.357 |
| Copy PIOP | gate | 2.8783 [2.7892, 2.8784] | 6.5409 [6.5400, 6.6694] | 0.440 | 2.272 |
| Copy PIOP | both | 2.7715 [2.7570, 2.9124] | 6.7526 [6.6428, 6.7891] | 0.410 | 2.436 |
| Copy PIOP | previous_on | 2.7165 [2.6629, 2.8752] | 6.4546 [6.3682, 6.8752] | 0.421 | 2.376 |
| Witness PCS opening | reference | 5.6742 [5.6621, 5.7493] | 6.1188 [6.0731, 6.2712] | 0.927 | 1.078 |
| Witness PCS opening | wp | 5.6724 [5.6119, 5.7149] | 6.2397 [5.9531, 6.4912] | 0.909 | 1.100 |
| Witness PCS opening | gate | 5.7092 [5.6738, 5.8573] | 6.1572 [5.9205, 6.2082] | 0.927 | 1.078 |
| Witness PCS opening | both | 5.5417 [5.2297, 5.6729] | 6.1214 [6.0510, 6.2206] | 0.905 | 1.105 |
| Witness PCS opening | previous_on | 5.6148 [5.4286, 5.7957] | 6.0150 [5.9919, 6.0600] | 0.933 | 1.071 |
| Public PCS opening | reference | 1.2986 [1.2756, 1.4309] | 1.0724 [1.0086, 1.0986] | 1.211 | 0.826 |
| Public PCS opening | wp | 1.3902 [1.3097, 1.4016] | 1.0841 [1.0129, 1.1123] | 1.282 | 0.780 |
| Public PCS opening | gate | 1.3532 [1.3205, 1.4065] | 1.1089 [0.9955, 1.1283] | 1.220 | 0.819 |
| Public PCS opening | both | 1.3180 [1.3040, 1.4024] | 1.0945 [0.9891, 1.1543] | 1.204 | 0.830 |
| Public PCS opening | previous_on | 1.2983 [1.2816, 1.3816] | 1.0341 [1.0272, 1.0589] | 1.256 | 0.796 |
| Commit and recommits | reference | 4.4692 [4.4120, 4.8132] | 4.8330 [4.3166, 4.9401] | 0.925 | 1.081 |
| Commit and recommits | wp | 4.7804 [4.4199, 4.7935] | 4.5084 [4.2396, 4.9376] | 1.060 | 0.943 |
| Commit and recommits | gate | 4.7985 [4.7745, 4.8328] | 4.2161 [4.0794, 4.9396] | 1.138 | 0.879 |
| Commit and recommits | both | 4.3048 [4.1515, 5.0527] | 4.4369 [4.4258, 4.6983] | 0.970 | 1.031 |
| Commit and recommits | previous_on | 4.3132 [4.2914, 4.7112] | 4.3179 [4.1899, 4.4802] | 0.999 | 1.001 |
| Release | reference | 2.9523 [2.8566, 3.0607] | 3.4279 [3.4263, 3.4547] | 0.861 | 1.161 |
| Release | wp | 3.0420 [2.8457, 3.0616] | 3.4489 [3.3219, 3.4923] | 0.882 | 1.134 |
| Release | gate | 2.9361 [2.8371, 3.0749] | 3.4514 [3.4012, 3.5451] | 0.851 | 1.175 |
| Release | both | 2.9824 [2.9174, 3.0083] | 3.4992 [3.4871, 3.5252] | 0.852 | 1.173 |
| Release | previous_on | 2.9935 [2.9594, 2.9967] | 3.4604 [3.3821, 3.4798] | 0.865 | 1.156 |
| Serialization | reference | 0.0007 [0.0007, 0.0008] | 0.0007 [0.0006, 0.0007] | 1.049 | 0.953 |
| Serialization | wp | 0.0007 [0.0007, 0.0009] | 0.0006 [0.0006, 0.0012] | 1.226 | 0.816 |
| Serialization | gate | 0.0007 [0.0007, 0.0010] | 0.0010 [0.0006, 0.0013] | 0.766 | 1.306 |
| Serialization | both | 0.0009 [0.0006, 0.0009] | 0.0006 [0.0006, 0.0011] | 1.509 | 0.663 |
| Serialization | previous_on | 0.0008 [0.0007, 0.0011] | 0.0006 [0.0006, 0.0006] | 1.265 | 0.791 |
| Online total | reference | 18.5286 [18.3341, 19.3210] | 23.8434 [23.1486, 24.2562] | 0.777 | 1.287 |
| Online total | wp | 19.0510 [18.5270, 19.3597] | 23.4535 [23.3540, 24.3981] | 0.812 | 1.231 |
| Online total | gate | 19.3530 [18.9973, 19.3983] | 23.5455 [22.7748, 23.8941] | 0.822 | 1.217 |
| Online total | both | 18.0731 [17.9292, 19.5146] | 23.6145 [23.5058, 24.0616] | 0.765 | 1.307 |
| Online total | previous_on | 18.2209 [18.0512, 19.2581] | 23.1648 [22.9516, 23.4673] | 0.787 | 1.271 |

### Changes from reference

Positive time reduction means faster; negative delta means less time. Positive F/M change improves Mikros's relative advantage.

| Stage | Arm | Mikros delta (s) | Mikros reduction | Reference/Mikros speedup | FLOCK delta (s) | FLOCK reduction | Reference/FLOCK speedup | F/M change |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Gate PIOP | wp | +0.2145 | -16.18% | 0.861 | +0.0348 | -2.00% | 0.980 | -12.20% |
| Gate PIOP | gate | +0.2187 | -16.49% | 0.858 | +0.0504 | -2.90% | 0.972 | -11.67% |
| Gate PIOP | both | -0.0041 | +0.31% | 1.003 | +0.0111 | -0.64% | 0.994 | +0.95% |
| Copy PIOP | wp | +0.0997 | -3.68% | 0.965 | -0.0019 | +0.03% | 1.000 | -3.58% |
| Copy PIOP | gate | +0.1671 | -6.16% | 0.942 | -0.0864 | +1.30% | 1.013 | -7.03% |
| Copy PIOP | both | +0.0603 | -2.22% | 0.978 | +0.1253 | -1.89% | 0.981 | -0.33% |
| Witness PCS opening | wp | -0.0018 | +0.03% | 1.000 | +0.1209 | -1.98% | 0.981 | +2.01% |
| Witness PCS opening | gate | +0.0349 | -0.62% | 0.994 | +0.0384 | -0.63% | 0.994 | +0.01% |
| Witness PCS opening | both | -0.1326 | +2.34% | 1.024 | +0.0026 | -0.04% | 1.000 | +2.44% |
| Public PCS opening | wp | +0.0916 | -7.06% | 0.934 | +0.0116 | -1.09% | 0.989 | -5.58% |
| Public PCS opening | gate | +0.0546 | -4.20% | 0.960 | +0.0364 | -3.40% | 0.967 | -0.77% |
| Public PCS opening | both | +0.0195 | -1.50% | 0.985 | +0.0221 | -2.06% | 0.980 | +0.55% |
| Commit and recommits | wp | +0.3112 | -6.96% | 0.935 | -0.3246 | +6.72% | 1.072 | -12.79% |
| Commit and recommits | gate | +0.3293 | -7.37% | 0.931 | -0.6169 | +12.76% | 1.146 | -18.75% |
| Commit and recommits | both | -0.1643 | +3.68% | 1.038 | -0.3961 | +8.20% | 1.089 | -4.69% |
| Release | wp | +0.0897 | -3.04% | 0.971 | +0.0210 | -0.61% | 0.994 | -2.35% |
| Release | gate | -0.0162 | +0.55% | 1.006 | +0.0235 | -0.68% | 0.993 | +1.24% |
| Release | both | +0.0300 | -1.02% | 0.990 | +0.0713 | -2.08% | 0.980 | +1.05% |
| Serialization | wp | +0.0000 | -2.72% | 0.974 | -0.0001 | +12.09% | 1.137 | -14.41% |
| Serialization | gate | +0.0000 | -1.35% | 0.987 | +0.0003 | -38.82% | 0.720 | +36.96% |
| Serialization | both | +0.0002 | -27.58% | 0.784 | -0.0001 | +11.34% | 1.128 | -30.51% |
| Online total | wp | +0.5224 | -2.82% | 0.973 | -0.3900 | +1.64% | 1.017 | -4.33% |
| Online total | gate | +0.8244 | -4.45% | 0.957 | -0.2979 | +1.25% | 1.013 | -5.46% |
| Online total | both | -0.4555 | +2.46% | 1.025 | -0.2290 | +0.96% | 1.010 | +1.54% |

| System | Arm | Samples | Proof bytes | Logical W/P | Ring W/P | Verify median [range] ms | RSS median [range] GiB |
|---|---|---:|---:|---:|---:|---:|---:|
| mikros | reference | 3 | 1202103 | 11/8 | 2/8 | 15.507 [15.429, 16.305] | 26.278 [26.275, 26.279] |
| mikros | wp | 3 | 1202103 | 11/8 | 2/8 | 16.601 [16.029, 16.905] | 26.276 [26.274, 26.278] |
| mikros | gate | 3 | 1202103 | 11/8 | 2/8 | 16.358 [15.391, 18.509] | 26.279 [26.275, 26.279] |
| mikros | both | 3 | 1202103 | 11/8 | 2/8 | 15.779 [15.296, 16.350] | 26.275 [26.275, 26.276] |
| mikros | previous_on | 3 | 1202103 | 11/8 | 2/8 | 16.423 [15.421, 16.629] | 26.278 [26.277, 26.282] |
| flock | reference | 3 | 1160639 | 7/3 | 4/3 | 15.947 [14.991, 16.599] | 26.308 [26.305, 26.310] |
| flock | wp | 3 | 1160639 | 7/3 | 4/3 | 16.188 [15.433, 17.913] | 26.308 [26.305, 26.310] |
| flock | gate | 3 | 1160639 | 7/3 | 4/3 | 15.533 [15.027, 17.218] | 26.309 [26.307, 26.311] |
| flock | both | 3 | 1160639 | 7/3 | 4/3 | 16.168 [15.221, 16.202] | 26.309 [26.308, 26.311] |
| flock | previous_on | 3 | 1160639 | 7/3 | 4/3 | 16.013 [15.970, 16.016] | 26.307 [26.307, 26.310] |

mikros reference online seconds, one per process: [18.334091, 18.528582, 19.320973].
mikros wp online seconds, one per process: [18.527006, 19.050973, 19.359682].
mikros gate online seconds, one per process: [19.398289, 19.352966, 18.997331].
mikros both online seconds, one per process: [17.929161, 18.073125, 19.514574].
mikros previous_on online seconds, one per process: [19.258085, 18.051204, 18.220877].
flock reference online seconds, one per process: [23.843443, 23.148558, 24.256202].
flock wp online seconds, one per process: [23.35396, 24.398078, 23.45348].
flock gate online seconds, one per process: [23.545535, 22.774841, 23.8941].
flock both online seconds, one per process: [23.50582, 24.061588, 23.614481].
flock previous_on online seconds, one per process: [23.164762, 23.467278, 22.951596].

Full per-sample timings, identity, proof fingerprints, resource records, previous-ON changes and input-file SHA-256 digests are in comparison.json.
