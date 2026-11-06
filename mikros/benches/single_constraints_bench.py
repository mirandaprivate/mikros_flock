#!/usr/bin/env python3
"""Single-instance AND/XOR circuit comparison using the existing PLONK worker."""
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
LABELS = {"mikros": "Mikros", "flock": "FLOCK skip-7"}
MATCHED_FIELDS = ("fixture_digest", "public_io", "log_gates", "seed", "pcs_backend")


def circuit_geometry(log_n, circuit="paired"):
    require(circuit in ("paired", "near-full"), "unknown binary circuit layout")
    domain = 1 << log_n
    near_full = circuit == "near-full"
    require(not near_full or domain > 257, "near-full circuit needs more than 257 rows")
    return {"circuit_layout": circuit, "genuine_gate_count": domain - 257 if near_full else domain,
            "private_input_bits": 256 if near_full else domain, "plonk_domain_size": domain,
            "copy_domain_size": 4 * domain, "r1cs_rows": domain if near_full else 4 * domain}


def parse_options(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--experiment", choices=("single-constraints", "circuit-sat"),
                        default="single-constraints", help=argparse.SUPPRESS)
    parser.add_argument("--worker-bin", type=Path, required=True, help=argparse.SUPPRESS)
    parser.add_argument("--log-n", type=int, help="binary constraints: 2^N")
    parser.add_argument("--threads", type=int)
    parser.add_argument("--cpu", type=int, help="default: first CPU allowed by the process affinity")
    parser.add_argument("--reps", type=int, default=1)
    parser.add_argument("--warmups", type=int, default=0)
    parser.add_argument("--seed", type=int, default=97)
    parser.add_argument("--time-bin", type=Path, default=Path("/usr/bin/time"))
    parser.add_argument("--output", type=Path)
    options = parser.parse_args(argv)
    circuit_sat = options.experiment == "circuit-sat"
    if options.log_n is None:
        options.log_n = 30 if circuit_sat else 25
    if options.threads is None:
        options.threads = 64 if circuit_sat else 1
    if not 14 <= options.log_n <= 30:
        parser.error("--log-n must be between 14 and 30")
    if options.threads < 1 or options.reps < 1 or options.warmups < 0:
        parser.error("--threads and --reps must be positive and --warmups nonnegative")
    if not 0 <= options.seed < 1 << 64:
        parser.error("--seed must fit an unsigned 64-bit integer")
    if options.cpu is not None and (options.cpu < 0 or options.threads != 1):
        parser.error("--cpu requires a nonnegative CPU and --threads 1")
    if options.output is None:
        options.output = ROOT / (f"target/paper-results/circuit-sat-2-{options.log_n}.log"
                                 if circuit_sat else "single_2_25.log")
    options.output = (ROOT / options.output).resolve()
    options.worker_bin = options.worker_bin.resolve()
    options.time_bin = options.time_bin.resolve()
    if options.output.suffix != ".log":
        parser.error("--output must end in .log")
    if options.output.parent == ROOT and (circuit_sat or options.output.name != "single_2_25.log"):
        parser.error("root paper logs are protected; use a destination under target/paper-results")
    return options


def parse_worker(text, options, system, peak_rss_kib):
    records = [json.loads(line) for line in text.splitlines() if line.strip()]
    require(records and records[0].get("type") == "metadata", "missing worker metadata")
    metadata = records[0]
    require(metadata.get("system") == system and metadata.get("mode") == "single"
            and metadata.get("instances") == 1 and metadata.get("log_gates") == options.log_n
            and metadata.get("threads") == metadata.get("all_core_threads") == options.threads
            and metadata.get("seed") == options.seed, "worker configuration mismatch")
    geometry = circuit_geometry(options.log_n, getattr(options, "circuit", "paired"))
    require(metadata.get("genuine_gate_count") == geometry["genuine_gate_count"],
            "genuine AND/XOR gate count mismatch")
    for key, value in geometry.items():
        if key in metadata or geometry["circuit_layout"] == "near-full":
            require(type(metadata.get(key)) is type(value) and metadata[key] == value,
                    f"binary circuit geometry mismatch: {key}")
    require(metadata.get("baseline_skip") == 7, "FLOCK must use skip 7")
    require(metadata.get("pcs_packed_direct") is True, "packed-direct PCS path must be enabled")
    require(metadata.get("pcs_backend") == "FLOCK native Ligerito Fast", "unexpected PCS backend")
    require(bool(metadata.get("fixture_digest")), "missing fixture digest")
    io = metadata.get("public_io")
    require(isinstance(io, list) and len(io) == 2 and all(type(v) is int and 0 <= v < 1 << 64 for v in io),
            "missing or invalid public I/O")
    require(metadata.get("arithmetization") == ("native R1CS" if system == "flock" else "binary PLONK"),
            "unexpected arithmetization")
    require(metadata.get("matrix_authentication") == ("public CSC" if system == "flock" else "preprocessed public PCS"),
            "unexpected matrix authentication")
    require(metadata.get("permutation_proof") is (system == "mikros"), "unexpected permutation proof path")
    require(metadata.get("repetitions") == options.reps and metadata.get("warmups") == options.warmups,
            "sampling configuration mismatch")
    rows = records[1:]
    require(len(rows) == options.reps + options.warmups, "worker sample count mismatch")
    for index, row in enumerate(rows):
        require(row.get("type") == "sample" and row.get("system") == system
                and row.get("mode") == "single" and row.get("instances") == 1
                and row.get("log_gates") == options.log_n and row.get("seed") == options.seed
                and row.get("threads") == options.threads, "sample configuration mismatch")
        for key in ("gates_per_instance", "total_gates", "genuine_gate_count"):
            if key in row:
                require(type(row[key]) is int and row[key] == geometry["genuine_gate_count"],
                        f"sample gate count mismatch: {key}")
        repetition = index if index < options.warmups else index - options.warmups
        require(row.get("repetition") == repetition and type(row.get("warmup")) is bool
                and row["warmup"] == (index < options.warmups), "sample order mismatch")
        require(all(row.get(key) is True for key in
                    ("verified", "transcript_match", "pcs_config_match")), "proof validation failed")
        number(row["times"]["prover_ms"])
        number(row["verify_ms"])
        for key, value in row["times"].items():
            if key.endswith("_ms"):
                require(type(value) in (int, float) and value >= 0
                        and value < float("inf"), f"invalid phase measurement: {key}")
        if system == "mikros":
            parts = row.get("constraint_breakdown", {})
            require(set(parts) == {"gate", "copy", "shared"}, "missing constraint breakdown")
            for metric, total in (("prover_ms", row["times"]["prover_ms"]), ("verifier_ms", row["verify_ms"])):
                for part in parts.values():
                    number(part[metric])
                require(math.isclose(sum(part[metric] for part in parts.values()), total,
                                     rel_tol=1e-9, abs_tol=1e-9), "component times do not sum to E2E")
            require(all(type(part["proof_bytes"]) is int and part["proof_bytes"] > 0 for part in parts.values())
                    and sum(part["proof_bytes"] for part in parts.values()) == row["proof_bytes"],
                    "component proof sizes do not sum to E2E")
    proof_size(rows, "proof_bytes")
    return {"metadata": metadata, "samples": rows, "peak_rss_kib": number(peak_rss_kib)}


def build_summary(reports):
    require([r["metadata"]["system"] for r in reports] == list(SYSTEMS), "missing comparison system")
    reference = reports[0]["metadata"]
    for report in reports:
        for key in MATCHED_FIELDS:
            require(report["metadata"][key] == reference[key], f"unequal circuit statement or PCS backend: {key}")
    systems = {}
    for report in reports:
        measured = [row for row in report["samples"] if not row["warmup"]]
        phases = sorted(key for key in measured[0]["times"] if key.endswith("_ms") and key != "prover_ms")
        systems[report["metadata"]["system"]] = {
            "prover_ms": stats([row["times"]["prover_ms"] for row in measured]),
            "verify_ms": stats([row["verify_ms"] for row in measured]),
            "proof_bytes": proof_size(report["samples"], "proof_bytes"),
            "ram_mib": report["peak_rss_kib"] / 1024,
            "warmup_samples": len(report["samples"]) - len(measured),
            "phase_medians_ms": {key: statistics.median(row["times"][key] for row in measured)
                                 for key in phases},
        }
        if report["metadata"]["system"] == "mikros":
            systems["mikros"]["constraint_breakdown"] = {
                part: {"prover_ms": stats([row["constraint_breakdown"][part]["prover_ms"] for row in measured]),
                       "verify_ms": stats([row["constraint_breakdown"][part]["verifier_ms"] for row in measured]),
                       "proof_bytes": proof_size([row["constraint_breakdown"][part] for row in report["samples"]], "proof_bytes")}
                for part in ("gate", "copy", "shared")}
    return systems


def render_report(doc):
    lines = [f"Single-instance binary AND/XOR circuit | N = 2^{doc['log_n']} | {doc['threads']} thread(s)",
             "Same AND/XOR circuit and public I/O; native FLOCK Ligerito Fast PCS; FLOCK skip = 7.",
             "Mikros: binary PLONK with packed permutation. FLOCK: native R1CS with public CSC matrices.",
             f"Circuit: {1 << doc['log_n']} genuine AND/XOR gates. FLOCK: {4 << doc['log_n']} R1CS rows after input/constant identities and padding.",
             f"Mikros: {1 << doc['log_n']} gate equations; {4 << doc['log_n']} positions in its copy domain.",
             "Times are medians; proof size is serialized bytes / 1024.", "",
             f"{'System':<16} {'Prover (ms)':>13} {'Verifier (ms)':>15} {'Proof (KiB)':>13} {'RAM (MiB)':>12} {'Samples':>9}",
             "-" * 83]
    for system, row in doc["systems"].items():
        lines.append(f"{LABELS[system]:<16} {row['prover_ms']['median']:>13.3f} "
                     f"{row['verify_ms']['median']:>15.3f} {row['proof_bytes']/1024:>13.3f} "
                     f"{row['ram_mib']:>12.2f} {row['prover_ms']['count']:>9}")
    ratio = doc["systems"]["flock"]["prover_ms"]["median"] / doc["systems"]["mikros"]["prover_ms"]["median"]
    lines += ["", f"FLOCK/Mikros prover time: {ratio:.3f}x", "",
              "Mikros single-instance components",
              f"{'Component':<24} {'Prover (ms)':>13} {'Verifier (ms)':>15} {'Proof (KiB)':>13}", "-" * 68]
    for part, label in (("gate", "Gate constraint"), ("copy", "Permutation argument"), ("shared", "Shared PCS/other")):
        item = doc["systems"]["mikros"]["constraint_breakdown"][part]
        lines.append(f"{label:<24} {item['prover_ms']['median']:>13.3f} {item['verify_ms']['median']:>15.3f} "
                     f"{item['proof_bytes']/1024:>13.3f}")
    lines += ["Permutation argument includes online copy preparation, permutation proof and the entire public PCS opening.",
              "The public opening also authenticates shared selector data; it is assigned here once, without a proportional split.",
              "Shared PCS/other includes the combined packed-opening reduction, witness PCS and remaining online work.", "",
              f"{'Online phase (median ms)':<32} {'Mikros':>13} {'FLOCK':>13}", "-" * 60]
    phases = sorted(set().union(*(r["phase_medians_ms"] for r in doc["systems"].values())))
    for phase in phases:
        values = [f"{doc['systems'][system]['phase_medians_ms'][phase]:>13.3f}"
                  if phase in doc["systems"][system]["phase_medians_ms"] else f"{'-':>13}" for system in SYSTEMS]
        lines.append(f"{phase:<32} {' '.join(values)}")
    lines += ["", "Phase medians are separate statistics and need not sum to the total median.",
              "Prover: complete online proof, including witness commitment, witness-dependent copy preparation,",
              "reductions, grinding, PCS openings, online recomputation and serialization.",
              "Fixed circuit/public preprocessing is offline. See each sample's memory_policy for retained/recomputed PCS data.",
              "Verifier: decoding and full proof verification, including public-matrix checks and PCS; transcript agreement is checked separately.",
              "RAM: whole-process GNU time maximum RSS, including offline preparation, warmup, proving and verification.",
              f"Sampling: {doc['warmups']} excluded warmup(s), then {doc['reps']} measured proofs per system.",
              f"CPU affinity: {doc['cpu_affinity']}. Every sample was verified.",
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
    prefix = "circuit-sat" if options.experiment == "circuit-sat" else "e2e-single"
    folder = ROOT / "target/paper-results" / f"{prefix}-2-{options.log_n}" / "runs" / (
        datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ") + f"-{time.time_ns()}")
    folder.mkdir(parents=True)
    reports = []
    for system in SYSTEMS:
        command = [str(options.worker_bin), "--worker", "--mode", "single", "--system", system,
                   "--log-gates", str(options.log_n), "--threads", str(options.threads), "--repetitions", str(options.reps),
                   "--warmups", str(options.warmups), "--seed", str(options.seed)]
        resource = folder / f"{system}.resource.txt"
        with (folder / f"{system}.stdout.txt").open("w", encoding="utf-8") as out, \
             (folder / f"{system}.stderr.txt").open("w", encoding="utf-8") as err:
            result = run_streamed([str(options.time_bin), "-v", "-o", str(resource)] + command,
                                  cwd=ROOT, env=environment(options.threads), stdout=out, stderr=err, cpu=cpu,
                                  label=f"{prefix}-2-{options.log_n}:{system}")
        require(result == 0, f"{system} worker failed (exit {result}); diagnostics: {report_path(folder)}")
        match = re.search(r"Maximum resident set size \(kbytes\):\s*(\d+)", resource.read_text())
        require(match is not None, "missing GNU time peak RSS")
        reports.append(parse_worker((folder / f"{system}.stdout.txt").read_text(encoding="utf-8"),
                                    options, system, int(match.group(1))))
    doc = {"experiment": options.experiment, "log_n": options.log_n, "instances": 1,
           "threads": options.threads, "cpu_affinity": cpu if cpu is not None else sorted(allowed),
           "seed": options.seed, "reps": options.reps,
           "warmups": options.warmups, "systems": build_summary(reports), "reports": reports,
           "worker_sha256": digest(options.worker_bin), "raw_directory": report_path(folder),
           "raw_data": report_path(options.output.with_suffix(".json"))}
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
        print(f"single-instance comparison failed: {error}", file=sys.stderr)
        sys.exit(1)
