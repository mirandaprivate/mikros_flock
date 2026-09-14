"""Independently validate paired samples and render all retained medians."""
from pathlib import Path
import csv
import json
import statistics
import sys

root = Path(sys.argv[1])
completion = json.loads((root/'completion.json').read_text())
assert completion['completed'] and completion['source_hashes_unchanged']
execution = json.loads((root/'execution.json').read_text())
accepted = [e for e in execution if e['valid_run']]
assert len(accepted)==completion['cases']
assert len({e['case'] for e in accepted})==len(accepted)
assert all(e['competing_builds'] and e['exit_code']==0 for e in execution if not e['valid_run'])
samples = []
for event in accepted:
    path = root/event.get('sample_file',event['case']+'.jsonl')
    lines = [json.loads(s) for s in path.read_text().splitlines()]
    assert lines[-1]['kind']=='completed'
    data = lines[:-1]
    assert len(data)==lines[-1]['rows']
    assert all(s['verified'] and s['transcript_equal'] for s in data)
    paired = {}
    for s in data:
        paired.setdefault(s['run'],[]).append(s)
    assert set(paired)==set(range(lines[-1]['runs']+1))
    for run,pair in paired.items():
        assert len(pair)==2 and {s['system'] for s in pair}=={'mikros','flock'}
        for key in ('input_hash','statement_hash','pcs_params','pcs_queries','pcs_log_inv_rates'):
            assert pair[0][key]==pair[1][key], key
        assert pair[0]['order']==0 and pair[1]['order']==1
    samples.extend(data)
# Same logical input and transcript must give the same proof across pool sizes.
cross_thread = {}
for s in samples:
    key = (s['log_n'],s['value_bits'],s['run'],s['system'])
    identity = (s['input_hash'],s['statement_hash'],s['proof_hash'])
    if key in cross_thread:
        assert cross_thread[key]==identity, ('cross-thread mismatch',key)
    cross_thread[key] = identity
groups = {}
for s in samples:
    if not s['warmup']:
        groups.setdefault((s['log_n'],s['value_bits'],s['threads'],s['system']),[]).append(s)
summary = []
for (n,w,t,system),group in sorted(groups.items()):
    assert len(group)==4 or len(group)==1  # a recorded pilot is also valid
    if len(group)==4:
        assert sorted(s['order'] for s in group)==[0,0,1,1]
    result = {'log_n':n,'value_bits':w,'threads':t,'system':system,'samples':len(group)}
    for section,fields in {
        'prover':['prepare_ms','commit_ms','piop_ms','bridge_ms','pcs_open_ms','prover_total_ms'],
        'verifier':['verify_setup_ms','verify_piop_ms','verify_bridge_ms','pcs_verify_ms','verifier_total_ms']}.items():
        for field in fields:
            v = [s[section][field] for s in group]
            result[field] = statistics.median(v)
            result[field+'_min'] = min(v)
            result[field+'_max'] = max(v)
    for field in ['proof_bytes','piop_bytes','bridge_bytes','pcs_opening_bytes','commitment_bytes',
                  'packed_witness_bytes_each','codeword_bytes_each']:
        result[field] = statistics.median(s[field] for s in group)
    result['embedded_profile'] = group[0]['embedded_profile']
    summary.append(result)
with (root/'summary.csv').open('w',newline='') as f:
    writer = csv.DictWriter(f,fieldnames=list(summary[0]))
    writer.writeheader()
    writer.writerows(summary)
lookup = {(s['log_n'],s['value_bits'],s['threads'],s['system']):s for s in summary}
report = ['# Mikros / Flock permutation + Binius/Ligerito 实测','',
    '相同原始 bit commitments，完整末端 PCS 认证；公开关系为 `f[sigma[i]]=g[i]`。',
    '单位 ms，全部正式样本中位数；每配置预热一对，随后四对交替运行。',
    'Fast100 / 小形状 UDR 诊断参数，不宣称整套组合严格 128-bit soundness；详见 README。','',
    '| log₂N | 位宽 | 线程 | Mikros 总证明 | Flock 总证明 | 耗时比 M/F | Mikros 验证 | Flock 验证 | Mikros proof KiB | Flock proof KiB |',
    '|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|']
for n,w,t in sorted({k[:3] for k in lookup}):
    a,b = [lookup[(n,w,t,s)] for s in ('mikros','flock')]
    report.append(f"| {n} | {w} | {t} | {a['prover_total_ms']:.3f} | {b['prover_total_ms']:.3f} | {a['prover_total_ms']/b['prover_total_ms']:.2f} | {a['verifier_total_ms']:.3f} | {b['verifier_total_ms']:.3f} | {a['proof_bytes']/1024:.2f} | {b['proof_bytes']/1024:.2f} |")
report += ['','## 分阶段中位数','',
    '| log₂N | 位宽 | 线程 | 系统 | 准备 | Commit | PIOP | Bridge | PCS open |',
    '|---:|---:|---:|---|---:|---:|---:|---:|---:|']
for s in summary:
    report.append(f"| {s['log_n']} | {s['value_bits']} | {s['threads']} | {s['system']} | "+
        ' | '.join(f'{s[k]:.3f}' for k in ('prepare_ms','commit_ms','piop_ms','bridge_ms','pcs_open_ms'))+' |')
report += ['',f"共 {len(samples)} 个证明（含预热）全部完成验证、反序列化和 transcript 同步检查。",
    f"共保留 {len(execution)} 个批次，其中 {len(execution)-len(accepted)} 个因检测到并发构建而排除；排除不依赖耗时。",
    '双方每对的原始输入摘要、公开 statement/cap 和 PCS 参数相同。所有原始计时保存在 JSONL；没有按耗时剔除样本。',
    '分阶段中位数之和不必等于总时间中位数。验证包含公开 sigma 的线性工作；输入 witness 不提供给验证器。',
    '证明大小不含公开承诺和 sigma；这些字节单独记在原始样本及 CSV 中。',
    '这是本地共享 WSL 宿主上的当前实现测量，不代表论文全部 packed reduction 优化或完整 Flock R1CS prover。','']
(root/'REPORT.md').write_text('\n'.join(report))
(root/'validation.json').write_text(json.dumps({'proofs_checked':len(samples),'measured_samples':sum(not s['warmup'] for s in samples),'paired_commitments':True,'cross_thread_proof_identity':True,'all_verified':True,'groups':len(summary)},indent=2)+'\n')
print('\n'.join(report[:18]))
