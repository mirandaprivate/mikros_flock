"""Freeze exact permutation kernels without depending on concurrent PCS work."""
from pathlib import Path
import hashlib
import json
import re
import subprocess
import time

here = Path(__file__).resolve().parent
repo = here.parents[1]
dest = here / "snapshot"
dest.mkdir(exist_ok=False)
paths = set(subprocess.check_output(["git", "ls-files", "--", "crates"], cwd=repo, text=True).splitlines())
paths.update(["Cargo.toml", "Cargo.lock", ".cargo/config.toml", ".clippy.toml", "LICENSE-MIT", "LICENSE-APACHE", "mikros/Cargo.toml", "mikros/build.rs"])
paths.update(p.relative_to(repo).as_posix() for p in (repo / "mikros/src").rglob("*.rs"))
paths.update(p.relative_to(repo).as_posix() for p in (repo / "mikros/benches").rglob("*.rs"))
paths.update("lookup/src/" + name for name in ("bits.rs", "claims.rs", "field.rs", "permutation.rs", "product_tree.rs", "sumcheck.rs", "product_tree_tests.rs"))
paths.add("lookup/README.md")
files = {}
for name in sorted(paths):
    source = repo / name
    assert source.is_file() and not source.is_symlink(), name
    data = source.read_bytes()
    target = dest / name
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(data)
    files[name] = hashlib.sha256(data).hexdigest()

# Only the module/dependency surface is reduced. All permutation, tree,
# sumcheck, bit encoding, field embedding and source-check code is byte-identical.
lib = (repo / "lookup/src/lib.rs").read_text()
lib = lib.replace("pub mod lasso;\n", "")
lib = re.sub(r'#\[cfg\(feature = "whir"\)\]\n(?:pub )?mod \w+;\n', "", lib)
(dest / "lookup/src/lib.rs").write_text(lib)
(dest / "lookup/Cargo.toml").write_text('''[package]
name = "mikros-permutation-snapshot"
version = "0.1.0"
edition = "2024"
license = "MIT OR Apache-2.0"
[workspace]
[dependencies]
flock-field = { path = "../crates/flock-field" }
flock-transcript = { path = "../crates/flock-transcript" }
flock-multilinear = { path = "../crates/flock-multilinear" }
blake3 = "1.5"
serde = { version = "1", features = ["derive"] }
rayon = "1.10"
''')
manifest = {"utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
            "base_revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=repo, text=True).strip(),
            "scope": "Exact existing permutation/tree/sumcheck/encoding/oracle sources and Flock crates. Lookup lib.rs/Cargo.toml expose permutation only; unrelated Lasso, Hadamard and in-progress WHIR code is not compiled.",
            "original_lib_sha256": hashlib.sha256((repo / "lookup/src/lib.rs").read_bytes()).hexdigest(),
            "copied_files_sha256": files,
            "generated_files_sha256": {name: hashlib.sha256((dest / name).read_bytes()).hexdigest() for name in ("lookup/src/lib.rs", "lookup/Cargo.toml")}}
(dest / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
print(f"Frozen {len(files)} source files in {dest}")
