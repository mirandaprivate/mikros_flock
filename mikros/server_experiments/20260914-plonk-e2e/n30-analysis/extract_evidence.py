"""Extract one checksum-verified, regular-file evidence archive to a fresh folder."""
import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import tarfile


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("archive", type=Path)
    p.add_argument("sha256")
    p.add_argument("destination", type=Path)
    a = p.parse_args()
    digest = hashlib.sha256(a.archive.read_bytes()).hexdigest()
    assert digest == a.sha256, "download/archive checksum mismatch"
    assert not a.destination.exists(), "refuse to overwrite existing evidence"
    with tarfile.open(a.archive, "r:gz") as stream:
        members = stream.getmembers()
        names = set()
        for member in members:
            name = PurePosixPath(member.name)
            assert member.isfile() and not name.is_absolute() and ".." not in name.parts
            assert "\\" not in member.name and ":" not in member.name and member.name not in names
            assert sum(m.size for m in members) < 100 * (1 << 20), "unexpectedly large evidence archive"
            names.add(member.name)
        a.destination.mkdir(parents=True)
        for member in members:
            path = a.destination.joinpath(*PurePosixPath(member.name).parts)
            assert path.resolve().is_relative_to(a.destination.resolve())
            path.parent.mkdir(parents=True, exist_ok=True)
            with path.open("xb") as f:
                f.write(stream.extractfile(member).read())
    receipt = {"archive_sha256": digest, "archive_bytes": a.archive.stat().st_size,
               "server_download_sha256_matches": True, "regular_files": len(names)}
    a.destination.with_suffix(".receipt.json").write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps(receipt))


if __name__ == "__main__":
    main()
