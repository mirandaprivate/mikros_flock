#!/usr/bin/env python3
"""Frozen native-64 SHA-256 comparison: optimized Mikros, FLOCK skip6 and skip7.

Three rotating batches each contain one warmup plus six formal samples. Both
FLOCK modes use the same compiled executable with an explicit --skip argument.
The exact executables, source receipt, commands, environment and logs are kept.
"""
import argparse
import json
import os
from pathlib import Path
import shutil
import signal
import statistics
import subprocess
import time

from run_engineering import COMMON_ENV, read_samples
from run_column_projection import (
    CPU_SPEC, FEATURES, PHASES, SHARED_DIGESTS, TOOLCHAIN,
    checked_hash, digest, number, preflight, records, require, save,
)

MODES = ("mikros", "flock6", "flock7")
FLOCK6_PROOF = "d9d8edac4085a9c75619e855c732891be59cb2561f555955da4cddb4a1efd823"


def command(binaries, mode, validation=False):
    require(mode in MODES, "unknown mode")
    args = [binaries["mikros"], "--reduction", "column", "--opening", "paired"] if mode == "mikros" else [
        binaries["flock"], "--compressions", "256", "--profile", "fast", "--skip", mode[-1]]
    args += ["--threads", "64", "--reps", "1" if validation else "6", "--warmups", "0" if validation else "1"]
    return args + (["--tamper"] if validation else [])


def checked_records(path, mode, validation=False):
    if mode == "mikros":
        return read_samples(path, mode, 1 if validation else 6, 0 if validation else 1, validation)
    meta, rows = records(path)
    skip = int(mode[-1])
    require(rows[0] is meta and sum(r.get("record") == "metadata" for r in rows) == 1, "metadata shape")
    require(meta["fixture"] == "sha256-independent-compressions" and meta["sha_compressions"] == 256, "SHA workload")
    require(meta["system"] == "flock-circuit-e2e" and meta["independent_compressions"] is True, "FLOCK mode")
    require(meta["reps"] == (1 if validation else 6) and meta["warmups"] == (0 if validation else 1), "FLOCK run configuration")
    require(meta["threads"] == meta["all_core_threads"] == 64 and FEATURES.issubset(meta["target_features"]), "native64 features/pools")
    require(meta["r1cs_c_identity"] is True and meta["r1cs_k_log"] == 15 and meta["native_k_skip"] == 6, "native public R1CS")
    require(meta["baseline_skip"] == skip and meta["univariate_skip"] == skip, "actual FLOCK skip path")
    require("public CSC" in meta["matrix_authentication"], "public matrix authentication")
    require(meta["pcs_params"] == {"m": 23, "profile": "fast", "log_batch_size": 6, "log_inv_rate": 1,
            "merkle_hash": "blake3", "num_lanes": 50}, "native PCS settings/layout")
    require(meta["pcs_queries"] == [244, 79, 48] and meta["pcs_query_grinding_bits"] == [16, 16, 16]
            and meta["pcs_fold_grinding_bits"] == [0, 0, 0], "native PCS soundness configuration")
    require(meta["zero_knowledge_claim"] is False, "proof mode")
    require(meta["verification"] == "complete non-deferred verifier; no witness input; matrix and sigma obligations discharged", "complete verification obligations")
    for key in SHARED_DIGESTS:
        checked_hash(meta[key], key)
    samples = [r for r in rows if r.get("record") == "sample"]
    negatives = [r for r in rows if r.get("record") == "negative_tests"]
    require(len(rows) == 1 + len(samples) + len(negatives), "unexpected records")
    require(len(samples) == (1 if validation else 7) and [r["iteration"] for r in samples] == list(range(len(samples))), "iterations")
    require(len(negatives) == int(validation), "negative record count")
    for row in samples:
        require(row["system"] == "flock-circuit-e2e" and type(row["warmup"]) is bool
                and row["warmup"] == (not validation and row["iteration"] == 0), "sample flags")
        require(row["verified"] is True and row["transcript_match"] is True, "complete verifier/FS failed")
        require(type(row["proof_bytes"]) is int and row["proof_bytes"] > 0, "proof bytes")
        for key in ("proof_blake3", "commitment_blake3"):
            checked_hash(row[key], key)
        for key in ("prove_with_commit_ms", "verify_ms"):
            number(row[key], key, True)
        if skip == 6:
            require(row["proof_blake3"] == FLOCK6_PROOF and row["proof_bytes"] == 310955, "frozen skip6 proof changed")
    if validation:
        negative = negatives[0]
        require(negative["skip"] == skip and negative["rejected"] == 20, "expected 20 rejected FLOCK tamper cases")
        cases = negative["cases"]
        require(type(cases) is list and len(cases) == 20 and all(type(case) is str and case for case in cases)
                and len(set(cases)) == 20, "FLOCK tamper cases must be distinct and named")
    return meta, samples, negatives


def audit(logs):
    result = {"instances": 256, "threads": 64, "batches": 3, "warmups_per_batch": 1,
              "formal_repetitions": 18, "modes": {}, "negative_tests": {}, "same_statement": True}
    shared = None
    for mode in MODES:
        metadata, samples = [], []
        for batch in range(3):
            meta, rows, _ = checked_records(logs / f"{mode}-{batch}.jsonl", mode)
            identity = {key: meta[key] for key in SHARED_DIGESTS}
            shared = shared or identity
            require(identity == shared, "fixture/circuit/R1CS/prepared witness differ")
            metadata.append(meta)
            samples.extend(rows)
        require(all(len({r[key] for r in samples}) == 1 for key in ("proof_blake3", "proof_bytes", "commitment_blake3")), "proof/commitment unstable")
        formal = [r for r in samples if not r["warmup"]]
        require(len(formal) == 18, "formal sample count")
        times = [r["timings"] for r in formal] if mode == "mikros" else formal
        fields = ("prover_ms",) + PHASES if mode == "mikros" else ("prove_with_commit_ms",)
        stats = {key: statistics.median(r[key] for r in times) for key in fields}
        stats.update(samples=18, warmup_samples=3, metadata=metadata,
            mean_prover_ms=statistics.mean(r[fields[0]] for r in times),
            prover_range_ms=[min(r[fields[0]] for r in times), max(r[fields[0]] for r in times)],
            verify_ms=statistics.median(r["verify_ms"] for r in formal),
            **{key: formal[0][key] for key in ("proof_bytes", "proof_blake3", "commitment_blake3")})
        result["modes"][mode] = stats
        meta, validation, negatives = checked_records(logs / f"validation-{mode}.jsonl", mode, True)
        require({key: meta[key] for key in SHARED_DIGESTS} == shared, "validation statement differs")
        require(all(validation[0][key] == formal[0][key] for key in ("proof_blake3", "proof_bytes", "commitment_blake3")), "validation proof differs")
        result["negative_tests"][mode] = negatives[0]
    require(result["modes"]["flock6"]["commitment_blake3"] == result["modes"]["flock7"]["commitment_blake3"], "skip changed committed witness")
    result["shared_statement_digests"] = shared
    m = result["modes"]["mikros"]["prover_ms"]
    result.update(flock6_over_mikros=result["modes"]["flock6"]["prove_with_commit_ms"] / m,
                  flock7_over_mikros=result["modes"]["flock7"]["prove_with_commit_ms"] / m)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target", required=True, type=Path)
    parser.add_argument("--logs", type=Path, default=Path("results-skip7"))
    parser.add_argument("--skip-build", action="store_true")
    args = parser.parse_args()
    root, target = Path.cwd().resolve(), args.target.resolve()
    logs = (root / args.logs).resolve()
    require(logs.is_relative_to(root), "logs must be in snapshot")
    manifest = root / "source_manifest.json"
    sources = json.loads(manifest.read_text(encoding="utf-8"))["files_sha256"]
    require(len(sources) >= 400 and Path(__file__).resolve().relative_to(root).as_posix() in sources, "runner absent from source receipt")
    def verify_sources():
        for name, expected in sources.items():
            path = (root / name).resolve()
            require(path.is_relative_to(root) and path.is_file() and digest(path) == expected, f"source mismatch: {name}")
    verify_sources()
    logs.mkdir(exist_ok=False)
    env = {k: v for k, v in os.environ.items() if not k.startswith(("FLOCK_", "MIKROS_", "E2E_", "LIG_", "RAYON_"))
           and k not in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_RUSTFLAGS", "RUSTC_WRAPPER",
                         "RUSTC_WORKSPACE_WRAPPER", "PCS_TRACE", "LD_PRELOAD")}
    env.update(COMMON_ENV, CARGO_TARGET_DIR=str(target))
    env["PATH"] = str(Path.home() / ".cargo/bin") + ":" + env.get("PATH", "")
    relevant_env = {k: v for k, v in env.items() if k.startswith(("FLOCK_", "MIKROS_", "RAYON_")) or k in ("RUSTFLAGS", "CARGO_TARGET_DIR")}
    status = {"state": "starting", "completed": []}
    def run(name, invocation, build=False):
        status["state"] = name
        save(logs / "status.json", status)
        if not build:
            save(logs / f"{name}.preflight.json", preflight())
            invocation = ["/usr/bin/time", "-v", "-o", str(logs / f"{name}.resource.txt"), "taskset", "-c", CPU_SPEC] + invocation
        save(logs / f"{name}.command.json", invocation)
        save(logs / f"{name}.environment.json", relevant_env)
        started = time.monotonic()
        with (logs / f"{name}.jsonl").open("w") as out, (logs / f"{name}.stderr.log").open("w") as err:
            process = subprocess.Popen(invocation, cwd=root, env=env, stdout=out, stderr=err, start_new_session=True)
            try:
                process.wait(timeout=900 if build else 300)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
        event = {"name": name, "exit_code": process.returncode, "wall_seconds": time.monotonic() - started}
        status["completed"].append(event)
        save(logs / "status.json", status)
        print(json.dumps(event), flush=True)
        require(process.returncode == 0, f"command failed: {name}")
    try:
        save(logs / "hardware.json", {"cpuinfo": Path("/proc/cpuinfo").read_text(), "meminfo": Path("/proc/meminfo").read_text(), "affinity": CPU_SPEC,
             "utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), "environment": relevant_env})
        save(logs / "preflight.json", preflight())
        order = [list(MODES[batch:] + MODES[:batch]) for batch in range(3)]
        save(logs / "run_plan.json", {"modes": MODES, "order": order, "reps": 6, "warmups": 1, "batches": 3, "profiling_enabled": False,
             "source_files": len(sources), "source_manifest_sha256": digest(manifest), "mikros_uses_compiled_defaults": True})
        built = {mode: str(target / "release/examples" / name) for mode, name in
                 (("mikros", "e2e_sha256_paired"), ("flock", "e2e_flock_baseline"))}
        receipt_path = root / "skip7-build.json"
        if args.skip_build:
            receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
            require(receipt["source_manifest_sha256"] == digest(manifest) and receipt["toolchain"] == TOOLCHAIN
                    and receipt["rustflags"] == COMMON_ENV["RUSTFLAGS"], "build receipt mismatch")
            require(receipt["binary_sha256"] == {p: digest(Path(p)) for p in built.values()}, "build bytes mismatch")
        else:
            for name in sources:
                os.utime(root / name, None)
            run("build", ["cargo", TOOLCHAIN, "build", "--release", "--offline", "--locked", "-j8", "-p", "mikros",
                          "--example", "e2e_sha256_paired", "--example", "e2e_flock_baseline"], True)
            receipt = {"source_manifest_sha256": digest(manifest), "toolchain": TOOLCHAIN, "rustflags": COMMON_ENV["RUSTFLAGS"],
                       "binary_sha256": {p: digest(Path(p)) for p in built.values()}}
            save(receipt_path, receipt)
        save(logs / "build-receipt.json", receipt)
        binary_dir = logs / "evidence-binaries"
        binary_dir.mkdir()
        binaries = {}
        for mode, name in built.items():
            destination = binary_dir / Path(name).name
            shutil.copy2(name, destination)
            require(digest(destination) == receipt["binary_sha256"][name], "copied executable differs")
            binaries[mode] = str(destination)
        save(logs / "binary-sha256.json", {str(Path(p).relative_to(logs)): digest(Path(p)) for p in binaries.values()})
        for mode in MODES:
            run(f"validation-{mode}", command(binaries, mode, True))
            checked_records(logs / f"validation-{mode}.jsonl", mode, True)
        for batch, sequence in enumerate(order):
            for mode in sequence:
                run(f"{mode}-{batch}", command(binaries, mode))
                checked_records(logs / f"{mode}-{batch}.jsonl", mode)
        save(logs / "summary.json", audit(logs))
        verify_sources()
        require(receipt["binary_sha256"] == {p: digest(Path(p)) for p in built.values()}, "built binaries changed during run")
        require(all(digest(Path(p)) == receipt["binary_sha256"][built[mode]] for mode, p in binaries.items()), "executed binary changed")
        status["state"] = "complete"
    except Exception as error:
        status.update(state="failed", error=str(error))
        raise
    finally:
        save(logs / "status.json", status)
        save(logs / "evidence_sha256.json", {p.relative_to(logs).as_posix(): digest(p) for p in sorted(logs.rglob("*"))
             if p.is_file() and p.name != "evidence_sha256.json"})


if __name__ == "__main__":
    main()
