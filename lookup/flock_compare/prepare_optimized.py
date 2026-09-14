"""Create an isolated, API-compatible optimization candidate from the frozen PIOP."""
from pathlib import Path
import shutil

here = Path(__file__).resolve().parent
dest = here / "optimized"
dest.mkdir(exist_ok=False)
(dest / "src/reference").mkdir(parents=True)
source = here / "snapshot/lookup"
for name in ("lib.rs", "bits.rs", "claims.rs", "field.rs", "permutation.rs", "product_tree.rs", "sumcheck.rs", "product_tree_tests.rs"):
    shutil.copyfile(source / "src" / name, dest / "src" / name)
for name in ("product_tree.rs", "sumcheck.rs"):
    shutil.copyfile(source / "src" / name, dest / "src/reference" / name)
(dest / "README.md").write_text("# Optimized Mikros permutation PIOP\n\nIsolated candidate; original frozen sources and results remain unchanged.\n")
(dest / "Cargo.toml").write_text('''[package]
name = "mikros-permutation-optimized"
version = "0.1.0"
edition = "2024"
license = "MIT OR Apache-2.0"

[workspace]
exclude = ["../snapshot", "../snapshot/lookup", "../snapshot/crates/*"]

[dependencies]
flock-field = { path = "../snapshot/crates/flock-field" }
flock-transcript = { path = "../snapshot/crates/flock-transcript" }
flock-hash = { path = "../snapshot/crates/flock-hash" }
flock-multilinear = { path = "../snapshot/crates/flock-multilinear" }
flock-parallel = { path = "../snapshot/crates/flock-parallel" }
flock-core = { path = "../snapshot/crates/flock-core" }
baseline = { package = "mikros-permutation-snapshot", path = "../snapshot/lookup" }
blake3 = { version = "1.5", features = ["rayon"] }
rayon = "1.10"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
bincode = "1.3"

[profile.release]
opt-level = 3

[lints.clippy]
needless_range_loop = "allow"
too_many_arguments = "allow"
type_complexity = "allow"
''')
print(dest)
