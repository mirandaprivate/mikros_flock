import os,sys,json,time,hashlib,subprocess
from pathlib import Path
sys.path.insert(0,'mikros/server_experiments/20260917-hashes')
from run_hashes import COMMON_ENV, CPU_SPEC, TOOLCHAIN
out=Path('validation-native'); out.mkdir(exist_ok=False)
env={k:v for k,v in os.environ.items() if not k.startswith(('FLOCK_','MIKROS_','E2E_','LIG_','RAYON_')) and k not in ('RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','CARGO_BUILD_RUSTFLAGS','RUSTC_WRAPPER','RUSTC_WORKSPACE_WRAPPER','PCS_TRACE','LD_PRELOAD')}
env.update(COMMON_ENV,CARGO_TARGET_DIR='/home/mscong1/mikros-experiments/20260912T-native64/v14-packed-aggregate-20260914/source/target',LC_ALL='C')
env['PATH']=str(Path.home()/'.cargo/bin')+':'+env.get('PATH','')
base=['cargo',TOOLCHAIN,'test','--release','--offline','--locked','-j8','--message-format=short']
jobs=[('core',['-p','flock-core','--lib']),('mikros',['-p','mikros','--lib']),('hashes',['-p','mikros','--example','e2e_hash_compare']),('keccak',['-p','flock-prover','--lib','r1cs_hashes::keccak']),('sha',['-p','mikros','--example','e2e_sha256_paired'])]
summary=[]
def digest(p): return hashlib.sha256(p.read_bytes()).hexdigest()
for name,args in jobs:
 command=['taskset','-c',CPU_SPEC]+base+args+['--','--test-threads=1']
 start=time.monotonic()
 with (out/(name+'.stdout.log')).open('w') as a,(out/(name+'.stderr.log')).open('w') as b:
  p=subprocess.run(command,env=env,stdout=a,stderr=b,timeout=1800)
 entry=dict(name=name,command=command,exit_code=p.returncode,wall_seconds=time.monotonic()-start,stdout_sha256=digest(out/(name+'.stdout.log')),stderr_sha256=digest(out/(name+'.stderr.log')))
 summary.append(entry)
 (out/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
 print(json.dumps(entry),flush=True)
 if p.returncode: raise SystemExit(p.returncode)
(out/'environment.json').write_text(json.dumps({k:v for k,v in env.items() if k.startswith(('FLOCK_','MIKROS_','RAYON_')) or k in ('RUSTFLAGS','CARGO_TARGET_DIR','LC_ALL')},indent=2)+'\n')
(out/'evidence_sha256.json').write_text(json.dumps({p.name:digest(p) for p in sorted(out.iterdir()) if p.is_file() and p.name!='evidence_sha256.json'},indent=2)+'\n')
print('NATIVE_VALIDATION_COMPLETE',flush=True)
