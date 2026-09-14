# 服务器 N=2^30：Mikros Packed 与 Flock

本报告仅由已完成的原始服务器记录生成。N=1,073,741,824 个 binary Hadamard 约束；三份 packed 输入共 384 MiB。

## 环境与比较范围

- CPU：Intel(R) Xeon(R) Gold 6342 CPU @ 2.80GHz；linux x86_64；编译目标特性：aes, pclmulqdq, gfni, avx2, avx512f, avx512bw, vpclmulqdq。
- Rust：rustc 1.96.0-nightly (ac7f9ec7d 2026-03-20); binary: rustc; commit-hash: ac7f9ec7da74d37fd28667c86bf117a39ba5b02a; commit-date: 2026-03-20; host: x86_64-unknown-linux-gnu; release: 1.96.0-nightly; LLVM version: 22.1.0。同一服务器 executable，复用相同 F128 backend 与 BLAKE3 transcript。
- 测量时间：2026-09-12T07:07:03Z 至 2026-09-12T07:19:19Z（UTC）；共享服务器，未声明独占。
- 同一 NUMA 节点 0，每个物理核选择一个逻辑 CPU；分别使用 1、6、24 个物理核。仅设置 CPU affinity，内存采用 Linux 默认 first-touch，没有显式内存绑定或交错分配。
- 每系统预热一次、正式测量六次；每个线程配置中两系统各先执行三次，使用逐 run 相同的输入。没有剔除慢样本；六次样本不构成统计显著性保证。
- 全部 witness 变换、动态查表构造、分配、权重、fold、transcript 与终端声明构造计入在线 prover；输入生成/digest、challenger 初始化、序列化和完整源输入核验在计时外。
- 两边均为 PIOP reduction，不含 packed-opening reduction、PCS commitment/opening；关闭 grinding。协议参数不同，不声称相同的 128-bit soundness，也不将它称作完整 SNARK 比较。
- Flock 保留生产实现的跨 proof scratch 池，Mikros scratch 在每份 proof 内分配与复用；主表是各自原生分配策略下的预热吞吐比较。进程峰值 RSS 另用冷启动独立进程测量。

| 线程 | CPU affinity |
| ---: | --- |
| 1 | 0 |
| 6 | 0, 1, 2, 3, 4, 5 |
| 24 | 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23 |

## 在线 prover

单位 ms。范围为六次正式样本的 min–max；Packed/Flock 大于 1 表示 Packed 中位数更慢。相对 1T 的扩展比在各系统内部计算。

| 线程 | Packed 中位数（范围） | Flock 中位数（范围） | Packed/Flock | Packed 相对 1T | Flock 相对 1T |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 10091.533（10088.373–10153.267） | 673.693（673.141–799.112） | 14.979× | 1.00× | 1.00× |
| 6 | 1759.024（1753.739–1764.590） | 129.784（124.491–151.574） | 13.553× | 5.74× | 5.19× |
| 24 | 567.076（528.383–573.974） | 60.175（51.678–77.973） | 9.424× | 17.80× | 11.20× |

在这台服务器的全部三组配置中，当前 Packed Mikros prover 都慢于 Flock。不能将此前 i7 / WSL2 小规模比较中的相对优势外推至此次 Xeon N=2^30 运行。

源码与编译特性表明，Flock 的 inner shift/reduce 使用 GFNI + AVX-512 专用分派，tail folding 使用 AVX-512 + VPCLMUL；Mikros 的 64 行聚合仍使用可移植 u64 转置和 subset-table 查询。两边使用相同 native 编译配置，但 CPU 专用内核的实现程度不同。本轮没有分阶段 profile 或指令集消融，不能据此把全部耗时差归因于某一种指令集、缓存或频率因素。

## Verifier、通信与内存

Verifier 仅验证 PIOP reduction；全部原始 A/B/C 声明另行认证。RSS 来自每系统/线程一个新进程，在 prover 返回后、verifier/源检查前读取 VmHWM，包含 packed 输入、runtime、表与工作区。单次冷启动 RSS 不是稳定内存分布估计，也不是完整进程运行至退出的峰值。

| 线程 | 系统 | verifier 中位数 (ms) | bincode bytes | prove 前 RSS (MiB) | prove 后进程峰值 RSS (MiB) |
| ---: | --- | ---: | ---: | ---: | ---: |
| 1 | packed | 0.1452 | 35,800 | 387.15 | 898.50 |
| 1 | flock | 0.2231 | 2,896 | 387.16 | 1156.06 |
| 6 | packed | 0.1489 | 35,800 | 387.39 | 897.00 |
| 6 | flock | 0.2248 | 2,896 | 387.37 | 1156.93 |
| 24 | packed | 0.1478 | 35,800 | 388.26 | 893.25 |
| 24 | flock | 0.3338 | 2,896 | 388.22 | 1159.18 |

## 计时外的完整源输入核验

每份预热、正式证明、RSS 和 smoke 都执行 verifier replay、transcript 状态一致性和三个完整源输入声明检查。下表是主实验中单份 proof 的源检查时间（秒），不包含在上面的 prover/verifier 时间中；这些线性扫描保留到 N=2^30，没有采样或省略。

| 线程 | 系统 | 源检查中位数 (s) | 源检查范围 (s) |
| ---: | --- | ---: | ---: |
| 1 | packed | 13.637 | 13.626–13.693 |
| 1 | flock | 9.164 | 9.155–9.217 |
| 6 | packed | 13.586 | 13.552–13.604 |
| 6 | flock | 9.159 | 9.156–9.229 |
| 24 | packed | 13.610 | 13.541–13.656 |
| 24 | flock | 9.157 | 9.147–9.164 |

主实验进程墙钟时间包含预热、六轮两系统运行、输入生成/哈希、所有源检查、序列化和输出，不能与单份 prover 时间直接比较。

| 线程 | 主实验整个进程 (s) |
| ---: | ---: |
| 1 | 238.001 |
| 6 | 174.935 |
| 24 | 166.107 |

## 核验与追溯

汇总器已检查 complete 状态、全部 12 个驱动阶段零退出、锁定/离线构建、CPU affinity 与物理核/NUMA 拓扑、各系统先执行三次、42 份主实验（含预热）、6 份独立 RSS 和 6 份 smoke，共 54 份证明记录。
smoke 使用 N=2^20 的 lookup/packed/Flock；两个 Mikros 内核证明摘要相同。所有线程、批次、RSS 中相同 N/run/protocol 的输入与序列化 proof 摘要完全一致；所有报告编译时源码指纹一致，服务器 source manifest 与本地 manifest 一致。源检查成功来自 benchmark 的完成记录，汇总器不会重新生成密码学证明。

- 源码 fingerprint（BLAKE3）：`8b3e1454063885e319f23b7d51ef16b53619d8c33491aa488967c4f19b168956`。
- 服务器 executable SHA256：`e3ca10502c3e0d0d7ccb3a8310f6ae6d57dfd3aff8b30759277d45004a91c067`。
- 原仓库 revision：`b684b1258e4b1f202bec24afd660ace851b09e5e`；具体源码以快照 manifest 为准。
- [服务器环境](results/server_environment.json)、[命令及墙钟记录](results/commands.json)、[完成状态](results/status.json)、[源码 manifest](source_manifest.json)、[汇总 CSV](results/summary.csv)。
- [Rust 测试日志](results/tests.log)；首次离线构建因依赖缓存不全停止，随后按 Cargo.lock 补齐依赖并成功重跑，最初记录保留在 bootstrap-* 文件中。
- CPU affinity 不等于独占 CPU；原始样本、loadavg 与环境记录均保留。

复核命令：`python3 mikros/server_experiments/summarize_server_n30.py --run-dir <本地运行目录>`。只有全部校验通过才生成 REPORT.md 与 summary.csv。

## 实际 cgroup 补充记录

另从驱动进程所属 cgroup 逐级采集至根节点，保留起止 cpu.stat、cpu.max 和 memory.max。
起止采样未发现有限 CPU 配额：是；未发现有限 memory.max：是。此检查不排除其他任务的竞争。
记录：[开始](results/effective_cgroups_start.json)、[结束](results/effective_cgroups_end.json)。
