"""Validate and unpack the immutable server evidence, rejecting unsafe paths."""
import hashlib
import json
from pathlib import Path
import tarfile

root = Path(__file__).resolve().parent
archive = root / "evidence.tar.gz"
expected = "e28d6a4a2c546bbb942cf2804a1593be70621e7bb1b95746d9c78713f1eba16a"
assert hashlib.sha256(archive.read_bytes()).hexdigest() == expected
with tarfile.open(archive) as tar:
    members = tar.getmembers()
    assert len({m.name for m in members}) == len(members)
    for member in members:
        path = (root / member.name).resolve()
        assert member.isfile() and path.is_relative_to(root)
        assert not Path(member.name).is_absolute()
        path.parent.mkdir(parents=True, exist_ok=True)
        data = tar.extractfile(member).read()
        assert len(data) == member.size
        path.write_bytes(data)
manifest = json.loads((root / "evidence-manifest.json").read_text())
for name, digest in manifest.items():
    path = (root / name).resolve()
    assert path.is_relative_to(root)
    assert hashlib.sha256(path.read_bytes()).hexdigest() == digest, name
print(json.dumps({"verified_files": len(manifest), "archive_sha256": expected}))
