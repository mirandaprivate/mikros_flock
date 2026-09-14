# 相同压缩比：Mikros-128 与 Flock-128

服务器正式样本：6 次/系统/尺寸，另有预热、独立 profile（若启用）与冷进程 RSS。64 份证明的 verifier、transcript 和原始输入声明检查全部成功。另有 7 个与 v3 重合的输入/旧协议组合，其证明字节保持一致。

64 个逻辑线程；CPU `0-47,48-55,72-79`，48 个物理核加 16 个 SMT sibling，双 NUMA 默认 first-touch。共享服务器非独占。双方 native 构建启用 GFNI、AVX-512F/BW/VBMI、VPCLMULQDQ。

`flock128` 跳过 7 轮，压缩 128；Mikros κ=128。`flock` 是保留的 skip6/压缩64基线。Mikros 两种复用变体各持独立 scratch；Flock 使用原生产全局容量池。RSS 来自每系统独立冷进程。

本组不运行 Flock64，Flock128 不会复用 Flock64 的较大缓冲区。smoke 在另一进程执行，退出后不影响主组内存池。

双方关闭 grinding，比较不包括 PCS 或 packed opening，也不声称相同 128-bit soundness。所有输入生成、序列化、源声明 oracle 均在 prover/verifier 计时之外。

## N = 2^30

| 系统 | 压缩比 | prover 中位数 ms（范围） | 相对 Flock128 耗时 | verifier 中位数 ms | 消息 bytes / bincode | 冷进程 RSS MiB |
|---|---:|---:|---:|---:|---:|---:|
| reuse | 128 | 105.268 (93.181–131.203) | 1.031× | 0.2784 | 35776 / 35800 | 913.98 |
| factoredreuse | 128 | 89.729 (81.111–110.019) | 0.879× | 0.2414 | 35776 / 35800 | 915.30 |
| flock128 | 128 | 102.138 (87.789–126.713) | 1.000× | 0.7257 | 4880 / 4912 | 791.97 |

相对耗时小于 1 表示本批中位数快于 Flock128。范围展示全部样本，未删除慢样本；不能仅凭略低中位数宣称稳定领先。

| 系统 | 首次预热证明 ms | 单独冷进程证明 ms |
|---|---:|---:|
| reuse | 112.629 | 107.818 |
| factoredreuse | 97.273 | 81.822 |
| flock128 | 97.892 | 93.754 |

首次预热证明在共用进程里顺序执行，可能已有 runtime/池/页面状态；单独冷进程是另一种生命周期，二者均不混入预热后的中位数。

同一 run 的配对比较：reuse 有 2/6 次快于 Flock128；逐 run 耗时比的中位数为 1.078×。
同一 run 的配对比较：factoredreuse 有 4/6 次快于 Flock128；逐 run 耗时比的中位数为 0.942×。

## 独立 profile

诊断样本单独运行，每项为 3 次均值；不同协议的阶段边界不能直接逐列等同。

reuse, N=2^30: setup_ms=8.450 ms; tensor_aggregate_ms=56.817 ms; certificate_split_ms=0.018 ms; c_pack_project_ms=4.834 ms; transcript_challenge_ms=0.136 ms; ab_pack_ms=18.072 ms; b_weight_ms=0.000 ms; inner_product_ms=12.292 ms; terminal_claims_ms=0.010 ms.
factoredreuse, N=2^30: setup_ms=2.429 ms; tensor_aggregate_ms=47.751 ms; certificate_split_ms=0.021 ms; c_pack_project_ms=7.592 ms; transcript_challenge_ms=0.140 ms; ab_pack_ms=14.798 ms; b_weight_ms=0.000 ms; inner_product_ms=15.151 ms; terminal_claims_ms=0.010 ms.
flock128, N=2^30: round1 URM=59.970 ms; round2 fused fold=15.587 ms; rounds 3+ tail=15.447 ms.

## 可复核材料

- [原始正式数据](results/comparison-64t.json)
- [环境与构建记录](results/server_environment.json)
- [完成状态](results/status.json)
- [源码逐文件摘要](source_manifest.json)
- [额外插值边界测试](validation/status.json)：两项测试分别覆盖全部 256 个 GF8 节点和 16 个域外点；计时结束后链接原 v4 Flock rlib 运行，未改变冻结内核。测试源文件单独记录摘要，不在原 307 文件源码包中。
- 源码压缩包 SHA256：`815a7b0c5b5746d7c19343ba1a020f26a0321c0f2bdd3277a39c54928af0be89`

v1–v3 为旧压缩64基线，保留原记录；同压缩比速度和内存结论须以此轮 Flock128 为准。
