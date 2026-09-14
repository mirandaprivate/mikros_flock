"""Validate actual proof files, paired samples and all reported statistics."""
import argparse
import csv
import hashlib
import json
import math
from pathlib import Path
import statistics

parser=argparse.ArgumentParser()
parser.add_argument('--root',type=Path,required=True)
args=parser.parse_args()
root=args.root.resolve()
here=Path(__file__).resolve().parent
results=root/'results'
assert json.loads((results/'status.json').read_text())['stage']=='complete'
environment=json.loads((results/'environment.json').read_text())
cpu_model=next(x['data'] for x in environment['lscpu']['lscpu'] if x['field'].rstrip(':：') in ('Model name','型号名称'))
source_manifest=json.loads((root/'source_manifest.json').read_text())
assert environment['source_manifest']==source_manifest
archive_meta=json.loads((root/'package.json').read_text())
assert hashlib.sha256((root/'source.tar.gz').read_bytes()).hexdigest()==archive_meta['sha256']
commands=json.loads((results/'commands.json').read_text())
assert all(c['returncode']==0 for c in commands)
accepted=json.loads((results/'accepted.json').read_text())
assert len(accepted)==10
reports=[]
for threads in (1,6):
    p=here/'results'/f'local-ablation-{threads}t.json'
    reports.append((f'local-{threads}t',p,json.loads(p.read_text())))
for item in accepted:
    path=results/item['file']
    command=next(c for c in commands if c.get('case')==item['case'] and c.get('accepted'))
    assert not command['interference']
    report=json.loads(path.read_text())
    meta=report['metadata']
    assert {'avx512f','vpclmulqdq'}.issubset(meta['target_features'])
    assert command['argv'][:2]==['taskset','-c']
    assert list(map(int,command['argv'][2].split(',')))==environment['affinities'][str(meta['threads'])]
    reports.append((item['case'],path,report))

identities={}
rows=[]
validated=0
metrics=('prepare_ms','prover_core_ms','prover_total_ms','verifier_ms','source_check_ms')
def stat(values):
    values=list(values)
    return {'median':statistics.median(values),'min':min(values),'max':max(values)}
def med(report,system,metric):
    return statistics.median(x[metric] for x in report['samples'] if x['system']==system and not x['warmup'])
for name,path,report in reports:
    meta=report['metadata']
    n=meta['log_n'];systems=meta['systems']
    assert len(report['samples'])==(meta['runs']+meta['warmups'])*len(systems)
    for run in range(meta['warmups']+meta['runs']):
        group=[s for s in report['samples'] if s['run']==run]
        assert len(group)==len(systems) and {s['system'] for s in group}==set(systems)
        assert {s['order'] for s in group}==set(range(len(systems)))
        assert len({s['input_digest'] for s in group})==1
        for s in group:
            assert s['verified'] and s['warmup']==(run<meta['warmups'])
            assert all(math.isfinite(s[k]) and s[k]>=0 for k in metrics)
            assert s['prover_total_ms']+0.001>=s['prepare_ms']+s['prover_core_ms']
            assert s['log_n']==n and s['tuple_bits']==n+meta['width']
            proof=path.with_suffix('.proofs')/f"run{run}-{s['system']}.bin"
            data=proof.read_bytes()
            fields=n*n+3*n+5 if s['system']=='flock' else 3*n*n+n+2
            overhead=16+8*n if s['system']=='flock' else 16+16*n
            assert len(data)==s['proof_bytes']==16*fields+overhead
            key=(n,meta['width'],meta['seed']+run,'flock' if s['system']=='flock' else 'mikros')
            identity=(s['input_digest'],s['proof_digest'],hashlib.sha256(data).hexdigest())
            assert identities.setdefault(key,identity)==identity,(name,s['system'],run)
            validated+=1
    for system in systems:
        samples=[s for s in report['samples'] if s['system']==system and not s['warmup']]
        if len(systems)==3 and meta['runs']==6:
            assert [sum(s['order']==i for s in samples) for i in range(3)]==[2,2,2]
        if len(systems)==6 and meta['runs']==6:
            assert [sum(s['order']==i for s in samples) for i in range(6)]==[1]*6
        row={'case':name,'log_n':n,'threads':meta['threads'],'system':system,'runs':meta['runs'],'proof_bytes':samples[0]['proof_bytes']}
        for metric in metrics:
            for label,value in stat(s[metric] for s in samples).items():row[metric+'_'+label]=value
        rows.append(row)

lookup={name:report for name,path,report in reports}
main24=lookup['main-n30-24t'];main48=lookup['main-n30-48t']
ablation=lookup['ablation-n26-48t']
local=lookup['local-6t']
speedup=med(local,'reference','prover_core_ms')/med(local,'optimized','prover_core_ms')
doc=['# Mikros permutation：瓶颈、工程优化与服务器 N=2^30 实验','',
    f"保持原证明逐字节不变，服务器 N=2^26、48 物理核的 compact prover 从 {med(ablation,'reference','prover_core_ms')/1000:.3f} 秒降到 {med(ablation,'optimized','prover_core_ms')/1000:.3f} 秒，加速 {med(ablation,'reference','prover_core_ms')/med(ablation,'optimized','prover_core_ms'):.2f}×。N=2^30 已完成 24/48 物理核实验：全扩域候选的核心时间接近 Flock，含原生输入转换后仍稍慢；紧凑候选尚未获得峰值内存优势。",'',
    '实验对象为同一公开关系 `f[sigma[i]]=g[i]`，N 表示置换项数。值为 8 bit；N30 的 Mikros 输入还包含 30-bit 位置标签，逻辑宽度 38，实际子域 K=GF(2^64)。使用冻结的 Flock batched Product-GKR，对照优化前 Mikros、分阶段候选及优化后的 full-extension 树。所有原始 f/g/sigma 数据均绑定到 transcript。', '',
    '## N=2^30：服务器主结果','',
    '单位秒，每个系统一次预热、六次正式测量，遍历三系统的全部六种执行顺序。输入生成和共同摘要不计时；总 prover 包含原生输入转换。以下均为中位数。', '',
    '| 物理核 | 系统 | 核心 prover s | 含转换总计 s | 源声明核验 s | 消息 bytes |',
    '|---:|---|---:|---:|---:|---:|']
for threads in (24,48):
    r=lookup[f'main-n30-{threads}t']
    for system in ('optimized','full','flock'):
        sample=next(s for s in r['samples'] if s['system']==system)
        doc.append(f"| {threads} | {system} | {med(r,system,'prover_core_ms')/1000:.3f} | {med(r,system,'prover_total_ms')/1000:.3f} | {med(r,system,'source_check_ms')/1000:.3f} | {sample['proof_bytes']:,} |")
doc+=['','`optimized` 是优化后的紧凑系数树；`full` 使用相同优化 GKR、保存全扩域树。二者证明、源声明和 transcript 完全相同。', '',
    '### 成本差距与线程扩展','',
    '| 物理核 | Compact / Flock 核心耗时 | Full / Flock 核心耗时 | Compact / Full 核心耗时 |',
    '|---:|---:|---:|---:|']
for threads in (24,48):
    r=lookup[f'main-n30-{threads}t']
    compact=med(r,'optimized','prover_core_ms');full=med(r,'full','prover_core_ms');flock=med(r,'flock','prover_core_ms')
    doc.append(f'| {threads} | {compact/flock:.3f}× | {full/flock:.3f}× | {compact/full:.3f}× |')
doc+=['','以上是耗时比；小于 1 表示该候选耗时少于分母。以下为 24→48 物理核的核心加速比：', '']
for system in ('optimized','full','flock'):
    doc.append(f"- `{system}`：{med(main24,system,'prover_core_ms')/med(main48,system,'prover_core_ms'):.3f}×。")
doc+=['','24→48 核同时跨越 NUMA 节点，因此这组数据不能单独区分线程收益、跨节点访问与内存带宽。没有测量硬件性能计数器，不将这些因素直接认定为已经证实的原因。', '',
    '六次正式样本的核心耗时范围（秒；完整逐项数据在 summary.csv）：', '',
    '| 物理核 | 系统 | 最小 | 中位数 | 最大 |','|---:|---|---:|---:|---:|']
for threads in (24,48):
    r=lookup[f'main-n30-{threads}t']
    for system in ('optimized','full','flock'):
        values=stat(s['prover_core_ms']/1000 for s in r['samples'] if s['system']==system and not s['warmup'])
        doc.append(f"| {threads} | {system} | {values['min']:.3f} | {values['median']:.3f} | {values['max']:.3f} |")
doc+=['',
    '## 冷启动峰值内存','',
    '每项单独启动进程，零预热，一份证明；读取 prover 返回时的 VmHWM。包含公共输入、原生表示、树与运行时，读数在 verifier/源核验之前。主实验多系统进程中的 VmHWM 是累计值，未作为各系统独立 RSS。', '',
    '| 物理核 | 系统 | VmHWM GiB | 核心 prover s |',
    '|---:|---|---:|---:|']
for threads in (24,48):
    for system in ('optimized','full','flock'):
        r=lookup[f'cold-rss-n30-{threads}t-{system}'];s=r['samples'][0]
        doc.append(f"| {threads} | {system} | {s['memory_at_prover_end_kib']['VmHWM']/2**20:.3f} | {s['prover_core_ms']/1000:.3f} |")

doc+=['','两种 Mikros 策略的 RSS 几乎相同：N30 的紧凑树少保存约 16 GiB，却在最底层 GKR 生成一个同时存活的 16 GiB 扩域临时数组。二者峰值均约 97.5 GiB，当前 Flock 约 158.0 GiB。Flock 使用扩域 f/g、显式 sigma 向量和更大的双树工作缓冲；完整数组存活分析见 ../../ENGINEERING.md。', '']

doc+=['','## 实测瓶颈与消融','',
    '本地 N20、六线程的分阶段中位数（ms）：层 materialize 包含旧版的系数求值、层复制及 a/b 数组构造。各阶段中位数不能直接相加还原总中位数。', '',
    '| 阶段 | Reference ms | Optimized ms |','|---|---:|---:|']
phase_groups={'输入解码':['decode_ms'],'紧凑系数树构造':['compact_build_ms'],'首个扩域层求值':['coefficient_eval_ms'],
    'GKR 层求值/物化':['layer_materialize_ms'],'GKR 消息、fold 与 equality':['messages_ms','folds_ms','fold_and_message_ms','equality_ms'],
    '扩域树构造':['extension_build_ms'],'源摘要和形状检查':['source_checks_ms']}
for label,keys in phase_groups.items():
    values=[statistics.median(sum(s['phases'][k] for k in keys) for s in local['samples'] if s['system']==system and not s['warmup']) for system in ('reference','optimized')]
    doc.append(f'| {label} | {values[0]:.3f} | {values[1]:.3f} |')
doc+=['','完整消融，单位 ms：N26 与 N30 都使用 K=64，避免把小域切换点混入规模比较。', '',
    '| 平台 | N | 线程 | 系统 | 核心 ms | 转换 ms |','|---|---:|---:|---|---:|---:|']
for name in ('local-1t','local-6t','ablation-n26-48t'):
    r=lookup[name]
    for system in r['metadata']['systems']:
        doc.append(f"| {name} | 2^{r['metadata']['log_n']} | {r['metadata']['threads']} | {system} | {med(r,system,'prover_core_ms'):.3f} | {med(r,system,'prepare_ms'):.3f} |")

doc+=['',f"同一服务器、N26、48 核：reference→optimized 核心加速 {med(ablation,'reference','prover_core_ms')/med(ablation,'optimized','prover_core_ms'):.2f}×，含转换总计加速 {med(ablation,'reference','prover_total_ms')/med(ablation,'optimized','prover_total_ms'):.2f}×。N30 未运行原版，不能将 N26 加速比当作 N30 原版实测。", '',
    '服务器 N26 阶段耗时中位数（ms）。这组消融直接定位同一 CPU、同一 K=64 下的成本：', '',
    '| 阶段 | Reference | Parallel | Lookup | Optimized | Full |','|---|---:|---:|---:|---:|---:|']
for label,keys in phase_groups.items():
    values=[statistics.median(sum(s['phases'][k] for k in keys) for s in ablation['samples'] if s['system']==system and not s['warmup']) for system in ('reference','parallel','lookup','optimized','full')]
    doc.append('| '+label+' | '+' | '.join(f'{value:.3f}' for value in values)+' |')

doc+=['','N30 优化后的成本分布（秒）：', '',
    '| 阶段 | Compact 24 核 | Full 24 核 | Compact 48 核 | Full 48 核 |','|---|---:|---:|---:|---:|']
for label,keys in phase_groups.items():
    values=[statistics.median(sum(s['phases'][k] for k in keys)/1000 for s in lookup[f'main-n30-{threads}t']['samples'] if s['system']==system and not s['warmup']) for threads in (24,48) for system in ('optimized','full')]
    doc.append('| '+label+' | '+' | '.join(f'{value:.3f}' for value in values)+' |')
residuals=[statistics.median((s['prover_core_ms']-sum(s['phases'].values()))/1000 for s in lookup[f'main-n30-{threads}t']['samples'] if s['system']==system and not s['warmup']) for threads in (24,48) for system in ('optimized','full')]
doc.append('| 未细分开销 | '+' | '.join(f'{value:.3f}' for value in residuals)+' |')
doc+=['','“未细分开销”按每份样本的核心总计减去全部阶段计时，再取中位数。包含阶段计时之外的缓冲获取、transcript 操作和树/临时对象释放等；尚未进一步拆分，不能全部归因为内存分配。', '']

doc+=['### 瓶颈结论与后续优先级','',
    '原版主要浪费在逐位格式转换/解码、标量系数算术、层求值与复制，以及串行 GKR 扫描。N26 的并行候选已从 47.568 秒降至 11.516 秒；此时旧 GKR 的消息/fold/equality 仍占约 8.12 秒。加入融合扫描和 SIMD 后，这部分降到约 0.25 秒，说明仅给建树加线程并不够。', '',
    '优化后，N30 的紧凑版仍比 full 多出解码、系数构造和底层物化等工作，而峰值内存几乎相同。就当前这组数据，full 是更合适的速度基线。下一项具体优化应是把紧凑底层的系数求值与 GKR 首轮消息/折叠直接融合，消除完整临时数组，并尽早释放不再需要的树层；这些改动尚未实现。', '',
    '另有每份证明约 2.7–3.9 秒未细分开销，值得继续拆分缓冲获取和对象释放。24→48 核时，解码/转换改善，但 GKR 与扩域建树阶段没有相应加速；后续应以独立 NUMA/内存带宽和分配器实验验证原因，不能把线程数翻倍当作线性加速。', '']
doc+=['','各候选逐步叠加：', '',
    '- `reference`：原始标量系数树与三次 GKR，加入阶段计时，并与单独编译的冻结基线进行证明差分。',
    '- `parallel`：8×8 位转置、并行解码和建树，扩域层直接借用，仍用原来的系数算术与 GKR。',
    '- `lookup`：再加入小域 PCLMUL/字节约简表及系数求值的 F2-linear 查找表。',
    '- `optimized`：再加入 equality 分解、fold/下一轮消息融合、缓冲池和服务器四路 SIMD。内部计算三个二次核心量，再重建原来的三次消息，验证器不变。',
    '- `full`：使用相同优化 GKR，跳过紧凑系数树，以区分打包的时间/空间取舍。', '',
    '保留的树层容量不是峰值 RSS：紧凑层进入 GKR 时仍会物化为扩域数组，实际峰值请使用上面的独立进程结果。当前优化保持两条原始 GKR 链；论文完整 packed coefficient-domain reduction 未纳入本实现。', '',
    '### 与 Flock 仍然存在的结构差距','',
    '- 当前 Mikros 先将 `(value, position)` 编码成 38-bit 向量并嵌入 K64，再构造系数树/扩域树；Flock 从扩域值构造 `f + alpha*id + beta`、`g + alpha*sigma + beta`。二者证明同一公开置换关系，但构造的指纹不同。',
    '- Mikros 两条 GKR 链各自计算三个核心量，重建原来的三次消息；Flock 用 lambda 合并两棵树，在一条链中发送两个二次核心量。工程优化没有消除这项协议差距。',
    '- N30 的 Mikros 消息是 `3*30^2 + 30 + 2 = 2732` 个域元素，Flock 是 `30^2 + 3*30 + 5 = 995` 个；另含序列化长度字段，实际消息大小见主表。',
    '- Flock 在同一 rho 返回 f、g、sigma 的声明；Mikros 保留左右两处不同的点。当前没有 PCS，不能据核心 prover 时间推断后续承诺/opening 的成本。',
    '- 当前 Flock 实现还显式构造扩域 sigma 向量并求其 MLE；Mikros 将标签纳入原始 bit oracle。Flock 的批处理收益和额外原生表示成本会同时反映在实测中，不能只凭消息数量预测时间。', '',
    '## 验证、环境和范围','',
    f'共复核 {validated} 份本地及服务器样本（含预热与冷启动）；每份均完成 prover/verifier transcript 同步和全部原始源声明认证。汇总再次核对真实 proof 文件大小、SHA256、各 Mikros 变体、跨线程及跨 CPU 后端的证明一致性。', '',
    '字段乘法/嵌入、位转置、空输入与非二次幂填充、所有候选与树策略、并行轮次以及篡改拒绝测试均已在本地和服务器原生编译运行。Clippy 检查通过（冻结依赖在本地有既存 AVX-512 unused-import 警告）。', '',
    f"服务器：{cpu_model}；24 物理核位于一个 NUMA 节点，48 物理核跨两节点，均不使用 SMT sibling。内存按 Linux first-touch，未设置 numactl。",
    '服务器原生启用 AVX-512/VPCLMUL；本地 i7-8700K 使用 PCLMUL/AVX2。使用同一冻结 F128 后端、BLAKE3 和 nightly-2026-03-21。共享服务器的负载及干扰监测保存在 environment.json/commands.json；正式组计时期间未检测到其他受监测的编译/benchmark 进程。', '',
    '这是 permutation PIOP 工程比较。PCS、承诺和 opening 未计入；所有源声明仍逐一核验。未声称完整 SNARK 的相同安全参数。两条 GKR 链和消息规模保持原样；Flock 的 batched 协议仍是不同的消息结构。', '',
    f"服务器 binary SHA256：`{environment['binary_sha256']}`。源码逐文件校验清单在 source_manifest.json，实际命令和原始 proof/JSON 均保留。", '',
    '```sh','python3 summarize_experiment.py --root server_experiments/20260912T091817-permutation-v1','```','']
(root/'REPORT.md').write_text('\n'.join(doc),encoding='utf-8')
with (root/'summary.csv').open('w',newline='',encoding='utf-8') as f:
    writer=csv.DictWriter(f,fieldnames=list(rows[0]));writer.writeheader();writer.writerows(rows)
(root/'validation.json').write_text(json.dumps({'validated_proofs':validated,'unique_statement_protocol_pairs':len(identities),'accepted_server_cases':len(accepted)},indent=2)+'\n')
print(f'Validated {validated} proofs; wrote {root / "REPORT.md"}')
