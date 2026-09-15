"""Validate same-binary switch-off runs; report exploratory E2E ratio effects."""
import argparse
import json
from pathlib import Path
import statistics

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('evidence', type=Path)
args = parser.parse_args()


def samples(folder):
    status = json.loads((folder / 'status.json').read_text())
    config = json.loads((folder / 'config.json').read_text())
    receipt = json.loads((folder / 'build-receipt.json').read_text())
    assert status['state'] == 'complete'
    assert status['source_hashes_verified_before_and_after'] and status['binary_hashes_verified_before_and_after']
    assert config['levels'] == [26] and config['threads'] == 64
    assert not config.get('diagnostic_trace') and config.get('allocator_policy', 'system') == 'system'
    result = {}
    for mode in ('single', 'multiple'):
        for system in ('mikros', 'flock'):
            lines = (folder / f'n26-{mode}-{system}.stdout.jsonl').read_text().splitlines()
            rows = [x for x in map(json.loads, lines) if x.get('type') == 'sample']
            assert len(rows) == 1
            x = rows[0]
            assert x['verified'] and x['transcript_match'] and x['pcs_config_match']
            result[mode, system] = x
    return result, config, receipt


primary = [samples(args.evidence / f'optimized-n26-r{i}') for i in (1, 2, 3)]
assert all(x[2] == primary[0][2] for x in primary)
base = primary[0][0]
medians = {k: statistics.median(x[0][k]['times']['prover_ms'] for x in primary) for k in base}
result = []
for folder in sorted(args.evidence.glob('ablation-*')):
    if not folder.is_dir():
        continue
    rows, config, receipt = samples(folder)
    assert receipt == primary[0][2], 'Ablations must use the same frozen source and binary'
    for k, x in rows.items():
        for field in ('proof_blake3', 'proof_bytes', 'witness_commitment_blake3', 'public_commitment_blake3',
                      'witness_pcs_params', 'public_pcs_params'):
            assert x[field] == base[k][field], (folder, k, field)
    for mode in ('single', 'multiple'):
        m, f = (rows[mode, s]['times']['prover_ms'] for s in ('mikros', 'flock'))
        baseline_ratio = medians[mode, 'flock'] / medians[mode, 'mikros']
        result.append({'directory': folder.name, 'disabled': config['disabled_optimizations'],
                       'mode': mode, 'samples': 1, 'scope': 'exploratory; confirm effects near timing variation',
                       'mikros_ms': m, 'flock_ms': f, 'all_on_ratio': baseline_ratio,
                       'off_ratio': f/m, 'relative_ratio_change_pct': 100*(f/m/baseline_ratio-1),
                       'same_proof_commitments_and_binary': True,
                       'times': {s: rows[mode, s]['times'] for s in ('mikros', 'flock')}})
out = args.evidence.parent / 'ablation-summary.json'
out.write_text(json.dumps(result, indent=2) + '\n')
print('| Disabled optimization | Mode | Mikros s | FLOCK s | F/M OFF | OFF vs all ON |')
print('|---|---|---:|---:|---:|---:|')
for r in result:
    print(f"| {','.join(r['disabled'])} | {r['mode']} | {r['mikros_ms']/1000:.3f} | {r['flock_ms']/1000:.3f} | {r['off_ratio']:.3f} | {r['relative_ratio_change_pct']:+.2f}% |")
print('Positive OFF-vs-ON means the disabled optimization may favor FLOCK more; this is a one-run screen.')
