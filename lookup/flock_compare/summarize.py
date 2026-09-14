"""Recalculate every statistic and check paired inputs, transcripts and scope."""
import argparse
import csv
import json
import math
from pathlib import Path
import statistics

parser = argparse.ArgumentParser()
parser.add_argument("run_dir", type=Path)
args = parser.parse_args()
root = args.run_dir.resolve()
environment = json.loads((root / "environment.json").read_text())
assert json.loads((root / "status.json").read_text())["stage"] == "complete"
commands = json.loads((root / "commands.json").read_text())
assert all(c["returncode"] == 0 for c in commands)
accepted = [c for c in commands if c["accepted"]]
excluded = [c for c in commands if not c["accepted"]]
assert len(accepted) == 12 and len({c["case"] for c in accepted}) == 12
assert all(not c["competing_build_observations"] for c in accepted)
assert all(c["competing_build_observations"] for c in excluded)
reports = []
identities = {}
systems = {"mikros_compact", "mikros_full", "flock"}
metrics = ["prepare_ms", "prover_core_ms", "prover_total_ms", "verifier_core_ms", "source_check_ms"]
proofs = 0
for command in accepted:
    name = command["case"]
    stage = command["stage"]
    r = json.loads((root / (stage + ".json")).read_text())
    m = r["metadata"]
    assert m["rayon_actual_threads"] == m["threads"]
    assert m["source_manifest"] == environment["snapshot_manifest"]
    if name != "smoke":
        assert m["runs"] == 6
    for kind in ("warmups", "samples"):
        for n in m["logs"]:
            for w in m["widths"]:
                for run in ([0] if kind == "warmups" else range(1, m["runs"] + 1)):
                    group = [s for s in r[kind] if (s["log_n"], s["value_bits"], s["run"]) == (n, w, run)]
                    assert len(group) == 3 and {s["system"] for s in group} == systems
                    assert {s["order"] for s in group} == {0, 1, 2}
                    assert len({s["input_digest"] for s in group}) == 1
                    assert len({s["proof_digest"] for s in group if s["system"] != "flock"}) == 1
                    for s in group:
                        assert s["tuple_bits"] == n + w
                        assert all(math.isfinite(s[k]) and s[k] >= 0 for k in metrics)
                        assert s["prover_total_ms"] + 0.001 >= s["prepare_ms"] + s["prover_core_ms"]
                        fields = n*n + 3*n + 5 if s["system"] == "flock" else 3*n*n + n + 2
                        overhead = 16 + 8*n if s["system"] == "flock" else 16 + 16*n
                        assert s["field_elements"] == fields
                        assert s["bincode_bytes"] == fields * 16 + overhead
                        key = (n, w, run, s["system"])
                        value = (s["input_digest"], s["proof_digest"])
                        assert identities.setdefault(key, value) == value
                        proofs += 1
    for s in r["summaries"]:
        samples = [x for x in r["samples"] if (x["log_n"], x["value_bits"], x["system"]) == (s["log_n"], s["value_bits"], s["system"])]
        assert len(samples) == s["runs"] == m["runs"]
        keys = metrics + (["sigma_aware_verifier_ms"] if s["system"] == "flock" else [])
        for metric in keys:
            values = [x[metric] for x in samples]
            for stat, expected in (("median", statistics.median(values)), ("min", min(values)), ("max", max(values))):
                assert math.isclose(s[metric][stat], expected, rel_tol=1e-12, abs_tol=1e-12)
        if name != "smoke":
            assert [sum(x["order"] == i for x in samples) for i in range(3)] == [2, 2, 2]
    reports.append((name, r))
assert len({r["metadata"]["source_manifest_blake3"] for _, r in reports}) == 1
assert len({r["metadata"]["benchmark_blake3"] for _, r in reports}) == 1
first = reports[0][1]["metadata"]
assert proofs == 237
focus = {}
for _, r in reports[1:]:
    for s in r["summaries"]:
        if (s["log_n"], s["value_bits"]) == (20, 8):
            focus.setdefault(r["metadata"]["threads"], {})[s["system"]] = s
one, six = focus[1], focus[6]
ratio = lambda group: group["mikros_compact"]["prover_core_ms"]["median"] / group["flock"]["prover_core_ms"]["median"]
doc = ["# Mikros 与 Flock：同一公开置换关系的实测", "",
    f"当前实现中 Flock 更快。在 N=2^20、8-bit 值下，Mikros compact / Flock 的核心 prover 耗时比为单线程 {ratio(one):.2f}×、六线程 {ratio(six):.2f}×。Mikros compact 的速度也低于自身 full-extension 对照，但保留树层容量从约 64 MiB 降至 32 MiB。具体转换成本、全部尺寸和验证结果如下。", "",
    "双方证明 f[sigma[i]] = g[i]。Mikros 使用 (f[i],i) 与 (g[i],sigma[i]) 的无损位串编码；Flock 调用现有带位置指纹的 batched Product-GKR。不是将无标签多重集合证明与带标签 copy constraint 直接比较。", "",
    f"CPU：{first['cpu']}；{first['os']}/{first['arch']}，WSL 共享桌面环境。Rust：{first['rustc']}。",
    f"编译特性：{', '.join(first['target_features'])}；两边使用同一 executable、F128 后端、BLAKE3，关闭 grinding。",
    f"绑核：单线程 CPU {environment['physical_cpu_selection'][0]}；六线程 CPU {environment['physical_cpu_selection']}，按 WSL 暴露的物理核去除 SMT 兄弟。未隔离 Windows 活动；正式计入的组在计时期间未检测到 Cargo/Rust 编译。", "",
    "Mikros 原有 permutation 代码及 Flock 源码已冻结；仅裁剪独立快照 crate 的模块/依赖声明，避免引入同时开发的 Lasso/PCS。所有协议内核保持原字节，逐文件 SHA256 见 environment.json。当前 Mikros 的低层优化作用于系数树构造和存储，GKR 仍使用两条独立的全扩域 degree-three sumcheck 链；Flock 使用合并的二次核心链及融合内核。此结果代表当前实现，不能作为论文全部 packed coefficient reduction 的性能。", "",
    "每系统/尺寸预热一次、正式六次，三系统遍历六种执行顺序。相同 run 的 witness 与 sigma 完全相同；不根据快慢筛选样本，发生编译干扰时排除并保存整组、重新运行该尺寸全部样本。Mikros compact 与 full 的证明逐字节相同。当前 Mikros 的树构造、sumcheck 和 fold 循环为标量，只有复用的 eq_table 在大表时调用 Rayon；Flock 使用现有生产并行路径。", "",
    "## Prover：原生输入下的核心计算", "",
    "单位 ms，中位数。compact/full 是 Mikros 的两种树表示；compact/Flock > 1 表示当前 Mikros 更慢。原生表示转换另表报告；core 保留 prove 内的输入检查、哈希和分配。", "",
    "| 线程 | N | value bits | Mikros tuple bits | compact | full | Flock | compact/Flock |",
    "|---:|---:|---:|---:|---:|---:|---:|---:|"]
rows = []
for name, r in reports[1:]:
    threads = r["metadata"]["threads"]
    for n in r["metadata"]["logs"]:
        for w in r["metadata"]["widths"]:
            s = {x["system"]: x for x in r["summaries"] if x["log_n"] == n and x["value_bits"] == w}
            a, b, f = [s[x] for x in ("mikros_compact", "mikros_full", "flock")]
            ac, bc, fc = [x["prover_core_ms"]["median"] for x in (a,b,f)]
            doc.append(f"| {threads} | 2^{n} | {w} | {n+w} | {ac:.3f} | {bc:.3f} | {fc:.3f} | {ac/fc:.2f}× |")
            for key, value in s.items():
                row = {"threads":threads,"log_n":n,"value_bits":w,"tuple_bits":n+w,"system":key,
                    "field_elements":value["field_elements"],"bincode_bytes":value["bincode_bytes"]}
                for metric in metrics:
                    for stat in ("median", "min", "max"):
                        row[metric+"_"+stat] = value[metric][stat]
                rows.append(row)
doc += ["", "## 从相同原始输入开始的总 prover", "",
    "包含 Mikros 的带标签 bit-matrix 构造、Statement 构造以及 Flock 的值到 F128 嵌入。双方共享的原始输入生成、公开 sigma 生成/绑定摘要及 challenger 初始化在计时外。不含序列化、verifier 或源输入核验。", "",
    "| 线程 | N | value bits | compact 总计 | full 总计 | Flock 总计 | compact 转换 | Flock 转换 |",
    "|---:|---:|---:|---:|---:|---:|---:|---:|"]
for name,r in reports[1:]:
    for n in r["metadata"]["logs"]:
        for w in r["metadata"]["widths"]:
            s={x["system"]:x for x in r["summaries"] if (x["log_n"],x["value_bits"])==(n,w)}
            a,b,f=[s[k] for k in ("mikros_compact","mikros_full","flock")]
            doc.append(f"| {r['metadata']['threads']} | 2^{n} | {w} | {a['prover_total_ms']['median']:.3f} | {b['prover_total_ms']['median']:.3f} | {f['prover_total_ms']['median']:.3f} | {a['prepare_ms']['median']:.3f} | {f['prepare_ms']['median']:.3f} |")
doc += ["", "## N=2^20、8-bit 值的验证和消息", "",
    "PIOP verifier 将所有终端声明留给源认证；Flock 的 sigma 声明也逐一认证，并额外运行实际 sigma-aware verifier。该额外接口的 O(N) 公开 sigma 求值单独报告，不能与不含 oracle 的 core verifier 混用。双方均未接 PCS，不宣称相同安全等级。", "",
    "| 线程 | 系统 | PIOP verify ms | 完整 source check ms | sigma-aware verify ms | proof bytes |",
    "|---:|---|---:|---:|---:|---:|"]
for _,r in reports[1:]:
    for s in r["summaries"]:
        if s["log_n"]==20 and s["value_bits"]==8:
            sigma = f"{s['sigma_aware_verifier_ms']['median']:.4f}" if s['sigma_aware_verifier_ms'] else "—"
            doc.append(f"| {r['metadata']['threads']} | {s['system']} | {s['verifier_core_ms']['median']:.4f} | {s['source_check_ms']['median']:.3f} | {sigma} | {s['bincode_bytes']:,} |")
doc += ["", "令 μ=log₂N，本次未 grinding 的消息包含：Mikros `3μ²+μ+2` 个 F128；Flock `μ²+3μ+5` 个 F128。两个 Mikros 模式只改变树表示，消息完全相同。字节表还计入 bincode 的向量长度字段。", "",
    "## 紧凑树究竟节省了什么", "",
    "下面只统计两棵树保留的层数据容量，不包含临时系数层、sumcheck 工作数组、输入或 allocator 开销，不能解释为峰值内存。K 是包含整个带标签输入的实际子域；扩域切换层 s 表示每个节点已合并 2^s 个原始因子。", "",
    "| N | value bits | tuple bits | K degree | compact switch s | compact 树层 MiB | full 树层 MiB |",
    "|---:|---:|---:|---:|---:|---:|---:|"]
for _, single in reports[1:]:
    if single["metadata"]["threads"] != 1:
        continue
    for n in single["metadata"]["logs"]:
        for w in single["metadata"]["widths"]:
            pair = {x["system"]: x for x in single["summaries"] if (x["log_n"],x["value_bits"]) == (n,w)}
            a, b = pair["mikros_compact"], pair["mikros_full"]
            st = a["tree_stats"]["left"]
            capacity = lambda x: sum(t["compact_bytes"] + t["extension_bytes"] for t in x["tree_stats"].values()) / 2**20
            doc.append(f"| 2^{n} | {w} | {n+w} | {st['subfield_bits']} | {st['switch_level']} | {capacity(a):.3f} | {capacity(b):.3f} |")

doc += ["", "从代码能确认的差异：", "",
    "- 当前 Mikros 分别证明左右乘积树，每轮直接形成三次多项式并折叠 a、b、eq；Flock 用随机线性组合合并两棵树，分离 equality 因子，只发送二次核心消息。",
    "- Mikros 从紧凑层临时求值为 F128 后继续做全扩域 GKR，额外增加系数求值与数组生成；其 K=32/64 乘法当前还是移位/XOR 循环。Flock 直接用当前机器支持的 PCLMUL 后端做 F128 乘法。小域或紧凑表示本身不保证更快。",
    "- Flock 将 fold 与下一轮消息计算融合、复用工作缓冲区并并行处理；Mikros 的这些主循环尚未采用相同实现。",
    "- compact 对比 full 是对整个树表示策略的实验；本轮没有阶段插桩，不能把两者的总时间差逐毫秒归因到某个内核。", ""]

doc += ["以 N=2^20、value=8、K=32、s=2 为例，每棵紧凑树先合并到二次、四次系数，再把四次节点求值到 E；GKR 回到最下面两层时，又分别把一次、二次系数求值到 E。按源码乘法表达式计数，仅建树和层求值就包含约 `3.25N−1` 次 E 乘法及 `1.5N` 次 K 乘法，全扩域树对应 `N−1` 次 E 乘法。这里不计双方相同的 GKR 算术，也未扣除编译器可能消除的常数乘法；这是静态操作数分析，不是实测阶段耗时。当前存储打包没有把 GKR 留在系数域，因而产生了额外算术。", ""]

doc += ["", "## 复核与复现", "",
    f"正式计入 {proofs} 份成功证明（含预热），均完成 verifier replay、transcript 同步和全部原始源声明核验。汇总脚本重新计算中位数/min/max，检查配对输入、六种顺序、消息大小以及跨线程证明摘要。min/max 与全部样本见 summary.csv 和原始 JSON。", "",
    f"本目录另保留 {len(excluded)} 个受编译干扰而整体排除的 attempt，依据及进程时间记录在 commands.json；之前两轮整批试跑也因编译干扰排除，保留于相邻 local-20260912-permutation 和 local-20260912-permutation-r2 目录。排除依据仅为外部编译事件，未按耗时筛选样本。", "",
    "消息不含公共 sigma、statement、承诺及 opening。tree_stats 仅反映已保存树层容量，不是进程 RSS；本轮不报告峰值内存。",
    "", "```sh", "# 在 lookup/flock_compare 中；快照已存在时不再次 prepare_snapshot", "cargo +nightly-2026-03-21 test --release --offline -j 1", "cargo +nightly-2026-03-21 build --release --offline -j 1", "python3 run_experiment.py --name NEW_RUN_ID", "python3 summarize.py results/NEW_RUN_ID", "```", "",
    f"Executable SHA256：`{environment['binary_sha256']}`。", ""]
(root/"REPORT.md").write_text("\n".join(doc),encoding="utf-8")
with (root/"summary.csv").open("w",newline="",encoding="utf-8") as stream:
    writer=csv.DictWriter(stream,fieldnames=list(rows[0]));writer.writeheader();writer.writerows(rows)
print(f"Validated {proofs} proofs; wrote {root/'REPORT.md'}")
