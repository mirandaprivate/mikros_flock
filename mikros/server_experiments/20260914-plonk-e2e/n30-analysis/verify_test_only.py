"""Verify v14 changes only the final test module, preserving measured v13 code."""
import hashlib
import json
from pathlib import Path
import tarfile

base = Path(__file__).resolve().parents[1]
old = json.loads((base / "v13/source_manifest.json").read_text())
new = json.loads((base / "v14/source_manifest.json").read_text())
changed = [k for k in old["files_sha256"] if old["files_sha256"][k] != new["files_sha256"][k]]
assert old["files_sha256"].keys() == new["files_sha256"].keys()
assert changed == ["mikros/src/extension_product.rs"]
parts = []
for version, manifest in [("v13", old), ("v14", new)]:
    with tarfile.open(base / version / "source.tar.gz", "r:gz") as archive:
        data = archive.extractfile(changed[0]).read()
        assert hashlib.sha256(data).hexdigest() == manifest["files_sha256"][changed[0]]
        marker = b"#[cfg(test)]\nmod tests {"
        assert data.count(marker) == 1
        parts.append(data.split(marker)[0])
assert parts[0] == parts[1], "production source changed"
receipt = {"changed_files": changed, "change_is_in_final_test_module_only": True,
           "all_other_383_sources_byte_identical": True,
           "production_prefix_sha256": hashlib.sha256(parts[0]).hexdigest(),
           "v13_source_manifest_sha256": hashlib.sha256((base / "v13/source_manifest.json").read_bytes()).hexdigest(),
           "v14_source_manifest_sha256": hashlib.sha256((base / "v14/source_manifest.json").read_bytes()).hexdigest()}
(base / "v14/production_equivalence.json").write_text(json.dumps(receipt, indent=2) + "\n")
print(json.dumps(receipt))
