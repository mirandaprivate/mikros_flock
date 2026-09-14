"""Freeze only authorized permutation source/build inputs and v5 experiment tools."""
import hashlib
import json
from pathlib import Path
import sys
import tarfile

here = Path(__file__).resolve().parent
outer = here.parent
destination = Path(sys.argv[1]).resolve()
assert not destination.exists(), destination
assert destination.is_relative_to(here), destination
old_manifest = outer / 'binary/server_experiments/20260912T-binary-v4/source_manifest.json'
old = json.loads(old_manifest.read_text())
files = {}
for name, digest in old['files_sha256'].items():
    path = outer / name
    if name == '.cargo/config.toml':
        path = outer / 'snapshot/.cargo/config.toml'
    if (name.startswith(('snapshot/', 'optimized/', 'streaming/', 'fused/', 'binary/'))
        and (path.suffix in ('.rs', '.toml') or path.name in ('Cargo.lock', 'manifest.json', 'README.md'))
    ) or name == '.cargo/config.toml':
        assert hashlib.sha256(path.read_bytes()).hexdigest() == digest, name
        files[name] = path
for folder in ('src', 'tests'):
    for path in (here / folder).rglob('*.rs'):
        files['binary_v5/' + path.relative_to(here).as_posix()] = path
for name in ('Cargo.toml', 'Cargo.lock', 'build.rs', 'README.md', 'PLAN.md',
             'server_driver.py', 'summarize_experiment.py', 'package_source.py'):
    files['binary_v5/' + name] = here / name
destination.mkdir(parents=True)
manifest = {
    'scope': 'Same authorized permutation source/build scope and v5 drivers; frozen Flock and v1-v4 retained. No credentials, git metadata, unrelated projects or build outputs.',
    'frozen_v4_manifest_sha256': hashlib.sha256(old_manifest.read_bytes()).hexdigest(),
    'files_sha256': {name: hashlib.sha256(path.read_bytes()).hexdigest() for name, path in sorted(files.items())},
}
mf = destination / 'source_manifest.json'
mf.write_text(json.dumps(manifest, indent=2) + '\n')
archive = destination / 'source.tar.gz'
with tarfile.open(archive, 'x:gz') as tar:
    for name, path in sorted(files.items()):
        tar.add(path, arcname='source/' + name, recursive=False)
    tar.add(mf, arcname='source/source_manifest.json', recursive=False)
record = {'files': len(files), 'archive_bytes': archive.stat().st_size,
          'source_bytes': sum(path.stat().st_size for path in files.values()),
          'sha256': hashlib.sha256(archive.read_bytes()).hexdigest()}
(destination / 'package.json').write_text(json.dumps(record, indent=2) + '\n')
print(json.dumps(record))
