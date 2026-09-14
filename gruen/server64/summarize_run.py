"""Audit and summarize a downloaded, completed server run (read-only by default)."""
import argparse
from collections import Counter
import csv
import hashlib
import json
import math
from pathlib import Path
import statistics as st

METHODS = ['gruen_2eval', 'gruen_skip_kappa', 'mikros_lagrange_reference', 'mikros_lagrange_optimized']
PHASES = ['setup_ms', 'helpers_or_quotient_ms', 'packing_weights_ms', 'packing_ms', 'rounds_ms', 'round_fold_ms', 'terminal_ms']


def audit(root):
    read = lambda name: json.loads((root / name).read_text())
    assert read('status.json') == {'stage': 'complete', 'accepted_cases': 4, 'source_and_binary_unchanged': True}
    env, commands, accepted = read('environment.json'), read('commands.json'), read('accepted.json')
    assert env['threads'] == 64 and len(set(env['selected_cpus'])) == 64
    assert env['build_tests_this_run'] is True
    assert all(c['returncode'] == 0 for c in commands)
    assert {'fmt', 'tests', 'clippy', 'build'} <= {c['stage'] for c in commands}
    report = {'environment': env, 'cases': [], 'verified_proofs': 0, 'proof_pairs': 0}
    identities = set()
    for case in accepted:
        folder = root / case['folder']
        assert folder.resolve().is_relative_to(root.resolve())
        command = next(c for c in commands if c['stage'] == case['folder'])
        assert command['accepted'] and not command['interference']
        completion = json.loads((folder / 'completion.json').read_text())
        assert completion['completed'] and completion['threads'] == 64
        assert completion['repetitions'] == 8 and completion['verified_proofs_including_warmups'] == 36
        assert completion['mikros_optimization_pairs_equal'] == 8
        with (folder / 'raw.csv').open() as file:
            rows = list(csv.DictReader(file))
        with (folder / 'summary.csv').open() as file:
            summaries = list(csv.DictReader(file))
        assert len(rows) == 32 and len(summaries) == 4
        field, n = rows[0]['field'], int(rows[0]['n'])
        assert field in ('Goldilocks', 'KoalaBear') and n in (20, 30)
        assert (field, n) not in identities
        identities.add((field, n))
        kappa = 2 if field == 'Goldilocks' else 4
        result = {'field': field, 'n': n, 'kappa': kappa, 'folder': case['folder'], 'methods': {}}
        for method in METHODS:
            selected = [r for r in rows if r['method'] == method]
            assert len(selected) == 8 and {int(r['rep']) for r in selected} == set(range(8))
            assert Counter(int(r['order']) for r in selected) == Counter({i: 2 for i in range(4)})
            for row in selected:
                assert row['field'] == field and int(row['n']) == n and int(row['threads']) == 64
                assert int(row['extension_degree']) == kappa
                assert int(row['k']) == (0 if method == METHODS[0] else kappa.bit_length()-1)
                assert row['verified'] == row['mikros_pair_equal'] == 'true'
                for key in ['total_ms', 'process_cpu_ms', 'verify_reduction_ms', 'oracle_ms', *PHASES]:
                    assert math.isfinite(float(row[key])) and float(row[key]) >= 0
                assert float(row['total_ms']) > 0
                assert sum(float(row[p]) for p in PHASES) <= float(row['total_ms']) + 0.001
            times = [float(r['total_ms']) for r in selected]
            summary = next(s for s in summaries if s['method'] == method)
            assert math.isclose(st.median(times), float(summary['median_ms']), abs_tol=1e-5)
            result['methods'][method] = {
                'median_ms': st.median(times), 'min_ms': min(times), 'max_ms': max(times),
                'median_process_cpu_over_wall': st.median(float(r['process_cpu_ms']) / float(r['total_ms']) for r in selected),
                'phases_median_ms': {p: st.median(float(r[p]) for r in selected) for p in PHASES},
                'proof_element_bytes': int(summary['proof_element_bytes']),
                'verifier_median_ms': st.median(float(r['verify_reduction_ms']) for r in selected),
            }
        for rep in range(8):
            a = folder / f'proof-{field}-n{n}-r{rep}-s2.txt'
            b = folder / f'proof-{field}-n{n}-r{rep}-s3.txt'
            assert a.read_bytes() == b.read_bytes()
        assert len(list(folder.glob('proof-*.txt'))) == 32
        optimized = result['methods'][METHODS[3]]['median_ms']
        result['gruen_over_mikros'] = {m: result['methods'][m]['median_ms'] / optimized for m in METHODS[:2]}
        by_rep = {(r['method'], int(r['rep'])): float(r['total_ms']) for r in rows}
        result['paired'] = {m: {
            'mikros_faster_repetitions': sum(by_rep[METHODS[3], i] < by_rep[m, i] for i in range(8)),
            'median_gruen_over_mikros': st.median(by_rep[m, i] / by_rep[METHODS[3], i] for i in range(8)),
        } for m in METHODS[:2]}
        result['raw_sha256'] = hashlib.sha256((folder / 'raw.csv').read_bytes()).hexdigest()
        report['cases'].append(result)
        report['verified_proofs'] += 36
        report['proof_pairs'] += 8
    assert identities == {(f, n) for f in ('Goldilocks', 'KoalaBear') for n in (20, 30)}
    return report


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('root', type=Path)
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    report = audit(args.root)
    encoded = json.dumps(report, indent=2) + '\n'
    if args.output:
        with args.output.open('x') as output:
            output.write(encoded)
        print(f"Validated {report['verified_proofs']} proofs and {report['proof_pairs']} exact Mikros pairs.")
    else:
        print(encoded)
