# 打包工程优化记录

本次优化保持 binary mixed-tensor Hadamard 协议、F128 域、kappa=128、挑战分布和完整证明消息不变。优化后的入口是 `prove(..., Kernel::Packed, ...)`。`Kernel::Lookup` 保留旧逐行实现，`Kernel::Packed8` 保留八行版本用于粒度消融，`Reference` 继续作为独立标量 oracle。

## 1. 跨行 bit slicing 与权重查表

原来的 transform LUT 只压缩列方向的 bit 变换，随后逐个 set bit 将一个行权重累加到 H。新内核先把 64 行的输入转置：每个 u64 存同一列在这 64 行上的 64 个 bit。T^tensor7 的 XOR 和 A/B 的 AND 因而同时处理 64 行。

对这 64 行，分成八组、每组八个 F128 行权重，在线构造八张 256-entry subset-XOR table：

`L_j[mask] = XOR_{bit k of mask = 1} row_weight[8*j+k]`。

每个系数的 u64 product mask 被分成八个 byte，查询八张表，将结果合并后只对 H 写回一次。行权重可以是任意 F128 值，不要求是固定挑战、F8 或特殊子域。所有转置、张量变换、动态表构造和 worker 缓冲区初始化都在 prover 的计时范围内。

8×8 和 64×64 transpose 的便携 SWAR/delta-swap 网络参考仓库 Flock 的私有实现，代码中保留出处。由于私有可见性以及本机公开 transpose fallback 的逐位循环，没有为此改变 Flock 公共接口或其算法。张量各级固定尺寸展开，使用安全 Rust，不新增 unsafe 内存操作。

N=2^24 时的源码级计数如下（非硬件指令计数）：

| 项目 | 旧 Lookup | Packed8 | Packed64 |
| --- | ---: | ---: | ---: |
| H 系数读改写 | 期望 71,663,616 | 35,831,808 | 4,478,976 |
| 动态权重表查询 | 无 | 35,831,808 | 35,831,808 |
| 动态权重表构造的 F128 加法 | 无 | 4,177,920 | 4,177,920 |

旧版期望来自独立均匀随机 A/B 的 product bit 为 1 的概率 1/4；打包版计数不依赖这些 bit 的密度。Packed64 相比 Packed8 的主要区别是减少 H 写回并扩大字运算处理范围，而非减少表查询数量。

每个 64-row worker 的 H 为 34,992 bytes，变换后 A/B 各 17,496 bytes，八张表共 32,768 bytes，另有少量转置/递归临时空间。只物化当前 block，不分配 m×2187 的完整变换矩阵。尾块输入和缺少的权重 lane 补零；并行任务边界对齐到 64 行。

## 2. 输入绑定与 C 投影融合

A/B 使用同一个 rho，现共用一份 64 KiB challenge-dependent byte table，在同一遍输入扫描中绑定两者，并把 B 的行权重直接乘到输出值上，消除重复构表和单独的 B weighting pass。

C 仍从传入的独立 C 输入计算，不用 A&B 替代。绑定一行后立即累计其 projection，不生成完整 c_packed 向量。使用 Flock 已公开的 `F128::mul_unreduced` 与 `F256Unreduced`，每个 chunk 汇总后才归约；GHASH 归约对 XOR 线性，所以求和值不变。

## 3. IP fold 与下一轮 message 融合

第一轮消息直接计算。随后每个 challenge 将四个输入折成两个输出，并在这些值仍可直接使用时累计下一轮消息；不再为下一条 message 单独扫描完整中间向量。两个半长 scratch Vec 在 proof 内分配一次并交替复用，分配计入在线时间，没有跨 proof 缓存。

乘积累加延后域归约。并行 worker 只写各自的输出切片；单线程直接执行串行内核。初版 4096-input 的并行门槛造成许多小任务，最终使用 32768-input 门槛和 4096-output chunks，较短的尾部直接串行处理。发送顺序、challenge 顺序、最后 A/B 观察以及返回坐标顺序均与旧实现相同。

## 4. 粒度选择的实测依据

同一个 executable、同一输入、相同融合 binding/IP 实现，只改变 Packed8/Packed64 的聚合粒度。每系统预热一次、六次正式样本，下面为 prover 中位数。该批带 Mikros 阶段时钟，用于粒度选择；最终无探针对照另见 OPTIMIZED_BENCHMARKS。

| N=2^24 | Packed8 | Packed64 | 八行/64 行 |
| --- | ---: | ---: | ---: |
| 1 线程 | 59.417 ms | 41.170 ms | 1.44× |
| 6 线程 | 22.481 ms | 15.489 ms | 1.45× |

原始数据：`results/optimization-pilot/rows64-vs8-1t.json` 和 `rows64-vs8-6t.json`。同批 Mikros 证明摘要完全相同。更早的 `rows8-*.json` 是初版八行与初版 IP 调度的三次试测，也保留原始数据，不混入最终统计。

复现粒度比较：

```sh
cargo +nightly-2026-03-21 bench -p mikros --bench hadamard_vs_flock -- --logs 20,24 --runs 6 --threads 1 --systems packed8,packed,flock --profile --output mikros/results/optimization-pilot/rows64-vs8-1t.json
```

## 5. 最终结果和代价

正式同轮对照的 N=2^24 prover 中位数：

| 线程 | 旧 Lookup | 新 Packed | Flock | Mikros 工程加速 |
| --- | ---: | ---: | ---: | ---: |
| 1 | 202.245 ms | 39.429 ms | 56.278 ms | 5.13× |
| 6 | 54.789 ms | 15.113 ms | 15.305 ms | 3.63× |

单线程新 Mikros 的耗时为 Flock 的约 70%，吞吐约 1.43×；六线程与 Flock 基本持平，约 1.3% 的中位数差异不足以声称稳定的领先。独立阶段均值显示，聚合由 194.47 降到 29.90 ms（1T，6.50×），由 49.84 降到 8.40 ms（6T，5.93×）。IP tail 由 3.39/3.56 降到 2.69/2.92 ms，收益小于聚合。

优化增加了内存使用：新 Mikros 的进程 peak RSS 为 17.03/26.89 MiB（1T/6T），旧版为 15.03/18.67 MiB，Flock 为 20.98/23.15 MiB。除了算法 scratch，进程 RSS 还受线程 runtime 和分配器影响，不能将差额全部归因于某一个数组。现在六线程 Mikros 的 RSS 高于 Flock，旧报告中的内存优势不能原样延用。

协议未变，消息仍是 Mikros 35,608 bytes、Flock 2,704 bytes（N=2^24）。本轮只覆盖随机 dense binary 输入的 PIOP reduction；原始全零等稀疏输入上旧版 set-bit 路径可能有不同的性能特点，正确性测试覆盖这些情况，但本次不宣称所有输入分布都获得相同加速。

## 6. 验证与公平比较的边界

34 个 Rust tests 全部通过，包含：全部 byte LUT 值、转置 basis、64 个 row lanes、任意 F128 权重、0/1/7/8/9/63/64/65/255/256/257/511/512/513 行边界、多线程分块、reference/lookup/packed8/packed 证明一致性、错误 C、消息篡改、原始三声明认证和 IP 调度边界。`cargo fmt`、`cargo clippy --all-targets --no-deps -- -D warnings` 通过；Flock 既有 AVX512 unused-import warning 没有修改。

`run_optimized.sh` 已完整执行。正式曲线、阶段实验和独立 RSS 共 272 份证明，每份都在计时外经过 verifier、transcript 状态和原始 A/B/C 声明检查。脚本还核对所有线程/批次相同输入的 proof digest，防止优化悄悄改变证明。所有最终报告的编译时 Mikros source fingerprint 相同，保存在 JSON 的 `mikros_sources_blake3`。

工程层面采用同一编译器、CPU 指令集、F128 backend、BLAKE3、输入、线程数和在线计时边界，保留 Flock 生产优化路径。双方各自的协议参数和 soundness 仍不同；没有人为替换 Mikros 随机坐标为 Flock 固定坐标，也没有声称相同的 128-bit soundness。packed-opening reduction 和底层 PCS 在两边都不计入本次对比。完整曲线与 RSS 明细见 [OPTIMIZED_BENCHMARKS.md](OPTIMIZED_BENCHMARKS.md)。
