# WHIR 接入验证记录

日期：2026-09-12。范围：`lookup/` 的 F128 WHIR adapter 及完整 permutation / indexed Lasso 认证组合。实现说明见 [WHIR.md](WHIR.md)，原 PIOP-only 对照数据仍保留在 [RESULTS.md](RESULTS.md)。

## 已完成验证

- Release：**24 个单元/集成测试 + 2 个文档示例全部通过**。原始输出：[whir-release-tests.log](results/whir-release-tests.log)。
- `--no-default-features`：**13 个 PIOP 单元/集成测试通过**，确认原 PIOP 可以不依赖 WHIR 单独编译使用。原始输出：[whir-disabled-tests.log](results/whir-disabled-tests.log)。
- `cargo clippy --offline -p mikros-lookup --all-targets --no-deps --message-format=short -j 2 -- -D warnings` 通过。本包没有 Clippy warning；Flock 的未使用 AVX512 imports 和 vendor 原有 dead-code warnings 属于依赖，未修改这些代码来消除警告。
- `cargo fmt -p mikros-lookup -- --check` 通过。

测试覆盖：

1. GHASH 运算、模多项式、F2 转换、基坐标往返和规范 16-byte 编码；additive RS 在多个长度上逐点与独立 Horner 求值相等。
2. 真正 full-F128 公共 covector opening：长度 2、8、64、1024；新的 verifier 在丢弃 prover data 后验证。错误值/context、篡改、截断和多余尾字节均拒绝。
3. 标准 WHIR 的奇素数扩域后端回归通过，确认移除 Standard 模式无用的 `1/2` 后原路径仍能证明并验证非零声明。
4. 完整 permutation/Lasso 的序列化往返、空实例、零位宽、重复地址、同值不同表位置、精确 128 bit 和随机聚合 129 bit 以上元组；compact/full 策略给出相同完整 proof。
5. 错输入承诺、辅助根替换、缺失/重排/多余 opening、五个源分别篡改、降低 security profile、改变公开表与 context 均拒绝。
6. 恶意 prover 提供**单独有效的 WHIR proof**，但源是非 Boolean E 向量或含非零 padding 时，完整源 verifier 拒绝不一致的 Booleanity/零尾部声明。
7. commitment 和 opening 的 NARG / hints 四条流各自加入尾字节都拒绝；commitment 情况还重新计算了 ID，使测试真正进入 EOF 检查，而不是只触发哈希不同。超大 wire 长度、零位宽绕过容量限制也拒绝。

此前 RESULTS.md 记录的主目录并行修改构建阻塞，在本次构建时已不再发生；这里的成功结果只对应本次实际读取的依赖状态。

## 端到端 smoke 数据

本地 WSL Ubuntu，i7-8700K，Rust `nightly-2026-03-21`，release，`RAYON_NUM_THREADS=1`。每组仅运行一次，无预热，没有稳定性或公平性能对比结论。

输入为 64 次查询、8 项公开表。P 包含配置、源承诺、辅助 witness、PIOP、Booleanity 和全部 WHIR openings；V 使用新的公开验证过程，包含公共 covector 构造。输入生成在计时外。proof 大小为 `bincode::serialize(proof).len()`，不含公共 statement；statement 的大小在原 JSON 单列。

| value bits | tuple bits | 协议 | P (ms) | V (ms) | proof (bytes) | 验证 |
|---|---|---|---:|---:|---:|---|
| 8 | 18 | permutation | 33.1544 | 3.1777 | 177160 | 通过 |
| 8 | 18 | Lasso | 7.5544 | 7.6121 | 448161 | 通过 |
| 119 | 129 | permutation | 60.4254 | 8.6452 | 268888 | 通过 |
| 119 | 129 | Lasso | 21.4707 | 11.9690 | 494025 | 通过 |

每个进程都先跑 permutation，因此它承担首次域、additive RS 和线程池初始化；Lasso 会复用已经初始化的公共缓存。**不能用此表直接比较两种协议的速度，也不能与此前 PIOP-only 时间计算加速比。** 129 bit 元组那组实际使用 2 个 E aggregation challenges，K=E，没有实例化更大的扩域。

原始输出：[8 bit](results/whir-64-8.json)、[119 bit / 129 bit tuple](results/whir-64-119.json)。构建日志：[whir-smoke-build.log](results/whir-smoke-build.log)。实际运行的 `target/release/examples/whir_lookup` 的运行后 SHA256：

```text
58e2c6ffcc8cd6a192194191ff26643bb6db6a565deb5b41fcba9860b695f2ad
```

当前是非 ZK、96-bit WHIR 子协议诊断配置；整体协议误差还包含 PIOP、Booleanity、padding 和 tuple 聚合。PCS 每 bit 存一个 E 元素，公共 covector 处理仍为线性成本。本记录不声称严格 128 bit soundness、PCS bit packing，或优于原生 Flock 完整 lookup。
