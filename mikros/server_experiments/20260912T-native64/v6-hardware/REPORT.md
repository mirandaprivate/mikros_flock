# v6：单变量 Hadamard 的硬件消融与 Flock-128 对比

同一可执行文件的 N=2^30 测量中，basisreuse prover 中位数 85.427 ms，旧核 reuse 为 95.523 ms，Flock-128 为 86.268 ms。候选/旧核 = 0.894，候选/Flock = 0.990；候选在 6 组配对输入中较 Flock 更快 3 次。

候选与 Flock 中位数相差 0.975%；候选观测范围 [80.306, 98.020] ms，Flock 为 [79.221, 97.236] ms。不能仅凭中位数略低称为稳定领先。

环境：Intel(R) Xeon(R) Gold 6342 CPU @ 2.80GHz，Linux x86_64；64 线程固定在 `0-47,48-55,72-79`，覆盖 48 个物理核心及 16 个 SMT sibling，两 NUMA 节点各 32 个逻辑 CPU。Linux 默认 first-touch，无显式内存绑定/交错；共享服务器，CPU 不独占。

使用 `+nightly-2026-03-21` 和 `target-cpu=native`；编译特性包含 GFNI、AVX512F/BW、VPCLMUL，报告确认 GFNI/AVX512 后端。N26 筛选与 N30 测量使用同一冻结 snapshot、同一 executable，分组统计。

## 实现与协议边界

`reuse` 保留 v5 Native 实现；`tiledreuse` 用 64 行、4 坐标的寄存器分块聚合；`basisreuse` 再用批量 GFNI 仿射变换完成原来的坐标转换。候选均需显式选择，默认仍为 `reuse,flock128`。名称中的 basis 指坐标转换实现：GF256 嵌入、扩域数学基、采样点、商多项式、挑战域均未改变。

冻结 benchmark 元数据中的 `its new GF256 basis` 措辞不精确，应理解为同一数学基上的新批量转换实现；原始元数据保留供追溯，不修改冻结源码或实验记录。

分块让累加值在寄存器中复用、批量转换减少标量查表，这是代码结构事实；缓存流量下降是据此提出的解释。本次没有硬件性能计数器，不能声称已测得 cache miss 或内存带宽下降。

下载后的汇编检查仅确认四坐标累加热循环没有 accumulator spill/reload，不代表整个聚合或坐标转换没有内存访问。具体依据见[本地汇编诊断](diagnostics/local-assembly-audit.md)；[perf 权限记录](diagnostics/perf-access.txt)说明为何没有硬件计数器数据。

比较边界是 κ=128、GF256 中间域的单变量 PIOP，终止于直接 packed-opening claims；未加入额外 2κ 列聚合、PCS 或承诺打开。Flock 使用 skip-7，同压缩比 128。BLAKE3、关闭 grinding，不据此声明相同安全等级。

## 分组计时与配对比较

每维度每系统一次 warmup，随后按 run 轮换顺序；正式次数能被系统数整除，各位置出现次数相同。各 reuse 模式独立保留 scratch 容量，动态值重新计算；Flock 使用生产内存池。首次 fixed-table 初始化在进程内某个先执行 proof 中发生，未扣除；warmup 不等于每系统独立冷启动。

### N26 候选筛选：screening，每系统 12 次

UTC：2026-09-12T10:39:26Z 至 2026-09-12T10:41:34Z。

| 系统 | prover 中位数 [min, max] ms | verifier 中位数 ms | 完整源检查中位数 ms | warmup prover ms |
|---|---:|---:|---:|---:|
| 旧核 reuse | 10.689 [7.910, 16.539] | 0.0296 | 841.098 | 15.860 |
| 寄存器分块 tiledreuse | 10.126 [4.940, 17.180] | 0.0304 | 843.449 | 11.296 |
| 分块及批量坐标转换 basisreuse | 9.738 [9.260, 17.100] | 0.0299 | 845.996 | 13.953 |
| Flock-128 | 10.953 [10.063, 11.723] | 0.3900 | 550.810 | 12.982 |

| 配对比值（分子/分母） | 两中位数之比 | 配对比值中位数 [min, max] | 分子较快 / 配对数 |
|---|---:|---:|---:|
| reuse/flock128 | 0.976 | 1.002 [0.714, 1.430] | 6/12 |
| tiledreuse/reuse | 0.947 | 0.915 [0.620, 1.069] | 9/12 |
| tiledreuse/flock128 | 0.924 | 0.941 [0.443, 1.485] | 10/12 |
| basisreuse/reuse | 0.911 | 0.905 [0.592, 2.162] | 9/12 |
| basisreuse/flock128 | 0.889 | 0.917 [0.820, 1.636] | 9/12 |

配对键为同组 (N, run)，并核对完整 input digest；比值小于 1 表示分子更快。配对比值中位数不同于两系统中位数之比。逐 run 时间、比值和输入摘要保存在 SUMMARY.json。

数据：[计时](screening/comparison-64t.json)、[环境](screening/server_environment.json)。

### N30 同 snapshot 测量：results，每系统 6 次

UTC：2026-09-12T10:46:09Z 至 2026-09-12T10:53:33Z。

| 系统 | prover 中位数 [min, max] ms | verifier 中位数 ms | 完整源检查中位数 ms | warmup prover ms |
|---|---:|---:|---:|---:|
| 旧核 reuse | 95.523 [79.388, 96.935] | 0.0393 | 13650.001 | 106.428 |
| 分块及批量坐标转换 basisreuse | 85.427 [80.306, 98.020] | 0.0323 | 13663.798 | 103.511 |
| Flock-128 | 86.268 [79.221, 97.236] | 0.4165 | 8831.552 | 80.307 |

| 配对比值（分子/分母） | 两中位数之比 | 配对比值中位数 [min, max] | 分子较快 / 配对数 |
|---|---:|---:|---:|
| reuse/flock128 | 1.107 | 1.106 [0.961, 1.117] | 2/6 |
| basisreuse/reuse | 0.894 | 0.925 [0.828, 1.079] | 4/6 |
| basisreuse/flock128 | 0.990 | 0.997 [0.915, 1.077] | 3/6 |

配对键为同组 (N, run)，并核对完整 input digest；比值小于 1 表示分子更快。配对比值中位数不同于两系统中位数之比。逐 run 时间、比值和输入摘要保存在 SUMMARY.json。

数据：[计时](results/comparison-64t.json)、[环境](results/server_environment.json)。

N26 用于候选选择，N30 是选定候选的后续评估；未把筛选样本并入后续样本。N30 只比较 Native 与 TiledBasis 组合候选，没有独立 tiledreuse 测量，不能将 N30 总改进分别归因于分块和批量转换。样本数、范围和胜场不保证统计显著性或跨机器性能。v5 报告及 v4 mixed-tensor 均保持历史记录，不用于本组加速比。

## 通信与独立 RSS

| 组 / N | 系统 | F128 元素 | payload bytes | bincode bytes | prove 前 VmRSS MiB | prove 后 VmHWM MiB | 冷 proof ms |
|---|---|---:|---:|---:|---:|---:|---:|
| screening / 26 | reuse | 296 | 4736 | 4760 | 未运行 | 未运行 | 未运行 |
| screening / 26 | tiledreuse | 296 | 4736 | 4760 | 未运行 | 未运行 | 未运行 |
| screening / 26 | basisreuse | 296 | 4736 | 4760 | 未运行 | 未运行 | 未运行 |
| screening / 26 | flock128 | 297 | 4752 | 4784 | 未运行 | 未运行 | 未运行 |
| results / 30 | reuse | 304 | 4864 | 4888 | 395.96 | 921.96 | 103.391 |
| results / 30 | basisreuse | 304 | 4864 | 4888 | 395.84 | 923.04 | 111.221 |
| results / 30 | flock128 | 305 | 4880 | 4912 | 395.90 | 791.90 | 100.858 |

RSS 是每系统/维度一个独立进程中的一次冷 proof；VmHWM 在 prove 后、verify 和源检查前读取。包含输入及运行时，不是 allocator 专属峰值；冷 proof 时间不混入正式统计。未运行 RSS 的组不推测内存数值。

## 阶段诊断

profile 来自独立进程，不混入正式计时；每系统另有一次 warmup，表中仅取正式诊断样本。阶段占比为阶段均值/该系统同组诊断 prover 均值，余项逐样本计算再取均值。计时器有额外开销；Flock 原始日志阶段精度为 0.01 ms。未运行 profile 的组不推测阶段耗时。

screening / N26 的小域聚合阶段均值：旧核 4.509 ms，组合候选 4.982 ms（候选/旧核 = 1.105）。这是该组 profile 样本的阶段比较，不与正式 prover 中位数拼接。

**screening / N26 / reuse**：3 个诊断样本，prover 均值 12.696 ms；诊断顺序位置不平衡。

| 阶段 | 均值 ms | 占诊断 prover 均值 |
|---|---:|---:|
| 权重与准备 | 1.360 | 10.71% |
| GF256 小域聚合 | 4.509 | 35.52% |
| 插值与商多项式 | 0.520 | 4.10% |
| C 打包与投影 | 0.858 | 6.76% |
| Transcript 与挑战 | 0.024 | 0.19% |
| A/B 打包 | 1.718 | 13.53% |
| 内积尾部 | 3.695 | 29.10% |
| 终端 claims | 0.006 | 0.05% |
| 未计入阶段的余项（含舍入） | 0.006 | 0.05% |

**screening / N26 / tiledreuse**：3 个诊断样本，prover 均值 24.115 ms；诊断顺序位置不平衡。

| 阶段 | 均值 ms | 占诊断 prover 均值 |
|---|---:|---:|
| 权重与准备 | 3.421 | 14.19% |
| GF256 小域聚合 | 8.259 | 34.25% |
| 插值与商多项式 | 0.929 | 3.85% |
| C 打包与投影 | 3.838 | 15.92% |
| Transcript 与挑战 | 0.023 | 0.09% |
| A/B 打包 | 4.961 | 20.57% |
| 内积尾部 | 2.674 | 11.09% |
| 终端 claims | 0.005 | 0.02% |
| 未计入阶段的余项（含舍入） | 0.005 | 0.02% |

**screening / N26 / basisreuse**：3 个诊断样本，prover 均值 15.666 ms；诊断顺序位置不平衡。

| 阶段 | 均值 ms | 占诊断 prover 均值 |
|---|---:|---:|
| 权重与准备 | 1.862 | 11.88% |
| GF256 小域聚合 | 4.982 | 31.81% |
| 插值与商多项式 | 0.627 | 4.00% |
| C 打包与投影 | 2.189 | 13.97% |
| Transcript 与挑战 | 0.027 | 0.17% |
| A/B 打包 | 2.615 | 16.69% |
| 内积尾部 | 3.351 | 21.39% |
| 终端 claims | 0.004 | 0.03% |
| 未计入阶段的余项（含舍入） | 0.009 | 0.05% |

**screening / N26 / flock128**：3 个诊断样本，prover 均值 20.546 ms；诊断顺序位置不平衡。

| 阶段 | 均值 ms | 占诊断 prover 均值 |
|---|---:|---:|
| 首轮 URM | 9.737 | 47.39% |
| 第二轮融合折叠 | 5.177 | 25.19% |
| 第三轮及后续尾部 | 5.137 | 25.00% |
| 未计入阶段的余项（含舍入） | 0.496 | 2.42% |

来源：[profile JSON](screening/profile-64t.json)、[Flock 阶段日志](screening/profile-64t.log)。

results / N30 的小域聚合阶段均值：旧核 39.228 ms，组合候选 32.844 ms（候选/旧核 = 0.837）。这是该组 profile 样本的阶段比较，不与正式 prover 中位数拼接。

**results / N30 / reuse**：3 个诊断样本，prover 均值 96.569 ms；诊断顺序位置平衡。

| 阶段 | 均值 ms | 占诊断 prover 均值 |
|---|---:|---:|
| 权重与准备 | 7.170 | 7.42% |
| GF256 小域聚合 | 39.228 | 40.62% |
| 插值与商多项式 | 0.458 | 0.47% |
| C 打包与投影 | 7.283 | 7.54% |
| Transcript 与挑战 | 0.033 | 0.03% |
| A/B 打包 | 19.782 | 20.48% |
| 内积尾部 | 22.602 | 23.41% |
| 终端 claims | 0.007 | 0.01% |
| 未计入阶段的余项（含舍入） | 0.006 | 0.01% |

**results / N30 / basisreuse**：3 个诊断样本，prover 均值 89.609 ms；诊断顺序位置平衡。

| 阶段 | 均值 ms | 占诊断 prover 均值 |
|---|---:|---:|
| 权重与准备 | 6.785 | 7.57% |
| GF256 小域聚合 | 32.844 | 36.65% |
| 插值与商多项式 | 0.410 | 0.46% |
| C 打包与投影 | 8.382 | 9.35% |
| Transcript 与挑战 | 0.024 | 0.03% |
| A/B 打包 | 20.146 | 22.48% |
| 内积尾部 | 21.007 | 23.44% |
| 终端 claims | 0.005 | 0.01% |
| 未计入阶段的余项（含舍入） | 0.005 | 0.01% |

**results / N30 / flock128**：3 个诊断样本，prover 均值 85.021 ms；诊断顺序位置平衡。

| 阶段 | 均值 ms | 占诊断 prover 均值 |
|---|---:|---:|
| 首轮 URM | 42.333 | 49.79% |
| 第二轮融合折叠 | 20.567 | 24.19% |
| 第三轮及后续尾部 | 21.757 | 25.59% |
| 未计入阶段的余项（含舍入） | 0.364 | 0.43% |

来源：[profile JSON](results/profile-64t.json)、[Flock 阶段日志](results/profile-64t.log)。

## 核验与来源

共核验 128 条 proof 记录，包含 smoke、warmup、正式、profile 和 RSS；服务器测试通过 72 项。各组 status=complete、全部命令零退出，命令日志 SHA256 与记录一致。同组及跨组 (N, run) 输入摘要一致；所有 Mikros 模式的 proof 字节摘要一致，Flock 自身亦一致。不同协议之间不要求 proof 摘要相同。

本地补充检查：格式检查通过，Mikros 的 `clippy --all-targets --no-deps -- -D warnings` 通过。首次包含依赖的严格 Clippy 因既有 flock-field x86_64 未使用 import 失败；该依赖问题保留，不将其表述为整个依赖树无警告。此处与服务器 72 项测试记录区分。

每条记录均完成 verifier、claims/transcript 一致性和独立完整源检查。输入生成/摘要、challenger 创建、序列化、源检查排除于 prove/verify 计时，但实际执行；源检查耗时单列。verifier 仅是归约重放。

源快照 base revision：`b684b1258e4b1f202bec24afd660ace851b09e5e`（并非声称工作区无修改）；精确文件内容由[source manifest](source_manifest.json) 和 [源包](source.tar.gz) 确定。本报告各组共享同一个[build artifact](hardware_build_artifact.json)，N30 复用已测试 binary。

Executable SHA256：`74b8a341c3c845d54f5f654ee66b57b4afff598c10a02479af2a485077001b28`。

Mikros BLAKE3：`2559f90d19ae6edc3ee042730228253267eb598bf6c8390f5fc7c4fae48844db`。

Flock BLAKE3：`448ffdfa0646285bf27012c2af2b3ced1b146e1d312c592a194ffb15150065bb`。

构建配置 BLAKE3：`cf6b3d7e00e2656f6c7e59dc47db0a7f6141344dd7f4f065edacbf2cae37b876`。

全部读取证据文件的 SHA256、后处理器 SHA256、配对数据及未四舍五入的统计在 [SUMMARY.json](SUMMARY.json)。后处理器不属于冻结源包，未修改其 manifest 或历史报告。
