#!/usr/bin/env python3
"""Compare legacy and column Mikros reductions on one public-CSC SHA snapshot.

Run from the extracted source snapshot root:
  python3 mikros/server_experiments/20260916-sha128/run_column_projection.py \\
      --target /absolute/path/to/cargo-target --logs results-column-projection256

Both Mikros modes explicitly select paired openings. Each of three rotating
process batches runs legacy, column and native FLOCK once, with one warmup and
six formal repetitions per process. This driver shares low-level utilities
with run_public_csc, but computes its own mode-aware summary.
"""

import argparse
import json
import math
import os
from pathlib import Path
import re
import statistics
import subprocess
import time

from run_public_csc import CPU_SPEC, FEATURES, TOOLCHAIN, digest, preflight, records, save


MODES = ("legacy", "column", "flock")
REDUCTIONS = MODES[:2]
PHASES = ("commit_ms", "reduction_f128_ms", "public_io_ms", "pair_reduction_ms", "pcs_open_ms")
SHARED_DIGESTS = (
    "fixture_digest_blake3", "circuit_digest_blake3", "r1cs_digest_blake3", "prepared_witness_blake3",
)
CLAIM_COUNTS = ("source_claims", "ring_claims", "inner_openings")
TEST_COUNTS = {"test-paired-pcs": 2, "test-image": 12, "test-shared-hadamard": 2, "test-sha": None}
NEGATIVE_COUNTS = {"legacy": 25, "column": 36}
HEX256 = re.compile(r"[0-9a-f]{64}\Z")


def require(condition, message):
    if not condition:
        raise ValueError(message)


def number(value, label, positive=False):
    require(type(value) in (int, float) and math.isfinite(value), f"invalid number: {label}")
    require(value > 0 if positive else value >= 0, f"invalid timing: {label}")


def checked_hash(value, label):
    require(isinstance(value, str) and HEX256.fullmatch(value), f"invalid hash: {label}")


def batch_order(batch):
    require(type(batch) is int and 0 <= batch < 3, "batch must be 0, 1 or 2")
    return MODES[batch:] + MODES[:batch]


def benchmark_command(binaries, mode, validation=False):
    require(mode in MODES, f"unknown mode: {mode}")
    require(not validation or mode in REDUCTIONS, "validation is only defined for Mikros reductions")
    if mode == "flock":
        command = [binaries["flock"], "--compressions", "256", "--profile", "fast"]
    else:
        command = [binaries["mikros"], "--opening", "paired", "--reduction", mode]
    command += ["--threads", "64", "--reps", "1" if validation else "6",
                "--warmups", "0" if validation else "1"]
    if validation:
        command.append("--tamper")
    return command


def checked_records(path, mode, validation=False):
    meta, rows = records(path)
    require(rows and rows[0] is meta and sum(r.get("record") == "metadata" for r in rows) == 1,
            f"metadata record shape: {path.name}")
    require(meta["schema_version"] == (1 if mode == "flock" else 2), f"metadata schema differs: {path.name}")
    require(meta["fixture"] == "sha256-independent-compressions" and meta["sha_compressions"] == 256,
            f"wrong SHA workload: {path.name}")
    require(meta["threads"] == meta["all_core_threads"] == 64, f"wrong thread pools: {path.name}")
    require(FEATURES.issubset(meta["target_features"]), f"missing native features: {path.name}")
    require(meta["r1cs_c_identity"] is True and meta["r1cs_k_log"] == 15 and meta["native_k_skip"] == 6,
            f"wrong public R1CS dimensions/C=I: {path.name}")
    require("public CSC" in meta["matrix_authentication"], f"wrong matrix authentication: {path.name}")
    for field in SHARED_DIGESTS:
        checked_hash(meta[field], field)
    expected_pcs = {"m": 23, "profile": "fast", "log_batch_size": 6, "log_inv_rate": 1,
                    "merkle_hash": "blake3", "num_lanes": 50 if mode == "flock" else None}
    require(meta["pcs_params"] == expected_pcs, f"native PCS settings/layout differ: {path.name}")
    require(meta["pcs_queries"] == [244, 79, 48], f"PCS query counts differ: {path.name}")
    if mode in REDUCTIONS:
        require(meta["reduction"] == mode, f"metadata reduction mismatch: {path.name}")
        require(meta["zero_knowledge"] is False and meta["packed_direct"] == "1", "wrong Mikros PCS/proof mode")
        for field, expected in {"reduction_field_bits": 128, "opening_logical_field_bits": 256,
                                "pcs_carrier_field_bits": 128, "instance_groups": 2,
                                "instances_per_group": 128, "carrier_words": 65536,
                                "logical_f256_words": 32768, "carrier_bytes": 1048576}.items():
            require(meta[field] == expected, f"Mikros representation differs: {field}")
    else:
        require(meta["system"] == "flock-circuit-e2e" and meta["independent_compressions"] is True,
                "wrong FLOCK baseline")
        require(meta["reps"] == 6 and meta["warmups"] == 1 and meta["zero_knowledge_claim"] is False,
                "wrong FLOCK run configuration")
        require(meta["pcs_query_grinding_bits"] == [16, 16, 16]
                and meta["pcs_fold_grinding_bits"] == [0, 0, 0], "wrong native PCS grinding")
        require(meta["verification"] == "complete non-deferred verifier; no witness input; matrix and sigma obligations discharged",
                "FLOCK verifier obligations differ")
    samples = [row for row in rows if row.get("record") == "sample"]
    negatives = [row for row in rows if row.get("record") == "negative_tests"]
    require(len(rows) == 1 + len(samples) + len(negatives), f"unknown records: {path.name}")
    require(len(samples) == (1 if validation else 7), f"wrong sample count: {path.name}")
    require([row["iteration"] for row in samples] == list(range(len(samples))), f"iteration sequence: {path.name}")
    require(len(negatives) == (1 if validation else 0), f"unexpected negative-test records: {path.name}")
    for row in samples:
        require(type(row["iteration"]) is int and type(row["warmup"]) is bool, "invalid sample flags")
        require(row["warmup"] == (not validation and row["iteration"] == 0), "warmup/iteration mismatch")
        require(row["verified"] is True and row["transcript_match"] is True, "incomplete verification or Fiat-Shamir mismatch")
        require(type(row["proof_bytes"]) is int and row["proof_bytes"] > 0, "invalid proof size")
        for field in ("proof_blake3", "commitment_blake3"):
            checked_hash(row[field], field)
        number(row["verify_ms"], "verify_ms", True)
        if mode == "flock":
            require(row["system"] == "flock-circuit-e2e", "wrong FLOCK sample")
            number(row["prove_with_commit_ms"], "prove_with_commit_ms", True)
        else:
            require(row["system"] == "mikros" and row["opening"] == "paired"
                    and row["reduction"] == mode, "wrong Mikros opening/reduction")
            checked_hash(row["piop_blake3"], "piop_blake3")
            times = row["timings"]
            number(times["prover_ms"], "prover_ms", True)
            for phase in PHASES:
                number(times[phase], phase)
            require(sum(times[p] for p in PHASES) <= times["prover_ms"] + 1e-5, "phase sum exceeds prover scope")
            for field in CLAIM_COUNTS:
                require(type(times[field]) is int and times[field] > 0, f"invalid claim count: {field}")
            require(times["inner_openings"] == 1, "expected one native inner opening")
    for row in negatives:
        require(row["opening"] == "paired" and row.get("reduction", mode) == mode,
                "negative-test opening/reduction mismatch")
        require(type(row["rejected"]) is int and row["rejected"] == NEGATIVE_COUNTS[mode],
                f"negative-test suite count differs: {mode}")
    return meta, samples, negatives


def test_results(logs):
    result = {}
    for name, expected_count in TEST_COUNTS.items():
        text = (logs / f"{name}.jsonl").read_text(encoding="utf-8")
        matches = re.findall(r"test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out", text)
        require(matches and all(m[0] == "ok" and int(m[2]) == int(m[3]) == int(m[4]) == 0 for m in matches),
                f"failed/ignored/measured tests: {name}")
        passed = sum(int(m[1]) for m in matches)
        require(passed > 0, f"empty test filter: {name}")
        require(expected_count is None or passed == expected_count, f"test count differs: {name}")
        result[name] = passed
    return result


def audit(logs):
    """Recompute each reduction separately; all warmup and validation rows count for integrity."""
    formal = {mode: [] for mode in MODES}
    all_samples = {mode: [] for mode in MODES}
    metadata = {mode: [] for mode in MODES}
    negatives = {}
    for batch in range(3):
        for mode in batch_order(batch):
            meta, samples, _ = checked_records(logs / f"{mode}-{batch}.jsonl", mode)
            metadata[mode].append(meta)
            all_samples[mode].extend(samples)
            formal[mode].extend(row for row in samples if not row["warmup"])
    validation_metadata = {}
    for mode in REDUCTIONS:
        meta, samples, rejected = checked_records(logs / f"validation-{mode}.jsonl", mode, True)
        validation_metadata[mode] = meta
        all_samples[mode].extend(samples)
        negatives[mode] = rejected[0]
    reference = metadata["legacy"][0]
    every_meta = [meta for group in metadata.values() for meta in group] + list(validation_metadata.values())
    for meta in every_meta:
        for field in SHARED_DIGESTS:
            require(meta[field] == reference[field], f"different SHA/R1CS/prepared witness: {field}")
    for mode in MODES:
        varying = {"offline_fixture_ms", "offline_witness_ms", "offline_transpose_ms"}
        mode_meta = metadata[mode] + ([validation_metadata[mode]] if mode in REDUCTIONS else [])
        first = {k: v for k, v in mode_meta[0].items() if k not in varying}
        require(all({k: v for k, v in meta.items() if k not in varying} == first for meta in mode_meta),
                f"metadata drifts within {mode}")
        for field in ("proof_blake3", "proof_bytes", "commitment_blake3"):
            require(len({row[field] for row in all_samples[mode]}) == 1, f"unstable {mode} {field}")
        if mode in REDUCTIONS:
            require(len({row["piop_blake3"] for row in all_samples[mode]}) == 1, f"unstable {mode} PIOP")
            for field in CLAIM_COUNTS:
                require(len({row["timings"][field] for row in all_samples[mode]}) == 1,
                        f"unstable {mode} claim count: {field}")
    require(len({row["commitment_blake3"] for mode in REDUCTIONS for row in all_samples[mode]}) == 1,
            "Mikros reductions commit different witnesses")
    tests = test_results(logs)
    result = {
        "instances": 256, "threads": 64, "batches": 3, "formal_repetitions": 18,
        "warmups_per_batch": 1, "repetitions_per_batch": 6,
        "process_order_by_batch": [list(batch_order(batch)) for batch in range(3)],
        "metadata": metadata, "validation_metadata": validation_metadata,
        "same_sha_public_statement": True, "same_r1cs_and_native_prepared_witness": True,
        "same_mikros_commitment": True, "matrix_authentication": "public CSC on all sides; native C=I",
        "native_pcs_backend": "FLOCK Ligerito Fast", "pcs_layouts_differ": True,
        "mikros_opening": "paired", "negative_tests": negatives,
        "negative_test_counts": {mode: row["rejected"] for mode, row in negatives.items()},
        "tests_passed": tests, "total_tests_passed": sum(tests.values()),
        "comparison_scope": "legacy and column reductions use the same Mikros commitment and paired opening policy; native FLOCK includes its full prover scope",
    }
    for mode, rows in formal.items():
        require(len(rows) == 18, f"expected 18 formal {mode} samples")
        times = rows if mode == "flock" else [row["timings"] for row in rows]
        fields = ("prove_with_commit_ms",) if mode == "flock" else ("prover_ms",) + PHASES
        summary = {field: statistics.median(row[field] for row in times) for field in fields}
        summary.update(samples=len(rows), warmup_samples=3, proof_bytes=rows[0]["proof_bytes"],
                       proof_blake3=rows[0]["proof_blake3"], commitment_blake3=rows[0]["commitment_blake3"],
                       verify_ms=statistics.median(row["verify_ms"] for row in rows),
                       prover_range_ms=[min(row[fields[0]] for row in times), max(row[fields[0]] for row in times)])
        if mode in REDUCTIONS:
            summary.update({field: times[0][field] for field in CLAIM_COUNTS})
            summary.update(reduction=mode, opening="paired", piop_blake3=rows[0]["piop_blake3"])
        result[mode] = summary
    result["legacy_over_column"] = result["legacy"]["prover_ms"] / result["column"]["prover_ms"]
    result["flock_over_column"] = result["flock"]["prove_with_commit_ms"] / result["column"]["prover_ms"]
    result["flock_over_legacy"] = result["flock"]["prove_with_commit_ms"] / result["legacy"]["prover_ms"]
    result["column_time_saved_percent_vs_legacy"] = 100 * (1 - result["column"]["prover_ms"] / result["legacy"]["prover_ms"])
    return result


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--target", required=True, type=Path)
    parser.add_argument("--logs", default="results-column-projection256", type=Path)
    args = parser.parse_args(argv)
    root = Path.cwd().resolve()
    logs = (root / args.logs).resolve()
    require(logs.is_relative_to(root), "logs must be inside the source snapshot")
    target = args.target.resolve()
    manifest = json.loads((root / "source_manifest.json").read_text(encoding="utf-8"))
    source_files = manifest["files_sha256"]
    own_path = Path(__file__).resolve().relative_to(root).as_posix()
    require(own_path in source_files, "new runner is not frozen in source_manifest.json")
    logs.mkdir(exist_ok=False)
    env = {k: v for k, v in os.environ.items() if not k.startswith(("FLOCK_", "MIKROS_", "E2E_", "LIG_"))
           and k not in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER", "PCS_TRACE", "LD_PRELOAD")}
    env.update(RUSTFLAGS="-C target-cpu=native", RAYON_NUM_THREADS="64", CARGO_TARGET_DIR=str(target),
               FLOCK_PCS_PACKED_DIRECT="1", FLOCK_WP_UNINIT="1")
    env["PATH"] = str(Path.home() / ".cargo/bin") + ":" + env.get("PATH", "")
    status = {"state": "starting", "completed": []}

    def verify_sources():
        for name, expected in source_files.items():
            path = (root / name).resolve()
            require(path.is_relative_to(root) and path.is_file(), f"unsafe/missing source: {name}")
            require(digest(path) == expected, f"source changed: {name}")

    def run(name, command, pinned=True):
        status["state"] = name
        save(logs / "status.json", status)
        if pinned:
            command = ["/usr/bin/time", "-v", "-o", str(logs / f"{name}.resource.txt"), "taskset", "-c", CPU_SPEC] + command
        save(logs / f"{name}.command.json", command)
        started = time.monotonic()
        with (logs / f"{name}.jsonl").open("w") as out, (logs / f"{name}.stderr.log").open("w") as err:
            process = subprocess.run(command, env=env, cwd=root, stdout=out, stderr=err)
        event = {"name": name, "exit_code": process.returncode, "wall_seconds": time.monotonic() - started}
        status["completed"].append(event)
        save(logs / "status.json", status)
        print(json.dumps(event), flush=True)
        require(process.returncode == 0, f"stage failed: {event}")

    try:
        verify_sources()
        save(logs / "hardware.json", {"cpuinfo": Path("/proc/cpuinfo").read_text(), "meminfo": Path("/proc/meminfo").read_text(),
             "affinity": CPU_SPEC, "utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
             "environment": {k: v for k, v in env.items() if k.startswith(("FLOCK_", "MIKROS_", "RAYON_", "RUSTFLAGS"))}})
        save(logs / "preflight.json", preflight())
        for name in source_files:
            os.utime(root / name, None)
        cargo = ["cargo", TOOLCHAIN]
        flags = ["--release", "--offline", "--locked", "-j8"]
        run("build", cargo + ["build"] + flags + ["-p", "mikros", "--example", "e2e_sha256_paired", "--example", "e2e_flock_baseline"], False)
        run("test-paired-pcs", cargo + ["test"] + flags + ["-p", "flock-core", "pcs::paired::tests", "--", "--test-threads=1"])
        run("test-image", cargo + ["test"] + flags + ["-p", "mikros", "--lib", "multiple_image::tests", "--", "--test-threads=1"])
        run("test-shared-hadamard", cargo + ["test"] + flags + ["-p", "mikros", "--lib", "hadamard::shared_batch_tests", "--", "--test-threads=1"])
        run("test-sha", cargo + ["test"] + flags + ["-p", "mikros", "--example", "e2e_sha256_paired", "--", "--test-threads=1"])
        test_results(logs)
        binaries = {"mikros": str(target / "release/examples/e2e_sha256_paired"),
                    "flock": str(target / "release/examples/e2e_flock_baseline")}
        hashes = {path: digest(Path(path)) for path in binaries.values()}
        save(logs / "binary-sha256.json", hashes)
        save(logs / "run_plan.json", {
            "modes": list(MODES), "process_order_by_batch": [list(batch_order(batch)) for batch in range(3)],
            "formal_commands": {mode: benchmark_command(binaries, mode) for mode in MODES},
            "validation_commands": {mode: benchmark_command(binaries, mode, True) for mode in REDUCTIONS},
        })
        for mode in REDUCTIONS:
            save(logs / f"validation-{mode}.preflight.json", preflight())
            run(f"validation-{mode}", benchmark_command(binaries, mode, True))
        for batch in range(3):
            for mode in batch_order(batch):
                save(logs / f"{mode}-{batch}.preflight.json", preflight())
                run(f"{mode}-{batch}", benchmark_command(binaries, mode))
        save(logs / "summary.json", audit(logs))
        verify_sources()
        require(all(digest(Path(path)) == expected for path, expected in hashes.items()), "binary changed during run")
        status["state"] = "complete"
    except Exception as error:
        status.update(state="failed", error=str(error))
        raise
    finally:
        save(logs / "status.json", status)
        save(logs / "evidence_sha256.json", {p.name: digest(p) for p in sorted(logs.iterdir())
             if p.name != "evidence_sha256.json" and p.is_file()})


if __name__ == "__main__":
    main()
