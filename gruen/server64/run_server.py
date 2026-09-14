#!/usr/bin/env python3
"""Run the isolated comparison ON the authorized Linux server; no SSH handling."""
import argparse
from collections import Counter
import csv
import hashlib
import json
import math
import os
from pathlib import Path
import statistics
import subprocess
import time

ROOT = Path(__file__).resolve().parent
TOOLCHAIN = "+nightly-2026-03-21"
LABELS = ["gruen_2eval", "gruen_skip_kappa", "mikros_lagrange_reference", "mikros_lagrange_optimized"]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def source_files():
    files = [ROOT / p for p in ("Cargo.toml", "Cargo.lock", "reference/Cargo.toml", "reference_manifest.json", "run_server.py", "package_source.py")]
    for folder in ("src", "tests", "reference/src", "reference/tests"):
        files += sorted((ROOT / folder).rglob("*.rs"))
    return files


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--out", type=Path, required=True)
    p.add_argument("--n", default="20,26,30")
    p.add_argument("--reps", type=int, default=8)
    p.add_argument("--threads", type=int, default=64)
    p.add_argument("--cpus", default="0-47,48-55,72-79")
    p.add_argument("--skip-build", action="store_true")
    p.add_argument("--cold-rss", action="store_true")
    p.add_argument("--wait-seconds", type=int, default=1800)
    args = p.parse_args()
    ns = [int(n) for n in args.n.split(",")]
    assert ns and len(set(ns)) == len(ns) and all(2 <= n <= 30 for n in ns)
    assert args.reps >= 4 and args.reps % 4 == 0, "balance every system position"
    cpus = []
    for part in args.cpus.split(","):
        edges = part.split("-")
        cpus += list(range(int(edges[0]), int(edges[-1]) + 1))
    assert len(cpus) == len(set(cpus)) == args.threads
    assert set(cpus) <= os.sched_getaffinity(0), "requested CPUs unavailable"
    out = args.out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    env = os.environ.copy()
    env["RUSTFLAGS"] = "-C target-cpu=native"
    env["CARGO_TERM_WIDTH"] = "200"
    env["LC_ALL"] = "C"
    env["RAYON_NUM_THREADS"] = str(args.threads)
    for name in ("CARGO_ENCODED_RUSTFLAGS", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER"):
        env.pop(name, None)

    def save(name, obj):
        (out / name).write_text(json.dumps(obj, indent=2) + "\n")

    def capture(argv):
        return subprocess.check_output(argv, cwd=ROOT, env=env, text=True).strip()

    def memory():
        return {line.split(":")[0]: int(line.split()[1]) * 1024
                for line in Path("/proc/meminfo").read_text().splitlines()
                if line.split(":")[0] in ("MemTotal", "MemAvailable", "SwapFree", "SwapTotal")}

    def cgroups():
        result = {"membership": Path("/proc/self/cgroup").read_text(), "visible_ancestors": []}
        memberships = [s[3:] for s in result["membership"].splitlines() if s.startswith("0::")]
        if len(memberships) == 1:
            base = Path("/sys/fs/cgroup").resolve()
            leaf = (base / memberships[0].lstrip("/")).resolve()
            if leaf.is_relative_to(base):
                while leaf.is_relative_to(base):
                    record = {"path": str(leaf)}
                    for key in ("cpu.max", "cpu.stat", "memory.max", "memory.current", "memory.events", "cpuset.cpus.effective", "cpuset.mems.effective"):
                        f = leaf / key
                        if f.is_file():
                            record[key] = f.read_text().strip()
                    result["visible_ancestors"].append(record)
                    if leaf == base:
                        break
                    leaf = leaf.parent
        return result

    hashes = {str(f.relative_to(ROOT)): sha(f) for f in source_files()}
    package_manifest = ROOT / "source_manifest.json"
    if package_manifest.is_file():
        for name, expected in json.loads(package_manifest.read_text()).items():
            f = (ROOT / name).resolve()
            assert f.is_relative_to(ROOT) and sha(f) == expected, "uploaded source manifest mismatch"
    for name, expected in json.loads((ROOT / "reference_manifest.json").read_text(encoding="utf-8-sig")).items():
        assert sha(ROOT / "reference" / name) == expected, "frozen reference modified"
    topology = capture(["lscpu", "-p=CPU,CORE,SOCKET,NODE"])
    selected = [[int(x) if x else -1 for x in row.split(",")] for row in topology.splitlines()
                if not row.startswith("#") and int(row.split(",")[0]) in cpus]
    physical = len({(r[2], r[1]) for r in selected})
    inventory = {"utc_start": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
                 "hostname": os.uname().nodename, "uname": list(os.uname()),
                 "lscpu": json.loads(capture(["lscpu", "-J"])), "topology": topology,
                 "selected_cpus": cpus, "physical_cores": physical, "extra_smt_threads": len(cpus)-physical,
                 "threads": args.threads, "rustflags": env["RUSTFLAGS"],
                 "rustc": capture(["rustc", TOOLCHAIN, "-Vv"]), "source_sha256": hashes,
                 "memory_start": memory(), "cgroups_start": cgroups(),
                 "scope": "same scalar field backend, parallel PIOP, no PCS; all online setup/allocation/release included; dense source oracle separately timed; first-touch NUMA, affinity not exclusivity"}
    save("environment.json", inventory)
    commands = []

    def busy(exclude=()):
        names = {"cargo", "rustc", "rust-lld", "compare", "hadamard_vs_flo", "packed_compare", "univariate_vs_f", "gruen-mikros-be"}
        found = []
        for path in Path("/proc").iterdir():
            if not path.name.isdigit() or int(path.name) in exclude:
                continue
            try:
                comm = (path / "comm").read_text().strip()
                state = (path / "stat").read_text().rsplit(")", 1)[1].split()[0]
                if comm in names and state != "Z":
                    found.append({"pid": int(path.name), "comm": comm})
            except (FileNotFoundError, PermissionError, ProcessLookupError):
                pass
        return found

    def idle(min_memory=0):
        deadline = time.monotonic() + args.wait_seconds
        while busy() or memory()["MemAvailable"] < min_memory:
            save("status.json", {"stage": "waiting", "busy": busy(), "memory": memory()})
            if time.monotonic() >= deadline:
                raise RuntimeError("server busy or memory unavailable; no other jobs stopped")
            time.sleep(10)
        # Visible memory cgroup limit must also fit the conservative reservation.
        for record in cgroups()["visible_ancestors"]:
            limit = record.get("memory.max", "max")
            if limit != "max":
                assert int(limit)-int(record.get("memory.current", "0")) >= min_memory, "cgroup memory budget"

    def execute(stage, argv, monitor=False):
        save("status.json", {"stage": stage})
        record = {"stage": stage, "argv": argv, "start": time.time(), "load_start": os.getloadavg(), "cgroup_start": cgroups(), "interference": []}
        with (out / (stage + ".log")).open("w") as log:
            proc = subprocess.Popen(argv, cwd=ROOT, env=env, stdout=log, stderr=subprocess.STDOUT)
            while proc.poll() is None:
                if monitor:
                    # /usr/bin/time has the benchmark as a child. Exclude only
                    # this command's actual process subtree, not other jobs.
                    owned = {proc.pid}
                    pending = [proc.pid]
                    while pending:
                        pid = pending.pop()
                        child_file = Path(f"/proc/{pid}/task/{pid}/children")
                        try:
                            children = [int(x) for x in child_file.read_text().split()]
                        except (FileNotFoundError, PermissionError, ProcessLookupError):
                            children = []
                        for child in children:
                            if child not in owned:
                                owned.add(child)
                                pending.append(child)
                    active = busy(owned)
                    if active:
                        record["interference"].append({"time": time.time(), "processes": active})
                time.sleep(2)
        record.update(returncode=proc.returncode, end=time.time(), load_end=os.getloadavg(), cgroup_end=cgroups())
        commands.append(record)
        save("commands.json", commands)
        assert proc.returncode == 0, f"{stage} failed; see retained log"
        return record

    idle()
    if not args.skip_build:
        execute("fmt", ["cargo", TOOLCHAIN, "fmt", "--all", "--", "--check"])
        execute("tests", ["cargo", TOOLCHAIN, "test", "--release", "--locked", "--offline", "-j", "8", "--", "--test-threads=1"])
        execute("clippy", ["cargo", TOOLCHAIN, "clippy", "--release", "--locked", "--offline", "-j", "8", "--all-targets", "--no-deps", "--", "-D", "warnings"])
        execute("build", ["cargo", TOOLCHAIN, "build", "--release", "--locked", "--offline", "-j", "8", "--bin", "compare"])
    binary = ROOT / "target/release/compare"
    inventory["binary_sha256"] = sha(binary)
    inventory["build_tests_this_run"] = not args.skip_build
    save("environment.json", inventory)
    accepted = []

    def validate_case(folder, n, reps, systems, warmups):
        raw = list(csv.DictReader((folder / "raw.csv").open()))
        summary = list(csv.DictReader((folder / "summary.csv").open()))
        assert len(raw) == reps*len(systems) and len(summary) == len(systems)
        assert all(r["verified"] == "true" and int(r["threads"]) == args.threads and int(r["n"]) == n for r in raw)
        completion = json.loads((folder / "completion.json").read_text())
        assert completion["verified_proofs_including_warmups"] == len(systems)*(reps+warmups)
        for s in summary:
            group = [r for r in raw if r["method"] == s["method"]]
            times = [float(r["total_ms"]) for r in group]
            assert all(math.isfinite(t) and t > 0 for t in times)
            assert math.isclose(statistics.median(times), float(s["median_ms"]), abs_tol=1e-5)
            if len(systems) == 4 and reps % 4 == 0:
                assert Counter(int(r["order"]) for r in group) == Counter({i: reps//4 for i in range(4)})
        proofs = list(folder.glob("proof-*.txt"))
        assert len(proofs) == len(raw)
        if 2 in systems and 3 in systems:
            assert all(r["mikros_pair_equal"] == "true" for r in raw)
            for f in folder.glob("proof-*-s2.txt"):
                assert f.read_bytes() == f.with_name(f.name.replace("-s2.txt", "-s3.txt")).read_bytes()
        return completion

    def case(name, field, n, reps, systems, warmups):
        for attempt in range(1, 4):
            idle(160*2**30 if n == 30 else 0)
            stage = f"{name}-attempt{attempt}"
            folder = out / stage
            argv = ["taskset", "-c", args.cpus, str(binary), "--n", str(n), "--fields", field,
                    "--threads", str(args.threads), "--reps", str(reps), "--systems", ",".join(map(str, systems)),
                    "--warmups", str(warmups), "--out", str(folder)]
            if len(systems) == 1:
                argv = ["/usr/bin/time", "-v", "-o", str(out / (stage+"-rss.txt")), *argv]
            rec = execute(stage, argv, monitor=True)
            completion = validate_case(folder, n, reps, systems, warmups)
            rec["accepted"] = not rec["interference"]
            save("commands.json", commands)
            if rec["accepted"]:
                accepted.append({"name": name, "folder": folder.name, "validation": completion})
                save("accepted.json", accepted)
                return
        raise RuntimeError("repeated interference; all attempts retained and excluded")

    # Only one benchmark process runs at a time. Interference exclusion depends
    # on concurrent process evidence, never on whether a duration looks slow.
    for n in ns:
        for field in ("Goldilocks", "KoalaBear"):
            case(f"{field}-n{n}-{args.threads}t", field, n, args.reps, [0,1,2,3], 1)
            if args.cold_rss and n == max(ns):
                for system in range(4):
                    case(f"rss-{field}-n{n}-s{system}", field, n, 1, [system], 0)
    assert all(sha(ROOT/name) == digest for name, digest in hashes.items()), "source changed during run"
    assert sha(binary) == inventory["binary_sha256"], "binary changed during run"
    inventory.update(memory_end=memory(), cgroups_end=cgroups(), utc_end=time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()))
    save("environment.json", inventory)
    save("status.json", {"stage": "complete", "accepted_cases": len(accepted), "source_and_binary_unchanged": True})


if __name__ == "__main__":
    main()
