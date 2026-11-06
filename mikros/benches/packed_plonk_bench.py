#!/usr/bin/env python3
"""Shared orchestration and accounting for packed binary PLONK PCS backends."""
import argparse
from dataclasses import dataclass
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

from paper_bench import ROOT, digest, environment, number, report_path, require, run_streamed, save, stats
from plonk_bitwidth_bench import (GRINDING_PHASES, PROVER_PHASES, VERIFIER_PHASES,
                                 nonnegative, packing_geometry, phase_stats)

FEATURES = ("gfni", "avx512f", "avx512bw", "vpclmulqdq")


@dataclass(frozen=True)
class Backend:
    experiment: str
    name: str
    label: str
    output_stem: str
    default_log_n: int
    opening_description: str
    verifier_description: str
    require_complete: bool = False
    required_profile: tuple = ()
    notes: tuple = ()


def parse_options(backend, argv=None):
    parser = argparse.ArgumentParser(description=f"Complete binary Mikros PLONK proofs using {backend.name}.")
    parser.add_argument("--experiment", choices=(backend.experiment,), default=backend.experiment, help=argparse.SUPPRESS)
    parser.add_argument("--worker-bin", type=Path, required=True, help=argparse.SUPPRESS)
    parser.add_argument("--log-n", type=int, default=backend.default_log_n)
    parser.add_argument("--threads", type=int, default=64)
    parser.add_argument("--reps", type=int, default=1)
    parser.add_argument("--warmups", type=int, default=0)
    parser.add_argument("--seed", type=int, default=97)
    parser.add_argument("--time-bin", type=Path, default=Path("/usr/bin/time"))
    parser.add_argument("--out", type=Path)
    options = parser.parse_args(argv)
    if not 14 <= options.log_n <= 30:
        parser.error("--log-n must be between 14 and 30")
    if not 1 <= options.threads <= 256 or not 1 <= options.reps <= 100 or not 0 <= options.warmups <= 10:
        parser.error("require 1..256 threads, 1..100 repetitions and 0..10 warmups")
    if not 0 <= options.seed < 1 << 64:
        parser.error("--seed must fit an unsigned 64-bit integer")
    options.out = (ROOT / (options.out or f"target/paper-results/{backend.experiment}-2-{options.log_n}")).resolve()
    if options.out == ROOT or options.out in ROOT.parents:
        parser.error("use a result directory separate from the repository root")
    options.worker_bin = options.worker_bin.resolve()
    options.time_bin = options.time_bin.resolve()
    return options


def expect(record, key, expected):
    require(type(record.get(key)) is type(expected) and record[key] == expected, f"worker configuration mismatch: {key}")


def validate_grinding(row):
    if "piop_grinding_ms" not in row:
        return
    counters = row["piop_grinding_ms"]
    require(isinstance(counters, dict) and all(key in counters for key in GRINDING_PHASES), "incomplete grinding counters")
    for key in GRINDING_PHASES:
        nonnegative(counters[key], "grinding " + key)
    require(math.isclose(counters["total_ms"], sum(counters[key] for key in ("gate_ms", "copy_ms", "opening_ms")),
                         rel_tol=1e-9, abs_tol=1e-6), "grinding total mismatch")
    require(counters["copy_initial_ms"] <= counters["copy_ms"] + 1e-6, "initial grinding exceeds copy grinding")
    for key, phase in (("gate_ms", "gates_ms"), ("copy_ms", "copy_ms"), ("opening_ms", "packed_opening_ms")):
        require(counters[key] <= row["times"][phase] + 1e-6, "grinding exceeds containing phase")


def parse_worker(text, options, peak_rss_kib, backend):
    records = [json.loads(line) for line in text.splitlines() if line.strip()]
    require(records and all(isinstance(record, dict) for record in records), "missing or invalid worker records")
    meta = records[0]
    circuit = getattr(options, "circuit", "paired")
    require(circuit in ("paired", "near-full"), "unknown binary circuit layout")
    domain = 1 << options.log_n
    gates = domain - 257 if circuit == "near-full" else domain
    expected = {"type": "metadata", "system": "mikros", "mode": "single", "arithmetization": "PLONK",
                "pcs_backend": backend.name, "value_bits": 1, "log_gates": options.log_n, "instances": 1,
                "gates_per_instance": gates, "total_gates": gates,
                "threads": options.threads, "all_core_threads": options.threads, "seed": options.seed,
                "repetitions": options.reps, "warmups": options.warmups}
    circuit_fields = {"circuit_layout": circuit, "genuine_gate_count": gates,
                      "private_input_bits": 256 if circuit == "near-full" else domain,
                      "plonk_domain_size": domain, "copy_domain_size": 4 * domain}
    expected.update({key: value for key, value in circuit_fields.items()
                     if key in meta or circuit == "near-full"})
    geometry = packing_geometry(options.log_n, 1)
    expected.update({key: geometry[key] for key in ("witness_words", "public_words")})
    for key, value in expected.items():
        expect(meta, key, value)
    if backend.required_profile:
        require(isinstance(meta.get("pcs_profile"), dict), "missing PCS security profile")
        for key, value in backend.required_profile:
            expect(meta["pcs_profile"], key, value)
        for group in ("witness", "public"):
            key = group + "_pcs_parameters"
            require(isinstance(meta.get(key), dict) and meta[key], "missing " + group + " PCS parameters")
    # These optional descriptive fields must agree if the worker supplies them.
    for key, value in geometry.items():
        if key in meta:
            expect(meta, key, value)
    if "target_features" in meta:
        require(isinstance(meta["target_features"], dict)
                and all(type(meta["target_features"].get(key)) is bool for key in FEATURES), "invalid CPU feature metadata")
    rows = records[1:]
    require(len(rows) == options.reps + options.warmups, "worker sample count mismatch")
    for index, row in enumerate(rows):
        warmup = index < options.warmups
        for key, value in {"type": "sample", "warmup": warmup,
                           "repetition": index if warmup else index - options.warmups,
                           "verified": True, "transcript_match": True}.items():
            expect(row, key, value)
        if backend.require_complete or "pcs_complete" in row:
            expect(row, "pcs_complete", True)
        for group in ("witness", "public"):
            proof_key = group + "_pcs_proof_bytes"
            if backend.require_complete or proof_key in row:
                require(type(row.get(proof_key)) is int and 0 < row[proof_key] < row.get("proof_bytes", 0),
                        "invalid or missing complete " + group + " PCS proof size")
        if "witness_pcs_proof_bytes" in row and "public_pcs_proof_bytes" in row:
            require(row["witness_pcs_proof_bytes"] + row["public_pcs_proof_bytes"] < row.get("proof_bytes", 0),
                    "PCS proof sizes exceed the complete PLONK proof")
        for key, value in expected.items():
            if key != "type" and key in row:
                expect(row, key, value)
        require(type(row.get("proof_bytes")) is int and row["proof_bytes"] > 0, "invalid serialized proof size")
        for key in ("verify_ms", "peak_rss_kib"):
            number(row.get(key))
        nonnegative(row.get("offline_ms"), "offline time")
        timers = row.get("times")
        require(isinstance(timers, dict) and all(key in timers for key in PROVER_PHASES), "missing prover phase")
        number(timers.get("prover_ms"))
        phase_values = [nonnegative(value, key) for key, value in timers.items() if key.endswith("_ms") and key != "prover_ms"]
        require(sum(phase_values) <= timers["prover_ms"] + 1e-6, "prover phases exceed total")
        verifier = row.get("verifier_times")
        require(isinstance(verifier, dict) and all(key in verifier for key in VERIFIER_PHASES), "missing verifier phase")
        require(sum(nonnegative(value, key) for key, value in verifier.items() if key.endswith("_ms"))
                <= row["verify_ms"] + 1e-6, "verifier phases exceed total")
        expect(timers, "pcs_claims", 2)
        expect(timers, "witness_claims", 11)
        expect(timers, "public_claims", 5 + 3 * (options.log_n + 2))
        for group in ("witness", "public"):
            key = group + "_pcs_params"
            if key in row:
                require(key in meta and row[key] == meta[key], "sample PCS parameters mismatch")
        if "proof_blake3" in row:
            require(isinstance(row["proof_blake3"], str) and re.fullmatch(r"[0-9a-f]{64}", row["proof_blake3"]), "invalid proof digest")
        validate_grinding(row)
    # /proc VmHWM and GNU time are separate measurements; no ordering is assumed.
    return {"metadata": meta, "samples": rows, "peak_rss_kib": number(peak_rss_kib)}


def build_summary(report):
    measured = [row for row in report["samples"] if not row["warmup"]]
    require(measured, "no measured proof")
    return {"prover_ms": stats([row["times"]["prover_ms"] for row in measured]),
            "verify_ms": stats([row["verify_ms"] for row in measured]),
            "proof_bytes": stats([row["proof_bytes"] for row in measured]),
            "ram_mib": report["peak_rss_kib"] / 1024,
            "prover_phases_ms": {key: phase_stats([row["times"][key] for row in measured]) for key in PROVER_PHASES},
            "verifier_phases_ms": {key: phase_stats([row["verifier_times"][key] for row in measured]) for key in VERIFIER_PHASES}}


def render_report(doc, backend):
    row = doc["summary"]
    lines = [f"Mikros binary PLONK | N = 2^{doc['log_n']} | {doc['threads']} thread(s)",
             f"PCS: {backend.name}; {backend.opening_description} for witness and public commitments.",
             *backend.notes,
             "Times and serialized proof sizes are medians; warmups are excluded.", "",
             f"{'System':<20} {'Prover (ms)':>14} {'Verifier (ms)':>15} {'Proof (KiB)':>13} {'RAM (MiB)':>13} {'Samples':>9}",
             "-" * 89,
             f"{'Mikros + ' + backend.label:<20} {row['prover_ms']['median']:>14.3f} {row['verify_ms']['median']:>15.3f} "
             f"{row['proof_bytes']['median']/1024:>13.3f} {row['ram_mib']:>13.2f} {row['prover_ms']['count']:>9}"]
    for title, key in (("Prover", "prover_phases_ms"), ("Verifier", "verifier_phases_ms")):
        lines.extend(["", f"{title} phases (median ms)", f"{'Phase':<30} {'Time (ms)':>14}"])
        lines.extend(f"{name:<30} {values['median']:>14.3f}" for name, values in row[key].items())
    lines.extend(["", "One alternating AND/XOR circuit with authenticated copy wiring and a public output.",
                  "Prover includes witness commitment, online field/layout conversion, gate and permutation proofs,",
                  "copy preparation, packed opening, both complete PCS openings, buffer release and serialization.",
                  "Witness generation, fixed public commitment and PCS/circuit preprocessing are offline.",
                  f"Public PCS opening is online. Verifier includes decoding and {backend.verifier_description}.",
                  "RAM is whole-process GNU time maximum RSS, including setup, warmups, proving and verification.",
                  "Phase medians need not sum to the total median. This report contains no FLOCK comparison arm.",
                  f"Sampling: {doc['warmups']} excluded warmup(s), then {doc['reps']} measured proof(s).",
                  f"CPU affinity: {doc['cpu_affinity']}. Every proof passed full verification and transcript agreement.",
                  f"Raw data: {doc['raw_data']}"])
    return "\n".join(lines) + "\n"


def parse_resource(text):
    match = re.search(r"Maximum resident set size \(kbytes\):\s*(\d+)", text)
    require(match is not None and re.search(r"^\s*Exit status:\s*0\s*$", text, re.M), "missing RSS or successful worker exit")
    return number(int(match.group(1)))


def worker_environment(threads):
    env = environment(threads)
    # Inherited stage profiling adds work to measured kernels.
    env.pop("RAAFOLD_PROFILE_STAGES", None)
    env.pop("ADDITIVE_FRI_PROFILE_STAGES", None)
    return env


def run(options, backend, *, driver_path, runner=run_streamed):
    require(options.worker_bin.is_file(), "compiled Cargo worker is missing")
    require(options.time_bin.is_file(), "GNU time is required")
    version = subprocess.run([str(options.time_bin), "--version"], capture_output=True, text=True)
    require(version.returncode == 0 and "GNU" in version.stdout, "--time-bin must be GNU time")
    require(hasattr(os, "sched_getaffinity"), "this benchmark requires Linux CPU affinity support")
    allowed = os.sched_getaffinity(0)
    require(options.threads <= len(allowed), "worker count exceeds the allowed CPU affinity")
    cpu = min(allowed) if options.threads == 1 else None
    folder = options.out / "runs" / (datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ") + f"-{time.time_ns()}")
    folder.mkdir(parents=True)
    args = ["--worker", "--log-gates", str(options.log_n), "--threads", str(options.threads),
            "--repetitions", str(options.reps), "--warmups", str(options.warmups), "--seed", str(options.seed)]
    resource = folder / "resource.txt"
    with (folder / "stdout.txt").open("w", encoding="utf-8") as stdout, (folder / "stderr.txt").open("w", encoding="utf-8") as stderr:
        result = runner([str(options.time_bin), "-v", "-o", str(resource), str(options.worker_bin), *args],
                              cwd=ROOT, env=worker_environment(options.threads), stdout=stdout, stderr=stderr, cpu=cpu,
                              label=f"{backend.experiment}-2-{options.log_n}")
    require(result == 0, f"{backend.label} worker failed (exit {result}); diagnostics: {report_path(folder)}")
    report = parse_worker((folder / "stdout.txt").read_text(encoding="utf-8"), options,
                          parse_resource(resource.read_text(encoding="utf-8")), backend)
    for row in report["samples"]:
        kind, total = ("warmup", options.warmups) if row["warmup"] else ("sample", options.reps)
        print(f"[plonk {backend.label}] {kind} {row['repetition']+1}/{total}: prover={row['times']['prover_ms']:.3f} ms "
              f"verifier={row['verify_ms']:.3f} ms proof={row['proof_bytes']} bytes", flush=True)
    output = options.out / (backend.output_stem + ".log")
    doc = {"schema_version": 1, "experiment": backend.experiment, "log_n": options.log_n, "instances": 1,
           "threads": options.threads, "reps": options.reps, "warmups": options.warmups, "seed": options.seed,
           "cpu_affinity": [cpu] if cpu is not None else sorted(allowed), "summary": build_summary(report),
           "report": report, "worker_sha256": digest(options.worker_bin), "driver_sha256": digest(driver_path),
           "worker_arguments": args, "raw_directory": report_path(folder), "raw_data": report_path(output.with_suffix(".json")),
           "raw_sha256": {name: digest(folder / name) for name in ("stdout.txt", "stderr.txt", "resource.txt")}}
    doc["shared_driver_sha256"] = digest(__file__)
    text = render_report(doc, backend)
    for destination in (output, output.with_suffix(".json")):
        if destination.exists():
            shutil.copy2(destination, folder / ("previous-" + destination.name))
    save(output.with_suffix(".json.tmp"), doc)
    output.with_suffix(".log.tmp").write_text(text, encoding="utf-8")
    os.replace(output.with_suffix(".json.tmp"), output.with_suffix(".json"))
    os.replace(output.with_suffix(".log.tmp"), output)
    print(text, end="", flush=True)
    print(f"Log: {report_path(output)}", flush=True)
    return doc


def main(backend, driver_path):
    try:
        run(parse_options(backend), backend, driver_path=driver_path)
    except (ValueError, OSError, KeyError, TypeError) as error:
        print(f"PLONK {backend.label} benchmark failed: {error}", file=sys.stderr)
        sys.exit(1)
