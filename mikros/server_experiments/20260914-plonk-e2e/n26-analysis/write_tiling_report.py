"""Fail-closed local N26 report writer; never runs or uploads experiments.

Requires the v11 source/archive receipts, all six diagnostic profiles, both
N14 pilots, and complete three-process primary arms. --recommend selects an
explicit measured arm; the writer never chooses a noisy fastest sample.
--check-only performs every audit but creates no report files.
"""

import argparse
import ast
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import sys
import tarfile
import tempfile

import summarize_tiling as tiling
from summarize_direct import CLAIM_COUNTS, COMMON_FIELDS, FEATURES, MODES, PHASES, SYSTEMS, require
from summarize_engineering import PROOF_FIELDS
from write_further_report import ROOT, EXPERIMENT, read_json, sha, link, case_map, seconds, LABELS

sys.path.insert(0, str(EXPERIMENT))
from aggregate_results import validate_case
from common import cases, source_path, CRITICAL_SOURCES, atomic_text


VERSION = EXPERIMENT / "v11"
EVIDENCE = VERSION / "evidence"
OUTPUT = ROOT / "e2e_n26_tiling_optimization_compare_flock.log"
DETAIL = EXPERIMENT / "n26-analysis/v11-tiling"
TRACES = {"PCS_TRACE": "1", "LIG_PROVE_TRACE": "1", "E2E_PIOP_TRACE": "1"}
HARNESS_LOGS = {f"harness-v11-{name}.log" for name in ("pilot", "profiles", "primary")}
WH_FIELDS = ("weights_ms", "aggregate_ms", "certificate_ms", "pack_ms", "weights_release_ms", "ip_with_release_ms")
GATE_FIELDS = ("message_ms", "fold_ms", "fused_fold_message_ms")
GATE_STAGES = ("virtual_product", "sumcheck", "virtual_product_release", "weighted_hadamard")
N30 = tuple(ROOT / f"e2e_{mode}_compare_flock.log" for mode in MODES)


def safe_member(name):
    path = PurePosixPath(name)
    require(name and not path.is_absolute() and ".." not in path.parts and "\\" not in name
            and all(":" not in part for part in path.parts),
            f"非法 archive 路径：{name}")
    require(path.as_posix() == name and path.parts[0] not in (".git", "target"),
            f"非规范 archive 路径：{name}")
    return path


def archive_files(path):
    """Read regular members only; no extraction or symlink following occurs."""
    result = {}
    with tarfile.open(path, "r:gz") as archive:
        for member in archive.getmembers():
            safe_member(member.name.rstrip("/"))
            if member.isdir():
                continue
            require(member.isfile() and member.name not in result, f"重复或非普通 archive 文件：{member.name}")
            stream = archive.extractfile(member)
            require(stream is not None, f"无法读取 archive member：{member.name}")
            result[member.name] = hashlib.sha256(stream.read()).hexdigest()
    return result


def source_audit():
    manifest_path = VERSION / "source_manifest.json"
    manifest = read_json(manifest_path)
    hashes = manifest.get("files_sha256", {})
    original = read_json(EXPERIMENT / "v1/source_manifest.json")["files_sha256"]
    require(len(hashes) == 384 and set(hashes) == set(original) and set(CRITICAL_SOURCES) <= set(hashes),
            "v11 源码不符合已授权的 384 路径清单")
    for name, expected in hashes.items():
        require(sha(source_path(ROOT.resolve(), name)) == expected, f"当前源码与冻结快照不同：{name}")
    receipt = read_json(VERSION / "upload_receipt.json")
    archive = VERSION / "source.tar.gz"
    require(receipt.get("server_sha256_verified") is True and receipt.get("same_allowlist_as_v1") is True
            and receipt.get("source_files") == 384, "源码上传校验回执缺失或失败")
    digest = sha(archive)
    require(receipt.get("archive_sha256") == digest and receipt.get("archive_bytes") == archive.stat().st_size
            and receipt.get("source_manifest_sha256") == sha(manifest_path), "源码包/清单/回执 SHA-256 不同")
    checksum = (VERSION / "source.tar.gz.sha256").read_text(encoding="utf-8-sig").split()
    require(checksum and checksum[0] == digest, "源码包 checksum 文件不同")
    require(archive_files(archive) == {**hashes, "source_manifest.json": sha(manifest_path)},
            "源码 archive 内容或路径与清单不同")
    return {"source_files": len(hashes), "source_manifest_sha256": sha(manifest_path),
            "source_archive_sha256": digest, "source_archive_bytes": archive.stat().st_size,
            "current_source_matches_snapshot": True, "upload_receipt": receipt}


def primary_paths():
    result = {}
    for arm in tiling.ARM_ENV:
        found = set(EVIDENCE.glob(f"primary-{arm}-n26-r*"))
        expected = {EVIDENCE / f"primary-{arm}-n26-r{run}" for run in range(1, 4)}
        if found:
            require(found == expected and all(folder.is_dir() for folder in found),
                    f"{arm} 主样本不完整：必须恰有 r1/r2/r3，禁止静默忽略残缺分支")
            result[arm] = sorted(found)
    require("reference" in result and len(result) >= 2, "缺少 reference 和至少一个完整优化主样本分支")
    return result


def evidence_archive_audit(directories):
    archive = VERSION / "evidence-v11.tar.gz"
    receipt = read_json(VERSION / "evidence_receipt.json")
    digest = sha(archive)
    require(receipt.get("server_sha256_verified") is True and receipt.get("archive_sha256") == digest
            and receipt.get("archive_bytes") == archive.stat().st_size, "证据 archive 下载回执或 SHA-256 不同")
    archived = archive_files(archive)
    allowed_dirs = {folder.name for folder in directories}
    require(allowed_dirs <= {PurePosixPath(name).parts[0] for name in archived}, "archive 缺少实验目录")
    require(HARNESS_LOGS <= set(archived), "archive 缺少三个 harness 日志")
    for name in archived:
        parts = PurePosixPath(name).parts
        require((len(parts) == 1 and name in HARNESS_LOGS) or
                (len(parts) == 2 and parts[0] in allowed_dirs), f"archive 出现非预期证据路径：{name}")
    extracted = {}
    for file in EVIDENCE.rglob("*"):
        require(not file.is_symlink(), f"证据路径不能是符号链接：{file}")
        if file.is_file():
            extracted[file.relative_to(EVIDENCE).as_posix()] = sha(file)
    require(archived == extracted, "证据 archive 与本地提取文件路径/内容不同")
    return {"evidence_archive_sha256": digest, "evidence_archive_bytes": archive.stat().st_size,
            "evidence_regular_files": len(archived), "member_sha256": archived, "download_receipt": receipt}


def identity_for(reference, arm):
    extra = tiling.ARM_ENV[arm]
    return {**reference,
            "measured_environment": {tiling.ROUTE_ENV: "1", tiling.WP_ENV: "1", **extra},
            **{key: env in extra for key, env in tiling.CONFIG_ENV.items()}}


def audit_folder(folder, identity, level, trace):
    inventory = read_json(folder / "evidence_sha256.json")
    files = {file.name: file for file in folder.iterdir() if file.is_file() and file.name != "evidence_sha256.json"}
    require(isinstance(inventory, dict) and set(inventory) == set(files), f"{folder} 证据清单路径不完整")
    require(all(sha(files[name]) == expected for name, expected in inventory.items()), f"{folder} 证据文件 SHA-256 不符")
    config, status = read_json(folder / "config.json"), read_json(folder / "status.json")
    receipt, summary = read_json(folder / "build-receipt.json"), read_json(folder / "summary.json")
    require(status.get("state") == summary.get("run_state") == "complete"
            and summary.get("config") == config and summary.get("all_comparisons_valid") is True
            and summary.get("all_requested_cases_complete") is True, f"{folder} 未完成或 summary/config 不同")
    for key in ("source_hashes_verified_before_and_after", "binary_hashes_verified_before_and_after"):
        require(status.get(key) is True, f"{folder} 未核验 {key}")
    require(summary.get("source_hashes_verified_before_and_after") is True, f"{folder} summary 未核验源码")
    for key in ("source_manifest_sha256", "binary_sha256"):
        require(receipt.get(key) == identity[key], f"{folder} receipt {key} 不同")
    for key in ("source_manifest_sha256", "seed", "toolchain", "cpu_affinity", "rustflags", "allocator_policy", "disabled_optimizations"):
        require(config.get(key) == identity[key], f"{folder} config {key} 不同")
    require(receipt.get("toolchain") == config["toolchain"] and receipt.get("rustflags") == config["rustflags"],
            f"{folder} 构建 receipt 工具链不同")
    require(config.get("levels") == [level] and config.get("repetitions") == 1
            and config.get("threads") == 64 and config.get("diagnostic_trace") is trace
            and config.get("piop_fingerprints") is True, f"{folder} 规模/thread/trace/fingerprint 不符")
    for key in ("packed_direct", *tiling.OLD_FLAGS, *tiling.CONFIG_ENV):
        require(config.get(key) is identity[key], f"{folder} 布尔开关 {key} 不匹配")
    require(config.get("measured_environment") == {**identity["measured_environment"], **(TRACES if trace else {})},
            f"{folder} 环境开关不同")
    native = (folder / "native-cfg.stdout.jsonl").read_text(encoding="utf-8")
    require(all(f'target_feature="{feature}"' in native for feature in FEATURES), f"{folder} native ISA 不完整")
    commands = status.get("commands", [])
    require(commands and all(c.get("state") == "complete" and c.get("exit_code") == 0 for c in commands),
            f"{folder} 存在失败/未完成命令")
    expected = cases([level])
    events = status.get("cases", [])
    require(len(events) == 4 and {e["id"] for e in events} == {c["id"] for c in expected}
            and all(e.get("state") == "complete" and e.get("exit_code") == 0 for e in events), f"{folder} case 状态不完整")
    saved = {case["id"]: case for case in summary.get("cases", [])}
    require(len(summary.get("cases", [])) == len(saved) == 4, f"{folder} summary case 数量错误")
    fresh = {}
    for case in expected:
        validated = validate_case(folder, case, 1, config["seed"])
        require(saved.get(case["id"]) == validated, f"{folder}/{case['id']} 重新 validate_case 后与 summary 不同")
        fresh[case["mode"], case["system"]] = validated
    for mode in MODES:
        left, right = (fresh[mode, system]["samples"][0] for system in SYSTEMS)
        require(all(left[field] == right[field] for field in COMMON_FIELDS), f"{folder}/{mode} M/F 承诺或 PCS 参数不同")
    return {"config": config, "status": status, "cases": fresh}


def match_proof(left, right, label):
    require(all(left[field] == right[field] for field in PROOF_FIELDS), f"{label} 完整证明/PIOP/声明/承诺/PCS 不同")
    require(all(left["times"][field] == right["times"][field] for field in CLAIM_COUNTS), f"{label} 声明数改变")


def native_test_records(folder, validated):
    tree = ast.parse((EXPERIMENT / "run_server.py").read_text(encoding="utf-8"))
    definitions = [node for node in ast.walk(tree) if isinstance(node, ast.Assign)
                   and any(isinstance(target, ast.Name) and target.id == "test_targets" for target in node.targets)]
    require(len(definitions) == 1, "无法识别冻结 harness 的 native test 列表")
    expected = {"test-" + name for name, _ in ast.literal_eval(definitions[0].value)}
    commands = [c for c in validated["status"]["commands"] if c["id"].startswith("test-")]
    require(len(commands) == len(expected) and {c["id"] for c in commands} == expected, "native test 目标缺失或重复")
    records = []
    for command in commands:
        file = folder / (command["id"] + ".stdout.jsonl")
        matches = re.findall(r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;", file.read_text(encoding="utf-8"))
        require(matches and all(int(failed) == 0 for _, failed, _ in matches), f"{file} 缺少成功测试结果")
        passed, ignored = (sum(int(row[index]) for row in matches) for index in (0, 2))
        require(passed > 0, f"{file} 没有实际匹配到测试")
        records.append({"command": command["id"], "passed": passed, "ignored": ignored, "sha256": sha(file)})
    return records


def primary_order(text, arms):
    """Read the logged schedule and require each observed begin/end in order."""
    lines = [line for line in text.splitlines() if line.strip()]
    require(lines, "主实验 harness 日志为空")
    header = json.loads(lines[0])
    require(header.get("trace") is False and header.get("threads") == 64
            and header.get("per_arm_repetitions") == 3
            and header.get("common") == ["--packed-direct", "--wp-uninit"],
            "主实验 harness 顺序头部配置错误")
    orders = header.get("fresh_process_orders")
    require(isinstance(orders, list) and len(orders) == 3
            and all(isinstance(order, list) and len(order) == len(arms) and set(order) == set(arms)
                    for order in orders), "主实验每轮顺序缺少/重复分支")
    expected = []
    for run, order in enumerate(orders, 1):
        for arm in order:
            expected.extend((f"PRIMARY_BEGIN {arm} {run}", f"PRIMARY_COMPLETE {arm} {run}"))
    expected.append("ALL_PRIMARY_COMPLETE")
    observed = [line for line in lines if line.startswith(("PRIMARY_BEGIN ", "PRIMARY_COMPLETE "))
                or line == "ALL_PRIMARY_COMPLETE"]
    require(observed == expected, "实际 PRIMARY_BEGIN/COMPLETE 与声明交错顺序不同或未完成")
    return {"header": header, "orders": orders, "observed_markers": observed}


def key_values(line):
    return dict(re.findall(r"([a-z_]+)=([^\s]+)", line))


def trace_rows(text, prefix, fields):
    rows = [key_values(line) for line in text.splitlines() if line.startswith(prefix + " ")]
    require(rows, f"诊断缺少 {prefix}")
    for row in rows:
        for field in fields:
            require(field in row and re.fullmatch(r"\d+(?:\.\d+)?", row[field]), f"{prefix} 缺少或非法 {field}")
            row[field] = float(row[field])
    return rows


def parse_trace(text, arm, mode, system):
    result = {"copy_stages": {}, "gate_stages": {}, "weighted_hadamard": [], "gate_internal": []}
    for row in re.finditer(r"e2e_piop_trace (copy|gate)=([^ ]+) stage=([^ ]+) ms=([\d.]+)", text):
        kind, owner, stage, value = row.groups()
        if owner == f"{system}-{mode}":
            target = result["copy_stages" if kind == "copy" else "gate_stages"]
            require(stage not in target, f"重复 {kind} stage：{stage}")
            target[stage] = float(value)
    require("wire_terminals" in result["copy_stages"] and "label_terminals" in result["copy_stages"], "缺少 copy terminal 诊断")
    if system == "mikros":
        wh = trace_rows(text, "mikros_weighted_hadamard", WH_FIELDS)
        expected = str(tiling.WEIGHTS in tiling.ARM_ENV[arm]).lower()
        require(all(row.get("tiled_weights") == expected for row in wh), "WH tiled flag 与 arm 不同")
        result["weighted_hadamard"] = wh
        result["weighted_hadamard_sum_ms"] = {field: sum(row[field] for row in wh) for field in WH_FIELDS}
        if mode == "single":
            require(len(wh) == 5 and all(row.get("log_n") == "26" for row in wh),
                    "single WH 规模或调用数错误：必须 5 次 log_n=26")
        if mode == "multiple":
            require(len(wh) == 1 and wh[0].get("log_n") == "33", "multiple WH 规模或调用数错误")
            require(set(GATE_STAGES) <= set(result["gate_stages"]), "缺少 multiple gate 子阶段")
            gate = trace_rows(text, "mikros_multiple_gate", GATE_FIELDS)
            require(len(gate) == 1 and gate[0].get("log_n") == "26" and gate[0].get("fusion") == "false"
                    and gate[0].get("selector_simd") == "false"
                    and gate[0].get("virtual_product") == str(tiling.VIRTUAL in tiling.ARM_ENV[arm]).lower(), "gate trace 开关/规模不同")
            paired = trace_rows(text, "mikros_copy_wire_terminals", ())
            require(len(paired) == 1 and paired[0].get("paired_requested") == paired[0].get("paired_active")
                    == str(tiling.PAIRED in tiling.ARM_ENV[arm]).lower(), "paired terminal trace 开关不同")
            result["gate_internal"] = gate
    return result


def audit_all(recommend):
    sources = source_audit()
    paths = primary_paths()
    require(recommend is None or recommend in paths, "推荐分支必须具有完整主样本，禁止由 profile 选取")
    previous = [EXPERIMENT / f"v10/evidence/wp_uninit-n26-r{run}" for run in range(1, 4)]
    comparison = tiling.compare(paths, previous)
    identity = comparison["arms"]["reference"]["identity"]
    require(identity["source_manifest_sha256"] == sources["source_manifest_sha256"], "主样本与 v11 源码清单不同")
    profiles = {arm: EVIDENCE / f"profile-n26-{arm}" for arm in tiling.ARM_ENV}
    pilots = {arm: EVIDENCE / f"native-pilot14-{arm}" for arm in ("reference", "all")}
    archives = evidence_archive_audit([*profiles.values(), *pilots.values(), *[p for group in paths.values() for p in group]])
    archives["primary_order"] = primary_order((EVIDENCE / "harness-v11-primary.log").read_text(encoding="utf-8"), paths)
    primary = case_map(comparison)
    for arm, folders in paths.items():
        for folder in folders:
            audit_folder(folder, identity_for(identity, arm), 26, False)
    pilot_data = {arm: audit_folder(folder, identity_for(identity, arm), 14, False) for arm, folder in pilots.items()}
    for mode in MODES:
        for system in SYSTEMS:
            left, right = (pilot_data[arm]["cases"][mode, system]["samples"][0] for arm in pilots)
            match_proof(left, right, f"N14 {mode}/{system}")
    native = native_test_records(pilots["reference"], pilot_data["reference"])
    diagnostic = {}
    for arm, folder in profiles.items():
        validated = audit_folder(folder, identity_for(identity, arm), 26, True)
        for mode in MODES:
            for system in SYSTEMS:
                sample = validated["cases"][mode, system]["samples"][0]
                match_proof(sample, primary[mode, system]["arms"]["reference"]["samples"][0], f"profile {arm}/{mode}/{system}")
                file = folder / f"n26-{mode}-{system}.stderr.log"
                diagnostic[arm, mode, system] = {"sample": sample, "file": file,
                                               "trace": parse_trace(file.read_text(encoding="utf-8"), arm, mode, system)}
    return comparison, sources, archives, native, diagnostic


def render(comparison, sources, archives, native, diagnostic, recommend):
    lines = ["# N26 Mikros 分块、双终点求值及缓冲复用优化", "",
             "64 threads，native GFNI/AVX-512/VPCLMUL。single 为一个 2^26 门 binary 电路；multiple 为 128 个相同 wiring、独立输入、每个 2^26 门的电路（总计 2^33 constraints）。双方使用相同 FLOCK 原生 commit、merged_tensor/open_batch_merged、Ligerito Fast PCS。", "",
             "基线开启 packed-direct + W′ first-write；此前 WP SIMD、selector SIMD、gate fusion 均关闭。本轮四项 Mikros 开关为 virtual product 按块生成、WH 权重按块生成、copy 两终点共享 witness 扫描、gate→WH 半长折叠缓冲复用。各项默认仍为 opt-in。", "",
             "Online 包含 commit/recommit、动态转换、PIOP、PCS、释放和序列化；offline 预处理排除，verify 单列。主样本每分支、每模式/系统三次独立新进程，报告中位数及范围。RSS 是包括 offline 的整进程 MaxRSS，不能当作 online peak；三次样本不足以自动宣称显著性。", "",
             "FLOCK 对所有新开关均为负对照；Mikros single 对 virtual/paired/scratch 为负对照，但 weights/all 可能影响其 WH。未修改代码的 commit/PCS 等阶段变化可能包含运行波动或内存布局的间接影响，不能仅凭这些数据认定为目标内核收益。", ""]
    primary = case_map(comparison)
    if recommend:
        options = ["--packed-direct", "--wp-uninit"] + ["--" + key.replace("_", "-") for key, env in tiling.CONFIG_ENV.items() if env in tiling.ARM_ENV[recommend]]
        lines += ["## 采用建议与实测结果", "", f"本轮 N26 multiple 的候选配置是 `{recommend}`：`{' '.join(options)}`。各项仍保留独立 opt-in；下面采用完整的三次主样本，不把单次 profile 外推为稳定 E2E 收益。", "",
                  "| 模式 | Mikros online s [范围] | FLOCK online s [范围] | F/M |", "|---|---:|---:|---:|"]
        for mode in MODES:
            m, f = (primary[mode, system]["arms"][recommend] for system in SYSTEMS)
            ratio = f["times_ms"]["prover_ms"]["median"] / m["times_ms"]["prover_ms"]["median"]
            lines.append(f"| {mode} | {seconds(m)} | {seconds(f)} | {ratio:.3f}× |")
        reference = primary["multiple", "mikros"]["arms"]["reference"]
        current = primary["multiple", "mikros"]["arms"][recommend]
        gate_old, gate_new = (arm["times_ms"]["gates_ms"]["median"] for arm in (reference, current))
        total_old, total_new = (arm["times_ms"]["prover_ms"]["median"] for arm in (reference, current))
        lines += ["", f"Mikros multiple 的 gate 中位数 {gate_old/1000:.4f}→{gate_new/1000:.4f}s，下降 {100*(1-gate_new/gate_old):.2f}%；online 中位数 {total_old/1000:.4f}→{total_new/1000:.4f}s，观察到下降 {100*(1-total_new/total_old):.2f}%。", "",
                  "## 总时间差值来自哪些阶段", "",
                  "下面选取两臂中各自总时间恰好为中位数的真实样本，逐阶段相减；避免把不同样本的阶段中位数拼成总差值。正数表示当前样本少用时间。该分解描述差值来源，并不证明全部差值的因果归属。", ""]
        old_sample, new_sample = (sorted(arm["samples"], key=lambda sample: sample["times"]["prover_ms"])[len(arm["samples"])//2] for arm in (reference, current))
        lines += [f"参考：`{Path(old_sample['directory']).name}`；当前：`{Path(new_sample['directory']).name}`。", "",
                  "| 阶段 | 参考 ms | 当前 ms | 少用 ms |", "|---|---:|---:|---:|"]
        for field, label in LABELS.items():
            old, new = old_sample["times"][field], new_sample["times"][field]
            lines.append(f"| {label} | {old:.3f} | {new:.3f} | {old-new:.3f} |")
        saved = old_sample["times"]["prover_ms"] - new_sample["times"]["prover_ms"]
        pcs_saved = old_sample["times"]["witness_pcs_open_ms"] - new_sample["times"]["witness_pcs_open_ms"]
        if saved > 0:
            lines += ["", f"其中未修改代码的 witness PCS 阶段差值为 {pcs_saved:.3f} ms，占总差值的 {100*pcs_saved/saved:.2f}%。因此不能将全部 online 降幅归因于本轮 gate/copy 内核；样本范围和 FLOCK 负对照仍须同时查看。", ""]
    else:
        lines += ["尚未指定采用配置；下面保留所有已完成的消融结果，不自动推荐中位数最小的一组。"]
    lines += ["", "## 正确性与证据", "",
              f"本轮 native tests：{sum(record['passed'] for record in native)} passed，{sum(record['ignored'] for record in native)} ignored，0 failed；计数从实际 test stdout 汇总。主样本 {12 * len(comparison['current_arms'])} 个、N26 单次诊断 24 个、N14 pilot 8 个 E2E 均通过完整验证。", "",
              "同一 system/mode 跨优化分支及 N26 前版 WP-on 基线，完整证明字节/hash、PIOP、terminal claims、承诺、PCS 参数、声明数一致。N14 两配置也逐项一致。所有 case 重新调用 validate_case，并核对原始 resource 文件、summary、status、source/binary receipt、native 配置及环境开关。", "",
              f"当前 {sources['source_files']} 项源码逐项匹配服务器冻结快照；源码路径集合与 v1 授权范围相同。源码包 SHA-256：`{sources['source_archive_sha256']}`；清单 SHA-256：`{sources['source_manifest_sha256']}`。", "",
              f"证据包 SHA-256：`{archives['evidence_archive_sha256']}`，{archives['evidence_archive_bytes']} bytes，{archives['evidence_regular_files']} 普通文件；逐项验证 archive、下载回执和本地提取内容。", "",
              "主样本交错运行顺序从 " + link(EVIDENCE / "harness-v11-primary.log") + " 的 JSON 头部读取，并逐项匹配实际 PRIMARY_BEGIN/PRIMARY_COMPLETE 和最终完成标志；以下不包含 pilot 或 trace。", "",
              "| 独立运行轮次 | 实际分支顺序 |", "|---|---|"]
    for run, order in enumerate(archives["primary_order"]["orders"], 1):
        lines.append(f"| r{run} | {' → '.join(order)} |")
    lines += ["",
              "## 单次诊断：只定位局部成本", "",
              "以下单位 ms，均为独立单次 trace；不进入主样本统计。父阶段包含其子阶段，不能相加。缓冲释放时机变化可能把成本移到 WH，应联合检查 gate+WH 及 E2E。", "",
              "数组规模估算与实测分开：N26 multiple 原始 virtual product 与完整 WH 权重表各为 1 GiB；两条 gate 半长缓冲共可复用 1 GiB；copy 两终点共同扫描理论上少读 3 GiB witness。这些是数组/算法访问量，未测量实际 DRAM 流量，不能直接换算时间收益。", "",
              "| 分支 | Gate total | Virtual product | Sumcheck wrapper | Product release | WH wrapper | SC message | SC fold | Copy total | Wire terminals | Label terminals |", "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"]
    for arm in tiling.ARM_ENV:
        item = diagnostic[arm, "multiple", "mikros"]
        trace, times = item["trace"], item["sample"]["times"]
        values = [times["gates_ms"], *[trace["gate_stages"][name] for name in GATE_STAGES],
                  trace["gate_internal"][0]["message_ms"], trace["gate_internal"][0]["fold_ms"], times["copy_ms"],
                  trace["copy_stages"]["wire_terminals"], trace["copy_stages"]["label_terminals"]]
        lines.append(f"| {arm} | " + " | ".join(f"{v:.3f}" for v in values) + " |")
    lines += ["", "| 模式 | 分支 | WH calls | 权重生成 | 聚合 | Certificate | Packing | 权重释放 | IP 含释放 |", "|---|---|---:|---:|---:|---:|---:|---:|---:|"]
    for mode in MODES:
        for arm in tiling.ARM_ENV:
            trace = diagnostic[arm, mode, "mikros"]["trace"]
            lines.append(f"| {mode} | {arm} | {len(trace['weighted_hadamard'])} | " +
                         " | ".join(f"{trace['weighted_hadamard_sum_ms'][field]:.3f}" for field in WH_FIELDS) + " |")
    lines += ["", "FLOCK multiple 负对照（ms）：", "", "| 分支 | Gate | Copy | Witness PCS | Public PCS | Online |", "|---|---:|---:|---:|---:|---:|"]
    for arm in tiling.ARM_ENV:
        times = diagnostic[arm, "multiple", "flock"]["sample"]["times"]
        lines.append(f"| {arm} | " + " | ".join(f"{times[key]:.3f}" for key in ("gates_ms", "copy_ms", "witness_pcs_open_ms", "public_pcs_open_ms", "prover_ms")) + " |")
    lines += ["", "## 完整主样本、阶段及原始记录", "", link(DETAIL / "comparison.json"), "", tiling.markdown(comparison),
              "## Native 测试明细", "", "| Target | Passed | Ignored | stdout SHA-256 |", "|---|---:|---:|---|"]
    lines += [f"| {r['command']} | {r['passed']} | {r['ignored']} | `{r['sha256']}` |" for r in native]
    lines += ["", "原始 trace 及调用次数/字段保存在 " + link(DETAIL / "report_audit.json") + "；各分支 `config.json` 与 command JSON 保留可复现命令。原 2^30 根目录日志不会被本脚本写入。", ""]
    return "\n".join(lines)


def self_test():
    """Small rejection tests for archive paths and trace extraction only."""
    for name in ("../secret", "/absolute", "a/../../b", "a\\b", "./a", ".git/config"):
        try:
            safe_member(name)
        except ValueError:
            pass
        else:
            raise AssertionError(f"unsafe path accepted: {name}")
    require(safe_member("profile-n26-all/config.json").name == "config.json", "normal member rejected")
    line = "mikros_weighted_hadamard log_n=33 tiled_weights=true " + " ".join(f"{key}=1.250000" for key in WH_FIELDS)
    rows = trace_rows(line, "mikros_weighted_hadamard", WH_FIELDS)
    require(rows[0]["log_n"] == "33" and sum(rows[0][field] for field in WH_FIELDS) == 7.5, "trace values")
    try:
        trace_rows(line.replace("aggregate_ms=1.250000", "aggregate_ms=nan"), "mikros_weighted_hadamard", WH_FIELDS)
    except ValueError:
        pass
    else:
        raise AssertionError("nonfinite trace accepted")
    single_wh = line.replace("log_n=33", "log_n=26").replace("tiled_weights=true", "tiled_weights=false")
    copy_trace = "\n".join(f"e2e_piop_trace copy=mikros-single stage={stage} ms=1.000000"
                           for stage in ("wire_terminals", "label_terminals"))
    def single_trace(wh):
        return "\n".join((copy_trace, *wh))
    require(len(parse_trace(single_trace([single_wh] * 5), "reference", "single", "mikros")["weighted_hadamard"]) == 5,
            "valid five-call single trace rejected")
    for wh in ([single_wh] * 4, [single_wh] * 6,
               [single_wh] * 4 + [single_wh.replace("log_n=26", "log_n=25")]):
        try:
            parse_trace(single_trace(wh), "reference", "single", "mikros")
        except ValueError as error:
            require("single WH" in str(error), "unexpected single trace rejection")
        else:
            raise AssertionError("missing, duplicate or wrong-scale single WH accepted")
    header = {"fresh_process_orders": [["reference", "paired"], ["paired", "reference"], ["reference", "paired"]],
              "trace": False, "common": ["--packed-direct", "--wp-uninit"], "threads": 64, "per_arm_repetitions": 3}
    logged = [json.dumps(header)]
    for run, order in enumerate(header["fresh_process_orders"], 1):
        for arm in order:
            logged.extend((f"PRIMARY_BEGIN {arm} {run}", "unrelated benchmark output", f"PRIMARY_COMPLETE {arm} {run}"))
    logged.append("ALL_PRIMARY_COMPLETE")
    text = "\n".join(logged)
    require(primary_order(text, ("reference", "paired"))["orders"] == header["fresh_process_orders"], "logged order extraction")
    for bad in (text.replace("ALL_PRIMARY_COMPLETE", ""),
                text.replace("PRIMARY_COMPLETE paired 2", "PRIMARY_COMPLETE reference 2"),
                text.replace("PRIMARY_BEGIN reference 1", "PRIMARY_BEGIN paired 1")):
        try:
            primary_order(bad, ("reference", "paired"))
        except ValueError:
            pass
        else:
            raise AssertionError("missing or mismatched primary order markers accepted")
    with tempfile.TemporaryDirectory(prefix="tiling-writer-test-") as tmp:
        archive = Path(tmp) / "duplicate.tar.gz"
        import io
        with tarfile.open(archive, "w:gz") as tar:
            for _ in range(2):
                info = tarfile.TarInfo("a")
                info.size = 1
                tar.addfile(info, io.BytesIO(b"x"))
        try:
            archive_files(archive)
        except ValueError:
            pass
        else:
            raise AssertionError("duplicate archive member accepted")
    print("Synthetic writer checks passed (18 checks); no experiment or report generated.")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--recommend", choices=tuple(tiling.ARM_ENV))
    parser.add_argument("--check-only", action="store_true")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        require(not args.recommend and not args.check_only, "self-test cannot consume evidence")
        self_test()
        return
    protected = {str(path): sha(path) for path in N30}
    result = audit_all(args.recommend)
    rendered = render(*result, args.recommend)
    comparison, sources, archives, native, diagnostic = result
    audit = {"source": sources, "archive": archives, "native_tests": native, "recommend": args.recommend,
             "protected_n30_sha256": protected,
             "profiles": {"/".join(key): {**value, "file": str(value["file"])} for key, value in diagnostic.items()}}
    json.dumps(audit, allow_nan=False)
    if not args.check_only:
        DETAIL.mkdir(parents=True, exist_ok=True)
        atomic_text(DETAIL / "comparison.json", json.dumps(comparison, indent=2, allow_nan=False) + "\n")
        atomic_text(DETAIL / "comparison.md", tiling.markdown(comparison))
        atomic_text(DETAIL / "report_audit.json", json.dumps(audit, indent=2, allow_nan=False) + "\n")
        atomic_text(OUTPUT, rendered)
    require(protected == {str(path): sha(path) for path in N30}, "原 2^30 日志意外改变")
    print(json.dumps({"complete_audit": True, "check_only": args.check_only,
                      "primary_arms": comparison["current_arms"], "native_passed": sum(r["passed"] for r in native),
                      "output": None if args.check_only else str(OUTPUT)}, ensure_ascii=False))


if __name__ == "__main__":
    main()
