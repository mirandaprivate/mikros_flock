"""Freeze an isolated v3 experiment and verify all earlier Rust sources."""
import hashlib
import json
from pathlib import Path
import sys
import tarfile

here = Path(__file__).resolve().parent
outer = here.parent
destination = Path(sys.argv[1]).resolve()
assert not destination.exists(), destination

driver = (outer/'streaming/server_driver.py').read_text(encoding='utf-8')
driver = driver.replace("work=source/'streaming'", "work=source/'fused'")
begin = driver.index("case('smoke-n20-48t'")
end = driver.index("inventory['memory_final']", begin)
driver = driver[:begin] + '''case('smoke-n20-48t',20,48,'previous,optimized,flock',1)
case('main-n26-48t',26,48,'previous,optimized,flock',6)
for threads in (24,48):
    case(f'main-n30-{threads}t',30,threads,'previous,optimized,flock',6)
    for system in ('previous','optimized','flock'):
        case(f'cold-rss-n30-{threads}t-{system}',30,threads,system,1,0,1)
''' + driver[end:]
(here/'server_driver.py').write_text(driver, encoding='utf-8')

files = {}
snapshot = json.loads((outer/'snapshot/manifest.json').read_text(encoding='utf-8'))
for group in ('copied_files_sha256','generated_files_sha256'):
    for name, digest in snapshot[group].items():
        file = outer/'snapshot'/name
        assert hashlib.sha256(file.read_bytes()).hexdigest() == digest, name
        files['snapshot/'+name] = file
files['snapshot/manifest.json'] = outer/'snapshot/manifest.json'
frozen = json.loads((outer/'streaming/server_experiments/20260912T103800-permutation-v2/source_manifest.json').read_text(encoding='utf-8'))
for candidate in ('optimized','streaming','fused'):
    for folder in ('src','tests'):
        for file in (outer/candidate/folder).rglob('*.rs'):
            name = candidate+'/'+file.relative_to(outer/candidate).as_posix()
            if candidate != 'fused':
                assert hashlib.sha256(file.read_bytes()).hexdigest() == frozen['files_sha256'][name], name
            files[name] = file
    for name in ('Cargo.toml','Cargo.lock','build.rs','README.md'):
        files[candidate+'/'+name] = outer/candidate/name
for name in ('server_driver.py','THEORY.md','package_source.py'):
    files['fused/'+name] = here/name
files['.cargo/config.toml'] = outer/'snapshot/.cargo/config.toml'
destination.mkdir(parents=True)
manifest = {'scope':'Frozen original Flock/Mikros and v1/v2 Rust; isolated fused v3. No credentials or builds.',
            'files_sha256':{name:hashlib.sha256(file.read_bytes()).hexdigest() for name,file in sorted(files.items())}}
mf = destination/'source_manifest.json'
mf.write_text(json.dumps(manifest,indent=2)+'\n',encoding='utf-8')
archive = destination/'source.tar.gz'
with tarfile.open(archive,'x:gz') as tar:
    for name,file in sorted(files.items()):
        tar.add(file,arcname='source/'+name)
    tar.add(mf,arcname='source/source_manifest.json')
record = {'files':len(files),'archive_bytes':archive.stat().st_size,
          'source_bytes':sum(file.stat().st_size for file in files.values()),
          'sha256':hashlib.sha256(archive.read_bytes()).hexdigest()}
(destination/'package.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf-8')
print(json.dumps(record))
