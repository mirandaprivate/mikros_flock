# Permutation 工程优化的计算顺序

本目录保留原 Mikros permutation 的证明格式、Fiat–Shamir 顺序和验证器。
完整论文中的 packed coefficient-domain reduction 不在这个候选的范围内。
性能数字以服务器实验报告和原始样本为准。

## 输入与建树

所有候选证明同一个公开关系 `f[sigma[i]] = g[i]`。基准先生成并摘要实际
`f/g/sigma`，再开始分别计时原生格式转换和 prover。

Mikros 将左侧 `(f[i], i)`、右侧 `(g[i], sigma[i])` 编码为 bit oracle。
N30 的 8-bit 值加 30-bit 位置共有 38 个逻辑位，嵌入 GF(2^64)。这不是
GF(2^38)。两边在相同随机挑战 z 上证明乘积一致。

原版计算顺序：

1. 构造 u128 行数组，再逐位写入列优先 oracle。
2. Prover 再逐位解码为行数组。
3. 标量构造单首项系数树；逐节点 Horner 求值，切换到扩域。
4. 构造扩域层；进入 GKR 时重新求值紧凑层，并复制/拆出 a、b 数组。

候选的顺序：

1. 每八行做一次 8×8 位转置，并行写入最终 oracle，省去完整 u128 中间数组。
2. 对紧凑策略，并行解码和建树；小域乘法使用相同模多项式的 PCLMUL/约简表。
3. 为固定挑战预计算 F2-linear 字节表，系数求值改为查表和 XOR。
4. 扩域层直接借用；紧凑层仍在被访问时物化成扩域数组。

FullExtension 直接解码/嵌入并构造扩域树。它省去紧凑系数树和后续紧凑层
物化；BitwidthAware 是否节省峰值内存，必须看 GKR 期间同时存活的数组，
不能仅比较保留树层的容量。

## 保留三次消息，降低内部计算成本

原版每层对 `eq(parent, x) * a(x) * b(x)` 做 sumcheck，每轮发送三个系数。
每轮先扫描 a、b、完整 equality 表计算消息，得到挑战后分别折叠三个数组。

新实现将当前 equality 因子分离。设本轮 parent 坐标是 r，之前轮次的
equality 前缀为 p，其余布尔坐标权重为 w。只累加三个二次核心量：

```
q0   = sum w * a0 * b0
q1   = sum w * a1 * b1
qinf = sum w * (a0 + a1) * (b0 + b1)
```

在特征 2 中，二次核心为
`Q(t) = q0 + (q0 + q1 + qinf)*t + qinf*t^2`。
当前三次多项式为 `p*(1+r+t)*Q(t)`，因此发送原格式的：

```
g0 = p*(1+r)*q0
g2 = p*(r*qinf + q0 + q1)
g3 = p*qinf
```

这不是把线上协议改成 degree-two sumcheck。消息和验证器保持原样；只将
内部计算重写，差分测试还检查下一次 transcript 挑战一致。

计算调度同时改变：

- equality 分成高、低两张约 sqrt(N) 的表，取消完整 equality 数组及其折叠。
- 收到本轮挑战后，每个块先折叠 a/b，再立即计算下一轮该块的消息贡献。
  合并扫描让新折叠的数据能在缓存中被使用。
- GKR 从交错子节点视图开始，省去初始 a/b 拆分副本。
- 四个工作缓冲从 Flock scratch pool 获取并复用；仅读取已经写入的前缀。
- 支持的服务器上使用四路 AVX-512/VPCLMUL，累加未约简乘积后统一约简。

## Flock 的计算顺序和仍有的区别

冻结的 Flock Product-GKR 先在扩域构造标签指纹和两棵乘积树。逐层时，用
lambda 合并左右两棵树的声明，执行一次 sumcheck，发送两个二次核心量。
折叠同样与下一轮消息融合。最终在同一 rho 返回 f、g、sigma 的声明。

本候选仍逐一执行左右两条链，每条链产生自己的最终点。N30 的实际序列化
消息大小为 Mikros 44,208 bytes、Flock 16,176 bytes。工程优化能减少扫描、
分配和算术成本，不能消除这些消息/开点差别。

反过来，冻结 Flock 的原生 f/g 和 sigma 向量均使用 F128；Mikros 用 bit
oracle 承载值和标签。Flock 还显式求 sigma 的 MLE。因此更少的线上消息并
不直接推出更少的实际时间或内存；应查看同一 CPU、输入、线程数下的数据。

## 验证和测量边界

`tests/differential.rs` 对照单独编译的冻结基线，检查全部候选和树策略的
证明字节、源声明及 transcript，并覆盖并行路径、填充和篡改拒绝。
每份 benchmark proof 都重放验证器并核验所有原始源声明，包括 Flock sigma。

测量分开记录原生输入转换、核心 prover、简短 verifier replay 和完整源
声明核验。完整源核验是读取 witness 的检查器，不是 PCS。当前不包含
承诺/opening 或 grinding，也未比较完整 SNARK 的同等安全参数。

阶段计时没有覆盖全部分配、释放和 transcript 开销；报告保留未细分时间。
服务器的多系统计时使用一次预热、六种平衡顺序和共同 scratch pool；内存
则使用每个系统单独启动的冷进程。24→48 物理核同时跨越 NUMA 节点，不能
只从这两组时间把变化归因到某一个硬件因素。

## N30 的内存存活关系

按当前代码的数组长度计算，N30/K64 有如下容量；这是静态容量分析，
实际驻留峰值以独立进程 VmHWM 为准：

| 数组 | Compact | Full | Flock |
|---|---:|---:|---:|
| 公共 f/g/sigma | 10 GiB | 10 GiB | 10 GiB |
| 原生输入（两份 38-bit oracle 或两份 F128 向量） | 9.5 GiB | 9.5 GiB | 32 GiB |
| 保留的两棵树（忽略 O(1) 根项） | 48 GiB | 64 GiB | 64 GiB |
| 最大紧凑层物化临时数组 | 16 GiB | 0 | 0 |
| 显式 F128 sigma 向量 | 0 | 0 | 16 GiB |
| sigma MLE 首轮折叠输出 | 0 | 0 | 8 GiB |
| GKR scratch 请求容量 | 16 GiB | 16 GiB | 32 GiB |

在 Compact 的最底层 GKR，保留树的 48 GiB 与 16 GiB 物化数组同时存活，
二者之和已经达到 Full 保留树的 64 GiB。这说明当前“树压缩了 16 GiB”
不能推出“峰值内存少 16 GiB”。

进一步降低 Compact 的峰值，需要将底层系数的求值与 GKR 首轮扫描直接
融合，避免物化完整 16 GiB 数组，或缩短已用完树层的存活时间。这两项
尚未实现。scratch 容量的所有页未必都被触碰，池中也可能留有其他缓冲，
所以不能简单将此表相加作为实测 RSS。
