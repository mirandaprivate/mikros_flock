# Mikros / Flock 完整电路证明实验

本实验把同一个 SHA-256 binary tree 电路接入完整的 Flock Ligerito PCS，比较两组 PIOP。验证器只接收公共电路、公共输入/输出、公共置换、PCS 参数和证明；不会接收 witness、A/B 表或 Mikros 的离线系数缓存。所有涉及已承诺 witness 的 PIOP 终端声明都必须经 PCS 认证；涉及公开矩阵或 σ 的声明由验证器清偿，才算完整证明通过。

当前默认使用新版 packed permutation 及 GFNI 融合聚合。双方仍复用 Flock production 的 dense commitment、merged sumcheck、Frobenius assist 和一次普通 Ligerito opening，默认同步 Mikros 两棵树，并共同使用八值表 gather 审计。最新 64 线程结果见 [permutation 优化报告](results/permutation-packed-20260914/OPTIMIZED.md)。v2 的对齐 PCS 结果保留在 `results/e2e/server-pcs-aligned64-20260913/`；v1 的 full-padded functional 结果保留在 `results/e2e/server-native64-20260913/`。

## 与 Flock 类似的终端归约

`--permutation packed|legacy|both` 选择 permutation，默认 `packed`；`both` 同时比较 Flock、旧 Mikros 和新版 Mikros。新版要求 `--reduction lockstep`，也是默认值。`--reduction legacy|cube|lockstep` 保留原 gather 消融；使用前两种时需显式指定 `--permutation legacy`。这些模式使用相同 commitment、PCS 参数与 opening 内核。Boolean 保持 Mikros A/B 两路 Laurent IP 与 Flock 合并 lincheck；A 的行权重为 χ，B 为 ℓχ，不能直接当作同一个查询删去一路。常量 pin 和 C 查询仍完整认证。下表中的 legacy/cube 行指旧 permutation 的 gather 消融；新版与旧版 lockstep 的终端声明数相同。

| 模式 | Flock PCS 输入 | Mikros PCS 输入 |
|---|---:|---:|
| legacy：逐 slot gather、Mikros 顺序两树 | 11 | 20 |
| cube：两系统都压缩每次 gather | 4 | 6 |
| lockstep：再同步 Mikros 两树 | 4 | 5 |

当前单类型 SHA schema 的八个 I/O word 地址是 `[0,1,4,5,6,7,2,3]`，完整覆盖 witness 的前三个 word-column 坐标。Cube 入口从公开地址逐项验证这个布局；其他布局明确拒绝。Prover 仍计算八个原始值；verifier 保留原端点与公开输入的加权重组检查，绑定全部八值和原始查询后，以显式两位 PoW 采三个新坐标 u。将八值按实际 word 地址重排后，其在 u 的 MLE 由一条现有 `TensorClaim` 认证，行因子为原 instance 因子、新三维 u、其余高位固定零，bit 列权重保持原值。

错误八值表对应一个次数至多三的非零多线性差多项式，每张表在交互代数分析中的新错误项至多 `3/|F128|`。两位 PoW 是这一局部事件的工作因子策略，不构成整体严格 128-bit 的新证明。Mikros permutation 的自动 grinding wrapper 在 cube 前结束，显式 cube nonce 由真实 `FsChallenger` 生成和验证，两系统一致。

Lockstep 每轮先绑定左右两份 cubic 消息，再取共同挑战；每层先绑定四个子节点值，再取共同 pairing 挑战。两条等式分别检查，未用相消替代验证。共同叶点上的两条原始 source 声明分别扣除公开 id/σ 标签，检查点和值相同后才共用一次 gather。两树仍各自计算，复用 compact/GFNI/AVX-512 内核；同步保留两套折叠 scratch，峰值内存可能增加，不能据声明数量推算耗时或内存降幅。

旧库级顺序证明 API 保持原行为；完整 E2E v3 使用新 domain 并绑定 mode，故 v3 legacy 对照不与 v2 证明字节相等。消融测量比较同版本同配置的实际在线开销，不能称为固定 transcript 的纯内核消融。

## 两类比较必须分开

`examples/e2e_bit_compare.rs` 是控制变量的主要比较：

| 项目 | 共同设置 |
|---|---|
| 电路 | 相同 SHA-256 tree、相同公共叶消息与根、相同 gate witness |
| 布线 | 每个 128-bit IO word 展开为 128 条 bit copy constraints |
| Hadamard 压缩 | κ = 128；Flock 使用 skip7 |
| witness commitment | 同一份 `UnionInstance::compact_witness` 输出，production integer-lane commitment |
| PCS | 相同 Fast 参数、`open_batch_merged`、Frobenius assist、单个 EqPoint 的 Ligerito opening |
| 并行 | 两个 PIOP 分支在同一线程池并行，之后统一打开全部声明 |

Mikros 分支使用 binary univariate Hadamard、Mikros packed-IP 线性检查和 Mikros binary permutation。Flock 分支使用现有 skip7 zerocheck、可配置 skip7 的 standalone lincheck 和 Product GKR。Flock 的原 production 默认没有被更改。

`examples/e2e_flock_baseline.rs` 是第三项 production 参照：它调用现有 `prove_fast_ligerito_union_circuit` 和完整 `verify_ligerito_union_circuit`，保留 dense-stack compaction、integer lanes、word-level wiring、production Boolean/lincheck 和 merged opening。

v2 会核对共同 bit 实验与这项 production 参照的完整 PCS 参数、嵌入配置摘要、query/grinding 列表及 commitment 字节摘要相等。布线粒度、PIOP、声明数量和旧接口内部的 assembly/compaction 计时仍有区别，因此 production 参照继续单列；主要受控比较是两套逐 bit PIOP 通过同一 PCS 路径证明。

## 电路与声明地址

共同 fixture 位于 `examples/e2e_common/mod.rs`。`--log-leaves k` 生成 `2^k` 个公共叶消息和 `2^(k+1)-1` 个 SHA-256 compression gates。每个内部 gate 的消息是两个子 gate 的 256-bit 输出拼接；IV、叶消息和根通过实际 copy constraints 绑定。根还在生成 fixture 时与 native SHA-256 计算对应。

行容量取 `2^nu`，其中 `nu = max(k+1, 7)`。多余行的 witness 与 constant pin 都为零。最小 padded witness 是 m22，与已注册的 PCS 配置范围一致。日志分别记录叶数量、实际 compression 数、padded witness 的 bit-log 和 wiring 的 bit-log；这些数不能混称同一个 N。

`TreeVerifier` 是公共视图，包含固定 R1CS/registry/circuit、公共 words、行容量和实际行数。它不包含中间 compression witness。

bit wiring 使用地址

```text
x = 128 * cell_word + bit
sigma_bit[x] = 128 * sigma_word[cell_word] + bit
```

一个 bit-MLE 点的低七位选择 word 内 bit，随后是 batch 行坐标，最后是 cell-slot 坐标。`bit_wiring.rs` 将任意 tensor functional 按 gate slot 分解成主 Z 上的 `TensorClaim`；public slot 的贡献由验证器从公共 words 直接计算。Mikros 在 legacy/cube 模式的两个输入端点分别产生 gather；lockstep 模式在完整验证两个来源后仅产生一次。Flock 的两个端点共享点，除 gather 重组检查外还检查 `f_eval == g_eval`。

所有 gather claim 都位于完整 padded Z 的虚拟地址空间，由共同 merged transport 使用 verifier 从公开 union 派生的 heights/unrank 映射到 compacted dense stack；禁止直接把 padded 地址当成 dense 地址。

## 复用 production PCS 的输入适配

`crates/flock-core/src/pcs/merged_tensor.rs` 是双方共用的输入适配器，终端 `TensorClaim` 仍由各自 PIOP 验证器计算得到。普通行权重 `[a,b]` 在 `s=a+b` 非零时写成 `s*[1+r,r]`，其中 `r=b/s`，所有 s 的乘积吸收到完整 128 列权重中。Flock 的旧 verifier 入口只接 skip64×额外一位的列权重，新增 weighted128 入口仅泛化这一标量检查，复用相同 merged verifier 主体；原 production API 保持原行为。

零和权重 `[a,a]` 不做除零或丢弃。若有 d 个这样的非零维度，把这些维度同时设为普通 MLE 点 t，得到次数至多 d 的多项式；使用 d+1 个固定不同节点的最高次系数插值恢复原 functional。每个分量都经同一 PCS 验证，分量值之和必须等于原始声明。零 functional 仅在原声明值等于零时省略。适配、分量计算及 transcript 绑定全部计入 online。

后续直接调用 production `open_batch_merged`；对应 verifier 使用 weighted128 入口。保留相同 RS 内核、dense merged sumcheck、multipoint Frobenius assist、VirtualEqBasis/JIT/lookahead 和一次 `open_batch_mixed_ligerito_seeded`。不同协议的终端声明数量可以不同，不能靠删掉声明或填补重复声明强行做成相同工作量。

## κ128 与常量列

`flock_boolean.rs` 显式调用 Flock 的 skip7 zerocheck，以及 `lincheck::prove_padded_with_grinding` / `verify_with_grinding` 的 `k_skip=7` 参数。其 row/column 坐标按 SHA witness 的 BatchMajor 布局转换；原 union lincheck 保持 skip6。

standalone lincheck 的内置 constant pin 假设所有行都是一，与本实验的零 padding 不一致。因此适配器只关闭该内置假设，保留原 CSC 矩阵运算，再增加一条独立的 constant-pin PCS 查询：真实行必须为一，padding 行必须为零。Mikros 的线性检查同样生成该约束。丢弃这条查询会留下 all-zero witness 漏洞，禁止仅核验 Hadamard 和布线后提前返回成功。

## 离线与在线计时

共同 bit 实验在证明计时前准备：

- 固定电路、CSC 矩阵、σ 及其公开 field tags；
- 相同 witness 的生成、bit packing 和 Flock 的 0/1 field 表示；
- Mikros 挑战无关的 tuple 编码和 compact tree coefficient layers；
- production dense compaction，以及每次证明所需的 owned q 拷贝。

这些时间和部分缓存容量单独记入 `offline_*` 字段，不计入 `timings.prover_ms`。

在线总时间从调用完整 prover 前开始，直到函数返回后停止，包括 commitment、statement/transcript 绑定、挑战相关的转换和表构造、Boolean 与 wiring PIOP、所有 gather、通用 PCS opening，以及函数内临时数据的归还/释放。证明字节序列化和验证时间另计。静态准备没有通过从融合阶段时间中估算扣除。

`boolean_ms` 与 `wiring_ms` 是两个并行分支各自经过的时间，不能相加当作总 PIOP 时间；使用 `parallel_piops_ms` 或完整 `prover_ms`。各阶段还可能包含线程调度等待。

production 示例的 `prove_with_commit_ms` 是现有完整接口的实际返回时间，包含内部的 union assembly 和 compaction。即使外部 witness generation 已移至离线，也不能把这个字段称为已经剔除全部离线转换的纯 online 结果。共同逐 bit 实验则已经将相同 compaction 例程明确移到计时前，并单独记录耗时。

每份证明，包括 warmup，都完整验证并核对下一条 transcript challenge。同一固定 witness 的重复证明还检查 proof digest 一致；跨系统及 production 参照检查 commitment 摘要一致。正式统计仅使用 `record == "sample"`、`warmup == false`、`verified == true`、`transcript_match == true` 的样本。保存原始 JSONL 后再计算中位数和波动范围；不同线程数或不同版本的结果不直接混合。

`/usr/bin/time -v` 记录的是整个进程的资源峰值。共同 `--system both` 进程同时保留两套 offline 表，不能据此给出某一个系统的独立峰值内存。

## 安全与完成范围

PCS 保留已注册的 Fast 配置及其 transport/query grinding。共同 Boolean 分支使用显式 8-bit PoW challenge 适配器；Flock permutation 使用原 Fast 的 degree-dependent grinding，Mikros permutation 使用依赖 bit 数量的初始过滤和 cubic-tail 过滤。日志记录具体策略；这些工程设置不构成新的“整体严格 128-bit”安全定理。

当前是带真实 commitment 和完整 PCS 验证的非交互电路 argument。没有增加 zero-knowledge masking，也没有把新的 verifier 编译进 Flock 的递归 R1CS tower。正常验证还会清偿公开矩阵/σ 声明，不能额外宣称 verifier 对任意电路是 polylog 成本。

## 本地验证与运行

仓库配置使用 `-C target-cpu=native`。编译功能随运行机器变化；GFNI/AVX-512 是否实际启用以程序 metadata 和硬件日志为准。

```sh
cargo +nightly-2026-03-21 build -p mikros --release --locked --offline \
  --example e2e_bit_compare --example e2e_flock_baseline

cargo +nightly-2026-03-21 test -p mikros --release --locked --offline \
  --example e2e_bit_compare -- --test-threads=1
cargo +nightly-2026-03-21 test -p mikros --release --locked --offline \
  --test packed_ip_roundtrip
cargo +nightly-2026-03-21 test -p mikros --release --locked --offline \
  --test flock_skip7_interpolation
cargo +nightly-2026-03-21 test -p mikros-permutation --release --locked --offline
cargo +nightly-2026-03-21 test -p flock-core --release --locked --offline \
  pcs::functional::tests -- --include-ignored
cargo +nightly-2026-03-21 test -p flock-core --release --locked --offline \
  pcs::merged_tensor::tests -- --include-ignored
cargo +nightly-2026-03-21 test -p flock-core --release --locked --offline \
  weighted -- --include-ignored

target/release/examples/e2e_bit_compare \
  --log-leaves 3 --threads 4 --reps 3 --warmups 1 --system both --tamper

target/release/examples/e2e_flock_baseline \
  --log-leaves 3 --threads 4 --reps 3 --warmups 1 --profile fast
```

以上示例在 Linux/WSL 运行。负面检查会篡改公共输出、PIOP/PCS 消息和 grinding 数据；源 witness oracle 只用于专门的单元测试，不是 benchmark 的 verifier 替代品。

## 服务器快照与结果

`server_experiments/package_e2e.py` 从当前工作区的明确源码范围生成 archive 和逐文件 SHA256 manifest；包含已跟踪的 Flock crate 文件以及新增 functional/skip128 内核、Mikros 模块、测试、examples 和构建文件。不会把 target、Git history 或其他实验目录作为输入。

```sh
python3 mikros/server_experiments/package_e2e.py <new-local-snapshot-directory>

# 在已上传、校验并解包的独立服务器源码目录中运行：
python3 mikros/server_experiments/run_packed_permutation_64.py \
  --log-n 30 --log-leaves 14 --reps 8 --e2e-reps 6 --logs results
```

driver 显式选择 `--permutation both`，将证明进程固定在 64 个 logical CPUs：`0-47,48-55,72-79`，并让 Rayon 与 all-core pool 都使用 64 线程。它保存编译/测试输出、硬件记录、完整原始样本及状态文件，并在运行前后核验源码 manifest。旧 `run_e2e_64.py` 显式使用 legacy permutation，仅用于历史 gather 消融。命令、时间边界、错误与未完成阶段以实际日志为准；只启动进程或只完成 PIOP 都不算实验完成。

用于正式对外结论前，应核对上传 archive、运行 executable 和下载证据的摘要，确认服务器没有干扰性并发任务，并保留全部正式样本。结果尚未完成时不得在本文补写推断的服务器数值。
