#!/usr/bin/env python3
"""Comparison orchestration for the Cargo bench paper entry points.

Workers contain the existing Rust proofs and timers. Each measurement process
is isolated; this driver only prepares commands, checks results and reports.
"""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import math
import os
from pathlib import Path
import platform
import queue
import re
import shutil
import statistics
import subprocess
import sys
import threading
import time

ROOT = Path(__file__).resolve().parents[2]
NAMES = {"pairreuse": "mikros", "flock128": "flock", "binius_switchover": "binius"}
LABELS = {"mikros": "Mikros", "flock": "FLOCK skip-7", "binius": "Binius SFF"}
HASHES = ("sha256", "keccak", "blake3")
SHARED_DIGESTS = ("fixture_digest_blake3", "circuit_digest_blake3", "r1cs_digest_blake3",
                  "prepared_witness_blake3", "public_digest_blake3")
FRI_BACKEND = "Additive FRI F128 equality"
NATIVE_BACKEND = "native Ligerito Fast"
# Canonical JSON receipt produced by setup.py for the pinned upstream archive.
BINIUS_RECEIPT_SHA256 = "3f2dff62658fecd2d4f4a6177e950a219915e8220986c3ac7017133a76eb30dc"


def require(condition, message):
    if not condition:
        raise ValueError(message)


def number(value):
    require(type(value) in (int, float) and math.isfinite(value) and value > 0,
            f"invalid positive measurement: {value!r}")
    return value


def stats(values):
    require(values, "no measured samples")
    values = [number(v) for v in values]
    return {"median": statistics.median(values), "mean": statistics.mean(values),
            "min": min(values), "max": max(values), "count": len(values)}


def proof_size(rows, key):
    sizes = [r[key] for r in rows]
    require(sizes and all(type(v) is int and v > 0 for v in sizes) and len(set(sizes)) == 1,
            "inconsistent or invalid serialized proof sizes")
    return sizes[0]


def build_e2e_summary(reports):
    systems = {}
    for system in ("mikros", "flock"):
        selected = [r for r in reports if r["metadata"]["system"] == system]
        require(selected, f"missing {system} measurements")
        all_rows = [r for report in selected for r in report["samples"]]
        for r in all_rows:
            require(r.get("verified") is True and r.get("transcript_match") is True,
                    "proof verification or transcript check failed")
            require(type(r.get("warmup")) is bool, "missing warmup marker")
            number(r["prover_ms"]); number(r["verify_ms"])
        measured = [r for r in all_rows if not r["warmup"]]
        systems[system] = {"prover_ms": stats([r["prover_ms"] for r in measured]),
                           "verify_ms": stats([r["verify_ms"] for r in measured]),
                           "proof_bytes": proof_size(all_rows, "proof_bytes"),
                           "ram_mib": max(number(r["peak_rss_kib"]) for r in selected) / 1024,
                           "warmup_samples": len(all_rows) - len(measured)}
        if any("constraint_breakdown" in r.get("timings", {}) for r in all_rows):
            for r in all_rows:
                parts = r.get("timings", {}).get("constraint_breakdown", {})
                require(set(parts) == {"gate", "copy", "shared"}, "incomplete constraint breakdown")
                for key, total_key in (("prover_ms", "prover_ms"), ("verifier_ms", "verify_ms")):
                    total = sum(number(parts[p][key]) for p in parts)
                    require(math.isclose(total, r[total_key], rel_tol=1e-9, abs_tol=1e-9),
                            "constraint timers do not sum to E2E time")
                require(all(type(parts[p]["proof_bytes"]) is int and parts[p]["proof_bytes"] > 0 for p in parts)
                        and sum(parts[p]["proof_bytes"] for p in parts) == r["proof_bytes"],
                        "constraint proof sizes do not sum to the serialized proof")
            systems[system]["constraint_breakdown"] = {
                part: {"prover_ms": stats([r["timings"]["constraint_breakdown"][part]["prover_ms"] for r in measured]),
                       "verify_ms": stats([r["timings"]["constraint_breakdown"][part]["verifier_ms"] for r in measured]),
                       "proof_bytes": proof_size([r["timings"]["constraint_breakdown"][part] for r in all_rows], "proof_bytes")}
                for part in ("gate", "copy", "shared")}
    return {"systems": systems}


def build_hadamard_summary(formal_reports, cold_reports):
    formal = [r for report in formal_reports for r in report["samples"]]
    cold = [r for report in cold_reports for r in report["samples"]]
    warmups = [r for report in formal_reports for r in report.get("warmups", [])]
    for r in formal + cold + warmups:
        require(all(r.get(k) is True for k in ("verifier_passed", "transcript_matched", "source_claims_verified")),
                "Hadamard verification, transcript or source check failed")
        number(r["prover_ms"]); number(r["verifier_ms"])
    systems = {}
    for native, system in NAMES.items():
        rows = [r for r in formal if r["system"] == native]
        if not rows:
            continue
        ram = [r for r in cold if r["system"] == native]
        require(ram, f"missing fresh-process RAM measurements for {system}")
        size_key = "proof_bytes" if system == "binius" else "bincode_bytes"
        all_rows = rows + ram + [r for r in warmups if r["system"] == native]
        systems[system] = {"prover_ms": stats([r["prover_ms"] for r in rows]),
                           "verify_ms": stats([r["verifier_ms"] for r in rows]),
                           "proof_bytes": proof_size(all_rows, size_key),
                           "ram_mib": statistics.median(number(r["vm_hwm_after_prove_kib"]) for r in ram) / 1024,
                           "warmup_samples": sum(r["system"] == native for r in warmups)}
    require("mikros" in systems and "flock" in systems, "missing Mikros/FLOCK comparison")
    return {"systems": systems}


def report_path(value):
    """Keep published reports independent of local account and checkout paths."""
    path = Path(value)
    absolute = path if path.is_absolute() else ROOT / path
    try:
        return absolute.resolve().relative_to(ROOT).as_posix()
    except ValueError:
        return path.name


def render_report(doc):
    title = "Binary Hadamard" if doc["experiment"] == "hadamard" else doc["experiment"].upper() + " end-to-end"
    size = f"N = 2^{doc['log_n']}" if doc["experiment"] == "hadamard" else f"{doc['instances']} instances"
    lines = [f"{title} | {size} | {doc['threads']} thread(s)",
             "Times are medians; proof size is serialized bytes / 1024.", "",
             f"{'System':<16} {'Prover (ms)':>13} {'Verifier (ms)':>15} {'Proof (KiB)':>13} {'RAM (MiB)':>12} {'Samples':>9}",
             "-" * 83]
    if doc.get("mikros_wiring") == "public":
        lines.insert(1, "Mikros public wiring: sparse sigma MLE checked directly by the verifier (online cost included).")
    elif doc.get("mikros_wiring") == "packed":
        lines.insert(1, "Mikros uses the original packed permutation argument for copy constraints.")
    if doc.get("mikros_pcs") == "fri":
        lines.insert(1, "PCS: Mikros additive FRI over F2^128; FLOCK native Ligerito Fast.")
        if doc["instances"] == 256:
            lines.insert(2, "Mikros uses two 128-instance packed R1CS groups, public matrix checks, and one merged FRI opening.")
    for system, row in doc["systems"].items():
        label = "Mikros FRI" if system == "mikros" and doc.get("mikros_pcs") == "fri" else LABELS[system]
        lines.append(f"{label:<16} {row['prover_ms']['median']:>13.3f} "
                     f"{row['verify_ms']['median']:>15.3f} {row['proof_bytes']/1024:>13.3f} "
                     f"{row['ram_mib']:>12.2f} {row['prover_ms']['count']:>9}")
    ratio = doc["systems"]["flock"]["prover_ms"]["median"] / doc["systems"]["mikros"]["prover_ms"]["median"]
    lines += ["", f"FLOCK/Mikros prover time: {ratio:.3f}x", ""]
    for system, row in doc["systems"].items():
        if "constraint_breakdown" not in row:
            continue
        lines += [f"{LABELS[system]} single-instance components",
                  f"{'Component':<20} {'Prover (ms)':>13} {'Verifier (ms)':>15} {'Proof (KiB)':>13}", "-" * 64]
        copy_label = "Permutation argument" if doc.get("mikros_wiring") == "packed" else "Copy constraint"
        for part, label in (("gate", "Gate constraint"), ("copy", copy_label), ("shared", "Shared PCS/other")):
            p = row["constraint_breakdown"][part]
            lines.append(f"{label:<20} {p['prover_ms']['median']:>13.3f} {p['verify_ms']['median']:>15.3f} {p['proof_bytes']/1024:>13.3f}")
        lines += [f"{copy_label} includes online witness-dependent preparation; component proofs include their grinding nonces.",
                  "Shared PCS/other is counted once. Per-sample components sum to E2E; component medians may not.", ""]
        if system == "mikros" and doc.get("mikros_pcs") == "fri":
            lines.insert(len(lines) - 1,
                         "Permutation argument also counts the full public PCS opening once; it authenticates gate selectors as well as copy labels.")
    lines += [f"{name.capitalize()}: {scope}" for name, scope in doc["measurement_scope"].items()]
    if doc.get("cpu_affinity") is not None:
        lines.append(f"Worker CPU affinity: {doc['cpu_affinity']}")
    if doc.get("validation"):
        lines.append(f"Separate validation: {doc['validation']['rejected_mutations']} mutations rejected")
    lines += ["Every measured proof was verified; warmups are excluded from timing statistics.",
              f"Raw data: {report_path(doc['raw_data'])}"]
    return "\n".join(lines) + "\n"


def parse_options(argv):
    p = argparse.ArgumentParser(description="Compare Mikros and FLOCK; print a table and save a .log plus raw .json.")
    p.add_argument("--experiment", choices=(*HASHES, "hadamard", "single", "multiple"), required=True, help=argparse.SUPPRESS)
    p.add_argument("--worker-bin", type=Path, required=True, help=argparse.SUPPRESS)
    p.add_argument("--threads", type=int, default=1)
    p.add_argument("--instances", type=int, choices=(1, 256), help="default: single suite 1, other entries 256")
    p.add_argument("--wiring", choices=("packed", "public"),
                   help="Mikros single-instance copy argument; FLOCK stays native")
    p.add_argument("--pcs", choices=("native", "fri"), default="native",
                   help="Mikros PCS (default: native); FLOCK stays native")
    p.add_argument("--log-n", type=int, default=30)
    p.add_argument("--reps", type=int, default=10, help="Mikros/FLOCK measured proofs per process (default: 10); Binius always 1")
    p.add_argument("--warmups", type=int, default=1)
    p.add_argument("--batches", type=int, default=1, help="E2E fresh measurement processes per system (default: 1; total samples = reps * batches)")
    p.add_argument("--rss-runs", type=int, default=3, help="Hadamard Mikros/FLOCK fresh RAM processes; Binius reuses its sole proof")
    p.add_argument("--cpu", type=int, help="bind all worker threads to this logical CPU (requires --threads 1)")
    p.add_argument("--output", type=Path, help="readable log; detailed JSON uses the same stem")
    p.add_argument("--hash-output-dir", type=Path,
                   help="multiple suite: per-hash e2e_flock_<hash>.log directory (default: repository root)")
    p.add_argument("--binius-bin", type=Path, help="reuse a prebuilt native AES32 Binius worker")
    p.add_argument("--without-binius", action="store_true", help="explicitly run only Mikros/FLOCK Hadamard")
    p.add_argument("--validate", action="store_true", help="run separate E2E tamper-validation processes")
    p.add_argument("--time-bin", type=Path, default=Path(shutil.which("gtime") or "/usr/bin/time"), help="GNU time executable")
    o = p.parse_args([a for a in argv if a != "--bench"])
    if o.instances is None:
        o.instances = 1 if o.experiment == "single" else 256
    if o.wiring is None:
        o.wiring = "packed"
    if o.experiment == "single" and o.instances != 1:
        p.error("single_hash_compare requires --instances 1")
    if o.experiment == "multiple" and o.instances != 256:
        p.error("multi_hash_fri_compare requires --instances 256")
    if o.hash_output_dir is not None and o.experiment != "multiple":
        p.error("--hash-output-dir requires the multiple-hash suite")
    if o.experiment == "multiple":
        o.hash_output_dir = (ROOT / (o.hash_output_dir or ROOT)).resolve()
    if min(o.threads, o.reps, o.batches, o.rss_runs) <= 0 or o.warmups < 0:
        p.error("threads, reps, batches and rss-runs must be positive; warmups must be nonnegative")
    if not 14 <= o.log_n <= 30:
        p.error("log-n must be between 14 and 30")
    if o.cpu is not None and (o.cpu < 0 or o.threads != 1):
        p.error("--cpu requires a nonnegative CPU and --threads 1")
    if o.experiment == "hadamard" and o.warmups != 1:
        p.error("Hadamard retains exactly one excluded warmup")
    if o.experiment == "hadamard" and o.validate:
        p.error("--validate is an E2E tamper-check option")
    if o.experiment != "hadamard" and (o.without_binius or o.binius_bin):
        p.error("Binius options apply only to binary_hadamard")
    if o.wiring == "public" and (o.experiment == "hadamard" or o.instances != 1):
        p.error("--wiring public requires a single-instance hash experiment")
    if o.pcs == "fri" and (o.experiment == "hadamard" or o.wiring != "packed"):
        p.error("--pcs fri requires a hash experiment with --wiring packed")
    if o.output is None:
        filename = f"compare_hadamard_2_{o.log_n}.log" if o.experiment == "hadamard" else (
            f"e2e_flock_{o.experiment}.log" if o.instances == 256 else f"single_hash_{o.experiment}_compare.log")
        if o.wiring == "public":
            filename = f"single_hash_{o.experiment}_public_wiring_compare.log"
        if o.experiment == "single":
            filename = "single_hash_compare.log" if o.wiring == "packed" else "single_hash_public_wiring_compare.log"
        if o.experiment == "multiple":
            filename = "multiple-hash-fri-vs-flock.log"
        if o.pcs == "fri" and o.instances == 1:
            o.output = ROOT / "target/paper-results" / ("single-hash-fri-vs-flock.log" if o.experiment == "single"
                        else f"single-{o.experiment}-fri-vs-flock.log")
        elif o.experiment == "multiple":
            o.output = ROOT / "target/paper-results" / filename
        else:
            o.output = ROOT / filename
    elif not o.output.is_absolute():
        o.output = ROOT / o.output
    o.output = o.output.resolve()
    if o.output.suffix != ".log":
        p.error("--output must end in .log (the JSON sidecar has suffix .json)")
    if o.pcs == "fri" and o.output.parent == ROOT.resolve():
        if o.instances != 256 or o.experiment not in HASHES or o.output.name != f"e2e_flock_{o.experiment}.log":
            p.error("Only 256-instance hash comparisons may refresh their matching root e2e_flock_<hash>.log")
    return o


def load(path):
    return json.loads(Path(path).read_text(encoding="utf-8"))


def save(path, value):
    Path(path).write_text(json.dumps(value, indent=2, allow_nan=False) + "\n", encoding="utf-8")


def digest(path):
    hasher = hashlib.sha256()
    with Path(path).open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            hasher.update(chunk)
    return hasher.hexdigest()


def environment(threads):
    # Match the production paths; inherited profiling/reference switches must
    # not silently change the comparison or disable protocol grinding.
    env = {k: v for k, v in os.environ.items() if not k.startswith(("FLOCK_", "MIKROS_", "RAYON_", "LIG_", "E2E_", "ADDITIVE_FRI_"))
           and k != "LD_PRELOAD"
           and not k.endswith(("_TRACE", "_PCORES_ONLY", "_CENSUS"))}
    env.update(RAYON_NUM_THREADS=str(threads), FLOCK_VERIFY_THREADS=str(threads),
               FLOCK_PCS_PACKED_DIRECT="1", FLOCK_WP_UNINIT="1", LC_ALL="C")
    return env


def validate_pcs_metadata(meta, options, system):
    """The optional FRI comparison must not accept an old native worker."""
    if getattr(options, "pcs", "native") != "fri":
        return
    expected_mode = "single" if options.instances == 1 else "multiple"
    require(meta.get("instances") == options.instances and meta.get("instance_mode") == expected_mode
            and meta.get("verifier_threads") == 1, "FRI comparison requires matching instance metadata and one verifier thread")
    require(meta.get("pcs_backend") == (FRI_BACKEND if system == "mikros" else NATIVE_BACKEND),
            "PCS backend mismatch")
    require(all(isinstance(meta.get(k), str) and re.fullmatch(r"[0-9a-f]{64}", meta[k])
                for k in SHARED_DIGESTS), "missing canonical statement/witness digests")
    if system == "mikros":
        key = "single_plonk" if options.instances == 1 else "multiple_r1cs"
        protocol = meta.get(key)
        require(isinstance(protocol, dict) and protocol.get("pcs_backend") == FRI_BACKEND,
                f"missing {key} FRI metadata")
        if options.instances == 1:
            require(meta.get("arithmetization") == "binary PLONK" and meta.get("mikros_wiring") == "packed",
                    "FRI worker must prove packed binary PLONK")
            claims = 2
            oracles = (("witness", "witness_words"), ("public", "public_table_words"))
        else:
            require(meta.get("arithmetization") == "binary packed R1CS",
                    "multiple FRI worker must prove binary packed R1CS")
            require(protocol.get("instances") == 256 and protocol.get("packed_groups") == 2
                    and protocol.get("instance_packing_width") == 128
                    and protocol.get("gate_hadamard_invocations") == 2
                    and protocol.get("permutation_proof") is False
                    and meta.get("r1cs_c_identity") is True
                    and type(meta.get("r1cs_k_log")) is int and 7 <= meta["r1cs_k_log"] <= 24
                    and protocol.get("witness_words") == (2 << meta["r1cs_k_log"]),
                    "multiple FRI packing or public R1CS dimensions mismatch")
            claims = 1
            oracles = (("witness", "witness_words"),)
        require(protocol.get("pcs_complete") is True and protocol.get("pcs_claims") == claims,
                f"FRI must authenticate all {claims} committed oracle(s)")
        profile = protocol.get("pcs_profile")
        require(isinstance(profile, dict) and profile.get("field_bits") == profile.get("packed_word_bits") == 128
                and profile.get("zero_knowledge") is False
                and profile.get("protocol") == "BaseFold MLE-check + additive FRI",
                "missing FRI protocol profile")
        require(type(profile.get("native_queries")) is int and profile["native_queries"] > 0
                and type(profile.get("query_security_bits")) is int and profile["query_security_bits"] > 0
                and type(profile.get("log_inv_rate")) is int and 0 <= profile["log_inv_rate"] <= 30
                and profile.get("rate") == 2.0 ** -profile["log_inv_rate"]
                and profile.get("query_security_scope") == "FRI query phase only; excludes folding and other protocol errors",
                "invalid FRI query/rate profile")
        for prefix, words_key in oracles:
            params = protocol.get(prefix + "_pcs_params")
            require(isinstance(params, dict) and type(params.get("words")) is int
                    and params["words"] > 0 and params["words"] & (params["words"] - 1) == 0
                    and params["words"] == protocol.get(words_key)
                    and type(params.get("log_words")) is int and params["log_words"] == params["words"].bit_length() - 1
                    and params.get("native_queries") == profile["native_queries"]
                    and params.get("log_inv_rate") == profile["log_inv_rate"],
                    f"missing or inconsistent {prefix} FRI parameters")


def parse_e2e_records(records, options, system, reps, warmups, validate=False):
    require(records and records[0]["record"] == "metadata", "missing E2E metadata")
    meta = records[0]
    require(meta["system"] == system and meta["hash"] == options.experiment and meta["instances"] == options.instances
            and meta["threads"] == meta["all_core_threads"] == options.threads
            and meta["baseline_skip"] == meta["univariate_skip"] == 7, "E2E parameter mismatch")
    if system == "mikros" and options.instances == 1:
        require(meta.get("mikros_wiring") == options.wiring, "Mikros wiring mode mismatch")
    validate_pcs_metadata(meta, options, system)
    rows = [r for r in records if r["record"] == "sample"]
    negative = [r for r in records if r["record"] == "negative_tests"]
    require(len(records) == 1 + len(rows) + len(negative) and len(rows) == reps + warmups,
            "unexpected E2E record count")
    require([r["iteration"] for r in rows] == list(range(reps + warmups))
            and [r["warmup"] for r in rows] == [i < warmups for i in range(reps + warmups)], "warmup/sample order mismatch")
    require(all(r["verified"] is True and r["transcript_match"] is True for r in rows), "verification failed")
    if getattr(options, "pcs", "native") == "fri":
        require(all(r.get("system") == system and r.get("hash") == options.experiment for r in rows),
                "E2E sample identity mismatch")
    require(len(negative) == int(validate) and all(r["rejected"] == len(r["cases"]) > 0 for r in negative),
            "tamper validation did not complete")
    return meta, rows, negative


def run_streamed(command, *, cwd, env, stdout, stderr, cpu=None, label="", heartbeat_seconds=15):
    """Retain worker output, relay progress, and show liveness during long proofs."""
    started = time.monotonic()
    with subprocess.Popen(command, cwd=cwd, env=env, stdout=stdout,
                          stderr=subprocess.PIPE, text=True, encoding="utf-8", errors="replace",
                          bufsize=1, preexec_fn=(lambda: os.sched_setaffinity(0, {cpu})) if cpu is not None else None) as process:
        pending = queue.Queue()

        def read_progress():
            try:
                for line in process.stderr:
                    pending.put(line)
            finally:
                pending.put(None)

        reader = threading.Thread(target=read_progress, daemon=True)
        reader.start()
        try:
            while True:
                try:
                    line = pending.get(timeout=heartbeat_seconds)
                except queue.Empty:
                    print(f"[{label}] still running; elapsed {time.monotonic() - started:.0f}s; pid {process.pid}",
                          file=sys.stderr, flush=True)
                    continue
                if line is None:
                    break
                stderr.write(line)
                stderr.flush()
                print(f"[{label}] {line.rstrip()}", file=sys.stderr, flush=True)
            returncode = process.wait()
        except BaseException:
            process.terminate()
            process.wait()
            raise
        finally:
            reader.join()
    print(f"[{label}] {'complete' if returncode == 0 else 'failed'}; elapsed {time.monotonic() - started:.1f}s",
          file=sys.stderr, flush=True)
    return returncode


class Runner:
    def __init__(self, options):
        self.o = options
        require(options.worker_bin.is_file(), "compiled Cargo worker is missing")
        require(options.time_bin.is_file(), "GNU time is required; install it or pass --time-bin /path/to/gtime")
        version = subprocess.run([str(options.time_bin), "--version"], capture_output=True, text=True)
        require(version.returncode == 0 and "GNU" in version.stdout, "--time-bin must be GNU time")
        self.cpu = options.cpu
        if hasattr(os, "sched_getaffinity"):
            allowed = os.sched_getaffinity(0)
            if self.cpu is None and options.threads == 1:
                self.cpu = min(allowed)
            require(self.cpu is None or self.cpu in allowed, "CPU is outside the allowed affinity")
        else:
            require(self.cpu is None, "CPU binding is only supported on Linux")
        self.env = environment(options.threads)
        if options.instances == 1 or getattr(options, "pcs", "native") == "fri":
            self.env["FLOCK_VERIFY_THREADS"] = "1"
        stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
        self.folder = ROOT / "target/paper-results" / f"{options.experiment}-{stamp}-{time.time_ns()}"
        self.folder.mkdir(parents=True)
        self.commands = []

    def run(self, name, command):
        print(f"[{self.o.experiment}] {name}", file=sys.stderr, flush=True)
        resource = self.folder / (name + ".resource.txt")
        cmd = [str(self.o.time_bin), "-v", "-o", str(resource)] + list(map(str, command))
        self.commands.append({"name": name, "command": cmd})
        save(self.folder / (name + ".command.json"), cmd)
        with (self.folder / (name + ".stdout.txt")).open("w") as out, (self.folder / (name + ".stderr.txt")).open("w") as err:
            returncode = run_streamed(cmd, cwd=ROOT, env=self.env, stdout=out, stderr=err,
                                      cpu=self.cpu, label=f"{self.o.experiment}:{name}")
        require(returncode == 0, f"{name} failed (exit {returncode}); diagnostics: {self.folder}")
        match = re.search(r"Maximum resident set size \(kbytes\):\s*(\d+)", resource.read_text())
        require(match, f"missing GNU time RSS: {resource}")
        return self.folder / (name + ".stdout.txt"), int(match.group(1))

    def e2e(self, system, batch, validate=False):
        o = self.o
        name = f"{system}-validation" if validate else f"{system}-{batch}"
        reps, warmups = (1, 0) if validate else (o.reps, o.warmups)
        command = [o.worker_bin, "--worker", "--hash", o.experiment, "--system", system,
                   "--instances", o.instances, "--skip", 7, "--threads", o.threads,
                   "--reps", reps, "--warmups", warmups] + (["--tamper"] if validate else [])
        if system == "mikros" and o.instances == 1:
            command += ["--wiring", o.wiring]
        if system == "mikros" and getattr(o, "pcs", "native") == "fri":
            command += ["--pcs", "fri"]
        stdout, peak = self.run(name, command)
        records = [json.loads(line) for line in stdout.read_text().splitlines() if line.strip()]
        meta, rows, negative = parse_e2e_records(records, o, system, reps, warmups, validate)
        return {"metadata": meta, "samples": rows, "peak_rss_kib": peak, "batch": batch, "negative_tests": negative}

    def hadamard(self, name, systems, binary=None, rss=False):
        o = self.o
        output = self.folder / (name + ".json")
        command = ([binary, "--log-n", o.log_n, "--switchover", 6, "--order", "high", "--composition", "native",
                    "--first-round-zero", "yes"] if binary else
                   [o.worker_bin, "--worker", "--logs", o.log_n, "--systems", ",".join(systems)])
        command += ["--threads", o.threads, "--runs", 1 if rss else o.reps, "--output", output]
        if rss:
            command.append("--rss")
        self.run(name, command)
        report = load(output); meta = report["metadata"]
        require(meta["threads"] == meta["rayon_actual_threads"] == o.threads, "Hadamard thread mismatch")
        require(len(report["samples"]) == len(systems) * (1 if rss else o.reps)
                and len(report["warmups"]) == (0 if rss else len(systems)), "Hadamard sample count mismatch")
        for s in systems:
            require(sorted(r["run"] for r in report["samples"] if r["system"] == s)
                    == ([0] if rss else list(range(1, o.reps + 1))), "Hadamard sample identity mismatch")
        require(all(r["log_n"] == o.log_n for r in report["samples"] + report["warmups"]), "Hadamard size mismatch")
        if binary:
            require(meta["switchover"] == 6 and meta["skip_rounds"] == 0 and meta["materialization_after_round"] == 7,
                    "Binius SFF schedule mismatch")
            require(meta["field_backend"] == "aes32" and meta["evaluation_order"] == "HighToLow"
                    and meta["composition_backend"] == "native" and meta["first_round_zero"] is True,
                    "Binius must use the native AES32, high-to-low SFF implementation")
            require(meta["upstream_revision"] == "47675e19c86c0fb676f75437073a77af8e337938"
                    and meta["upstream_modified"] is False, "Binius upstream version mismatch")
        else:
            require(meta["flock_skip_rounds"] == 7 and meta["kappa"] == 128, "Hadamard compression mismatch")
        return report


def binius_binary(o):
    if o.binius_bin is not None:
        require(o.binius_bin.is_file(), "--binius-bin does not exist")
        return o.binius_bin.resolve()
    package = ROOT / "mikros/binius_switchover_hadamard"
    print("Verifying pinned Binius sources (outside measurement)...", file=sys.stderr, flush=True)
    if not (package / "upstream-source.json").is_file() or not (package / "upstream").is_dir():
        subprocess.run([sys.executable, str(package / "setup.py")], cwd=ROOT, check=True, stdout=sys.stderr)
    receipt = load(package / "upstream-source.json")
    require(hashlib.sha256(json.dumps(receipt, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
            == BINIUS_RECEIPT_SHA256, "Binius source receipt differs from the pinned archive")
    upstream = (package / "upstream").resolve()
    inventory = {p.relative_to(upstream).as_posix() for p in upstream.rglob("*") if p.is_file()}
    require(inventory == set(receipt["files_sha256"]), "Binius source inventory differs")
    for name, expected in receipt["files_sha256"].items():
        path = (upstream / name).resolve()
        require(path.is_relative_to(upstream) and digest(path) == expected, f"Binius source changed: {name}")
    # A separate Cargo workspace and target directory avoid contending with the
    # outer cargo bench build lock. Compilation is never inside a RAM worker.
    target = ROOT / "target/paper-binius"
    env = os.environ.copy(); env["CARGO_TARGET_DIR"] = str(target)
    print("Building native Binius AES32 (outside measurement)...", file=sys.stderr, flush=True)
    subprocess.run(["cargo", "build", "--release", "--locked", "--manifest-path", str(package / "Cargo.toml"),
                    "--features", "aes32", "--bin", "binius-switchover-hadamard"], cwd=ROOT, env=env, check=True)
    return target / "release/binius-switchover-hadamard"


def collect_experiment(o):
    runner = Runner(o)
    try:
        formal, cold, validations = [], [], []
        binaries = {str(o.worker_bin): digest(o.worker_bin)}
        if o.experiment == "hadamard":
            binius = None if o.without_binius else binius_binary(o)
            if binius:
                binaries[str(binius)] = digest(binius)
            formal.append(runner.hadamard("formal-mikros-flock", ("pairreuse", "flock128")))
            if binius:
                # One cold proof supplies both timing and post-prove RAM. Do not
                # silently multiply the user's one-run Binius budget with a
                # warmup or separate RAM processes.
                binius_report = runner.hadamard("formal-binius", ("binius_switchover",), binius, rss=True)
                formal.append(binius_report)
                cold.append(binius_report)
            for batch in range(o.rss_runs):
                for system in ("pairreuse", "flock128"):
                    cold.append(runner.hadamard(f"rss-{system}-{batch}", (system,), rss=True))
            # All systems must use the same inputs for each run identifier.
            identities = {}
            for report in formal + cold:
                for row in report["samples"] + report["warmups"]:
                    require(identities.setdefault(row["run"], row["input_digest"]) == row["input_digest"], "Hadamard inputs differ")
            result = build_hadamard_summary(formal, cold)
            scopes = {"prover": "PIOP reduction only; input preparation, serialization, source checks and PCS excluded.",
                      "verifier": "Reduction replay only; no PCS." + (" Binius switchover=6 uses seven small-field folds, no skipped messages." if binius else ""),
                      "ram": f"Mikros/FLOCK: median of {o.rss_runs} fresh-process VmHWM readings immediately after proving, before verification/source checks; includes inputs and setup."
                             + (" Binius: VmHWM from its sole cold measured proof; no additional RAM run." if binius else ""),
                      "sampling": f"Mikros/FLOCK: {o.reps} measured proofs each after one excluded warmup."
                                  + (" Binius: one cold measured proof, with no warmup." if binius else "")}
            measurement_plan = {s: {"measured_proofs": o.reps, "excluded_warmups": 1,
                                    "fresh_ram_proofs": o.rss_runs, "ram_reuses_measured_proof": False}
                                for s in ("mikros", "flock")}
            if binius:
                measurement_plan["binius"] = {"measured_proofs": 1, "excluded_warmups": 0,
                                               "fresh_ram_proofs": 0, "ram_reuses_measured_proof": True}
        else:
            if o.validate:
                validations = [runner.e2e(s, 0, True) for s in ("mikros", "flock")]
            for batch in range(o.batches):
                order = ("mikros", "flock") if batch % 2 == 0 else ("flock", "mikros")
                formal.extend(runner.e2e(s, batch) for s in order)
            identities = [{k: r["metadata"][k] for k in SHARED_DIGESTS} for r in formal + validations]
            require(all(i == identities[0] for i in identities), "E2E statements differ")
            result = build_e2e_summary(formal)
            scopes = {"prover": "Online conversion, commitment, reductions, grinding and native FLOCK Ligerito PCS included; offline setup/transpose and prepared-input clone excluded.",
                      "verifier": "Complete non-deferred verification, including matrix and PCS checks; public-statement preprocessing excluded.",
                      "ram": f"Maximum of {o.batches} measurement-process GNU time peaks per system; includes setup, warmup, proving, verification and allocator retention."}
            if getattr(o, "pcs", "native") == "fri":
                protocol_key = "single_plonk" if o.instances == 1 else "multiple_r1cs"
                fri_profile = next(r["metadata"][protocol_key]["pcs_profile"] for r in formal if r["metadata"]["system"] == "mikros")
                reductions = "gate and copy reductions" if o.instances == 1 else "packed R1CS and public matrix reductions"
                scopes["prover"] = (f"Online conversion, witness commitment, {reductions}, grinding and all PCS openings included; "
                                    "offline setup/transpose and prepared-input clone excluded. Mikros uses additive FRI; FLOCK uses native Ligerito Fast.")
                scopes["pcs"] = (f"The PCS backends differ. FRI uses F2^128, rate 1/{1 << fri_profile['log_inv_rate']} "
                                 f"and {fri_profile['native_queries']} queries ({fri_profile['query_security_bits']}-bit query-phase budget, "
                                 "excluding folding and other protocol errors); see raw metadata for both parameter sets. Neither proof claims zero knowledge.")
                operation = {"sha256": "SHA-256 compression with standard IV, without message padding",
                             "keccak": "Keccak-f[1600] 24-round permutation, without sponge padding",
                             "blake3": "BLAKE3 compression with standard IV, counter 0, block length 64 and flags 11; full 512-bit output"}[o.experiment]
                scopes["workload"] = f"{o.instances} independent instance(s) of {operation}."
                scopes["sampling"] = f"{o.reps} measured proofs after {o.warmups} excluded warmups in each of {o.batches} fresh processes per system."
            measurement_plan = {s: {"measured_proofs": o.reps * o.batches, "excluded_warmups": o.warmups * o.batches,
                                    "fresh_ram_proofs": 0, "ram_reuses_measured_proof": True}
                                for s in ("mikros", "flock")}
        doc = {"schema_version": 2, "experiment": o.experiment, "threads": o.threads, "instances": o.instances,
               "mikros_wiring": o.wiring if o.instances == 1 and o.experiment != "hadamard" else None,
               "mikros_pcs": getattr(o, "pcs", "native"),
               "log_n": o.log_n, "flock_skip": 7, "mikros_kappa": 128, "reps": o.reps, "warmups": o.warmups,
               "batches": o.batches, "rss_runs": o.rss_runs, "cpu_affinity": runner.cpu,
               "measurement_scope": scopes, "measurement_plan": measurement_plan,
               "raw_data": str(o.output.with_suffix(".json")),
               "raw_directory": str(runner.folder), "created_utc": datetime.now(timezone.utc).isoformat(),
               "platform": platform.platform(), "binary_sha256": binaries, "commands": runner.commands,
               "environment": {k: runner.env[k] for k in ("RAYON_NUM_THREADS", "FLOCK_VERIFY_THREADS", "FLOCK_PCS_PACKED_DIRECT", "FLOCK_WP_UNINIT")},
               **result, "raw_reports": formal, "cold_ram_reports": cold, "validation_reports": validations,
               "validation": {"rejected_mutations": sum(n["rejected"] for r in validations for n in r["negative_tests"])} if validations else None}
        return doc, runner
    except BaseException as error:
        save(runner.folder / "status.json", {"state": "failed", "error": str(error)})
        raise


def publish_report(doc, output, folder, text):
    # Keep the previous published files in the run directory. The caller only
    # reaches this point after all measurements for this report have succeeded.
    try:
        output.parent.mkdir(parents=True, exist_ok=True)
        for dst in (output, output.with_suffix(".json")):
            if dst.exists():
                shutil.copy2(dst, folder / ("previous" + dst.suffix))
        staged_json = output.with_suffix(".json.tmp")
        staged_log = output.with_suffix(".log.tmp")
        save(staged_json, doc); staged_log.write_text(text, encoding="utf-8")
        os.replace(staged_json, output.with_suffix(".json")); os.replace(staged_log, output)
        save(folder / "status.json", {"state": "complete", "log": str(output), "raw_data": doc["raw_data"]})
    except BaseException as error:
        save(folder / "status.json", {"state": "failed", "error": str(error)})
        raise


def collect_single_suite(o):
    runner = Runner(o)
    try:
        reports = {}
        argument = ("packed R1CS" if o.instances == 256 else
                    "packed permutation argument" if o.wiring == "packed" else "public wiring")
        for index, name in enumerate(HASHES, 1):
            print(f"[{o.experiment}] hash {index}/{len(HASHES)}: {name}; Mikros {argument} vs FLOCK skip-7",
                  file=sys.stderr, flush=True)
            child = argparse.Namespace(**vars(o))
            child.experiment = name
            child.output = runner.folder / f"{name}.log"
            # Use the same selected CPU and executable for every hash. Each
            # system still runs in fresh measurement processes as usual.
            child.cpu = runner.cpu
            doc, child_runner = collect_experiment(child)
            publish_report(doc, child.output, child_runner.folder, render_report(doc))
            reports[name] = doc
        return {"schema_version": 1, "experiment": o.experiment, "instances": o.instances,
                "threads": o.threads, "mikros_wiring": o.wiring if o.instances == 1 else None, "flock_skip": 7,
                "mikros_pcs": getattr(o, "pcs", "native"),
                "reps": o.reps, "warmups": o.warmups, "batches": o.batches,
                "cpu_affinity": runner.cpu, "created_utc": datetime.now(timezone.utc).isoformat(),
                "raw_data": str(o.output.with_suffix(".json")), "raw_directory": str(runner.folder),
                "hashes": reports}, runner
    except BaseException as error:
        save(runner.folder / "status.json", {"state": "failed", "error": str(error)})
        raise


def render_single_report(doc):
    argument = ("packed R1CS" if doc.get("instances", 1) == 256 else
                "packed permutation argument" if doc["mikros_wiring"] == "packed" else "public wiring")
    mode = "256-instance" if doc.get("instances", 1) == 256 else "Single-instance"
    heading = (f"{mode} hash comparison | {doc['threads']} thread(s) | "
               f"Mikros {'additive FRI, ' if doc.get('mikros_pcs') == 'fri' else ''}{argument} vs FLOCK skip-7\n"
               f"Combined raw data: {report_path(doc['raw_data'])}\n\n")
    return heading + "\n".join(render_report(doc["hashes"][name]) for name in HASHES)


def main(argv=None):
    o = parse_options(sys.argv[1:] if argv is None else argv)
    if o.experiment in ("single", "multiple"):
        doc, runner = collect_single_suite(o)
        if o.experiment == "multiple":
            # All six measurement groups must pass before replacing any of the
            # three published hash comparisons. Old files are archived by the
            # existing publisher in each hash's diagnostics directory.
            for name in HASHES:
                child = doc["hashes"][name]
                output = o.hash_output_dir / f"e2e_flock_{name}.log"
                child["raw_data"] = str(output.with_suffix(".json"))
                publish_report(child, output, Path(child["raw_directory"]), render_report(child))
        text = render_single_report(doc)
    else:
        doc, runner = collect_experiment(o)
        text = render_report(doc)
    publish_report(doc, o.output, runner.folder, text)
    print(text, end="", flush=True)
    print(f"Log: {report_path(o.output)}", file=sys.stderr)


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        print(f"Benchmark failed: {error}", file=sys.stderr)
        sys.exit(1)
