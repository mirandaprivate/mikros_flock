#!/usr/bin/env python3
"""Run an already-built, single-thread comparison and preserve its provenance."""
import argparse
import csv
import datetime as dt
import hashlib
import json
import os
from pathlib import Path
import platform
import resource
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--n", default="12,16,20")
    parser.add_argument("--reps", type=int, default=8)
    parser.add_argument("--cpu", type=int, default=2)
    parser.add_argument("--fields", default="Goldilocks,KoalaBear")
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()
    out = args.out.resolve()
    # Existing result directories are never overwritten.
    out.mkdir(parents=True, exist_ok=False)
    if args.cpu not in os.sched_getaffinity(0):
        raise SystemExit(f"CPU {args.cpu} is unavailable")
    binary = ROOT / "target/release/gruen-mikros-bench"
    if not binary.is_file():
        raise SystemExit("Build gruen-mikros-bench --release first")
    files = sorted((ROOT / "src").rglob("*.rs")) + sorted((ROOT / "tests").rglob("*.rs"))
    files += [ROOT / "Cargo.toml", ROOT / "Cargo.lock", Path(__file__).resolve()]
    source_hashes = {str(p.relative_to(ROOT)): sha(p) for p in files}
    command = ["taskset", "-c", str(args.cpu), str(binary), "--n", args.n,
               "--reps", str(args.reps), "--fields", args.fields, "--out", str(out)]
    capture = lambda cmd: subprocess.run(cmd, text=True, capture_output=True, check=True).stdout.strip()
    metadata = {
        "timestamp_utc": dt.datetime.now(dt.timezone.utc).isoformat(),
        "platform": platform.platform(),
        "rustc": capture(["rustc", "+nightly-2026-03-21", "--version", "--verbose"]),
        "cpu": capture(["lscpu"]),
        "process_snapshot_before": capture(["ps", "-eo", "pid,comm,pcpu", "--sort=-pcpu"]),
        "command": command, "binary_sha256": sha(binary), "source_sha256": source_hashes,
        "timing": "single-thread outer wall-clock timers include prover cleanup; all setup included; deterministic tape and witness generation excluded; reduction checks after all four timed provers; three independent dense source oracles precomputed per field/size",
        "order": "one warmup/method; cyclic rotation, eight repetitions balance all four positions twice",
        "backend": "shared scalar Rust u128 modular arithmetic; no Plonky3 SIMD, FLINT or PCS in measured code",
        "field_profiles": {
            "Goldilocks": {"p": "18446744069414584321", "degree": 2, "polynomial": "X^2-7", "basis": "1,X", "bytes": 16},
            "KoalaBear": {"p": "2130706433", "degree": 4, "polynomial": "X^4-3", "basis": "1,X^2,X,X^3", "bytes": 16},
        },
    }
    (out / "metadata.json").write_text(json.dumps(metadata, indent=2) + "\n")
    start = time.perf_counter()
    with (out / "execution.log").open("w") as log:
        status = subprocess.run(command, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT).returncode
    usage = resource.getrusage(resource.RUSAGE_CHILDREN)
    validation = {"exit_code": status, "wall_seconds": time.perf_counter() - start,
                  "whole_run_child_maxrss_kib": usage.ru_maxrss,
                  "rss_scope": "whole benchmark process, not method-specific peak memory",
                  "binary_unchanged": sha(binary) == metadata["binary_sha256"],
                  "source_unchanged": all(sha(ROOT / p) == h for p, h in source_hashes.items())}
    if status == 0:
        raw = list(csv.DictReader((out / "raw.csv").open()))
        summary = list(csv.DictReader((out / "summary.csv").open()))
        expected_groups = len(args.fields.split(",")) * len(args.n.split(",")) * 4
        assert len(raw) == expected_groups * args.reps
        assert len(summary) == expected_groups
        assert all(r["verified"] == "true" and r["mikros_pair_equal"] == "true" for r in raw)
        assert all(float(r["total_ms"]) > 0 for r in raw)
        validation.update(raw_rows=len(raw), summary_rows=len(summary), all_proofs_verified=True,
                          paired_mikros_messages_equal=True,
                          completion=json.loads((out / "completion.json").read_text()))
    validation["passed"] = status == 0 and validation["binary_unchanged"] and validation["source_unchanged"]
    (out / "validation.json").write_text(json.dumps(validation, indent=2) + "\n")
    print(json.dumps(validation, indent=2), flush=True)
    if not validation["passed"]:
        raise SystemExit(status or 1)


if __name__ == "__main__":
    main()
