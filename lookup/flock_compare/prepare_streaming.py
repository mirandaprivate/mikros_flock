"""One-time isolated candidate setup. Never changes the previous candidate."""
from pathlib import Path
import shutil

base = Path(__file__).resolve().parent
old = base / 'optimized'
new = base / 'streaming'
assert not new.exists()
new.mkdir()
for name in ('src', 'tests'):
    shutil.copytree(old / name, new / name)
for name in ('Cargo.toml', 'Cargo.lock', 'build.rs', 'server_driver.py', 'verify_download.py'):
    shutil.copy2(old / name, new / name)
f = new / 'Cargo.toml'
s = f.read_text(encoding='utf-8')
s = s.replace('name = "mikros-permutation-optimized"', 'name = "mikros-permutation-streaming"', 1)
s = s.replace('[dependencies]', '[dependencies]\nprevious = { package = "mikros-permutation-optimized", path = "../optimized" }')
f.write_text(s, encoding='utf-8')
for f in (new/'src/bin/compare.rs', new/'tests/differential.rs'):
    f.write_text(f.read_text(encoding='utf-8').replace('mikros_permutation_optimized', 'mikros_permutation_streaming'), encoding='utf-8')
print('Created isolated streaming candidate')
