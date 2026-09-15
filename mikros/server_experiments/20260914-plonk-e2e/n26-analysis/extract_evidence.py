"""Verify a retrieved evidence archive before adding files to a local directory."""
import argparse
import hashlib
from pathlib import Path
import tarfile

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('archive', type=Path)
parser.add_argument('sha256')
parser.add_argument('destination', type=Path)
args = parser.parse_args()
assert hashlib.sha256(args.archive.read_bytes()).hexdigest() == args.sha256
root = args.destination.resolve()
root.mkdir(parents=True, exist_ok=True)
with tarfile.open(args.archive) as archive:
    members = archive.getmembers()
    for member in members:
        target = (root / member.name).resolve()
        assert target.is_relative_to(root), member.name
        assert member.isdir() or member.isfile(), member.name
        if member.isfile() and target.exists():
            assert target.read_bytes() == archive.extractfile(member).read(), member.name
    for member in members:
        target = root / member.name
        if member.isdir():
            target.mkdir(parents=True, exist_ok=True)
        elif not target.exists():
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(archive.extractfile(member).read())
print(f'Verified archive SHA-256 and {len(members)} members; extracted to {root}')
