# LaTeX 对照与 N26 工程优化（2026-09-15）

本轮以当前仓库 LaTeX、现有 binary 实现和相同 FLOCK 原生 PCS 为边界。
新内核均为可独立启用的实验选项，默认保留参考路径；性能结论以服务器消融结果为准。
不修改用户的 LaTeX，不改变现有证明消息、挑战顺序、终端声明或承诺布局。

## 文稿对照

正文入口是 `latex/main.tex` 引入的 `4-block.tex` 和 `6-together.tex`；
`5-pcs.tex` 由 appendix 引入。`6-together.tex` 的 Binary-Field Instantiation
位于 `\iffalse` 内，不能当作当前生效的正文。

`6-together.tex` 的 multiple 协议将 128 个相同 wiring、独立输入的实例按列打包。
它保留三条已承诺 wire、一个未承诺的 virtual product、五条 selector 终值、
用于认证 virtual product 的 Hadamard proof，以及两棵扩域 product tree。
这与当前 multiple E2E 的算法结构对应。

需要区别具体参数：`4-block.tex` 的小域参数算法从所需 `kappa=128`、`q=2`
出发选择 `t=13` 并将 kappa 调整到 130；代码使用 GF256 中间域和 kappa=128。
后者满足中间域嵌入和插值所需的 `8 | 128`、`256 >= 255`，
但不是逐字实现文稿的那条取参算法。这里继续测量原有 binary 实例化，
不因工程优化而替换域、packing 或 PCS；也不将当前 binary 测量直接等同于
正文中带 `q >= 2*kappa-1` 前提的复杂度定理。

## 本轮实现

### Direct W-prime：张量分段与四路乘法

在 native mixed PCS 的 direct 部分，行权重分解为 `lo[v] * hi[u]`。
原实现逐 slot、逐 direct group 计算 `s[c0] * (lo[v] * hi[u])`。
新实现将 `s[c0] * hi[u]` 提到段外，每个 slot 只做一次乘法，
通过四路 AVX-512 / VPCLMUL 累加到 W-prime，不新增完整权重表。
段边界同时受低位 tensor 和列位 c0 限制；小域、未对齐输入、尾部均有参考路径。

开关：`FLOCK_DIRECT_WP_SIMD=1`，对应 harness `--direct-wp-simd`。
它要求本实验同时开启 `--packed-direct`。同一个内核对双方开放；
当前 multiple 的 Mikros 有三个 direct 行点组，FLOCK 有一个，
因此分别测量双方收益，不能只测 Mikros 后外推相对优势。

### Mikros multiple gate：检查 selector 恒等式后重排残差

当完整 selector 周期逐项满足
`qR=qL=q`、`qM=1+q`、`qO=1`、`qC=0` 时，残差精确等于

```
q*(L+R+M) + (M+O).
```

新内核先做 XOR 合并，再用四路 VPCLMUL 累加，减少重复域乘法。
检查的是 selector 本身，不假定 witness 满足 gate，也不假定 M 已被正确认证。
不满足任一 selector 恒等式时回退原有通用内核。
四条 wire 的折叠、所有终值、五条 public claims 和 Hadamard 认证全部保留。

开关：`MIKROS_GATE_SELECTOR_SIMD=1`，对应 harness `--gate-selector-simd`。
它只影响 Mikros multiple gate。FLOCK 已有该固定 AND/XOR 电路的专用规约。

### 第二轮：gate 折叠和消息融合

`MIKROS_GATE_FUSION=1` 保留四条 wire 的所有折叠结果，但将输出按 tile 写入，
马上在已初始化的 tile 上累计下一轮消息。selector 先按刚收到的挑战折叠，
下一轮的 cubic 转换、消息发送和挑战采样仍在原来的位置进行。
tile 同时对齐输出相邻对和低位 equality tensor，避免跨块遗漏或重复计权。
N26 理论上可少从大数组再次读取约 4 GiB 的折叠结果；是否变快由独立实验判断。

### 第二轮：W-prime 避免覆盖前的完整清零

`FLOCK_WP_UNINIT=1` 对 Mikros 和 FLOCK 同样启用。
N26 multiple 的 W-prime 输出有 2 GiB；原来先完整清零，再逐 slot 写计算结果。
新实现只预留容量，各 worker 首次写满自己的区域，在该区域全部初始化后才允许读取，
等待所有 worker 完成后才设置向量长度。证明数学、RS/direct maps、释放时机和 q-prime 不变。
诊断分别计量 reserve/zero-init 和 fill，避免把初始化时间误归因于域乘法。

v10 单独比较 reference、仅 gate fusion、仅 W-prime first-write、两者组合，
关闭 v9 的两个算术开关。共享的 W-prime 优化须按双方实测的相对收益决定是否推荐。

## 后续优先级（尚未实现或测得收益）

| 方向 | 当前重复工作 | 可测的工程改法 |
|---|---|---|
| Virtual product 首轮按块生成 | 先物化 N 个 F128 的 M | 初轮消息和第一次折叠逐块计算 AND，仅保留折叠后的 M；N26 初始数组为 1 GiB |
| Hadamard 行权重按块生成 | 物化完整 N 项 tensor 权重 | 两段小 tensor 加按块生成；需核算重复生成乘法与访存节省，N26 完整表为 1 GiB |
| Copy 的两个 wire 终值一起求值 | 对同一三条 wire 分别扫描 | 同一 tile 累加两个点，理论可少读约 3 GiB；该阶段占比较小 |

这些改法只针对 Mikros 当前路径。GFNI 插值聚合、packing 与首轮 IP 消息融合、
IP / product-tree 折叠融合、隐式叶子和 copy 缓存已经存在，不计为本轮新增。
所有 XOR 门的 virtual product 仍需要 Hadamard 认证，不能因为 selector 为零就省略。

## 减少认证声明属于另一个实验

把五条 selector 只认证为一个加权和、对固定 selector 解析验证、
或者让两棵 copy tree 共用终点，都将修改当前终端认证或挑战合同。
这些选项不能归入本轮证明字节保持的工程消融。
三条普通 wire query 的任意系数加零 padding 通常不形成一个 rank-one MLE 点；
Hadamard 两条 weighted query 的行权重和列投影也不能直接套用 packed-direct。

## 测量边界

服务器 64 threads、native GFNI/AVX-512/VPCLMUL；N=2^26。
single 为一个 N 门电路；multiple 为 128 个各 N 门电路，总计 2^33 个 binary constraints。
每轮四臂均开启 packed-direct，并在该轮使用同一个冻结源码和可执行文件。
v9 比较 reference、仅 W-prime SIMD、仅 selector SIMD、两者组合；
v10 比较 reference、仅 gate fusion、仅 W-prime 首次写入、两者组合。
采用 system allocator 和既有 discard/recommit 生命周期；online 包含 commit、
recommit、动态转换、PIOP、PCS、release、序列化，offline 不计入。
主测量使用每臂三次独立新进程，trace 运行单独用于诊断。
完整证明字节及 hash、PIOP、terminal claims、承诺、PCS 参数、声明数须跨臂一致。
各阶段中位数不必加和为总时间中位数；未修改阶段的波动不能归因于新内核。

## 已完成的实测结论

两轮共 96 个 N26 主样本、16 个 N26 单次诊断、16 个 N14 pilot 均验证通过。
v9 的 214 项、v10 的 220 项 native tests 均通过，各有一项原有 microbenchmark 忽略。
跨各臂的同一 system/mode，完整证明 hash 和长度、PIOP、terminal claims、承诺、
PCS 参数及声明数都一致；参考证明也与前一版匹配。

本轮 N26 multiple 建议先只启用 `--packed-direct --wp-uninit`。
Mikros 的 witness PCS 主样本中位数从 5.7081s 降到 4.7468s；
online 从 19.5677s 降到 17.4973s，同配置 FLOCK 为 23.1933s，F/M 约 1.326。
Mikros 的约 10.58% online 降幅还包含未修改上游阶段的波动，不能全部归因于新内核。
单次诊断中，双方 W-prime 均约省 0.76s：Mikros 996.42→232.42ms，
FLOCK 1026.40→257.72ms；初始化下降后，fill 耗时基本相当。

gate fusion 的独立诊断降低 sumcheck wrapper 时间 537.957→457.911ms，
但主样本中组合配置不优于仅 W-prime，因此保留独立开关，不把全部开关默认开启。
两个 v9 算术开关也继续默认关闭。single 的 Mikros online 基本不变，
所选配置为 Mikros 5.1965s、FLOCK 9.3977s；只有 gate fusion 对 single 是纯负对照，
W-prime 首次写入也能作用于 single 较小的矩形输出。

当前尚未达到整体 2× FLOCK。剩余主要成本包括 PCS/commit/recommit/release，
Mikros 专属的后续工程方向是上表中的 virtual-product 和权重表按块生成，
以及在 gate 和 Hadamard 之间复用缓冲；这些尚未实现或测量。
详细逐阶段数据、负结果和复现方法见根目录 `e2e_n26_further_optimization_compare_flock.log`。
