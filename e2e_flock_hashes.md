# Mikros 与 FLOCK skip=7：三种原生 hash 工作负载

同一服务器、64 线程；每项 256 个独立调用。每种系统分 3 批运行，每批 1 次预热和 6 次正式测量。
下表 prover/verifier time 为 18 次正式测量中位数；proof size 包含承诺和完整证明；RAM 为三批进程峰值中的最大值。

| 工作负载 | 系统 | Prover ms | Proof bytes | Proof KiB | Verifier ms | RAM MiB |
|---|---|---:|---:|---:|---:|---:|
| sha256 | Mikros | 60.826 | 376,538 | 367.713 | 22.687 | 596.637 |
| sha256 | FLOCK skip=7 | 89.389 | 313,947 | 306.589 | 16.084 | 636.363 |
| keccak | Mikros | 85.749 | 389,434 | 380.307 | 39.330 | 56.875 |
| keccak | FLOCK skip=7 | 125.247 | 296,803 | 289.847 | 22.403 | 122.535 |
| blake3 | Mikros | 46.522 | 364,178 | 355.643 | 22.015 | 629.008 |
| blake3 | FLOCK skip=7 | 60.065 | 286,579 | 279.862 | 11.436 | 650.777 |

FLOCK/Mikros 的 prover time 比值：sha256 **1.470×**；keccak **1.461×**；blake3 **1.291×**。

正式样本 prover time 范围（ms）：
- sha256: mikros 32.449–73.247；flock 59.085–111.918
- keccak: mikros 50.162–111.867；flock 89.087–159.731
- blake3: mikros 31.769–58.599；flock 40.741–72.421

各批进程峰值 RSS（MiB，依批次顺序）：
- sha256 / mikros: 596.637, 596.566, 596.555
- sha256 / flock: 631.891, 634.129, 636.363
- keccak / mikros: 40.395, 56.875, 42.402
- keccak / flock: 119.555, 122.535, 107.906
- blake3 / mikros: 626.250, 629.008, 628.910
- blake3 / flock: 650.145, 650.777, 646.227

## 测量范围

SHA-256 测量固定标准 IV 的 compression；Keccak 测量 24 轮 Keccak-f[1600] permutation；BLAKE3 测量 compression 并验证完整 512-bit 输出。均为独立调用，未测消息填充、sponge 或通用 hash DAG。

Prover 包含在线转换、承诺、归约、配置的 grinding、公开 I/O 或原生 wiring，以及 PCS opening。离线 fixture/witness、transpose、公开 I/O 表预处理和准备好的输入复制不计时。Verifier 包含完整验证，但不包含公开 statement 到 I/O 表的可信预处理。

RAM 是 GNU time 的整个进程峰值，包含离线准备、预热、prover、verifier 和 allocator 保留内存；不能解释为在线 prover 独占峰值。负例验证进程不计入表中 RAM。

双方每种 hash 使用相同的 FLOCK Ligerito Fast PCS profile、查询数和 PCS grinding。Mikros 使用 full carrier（num_lanes=None），FLOCK 使用原生压缩多列布局；这不意味着承诺布局或元素数量相同。SHA-256/BLAKE3 使用公开 A/B CSC、C=I；Keccak 双方使用恢复的原生 compact walker、C=I。

PIOP grinding 保留各协议配置：Mikros 挑战事件为 8 bit，FLOCK 按 degree 配置（skip7 首轮 8 bit、普通尾轮 2 bit）。本实验不据此声称两者整体达到相同的 128-bit soundness。

## 验证与证据

408 个源码文件、固定源码包、实际执行 ELF、全部原始样本/命令/环境及 GNU time RSS 均经本地摘要核验；从原始数据重算的统计与服务器 summary 完全一致。每份样本在服务器经过完整 verifier 和 transcript 检查；本地审计没有重新执行密码学验证。

Native 测试共 699 项通过、11 项标记 ignored；没有失败或仅被过滤后未运行的测试组。

负例拒绝数：sha256: Mikros 60 / FLOCK 55；keccak: Mikros 137 / FLOCK 127；blake3: Mikros 74 / FLOCK 67。

FLOCK SHA-256 skip7 的 proof hash 与上一轮相同；新通用 Mikros driver 使用不同 transcript，未声称其 proof hash 与旧 SHA wrapper 相同。

源码包 SHA-256：`1953a63658f0eec846e551f0e2bb3ac40dd68665bcd486f808e63aecdbbe62fd`。执行 ELF SHA-256：`3ae54d4bda37dfe0ce866aaa2eb1426992afbf70156af70c1d5141736b674ded`。

Keccak 恢复来源：commit `c2d0c2485a54f7b7694e19f4f59730ffde3405cf`，blob `15723f053fb7e3820ace6d86ae70b7dd36d32142`。

本次原始源码快照保持冻结，不应原地修改；若修改源码，应另建快照并重新测量。

证据目录：`C:\Users\miranda\git\mikros_flock\mikros\server_experiments\20260917-hashes\v1\evidence\results-hashes`。完整每组数据见 `e2e_flock_sha256.log`、`e2e_flock_keccak.log`、`e2e_flock_blake3.log`。
