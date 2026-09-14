# Mikros 为什么更慢，本轮优化解决了什么

本轮已经在双路 Xeon Gold 6342 上完成 native 编译和 64 线程实验。双方都启用 GFNI、AVX-512 和 VPCLMUL；固定使用 48 个物理核加 16 个 SMT sibling，跨两个 NUMA 节点。源码与数据属于同一 v3 快照，详细验收和原始日志见 [REPORT.md](REPORT.md)，图见 [native64_comparison.svg](native64_comparison.svg)。

## 实测结论

N=2^30 的主对照各测八次，Mikros 旧版 **406.56 ms**，native 每份证明新建工作区 **139.57 ms**，native 复用容量 **82.03 ms**，Flock **82.93 ms**。相对旧版分别加速 **2.91×** 和 **4.96×**。普通复用与 Flock 在这组基本持平；中位数差约 1%，小于观测波动。

补充的三系统容量复用实验各测六次，普通复用 **99.27 ms**，分层复用 **93.96 ms**，Flock **72.69 ms**。分层使 Mikros 中位数降低 **5.3%**，但仍用时 Flock 的 **1.29×**。两组单独保留，不混样本、不挑最小值。当前证据支持把差距缩到同一量级，**不支持稳定超过 Flock**；也不能只拿主实验的近似持平概括所有批次。

## 改变了哪些实际工作

- **变换立即消费**：保留 tensor 递归共享，在小叶节点中生成 A/B 结果后立即 AND 和聚合，消除两条完整 2187-u64 临时数组。
- **真正的 GFNI 聚合**：把任意 F128 权重的 subset-XOR 映射编码成二进制 affine matrices；每 64 行原来 32 KiB 的权重表变为 1 KiB 矩阵。最终使用 243 系数叶节点、64 系数批次和 byte planes，避免早期版本的水平归并。
- **VPCLMUL 与融合**：复用 Flock 的四路 F128 原语；packing 同时应用 B 权重并累计 IP 首消息，C 投影和 IP fold/message 使用宽累加器。N30 两条初始 IP 数组合计 256 MiB，首消息融合减少一次完整读取。
- **并行初始化与复用**：实际生成结果的线程直接写入 `MaybeUninit` 输出，全部写满后才发布长度；取消大数组的额外串行清零。blocked 行权重与持久 `ProverScratch` 分别减少 setup 和重复分配/释放。
- **分层权重候选**：保留原始随机挑战，将行权重分成外层与内层 tensor factors，共享本次证明内的 GFNI 矩阵，再用 VPCLMUL 乘外层权重。不是把随机挑战替换成固定值。

N24 的同 binary 消融中，旧版 24.17 → stream 11.57 → GFNI 9.90 → wide IP 8.91 → fusion 8.31 → native 5.91 ms，均为十六次中位数。这是累计工程变体对照，不能把总差值当成单独一条指令的收益。

## 剩余瓶颈及计时边界

以下为独立诊断的三次**均值**，不与主实验中位数相加。单位 ms。

| 阶段 | 旧版 | native 每次分配 | native 复用 |
| --- | ---: | ---: | ---: |
| 权重/setup | 21.03 | 12.30 | 3.16 |
| 张量聚合 | 272.57 | 41.38 | 49.24 |
| C 投影 | 4.70 | 4.53 | 6.14 |
| A/B packing | 12.31 | 19.01 | 12.29 |
| IP 阶段 | 94.56 | 15.59 | 11.62 |
| prover 总均值 | 413.90 | 135.89 | 82.64 |

张量从 272.57 降到 41.38 ms。复用版张量仍占总均值 **59.6%**，是最大的剩余阶段。GFNI 已消除了昂贵的随机查表路径，但仍需变换并聚合全部 2187 个系数；本轮没有用性能计数器把该阶段进一步分解成指令吞吐与 cache miss。

IP 阶段的下降不能全部归为乘法加速：首消息移入 packing，且 native 工作区的销毁边界发生变化。native 每次分配的总时间中有 **42.92 ms** 不在九个内部阶段内；大工作区在最后 checkpoint 后、调用方停止总计时前销毁，与该残差的位置吻合。复用版残差接近零。总计时已经包含这些成本；没有系统调用跟踪，不能把全部残差都断言为 `munmap`。仅比较 IP timer 会夸大独立 IP 内核的改善。

Flock 的 [scratch.rs](../../../crates/flock-core/src/scratch.rs) 有跨证明 F128 内存池，zerocheck 结束时归还四条 tail 数组。因此 `reuse` 对 Flock 比 `native` 对 Flock 更接近双方都保留容量的工程对照。Mikros 使用调用方拥有的 scratch，两者不共享池。独立冷进程的峰值 RSS 为普通复用约 **908 MiB**、Flock 约 **1162 MiB**，包含输入和运行时，不是纯算法空间。

## 分层版本的取舍与使用

在独立新建工作区的 N30 对照中，native 157.06 → factored 139.43 ms，中位数下降 11.2%；N24 则由 5.91 变成 6.18 ms，略慢。分层不是全尺寸默认收益：N30/64线程共享 1024 份块内矩阵，而普通路径构造 131072 份，同时新增 279936 次外层 F128 缩放。

当前 `Kernel::Packed` 保留通用 native 路径。连续证明用 `prove_with_scratch(..., Kernel::PackedNative, ..., &mut scratch)`；分层候选显式选择 `Kernel::PackedFactored`。CLI 分别为 `reuse`、`factoredreuse`，每个变体保留独立容量。示例和完整映射见 [NATIVE_OPTIMIZATION_NOTES.md](../../NATIVE_OPTIMIZATION_NOTES.md)。本轮已实现和测量这些优化，未声称穷尽所有可能的 binary 优化。

Flock 还利用协议固定的七个内层 equality 维度做小域归约；Mikros 的随机扩域挑战不能为追求相同算术而直接改成这些常量。协议消息也没有缩小：N30 Mikros **35,800 bytes**，Flock **2,896 bytes**。SIMD 和内存优化不改变这约 12.36 倍的消息大小差异。

所有比较只包含 PIOP reduction，不含 PCS 或 packed opening；双方 BLAKE3、grinding 关闭，不声称相同 soundness。每份 benchmark proof 都在计时外执行 verifier、transcript 检查和三个原始输入声明核验。共享服务器、默认 NUMA first-touch、SMT 和有限样本限制结果的精度，当前数据没有单独识别波动的具体成因。
