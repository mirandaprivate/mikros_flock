# 全文写作修订说明

## 范围与参照

本轮修订覆盖 main.tex 实际包含的摘要、Sections 1–8、三个附录及六个伪代码，共修改 19 个 TeX 文件。未修改未被主文件使用的旧稿、参考文献数据或实验数据。

以 literature 中的 BaseFold（CRYPTO 2024）、Blaze（EUROCRYPT 2025）及 binary-tower succinct arguments（EUROCRYPT 2025）为主要语言与组织参照。采用 academic-research-suite 的写作风格校准流程，提取其问题陈述、归约解释、协议组织和证明结构的共性，未复制论文的特色措辞。

主要调整如下：

- 摘要和引言先明确小 witness 与大 challenge field 的成本差异，再给出核心归约：长度 N 的基域关系经 polynomial packing 变为长度 m=N/kappa 的扩域关系。
- Technical Overview 按“保留关系的多项式恒等式—随机求值—短向量证明—固定 commitment 的认证”展开，压缩重复动机。
- Building Blocks 先说明编码保留的量，再描述协议与复杂度；PCS 明确固定基底 packing 与挑战相关 packing 的区别。
- Section 6 先解释单实例内 packing 与同一电路跨实例 packing 的结构差异，再给出 gate/copy constraints 和组合界。
- 附录证明统一为 completeness、soundness、efficiency 的局部结构，保留每个证明独立的 subsection。伪代码加入简短阶段注释，并全部使用行内数学。
- 实验节补充现有表格的分析，区分 component measurement 与 end-to-end proof，区分 timing 与 RSS；明确 64-bit prover 未获得加速。
- 摘要删除未有数据支持的 XX 倍端到端加速占位，改为现有 8-bit component 数据；没有增加完整 zkML 或端到端性能主张。

## 保留与局部修正

保留原有符号体系、有效 labels、citation keys、定理/命题的复杂度与 soundness 界。Sections 5–6 的显示公式除空白外未改变。补充 O(kN) 重复检查成本所需的 m>=kappa 条件，并将 binary PCS 连接描述限定为兼容的 code-based PCS。

修正了可从当前正文直接确认的局部错误：多余的定界命令、log_2 kappa 的排版笔误、置换树中应为 d 的两处 w，以及 IP 终端点的所属域。依据 Fact ip 的 same-point sumcheck 接口，删除了旧伪代码/证明中误留的终端挑战逆元；Laurent packing 中的负幂保持不变。

## 需要单独数学审校的问题

以下问题存在于修订前的稿件中。本轮未据此重设计协议，也未改动对应的效率与 soundness 界。因此，本次语言修订不构成对这些界的数学认证。

### 1. 终端查询的基底表示

Definition tensorspan 定义 ts(x) 的每个因子为 (1-x,x)。但 alg/alg-ip.tex 将 monomial packing 的 (1,x) 型因子直接当作 MLE 点；二者并不相同。

同样，alg/alg-hadamard.tex 的最终 MLE 点及 appx-proof.tex 中的对应等式使用了 ts(z) Hadamard ts(y)=ts(z Hadamard y)。左边单个因子的第一坐标为 (1-z)(1-y)，右边则为 1-zy，通常不相等。alg/alg-permutation.tex 也将一般 tensor factors 直接写成一个标准 MLE point，需要一并检查。

Section 5 已允许任意两坐标 tensor factors，因而有现成的接口可用于统一这些终端查询；仍需同步伪代码返回值、附录证明和组合调用，而非仅改称谓。

### 2. 置换列乘积树的系数层归约

位置：4-block.tex 中 “Represent each entry ... by its base-field coefficients” 段；alg/alg-permutation.tex 中 coefficient-packing reduction 步骤；appx-proof.tex 的 permutation proof。

P(z)Q(z) 包含不同系数之间的交叉项。将两个多项式的系数向量逐坐标相乘只能保留对角项，单纯把求值权重吸收到 tensor query 不能证明它等价于当前 Hadamard 协议的基域输入关系。附录当前验证的是求值后的扩域关系，尚未给出所声称的系数层调用及其成本推导。

### 3. 置换行乘积树的随机合并误差

alg/alg-permutation.tex 每层随机合并两个终端 claim，但 appx-proof.tex 中行树的计数仅累加 Fact ip 的 2 log(length)/|E|。需要显式分析合并引入的失败事件，并核对它是否已被现有 epsilon_GP 覆盖；Section 6.2 对类似合并有单独计数，可用作核对参照。本轮未直接修改 soundness 公式。

## 验证与恢复

静态检查结果：

- 活动 include 图包含 23 个 TeX 文件。
- 112 个 labels，无重名；194 处交叉引用均可解析。
- 30 个 citation keys 均存在，修订前后集合相同；ref.bib 字节未变。
- 括号、数学定界符和 LaTeX 环境配对通过。
- proof 环境中没有 paragraph 标题；六个伪代码没有显示公式。
- 63 处公式引用均使用 Eq.~\eqref；207 个显示公式已检查引导冒号与句末标点。
- 实验表中的数值 token 未变；未使用的旧稿未变。

检查脚本：tmp/validate_writing_revision.py。该脚本只读，不改写源文件。显示公式的差异已经逐项核查，主要是标点、转行内数学、删除重复证明和上述局部修正。

本轮没有完成 TeX 编译或 PDF 视觉检查，仍需在 Overleaf 重新编译确认排版。

修改前快照位于 tmp/writing-revision-2026-09-08-before/。该快照保留了本轮开始时的源文件，不覆盖此前的用户修改。所有修订均仅保存在本地。
