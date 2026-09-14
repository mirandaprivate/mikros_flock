# Mikros 与 Flock 的 permutation proof：优势从哪里来

分析对象是本仓库冻结的 Flock `product_gkr::prove_batched` 与 Mikros
permutation 实现。论文中的 packed permutation 单独讨论：当前代码尚未
实现其全部系数域 Hadamard reduction。复杂度及计时均不包含 PCS，不能
直接推导完整 SNARK 的优劣。

## 1. 相同关系，不同指纹编码

设 N=2^n，σ 是公开置换，值宽为 w，证明 f[σ(i)]=g[i]。

Mikros 将 `(value, position)` 的 w+n 个二进制坐标，确定性地注入
K=GF(2^k) 再嵌入 E=GF(2^128)。记这个注入为 e，则检查

    ∏ᵢ (z + e(fᵢ,i)) = ∏ᵢ (z + e(gᵢ,σ(i)))。

注入没有随机压缩碰撞；位置标签也包含在需要认证的位 oracle 中。
它要求 w+n≤k≤128，且这里选取能容纳宽度的二元塔子域。

Flock 对已嵌入 E 的值采用随机线性组合：

    ∏ᵢ (fᵢ + α·i + β) = ∏ᵢ (gᵢ + α·σ(i) + β)。

Flock 最终认证 f、g、sσ 三个 MLE 值；σ 的声明不能省略。对本实验
保证为置换的 σ，两侧最高齐次部分相同，错误关系的二元差多项式总次数
至多 N−1。Mikros 的一元差多项式次数也至多 N−1。因此两者的指纹
Schwartz–Zippel 上界均为 (N−1)/|E|；这里不计后续 reduction/PCS/哈希
错误，也不是 Fiat–Shamir 的完整安全证明。若不假设 σ 是置换，Flock
一般应使用至多 N 的总次数界。

N=2^30、|E|=2^128 时，这个指纹上界约为 2^-98，不能仅因使用 F128
就声称整体具有 128-bit soundness。确定性编码的好处是编码结构及无独立
压缩挑战，不是消除 N/|E| 这一项。

## 2. 实际计算顺序

Flock：随机仿射叶子 → 全扩域乘积树 → 两棵树逐层同步 reduction →
λ 合并的二次 core sumcheck → 同一行开点 ρ。

Mikros v2：位矩阵解码 → 紧凑子域叶子 → 小多项式系数乘积 → 求值并
建立扩域上层 → 左树 GKR → 右树 GKR。最底层按块临时求值，层用完即
释放，但两条 transcript 仍各自独立。

本轮 v3：在 K64、容量 128 bit 的路径，将解码、紧凑叶子写入、叶子
求值与相邻乘法合并；直接得到首个扩域层，之后保持 v2 reduction。
正确性来自 evaluation 的环同态：

    ev_z((Z+a)(Z+b)) = (z+e(a))(z+e(b))。

K64→E 的嵌入是域同态。旧路径的两个 K64 系数 `(ab,a+b)` 无需单独
形成数组；最高次项为 1。新路径仍存放紧凑 K64 叶子，保留之后的流式
GKR 输入。它以 E 乘法替代首层 K64 系数乘法，利用 SIMD 和减少内存
遍历换取速度；这是需要实测的成本交换，不是减少总算术工作的定理。

更具体地说，忽略固定大小的表预计算，v3 每个 grand product 把首层
N/2 次 K64 乘法换成 N/2 次 F128 乘法；后续 GKR 乘法不变。所以即使
v3 更快，也不能将其描述为“减少了扩域乘法次数”。它利用的是 SIMD、
较少的查表和较少的数组流量，而非稿件完整系数域方案的 EE 数量优势。

N30 时旧首层系数临时数组每棵树占 8 GiB；左右树依次构造，所以不能
把取消它们表述为同时降低 16 GiB 峰值。若峰值发生在 GKR 阶段，取消
建树临时数组可能完全不改变整个进程的峰值。

单看必需的大数组流量，每棵 N30 树的旧路径需额外读取 8 GiB 紧凑
叶子、写入 8 GiB 系数数组、再读回该 8 GiB 系数数组。融合路径省去
这 24 GiB 的读写，两棵树合计 48 GiB。这是源码层面的流量估算，
不包含缓存行写分配、清零、NUMA 或页回收，不能当作实测 DRAM 流量。

每一对 38-bit 叶子的查表工作也改变了：旧求值器处理两个完整 K64
系数，使用 16 次字节查表；新路径对两个叶子各查 5 次表，再做一次
F128 乘法。旧路径还需 K64 乘法及其 reduction 查表。两种乘法的
硬件成本不同，因此“16→10 次求值查表”也只是局部操作计数。
每个字节表包含 256 个 16-byte F128 值；该首层求值器的表分配从
64 KiB 变为 20 KiB。这个静态尺寸差异不等于缓存未命中率的实测结果。

## 3. 为什么当前 Flock 的证明更小

令 S=Σ(k=0..n−1)k=n(n−1)/2。

| 项目 | 当前 Mikros | 冻结 Flock batched |
|---|---:|---:|
| sumcheck 轮数 | 2S | S |
| 每轮发送的 E 元素 | 3 | 2 |
| 每层边界值总数 | 4 | 4 |
| 总 E 元素数 | 3n²+n+2 | n²+3n+5 |
| 最终行开点 | 两个 | 一个 |

Mikros 总数为 `2 roots + 2×(3S+2n)`。Flock 为
`2 roots + 2S+4n + 3 source evals`。实际 bincode 还包括向量长度：
Mikros 增加 16+16n 字节，Flock 增加 16+8n 字节（关闭 grinding）。
所以 n=30 时为 44,208 和 16,176 字节，当前 Mikros 大约是 2.73 倍。

“二次 core”指将已知 eq 线性因子从消息中分离；加权被加数仍涉及
eq·A·B。Mikros 的 v2/v3 内核已经利用这一分解做工程计算，却为了保持
原 transcript 仍发送三次消息。Flock 进一步只发送两个 core 值，而且
批处理左右电路。不要据“轮数减半”推断所有乘法也减半：左右两个乘积
门仍都必须计算。

## 4. Mikros 的理论潜力与当前实现的边界

本地稿件 `latex/4-block.tex` 的 packed permutation 命题，设 N=mκ，
对一个 grand product 给出的 schoolbook 成本为

    O(Nκ) BB + O(N) BE
      + [10N/κ + O(κ + log²m + log m log κ)] EE。

BB 为基域乘法，BE 为基域系数与扩域权重的运算，EE 为扩域乘法。
两个 committed grand products 要计两份成本。稿件给出的峰值内存为
O(N+κ²) 个基域坐标；当 m≥κ 时为 O(N)。这是稿件协议的成本声明，
此处复核其与代码的适用范围，未重新证明全部命题。

稿件中 κ 也是扩域次数 [E:F]，主实例要求 |F|>κ；它不只是一个可
任意调大的数组分块参数。若将带标签的元素视为 F=K64，固定 E=F128
就得到 κ=2，而且此时 BB 是 K64 乘法，不是单 bit 的 AND。改用更
小工作域、扩大扩域或采用中间域版本，会改变编码、算术成本和表空间，
应重新计数，不能直接套用 bit witness 的成本。

其优势是把一部分昂贵 EE 工作换成便宜的系数运算和 BE；保留
`O(Nκ) BB` 与 `O(N) BE` 后才能判断是否划算。不能只看 `10N/κ` 就
宣称整体 κ 倍加速。粗略成本模型 `aNκ+bN+cN/κ` 的最优 κ 约为
sqrt(c/a)，还受合法域、packing、内存和 lower-order terms 限制。
这个估算仅在 a/b/c 可以近似固定时成立；改变工作域时它们通常也会变。

当前实现将系数求值成 E 元素后，仍对全部层做普通 E-GKR，没有完整
的 packed Hadamard quotient/reduction。因此其 GKR 总体仍是 O(N)
扩域工作，不能使用上面的 10N/κ 作为当前实测程序的 EE 计数。
特别是 N30 的 8-bit 值加 30-bit 位置已占 38 bit，落入 K64；128-bit
系数容量只能容纳 degree-2 monic polynomial 的两个非首项系数，
于是当前切换深度仅一层。不能把原始值是 8 bit 等同于带位置 tuple
仍能用 K8，或声称当前获得 16 路 permutation packing。

## 5. 内存优势是已实现的结构选择，不是 Flock 的下界

在 K64 切换一层时，每棵 Mikros 树保留 N 个 8-byte 叶子和 N−1 个
16-byte 扩域上层元素，约 24N 字节。完整扩域树约 32N 字节。
这是不考虑按层释放的树数据总量，不是进程峰值 RSS。

更一般地，设实现切换前合并 B 个叶子（此处 B 是数组布局参数），若
保留 log₂B 个紧凑层、每层省略 monic 首项，单树数据分配量约为
`Nk·log₂B/8 + 32N/B` 字节。假设扩大系数容器，固定 k=64 时，B=2、4
都约为 24N 字节，B=8 则变成 28N 字节。因此仅扩大系数容器、保留
更多紧凑层，未必继续省内存。这是当前“保存各层供普通 GKR 使用”
结构的计数，不是稿件完整系数域协议的内存公式。

本实验两份 Mikros 位 oracle 合计 2(w+n)N/8 字节；N30 为 9.5 GiB。
Flock 的两份已嵌入 F128 输入合计 32 GiB。两者另外共用原始 f/g/σ
基准输入，且都将原生格式转换计入 total。Mikros 顺序处理两条链，
最大申请四份 N/4 的 F128 scratch；Flock batched 需要八份。
申请容量不等于全部驻留页，RSS 必须实测。

当前 Mikros 在进入第一条 GKR 链之前已构造好两棵树。N30 时，两树
约 48 GiB，位 oracle 约 9.5 GiB，基准公共 f/g/σ 约 10 GiB；这些
数组合计约 67.5 GiB 分配量，尚未计入线程、表和 scratch。即便省去
构造中的短期数组，也仍会到达这个两树同时存活的阶段。进一步减少
这一部分内存，需要改变树的存活顺序、重算策略或证明结构。

本次 Flock 未使用 live mask，其 dense 路径还生成 N 个 F128 的
sσ 向量，N30 时为 16 GiB。该辅助向量用于终端 σ 求值；有 mask 的
分支采用不同做法，不应把 dense 路径的内存数推广到全部 Flock 配置。

终端源认证也有不同工作：Flock 复用同一个 ρ 的权重来检查 f/g/σ，
Mikros 检查两个不同 ρ 下的位 oracle 线性组合。直接扫描输入的认证
耗时不能等同于将来 PCS 的时间；PCS 还涉及不同 oracle 布局、bit
维度归约及 batching，因此“两个声明比三个声明少”本身不足以证明
Mikros opening 更便宜。

这解释了上轮 67.49 GiB 对 158.01 GiB 的主要结构来源。Flock 也可
采用按需叶子、提前释放等手段；不能将此实现差距当作不可消除的协议
内存下界。相反，将 Mikros 改为左右同步 batched 后，也需要重新检查
scratch 与树层同时存活的内存成本。

## 6. 优势判断与下一项协议优化

- 当前 Mikros 的实证优势首先是内存。是否更快要看本轮同机配对实验，
  源认证与 PCS 成本必须另列，不能由 core 时间推断端到端证明优势。
- 当前 Flock 在 proof bytes、交互轮次和共同 opening point 上占优；
  Mikros 较小的输入表示并不自动使其总证明更小。
- Mikros 的长远优势应来自完整系数域 reduction，使更多工作停留在
  小域。只优化通用 GKR 常数项，无法兑现论文所述 EE 降低。
- 可移植的协议改动是左右 λ-batching 和二次 core 消息；它们与紧凑
  叶子兼容。若保留四个每层边界值和两个 roots，新的 batched Mikros
  主体可为 n²+3n+2 个 E 元素。此为消息结构推导，尚未实现，需新
  transcript 域分离、批处理 soundness 分析和新 verifier，不能作为
  本轮优化结果或完整 PCS 大小预测。

## 来源

本地源码是本次实验口径的主要依据：

- `../streaming/src/permutation.rs`、`src/fast_tree.rs`、`src/fast_sumcheck.rs`、`src/sumcheck.rs`。
- `../snapshot/crates/flock-core/src/product_gkr.rs`：`prove_batched`、`BatchedLayerProof`、`ProductGkrBatchedProof`、`ProductGkrBatchedClaim`。
- `../../../latex/4-block.tex` 的 packed permutation 命题及 `../../../latex/appx-proof.tex` 的对应证明。

公开资料用于核对背景：[Flock 原论文](https://arxiv.org/html/2607.27491v1)
第 1、4 节以 batch R1CS 的 zerocheck/lincheck 为主，批处理可以避免
通用 permutation 开销。本报告比较的是本地 `product_gkr` 组件，不应
把原论文的 hash throughput 视为本 permutation 基准成绩。
