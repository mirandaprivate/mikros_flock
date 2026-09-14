"""Cross-check formal suites and their archived measured source versions."""
from pathlib import Path
import hashlib
import json
import sys
import zipfile

def main():
    folders=[Path(p) for p in sys.argv[1:]]
    golden={}
    counts={}
    for folder in folders:
        metadata=json.loads((folder/'environment.json').read_text())
        with zipfile.ZipFile(folder/'sources.zip') as source:
            for name,expected in metadata['sources_sha256'].items():
                assert hashlib.sha256(source.read(name)).hexdigest()==expected,name
        validation=json.loads((folder/'validation.json').read_text())
        assert validation['valid']
        counts[str(folder)]=validation['proofs']
        execution=json.loads((folder/'execution.json').read_text())
        for event in execution:
            if not event['valid_run']:continue
            for line in (folder/event['sample_file']).read_text().splitlines():
                r=json.loads(line)
                if r['kind']!='sample':continue
                protocol='flock' if r['system']=='flock' else (
                    'batched' if r['mode'] in ('batched','bridge','full_optimized') else
                    'packed' if r['mode']=='packed' else 'legacy')
                key=(r['log_n'],r['value_bits'],r['run'],protocol)
                value=(r['input_hash'],r['statement_hash'],r['proof_hash'])
                if key in golden:assert golden[key]==value,'cross-suite transcript mismatch'
                golden[key]=value
    output={'valid':True,'source_archives_match':True,'cross_suite_proofs_match':True,
        'proof_counts':counts,'total_proofs':sum(counts.values())}
    here=Path(__file__).resolve().parent
    (here/'results/validation-all.json').write_text(json.dumps(output,indent=2)+'\n')
    print(json.dumps(output))

if __name__=='__main__':
    main()
