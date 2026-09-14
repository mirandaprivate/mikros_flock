#!/usr/bin/env python3
"""Build and run a single-thread FLINT arithmetic benchmark, preserving raw trials."""
import argparse
import csv
import datetime as dt
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import statistics
import subprocess
import time

from verify_fields import PROFILES, verify

ROOT = Path(__file__).resolve().parent
OPS = ("add", "sub", "mul", "square", "scale", "inverse", "fold")


def capture(cmd):
    result = subprocess.run(cmd, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    return result.stdout.strip()


def sha256(path):
    h = hashlib.sha256()
    with Path(path).open("rb") as f:
        for block in iter(lambda: f.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def write_json(path, data):
    with path.open("x", encoding="utf-8") as f:
        json.dump(data, f, indent=2)
        f.write("\n")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--reps", type=int, default=9)
    parser.add_argument("--ms", type=float, default=150)
    parser.add_argument("--cpu", type=int, default=2)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--sanitize", action="store_true")
    args = parser.parse_args()
    if not 1 <= args.reps <= 50 or not 10 <= args.ms <= 2000:
        parser.error("reps must be 1..50; ms must be 10..2000")
    if args.cpu not in os.sched_getaffinity(0):
        parser.error("requested CPU is unavailable")
    prefix = ROOT / "deps/prefix"
    if not (prefix / "include/flint/flint.h").is_file():
        parser.error("Run bash setup.sh to build FLINT locally first")
    build = ROOT / "build"
    build.mkdir(exist_ok=True)
    binary = build / ("flint-bench-sanitize" if args.sanitize else "flint-bench")
    flags = ["-std=c11", "-Wall", "-Wextra", "-Werror", "-O3", "-march=native"]
    if args.sanitize:
        flags += ["-O1", "-g", "-fsanitize=address,undefined", "-fno-omit-frame-pointer"]
    compile_cmd = ["gcc", *flags, str(ROOT / "bench.c"), "-I" + str(prefix / "include"),
                   "-L" + str(prefix / "lib"), "-Wl,-rpath," + str(prefix / "lib"),
                   "-lflint", "-lmpfr", "-lgmp", "-lm", "-o", str(binary)]
    subprocess.run(compile_cmd, check=True)
    independent = verify()
    now = dt.datetime.now(dt.timezone.utc)
    out = args.out or ROOT / "results" / now.strftime("%Y%m%dT%H%M%SZ-1t")
    out = out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    write_json(out / "python_validation.json", independent)
    command = ["taskset", "-c", str(args.cpu), str(binary), str(out), str(args.reps), str(args.ms)]
    metadata = {
        "timestamp_utc": now.isoformat(), "platform": platform.platform(),
        "cpu_info": capture(["lscpu"]), "compiler": capture(["gcc", "--version"]),
        "compile_command": compile_cmd, "command": command,
        "sanitized_harness": args.sanitize, "library_sanitized": False,
        "pinned_logical_cpu": args.cpu,
        "linked_libraries": capture(["ldd", str(binary)]),
        "process_snapshot_before": capture(["ps", "-eo", "pid,comm,pcpu", "--sort=-pcpu"]),
        "seed": "SplitMix64: 0x464c494e5442454e XOR base modulus; rejection sampling",
        "measurement": "CLOCK_MONOTONIC wall time; CLOCK_THREAD_CPUTIME_ID also recorded; no overhead subtraction",
        "workload": "256 independent random input pairs repeatedly processed; outputs do not alias inputs; fixed preallocated buffers",
        "scope": "public fq_nmod API hot-buffer throughput including library-internal work/allocations, excluding context setup, input generation, oracle verification and harness buffer allocation",
    }
    provenance = {}
    for path in (ROOT / "bench.c", ROOT / "verify_fields.py", ROOT / "run.py", ROOT / "setup.sh", binary,
                 prefix / "lib/libflint.so", Path("/lib/x86_64-linux-gnu/libgmp.so.10"),
                 Path("/lib/x86_64-linux-gnu/libmpfr.so.6")):
        provenance[str(path)] = sha256(path)
    write_json(out / "sha256.json", provenance)
    for log in ("configure.log", "download-sha256.txt"):
        source = build / log
        if source.exists():
            shutil.copy2(source, out / log)
    write_json(out / "environment.json", metadata)
    start = time.monotonic()
    with (out / "execution.log").open("x") as log:
        process = subprocess.Popen(command, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        for line in process.stdout:
            print(line, end="", flush=True)
            log.write(line)
            log.flush()
        status = process.wait()
    if status:
        raise RuntimeError(f"benchmark exited {status}; incomplete artifacts in {out}")
    fields = json.loads((out / "field_metadata.json").read_text())
    expected_profiles = [(n, str(p), d, f"X^{d}-{c}") for n, p, d, c in PROFILES]
    actual_profiles = [(f["name"], f["p"], f["degree"], f["polynomial"]) for f in fields["fields"]]
    assert actual_profiles == expected_profiles
    assert fields["threads"] == 1 and fields["flint_bits"] == 64
    c_validation = json.loads((out / "c_validation.json").read_text())
    assert c_validation["passed"]
    with (out / "raw.csv").open(newline="") as f:
        raw = list(csv.DictReader(f))
    assert len(raw) == len(PROFILES) * len(OPS) * args.reps
    summary = []
    for name, p, degree, _ in PROFILES:
        for op in OPS:
            rows = [r for r in raw if r["field"] == name and r["operation"] == op]
            assert sorted(int(r["rep"]) for r in rows) == list(range(1, args.reps + 1))
            assert len({r["operations"] for r in rows}) == 1
            ns = [float(r["ns_per_op"]) for r in rows]
            assert min(ns) > 0
            median = statistics.median(ns)
            summary.append({
                "field": name, "p": p, "degree": degree, "operation": op,
                "reps": len(rows), "median_ns_per_op": median,
                "min_ns_per_op": min(ns), "max_ns_per_op": max(ns),
                "median_mops": 1000 / median,
                "median_cpu_ns_per_op": statistics.median(float(r["cpu_ns_per_op"]) for r in rows),
                "median_abs_deviation_ns": statistics.median(abs(n - median) for n in ns),
            })
    with (out / "summary.csv").open("x", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=list(summary[0]))
        writer.writeheader()
        writer.writerows(summary)
    write_json(out / "completion.json", {
        "passed": True, "elapsed_seconds": time.monotonic() - start,
        "samples": len(raw), "independent_oracle_checks": c_validation["independent_oracle_checks"],
        "python_irreducibility_passed": True,
        "process_snapshot_after": capture(["ps", "-eo", "pid,comm,pcpu", "--sort=-pcpu"]),
    })
    table = ["| Field | degree | add ns | mul ns | square ns | scale ns | inverse ns | fold ns |",
             "|---|---:|---:|---:|---:|---:|---:|---:|"]
    for name, _, degree, _ in PROFILES:
        cells = [next(r for r in summary if r["field"] == name and r["operation"] == op)
                 for op in ("add", "mul", "square", "scale", "inverse", "fold")]
        table.append(f"| {name} | {degree} | " + " | ".join(f'{r["median_ns_per_op"]:.2f}' for r in cells) + " |")
    text = "\n".join(table) + "\n"
    (out / "table.md").write_text(text)
    print(text)
    print(f"Artifacts: {out}")


if __name__ == "__main__":
    main()
