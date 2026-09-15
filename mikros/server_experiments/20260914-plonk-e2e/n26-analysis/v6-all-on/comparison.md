# N26 E2E measured comparison

Online including conversions, release and recommit; offline excluded

Median of separate processes; phase medians need not sum to total median

Allocator: baseline=system, optimized=system.

Disabled optimizations, identically for both systems: {'single': [], 'multiple': []}.

## single

| Stage (seconds) | Baseline Mikros | Baseline FLOCK | Optimized Mikros | Optimized FLOCK | FLOCK / Mikros after |
|---|---:|---:|---:|---:|---:|
| Gate PIOP | 0.0685 | 0.0170 | 0.0809 | 0.0169 | 0.209 |
| Copy PIOP | 1.3443 | 6.6629 | 1.3537 | 6.6556 | 4.917 |
| Witness PCS opening | 0.3345 | 0.3165 | 0.2301 | 0.2086 | 0.907 |
| Public PCS opening | 3.5899 | 2.4009 | 1.2025 | 0.8966 | 0.746 |
| Commit and recommits | 0.6447 | 0.6428 | 0.6403 | 0.6712 | 1.048 |
| Release | 1.7359 | 0.9172 | 1.6896 | 0.9814 | 0.581 |
| Serialization | 0.0007 | 0.0006 | 0.0007 | 0.0006 | 0.877 |
| Online total | 7.8029 | 11.0614 | 5.2076 | 9.4560 | 1.816 |

mikros optimized total samples (seconds): [5.150012384, 5.247330785, 5.207580401].

flock optimized total samples (seconds): [9.522698686, 9.455976594, 9.284187145].

## multiple

| Stage (seconds) | Baseline Mikros | Baseline FLOCK | Optimized Mikros | Optimized FLOCK | FLOCK / Mikros after |
|---|---:|---:|---:|---:|---:|
| Gate PIOP | 1.3935 | 1.7922 | 1.3684 | 1.8144 | 1.326 |
| Copy PIOP | 6.0391 | 7.7473 | 2.6881 | 6.7206 | 2.500 |
| Witness PCS opening | 12.4261 | 12.1982 | 7.1311 | 6.7707 | 0.949 |
| Public PCS opening | 3.1571 | 2.3221 | 1.2914 | 1.0305 | 0.798 |
| Commit and recommits | 4.2693 | 4.4166 | 4.4114 | 4.2491 | 0.963 |
| Release | 3.0266 | 3.5723 | 3.1998 | 3.5794 | 1.119 |
| Serialization | 0.0010 | 0.0006 | 0.0006 | 0.0006 | 0.890 |
| Online total | 30.1239 | 31.9214 | 20.0379 | 24.0434 | 1.200 |

mikros optimized total samples (seconds): [21.211236089, 19.923003636, 20.037871761].

flock optimized total samples (seconds): [24.424473618, 24.04342505, 24.039764374].

