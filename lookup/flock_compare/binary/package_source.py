"""Package only the permutation sources and explicitly listed experiment files."""
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
old = json.loads((outer / 'fused/server_experiments/20260912T120220-permutation-v3/source_manifest.json').read_text())
files = {}
updated_docs = {}
for name, digest in old['files_sha256'].items():
    path = outer / name
    if name == '.cargo/config.toml':
        path = outer / 'snapshot/.cargo/config.toml'
    # Preserve all old protocol and build inputs; old drivers are not needed.
    if name.startswith(('snapshot/', 'optimized/', 'streaming/', 'fused/')) and (
        path.suffix in ('.rs', '.toml') or path.name in ('Cargo.lock', 'manifest.json', 'README.md')
    ) or name == '.cargo/config.toml':
        actual = hashlib.sha256(path.read_bytes()).hexdigest()
        if actual != digest and path.name == 'README.md':
            # Old libraries include their README as Rust documentation. The
            # post-experiment report link is harmless but must be accounted for.
            updated_docs[name] = {'v3_sha256': digest, 'current_sha256': actual}
        else:
            assert actual == digest, name
        files[name] = path
for folder in ('src', 'tests'):
    for path in (here / folder).rglob('*.rs'):
        files['binary/' + path.relative_to(here).as_posix()] = path
for name in ('Cargo.toml', 'Cargo.lock', 'build.rs', 'README.md', 'PLAN.md',
             'server_driver.py', 'summarize_experiment.py', 'package_source.py'):
    files['binary/' + name] = here / name
destination.mkdir(parents=True)
manifest = {
    'scope': 'Permutation-only source and build inputs, frozen Flock and v1/v2/v3, binary candidate. No credentials, git metadata or build outputs.',
    'frozen_v3_manifest_sha256': hashlib.sha256((outer / 'fused/server_experiments/20260912T120220-permutation-v3/source_manifest.json').read_bytes()).hexdigest(),
    'updated_documentation': updated_docs,
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
