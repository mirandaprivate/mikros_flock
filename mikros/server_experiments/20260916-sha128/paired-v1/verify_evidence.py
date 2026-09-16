"""Verify downloaded bytes and recompute the server's benchmark summary."""
import hashlib
import json
from pathlib import Path
import re
import sys
import tarfile

root = Path(__file__).resolve().parent
sys.path.insert(0, str(root.parent))
from run_paired import audit


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


archive = root / "paired-v1-evidence.tar.gz"
expected = "2bf8520e2d4be0e048e2249abe73c347f56ebf56deeeaa71683647d48f0cfcd4"
assert digest(archive) == expected
destination = root / "evidence"
destination.mkdir(exist_ok=True)
with tarfile.open(archive) as tar:
    for member in tar.getmembers():
        assert (destination / member.name).resolve().is_relative_to(destination.resolve())
        assert member.isfile() or member.isdir()
    tar.extractall(destination, filter="data")
logs = destination / "results-paired256"
hashes = json.loads((logs / "evidence_sha256.json").read_text())
assert all(digest(logs / name) == expected for name, expected in hashes.items())
assert json.loads((logs / "status.json").read_text())["state"] == "complete"
summary = audit(logs)
assert summary == json.loads((logs / "summary.json").read_text())
tests = {}
for name in ("test-paired-pcs", "test-sha"):
    counts = re.findall(r"test result: ok\. (\d+) passed; 0 failed", (logs / f"{name}.jsonl").read_text())
    assert counts
    tests[name] = sum(map(int, counts))
assert tests == {"test-paired-pcs": 2, "test-sha": 16}
result = {
    "archive_sha256": expected,
    "evidence_files_verified": len(hashes),
    "summary_recomputed_and_matches": True,
    "tests_passed": tests,
    "source_archive_sha256": digest(root / "source.tar.gz"),
    "paired_over_separate_time_saved_percent": 100 * (1 - summary["paired"]["prover_ms"] / summary["separate"]["prover_ms"]),
    "paired_over_flock_time_saved_percent": 100 * (1 - summary["paired"]["prover_ms"] / summary["flock"]["prove_with_commit_ms"]),
    "paired_pcs_time_saved_percent": 100 * (1 - summary["paired"]["pcs_open_ms"] / summary["separate"]["pcs_open_ms"]),
}
(root / "local_audit.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps(result, indent=2))
