# Binary Mikros Hadamard proof 与 Flock degree-two zerocheck 实现计划

2026-09-12 新稿迁移已完成：默认 API 已改为仓库 `latex/` 根目录中的 univariate + GF256 intermediate-field 协议；以下 M0–M7 记录旧 mixed-tensor 实现。新版使用128列声明、127个升幂商系数、一个 `E\H` 挑战和原IP尾部。旧代码及benchmark显式移入legacy范围，v1–v4快照不变。新协议/查表实现见 [README.md](README.md)。服务器真GFNI/64线程测试67项及32份实验证明全部通过；N30六次主计时、独立profile/RSS和下载摘要校验完成，见 [v5报告](server_experiments/20260912T-native64/v5-univariate/REPORT.md)。结果不混入旧协议报告。

状态：首版 binary PIOP 已实现并完成编译、测试及本机 Flock 对比，见 `README.md` 和 `BENCHMARKS.md`。已交付 M0–M4 的核心实现、总耗时/RSS 对比和实际 prover 阶段计时，瓶颈分析见 `PERFORMANCE_ANALYSIS.md`；M5 packed-opening reduction 尚未实现，不计入当前成果。

工程优化已完成：保留 `Kernel::Lookup`，新增跨行 bit-sliced 聚合、动态权重 subset-XOR 表、融合 A/B packing 与 B weighting、C 流式投影及 IP fold/message 融合。同输入粒度消融后选择 64 行 `Kernel::Packed`，保留 `Packed8`。34 个 tests 和正式实验的 272 份证明检查通过，所有动态构表与分配计入 prover。独立同口径结果见 `OPTIMIZED_BENCHMARKS.md` 和 `results/optimized/`，与旧报告分开。

## 1. 目标与首版范围

在 `mikros/` 下新增 Rust workspace crate，实现二进制输入上的 Hadamard 关系 `A ∘ B = C`，并与当前仓库的 Flock degree-two zerocheck 做同口径比较。

首版采用论文专门的 **Binary Hadamard Packing in a Mixed Tensor Basis** 构造。实现 prover、verifier、Fiat–Shamir transcript 和终端求值声明；测试与基准校验通过独立的原始输入求值器核验所有声明。第一组性能结果报告 PIOP reduction，不称作已接入承诺的完整 SNARK。

后续阶段接入 packed-opening reduction，并分别报告增加的开销。底层 PCS 的 commitment/opening 若加入端到端比较，必须在两边采用一致的测量边界。

## 2. 协议依据与实现决定

以下路径均相对于仓库根目录；以本次检查的当前源文件为准，不读取 `latex/mikros/tmp/` 中的备份作为实现规范。

| 项目 | 源码依据 | 实现决定 |
| --- | --- | --- |
| 当前论文入口 | `latex/mikros/main.tex` | 以其加载的正文与附录为准 |
| Binary 构造 | `latex/mikros/4-block.tex:415`；`latex/mikros/appx-tech.tex:38` | 使用 Boolean tensor interpolation 和混合张量基商多项式 |
| 张量乘积与系数归属 | `latex/mikros/appx-tech.tex:83`、`:128` | 按第一个取值为 2 的坐标分配商系数；保持混合基，不转换为单项式基 |
| 消息顺序与随机恒等式检验 | `latex/mikros/appx-tech.tex:220` | 先固定列声明与商系数，再抽取独立的扩域挑战 |
| 三个终端声明 | `latex/mikros/appx-proof.tex:483` | 直接产生 A、B、C 的 weighted matrix-evaluation claims |
| Packed opening 接口 | `latex/mikros/5-pcs.tex:48` | 支持任意列权重与两项张量因子形式的行权重 |
| Flock 基线 | `crates/flock-core/src/zerocheck.rs:531`、`:1283` | 调用现有 `prove_packed` 和 `verify`，保持默认优化路径 |
| 查表参考 | `crates/flock-core/src/zerocheck/multilinear.rs:420` | 借鉴按字节建立 subset-XOR 表的方式，使用 Mikros 自己的线性映射 |

论文还给出 intermediate-field tabulated variant（`4-block.tex:429`）。它是另一条合法路线，需要中间域插值、采样与单变量商；本计划默认实现专门的 binary mixed-tensor 路线。若之后比较该路线，作为独立变体命名与计时。

`alg/alg-hadamard.tex` 是单变量流程；binary 附录明确要求直接认证三个终端声明，不再增加旧流程中的列聚合（`appx-proof.tex:508`）。因此不能原样照搬该伪代码。

## 3. 参数、数据布局与复用

- 首个正式配置：`F = F2`，`E = F2^128`，`kappa = 128 = 2^7`，`N = m * kappa`，`m` 为 2 的幂。`m` 始终表示 Mikros 的行数；适配 Flock API 时使用变量名 `log_n`，避免与 Flock 参数 `m = log2(N)` 混淆。
- 输入采用连续的 LSB-first packed bytes，三份输入共 `3N/8` 字节。按连续 128 个输入位组成一行；显式记录与论文矩阵展开方式的置换关系。两种证明系统接收同一份位序一致的 A/B/C 输入。
- 张量坐标采用论文的 lexicographic 顺序，即第一坐标对应最显著列索引位；调用 `flock-multilinear` 时显式选择 `IndexOrder`，不能依赖默认顺序。
- 复用 `flock-field::F128`，复用 `flock-transcript::{challenger::Challenger, challenger::FsChallenger}`，复用 `flock-multilinear` 的 equality table/evaluation/folding 和 `flock-parallel` 的线程池。
- 先核验 F128 的 GHASH 基表示、ONE 与坐标提取；packed-opening 的固定基必须与实际 field type 一致，不能把 raw integer 1 或 Flock 的 GF256 嵌入当作默认基。
- 小尺寸张量测试可以改变 `s`；只有 `s=7`、`E=F2^128` 的配置用于本计划中遵循论文扩域次数约束的正式对比。

默认参数下，混合张量有 `3^7 = 2187` 个系数槽，其中 128 个是列声明，2059 个是商系数。七个商的槽数依次为 `729, 486, 324, 216, 144, 96, 64`。这两部分的裸 field payload 为 34,992 字节；还需另外计入投影标量、inner-product 消息与终端值，不把该数字当作完整 proof size。

## 4. 实施顺序与验收

### M0 — Crate、位序契约与 Flock 基线

在根 `Cargo.toml` 注册 `mikros/`，新增包 `mikros`。新实现、测试、benchmark 和报告均放在该目录。以 path dependency 复用 Flock crate，避免复制其 field 和 transcript 实现。

新增基线适配器，直接调用 `flock_core::zerocheck::prove_packed` / `verify`。共同基准从 `log_n >= 13` 开始：当前 packed prover 要求至少 `K_SKIP + N_INNER = 13`，其中 `K_SKIP=6`。

为基线提供独立终端声明核验器。Flock 的 A/B 声明点是 `(z, mlv_challenges)`，C 的声明点是 `(z, r_rest)`；`z` 使用 skip 的 Lagrange 权重，不能按普通 MLE 坐标展开。

验收：固定输入的 honest prove/verify 和三个原始输入求值检查全部通过；输入布局用 basis-vector、跨字节位和跨行位测试确认；不改写 Flock 的默认优化算法。

### M1 — Binary 混合张量参考实现

实现 `T = [[1,0], [0,1], [1,1]]` 的递归张量变换。逐行生成 A/B 的变换位，AND 得到乘积位，并按位选择是否 XOR 对应行权重到扩域系数。

按三进制索引中首个 2 的坐标确定商编号及局部槽位。保留输入、行权重、2187 个聚合系数与单行临时空间；不物化 `m × 3^s` 矩阵。

独立实现直接 Boolean Lagrange evaluation 和混合基商求值，用于比较恒等式两边。列声明来自 A/B 的加权乘积；C 的投影从传入的 C 单独计算，不能在 prover 中用 `A & B` 替换用户传入的 C。

验收：小维度穷举、随机行与稀疏行、全零/全一、单 bit 位置测试通过；商槽数及归属正确；随机扩域点的证书恒等式成立；参考实现不依赖未来的 LUT 内核。

### M2 — 完整 PIOP reduction 与 verifier

实现以下协议顺序：

1. 绑定协议版本、参数和由上层提供的输入承诺/公开语句标识；生成独立行、列 MLE 张量权重 `l`、`r`。
2. prover 发送 C 的投影标量、列声明向量及所有商系数；verifier 检查投影与列声明的内积一致。
3. 吸收上述消息后抽取 `x ∈ E^7`；形成 `rho = psi(x)` 并检验 mixed-tensor 恒等式对应的长度 m inner-product claim。
4. 使用 degree-two inner-product sumcheck，把 `⟨A rho, (B rho) ∘ l⟩` 约化为两个终端值。
5. 返回三个终端矩阵求值声明：C 的 `(r,l)`，A 的 `(psi(x),eq(y))`，B 的 `(psi(x),l ∘ eq(y))`。B 的行权重保留为逐坐标相乘的两项张量因子。

若 `l = eq(z)`，B 的每个两项因子是 `((1-z_j)(1-y_j), z_j*y_j)`，一般不等于 `(1-z_j*y_j, z_j*y_j)`。对此加入直接求值回归测试，避免误写成 `eq(z ∘ y)`。

先检查已有 inner-product/sumcheck helpers 的消息与坐标接口；可直接复用的计算原语保留，Mikros 的协议语义在自身模块内实现。verifier 显式检查长度和参数，错误输入返回错误，不依赖 panic。

验收：reference prover 与 verifier 的 transcript replay 一致；错误 C、篡改商、列声明、IP 消息、终端值、截断/额外消息和参数不匹配均由内部验证或终端核验拒绝。包括“保持最终 A/B 乘积但分别修改其值”的测试，防止遗漏源输入绑定。`FsChallenger` 用于有效协议执行；测试用固定挑战只在测试路径使用。

本阶段的原始输入求值器是测试用 oracle，不是 PCS。基准若使用输入摘要作为语句标识，摘要生成在两边同样处理，且不将其描述为 PCS opening。

### M3 — 借鉴 Flock 的查表优化

优化一：将三个张量变量组合成 8-bit 输入块。预计算 `L[v] = T^(tensor 3) v`，每个输出有 27 位，用一个 `u32` 存储。256 项表共 1 KiB（不计对象元数据）。

对一行 128 位执行 16 次查表，再对这 16 个 `u32` 块应用剩余四个变量的张量变换，得到 81 个块、共 2187 个有效位。该上层变换需要 65 次 `u32` XOR；与对应 B 块 AND 后累计行权重。实现时显式验证分块次序与三进制槽位的一致性。

优化二：挑战给定后，为 `A psi(x)` / `B psi(x)` 建立按字节的 subset-XOR table，借鉴 Flock `UniSkipFoldTable` 的构表递推。该表依赖每次证明的挑战，其构建开销必须计入 online prover time。

若 profile 显示行权重聚合占主导，再评估按若干行构造 subset-XOR 权重表的方案；以差分测试和实际阶段耗时决定是否保留。Flock 的 inverse-NTT、GF256 conversion 表各有特定基与索引，不直接作为 Mikros 的张量变换表使用。

验收：穷举验证 256 个小表项；随机/结构化输入下，优化与参考版本的全部列声明、商、proof 消息和终端声明一致；分别记录永久表空间和每次证明构表开销。

### M4 — 同口径性能比较和报告

使用一个 `harness = false` benchmark binary，运行 `mikros-reference`、`mikros-lookup`、`flock-degree2`。不同系统、线程配置和 RSS 测试串行运行。

| 项目 | 统一规则 |
| --- | --- |
| 约束 | 相同 packed A/B/C；honest case `C=A&B`；生成发生在计时前 |
| 规模 | smoke：`log_n=13,16`；主要曲线：`16,18,20,22,24`；更大规模按可用 RAM 扩展 |
| 线程 | 先 1T，再选择不超物理核心数的统一线程数；记录实际线程池配置 |
| transcript | 同一 hash 实现与参数，独立协议 domain；两边 grinding 均关闭并明确记录 |
| 预处理 | witness-independent 固定表记录 cold setup time 和常驻空间；warm online 使用已建表 |
| 在线范围 | 权重生成、witness-dependent preprocessing、challenge-dependent LUT、所有 reduction rounds、输出 C 检查及 transcript hashing |
| 正确性 | 每个测量样本都在计时外 replay verifier 并核验全部终端声明；失败样本终止报告 |
| 重复 | 每配置至少 1 次 warm-up、5 次 measured run，配对相同输入并交替系统执行顺序；资源不足时显式记录减少的次数 |
| 指标 | prover median/min、verifier time、吞吐、裸 field payload 与实际序列化 bytes、独立进程 peak RSS |
| 内存 | 包含 packed 输入、常驻表和工作区；RSS 与 timing 分开运行，不同时持有多轮 witness |
| 扩域安全口径 | `F128` 不自动等于 128-bit soundness；分别记录协议误差与 grinding 配置，再解释速度比 |

参数表明确区分 Mikros 的 packing dimension `s=7` 与 Flock 的 skip dimension `K_SKIP=6`。degree-two 指约束 `a_i*b_i+c_i` 及 inner-product 多项式的次数；Flock 的 equality 因子另行处理，不能据此把其整个加权多项式误称为二次。

分阶段计时至少覆盖权重、系数/商构造、挑战后 packing、IP tail；完整在线计时另行运行，避免用分阶段计时器改变主结果。报告按各规模给出速度比，保留无加速或回退结果。

验收：生成可复现 CSV/JSON 和 `mikros/BENCHMARKS.md`，包含命令、Git revision 与 dirty 状态、输入 seed、Rust 版本、OS/CPU、目标指令集、线程数及测量范围。既有论文实验在 `7-experiment.tex:12` 明确没有测 binary tabulation，不引用其速度数据作为本实现结果。

### M5 — Packed-opening reduction 与组合验证

依据 `5-pcs.tex` 的 arbitrary-column-weight / tensor-row-query 接口，将 A、B、C 三个声明约化到固定基 packed vector 的 MLE openings。先审查 Flock ring-switch / tensor-algebra helpers 的输入布局和适用查询；只复用契约吻合的部分。

该阶段独立报告 `Hadamard reduction + packed-opening reduction`。若与 Flock 比较此层，需要同样计入其对应的 opening reduction；不能将 Mikros 多算一层的结果与仅 Flock zerocheck 相除。

验收：packed vector 与原始位矩阵的基映射一致；随机 query 下 packed-opening reduction 与直接求值一致；错误 source claim 被拒绝。底层 PCS 集成后，再报告承诺绑定的完整证明验证结果。

## 5. 预期目录与接口边界

以下均为拟新增文件，模块拆分可在实现时按既有可复用接口微调。

```text
mikros/
  Cargo.toml
  PLAN.md
  README.md
  src/
    lib.rs
    layout.rs             # packed 输入、矩阵/张量索引和长度校验
    tensor.rs             # scalar reference 与 mixed-basis coefficients
    lookup.rs             # 1 KiB transform LUT 与动态 subset-XOR tables
    hadamard.rs           # prover/verifier、消息、终端矩阵声明
    inner_product.rs      # Mikros IP transcript，复用适用的底层计算原语
    packed_opening.rs     # M5
  tests/
    tensor_reference.rs
    hadamard_roundtrip.rs
    malformed_proof.rs
    support/              # 独立 source-oracle 与 Flock claim 检查
  benches/
    hadamard_vs_flock.rs
  BENCHMARKS.md
  results/
```

公共接口先围绕 packed inputs、参数、proof、verification error 和 deferred claims 设计。既有 Flock 的 proof types 保持原样；benchmark adapter 仅用于统一测量和检查，不把不同语义的声明强制伪装成同一种 MLE。

## 6. 验证命令与环境前置条件

crate 建立后预期执行：

```text
cargo test -p mikros --release
cargo fmt --all -- --check
cargo clippy -p mikros --all-targets -- -D warnings
cargo bench -p mikros --bench hadamard_vs_flock
```

benchmark 的规模、线程、重复次数、系统选择和 RSS 子进程参数在 M4 中定义并写入 README；当前不假定尚未实现的 CLI 参数已存在。

规划时当前 PowerShell 未找到 cargo/rustc。实施时已确认并使用 Ubuntu/WSL2 中的 `nightly-2026-03-21`，实际为 `rustc 1.96.0-nightly`；本机 i7-8700K 的编译路径启用 PCLMUL/AVX2，单线程与 6 线程使用同一 executable。具体测量和范围见 `BENCHMARKS.md`。

## 7. 指定 skill 的配置进度

已读取 `setup-matt-pocock-skills/SKILL.md` 及其四个模板，并检查远程、根目录文档和目录布局。当前有 GitHub remote `mirandaprivate/mikros_flock`；根目录没有 `AGENTS.md`、`CLAUDE.md`、`CONTEXT.md`、`CONTEXT-MAP.md` 或既有 `docs/agents/` 配置。

该 skill 要求依次确认任务记录位置、triage 状态名称和 domain docs 布局，再展示配置草案。用户随后明确要求直接实现并与 Flock 比较，因此实施工作独立推进；本轮未设置任务记录平台或创建根 agent 配置。

协议与实现工作不依赖任务记录平台。此文件不代表 skill 配置已完成，也没有创建 GitHub Issue。

## 8. M6 — 64 线程 native 工程优化（已完成）

在保持 binary Mikros 消息、挑战顺序和终端声明不变的条件下，完成流式 tensor、GFNI affine aggregation、AVX-512/VPCLMUL packing/IP/C、首消息融合、blocked row weights、并行输出初始化、可复用 scratch 和分层权重候选。旧路径保留为同 binary 消融；Flock 源码未改。

服务器按 [SERVER64_PLAN.md](server_experiments/SERVER64_PLAN.md) 固定 64 个逻辑 CPU，完成 N24 八变体十六次、N30 四系统八次，以及两组三系统六次补测。服务器 34 项单元测试和 12 项集成测试通过，实际执行硬件路径；每份实验 proof 均核验 verifier、transcript 和全部原始输入声明。最终结果、阶段/RSS 与版本来源见 [报告](server_experiments/20260912T-native64/REPORT.md)，工程解释见 [瓶颈分析](server_experiments/20260912T-native64/FINDINGS.md)。M5 的 PCS/packed opening 范围仍独立。

## 9. M7 — 相同 128 倍压缩的 Flock 对照（已完成）

新增明确的 skip7 PIOP prover/verifier、128-lane GFNI/AVX-512 LDE 与首轮、16-byte 行 VPCLMUL 融合折叠；保留 skip6 和现有 PCS 接口。此阶段按用户要求修改了 Flock 源码，M6 的“Flock 未改”仅描述历史 v1–v3。

按 [同压缩比方案](server_experiments/EQUAL_COMPRESSION_PLAN.md)，v4 在相同服务器与 64 个逻辑 CPU 上完成 N30 三系统六次主计时、三次独立 profile、独立冷进程 RSS，以及另起进程的 Flock64/128 各四次参数对照。共 104 份实验证明全部完成源声明核验。新增 skip7、旧协议/NTT 回归及额外两项全 GF8 节点插值测试成功；冻结源码与下载证据摘要均通过校验。结果见 [v4 报告](server_experiments/20260912T-native64/v4/EQUAL_COMPRESSION_REPORT.md) 和 [分析](server_experiments/20260912T-native64/v4/FINDINGS.md)。
