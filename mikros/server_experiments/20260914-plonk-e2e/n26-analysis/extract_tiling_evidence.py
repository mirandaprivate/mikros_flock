"""Extract only the hash-verified v11 evidence archive into a fresh directory."""
import hashlib
import json
from pathlib import Path
import tarfile

from write_tiling_report import VERSION, EVIDENCE, archive_files, safe_member

EXPECTED = "5b3ec90bdd6d0e86771e0c48a0e30e0d6ab0f8d9076bd680ef4960d1ed20afda"
archive = VERSION / "evidence-v11.tar.gz"
digest = hashlib.sha256(archive.read_bytes()).hexdigest()
assert digest == EXPECTED, "Server/download SHA-256 mismatch"
members = archive_files(archive)
assert not EVIDENCE.exists(), "Refuse to overwrite an existing extraction"
with tarfile.open(archive, "r:gz") as stream:
    EVIDENCE.mkdir()
    for item in stream.getmembers():
        relative = safe_member(item.name.rstrip("/"))
        destination = EVIDENCE.joinpath(*relative.parts)
        assert destination.resolve().is_relative_to(EVIDENCE.resolve())
        if item.isdir():
            destination.mkdir(parents=True, exist_ok=True)
        else:
            assert item.isfile()
            data = stream.extractfile(item).read()
            assert hashlib.sha256(data).hexdigest() == members[item.name]
            destination.parent.mkdir(parents=True, exist_ok=True)
            with destination.open("xb") as output:
                output.write(data)
receipt = {"archive_sha256": digest, "archive_bytes": archive.stat().st_size,
           "server_sha256_verified": True, "regular_files": len(members)}
(VERSION / "evidence_receipt.json").write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8")
print(json.dumps(receipt))
