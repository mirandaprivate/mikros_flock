# N26 E2E measured comparison

Online including conversions, release and recommit; offline excluded

Median of separate processes; phase medians need not sum to total median

Allocator: baseline=system, optimized=system.

Disabled optimizations, identically for both systems: {'single': [], 'multiple': ['sparse-bitbanks', 'jit-fused']}.

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
| Gate PIOP | 1.3935 | 1.7922 | 1.5337 | 1.8025 | 1.175 |
| Copy PIOP | 6.0391 | 7.7473 | 2.9360 | 6.7522 | 2.300 |
| Witness PCS opening | 12.4261 | 12.1982 | 11.3913 | 10.5030 | 0.922 |
| Public PCS opening | 3.1571 | 2.3221 | 2.2894 | 2.0113 | 0.879 |
| Commit and recommits | 4.2693 | 4.4166 | 4.7217 | 4.3950 | 0.931 |
| Release | 3.0266 | 3.5723 | 2.7622 | 3.3112 | 1.199 |
| Serialization | 0.0010 | 0.0006 | 0.0010 | 0.0005 | 0.562 |
| Online total | 30.1239 | 31.9214 | 25.8246 | 29.3153 | 1.135 |

mikros optimized total samples (seconds): [25.82462701, 24.305790043000002, 26.016134092].

flock optimized total samples (seconds): [29.315293073, 28.35968683, 29.45506017].

