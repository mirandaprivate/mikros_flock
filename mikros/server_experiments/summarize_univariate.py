#!/usr/bin/env python3
"""Validate a downloaded v5 univariate run and write REPORT.md and SUMMARY.json."""
from __future__ import annotations

import argparse
from collections import Counter
import json
import math
from pathlib import Path
import re
import statistics

import run_univariate_64 as driver


base = driver.base
require = driver.require
PHASES = {
    "setup_ms": "权重与准备",
    "small_field_aggregate_ms": "GF256 小域聚合",
    "interpolate_quotient_ms": "插值与商多项式",
    "c_pack_project_ms": "C 打包与投影",
    "transcript_challenge_ms": "Transcript 与挑战",
    "ab_pack_ms": "A/B 打包",
    "inner_product_ms": "内积尾部",
    "terminal_claims_ms": "终端 claims",
}
FLOCK_PHASES = {
    "round1 URM": "首轮 URM",
    "round2 fused fold": "第二轮融合折叠",
    "rounds 3+ tail": "第三轮及后续尾部",
}
FLOCK_TIMING = re.compile(r"\[zc-timing\] (round1 URM|round2 fused fold|rounds 3\+ tail): ([0-9.]+) ms")
FLOCK_SAMPLE = re.compile(r"^n=(\d+) run=(\d+) flock128:")
META_KEYS = (
    "rustc", "cpu", "os", "arch", "compiled_target_features", "native_backend",
    "mikros_sources_blake3", "flock_sources_blake3", "build_configuration_blake3",
)


def all_samples(report):
    return report["warmups"] + report["samples"]


def stats(values):
    return {"samples": len(values), "median": statistics.median(values),
            "min": min(values), "max": max(values), "mean": statistics.mean(values)}


def parse_flock_phases(path, report):
    """Attach stderr phase timers to the following completed benchmark record."""
    found, pending = {}, {}
    for line in path.read_text(encoding="utf-8").splitlines():
        match = FLOCK_TIMING.search(line)
        if match:
            key, value = match[1], float(match[2])
            require(key not in pending and math.isfinite(value) and value >= 0,
                    f"{path.name}: duplicate or invalid Flock phase")
            pending[key] = value
        match = FLOCK_SAMPLE.search(line)
        if match:
            key = int(match[1]), int(match[2])
            require(set(pending) == set(FLOCK_PHASES) and key not in found,
                    f"{path.name}: incomplete or duplicate Flock profile")
            found[key], pending = pending, {}
    expected = {(s["log_n"], s["run"]) for s in all_samples(report) if s["system"] == "flock128"}
    require(not pending and set(found) == expected, f"{path.name}: Flock phase coverage")
    for sample in all_samples(report):
        if sample["system"] == "flock128":
            # Each of the three printed timers is rounded to 0.01 ms.
            require(sum(found[sample["log_n"], sample["run"]].values()) <= sample["prover_ms"] + 0.016,
                    f"{path.name}: Flock phase sum exceeds prover time")
    return found


def validate(root):
    results = root / "results"
    status = base.load(results / "status.json")
    environment = base.load(results / "server_environment.json")
    commands = base.load(results / "commands.json")
    manifest = base.load(root / "source_manifest.json")
    require(status["stage"] == "complete", "driver has not completed")
    require(status["threads"] == 64 and status["systems"] == driver.SYSTEMS,
            "unexpected main systems or thread count")
    logs, runs = status["logs"], status["measured_runs"]
    require(logs == [30] and runs == 6 and status["profile_runs"] == 3,
            "this report requires the planned N=2^30, six-run comparison and three-run profile")
    specs = {
        "smoke.json": (driver.SMOKE_SYSTEMS, [14], 1, {}),
        "comparison-64t.json": (driver.SYSTEMS, logs, runs, {}),
        "profile-64t.json": (driver.SYSTEMS, logs, 3, {"profile": True}),
        **{f"rss-{system}-n30-64t.json": ([system], logs, 1, {"rss": True}) for system in driver.SYSTEMS},
    }
    require(len(status["reports"]) == len(specs) and set(status["reports"]) == set(specs),
            "status report inventory differs from the planned experiment")
    reports = {name: driver.check_report(results / name, systems, ns, count, **flags)
               for name, (systems, ns, count, flags) in specs.items()}
    metadata = reports["comparison-64t.json"]["metadata"]
    for name, report in reports.items():
        require(all(report["metadata"][key] == metadata[key] for key in META_KEYS),
                f"{name}: inconsistent code, compiler or hardware metadata")
    require(environment["source_manifest"] == manifest, "server and local source manifests differ")
    require(environment["threads"] == 64 and environment["cpu_spec"] == base.CPU_SPEC
            and environment["selected_cpus"] == base.CPUS, "unexpected CPU affinity")
    require(environment["systems"] == driver.SYSTEMS and environment["logs"] == logs
            and environment["runs"] == runs and environment["profile_runs"] == 3, "environment run configuration")
    selected = [row for row in environment["topology"] if row[0] in base.CPUS]
    cores = Counter((socket, core) for _, core, socket, _ in selected)
    require(len(selected) == 64 and {row[0] for row in selected} == set(base.CPUS)
            and Counter(row[3] for row in selected) == {0: 32, 1: 32}
            and len(cores) == 48 and Counter(cores.values()) == {1: 32, 2: 16}, "CPU topology mismatch")
    require(environment["selected_physical_cores"] == 48 and environment["additional_smt_siblings"] == 16,
            "physical-core/SMT metadata mismatch")
    expected_orders = [[driver.SYSTEMS[(run + i) % 2] for i in range(2)] for run in range(1, runs + 1)]
    require(environment["main_execution_orders"] == expected_orders, "main order metadata mismatch")
    artifact = environment["build_artifact"]
    require(artifact["source_manifest"] == manifest and artifact["bench"] == driver.BENCH
            and artifact["toolchain"] == environment["toolchain"] == base.TOOLCHAIN
            and artifact["cpu_spec"] == base.CPU_SPEC
            and artifact["executable_sha256"] == environment["executable_sha256"], "build artifact mismatch")
    require(base.HEX64.fullmatch(artifact["executable_sha256"])
            and driver.REQUIRED_FEATURES <= set(artifact["native_target_features"]), "unverified native artifact")
    if (root / "univariate_build_artifact.json").exists():
        require(base.load(root / "univariate_build_artifact.json") == artifact, "local build artifact differs")
    validation = artifact["validation_commands"]
    require(len(validation) == len(driver.VALIDATION_STAGES)
            and {c["stage"] for c in validation} == driver.VALIDATION_STAGES
            and all(c["returncode"] == 0 for c in validation) and artifact["passed_tests"] >= 67,
            "build artifact lacks successful tests/build")
    expected_stages = {Path(name).stem for name in specs}
    if not environment["skip_build"]:
        expected_stages |= driver.VALIDATION_STAGES
    require(len(commands) == len(expected_stages) and {c["stage"] for c in commands} == expected_stages,
            "command inventory mismatch")
    hashes = {}
    for command in commands:
        stage = command["stage"]
        require(command["returncode"] == 0 and command["argv"][:3] == ["taskset", "-c", base.CPU_SPEC],
                f"{stage}: failed command or changed affinity")
        require(command["environment_overrides"] == ({"FLOCK_ZC_TIMING": "1"} if stage == "profile-64t" else {}),
                f"{stage}: unexpected profiling environment")
        log_name = "build.jsonl" if stage == "build" else f"{stage}.log"
        hashes[f"results/{log_name}"] = base.sha256(results / log_name)
        require(hashes[f"results/{log_name}"] == command["log_sha256"], f"{stage}: command log hash mismatch")
    inputs, proofs, count = {}, {}, 0
    for name, report in reports.items():
        for sample in all_samples(report):
            count += 1
            key = sample["log_n"], sample["run"]
            require(inputs.setdefault(key, sample["input_digest"]) == sample["input_digest"],
                    f"{name}: input changed across modes")
            identity = (*key, "flock128" if sample["system"] == "flock128" else "mikros-univariate")
            require(proofs.setdefault(identity, sample["proof_digest"]) == sample["proof_digest"],
                    f"{name}: proof changed across kernels/modes")
            if sample["mikros_phases"]:
                require(set(sample["mikros_phases"]) == set(PHASES), f"{name}: changed phase schema")
    require(count == status["validated_proof_records"] == environment["validated_proof_records"],
            "validated proof count mismatch")
    require(all(environment[key] == metadata[key] for key in ("mikros_sources_blake3", "flock_sources_blake3")),
            "final environment source fingerprints differ")
    for path in [root / "source_manifest.json", *(results / name for name in specs),
                 *(results / name for name in ("status.json", "commands.json", "server_environment.json"))]:
        hashes[path.relative_to(root).as_posix()] = base.sha256(path)
    flock = parse_flock_phases(results / "profile-64t.log", reports["profile-64t.json"])
    return reports, environment, count, hashes, flock


def summarize(root):
    reports, environment, count, hashes, flock = validate(root)
    main, profile = reports["comparison-64t.json"], reports["profile-64t.json"]
    systems = {}
    for system in driver.SYSTEMS:
        selected = [s for s in main["samples"] if s["system"] == system]
        warmup = next(s for s in main["warmups"] if s["system"] == system)
        rss = reports[f"rss-{system}-n30-64t.json"]["samples"][0]
        diagnostic = [s for s in profile["samples"] if s["system"] == system]
        timers = [s["mikros_phases"] if system != "flock128" else flock[s["log_n"], s["run"]] for s in diagnostic]
        average = statistics.mean(s["prover_ms"] for s in diagnostic)
        phases = {key: {"mean_ms": statistics.mean(p[key] for p in timers),
                        "share_of_mean_prover": statistics.mean(p[key] for p in timers) / average}
                  for key in (PHASES if system != "flock128" else FLOCK_PHASES)}
        residual = statistics.mean(s["prover_ms"] - sum(p.values()) for s, p in zip(diagnostic, timers))
        phases["unaccounted_ms"] = {"mean_ms": residual, "share_of_mean_prover": residual / average}
        systems[system] = {
            **{key: stats([s[key] for s in selected]) for key in ("prover_ms", "verifier_ms", "input_claim_check_ms")},
            **{key: selected[0][key] for key in ("payload_field_elements", "payload_field_bytes", "bincode_bytes")},
            "main_cold_warmup_prover_ms": warmup["prover_ms"],
            "rss": {key: rss[key] for key in ("vm_rss_before_prove_kib", "vm_hwm_after_prove_kib", "prover_ms", "input_claim_check_ms")},
            "profile": {"samples": len(diagnostic), "prover_ms": stats([s["prover_ms"] for s in diagnostic]), "phases": phases},
        }
    paired = []
    for run in range(1, main["metadata"]["runs"] + 1):
        group = {s["system"]: s for s in main["samples"] if s["run"] == run}
        mikros, flock_sample = group["reuse"], group["flock128"]
        paired.append({"run": run, "input_digest": mikros["input_digest"], "mikros_ms": mikros["prover_ms"],
                       "flock128_ms": flock_sample["prover_ms"], "mikros_over_flock": mikros["prover_ms"] / flock_sample["prover_ms"]})
    return {
        "schema_version": 1, "protocol": main["metadata"]["protocol"], "log_n": 30,
        "metadata": main["metadata"], "environment": {key: environment[key] for key in (
            "hostname", "utc_start", "utc_end", "cpu_spec", "threads", "selected_physical_cores", "additional_smt_siblings",
            "numa_policy", "thread_policy", "toolchain", "loadavg_start", "loadavg_end", "memory_start_kib", "executable_sha256")},
        "validation": {"proof_records": count, "passed_tests": environment["build_artifact"]["passed_tests"],
                       "source_manifest_matched": True, "cross_report_inputs_and_proof_digests_matched": True,
                       "artifact_sha256": hashes},
        "systems": systems, "paired": paired,
        "comparison": {"ratio_of_prover_medians": systems["reuse"]["prover_ms"]["median"] / systems["flock128"]["prover_ms"]["median"],
                       "paired_mikros_over_flock": stats([p["mikros_over_flock"] for p in paired]),
                       "mikros_wins": sum(p["mikros_over_flock"] < 1 for p in paired),
                       "ties": sum(p["mikros_over_flock"] == 1 for p in paired), "pairs": len(paired)},
    }


def render(summary):
    systems, comparison = summary["systems"], summary["comparison"]
    env, meta = summary["environment"], summary["metadata"]
    names = {"reuse": "Mikros univariate（reuse）", "flock128": "Flock skip-7"}
    lines = [
        "# 单变量 Mikros 与 Flock-128：64 线程服务器结果", "",
        f"N=2^30，Mikros prover 中位数 {systems['reuse']['prover_ms']['median']:.3f} ms，"
        f"Flock-128 为 {systems['flock128']['prover_ms']['median']:.3f} ms；"
        f"中位数之比（Mikros/Flock）为 {comparison['ratio_of_prover_medians']:.3f}。"
        f"{comparison['pairs']} 组相同输入配对中，Mikros 较快 {comparison['mikros_wins']} 次，持平 {comparison['ties']} 次。", "",
        f"服务器 CPU：{meta['cpu']}；Linux x86_64，64 线程，固定 CPU 集合 `{env['cpu_spec']}`。"
        "覆盖两 NUMA 节点，每节点 32 个逻辑 CPU，总计 48 个物理核心及 16 个 SMT sibling。"
        "使用 Linux 默认 first-touch，没有显式内存绑定或交错；服务器共享，CPU 不独占。", "",
        f"构建使用 `{env['toolchain']}`、`target-cpu=native`，报告确认 `gfni-avx512` 后端与所需 GFNI/AVX512/VPCLMUL 特性。"
        f"测量时间：{env['utc_start']} 至 {env['utc_end']}。", "",
        "本版按 `latex/4-block.tex` 实现 κ=128 的单变量 Hadamard：GF256 中间域、16 个扩域坐标、255 个采样点。"
        "终止于直接 packed-opening claims，未加入额外 2κ 列聚合或 PCS；Flock 使用 skip-7，压缩比同为 128。"
        "两者使用 BLAKE3、关闭 grinding；这不构成相同安全等级或完整 PCS 性能声明。", "",
        "## 正式计时", "",
        "每系统先执行一次 warmup，随后 6 次正式测量；轮换顺序，两系统各先执行 3 次。"
        "reuse 保留调用方 scratch 容量，动态值逐次重建；Flock 使用生产实现的内存池。"
        "固定表初始化计入首次 proof；预热时间与正式数据分别展示。", "",
        "| 系统 | prover 中位数 [min, max] ms | verifier 中位数 ms | 源输入检查中位数 ms | 首次 warmup prover ms |",
        "|---|---:|---:|---:|---:|",
    ]
    for system, data in systems.items():
        p = data["prover_ms"]
        lines.append(f"| {names[system]} | {p['median']:.3f} [{p['min']:.3f}, {p['max']:.3f}] | "
                     f"{data['verifier_ms']['median']:.4f} | {data['input_claim_check_ms']['median']:.3f} | {data['main_cold_warmup_prover_ms']:.3f} |")
    ratio = comparison["paired_mikros_over_flock"]
    lines += ["", f"逐输入配对比值 Mikros/Flock 的中位数为 {ratio['median']:.3f}，范围 [{ratio['min']:.3f}, {ratio['max']:.3f}]；"
              "小于 1 表示该输入上 Mikros 更快。配对比值中位数与两系统各自中位数之比是不同统计量。", "",
              "| run | Mikros ms | Flock-128 ms | Mikros/Flock |", "|---|---:|---:|---:|"]
    lines += [f"| {p['run']} | {p['mikros_ms']:.3f} | {p['flock128_ms']:.3f} | {p['mikros_over_flock']:.3f} |" for p in summary["paired"]]
    lines += ["", "仅 6 个配对样本，范围和胜场不保证统计显著性或跨机器性能。", "", "## 通信与独立 RSS", "",
              "| 系统 | F128 元素数 | field payload bytes | bincode bytes | 证明前 VmRSS MiB | 证明后 VmHWM MiB | 冷 proof ms |", "|---|---:|---:|---:|---:|---:|---:|"]
    for system, data in systems.items():
        rss = data["rss"]
        lines.append(f"| {names[system]} | {data['payload_field_elements']} | {data['payload_field_bytes']} | {data['bincode_bytes']} | "
                     f"{rss['vm_rss_before_prove_kib']/1024:.2f} | {rss['vm_hwm_after_prove_kib']/1024:.2f} | {rss['prover_ms']:.3f} |")
    lines += ["", "RSS 每系统仅一次全新独立进程，在 prove 后、verify 与源输入检查前读取 VmHWM；"
              "包含输入与运行时内存，不是 allocator 专属峰值。冷 proof 时间仅作参考，不混入正式预热后计时。", "",
              "## 阶段诊断", "", "这是另一个独立进程的 profile=3 测量，另外有一次 warmup；正式统计不含这些样本。"
              "占比为阶段均值/同一系统诊断 prover 均值，排除 warmup，不用跨组中位数相减。"
              "3 次诊断顺序不平衡，计时器也有开销，不能当作正式胜负或加速比。", ""]
    for system, data in systems.items():
        profile = data["profile"]
        labels = PHASES if system != "flock128" else FLOCK_PHASES
        lines += [f"**{names[system]}**：{profile['samples']} 个诊断样本，prover 均值 {profile['prover_ms']['mean']:.3f} ms。", "",
                  "| 阶段 | 均值 ms | 占诊断总均值 |", "|---|---:|---:|"]
        for key, phase in profile["phases"].items():
            lines.append(f"| {labels.get(key, '未计入阶段的余项（含日志舍入）')} | {phase['mean_ms']:.3f} | {phase['share_of_mean_prover']*100:.2f}% |")
        lines.append("")
    validation = summary["validation"]
    lines += ["Flock 阶段来自原始日志，单项保留 0.01 ms；余项逐样本计算后取均值。", "", "## 校验与边界", "",
              f"本次共校验 {validation['proof_records']} 条 proof 记录（含 smoke、warmup、正式、profile 和 RSS），"
              f"服务器测试通过 {validation['passed_tests']} 项。驱动状态 complete、全部命令零退出；"
              "本地与服务器 source manifest、代码/构建指纹及命令日志 SHA256 一致。"
              "同 (N, run) 输入在各模式中一致，各协议内部的 proof 摘要一致；不同协议不要求 proof 摘要相同。", "",
              "每条记录均完成 verifier、claims/transcript 一致性及独立完整源输入检查。"
              "输入生成/摘要、challenger 创建、序列化和源输入检查排除于 prove/verify 计时；检查仍实际执行，其开销列于上表。"
              "verifier 时间仅是归约重放，不是带承诺打开的完整验证时间。", "",
              "旧 v4 mixed-tensor 使用不同协议，仅作为历史记录；本报告不与其混合样本或跨组推导改进幅度。", "",
              "原始数据：[正式计时](results/comparison-64t.json)、[阶段诊断](results/profile-64t.json)、"
              "[诊断日志](results/profile-64t.log)、[服务器环境](results/server_environment.json)、"
              "[机器可读汇总](SUMMARY.json)。", ""]
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True, help="downloaded v5-univariate directory")
    root = parser.parse_args().root.resolve()
    summary = summarize(root)
    report = render(summary)
    # Validate everything before writing either deliverable; never edit raw data.
    (root / "SUMMARY.json").write_text(json.dumps(summary, ensure_ascii=False, indent=2, allow_nan=False) + "\n", encoding="utf-8")
    (root / "REPORT.md").write_text(report, encoding="utf-8")
    print(f"Validated {summary['validation']['proof_records']} proof records; wrote {root / 'REPORT.md'} and SUMMARY.json")


if __name__ == "__main__":
    try:
        main()
    except (ValueError, KeyError, OSError, TypeError, IndexError) as error:
        raise SystemExit(f"univariate summary failed: {error}") from error
