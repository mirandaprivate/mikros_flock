# N26 E2E measured comparison

Online including conversions, release and recommit; offline excluded

Median of separate processes; phase medians need not sum to total median

Allocator: baseline=system, optimized=system.

Disabled optimizations, identically for both systems: {'single': [], 'multiple': ['sparse-bitbanks']}.

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
| Gate PIOP | 1.3935 | 1.7922 | 1.5116 | 1.8081 | 1.196 |
| Copy PIOP | 6.0391 | 7.7473 | 2.9877 | 6.7887 | 2.272 |
| Witness PCS opening | 12.4261 | 12.1982 | 7.4250 | 7.5356 | 1.015 |
| Public PCS opening | 3.1571 | 2.3221 | 1.3468 | 1.0033 | 0.745 |
| Commit and recommits | 4.2693 | 4.4166 | 5.0636 | 4.3022 | 0.850 |
| Release | 3.0266 | 3.5723 | 3.0336 | 3.5221 | 1.161 |
| Serialization | 0.0010 | 0.0006 | 0.0009 | 0.0006 | 0.680 |
| Online total | 30.1239 | 31.9214 | 21.3540 | 25.0743 | 1.174 |

mikros optimized total samples (seconds): [21.353988408, 21.51642939, 19.920677307].

flock optimized total samples (seconds): [24.797820175, 25.074296105, 25.157850666].

