#!/usr/bin/env python3
"""Independently verify v3 server evidence and regenerate the Chinese report.

Run after downloading the complete evidence bundle into this directory:
    python3 analyze.py [--source-sha256 EXPECTED_UPLOAD_SHA256]

No measurements or test counts are supplied by this script. Missing, incomplete,
v2, or inconsistent evidence is an error; output is written only after checks pass.
The expected upload digest defaults to the independently recorded v3 upload;
the source archive itself is not part of the downloaded result bundle.
"""

import argparse
import hashlib
import json
import math
import re
import statistics as stats
from pathlib import Path

ROOT = Path(__file__).resolve().parent
LOGS = ROOT / "results-e2e"
SOURCE_SHA256 = "a12caa90ee14b59abbd32f75bd1cf0cbd9de88872b67cc45f96f268b9d156011"
MODES = {
    "legacy": {"flock": 11, "mikros": 20},
    "cube": {"flock": 4, "mikros": 6},
    "lockstep": {"flock": 4, "mikros": 5},
}
STAGES = ("prover_ms", "commit_ms", "boolean_ms", "wiring_ms",
          "parallel_piops_ms", "pcs_open_ms")
COUNTS = ("tensor_claims", "pcs_ring_claims", "pcs_inner_openings")
PCS_KEYS = ("pcs_params", "pcs_config_blake3", "pcs_queries",
            "pcs_query_grinding_bits", "pcs_fold_grinding_bits", "native_root_u32")
FEATURES = {"avx512f", "avx512bw", "gfni", "vpclmulqdq"}
TEST_STAGES = {
    "test-pcs", "test-merged-tensor", "test-weighted-pcs", "test-permutation",
    "test-e2e-helpers", "test-packed-ip", "test-mikros-lincheck",
    "test-flock_skip7_interpolation", "test-univariate_roundtrip",
}
READ_PATHS = set()


def require(condition, message):
    if not condition:
        raise ValueError(message)


def read_text(path):
    READ_PATHS.add(path.relative_to(ROOT).as_posix())
    return path.read_text(encoding="utf-8-sig")


def load(path):
    return json.loads(read_text(path))


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def digest_ok(value):
    return isinstance(value, str) and re.fullmatch(r"[0-9a-f]{64}", value) is not None


def finite_nonnegative(value):
    return type(value) in (int, float) and math.isfinite(value) and value >= 0


def check_evidence(expected_source):
    manifest = load(ROOT / "evidence-manifest.json")
    require(isinstance(manifest, dict) and manifest, "empty evidence manifest")
    for name, expected in manifest.items():
        path = (ROOT / name).resolve()
        require(path.is_relative_to(ROOT) and path.is_file(), f"invalid evidence path: {name}")
        require(digest_ok(expected) and sha256(path) == expected, f"evidence SHA256 mismatch: {name}")
    evidence_sha = sha256(ROOT / "evidence.tar.gz")
    sidecar = read_text(ROOT / "evidence.tar.gz.sha256").split()
    require(sidecar and sidecar[0] == evidence_sha, "evidence archive SHA256 mismatch")
    source_sha = load(ROOT / "run-extra.json")["archive_sha256"]
    require(digest_ok(source_sha), "invalid uploaded source digest")
    if expected_source is not None:
        require(digest_ok(expected_source) and expected_source == source_sha,
                "uploaded source digest differs from independent expected digest")
    sources = load(ROOT / "source_manifest.json")["files_sha256"]
    require(sources and all(digest_ok(x) for x in sources.values()), "invalid source manifest")
    binaries = load(LOGS / "binary-sha256.json")
    require(len(binaries) == 2 and all(digest_ok(x) for x in binaries.values()), "invalid binary digests")
    require({Path(p).name for p in binaries} == {"e2e_bit_compare", "e2e_flock_baseline"},
            "unexpected measured executables")
    return manifest, {"source_archive_sha256": source_sha, "evidence_archive_sha256": evidence_sha,
                      "independent_source_digest_supplied": expected_source is not None,
                      "source_files": len(sources), "binary_sha256": binaries}


def command_options(name):
    argv = load(LOGS / f"{name}.command.json")
    require(isinstance(argv, list) and all(isinstance(x, str) for x in argv), f"{name}: bad command")
    options = {}
    for key in ("--log-leaves", "--threads", "--reps", "--warmups", "--reduction", "--profile"):
        require(argv.count(key) <= 1, f"{name}: duplicate option {key}")
        if key in argv:
            index = argv.index(key) + 1
            require(index < len(argv), f"{name}: missing option value")
            options[key] = argv[index]
    require("taskset" in argv and "-c" in argv, f"{name}: missing CPU pinning")
    return argv, options


def records(name):
    result = []
    for number, line in enumerate(read_text(LOGS / f"{name}.stdout").splitlines(), 1):
        if line.strip():
            try:
                item = json.loads(line)
            except json.JSONDecodeError as error:
                raise ValueError(f"{name}:{number}: non-JSON measurement output") from error
            require(isinstance(item, dict) and item.get("record") in
                    {"metadata", "sample", "negative_tests"}, f"{name}:{number}: unexpected record")
            result.append(item)
    require(result and result[0]["record"] == "metadata", f"{name}: missing initial metadata")
    require(sum(x["record"] == "metadata" for x in result) == 1, f"{name}: duplicate metadata")
    return result


def check_metadata(metadata, name, mode, options):
    require(metadata["threads"] == metadata["all_core_threads"] == 64, f"{name}: wrong thread pools")
    require(FEATURES <= set(metadata["target_features"]), f"{name}: missing native kernels")
    require(int(options["--threads"]) == 64, f"{name}: command thread mismatch")
    require(metadata["log_leaves"] == int(options["--log-leaves"]), f"{name}: command size mismatch")
    require(metadata["fixture"] == "sha256-binary-tree", f"{name}: unexpected fixture")
    require(metadata["pcs_params"]["profile"] == "fast", f"{name}: PCS profile is not Fast")
    require(metadata["pcs_params"]["m"] == metadata["dense_m"], f"{name}: non-dense PCS parameters")
    require(digest_ok(metadata["pcs_config_blake3"]), f"{name}: malformed PCS config digest")
    if mode is None:
        require(metadata["schema_version"] == 1 and metadata["system"] == "flock-circuit-e2e",
                f"{name}: wrong production schema")
        require(options.get("--profile") == "fast", f"{name}: production profile mismatch")
        require(metadata["zero_knowledge_claim"] is False, f"{name}: unexpected ZK claim")
    else:
        require(metadata["schema_version"] == 3, f"{name}: requires v3 (v2 is not an ablation)")
        require(metadata["reduction"] == mode == options.get("--reduction", "lockstep"),
                f"{name}: wrong reduction mode")
        require(metadata["pcs_path"] == "flock-production-dense-merged", f"{name}: wrong PCS path")
        require(metadata["compression_kappa"] == 128 and metadata["flock_skip"] == 7,
                f"{name}: compression mismatch")
        require(metadata["gather_audit_bits"] == (0 if mode == "legacy" else 2) and
                metadata["gather_audit_degree"] == (0 if mode == "legacy" else 3),
                f"{name}: cube audit policy mismatch")
        require(metadata["zero_knowledge"] is False, f"{name}: unexpected ZK claim")
        require(0 < metadata["dense_words"] <= metadata["committed_words"] == 1 << (metadata["dense_m"] - 7),
                f"{name}: wrong zero-padded dense commitment size")


def check_samples(rows, name, system, reps, warmups, mode):
    samples = [x for x in rows if x["record"] == "sample" and x["system"] == system]
    require(len(samples) == reps + warmups, f"{name}/{system}: sample count mismatch")
    require(sorted(x["iteration"] for x in samples) == list(range(reps + warmups)),
            f"{name}/{system}: incomplete/duplicate iterations")
    for sample in samples:
        require(type(sample["warmup"]) is bool and sample["warmup"] == (sample["iteration"] < warmups),
                f"{name}/{system}: wrong warmup flag")
        require(sample["verified"] is True and sample["transcript_match"] is True,
                f"{name}/{system}: incomplete proof/transcript verification")
        require(digest_ok(sample["proof_blake3"]) and digest_ok(sample["commitment_blake3"]),
                f"{name}/{system}: invalid proof or commitment digest")
        require(type(sample["proof_bytes"]) is int and sample["proof_bytes"] > 0,
                f"{name}/{system}: invalid proof length")
        require(finite_nonnegative(sample["verify_ms"]), f"{name}/{system}: invalid verifier timing")
        if mode is None:
            require(finite_nonnegative(sample["prove_with_commit_ms"]), f"{name}: invalid prover timing")
        else:
            timing = sample["timings"]
            require(sample["pcs_config_match"] is True, f"{name}/{system}: unverified PCS config")
            require(all(finite_nonnegative(timing[k]) for k in STAGES), f"{name}: invalid stage timing")
            require(all(type(timing[k]) is int for k in COUNTS), f"{name}: noninteger claim counts")
            require(timing["tensor_claims"] == timing["pcs_ring_claims"] == MODES[mode][system] and
                    timing["pcs_inner_openings"] == 1, f"{name}/{system}: wrong complete PCS obligations")
    require(len({x["proof_blake3"] for x in samples}) == 1 and
            len({x["proof_bytes"] for x in samples}) == 1,
            f"{name}/{system}: proof changed across repetitions (including warmup)")
    require(len({x["commitment_blake3"] for x in samples}) == 1, f"{name}/{system}: changing commitment")
    return [x for x in samples if not x["warmup"]]


def analyze_run(name, mode, validation=False):
    rows = records(name)
    metadata = rows[0]
    argv, options = command_options(name)
    reps, warmups = int(options["--reps"]), int(options["--warmups"])
    require(reps > 0 and warmups >= 0, f"{name}: invalid repetitions")
    require((reps == 1 and warmups == 0 and "--tamper" in argv) if validation else
            (warmups == 1 and "--tamper" not in argv), f"{name}: wrong validation/timing command")
    check_metadata(metadata, name, mode, options)
    systems = ("flock-circuit-e2e",) if mode is None else ("flock", "mikros")
    require({x["system"] for x in rows if x["record"] == "sample"} == set(systems),
            f"{name}: unexpected systems")
    result = {"name": name, "metadata": metadata, "reps": reps, "warmups": warmups,
              "formal_proofs": reps * len(systems), "warmup_proofs": warmups * len(systems)}
    all_samples = [x for x in rows if x["record"] == "sample"]
    require(len({x["commitment_blake3"] for x in all_samples}) == 1, f"{name}: unequal commitments")
    result["commitment_blake3"] = all_samples[0]["commitment_blake3"]
    for system in systems:
        xs = check_samples(rows, name, system, reps, warmups, mode)
        prover_key = "prove_with_commit_ms" if mode is None else "prover_ms"
        values = [x[prover_key] if mode is None else x["timings"][prover_key] for x in xs]
        value = {"verify_ms": stats.median(x["verify_ms"] for x in xs),
                 "verify_range_ms": [min(x["verify_ms"] for x in xs), max(x["verify_ms"] for x in xs)],
                 "proof_bytes": xs[0]["proof_bytes"], "proof_blake3": xs[0]["proof_blake3"],
                 "prover_ms": stats.median(values), "prover_range_ms": [min(values), max(values)]}
        if mode is not None:
            value.update({key: stats.median(x["timings"][key] for x in xs) for key in STAGES})
            value.update({key: xs[0]["timings"][key] for key in COUNTS})
            value["other_ms"] = stats.median(x["timings"]["prover_ms"] - sum(x["timings"][k] for k in
                                             ("commit_ms", "parallel_piops_ms", "pcs_open_ms")) for x in xs)
        result[system if mode is not None else "production"] = value
    negatives = [x for x in rows if x["record"] == "negative_tests"]
    if validation:
        require(len(negatives) == 2 and {x["system"] for x in negatives} == set(systems),
                f"{name}: missing/duplicate negative-test records")
        require(all(type(x["rejected"]) is int and x["rejected"] > 0 for x in negatives),
                f"{name}: invalid rejection counts")
        result["negative"] = {x["system"]: x["rejected"] for x in negatives}
    else:
        require(not negatives, f"{name}: tamper tests included in timing run")
    if mode is not None:
        require(result["flock"]["prover_ms"] > 0, f"{name}: zero prover time")
        result["mikros_over_flock"] = result["mikros"]["prover_ms"] / result["flock"]["prover_ms"]
    return result


def server_summary(row):
    """Reconstruct the server summary schema from independently checked samples."""
    m = row["metadata"]
    result = {key: m[key] for key in ("logical_bit_log", "wiring_bit_log", "sha_compressions",
                                     "log_leaves", "pcs_params", "pcs_path", "reduction")}
    result.update(same_production_pcs_config=True, same_commitment=True,
                  commitment_blake3=row["commitment_blake3"], mikros_over_flock=row["mikros_over_flock"])
    keys = STAGES + COUNTS + ("proof_bytes", "verify_ms", "prover_range_ms")
    for system in ("flock", "mikros"):
        result[system] = {key: row[system][key] for key in keys}
    return result


def check_summary(filename, expected, keys):
    observed = load(LOGS / filename)
    require(isinstance(observed, list) and len(observed) == len(expected), f"{filename}: row count differs")
    key = lambda row: tuple(row[k] for k in keys)
    require(len({key(x) for x in observed}) == len(observed), f"{filename}: duplicate rows")
    require(sorted(observed, key=key) == sorted(expected, key=key), f"{filename}: independent recomputation differs")


def same_production(row, production):
    for key in PCS_KEYS:
        require(row["metadata"][key] == production["metadata"][key], f'{row["name"]}: production mismatch: {key}')
    require(row["metadata"]["sha_compressions"] == production["metadata"]["gate_count"], "fixture size mismatch")
    require(row["commitment_blake3"] == production["commitment_blake3"], "production commitment differs")
    row["same_production_pcs_and_commitment"] = True


def test_counts(status):
    names = {x["name"] for x in status["completed"] if x["name"].startswith("test-")}
    require(TEST_STAGES <= names, "required native test stages are missing")
    require({x.stem for x in LOGS.glob("test-*.stdout")} == names, "test log/status set mismatch")
    counts = {}
    pattern = r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out"
    for name in sorted(names):
        body = read_text(LOGS / f"{name}.stdout")
        summaries = re.findall(pattern, body)
        require(summaries and "test result: FAILED" not in body, f"{name}: missing success summaries")
        totals = [sum(int(row[i]) for row in summaries) for i in range(5)]
        require(totals[0] > 0 and totals[1] == 0, f"{name}: no passing tests or failed tests")
        counts[name] = dict(zip(("passed", "failed", "ignored", "measured", "filtered_out"), totals))
        counts[name]["test_binaries"] = len(summaries)
    return counts


def cache_recovery():
    """Retain the unsuccessful cached build separately from the complete rerun."""
    details = load(ROOT / "cache-recovery.json")
    require(isinstance(details, dict) and details, "missing cache recovery explanation")
    touched = details["touched_sources"]
    sources = load(ROOT / "source_manifest.json")["files_sha256"]
    require(isinstance(touched, list) and touched and len(set(touched)) == len(touched) and
            set(touched) <= set(sources), "cache recovery paths not covered by the source manifest")
    failed = load(ROOT / "results-build-cache-failure" / "status.json")
    require(failed["state"] == "failed" and failed["failed"], "initial failed build evidence missing")
    require(all(x["name"] == "build" and x["exit_code"] != 0 for x in failed["failed"]),
            "initial run failed outside the documented build stage")
    stderr = read_text(ROOT / "results-build-cache-failure" / "build.stderr")
    driver_log = read_text(ROOT / "driver-build-cache-failure.log")
    require(stderr.strip() and driver_log.strip(), "initial failed build diagnostics missing")
    return {"details": details, "touched_source_count": len(touched), "failed_status": failed,
            "build_stderr": "results-build-cache-failure/build.stderr",
            "driver_log": "driver-build-cache-failure.log",
            "included_in_measurement_statistics": False}


def resource_record(name, hardware, validation=False):
    body = read_text(LOGS / f"{name}.resource.txt")
    match = re.search(r"Maximum resident set size \(kbytes\):\s*(\d+)", body)
    require(match and re.search(r"Exit status:\s*0(?:\s|$)", body), f"{name}: incomplete process resource report")
    argv, _ = command_options(name)
    require(argv[argv.index("-c") + 1] == hardware["cpu_affinity_spec"], f"{name}: affinity differs")
    result = {"maximum_process_rss_kib": int(match.group(1)), "scope": "whole executable, not one system"}
    if not validation:
        before = load(LOGS / f"{name}.resources-before.json")
        require(0 <= before["selected_cpu_busy_fraction"] < 0.10 and
                before["mem_available_kib"] > 64 * 1024 * 1024, f"{name}: failed resource precheck")
        require(set(hardware["selected_cpus"]) <= set(before["allowed_affinity"]), f"{name}: unavailable CPUs")
        result["before"] = before
    return result


def number_range(value):
    lo, hi = value["prover_range_ms"]
    return f'{value["prover_ms"]:.3f} [{lo:.3f}, {hi:.3f}]'


def report(data):
    common, production, ablations = data["common"], data["production"], data["ablations"]
    largest = common[-1]
    ratio = largest["mikros_over_flock"]
    direction = "增加" if ratio >= 1 else "减少"
    sizes = sorted({(row["warmups"], row["reps"]) for row in common + ablations + production})
    schedule = "；".join(f"预热 {w} 次、正式 {r} 次" for w, r in sizes)
    lines = [
        "# 同一 Flock production PCS：Mikros / Flock E2E v3",
        "",
        f'最大规模 2^{largest["metadata"]["logical_bit_log"]} witness bits，lockstep 模式的在线 prover 中位数：Flock {largest["flock"]["prover_ms"]/1000:.3f} s，Mikros {largest["mikros"]["prover_ms"]/1000:.3f} s；M/F = {ratio:.4f}，Mikros 时间{direction} {abs(ratio-1)*100:.1f}%。',
        "",
        "双方复用同一 production PCS 路径：dense witness compaction、commit_lane_major、open_batch_merged、merged sumcheck、Frobenius assist，以及单个 EqPoint Ligerito opening。完整 PcsParams、embedded Fast 配置摘要、query 数及 grinding 向量逐项相等；每档双方、各消融模式与原生 production 的序列化 commitment 摘要相同。相同 PCS 接口与配置不要求相同的终端声明数量或证明长度。",
        "",
        "## 主比较：lockstep，64 threads",
        "",
        f'服务器 CPU：{data["hardware"]["cpu_model"]}。固定 {len(data["hardware"]["selected_cpus"])} 个 logical CPUs（{data["hardware"]["cpu_affinity_spec"]}），两个线程池各 64；release target-cpu=native，GFNI、AVX-512F/BW、VPCLMULQDQ 均启用。κ = 128，Flock skip = 7。{schedule}，两系统交替顺序；表中为正式样本中位数及 min/max。',
        "",
        "| Witness bits | Wiring bits | SHA compression 数 | Flock ms [min, max] | Mikros ms [min, max] | M/F |",
        "|---|---|---:|---:|---:|---:|",
    ]
    for row in common:
        m = row["metadata"]
        lines.append(f'| 2^{m["logical_bit_log"]} | 2^{m["wiring_bit_log"]} | {m["sha_compressions"]:,} | {number_range(row["flock"])} | {number_range(row["mikros"])} | {row["mikros_over_flock"]:.4f} |')
    lines += [
        "",
        "Online 包含 commitment、PIOP、挑战相关的输入和查询转换、gather/cube audit、PCS adapter、完整 merged opening，以及 prover 函数返回时的临时数据释放。Offline witness/tuple/系数准备、dense compaction 和 owned q 拷贝在计时前完成；验证与序列化单独计量。未从融合阶段估算扣除 online 转换。",
        "",
        "## 2^30 的阶段时间",
        "",
        "| 阶段 | Flock ms | Mikros ms | M − F ms |",
        "|---|---:|---:|---:|",
    ]
    for title, key in (("Commitment", "commit_ms"), ("Boolean 分支", "boolean_ms"),
                       ("Wiring 分支", "wiring_ms"), ("并行 PIOP", "parallel_piops_ms"),
                       ("查询适配 + production merged PCS", "pcs_open_ms"),
                       ("逐样本外层剩余时间的中位数", "other_ms"), ("完整 online prover", "prover_ms")):
        f, m = largest["flock"][key], largest["mikros"][key]
        lines.append(f"| {title} | {f:.3f} | {m:.3f} | {m-f:+.3f} |")
    lines += [
        "",
        "Boolean 与 wiring 在同一线程池并行，分支时间不能相加；各阶段独立取中位数，也不要求相加精确等于总中位数。剩余时间逐样本计算为 prover − commit − 并行 PIOP − PCS，再取中位数。PCS 时间包含在线 adapter，未单独测量每个内部核。",
        "",
        "## 最大档的同版本模式消融",
        "",
        "| 模式 | 系统 | Prover ms [min, max] | 并行 PIOP ms | Wiring ms | PCS ms | Tensor / ring / inner | 完整证明 B |",
        "|---|---|---:|---:|---:|---:|---|---:|",
    ]
    mode_rows = {row["metadata"]["reduction"]: row for row in ablations + [largest]}
    for mode in MODES:
        for system in ("flock", "mikros"):
            v = mode_rows[mode][system]
            lines.append(f'| {mode} | {system} | {number_range(v)} | {v["parallel_piops_ms"]:.3f} | {v["wiring_ms"]:.3f} | {v["pcs_open_ms"]:.3f} | {v["tensor_claims"]}/{v["pcs_ring_claims"]}/{v["pcs_inner_openings"]} | {v["proof_bytes"]:,} |')
    lines += ["", "| 模式 | Mikros / Flock online 时间 |", "|---|---:|"]
    for mode in MODES:
        lines.append(f'| {mode} | {mode_rows[mode]["mikros_over_flock"]:.4f} |')
    for system in ("flock", "mikros"):
        a, b, c = (mode_rows[mode][system]["prover_ms"] for mode in MODES)
        lines.append(f'\n{system}：cube / legacy = {b/a:.4f}；lockstep / cube = {c/b:.4f}（均为总时间中位数之比）。')
    lines += [
        "",
        "legacy 保留原 gather：Flock 3 个 Boolean + 8 个 wiring = 11，Mikros 4 个 Boolean + 两端各 8 个 wiring = 20。cube 先绑定每端完整 8 值表，保留原端点的加权重组检查，再用一次三变量随机 cube audit 将每端交给 PCS 的查询从 8 个降为 1 个，因此为 Flock 4 / Mikros 6。lockstep 将 Mikros 两棵树尾部共同归约到同一终端查询，再做 cube audit，得到 Flock 4 / Mikros 5；没有删除未认证的声明。",
        "",
        "cube audit 的差多项式总次数至多 3，裸代数事件的界为 3/q（q = 2^128）；实现为此事件配置显式 2-bit PoW，全部 8 个表值及源声明先绑定，随后采样三个坐标。这只是该局部事件的参数说明，不是新的整体 128-bit 安全性定理。",
        "",
        "这些是同一 v3 源码和 PCS 的模式比较，不是保持挑战不变的单核消融：v3 transcript 显式绑定 reduction mode，各模式和系统域不同，因此挑战不同。旧 v2 使用旧域与旧协议顺序，不能并入本表当作同 transcript 的 legacy 样本。即使 Flock 的 cube 与 lockstep 使用相同算法路径，域标签不同仍会改变挑战和具体工作量。",
        "",
        "## 验证、证明长度与义务数量",
        "",
        "| Witness bit-log | Flock verifier ms | Mikros verifier ms | Flock / Mikros 完整证明 B | Flock / Mikros tensor claims |",
        "|---|---:|---:|---:|---:|",
    ]
    for row in common:
        f, m = row["flock"], row["mikros"]
        lines.append(f'| {row["metadata"]["logical_bit_log"]} | {f["verify_ms"]:.3f} | {m["verify_ms"]:.3f} | {f["proof_bytes"]:,} / {m["proof_bytes"]:,} | {f["tensor_claims"]} / {m["tensor_claims"]} |')
    c = data["proof_counts"]
    lines += [
        "",
        f'原始日志累计 {data["native_tests_passed"]} 次定向 native 测试通过（跨测试命令的执行次数，可能包含重叠过滤器，不宣称互异测试数）。{c["total"]} 份实际证明全部完成验证及 transcript 终态检查：计时正式样本 {c["timing_formal"]} 份，预热 {c["warmup"]} 份，validation {c["validation"]} 份。同一输入、系统及模式下的 proof 摘要和长度在包括预热的所有重复中稳定。',
        "",
        "| Validation 模式 | Flock 拒绝篡改项 | Mikros 拒绝篡改项 |",
        "|---|---:|---:|",
    ]
    for row in data["validations"]:
        lines.append(f'| {row["metadata"]["reduction"]} | {row["negative"]["flock"]} | {row["negative"]["mikros"]} |')
    lines += [
        "",
        "测试数与篡改拒绝数均从本次日志动态汇总，未套用历史固定计数。逐命令结果及全部模式的 verifier 范围见 analysis.json。上述 transcript 一致性是各证明的 prover/verifier 终态相等，不表示不同模式的挑战相等。",
        "",
        "## Flock 原生 word-wiring 接口参考",
        "",
        "| SHA compression 数 | 完整接口 prover ms [min, max] | verifier ms | 完整证明 B |",
        "|---:|---:|---:|---:|",
    ]
    for row in production:
        v = row["production"]
        lines.append(f'| {row["metadata"]["gate_count"]:,} | {number_range(v)} | {v["verify_ms"]:.3f} | {v["proof_bytes"]:,} |')
    lines += [
        "",
        "此列复用相同 PCS 与 commitment，但使用原 word wiring、默认 Boolean/lincheck、packed-direct gather 和原中间结果融合；完整接口还包含内部 assembly/compaction。因此单列报告，不能把它与逐 bit 主比较的差值全部归给 Mikros PIOP。",
        "",
        "三个 prover 均配置 64 线程。原生 production 的 PIOP/PCS verifier 核心在自己的 1 线程 verifier_pool 中运行，外层公开矩阵检查仍可能使用全局池；共同 bit verifier 可以使用全局 64 线程。因此 production verifier 列不是同线程的速度比较。",
        "",
        "## PCS 布局与实验边界",
        "",
        "- SHA 声明行数为 2^nu−1，使用真实 partial-height 布局；没有通过修改公共 counts 或 padding 强行触发只适用于 full-height prefix 的 block-first 分支。",
        "- 任意 128 列权重直接进入 weighted verifier；非零和行因子归一化后的 scale 并入列权重。d 个非零零和行因子通过 d+1 个固定节点的最高次系数插值分解，各分量均由相同 PCS 认证，并验证重组值。零 functional 只允许零声明。本次原始 tensor 数与实际 ring 数逐样本检查相等，最终内层 opening 数均为 1。",
        "- Mikros lincheck 的 Laurent IP 输入为 F128，不能把独立 binary IP 快核的收益直接代入；Boolean 分支仍有两路公开矩阵下推。不同子协议的阶段差仅由实测定位，不把全部差值归因于声明数量。",
        "- 两系统的 offline 表可同时驻留于同一测量进程。下表是 /usr/bin/time 的整个进程峰值 RSS，包含预处理、两系统、预热、验证及序列化，不是单系统 prover 内存。资源预检只覆盖启动前窗口，不能证明全程没有竞争。",
        "",
        "| 测量进程 | 整进程峰值 RSS GiB |",
        "|---|---:|",
    ]
    for row in common + ablations + production:
        lines.append(f'| {row["name"]} | {row["resources"]["maximum_process_rss_kib"]/(1024**2):.3f} |')
    evidence, status = data["evidence"], data["status"]
    lines += [
        "",
        f'- 初次 v3 构建失败：复用的 v2 Cargo 缓存与源码归档的零 mtime 导致 permutation 新接口未重编。随后依据 v2/v3 manifest 对变更的 {data["cache_recovery"]["touched_source_count"]} 个文件更新 mtime，保持源码内容及 SHA256 不变，触发重编后重新执行完整流程。初次失败是构建缓存问题，不计入协议验证失败或性能样本；results-build-cache-failure/、driver-build-cache-failure.log 和 cache-recovery.json 均保留并核验完整性。',
        "- 重复样本采用固定 witness 和同模式确定性 transcript，测量运行波动，不代表跨输入总体统计。未增加 ZK masking，也未接入递归 verifier；公开矩阵与 sigma 义务仍由完整 verifier 检查。",
        f'- 源码上传包记录 SHA256：`{evidence["source_archive_sha256"]}`；下载证据包 SHA256：`{evidence["evidence_archive_sha256"]}`。逐文件完整性已核验；二进制前后摘要检查由已成功完成的服务器 driver 执行，本地保留其摘要记录。',
        f'- 独立指定的源码上传 SHA256：{"已核对" if evidence["independent_source_digest_supplied"] else "未提供；仅核验 hash manifest 内的上传记录，未在本地重新解包源码包"}。',
        f'- 服务器 UTC：{status["started_utc"]} 至 {status["finished_utc"]}；所有记录阶段退出码为零。独立复算与 summary.json、ablation-summary.json、production-summary.json 逐字段完全一致。',
        "",
        "运行 `python3 analyze.py` 可重新核验本目录完整证据并生成 analysis.json 和本报告；可附加 `--source-sha256` 指定独立保存的上传包摘要。缺少日志或检查失败时不会生成新结果。",
    ]
    return "\n".join(lines) + "\n"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-sha256", default=SOURCE_SHA256,
                        help="expected uploaded archive digest (defaults to the recorded v3 upload)")
    args = parser.parse_args()
    manifest, evidence = check_evidence(args.source_sha256)
    status = load(LOGS / "status.json")
    require(status["state"] == "complete" and not status["failed"], "server run is incomplete/failed")
    completed = status["completed"]
    require(completed and all(x["exit_code"] == 0 for x in completed), "nonzero stage exit code")
    completed_names = {x["name"] for x in completed}
    require(len(completed_names) == len(completed), "duplicate completed stages")
    hardware = load(LOGS / "hardware.json")
    require(len(set(hardware["selected_cpus"])) == len(hardware["selected_cpus"]) == 64, "not 64 pinned CPUs")
    require(hardware["rustflags"] == "-C target-cpu=native" and hardware["rayon_num_threads"] == "64",
            "non-native or wrong-thread build environment")
    model = re.search(r"^model name\s*:\s*(.+)$", hardware["cpuinfo"], re.MULTILINE)
    require(model is not None, "missing CPU model")
    hardware["cpu_model"] = model.group(1).strip()
    leaves = sorted(int(re.fullmatch(r"common-leaves(\d+)\.stdout", path.name).group(1))
                    for path in LOGS.glob("common-leaves*.stdout"))
    require(leaves == [6, 10, 14], "expected three requested scales (leaves 6, 10, 14)")
    require({path.name for path in LOGS.glob("production*.stdout")} ==
            {f"production-leaves{x}.stdout" for x in leaves}, "unexpected production run set")
    require({path.name for path in LOGS.glob("ablation-*.stdout")} ==
            {"ablation-legacy.stdout", "ablation-cube.stdout"}, "unexpected ablation set")
    require({path.name for path in LOGS.glob("validation*.stdout")} ==
            {"validation-m22.stdout", "validation-legacy.stdout", "validation-cube.stdout"}, "unexpected validation set")
    common = [analyze_run(f"common-leaves{x}", "lockstep") for x in leaves]
    require([x["metadata"]["logical_bit_log"] for x in common] == [22, 26, 30], "wrong witness scales")
    production = [analyze_run(f"production-leaves{x}", None) for x in leaves]
    ablations = [analyze_run(f"ablation-{mode}", mode) for mode in ("legacy", "cube")]
    validations = [analyze_run(name, mode, validation=True) for name, mode in
                   (("validation-m22", "lockstep"), ("validation-legacy", "legacy"), ("validation-cube", "cube"))]
    for row in common + ablations:
        if row in ablations:
            require(row["metadata"]["log_leaves"] == leaves[-1], "ablation is not the maximum scale")
        prod = next(p for p in production if p["metadata"]["log_leaves"] == row["metadata"]["log_leaves"])
        same_production(row, prod)
    for row in validations:
        require(row["metadata"]["log_leaves"] == 3, "unexpected validation fixture size")
        require(row["commitment_blake3"] == validations[0]["commitment_blake3"], "validation mode commitments differ")
        require(all(row["metadata"][key] == validations[0]["metadata"][key] for key in PCS_KEYS),
                "validation mode PCS config differs")
    timing_runs = common + ablations + production
    require(len({(r["reps"], r["warmups"]) for r in timing_runs}) == 1, "timing schedules differ")
    all_runs = timing_runs + validations
    require({r["name"] for r in all_runs} <= completed_names, "measurement stages missing from status")
    for row in all_runs:
        row["resources"] = resource_record(row["name"], hardware, row in validations)
    check_summary("summary.json", [server_summary(r) for r in common], ("log_leaves",))
    check_summary("ablation-summary.json", [server_summary(r) for r in ablations], ("reduction",))
    check_summary("production-summary.json", [
        {"log_leaves": r["metadata"]["log_leaves"], "gate_count": r["metadata"]["gate_count"],
         "prove_with_commit_ms": r["production"]["prover_ms"], "proof_bytes": r["production"]["proof_bytes"],
         "timing_boundary": r["metadata"]["timing_boundary"]} for r in production], ("log_leaves",))
    tests = test_counts(status)
    recovery = cache_recovery()
    counts = {"timing_formal": sum(r["formal_proofs"] for r in timing_runs),
              "warmup": sum(r["warmup_proofs"] for r in timing_runs),
              "validation": sum(r["formal_proofs"] for r in validations)}
    counts["total"] = sum(counts.values())
    exempt = {"evidence-manifest.json", "evidence.tar.gz.sha256"}
    require(READ_PATHS - exempt <= set(manifest), f"used evidence not covered by manifest: {sorted(READ_PATHS - exempt - set(manifest))}")
    data = {"analysis_schema_version": 3, "common": common, "ablations": ablations, "production": production,
            "validations": validations, "native_test_commands": tests,
            "native_tests_passed": sum(x["passed"] for x in tests.values()), "proof_counts": counts,
            "all_production_pcs_and_commitments_match": True, "server_summaries_match": True,
            "hardware": hardware, "status": status, "evidence": evidence, "cache_recovery": recovery}
    rendered = report(data)
    encoded = json.dumps(data, ensure_ascii=False, indent=2, allow_nan=False) + "\n"
    # All validation and rendering must finish before replacing either deliverable.
    (ROOT / "analysis.json.tmp").write_text(encoded, encoding="utf-8")
    (ROOT / "RESULTS.md.tmp").write_text(rendered, encoding="utf-8")
    (ROOT / "analysis.json.tmp").replace(ROOT / "analysis.json")
    (ROOT / "RESULTS.md.tmp").replace(ROOT / "RESULTS.md")
    print(json.dumps({"native_tests_passed": data["native_tests_passed"], "proof_counts": counts,
                      "negative": {r["metadata"]["reduction"]: r["negative"] for r in validations},
                      "all_production_pcs_and_commitments_match": True, "server_summaries_match": True,
                      "lockstep_mikros_over_flock": [r["mikros_over_flock"] for r in common]}, indent=2))


if __name__ == "__main__":
    main()
