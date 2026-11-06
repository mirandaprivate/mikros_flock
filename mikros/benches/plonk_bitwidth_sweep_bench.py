#!/usr/bin/env python3
"""Checkpointed Mikros PLONK field-degree x circuit-size sweep, with full PCS."""
import argparse
from contextlib import contextmanager
import csv
from datetime import datetime, timezone
import io
import json
import math
import os
from pathlib import Path
import queue
import re
import signal
import subprocess
import sys
import threading
import time
from types import SimpleNamespace

import plonk_bitwidth_bench as single
from paper_bench import ROOT, digest, environment, require

DEFAULT_LOG_NS = (20, 22, 24, 26, 28, 30)
METRICS = ("prover_ms", "verify_ms", "proof_bytes", "ram_mib")
STATUSES = {"pending", "running", "ok", "memory_skipped", "pcs_unsupported", "guarded", "error"}
MODEL = {"id": "plonk-live-storage-v2", "public_factor": 3, "witness_factor": 4,
         "merkle_bytes": "8 * storage_bytes / 2^initial_k from the native Fast PCS configuration",
         "copy_bytes_per_gate": {"32": 160, "64": 192, "128": 384, "256": 512}, "fixed_margin_gib": 1}


def now():
    return datetime.now(timezone.utc).isoformat()


def parse_options(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--experiment", choices=("plonk-bitwidth-sweep",), default="plonk-bitwidth-sweep", help=argparse.SUPPRESS)
    parser.add_argument("--worker-bin", required=True, type=Path, help=argparse.SUPPRESS)
    parser.add_argument("--log-ns", default=",".join(map(str, DEFAULT_LOG_NS)))
    parser.add_argument("--bits", default=",".join(map(str, single.SUPPORTED_BITS)))
    parser.add_argument("--threads", type=int, default=64)
    parser.add_argument("--reps", type=int, default=1)
    parser.add_argument("--warmups", type=int, default=0)
    parser.add_argument("--seed", type=int, default=97)
    parser.add_argument("--memory-budget-gib", type=float, help="upper bound on the per-case estimated memory; also limited by live available memory")
    parser.add_argument("--reserve-gib", type=float, default=16, help="host MemAvailable reserve, enforced before and during each worker")
    parser.add_argument("--poll-seconds", type=float, default=1)
    parser.add_argument("--resume", action="store_true", help="strictly validate/reuse successful and guarded cases; retry pending/error cases, recheck preflight skips")
    parser.add_argument("--time-bin", type=Path, default=Path("/usr/bin/time"))
    parser.add_argument("--out", type=Path, default=ROOT / "target/paper-results/plonk-bitwidth-sweep")
    args = parser.parse_args(argv)
    for key, allowed in (("log_ns", range(14, 31)), ("bits", single.SUPPORTED_BITS)):
        try:
            values = [int(value) for value in getattr(args, key).split(",")]
        except ValueError:
            parser.error(f"--{key.replace('_', '-')} must be a comma-separated integer list")
        if not values or len(set(values)) != len(values) or any(value not in allowed for value in values):
            parser.error(f"invalid or duplicate --{key.replace('_', '-')} values")
        setattr(args, key, sorted(values))
    if not 1 <= args.threads <= 256 or not 1 <= args.reps <= 100 or not 0 <= args.warmups <= 10 or not 0 <= args.seed < 1 << 64:
        parser.error("require 1..256 threads, 1..100 samples, 0..10 warmups and a u64 seed")
    if not math.isfinite(args.reserve_gib) or args.reserve_gib <= 0 or not math.isfinite(args.poll_seconds) or not 0.1 <= args.poll_seconds <= 15:
        parser.error("require a positive finite memory reserve and 0.1..15 second polling")
    if args.memory_budget_gib is not None and (not math.isfinite(args.memory_budget_gib) or args.memory_budget_gib <= 0):
        parser.error("--memory-budget-gib must be positive and finite")
    args.out = (ROOT / args.out).resolve()
    if args.out == ROOT or args.out in ROOT.parents:
        parser.error("use an output directory separate from the repository root")
    args.worker_bin = args.worker_bin.resolve()
    args.time_bin = args.time_bin.resolve()
    return args


def memory_estimate(log_n, bits):
    """Live table/codeword/tree storage plus copy peak; this is not measured RSS."""
    shape = single.packing_geometry(log_n, bits)
    public_bytes = 16 * shape["public_words"]
    witness_bytes = 16 * shape["witness_words"]
    copy_bytes = MODEL["copy_bytes_per_gate"][str(shape["tag_bits"])] * (1 << log_n)
    params, missing = pcs_preflight(shape)
    if missing:
        return {"model": MODEL, "physical_gib": None, "estimated_gib": None,
                "reason": "No capacity estimate for an unsupported native PCS configuration", "missing_pcs": missing}
    merkle = sum(8 * size / 2**params[group]["log_batch_size"]
                 for group, size in (("public", public_bytes), ("witness", witness_bytes)))
    physical = (MODEL["public_factor"] * public_bytes + MODEL["witness_factor"] * witness_bytes + merkle + copy_bytes) / 2**30
    return {"model": MODEL, "physical_gib": physical,
            "estimated_gib": physical + MODEL["fixed_margin_gib"]}


def available_memory_gib():
    text = Path("/proc/meminfo").read_text(encoding="ascii")
    match = re.search(r"^MemAvailable:\s*(\d+)\s+kB$", text, re.M)
    require(match is not None, "Linux MemAvailable is required for the host memory guard")
    return int(match.group(1)) / 2**20


def checked_available(reader):
    value = reader()
    require(type(value) in (int, float) and math.isfinite(value) and value >= 0, "invalid host available-memory measurement")
    return value


def pcs_preflight(shape):
    missing = []
    params = {}
    for group in ("witness", "public"):
        words = shape[group + "_words"]
        m = words.bit_length() + 6
        if not (single.CONFIG_ROOT / f"m{m}_fast.toml").is_file():
            missing.append(f"{group}: m{m}_fast.toml")
        else:
            # An existing but invalid configuration is an error, not unsupported.
            params[group] = single.expected_pcs_params(words)
    return params, missing


def worker_options(options, log_n):
    return SimpleNamespace(log_n=log_n, threads=options.threads, reps=options.reps,
                           warmups=options.warmups, seed=options.seed)


def identity(options, estimates, affinity):
    worker_env = environment(options.threads)
    return {"worker_sha256": digest(options.worker_bin), "time_sha256": digest(options.time_bin),
            "driver_sha256": digest(Path(__file__)), "single_driver_sha256": digest(Path(single.__file__)),
            "paper_bench_sha256": digest(Path(__file__).with_name("paper_bench.py")),
            "pcs_configs_sha256": {path.name: digest(path) for path in sorted(single.CONFIG_ROOT.glob("m*_fast.toml"))},
            "cpu_affinity": affinity, "memory_estimates": estimates,
            "worker_environment": {key: value for key, value in worker_env.items()
                                   if key.startswith(("FLOCK_", "MIKROS_", "RAYON_", "LIG_", "E2E_")) or key == "LC_ALL"}}


def configuration(options):
    return {key: getattr(options, key) for key in ("log_ns", "bits", "threads", "reps", "warmups", "seed",
                                                  "memory_budget_gib", "reserve_gib", "poll_seconds")}


def empty_metrics():
    return dict.fromkeys(METRICS)


def metrics(summary):
    return {"prover_ms": summary["prover_ms"]["median"], "verify_ms": summary["verify_ms"]["median"],
            "proof_bytes": summary["proof_bytes"], "ram_mib": summary["ram_mib"]}


def write_atomic(path, text):
    temporary = path.with_name(path.name + ".tmp")
    temporary.write_text(text, encoding="utf-8", newline="")
    os.replace(temporary, path)


def render_report(doc):
    cfg = doc["config"]
    lines = ["Mikros PLONK field-degree x circuit-size sweep; complete native Ligerito Fast PCS",
             f"{cfg['threads']} threads; {cfg['reps']} measured sample(s), {cfg['warmups']} warmup(s) per case; seed {cfg['seed']}.",
             "GF(2^b) ADD/MUL arithmetic, same wiring recipe; there is no FLOCK comparison arm.",
             "Prover includes all online phases and PIOP grinding. Fixed public commitment is offline; public opening is online.",
             "Input is canonical packed storage; witness generation and input layout preparation are offline.",
             "RAM is GNU time process peak RSS including setup, proving and verification; estimates are not measurements.",
             "Missing values are blank; memory_skipped means estimated capacity exceeded, not measured OOM.",
             "guarded means the host memory reserve stopped that case; later cases are still evaluated. Resume does not retry guarded cases.",
             f"Checkpoint revision {doc['revision']}; completed={doc['completed']}", "",
             f"{'logN':>4} {'b':>4} {'Status':<17} {'Prover(ms)':>14} {'Verifier(ms)':>14} {'Proof(B)':>12} {'RAM(MiB)':>12}", "-" * 90]
    for case in doc["cases"]:
        m = case["metrics"]
        values = ["" if m[key] is None else (str(m[key]) if key == "proof_bytes" else f"{m[key]:.3f}") for key in METRICS]
        lines.append(f"{case['log_n']:>4} {case['value_bits']:>4} {case['status']:<17} {values[0]:>14} {values[1]:>14} {values[2]:>12} {values[3]:>12}")
        if case["reason"]:
            lines.append("  " + case["reason"])
    for case in doc["cases"]:
        if case["status"] != "ok":
            continue
        summary = case["summary"]
        lines += ["", f"{case['case_id']} phases (ms; medians):"]
        for group in ("prover_phases_ms", "verifier_phases_ms"):
            lines.append("  " + group + ": " + ", ".join(f"{key}={value['median']:.3f}" for key, value in summary[group].items()))
        lines.append(f"  PIOP grinding={summary['piop_grinding_ms']['total_ms']['median']:.3f}; online minus PIOP grinding={summary['online_minus_piop_grinding_ms']['median']:.3f} (diagnostic, not E2E).")
    return "\n".join(lines) + "\n"


def checkpoint(out, doc):
    doc["revision"] += 1
    doc["updated_at"] = now()
    write_atomic(out / "checkpoint.json", json.dumps(doc, indent=2, allow_nan=False) + "\n")
    buffer = io.StringIO(newline="")
    writer = csv.DictWriter(buffer, fieldnames=["case_id", "log_n", "value_bits", "status", "reason", *METRICS,
                                               "estimated_gib", "available_gib", *single.PROVER_PHASES])
    writer.writeheader()
    for case in doc["cases"]:
        row = {key: case[key] for key in ("case_id", "log_n", "value_bits", "status", "reason", "available_gib")}
        row.update(case["metrics"])
        row["estimated_gib"] = case["memory_estimate"]["estimated_gib"]
        for phase in single.PROVER_PHASES:
            row[phase] = case["summary"]["prover_phases_ms"][phase]["median"] if case["status"] == "ok" else None
        writer.writerow(row)
    write_atomic(out / "results.csv", buffer.getvalue())
    write_atomic(out / "summary.log", render_report(doc))


def process_alive(pid):
    try:
        os.kill(pid, 0)
    except ProcessLookupError:
        return False
    except PermissionError:
        return True
    return True


@contextmanager
def output_lock(out):
    path = out / ".sweep.lock"
    token = f"{os.getpid()}-{time.time_ns()}"
    try:
        with path.open("x", encoding="ascii") as stream:
            json.dump({"pid": os.getpid(), "token": token}, stream)
    except FileExistsError:
        old = json.loads(path.read_text(encoding="ascii"))
        require(not process_alive(old["pid"]), "another sweep driver owns this output directory")
        path.unlink()
        with path.open("x", encoding="ascii") as stream:
            json.dump({"pid": os.getpid(), "token": token}, stream)
    try:
        yield
    finally:
        if path.exists() and json.loads(path.read_text(encoding="ascii")).get("token") == token:
            path.unlink()


def stop_owned_group(process):
    """Only the new session created for this Popen belongs to this driver."""
    try:
        os.killpg(process.pid, signal.SIGTERM)
    except ProcessLookupError:
        return False
    try:
        process.wait(timeout=5)
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGKILL)
        process.wait(timeout=10)
    # GNU time may exit before a worker that ignored SIGTERM. The group remains
    # ours while such a descendant exists, even after its leader has exited.
    try:
        os.killpg(process.pid, 0)
    except ProcessLookupError:
        return True
    try:
        os.killpg(process.pid, signal.SIGKILL)
    except ProcessLookupError:
        pass
    return True


def run_guarded(command, *, folder, env, reserve_gib, poll_seconds, available, on_start, label):
    require(os.name == "posix" and hasattr(os, "killpg"), "guarded sweep workers require Linux process groups")
    events = queue.Queue()
    started = time.monotonic()
    minimum = checked_available(available)
    guarded = minimum < reserve_gib
    if guarded:
        return {"exit_code": None, "guarded": True, "minimum_available_gib": minimum, "elapsed_seconds": 0}
    process = subprocess.Popen(command, cwd=ROOT, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                               text=True, encoding="utf-8", errors="replace", bufsize=1, start_new_session=True)
    def read_stream(name, stream):
        for line in stream:
            events.put((name, line))
        events.put((name, None))
    try:
        on_start(process.pid)
        readers = [threading.Thread(target=read_stream, args=(name, stream), daemon=True)
                   for name, stream in (("stdout", process.stdout), ("stderr", process.stderr))]
        for reader in readers:
            reader.start()
        with (folder / "stdout.txt").open("w", encoding="utf-8") as stdout, \
             (folder / "stderr.txt").open("w", encoding="utf-8") as stderr, \
             (folder / "memory.jsonl").open("w", encoding="utf-8") as memory:
            streams = {"stdout": stdout, "stderr": stderr}
            closed = set()
            next_poll = next_heartbeat = time.monotonic()
            while len(closed) < 2 or process.poll() is None:
                current = time.monotonic()
                if current >= next_poll:
                    value = checked_available(available)
                    minimum = min(minimum, value)
                    memory.write(json.dumps({"elapsed_seconds": current - started, "available_gib": value}) + "\n")
                    memory.flush()
                    next_poll = current + poll_seconds
                    if value < reserve_gib and (process.poll() is None or len(closed) < 2):
                        stopped = stop_owned_group(process)
                        guarded = guarded or stopped
                        if stopped:
                            print(f"[{label}] host memory guard: available={value:.3f} GiB < reserve={reserve_gib:.3f} GiB", flush=True)
                if current >= next_heartbeat:
                    print(f"[{label}] running; elapsed {current-started:.0f}s; available >= {minimum:.2f} GiB", flush=True)
                    next_heartbeat = current + 15
                try:
                    name, line = events.get(timeout=min(poll_seconds, 0.25))
                except queue.Empty:
                    continue
                if line is None:
                    closed.add(name)
                else:
                    streams[name].write(line)
                    streams[name].flush()
                    if name == "stderr":
                        print(f"[{label}] {line.rstrip()}", flush=True)
                    else:
                        try:
                            record = json.loads(line)
                            if record.get("type") == "sample":
                                print(f"[{label}] sample {record['repetition']+1}: prover={record['times']['prover_ms']:.3f} ms verifier={record['verify_ms']:.3f} ms proof={record['proof_bytes']} bytes verified={record['verified']}", flush=True)
                        except (ValueError, KeyError, TypeError):
                            print(f"[{label}] stdout: {line.rstrip()}", flush=True)
        return {"exit_code": process.wait(), "guarded": guarded, "minimum_available_gib": minimum,
                "elapsed_seconds": time.monotonic() - started}
    except BaseException:
        stop_owned_group(process)
        raise
    finally:
        process.stdout.close()
        process.stderr.close()


def classify_exit(result):
    if result["guarded"]:
        return "guarded", "Stopped by the live host MemAvailable guard; no successful measurement."
    if result["exit_code"] != 0:
        return "error", f"Worker exited {result['exit_code']}; no OOM claim without a triggered memory guard."
    return "ok", None


def raw_hashes(folder):
    return {path.name: digest(path) for path in sorted(folder.iterdir()) if path.is_file()}


def parse_case(folder, options, log_n, bits):
    resource = (folder / "resource.txt").read_text(encoding="utf-8")
    match = re.search(r"Maximum resident set size \(kbytes\):\s*(\d+)", resource)
    require(match is not None and re.search(r"Exit status:\s*0\s*$", resource, re.M), "missing RSS or successful GNU time exit")
    report = single.parse_worker((folder / "stdout.txt").read_text(encoding="utf-8"), worker_options(options, log_n), bits, int(match.group(1)))
    summary = single.build_summary([report], [bits])[0]
    return report, summary


def validate_resume(doc, options, expected_identity):
    require(doc.get("schema_version") == 1 and doc.get("experiment") == "plonk-bitwidth-sweep", "unsupported checkpoint")
    require(doc.get("config") == configuration(options), "resume configuration differs from checkpoint")
    require(doc.get("identity") == expected_identity, "resume executable, source, PCS, affinity, environment or memory model differs")
    require([(c["log_n"], c["value_bits"]) for c in doc["cases"]] == [(n, b) for n in options.log_ns for b in options.bits], "resume case grid mismatch")
    for case in doc["cases"]:
        require(case["status"] in STATUSES, "invalid checkpoint status")
        if case.get("active_pid") is not None:
            require(not process_alive(case["active_pid"]), "a previous worker may still be running; refusing concurrent resume")
        if case["status"] not in ("ok", "guarded"):
            require(case["metrics"] == empty_metrics(), "unsuccessful case must not contain measured metrics")
            continue
        folder = (options.out / case["raw_directory"]).resolve()
        require(options.out in folder.parents, "raw result escapes output directory")
        require(raw_hashes(folder) == case["artifacts_sha256"], f"raw artifact hash mismatch: {case['case_id']}")
        if case["status"] == "guarded":
            attempt = case["attempts"][-1]
            require(case["metrics"] == empty_metrics() and case["report"] is None and case["summary"] is None,
                    "guarded case must not contain successful measurements")
            require(attempt.get("status") == "guarded" and attempt.get("guarded") is True
                    and attempt.get("minimum_available_gib", float("inf")) < options.reserve_gib
                    and attempt.get("artifacts_sha256") == case["artifacts_sha256"], "missing memory-guard evidence")
            continue
        report, summary = parse_case(folder, options, case["log_n"], case["value_bits"])
        require(report == case["report"] and summary == case["summary"] and metrics(summary) == case["metrics"], "resume parsed measurements differ from checkpoint")
        require(case["geometry"] == single.packing_geometry(case["log_n"], case["value_bits"]), "resume packing geometry differs")


def run(options, *, estimate=memory_estimate, available=available_memory_gib, launcher=run_guarded, affinity=None):
    require(options.worker_bin.is_file() and options.time_bin.is_file(), "missing worker or GNU time executable")
    if affinity is None:
        require(hasattr(os, "sched_getaffinity"), "Linux CPU affinity is required")
        affinity = sorted(os.sched_getaffinity(0))
    require(len(affinity) >= options.threads, "fewer allowed CPUs than requested workers")
    estimates = {f"n{n}-b{b}": estimate(n, b) for n in options.log_ns for b in options.bits}
    for value in estimates.values():
        if value["estimated_gib"] is not None:
            single.nonnegative(value["estimated_gib"], "memory estimate")
    expected_identity = identity(options, estimates, affinity)
    options.out.mkdir(parents=True, exist_ok=True)
    with output_lock(options.out):
        path = options.out / "checkpoint.json"
        if options.resume:
            require(path.is_file(), "--resume requires checkpoint.json")
            doc = json.loads(path.read_text(encoding="utf-8"))
            validate_resume(doc, options, expected_identity)
        else:
            require(not any((options.out / name).exists() for name in ("checkpoint.json", "results.csv", "summary.log", "raw")),
                    "results exist; use a new directory or --resume")
            doc = {"schema_version": 1, "experiment": "plonk-bitwidth-sweep", "config": configuration(options),
                   "identity": expected_identity, "created_at": now(), "revision": 0, "completed": False, "cases": []}
            for n in options.log_ns:
                for b in options.bits:
                    case_id = f"n{n}-b{b}"
                    doc["cases"].append({"case_id": case_id, "log_n": n, "value_bits": b, "status": "pending", "reason": None,
                                         "geometry": single.packing_geometry(n, b), "memory_estimate": estimates[case_id],
                                         "available_gib": None, "metrics": empty_metrics(), "summary": None, "report": None,
                                         "raw_directory": None, "artifacts_sha256": {}, "attempts": [], "active_pid": None})
        doc["completed"] = False
        checkpoint(options.out, doc)
        for case in doc["cases"]:
            if case["status"] in ("ok", "guarded"):
                print(f"[{case['case_id']}] reused strictly validated {case['status']} result", flush=True)
                continue
            case.update(status="pending", reason=None, metrics=empty_metrics(), summary=None, report=None, active_pid=None)
            attempt = None
            try:
                require(identity(options, estimates, affinity) == expected_identity, "sweep executable, source or configuration changed during the run")
                _, missing = pcs_preflight(case["geometry"])
                if missing:
                    case.update(status="pcs_unsupported", reason="Missing native Fast PCS configuration: " + ", ".join(missing))
                    checkpoint(options.out, doc)
                    print(f"[{case['case_id']}] {case['reason']}", flush=True)
                    continue
                require(case["memory_estimate"]["estimated_gib"] is not None, "missing memory estimate for a supported PCS configuration")
                value = checked_available(available)
                case["available_gib"] = value
                budget = max(0, value - options.reserve_gib)
                if options.memory_budget_gib is not None:
                    budget = min(budget, options.memory_budget_gib)
                if case["memory_estimate"]["estimated_gib"] > budget:
                    case.update(status="memory_skipped", reason=f"Estimated peak {case['memory_estimate']['estimated_gib']:.3f} GiB exceeds admissible budget {budget:.3f} GiB (host available {value:.3f}, reserve {options.reserve_gib:.3f}); not attempted, not measured OOM.")
                    checkpoint(options.out, doc)
                    print(f"[{case['case_id']}] {case['reason']}", flush=True)
                    continue
                folder = options.out / "raw" / case["case_id"] / f"attempt-{len(case['attempts']) + 1:03}"
                folder.mkdir(parents=True)
                for name in ("stdout.txt", "stderr.txt", "resource.txt", "memory.jsonl"):
                    (folder / name).touch()
                case.update(status="running", raw_directory=folder.relative_to(options.out).as_posix())
                attempt = {"started_at": now(), "raw_directory": case["raw_directory"]}
                case["attempts"].append(attempt)
                checkpoint(options.out, doc)
                def on_start(pid):
                    case["active_pid"] = pid
                    checkpoint(options.out, doc)
                command = [str(options.time_bin), "-v", "-o", str(folder / "resource.txt"), str(options.worker_bin), "--worker",
                           "--log-gates", str(case["log_n"]), "--bits", str(case["value_bits"]), "--threads", str(options.threads),
                           "--repetitions", str(options.reps), "--warmups", str(options.warmups), "--seed", str(options.seed)]
                result = launcher(command, folder=folder, env=environment(options.threads), reserve_gib=options.reserve_gib,
                                  poll_seconds=options.poll_seconds, available=available, on_start=on_start, label=case["case_id"])
                case["active_pid"] = None
                attempt.update(result, finished_at=now(), artifacts_sha256=raw_hashes(folder))
                case["artifacts_sha256"] = attempt["artifacts_sha256"]
                case["status"], case["reason"] = classify_exit(result)
                if case["status"] == "ok":
                    require(identity(options, estimates, affinity) == expected_identity, "sweep executable, source or configuration changed while the worker ran")
                    report, summary = parse_case(folder, options, case["log_n"], case["value_bits"])
                    case.update(report=report, summary=summary, metrics=metrics(summary))
                attempt["status"] = case["status"]
                checkpoint(options.out, doc)
                if case["status"] == "error":
                    print(render_report(doc), end="", flush=True)
                    return doc, 1
            except Exception as error:
                case.update(status="error", reason=str(error), active_pid=None, metrics=empty_metrics(), summary=None, report=None)
                if attempt is not None:
                    folder = options.out / case["raw_directory"]
                    case["artifacts_sha256"] = raw_hashes(folder)
                    case["attempts"][-1].update(status="error", reason=str(error), artifacts_sha256=case["artifacts_sha256"])
                checkpoint(options.out, doc)
                print(f"[{case['case_id']}] error: {error}", file=sys.stderr, flush=True)
                return doc, 1
        doc["completed"] = True
        checkpoint(options.out, doc)
        print(render_report(doc), end="", flush=True)
        return doc, 0


if __name__ == "__main__":
    try:
        _, exit_code = run(parse_options())
        sys.exit(exit_code)
    except (ValueError, OSError, KeyError, TypeError, json.JSONDecodeError) as error:
        print(f"PLONK bitwidth sweep failed: {error}", file=sys.stderr)
        sys.exit(1)
