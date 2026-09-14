# 首版验证与对照记录

日期：2026-09-12。范围：本目录的 permutation / indexed Lasso PIOP；固定 E=F128，无 PCS、无零知识掩码、无 grinding。

后续已接入 WHIR，当前验证状态和独立端到端 smoke 数据见 [WHIR_RESULTS.md](WHIR_RESULTS.md)。下文保留首次 PIOP 测量和当时的构建状态，时间不包含新增 PCS。

## 正确性

lookup 最终实现曾在 debug 下通过 14 项检查：1 个独立树层单元测试、12 个协议集成测试、1 个 README API doctest。release 下同样的 14 项检查曾通过；之后去除了切换层的冗余存储，最终版本重新通过上述 debug 测试，并通过两组 release 对照的全部源认证。lookup 的 Clippy `--all-targets --no-deps -- -D warnings` 在当时的依赖状态下通过。依赖 flock-field 有既有的 AVX512 unused-import warning，本任务未修改它。

最终复核期间，主目录 `mikros/` 的路径依赖仍在并行修改：先触发 nightly 诊断渲染 ICE；换成 `--message-format=short` 后，编译器明确报出 `mikros/src/tensor/packed.rs` 引用的 `x86_64` 模块文件尚不存在。该模块随后落盘，再次复核停在 `mikros/src/lookup.rs:76` 的 E0004：新加的 `Kernel::PackedWide` / `Kernel::PackedFused` 尚未被 match 覆盖。错误发生在编译本目录之前，本任务没有修改主目录源码。已完成的测试/对照对应各次构建时的依赖状态，不能把它们表述为对随后所有主目录改动的验证。最后一次 Clippy 复核因此未通过，需要主任务的依赖改动完成后重跑。

覆盖范围：

- GF(2^k) 的实际嵌入与乘法/Frobenius 相容性，k=1,2,4,8,16,32,64,128；拒绝 K 容量超过 128。
- 原始 bit 数据的位宽、列序、尾部 padding 和序列化；perm 的空输入、单元素、非二次幂、重复值、错误重数和零 fingerprint 因子。
- 8 个合法容量设置以及 127/128/129 bit tuple 边界；逐层结果与独立的直接块乘积相等。
- 根、sumcheck 每类系数、终端消息、源摘要、元组拆分、进位证书和终端求值被篡改时拒绝；缺失/多余消息和缺失增量证明被拒绝。
- 重复读取、重复表值、未访问地址、空查询、错误值/地址、counter wraparound、旧计数、伪造 carry/final counter。
- 1/3 线程下相同 proof、相同终端声明和相同下一挑战；全部已测 compact/full-extension 路径的证明逐字节一致。

## 环境与方法

实机核验为 WSL Ubuntu，Intel Core i7-8700K（6 核 / 12 线程）。工具链 `nightly-2026-03-21`，实际 `rustc 1.96.0-nightly (ac7f9ec7d 2026-03-20)`，LLVM 22.1.0。release opt-level=3，继承仓库 `.cargo/config.toml` 的 `-C target-cpu=native`。

每种策略预热一次，再交替运行 5 次；`RAYON_NUM_THREADS=1`，BLAKE3。表、查询和公共 Statement 的构造在计时外；`prove` 内部的源校验/哈希、辅助 witness、tuple、树和归约都计入 prover 时间。每次验证全部源声明，并核对 prover/verifier transcript 同步以及两个策略的 proof 相等。线性 source checker 单独计时。

这是共享桌面环境的小规模可复现对照，没有进程隔离或 CPU 绑核；不能当成吞吐量/峰值 RSS 的最终实验。保存的树层字节是代码中实际缓冲区长度之和，**不含输入、临时向量或整个进程内存**。

两组对照均来自同一已编译的 `target/release/examples/compare`，SHA256 为 `245e5a9546f752d82b7f083c3656a540d4e78fa4db2308fcf9cb5609072b0aa4`。每组内部的两种策略使用完全相同的依赖二进制。后续主目录依赖变动后需要重新构建再测量，不能将新构建与本记录混合计算比值。

## 8 bit 输入：N=1024，M=64

Permutation 的 K 为 GF(2^8)，最后紧凑层 degree=16。Lasso 的总 tuple 宽度为 `6+8+11=25` bit，K 为 GF(2^32)，最后紧凑层 degree=4。

中位数，单位 ms：

| 协议 | 策略 | Prover | PIOP verifier | 线性 source checker | 两棵树保存的字节 | 序列化 proof 字节 |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| Permutation | 全 F128 | 0.2844 | 0.0408 | 0.3313 | 65,504 | 5,168 |
| Permutation | Bitwidth-aware | 0.5652 | 0.0413 | 0.3448 | 12,256 | 5,168 |
| Lasso | 全 F128 | 2.4565 | 0.4520 | 1.6043 | 131,040 | 41,721 |
| Lasso | Bitwidth-aware | 2.8304 | 0.4572 | 1.5689 | 65,504 | 41,721 |

本组保存的树层字节分别减少 81.3% 和 50.0%，prover 分别耗时约 1.99 倍和 1.15 倍。证明大小不变。当前实现仍需要标量系数卷积与读取系数层时的求值，GKR 归约继续使用 F128，因而没有得到完整 small-field prover 的算术收益。这是实现结构的解释；本次没有阶段 profiler 数据，不能进一步把时间差精确归因到某一个内核。

原始样本：[compare-1t-8bit.json](results/compare-1t-8bit.json)。复现：

```sh
RAYON_NUM_THREADS=1 cargo +nightly-2026-03-21 run --release --offline -j 2 --example compare -- 10 8 64 5
```

## 真实占用第 129 bit 的 tuple

第二组取 N=32、M=1、value_bits=123。地址宽度为零、counter_bits=6，总 tuple=129 bit；全部查询访问唯一表项，Final counter=32，其最高位恰好位于全局 bit 128，因此第二个聚合块实际非零。

结果确认：`randomized_tuple=true`，两个独立 `η_j∈F128`，`K.bits=128`，`compact_bytes=0`。两种策略都直接使用同一 E 树，proof 完全相同，序列化 Lasso proof 为 37,289 字节。该组用于检验容量回退；这两条相同代码路径的小样本时间波动不代表不同算法的收益。

原始样本：[compare-1t-129bit-tuple.json](results/compare-1t-129bit-tuple.json)。复现：

```sh
RAYON_NUM_THREADS=1 cargo +nightly-2026-03-21 run --release --offline -j 2 --example compare -- 5 123 1 5
```

## 实现边界

这份代码完成了位宽感知的树表示、两种容量回退、permutation 和 Lasso 的 PIOP 连接。系数层使用求值后的通用 degree-three GKR，尚未实现论文全部 packed coefficient reduction 的优化。下一步若要减少 prover 时间，应针对这条归约路径，而不能从已保存树层字节的减少直接推断速度提升。完整 Flock lookup 或 degree-two zerocheck 未在本目录被当成等价端到端对照。
