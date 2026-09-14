#!/usr/bin/env python3
"""Matched legacy/packed permutation and same-PCS E2E on the authorized server.

Run from an extracted package_e2e.py source snapshot. Standalone --log-n counts
permutation cells; E2E --log-leaves counts SHA tree leaves. Their bit counts are
reported separately and must not be equated.
"""
import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import re
import statistics
import subprocess
import time


CPU_SPEC = "0-47,48-55,72-79"
CPUS = set(range(56)) | set(range(72, 80))
TOOLCHAIN = "+nightly-2026-03-21"
NATIVE_FEATURES = {"avx512f", "avx512bw", "gfni", "vpclmulqdq"}
SYSTEMS = ("flock", "mikros", "mikros_packed")
assert len(CPUS) == 64


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_records(path):
    records = [json.loads(line) for line in path.read_text().splitlines() if line.strip()]
    metadata = [row for row in records if row.get("record") == "metadata"]
    require(len(metadata) == 1, f"{path.name}: expected exactly one metadata record")
    return metadata[0], records


def native_metadata(metadata):
    require(metadata["threads"] == metadata["all_core_threads"] == 64,
            "both Rayon pools must use exactly 64 threads")
    require(NATIVE_FEATURES.issubset(metadata["target_features"]),
            "benchmark binary lacks required GFNI/AVX-512/VPCLMUL features")


def statistics_row(group, timing_keys, nested=False):
    require(bool(group), "empty sample group")
    values = [row["timings"] if nested else row for row in group]
    for row in values:
        for key in timing_keys:
            require(isinstance(row[key], (int, float)) and math.isfinite(row[key]) and row[key] >= 0,
                    f"invalid timing {key}: {row[key]}")
    require(len({row["proof_blake3"] for row in group}) == 1, "proof changed between repetitions")
    require(len({row["proof_bytes"] for row in group}) == 1, "proof size changed between repetitions")
    result = {key: statistics.median(row[key] for row in values) for key in timing_keys}
    result.update(proof_bytes=group[0]["proof_bytes"],
                  proof_blake3=group[0]["proof_blake3"],
                  verify_ms=statistics.median(row["verify_ms"] for row in group),
                  prover_range_ms=[min(row["prover_ms"] for row in values),
                                   max(row["prover_ms"] for row in values)],
                  repetitions=len(group))
    return result


def validate_standalone(path, log_n, reps, warmups, balanced):
    metadata, records = read_records(path)
    native_metadata(metadata)
    require(metadata["schema_version"] == 1, "unsupported standalone schema")
    require(metadata["log_n"] == log_n and metadata["permutation_cells"] == 1 << log_n,
            "wrong standalone permutation size")
    require(metadata["tuple_width"] == log_n + 1, "bit-address tuple width mismatch")
    require(metadata["reps"] == reps and metadata["warmups"] == warmups, "wrong run counts")
    field_bits = 1 << (metadata["tuple_width"] - 1).bit_length()
    require(metadata["packed_kappa"] == 128 // field_bits, "incorrect tuple packing factor")
    all_samples = [row for row in records if row.get("record") == "sample"]
    require(len(all_samples) == 2 * (reps + warmups), "missing standalone samples")
    require(all(row["verified"] and row["transcript_match"] and row["original_source_checked"]
                for row in all_samples), "standalone proof/transcript/original-source check failed")
    samples = [row for row in all_samples if not row["warmup"]]
    require(len(samples) == 2 * reps, "wrong measured standalone count")
    result = {"metadata": metadata, "all_original_source_checks_passed": True}
    for variant in ("legacy", "packed"):
        group = [row for row in samples if row["variant"] == variant]
        require(len(group) == reps, f"wrong {variant} count")
        require(sorted(row["iteration"] for row in group) == list(range(1, reps + 1)),
                f"wrong {variant} iterations")
        if balanced:
            require(all(sum(row["slot"] == slot for row in group) == reps // 2 for slot in (0, 1)),
                    f"{variant} order is not balanced")
        result[variant] = statistics_row(group, ("prover_ms", "source_check_ms"))
        result[variant]["logical_field_elements"] = group[0]["logical_field_elements"]
        phase_keys = set(group[0]["timings"])
        require(all(set(row["timings"]) == phase_keys for row in group), "phase schema changed")
        result[variant]["phases_ms"] = {
            key: statistics.median(row["timings"][key] for row in group) for key in sorted(phase_keys)
        }
    require(result["legacy"]["prover_ms"] > 0 and result["packed"]["prover_ms"] > 0,
            "zero standalone prover time")
    result["packed_over_legacy"] = result["packed"]["prover_ms"] / result["legacy"]["prover_ms"]
    result["packed_speedup"] = 1 / result["packed_over_legacy"]
    return result


def validate_e2e(path, leaves, reps, warmups, balanced, tamper=False):
    metadata, records = read_records(path)
    native_metadata(metadata)
    require(metadata["schema_version"] == 4, "unsupported E2E schema")
    require(metadata["log_leaves"] == leaves, "wrong E2E tree size")
    require(metadata["permutation"] == "both" and metadata["reduction"] == "lockstep",
            "E2E must compare both permutation protocols with the lockstep reduction")
    require(metadata["pcs_path"] == "flock-production-dense-merged", "wrong PCS path")
    require(metadata["compression_kappa"] == 128 and metadata["flock_skip"] == 7,
            "Boolean proof compression mismatch")
    require(metadata["gather_audit_bits"] == 2 and metadata["gather_audit_degree"] == 3,
            "incorrect gather audit policy")
    require(metadata["wiring_bits"] == 1 << metadata["wiring_bit_log"], "wrong wiring bit count")
    require(metadata["logical_bit_log"] == max(leaves + 16, 22) and metadata["wiring_bit_log"] == max(leaves + 12, 18),
            "E2E fixture dimensions changed; inspect the new scale before running")
    all_samples = [row for row in records if row.get("record") == "sample"]
    require(len(all_samples) == 3 * (reps + warmups), "missing E2E samples")
    require({row["system"] for row in all_samples} == set(SYSTEMS), "wrong E2E systems")
    require(all(row["verified"] and row["transcript_match"] and row["pcs_config_match"]
                for row in all_samples), "E2E verification/transcript/PCS check failed")
    require(len({row["commitment_blake3"] for row in all_samples}) == 1,
            "systems did not commit the same witness with the same PCS")
    claims = {"flock": 4, "mikros": 5, "mikros_packed": 5}
    variants = {"flock": "not_applicable", "mikros": "legacy", "mikros_packed": "packed"}
    for row in all_samples:
        require(row["permutation_variant"] == variants[row["system"]], "wrong permutation variant")
        require(row["timings"]["tensor_claims"] == row["timings"]["pcs_ring_claims"] == claims[row["system"]],
                "unexpected number of authenticated PCS claims")
        require(row["timings"]["pcs_inner_openings"] == 1, "PCS must use one shared inner opening")
    samples = [row for row in all_samples if not row["warmup"]]
    require(len(samples) == 3 * reps, "wrong measured E2E count")
    result = {"metadata": metadata, "same_commitment": True, "same_production_pcs_config": True,
              "all_terminal_claims_authenticated": True,
              "commitment_blake3":all_samples[0]["commitment_blake3"]}
    keys = ("prover_ms", "commit_ms", "boolean_ms", "wiring_ms", "parallel_piops_ms", "pcs_open_ms")
    for system in SYSTEMS:
        group = [row for row in samples if row["system"] == system]
        require(len(group) == reps, f"wrong {system} count")
        require(sorted(row["iteration"] for row in group) == list(range(warmups, warmups + reps)),
                f"wrong {system} iterations")
        if balanced:
            require(all(sum(row["system"] == system for row in samples[slot::3]) == reps // 3
                        for slot in range(3)), f"{system} order is not balanced")
        result[system] = statistics_row(group, keys, nested=True)
        result[system].update(tensor_claims=claims[system], pcs_ring_claims=claims[system], pcs_inner_openings=1)
        grinding_keys = ("wiring_grinding_ms", "initial_fingerprint_grinding_ms")
        if system == "flock":
            require(all(row["timings"][key] is None for row in group for key in grinding_keys),
                    "Flock grinding attribution is unavailable and must remain null")
            result[system].update(wiring_grinding_ms=None, initial_fingerprint_grinding_ms=None,
                                  wiring_compute_ms=None)
        else:
            for row in group:
                timings = row["timings"]
                for key in grinding_keys:
                    require(isinstance(timings[key], (int, float)) and math.isfinite(timings[key]) and timings[key] >= 0,
                            f"missing or invalid {system} {key}")
                require(timings["initial_fingerprint_grinding_ms"] <= timings["wiring_grinding_ms"] <= timings["wiring_ms"],
                        f"{system} grinding attribution exceeds its enclosing timer")
            result[system].update({key:statistics.median(row["timings"][key] for row in group)
                                   for key in grinding_keys})
            result[system]["wiring_compute_ms"] = statistics.median(
                row["timings"]["wiring_ms"] - row["timings"]["wiring_grinding_ms"] for row in group)
    for numerator, denominator in (("mikros_packed", "mikros"), ("mikros_packed", "flock"), ("mikros", "flock")):
        require(result[denominator]["prover_ms"] > 0, "zero E2E prover time")
        result[numerator + "_over_" + denominator] = result[numerator]["prover_ms"] / result[denominator]["prover_ms"]
    result["packed_wiring_over_legacy"] = result["mikros_packed"]["wiring_ms"] / result["mikros"]["wiring_ms"]
    require(result["mikros"]["wiring_compute_ms"] > 0, "zero legacy wiring compute time")
    result["packed_wiring_compute_over_legacy"] = result["mikros_packed"]["wiring_compute_ms"] / result["mikros"]["wiring_compute_ms"]
    result["grinding_attribution"] = (
        "Primary prover_ms includes all PoW. Wiring compute is the median of per-sample wiring_ms minus "
        "wiring_grinding_ms. Each system uses a different transcript and a fixed nonce workload across "
        "repetitions, so differences in initial fingerprint grinding are not protocol speedups. Flock "
        "grinding attribution is unavailable. Do not subtract wiring grinding from parallel E2E wall time.")
    if tamper:
        negative = [row for row in records if row.get("record") == "negative_tests"]
        require(len(negative) == 3 and {row["system"] for row in negative} == set(SYSTEMS),
                "missing per-system E2E negative tests")
        require(all(row["rejected"] > 0 for row in negative), "no rejected E2E tampering cases")
        result["negative_tests"] = negative
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--logs", default="results-packed-permutation")
    parser.add_argument("--log-n", type=int, default=30)
    parser.add_argument("--log-leaves", type=int, default=14)
    parser.add_argument("--reps", type=int, default=8)
    parser.add_argument("--e2e-reps", type=int, default=6)
    args = parser.parse_args()
    require(8 <= args.log_n <= 30 and 3 <= args.log_leaves <= 14, "unsupported benchmark scale")
    require(args.reps > 0 and args.reps % 2 == 0, "standalone repetitions must be positive and even")
    require(args.e2e_reps > 0 and args.e2e_reps % 3 == 0, "E2E repetitions must be a positive multiple of three")
    root = Path.cwd().resolve()
    logs = (root / args.logs).resolve()
    require(logs.is_relative_to(root) and logs != root, "logs must be a subdirectory of this source snapshot")
    logs.mkdir(exist_ok=False)
    env = os.environ.copy()
    cleared = []
    for key in list(env):
        if key.startswith(("FLOCK_", "MIKROS_")) or key in (
                "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER", "CARGO_BUILD_RUSTFLAGS"):
            cleared.append(key)
            env.pop(key)
    env.update(RUSTFLAGS="-C target-cpu=native", RAYON_NUM_THREADS="64", CARGO_TARGET_DIR=str(root / "target"))
    env["PATH"] = str(Path.home() / ".cargo/bin") + ":" + env.get("PATH", "")
    status = {"state":"starting", "completed":[], "failed":[],
              "started_utc":time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}

    def save(name, value):
        temporary = logs / (name + ".tmp")
        temporary.write_text(json.dumps(value, indent=2) + "\n")
        temporary.replace(logs / name)

    def verify_sources():
        manifest_path = root / "source_manifest.json"
        manifest = json.loads(manifest_path.read_text())
        require(bool(manifest["files_sha256"]), "empty source manifest")
        for name, expected in manifest["files_sha256"].items():
            path = (root / name).resolve()
            require(path.is_relative_to(root) and path.is_file(), f"invalid manifest path: {name}")
            require(sha256(path) == expected, f"source hash mismatch: {name}")
        return manifest

    def resource_snapshot(name, minimum_gib=64):
        require(CPUS.issubset(os.sched_getaffinity(0)), "requested CPUs unavailable")

        def cpu_times():
            result = {}
            for line in Path("/proc/stat").read_text().splitlines():
                fields = line.split()
                if fields[0].startswith("cpu") and fields[0][3:].isdigit() and int(fields[0][3:]) in CPUS:
                    values = list(map(int, fields[1:9]))
                    result[fields[0]] = (sum(values), values[3] + values[4])
            require(len(result) == 64, "missing per-CPU counters")
            return result

        before = cpu_times()
        time.sleep(1)
        after = cpu_times()
        total = sum(after[c][0] - before[c][0] for c in before)
        idle = sum(after[c][1] - before[c][1] for c in before)
        busy = 1 - idle / max(total, 1)
        available = int(next(line.split()[1] for line in Path("/proc/meminfo").read_text().splitlines()
                             if line.startswith("MemAvailable:")))
        save(name + ".resources-before.json", {"selected_cpu_busy_fraction":busy, "mem_available_kib":available,
             "minimum_memory_gib":minimum_gib, "loadavg":os.getloadavg(), "allowed_affinity":sorted(os.sched_getaffinity(0))})
        require(busy < 0.10, f"selected CPUs busy before {name}: {busy:.1%}")
        require(available > minimum_gib * 1024 * 1024, f"less than {minimum_gib} GiB memory available")

    def run(name, command, pinned=False):
        status["state"] = name
        save("status.json", status)
        if pinned:
            command = ["/usr/bin/time", "-v", "-o", str(logs / (name + ".resource.txt")), "taskset", "-c", CPU_SPEC] + command
        save(name + ".command.json", command)
        start = time.monotonic()
        with (logs / (name + ".stdout")).open("w") as stdout, (logs / (name + ".stderr")).open("w") as stderr:
            process = subprocess.run(command, cwd=root, env=env, stdout=stdout, stderr=stderr)
        event = {"name":name, "exit_code":process.returncode, "wall_seconds":time.monotonic() - start}
        status["completed" if process.returncode == 0 else "failed"].append(event)
        save("status.json", status)
        print(json.dumps(event), flush=True)
        require(process.returncode == 0, f"{name} failed; stdout and stderr retained")
        if name.startswith("test-"):
            passed = sum(map(int, re.findall(r"test result: ok\. (\d+) passed;", (logs / (name + ".stdout")).read_text())))
            require(passed > 0, f"{name} matched no passing tests")

    try:
        manifest = verify_sources()
        manifest_hash = sha256(root / "source_manifest.json")
        save("config.json", {**vars(args), "warmups":1, "e2e_warmups":1, "cpu_affinity_spec":CPU_SPEC,
             "toolchain":TOOLCHAIN, "rustflags":env["RUSTFLAGS"], "cleared_environment_keys":sorted(cleared),
             "source_manifest_sha256":manifest_hash, "tests_skipped":False})
        save("hardware.json", {"utc":status["started_utc"], "hostname":os.uname().nodename,
             "selected_cpus":sorted(CPUS), "cpuinfo":Path("/proc/cpuinfo").read_text(),
             "meminfo":Path("/proc/meminfo").read_text(), "cgroup":Path("/proc/self/cgroup").read_text()})
        resource_snapshot("initial", max(64, (128 * (1 << args.log_n)) / (1 << 30)))
        # Archives deliberately use reproducible zero mtimes. A copied Cargo
        # cache must rebuild our current snapshot instead of trusting older
        # mtimes than its dependency records. Content hashes remain unchanged.
        for name in manifest["files_sha256"]:
            (root / name).touch()
        run("rustc", ["rustc", TOOLCHAIN, "-Vv"])
        cargo = ["cargo", TOOLCHAIN]
        build_flags = ["--release", "--offline", "--locked", "-j8"]
        run("build", cargo + ["build"] + build_flags + ["-p", "mikros", "--example", "permutation_packed_compare", "--example", "e2e_bit_compare"])
        for name, targets in (
                ("permutation", ["-p", "mikros-permutation"]),
                ("e2e-helpers", ["-p", "mikros", "--example", "e2e_bit_compare"]),
                ("pcs-functional", ["-p", "flock-core", "pcs::functional::tests", "--", "--include-ignored"]),
                ("pcs-merged-tensor", ["-p", "flock-core", "pcs::merged_tensor::tests", "--", "--include-ignored"]),
                ("pcs-weighted", ["-p", "flock-core", "weighted", "--", "--include-ignored"])):
            run("test-" + name, cargo + ["test"] + build_flags + targets, pinned=True)
        standalone = str(root / "target/release/examples/permutation_packed_compare")
        e2e = str(root / "target/release/examples/e2e_bit_compare")
        binary_hashes = {name:sha256(Path(name)) for name in (standalone, e2e)}
        save("binary-sha256.json", binary_hashes)
        run("validation-permutation", [standalone, "--log-n", "14", "--threads", "64", "--reps", "2", "--warmups", "0"], pinned=True)
        standalone_validation = validate_standalone(logs / "validation-permutation.stdout", 14, 2, 0, balanced=True)
        run("validation-e2e", [e2e, "--log-leaves", "3", "--threads", "64", "--reps", "1", "--warmups", "0",
                              "--permutation", "both", "--reduction", "lockstep", "--tamper"], pinned=True)
        e2e_validation = validate_e2e(logs / "validation-e2e.stdout", 3, 1, 0, balanced=False, tamper=True)
        save("validation-summary.json", {"standalone":standalone_validation, "e2e":e2e_validation})
        resource_snapshot("permutation", max(64, (128 * (1 << args.log_n)) / (1 << 30)))
        run("permutation", [standalone, "--log-n", str(args.log_n), "--threads", "64", "--reps", str(args.reps), "--warmups", "1"], pinned=True)
        standalone_result = validate_standalone(logs / "permutation.stdout", args.log_n, args.reps, 1, balanced=True)
        save("permutation-summary.json", standalone_result)
        resource_snapshot("e2e")
        run("e2e", [e2e, "--log-leaves", str(args.log_leaves), "--threads", "64", "--reps", str(args.e2e_reps), "--warmups", "1",
                    "--permutation", "both", "--reduction", "lockstep"], pinned=True)
        e2e_result = validate_e2e(logs / "e2e.stdout", args.log_leaves, args.e2e_reps, 1, balanced=True)
        save("e2e-summary.json", e2e_result)
        verify_sources()
        require(sha256(root / "source_manifest.json") == manifest_hash, "manifest changed during run")
        for name, expected in binary_hashes.items():
            require(sha256(Path(name)) == expected, f"binary changed during run: {name}")
        save("summary.json", {"source_hashes_verified_before_and_after":True, "binary_hashes_verified_before_and_after":True,
             "source_manifest_sha256":manifest_hash, "standalone":standalone_result, "e2e":e2e_result})
        status["state"] = "complete"
    except Exception as error:
        status["state"] = "failed"
        status["error"] = str(error)
        raise
    finally:
        status["finished_utc"] = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())
        save("status.json", status)


if __name__ == "__main__":
    main()
