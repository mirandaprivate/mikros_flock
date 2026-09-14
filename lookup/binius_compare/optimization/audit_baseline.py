from pathlib import Path
import hashlib
import json

here = Path(__file__).resolve().parent
snapshot = here.parent / 'snapshot'
manifest = json.loads((snapshot / 'manifest.json').read_text())
bad = []
expected_files = dict(manifest['copied_files_sha256'])
expected_files.update(manifest.get('generated_files_sha256', {}))
for name, expected in expected_files.items():
    actual = hashlib.sha256((snapshot / name).read_bytes()).hexdigest()
    if actual != expected:
        bad.append(name)
print(json.dumps({'checked':len(expected_files), 'mismatches':bad}))
if bad:
    raise SystemExit(1)
