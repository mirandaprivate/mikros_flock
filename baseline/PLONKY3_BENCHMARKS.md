# 直接调用本地 Plonky3 的 Hadamard 比较

由 summarize.py 从 results/plonky3/ 的原始 JSON 生成。本机 WSL 小规模检查，未运行服务器实验。

CPU：model name	: Intel(R) Core(TM) i7-8700K CPU @ 3.70GHz。Rust：`rustc 1.96.0-nightly (ac7f9ec7d 2026-03-20); binary: rustc; commit-hash: ac7f9ec7da74d37fd28667c86bf117a39ba5b02a; commit-date: 2026-03-20; host: x86_64-unknown-linux-gnu; release: 1.96.0-nightly; LLVM version: 22.1.0`。

代码 fingerprint：`def7b936f7424ced68bbca9c72025ede6fa92c4d34b82bc3892c58d8d60c340c`。

degree-2 关系为 AB-C；baseline 使用 cubic weighted sumcheck，Flock 使用现有 skip=6 路径。输入均为相同 packed F2 数据。没有 PCS、grinding 或同安全等级声明。

small_field_first 直接调用 ../mikros/mikros/src/baseline.rs 的 prover/verifier，使用原生 BinaryField8/BinaryField128、factored weights、fold 和 BinaryChallenger。packed bits 转字节域及 C 的原生求值计入证明时间。Dao–Thaler/Flock 使用 GHASH F128；两者与 Plonky3 的差值包含域后端、表示与 transcript 差别，不能只归因于协议。

原生 BinaryField8 Packing::WIDTH = 1。编译 flags：`-C target-cpu=native --diagnostic-width=200`。

| N | 线程 | 系统 | 样本数 | prove中位数(ms) | min–max(ms) | verify(ms) | bytes |
| ---: | ---: | --- | ---: | ---: | ---: | ---: | ---: |
| 2^13 | 1 | dao_thaler | 2 | 0.365 | 0.305–0.425 | 0.0074 | 680 |
| 2^13 | 1 | dao_thaler_generic | 2 | 0.229 | 0.220–0.238 | 0.0059 | 680 |
| 2^13 | 1 | flock | 2 | 0.382 | 0.360–0.405 | 0.1555 | 2352 |
| 2^13 | 1 | ghash_full_extension | 2 | 0.356 | 0.334–0.378 | 0.0063 | 680 |
| 2^13 | 1 | ghash_small_field_first | 2 | 0.210 | 0.210–0.210 | 0.0060 | 680 |
| 2^13 | 1 | small_field_first | 2 | 4.033 | 3.284–4.782 | 0.0563 | 792 |
| 2^13 | 1 | small_field_first_full_extension | 2 | 4.131 | 4.053–4.208 | 0.0478 | 792 |
| 2^20 | 1 | dao_thaler | 6 | 35.219 | 34.345–37.017 | 0.0092 | 1016 |
| 2^20 | 1 | flock | 6 | 4.166 | 4.069–5.318 | 0.1634 | 2576 |
| 2^20 | 1 | small_field_first | 6 | 331.027 | 324.554–378.923 | 0.0500 | 1184 |
| 2^20 | 3 | dao_thaler | 6 | 17.698 | 17.364–19.554 | 0.0089 | 1016 |
| 2^20 | 3 | flock | 6 | 2.081 | 1.941–3.958 | 0.1643 | 2576 |
| 2^20 | 3 | small_field_first | 6 | 151.747 | 145.281–152.607 | 0.0570 | 1184 |

共 63 份证明（含预热）通过 verifier、transcript 和全部源输入声明检查。汇总脚本另核对了输入一致性、跨线程及优化/对照证明摘要一致性、消息大小与统计量；没有删除慢样本。

[原始 JSON 与 CSV](results/plonky3/)；[接口、实现差别及复现命令](README.md)；[历史 GHASH 重写版结果](BENCHMARKS.md)。
