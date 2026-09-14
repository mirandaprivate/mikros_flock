#!/usr/bin/env python3
"""Run a verified 64-thread experiment locally on the destination; no SSH."""
from __future__ import annotations

import argparse
from collections import Counter
import hashlib
import json
import math
import os
from pathlib import Path, PurePosixPath
import re
import shutil
import statistics
import subprocess
import time


THREADS = 64
CPU_SPEC = "0-47,48-55,72-79"
CPUS = list(range(48)) + list(range(48, 56)) + list(range(72, 80))
SYSTEMS = ("lookup", "legacy", "stream", "simdv1", "simd", "wide", "fused", "native", "reuse", "factored", "factoredreuse", "packed", "flock", "flock128")
FLOCK_SYSTEMS = ("flock", "flock128")
DEFAULT_SYSTEMS = "reuse,factoredreuse,flock128"
TOOLCHAIN = "+nightly-2026-03-21"
HEX64 = re.compile(r"[0-9a-f]{64}\Z")
CLEARED_ENV = (
    "FLOCK_ZC_TIMING", "FLOCK_POOL_TRACE", "FLOCK_SPARSE_GATE", "RAYON_NUM_THREADS",
    "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_RUSTFLAGS",
    "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS", "CARGO_BUILD_TARGET",
    "RUSTC", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER",
)


def require(condition, message):
    if not condition:
        raise ValueError(message)


def utc():
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())


def load(path):
    return json.loads(path.read_text(encoding="utf-8"))


def save(path, value):
    temporary = path.with_suffix(path.suffix + ".tmp")
    temporary.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")
    temporary.replace(path)


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def memory():
    wanted = {"MemTotal", "MemAvailable", "SwapTotal", "SwapFree"}
    return {key: int(value.split()[0]) for key, value in
            (line.split(":", 1) for line in Path("/proc/meminfo").read_text().splitlines())
            if key in wanted}


def effective_cgroups():
    """Capture the inherited cgroup and every visible cgroup-v2 ancestor."""
    membership = Path("/proc/self/cgroup").read_text().strip()
    result = {"utc": utc(), "pid": os.getpid(), "proc_self_cgroup": membership}
    entries = [line[3:] for line in membership.splitlines() if line.startswith("0::")]
    mounts = [line for line in Path("/proc/self/mountinfo").read_text().splitlines()
              if " - cgroup2 " in line]
    if len(entries) != 1 or len(mounts) != 1:
        return {**result, "available": False, "reason": "no unique visible cgroup-v2 mount"}
    fields = mounts[0].split(" - ", 1)[0].split()
    unescape = lambda value: re.sub(r"\\([0-7]{3})", lambda match: chr(int(match[1], 8)), value)
    mount_root = PurePosixPath(unescape(fields[3]))
    mount_point = Path(unescape(fields[4])).resolve()
    member = PurePosixPath(entries[0])
    if ".." in member.parts or not member.is_relative_to(mount_root):
        return {**result, "available": False, "reason": "cgroup membership outside visible mount root"}
    leaf = (mount_point / str(member.relative_to(mount_root))).resolve()
    require(leaf.is_relative_to(mount_point), "invalid cgroup path")
    result.update(available=True, mount_root=str(mount_root), mount_point=str(mount_point), ancestors=[])
    current = leaf
    while True:
        record = {"path": str(current)}
        for name in ("cpu.max", "cpu.stat", "cpuset.cpus.effective", "cpuset.mems.effective",
                     "memory.max", "memory.current", "memory.events"):
            path = current / name
            if path.is_file():
                record[name] = path.read_text().strip()
        result["ancestors"].append(record)
        if current == mount_point:
            break
        current = current.parent
    return result


def verified_manifest(source):
    manifest = load(source / "source_manifest.json")
    require(bool(manifest["files_sha256"]), "empty source manifest")
    for name, expected in manifest["files_sha256"].items():
        path = (source / name).resolve()
        require(path.is_relative_to(source), f"manifest path outside source: {name}")
        require(HEX64.fullmatch(expected) and sha256(path) == expected, f"source hash mismatch: {name}")
    config = (source / ".cargo/config.toml").read_text()
    require(re.search(r'rustflags\s*=\s*\[\s*"-C"\s*,\s*"target-cpu=native"\s*\]', config),
            "expected repository target-cpu=native configuration")
    return manifest


def orders(systems, runs):
    result = []
    for run in range(1, runs + 1):
        shift = ((run - 1) // 2) % len(systems)
        order = systems[shift:] + systems[:shift]
        result.append(list(reversed(order)) if run % 2 == 0 else order)
    return result


def balanced_orders(systems, runs):
    plan = orders(systems, runs)
    counts = {system: [sum(row[position] == system for row in plan)
                       for position in range(len(systems))] for system in systems}
    require(all(len(set(values)) == 1 for values in counts.values()),
            f"unbalanced benchmark order: {counts}; use {2 * len(systems)} runs or a multiple")
    return plan


def check_report(path, systems, logs, runs, *, rss=False, profile=False):
    """Check raw records before accepting a stage, including all warmups."""
    report = load(path)
    meta = report["metadata"]
    require(report["schema_version"] == 1, f"unsupported report: {path}")
    for key, value in (("threads", THREADS), ("rayon_actual_threads", THREADS),
                       ("systems", systems), ("logs", logs), ("runs", runs),
                       ("mikros_phase_profiling", profile),
                       ("warmups_per_system_per_dimension", 0 if rss else 1),
                       ("mode", "fresh_process_rss_cold_proof" if rss else "paired_warm_timing")):
        require(meta[key] == value, f"{path.name}: metadata {key} mismatch")
    require(meta["transcript_hash"] == "blake3" and "no equal-128-bit-security claim" in meta["grinding"],
            f"{path.name}: changed transcript/security settings")
    require(HEX64.fullmatch(meta["mikros_sources_blake3"]), f"{path.name}: invalid source fingerprint")
    require(HEX64.fullmatch(meta["flock_sources_blake3"]), f"{path.name}: invalid Flock source fingerprint")
    require(meta["compression"] == {"mikros_kappa": 128, "flock": {"skip_rounds": 6, "factor": 64},
                                     "flock128": {"skip_rounds": 7, "factor": 128}},
            f"{path.name}: unexpected compression parameters")
    require(meta["tuning_environment"] == ({"FLOCK_ZC_TIMING": "1"} if profile else {}),
            f"{path.name}: unexpected benchmark tuning environment")
    require(len(report["samples"]) == len(logs) * len(systems) * runs, f"{path.name}: sample count")
    require(len(report["warmups"]) == (0 if rss else len(logs) * len(systems)), f"{path.name}: warmup count")
    require(all(sample["run"] > 0 for sample in report["samples"])
            and all(sample["run"] == 0 for sample in report["warmups"]), f"{path.name}: misplaced warmup")
    seen = set()
    for sample in report["samples"] + report["warmups"]:
        key = (sample["log_n"], sample["run"], sample["system"])
        require(key not in seen, f"{path.name}: duplicate sample {key}")
        seen.add(key)
        require(key[0] in logs and key[2] in systems and 0 <= key[1] <= runs, f"{path.name}: invalid sample")
        for name in ("prover_ms", "verifier_ms", "input_claim_check_ms"):
            require(math.isfinite(sample[name]) and sample[name] > 0, f"{path.name}: invalid {name}")
        for name in ("input_digest", "proof_digest"):
            require(HEX64.fullmatch(sample[name]), f"{path.name}: invalid {name}")
        n = sample["log_n"]
        if sample["system"] in FLOCK_SYSTEMS:
            k_skip = 7 if sample["system"] == "flock128" else 6
            fields, overhead = 2 * (1 << k_skip) + 3 + 2 * (n - k_skip), 32
        else:
            fields, overhead = 2190 + 2 * (n - 7), 24
        require((sample["payload_field_elements"], sample["payload_field_bytes"], sample["bincode_bytes"])
                == (fields, fields * 16, fields * 16 + overhead), f"{path.name}: changed proof shape")
        if rss:
            before, peak = sample["vm_rss_before_prove_kib"], sample["vm_hwm_after_prove_kib"]
            require(isinstance(before, int) and isinstance(peak, int) and peak >= before > 0,
                    f"{path.name}: invalid RSS")
        else:
            require(sample["vm_rss_before_prove_kib"] is None and sample["vm_hwm_after_prove_kib"] is None,
                    f"{path.name}: RSS mixed into warm timing")
        phases = sample.get("mikros_phases")
        require(bool(phases) == (profile and sample["system"] not in FLOCK_SYSTEMS), f"{path.name}: phase mode mismatch")
        if phases:
            require(all(math.isfinite(value) and value >= 0 for value in phases.values())
                    and sum(phases.values()) <= sample["prover_ms"] + 0.0001, f"{path.name}: invalid phases")
    expected = {(n, run, system) for n in logs for system in systems
                for run in range(1 if rss else 0, runs + 1)}
    require(seen == expected, f"{path.name}: sample coverage mismatch")
    for n in logs:
        for run in range(1 if rss else 0, runs + 1):
            samples = [s for s in report["samples"] + report["warmups"] if s["log_n"] == n and s["run"] == run]
            require(len({s["input_digest"] for s in samples}) == 1, f"{path.name}: unpaired input")
            expected_order = systems if run == 0 else orders(systems, runs)[run - 1]
            require(sorted(s["order"] for s in samples) == list(range(len(systems)))
                    and [s["system"] for s in sorted(samples, key=lambda s: s["order"])] == expected_order,
                    f"{path.name}: unexpected execution order")
            require(len({s["proof_digest"] for s in samples if s["system"] not in FLOCK_SYSTEMS}) <= 1,
                    f"{path.name}: Mikros variants changed proof bytes")
    require(len(report["summaries"]) == len(logs) * len(systems), f"{path.name}: summary count")
    require({(s["log_n"], s["system"]) for s in report["summaries"]}
            == {(n, system) for n in logs for system in systems}, f"{path.name}: summary coverage")
    for summary in report["summaries"]:
        selected = [s for s in report["samples"] if (s["log_n"], s["system"]) == (summary["log_n"], summary["system"])]
        require(summary["runs"] == len(selected) == runs, f"{path.name}: summary run count")
        for name in ("prover_ms", "verifier_ms", "input_claim_check_ms"):
            values = [sample[name] for sample in selected]
            for stat, expected_value in (("median", statistics.median(values)), ("min", min(values)), ("max", max(values))):
                require(math.isclose(summary[name][stat], expected_value, rel_tol=1e-12), f"{path.name}: incorrect {name}/{stat}")
        for name in ("payload_field_elements", "payload_field_bytes", "bincode_bytes"):
            require(summary[name] == selected[0][name], f"{path.name}: summary proof size")
    return report


def parse_args():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True, help="run root containing source/source_manifest.json")
    parser.add_argument("--systems", default=DEFAULT_SYSTEMS, help="comma-separated benchmark variants")
    parser.add_argument("--logs", default="30", help="comma-separated log2 dimensions")
    parser.add_argument("--runs", type=int, default=6, help="balanced ordinary timing samples per system")
    parser.add_argument("--profile", action="store_true", help="also run a separate 3-sample diagnostic pass")
    parser.add_argument("--skip-build", action="store_true", help="reuse the root's hash-verified tested executable")
    parser.add_argument("--results-name", default="results", help="fresh child directory; e.g. ablation for a second run")
    args = parser.parse_args()
    args.systems = args.systems.split(",")
    args.logs = [int(value) for value in args.logs.split(",")]
    require(args.systems and len(set(args.systems)) == len(args.systems)
            and set(args.systems) <= set(SYSTEMS), f"systems must be distinct members of {SYSTEMS}")
    require(args.logs and len(set(args.logs)) == len(args.logs) and all(13 <= n <= 30 for n in args.logs),
            "logs must be distinct values from 13 through 30; larger dimensions need a separate resource plan")
    require("flock128" not in args.systems or min(args.logs) >= 14, "flock128 requires log_n >= 14")
    require(args.runs > 0, "runs must be positive")
    balanced_orders(args.systems, args.runs)
    require(re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9_.-]*", args.results_name), "invalid results-name")
    return args


def experiment(args):
    root = args.root.resolve()
    source = (root / "source").resolve()
    results = root / args.results_name
    require(source.is_dir(), "missing source directory")
    require(not results.exists() or not any(results.iterdir()), "results directory must be fresh; preserve previous logs with --results-name")
    results.mkdir(exist_ok=True)

    def status(stage, **extra):
        record = {"utc": utc(), "stage": stage, **extra}
        save(results / "status.json", record)
        print(json.dumps(record), flush=True)

    status("preflight")
    commands = []
    try:
        manifest = verified_manifest(source)
        if (root / "source_manifest.json").is_file():
            require(load(root / "source_manifest.json") == manifest, "root and source manifests differ")
        available = memory()
        require(available["MemAvailable"] > 8 * 1024 * 1024, "need at least 8 GiB available memory")
        require(shutil.disk_usage(root).free > (1 if args.skip_build else 4) * 1024**3, "insufficient free disk")
        env = os.environ.copy()
        env["LC_ALL"] = "C"
        env["PATH"] = str(Path.home() / ".cargo/bin") + os.pathsep + env.get("PATH", "")
        for key in CLEARED_ENV:
            env.pop(key, None)
        env["CARGO_BUILD_JOBS"] = str(THREADS)
        topology = [tuple(map(int, line.split(","))) for line in
                    subprocess.check_output(["lscpu", "-p=CPU,CORE,SOCKET,NODE"], text=True, env=env).splitlines()
                    if line and not line.startswith("#")]
        selected = [row for row in topology if row[0] in CPUS]
        require(set(CPUS) <= os.sched_getaffinity(0), "not all planned 64 CPUs are available")
        require(len(selected) == THREADS and Counter(row[3] for row in selected) == {0: 32, 1: 32},
                "expected 32 selected logical CPUs on each NUMA node")
        cores = Counter((socket, core) for _, core, socket, _ in selected)
        require(len(cores) == 48 and Counter(cores.values()) == {1: 32, 2: 16},
                "expected 48 physical cores plus 16 SMT siblings")
        require(all(len({(socket, core) for _, core, socket, cpu_node in selected if cpu_node == node}) == 24
                    for node in (0, 1)), "expected 24 physical cores per NUMA node")
        start_cgroups = effective_cgroups()
        save(results / "effective_cgroups_start.json", start_cgroups)
        inventory = {
            "utc_start": utc(), "hostname": os.uname().nodename, "uname": list(os.uname()),
            "lscpu": json.loads(subprocess.check_output(["lscpu", "-J"], env=env)),
            "topology": topology, "allowed_cpus": sorted(os.sched_getaffinity(0)),
            "cpu_spec": CPU_SPEC, "selected_cpus": CPUS, "threads": THREADS,
            "selected_logical_cpus_per_numa_node": {"0": 32, "1": 32},
            "selected_physical_cores": 48, "additional_smt_siblings": 16,
            "memory_start_kib": available, "loadavg_start": os.getloadavg(),
            "source_manifest": manifest, "toolchain": TOOLCHAIN, "cleared_environment_keys": CLEARED_ENV,
            "build_rustflags": "repository .cargo/config.toml: -C target-cpu=native",
            "systems": args.systems, "logs": args.logs, "runs": args.runs,
            "main_execution_orders": balanced_orders(args.systems, args.runs),
            "profile_enabled": args.profile, "profile_runs": 3 if args.profile else 0,
            "skip_build": args.skip_build,
            "numa_policy": "Two NUMA nodes, 32 selected logical CPUs each; Linux default first-touch; no explicit memory binding or interleave",
            "thread_policy": "Same 64-CPU taskset for tests/build/all benchmark processes; global Rayon and Flock pools both use --threads 64; 48 physical cores plus 16 SMT siblings; shared server, no CPU exclusivity",
            "timing_policy": "One warmup/system/dimension plus balanced measured runs; ordinary main and fresh-process RSS have no phase instrumentation; optional separate profile has 3 runs and no position-balance claim; no setup subtracted; full source-input checks retained outside prove/verify timings",
            "security_scope": "PIOP reductions only, no PCS commitments/openings; BLAKE3, grinding disabled; no equal-128-bit-soundness claim",
        }
        save(results / "server_environment.json", inventory)

        def run(stage, argv, log_name, *, profiling=False):
            start = time.monotonic()
            record = {"stage": stage, "argv": argv, "utc_start": utc(),
                      "loadavg_start": os.getloadavg(), "memory_start_kib": memory(),
                      "effective_cgroups_start": effective_cgroups(),
                      "environment_overrides": {"FLOCK_ZC_TIMING": "1"} if profiling else {}}
            process_env = env.copy()
            if profiling:
                process_env["FLOCK_ZC_TIMING"] = "1"
            status(stage, command=argv, log=log_name)
            with (results / log_name).open("w", encoding="utf-8") as log:
                process = subprocess.Popen(argv, cwd=source, env=process_env, stdout=log, stderr=subprocess.STDOUT)
                record["pid"] = process.pid
                status(stage, pid=process.pid, log=log_name)
                returncode = process.wait()
            record.update(returncode=returncode, wall_seconds=time.monotonic() - start,
                          utc_end=utc(), loadavg_end=os.getloadavg(), memory_end_kib=memory(),
                          effective_cgroups_end=effective_cgroups())
            commands.append(record)
            save(results / "commands.json", commands)
            require(returncode == 0, f"{stage} failed with exit {returncode}; see {log_name}")

        prefix = ["taskset", "-c", CPU_SPEC]
        artifact_path = root / "build_artifact.json"
        if args.skip_build:
            artifact = load(artifact_path)
            require(artifact["source_manifest"] == manifest and artifact["toolchain"] == TOOLCHAIN
                    and artifact["cpu_spec"] == CPU_SPEC, "cached build configuration/source mismatch")
            require({record["stage"] for record in artifact["validation_commands"]} == {"tests", "tests-flock-skip7", "tests-flock-regression", "tests-ntt", "build"}
                    and all(record["returncode"] == 0 for record in artifact["validation_commands"]),
                    "cached executable has no successful tests/build provenance")
            executable = Path(artifact["executable"])
            require(executable.is_file() and sha256(executable) == artifact["executable_sha256"], "cached executable hash mismatch")
        else:
            cargo = str(Path.home() / ".cargo/bin/cargo")
            run("tests", prefix + [cargo, TOOLCHAIN, "test", "-p", "mikros", "--release", "--locked", "--offline"], "tests.log")
            run("tests-flock-skip7", prefix + [cargo, TOOLCHAIN, "test", "-p", "flock-core", "--release", "--lib", "skip7", "--locked", "--offline"], "tests-flock-skip7.log")
            run("tests-flock-regression", prefix + [cargo, TOOLCHAIN, "test", "-p", "flock-core", "--release", "--lib", "zerocheck::tests", "--locked", "--offline"], "tests-flock-regression.log")
            run("tests-ntt", prefix + [cargo, TOOLCHAIN, "test", "-p", "flock-core", "--release", "--lib", "ntt::", "--locked", "--offline"], "tests-ntt.log")
            run("build", prefix + [cargo, TOOLCHAIN, "bench", "-p", "mikros", "--bench", "hadamard_vs_flock",
                                 "--no-run", "--locked", "--offline", "--message-format=json"], "build.jsonl")
            artifacts = []
            for line in (results / "build.jsonl").read_text().splitlines():
                if not line.startswith("{"):
                    continue
                message = json.loads(line)
                if (message.get("reason") == "compiler-artifact" and message.get("target", {}).get("name") == "hadamard_vs_flock"
                        and message.get("executable")):
                    artifacts.append(message["executable"])
            require(len(artifacts) == 1, "expected exactly one benchmark executable")
            executable = Path(artifacts[0]).resolve()
            artifact = {"utc": utc(), "source_manifest": manifest, "toolchain": TOOLCHAIN, "cpu_spec": CPU_SPEC,
                        "executable": str(executable), "executable_sha256": sha256(executable),
                        "validation_results_directory": str(results), "validation_commands": commands.copy()}
            save(artifact_path, artifact)
        inventory["build_artifact"] = artifact
        inventory["executable_sha256"] = artifact["executable_sha256"]
        save(results / "server_environment.json", inventory)
        reports = []

        def benchmark(stage, systems, logs, runs, *, rss=False, profiling=False):
            output = results / f"{stage}.json"
            argv = prefix + [str(executable), "--logs", ",".join(map(str, logs)), "--runs", str(runs),
                             "--threads", str(THREADS), "--systems", ",".join(systems), "--output", str(output)]
            if rss:
                argv.append("--rss")
            if profiling:
                argv.append("--profile")
            run(stage, argv, f"{stage}.log", profiling=profiling)
            reports.append((output.name, check_report(output, systems, logs, runs, rss=rss, profile=profiling)))

        benchmark("smoke", list(SYSTEMS), [20], 1)
        required_features = {"gfni", "avx512f", "avx512bw", "avx512vbmi", "vpclmulqdq", "pclmulqdq", "sse4.1"}
        require(required_features <= set(reports[0][1]["metadata"]["compiled_target_features"]),
                "native smoke executable lacks the recorded GFNI/AVX-512/VPCLMUL build features")
        benchmark("comparison-64t", args.systems, args.logs, args.runs)
        if args.profile:
            benchmark("profile-64t", args.systems, args.logs, 3, profiling=True)
        for log_n in args.logs:
            for system in args.systems:
                benchmark(f"rss-{system}-n{log_n}-64t", [system], [log_n], 1, rss=True)
        fingerprints = {report["metadata"]["mikros_sources_blake3"] for _, report in reports}
        require(len(fingerprints) == 1, "reports used different compiled sources")
        for key in ("cpu", "os", "arch", "rustc", "compiled_target_features", "flock_sources_blake3"):
            require(all(report["metadata"][key] == reports[0][1]["metadata"][key] for _, report in reports),
                    f"reports used different {key}")
        identities, inputs = {}, {}
        count = 0
        for name, report in reports:
            for sample in report["samples"] + report["warmups"]:
                count += 1
                base = (sample["log_n"], sample["run"])
                require(inputs.setdefault(base, sample["input_digest"]) == sample["input_digest"], f"{name}: input digest changed across reports")
                key = (*base, sample["system"] if sample["system"] in FLOCK_SYSTEMS else "mikros")
                identity = (sample["input_digest"], sample["proof_digest"])
                require(identities.setdefault(key, identity) == identity, f"{name}: proof changed across variants/modes")
        require(verified_manifest(source) == manifest and sha256(executable) == inventory["executable_sha256"],
                "source or executable changed during experiment")
        inventory.update(utc_end=utc(), memory_end_kib=memory(), loadavg_end=os.getloadavg(),
                         mikros_sources_blake3=next(iter(fingerprints)), validated_proof_records=count)
        save(results / "server_environment.json", inventory)
        save(results / "effective_cgroups_end.json", effective_cgroups())
        status("complete", threads=THREADS, systems=args.systems, logs=args.logs, measured_runs=args.runs,
               profile_runs=3 if args.profile else 0, validated_proof_records=count,
               reports=[name for name, _ in reports])
    except Exception as error:
        save(results / "effective_cgroups_end.json", effective_cgroups())
        status("failed", error=str(error))
        raise


if __name__ == "__main__":
    try:
        experiment(parse_args())
    except (ValueError, KeyError, OSError, subprocess.SubprocessError) as error:
        raise SystemExit(f"server64 experiment failed: {error}") from error
