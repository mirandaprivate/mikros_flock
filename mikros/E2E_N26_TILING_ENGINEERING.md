# Mikros N26 分块与缓冲复用（v11）

本轮延续 v10 的 `--packed-direct --wp-uninit` 作为双方共同基线。
下述四个新选项均只影响 Mikros，默认关闭。原有 selector SIMD、direct W′ SIMD
和 gate fusion 继续关闭，以便区分本轮改动的收益。

## 与 LaTeX 的对应

`latex/alg/alg-multiple-instance.tex` 要求保留 virtual M 的终值及其 Hadamard
认证，也要求两棵 copy tree 的终点独立。这里仅改变同一值的生成和内存生命周期：
所有消息、挑战顺序、终端声明及 PCS 均保持不变。

## 实现

| 选项 | 改动 | N26 multiple 的理论内存差异 |
|---|---|---|
| `--gate-virtual-product` | 首轮消息及第一次折叠按块生成原始 L AND R；之后独立折叠 M | 消除完整 1 GiB 初始 M 数组 |
| `--wh-tiled-weights` | 保留高/低两个张量，在原有 2048 行聚合和 1024 行 packing 边界生成权重 | 1 GiB 权重表换为 1 MiB 高张量、16 KiB 低张量及每个活跃任务 16–32 KiB tile |
| `--gate-wh-scratch` | 移交两个已无用 gate fold Vec，作为 IP 的下一轮输出缓冲 | 复用两条 N/2 容量，共 1 GiB；要求 virtual product 开关 |
| `--copy-paired-terminals` | 两个独立点共享每条 wire 的扫描、SIMD load 和小张量准备 | 理论 witness 读取由 6 GiB 降为 3 GiB |

以上是数组尺寸和循环访问量，并非实测 DRAM 流量或 RSS 降幅。
分块权重在聚合和 packing 各生成一次；是否划算由测量决定。
缓冲移交延长其存活期，因此需要同时检查 IP 时间、最终释放时间和 MaxRSS。

首轮 M 的块只由原始 packed L/R 生成。不能把已经折叠的 L/R 做 AND 来替代
折叠后的 M。所有 XOR 行同样保留 Hadamard 认证。
权重允许任意 pair、零 pair 和 `[t,t]`，没有归一化或求逆。
IP 只复用 Vec 容量，旧值全部覆盖。所有移交缓冲在本次证明返回前释放。

已有 GFNI 插值聚合、共享 packing 查表、四路未约化 IP 首消息累加均保留。
本轮为 tile 权重生成和双点 dot 复用现有四路域乘法内核。

## 测量与验证

服务器 64 threads，固定 affinity `0-47,48-55,72-79`，native
GFNI/AVX-512/VPCLMUL；single 是一个 N26 电路，multiple 是 128 个相同 wiring、
独立输入的 N26 电路，总共 2^33 constraints。
双方使用相同原生 FLOCK PCS、承诺源、参数、system allocator 和释放/recommit 策略。
online 包含转换、commit/recommit、PIOP、PCS、释放及序列化；offline 排除，verify 独立。
MaxRSS 覆盖整个新进程，包括 offline 和 verify。

本地便携路径 116 项库测试和 22 项 E2E 测试通过。服务器 native 测试实际通过
232 项，另有一项原有测试 ignored。N14 参考及全开配置的完整证明和声明已核对一致。
N26 的带 trace 诊断用于观察各模块，不计入主样本。主结果至少每臂三次新进程，
同一系统各臂的完整 proof hash/长度、PIOP、terminal claims、commitment、PCS 参数和
声明数必须一致。FLOCK 是新选项的全程负对照；Mikros single 仅会受到 tiled weights
影响，不能将全部 single 配置当作负对照。

源码快照：`server_experiments/20260914-plonk-e2e/v11/`，仍为已授权的 384 个文件。
原 N30 报告不变。结果由 `n26-analysis/summarize_tiling.py` 审计；性能结论以完整
服务器证据生成的根目录 `e2e_n26_tiling_optimization_compare_flock.log` 为准。

## 已完成的实测结果

六个配置各三次、每次四种 mode/system，共 72 个 N26 主样本；另有 24 个 N26
独立诊断和 8 个 N14 pilot。全部验证通过，跨配置完整证明及前版 v10 WP-on
基线的证明、声明、承诺和 PCS 参数均一致。当前 384 项源码与 v11 快照一致。

全开组合是本轮 N26 multiple 的候选配置：
`--packed-direct --wp-uninit --gate-virtual-product --wh-tiled-weights --copy-paired-terminals --gate-wh-scratch`。
所有新开关默认仍关闭，可以单独做消融。

| 指标（中位数） | 本轮参考 | 四项全开 |
|---|---:|---:|
| Mikros multiple gate | 1.3441 s | 1.0217 s |
| Mikros multiple copy | 2.6298 s | 2.6895 s |
| Mikros multiple online | 17.6922 s | 16.7468 s |
| FLOCK multiple online | 22.0857 s | 22.3990 s |
| Mikros single online | 4.9353 s | 4.9420 s |
| FLOCK single online | 9.1640 s | 9.2085 s |

Gate 下降约 24%，其全开样本范围 1.0014–1.1935 s，参考为 1.3279–1.4900 s。
Multiple E2E 观察到约 5.34% 中位数降幅，当前 F/M 约 1.338；仍未达到 2×。
两臂总时间中位数所对应的实际样本间，有 475.923 ms 差值来自未修改代码的 witness PCS，
约占总差值的 50.34%。这可能包含运行波动或内存布局的间接影响，不能全部归因于新内核。
single 总时间基本不变；copy 虽然单次诊断的 wire terminals 从 188.743 降至约 41.203 ms，
其主样本总时间未呈现稳定下降，不能将局部加速直接当成 copy 或 E2E 加速。

全开 Mikros multiple 的整进程 MaxRSS 中位数约 26.28 GiB，仍由 PCS 等更大的阶段主导；
省去中间数组并不意味着该整进程峰值会减少相同字节数。
三次主样本的 online 范围为 16.6515–17.6620 s，参考为 16.8649–18.1611 s，
报告保留全部原始样本及交错执行顺序。
