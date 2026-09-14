# Mikros binary permutation v5: outcome

进一步优化有效，主要来自有预算上限的树缓冲复用。单独的 AVX-512
寄存器融合没有显示收益，仍保留为实验选项。代码位于独立的 `binary_v5`
crate；冻结的 binary v4 和 Flock 是本轮实际重新运行的比较基线。

用户随后明确：**offline 转换排除，online 转换计入**。按该口径，
下面的“含原生输入转换”只作历史辅助数据。现有 core 已包含全部
online 转换，但还混有可离线处理的紧凑系数、解码和 Flock 静态 σ 表，
因此不能把本页数字或加速比例改称严格的纯 online 结果。分类、源码
依据及待重测边界见 [offline/online 计时口径](OFFLINE_ONLINE.md)。

## N = 2^30，64 threads

下表为同组 6 次正式样本的中位数，每系统另有 1 次预热。`combined`
在 N26 消融后按预定的平均 core 最小规则选定，随后独立运行 N30。

| 指标 | Mikros v4 | v5 combined | Flock |
|---|---:|---:|---:|
| Prover core | 9.371 s | **6.384 s** | 12.126 s |
| 含原生输入转换 | 10.889 s | **8.077 s** | 13.280 s |
| 再含 verifier 与完整源检查 | 12.933 s | **10.084 s** | 13.901 s |

v5 core 比 v4 少 **31.88%**，比 Flock 少 **47.35%**；含转换分别少
25.82% 和 39.18%。以上三个指标对两条基线都是 6/6 配对胜。表内排除
批末清池；这些是 PIOP 测量，未包含 PCS、承诺、opening 或 grinding，
也不建立协议整体安全性相同的结论。N 是 permutation item 数，数值为
1 bit，公开位置标签为 30 bits，Mikros 用 K32 编码。Flock 使用冻结的
batched Product-GKR 接口，该接口没有 Hadamard 的 skip 参数。

## 收益与剩余瓶颈

同次 N30 正式样本的平均阶段时间：

| 阶段 | v4 | combined |
|---|---:|---:|
| 首层构造/求值 | 2.465 s | 2.355 s |
| 扩域上层建树 | 0.347 s | 0.343 s |
| GKR 消息、折叠与 equality | 3.453 s | 3.580 s |
| 树层同步释放/归还池 | 2.907 s | 0.000030 s |

树池避免每份证明都同步释放大层，随后真正复用分配：主组保留 30 个
缓冲，共 **31.998 GiB**，后 6 次证明产生 **180 次命中**。两种复用
候选的树池互相独立，各有 32 GiB capacity 预算、32 个缓冲上限和
1 MiB 门槛；它们与旧/新 Mikros 及 Flock 已有的共享 scratch 池分开。
缓冲命中只复用存储，每次都完整重写数学值。

寄存器融合确实省掉 Separate 内核折叠输出的一次重读，但没有减少实测
时间。N26 register-only core 中位数为 823.585 ms，旧版为 800.190 ms，
慢 2.92%，仅 1/8 配对胜。combined 对 reuse 的 N26 平均 core 略低
（601.991/607.128 ms），但中位数反而慢 1.65%，仅 3/8 胜。因此 N30
的 combined 结果不能用来宣称寄存器部分有效，也不能宣称 combined
稳定胜过仅复用路径。N30 combined 的 fold/message 阶段比 v4 多约
0.125 s。当前主要计算热点仍是 GKR，其次是首层构造。

服务器机器码显示 Separate 热循环无 ZMM 栈溢出，也无输出重读；块外
仍存在栈帧、累计器写回和横向归约。当前导出的函数没有覆盖被调用的
Interleaved/Compact dense 热循环，未采集硬件计数器，因此不能把回退
定量归因于寄存器溢出、缓存或 NUMA。

## 内存和清理成本

| 独立进程状态 | v4 峰值 RSS | combined 峰值 RSS | Flock 峰值 RSS |
|---|---:|---:|---:|
| 冷：1 份证明 | 49.740 GiB | 63.759 GiB | 158.001 GiB |
| 暖：1 预热 + 1 正式 | 63.736 GiB | 63.759 GiB | 164.000 GiB |

暖进程的峰值包含预热；每格只来自一个独立进程。证明返回后的 RSS
约为 v4 31.760 GiB、combined 63.759 GiB。树池保留容量不能解释成
已经释放的内存，也不是工作中所有分配的总上限。

冷 combined 树池清空 **3.378 s**，连同共享 scratch 清空共 **4.447 s**；
v4 的批末清理为 1.495 s。冷 combined 没有树池命中，所以不能把其
停表前的全部改善当作分配复用加速。计入输入生成、转换、证明、全部
检查、证明输出和最终清理的冷批次总墙钟为 v4 **19.297 s**、combined
**17.628 s**；这是单样本结果，不能保证稳定的冷启动加速。暖独立
进程的两份证明及全部上述成本合计为 36.177/31.459 s。

## 使用与验证

通过现有 `permutation::prove_profiled` 显式选择：

- `Engine::Recycled`：仅树缓冲复用，适合评估重复证明的工程收益。
- `Engine::Combined`：本轮 N30 实测候选，包含实验性寄存器融合。
- `Engine::Registers`：寄存器融合消融，当前不作为加速建议。

`permutation::prove` 默认仍为 `Engine::Fused`。用
`tree_pool::stats(0/1)` 查看两池状态；批次结束调用
`tree_pool::clear_all()` 同步清理树池。现有 Flock scratch 池独立，
需要完整清理时另调用其 `scratch::clear()`。

本地和服务器各通过 17 项测试。服务器所有 71 份证明均通过 verifier、
完整源检查及 transcript 核验；所有 Mikros 变体保持相同证明字节。
N30 Mikros/Flock 序列化证明仍为 44,208/16,176 bytes。415 个源码/构建
文件、110 个下载证据文件哈希已核验，9 组实验无被排除的干扰尝试。
64 threads 使用 48 个物理核心加 16 个 SMT sibling；编译启用服务器
native GFNI/AVX-512/VPCLMUL 等特性，新增融合内核实际使用 AVX-512
和 VPCLMUL。主机共享且不独占。

详见 [完整报告](server_experiments/20260912T-permutation-v5/REPORT.md)、
[原始统计](server_experiments/20260912T-permutation-v5/SUMMARY.json)、
[预先选择记录](server_experiments/20260912T-permutation-v5/selection.json)、
[独立审计](server_experiments/20260912T-permutation-v5/AUDIT.md)。
冻结汇总脚本需 Python 3.11+ 解析 ISO UTC `Z`；本地实际用 Python
3.12.14 生成报告，原源码未修改。
