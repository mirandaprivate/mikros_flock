"""Validate downloaded binary-permutation evidence; keep widths and dimensions separate."""
from __future__ import annotations

import argparse
from collections import Counter
import csv
import json
import math
from pathlib import Path
import re
import statistics
import tarfile

import server_driver as driver

read, sha, require = driver.read, driver.sha, driver.require
LABELS = {'previous': 'Mikros fused v3', 'optimized': 'Mikros binary K32 fusion', 'flock': 'Flock product_gkr batched'}
PHASE_GROUPS = {
    '解码/首层构造/求值（含融合）': ('decode_ms', 'compact_build_ms', 'coefficient_eval_ms', 'fused_leaf_build_ms'),
    '扩域上层建树': ('extension_build_ms',),
    'GKR 消息/折叠': ('messages_ms', 'fold_and_message_ms', 'folds_ms', 'equality_ms'),
    '同步层释放': ('layer_release_ms',),
    'prover 内源检查': ('source_checks_ms',),
    '缓冲与物化': ('scratch_setup_ms', 'layer_materialize_ms'),
}


def stats(values):
    return dict(samples=len(values), mean=statistics.mean(values), median=statistics.median(values), min=min(values), max=max(values))


def paired(report, numerator, denominator, metric):
    samples = [s for s in report['samples'] if not s['warmup']]
    left = {s['run']: s for s in samples if s['system'] == numerator}
    right = {s['run']: s for s in samples if s['system'] == denominator}
    require(set(left) == set(right), 'unpaired run indices')
    pairs = []
    for run in sorted(left):
        a, b = left[run], right[run]
        require(a['input_digest'] == b['input_digest'] and a[metric] > 0 and b[metric] > 0, 'unpaired input/timer')
        pairs.append(dict(run=run, input_digest=a['input_digest'], numerator_ms=a[metric], denominator_ms=b[metric], ratio=a[metric]/b[metric]))
    return dict(numerator=numerator, denominator=denominator, metric=metric, pairs=pairs,
                ratio_of_medians=statistics.median(p['numerator_ms'] for p in pairs)/statistics.median(p['denominator_ms'] for p in pairs),
                paired_ratios=stats([p['ratio'] for p in pairs]), wins=sum(p['ratio'] < 1 for p in pairs))


def summarize(root):
    res = root / 'results'
    status, env, commands = read(res/'status.json'), read(res/'environment.json'), read(res/'commands.json')
    require(status['stage'] == 'complete', 'driver incomplete')
    manifest, package = read(root/'source_manifest.json'), read(root/'package.json')
    require(env['source_manifest'] == manifest and sha(root/'source.tar.gz') == package['sha256'], 'source provenance')
    with tarfile.open(root/'source.tar.gz', 'r:gz') as archive:
        for name, expected in manifest['files_sha256'].items():
            import hashlib
            require(hashlib.sha256(archive.extractfile('source/'+name).read()).hexdigest() == expected, f'archived source: {name}')
    here = Path(__file__).resolve().parent
    for filename in ('server_driver.py', 'summarize_experiment.py'):
        require(sha(here/filename) == manifest['files_sha256']['binary/'+filename], f'checker differs from snapshot: {filename}')
    download, evidence = read(root/'download_validation.json'), read(root/'results_manifest.json')
    require(sha(root/'results.tar.gz') == download['archive_sha256'], 'download archive hash')
    for name, expected in evidence['files_sha256'].items():
        path = (root/name).resolve()
        require(path.is_relative_to(root) and sha(path) == expected, f'downloaded evidence: {name}')
    require(download['verified_files'] == len(evidence['files_sha256']), 'download inventory count')
    require(env['threads'] == 64 and env['cpus'] == driver.CPUS and env['cpu_spec'] == driver.CPU_SPEC, 'environment affinity')
    selected = [row for row in env['topology'] if row[0] in driver.CPUS]
    cores = Counter((socket, core) for _, core, socket, _ in selected)
    require(len(selected) == 64 and len(cores) == 48 and Counter(cores.values()) == {1: 32, 2: 16}
            and Counter(row[3] for row in selected) == {0: 32, 1: 32}, 'physical core/SMT topology')
    require(driver.HEX64.fullmatch(env['binary_sha256']) and env['toolchain'] == driver.TOOLCHAIN
            and env['rustflags'] == '-C target-cpu=native', 'build metadata')
    plan = driver.case_plan(env['widths'], env['width8_n30'])
    require(env['case_plan'] == plan, 'case plan differs from CLI')
    accepted = read(res/'accepted.json')
    require(len(accepted) == len(plan) == status['accepted_cases'], 'accepted case count')
    for actual, expected in zip(accepted, plan):
        require(all(actual[k] == v for k, v in expected.items()), 'accepted case configuration')
    plan_by_name = {case['case']: case for case in plan}
    accepted_by_name = {case['case']: case for case in accepted}
    require(len(accepted_by_name) == len(plan), 'duplicate accepted case')
    build = [c for c in commands if 'case' not in c]
    require([c['stage'] for c in build] == ['fetch', 'native-tests', 'native-build'], 'build/test commands')
    tests = (res/'native-tests.log').read_text()
    test_count = sum(map(int, re.findall(r'test result: ok\. (\d+) passed', tests)))
    require(test_count == env['passed_tests'] > 0 and 'test result: FAILED' not in tests, 'test results')
    reports, identities, inputs = {}, {}, {}
    excluded, excluded_proofs, accepted_records = [], 0, 0
    reference_meta = None
    for command in commands:
        require(command['returncode'] == 0 and command['argv'][:3] == ['taskset', '-c', driver.CPU_SPEC], 'command exit/affinity')
        require(sha(res/(command['stage']+'.log')) == command['log_sha256'], 'command log SHA')
        if 'case' not in command:
            continue
        case = plan_by_name[command['case']]
        path = res/(command['stage']+'.json')
        report = driver.check_report(path, case)
        require(sha(path) == command['report_sha256'], 'report SHA')
        for flag, expected in (('--width', str(case['width'])), ('--log-n', str(case['log_n'])), ('--threads', '64'),
                               ('--runs', str(case['runs'])), ('--warmups', str(case['warmups'])), ('--seed', str(case['seed'])),
                               ('--systems', ','.join(case['systems']))):
            require(command['argv'][command['argv'].index(flag)+1] == expected, f'command {flag}')
        require(Path(command['argv'][3]).name == 'compare' and Path(command['argv'][command['argv'].index('--out')+1]).name == path.name, 'wrong executable/output')
        meta = report['metadata']
        if reference_meta is None:
            reference_meta = meta
        require(all(meta[k] == reference_meta[k] for k in ('rustc', 'target_features', 'generator', 'scope', 'memory_scope')), 'mixed compiler/protocol metadata')
        for sample in report['samples']:
            key = (meta['width'], meta['log_n'], meta['seed']+sample['run'])
            require(inputs.setdefault(key, sample['input_digest']) == sample['input_digest'], 'input changed across modes')
            protocol = 'flock' if sample['system'] == 'flock' else 'mikros'
            proof = path.with_suffix('.proofs') / f"run{sample['run']}-{sample['system']}.bin"
            identity = (sample['input_digest'], sample['proof_digest'], sha(proof))
            require(identities.setdefault((*key, protocol), identity) == identity, 'proof bytes/digests differ across modes')
        require(command['accepted'] == (not command['interference']), 'interference exclusion policy')
        if command['accepted']:
            entry = accepted_by_name[command['case']]
            require(entry['file'] == path.name and entry['proof_records'] == len(report['samples'])
                    and command['case'] not in reports, 'accepted report identity')
            reports[command['case']] = report
            accepted_records += len(report['samples'])
        else:
            excluded.append(command['stage'])
            excluded_proofs += len(report['samples'])
    require(len(reports) == len(plan) and accepted_records == status['validated_proof_records'], 'validated proof count')
    rows, groups = [], []
    for case in plan:
        report = reports[case['case']]
        group = dict(case=case, systems={}, comparisons=[], file=accepted_by_name[case['case']]['file'])
        for system in case['systems']:
            samples = [s for s in report['samples'] if s['system'] == system and not s['warmup']]
            measurements = {metric: stats([s[metric] for s in samples]) for metric in driver.METRICS}
            measurements['total_with_verify_and_source_ms'] = stats([s['prover_total_ms']+s['verifier_ms']+s['source_check_ms'] for s in samples])
            measurements.update(proof_bytes=samples[0]['proof_bytes'], tuple_bits=samples[0]['tuple_bits'],
                                warmup_core_ms=[s['prover_core_ms'] for s in report['samples'] if s['system'] == system and s['warmup']],
                                cold_rss_kib=samples[0]['memory_at_prover_end_kib'] if case['warmups'] == 0 else None)
            if system != 'flock':
                measurements['tree_stats'] = samples[0]['tree_stats']
                measurements['phases'] = {label: stats([sum(s['phases'].get(k, 0) for k in keys) for s in samples])
                                           for label, keys in PHASE_GROUPS.items()}
            group['systems'][system] = measurements
            row = dict(case=case['case'], width=case['width'], log_n=case['log_n'], threads=64, system=system, runs=case['runs'])
            for metric in driver.METRICS:
                row.update({metric+'_'+stat: value for stat, value in measurements[metric].items() if stat != 'samples'})
            rows.append(row)
        if len(case['systems']) == 3:
            group['comparisons'] = [paired(report, 'optimized', base, metric) for base in ('previous', 'flock')
                                    for metric in ('prover_core_ms', 'prover_total_ms')]
        groups.append(group)
    provenance = {name: sha(root/name) for name in ('source_manifest.json', 'source.tar.gz', 'package.json',
                  'results_manifest.json', 'results.tar.gz', 'download_validation.json')}
    return dict(schema_version=1, validated_proofs=accepted_records, validated_excluded_proofs=excluded_proofs,
                distinct_identities=len(identities), accepted_cases=len(plan), excluded_attempts=excluded,
                source_files=len(manifest['files_sha256']), native_tests=test_count, environment=env,
                evidence_files=len(evidence['files_sha256']), provenance_sha256=provenance, groups=groups), rows


def render(summary):
    env = summary['environment']
    lines = ['# Binary permutation：K32 融合与 Flock 对比', '',
             '本轮以 binary 值（width=1）为主：N30 的值与位置标签合计 31 bits，按实际 tree_stats 使用 K32。'
             'previous 为冻结的 fused v3，optimized 新增 K32 融合构造；width=8 回归单独列出，N30 若执行则为 38-bit tuple/K64。', '',
             'Flock 使用快照 `product_gkr::prove_batched`。两者验证同一关系 f[σ(i)]=g(i)，'
             '没有沿用 Hadamard 的 skip/compression 对齐，也不声称同安全等级。当前为 PIOP，未接入 PCS、承诺 opening 或 grinding。', '',
             f"环境：64 threads，固定 `{driver.CPU_SPEC}`，48 物理核心加 16 SMT sibling，两个 NUMA 节点各 32 logical CPUs。"
             'Linux first-touch，没有显式内存绑定；共享服务器。Rust nightly-2026-03-21、target-cpu=native；实际 ISA 与原始记录一致。', '',
             '## 正式配对结果', '',
             '每组一次预热、六次正式样本覆盖全部六种系统顺序；smoke 只有一次正式样本。'
             '核心计时保留 prover 内同步释放；含转换计时额外包括原生表示转换。公共输入生成/摘要排除，完整源认证单列且执行。', '',
             '| case | 系统 | core median [min,max] s | core mean s | 含转换 median s | verifier ms | 源认证 median s |',
             '|---|---|---:|---:|---:|---:|---:|']
    for g in summary['groups']:
        if g['case']['warmups'] == 0:
            continue
        for name, s in g['systems'].items():
            p = s['prover_core_ms']
            lines.append(f"| {g['case']['case']} | {LABELS[name]} | {p['median']/1000:.3f} [{p['min']/1000:.3f},{p['max']/1000:.3f}] | {p['mean']/1000:.3f} | {s['prover_total_ms']['median']/1000:.3f} | {s['verifier_ms']['median']:.3f} | {s['source_check_ms']['median']/1000:.3f} |")
    lines += ['', '比值为 optimized/对照，小于 1 表示候选更快；逐份按相同 width/N/seed/run/input 配对，再取比值中位数。', '',
              '| case | 对照 | metric | 两中位数之比 | 配对比值 median [min,max] | 候选较快/样本数 |', '|---|---|---|---:|---:|---:|']
    for g in summary['groups']:
        for c in g['comparisons']:
            p = c['paired_ratios']
            lines.append(f"| {g['case']['case']} | {c['denominator']} | {c['metric']} | {c['ratio_of_medians']:.4f} | {p['median']:.4f} [{p['min']:.4f},{p['max']:.4f}] | {c['wins']}/{p['samples']} |")
    lines += ['', '各 width 与 N 严格分开，不把 width8/K64 的收益套到 binary/K32，也不混合 smoke、正式计时或 RSS。'
              '保留回退与全部波动，样本胜场不保证统计显著性或跨环境稳定性。', '', '## 构造与 GKR 阶段', '',
              '阶段来自同一次 instrumented prover，均值按实际样本计算；中位数之和不是总中位数。'
              '融合构造应与原解码+紧凑层构造+求值的合计比较，不把命名阶段单独移除误认为免费。', '',
              '| case | 系统 | 阶段 | mean s | median [min,max] s |', '|---|---|---|---:|---:|']
    for g in summary['groups']:
        if g['case']['warmups'] == 0 or g['case']['log_n'] == 20:
            continue
        for name, s in g['systems'].items():
            for label, p in s.get('phases', {}).items():
                lines.append(f"| {g['case']['case']} | {name} | {label} | {p['mean']/1000:.3f} | {p['median']/1000:.3f} [{p['min']/1000:.3f},{p['max']/1000:.3f}] |")
    lines += ['', '## 独立冷进程 RSS 与证明字节', '',
              '| width | N | 系统 | VmHWM GiB | 冷 core s | 冷 total s | 实际 proof bytes |', '|---:|---:|---|---:|---:|---:|---:|']
    for g in summary['groups']:
        if g['case']['warmups'] != 0:
            continue
        for name, s in g['systems'].items():
            lines.append(f"| {g['case']['width']} | {g['case']['log_n']} | {name} | {s['cold_rss_kib']['VmHWM']/2**20:.3f} | {s['prover_core_ms']['median']/1000:.3f} | {s['prover_total_ms']['median']/1000:.3f} | {s['proof_bytes']} |")
    lines += ['', '每个 RSS 数字来自单系统、无预热、一次 proof 的独立进程，prover 返回后、verifier/源扫描前读取。'
              '它包括公共输入和运行时，不能用混合系统进程的累计 VmHWM 作对照。'
              '证明字节以实际文件核验；N30 Mikros 为 44,208 bytes，Flock 为 16,176 bytes，width 不改变这两个消息长度公式。', '',
              '完整源认证是原始输入直算核验，不等于 PCS。SUMMARY.json 同时给出逐样本 total+verifier+source 的统计，'
              '不能把各阶段中位数直接相加。未测硬件计数器，不将计时差异具体归因于 cache、NUMA 或分配器。', '',
              '## 验收与来源', '',
              f"接受 {summary['accepted_cases']} 个 case、{summary['validated_proofs']} 份证明；另有 {summary['validated_excluded_proofs']} 份受干扰尝试的证明核验通过但计时排除。"
              f"服务器测试通过 {summary['native_tests']} 项；下载 {summary['evidence_files']} 个文件、冻结 {summary['source_files']} 个源码/构建文件的 SHA256 核验通过。", '',
              '排除尝试：' + (', '.join(summary['excluded_attempts']) or '无') + '。每次等待及监测记录均保留；只检查同用户受监测的 benchmark/build，不能保证服务器没有其他负载。'
              '等待有上限，不终止其他进程。', '',
              '所有记录先执行 verifier、claims/transcript 一致性与完整源核验，再输出 verified。'
              '下载后检查 JSON/JSONL 相同、执行顺序、输入与 proof digest、一致的实际 Mikros proof SHA256 和文件长度；'
              '不同协议不要求相同证明摘要。', '',
              f"Source archive SHA256：`{summary['provenance_sha256']['source.tar.gz']}`。", '',
              f"Executable SHA256：`{env['binary_sha256']}`。", '',
              '原始证据：[命令](results/commands.json)、[环境](results/environment.json)、[接受清单](results/accepted.json)、'
              '[等待记录](results/waits.json)、[源码清单](source_manifest.json)、[计划](../../PLAN.md)。'
              '完整数值、阶段、逐输入配对与来源见 [SUMMARY.json](SUMMARY.json) 和 [summary.csv](summary.csv)。', '']
    return '\n'.join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, required=True, help='downloaded binary experiment directory')
    root = parser.parse_args().root.resolve()
    summary, rows = summarize(root)
    (root/'REPORT.md').write_text(render(summary), encoding='utf-8')
    driver.save(root/'SUMMARY.json', summary)
    with (root/'summary.csv').open('w', newline='', encoding='utf-8') as file:
        writer = csv.DictWriter(file, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)
    print(json.dumps({key: summary[key] for key in ('validated_proofs', 'native_tests', 'accepted_cases', 'excluded_attempts')}))


if __name__ == '__main__':
    main()
