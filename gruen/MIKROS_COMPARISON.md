# Gruen 与 Mikros：Goldilocks / KoalaBear 扩域对照

2026-09-12。本次在 `gruen/` 中加入 KoalaBear 四次扩域和 Mikros 加权 Hadamard 归约，并对 Lagrange 插值做优化。所有方法证明同一种原始关系 `a_i*b_i-c_i=0`，使用同一份基域输入、同一扩域算术、同一规模和确定性 verifier tape。

## 比较范围

沿用现有 Gruen 实验的 **单线程、标量、无 PCS 的交互归约内核**。这里的 Mikros 是将本地 sibling `../mikros/mikros/src/hadamard.rs` 的商多项式、二次 sumcheck 和两组列一致性检查移植到 `gruen` 的共同后端；不是直接调用 sibling 的 Plonky3 SIMD/WHIR prover。两种域的底层乘法都用现有 Rust `u128 % p` 后端，没有把 FLINT 的 microbenchmark 时间接到归约时间上。

| 基域 | p | 扩域 | 存储基 | kappa | 固定 skip k |
|---|---:|---|---|---:|---:|
| Goldilocks | 18446744069414584321 | `E=F[X]/(X^2-7)` | `[1,X]` | 2 | 1 |
| KoalaBear | 2130706433 | `E=F[X]/(X^4-3)` | `[1,X^2,X,X^3]` | 4 | 2 |

KoalaBear 是 `2^31-2^24+1`，two-adicity 24；不是 Mersenne31。四次扩域用 `u^2=3,v^2=u` 表示，即 `v^4=3`，并显式记录相对自然幂基的排列。独立 schoolbook 乘法测试核对这项转换。两种 E 元素都占 16 bytes；域大小分别约 2^128 和 2^123.95，不能称为相同的严格 128-bit soundness。

四个固定实现：

1. `gruen_2eval`：原有不跳轮 Gruen 恢复，作为主要普通轮对照。
2. `gruen_skip_kappa`：原有基域 coset FFT 跳轮，固定一次合并 kappa 行；两种基域都支持对应的子群。保留 `gruen_2eval`，避免只与较慢的跳轮路径比较。
3. `mikros_lagrange_reference`：同一 Mikros 归约的通用 Lagrange/cardinal 多项式、额外节点稠密点积和全部权重打包。
4. `mikros_lagrange_optimized`：下面的插值优化。协议消息与 reference **逐域元素完全相等**，共享同一个 verifier。

## Mikros 的证明工作

将每 kappa 个连续输入视为同一 row 的 kappa 个 column；低位 Boolean 坐标对应 column，与 Gruen 的输入布局一致。令 `w_i=eq(z_hi,i)`，发送 `s_j=Σ_i w_i*a_ij*b_ij`。同时独立读取输入 C，计算并发送 `c_eval=Σ_ij eq(z_lo,j)w_i*c_ij`，检查 `Σ_j eq(z_lo,j)s_j=c_eval`。

在 `H={1,...,kappa}` 上插值 A、B；构造

```text
P(X) = Σ_i w_i A_i(X) B_i(X)
S(X) = Σ_j s_j L_j(X)
Q(X) = (P(X)-S(X)) / V_H(X),      deg Q <= kappa-2.
```

辅助消息固定后，挑战 x 给出 `rho_j=L_j(x)`。对 `A_i(x)` 和 `w_i B_i(x)` 做长度 `N/kappa` 的二次 sumcheck，每轮发送 2 个域元素。随后发送两组 terminal columns，分别检查它们以 rho 聚合后等于两个 sumcheck 终端值，再用新的 column challenge 归约回 A/B 源 oracle。

B 的源查询权重是 `eq(r_hi,i)*eq(z_hi,i)`，保留为两个 equality 因子的乘积，不能改成 `eq(r_hi*z_hi,i)`。C 的原始源求值也必须认证；prover 不用 `A*B` 替换提供的 C。当前 oracle 是独立读取 witness 的 O(N) 检查，和原 Gruen 一样不计入归约 verifier，不冒充 PCS。

Mikros 使用连续整数插值域，Gruen skip 使用基域乘法子群/coset；这是各自归约的实际选择，终端查询可以不同，但都认证回同一份原始 A/B/C。

## Lagrange 优化

- **参数准备**：复用一个消失多项式，通过 synthetic division 构造 cardinal 多项式，并批量求逆。构造从 O(kappa^3) 降为 O(kappa^2) 基域工作。
- **额外节点求值**：在连续节点上用后向有限差分求出 `kappa+1,...,2*kappa-1` 的值，移除逐 row 的稠密 Lagrange 点积，主要只需基域加减。`kappa=2` 直接使用常数商 `Σ_i w_i(a_i1-a_i0)(b_i1-b_i0)`。
- **挑战权重**：`kappa=2` 直接取 `[2-x,x-1]`；`kappa=4` 共享成对乘积，只用 6 个 E 乘法生成 4 个 cardinal 分子。一般宽度使用 prefix/suffix 产品，O(kappa) E 工作且不需要 E 求逆。节点挑战通过测试，不产生 0/0。
- **归一化打包**：由 `Σ rho_j=1`，使用 `a_0+Σ_{j>0}rho_j(a_j-a_0)`，每个 A/B block 少一次 E×F 乘法。

这些优化没有删去 C 投影、商消息、列一致性或源认证。尚未移植 sibling 的 SIMD、split-weight 表或 terminal recovery；本实验用于同后端算法/插值对照，不表示原生实现的峰值。

## 复现与验收

实现入口：`src/mikros.rs`。新 benchmark：`src/bin/gruen_mikros.rs`。验证来源与参数的本地文件哈希：`reference/mikros-source.json`；参考 sibling 仅只读访问。

已通过 33 项测试：原四域与新增 KoalaBear 的域律、FFT、商和 Gruen 归约；Mikros 商的独立系数展开/长除法、插值节点与非节点差分、全部字段篡改、独立原始输入 oracle、错误 C、空内层轮数与边界挑战。格式检查和全目标 `clippy -D warnings` 均通过。日志：[checks.log](results/2026-09-12-mikros-validation/checks.log)。

```bash
cd /mnt/c/Users/miranda/git/mikros_flock/gruen
cargo +nightly-2026-03-21 test --release --offline
cargo +nightly-2026-03-21 clippy --offline --all-targets -- -D warnings
cargo +nightly-2026-03-21 build --release --offline --bin gruen-mikros-bench
python3 scripts/run_mikros_compare.py --n 12,16,20 --reps 8 --cpu 2 --out results/NEW_RUN
```

每项预热一次，8 次正式重复让四个执行位置各出现两次。两种域、三个规模均固定 k，不扫描最优参数。所有 public setup、equality tables、C 投影、分配、helper、packing、sumcheck、folding、terminal 工作及函数退出清理都计入外层 prover wall time。输入/challenge 生成在外；每组所有 prover 测量结束后才验证 proof，避免在测量之间插入完整 witness 扫描。

输出包括 raw.csv、summary.csv、completion.json、validation.json，以及 binary/source SHA256、CPU 和进程快照。目录/CSV 拒绝覆盖历史结果。proof_element_bytes 仅为代数消息 payload，不是带 PCS 的完整证明大小。各阶段单独取中位数，其和不保证等于总时间中位数。

## 实测结果

环境：WSL Ubuntu、Intel Core i7-8700K，`taskset -c 2` 固定一个逻辑 CPU，Rust `nightly-2026-03-21`，release/thin LTO。没有控制 Windows 宿主负载、频率或 SMT 邻核；以下保留中位数、离散程度和全部原始样本，不把波动当成优化收益。

首轮按 Goldilocks、KoalaBear 顺序测试，每组 8 次。单位为 **prover 总耗时 ms，中位数**，`N=2^n`：

| 域 | n | Gruen 逐轮 | Gruen skip-kappa | Mikros reference | Mikros optimized |
|---|---:|---:|---:|---:|---:|
| Goldilocks 二次扩域 | 12 | 2.494 | 3.405 | 3.177 | 2.747 |
| Goldilocks 二次扩域 | 16 | 40.361 | 56.035 | 49.959 | 43.342 |
| Goldilocks 二次扩域 | 20 | 644.120 | 893.312 | 805.746 | 707.760 |
| KoalaBear 四次扩域 | 12 | 0.703 | 0.766 | 0.482 | 0.416 |
| KoalaBear 四次扩域 | 16 | 15.721 | 14.079 | 10.216 | 9.302 |
| KoalaBear 四次扩域 | 20 | 177.039 | 160.561 | 118.818 | 113.163 |

首轮 KoalaBear 波动较大，因此追加了 **相同二进制、相同输入、反转域顺序** 的 n=20 确认实验，每组仍为 8 次；没有重新编译或选择最优样本。确认结果如下（全部为 ms；MAD 为绝对中位差，不是置信区间）：

| 域 | 方法 | 中位数 | min–max | MAD |
|---|---|---:|---:|---:|
| Goldilocks | Gruen 逐轮 | 635.658 | 632.021–672.270 | 2.810 |
| Goldilocks | Gruen skip-kappa | 862.947 | 845.778–913.902 | 12.763 |
| Goldilocks | Mikros reference | 793.974 | 778.648–846.096 | 14.162 |
| Goldilocks | Mikros optimized | 691.983 | 685.771–696.257 | 1.689 |
| KoalaBear | Gruen 逐轮 | 169.960 | 158.858–177.404 | 5.490 |
| KoalaBear | Gruen skip-kappa | 156.079 | 152.303–160.238 | 1.946 |
| KoalaBear | Mikros reference | 109.411 | 101.848–116.866 | 2.792 |
| KoalaBear | Mikros optimized | 108.996 | 100.998–121.754 | 4.643 |

两轮 n=20 测量支持的结论：

- **Goldilocks**：优化包使 Mikros 耗时下降 **12.2% / 12.8%**；优化后比 Gruen 逐轮仍慢 **9.9% / 8.9%**，比 Gruen skip-kappa 快 **1.26 / 1.25 倍**。因此不能说 Mikros 在 Goldilocks 上击败了最佳 Gruen 对照。
- **KoalaBear**：优化后 Mikros 比 Gruen 逐轮快 **1.56 / 1.56 倍**，比 Gruen skip-kappa 快 **1.42 / 1.43 倍**。但 Mikros reference 到 optimized 的耗时下降仅 **4.8% / 0.4%**，小于/接近观测波动；本实验没有证明 KoalaBear 上存在稳定的插值优化总收益。Mikros 相对 Gruen 的优势在 reference 中已存在。
- 两种域的绝对时间只反映这里的共同标量后端。扩域次数、基域位宽、归约压缩倍数和模乘实现都会影响结果，不能由本表推断原生 Goldilocks 算术必然慢于原生 KoalaBear 算术。

## 时间具体花在哪里

以下使用确认实验 n=20 的阶段中位数，单位 ms。`helpers/Q` 对 Mikros 包含独立 C 投影、column products 和商，对 Gruen skip 包含跳轮商构造。`terminal` 是 Mikros 的两组列一致性归约。

| 域 | 方法 | setup | helpers/Q | packing | sumcheck 消息 | fold | terminal |
|---|---|---:|---:|---:|---:|---:|---:|
| Goldilocks | Gruen 逐轮 | 94.987 | 0 | 0 | 250.098 | 290.291 | 0 |
| Goldilocks | Gruen skip | 95.720 | 280.001 | 149.118 | 191.654 | 143.149 | 0 |
| Goldilocks | Mikros reference | 47.465 | 201.016 | 145.648 | 98.354 | 97.794 | 195.128 |
| Goldilocks | Mikros optimized | 47.582 | 159.689 | 96.393 | 97.485 | 96.623 | 191.361 |
| KoalaBear | Gruen 逐轮 | 25.003 | 0 | 0 | 75.214 | 67.389 | 0 |
| KoalaBear | Gruen skip | 12.651 | 64.291 | 23.729 | 30.534 | 21.246 | 0 |
| KoalaBear | Mikros reference | 6.271 | 28.533 | 18.005 | 14.122 | 13.516 | 23.200 |
| KoalaBear | Mikros optimized | 6.285 | 29.419 | 17.483 | 14.935 | 13.640 | 24.250 |

Goldilocks 的 Mikros 优化主要落在每个 block 的工作：helper 减少约 **41.3 ms**，packing 减少约 **49.3 ms**。仅一次的挑战 Lagrange 权重计算不到 **0.002 ms**，不可能解释约 102 ms 的总节省；本次是插值、商构造和归一化打包共同优化，没有把整个收益归给某一个插值公式。

与 Goldilocks Gruen 逐轮相比，Mikros 的 sumcheck 消息和 fold 合计从 **540.4 ms** 降至 **194.1 ms**，节约约 **346.3 ms**；但 helper、packing、terminal 合计新增约 **447.4 ms**，即使 setup 更少，仍未抵消新增工作。terminal 约占 Mikros 总耗时的 **28%**。这是当前移植版本的实际差距，并非 Mikros 的理论下界；目前仍计算全部 terminal columns，没有使用 sibling 的 terminal recovery。

KoalaBear 的 kappa=4 将内层输入缩至 N/4，而 Goldilocks 的 kappa=2 只缩至 N/2。KoalaBear 上 Mikros 的消息和 fold 合计约 **28.6 ms**，Gruen 逐轮约 **142.6 ms**；节省足以覆盖新增的 helper、packing 和 terminal。与 Gruen skip 相比，Mikros 的 helper 约 **29.4 ms** 对 **64.3 ms**，内层消息和 fold 约 **28.6 ms** 对 **51.8 ms**；即使计入额外 terminal，仍更快。

KoalaBear 的有限差分移除了额外节点的稠密基域乘法，但保持了 weighted products、C 和各列累加所需的扩域工作，并增加差分加减及循环。这里 helper 实测没有稳定下降，不能仅凭乘法数量减少宣称提速。进一步优化应围绕整个 helper/terminal 的数据通路测量，而不是只优化一次性 Lagrange 权重。

## 消息大小与验证范围

n=20 的代数 payload（两种 E 均按 16 bytes/元素计）：

| 域 | Gruen 逐轮 | Gruen skip-kappa | Mikros 两版本 |
|---|---:|---:|---:|
| Goldilocks | 63 E / 1008 B | 61 E / 976 B | 48 E / 768 B |
| KoalaBear | 63 E / 1008 B | 60 E / 960 B | 54 E / 864 B |

独立源 oracle、PCS、Fiat–Shamir 和序列化都不在上述 payload/归约计时内。确定性 tape 仅用于可复现实验；实际交互实现必须在对应消息固定后才给出挑战，尤其 Mikros 的最终 column aggregation challenge。已有 verifier 的时间仅代表拿到可信源求值之后的代数检查。

首轮验证了 **216** 份 proof（含预热）及 **48** 对优化前后相同消息；确认实验验证了 **72** 份 proof 及 **16** 对相同消息。合计 **288** 份 proof 验证通过、**64** 对 Mikros 消息逐元素一致。两轮二进制 SHA256 相同，各自运行前后源代码与二进制哈希也均未变化。内存记录仅是整次 benchmark 进程峰值，不能当作每种方法的峰值内存对比。

证据文件：

- 首轮：[summary.csv](results/2026-09-12-mikros-gold-koala-1t/summary.csv)、[raw.csv](results/2026-09-12-mikros-gold-koala-1t/raw.csv)、[metadata.json](results/2026-09-12-mikros-gold-koala-1t/metadata.json)、[validation.json](results/2026-09-12-mikros-gold-koala-1t/validation.json)。
- 反转域顺序确认：[summary.csv](results/2026-09-12-mikros-gold-koala-confirm-1t/summary.csv)、[raw.csv](results/2026-09-12-mikros-gold-koala-confirm-1t/raw.csv)、[metadata.json](results/2026-09-12-mikros-gold-koala-confirm-1t/metadata.json)、[validation.json](results/2026-09-12-mikros-gold-koala-confirm-1t/validation.json)。
- 编译与测试：[checks.log](results/2026-09-12-mikros-validation/checks.log)。
