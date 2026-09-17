# Binary Hadamard baselines

同一三输入关系 `A[i] * B[i] - C[i] = 0`，输入是 LSB-first bit-packed F2。**`small_field_first` 直接调用本地 Plonky3 实现**。此机器的 `../mikros` 本身就是 Plonky3 checkout，没有额外的 `../mikros/Plonky3` 子目录。

## 实际调用与对照

`Cargo.toml` 通过 `../../mikros/mikros` 引用 `mikros-experiments`，通过 `../../mikros/{binary-field,field,sumcheck,...}` 引用同一 checkout 的 p3 crates。路径以 `baseline/` 为基准；未复制或重写原生 sumcheck callback。

| CLI system | 实现 | 工作域 / 扩域 | equality weights |
| --- | --- | --- | --- |
| `small_field_first` | 原生 `baseline::prove_hadamard` / `verify_hadamard` | `BinaryField8` / `BinaryField128` | 原生 factored low/high weights |
| `small_field_first_full_extension` | 原生 `prove_hadamard_full_extension`，同一 verifier | 同上，首轮前全部提升到扩域 | 同上 |
| `dao_thaler` | 本目录 Algorithm 2/3 适配 | packed F2 / Flock GHASH F128 | 固定最低七个 tower 坐标，位操作和基转换 |
| `dao_thaler_generic` | Dao–Thaler 通用乘法对照 | 同上 | 通用域乘法 |
| `ghash_small_field_first` | 上一版 GHASH 重写实现 | packed F2 / GHASH F128 | 物化 N/2 weights |
| `ghash_full_extension` | 上述 GHASH 对照，首轮前提升全部 A/B | 同上 | 同上 |
| `flock` | `flock_core::zerocheck::{prove_packed,verify}` | packed F2 / GHASH F128 | Flock 生产实现 |

原生 Small-field-first 保留参考实现的 base-field-first 存储、fold/packing API、轮消息和 BLAKE3 `BinaryChallenger<HashChallenger>`。三次插值需要四个不同节点，不能以 Gf2 作为工作域；将输入 bits 嵌入 BinaryField8 的 0/1，关系仍是 binary Hadamard。首个随机折叠后使用 BinaryField128。实际 packing width 记录在 JSON；使用 packing API 不等于已有多 lane SIMD 后端。

三组优化/对照各自使用相同 transcript，证明字节必须一致。不同域表示、协议和 transcript 编码之间不要求字节一致。

## 三输入归约与接口

degree-2 指 AB-C 的次数。先求 `c_eval = C~(z)`，再以 cubic weighted sumcheck 证明 `sum_x eq(z,x) A~(x) B~(x) = c_eval`。原生 API 归约 A/B 的 weighted product；`src/plonky3.rs` 补齐 C(z) 及第三个源声明。C 使用原生 `Poly::eval_base`。bit→BinaryField8 转换、C 求值、动态分配、原生证明、fold 和 transcript 全计入 prove 时间。

| API | 用途 |
| --- | --- |
| `Statement::from_inputs(log_n,a,b,c)` | 校验 packed 形状并生成公开输入实验标识 |
| `plonky3::challenger(domain)` | 原生二进制域 BLAKE3 challenger |
| `plonky3::prove(&statement,a,b,c,method,&mut ch)` | 返回原生证明字段封装 |
| `plonky3::verify(&statement,&proof,&mut ch)` | 调用原生 verifier，返回三个 Claims |
| `plonky3::check_claims(&statement,a,b,c,&claims)` | 从原始 bits 独立核验三项声明及 digest |
| 根 API `prove/verify/check_claims` | GHASH Dao–Thaler 与明确标名的 GHASH 对照 |

原生 `plonky3::Claims` 坐标为**最高位在前**，遵循 Plonky3；根 API 的 GHASH `Claims` 为最低位在前。C 使用初始 z 点，A/B 使用末端 sumcheck 点。

支持 `0 <= log_n <= 30`；每输入 `ceil(2^log_n/8)` 字节，未使用高位必须为零。原生证明保留 `GenericDegreeProof` 的 claimed_sum、三节点轮消息、空 PoW 数组及 final A/B。其 bincode 大小为 `56*log_n+64` 字节，GHASH 为 `48*log_n+56`；两者 field payload 都是 `3*log_n+3` 个 128-bit 值，其余为 framing。

这是 PIOP 归约，没有 PCS、grinding 或相同安全等级声明。真实用途须绑定源 commitments 并认证三个返回声明。公开输入 digest 不是 polynomial commitment；`check_claims` 是计时外的 O(N) 直接源核验。

## Dao–Thaler 与表示

`src/dao_thaler.rs` 保留 Algorithm 2/3 适配。最低 `min(log_n,7)` 坐标固定为 canonical Wiedemann/Fan–Paar generators，其余随机。先逆序处理随机坐标，再按高到低处理固定坐标。GHASH 原始 bits 不是 canonical tower 系数，因此先转换 suffix seeds，在至多 128 项小块中用 Algorithm 3 展开，再转回 GHASH。两张固定转换表共 128 KiB；首次初始化与在线转换都计时。

Dao–Thaler 参考源为 `../mikros/baseline/src/lib.rs` 和 `../mikros/mikros/native-binius/src/dao_thaler.rs`。原生 adapter、核心 p3 源码与清单 SHA256 见 [reference_sources.json](reference_sources.json)；checkout HEAD 不能替代未提交源码的 hash。

## 构建、测试、计时

从 `mikros_flock` 根目录，在 Linux/WSL 执行。要求 sibling Plonky3 checkout 存在。本 crate 使用独立 Cargo.lock 和 `baseline/target`。

```sh
# 加宽诊断，规避当前 nightly 渲染某些依赖警告时的崩溃。
export RUSTFLAGS='-C target-cpu=native --diagnostic-width=200'
cargo +nightly-2026-03-21 test --manifest-path baseline/Cargo.toml --release --locked --offline
cargo +nightly-2026-03-21 fmt --manifest-path baseline/Cargo.toml -p hadamard-baselines -- --check
cargo +nightly-2026-03-21 clippy --manifest-path baseline/Cargo.toml --all-targets --no-deps --locked --offline -- -D warnings
cargo +nightly-2026-03-21 bench --manifest-path baseline/Cargo.toml --bench hadamard --locked --offline -- \
  --log-n 20 --runs 6 --threads 1 --systems small_field_first,dao_thaler,flock \
  --output results/plonky3/n20-1t.json
python3 baseline/summarize.py
```

`--output` 相对 `baseline/`。Flock 要求 `log_n >= 13`。所有方法共用 global Rayon 线程池、编译目标和每轮输入。每方法预热一次；三系统六次测量覆盖六种排列，保留全部样本。输入生成、输入标识、序列化和源核验在计时外。原生 verifier 中 O(log N) 的证明封装 clone 计入 verify 时间。

Plonky3 与 Flock 的扩域同构，但表示、域后端、工作域及 transcript 不同，差值不能仅归因于协议。原生测试核验优化/对照和单/多线程证明一致性、非法关系、消息篡改、三个源声明、坐标方向及 GHASH 同构下的独立求值。
