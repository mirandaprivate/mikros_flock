#!/usr/bin/env python3
"""Bounded native-64 engineering ladder on the frozen column/paired protocol.

Run from a new source snapshot containing this runner in source_manifest.json.
Stages have separate default log directories; existing directories are refused.
--selected-json accepts the original seven keys or all ten engineering keys;
the original format explicitly receives zero for the three refinement switches.
After a stage builds the binaries, --skip-build verifies its snapshot-local
engineering-build.json receipt before reusing those exact executable bytes.
Unit tests are run separately by the experiment owner; this runner builds the
two examples and runs complete baseline/selected tamper checks.
"""

import argparse
import json
import os
from pathlib import Path
import signal
import statistics
import subprocess
import time

from run_column_projection import (
    CPU_SPEC, FEATURES, PHASES, SHARED_DIGESTS, TOOLCHAIN,
    checked_hash, digest, number, preflight, records, require, save,
)


PROOF_HASH = "a8c04f1e0baf0fa22dc2c8e1dff067426712a11cb8c3d64d41c822fa09299161"
COMMITMENT_HASH = "c6809c5cefa78402d6b6120e2ad65bd9a50564762935c0afc4427c2bc947dada"
PIOP_HASH = "3a7a17aaff16a5a8933ac769e29b1c06a2733214bfa12db2ddbcb753d077d79f"
CSC = "MIKROS_PUBLIC_CSC_SINGLE"
SUPPORT = "MIKROS_PCS_LOW_BIT_SUPPORT"
TABLE = "MIKROS_PCS_REUSE_UNIT_TABLE"
WIDE = "MIKROS_HADAMARD_WIDE_PACK"
GROUP = "MIKROS_HADAMARD_GROUP_TILES"
ROWS = "MIKROS_HADAMARD_TILE_ROWS"
IP = "MIKROS_IP_BATCH_PARALLEL"
BLOCKED = "MIKROS_IMAGE_BLOCKED_WEIGHTS"
SERIAL = "MIKROS_SERIAL_DEINTERLEAVE"
GFNI_QUOTIENT = "MIKROS_HADAMARD_GFNI_QUOTIENT"
ORIGINAL_SWITCHES = (CSC, SUPPORT, TABLE, WIDE, GROUP, IP)
REFINEMENT_SWITCHES = (BLOCKED, SERIAL, GFNI_QUOTIENT)
SWITCHES = ORIGINAL_SWITCHES + REFINEMENT_SWITCHES
BASELINE = {**{key: "0" for key in SWITCHES}, ROWS: "1024"}
COMMON_ENV = {"RUSTFLAGS": "-C target-cpu=native", "RAYON_NUM_THREADS": "64",
              "FLOCK_PCS_PACKED_DIRECT": "1", "FLOCK_WP_UNINIT": "1"}


def selected_environment(path):
    value = json.loads(path.read_text(encoding="utf-8-sig"))
    original_keys = set(ORIGINAL_SWITCHES) | {ROWS}
    require(type(value) is dict and set(value) in (original_keys, set(BASELINE)),
            "selected JSON must contain all seven original keys or all ten keys")
    if set(value) == original_keys:
        value = {**value, **{key: "0" for key in REFINEMENT_SWITCHES}}
    require(all(type(value[key]) is str and value[key] in ("0", "1") for key in SWITCHES), "selected switches must be string 0 or 1")
    require(value[ROWS] in ("512", "1024", "2048"), "selected TILE_ROWS must be string 512, 1024 or 2048")
    return value


def ladder(ip_tile_rows):
    modes = {"baseline": dict(BASELINE)}
    current = dict(BASELINE)
    for name, key in (("csc", CSC), ("pcs_support", SUPPORT), ("pcs_table", TABLE), ("wide", WIDE)):
        current = {**current, key: "1"}
        modes[name] = current
    for rows in (512, 1024, 2048):
        modes[f"tiles{rows}"] = {**current, GROUP: "1", ROWS: str(rows)}
    modes["ip"] = {**modes[f"tiles{ip_tile_rows}"], IP: "1"}
    return modes


def stage_modes(stage, selected=None, ip_tile_rows=1024):
    all_modes = ladder(ip_tile_rows)
    if stage == "ablation":
        return {**all_modes, "flock": dict(BASELINE)}
    if stage == "formal":
        require(selected is not None, "formal requires --selected-json")
        return {"baseline": dict(BASELINE), "selected": dict(selected), "flock": dict(BASELINE)}
    if stage == "refinement":
        require(selected is not None, "refinement requires --selected-json")
        # The input's original seven keys define the fixed selected baseline.
        # Reset all three new switches even when the input uses the ten-key form.
        current = {**selected, **{key: "0" for key in REFINEMENT_SWITCHES}}
        modes = {"selected": current}
        for name, key in (("blocked", BLOCKED), ("serial", SERIAL), ("gfni", GFNI_QUOTIENT)):
            current = {**current, key: "1"}
            modes[name] = current
        return {**modes, "flock": dict(BASELINE)}
    require(stage == "profile", "unknown stage")
    modes = {name: all_modes[name] for name in ("baseline", "csc")}
    if selected is not None:
        modes["selected"] = dict(selected)
    return modes


def orders(modes):
    names = list(modes)
    return [names[(batch * len(names)) // 3:] + names[:(batch * len(names)) // 3] for batch in range(3)]


def command(binaries, mode, reps, warmups=1, tamper=False):
    require(not tamper or mode != "flock", "Mikros-only tamper command")
    args = [binaries["flock"], "--compressions", "256", "--profile", "fast"] if mode == "flock" else [
        binaries["mikros"], "--reduction", "column", "--opening", "paired"]
    args += ["--threads", "64", "--reps", str(reps), "--warmups", str(warmups)]
    return args + (["--tamper"] if tamper else [])


def read_samples(path, mode, reps, warmups=1, tamper=False):
    meta, rows = records(path)
    require(rows[0] is meta and sum(row.get("record") == "metadata" for row in rows) == 1, "metadata record shape")
    require(meta["fixture"] == "sha256-independent-compressions" and meta["sha_compressions"] == 256, "wrong SHA workload")
    require(meta["threads"] == meta["all_core_threads"] == 64 and FEATURES.issubset(meta["target_features"]), "wrong native64 features/pools")
    require(meta["r1cs_c_identity"] is True and meta["r1cs_k_log"] == 15 and meta["native_k_skip"] == 6, "wrong native public R1CS")
    require("public CSC" in meta["matrix_authentication"], "matrix authentication differs")
    expected_pcs = {"m": 23, "profile": "fast", "log_batch_size": 6, "log_inv_rate": 1,
                    "merkle_hash": "blake3", "num_lanes": 50 if mode == "flock" else None}
    require(meta["pcs_params"] == expected_pcs, "native PCS parameters differ")
    require(meta["pcs_queries"] == [244, 79, 48], "native PCS query counts differ")
    for key in SHARED_DIGESTS:
        checked_hash(meta[key], key)
    if mode != "flock":
        require(meta["schema_version"] == 2 and meta["reduction"] == "column", "wrong reduction metadata")
        require(meta["zero_knowledge"] is False and meta["packed_direct"] == "1", "wrong proof mode")
    samples = [row for row in rows if row.get("record") == "sample"]
    negatives = [row for row in rows if row.get("record") == "negative_tests"]
    require(len(rows) == 1 + len(samples) + len(negatives), "unexpected JSONL records")
    require(len(samples) == reps + warmups and [r["iteration"] for r in samples] == list(range(reps + warmups)), "wrong iterations/sample count")
    require(len(negatives) == (1 if tamper else 0), "negative record count differs")
    for row in samples:
        require(type(row["warmup"]) is bool and row["warmup"] == (row["iteration"] < warmups), "wrong warmup flag")
        require(row["verified"] is True and row["transcript_match"] is True, "full verification/Fiat-Shamir failed")
        require(type(row["proof_bytes"]) is int and row["proof_bytes"] > 0, "invalid proof bytes")
        for key in ("proof_blake3", "commitment_blake3"):
            checked_hash(row[key], key)
        number(row["verify_ms"], "verify_ms", True)
        if mode == "flock":
            require(row["system"] == "flock-circuit-e2e", "wrong FLOCK sample")
            require(row["proof_blake3"] == "d9d8edac4085a9c75619e855c732891be59cb2561f555955da4cddb4a1efd823"
                    and row["proof_bytes"] == 310955, "native FLOCK proof changed from frozen baseline")
            number(row["prove_with_commit_ms"], "prove_with_commit_ms", True)
        else:
            require(row["system"] == "mikros" and row["reduction"] == "column" and row["opening"] == "paired", "wrong Mikros sample")
            require(row["proof_blake3"] == PROOF_HASH and row["proof_bytes"] == 376547, "optimization changed frozen proof bytes")
            require(row["commitment_blake3"] == COMMITMENT_HASH and row["piop_blake3"] == PIOP_HASH, "optimization changed commitment/F128 reduction")
            timings = row["timings"]
            number(timings["prover_ms"], "prover_ms", True)
            for phase in PHASES:
                number(timings[phase], phase)
            require(sum(timings[key] for key in PHASES) <= timings["prover_ms"] + 1e-5, "phase sum exceeds prover scope")
            require(all(timings[key] == count for key, count in {"source_claims": 6, "ring_claims": 2, "inner_openings": 1}.items()), "claim counts differ")
    if tamper:
        require(negatives[0]["opening"] == "paired" and negatives[0]["rejected"] == 36, "expected 36 rejected tamper cases")
    return meta, samples, negatives


def summarize(logs, modes, reps, validation_modes, stage):
    result = {"stage": stage, "instances": 256, "threads": 64, "batches": 3,
              "reps_per_batch": reps, "warmups_per_batch": 1, "mode_environments": modes,
              "process_order_by_batch": orders(modes), "modes": {}, "negative_tests": {},
              "frozen_column_proof_blake3": PROOF_HASH, "same_protocol_and_mikros_proof": True}
    reference = None
    for mode in modes:
        samples, metadata = [], []
        for batch in range(3):
            meta, batch_samples, _ = read_samples(logs / f"{mode}-{batch}.jsonl", mode, reps)
            shared = {key: meta[key] for key in SHARED_DIGESTS}
            if reference is None:
                reference = shared
            require(shared == reference, "mode uses different fixture/circuit/R1CS/prepared witness")
            samples.extend(batch_samples)
            metadata.append(meta)
        require(all(len({row[key] for row in samples}) == 1 for key in ("proof_blake3", "proof_bytes", "commitment_blake3")), "unstable proof/commitment across batches")
        formal = [row for row in samples if not row["warmup"]]
        require(len(formal) == 3 * reps, "formal repetition count")
        fields = ("prove_with_commit_ms",) if mode == "flock" else ("prover_ms",) + PHASES
        times = formal if mode == "flock" else [row["timings"] for row in formal]
        summary = {key: statistics.median(row[key] for row in times) for key in fields}
        summary.update(samples=len(formal), warmup_samples=3, verify_ms=statistics.median(row["verify_ms"] for row in formal),
                       prover_range_ms=[min(row[fields[0]] for row in times), max(row[fields[0]] for row in times)],
                       proof_bytes=formal[0]["proof_bytes"], proof_blake3=formal[0]["proof_blake3"], metadata=metadata)
        result["modes"][mode] = summary
    for label in validation_modes:
        meta, _, negatives = read_samples(logs / f"validation-{label}.jsonl", label, 1, 0, True)
        require({key: meta[key] for key in SHARED_DIGESTS} == reference, "validation statement differs")
        result["negative_tests"][label] = negatives[0]["rejected"]
    reference_mode = "selected" if stage == "refinement" else "baseline"
    reference_time = result["modes"][reference_mode]["prover_ms"]
    result["reference_mode"] = reference_mode
    for mode, row in result["modes"].items():
        elapsed = row["prove_with_commit_ms"] if mode == "flock" else row["prover_ms"]
        row[f"{reference_mode}_over_mode"] = reference_time / elapsed
    result["shared_statement_digests"] = reference
    return result


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--stage", required=True, choices=("profile", "ablation", "refinement", "formal"))
    parser.add_argument("--target", required=True, type=Path)
    parser.add_argument("--logs", type=Path)
    parser.add_argument("--selected-json", type=Path)
    parser.add_argument("--ip-tile-rows", type=int, choices=(512, 1024, 2048), default=1024)
    parser.add_argument("--skip-build", action="store_true")
    parser.add_argument("--compare-pcs", action="store_true", help="formal only: add the selected configuration with both Mikros PCS switches disabled")
    parser.add_argument("--command-timeout-seconds", type=int, default=300)
    args = parser.parse_args(argv)
    require(1 <= args.command_timeout_seconds <= 1800, "command timeout must be 1..1800 seconds")
    root, target = Path.cwd().resolve(), args.target.resolve()
    selected = selected_environment(args.selected_json) if args.selected_json else None
    modes = stage_modes(args.stage, selected, args.ip_tile_rows)
    if args.compare_pcs:
        require(args.stage == "formal", "--compare-pcs requires formal")
        require(selected[SUPPORT] == selected[TABLE] == "1", "PCS comparison requires both switches selected")
        modes = {"baseline": modes["baseline"], "core": {**selected, SUPPORT: "0", TABLE: "0"},
                 "selected": modes["selected"], "flock": modes["flock"]}
    require(args.stage != "ablation" or selected is None, "ablation uses the explicit ladder; selected JSON belongs to profile/formal")
    reps = 3 if args.stage == "profile" else 6
    candidate = modes["gfni"] if args.stage == "refinement" else selected or modes.get("ip") or modes["csc"]
    validation_modes = {"baseline": dict(BASELINE), "selected": dict(candidate)}
    logs = (root / (args.logs or f"results-engineering-{args.stage}")).resolve()
    require(logs.is_relative_to(root), "logs must be inside this snapshot")
    manifest_path = root / "source_manifest.json"
    source_files = json.loads(manifest_path.read_text(encoding="utf-8"))["files_sha256"]
    own_name = Path(__file__).resolve().relative_to(root).as_posix()
    require(len(source_files) >= 400 and own_name in source_files, "snapshot must include 400+ source files and this runner")
    def verify_sources():
        for name, expected in source_files.items():
            path = (root / name).resolve()
            require(path.is_relative_to(root) and path.is_file() and digest(path) == expected, f"source mismatch: {name}")
    verify_sources()
    logs.mkdir(exist_ok=False)
    base_env = {key: value for key, value in os.environ.items()
                if not key.startswith(("FLOCK_", "MIKROS_", "E2E_", "LIG_", "RAYON_"))
                and key not in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_RUSTFLAGS",
                                "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER", "PCS_TRACE", "LD_PRELOAD")}
    base_env.update(COMMON_ENV, CARGO_TARGET_DIR=str(target))
    base_env["PATH"] = str(Path.home() / ".cargo/bin") + ":" + base_env.get("PATH", "")
    def environment(toggles):
        env = {**base_env, **toggles}
        if args.stage == "profile":
            env.update(MIKROS_PROFILE="1", PCS_TRACE="1")
        return env
    def recorded_env(env):
        return {key: value for key, value in env.items() if key.startswith(("FLOCK_", "MIKROS_", "RAYON_"))
                or key in ("RUSTFLAGS", "CARGO_TARGET_DIR", "PCS_TRACE")}
    status = {"stage": args.stage, "state": "starting", "completed": []}
    binaries = {"mikros": str(target / "release/examples/e2e_sha256_paired"),
                "flock": str(target / "release/examples/e2e_flock_baseline")}
    receipt_path = root / "engineering-build.json"
    def run(name, invocation, toggles, build=False):
        status["state"] = name
        save(logs / "status.json", status)
        env = environment(toggles)
        if not build:
            save(logs / f"{name}.preflight.json", preflight())
            invocation = ["/usr/bin/time", "-v", "-o", str(logs / f"{name}.resource.txt"), "taskset", "-c", CPU_SPEC] + invocation
        save(logs / f"{name}.command.json", invocation)
        save(logs / f"{name}.environment.json", recorded_env(env))
        started = time.monotonic()
        with (logs / f"{name}.jsonl").open("w") as out, (logs / f"{name}.stderr.log").open("w") as err:
            process = subprocess.Popen(invocation, env=env, cwd=root, stdout=out, stderr=err, start_new_session=True)
            try:
                process.wait(timeout=900 if build else args.command_timeout_seconds)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
        event = {"name": name, "exit_code": process.returncode, "wall_seconds": time.monotonic() - started}
        status["completed"].append(event)
        save(logs / "status.json", status)
        print(json.dumps(event), flush=True)
        require(process.returncode == 0, f"stage failed or timed out: {name}")
    try:
        save(logs / "hardware.json", {"cpuinfo": Path("/proc/cpuinfo").read_text(), "meminfo": Path("/proc/meminfo").read_text(),
             "affinity": CPU_SPEC, "utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
             "environment": recorded_env(environment(BASELINE))})
        save(logs / "preflight.json", preflight())
        save(logs / "run_plan.json", {"stage": args.stage, "modes": modes, "order": orders(modes),
             "validation_modes": validation_modes, "reps": reps, "warmups": 1, "batches": 3,
             "profiling_enabled": args.stage == "profile", "source_files": len(source_files),
             "source_manifest_sha256": digest(manifest_path), "selected_json_sha256": digest(args.selected_json) if args.selected_json else None})
        if args.skip_build:
            receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
            require(receipt["source_manifest_sha256"] == digest(manifest_path) and receipt["toolchain"] == TOOLCHAIN
                    and receipt["rustflags"] == COMMON_ENV["RUSTFLAGS"], "skip-build receipt belongs to another snapshot/configuration")
            require(receipt["binary_sha256"] == {path: digest(Path(path)) for path in binaries.values()}, "skip-build executable bytes differ")
            save(logs / "build-reused.json", receipt)
        else:
            for name in source_files:
                os.utime(root / name, None)
            run("build", ["cargo", TOOLCHAIN, "build", "--release", "--offline", "--locked", "-j8", "-p", "mikros",
                          "--example", "e2e_sha256_paired", "--example", "e2e_flock_baseline"], BASELINE, True)
            receipt = {"source_manifest_sha256": digest(manifest_path), "toolchain": TOOLCHAIN,
                       "rustflags": COMMON_ENV["RUSTFLAGS"], "binary_sha256": {path: digest(Path(path)) for path in binaries.values()},
                       "build_logs": str(logs)}
            save(receipt_path, receipt)
        save(logs / "binary-sha256.json", receipt["binary_sha256"])
        for label, toggles in validation_modes.items():
            run(f"validation-{label}", command(binaries, label, 1, 0, True), toggles)
            read_samples(logs / f"validation-{label}.jsonl", label, 1, 0, True)
        for batch, sequence in enumerate(orders(modes)):
            for mode in sequence:
                run(f"{mode}-{batch}", command(binaries, mode, reps), modes[mode])
                read_samples(logs / f"{mode}-{batch}.jsonl", mode, reps)
        save(logs / "summary.json", summarize(logs, modes, reps, validation_modes, args.stage))
        verify_sources()
        require(receipt["binary_sha256"] == {path: digest(Path(path)) for path in binaries.values()}, "executable bytes changed during stage")
        status["state"] = "complete"
    except Exception as error:
        status.update(state="failed", error=str(error))
        raise
    finally:
        save(logs / "status.json", status)
        save(logs / "evidence_sha256.json", {path.name: digest(path) for path in sorted(logs.iterdir())
             if path.is_file() and path.name != "evidence_sha256.json"})


if __name__ == "__main__":
    main()
