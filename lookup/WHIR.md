# F128 WHIR PCS integration

本目录参考 `../mikros/mikros/whir/src/pcs.rs` 的承诺/独立 opening transcript、承诺重放、final-form 校验和严格 EOF 接口，加入真实的 **GHASH F128 WHIR**。`authenticated::{prove_permutation, verify_permutation, prove_lasso, verify_lasso}` 认证完整 PIOP 的源声明，验证端不接收源 witness。

## 域与 RS 适配

本地 Mikros 的现有 WHIR adapter 用奇素数扩域；它的 Two-adic FFT 不能直接用于特征二。这里保留 WHIR 的 IRS/Merkle、OOD、folding、sumcheck、opening 和 verifier，在 `lookup/` 内增加：

- `whir_field.rs`：arkworks `Field` 对 Flock GHASH 的精确适配。基域为真正 F2，扩域多项式为 `X^128+X^7+X^2+X+1`。整数转换按模 2 处理，raw bit coordinates 单独转换，不把 F128 伪装成奇素数域。
- `whir_rs.rs`：RS evaluation domain 为 GHASH 多项式基下的 raw indices `0..codeword_length`，这些是互异域元素。通过 additive subspace vanishing polynomials 递归取余，对同一个单变量系数多项式求值。独立 Horner 测试验证每个输出；不是模约减到奇素数域，也没有额外扩域。当前为参考编码内核，没有 FFT/SIMD 优化性能承诺。
- vendored WHIR 的两处小修改：对外提供 RS message 的只读访问；Standard/non-ZK sumcheck 不再无条件计算不存在的 `1/2`。ZK 构造仍拒绝特征二。实际非 ZK 路径不调用 multiplicative generator。

WHIR 输入向量在此表示原 bit oracle 的 Boolean-cube evaluation table，同时作为 IRS 内部单变量编码的系数列表；WHIR 保持这一接口原有约定。没有对 witness 做额外的 Boolean-to-monomial 变换。查询以 `Covector` 给出，因此 row/column offset、low-to-high 与 Mikros high-to-low 顺序都在公共权重中明确处理。

## 完整认证顺序

1. permutation 承诺 Left、Right；Lasso 承诺 Values、Addresses、ReadCounters、Carry、FinalCounters。每个 bit 展开为 E 中的 0/1，列优先扁平化，补零到二次幂（至少 2）。公共表仍是 statement 中的公开表。
2. 按固定 source 顺序，把 WHIR commitment ID、源形状、固定 PCS profile 和应用 context 写入外层 transcript。Lasso 的三个辅助根也写入 PIOP 的 auxiliary identifiers。这里的 identifier 是实际 WHIR 承诺，不再使用不能与 PCS 相连的直接数据摘要。
3. 运行原 permutation/Lasso PIOP。全部根固定之后才取 tuple aggregation challenges、grand-product challenge。仍有 `K≤E`，128 bit 以上元组仍只用 E challenges 聚合。
4. 对每个已承诺源采样 `r`，证明 `Σ_i eq(r,i) f(i)(f(i)+1)=0`。degree-three sumcheck 的终端须满足 `right=left+1`，并用 WHIR 认证 `left=f(q)`。此外，若有 padding，用公共线性 query 认证尾部在随机 `r` 处的求值为零。两者防止把任意 E witness 当作 Boolean bit oracle，或伪造隐式零尾部。
5. 把该源的所有 PIOP 声明、Booleanity 终端声明、padding 声明合并为一次 WHIR opening。Permutation 有 2 个 WHIR opening；Lasso 有 5 个，覆盖通常的 11 个 PIOP 源声明（空查询时 8 个）。
6. verifier 重放 commitment，核对 commitment ID，检查 WHIR reduction **及 final linear forms**，并检查 commitment/opening 两个 transcript 的 NARG 和 hint EOF。完整 verifier 不调用 `Claims::check`。

所有查询都是从公共 statement 和已验证的 PIOP transcript 推导的权重，不把 prover 声明的任意 query 当作可信输入。每次 opening 的独立 transcript 还绑定完整有序权重/值列表和外层派生 context；opening 摘要回写外层 transcript。

## 参数与边界

- Unique decoding，rate 1/2，folding factor `min(4,num_vars)`，BLAKE3，**96 bit WHIR 子协议诊断目标**，全部 PoW slots 为零，包括 initial-skip slot。配置构造检查实际 target；完整 verifier 固定此 profile，拒绝参数降级。
- 这是非 ZK 研究实现。96 bit 是 WHIR backend 的子协议参数计算结果，不是整个组合的 union-bound / Fiat–Shamir 安全证明，更不是严格 128 bit soundness 结论。
- 每个源最多 `2^20` 个 E 元素，行数也限制为 `2^20`（包括零位宽源和公开表）。每个 opening 最多 32 个线性声明，内层 proof 最多 64 MiB。解析长度先核验剩余字节再分配；多余尾字节、截断和错 commitment 都拒绝。
- verifier 不读取私有源，但目前构造并求值显式公共 covectors，耗时/内存随源长度线性增长。公开表处理也为线性时间。
- 位宽优化仍在乘积树中；PCS 当前每 bit 存一个 E 元素，并增加 Booleanity 工作。不能把此前 PIOP 时间当成这版端到端时间，也不能据此声称优于原生 Flock 完整 lookup。
- 公共 statement 确定所证明的输入承诺。调用者应保存/核对应用预期的 statement，不能让攻击者替换整份 statement 后仍视为同一个任务。

## 来源与本地改动

`vendor/whir` 的完整包复制自同机 `../mikros/mikros/whir/vendor/whir`，保留原 licenses、README、benches 和 source。该 sibling README 记录来源为 brakefold `0818457cf50e464d5a3bb29146259ec141e8f5f1`，其包内历史 UPSTREAM.md 指向 worldfnd/whir `92652ca01e215548c98e11834e110c43994b94c1`。**这些是 sibling 记录的来源，不是声称当前副本等于未经修改的 upstream。** 副本已包含 sibling 的 final-form 零值修复、规范解码、hint 长度边界、EOF 和完整声明检查等修复。

本次只在 `lookup/` 修改代码，未修改 sibling 或主任务 `mikros/`。`vendor-source-manifest.json` 记录被复制的 sibling 文件 SHA256 及本地文件 SHA256；WHIR vendor 代码新增差异只有上述两处。其他新增代码在 `src/whir_*` 和 `src/authenticated.rs`。Cargo.lock 固定本 workspace 的解析版本。

## 运行

在 `lookup/` 的 WSL 目录中：

```sh
CARGO_TERM_WIDTH=200 RAYON_NUM_THREADS=2 cargo +nightly-2026-03-21 test --offline -p mikros-lookup --message-format=short -j 2
CARGO_TERM_WIDTH=200 RAYON_NUM_THREADS=1 cargo +nightly-2026-03-21 run --release --offline -p mikros-lookup --message-format=short -j 2 --example whir_lookup -- 64 8
```

`whir_lookup` 参数为 `rows value_bits`，输出 proof/statement 字节、P/V 时间和 verified 状态。输入生成不计时，P 包含配置构造、承诺、辅助 witness、PIOP、Booleanity 和全部 opening。它是单次 smoke measurement，没有预热或稳定性统计。测试及实际输出见 `WHIR_RESULTS.md`。
