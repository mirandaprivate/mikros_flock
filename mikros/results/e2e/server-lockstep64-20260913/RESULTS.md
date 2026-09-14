# 同一 Flock production PCS：Mikros / Flock E2E v3

最大规模 2^30 witness bits，lockstep 模式的在线 prover 中位数：Flock 1.530 s，Mikros 0.812 s；M/F = 0.5304，Mikros 时间减少 47.0%。

双方复用同一 production PCS 路径：dense witness compaction、commit_lane_major、open_batch_merged、merged sumcheck、Frobenius assist，以及单个 EqPoint Ligerito opening。完整 PcsParams、embedded Fast 配置摘要、query 数及 grinding 向量逐项相等；每档双方、各消融模式与原生 production 的序列化 commitment 摘要相同。相同 PCS 接口与配置不要求相同的终端声明数量或证明长度。

## 主比较：lockstep，64 threads

服务器 CPU：Intel(R) Xeon(R) Gold 6342 CPU @ 2.80GHz。固定 64 个 logical CPUs（0-47,48-55,72-79），两个线程池各 64；release target-cpu=native，GFNI、AVX-512F/BW、VPCLMULQDQ 均启用。κ = 128，Flock skip = 7。预热 1 次、正式 3 次，两系统交替顺序；表中为正式样本中位数及 min/max。

| Witness bits | Wiring bits | SHA compression 数 | Flock ms [min, max] | Mikros ms [min, max] | M/F |
|---|---|---:|---:|---:|---:|
| 2^22 | 2^18 | 127 | 133.204 [124.012, 145.178] | 174.092 [171.266, 183.962] | 1.3070 |
| 2^26 | 2^22 | 2,047 | 319.239 [317.187, 353.862] | 239.187 [237.880, 292.084] | 0.7492 |
| 2^30 | 2^26 | 32,767 | 1530.045 [1500.835, 1543.031] | 811.590 [769.390, 859.547] | 0.5304 |

Online 包含 commitment、PIOP、挑战相关的输入和查询转换、gather/cube audit、PCS adapter、完整 merged opening，以及 prover 函数返回时的临时数据释放。Offline witness/tuple/系数准备、dense compaction 和 owned q 拷贝在计时前完成；验证与序列化单独计量。未从融合阶段估算扣除 online 转换。

## 2^30 的阶段时间

| 阶段 | Flock ms | Mikros ms | M − F ms |
|---|---:|---:|---:|
| Commitment | 52.152 | 47.407 | -4.745 |
| Boolean 分支 | 314.133 | 328.585 | +14.451 |
| Wiring 分支 | 1174.559 | 451.930 | -722.629 |
| 并行 PIOP | 1174.630 | 451.991 | -722.639 |
| 查询适配 + production merged PCS | 280.418 | 267.208 | -13.211 |
| 逐样本外层剩余时间的中位数 | 12.386 | 7.846 | -4.540 |
| 完整 online prover | 1530.045 | 811.590 | -718.454 |

Boolean 与 wiring 在同一线程池并行，分支时间不能相加；各阶段独立取中位数，也不要求相加精确等于总中位数。剩余时间逐样本计算为 prover − commit − 并行 PIOP − PCS，再取中位数。PCS 时间包含在线 adapter，未单独测量每个内部核。

## 最大档的同版本模式消融

| 模式 | 系统 | Prover ms [min, max] | 并行 PIOP ms | Wiring ms | PCS ms | Tensor / ring / inner | 完整证明 B |
|---|---|---:|---:|---:|---:|---|---:|
| legacy | flock | 1649.253 [1571.472, 1659.618] | 1155.163 | 1155.073 | 404.056 | 11/11/1 | 487,810 |
| legacy | mikros | 1138.782 [1132.917, 1181.950] | 491.427 | 491.352 | 577.921 | 20/20/1 | 555,194 |
| cube | flock | 1495.584 [1472.373, 1514.873] | 1166.444 | 1166.364 | 252.820 | 4/4/1 | 458,866 |
| cube | mikros | 864.598 [818.225, 876.636] | 483.790 | 483.730 | 281.361 | 6/6/1 | 497,306 |
| lockstep | flock | 1530.045 [1500.835, 1543.031] | 1174.630 | 1174.559 | 280.418 | 4/4/1 | 458,866 |
| lockstep | mikros | 811.590 [769.390, 859.547] | 451.991 | 451.930 | 267.208 | 5/5/1 | 490,218 |

| 模式 | Mikros / Flock online 时间 |
|---|---:|
| legacy | 0.6905 |
| cube | 0.5781 |
| lockstep | 0.5304 |

flock：cube / legacy = 0.9068；lockstep / cube = 1.0230（均为总时间中位数之比）。

mikros：cube / legacy = 0.7592；lockstep / cube = 0.9387（均为总时间中位数之比）。

legacy 保留原 gather：Flock 3 个 Boolean + 8 个 wiring = 11，Mikros 4 个 Boolean + 两端各 8 个 wiring = 20。cube 先绑定每端完整 8 值表，保留原端点的加权重组检查，再用一次三变量随机 cube audit 将每端交给 PCS 的查询从 8 个降为 1 个，因此为 Flock 4 / Mikros 6。lockstep 将 Mikros 两棵树尾部共同归约到同一终端查询，再做 cube audit，得到 Flock 4 / Mikros 5；没有删除未认证的声明。

cube audit 的差多项式总次数至多 3，裸代数事件的界为 3/q（q = 2^128）；实现为此事件配置显式 2-bit PoW，全部 8 个表值及源声明先绑定，随后采样三个坐标。这只是该局部事件的参数说明，不是新的整体 128-bit 安全性定理。

这些是同一 v3 源码和 PCS 的模式比较，不是保持挑战不变的单核消融：v3 transcript 显式绑定 reduction mode，各模式和系统域不同，因此挑战不同。旧 v2 使用旧域与旧协议顺序，不能并入本表当作同 transcript 的 legacy 样本。即使 Flock 的 cube 与 lockstep 使用相同算法路径，域标签不同仍会改变挑战和具体工作量。

## 验证、证明长度与义务数量

| Witness bit-log | Flock verifier ms | Mikros verifier ms | Flock / Mikros 完整证明 B | Flock / Mikros tensor claims |
|---|---:|---:|---:|---:|
| 22 | 37.239 | 29.342 | 313,938 / 334,282 | 4 / 5 |
| 26 | 151.394 | 25.203 | 383,482 / 408,818 | 4 / 5 |
| 30 | 1232.463 | 49.145 | 458,866 / 490,218 | 4 / 5 |

原始日志累计 64 次定向 native 测试通过（跨测试命令的执行次数，可能包含重叠过滤器，不宣称互异测试数）。58 份实际证明全部完成验证及 transcript 终态检查：计时正式样本 39 份，预热 13 份，validation 6 份。同一输入、系统及模式下的 proof 摘要和长度在包括预热的所有重复中稳定。

| Validation 模式 | Flock 拒绝篡改项 | Mikros 拒绝篡改项 |
|---|---:|---:|
| lockstep | 19 | 23 |
| legacy | 24 | 34 |
| cube | 19 | 25 |

测试数与篡改拒绝数均从本次日志动态汇总，未套用历史固定计数。逐命令结果及全部模式的 verifier 范围见 analysis.json。上述 transcript 一致性是各证明的 prover/verifier 终态相等，不表示不同模式的挑战相等。

## Flock 原生 word-wiring 接口参考

| SHA compression 数 | 完整接口 prover ms [min, max] | verifier ms | 完整证明 B |
|---:|---:|---:|---:|
| 127 | 84.431 [74.514, 89.640] | 14.906 | 298,099 |
| 2,047 | 136.442 [130.815, 141.422] | 20.756 | 366,523 |
| 32,767 | 549.886 [512.913, 579.231] | 30.432 | 440,787 |

此列复用相同 PCS 与 commitment，但使用原 word wiring、默认 Boolean/lincheck、packed-direct gather 和原中间结果融合；完整接口还包含内部 assembly/compaction。因此单列报告，不能把它与逐 bit 主比较的差值全部归给 Mikros PIOP。

三个 prover 均配置 64 线程。原生 production 的 PIOP/PCS verifier 核心在自己的 1 线程 verifier_pool 中运行，外层公开矩阵检查仍可能使用全局池；共同 bit verifier 可以使用全局 64 线程。因此 production verifier 列不是同线程的速度比较。

## PCS 布局与实验边界

- SHA 声明行数为 2^nu−1，使用真实 partial-height 布局；没有通过修改公共 counts 或 padding 强行触发只适用于 full-height prefix 的 block-first 分支。
- 任意 128 列权重直接进入 weighted verifier；非零和行因子归一化后的 scale 并入列权重。d 个非零零和行因子通过 d+1 个固定节点的最高次系数插值分解，各分量均由相同 PCS 认证，并验证重组值。零 functional 只允许零声明。本次原始 tensor 数与实际 ring 数逐样本检查相等，最终内层 opening 数均为 1。
- Mikros lincheck 的 Laurent IP 输入为 F128，不能把独立 binary IP 快核的收益直接代入；Boolean 分支仍有两路公开矩阵下推。不同子协议的阶段差仅由实测定位，不把全部差值归因于声明数量。
- 两系统的 offline 表可同时驻留于同一测量进程。下表是 /usr/bin/time 的整个进程峰值 RSS，包含预处理、两系统、预热、验证及序列化，不是单系统 prover 内存。资源预检只覆盖启动前窗口，不能证明全程没有竞争。

| 测量进程 | 整进程峰值 RSS GiB |
|---|---:|
| common-leaves6 | 0.731 |
| common-leaves10 | 1.668 |
| common-leaves14 | 16.305 |
| ablation-legacy | 16.318 |
| ablation-cube | 16.308 |
| production-leaves6 | 0.628 |
| production-leaves10 | 0.884 |
| production-leaves14 | 3.156 |

- 初次 v3 构建失败：复用的 v2 Cargo 缓存与源码归档的零 mtime 导致 permutation 新接口未重编。随后依据 v2/v3 manifest 对变更的 11 个文件更新 mtime，保持源码内容及 SHA256 不变，触发重编后重新执行完整流程。初次失败是构建缓存问题，不计入协议验证失败或性能样本；results-build-cache-failure/、driver-build-cache-failure.log 和 cache-recovery.json 均保留并核验完整性。
- 重复样本采用固定 witness 和同模式确定性 transcript，测量运行波动，不代表跨输入总体统计。未增加 ZK masking，也未接入递归 verifier；公开矩阵与 sigma 义务仍由完整 verifier 检查。
- 源码上传包记录 SHA256：`a12caa90ee14b59abbd32f75bd1cf0cbd9de88872b67cc45f96f268b9d156011`；下载证据包 SHA256：`e28d6a4a2c546bbb942cf2804a1593be70621e7bb1b95746d9c78713f1eba16a`。逐文件完整性已核验；二进制前后摘要检查由已成功完成的服务器 driver 执行，本地保留其摘要记录。
- 独立指定的源码上传 SHA256：已核对。
- 服务器 UTC：2026-09-13T08:40:37Z 至 2026-09-13T08:42:43Z；所有记录阶段退出码为零。独立复算与 summary.json、ablation-summary.json、production-summary.json 逐字段完全一致。

运行 `python3 analyze.py` 可重新核验本目录完整证据并生成 analysis.json 和本报告；可附加 `--source-sha256` 指定独立保存的上传包摘要。缺少日志或检查失败时不会生成新结果。
