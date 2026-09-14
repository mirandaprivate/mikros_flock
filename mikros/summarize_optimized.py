#!/usr/bin/env python3
"""Validate optimized comparisons and byte-identical Mikros proof digests."""
import csv
import json
import math
from pathlib import Path
from statistics import mean

from summarize_results import read_report

ROOT = Path(__file__).resolve().parent
RESULTS = ROOT / "results" / "optimized"


def checked(path):
    report = read_report(path)
    for kind in ("samples", "warmups"):
        samples = report[kind]
        for sample in samples:
            if "mikros_phases" in sample:
                phases = sample["mikros_phases"].values()
                assert all(math.isfinite(v) and v >= 0 for v in phases)
                assert sum(phases) <= sample["prover_ms"] + 0.0001
        for key in {(s["log_n"], s["run"]) for s in samples}:
            pair = [s for s in samples if (s["log_n"], s["run"]) == key]
            assert len({s["input_digest"] for s in pair}) == 1
            mikros = [s for s in pair if s["system"] != "flock"]
            assert len({s["proof_digest"] for s in mikros}) <= 1, (path, key)
    return report


def main():
    reports = [(path, checked(path)) for path in sorted(RESULTS.glob("comparison-*t.json"))]
    assert reports
    assert {r["metadata"]["threads"] for _, r in reports} == {1, 6}
    rows, count = [], 0
    all_reports = [(path, checked(path)) for path in sorted(RESULTS.glob("*.json")) if path.name != "summary.json"]
    assert len({r["metadata"]["mikros_sources_blake3"] for _, r in all_reports}) == 1
    identities = {}
    for path, report in all_reports:
        for sample in report["samples"] + report["warmups"]:
            key = (sample["log_n"], sample["run"], sample["system"])
            identity = (sample["input_digest"], sample["proof_digest"])
            assert identities.setdefault(key, identity) == identity, (path, key)
    for _, report in all_reports:
        count += len(report["samples"]) + len(report["warmups"])
    first = reports[0][1]["metadata"]
    doc = ["# Mikros 打包工程优化与 Flock 对比", "",
           "本报告由 summarize_optimized.py 从原始 JSON 生成。每份证明已在 benchmark 内完成 verifier、transcript 和全部源输入声明核验；脚本进一步核对统计量、输入摘要及 Mikros 各内核的序列化证明摘要完全一致。", "",
           "## 测量范围", "",
           f"- CPU：{first['cpu']}；{first['os']} {first['arch']}；指令集：{', '.join(first['compiled_target_features'])}。",
           f"- Rust：{first['rustc']}；同一 executable、F128 backend、BLAKE3 transcript 和统一线程配置。",
           "- lookup 为保留的旧逐行 LUT；packed 为多行打包聚合、融合输入绑定和融合 IP tail；Flock 调用原生产 prove_packed，算法未改。",
           "- 主要曲线每系统预热一次，六次正式样本；三系统循环六种执行排列，均衡先后位置。正式计时关闭 FLOCK_ZC_TIMING。",
           "- 同一 N 和 run 使用完全相同的 packed A/B/C。所有 witness 变换、动态构表、分配、权重、折叠、transcript 和声明构造计入在线 prover 时间；不在计时外缓存 witness 或 challenge-dependent 表。",
           "- 不计入输入生成/digest、challenger 初始化、序列化与源输入 oracle；双方均不含 packed-opening reduction、PCS commitment/opening。grinding 均关闭，不声明相同的 128-bit soundness。",
           "- 两个 Mikros 内核的协议参数、proof bytes 和 transcript 不变。这使优化前后比较具有相同协议语义；与 Flock 的比较仍保留不同协议和参数的区别。", "",
           "## 在线 prover 中位数", "",
           "单位 ms。旧/新 > 1 为工程优化加速；新/Flock > 1 表示优化后的 Mikros 仍慢。", "",
           "| 线程 | N | 旧 lookup | 新 packed | Flock | 旧/新 | 新/Flock |",
           "| ---: | ---: | ---: | ---: | ---: | ---: | ---: |"]
    for path, report in reports:
        meta = report["metadata"]
        assert meta["logs"] == [16, 18, 20, 22, 24]
        assert meta["runs"] == 6
        assert set(meta["systems"]) == {"lookup", "packed", "flock"}
        assert not meta["mikros_phase_profiling"]
        assert "FLOCK_ZC_TIMING" not in meta["tuning_environment"]
        by_key = {(s["log_n"], s["system"]): s for s in report["summaries"]}
        for n in meta["logs"]:
            old, new, flock = [by_key[n, name] for name in ("lookup", "packed", "flock")]
            a, b, c = [s["prover_ms"]["median"] for s in (old, new, flock)]
            doc.append(f"| {meta['threads']} | 2^{n} | {a:.3f} | {b:.3f} | {c:.3f} | {a/b:.2f}× | {b/c:.2f}× |")
            rows.append({"threads": meta["threads"], "log_n": n,
                         "lookup_ms": a, "packed_ms": b, "flock_ms": c,
                         "speedup": a/b, "packed_over_flock": b/c,
                         "packed_min_ms": new["prover_ms"]["min"], "packed_max_ms": new["prover_ms"]["max"],
                         "packed_verify_ms": new["verifier_ms"]["median"], "flock_verify_ms": flock["verifier_ms"]["median"],
                         "packed_bytes": new["bincode_bytes"], "flock_bytes": flock["bincode_bytes"]})
    doc += ["", "## 最大尺寸 verifier、通信量与进程峰值 RSS", "",
            "RSS 来自每系统/线程一个全新进程，在 prove 返回后、verify 和源输入检查之前读取 VmHWM；包含输入与 runtime。它不是仅工作区字节数。", "",
            "| 线程 | 系统 | N | verify (ms) | bincode bytes | RSS peak (MiB) |",
            "| ---: | --- | ---: | ---: | ---: | ---: |"]
    rss = {(r["metadata"]["threads"], r["samples"][0]["system"]): r["samples"][0]
           for p, r in all_reports if p.name.startswith("rss-")}
    for _, report in reports:
        threads, n = report["metadata"]["threads"], max(report["metadata"]["logs"])
        for s in report["summaries"]:
            if s["log_n"] == n:
                memory = rss.get((threads, s["system"]), {}).get("vm_hwm_after_prove_kib")
                memory_text = f"{memory/1024:.2f}" if memory is not None else "未测"
                doc.append(f"| {threads} | {s['system']} | 2^{n} | {s['verifier_ms']['median']:.4f} | {s['bincode_bytes']} | {memory_text} |")
    doc += ["", "## Mikros 阶段分解", "",
            "使用独立 --profile 测量，列出最大 N 的六次均值；不混入上面的无探针中位数。packed 的 A/B packing 已包含 B row weighting。", "",
            "| 线程 | 内核 | 总计 | 聚合 | C 投影 | A/B binding + B weighting | IP tail | 其他 |",
            "| ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: |"]
    for path, report in all_reports:
        if not path.name.startswith("phases-"):
            continue
        threads, n = report["metadata"]["threads"], max(report["metadata"]["logs"])
        for system in ("lookup", "packed"):
            samples = [s for s in report["samples"] if s["system"] == system and s["log_n"] == n]
            total = mean(s["prover_ms"] for s in samples)
            phases = {key: mean(s["mikros_phases"][key] for s in samples) for key in samples[0]["mikros_phases"]}
            core = [phases["tensor_aggregate_ms"], phases["c_pack_project_ms"], phases["ab_pack_ms"] + phases["b_weight_ms"], phases["inner_product_ms"]]
            doc.append(f"| {threads} | {system} | {total:.3f} | " + " | ".join(f"{x:.3f}" for x in core + [total - sum(core)]) + " |")
    doc += ["", "## 原始数据与复现", "",
            f"本目录原始报告共包含 {count} 份已核验的证明，含预热、独立阶段实验及 RSS 样本。min/max 和所有样本均保留，没有剔除慢样本。", "",
            "`bash mikros/run_optimized.sh` 串行构建、测试和复测；`python3 mikros/summarize_optimized.py` 重新核对并生成本报告及 results/optimized/summary.csv。实现与实验取舍见 [OPTIMIZATION_NOTES.md](OPTIMIZATION_NOTES.md)。", ""]
    (ROOT / "OPTIMIZED_BENCHMARKS.md").write_text("\n".join(doc), encoding="utf-8")
    with (RESULTS / "summary.csv").open("w", newline="") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)
    print(f"Validated {count} proofs and identical paired Mikros proof digests; wrote OPTIMIZED_BENCHMARKS.md")
    print(json.dumps(rows, indent=2))


if __name__ == "__main__":
    main()
