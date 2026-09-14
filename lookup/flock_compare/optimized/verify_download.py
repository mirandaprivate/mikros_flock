"""Verify and unpack only this experiment's immutable result bundle."""
import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import tarfile

parser = argparse.ArgumentParser()
parser.add_argument("archive", type=Path)
parser.add_argument("--sha256", required=True)
args = parser.parse_args()
archive = args.archive.resolve()
root = archive.parent
digest = hashlib.sha256(archive.read_bytes()).hexdigest()
assert digest == args.sha256.lower(), "archive digest mismatch"

with tarfile.open(archive, "r:gz") as bundle:
    members = bundle.getmembers()
    names = [m.name for m in members]
    assert len(names) == len(set(names)), "duplicate archive member"
    assert all(m.isfile() for m in members), "bundle must contain only regular files"
    assert "results_manifest.json" in names
    manifest = json.load(bundle.extractfile("results_manifest.json"))
    expected = manifest["files_sha256"]
    assert set(names) == set(expected) | {"results_manifest.json"}
    payloads = []
    for member in members:
        relative = PurePosixPath(member.name)
        assert not relative.is_absolute() and ".." not in relative.parts
        assert relative.parts[0] == "results" or member.name in ("driver.log", "results_manifest.json")
        dest = (root / Path(*relative.parts)).resolve()
        assert root in dest.parents and not dest.exists(), f"unsafe/existing path {dest}"
        payload = bundle.extractfile(member).read()
        if member.name != "results_manifest.json":
            assert hashlib.sha256(payload).hexdigest() == expected[member.name], member.name
        payloads.append((dest, payload))
    # Write only after every member and digest has been checked.
    for dest, payload in payloads:
        dest.parent.mkdir(parents=True, exist_ok=True)
        with dest.open("xb") as out:
            out.write(payload)

record = {"archive": archive.name, "archive_sha256": digest, "verified_files": len(expected)}
(root / "download_validation.json").write_text(json.dumps(record, indent=2) + "\n")
print(json.dumps(record))
