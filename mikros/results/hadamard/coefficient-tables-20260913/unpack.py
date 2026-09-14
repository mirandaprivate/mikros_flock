"""Verify and extract the credential-free server evidence without overwriting data."""
import argparse
import hashlib
from pathlib import Path
import tarfile

p = argparse.ArgumentParser(description=__doc__)
p.add_argument("archive", type=Path)
p.add_argument("sha256")
p.add_argument("destination", type=Path)
a = p.parse_args()
assert hashlib.sha256(a.archive.read_bytes()).hexdigest() == a.sha256
root = a.destination.resolve()
root.mkdir(parents=True, exist_ok=True)
with tarfile.open(a.archive) as archive:
    members = archive.getmembers()
    seen = set()
    for member in members:
        target = (root / member.name).resolve()
        assert member.isfile() and target.is_relative_to(root) and target != root
        assert target not in seen
        seen.add(target)
        data = archive.extractfile(member).read()
        if target.exists():
            assert target.read_bytes() == data, f"refusing to overwrite {target}"
        else:
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
print(f"Verified {len(seen)} evidence files")
