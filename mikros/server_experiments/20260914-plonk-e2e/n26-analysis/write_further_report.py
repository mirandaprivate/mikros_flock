"""Build the root N26 Chinese report only after complete v9/v10 evidence audits.

Local-only evidence reader. No benchmark, upload or N30 log is touched. Missing
v10 primary, native-test or profile evidence is an error; no partial report is
written. --check-only performs the same audit without writing the root log.
"""

import argparse
import hashlib
import json
from pathlib import Path
import re
import shlex

from summarize_direct import COMMON_FIELDS, CLAIM_COUNTS, FEATURES, MODES, PHASES, SYSTEMS, require
import summarize_engineering as engineering
import summarize_fusion as memory


ROOT = Path(__file__).resolve().parents[4]
EXPERIMENT = ROOT / "mikros/server_experiments/20260914-plonk-e2e"
OUTPUT = ROOT / "e2e_n26_further_optimization_compare_flock.log"
REPORTS = {"v9": EXPERIMENT / "n26-analysis/v9-engineering/comparison.json",
           "v10": EXPERIMENT / "n26-analysis/v10-memory/comparison.json"}
ARMS = {"v9": tuple(engineering.ARM_ENV), "v10": tuple(memory.ARM_ENV)}
TRACES = {"PCS_TRACE": "1", "LIG_PROVE_TRACE": "1", "E2E_PIOP_TRACE": "1"}
LABELS = {"gates_ms": "Gate PIOP", "copy_ms": "Copy PIOP", "witness_pcs_open_ms": "Witness PCS",
          "public_pcs_open_ms": "Public PCS", "commit_and_recommit_ms": "Commit/recommit",
          "release_ms": "释放", "serialization_ms": "序列化", "prover_ms": "Online 总计"}


def read_json(path):
    require(path.is_file(), f"缺少完整证据，拒绝生成报告：{path}")
    return json.loads(path.read_text(encoding="utf-8-sig"))


def sha(path):
    require(path.is_file(), f"缺少证据文件：{path}")
    return hashlib.sha256(path.read_bytes()).hexdigest()


def local_path(value):
    """Map saved Windows/WSL paths to this checkout, never to another project."""
    normalized = str(value).replace("\\", "/")
    marker = "mikros/server_experiments/20260914-plonk-e2e/"
    require(marker in normalized, f"证据路径不属于本实验：{value}")
    path = (EXPERIMENT / normalized.split(marker, 1)[1]).resolve()
    require(path.is_relative_to(EXPERIMENT.resolve()), f"证据路径越界：{value}")
    return path


def link(path, label=None):
    relative = path.resolve().relative_to(ROOT.resolve()).as_posix()
    return f"[{label or relative}]({relative})"


def normalized(value):
    if isinstance(value, dict):
        return {str(local_path(k)) if "mikros/server_experiments/20260914-plonk-e2e/" in k.replace("\\", "/") else k:
                normalized(v) for k, v in value.items()}
    if isinstance(value, list):
        return [normalized(v) for v in value]
    if isinstance(value, str) and "mikros/server_experiments/20260914-plonk-e2e/" in value.replace("\\", "/"):
        return str(local_path(value))
    return value


def audit_comparison(version, saved):
    validation = saved.get("validation", {})
    require(validation.get("all_comparisons_valid") is True
            and validation.get("all_arms_packed_direct_on") is True
            and validation.get("configuration_booleans_match_exact_environment") is True
            and validation.get("nontrace_only") is True
            and validation.get("minimum_samples_per_arm_case") == 3,
            f"{version} comparison 的 validation 不完整或失败")
    expected = ("four_arms_same_frozen_source_binary_seed_and_parameters",
                "full_proof_hash_bytes_piop_terminal_claims_commitments_and_pcs_match",
                "logical_and_ring_claim_counts_match") if version == "v9" else (
                "same_v10_source_binary_seed_and_parameters", "v9_arithmetic_optimizations_off",
                "full_proof_hash_bytes_piop_claims_commitments_pcs_equal", "logical_and_ring_counts_equal")
    require(all(validation.get(key) is True for key in expected), f"{version} 同源/证明/计数审计未通过")
    paths = [[local_path(p) for p in saved["arms"][arm]["directories"]] for arm in ARMS[version]]
    previous = "previous_on" if version == "v9" else "previous_reference"
    previous_paths = [local_path(p) for p in saved["arms"].get(previous, {}).get("directories", [])]
    fresh = (engineering.compare if version == "v9" else memory.compare)(*paths, previous_paths)
    require(normalized(saved) == normalized(fresh), f"{version} 已审计 comparison 与重新读取的原始证据不一致")
    for path, expected_sha in saved["evidence_sha256"].items():
        require(sha(local_path(path)) == expected_sha, f"{version} 证据 SHA-256 已改变：{path}")
    source_manifest = EXPERIMENT / version / "source_manifest.json"
    require(sha(source_manifest) == fresh["arms"]["reference"]["identity"]["source_manifest_sha256"],
            f"{version} 冻结源码清单 SHA-256 不一致")
    return fresh


def case_map(report):
    return {(case["mode"], case["system"]): case for case in report["cases"]}


def audit_folder(folder, identity, trace):
    inventory = read_json(folder / "evidence_sha256.json")
    require(isinstance(inventory, dict) and inventory, f"{folder} 缺少证据清单")
    for name, expected in inventory.items():
        file = (folder / name).resolve()
        require(file.is_relative_to(folder.resolve()) and sha(file) == expected,
                f"{folder} 证据文件校验失败：{name}")
    config, status = read_json(folder / "config.json"), read_json(folder / "status.json")
    receipt = read_json(folder / "build-receipt.json")
    require(status.get("state") == "complete"
            and status.get("source_hashes_verified_before_and_after") is True
            and status.get("binary_hashes_verified_before_and_after") is True,
            f"{folder} 状态未完成或未核验源码/二进制")
    for key in ("source_manifest_sha256", "binary_sha256"):
        require(receipt.get(key) == identity[key], f"{folder} {key} 与主样本不同")
    for key in ("source_manifest_sha256", "seed", "toolchain", "cpu_affinity", "rustflags", "allocator_policy"):
        require(config.get(key) == identity[key], f"{folder} 配置 {key} 与主样本不同")
    require(config.get("diagnostic_trace") is trace and config.get("threads") == 64,
            f"{folder} trace/thread 配置错误")
    require(config.get("measured_environment") == {**identity["measured_environment"], **(TRACES if trace else {})},
            f"{folder} 意外环境开关")
    for key in ("packed_direct", "direct_wp_simd", "gate_selector_simd", "gate_fusion", "wp_uninit"):
        if key in identity:
            require(config.get(key) is identity[key], f"{folder} 布尔开关 {key} 不匹配")
    cfg_text = (folder / "native-cfg.stdout.jsonl").read_text(encoding="utf-8")
    require(all(f'target_feature="{f}"' in cfg_text for f in FEATURES), f"{folder} native ISA 不完整")
    return config, status


def native_tests(version, report):
    folder = EXPERIMENT / version / "evidence/native-pilot14-reference"
    _, status = audit_folder(folder, report["arms"]["reference"]["identity"], False)
    commands = [c for c in status.get("commands", []) if c["id"].startswith("test-")]
    require(commands, f"{version} 缺少 native tests")
    records = []
    for command in commands:
        require(command.get("state") == "complete" and command.get("exit_code") == 0,
                f"{version} native test 失败：{command['id']}")
        file = folder / (command["id"] + ".stdout.jsonl")
        matches = re.findall(r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;", file.read_text(encoding="utf-8"))
        require(matches and all(int(failed) == 0 for _, failed, _ in matches), f"{file} 无成功测试结果")
        records.append((file, sum(int(passed) for passed, _, _ in matches)))
    return records


def profiles(version, report):
    result = {}
    cases = case_map(report)
    for arm in ("reference", "both"):
        folder = EXPERIMENT / version / f"evidence/profile-n26-{arm}"
        config, status = audit_folder(folder, report["arms"][arm]["identity"], True)
        require(config.get("levels") == [26] and config.get("repetitions") == 1,
                f"{folder} profile 规模错误")
        summary = read_json(folder / "summary.json")
        require(summary.get("config") == config and summary.get("all_comparisons_valid") is True
                and summary.get("all_requested_cases_complete") is True,
                f"{folder} profile 未完成")
        completed = {case["id"]: case for case in status["cases"]}
        for mode in MODES:
            for system in SYSTEMS:
                stem = f"n26-{mode}-{system}"
                require(completed.get(stem, {}).get("exit_code") == 0
                        and completed[stem].get("state") == "complete", f"{folder}/{stem} 未完成")
                rows = [json.loads(line) for line in (folder / (stem + ".stdout.jsonl")).read_text().splitlines() if line]
                samples = [row for row in rows if row.get("type") == "sample"]
                require(len(samples) == 1, f"{folder}/{stem} profile 样本数错误")
                sample = samples[0]
                reference = cases[mode, system]["arms"][arm]["samples"][0]
                require(all(sample.get(key) == reference[key] for key in
                            ("mode", "system", "log_gates", "instances", "gates_per_instance", "total_gates", "threads", "seed")),
                        f"{folder}/{stem} profile 电路规模/seed/thread 不匹配")
                for field in engineering.PROOF_FIELDS:
                    require(sample.get(field) == reference[field], f"{folder}/{stem} profile {field} 不匹配")
                require(all(sample.get(field) is True for field in ("verified", "transcript_match", "pcs_config_match", "native_pcs_identity_transport")),
                        f"{folder}/{stem} profile 验证失败")
                require(all(sample["times"].get(field) == reference["times"][field] for field in CLAIM_COUNTS),
                        f"{folder}/{stem} profile claims 数量改变")
                log = folder / (stem + ".stderr.log")
                result[arm, mode, system] = {"sample": sample, "log": log, "text": log.read_text(encoding="utf-8")}
    return result


def metric(text, pattern, required=True):
    values = re.findall(pattern, text)
    require(values or not required, f"profile 缺少必要指标：{pattern}")
    return float(values[0]) if values else None


def seconds(summary, key="prover_ms"):
    t = summary["times_ms"][key]
    return f"{t['median']/1000:.4f} [{t['min']/1000:.4f}, {t['max']/1000:.4f}]"


def audit_all():
    # Check both report paths before re-reading hundreds of existing v9 files.
    saved = {version: read_json(path) for version, path in REPORTS.items()}
    reports = {version: audit_comparison(version, data) for version, data in saved.items()}
    old_id, new_id = (reports[v]["arms"]["reference"]["identity"] for v in ("v9", "v10"))
    for key in old_id:
        if key not in ("source_manifest_sha256", "binary_sha256"):
            require(old_id[key] == new_id[key], f"v9/v10 reference {key} 不同")
    old_cases, new_cases = (case_map(reports[v]) for v in ("v9", "v10"))
    for key in old_cases:
        old, new = (cases[key]["arms"]["reference"]["samples"][0] for cases in (old_cases, new_cases))
        require(all(old[field] == new[field] for field in engineering.PROOF_FIELDS), f"v9/v10 {key} 基准完整证明或输入改变")
        require(all(old["times"][field] == new["times"][field] for field in CLAIM_COUNTS), f"v9/v10 {key} counts 改变")
    tests = {version: native_tests(version, report) for version, report in reports.items()}
    traces = {version: profiles(version, report) for version, report in reports.items()}
    require((ROOT / "mikros/E2E_N26_LATEX_ENGINEERING.md").is_file(), "缺少 LaTeX/工程边界说明")
    return reports, tests, traces


def render(reports, tests, traces):
    lines = ["# N26 进一步工程优化与 FLOCK 对比", "",
             "本报告仅针对 64 threads、N=2^26：single 为一个 2^26 门 binary 电路；multiple 为 128 个相同 wiring、独立输入的 2^26 门电路，总计 2^33 门。双方统一使用 FLOCK 原生 commit、merged_tensor/open_batch_merged 与 Ligerito Fast PCS。", "",
             "Online 包括承诺/重新承诺、online 转换、PIOP、PCS opening、释放和序列化；offline 输入/公共表预处理不计入，verify 单独测量且不计入 online prover time。RSS 是整进程 MaxRSS（包含 offline），不是纯 online prover peak。主样本为每轮四臂、每臂每 mode/system 至少三个独立新进程；trace 单次诊断不进入主样本统计。阶段中位数不相加，不据三个样本宣称统计显著性或机械宣称达到 2×。", "",
             "v9：reference、WP SIMD、selector SIMD、both。v10：reference、gate fusion、W′ 免预先清零、both；v10 关闭 v9 两个算术开关。PackedDirect 全部开启。W′ 优化对 Mikros/FLOCK 同样开放，以双方实测的相对收益判断。reference/both 是消融标签，不是默认生产切换；新 flags 保持 opt-in。", ""]
    selected = case_map(reports["v10"])
    lines += [
        "## 本轮采用建议", "",
        "对 N26 multiple，建议在既有 packed-direct 路线上先只启用 `FLOCK_WP_UNINIT=1`（harness：`--packed-direct --wp-uninit`）。它有明确的 W′ 局部计时改善，也是本轮四臂中 multiple online 中位数最低的一臂；同一个选项对 FLOCK 同样启用。旧的两个算术开关继续关闭，gate fusion 保留独立实验开关，不能假定全部开启最好。所有新内核仍为 opt-in，未将这组 N26 结果外推为其他规模的默认策略。", "",
        "| 仅启用 W′ 首次写入 | Mikros online (s) | FLOCK online (s) | F/M |",
        "|---|---:|---:|---:|",
    ]
    for mode in MODES:
        m, f = (selected[mode, system]["arms"]["wp_uninit"] for system in SYSTEMS)
        ratio = f["times_ms"]["prover_ms"]["median"] / m["times_ms"]["prover_ms"]["median"]
        lines.append(f"| {mode} | {seconds(m)} | {seconds(f)} | {ratio:.3f}× |")
    m = selected["multiple", "mikros"]["arms"]
    old, new = (m[a]["times_ms"]["prover_ms"]["median"] for a in ("reference", "wp_uninit"))
    old_pcs, new_pcs = (m[a]["times_ms"]["witness_pcs_open_ms"]["median"] for a in ("reference", "wp_uninit"))
    lines += ["", f"Mikros multiple online 中位数观测为 {old/1000:.4f}→{new/1000:.4f}s（减少 {100*(1-new/old):.2f}%）；目标 witness PCS 为 {old_pcs/1000:.4f}→{new_pcs/1000:.4f}s。未修改的上游阶段也有波动，不能把全部 online 差值归因于 W′ 内核。本轮尚未达到整体 2× FLOCK。", ""]
    for version, report in reports.items():
        cases = case_map(report)
        lines += [f"## {version} 主样本", "", link(REPORTS[version]) + "；" + link(REPORTS[version].with_suffix(".md")), "",
                  "时间为中位数 [最小值, 最大值] 秒；F/M>1 表示 Mikros 较快。", "",
                  "| 模式 | 分支 | Mikros 总时间 | FLOCK 总时间 | F/M | 相对 reference 的 F/M 变化 |",
                  "|---|---|---:|---:|---:|---:|"]
        for mode in MODES:
            ratios = {}
            for arm in ARMS[version]:
                m, f = (cases[mode, system]["arms"][arm] for system in SYSTEMS)
                ratios[arm] = f["times_ms"]["prover_ms"]["median"] / m["times_ms"]["prover_ms"]["median"]
                gain = 100 * (ratios[arm] / ratios["reference"] - 1)
                lines.append(f"| {mode} | {arm} | {seconds(m)} | {seconds(f)} | {ratios[arm]:.3f}× | {gain:+.2f}% |")
        lines += ["", "reference/both 各阶段中位数和范围：", "",
                  "| 模式 | 阶段 | M reference | M both | F reference | F both |", "|---|---|---:|---:|---:|---:|"]
        for mode in MODES:
            for phase in PHASES:
                values = [seconds(cases[mode, system]["arms"][arm], phase)
                          for system in SYSTEMS for arm in ("reference", "both")]
                lines.append(f"| {mode} | {LABELS[phase]} | {' | '.join(values)} |")
        targeted = [("wp", "witness_pcs_open_ms"), ("gate", "gates_ms")] if version == "v9" else [
                    ("fusion", "gates_ms"), ("wp_uninit", "witness_pcs_open_ms")]
        lines += ["", "单独开关对应阶段（正百分比表示耗时减少）：", "",
                  "| 模式 | 开关 | 系统 | 目标阶段 | reference→单独开关 (s) | 时间减少 |", "|---|---|---|---|---:|---:|"]
        for mode in MODES:
            for arm, phase in targeted:
                for system in SYSTEMS:
                    old, new = (cases[mode, system]["arms"][a]["times_ms"][phase]["median"] for a in ("reference", arm))
                    lines.append(f"| {mode} | {arm} | {system} | {LABELS[phase]} | {old/1000:.4f}→{new/1000:.4f} | {100*(old-new)/old:+.2f}% |")
        control = ("single 对 v9 两个开关均是负对照：不调用 multiple selector 内核，且所有 opening 为 ring claims，不执行 direct-W′ SIMD。其变化不能解释为这些目标内核的收益。"
                   if version == "v9" else
                   "single 只对 gate fusion 是负对照。共享 WP_UNINIT 不要求 direct claim，因此仍会作用于 single 的矩形 W′（该实验每条 W′ 为 2^20 words，即 16 MiB）；不能把 v10 所有 single 分支都称为无效路径。")
        lines += ["", control, ""]

    cases9 = case_map(reports["v9"])
    m9 = cases9["multiple", "mikros"]["arms"]
    require(all(len(m9[arm]["samples"]) % 2 == 1 for arm in ("reference", "both")),
            "v9 总中位数的具体样本归因要求奇数样本数")
    old, new = (sorted(m9[arm]["samples"], key=lambda s: s["times"]["prover_ms"])[len(m9[arm]["samples"]) // 2]
                for arm in ("reference", "both"))
    total_saved = old["times"]["prover_ms"] - new["times"]["prover_ms"]
    commit_saved = old["times"]["commit_and_recommit_ms"] - new["times"]["commit_and_recommit_ms"]
    require(total_saved != 0, "v9 总差值为0，不能计算commit占比")
    lines += ["## v9 负结果与控制路径归因", "",
              f"Mikros multiple 总中位数 reference {m9['reference']['times_ms']['prover_ms']['median']/1000:.4f}s，WP {m9['wp']['times_ms']['prover_ms']['median']/1000:.4f}s，gate {m9['gate']['times_ms']['prover_ms']['median']/1000:.4f}s，both {m9['both']['times_ms']['prover_ms']['median']/1000:.4f}s。单独 WP 的 witness PCS 基本不变，selector 的目标 gate 阶段没有改善；两个开关继续保留默认关闭。", "",
              f"不相加各阶段中位数。取产生总中位数的具体样本 {Path(old['directory']).name} 与 {Path(new['directory']).name}：总差值 {total_saved:.3f}ms，未改的 commit/recommit 差值 {commit_saved:.3f}ms，占差值 {100*commit_saved/total_saved:.1f}%。这是样本差分，不能视为该阶段的因果贡献。", "",
              "| 两个具体样本的阶段 | reference (ms) | both (ms) | both-reference (ms) |", "|---|---:|---:|---:|"]
    for phase in PHASES:
        a, b = old["times"][phase], new["times"][phase]
        lines.append(f"| {LABELS[phase]} | {a:.3f} | {b:.3f} | {b-a:+.3f} |")
    f9 = cases9["multiple", "flock"]["arms"]
    a, b = (f9[arm]["times_ms"]["commit_and_recommit_ms"]["median"] for arm in ("reference", "gate"))
    lines += ["", f"另一个负对照：gate-only 对 FLOCK 不改变内核，但其 commit/recommit 从 {a/1000:.4f}s 变为 {b/1000:.4f}s（减少 {100*(a-b)/a:.2f}%）。single 的目标分支也不执行，但阶段仍波动。both 的较小总时间不能自动归因于协同优化。", ""]

    lines += ["## v10 总差值的实际样本分解", "",
              "下面比较各系统产生总时间中位数的两次实际运行，不相加不同运行的阶段中位数。该分解说明观测差值出现在哪些阶段，不等于因果归因。", "",
              "| 系统 | reference / wp_uninit 样本 | 总节省 (ms) | Witness PCS 节省 (ms) | 未改 commit/gate/copy 合计节省 (ms) |",
              "|---|---|---:|---:|---:|"]
    for system in SYSTEMS:
        groups = selected["multiple", system]["arms"]
        require(all(len(groups[a]["samples"]) % 2 == 1 for a in ("reference", "wp_uninit")),
                "v10 实际总中位数归因要求奇数样本数")
        old, new = (sorted(groups[a]["samples"], key=lambda s: s["times"]["prover_ms"])[len(groups[a]["samples"]) // 2]
                    for a in ("reference", "wp_uninit"))
        saved = {p: old["times"][p] - new["times"][p] for p in PHASES}
        upstream = sum(saved[p] for p in ("commit_and_recommit_ms", "gates_ms", "copy_ms"))
        lines.append(f"| {system} | {Path(old['directory']).name} / {Path(new['directory']).name} | {saved['prover_ms']:.3f} | {saved['witness_pcs_open_ms']:.3f} | {upstream:.3f} |")
    lines += ["", "因此 Mikros 较大的 online 降幅不能全部当作相对 FLOCK 的内核优势；独立 profile 显示双方 W′ 局部都约省 0.76s。此处推荐依据同时包含明确的局部改善和完整 E2E 样本，而非把全部上游差值计入优化收益。", ""]

    lines += ["## 单次 profile：局部计算与整段时间分开", "",
              "以下均为独立单次 trace，不能当作主样本或显著性证据。W′ generation 包含 allocation/zero-init 与 fill；免预先清零后，物理页首次触碰、缺页及初始化成本会进入 fill，不能只看 allocation≈0 就宣称整个内存成本消失。", "",
              "| 版本 | 系统 | 分支 | W′ allocate/zero-init (ms) | W′ fill (ms) | W′ generation (ms) | q′ fold (ms) | Witness PCS (ms) |", "|---|---|---|---:|---:|---:|---:|---:|"]
    for version in reports:
        for system in SYSTEMS:
            for arm in ("reference", "both"):
                p = traces[version][arm, "multiple", system]
                values = [metric(p["text"], r"W-prime allocate/zero-init \([^\n]*\):\s*([\d.]+) ms", version == "v10"),
                          metric(p["text"], r"W-prime fill \([^\n]*\):\s*([\d.]+) ms", version == "v10"),
                          metric(p["text"], r"W-prime generation \([^\n]*\):\s*([\d.]+) ms"),
                          metric(p["text"], r"q-prime fold:\s*([\d.]+) ms"), p["sample"]["times"]["witness_pcs_open_ms"]]
                cells = ["未单独计时" if x is None else f"{x:.3f}" for x in values]
                lines.append(f"| {version} | {system} | {arm} | {' | '.join(cells)} |")
    lines += ["", "v10 Mikros multiple gate 的 wrapper 与 sumcheck 内部：", "",
              "| 分支 | virtual product | sumcheck wrapper | virtual product release | WH | message | fold | fused fold+message | gate total |",
              "|---|---:|---:|---:|---:|---:|---:|---:|---:|"]
    for arm in ("reference", "both"):
        p = traces["v10"][arm, "multiple", "mikros"]
        values = [metric(p["text"], rf"gate=mikros-multiple stage={stage} ms=([\d.]+)")
                  for stage in ("virtual_product", "sumcheck", "virtual_product_release", "weighted_hadamard")]
        values += [metric(p["text"], rf"(?:^|\s){field}=([\d.]+)")
                   for field in ("message_ms", "fold_ms", "fused_fold_message_ms")]
        values.append(p["sample"]["times"]["gates_ms"])
        lines.append(f"| {arm} | " + " | ".join(f"{v:.3f}" for v in values) + " |")
    lines += ["", "单位均为 ms；内部 message/fold/fused 是 sumcheck wrapper 的子阶段，不包含全部 Vec 析构、其他 book-keeping，不能把内部计时之和当作整个 sumcheck。wrapper 又包含于 gate total，不能将父子阶段重复相加。v10 组合的 E2E 表现还需与仅 fusion、仅 W′ 的主样本对照；共享 W′ 改动也可能让 FLOCK 获益更多，不预设结论。", ""]

    lines += ["## 正确性、资源和证据链", "",
              "每轮四臂使用相同冻结源码、binary、seed、线程亲和与 PCS 参数；同一个 mode/system 跨分支的完整 proof hash/bytes、PIOP hash、terminal claims hash、commitments、逻辑/ring claim 数一致。v9/v10 reference 也逐项核对一致，只有 source/binary 可以改变。不要求 Mikros 与 FLOCK 的证明相同；双方相同的是输入承诺和 PCS 参数。每一个主样本均 verified、transcript_match、pcs_config_match、native_pcs_identity_transport=true。", "",
              "| 版本 | 模式 | 系统 | 分支 | 样本数 | Proof bytes | Logical W/P | Ring W/P | Verify ms 中位数[范围] | 整进程 MaxRSS GiB 中位数[范围] |",
              "|---|---|---|---|---:|---:|---:|---:|---:|---:|"]
    for version, report in reports.items():
        for case in report["cases"]:
            for arm in ARMS[version]:
                s = case["arms"][arm]
                c = s["claim_counts"]
                cells = [f"{s[k]['median']:.3f} [{s[k]['min']:.3f},{s[k]['max']:.3f}]" for k in ("verify_ms", "rss_gib")]
                lines.append(f"| {version} | {case['mode']} | {case['system']} | {arm} | {s['sample_count']} | {s['proof_bytes']} | {c['witness_claims']}/{c['public_claims']} | {c['witness_ring_claims']}/{c['public_ring_claims']} | {' | '.join(cells)} |")
    for version in reports:
        lines += ["", f"{version} native 测试通过数：**{sum(count for _, count in tests[version])}**，从每个 test 命令的结果实际累计（可能有覆盖重叠），所有命令退出码为 0。"]
        lines += [f"- {link(file)}：{count} passed" for file, count in tests[version]]
        primary_count = sum(case["arms"][arm]["sample_count"] for case in reports[version]["cases"] for arm in ARMS[version])
        lines += [f"{version} 本轮主样本共 {primary_count} 个完整 proof，逐样本验证标志均通过；不含前一版本样本和单次 profile。"]
    lines += ["", "完整证明和 PCS 元数据（所有分支及两个版本相同，故按 mode/system 展示一次）："]
    for case in reports["v10"]["cases"]:
        sample = case["arms"]["reference"]["samples"][0]
        lines += ["", f"### {case['mode']} / {case['system']}", "", "```json",
                  json.dumps({k: sample[k] for k in engineering.PROOF_FIELDS}, ensure_ascii=False, indent=2), "```"]
    lines += ["", "## 每次主样本与复现", "",
              "以下原始 online 数值单位 ms，按证据目录顺序，不重新排序或剔除较慢样本。全部原始阶段 ms、资源和验证字段保留在各 comparison.json 的 samples 中。"]
    for version, report in reports.items():
        identity = report["arms"]["reference"]["identity"]
        receipt = local_path(report["arms"]["reference"]["directories"][0]) / "build-receipt.json"
        lines += ["", f"### {version} source / binary / evidence", "",
                  f"源码清单：{link(EXPERIMENT / version / 'source_manifest.json')}；SHA-256 `{identity['source_manifest_sha256']}`。",
                  f"binary SHA-256 `{identity['binary_sha256']}`；{link(receipt)}（由每次 build-receipt 和前后校验记录认证，分析脚本不声称重新读取服务器 binary）。",
                  f"comparison SHA-256 `{sha(REPORTS[version])}`；{link(REPORTS[version])} 内列出每项 evidence SHA-256。"]
        for case in report["cases"]:
            for arm in ARMS[version]:
                samples = case["arms"][arm]["samples"]
                lines += [f"- {case['mode']} / {case['system']} / {arm}：{[s['times']['prover_ms'] for s in samples]}；" +
                          "、".join(link(local_path(s["directory"]) / "summary.json", Path(s["directory"]).name) for s in samples)]
        for arm in ARMS[version]:
            folder = local_path(report["arms"][arm]["directories"][0])
            command_file = folder / "n26-multiple-mikros.command.json"
            command = read_json(command_file)
            lines += ["", f"{version}/{arm} 复现命令示例：{link(command_file)}（其他 mode/system 的精确 command.json 位于同目录）。", "", "```sh",
                      "cd " + shlex.quote(command["cwd"]),
                      "env " + " ".join(shlex.quote(f"{k}={v}") for k, v in command["env"].items()) + " " + shlex.join(command["command"]), "```"]
        for arm in ("reference", "both"):
            for system in SYSTEMS:
                log = traces[version][arm, "multiple", system]["log"]
                lines += [f"- 单次 profile：{link(log)}；SHA-256 `{sha(log)}`。"]
    lines += ["", "## 实现边界与后续方向", "",
              "开关与代码：FLOCK_DIRECT_WP_SIMD / FLOCK_WP_UNINIT 位于 " + link(ROOT / "crates/flock-core/src/pcs.rs") +
              "；MIKROS_GATE_SELECTOR_SIMD / MIKROS_GATE_FUSION 位于 " + link(ROOT / "mikros/src/multiple_gate.rs") + "。所有新增开关仍默认关闭。", "",
              "LaTeX 的有效入口、GF256/κ=128 与正文具体 t=13/κ调整规则的差异见 " + link(ROOT / "mikros/E2E_N26_LATEX_ENGINEERING.md") + "；本轮不改变协议、域或 PCS 参数。", "",
              "后续尚未实现的候选：virtual product 初轮按块生成，避免完整 1 GiB 初始 M；WH 张量权重按块生成，核算重复乘法与少写少读的取舍；copy 两个终点共用一次 witness 读取。gate fold+message fusion 已在 v10 实现并参与本报告实验，不重复列为未来工作。", ""]
    return "\n".join(lines)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check-only", action="store_true", help="审计并渲染验证，但不写根目录日志")
    args = parser.parse_args()
    try:
        reports, tests, traces = audit_all()
        text = render(reports, tests, traces)
    except (ValueError, OSError, KeyError, TypeError) as error:
        parser.exit(2, f"证据未完整通过审计，未写报告：{error}\n")
    if args.check_only:
        print("全部输入和报告渲染检查通过；未写文件。")
        return
    OUTPUT.write_text(text, encoding="utf-8")
    print(OUTPUT)


if __name__ == "__main__":
    main()
