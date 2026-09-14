"""Import reviewed local kernels into an isolated PCS optimization workspace."""
from pathlib import Path
import hashlib
import json

here = Path(__file__).resolve().parent
source = here.parents[1] / 'flock_compare' / 'optimized'
files = ['bits.rs', 'claims.rs', 'field.rs', 'fast_tree.rs', 'fast_sumcheck.rs',
         'kernels.rs', 'lib.rs', 'permutation.rs', 'profile.rs', 'sumcheck.rs',
         'product_tree_tests.rs', 'reference/product_tree.rs', 'reference/sumcheck.rs']
manifest = {}
for name in files:
    origin = source / 'src' / name
    dest = here / 'kernel' / 'src' / name
    dest.parent.mkdir(parents=True, exist_ok=True)
    data = origin.read_bytes()
    with dest.open('xb') as stream:
        stream.write(data)
    manifest[str(origin.relative_to(here.parents[2]))] = hashlib.sha256(data).hexdigest()
for name in ['lib.rs', 'main.rs', 'tests.rs']:
    origin = here.parent / 'src' / name
    dest = here / 'src' / name
    dest.parent.mkdir(parents=True, exist_ok=True)
    with dest.open('xb') as stream:
        stream.write(origin.read_bytes())
    manifest[str(origin.relative_to(here.parents[2]))] = hashlib.sha256(origin.read_bytes()).hexdigest()
(here / 'IMPORTS.json').write_text(json.dumps(manifest, indent=2) + '\n')
