# 服务器 N = 2^30 实验

新单变量协议使用独立驱动 `run_univariate_64.py` 和 `univariate_vs_flock` benchmark，默认 N30、64线程、`reuse,flock128` 各六轮，并分别测profile/RSS。它核验GFNI/AVX-512编译特性、67项以上测试、全部proof/source声明、源码和二进制摘要。新结果目录为 `20260912T-native64/v5-univariate/`，下方v1–v4均为旧mixed-tensor协议。不要将旧驱动的消息长度校验直接套到新协议。

[v5单变量报告](20260912T-native64/v5-univariate/REPORT.md) 已完成：32份证明全部核验，315个源码文件和23个下载证据文件摘要一致；六轮Mikros/Flock128中位数98.41/90.29ms。复现汇总：`python3 mikros/server_experiments/summarize_univariate.py --root mikros/server_experiments/20260912T-native64/v5-univariate`。该报告与下方旧协议结果独立。

最新完成 [v4 同压缩比报告](20260912T-native64/v4/EQUAL_COMPRESSION_REPORT.md)：`2^t=κ=128`，Flock 跳过7轮，双方 native GFNI/AVX-512/VPCLMUL、64线程。主组 `reuse,factoredreuse,flock128` 各六次，另有独立 profile/RSS；Flock64/128 参数对照在另一组进程各四次，避免较大缓冲池影响主组。分析见 [v4 FINDINGS](20260912T-native64/v4/FINDINGS.md)。下方 v1–v3 的 Flock 均为压缩64历史数据。

当前 `run_server_64.py` 默认同压缩比三系统、六次；旧运行命令和驱动版本保存在各版本源码包内。v4 汇总命令：

```sh
python3 mikros/server_experiments/summarize_equal_compression.py mikros/server_experiments/20260912T-native64/v4
python3 mikros/server_experiments/summarize_equal_compression.py mikros/server_experiments/20260912T-native64/v4 --results-name parameter-sweep
```

此前完成 [64 线程 native 优化报告](20260912T-native64/REPORT.md) 和 [瓶颈分析](20260912T-native64/FINDINGS.md)：同一 v3 binary、GFNI/AVX-512/VPCLMUL，N24 八变体十六次消融、N30 四系统八次主测量、两组六次分层权重补测，以及独立阶段/RSS。源码、原始数据和下载校验清单保留在各版本目录；图为 [SVG](20260912T-native64/native64_comparison.svg) / [PNG](20260912T-native64/native64_comparison.png)。

64 线程的 CPU/NUMA、scratch、顺序均衡和完整执行命令见 [SERVER64_PLAN.md](SERVER64_PLAN.md)，驱动为 `run_server_64.py`。下载的数据可重新校验并生成报告和图：

```sh
python3 mikros/server_experiments/summarize_native64.py --run-dir mikros/server_experiments/20260912T-native64 --final-version v3
python3 mikros/server_experiments/plot_native64.py --run-dir mikros/server_experiments/20260912T-native64
```

两边都保留容量的 `reuse` / Flock 对照与每份证明新建 scratch 的 `native` 分开报告；分层候选也与正式主实验分组保存。48 个物理核加 16 个 SMT sibling、跨 NUMA 的配置不同于下面的历史 24 物理核配置。

已完成：[2026-09-12 Xeon Gold 6342 的 1／6／24 线程报告](20260912T065538-n30/REPORT.md)。原始结果保留于该实验目录的 `results/`；源码快照与上传、下载文件的校验摘要均保留。

`run_server_n30.py` 在 Linux 服务器原生编译当前源码，运行正确性测试、小尺寸差分检查，再串行执行 1、6、24 线程的 Packed Mikros / Flock 对照。每组正式计时包含每系统一次预热和六个样本；每个系统/线程另开进程测量冷启动峰值 RSS。

驱动在当前允许的 CPU 集合中选择一个 NUMA 节点，每个物理核只取一个逻辑 CPU。需要该节点至少有 24 个可用物理核。CPU 通过 `taskset` 限定；内存使用 Linux 默认 first-touch 策略，没有显式绑定。此配置不代表服务器独占。

源码包通过 `package_source.py` 的文件白名单生成，带逐文件 SHA256 清单，不含凭据、`.git`、构建产物或其他项目目录。实验使用 `nightly-2026-03-21`、`Cargo.lock` 和仓库的 `-C target-cpu=native`，应在测量服务器重新编译。

```sh
# 在仓库根目录打包；为每轮实验指定新的目录。
python3 mikros/server_experiments/package_source.py mikros/server_experiments/RUN_ID

# 将获得授权的源码包上传并校验 SHA256，解压到服务器 RUN_ROOT/source。
# 若依赖缓存不完整，先在该 source 目录补齐锁定依赖：
cargo +nightly-2026-03-21 fetch --locked

# 在服务器执行；RUN_ROOT 是包含 source/ 的绝对路径。
python3 RUN_ROOT/source/mikros/server_experiments/run_server_n30.py --root RUN_ROOT

# 将服务器 results/ 下载至本地对应实验目录后生成报告。
python3 mikros/server_experiments/summarize_server_n30.py --run-dir mikros/server_experiments/RUN_ID
```

驱动校验源文件清单，保存 `server_environment.json`、实际命令、执行时间、退出状态及逐样本结果。`status.json` 的 `complete` 表示所有步骤结束；构建、测试或证明检查失败时立即停止。应使用新实验目录，避免覆盖已有结果。

统计范围沿用本地工程优化对照：包含所有在线 reduction 工作及动态构表；不含输入生成、序列化和源输入声明核验。源输入声明仍逐一检查并单独记时。双方均未包含 PCS，grinding 关闭，不声称相同安全等级。RSS 在 prover 返回后、verify 和源输入检查前读取，包含输入和 runtime，不能解读为纯算法工作区。
