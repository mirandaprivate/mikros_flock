"""Package only frozen baseline, previous candidate, and streaming candidate."""
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
original=json.loads((outer/'snapshot/manifest.json').read_text(encoding='utf-8'))
for group in ('copied_files_sha256','generated_files_sha256'):
    for name,digest in original[group].items():
        p=outer/'snapshot'/name
        assert hashlib.sha256(p.read_bytes()).hexdigest()==digest,name
        files['snapshot/'+name]=p
files['snapshot/manifest.json']=outer/'snapshot/manifest.json'
previous=json.loads((outer/'optimized/server_experiments/20260912T091817-permutation-v1/source_manifest.json').read_text(encoding='utf-8'))
for candidate in ('optimized','streaming'):
    for folder in ('src','tests'):
        for p in (outer/candidate/folder).rglob('*.rs'):
            name=candidate+'/'+p.relative_to(outer/candidate).as_posix()
            if candidate=='optimized': assert hashlib.sha256(p.read_bytes()).hexdigest()==previous['files_sha256'][name],name
            files[name]=p
    for name in ('Cargo.toml','Cargo.lock','build.rs','README.md'):
        files[candidate+'/'+name]=outer/candidate/name
files['streaming/server_driver.py']=here/'server_driver.py'
files['.cargo/config.toml']=outer/'snapshot/.cargo/config.toml'
manifest={'scope':'Unchanged previous candidate, frozen Flock, streaming permutation candidate; no credentials or builds',
          'files_sha256':{name:hashlib.sha256(p.read_bytes()).hexdigest() for name,p in sorted(files.items())}}
mf=destination/'source_manifest.json'
mf.write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf-8')
archive=destination/'source.tar.gz'
with tarfile.open(archive,'x:gz') as tar:
    for name,p in sorted(files.items()): tar.add(p,arcname='source/'+name)
    tar.add(mf,arcname='source/source_manifest.json')
record={'files':len(files),'archive_bytes':archive.stat().st_size,'source_bytes':sum(p.stat().st_size for p in files.values()),'sha256':hashlib.sha256(archive.read_bytes()).hexdigest()}
(destination/'package.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf-8')
print(json.dumps(record))
