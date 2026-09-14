# Packed Permutation 工程优化：新版已超过旧版

2026-09-14，双路 Xeon Gold 6342，64 threads，GFNI / AVX-512 / VPCLMUL。
本轮将新版的系数嵌入与聚合融合，并将最高 compact layer 的求值改为 GFNI。
新版协议、证明消息、挑战顺序和原始源认证保持不变。

这里的“旧版”指 legacy Mikros permutation；“优化前新版”指上一轮 v2 packed
实现。本轮冻结的默认实现为 v4，原始数据及源码保存在同名版本目录。

## 完整证明结果

| 工作负载 | 旧版 Mikros | 本轮新版 Mikros | 新版耗时降低 |
|---|---:|---:|---:|
| Permutation：2^30 bit-address cells | 5.45309 s | **3.87102 s** | **29.01%** |
| E2E：2^30 witness bits、2^26 wiring bits | 722.10 ms | **638.47 ms** | **11.58%** |
| E2E wiring 分支 | 422.50 ms | 338.14 ms | 19.97% |
| wiring 扣除逐样本实测 permutation PoW | 383.10 ms | 317.06 ms | 17.24% |

本轮 Flock E2E 为 **1288.36 ms**，新版约快 **2.02 倍**；相对旧 Mikros，
standalone 约快 1.41 倍、E2E 约快 1.13 倍。Standalone 不含 PCS，不能称为 SNARK。
E2E 使用相同 witness、commitment、生产 dense merged PCS 配置和 Ligerito backend；
Flock / 旧 Mikros / 新 Mikros 的 tensor claims 为 4 / 5 / 5，最终均为一个 inner opening。
E2E 包含 commitment、PIOPs、在线转换、gather、PCS opening 和配置要求的 PoW。

Standalone 每组 8 次正式样本，交替顺序，另各有一次预热：旧版范围
5.21432–5.54737 s，新版 3.83971–3.95284 s，区间不重叠。
E2E 每组 6 次正式样本、三系统轮转，各顺序位置出现两次，另各有一次预热：
旧版范围 668.13–816.29 ms，新版 602.75–680.73 ms，Flock 1238.27–1390.86 ms。
E2E 新旧区间少量重叠；上述收益是本轮固定输入和固定 transcript 的中位数结果，
不能外推为所有电路或随机实例都达到同样改善。

Boolean 与 wiring 分支并行，分项不能直接相加。旧/新 wiring PoW 中位数为
36.14 / 27.77 ms，包含初始 fingerprint 的 35.57 / 27.29 ms。不同协议 transcript
对应不同固定 nonce 工作量，因此这部分差异不全是协议计算收益。表中最后一行先
逐样本扣除再取中位数；没有从 E2E wall time 直接减去并行分支的 PoW。Flock 的
PoW 未单独计时，仍为 null。

## 实际改动

1. **系数嵌入与加权累加融合。** 原内核对每个 GF(2^32) 系数做四次 byte-linear
   查表，构造 F128，再累加。现在将连续系数经 GFNI 批量嵌入到最多 16 KiB 的
   局部 tile，立即用四路 VPCLMUL 消费。没有创建全量扩域系数数组。
2. **最高层求值使用 GFNI。** 对已有 16-byte polynomial coefficient 行做
   challenge-dependent binary-linear map，输出保持物理行序。映射矩阵在线构造。
3. **保留权重坐标分解候选。** 另一条路径将共享 low equality weights 在线转换为
   GF(2^32) 上的四个坐标，对原始 u32 系数做未约化乘积累加，块末才约化并重组。
   使用的是实际子域基，不是把不同域坐标强制转换。它与 GFNI 性能接近，保留为可选。

默认 `MIKROS_PACKED_AGGREGATE=auto`：支持的 GF32 大块使用 GFNI；少于 64 行、
其他字段/shape 或不支持的 CPU 使用原路径。可显式选择 `baseline`、`gfni`、`tower`。
最高层 GFNI 求值独立于该聚合开关，比较聚合内核时不能把它的收益混入。
缓存仅保存固定公开域的线性映射；挑战相关工作没有移到 offline。

## 聚合消融

v3 独立探测使用同一 native binary、64 threads、8192 个共享 low weights。
每个 shape/kernel 有 6 次正式样本及一次预热，顺序轮转，所有结果逐系数与原内核相等。
`point` 和基础 equality 表是共同输入，不在微基准计时内；每次 Plan 构造及候选
额外的挑战权重转换在计时内。完整 prover 则包含基础 equality 构造。

2^25 行时的中位数，单位 ms：

| 列数 × 每列系数数 | 原聚合 | GFNI | 权重分解 | GFNI 加速 |
|---|---:|---:|---:|---:|
| 4 × 4 | 180.153 | 25.102 | 24.791 | 7.18× |
| 3 × 4 | 136.653 | 23.684 | 23.730 | 5.77× |
| 4 × 2 | 90.277 | 14.854 | 15.452 | 6.08× |
| 3 × 2 | 67.136 | 11.182 | 11.211 | 6.00× |

GFNI 与权重分解的样本区间均重叠，不能宣称其中一条稳定胜出。默认统一使用 GFNI，
另一条保留消融入口。较小的 2^21 行探测也获得约 6.5–7.9 倍改善；较大输入占
768–2048 MiB，超过 LLC，收益并非仅存在于小矩阵。没有硬件计数器证据证明带宽已饱和。
两条候选每四行每系数都仍需四条 VPCLMUL；改善来自嵌入/访存/约化的工程安排，
不能解释为 carryless multiplication 指令减半。

## 瓶颈变化

| 新版阶段 | 优化前 v2 | 本轮 v4 |
|---|---:|---:|
| 完整 permutation | 5.92159 s | 3.87102 s |
| packed 系数聚合及 quotient | 2.64310 s | **0.38355 s** |
| packed 列投影 | 0.34364 s | 0.35722 s |
| 最高 compact 层求值 | 0.36085 s | 0.29110 s |
| fold-and-next-message | 1.56188 s | 1.89076 s |
| extension tree GKR，含重叠分项 | 1.17979 s | 1.35448 s |

这是两个批次：旧协议基线也从 v2 的 4.68785 s 变为本轮 5.45309 s，故该表用于
定位瓶颈，不能作为每项优化的严格隔离消融。聚合约快 6.89 倍，与同一二进制内的
微基准一致；整体新版跨批次耗时减少 34.63%。最终新旧协议结论采用本轮同批对照。
最高层 GFNI 求值尚没有单独关闭/开启的完整 proof 消融，不能独占归因其阶段变化。

当前聚合约占新版 9.9%，而 fold-and-next-message 约占 48.8%，成为主要耗时。
`packed_aggregate_ms` 已包含于 `messages_ms`，`packed_projection_ms` 包含于
`layer_materialize_ms`，`packed_extension_gkr_ms` 与树证明分项重叠，不能重复相加。
后续可评估利用已知 sumcheck claim 少算一次局部评估；本轮尚未实现或计入收益。

## 正确性、准备和资源

最终服务器检查：55 项 permutation 测试、10 项 E2E helper、9 项 PCS 测试全部通过，
共 **74 项**；小规模完整 E2E 的 72 个篡改用例全部被拒绝。原生探测每组另通过
42 项库测试和一个显式忽略的性能测试；非零起点、64-row 边界、尾部、退化权重和
跨域 fallback 均有回归检查。正式样本全部通过验证、transcript 重放及原始源检查。

与 v2 相比，所有系统的完整 proof 哈希与长度保持一致：

| 证明 | BLAKE3 | bytes |
|---|---|---:|
| Standalone 旧版 | `d0506a2eda7d148161a645d0668b22a6094c5bfc02e46f85abd995d788ab8019` | 44208 |
| Standalone 新版 | `0805ec0438e5308ea673bec5a0e14bb2de7ac2efc95ea0eb38985c7b935ce576` | 44344 |
| E2E Flock | `5016cb6132627d789a78d7d02440984c6b0405bb42d9a49aa1d6d113be8318da` | 458866 |
| E2E 旧版 | `34f4d984c9a09a522498c239c97274a89e96dc09e68a6b1d2e70272a342bb94d` | 490218 |
| E2E 新版 | `38bceeafdd723ab13e69bc3ab09566faef9b5536a87d58b7d9918ac50b68fec4` | 490330 |

相同 E2E commitment：`c56c0855e51cbbed008c8c77998d8b9e2b01f3c2b6f7d82e99f67e46f722bde2`。
Standalone 逻辑消息仍为旧版 2732、新版 2736 个 F128；本轮优化没有缩短证明。

按约定，挑战无关输入转换/系数准备不计入 online，但它依赖 witness，并非仅电路 setup。
N30 共同 fixture 为 30.549 s，旧/新 offline preparation 为 5.529 / 18.687 s；
prepared 数据仍约 24 / 36 GiB，共同 public sigma/index cache 另占 24 GiB。
所以本轮优势是 online prover 时间，未消除新版准备及内存代价。

64 线程固定在 `0-47,48-55,72-79`，对应 48 个物理核和 16 个额外 SMT 线程。
Standalone 两系统同进程峰值 RSS 约 152.12 GiB，E2E 三系统同进程约 19.95 GiB，
均无 major faults / swap；这些不是某一个系统的独立峰值。旧/新 buffer pool 上限
仍各为 32 GiB，批末保留约 16 / 22 GiB。批末同步 scratch teardown 为 3.458 s，
单独记录，未并入预热后的 proof 时间。预热样本不代表完全冷启动。

## 复现与证据

使用 `package_e2e.py` 冻结源码；每份 368 个文件，不含凭据、Git 历史或构建产物。
驱动先验证源码哈希、刷新时间戳避免误用复制的 Cargo 缓存，然后 native 构建、
测试和测量，结束后再验证源码及二进制哈希。最终默认源码与本地实现一致。

- [正式结果 summary](../../server_experiments/20260914-packed-permutation/v4/results/summary.json)
- [正式原始 permutation 样本](../../server_experiments/20260914-packed-permutation/v4/results/permutation.stdout)
- [正式原始 E2E 样本](../../server_experiments/20260914-packed-permutation/v4/results/e2e.stdout)
- [大输入聚合探测](../../server_experiments/20260914-packed-permutation/v3/results-large/summary.json)
- [小输入聚合探测](../../server_experiments/20260914-packed-permutation/v3/results/summary.json)
- [保留的优化前负结果](RESULTS.md)

v3 source SHA256：`4e77ff6665070e3243a49f7a1a42d4ef44db8bc2c1d702ee3a3046384109965d`。
v3 evidence SHA256：`f4fd6ee46f1a037fe69538d8f24ebff82f4ca9d4a68266cea986270a36b03a49`。
v4 source SHA256：`f9d0b8fda303dd27d7e4ea810c31812a320c10b1c96759714125a379c1e137d6`。
v4 evidence SHA256：`bc1b2f93613de8f833779f020d233835881faf71df4500072d5adc8282722808`。

服务器目录位于原授权实验根目录内的 `v13-packed-aggregate-probe-20260914`
和 `v14-packed-aggregate-20260914`。两轮均已完成。
