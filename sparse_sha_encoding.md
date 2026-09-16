本轮为 Mikros 的显式稀疏矩阵 lookup 增加了可选 SHA-256 编码，使三矩阵合计非零元素从 **47,399,406 降至 777,672，减少 98.36%**。这里记录的是矩阵规模和正确性检查；**目前没有据此得出的端到端 prover 加速结论**。

实现位于 `crates/flock-prover/src/r1cs_hashes/sha2.rs`，入口为 `build_sparse_block_r1cs` 和 `build_sparse_block_witness`。原来的 `build_block_r1cs`、witness 生成器及 `Sha2LincheckCircuit` 继续使用默认编码。

默认编码把 schedule 的 W[t] 和每轮 T1 写成符号 XOR 表达式，只隔一轮保存 E/A。这样减少了独立 witness 槽位，但后续消费者会反复展开这些表达式；W 的递归依赖及跨轮状态使线性支持集很大。这些支持集已做 GF(2) 的重复项消去，不能靠再次去重消除主要成本。

FLOCK 可以通过公开电路的 walking 或 CSC 线性组合计算矩阵查询，不必把每个非零位置都变成需要另外认证的 lookup 记录。这并不表示展开免费：当前 SHA fixture 也构建 CSC，仍有相关构造、存储和计算成本。显式 SPARK 路径则需要为非零位置准备地址、时间戳、值以及查询相关的读取向量，并执行内存一致性证明和 PCS opening；这些工作直接受 nnz 和补齐后的表长影响。因此，适合减少 witness 槽位的编码，未必适合显式稀疏 lookup。

新编码在原有 padding 内保存中间结果，区间均为左闭右开：

| 新增内容 | witness 区间 | 位数 |
|---|---|---:|
| W[16..64] | [25,470, 27,006) | 1,536 |
| 每轮 T1 | [27,006, 29,054) | 2,048 |
| 原偶数轮 E_NEW | [29,054, 30,078) | 1,024 |
| 原偶数轮 A_NEW | [30,078, 31,102) | 1,024 |

对每个新保存的比特 s，新增约束是 `raw(z) · z[pin] = z[s]`，之后的消费者使用 `z[s]`。在外层认证 `z[pin]=1` 的条件下，这正是给原线性表达式取一个中间变量；依照 schedule 和轮状态的计算顺序消去这些变量，可恢复原表达式。原来的 carry、AND 计算关系按同样的中间变量替换保留，矩阵条目会改变。将新解的原有效前缀保留、把原 padding 恢复为零，可得到原编码的 witness；原解也可按这些定义扩展成新解。

奇数轮 E/A 复用原槽位。原有效前缀 `[0,25,470)`、全部 public IO 地址以及 pin 位置 **25,469** 不变；`C=I`。有效槽位数从 25,470 增加到 **31,102**，剩余 `[31,102,32,768)` 仍约束为零。每实例矩阵维度和 witness 长度仍是 **K=2^15=32,768**；对于固定实例数，总域大小不增加。这里的 SHA 语义是独立 compression，不是自动包含消息 padding 的完整哈希。

| 矩阵 nnz | 默认编码 | 新稀疏编码 |
|---|---:|---:|
| A | 28,839,569 | 535,556 |
| B | 18,527,069 | 209,348 |
| C | 32,768 | 32,768 |
| 合计 | **47,399,406** | **777,672** |

合计规模相差 **60.95 倍**。非零元素减少会缩小相关 lookup 工作，但 Hadamard、两个 IP、查询生成、PCS、内存分配等也影响总时间，不能把这个比例当作 prover 加速倍数。比较时应披露 Mikros 使用了不同的等价 SHA 编码，FLOCK 保留原生编码。

已完成本地 WSL release 检查：新增测试覆盖全零消息、全一消息和两组随机输入，检查新 R1CS 关系、SHA 参考输出、旧前缀一致、pin 及零 padding；每组还翻转 schedule、T1、额外 E/A 和 padding 的边界位置，共 **40 次篡改均被约束检查拒绝**。原默认 witness/reference 回归也通过。默认 density 测试和新增测试分别输出上表的 nnz。对应命令：

```sh
cargo test --release --offline --locked -p flock-prover --lib sparse_schedule -- --test-threads=1 --nocapture
cargo test --release --offline --locked -p flock-prover --lib r1cs_hashes::sha2::tests::block_witness_satisfies_matrix_and_matches_reference -- --test-threads=1
cargo test --release --offline --locked -p flock-prover --lib r1cs_hashes::sha2::tests::sha2_density_profile -- --ignored --test-threads=1 --nocapture
```

这些是编码及约束检查，不能代替完整 proof 的 PCS 验证、转录一致性、端到端篡改检查和服务器性能实验。
