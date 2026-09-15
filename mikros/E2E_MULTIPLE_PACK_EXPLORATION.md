# Multiple packing 与 PCS 声明压缩：代码核对和候选方案

后续测试已完成：原生 mixed packed-direct 路由已实现，并修正了其列内串行扫描瓶颈。
N26/64threads multiple 中位数为 Mikros 18.22 s、FLOCK 23.16 s；逻辑声明数仍为19/10，
减少的是 ring-switch 工作。实现、负结果和最终证据见[实测报告](C:/Users/miranda/git/mikros_flock/e2e_n26_packed_direct_compare_flock.log)。
下文保留最初基于 v6 的探索记录；随机合并声明和同步两棵树仍是未实现的候选方案。

原探索记录（2026-09-15，基于 v6）：范围为 multiple E2E，128 个相同 wiring 的独立输入，沿用 FLOCK 原生 commitment 和 Ligerito。当时仅完成探索，尚未修改生产协议或运行新 benchmark。

**可以减少逻辑声明，也发现了一个比单纯减少条数更值得先实现的路径：把普通 packed-F128 MLE 声明送入已经存在的 `PackedDirectClaim`。** 当前适配器没有使用这一入口。

## 当前到底打开了多少

| Multiple | Gate witness | Copy witness | Public selectors | Public labels | 完整 PCS opening proof |
|---|---:|---:|---:|---:|---:|
| Mikros | 3 ordinary + 2 Hadamard | 6 ordinary | 5 | 3 | 2 |
| FLOCK | 4 gate functionals | 3 ordinary | 0 | 3 | 2 |

Mikros 的 11 witness + 8 public 是 **19 条逻辑声明**，并非 19 份 PCS proof。两者已经分别对 witness、public 各做一次 merged opening。

当前 v6 multiple trace 中，Mikros witness 的 11 条声明已经复用为 5 个不同 gate-row points；public 的 8 条复用为 2 个。FLOCK 对应为 5 和 1 个。因此不能按 11→5 或 8→2 的比例预测耗时。

来源：[gate 声明构造](C:/Users/miranda/git/mikros_flock/mikros/examples/e2e_plonk/gates.rs:32)、[copy 声明构造](C:/Users/miranda/git/mikros_flock/mikros/examples/e2e_plonk/copy.rs:600)、[两次 opening](C:/Users/miranda/git/mikros_flock/mikros/examples/e2e_plonk_compare.rs:243)、[相同行点分组](C:/Users/miranda/git/mikros_flock/crates/flock-core/src/pcs.rs:2291)。数据来自 `mikros/server_experiments/20260914-plonk-e2e/v6/evidence/profile-n26/`。

## 第一优先级：复用原生 PackedDirectClaim

对已打包的 wire，有：

\[
\widehat w_\ell(x)=\sum_i\chi_x(i)\widehat w_{\ell,i}
=\sum_{i,j}\chi_x(i)\beta_j W_{\ell,i,j}.
\]

这本来就是 packed words 上的普通 MLE，不必重新按 128 个 bit columns 做 ring-switch。代码中 `PackedDirectClaim` 明确提供了这条路径；`open_batch_merged` 支持它与任意 bit-linear functionals 混合，verifier 也有对应入口。

**当前遗漏点：** `merged_tensor::open_impl` 和 `merged_tensor::verify` 都给原生入口传了空的 `packed_direct` 列表，因而所有声明仍走 ring-switch。

| Witness 路由 | 当前 | 建议的混合路由 |
|---|---|---|
| Mikros | 11 条 ring-switch | **9 条 packed-direct + 2 条 Hadamard ring-switch** |
| FLOCK | 7 条 ring-switch | **3 条 packed-direct + 4 条 gate ring-switch** |

逻辑声明数在这一步不变，实际需要 ring-switch 的 Mikros witness 声明从 11 降至 2。9 条 direct 可以复用 gate、两个 copy terminal 共 3 个 scalar row groups。FLOCK 的 3 条 direct 共用 1 个 copy row group。对双方使用同一个精确分类器，才能公平比较；这不是关闭 FLOCK 已有能力。

分类条件应检查 **全部 128 项**：`columns[j] == lambda * beta[j]`。在 `merged_tensor` 已归一化的 component 上，非零 `lambda=columns[0]`，原 component value 除以 lambda 后，得到 `x_outer[1..]` 处的普通 packed MLE。零查询沿用已有检查；不符合条件的列向量保留原路径。不得只检查第一项、不得把任意 rho 当成 beta。

仍使用同一个源数组、commitment、ProverData、PCS 参数和 Ligerito；不需要额外承诺，也不需要在线转换整个 witness。普通声明改走已有 scalar maps，可以省掉相应的 bitbank 和通用 F2-linear 映射处理。merged sumcheck、编码/承诺和 inner opening 的共同成本仍存在，不能据 11→2 推断整个 PCS 提速 5.5 倍。

这一改动保留 Mikros 的 gate/copy PIOP 及其终端声明，**会改变外层 PCS 证明和挑战流**。验证目标应是原 PIOP transcript、终端求值、commitment 不变，以及 native mixed verifier 接受；不能要求完整 proof bytes 与旧适配器一致。

来源：[已有直接入口](C:/Users/miranda/git/mikros_flock/crates/flock-core/src/pcs.rs:150)、[混合 prover](C:/Users/miranda/git/mikros_flock/crates/flock-core/src/pcs.rs:2509)、[混合 verifier](C:/Users/miranda/git/mikros_flock/crates/flock-core/src/pcs.rs:3169)、[适配器空参数](C:/Users/miranda/git/mikros_flock/crates/flock-core/src/pcs/merged_tensor.rs:274)、[scalar 分组](C:/Users/miranda/git/mikros_flock/crates/flock-core/src/pcs.rs:2684)、[scalar assist](C:/Users/miranda/git/mikros_flock/crates/flock-core/src/pcs.rs:2856)。

## 第二层：只认证 verifier 实际需要的 public 组合

Gate terminal 只使用：

\[
S(x)=v_Lq_L(x)+v_Rq_R(x)+v_Mq_M(x)+v_Oq_O(x)
       +\Bigl(\sum_j\beta_j\Bigr)q_C(x).
\]

三个普通 wire 值和虚拟乘积值分别经 witness PCS、Hadamard 认证后，可把上述 S 作为一条 public functional：相同行权重，128 列权重的前五项为 `[vL,vR,vM,vO,sum(beta)]`。这认证最终残差所需的关系，不再声称五个报告的 selector 值分别正确。它不是用固定系数合并五个本来必须分别成立的等式。

同理，右侧 copy terminal 只需要三个带 wire-prefix 权重的 label 值之和。其三条查询同 row point、不同 column functional，可以将列权重与目标分别相加，得到一条 public 查询。

所以一般电路的 **public 8→2** 有明确的终端关系依据，不需要新增随机 packing。但需要修改 terminal-authentication 契约及其论证，PCS proof/transcript 也会变化。native row grouping 已复用相同行点，实际速度收益必须测量。

当前固定 alternating AND/XOR 电路还有更强的特例：selectors 为 `(x0,x0,1+x0,1,0)`，verifier 可直接计算并检查，不需要 selector PCS 查询。加上 label 合并后 public 可剩 **1 条**。这不能推广为任意 PLONK 电路都不需要 selector openings；公平比较也应允许 FLOCK 合并它的三条 label 查询。

来源：[终端 gate 残差](C:/Users/miranda/git/mikros_flock/mikros/src/multiple_gate.rs:120)、[固定 selectors](C:/Users/miranda/git/mikros_flock/mikros/examples/e2e_plonk/gates.rs:77)、[copy 最终使用的和](C:/Users/miranda/git/mikros_flock/mikros/examples/e2e_plonk/copy.rs:618)。

## 第三层：随机 block 合并，通常把 witness 11→5

128 次执行已用固定基 beta 打包，不能再把多个任意 F128 求值无损塞进一个 F128。例如在特征二中，`beta0*beta1 + beta1*beta0 = 0`，非零的一对扩域系数已经可映射到零；基只对底层 F2 系数线性独立。

可以做的是 **在值绑定后，随机合并同一个 gate 行点上的三个 wire 声明**。承诺布局为 `[WL,WR,WO,pad]`。Gate 部分先绑定三个值，再采新 s∈F128²，检查：

\[
\widetilde W(x,s)=\sum_{\ell=0}^{2}\chi_s(\ell)v_\ell.
\]

这同时将 padding 在 x 处的声明设为 0；不是假定恶意 prover 的 padding 天然为零。四个块的误差是独立 multilinear basis 的系数，错误组合为总次数至多 2 的非零多项式，所以随机检查的额外错误概率至多 `2/|F128|`。

Copy 的现有值已经包含树终点 p 的前缀：`t_l=chi_p(l)*W_l(x)`。无需求逆或拒绝 0/1 挑战：先绑定三个 t，再采新 s，把新查询最后两维设为：

\[
[(1+p_i)(1+s_i),\ p_i s_i],\qquad i=0,1,
\]

目标为 `sum_l chi_s(l)*t_l`，仍保留原树的 p 做叶子重组。不得在看到值后复用已知挑战，也不得用新 s 替换树的旧挑战。

三组普通 wire 声明——gate 一组、两个 copy terminal 各一组——因此从 9→3；两个 Hadamard terminal 的行权重通常不同，仍保留 2 条，合计通常 **5 条 witness**。再配合 public 残差组合，得到 **5W+2P=7 条逻辑声明**。与 direct routing 可以组合，但 native scalar grouping 已能复用这些普通声明，所以逻辑减少未必进一步减少源扫描。

保留原消息的版本仍需发送并绑定原来的九个普通 wire scalars；这里压缩的是交给 PCS 的认证关系条数，并不是把这些扩域消息无损压成三个元素。

Copy 的新行因子可能产生非零 zero-sum pair，现 `merged_tensor` 用 d+1 个 component 处理；不能承诺所有挑战下 native component 数都恰为 5。三个随机合并的保守附加错误界为 `6/|F128|`，还要加原 PIOP/PCS 的错误；这不是完整安全定理。

如果不新增随机挑战、也不额外认证 padding，当前三段 mask `[chi_p(0),chi_p(1),chi_p(2),0]` 一般不能写成两位 rank-one tensor。因此每棵 copy 树安全的确定性压缩是 3W→2W，而非直接1W。

来源：[低位优先且允许非归一化 rows 的 TensorClaim](C:/Users/miranda/git/mikros_flock/crates/flock-core/src/pcs/functional.rs:32)、[带前缀的 wire 查询](C:/Users/miranda/git/mikros_flock/mikros/examples/e2e_plonk/copy.rs:456)、[退化因子适配](C:/Users/miranda/git/mikros_flock/crates/flock-core/src/pcs/merged_tensor.rs:68)。

## 更进一步：同步两棵 copy 树

当前两棵树顺序归约，留下两个独立行点。若每一轮先绑定两边消息，再采同一个 sumcheck 挑战；每层末先绑定两边 children，再采共同 merge 挑战，分别验证双方等式，就能使两个终点相同。

它们可以共用同一组 wire 声明。结合上面的随机 block 合并，copy 可从 2W+1P 进一步到通常 1W+1P；整个 generic multiple 因而有 **4W+2P** 的研究目标，并有机会真正把 witness 不同行点从 5 减到 4。两树消息必须先共同绑定，不能在第一棵树完成后让第二棵使用已公开挑战。

这会修改 product-tree 协议、消息顺序和 verifier，且两树当前层可能需同时驻留，影响刚实现的跨树顺序 buffer 复用。不能只凭声明变少断言更快。当前 FLOCK 已有联合 product-tree 思路，因此这一步应比较增量收益及额外内存。

来源：[当前顺序 prove_pair](C:/Users/miranda/git/mikros_flock/mikros/src/extension_product.rs:688)。

## 实施优先级与本次验证范围

1. 先接通原生 mixed packed-direct 路由，原 PIOP 不变，双方使用相同分类器；小规模 native proof 验证后，再测 N26 的 RS、merged sumcheck、assist、inner PCS 和 E2E。
2. 再做 public 残差/标签组合；固定 selector 解析检查作为独立的 fixture 专用配置。
3. 随机 block 合并和两树同步作为协议级实验，单独推导安全界、计量内存，不混入此前 proof-byte-equivalent 的工程优化记录。

本次完成代码与 LaTeX 的交叉审查，并运行独立代数脚本：[check_pack_claims.py](C:/Users/miranda/git/mikros_flock/mikros/server_experiments/20260914-plonk-e2e/n26-analysis/check_pack_claims.py)。227 项检查通过，包括 GF(2^128) 下 scalar-column/direct 等式及前缀随机合并，GF(16) 穷举挑战对中最大根数 31/256，符合 degree≤2 的界。**这不是原生 mixed PCS 验证或性能测量；上述新方案尚未实现、未给出新的加速数字。**
