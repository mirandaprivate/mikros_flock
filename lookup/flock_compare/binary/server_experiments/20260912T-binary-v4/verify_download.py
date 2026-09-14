"""Verify the independently recorded remote archive SHA before extracting evidence."""
import argparse
import hashlib
import json
from pathlib import Path
import tarfile

parser = argparse.ArgumentParser()
parser.add_argument('--sha256', required=True)
args = parser.parse_args()
root = Path(__file__).resolve().parent
archive = root / 'results.tar.gz'
actual = hashlib.sha256(archive.read_bytes()).hexdigest()
assert actual == args.sha256, 'download archive SHA256 mismatch'
assert not (root / 'results').exists(), 'do not overwrite prior results'
with tarfile.open(archive, 'r:gz') as tar:
    members = tar.getmembers()
    names = [member.name for member in members]
    assert len(names) == len(set(names)), 'duplicate members'
    assert all(member.isfile() and (root / member.name).resolve().is_relative_to(root)
               and (member.name.startswith('results/') or member.name == 'results_manifest.json')
               for member in members), 'invalid archive member'
    manifest = json.load(tar.extractfile('results_manifest.json'))
    assert set(names) == set(manifest['files_sha256']) | {'results_manifest.json'}
    for name, expected in manifest['files_sha256'].items():
        assert hashlib.sha256(tar.extractfile(name).read()).hexdigest() == expected, name
    tar.extractall(root)
for name, expected in manifest['files_sha256'].items():
    assert hashlib.sha256((root / name).read_bytes()).hexdigest() == expected, name
record = {'archive_sha256': actual, 'verified_files': len(manifest['files_sha256'])}
(root / 'download_validation.json').write_text(json.dumps(record, indent=2) + '\n')
print(json.dumps(record))
