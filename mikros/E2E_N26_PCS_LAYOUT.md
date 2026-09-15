# N26 E2E：Mikros 与 FLOCK 的承诺数据和 PCS 接口

核对日期：2026-09-15。本文依据当前仓库 `latex/`、E2E fixture 和服务器 N26 三轮基线记录；仅说明数据布局和接口，不将正在测量的优化结果视为已完成结论。

**Mikros 可以与 FLOCK 承诺同样的数据；当前 E2E 已经这样实现。** 两者使用同一个 fixture 生成 witness/public 数组，同一组 FLOCK 原生 PCS 参数，并分别把所有终端声明送入 witness/public 两组 opening。三轮基线的同模式 M/F 对照中，两项 commitment 的 BLAKE3 哈希和两组 PCS 参数均相同，12 个完整证明均验证成功，且各自 prover/verifier transcript 一致。这里的相同是承诺内容和 PCS 配置相同；Mikros 与 FLOCK 的 PIOP、opening 声明、完整证明字节和 transcript 并不相同。[统一的 commit/open 调用](C:/Users/miranda/git/mikros_flock/mikros/examples/e2e_plonk_compare.rs:111)

**当前 LaTeX 承诺的是原始三条 wire，虚拟乘积不另作承诺。** single 的 witness 逻辑长度为 `3m`，其中 `m=N/128`；public 为 `(5+3h)m`。multiple 把 128 次执行的同位置 wire bits 打包，witness 逻辑长度为 `3N`，public 长度为 `N`，合计 `4N`。虚拟乘积由 Hadamard proof 归约回原始 wire 的声明，copy 的随机指纹和中间树也不需要另加源数据承诺。[single 逻辑布局](C:/Users/miranda/git/mikros_flock/latex/6-together.tex:164)、[multiple 逻辑布局及虚拟乘积](C:/Users/miranda/git/mikros_flock/latex/6-together.tex:279)、[multiple public 与总长度](C:/Users/miranda/git/mikros_flock/latex/6-together.tex:366)

本次取 `N=2^26=67,108,864`、一个 public output、`h=ceil(log2(3N+1))=28`，所以五个 selector 加三组 position digits 共 `5+3h=89` 个 bit columns。multiple 是 **128 个各有 N 门的执行**，总计 `128N=2^33` 个 binary constraints。single 的一个 F128 source word 装同一 wire 上 128 个 gate positions；multiple 的一个 F128 source word 装同一 gate position 上 128 次执行的 bits。[尺寸与 packing](C:/Users/miranda/git/mikros_flock/mikros/examples/e2e_plonk/fixture.rs:64)、[witness 生成](C:/Users/miranda/git/mikros_flock/mikros/examples/e2e_plonk/fixture.rs:154)

**实际 native 布局保留二次幂 padding，必须与论文逻辑长度分开。** witness 的三块填入四块容量，第四块为零；single public 的 89 块填入 128 块容量。multiple public 每行把 89 个有效 bit columns 放入一个 128-bit word，剩余位为零。public I/O 在终端关系中直接处理，第四个 witness 块不是额外的一条 witness，也不是虚拟乘积。[padding 和 public 生成](C:/Users/miranda/git/mikros_flock/mikros/examples/e2e_plonk/fixture.rs:154)

下表的数量均为 **F128 words**，每个 word 为 16 bytes，Mikros 与 FLOCK 共用同一张表。编码长度仅指 L0 RS codeword，不包括 Merkle tree、递归 PCS 数据或其他 scratch。

| 模式与承诺组 | LaTeX 逻辑 source words | 实际 source words | 实际 source 存储 | rate 1/2 的 L0 encoded words |
| --- | ---: | ---: | ---: | ---: |
| single witness | `3m` = 1,572,864 | `4m` = 2,097,152 | 32 MiB | 4,194,304 |
| single public | `89m` = 46,661,632 | `128m` = 67,108,864 | 1 GiB | 134,217,728 |
| multiple witness | `3N` = 201,326,592 | `4N` = 268,435,456 | 4 GiB | 536,870,912 |
| multiple public | `N` = 67,108,864 | `N` = 67,108,864 | 1 GiB | 134,217,728 |

因此 single 的逻辑 source 为 `92m`（736 MiB），native 实际为 `132m`（1,056 MiB）；multiple 的逻辑 source 为 `4N`（4 GiB），native 实际为 **`5N`（5 GiB）**。multiple L0 编码总量为 `10N` 个 F128 words（10 GiB），不是新增了 `10N` 条 witness 声明。编码扩张来自共同的 `log_inv_rate=1`。[PCS source/codeword 长度公式](C:/Users/miranda/git/mikros_flock/crates/flock-core/src/pcs/commit.rs:172)

不要把 source words、binary positions、encoded words 和 opening claims 混为一个“元素数”。实际 source word 的 bit 容量是其数量乘 128，包含固定零 padding；它不等于 circuit gate 数。L0 encoded word 是 RS 编码后的域元素；它也不等于证明传输字节。顶层 commitment 发送的是 Merkle cap 和参数，完整 proof 还包含 PCS 内部递归承诺及认证数据。[commitment 与 prover data](C:/Users/miranda/git/mikros_flock/crates/flock-core/src/pcs/commit.rs:278)

对应的 N26 native 参数如下。`m` 是该组 padded binary domain 的 `log2`，所以 source word 数为 `2^(m-7)`；它不是前文 single 的数学符号 `m=N/128`。各项均来自基线记录，M/F 完全相同；`num_lanes=null` 表示采用完整二次幂 lane 数。

| 组 | PCS 参数 `m` | `log_batch_size` | `log_inv_rate` | profile / hash |
| --- | ---: | ---: | ---: | --- |
| single witness | 28 | 4 | 1 | Fast / BLAKE3 |
| single public | 33 | 6 | 1 | Fast / BLAKE3 |
| multiple witness | 35 | 6 | 1 | Fast / BLAKE3 |
| multiple public | 33 | 6 | 1 | Fast / BLAKE3 |

**当前的差别主要是需要认证的线性查询数量和形式，而不是承诺了更多源元素。** 原始基线记录的 opening 声明如下；这些声明对原数组提出不同求值要求，不会按声明数量复制 commitment 的整个 source/codeword。

| 模式 | Mikros witness claims | FLOCK witness claims | Mikros public claims | FLOCK public claims |
| --- | ---: | ---: | ---: | ---: |
| single | 11 | 7 | 89 | 84 |
| multiple | 11 | 7 | 8 | 3 |

当前固定 AND/XOR fixture 的 FLOCK gate prover 将 selector 关系化为已知的线性关系，因此该 gate 路径不生成 public claims；Mikros 当前 PLONK reduction 保留五条 selector claims。这解释了两种模式 public claims 都相差五条，尽管两者实际承诺的 public 数组相同。[FLOCK gate 终端关系](C:/Users/miranda/git/mikros_flock/mikros/examples/e2e_plonk/gates.rs:349)

工程优化可以在保持原始声明、消息和挑战顺序的情况下合并相同行点的计算、复用 bit 转置、融合 basis 生成和折叠；这减少计算，不等于删除声明。若要把 Mikros 的声明数直接减到 FLOCK 的数量，需要另外检查 reduction 是否改变，不能作为保持证明字节不变的工程优化宣称。

**论文接口与当前测量使用的 PCS 实例也要分开。** Section 4 描述固定 basis packing 的 PCS 接口；`latex/5-pcs.tex` 经技术附录载入，并继续描述 Blaze code-switching 实例。当前 benchmark 按用户要求统一使用 **FLOCK native commit + merged_tensor/open_batch_merged + Ligerito Fast**，不应将其标为 Blaze 实验。[packed PCS 接口](C:/Users/miranda/git/mikros_flock/latex/4-block.tex:809)、[附录载入](C:/Users/miranda/git/mikros_flock/latex/appx-tech.tex:36)、[Blaze 实例说明](C:/Users/miranda/git/mikros_flock/latex/5-pcs.tex:288)、[实际 benchmark 路径](C:/Users/miranda/git/mikros_flock/mikros/examples/e2e_plonk_compare.rs:298)

基线核对证据位于 [第 1 轮](C:/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/n26-baseline/evidence/baseline-n26-r1/summary.json)、[第 2 轮](C:/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/n26-baseline/evidence/baseline-n26-r2/summary.json)、[第 3 轮](C:/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/n26-baseline/evidence/baseline-n26-r3/summary.json)。本说明核对了每轮原始 `n26-*-*.stdout.jsonl` 的 commitment 哈希、参数、claim 数以及验证状态；未据此宣称优化后的耗时或加速比。
