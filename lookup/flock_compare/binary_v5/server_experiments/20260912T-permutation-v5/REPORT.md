# Permutation v5：寄存器融合与受预算约束的树层复用

N26 筛选按预先固定的 measured mean core 最小规则选择 `combined`；精确平局按名称。选择记录于 `2026-09-12T14:59:50Z`，早于独立 N30 的第一条命令，源码和 executable 未变。保留其余候选和反向结果，不按 N30 的有利样本重新选择。

本轮只测 width=1：N30 tuple=31 bits/K32。previous 为冻结 binary v4；register 改寄存器内 fold+next-message；reuse 加树层池；combined 组合两项。Mikros 的相同输入须保持相同证明字节和 transcript。公共默认仍为 Engine::Fused，候选通过显式 Engine 选择，筛选结果不自动改变默认。Flock 是 snapshot product_gkr::prove_batched；没有 Hadamard skip 对齐，不声称同安全等级，未包含 PCS/grinding。

64 threads 固定 `0-47,48-55,72-79`，48 物理核心+16 SMT sibling，两 NUMA 各32逻辑CPU；Linux first-touch，共享非独占服务器。Rust nightly-2026-03-21、target-cpu=native；所有 case 同一 executable。

已有 Flock 全局 F128 scratch pool 仍由 previous、新候选和 Flock 共享。新增树层池彼此独立：slot0用于 reuse，slot1用于 combined，各限32 GiB allocation capacity；预算不是已驻留内存。每次都重写数学值。树层归还可能保留容量而非立即归还操作系统；最终同步清空时间明确排除在 proof timer 外并在下表报告。

## 主计时与消融

N20 smoke五系统各1次、无预热；N26四系统各1预热+8正式，Williams ABDC/BCAD/CDBA/DACB重复两轮，位置与相邻前驱均衡而非覆盖24排列。N30旧版/选中候选/Flock各1预热+6正式，完整六种排列。每项统计仅用本组正式样本，不混合不同N、独立RSS进程或旧binary时序。

| case | 系统 | core median [min,max] s | core mean s | total median s | verifier ms | source median s |
|---|---|---:|---:|---:|---:|---:|
| smoke-w1-n20-64t | Mikros binary v4 | 0.027 [0.027,0.027] | 0.027 | 0.032 | 0.199 | 0.003 |
| smoke-w1-n20-64t | 寄存器 fold/message | 0.024 [0.024,0.024] | 0.024 | 0.028 | 0.273 | 0.003 |
| smoke-w1-n20-64t | 树层池复用 | 0.021 [0.021,0.021] | 0.021 | 0.025 | 0.201 | 0.003 |
| smoke-w1-n20-64t | 寄存器与树层池组合 | 0.019 [0.019,0.019] | 0.019 | 0.023 | 0.197 | 0.002 |
| smoke-w1-n20-64t | Flock product_gkr batched | 0.036 [0.036,0.036] | 0.036 | 0.039 | 0.340 | 0.001 |
| screening-w1-n26-64t | Mikros binary v4 | 0.800 [0.786,0.851] | 0.805 | 0.883 | 0.345 | 0.084 |
| screening-w1-n26-64t | 寄存器 fold/message | 0.824 [0.804,0.864] | 0.826 | 0.896 | 0.343 | 0.084 |
| screening-w1-n26-64t | 树层池复用 | 0.588 [0.578,0.672] | 0.607 | 0.663 | 0.347 | 0.084 |
| screening-w1-n26-64t | 寄存器与树层池组合 | 0.598 [0.591,0.627] | 0.602 | 0.667 | 0.343 | 0.083 |
| main-w1-n30-64t | Mikros binary v4 | 9.371 [8.400,9.830] | 9.298 | 10.889 | 0.460 | 2.033 |
| main-w1-n30-64t | 寄存器与树层池组合 | 6.384 [6.042,6.861] | 6.402 | 8.077 | 0.464 | 1.993 |
| main-w1-n30-64t | Flock product_gkr batched | 12.126 [11.594,12.846] | 12.102 | 13.280 | 0.782 | 0.591 |

比值为分子/分母，小于1表示分子较快。先按同组run/input逐样本配对；均值、中位数、配对中位数全部保留。

| case | 分子/分母 | metric | 两中位数之比 | paired median [min,max] | 分子较快/样本数 |
|---|---|---|---:|---:|---:|
| smoke-w1-n20-64t | register/previous | prover_core_ms | 0.8689 | 0.8689 [0.8689,0.8689] | 1/1 |
| smoke-w1-n20-64t | register/previous | prover_total_ms | 0.8631 | 0.8631 [0.8631,0.8631] | 1/1 |
| smoke-w1-n20-64t | register/flock | prover_core_ms | 0.6505 | 0.6505 [0.6505,0.6505] | 1/1 |
| smoke-w1-n20-64t | register/flock | prover_total_ms | 0.7075 | 0.7075 [0.7075,0.7075] | 1/1 |
| smoke-w1-n20-64t | reuse/previous | prover_core_ms | 0.7566 | 0.7566 [0.7566,0.7566] | 1/1 |
| smoke-w1-n20-64t | reuse/previous | prover_total_ms | 0.7856 | 0.7856 [0.7856,0.7856] | 1/1 |
| smoke-w1-n20-64t | reuse/flock | prover_core_ms | 0.5664 | 0.5664 [0.5664,0.5664] | 1/1 |
| smoke-w1-n20-64t | reuse/flock | prover_total_ms | 0.6439 | 0.6439 [0.6439,0.6439] | 1/1 |
| smoke-w1-n20-64t | combined/previous | prover_core_ms | 0.6832 | 0.6832 [0.6832,0.6832] | 1/1 |
| smoke-w1-n20-64t | combined/previous | prover_total_ms | 0.6982 | 0.6982 [0.6982,0.6982] | 1/1 |
| smoke-w1-n20-64t | combined/flock | prover_core_ms | 0.5115 | 0.5115 [0.5115,0.5115] | 1/1 |
| smoke-w1-n20-64t | combined/flock | prover_total_ms | 0.5723 | 0.5723 [0.5723,0.5723] | 1/1 |
| smoke-w1-n20-64t | combined/register | prover_core_ms | 0.7863 | 0.7863 [0.7863,0.7863] | 1/1 |
| smoke-w1-n20-64t | combined/register | prover_total_ms | 0.8089 | 0.8089 [0.8089,0.8089] | 1/1 |
| smoke-w1-n20-64t | combined/reuse | prover_core_ms | 0.9031 | 0.9031 [0.9031,0.9031] | 1/1 |
| smoke-w1-n20-64t | combined/reuse | prover_total_ms | 0.8888 | 0.8888 [0.8888,0.8888] | 1/1 |
| screening-w1-n26-64t | register/previous | prover_core_ms | 1.0292 | 1.0239 [0.9542,1.0968] | 1/8 |
| screening-w1-n26-64t | register/previous | prover_total_ms | 1.0140 | 1.0135 [0.9461,1.0778] | 1/8 |
| screening-w1-n26-64t | reuse/previous | prover_core_ms | 0.7353 | 0.7475 [0.6794,0.8429] | 8/8 |
| screening-w1-n26-64t | reuse/previous | prover_total_ms | 0.7509 | 0.7625 [0.6939,0.8417] | 8/8 |
| screening-w1-n26-64t | combined/previous | prover_core_ms | 0.7475 | 0.7531 [0.6986,0.7728] | 8/8 |
| screening-w1-n26-64t | combined/previous | prover_total_ms | 0.7550 | 0.7652 [0.7117,0.7854] | 8/8 |
| screening-w1-n26-64t | combined/register | prover_core_ms | 0.7263 | 0.7300 [0.6933,0.7502] | 8/8 |
| screening-w1-n26-64t | combined/register | prover_total_ms | 0.7446 | 0.7505 [0.7141,0.7721] | 8/8 |
| screening-w1-n26-64t | combined/reuse | prover_core_ms | 1.0165 | 1.0112 [0.9082,1.0690] | 3/8 |
| screening-w1-n26-64t | combined/reuse | prover_total_ms | 1.0056 | 1.0056 [0.9193,1.0540] | 3/8 |
| main-w1-n30-64t | combined/previous | prover_core_ms | 0.6812 | 0.6759 [0.6453,0.7465] | 6/6 |
| main-w1-n30-64t | combined/previous | prover_total_ms | 0.7418 | 0.7363 [0.7129,0.8003] | 6/6 |
| main-w1-n30-64t | combined/flock | prover_core_ms | 0.5265 | 0.5395 [0.4821,0.5650] | 6/6 |
| main-w1-n30-64t | combined/flock | prover_total_ms | 0.6082 | 0.6217 [0.5702,0.6461] | 6/6 |

register/previous 与 reuse/previous 分别衡量两项独立改动；combined/register 和 combined/reuse 检查组合增量。不把其他候选或旧批次的时间相减来分配本组收益。小样本胜场不保证统计显著性或跨环境稳定性。

## 同次 prover 的阶段

阶段均值使用本组正式样本；构造按解码/compact构造/求值/融合构造合计。“层归还/释放”包含同步释放或归还池的耗时；归还较快并不意味着内存释放免费。

| case | 系统 | 阶段 | mean s | median [min,max] s |
|---|---|---|---:|---:|
| screening-w1-n26-64t | previous | 解码/首层构造/求值（含融合） | 0.110 | 0.109 [0.108,0.113] |
| screening-w1-n26-64t | previous | 扩域上层建树 | 0.043 | 0.042 [0.042,0.044] |
| screening-w1-n26-64t | previous | GKR 消息/折叠 | 0.431 | 0.429 [0.419,0.451] |
| screening-w1-n26-64t | previous | 层归还/释放 | 0.208 | 0.205 [0.198,0.234] |
| screening-w1-n26-64t | previous | prover 内源检查 | 0.013 | 0.013 [0.013,0.014] |
| screening-w1-n26-64t | previous | 缓冲与物化 | 0.000 | 0.000 [0.000,0.000] |
| screening-w1-n26-64t | register | 解码/首层构造/求值（含融合） | 0.111 | 0.111 [0.109,0.112] |
| screening-w1-n26-64t | register | 扩域上层建树 | 0.044 | 0.043 [0.040,0.052] |
| screening-w1-n26-64t | register | GKR 消息/折叠 | 0.443 | 0.439 [0.430,0.465] |
| screening-w1-n26-64t | register | 层归还/释放 | 0.215 | 0.210 [0.204,0.231] |
| screening-w1-n26-64t | register | prover 内源检查 | 0.014 | 0.014 [0.013,0.014] |
| screening-w1-n26-64t | register | 缓冲与物化 | 0.000 | 0.000 [0.000,0.000] |
| screening-w1-n26-64t | reuse | 解码/首层构造/求值（含融合） | 0.112 | 0.104 [0.095,0.142] |
| screening-w1-n26-64t | reuse | 扩域上层建树 | 0.052 | 0.049 [0.047,0.061] |
| screening-w1-n26-64t | reuse | GKR 消息/折叠 | 0.428 | 0.423 [0.411,0.458] |
| screening-w1-n26-64t | reuse | 层归还/释放 | 0.000 | 0.000 [0.000,0.000] |
| screening-w1-n26-64t | reuse | prover 内源检查 | 0.014 | 0.014 [0.013,0.017] |
| screening-w1-n26-64t | reuse | 缓冲与物化 | 0.000 | 0.000 [0.000,0.000] |
| screening-w1-n26-64t | combined | 解码/首层构造/求值（含融合） | 0.097 | 0.097 [0.097,0.100] |
| screening-w1-n26-64t | combined | 扩域上层建树 | 0.048 | 0.046 [0.045,0.058] |
| screening-w1-n26-64t | combined | GKR 消息/折叠 | 0.441 | 0.441 [0.433,0.450] |
| screening-w1-n26-64t | combined | 层归还/释放 | 0.000 | 0.000 [0.000,0.000] |
| screening-w1-n26-64t | combined | prover 内源检查 | 0.015 | 0.013 [0.013,0.026] |
| screening-w1-n26-64t | combined | 缓冲与物化 | 0.000 | 0.000 [0.000,0.000] |
| main-w1-n30-64t | previous | 解码/首层构造/求值（含融合） | 2.465 | 2.477 [2.418,2.492] |
| main-w1-n30-64t | previous | 扩域上层建树 | 0.347 | 0.349 [0.336,0.356] |
| main-w1-n30-64t | previous | GKR 消息/折叠 | 3.453 | 3.381 [3.141,3.784] |
| main-w1-n30-64t | previous | 层归还/释放 | 2.907 | 3.009 [2.289,3.175] |
| main-w1-n30-64t | previous | prover 内源检查 | 0.123 | 0.121 [0.109,0.150] |
| main-w1-n30-64t | previous | 缓冲与物化 | 0.000 | 0.000 [0.000,0.000] |
| main-w1-n30-64t | combined | 解码/首层构造/求值（含融合） | 2.355 | 2.287 [2.234,2.687] |
| main-w1-n30-64t | combined | 扩域上层建树 | 0.343 | 0.344 [0.292,0.393] |
| main-w1-n30-64t | combined | GKR 消息/折叠 | 3.580 | 3.670 [3.322,3.680] |
| main-w1-n30-64t | combined | 层归还/释放 | 0.000 | 0.000 [0.000,0.000] |
| main-w1-n30-64t | combined | prover 内源检查 | 0.123 | 0.121 [0.104,0.148] |
| main-w1-n30-64t | combined | 缓冲与物化 | 0.000 | 0.000 [0.000,0.000] |

## 独立冷/暖进程与保留容量

cold进程=无预热、1次proof；warm进程=1次预热+1次正式proof，表内计时只取正式proof。二者都是单系统独立进程，但warm的VmHWM仍是包含预热的进程历史峰值。VmHWM在prover返回、verify/source前读取。不使用多系统进程的累计VmHWM比较单系统内存。

| case | core s | total s | prover后VmHWM GiB | prover后VmRSS GiB | cleanup前两树池保留 GiB | proof bytes |
|---|---:|---:|---:|---:|---:|---:|
| cold-rss-w1-n30-64t-previous | 10.712 | 12.283 | 49.740 | 31.760 | 0.000 | 44208 |
| cold-rss-w1-n30-64t-combined | 6.288 | 7.959 | 63.759 | 63.759 | 31.998 | 44208 |
| cold-rss-w1-n30-64t-flock | 13.197 | 14.337 | 158.001 | 74.026 | 0.000 | 16176 |
| warm-rss-w1-n30-64t-previous | 10.414 | 11.975 | 63.736 | 31.760 | 0.000 | 44208 |
| warm-rss-w1-n30-64t-combined | 6.434 | 8.111 | 63.759 | 63.759 | 31.998 | 44208 |
| warm-rss-w1-n30-64t-flock | 13.107 | 14.284 | 164.000 | 80.088 | 0.000 | 16176 |

上述每种状态只有一次正式proof，冷暖差异属于诊断，不是稳定冷启动保证。保留容量按pool实际capacity计费；VmRSS还受现有全局scratch、allocator与运行时影响。全部pool原始计数、每次proof快照和warmup快照保留在SUMMARY.json。

## 显式批末清理

所有原始输入、转换输入和proof对象离开作用域后，先清两个树池，再清共享F128 scratch；这段清理在core/total之外，因此单独给出，不能将其停表后发生解释为零成本。batch wall包含warmup、输入生成、verify、完整source核验与proof写出，不是proving-only，也不与逐proof中位数拼接。

| case | 树池清空 s | shared scratch清空 s | 全部cleanup s | 清理前/后VmRSS GiB | batch含cleanup s |
|---|---:|---:|---:|---:|---:|
| smoke-w1-n20-64t | 0.000 | 0.005 | 0.005 | 0.124 / 0.059 | 0.181 |
| screening-w1-n26-64t | 0.414 | 0.079 | 0.493 | 4.912 / 0.162 | 34.853 |
| main-w1-n30-64t | 2.885 | 3.680 | 6.566 | 72.010 / 0.072 | 315.323 |
| cold-rss-w1-n30-64t-previous | 0.000 | 1.495 | 1.495 | 14.011 / 0.011 | 19.297 |
| cold-rss-w1-n30-64t-combined | 3.378 | 1.069 | 4.447 | 46.008 / 0.010 | 17.628 |
| cold-rss-w1-n30-64t-flock | 0.000 | 2.220 | 2.220 | 32.026 / 0.026 | 22.244 |
| warm-rss-w1-n30-64t-previous | 0.000 | 1.393 | 1.393 | 14.011 / 0.011 | 36.177 |
| warm-rss-w1-n30-64t-combined | 3.139 | 1.017 | 4.156 | 46.008 / 0.010 | 31.459 |
| warm-rss-w1-n30-64t-flock | 0.000 | 2.551 | 2.551 | 38.010 / 0.010 | 42.188 |

SUMMARY.json保留batch清理前/后完整时钟与两池原始stats。清空后RSS是否立即下降由实际记录决定，不将retained bytes等同于RSS差，也不推断cache/NUMA/allocator原因。

## 完整性与边界

接受9个case、71份证明；另核验0份干扰尝试证明、仅排除其计时。native测试通过17项，110个证据文件和415个源码/构建文件SHA256核验通过。

排除尝试：无。等待有上限、记录检测到的同用户benchmark/build干扰，不终止别的进程。未检测到受监测进程不表示主机独占。

每份proof先执行verifier、claims/transcript一致性及完整源检查，再发出verified。下载后检查实际proof bytes、Mikros各变体的原始文件SHA256/proof digest一致、JSON/JSONL、执行顺序与输入身份。Flock与Mikros不要求相同证明摘要。N30消息长度仍为Mikros44,208/Flock16,176 bytes。

输入生成/摘要不计入core/total；系统转换计入total；源认证单列且完整执行。SUMMARY还给出逐样本total+verify+source统计（排除批末cleanup），它仍不是PCS或完整SNARK。

Source SHA256：`21989af1a3db2b3c1e4a62ac6115d3806b8b7a3da1a8650acad01cf89350c249`。

Executable SHA256：`57cc3e617f657fceddfdac55218c1fcae0186d804d51e8a3c75f231020a0e7df`。

[选择](selection.json)、[命令](results/commands.json)、[环境](results/environment.json)、[接受清单](results/accepted.json)、[等待](results/waits.json)、[源码](source_manifest.json)、[SUMMARY.json](SUMMARY.json)、[summary.csv](summary.csv)。
