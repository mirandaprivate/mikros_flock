"""Validate complete paired batches, then summarize every measured arm."""
from pathlib import Path
from collections import defaultdict, Counter
import csv
import json
import statistics
import sys

def protocol(row):
    if row['system']=='flock':
        return 'flock'
    if row['mode'] in ('batched','bridge','full_optimized'):
        return 'batched'
    return 'packed' if row['mode']=='packed' else 'legacy'

def main():
    folder=Path(sys.argv[1])
    metadata=json.loads((folder/'environment.json').read_text())
    assert json.loads((folder/'completion.json').read_text())['completed']
    execution=json.loads((folder/'execution.json').read_text())
    valid=[e for e in execution if e['valid_run']]
    assert len(valid)==len(metadata['cases'])
    rows=[]
    for event,case in zip(valid,metadata['cases']):
        n,w,threads,runs,modes=case
        records=[json.loads(line) for line in (folder/event['sample_file']).read_text().splitlines()]
        assert records[-1]['kind']=='completed'
        samples=records[:-1]
        arms={('mikros',m) for m in modes.split(',')}|{('flock','reference'),('flock','bridge')}
        assert len(samples)==(runs+1)*len(arms)==records[-1]['rows']
        positions=defaultdict(Counter)
        for run in range(runs+1):
            pair=[r for r in samples if r['run']==run]
            assert len(pair)==len(arms)
            assert {(r['system'],r['mode']) for r in pair}==arms
            for r in pair:
                assert (r['log_n'],r['value_bits'],r['threads'])==(n,w,threads)
                assert r['warmup']==(run==0) and r['verified'] and r['transcript_equal']
                assert r['kind']=='sample'
                if run:
                    positions[(r['system'],r['mode'])][r['order']]+=1
            for key in ['input_hash','statement_hash','pcs_params','pcs_queries','pcs_log_inv_rates']:
                assert len({json.dumps(r[key],sort_keys=True) for r in pair})==1,key
            hashes=defaultdict(set)
            for r in pair:
                hashes[protocol(r)].add(r['proof_hash'])
            assert all(len(h)==1 for h in hashes.values())
        if runs==len(arms):
            assert all(set(c.values())=={1} and len(c)==len(arms) for c in positions.values())
        rows.extend(samples)
    # Identical input, protocol, and statement must produce identical proof bytes
    # across thread counts and implementation modes, including warmups.
    golden={}
    for r in rows:
        key=(r['log_n'],r['value_bits'],r['run'],r['system'],protocol(r))
        value=(r['input_hash'],r['statement_hash'],r['proof_hash'])
        if key in golden:
            assert golden[key]==value,'thread/mode differential'
        golden[key]=value
    groups=defaultdict(list)
    for r in rows:
        if not r['warmup']:
            groups[(r['log_n'],r['value_bits'],r['threads'],r['system'],r['mode'])].append(r)
    summary=[]
    for (n,w,t,system,mode),samples in sorted(groups.items()):
        row={'log_n':n,'value_bits':w,'threads':t,'system':system,'mode':mode,'samples':len(samples)}
        for key in ('prepare_ms','commit_ms','piop_ms','bridge_ms','pcs_open_ms','prover_total_ms'):
            values=[r['prover'][key] for r in samples]
            row[key]=statistics.median(values)
            row[key+'_mean']=statistics.mean(values)
        row['prover_min_ms']=min(r['prover']['prover_total_ms'] for r in samples)
        row['prover_max_ms']=max(r['prover']['prover_total_ms'] for r in samples)
        row['verify_ms']=statistics.median(r['verifier']['verifier_total_ms'] for r in samples)
        row['proof_bytes']=statistics.median(r['proof_bytes'] for r in samples)
        detail=[r['prover']['piop_detail'] for r in samples]
        for key in sorted(next((d.keys() for d in detail if d is not None),[])):
            row['detail_'+key]=statistics.median(d[key] for d in detail)
        summary.append(row)
    fields=list(dict.fromkeys(k for r in summary for k in r))
    with (folder/'summary.csv').open('w',newline='') as stream:
        writer=csv.DictWriter(stream,fieldnames=fields);writer.writeheader();writer.writerows(summary)
    (folder/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    report=['# Mikros permutation + Binius: ordered optimization',
        '',f"Suite: {metadata['suite']}. Unit: ms, measured medians. Same raw bit commitments and PCS policy.",
        'Reference is the instrumented original path; Flock/reference retains the original adapter.',
        'Flock/bridge uses the same optimized bit conversion, byte embedding and column evaluator as Mikros.',
        '', '| log N | bits | threads | system / mode | prepare | commit | PIOP | bridge | PCS open | total | verify | proof KiB |',
        '|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|']
    for r in summary:
        times=' | '.join(f'{r[k]:.3f}' for k in ('prepare_ms','commit_ms','piop_ms','bridge_ms','pcs_open_ms','prover_total_ms','verify_ms'))
        report.append(f"| {r['log_n']} | {r['value_bits']} | {r['threads']} | {r['system']}/{r['mode']} | {times} | {r['proof_bytes']/1024:.2f} |")
    report+=['','## PIOP phase attribution','',
        'Phase medians are independently calculated and need not sum to the total median.',
        'Fused message/fold timing is a single combined phase, not separately estimated.',
        '', '| log N | bits | threads | mode | checks | decode | compact tree | coefficient eval | extension tree | materialize | eq | messages | fold | fused fold/message | packed quotient | packed projection | packed columns | skipped rounds |',
        '|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|']
    keys=('source_checks_ms','decode_ms','compact_build_ms','coefficient_eval_ms','extension_build_ms',
        'layer_materialize_ms','equality_ms','messages_ms','folds_ms','fold_and_message_ms',
        'packed_quotient_ms','packed_projection_ms','packed_columns_ms','skipped_rounds')
    for r in summary:
        if r['system']!='mikros':continue
        values=' | '.join(f"{r.get('detail_'+k,0):.3f}" for k in keys)
        report.append(f"| {r['log_n']} | {r['value_bits']} | {r['threads']} | {r['mode']} | {values} |")
    measured=sum(not r['warmup'] for r in rows)
    report+=['',f'Validated {len(rows)} serialized proofs ({measured} measured), including all terminal PCS openings.',
        'Checked same-input statements/parameters, same-protocol proof bytes across modes/threads, and balanced cyclic order in formal suites.',
        f"Excluded and preserved {len(execution)-len(valid)} interfered attempts; no timing-based exclusions.",
        'Fast100/diagnostic PCS parameters; no ZK, no claim of equal complete 128-bit soundness or complete Flock R1CS throughput.',
        'Packed changes the protocol and adds quotient/column identity checks. See PACKED_REDUCTION.md.',
        'This is a shared WSL host; raw samples, ranges, compiler/CPU and source hashes are retained.']
    (folder/'REPORT.md').write_text('\n'.join(report)+'\n')
    (folder/'validation.json').write_text(json.dumps({'valid':True,'proofs':len(rows),'measured':measured,
        'paired_statements':True,'all_pcs_verified':True,'same_protocol_bytes':True,
        'cross_thread_differentials':True,'order_checked':True},indent=2)+'\n')
    print(json.dumps({'groups':len(groups),'proofs':len(rows),'measured':measured}))

if __name__=='__main__':
    main()
