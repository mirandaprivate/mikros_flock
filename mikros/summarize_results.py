"""Validate benchmark samples and regenerate CSV/Markdown from saved JSON.

Uses only the Python standard library. Run after run_comparison.sh.
"""
from __future__ import annotations

import csv
import json
import math
import statistics
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent
RESULTS = ROOT / "results"


def read_report(path: Path) -> dict:
    report = json.loads(path.read_text(encoding="utf-8"))
    meta = report["metadata"]
    assert meta["threads"] == meta["rayon_actual_threads"], path
    groups = {}
    for sample in report["samples"]:
        groups.setdefault((sample["log_n"], sample["system"]), []).append(sample)
        assert sample["payload_field_bytes"] == sample["payload_field_elements"] * 16
        n = sample["log_n"]
        if sample["system"] == "flock":
            fields, overhead = 131 + 2 * (n - 6), 32
        else:
            fields, overhead = 2190 + 2 * (n - 7), 24
        assert sample["payload_field_elements"] == fields
        assert sample["bincode_bytes"] == 16 * fields + overhead
    for summary in report["summaries"]:
        samples = groups[(summary["log_n"], summary["system"])]
        assert len(samples) == summary["runs"] == meta["runs"]
        for key in ("prover_ms", "verifier_ms", "input_claim_check_ms"):
            raw = [sample[key] for sample in samples]
            assert all(math.isfinite(value) and value >= 0 for value in raw)
            for stat, expected in (("median", statistics.median(raw)), ("min", min(raw)), ("max", max(raw))):
                assert math.isclose(summary[key][stat], expected, rel_tol=1e-12), (path, key, stat)
    # Within each dimension/run all systems must receive identical witnesses.
    for n in meta["logs"]:
        for run in range(1, meta["runs"] + 1):
            pair = [s for s in report["samples"] if s["log_n"] == n and s["run"] == run]
            assert {s["system"] for s in pair} == set(meta["systems"])
            assert len({s["input_digest"] for s in pair}) == 1
            assert len({s["order"] for s in pair}) == len(pair)
    return report


def main() -> None:
    timings = [read_report(path) for path in sorted(RESULTS.glob("comparison-*t.json"))]
    assert timings, "run comparison timings first"
    timings.sort(key=lambda report: report["metadata"]["threads"])
    rss = [(path.name, read_report(path)) for path in sorted(RESULTS.glob("rss-*.json"))]
    smoke_path = RESULTS / "smoke-1t.json"
    smoke = read_report(smoke_path) if smoke_path.exists() else None
    rows = []
    for report in timings:
        for summary in report["summaries"]:
            rows.append({
                "threads": report["metadata"]["threads"],
                "log_n": summary["log_n"],
                "system": summary["system"],
                "runs": summary["runs"],
                "prover_median_ms": summary["prover_ms"]["median"],
                "prover_min_ms": summary["prover_ms"]["min"],
                "prover_max_ms": summary["prover_ms"]["max"],
                "verifier_median_ms": summary["verifier_ms"]["median"],
                "payload_bytes": summary["payload_field_bytes"],
                "bincode_bytes": summary["bincode_bytes"],
            })
    with (RESULTS / "summary.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)

    first = timings[0]["metadata"]
    observed_date = datetime.fromtimestamp(first["unix_timestamp_seconds"], timezone.utc).strftime("%Y-%m-%d UTC")
    doc = [
        "# Binary Mikros 与 Flock：本机实测", "",
        "本报告由 `summarize_results.py` 从已保存 JSON 生成，并重新检查样本数量、配对输入、消息大小和统计量。", "",
        "这是优化前 Lookup 的历史结果。已完成的 64 行打包工程优化及新对照见 [OPTIMIZED_BENCHMARKS.md](OPTIMIZED_BENCHMARKS.md)；下文不代表新 Packed 的性能。", "",
        "当前 mixed-tensor 实现在较大输入上，Flock 的 prover 更快；Mikros 的 verifier 用时更低、此次峰值 RSS 更低，但证明消息明显更大。小输入的多线程结果有较大的调度开销和波动，完整数据如下。", "",
        "## 环境与测量范围", "",
        f"- 测量日期：{observed_date}；CPU：{first['cpu']}；OS：Ubuntu/WSL2 Linux x86-64。",
        f"- 编译器：`{first['rustc']}`。",
        f"- 编译路径：`-C target-cpu=native`；报告确认启用 `{', '.join(first['compiled_target_features'])}`，本机没有启用 AVX-512。",
        f"- 基线 revision：`{first['git_revision']}`；工作区包含新增 Mikros 代码，Git dirty=true。Flock 的算法源文件未修改。",
        "- 每尺寸每系统预热 1 次，计时 5 次，报告中位数；系统执行顺序按轮次轮换，使用完全相同的 packed A/B/C 与输入 digest。",
        "- 两边均使用 F128、BLAKE3 transcript、关闭 grinding；Mikros kappa=128，Flock K_SKIP=6。此处不声称两者具有相同的 128-bit soundness。",
        "- 计时包含全部在线 reduction 工作与每次证明的动态查表构建；固定表的首次使用保留在预热记录中，未从时间中扣除。",
        "- 排除输入生成/digest、challenger 初始化、序列化、源输入求值、packed-opening reduction 和底层 PCS。每一份证明仍在计时外重放 verifier、核对 transcript 状态并核验全部三个源输入声明。",
        "- verifier 时间是 PIOP verifier 时间；这不是已接入 commitment/opening 的完整 SNARK 测量。后续独立阶段诊断见 [PERFORMANCE_ANALYSIS.md](PERFORMANCE_ANALYSIS.md)，本表保留原始无诊断日志的总耗时测量。", "",
        "## Prover 时间", "",
        "单位为 ms。`Mikros/Flock` 是耗时之比，大于 1 表示 Mikros 更慢。", "",
        "| 线程 | N | Mikros lookup | Flock degree-two | Mikros/Flock |",
        "| ---: | ---: | ---: | ---: | ---: |",
    ]
    for report in timings:
        meta = report["metadata"]
        by_key = {(s["log_n"], s["system"]): s for s in report["summaries"]}
        for n in meta["logs"]:
            a, b = by_key[(n, "lookup")], by_key[(n, "flock")]
            x, y = a["prover_ms"]["median"], b["prover_ms"]["median"]
            doc.append(f"| {meta['threads']} | 2^{n} | {x:.3f} | {y:.3f} | {x/y:.2f} |")
    doc += ["", "## 最大测量尺寸的 verifier 与通信量", "",
        "消息大小按实际 bincode 序列化测量，不包括公共输入或任何 PCS 消息。", "",
        "| 线程 | N | Mikros verify (ms) | Flock verify (ms) | Mikros bytes | Flock bytes |",
        "| ---: | ---: | ---: | ---: | ---: | ---: |"]
    for report in timings:
        n = max(report["metadata"]["logs"])
        lookup = {s["system"]: s for s in report["summaries"] if s["log_n"] == n}
        a, b = lookup["lookup"], lookup["flock"]
        doc.append(f"| {report['metadata']['threads']} | 2^{n} | {a['verifier_ms']['median']:.4f} | {b['verifier_ms']['median']:.4f} | {a['bincode_bytes']:,} | {b['bincode_bytes']:,} |")
    doc += ["", "## 独立进程峰值 RSS", "",
        "每格为一个新进程中的单次冷启动运行。Linux `VmHWM` 在 prover 返回后、verify 与源输入核验前读取；包含输入、runtime、线程和表，不能解释为纯算法工作区。计时表不使用这些冷启动时间。", "",
        "| 线程 | N | 系统 | 证明前 RSS (MiB) | 峰值 RSS (MiB) |",
        "| ---: | ---: | --- | ---: | ---: |"]
    for name, report in sorted(rss, key=lambda item: (item[1]["metadata"]["threads"], item[1]["samples"][0]["system"])):
        s = report["samples"][0]
        before, peak = s["vm_rss_before_prove_kib"], s["vm_hwm_after_prove_kib"]
        assert before is not None and peak is not None
        doc.append(f"| {report['metadata']['threads']} | 2^{s['log_n']} | {s['system']} | {before/1024:.3f} | {peak/1024:.3f} |")
    if smoke:
        doc += ["", "## 标量参考与查表的差分运行", "",
            "参考版用于正确性 oracle，递归变换含大量临时分配；下面的差值不能当作对优化标量实现的算法级加速比。", "",
            "| N | reference prover (ms) | lookup prover (ms) |",
            "| ---: | ---: | ---: |"]
        by_key = {(s["log_n"], s["system"]): s for s in smoke["summaries"]}
        for n in smoke["metadata"]["logs"]:
            doc.append(f"| 2^{n} | {by_key[(n, 'reference')]['prover_ms']['median']:.3f} | {by_key[(n, 'lookup')]['prover_ms']['median']:.3f} |")
    all_reports = timings + [report for _, report in rss] + ([smoke] if smoke else [])
    count = sum(len(r["samples"]) + len(r["warmups"]) for r in all_reports)
    doc += ["", "## 正确性与复现", "",
        f"这些 JSON 共保存 {count} 份成功重放及源输入核验的证明，包括预热、正式样本和 RSS 运行。任何 verifier、transcript 或源输入检查失败都会使 benchmark 返回失败，不写入成功样本。",
        "", "```sh", "bash mikros/run_comparison.sh", "python3 mikros/summarize_results.py", "```", "",
        "逐样本数值与完整环境在 [results/](results/)；汇总数据在 [summary.csv](results/summary.csv)。重运行会更新同名结果文件。",
        "", "本机结果仅覆盖当前实现和编译路径。新增阶段 profile 已定位张量聚合为大尺寸主瓶颈，详见 [PERFORMANCE_ANALYSIS.md](PERFORMANCE_ANALYSIS.md)；优化收益尚待实现和测量。", "",
    ]
    (ROOT / "BENCHMARKS.md").write_text("\n".join(doc), encoding="utf-8")
    print(f"Validated {count} proofs; wrote BENCHMARKS.md and results/summary.csv")


if __name__ == "__main__":
    main()
