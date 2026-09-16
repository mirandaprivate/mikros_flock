# Mikros 与 FLOCK skip=7：2^30 个 binary Hadamard 约束

同一服务器、64 线程、同一份 packed binary 输入；Mikros κ=128，FLOCK skip=7。
比较独立 Hadamard PIOP；不含 PCS，也不是完整 E2E SNARK。双方关闭 grinding。

3 个进程批次，每批 1 次预热和 6 次正式测量。时间为每系统 18 次正式计时的中位数；每批复用同样的 6 个确定性输入种子，不能视为 18 份独立 witness。

| 系统 | Prover ms | PIOP proof bytes | Verifier replay ms | 冷启动 prover 后峰值 RAM MiB |
|---|---:|---:|---:|---:|
| Mikros | 64.170 | 4,888 | 0.0407 | 913.88 |
| FLOCK skip=7 | 90.538 | 4,912 | 0.6126 | 792.94 |

FLOCK/Mikros 的 prover 中位数之比为 **1.411×**；Mikros 在 18 个配对计时中快于 FLOCK **18 次**。

正式样本范围（保留所有样本，未剔除长尾）：
- Mikros：prover 49.684–80.532 ms，verifier replay 0.0320–0.0857 ms。
- FLOCK skip=7：prover 80.755–105.169 ms，verifier replay 0.4134–1.2962 ms。

Verifier replay 只检查归约协议，仍返回待认证声明。所有证明的全部源输入声明均在服务器通过独立的线性时间 oracle 检查；该 oracle 不计入 prover/verifier replay 时间，也不替代 PCS。

| 系统 | 源声明 oracle 中位数 ms | 逻辑消息 F128 数 | 逻辑消息 bytes |
|---|---:|---:|---:|
| Mikros | 13538.590 | 304 | 4,864 |
| FLOCK skip=7 | 8876.891 | 305 | 4,880 |

RAM 主表取各系统独立冷进程在 prover 返回、verifier/oracle 执行前读取的 VmHWM 最大值。它包含 384 MiB 原始输入及运行时，不是 prover 的增量内存。GNU time 的全进程峰值覆盖随后 verifier/oracle，另记在 JSON 的 full_process_rss 中；不能与主表混用。

Mikros 使用 PairAggregate 补点内核和 scratch 容量复用，FLOCK 使用原生 skip7 与生产内存池。随机权重、动态表及在线 packing 均计入 prover；固定表首次初始化计入预热或冷进程。独立 profile 样本仅供阶段诊断，未计入正式中位数。

每份原始记录须通过 verifier、Fiat–Shamir 下一挑战一致性和全部源声明检查。本地报告审计重新核对源码包、实际 ELF、命令、CPU 设置、证据哈希及原始统计；本地未重新执行密码学验证。

原始证据：`C:\Users\miranda\git\mikros_flock\mikros\server_experiments\20260917-hadamard\v1\evidence\results-hadamard`。

完整配置、所有原始样本、profile、RAM 与审计结果见 [compare_hadamard_2_30.log](compare_hadamard_2_30.log)。
