#!/usr/bin/env python3
"""Validate downloaded native64 experiments before generating the final report."""
from __future__ import annotations

import argparse
from collections import Counter
import csv
import importlib.util
import json
import math
from pathlib import Path
import re
from statistics import mean


MIKROS = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("mikros_shared_summary", MIKROS / "summarize_results.py")
assert SPEC is not None and SPEC.loader is not None
SHARED = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SHARED)
read_report = SHARED.read_report
CPU_SPEC = "0-47,48-55,72-79"
CPUS = list(range(56)) + list(range(72, 80))
HEX64 = re.compile(r"[0-9a-f]{64}\Z")
PHASE_KEYS = {
    "setup_ms", "tensor_aggregate_ms", "certificate_split_ms", "c_pack_project_ms",
    "transcript_challenge_ms", "ab_pack_ms", "b_weight_ms", "inner_product_ms", "terminal_claims_ms",
}
FLOCK_PHASE = re.compile(r"\[zc-timing\] (round1 URM|round2 fused fold|rounds 3\+ tail): ([0-9.]+) ms")
FLOCK_SAMPLE = re.compile(r"(?:warmup n=2\^(\d+) flock\s*:|n=2\^(\d+) run=(\d+) flock\s*:)")
FLOCK_KEYS = {"round1 URM": "round1_urm_ms", "round2 fused fold": "round2_fused_fold_ms", "rounds 3+ tail": "rounds_3_plus_tail_ms"}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def load(path):
    return json.loads(path.read_text(encoding="utf-8"))


def digest(value, label):
    require(isinstance(value, str) and HEX64.fullmatch(value), f"invalid {label}")


def finite(value, label, *, positive=False):
    require(isinstance(value, (int, float)) and not isinstance(value, bool) and math.isfinite(value)
            and (value > 0 if positive else value >= 0), f"invalid {label}: {value!r}")


def samples(report):
    return report["samples"] + report["warmups"]


def expected_orders(systems, runs):
    result = []
    for run in range(1, runs + 1):
        shift = ((run - 1) // 2) % len(systems)
        order = systems[shift:] + systems[:shift]
        result.append(list(reversed(order)) if run % 2 == 0 else order)
    return result


def flock_phases(path, report):
    found, pending = {}, {}
    for line in path.read_text(encoding="utf-8").splitlines():
        match = FLOCK_PHASE.search(line)
        if match:
            key = FLOCK_KEYS[match[1]]
            require(key not in pending, f"{path}: duplicate pending Flock phase")
            pending[key] = float(match[2])
            finite(pending[key], f"{path}: {key}")
        match = FLOCK_SAMPLE.search(line)
        if match:
            key = (int(match[1] or match[2]), int(match[3] or 0))
            require(len(pending) == 3 and key not in found, f"{path}: incomplete/duplicate Flock phase record")
            found[key], pending = pending, {}
    require(not pending, f"{path}: unpaired trailing Flock phases")
    wanted = {(sample["log_n"], sample["run"]) for sample in samples(report) if sample["system"] == "flock"}
    require(set(found) == wanted, f"{path}: Flock phase coverage mismatch")
    for sample in samples(report):
        if sample["system"] == "flock":
            phases = found[sample["log_n"], sample["run"]]
            require(sample["prover_ms"] - sum(phases.values()) >= -0.016,
                    f"{path}: Flock phases exceed total beyond 0.01-ms log rounding")
    return found


def checked_report(path, *, ordinary=False):
    report = read_report(path)
    meta = report["metadata"]
    require(report["schema_version"] == 1, f"{path}: schema")
    require(meta["threads"] == meta["rayon_actual_threads"] == 64, f"{path}: threads")
    systems, logs, runs = meta["systems"], meta["logs"], meta["runs"]
    require(systems and len(set(systems)) == len(systems), f"{path}: systems")
    require(logs and len(set(logs)) == len(logs) and all(isinstance(n, int) and n >= 13 for n in logs), f"{path}: logs")
    require(isinstance(runs, int) and runs > 0, f"{path}: runs")
    digest(meta["mikros_sources_blake3"], f"{path}: source fingerprint")
    require(meta["transcript_hash"] == "blake3" and meta["grinding"].startswith("disabled for both"), f"{path}: protocol settings")
    mode = meta["mode"]
    require(mode in ("paired_warm_timing", "fresh_process_rss_cold_proof"), f"{path}: mode")
    rss = mode == "fresh_process_rss_cold_proof"
    profile = meta["mikros_phase_profiling"]
    require(isinstance(profile, bool), f"{path}: profile flag")
    tuning = meta["tuning_environment"]
    require(tuning in ({}, {"FLOCK_ZC_TIMING": "1"}), f"{path}: unexpected tuning environment")
    flock_profile = tuning.get("FLOCK_ZC_TIMING") == "1"
    if ordinary or rss:
        require(not profile and not flock_profile, f"{path}: instrumented primary/RSS timing")
    require(meta["warmups_per_system_per_dimension"] == (0 if rss else 1), f"{path}: warmup policy")
    if rss:
        require(len(systems) == len(logs) == runs == 1, f"{path}: RSS must be one cold proof")
    for kind, expected_runs in (("samples", range(1, runs + 1)), ("warmups", [] if rss else [0])):
        entries = report[kind]
        expected = {(n, run, system) for n in logs for run in expected_runs for system in systems}
        require(len(entries) == len(expected) and {(s["log_n"], s["run"], s["system"]) for s in entries} == expected,
                f"{path}: {kind} coverage")
        for sample in entries:
            for key in ("input_digest", "proof_digest"):
                digest(sample[key], f"{path}: {key}")
            for key in ("prover_ms", "verifier_ms", "input_claim_check_ms"):
                finite(sample[key], f"{path}: {key}", positive=True)
            n, system = sample["log_n"], sample["system"]
            fields, overhead = (131 + 2 * (n - 6), 32) if system == "flock" else (2190 + 2 * (n - 7), 24)
            require((sample["payload_field_elements"], sample["payload_field_bytes"], sample["bincode_bytes"])
                    == (fields, fields * 16, fields * 16 + overhead), f"{path}: proof shape")
            phases = sample.get("mikros_phases")
            require(bool(phases) == (profile and system != "flock"), f"{path}: phase mode")
            if phases:
                require(set(phases) == PHASE_KEYS, f"{path}: changed Mikros phase fields")
                for key, value in phases.items():
                    finite(value, f"{path}: {key}")
                require(sum(phases.values()) <= sample["prover_ms"] + 0.0001, f"{path}: phase sum exceeds total")
            before, peak = sample["vm_rss_before_prove_kib"], sample["vm_hwm_after_prove_kib"]
            if rss:
                finite(before, f"{path}: RSS before", positive=True)
                finite(peak, f"{path}: RSS peak", positive=True)
                require(peak >= before, f"{path}: RSS peak")
            else:
                require(before is None and peak is None, f"{path}: RSS/ordinary modes mixed")
        for n in logs:
            for run in expected_runs:
                paired = [s for s in entries if (s["log_n"], s["run"]) == (n, run)]
                require(sorted(s["order"] for s in paired) == list(range(len(systems))), f"{path}: order coverage")
                require(len({s["input_digest"] for s in paired}) == 1, f"{path}: input pairing")
                require(len({s["proof_digest"] for s in paired if s["system"] != "flock"}) <= 1, f"{path}: Mikros proof changed")
                expected_order = systems if run == 0 else expected_orders(systems, runs)[run - 1]
                require([s["system"] for s in sorted(paired, key=lambda s: s["order"])] == expected_order,
                        f"{path}: unexpected rotate/reverse order")
    require(len(report["summaries"]) == len(logs) * len(systems)
            and {(s["log_n"], s["system"]) for s in report["summaries"]} == {(n, s) for n in logs for s in systems},
            f"{path}: summary coverage")
    for summary in report["summaries"]:
        require(summary["constraints"] == 1 << summary["log_n"], f"{path}: constraint count")
        selected = next(s for s in report["samples"] if (s["log_n"], s["system"]) == (summary["log_n"], summary["system"]))
        for key in ("payload_field_elements", "payload_field_bytes", "bincode_bytes"):
            require(summary[key] == selected[key], f"{path}: summary {key}")
    if ordinary:
        for n in logs:
            for system in systems:
                counts = Counter(s["order"] for s in report["samples"] if s["log_n"] == n and s["system"] == system)
                require(set(counts) == set(range(len(systems))) and len(set(counts.values())) == 1,
                        f"{path}: unbalanced primary sample positions")
    return report, flock_phases(path.with_suffix(".log"), report) if flock_profile and "flock" in systems else {}


def validate_environment(version_dir, directory):
    environment = load(directory / "server_environment.json")
    manifest = load(version_dir / "source_manifest.json")
    require(environment["source_manifest"] == manifest, f"{directory}: local/server manifest mismatch")
    require(bool(manifest["files_sha256"]), f"{directory}: empty manifest")
    for name, value in manifest["files_sha256"].items():
        digest(value, f"{directory}: manifest {name}")
    digest(environment["executable_sha256"], f"{directory}: executable SHA256")
    require(environment["threads"] == 64 and environment["cpu_spec"] == CPU_SPEC
            and environment["selected_cpus"] == CPUS, f"{directory}: wrong 64-CPU affinity")
    require(set(CPUS) <= set(environment["allowed_cpus"]), f"{directory}: unavailable selected CPUs")
    selected = [tuple(row) for row in environment["topology"] if row[0] in CPUS]
    require(len(selected) == 64 and Counter(row[3] for row in selected) == {0: 32, 1: 32}, f"{directory}: NUMA topology")
    cores = Counter((socket, core) for _, core, socket, _ in selected)
    require(len(cores) == 48 and Counter(cores.values()) == {1: 32, 2: 16}, f"{directory}: physical/SMT topology")
    require(all(len({(socket, core) for _, core, socket, cpu_node in selected if cpu_node == node}) == 24
                for node in (0, 1)), f"{directory}: expected 24 physical cores per NUMA node")
    require(environment["selected_physical_cores"] == 48 and environment["additional_smt_siblings"] == 16,
            f"{directory}: physical/SMT metadata")
    artifact = environment["build_artifact"]
    require(artifact["source_manifest"] == manifest and artifact["executable_sha256"] == environment["executable_sha256"]
            and artifact["toolchain"] == "+nightly-2026-03-21" and artifact["cpu_spec"] == CPU_SPEC,
            f"{directory}: build provenance mismatch")
    validation = artifact["validation_commands"]
    require(len(validation) == 2 and {command["stage"] for command in validation} == {"tests", "build"}
            and all(command["returncode"] == 0 for command in validation), f"{directory}: missing successful tests/build")
    for command in validation:
        require(command["argv"][:3] == ["taskset", "-c", CPU_SPEC]
                and {"--locked", "--offline", "+nightly-2026-03-21"} <= set(command["argv"]),
                f"{directory}: test/build configuration")
    return environment


def option(argv, flag):
    require(argv.count(flag) == 1 and argv.index(flag) + 1 < len(argv), f"missing/duplicate argument {flag}")
    return argv[argv.index(flag) + 1]


def load_group(version_dir, name):
    directory = version_dir / name
    status = load(directory / "status.json")
    require(status["stage"] == "complete", f"{directory}: experiment is not complete")
    environment = validate_environment(version_dir, directory)
    filenames = status["reports"]
    require(filenames and len(set(filenames)) == len(filenames)
            and all(Path(filename).name == filename and filename.endswith(".json") for filename in filenames),
            f"{directory}: invalid status report list")
    require("comparison-64t.json" in filenames and "smoke.json" in filenames, f"{directory}: missing primary/smoke")
    commands = load(directory / "commands.json")
    require(commands and len({command["stage"] for command in commands}) == len(commands)
            and all(command["returncode"] == 0 for command in commands), f"{directory}: command failures/duplicates")
    expected_stages = {Path(filename).stem for filename in filenames}
    if not environment["skip_build"]:
        expected_stages |= {"tests", "build"}
    require({command["stage"] for command in commands} == expected_stages, f"{directory}: command coverage")
    by_stage = {command["stage"]: command for command in commands}
    records = []
    for filename in filenames:
        path = directory / filename
        report, phases = checked_report(path, ordinary=filename == "comparison-64t.json")
        meta = report["metadata"]
        command = by_stage[path.stem]
        argv = command["argv"]
        require(argv[:3] == ["taskset", "-c", CPU_SPEC] and argv[3] == environment["build_artifact"]["executable"],
                f"{path}: executable/CPU-set command mismatch")
        require(option(argv, "--threads") == "64" and option(argv, "--systems").split(",") == meta["systems"]
                and [int(n) for n in option(argv, "--logs").split(",")] == meta["logs"]
                and int(option(argv, "--runs")) == meta["runs"]
                and Path(option(argv, "--output")).name == filename, f"{path}: command/report parameters mismatch")
        require(("--rss" in argv) == (meta["mode"] == "fresh_process_rss_cold_proof")
                and ("--profile" in argv) == meta["mikros_phase_profiling"], f"{path}: command/report mode mismatch")
        require(command["environment_overrides"] == meta["tuning_environment"], f"{path}: timing environment mismatch")
        finite(command["wall_seconds"], f"{path}: process wall time", positive=True)
        records.append({"version": version_dir.name, "group": name, "path": path, "report": report,
                        "flock_phases": phases, "command": command, "environment": environment})
    primary = next(record["report"] for record in records if record["path"].name == "comparison-64t.json")
    meta = primary["metadata"]
    for key, actual in (("threads", 64), ("systems", meta["systems"]), ("logs", meta["logs"]), ("measured_runs", meta["runs"])):
        require(status[key] == actual, f"{directory}: status {key}")
    require(environment["systems"] == meta["systems"] and environment["logs"] == meta["logs"]
            and environment["runs"] == meta["runs"] and environment["main_execution_orders"] == expected_orders(meta["systems"], meta["runs"]),
            f"{directory}: primary/environment configuration mismatch")
    wanted_rss = {(n, system) for n in meta["logs"] for system in meta["systems"]}
    rss = [record for record in records if record["report"]["metadata"]["mode"] == "fresh_process_rss_cold_proof"]
    require(len(rss) == len(wanted_rss) and {(r["report"]["samples"][0]["log_n"], r["report"]["samples"][0]["system"]) for r in rss} == wanted_rss,
            f"{directory}: fresh RSS coverage")
    profile_records = [r for r in records if r["path"].name == "profile-64t.json"]
    require(len(profile_records) == int(environment["profile_enabled"]), f"{directory}: profile report coverage")
    for record in profile_records:
        pm = record["report"]["metadata"]
        require(pm["systems"] == meta["systems"] and pm["logs"] == meta["logs"] and pm["runs"] == status["profile_runs"]
                == environment["profile_runs"] and pm["mikros_phase_profiling"], f"{directory}: profile parameters")
    count = sum(len(samples(record["report"])) for record in records)
    require(status["validated_proof_records"] == environment["validated_proof_records"] == count, f"{directory}: proof record count")
    return records


def check_identities(records, final_version):
    identities, inputs, fingerprints, environments, platforms = {}, {}, {}, {}, {}
    for record in records:
        report, version, path = record["report"], record["version"], record["path"]
        fingerprint = report["metadata"]["mikros_sources_blake3"]
        require(fingerprints.setdefault(version, fingerprint) == fingerprint, f"{version}: multiple source fingerprints within version")
        platform = {key: report["metadata"][key] for key in ("cpu", "os", "arch", "rustc", "compiled_target_features")}
        require(platforms.setdefault(version, platform) == platform, f"{version}: inconsistent CPU/compiler/target features across reports")
        environment = record["environment"]
        if environment is not None:
            require(environment["mikros_sources_blake3"] == fingerprint, f"{path}: environment fingerprint")
            invariant = (environment["executable_sha256"], environment["source_manifest"])
            require(environments.setdefault(version, invariant) == invariant, f"{version}: different executable/manifest across groups")
            required_features = {"gfni", "avx512f", "avx512bw", "vpclmulqdq", "pclmulqdq"}
            if version == final_version:
                required_features |= {"avx512vbmi", "sse4.1"}
            require(required_features <= set(platform["compiled_target_features"]), f"{path}: missing native instruction prerequisites")
        for sample in samples(report):
            base = (sample["log_n"], sample["run"])
            require(inputs.setdefault(base, sample["input_digest"]) == sample["input_digest"], f"{path}: cross-version input mismatch {base}")
            key = (*base, "flock" if sample["system"] == "flock" else "mikros")
            pair = (sample["input_digest"], sample["proof_digest"])
            require(identities.setdefault(key, pair) == pair, f"{path}: cross-version/variant proof mismatch {key}")
    return fingerprints


def validate(run_dir, final_version):
    require(__debug__, "do not use python -O: shared read_report uses assertions")
    require(re.fullmatch(r"v[1-9][0-9]*", final_version), "final-version must look like v3")
    final_dir = run_dir / final_version
    require((final_dir / "results/status.json").is_file() and (final_dir / "ablation/status.json").is_file(),
            "final results and ablation have not both been downloaded; no final artifacts written")
    for group in ("candidate", "pooled-candidate"):
        status_path = final_dir / group / "status.json"
        require(status_path.is_file() and load(status_path).get("stage") == "complete",
                f"final {group} has not completed/downloaded; no final artifacts written")
    for relative in ("baseline/profile-64t.json", "v1/results/status.json", "v1/pilot-n30.json", "v2/results/status.json"):
        require((run_dir / relative).is_file(), f"missing requested development record {relative}; no final artifacts written")
    records = []
    for version in sorted((p for p in run_dir.iterdir() if p.is_dir() and re.fullmatch(r"v[1-9][0-9]*", p.name)),
                          key=lambda p: int(p.name[1:])):
        for directory in sorted(version.iterdir()):
            status_path = directory / "status.json"
            if not directory.is_dir() or not status_path.is_file():
                continue
            status = load(status_path)
            if status.get("stage") == "complete":
                records.extend(load_group(version, directory.name))
            else:
                require(directory.name not in ("results", "ablation"), f"{directory}: required group is not complete")
        # pilot-launch.json is command provenance, not a benchmark report.
        for path in sorted(version.glob("pilot-n[0-9]*.json")):
            report, phases = checked_report(path)
            records.append({"version": version.name, "group": "pilot", "path": path, "report": report,
                            "flock_phases": phases, "command": None, "environment": None})
    baseline_path = run_dir / "baseline/profile-64t.json"
    if baseline_path.is_file():
        report, phases = checked_report(baseline_path)
        records.append({"version": "baseline", "group": "pilot", "path": baseline_path, "report": report,
                        "flock_phases": phases, "command": None, "environment": None})
    require(records, "no reports")
    fingerprints = check_identities(records, final_version)
    final_main = next(r for r in records if r["version"] == final_version and r["group"] == "results"
                      and r["path"].name == "comparison-64t.json")
    require(final_main["report"]["metadata"]["logs"] == [30]
            and final_main["report"]["metadata"]["systems"] in (["legacy", "native", "reuse", "flock"],
                                                                ["legacy", "factored", "factoredreuse", "flock"])
            and final_main["report"]["metadata"]["runs"] == 8, "final primary matrix differs from planned N30, four systems, eight runs")
    for group, systems, profile in (("candidate", ["native", "factored", "flock"], True),
                                    ("pooled-candidate", ["reuse", "factoredreuse", "flock"], False)):
        primary = next(r for r in records if r["version"] == final_version and r["group"] == group
                       and r["path"].name == "comparison-64t.json")
        meta = primary["report"]["metadata"]
        require(meta["systems"] == systems and meta["logs"] == [30] and meta["runs"] == 6
                and primary["environment"]["profile_enabled"] == profile,
                f"{group}: unexpected planned N30 candidate matrix/profile mode")
    candidate_environment = next(r["environment"] for r in records if r["version"] == final_version and r["group"] == "candidate")
    for record in records:
        if record["version"] == final_version and record["group"] in ("candidate", "pooled-candidate") and record["environment"] is not None:
            require(record["environment"]["utc_start"] >= final_main["environment"]["utc_end"],
                    "this run's candidate evaluation must start after the primary experiment completes")
            if record["group"] == "pooled-candidate":
                require(record["environment"]["utc_start"] >= candidate_environment["utc_end"],
                        "pooled-candidate must start after candidate completes")
    return records, fingerprints


def report_kind(record):
    if record["group"] == "pilot":
        meta = record["report"]["metadata"]
        return "pilot_profile" if meta["mikros_phase_profiling"] or meta["tuning_environment"].get("FLOCK_ZC_TIMING") == "1" else "pilot"
    if record["path"].name == "comparison-64t.json":
        return "main" if record["group"] == "results" else "ablation" if record["group"] == "ablation" else "candidate"
    if record["path"].name == "profile-64t.json":
        return "profile"
    return "rss" if record["report"]["metadata"]["mode"] == "fresh_process_rss_cold_proof" else "smoke"


def table_rows(records, run_dir):
    rows = []
    for record in records:
        report, kind = record["report"], report_kind(record)
        by_key = {(s["log_n"], s["system"]): s for s in report["summaries"]}
        warmups = {(s["log_n"], s["system"]): s for s in report["warmups"]}
        for summary in report["summaries"]:
            n, system = summary["log_n"], summary["system"]
            selected = [s for s in report["samples"] if (s["log_n"], s["system"]) == (n, system)]
            warmup = warmups.get((n, system), {})
            baseline = by_key.get((n, "legacy"))
            flock = by_key.get((n, "flock"))
            row = {"version": record["version"], "group": record["group"], "kind": kind,
                   "report": record["path"].relative_to(run_dir).as_posix(), "threads": 64,
                   "log_n": n, "system": system, "runs": summary["runs"],
                   "source_fingerprint": report["metadata"]["mikros_sources_blake3"],
                   "prover_median_ms": summary["prover_ms"]["median"], "prover_min_ms": summary["prover_ms"]["min"],
                   "prover_max_ms": summary["prover_ms"]["max"], "prover_mean_ms": mean(s["prover_ms"] for s in selected),
                   "legacy_over_system": baseline["prover_ms"]["median"] / summary["prover_ms"]["median"] if baseline else "",
                   "flock_over_system": flock["prover_ms"]["median"] / summary["prover_ms"]["median"] if flock else "",
                   "system_over_flock": summary["prover_ms"]["median"] / flock["prover_ms"]["median"] if flock else "",
                   "verifier_median_ms": summary["verifier_ms"]["median"], "bincode_bytes": summary["bincode_bytes"],
                   "payload_field_bytes": summary["payload_field_bytes"], "first_warmup_prover_ms": warmup.get("prover_ms", ""),
                   "source_check_median_ms": summary["input_claim_check_ms"]["median"],
                   "source_check_min_ms": summary["input_claim_check_ms"]["min"], "source_check_max_ms": summary["input_claim_check_ms"]["max"],
                   "rss_before_mib": selected[0]["vm_rss_before_prove_kib"] / 1024 if kind == "rss" else "",
                   "rss_peak_mib": selected[0]["vm_hwm_after_prove_kib"] / 1024 if kind == "rss" else "",
                   "process_wall_seconds": record["command"]["wall_seconds"] if record["command"] else ""}
            phases = []
            for sample in selected:
                if sample.get("mikros_phases"):
                    phases.append(sample["mikros_phases"])
                elif system == "flock" and record["flock_phases"]:
                    phases.append(record["flock_phases"][n, sample["run"]])
            if phases:
                for key in phases[0]:
                    row[f"phase_mean_{key}"] = mean(p[key] for p in phases)
                row["phase_mean_residual_ms"] = mean(s["prover_ms"] - sum(p.values()) for s, p in zip(selected, phases))
            rows.append(row)
    return rows


def fmt(value, digits=3):
    return "—" if value == "" or value is None else f"{value:,.{digits}f}"


def render(run_dir, final_version, records, fingerprints, rows):
    main = [row for row in rows if row["version"] == final_version and row["kind"] == "main"]
    by_system = {row["system"]: row for row in main}
    first_record = next(r for r in records if r["version"] == final_version and r["group"] == "results"
                        and r["path"].name == "comparison-64t.json")
    meta = first_record["report"]["metadata"]
    fresh_name, reuse_name = meta["systems"][1:3]
    fresh, reused, flock = by_system[fresh_name], by_system[reuse_name], by_system["flock"]
    fresh_ratio = fresh["prover_median_ms"] / flock["prover_median_ms"]
    reuse_ratio = reused["prover_median_ms"] / flock["prover_median_ms"]
    gap = abs(reused["prover_median_ms"] - flock["prover_median_ms"])
    relative_gap = gap / flock["prover_median_ms"]
    spans = [row["prover_max_ms"] - row["prover_min_ms"] for row in (reused, flock)]
    overlap = max(reused["prover_min_ms"], flock["prover_min_ms"]) <= min(reused["prover_max_ms"], flock["prover_max_ms"])
    # A descriptive near-tie label, not a significance test or an assertion
    # about future measurements. Otherwise report the ratio/ranges neutrally.
    near_tie = relative_gap <= 0.05 and overlap and min(spans) > 0 and gap <= 0.1 * min(spans)
    range_text = (f"{reuse_name} 的样本范围为 {fmt(reused['prover_min_ms'],1)}–{fmt(reused['prover_max_ms'],1)} ms，"
                  f"Flock 为 {fmt(flock['prover_min_ms'],1)}–{fmt(flock['prover_max_ms'],1)} ms；"
                  f"两者中位数差为 {gap:.3f} ms（约 {100 * relative_gap:.2f}%）。")
    interpretation = (f"主组八次观测中，{reuse_name} 与 Flock 基本持平。" + range_text
                      + "中位数差远小于各自观测范围的宽度，不能据此称为稳定胜出。"
                      if near_tie else range_text + "这些中位数和范围描述本组样本，不自动构成统计显著性证据。")
    pooled = {row["system"]: row for row in rows if row["version"] == final_version
              and row["group"] == "pooled-candidate" and row["kind"] == "candidate"}
    pooled_flock = pooled["flock"]["prover_median_ms"]
    pooled_reuse_ratio = pooled["reuse"]["prover_median_ms"] / pooled_flock
    pooled_factored_ratio = pooled["factoredreuse"]["prover_median_ms"] / pooled_flock
    pooled_text = (f"随后独立进行的六次 pooled-candidate 补测中，reuse 中位数为 {fmt(pooled['reuse']['prover_median_ms'])} ms、"
                   f"factoredreuse 为 {fmt(pooled['factoredreuse']['prover_median_ms'])} ms，"
                   f"同组 Flock 为 {fmt(pooled_flock)} ms；相对该组 Flock 的耗时比分别为 {pooled_reuse_ratio:.4f} 和 {pooled_factored_ratio:.4f}。")
    if min(pooled_reuse_ratio, pooled_factored_ratio) > 1:
        pooled_text += "两种复用路径在这组补测中的中位数均高于 Flock；主组的接近结果不能概括成稳定追平或超越 Flock。"
    else:
        pooled_text += "这组数据与主组分别保留，不混合样本或借用另一组 Flock 计算比值；不同组的观察不能自动外推为稳定优势。"
    resources = []
    related = [("FINDINGS.md", "结果与瓶颈解释"), ("../../NATIVE_OPTIMIZATION_NOTES.md", "实现说明")]
    links = [f"[{label}]({relative})" for relative, label in related if (run_dir / relative).is_file()]
    if (run_dir / "native64_comparison.png").is_file():
        resources += ["![64 线程原始计时样本与阶段对比](native64_comparison.png)", ""]
    if (run_dir / "native64_comparison.svg").is_file():
        links.append("[矢量图](native64_comparison.svg)")
    if links:
        resources += [" · ".join(links), ""]
    total = sum(len(samples(record["report"])) for record in records)
    doc = ["# Binary Mikros：64 线程工程优化与 Flock", "",
           f"{final_version} 同一 binary、N=2^30、64 线程下，warm prover 中位数为：legacy {fmt(by_system['legacy']['prover_median_ms'])} ms、{fresh_name} {fmt(fresh['prover_median_ms'])} ms、{reuse_name} {fmt(reused['prover_median_ms'])} ms、Flock {fmt(flock['prover_median_ms'])} ms。{fresh_name}/Flock 耗时比为 {fresh_ratio:.2f}，{reuse_name}/Flock 为 {reuse_ratio:.3f}；相对 legacy 的工程加速分别为 {fresh['legacy_over_system']:.2f}× 和 {reused['legacy_over_system']:.2f}×。", "",
           interpretation, "",
           pooled_text, "",
           f"本报告由原始 JSON/log 重新校验生成，共覆盖 {total} 份 benchmark 已成功执行 verifier、transcript 和全部源输入声明核验的证明记录，含开发阶段、预热、诊断及 RSS。汇总器核对记录与摘要，不重新执行密码学验证。", "",
           *resources,
           "## 环境与测量边界", "",
           f"CPU：{meta['cpu']}；{meta['os']} {meta['arch']}；Rust：`{meta['rustc']}`；native target features：`{', '.join(meta['compiled_target_features'])}`。", "",
           "固定 CPU 集合 `0-47,48-55,72-79`，两个 NUMA 节点各 32 个逻辑 CPU，总计 48 个物理核和 16 个额外 SMT sibling。tests/build/benchmark 的 taskset 命令、全局 Rayon 与 Flock 的 64 线程配置已核对。共享服务器未独占 CPU；不显式绑定或交错 NUMA 内存，采用默认 first-touch。cgroup、load average 和命令起止记录保留在各版本原始环境中。", "",
           "Mikros 各工程变体使用相同协议、参数和逐字节 proof digest；不同版本源码 fingerprint 分开保存。Flock 使用原生产 degree-two zerocheck，协议和参数不同。双方使用 F128、BLAKE3、关闭 grinding；范围是 PIOP reduction，不含 packed-opening reduction、PCS commitment/opening，不声称相同的 128-bit soundness。", "",
           "主计时包含每份证明的分配、动态构表、witness 变换、权重、折叠、transcript 和声明构造；不扣除 setup。native/factored 在同一个已预热的进程中每次新建 scratch，此处的 fresh allocation 不等于新进程冷启动。输入生成/digest、challenger 初始化、序列化与全部源输入 oracle 在 prove/verify 计时外，但仍逐份执行。reuse/factoredreuse 各自保留独立 scratch 容量，所有挑战相关内容重算；Flock 保留其生产 pool 复用。factored/factoredreuse 使用分层权重分支，native/reuse 使用完整 row-weight 路径；正式表明确实际测量的名称。", "",
           "Flock 的 [scratch.rs](../../../crates/flock-core/src/scratch.rs#L37) 使用进程全局 F128 缓冲池，take 优先取得容量足够的旧缓冲；[zerocheck.rs](../../../crates/flock-core/src/zerocheck.rs#L1239) 在结束时用 give_f128 归还四个尾部缓冲，供后续证明复用。因此，在保留容量策略上，reuse 是与生产 Flock 更对称的对照；native 同时展示每次新建并销毁临时 scratch 的 API 成本。两种 Mikros 模式每次都重建挑战相关内容、产生相同消息，并在 prove 之后执行相同的 verifier、transcript 与全部源输入声明检查。", "",
           "每系统一次预热、八次正式样本。轮换和反序排列使每系统在四个位置各出现两次。保留全部 min/max，不删除慢样本；八次观察不构成统计显著性保证。64 线程跨 NUMA 并使用部分 SMT，不能把它与旧单 NUMA 的 24 线程结果之比解释为只改变线程数的扩展效率。", "",
           "## 正式 N30 warm prover", "", "单位 ms；legacy/本项为工程加速，Flock/本项大于 1 表示本项更快。", "",
           "| 系统 | 中位数 | min–max | legacy/本项 | Flock/本项 | 首次预热 prove |", "| --- | ---: | ---: | ---: | ---: | ---: |"]
    for row in main:
        doc.append(f"| {row['system']} | {fmt(row['prover_median_ms'])} | {fmt(row['prover_min_ms'])}–{fmt(row['prover_max_ms'])} | {fmt(row['legacy_over_system'],2)}× | {fmt(row['flock_over_system'],2)} | {fmt(row['first_warmup_prover_ms'])} |")
    doc += ["", "首次预热是该变体在配对进程内的第一次证明，保留首次构表/分配成本；进程和其他变体可能已经预热，因此不能称为独立冷启动。下表的 RSS 过程才是全新进程中的单次冷证明。", "",
            "## Verifier、通信、独立冷进程 RSS 和外部检查", "",
            "| 系统 | verify 中位数 ms | bincode bytes | 冷 prove ms | 冷前 RSS MiB | 冷峰值 RSS MiB | 源检查中位数 ms | 源检查 min–max ms |", "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"]
    for row in main:
        cold = next(r for r in rows if r['version'] == final_version and r['group'] == 'results' and r['kind'] == 'rss'
                    and r['log_n'] == row['log_n'] and r['system'] == row['system'])
        doc.append(f"| {row['system']} | {fmt(row['verifier_median_ms'],4)} | {row['bincode_bytes']:,} | {fmt(cold['prover_median_ms'])} | {fmt(cold['rss_before_mib'],2)} | {fmt(cold['rss_peak_mib'],2)} | {fmt(row['source_check_median_ms'])} | {fmt(row['source_check_min_ms'])}–{fmt(row['source_check_max_ms'])} |")
    doc += ["", "RSS 的 VmHWM 在 prove 返回后、verify/源输入核验前读取，包含输入、runtime、线程、表和工作区，不是纯算法工作区。独立 RSS 冷时间只作补充，不混入 warm 中位数。冷过程中的 reuse 同样从空容量开始，但在 prove 返回后继续持有 scratch；native 在计时结束前销毁其临时 scratch，Flock 则已将缓冲归还持久池。因此不同冷 API 的容量释放边界仍有所区别。所有冷过程的 source-check 时间及完整进程 wall time 也保存在 summary.csv。", "",
            "## 独立阶段诊断", "", "以下采用独立 profile 的正式样本均值以保留可加性，不混入无探针主表。Mikros 的 A/B binding 已含 B 权重；fused/native/reuse/factored/factoredreuse 还把 IP 首轮消息并入 binding，因此各阶段的职责有所变化。分层权重也会改变 setup 与使用权重阶段的职责。聚合阶段只表示整个 tensor aggregate，不能直接解释成某条内部指令成本。", "",
            "| 组别 | N | 系统 | 样本数 | 总均值 ms | setup | tensor | C 投影 | A/B+权重 | IP | 其他/剩余 |", "| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |"]
    for row in rows:
        if row['version'] != final_version or row['kind'] != 'profile' or row['system'] == 'flock':
            continue
        values = [row['phase_mean_setup_ms'], row['phase_mean_tensor_aggregate_ms'], row['phase_mean_c_pack_project_ms'],
                  row['phase_mean_ab_pack_ms'] + row['phase_mean_b_weight_ms'], row['phase_mean_inner_product_ms']]
        doc.append(f"| {row['group']} | 2^{row['log_n']} | {row['system']} | {row['runs']} | {fmt(row['prover_mean_ms'])} | "
                   + " | ".join(fmt(value) for value in values + [row['prover_mean_ms'] - sum(values)]) + " |")
    flock_rows = [r for r in rows if r['version'] == final_version and r['kind'] == 'profile' and r['system'] == 'flock']
    if flock_rows:
        doc += ["", "| 组别 | N | Flock 总均值 ms | round1 URM | round2 fused fold | rounds 3+ | 剩余 |", "| --- | ---: | ---: | ---: | ---: | ---: | ---: |"]
        for row in flock_rows:
            keys = ('prover_mean_ms', 'phase_mean_round1_urm_ms', 'phase_mean_round2_fused_fold_ms', 'phase_mean_rounds_3_plus_tail_ms', 'phase_mean_residual_ms')
            doc.append(f"| {row['group']} | 2^{row['log_n']} | " + " | ".join(fmt(row[key]) for key in keys) + " |")
        doc += ["", "Flock 阶段从服务器本地原始 log 与对应 N/run 逐条连接，打印精度为 0.01 ms，最多允许 0.016 ms 的合计舍入误差。剩余项包含未覆盖工作与计时/日志开销；profile 的三次样本不保证执行位置均衡。"]
    doc += ["", "## 同版本逐项消融", "",
            "legacy→stream→simd→wide→fused→native 为累加工程路径：流式 tensor、GFNI 聚合、四路 VPCLMUL IP、packing/首轮消息与 C 投影融合、row-weight 构建。simdv1 是早期 GFNI 候选；factored 是分层权重的独立分支，不应擅自纳入上述累加链。native→reuse、factored→factoredreuse 分别只改变各自 scratch 容量生命周期。具体路径以本版本 benchmark 的 native_ablation_variants 元数据及源码为准，不能从对 legacy 的总比值推断单项净贡献。", "",
            "| N | 系统 | 正式样本 | prove 中位数 ms | min–max ms | legacy/本项 | verify ms | bytes |", "| ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: |"]
    for row in rows:
        if row['version'] == final_version and row['kind'] == 'ablation':
            doc.append(f"| 2^{row['log_n']} | {row['system']} | {row['runs']} | {fmt(row['prover_median_ms'])} | {fmt(row['prover_min_ms'])}–{fmt(row['prover_max_ms'])} | {fmt(row['legacy_over_system'],2)}× | {fmt(row['verifier_median_ms'],4)} | {row['bincode_bytes']:,} |")
    candidate_rows = [row for row in rows if row['version'] == final_version and row['kind'] == 'candidate']
    if candidate_rows:
        doc += ["", "### 独立候选比较", "",
                "本轮 candidate 在主测量完成后串行执行，补充评估 native/factored 的每次分配路径；随后 pooled-candidate 比较 reuse/factoredreuse 的保留容量路径。两组均包含各自 Flock 对照，不用于预选本轮正式路径，也不将候选样本并入 results 中位数。各组只使用本组的 legacy/Flock 计算比值；不存在的基线留空。pooled-candidate 不启用 phase profile。有限样本及共享服务器环境仍限制结果的精度，不构成统计显著性保证。", "",
                "| 组别 | N | 系统 | 样本数 | prove 中位数 ms | min–max ms | legacy/本项 | Flock/本项 |", "| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: |"]
        for row in candidate_rows:
            legacy_ratio = "" if row['legacy_over_system'] == "" else fmt(row['legacy_over_system'], 2) + "×"
            flock_ratio = "" if row['flock_over_system'] == "" else fmt(row['flock_over_system'], 2)
            doc.append(f"| {row['group']} | 2^{row['log_n']} | {row['system']} | {row['runs']} | {fmt(row['prover_median_ms'])} | {fmt(row['prover_min_ms'])}–{fmt(row['prover_max_ms'])} | {legacy_ratio} | {flock_ratio} |")
    doc += ["", "消融使用自己的无探针均衡主样本。小尺寸收益不能直接外推到 N30；所有变体的首次预热、独立 RSS、source-check 与 profile 均值在 summary.csv 中单列。", "",
            "## 开发过程记录", "",
            "以下各版本和先导运行具有各自 source fingerprint，不合并样本、不替换正式结果，也不据跨版本的非配对时间推断单项收益。独立 baseline/pilot 若没有 driver status/commands/environment，只验证其 JSON、原始阶段日志和摘要，不能声称本地已复核其 CPU affinity 或完整构建执行凭据。", "",
            "| 版本 | 文件 | 模式 | N | 系统 | 正式样本 | prove 中位数 ms | min–max ms |", "| --- | --- | --- | ---: | --- | ---: | ---: | ---: |"]
    for row in rows:
        if row['version'] != final_version and row['kind'] in ('main', 'ablation', 'candidate', 'profile', 'pilot', 'pilot_profile'):
            doc.append(f"| {row['version']} | [{row['report']}]({row['report']}) | {row['kind']} | 2^{row['log_n']} | {row['system']} | {row['runs']} | {fmt(row['prover_median_ms'])} | {fmt(row['prover_min_ms'])}–{fmt(row['prover_max_ms'])} |")
    doc += ["", "## 来源与验收", "",
            "| 版本 | 已核验记录数（含预热/RSS） | benchmark source fingerprint |", "| --- | ---: | --- |"]
    for version, fingerprint in fingerprints.items():
        count = sum(len(samples(r['report'])) for r in records if r['version'] == version)
        doc.append(f"| {version} | {count} | `{fingerprint}` |")
    incomplete = []
    for version in run_dir.iterdir():
        if version.is_dir() and re.fullmatch(r"v[1-9][0-9]*", version.name):
            for directory in version.iterdir():
                path = directory / "status.json"
                if directory.is_dir() and path.is_file() and load(path).get("stage") != "complete":
                    incomplete.append(f"{version.name}/{directory.name}（{load(path).get('stage', 'unknown')}）")
    if incomplete:
        doc += ["", "未纳入的未完成辅助组：" + "、".join(incomplete) + "。"]
    doc += ["", "各 driver 目录均核对 status=complete、报告清单、命令全部零退出、原生 locked/offline tests/build 来源、样本/预热覆盖、均衡主顺序、summary 中位数/min/max、消息尺寸、phase 字段及加和、独立 RSS 模式、各版本本地 source_manifest 与 server_environment 完全一致。跨所有版本核对同 N/run 输入摘要，以及全部 Mikros 内核和 Flock 各自的 proof digest。", "",
            "这些摘要检查证明已保存的逐字节一致性，不代替完整源码安全审计或 PCS 验证。原始数据和日志与各版本目录一并保留；[summary.csv](summary.csv) 包含全部表格模式和逐阶段均值。", ""]
    return "\n".join(doc)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--final-version", default="v3")
    args = parser.parse_args()
    run_dir = args.run_dir.resolve()
    records, fingerprints = validate(run_dir, args.final_version)
    rows = table_rows(records, run_dir)
    document = render(run_dir, args.final_version, records, fingerprints, rows)
    # Do not write either artifact until every report and invariant has passed.
    fields = list(dict.fromkeys(key for row in rows for key in row))
    with (run_dir / "summary.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=fields)
        writer.writeheader()
        writer.writerows(rows)
    (run_dir / "REPORT.md").write_text(document, encoding="utf-8")
    print(f"Validated {sum(len(samples(r['report'])) for r in records)} proof records across {len(fingerprints)} source groups; wrote REPORT.md and summary.csv")


if __name__ == "__main__":
    try:
        main()
    except (AssertionError, KeyError, ValueError, OSError, StopIteration) as error:
        raise SystemExit(f"native64 summary refused: {error}") from error
