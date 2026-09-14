# 优化前的 binary Mikros lookup 为什么比 Flock 慢

本文保留旧 Lookup 实现的诊断及原始数据。后续已完成跨行打包工程优化；新实测见 [OPTIMIZED_BENCHMARKS.md](OPTIMIZED_BENCHMARKS.md)，实施说明见 [OPTIMIZATION_NOTES.md](OPTIMIZATION_NOTES.md)。下文的“当前”均指当时尚未优化的 Lookup。

2026-09-12。本结论针对本仓库的 direct binary mixed-tensor Hadamard 实现（κ=128），与 Flock 默认 dense degree-two zerocheck 比较。测量边界均为 PIOP reduction，不含 PCS；不将结果推广到所有 Mikros 变体，也不声称双方有相同的 128-bit soundness。

**大尺寸的瓶颈是 Mikros 的张量证书聚合：单线程约占 93%，六线程约占 85%。** 当前 byte LUT 加速了 bit transform，但后续仍对每行的非零 product coordinates 分别更新完整 F128 系数。Flock 先以固定坐标、小域和专门查表合并大量贡献，再做完整 F128 加权；两者进入大域聚合的粒度明显不同。

## 1. 总差距：无阶段探针的正式计时

同一台 i7-8700K、WSL2、nightly-2026-03-21、PCLMUL/AVX2；每组预热一次，五次测量的中位数，单位 ms。

| N=2^24 | 原测 Mikros | 原测 Flock | 比值 | 本次对照 Mikros | 本次对照 Flock | 比值 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 线程 | 210.37 | 56.04 | 3.75× | 222.94 | 57.37 | 3.89× |
| 6 线程 | 56.54 | 15.48 | 3.65× | 63.16 | 16.96 | 3.72× |

原测见 [BENCHMARKS.md](BENCHMARKS.md)，本次见 [control-1t.json](results/profile/control-1t.json) 和 [control-6t.json](results/profile/control-6t.json)。本次六线程 Mikros 有一个 103.04 ms 样本，min 为 57.27 ms，故保留所有样本并使用中位数；没有删去慢样本。机器调度存在波动，不能把两批绝对耗时变化解释成代码优化或退化。

## 2. 实际 prover 阶段分解

Mikros 的 `prove` 与新增 `prove_profiled` 共用 `prove_impl`，只在后者启用九个阶段时钟。Flock 直接使用生产 `prove_packed` 的 `FLOCK_ZC_TIMING`；没有拿旧的复制版 phase benchmark 替代生产路径。

下表为 N=2^24 的五次预热后样本的**算术平均**，因此各阶段与 residual 可加回平均总时间。单位 ms；占比也以同批均值计算，不能与上一表中位数直接相减。

| Mikros 阶段 | 1 线程 | 占比 | 6 线程 | 占比 |
| --- | ---: | ---: | ---: | ---: |
| 初始挑战、row/column equality weights | 2.093 | 1.0% | 1.637 | 2.9% |
| **tensor transform + 系数聚合** | **193.242** | **93.2%** | **48.292** | **85.3%** |
| 划分 columns / quotients | 0.033 | <0.1% | 0.018 | <0.1% |
| C packing + projection | 3.073 | 1.5% | 1.383 | 2.4% |
| 吸收证书、采样 x、生成 rho | 0.097 | <0.1% | 0.105 | 0.2% |
| A/B packing，包含动态 LUT 构造 | 4.351 | 2.1% | 1.449 | 2.6% |
| B 乘 row weights | 0.695 | 0.3% | 0.311 | 0.5% |
| IP sumcheck 全部 rounds 与 folds | 3.669 | 1.8% | 3.438 | 6.1% |
| terminal claims + residual | 0.008 | <0.1% | 0.006 | <0.1% |
| **总计** | **207.260** | **100%** | **56.638** | **100%** |

| Flock 生产计时阶段 | 1 线程 | 6 线程 |
| --- | ---: | ---: |
| round1 URM | 44.546 | 9.734 |
| round2 fused fold | 5.130 | 2.088 |
| rounds 3+ tail | 7.186 | 4.912 |
| residual | 0.184 | 0.184 |
| **总计** | **57.046** | **16.918** |

Mikros 仅张量聚合一项就显著超过 Flock 整个首轮。两边阶段并非逐项相同：Flock round1 还包含 inverse-NTT table 构造、C lift/restore；Mikros C projection 单列。因此这是瓶颈位置的对照，不是同一个函数的微基准。

Flock round2 包含挑战相关折叠表构造、packed fold、第一条 message、挑战以及一条诊断输出；tail 包含后续折叠、message、最终绑定和 scratch 回收。Residual 包含初始挑战、证书吸收、C interpolation、最终组装及计时/日志开销，不能叫作纯 setup。

N=2^20 时 Mikros 聚合占比为 88.9%（1T）/56.8%（6T）：小规模多线程的调度、权重、packing 等固定开销更明显。N=2^24 时聚合从 193.24 ms 降到 48.29 ms，约 4.00×；IP tail 从 3.67 到 3.44 ms，仅约 1.07×，是聚合优化之后需处理的并行短板。

原始数据：[wsl-phases-1t.json](results/profile/wsl-phases-1t.json)、[wsl-phases-6t.json](results/profile/wsl-phases-6t.json)，逐次 Flock 日志见同名 `.log`。[summary.json](results/profile/summary.json) 将 Flock 日志关联到每条样本；[summary.csv](results/profile/summary.csv) 可直接做图。

## 3. 为什么 bit 运算和查表仍然贵

每行 128 个 bit，经 T^⊗7 变为 3^7=2187 个 bit，其中 128 个 Boolean column slots，2059 个 quotient slots。当前 [tensor.rs](src/tensor.rs) 的 lookup 实现为：

1. A/B 各做 16 次 byte LUT，得到 16 个各含 27 bit 的 u32。
2. 各做 65 次外层 u32 XOR，变成 81 个 u32；A/B 对应 word 做 AND。
3. 对每个非零 product bit 执行 `trailing_zeros`、算系数索引、读/XOR/写一个 F128 系数、清除最低 set bit，然后循环。

LUT 主要覆盖第 1 步。第 3 步的加权累加仍是逐行逐系数的完整域读改写；不能把它计作一个免费的 Boolean XOR。

对基准使用的独立均匀随机 A/B，每个变换坐标是非零线性形式，结果为 1 的概率为 1/2；两边相乘为 1 的概率为 1/4。不同坐标之间不必独立，期望计数仍成立。

N=2^24 对应 m=N/128=131072 行，因此当前实现预计进行

`m × 2187 / 4 = 71,663,616`

次 F128 accumulator 更新，其中 quotient slots 约 67,469,312 次。完整 source-level 工作量如下；这不是硬件指令计数或性能计数器测量。

| 当前 Mikros 工作 | N=2^24 次数 |
| --- | ---: |
| 固定 u32 transform LUT 读取 | 4,194,304 |
| 外层 u32 XOR | 17,039,360 |
| u32 AND | 10,616,832 |
| F128 系数更新，期望 | 71,663,616 |
| 三次 input packing 的 F128 查表/累加 | 6,291,456 |
| 三次动态 packing 表构造的 F128 XOR | 12,240 |

每个 worker 的 H 数组为 2187×16=34,992 字节。变量长度 set-bit 循环、索引计算和这个数组的读改写是需要进一步微基准的对象。当前 phase timer 将 transform 和累加一起测量，没有拆分两者，也没有采集 cache/branch miss；所以不能宣称已经证明“主要是缓存未命中”或“主要是 DRAM 带宽”。

论文 `latex/mikros/appx-tech.tex:166–218` 本身也区分扩域 XOR 与 bit XOR，并指出算术计数不包含 loads、selection masks、writes。少用扩域乘法并不自动意味着运行更快。

## 4. Flock 具体做得不同的地方

Flock 默认 skip 六维，七个 inner equality coordinates 为特殊固定常数。其生产 round1 每个 outer block 处理 8192 bit：先将八组 small-coordinate 贡献在 F8 中乘法、shift/reduce，再将十六组 medium-coordinate 贡献通过 `gamma^b * phi8(v)` 转换表合并，最后才乘随机 outer equality weights。

在 N=2^24 的 dense 路径上，源码计数为 2048 个这样的 block。AB+C 的转换累计共 4,194,304 次 F128 查表/XOR，outer-low/high weighting 共 278,528 次 F128 乘法；另外还有 16,777,216 次 F8 乘法、NTT 查表、shift/reduce、C transpose 等工作。计数不是整个 prover 的全部指令，也不能把 F8 乘法、F128 XOR、F128 乘法按同价相加。

关键差别是**以更大的块合并小表示的贡献后，再做完整域加权**。Mikros 现在每处理 128 bit 就逐项更新 H。`2187/128≈17.09` 是 Mikros 的变换膨胀率，不是应有的运行时间比；也不能简单将两边的证书系数数量之比当作复杂度比。

本机实际启用的是 SSE2 inverse-NTT LUT、无 GFNI 的 portable F8 kernel、PCLMUL F128，以及 packed fold/message 和后续 2→1 fold/message 融合。Compact-K、round2 lookahead、4→1 cascade 在这里由 aarch64+AES 条件关闭；precompute/commit overlap 和 stripe-C 也不是这个 benchmark 的入口，不能拿这些优化解释当前优势。双方复用相同的 F128 arithmetic backend。

证据位置：`crates/flock-core/src/zerocheck.rs:694–742,830–845,991–1012,1175–1196`；`zerocheck/univariate_skip_optimized.rs:349–413,888–925`；`zerocheck/univariate_skip_optimized/kernels/portable.rs:30–81`；`ntt/inv_table.rs:177–182`。

## 5. 协议代价与实现代价如何区分

| 项目 | 性质 | 本次证据/含义 |
| --- | --- | --- |
| 2187 mixed coefficients，其中 2059 quotient slots | 当前 mixed-tensor 证书结构 | 改变系数数量需要不同编码或协议论证 |
| 每个 set bit 独立更新 F128 H | 当前实现选择 | 可以按行分块、转置、查表重排，保持输出不变 |
| 两次 rho packing 重建相同表 | 当前实现选择 | A/B 可共享，规模是每次 4080 次构表 XOR |
| IP message、A fold、B fold 分别扫描 | 当前实现选择 | 融合/并行有空间，但当前并非主瓶颈 |
| Flock 七个固定 inner coordinates | Flock 协议与专用内核配合 | 不能直接替换 Mikros 的随机 row challenge |

Mikros 的 IP 初始向量长 N/128，Flock 尾部是 N/64；“Mikros 尾部数组更大”不成立。Mikros 当前 `fold_low` 分别串行折叠 A/B，解释了 tail 扩线程后改善有限，但其绝对时间比聚合小得多。

N=2^24 的 PIOP 序列化消息为 35,608 vs 2,704 bytes（13.17×）；Mikros 单次 transcript/certificate 阶段仅约 0.1 ms，因此消息大不是这次约 150 ms 单线程差距的主要原因。原测 verifier/RSS 对 Mikros 较有利，见 BENCHMARKS，但尚未计入 packed opening 与 PCS，不能据此比较完整 SNARK。

## 6. 优化优先级与可验证目标

1. **先改变 H 的加权聚合方式。** 保持协议的一种候选是每八行构造 `L[mask] = XOR of selected row weights`，将这八行的 product bits 转置成每个系数一个 byte，用一次动态表查询连续更新该系数。表占 4 KiB，依赖 row challenge，必须在线生成。它减少逐个 set-bit 更新，但增加 transpose 和构表；需与当前内核差分并实测，尚未实现或承诺加速。
2. 在同一个聚合内核上评估固定尺寸展开、跨行 SIMD、系数分块。先测 transform 与加权更新各自耗时，避免把猜测当作缓存诊断。
3. 聚合改善后，再共享 A/B packing 表、融合 packing/row weighting、融合并行 IP folds。单线程即使把当前聚合之外的全部约 6.8% 开销消掉，也只能获得约 1.07× 总加速；仅复用 packing 表无法填平约 3.7× 差距。
4. 若允许比较另一条 Mikros 路线，可另实现论文 `4-block.tex:429–523` 的 GF256 univariate tabulation（κ=128 时 255 个采样点）。这是另一种证书与协议；当前 H 一般是 F128，不能直接截为 F8。

按本次 phase 均值，若其余开销和 Flock 时间均保持不变，为追平 Flock，单线程聚合需从 193.24 ms 降到约 43.03 ms（约 4.49×），六线程需从 48.29 ms 降到约 8.57 ms（约 5.63×）。这是由计时得到的目标预算，不是已测得或保证能够达到的优化收益。

## 7. 测量限制与复现

首轮诊断把 Flock 的 stderr 直接写到 `/mnt/c`，跨文件系统日志显著干扰计时：N=2^20、1T 总中位数约 8.59 ms。已保留这批 pilot 文件 `results/profile/phases-*.{json,log}`，不用于上面的阶段表。改为先写 WSL `/tmp`、结束后复制日志，复测为 4.54 ms；无日志对照为 4.52 ms。`/tmp` 是 Linux 文件系统临时目录，并不假定它是 RAM disk。残余的 timer/logging 与调度扰动仍存在，所以正式速度比使用 control/原测数据。

本轮新增一个 profile/normal 证明、claims、transcript 一致性测试，Rust tests 总计 21 个，全部通过。主要 control + 修正后的 profile 共 96 份证明（含预热），另外保留 pilot 的 48 份；每份均完成 verifier replay、transcript 一致性和原始 A/B/C 三声明检查，输入检查在 prove 计时外。

```sh
cargo +nightly-2026-03-21 test -p mikros --release
bash mikros/run_profile.sh
python3 mikros/summarize_profile.py
```

脚本串行测量 N=2^20/2^24、1/6 线程，各预热一次后跑五次。报告保留编译器、CPU features、线程、输入摘要与全部样本；汇总脚本校验系统间/批次间输入一致、日志样本对应、阶段加和。profile API 不改变协议消息，Flock 算法文件没有修改。
