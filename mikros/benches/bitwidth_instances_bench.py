#!/usr/bin/env python3
"""Complete F128-FRI proofs across field widths: single PLONK and 256-instance R1CS."""
import argparse
import csv
from datetime import datetime, timezone
import io
import json
import math
import os
from pathlib import Path
import queue
import re
import subprocess
import sys
import threading
import time

from paper_bench import ROOT, digest, environment, require, stats
from plonk_bitwidth_bench import packing_geometry
from plonk_bitwidth_sweep_bench import (
    available_memory_gib, checked_available, output_lock, process_alive,
    stop_owned_group, write_atomic,
)

WIDTHS = (1, 2, 4, 8, 16, 32, 64, 128)
CASES = ("single", "multiple")
INSTANCES = {"single": 1, "multiple": 256}
ARITHMETIZATION = {"single": "PLONK", "multiple": "Packed R1CS"}
PCS = "Additive FRI F128 equality"
METRICS = ("prover_ms", "verify_ms", "proof_bytes", "ram_mib")
STATUSES = {"pending", "running", "ok", "memory_skipped", "guarded", "error"}


def now():
    return datetime.now(timezone.utc).isoformat()


def parse_options(argv=None):
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--experiment", choices=("bitwidth-instances",), default="bitwidth-instances", help=argparse.SUPPRESS)
    p.add_argument("--worker-bin", type=Path, required=True, help=argparse.SUPPRESS)
    p.add_argument("--bits", default=",".join(map(str, WIDTHS)))
    p.add_argument("--log-n", type=int, default=20, help="log2 gates per instance")
    p.add_argument("--threads", type=int, default=1)
    p.add_argument("--reps", type=int, default=1)
    p.add_argument("--warmups", type=int, default=0)
    p.add_argument("--seed", type=int, default=97)
    p.add_argument("--cpu", type=int, help="single-thread CPU; default is first allowed CPU")
    p.add_argument("--out", type=Path, default=ROOT / "target/paper-results/bitwidth-instances-2-20")
    p.add_argument("--resume", action="store_true")
    p.add_argument("--memory-budget-gib", type=float, default=300,
                   help="stop only this worker group if its sampled resident memory exceeds the budget")
    p.add_argument("--reserve-gib", type=float, default=32, help="minimum live host MemAvailable")
    p.add_argument("--poll-seconds", type=float, default=1)
    p.add_argument("--time-bin", type=Path, default=Path("/usr/bin/time"))
    p.add_argument("--no-plots", action="store_true", help="save measurements now; plot the checkpoint separately")
    o = p.parse_args(argv)
    try:
        o.bits = sorted(int(x) for x in o.bits.split(","))
    except ValueError:
        p.error("--bits must list distinct values from 1,2,4,8,16,32,64,128")
    if not o.bits or len(o.bits) != len(set(o.bits)) or any(b not in WIDTHS for b in o.bits):
        p.error("--bits must list distinct values from 1,2,4,8,16,32,64,128")
    if not 14 <= o.log_n <= 30 or not 1 <= o.threads <= 256 or not 1 <= o.reps <= 100 or not 0 <= o.warmups <= 10:
        p.error("require log-n 14..30, threads 1..256, reps 1..100, warmups 0..10")
    if not 0 <= o.seed < 1 << 64 or (o.cpu is not None and (o.cpu < 0 or o.threads != 1)):
        p.error("require a u64 seed; --cpu requires a nonnegative CPU and --threads 1")
    if any(not math.isfinite(x) or x <= 0 for x in (o.memory_budget_gib, o.reserve_gib)) or not 0.1 <= o.poll_seconds <= 15:
        p.error("require positive finite memory limits and poll-seconds 0.1..15")
    o.out = (ROOT / o.out).resolve()
    if o.out == ROOT or o.out in ROOT.parents:
        p.error("use a separate result directory")
    o.worker_bin, o.time_bin = o.worker_bin.resolve(), o.time_bin.resolve()
    return o


def positive(value, label, zero=False):
    require(type(value) in (int, float) and math.isfinite(value) and (value >= 0 if zero else value > 0), f"invalid {label}")
    return value


def expect(record, key, value):
    require(type(record.get(key)) is type(value) and record[key] == value, f"configuration mismatch: {key}")


def parse_worker(text, o, case, bits, rss):
    records = [json.loads(line) for line in text.splitlines() if line.strip()]
    require(records and all(isinstance(r, dict) for r in records), "missing worker JSON records")
    meta, rows = records[0], records[1:]
    expected = {"type": "metadata", "system": "mikros", "instances": INSTANCES[case], "value_bits": bits,
                "log_gates": o.log_n, "gates_per_instance": 1 << o.log_n,
                "total_gates": (1 << o.log_n) * INSTANCES[case], "threads": o.threads,
                "seed": o.seed, "repetitions": o.reps, "warmups": o.warmups, "pcs_backend": PCS,
                "kappa": 128 // bits}
    for key, value in expected.items():
        expect(meta, key, value)
    arith = meta.get("arithmetization")
    require(arith == "PLONK" if case == "single" else arith in ("Packed R1CS", "R1CS"), "unexpected arithmetization")
    profile = meta.get("pcs_profile")
    require(isinstance(profile, dict), "missing FRI profile")
    for key, value in (("field_bits", 128), ("packed_word_bits", 128), ("rate", 0.5), ("native_queries", 241)):
        expect(profile, key, value)
    if case == "single":
        for key, value in packing_geometry(o.log_n, bits).items():
            expect(meta, key, value)
        require(meta.get("permutation_proof") is True, "single PLONK must authenticate copy wiring")
    else:
        for key, value in (("r1cs_rows", 4 << o.log_n), ("instance_packing_width", 128//bits),
                           ("packed_groups", 2*bits), ("witness_words", (4 << o.log_n)*(2*bits)),
                           ("public_words", 0), ("permutation_proof", False)):
            expect(meta, key, value)
        matrix = meta.get("matrix_authentication")
        require(isinstance(matrix, str) and "public" in matrix and "C=I" in matrix,
                "batch must use the shared public matrices with C=I")
    require(len(rows) == o.reps + o.warmups, "sample count mismatch")
    for index, row in enumerate(rows):
        warmup = index < o.warmups
        for key, value in (("type", "sample"), ("value_bits", bits), ("warmup", warmup),
                           ("repetition", index if warmup else index-o.warmups),
                           ("verified", True), ("transcript_match", True), ("pcs_complete", True)):
            expect(row, key, value)
        for key in ("instances", "log_gates", "gates_per_instance", "total_gates", "threads", "seed"):
            if key in row:
                expect(row, key, expected[key])
        require(type(row.get("proof_bytes")) is int, "proof size must be integral bytes")
        positive(row["proof_bytes"], "proof bytes")
        positive(row.get("verify_ms"), "verifier time")
        positive(row.get("peak_rss_kib"), "prover VmHWM")
        timers = row.get("times")
        require(isinstance(timers, dict), "missing online timers")
        positive(timers.get("prover_ms"), "prover time")
        for key, value in timers.items():
            if key.endswith("_ms"):
                positive(value, key, zero=True)
        if "pcs_claims" in timers:
            expect(timers, "pcs_claims", 2 if case == "single" else 1)
        if case == "single":
            expect(timers, "witness_claims", 11)
            expect(timers, "public_claims", 5 + 3 * ((o.log_n + 2 + bits - 1) // bits))
        for group in (("witness", "public") if case == "single" else ("witness",)):
            key = group + "_pcs_proof_bytes"
            require(type(row.get(key)) is int and 0 < row[key] < row["proof_bytes"], f"missing complete {group} PCS opening")
    measured = [r for r in rows if not r["warmup"]]
    summary = {"prover_ms": stats([r["times"]["prover_ms"] for r in measured]),
               "verify_ms": stats([r["verify_ms"] for r in measured]),
               "proof_bytes": stats([r["proof_bytes"] for r in measured]), "ram_mib": positive(rss, "process RSS") / 1024}
    return {"metadata": meta, "samples": rows, "peak_rss_kib": rss}, summary


def parse_case(folder, o, case, bits):
    resource = (folder / "resource.txt").read_text(encoding="utf-8")
    m = re.search(r"Maximum resident set size \(kbytes\):\s*(\d+)", resource)
    require(m is not None and re.search(r"^\s*Exit status:\s*0\s*$", resource, re.M), "missing GNU time RSS or successful exit")
    return parse_worker((folder / "stdout.txt").read_text(encoding="utf-8"), o, case, bits, int(m.group(1)))


def owned_rss_gib(pid):
    """RSS of the fresh GNU-time process and its descendants, never other users' jobs."""
    pending, seen, total = [pid], set(), 0
    while pending:
        current = pending.pop()
        if current in seen:
            continue
        seen.add(current)
        try:
            status = Path(f"/proc/{current}/status").read_text()
            m = re.search(r"^VmRSS:\s*(\d+)\s+kB$", status, re.M)
            total += int(m.group(1)) if m else 0
            pending.extend(map(int, Path(f"/proc/{current}/task/{current}/children").read_text().split()))
        except FileNotFoundError:
            pass
    return total / 2**20


def run_guarded(command, *, folder, env, o, cpu, available, on_start, label):
    require(os.name == "posix" and hasattr(os, "killpg"), "workers require Linux process groups")
    if checked_available(available) < o.reserve_gib:
        return {"exit_code": None, "guarded": True, "guard_reason": "host reserve before launch"}
    events, started = queue.Queue(), time.monotonic()
    process = subprocess.Popen(command, cwd=ROOT, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                               text=True, encoding="utf-8", errors="replace", bufsize=1, start_new_session=True,
                               preexec_fn=(lambda: os.sched_setaffinity(0, {cpu})) if cpu is not None else None)
    def read(name, stream):
        for line in stream:
            events.put((name, line))
        events.put((name, None))
    guarded, reason, peak, minimum = False, None, 0.0, float("inf")
    try:
        on_start(process.pid)
        for name, stream in (("stdout", process.stdout), ("stderr", process.stderr)):
            threading.Thread(target=read, args=(name, stream), daemon=True).start()
        with (folder / "stdout.txt").open("w", encoding="utf-8") as out, \
             (folder / "stderr.txt").open("w", encoding="utf-8") as err, \
             (folder / "memory.jsonl").open("w", encoding="utf-8") as mem:
            streams, closed = {"stdout": out, "stderr": err}, set()
            next_poll = next_heartbeat = started
            while len(closed) < 2 or process.poll() is None:
                current = time.monotonic()
                if current >= next_poll:
                    avail, resident = checked_available(available), owned_rss_gib(process.pid)
                    minimum, peak = min(minimum, avail), max(peak, resident)
                    mem.write(json.dumps({"elapsed_seconds": current-started, "available_gib": avail,
                                          "owned_rss_gib": resident}) + "\n")
                    mem.flush()
                    next_poll = current + o.poll_seconds
                    if not guarded and (avail < o.reserve_gib or resident > o.memory_budget_gib):
                        reason = "host reserve" if avail < o.reserve_gib else "worker memory budget"
                        guarded = stop_owned_group(process)
                        print(f"[{label}] {reason} triggered; this worker group stopped", flush=True)
                if current >= next_heartbeat:
                    print(f"[{label}] running {current-started:.0f}s; owned RSS {peak:.2f} GiB; host available >= {minimum:.2f} GiB", flush=True)
                    next_heartbeat = current + 15
                try:
                    name, line = events.get(timeout=min(o.poll_seconds, 0.25))
                except queue.Empty:
                    continue
                if line is None:
                    closed.add(name)
                    continue
                streams[name].write(line)
                streams[name].flush()
                if name == "stderr":
                    print(f"[{label}] {line.rstrip()}", flush=True)
                else:
                    try:
                        row = json.loads(line)
                        if row.get("type") == "sample":
                            kind = "warmup" if row["warmup"] else "sample"
                            count = o.warmups if row["warmup"] else o.reps
                            print(f"[{label}] {kind} {row['repetition']+1}/{count}: prover={row['times']['prover_ms']:.3f} ms "
                                  f"verifier={row['verify_ms']:.3f} ms proof={row['proof_bytes']} B "
                                  f"prover VmHWM={row['peak_rss_kib']/1024:.2f} MiB", flush=True)
                    except (ValueError, KeyError, TypeError):
                        print(f"[{label}] stdout: {line.rstrip()}", flush=True)
        return {"exit_code": process.wait(), "guarded": guarded, "guard_reason": reason,
                "maximum_owned_rss_gib": peak, "minimum_available_gib": minimum,
                "elapsed_seconds": time.monotonic()-started}
    except BaseException:
        stop_owned_group(process)
        raise
    finally:
        process.stdout.close()
        process.stderr.close()


def empty_metrics():
    return dict.fromkeys(METRICS)


def metrics(summary):
    return {key: summary[key]["median"] if key != "ram_mib" else summary[key] for key in METRICS}


def memory_estimate(log_n, bits, case):
    """Conservative preflight model, never a reported RAM measurement."""
    n = 1 << log_n
    if case == "multiple":
        # 4N R1CS rows, 256/(128/b)=2b packed instance groups.
        witness_bytes = 16 * 4 * n * (2 * bits)
        return {"model": "packed-r1cs-fri-v1", "witness_bytes": witness_bytes,
                "estimated_gib": witness_bytes * 12 / 2**30 + 8}
    shape = packing_geometry(log_n, bits)
    source_bytes = 16 * (shape["witness_words"] + shape["public_words"])
    copy_bytes = n * {32: 160, 64: 192, 128: 384, 256: 512}[shape["tag_bits"]]
    return {"model": "single-plonk-fri-conservative-v1", "source_bytes": source_bytes,
            "estimated_gib": (source_bytes * 12 + copy_bytes) / 2**30 + 8}


def raw_hashes(folder):
    return {p.name: digest(p) for p in sorted(folder.iterdir()) if p.is_file()}


def configuration(o):
    return {k: getattr(o, k) for k in ("bits", "log_n", "threads", "reps", "warmups", "seed", "memory_budget_gib", "reserve_gib", "poll_seconds")}


def render_report(doc):
    cfg = doc["config"]
    lines = [f"Mikros field-width experiment | 2^{cfg['log_n']} gates per instance | {cfg['threads']} thread(s)",
             "Single: PLONK with copy argument; batch: 256 independent instances, Packed R1CS with public matrices.",
             "Both use F128 additive FRI. All four metrics are TOTAL costs; batch values are not divided by 256.",
             "kappa=128/b; at b=128, kappa=1 and there is no subfield packing gain.",
             "Online prover includes witness commit, reductions, complete openings, grinding, release and serialization.",
             "Public preprocessing and witness generation are offline; required public opening stays online.",
             "RAM: fresh-process GNU time peak RSS, including setup, warmups, proving and verification.",
             "Missing cases remain blank; memory guards are not measured OOM results.",
             f"Sampling: {cfg['warmups']} warmups, {cfg['reps']} measured proof(s) per case.", "",
             f"{'Case':<10} {'b':>4} {'kappa':>6} {'Status':<16} {'Prover ms':>14} {'Verifier ms':>13} {'Proof KiB':>12} {'RAM MiB':>12}"]
    for c in doc["cases"]:
        m = c["metrics"]
        values = ["" if m[k] is None else f"{m[k]/(1024 if k == 'proof_bytes' else 1):.3f}" for k in METRICS]
        lines.append(f"{c['case']:<10} {c['value_bits']:>4} {c['kappa']:>6} {c['status']:<16} {values[0]:>14} {values[1]:>13} {values[2]:>12} {values[3]:>12}")
        if c["reason"]:
            lines.append("  " + c["reason"])
    return "\n".join(lines) + "\n"


def checkpoint(o, doc):
    doc["updated_at"] = now()
    write_atomic(o.out / "checkpoint.json", json.dumps(doc, indent=2, allow_nan=False) + "\n")
    stream = io.StringIO(newline="")
    names = ["case", "instances", "arithmetization", "log_n", "value_bits", "kappa", "gates_per_instance", "total_gates", "status", *METRICS]
    writer = csv.DictWriter(stream, fieldnames=names)
    writer.writeheader()
    for c in doc["cases"]:
        row = {k: c[k] for k in names if k not in METRICS}
        row.update(c["metrics"])
        writer.writerow(row)
    write_atomic(o.out / "results.csv", stream.getvalue())
    write_atomic(o.out / "summary.log", render_report(doc))


def run(o, *, available=available_memory_gib, launcher=run_guarded, affinity=None):
    require(o.worker_bin.is_file() and o.time_bin.is_file(), "missing worker or GNU time executable")
    allowed = sorted(os.sched_getaffinity(0)) if affinity is None else sorted(affinity)
    require(len(allowed) >= o.threads, "requested threads exceed allowed CPU affinity")
    cpu = (o.cpu if o.cpu is not None else allowed[0]) if o.threads == 1 else None
    require(cpu is None or cpu in allowed, "selected CPU is outside allowed affinity")
    env = environment(o.threads)
    for key in ("RAAFOLD_PROFILE_STAGES", "ADDITIVE_FRI_PROFILE_STAGES", "MIKROS_PERMUTATION_PROFILE"):
        env.pop(key, None)
    identity = {"worker_sha256": digest(o.worker_bin), "time_sha256": digest(o.time_bin),
                "source_sha256": {p.name: digest(p) for p in [Path(__file__), Path(__file__).with_name("paper_bench.py"),
                                  Path(__file__).with_name("plonk_bitwidth_bench.py"), Path(__file__).with_name("plonk_bitwidth_sweep_bench.py")]},
                "cpu_affinity": [cpu] if cpu is not None else allowed,
                "worker_environment": {k: v for k, v in env.items() if k.startswith(("FLOCK_", "MIKROS_", "RAYON_", "LIG_", "E2E_")) or k == "LC_ALL"}}
    o.out.mkdir(parents=True, exist_ok=True)
    with output_lock(o.out):
        path = o.out / "checkpoint.json"
        if o.resume:
            doc = json.loads(path.read_text(encoding="utf-8"))
            require(doc.get("schema_version") == 1 and doc.get("experiment") == "bitwidth-instances", "unexpected checkpoint")
            require(doc["config"] == configuration(o) and doc["identity"] == identity, "resume executable, configuration, source or environment differs")
            require([(c["value_bits"], c["case"]) for c in doc["cases"]] == [(b, c) for b in o.bits for c in CASES], "resume grid differs")
            for c in doc["cases"]:
                require(c["status"] in STATUSES, "unknown checkpoint status")
                for key, value in (("instances", INSTANCES[c["case"]]), ("kappa", 128//c["value_bits"]),
                                   ("log_n", o.log_n), ("gates_per_instance", 1 << o.log_n),
                                   ("total_gates", (1 << o.log_n)*INSTANCES[c["case"]]),
                                   ("arithmetization", ARITHMETIZATION[c["case"]])):
                    expect(c, key, value)
                require(c.get("memory_estimate") == memory_estimate(o.log_n, c["value_bits"], c["case"]), "resume memory model differs")
                require(c.get("active_pid") is None or not process_alive(c["active_pid"]), "previous worker may still be active")
                if c["status"] == "ok":
                    folder = (o.out / c["raw_directory"]).resolve()
                    require(o.out in folder.parents and raw_hashes(folder) == c["raw_sha256"], "resume raw artifact mismatch")
                    report, summary = parse_case(folder, o, c["case"], c["value_bits"])
                    require(report == c["report"] and summary == c["summary"] and metrics(summary) == c["metrics"], "resume measurements differ")
                else:
                    require(c["metrics"] == empty_metrics(), "failed case contains measurements")
        else:
            require(not path.exists() and not (o.out / "raw").exists(), "results exist; use --resume or a new directory")
            doc = {"schema_version": 1, "experiment": "bitwidth-instances", "config": configuration(o),
                   "identity": identity, "created_at": now(), "completed": False, "cases": []}
            for b in o.bits:
                for case in CASES:
                    doc["cases"].append({"case_id": f"{case}-b{b}", "case": case, "instances": INSTANCES[case],
                        "arithmetization": ARITHMETIZATION[case], "log_n": o.log_n, "value_bits": b, "kappa": 128//b,
                        "gates_per_instance": 1 << o.log_n, "total_gates": (1 << o.log_n)*INSTANCES[case],
                        "status": "pending", "reason": None, "metrics": empty_metrics(), "report": None,
                        "summary": None, "raw_directory": None, "raw_sha256": {}, "attempts": [], "active_pid": None,
                        "memory_estimate": memory_estimate(o.log_n, b, case)})
        doc["completed"] = False
        checkpoint(o, doc)
        for c in doc["cases"]:
            if c["status"] == "ok":
                print(f"[{c['case_id']}] reused validated result", flush=True)
                continue
            c.update(status="pending", reason=None, metrics=empty_metrics(), report=None, summary=None, active_pid=None)
            avail = checked_available(available)
            estimate = c["memory_estimate"]["estimated_gib"]
            if estimate > min(o.memory_budget_gib, avail-o.reserve_gib):
                c.update(status="memory_skipped", reason=f"Estimated {estimate:.2f} GiB exceeds budget {o.memory_budget_gib:.2f} GiB or available-minus-reserve {avail-o.reserve_gib:.2f} GiB; not launched, not measured OOM.")
                checkpoint(o, doc)
                continue
            folder = o.out / "raw" / c["case_id"] / str(time.time_ns())
            folder.mkdir(parents=True)
            c.update(status="running", raw_directory=str(folder.relative_to(o.out)))
            command = [str(o.time_bin), "-v", "-o", str(folder / "resource.txt"), str(o.worker_bin),
                       "--worker", "--case", c["case"], "--bits", str(c["value_bits"]), "--log-gates", str(o.log_n),
                       "--threads", str(o.threads), "--repetitions", str(o.reps), "--warmups", str(o.warmups), "--seed", str(o.seed)]
            write_atomic(folder / "command.json", json.dumps(command) + "\n")
            def on_start(pid):
                c["active_pid"] = pid
                checkpoint(o, doc)
            print(f"[{c['case_id']}] start: {c['instances']} instance(s), b={c['value_bits']}, kappa={c['kappa']}", flush=True)
            checkpoint(o, doc)
            try:
                result = launcher(command, folder=folder, env=env, o=o, cpu=cpu, available=available, on_start=on_start, label=c["case_id"])
                c["attempts"].append(result)
                if result["guarded"]:
                    c.update(status="guarded", reason=f"Memory guard: {result.get('guard_reason')}; no successful measurement.")
                elif result["exit_code"] != 0:
                    c.update(status="error", reason=f"Worker exited {result['exit_code']}; not classified as OOM without guard evidence.")
                else:
                    report, summary = parse_case(folder, o, c["case"], c["value_bits"])
                    c.update(status="ok", report=report, summary=summary, metrics=metrics(summary))
                    print(f"[{c['case_id']}] verified: prover={c['metrics']['prover_ms']:.3f} ms, verifier={c['metrics']['verify_ms']:.3f} ms, "
                          f"proof={c['metrics']['proof_bytes']} B, process peak RSS={c['metrics']['ram_mib']:.2f} MiB", flush=True)
            except (OSError, ValueError, KeyError, TypeError) as error:
                c.update(status="error", reason=str(error))
            finally:
                c["active_pid"] = None
                c["raw_sha256"] = raw_hashes(folder)
                checkpoint(o, doc)
        doc["completed"] = True
        checkpoint(o, doc)
    print(render_report(doc), end="", flush=True)
    return doc, int(any(c["status"] == "error" for c in doc["cases"]))


def main(argv=None):
    o = parse_options(argv)
    _, code = run(o)
    if not o.no_plots:
        from plot_bitwidth_instances import export
        export(o.out / "checkpoint.json", o.out / "plots")
    return code


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, KeyError, TypeError, ImportError) as error:
        print(f"bitwidth-instance experiment failed: {error}", file=sys.stderr)
        sys.exit(1)
