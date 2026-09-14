"""Verify the frozen evidence archive and extract without overwriting files."""
from pathlib import Path
import hashlib
import json
import tarfile

root = Path(__file__).resolve().parent
archive = root / 'evidence.tar.gz'
assert hashlib.sha256(archive.read_bytes()).hexdigest() == '27f1f8a0f3e6b295a7ef8b4d48434481242293731b171f239ee975e429badeaf'
destination = root / 'server_evidence'
with tarfile.open(archive) as tar:
    members = tar.getmembers()
    assert len({m.name for m in members}) == len(members)
    contents = {}
    for member in members:
        target = (destination / member.name).resolve()
        assert member.isfile() and target.is_relative_to(destination.resolve())
        contents[member.name] = tar.extractfile(member).read()
    manifest = json.loads(contents['evidence-manifest.json'])
    assert set(contents) == set(manifest) | {'evidence-manifest.json'}
    for name, digest in manifest.items():
        assert hashlib.sha256(contents[name]).hexdigest() == digest, name
    for name, data in contents.items():
        target = destination / name
        if target.exists():
            assert target.read_bytes() == data, name
        else:
            target.parent.mkdir(parents=True, exist_ok=True)
            with target.open('xb') as output:
                output.write(data)
print(f'Verified {len(manifest)} evidence files and the archive hash.')
