#!/usr/bin/env python3
"""One binary AND/XOR circuit: Mikros additive FRI versus native FLOCK skip-7."""
import argparse
from datetime import datetime, timezone
import json
import math
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import time

import plonk_fri_bench as fri
import single_constraints_bench as native
from paper_bench import ROOT, digest, number, report_path, require, run_streamed, save, stats
from plonk_bitwidth_bench import phase_stats

SYSTEMS = ("mikros", "flock")
LABELS = {"mikros": "Mikros + FRI", "flock": "FLOCK skip-7"}
MATCHED_FIELDS = ("fixture_digest", "public_io", "log_gates", "seed", "threads", "instances",
                  "circuit_layout", "genuine_gate_count", "private_input_bits",
                  "plonk_domain_size", "copy_domain_size")


def parse_options(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--experiment", choices=("single-constraints", "circuit-sat"),
                        default="single-constraints", help=argparse.SUPPRESS)
    parser.add_argument("--worker-bin", type=Path, required=True, help=argparse.SUPPRESS)
    parser.add_argument("--system", choices=("both", *SYSTEMS), default="both",
                        help="select a single backend for alternating fresh-process runs")
    parser.add_argument("--circuit", choices=("paired", "near-full"), default="paired")
    parser.add_argument("--log-n", type=int,
                        help="log2 PLONK padded domain; paired has 2^N gates, near-full has 2^N-257 gates")
    parser.add_argument("--threads", type=int)
    parser.add_argument("--cpu", type=int, help="single-thread CPU; default is first allowed CPU")
    parser.add_argument("--reps", type=int, default=1)
    parser.add_argument("--warmups", type=int, default=0)
    parser.add_argument("--seed", type=int, default=97)
    parser.add_argument("--time-bin", type=Path, default=Path("/usr/bin/time"))
    parser.add_argument("--output", type=Path, help=".log destination; raw JSON uses the same stem")
    options = parser.parse_args(argv)
    circuit_sat = options.experiment == "circuit-sat"
    if options.log_n is None:
        options.log_n = 30 if circuit_sat else 25
    if options.threads is None:
        options.threads = 64 if circuit_sat else 1
    if not 14 <= options.log_n <= 30:
        parser.error("--log-n must be between 14 and 30")
    if not 1 <= options.threads <= 256 or not 1 <= options.reps <= 100 or not 0 <= options.warmups <= 10:
        parser.error("require 1..256 threads, 1..100 repetitions and 0..10 warmups")
    if not 0 <= options.seed < 1 << 64:
        parser.error("--seed must fit an unsigned 64-bit integer")
    if options.cpu is not None and (options.cpu < 0 or options.threads != 1):
        parser.error("--cpu requires a nonnegative CPU and --threads 1")
    if options.output is None:
        prefix = "circuit-sat" if circuit_sat else "single"
        if options.circuit != "paired":
            prefix += "-" + options.circuit
        suffix = "" if options.system == "both" else "-" + options.system
        options.output = Path(f"target/paper-results/{prefix}-fri-vs-flock-2-{options.log_n}{suffix}.log")
    options.output = (ROOT / options.output).resolve()
    options.worker_bin = options.worker_bin.resolve()
    options.time_bin = options.time_bin.resolve()
    if options.output.suffix != ".log":
        parser.error("--output must end in .log")
    if options.output.parent == ROOT:
        parser.error("historical root logs are protected; use a destination under target/paper-results")
    return options


def validate_breakdown(row):
    parts = row.get("constraint_breakdown")
    require(isinstance(parts, dict) and set(parts) == {"gate", "copy", "shared"},
            "missing PLONK constraint breakdown")
    for metric, total in (("prover_ms", row["times"]["prover_ms"]), ("verifier_ms", row["verify_ms"])):
        for part in parts.values():
            number(part[metric])
        require(math.isclose(sum(part[metric] for part in parts.values()), total, rel_tol=1e-9, abs_tol=1e-6),
                "component times do not sum to E2E")
    require(all(type(p.get("proof_bytes")) is int and p["proof_bytes"] > 0 for p in parts.values())
            and sum(p["proof_bytes"] for p in parts.values()) == row["proof_bytes"],
            "component proof sizes do not sum to E2E")
    times, verifier = row["times"], row["verifier_times"]
    for actual, expected in ((parts["gate"]["prover_ms"], times["gates_ms"]),
                             (parts["gate"]["verifier_ms"], verifier["gate_ms"]),
                             (parts["copy"]["prover_ms"], times["copy_prepare_ms"] + times["copy_ms"] + times["public_pcs_open_ms"]),
                             (parts["copy"]["verifier_ms"], verifier["copy_ms"] + verifier["public_pcs_ms"])):
        require(math.isclose(actual, expected, rel_tol=1e-9, abs_tol=1e-6), "incorrect permutation argument accounting")


def parse_worker(text, options, system, peak_rss_kib):
    require(system in SYSTEMS, "unknown comparison system")
    report = (fri.parse_worker(text, options, peak_rss_kib) if system == "mikros"
              else native.parse_worker(text, options, "flock", peak_rss_kib))
    meta = report["metadata"]
    io = meta.get("public_io")
    require(isinstance(io, list) and len(io) == 2 and type(io[0]) is int and io[0] in (0, 1)
            and type(io[1]) is int and io[1] == 0, "invalid binary public output")
    fixture = meta.get("comparison_fixture_digest" if system == "mikros" else "fixture_digest")
    require(isinstance(fixture, str) and re.fullmatch(r"[0-9a-f]{64}", fixture), "missing comparison fixture digest")
    geometry = native.circuit_geometry(options.log_n, options.circuit)
    for key, value in geometry.items():
        if key != "r1cs_rows":
            fri.expect(meta, key, value)
    if system == "mikros":
        require(meta.get("matrix_authentication") == "preprocessed public PCS"
                and meta.get("permutation_proof") is True, "unexpected Mikros wiring authentication")
        for row in report["samples"]:
            validate_breakdown(row)
    else:
        require(meta.get("r1cs_rows") == geometry["r1cs_rows"] and meta.get("C") == "identity"
                and meta.get("r1cs_outer_blocks") == 1, "unexpected native R1CS matrix layout")
        fri.expect(meta, "witness_words", max(geometry["r1cs_rows"] // 128, 1 << 15))
        for row in report["samples"]:
            times = row["times"]
            require(all(key in times for key in ("commit_ms", "matrix_apply_ms", "gates_ms", "lincheck_input_ms",
                                                 "lincheck_ms", "witness_pcs_open_ms", "serialization_ms")),
                    "missing native online prover phase")
            require(sum(value for key, value in times.items() if key.endswith("_ms") and key != "prover_ms")
                    <= times["prover_ms"] + 1e-6, "native prover phases exceed total")
            verifier = row.get("verifier_times")
            require(isinstance(verifier, dict) and all(key in verifier for key in ("gates_ms", "lincheck_ms", "witness_pcs_ms")),
                    "missing native verifier phase")
            values = [value for key, value in verifier.items() if key.endswith("_ms")]
            require(all(type(value) in (int, float) and math.isfinite(value) and value >= 0 for value in values),
                    "invalid native verifier phase")
            require(sum(values) <= row["verify_ms"] + 1e-6, "native verifier phases exceed total")
    # Normalize only the reporting statement. Cryptographic fixture digests and
    # transcript domains remain backend-specific and are retained above.
    report["comparison_statement"] = {key: meta[key] for key in MATCHED_FIELDS if key != "fixture_digest"}
    report["comparison_statement"]["fixture_digest"] = fixture
    return report


def build_summary(reports, selected_system="both"):
    selected = SYSTEMS if selected_system == "both" else (selected_system,)
    require(tuple(r["metadata"]["system"] for r in reports) == selected, "missing or duplicate comparison system")
    reference = reports[0]["comparison_statement"]
    systems = {}
    for report in reports:
        for key in MATCHED_FIELDS:
            require(report["comparison_statement"][key] == reference[key], f"unequal circuit statement: {key}")
        system = report["metadata"]["system"]
        measured = [row for row in report["samples"] if not row["warmup"]]
        require(measured, "no measured proof")
        phases = sorted(k for k in measured[0]["times"] if k.endswith("_ms") and k != "prover_ms")
        verifier_phases = sorted(k for k in measured[0].get("verifier_times", {}) if k.endswith("_ms"))
        summary = {"pcs_backend": report["metadata"]["pcs_backend"],
                   "prover_ms": stats([r["times"]["prover_ms"] for r in measured]),
                   "verify_ms": stats([r["verify_ms"] for r in measured]),
                   "proof_bytes": stats([r["proof_bytes"] for r in measured]),
                   "ram_mib": report["peak_rss_kib"] / 1024,
                   "warmup_samples": len(report["samples"]) - len(measured),
                   "prover_phases_ms": {k: phase_stats([r["times"][k] for r in measured]) for k in phases},
                   "verifier_phases_ms": {k: phase_stats([r["verifier_times"][k] for r in measured]) for k in verifier_phases}}
        if system == "mikros":
            summary["constraint_breakdown"] = {
                part: {metric: stats([r["constraint_breakdown"][part][metric] for r in measured])
                       for metric in ("prover_ms", "verifier_ms", "proof_bytes")}
                for part in ("gate", "copy", "shared")}
        systems[system] = summary
    return systems


def render_report(doc):
    systems = doc["systems"]
    geometry = native.circuit_geometry(doc["log_n"], doc.get("circuit", "paired"))
    lines = [f"Single-instance binary AND/XOR circuit | {geometry['circuit_layout']} | PLONK domain = 2^{doc['log_n']} | {doc['threads']} thread(s)",
             "Mikros: packed binary PLONK + additive FRI F128. FLOCK: native public-CSC R1CS + Ligerito Fast, skip 7.",
             "The systems use different PCS backends; circuit statement, public output, seed and worker count are matched.",
             f"Circuit: {geometry['genuine_gate_count']} genuine AND/XOR gates; {geometry['private_input_bits']} independent binary inputs.",
             f"FLOCK: {geometry['r1cs_rows']} R1CS rows including input/constant identities and padding.",
             f"Mikros: {geometry['plonk_domain_size']} padded gate rows; {geometry['copy_domain_size']} copy-domain positions.",
             "Times and serialized proof sizes are medians; warmups are excluded.", "",
             f"{'System':<18} {'Prover (ms)':>14} {'Verifier (ms)':>15} {'Proof (KiB)':>13} {'RAM (MiB)':>13} {'Samples':>9}",
             "-" * 87]
    for system, row in systems.items():
        lines.append(f"{LABELS[system]:<18} {row['prover_ms']['median']:>14.3f} {row['verify_ms']['median']:>15.3f} "
                     f"{row['proof_bytes']['median']/1024:>13.3f} {row['ram_mib']:>13.2f} {row['prover_ms']['count']:>9}")
    if all(system in systems for system in SYSTEMS):
        ratio = systems["flock"]["prover_ms"]["median"] / systems["mikros"]["prover_ms"]["median"]
        lines += ["", f"FLOCK/Mikros prover time: {ratio:.3f}x"]
    else:
        lines += ["", "Single-backend run; no paired speedup is inferred."]
    if "mikros" in systems:
        lines += ["", "Mikros single-instance components",
                  f"{'Component':<24} {'Prover (ms)':>14} {'Verifier (ms)':>15} {'Proof (KiB)':>13}"]
        for part, label in (("gate", "Gate constraint"), ("copy", "Permutation argument"), ("shared", "Shared PCS/other")):
            row = systems["mikros"]["constraint_breakdown"][part]
            lines.append(f"{label:<24} {row['prover_ms']['median']:>14.3f} {row['verifier_ms']['median']:>15.3f} {row['proof_bytes']['median']/1024:>13.3f}")
        lines += ["Permutation argument includes online copy preparation, permutation proof and the entire public PCS opening.",
                  "That public opening also authenticates shared selector data and is counted here once.",
                  "Shared PCS/other includes combined packed-opening reduction, witness PCS, commitment and remaining online work."]
    for title, key in (("Online prover", "prover_phases_ms"), ("Verifier", "verifier_phases_ms")):
        lines += ["", f"{title} phases (median ms)", f"{'Phase':<30} " + " ".join(f"{LABELS[s]:>15}" for s in systems)]
        phases = sorted(set().union(*(row[key] for row in systems.values())))
        for phase in phases:
            values = [f"{row[key][phase]['median']:>15.3f}" if phase in row[key] else f"{'-':>15}" for row in systems.values()]
            lines.append(f"{phase:<30} " + " ".join(values))
    lines += ["", "Mikros FRI: rate 1/2; 241 queries; 100-bit query-phase budget excluding folding and other protocol errors; non-ZK.",
              "Prover includes witness commitment, online input/layout conversion, reductions, grinding, complete PCS openings and serialization.",
              "Mikros additionally proves copy wiring; FLOCK includes matrix application and native public-CSC lincheck.",
              "Witness generation and fixed circuit/public commitment preprocessing are offline; public PCS opening is online.",
              "Verifier includes decoding and complete proof checks, including PCS and FLOCK's public-matrix evaluation.",
              "RAM is whole-process GNU time maximum RSS, including offline setup, warmup, proving and verification.",
              "Component and phase medians are separate statistics and need not sum to the total median.",
              f"Sampling: one fresh process per selected system; {doc['warmups']} excluded warmup(s), then {doc['reps']} measured proof(s).",
              f"CPU affinity: {doc['cpu_affinity']}. Every proof passed full verification and transcript agreement.",
              f"Raw data: {doc['raw_data']}"]
    return "\n".join(lines) + "\n"


def run(options):
    require(options.worker_bin.is_file(), "compiled Cargo worker is missing")
    require(options.time_bin.is_file(), "GNU time is required")
    version = subprocess.run([str(options.time_bin), "--version"], capture_output=True, text=True)
    require(version.returncode == 0 and "GNU" in version.stdout, "--time-bin must be GNU time")
    require(hasattr(os, "sched_getaffinity"), "comparison requires Linux CPU affinity support")
    allowed = os.sched_getaffinity(0)
    require(options.threads <= len(allowed), "worker count exceeds allowed CPU affinity")
    cpu = (min(allowed) if options.cpu is None else options.cpu) if options.threads == 1 else None
    require(cpu is None or cpu in allowed, "CPU is outside allowed affinity")
    folder = options.output.with_suffix("") / "runs" / (datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ") + f"-{time.time_ns()}")
    folder.mkdir(parents=True)
    selected = SYSTEMS if options.system == "both" else (options.system,)
    reports, commands, hashes = [], {}, {}
    for system in selected:
        args = ["--worker", "--system", system, "--circuit", options.circuit,
                "--log-gates", str(options.log_n), "--threads", str(options.threads),
                "--repetitions", str(options.reps), "--warmups", str(options.warmups), "--seed", str(options.seed)]
        resource = folder / f"{system}.resource.txt"
        with (folder / f"{system}.stdout.txt").open("w", encoding="utf-8") as out, \
             (folder / f"{system}.stderr.txt").open("w", encoding="utf-8") as err:
            result = run_streamed([str(options.time_bin), "-v", "-o", str(resource), str(options.worker_bin), *args],
                                  cwd=ROOT, env=fri.worker_environment(options.threads), stdout=out, stderr=err, cpu=cpu,
                                  label=f"fri-vs-flock-2-{options.log_n}:{system}")
        require(result == 0, f"{system} worker failed (exit {result}); diagnostics: {report_path(folder)}")
        report = parse_worker((folder / f"{system}.stdout.txt").read_text(encoding="utf-8"), options, system,
                              fri.parse_resource(resource.read_text(encoding="utf-8")))
        reports.append(report)
        commands[system] = args
        hashes[system] = {name: digest(folder / f"{system}.{name}") for name in ("stdout.txt", "stderr.txt", "resource.txt")}
        for row in report["samples"]:
            kind, total = ("warmup", options.warmups) if row["warmup"] else ("sample", options.reps)
            print(f"[{LABELS[system]}] {kind} {row['repetition']+1}/{total}: prover={row['times']['prover_ms']:.3f} ms "
                  f"verifier={row['verify_ms']:.3f} ms proof={row['proof_bytes']} bytes", flush=True)
    doc = {"schema_version": 1, "experiment": options.experiment, "comparison": "mikros-fri-vs-flock-native",
           "selected_system": options.system, "circuit": options.circuit,
           "circuit_geometry": native.circuit_geometry(options.log_n, options.circuit),
           "log_n": options.log_n, "instances": 1, "threads": options.threads,
           "reps": options.reps, "warmups": options.warmups, "seed": options.seed,
           "cpu_affinity": [cpu] if cpu is not None else sorted(allowed), "systems": build_summary(reports, options.system),
           "reports": reports, "worker_sha256": digest(options.worker_bin), "driver_sha256": digest(__file__),
           "parser_sha256": {"fri": digest(fri.__file__), "shared_fri": digest(fri.common.__file__), "flock": digest(native.__file__)},
           "worker_arguments": commands, "raw_directory": report_path(folder), "raw_sha256": hashes,
           "raw_data": report_path(options.output.with_suffix(".json"))}
    text = render_report(doc)
    for destination in (options.output, options.output.with_suffix(".json")):
        if destination.exists():
            shutil.copy2(destination, folder / ("previous-" + destination.name))
    save(options.output.with_suffix(".json.tmp"), doc)
    options.output.with_suffix(".log.tmp").write_text(text, encoding="utf-8")
    os.replace(options.output.with_suffix(".json.tmp"), options.output.with_suffix(".json"))
    os.replace(options.output.with_suffix(".log.tmp"), options.output)
    print(text, end="", flush=True)
    print(f"Log: {report_path(options.output)}", flush=True)
    return doc


if __name__ == "__main__":
    try:
        run(parse_options())
    except (ValueError, OSError, KeyError, TypeError) as error:
        print(f"FRI/FLOCK comparison failed: {error}", file=sys.stderr)
        sys.exit(1)
