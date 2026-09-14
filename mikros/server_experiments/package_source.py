#!/usr/bin/env python3
"""Package an allowlisted source snapshot, never the workspace or credentials."""
import argparse
import hashlib
import io
import json
from pathlib import Path
import subprocess
import tarfile


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[2]
    paths = set(subprocess.check_output(["git", "ls-files", "--", "crates"], cwd=root, text=True).splitlines())
    paths.update(["Cargo.toml", "Cargo.lock", ".cargo/config.toml", ".clippy.toml", "README.md",
                  "LICENSE-MIT", "LICENSE-APACHE", "mikros/Cargo.toml", "mikros/build.rs",
                  "mikros/server_experiments/run_server_n30.py"])
    paths.add("mikros/server_experiments/run_server_64.py")
    paths.add("mikros/server_experiments/run_univariate_64.py")
    paths.add("mikros/server_experiments/run_hardware_64.py")
    paths.add("mikros/server_experiments/run_register_64.py")
    paths.add("mikros/server_experiments/run_aggregate_64.py")
    paths.add("mikros/server_experiments/package_source.py")
    # The current mikros dev-dependencies and workspace include permutation;
    # cargo test also builds the examples and their shared PCS adapters.
    paths.add("mikros/permutation/Cargo.toml")
    paths.add("mikros/permutation/README.md")
    # Explicit allowlist for this task's new Flock-128 kernels; do not include
    # unrelated untracked crates, projects, build directories, or results.
    paths.update([
        "crates/flock-core/src/zerocheck/univariate_skip_optimized/skip128.rs",
        "crates/flock-core/src/zerocheck/univariate_skip_optimized/skip128/x86_64.rs",
        "crates/flock-core/src/zerocheck/multilinear/dense_skip7.rs",
        "crates/flock-core/src/pcs/functional.rs",
        "crates/flock-core/src/pcs/merged_tensor.rs",
    ])
    for directory in ("src", "tests", "benches", "examples", "permutation/src", "permutation/tests"):
        paths.update(path.relative_to(root).as_posix() for path in (root / "mikros" / directory).rglob("*.rs"))
    manifest = {"base_revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip(),
                "snapshot_scope": "Tracked crates plus explicit new Flock-128 kernels, root build files, current Mikros Rust sources/tests/bench and credential-free experiment driver; excludes .git, credentials, unrelated projects and all build/results directories",
                "files_sha256": {}}
    args.output.mkdir(parents=True, exist_ok=True)
    archive = args.output / "source.tar.gz"
    if archive.exists():
        raise SystemExit("snapshot already exists; choose a new output directory")
    with tarfile.open(archive, "w:gz") as tar:
        for name in sorted(paths):
            path = root / name
            assert path.is_file() and not path.is_symlink(), name
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
    (args.output / "source.tar.gz.sha256").write_text(f"{digest}  source.tar.gz\n")
    print(json.dumps({"files": len(paths), "archive_bytes": archive.stat().st_size, "sha256": digest}))


if __name__ == "__main__":
    main()
