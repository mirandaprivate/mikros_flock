#!/usr/bin/env python3
"""Complete Mikros/FLOCK circuit-SAT proofs over the same public CSC matrices."""
import argparse
from datetime import datetime, timezone
import json
import math
import os
from pathlib import Path
import re
import shutil
import statistics
import subprocess
import sys
import time

from paper_bench import (ROOT, digest, environment, number, proof_size, report_path,
                         require, run_streamed, save, stats)

SYSTEMS = ("mikros", "flock")
LABELS = {"mikros": "Mikros kappa-128", "flock": "FLOCK skip-7"}
MATCHED_FIELDS = ("fixture_digest", "witness_digest", "matrix_digest", "public_io",
                  "matrix_digest_scope", "log_gates", "seed", "pcs_backend", "r1cs_rows", "witness_words")
PROVER_PHASES = ("commit_ms", "matrix_apply_ms", "gates_ms", "lincheck_input_ms",
                 "lincheck_ms", "witness_pcs_open_ms", "serialization_ms")
VERIFIER_PHASES = ("gates_ms", "lincheck_ms", "witness_pcs_ms")
CONFIG_ROOT = Path(__file__).resolve().parents[2] / "crates/flock-core/configs/ligerito"


def parse_options(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--experiment", choices=("circuit-sat-public-matrix",),
                        default="circuit-sat-public-matrix", help=argparse.SUPPRESS)
    parser.add_argument("--worker-bin", type=Path, required=True, help=argparse.SUPPRESS)
    parser.add_argument("--log-n", type=int, default=30, help="genuine AND/XOR gates: 2^N")
    parser.add_argument("--threads", type=int, default=64)
    parser.add_argument("--cpu", type=int, help="one-thread runs only; default: first allowed CPU")
    parser.add_argument("--reps", type=int, default=1)
    parser.add_argument("--warmups", type=int, default=0)
    parser.add_argument("--seed", type=int, default=97)
    parser.add_argument("--time-bin", type=Path, default=Path("/usr/bin/time"))
    parser.add_argument("--output", type=Path)
    options = parser.parse_args(argv)
    if not 14 <= options.log_n <= 30:
        parser.error("--log-n must be between 14 and 30")
    if min(options.threads, options.reps) < 1 or options.warmups < 0:
        parser.error("--threads and --reps must be positive; --warmups must be nonnegative")
    if not 0 <= options.seed < 1 << 64:
        parser.error("--seed must fit an unsigned 64-bit integer")
    if options.cpu is not None and (options.cpu < 0 or options.threads != 1):
        parser.error("--cpu requires a nonnegative CPU and --threads 1")
    if options.output is None:
        options.output = ROOT / f"target/paper-results/circuit-sat-public-matrix-2-{options.log_n}.log"
    options.output = (ROOT / options.output).resolve()
    options.worker_bin = options.worker_bin.resolve()
    options.time_bin = options.time_bin.resolve()
    if options.output.suffix != ".log":
        parser.error("--output must end in .log")
    if options.output.parent == ROOT:
        parser.error("root paper logs are protected; use a destination under target/paper-results")
    return options


def nonnegative(value, label):
    require(type(value) in (int, float) and math.isfinite(value) and value >= 0,
            f"invalid {label}")
    return value


def expected_pcs_params(log_n):
    m = max(log_n + 2, 22)
    config = (CONFIG_ROOT / f"m{m}_fast.toml").read_text(encoding="utf-8")
    initial = re.search(r"^initial_k\s*=\s*(\d+)\s*$", config, re.M)
    require(initial is not None and re.search(r'^field\s*=\s*"f256"\s*$', config, re.M),
            "invalid native PCS configuration")
    return {"m": m, "profile": "fast", "log_inv_rate": 1, "log_batch_size": int(initial.group(1)),
            "num_lanes": None, "merkle_hash": "blake3"}


def parse_worker(text, options, system, peak_rss_kib):
    records = [json.loads(line) for line in text.splitlines() if line.strip()]
    require(records and records[0].get("type") == "metadata", "missing worker metadata")
    meta = records[0]
    require(meta.get("system") == system and meta.get("mode") == "single"
            and meta.get("instances") == 1 and meta.get("log_gates") == options.log_n
            and meta.get("threads") == meta.get("all_core_threads") == options.threads
            and meta.get("seed") == options.seed, "worker configuration mismatch")
    require(meta.get("genuine_gate_count") == 1 << options.log_n
            and meta.get("r1cs_rows") == 4 << options.log_n
            and meta.get("r1cs_outer_blocks") == 1 and meta.get("C") == "identity",
            "R1CS circuit dimensions or C matrix mismatch")
    require(meta.get("arithmetization") == "native R1CS" and meta.get("matrix_authentication") == "public CSC",
            "both systems must use native R1CS with public CSC matrices")
    require(meta.get("permutation_proof") is False, "unexpected permutation argument")
    require(meta.get("baseline_skip") == 7, "FLOCK baseline must use skip 7")
    require(meta.get("compression_kappa") == 128, "both systems must use compression factor 128")
    require(meta.get("pcs_packed_direct") is True and meta.get("pcs_backend") == "FLOCK native Ligerito Fast",
            "unexpected PCS path")
    require(meta.get("repetitions") == options.reps and meta.get("warmups") == options.warmups,
            "sampling configuration mismatch")
    require(meta.get("witness_words") == max(1 << (options.log_n - 5), 1 << 15), "witness size mismatch")
    for key in ("fixture_digest", "witness_digest", "matrix_digest"):
        require(isinstance(meta.get(key), str) and re.fullmatch(r"[0-9a-f]{64}", meta[key]), f"missing or invalid {key}")
    require(isinstance(meta.get("matrix_digest_scope"), str) and meta["matrix_digest_scope"], "missing matrix digest scope")
    require(meta.get("witness_claims") == 3 and meta.get("public_claims") == 0
            and meta.get("witness_commitments") == 1 and meta.get("public_commitments") == 0,
            "unexpected metadata commitment/claim counts")
    io = meta.get("public_io")
    require(isinstance(io, list) and len(io) == 2 and type(io[0]) is int and io[0] in (0, 1)
            and type(io[1]) is int and io[1] == 0, "invalid binary public output")
    rows = records[1:]
    require(len(rows) == options.reps + options.warmups, "worker sample count mismatch")
    for index, row in enumerate(rows):
        require(row.get("type") == "sample" and row.get("system") == system and row.get("mode") == "single"
                and row.get("instances") == 1 and row.get("log_gates") == options.log_n
                and row.get("threads") == options.threads and row.get("seed") == options.seed,
                "sample configuration mismatch")
        require(row.get("gates_per_instance") == row.get("total_gates") == 1 << options.log_n,
                "sample gate count mismatch")
        warmup = index < options.warmups
        require(type(row.get("warmup")) is bool and row["warmup"] == warmup
                and row.get("repetition") == (index if warmup else index - options.warmups), "sample order mismatch")
        require(all(row.get(key) is True for key in ("verified", "transcript_match", "pcs_config_match")),
                "full proof verification failed")
        times = row["times"]
        number(times["prover_ms"])
        number(row["verify_ms"])
        require(all(key in times for key in PROVER_PHASES), "missing prover phase")
        phases = {key: nonnegative(value, key) for key, value in times.items()
                  if key.endswith("_ms") and key != "prover_ms"}
        require(sum(phases.values()) <= times["prover_ms"] + 1e-6, "prover phases exceed total")
        verifier = row["verifier_times"]
        require(all(key in verifier for key in VERIFIER_PHASES), "missing verifier phase")
        require(sum(nonnegative(value, key) for key, value in verifier.items() if key.endswith("_ms"))
                <= row["verify_ms"] + 1e-6, "verifier phases exceed total")
        require(times.get("witness_claims") == times.get("native_pcs_claims") == 3
                and times.get("public_claims") == 0 and times.get("witness_commitments") == 1
                and times.get("public_commitments") == 0,
                "expected three witness claims, one commitment, no public claims")
        params = row["witness_pcs_params"]
        require(params == expected_pcs_params(options.log_n), "unexpected witness PCS parameters")
        require("public_pcs_params" not in row and "public_commitment_blake3" not in row,
                "public matrices must not introduce a PCS commitment")
        for key in ("proof_blake3", "piop_blake3", "terminal_claims_blake3", "witness_commitment_blake3"):
            require(isinstance(row.get(key), str) and re.fullmatch(r"[0-9a-f]{64}", row[key]), f"invalid {key}")
    proof_size(rows, "proof_bytes")
    return {"metadata": meta, "samples": rows, "peak_rss_kib": number(peak_rss_kib)}


def build_summary(reports):
    require([report["metadata"]["system"] for report in reports] == list(SYSTEMS), "missing comparison system")
    reference = reports[0]["metadata"]
    reference_sample = reports[0]["samples"][0]
    for report in reports:
        for key in MATCHED_FIELDS:
            require(report["metadata"][key] == reference[key], f"unequal public-matrix statement: {key}")
        for row in report["samples"]:
            for key in ("witness_pcs_params", "witness_commitment_blake3"):
                require(row[key] == reference_sample[key], f"unequal witness PCS: {key}")
    systems = {}
    for report in reports:
        measured = [row for row in report["samples"] if not row["warmup"]]
        require(measured, "no measured proofs")
        phases = sorted(key for key in measured[0]["times"] if key.endswith("_ms") and key != "prover_ms")
        verify_phases = sorted(key for key in measured[0]["verifier_times"] if key.endswith("_ms"))
        systems[report["metadata"]["system"]] = {
            "prover_ms": stats([row["times"]["prover_ms"] for row in measured]),
            "linear_reduction_ms": stats([sum(row["times"].get(key, 0.0) for key in
                                              ("linear_prepare_ms", "lincheck_input_ms", "lincheck_ms"))
                                           for row in measured]),
            "verify_ms": stats([row["verify_ms"] for row in measured]),
            "proof_bytes": proof_size(report["samples"], "proof_bytes"),
            "ram_mib": report["peak_rss_kib"] / 1024,
            "warmup_samples": len(report["samples"]) - len(measured),
            "prover_phase_medians_ms": {key: statistics.median(row["times"][key] for row in measured) for key in phases},
            "verifier_phase_medians_ms": {key: statistics.median(row["verifier_times"][key] for row in measured) for key in verify_phases},
        }
    return systems


def render_report(doc):
    lines = [f"Binary circuit SAT, public matrices | N = 2^{doc['log_n']} | {doc['threads']} thread(s)",
             "One connected AND/XOR circuit; identical native R1CS witness and public CSC matrices; C = I.",
             f"Circuit: {1 << doc['log_n']} genuine gates; {4 << doc['log_n']} padded R1CS rows.",
             "Mikros kappa = 128; FLOCK skip = 7. Both use the same native Ligerito Fast witness PCS.",
             "Each proof commits one witness vector, opens three witness claims and has no public PCS claims.",
             "Times are medians; proof size is serialized bytes / 1024.", "",
             f"{'System':<20} {'Prover (ms)':>13} {'Verifier (ms)':>15} {'Proof (KiB)':>13} {'RAM (MiB)':>12} {'Samples':>9}",
             "-" * 87]
    for system in SYSTEMS:
        row = doc["systems"][system]
        lines.append(f"{LABELS[system]:<20} {row['prover_ms']['median']:>13.3f} "
                     f"{row['verify_ms']['median']:>15.3f} {row['proof_bytes']/1024:>13.3f} "
                     f"{row['ram_mib']:>12.2f} {row['prover_ms']['count']:>9}")
    ratio = doc["systems"]["flock"]["prover_ms"]["median"] / doc["systems"]["mikros"]["prover_ms"]["median"]
    lines += ["", f"FLOCK/Mikros prover time: {ratio:.3f}x", "",
              "Linear reduction including preparation (median ms)",
              f"{'Mikros':>13} {'FLOCK':>13}",
              " ".join(f"{doc['systems'][system]['linear_reduction_ms']['median']:>13.3f}" for system in SYSTEMS),
              "Each sample sums linear_prepare_ms + lincheck_input_ms + lincheck_ms before computing statistics.",
              "Mikros reports challenge-dependent matrix folding separately in linear_prepare_ms;",
              "FLOCK includes matrix folding and the inner-product reduction in lincheck_ms."]
    for scope, key in (("Prover", "prover_phase_medians_ms"), ("Verifier", "verifier_phase_medians_ms")):
        lines += ["", f"{scope} phases (median ms)", f"{'Phase':<32} {'Mikros':>13} {'FLOCK':>13}", "-" * 60]
        phases = sorted(set().union(*(report[key] for report in doc["systems"].values())))
        for phase in phases:
            values = [f"{doc['systems'][system][key][phase]:>13.3f}" if phase in doc["systems"][system][key]
                      else f"{'-':>13}" for system in SYSTEMS]
            lines.append(f"{phase:<32} {' '.join(values)}")
    lines += ["", "Prover includes witness commitment, online matrix images, gate reduction, public-matrix lincheck, PCS opening and serialization.",
              "Fixed public matrix construction and witness generation are offline. Both source authentication and public output are verified.",
              "Verifier includes decoding, reduction replay, full public CSC evaluation and PCS verification.",
              "RAM is whole-process GNU time maximum RSS, including offline setup, warmups, proving and verification.",
              "Per-phase medians are separate statistics and need not sum to the total median.",
              f"Sampling: {doc['warmups']} excluded warmup(s), then {doc['reps']} measured proof(s) per system.",
              f"CPU affinity: {doc['cpu_affinity']}. Every sample passed full verification and transcript agreement.",
              f"Raw data: {report_path(doc['raw_data'])}"]
    return "\n".join(lines) + "\n"


def run(options):
    require(options.worker_bin.is_file(), "compiled Cargo worker is missing")
    require(options.time_bin.is_file(), "GNU time is required")
    version = subprocess.run([str(options.time_bin), "--version"], capture_output=True, text=True)
    require(version.returncode == 0 and "GNU" in version.stdout, "--time-bin must be GNU time")
    require(hasattr(os, "sched_getaffinity"), "this comparison requires Linux CPU affinity support")
    allowed = os.sched_getaffinity(0)
    require(options.threads <= len(allowed), "worker count exceeds the allowed CPU affinity")
    cpu = (min(allowed) if options.cpu is None else options.cpu) if options.threads == 1 else None
    require(cpu is None or cpu in allowed, "CPU is outside the allowed affinity")
    folder = ROOT / "target/paper-results" / f"circuit-sat-public-matrix-2-{options.log_n}" / "runs" / (
        datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ") + f"-{time.time_ns()}")
    folder.mkdir(parents=True)
    reports = []
    for system in SYSTEMS:
        command = [str(options.worker_bin), "--worker", "--system", system, "--log-gates", str(options.log_n),
                   "--threads", str(options.threads), "--repetitions", str(options.reps), "--warmups", str(options.warmups),
                   "--seed", str(options.seed)]
        resource = folder / f"{system}.resource.txt"
        with (folder / f"{system}.stdout.txt").open("w", encoding="utf-8") as stdout, \
             (folder / f"{system}.stderr.txt").open("w", encoding="utf-8") as stderr:
            result = run_streamed([str(options.time_bin), "-v", "-o", str(resource)] + command,
                                  cwd=ROOT, env=environment(options.threads), stdout=stdout, stderr=stderr, cpu=cpu,
                                  label=f"public-matrix-2-{options.log_n}:{system}")
        require(result == 0, f"{system} worker failed (exit {result}); diagnostics: {report_path(folder)}")
        resource_text = resource.read_text()
        match = re.search(r"Maximum resident set size \(kbytes\):\s*(\d+)", resource_text)
        require(match is not None, "missing GNU time peak RSS")
        require(re.search(r"Exit status:\s*0\s*$", resource_text, re.M) is not None, "GNU time reports a failed worker")
        reports.append(parse_worker((folder / f"{system}.stdout.txt").read_text(encoding="utf-8"),
                                    options, system, int(match.group(1))))
    doc = {"experiment": options.experiment, "log_n": options.log_n, "instances": 1,
           "threads": options.threads, "cpu_affinity": cpu if cpu is not None else sorted(allowed),
           "seed": options.seed, "reps": options.reps, "warmups": options.warmups,
           "systems": build_summary(reports), "reports": reports, "worker_sha256": digest(options.worker_bin),
           "raw_directory": report_path(folder), "raw_data": report_path(options.output.with_suffix(".json"))}
    text = render_report(doc)
    options.output.parent.mkdir(parents=True, exist_ok=True)
    for destination in (options.output, options.output.with_suffix(".json")):
        if destination.exists():
            shutil.copy2(destination, folder / ("previous-" + destination.name))
    staged_json = options.output.with_suffix(".json.tmp")
    staged_log = options.output.with_suffix(".log.tmp")
    save(staged_json, doc)
    staged_log.write_text(text, encoding="utf-8")
    os.replace(staged_json, options.output.with_suffix(".json"))
    os.replace(staged_log, options.output)
    print(text, end="", flush=True)
    print(f"Log: {report_path(options.output)}", flush=True)
    return doc


if __name__ == "__main__":
    try:
        run(parse_options())
    except (ValueError, OSError, KeyError, json.JSONDecodeError) as error:
        print(f"public-matrix comparison failed: {error}", file=sys.stderr)
        sys.exit(1)
