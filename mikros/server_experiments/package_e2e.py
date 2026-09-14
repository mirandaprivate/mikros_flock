#!/usr/bin/env python3
"""Allowlisted current E2E sources; no credentials, history, or build outputs."""
import argparse
import hashlib
import io
import json
from pathlib import Path
import subprocess
import tarfile

root = Path(__file__).resolve().parents[2]
p = argparse.ArgumentParser()
p.add_argument("output", type=Path)
args = p.parse_args()
paths = set(subprocess.check_output(["git", "ls-files", "--", "crates"], cwd=root, text=True).splitlines())
paths.update(["Cargo.toml", "Cargo.lock", ".cargo/config.toml", ".clippy.toml", "README.md",
              "LICENSE-MIT", "LICENSE-APACHE", "mikros/Cargo.toml", "mikros/build.rs",
              "mikros/permutation/Cargo.toml", "mikros/permutation/README.md", "mikros/E2E.md",
              "mikros/permutation/SOUNDNESS.md", "mikros/permutation/PACKED.md",
              "mikros/server_experiments/run_e2e_64.py",
              "mikros/server_experiments/package_e2e.py",
              "mikros/server_experiments/run_packed_permutation_64.py",
              "mikros/server_experiments/run_packed_aggregate_probe.py",
              "crates/flock-core/src/pcs/functional.rs",
              "crates/flock-core/src/pcs/merged_tensor.rs",
              "crates/flock-core/src/zerocheck/multilinear/dense_skip7.rs",
              "crates/flock-core/src/zerocheck/univariate_skip_optimized/skip128.rs"])
for directory in ["mikros/src", "mikros/tests", "mikros/benches", "mikros/examples",
                  "mikros/permutation/src", "mikros/permutation/tests",
                  "crates/flock-core/src/zerocheck/univariate_skip_optimized/skip128"]:
    paths.update(x.relative_to(root).as_posix() for x in (root / directory).rglob("*.rs"))
manifest = {"base_revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip(),
            "scope": "Root Flock crates and current Mikros E2E sources, tests, examples, build files and experiment driver only",
            "files_sha256": {}}
args.output.mkdir(parents=True, exist_ok=True)
archive = args.output / "source.tar.gz"
if archive.exists():
    raise SystemExit("snapshot already exists; choose a new output directory")
with tarfile.open(archive, "w:gz") as tar:
    for name in sorted(paths):
        path = root / name
        assert path.is_file() and not path.is_symlink(), name
        assert ".git" not in path.parts and "target" not in path.parts, name
        data = path.read_bytes()
        manifest["files_sha256"][name] = hashlib.sha256(data).hexdigest()
        info = tarfile.TarInfo(name)
        info.size, info.mode = len(data), 0o644
        tar.addfile(info, io.BytesIO(data))
    data = (json.dumps(manifest, indent=2) + "\n").encode()
    info = tarfile.TarInfo("source_manifest.json")
    info.size, info.mode = len(data), 0o644
    tar.addfile(info, io.BytesIO(data))
(args.output / "source_manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
digest = hashlib.sha256(archive.read_bytes()).hexdigest()
(args.output / "source.tar.gz.sha256").write_text(f"{digest}  source.tar.gz\n", newline="\n")
print(json.dumps({"files": len(paths), "archive_bytes": archive.stat().st_size, "sha256": digest}))
