# Binary Mikros 与 Flock：本机实测

本报告由 `summarize_results.py` 从已保存 JSON 生成，并重新检查样本数量、配对输入、消息大小和统计量。

这是优化前 Lookup 的历史结果。已完成的 64 行打包工程优化及新对照见 [OPTIMIZED_BENCHMARKS.md](OPTIMIZED_BENCHMARKS.md)；下文不代表新 Packed 的性能。

当前 mixed-tensor 实现在较大输入上，Flock 的 prover 更快；Mikros 的 verifier 用时更低、此次峰值 RSS 更低，但证明消息明显更大。小输入的多线程结果有较大的调度开销和波动，完整数据如下。

## 环境与测量范围

- 测量日期：2026-09-12 UTC；CPU：Intel(R) Core(TM) i7-8700K CPU @ 3.70GHz；OS：Ubuntu/WSL2 Linux x86-64。
- 编译器：`rustc 1.96.0-nightly (ac7f9ec7d 2026-03-20); binary: rustc; commit-hash: ac7f9ec7da74d37fd28667c86bf117a39ba5b02a; commit-date: 2026-03-20; host: x86_64-unknown-linux-gnu; release: 1.96.0-nightly; LLVM version: 22.1.0`。
- 编译路径：`-C target-cpu=native`；报告确认启用 `aes, pclmulqdq, avx2`，本机没有启用 AVX-512。
- 基线 revision：`b684b1258e4b1f202bec24afd660ace851b09e5e`；工作区包含新增 Mikros 代码，Git dirty=true。Flock 的算法源文件未修改。
- 每尺寸每系统预热 1 次，计时 5 次，报告中位数；系统执行顺序按轮次轮换，使用完全相同的 packed A/B/C 与输入 digest。
- 两边均使用 F128、BLAKE3 transcript、关闭 grinding；Mikros kappa=128，Flock K_SKIP=6。此处不声称两者具有相同的 128-bit soundness。
- 计时包含全部在线 reduction 工作与每次证明的动态查表构建；固定表的首次使用保留在预热记录中，未从时间中扣除。
- 排除输入生成/digest、challenger 初始化、序列化、源输入求值、packed-opening reduction 和底层 PCS。每一份证明仍在计时外重放 verifier、核对 transcript 状态并核验全部三个源输入声明。
- verifier 时间是 PIOP verifier 时间；这不是已接入 commitment/opening 的完整 SNARK 测量。后续独立阶段诊断见 [PERFORMANCE_ANALYSIS.md](PERFORMANCE_ANALYSIS.md)，本表保留原始无诊断日志的总耗时测量。

## Prover 时间

单位为 ms。`Mikros/Flock` 是耗时之比，大于 1 表示 Mikros 更慢。

| 线程 | N | Mikros lookup | Flock degree-two | Mikros/Flock |
| ---: | ---: | ---: | ---: | ---: |
| 1 | 2^16 | 1.126 | 0.657 | 1.71 |
| 1 | 2^18 | 3.562 | 1.661 | 2.14 |
| 1 | 2^20 | 13.035 | 4.081 | 3.19 |
| 1 | 2^22 | 52.717 | 14.954 | 3.53 |
| 1 | 2^24 | 210.367 | 56.041 | 3.75 |
| 6 | 2^16 | 1.134 | 1.622 | 0.70 |
| 6 | 2^18 | 2.286 | 2.733 | 0.84 |
| 6 | 2^20 | 6.644 | 3.220 | 2.06 |
| 6 | 2^22 | 18.010 | 7.010 | 2.57 |
| 6 | 2^24 | 56.545 | 15.480 | 3.65 |

## 最大测量尺寸的 verifier 与通信量

消息大小按实际 bincode 序列化测量，不包括公共输入或任何 PCS 消息。

| 线程 | N | Mikros verify (ms) | Flock verify (ms) | Mikros bytes | Flock bytes |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 2^24 | 0.1119 | 0.1691 | 35,608 | 2,704 |
| 6 | 2^24 | 0.1126 | 0.1971 | 35,608 | 2,704 |

## 独立进程峰值 RSS

每格为一个新进程中的单次冷启动运行。Linux `VmHWM` 在 prover 返回后、verify 与源输入核验前读取；包含输入、runtime、线程和表，不能解释为纯算法工作区。计时表不使用这些冷启动时间。

| 线程 | N | 系统 | 证明前 RSS (MiB) | 峰值 RSS (MiB) |
| ---: | ---: | --- | ---: | ---: |
| 1 | 2^24 | flock | 8.715 | 20.781 |
| 1 | 2^24 | lookup | 8.758 | 14.801 |
| 6 | 2^24 | flock | 12.902 | 24.926 |
| 6 | 2^24 | lookup | 12.746 | 18.676 |

## 标量参考与查表的差分运行

参考版用于正确性 oracle，递归变换含大量临时分配；下面的差值不能当作对优化标量实现的算法级加速比。

| N | reference prover (ms) | lookup prover (ms) |
| ---: | ---: | ---: |
| 2^13 | 22.178 | 0.346 |
| 2^16 | 83.257 | 1.207 |
| 2^18 | 369.546 | 3.587 |

## 正确性与复现

这些 JSON 共保存 178 份成功重放及源输入核验的证明，包括预热、正式样本和 RSS 运行。任何 verifier、transcript 或源输入检查失败都会使 benchmark 返回失败，不写入成功样本。

```sh
bash mikros/run_comparison.sh
python3 mikros/summarize_results.py
```

逐样本数值与完整环境在 [results/](results/)；汇总数据在 [summary.csv](results/summary.csv)。重运行会更新同名结果文件。

本机结果仅覆盖当前实现和编译路径。新增阶段 profile 已定位张量聚合为大尺寸主瓶颈，详见 [PERFORMANCE_ANALYSIS.md](PERFORMANCE_ANALYSIS.md)；优化收益尚待实现和测量。
