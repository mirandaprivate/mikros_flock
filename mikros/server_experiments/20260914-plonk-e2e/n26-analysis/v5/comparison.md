# N26 E2E measured comparison

Online including conversions, release and recommit; offline excluded

Median of separate processes; phase medians need not sum to total median

Allocator: baseline=system, optimized=system.

## single

| Stage (seconds) | Baseline Mikros | Baseline FLOCK | Optimized Mikros | Optimized FLOCK | FLOCK / Mikros after |
|---|---:|---:|---:|---:|---:|
| Gate PIOP | 0.0685 | 0.0170 | 0.0734 | 0.0168 | 0.228 |
| Copy PIOP | 1.3443 | 6.6629 | 1.3385 | 6.5070 | 4.861 |
| Witness PCS opening | 0.3345 | 0.3165 | 0.2300 | 0.2419 | 1.052 |
| Public PCS opening | 3.5899 | 2.4009 | 1.1931 | 0.8884 | 0.745 |
| Commit and recommits | 0.6447 | 0.6428 | 0.6077 | 0.6273 | 1.032 |
| Release | 1.7359 | 0.9172 | 1.6695 | 0.9799 | 0.587 |
| Serialization | 0.0007 | 0.0006 | 0.0008 | 0.0006 | 0.710 |
| Online total | 7.8029 | 11.0614 | 5.1216 | 9.3808 | 1.832 |

mikros optimized total samples (seconds): [5.081831510999999, 5.213115973, 5.121649528].

flock optimized total samples (seconds): [9.380788112, 9.295588522, 9.566152272].

## multiple

| Stage (seconds) | Baseline Mikros | Baseline FLOCK | Optimized Mikros | Optimized FLOCK | FLOCK / Mikros after |
|---|---:|---:|---:|---:|---:|
| Gate PIOP | 1.3935 | 1.7922 | 1.3718 | 1.7618 | 1.284 |
| Copy PIOP | 6.0391 | 7.7473 | 4.3382 | 6.7254 | 1.550 |
| Witness PCS opening | 12.4261 | 12.1982 | 7.1048 | 6.6277 | 0.933 |
| Public PCS opening | 3.1571 | 2.3221 | 1.2909 | 1.0272 | 0.796 |
| Commit and recommits | 4.2693 | 4.4166 | 4.3727 | 4.4520 | 1.018 |
| Release | 3.0266 | 3.5723 | 3.2280 | 3.5831 | 1.110 |
| Serialization | 0.0010 | 0.0006 | 0.0006 | 0.0005 | 0.931 |
| Online total | 30.1239 | 31.9214 | 21.6109 | 24.0366 | 1.112 |

mikros optimized total samples (seconds): [21.603088821, 21.610940133, 22.03087823].

flock optimized total samples (seconds): [24.501758015, 23.858722613, 24.036555341].

