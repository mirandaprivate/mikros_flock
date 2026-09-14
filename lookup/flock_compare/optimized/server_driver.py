"""Native, bounded, serial server experiment. Never stops another user's work."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import time

parser=argparse.ArgumentParser()
parser.add_argument('--root',type=Path,required=True)
args=parser.parse_args()
root=args.root.resolve()
assert root.parent == Path('/home/mscong1/mikros-permutation-experiments')
source=root/'source'
work=source/'optimized'
results=root/'results'
results.mkdir(exist_ok=False)
commands=[]
def save(name,data):
    (results/name).write_text(json.dumps(data,indent=2)+'\n')
def status(stage,**kw):
    rec={'stage':stage,'utc':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),**kw}
    save('status.json',rec);print(json.dumps(rec),flush=True)

manifest=json.loads((source/'source_manifest.json').read_text())
for name,digest in manifest['files_sha256'].items():
    file=(source/name).resolve();assert source in file.parents
    assert hashlib.sha256(file.read_bytes()).hexdigest()==digest,name

env=os.environ.copy()
env['PATH']='/home/mscong1/.cargo/bin:'+env.get('PATH','')
env['RUSTFLAGS']='-C target-cpu=native'
env['LC_ALL']='C'
for key in ('GKR_TRACE','FLOCK_ZC_TIMING','FLOCK_POOL_TRACE','FLOCK_SPARSE_GATE','RAYON_NUM_THREADS'):
    env.pop(key,None)
topology=subprocess.check_output(['lscpu','-p=CPU,CORE,SOCKET,NODE'],text=True)
allowed=os.sched_getaffinity(0)
cores={}
for row in topology.splitlines():
    if row.startswith('#'):continue
    cpu,core,socket,node=map(int,row.split(','))
    if cpu in allowed:cores.setdefault((node,socket,core),cpu)
physical=[(key[0],cpu) for key,cpu in sorted(cores.items())]
nodes=sorted({node for node,cpu in physical})
single=[cpu for node,cpu in physical if node==nodes[0]][:24]
dual=[cpu for node,cpu in physical][:48]
assert len(single)==24 and len(dual)==48
def meminfo():
    return {line.split(':')[0]:int(line.split()[1])*1024 for line in Path('/proc/meminfo').read_text().splitlines()}
def busy(exclude=()):
    names={'cargo','rustc','rust-lld','compare','hadamard_vs_flo','packed_compare'}
    out=[]
    for p in Path('/proc').iterdir():
        if not p.name.isdigit() or int(p.name) in exclude:continue
        try:
            if p.stat().st_uid!=os.getuid():continue
            comm=(p/'comm').read_text().strip()
            state=(p/'stat').read_text().rsplit(')',1)[1].split()[0]
            if comm in names and state!='Z':out.append({'pid':int(p.name),'comm':comm})
        except (FileNotFoundError,PermissionError,ProcessLookupError):pass
    return out
def idle(min_available=0):
    quiet=0
    while quiet<3:
        active=busy();available=meminfo()['MemAvailable']
        status('waiting_for_idle',competing_processes=active,available_bytes=available,required_bytes=min_available,quiet_checks=quiet)
        quiet=quiet+1 if not active and available>=min_available else 0
        time.sleep(5)
inventory={'source_manifest':manifest,'uname':list(os.uname()),'topology':topology,
    'lscpu':json.loads(subprocess.check_output(['lscpu','-J'])),
    'affinities':{'24':single,'48':dual},'memory_initial':meminfo(),'load_initial':os.getloadavg(),
    'memory_policy':'Linux first-touch, no numactl override; 24 physical cores on one NUMA node, 48 physical cores across two nodes. Shared host.',
    'toolchain':subprocess.check_output(['/home/mscong1/.cargo/bin/rustc','+nightly-2026-03-21','-Vv'],text=True),
    'rustflags':env['RUSTFLAGS']}
save('environment.json',inventory)

def execute(stage,argv,monitor=False):
    status(stage)
    start=time.monotonic()
    record={'stage':stage,'argv':argv,'utc_start':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime()),'load_start':os.getloadavg(),'interference':[]}
    with (results/(stage+'.log')).open('w') as log:
        p=subprocess.Popen(argv,cwd=work,env=env,stdout=log,stderr=subprocess.STDOUT)
        status(stage,pid=p.pid)
        while p.poll() is None:
            if monitor:
                active=busy((p.pid,))
                if active:record['interference'].append({'elapsed':time.monotonic()-start,'processes':active})
            time.sleep(2)
    record.update(returncode=p.returncode,wall_seconds=time.monotonic()-start,load_end=os.getloadavg())
    commands.append(record);save('commands.json',commands)
    if p.returncode:
        status('failed',failed_stage=stage,returncode=p.returncode)
        raise SystemExit(f'{stage} failed; see log')
    return record

idle()
cargo='/home/mscong1/.cargo/bin/cargo'
execute('fetch',[cargo,'+nightly-2026-03-21','fetch','--locked'])
execute('native-tests',[cargo,'+nightly-2026-03-21','test','--release','--locked','-j','8','--','--test-threads=1'])
execute('native-build',[cargo,'+nightly-2026-03-21','build','--release','--locked','-j','8','--bin','compare'])
binary=work/'target/release/compare'
inventory['binary_sha256']=hashlib.sha256(binary.read_bytes()).hexdigest()
save('environment.json',inventory)

accepted=[]
def case(name,n,threads,systems,runs=6,warmups=1,seed=0):
    cpus=single if threads==24 else dual
    for attempt in range(1,4):
        idle(220*2**30 if n==30 else 0)
        stage=f'{name}-attempt{attempt}'
        output=results/(stage+'.json')
        argv=['taskset','-c',','.join(map(str,cpus)),str(binary),'--log-n',str(n),'--threads',str(threads),
              '--systems',systems,'--runs',str(runs),'--warmups',str(warmups),'--seed',str(seed),'--out',str(output)]
        rec=execute(stage,argv,True)
        rec['case']=name;rec['accepted']=not rec['interference'];save('commands.json',commands)
        if rec['accepted']:
            accepted.append({'case':name,'file':output.name});save('accepted.json',accepted);return
        status('retry_after_interference',excluded_stage=stage)
    raise SystemExit('Repeated interference; all attempts retained')

case('smoke-n20-48t',20,48,'optimized,full,flock',1)
# N26 and N30 both use a 64-bit subfield for the 8-bit value plus public tag.
case('ablation-n26-48t',26,48,'reference,parallel,lookup,optimized,full,flock',6)
for threads in (24,48):
    case(f'main-n30-{threads}t',30,threads,'optimized,full,flock')
    for system in ('optimized','full','flock'):
        case(f'cold-rss-n30-{threads}t-{system}',30,threads,system,1,0,1)
inventory['memory_final']=meminfo();inventory['load_final']=os.getloadavg()
save('environment.json',inventory)
status('complete',accepted_cases=len(accepted))
