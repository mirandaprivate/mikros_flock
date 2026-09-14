# 更新后的 Packed Permutation：实现与服务器实测

后续工程优化已完成并超过旧版，见 [v4 优化结果](OPTIMIZED.md)。下文保留 v1/v2 原始结果。

2026-09-14，64 threads，双路 Xeon Gold 6342。结论：新版已按 LaTeX 正文实现，
并加入 GFNI / AVX-512 / VPCLMUL 及缓冲复用，但本轮 **没有超过旧版 Mikros**。
在同一最终二进制内，N30 permutation 中位数慢 26.3%；同 PCS E2E 中位数慢 4.2%，
后者样本区间重叠，不能据此宣称有稳定改善。新版 E2E 仍比本轮 Flock 快约 1.73 倍。

## 最终对照：v2

| 工作负载 | 旧 Mikros | 新 packed Mikros | 新 / 旧 |
|---|---:|---:|---:|
| Permutation，2^30 bit-address cells | 4.68785 s | 5.92159 s | 1.2632 |
| E2E，2^30 witness bits、2^26 wiring bits | 706.55 ms | 736.33 ms | 1.0421 |
| E2E 中的 wiring 分支 | 399.34 ms | 445.71 ms | 1.1161 |
| wiring 扣除已测 permutation PoW | 359.00 ms | 418.35 ms | 1.1653 |

本轮 Flock E2E 为 **1273.16 ms**。三组使用相同原始 witness、commitment、
生产 dense merged PCS 配置、Ligerito backend 和一个最终 inner opening。
源认证数量为 Flock 4、旧 Mikros 5、新 Mikros 5；没有用直接源求值替代 E2E 的 PCS。

Permutation 每组 8 次有效样本，顺序交替；E2E 每组 6 次，三系统轮转，
每个系统在每个顺序位置各出现两次。各组另保留一次预热。
Permutation 范围为旧版 4.509–4.871 s、新版 5.716–6.238 s。
E2E 范围为旧版 648.82–768.92 ms、新版 713.05–775.16 ms、Flock 1239.46–1320.41 ms。
这是单台服务器、固定输入和每个协议固定 transcript 的测量，不是多随机实例的统计显著性结论。

## 工程优化消除了什么

保留了 v1 完整负结果。v1 已使用四路 VPCLMUL 聚合及现有 native sumcheck，
投影仍是通用查表，每次重新分配 children，扩域树直接释放。
v2 改用 64 行 GFNI 线性变换，并为 children 和扩域树复用独立、有界的缓冲池。
矩阵构造、全部数据写入、挑战相关计算仍在线。v1/v2 的新协议证明哈希完全一致。

| 新协议阶段 | v1 | v2 |
|---|---:|---:|
| 完整 permutation | 12.38617 s | 5.92159 s |
| packed 列投影 | 5.51320 s | 0.34364 s |
| packed 系数聚合及 quotient | 2.61132 s | 2.64310 s |
| 扩域树 GKR，含该树释放 | 1.99916 s | 1.17979 s |
| 记录的 layer release | 751.64 ms | 0.13 ms |

投影约快 **16.0 倍**，新协议总时间较 v1 减少 **52.2%**。
这两个版本来自先后运行的独立批次；旧版在 v1 为 4.18101 s，在 v2 为 4.68785 s，
因此最终新旧协议比较必须使用上表的同批 v2 基线，不能挑选不同批次中的有利数字。
`packed_extension_gkr_ms` 与其他 GKR 计时重叠，`packed_aggregate_ms` 已包含于
`messages_ms`，`packed_projection_ms` 已包含于 `layer_materialize_ms`；不能重复相加。

## 为什么仍未超过旧版

N30 bit wiring 的元素含 bit 值与位置标签，编码宽度为 31 bits，实际 tuple field
是 GF(2^32)，因此 **kappa=4**。这里不是不带标签的 GF(2) 多重集 permutation；
后者的 kappa=128，本轮大规模数字不能外推到该情形。

新版把最大的两个尾部 Hadamard/sumcheck 向量长度从 N/2、N/4 降到 N/8、N/16。
最终批次的 fold-and-next-message 合计从旧版 3.25543 s 降到 1.56188 s，节省约 1.69 s。
但每个 compact level 还必须对 column polynomial 和离域 eta polynomial 的系数做加权聚合。
这项新增工作耗时 **2.64310 s，占新版 online 时间约 44.6%**，再加 0.34364 s 投影，
抵消了较短尾部的收益。当前主要瓶颈已经从投影转移到系数聚合。

聚合当前使用正确的域嵌入和四路未约化乘积，仍需扫描全部系数、做 byte-linear embedding
并加权累加。下一项可评估的工程方向是批量 GFNI 系数嵌入与加权累加融合；
它尚未实现、也没有性能结论，不能据此承诺会超过旧版。

E2E Boolean 与 wiring 分支并行，分项不能简单相加。PoW 仍在主 prover time 内：
旧/new wiring wrapper 的 PoW 中位数分别为 40.03/24.44 ms，其中初始 fingerprint 为
39.50/23.99 ms。两协议的 transcript 不同，固定 nonce 工作量不同，这部分差值不是
协议计算收益。表中 wiring compute 是先逐样本扣除再取中位数；没有从并行 E2E wall time
直接扣除该值。Flock 的 PoW 未单独计时，记录为 null。

## Offline、内存和消息

按照本次约定，挑战无关的输入转换、系数与补点多项式准备不计入 online。
所有 z/x/row-weight 相关表、聚合、投影、折叠和源查询转换均计入。
这种 offline 准备包含依赖 witness 的工作，不是仅依赖电路的通用 setup。

| N30 准备与消息 | 旧版 | 新版 |
|---|---:|---:|
| variant offline preparation | 5.36846 s | 18.80475 s |
| 保留的 prepared 数据 | 约 24 GiB | 36 GiB |
| 第一次 proof 样本，未纳入中位数 | 5.92054 s | 6.00361 s |
| 逻辑消息 F128 数 | 2732 | 2736 |
| standalone 序列化 proof | 44,208 B | 44,344 B |
| E2E proof | 490,218 B | 490,330 B |

共同 fixture preparation 为 29.83178 s；共同公开 sigma/index cache 另占 24 GiB。
旧版 prepared 数量包含约 32 KiB 的 leaf embedding 表，表中按 GiB 四舍五入。
两棵树共有两层 packed reduction，因此新增证书与减少的 cubic 消息抵消后，
逻辑消息仅多 4 个 F128，即 64 B；序列化长度还包含不同 Vec header 和 E2E nonce 数量。

旧/new tree pool 的上限均为 32 GiB、32 个 buffer，使用独立槽位。
批末分别保留约 16/22 GiB。整个 standalone 进程同时持有两组 prepared 数据，
最大 RSS 为 152.12 GiB；该数字不是某一个协议的独立峰值。
E2E 三系统同进程最大 RSS 为 19.95 GiB。两次正式任务均无 major faults 或 swap。
standalone 批末显式同步释放 tree/scratch pools 耗时 3.65437 s，单独记录，
未算作任何一个预热后的 proof 时间。预热样本共享已初始化公共设施，不代表完全冷启动。

Standalone 是无 grinding、无 PCS 的 PIOP 测量；每个返回 query 均在计时外用原始 bits
独立求值检查。E2E 则包含 commitment、在线 PIOPs、转换、gather、grinding 与实际 PCS opening。
E2E log-leaves=14 有 32767 次 SHA compression，但 permutation 规模为 2^26 bits；
不能把它称为 2^30 permutation cells 的 E2E。

## 实现和验证

- 协议：`mikros/permutation/src/packed.rs`；域/离域插值：`packed/domain.rs`。
- 系数聚合：`packed/aggregate_kernel.rs`；GFNI 投影：`packed/projection_kernel.rs`。
- 与源及 PCS 的衔接：`wiring.rs` 的 `BitTensorClaim`，支持任意列权重与 low-first 行因子。
- 旧协议和默认旧入口保留；E2E 新版使用 `--permutation packed`，对照用 `--permutation both`。
- 以 `latex/4-block.tex` 的更新正文为准；permutation 算法与证明附录仍是旧版，本次没有改写 LaTeX。

服务器最终 **62 项测试通过**：permutation 43、E2E helper 10、PCS functional 2、
merged tensor 3、weighted PCS 4。小规模 E2E 分别拒绝 Flock 19、旧 Mikros 23、新版30种篡改，
共72项。测试包含真实中间域、独立 source oracle、padding、全输入基向量、native/scalar
聚合一致性、GFNI 尾部边界、复用缓冲跨不同挑战、proof/transcript replay，以及 PCS 认证。
所有正式样本验证通过，证明哈希在重复间稳定；v1/v2 中旧、新 permutation 及三组 E2E
各自的 proof 哈希相同。源码和二进制哈希在服务器运行前后均核对。

## 重现和原始记录

固定 affinity：`0-47,48-55,72-79`；两个 Rayon pool 均64线程。
编译：`nightly-2026-03-21`、`-C target-cpu=native`，确认
`avx512f,avx512bw,gfni,vpclmulqdq`。64逻辑线程覆盖48物理核；并非64物理核。

```sh
python3 mikros/server_experiments/run_packed_permutation_64.py \
  --log-n 30 --log-leaves 14 --reps 8 --e2e-reps 6
```

驱动需在源码快照根目录运行，会原生构建、测试、做资源预检，并保存 JSONL、资源和校验记录。

- [v1 完整结果](../../server_experiments/20260914-packed-permutation/v1/results/summary.json)
- [v2 完整结果](../../server_experiments/20260914-packed-permutation/v2/results/summary.json)
- [v2 permutation 原始样本](../../server_experiments/20260914-packed-permutation/v2/results/permutation.stdout)
- [v2 E2E 原始样本](../../server_experiments/20260914-packed-permutation/v2/results/e2e.stdout)
- [跨版本 proof 检查](../../server_experiments/20260914-packed-permutation/v2/results/cross-version-check.json)
- [源码清单](../../server_experiments/20260914-packed-permutation/v2/source_manifest.json)
- [协议接口与测量边界](../../permutation/PACKED.md)

v1 source archive SHA256: `7bd19885e225e35b21c463152fd336296daff837c641fcdac9cd9b482b093687`。
v2 source archive SHA256: `4ce4758aa49b898cf7edde9903db9e3d5fef0e23642db8b8c4b25842e5f3da52`。
所依据 `latex/4-block.tex` SHA256: `69143a43d6dd972cee634c01fdfd88000b2862429094cd347f0a53a09d45959e`。
