#!/usr/bin/env python3
"""Run paired N=2^30 experiments locally on the SSH destination. No credentials."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import time


def utc():
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", type=Path, required=True)
    args = parser.parse_args()
    root = args.root.resolve()
    source = root / "source"
    results = root / "results"
    results.mkdir(exist_ok=True)
    env = os.environ.copy()
    env["LC_ALL"] = "C"
    env["PATH"] = str(Path.home() / ".cargo/bin") + os.pathsep + env["PATH"]
    for key in ("FLOCK_ZC_TIMING", "FLOCK_POOL_TRACE", "FLOCK_SPARSE_GATE", "RAYON_NUM_THREADS", "RUSTFLAGS"):
        env.pop(key, None)
    cargo = str(Path.home() / ".cargo/bin/cargo")
    toolchain = "+nightly-2026-03-21"

    def save(name, obj):
        target = results / name
        temporary = target.with_suffix(target.suffix + ".tmp")
        temporary.write_text(json.dumps(obj, indent=2) + "\n")
        temporary.replace(target)

    def status(stage, **extra):
        record = {"utc": utc(), "stage": stage, **extra}
        save("status.json", record)
        print(json.dumps(record), flush=True)

    def memory():
        wanted = {"MemTotal", "MemAvailable", "SwapTotal", "SwapFree"}
        return {k: int(v.split()[0]) for k, v in
                (line.split(":", 1) for line in Path("/proc/meminfo").read_text().splitlines()) if k in wanted}

    manifest = json.loads((source / "source_manifest.json").read_text())
    for name, expected in manifest["files_sha256"].items():
        path = (source / name).resolve()
        assert path.is_relative_to(source), name
        assert hashlib.sha256(path.read_bytes()).hexdigest() == expected, name
    assert memory()["MemAvailable"] > 8 * 1024 * 1024, "need at least 8 GiB available memory"
    assert shutil.disk_usage(root).free > 4 * 1024**3, "need at least 4 GiB disk before build"
    lscpu = json.loads(subprocess.check_output(["lscpu", "-J"], env=env))
    topology_text = subprocess.check_output(["lscpu", "-p=CPU,CORE,SOCKET,NODE"], text=True, env=env)
    topology = [tuple(map(int, line.split(","))) for line in topology_text.splitlines() if not line.startswith("#")]
    allowed = os.sched_getaffinity(0)
    nodes = sorted({node for cpu, _, _, node in topology if cpu in allowed})
    node = nodes[0]
    physical = {}
    for cpu, core, socket, cpu_node in topology:
        if cpu in allowed and cpu_node == node:
            physical.setdefault((socket, core), cpu)
    cpus = sorted(physical.values())
    assert len(cpus) >= 24, "planned comparison needs 24 physical cores on one NUMA node"
    configs = {threads: cpus[:threads] for threads in (1, 6, 24)}
    cgroup = {}
    for name in ("cpu.max", "cpuset.cpus.effective", "cpuset.mems.effective", "memory.max"):
        path = Path("/sys/fs/cgroup") / name
        if path.exists():
            cgroup[name] = path.read_text().strip()
    inventory = {"utc_start": utc(), "hostname": os.uname().nodename,
                 "uname": list(os.uname()), "lscpu": lscpu, "topology": topology,
                 "allowed_cpus": sorted(allowed), "selected_numa_node": node,
                 "config_cpu_lists": configs, "memory_start_kib": memory(),
                 "cgroup_root_limits": cgroup, "loadavg_start": os.getloadavg(),
                 "numa_policy": "All processes restricted to physical cores on one NUMA node; Linux default first-touch memory placement, no explicit memory binding or interleave",
                 "source_manifest": manifest, "build_rustflags": "repository .cargo/config.toml: -C target-cpu=native",
                 "thread_policy": "One logical CPU per distinct physical core; global Rayon and Flock all-core pools use the same --threads",
                 "timing_policy": "6 measured runs plus one warmup/system; packed and flock alternate equally; original-input source checks retained outside prover/verify timings"}
    save("server_environment.json", inventory)

    commands = []
    def run(stage, argv, log_name):
        start = time.monotonic()
        record = {"stage": stage, "argv": argv, "utc_start": utc(),
                  "loadavg_start": os.getloadavg(), "memory_start_kib": memory()}
        status(stage, command=argv, log=log_name)
        with (results / log_name).open("w") as log:
            process = subprocess.Popen(argv, cwd=source, env=env, stdout=log, stderr=subprocess.STDOUT)
            status(stage, pid=process.pid, log=log_name)
            returncode = process.wait()
        record.update(returncode=returncode, wall_seconds=time.monotonic()-start,
                      utc_end=utc(), loadavg_end=os.getloadavg(), memory_end_kib=memory())
        commands.append(record)
        save("commands.json", commands)
        if returncode:
            status("failed", failed_stage=stage, returncode=returncode, log=log_name)
            raise SystemExit(returncode)

    # Offline builds use the destination's cache and must never silently alter
    # the lockfile. A missing dependency is reported for explicit remediation.
    run("tests", [cargo, toolchain, "test", "-p", "mikros", "--release", "--locked", "--offline"], "tests.log")
    run("build", [cargo, toolchain, "bench", "-p", "mikros", "--bench", "hadamard_vs_flock",
                  "--no-run", "--locked", "--offline", "--message-format=json"], "build.jsonl")
    executables = []
    for line in (results / "build.jsonl").read_text().splitlines():
        if not line.startswith("{"):
            continue
        message = json.loads(line)
        if (message.get("reason") == "compiler-artifact" and
                message.get("target", {}).get("name") == "hadamard_vs_flock" and message.get("executable")):
            executables.append(message["executable"])
    assert len(executables) == 1, executables
    executable = executables[0]
    inventory["executable_sha256"] = hashlib.sha256(Path(executable).read_bytes()).hexdigest()
    save("server_environment.json", inventory)
    # Smoke test actual native instruction paths before allocating the N30 witness.
    run("smoke", ["taskset", "-c", ",".join(map(str, configs[1])), executable,
                  "--logs", "20", "--runs", "1", "--threads", "1", "--systems", "lookup,packed,flock",
                  "--output", str(results / "smoke.json")], "smoke.log")
    for threads, affinity in configs.items():
        prefix = ["taskset", "-c", ",".join(map(str, affinity)), executable]
        run(f"comparison-{threads}t", prefix + ["--logs", "30", "--runs", "6", "--threads", str(threads),
            "--systems", "packed,flock", "--output", str(results / f"comparison-{threads}t.json")], f"comparison-{threads}t.log")
        for system in ("packed", "flock"):
            run(f"rss-{system}-{threads}t", prefix + ["--logs", "30", "--threads", str(threads),
                "--systems", system, "--rss", "--output", str(results / f"rss-{system}-{threads}t.json")],
                f"rss-{system}-{threads}t.log")
    inventory.update(utc_end=utc(), memory_end_kib=memory(), loadavg_end=os.getloadavg())
    save("server_environment.json", inventory)
    status("complete", configs=list(configs), log_n=30, measured_runs=6)


if __name__ == "__main__":
    main()
