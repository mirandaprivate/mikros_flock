"""Summarize complete independent N26 server runs; no benchmark execution."""
import argparse
import hashlib
import json
from pathlib import Path
import statistics

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--baseline', nargs='+', type=Path, required=True)
parser.add_argument('--optimized', nargs='+', type=Path, required=True)
parser.add_argument('--multiple-optimized', nargs='+', type=Path,
                    help='Optional symmetric, explicitly selected configuration for multiple mode')
parser.add_argument('--multiple-disabled', nargs='*', default=[],
                    help='Exact controls expected in every multiple-selected run')
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
keys = ['gates_ms', 'copy_ms', 'witness_pcs_open_ms', 'public_pcs_open_ms',
        'commit_and_recommit_ms', 'release_ms', 'serialization_ms', 'prover_ms']
labels = ['Gate PIOP', 'Copy PIOP', 'Witness PCS opening', 'Public PCS opening',
          'Commit and recommits', 'Release', 'Serialization', 'Online total']
evidence = {}


def read(path):
    data = path.read_bytes()
    evidence[str(path)] = hashlib.sha256(data).hexdigest()
    return data.decode()


def collect(paths, disabled=()):
    cases = {}
    policies, binaries = set(), set()
    for folder in paths:
        status = json.loads(read(folder / 'status.json'))
        config = json.loads(read(folder / 'config.json'))
        receipt = json.loads(read(folder / 'build-receipt.json'))
        policies.add(config.get('allocator_policy', 'system'))
        binaries.add(receipt['binary_sha256'])
        assert status['state'] == 'complete', folder
        assert config['threads'] == 64 and config['levels'] == [26]
        assert not config.get('diagnostic_trace', False)
        assert sorted(config.get('disabled_optimizations', [])) == sorted(disabled)
        assert status['source_hashes_verified_before_and_after']
        assert status['binary_hashes_verified_before_and_after']
        for mode in ('single', 'multiple'):
            samples = {}
            for system in ('mikros', 'flock'):
                file = folder / f'n26-{mode}-{system}.stdout.jsonl'
                rows = [json.loads(s) for s in read(file).splitlines()]
                metadata = [x for x in rows if x['type'] == 'metadata']
                assert len(metadata) == 1 and metadata[0]['all_core_threads'] == 64
                assert all(metadata[0]['target_features'].values())
                rows = [x for x in rows if x['type'] == 'sample']
                assert len(rows) == 1, 'Use independent fresh processes, one sample each'
                x = rows[0]
                assert x['verified'] and x['transcript_match'] and x['pcs_config_match']
                assert x['gates_per_instance'] == 2**26
                assert x['instances'] == (1 if mode == 'single' else 128)
                x['times']['commit_and_recommit_ms'] = sum(x['times'][k] for k in
                    ['commit_ms', 'recommit_ms', 'public_recommit_ms'])
                cases.setdefault((mode, system), []).append(x)
                samples[system] = x
            for k in ('witness_commitment_blake3', 'public_commitment_blake3',
                      'witness_pcs_params', 'public_pcs_params'):
                assert samples['mikros'][k] == samples['flock'][k], (folder, mode, k)
    assert len(policies) == len(binaries) == 1, 'Do not mix binaries or allocator policies within a variant'
    return cases, next(iter(policies)), next(iter(binaries))


baseline, base_policy, base_binary = collect(args.baseline)
optimized, opt_policy, opt_binary = collect(args.optimized)
if args.multiple_optimized:
    selected, selected_policy, selected_binary = collect(args.multiple_optimized, args.multiple_disabled)
    assert selected_policy == opt_policy and selected_binary == opt_binary
    for system in ('mikros', 'flock'):
        optimized['multiple', system] = selected['multiple', system]
else:
    assert not args.multiple_disabled, 'Selected controls require matching evidence directories'
output = {'scale': 'N26 per circuit; multiple=128 circuits', 'threads': 64,
          'scope': 'Online including conversions, release and recommit; offline excluded',
          'statistic': 'Median of separate processes; phase medians need not sum to total median',
          'baseline_directories': [str(x) for x in args.baseline],
          'optimized_directories': [str(x) for x in args.optimized], 'cases': []}
output['multiple_selected_directories'] = [str(x) for x in args.multiple_optimized or []]
output['disabled_optimizations'] = {'single': [], 'multiple': args.multiple_disabled}
output.update(baseline_allocator=base_policy, optimized_allocator=opt_policy,
              baseline_binary_sha256=base_binary, optimized_binary_sha256=opt_binary)
lines = ['# N26 E2E measured comparison', '', output['scope'], '', output['statistic'], '']
lines += [f'Allocator: baseline={base_policy}, optimized={opt_policy}.', '']
lines += [f"Disabled optimizations, identically for both systems: {output['disabled_optimizations']}.", '']
for mode in ('single', 'multiple'):
    lines += [f'## {mode}', '', '| Stage (seconds) | Baseline Mikros | Baseline FLOCK | Optimized Mikros | Optimized FLOCK | FLOCK / Mikros after |',
              '|---|---:|---:|---:|---:|---:|']
    for system in ('mikros', 'flock'):
        rows = baseline[mode, system] + optimized[mode, system]
        for k in ('proof_blake3', 'proof_bytes', 'witness_commitment_blake3',
                  'public_commitment_blake3', 'witness_pcs_params', 'public_pcs_params'):
            assert all(x[k] == rows[0][k] for x in rows), (mode, system, k)
        case = {'mode': mode, 'system': system, 'unchanged_proof_and_commitments': True,
                'proof_blake3': rows[0]['proof_blake3'], 'proof_bytes': rows[0]['proof_bytes']}
        for variant, data in [('baseline', baseline), ('optimized', optimized)]:
            group = data[mode, system]
            case[variant] = {'samples': len(group), 'times_ms': {k: {
                'median': statistics.median(x['times'][k] for x in group),
                'min': min(x['times'][k] for x in group),
                'max': max(x['times'][k] for x in group)} for k in keys}}
        output['cases'].append(case)
    for k, label in zip(keys, labels):
        values = [statistics.median(x['times'][k] for x in data[mode, system]) / 1000
                  for data in (baseline, optimized) for system in ('mikros', 'flock')]
        ratio = values[3] / values[2]
        lines.append('| ' + label + ' | ' + ' | '.join(f'{v:.4f}' for v in values) + f' | {ratio:.3f} |')
    for system in ('mikros', 'flock'):
        values = [x['times']['prover_ms'] / 1000 for x in optimized[mode, system]]
        lines += ['', f'{system} optimized total samples (seconds): {values}.']
    lines.append('')
output['evidence_sha256'] = evidence
args.output.mkdir(parents=True, exist_ok=True)
(args.output / 'comparison.json').write_text(json.dumps(output, indent=2) + '\n')
(args.output / 'comparison.md').write_text('\n'.join(lines) + '\n')
print('\n'.join(lines))
