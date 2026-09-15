# N26 E2E measured comparison

Online including conversions, release and recommit; offline excluded

Median of separate processes; phase medians need not sum to total median

## single

| Stage (seconds) | Baseline Mikros | Baseline FLOCK | Optimized Mikros | Optimized FLOCK | FLOCK / Mikros after |
|---|---:|---:|---:|---:|---:|
| Gate PIOP | 0.0685 | 0.0170 | 0.0874 | 0.0168 | 0.192 |
| Copy PIOP | 1.3443 | 6.6629 | 1.3421 | 6.5735 | 4.898 |
| Witness PCS opening | 0.3345 | 0.3165 | 0.2466 | 0.2044 | 0.829 |
| Public PCS opening | 3.5899 | 2.4009 | 1.8891 | 1.0655 | 0.564 |
| Commit and recommits | 0.6447 | 0.6428 | 0.6320 | 0.6335 | 1.002 |
| Release | 1.7359 | 0.9172 | 1.6678 | 0.9961 | 0.597 |
| Serialization | 0.0007 | 0.0006 | 0.0008 | 0.0006 | 0.685 |
| Online total | 7.8029 | 11.0614 | 5.8885 | 9.5325 | 1.619 |

mikros optimized total samples (seconds): [5.96012179, 5.825582173, 5.888510258].

flock optimized total samples (seconds): [9.532491269, 9.561153535999999, 9.424258515].

## multiple

| Stage (seconds) | Baseline Mikros | Baseline FLOCK | Optimized Mikros | Optimized FLOCK | FLOCK / Mikros after |
|---|---:|---:|---:|---:|---:|
| Gate PIOP | 1.3935 | 1.7922 | 1.3547 | 1.8115 | 1.337 |
| Copy PIOP | 6.0391 | 7.7473 | 4.5325 | 6.6928 | 1.477 |
| Witness PCS opening | 12.4261 | 12.1982 | 8.2816 | 8.2135 | 0.992 |
| Public PCS opening | 3.1571 | 2.3221 | 1.2995 | 1.0973 | 0.844 |
| Commit and recommits | 4.2693 | 4.4166 | 4.2841 | 4.7462 | 1.108 |
| Release | 3.0266 | 3.5723 | 3.2699 | 3.5904 | 1.098 |
| Serialization | 0.0010 | 0.0006 | 0.0012 | 0.0008 | 0.700 |
| Online total | 30.1239 | 31.9214 | 23.0504 | 26.3654 | 1.144 |

mikros optimized total samples (seconds): [23.050354061, 23.674748934, 22.689445815].

flock optimized total samples (seconds): [26.390400052, 26.365395736, 26.123534779].

