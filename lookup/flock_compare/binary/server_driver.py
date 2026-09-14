"""Bounded, serial binary-permutation comparison on the authorized 64-CPU server."""
from __future__ import annotations

import argparse
from collections import Counter
import hashlib
import itertools
import json
import math
import os
from pathlib import Path
import re
import subprocess
import time

PARENT = Path('/home/mscong1/mikros-experiments/20260912T-native64')
CPU_SPEC = '0-47,48-55,72-79'
CPUS = list(range(48)) + list(range(48, 56)) + list(range(72, 80))
SYSTEMS = ['previous', 'optimized', 'flock']
TOOLCHAIN = '+nightly-2026-03-21'
METRICS = ('prepare_ms', 'prover_core_ms', 'prover_total_ms', 'verifier_ms', 'source_check_ms')
ORDERS = [(0, 1, 2), (1, 2, 0), (2, 0, 1), (2, 1, 0), (0, 2, 1), (1, 0, 2)]
HEX64 = re.compile(r'[0-9a-f]{64}')


def require(value, message):
    if not value:
        raise ValueError(message)


def read(path):
    return json.loads(path.read_text(encoding='utf-8'))


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def utc():
    return time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())


def save(path, value):
    path.write_text(json.dumps(value, indent=2, allow_nan=False) + '\n', encoding='utf-8')


def expand_cpus(spec):
    cpus = []
    for item in spec.split(','):
        bounds = list(map(int, item.split('-')))
        cpus.extend(range(bounds[0], bounds[-1] + 1))
    return cpus


def case_plan(widths, width8_n30=False):
    cases = []
    for width in widths:
        for label, n, runs in [('smoke', 20, 1), ('screening', 26, 6)]:
            cases.append(dict(case=f'{label}-w{width}-n{n}-64t', width=width, log_n=n,
                              systems=SYSTEMS, runs=runs, warmups=1, seed=0))
        if width == 1 or width8_n30:
            cases.append(dict(case=f'main-w{width}-n30-64t', width=width, log_n=30,
                              systems=SYSTEMS, runs=6, warmups=1, seed=0))
            for system in SYSTEMS:
                cases.append(dict(case=f'cold-rss-w{width}-n30-64t-{system}', width=width,
                                  log_n=30, systems=[system], runs=1, warmups=0, seed=1))
    return cases


def check_report(path, case):
    """Validate every raw record and saved proof; callable without server access."""
    report = read(path)
    require(report['schema_version'] == 2, f'{path.name}: schema')
    meta = report['metadata']
    for key in ('width', 'log_n', 'systems', 'runs', 'warmups', 'seed'):
        require(meta[key] == case[key], f'{path.name}: {key}')
    require(meta['threads'] == 64 and {'avx512f', 'vpclmulqdq'} <= set(meta['target_features']), 'native 64-thread metadata')
    require('1.96.0-nightly' in meta['rustc'] and 'no PCS/grinding' in meta['scope'], 'compiler/protocol scope')
    affinity = next(line.split(':', 1)[1].strip() for line in meta['affinity'] if line.startswith('Cpus_allowed_list:'))
    require(expand_cpus(affinity) == CPUS, 'benchmark CPU affinity')
    samples = report['samples']
    require(len(samples) == (meta['runs'] + meta['warmups']) * len(meta['systems']), 'record count')
    stream = [json.loads(line) for line in path.with_suffix('.samples.jsonl').read_text().splitlines() if line]
    require(stream == samples, 'JSON/JSONL mismatch')
    for run in range(meta['runs'] + meta['warmups']):
        group = [s for s in samples if s['run'] == run]
        systems = meta['systems']
        order = ORDERS[(run - meta['warmups']) % 6] if len(systems) == 3 and run >= meta['warmups'] else range(len(systems))
        require([s['system'] for s in group] == [systems[i] for i in order], 'execution order')
        require([s['order'] for s in group] == list(range(len(systems))), 'position indices')
        require(len({s['input_digest'] for s in group}) == 1, 'inputs differ within run')
        mikros = []
        for sample in group:
            require(sample['verified'] is True and sample['warmup'] == (run < meta['warmups']), 'unverified/misplaced proof')
            require(sample['log_n'] == meta['log_n'] and sample['value_bits'] == meta['width']
                    and sample['tuple_bits'] == meta['log_n'] + meta['width'], 'wrong statement width')
            require(all(HEX64.fullmatch(sample[k]) for k in ('input_digest', 'proof_digest')), 'invalid digest')
            require(all(math.isfinite(sample[k]) and sample[k] >= 0 for k in METRICS), 'invalid timer')
            require(sample['prover_total_ms'] + 0.001 >= sample['prepare_ms'] + sample['prover_core_ms'], 'timing boundary')
            n = meta['log_n']
            flock = sample['system'] == 'flock'
            fields = n*n + 3*n + 5 if flock else 3*n*n + n + 2
            overhead = 16 + 8*n if flock else 16 + 16*n
            proof = path.with_suffix('.proofs') / f"run{run}-{sample['system']}.bin"
            require(proof.stat().st_size == sample['proof_bytes'] == fields*16 + overhead, 'actual proof byte length')
            if not flock:
                require(all(math.isfinite(x) and x >= 0 for x in sample['phases'].values()), 'invalid phase')
                for stats in sample['tree_stats'].values():
                    require(stats['subfield_bits'] in (1, 2, 4, 8, 16, 32, 64, 128)
                            and stats['subfield_bits'] >= sample['tuple_bits'], 'coefficient field cannot encode tuple')
                mikros.append((sample['proof_digest'], sha(proof)))
            memory = sample['memory_at_prover_end_kib']
            require(memory['VmHWM'] >= memory['VmRSS'] > 0, 'missing/invalid RSS')
        require(len(set(mikros)) <= 1, 'Mikros raw proofs differ')
    return report


def verify_sources(source):
    manifest = read(source / 'source_manifest.json')
    for name, expected in manifest['files_sha256'].items():
        path = (source / name).resolve()
        require(path.is_relative_to(source) and sha(path) == expected, f'source SHA: {name}')
    required = {'binary/server_driver.py', 'binary/Cargo.toml', 'binary/Cargo.lock', 'binary/build.rs',
                'fused/Cargo.toml', 'snapshot/crates/flock-core/src/product_gkr.rs'}
    for path in source.rglob('*.rs'):
        if 'target' not in path.relative_to(source).parts:
            required.add(path.relative_to(source).as_posix())
    require(required <= set(manifest['files_sha256']), 'source manifest omits compiled files')
    return manifest


def memory():
    return {line.split(':')[0]: int(line.split()[1])*1024 for line in Path('/proc/meminfo').read_text().splitlines()}


def cgroups():
    records = []
    entry = next((line[3:] for line in Path('/proc/self/cgroup').read_text().splitlines() if line.startswith('0::')), None)
    if entry is None:
        return records
    mount = Path('/sys/fs/cgroup')
    path = mount / entry.lstrip('/')
    while path.is_relative_to(mount):
        record = {'path': str(path)}
        for name in ('memory.max', 'memory.current', 'cpu.max', 'cpu.stat'):
            if (path / name).exists():
                record[name] = (path / name).read_text().strip()
        records.append(record)
        if path == mount:
            break
        path = path.parent
    return records


def competing_processes(exclude=()):
    found = []
    names = {'cargo', 'rustc', 'rust-lld', 'compare', 'hadamard_vs_flo', 'univariate_vs_f', 'packed_compare'}
    for path in Path('/proc').iterdir():
        if not path.name.isdigit() or int(path.name) in exclude:
            continue
        try:
            if path.stat().st_uid != os.getuid():
                continue
            comm = (path / 'comm').read_text().strip()
            state = (path / 'stat').read_text().rsplit(')', 1)[1].split()[0]
            if state != 'Z' and (comm in names or 'bench' in comm):
                found.append({'pid': int(path.name), 'comm': comm})
        except (FileNotFoundError, PermissionError, ProcessLookupError):
            pass
    return found


def parse_args():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, required=True)
    parser.add_argument('--widths', default='1', help='distinct widths 1,8; each gets smoke and N26')
    parser.add_argument('--width8-n30', action='store_true', help='also run N30 main/RSS for selected width 8')
    parser.add_argument('--max-idle-seconds', type=int, default=1800, help='bounded wait before each command/case')
    parser.add_argument('--attempts', type=int, default=3, help='maximum whole-case attempts after detected interference')
    args = parser.parse_args()
    args.widths = [int(x) for x in args.widths.split(',')]
    require(args.widths and len(set(args.widths)) == len(args.widths) and set(args.widths) <= {1, 8}, 'widths must be 1 and/or 8')
    require(not args.width8_n30 or 8 in args.widths, '--width8-n30 requires width 8')
    require(15 <= args.max_idle_seconds <= 7200 and 1 <= args.attempts <= 3, 'wait/attempt bounds')
    return args


def experiment(args):
    root = args.root.resolve()
    require(root.parent == PARENT and re.fullmatch(r'permutation-binary-v4(?:-[A-Za-z0-9_-]+)?', root.name), 'unauthorized experiment root')
    source = (root / 'source').resolve()
    require(source.is_relative_to(root) and source.is_dir(), 'missing source snapshot')
    work = source / 'binary'
    results = root / 'results'
    results.mkdir(exist_ok=False)
    commands, accepted, waits = [], [], []

    def status(stage, **extra):
        record = {'stage': stage, 'utc': utc(), **extra}
        save(results / 'status.json', record)
        print(json.dumps(record), flush=True)

    def idle(required=0):
        start, quiet = time.monotonic(), 0
        while quiet < 3:
            active, mem, groups = competing_processes(), memory(), cgroups()
            available = min([mem['MemAvailable']] + [max(0, int(g['memory.max'])-int(g['memory.current']))
                            for g in groups if g.get('memory.max', 'max') != 'max' and 'memory.current' in g])
            record = {'utc': utc(), 'competing_processes': active, 'available_bytes': available,
                      'required_bytes': required, 'elapsed_seconds': time.monotonic()-start, 'cgroups': groups}
            waits.append(record)
            save(results / 'waits.json', waits)
            quiet = quiet + 1 if not active and available >= required else 0
            if quiet == 3:
                return
            status('waiting_for_idle', **record)
            require(time.monotonic()-start < args.max_idle_seconds, 'bounded idle/resource wait expired; no processes stopped')
            time.sleep(5)

    try:
        manifest = verify_sources(source)
        require(read(root / 'source_manifest.json') == manifest, 'root/source manifests differ')
        env = os.environ.copy()
        env['PATH'] = '/home/mscong1/.cargo/bin:' + env.get('PATH', '')
        env.update(RUSTFLAGS='-C target-cpu=native', LC_ALL='C', CARGO_BUILD_JOBS='8')
        for key in ('GKR_TRACE', 'FLOCK_ZC_TIMING', 'FLOCK_POOL_TRACE', 'FLOCK_SPARSE_GATE', 'RAYON_NUM_THREADS',
                    'CARGO_ENCODED_RUSTFLAGS', 'CARGO_TARGET_DIR', 'RUSTC_WRAPPER', 'RUSTC_WORKSPACE_WRAPPER'):
            env.pop(key, None)
        topology = [tuple(map(int, row.split(','))) for row in subprocess.check_output(
            ['lscpu', '-p=CPU,CORE,SOCKET,NODE'], text=True).splitlines() if row and not row.startswith('#')]
        selected = [row for row in topology if row[0] in CPUS]
        cores = Counter((socket, core) for _, core, socket, _ in selected)
        require(set(CPUS) <= os.sched_getaffinity(0) and len(selected) == 64 and len(cores) == 48
                and Counter(cores.values()) == {1: 32, 2: 16} and Counter(row[3] for row in selected) == {0: 32, 1: 32}, 'CPU topology mismatch')
        plan = case_plan(args.widths, args.width8_n30)
        inventory = {'utc_start': utc(), 'source_manifest': manifest, 'uname': list(os.uname()), 'topology': topology,
                     'lscpu': json.loads(subprocess.check_output(['lscpu', '-J'])), 'cpu_spec': CPU_SPEC, 'cpus': CPUS,
                     'threads': 64, 'selected_physical_cores': 48, 'additional_smt_siblings': 16,
                     'memory_initial': memory(), 'load_initial': os.getloadavg(), 'cgroups_initial': cgroups(),
                     'widths': args.widths, 'width8_n30': args.width8_n30, 'case_plan': plan,
                     'max_idle_seconds': args.max_idle_seconds, 'maximum_attempts': args.attempts,
                     'memory_policy': 'Linux default first-touch, no explicit NUMA binding; shared non-exclusive host',
                     'toolchain': TOOLCHAIN, 'rustflags': env['RUSTFLAGS'], 'environment_overrides': {k: env[k] for k in ('RUSTFLAGS', 'LC_ALL', 'CARGO_BUILD_JOBS')}}
        save(results / 'environment.json', inventory)
        prefix = ['taskset', '-c', CPU_SPEC]

        def execute(stage, argv, monitor=False):
            record = {'stage': stage, 'argv': argv, 'cwd': str(work), 'utc_start': utc(), 'load_start': os.getloadavg(),
                      'memory_start': memory(), 'cgroups_start': cgroups(), 'interference': []}
            commands.append(record)
            save(results / 'commands.json', commands)
            start = time.monotonic()
            logfile = results / (stage + '.log')
            with logfile.open('w', encoding='utf-8') as log:
                process = subprocess.Popen(argv, cwd=work, env=env, stdout=log, stderr=subprocess.STDOUT)
                status(stage, pid=process.pid)
                while process.poll() is None:
                    if monitor:
                        active = competing_processes((process.pid,))
                        if active:
                            record['interference'].append({'elapsed': time.monotonic()-start, 'processes': active})
                            save(results / 'commands.json', commands)
                    time.sleep(2)
            record.update(returncode=process.returncode, utc_end=utc(), wall_seconds=time.monotonic()-start,
                          load_end=os.getloadavg(), memory_end=memory(), cgroups_end=cgroups(), log_sha256=sha(logfile))
            save(results / 'commands.json', commands)
            require(process.returncode == 0, f'{stage} failed; see retained log')
            return record

        cargo = '/home/mscong1/.cargo/bin/cargo'
        for stage, cargo_args in [('fetch', ['fetch', '--locked']),
                                  ('native-tests', ['test', '--release', '--locked', '--offline', '-j', '8', '--', '--test-threads=1']),
                                  ('native-build', ['build', '--release', '--locked', '--offline', '-j', '8', '--bin', 'compare'])]:
            idle()
            execute(stage, prefix + [cargo, TOOLCHAIN] + cargo_args)
        test_log = (results / 'native-tests.log').read_text()
        inventory['passed_tests'] = sum(map(int, re.findall(r'test result: ok\. (\d+) passed', test_log)))
        require(inventory['passed_tests'] > 0 and 'test result: FAILED' not in test_log, 'missing passing test suite')
        binary = work / 'target/release/compare'
        inventory['binary_sha256'] = sha(binary)
        save(results / 'environment.json', inventory)
        for case in plan:
            for attempt in range(1, args.attempts + 1):
                idle(220 * 2**30 if case['log_n'] == 30 else 0)
                require(sha(binary) == inventory['binary_sha256'], 'executable changed')
                stage = f"{case['case']}-attempt{attempt}"
                output = results / (stage + '.json')
                argv = prefix + [str(binary), '--log-n', str(case['log_n']), '--width', str(case['width']),
                                 '--threads', '64', '--systems', ','.join(case['systems']), '--runs', str(case['runs']),
                                 '--warmups', str(case['warmups']), '--seed', str(case['seed']), '--out', str(output)]
                record = execute(stage, argv, True)
                report = check_report(output, case)
                record.update(case=case['case'], accepted=not record['interference'], report_sha256=sha(output))
                save(results / 'commands.json', commands)
                if record['accepted']:
                    accepted.append({**case, 'file': output.name, 'proof_records': len(report['samples'])})
                    save(results / 'accepted.json', accepted)
                    break
                status('retry_after_interference', excluded_stage=stage)
            else:
                raise ValueError('repeated interference; every attempt retained, no other process stopped')
        require(verify_sources(source) == manifest and sha(binary) == inventory['binary_sha256'], 'source/executable changed')
        inventory.update(utc_end=utc(), memory_final=memory(), load_final=os.getloadavg(), cgroups_final=cgroups())
        save(results / 'environment.json', inventory)
        status('complete', accepted_cases=len(accepted), validated_proof_records=sum(c['proof_records'] for c in accepted))
    except Exception as error:
        status('failed', error=str(error))
        raise


if __name__ == '__main__':
    experiment(parse_args())
