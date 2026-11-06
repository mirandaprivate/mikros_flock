#!/usr/bin/env python3
"""Mikros single-instance PLONK proofs across binary-field witness bitwidths."""
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

SUPPORTED_BITS = (1, 2, 4, 8, 16, 32, 64, 128)
COPY_METADATA = ("copy_field_bits", "copy_mode")
CONFIG_ROOT = ROOT / "crates/flock-core/configs/ligerito"
PROVER_PHASES = ("commit_ms", "gates_ms", "copy_prepare_ms", "copy_ms", "packed_opening_ms",
                 "witness_pcs_open_ms", "public_pcs_open_ms", "serialization_ms", "release_ms")
VERIFIER_PHASES = ("gate_ms", "copy_ms", "opening_ms", "witness_pcs_ms", "public_pcs_ms")
GRINDING_PHASES = ("gate_ms", "copy_ms", "copy_initial_ms", "opening_ms", "total_ms")


def parse_options(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--experiment", choices=("plonk-bitwidth",), default="plonk-bitwidth", help=argparse.SUPPRESS)
    parser.add_argument("--worker-bin", type=Path, required=True, help=argparse.SUPPRESS)
    parser.add_argument("--log-n", type=int, default=26, help="single-instance circuit size: 2^N gates")
    parser.add_argument("--threads", type=int, default=64)
    parser.add_argument("--bits", default=",".join(map(str, SUPPORTED_BITS)), help="comma-separated binary-field degrees")
    parser.add_argument("--reps", type=int, default=1)
    parser.add_argument("--warmups", type=int, default=0)
    parser.add_argument("--seed", type=int, default=97)
    parser.add_argument("--cpu", type=int, help="one-thread runs only; default: first allowed CPU")
    parser.add_argument("--time-bin", type=Path, default=Path("/usr/bin/time"))
    parser.add_argument("--out", type=Path, help="directory for the readable report and raw measurements")
    options = parser.parse_args(argv)
    try:
        options.bits = [int(value) for value in options.bits.split(",")]
    except ValueError:
        parser.error("--bits must be a comma-separated list from 1,2,4,8,16,32,64,128")
    if not options.bits or len(set(options.bits)) != len(options.bits) or any(value not in SUPPORTED_BITS for value in options.bits):
        parser.error("--bits must contain distinct field degrees from 1,2,4,8,16,32,64,128")
    if not 14 <= options.log_n <= 30:
        parser.error("--log-n must be between 14 and 30")
    if not 1 <= options.threads <= 256 or not 1 <= options.reps <= 100 or not 0 <= options.warmups <= 10:
        parser.error("require 1..256 threads, 1..100 repetitions and 0..10 warmups")
    if not 0 <= options.seed < 1 << 64:
        parser.error("--seed must fit an unsigned 64-bit integer")
    if options.cpu is not None and (options.cpu < 0 or options.threads != 1):
        parser.error("--cpu requires a nonnegative CPU and --threads 1")
    if options.out is None:
        options.out = ROOT / f"target/paper-results/plonk-bitwidth-wide-2-{options.log_n}"
    options.out = (ROOT / options.out).resolve()
    if options.out == ROOT or options.out in ROOT.parents:
        parser.error("use a result directory separate from the repository root")
    options.worker_bin = options.worker_bin.resolve()
    options.time_bin = options.time_bin.resolve()
    return options


def next_power_of_two(value):
    return 1 << (value - 1).bit_length()


def packing_geometry(log_n, bits):
    require(bits in SUPPORTED_BITS, "unsupported base field")
    n = 1 << log_n
    kappa = 128 // bits
    tag_bits = next_power_of_two(log_n + 2 + bits)
    digits = (log_n + 2 + bits - 1) // bits
    return {"value_bits": bits, "base_field_bits": bits, "kappa": kappa,
            "tag_bits": tag_bits, "copy_kappa": max(1, 128 // tag_bits),
            "copy_field_bits": max(128, tag_bits),
            "copy_mode": "packed" if tag_bits < 128 else ("unpacked_f128" if tag_bits == 128 else "unpacked_f256"),
            "witness_words": max(4 * (n // kappa), 1 << 15),
            "public_words": max(next_power_of_two(5 + 3 * digits) * (n // kappa), 1 << 15)}


def nonnegative(value, label):
    require(type(value) in (int, float) and math.isfinite(value) and value >= 0, f"invalid {label}")
    return value


def expected_pcs_params(words):
    require(words > 0 and words & (words - 1) == 0, "PCS word count must be a power of two")
    m = words.bit_length() - 1 + 7
    path = CONFIG_ROOT / f"m{m}_fast.toml"
    require(path.is_file(), f"missing native Fast PCS configuration m{m} for {words} packed words")
    config = path.read_text(encoding="utf-8")
    initial = re.search(r"^initial_k\s*=\s*(\d+)\s*$", config, re.M)
    require(initial is not None and re.search(r'^field\s*=\s*"f256"\s*$', config, re.M),
            "invalid native PCS configuration")
    return {"m": m, "profile": "fast", "log_inv_rate": 1, "log_batch_size": int(initial.group(1)),
            "num_lanes": None, "merkle_hash": "blake3"}


def phase_stats(values):
    require(values, "no phase measurements")
    values = [nonnegative(value, "phase measurement") for value in values]
    return {"median": statistics.median(values), "mean": statistics.mean(values),
            "min": min(values), "max": max(values), "count": len(values)}


def parse_worker(text, options, bits, peak_rss_kib):
    records = [json.loads(line) for line in text.splitlines() if line.strip()]
    require(records and records[0].get("type") == "metadata", "missing worker metadata")
    meta = records[0]
    require(meta.get("system") == "mikros" and meta.get("mode") == "single" and meta.get("arithmetization") == "PLONK",
            "expected Mikros single-instance PLONK")
    require(meta.get("log_gates") == options.log_n and meta.get("instances") == 1
            and meta.get("gates_per_instance") == meta.get("total_gates") == 1 << options.log_n,
            "circuit dimensions mismatch")
    require(meta.get("threads") == meta.get("all_core_threads") == options.threads and meta.get("seed") == options.seed,
            "worker configuration mismatch")
    require(meta.get("repetitions") == options.reps and meta.get("warmups") == options.warmups, "sampling configuration mismatch")
    require(meta.get("pcs_backend") == "FLOCK native Ligerito Fast", "unexpected PCS backend")
    present_copy_metadata = {key for key in COPY_METADATA if key in meta}
    legacy_copy_metadata = not present_copy_metadata and bits <= 16
    require(legacy_copy_metadata or present_copy_metadata == set(COPY_METADATA), "missing or incomplete copy-field metadata")
    for key, expected in packing_geometry(options.log_n, bits).items():
        if legacy_copy_metadata and key in COPY_METADATA:
            continue
        require(meta.get(key) == expected, f"field or packing mismatch: {key}")
    require(meta.get("field_semantics") == "GF(2^b); ADD is field addition, MUL is field multiplication",
            "unexpected witness field semantics")
    for group in ("witness", "public"):
        require(meta.get(group + "_pcs_params") == expected_pcs_params(meta[group + "_words"]),
                "unexpected " + group + " PCS parameters")
    require(isinstance(meta.get("target_features"), dict)
            and all(type(meta["target_features"].get(key)) is bool for key in ("gfni", "avx512f", "avx512bw", "vpclmulqdq")),
            "missing CPU feature metadata")
    rows = records[1:]
    require(len(rows) == options.reps + options.warmups, "worker sample count mismatch")
    for index, row in enumerate(rows):
        warmup = index < options.warmups
        require(row.get("type") == "sample" and row.get("value_bits") == bits, "sample field mismatch")
        require(type(row.get("warmup")) is bool and row["warmup"] == warmup
                and row.get("repetition") == (index if warmup else index - options.warmups), "sample order mismatch")
        require(row.get("verified") is True and row.get("transcript_match") is True, "full proof verification failed")
        for group in ("witness", "public"):
            require(row.get(group + "_pcs_params") == meta[group + "_pcs_params"], "sample PCS parameters mismatch")
        if "pcs_config_match" in row:
            require(row["pcs_config_match"] is True, "PCS configuration check failed")
        for key, expected in (("system", "mikros"), ("mode", "single"), ("log_gates", options.log_n),
                              ("instances", 1), ("threads", options.threads), ("seed", options.seed)):
            if key in row:
                require(row[key] == expected, f"sample configuration mismatch: {key}")
        times = row["times"]
        number(times["prover_ms"])
        number(row["verify_ms"])
        number(row["peak_rss_kib"])
        nonnegative(row["offline_ms"], "offline time")
        require(all(key in times for key in PROVER_PHASES), "missing prover phase")
        phase_values = [nonnegative(value, key) for key, value in times.items() if key.endswith("_ms") and key != "prover_ms"]
        require(sum(phase_values) <= times["prover_ms"] + 1e-6, "prover phases exceed total")
        verifier = row["verifier_times"]
        require(all(key in verifier for key in VERIFIER_PHASES), "missing verifier phase")
        require(sum(nonnegative(value, key) for key, value in verifier.items() if key.endswith("_ms"))
                <= row["verify_ms"] + 1e-6, "verifier phases exceed total")
        grinding = row["piop_grinding_ms"]
        require(all(key in grinding for key in GRINDING_PHASES), "missing PIOP grinding counters")
        for key in GRINDING_PHASES:
            nonnegative(grinding[key], "PIOP grinding " + key)
        require(math.isclose(grinding["total_ms"], sum(grinding[key] for key in ("gate_ms", "copy_ms", "opening_ms")),
                             rel_tol=1e-9, abs_tol=1e-6), "PIOP grinding total mismatch")
        require(grinding["copy_initial_ms"] <= grinding["copy_ms"] + 1e-6, "initial copy grinding exceeds copy grinding")
        require(grinding["total_ms"] < times["prover_ms"], "PIOP grinding exceeds online total")
        for key, phase in (("gate_ms", "gates_ms"), ("copy_ms", "copy_ms"), ("opening_ms", "packed_opening_ms")):
            require(grinding[key] <= times[phase] + 1e-6, "PIOP grinding exceeds containing phase")
        require(type(times.get("witness_claims")) is int and times["witness_claims"] > 0
                and type(times.get("public_claims")) is int and times["public_claims"] > 0
                and times.get("native_pcs_claims") == 2, "packed opening claim count mismatch")
    proof_size(rows, "proof_bytes")
    return {"metadata": meta, "samples": rows, "peak_rss_kib": number(peak_rss_kib)}


def build_summary(reports, widths):
    require([report["metadata"]["value_bits"] for report in reports] == widths, "missing or reordered bitwidth result")
    reference = reports[0]["metadata"]
    summaries = []
    for report in reports:
        meta = report["metadata"]
        for key in ("log_gates", "instances", "threads", "all_core_threads", "seed", "repetitions", "warmups", "pcs_backend", "target_features"):
            require(meta[key] == reference[key], f"unequal run configuration: {key}")
        measured = [row for row in report["samples"] if not row["warmup"]]
        require(measured, "no measured proof")
        phase_names = sorted(key for key in measured[0]["times"] if key.endswith("_ms") and key != "prover_ms")
        verifier_names = sorted(key for key in measured[0]["verifier_times"] if key.endswith("_ms"))
        summaries.append({**packing_geometry(meta["log_gates"], meta["value_bits"]),
                          "prover_ms": stats([row["times"]["prover_ms"] for row in measured]),
                          "online_minus_piop_grinding_ms": stats([row["times"]["prover_ms"] - row["piop_grinding_ms"]["total_ms"]
                                                                  for row in measured]),
                          "piop_grinding_ms": {key: phase_stats([row["piop_grinding_ms"][key] for row in measured])
                                               for key in GRINDING_PHASES},
                          "verify_ms": stats([row["verify_ms"] for row in measured]),
                          "proof_bytes": proof_size(report["samples"], "proof_bytes"),
                          "ram_mib": report["peak_rss_kib"] / 1024,
                          "warmup_samples": len(report["samples"]) - len(measured),
                          "prover_phases_ms": {key: phase_stats([row["times"][key] for row in measured]) for key in phase_names},
                          "verifier_phases_ms": {key: phase_stats([row["verifier_times"][key] for row in measured]) for key in verifier_names}})
    return summaries


def render_report(doc):
    lines = [f"Mikros PLONK witness-field bitwidth | N = 2^{doc['log_n']} | {doc['threads']} thread(s)",
             "One single-instance PLONK circuit per binary field GF(2^b); complete proof with native Ligerito Fast PCS.",
             "b is the base-field degree, not an integer range bound. There is no FLOCK comparison arm in this sweep.",
             "Times are medians; proof size is serialized bytes / 1024.", "",
             f"{'b':>4} {'kappa':>7} {'Tag bits':>10} {'Copy kappa':>12} {'Copy F bits':>12} {'Copy mode':>15} {'Witness words':>16} {'Public words':>16}", "-" * 100]
    for row in doc["widths"]:
        geometry = packing_geometry(doc["log_n"], row["value_bits"])
        lines.append(f"{row['value_bits']:>4} {row['kappa']:>7} {row['tag_bits']:>10} {row['copy_kappa']:>12} "
                     f"{row.get('copy_field_bits', geometry['copy_field_bits']):>12} {row.get('copy_mode', geometry['copy_mode']):>15} "
                     f"{row['witness_words']:>16} {row['public_words']:>16}")
    lines += ["", f"{'b':>4} {'Prover (ms)':>14} {'Verifier (ms)':>15} {'Proof (KiB)':>13} {'RAM (MiB)':>13} {'Samples':>9}", "-" * 75]
    for row in doc["widths"]:
        lines.append(f"{row['value_bits']:>4} {row['prover_ms']['median']:>14.3f} {row['verify_ms']['median']:>15.3f} "
                     f"{row['proof_bytes']/1024:>13.3f} {row['ram_mib']:>13.2f} {row['prover_ms']['count']:>9}")
    lines += ["", "PIOP grinding diagnostic (ms)",
              f"{'b':>4} {'PIOP grinding':>16} {'Initial copy grind':>20} {'Online minus PIOP grinding':>29}"]
    for row in doc["widths"]:
        lines.append(f"{row['value_bits']:>4} {row['piop_grinding_ms']['total_ms']['median']:>16.3f} "
                     f"{row['piop_grinding_ms']['copy_initial_ms']['median']:>20.3f} "
                     f"{row['online_minus_piop_grinding_ms']['median']:>29.3f}")
    lines += ["Online minus PIOP grinding is a diagnostic, not the complete E2E prover time; PCS grinding remains included.",
              "Subtract within each sample before computing statistics. Grinding counters are nested in the phases below."]
    for label, key in (("Prover", "prover_phases_ms"), ("Verifier", "verifier_phases_ms")):
        lines += ["", f"{label} phases (median ms)", f"{'Phase':<28} " + " ".join(f"{'b=' + str(row['value_bits']):>13}" for row in doc["widths"])]
        names = sorted(set().union(*(row[key] for row in doc["widths"])))
        for phase in names:
            values = [f"{row[key][phase]['median']:>13.3f}" if phase in row[key] else f"{'-':>13}" for row in doc["widths"]]
            lines.append(f"{phase:<28} " + " ".join(values))
    lines += ["", "The wiring and gate count are fixed; arithmetic is over GF(2^b), so the statement changes with b.",
              "b=1 uses the specialized binary gate kernel; wider fields use the field reduction path, with gate kappa=1 at b=128.",
              "Copy tags below 128 bits use packed permutation; 128/256-bit tags use ordinary ProductGKR over F128/F256.",
              "Copy F bits is the reduction-field degree. Packed opening still reduces all claims to two native PCS terminals.",
              "Input is supplied in canonical packed-bit layout; witness generation and input-layout preparation are offline.",
              "Prover includes packed-input cloning, witness commitment, gate reduction, challenge-dependent conversion and copy preparation,",
              "permutation proving, packed opening, both witness/public PCS openings, buffer release and serialization.",
              "Fixed public commitment and circuit preprocessing are offline. Copy-tag conversion and public PCS opening are online.",
              "Verifier includes decoding and full verification of gates, permutation, packed opening and both PCS openings.",
              "RAM is whole-process GNU time maximum RSS, including offline setup, warmups, proving and verification.",
              "Each bitwidth runs in its own process. Phase medians need not sum to the total median.",
              f"Sampling: {doc['warmups']} excluded warmup(s), then {doc['reps']} measured proof(s) per bitwidth.",
              f"CPU affinity: {doc['cpu_affinity']}. Every proof passed full verification and transcript agreement.",
              f"Raw data: {report_path(doc['raw_data'])}"]
    return "\n".join(lines) + "\n"


def run(options):
    require(options.worker_bin.is_file(), "compiled Cargo worker is missing")
    # Check every selected shape before committing resources to any proof.
    for bits in options.bits:
        geometry = packing_geometry(options.log_n, bits)
        for group in ("witness", "public"):
            expected_pcs_params(geometry[group + "_words"])
    require(options.time_bin.is_file(), "GNU time is required")
    version = subprocess.run([str(options.time_bin), "--version"], capture_output=True, text=True)
    require(version.returncode == 0 and "GNU" in version.stdout, "--time-bin must be GNU time")
    require(hasattr(os, "sched_getaffinity"), "this comparison requires Linux CPU affinity support")
    allowed = os.sched_getaffinity(0)
    require(options.threads <= len(allowed), "worker count exceeds the allowed CPU affinity")
    cpu = (min(allowed) if options.cpu is None else options.cpu) if options.threads == 1 else None
    require(cpu is None or cpu in allowed, "CPU is outside the allowed affinity")
    folder = options.out / "runs" / (datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ") + f"-{time.time_ns()}")
    folder.mkdir(parents=True)
    reports = []
    for bits in options.bits:
        command = [str(options.worker_bin), "--worker", "--log-gates", str(options.log_n), "--bits", str(bits),
                   "--threads", str(options.threads), "--repetitions", str(options.reps), "--warmups", str(options.warmups),
                   "--seed", str(options.seed)]
        resource = folder / f"b{bits}.resource.txt"
        with (folder / f"b{bits}.stdout.txt").open("w", encoding="utf-8") as stdout, \
             (folder / f"b{bits}.stderr.txt").open("w", encoding="utf-8") as stderr:
            result = run_streamed([str(options.time_bin), "-v", "-o", str(resource)] + command,
                                  cwd=ROOT, env=environment(options.threads), stdout=stdout, stderr=stderr, cpu=cpu,
                                  label=f"plonk-2-{options.log_n}:b{bits}")
        require(result == 0, f"b={bits} worker failed (exit {result}); diagnostics: {report_path(folder)}")
        resource_text = resource.read_text()
        match = re.search(r"Maximum resident set size \(kbytes\):\s*(\d+)", resource_text)
        require(match is not None and re.search(r"Exit status:\s*0\s*$", resource_text, re.M), "missing RSS or successful worker exit")
        report = parse_worker((folder / f"b{bits}.stdout.txt").read_text(encoding="utf-8"), options, bits, int(match.group(1)))
        reports.append(report)
        for row in report["samples"]:
            kind = "warmup" if row["warmup"] else "sample"
            total = options.warmups if row["warmup"] else options.reps
            print(f"[plonk b={bits}] {kind} {row['repetition']+1}/{total}: prover={row['times']['prover_ms']:.3f} ms "
                  f"verifier={row['verify_ms']:.3f} ms proof={row['proof_bytes']} bytes", flush=True)
    output = options.out / "plonk_bitwidth.log"
    doc = {"experiment": "plonk-bitwidth", "log_n": options.log_n, "instances": 1, "bits": options.bits,
           "threads": options.threads, "seed": options.seed, "reps": options.reps, "warmups": options.warmups,
           "cpu_affinity": cpu if cpu is not None else sorted(allowed), "widths": build_summary(reports, options.bits),
           "reports": reports, "worker_sha256": digest(options.worker_bin),
           "raw_directory": report_path(folder), "raw_data": report_path(output.with_suffix(".json"))}
    text = render_report(doc)
    for destination in (output, output.with_suffix(".json")):
        if destination.exists():
            shutil.copy2(destination, folder / ("previous-" + destination.name))
    staged_json = output.with_suffix(".json.tmp")
    staged_log = output.with_suffix(".log.tmp")
    save(staged_json, doc)
    staged_log.write_text(text, encoding="utf-8")
    os.replace(staged_json, output.with_suffix(".json"))
    os.replace(staged_log, output)
    print(text, end="", flush=True)
    print(f"Log: {report_path(output)}", flush=True)
    return doc


if __name__ == "__main__":
    try:
        run(parse_options())
    except (ValueError, OSError, KeyError, json.JSONDecodeError) as error:
        print(f"PLONK bitwidth comparison failed: {error}", file=sys.stderr)
        sys.exit(1)
