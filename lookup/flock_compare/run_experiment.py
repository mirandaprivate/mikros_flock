#!/usr/bin/env python3
"""Run locally and serially; all outputs stay in this isolated experiment."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

here = Path(__file__).resolve().parent
parser = argparse.ArgumentParser()
parser.add_argument("--name", required=True)
args = parser.parse_args()
results = here / "results" / args.name
results.mkdir(parents=True, exist_ok=False)
binary = here / "target/release/permutation-flock-compare"
assert binary.is_file()
manifest = json.loads((here / "snapshot/manifest.json").read_text())
for group in ("copied_files_sha256", "generated_files_sha256"):
    for name, digest in manifest[group].items():
        assert hashlib.sha256((here / "snapshot" / name).read_bytes()).hexdigest() == digest, name
env = os.environ.copy()
for key in ("FLOCK_ZC_TIMING", "FLOCK_POOL_TRACE", "FLOCK_SPARSE_GATE", "GKR_TRACE", "RAYON_NUM_THREADS"):
    env.pop(key, None)
env["LC_ALL"] = "C"
topology = subprocess.check_output(["lscpu", "-p=CPU,CORE,SOCKET,NODE"], text=True)
physical = {}
allowed = os.sched_getaffinity(0)
for line in topology.splitlines():
    if line.startswith("#"):
        continue
    cpu, core, socket, _ = line.split(",")
    if int(cpu) in allowed:
        physical.setdefault((socket, core), int(cpu))
cpus = sorted(physical.values())
assert len(cpus) >= 6
commands = []
def save(name, obj):
    (results / name).write_text(json.dumps(obj, indent=2) + "\n")
def status(stage, **more):
    rec = {"stage": stage, "utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), **more}
    save("status.json", rec)
    print(json.dumps(rec), flush=True)

inventory = {"start_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
    "lscpu": json.loads(subprocess.check_output(["lscpu", "-J"])), "topology": topology,
    "physical_cpu_selection": cpus[:6], "affinity_policy": "taskset: one logical CPU per WSL-reported physical core; shared Windows/WSL host, not exclusive",
    "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
    "cargo_lock_sha256": hashlib.sha256((here / "Cargo.lock").read_bytes()).hexdigest(),
    "snapshot_manifest": manifest, "build_flags": "release opt-level=3; repository -C target-cpu=native", "uname": list(os.uname())}
save("environment.json", inventory)

def competing_builds():
    names = {"cargo", "rustc", "rust-lld", "ld", "cc1", "cc1plus"}
    found = []
    for proc in Path("/proc").iterdir():
        if not proc.name.isdigit():
            continue
        try:
            comm = (proc / "comm").read_text().strip()
            if comm in names:
                found.append({"pid": int(proc.name), "comm": comm})
        except (FileNotFoundError, PermissionError, ProcessLookupError):
            pass
    return found

def wait_idle():
    consecutive = 0
    while consecutive < 3:
        busy = competing_builds()
        status("waiting_for_idle", competing_builds=busy, quiet_checks=consecutive)
        consecutive = consecutive + 1 if not busy else 0
        time.sleep(5)

def run(stage, case, threads, logs, widths, runs):
    wait_idle()
    cpu_list = ",".join(map(str, cpus[:threads]))
    command = ["taskset", "-c", cpu_list, str(binary), "--logs", logs, "--widths", widths,
        "--runs", str(runs), "--threads", str(threads), "--out", str(results / (stage + ".json"))]
    rec = {"stage": stage, "case": case, "argv": command, "loadavg_start": os.getloadavg(),
           "utc_start": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}
    start = time.monotonic()
    with (results / (stage + ".log")).open("w") as log:
        p = subprocess.Popen(command, cwd=here, env=env, stdout=log, stderr=subprocess.STDOUT)
        status(stage, pid=p.pid, log=stage + ".log")
        rec["competing_build_observations"] = []
        while p.poll() is None:
            busy = competing_builds()
            if busy:
                rec["competing_build_observations"].append({"elapsed_seconds": time.monotonic()-start, "processes": busy})
            time.sleep(2)
        rec.update(returncode=p.returncode, wall_seconds=time.monotonic()-start, loadavg_end=os.getloadavg())
    rec["accepted"] = rec["returncode"] == 0 and not rec["competing_build_observations"]
    commands.append(rec)
    save("commands.json", commands)
    if rec["returncode"]:
        status("failed", failed_stage=stage, returncode=rec["returncode"])
        raise SystemExit(rec["returncode"])
    if rec["competing_build_observations"]:
        status("interference_detected", excluded_stage=stage, next_action="retry_entire_case_after_idle")
    return rec["accepted"]

def run_clean(case, threads, log_n, width, runs=6):
    for attempt in range(1, 6):
        if run(f"{case}-attempt{attempt}", case, threads, str(log_n), str(width), runs):
            return
    status("persistent_interference", case=case)
    raise SystemExit("Five whole-case attempts encountered concurrent builds; all data retained")

run_clean("smoke", 1, 10, 8, 1)
for log_n in (12, 16, 20):
    for width in (1, 8, 32):
        run_clean(f"comparison-1t-n{log_n}-w{width}", 1, log_n, width)
for log_n in (16, 20):
    run_clean(f"comparison-6t-n{log_n}-w8", 6, log_n, 8)
inventory["end_utc"] = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
save("environment.json", inventory)
status("complete")
