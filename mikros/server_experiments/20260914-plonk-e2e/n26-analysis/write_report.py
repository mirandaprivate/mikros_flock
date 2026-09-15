"""Render the final N26 comparison from validated summaries and raw diagnostics."""
import argparse
import json
from pathlib import Path
import re

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--selected', type=Path, required=True)
parser.add_argument('--all-on', type=Path, required=True)
parser.add_argument('--evidence', type=Path, required=True)
parser.add_argument('--sparse-off', type=Path, required=True)
parser.add_argument('--sparse-jit-off', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
selected = json.loads(args.selected.read_text())
all_on = json.loads(args.all_on.read_text())
sparse_off = json.loads(args.sparse_off.read_text())
sparse_jit_off = json.loads(args.sparse_jit_off.read_text())
assert selected['optimized_binary_sha256'] == all_on['optimized_binary_sha256']
assert selected['optimized_allocator'] == all_on['optimized_allocator'] == 'system'
for data in (selected, all_on, sparse_off, sparse_jit_off):
    assert data['optimized_binary_sha256'] == selected['optimized_binary_sha256']
    assert data['optimized_allocator'] == 'system'
    for entry in data['cases']:
        assert entry['unchanged_proof_and_commitments']
        assert entry['baseline']['samples'] == entry['optimized']['samples'] == 3
pilot = args.evidence / 'native-pilot14'
assert json.loads((pilot / 'status.json').read_text())['state'] == 'complete'
test_counts = {}
for path in pilot.glob('test-*.stdout.jsonl'):
    test_counts[path.name] = sum(map(int, re.findall(r'test result: ok\. (\d+) passed;', path.read_text())))
assert len(test_counts) == 17 and all(test_counts.values())
assert test_counts['test-mikros.stdout.jsonl'] == 100
assert test_counts['test-e2e-fixture.stdout.jsonl'] == 17
for mode in ('single', 'multiple'):
    for system in ('mikros', 'flock'):
        rows = [json.loads(x) for x in (pilot / f'n14-{mode}-{system}.stdout.jsonl').read_text().splitlines()]
        samples = [x for x in rows if x['type'] == 'sample']
        assert len(samples) == 1 and samples[0]['verified'] and samples[0]['transcript_match']


def case(data, mode, system):
    return next(x for x in data['cases'] if x['mode'] == mode and x['system'] == system)


def seconds(data, mode, system, field='prover_ms', variant='optimized'):
    return case(data, mode, system)[variant]['times_ms'][field]['median'] / 1000


def profile(folder):
    directory = args.evidence / folder
    status = json.loads((directory / 'status.json').read_text())
    config = json.loads((directory / 'config.json').read_text())
    receipt = json.loads((directory / 'build-receipt.json').read_text())
    assert status['state'] == 'complete'
    assert status['source_hashes_verified_before_and_after'] and status['binary_hashes_verified_before_and_after']
    assert receipt['binary_sha256'] == selected['optimized_binary_sha256']
    assert receipt == json.loads((pilot / 'build-receipt.json').read_text())
    assert config['levels'] == [26] and config['threads'] == 64 and config['seed'] == 97
    assert config['diagnostic_trace'] and config['allocator_policy'] == 'system'
    assert config['disabled_optimizations'] == (['tree-pool'] if folder == 'profile-pool-off' else [])
    samples = [json.loads(x) for x in (directory / 'n26-multiple-mikros.stdout.jsonl').read_text().splitlines()
               if json.loads(x)['type'] == 'sample']
    assert len(samples) == 1 and samples[0]['verified'] and samples[0]['transcript_match']
    assert samples[0]['proof_blake3'] == case(selected, 'multiple', 'mikros')['proof_blake3']
    path = directory / 'n26-multiple-mikros.stderr.log'
    rows = [json.loads(x.split('tree_profile=', 1)[1])
            for x in path.read_text().splitlines() if 'tree_profile=' in x]
    assert len(rows) == 1
    return rows[0]


def rss(paths, mode, system):
    values = []
    for directory in paths:
        text = (Path(directory) / f'n26-{mode}-{system}.resource.txt').read_text()
        values.append(int(re.search(r'Maximum resident set size \(kbytes\):\s*(\d+)', text)[1]) / 2**20)
        assert int(re.search(r'Major \(requiring I/O\) page faults:\s*(\d+)', text)[1]) == 0
    return f'{min(values):.2f}–{max(values):.2f}'


lines = [
    '# N26 Mikros / FLOCK E2E 工程优化实测', '',
    '日期：2026-09-15。服务器：192.168.219.251，双 Xeon Gold 6342；固定 64 个逻辑 CPU，两个工作线程池均为 64。',
    '两者使用同一 FLOCK 原生 PCS、相同 source 数据和承诺哈希，启用 GFNI、AVX-512、VPCLMUL；编译参数为 `-C target-cpu=native`。', '',
    '**规模与口径：** single 为一个 2^26 门电路；multiple 为同一 wiring 的 128 个独立输入，每个都是 2^26 门，共 2^33 个 binary constraints。',
    '电路为连接的交替 AND/XOR 电路，seed=97。静态输入准备和初次 public commitment 为 offline；online 包括 witness commitment、所有在线转换、PIOP、重新承诺、两个 PCS opening、释放和序列化。验证单独计时。',
    '表中为三次独立进程的中位数，单位秒；各阶段中位数不必相加等于总中位数。倍数定义为 FLOCK / Mikros。', '',
    '## 总时间与选择策略', '',
    '| 模式 | 本轮优化前 Mikros | 本轮优化前 FLOCK | 最终 Mikros | 最终 FLOCK | 最终 F/M |',
    '|---|---:|---:|---:|---:|---:|',
]
for mode in ('single', 'multiple'):
    values = [seconds(selected, mode, s, variant='baseline') for s in ('mikros', 'flock')]
    values += [seconds(selected, mode, s) for s in ('mikros', 'flock')]
    lines.append('| ' + mode + ' | ' + ' | '.join(f'{x:.4f}' for x in values) + f' | {values[-1]/values[-2]:.3f} |')
lines += ['', f"筛选后禁用项：`{json.dumps(selected['disabled_optimizations'], ensure_ascii=False)}`。同一模式双方使用完全相同的开关。",
          '系统 allocator 保持默认；glibc-reuse 仅保留为负面消融证据，不纳入最终配置。它在旧版单次对照中使 single 的 F/M 从 1.693 降至 1.421，multiple 从 1.140 降至 1.023，同时增加 RSS。',
          '全部候选开启的结果并列保留，避免把关闭双方优化后产生的倍数变化称为 Mikros 自身提速。', '']
lines += ['multiple 的组合复测，每项同样为三次独立进程：', '',
          '| 配置 | Mikros 秒 | FLOCK 秒 | F/M |', '|---|---:|---:|---:|']
for label, data in [('现有内核全部开启，系统 allocator', all_on),
                    ('双方禁用 sparse-bitbanks', sparse_off),
                    ('双方禁用 sparse-bitbanks 和 jit-fused', sparse_jit_off)]:
    m, f = (seconds(data, 'multiple', s) for s in ('mikros', 'flock'))
    lines.append(f'| {label} | {m:.4f} | {f:.4f} | {f/m:.3f} |')
lines += ['',
          '最终保留双方对称的现有共享内核；新加入的树缓冲池只作用于 Mikros。上述回退没有改善本组 E2E 中位数，因此没有作为提速改动纳入。复测中 commit/gate 等其他阶段也有波动，不能把几个百分点的倍数变化解释为已证明的因果优势。',
          '稀疏剪裁按 fold 数量看，single public 为 Mikros 768→448、FLOCK 128→128；multiple witness 为 Mikros 20→16、FLOCK 20→12。这说明单项结构收益随模式变化，不能只凭次数或一次计时决定完整 E2E 的收益。',
          '全局 allocator 复用更有利于 FLOCK 的明显负面结果保留，不纳入最终配置；后续投入优先放在 Mikros 独有热点。', '']
for mode in ('single', 'multiple'):
    lines += [f'## {mode}：筛选后的阶段分解', '',
              '| 阶段 | Mikros 秒 | FLOCK 秒 | F/M |', '|---|---:|---:|---:|']
    for key, label in [('gates_ms', 'Gate PIOP'), ('copy_ms', 'Copy / permutation PIOP'),
                       ('witness_pcs_open_ms', 'Witness PCS opening'), ('public_pcs_open_ms', 'Public PCS opening'),
                       ('commit_and_recommit_ms', 'Commit + 两次 recommit'), ('release_ms', '阶段边界释放'),
                       ('serialization_ms', '序列化'), ('prover_ms', 'Online 总时间')]:
        m, f = (seconds(selected, mode, s, key) for s in ('mikros', 'flock'))
        lines.append(f'| {label} | {m:.4f} | {f:.4f} | {f/m:.3f} |')
    paths = selected['multiple_selected_directories'] if mode == 'multiple' and selected['multiple_selected_directories'] else selected['optimized_directories']
    for system in ('mikros', 'flock'):
        c = case(selected, mode, system)
        total = c['optimized']['times_ms']['prover_ms']
        lines += ['', f"{system} 总时间 min/median/max：{total['min']/1000:.4f} / {total['median']/1000:.4f} / {total['max']/1000:.4f} 秒；整个进程 peak RSS 范围 {rss(paths, mode, system)} GiB。"]
    lines.append('')
on, off = profile('profile-n26'), profile('profile-pool-off')
lines += ['## 已实现的工程改动', '',
          '- PCS 按相同行点合并计算，复用 column bitbanks；两路联合折叠减少重复源读取。',
          '- 常量 source 列只在线扫描确认一次，随后为多个行点复用；single public 实测 63/128 列恒定。检测和构表均计入 PCS opening。',
          '- 普通 MLE 终端查询使用 F128 点积；三条 public labels 共享一次行折叠。',
          '- Mikros multiple 复用在线生成的叶子缩放缓存；融合本轮树折叠和下一轮消息；加入一次证明内的精确容量缓冲池。',
          '- JIT 首轮折叠融合和稀疏 bitbank 剪裁均有独立开关；是否纳入按上面同模式对称配置执行，原始消融不删除。', '',
          '## Mikros 专用乘积树复用', '',
          '同一二进制、相同 seed 且证明字节哈希一致的单次诊断对照如下；诊断样本不参与上面的主中位数。池在一次 prove_pair 内复用精确容量，跨层和两棵顺序树共享。', '',
          '| 指标 | 池关闭 | 池开启 |', '|---|---:|---:|']
for key in ('construction_ms', 'reduction_ms', 'release_ms'):
    lines.append(f'| {key}，秒 | {off[key]/1000:.4f} | {on[key]/1000:.4f} |')
lines += [f"| field buffer 容量，GiB | {off['peak_field_buffer_bytes']/2**30:.4f} | {on['peak_field_buffer_bytes']/2**30:.4f} |", '',
          '`release_ms` 是 tree reduction 的内部子项，已包含在 copy_ms 内，不额外相加。所有缓存均在 prove_pair 返回前同步释放；不存在把最终释放移到 online 计时外的做法。',
          'N26 树长 L=2^28；池容量上限为 (3L/2−2) 个 F128，约 6 GiB；原版为 (L−1) 个 F128，约 4 GiB。默认仅用于 log_len≤28，未测量的 N30 保持原来的树分配策略。', '',
          '## 当前瓶颈与 2 倍目标', '']
for mode in ('single', 'multiple'):
    m, f = (seconds(selected, mode, s) for s in ('mikros', 'flock'))
    residual = sum(seconds(selected, mode, 'mikros', k) for k in
                   ('witness_pcs_open_ms', 'public_pcs_open_ms', 'commit_and_recommit_ms', 'release_ms'))
    lines.append(f'{mode}：当前 Mikros {m:.3f} 秒；达到当前 FLOCK 的 2 倍速度需要不超过 {f/2:.3f} 秒。Mikros 的 PCS opening、commit/recommit 和阶段边界释放的阶段中位数合计约 {residual:.3f} 秒。')
    pm, pf = (sum(seconds(selected, mode, s, k) for k in ('gates_ms', 'copy_ms')) for s in ('mikros', 'flock'))
    lines.append(f'两项 PIOP 的阶段中位数合计为 Mikros {pm:.3f} 秒、FLOCK {pf:.3f} 秒，约 {pf/pm:.3f} 倍；这不是 E2E 时间。')
lines += ['',
          '上述剩余成本只说明“其余阶段固定时，继续压缩 gate/copy 的空间”，不是对未来算法、布局或内存策略的理论下界。当前结果未达到两种模式均约 2 倍，不能用 copy 子阶段的倍数代替 E2E。', '',
          '## PCS 元素数量与 LaTeX', '',
          '当前已经同量、同内容承诺。single：witness 2^21 个 F128 words，public 2^26 个；multiple：witness 2^28 个，public 2^26 个。上述是包含 padding 的源 word 数；双方 rate-1/2 的 L0 编码 word 数另翻倍。',
          'Mikros 的更多终端声明会增加 opening 工作，不会使源承诺数组变长。LaTeX 的虚拟乘积与 copy 指纹/中间树都不增加源承诺。逻辑长度与 native padding 的完整核对见 `mikros/E2E_N26_PCS_LAYOUT.md`。', '',
          '## 验证与原始证据', '',
          '服务器 native 测试共 17 个非空测试目标通过，包括 Mikros lib 100 项、E2E fixture 17 项，以及 PCS/GKR/permutation 目标；四个 N14 完整证明通过。',
          '所有主样本完整验证成功；同一模式 M/F 的 witness/public commitment hash 与 PCS 参数相同。每个系统自身在优化前后 proof hash、proof bytes 和承诺不变；每个样本的 prover/verifier transcript 一致。Mikros 与 FLOCK 的完整证明本来就不同。',
          'RSS 是整个进程（offline、online、verify）的最大值；所列主样本 major page faults 均为 0。当前证据限于本电路与 seed，并未从小样本推断统计显著性。',
          f"最终二进制 SHA-256：`{selected['optimized_binary_sha256']}`。",
          f"筛选结果与逐文件证据 SHA-256：`{args.selected.as_posix()}`。",
          f"全部开启结果：`{args.all_on.as_posix()}`。",
          f"组合复测：`{args.sparse_off.as_posix()}` 和 `{args.sparse_jit_off.as_posix()}`。",
          f"native 测试、资源和 trace：`{args.evidence.as_posix()}`。",
          '逐项 OFF 消融：`mikros/server_experiments/20260914-plonk-e2e/v5/ablation-summary.json`（每项单次，作为筛选；小差异不能当作显著）。',
          '原 N30 根目录 e2e_single_compare_flock.log 和 e2e_multiple_compare_flock.log 保留；本文件只报告 N26。', '',
]
args.output.write_text('\n'.join(lines), encoding='utf-8')
print(f'Wrote {args.output}')
