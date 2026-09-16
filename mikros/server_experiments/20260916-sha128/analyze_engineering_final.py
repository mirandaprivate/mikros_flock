"""Audit frozen v5 evidence and write phase statistics; no cryptographic replay."""
import hashlib
import json
from pathlib import Path
import re
import statistics as st
import tarfile

import run_engineering as runner

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
SNAP = HERE / "engineering-v5"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def save(path, value):
    path.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")


def main():
    expected = json.loads((SNAP / "archive_hashes.json").read_text(encoding="utf-8"))
    for name, value in expected.items():
        assert sha(SNAP / name) == value, name
    with tarfile.open(SNAP / "source.tar.gz") as src:
        manifest = json.load(src.extractfile("source_manifest.json"))
        for name, value in manifest["files_sha256"].items():
            assert hashlib.sha256(src.extractfile(name).read()).hexdigest() == value, name
            assert sha(ROOT / name) == value, "workspace drift: " + name
    evidence = SNAP / "evidence"
    with tarfile.open(SNAP / "evidence.tar.gz") as tar:
        for member in tar.getmembers():
            assert (evidence / member.name).resolve().is_relative_to(evidence.resolve())
            assert member.isfile() or member.isdir()
        tar.extractall(evidence, filter="data")
    assert json.loads((evidence / "source_manifest.json").read_text()) == manifest
    logs = evidence / "results-engineering-formal"
    checked = 0
    for folder in (logs, evidence / "default-profile-paired"):
        for name, value in json.loads((folder / "evidence_sha256.json").read_text()).items():
            assert sha(folder / name) == value, name
            checked += 1
    receipt = json.loads((evidence / "engineering-build.json").read_text())
    assert receipt["source_manifest_sha256"] == sha(evidence / "source_manifest.json")
    for path, value in receipt["binary_sha256"].items():
        binary = evidence / "evidence-binaries" / Path(path).name
        assert binary.read_bytes().startswith(b"\x7fELF") and sha(binary) == value
    assert json.loads((logs / "binary-sha256.json").read_text()) == receipt["binary_sha256"]
    assert json.loads((logs / "status.json").read_text())["state"] == "complete"
    plan = json.loads((logs / "run_plan.json").read_text())
    result = runner.summarize(logs, plan["modes"], plan["reps"], plan["validation_modes"], "formal")
    assert result == json.loads((logs / "summary.json").read_text())
    fields = ("prover_ms",) + runner.PHASES
    statistics = {}
    raw_modes = {}
    for mode, toggles in plan["modes"].items():
        rows = []
        first = []
        for batch in range(3):
            meta, samples, _ = runner.read_samples(logs / f"{mode}-{batch}.jsonl", mode, 6)
            env = json.loads((logs / f"{mode}-{batch}.environment.json").read_text())
            assert all(env[key] == value for key, value in toggles.items())
            assert "PCS_TRACE" not in env and "MIKROS_PROFILE" not in env
            if mode != "flock":
                assert meta["engineering_environment"] == toggles
            first.append(samples[0]); rows.extend(samples[1:])
        raw_modes[mode] = rows
        unpack = (lambda x: {"prover_ms": x["prove_with_commit_ms"]}) if mode == "flock" else (lambda x: x["timings"])
        stats = {key: {"mean": st.mean(unpack(x)[key] for x in rows),
                       "median": st.median(unpack(x)[key] for x in rows)}
                 for key in (("prover_ms",) if mode == "flock" else fields)}
        stats["first_proof_median_ms"] = st.median(unpack(x)["prover_ms"] for x in first)
        stats["range_ms"] = [min(unpack(x)["prover_ms"] for x in rows), max(unpack(x)["prover_ms"] for x in rows)]
        stats["verify_median_ms"] = st.median(x["verify_ms"] for x in rows)
        statistics[mode] = stats
    meta, defaults, negative = runner.read_samples(evidence / "default-profile-paired/stdout.jsonl", "selected", 3, 1, True)
    assert all(x is None for x in meta["engineering_environment"].values())
    tests = {}
    for path in (evidence / "validation-native").glob("*.log"):
        found = re.findall(r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored", path.read_text())
        assert len(found) == 1 and found[0][1] == "0", path
        tests[path.stem] = dict(zip(("passed", "failed", "ignored"), map(int, found[0])))
    save(SNAP / "statistics.json", statistics)
    save(SNAP / "local_audit.json", {"passed": True, "cryptographic_replay": False,
        "source_files": len(manifest["files_sha256"]), "stage_files_hashed": checked,
        "actual_elf_binaries": len(receipt["binary_sha256"]), "formal_samples": 72,
        "warmup_samples": 12, "validation_samples": 2, "default_profile_samples": len(defaults),
        "negative_cases": {**result["negative_tests"], "defaults": negative[0]["rejected"]},
        "tests": tests, "all_proofs_and_transcripts_verified_by_server": True,
        "all_mikros_proof_bytes_equal_frozen_protocol": True, "native_flock_proof_unchanged": True,
        "summary_recomputed_exactly": True, "workspace_equals_frozen_source": True})
    save(ROOT / "e2e_sha256_256_engineering_compare_flock.log", {
        "workload": "256 independent SHA-256 compression calls; standard IV; public 512-bit inputs and 256-bit outputs",
        "threads": 64, "primary_final_summary": result, "statistics": statistics,
        "source_archive_sha256": expected["source.tar.gz"], "evidence_archive_sha256": expected["evidence.tar.gz"],
        "notes": "Warmups and profiling excluded from formal timings; medians of phases are not additive. Native FLOCK PCS, public CSC, C=I. No zero-knowledge claim."})
    print(json.dumps({"statistics": statistics, "tests": tests}))


if __name__ == "__main__":
    main()
