# 相同压缩比：Mikros-128 与 Flock-128

服务器正式样本：4 次/系统/尺寸，另有预热、独立 profile（若启用）与冷进程 RSS。40 份证明的 verifier、transcript 和原始输入声明检查全部成功。另有 5 个与 v3 重合的输入/旧协议组合，其证明字节保持一致。

64 个逻辑线程；CPU `0-47,48-55,72-79`，48 个物理核加 16 个 SMT sibling，双 NUMA 默认 first-touch。共享服务器非独占。双方 native 构建启用 GFNI、AVX-512F/BW/VBMI、VPCLMULQDQ。

`flock128` 跳过 7 轮，压缩 128；Mikros κ=128。`flock` 是保留的 skip6/压缩64基线。Mikros 两种复用变体各持独立 scratch；Flock 使用原生产全局容量池。RSS 来自每系统独立冷进程。

本组包含两个 Flock 参数，它们共用容量池；首次 warmup 不代表独立首次分配。

双方关闭 grinding，比较不包括 PCS 或 packed opening，也不声称相同 128-bit soundness。所有输入生成、序列化、源声明 oracle 均在 prover/verifier 计时之外。

## N = 2^30

| 系统 | 压缩比 | prover 中位数 ms（范围） | 相对 Flock128 耗时 | verifier 中位数 ms | 消息 bytes / bincode | 冷进程 RSS MiB |
|---|---:|---:|---:|---:|---:|---:|
| flock | 64 | 114.516 (90.774–142.120) | 0.983× | 0.3795 | 2864 / 2896 | 1167.81 |
| flock128 | 128 | 116.457 (92.588–126.965) | 1.000× | 0.6247 | 4880 / 4912 | 791.71 |

相对耗时小于 1 表示本批中位数快于 Flock128。范围展示全部样本，未删除慢样本；不能仅凭略低中位数宣称稳定领先。

| 系统 | 首次预热证明 ms | 单独冷进程证明 ms |
|---|---:|---:|
| flock | 101.684 | 95.366 |
| flock128 | 122.585 | 95.720 |

首次预热证明在共用进程里顺序执行，可能已有 runtime/池/页面状态；单独冷进程是另一种生命周期，二者均不混入预热后的中位数。

同一 run 的配对比较：flock 有 2/4 次快于 Flock128；逐 run 耗时比的中位数为 1.057×。

## 可复核材料

- [原始正式数据](parameter-sweep/comparison-64t.json)
- [环境与构建记录](parameter-sweep/server_environment.json)
- [完成状态](parameter-sweep/status.json)
- [源码逐文件摘要](source_manifest.json)
- [额外插值边界测试](validation/status.json)：两项测试分别覆盖全部 256 个 GF8 节点和 16 个域外点；计时结束后链接原 v4 Flock rlib 运行，未改变冻结内核。测试源文件单独记录摘要，不在原 307 文件源码包中。
- 源码压缩包 SHA256：`815a7b0c5b5746d7c19343ba1a020f26a0321c0f2bdd3277a39c54928af0be89`

v1–v3 为旧压缩64基线，保留原记录；同压缩比速度和内存结论须以此轮 Flock128 为准。
