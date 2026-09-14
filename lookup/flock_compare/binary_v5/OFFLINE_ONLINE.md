# Offline / online accounting

用户指定的主比较口径：offline 转换不计入；online 转换必须计入。
这里假设本次输入和公开置换已经固定，以挑战产生前能否完成准备工作
区分两阶段。仅仅把操作命名为 conversion、packing 或 lookup，不能
成为排除计时的理由。

## 当前源码中的边界

| 操作 | 应属阶段 | v5 实际计时位置 |
|---|---|---|
| Mikros 值/位置拼接、PackedBits 转置、statement digest | offline | prepare_ms |
| Flock f/g 的 u8 → F128 固定域嵌入 | offline | prepare_ms |
| Mikros 重复源数据/digest 检查 | 可在挑战前检查固定输入 | prover_core_ms |
| Mikros K32 解码、紧凑二次多项式系数构造 | 可 offline | prover_core_ms，融合内核中 |
| Mikros 依赖 z 的求值查表、求值、扩域建树 | online，必须计入 | prover_core_ms |
| Flock sigma → s_sig_vec 静态域元素表 | offline | prover_core_ms |
| Flock 依赖 alpha/beta 的 fingerprint、建树 | online，必须计入 | prover_core_ms |
| 两方挑战驱动的 GKR 折叠、消息、最终声明求值 | online，必须计入 | prover_core_ms |

证据：`src/bin/compare.rs:89–108,178–183,277–291`；
`src/permutation.rs:185–197,227–229`；`src/fast_tree.rs:294–311`；
`src/field.rs:276–286`；
`../snapshot/crates/flock-core/src/product_gkr.rs:1572–1577,1611–1649,1786–1798`。

Flock 的静态 s_sig_vec 构造可离线，但最终 s_sigma(rho) 依赖 GKR 的
rho，仍属于 online。Mikros 的二次多项式系数可离线，但其在随机 z
下的求值表及求值必须 online。当前 fused_leaf_build_ms 同时包含
这两部分，不能整段扣除。固定域算术表可以预建，挑战相关查表不能
沿用前一份证明的随机值。

## 现有数字应如何使用

`prepare_ms` 内两方均没有挑战相关转换，按用户口径排除。
`prover_total_ms` 含该 offline 准备，不作为 online 主指标。
`prover_core_ms` 包含所有 online 转换，但也包含上述尚未拆分的可离线
工作。原 N30 中位数 9.371 / 6.384 / 12.126 秒继续表示 v4 / combined /
Flock 的**当前 prover 调用时间**，不能标为已经彻底剔除 offline 的
纯 online 比较，原加速比例也不能直接移作该新口径的加速比例。

严格 online 数字需要双方提供明确的预处理输出，在计时开始前构造
离线存储，再对挑战相关步骤及协议执行整体计时。旧数据没有足够细的
独立计时，不能事后通过扣除融合阶段、减中位数或估算访存时间获得。
该严格分离实验尚未执行；本文件是计费规则和源码审计记录。

预处理输出的驻留内存应纳入实际内存统计；offline 时间排除不意味着
离线存储免费。同步清理、批末清池和长期复用的成本仍按 RESULTS.md
单列；完整 verifier/source 检查也保持独立报告，不能因其含格式转换
而删除挑战相关检查。冻结的源码、原始实验记录及报告保持原样。
