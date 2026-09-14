"""Sequential, pinned, paired ablations. Never overwrite a prior experiment."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import platform
import subprocess
import time
from archive_sources import archive

HERE = Path(__file__).resolve().parent
ALL_MODES = 'reference,full_extension,direct,lookup,fused,batched,bridge,full_optimized,packed'
FINAL_MODES = 'reference,bridge,full_optimized,packed'

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def competitors(exclude=()):
    names = {'cargo','rustc','permutation-flo','hadamard_vs_flo','mikros-binius-o',
             'mikros-binius-p','flint_extension','compare','gruen-compare'}
    found = []
    for p in Path('/proc').iterdir():
        if not p.name.isdigit() or int(p.name) in exclude:
            continue
        try:
            name = (p/'comm').read_text().strip()
            if name in names:
                found.append({'pid':int(p.name),'name':name})
        except OSError:
            pass
    return found

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--name',required=True)
    parser.add_argument('--suite',choices=['pilot','ablation','scaling'],required=True)
    args = parser.parse_args()
    if Path(args.name).name != args.name:
        raise ValueError('name must be a single directory component')
    out = HERE/'results'/args.name
    out.mkdir(parents=True,exist_ok=False)
    subprocess.run(['python3','audit_baseline.py'],cwd=HERE,check=True)
    exe = HERE/'target/release/mikros-binius-optimized'
    physical = {}
    topology = subprocess.check_output(['lscpu','-p=CPU,CORE,SOCKET'],text=True)
    for line in topology.splitlines():
        if line.startswith('#'):
            continue
        cpu,core,socket = map(int,line.split(','))
        if cpu in os.sched_getaffinity(0):
            physical.setdefault((socket,core),cpu)
    cores = sorted(physical.values())
    one = [2] if 2 in cores else cores[:1]
    if args.suite=='pilot':
        cases = [(16,8,1,1,ALL_MODES)]
    elif args.suite=='ablation':
        cases = [(20,8,t,11,ALL_MODES) for t in (1,6) if t<=len(cores)]
    else:
        cases = [(n,w,1,6,FINAL_MODES) for n in (16,18,20) for w in (1,8,32)]
        if len(cores)>=6:
            cases.append((20,8,6,6,FINAL_MODES))
    files = [HERE/'Cargo.toml',HERE/'Cargo.lock',HERE/'kernel/Cargo.toml',
             HERE/'run_experiment.py',HERE/'archive_sources.py',HERE/'summarize.py',HERE/'IMPORTS.json']
    for base in ('src','kernel/src','kernel/tests'):
        files.extend(sorted((HERE/base).rglob('*.rs')))
    source_hashes = {str(p.relative_to(HERE)):sha(p) for p in files}
    frozen_sources = json.loads((HERE.parent/'snapshot/manifest.json').read_text())
    metadata = {'utc':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),
        'suite':args.suite,'cases':cases,'physical_cpus':cores,'platform':platform.platform(),
        'lscpu':subprocess.check_output(['lscpu'],text=True),
        'cpuinfo':Path('/proc/cpuinfo').read_text(),
        'rustc':subprocess.check_output(['rustc','+nightly-2026-03-21','-Vv'],text=True),
        'root_cargo_config':(HERE.parents[2]/'.cargo/config.toml').read_text(),
        'sources_sha256':source_hashes,'executable_sha256':sha(exe),
        'baseline_manifest':frozen_sources,
        'scope':'Identical public permutation, bit commitments and Fast100/diagnostic PCS policy. Different PIOP protocols have different algebraic soundness bounds. Not a complete Flock prover or an equal 128-bit security claim.'}
    (out/'environment.json').write_text(json.dumps(metadata,indent=2)+'\n')
    archive(out)
    events=[]
    for n,w,threads,runs,modes in cases:
        affinity=one if threads==1 else cores[:threads]
        case=f'n{n}-w{w}-{threads}t'
        for attempt in range(1,6):
            deadline=time.monotonic()+300
            quiet=time.monotonic()
            while time.monotonic()-quiet<5:
                found=competitors()
                if found:
                    quiet=time.monotonic()
                if time.monotonic()>deadline:
                    raise RuntimeError('No quiet benchmark window; all records retained')
                time.sleep(.5)
            stem=f'{case}-attempt{attempt}'
            command=['taskset','-c',','.join(map(str,affinity)),str(exe),
                '--logs',str(n),'--widths',str(w),'--runs',str(runs),'--threads',str(threads),
                '--modes',modes,'--out',str(out/(stem+'.jsonl'))]
            started=time.time()
            interference=[]
            with (out/(stem+'.log')).open('x') as log:
                child=subprocess.Popen(command,cwd=HERE,stdout=log,stderr=subprocess.STDOUT)
                while child.poll() is None:
                    interference.extend(competitors((child.pid,)))
                    time.sleep(.5)
            event={'case':case,'attempt':attempt,'sample_file':stem+'.jsonl','command':command,
                'exit_code':child.returncode,'seconds':time.time()-started,
                'interference':interference,'valid_run':child.returncode==0 and not interference}
            events.append(event)
            (out/'execution.json').write_text(json.dumps(events,indent=2)+'\n')
            print(json.dumps({k:v for k,v in event.items() if k not in ('command','interference')}),flush=True)
            if child.returncode:
                raise RuntimeError('Proof/verification failure, batch retained')
            if not interference:
                break
        else:
            raise RuntimeError('Five interfered batches, all retained')
    assert all(sha(HERE/name)==digest for name,digest in source_hashes.items())
    assert sha(exe)==metadata['executable_sha256']
    subprocess.run(['python3','audit_baseline.py'],cwd=HERE,check=True)
    (out/'completion.json').write_text(json.dumps({'completed':True,'cases':len(cases),
        'sources_unchanged':True,'executable_unchanged':True,'baseline_unchanged':True})+'\n')

if __name__=='__main__':
    main()
