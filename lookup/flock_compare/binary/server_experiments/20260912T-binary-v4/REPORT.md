# Binary permutation：K32 融合与 Flock 对比

本轮以 binary 值（width=1）为主：N30 的值与位置标签合计 31 bits，按实际 tree_stats 使用 K32。previous 为冻结的 fused v3，optimized 新增 K32 融合构造；width=8 回归单独列出，N30 若执行则为 38-bit tuple/K64。

Flock 使用快照 `product_gkr::prove_batched`。两者验证同一关系 f[σ(i)]=g(i)，没有沿用 Hadamard 的 skip/compression 对齐，也不声称同安全等级。当前为 PIOP，未接入 PCS、承诺 opening 或 grinding。

环境：64 threads，固定 `0-47,48-55,72-79`，48 物理核心加 16 SMT sibling，两个 NUMA 节点各 32 logical CPUs。Linux first-touch，没有显式内存绑定；共享服务器。Rust nightly-2026-03-21、target-cpu=native；实际 ISA 与原始记录一致。

## 正式配对结果

每组一次预热、六次正式样本覆盖全部六种系统顺序；smoke 只有一次正式样本。核心计时保留 prover 内同步释放；含转换计时额外包括原生表示转换。公共输入生成/摘要排除，完整源认证单列且执行。

| case | 系统 | core median [min,max] s | core mean s | 含转换 median s | verifier ms | 源认证 median s |
|---|---|---:|---:|---:|---:|---:|
| smoke-w1-n20-64t | Mikros fused v3 | 0.026 [0.026,0.026] | 0.026 | 0.030 | 0.290 | 0.003 |
| smoke-w1-n20-64t | Mikros binary K32 fusion | 0.021 [0.021,0.021] | 0.021 | 0.026 | 0.198 | 0.003 |
| smoke-w1-n20-64t | Flock product_gkr batched | 0.036 [0.036,0.036] | 0.036 | 0.038 | 0.342 | 0.001 |
| screening-w1-n26-64t | Mikros fused v3 | 0.706 [0.694,0.734] | 0.709 | 0.774 | 0.340 | 0.080 |
| screening-w1-n26-64t | Mikros binary K32 fusion | 0.632 [0.596,0.643] | 0.628 | 0.710 | 0.342 | 0.082 |
| screening-w1-n26-64t | Flock product_gkr batched | 1.085 [1.065,1.127] | 1.090 | 1.160 | 0.588 | 0.039 |
| main-w1-n30-64t | Mikros fused v3 | 9.722 [9.493,10.074] | 9.767 | 11.489 | 0.467 | 2.022 |
| main-w1-n30-64t | Mikros binary K32 fusion | 8.620 [8.225,8.862] | 8.590 | 10.277 | 0.462 | 2.044 |
| main-w1-n30-64t | Flock product_gkr batched | 12.060 [11.685,12.238] | 12.032 | 13.174 | 0.786 | 0.603 |
| smoke-w8-n20-64t | Mikros fused v3 | 0.030 [0.030,0.030] | 0.030 | 0.035 | 0.241 | 0.004 |
| smoke-w8-n20-64t | Mikros binary K32 fusion | 0.022 [0.022,0.022] | 0.022 | 0.026 | 0.197 | 0.004 |
| smoke-w8-n20-64t | Flock product_gkr batched | 0.044 [0.044,0.044] | 0.044 | 0.047 | 0.344 | 0.001 |
| screening-w8-n26-64t | Mikros fused v3 | 0.992 [0.878,1.069] | 0.982 | 1.084 | 0.345 | 0.103 |
| screening-w8-n26-64t | Mikros binary K32 fusion | 0.973 [0.918,1.019] | 0.974 | 1.065 | 0.342 | 0.105 |
| screening-w8-n26-64t | Flock product_gkr batched | 1.429 [1.388,1.515] | 1.439 | 1.498 | 0.588 | 0.041 |

比值为 optimized/对照，小于 1 表示候选更快；逐份按相同 width/N/seed/run/input 配对，再取比值中位数。

| case | 对照 | metric | 两中位数之比 | 配对比值 median [min,max] | 候选较快/样本数 |
|---|---|---|---:|---:|---:|
| smoke-w1-n20-64t | previous | prover_core_ms | 0.8302 | 0.8302 [0.8302,0.8302] | 1/1 |
| smoke-w1-n20-64t | previous | prover_total_ms | 0.8604 | 0.8604 [0.8604,0.8604] | 1/1 |
| smoke-w1-n20-64t | flock | prover_core_ms | 0.5986 | 0.5986 [0.5986,0.5986] | 1/1 |
| smoke-w1-n20-64t | flock | prover_total_ms | 0.6681 | 0.6681 [0.6681,0.6681] | 1/1 |
| screening-w1-n26-64t | previous | prover_core_ms | 0.8950 | 0.8874 [0.8424,0.9240] | 6/6 |
| screening-w1-n26-64t | previous | prover_total_ms | 0.9170 | 0.9111 [0.8637,0.9442] | 6/6 |
| screening-w1-n26-64t | flock | prover_core_ms | 0.5822 | 0.5774 [0.5393,0.6011] | 6/6 |
| screening-w1-n26-64t | flock | prover_total_ms | 0.6120 | 0.6066 [0.5703,0.6290] | 6/6 |
| main-w1-n30-64t | previous | prover_core_ms | 0.8867 | 0.8799 [0.8466,0.9110] | 6/6 |
| main-w1-n30-64t | previous | prover_total_ms | 0.8945 | 0.8922 [0.8588,0.9187] | 6/6 |
| main-w1-n30-64t | flock | prover_core_ms | 0.7148 | 0.7148 [0.6899,0.7387] | 6/6 |
| main-w1-n30-64t | flock | prover_total_ms | 0.7801 | 0.7810 [0.7521,0.8003] | 6/6 |
| smoke-w8-n20-64t | previous | prover_core_ms | 0.7401 | 0.7401 [0.7401,0.7401] | 1/1 |
| smoke-w8-n20-64t | previous | prover_total_ms | 0.7559 | 0.7559 [0.7559,0.7559] | 1/1 |
| smoke-w8-n20-64t | flock | prover_core_ms | 0.5046 | 0.5046 [0.5046,0.5046] | 1/1 |
| smoke-w8-n20-64t | flock | prover_total_ms | 0.5614 | 0.5614 [0.5614,0.5614] | 1/1 |
| screening-w8-n26-64t | previous | prover_core_ms | 0.9812 | 1.0079 [0.9062,1.0454] | 2/6 |
| screening-w8-n26-64t | previous | prover_total_ms | 0.9831 | 1.0033 [0.9190,1.0443] | 2/6 |
| screening-w8-n26-64t | flock | prover_core_ms | 0.6808 | 0.6749 [0.6451,0.7135] | 6/6 |
| screening-w8-n26-64t | flock | prover_total_ms | 0.7110 | 0.7046 [0.6755,0.7421] | 6/6 |

各 width 与 N 严格分开，不把 width8/K64 的收益套到 binary/K32，也不混合 smoke、正式计时或 RSS。保留回退与全部波动，样本胜场不保证统计显著性或跨环境稳定性。

## 构造与 GKR 阶段

阶段来自同一次 instrumented prover，均值按实际样本计算；中位数之和不是总中位数。融合构造应与原解码+紧凑层构造+求值的合计比较，不把命名阶段单独移除误认为免费。

| case | 系统 | 阶段 | mean s | median [min,max] s |
|---|---|---|---:|---:|
| screening-w1-n26-64t | previous | 解码/首层构造/求值（含融合） | 0.235 | 0.236 [0.227,0.243] |
| screening-w1-n26-64t | previous | 扩域上层建树 | 0.029 | 0.028 [0.027,0.032] |
| screening-w1-n26-64t | previous | GKR 消息/折叠 | 0.301 | 0.305 [0.280,0.313] |
| screening-w1-n26-64t | previous | 同步层释放 | 0.134 | 0.131 [0.111,0.159] |
| screening-w1-n26-64t | previous | prover 内源检查 | 0.009 | 0.009 [0.008,0.009] |
| screening-w1-n26-64t | previous | 缓冲与物化 | 0.000 | 0.000 [0.000,0.000] |
| screening-w1-n26-64t | optimized | 解码/首层构造/求值（含融合） | 0.111 | 0.111 [0.110,0.113] |
| screening-w1-n26-64t | optimized | 扩域上层建树 | 0.029 | 0.029 [0.026,0.032] |
| screening-w1-n26-64t | optimized | GKR 消息/折叠 | 0.292 | 0.291 [0.272,0.312] |
| screening-w1-n26-64t | optimized | 同步层释放 | 0.186 | 0.182 [0.175,0.208] |
| screening-w1-n26-64t | optimized | prover 内源检查 | 0.009 | 0.009 [0.008,0.012] |
| screening-w1-n26-64t | optimized | 缓冲与物化 | 0.000 | 0.000 [0.000,0.000] |
| main-w1-n30-64t | previous | 解码/首层构造/求值（含融合） | 4.342 | 4.300 [4.285,4.485] |
| main-w1-n30-64t | previous | 扩域上层建树 | 0.376 | 0.375 [0.368,0.392] |
| main-w1-n30-64t | previous | GKR 消息/折叠 | 3.022 | 3.029 [2.947,3.093] |
| main-w1-n30-64t | previous | 同步层释放 | 1.891 | 1.897 [1.670,2.072] |
| main-w1-n30-64t | previous | prover 内源检查 | 0.135 | 0.134 [0.119,0.156] |
| main-w1-n30-64t | previous | 缓冲与物化 | 0.000 | 0.000 [0.000,0.000] |
| main-w1-n30-64t | optimized | 解码/首层构造/求值（含融合） | 2.492 | 2.497 [2.450,2.518] |
| main-w1-n30-64t | optimized | 扩域上层建树 | 0.358 | 0.358 [0.343,0.368] |
| main-w1-n30-64t | optimized | GKR 消息/折叠 | 2.990 | 2.964 [2.895,3.143] |
| main-w1-n30-64t | optimized | 同步层释放 | 2.618 | 2.684 [2.329,2.900] |
| main-w1-n30-64t | optimized | prover 内源检查 | 0.130 | 0.133 [0.096,0.143] |
| main-w1-n30-64t | optimized | 缓冲与物化 | 0.000 | 0.000 [0.000,0.000] |
| screening-w8-n26-64t | previous | 解码/首层构造/求值（含融合） | 0.112 | 0.112 [0.110,0.115] |
| screening-w8-n26-64t | previous | 扩域上层建树 | 0.086 | 0.086 [0.078,0.091] |
| screening-w8-n26-64t | previous | GKR 消息/折叠 | 0.471 | 0.469 [0.414,0.518] |
| screening-w8-n26-64t | previous | 同步层释放 | 0.291 | 0.285 [0.257,0.332] |
| screening-w8-n26-64t | previous | prover 内源检查 | 0.018 | 0.017 [0.015,0.023] |
| screening-w8-n26-64t | previous | 缓冲与物化 | 0.000 | 0.000 [0.000,0.000] |
| screening-w8-n26-64t | optimized | 解码/首层构造/求值（含融合） | 0.114 | 0.115 [0.110,0.117] |
| screening-w8-n26-64t | optimized | 扩域上层建树 | 0.088 | 0.085 [0.081,0.100] |
| screening-w8-n26-64t | optimized | GKR 消息/折叠 | 0.469 | 0.462 [0.444,0.497] |
| screening-w8-n26-64t | optimized | 同步层释放 | 0.283 | 0.284 [0.259,0.305] |
| screening-w8-n26-64t | optimized | prover 内源检查 | 0.018 | 0.016 [0.015,0.023] |
| screening-w8-n26-64t | optimized | 缓冲与物化 | 0.000 | 0.000 [0.000,0.000] |

## 独立冷进程 RSS 与证明字节

| width | N | 系统 | VmHWM GiB | 冷 core s | 冷 total s | 实际 proof bytes |
|---:|---:|---|---:|---:|---:|---:|
| 1 | 30 | previous | 49.736 | 10.537 | 12.294 | 44208 |
| 1 | 30 | optimized | 49.737 | 10.884 | 12.577 | 44208 |
| 1 | 30 | flock | 158.000 | 12.834 | 13.931 | 16176 |

每个 RSS 数字来自单系统、无预热、一次 proof 的独立进程，prover 返回后、verifier/源扫描前读取。它包括公共输入和运行时，不能用混合系统进程的累计 VmHWM 作对照。证明字节以实际文件核验；N30 Mikros 为 44,208 bytes，Flock 为 16,176 bytes，width 不改变这两个消息长度公式。

完整源认证是原始输入直算核验，不等于 PCS。SUMMARY.json 同时给出逐样本 total+verifier+source 的统计，不能把各阶段中位数直接相加。未测硬件计数器，不将计时差异具体归因于 cache、NUMA 或分配器。

## 验收与来源

接受 8 个 case、78 份证明；另有 0 份受干扰尝试的证明核验通过但计时排除。服务器测试通过 8 项；下载 110 个文件、冻结 394 个源码/构建文件的 SHA256 核验通过。

排除尝试：无。每次等待及监测记录均保留；只检查同用户受监测的 benchmark/build，不能保证服务器没有其他负载。等待有上限，不终止其他进程。

所有记录先执行 verifier、claims/transcript 一致性与完整源核验，再输出 verified。下载后检查 JSON/JSONL 相同、执行顺序、输入与 proof digest、一致的实际 Mikros proof SHA256 和文件长度；不同协议不要求相同证明摘要。

Source archive SHA256：`a611bfc25b7f1dafb788843c26f5b84e336d862adccc2667c458571598ddb437`。

Executable SHA256：`b3ebe1efce5b675de1f82992b73263b771efc9ea6281bc402dd3abfaaf9a3467`。

原始证据：[命令](results/commands.json)、[环境](results/environment.json)、[接受清单](results/accepted.json)、[等待记录](results/waits.json)、[源码清单](source_manifest.json)、[计划](../../PLAN.md)。完整数值、阶段、逐输入配对与来源见 [SUMMARY.json](SUMMARY.json) 和 [summary.csv](summary.csv)。
