# 64 线程 binary Hadamard 工程优化实验

此文是实验方案，不包含尚未测得的服务器结果。驱动是 `run_server_64.py`，直接在实验服务器上运行；不提供 SSH 或凭据操作。旧 N30 的 1/6/24 线程结果保留为历史记录。

## CPU 与内存策略

服务器为双路 Xeon Gold 6342，48 个物理核、96 个逻辑 CPU。固定 `taskset -c 0-47,48-55,72-79`：两个 NUMA 节点各选 32 个逻辑 CPU，总计 48 个物理核加 16 个 SMT sibling。驱动核对实际 `lscpu` 拓扑、允许 CPU 集合和每节点物理核数，不因资源不足而自动更换 CPU。

tests、native 构建、smoke、主计时、独立 profile 和 RSS 都继承同一个 CPU 集合。benchmark 的全局 Rayon pool 与 Flock all-core pool 均为 64 线程。CPU affinity 不等于 CPU 独占；这是共享服务器。所有系统使用 Linux 默认 first-touch 内存放置，不显式绑定或交错 NUMA 内存。输入生成、源输入检查与线程调度可能影响跨 NUMA 流量，应保留此限制。

记录 `lscpu`、CPU 拓扑、内存、load average、编译器、native target features、二进制 SHA256、source manifest 和 benchmark source fingerprint。驱动沿实际 cgroup-v2 leaf 到每个可见祖先记录 `cpu.max`、`cpu.stat`、`memory.max`、`memory.events` 和有效 cpuset，保留全程及逐命令的起止快照。祖先统计可能包含其他任务，不能归因于单次证明。cgroup namespace 若隐藏祖先，也不能据此断言不存在外部配额。

64 线程跨两个 NUMA 节点且使用部分 SMT，之前 24 线程只使用单节点物理核；两次结果不能解释成仅改变线程数的干净扩展实验。主要工程收益在同一新 binary 内比较 `legacy` 与新候选。

## 变体与比较边界

最终 CLI 名称以 benchmark 的实际实现为准；驱动按已协调的名称检查并透传：

| 系统 | 预期消融语义 |
| --- | --- |
| `lookup` | 保留旧逐行 LUT，仅在小尺寸 smoke 中作一致性对照 |
| `legacy` | 本轮优化前的 64-row tensor 和旧 lookup/IP 实现 |
| `stream` | 仅替换 tensor 的流式聚合路径 |
| `simdv1` | 保留的早期 GFNI 聚合候选，用于比较后续 GFNI 调度改进 |
| `simd` | 流式 tensor 加 GFNI 聚合，保留旧 packing/IP |
| `wide` | 在 simd 上加入 VPCLMUL IP，尚不融合 packing/首轮消息 |
| `fused` | 在 wide 上融合 packing/首轮消息，并采用新的 C 投影 |
| `native` | 在 fused 上加入分块 row weights，每次证明新建 scratch |
| `reuse` | 与 native 相同算法，跨证明保留 scratch 容量 |
| `factored` | 分层 row weights 的独立分支，每次证明新建 scratch |
| `factoredreuse` | 与 factored 相同算法，保留独立 scratch 容量 |
| `packed` | 默认最终优化路径；smoke 检查其与所有 Mikros 变体证明一致 |
| `flock` | Flock 原生产 degree-two zerocheck 路径 |

已核对 Kernel dispatch 与 benchmark `native_ablation_variants`：legacy→stream→simd→wide→fused→native 是累加优化路径，相邻变体对比才对应表中单项增量；不能把每行对 legacy 的总收益当作该单项的独立贡献。simdv1 是早期 GFNI 候选，factored 是分层权重分支，不按表格次序将它们擅自串入累加链。`native` 对 `reuse`、`factored` 对 `factoredreuse` 分别隔离各自 scratch 生命周期收益；两种复用变体必须持有独立 scratch，避免相互预热。主表同时展示选中路径的 fresh/reuse，保留 Flock 生产实现本身的 pool 复用；新 scratch 分配和动态构表都在 prove 内计时，不扣除 setup。冷进程 RSS 会包含每个变体的首次分配，不能以 warm reuse 时间代替 cold 时间。

各 Mikros 变体必须使用相同协议、参数、挑战域、输入、序列化格式和 proof digest。Flock 的协议不同，proof digest 无需与 Mikros 相同；Flock 自身同一输入和 run 的摘要须跨计时/profile/RSS 一致。所有证明保留 PIOP verifier、transcript 状态及全部源输入声明核验。输入生成和 digest、challenger 初始化、序列化、源输入 oracle 不在 prover/verifier 时间内，原始 JSON 仍保存 source-check 耗时。

比较范围为 binary F128 PIOP reduction；双方使用 BLAKE3、关闭 grinding，不包含 packed-opening reduction、PCS commitment/opening，不声称具有相同的 128-bit soundness。

## CLI 与执行矩阵

v3 在独立 run root 下放置经 manifest 核验的 `source/`。先执行 N24 的八变体消融，构建和测试只在此次执行：

```sh
python3 source/mikros/server_experiments/run_server_64.py \
  --root /path/to/run --results-name ablation \
  --systems legacy,stream,simdv1,simd,wide,fused,native,factored \
  --logs 24 --runs 16 --profile
```

driver 默认值仍为 `--systems legacy,native,reuse,flock --logs 30 --runs 8`，v3 按上述显式参数先消融。工具链固定 nightly-2026-03-21；先 `cargo test -p mikros --release --locked --offline`，再 native `cargo bench --no-run --locked --offline`，从 Cargo JSON 中选择唯一 benchmark executable。缺失依赖会停止，不修改 lockfile 或静默联网。smoke 后核对已记录的 GFNI、AVX-512F/BW/VBMI、VPCLMULQDQ、PCLMULQDQ 和 SSE4.1 build features，缺失则不进入主测量。

执行顺序是：

1. 64 线程 N20 smoke：本版本 driver 实际列出的全部变体，各一次预热和一次正式证明。变体集合随版本保留在 metadata/status 中。
2. 主计时：所选系统、每个 N 一次预热及 `--runs` 次正式样本；关闭 phase 计时。
3. 指定 `--profile` 时额外执行同系统/尺寸的三次诊断及一次预热；仅此过程设置 `FLOCK_ZC_TIMING=1` 并传 `--profile`。三个样本不保证位置平衡，不混入主计时表。
4. 每个所选系统、每个 N 各启动一个全新进程作 RSS：无预热、一次冷证明，关闭 phase 计时。`VmHWM` 在 prove 返回后、verify 和源输入检查前读取，包含输入、runtime、线程、表和工作区。

不固定写死总记录数。若 smoke 有 S 个系统、正式组有 K 个系统、D 个尺寸、R 次正式样本，则无 profile 时共 2S + DK(R+2) 份记录，加 profile 后再增加 4DK 份。最终以 `status.reports` 指向的真实样本/预热数量核对 `validated_proof_records`；不能把 smoke、预热、profile 和 RSS 都当作正式统计样本。

主计时逐 run 采用 benchmark 的 rotate/reverse 排列，并核对每系统处于每个顺序位置的次数相等。四系统八次满足；四系统四次不满足，驱动会拒绝。两系统六次也满足。均衡顺序降低固定位置偏差，八次样本不是统计显著性保证，不删慢样本。

消融完成后，用同一已测试且 hash 匹配的 binary 在 `results/` 执行本轮确定的正式四系统八次测量：

```sh
python3 source/mikros/server_experiments/run_server_64.py \
  --root /path/to/run --skip-build --results-name results \
  --systems legacy,native,reuse,flock --logs 30 --runs 8 --profile
```

主测量完成后，串行执行独立 candidate 组，补充比较分层权重与 native/Flock。它不是本轮正式路径的预选实验，样本不并入 results 中位数：

```sh
python3 source/mikros/server_experiments/run_server_64.py \
  --root /path/to/run --skip-build --results-name candidate \
  --systems native,factored,flock --logs 30 --runs 6 --profile
```

等 candidate 的 profile 与 RSS 也全部完成后，再串行执行独立的容量保留对照；这一组不启用 profile，仍保留全部源输入检查与逐系统新进程 RSS：

```sh
python3 source/mikros/server_experiments/run_server_64.py \
  --root /path/to/run --skip-build --results-name pooled-candidate \
  --systems reuse,factoredreuse,flock --logs 30 --runs 6
```

candidate 比较每次新建 scratch 的路径，pooled-candidate 比较保留容量的路径，各自包含 Flock。两组的样本、Flock 基线和计时过程分别保存，不跨组混合或用某组较快的 Flock 来重新计算另一组比值。两种复用内核使用独立 scratch，避免相互预热。

八变体十六次、四变体八次、三变体六次都保证每个顺序位置出现两次。candidate 的 Flock/本项比值只使用同组 Flock；组内没有 legacy 时不填 legacy 加速比。有限重复不能被描述成统计显著性保证。汇总器兼容今后另行正式测量 `legacy,factored,factoredreuse,flock` 的数据，但不由本轮候选结果擅自替换正式结果。

`--skip-build` 读取 root 下 `build_artifact.json`，要求原 tests/build 成功记录、toolchain、CPU 集合、manifest 和 executable SHA256 完全匹配；它不声称重新运行了测试。结果目录必须为空，使用新 `--results-name` 保存每次执行，禁止覆盖已完成或失败记录。`--logs` 支持 13 到 30 的不同整数，较大输入需另作资源计划。

## 保存与验收

默认 `results/` 内包含 `server_environment.json`、`commands.json`、`status.json`、`effective_cgroups_start.json` 和 `effective_cgroups_end.json`；每个 stage 都有 JSON 和原始 log。主表原始文件是 `comparison-64t.json`，诊断是 `profile-64t.json`，RSS 是 `rss-<system>-n<log>-64t.json`。服务器本地写日志，完成后再传回，避免把远程或挂载盘日志延迟计入诊断。

验收检查正式样本/预热覆盖、执行顺序、摘要格式、每份 proof shape、正且有限的时间、summary 中位数/min/max、profile phase 总和、RSS 形态。跨全部报告核对源代码 fingerprint、同 N/run 的输入摘要和同协议 proof 摘要。所有命令须零退出，末尾重新核对 manifest 和 executable SHA256，全部通过后才写 `status.stage=complete`。

下载后的 `summarize_native64.py --run-dir /path/to/20260912T-native64 --final-version v3` 扫描每个版本下 status=complete 的直接子目录，并读取 status 的真实报告清单；results、ablation、candidate、pooled-candidate 分组保留。还核对 baseline/profile-64t 和 v1/pilot-n30 等开发记录。同版本 fingerprint 必须一致，版本之间允许改变源码，但同 N/run 的输入和 Mikros 各内核/Flock 各自 proof digest 必须一致。每版本 server_environment 必须匹配其本地 source_manifest。正式 results/ablation、两个指定候选组和开发记录未完整下载时，汇总器拒绝生成最终 REPORT.md/summary.csv；同时核对 results→candidate→pooled-candidate 的起止时间，防止并行负载混入本轮对照。

结果报告应按尺寸列每变体 prover 中位数与 min/max、相对 legacy 的工程加速、与 Flock 的耗时比、verifier、消息 bytes、独立冷 RSS 和 source-check 外部耗时；profile 单列均值和阶段，不混入主中位数。解释 scratch 复用收益、NUMA/SMT 和共享服务器的限制。若最终 `packed` 的选择不同于已正式测量的 native/reuse 或 factored/factoredreuse 路径，应补测其正式样本，不能仅由 smoke 推断其性能。旧 binary 的 baseline64/profile 只作先导诊断，不能混入新 source fingerprint 的配对主表。
