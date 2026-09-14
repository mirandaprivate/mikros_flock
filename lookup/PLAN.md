# Bitwidth-aware permutation 与 permutation-based Lasso lookup

日期：2026-09-12。状态：用户已确认两种容量回退且 K 不得超过 E；M0–M4 的首版 PIOP 已实现，验收与对照结果见 README.md / RESULTS.md。用户随后要求参考本地 `../mikros/` 接入 WHIR；M6 的实现和验证边界见 WHIR.md。M5 的进一步内核优化保留为后续范围。下文保留原始规划依据，当前交付范围以 README.md 为准。

## 1. 目标与规划方式

在 `lookup/` 实现两个可组合的模块：

1. Bitwidth-aware permutation proof：低层保留紧凑的小域/系数表示，使用 Mikros 的乘积树归约；当继续扩展超出配置的扩域容量时，在现有扩域中使用随机挑战归约。
2. Bitwidth-aware Lasso lookup：用读写集合的 permutation proof 证明查询值来自指定表，保留 value、address、counter 的各自位宽。

按用户“先用 /mattpock 做计划”的要求，本轮采用本地 Matt Pocock `ask-matt` 的规格→带依赖的可验收步骤路线。没有找到名为 `mattpock` 的独立 skill。已阅读 `ask-matt`、`to-spec`、`to-tickets` 和 `setup-matt-pocock-skills`。这是一份本地规划草案，不宣称已完成仓库级 skill setup，也未向外部 tracker 发布 issues。

`lookup/` 采用独立 Cargo workspace、锁文件和 target，全部新增实现集中在此目录。现有 Flock 算术、challenger 和 Mikros binary Hadamard 作为路径依赖复用；已重新核验公开接口。下文保留规划依据，实际实现边界以本段及 README.md 为准。

## 2. 已核验的依据

| 来源 | 本计划采用的内容 |
| --- | --- |
| `latex/mikros/4-block.tex:525–782` | 先固定输入，再随机 grand-product fingerprint；底层系数多项式树，上层扩域乘积树；反向归约至原始输入声明 |
| `latex/mikros/alg/alg-permutation.tex` | permutation 消息顺序、两棵树、随机合并子声明与最终去除公开 shift |
| `latex/mikros/appx-proof.tex:602` 起 | permutation fingerprint 的次数界与各层归约误差 |
| `mikros/src/hadamard.rs` | 现有接口只证明 packed binary Hadamard，并返回 deferred claims；不能直接当作任意位宽的多项式乘法证明 |
| `crates/flock-field/src/lib.rs`、`phi8.rs` | F128、GF(2^8) 的 F8 和经验证的 phi8 嵌入；F8 的 raw byte 不是同值的 GHASH 域元素表示 |
| [Lasso 原论文](https://eprint.iacr.org/2023/1216)；[作者 Jolt 论文附录 B.3](https://people.cs.georgetown.edu/jthaler/Jolt-paper.pdf) | indexed lookup 的离线读写集合归约、计数器、tuple fingerprint 与 grand-product 接口 |

当前 `latex/mikros` 的有效正文没有 Lasso lookup 构造；Lasso 部分是以公开原始来源为依据的新增组合设计。其他工作目录的旧 lookup 草稿与当前 checkout 不一致，不能作为这里已经实现或已证明的规范。

## 3. 语义与边界

- 证明域首版固定为 `E = GF(2^128)`，复用 Flock F128；不因中间表示变宽而自动启用更大的证明域。
- 值、地址、计数器都是带显式宽度的 packed 数据。位宽是协议参数，并绑定到 transcript；不根据 witness 中偶然出现的 leading zeros 改变协议形状。
- 区分整数位串、底域元素、域基坐标与多项式系数。任意 w-bit 子空间不自动是 GF(2^w) 子域；只有实际存在、验证了乘法兼容性的嵌入才可用于子域运算。
- 先支持显式公开表与 indexed lookup：`query_value[i] = table[query_address[i]]`。无地址 lookup 可由 prover 提供并绑定地址 witness，复用同一接口。
- 同表重复值合法；地址必须显式编码，不能将两个同值的表位置合并为一个位置。
- 首版交付 prover、verifier、transcript 和需要认证的终端声明；原始输入直接求值只用于测试。最终是否接具体 PCS 单列，不将数据摘要或直接重算称为 PCS proof。
- structured-table Lasso/Surge、超大隐式表分解、完整 zkVM 集成不是首个可验收版本；本计划先交付显式表的 permutation-based Lasso。

### 已确认的用户语义

“超过 extension field 时随机聚合”可能指两处：

1. 乘积树中间系数表示继续增长，超过目标证明域的可用编码容量；
2. lookup 的 value/address/counter 元组总位宽超过一个 F128。

用户已回复：“两者都是，K不能超过E，帮我实现”。两处均已默认启用；E 固定为 F128，不构造更大的 K。精确树层预算为 `2^t*k` bit，k 为实际子域次数；超过预算时采用同一个 grand-product z 的多项式求值，保留乘法关系。超宽 tuple 使用额外独立的 E 线性聚合挑战。

## 4. Permutation 协议设计

### 4.1 公共语句

输入是两个已绑定的等长向量及布局/位宽参数，目标是多重集合相等。双方使用相同 padding，并将原始长度、padding 规则和布局写入 transcript。

固定输入后采样 `z ∈ E`，比较：

`GrandProd(z + a) = GrandProd(z + b)`。

支持重复值和零因子；不使用可能在零因子处失败的 running-product 除法检查。零因子不会自动触发重采样。

### 4.2 底层保持紧凑表示

沿用论文 `C_0(Z)=Z+D`、`C_{t+1}(Z)=C_{t,L}(Z)·C_{t,R}(Z)` 的系数树。每层记录实际的底域、系数个数、首一项是否省略、逻辑长度和物理字节数。

低层采用 bit/byte packing、按机器字批量处理和适用的查表；先计算系数，再在切换处求值，避免从第一片叶子就物化全部 F128 中间数组。

切换条件根据**下一层所需的完整编码预算**制定，包含每个系数占用的底域坐标数；不能把多项式 degree、字节长度和扩域次数混为一谈，也不能只写未经证明的 `2^t*w` 判断。M0 输出精确预算公式及边界示例。

### 4.3 超容量时的处理

若下一层不再适合当前精确系数表示，就在固定的 E 中对尚未取模的系数多项式作随机求值，并进入普通 E-valued product tree。用于新恒等式检验的挑战必须在相应输入/证书固定后采样；同一父子关系必须使用相容的求值点。

求值 `P(X) -> P(r)` 保持未取模多项式的乘法关系。任意坐标随机线性投影一般不满足 `R(uv)=R(u)R(v)`，所以不能把更大扩域的乘法结果随意压缩回 F128 并沿用原来的乘法检查。

需要明确区分：grand-product challenge z、切换/系数检验挑战、sumcheck challenges、子声明 batching challenges。只有给出一致的次数和 transcript 论证后才复用挑战。

### 4.4 Verifier 与终端声明

从公共 grand product 反向展开扩域树及系数树，每层证明父值来自正确的两个子向量，并绑定回原始输入。不能只验证最终两个乘积相等。

系数树的乘法是系数卷积，不能逐项套用现有 binary Hadamard API。实施前先明确卷积/求值与 packed Hadamard reduction 的桥接；这一桥接是 M2 的验收内容。

终端声明保留原始 source ID、布局与实际查询权重。若权重是任意两项 tensor factors，就如实返回 factors；没有验证归一化条件时不冒充普通 MLE 点。

## 5. Lasso lookup 组合

设表长为 M，查询数为 N。每条查询携带地址和值，并有该地址的访问前计数器 c_i；每个表地址有终止计数器 C_j。

构造长度相同的两组多重集合：

- `Init = {(j, T[j], 0)}`；
- `Read = {(address_i, value_i, c_i)}`；
- `Write = {(address_i, value_i, c_i+1)}`；
- `Final = {(j, T[j], C_j)}`。

调用 permutation proof 证明：

`Init ⊎ Write = Read ⊎ Final`。

这是 Lasso 的显式表离线 memory-checking 归约；表、查询、地址、计数器及其派生元组之间的连接也必须进入证明。公开表部分可由 verifier 计算其必要的公开求值，不把它算作已认证的私有 witness。

### 5.1 位宽与计数器

地址保留 `ceil(log2 M)` 个显式 bit 坐标；计数器宽度至少为 `ceil(log2(N+1))`。M=1/N=0 等零宽度边界有单独的规范，不能导致空移位或空表越界。

二进制域的 `c+1` 是 XOR，不能表示整数计数。首版候选使用二进制整数 counter，加 Boolean carry/increment 约束，证明无 wraparound，并验证需要的范围条件。局部约束与其 witness 也必须经过 prover/verifier；不能只在 witness builder 里检查。

乘法型 `omega^c` counter 可作为后续替代，需另外验证非零性、omega 的阶大于可出现的周期，以及其位宽代价，不能未经论证替换整数 counter。

### 5.2 元组装得下时精确打包

若全部字段的总位宽不超过 E 的 128 个二进制坐标，就按固定、互不重叠的基坐标拼接，得到注入编码。字段顺序、宽度、table ID 和 padding 语义均固定并绑定。

这是位串的注入编码，不是整数乘法或子域嵌入的默认实现。表与查询必须使用完全相同的基和布局。

### 5.3 元组超容量时随机聚合

若用户确认此处启用 fallback，将元组拆为若干精确的 F128 coordinate chunks `u_0,...,u_{d-1}`，在原始字段固定后采样独立的 `eta_j ∈ E`，计算：

`compressed_tuple = sum_j eta_j * u_j`。

随后用独立的 grand-product challenge z 调用 permutation proof。聚合关系必须绑定回原始字段；不能让 prover 自由提交一组与原始 value/address/counter 无关的压缩值。

记录压缩与整个多重集合检验的 soundness。一个固定不同元组对的 `1/|E|` 碰撞界不等于整个 lookup proof 的总误差；M0/M4 必须给出包含集合规模与各层归约的组合界。

## 6. 拟定模块与验证接口

代码落在独立的 `lookup/` crate，预期包含以下职责；具体函数签名在 M0 核验依赖后确定：

| 模块 | 职责 |
| --- | --- |
| layout / encoding | 位宽、basis、packed storage、合法长度、padding 与 tuple 编码 |
| product_tree | 紧凑系数树、容量预算、求值切换与 E-valued tree |
| permutation | proof 消息、prover、verifier、两边 fingerprint 与 source claims |
| lasso | table/query/counter witness、读写集合、permutation 组合与局部约束 |
| claims | 明确 source 和布局的终端求值声明、PCS 对接边界 |
| tests / benches | 独立正确性 oracle、攻击输入、位宽曲线与阶段计时 |

首选从最高接口测试：permutation 接受且仅接受相同多重集合；lookup 接受且仅接受合法 table reads，并验证所有终端声明。优化内核另做少量必要的位序/边界差分测试。

F128、phi8、Fiat–Shamir challenger、线程池、多线性权重与适用的算术内核复用 Flock。现有 Mikros IP 模块不是公开接口，binary Hadamard 也没有任意位宽支持；不要假设这些接口已经可以直接调用。若需最小公开适配，在独立接入步骤处理，不在本轮计划中修改主任务文件。

## 7. 实施步骤与依赖

每一步都交付一个可独立验证的完整行为。

| 步骤 | 前置 | 交付与验收 |
| --- | --- | --- |
| M0 规格与最小可运行基线 | 用户确认 fallback 作用范围 | 固定数据/域语义与编码预算；建立小规模 conventional F128 permutation prover/verifier；输出明确的 source claims 和 soundness 清单 |
| M1 小位宽 permutation | M0 | 小规模紧凑系数树与完整反向验证贯通；和 independent grand-product/source oracle 一致；错误元素、重数、padding、消息篡改拒绝 |
| M2 容量切换 permutation | M1 | 明确卷积到 packed reduction 的桥接；跨容量自动切换至 E，全部父子与源输入链接验证；强制不同合法切换层得到同一语句结果 |
| M3 单表 indexed lookup | M0 | conventional permutation 后端驱动 Init/Write 与 Read/Final；整数计数增量/范围也进入证明；重复读取、同值不同地址和未访问表项通过 |
| M4 Bitwidth-aware Lasso | M2、M3 | 改接紧凑 permutation；精确 tuple 编码；若确认则实现超宽 tuple 随机聚合及其连接证明；返回完整源输入声明 |
| M5 工程优化与公平实验 | M4 | packed/SWAR、byte LUT、共享挑战表、流式树层与并行；与同语句 conventional F128 后端同场计时，全部样本验证 |
| M6 PCS 接入（独立范围） | M4、选定 PCS | 认证全部终端声明，测完整 lookup proof；不能混同 M5 的 PIOP-only 数据 |

M3 可在 M1/M2 开发期间使用 conventional permutation 后端推进；不需要等待底层优化完成。M5 的优化必须保持 reference/optimized 消息语义和 verifier 行为一致，任何协议变化另行命名。

## 8. 关键测试

- permutation：随机重排、重复值、不同 multiplicity、两端相同 padding、非二次幂长度、N=0/1 的明确定义、零 fingerprint 因子。
- 位宽与容量：合法基/域嵌入；1/8-bit 初始用例；总编码 127/128/129 bit；恰好装满和下一层首次超过容量；不可将带填充的末尾 bit 纳入有效语句。
- lookup：所有查询相同、均匀/偏斜查询、重复 table values、不存在值、错地址、地址越界、未访问条目、空查询。
- counter：伪造旧计数、跳号、wraparound、进位篡改、伪造终止计数、没有合法初始化来源的伪造访问链。
- 连接与 transcript：修改原始字段而保持压缩值/末端乘积、在看到挑战后替换 witness、错误 domain/位宽/长度、截断/多余消息；检查终端 source claims 全部被认证。
- 多线程：reference/optimized、1T/多线程、尾块和任务分块边界一致。对存在不同合法 proof 编码的算法，用独立语句验证而非不成立的逐字节相等要求。

## 9. 实验口径

比较对象是相同 permutation/lookup 语句的 conventional 全 F128 后端与 bitwidth-aware 后端；Flock degree-two zerocheck 只作为相应算术组件的对照，不能拿完整 Lasso 总时间直接除以单个 zerocheck 时间。

固定 E、输入、table、地址分布、线程数、hash、grinding 和 PCS 边界。参数矩阵覆盖查询数 N、表长 M、值/地址/counter 位宽、重复率、实际切换层，以及 tuple 是否使用随机压缩。

报告 end-to-end online prover、verifier、消息字节、独立进程峰值 RSS，以及 witness/counter 构造、紧凑树、切换求值、扩域树、局部约束和终端归约的阶段耗时。主计时包含所有 witness-dependent 预处理、动态构表与转置；输入生成另列。预热后至少五次，保存全部样本并核验。

## 10. 完成标准

M4 完成才表示两种请求的 PIOP 均已实现；只生成 witness 或比较两边 grand products 不算实现 permutation proof。M5 完成后才能报告 bitwidth-aware 的实际收益。M6 未完成时明确标注未含 PCS，不能称为完整已认证 SNARK。

实施记录：已交付小域嵌入、紧凑系数树、全 E 对照路径、逐层 GKR、permutation prover/verifier、Lasso 读写集合、Mikros 进位约束和所有源求值连接。M2 使用通用 degree-three GKR 验证求值后的系数层乘法，因此尚未实现论文全部系数层 packed reduction 的算术优化。首版 PIOP 结果保留在 RESULTS.md。后续按用户要求接入的 WHIR PCS、源 Booleanity 和全部 opening 认证见 WHIR.md / WHIR_RESULTS.md；更多 SIMD 优化仍未列为已完成。
