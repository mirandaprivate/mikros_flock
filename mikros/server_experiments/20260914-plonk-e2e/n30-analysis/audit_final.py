"""Audit the final native validation and its test-only source transition."""
import json
from pathlib import Path
import re
import runpy

from report_tree import audit, digest, IDENTITY, read

base = Path(__file__).resolve().parents[1]
root = base.parents[2]
manifest = read(base / "v14/source_manifest.json")
assert len(manifest["files_sha256"]) == 384
for name, expected in manifest["files_sha256"].items():
    assert digest(root / name) == expected, f"working source drift: {name}"
runpy.run_path(str(base / "n30-analysis/verify_test_only.py"))
manifests = {digest(base / v / "source_manifest.json") for v in ("v13", "v14")}
folder = base / "v14/evidence-validation/native-pilot14-large-pool"
config = read(folder / "config.json")
assert not config["tests_skipped"] and not config["build_skipped"]
assert config["tree_large_pool"]
new = audit(folder, manifests)
old = audit(base / "v13/evidence-n26/native-pilot14-register-reference", manifests)
for row in new:
    prior = next(x for x in old if x["case"]["id"] == row["case"]["id"])
    assert all(row["sample"][k] == prior["sample"][k] for k in IDENTITY)

status = read(folder / "status.json")
receipt = read(folder / "evidence_sha256.json")
counts = {"passed": 0, "failed": 0, "ignored": 0}
commands = [c for c in status["commands"] if c["id"].startswith("test-")]
assert len(commands) == 20
for command in commands:
    assert command["state"] == "complete" and command["exit_code"] == 0
    name = command["id"] + ".stdout.jsonl"
    assert name in receipt
    results = re.findall(r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;", (folder / name).read_text())
    assert results, f"missing Rust test results: {name}"
    for result in results:
        for key, number in zip(counts, result):
            counts[key] += int(number)
assert counts == {"passed": 234, "failed": 0, "ignored": 1}
historical = {
    "e2e_single_compare_flock.log": "50863ef419bd50027af244489d2f2bec126d7373a39cdef9e3480e3a86b772ff",
    "e2e_multiple_compare_flock.log": "cfe3116c090a655227d708c87836846d1d11915418780ffd8d3112a35dacc942",
}
assert all(digest(root / name) == expected for name, expected in historical.items())
result = {
    "native_tests": counts,
    "native_test_commands": len(commands),
    "large_pool_enabled_during_native_tests": True,
    "v14_pilot_full_proofs_equal_v13": True,
    "current_384_sources_match_v14": True,
    "historical_root_logs_unchanged": historical,
    "v14_source_manifest_sha256": digest(base / "v14/source_manifest.json"),
    "v14_native_binary_sha256": read(folder / "build-receipt.json")["binary_sha256"],
}
(base / "v14/final_validation.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps(result, indent=2))
