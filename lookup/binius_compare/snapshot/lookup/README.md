# Bitwidth-aware permutation / indexed Lasso

固定证明域 `E = GF(2^128)`（Flock 的 GHASH `F128`），实现 permutation PIOP、显式表 indexed Lasso，以及默认启用的 **WHIR PCS 完整认证组合**。所有文件和构建产物位于独立的 `lookup/` Cargo workspace，依赖现有 Flock 算术、transcript 和 Mikros binary Hadamard。

原 PIOP 对照数据见 [RESULTS.md](RESULTS.md)。WHIR 接入依据、协议顺序、限制和运行方式见 [WHIR.md](WHIR.md)。

## 已实现的两种容量回退

1. **乘积树**：逻辑位宽 `w` 嵌入实际子域 `K=GF(2^k)`，其中 `k=max(1,w).next_power_of_two()`，且 `k≤128`。叶子为 `Z+d`；第 `t` 层为首一多项式，省略公开的首项后有 `2^t` 个 K 系数。当 `2^t*k≤128` 时紧凑存储并做 K 上的系数卷积。下一层装不下时，在 transcript 的随机 `z∈E` 处求值，之后直接做 E 乘积树。这里存储的是 `K[Z]` 的系数，不把整段系数误当作一个更大的域。
2. **Lasso 元组**：总位宽 `W=address_bits+value_bits+counter_bits`。`W≤128` 时按 K 的基坐标无损拼接。`W>128` 时拆成最多 128 bit 的块，并用独立随机 `η_j∈E` 计算 `Ση_j block_j`，此时 `K=E`。所有原始/辅助源摘要固定后才抽取 `η_j`；之后独立抽取 grand-product 的 `z`。

两条路径都不实例化大于 E 的域。`Options::coefficient_capacity_bits` 只允许 `1..=128`，可强制提前切换以测试边界；它不改变证明消息。小域使用真实的乘法兼容嵌入，8 bit 复用 Flock `phi8`，其他合法子域固定生成基并缓存。任意 3 bit 值使用 GF(16) 的三维坐标子空间，不声称 GF(8) 是 GF(2^128) 的子域。

## 证明与输入连接

Permutation 不只比较两个乘积根。验证器逐层验证 degree-three GKR sumcheck `eq(r,x)·A(x)·B(x)`，再随机合并左右子声明，最终归约到两个原始 bit oracle 的线性求值。两边补齐到相同的二次幂长度，补齐值为零，shift 因子为 z；无需除法或重采样，零因子合法。空 permutation 使用一个公开零 padding 叶子。

Lasso 证明以下带地址多重集合相等：

```text
Init  = (j, table[j], 0)                 for j in [0, M)
Read  = (a[i], value[i], read_count[i])  for i in [0, N)
Write = (a[i], value[i], read_count[i]+1)
Final = (j, table[j], final_count[j])
Init ⊎ Write = Read ⊎ Final
```

`address_bits=ceil(log2 M)`，`counter_bits=ceil(log2(N+1))`；计数是无符号整数。对于每次读取，设 `h_0=1,h_c=0`，用 Mikros binary Hadamard 证明所有 `h_j·c_j=h_(j+1)`，再以 `write_j=c_j XOR h_j` 连接 Write。最高进位固定为零，排除整数 wraparound；严格递增的时间戳排除无初始化来源的循环。这不是在特征二域中把 counter 直接加 `F128::ONE`。

两棵树的叶声明拆回 value/address/read counter/carry/final counter，进位 Hadamard 的三个矩阵声明也映射回这些源。`verify` 返回 **全部 11 个**求值声明（无查询时为 8 个）；调用者必须认证所有声明和相应的 bit oracle。原始 bit 矩阵按 `bit(column * rows + row)` 存储；每个声明显式记录查询权重和偏移。

同值不同地址、重复读取、未访问表项和空查询都支持。公开表要求 `M≥1`；每个原始字段最多 128 bit，元组总宽度可以超过 128。公开形状在分配前校验，乘积叶数和计数约束数各限制为 `2^30`。

## 使用

默认启用 `whir` feature。完整验证使用 `authenticated`，verifier 不接收原始 values、addresses 或辅助 witness：

```rust
# #[cfg(feature = "whir")] {
use mikros_lookup::{authenticated, bits::PackedBits, permutation::Options};
let table = PackedBits::from_rows(&[5, 7], 3)?;
let values = PackedBits::from_rows(&[7, 5, 7], 3)?;
let addresses = PackedBits::from_rows(&[1, 0, 1], 1)?;
let (statement, proof, _stats) = authenticated::prove_lasso(
    table, &values, &addresses, Options::default(), b"my-application-v1")?;
authenticated::verify_lasso(&statement, &proof, b"my-application-v1")?;
# }
# Ok::<(), mikros_lookup::Error>(())
```

Permutation 对应 `authenticated::{prove_permutation, verify_permutation}`。验证时应使用应用预期的公共 statement 和 context；不能接受攻击者另选的 statement 作为原任务。`whir_pcs::Pcs` 另提供独立的 F128 线性声明 commit/open/verify 接口。

以下低层接口用于单独运行 PIOP（`--no-default-features` 可去掉 WHIR 依赖）：

```rust
use flock_hash::HashKind;
use flock_transcript::challenger::FsChallenger;
use mikros_lookup::{bits::PackedBits, lasso, permutation::Options};

let table = PackedBits::from_rows(&[5, 7], 3)?;
let values = PackedBits::from_rows(&[7, 5, 7], 3)?;
let addresses = PackedBits::from_rows(&[1, 0, 1], 1)?;
let statement = lasso::Statement::new(table, &values, &addresses)?;
let mut prover = FsChallenger::with_hash(b"application-v1", HashKind::Blake3);
let output = lasso::prove(&statement, &values, &addresses,
                         Options::default(), &mut prover)?;
let mut verifier = FsChallenger::with_hash(b"application-v1", HashKind::Blake3);
let claims = lasso::verify(&statement, &output.proof, &mut verifier)?;
// Test/reference oracle only. Replace with authentication of ALL claims in a PCS integration.
claims.check(&output.auxiliary.sources(&values, &addresses))?;
# Ok::<(), mikros_lookup::Error>(())
```

Permutation 使用 `permutation::{Statement::new, prove, verify}`；源认证为 `claims.check(&[(Source::Left,&left),(Source::Right,&right)])`。`Strategy::FullExtension` 是相同语句、相同 transcript、相同 verifier 的全 F128 树基线。

在 Windows PowerShell 从仓库根目录运行：

```powershell
wsl -d Ubuntu --cd /mnt/c/Users/miranda/git/mikros_flock/lookup --exec bash -lc 'CARGO_TERM_WIDTH=200 RAYON_NUM_THREADS=2 cargo +nightly-2026-03-21 test --offline -p mikros-lookup --message-format=short -j 2'
wsl -d Ubuntu --cd /mnt/c/Users/miranda/git/mikros_flock/lookup --exec bash -lc 'CARGO_TERM_WIDTH=200 RAYON_NUM_THREADS=1 cargo +nightly-2026-03-21 run --release --offline -p mikros-lookup --message-format=short -j 2 --example whir_lookup -- 64 8'
wsl -d Ubuntu --cd /mnt/c/Users/miranda/git/mikros_flock/lookup --exec bash -lc 'RAYON_NUM_THREADS=1 cargo +nightly-2026-03-21 run --release --offline -j 2 --example compare -- 10 8 64 5'
```

`compare` 的参数为 `log_n value_bits table_rows repeats`。两种实现交替运行，预热一次，逐样本检查全部源声明、transcript 同步和证明相等；输出 JSON 包括所有时间样本、消息大小和树存储字节。计时包含 prover 内部源哈希与辅助 witness 构造；输入生成在外，线性 source checker 单独列出。首次子域基生成在预热中完成，是公开、与 witness 无关的初始化。

## 验证边界与性能口径

- 低层 `permutation::verify` / `lasso::verify` 仍返回待认证声明，`Claims::check` 仍只是线性参考检查器。`authenticated` 使用真实 WHIR 承诺认证全部终端声明，并证明每个源的 Booleanity 和零 padding。WHIR 源向量目前每 bit 占一个 E 元素，最多 `2^20` 个元素；public covector 构造/最终检查仍为线性时间，不宣称完整 verifier 为对数时间或 PCS 已实现 bit packing。
- 本版没有零知识掩码或 grinding，不宣称严格 128 bit soundness。固定无损编码下，错误 permutation 的 fingerprint 概率至多约 `(L-1)/|E|`；此外还有 degree-three sumcheck、随机合并及超宽 tuple 聚合的误差。L 为补齐后的叶数。超宽聚合额外引入随机线性压缩误差，不能忽略。
- **位宽优化目前作用于树构造和保存的层；GKR sumcheck 仍在 E 中执行。** 读取紧凑层时会求值并分配临时 E 向量。因此 `compact_bytes+extension_bytes` 只是已保存树层大小，不是进程峰值 RSS。不能据此声称实现了论文全部 small-field proof 算术复杂度。
- 系数卷积采用标量参考内核，1 bit 使用 XOR carryless 特化；尚未实现更广的 SIMD、跨层融合或通用 packed polynomial Hadamard。`compare` 衡量这一版本和同语句全扩域基线，不是 Flock 完整 lookup 或单次 degree-two zerocheck 的比值。

## 依据

Mikros 构造依据本仓库 `latex/mikros/4-block.tex` 的 Packed Permutation Proof、`alg/alg-permutation.tex` 和 `appx-proof.tex` 的 permutation 分析。代码采用系数树及随机求值的思路，并用通用 GKR 连接各层；不声称逐行实现论文的全部系数层 packed reduction。

当前这份 LaTeX 正文没有 Lasso 构造。离线读写集合和计数器的组合依据作者的 [Jolt 论文附录 B.3](https://people.cs.georgetown.edu/jthaler/Jolt-paper.pdf) 及 [Lasso 原论文](https://eprint.iacr.org/2023/1216)；本目录在此基础上增加 bitwidth-aware 编码、容量回退和 binary 进位证明。
