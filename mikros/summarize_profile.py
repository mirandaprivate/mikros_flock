#!/usr/bin/env python3
"""Validate paired runs and join production Flock timing logs to raw samples."""
import csv
import json
from pathlib import Path
import re
from statistics import mean, median

ROOT = Path(__file__).resolve().parent
RESULTS = ROOT / "results" / "profile"
PHASE = re.compile(r"\[zc-timing\] (round1 URM|round2 fused fold|rounds 3\+ tail): ([0-9.]+) ms")
SAMPLE = re.compile(r"(?:warmup n=2\^(\d+) flock\s*:|n=2\^(\d+) run=(\d+) flock\s*:)")
PHASE_NAMES = {
    "round1 URM": "round1_urm_ms",
    "round2 fused fold": "round2_fused_fold_ms",
    "rounds 3+ tail": "rounds_3_plus_tail_ms",
}


def flock_phases(path):
    found, pending = {}, {}
    for line in path.read_text().splitlines():
        phase = PHASE.search(line)
        if phase:
            name = PHASE_NAMES[phase[1]]
            assert name not in pending, (path, line)
            pending[name] = float(phase[2])
        sample = SAMPLE.search(line)
        if sample:
            assert len(pending) == 3, (path, line, pending)
            key = (int(sample[1] or sample[2]), int(sample[3] or 0))
            assert key not in found
            found[key] = pending
            pending = {}
    assert not pending
    return found


def main():
    rows, joined, total_proofs = [], [], 0
    for threads in (1, 6):
        records = {}
        for mode in ("control", "phases"):
            prefix = "wsl-phases" if mode == "phases" else "control"
            path = RESULTS / f"{prefix}-{threads}t.json"
            report = json.loads(path.read_text())
            assert report["metadata"]["threads"] == threads
            assert report["metadata"]["mikros_phase_profiling"] == (mode == "phases")
            logs = flock_phases(path.with_suffix(".log")) if mode == "phases" else None
            total_proofs += len(report["samples"]) + len(report["warmups"])
            for sample in report["samples"] + report["warmups"]:
                key = (sample["log_n"], sample["run"])
                pair_key = (*key, sample["system"])
                if mode == "control":
                    records[pair_key] = sample["input_digest"]
                else:
                    assert records[pair_key] == sample["input_digest"]
                other = next(s for s in report["samples"] + report["warmups"]
                             if s["log_n"] == key[0] and s["run"] == key[1]
                             and s["system"] != sample["system"])
                assert sample["input_digest"] == other["input_digest"]
                if mode == "phases":
                    phases = dict(sample["mikros_phases"] if sample["system"] == "lookup" else logs[key])
                    # Flock prints hundredths of a millisecond, so allow rounding.
                    residual = sample["prover_ms"] - sum(phases.values())
                    assert residual >= -0.016, (key, residual)
                    phases["residual_ms"] = residual
                    sample["phases"] = phases
                    joined.append({"threads": threads, **sample})
            for log_n in report["metadata"]["logs"]:
                for system in ("lookup", "flock"):
                    selected = [s for s in report["samples"] if s["log_n"] == log_n and s["system"] == system]
                    entry = {
                        "threads": threads, "log_n": log_n, "mode": mode, "system": system,
                        "runs": len(selected),
                        "total_median_ms": median(s["prover_ms"] for s in selected),
                        "total_mean_ms": mean(s["prover_ms"] for s in selected),
                        "total_min_ms": min(s["prover_ms"] for s in selected),
                        "total_max_ms": max(s["prover_ms"] for s in selected),
                    }
                    if mode == "phases":
                        entry["phase_mean_ms"] = {p: mean(s["phases"][p] for s in selected) for p in selected[0]["phases"]}
                        entry["phase_median_ms"] = {p: median(s["phases"][p] for s in selected) for p in selected[0]["phases"]}
                        assert abs(sum(entry["phase_mean_ms"].values()) - entry["total_mean_ms"]) < 1e-8
                    rows.append(entry)
    result = {"verified_proofs_in_raw_reports": total_proofs,
              "phase_statistic": "Mean of 5 warm runs: additive decomposition; warmup excluded. Flock logs rounded to 0.01 ms. Residual includes uncovered work and timing/log overhead.",
              "summaries": rows, "joined_profile_samples_including_warmups": joined}
    (RESULTS / "summary.json").write_text(json.dumps(result, indent=2) + "\n")
    with (RESULTS / "summary.csv").open("w", newline="") as out:
        writer = csv.writer(out)
        writer.writerow(["threads", "log_n", "mode", "system", "total_median_ms", "total_mean_ms", "phase", "phase_mean_ms"])
        for row in rows:
            for name, value in row.get("phase_mean_ms", {"total": row["total_mean_ms"]}).items():
                writer.writerow([row[k] for k in ("threads", "log_n", "mode", "system", "total_median_ms", "total_mean_ms")] + [name, value])
    print(json.dumps({"verified_proofs_in_raw_reports": total_proofs, "summaries": rows}, indent=2))


if __name__ == "__main__":
    main()
