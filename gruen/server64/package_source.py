#!/usr/bin/env python3
"""Create a new whitelisted source archive; never include credentials/targets."""
import argparse
import hashlib
import json
from pathlib import Path
import tarfile
from run_server import ROOT, source_files, sha

p = argparse.ArgumentParser()
p.add_argument("--out", type=Path, required=True)
args = p.parse_args()
args.out.mkdir(parents=True, exist_ok=False)
files = source_files() + [ROOT / "README.md", ROOT / ".gitignore"]
manifest = {str(f.relative_to(ROOT)): sha(f) for f in files}
(args.out / "source_manifest.json").write_text(json.dumps(manifest, indent=2)+"\n")
archive = args.out / "source.tar.gz"
with tarfile.open(archive, "w:gz") as tar:
    for f in files:
        tar.add(f, arcname="source/" + str(f.relative_to(ROOT)))
    tar.add(args.out / "source_manifest.json", arcname="source/source_manifest.json")
(args.out / "source.tar.gz.sha256").write_text(hashlib.sha256(archive.read_bytes()).hexdigest()+"  source.tar.gz\n")
print(json.dumps({"archive": str(archive), "files": len(files), "bytes": archive.stat().st_size}))
