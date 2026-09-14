#!/usr/bin/env python3
"""Summarize downloaded v7 register/packing experiments without changing frozen sources."""
from __future__ import annotations

import argparse
from collections import Counter
import json
from pathlib import Path
import re
import statistics

import run_register_64 as driver
from summarize_univariate import PHASES, FLOCK_PHASES, META_KEYS, all_samples, parse_flock_phases, stats


base = driver.base
require = driver.require
NAMES = {"reuse": "原始 Native reuse", "tiledreuse": "分块 tiledreuse",
         "basisreuse": "v6 基线 basisreuse", "registerreuse": "转换候选 registerreuse",
         "registerpackedreuse": "转换及打包候选 registerpackedreuse", "flock128": "Flock-128"}


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
    require(profile_runs in (0, len(systems)) and isinstance(rss, bool), f"{name}: diagnostic flags")
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
            require(observed == artifact["passed_tests"] and "univariate::domain::tests::" in output
                    and all(test in output for test in driver.REQUIRED_TESTS),
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
                       for baseline in ("basisreuse", "registerreuse", "reuse", "flock128")
                       if baseline in systems and baseline != system
                       and (baseline != "registerreuse" or system in ("registerpacked", "registerpackedreuse"))]
        dimensions.append({"log_n": n, "runs": main["metadata"]["runs"], "systems": systems, "comparisons": comparisons})
    return {"name": group["name"], "metadata": main["metadata"], "proof_records": group["proof_records"],
            "environment": {key: group["environment"][key] for key in (
                "hostname", "utc_start", "utc_end", "cpu_spec", "threads", "selected_physical_cores", "additional_smt_siblings",
                "toolchain", "skip_build", "profile_runs", "rss_requested", "numa_policy", "thread_policy",
                "loadavg_start", "loadavg_end", "memory_start_kib", "memory_end_kib", "executable_sha256")},
            "dimensions": dimensions}


def cross_version_identities(groups, prior_root):
    """Compare overlapping input/proof identities without mixing timing samples."""
    status_paths = [prior_root / name / "status.json" for name in ("screening", "results")]
    if not all(path.is_file() for path in status_paths):
        return None
    inputs, proofs, hashes = {}, {}, {}
    for status_path in status_paths:
        status = base.load(status_path)
        require(status["stage"] == "complete", "v6 reference group is incomplete")
        hashes[status_path.relative_to(prior_root).as_posix()] = base.sha256(status_path)
        for filename in status["reports"]:
            require(Path(filename).name == filename, "invalid v6 reference report path")
            path = status_path.parent / filename
            report = base.load(path)
            require(report["metadata"]["protocol"] == "mikros-univariate-gf256-v1", "v6 reference protocol")
            hashes[path.relative_to(prior_root).as_posix()] = base.sha256(path)
            for sample in all_samples(report):
                require(all(sample[key] is True for key in ("verifier_passed", "source_claims_verified", "transcript_matched")),
                        "v6 reference includes an unverified proof")
                key = sample["log_n"], sample["run"]
                require(inputs.setdefault(key, sample["input_digest"]) == sample["input_digest"], "v6 reference input inconsistency")
                identity = (*key, "flock128" if sample["system"] == "flock128" else "mikros-univariate")
                require(proofs.setdefault(identity, sample["proof_digest"]) == sample["proof_digest"], "v6 reference proof inconsistency")
    matched, unmatched, unique = 0, 0, set()
    for group in groups:
        for report in group["reports"].values():
            for sample in all_samples(report):
                key = sample["log_n"], sample["run"]
                identity = (*key, "flock128" if sample["system"] == "flock128" else "mikros-univariate")
                if identity not in proofs:
                    unmatched += 1
                    continue
                require(inputs[key] == sample["input_digest"] and proofs[identity] == sample["proof_digest"],
                        f"v6/v7 identity differs for {identity}")
                matched += 1
                unique.add(identity)
    return {"reference_version": "v6-hardware", "matched_records": matched, "records_without_reference": unmatched,
            "unique_matched_identities": len(unique), "reference_reports_sha256": hashes,
            "scope": "Overlapping (log_n,run,input_digest,protocol-family) identities only; no cross-version timing aggregation"}


def summarize(root, names):
    root = root.resolve()
    manifest = base.load(root / "source_manifest.json")
    artifact = base.load(root / "register_build_artifact.json")
    require(artifact["source_manifest"] == manifest and artifact["bench"] == driver.BENCH
            and artifact["toolchain"] == base.TOOLCHAIN and artifact["cpu_spec"] == base.CPU_SPEC
            and base.HEX64.fullmatch(artifact["executable_sha256"])
            and driver.REQUIRED_FEATURES <= set(artifact["native_target_features"]), "unverified native build artifact")
    validation = artifact["validation_commands"]
    require(len(validation) == len(driver.VALIDATION_STAGES)
            and {c["stage"] for c in validation} == driver.VALIDATION_STAGES
            and all(c["returncode"] == 0 for c in validation) and artifact["passed_tests"] >= 75
            and artifact["required_test_names"] == list(driver.REQUIRED_TESTS),
            "build/test validation missing")
    # The checker is part of the frozen package; this postprocessor is not.
    repository = Path(__file__).resolve().parents[2]
    for filename in ("run_register_64.py", "run_server_64.py"):
        relative = f"mikros/server_experiments/{filename}"
        require(base.sha256(repository / relative) == manifest["files_sha256"][relative], f"checker changed since snapshot: {filename}")
    require("mikros/server_experiments/summarize_register.py" not in manifest["files_sha256"], "postprocessor unexpectedly inside frozen source")
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
    paths = [root / "source_manifest.json", root / "register_build_artifact.json"]
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
    for filename in ("local-basis-assembly-audit.md", "local-packing-assembly-audit.md", "preflight.json"):
        note = root / "diagnostics" / filename
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
                                                for filename in ("summarize_register.py", "summarize_univariate.py")},
                       "cross_group_code_inputs_and_proofs_matched": True,
                       "cross_version_identities": cross_version_identities(groups, root.parent / "v6-hardware")},
        "preflight": base.load(root / "diagnostics/preflight.json") if (root / "diagnostics/preflight.json").exists() else None,
        "groups": [group_summary(group) for group in groups],
    }


def render(summary):
    first = summary["groups"][0]
    meta, env, validation = first["metadata"], first["environment"], summary["validation"]
    lines = ["# v7：寄存器转换与打包优化的同 binary 比较", ""]
    for group in summary["groups"]:
        for d in group["dimensions"]:
            if d["log_n"] != 30:
                continue
            for candidate in ("registerreuse", "registerpackedreuse"):
                if not {candidate, "basisreuse", "flock128"} <= set(d["systems"]):
                    continue
                values = {s: d["systems"][s]["prover_ms"] for s in (candidate, "basisreuse", "flock128")}
                new, old, flock = [values[s]["median"] for s in (candidate, "basisreuse", "flock128")]
                paired = next(c for c in d["comparisons"] if (c["numerator"], c["denominator"]) == (candidate, "flock128"))
                baseline_pair = next(c for c in d["comparisons"] if (c["numerator"], c["denominator"]) == (candidate, "basisreuse"))
                lines += [f"N=2^30 的 {group['name']} 组：{candidate} prover 中位数 {new:.3f} ms，"
                          f"v6 基线 basisreuse 为 {old:.3f} ms，Flock-128 为 {flock:.3f} ms。"
                          f"候选/基线中位数之比 = {new/old:.3f}，候选/Flock = {new/flock:.3f}；"
                          f"候选对 Flock 的配对比值中位数 = {paired['paired_ratios']['median']:.3f}，"
                          f"{d['runs']} 个配对中较快 {paired['numerator_wins']} 次。", "",
                          f"候选范围 [{values[candidate]['min']:.3f}, {values[candidate]['max']:.3f}] ms；"
                          f"Flock 范围 [{values['flock128']['min']:.3f}, {values['flock128']['max']:.3f}] ms。"
                          "上述差异仅描述这批配对样本，不据此保证稳定领先。", ""]
                lines += [f"相对 basisreuse，候选的逐输入配对比值中位数为 {baseline_pair['paired_ratios']['median']:.3f}，"
                          f"较快 {baseline_pair['numerator_wins']}/{d['runs']} 次；"
                          f"prover 均值为候选 {values[candidate]['mean']:.3f} ms、基线 {values['basisreuse']['mean']:.3f} ms。", ""]
                if (new / old - 1) * (baseline_pair["paired_ratios"]["median"] - 1) < 0:
                    lines += ["对基线的中位数比较与配对统计方向不同，不能只选择较有利的指标；本轮尚不能确认端到端改进。候选保持显式可选，默认路径不变。", ""]
                if new < flock and old < flock:
                    lines += ["本组基线和候选的中位数都低于 Flock，不能将两者与 Flock 的全部差距归因于新候选。", ""]
    lines += [f"环境：{meta['cpu']}，Linux x86_64，64 线程固定在 `{env['cpu_spec']}`；"
              "48 个物理核心加 16 个 SMT sibling，两 NUMA 节点各 32 个逻辑 CPU。"
              "Linux 默认 first-touch，无显式内存绑定/交错；服务器共享，CPU 不独占。", "",
              f"构建使用 `{env['toolchain']}`、`target-cpu=native`，GFNI/AVX512 后端及所需 ISA 特性已核验。"
              "各组使用相同冻结源码与 executable，按组和 N 分开统计。", "",
              "## 候选与边界", "",
              "`basisreuse` 是保持不变的 v6 基线；`registerreuse` 是同一坐标转换的寄存器优化候选。"
              "`registerpackedreuse` 还改变 A/B 打包中的 IP 消息累加方式：每个 chunk 内持续累加，到 chunk 末尾才水平合并。"
              "因此 registerpacked 不属于仅优化首轮聚合的消融。若列出 `reuse`，它仍代表最初的 Native 基线。", "",
              "冻结 metadata 中的 `across packing chunks` 措辞不精确；实现没有跨 Rayon 任务共享累加器，原始记录保留供追溯。", "",
              "这些是显式可选的实现路径：GF256 嵌入、扩域数学基、采样点、商多项式、挑战域和证明格式均不因此改变。"
              "同输入的全部 Mikros 路径必须给出相同 proof 字节及 claims/transcript。"
              "寄存器候选的名称本身不证明实际无 spill；缓存流量或带宽的变化也不能仅由耗时反推。", "",
              "比较的是 κ=128 的单变量 Hadamard PIOP，终止于直接 packed-opening claims；未包含额外 2κ 列聚合、PCS 或承诺打开。"
              "Flock 为 skip-7，同压缩比 128。BLAKE3、关闭 grinding，不作相同安全等级声明。", "",
              "## 分组计时", "",
              "每维度每系统一次 warmup，然后轮转执行；正式次数是系统数的整数倍，各系统在每个位置出现次数相同。"
              "每个 reuse 模式拥有独立 scratch，动态值重新计算；Flock 使用生产内存池。"
              "固定表初始化未扣除，发生在进程中的首次使用；每系统 warmup 不是独立冷启动。", ""]
    for group in summary["groups"]:
        for d in group["dimensions"]:
            lines += [f"### {group['name']} / N{d['log_n']}：每系统 {d['runs']} 次", "",
                      f"UTC：{group['environment']['utc_start']} 至 {group['environment']['utc_end']}。", "",
                      "| 系统 | prover 中位数 [min, max] ms | prover 均值 ms | verifier 中位数 ms | 源输入检查中位数 ms | warmup prover ms |",
                      "|---|---:|---:|---:|---:|---:|"]
            for system, record in d["systems"].items():
                p = record["prover_ms"]
                lines.append(f"| {NAMES.get(system, system)} | {p['median']:.3f} [{p['min']:.3f}, {p['max']:.3f}] | "
                             f"{p['mean']:.3f} | {record['verifier_ms']['median']:.4f} | {record['input_claim_check_ms']['median']:.3f} | {record['warmup_prover_ms']:.3f} |")
            lines += ["", "| 分子/分母 | 两中位数之比 | 配对比值中位数 [min, max] | 分子较快 / 配对数 |", "|---|---:|---:|---:|"]
            for c in d["comparisons"]:
                p = c["paired_ratios"]
                lines.append(f"| {c['numerator']}/{c['denominator']} | {c['ratio_of_medians']:.3f} | "
                             f"{p['median']:.3f} [{p['min']:.3f}, {p['max']:.3f}] | {c['numerator_wins']}/{p['samples']} |")
            if d["log_n"] == 30:
                if {"registerreuse", "registerpackedreuse"} <= set(d["systems"]):
                    lines += ["", "本 N30 组同时包含转换候选和转换+打包候选，可给出同组增量比较；"
                              "各阶段归因仍需结合独立 profile 与波动，不能从总耗时精确分摊贡献。"]
                else:
                    lines += ["", "本 N30 组未同时包含 registerreuse 与 registerpackedreuse，不能分别量化转换优化和额外打包优化的独立贡献。"]
            lines += ["", f"来源：[计时]({group['name']}/comparison-64t.json)、[环境]({group['name']}/server_environment.json)。", ""]
    lines += ["配对键为同组 (N, run)，并核对完整 input digest；比值小于 1 表示分子更快。"
              "两系统中位数之比与逐输入比值中位数不同。逐 run 时间及摘要保存在 SUMMARY.json。"
              "N26 若用于候选筛选，其样本不并入后续 N30；样本范围和胜场不保证统计显著性或跨机器性能。", "",
              "## 通信与独立 RSS", "", "| 组 / N | 系统 | F128 元素 | payload bytes | bincode bytes | prove 前 VmRSS MiB | prove 后 VmHWM MiB | 冷 proof ms |",
              "|---|---|---:|---:|---:|---:|---:|---:|"]
    for group in summary["groups"]:
        for d in group["dimensions"]:
            for system, record in d["systems"].items():
                rss = record["rss"]
                memory = f"{rss['vm_rss_before_prove_kib']/1024:.2f} | {rss['vm_hwm_after_prove_kib']/1024:.2f} | {rss['prover_ms']:.3f}" if rss else "未运行 | 未运行 | 未运行"
                lines.append(f"| {group['name']} / {d['log_n']} | {system} | {record['payload_field_elements']} | {record['payload_field_bytes']} | {record['bincode_bytes']} | {memory} |")
    lines += ["", "RSS 每系统/维度来自一个独立进程的一次冷 proof；VmHWM 在 prove 后、verify 与源输入检查前读取。"
              "它包含输入及运行时，不是 allocator 专属峰值；冷计时不混入正式统计。未执行的 RSS 不推测。", "",
              "## 阶段诊断", "", "profile 是独立进程的一次完整轮转，各位置平衡；每系统诊断次数等于所选系统数，另有一次 warmup。"
              "阶段均值和占比排除 warmup：占比为阶段均值/该系统同组诊断 prover 均值，余项逐样本计算再取均值。"
              "计时器有开销，Flock 日志保留 0.01 ms；诊断不并入正式计时。", ""]
    for group in summary["groups"]:
        profiled = False
        for d in group["dimensions"]:
            if {"basisreuse", "registerpackedreuse"} <= set(d["systems"]):
                baseline_profile = d["systems"]["basisreuse"]["profile"]
                candidate_profile = d["systems"]["registerpackedreuse"]["profile"]
                if baseline_profile and candidate_profile:
                    baseline_pack = baseline_profile["phases"]["ab_pack_ms"]["mean_ms"]
                    candidate_pack = candidate_profile["phases"]["ab_pack_ms"]["mean_ms"]
                    lines += [f"{group['name']} / N{d['log_n']} 的 AB 打包阶段均值为基线 {baseline_pack:.3f} ms、"
                              f"候选 {candidate_pack:.3f} ms；同组诊断 prover 总均值为基线 {baseline_profile['prover_ms']['mean']:.3f} ms、"
                              f"候选 {candidate_profile['prover_ms']['mean']:.3f} ms。", ""]
                    if d["log_n"] == 30 and candidate_pack >= baseline_pack:
                        lines += [f"N30 的 {candidate_profile['samples']} 个诊断样本未复现 N26 打包阶段的缩短。"
                                  "诊断样本少、结果有波动，不能宣称 N30 打包耗时减半或稳定加速；"
                                  "这些阶段均值也不与正式计时混合。", ""]
            for system, record in d["systems"].items():
                profile = record["profile"]
                if profile is None:
                    continue
                profiled = True
                labels = FLOCK_PHASES if system == "flock128" else PHASES
                lines += [f"**{group['name']} / N{d['log_n']} / {system}**：{profile['samples']} 个诊断样本，"
                          f"prover 均值 {profile['prover_ms']['mean']:.3f} ms。", "",
                          "| 阶段 | 均值 ms | 占诊断 prover 均值 |", "|---|---:|---:|"]
                for key, phase in profile["phases"].items():
                    lines.append(f"| {labels.get(key, '未计入阶段的余项（含舍入）')} | {phase['mean_ms']:.3f} | {phase['share_of_mean_prover']*100:.2f}% |")
                lines.append("")
        lines += ([f"来源：[profile JSON]({group['name']}/profile-64t.json)、[Flock 阶段日志]({group['name']}/profile-64t.log)。", ""]
                  if profiled else [f"{group['name']} 未执行 profile。", ""])
    for note, label in (("diagnostics/local-basis-assembly-audit.md", "本地坐标转换汇编诊断"),
                        ("diagnostics/local-packing-assembly-audit.md", "本地打包汇编诊断")):
        if note in validation["artifact_sha256"]:
            lines += [f"[{label}]({note})。指令与内存操作数分析不等于实际 cache miss、DRAM 流量或硬件事件计数。", ""]
    if summary["preflight"] is not None:
        preflight = summary["preflight"]
        lines += [f"[本轮前置检查](diagnostics/preflight.json)记录 `numactl={json.dumps(preflight['numactl'])}`、"
                  f"`perf_event_paranoid={preflight['perf_event_paranoid']}`。本轮未实际尝试 perf 采集，也未实施 NUMA 调优；"
                  "这些前置状态不能当作 v7 的 perf 失败日志或性能计数器结果。", ""]
    lines += ["## 核验与来源", "",
              f"共核验 {validation['proof_records']} 条 proof 记录（含 smoke、warmup、正式、profile、RSS），服务器测试通过 {validation['passed_tests']} 项。"
              "各组 status=complete，全部命令零退出，日志 SHA256 与命令记录一致；组间共用相同 code/compiler/ISA/可执行文件。", "",
              "本地运行时测试通过 75 项；显式指定 `--target x86_64-unknown-linux-gnu`、"
              "`RUSTFLAGS='-C target-cpu=icelake-server'` 的 `clippy --all-targets --no-deps -- -D warnings` 通过，格式检查通过。"
              "该 Clippy 结论限于 Mikros 检查范围，不外推到整个依赖树。服务器测试数与上述本地检查分别记录。", "",
              "全部记录均完成 verifier、claims/transcript 一致性和独立完整源输入检查；同 (N, run) 输入摘要一致，"
              "全部 Mikros 模式的 proof 摘要一致，Flock 自身亦一致。不同协议间不要求 proof 摘要相同。"
              "输入生成/摘要、challenger 创建、序列化、源检查排除于 prove/verify 计时，但仍实际执行；verifier 仅为归约重放。", "",
              f"冻结源码 base revision：`{validation['base_revision']}`；这不等于工作区无修改，精确文件由"
              "[source manifest](source_manifest.json) 和[源包](source.tar.gz)确定。"
              "各组使用同一 [register build artifact](register_build_artifact.json)。", "",
              f"Executable SHA256：`{validation['executable_sha256']}`。", "",
              f"Mikros BLAKE3：`{validation['mikros_sources_blake3']}`。", "",
              f"Flock BLAKE3：`{validation['flock_sources_blake3']}`。", "",
              f"构建配置 BLAKE3：`{validation['build_configuration_blake3']}`。", "",
              "证据 SHA256、后处理器 SHA256、配对记录及未四舍五入统计见 [SUMMARY.json](SUMMARY.json)。"
              "v5/v6 驱动、校验器和历史结果保持原状，历史样本不与 v7 混合；本后处理器位于冻结源包之外。", ""]
    cross_version = validation["cross_version_identities"]
    if cross_version:
        lines += [f"另与 v6 已完成记录核对重叠的输入/证明身份：v7 中 {cross_version['matched_records']} 条记录匹配，"
                  f"涵盖 {cross_version['unique_matched_identities']} 个不同 (N, run, 协议族)；"
                  f"{cross_version['records_without_reference']} 条记录没有对应 v6 run，未声称经过跨版本核对。"
                  "这只检验同输入下证明摘要一致，不混合源码指纹、计时样本或据此计算跨版本加速比。", ""]
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True, help="downloaded v7-register root")
    parser.add_argument("--groups", default="screening,results", help="comma-separated completed result children")
    args = parser.parse_args()
    root = args.root.resolve()
    require(root.name == "v7-register", "only the v7-register report root is supported")
    names = args.groups.split(",")
    require(names and len(names) == len(set(names)) and all(re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9_.-]*", name) for name in names), "invalid groups")
    summary = summarize(root, names)
    rendered = render(summary)
    (root / "SUMMARY.json").write_text(json.dumps(summary, ensure_ascii=False, indent=2, allow_nan=False) + "\n", encoding="utf-8")
    (root / "REPORT.md").write_text(rendered, encoding="utf-8")
    print(f"Validated {summary['validation']['proof_records']} proof records; wrote {root / 'REPORT.md'} and SUMMARY.json")


if __name__ == "__main__":
    try:
        main()
    except (ValueError, KeyError, OSError, TypeError, IndexError) as error:
        raise SystemExit(f"register summary failed: {error}") from error

