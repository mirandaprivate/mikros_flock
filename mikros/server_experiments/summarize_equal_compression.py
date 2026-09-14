#!/usr/bin/env python3
"""Validate and summarize a completed equal-compression server experiment."""
import argparse
import csv
import hashlib
import io
import json
from pathlib import Path
import re
import statistics
import tarfile

from run_server_64 import FLOCK_SYSTEMS, check_report, balanced_orders, require


def load(path):
    return json.loads(path.read_text())


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("version", type=Path)
    parser.add_argument("--results-name", default="results")
    args = parser.parse_args()
    version = args.version.resolve()
    results = version / args.results_name
    status = load(results / "status.json")
    require(status["stage"] == "complete", "experiment is not complete")
    systems, logs, runs = status["systems"], status["logs"], status["measured_runs"]
    require("flock128" in systems, "missing equal-compression Flock baseline")
    balanced_orders(systems, runs)
    manifest = load(version / "source_manifest.json")
    archive = version / "source.tar.gz"
    expected_sha = (version / "source.tar.gz.sha256").read_text().split()[0]
    require(hashlib.sha256(archive.read_bytes()).hexdigest() == expected_sha, "source archive digest")
    with tarfile.open(archive) as tar:
        require(set(tar.getnames()) == set(manifest["files_sha256"]) | {"source_manifest.json"}, "archive inventory")
        for name, digest in manifest["files_sha256"].items():
            require(hashlib.sha256(tar.extractfile(name).read()).hexdigest() == digest, f"archived source {name}")
    environment = load(results / "server_environment.json")
    require(environment["build_artifact"]["source_manifest"] == manifest, "compiled source provenance")
    require(all(c["returncode"] == 0 for c in load(results / "commands.json")), "failed command")
    validation = load(version / "validation/status.json")
    require(validation["stage"] == "complete" and validation["tests"] == 2
            and all(c["exit"] == 0 for c in validation["commands"]), "standalone interpolation tests failed")
    require(hashlib.sha256((version / "flock_skip7_interpolation.rs").read_bytes()).hexdigest()
            == validation["source_sha256"], "standalone test source provenance")
    reports = {}
    identities, inputs, fingerprints, count = {}, {}, set(), 0
    for name in status["reports"]:
        path = results / name
        raw = load(path)
        meta = raw["metadata"]
        report = check_report(path, meta["systems"], meta["logs"], meta["runs"],
                              rss=name.startswith("rss-"), profile=name.startswith("profile-"))
        reports[name] = report
        fingerprints.add((meta["mikros_sources_blake3"], meta["flock_sources_blake3"]))
        for sample in report["samples"] + report["warmups"]:
            count += 1
            base = (sample["log_n"], sample["run"])
            require(inputs.setdefault(base, sample["input_digest"]) == sample["input_digest"], "input pairing")
            protocol = sample["system"] if sample["system"] in FLOCK_SYSTEMS else "mikros"
            key = (*base, protocol)
            require(identities.setdefault(key, sample["proof_digest"]) == sample["proof_digest"], "proof identity")
    require(len(fingerprints) == 1 and count == status["validated_proof_records"], "source/count mismatch")
    # Old skip6 and all Mikros variants must retain their proof bytes. Compare
    # matching deterministic inputs with archived v3 runs when available;
    # skip7 deliberately has a different parameter and transcript domain.
    old_identities = {}
    historical = version.parent / "v3"
    for group in ("results", "candidate", "pooled-candidate", "ablation"):
        path = historical / group / "comparison-64t.json"
        if not path.is_file():
            continue
        old = load(path)
        for s in old["samples"] + old["warmups"]:
            protocol = "flock" if s["system"] == "flock" else "mikros"
            key = (s["log_n"], s["run"], protocol)
            identity = (s["input_digest"], s["proof_digest"])
            require(old_identities.setdefault(key, identity) == identity, "old archive identity conflict")
    overlap = 0
    for key, digest in identities.items():
        if key in old_identities:
            require(old_identities[key] == (inputs[key[:2]], digest), "legacy proof bytes changed")
            overlap += 1
    main_report = reports["comparison-64t.json"]
    require(main_report["metadata"]["systems"] == systems and main_report["metadata"]["logs"] == logs,
            "main matrix differs from completed status")
    rows = []
    text = ["# 相同压缩比：Mikros-128 与 Flock-128", "",
            f"服务器正式样本：{runs} 次/系统/尺寸，另有预热、独立 profile（若启用）与冷进程 RSS。{count} 份证明的 verifier、transcript 和原始输入声明检查全部成功。另有 {overlap} 个与 v3 重合的输入/旧协议组合，其证明字节保持一致。", "",
            "64 个逻辑线程；CPU `0-47,48-55,72-79`，48 个物理核加 16 个 SMT sibling，双 NUMA 默认 first-touch。共享服务器非独占。双方 native 构建启用 GFNI、AVX-512F/BW/VBMI、VPCLMULQDQ。", "",
            "`flock128` 跳过 7 轮，压缩 128；Mikros κ=128。`flock` 是保留的 skip6/压缩64基线。Mikros 两种复用变体各持独立 scratch；Flock 使用原生产全局容量池。RSS 来自每系统独立冷进程。", "",
            ("本组包含两个 Flock 参数，它们共用容量池；首次 warmup 不代表独立首次分配。" if "flock" in systems
             else "本组不运行 Flock64，Flock128 不会复用 Flock64 的较大缓冲区。smoke 在另一进程执行，退出后不影响主组内存池。"), "",
            "双方关闭 grinding，比较不包括 PCS 或 packed opening，也不声称相同 128-bit soundness。所有输入生成、序列化、源声明 oracle 均在 prover/verifier 计时之外。", ""]
    for n in logs:
        selected = [s for s in main_report["summaries"] if s["log_n"] == n]
        base = next(s for s in selected if s["system"] == "flock128")
        text += [f"## N = 2^{n}", "", "| 系统 | 压缩比 | prover 中位数 ms（范围） | 相对 Flock128 耗时 | verifier 中位数 ms | 消息 bytes / bincode | 冷进程 RSS MiB |", "|---|---:|---:|---:|---:|---:|---:|"]
        for s in selected:
            system, timing = s["system"], s["prover_ms"]
            rss = reports[f"rss-{system}-n{n}-64t.json"]["samples"][0]["vm_hwm_after_prove_kib"] / 1024
            row = {"log_n": n, "system": system, "compression": 64 if system == "flock" else 128,
                   "prover_median_ms": timing["median"], "prover_min_ms": timing["min"], "prover_max_ms": timing["max"],
                   "ratio_to_flock128": timing["median"] / base["prover_ms"]["median"],
                   "verifier_median_ms": s["verifier_ms"]["median"], "payload_bytes": s["payload_field_bytes"],
                   "bincode_bytes": s["bincode_bytes"], "cold_rss_mib": rss}
            mine = {p["run"]: p["prover_ms"] for p in main_report["samples"] if p["system"] == system and p["log_n"] == n}
            baseline = {p["run"]: p["prover_ms"] for p in main_report["samples"] if p["system"] == "flock128" and p["log_n"] == n}
            row["wins_vs_flock128"] = sum(mine[run] < baseline[run] for run in mine) if system != "flock128" else None
            row["median_paired_ratio"] = statistics.median(mine[run] / baseline[run] for run in mine)
            rows.append(row)
            text.append(f"| {system} | {row['compression']} | {timing['median']:.3f} ({timing['min']:.3f}–{timing['max']:.3f}) | {row['ratio_to_flock128']:.3f}× | {s['verifier_ms']['median']:.4f} | {s['payload_field_bytes']} / {s['bincode_bytes']} | {rss:.2f} |")
        text += ["", "相对耗时小于 1 表示本批中位数快于 Flock128。范围展示全部样本，未删除慢样本；不能仅凭略低中位数宣称稳定领先。", "",
                 "| 系统 | 首次预热证明 ms | 单独冷进程证明 ms |", "|---|---:|---:|"]
        for system in systems:
            warm = next(s for s in main_report["warmups"] if s["system"] == system and s["log_n"] == n)
            cold = reports[f"rss-{system}-n{n}-64t.json"]["samples"][0]
            text.append(f"| {system} | {warm['prover_ms']:.3f} | {cold['prover_ms']:.3f} |")
        text += ["", "首次预热证明在共用进程里顺序执行，可能已有 runtime/池/页面状态；单独冷进程是另一种生命周期，二者均不混入预热后的中位数。", ""]
        for row in rows:
            if row["log_n"] == n and row["system"] != "flock128":
                text.append(f"同一 run 的配对比较：{row['system']} 有 {row['wins_vs_flock128']}/{runs} 次快于 Flock128；逐 run 耗时比的中位数为 {row['median_paired_ratio']:.3f}×。")
        text.append("")
    if "profile-64t.json" in reports:
        profile = reports["profile-64t.json"]
        text += ["## 独立 profile", "", "诊断样本单独运行，每项为 3 次均值；不同协议的阶段边界不能直接逐列等同。", ""]
        for system in systems:
            samples = [s for s in profile["samples"] if s["system"] == system]
            if system not in FLOCK_SYSTEMS:
                for n in logs:
                    chosen = [s for s in samples if s["log_n"] == n]
                    text.append(f"{system}, N=2^{n}: " + "; ".join(
                        f"{key}={statistics.mean(s['mikros_phases'][key] for s in chosen):.3f} ms"
                        for key in chosen[0]["mikros_phases"]) + ".")
        pending = {}
        values = {}
        for line in (results / "profile-64t.log").read_text().splitlines():
            phase = re.search(r"\[zc-timing\] (round1 URM|round2 fused fold|rounds 3\+ tail): ([0-9.]+) ms", line)
            if phase:
                pending[phase[1]] = float(phase[2])
            end = re.search(r"n=2\^(\d+) run=\d+ (flock128|flock)\s*:", line)
            warm = re.search(r"warmup n=2\^\d+ (flock128|flock)\s*:", line)
            if warm:
                pending = {}
            if end:
                require(len(pending) == 3, "missing Flock profile phases")
                values.setdefault((end[2], int(end[1])), []).append(pending)
                pending = {}
        for (system, n), records in values.items():
            require(len(records) == 3, "profile sample count")
            text.append(f"{system}, N=2^{n}: " + "; ".join(
                f"{key}={statistics.mean(s[key] for s in records):.3f} ms" for key in records[0]) + ".")
        text.append("")
    text += ["## 可复核材料", "",
             f"- [原始正式数据]({args.results_name}/comparison-64t.json)",
             f"- [环境与构建记录]({args.results_name}/server_environment.json)",
             f"- [完成状态]({args.results_name}/status.json)",
             "- [源码逐文件摘要](source_manifest.json)",
             "- [额外插值边界测试](validation/status.json)：两项测试分别覆盖全部 256 个 GF8 节点和 16 个域外点；计时结束后链接原 v4 Flock rlib 运行，未改变冻结内核。测试源文件单独记录摘要，不在原 307 文件源码包中。",
             f"- 源码压缩包 SHA256：`{expected_sha}`", "",
             "v1–v3 为旧压缩64基线，保留原记录；同压缩比速度和内存结论须以此轮 Flock128 为准。"]
    output = "\n".join(text) + "\n"
    report_path = version / ("EQUAL_COMPRESSION_REPORT.md" if args.results_name == "results" else f"{args.results_name}-REPORT.md")
    report_path.write_text(output, encoding="utf-8")
    buffer = io.StringIO()
    writer = csv.DictWriter(buffer, fieldnames=rows[0].keys())
    writer.writeheader()
    writer.writerows(rows)
    report_path.with_suffix(".csv").write_text(buffer.getvalue(), encoding="utf-8")
    print(json.dumps({"validated_proofs": count, "report": str(report_path), "rows": rows}, indent=2))


if __name__ == "__main__":
    main()
