"""Validate all saved proof bytes, pairing/order and provenance; write v3 report."""
import argparse
import csv
import hashlib
import itertools
import json
import math
from pathlib import Path
from statistics import median

parser=argparse.ArgumentParser()
parser.add_argument('--root',type=Path,required=True)
root=parser.parse_args().root.resolve()
here=Path(__file__).resolve().parent
def read(path):return json.loads(path.read_text(encoding='utf-8'))
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
res=root/'results'
assert read(res/'status.json')['stage']=='complete'
accepted=read(res/'accepted.json'); assert len(accepted)==10
commands=read(res/'commands.json'); assert all(c['returncode']==0 for c in commands)
excluded=[c['stage'] for c in commands if c.get('case') and not c.get('accepted')]
env=read(res/'environment.json')
assert env['source_manifest']==read(root/'source_manifest.json')
assert sha(root/'source.tar.gz')==read(root/'package.json')['sha256']
download=read(root/'download_validation.json')
assert sha(root/'results.tar.gz')==download['archive_sha256']
for name,digest in read(root/'results_manifest.json')['files_sha256'].items():
    path=(root/name).resolve();assert root in path.parents
    assert sha(path)==digest,name
reports=[('local-n26-6t',here/'results/local-n26-v3.json',read(here/'results/local-n26-v3.json'))]
for case in accepted:
    path=res/case['file']; report=read(path); meta=report['metadata']
    cmd=next(c for c in commands if c.get('case')==case['case'] and c.get('accepted'))
    assert not cmd['interference']
    assert {'avx512f','vpclmulqdq'}.issubset(meta['target_features'])
    assert cmd['argv'][:2]==['taskset','-c']
    assert list(map(int,cmd['argv'][2].split(',')))==env['affinities'][str(meta['threads'])]
    reports.append((case['case'],path,report))

identities={}; rows=[]; count=0
metrics=('prepare_ms','prover_core_ms','prover_total_ms','verifier_ms','source_check_ms')
for name,path,report in reports:
    meta=report['metadata']; n=meta['log_n']; systems=meta['systems']
    assert len(report['samples'])==(meta['runs']+meta['warmups'])*len(systems)
    orders=[]
    for run in range(meta['runs']+meta['warmups']):
        group=[s for s in report['samples'] if s['run']==run]
        assert len(group)==len(systems) and {s['system'] for s in group}==set(systems)
        assert {s['order'] for s in group}==set(range(len(systems)))
        assert len({s['input_digest'] for s in group})==1
        if run>=meta['warmups']:orders.append(tuple(s['system'] for s in sorted(group,key=lambda s:s['order'])))
        for s in group:
            assert s['verified'] and s['warmup']==(run<meta['warmups'])
            assert s['log_n']==n and s['tuple_bits']==n+meta['width']
            assert all(math.isfinite(s[k]) and s[k]>=0 for k in metrics)
            assert s['prover_total_ms']+0.001>=s['prepare_ms']+s['prover_core_ms']
            data=(path.with_suffix('.proofs')/f"run{run}-{s['system']}.bin").read_bytes()
            fields=n*n+3*n+5 if s['system']=='flock' else 3*n*n+n+2
            overhead=16+8*n if s['system']=='flock' else 16+16*n
            assert len(data)==s['proof_bytes']==16*fields+overhead
            key=(n,meta['width'],meta['seed']+run,'flock' if s['system']=='flock' else 'mikros')
            identity=(s['input_digest'],s['proof_digest'],hashlib.sha256(data).hexdigest())
            assert identities.setdefault(key,identity)==identity,(name,s['system'],run)
            if s['system']=='optimized' and n==30:
                assert s['phases']['fused_leaf_build_ms']>0
                assert s['phases']['decode_ms']==s['phases']['compact_build_ms']==s['phases']['coefficient_eval_ms']==0
            count+=1
    if len(systems)==3 and meta['runs']==6:
        assert set(orders)==set(itertools.permutations(systems)) and len(orders)==6
    for system in systems:
        samples=[s for s in report['samples'] if not s['warmup'] and s['system']==system]
        row={'case':name,'system':system,'log_n':n,'threads':meta['threads'],'runs':meta['runs']}
        for metric in metrics:
            values=[s[metric] for s in samples]
            row.update({metric+'_min':min(values),metric+'_median':median(values),metric+'_max':max(values)})
        rows.append(row)
assert count==96,count

old=here.parent/'streaming/server_experiments/20260912T103800-permutation-v2'
cross=0
for case in read(old/'results/accepted.json'):
    path=old/'results'/case['file']; report=read(path); meta=report['metadata']
    for s in report['samples']:
        key=(meta['log_n'],meta['width'],meta['seed']+s['run'],'flock' if s['system']=='flock' else 'mikros')
        if key not in identities:continue
        proof=path.with_suffix('.proofs')/f"run{s['run']}-{s['system']}.bin"
        assert identities[key]==(s['input_digest'],s['proof_digest'],sha(proof))
        cross+=1
assert cross==124,cross

lookup={name:report for name,path,report in reports}
systems=('previous','optimized','flock')
labels={'previous':'Mikros v2 流式','optimized':'Mikros v3 融合','flock':'Flock'}
def samples(report,system):return [s for s in report['samples'] if not s['warmup'] and s['system']==system]
def med(report,system,key):return median(s[key] for s in samples(report,system))
def phase(report,system,keys):return median(sum(s['phases'].get(k,0) for k in keys) for s in samples(report,system))/1000
def rss(threads,system):return lookup[f'cold-rss-n30-{threads}t-{system}']['samples'][0]['memory_at_prover_end_kib']['VmHWM']/2**20
def paired(report,a,b,key):
    old={s['run']:s[key] for s in samples(report,a)}
    return median(old[s['run']]/s[key] for s in samples(report,b))
r48=lookup['main-n30-48t']
doc=['# Mikros permutation v3：融合建树与 Flock 对比','',
    '将 v2 的解码、紧凑叶子写入、第一层系数乘积和求值，替换为一次融合扫描；保留紧凑叶子，直接生成第一层扩域乘积。仅 K64、128-bit 系数容量启用。原始证明字节、挑战顺序和验证器不变。', '',
    f"N=2^30、48 个物理核的核心耗时中位数：v2 {med(r48,'previous','prover_core_ms')/1000:.3f} s，v3 {med(r48,'optimized','prover_core_ms')/1000:.3f} s，Flock {med(r48,'flock','prover_core_ms')/1000:.3f} s。含原生输入转换的对应中位数为 {med(r48,'previous','prover_total_ms')/1000:.3f} / {med(r48,'optimized','prover_total_ms')/1000:.3f} / {med(r48,'flock','prover_total_ms')/1000:.3f} s。源认证另列，尚未接入 PCS。", '',
    f"本轮 v3 相对 v2 的核心耗时中位数减少 {100*(1-med(r48,'optimized','prover_core_ms')/med(r48,'previous','prover_core_ms')):.1f}%，含转换减少 {100*(1-med(r48,'optimized','prover_total_ms')/med(r48,'previous','prover_total_ms')):.1f}%。峰值内存基本不变，仍比本次 Flock dense 实现低 {100*(1-rss(48,'optimized')/rss(48,'flock')):.1f}%。这是本次实现和配置的结果，不是协议的内存下界。", '',
    '## N30 主结果','',
    '每组一次预热、六份正式配对输入；三种系统恰好覆盖六种执行顺序。N 是置换项数，8-bit 值加 30-bit 位置标签，实际为 K64。时间为正式样本中位数；RSS 来自单独冷进程，不能从混合计时进程的累计 VmHWM 比较。', '',
    '| 物理核 | 系统 | 核心 s | 含转换 s | 源认证 s | 峰值 GiB |','|---:|---|---:|---:|---:|---:|']
for threads in (24,48):
    report=lookup[f'main-n30-{threads}t']
    for s in systems:
        doc.append(f"| {threads} | {labels[s]} | {med(report,s,'prover_core_ms')/1000:.3f} | {med(report,s,'prover_total_ms')/1000:.3f} | {med(report,s,'source_check_ms')/1000:.3f} | {rss(threads,s):.3f} |")
doc+=['','同输入配对加速：先逐份求 v2/v3 比值，再取中位数；大于 1 表示加速。','',
    '| 物理核 | 核心加速 | 含转换加速 | v3 相对 v2 内存减少 |','|---:|---:|---:|---:|']
for threads in (24,48):
    report=lookup[f'main-n30-{threads}t']
    memory_delta=100*(1-rss(threads,'optimized')/rss(threads,'previous'))
    if abs(memory_delta)<0.005:memory_delta=0.0
    doc.append(f"| {threads} | {paired(report,'previous','optimized','prover_core_ms'):.3f}× | {paired(report,'previous','optimized','prover_total_ms'):.3f}× | {memory_delta:.2f}% |")
doc+=['','如果连同本基准的 verifier 重放与完整源扫描一起计时，逐样本相加再取中位数如下。这仍然没有 PCS，不能称为完整 SNARK 时间；也不能简单把前表中位数相加。','',
    '| 物理核 | 系统 | 含转换+verifier+直接源认证 s |','|---:|---|---:|']
for threads in (24,48):
    report=lookup[f'main-n30-{threads}t']
    for system in systems:
        combined=median(s['prover_total_ms']+s['verifier_ms']+s['source_check_ms'] for s in samples(report,system))/1000
        doc.append(f'| {threads} | {labels[system]} | {combined:.3f} |')
doc+=['','## 波动与阶段归因','',
    '| 物理核 | 系统 | 核心最小 s | 中位数 s | 最大 s |','|---:|---|---:|---:|---:|']
for threads in (24,48):
    report=lookup[f'main-n30-{threads}t']
    for system in systems:
        values=[s['prover_core_ms']/1000 for s in samples(report,system)]
        doc.append(f'| {threads} | {labels[system]} | {min(values):.3f} | {median(values):.3f} | {max(values):.3f} |')
doc+=['','48 核阶段中位数（秒）；融合构造将原来三个阶段合在一起，因此应该比较合计。阶段中位数之和不一定等于总中位数。','',
    '| 阶段 | v2 | v3 |','|---|---:|---:|']
groups={'解码+首层系数构造+求值/融合构造':['decode_ms','compact_build_ms','coefficient_eval_ms','fused_leaf_build_ms'],
        '扩域上层建树':['extension_build_ms'],'GKR 消息/折叠':['messages_ms','fold_and_message_ms','folds_ms','equality_ms'],
        '同步层释放':['layer_release_ms'],'prover 内源检查':['source_checks_ms'],'缓冲获取':['scratch_setup_ms']}
for label,keys in groups.items():doc.append(f"| {label} | {phase(r48,'previous',keys):.3f} | {phase(r48,'optimized',keys):.3f} |")
doc+=['','v3 取消每棵树首层的 8 GiB 系数临时数组，但两棵树依次构造、整个过程峰值可能出现在 GKR；因此不要推断进程峰值必然少 16 GiB。两版本均同步清理，清理时间保留在 core 内。', '',
    '融合构造是实测加速来源：48 核该阶段由 4.552 s 降至 2.647 s。同步层释放却由 2.496 s 上升至 3.081 s，抵消部分收益；GKR 消息/折叠基本相近。未使用硬件计数器或内核跟踪，不把释放变化具体归因于某种缓存、NUMA 或分配器机制。下一步主要可优化项是输入转换、同步释放和源认证；证明大小的差距则需要修改 batching/消息协议。', '',
    '## N26 与本地验证','',
    '| 环境 | 系统 | 核心中位数 s | 含转换中位数 s |','|---|---|---:|---:|']
for name,label in [('local-n26-6t','本地 WSL / 6 线程'),('main-n26-48t','服务器 / 48 核')]:
    for s in systems:doc.append(f"| {label} | {labels[s]} | {med(lookup[name],s,'prover_core_ms')/1000:.3f} | {med(lookup[name],s,'prover_total_ms')/1000:.3f} |")
doc+=['','本地 Windows/WSL 与其他应用共享资源，出现明显波动；不以本地数字替代服务器配对结论。','',
    '## 理论优势的适用范围','',
    'Mikros 当前优势是较小的原生位表示、紧凑叶子和按层释放。当前两条独立三次 GKR 链的 proof 为 44,208 bytes，Flock 的 batched 二次 core 为 16,176 bytes，后者还将 f/g/σ 归约到同一行开点。', '',
    '稿件的完整系数域方案给出单个 grand product 的 O(Nκ) BB + O(N) BE + (10N/κ + lower-order terms) EE；当前程序没有实现完整 packed Hadamard reduction，不能声称已经获得该 EE 计数或 κ 倍速度提升。详见 [THEORY.md](../../THEORY.md)。', '',
    '## 可复核性与限制','',
    f'本报告采用的 {count} 份本地/服务器证明全部重放 verifier、核验全部源声明并检查实际文件大小；与 v2 的 {cross} 份重叠 proof 核对了原始字节 SHA256、input digest 和 proof digest。新旧 Mikros 证明及 transcript 相同。', '',
    '因检测到其他 benchmark/build 进程而排除的完整尝试：'+(', '.join(excluded) if excluded else '无')+'。排除尝试的原始日志和数据保留在归档中，未计入上述统计或证明份数；脚本待空闲后重跑整组。', '',
    f"{len(accepted)} 个服务器 case 成功。服务器 native AVX512/VPCLMUL 差分测试通过，本地差分测试及 Clippy 通过。结果归档 {download['verified_files']} 个文件的哈希检查通过。v1/v2 Rust 与冻结 Flock 文件逐项校验未变。", '',
    '双路 Xeon Gold 6342，24 核固定单 NUMA 节点，48 核覆盖两节点，均使用物理核、不使用 SMT sibling。Rust nightly-2026-03-21、target-cpu=native。共享服务器：接受的 case 未检测到受监测的同用户 benchmark/build 干扰，并不声称主机完全独占。未测硬件性能计数器。', '',
    '公共输入生成、公共摘要不计时；系统原生表示转换计入 total，源认证单独计时。源认证是完整输入直算检查，不等同于 PCS 验证；本基准不含承诺、opening、grinding 或完整 SNARK 安全性对齐。', '',
    f"Source SHA256: `{read(root/'package.json')['sha256']}`。", '',
    f"Binary SHA256: `{env['binary_sha256']}`。", '',
    '原始 JSON/JSONL、实际 proofs、命令与日志保留在 results/，完整指标在 summary.csv。','']
(root/'REPORT.md').write_text('\n'.join(doc),encoding='utf-8')
with (root/'summary.csv').open('w',newline='',encoding='utf-8') as f:
    writer=csv.DictWriter(f,fieldnames=list(rows[0]));writer.writeheader();writer.writerows(rows)
record={'validated_proofs':count,'cross_version_v2_proofs':cross,'distinct_statement_protocol_pairs':len(identities),'accepted_server_cases':len(accepted),'excluded_attempts':excluded}
(root/'validation.json').write_text(json.dumps(record,indent=2)+'\n',encoding='utf-8')
print(json.dumps(record)); print(root/'REPORT.md')
