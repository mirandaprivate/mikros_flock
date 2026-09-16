#!/usr/bin/env python3
"""Audit downloaded skip7-v1 evidence without modifying the frozen snapshot sources.

Writes local_audit.json and audit_statistics.json beside source_manifest.json.
Remote absolute executable paths are resolved only by their validated basename
inside the downloaded evidence-binaries directory; they are never opened.
"""
import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import statistics
import tarfile
from datetime import datetime, timezone

import run_skip7
from run_engineering import COMMON_ENV
from run_column_projection import CPU_SPEC, PHASES, TOOLCHAIN, digest, require

ARCHIVE_SHA256 = "6ab2ef00bd57adef1b17868885bf9bf898dc074df060780aa958fe3a5994ef82"
EXECUTABLES = {"mikros": "e2e_sha256_paired", "flock": "e2e_flock_baseline"}


def read_json(path):
    return json.loads(path.read_text(encoding="utf-8"))


def save_json(path, value):
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def inside(base, relative):
    require(type(relative) is str and relative and "\\" not in relative, "invalid relative artifact path")
    parts = PurePosixPath(relative)
    require(not parts.is_absolute() and ".." not in parts.parts and ":" not in relative, "unsafe artifact path")
    result = base.joinpath(*parts.parts).resolve()
    require(result.is_relative_to(base.resolve()) and result.is_file() and not result.is_symlink(), "artifact missing/outside root")
    return result


def remote_file_name(value):
    require(type(value) is str and value.startswith("/") and "\\" not in value, "expected remote POSIX path")
    path = PurePosixPath(value)
    require(".." not in path.parts and path.name in EXECUTABLES.values(), "unexpected remote executable")
    return path.name


def statistics_report(logs, recomputed):
    result = {"schema_version": 1, "workload": "256 independent SHA-256 compression calls; public CSC; C=I; native Ligerito Fast PCS",
              "threads": 64, "timing_scope": "prover with commitment; offline fixture/witness/input clone excluded; verify and warmup separate",
              "formal_per_mode": 18, "warmups_per_mode": 3, "modes": {}}
    for mode in run_skip7.MODES:
        batches, formal, warmups = [], [], []
        for batch in range(3):
            _, samples, _ = run_skip7.checked_records(logs / f"{mode}-{batch}.jsonl", mode)
            rows = [r for r in samples if not r["warmup"]]
            warm = [r for r in samples if r["warmup"]]
            elapsed = lambda row: row["timings"]["prover_ms"] if mode == "mikros" else row["prove_with_commit_ms"]
            formal.extend(rows)
            warmups.extend(elapsed(row) for row in warm)
            batches.append({"batch": batch, "formal_samples": len(rows),
                            "median_prover_ms": statistics.median(elapsed(row) for row in rows),
                            "mean_prover_ms": statistics.mean(elapsed(row) for row in rows),
                            "warmup_prover_ms": [elapsed(row) for row in warm]})
        values = [elapsed(row) for row in formal]
        summary = recomputed["modes"][mode]
        row = {"median_prover_ms": statistics.median(values), "mean_prover_ms": statistics.mean(values),
               "min_prover_ms": min(values), "max_prover_ms": max(values),
               "median_verify_ms": statistics.median(r["verify_ms"] for r in formal),
               "mean_verify_ms": statistics.mean(r["verify_ms"] for r in formal),
               "proof_bytes": summary["proof_bytes"], "proof_blake3": summary["proof_blake3"],
               "commitment_blake3": summary["commitment_blake3"], "batches": batches,
               "warmup_prover_ms": warmups, "warmup_median_ms": statistics.median(warmups),
               "warmup_mean_ms": statistics.mean(warmups)}
        if mode == "mikros":
            row["phases_ms"] = {phase: {"median": statistics.median(r["timings"][phase] for r in formal),
                                           "mean": statistics.mean(r["timings"][phase] for r in formal)} for phase in PHASES}
        result["modes"][mode] = row
    for metric in ("median_prover_ms", "mean_prover_ms"):
        times = {mode: row[metric] for mode, row in result["modes"].items()}
        result[f"ratios_of_{metric}"] = {
            "flock6_over_mikros": times["flock6"] / times["mikros"],
            "flock7_over_mikros": times["flock7"] / times["mikros"],
            "flock7_over_flock6": times["flock7"] / times["flock6"],
            "flock6_over_flock7": times["flock6"] / times["flock7"],
            "mikros_time_reduction_vs_flock6_pct": 100 * (1 - times["mikros"] / times["flock6"]),
            "mikros_time_reduction_vs_flock7_pct": 100 * (1 - times["mikros"] / times["flock7"]),
        }
    result["negative_tests"] = {mode: record["rejected"] for mode, record in recomputed["negative_tests"].items()}
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--snapshot", type=Path, default=Path(__file__).resolve().parent / "skip7-v1")
    parser.add_argument("--repository", type=Path, default=Path(__file__).resolve().parents[3])
    args = parser.parse_args()
    snapshot, repository = args.snapshot.resolve(), args.repository.resolve()
    logs = snapshot / "evidence/results-skip7"
    archive, manifest_file = snapshot / "source.tar.gz", snapshot / "source_manifest.json"
    require(digest(archive) == ARCHIVE_SHA256, "frozen source archive hash differs")
    manifest = read_json(manifest_file)
    sources = manifest["files_sha256"]
    require(len(sources) == 403, "expected exactly 403 frozen source files")
    with tarfile.open(archive, "r:gz") as tar:
        names = tar.getnames()
        require(len(names) == len(set(names)) and set(names) == set(sources) | {"source_manifest.json"}, "archive member list differs")
        manifest_bytes = tar.extractfile("source_manifest.json").read()
        # package_e2e writes the local JSON with Windows CRLF but embeds LF
        # bytes in the tarball. Server build receipts hash those archive bytes.
        require(json.loads(manifest_bytes) == manifest, "manifest differs from authenticated archive")
        manifest_sha256 = hashlib.sha256(manifest_bytes).hexdigest()
        for name, expected in sources.items():
            member = tar.getmember(name)
            require(member.isfile() and not member.issym() and not member.islnk(), "source member is not a regular file")
            require(hashlib.sha256(tar.extractfile(member).read()).hexdigest() == expected, f"archive source hash: {name}")
            require(digest(inside(repository, name)) == expected, f"current source differs from snapshot: {name}")

    evidence = read_json(logs / "evidence_sha256.json")
    actual = {path.relative_to(logs).as_posix() for path in logs.rglob("*") if path.is_file()
              and path != logs / "evidence_sha256.json"}
    require(set(evidence) == actual, "missing or unlisted evidence files")
    for name, expected in evidence.items():
        require(digest(inside(logs, name)) == expected, f"evidence hash mismatch: {name}")
    receipt = read_json(logs / "build-receipt.json")
    require(receipt["source_manifest_sha256"] == manifest_sha256, "build receipt source differs")
    require(receipt["toolchain"] == TOOLCHAIN and receipt["rustflags"] == COMMON_ENV["RUSTFLAGS"], "build toolchain/config differs")
    built = receipt["binary_sha256"]
    require(len(built) == 2 and {remote_file_name(name) for name in built} == set(EXECUTABLES.values()), "build executable set differs")
    require(len({str(PurePosixPath(name).parents[2]) for name in built}) == 1, "build target directories differ")
    target = str(PurePosixPath(next(iter(built))).parents[2])
    expected_environment = {**COMMON_ENV, "CARGO_TARGET_DIR": target}
    executed = read_json(logs / "binary-sha256.json")
    require(set(executed) == {f"evidence-binaries/{name}" for name in EXECUTABLES.values()}, "executed binary set differs")
    for remote, expected in built.items():
        relative = f"evidence-binaries/{remote_file_name(remote)}"
        path = inside(logs, relative)
        header = path.read_bytes()[:20]
        require(header[:6] == b"\x7fELF\x02\x01" and header[18:20] == b"\x3e\x00", "expected Linux x86-64 ELF evidence")
        require(digest(path) == expected == executed[relative], "executed/build ELF hash differs")

    plan, status = read_json(logs / "run_plan.json"), read_json(logs / "status.json")
    order = [list(run_skip7.MODES[batch:] + run_skip7.MODES[:batch]) for batch in range(3)]
    require(plan["order"] == order and plan["reps"] == 6 and plan["warmups"] == 1 and plan["batches"] == 3, "process plan differs")
    require(plan["profiling_enabled"] is False and plan["mikros_uses_compiled_defaults"] is True, "profiling/optimization scope differs")
    require(plan["source_files"] == 403 and plan["source_manifest_sha256"] == manifest_sha256, "run plan source differs")
    runs = [(f"validation-{mode}", mode, True) for mode in run_skip7.MODES]
    runs += [(f"{mode}-{batch}", mode, False) for batch, sequence in enumerate(order) for mode in sequence]
    completed = [event["name"] for event in status["completed"]]
    require(status["state"] == "complete" and completed in (["build"] + [r[0] for r in runs], [r[0] for r in runs]), "incomplete/reordered execution")
    require(all(event["exit_code"] == 0 for event in status["completed"]), "unsuccessful command")
    for name, mode, validation in runs:
        command = read_json(logs / f"{name}.command.json")
        require(command[:3] == ["/usr/bin/time", "-v", "-o"] and command[4:7] == ["taskset", "-c", CPU_SPEC], "native affinity wrapper differs")
        exe_name = EXECUTABLES["mikros" if mode == "mikros" else "flock"]
        require(remote_file_name(command[7]) == exe_name
                and PurePosixPath(command[7]).parts[-3:] == (logs.name, "evidence-binaries", exe_name), "wrong executable path")
        require(command[7:] == run_skip7.command({"mikros": command[7], "flock": command[7]}, mode, validation), "wrong proof CLI flags")
        require(read_json(logs / f"{name}.environment.json") == expected_environment, "runtime environment differs")
        idle = read_json(logs / f"{name}.preflight.json")
        require(0 <= idle["selected_cpu_busy_fraction"] < 0.10 and idle["mem_available_kib"] > 8 * 1024 * 1024, "preflight thresholds failed")
    expected_jsonl = {f"{name}.jsonl" for name, _, _ in runs} | ({"build.jsonl"} if "build" in completed else set())
    require({p.name for p in logs.glob("*.jsonl")} == expected_jsonl, "unexpected/missing JSONL files")
    all_records = {path.name: [json.loads(line) for line in path.read_text(encoding="utf-8").splitlines() if line.strip()]
                   for path in logs.glob("*.jsonl")}
    record_count = sum(len(rows) for rows in all_records.values())
    recomputed = run_skip7.audit(logs)
    require(recomputed == read_json(logs / "summary.json"), "server summary differs from locally recomputed raw results")
    report = statistics_report(logs, recomputed)
    result = {"schema_version": 1, "passed": True, "audited_utc": datetime.now(timezone.utc).isoformat(),
              "source_archive_sha256": ARCHIVE_SHA256, "source_manifest_sha256": manifest_sha256,
              "local_manifest_file_sha256": digest(manifest_file),
              "current_sources_verified": len(sources), "archive_sources_verified": len(sources),
              "evidence_files_verified": len(evidence), "elf_files_verified": len(executed),
              "jsonl_files_verified": len(expected_jsonl), "jsonl_records_read": record_count,
              "formal_samples_verified": 54, "warmup_samples_verified": 9, "validation_samples_verified": 3,
              "raw_summary_exact_match": True, "complete_verification_and_fs": True,
              "same_public_statement": True, "same_flock6_flock7_commitment": True,
              "negative_tests": report["negative_tests"], "executed_binary_sha256": executed,
              "server_summary_sha256": digest(logs / "summary.json"),
              "statistics_file": "audit_statistics.json", "statistics_sha256": None,
              "local_audit_script_sha256": digest(Path(__file__))}
    save_json(snapshot / "audit_statistics.json", report)
    result["statistics_sha256"] = digest(snapshot / "audit_statistics.json")
    save_json(snapshot / "local_audit.json", result)
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
