"""Recompute the report from immutable downloaded server evidence (stdlib only)."""
import hashlib
import json
import re
import statistics as stats
from pathlib import Path

ROOT = Path(__file__).resolve().parent
LOGS = ROOT / "results-e2e"
for name, expected in json.loads((ROOT / "evidence-manifest.json").read_text()).items():
    assert hashlib.sha256((ROOT / name).read_bytes()).hexdigest() == expected, name
assert hashlib.sha256((ROOT / "evidence.tar.gz").read_bytes()).hexdigest() == (ROOT / "evidence.tar.gz.sha256").read_text().split()[0]
status = json.loads((LOGS / "status.json").read_text())
assert status["state"] == "complete" and not status["failed"]
assert all(x["exit_code"] == 0 for x in status["completed"])

def records(name):
    return [json.loads(line) for line in (LOGS / name).read_text().splitlines() if line.strip()]

def samples(rows, system=None):
    result = [x for x in rows if x["record"] == "sample" and not x["warmup"] and (system is None or x["system"] == system)]
    assert len(result) == 3 and sorted(x["iteration"] for x in result) == [1, 2, 3]
    assert all(x["verified"] and x["transcript_match"] for x in result)
    assert len({x["proof_blake3"] for x in result}) == 1
    return result

original = {x["log_leaves"]: x for x in json.loads((LOGS / "summary.json").read_text())}
common = []
production = []
for leaves in (6, 10, 14):
    rows = records(f"common-leaves{leaves}.stdout")
    meta = next(x for x in rows if x["record"] == "metadata")
    assert meta["threads"] == meta["all_core_threads"] == 64
    assert meta["compression_kappa"] == 128 and meta["flock_skip"] == 7
    assert {"avx512f", "avx512bw", "gfni", "vpclmulqdq"} <= set(meta["target_features"])
    result = {"metadata": meta}
    for system, count in (("flock", 11), ("mikros", 20)):
        group = samples(rows, system)
        assert all(x["timings"]["tensor_claims"] == count for x in group)
        medians = {key: stats.median(x["timings"][key] for x in group) for key in group[0]["timings"]}
        for key in ("prover_ms", "commit_ms", "boolean_ms", "wiring_ms", "parallel_piops_ms", "pcs_open_ms"):
            assert medians[key] == original[leaves][system][key]
        result[system] = dict(medians, verify_ms=stats.median(x["verify_ms"] for x in group),
                              proof_bytes=group[0]["proof_bytes"],
                              range_ms=[min(x["timings"]["prover_ms"] for x in group), max(x["timings"]["prover_ms"] for x in group)],
                              other_ms=stats.median(x["timings"]["prover_ms"] - sum(x["timings"][k] for k in ("commit_ms", "parallel_piops_ms", "pcs_open_ms")) for x in group),
                              pcs_fraction=stats.median(x["timings"]["pcs_open_ms"] / x["timings"]["prover_ms"] for x in group))
    result["ratio"] = result["mikros"]["prover_ms"] / result["flock"]["prover_ms"]
    result["both_process_peak_rss_gib"] = int(re.search(r"Maximum resident set size \(kbytes\): (\d+)", (LOGS / f"common-leaves{leaves}.resource.txt").read_text()).group(1)) / 2**20
    common.append(result)
    rows = records(f"production-leaves{leaves}.stdout")
    pmeta = next(x for x in rows if x["record"] == "metadata")
    assert pmeta["threads"] == pmeta["all_core_threads"] == 64
    assert pmeta["native_root_u32"] == meta["native_root_u32"]
    assert pmeta["gate_count"] == meta["sha_compressions"]
    group = samples(rows)
    production.append({"metadata": pmeta, "prover_ms": stats.median(x["prove_with_commit_ms"] for x in group),
                       "range_ms": [min(x["prove_with_commit_ms"] for x in group), max(x["prove_with_commit_ms"] for x in group)],
                       "verify_ms": stats.median(x["verify_ms"] for x in group), "proof_bytes": group[0]["proof_bytes"]})

negative = {x["system"]: x["rejected"] for x in records("validation-m22.stdout") if x["record"] == "negative_tests"}
assert negative == {"flock": 18, "mikros": 28}
tests = sum(int(n) for file in LOGS.glob("test-*.stdout") for n in re.findall(r"test result: ok\. (\d+) passed", file.read_text()))
assert tests == 44
analysis = {"common": common, "production": production, "negative_tests": negative, "native_tests_passed": tests}
(ROOT / "analysis.json").write_text(json.dumps(analysis, indent=2) + "\n")

lines = [
    "# Mikros / Flock 逐 bit wiring E2E：服务器实测",
    "",
    "2026-09-13，64 线程 native 构建。三档完整证明均通过验证。在共同 bit-wiring、κ128、相同 PCS 参数的适配路径中，Mikros 在线 prover 比 Flock 慢 18.4%–28.0%。最大规模的 PIOP 更快，但 opening 前端的额外工作抵消了收益。",
    "",
    "## 共同配置下的在线 prover",
    "",
    "计时包含 commitment、挑战相关转换、Boolean/wiring 两分支、全部 gather、PCS opening 和函数返回时的临时数据释放。排除 offline 输入/系数准备、owned Z 拷贝、验证及序列化。序列化记录完整字节数，未单独记录耗时。每系统每档一次预热、三次正式样本，交替先后顺序。表中时间为中位数，范围为三次正式样本的 min/max。",
    "",
    "| padded witness bits | wiring bits | SHA-256 compression 数 | Flock ms [min, max] | Mikros ms [min, max] | Mikros/Flock |",
    "|---|---|---:|---:|---:|---:|",
]
for r in common:
    m, f, k = r["metadata"], r["flock"], r["mikros"]
    lines.append(f'| 2^{m["logical_bit_log"]} | 2^{m["wiring_bit_log"]} | {m["sha_compressions"]:,} | {f["prover_ms"]:.3f} [{f["range_ms"][0]:.3f}, {f["range_ms"][1]:.3f}] | {k["prover_ms"]:.3f} [{k["range_ms"][0]:.3f}, {k["range_ms"][1]:.3f}] | {r["ratio"]:.4f} |')
lines += ["", "这里的 2^30 是 padded Boolean witness 的 bit 数；对应 32,767 个 SHA-256 compression gates、2^26 个 wiring bits。每个 IO word 的 128 个 bit 分别参加 copy constraints：`sigma_bit[128*i+j] = 128*sigma_word[i]+j`。双方使用同一 SHA 树、完整 padded Z、κ=128（Flock skip7）和 functional Ligerito PCS。", "", "## 最大规模的时间差来自哪里", "", "| 阶段 | Flock ms | Mikros ms |", "|---|---:|---:|"]
r = common[-1]
for label, key in (("Commitment", "commit_ms"), ("Boolean（Hadamard + lincheck）", "boolean_ms"), ("Wiring（permutation + gather）", "wiring_ms"), ("并行 PIOP 总时间", "parallel_piops_ms"), ("PCS opening", "pcs_open_ms"), ("完整 prover", "prover_ms")):
    lines.append(f'| {label} | {r["flock"][key]:.3f} | {r["mikros"][key]:.3f} |')
lines += [
    "",
    f'Mikros 的并行 PIOP 耗时为 Flock 的 {r["mikros"]["parallel_piops_ms"] / r["flock"]["parallel_piops_ms"]:.3f} 倍，耗时减少 {(1-r["mikros"]["parallel_piops_ms"] / r["flock"]["parallel_piops_ms"])*100:.1f}%；PCS opening 则多约 {(r["mikros"]["pcs_open_ms"]-r["flock"]["pcs_open_ms"])/1000:.3f} 秒。完整 prover 多 {(r["mikros"]["prover_ms"]-r["flock"]["prover_ms"])/1000:.3f} 秒（{(r["ratio"]-1)*100:.1f}%）。',
    "",
    "Boolean 与 wiring 并行，不能相加。每阶段独立取中位数，也不能要求阶段中位数精确加总为总时间中位数。`analysis.json` 的 other_ms 按每个样本先算未单列的余项，再取中位数。",
    "",
    "当前 Flock 产生 11 条 PCS tensor claims：Boolean 3 条，加 wiring 的 8 条。Mikros 产生 20 条：Boolean 4 条，加两个 permutation 端点各 8 条。因此相同 PCS 后端仍有不同 opening 工作量；最终双方各运行一次合并的 Ligerito opening，不能称为 11/20 次完整 PCS。",
    "",
    "`crates/flock-core/src/pcs/functional.rs` 的当前通用前端逐 claim 展开完整 row weights、扫描 witness，再生成完整 basis contribution。第二端点额外带来 8 条查询，分开的 A/B lincheck 再多 1 条。这与 opening 变慢的方向一致；当前计时未单独分离 ring-switch、basis 构造和 Ligerito，不能把全部差值精确归给某一操作。",
    "",
    "后续最直接的工程候选是利用 gate/pin 查询的稀疏支持：每条只涉及 256 个 constraint chunks 中的一个，却仍走全表扫描。局部折叠、复用同一端点的 instance weights、直接累加最终 basis，可在保持每条声明和 transcript 顺序不变的前提下减少工作。这些优化尚未计入本次数字。",
    "",
    "## 完整验证与证明体积",
    "",
    "| witness bit-log | Flock 验证 ms | Mikros 验证 ms | Flock 完整证明 B | Mikros 完整证明 B |",
    "|---|---:|---:|---:|---:|",
]
for r in common:
    lines.append(f'| {r["metadata"]["logical_bit_log"]} | {r["flock"]["verify_ms"]:.3f} | {r["mikros"]["verify_ms"]:.3f} | {r["flock"]["proof_bytes"]:,} | {r["mikros"]["proof_bytes"]:,} |')
lines += ["", "验证器只接收公共数据和证明；所有 witness 终端声明经 PCS 认证，公开矩阵和 sigma 声明由验证器计算检查。验证时间包含这些公开声明检查，排除静态 verifier 数据构造。正式样本和预热均完整验证，重复证明的 digest 和下一条 transcript challenge 一致。native 测试共 44 项通过；另 Flock 18 项、Mikros 28 项 E2E 篡改均被拒绝。", "", "## Flock production 接口的独立参考", "", "| compression 数 | 完整接口 prover ms [min, max] | verifier ms | 完整证明 B |", "|---:|---:|---:|---:|"]
for r in production:
    lines.append(f'| {r["metadata"]["gate_count"]:,} | {r["prover_ms"]:.3f} [{r["range_ms"][0]:.3f}, {r["range_ms"][1]:.3f}] | {r["verify_ms"]:.3f} | {r["proof_bytes"]:,} |')
lines += [
    "",
    "这是原 `prove_fast_ligerito_union_circuit`：保留 word wiring、dense compaction、50 integer lanes、原默认 Boolean/lincheck 和 merged opening。内部 assembly/compaction 仍计入完整接口时间，未从中估算扣除。它和上面的共同 bit 路径在布局、布线粒度及 opening 处理上不同，因此单列工程参考，不能用差值衡量纯 PIOP 替换收益。两条路径的同规模 native SHA 根与实际 gate 数已经核对一致。",
    "",
    "## Offline 与内存记录",
    "",
    "| witness bit-log | Mikros 系数准备 ms（一次） | 系数 payload MiB | 两系统共同进程峰值 RSS GiB |",
    "|---|---:|---:|---:|",
]
for r in common:
    m = r["metadata"]
    lines.append(f'| {m["logical_bit_log"]} | {m["offline_mikros_coefficients_ms"]:.3f} | {m["offline_mikros_coefficients_resident_bytes"]/2**20:.3f} | {r["both_process_peak_rss_gib"]:.3f} |')
lines += [
    "",
    "系数依赖 witness、但不依赖随机挑战，按用户要求从 online 中排除；更换 witness 后仍需重新生成。offline_wiring_ms 混合准备两系统的表示，不能拆称某一系统独立的 offline 总成本。m30 的 Flock value、Flock sigma tags、Mikros sigma tags 各另占 1 GiB payload。RSS 包括两套缓存、准备、预热、证明和验证，不能据此判断某一系统独立内存峰值。",
    "",
    "## 复现与限制",
    "",
    "- CPU：Intel Xeon Gold 6342 @ 2.80 GHz，主机约 377.51 GiB RAM。固定 64 logical CPUs：`0-47,48-55,72-79`；Rayon 与 Flock all-core pool 各设 64，非总共 64 个 OS 线程的断言。",
    "- 工具链：nightly-2026-03-21 / rustc 1.96.0-nightly，LLVM 22.1.0；release、`-C target-cpu=native`、offline locked build。metadata 确认 AVX2、AVX-512F/BW、GFNI、VPCLMULQDQ 启用；没有做逐指令占比测量。",
    "- 执行时间：2026-09-13 00:59:03–01:01:36（UTC+8）。所有阶段退出码为零，前后源码及二进制摘要一致。原始日志、命令、硬件与资源记录均在 `results-e2e/`。",
    "- 预检选定 CPU 的 1 秒忙碌率不超过 0.047%；cgroup 本层 memory.max/high 为 max。未做覆盖整场实验及全部祖先 cgroup 的隔离审计，不能宣称完全无资源竞争。",
    "- 三次正式样本来自同一固定 witness/transcript，报告运行波动，不代表跨输入的统计总体。",
    "- 未增加 zero-knowledge masking，未将新 verifier 接入递归 R1CS tower。当前交付是有真实 commitment、完整 PCS 验证的非交互电路 argument。公开 sigma 检查仍有线性成本，不宣称任意电路的 polylog verifier。",
    "- 保留原 Fast PCS；Boolean PoW8，permutation 按日志的事件工作因子策略。不同协议的这些设置不构成新的整体严格 128-bit 安全性等价定理。",
    "",
    "冻结源码 SHA256：`f22f9d9316e4e9480fdffa29d2255ef84393d4ac74a2ded188fb5d06f174c21c`。证据 archive SHA256：`1b52be8c205e76264f52e1ac12c2d93e5f185a5f6935107ae1197e2bcbdabfc7`。逐文件及 executable 摘要见 `evidence-manifest.json` 与 `results-e2e/binary-sha256.json`。",
    "",
    "在本目录运行 `python3 analyze.py` 可重新校验证据并生成本报告及 `analysis.json`。方法和代码入口见 [E2E.md](../../../E2E.md)，原始服务器主统计见 [summary.json](results-e2e/summary.json)。",
]
(ROOT / "RESULTS.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
print(json.dumps({"evidence_verified": True, "native_tests": tests, "negative_tests": negative,
                  "common_ratios": [x["ratio"] for x in common]}, indent=2))
