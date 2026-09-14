# 使用同一 production PCS 的 Mikros / Flock E2E

PCS 对齐已完成，并在服务器上重跑三档逐 bit wiring 实验。双方实际复用 Flock production 的 dense commitment、merged sumcheck、Frobenius assist 和单个 EqPoint Ligerito opening；全部 PCS 配置及承诺摘要与未替换 PIOP 的 Flock 原生 E2E 交叉核验相等。

最大规模 2^30 witness bits：Flock 1.636 s，Mikros 1.167 s，Mikros 在线时间减少 28.7%（加速比 1.40×）。这组数字采用已对齐的 PCS 路径；旧 full-padded functional v1 数据保持独立。

## PCS 一致性的实际检查

| 检查项 | 三路共同结果 |
|---|---|
| 承诺输入 | 同一 SHA witness，通过 `UnionInstance::compact_witness` 生成同一 dense stack |
| Commitment | 同一 `commit_lane_major`，50 个 active lanes，BLAKE3 cap |
| 参数 | 完整 `PcsParams` 相等，包括 m、rate、batch size、profile、lanes、hash |
| 安全配置 | embedded config 摘要、PCS query 数及 grinding 向量逐项相等 |
| 承诺结果 | 每档双方和 production 全部样本的序列化 commitment 摘要相同 |
| Opening | 共用 `open_batch_merged`：jagged weight/首轮融合、merged sumcheck、multipoint Frobenius assist |
| 内层 PCS | 每份证明只有一个 `open_batch_mixed_ligerito_seeded` 的 EqPoint opening |
| Verifier | 共用原 merged verifier 主体，只把旧 skip64 列权重检查泛化为完整 128 列权重 |

SHA 的声明行数是 2^nu−1，属于 partial-height layout，实际没有触发仅适用于 full-height prefix 的 block-first 路径。保留 production 的真实分支条件，未通过篡改 counts/零 padding 强行启用该路径。

## 同一 PCS、逐 bit wiring 的在线 prover

服务器 Intel Xeon Gold 6342，固定 64 logical CPUs，两个线程池各 64，release native（GFNI、AVX-512F/BW、VPCLMULQDQ）。双方 κ128，Flock skip7。每系统每档预热 1 次、正式 3 次，交替顺序；以下为正式样本中位数及 min/max。

| Witness bits | Wiring bits | SHA compression 数 | Flock ms [min,max] | Mikros ms [min,max] | M/F |
|---|---|---:|---:|---:|---:|
| 2^22 | 2^18 | 127 | 277.495 [267.089,280.108] | 343.955 [324.273,387.879] | 1.2395 |
| 2^26 | 2^22 | 2,047 | 452.371 [421.556,458.394] | 498.490 [475.792,537.346] | 1.1020 |
| 2^30 | 2^26 | 32,767 | 1635.949 [1619.698,1657.936] | 1166.604 [1144.701,1219.695] | 0.7131 |

Online 包含 commit、PIOP、所有挑战相关的权重转换/分量计算、gather、merged PCS 和函数返回时的临时数据释放。Offline witness/tuple/系数准备、dense compaction、owned q 拷贝、验证和序列化不计入。compaction 使用相同例程，在计时前明确执行，未从融合阶段估算扣除。

## 最大规模阶段分解

| 阶段 | Flock ms | Mikros ms |
|---|---:|---:|
| Commitment | 62.025 | 56.247 |
| Boolean 分支 | 326.028 | 354.961 |
| Wiring 分支 | 1099.778 | 488.166 |
| 并行 PIOP 总时间 | 1099.859 | 488.256 |
| 输入适配 + production merged PCS | 445.858 | 625.055 |

Boolean/wiring 并行，不能相加；各阶段独立取中位数，也不要求中位数精确加总。`pcs_open_ms` 包括新查询的在线适配及原生 merged opening 全流程，未单独拆量每一内核。

相同 PCS 不意味着相同查询工作量：Flock 为 11 个原始 tensor claims（Boolean 3 + wiring 8），Mikros 为 20 个（Boolean 4 + 两个 wiring 端点各 8）。保留全部声明，双方最终各开一个内层 EqPoint；没有把 20 条声明删成 11 条，也没有给 Flock 填补无用查询。

输入适配把 `[a,b]` 非零和权重写成 scale×普通 eq 权重，scale 进入 128 列权重。d 个 `[a,a]` 非零零和维使用 d+1 个固定节点的最高次系数插值，所有分量都经同一 PCS 认证并检查求和；零 functional 必须值为零。旧 production prover 的算法和 proof 结构不变。

## 完整证明验证

| Witness bit-log | Flock / Mikros verifier ms | Flock / Mikros 完整证明 B | 原始 claims / ring claims / inner openings（F; M） |
|---|---:|---:|---|
| 22 | 37.282 / 26.719 | 342,878 / 397,814 | 11/11/1; 20/20/1 |
| 26 | 147.814 / 45.095 | 412,422 / 473,006 | 11/11/1; 20/20/1 |
| 30 | 1187.719 / 90.721 | 487,806 / 555,190 | 11/11/1; 20/20/1 |

51 个定向 native 测试通过；完整 E2E 篡改检查 Flock 23 项、Mikros 33 项均拒绝。38 份实际证明（27 正式、9 预热、2 validation）均完整验证并通过 transcript 一致性检查。同一输入的重复 proof digest 稳定。

新增测试包含：旧 skip64 与 weighted128 verifier 的相同终态、adapter 与直接调用生产 merged prover 的完整 proof 相等、非归一化/零和源 functional 等价，以及两个 component 同时篡改但总和不变仍被 PCS 拒绝。

## Flock 原生 word-wiring 接口参考

| SHA compression 数 | prover 完整接口 ms [min,max] | verifier ms | 完整证明 B |
|---:|---:|---:|---:|
| 127 | 84.145 [72.714,98.850] | 12.725 | 298,099 |
| 2,047 | 156.530 [131.949,177.185] | 15.852 | 366,523 |
| 32,767 | 557.966 [508.326,586.580] | 25.191 | 440,787 |

此参考与主比较的 PCS 已对齐；它仍使用 word wiring、原默认 Boolean/lincheck、packed-direct gather 和融合的中间结果，并且旧完整接口包含内部 assembly/compaction。因此该列继续单列，不能将其和逐 bit 主比较的差值全部归给 Mikros PIOP。
生产接口沿用自己的默认验证调度，PIOP/PCS 验证核心在 1 线程 verifier_pool 内，外层公开矩阵检查仍可能使用全局池；共同 bit 实验的 verifier 可使用全局 64 线程。三路的 prover 都按 64 线程测量，production 的 verifier 列不作为同线程速度对比。

## 证据与限制

- 源码 archive SHA256：`28189bdd8c780c8a44f4ba152d0d63d52e27f745aae82d7739a10f9732ccb5cb`；证据 archive SHA256：`77f47016a4625ebc40101a9a132fc31cefc24e8db653517dea1831d28b41b4a4`。逐文件及 executable 摘要见 evidence-manifest.json 和 results-e2e/binary-sha256.json。
- 服务器运行 UTC：2026-09-12T17:27:24Z 至 2026-09-12T17:29:37Z，所有阶段退出码为零，源码和二进制前后摘要一致。
- 原始样本、构建/测试输出、命令和资源记录均保留在 results-e2e/；本脚本独立复算并核对服务器 summary。
- 单进程同时保留两套 offline 表，RSS 不能归给单独某个系统。资源预检仅覆盖启动前短窗口，不证明整个实验无竞争。
- 三次样本使用固定 witness/transcript，反映运行波动，不代表跨输入统计总体。commit 改变会改变 PIOP 挑战，因此旧 v1 与 v2 的差值不是保持 transcript 不变的单核消融。
- 未增加 zero-knowledge masking，未接入递归 verifier；公开矩阵/sigma 检查仍按完整 verifier 执行。相同 Fast PCS 不构成对两种不同 PIOP 的新整体 128-bit 安全性定理。

运行 `python3 analyze.py` 可重新核验证据并生成本报告和 analysis.json。实现与方法见 [E2E.md](../../../E2E.md)，统计见 [summary.json](results-e2e/summary.json)。
