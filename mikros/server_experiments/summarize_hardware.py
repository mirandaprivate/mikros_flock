#!/usr/bin/env python3
"""Summarize downloaded v6 screening/results without changing frozen sources."""
from __future__ import annotations

import argparse
from collections import Counter
import json
from pathlib import Path
import re
import statistics

import run_hardware_64 as driver
from summarize_univariate import PHASES, FLOCK_PHASES, META_KEYS, all_samples, parse_flock_phases, stats


base = driver.base
require = driver.require
NAMES = {"reuse": "旧核 reuse", "tiledreuse": "寄存器分块 tiledreuse",
         "basisreuse": "分块及批量坐标转换 basisreuse", "flock128": "Flock-128"}


def read_group(root, name, manifest, artifact):
    directory = root / name
    status = base.load(directory / "status.json")
    environment = base.load(directory / "server_environment.json")
    commands = base.load(directory / "commands.json")
    require(status["stage"] == "complete", f"{name}: driver is not complete")
    systems, logs, runs = status["systems"], status["logs"], status["measured_runs"]
    require(systems and len(set(systems)) == len(systems) and set(systems) <= set(driver.SYSTEMS)
            and runs > 0 and runs % len(systems) == 0, f"{name}: systems or position balance")
    require(logs and len(set(logs)) == len(logs) and all(14 <= n <= 30 for n in logs), f"{name}: dimensions")
    profile_runs, rss = status["profile_runs"], status["rss_requested"]
    require(profile_runs in (0, 3) and isinstance(rss, bool), f"{name}: diagnostic flags")
    specifications = {
        "smoke.json": (driver.SMOKE_SYSTEMS, [14], 1, {}),
        "comparison-64t.json": (systems, logs, runs, {}),
    }
    if profile_runs:
        specifications["profile-64t.json"] = (systems, logs, profile_runs, {"profile": True})
    if rss:
        specifications.update({f"rss-{system}-n{n}-64t.json": ([system], [n], 1, {"rss": True})
                               for n in logs for system in systems})
    require(len(status["reports"]) == len(specifications) and set(status["reports"]) == set(specifications),
            f"{name}: report inventory")
    reports = {filename: driver.check_report(directory / filename, ss, ns, count, **flags)
               for filename, (ss, ns, count, flags) in specifications.items()}
    meta = reports["comparison-64t.json"]["metadata"]
    require(all(report["metadata"][key] == meta[key] for report in reports.values() for key in META_KEYS),
            f"{name}: hardware/source metadata differ between reports")
    require(status["threads"] == environment["threads"] == 64
            and environment["cpu_spec"] == base.CPU_SPEC and environment["selected_cpus"] == base.CPUS,
            f"{name}: CPU affinity")
    require(environment["systems"] == systems and environment["logs"] == logs and environment["runs"] == runs
            and environment["profile_runs"] == profile_runs and environment["rss_requested"] == rss,
            f"{name}: environment/command configuration")
    selected = [row for row in environment["topology"] if row[0] in base.CPUS]
    cores = Counter((socket, core) for _, core, socket, _ in selected)
    require(len(selected) == 64 and {row[0] for row in selected} == set(base.CPUS)
            and Counter(row[3] for row in selected) == {0: 32, 1: 32}
            and len(cores) == 48 and Counter(cores.values()) == {1: 32, 2: 16}
            and environment["selected_physical_cores"] == 48 and environment["additional_smt_siblings"] == 16,
            f"{name}: physical-core/SMT topology")
    orders = [[systems[(run + i) % len(systems)] for i in range(len(systems))] for run in range(1, runs + 1)]
    require(environment["main_execution_orders"] == orders, f"{name}: recorded execution order")
    require(environment["source_manifest"] == manifest and environment["build_artifact"] == artifact
            and environment["toolchain"] == artifact["toolchain"]
            and environment["executable_sha256"] == artifact["executable_sha256"], f"{name}: source/build provenance")
    require(all(environment[key] == meta[key] for key in ("mikros_sources_blake3", "flock_sources_blake3")),
            f"{name}: final source fingerprint")
    expected_stages = {Path(filename).stem for filename in specifications}
    if not environment["skip_build"]:
        expected_stages |= driver.VALIDATION_STAGES
    require(len(commands) == len(expected_stages) and {c["stage"] for c in commands} == expected_stages,
            f"{name}: command inventory")
    for command in commands:
        stage = command["stage"]
        require(command["returncode"] == 0 and command["argv"][:3] == ["taskset", "-c", base.CPU_SPEC],
                f"{name}/{stage}: unsuccessful command or different affinity")
        require(command["environment_overrides"] == ({"FLOCK_ZC_TIMING": "1"} if stage == "profile-64t" else {}),
                f"{name}/{stage}: profiling environment")
        filename = "build.jsonl" if stage == "build" else f"{stage}.log"
        require(base.sha256(directory / filename) == command["log_sha256"], f"{name}/{stage}: log SHA256")
        if stage == "tests":
            output = (directory / filename).read_text(encoding="utf-8")
            observed = sum(map(int, re.findall(r"test result: ok\. (\d+) passed", output)))
            require(observed == artifact["passed_tests"] and "univariate::domain::tests::" in output,
                    f"{name}: test log/artifact count")
    count = sum(len(all_samples(report)) for report in reports.values())
    require(count == status["validated_proof_records"] == environment["validated_proof_records"], f"{name}: proof count")
    flock = parse_flock_phases(directory / "profile-64t.log", reports["profile-64t.json"]) if profile_runs else {}
    return {"name": name, "reports": reports, "environment": environment, "proof_records": count, "flock_phases": flock}


def paired_comparison(selected, numerator, denominator):
    left = {s["run"]: s for s in selected if s["system"] == numerator}
    right = {s["run"]: s for s in selected if s["system"] == denominator}
    require(set(left) == set(right), "unpaired run indices")
    pairs = []
    for run in sorted(left):
        a, b = left[run], right[run]
        require(a["input_digest"] == b["input_digest"], "unpaired source inputs")
        pairs.append({"run": run, "input_digest": a["input_digest"], "numerator_ms": a["prover_ms"],
                      "denominator_ms": b["prover_ms"], "ratio": a["prover_ms"] / b["prover_ms"]})
    return {"numerator": numerator, "denominator": denominator, "pairs": pairs,
            "ratio_of_medians": statistics.median(p["numerator_ms"] for p in pairs)
                                / statistics.median(p["denominator_ms"] for p in pairs),
            "paired_ratios": stats([p["ratio"] for p in pairs]),
            "numerator_wins": sum(p["ratio"] < 1 for p in pairs), "ties": sum(p["ratio"] == 1 for p in pairs)}


def group_summary(group):
    reports = group["reports"]
    main, profile = reports["comparison-64t.json"], reports.get("profile-64t.json")
    dimensions = []
    for n in main["metadata"]["logs"]:
        selected = [s for s in main["samples"] if s["log_n"] == n]
        systems = {}
        for system in main["metadata"]["systems"]:
            samples = [s for s in selected if s["system"] == system]
            warmup = next(s for s in main["warmups"] if (s["log_n"], s["system"]) == (n, system))
            rss_report = reports.get(f"rss-{system}-n{n}-64t.json")
            rss = rss_report["samples"][0] if rss_report else None
            phases = None
            if profile:
                diagnostic = [s for s in profile["samples"] if (s["log_n"], s["system"]) == (n, system)]
                timers = [group["flock_phases"][n, s["run"]] if system == "flock128" else s["mikros_phases"] for s in diagnostic]
                keys = FLOCK_PHASES if system == "flock128" else PHASES
                require(all(set(p) == set(keys) for p in timers), f"{group['name']}: changed phase schema")
                total = statistics.mean(s["prover_ms"] for s in diagnostic)
                values = {key: statistics.mean(p[key] for p in timers) for key in keys}
                values["unaccounted_ms"] = statistics.mean(s["prover_ms"] - sum(p.values()) for s, p in zip(diagnostic, timers))
                phases = {"samples": len(diagnostic), "prover_ms": stats([s["prover_ms"] for s in diagnostic]),
                          "phases": {key: {"mean_ms": value, "share_of_mean_prover": value / total} for key, value in values.items()}}
            systems[system] = {
                **{key: stats([s[key] for s in samples]) for key in ("prover_ms", "verifier_ms", "input_claim_check_ms")},
                **{key: samples[0][key] for key in ("payload_field_elements", "payload_field_bytes", "bincode_bytes")},
                "warmup_prover_ms": warmup["prover_ms"], "profile": phases,
                "rss": {key: rss[key] for key in ("vm_rss_before_prove_kib", "vm_hwm_after_prove_kib", "prover_ms", "input_claim_check_ms")} if rss else None,
            }
        comparisons = [paired_comparison(selected, system, baseline) for system in systems if system != "flock128"
                       for baseline in ("reuse", "flock128") if baseline in systems and baseline != system]
        dimensions.append({"log_n": n, "runs": main["metadata"]["runs"], "systems": systems, "comparisons": comparisons})
    return {"name": group["name"], "metadata": main["metadata"], "proof_records": group["proof_records"],
            "environment": {key: group["environment"][key] for key in (
                "hostname", "utc_start", "utc_end", "cpu_spec", "threads", "selected_physical_cores", "additional_smt_siblings",
                "toolchain", "skip_build", "profile_runs", "rss_requested", "numa_policy", "thread_policy",
                "loadavg_start", "loadavg_end", "memory_start_kib", "memory_end_kib", "executable_sha256")},
            "dimensions": dimensions}


def summarize(root, names):
    manifest = base.load(root / "source_manifest.json")
    artifact = base.load(root / "hardware_build_artifact.json")
    require(artifact["source_manifest"] == manifest and artifact["bench"] == driver.BENCH
            and artifact["toolchain"] == base.TOOLCHAIN and artifact["cpu_spec"] == base.CPU_SPEC
            and base.HEX64.fullmatch(artifact["executable_sha256"])
            and driver.REQUIRED_FEATURES <= set(artifact["native_target_features"]), "unverified native build artifact")
    validation = artifact["validation_commands"]
    require(len(validation) == len(driver.VALIDATION_STAGES)
            and {c["stage"] for c in validation} == driver.VALIDATION_STAGES
            and all(c["returncode"] == 0 for c in validation) and artifact["passed_tests"] >= 67,
            "build/test validation missing")
    # The checker is part of the frozen package; this postprocessor is not.
    repository = Path(__file__).resolve().parents[2]
    for filename in ("run_hardware_64.py", "run_server_64.py"):
        relative = f"mikros/server_experiments/{filename}"
        require(base.sha256(repository / relative) == manifest["files_sha256"][relative], f"checker changed since snapshot: {filename}")
    require("mikros/server_experiments/summarize_hardware.py" not in manifest["files_sha256"], "postprocessor unexpectedly inside frozen source")
    groups = [read_group(root, name, manifest, artifact) for name in names]
    reference = groups[0]["reports"]["comparison-64t.json"]["metadata"]
    inputs, proofs = {}, {}
    for group in groups:
        for report in group["reports"].values():
            require(all(report["metadata"][key] == reference[key] for key in META_KEYS), "groups do not share compiler, code and hardware")
            for sample in all_samples(report):
                key = sample["log_n"], sample["run"]
                require(inputs.setdefault(key, sample["input_digest"]) == sample["input_digest"], "input changed across groups/modes")
                key = (*key, "flock128" if sample["system"] == "flock128" else "mikros-univariate")
                require(proofs.setdefault(key, sample["proof_digest"]) == sample["proof_digest"], "proof changed across kernels/groups/modes")
    paths = [root / "source_manifest.json", root / "hardware_build_artifact.json"]
    for name in names:
        paths.extend(path for path in (root / name).iterdir() if path.is_file() and path.suffix in (".json", ".log", ".jsonl"))
    for filename in ("source.tar.gz", "source.tar.gz.sha256"):
        if (root / filename).exists():
            paths.append(root / filename)
    if (root / "source.tar.gz.sha256").exists():
        expected = (root / "source.tar.gz.sha256").read_text(encoding="utf-8").split()[0]
        require(expected == base.sha256(root / "source.tar.gz"), "source archive SHA256 mismatch")
    evidence_path = root / "evidence_manifest.json"
    if evidence_path.exists():
        evidence = base.load(evidence_path)
        require(evidence["source_archive_sha256"] == base.sha256(root / "source.tar.gz"), "evidence/source archive mismatch")
        paths.append(evidence_path)
        for filename, expected in evidence["files_sha256"].items():
            path = (root / filename).resolve()
            require(path.is_relative_to(root) and base.sha256(path) == expected, f"downloaded evidence SHA256: {filename}")
            paths.append(path)
    if (root / "evidence.tar.gz.sha256").exists():
        expected = (root / "evidence.tar.gz.sha256").read_text(encoding="utf-8").split()[0]
        require(expected == base.sha256(root / "evidence.tar.gz"), "evidence archive SHA256 mismatch")
        paths.extend([root / "evidence.tar.gz", root / "evidence.tar.gz.sha256"])
    note = root / "diagnostics/local-assembly-audit.md"
    if note.exists():
        paths.append(note)
    return {
        "schema_version": 1, "protocol": reference["protocol"],
        "validation": {"proof_records": sum(g["proof_records"] for g in groups), "passed_tests": artifact["passed_tests"],
                       "base_revision": manifest["base_revision"], "executable_sha256": artifact["executable_sha256"],
                       "mikros_sources_blake3": reference["mikros_sources_blake3"], "flock_sources_blake3": reference["flock_sources_blake3"],
                       "build_configuration_blake3": reference["build_configuration_blake3"],
                       "artifact_sha256": {path.relative_to(root).as_posix(): base.sha256(path) for path in paths},
                       "postprocessor_sha256": {filename: base.sha256(Path(__file__).with_name(filename))
                                                for filename in ("summarize_hardware.py", "summarize_univariate.py")},
                       "cross_group_code_inputs_and_proofs_matched": True},
        "groups": [group_summary(group) for group in groups],
    }


def render(summary):
    first = summary["groups"][0]
    meta, env, validation = first["metadata"], first["environment"], summary["validation"]
    lines = ["# v6：单变量 Hadamard 的硬件消融与 Flock-128 对比", ""]
    for group in summary["groups"]:
        for dimension in group["dimensions"]:
            systems = dimension["systems"]
            if dimension["log_n"] == 30 and {"reuse", "basisreuse", "flock128"} <= set(systems):
                new, old, flock = [systems[s]["prover_ms"]["median"] for s in ("basisreuse", "reuse", "flock128")]
                compared = next(c for c in dimension["comparisons"] if (c["numerator"], c["denominator"]) == ("basisreuse", "flock128"))
                lines += [f"同一可执行文件的 N=2^30 测量中，basisreuse prover 中位数 {new:.3f} ms，"
                          f"旧核 reuse 为 {old:.3f} ms，Flock-128 为 {flock:.3f} ms。候选/旧核 = {new/old:.3f}，"
                          f"候选/Flock = {new/flock:.3f}；候选在 {dimension['runs']} 组配对输入中较 Flock 更快 {compared['numerator_wins']} 次。", "",
                          f"候选与 Flock 中位数相差 {abs(new/flock-1)*100:.3f}%；候选观测范围 "
                          f"[{systems['basisreuse']['prover_ms']['min']:.3f}, {systems['basisreuse']['prover_ms']['max']:.3f}] ms，"
                          f"Flock 为 [{systems['flock128']['prover_ms']['min']:.3f}, {systems['flock128']['prover_ms']['max']:.3f}] ms。"
                          "不能仅凭中位数略低称为稳定领先。", ""]
    lines += [f"环境：{meta['cpu']}，Linux x86_64；64 线程固定在 `{env['cpu_spec']}`，"
              "覆盖 48 个物理核心及 16 个 SMT sibling，两 NUMA 节点各 32 个逻辑 CPU。"
              "Linux 默认 first-touch，无显式内存绑定/交错；共享服务器，CPU 不独占。", "",
              f"使用 `{env['toolchain']}` 和 `target-cpu=native`；编译特性包含 GFNI、AVX512F/BW、VPCLMUL，"
              "报告确认 GFNI/AVX512 后端。N26 筛选与 N30 测量使用同一冻结 snapshot、同一 executable，分组统计。", "",
              "## 实现与协议边界", "",
              "`reuse` 保留 v5 Native 实现；`tiledreuse` 用 64 行、4 坐标的寄存器分块聚合；"
              "`basisreuse` 再用批量 GFNI 仿射变换完成原来的坐标转换。候选均需显式选择，默认仍为 `reuse,flock128`。"
              "名称中的 basis 指坐标转换实现：GF256 嵌入、扩域数学基、采样点、商多项式、挑战域均未改变。", "",
              "冻结 benchmark 元数据中的 `its new GF256 basis` 措辞不精确，应理解为同一数学基上的新批量转换实现；"
              "原始元数据保留供追溯，不修改冻结源码或实验记录。", "",
              "分块让累加值在寄存器中复用、批量转换减少标量查表，这是代码结构事实；"
              "缓存流量下降是据此提出的解释。本次没有硬件性能计数器，不能声称已测得 cache miss 或内存带宽下降。", "",
              "下载后的汇编检查仅确认四坐标累加热循环没有 accumulator spill/reload，不代表整个聚合或坐标转换没有内存访问。"
              "具体依据见[本地汇编诊断](diagnostics/local-assembly-audit.md)；"
              "[perf 权限记录](diagnostics/perf-access.txt)说明为何没有硬件计数器数据。", "",
              "比较边界是 κ=128、GF256 中间域的单变量 PIOP，终止于直接 packed-opening claims；"
              "未加入额外 2κ 列聚合、PCS 或承诺打开。Flock 使用 skip-7，同压缩比 128。"
              "BLAKE3、关闭 grinding，不据此声明相同安全等级。", "",
              "## 分组计时与配对比较", "",
              "每维度每系统一次 warmup，随后按 run 轮换顺序；正式次数能被系统数整除，各位置出现次数相同。"
              "各 reuse 模式独立保留 scratch 容量，动态值重新计算；Flock 使用生产内存池。"
              "首次 fixed-table 初始化在进程内某个先执行 proof 中发生，未扣除；warmup 不等于每系统独立冷启动。", ""]
    for group in summary["groups"]:
        for d in group["dimensions"]:
            label = "N26 候选筛选" if d["log_n"] == 26 else f"N{d['log_n']} 同 snapshot 测量"
            lines += [f"### {label}：{group['name']}，每系统 {d['runs']} 次", "",
                      f"UTC：{group['environment']['utc_start']} 至 {group['environment']['utc_end']}。", "",
                      "| 系统 | prover 中位数 [min, max] ms | verifier 中位数 ms | 完整源检查中位数 ms | warmup prover ms |",
                      "|---|---:|---:|---:|---:|"]
            for system, s in d["systems"].items():
                p = s["prover_ms"]
                lines.append(f"| {NAMES.get(system, system)} | {p['median']:.3f} [{p['min']:.3f}, {p['max']:.3f}] | "
                             f"{s['verifier_ms']['median']:.4f} | {s['input_claim_check_ms']['median']:.3f} | {s['warmup_prover_ms']:.3f} |")
            lines += ["", "| 配对比值（分子/分母） | 两中位数之比 | 配对比值中位数 [min, max] | 分子较快 / 配对数 |", "|---|---:|---:|---:|"]
            for c in d["comparisons"]:
                p = c["paired_ratios"]
                lines.append(f"| {c['numerator']}/{c['denominator']} | {c['ratio_of_medians']:.3f} | "
                             f"{p['median']:.3f} [{p['min']:.3f}, {p['max']:.3f}] | {c['numerator_wins']}/{p['samples']} |")
            lines += ["", "配对键为同组 (N, run)，并核对完整 input digest；比值小于 1 表示分子更快。"
                      "配对比值中位数不同于两系统中位数之比。逐 run 时间、比值和输入摘要保存在 SUMMARY.json。", "",
                      f"数据：[计时]({group['name']}/comparison-64t.json)、[环境]({group['name']}/server_environment.json)。", ""]
    lines += ["N26 用于候选选择，N30 是选定候选的后续评估；未把筛选样本并入后续样本。"
              "N30 只比较 Native 与 TiledBasis 组合候选，没有独立 tiledreuse 测量，不能将 N30 总改进分别归因于分块和批量转换。"
              "样本数、范围和胜场不保证统计显著性或跨机器性能。v5 报告及 v4 mixed-tensor 均保持历史记录，不用于本组加速比。", "",
              "## 通信与独立 RSS", "", "| 组 / N | 系统 | F128 元素 | payload bytes | bincode bytes | prove 前 VmRSS MiB | prove 后 VmHWM MiB | 冷 proof ms |",
              "|---|---|---:|---:|---:|---:|---:|---:|"]
    for group in summary["groups"]:
        for d in group["dimensions"]:
            for system, s in d["systems"].items():
                rss = s["rss"]
                memory = f"{rss['vm_rss_before_prove_kib']/1024:.2f} | {rss['vm_hwm_after_prove_kib']/1024:.2f} | {rss['prover_ms']:.3f}" if rss else "未运行 | 未运行 | 未运行"
                lines.append(f"| {group['name']} / {d['log_n']} | {system} | {s['payload_field_elements']} | {s['payload_field_bytes']} | {s['bincode_bytes']} | {memory} |")
    lines += ["", "RSS 是每系统/维度一个独立进程中的一次冷 proof；VmHWM 在 prove 后、verify 和源检查前读取。"
              "包含输入及运行时，不是 allocator 专属峰值；冷 proof 时间不混入正式统计。未运行 RSS 的组不推测内存数值。", "",
              "## 阶段诊断", "", "profile 来自独立进程，不混入正式计时；每系统另有一次 warmup，表中仅取正式诊断样本。"
              "阶段占比为阶段均值/该系统同组诊断 prover 均值，余项逐样本计算再取均值。"
              "计时器有额外开销；Flock 原始日志阶段精度为 0.01 ms。未运行 profile 的组不推测阶段耗时。", ""]
    for group in summary["groups"]:
        profiled = False
        for d in group["dimensions"]:
            if {"reuse", "basisreuse"} <= set(d["systems"]):
                old_profile = d["systems"]["reuse"]["profile"]
                new_profile = d["systems"]["basisreuse"]["profile"]
                if old_profile and new_profile:
                    old_aggregate = old_profile["phases"]["small_field_aggregate_ms"]["mean_ms"]
                    new_aggregate = new_profile["phases"]["small_field_aggregate_ms"]["mean_ms"]
                    aggregate_ratio = f"{new_aggregate/old_aggregate:.3f}" if old_aggregate else "未定义（旧核均值为零）"
                    lines += [f"{group['name']} / N{d['log_n']} 的小域聚合阶段均值：旧核 {old_aggregate:.3f} ms，"
                              f"组合候选 {new_aggregate:.3f} ms（候选/旧核 = {aggregate_ratio}）。"
                              "这是该组 profile 样本的阶段比较，不与正式 prover 中位数拼接。", ""]
            for system, s in d["systems"].items():
                p = s["profile"]
                if p is None:
                    continue
                profiled = True
                labels = FLOCK_PHASES if system == "flock128" else PHASES
                balanced = p["samples"] % len(d["systems"]) == 0
                lines += [f"**{group['name']} / N{d['log_n']} / {system}**：{p['samples']} 个诊断样本，"
                          f"prover 均值 {p['prover_ms']['mean']:.3f} ms；诊断顺序{'位置平衡' if balanced else '位置不平衡'}。", "",
                          "| 阶段 | 均值 ms | 占诊断 prover 均值 |", "|---|---:|---:|"]
                for key, phase in p["phases"].items():
                    lines.append(f"| {labels.get(key, '未计入阶段的余项（含舍入）')} | {phase['mean_ms']:.3f} | {phase['share_of_mean_prover']*100:.2f}% |")
                lines.append("")
        if profiled:
            lines += [f"来源：[profile JSON]({group['name']}/profile-64t.json)、[Flock 阶段日志]({group['name']}/profile-64t.log)。", ""]
        else:
            lines += [f"{group['name']} 未运行 profile。", ""]
    lines += ["## 核验与来源", "",
              f"共核验 {validation['proof_records']} 条 proof 记录，包含 smoke、warmup、正式、profile 和 RSS；"
              f"服务器测试通过 {validation['passed_tests']} 项。各组 status=complete、全部命令零退出，命令日志 SHA256 与记录一致。"
              "同组及跨组 (N, run) 输入摘要一致；所有 Mikros 模式的 proof 字节摘要一致，Flock 自身亦一致。"
              "不同协议之间不要求 proof 摘要相同。", "",
              "本地补充检查：格式检查通过，Mikros 的 `clippy --all-targets --no-deps -- -D warnings` 通过。"
              "首次包含依赖的严格 Clippy 因既有 flock-field x86_64 未使用 import 失败；该依赖问题保留，"
              "不将其表述为整个依赖树无警告。此处与服务器 72 项测试记录区分。", "",
              "每条记录均完成 verifier、claims/transcript 一致性和独立完整源检查。输入生成/摘要、challenger 创建、"
              "序列化、源检查排除于 prove/verify 计时，但实际执行；源检查耗时单列。verifier 仅是归约重放。", "",
              f"源快照 base revision：`{validation['base_revision']}`（并非声称工作区无修改）；精确文件内容由"
              "[source manifest](source_manifest.json) 和 [源包](source.tar.gz) 确定。本报告各组共享同一个"
              "[build artifact](hardware_build_artifact.json)，N30 复用已测试 binary。", "",
              f"Executable SHA256：`{validation['executable_sha256']}`。", "",
              f"Mikros BLAKE3：`{validation['mikros_sources_blake3']}`。", "",
              f"Flock BLAKE3：`{validation['flock_sources_blake3']}`。", "",
              f"构建配置 BLAKE3：`{validation['build_configuration_blake3']}`。", "",
              "全部读取证据文件的 SHA256、后处理器 SHA256、配对数据及未四舍五入的统计在 [SUMMARY.json](SUMMARY.json)。"
              "后处理器不属于冻结源包，未修改其 manifest 或历史报告。", ""]
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True, help="downloaded v6-hardware root")
    parser.add_argument("--groups", default="screening,results", help="comma-separated completed result children")
    args = parser.parse_args()
    root = args.root.resolve()
    require(root.name == "v6-hardware", "only the v6-hardware report root is supported")
    names = args.groups.split(",")
    require(names and len(names) == len(set(names)) and all(re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9_.-]*", n) for n in names), "invalid groups")
    summary = summarize(root, names)
    rendered = render(summary)
    (root / "SUMMARY.json").write_text(json.dumps(summary, ensure_ascii=False, indent=2, allow_nan=False) + "\n", encoding="utf-8")
    (root / "REPORT.md").write_text(rendered, encoding="utf-8")
    print(f"Validated {summary['validation']['proof_records']} proof records; wrote {root / 'REPORT.md'} and SUMMARY.json")


if __name__ == "__main__":
    try:
        main()
    except (ValueError, KeyError, OSError, TypeError, IndexError) as error:
        raise SystemExit(f"hardware summary failed: {error}") from error
