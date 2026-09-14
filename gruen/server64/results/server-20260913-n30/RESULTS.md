# Mikros 与 Gruen：两域 N=2^30、64 线程服务器实测

2026-09-13。使用 `gruen/server64` 已有共同域后端和并行实现，未改动归约协议或算术内核。
Mikros 使用补点/Lagrange 优化版；同时保留同消息 reference、Gruen 普通轮和固定 skip-kappa 对照。

## Prover 时间

每种方法预热一次，正式重复八次；四种方法轮换执行位置，每个位置各两次。
下表为正式 prover wall time，单位 **秒**，不包括原始输入生成和独立 source oracle。

| 域 | 方法 | 中位数 | 最小–最大 |
|---|---|---:|---:|
| Goldilocks | Gruen 普通轮 | 7.408 | 7.298–7.541 |
| Goldilocks | Gruen skip-kappa | 9.263 | 9.098–9.548 |
| Goldilocks | Mikros reference | 7.026 | 6.848–7.141 |
| Goldilocks | **Mikros optimized** | **6.701** | **6.534–6.896** |
| KoalaBear | Gruen 普通轮 | 8.501 | 8.234–8.584 |
| KoalaBear | Gruen skip-kappa | 6.139 | 6.069–6.421 |
| KoalaBear | Mikros reference | 4.877 | 4.810–5.182 |
| KoalaBear | **Mikros optimized** | **4.770** | **4.742–4.828** |

Goldilocks：相对较快的 Gruen 普通轮，Mikros 的加速比为 **1.105×**，耗时降低 **9.54%**；相对跳轮版为 **1.382×**。

KoalaBear：相对较快的 Gruen 跳轮版，Mikros 的加速比为 **1.287×**，耗时降低 **22.29%**；相对普通轮为 **1.782×**。

两个域中，Mikros optimized 对两种 Gruen 对照均在 **8/8** 个同序号重复中更快。
本批 Mikros reference → optimized 的中位数降幅为 Goldilocks **4.61%**、KoalaBear **2.19%**。
后者较小，不能把 Mikros 相对 Gruen 的全部优势归因于插值优化。

## 差距所在

以下是同一计时中的阶段中位数，单位秒。各阶段单独取中位数，不应相加当作总时间。
外层 prover 计时还包含函数返回前的内存释放等工作。

| 域与方法 | setup | helpers / Q | packing | sumcheck 消息 | fold | terminal |
|---|---:|---:|---:|---:|---:|---:|
| Goldilocks Gruen 普通轮 | 0.652 | 0 | 0 | 1.359 | 4.454 | 0 |
| Goldilocks Gruen skip | 0.673 | 2.237 | 1.038 | 0.886 | 3.563 | 0 |
| Goldilocks Mikros optimized | 0.323 | 0.615 | 0.793 | 0.449 | 2.626 | 0.935 |
| KoalaBear Gruen 普通轮 | 0.901 | 0 | 0 | 2.028 | 4.624 | 0 |
| KoalaBear Gruen skip | 0.485 | 1.614 | 0.589 | 0.822 | 2.073 | 0 |
| KoalaBear Mikros optimized | 0.226 | 0.693 | 0.611 | 0.348 | 1.605 | 0.738 |

Mikros 缩短了内层 sumcheck，并用二次消息减少运算，消息与折叠均更便宜；代价是额外 helpers、打包及两组终端列一致性检查。Goldilocks 的压缩比只有 2，净收益较小。KoalaBear 压缩比为 4，且 Gruen 跳轮商构造比 Mikros helpers 更贵，因此净收益更大。此解释针对当前实现，不表示理论下界。

## 参数与计时边界

- Goldilocks：p=18446744069414584321，E=F[X]/(X²−7)，kappa=2，Gruen 跳 1 轮。
- KoalaBear：p=2130706433，E=F[X]/(X⁴−3)，kappa=4，Gruen 跳 2 轮。扩域存储基为 `[1,X²,X,X³]`。
- N 指 **2^30 条基域 Hadamard 约束**，不是 2^30 个输入比特。每种域内输入和确定性 verifier tape 相同。
- 两者共享标量域算术，使用 `-C target-cpu=native` 编译并通过 Rayon 并行；不是 Plonky3 手写 SIMD 峰值对比。
- 固定域 setup、equality tables、C、在线补点、打包、商、sumcheck、terminal、分配及返回前释放均计入 prover。未减去任何 online 转换成本。
- 全量 source oracle 预先按查询种类计算、独立计时；每份 proof 都使用相应 oracle 验证。它不属于 succinct PCS opening。
- 本实验是 Hadamard **PIOP 归约**，没有 PCS、Fiat–Shamir 或 E2E SNARK。两种扩域的大小不同，不宣称相同严格安全位数。

CPU 为双路 Xeon Gold 6342。实际创建 64-worker Rayon pool，固定亲和性 `0-47,48-55,72-79`，使用 48 个物理核和 16 个额外 SMT 线程，两个 NUMA 节点各 32 个逻辑 CPU。采用 first-touch，无独占 CPU 预留；驱动未检测到已知竞争 benchmark/build 进程。N30 的 process CPU / wall 中位数约为 36.4–43.5，不能称为 64 个物理核满载。

## 验证与复现

原生 release 的 **35 项测试**、fmt、Clippy 和构建均通过；测试包括 1/4/64 线程与冻结串行 reference 的逐消息比较。两个域的 N20 检查和 N30 正式实验共 **144 份证明**（含预热）全部验证；正式样本中 **32 对** Mikros reference/optimized 完整消息完全相同。源码、可执行文件哈希在实验前后不变。

首个启动在格式检查时发现服务器缺少 rustfmt，未产生实验测量；安装指定 nightly 的 rustfmt/clippy 后，在新 `results-v2` 目录完整重跑。首次失败日志与组件安装日志保留。未启用额外冷启动 RSS 实验，不报告峰值内存结论。

- [独立统计及配对核验](analysis.json)
- [原始结果目录](server_evidence/results-v2/accepted.json)
- [环境与源码/二进制哈希](server_evidence/results-v2/environment.json)
- [构建、测试及实验命令](server_evidence/results-v2/commands.json)
- [测试日志](server_evidence/results-v2/tests.log)
- [证据文件哈希清单](server_evidence/evidence-manifest.json)

证据归档 `evidence.tar.gz` SHA256：`27f1f8a0f3e6b295a7ef8b4d48434481242293731b171f239ee975e429badeaf`，163 个文件逐项核验。
冻结源码包位于 `../package-20260913-n30/`，SHA256：`f4a2ecf4d1c6e8a604bfe2e2f87254b271dff1ff62c31af253247e809109ce2d`。

```sh
python3 unpack.py
python3 ../../summarize_run.py server_evidence/results-v2
```

完整服务器路径：`/home/mscong1/mikros-experiments/20260912T-native64/gruen-20260913-n30/`。源码和历史结果保留，实验进程已经退出。
