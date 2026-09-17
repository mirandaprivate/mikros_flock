#!/usr/bin/env python3
"""Fetch and verify the unmodified, pinned original Binius source dependency."""
import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import tarfile
import urllib.request

REVISION = "47675e19c86c0fb676f75437073a77af8e337938"
SHA256 = "8bf9add0596fa5a85aa767d40e1a4904aa4c1de7ce93f8b93294fda5e13b427f"
URL = f"https://codeload.github.com/IrreducibleOSS/binius/tar.gz/{REVISION}"
HERE = Path(__file__).resolve().parent


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--archive", type=Path)
    args = parser.parse_args()
    archive = args.archive or HERE / "upstream.tar.gz"
    if not archive.exists():
        urllib.request.urlretrieve(URL, archive)
    if hashlib.sha256(archive.read_bytes()).hexdigest() != SHA256:
        raise ValueError("upstream archive hash mismatch")
    destination = HERE / "upstream"
    inventory = {}
    with tarfile.open(archive, "r:gz") as tar:
        for member in tar.getmembers():
            parts = PurePosixPath(member.name).parts
            if not parts or parts[0] != f"binius-{REVISION}" or ".." in parts:
                raise ValueError(f"unsafe archive member: {member.name}")
            if member.isdir():
                continue
            if not member.isfile():
                raise ValueError(f"non-regular upstream member: {member.name}")
            relative = Path(*parts[1:])
            target = destination / relative
            if not target.resolve().is_relative_to(destination.resolve()):
                raise ValueError("upstream path escaped destination")
            data = tar.extractfile(member).read()
            if target.exists() and target.read_bytes() != data:
                raise ValueError(f"local upstream file differs: {relative}")
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
            inventory[relative.as_posix()] = hashlib.sha256(data).hexdigest()
    receipt = {"repository": "https://github.com/IrreducibleOSS/binius", "revision": REVISION,
               "archive_sha256": SHA256, "files_sha256": inventory}
    (HERE / "upstream-source.json").write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8")
    print(f"Verified original Binius {REVISION}: {len(inventory)} source files")


if __name__ == "__main__":
    main()
