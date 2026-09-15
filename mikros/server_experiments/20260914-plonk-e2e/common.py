"""Shared, dependency-free helpers for this experiment's source and evidence."""
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import time

CPU_SPEC = "0-47,48-55,72-79"
CPUS = set(range(56)) | set(range(72, 80))
NATIVE_FEATURES = {"avx512f", "avx512bw", "gfni", "vpclmulqdq"}
TOOLCHAIN = "+nightly-2026-03-21"
AUTHORIZED_ROOT = Path("/home/mscong1/mikros-experiments/20260912T-native64")
LEVELS = (14, 20, 26, 30)
SYSTEMS = ("mikros", "flock")
MODES = ("single", "multiple")
SCRIPT_DIR = "mikros/server_experiments/20260914-plonk-e2e"
CRITICAL_SOURCES = (
    "mikros/examples/e2e_plonk_compare.rs",
    "mikros/examples/e2e_plonk/fixture.rs",
    "mikros/examples/e2e_plonk/gates.rs",
    "mikros/examples/e2e_plonk/copy.rs",
    "mikros/examples/e2e_plonk/queries.rs",
    "crates/flock-core/src/product_gkr.rs",
    "crates/flock-core/src/pcs/commit.rs",
    "crates/flock-core/src/pcs/ligerito.rs",
    "crates/flock-core/src/pcs/merged_tensor.rs",
    *(f"crates/flock-core/configs/ligerito/m{m}_fast.toml" for m in range(36, 40)),
)


def require(ok, message):
    if not ok:
        raise RuntimeError(message)


def utc():
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())


def sha256(path):
    digest = hashlib.sha256()
    with Path(path).open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def atomic_text(path, text):
    path = Path(path)
    temporary = path.with_name(path.name + f".tmp-{os.getpid()}")
    try:
        with temporary.open("w", encoding="utf-8", newline="\n") as stream:
            stream.write(text)
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temporary, path)
    finally:
        if temporary.exists():
            temporary.unlink()


def save(path, value):
    atomic_text(path, json.dumps(value, indent=2, allow_nan=False) + "\n")


def source_path(root, name):
    relative = PurePosixPath(name)
    require(not relative.is_absolute() and ".." not in relative.parts,
            f"invalid manifest path {name!r}")
    require(not set(relative.parts) & {".git", "target", "__pycache__", ".env"},
            f"excluded path {name!r}")
    path = root.joinpath(*relative.parts)
    require(not path.is_symlink() and path.is_file() and path.resolve().is_relative_to(root),
            f"source is not a regular in-scope file: {name}")
    return path


def verify_sources(root):
    root = Path(root).resolve()
    manifest_path = root / "source_manifest.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    hashes = manifest["files_sha256"]
    require(isinstance(hashes, dict) and hashes, "empty source manifest")
    require(set(CRITICAL_SOURCES).issubset(hashes), "manifest misses required native PCS/E2E sources")
    for name, expected in hashes.items():
        require(sha256(source_path(root, name)) == expected, f"source hash mismatch: {name}")
    return manifest, sha256(manifest_path)


def case_id(mode, system, log_gates):
    return f"n{log_gates}-{mode}-{system}"


def cases(levels, modes=MODES):
    require(bool(modes) and len(set(modes)) == len(modes) and set(modes).issubset(MODES), "invalid case modes")
    return [
        {"id": case_id(mode, system, level), "mode": mode, "system": system,
         "log_gates": level, "gates_per_instance": 1 << level,
         "instances": 1 if mode == "single" else 128,
         "total_gates": (1 << level) * (1 if mode == "single" else 128)}
        for level in levels for mode in MODES if mode in modes
        for system in (SYSTEMS if mode == "single" else tuple(reversed(SYSTEMS)))
    ]
