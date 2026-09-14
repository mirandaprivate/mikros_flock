# Mikros-128 与 Flock-128 的同压缩比实验

用户指定 `2^t = κ`。当前 Mikros 的 `κ = 128`，因此新增 Flock `t = 7`，保留原 `t = 6` 入口与历史数据。v1–v3 的 `flock` 均表示压缩 64；这些历史结果不能用来宣称 Mikros 在同压缩比下有一半的尾部向量。

## 实现与正确性

- 新增明确的 dense PIOP `prove_packed_skip7` / `verify_skip7` 入口。参数由 verifier 固定，使用独立 transcript domain；最小 `log_n = 14`。
- GF(256) 中 128 个 S 节点与 128 个 Λ 节点互不重复；A/B 行多项式次数至多 127，AB 次数至多 254。保留插值点落在节点上的分支。
- 复用三维 small + 四维 medium friendly constants、C 提取、查表、并行 split-eq、尾部折叠与容量池；扩展为真正的 128-lane GFNI/AVX-512 首轮及 16-byte 行的 VPCLMUL 融合折叠。
- 旧 skip6 API、PCS、s_hat_v 捕获、稀疏及 ARM 专用路径保持兼容。skip7 此轮仅为 dense binary PIOP，不连接旧 skip6 PCS。
- 对照独立 NTT/Lagrange/朴素 round1 和消息数学计算；测试输入两半、错误证明、错误参数与 transcript；保留旧 zerocheck/NTT 回归。benchmark 每个证明均核验 verifier、transcript 及三个原始输入声明。

## 正式测量

新的 `20260912T-native64/v4/` 保存源码、manifest、二进制和结果。沿用已授权服务器与目标父目录；不覆盖历史版本。

同一 native binary、64 线程、CPU `0-47,48-55,72-79`，双方启用实际可用 GFNI/AVX-512/VPCLMUL。NUMA 使用默认 first-touch，共享服务器无独占。

正式组为 `reuse,factoredreuse,flock128`，`N = 2^30`，每系统 1 次预热、6 次正式计时。rotate/reverse 顺序使每系统在每个位置出现两次。Mikros 两变体各自复用独立 scratch；Flock128 使用原生产容量池。所有依赖本次挑战的数据仍在 prove 内重算。

在主组完整结束后，另起 `parameter-sweep` 组测 `flock,flock128` 各 4 次，展示 Flock 多跳一轮的代价与收益。两个 Flock 参数在该组共用生产容量池，warmup 不是独立首次分配成本；这组不混入主组。主组不运行 Flock64，避免其较大缓冲区影响相同压缩比测量。所有 driver smoke 也在独立进程，退出后不影响主组的容量池。

附加独立三次 profile 与各系统单独冷进程 RSS，避免混入正式时间。warmup 的首次证明时间单独保存，不作为预热中位数。只比较同批样本，不用旧批次最快值替换基线。

`N = 2^30` 的预期逻辑消息量（不含 Vec 长度/nonce 数量编码）：Mikros 为 2236 个 F128 = 35776 bytes；Flock64 为 179 个 = 2864 bytes；Flock128 为 305 个 = 4880 bytes。benchmark 另记录 bincode 总长，由测试核验这些形状。

双方关闭 grinding，不能称具有相同 128-bit soundness；不包括 commitment、packed opening 或 PCS。相同压缩比下，双方初始尾部向量都为两条 `N/128` 的 F128 数组；整个 prover 的峰值内存仍须实测。

## 结果解释

报告同组中位数、范围、warmup、冷进程 RSS、PIOP verifier 和消息长度；解释首轮/折叠阶段边界不同。只有同压缩比的新测量完成后，才更新 Mikros 的速度和内存优势判断。即使某一批中位数略快，也不据少量共享服务器样本宣称稳定超过。
