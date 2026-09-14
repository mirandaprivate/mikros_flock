#!/usr/bin/env python3
"""Bounded native probe of packed coefficient aggregation, before full proofs.

Run from an extracted source snapshot. This driver can be launched in the
background; status.json and each command's stdout/stderr remain on disk. It
builds and runs all non-ignored permutation library tests, then explicitly runs
the ignored aggregation benchmark. It does not run standalone or E2E proofs.
"""
import argparse
import collections
import json
import math
import os
from pathlib import Path
import re
import signal
import statistics
import subprocess
import time

from run_packed_permutation_64 import CPUS, CPU_SPEC, NATIVE_FEATURES, TOOLCHAIN, require, sha256


KERNELS = ("baseline", "gfni", "tower")
SHAPES = ((4, 4), (3, 4), (4, 2), (3, 2))
BENCHMARK = "packed_aggregate_kernel_benchmark"


def parse_probe(stdout):
    """Strip Rust's test-harness prose while preserving the benchmark JSON."""
    records = []
    for line in stdout.splitlines():
        start = line.find('{"record":"aggregate_')
        if start < 0:
            continue
        row = json.loads(line[start:])
        require(row["record"] in ("aggregate_metadata", "aggregate_sample"), "unexpected probe record")
        records.append(row)
    require(bool(records), "benchmark emitted no JSON records")
    return records


def summarize(records, log_rows, reps):
    metadata = [row for row in records if row["record"] == "aggregate_metadata"]
    require(len(metadata) == 1, "benchmark must emit one metadata record")
    metadata = metadata[0]
    require(metadata["threads"] == 64 and metadata["log_rows"] == log_rows and metadata["reps"] == reps,
            "benchmark dimensions or thread count differ from configuration")
    require(metadata["low_weight_log"] == 13 and metadata["online_weight_conversion_included"] is True,
            "low-weight blocking or online-conversion timing boundary changed")
    require(metadata["native_kernels_compiled"] is True, "probe binary did not compile the native kernels")
    samples = [row for row in records if row["record"] == "aggregate_sample"]
    require(len(samples) == len(SHAPES) * len(KERNELS) * (reps + 1), "incomplete probe sample set")
    require(all(row["verified"] is True for row in samples), "a candidate changed coefficient sums")
    require({(row["columns"], row["degree"]) for row in samples} == set(SHAPES), "unexpected coefficient shapes")
    require({row["kernel"] for row in samples} == set(KERNELS), "unexpected probe kernels")
    for row in samples:
        require(isinstance(row["milliseconds"], (int, float)) and math.isfinite(row["milliseconds"])
                and row["milliseconds"] >= 0, "invalid probe time")
        require(row["warmup"] is (row["iteration"] == 0), "warmup flag and iteration disagree")
    groups = []
    for columns, degree in SHAPES:
        shape_groups = []
        for kernel in KERNELS:
            all_group = [row for row in samples if (row["kernel"], row["columns"], row["degree"])
                         == (kernel, columns, degree)]
            require(sorted(row["iteration"] for row in all_group) == list(range(reps + 1)), "missing or duplicate iterations")
            measured = [row for row in all_group if not row["warmup"]]
            require(len(measured) == reps, "wrong number of measured repetitions")
            require(collections.Counter(row["slot"] for row in measured) == {0:reps // 3, 1:reps // 3, 2:reps // 3},
                    "kernel order is not balanced across slots")
            times = [row["milliseconds"] for row in measured]
            require(statistics.median(times) > 0, "zero median probe time")
            shape_groups.append({"kernel":kernel, "columns":columns, "degree":degree,
                                 "median_ms":statistics.median(times), "range_ms":[min(times), max(times)],
                                 "repetitions":reps, "warmup_ms":all_group[0]["milliseconds"]})
        baseline = next(row["median_ms"] for row in shape_groups if row["kernel"] == "baseline")
        for row in shape_groups:
            row["over_baseline"] = row["median_ms"] / baseline
            row["speedup_vs_baseline"] = baseline / row["median_ms"]
        groups.extend(shape_groups)
    return {"metadata":metadata, "measured_samples":len(SHAPES) * len(KERNELS) * reps,
            "warmup_samples":len(SHAPES) * len(KERNELS), "all_verified":True,
            "groups":groups,
            "scope":"Coefficient aggregation microbenchmark only; preparation of challenge-dependent weights is online. "
                    "Medians exclude one warmup per kernel and shape. Results do not establish whole-proof or E2E speedups."}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--logs", default="results-packed-aggregate-probe")
    parser.add_argument("--log-rows", type=int, default=21)
    parser.add_argument("--reps", type=int, default=6)
    parser.add_argument("--timeout-seconds", type=int, default=1800,
                        help="Upper bound per command, including build/test/probe (default: 1800)")
    args = parser.parse_args()
    require(13 <= args.log_rows <= 25, "log rows must be between 13 and 25")
    require(args.reps > 0 and args.reps % 3 == 0, "repetitions must be a positive multiple of three")
    require(1 <= args.timeout_seconds <= 7200, "command timeout must be between 1 and 7200 seconds")
    root = Path.cwd().resolve()
    logs = (root / args.logs).resolve()
    require(logs.is_relative_to(root) and logs != root, "results must stay inside this source snapshot")
    logs.mkdir(exist_ok=False)
    env = os.environ.copy()
    cleared = []
    for key in list(env):
        if key.startswith(("FLOCK_", "MIKROS_")) or key in (
                "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER", "CARGO_BUILD_RUSTFLAGS"):
            cleared.append(key)
            env.pop(key)
    env.update(RUSTFLAGS="-C target-cpu=native", RAYON_NUM_THREADS="64", CARGO_TARGET_DIR=str(root / "target"),
               MIKROS_AGG_BENCH_LOG_ROWS=str(args.log_rows), MIKROS_AGG_BENCH_THREADS="64", MIKROS_AGG_BENCH_REPS=str(args.reps))
    env["PATH"] = str(Path.home() / ".cargo/bin") + ":" + env.get("PATH", "")
    status = {"state":"starting", "pid":os.getpid(), "completed":[], "failed":[],
              "started_utc":time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}

    def save(name, data):
        temporary = logs / (name + ".tmp")
        temporary.write_text(json.dumps(data, indent=2) + "\n")
        temporary.replace(logs / name)

    def verify_sources():
        manifest = json.loads((root / "source_manifest.json").read_text())
        require(bool(manifest["files_sha256"]), "empty source manifest")
        for name, expected in manifest["files_sha256"].items():
            path = (root / name).resolve()
            require(path.is_relative_to(root) and path.is_file(), f"invalid manifest path: {name}")
            require(sha256(path) == expected, f"source hash mismatch: {name}")
        return manifest

    def resource_snapshot(name):
        require(CPUS.issubset(os.sched_getaffinity(0)), "planned 64 CPUs unavailable")

        def counters():
            result = {}
            for line in Path("/proc/stat").read_text().splitlines():
                values = line.split()
                if values[0].startswith("cpu") and values[0][3:].isdigit() and int(values[0][3:]) in CPUS:
                    ticks = list(map(int, values[1:9]))
                    result[values[0]] = (sum(ticks), ticks[3] + ticks[4])
            require(len(result) == 64, "missing selected CPU counters")
            return result

        before = counters()
        time.sleep(1)
        after = counters()
        ticks = sum(after[c][0] - before[c][0] for c in before)
        idle = sum(after[c][1] - before[c][1] for c in before)
        busy = 1 - idle / max(1, ticks)
        available = int(next(line.split()[1] for line in Path("/proc/meminfo").read_text().splitlines()
                             if line.startswith("MemAvailable:")))
        save(name + ".resources-before.json", {"selected_cpu_busy_fraction":busy, "mem_available_kib":available,
             "allowed_affinity":sorted(os.sched_getaffinity(0)), "loadavg":os.getloadavg()})
        require(busy < 0.10, f"selected CPUs busy before {name}: {busy:.1%}")
        require(available > 16 * 1024 * 1024, "less than 16 GiB memory available")

    def stop_child(process):
        # Every command owns a fresh session/process group. Never signal other
        # experiment jobs, inherited process groups, or processes by name.
        if process.poll() is None:
            try:
                os.killpg(process.pid, signal.SIGTERM)
            except ProcessLookupError:
                return
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                process.wait(timeout=5)

    def run(name, command, pinned=False):
        status["state"] = name
        save("status.json", status)
        if pinned:
            command = ["/usr/bin/time", "-v", "-o", str(logs / (name + ".resource.txt")), "taskset", "-c", CPU_SPEC] + command
        save(name + ".command.json", command)
        start = time.monotonic()
        timed_out = False
        with (logs / (name + ".stdout")).open("w") as stdout, (logs / (name + ".stderr")).open("w") as stderr:
            process = subprocess.Popen(command, cwd=root, env=env, stdout=stdout, stderr=stderr, start_new_session=True)
            try:
                process.wait(timeout=args.timeout_seconds)
            except subprocess.TimeoutExpired:
                timed_out = True
                stop_child(process)
            except BaseException:
                stop_child(process)
                raise
        event = {"name":name, "exit_code":process.returncode, "timed_out":timed_out,
                 "wall_seconds":time.monotonic() - start}
        status["completed" if process.returncode == 0 and not timed_out else "failed"].append(event)
        save("status.json", status)
        print(json.dumps(event), flush=True)
        require(not timed_out, f"{name} exceeded {args.timeout_seconds}s; its own child group was stopped")
        require(process.returncode == 0, f"{name} failed; complete stdout/stderr retained")

    try:
        manifest = verify_sources()
        manifest_hash = sha256(root / "source_manifest.json")
        save("config.json", {**vars(args), "threads":64, "warmups":1, "cpu_affinity_spec":CPU_SPEC,
             "toolchain":TOOLCHAIN, "rustflags":env["RUSTFLAGS"], "source_manifest_sha256":manifest_hash,
             "cleared_environment_keys":sorted(cleared), "benchmark_environment":{k:v for k,v in env.items() if k.startswith("MIKROS_AGG_BENCH_")}})
        cpuinfo = Path("/proc/cpuinfo").read_text()
        save("hardware.json", {"utc":status["started_utc"], "hostname":os.uname().nodename,
             "selected_cpus":sorted(CPUS), "cpuinfo":cpuinfo, "meminfo":Path("/proc/meminfo").read_text(),
             "cgroup":Path("/proc/self/cgroup").read_text(), "rustflags":env["RUSTFLAGS"]})
        for block in cpuinfo.split("\n\n"):
            fields = dict(line.split(":", 1) for line in block.splitlines() if ":" in line)
            fields = {key.strip():value.strip() for key,value in fields.items()}
            if fields.get("processor", "").isdigit() and int(fields["processor"]) in CPUS:
                require(NATIVE_FEATURES.issubset(fields.get("flags", "").split()), "selected CPU lacks required native features")
        resource_snapshot("initial")
        # Reproducible source tarballs use zero mtimes. Refresh mtimes only so
        # Cargo cannot mistake a copied cache for the current source contents.
        for name in manifest["files_sha256"]:
            (root / name).touch()
        run("rustc", ["rustc", TOOLCHAIN, "-Vv"])
        run("native-cfg", ["rustc", TOOLCHAIN, "-C", "target-cpu=native", "--print", "cfg"])
        target_features = re.findall(r'^target_feature="([^"]+)"$', (logs / "native-cfg.stdout").read_text(), re.MULTILINE)
        require(NATIVE_FEATURES.issubset(target_features), "native compiler target lacks required features")
        run("build-tests", ["cargo", TOOLCHAIN, "test", "--release", "--offline", "--locked", "-j8", "-p", "mikros-permutation",
                            "--lib", "--no-run", "--message-format=json"])
        artifacts = [json.loads(line) for line in (logs / "build-tests.stdout").read_text().splitlines() if line.strip()]
        executables = {row["executable"] for row in artifacts if row.get("reason") == "compiler-artifact"
                       and row.get("executable") and row.get("profile", {}).get("test")
                       and row.get("target", {}).get("name") == "mikros_permutation"}
        require(len(executables) == 1, "build did not identify exactly one permutation library test executable")
        binary = Path(executables.pop()).resolve()
        require(binary.is_relative_to(root / "target") and binary.is_file(), "test executable outside this snapshot target")
        binary_hash = sha256(binary)
        save("binary-sha256.json", {str(binary):binary_hash})
        run("list-tests", [str(binary), "--list"], pinned=True)
        names = [line.removesuffix(": test") for line in (logs / "list-tests.stdout").read_text().splitlines() if line.endswith(": test")]
        matches = [name for name in names if name.endswith("::" + BENCHMARK)]
        require(len(matches) == 1, "ignored benchmark name is missing or ambiguous")
        run("test-permutation-lib", [str(binary), "--test-threads", "1"], pinned=True)
        test_output = (logs / "test-permutation-lib.stdout").read_text()
        test_result = re.search(r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out;", test_output)
        require(test_result is not None and int(test_result[1]) > 0 and int(test_result[2]) == 0 and int(test_result[5]) == 0,
                "library correctness tests did not all complete successfully")
        resource_snapshot("probe")
        run("probe", [str(binary), matches[0], "--exact", "--ignored", "--nocapture", "--test-threads", "1"], pinned=True)
        probe_output = (logs / "probe.stdout").read_text()
        require(re.search(r"test result: ok\. 1 passed; 0 failed;", probe_output), "the ignored benchmark did not pass")
        records = parse_probe(probe_output)
        (logs / "probe-records.jsonl").write_text("".join(json.dumps(row) + "\n" for row in records))
        (logs / "samples.jsonl").write_text("".join(json.dumps(row) + "\n" for row in records if row["record"] == "aggregate_sample"))
        summary = summarize(records, args.log_rows, args.reps)
        verify_sources()
        require(sha256(root / "source_manifest.json") == manifest_hash, "manifest changed during probe")
        require(sha256(binary) == binary_hash, "test binary changed during probe")
        summary.update(source_manifest_sha256=manifest_hash, binary_sha256=binary_hash,
                       source_and_binary_verified_before_and_after=True, native_target_features=target_features,
                       library_tests_passed=int(test_result[1]), library_tests_ignored=int(test_result[3]),
                       samples_sha256=sha256(logs / "samples.jsonl"), stdout_sha256=sha256(logs / "probe.stdout"))
        save("summary.json", summary)
        status["state"] = "complete"
    except BaseException as error:
        status["state"] = "failed"
        status["error"] = f"{type(error).__name__}: {error}"
        raise
    finally:
        status["finished_utc"] = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
        save("status.json", status)


if __name__ == "__main__":
    main()
