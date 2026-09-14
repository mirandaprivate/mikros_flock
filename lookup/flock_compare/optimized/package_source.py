"""Whitelist package: frozen baseline + this candidate, no credentials or builds."""
import hashlib
import json
from pathlib import Path
import tarfile
import sys

here=Path(__file__).resolve().parent
outer=here.parent
destination=Path(sys.argv[1]).resolve()
destination.mkdir(parents=True,exist_ok=False)
files={}
original=json.loads((outer/'snapshot/manifest.json').read_text())
for group in ('copied_files_sha256','generated_files_sha256'):
    for name,digest in original[group].items():
        path=outer/'snapshot'/name
        assert hashlib.sha256(path.read_bytes()).hexdigest()==digest,name
        files['snapshot/'+name]=path
files['snapshot/manifest.json']=outer/'snapshot/manifest.json'
for folder in ('src','tests'):
    for path in (here/folder).rglob('*.rs'):
        files['optimized/'+path.relative_to(here).as_posix()]=path
for name in ('Cargo.toml','Cargo.lock','build.rs','README.md','server_driver.py'):
    files['optimized/'+name]=here/name
files['.cargo/config.toml']=outer/'snapshot/.cargo/config.toml'
manifest={'scope':'Frozen Mikros/Flock permutation reference and same-message optimized candidate only',
          'files_sha256':{name:hashlib.sha256(path.read_bytes()).hexdigest() for name,path in sorted(files.items())}}
manifest_path=destination/'source_manifest.json'
manifest_path.write_text(json.dumps(manifest,indent=2)+'\n')
archive=destination/'source.tar.gz'
with tarfile.open(archive,'w:gz') as tar:
    for name,path in sorted(files.items()):tar.add(path,arcname='source/'+name)
    tar.add(manifest_path,arcname='source/source_manifest.json')
record={'files':len(files),'archive_bytes':archive.stat().st_size,'source_bytes':sum(p.stat().st_size for p in files.values()),'sha256':hashlib.sha256(archive.read_bytes()).hexdigest()}
(destination/'package.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps(record))
