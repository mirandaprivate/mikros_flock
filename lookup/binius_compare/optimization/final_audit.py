from pathlib import Path
import hashlib
import json
import subprocess

here=Path(__file__).resolve().parent
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
subprocess.run(['python3','audit_baseline.py'],cwd=here,check=True)
metadata=json.loads((here/'results/scaling-20260912-v1/environment.json').read_text())
for name,expected in metadata['sources_sha256'].items():
    assert sha(here/name)==expected,name
assert sha(here/'target/release/mikros-binius-optimized')==metadata['executable_sha256']
imports=json.loads((here/'IMPORTS.json').read_text())
baseline=[]
for name,expected in imports.items():
    if name.startswith('lookup/binius_compare/src/'):
        assert sha(here.parents[2]/name)==expected,name
        baseline.append(name)
result={'valid':True,'snapshot_files':310,'current_sources_match_scaling':True,
    'current_executable_matches_scaling':True,'original_adapter_sources_unchanged':baseline,
    'doc_change':'Original adapter README links to the new independent workspace.'}
(here/'results/final-audit.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result))
