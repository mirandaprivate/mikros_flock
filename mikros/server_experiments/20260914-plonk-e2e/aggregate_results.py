#!/usr/bin/env python3
"""Validate actual per-process JSONL and report complete or explicitly missing cases."""
import argparse
import json
import math
from pathlib import Path
import statistics

from common import MODES, SYSTEMS, NATIVE_FEATURES, atomic_text, cases, require, save, sha256, utc

TIMING_KEYS = ("prover_ms", "commit_ms", "recommit_ms", "public_recommit_ms", "gates_ms",
               "copy_ms", "release_ms", "witness_pcs_open_ms", "public_pcs_open_ms", "serialization_ms")
PCS_PATH = "FLOCK native commit + merged_tensor/open_batch_merged + Ligerito Fast"


def read_jsonl(path):
    records = []
    for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
        if not line.strip():
            continue
        try:
            record = json.loads(line)
        except json.JSONDecodeError as error:
            raise RuntimeError(f"{path.name}:{number}: malformed JSONL") from error
        require(isinstance(record, dict), f"{path.name}:{number}: expected object")
        records.append(record)
    return records


def time_resources(path):
    result = {"raw_path": path.name}
    if not path.exists():
        result["available"] = False
        return result
    labels = {
        "Maximum resident set size (kbytes)": "max_rss_kib",
        "Major (requiring I/O) page faults": "major_page_faults",
        "Minor (reclaiming a frame) page faults": "minor_page_faults",
        "Swaps": "swaps", "Exit status": "time_exit_status",
        "File system inputs": "filesystem_inputs", "File system outputs": "filesystem_outputs",
    }
    for line in path.read_text().splitlines():
        key, separator, value = line.strip().rpartition(": ")
        if separator and key in labels:
            result[labels[key]] = int(value.strip())
    result["available"] = "max_rss_kib" in result
    if result["available"]:
        result["max_rss_gib"] = result["max_rss_kib"] / (1 << 20)
    return result


def validate_case(logs, case, repetitions, seed):
    config = json.loads((logs / "config.json").read_text())
    hash_keys = ["proof_blake3", "witness_commitment_blake3", "public_commitment_blake3"]
    if config.get("piop_fingerprints", False):
        hash_keys.extend(("piop_blake3", "terminal_claims_blake3"))
    records = read_jsonl(logs / f"{case['id']}.stdout.jsonl")
    metadata_rows = [r for r in records if r.get("type") == "metadata"]
    require(len(metadata_rows) == 1, "expected exactly one metadata record")
    metadata = metadata_rows[0]
    for key in ("mode", "system", "log_gates", "instances"):
        require(metadata.get(key) == case[key], f"metadata {key}: expected {case[key]!r}, got {metadata.get(key)!r}")
    require(metadata.get("threads") == metadata.get("all_core_threads") == 64 and metadata.get("seed") == seed,
            "wrong Rayon/all-core thread count or seed")
    require(all(metadata.get("target_features", {}).get(f) is True for f in NATIVE_FEATURES),
            "binary metadata lacks native features")
    require(metadata.get("repetitions") == repetitions, "wrong repetition count")
    require(not any(r.get("type") == "error" for r in records), "driver emitted an error record")
    rows = [r for r in records if r.get("type") == "sample"]
    require(len(rows) == repetitions, f"expected {repetitions} measured samples, got {len(rows)}")
    for row in rows:
        for key in ("mode", "system", "log_gates", "gates_per_instance", "instances", "total_gates"):
            require(row.get(key) == case[key], f"sample {key} mismatch")
        require(row.get("threads") == 64 and row.get("seed") == seed, "sample thread/seed mismatch")
        require(row.get("pcs_path") == PCS_PATH and row.get("native_pcs_identity_transport") is True,
                "unexpected PCS path")
        for key in ("witness_pcs_params", "public_pcs_params"):
            params = row.get(key)
            require(isinstance(params, dict) and params.get("log_inv_rate") == 1, f"missing {key}")
            require(str(params.get("profile", "")).lower() == "fast", "PCS profile must be Fast")
        require(row.get("verified") is True and row.get("transcript_match") is True
                and row.get("pcs_config_match") is True, "proof/transcript/PCS validation failed")
        timings = row.get("times", {})
        for key in TIMING_KEYS:
            value = timings.get(key)
            require(isinstance(value, (int, float)) and not isinstance(value, bool)
                    and math.isfinite(value) and value >= 0, f"missing/invalid timing: {key}")
        require(timings["prover_ms"] > 0, "zero prover time")
        require(sum(timings[k] for k in TIMING_KEYS if k != "prover_ms") <= timings["prover_ms"] * 1.01 + 1,
                "stage timers exceed E2E wall time")
        require(isinstance(row.get("proof_bytes"), int) and row["proof_bytes"] > 0, "invalid proof size")
        for key in hash_keys:
            require(isinstance(row.get(key), str) and len(row[key]) == 64
                    and all(c in "0123456789abcdef" for c in row[key]), f"missing/invalid {key}")
        require(isinstance(row.get("verify_ms"), (int, float)) and math.isfinite(row["verify_ms"])
                and row["verify_ms"] >= 0, "invalid verify time")
        require(timings["public_claims"] > 0 and timings["witness_claims"] > 0,
                "witness and public terminal claims must both be authenticated")
    require(sorted(r.get("repetition") for r in rows) == list(range(repetitions)), "unbalanced sample indices")
    for key in hash_keys:
        require(len({r[key] for r in rows}) == 1, f"{key} changes across repetitions")
    resources = time_resources(logs / f"{case['id']}.resource.txt")
    require(resources.get("available") and resources.get("time_exit_status") == 0,
            "missing successful /usr/bin/time resource evidence")
    return {
        **case, "state": "complete", "metadata": metadata, "repetitions": len(rows),
        "timings_median_ms": {key: statistics.median(r["times"][key] for r in rows) for key in TIMING_KEYS},
        "prover_range_ms": [min(r["times"]["prover_ms"] for r in rows), max(r["times"]["prover_ms"] for r in rows)],
        "verify_median_ms": statistics.median(r["verify_ms"] for r in rows),
        "witness_commitment_blake3": rows[0]["witness_commitment_blake3"],
        "public_commitment_blake3": rows[0]["public_commitment_blake3"],
        "witness_pcs_params": rows[0]["witness_pcs_params"], "public_pcs_params": rows[0]["public_pcs_params"],
        "resources": resources,
        "raw_stdout": f"{case['id']}.stdout.jsonl", "raw_stderr": f"{case['id']}.stderr.log",
        "samples": rows,
    }


def aggregate(logs):
    logs = Path(logs).resolve()
    config = json.loads((logs / "config.json").read_text())
    status = json.loads((logs / "status.json").read_text())
    events = {event["id"]: event for event in status.get("cases", [])}
    result = {"schema_version": 1, "generated_utc": utc(), "config": config,
              "run_state": status.get("state"), "source_hashes_verified_before_and_after":
              status.get("source_hashes_verified_before_and_after", False), "cases": [], "comparisons": []}
    modes = config.get("modes", MODES)
    for case in cases(config["levels"], modes):
        event = events.get(case["id"], {})
        if event.get("state") != "complete":
            item = {**case, "state": event.get("state", "not_run"), "event": event,
                    "resources": time_resources(logs / f"{case['id']}.resource.txt"),
                    "raw_stdout": f"{case['id']}.stdout.jsonl", "raw_stderr": f"{case['id']}.stderr.log"}
        else:
            try:
                item = validate_case(logs, case, config["repetitions"], config["seed"])
            except Exception as error:
                item = {**case, "state": "invalid", "error": str(error), "event": event}
        result["cases"].append(item)
    for mode in MODES:
        if mode not in modes:
            continue
        for level in config["levels"]:
            pair = {c["system"]: c for c in result["cases"] if c["mode"] == mode and c["log_gates"] == level}
            comparison = {"mode": mode, "log_gates": level, "state": "incomplete"}
            if all(pair[s]["state"] == "complete" for s in SYSTEMS):
                left, right = pair["mikros"], pair["flock"]
                same_pcs = all(left[k] == right[k] for k in ("witness_pcs_params", "public_pcs_params"))
                same_commitment = all(left[k] == right[k] for k in ("witness_commitment_blake3", "public_commitment_blake3"))
                comparison.update(same_pcs_params=same_pcs, same_commitment=same_commitment)
                if same_pcs and same_commitment:
                    a, b = left["timings_median_ms"]["prover_ms"], right["timings_median_ms"]["prover_ms"]
                    comparison.update(state="complete", mikros_over_flock=a / b,
                                      flock_over_mikros=b / a, mikros_minus_flock_ms=a - b)
                else:
                    comparison.update(state="invalid", error="commitment or PCS differs between systems")
            result["comparisons"].append(comparison)
    result["all_requested_cases_complete"] = all(c["state"] == "complete" for c in result["cases"])
    result["all_comparisons_valid"] = all(c["state"] == "complete" for c in result["comparisons"])
    result["scope"] = (
        "Single is one circuit of N genuine binary gates; multiple is 128 identical circuit shapes "
        "each with N gates, 128*N total. One fresh process per system/case. Prover time must include "
        "commit, recommit, online conversion, gate/copy PIOPs and original native FLOCK PCS opening. "
        "Offline fixture/static preprocessing and initial public commitment are excluded. Public "
        "recommit is included online. Flock A/B conversion is inside gates_ms; challenge-dependent "
        "copy conversion/evaluation/query conversion is inside copy_ms, not a separately measured timer. "
        "Raw stage records are retained. "
        "One repetition is a descriptive observation, not a statistical speedup estimate.")
    save(logs / "summary.json", result)
    for mode in MODES:
        report = {"mode": mode, "run_state": result["run_state"], "scope": result["scope"],
                  "run_metadata": config, "cases": [c for c in result["cases"] if c["mode"] == mode],
                  "comparisons": [c for c in result["comparisons"] if c["mode"] == mode]}
        atomic_text(logs / f"{mode}_instance.log", json.dumps(report, indent=2, allow_nan=False) + "\n")
    return result


def evidence_hashes(logs):
    logs = Path(logs)
    save(logs / "evidence_sha256.json", {p.name: sha256(p) for p in sorted(logs.iterdir())
         if p.is_file() and p.name != "evidence_sha256.json" and ".tmp-" not in p.name})


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("logs", type=Path)
    arguments = parser.parse_args()
    outcome = aggregate(arguments.logs)
    evidence_hashes(arguments.logs)
    print(json.dumps({"all_requested_cases_complete": outcome["all_requested_cases_complete"],
                      "all_comparisons_valid": outcome["all_comparisons_valid"]}))
