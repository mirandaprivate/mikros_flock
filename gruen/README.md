# Gruen / univariate-skip zerocheck experiments

Standalone scalar Hadamard reductions over prime fields and their extensions.
The parallel Goldilocks / KoalaBear comparison is in [server64/](server64/README.md).

## 来源与实现范围

参考 [han0110/p3-playground 的 univariate_skip.rs](https://github.com/han0110/p3-playground/blob/394ea080e1ad38748d36d7ca9aea84004255e733/hyperplonk/src/sumcheck/air/univariate_skip.rs)，固定 commit `394ea080e1ad38748d36d7ca9aea84004255e733`。对应的 `regular.rs`、`sumcheck/mod.rs`、原始许可证保存在 `reference/`。这些参考文件保持原样；本 crate 是新的标量、单线程、二次约束专用实现。

输入是 `N=2^n` 行基域数据 `(a_i,b_i,c_i)`，约束为 `a_i*b_i-c_i=0`。实现参考中的零检查分支；没有 AIR 行选择器、交互/eval-check 分支、SIMD packing、并行 DFT、PCS 或完整 HyperPlonk 集成。

三个实验路径：

- `regular_3eval`：普通 equality-weighted zerocheck，每轮直接求约束在 0、1、2 的加权值。
- `gruen_2eval`：对应参考 `regular.rs` 的加权多项式恢复；直接求 1、2，利用当前 claim 恢复 0，并把当前坐标的线性 equality 因子在循环外乘回。这是较好的普通轮对照。
- `gruen_skip_k`：固定 `k=log2(kappa)`，其中 `kappa=[E:F]` 是当前挑战扩域次数；一次合并 kappa 行，再接相同的 `gruen_2eval` 普通轮。基准不扫描 k，也不按耗时或 two-adicity 缩小 k。

两条普通轮路径输出相同的压缩三次轮多项式。这里的 Gruen 普通轮恢复特指已实现的机制，不泛指所有文献中称作 Gruen 的优化。没有把二进制域 Flock 的结果混入四个奇素数域比较。

## 跳轮公式

设 `B=kappa=2^k`，H 是基域中的 B 个互异插值节点，`Psi_H(X)=product_{h in H}(X-h)`。按自然序把每 B 行插值为三个次数小于 B 的多项式 `A_t(X),B_t(X),C_t(X)`；t 是剩余 Boolean 坐标。定义

```text
G(X) = sum_t eq(z_hi,t) * (A_t(X)*B_t(X)-C_t(X))
Q(X) = G(X)/Psi_H(X),                 deg Q <= B-2.
```

有效见证保证 `G` 在 H 上为零。存在 B 阶基域乘法子群时，沿用参考代码的 radix-2 IFFT、coset shift、FFT 路径，此时 `Psi_H(X)=X^B-1`。不存在该子群时，使用 H={0,...,B-1}、S={B,...,2B-1}，预计算一般 Lagrange 求值矩阵，将三个基域列求值到 S；逐点除以 `Psi_H(s)` 后聚合，在挑战扩域插值 Q。这个一般节点路径是对参考算法的推广，未把 FFT 移到扩域。

两条路径均用 B 个商求值点，并利用二次约束的精确次数界仅发送 B-1 个系数。一般节点路径的在线矩阵运算为 O(N*B)，domain setup 采用 O(B^3) 的直接基多项式构造；全部计入 prover 时间。

收到挑战 x 后，初始 claim 为 `Psi_H(x)*Q(x)`，用 H 上的 Lagrange 权重把每 B 行折叠成扩域的一行，然后执行 n-k 轮普通 zerocheck。最后的 oracle 点包含低 k 位的单变量 Lagrange 权重与其余位的 multilinear 权重；它并不等于原始 n 变量多项式在一个普通 n 维点上的直接 opening。验证器按基域和期望 k 独立计算对应的 `Psi_H(x)`。

低坐标采用 little-endian 布局。挑战恰好落在 H 上时返回单位向量，避免 barycentric 公式出现 0/0；普通轮 z=1 时直接求 h(0)，避免恢复公式除以零。

## 四个域

| 基域 | 模数 | 基域 two-adicity | kappa | 固定跳轮 k | 插值路径 |
|---|---:|---:|---:|---:|---|
| F251 | 251 | 1 | 16 | 4 | 一般 Lagrange |
| F65521 | 65521 | 4 | 8 | 3 | radix-2 coset |
| Mersenne31 | 2147483647 | 1 | 4 | 2 | 一般 Lagrange |
| Goldilocks | 18446744069414584321 | 32 | 2 | 1 | radix-2 coset |

`kappa_skip_rounds` 从扩域 trait 的 `DEGREE` 计算 k，检查 kappa 为二的幂。要求 `N>=kappa`，不满足时明确报错，不截断为 n。每种扩域元素都是 16 bytes。two-adicity 决定是否能使用 FFT，而不再决定实验跳几轮。API 为 FFT 路径设置 k<=16 的分配上限；一般节点路径设置 B<=256 且 2B<=p，四个固定 profile 均满足条件。

扩域是递归二次扩张，基域系数分别使用 u8/u16/u32/u64 原生大小。各扩域使用静态类型表示，测试通过 Euler 判据检查不可约性：

- F251：`u0^2=-1, u1^2=1+u0, u2^2=u1, u3^2=u2`。
- F65521：`u0^2=17, u1^2=u0, u2^2=u1`。
- Mersenne31：`u0^2=-1, u1^2=2+u0`。
- Goldilocks：`u0^2=7`。

乘法采用递归 Karatsuba 与稀疏非平方元乘法，求逆采用递归范数。基域乘法使用 Rust 的 u128 模约减，没有使用各域专用的 SIMD/Montgomery/Goldilocks 快速约减。因此结果表示这套共同后端上的算法比较，不能等同于 Plonky3 的峰值性能。挑战域大小约为 2^127.54、2^128、2^124、2^128，**不等于协议已达到 128-bit soundness**；还需扣除次数/轮数等损失。

## 验证与安全边界

实现的是交互归约内核和实验 harness。`Challenges::deterministic` 使用 SplitMix64，并提前提供整条 verifier tape，以便精确复现性能；它不是密码学 RNG/Fiat–Shamir transcript。真实交互集成必须在收到每轮消息后才揭示相应挑战。

`verify` 检查形状、期望的 n/k、所有压缩轮消息和终端恒等式，接收外部认证的终端 oracle 求值。实验用 `oracle_evaluations` 全量读取原见证，独立计算混合 Lagrange/multilinear 求值。该 O(N) oracle 替代 PCS opening；不能宣称已有 succinct verifier、完整证明系统或可安全部署的非交互证明。

`proof_element_bytes` 仅为商、轮消息与三个终端值的原始域元素存储字节，不含参数元数据、序列化、承诺、opening 或 transcript。测试覆盖四个域的算术边界、扩域不可约性/域律、直接 DFT oracle、schoolbook 商多项式除法、普通轮消息等价、有效证明、错误见证、篡改消息、错误形状和端点挑战。

## 复现实验

在 WSL Ubuntu 中执行，使用本机已验证存在的工具链：

```bash
cd gruen
cargo +nightly-2026-03-21 fmt -- --check
cargo +nightly-2026-03-21 test --offline
cargo +nightly-2026-03-21 clippy --offline --all-targets -- -D warnings
cargo +nightly-2026-03-21 build --offline --release
taskset -c 2 ./target/release/gruen-bench --n 12,16,20 --reps 5 --out results/reproduction-kappa
```

测试/构建与测量分开执行。计时是单线程、逐项串行运行，每种方法预热一次，正式重复时轮转执行顺序。见证与挑战生成不计入 prover；equality tables、domain setup、商多项式、初始折叠、普通轮、后续折叠与分配计入。每次生成的 proof 都验证；独立 oracle 求值按 n/k 预先计算并单独计时，未混入 prover。

CSV 保存每次原始样本和中位数/最小值/最大值、分阶段耗时、归约验证耗时、oracle 耗时与域元素数量，并显式记录 kappa、实际 block_size 和 interpolation_domain。分阶段独立取中位数，所以相加未必严格等于总时间中位数。输出文件通过 create_new 创建，拒绝覆盖已有测量。默认结果目录相对本 crate 的编译时路径，不依赖启动 cwd。
