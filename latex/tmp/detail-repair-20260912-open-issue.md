# 置换归约：本轮未替换的证明缺口

本轮已修正 Hadamard 的张量查询、相应乘法计数和比较表的技术条件；Our Contributions 保持原文。下面的问题不能通过补一个归一化因子解决。为避免将不同构造及不同复杂度界直接写入正文，置换算法与其复杂度命题暂未替换。

## 1. 行树：可用同点的三次 sumcheck 修复

对于长度为 n=2^ell 的声明 c=<xi,u∘v>，当前 ordinary inner-product reduction 产生对 u 和 v∘xi 的查询。将两项任意合并，一般不再得到一个 MLE 张量。

有效的替代步骤是，对公开权重的多线性扩展与两个子向量的多线性扩展之积运行 sumcheck：

\[
c=\sum_{X\in\{0,1\}^{\ell}}\widetilde\xi(X)\widetilde u(X)\widetilde v(X).
\]

每个变量的次数至多为 3，两个子声明均位于同一点 t。先固定子值 v_L、v_R，再采样 eta，合并为：

\[
c'=(1-\eta)v_L+\eta v_R
  =\langle u\mathbin\|v,\operatorname{ts}(\eta\mathbin\|t)\rangle.
\]

每层的错误界为 (3 ell+1)/|E|。设 h=log_2 m，整棵行树的错误界为 [3h(h-1)/2+h]/|E|。此步骤已有 Section 6.2 的同点 product-tree reduction 可复用。它不解决下面的列树问题。

## 2. 列树：系数乘积不是 Hadamard 乘积

在第 t 层令 d=2^t。两项首一多项式的乘积为：

\[
\left(Z^d+\sum_{j=0}^{d-1}A_jZ^j\right)
\left(Z^d+\sum_{k=0}^{d-1}B_kZ^k\right)
=Z^{2d}+Z^d\sum_j(A_j+B_j)Z^j
+\sum_{j,k}A_jB_kZ^{j+k}.
\]

最后一项是卷积。当前 `4-block.tex` 和 `alg/alg-permutation.tex` 没有给出将其转换为 Algorithm 2 的明确编码。即使复制输入，将所有 A_j B_k 变成 Hadamard 关系，终端也通常是系数的任意线性查询，而非同一挑战 z 处的多项式求值；递归的查询类型还需要单独证明。

第 t 层实际相乘的节点数为 n_t=N/2^(t+1)，总和为 N-m。`appx-proof.tex` 使用的短内积长度 m/2^(log_2 kappa-t) 不能由现有文字推出。因此当前列树的 soundness 求和、10N/kappa 次扩域乘法，以及依赖这一归约的整体界，尚不能确认为已证明。

## 3. 可证明的保守替代及其代价

系数表示可以只用于构造树，再在每个实际求值层运行上述同点三次 sumcheck。这样查询接口闭合，但证明阶段需要 O(N) 次扩域运算，即 O(N kappa^2) 次基域运算；普通驻留实现需要 O(N kappa) 个基域坐标。

这会改变当前宣称的 bitwidth-aware 置换复杂度。若保留 O(N kappa) 目标，则需要新的、显式的列树归约及递归查询不变量，而非沿用当前成本求和。

## 本轮验证范围

- 独立代数测试覆盖 F2、F3、F5、F17、F4、F8、F16。
- 663 个逐坐标恒等式测试，包括 52 个归一化因子为零的情况；700 个矩阵终端查询测试通过。
- 24 个主文档实际引入的 TeX 文件通过括号、环境、citation 和 label 静态检查。
- Our Contributions、置换算法、Section 6 和 ref.bib 与本轮修改前快照一致。
- 本地无 TeX 编译器，未进行 PDF 编译或版面验证。
