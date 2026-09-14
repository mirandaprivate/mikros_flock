"""Run paired proofs sequentially, preserve samples and record provenance."""
from pathlib import Path
import argparse
import hashlib
import json
import os
import platform
import subprocess
import time

HERE = Path(__file__).resolve().parent

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def competitors():
    found = []
    for entry in Path('/proc').iterdir():
        if not entry.name.isdigit():
            continue
        try:
            name = (entry/'comm').read_text().strip()
            if name in ('cargo', 'rustc', 'permutation-flo', 'hadamard_vs_flo'):
                found.append({'pid': int(entry.name), 'name': name})
        except (OSError, ProcessLookupError):
            pass
    return found

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--name', required=True)
    parser.add_argument('--pilot', action='store_true')
    args = parser.parse_args()
    if Path(args.name).name != args.name:
        raise ValueError('name must be one directory component')
    out = HERE/'results'/args.name
    out.mkdir(parents=True, exist_ok=False)
    executable = HERE/'target/release/mikros-binius-permutation'
    rows = subprocess.check_output(['lscpu','-p=CPU,CORE,SOCKET'],text=True)
    allowed = os.sched_getaffinity(0)
    cores = {}
    for row in rows.splitlines():
        if row.startswith('#'):
            continue
        cpu,core,socket = map(int,row.split(','))
        if cpu in allowed:
            cores.setdefault((socket,core),cpu)
    cpus = sorted(cores.values())
    if not cpus:
        raise RuntimeError('no allowed CPU')
    # CPU 2 matches the earlier FLINT single-thread run when available.
    single = [2] if 2 in cpus else cpus[:1]
    configs = [(16,8,1,1)] if args.pilot else [
        (n,w,1,4) for n in (16,18,20) for w in (1,8,32)]
    if not args.pilot and len(cpus)>=6:
        configs.append((20,8,6,4))
    sources = {str(p.relative_to(HERE)):sha(p) for p in [
        HERE/'Cargo.toml',HERE/'Cargo.lock',HERE/'src/lib.rs',HERE/'src/main.rs',
        HERE/'src/tests.rs',HERE/'run_experiment.py',HERE/'snapshot/manifest.json']}
    metadata = {'utc':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),
        'platform':platform.platform(),'cpuinfo':Path('/proc/cpuinfo').read_text(),
        'lscpu':subprocess.check_output(['lscpu'],text=True),'physical_cpus':cpus,
        'rustc':subprocess.check_output(['rustc','+nightly-2026-03-21','-Vv'],text=True),
        'executable_sha256':sha(executable),'sources_sha256':sources,'configs':configs,
        'scope':'Same bit commitments, same known permutation, native product_gkr versus Mikros coefficient tree. Both authenticate all terminal claims with Flock ring-switching + Ligerito. No ZK or equal 128-bit soundness assertion.'}
    (out/'environment.json').write_text(json.dumps(metadata,indent=2)+'\n')
    events = []
    for n,w,threads,runs in configs:
        affinity = single if threads==1 else cpus[:threads]
        base = f'n{n}-w{w}-{threads}t'
        for attempt in range(1,6):
            deadline = time.monotonic()+300
            quiet_since = time.monotonic()
            while time.monotonic()-quiet_since<5:
                if competitors():
                    quiet_since = time.monotonic()
                if time.monotonic()>deadline:
                    raise RuntimeError('no quiet measurement window; all attempts retained')
                time.sleep(.5)
            stem = f'{base}-attempt{attempt}'
            command = ['taskset','-c',','.join(map(str,affinity)),str(executable),
                '--logs',str(n),'--widths',str(w),'--runs',str(runs),'--threads',str(threads),
                '--out',str(out/(stem+'.jsonl'))]
            started = time.time()
            interference = []
            with (out/(stem+'.log')).open('w') as log:
                child = subprocess.Popen(command,cwd=HERE,stdout=log,stderr=subprocess.STDOUT)
                while child.poll() is None:
                    interference.extend(competitors())
                    time.sleep(.5)
            event = {'case':base,'attempt':attempt,'sample_file':stem+'.jsonl',
                'command':command,'seconds':time.time()-started,
                'exit_code':child.returncode,'competing_builds':interference,
                'valid_run':child.returncode==0 and not interference}
            events.append(event)
            (out/'execution.json').write_text(json.dumps(events,indent=2)+'\n')
            print(json.dumps({k:v for k,v in event.items() if k not in ('command','competing_builds')},
                ensure_ascii=False),flush=True)
            if child.returncode:
                raise RuntimeError(f'{base} proof execution failed; all files retained')
            if not interference:
                break
        else:
            raise RuntimeError(f'{base}: five build-interfered attempts; all files retained')
    # Verify source provenance did not change while measuring.
    assert all(sha(HERE/name)==digest for name,digest in sources.items())
    assert sha(executable)==metadata['executable_sha256']
    (out/'completion.json').write_text(json.dumps({'completed':True,
        'cases':sum(e['valid_run'] for e in events),'attempts':len(events),'source_hashes_unchanged':True})+'\n')

if __name__=='__main__':
    main()
