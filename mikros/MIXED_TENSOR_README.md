# Binary Mikros Hadamard proof

本 crate 实现 `A ∘ B = C` 的 binary mixed-tensor PIOP reduction，并调用仓库现有的 Flock degree-two zerocheck 做相同测量边界的比较。首版对应 [PLAN.md](PLAN.md) 的 M0–M4：包含 prover、verifier、Fiat–Shamir transcript、原始输入校验和 benchmark；M5 的 packed-opening reduction 留待后续实现。

`Kernel::Packed` 现使用 64 行 bit slicing、流式 GFNI 张量聚合、AVX-512/VPCLMUL、packing 与 IP 首消息融合，以及并行输出初始化。提供 `ProverScratch` 复用容量；协议消息与旧实现一致。实现和消融映射见 [NATIVE_OPTIMIZATION_NOTES.md](NATIVE_OPTIMIZATION_NOTES.md)。早期打包版的结果与取舍保留在 [OPTIMIZED_BENCHMARKS.md](OPTIMIZED_BENCHMARKS.md) 和 [OPTIMIZATION_NOTES.md](OPTIMIZATION_NOTES.md)。

历史 [Xeon 服务器 N=2^30 实测](server_experiments/20260912T065538-n30/REPORT.md) 使用 1、6、24 个同一 NUMA 节点的物理核，每组六次正式样本及独立 RSS，测量的是本轮 native 优化之前的 Packed。原始结果及复现方式见 [server_experiments](server_experiments/README.md)。

此前 [64 线程 native 实验报告](server_experiments/20260912T-native64/REPORT.md) 包含 GFNI/AVX-512/VPCLMUL、八变体消融及分层权重补测。N30 八次主计时中，旧版 406.56 ms、native 每次分配 139.57 ms、复用 82.03 ms、Flock 82.93 ms；后续六次复用对照中，普通复用 99.27 ms、分层复用 93.96 ms、Flock 72.69 ms。**这些历史 Flock 数据均为压缩64，与 Mikros 的压缩128不同。** 分组保留，不能据主实验的近似持平宣称稳定超过 Flock。具体瓶颈和计时边界见 [FINDINGS.md](server_experiments/20260912T-native64/FINDINGS.md)。

按 `2^t = κ = 128` 新增 `flock128`（skip7），采用 128-lane GFNI/AVX-512 首轮、16-byte 行查表、VPCLMUL 融合折叠和原 Flock 容量池。旧 `flock`（skip6）及 PCS 接口保持兼容。新的 [同压缩比方案](server_experiments/EQUAL_COMPRESSION_PLAN.md) 将主实验与 Flock64/128 参数对照分进程运行；同压缩比后，双方均为 `log_n−7` 轮尾部协议，不能再把 Mikros 尾部向量小一半当作协议优势。

[v4 同压缩比实测](server_experiments/20260912T-native64/v4/EQUAL_COMPRESSION_REPORT.md) 已完成：N30、64 线程、六次正式样本的中位数为普通 Mikros 复用 **105.27 ms**、分层复用 **89.73 ms**、Flock128 **102.14 ms**。分层版本本批中位数低 12.15%，4/6 组配对更快，但尚不足以声称稳定领先；峰值 RSS 为 **915.30 / 791.97 MiB**，Mikros 内存更高。通信、verifier 与瓶颈分析见 [同压缩比结论](server_experiments/20260912T-native64/v4/FINDINGS.md)。

`verify` 返回 A、B、C 的三个待认证求值声明。`check_claims` 使用调用方持有的原始输入逐一核验它们。此校验器适用于测试和公开输入实验，耗时随输入长度增长，**不是 PCS**；这版没有底层 commitment/opening，也不是已接入承诺的完整 SNARK。

## 协议依据和表示

实现依据当前 `latex/mikros/main.tex` 加载的源文件：

- [appx-tech.tex](../latex/mikros/appx-tech.tex) 的 **Binary Hadamard Packing in a Mixed Tensor Basis**：三项乘积恒等式、混合基系数、商系数归属和 challenge 顺序。
- [appx-proof.tex](../latex/mikros/appx-proof.tex) 的 binary Hadamard proof：最终直接产生 A、B、C 三个 matrix-evaluation claims。
- [4-block.tex](../latex/mikros/4-block.tex) 的 **Binary-field variant**；后面的 intermediate-field tabulated variant 是另一种构造，本 crate 不实现该变体。

`alg/alg-hadamard.tex` 描述单变量流程，因此这里没有照搬其额外列聚合步骤。实现保留 Boolean mixed tensor basis，不做中间域插值或 monomial 展开。

参数固定为 `F = F2`、`E = F2^128`、`kappa = 128 = 2^7`、`N = 2^log_n = 128m`。扩域运算复用 `flock_field::F128` 的 GHASH 表示，约化多项式是 `x^128 + x^7 + x^2 + x + 1`。Mikros 接受 `log_n >= 7` 且形状可表示的输入；`flock` 要求 `log_n >= 13`，`flock128` 要求 `log_n >= 14`。

每份输入恰有 `N/8` 字节。连续 16 字节组成一行；行内列 `j` 是第 `j/8` 个字节的第 `j%8` 位，位序为 LSB-first。平坦位置是 `row * 128 + column`。与字节位序分开，tensor 坐标采用 lexicographic / `HighToLow` 顺序：第一个坐标对应行或列索引的最高位。该显式行优先存储与论文的按列展开向量之间存在转置式索引置换；协议始终在同一矩阵关系上操作，两种 benchmark 系统接收完全相同的 packed bytes。

对七个列变量，基为 `chi(X) = tensor_j (1-X_j, X_j, X_j^2-X_j)`。`T = [[1,0],[0,1],[1,1]]` 将每个二元维度变成三元系数维度。prover 计算 `H = (U_A AND U_B)^T l`，其中 `U_A = A (T^tensor7)^T`，按行处理，不存储完整的变换矩阵。

`H` 共 2187 个 `F128` 元素：128 个 Boolean column claims，以及 2059 个 quotient coefficients。含数字 2 的三进制索引属于商系数，第一个 2 的位置确定 `h_j`。**序列化的 `Proof::quotients` 是递增 lexicographic ternary index 中去除 Boolean 位置后的平坦序列**，不是把七个 `h_j` 数组依次连接；verifier 按相同规则重建完整 tensor。七个商的系数数量分别为 729、486、324、216、144、96、64。

列声明和商系数先写入 transcript，随后抽取七个独立扩域 challenge。证书求值产生长度 `m` 的 inner-product claim，再运行 `log_n-7` 轮 sumcheck。

## API 和公开输入示例

| API | 返回内容 |
| --- | --- |
| `prove(a, b, c, log_n, kernel, challenger)` | `Result<(Proof, Claims), Error>` |
| `prove_with_scratch(a, b, c, log_n, kernel, challenger, scratch)` | 相同协议，复用 `ProverScratch::default()` 的分配容量；每份证明重建动态内容 |
| `verify(proof, log_n, challenger)` | 检查 PIOP 消息，返回 `Result<Claims, Error>` |
| `check_claims(a, b, c, log_n, claims)` | 对原始输入核验全部三个声明 |
| `Proof::field_elements()` | 不含序列化长度和 PCS 消息的 `F128` payload 数量 |
| `Kernel::Reference` / `Kernel::Lookup` | 同一协议的标量参考与查表实现，消息应逐项相同 |
| `Kernel::Packed` / `Kernel::PackedNative` | 当前完整 native 实现；张量 GFNI 路径有运行时检测，硬件不足时保留 portable 路径 |
| `Kernel::PackedLegacy` | 本轮优化前的 64 行算法，用于同 binary 消融 |
| `Kernel::PackedFactored` | 在 Hadamard prover 中共享块内 GFNI 权重矩阵的独立候选 |
| `Kernel::Packed8` | 保留的八行前驱，用于与 64 行粒度消融 |

`prove`/`verify` 都要求调用方在进入协议前，将可信的公开 statement digest 或 commitment roots 写入 challenger。内部 domain separation 绑定协议版本和维度，不会替调用方绑定输入。双方必须使用相同的 digest、域标签和 hash 配置。使用真实 `FsChallenger`；确定性测试 RNG 只用于生成 benchmark witness。

以下例子用于双方都持有原始输入的实验。`blake3` 是此 crate 已声明的开发依赖，外部调用方复制示例时也需要该依赖。

```rust
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros::{Error, Kernel, Proof, check_claims, prove, verify};

fn prove_and_check_public_inputs(
    a: &[u8],
    b: &[u8],
    c: &[u8],
    log_n: usize,
) -> Result<Proof, Error> {
    let mut hash = blake3::Hasher::new();
    hash.update(b"public-binary-hadamard-inputs-v1");
    hash.update(&(log_n as u64).to_le_bytes());
    for input in [a, b, c] {
        hash.update(&(input.len() as u64).to_le_bytes());
        hash.update(input);
    }
    let digest = *hash.finalize().as_bytes();
    let challenger = || {
        let mut ch = FsChallenger::with_hash(
            b"public-binary-hadamard-proof-v1",
            HashKind::Blake3,
        );
        ch.observe_bytes(&digest);
        ch
    };

    let (proof, _) = prove(a, b, c, log_n, Kernel::Packed, &mut challenger())?;
    let claims = verify(&proof, log_n, &mut challenger())?;
    check_claims(a, b, c, log_n, &claims)?;
    Ok(proof)
}
```

仅 `prove` 返回成功不代表关系成立；它不会预先扫描并拒绝错误 witness。验收需要 verifier replay 和全部终端声明认证。

`MatrixClaim` 表示 `value = row_tensor^T * matrix * column_weights`，其中 `row_factors` 存储任意两项 tensor factors。设最初行权重是 `l = ts(z)`，inner-product 最终点为 `y`，A 的行因子为 `[1-y_j, y_j]`，B 的行因子是 `[(1-z_j)(1-y_j), z_j*y_j]`。B 的两项一般不相加为 1，不能简写为 `ts(z*y)`。C 的声明认证最初的 `l^T C r`。这也是后续 packed opening 必须保留该接口的原因。

## 查表和消息大小

固定 tensor 表将一个 byte 的八位映射到 `T^tensor3` 的 27 位结果，256 个 `u32` 共 **1 KiB**，编译时生成，运行时只读。每行进行 16 次查表，再对 16 个 `u32` 执行四个外层 tensor 维度，得到 81 个各含 27 个有效位的块。这一步需要 65 次 word XOR，逐行路径不分配堆内存。并行聚合为任务保留局部 `H` accumulator，随后按域加法合并。

`lookup::pack_rows` 为传入的 128 个列权重建立十六个 byte subset-XOR 表，共 **64 KiB**，每行只需 16 次表查询。它们依赖本次权重，**每次调用重新构造**，表构造时间计入在线 prover 时间；它们不是前述固定 tensor 表。Reference/Lookup 分别绑定 C、A、B；Packed 系列共同处理 A/B，并融合 C 投影，native 还在 packing 时累计 IP 首轮消息。此实现借鉴 Flock 的 byte chunk / subset-XOR 思路，表的内容是 Mikros 自身的线性映射。

当前消息格式的精确 payload 为：

| 系统 | `F128` 元素数 | 默认 bincode 1.x 字节数 |
| --- | --- | --- |
| Mikros，固定 `kappa=128` | `2190 + 2*(log_n-7)` | `16*field_elements + 24` |
| Flock，固定 `K_SKIP=6`，关闭 grinding | `131 + 2*(log_n-6)` | `16*field_elements + 32` |
| Flock128，`K_SKIP_128=7`，关闭 grinding | `259 + 2*(log_n-7)` | `16*field_elements + 32` |

Mikros 包含一个 projected C scalar、128 个列声明、2059 个商系数、每轮两个元素和两个 terminal values；三个 `Vec` 的长度前缀产生 24 字节开销。Flock 包含两组各 `2^t` 个 first-round samples、每轮两个元素、三个 terminal values；其四个 `Vec` 包括空的 grinding nonce 列表。N30 时 Flock128 为 4880 bytes payload / 4912 bytes bincode，Mikros 为 35776 / 35800 bytes。上述不包括 caller statement、digest 或任何 PCS 消息；benchmark 同时记录逻辑 payload 和实际 bincode 大小。

## 构建与验证

本地构建和测试使用 WSL Ubuntu 下的 Rust `1.96.0-nightly`，toolchain 为 `nightly-2026-03-21`。仓库 `.cargo/config.toml` 已设置 `-C target-cpu=native`，因此测量报告记录编译启用的 CPU 特性。

以下命令在 WSL 的仓库根目录执行：

```sh
cd /mnt/c/Users/miranda/git/mikros_flock
cargo +nightly-2026-03-21 test -p mikros --release
cargo +nightly-2026-03-21 fmt -p mikros -- --check
cargo +nightly-2026-03-21 clippy -p mikros --all-targets --no-deps -- -D warnings
```

测试覆盖全部 byte LUT 输入、tensor 坐标基、商系数分组、独立多项式恒等式、reference/lookup 消息一致性、边界位、结构化 witness、错误 C、各类 proof 消息篡改、序列化、transcript replay 和终端声明的独立输入认证。Flock 的终端声明也通过独立输入求值器检查；其中 C 使用初始 equality point，不能误用 A/B 的最终 sumcheck 点。

## Benchmark CLI

`flock` 调用 `flock_core::zerocheck::{prove_packed, verify}`；同压缩比的 `flock128` 调用新增 `{prove_packed_skip7, verify_skip7}`，均证明 degree-two `a_i*b_i-c_i` 关系。skip7 仅支持 dense binary PIOP，使用独立 transcript domain，并由 verifier 固定参数；不接入旧 skip6 PCS。源输入 oracle 使用显式 `k_skip`，分别核验 8/16-byte 行与各自尾部坐标。

相同压缩比的实验使用显式参数：

```sh
cargo +nightly-2026-03-21 bench -p mikros --bench hadamard_vs_flock -- --logs 20,24 --runs 6 --threads 64 --systems reuse,factoredreuse,flock128 --output mikros/results/equal-compression.json
```

下方保留的旧示例及 benchmark 默认 `flock` 仍代表压缩64。

```sh
cargo +nightly-2026-03-21 bench -p mikros --bench hadamard_vs_flock -- --help
cargo +nightly-2026-03-21 bench -p mikros --bench hadamard_vs_flock -- --logs 13,16 --runs 3 --threads 1 --systems reference,lookup,flock --output mikros/results/smoke-1t.json
cargo +nightly-2026-03-21 bench -p mikros --bench hadamard_vs_flock -- --logs 16,20,22,24 --runs 5 --threads 1 --systems lookup,flock --output mikros/results/timing-1t.json
```

| 参数 | 含义与默认值 |
| --- | --- |
| `--logs` | 逗号分隔的 `log2(N)`，默认 `13,16,18,20,22,24` |
| `--runs` | 每个尺寸和系统的计时次数，默认 5 |
| `--threads` | 同时配置 Rayon 和 Flock all-core pool，默认 1 |
| `--systems` | 可选 `reference,lookup,packed,packed8,legacy,stream,simdv1,simd,wide,fused,native,reuse,factored,factoredreuse,flock`；默认 `reference,lookup,packed,flock` |
| `--output` | 写出 JSON；相对路径统一相对于仓库根目录；省略则输出到标准输出 |
| `--rss` | 单系统、单尺寸、独立进程的一次冷启动证明，强制 `runs=1`，不预热 |
| `--profile` | 在实际 Mikros prover 中记录九个阶段；Flock 阶段另用 `FLOCK_ZC_TIMING=1` |

普通计时为每个尺寸的每个系统先预热一次，保存预热数据，然后对每个 run 使用相同的 A/B/C 输入与 observed digest，交替并轮换系统执行顺序。每组输出 median/min/max 和所有原始样本。`reference` 用于小尺寸差分及消融，大尺寸正式比较可以选择 `lookup,flock`。

prover 时间包含权重生成、证书构造、输入绑定、所有 tail rounds、Fiat–Shamir 和函数内部的在线工作。输入生成、digest、challenger 初始化、序列化，以及原始输入的三声明检查均不计入 prover/verifier 时间；输入检查仍会执行并单独记录耗时。固定表的一次性首次使用成本由预热样本保留，当前没有独立 table-setup timer，也没有从 prover 时间中减去表初始化。

双方使用 BLAKE3 transcript 并关闭 grinding。这是同一域和明确参数下的 PIOP 性能比较，不声称两边达到了相同的 128-bit soundness。commitments、packed-opening reductions 和底层 PCS openings 均未加入本轮测量。

Linux RSS 必须分别启动新进程，例如：

```sh
cargo +nightly-2026-03-21 bench -p mikros --bench hadamard_vs_flock -- --logs 24 --threads 1 --systems lookup --rss --output mikros/results/rss-lookup-24-1t.json
cargo +nightly-2026-03-21 bench -p mikros --bench hadamard_vs_flock -- --logs 24 --threads 1 --systems flock --rss --output mikros/results/rss-flock-24-1t.json
```

RSS 样本记录证明前的 `/proc/self/status` `VmRSS` 和 prover 返回后、verify/输入检查之前的 `VmHWM`。`VmHWM` 是该进程自启动以来的最高驻留内存，包含输入、初始化、线程 runtime 和表，不是单独的 prover workspace；没有用前后差值冒充峰值内存。非 Linux 环境下该指标为空。

JSON 还记录输入 digest、执行顺序、CPU、实际编译器、启用的 target features、线程数、Git revision/dirty state 和已知性能参数。已完成的本机结果见 [BENCHMARKS.md](BENCHMARKS.md)。可运行 `bash mikros/run_comparison.sh` 串行复现，再用 `python3 mikros/summarize_results.py` 校验原始数据并生成报告。

旧版 Lookup 为什么更慢及分阶段证据见 [PERFORMANCE_ANALYSIS.md](PERFORMANCE_ANALYSIS.md)。运行 `bash mikros/run_profile.sh` 复现旧版阶段诊断及无探针对照；脚本先把诊断日志写到 WSL `/tmp`，结束后复制到工作区，避免 `/mnt/c` 的逐条日志写入干扰计时。`python3 mikros/summarize_profile.py` 将生产 Flock 日志与样本逐一关联并汇总。普通 `prove` 不调用阶段时钟；`hadamard::prove_profiled` 共用相同协议实现并返回 `ProverProfile`。

以上旧版诊断保留在原路径；优化后的正式对照使用 `bash mikros/run_optimized.sh`，串行测量 lookup/packed/Flock、六种顺序轮换、独立阶段及 RSS。结果写入 `results/optimized/`；`python3 mikros/summarize_optimized.py` 校验并生成报告。新 benchmark 额外保存计时外的 proof digest 和编译时 source fingerprint。Packed 的 `ab_pack_ms` 包含已经融合的 B weighting，不能再次把它当作独立大阶段相加。
