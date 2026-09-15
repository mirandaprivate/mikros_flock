#!/usr/bin/env python3
"""Freeze the existing E2E allowlist plus new native PCS shapes and this harness.

No transfer is performed. Run only after root declares the source ready.
"""
import argparse
import hashlib
import io
import json
from pathlib import Path
import subprocess
import tarfile

from common import CRITICAL_SOURCES, SCRIPT_DIR, require, save, sha256, source_path, atomic_text


def source_names(root):
    names = set(subprocess.check_output(
        ["git", "ls-files", "--", "crates"], cwd=root, text=True).splitlines())
    names.update([
        "Cargo.toml", "Cargo.lock", ".cargo/config.toml", ".clippy.toml", "README.md",
        "LICENSE-MIT", "LICENSE-APACHE", "mikros/Cargo.toml", "mikros/build.rs",
        "mikros/permutation/Cargo.toml", "mikros/permutation/README.md",
        "mikros/permutation/SOUNDNESS.md", "mikros/permutation/PACKED.md", "mikros/E2E.md",
        "mikros/server_experiments/package_e2e.py",
    ])
    for directory in ("mikros/src", "mikros/tests", "mikros/benches", "mikros/examples",
                      "mikros/permutation/src", "mikros/permutation/tests"):
        names.update(p.relative_to(root).as_posix() for p in (root / directory).rglob("*.rs"))
    # git ls-files alone misses newly introduced native source/config files.
    for crate in (root / "crates").iterdir():
        if (crate / "src").is_dir():
            names.update(p.relative_to(root).as_posix() for p in (crate / "src").rglob("*.rs"))
    names.update(p.relative_to(root).as_posix()
                 for p in (root / "crates/flock-core/configs/ligerito").glob("*.toml"))
    for pattern in ("*.py", "*.sh", "README.md"):
        names.update(p.relative_to(root).as_posix() for p in (root / SCRIPT_DIR).glob(pattern))
    require(set(CRITICAL_SOURCES).issubset(names), "missing critical E2E/native PCS files")
    return sorted(names)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("output", type=Path)
    parser.add_argument("--list-only", action="store_true", help="inventory only; do not freeze files")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[3]
    names = source_names(root)
    if args.list_only:
        for name in names:
            source_path(root, name)
        print(json.dumps({"files": len(names), "paths": names}, indent=2))
        return
    output = args.output.resolve()
    require(output.is_relative_to(root / SCRIPT_DIR), "package output must stay in this experiment directory")
    output.mkdir(parents=True, exist_ok=True)
    require(not any(output.glob("source*")), "source snapshot exists; choose a fresh output directory")
    manifest = {
        "schema_version": 1,
        "base_revision": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip(),
        "scope": "Root Flock crates/configs and Mikros source/tests/examples/build files plus PLONK E2E harness; no credentials/history/build outputs",
        "critical_sources": list(CRITICAL_SOURCES), "files_sha256": {}, "files_bytes": {},
    }
    temporary = output / "source.tar.gz.partial"
    require(not temporary.exists(), "unfinished package exists; inspect it before retrying")
    with temporary.open("xb") as stream, tarfile.open(fileobj=stream, mode="w:gz") as tar:
        for name in names:
            data = source_path(root, name).read_bytes()
            manifest["files_sha256"][name] = hashlib.sha256(data).hexdigest()
            manifest["files_bytes"][name] = len(data)
            info = tarfile.TarInfo(name)
            info.size, info.mode, info.mtime = len(data), 0o644, 0
            tar.addfile(info, io.BytesIO(data))
        content = (json.dumps(manifest, indent=2) + "\n").encode()
        info = tarfile.TarInfo("source_manifest.json")
        info.size, info.mode, info.mtime = len(content), 0o644, 0
        tar.addfile(info, io.BytesIO(content))
    # Catch concurrent edits before publishing an immutable source archive.
    for name, digest in manifest["files_sha256"].items():
        require(sha256(source_path(root, name)) == digest, f"source changed during freeze: {name}")
    archive = output / "source.tar.gz"
    temporary.replace(archive)
    save(output / "source_manifest.json", manifest)
    digest = sha256(archive)
    atomic_text(output / "source.tar.gz.sha256", f"{digest}  source.tar.gz\n")
    print(json.dumps({"files": len(names), "source_bytes": sum(manifest["files_bytes"].values()),
                      "archive_bytes": archive.stat().st_size, "archive_sha256": digest}))


if __name__ == "__main__":
    main()
