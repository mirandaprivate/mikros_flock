# Mikros permutation + Flock Binius/Ligerito PCS

优化实现见 [optimization/README.md](optimization/README.md)：包含分阶段对照、
PCLMUL/查表、合并 GKR、并行化及系数域归约。优化 crate 的默认路径为
`FullOptimized`；本目录原入口继续保留为可复现基线。

本目录为独立 Cargo workspace，提供 `mikros_binius_permutation::{prove, verify}`。
使用 Flock 的 **Binius ring-switching + Ligerito** PCS，认证现有 Mikros permutation
PIOP 和 Flock 原生 `product_gkr::prove_batched` 的全部末端声明。
构建、源快照和结果均留在本目录，不修改原协议实现或已有 WHIR 对照。

## 相同语句与承诺

两边证明相同关系 `f[sigma[i]] = g[i]`，其中 sigma 是公开的真正排列。
输入为 `2^log_n` 个 `value_bits` 位整数，目前支持 `log_n=15..20`、`value_bits=1..64`。
两边都承诺 f、g 的原始 column-major bit 矩阵：连续 128 bit 打包为一个 F128，
采用 Flock 的交织 additive-NTT/RS 编码及 BLAKE3 Merkle cap。
位宽补齐到二次幂，额外位列固定为零；承诺实际保留 bit packing。

- Mikros 调用原 `Strategy::BitwidthAware`，将 `(f[i],i)` 和 `(g[i],sigma[i])`
  编码成 `value_bits+log_n` 位的无损元组。公开 tag 部分由验证器计算并从末端声明中分离。
- Flock 调用未经修改的原生 batched product-GKR 和 sigma-aware verifier，
  f、g 使用既有 `SmallField` 单射嵌入 F128。随机 value/position fingerprint 保持原样。
- 两边只有原始值需要承诺，公开 sigma 不重复承诺。每个样本检查两边 statement/cap 的字节完全相同。

## 末端认证

末端声明为 `sum_b basis[b] * bit_column_b(row_point) = value`。
证明者发送全部位列在 row_point 的求值，验证器检查加权和及补齐列的零值。
这些求值吸收进 transcript **之后**才抽取随机列坐标，将它们归约到一次 bit-MLE opening。
该 opening 通过 Flock 现有的 ring-switching + recursive Ligerito 进行认证。
在 ring-switch 前六位传入普通 `eq` 权重，其余坐标传入 `x_outer`；没有假装成 univariate-skip 点。

源为 F128 的完整 F2 坐标分解，bitness 由打包表示给出。
补齐列的求值在 PIOP 随机行点检查为零，并由同一次 PCS opening 认证。
随机列归约额外贡献至多 `log2(padded_width)/|F128|` 的代数误差；还需计入零 padding 检查、
permutation fingerprint、GKR、ring-switch 和 PCS 的误差。

验证器仅接收公开 `Statement`、`Proof`、期望系统及应用 context，不接收 f/g。
所有挑战之前先绑定两份 cap、参数、位宽和 sigma。调用方必须持有期望的 Statement；
不得把攻击者另选的承诺当成原任务的承诺。Mikros 原 API 的源摘要保留为 transcript 元数据，
这里的认证语义以 PCS commitments 为准，并不另行证明原哈希 preimage 关系。

```rust,ignore
use mikros_binius_permutation::{prove, verify, System};
let out = prove(&f, &g, &sigma, 8, System::Mikros, b"my-application")?;
let (verification_times, _) = verify(
    &out.statement, &out.proof, System::Mikros, b"my-application")?;
```

## 参数与测量范围

每种形状的两边使用相同 PCS 参数：L0 码率 1/2、BLAKE3、F128 编码和 F256 Ligerito
折叠算术。注册形状使用 Flock 的 `Fast100` 配置，较小形状使用显式 `default_config`
UDR 诊断配置；输出记录完整查询数、递归码率和是否使用注册配置。
PIOP、位列归约及 ring-switch 外层关闭 grinding。没有 ZK 包装，不宣称整套组合达到相同的
128-bit soundness；也不将其标为当前 Flock 默认 Fast/128 的完整证明器性能。

`prover_total_ms` 包含输入检查/表示转换、两份承诺、PIOP、位列归约及两个 PCS opening。
细项分别记录 `prepare_ms`、`commit_ms`、`piop_ms`、`bridge_ms`、`pcs_open_ms`；
总时间还包含少量阶段间分配和释放，不能以分阶段中位数之和代替总时间中位数。
验证包含公开 sigma 的 O(N) 检查/求值，不宣称整个 verifier 是对数时间。
Flock 的 sigma 求值在其原生 verifier 内，Mikros 的公开 tag 求值在 bridge 内。
输入随机生成、序列化/反序列化及额外 transcript 同步断言在计时外。
每个计时证明先序列化、再反序列化，验证全部源 opening 并检查 transcript 同步。

证明字节数包含 PIOP、位列声明和全部 PCS opening；commitment cap 字节另列，
公开 sigma 的 statement 字节另列。没有用 O(N) 原始源 checker 代替 PCS。
比较的是现有 Mikros 系数树实现；其 GKR 仍主要在 F128 上执行，不能代表论文全部优化的上限。

## 验证与复现

`src/tests.rs` 包含诚实证明、非二次幂位宽、两份相同承诺、context/系统替换、
错误输入/公开映射、缺失 source、位列求值、零 padding、ring-switch、Ligerito 消息篡改，
以及保持 PIOP 加权和不变的位列篡改。Flock 的 sigma-aware verifier 使用真实公开 sigma，
其兼容字段 `proof.s_sigma_eval` 不参与判定，测试篡改的是实际绑定的输入求值。

在 WSL 中从本目录运行：

```sh
cargo +nightly-2026-03-21 fmt -p mikros-binius-permutation -- --check
cargo +nightly-2026-03-21 clippy --release --offline -j 2 --all-targets --no-deps -- -D warnings
cargo +nightly-2026-03-21 test --release --offline -j 2 -- --test-threads=1
cargo +nightly-2026-03-21 build --release --offline -j 2
python3 run_experiment.py --name NEW_RUN_ID
python3 summarize.py results/NEW_RUN_ID
```

每个配置先做一对预热，再做四对交替顺序的独立输入；不剔除慢样本。
单线程配置为 `log_n=16,18,20`、位宽 `1,8,32`；另测 `log_n=20,width=8` 的六线程配置。
进程绑定物理核，两个 Rayon pool 的线程数相同。每批前等待至少五秒无构建活动。
测量中检测到 Cargo/rustc 构建干扰时保留该批完整记录，等待安静后重测整个配置，最多五次；
最终只统计无构建干扰的完整批次，不按计时数值筛选。WSL/Windows 仍是共享宿主环境。

`snapshot/manifest.json` 记录当前协议源文件 SHA-256 和原 git revision。
`prepare_snapshot.py` 复用既有 `flock_compare` 的冻结程序，仅在新目录不存在时创建快照。
协议文件逐字节复制，生成的 lookup lib/manifest 只缩小模块和依赖范围。
