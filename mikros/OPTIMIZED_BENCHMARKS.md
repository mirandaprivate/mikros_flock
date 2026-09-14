# Mikros 打包工程优化与 Flock 对比

本报告由 summarize_optimized.py 从原始 JSON 生成。每份证明已在 benchmark 内完成 verifier、transcript 和全部源输入声明核验；脚本进一步核对统计量、输入摘要及 Mikros 各内核的序列化证明摘要完全一致。

## 测量范围

- CPU：Intel(R) Core(TM) i7-8700K CPU @ 3.70GHz；linux x86_64；指令集：aes, pclmulqdq, avx2。
- Rust：rustc 1.96.0-nightly (ac7f9ec7d 2026-03-20); binary: rustc; commit-hash: ac7f9ec7da74d37fd28667c86bf117a39ba5b02a; commit-date: 2026-03-20; host: x86_64-unknown-linux-gnu; release: 1.96.0-nightly; LLVM version: 22.1.0；同一 executable、F128 backend、BLAKE3 transcript 和统一线程配置。
- lookup 为保留的旧逐行 LUT；packed 为多行打包聚合、融合输入绑定和融合 IP tail；Flock 调用原生产 prove_packed，算法未改。
- 主要曲线每系统预热一次，六次正式样本；三系统循环六种执行排列，均衡先后位置。正式计时关闭 FLOCK_ZC_TIMING。
- 同一 N 和 run 使用完全相同的 packed A/B/C。所有 witness 变换、动态构表、分配、权重、折叠、transcript 和声明构造计入在线 prover 时间；不在计时外缓存 witness 或 challenge-dependent 表。
- 不计入输入生成/digest、challenger 初始化、序列化与源输入 oracle；双方均不含 packed-opening reduction、PCS commitment/opening。grinding 均关闭，不声明相同的 128-bit soundness。
- 两个 Mikros 内核的协议参数、proof bytes 和 transcript 不变。这使优化前后比较具有相同协议语义；与 Flock 的比较仍保留不同协议和参数的区别。

## 在线 prover 中位数

单位 ms。旧/新 > 1 为工程优化加速；新/Flock > 1 表示优化后的 Mikros 仍慢。

| 线程 | N | 旧 lookup | 新 packed | Flock | 旧/新 | 新/Flock |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 2^16 | 1.201 | 0.262 | 0.830 | 4.59× | 0.32× |
| 1 | 2^18 | 3.588 | 0.729 | 1.687 | 4.92× | 0.43× |
| 1 | 2^20 | 13.015 | 2.552 | 4.002 | 5.10× | 0.64× |
| 1 | 2^22 | 51.878 | 9.928 | 14.976 | 5.23× | 0.66× |
| 1 | 2^24 | 202.245 | 39.429 | 56.278 | 5.13× | 0.70× |
| 6 | 2^16 | 1.344 | 0.446 | 2.070 | 3.01× | 0.22× |
| 6 | 2^18 | 2.737 | 0.852 | 2.508 | 3.21× | 0.34× |
| 6 | 2^20 | 6.695 | 2.369 | 3.173 | 2.83× | 0.75× |
| 6 | 2^22 | 16.954 | 6.047 | 6.737 | 2.80× | 0.90× |
| 6 | 2^24 | 54.789 | 15.113 | 15.305 | 3.63× | 0.99× |

## 最大尺寸 verifier、通信量与进程峰值 RSS

RSS 来自每系统/线程一个全新进程，在 prove 返回后、verify 和源输入检查之前读取 VmHWM；包含输入与 runtime。它不是仅工作区字节数。

| 线程 | 系统 | N | verify (ms) | bincode bytes | RSS peak (MiB) |
| ---: | --- | ---: | ---: | ---: | ---: |
| 1 | lookup | 2^24 | 0.1143 | 35608 | 15.03 |
| 1 | packed | 2^24 | 0.1200 | 35608 | 17.03 |
| 1 | flock | 2^24 | 0.1694 | 2704 | 20.98 |
| 6 | lookup | 2^24 | 0.1121 | 35608 | 18.67 |
| 6 | packed | 2^24 | 0.1127 | 35608 | 26.89 |
| 6 | flock | 2^24 | 0.1700 | 2704 | 23.15 |

## Mikros 阶段分解

使用独立 --profile 测量，列出最大 N 的六次均值；不混入上面的无探针中位数。packed 的 A/B packing 已包含 B row weighting。

| 线程 | 内核 | 总计 | 聚合 | C 投影 | A/B binding + B weighting | IP tail | 其他 |
| ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | lookup | 207.692 | 194.466 | 2.963 | 4.701 | 3.393 | 2.169 |
| 1 | packed | 39.997 | 29.899 | 1.326 | 3.973 | 2.693 | 2.106 |
| 6 | lookup | 58.977 | 49.840 | 1.562 | 1.824 | 3.561 | 2.190 |
| 6 | packed | 15.739 | 8.402 | 0.572 | 1.376 | 2.922 | 2.467 |

## 原始数据与复现

本目录原始报告共包含 272 份已核验的证明，含预热、独立阶段实验及 RSS 样本。min/max 和所有样本均保留，没有剔除慢样本。

`bash mikros/run_optimized.sh` 串行构建、测试和复测；`python3 mikros/summarize_optimized.py` 重新核对并生成本报告及 results/optimized/summary.csv。实现与实验取舍见 [OPTIMIZATION_NOTES.md](OPTIMIZATION_NOTES.md)。
