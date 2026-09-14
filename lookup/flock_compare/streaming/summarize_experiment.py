"""Validate real proof artifacts and summarize the paired streaming experiment."""
import argparse
import csv
import hashlib
import json
import math
from pathlib import Path
from statistics import median

parser=argparse.ArgumentParser()
parser.add_argument('--root',type=Path,required=True)
root=parser.parse_args().root.resolve()
here=Path(__file__).resolve().parent
def read(p): return json.loads(p.read_text(encoding='utf-8'))
results=root/'results'
assert read(results/'status.json')['stage']=='complete'
accepted=read(results/'accepted.json'); assert len(accepted)==14
commands=read(results/'commands.json'); assert all(c['returncode']==0 for c in commands)
environment=read(results/'environment.json')
assert environment['source_manifest']==read(root/'source_manifest.json')
assert hashlib.sha256((root/'source.tar.gz').read_bytes()).hexdigest()==read(root/'package.json')['sha256']
reports=[('local-6t',here/'results/local-v2-final.json',read(here/'results/local-v2-final.json'))]
for case in accepted:
    p=results/case['file']; report=read(p); meta=report['metadata']
    command=next(c for c in commands if c.get('case')==case['case'] and c.get('accepted'))
    assert not command['interference']
    assert {'avx512f','vpclmulqdq'}.issubset(meta['target_features'])
    assert command['argv'][:2]==['taskset','-c']
    assert list(map(int,command['argv'][2].split(',')))==environment['affinities'][str(meta['threads'])]
    reports.append((case['case'],p,report))

metrics=('prepare_ms','prover_core_ms','prover_total_ms','verifier_ms','source_check_ms')
identities={}; validated=0; rows=[]
for name,path,report in reports:
    meta=report['metadata']; systems=meta['systems']; n=meta['log_n']
    assert len(report['samples'])==(meta['runs']+meta['warmups'])*len(systems)
    for run in range(meta['runs']+meta['warmups']):
        group=[s for s in report['samples'] if s['run']==run]
        assert {s['system'] for s in group}==set(systems) and len(group)==len(systems)
        assert {s['order'] for s in group}==set(range(len(systems)))
        assert len({s['input_digest'] for s in group})==1
        for s in group:
            assert s['verified'] and s['warmup']==(run<meta['warmups'])
            assert s['log_n']==n and s['tuple_bits']==n+meta['width']
            assert all(math.isfinite(s[k]) and s[k]>=0 for k in metrics)
            assert s['prover_total_ms']+0.001>=s['prover_core_ms']+s['prepare_ms']
            data=(path.with_suffix('.proofs')/f"run{run}-{s['system']}.bin").read_bytes()
            fields=n*n+3*n+5 if s['system']=='flock' else 3*n*n+n+2
            overhead=16+8*n if s['system']=='flock' else 16+16*n
            assert len(data)==s['proof_bytes']==16*fields+overhead
            key=(n,meta['width'],meta['seed']+run,'flock' if s['system']=='flock' else 'mikros')
            identity=(s['input_digest'],s['proof_digest'],hashlib.sha256(data).hexdigest())
            assert identities.setdefault(key,identity)==identity,(name,s['system'],run)
            validated+=1
    for system in systems:
        samples=[s for s in report['samples'] if s['system']==system and not s['warmup']]
        if meta['runs']==len(systems):
            assert [sum(s['order']==i for s in samples) for i in range(len(systems))]==[1]*len(systems)
        row={'case':name,'system':system,'log_n':n,'threads':meta['threads'],'runs':meta['runs']}
        for metric in metrics:
            values=[s[metric] for s in samples]
            row.update({metric+'_median':median(values),metric+'_min':min(values),metric+'_max':max(values)})
        rows.append(row)
assert validated==166

# Also check correspondence with already validated v1 server proof artifacts.
old=here.parent/'optimized/server_experiments/20260912T091817-permutation-v1'
cross_version=0
for case in read(old/'results/accepted.json'):
    path=old/'results'/case['file']; report=read(path); meta=report['metadata']
    for s in report['samples']:
        key=(meta['log_n'],meta['width'],meta['seed']+s['run'],'flock' if s['system']=='flock' else 'mikros')
        if key not in identities: continue
        data=(path.with_suffix('.proofs')/f"run{s['run']}-{s['system']}.bin").read_bytes()
        assert identities[key]==(s['input_digest'],s['proof_digest'],hashlib.sha256(data).hexdigest())
        cross_version+=1

lookup={name:r for name,p,r in reports}
labels={'previous':'v1 紧凑','previous_full':'v1 全扩域','release':'v2 提前释放','optimized':'v2 流式紧凑','full':'v2 全扩域','flock':'Flock'}
systems=('previous','previous_full','optimized','full','flock')
def samples(r,system):return [s for s in r['samples'] if s['system']==system and not s['warmup']]
def med(r,system,metric):return median(s[metric] for s in samples(r,system))
def paired(r,before,after,metric):
    a={s['run']:s[metric] for s in samples(r,before)}
    return median(a[s['run']]/s[metric] for s in samples(r,after))
def rss(threads,system):return lookup[f'cold-rss-n30-{threads}t-{system}']['samples'][0]['memory_at_prover_end_kib']['VmHWM']/2**20

r48=lookup['main-n30-48t']
doc=['# Mikros permutation 第二轮：流式求值与提前释放','',
    '本轮将 v1 紧凑/全扩域实现与 v2 候选独立编译，在同一进程内轮换执行。证明和 transcript 保持不变；目标是消除紧凑层完整物化并降低树层同时存活的内存。', '',
    f"实测 N=2^30、48 个物理核：紧凑版核心耗时中位数从 {med(r48,'previous','prover_core_ms')/1000:.3f} s 降至 {med(r48,'optimized','prover_core_ms')/1000:.3f} s，含输入转换从 {med(r48,'previous','prover_total_ms')/1000:.3f} s 降至 {med(r48,'optimized','prover_total_ms')/1000:.3f} s；峰值内存从 {rss(48,'previous'):.2f} GiB 降至 {rss(48,'optimized'):.2f} GiB，减少 {100*(1-rss(48,'optimized')/rss(48,'previous')):.1f}%。", '',
    f"与 Flock 比较，v2 紧凑版含转换耗时为 {med(r48,'optimized','prover_total_ms')/1000:.3f} s，Flock 为 {med(r48,'flock','prover_total_ms')/1000:.3f} s，二者接近；不能将这一微小差值视为稳定的速度胜出。峰值内存比 Flock 的 {rss(48,'flock'):.2f} GiB 低 {100*(1-rss(48,'optimized')/rss(48,'flock')):.1f}%。源核验另列，不包含在上述耗时内。全扩域版本主要获得内存收益，核心耗时略有回退。", '',
    '## N=2^30 主结果','',
    'N 表示置换项数，值为 8 bit，加 30-bit 位置标签共 38 个逻辑位，使用 K64。每个线程配置一次预热、五次正式配对输入，五种循环顺序令每个系统在每个顺序位置恰好出现一次。表中时间为正式样本中位数；峰值内存来自另行启动的单系统冷进程。', '',
    '| 物理核 | 系统 | 核心 s | 含转换 s | 源核验 s | 冷进程峰值 GiB |',
    '|---:|---|---:|---:|---:|---:|']
for threads in (24,48):
    r=lookup[f'main-n30-{threads}t']
    for system in systems:
        doc.append(f"| {threads} | {labels[system]} | {med(r,system,'prover_core_ms')/1000:.3f} | {med(r,system,'prover_total_ms')/1000:.3f} | {med(r,system,'source_check_ms')/1000:.3f} | {rss(threads,system):.3f} |")
doc+=['','## 同输入的改进幅度','',
    '加速比是逐份相同输入的“旧耗时 / 新耗时”再取中位数；大于 1 表示加速。内存为独立冷进程对比。', '',
    '| 物理核 | 更新 | 核心加速 | 含转换加速 | 峰值内存减少 |','|---:|---|---:|---:|---:|']
for threads in (24,48):
    r=lookup[f'main-n30-{threads}t']
    for before,after in (('previous','optimized'),('previous_full','full')):
        doc.append(f"| {threads} | {labels[before]} → {labels[after]} | {paired(r,before,after,'prover_core_ms'):.3f}× | {paired(r,before,after,'prover_total_ms'):.3f}× | {100*(1-rss(threads,after)/rss(threads,before)):.2f}% |")

doc+=['','核心耗时的最小/中位数/最大值（秒）；完整样本和其他指标在 summary.csv 与原始 JSON：', '',
    '| 物理核 | 系统 | 最小 | 中位数 | 最大 |','|---:|---|---:|---:|---:|']
for threads in (24,48):
    r=lookup[f'main-n30-{threads}t']
    for system in systems:
        values=[s['prover_core_ms']/1000 for s in samples(r,system)]
        doc.append(f'| {threads} | {labels[system]} | {min(values):.3f} | {median(values):.3f} | {max(values):.3f} |')

doc+=['','## N26 消融','',
    'N26 同样使用 K64。六个系统一次预热、六次循环配对，隔离提前释放和流式读取的影响；该组不再运行最初的标量原版。', '',
    '| 系统 | 核心 ms | 层物化 ms | 层释放 ms | 缓冲获取 ms | GKR 消息及折叠 ms |',
    '|---|---:|---:|---:|---:|---:|']
ablation=lookup['ablation-n26-48t']
def phase(r,system,keys): return median(sum(s['phases'].get(k,0) for k in keys) for s in samples(r,system))
gkr=('messages_ms','fold_and_message_ms','folds_ms','equality_ms')
for system in ('previous','release','optimized','previous_full','full'):
    release=f"{phase(ablation,system,['layer_release_ms']):.3f}" if system not in ('previous','previous_full') else '未单列'
    scratch=f"{phase(ablation,system,['scratch_setup_ms']):.3f}" if system not in ('previous','previous_full') else '未单列'
    doc.append(f"| {labels[system]} | {med(ablation,system,'prover_core_ms'):.3f} | {phase(ablation,system,['layer_materialize_ms']):.3f} | {release} | {scratch} | {phase(ablation,system,gkr):.3f} |")
doc+=['','N30 / 48 核候选阶段中位数（秒）：', '',
    '| 阶段 | v2 流式紧凑 | v2 全扩域 |','|---|---:|---:|']
groups={'源摘要/形状':['source_checks_ms'],'解码':['decode_ms'],'紧凑建树':['compact_build_ms'],'初始求值':['coefficient_eval_ms'],'扩域建树':['extension_build_ms'],'整层物化':['layer_materialize_ms'],'GKR 消息/折叠':gkr,'层释放':['layer_release_ms'],'缓冲获取':['scratch_setup_ms']}
for label,keys in groups.items():
    values=[phase(lookup['main-n30-48t'],s,keys)/1000 for s in ('optimized','full')]
    doc.append(f'| {label} | {values[0]:.3f} | {values[1]:.3f} |')
residual=[median((s['prover_core_ms']-sum(s['phases'].values()))/1000 for s in samples(lookup['main-n30-48t'],system)) for system in ('optimized','full')]
doc.append(f'| 未细分开销 | {residual[0]:.3f} | {residual[1]:.3f} |')
doc+=['','阶段中位数不能直接相加还原总中位数。流式紧凑层的求值计入 GKR 消息/折叠；“整层物化”下降不代表所有求值工作消失。', '',
    '当前仍需关注的开销是 GKR 消息/折叠、同步释放树层及输入解码。48 核紧凑版的阶段中位数分别约为 3.183 s、2.575 s、2.238 s。缓冲获取只有微秒量级，未细分开销约 1 ms；因此不能再把上一轮数秒的未细分时间归因于缓冲池获取。N26 消融中，仅提前释放的核心耗时比 v1 更长，组合优化才取得时间收益；该实验未将流式读取与叶子查表精简各自的贡献进一步分离。', '',
    '## 实现与取舍','',
    '- 按至多 16 KiB 块求值紧凑层，在首轮消息与折叠扫描中立即消费；取消 N30 的完整 16 GiB 扩域临时数组。挑战到来前后的两次读取仍需分别求值。',
    '- 每个树层用完后同步释放，并计入核心 prover；没有后台释放或将清理移出计时。新增 layer_release_ms 与 scratch_setup_ms，拆分上一轮未细分开销。',
    '- 用固定字宽读取紧凑系数；分开流式调度与 dense SIMD 热循环。对叶子只预计算实际逻辑位：38-bit/K64 从 8 个字节表减至 5 个。',
    '- full 也提前释放层，并精简叶子查表。它提供不同的时间/空间取舍，结论应同时看总耗时与峰值内存。',
    '- 仍是原来的两条三次 GKR 链；没有实施新的 batched 协议或论文完整 packed coefficient-domain reduction。Mikros 消息仍为 44,208 bytes，Flock 为 16,176 bytes。', '',
    '## 验证、环境与范围','',
    f'本轮 {validated} 份本地/服务器基准证明（含预热和冷进程）全部核验；另与上一轮 {cross_version} 份重叠输入的原始 proof 文件核对 SHA256 和 transcript digest。每份运行重放 verifier 并认证全部原始源声明，包括 Flock sigma。', '',
    '本地差分测试和 Clippy 通过，服务器原生 AVX-512/VPCLMUL 差分测试通过。上一版 Rust 源码逐文件匹配 v1 冻结清单；新旧版本在一个 binary 中独立编译。', '',
    '服务器为双路 Xeon Gold 6342。24 核限制在一个 NUMA 节点，48 核跨两个节点，不使用 SMT sibling；同一 Rust nightly-2026-03-21、native 编译及 F128 后端。共享主机，在已接受实验期间未检测到其他受监测的 benchmark/build 进程。未测硬件性能计数器，不将线程扩展的原因归于某一个硬件因素。', '',
    '原生输入转换计入 total，公共输入生成与公共摘要排除。源核验另列；不是 PCS 验证。本实验不含 PCS/承诺/opening 或 grinding，未声称完整 SNARK 的同等安全参数。计时进程共享池，RSS 则使用单系统冷进程，避免将累计 VmHWM 当作单系统内存。', '',
    f"binary SHA256: `{environment['binary_sha256']}`。源包、逐文件校验清单、命令、日志、真实证明及原始样本均保留。", '',
    '```sh','python3 summarize_experiment.py --root server_experiments/20260912T103800-permutation-v2','```','']
(root/'REPORT.md').write_text('\n'.join(doc),encoding='utf-8')
with (root/'summary.csv').open('w',newline='',encoding='utf-8') as f:
    writer=csv.DictWriter(f,fieldnames=list(rows[0]));writer.writeheader();writer.writerows(rows)
(root/'validation.json').write_text(json.dumps({'validated_proofs':validated,'cross_version_proofs':cross_version,'distinct_statement_protocol_pairs':len(identities),'accepted_server_cases':len(accepted)},indent=2)+'\n',encoding='utf-8')
print(f'Validated {validated} current proofs and {cross_version} v1 overlaps; wrote {root / "REPORT.md"}')
