"""Verify downloaded v2 evidence and regenerate the PCS alignment report."""
import hashlib
import json
import re
import statistics as stats
from pathlib import Path

ROOT = Path(__file__).resolve().parent
LOGS = ROOT / "results-e2e"
SOURCE_SHA = "28189bdd8c780c8a44f4ba152d0d63d52e27f745aae82d7739a10f9732ccb5cb"
for name, digest in json.loads((ROOT / "evidence-manifest.json").read_text()).items():
    assert hashlib.sha256((ROOT / name).read_bytes()).hexdigest() == digest, name
evidence_sha = hashlib.sha256((ROOT / "evidence.tar.gz").read_bytes()).hexdigest()
assert evidence_sha == (ROOT / "evidence.tar.gz.sha256").read_text().split()[0]
assert json.loads((ROOT / "run-extra.json").read_text())["archive_sha256"] == SOURCE_SHA
status = json.loads((LOGS / "status.json").read_text())
assert status["state"] == "complete" and not status["failed"]
assert all(x["exit_code"] == 0 for x in status["completed"])

def records(name):
    return [json.loads(line) for line in (LOGS / name).read_text().splitlines() if line.strip()]

def group(rows, system=None):
    all_samples = [x for x in rows if x["record"] == "sample" and (system is None or x["system"] == system)]
    assert len(all_samples) == 4
    assert all(x["verified"] and x["transcript_match"] for x in all_samples)
    assert len({x["proof_blake3"] for x in all_samples}) == 1
    formal = [x for x in all_samples if not x["warmup"]]
    assert sorted(x["iteration"] for x in formal) == [1, 2, 3]
    return formal

common, production = [], []
summary = {r["log_leaves"]: r for r in json.loads((LOGS / "summary.json").read_text())}
proofs_checked = 0
for leaves in (6, 10, 14):
    cr, pr = records(f"common-leaves{leaves}.stdout"), records(f"production-leaves{leaves}.stdout")
    cm, pm = cr[0], pr[0]
    assert cm["schema_version"] == 2 and cm["pcs_path"] == "flock-production-dense-merged"
    for m in (cm, pm):
        assert m["threads"] == m["all_core_threads"] == 64
        assert {"avx512f", "avx512bw", "gfni", "vpclmulqdq"} <= set(m["target_features"])
    assert cm["compression_kappa"] == 128 and cm["flock_skip"] == 7
    for k in ("pcs_params", "pcs_config_blake3", "pcs_queries", "pcs_query_grinding_bits", "pcs_fold_grinding_bits", "native_root_u32"):
        assert cm[k] == pm[k], (leaves, k)
    raw = [x for x in cr + pr if x["record"] == "sample"]
    assert len({x["commitment_blake3"] for x in raw}) == 1
    proofs_checked += len(raw)
    row = {"metadata": cm, "commitment_blake3": raw[0]["commitment_blake3"], "same_production_pcs_and_commitment": True}
    for name, expected_claims in (("flock", 11), ("mikros", 20)):
        xs = group(cr, name)
        assert all(x["pcs_config_match"] and x["timings"]["pcs_inner_openings"] == 1 for x in xs)
        assert all(x["timings"]["tensor_claims"] == expected_claims for x in xs)
        v = {k: stats.median(x["timings"][k] for x in xs) for k in xs[0]["timings"]}
        v.update(verify_ms=stats.median(x["verify_ms"] for x in xs), proof_bytes=xs[0]["proof_bytes"],
                 range_ms=[min(x["timings"]["prover_ms"] for x in xs), max(x["timings"]["prover_ms"] for x in xs)],
                 other_ms=stats.median(x["timings"]["prover_ms"] - sum(x["timings"][k] for k in ("commit_ms", "parallel_piops_ms", "pcs_open_ms")) for x in xs))
        for k in ("prover_ms", "commit_ms", "parallel_piops_ms", "pcs_open_ms", "verify_ms", "proof_bytes"):
            assert v[k] == summary[leaves][name][k]
        row[name] = v
    row["ratio"] = row["mikros"]["prover_ms"] / row["flock"]["prover_ms"]
    common.append(row)
    xs = group(pr)
    production.append({"metadata": pm, "prover_ms": stats.median(x["prove_with_commit_ms"] for x in xs),
                       "range_ms": [min(x["prove_with_commit_ms"] for x in xs), max(x["prove_with_commit_ms"] for x in xs)],
                       "verify_ms": stats.median(x["verify_ms"] for x in xs), "proof_bytes": xs[0]["proof_bytes"]})
validation = records("validation-m22.stdout")
assert all(x["verified"] and x["transcript_match"] for x in validation if x["record"] == "sample")
proofs_checked += sum(x["record"] == "sample" for x in validation)
negative = {x["system"]: x["rejected"] for x in validation if x["record"] == "negative_tests"}
assert negative == {"flock": 23, "mikros": 33}
tests = sum(int(n) for f in LOGS.glob("test-*.stdout") for n in re.findall(r"test result: ok\. (\d+) passed", f.read_text()))
assert tests == 51
(ROOT / "analysis.json").write_text(json.dumps({"common": common, "production": production, "native_tests": tests,
                                                "negative": negative, "proofs_verified": proofs_checked}, indent=2) + "\n")

largest = common[-1]
direction = "增加" if largest["ratio"] >= 1 else "减少"
lines = [
    "# 使用同一 production PCS 的 Mikros / Flock E2E",
    "",
    "PCS 对齐已完成，并在服务器上重跑三档逐 bit wiring 实验。双方实际复用 Flock production 的 dense commitment、merged sumcheck、Frobenius assist 和单个 EqPoint Ligerito opening；全部 PCS 配置及承诺摘要与未替换 PIOP 的 Flock 原生 E2E 交叉核验相等。",
    "",
    f'最大规模 2^30 witness bits：Flock {largest["flock"]["prover_ms"]/1000:.3f} s，Mikros {largest["mikros"]["prover_ms"]/1000:.3f} s，Mikros 在线时间{direction} {abs(largest["ratio"]-1)*100:.1f}%（加速比 {1/largest["ratio"]:.2f}×）。这组数字采用已对齐的 PCS 路径；旧 full-padded functional v1 数据保持独立。',
    "",
    "## PCS 一致性的实际检查",
    "",
    "| 检查项 | 三路共同结果 |",
    "|---|---|",
    "| 承诺输入 | 同一 SHA witness，通过 `UnionInstance::compact_witness` 生成同一 dense stack |",
    "| Commitment | 同一 `commit_lane_major`，50 个 active lanes，BLAKE3 cap |",
    "| 参数 | 完整 `PcsParams` 相等，包括 m、rate、batch size、profile、lanes、hash |",
    "| 安全配置 | embedded config 摘要、PCS query 数及 grinding 向量逐项相等 |",
    "| 承诺结果 | 每档双方和 production 全部样本的序列化 commitment 摘要相同 |",
    "| Opening | 共用 `open_batch_merged`：jagged weight/首轮融合、merged sumcheck、multipoint Frobenius assist |",
    "| 内层 PCS | 每份证明只有一个 `open_batch_mixed_ligerito_seeded` 的 EqPoint opening |",
    "| Verifier | 共用原 merged verifier 主体，只把旧 skip64 列权重检查泛化为完整 128 列权重 |",
    "",
    "SHA 的声明行数是 2^nu−1，属于 partial-height layout，实际没有触发仅适用于 full-height prefix 的 block-first 路径。保留 production 的真实分支条件，未通过篡改 counts/零 padding 强行启用该路径。",
    "",
    "## 同一 PCS、逐 bit wiring 的在线 prover",
    "",
    "服务器 Intel Xeon Gold 6342，固定 64 logical CPUs，两个线程池各 64，release native（GFNI、AVX-512F/BW、VPCLMULQDQ）。双方 κ128，Flock skip7。每系统每档预热 1 次、正式 3 次，交替顺序；以下为正式样本中位数及 min/max。",
    "",
    "| Witness bits | Wiring bits | SHA compression 数 | Flock ms [min,max] | Mikros ms [min,max] | M/F |",
    "|---|---|---:|---:|---:|---:|",
]
for r in common:
    m, f, k = r["metadata"], r["flock"], r["mikros"]
    lines.append(f'| 2^{m["logical_bit_log"]} | 2^{m["wiring_bit_log"]} | {m["sha_compressions"]:,} | {f["prover_ms"]:.3f} [{f["range_ms"][0]:.3f},{f["range_ms"][1]:.3f}] | {k["prover_ms"]:.3f} [{k["range_ms"][0]:.3f},{k["range_ms"][1]:.3f}] | {r["ratio"]:.4f} |')
lines += [
    "",
    "Online 包含 commit、PIOP、所有挑战相关的权重转换/分量计算、gather、merged PCS 和函数返回时的临时数据释放。Offline witness/tuple/系数准备、dense compaction、owned q 拷贝、验证和序列化不计入。compaction 使用相同例程，在计时前明确执行，未从融合阶段估算扣除。",
    "",
    "## 最大规模阶段分解",
    "",
    "| 阶段 | Flock ms | Mikros ms |",
    "|---|---:|---:|",
]
for title, key in (("Commitment", "commit_ms"), ("Boolean 分支", "boolean_ms"), ("Wiring 分支", "wiring_ms"),
                   ("并行 PIOP 总时间", "parallel_piops_ms"), ("输入适配 + production merged PCS", "pcs_open_ms")):
    lines.append(f'| {title} | {largest["flock"][key]:.3f} | {largest["mikros"][key]:.3f} |')
lines += [
    "",
    "Boolean/wiring 并行，不能相加；各阶段独立取中位数，也不要求中位数精确加总。`pcs_open_ms` 包括新查询的在线适配及原生 merged opening 全流程，未单独拆量每一内核。",
    "",
    "相同 PCS 不意味着相同查询工作量：Flock 为 11 个原始 tensor claims（Boolean 3 + wiring 8），Mikros 为 20 个（Boolean 4 + 两个 wiring 端点各 8）。保留全部声明，双方最终各开一个内层 EqPoint；没有把 20 条声明删成 11 条，也没有给 Flock 填补无用查询。",
    "",
    "输入适配把 `[a,b]` 非零和权重写成 scale×普通 eq 权重，scale 进入 128 列权重。d 个 `[a,a]` 非零零和维使用 d+1 个固定节点的最高次系数插值，所有分量都经同一 PCS 认证并检查求和；零 functional 必须值为零。旧 production prover 的算法和 proof 结构不变。",
    "",
    "## 完整证明验证",
    "",
    "| Witness bit-log | Flock / Mikros verifier ms | Flock / Mikros 完整证明 B | 原始 claims / ring claims / inner openings（F; M） |",
    "|---|---:|---:|---|",
]
for r in common:
    f, k = r["flock"], r["mikros"]
    lines.append(f'| {r["metadata"]["logical_bit_log"]} | {f["verify_ms"]:.3f} / {k["verify_ms"]:.3f} | {f["proof_bytes"]:,} / {k["proof_bytes"]:,} | {f["tensor_claims"]}/{f["pcs_ring_claims"]}/{f["pcs_inner_openings"]}; {k["tensor_claims"]}/{k["pcs_ring_claims"]}/{k["pcs_inner_openings"]} |')
lines += [
    "",
    f'{tests} 个定向 native 测试通过；完整 E2E 篡改检查 Flock {negative["flock"]} 项、Mikros {negative["mikros"]} 项均拒绝。{proofs_checked} 份实际证明（27 正式、9 预热、2 validation）均完整验证并通过 transcript 一致性检查。同一输入的重复 proof digest 稳定。',
    "",
    "新增测试包含：旧 skip64 与 weighted128 verifier 的相同终态、adapter 与直接调用生产 merged prover 的完整 proof 相等、非归一化/零和源 functional 等价，以及两个 component 同时篡改但总和不变仍被 PCS 拒绝。",
    "",
    "## Flock 原生 word-wiring 接口参考",
    "",
    "| SHA compression 数 | prover 完整接口 ms [min,max] | verifier ms | 完整证明 B |",
    "|---:|---:|---:|---:|",
]
for r in production:
    lines.append(f'| {r["metadata"]["gate_count"]:,} | {r["prover_ms"]:.3f} [{r["range_ms"][0]:.3f},{r["range_ms"][1]:.3f}] | {r["verify_ms"]:.3f} | {r["proof_bytes"]:,} |')
lines += [
    "",
    "此参考与主比较的 PCS 已对齐；它仍使用 word wiring、原默认 Boolean/lincheck、packed-direct gather 和融合的中间结果，并且旧完整接口包含内部 assembly/compaction。因此该列继续单列，不能将其和逐 bit 主比较的差值全部归给 Mikros PIOP。",
    "生产接口沿用自己的默认验证调度，PIOP/PCS 验证核心在 1 线程 verifier_pool 内，外层公开矩阵检查仍可能使用全局池；共同 bit 实验的 verifier 可使用全局 64 线程。三路的 prover 都按 64 线程测量，production 的 verifier 列不作为同线程速度对比。",
    "",
    "## 证据与限制",
    "",
    f'- 源码 archive SHA256：`{SOURCE_SHA}`；证据 archive SHA256：`{evidence_sha}`。逐文件及 executable 摘要见 evidence-manifest.json 和 results-e2e/binary-sha256.json。',
    f'- 服务器运行 UTC：{status["started_utc"]} 至 {status["finished_utc"]}，所有阶段退出码为零，源码和二进制前后摘要一致。',
    "- 原始样本、构建/测试输出、命令和资源记录均保留在 results-e2e/；本脚本独立复算并核对服务器 summary。",
    "- 单进程同时保留两套 offline 表，RSS 不能归给单独某个系统。资源预检仅覆盖启动前短窗口，不证明整个实验无竞争。",
    "- 三次样本使用固定 witness/transcript，反映运行波动，不代表跨输入统计总体。commit 改变会改变 PIOP 挑战，因此旧 v1 与 v2 的差值不是保持 transcript 不变的单核消融。",
    "- 未增加 zero-knowledge masking，未接入递归 verifier；公开矩阵/sigma 检查仍按完整 verifier 执行。相同 Fast PCS 不构成对两种不同 PIOP 的新整体 128-bit 安全性定理。",
    "",
    "运行 `python3 analyze.py` 可重新核验证据并生成本报告和 analysis.json。实现与方法见 [E2E.md](../../../E2E.md)，统计见 [summary.json](results-e2e/summary.json)。",
]
(ROOT / "RESULTS.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
print(json.dumps({"tests": tests, "negative": negative, "proofs_verified": proofs_checked,
                  "all_production_pcs_and_commitments_match": True, "ratios": [x["ratio"] for x in common]}, indent=2))
