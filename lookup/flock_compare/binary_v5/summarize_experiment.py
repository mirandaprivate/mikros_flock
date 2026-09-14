"""Validate permutation v5 ablations, selection, cold/warm memory and explicit final cleanup."""
from __future__ import annotations

import argparse
from collections import Counter
from datetime import datetime
import csv
import json
import math
from pathlib import Path
import re
import statistics
import tarfile

import server_driver as driver

read, sha, require = driver.read, driver.sha, driver.require
LABELS = {'previous': 'Mikros binary v4', 'register': '寄存器 fold/message', 'reuse': '树层池复用',
          'combined': '寄存器与树层池组合', 'flock': 'Flock product_gkr batched'}
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
    root = root.resolve()
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
        require(sha(here/filename) == manifest['files_sha256']['binary_v5/'+filename], f'checker differs from snapshot: {filename}')
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
    selection = read(root/'selection.json')
    require(env['selection'] == selection and env['selected_candidate'] == selection['selected']
            and selection['executable_sha256'] == env['binary_sha256'], 'selection provenance')
    require('selection.json' in evidence['files_sha256'], 'download inventory omits frozen selection')
    plan = driver.case_plan(selection['selected'])
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
    screening = reports['screening-w1-n26-64t']
    selected, means = driver.choose_candidate(screening)
    require(selected == selection['selected'] and all(math.isclose(means[name], selection['candidate_mean_core_ms'][name], rel_tol=1e-12)
                                                     for name in driver.CANDIDATES), 'candidate is not the predeclared minimum-mean choice')
    require(selection['screening_file'] == accepted_by_name['screening-w1-n26-64t']['file']
            and sha(res/selection['screening_file']) == selection['screening_sha256'], 'selection used a different screening report')
    main_command = next(c for c in commands if c.get('case') == 'main-w1-n30-64t')
    screen_command = next(c for c in commands if c.get('case') == 'screening-w1-n26-64t' and c['accepted'])
    require(datetime.fromisoformat(screen_command['utc_end']) <= datetime.fromisoformat(selection['utc'])
            < datetime.fromisoformat(main_command['utc_start']), 'selection chronology')
    rows, groups = [], []
    for case in plan:
        report = reports[case['case']]
        group = dict(case=case, systems={}, comparisons=[], file=accepted_by_name[case['case']]['file'], cleanup=report['cleanup'])
        for system in case['systems']:
            samples = [s for s in report['samples'] if s['system'] == system and not s['warmup']]
            measurements = {metric: stats([s[metric] for s in samples]) for metric in driver.METRICS}
            measurements['total_with_verify_and_source_ms'] = stats([s['prover_total_ms']+s['verifier_ms']+s['source_check_ms'] for s in samples])
            measurements.update(proof_bytes=samples[0]['proof_bytes'], tuple_bits=samples[0]['tuple_bits'],
                                warmup_core_ms=[s['prover_core_ms'] for s in report['samples'] if s['system'] == system and s['warmup']],
                                standalone_memory_kib=samples[0]['memory_at_prover_end_kib'] if '-rss-' in case['case'] else None,
                                memory_samples=[{'run': s['run'], 'warmup': s['warmup'], 'memory_kib': s['memory_at_prover_end_kib']}
                                                for s in report['samples'] if s['system'] == system],
                                tree_pool_samples=[{'run': s['run'], 'warmup': s['warmup'], 'tree_pools': s['tree_pools']}
                                                   for s in report['samples'] if s['system'] == system])
            if system != 'flock':
                measurements['tree_stats'] = samples[0]['tree_stats']
                measurements['phases'] = {label: stats([sum(s['phases'].get(k, 0) for k in keys) for s in samples])
                                           for label, keys in PHASE_GROUPS.items()}
            group['systems'][system] = measurements
            row = dict(case=case['case'], width=case['width'], log_n=case['log_n'], threads=64, system=system, runs=case['runs'])
            for metric in driver.METRICS:
                row.update({metric+'_'+stat: value for stat, value in measurements[metric].items() if stat != 'samples'})
            rows.append(row)
        systems = case['systems']
        pairs = [(candidate, base) for candidate in driver.CANDIDATES if candidate in systems
                 for base in ('previous', 'flock') if base in systems]
        pairs.extend(('combined', base) for base in ('register', 'reuse') if 'combined' in systems and base in systems)
        group['comparisons'] = [paired(report, a, b, metric) for a, b in pairs
                                for metric in ('prover_core_ms', 'prover_total_ms')]
        groups.append(group)
    provenance = {name: sha(root/name) for name in ('source_manifest.json', 'source.tar.gz', 'package.json',
                  'results_manifest.json', 'results.tar.gz', 'download_validation.json', 'selection.json')}
    return dict(schema_version=1, validated_proofs=accepted_records, validated_excluded_proofs=excluded_proofs,
                distinct_identities=len(identities), accepted_cases=len(plan), excluded_attempts=excluded,
                source_files=len(manifest['files_sha256']), native_tests=test_count, environment=env,
                evidence_files=len(evidence['files_sha256']), provenance_sha256=provenance, selection=selection, groups=groups), rows


def render(summary):
    env = summary['environment']
    selection = summary['selection']
    lines = ['# Permutation v5：寄存器融合与受预算约束的树层复用', '',
             f"N26 筛选按预先固定的 measured mean core 最小规则选择 `{selection['selected']}`；精确平局按名称。"
             f"选择记录于 `{selection['utc']}`，早于独立 N30 的第一条命令，源码和 executable 未变。"
             '保留其余候选和反向结果，不按 N30 的有利样本重新选择。', '',
             '本轮只测 width=1：N30 tuple=31 bits/K32。previous 为冻结 binary v4；register 改寄存器内 fold+next-message；'
             'reuse 加树层池；combined 组合两项。Mikros 的相同输入须保持相同证明字节和 transcript。'
             '公共默认仍为 Engine::Fused，候选通过显式 Engine 选择，筛选结果不自动改变默认。'
             'Flock 是 snapshot product_gkr::prove_batched；没有 Hadamard skip 对齐，不声称同安全等级，未包含 PCS/grinding。', '',
             f"64 threads 固定 `{driver.CPU_SPEC}`，48 物理核心+16 SMT sibling，两 NUMA 各32逻辑CPU；"
             'Linux first-touch，共享非独占服务器。Rust nightly-2026-03-21、target-cpu=native；所有 case 同一 executable。', '',
             '已有 Flock 全局 F128 scratch pool 仍由 previous、新候选和 Flock 共享。新增树层池彼此独立：slot0用于 reuse，'
             'slot1用于 combined，各限32 GiB allocation capacity；预算不是已驻留内存。每次都重写数学值。'
             '树层归还可能保留容量而非立即归还操作系统；最终同步清空时间明确排除在 proof timer 外并在下表报告。', '',
             '## 主计时与消融', '',
             'N20 smoke五系统各1次、无预热；N26四系统各1预热+8正式，Williams ABDC/BCAD/CDBA/DACB重复两轮，'
             '位置与相邻前驱均衡而非覆盖24排列。N30旧版/选中候选/Flock各1预热+6正式，完整六种排列。'
             '每项统计仅用本组正式样本，不混合不同N、独立RSS进程或旧binary时序。', '',
             '| case | 系统 | core median [min,max] s | core mean s | total median s | verifier ms | source median s |',
             '|---|---|---:|---:|---:|---:|---:|']
    for g in summary['groups']:
        if '-rss-' in g['case']['case']:
            continue
        for name, data in g['systems'].items():
            p = data['prover_core_ms']
            lines.append(f"| {g['case']['case']} | {LABELS[name]} | {p['median']/1000:.3f} [{p['min']/1000:.3f},{p['max']/1000:.3f}] | {p['mean']/1000:.3f} | {data['prover_total_ms']['median']/1000:.3f} | {data['verifier_ms']['median']:.3f} | {data['source_check_ms']['median']/1000:.3f} |")
    lines += ['', '比值为分子/分母，小于1表示分子较快。先按同组run/input逐样本配对；均值、中位数、配对中位数全部保留。', '',
              '| case | 分子/分母 | metric | 两中位数之比 | paired median [min,max] | 分子较快/样本数 |',
              '|---|---|---|---:|---:|---:|']
    for g in summary['groups']:
        for c in g['comparisons']:
            p = c['paired_ratios']
            lines.append(f"| {g['case']['case']} | {c['numerator']}/{c['denominator']} | {c['metric']} | {c['ratio_of_medians']:.4f} | {p['median']:.4f} [{p['min']:.4f},{p['max']:.4f}] | {c['wins']}/{p['samples']} |")
    lines += ['', 'register/previous 与 reuse/previous 分别衡量两项独立改动；combined/register 和 combined/reuse 检查组合增量。'
              '不把其他候选或旧批次的时间相减来分配本组收益。小样本胜场不保证统计显著性或跨环境稳定性。', '',
              '## 同次 prover 的阶段', '',
              '阶段均值使用本组正式样本；构造按解码/compact构造/求值/融合构造合计。'
              '“层归还/释放”包含同步释放或归还池的耗时；归还较快并不意味着内存释放免费。', '',
              '| case | 系统 | 阶段 | mean s | median [min,max] s |', '|---|---|---|---:|---:|']
    for g in summary['groups']:
        if '-rss-' in g['case']['case'] or g['case']['log_n'] == 20:
            continue
        for name, data in g['systems'].items():
            for label, p in data.get('phases', {}).items():
                label = label.replace('同步层释放', '层归还/释放')
                lines.append(f"| {g['case']['case']} | {name} | {label} | {p['mean']/1000:.3f} | {p['median']/1000:.3f} [{p['min']/1000:.3f},{p['max']/1000:.3f}] |")
    lines += ['', '## 独立冷/暖进程与保留容量', '',
              'cold进程=无预热、1次proof；warm进程=1次预热+1次正式proof，表内计时只取正式proof。'
              '二者都是单系统独立进程，但warm的VmHWM仍是包含预热的进程历史峰值。VmHWM在prover返回、verify/source前读取。'
              '不使用多系统进程的累计VmHWM比较单系统内存。', '',
              '| case | core s | total s | prover后VmHWM GiB | prover后VmRSS GiB | cleanup前两树池保留 GiB | proof bytes |',
              '|---|---:|---:|---:|---:|---:|---:|']
    for g in summary['groups']:
        if '-rss-' not in g['case']['case']:
            continue
        data = next(iter(g['systems'].values()))
        memory = data['standalone_memory_kib']
        retained = sum(p['retained_bytes'] for p in g['cleanup']['tree_pools_before'])
        lines.append(f"| {g['case']['case']} | {data['prover_core_ms']['median']/1000:.3f} | {data['prover_total_ms']['median']/1000:.3f} | {memory['VmHWM']/2**20:.3f} | {memory['VmRSS']/2**20:.3f} | {retained/2**30:.3f} | {data['proof_bytes']} |")
    lines += ['', '上述每种状态只有一次正式proof，冷暖差异属于诊断，不是稳定冷启动保证。'
              '保留容量按pool实际capacity计费；VmRSS还受现有全局scratch、allocator与运行时影响。'
              '全部pool原始计数、每次proof快照和warmup快照保留在SUMMARY.json。', '',
              '## 显式批末清理', '',
              '所有原始输入、转换输入和proof对象离开作用域后，先清两个树池，再清共享F128 scratch；'
              '这段清理在core/total之外，因此单独给出，不能将其停表后发生解释为零成本。'
              'batch wall包含warmup、输入生成、verify、完整source核验与proof写出，不是proving-only，也不与逐proof中位数拼接。', '',
              '| case | 树池清空 s | shared scratch清空 s | 全部cleanup s | 清理前/后VmRSS GiB | batch含cleanup s |',
              '|---|---:|---:|---:|---:|---:|']
    for g in summary['groups']:
        c = g['cleanup']
        lines.append(f"| {g['case']['case']} | {c['tree_pool_clear_ms']/1000:.3f} | {c['shared_scratch_clear_ms']/1000:.3f} | {c['total_ms']/1000:.3f} | {c['memory_before_kib']['VmRSS']/2**20:.3f} / {c['memory_after_kib']['VmRSS']/2**20:.3f} | {c['batch_including_cleanup_ms']/1000:.3f} |")
    lines += ['', 'SUMMARY.json保留batch清理前/后完整时钟与两池原始stats。清空后RSS是否立即下降由实际记录决定，'
              '不将retained bytes等同于RSS差，也不推断cache/NUMA/allocator原因。', '',
              '## 完整性与边界', '',
              f"接受{summary['accepted_cases']}个case、{summary['validated_proofs']}份证明；另核验{summary['validated_excluded_proofs']}份干扰尝试证明、仅排除其计时。"
              f"native测试通过{summary['native_tests']}项，{summary['evidence_files']}个证据文件和{summary['source_files']}个源码/构建文件SHA256核验通过。", '',
              '排除尝试：'+(', '.join(summary['excluded_attempts']) or '无')+'。等待有上限、记录检测到的同用户benchmark/build干扰，不终止别的进程。'
              '未检测到受监测进程不表示主机独占。', '',
              '每份proof先执行verifier、claims/transcript一致性及完整源检查，再发出verified。下载后检查实际proof bytes、'
              'Mikros各变体的原始文件SHA256/proof digest一致、JSON/JSONL、执行顺序与输入身份。Flock与Mikros不要求相同证明摘要。'
              'N30消息长度仍为Mikros44,208/Flock16,176 bytes。', '',
              '输入生成/摘要不计入core/total；系统转换计入total；源认证单列且完整执行。'
              'SUMMARY还给出逐样本total+verify+source统计（排除批末cleanup），它仍不是PCS或完整SNARK。', '',
              f"Source SHA256：`{summary['provenance_sha256']['source.tar.gz']}`。", '',
              f"Executable SHA256：`{env['binary_sha256']}`。", '',
              '[选择](selection.json)、[命令](results/commands.json)、[环境](results/environment.json)、[接受清单](results/accepted.json)、'
              '[等待](results/waits.json)、[源码](source_manifest.json)、[SUMMARY.json](SUMMARY.json)、[summary.csv](summary.csv)。', '']
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
