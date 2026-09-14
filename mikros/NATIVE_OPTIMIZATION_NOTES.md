# Binary Mikros 的 native 工程实现

本文对应本轮新增的流式张量、GFNI、AVX-512/VPCLMUL、并行输出初始化与容量复用实现。它说明各路径实际执行什么，不提供未测量的性能结论。`factored` 保留为独立候选；是否将它选为默认路径，应依据同版本服务器实验决定。本文不声称已经穷尽所有可能的 binary 优化。

## 协议与比较范围

这些路径保持 binary Mikros 的协议、证明字段和挑战顺序不变。固定参数仍为 `kappa=128`、`s=7`，张量输出为 `3^7=2187` 个 F128 系数，其中 128 个 Boolean 位置成为 column claims，另外 2059 个成为 quotient coefficients。系数维持原来的三进制字典序，输入行与行挑战均使用 HighToLow 顺序。

Prover 仍先观察协议标签、尺寸和 kappa，采样行/列挑战，构造并观察 C 投影与完整 certificate，再采样 packing 挑战。IP 每轮仍观察两个域元素后才采样折叠挑战，最后观察两条终端值。融合计算只改变消息值的计算时机，不提前采样或观察挑战。Verifier 与三条 deferred input claims 的含义不变。

Flock 对照继续调用生产 `zerocheck::prove_packed`，复用同仓库 F128 与 transcript 实现。GFNI 在 Mikros 中承担任意二进制线性映射；没有将随机权重替换成固定值或小域元素。测量范围仍是 PIOP reduction，不包含 PCS commitment/opening 或 packed-opening reduction，也不声称两套协议具有相同的 128-bit soundness。

源码：[hadamard.rs](C:/Users/miranda/git/mikros_flock/mikros/src/hadamard.rs)、[tensor.rs](C:/Users/miranda/git/mikros_flock/mikros/src/tensor.rs)、[flock_baseline.rs](C:/Users/miranda/git/mikros_flock/mikros/src/flock_baseline.rs)。

## 变体与 API 映射

下表是当前 benchmark CLI 到库内核的映射。同一 CLI 名称在不同开发版本中的具体实现，应同时结合记录里的源码 fingerprint 判断。

| CLI 名称 | `Kernel` | 张量聚合 | packing / IP / 行权重 | 跨证明容量 |
| --- | --- | --- | --- | --- |
| `reference` | `Reference` | 独立标量递归 oracle | 原始路径 | 不保留 |
| `lookup` | `Lookup` | 每行 byte LUT | 原始路径 | 不保留 |
| `packed8` | `Packed8` | 8 行打包 | 旧 fused binding 与旧 fused IP | 不保留 |
| `legacy` | `PackedLegacy` | 原 64 行完整变换与 subset LUT | 旧 fused binding 与旧 fused IP | 不保留 |
| `stream` | `PackedStream` | 27 系数 leaf，立即消费 | 与 `legacy` 相同 | 不保留 |
| `simdv1` | `PackedSimdV1` | GFNI，8 系数组与水平 XOR 归并 | 与 `legacy` 相同 | 不保留 |
| `simd` | `PackedSimd` | GFNI，243 系数 leaf 与 64 系数批处理 | 与 `legacy` 相同 | 不保留 |
| `wide` | `PackedWide` | 与 `simd` 相同 | native IP；packing、C、行权重仍为旧路径 | 不保留 |
| `fused` | `PackedFused` | 与 `simd` 相同 | packing/IP 首消息融合、native C 与 IP；旧行权重 | 不保留 |
| `native` | `PackedNative` | 与 `simd` 相同 | `fused` 加 blocked 行权重 | 不保留 |
| `reuse` | `PackedNative` | 与 `native` 相同 | 与 `native` 相同 | 独立 scratch |
| `factored` | `PackedFactored` | 内层权重矩阵共享，外层 F128 缩放 | 其余与 `native` 相同 | 不保留 |
| `factoredreuse` | `PackedFactored` | 与 `factored` 相同 | 与 `factored` 相同 | 另一份独立 scratch |
| `packed` | `Packed` | 当前选择的 64 行 native 路径 | 当前完整 native 路径 | 不保留 |
| `flock` | 不适用 | Flock degree-two zerocheck | Flock 生产实现 | Flock 自身的 pool |

`legacy → stream → simd → wide → fused → native` 是累计工程消融链；`simdv1` 是保留的早期候选，`factored` 是权重分层的独立分支。`native → reuse` 和 `factored → factoredreuse` 分别比较容量生命周期。`wide` 的 native IP 同时包含向量运算、融合折叠和输出初始化方式，因此不能把总差值解释为单独一条 VPCLMUL 指令的收益。

`tensor::aggregate` 接受任意完整行权重。在这个通用入口中，`PackedFactored` 使用普通 native 聚合；实际 Hadamard prover 会通过 `tensor::aggregate_factored` 传入原始行挑战，才能使用经过数学约束的权重分层。这避免从任意 F128 权重中猜测张量积结构。

源码：[内核枚举和派发](C:/Users/miranda/git/mikros_flock/mikros/src/tensor.rs)、[prover 集成](C:/Users/miranda/git/mikros_flock/mikros/src/hadamard.rs)、[benchmark 变体与元数据](C:/Users/miranda/git/mikros_flock/mikros/benches/hadamard_vs_flock.rs)。

## 张量变换立即消费

每 64 行输入转置为 128 个 `u64`：每个字对应一个输入列，字内 64 个 bit 对应 64 行。原 `legacy` 对 A/B 分别生成 2187 个 `u64`，然后逐项 AND、查表并累加 H。

Portable `stream` 保留张量递归共享的 low/high/sum 子问题，但在 8 输入列的 leaf 只生成 27 个 A 系数和 27 个 B 系数。立即计算 AND 并消费后继续下一 leaf，不再保存两个完整的 2187 元素中间数组。每 leaf 还提供 5 个零 mask，供向量消费者使用完整批次；portable 消费者只处理 27 个真实系数。

随机行权重仍在每个 64 行块中生成八张 256 项 F128 subset-XOR 表。构表属于本次证明的在线计算；固定的二进制变换表和排列索引则与挑战无关。

源码：[packed.rs 的 `stream_128`、`stream_eight` 和 `aggregate_chunk64_stream`](C:/Users/miranda/git/mikros_flock/mikros/src/tensor/packed.rs)。

## GFNI：任意权重的线性映射

对于一个 8 行 stripe，某个系数的 mask 为 `m`，需要计算 `Σ m_i w_i`。将输出 F128 分成 16 个字节后，每个输出字节都是从 8 个 mask bit 到 8 个输出 bit 的二进制线性映射，可以编码为 GFNI 的 8×8 affine matrix，使用零 affine constant 执行。

每个 64 行块有 8 个 stripe，每个 stripe 对应 16 个输出字节，因此矩阵表示为 `[[u64; 8]; 16]`，共 1 KiB。它表达与八张 subset 表相同的任意 F128 权重映射，不依赖权重属于 GF(256)。这些矩阵每份证明按实际权重重建。

`simdv1` 每次处理 8 个系数，将 stripe 放在向量的不同 qword 中，执行 GFNI 后水平 XOR 八个 stripe，得到八个输出系数字节。该路径保留用于比较早期向量布局。

`simd` 使用更宽的消费批次：

1. 将 128 输入列递归分解为九个 32 列 leaf；每个 leaf 生成 243 个系数。
2. leaf 的 AND masks 补零到 256，分成四组 64 个系数。
3. `mask_byte_planes` 将一组 64 个 `u64` 转为八个 byte plane，每个 plane 包含 64 个系数在一个 stripe 中的 mask。
4. 每个矩阵广播到对应 stripe 的 64 个输入字节；GFNI 后在向量内 XOR 合并各 stripe，直接更新 64 个系数的输出 byte plane。

这种布局避免为每 8 个系数反复做水平 qword 归并。完整 64 行块的矩阵构造也使用硬件转置；不足 64 行时回退到标量矩阵构造与 portable 转置，缺失行视为零。

累加器使用 16 个输出 byte plane，plane stride 为 `2187` 向上对齐到 64 的 `2240`。leaf padding 只产生零贡献，允许向后覆盖到下一 leaf 或最后 padding 区的 XOR 操作。任务结束时按小端顺序恢复 F128 的 `lo`、`hi` 两个字，不改变域表示。

源码：[packed/x86_64.rs 的 `affine_matrices`、`affine_matrices_hardware`、`consume`、`consume64`、`stream_wide_leaf`](C:/Users/miranda/git/mikros_flock/mikros/src/tensor/packed/x86_64.rs)。

## packing、IP 首消息与 C 投影

挑战 `rho` 仍通过 16 张 byte subset-XOR 表绑定每个 128-bit 输入行。A/B 共享同一份本次挑战的 binding table。新 packing 每次生成相邻的最多 8 行，立即应用 B 的行权重，并累计 IP 首轮消息：

\[
a_t^*=A_t\rho,\qquad b_t^*=l_t(B_t\rho),
\]

\[
g(0)=\sum_i a^*_{2i}b^*_{2i},\qquad
g_\infty=\sum_i(a^*_{2i}+a^*_{2i+1})(b^*_{2i}+b^*_{2i+1}).
\]

随后将 A/B 值写入 IP 数组，供挑战到达后折叠。消息不需要再扫描完整的两个数组。并行 chunk 从偶数行开始，不会拆开一个消息 summand；长度 1 的 IP 没有轮消息，预计算的零结果不会被观察。

C 仍边 packing 边投影，不物化 packed C 数组。native 路径每次将四个绑定值与四个行权重送入 Flock 的 `WideGhashX4`，跨多次乘积累加后再约化。F128 约化关于 XOR 线性，因此延后约化不改变结果。

源码：[lookup.rs](C:/Users/miranda/git/mikros_flock/mikros/src/lookup.rs)、[lookup/x86_64.rs](C:/Users/miranda/git/mikros_flock/mikros/src/lookup/x86_64.rs)。

## 四路 IP 折叠与下一条消息

native IP 使用仓库已有的 `ghash_mul_x4` 和 `WideGhashX4`，没有修改 Flock 的域运算。每次读取 16 个 A 输入和 16 个 B 输入，将各自相邻两项按挑战折叠为 8 项。在这些结果仍位于寄存器时，拆分为偶/奇项，累计下一轮的 `g(0)` 和 `g_infinity`，再写出折叠结果。

首消息可以由 fused packing 传入；`wide` 消融则自行扫描 packed A/B 得到它。每轮 transcript 观察和挑战采样共用原顺序。只剩两项时使用相同的一乘法插值，长度 1 直接产生终端值；非整向量尾部使用标量核。并行输出 chunk 为 4096 项，输入达到 32768 项且线程数大于 1 时才派发任务，短轮保持直接循环。

源码：[inner_product.rs](C:/Users/miranda/git/mikros_flock/mikros/src/inner_product.rs)、[inner_product/x86_64.rs](C:/Users/miranda/git/mikros_flock/mikros/src/inner_product/x86_64.rs)、[Flock 四路 GHASH 实现](C:/Users/miranda/git/mikros_flock/crates/flock-field/src/gf2_128/x86_64.rs)。

## 输出初始化与 first-touch

native packing、IP fold 和较大的 blocked row-weight 输出不再先对全长 Vec 串行写零，再由工作线程覆盖。它们清空 Vec 的逻辑长度、保留或申请容量，通过 `spare_capacity_mut` 得到 `MaybeUninit<F128>` 输出区，让实际的 packing、fold 或 scale 写入完成初始化。

各线程只写自己的输出区。完整标量尾部和 SIMD store 覆盖所有目标元素；所有 Rayon 任务成功结束后才调用 `set_len`。在此之前 panic 会使尚未提交的 Vec 保持合法空状态；F128 是 Copy，部分已写入的元素不需要析构，也不会暴露为可读取的 F128 slice。缓存再利用时同样覆盖全部本次输出。

这种布局去掉额外的全数组清零，也让处理各块的线程有机会成为首次触页者。它没有固定线程到 NUMA 节点，也没有调用 NUMA memory binding/interleave；不能据此宣称已保证页面均匀放置。旧 `legacy` packing 与 IP 实现保留原状。

源码：[inner_product.rs 的 `prepare_output` 与 `fold_and_message_native`](C:/Users/miranda/git/mikros_flock/mikros/src/inner_product.rs)、[lookup.rs 的 fused packing](C:/Users/miranda/git/mikros_flock/mikros/src/lookup.rs)、[weights.rs](C:/Users/miranda/git/mikros_flock/mikros/src/weights.rs)。

## Blocked 行权重

行挑战的 equality tensor 仍使用 `(1+r, r)`。小表生成时先计算 `high=value*r`，再用 `low=value+high`，替代对同一个 value 的两次乘法。

超过 10 个坐标时，将最后 10 个坐标组成长度 1024 的 inner 表，前缀组成 outer 表。每个 outer 值独立缩放整个 inner 表，输出顺序为 `outer_index * 1024 + inner_index`，与 HighToLow 的前缀/后缀排列相同。长输出按块并行，缩放采用四路 F128 乘法或标量回退。小因子和完整输出都由本次挑战重新计算。

源码：[weights.rs 的 `small`、`scale` 和 `fill`](C:/Users/miranda/git/mikros_flock/mikros/src/weights.rs)。

## Factored 权重候选

对行号 `t=(u,v)`，真实随机行权重满足 `l_(u,v)=L_u W_v`。因此可以计算

\[
H_j=\sum_u L_u\left(\sum_v W_v M_{u,v,j}\right),
\]

其中 `M` 是 A/B 张量变换后的二进制按位乘积。候选路径在同一份证明内为 inner 权重 `W` 预构造 GFNI 矩阵，所有 outer 块只读共享这些矩阵。每个 outer 块先完成 inner 聚合，再将 2187 个 F128 系数乘以 `L_u`，最后合并任务结果。它不做除法，因此行挑战含 0 或 1 也适用。

当前入口将 inner block 上限设为 `2^16` 行，并根据输入长度和线程数缩小它，为 outer 层保留足够任务。inner 少于 64 行或硬件不支持时回退到普通 native/portable 聚合。Hadamard 的其他阶段仍需要完整行权重，当前实现仍为 A/B packing 和 C 投影构造完整 `l`；不能把它描述为消除了完整行权重数组。

该分支节省重复矩阵构造，但增加每个 outer 块的系数缩放，并改变任务粒度与工作区生命周期。它是否更快、是否适合作为默认路径，必须由同尺寸、同线程、同版本的 `native/factored` 与 `reuse/factoredreuse` 对照决定。本文不预先选择结果。

源码：[packed.rs 的 `aggregate64_factored`](C:/Users/miranda/git/mikros_flock/mikros/src/tensor/packed.rs)、[packed/x86_64.rs 的 `MatrixCache`、`prepare_factored`、`aggregate_factored_chunk`](C:/Users/miranda/git/mikros_flock/mikros/src/tensor/packed/x86_64.rs)。

## 特征门控与回退

| 运算 | 启用条件 | 回退 |
| --- | --- | --- |
| GFNI tensor / 硬件转置 | x86_64 runtime 检查 `gfni,avx512f,avx512bw,avx512vbmi`，对应函数带相同 target features | portable streamed tensor、SWAR 转置、subset tables |
| 四路 F128 packing/IP/C/row-weight scale | x86_64 编译条件同时满足 `avx512f,avx512bw,vpclmulqdq,pclmulqdq,sse4.1` | 原 F128 运算与标量未约化乘积累加 |
| factored outer scale | 同一组四路 F128 编译条件；tensor 入口另检查 GFNI 条件 | 标量 F128 scale；不满足分层入口条件时用普通聚合 |

GFNI 的 runtime 派发与 F128 的编译期选择是两种不同机制。四路函数复用 Flock 在相同编译条件下提供的公开实现。所有向量读取/写入都限定完整批次，并保留尾部处理。所谓 portable fallback 指对应特征未启用时的构建路径，不表示用 `-C target-cpu=native` 生成的整个程序可以迁移到缺少这些指令的 CPU。

源码：[GFNI feature dispatch](C:/Users/miranda/git/mikros_flock/mikros/src/tensor/packed/x86_64.rs)、[native IP gate](C:/Users/miranda/git/mikros_flock/mikros/src/inner_product.rs)、[native lookup gate](C:/Users/miranda/git/mikros_flock/mikros/src/lookup.rs)、[构建 flags](C:/Users/miranda/git/mikros_flock/.cargo/config.toml)。

## Scratch、冷进程与测量口径

`prove` 为每次调用创建新的 `ProverScratch`；`prove_with_scratch` 允许调用方显式保留行权重、packed A/B 与两条 IP 目标数组的容量。`prove_profiled` 和 `prove_profiled_with_scratch` 使用同一 prover 路径，仅增加九个阶段计时边界。逻辑上复用的是容量，旧的 witness 或挑战计算结果不会作为下一份证明的数据来源。

binding tables、GFNI 矩阵、factored inner/outer 权重、证书和所有消息仍在每次证明中重建。Scratch 没有为这些对象提供跨证明缓存。Benchmark 的 `reuse` 与 `factoredreuse` 分别拥有独立的持久 scratch，避免候选之间共享一次预分配。

Flock 保留的是实际的跨证明内存池，不只是 Rayon/工作线程池。[scratch.rs 的静态 `POOL`](C:/Users/miranda/git/mikros_flock/crates/flock-core/src/scratch.rs:37) 用 Mutex 管理 F128 数组；[`take_f128`](C:/Users/miranda/git/mikros_flock/crates/flock-core/src/scratch.rs:68) 优先获取容量足够的数组，[`give_f128`](C:/Users/miranda/git/mikros_flock/crates/flock-core/src/scratch.rs:148) 将数组交回池中。Zerocheck 在观察终端值之后，[回收四条 tail 数组](C:/Users/miranda/git/mikros_flock/crates/flock-core/src/zerocheck.rs:1238)，其中长度已缩为 1 的两条数组仍保留较大容量，供后续阶段或下一次证明使用。

因此，`reuse` 与 Flock 更接近双方都保留容量的工程对照；`native` 的每份证明独立申请/释放工作区成本应另行报告，不能把这两组比较都描述为相同 allocation policy。两边的具体池结构、保留对象和生命周期仍有差别。Mikros 使用调用方拥有的 `ProverScratch`，不借用或共享 Flock 的全局池；每份证明依赖 witness/挑战的内容仍重算。

连续证明可以复用同一个 scratch。下面的函数使用真实 API 签名；每个 case 依次提供 A/B/C、`log_n` 和调用方计算的 statement digest，返回的 Claims 仍交给应用已有的输入认证流程。Verifier 应使用相同的应用 domain 与 statement digest：

```rust
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros::{Claims, Error, Kernel, Proof, ProverScratch, prove_with_scratch};

fn prove_batch(
    cases: &[(&[u8], &[u8], &[u8], usize, [u8; 32])],
) -> Result<Vec<(Proof, Claims)>, Error> {
    let mut scratch = ProverScratch::default();
    let mut proofs = Vec::with_capacity(cases.len());
    for &(a, b, c, log_n, statement_digest) in cases {
        let mut challenger =
            FsChallenger::with_hash(b"my-mikros-application-v1", HashKind::Blake3);
        challenger.observe_bytes(&statement_digest);
        proofs.push(prove_with_scratch(
            a, b, c, log_n, Kernel::PackedNative, &mut challenger, &mut scratch,
        )?);
    }
    Ok(proofs)
}
```

`statement_digest` 应绑定该 case 的真实 statement 或 commitment，不能为多份不同输入复用不相关的常量。相同容量可以反复使用；挑战、证明和 transcript 都是每次独立创建的。

普通计时保留预热记录，再做配对输入与轮换顺序的正式样本；`reuse` 的预热负责首次分配，正式样本利用已存在容量。配对进程内某变体的第一次预热不等于独立冷启动，因为 runtime 和其他变体可能已经运行。`--rss` 使用全新进程、单系统、单尺寸、单次冷证明；在 prove 返回后、verify 与源输入检查前读取进程 VmHWM，包含输入、线程、表和工作区。冷证明时间不并入 warm 中位数。

无探针主计时与单独 profile 分开。packing/IP 融合把首消息的工作移入 `ab_pack_ms`，所以跨变体比较单个阶段必须结合职责变化。阶段分析使用正式样本均值保留可加性；所有动态 setup 仍计入总 prove 时间，不事后扣除。跨源码版本的 pilot 用于定位问题，不与最终同版本样本混合。

源码：[public scratch API](C:/Users/miranda/git/mikros_flock/mikros/src/hadamard.rs)、[benchmark](C:/Users/miranda/git/mikros_flock/mikros/benches/hadamard_vs_flock.rs)、[服务器 driver](C:/Users/miranda/git/mikros_flock/mikros/server_experiments/run_server_64.py)、[原始记录汇总器](C:/Users/miranda/git/mikros_flock/mikros/server_experiments/summarize_native64.py)。

## 正确性检查的依据

实现中的差分测试覆盖任意 F128 权重、所有输入列/行 bit 的转置位置、GFNI affine map、27/243 leaf 的 padding、不同 chunk/线程切分、IP 向量尾部、长度 1/2、0/1 挑战、scratch 增长/缩小及部分输出写入 panic 后重试。Factored 测试另用逐行独立乘积构造完整权重，检查前缀/后缀拆分与 Boolean 挑战。

完整 prover 测试比较各内核的 Proof、Claims 和 transcript 后续挑战。Benchmark 对每份成功记录执行 verifier replay、transcript 状态比较和全部三个原始输入声明检查，再保存 bincode proof 的 BLAKE3 digest。上述检查能够验证工程路径保持结果一致；具体哪一版本、哪种硬件路径实际编译并执行通过，应以该版本服务器 tests/log 与结果 manifest 为准。

源码：[hadamard_roundtrip.rs](C:/Users/miranda/git/mikros_flock/mikros/tests/hadamard_roundtrip.rs)、[IP 测试](C:/Users/miranda/git/mikros_flock/mikros/src/inner_product.rs)、[packing 测试](C:/Users/miranda/git/mikros_flock/mikros/src/lookup.rs)、[tensor 测试](C:/Users/miranda/git/mikros_flock/mikros/src/tensor/packed.rs)、[GFNI 测试](C:/Users/miranda/git/mikros_flock/mikros/src/tensor/packed/x86_64.rs)。
