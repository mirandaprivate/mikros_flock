"""Bounded local evidence consistency audit; does not replay proofs or execute ELF."""
import hashlib, json, math, re, statistics, sys, tarfile, types
from pathlib import Path, PurePosixPath
ROOT = Path(__file__).resolve().parent
PINS = {'v3': '6c8c82681171dd3f91c088509379e62e41cc07e72a7f6b381b853bfc425ef971', 'v4': 'c07aed56fb6e9e51ed514efcf07d374e6a8d4340dd104e9326160a9408c833d4'}
sha = lambda b: hashlib.sha256(b).hexdigest()
read = lambda p: json.loads(p.read_text(encoding='utf-8-sig'))
def check(ok, message):
    if not ok: raise ValueError(message)
def save(p, obj):
    p.write_text(json.dumps(obj, indent=2) + '\n', encoding='utf-8')
def archive(path):
    with tarfile.open(path, 'r:gz') as tf:
        members = tf.getmembers(); names = [m.name for m in members]
        check(len(names) == len(set(n.casefold() for n in names)), 'duplicate archive paths')
        for m in members:
            p = PurePosixPath(m.name)
            check((m.isfile() or m.isdir()) and not p.is_absolute() and '..' not in p.parts and '\\' not in m.name and ':' not in m.name, 'unsafe archive member: ' + m.name)
        return {m.name: tf.extractfile(m).read() for m in members if m.isfile()}
def extract(files, dest):
    dest.mkdir(exist_ok=True); dest = dest.resolve()
    for name, data in files.items():
        p = dest / name
        check(p.resolve().is_relative_to(dest), 'extract escapes target')
        if p.exists(): check(p.is_file() and p.read_bytes() == data, 'existing evidence differs: ' + name)
        else: p.parent.mkdir(parents=True, exist_ok=True); p.write_bytes(data)
    check({str(p.relative_to(dest)).replace('\\', '/') for p in dest.rglob('*') if p.is_file()} == set(files), 'extra extracted files')
def load_runner(source):
    for name in ('run_server', 'run_public_csc', 'run_column_projection', 'run_engineering'):
        path = 'mikros/server_experiments/20260916-sha128/' + name + '.py'
        module = types.ModuleType(name); module.__file__ = path; sys.modules[name] = module
        exec(compile(source[path], path, 'exec'), module.__dict__)
    return module
def profile(logs, plan):
    out = {'method': 'Exclude first record of each kind/side in each process (warmup); sum corresponding A+B before median; nested records remain separate.', 'modes': {}}
    for mode in plan['modes']:
        grouped = {}
        for batch in range(3):
            rows = [json.loads(l) for l in (logs / f'{mode}-{batch}.stderr.log').read_text(encoding='utf-8').splitlines() if l.startswith('{')]
            kinds = {(r['record'], r.get('side', '')) for r in rows}; current = {}
            for rec, side in sorted(kinds):
                seq = [r for r in rows if (r['record'], r.get('side', '')) == (rec, side)]
                check(len(seq) == plan['reps'] + 1, 'profile record count')
                current[(rec, side)] = seq[1:]
            rec = 'mikros_public_image_batch_profile'; aa, bb = current[(rec, 'A')], current[(rec, 'B')]
            current[(rec, 'A+B')] = [{k: a[k] + b[k] for k in a if k.endswith('_ms')} for a, b in zip(aa, bb)]
            for (rec, side), seq in current.items(): grouped.setdefault(rec + (':' + side if side else ''), []).append(seq)
        out['modes'][mode] = {}
        for kind, batches in grouped.items():
            rows = sum(batches, []); keys = [k for k in rows[0] if k.endswith('_ms')]
            check(all(math.isfinite(r[k]) and r[k] >= 0 for r in rows for k in keys), 'invalid profile time')
            out['modes'][mode][kind] = {'samples': len(rows), 'median_ms': {k: statistics.median(r[k] for r in rows) for k in keys}, 'batch_medians_ms': [{k: statistics.median(r[k] for r in b) for k in keys} for b in batches]}
    return out
for version, evidence_pin in PINS.items():
    snap = ROOT / ('engineering-' + version); tar = ROOT / f'engineering-{version}-evidence.tar.gz'
    check(sha(tar.read_bytes()) == evidence_pin, 'evidence archive SHA256')
    source_sha = sha((snap / 'source.tar.gz').read_bytes()); check(source_sha == (snap / 'source.tar.gz.sha256').read_text().split()[0], 'source archive SHA256')
    source = archive(snap / 'source.tar.gz'); manifest = json.loads(source['source_manifest.json']); listed = manifest['files_sha256']
    check(set(source) == set(listed) | {'source_manifest.json'} and len(listed) == 401, 'source inventory')
    check(all(sha(source[n]) == h for n, h in listed.items()), 'source file hash'); manifest_hash = sha(source['source_manifest.json'])
    files = archive(tar); dest = snap / 'evidence'; extract(files, dest)
    check(files['source_manifest.json'] == source['source_manifest.json'], 'evidence source manifest')
    runner = load_runner(source); receipt = read(dest / 'engineering-build.json'); binaries_hash = receipt['binary_sha256']
    check(receipt['source_manifest_sha256'] == manifest_hash and receipt['toolchain'] == runner.TOOLCHAIN and receipt['rustflags'] == runner.COMMON_ENV['RUSTFLAGS'], 'build receipt')
    binaries = {'flock' if Path(n).name == 'e2e_flock_baseline' else 'mikros': n for n in binaries_hash}; check(len(binaries) == 2, 'binary inventory')
    actual = {}
    for name, expected in binaries_hash.items():
        p = dest / 'evidence-binaries' / Path(name).name
        if version == 'v4':
            b = p.read_bytes(); check(b[:4] == b'\x7fELF' and b[4:6] == bytes([2, 1]) and int.from_bytes(b[18:20], 'little') == 62 and sha(b) == expected, 'actual ELF hash/architecture'); actual[p.name] = expected
        else: check(not p.exists(), 'unexpected v3 executable')
    audit = {'passed': True, 'scope': 'Downloaded records and source/evidence consistency; no cryptographic proof replay, no binary execution.', 'source_archive_sha256': source_sha, 'source_files_verified': len(listed), 'evidence_archive_sha256': evidence_pin, 'archive_files_verified': len(files), 'actual_elf_sha256': actual, 'binary_receipt_sha256': binaries_hash, 'actual_binary_limitation': 'v3 has receipts only; executable bytes unavailable' if version == 'v3' else None, 'stages': {}, 'tests': {}}
    reference = None
    for logs in sorted(dest.glob('results-engineering-*')):
        hashes = read(logs / 'evidence_sha256.json'); check(set(hashes) == {p.name for p in logs.iterdir() if p.is_file()} - {'evidence_sha256.json'}, 'log manifest inventory')
        check(all(sha((logs / n).read_bytes()) == h for n, h in hashes.items()), 'log hash')
        plan = read(logs / 'run_plan.json'); status = read(logs / 'status.json'); expected_names = ['validation-' + m for m in plan['validation_modes']] + [f'{m}-{b}' for b, order in enumerate(plan['order']) for m in order]
        check(status['state'] == 'complete' and all(r['exit_code'] == 0 for r in status['completed']) and [r['name'] for r in status['completed'] if r['name'] != 'build'] == expected_names, 'status/order')
        check(plan['batches'] == 3 and plan['warmups'] == 1 and plan['reps'] == (3 if plan['stage'] == 'profile' else 6) and plan['order'] == runner.orders(plan['modes']), 'plan repetitions/order')
        check(plan['source_manifest_sha256'] == manifest_hash and plan['source_files'] == len(listed) and read(logs / 'binary-sha256.json') == binaries_hash, 'stage source/binary receipt')
        if (logs / 'build-reused.json').exists(): check(read(logs / 'build-reused.json') == receipt, 'reused receipt')
        if plan['selected_json_sha256']: check(any(sha(v) == plan['selected_json_sha256'] for n, v in files.items() if n.startswith('selected-')), 'selected env JSON provenance')
        hardware = read(logs / 'hardware.json'); check(hardware['affinity'] == runner.CPU_SPEC, 'CPU affinity')
        flags = re.findall(r'^flags\s*:\s*(.*)$', hardware['cpuinfo'], re.M); check(flags and all(runner.FEATURES <= set(f.split()) for f in flags), 'hardware flags')
        summary = runner.summarize(logs, plan['modes'], plan['reps'], plan['validation_modes'], plan['stage']); check(summary == read(logs / 'summary.json'), 'full summary recomputation')
        if reference is None: reference = summary['shared_statement_digests']
        check(summary['shared_statement_digests'] == reference, 'statement differs between stages')
        medians = {}
        for name in expected_names:
            tamper = name.startswith('validation-'); mode = name[11:] if tamper else name.rsplit('-', 1)[0]; env = plan['validation_modes' if tamper else 'modes'][mode]
            meta, samples, _ = runner.read_samples(logs / (name + '.jsonl'), mode, 1 if tamper else plan['reps'], 0 if tamper else 1, tamper)
            rawenv = read(logs / (name + '.environment.json')); expected = {**runner.COMMON_ENV, **env, 'CARGO_TARGET_DIR': str(PurePosixPath(binaries['mikros']).parents[2])}
            if plan['stage'] == 'profile': expected.update(MIKROS_PROFILE='1', PCS_TRACE='1')
            check(rawenv == expected and (mode == 'flock' or meta['engineering_environment'] == env), 'recorded environment')
            cmd = read(logs / (name + '.command.json')); check(cmd[:3] == ['/usr/bin/time', '-v', '-o'] and Path(cmd[3]).name == name + '.resource.txt' and cmd[4:7] == ['taskset', '-c', runner.CPU_SPEC] and cmd[7:] == runner.command(binaries, mode, 1 if tamper else plan['reps'], 0 if tamper else 1, tamper), 'benchmark command')
            pre = read(logs / (name + '.preflight.json')); check(0 <= pre['selected_cpu_busy_fraction'] < .10 and pre['mem_available_kib'] > 8 * 1024 * 1024, 'preflight')
            check(re.search(r'Exit status:\s*0\s*$', (logs / (name + '.resource.txt')).read_text(), re.M), 'resource exit status')
            if not tamper: medians.setdefault(mode, []).extend(r['prove_with_commit_ms'] if mode == 'flock' else r['timings']['prover_ms'] for r in samples if not r['warmup'])
        medians = {m: statistics.median(v) for m, v in medians.items()}; check(all(t == summary['modes'][m]['prove_with_commit_ms' if m == 'flock' else 'prover_ms'] for m, t in medians.items()), 'independent total medians')
        audit['stages'][logs.name] = {'evidence_files_hashed': len(hashes), 'formal_samples': len(plan['modes']) * 3 * plan['reps'], 'warmup_samples': len(plan['modes']) * 3, 'validation_samples': len(plan['validation_modes']), 'negative_rejections': summary['negative_tests'], 'median_prover_ms': medians, 'full_summary_equal': True, 'raw_records_environments_commands_verified': True}
        if version == 'v4' and plan['stage'] == 'profile': save(snap / 'profile_summary.json', profile(logs, plan))
    for p in sorted((dest / 'validation-native').glob('*.log')):
        matches = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;', p.read_text(encoding='utf-8')); check(matches and all(int(a) > 0 and int(b) == 0 for a, b, c in matches), 'test results')
        audit['tests'][p.name] = [{'passed': int(a), 'failed': int(b), 'ignored': int(c)} for a, b, c in matches]
    audit['shared_statement_digests'] = reference; save(snap / 'local_audit.json', audit)
    print(version, 'PASS', 'source', len(listed), 'archive files', len(files), 'actual ELF', len(actual), 'stages', {k: v['median_prover_ms'] for k, v in audit['stages'].items()})
