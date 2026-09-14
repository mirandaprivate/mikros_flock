#!/usr/bin/env python3
"""Summarize downloaded v8 first-aggregation experiments without changing frozen sources."""
from __future__ import annotations

import argparse
from collections import Counter
from datetime import datetime
import json
from pathlib import Path
import re
import statistics

import run_aggregate_64 as driver
from summarize_univariate import PHASES, FLOCK_PHASES, META_KEYS, all_samples, parse_flock_phases, stats


base = driver.base
require = driver.require
NAMES = {"basisreuse": "v6 基线 basisreuse", "splitreuse": "分离消费 splitreuse",
         "halfreuse": "半块消费 halfreuse", "pairreuse": "双输入展开 pairreuse",
         "affinereuse": "H 仿射候选 affinereuse", "jointreuse": "组合候选 jointreuse", "flock128": "Flock-128"}


def read_group(root, name, manifest, artifact):
    directory = root / name
    status = base.load(directory / "status.json")
    environment = base.load(directory / "server_environment.json")
    commands = base.load(directory / "commands.json")
    require(status["stage"] == "complete", f"{name}: driver is not complete")
    systems, logs, runs = status["systems"], status["logs"], status["measured_runs"]
    require(driver.parse_systems(",".join(systems)) == systems
            and runs > 0 and runs % len(systems) == 0, f"{name}: systems or position balance")
    smoke = status["smoke_systems"]
    require(driver.parse_systems(",".join(smoke)) == smoke, f"{name}: smoke systems")
    required_tests, minimum_tests = status["required_test_names"], status["minimum_tests"]
    require(required_tests and len(set(required_tests)) == len(required_tests)
            and minimum_tests >= driver.DEFAULT_MIN_TESTS, f"{name}: test requirements")
    require(logs and len(set(logs)) == len(logs) and all(14 <= n <= 30 for n in logs), f"{name}: dimensions")
    profile_runs, rss = status["profile_runs"], status["rss_requested"]
    require(profile_runs in (0, len(systems)) and isinstance(rss, bool), f"{name}: diagnostic flags")
    specifications = {
        "smoke.json": (smoke, [14], 1, {}),
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
    require(environment["smoke_systems"] == smoke and environment["required_test_names"] == required_tests
            and environment["minimum_tests"] == minimum_tests and artifact["passed_tests"] >= minimum_tests
            and set(required_tests) <= set(artifact["required_test_names"]), f"{name}: test/smoke configuration")
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
                    and driver.required_tests_passed(output, artifact["required_test_names"]),
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


def comparison_pairs(systems, *, include_flock):
    pairs = [(system, baseline) for system in systems if system != "flock128"
             for baseline in (("basisreuse", "flock128") if include_flock else ("basisreuse",))
             if baseline in systems and baseline != system]
    pairs.extend((numerator, denominator) for numerator, denominator in
                 (("affinereuse", "splitreuse"), ("jointreuse", "pairreuse"))
                 if numerator in systems and denominator in systems)
    return pairs


def diagnostic_aggregate_comparisons(profile, n, systems):
    """Pair aggregate timers only within the independent profile process."""
    if profile is None or "basisreuse" not in systems:
        return []
    require(profile["metadata"]["profile"] is True, "aggregation pairs require profile data")
    stage = "small_field_aggregate_ms"
    selected = []
    for sample in profile["samples"]:
        if sample["log_n"] != n or sample["system"] == "flock128":
            continue
        value = sample["mikros_phases"][stage]
        require(value > 0, "cannot form aggregate timer ratios with a zero timer")
        # Reuse the paired arithmetic helper with the timer as its input metric;
        # no end-to-end samples or warmups enter this diagnostic dataset.
        selected.append({"system": sample["system"], "run": sample["run"],
                         "input_digest": sample["input_digest"], "prover_ms": value})
    return [{**paired_comparison(selected, numerator, denominator), "stage": stage,
             "mode": "independent_profile_diagnostic"}
            for numerator, denominator in comparison_pairs(systems, include_flock=False)]


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
                series = {key: [p[key] for p in timers] for key in keys}
                series["unaccounted_ms"] = [s["prover_ms"] - sum(p.values()) for s, p in zip(diagnostic, timers)]
                phases = {"samples": len(diagnostic), "prover_ms": stats([s["prover_ms"] for s in diagnostic]),
                          "phases": {key: {"statistics_ms": stats(values), "mean_ms": statistics.mean(values),
                                           "share_of_mean_prover": statistics.mean(values) / total}
                                     for key, values in series.items()}}
                if system != "flock128":
                    phases["non_aggregate_ms"] = stats([s["prover_ms"] - p["small_field_aggregate_ms"]
                                                        for s, p in zip(diagnostic, timers)])
            systems[system] = {
                **{key: stats([s[key] for s in samples]) for key in ("prover_ms", "verifier_ms", "input_claim_check_ms")},
                **{key: samples[0][key] for key in ("payload_field_elements", "payload_field_bytes", "bincode_bytes")},
                "warmup_prover_ms": warmup["prover_ms"], "profile": phases,
                "rss": {key: rss[key] for key in ("vm_rss_before_prove_kib", "vm_hwm_after_prove_kib", "prover_ms", "input_claim_check_ms")} if rss else None,
            }
        comparisons = [paired_comparison(selected, numerator, denominator)
                       for numerator, denominator in comparison_pairs(systems, include_flock=True)]
        dimensions.append({"log_n": n, "runs": main["metadata"]["runs"], "systems": systems, "comparisons": comparisons,
                           "diagnostic_aggregate_comparisons": diagnostic_aggregate_comparisons(profile, n, systems)})
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
        require(status["stage"] == "complete", "v7 reference group is incomplete")
        hashes[status_path.relative_to(prior_root).as_posix()] = base.sha256(status_path)
        for filename in status["reports"]:
            require(Path(filename).name == filename, "invalid v7 reference report path")
            path = status_path.parent / filename
            report = base.load(path)
            require(report["metadata"]["protocol"] == "mikros-univariate-gf256-v1", "v7 reference protocol")
            hashes[path.relative_to(prior_root).as_posix()] = base.sha256(path)
            for sample in all_samples(report):
                require(all(sample[key] is True for key in ("verifier_passed", "source_claims_verified", "transcript_matched")),
                        "v7 reference includes an unverified proof")
                key = sample["log_n"], sample["run"]
                require(inputs.setdefault(key, sample["input_digest"]) == sample["input_digest"], "v7 reference input inconsistency")
                identity = (*key, "flock128" if sample["system"] == "flock128" else "mikros-univariate")
                require(proofs.setdefault(identity, sample["proof_digest"]) == sample["proof_digest"], "v7 reference proof inconsistency")
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
                        f"v7/v8 identity differs for {identity}")
                matched += 1
                unique.add(identity)
    return {"reference_version": "v7-register", "matched_records": matched, "records_without_reference": unmatched,
            "unique_matched_identities": len(unique), "reference_reports_sha256": hashes,
            "scope": "Overlapping (log_n,run,input_digest,protocol-family) identities only; no cross-version timing aggregation"}


def summarize(root, names):
    root = root.resolve()
    manifest = base.load(root / "source_manifest.json")
    artifact = base.load(root / "aggregate_build_artifact.json")
    require(artifact["source_manifest"] == manifest and artifact["bench"] == driver.BENCH
            and artifact["toolchain"] == base.TOOLCHAIN and artifact["cpu_spec"] == base.CPU_SPEC
            and base.HEX64.fullmatch(artifact["executable_sha256"])
            and driver.REQUIRED_FEATURES <= set(artifact["native_target_features"]), "unverified native build artifact")
    validation = artifact["validation_commands"]
    require(len(validation) == len(driver.VALIDATION_STAGES)
            and {c["stage"] for c in validation} == driver.VALIDATION_STAGES
            and all(c["returncode"] == 0 for c in validation) and artifact["passed_tests"] >= artifact["minimum_tests"] >= driver.DEFAULT_MIN_TESTS
            and artifact["required_test_names"],
            "build/test validation missing")
    # The checker is part of the frozen package; this postprocessor is not.
    repository = Path(__file__).resolve().parents[2]
    for filename in ("run_aggregate_64.py", "run_server_64.py"):
        relative = f"mikros/server_experiments/{filename}"
        require(base.sha256(repository / relative) == manifest["files_sha256"][relative], f"checker changed since snapshot: {filename}")
    require("mikros/server_experiments/summarize_aggregate.py" not in manifest["files_sha256"], "postprocessor unexpectedly inside frozen source")
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
    paths = [root / "source_manifest.json", root / "aggregate_build_artifact.json"]
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
    for filename in ("local-aggregate-assembly-audit.md", "local-pair-assembly-audit.md",
                     "local-consumer-assembly-audit.md", "preflight.json"):
        note = root / "diagnostics" / filename
        if note.exists():
            paths.append(note)
    assembly_path = root / "diagnostics/assembly_manifest.json"
    if assembly_path.exists():
        assembly = base.load(assembly_path)
        require(assembly["executable_sha256"] == artifact["executable_sha256"], "assembly uses a different executable")
        paths.append(assembly_path)
        for function in assembly["functions"]:
            path = (root / function["file"]).resolve()
            require(path.is_relative_to(root) and base.sha256(path) == function["sha256"], "assembly excerpt SHA256 mismatch")
            paths.append(path)
    local_test_count = None
    local_test_path = root / "local-tests.log"
    if local_test_path.exists():
        output = local_test_path.read_text(encoding="utf-8")
        local_test_count = sum(map(int, re.findall(r"test result: ok\. (\d+) passed", output)))
        require(local_test_count > 0 and "test result: FAILED" not in output
                and driver.required_tests_passed(output, artifact["required_test_names"]), "local test log lacks required passing tests")
        paths.append(local_test_path)
    selection_path = root / "selection.json"
    selection = base.load(selection_path) if selection_path.exists() else None
    if selection:
        for group in groups:
            meta = group["reports"]["comparison-64t.json"]["metadata"]
            if 30 not in meta["logs"]:
                continue
            require(all(meta[key] == selection[key] for key in ("systems", "logs", "runs"))
                    and group["environment"]["profile_runs"] == selection["profile_runs"]
                    and group["environment"]["rss_requested"] == selection["rss"], "N30 differs from recorded selection")
            require(datetime.fromisoformat(selection["utc"]) < datetime.fromisoformat(group["environment"]["utc_start"]),
                    "candidate selection was not recorded before N30 launch")
        paths.append(selection_path)
    if (root / "PLAN.md").exists():
        paths.append(root / "PLAN.md")
    source_audit_path = root / "source-audit.json"
    source_audit = base.load(source_audit_path) if source_audit_path.exists() else None
    if source_audit:
        require(source_audit["versions"]["v8-aggregate"]["source_archive_sha256"] == base.sha256(root / "source.tar.gz")
                and source_audit["versions"]["v8-aggregate"]["source_manifest_sha256"] == base.sha256(root / "source_manifest.json"),
                "source audit refers to a different snapshot")
        require(all(check["whitespace_removed_body_equal"] for check in source_audit["baseline_function_checks"]),
                "baseline source function changed")
        paths.append(source_audit_path)
    return {
        "schema_version": 1, "protocol": reference["protocol"],
        "validation": {"proof_records": sum(g["proof_records"] for g in groups), "passed_tests": artifact["passed_tests"],
                       "base_revision": manifest["base_revision"], "executable_sha256": artifact["executable_sha256"],
                       "mikros_sources_blake3": reference["mikros_sources_blake3"], "flock_sources_blake3": reference["flock_sources_blake3"],
                       "build_configuration_blake3": reference["build_configuration_blake3"],
                       "artifact_sha256": {path.relative_to(root).as_posix(): base.sha256(path) for path in paths},
                       "postprocessor_sha256": {filename: base.sha256(Path(__file__).with_name(filename))
                                                for filename in ("summarize_aggregate.py", "summarize_univariate.py")},
                       "cross_group_code_inputs_and_proofs_matched": True,
                       "cross_version_identities": cross_version_identities(groups, root.parent / "v7-register")},
        "preflight": base.load(root / "diagnostics/preflight.json") if (root / "diagnostics/preflight.json").exists() else None,
        "source_audit": source_audit,
        "selection": selection,
        "local_test_count": local_test_count,
        "groups": [group_summary(group) for group in groups],
    }


def render(summary):
    first = summary["groups"][0]
    meta, env, validation = first["metadata"], first["environment"], summary["validation"]
    lines = ["# v8：首轮聚合优化与 Flock-128 的同 binary 比较", ""]
    for group in summary["groups"]:
        for d in group["dimensions"]:
            if d["log_n"] != 30:
                continue
            if all(name in d["systems"] for name in ("basisreuse", "pairreuse", "jointreuse", "flock128")):
                pair, joint = (d["systems"][name]["prover_ms"] for name in ("pairreuse", "jointreuse"))
                comparisons = [c for c in d["comparisons"] if c["numerator"] == "pairreuse"
                               and c["denominator"] in ("basisreuse", "flock128")]
                if (len(comparisons) == 2 and all(c["numerator_wins"] == d["runs"] for c in comparisons)
                        and joint["median"] < pair["median"] and pair["mean"] < joint["mean"] and pair["max"] < joint["max"]):
                    lines += [f"按本批独立 N30 的 {d['runs']} 次测量，推荐显式选择 **pairreuse** 作为较稳妥的工程选项："
                              "它每次均快于同组 basisreuse 和 Flock-128。"
                              f"Joint 中位数更低，但均值 {joint['mean']:.3f} ms 高于 Pair 的 {pair['mean']:.3f} ms，"
                              f"最大值 {joint['max']:.3f} ms，高于 Pair 的 {pair['max']:.3f} ms。"
                              "保留所有长尾样本；这不证明跨机器稳定性或绝对最快，也不自动改变默认内核。", ""]
            for c in d["comparisons"]:
                if c["denominator"] != "basisreuse":
                    continue
                candidate = c["numerator"]
                new = d["systems"][candidate]["prover_ms"]
                baseline = d["systems"]["basisreuse"]["prover_ms"]
                lines += [f"N=2^30 的 {group['name']} 组：{candidate} prover 中位数 {new['median']:.3f} ms，"
                          f"basisreuse 基线 {baseline['median']:.3f} ms；两中位数之比 {c['ratio_of_medians']:.3f}。"
                          f"同输入配对比值中位数 {c['paired_ratios']['median']:.3f}，{c['numerator_wins']}/{d['runs']} 次候选较快。"
                          f"prover 均值分别为 {new['mean']:.3f} / {baseline['mean']:.3f} ms。", ""]
                if (c["ratio_of_medians"] - 1) * (c["paired_ratios"]["median"] - 1) < 0:
                    lines += ["原始中位数与配对统计方向不同，不能只采用较有利的指标来宣称端到端改进。", ""]
                against_flock = next((p for p in d["comparisons"] if p["numerator"] == candidate and p["denominator"] == "flock128"), None)
                if against_flock:
                    flock = d["systems"]["flock128"]["prover_ms"]
                    lines += [f"同组 Flock-128 prover 中位数 {flock['median']:.3f} ms；候选/Flock 的两中位数之比 "
                              f"{against_flock['ratio_of_medians']:.3f}，配对比值中位数 {against_flock['paired_ratios']['median']:.3f}，"
                              f"候选较快 {against_flock['numerator_wins']}/{d['runs']} 次。"
                              "与 Flock 的全部差距不能自动归因于新增聚合优化，应同时看同组 basisreuse 对照。", ""]
            joint_pair = next((c for c in d["comparisons"] if (c["numerator"], c["denominator"]) == ("jointreuse", "pairreuse")), None)
            if joint_pair:
                joint, pair = (d["systems"][name]["prover_ms"] for name in ("jointreuse", "pairreuse"))
                mean_ratio = joint["mean"] / pair["mean"]
                lines += [f"Joint/Pair：两中位数之比 {joint_pair['ratio_of_medians']:.3f}，均值之比 {mean_ratio:.3f}；"
                          f"同输入配对比值中位数 {joint_pair['paired_ratios']['median']:.3f}，Joint 较快 {joint_pair['numerator_wins']}/{d['runs']} 次。"
                          f"样本范围分别为 [{joint['min']:.3f}, {joint['max']:.3f}] / [{pair['min']:.3f}, {pair['max']:.3f}] ms。", ""]
                if (joint_pair["ratio_of_medians"] - 1) * (mean_ratio - 1) < 0:
                    lines += ["两者中位数与均值的排序相反，长尾记录仍全部保留；不能只按较低中位数判定整体最快。", ""]
    lines += [f"环境：{meta['cpu']}，Linux x86_64，64 线程固定在 `{env['cpu_spec']}`；"
              "48 个物理核心加 16 个 SMT sibling，两 NUMA 节点各 32 个逻辑 CPU。"
              "Linux 默认 first-touch，无显式内存绑定/交错；共享服务器，CPU 不独占。", "",
              f"`{env['toolchain']}`、`target-cpu=native`，GFNI/AVX512 及所需 ISA 特性已核验。"
              "各组使用同一冻结源码与 executable，分别统计 N26 筛选和独立 N30 测量。", "",
              "## 实现与协议边界", "",
              "`basisreuse` 保留 v6 基线。split/half 改变首轮消费方式；pair 改变首轮 A/B continuation 展开；"
              "affine 在 split 基础上优化 H 部分；joint 组合 pair 与 affine。这些路径保持 Batch 坐标转换与其他协议阶段一致，"
              "不混入 v7 的额外 A/B 打包优化。候选名称与本文数值都以实际 systems 列表为准。", "",
              "GF256 嵌入、扩域数学基、采样点、商多项式、挑战域、证明格式保持一致；所有 Mikros 内核必须对同输入给出相同证明摘要。"
              "代码或汇编中的寄存器与访存变化不能直接当作实测 cache miss、DRAM 带宽或端到端加速。", "",
              "比较边界为 κ=128 的单变量 PIOP，终止于直接 packed-opening claims，不含额外 2κ 列聚合、PCS 或承诺打开。"
              "Flock 使用 skip-7，同压缩比 128；BLAKE3、关闭 grinding，不作相同安全等级声明。", "",
              "## 正式端到端计时", "",
              "每系统每维度一次 warmup，随后按 run 轮转，各位置出现次数相同；各 reuse 模式独立保留 scratch，动态值重新计算。"
              "Flock 使用生产内存池。固定表首次初始化未扣除；进程中的各系统 warmup 不等于各自独立冷启动。", ""]
    if summary["selection"] is not None:
        selection = summary["selection"]
        lines += [f"[预先记录的候选选择](selection.json)于 `{selection['utc']}` 保留 "
                  f"`{', '.join(selection['systems'])}`，N30 每系统 {selection['runs']} 次正式测量，"
                  f"另有 {selection['profile_runs']} 次独立 profile。N26 的 pair/joint 均值与中位数排序不同，"
                  "因此两者都进入独立 N30；未合并筛选数据。N30 结果纳入本报告时，核验配置与选择时间早于启动。", ""]
    for group in summary["groups"]:
        for d in group["dimensions"]:
            lines += [f"### {group['name']} / N{d['log_n']}：每系统 {d['runs']} 次", "",
                      f"UTC：{group['environment']['utc_start']} 至 {group['environment']['utc_end']}。", "",
                      "| 系统 | prover 中位数 [min, max] ms | prover 均值 ms | verifier 中位数 ms | 源检查中位数 ms | warmup prover ms |",
                      "|---|---:|---:|---:|---:|---:|"]
            for system, s in d["systems"].items():
                p = s["prover_ms"]
                lines.append(f"| {NAMES.get(system, system)} | {p['median']:.3f} [{p['min']:.3f}, {p['max']:.3f}] | {p['mean']:.3f} | "
                             f"{s['verifier_ms']['median']:.4f} | {s['input_claim_check_ms']['median']:.3f} | {s['warmup_prover_ms']:.3f} |")
            lines += ["", "| 正式计时分子/分母 | 两中位数之比 | 配对比值中位数 [min, max] | 分子较快 / 样本数 |", "|---|---:|---:|---:|"]
            for c in d["comparisons"]:
                p = c["paired_ratios"]
                lines.append(f"| {c['numerator']}/{c['denominator']} | {c['ratio_of_medians']:.3f} | {p['median']:.3f} [{p['min']:.3f}, {p['max']:.3f}] | {c['numerator_wins']}/{p['samples']} |")
            lines += ["", f"来源：[正式计时]({group['name']}/comparison-64t.json)、[环境]({group['name']}/server_environment.json)。", ""]
    lines += ["端到端配对使用同组 (N, run)，并核对完整输入摘要；比值小于 1 表示分子更快。"
              "N26 筛选样本不并入 N30，未测量的候选不分配 N30 独立贡献。均值、中位数、配对比值是不同统计量；"
              "样本范围与胜场不保证统计显著性或跨机器稳定性。", "",
              "## 独立 profile：聚合阶段", "",
              "本节全部数据来自另一个 profile 进程，其次数等于系统数，另有一次 warmup；这里排除 warmup，完整轮转一次。"
              "聚合阶段配对只使用同一 profile 组的相同 run/input，不使用正式组的时钟值。"
              "阶段比值仅描述诊断样本，不能当作主组端到端配对加速。非聚合余量逐样本以 prover−aggregate 计算。"
              "计时器有开销且诊断样本少。", ""]
    for group in summary["groups"]:
        for d in group["dimensions"]:
            selected = {name: s["profile"] for name, s in d["systems"].items() if name != "flock128" and s["profile"] is not None}
            if not selected:
                lines += [f"{group['name']} / N{d['log_n']} 未运行 Mikros profile。", ""]
                continue
            lines += [f"### {group['name']} / N{d['log_n']} 的 aggregate 诊断", "",
                      "| 系统 | 样本数 | aggregate 中位数 [min, max] ms | aggregate 均值 ms | 占诊断总均值 | 非聚合均值 ms | 诊断 prover 均值 ms |",
                      "|---|---:|---:|---:|---:|---:|---:|"]
            for system, p in selected.items():
                phase = p["phases"]["small_field_aggregate_ms"]
                t = phase["statistics_ms"]
                lines.append(f"| {system} | {t['samples']} | {t['median']:.3f} [{t['min']:.3f}, {t['max']:.3f}] | "
                             f"{t['mean']:.3f} | {phase['share_of_mean_prover']*100:.2f}% | {p['non_aggregate_ms']['mean']:.3f} | {p['prover_ms']['mean']:.3f} |")
            lines += ["", "| aggregate 诊断分子/分母 | 阶段中位数之比 | 同 profile run 比值中位数 [min, max] | 分子较快 / 诊断样本数 |", "|---|---:|---:|---:|"]
            for c in d["diagnostic_aggregate_comparisons"]:
                p = c["paired_ratios"]
                lines.append(f"| {c['numerator']}/{c['denominator']} | {c['ratio_of_medians']:.3f} | {p['median']:.3f} [{p['min']:.3f}, {p['max']:.3f}] | {c['numerator_wins']}/{p['samples']} |")
            lines += ["", f"来源：[独立 profile JSON]({group['name']}/profile-64t.json)。", ""]
            if d["log_n"] == 30 and "basisreuse" in selected:
                baseline = selected["basisreuse"]
                aggregate = baseline["phases"]["small_field_aggregate_ms"]["mean_ms"]
                for name, profile in selected.items():
                    if name == "basisreuse":
                        continue
                    change = (profile["phases"]["small_field_aggregate_ms"]["mean_ms"] / aggregate - 1) * 100
                    lines += [f"仅在这 {profile['samples']} 次 N30 profile 中，{name} 的聚合均值相对基线变化 {change:+.2f}%；"
                              f"非聚合余量均值 {profile['non_aggregate_ms']['mean']:.3f} ms，基线为 {baseline['non_aggregate_ms']['mean']:.3f} ms。"
                              f"诊断总均值分别为 {profile['prover_ms']['mean']:.3f} / {baseline['prover_ms']['mean']:.3f} ms。", ""]
                lines += ["本组阶段与非聚合余量都有波动，不沿用 N26 的阶段分布来解释 N30。"
                          "独立 profile 的总耗时也不与正式计时相减或拼接；这些数据未测定 cache、NUMA、频率或其他波动原因。", ""]
    lines += ["## 独立 profile：全部阶段均值", "",
              "占比=阶段均值/同系统诊断 prover 均值；余项逐样本计算后取均值。Flock 阶段来自保留 0.01 ms 的日志。"
              "全部阶段的 median/min/max/count 与逐 run aggregate 配对记录均在 SUMMARY.json。", ""]
    for group in summary["groups"]:
        for d in group["dimensions"]:
            for system, s in d["systems"].items():
                p = s["profile"]
                if p is None:
                    continue
                labels = FLOCK_PHASES if system == "flock128" else PHASES
                lines += [f"**{group['name']} / N{d['log_n']} / {system}**：{p['samples']} 样本，诊断 prover 均值 {p['prover_ms']['mean']:.3f} ms。", "",
                          "| 阶段 | 均值 ms | 占诊断总均值 |", "|---|---:|---:|"]
                for key, phase in p["phases"].items():
                    lines.append(f"| {labels.get(key, '未计入阶段的余项（含舍入）')} | {phase['mean_ms']:.3f} | {phase['share_of_mean_prover']*100:.2f}% |")
                lines.append("")
    lines += ["## 通信与独立 RSS", "", "| 组 / N | 系统 | F128 元素 | payload bytes | bincode bytes | prove 前 VmRSS MiB | prove 后 VmHWM MiB | 冷 proof ms |",
              "|---|---|---:|---:|---:|---:|---:|---:|"]
    for group in summary["groups"]:
        for d in group["dimensions"]:
            for system, s in d["systems"].items():
                rss = s["rss"]
                memory = f"{rss['vm_rss_before_prove_kib']/1024:.2f} | {rss['vm_hwm_after_prove_kib']/1024:.2f} | {rss['prover_ms']:.3f}" if rss else "未运行 | 未运行 | 未运行"
                lines.append(f"| {group['name']} / {d['log_n']} | {system} | {s['payload_field_elements']} | {s['payload_field_bytes']} | {s['bincode_bytes']} | {memory} |")
    lines += ["", "RSS 每系统/维度一个独立进程的一次冷 proof，在 prove 后、verify 与源检查前读取 VmHWM；包含输入和运行时，"
              "不是 allocator 专属峰值。冷 proof 不混入正式样本，未执行的 RSS 不推测。", ""]
    for name, label in (("local-aggregate-assembly-audit.md", "汇编诊断索引"),
                        ("local-pair-assembly-audit.md", "双输入展开汇编诊断"),
                        ("local-consumer-assembly-audit.md", "消费内核汇编诊断")):
        note = f"diagnostics/{name}"
        if note in validation["artifact_sha256"]:
            lines += [f"[{label}]({note})为单独的源码/指令分析，不等于实际硬件事件或端到端测量。", ""]
    if "diagnostics/local-pair-assembly-audit.md" in validation["artifact_sha256"]:
        lines += ["Pair 的机器码除固定展开外，也改变了 `consume_four` 的内联；"
                  "所测收益属于整个工程候选，不能全部定量归于删除动态分派或跳转。", ""]
    if summary["preflight"] is not None:
        lines += ["[本轮前置状态](diagnostics/preflight.json)单独保留；前置配置不等于执行了 NUMA 或 perf 实验。", ""]
    if summary["source_audit"] is not None:
        audit = summary["source_audit"]
        lines += [f"[冻结源码核对](source-audit.json)：相对 v7 新增或改变 {audit['changed_or_added_file_count']} 个文件；"
                  "Flock、field、A/B 打包与 IP 文件 SHA256 不变。基线 `aggregate_block`、`consume_four`、"
                  "`continuation_registers` 函数体移除空白后相同。该结论限于源码，编译单元改变仍可能影响指令布局或内联。", ""]
    if summary["local_test_count"] is not None:
        lines += [f"[本地测试日志](local-tests.log)：{summary['local_test_count']} 项通过，4 个新增数学/内核测试均在成功列表。"
                  "服务器另核验 native 路径；本地测试数不与服务器重复相加。", ""]
    if "PLAN.md" in validation["artifact_sha256"]:
        lines += ["完整执行计划与保留范围见 [PLAN.md](PLAN.md)。", ""]
    lines += ["## 核验与来源", "",
              f"共核验 {validation['proof_records']} 条 proof 记录，服务器测试通过 {validation['passed_tests']} 项。"
              "实际要求通过的测试名与门槛记录在 build artifact、各组 status/environment；命名测试必须在日志中显示成功。"
              "各组 status=complete、全部命令零退出，日志和下载证据的 SHA256 一致。", "",
              "每条记录均执行 verifier、claims/transcript 一致性及独立完整源输入检查。"
              "输入生成/摘要、challenger 创建、序列化和源检查排除于 prove/verify 计时，但实际执行；verifier 仅为归约重放。"
              "组内与跨组的相同 (N, run) 输入摘要一致，各协议内部 proof 摘要一致；不同协议之间不要求摘要相同。", "",
              f"源 snapshot base revision：`{validation['base_revision']}`；精确文件内容由 [source manifest](source_manifest.json) 和[源包](source.tar.gz)确定，"
              "不将 base revision 当作无修改工作区。各组使用同一 [aggregate build artifact](aggregate_build_artifact.json)。", "",
              f"Executable SHA256：`{validation['executable_sha256']}`。", "",
              f"Mikros BLAKE3：`{validation['mikros_sources_blake3']}`。", "",
              f"Flock BLAKE3：`{validation['flock_sources_blake3']}`。", "",
              f"构建配置 BLAKE3：`{validation['build_configuration_blake3']}`。", "",
              "全部证据/后处理器 SHA256、原始精度统计与两类配对记录见 [SUMMARY.json](SUMMARY.json)。"
              "旧驱动、校验器与历史报告不修改，历史计时不与 v8 合并；新后处理器在冻结源包之外。", ""]
    cross = validation["cross_version_identities"]
    if cross:
        lines += [f"仅就重叠输入/证明身份与 {cross['reference_version']} 核对：{cross['matched_records']} 条 v8 记录匹配，"
                  f"{cross['records_without_reference']} 条无对应参考 run，覆盖 {cross['unique_matched_identities']} 个唯一协议身份。"
                  "这不混合版本源码指纹或计时样本。", ""]
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True, help="downloaded v8-aggregate root")
    parser.add_argument("--groups", default="screening,results", help="comma-separated completed result children")
    args = parser.parse_args()
    root = args.root.resolve()
    require(root.name == "v8-aggregate", "only the v8-aggregate report root is supported")
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
        raise SystemExit(f"aggregate summary failed: {error}") from error

