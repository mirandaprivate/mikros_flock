# 单变量 Mikros 与 Flock-128：64 线程服务器结果

N=2^30，Mikros prover 中位数 98.406 ms，Flock-128 为 90.293 ms；中位数之比（Mikros/Flock）为 1.090。6 组相同输入配对中，Mikros 较快 2 次，持平 0 次。

服务器 CPU：Intel(R) Xeon(R) Gold 6342 CPU @ 2.80GHz；Linux x86_64，64 线程，固定 CPU 集合 `0-47,48-55,72-79`。覆盖两 NUMA 节点，每节点 32 个逻辑 CPU，总计 48 个物理核心及 16 个 SMT sibling。使用 Linux 默认 first-touch，没有显式内存绑定或交错；服务器共享，CPU 不独占。

构建使用 `+nightly-2026-03-21`、`target-cpu=native`，报告确认 `gfni-avx512` 后端与所需 GFNI/AVX512/VPCLMUL 特性。测量时间：2026-09-12T10:18:07Z 至 2026-09-12T10:23:58Z。

本版按 `latex/4-block.tex` 实现 κ=128 的单变量 Hadamard：GF256 中间域、16 个扩域坐标、255 个采样点。终止于直接 packed-opening claims，未加入额外 2κ 列聚合或 PCS；Flock 使用 skip-7，压缩比同为 128。两者使用 BLAKE3、关闭 grinding；这不构成相同安全等级或完整 PCS 性能声明。

## 正式计时

每系统先执行一次 warmup，随后 6 次正式测量；轮换顺序，两系统各先执行 3 次。reuse 保留调用方 scratch 容量，动态值逐次重建；Flock 使用生产实现的内存池。固定表初始化计入首次 proof；预热时间与正式数据分别展示。

| 系统 | prover 中位数 [min, max] ms | verifier 中位数 ms | 源输入检查中位数 ms | 首次 warmup prover ms |
|---|---:|---:|---:|---:|
| Mikros univariate（reuse） | 98.406 [73.379, 128.863] | 0.0380 | 13645.938 | 102.778 |
| Flock skip-7 | 90.293 [76.131, 125.664] | 0.5783 | 8835.311 | 83.998 |

逐输入配对比值 Mikros/Flock 的中位数为 1.067，范围 [0.774, 1.500]；小于 1 表示该输入上 Mikros 更快。配对比值中位数与两系统各自中位数之比是不同统计量。

| run | Mikros ms | Flock-128 ms | Mikros/Flock |
|---|---:|---:|---:|
| 1 | 127.206 | 125.664 | 1.012 |
| 2 | 73.379 | 76.131 | 0.964 |
| 3 | 110.558 | 94.693 | 1.168 |
| 4 | 86.253 | 76.877 | 1.122 |
| 5 | 128.863 | 85.894 | 1.500 |
| 6 | 75.048 | 96.992 | 0.774 |

仅 6 个配对样本，范围和胜场不保证统计显著性或跨机器性能。

## 通信与独立 RSS

| 系统 | F128 元素数 | field payload bytes | bincode bytes | 证明前 VmRSS MiB | 证明后 VmHWM MiB | 冷 proof ms |
|---|---:|---:|---:|---:|---:|---:|
| Mikros univariate（reuse） | 304 | 4864 | 4888 | 395.80 | 916.07 | 106.028 |
| Flock skip-7 | 305 | 4880 | 4912 | 395.80 | 791.88 | 85.874 |

RSS 每系统仅一次全新独立进程，在 prove 后、verify 与源输入检查前读取 VmHWM；包含输入与运行时内存，不是 allocator 专属峰值。冷 proof 时间仅作参考，不混入正式预热后计时。

## 阶段诊断

这是另一个独立进程的 profile=3 测量，另外有一次 warmup；正式统计不含这些样本。占比为阶段均值/同一系统诊断 prover 均值，排除 warmup，不用跨组中位数相减。3 次诊断顺序不平衡，计时器也有开销，不能当作正式胜负或加速比。

**Mikros univariate（reuse）**：3 个诊断样本，prover 均值 99.934 ms。

| 阶段 | 均值 ms | 占诊断总均值 |
|---|---:|---:|
| 权重与准备 | 9.542 | 9.55% |
| GF256 小域聚合 | 55.690 | 55.73% |
| 插值与商多项式 | 0.666 | 0.67% |
| C 打包与投影 | 3.027 | 3.03% |
| Transcript 与挑战 | 0.037 | 0.04% |
| A/B 打包 | 13.896 | 13.91% |
| 内积尾部 | 17.065 | 17.08% |
| 终端 claims | 0.006 | 0.01% |
| 未计入阶段的余项（含日志舍入） | 0.006 | 0.01% |

**Flock skip-7**：3 个诊断样本，prover 均值 98.804 ms。

| 阶段 | 均值 ms | 占诊断总均值 |
|---|---:|---:|
| 首轮 URM | 57.957 | 58.66% |
| 第二轮融合折叠 | 20.427 | 20.67% |
| 第三轮及后续尾部 | 20.013 | 20.26% |
| 未计入阶段的余项（含日志舍入） | 0.407 | 0.41% |

Flock 阶段来自原始日志，单项保留 0.01 ms；余项逐样本计算后取均值。

## 校验与边界

本次共校验 32 条 proof 记录（含 smoke、warmup、正式、profile 和 RSS），服务器测试通过 67 项。驱动状态 complete、全部命令零退出；本地与服务器 source manifest、代码/构建指纹及命令日志 SHA256 一致。同 (N, run) 输入在各模式中一致，各协议内部的 proof 摘要一致；不同协议不要求 proof 摘要相同。

每条记录均完成 verifier、claims/transcript 一致性及独立完整源输入检查。输入生成/摘要、challenger 创建、序列化和源输入检查排除于 prove/verify 计时；检查仍实际执行，其开销列于上表。verifier 时间仅是归约重放，不是带承诺打开的完整验证时间。

旧 v4 mixed-tensor 使用不同协议，仅作为历史记录；本报告不与其混合样本或跨组推导改进幅度。

原始数据：[正式计时](results/comparison-64t.json)、[阶段诊断](results/profile-64t.json)、[诊断日志](results/profile-64t.log)、[服务器环境](results/server_environment.json)、[机器可读汇总](SUMMARY.json)。
