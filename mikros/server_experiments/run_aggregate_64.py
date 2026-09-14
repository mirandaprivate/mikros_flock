#!/usr/bin/env python3
"""Run same-binary first-aggregation ablations on the authorized server.

This independent driver runs locally on the destination and never uses SSH.
Every run needs a fresh results child; --skip-build reuses a hash-verified
aggregate_build_artifact.json in the same frozen source snapshot. Profile and
fresh-process RSS diagnostics run only when explicitly requested.
"""
from __future__ import annotations

import argparse
from collections import Counter
import json
import math
import os
from pathlib import Path
import re
import shutil
import statistics
import subprocess
import time

import run_server_64 as base


DEFAULT_SYSTEMS = "basisreuse,splitreuse,halfreuse,pairreuse,affinereuse,jointreuse,flock128"
DEFAULT_SMOKE_SYSTEMS = "reference,lookup,basis,split,half,pair,affine,joint,flock128"
DEFAULT_REQUIRED_TESTS = ",".join((
    "aggregate_eight_consumers_match_scalar_all_lanes_and_partial_tiles",
    "aggregate_eight_schedules_match_reference_across_row_boundaries",
    "paired_register_continuation_matches_single_and_scalar_all_lanes",
    "affine_h_matches_bitwise_weights_all_subsets_tails_and_accumulation",
))
DEFAULT_MIN_TESTS = 80
BENCH = "univariate_vs_flock"
REQUIRED_FEATURES = {"gfni", "avx512f", "avx512bw", "vpclmulqdq", "pclmulqdq", "sse4.1"}
VALIDATION_STAGES = {"rustc-version", "rustc-cfg", "tests", "build"}
require = base.require


def check_report(path, systems, logs, runs, *, rss=False, profile=False):
    """Validate this benchmark's schema, including its RSS run index of zero."""
    report = base.load(path)
    meta = report["metadata"]
    require(report["schema_version"] == 1, f"{path.name}: unsupported schema")
    for key, expected in {
        "protocol": "mikros-univariate-gf256-v1", "latex": "latex/4-block.tex",
        "kappa": 128, "intermediate_field_bits": 8, "extension_coordinates": 16,
        "flock_skip_rounds": 7, "threads": base.THREADS,
        "rayon_actual_threads": base.THREADS,
        "systems": systems, "logs": logs, "runs": runs, "profile": profile,
        "mode": "fresh_process_rss_cold_proof" if rss else "paired_profile_diagnostic" if profile else "paired_warm_timing",
        "position_balanced": runs % len(systems) == 0,
        "os": "linux", "arch": "x86_64",
    }.items():
        require(meta[key] == expected, f"{path.name}: metadata {key} mismatch")
    require(meta["grinding"] == "disabled; no equal-security claim", f"{path.name}: security setting")
    require("no extra column-to-tensor aggregation or PCS" in meta["scope"], f"{path.name}: protocol boundary")
    require(meta["native_backend"] == "gfni-avx512", f"{path.name}: required native backend unavailable")
    if any(s.startswith(("coeffclmul", "coeffnibble")) for s in systems):
        require(meta.get("carryless_backend") == "vpclmul-avx512", f"{path.name}: carryless hardware path not selected")
    require(isinstance(meta["rustc"], str) and "1.96.0-nightly" in meta["rustc"], f"{path.name}: rustc metadata")
    if "flock_actual_threads" in meta:
        require(meta["flock_actual_threads"] == base.THREADS, f"{path.name}: Flock pool size")
    require(REQUIRED_FEATURES <= set(meta["compiled_target_features"]), f"{path.name}: native build features")
    require(isinstance(meta["cpu"], str) and meta["cpu"], f"{path.name}: missing CPU metadata")
    require("select the first aggregation only" in meta["kernel_scope"]
            and "native/reuse preserve the v5 kernel" in meta["kernel_scope"], f"{path.name}: unclear kernel scope")
    if any(system.startswith("registerpacked") for system in systems):
        require("registerpacked/registerpackedreuse" in meta["kernel_scope"]
                and "also defer IP message horizontal folds across packing chunks" in meta["kernel_scope"],
                f"{path.name}: unclear additional packing optimization scope")
    require("independent per-system allocation capacity" in meta["scratch"], f"{path.name}: scratch isolation metadata")
    for name in ("mikros_sources_blake3", "flock_sources_blake3", "build_configuration_blake3"):
        require(base.HEX64.fullmatch(meta[name]), f"{path.name}: invalid {name}")
    require(len(report["samples"]) == len(logs) * len(systems) * runs, f"{path.name}: sample count")
    require(len(report["warmups"]) == (0 if rss else len(logs) * len(systems)), f"{path.name}: warmup count")
    require(all(s["run"] == 0 for s in report["warmups"]), f"{path.name}: misplaced warmup")
    require(all(s["run"] == 0 if rss else 1 <= s["run"] <= runs for s in report["samples"]),
            f"{path.name}: invalid measured run index")
    seen = set()
    samples = report["samples"] + report["warmups"]
    for sample in samples:
        key = sample["log_n"], sample["run"], sample["system"]
        require(key not in seen, f"{path.name}: duplicate sample {key}")
        seen.add(key)
        for name in ("prover_ms", "verifier_ms", "input_claim_check_ms"):
            require(math.isfinite(sample[name]) and sample[name] > 0, f"{path.name}: invalid {name}")
        for name in ("input_digest", "proof_digest"):
            require(base.HEX64.fullmatch(sample[name]), f"{path.name}: invalid {name}")
        for name in ("verifier_passed", "transcript_matched", "source_claims_verified"):
            require(sample[name] is True, f"{path.name}: failed {name}")
        if "source_check_reuse" in meta:
            require(type(meta["source_check_reuse"]) is bool, f"{path.name}: source cache policy type")
            require(sample.get("source_check_method") in {"full_oracle", "exact_claim_cache"},
                    f"{path.name}: unknown source check method")
            require(isinstance(sample.get("source_check_anchor"), str), f"{path.name}: missing source check anchor")
        fields = (259 if sample["system"] == "flock128" else 258) + 2 * (sample["log_n"] - 7)
        overhead = 32 if sample["system"] == "flock128" else 24
        require((sample["payload_field_elements"], sample["payload_field_bytes"], sample["bincode_bytes"])
                == (fields, fields * 16, fields * 16 + overhead), f"{path.name}: changed proof shape")
        before, peak = sample["vm_rss_before_prove_kib"], sample["vm_hwm_after_prove_kib"]
        if rss:
            require(isinstance(before, int) and isinstance(peak, int) and peak >= before > 0,
                    f"{path.name}: invalid RSS")
        else:
            require(before is None and peak is None, f"{path.name}: RSS mixed into ordinary timing")
        phases = sample["mikros_phases"]
        require(bool(phases) == (profile and sample["system"] != "flock128"), f"{path.name}: profiling mode")
        if phases:
            require(all(math.isfinite(value) and value >= 0 for value in phases.values())
                    and sum(phases.values()) <= sample["prover_ms"] + 0.001, f"{path.name}: invalid phases")
    run_indices = [0] if rss else list(range(runs + 1))
    expected = {(n, run, system) for n in logs for run in run_indices for system in systems}
    require(seen == expected, f"{path.name}: incomplete sample coverage")
    for n in logs:
        for run in run_indices:
            group = [s for s in samples if (s["log_n"], s["run"]) == (n, run)]
            require(len({s["input_digest"] for s in group}) == 1, f"{path.name}: inputs are not paired")
            expected_order = [systems[(run + i) % len(systems)] for i in range(len(systems))]
            require(sorted(s["order"] for s in group) == list(range(len(systems)))
                    and [s["system"] for s in sorted(group, key=lambda s: s["order"])] == expected_order,
                    f"{path.name}: unexpected execution order")
            require(len({s["proof_digest"] for s in group if s["system"] != "flock128"}) <= 1,
                    f"{path.name}: Mikros kernels changed proof bytes")
            if "source_check_reuse" in meta:
                anchor = None
                for sample in sorted(group, key=lambda s: s["order"]):
                    if sample["system"] == "flock128" or not meta["source_check_reuse"] or anchor is None:
                        require(sample["source_check_method"] == "full_oracle"
                                and sample["source_check_anchor"] == sample["system"],
                                f"{path.name}: missing independent source anchor")
                        if sample["system"] != "flock128":
                            anchor = sample["system"]
                    else:
                        require(sample["source_check_method"] == "exact_claim_cache"
                                and sample["source_check_anchor"] == anchor,
                                f"{path.name}: source check cache crossed inputs or lacked an earlier full oracle")
    summaries = report["summaries"]
    require(len(summaries) == len(logs) * len(systems)
            and {(s["log_n"], s["system"]) for s in summaries} == {(n, s) for n in logs for s in systems},
            f"{path.name}: summary coverage")
    for summary in summaries:
        selected = [s for s in report["samples"]
                    if (s["log_n"], s["system"]) == (summary["log_n"], summary["system"])]
        require(summary["runs"] == len(selected) == runs, f"{path.name}: summary run count")
        for name in ("payload_field_bytes", "bincode_bytes"):
            require(summary[name] == selected[0][name], f"{path.name}: summary proof size")
        for name in ("prover_ms", "verifier_ms", "input_claim_check_ms"):
            values = [s[name] for s in selected]
            for stat, value in (("median", statistics.median(values)), ("min", min(values)), ("max", max(values))):
                require(math.isclose(summary[name][stat], value, rel_tol=1e-12), f"{path.name}: incorrect {name}/{stat}")
    return report


def verified_sources(source):
    manifest = base.verified_manifest(source)
    required = {"Cargo.toml", "Cargo.lock", ".cargo/config.toml", "mikros/Cargo.toml", "mikros/build.rs",
                "mikros/server_experiments/run_aggregate_64.py", "mikros/server_experiments/run_register_64.py",
                "mikros/server_experiments/run_hardware_64.py",
                "mikros/server_experiments/run_univariate_64.py",
                "mikros/server_experiments/run_server_64.py"}
    # A newly added module must not compile outside the upload's source inventory.
    for directory in [source / "crates", source / "mikros/src", source / "mikros/tests", source / "mikros/benches"]:
        required.update(path.relative_to(source).as_posix() for path in directory.rglob("*.rs"))
    require(required <= set(manifest["files_sha256"]), "source inventory omits required sources/build files/drivers")
    return manifest


def parse_systems(value):
    systems = [name.strip() for name in value.split(",")]
    require(systems and len(set(systems)) == len(systems)
            and all(re.fullmatch(r"[a-z][a-z0-9_]*", name) for name in systems),
            "systems must be distinct lowercase benchmark identifiers")
    return systems


def required_tests_passed(output, names):
    passed = set(re.findall(r"^test ([A-Za-z0-9_:]+) \.\.\. ok$", output, re.MULTILINE))
    return all(any(actual == name or actual.endswith("::" + name) for actual in passed) for name in names)


def parse_args():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True, help="new run root containing source/source_manifest.json")
    parser.add_argument("--systems", default=DEFAULT_SYSTEMS, help="comma-separated benchmark systems")
    parser.add_argument("--smoke-systems", default=DEFAULT_SMOKE_SYSTEMS, help="comma-separated systems for the N14 proof-equivalence smoke")
    parser.add_argument("--required-tests", default=DEFAULT_REQUIRED_TESTS, help="comma-separated test names that must pass, short or fully qualified")
    parser.add_argument("--min-tests", type=int, default=DEFAULT_MIN_TESTS, help="minimum total passing tests in the native server build")
    parser.add_argument("--logs", default="26", help="comma-separated log dimensions, 14 through 30")
    parser.add_argument("--runs", type=int, default=14, help="positive multiple of the system count for position balance")
    parser.add_argument("--profile", action="store_true", help="run independent diagnostics with runs equal to the selected system count")
    parser.add_argument("--rss", action="store_true", help="run one cold proof per selected system/dimension in a fresh process")
    parser.add_argument("--reuse-source-checks", action="store_true", help="reuse only exactly equal verified Mikros claims for the same immutable inputs; every proof still verified")
    parser.add_argument("--results-name", default="results", help="fresh output child; use another name for a debug run")
    parser.add_argument("--skip-build", action="store_true", help="reuse the hash-verified tested aggregate build in this source snapshot")
    args = parser.parse_args()
    args.systems = parse_systems(args.systems)
    args.smoke_systems = parse_systems(args.smoke_systems)
    args.required_tests = [value.strip() for value in args.required_tests.split(",")]
    require(args.required_tests and len(set(args.required_tests)) == len(args.required_tests)
            and all(re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*", name) for name in args.required_tests),
            "set actual new aggregate test names with --required-tests before running")
    require(args.min_tests >= DEFAULT_MIN_TESTS, f"min-tests must be at least {DEFAULT_MIN_TESTS}")
    args.logs = [int(value) for value in args.logs.split(",")]
    require(args.logs and len(set(args.logs)) == len(args.logs) and all(14 <= n <= 30 for n in args.logs),
            "logs must be distinct dimensions from 14 through 30")
    require(args.runs > 0 and args.runs % len(args.systems) == 0,
            "main runs must be positive and divisible by the number of systems")
    require(re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9_.-]*", args.results_name), "invalid results-name")
    return args


def experiment(args):
    root = args.root.resolve()
    source = (root / "source").resolve()
    results = root / args.results_name
    require(source.is_dir() and source.is_relative_to(root), "source must be inside the experiment root")
    require(root.name not in {"v1", "v2", "v3", "v4", "v5-univariate", "v6-hardware", "v7-register"}, "historical experiment roots must remain unchanged")
    require(not results.exists() or (results.is_dir() and not any(results.iterdir())), "results directory must be fresh")
    results.mkdir(exist_ok=True)
    commands = []

    def status(stage, **extra):
        record = {"utc": base.utc(), "stage": stage, **extra}
        base.save(results / "status.json", record)
        print(json.dumps(record), flush=True)

    status("preflight")
    try:
        manifest = verified_sources(source)
        if (root / "source_manifest.json").is_file():
            require(base.load(root / "source_manifest.json") == manifest, "root/source manifests differ")
        available = base.memory()
        require(available["MemAvailable"] > 8 * 1024 * 1024, "need at least 8 GiB available memory")
        require(shutil.disk_usage(root).free > (1 if args.skip_build else 4) * 1024**3, "insufficient free disk")
        environment = os.environ.copy()
        environment["LC_ALL"] = "C"
        environment["PATH"] = str(Path.home() / ".cargo/bin") + os.pathsep + environment.get("PATH", "")
        for key in base.CLEARED_ENV:
            environment.pop(key, None)
        environment["CARGO_BUILD_JOBS"] = str(base.THREADS)
        environment["CARGO_TARGET_DIR"] = str(source / "target")
        topology = [tuple(map(int, line.split(","))) for line in
                    subprocess.check_output(["lscpu", "-p=CPU,CORE,SOCKET,NODE"], text=True, env=environment).splitlines()
                    if line and not line.startswith("#")]
        selected = [row for row in topology if row[0] in base.CPUS]
        require(set(base.CPUS) <= os.sched_getaffinity(0), "the planned 64 CPUs are not all available")
        require(len(selected) == base.THREADS and Counter(row[3] for row in selected) == {0: 32, 1: 32},
                "expected 32 selected logical CPUs per NUMA node")
        cores = Counter((socket, core) for _, core, socket, _ in selected)
        require(len(cores) == 48 and Counter(cores.values()) == {1: 32, 2: 16}, "unexpected physical-core/SMT layout")
        require(all(len({(socket, core) for _, core, socket, cpu_node in selected if cpu_node == node}) == 24
                    for node in (0, 1)), "expected 24 physical cores per NUMA node")
        inventory = {
            "utc_start": base.utc(), "hostname": os.uname().nodename, "uname": list(os.uname()),
            "lscpu": json.loads(subprocess.check_output(["lscpu", "-J"], env=environment)),
            "topology": topology, "allowed_cpus": sorted(os.sched_getaffinity(0)),
            "cpu_spec": base.CPU_SPEC, "selected_cpus": base.CPUS, "threads": base.THREADS,
            "selected_physical_cores": 48, "additional_smt_siblings": 16,
            "memory_start_kib": available, "loadavg_start": os.getloadavg(), "source_manifest": manifest,
            "toolchain": base.TOOLCHAIN, "cleared_environment_keys": base.CLEARED_ENV,
            "environment_overrides": {"LC_ALL": "C", "CARGO_BUILD_JOBS": str(base.THREADS),
                                      "CARGO_TARGET_DIR": str(source / "target")},
            "build_rustflags": "repository .cargo/config.toml: -C target-cpu=native",
            "systems": args.systems, "smoke_systems": args.smoke_systems, "logs": args.logs, "runs": args.runs,
            "required_test_names": args.required_tests, "minimum_tests": args.min_tests,
            "main_execution_orders": [[args.systems[(run + i) % len(args.systems)] for i in range(len(args.systems))]
                                      for run in range(1, args.runs + 1)],
            "profile_runs": len(args.systems) if args.profile else 0, "rss_requested": args.rss, "skip_build": args.skip_build,
            "source_check_reuse": args.reuse_source_checks,
            "numa_policy": "Linux default first-touch, no explicit memory binding/interleave; two NUMA nodes",
            "thread_policy": "Same 64-CPU taskset for all children; both pools request 64 threads; shared server, no CPU exclusivity",
            "timing_policy": "Main: one warmup then position-balanced ordinary timings; optional independent profile uses one full rotation and remains diagnostic; optional RSS uses one cold proof per fresh process; all input checks outside prove/verify timing",
            "security_scope": "Direct packed-opening PIOP; no extra column aggregation or PCS; BLAKE3, grinding disabled, no equal-security claim",
            "verification_policy": "Every proof passes its verifier and prover/verifier claims/transcript checks. Source checks use a full oracle by default; opt-in reuse requires exact complete Claims equality with a full-oracle anchor on the same immutable inputs and log in the same run. Each benchmark process must exit zero before its report is accepted",
        }
        base.save(results / "effective_cgroups_start.json", base.effective_cgroups())
        base.save(results / "server_environment.json", inventory)

        def run(stage, argv, log_name, *, profiling=False):
            record = {"stage": stage, "argv": argv, "cwd": str(source), "utc_start": base.utc(),
                      "loadavg_start": os.getloadavg(), "memory_start_kib": base.memory(),
                      "effective_cgroups_start": base.effective_cgroups(),
                      "environment_overrides": {"FLOCK_ZC_TIMING": "1"} if profiling else {}}
            commands.append(record)
            base.save(results / "commands.json", commands)
            child_environment = environment.copy()
            if profiling:
                child_environment["FLOCK_ZC_TIMING"] = "1"
            started = time.monotonic()
            status(stage, command=argv, log=log_name)
            log_path = results / log_name
            with log_path.open("w", encoding="utf-8") as log:
                child = subprocess.Popen(argv, cwd=source, env=child_environment, stdout=log, stderr=subprocess.STDOUT)
                record["pid"] = child.pid
                base.save(results / "commands.json", commands)
                status(stage, pid=child.pid, log=log_name)
                returncode = child.wait()
            record.update(returncode=returncode, wall_seconds=time.monotonic() - started, utc_end=base.utc(),
                          loadavg_end=os.getloadavg(), memory_end_kib=base.memory(), log_sha256=base.sha256(log_path),
                          effective_cgroups_end=base.effective_cgroups())
            base.save(results / "commands.json", commands)
            require(returncode == 0, f"{stage} failed with exit {returncode}; see {log_name}")

        prefix = ["taskset", "-c", base.CPU_SPEC]
        artifact_path = root / "aggregate_build_artifact.json"
        if args.skip_build:
            artifact = base.load(artifact_path)
            require(artifact["source_manifest"] == manifest and artifact["toolchain"] == base.TOOLCHAIN
                    and artifact["cpu_spec"] == base.CPU_SPEC and artifact["bench"] == BENCH,
                    "cached build/source/configuration mismatch")
            require({c["stage"] for c in artifact["validation_commands"]} == VALIDATION_STAGES
                    and all(c["returncode"] == 0 for c in artifact["validation_commands"]), "cached build lacks successful validation")
            require(artifact["passed_tests"] >= args.min_tests
                    and set(args.required_tests) <= set(artifact["required_test_names"]),
                    "cached build lacks requested aggregate test validation")
            executable = Path(artifact["executable"]).resolve()
            require(executable.is_relative_to(source) and executable.is_file()
                    and base.sha256(executable) == artifact["executable_sha256"], "cached executable hash/path mismatch")
        else:
            cargo = str(Path.home() / ".cargo/bin/cargo")
            rustup = str(Path.home() / ".cargo/bin/rustup")
            rustc_prefix = prefix + [rustup, "run", base.TOOLCHAIN.lstrip("+"), "rustc"]
            run("rustc-version", rustc_prefix + ["-Vv"], "rustc-version.log")
            run("rustc-cfg", rustc_prefix + ["-C", "target-cpu=native", "--print", "cfg"], "rustc-cfg.log")
            features = re.findall(r'^target_feature="([^"]+)"$', (results / "rustc-cfg.log").read_text(), re.MULTILINE)
            require(REQUIRED_FEATURES <= set(features), "server/toolchain does not enable required native features")
            run("tests", prefix + [cargo, base.TOOLCHAIN, "test", "-p", "mikros", "--release", "--locked", "--offline"], "tests.log")
            test_output = (results / "tests.log").read_text()
            test_count = sum(map(int, re.findall(r"test result: ok\. (\d+) passed", test_output)))
            require(test_count >= args.min_tests and "univariate::domain::tests::" in test_output
                    and required_tests_passed(test_output, args.required_tests),
                    "required test count or named aggregate tests did not pass")
            run("build", prefix + [cargo, base.TOOLCHAIN, "bench", "-p", "mikros", "--bench", BENCH,
                                   "--no-run", "--locked", "--offline", "--message-format=json"], "build.jsonl")
            artifacts = []
            for line in (results / "build.jsonl").read_text().splitlines():
                if not line.startswith("{"):
                    continue
                message = json.loads(line)
                if (message.get("reason") == "compiler-artifact" and message.get("target", {}).get("name") == BENCH
                        and message.get("executable")):
                    artifacts.append(message["executable"])
            require(len(artifacts) == 1, "expected exactly one univariate benchmark executable")
            executable = Path(artifacts[0]).resolve()
            require(executable.is_file() and executable.is_relative_to(source), "benchmark executable outside source/build root")
            artifact = {"utc": base.utc(), "bench": BENCH, "source_manifest": manifest, "toolchain": base.TOOLCHAIN,
                        "cpu_spec": base.CPU_SPEC, "executable": str(executable), "executable_sha256": base.sha256(executable),
                        "native_target_features": features, "rustc_version": (results / "rustc-version.log").read_text().strip(),
                        "passed_tests": test_count, "required_test_names": args.required_tests,
                        "minimum_tests": args.min_tests,
                        "validation_results_directory": str(results),
                        "validation_commands": commands.copy()}
            base.save(artifact_path, artifact)
        require(REQUIRED_FEATURES <= set(artifact["native_target_features"]), "cached native feature mismatch")
        inventory.update(build_artifact=artifact, executable_sha256=artifact["executable_sha256"])
        base.save(results / "server_environment.json", inventory)
        reports = []

        def benchmark(stage, systems, logs, runs, *, rss=False, profiling=False):
            require(base.sha256(executable) == artifact["executable_sha256"], "executable changed before benchmark")
            output = results / f"{stage}.json"
            argv = prefix + [str(executable), "--logs", ",".join(map(str, logs)), "--runs", str(runs),
                             "--threads", str(base.THREADS), "--systems", ",".join(systems), "--output", str(output)]
            if rss:
                argv.append("--rss")
            if profiling:
                argv.append("--profile")
            if args.reuse_source_checks:
                argv.append("--reuse-source-checks")
            run(stage, argv, f"{stage}.log", profiling=profiling)
            report = check_report(output, systems, logs, runs, rss=rss, profile=profiling)
            require(report["metadata"].get("source_check_reuse") is args.reuse_source_checks,
                    f"{output.name}: requested source-check policy was not applied")
            reports.append((output.name, report))

        benchmark("smoke", args.smoke_systems, [14], 1)
        benchmark("comparison-64t", args.systems, args.logs, args.runs)
        if args.profile:
            benchmark("profile-64t", args.systems, args.logs, len(args.systems), profiling=True)
        if args.rss:
            for log_n in args.logs:
                for system in args.systems:
                    benchmark(f"rss-{system}-n{log_n}-64t", [system], [log_n], 1, rss=True)
        for key in ("rustc", "cpu", "os", "arch", "compiled_target_features", "native_backend",
                    "mikros_sources_blake3", "flock_sources_blake3", "build_configuration_blake3"):
            require(all(report["metadata"][key] == reports[0][1]["metadata"][key] for _, report in reports),
                    f"reports have inconsistent {key}")
        inputs, identities = {}, {}
        count = 0
        for name, report in reports:
            for sample in report["samples"] + report["warmups"]:
                count += 1
                key = sample["log_n"], sample["run"]
                require(inputs.setdefault(key, sample["input_digest"]) == sample["input_digest"], f"{name}: input changed between reports")
                key = (*key, "flock128" if sample["system"] == "flock128" else "mikros-univariate")
                require(identities.setdefault(key, sample["proof_digest"]) == sample["proof_digest"], f"{name}: proof changed between kernels/modes")
        require(verified_sources(source) == manifest and base.sha256(executable) == artifact["executable_sha256"],
                "source or executable changed during experiment")
        inventory.update(utc_end=base.utc(), memory_end_kib=base.memory(), loadavg_end=os.getloadavg(),
                         mikros_sources_blake3=reports[0][1]["metadata"]["mikros_sources_blake3"],
                         flock_sources_blake3=reports[0][1]["metadata"]["flock_sources_blake3"], validated_proof_records=count)
        base.save(results / "server_environment.json", inventory)
        base.save(results / "effective_cgroups_end.json", base.effective_cgroups())
        status("complete", threads=base.THREADS, systems=args.systems, smoke_systems=args.smoke_systems,
               required_test_names=args.required_tests, minimum_tests=args.min_tests, logs=args.logs, measured_runs=args.runs,
               profile_runs=len(args.systems) if args.profile else 0, rss_requested=args.rss,
               validated_proof_records=count, reports=[name for name, _ in reports])
    except Exception as error:
        try:
            base.save(results / "effective_cgroups_end.json", base.effective_cgroups())
        except OSError:
            pass
        status("failed", error=str(error))
        raise


if __name__ == "__main__":
    try:
        experiment(parse_args())
    except (ValueError, KeyError, OSError, subprocess.SubprocessError) as error:
        raise SystemExit(f"aggregate ablation experiment failed: {error}") from error
