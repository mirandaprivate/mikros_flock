#!/usr/bin/env python3
"""Verified independent SHA-256 compression comparison on the 64-thread server."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import statistics
import subprocess
import time

CPU_SPEC = "0-47,48-55,72-79"
CPUS = set(range(56)) | set(range(72, 80))
TOOLCHAIN = "+nightly-2026-03-21"
FEATURES = {"avx512f", "avx512bw", "gfni", "vpclmulqdq"}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def save(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n")


def records(path):
    rows = [json.loads(line) for line in path.read_text().splitlines() if line.strip()]
    return next(row for row in rows if row["record"] == "metadata"), rows


def preflight():
    def cpu():
        return {int(s[0][3:]): (sum(map(int, s[1:9])), int(s[4]) + int(s[5]))
                for line in Path("/proc/stat").read_text().splitlines()
                if (s := line.split())[0].startswith("cpu") and s[0][3:].isdigit()
                and int(s[0][3:]) in CPUS}
    assert CPUS.issubset(os.sched_getaffinity(0))
    before = cpu()
    time.sleep(1)
    after = cpu()
    total = sum(after[c][0] - before[c][0] for c in CPUS)
    idle = sum(after[c][1] - before[c][1] for c in CPUS)
    busy = 1 - idle / max(total, 1)
    available = int(next(s.split()[1] for s in Path("/proc/meminfo").read_text().splitlines()
                         if s.startswith("MemAvailable:")))
    assert busy < 0.10, f"selected CPUs busy: {busy:.1%}"
    assert available > 8 * 1024 * 1024
    return {"selected_cpu_busy_fraction": busy, "mem_available_kib": available,
            "loadavg": os.getloadavg()}


def audit(logs, batches, reps):
    common, production, common_meta, production_meta = [], [], [], []
    for batch in range(batches):
        for kind, target, meta_target in (("common", common, common_meta),
                                           ("production", production, production_meta)):
            meta, rows = records(logs / f"{kind}-{batch}.jsonl")
            assert meta["threads"] == meta["all_core_threads"] == 64
            assert FEATURES.issubset(meta["target_features"])
            assert meta.get("sha_compressions", meta.get("gate_count")) == 128
            assert meta["fixture"] == "sha256-independent-compressions"
            meta_target.append(meta)
            all_samples = [row for row in rows if row["record"] == "sample"]
            assert len(all_samples) == (reps + 1) * (2 if kind == "common" else 1)
            assert all(row["verified"] and row["transcript_match"] for row in all_samples)
            group = [row for row in all_samples if not row["warmup"]]
            assert len(group) == reps * (2 if kind == "common" else 1)
            assert all(row["verified"] and row["transcript_match"] for row in group)
            target.extend(group)
    for meta in common_meta + production_meta:
        for key in ("fixture_digest_blake3", "circuit_digest_blake3", "pcs_params", "pcs_config_blake3", "pcs_queries",
                    "pcs_query_grinding_bits", "pcs_fold_grinding_bits"):
            assert meta[key] == common_meta[0][key], key
    assert len({row["commitment_blake3"] for row in common + production}) == 1
    assert all(row["pcs_config_match"] and row["timings"]["pcs_inner_openings"] == 1 for row in common)
    assert common_meta[0]["compression_kappa"] == 128 and common_meta[0]["flock_skip"] == 7
    summary = {"compressions": 128, "threads": 64, "batches": batches, "reps_per_batch": reps,
               "common_metadata": common_meta, "production_metadata": production_meta,
               "same_commitment": True, "same_pcs": True, "verified": True,
               "zero_knowledge_claim": False,
               "workload": "128 independent compression calls, fixed SHA256 IV, public 512-bit blocks and public 256-bit outputs; no message padding",
               "comparison_scope": "Common bit wiring is the controlled comparison; native production uses word wiring and includes assembly/compaction."}
    for system in ("flock", "mikros_packed", "flock-production"):
        group = production if system == "flock-production" else [r for r in common if r["system"] == system]
        assert len(group) == batches * reps
        assert len({r["proof_blake3"] for r in group}) == len({r["proof_bytes"] for r in group}) == 1
        keys = ("prove_with_commit_ms",) if system == "flock-production" else (
            "prover_ms", "commit_ms", "boolean_ms", "wiring_ms", "parallel_piops_ms", "pcs_open_ms")
        times = group if system == "flock-production" else [r["timings"] for r in group]
        row = {k: statistics.median(r[k] for r in times) for k in keys}
        total = keys[0]
        row.update(prover_range_ms=[min(r[total] for r in times), max(r[total] for r in times)],
                   samples=len(group), verify_ms=statistics.median(r["verify_ms"] for r in group),
                   proof_bytes=group[0]["proof_bytes"], proof_blake3=group[0]["proof_blake3"],
                   compressions_per_second=128000 / row[total])
        summary[system] = row
    summary["flock_bit_over_mikros"] = summary["flock"]["prover_ms"] / summary["mikros_packed"]["prover_ms"]
    summary["flock_production_over_mikros"] = summary["flock-production"]["prove_with_commit_ms"] / summary["mikros_packed"]["prover_ms"]
    _, validation = records(logs / "validation.jsonl")
    checked = [r for r in validation if r["record"] == "sample"]
    assert len(checked) == 2
    assert all(r["verified"] and r["transcript_match"] and r["pcs_config_match"] for r in checked)
    assert {r["commitment_blake3"] for r in checked} == {common[0]["commitment_blake3"]}
    negatives = [r for r in validation if r["record"] == "negative_tests"]
    assert {r["system"] for r in negatives} == {"flock", "mikros_packed"}
    assert all(r["rejected"] > 0 for r in negatives)
    summary["negative_tests"] = negatives
    save(logs / "summary.json", summary)
    return summary


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--logs", default="results-sha128")
    p.add_argument("--target", required=True)
    p.add_argument("--batches", type=int, default=3)
    p.add_argument("--reps", type=int, default=6)
    args = p.parse_args()
    root = Path.cwd().resolve()
    logs = (root / args.logs).resolve()
    assert logs.is_relative_to(root)
    logs.mkdir(exist_ok=False)
    target = Path(args.target).resolve()
    env = {k: v for k, v in os.environ.items() if not k.startswith(("FLOCK_", "MIKROS_", "E2E_", "LIG_"))
           and k not in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTC_WRAPPER", "PCS_TRACE", "LD_PRELOAD")}
    env.update(RUSTFLAGS="-C target-cpu=native", RAYON_NUM_THREADS="64", CARGO_TARGET_DIR=str(target))
    env["PATH"] = str(Path.home() / ".cargo/bin") + ":" + env.get("PATH", "")
    manifest = json.loads((root / "source_manifest.json").read_text())
    status = {"state": "starting", "completed": []}

    def verify_sources():
        for name, expected in manifest["files_sha256"].items():
            assert digest(root / name) == expected, name

    def run(name, command, pinned=True):
        status["state"] = name
        save(logs / "status.json", status)
        if pinned:
            command = ["/usr/bin/time", "-v", "-o", str(logs / f"{name}.resource.txt"), "taskset", "-c", CPU_SPEC] + command
        save(logs / f"{name}.command.json", command)
        start = time.monotonic()
        with (logs / f"{name}.jsonl").open("w") as out, (logs / f"{name}.stderr.log").open("w") as err:
            result = subprocess.run(command, env=env, stdout=out, stderr=err, cwd=root)
        event = {"name": name, "exit_code": result.returncode, "wall_seconds": time.monotonic() - start}
        status["completed"].append(event)
        save(logs / "status.json", status)
        print(json.dumps(event), flush=True)
        assert result.returncode == 0, event

    try:
        verify_sources()
        save(logs / "hardware.json", {"cpuinfo": Path("/proc/cpuinfo").read_text(),
             "meminfo": Path("/proc/meminfo").read_text(), "cpu_affinity": CPU_SPEC,
             "utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), "environment": {
                 k: env[k] for k in ("RUSTFLAGS", "RAYON_NUM_THREADS", "CARGO_TARGET_DIR")}})
        save(logs / "preflight.json", preflight())
        for name in manifest["files_sha256"]:
            os.utime(root / name, None)
        run("rustc", ["rustc", TOOLCHAIN, "-Vv"], False)
        cargo = ["cargo", TOOLCHAIN]
        flags = ["--release", "--offline", "--locked", "-j8", "-p", "mikros"]
        run("build", cargo + ["build"] + flags + ["--example", "e2e_bit_compare", "--example", "e2e_flock_baseline"], False)
        run("tests", cargo + ["test"] + flags + ["--example", "e2e_bit_compare", "--", "--test-threads=1"])
        common = str(target / "release/examples/e2e_bit_compare")
        production = str(target / "release/examples/e2e_flock_baseline")
        binaries = {s: digest(Path(s)) for s in (common, production)}
        save(logs / "binary-sha256.json", binaries)
        base = ["--compressions", "128", "--threads", "64"]
        common_flags = ["--system", "both", "--permutation", "packed", "--reduction", "lockstep"]
        run("validation", [common] + base + common_flags + ["--reps", "1", "--warmups", "0", "--tamper"])
        for batch in range(args.batches):
            order = [("common", common), ("production", production)]
            if batch % 2:
                order.reverse()
            for kind, binary in order:
                save(logs / f"{kind}-{batch}.preflight.json", preflight())
                extra = common_flags if kind == "common" else ["--profile", "fast"]
                run(f"{kind}-{batch}", [binary] + base + extra + ["--reps", str(args.reps), "--warmups", "1"])
        audit(logs, args.batches, args.reps)
        verify_sources()
        assert all(digest(Path(name)) == value for name, value in binaries.items())
        status["state"] = "complete"
    except Exception as error:
        status.update(state="failed", error=str(error))
        raise
    finally:
        save(logs / "status.json", status)
        save(logs / "evidence_sha256.json", {p.name: digest(p) for p in sorted(logs.iterdir())
                                            if p.is_file() and p.name != "evidence_sha256.json"})


if __name__ == "__main__":
    main()
