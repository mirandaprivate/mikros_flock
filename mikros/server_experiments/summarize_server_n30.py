#!/usr/bin/env python3
"""Validate a completed server N30 run before writing its local report."""
from __future__ import annotations

import argparse
import csv
import importlib.util
import json
import math
from pathlib import Path
import re


THREADS = (1, 6, 24)
SYSTEMS = ("packed", "flock")
HEX64 = re.compile(r"[0-9a-f]{64}\Z")
MIKROS = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("mikros_summarize_results", MIKROS / "summarize_results.py")
assert spec is not None and spec.loader is not None
shared = importlib.util.module_from_spec(spec)
spec.loader.exec_module(shared)
read_report = shared.read_report


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def load(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def digest(value, label: str) -> None:
    require(isinstance(value, str) and HEX64.fullmatch(value) is not None, f"invalid {label}")


def finite(value, label: str, positive: bool = False) -> None:
    require(isinstance(value, (int, float)) and math.isfinite(value)
            and (value > 0 if positive else value >= 0), f"invalid {label}: {value!r}")


def checked_report(path: Path, threads: int, systems: tuple[str, ...], log_n: int,
                   runs: int, rss: bool = False) -> dict:
    report = read_report(path)
    meta = report["metadata"]
    require(meta["threads"] == meta["rayon_actual_threads"] == threads, f"{path}: threads")
    require(meta["logs"] == [log_n] and meta["runs"] == runs, f"{path}: dimensions/runs")
    require(meta["systems"] == list(systems), f"{path}: systems/order")
    require(meta["mikros_phase_profiling"] is False, f"{path}: phase probes enabled")
    require(meta["transcript_hash"] == "blake3" and meta["grinding"].startswith("disabled for both"),
            f"{path}: transcript/grinding")
    require(not {"FLOCK_ZC_TIMING", "FLOCK_POOL_TRACE", "FLOCK_SPARSE_GATE", "RAYON_NUM_THREADS", "RUSTFLAGS"}
            .intersection(meta["tuning_environment"]), f"{path}: unexpected tuning override")
    expected_mode = "fresh_process_rss_cold_proof" if rss else "paired_warm_timing"
    require(meta["mode"] == expected_mode, f"{path}: measurement mode")
    require(meta["warmups_per_system_per_dimension"] == (0 if rss else 1), f"{path}: warmups")
    digest(meta["mikros_sources_blake3"], f"{path} source fingerprint")
    require(len(report["samples"]) == runs * len(systems), f"{path}: sample count")
    require(len(report["warmups"]) == (0 if rss else len(systems)), f"{path}: warmup count")
    require(len(report["summaries"]) == len(systems), f"{path}: summary count")
    require({(s["log_n"], s["system"]) for s in report["summaries"]}
            == {(log_n, s) for s in systems}, f"{path}: summary coverage")
    for kind, expected_runs in (("samples", set(range(1, runs + 1))), ("warmups", set() if rss else {0})):
        entries = report[kind]
        require({(s["run"], s["system"]) for s in entries}
                == {(r, system) for r in expected_runs for system in systems}, f"{path}: {kind} coverage")
        for sample in entries:
            require(sample["log_n"] == log_n, f"{path}: sample dimension")
            digest(sample["input_digest"], f"{path} input digest")
            digest(sample["proof_digest"], f"{path} proof digest")
            for key in ("prover_ms", "verifier_ms", "input_claim_check_ms"):
                finite(sample[key], f"{path} {key}", positive=True)
            fields = 131 + 2 * (log_n - 6) if sample["system"] == "flock" else 2190 + 2 * (log_n - 7)
            overhead = 32 if sample["system"] == "flock" else 24
            require(sample["payload_field_elements"] == fields
                    and sample["payload_field_bytes"] == fields * 16
                    and sample["bincode_bytes"] == fields * 16 + overhead, f"{path}: message size")
            require("mikros_phases" not in sample, f"{path}: unexpected phase samples")
            if rss:
                before, peak = sample["vm_rss_before_prove_kib"], sample["vm_hwm_after_prove_kib"]
                finite(before, f"{path} RSS before", positive=True)
                finite(peak, f"{path} RSS peak", positive=True)
                require(peak >= before, f"{path}: RSS peak smaller than current RSS")
            else:
                require(sample["vm_rss_before_prove_kib"] is None
                        and sample["vm_hwm_after_prove_kib"] is None, f"{path}: timing/RSS mixed")
        for run in expected_runs:
            pair = [s for s in entries if s["run"] == run]
            require({s["order"] for s in pair} == set(range(len(systems))), f"{path}: execution positions")
            require(len({s["input_digest"] for s in pair}) == 1, f"{path}: paired input mismatch")
            mikros = [s for s in pair if s["system"] != "flock"]
            require(len({s["proof_digest"] for s in mikros}) <= 1, f"{path}: Mikros kernel proof mismatch")
    if runs == 6 and systems == SYSTEMS:
        for system in systems:
            require(sum(s["system"] == system and s["order"] == 0 for s in report["samples"]) == 3,
                    f"{path}: first-position imbalance")
    for summary in report["summaries"]:
        sample = next(s for s in report["samples"] if s["system"] == summary["system"])
        for key in ("payload_field_elements", "payload_field_bytes", "bincode_bytes"):
            require(summary[key] == sample[key], f"{path}: summary {key}")
    return report


def validate(run_dir: Path) -> tuple[dict, dict, dict, list[dict]]:
    results = run_dir / "results"
    status, environment = load(results / "status.json"), load(results / "server_environment.json")
    require(status["stage"] == "complete" and status["configs"] == list(THREADS)
            and status["log_n"] == 30 and status["measured_runs"] == 6, "server run is not complete")
    manifest = load(run_dir / "source_manifest.json")
    require(environment["source_manifest"] == manifest, "local/server source manifest mismatch")
    require(bool(manifest["files_sha256"]), "empty source manifest")
    for name, value in manifest["files_sha256"].items():
        digest(value, f"manifest {name}")
    digest(environment["executable_sha256"], "server executable SHA256")
    environment["observed_cgroups"] = {}
    for phase in ("start", "end"):
        cgroup_path = results / f"effective_cgroups_{phase}.json"
        if cgroup_path.exists():
            environment["observed_cgroups"][phase] = load(cgroup_path)
    topology = {cpu: (core, socket, node) for cpu, core, socket, node in environment["topology"]}
    allowed, node = set(environment["allowed_cpus"]), environment["selected_numa_node"]
    configs = {int(k): v for k, v in environment["config_cpu_lists"].items()}
    require(set(configs) == set(THREADS), "unexpected affinity configurations")
    for threads, cpus in configs.items():
        require(len(cpus) == len(set(cpus)) == threads and set(cpus) <= allowed, "invalid allowed CPU list")
        require(all(cpu in topology and topology[cpu][2] == node for cpu in cpus), "CPU list crosses NUMA nodes")
        require(len({topology[cpu][:2] for cpu in cpus}) == threads, "CPU list contains SMT siblings")
        require(cpus == configs[24][:threads], "thread configurations use different core prefixes")

    command_list = load(results / "commands.json")
    expected = {"tests", "build", "smoke"} | {f"comparison-{t}t" for t in THREADS} | {
        f"rss-{s}-{t}t" for t in THREADS for s in SYSTEMS}
    commands = {c["stage"]: c for c in command_list}
    require(len(commands) == len(command_list) and set(commands) == expected, "missing/duplicate/unexpected command stages")
    for stage, command in commands.items():
        require(command["returncode"] == 0, f"failed stage: {stage}")
        finite(command["wall_seconds"], f"{stage} wall seconds", positive=True)
        argv = command["argv"]
        if stage in ("tests", "build"):
            require("--locked" in argv and "--offline" in argv, f"{stage}: dependency policy")
        else:
            threads = 1 if stage == "smoke" else int(stage.rsplit("-", 1)[1][:-1])
            require(argv[:2] == ["taskset", "-c"]
                    and argv[2] == ",".join(map(str, configs[threads])), f"{stage}: command affinity")
            require(argv[argv.index("--threads") + 1] == str(threads), f"{stage}: command thread count")
            require(("--rss" in argv) == stage.startswith("rss-"), f"{stage}: RSS command mode")
    executable_paths = {c["argv"][3] for stage, c in commands.items() if stage not in ("tests", "build")}
    require(len(executable_paths) == 1, "benchmark executable changed between runs")

    timings = {t: checked_report(results / f"comparison-{t}t.json", t, SYSTEMS, 30, 6) for t in THREADS}
    rss = {(t, s): checked_report(results / f"rss-{s}-{t}t.json", t, (s,), 30, 1, True)
           for t in THREADS for s in SYSTEMS}
    smoke = checked_report(results / "smoke.json", 1, ("lookup", "packed", "flock"), 20, 1)
    reports = list(timings.values()) + list(rss.values()) + [smoke]
    fingerprints = {r["metadata"]["mikros_sources_blake3"] for r in reports}
    require(len(fingerprints) == 1, "benchmark source fingerprints differ")
    for key in ("cpu", "os", "arch", "rustc", "compiled_target_features"):
        require(all(r["metadata"][key] == reports[0]["metadata"][key] for r in reports), f"inconsistent {key}")
    identities = {}
    for report in reports:
        for sample in report["samples"] + report["warmups"]:
            # All Mikros kernels have the same protocol, including the smoke oracle.
            protocol = "flock" if sample["system"] == "flock" else "mikros"
            key = (sample["log_n"], sample["run"], protocol)
            identity = (sample["input_digest"], sample["proof_digest"])
            require(identities.setdefault(key, identity) == identity, f"cross-report identity mismatch: {key}")
    count = sum(len(r["samples"]) + len(r["warmups"]) for r in reports)
    require(count == 54, f"expected 54 checked proofs, found {count}")

    baseline = {s["system"]: s["prover_ms"]["median"] for s in timings[1]["summaries"]}
    rows = []
    for threads in THREADS:
        summaries = {s["system"]: s for s in timings[threads]["summaries"]}
        ratio = summaries["packed"]["prover_ms"]["median"] / summaries["flock"]["prover_ms"]["median"]
        for system in SYSTEMS:
            s, memory = summaries[system], rss[threads, system]["samples"][0]
            rows.append({
                "threads": threads, "system": system, "log_n": 30,
                "prover_median_ms": s["prover_ms"]["median"], "prover_min_ms": s["prover_ms"]["min"],
                "prover_max_ms": s["prover_ms"]["max"], "packed_over_flock": ratio,
                "speedup_over_1_thread": baseline[system] / s["prover_ms"]["median"],
                "verifier_median_ms": s["verifier_ms"]["median"], "bincode_bytes": s["bincode_bytes"],
                "source_check_median_ms": s["input_claim_check_ms"]["median"],
                "source_check_min_ms": s["input_claim_check_ms"]["min"],
                "source_check_max_ms": s["input_claim_check_ms"]["max"],
                "rss_before_mib": memory["vm_rss_before_prove_kib"] / 1024,
                "rss_peak_mib": memory["vm_hwm_after_prove_kib"] / 1024,
                "rss_source_check_ms": memory["input_claim_check_ms"],
                "comparison_process_wall_seconds": commands[f"comparison-{threads}t"]["wall_seconds"],
                "rss_process_wall_seconds": commands[f"rss-{system}-{threads}t"]["wall_seconds"],
            })
    return environment, commands, reports[0]["metadata"], rows


def render(environment: dict, commands: dict, metadata: dict, rows: list[dict]) -> str:
    node = environment["selected_numa_node"]
    text = ["# 服务器 N=2^30：Mikros Packed 与 Flock", "",
            "本报告仅由已完成的原始服务器记录生成。N=1,073,741,824 个 binary Hadamard 约束；三份 packed 输入共 384 MiB。", "",
            "## 环境与比较范围", "",
            f"- CPU：{metadata['cpu']}；{metadata['os']} {metadata['arch']}；编译目标特性：{', '.join(metadata['compiled_target_features'])}。",
            f"- Rust：{metadata['rustc']}。同一服务器 executable，复用相同 F128 backend 与 BLAKE3 transcript。",
            f"- 测量时间：{environment['utc_start']} 至 {environment['utc_end']}（UTC）；共享服务器，未声明独占。",
            f"- 同一 NUMA 节点 {node}，每个物理核选择一个逻辑 CPU；分别使用 1、6、24 个物理核。仅设置 CPU affinity，内存采用 Linux 默认 first-touch，没有显式内存绑定或交错分配。",
            "- 每系统预热一次、正式测量六次；每个线程配置中两系统各先执行三次，使用逐 run 相同的输入。没有剔除慢样本；六次样本不构成统计显著性保证。",
            "- 全部 witness 变换、动态查表构造、分配、权重、fold、transcript 与终端声明构造计入在线 prover；输入生成/digest、challenger 初始化、序列化和完整源输入核验在计时外。",
            "- 两边均为 PIOP reduction，不含 packed-opening reduction、PCS commitment/opening；关闭 grinding。协议参数不同，不声称相同的 128-bit soundness，也不将它称作完整 SNARK 比较。",
            "- Flock 保留生产实现的跨 proof scratch 池，Mikros scratch 在每份 proof 内分配与复用；主表是各自原生分配策略下的预热吞吐比较。进程峰值 RSS 另用冷启动独立进程测量。", "",
            "| 线程 | CPU affinity |", "| ---: | --- |"]
    for threads in THREADS:
        cpus = environment["config_cpu_lists"][str(threads)]
        text.append(f"| {threads} | {', '.join(map(str, cpus))} |")
    text += ["", "## 在线 prover", "",
             "单位 ms。范围为六次正式样本的 min–max；Packed/Flock 大于 1 表示 Packed 中位数更慢。相对 1T 的扩展比在各系统内部计算。", "",
             "| 线程 | Packed 中位数（范围） | Flock 中位数（范围） | Packed/Flock | Packed 相对 1T | Flock 相对 1T |",
             "| ---: | ---: | ---: | ---: | ---: | ---: |"]
    for threads in THREADS:
        packed, flock = [next(r for r in rows if r["threads"] == threads and r["system"] == s) for s in SYSTEMS]
        def interval(r):
            return f"{r['prover_median_ms']:.3f}（{r['prover_min_ms']:.3f}–{r['prover_max_ms']:.3f}）"
        text.append(f"| {threads} | {interval(packed)} | {interval(flock)} | {packed['packed_over_flock']:.3f}× | "
                    f"{packed['speedup_over_1_thread']:.2f}× | {flock['speedup_over_1_thread']:.2f}× |")
    if all(r["packed_over_flock"] > 1 for r in rows):
        text += ["", "在这台服务器的全部三组配置中，当前 Packed Mikros prover 都慢于 Flock。不能将此前 i7 / WSL2 小规模比较中的相对优势外推至此次 Xeon N=2^30 运行。", "",
                 "源码与编译特性表明，Flock 的 inner shift/reduce 使用 GFNI + AVX-512 专用分派，tail folding 使用 AVX-512 + VPCLMUL；Mikros 的 64 行聚合仍使用可移植 u64 转置和 subset-table 查询。两边使用相同 native 编译配置，但 CPU 专用内核的实现程度不同。本轮没有分阶段 profile 或指令集消融，不能据此把全部耗时差归因于某一种指令集、缓存或频率因素。"]
    text += ["", "## Verifier、通信与内存", "",
             "Verifier 仅验证 PIOP reduction；全部原始 A/B/C 声明另行认证。RSS 来自每系统/线程一个新进程，在 prover 返回后、verifier/源检查前读取 VmHWM，包含 packed 输入、runtime、表与工作区。单次冷启动 RSS 不是稳定内存分布估计，也不是完整进程运行至退出的峰值。", "",
             "| 线程 | 系统 | verifier 中位数 (ms) | bincode bytes | prove 前 RSS (MiB) | prove 后进程峰值 RSS (MiB) |",
             "| ---: | --- | ---: | ---: | ---: | ---: |"]
    for r in rows:
        text.append(f"| {r['threads']} | {r['system']} | {r['verifier_median_ms']:.4f} | {r['bincode_bytes']:,} | "
                    f"{r['rss_before_mib']:.2f} | {r['rss_peak_mib']:.2f} |")
    text += ["", "## 计时外的完整源输入核验", "",
             "每份预热、正式证明、RSS 和 smoke 都执行 verifier replay、transcript 状态一致性和三个完整源输入声明检查。下表是主实验中单份 proof 的源检查时间（秒），不包含在上面的 prover/verifier 时间中；这些线性扫描保留到 N=2^30，没有采样或省略。", "",
             "| 线程 | 系统 | 源检查中位数 (s) | 源检查范围 (s) |",
             "| ---: | --- | ---: | ---: |"]
    for r in rows:
        text.append(f"| {r['threads']} | {r['system']} | {r['source_check_median_ms']/1000:.3f} | "
                    f"{r['source_check_min_ms']/1000:.3f}–{r['source_check_max_ms']/1000:.3f} |")
    text += ["", "主实验进程墙钟时间包含预热、六轮两系统运行、输入生成/哈希、所有源检查、序列化和输出，不能与单份 prover 时间直接比较。", "",
             "| 线程 | 主实验整个进程 (s) |", "| ---: | ---: |"]
    for threads in THREADS:
        text.append(f"| {threads} | {commands[f'comparison-{threads}t']['wall_seconds']:.3f} |")
    text += ["", "## 核验与追溯", "",
             "汇总器已检查 complete 状态、全部 12 个驱动阶段零退出、锁定/离线构建、CPU affinity 与物理核/NUMA 拓扑、各系统先执行三次、42 份主实验（含预热）、6 份独立 RSS 和 6 份 smoke，共 54 份证明记录。",
             "smoke 使用 N=2^20 的 lookup/packed/Flock；两个 Mikros 内核证明摘要相同。所有线程、批次、RSS 中相同 N/run/protocol 的输入与序列化 proof 摘要完全一致；所有报告编译时源码指纹一致，服务器 source manifest 与本地 manifest 一致。源检查成功来自 benchmark 的完成记录，汇总器不会重新生成密码学证明。", "",
             f"- 源码 fingerprint（BLAKE3）：`{metadata['mikros_sources_blake3']}`。",
             f"- 服务器 executable SHA256：`{environment['executable_sha256']}`。",
             f"- 原仓库 revision：`{environment['source_manifest']['base_revision']}`；具体源码以快照 manifest 为准。",
             "- [服务器环境](results/server_environment.json)、[命令及墙钟记录](results/commands.json)、[完成状态](results/status.json)、[源码 manifest](source_manifest.json)、[汇总 CSV](results/summary.csv)。",
             "- [Rust 测试日志](results/tests.log)；首次离线构建因依赖缓存不全停止，随后按 Cargo.lock 补齐依赖并成功重跑，最初记录保留在 bootstrap-* 文件中。",
             "- CPU affinity 不等于独占 CPU；原始样本、loadavg 与环境记录均保留。", "",
             "复核命令：`python3 mikros/server_experiments/summarize_server_n30.py --run-dir <本地运行目录>`。只有全部校验通过才生成 REPORT.md 与 summary.csv。", ""]
    cgroups = environment["observed_cgroups"]
    if set(cgroups) == {"start", "end"}:
        records = [record for snapshot in cgroups.values() for record in snapshot["effective_ancestors"].values()]
        unlimited_cpu = all(record.get("cpu.max", "max").split()[0] == "max" for record in records)
        unlimited_memory = all(record.get("memory.max", "max") == "max" for record in records)
        text += ["## 实际 cgroup 补充记录", "",
                 "另从驱动进程所属 cgroup 逐级采集至根节点，保留起止 cpu.stat、cpu.max 和 memory.max。",
                 f"起止采样未发现有限 CPU 配额：{'是' if unlimited_cpu else '否'}；未发现有限 memory.max：{'是' if unlimited_memory else '否'}。此检查不排除其他任务的竞争。",
                 "记录：[开始](results/effective_cgroups_start.json)、[结束](results/effective_cgroups_end.json)。", ""]
    return "\n".join(text)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-dir", type=Path, required=True)
    args = parser.parse_args()
    require(__debug__, "run without python -O: the shared read_report uses assertions")
    run_dir = args.run_dir.resolve()
    environment, commands, metadata, rows = validate(run_dir)
    document = render(environment, commands, metadata, rows)
    (run_dir / "REPORT.md").write_text(document, encoding="utf-8")
    with (run_dir / "results" / "summary.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)
    print(f"Validated 54 proof records; wrote {run_dir / 'REPORT.md'} and results/summary.csv")


if __name__ == "__main__":
    try:
        main()
    except (AssertionError, KeyError, ValueError, OSError) as error:
        raise SystemExit(f"Server N30 summary validation failed: {error}") from error
