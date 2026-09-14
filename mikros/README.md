# Binary Mikros: univariate + GF(256)

默认实现按仓库 **`latex/` 根目录**的新稿，使用单变量证书和GF256中间域。`latex/mikros/` 是旧副本。依据为 [4-block.tex](../latex/4-block.tex) 的 Hadamard/intermediate-field 部分和 [appx-tech.tex](../latex/appx-tech.tex) 的 tabulated implementation。首次核对时六个关键文件与Overleaf副本SHA256相同；随后Overleaf于17:58仅改写4-block开头导言，协议内容不变。本实现以仓库17:36版本为准，摘要见 [provenance](univariate_latex_provenance.json)。

参数为 `F=GF(2)`、`K=GF(256)`、`E=GF(2^128)`、`κ=128`、`m=N/128`，证明关系 `A ∘ B = C`。连续16字节是一行，列j对应字节j/8的bit j%8；Flock128接收同一份packed bytes。行优先存储与稿件按列展开之间的索引置换保持显式。

## 证书与接口

固定 `H={0,...,127}`、`S={0,...,254}`，数字使用Flock的AES GF256表示。行插值 `g_A=Aψ`、`g_B=Bψ` 各至多127次，残差 `f=<l,g_A∘g_B>-g_C` 至多254次。`f=Ψh` 且 `deg h<=126`，发送128个列声明和127个**升幂单项式**商系数。

发送证书后抽取一个 `x∈E\H`，令 `ρ=ψ(x)`，运行长度m的IP。H内挑战重采样；GF256中H外的点仍合法。新标签为 `mikros-binary-hadamard-univariate-gf256-v1`，与旧mixed-tensor transcript不同。

实现停在正文允许的 **direct packed-opening** 接口。A/B列权重为Lagrange向量ρ；B行因子保留 `[(1-z_j)(1-y_j),z_j*y_j]`，不要求两项相加为1。未加入 `alg-hadamard` 额外的两组128元素列聚合及fresh MLE point。`verify` 返回A/B/C三个待认证声明；`check_claims` 用原始输入独立核验，耗时另计。当前不含PCS或完整SNARK。

## 实现路径

E2E 默认使用新版 packed permutation 和 GFNI 融合聚合；显式 `--permutation legacy`
保留旧协议，`--permutation both` 对比全部版本。使用同一 PCS 的服务器结果见
[最新优化报告](results/permutation-packed-20260914/OPTIMIZED.md)，入口说明见 [E2E.md](E2E.md)。

当前选用原来的补点方案 `Kernel::PairAggregate`；独立 Hadamard benchmark 默认对比 `pairreuse,flock128`，E2E 继续使用 `PairAggregate`。2026-09-13 的系数卷积优化已停止，仅保留为显式实验选项和历史记录；不会由默认入口选中。

在经满秩验证的K-basis `η_j=x^j (j=0..15)` 下，分解扩域权重并流式计算 `U=Lᵀ((AP)∘(BP))`。只对255个聚合结果插值，不分配扩展后的witness矩阵。

- [field.rs](src/univariate/field.rs)：真实256×256乘法表（64 KiB）、Flock嵌入、双向坐标转换表（合计128 KiB）。GHASH原始16字节不能直接当GF256坐标。
- [domain.rs](src/univariate/domain.rs)：共享byte-block插值表；native复用Flock的32 KiB压缩continuation表。商系数先在GF256坐标内查表插值，再组合成F128。
- [univariate.rs](src/univariate.rs)：Reference用扩域乘法核对聚合；Lookup使用乘法表和每行动态subset-XOR表；Native在支持硬件上用GFNI/AVX-512执行相同AES域运算，否则回退查表。
- [tiled.rs](src/univariate/tiled.rs)：Tiled缓存64行的continuation乘积，每次把4个GF256坐标的16个累加器保留在ZMM寄存器中。TiledBasis再使用批量GFNI仿射指令完成GHASH到GF256坐标的转换；数学基、证明消息与挑战顺序均与Native相同。两个新路径均按CPU能力分派，缺少GFNI/AVX-512时回退查表。
- RegisterBasis保留同一批量转换，逐项构造输出平面，避免整片预清零。RegisterPacked在此基础上用 [宽打包内核](src/lookup/x86_64.rs)，把首条IP消息的四路乘积累计到任务末尾再水平合并；原TiledBasis和原打包内核继续作为对照。运行时GFNI分派与原实现相同；宽打包由AVX-512/VPCLMUL编译特性控制，缺少这些特性时使用原打包路径。
- 首轮聚合工程候选：`SplitAggregate` 分开 H 与 continuation 的八坐标累计；`HalfAggregate` 按采样点半区处理八坐标；`PairAggregate` 固定展开 A/B 的插值查表；`AffineAggregate` 在 Split 基础上把八行 H 掩码改为 GFNI 仿射映射；`JointAggregate` 组合 Pair 与 Affine。实现见 [eight.rs](src/univariate/tiled/eight.rs) 和 [affine_h.rs](src/univariate/tiled/affine_h.rs)。五者均保留 TiledBasis 的 Batch 坐标转换和原打包/IP，仅改变首轮的执行方式；不支持 GFNI/AVX-512 的主机仍回退查表。
- [hadamard.rs](src/hadamard.rs)：所有聚合模式共同复用已优化的权重构造、AB packing、B加权/IP首消息融合、C投影、VPCLMUL IP与scratch。

固定表由OnceLock共享，首次构造计入预热/冷启动。每份证明的随机权重和动态表都在prover内重建、计时。

| API | 用途 |
| --- | --- |
| `prove(a,b,c,log_n,kernel,challenger)` | 每次创建scratch，返回 `(Proof,Claims)` |
| `prove_with_scratch(...,challenger,scratch)` | 复用 `ProverScratch` 容量 |
| `verify(proof,log_n,challenger)` | 核验PIOP，返回待认证声明 |
| `check_claims(a,b,c,log_n,claims)` | 独立原始输入核验 |
| `Kernel::{Reference,Lookup,Native,Tiled,TiledBasis,RegisterBasis}` | 同一新协议的六种首轮聚合路径；Native和TiledBasis保留原实现作为对照 |
| `Kernel::RegisterPacked` | RegisterBasis聚合，加上任务内持续保留宽累计器的AB打包 |
| `hadamard::prove_profiled[_with_scratch]` | 单独记录阶段耗时 |

调用方必须在进入协议前，把同一份可信statement digest/commitment roots写入prover和verifier的challenger。验收需要verifier replay和全部终端声明认证；仅prover返回成功不代表关系成立。

## 与Flock比较

| 系统 | F128 payload数量 | N30 payload字节 | N30 bincode字节 |
| --- | --- | --- | --- |
| 新Mikros univariate | `258+2*(log_n-7)` | 4864 | 4888 |
| Flock128，skip7 | `259+2*(log_n-7)` | 4880 | 4912 |
| 旧Mikros mixed-tensor | `2190+2*(log_n-7)` | 35776 | 35800 |

新Mikros包括1个C投影、128个列声明、127个商系数、每IP轮2个元素和2个terminal值。上述均不含PCS；双方关闭grinding，不宣称相同安全等级。

```sh
cargo +nightly-2026-03-21 test -p mikros --release --locked --offline
cargo +nightly-2026-03-21 fmt -p mikros -- --check
cargo +nightly-2026-03-21 bench -p mikros --bench univariate_vs_flock -- --logs 16,20 --runs 6 --threads 64 --systems reuse,flock128 --output mikros/results/univariate.json
```

每组预热后按相同输入配对并轮换顺序，记录样本、proof digest、源码指纹、verifier和源输入检查时间。`--profile`独立诊断；RSS使用 `--rss --runs 1`，要求单系统、单尺寸、新进程，在prover返回后且核验前读取整个进程峰值。

硬件消融使用 `--systems reuse,tiledreuse,basisreuse,flock128`，分别对应Native、Tiled、TiledBasis和Flock128。每个系统有独立scratch；`native,tiled,basis`则每次新建scratch，释放也计入prover时间。正式比较关闭阶段计时，另起进程测profile和RSS。实验驱动为 [run_hardware_64.py](server_experiments/run_hardware_64.py)，支持先在较小尺寸筛选，再以同一个冻结可执行文件测N30。

首轮聚合消融使用 `--systems basisreuse,splitreuse,halfreuse,pairreuse,affinereuse,jointreuse,flock128`。新驱动 [run_aggregate_64.py](server_experiments/run_aggregate_64.py) 默认 N26、64 线程、七系统各 14 次正式计时；加 `--profile` 单独测各阶段，加 `--rss` 分进程测峰值。无 `reuse` 后缀的 `split,half,pair,affine,joint` 每份证明新建 scratch。GFNI 仿射映射按真实动态权重构造，计时包含该准备成本。

[v8 首轮聚合实验](server_experiments/20260912T-native64/v8-aggregate/REPORT.md)：N30 同一 executable、64 线程、四系统各八次正式计时，TiledBasis / Pair / Joint / Flock128 的 prover 中位数分别为 **91.75 / 66.27 / 61.24 / 89.54 ms**。Pair 对基线快 **8/8** 次，中位数降低 **27.77%**；Joint 快 **7/8** 次，但范围 **54.43–164.52 ms**，均值 **75.65 ms** 高于 Pair 的 **64.41 ms**。因此本批更稳妥的工程选择是显式使用 `Kernel::PairAggregate` / `pairreuse`，Joint 继续作为可选组合，原默认未变。独立四次 profile 的首轮聚合均值为基线 **59.44 ms**、Pair **31.89 ms**、Joint **27.83 ms**；它们属于另一个进程，不能混入正式 prover 计时。N30 profile 的其他阶段亦有明显波动，不能从耗时推断缓存或 NUMA 原因。全部原始样本、均值、范围和汇编证据保留在报告中。

v7候选消融使用 `--systems basisreuse,registerreuse,registerpackedreuse,flock128`，对应v6基线、转换候选、转换加打包候选和Flock。新驱动 [run_register_64.py](server_experiments/run_register_64.py) 保留独立源码及结果目录，正式计时与profile均完整轮转执行位置；`register,registerpacked`是对应的新建scratch模式。证明消息和挑战顺序不变。

[v5服务器实测](server_experiments/20260912T-native64/v5-univariate/REPORT.md) 已完成：N30、64线程、真实GFNI/AVX-512/VPCLMUL，六次prover中位数Mikros **98.41 ms**、Flock128 **90.29 ms**，Mikros本批慢约9%，仅2/6组配对更快。协议verifier为 **0.038 / 0.578 ms**，独立进程峰值RSS为 **916.07 / 791.88 MiB**。GF256聚合占Mikros诊断总均值55.73%；样本波动较大，不据此声称稳定胜负。服务器67项测试、全部32份实验证明通过；源码包及23个下载证据文件摘要已验证。本地Clippy通过（依赖Flock保留原有unused-import警告）。

[v6硬件优化实测](server_experiments/20260912T-native64/v6-hardware/REPORT.md) 使用同一冻结可执行文件、N30、64线程、六组配对：Native为 **95.52 ms**，TiledBasis为 **85.43 ms**，Flock128为 **86.27 ms**。TiledBasis相对Native中位数降低 **10.57%**，配对较快4/6次；与Flock仅相差约1%，较快3/6次，不能据此认定稳定领先。独立profile中的GF256聚合均值从 **39.23 ms** 降至 **32.84 ms**；该阶段仍占新核诊断总均值36.65%，AB打包与IP合计45.93%。这组加速比只使用v6内部对照，不使用历史v5时间。

v6的72项服务器测试及128条实验证明记录全部通过；317个打包源码文件和45个下载证据文件逐项校验摘要。新旧Mikros的证明字节一致。独立冷进程峰值RSS分别为Native **921.96 MiB**、TiledBasis **923.04 MiB**、Flock **791.90 MiB**，本次未减少大数组容量。实际机器码分析见 [汇编审计](server_experiments/20260912T-native64/v6-hardware/diagnostics/local-assembly-audit.md)：分块热循环16个ZMM累加器没有spill/reload。服务器不允许读取硬件性能计数器，因此文中访存量是汇编操作数字节计数，不是DRAM或cache miss实测。

[v7继续优化实验](server_experiments/20260912T-native64/v7-register/REPORT.md) 已完成：N26四方案各12次筛选后，N30对TiledBasis、RegisterPacked、Flock各测9次。N30 prover中位数分别为 **82.33 / 83.00 / 93.19 ms**；均值分别为 **84.92 / 81.11 / 95.67 ms**。新核对基线较快6/9次，逐输入配对比值中位数0.948，但总体中位数没有降低，因此**尚未确认N30端到端稳定加速，默认保持不变**。本批两条Mikros路径的中位数均低于Flock，不能把全部差距归因于这次改动。

N26诊断中AB打包均值从1.70降至0.87 ms，N30独立诊断却为基线15.08、新核17.62 ms；不把小规模阶段收益外推到N30。汇编确认[基转换](server_experiments/20260912T-native64/v7-register/diagnostics/local-basis-assembly-audit.md)消除了目标memset及输入平面溢出，[打包](server_experiments/20260912T-native64/v7-register/diagnostics/local-packing-assembly-audit.md)将水平合并移至任务末尾，但仍有两个累计分量落栈。v7的76项服务器测试、149条证明记录及65份下载证据均已核验；318个打包源文件保持与测试快照一致。新路径作为实验选项保留。

## 历史保留

旧协议为 `mikros::mixed_tensor`，兼容入口 `mikros::legacy`，旧Kernel为 `mikros::tensor::Kernel`。`hadamard_vs_flock` 及原回归测试显式调用legacy；新协议使用独立benchmark `univariate_vs_flock`。

旧文档在 [MIXED_TENSOR_README.md](MIXED_TENSOR_README.md)，其中示例需把mikros导入改为mikros::legacy。v1–v4源码包和结果未改；[v4同压缩比报告](server_experiments/20260912T-native64/v4/EQUAL_COMPRESSION_REPORT.md) 仍描述旧混合张量协议，不能用它推断新单变量版性能。
