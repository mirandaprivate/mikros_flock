# 完成结果：Mikros permutation 接入 Binius/Ligerito

正式结果为 [local-20260912-r2/REPORT.md](results/local-20260912-r2/REPORT.md)，
全部逐次样本、参数、源码/可执行文件哈希和 [summary.csv](results/local-20260912-r2/summary.csv) 一并保存。
使用本地 i7-8700K / WSL，固定物理 CPU 亲和性，每配置四对正式样本，报告中位数。

| N | value bits | 线程 | Mikros 完整证明 ms | Flock 完整证明 ms | Mikros/Flock |
|---:|---:|---:|---:|---:|---:|
| 2^20 | 1 | 1 | 1085.945 | 267.268 | 4.06 |
| 2^20 | 8 | 1 | 1405.592 | 450.299 | 3.12 |
| 2^20 | 32 | 1 | 2394.364 | 1159.318 | 2.07 |
| 2^20 | 8 | 6 | 1406.295 | 349.118 | 4.03 |

两边承诺完全相同的原始 bit 矩阵，并使用相同的 Flock Binius ring-switching + Ligerito
认证全部末端声明。Flock 端调用原生 batched product-GKR；这是统一 PCS 参数下的 permutation
对比，不是完整 R1CS prover 的性能。

在百万行、8-bit、单线程配置：

- PIOP：Mikros 805.261 ms，Flock 158.524 ms。
- 准备/表示转换：387.461 ms 与 114.493 ms。
- 承诺：16.835 ms 与 16.815 ms。
- 位列归约及公开 tag 处理：174.344 ms 与 132.398 ms。
- PCS opening：26.244 ms 与 25.600 ms。
- 完整验证：53.896 ms 与 40.162 ms；包括公开 sigma 的线性处理，不读取私有 f/g。
- proof：653.64 KiB 与 641.56 KiB，另计公开 cap 和 sigma。

接入共同 PCS 后，主要差距仍在当前 Mikros 的系数树/GKR PIOP 和额外 tuple 表示转换，
PCS opening 的差距很小。当前 Mikros 主循环仍主要为标量，六线程下总时间几乎不变；
Flock 原生 PIOP 从 158.524 ms 降到 60.234 ms，但总证明还包含公共适配层的串行工作。
这些是本次实现和测量的观察，不是对 Mikros 论文最优复杂度的结论。

正式运行十组配置，100 个证明（包括20个预热证明）全部验证通过；80 个正式样本进入统计。
正式运行没有检测到并发构建，未剔除慢样本。预跑和受构建干扰而中止的 r1 全部保留，
不混入 r2 的结果。[验证记录](CHECKS.md) 和 [接口/复现说明](README.md)。

两边采用相同的 Fast100 / 小形状 UDR 诊断 PCS 参数，外层不做 grinding，无 ZK 包装；
不宣称整套组合具有严格或相同的 128-bit soundness。
