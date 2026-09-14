#!/usr/bin/env python3
"""Legacy-permutation gather ablations on the authorized 64-CPU server.

For the current packed/legacy/Flock comparison use run_packed_permutation_64.py.
The explicit legacy flag keeps these historical gather ablations reproducible
after the E2E executable's default permutation changes to packed.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess
import time

p = argparse.ArgumentParser()
p.add_argument("--logs", default="results-e2e")
p.add_argument("--log-leaves", default="6,10,14")
p.add_argument("--reps", type=int, default=3)
args = p.parse_args()
root = Path.cwd().resolve()
logs = (root / args.logs).resolve()
assert logs.is_relative_to(root)
logs.mkdir(exist_ok=False)
CPU_SPEC = "0-47,48-55,72-79"
CPUS = set(range(56)) | set(range(72, 80))
assert len(CPUS) == 64
TOOLCHAIN = "+nightly-2026-03-21"
env = os.environ.copy()
for key in list(env):
    if key.startswith("FLOCK_") or key in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTC_WRAPPER", "CARGO_BUILD_RUSTFLAGS"):
        env.pop(key)
env.update(RUSTFLAGS="-C target-cpu=native", RAYON_NUM_THREADS="64", CARGO_TARGET_DIR=str(root / "target"))
env["PATH"] = str(Path.home() / ".cargo/bin") + ":" + env.get("PATH", "")
status = {"state": "starting", "completed": [], "failed": [], "started_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}

def save(name, data):
    tmp = logs / (name + ".tmp")
    tmp.write_text(json.dumps(data, indent=2) + "\n")
    tmp.replace(logs / name)

def verify_sources():
    manifest = json.loads((root / "source_manifest.json").read_text())
    for name, expected in manifest["files_sha256"].items():
        target = (root / name).resolve()
        assert target.is_relative_to(root) and target.is_file(), name
        assert hashlib.sha256(target.read_bytes()).hexdigest() == expected, name

def resource_snapshot(name):
    def cpu():
        result = {}
        for line in Path("/proc/stat").read_text().splitlines():
            fields = line.split()
            if fields[0].startswith("cpu") and fields[0][3:].isdigit() and int(fields[0][3:]) in CPUS:
                values = list(map(int, fields[1:9]))
                result[fields[0]] = (sum(values), values[3] + values[4])
        return result
    assert CPUS.issubset(os.sched_getaffinity(0)), "requested CPUs unavailable"
    before = cpu()
    time.sleep(1)
    after = cpu()
    total = sum(after[c][0] - before[c][0] for c in before)
    idle = sum(after[c][1] - before[c][1] for c in before)
    busy = 1 - idle / max(total, 1)
    available = int(next(line.split()[1] for line in Path("/proc/meminfo").read_text().splitlines() if line.startswith("MemAvailable:")))
    save(name + ".resources-before.json", {"selected_cpu_busy_fraction": busy, "mem_available_kib": available,
                                          "loadavg": os.getloadavg(), "allowed_affinity": sorted(os.sched_getaffinity(0))})
    assert busy < 0.10, f"selected CPUs busy before {name}: {busy:.1%}"
    assert available > 64 * 1024 * 1024, "less than 64 GiB available"

def run(name, command, pinned=False):
    status["state"] = name
    save("status.json", status)
    if pinned:
        command = ["/usr/bin/time", "-v", "-o", str(logs / (name + ".resource.txt")),
                   "taskset", "-c", CPU_SPEC] + command
    save(name + ".command.json", command)
    start = time.monotonic()
    with (logs / (name + ".stdout")).open("w") as stdout, (logs / (name + ".stderr")).open("w") as stderr:
        result = subprocess.run(command, cwd=root, env=env, stdout=stdout, stderr=stderr)
    event = {"name": name, "exit_code": result.returncode, "wall_seconds": time.monotonic() - start}
    status["completed" if result.returncode == 0 else "failed"].append(event)
    save("status.json", status)
    print(json.dumps(event), flush=True)
    if result.returncode:
        raise RuntimeError(f"{name} failed; all output retained")

try:
    verify_sources()
    hardware = {"utc": status["started_utc"], "hostname": os.uname().nodename,
                "cpu_affinity_spec": CPU_SPEC, "selected_cpus": sorted(CPUS),
                "cpuinfo": Path("/proc/cpuinfo").read_text(), "meminfo": Path("/proc/meminfo").read_text(),
                "cgroup": Path("/proc/self/cgroup").read_text(), "rustflags": env["RUSTFLAGS"],
                "rayon_num_threads": env["RAYON_NUM_THREADS"]}
    save("hardware.json", hardware)
    resource_snapshot("initial")
    run("rustc", ["rustc", TOOLCHAIN, "-Vv"])
    run("build", ["cargo", TOOLCHAIN, "build", "--release", "--offline", "--locked", "-j8", "-p", "mikros",
                  "--example", "e2e_bit_compare", "--example", "e2e_flock_baseline"])
    run("test-pcs", ["cargo", TOOLCHAIN, "test", "--release", "--offline", "--locked", "-j8", "-p", "flock-core",
                     "pcs::functional::tests", "--", "--include-ignored"], True)
    run("test-merged-tensor", ["cargo", TOOLCHAIN, "test", "--release", "--offline", "--locked", "-j8", "-p", "flock-core",
                               "pcs::merged_tensor::tests", "--", "--include-ignored"], True)
    run("test-weighted-pcs", ["cargo", TOOLCHAIN, "test", "--release", "--offline", "--locked", "-j8", "-p", "flock-core",
                              "weighted", "--", "--include-ignored"], True)
    run("test-permutation", ["cargo", TOOLCHAIN, "test", "--release", "--offline", "--locked", "-j8", "-p", "mikros-permutation"], True)
    run("test-e2e-helpers", ["cargo", TOOLCHAIN, "test", "--release", "--offline", "--locked", "-j8", "-p", "mikros",
                             "--example", "e2e_bit_compare"], True)
    run("test-packed-ip", ["cargo", TOOLCHAIN, "test", "--release", "--offline", "--locked", "-j8", "-p", "mikros",
                           "--test", "packed_ip_roundtrip"], True)
    run("test-mikros-lincheck", ["cargo", TOOLCHAIN, "test", "--release", "--offline", "--locked", "-j8", "-p", "mikros",
                                 "--lib", "e2e_lincheck"], True)
    for test in ("flock_skip7_interpolation", "univariate_roundtrip"):
        run("test-" + test, ["cargo", TOOLCHAIN, "test", "--release", "--offline", "--locked", "-j8", "-p", "mikros",
                             "--test", test], True)
    binary = str(root / "target/release/examples/e2e_bit_compare")
    production = str(root / "target/release/examples/e2e_flock_baseline")
    binary_hashes = {name: hashlib.sha256(Path(name).read_bytes()).hexdigest() for name in (binary, production)}
    save("binary-sha256.json", binary_hashes)
    run("validation-m22", [binary, "--permutation", "legacy", "--log-leaves", "3", "--threads", "64", "--reps", "1", "--warmups", "0", "--tamper"], True)
    for mode in ("legacy", "cube"):
        run("validation-" + mode, [binary, "--permutation", "legacy", "--log-leaves", "3", "--threads", "64", "--reps", "1",
                                  "--warmups", "0", "--reduction", mode, "--tamper"], True)
    for leaves in [int(x) for x in args.log_leaves.split(",")]:
        resource_snapshot(f"common-leaves{leaves}")
        run(f"common-leaves{leaves}", [binary, "--permutation", "legacy", "--log-leaves", str(leaves), "--threads", "64", "--reps", str(args.reps), "--warmups", "1"], True)
        resource_snapshot(f"production-leaves{leaves}")
        run(f"production-leaves{leaves}", [production, "--log-leaves", str(leaves), "--threads", "64", "--reps", str(args.reps), "--warmups", "1", "--profile", "fast"], True)
    largest = max(int(x) for x in args.log_leaves.split(","))
    for mode in ("legacy", "cube"):
        resource_snapshot("ablation-" + mode)
        run("ablation-" + mode, [binary, "--permutation", "legacy", "--log-leaves", str(largest), "--threads", "64",
                                "--reps", str(args.reps), "--warmups", "1", "--reduction", mode], True)
    verify_sources()
    for name, expected in binary_hashes.items():
        assert hashlib.sha256(Path(name).read_bytes()).hexdigest() == expected
    summary = []
    ablation = []
    for file in sorted(list(logs.glob("common-leaves*.stdout")) + list(logs.glob("ablation-*.stdout"))):
        records = [json.loads(line) for line in file.read_text().splitlines() if line.strip()]
        metadata = next(x for x in records if x["record"] == "metadata")
        assert metadata["threads"] == metadata["all_core_threads"] == 64
        assert metadata["compression_kappa"] == 128 and metadata["flock_skip"] == 7
        assert metadata["pcs_path"] == "flock-production-dense-merged"
        assert metadata["schema_version"] in (3, 4)
        assert metadata.get("permutation", "legacy") == "legacy"
        mode = metadata["reduction"]
        assert mode in ("legacy", "cube", "lockstep")
        assert metadata["gather_audit_bits"] == (0 if mode == "legacy" else 2)
        assert metadata["gather_audit_degree"] == (0 if mode == "legacy" else 3)
        assert {"avx512f", "avx512bw", "gfni", "vpclmulqdq"}.issubset(metadata["target_features"])
        samples = [x for x in records if x["record"] == "sample" and not x["warmup"]]
        assert len(samples) == 2 * args.reps and all(x["verified"] and x["transcript_match"] for x in samples)
        assert all(x["pcs_config_match"] and x["timings"]["pcs_inner_openings"] == 1 for x in samples)
        expected = {"legacy":{"flock":11,"mikros":20}, "cube":{"flock":4,"mikros":6},
                    "lockstep":{"flock":4,"mikros":5}}[mode]
        assert all(x["timings"]["tensor_claims"] == x["timings"]["pcs_ring_claims"] == expected[x["system"]] for x in samples)
        assert len({x["commitment_blake3"] for x in samples}) == 1
        production_records = [json.loads(line) for line in
                              (logs / f'production-leaves{metadata["log_leaves"]}.stdout').read_text().splitlines() if line.strip()]
        production_metadata = next(x for x in production_records if x["record"] == "metadata")
        for key in ("pcs_params", "pcs_config_blake3", "pcs_queries", "pcs_query_grinding_bits", "pcs_fold_grinding_bits", "native_root_u32"):
            assert metadata[key] == production_metadata[key], key
        production_samples = [x for x in production_records if x["record"] == "sample"]
        assert all(x["commitment_blake3"] == samples[0]["commitment_blake3"] for x in production_samples)
        row = {"logical_bit_log": metadata["logical_bit_log"], "wiring_bit_log": metadata["wiring_bit_log"],
               "sha_compressions": metadata["sha_compressions"], "log_leaves": metadata["log_leaves"],
               "pcs_params":metadata["pcs_params"], "pcs_path":metadata["pcs_path"], "reduction":mode,
               "same_production_pcs_config":True, "same_commitment":True,
               "commitment_blake3":samples[0]["commitment_blake3"]}
        for system in ("flock", "mikros"):
            group = [x for x in samples if x["system"] == system]
            assert len(group) == args.reps and sorted(x["iteration"] for x in group) == list(range(1,args.reps+1))
            assert len({x["proof_blake3"] for x in group}) == 1
            row[system] = {key: statistics.median(x["timings"][key] for x in group)
                           for key in ("prover_ms", "commit_ms", "boolean_ms", "wiring_ms", "parallel_piops_ms", "pcs_open_ms")}
            row[system].update(proof_bytes=group[0]["proof_bytes"], verify_ms=statistics.median(x["verify_ms"] for x in group),
                               tensor_claims=group[0]["timings"]["tensor_claims"],
                               pcs_ring_claims=group[0]["timings"]["pcs_ring_claims"],
                               pcs_inner_openings=group[0]["timings"]["pcs_inner_openings"])
            row[system]["prover_range_ms"] = [min(x["timings"]["prover_ms"] for x in group), max(x["timings"]["prover_ms"] for x in group)]
        row["mikros_over_flock"] = row["mikros"]["prover_ms"] / row["flock"]["prover_ms"]
        (summary if file.name.startswith("common-") else ablation).append(row)
    save("summary.json", summary)
    save("ablation-summary.json", ablation)
    production_summary = []
    for file in sorted(logs.glob("production-leaves*.stdout")):
        records = [json.loads(line) for line in file.read_text().splitlines() if line.strip()]
        metadata = next(x for x in records if x["record"] == "metadata")
        assert metadata["threads"] == metadata["all_core_threads"] == 64
        samples = [x for x in records if x["record"] == "sample" and not x["warmup"]]
        assert len(samples) == args.reps and sorted(x["iteration"] for x in samples) == list(range(1,args.reps+1))
        assert all(x["verified"] and x["transcript_match"] for x in samples)
        assert len({x["proof_blake3"] for x in samples}) == 1
        production_summary.append({"log_leaves":metadata["log_leaves"], "gate_count":metadata["gate_count"],
                                   "prove_with_commit_ms":statistics.median(x["prove_with_commit_ms"] for x in samples),
                                   "proof_bytes":samples[0]["proof_bytes"],
                                   "timing_boundary":metadata["timing_boundary"]})
    save("production-summary.json", production_summary)
    status["state"] = "complete"
except Exception as error:
    status["state"] = "failed"
    status["error"] = str(error)
    raise
finally:
    status["finished_utc"] = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
    save("status.json", status)
