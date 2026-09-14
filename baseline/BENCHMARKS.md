# Baseline 本机功能与小规模计时检查

> 历史结果：这里的 `small_field_first` 是旧 GHASH 重写版，现名 `ghash_small_field_first`。这里的数据不代表现在直接调用的 Plonky3 实现；新结果见 [PLONKY3_BENCHMARKS.md](PLONKY3_BENCHMARKS.md)。原始 JSON 保留不变。

由 summarize.py 从已保存的原始 JSON 生成。此处验证实现与计时入口可用；不是服务器实验，也不等于原生 Plonky3/Binius 性能。

CPU：model name	: Intel(R) Core(TM) i7-8700K CPU @ 3.70GHz。Rust：`rustc 1.96.0-nightly (ac7f9ec7d 2026-03-20); binary: rustc; commit-hash: ac7f9ec7da74d37fd28667c86bf117a39ba5b02a; commit-date: 2026-03-20; host: x86_64-unknown-linux-gnu; release: 1.96.0-nightly; LLVM version: 22.1.0`。

代码 fingerprint：`9237019fd89214be167a75ec4ce7f608d5f3d67debb39b73cd1f40194578a028`。

degree-2 关系为 AB-C；baseline 使用 cubic weighted sumcheck，Flock 使用现有 skip=6 路径。双方同为 packed F2 输入和 GHASH F128 扩域。没有 PCS、grinding 或同安全等级声明。

Dao–Thaler 的 canonical/GHASH 基转换计入在线时间。当前 baseline 物化 N/2 个 equality weights，尚未使用参考 Plonky3 adapter 的 factored-weight 工程实现，因此不能把差值归因于协议本身。

| N | 线程 | 系统 | 样本数 | prove中位数(ms) | min–max(ms) | verify(ms) | bytes |
| ---: | ---: | --- | ---: | ---: | ---: | ---: | ---: |
| 2^13 | 1 | dao_thaler | 2 | 0.336 | 0.262–0.409 | 0.0276 | 680 |
| 2^13 | 1 | dao_thaler_generic | 2 | 0.212 | 0.207–0.217 | 0.0050 | 680 |
| 2^13 | 1 | flock | 2 | 0.413 | 0.410–0.416 | 0.1531 | 2352 |
| 2^13 | 1 | small_field_first | 2 | 0.216 | 0.200–0.232 | 0.0056 | 680 |
| 2^13 | 1 | small_field_first_full_extension | 2 | 0.422 | 0.396–0.449 | 0.0091 | 680 |
| 2^20 | 1 | dao_thaler | 6 | 37.336 | 35.878–41.410 | 0.0094 | 1016 |
| 2^20 | 1 | flock | 6 | 4.630 | 4.142–5.732 | 0.1634 | 2576 |
| 2^20 | 1 | small_field_first | 6 | 29.068 | 28.641–30.436 | 0.0104 | 1016 |
| 2^20 | 3 | dao_thaler | 6 | 22.947 | 21.762–23.701 | 0.0090 | 1016 |
| 2^20 | 3 | flock | 6 | 2.384 | 2.079–2.822 | 0.1644 | 2576 |
| 2^20 | 3 | small_field_first | 6 | 18.467 | 17.232–21.667 | 0.0106 | 1016 |

共 57 份证明（含预热）通过 verifier、transcript 和全部源输入声明检查。汇总脚本另核对了输入一致性、跨线程及优化/对照证明摘要一致性、消息大小与统计量；没有删除慢样本。

[原始 JSON 与 CSV](results/)；[接口、实现差别及复现命令](README.md)。
