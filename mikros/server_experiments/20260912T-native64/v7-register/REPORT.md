# v7：寄存器转换与打包优化的同 binary 比较

N=2^30 的 results 组：registerpackedreuse prover 中位数 83.000 ms，v6 基线 basisreuse 为 82.329 ms，Flock-128 为 93.193 ms。候选/基线中位数之比 = 1.008，候选/Flock = 0.891；候选对 Flock 的配对比值中位数 = 0.845，9 个配对中较快 8 次。

候选范围 [67.437, 101.412] ms；Flock 范围 [78.114, 125.778] ms。上述差异仅描述这批配对样本，不据此保证稳定领先。

相对 basisreuse，候选的逐输入配对比值中位数为 0.948，较快 6/9 次；prover 均值为候选 81.107 ms、基线 84.925 ms。

对基线的中位数比较与配对统计方向不同，不能只选择较有利的指标；本轮尚不能确认端到端改进。候选保持显式可选，默认路径不变。

本组基线和候选的中位数都低于 Flock，不能将两者与 Flock 的全部差距归因于新候选。

环境：Intel(R) Xeon(R) Gold 6342 CPU @ 2.80GHz，Linux x86_64，64 线程固定在 `0-47,48-55,72-79`；48 个物理核心加 16 个 SMT sibling，两 NUMA 节点各 32 个逻辑 CPU。Linux 默认 first-touch，无显式内存绑定/交错；服务器共享，CPU 不独占。

构建使用 `+nightly-2026-03-21`、`target-cpu=native`，GFNI/AVX512 后端及所需 ISA 特性已核验。各组使用相同冻结源码与 executable，按组和 N 分开统计。

## 候选与边界

`basisreuse` 是保持不变的 v6 基线；`registerreuse` 是同一坐标转换的寄存器优化候选。`registerpackedreuse` 还改变 A/B 打包中的 IP 消息累加方式：每个 chunk 内持续累加，到 chunk 末尾才水平合并。因此 registerpacked 不属于仅优化首轮聚合的消融。若列出 `reuse`，它仍代表最初的 Native 基线。

冻结 metadata 中的 `across packing chunks` 措辞不精确；实现没有跨 Rayon 任务共享累加器，原始记录保留供追溯。

这些是显式可选的实现路径：GF256 嵌入、扩域数学基、采样点、商多项式、挑战域和证明格式均不因此改变。同输入的全部 Mikros 路径必须给出相同 proof 字节及 claims/transcript。寄存器候选的名称本身不证明实际无 spill；缓存流量或带宽的变化也不能仅由耗时反推。

比较的是 κ=128 的单变量 Hadamard PIOP，终止于直接 packed-opening claims；未包含额外 2κ 列聚合、PCS 或承诺打开。Flock 为 skip-7，同压缩比 128。BLAKE3、关闭 grinding，不作相同安全等级声明。

## 分组计时

每维度每系统一次 warmup，然后轮转执行；正式次数是系统数的整数倍，各系统在每个位置出现次数相同。每个 reuse 模式拥有独立 scratch，动态值重新计算；Flock 使用生产内存池。固定表初始化未扣除，发生在进程中的首次使用；每系统 warmup 不是独立冷启动。

### screening / N26：每系统 12 次

UTC：2026-09-12T11:32:45Z 至 2026-09-12T11:34:56Z。

| 系统 | prover 中位数 [min, max] ms | prover 均值 ms | verifier 中位数 ms | 源输入检查中位数 ms | warmup prover ms |
|---|---:|---:|---:|---:|---:|
| v6 基线 basisreuse | 10.014 [9.145, 17.490] | 10.900 | 0.0299 | 866.994 | 16.008 |
| 转换候选 registerreuse | 9.723 [9.142, 16.402] | 10.303 | 0.0295 | 866.979 | 13.932 |
| 转换及打包候选 registerpackedreuse | 9.628 [8.956, 11.576] | 9.777 | 0.0298 | 866.249 | 14.034 |
| Flock-128 | 11.234 [9.724, 11.724] | 11.047 | 0.3885 | 548.960 | 13.214 |

| 分子/分母 | 两中位数之比 | 配对比值中位数 [min, max] | 分子较快 / 配对数 |
|---|---:|---:|---:|
| basisreuse/flock128 | 0.891 | 0.893 [0.800, 1.578] | 8/12 |
| registerreuse/basisreuse | 0.971 | 0.982 [0.694, 1.079] | 7/12 |
| registerreuse/flock128 | 0.865 | 0.881 [0.814, 1.480] | 11/12 |
| registerpackedreuse/basisreuse | 0.961 | 0.976 [0.662, 1.044] | 7/12 |
| registerpackedreuse/registerreuse | 0.990 | 0.973 [0.706, 1.034] | 7/12 |
| registerpackedreuse/flock128 | 0.857 | 0.876 [0.795, 1.045] | 10/12 |

来源：[计时](screening/comparison-64t.json)、[环境](screening/server_environment.json)。

### results / N30：每系统 9 次

UTC：2026-09-12T11:36:26Z 至 2026-09-12T11:45:50Z。

| 系统 | prover 中位数 [min, max] ms | prover 均值 ms | verifier 中位数 ms | 源输入检查中位数 ms | warmup prover ms |
|---|---:|---:|---:|---:|---:|
| v6 基线 basisreuse | 82.329 [69.547, 106.213] | 84.925 | 0.0589 | 13964.303 | 90.816 |
| 转换及打包候选 registerpackedreuse | 83.000 [67.437, 101.412] | 81.107 | 0.0473 | 13965.977 | 83.476 |
| Flock-128 | 93.193 [78.114, 125.778] | 95.667 | 0.5930 | 8804.502 | 100.657 |

| 分子/分母 | 两中位数之比 | 配对比值中位数 [min, max] | 分子较快 / 配对数 |
|---|---:|---:|---:|
| basisreuse/flock128 | 0.883 | 0.883 [0.777, 1.064] | 8/9 |
| registerpackedreuse/basisreuse | 1.008 | 0.948 [0.861, 1.087] | 6/9 |
| registerpackedreuse/flock128 | 0.891 | 0.845 [0.720, 1.107] | 8/9 |

本 N30 组未同时包含 registerreuse 与 registerpackedreuse，不能分别量化转换优化和额外打包优化的独立贡献。

来源：[计时](results/comparison-64t.json)、[环境](results/server_environment.json)。

配对键为同组 (N, run)，并核对完整 input digest；比值小于 1 表示分子更快。两系统中位数之比与逐输入比值中位数不同。逐 run 时间及摘要保存在 SUMMARY.json。N26 若用于候选筛选，其样本不并入后续 N30；样本范围和胜场不保证统计显著性或跨机器性能。

## 通信与独立 RSS

| 组 / N | 系统 | F128 元素 | payload bytes | bincode bytes | prove 前 VmRSS MiB | prove 后 VmHWM MiB | 冷 proof ms |
|---|---|---:|---:|---:|---:|---:|---:|
| screening / 26 | basisreuse | 296 | 4736 | 4760 | 未运行 | 未运行 | 未运行 |
| screening / 26 | registerreuse | 296 | 4736 | 4760 | 未运行 | 未运行 | 未运行 |
| screening / 26 | registerpackedreuse | 296 | 4736 | 4760 | 未运行 | 未运行 | 未运行 |
| screening / 26 | flock128 | 297 | 4752 | 4784 | 未运行 | 未运行 | 未运行 |
| results / 30 | basisreuse | 304 | 4864 | 4888 | 395.81 | 923.16 | 93.676 |
| results / 30 | registerpackedreuse | 304 | 4864 | 4888 | 395.89 | 922.96 | 92.186 |
| results / 30 | flock128 | 305 | 4880 | 4912 | 395.80 | 791.83 | 94.911 |

RSS 每系统/维度来自一个独立进程的一次冷 proof；VmHWM 在 prove 后、verify 与源输入检查前读取。它包含输入及运行时，不是 allocator 专属峰值；冷计时不混入正式统计。未执行的 RSS 不推测。

## 阶段诊断

profile 是独立进程的一次完整轮转，各位置平衡；每系统诊断次数等于所选系统数，另有一次 warmup。阶段均值和占比排除 warmup：占比为阶段均值/该系统同组诊断 prover 均值，余项逐样本计算再取均值。计时器有开销，Flock 日志保留 0.01 ms；诊断不并入正式计时。

screening / N26 的 AB 打包阶段均值为基线 1.698 ms、候选 0.871 ms；同组诊断 prover 总均值为基线 11.724 ms、候选 9.308 ms。

**screening / N26 / basisreuse**：4 个诊断样本，prover 均值 11.724 ms。

| 阶段 | 均值 ms | 占诊断 prover 均值 |
|---|---:|---:|
| 权重与准备 | 1.372 | 11.70% |
| GF256 小域聚合 | 4.895 | 41.75% |
| 插值与商多项式 | 0.380 | 3.24% |
| C 打包与投影 | 0.913 | 7.79% |
| Transcript 与挑战 | 0.020 | 0.17% |
| A/B 打包 | 1.698 | 14.48% |
| 内积尾部 | 2.436 | 20.78% |
| 终端 claims | 0.005 | 0.04% |
| 未计入阶段的余项（含舍入） | 0.005 | 0.05% |

**screening / N26 / registerreuse**：4 个诊断样本，prover 均值 11.848 ms。

| 阶段 | 均值 ms | 占诊断 prover 均值 |
|---|---:|---:|
| 权重与准备 | 1.438 | 12.14% |
| GF256 小域聚合 | 5.064 | 42.74% |
| 插值与商多项式 | 0.371 | 3.13% |
| C 打包与投影 | 1.049 | 8.86% |
| Transcript 与挑战 | 0.021 | 0.18% |
| A/B 打包 | 1.756 | 14.82% |
| 内积尾部 | 2.142 | 18.08% |
| 终端 claims | 0.003 | 0.03% |
| 未计入阶段的余项（含舍入） | 0.004 | 0.03% |

**screening / N26 / registerpackedreuse**：4 个诊断样本，prover 均值 9.308 ms。

| 阶段 | 均值 ms | 占诊断 prover 均值 |
|---|---:|---:|
| 权重与准备 | 0.732 | 7.87% |
| GF256 小域聚合 | 4.902 | 52.66% |
| 插值与商多项式 | 0.362 | 3.88% |
| C 打包与投影 | 0.992 | 10.66% |
| Transcript 与挑战 | 0.018 | 0.19% |
| A/B 打包 | 0.871 | 9.36% |
| 内积尾部 | 1.425 | 15.31% |
| 终端 claims | 0.003 | 0.03% |
| 未计入阶段的余项（含舍入） | 0.003 | 0.04% |

**screening / N26 / flock128**：4 个诊断样本，prover 均值 11.083 ms。

| 阶段 | 均值 ms | 占诊断 prover 均值 |
|---|---:|---:|
| 首轮 URM | 6.955 | 62.75% |
| 第二轮融合折叠 | 2.065 | 18.63% |
| 第三轮及后续尾部 | 1.800 | 16.24% |
| 未计入阶段的余项（含舍入） | 0.263 | 2.37% |

来源：[profile JSON](screening/profile-64t.json)、[Flock 阶段日志](screening/profile-64t.log)。

results / N30 的 AB 打包阶段均值为基线 15.084 ms、候选 17.617 ms；同组诊断 prover 总均值为基线 83.991 ms、候选 87.250 ms。

N30 的 3 个诊断样本未复现 N26 打包阶段的缩短。诊断样本少、结果有波动，不能宣称 N30 打包耗时减半或稳定加速；这些阶段均值也不与正式计时混合。

**results / N30 / basisreuse**：3 个诊断样本，prover 均值 83.991 ms。

| 阶段 | 均值 ms | 占诊断 prover 均值 |
|---|---:|---:|
| 权重与准备 | 6.996 | 8.33% |
| GF256 小域聚合 | 40.827 | 48.61% |
| 插值与商多项式 | 0.567 | 0.67% |
| C 打包与投影 | 6.491 | 7.73% |
| Transcript 与挑战 | 0.025 | 0.03% |
| A/B 打包 | 15.084 | 17.96% |
| 内积尾部 | 13.988 | 16.65% |
| 终端 claims | 0.008 | 0.01% |
| 未计入阶段的余项（含舍入） | 0.005 | 0.01% |

**results / N30 / registerpackedreuse**：3 个诊断样本，prover 均值 87.250 ms。

| 阶段 | 均值 ms | 占诊断 prover 均值 |
|---|---:|---:|
| 权重与准备 | 8.442 | 9.68% |
| GF256 小域聚合 | 39.915 | 45.75% |
| 插值与商多项式 | 0.388 | 0.44% |
| C 打包与投影 | 5.677 | 6.51% |
| Transcript 与挑战 | 0.043 | 0.05% |
| A/B 打包 | 17.617 | 20.19% |
| 内积尾部 | 15.142 | 17.35% |
| 终端 claims | 0.020 | 0.02% |
| 未计入阶段的余项（含舍入） | 0.005 | 0.01% |

**results / N30 / flock128**：3 个诊断样本，prover 均值 99.622 ms。

| 阶段 | 均值 ms | 占诊断 prover 均值 |
|---|---:|---:|
| 首轮 URM | 51.890 | 52.09% |
| 第二轮融合折叠 | 26.623 | 26.72% |
| 第三轮及后续尾部 | 20.760 | 20.84% |
| 未计入阶段的余项（含舍入） | 0.349 | 0.35% |

来源：[profile JSON](results/profile-64t.json)、[Flock 阶段日志](results/profile-64t.log)。

[本地坐标转换汇编诊断](diagnostics/local-basis-assembly-audit.md)。指令与内存操作数分析不等于实际 cache miss、DRAM 流量或硬件事件计数。

[本地打包汇编诊断](diagnostics/local-packing-assembly-audit.md)。指令与内存操作数分析不等于实际 cache miss、DRAM 流量或硬件事件计数。

[本轮前置检查](diagnostics/preflight.json)记录 `numactl=null`、`perf_event_paranoid=4`。本轮未实际尝试 perf 采集，也未实施 NUMA 调优；这些前置状态不能当作 v7 的 perf 失败日志或性能计数器结果。

## 核验与来源

共核验 149 条 proof 记录（含 smoke、warmup、正式、profile、RSS），服务器测试通过 76 项。各组 status=complete，全部命令零退出，日志 SHA256 与命令记录一致；组间共用相同 code/compiler/ISA/可执行文件。

本地运行时测试通过 75 项；显式指定 `--target x86_64-unknown-linux-gnu`、`RUSTFLAGS='-C target-cpu=icelake-server'` 的 `clippy --all-targets --no-deps -- -D warnings` 通过，格式检查通过。该 Clippy 结论限于 Mikros 检查范围，不外推到整个依赖树。服务器测试数与上述本地检查分别记录。

全部记录均完成 verifier、claims/transcript 一致性和独立完整源输入检查；同 (N, run) 输入摘要一致，全部 Mikros 模式的 proof 摘要一致，Flock 自身亦一致。不同协议间不要求 proof 摘要相同。输入生成/摘要、challenger 创建、序列化、源检查排除于 prove/verify 计时，但仍实际执行；verifier 仅为归约重放。

冻结源码 base revision：`b684b1258e4b1f202bec24afd660ace851b09e5e`；这不等于工作区无修改，精确文件由[source manifest](source_manifest.json) 和[源包](source.tar.gz)确定。各组使用同一 [register build artifact](register_build_artifact.json)。

Executable SHA256：`3da890dca453354ffff72a039c9a3715b05b08f7b4a0dd7a2b65e12a80c572ea`。

Mikros BLAKE3：`2940593c8147b2352c32dd90c7448d447d9ea41e667260124c48b391675ba454`。

Flock BLAKE3：`448ffdfa0646285bf27012c2af2b3ced1b146e1d312c592a194ffb15150065bb`。

构建配置 BLAKE3：`cf6b3d7e00e2656f6c7e59dc47db0a7f6141344dd7f4f065edacbf2cae37b876`。

证据 SHA256、后处理器 SHA256、配对记录及未四舍五入统计见 [SUMMARY.json](SUMMARY.json)。v5/v6 驱动、校验器和历史结果保持原状，历史样本不与 v7 混合；本后处理器位于冻结源包之外。

另与 v6 已完成记录核对重叠的输入/证明身份：v7 中 140 条记录匹配，涵盖 44 个不同 (N, run, 协议族)；9 条记录没有对应 v6 run，未声称经过跨版本核对。这只检验同输入下证明摘要一致，不混合源码指纹、计时样本或据此计算跨版本加速比。
