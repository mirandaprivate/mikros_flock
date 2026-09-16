#!/usr/bin/env python3
"""Frozen native-64 binary Hadamard PIOP comparison at N=2**30.

Run from an extracted source root containing source_manifest.json. The existing
univariate_vs_flock bench is the sole prover entry point: pairreuse versus
flock128 (skip 7). This measures a PIOP reduction, not an E2E SNARK or PCS.
Three independent paired processes each contain one warmup and six formal
repetitions. A separate three-repetition profile and six fresh-process cold RSS
runs are diagnostic and never enter the formal timing summary.
"""
from __future__ import annotations

import argparse
import json
import math
import os
from pathlib import Path
import re
import shutil
import signal
import statistics
import subprocess
import sys
import time

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "20260916-sha128"))
from run_univariate_64 import check_report
from run_column_projection import CPU_SPEC, FEATURES, TOOLCHAIN, digest, preflight, require, save

BENCH = "univariate_vs_flock"
SYSTEMS = ["pairreuse", "flock128"]
THREADS = 64
LOG_N = 30
RUSTFLAGS = "-C target-cpu=native"
SCOPE = "Binary Hadamard packed-opening PIOP only; no PCS, commitment, extra column-to-tensor aggregation or E2E SNARK"
TIMING_SCOPE = ("Prover includes witness-dependent transforms, GF256 work, packing and IP tail; "
                "excludes input generation/digest, challenger creation, serialization and independent source-claim oracle. "
                "Verifier is reduction replay only; input-claim oracle time is separately reported.")
MEMORY_SCOPE = ("Fresh-process VmRSS immediately before prove and VmHWM immediately after prove, before verify/source oracle; "
                "both include packed input/runtime/setup and the HWM can include earlier preparation. "
                "GNU time peak is the complete process including input generation, prover, verifier, full source oracle and serialization. "
                "HWM minus initial RSS is an observed increase, not a precise allocation measurement.")
FINGERPRINTS = ("rustc", "cpu", "os", "arch", "compiled_target_features", "native_backend", "carryless_backend",
                "mikros_sources_blake3", "flock_sources_blake3", "build_configuration_blake3")
REQUIRED_TESTS = {
    "test-univariate": {
        "univariate_kernels_have_identical_bytes_claims_and_transcripts",
        "univariate_binds_every_message_class_and_rejects_false_c",
        "univariate_product_preserving_forgery_needs_source_authentication",
        "univariate_source_checks_all_inputs_and_preserves_weighted_b_factors",
        "univariate_reused_scratch_does_not_retain_witness_or_shape_state",
    },
    "test-zerocheck": {
        "zerocheck::tests::skip7_honest_roundtrip_matches_source_claims",
        "zerocheck::tests::skip7_rejects_corrupt_messages_and_false_witness",
        "zerocheck::tests::skip7_rejects_shapes_dimensions_and_wrong_domain",
        "zerocheck::tests::skip7_domain_is_distinct_and_skip6_domain_is_unchanged",
        "zerocheck::tests::skip7_grinding_roundtrip_and_tape",
    },
}


def utc():
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())


def stat(values):
    require(bool(values) and all(type(x) in (int, float) and math.isfinite(x) for x in values), "invalid summary values")
    return {"median": statistics.median(values), "mean": statistics.mean(values), "min": min(values), "max": max(values)}


def peak_rss(path):
    text = path.read_text(encoding="utf-8")
    peaks = re.findall(r"^\s*Maximum resident set size \(kbytes\):\s*(\d+)\s*$", text, re.MULTILINE)
    exits = re.findall(r"^\s*Exit status:\s*(\d+)\s*$", text, re.MULTILINE)
    require(len(peaks) == 1 and int(peaks[0]) > 0 and exits == ["0"], f"invalid GNU time report: {path.name}")
    return int(peaks[0])


def command(binary, output, systems, log_n, runs, *, rss=False, profile=False):
    argv = [str(binary), "--logs", str(log_n), "--runs", str(runs), "--threads", str(THREADS),
            "--systems", ",".join(systems), "--output", str(output)]
    if rss:
        argv.append("--rss")
    if profile:
        argv.append("--profile")
    return argv


def batch_systems(batch):
    require(batch in range(3), "invalid batch")
    return SYSTEMS if batch % 2 == 0 else SYSTEMS[::-1]


def verified_report(path, systems, log_n, runs, *, rss=False, profile=False):
    report = check_report(path, systems, [log_n], runs, rss=rss, profile=profile)
    meta = report["metadata"]
    require(meta["source_check_reuse"] is False, f"source checks reused: {path.name}")
    require(FEATURES.issubset(meta["compiled_target_features"]), f"native features absent: {path.name}")
    for row in report["samples"] + report["warmups"]:
        require(row["source_check_method"] == "full_oracle" and row["source_check_anchor"] == row["system"],
                f"missing independent full source oracle: {path.name}")
    return report


def test_results(logs):
    result = {}
    for name, required in REQUIRED_TESTS.items():
        text = (logs / f"{name}.stdout.log").read_text(encoding="utf-8")
        summaries = re.findall(r"test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out", text)
        require(len(summaries) == 1 and summaries[0][0] == "ok" and summaries[0][2:5] == ("0", "0", "0"),
                f"invalid test result: {name}")
        passed = int(summaries[0][1])
        require(passed == 10 if name == "test-univariate" else passed >= 6, f"required tests did not run: {name}")
        for test in required:
            require(re.search(r"^test " + re.escape(test) + r" \.\.\. ok$", text, re.MULTILINE) is not None,
                    f"required test missing: {test}")
        result[name] = passed
    return result


def profile_flock(logs):
    text = (logs / "profile.stderr.log").read_text(encoding="utf-8")
    labels = {"round1_urm_ms": "round1 URM", "round2_fold_ms": "round2 fused fold", "tail_ms": "rounds 3+ tail"}
    result = {}
    for key, label in labels.items():
        values = [float(x) for x in re.findall(r"\[zc-timing\] " + re.escape(label) + r": ([0-9.]+) ms", text)]
        require(len(values) == 4, f"FLOCK profile lacks warmup plus three {label} timings")
        result[key] = {"warmup": values[0], "samples": values[1:], **stat(values[1:])}
    return result


def audit(logs):
    reports = {"smoke": verified_report(logs / "smoke.json", SYSTEMS, 16, 2)}
    for batch in range(3):
        reports[f"formal-{batch}"] = verified_report(logs / f"formal-{batch}.json", batch_systems(batch), LOG_N, 6)
    reports["profile"] = verified_report(logs / "profile.json", SYSTEMS, LOG_N, 3, profile=True)
    for repetition in range(3):
        for system in SYSTEMS:
            name = f"rss-{system}-{repetition}"
            reports[name] = verified_report(logs / f"{name}.json", [system], LOG_N, 1, rss=True)
    reference = reports["smoke"]["metadata"]
    inputs, proofs = {}, {}
    for name, report in reports.items():
        require(all(report["metadata"][key] == reference[key] for key in FINGERPRINTS), f"build/source drift: {name}")
        for row in report["samples"] + report["warmups"]:
            key = row["log_n"], row["run"]
            require(inputs.setdefault(key, row["input_digest"]) == row["input_digest"], f"different paired inputs: {name}")
            proof_key = (*key, row["system"])
            require(proofs.setdefault(proof_key, row["proof_digest"]) == row["proof_digest"], f"proof changed with profile/RSS/repetition: {name}")
    result = {
        "schema_version": 1, "scope": SCOPE, "log_n": LOG_N, "binary_constraints": 1 << LOG_N,
        "threads": THREADS, "cpu_spec": CPU_SPEC, "mikros_kappa": 128, "flock_skip": 7,
        "intermediate_field_bits": 8, "grinding": "disabled by both existing PIOP APIs; no equal-security or E2E SNARK claim",
        "timing_scope": TIMING_SCOPE, "memory_scope": MEMORY_SCOPE,
        "formal_process_batches": 3, "warmups_per_batch_per_system": 1, "formal_samples_per_system": 18,
        "formal_repetitions_per_batch": 6, "fresh_process_rss_repetitions_per_system": 3,
        "unique_formal_witnesses": 6, "repetitions_per_formal_witness_per_system": 3,
        "input_seed_policy": "Each run index 0..6 uses its deterministic seed, identical across systems and repeated in each batch. Fresh-process RSS uses run 0.",
        "verification_policy": "Every proof passed reduction replay, claims/transcript equality and full independent input-claim oracle outside timing.",
        "proof_size_scope": "PIOP field payload and bincode serialization only; no PCS/openings or commitment",
        "build_fingerprints": {key: reference[key] for key in FINGERPRINTS},
        "tests_passed": test_results(logs), "systems": {}, "formal_raw_reports": [],
        "smoke_report": reports["smoke"], "profile_report": reports["profile"],
        "profile_flock_stderr_phases": profile_flock(logs),
        "profile_scope": "Independent instrumented process, one warmup and three samples per system; not included in formal times. FLOCK stderr phases rounded to 0.01 ms.",
    }
    for batch in range(3):
        name = f"formal-{batch}"
        result["formal_raw_reports"].append({"name": name, "report": reports[name],
                                             "whole_process_peak_rss_kib": peak_rss(logs / f"{name}.resource.txt")})
    for system in SYSTEMS:
        formal = [row for batch in range(3) for row in reports[f"formal-{batch}"]["samples"] if row["system"] == system]
        warmups = [row for batch in range(3) for row in reports[f"formal-{batch}"]["warmups"] if row["system"] == system]
        require(len(formal) == 18 and len(warmups) == 3, "formal/warmup count")
        for key in ("payload_field_elements", "payload_field_bytes", "bincode_bytes"):
            require(len({row[key] for row in formal + warmups}) == 1, f"proof size drift: {system}")
        rss_rows = []
        for repetition in range(3):
            name = f"rss-{system}-{repetition}"
            row = reports[name]["samples"][0]
            peak = peak_rss(logs / f"{name}.resource.txt")
            # GNU time may round differently from /proc; never equate these scopes.
            rss_rows.append({"report_name": name, "sample": row, "whole_process_peak_rss_kib": peak,
                             "observed_prove_hwm_minus_before_rss_kib": row["vm_hwm_after_prove_kib"] - row["vm_rss_before_prove_kib"]})
        entry = {key: stat([row[key] for row in formal]) for key in ("prover_ms", "verifier_ms", "input_claim_check_ms")}
        entry.update({key: formal[0][key] for key in ("payload_field_elements", "payload_field_bytes", "bincode_bytes")})
        entry.update(formal_samples=formal, warmups=warmups, cold_rss_reports=rss_rows,
                     cold_prover_ms=stat([row["sample"]["prover_ms"] for row in rss_rows]),
                     vm_rss_before_prove_kib=stat([row["sample"]["vm_rss_before_prove_kib"] for row in rss_rows]),
                     vm_hwm_after_prove_kib=stat([row["sample"]["vm_hwm_after_prove_kib"] for row in rss_rows]),
                     whole_process_peak_rss_kib=stat([row["whole_process_peak_rss_kib"] for row in rss_rows]))
        if system == "pairreuse":
            phases = [row["mikros_phases"] for row in reports["profile"]["samples"] if row["system"] == system]
            require(len(phases) == 3 and all(set(row) == set(phases[0]) for row in phases), "Mikros profile phase shape")
            entry["profile_phase_ms"] = {key: stat([row[key] for row in phases]) for key in phases[0]}
        result["systems"][system] = entry
    result["flock_over_mikros_prover"] = result["systems"]["flock128"]["prover_ms"]["median"] / result["systems"]["pairreuse"]["prover_ms"]["median"]
    result["validated_proof_records"] = sum(len(report["samples"]) + len(report["warmups"]) for report in reports.values())
    result["total_tests_passed"] = sum(result["tests_passed"].values())
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target", required=True, type=Path, help="shared Cargo target directory; executed binary is frozen in evidence")
    parser.add_argument("--logs", type=Path, default=Path("results-hadamard"), help="fresh results directory inside source snapshot")
    args = parser.parse_args()
    root, target = Path.cwd().resolve(), args.target.resolve()
    logs = (root / args.logs).resolve()
    require(logs.is_relative_to(root) and logs != root, "logs must be a child of snapshot")
    manifest_path = root / "source_manifest.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    sources = manifest["files_sha256"]
    required = {"Cargo.toml", "Cargo.lock", ".cargo/config.toml", "mikros/build.rs", "mikros/Cargo.toml",
                "mikros/benches/univariate_vs_flock.rs", "mikros/benches/support/source_checks.rs",
                "mikros/server_experiments/run_univariate_64.py", "mikros/server_experiments/run_server_64.py",
                "mikros/server_experiments/20260916-sha128/run_column_projection.py",
                "mikros/server_experiments/20260916-sha128/run_public_csc.py",
                "mikros/server_experiments/20260916-sha128/run_server.py", Path(__file__).resolve().relative_to(root).as_posix()}
    for folder in ("crates", "mikros/src", "mikros/benches", "mikros/tests"):
        required.update(path.relative_to(root).as_posix() for path in (root / folder).rglob("*.rs"))
    require(len(sources) >= 400 and required.issubset(sources), "source receipt incomplete or runner/helpers absent")

    def verify_sources():
        for name, expected in sources.items():
            path = (root / name).resolve()
            require(path.is_relative_to(root) and path.is_file() and re.fullmatch(r"[0-9a-f]{64}", expected)
                    and digest(path) == expected, f"source mismatch: {name}")
        require(json.loads(manifest_path.read_text(encoding="utf-8")) == manifest, "source manifest changed")

    verify_sources()
    logs.mkdir(exist_ok=False)
    removed = {k for k in os.environ if k.startswith(("FLOCK_", "MIKROS_", "E2E_", "LIG_", "RAYON_"))
               or k in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_RUSTFLAGS", "CARGO_BUILD_TARGET",
                        "RUSTC", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER", "PCS_TRACE", "LD_PRELOAD")
               or (k.startswith("CARGO_TARGET_") and k.endswith("_RUSTFLAGS"))}
    env = {k: v for k, v in os.environ.items() if k not in removed}
    env.update(RUSTFLAGS=RUSTFLAGS, RAYON_NUM_THREADS=str(THREADS), CARGO_TARGET_DIR=str(target), LC_ALL="C")
    env["PATH"] = str(Path.home() / ".cargo/bin") + ":" + env.get("PATH", "")
    relevant_env = {key: env[key] for key in ("RUSTFLAGS", "RAYON_NUM_THREADS", "CARGO_TARGET_DIR", "LC_ALL")}
    status = {"state": "starting", "utc_start": utc(), "completed": []}

    def run(name, invocation, *, profile=False, timeout=1800):
        status["state"] = name
        save(logs / "status.json", status)
        save(logs / f"{name}.preflight.json", preflight())
        invocation = ["/usr/bin/time", "-v", "-o", str(logs / f"{name}.resource.txt"), "taskset", "-c", CPU_SPEC] + invocation
        child_env = env.copy()
        if profile:
            child_env["FLOCK_ZC_TIMING"] = "1"
        save(logs / f"{name}.command.json", invocation)
        save(logs / f"{name}.environment.json", {**relevant_env, **({"FLOCK_ZC_TIMING": "1"} if profile else {})})
        event = {"name": name, "utc_start": utc()}
        started = time.monotonic()
        with (logs / f"{name}.stdout.log").open("w", encoding="utf-8") as out, (logs / f"{name}.stderr.log").open("w", encoding="utf-8") as err:
            process = subprocess.Popen(invocation, cwd=root, env=child_env, stdout=out, stderr=err, start_new_session=True)
            try:
                process.wait(timeout=timeout)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
                event["timeout"] = True
        event.update(exit_code=process.returncode, wall_seconds=time.monotonic() - started, utc_end=utc())
        if process.returncode == 0:
            event["whole_process_peak_rss_kib"] = peak_rss(logs / f"{name}.resource.txt")
        status["completed"].append(event)
        save(logs / "status.json", status)
        print(json.dumps(event), flush=True)
        require(process.returncode == 0, f"command failed: {name}; see {name}.stderr.log / {name}.stdout.log")

    try:
        save(logs / "hardware.json", {"cpuinfo": Path("/proc/cpuinfo").read_text(), "meminfo": Path("/proc/meminfo").read_text(),
             "hostname": os.uname().nodename, "uname": list(os.uname()), "affinity": CPU_SPEC,
             "allowed_cpus": sorted(os.sched_getaffinity(0)), "utc": utc(), "environment": relevant_env,
             "removed_environment_keys": sorted(removed), "numa_policy": "Linux default first touch; no explicit memory bind/interleave",
             "thread_policy": "Same fixed 64 logical CPUs; 48 physical cores plus 16 SMT siblings; sequential processes on shared host"})
        save(logs / "run_plan.json", {"scope": SCOPE, "systems": SYSTEMS, "log_n": LOG_N,
             "batch_systems": [batch_systems(batch) for batch in range(3)], "batches": 3, "runs": 6, "warmups": 1,
             "unique_formal_witnesses": 6, "repetitions_per_formal_witness_per_system": 3,
             "profile_runs": 3, "rss_cold_repetitions": 3, "timing_scope": TIMING_SCOPE, "memory_scope": MEMORY_SCOPE,
             "source_files": len(sources), "source_manifest_sha256": digest(manifest_path),
             "grinding": "disabled by existing PIOP APIs, not an environment override"})
        for name in sources:
            os.utime(root / name, None)
        cargo = ["cargo", TOOLCHAIN]
        flags = ["--release", "--offline", "--locked", "-j8"]
        run("rustc-version", ["rustup", "run", TOOLCHAIN.lstrip("+"), "rustc", "-Vv"])
        run("rustc-cfg", ["rustup", "run", TOOLCHAIN.lstrip("+"), "rustc", "-C", "target-cpu=native", "--print", "cfg"])
        features = re.findall(r'^target_feature="([^"]+)"$', (logs / "rustc-cfg.stdout.log").read_text(), re.MULTILINE)
        require(FEATURES.issubset(features), "server/toolchain missing native features")
        run("test-univariate", cargo + ["test"] + flags + ["-p", "mikros", "--test", "univariate_roundtrip", "--", "--test-threads=1"])
        run("test-zerocheck", cargo + ["test"] + flags + ["-p", "flock-core", "--lib", "zerocheck::tests", "--", "--test-threads=1"])
        tests = test_results(logs)
        run("build", cargo + ["bench", "--offline", "--locked", "-j8", "-p", "mikros", "--bench", BENCH,
                               "--no-run", "--message-format=json"])
        messages = [json.loads(line) for line in (logs / "build.stdout.log").read_text(encoding="utf-8").splitlines() if line.startswith("{")]
        artifacts = [message for message in messages if message.get("reason") == "compiler-artifact"
                     and message.get("target", {}).get("name") == BENCH and "bench" in message["target"]["kind"] and message.get("executable")]
        require(len(artifacts) == 1, "expected exactly one compiler-reported benchmark executable")
        built = Path(artifacts[0]["executable"]).resolve()
        require(built.is_relative_to(target) and built.is_file() and built.read_bytes()[:4] == b"\x7fELF", "invalid bench ELF artifact")
        require(any(message.get("reason") == "build-finished" and message.get("success") is True for message in messages), "missing successful compiler receipt")
        binary_dir = logs / "evidence-binaries"
        binary_dir.mkdir()
        binary = binary_dir / BENCH
        shutil.copy2(built, binary)
        require(digest(binary) == digest(built), "frozen executable differs")
        receipt = {"source_manifest_sha256": digest(manifest_path), "source_files": len(sources), "toolchain": TOOLCHAIN,
                   "rustflags": RUSTFLAGS, "bench": BENCH, "cargo_artifact": artifacts[0], "built_path": str(built),
                   "binary_relative_path": binary.relative_to(logs).as_posix(), "binary_sha256": digest(binary),
                   "build_stdout_sha256": digest(logs / "build.stdout.log"), "native_target_features": features,
                   "rustc_version": (logs / "rustc-version.stdout.log").read_text().strip(), "tests_passed": tests}
        save(logs / "build-receipt.json", receipt)

        def benchmark(name, systems, log_n, runs, *, rss=False, profile=False):
            require(digest(binary) == receipt["binary_sha256"], "frozen binary changed")
            output = logs / f"{name}.json"
            run(name, command(binary, output, systems, log_n, runs, rss=rss, profile=profile), profile=profile)
            verified_report(output, systems, log_n, runs, rss=rss, profile=profile)

        benchmark("smoke", SYSTEMS, 16, 2)
        for batch in range(3):
            benchmark(f"formal-{batch}", batch_systems(batch), LOG_N, 6)
        benchmark("profile", SYSTEMS, LOG_N, 3, profile=True)
        for repetition in range(3):
            for system in batch_systems(repetition):
                benchmark(f"rss-{system}-{repetition}", [system], LOG_N, 1, rss=True)
        save(logs / "summary.json", audit(logs))
        verify_sources()
        require(digest(binary) == receipt["binary_sha256"], "executed binary changed during run")
        status.update(state="complete", utc_end=utc())
    except Exception as error:
        status.update(state="failed", error=str(error), utc_end=utc())
        raise
    finally:
        save(logs / "status.json", status)
        save(logs / "evidence_sha256.json", {path.relative_to(logs).as_posix(): digest(path) for path in sorted(logs.rglob("*"))
             if path.is_file() and path != logs / "evidence_sha256.json"})


if __name__ == "__main__":
    main()
