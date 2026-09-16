#!/usr/bin/env python3
"""Audit frozen server Hadamard evidence and write the root comparison.

No prover or verifier is executed here. Cryptographic verification and the
independent source oracle ran on the server; this rechecks their receipts,
source/ELF identity, measurement configuration and raw-sample statistics.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib
import json
import math
from pathlib import Path, PurePosixPath
import re
import statistics
import sys
import tarfile

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
SYSTEMS = ("pairreuse", "flock128")
LABELS = {"pairreuse": "Mikros", "flock128": "FLOCK skip=7"}
CPU_SPEC = "0-47,48-55,72-79"
TOOLCHAIN = "+nightly-2026-03-21"
HEX64 = re.compile(r"[0-9a-f]{64}\Z")
SCOPE = {
    "workload": "one dense binary Hadamard relation A o B = C over 2^30 bit constraints; same packed inputs for both systems",
    "protocol": "direct packed-opening PIOP only; excludes PCS and additional column-to-tensor aggregation; this is not a full SNARK",
    "prover_time": "online random weights, dynamic lookup tables, aggregation, interpolation, packing and IP/zerocheck tail included; input generation/digest, challenger initialization, proof serialization and source-oracle checks excluded",
    "verifier_time": "PIOP reduction replay only; returns deferred source claims, whose independent linear-time source oracle is reported separately",
    "proof_size": "bincode serialization of PIOP messages only; logical F128 payload also reported; no commitment or PCS opening",
    "allocation": "Mikros PairAggregate retains only scratch capacity across warm proofs; FLOCK uses its production allocation pool; all witness/challenge-dependent values are rebuilt online",
    "cold_ram": "per-system fresh-process /proc/self/status VmHWM immediately after one cold proof and before verifier/source oracle; includes 384 MiB packed input and runtime, not incremental prover-only allocation",
    "process_ram": "GNU time maximum resident set size over the complete child process, including verification and linear-time source checks; reported separately from post-prover VmHWM",
    "security": "both standalone PIOPs disable grinding; no equal 128-bit soundness or zero-knowledge claim",
    "repetition": "three independent process batches, each one warmup and six formal rounds; the same six deterministic witness seeds recur in each batch, so 18 timings are not 18 independent witnesses",
    "profiling": "independent diagnostic processes only; phase measurements are not pooled into formal timings",
}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def sha(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def load(path):
    return json.loads(path.read_text(encoding="utf-8"))


def save(path, value):
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def safe_name(name):
    require(isinstance(name, str) and name, "empty receipt path")
    path = PurePosixPath(name)
    require(not path.is_absolute() and ".." not in path.parts and "\\" not in name
            and ":" not in name, f"unsafe receipt path: {name}")
    return path


def safe_file(root, name):
    safe_name(name)
    path = root / name
    require(not path.is_symlink() and path.resolve().is_relative_to(root.resolve()) and path.is_file(),
            f"missing or unsafe receipt file: {name}")
    return path


def audit_source(archive, expected_sha, manifest_path):
    require(HEX64.fullmatch(expected_sha) and sha(archive) == expected_sha, "source archive SHA-256 mismatch")
    with tarfile.open(archive, "r:gz") as tar:
        members = tar.getmembers()
        names = [member.name for member in members]
        require(len(names) == len(set(names)) and all(member.isfile() for member in members),
                "duplicate or non-regular source archive member")
        for name in names:
            safe_name(name)
        stream = tar.extractfile("source_manifest.json")
        require(stream is not None, "source manifest absent from archive")
        manifest_bytes = stream.read()
        manifest = json.loads(manifest_bytes)
        hashes = manifest["files_sha256"]
        require(set(names) == set(hashes) | {"source_manifest.json"}, "archive source inventory mismatch")
        for name, expected in hashes.items():
            require(HEX64.fullmatch(expected), f"invalid source digest: {name}")
            require(hashlib.sha256(tar.extractfile(name).read()).hexdigest() == expected,
                    f"archive member SHA-256 mismatch: {name}")
        require(manifest_path.read_bytes() == manifest_bytes, "downloaded source manifest differs from archive")
        # Only import check_report after its code and dependencies are pinned.
        for name in ("run_univariate_64.py", "run_server_64.py", "20260917-hadamard/run_hadamard.py",
                     "20260916-sha128/run_column_projection.py", "20260916-sha128/run_public_csc.py",
                     "20260916-sha128/run_server.py"):
            local = HERE.parent / name
            relative = local.relative_to(ROOT).as_posix()
            require(relative in hashes and sha(local) == hashes[relative], f"audit helper differs from frozen source: {relative}")
    return manifest


def audit_inventory(folder):
    inventory = load(folder / "evidence_sha256.json")
    require(isinstance(inventory, dict) and inventory, "empty evidence inventory")
    actual = {path.relative_to(folder).as_posix() for path in folder.rglob("*")
              if path.is_file() and path.name != "evidence_sha256.json"}
    require(actual == set(inventory), "missing or unreceipted evidence files")
    for name, expected in inventory.items():
        require(HEX64.fullmatch(expected) and sha(safe_file(folder, name)) == expected,
                f"evidence SHA-256 mismatch: {name}")
    return inventory


def timing_stats(values):
    require(values and all(isinstance(value, (float, int)) and math.isfinite(value) and value > 0 for value in values),
            "invalid timing values")
    return {"median": statistics.median(values), "mean": statistics.mean(values),
            "min": min(values), "max": max(values), "count": len(values)}


def resource_rss(path):
    text = path.read_text(encoding="utf-8")
    values = re.findall(r"Maximum resident set size \(kbytes\):\s*(\d+)", text)
    codes = re.findall(r"Exit status:\s*(\d+)", text)
    require(len(values) == 1 and int(values[0]) > 0 and codes == ["0"], f"invalid GNU time resource log: {path.name}")
    return int(values[0])


def aggregate(reports, rss_reports, profile_reports):
    require(len(reports) == 3, "expected exactly three formal process batches")
    raw = [{"batch": batch, **sample} for batch, report in enumerate(reports) for sample in report["samples"]]
    warmups = [{"batch": batch, **sample} for batch, report in enumerate(reports) for sample in report["warmups"]]
    result = {}
    for system in SYSTEMS:
        rows = [sample for sample in raw if sample["system"] == system]
        require(len(rows) == 18 and len({sample["input_digest"] for sample in rows}) == 6,
                f"{system}: expected 18 timings of six repeated deterministic witnesses")
        by_seed = {}
        for run in range(1, 7):
            samples = [sample for sample in rows if sample["run"] == run]
            require(len(samples) == 3 and len({sample["proof_digest"] for sample in samples}) == 1
                    and len({sample["input_digest"] for sample in samples}) == 1,
                    f"{system}: deterministic proof or input changed across batches")
            by_seed[str(run)] = {"input_digest": samples[0]["input_digest"], "proof_digest": samples[0]["proof_digest"]}
        for key in ("payload_field_elements", "payload_field_bytes", "bincode_bytes"):
            require(len({sample[key] for sample in rows}) == 1, f"{system}: proof length changed")
        cold = [sample for report in rss_reports for sample in report["samples"] if sample["system"] == system]
        require(cold, f"{system}: cold RAM measurements absent")
        peaks = [sample["vm_hwm_after_prove_kib"] for sample in cold]
        result[system] = {
            "name": LABELS[system], "prover_ms": timing_stats([sample["prover_ms"] for sample in rows]),
            "verifier_ms": timing_stats([sample["verifier_ms"] for sample in rows]),
            "input_claim_check_ms": timing_stats([sample["input_claim_check_ms"] for sample in rows]),
            "proof_bytes": rows[0]["bincode_bytes"], "proof_kib": rows[0]["bincode_bytes"] / 1024,
            "payload_field_elements": rows[0]["payload_field_elements"],
            "payload_field_bytes": rows[0]["payload_field_bytes"],
            "post_prover_cold_peak_rss_kib": {"samples": peaks, "median": statistics.median(peaks), "max": max(peaks)},
            "post_prover_cold_peak_rss_mib": max(peaks) / 1024,
            "deterministic_seed_digests": by_seed,
        }
    paired = []
    for batch in range(3):
        for run in range(1, 7):
            pair = {sample["system"]: sample for sample in raw if sample["batch"] == batch and sample["run"] == run}
            require(set(pair) == set(SYSTEMS) and pair["pairreuse"]["input_digest"] == pair["flock128"]["input_digest"],
                    "formal timing pair uses different inputs")
            paired.append({"batch": batch, "run": run,
                           "flock_over_mikros": pair["flock128"]["prover_ms"] / pair["pairreuse"]["prover_ms"]})
    return {"systems": result,
            "ratio_of_prover_medians_flock_over_mikros": result["flock128"]["prover_ms"]["median"] / result["pairreuse"]["prover_ms"]["median"],
            "paired_prover_ratio": timing_stats([pair["flock_over_mikros"] for pair in paired]),
            "mikros_faster_pairs": sum(pair["flock_over_mikros"] > 1 for pair in paired),
            "paired_ratios": paired, "raw_formal_samples": raw, "raw_warmups": warmups,
            "raw_cold_rss_reports": rss_reports, "raw_profile_reports": profile_reports}


def write_outputs(output, document):
    output.mkdir(parents=True, exist_ok=True)
    save(output / "compare_hadamard_2_30.log", document)
    comparison = document["comparison"]
    lines = ["# Mikros 与 FLOCK skip=7：2^30 个 binary Hadamard 约束", "",
             "同一服务器、64 线程、同一份 packed binary 输入；Mikros κ=128，FLOCK skip=7。",
             "比较独立 Hadamard PIOP；不含 PCS，也不是完整 E2E SNARK。双方关闭 grinding。", "",
             "3 个进程批次，每批 1 次预热和 6 次正式测量。时间为每系统 18 次正式计时的中位数；"
             "每批复用同样的 6 个确定性输入种子，不能视为 18 份独立 witness。", "",
             "| 系统 | Prover ms | PIOP proof bytes | Verifier replay ms | 冷启动 prover 后峰值 RAM MiB |",
             "|---|---:|---:|---:|---:|"]
    for system in SYSTEMS:
        stats = comparison["systems"][system]
        lines.append(f"| {LABELS[system]} | {stats['prover_ms']['median']:.3f} | {stats['proof_bytes']:,} | "
                     f"{stats['verifier_ms']['median']:.4f} | {stats['post_prover_cold_peak_rss_mib']:.2f} |")
    ratio = comparison["ratio_of_prover_medians_flock_over_mikros"]
    lines += ["", f"FLOCK/Mikros 的 prover 中位数之比为 **{ratio:.3f}×**；"
              f"Mikros 在 18 个配对计时中快于 FLOCK **{comparison['mikros_faster_pairs']} 次**。", "",
              "正式样本范围（保留所有样本，未剔除长尾）："]
    for system in SYSTEMS:
        stats = comparison["systems"][system]
        lines.append(f"- {LABELS[system]}：prover {stats['prover_ms']['min']:.3f}–{stats['prover_ms']['max']:.3f} ms，"
                     f"verifier replay {stats['verifier_ms']['min']:.4f}–{stats['verifier_ms']['max']:.4f} ms。")
    lines += ["", "Verifier replay 只检查归约协议，仍返回待认证声明。所有证明的全部源输入声明均在服务器"
              "通过独立的线性时间 oracle 检查；该 oracle 不计入 prover/verifier replay 时间，也不替代 PCS。", "",
              "| 系统 | 源声明 oracle 中位数 ms | 逻辑消息 F128 数 | 逻辑消息 bytes |",
              "|---|---:|---:|---:|"]
    for system in SYSTEMS:
        stats = comparison["systems"][system]
        lines.append(f"| {LABELS[system]} | {stats['input_claim_check_ms']['median']:.3f} | "
                     f"{stats['payload_field_elements']} | {stats['payload_field_bytes']:,} |")
    lines += ["", "RAM 主表取各系统独立冷进程在 prover 返回、verifier/oracle 执行前读取的 VmHWM 最大值。"
              "它包含 384 MiB 原始输入及运行时，不是 prover 的增量内存。GNU time 的全进程峰值覆盖"
              "随后 verifier/oracle，另记在 JSON 的 full_process_rss 中；不能与主表混用。", "",
              "Mikros 使用 PairAggregate 补点内核和 scratch 容量复用，FLOCK 使用原生 skip7 与生产内存池。"
              "随机权重、动态表及在线 packing 均计入 prover；固定表首次初始化计入预热或冷进程。"
              "独立 profile 样本仅供阶段诊断，未计入正式中位数。", "",
              "每份原始记录须通过 verifier、Fiat–Shamir 下一挑战一致性和全部源声明检查。"
              "本地报告审计重新核对源码包、实际 ELF、命令、CPU 设置、证据哈希及原始统计；"
              "本地未重新执行密码学验证。", "",
              f"原始证据：`{document['source_and_evidence']['evidence_directory']}`。", "",
              "完整配置、所有原始样本、profile、RAM 与审计结果见 [compare_hadamard_2_30.log](compare_hadamard_2_30.log)。"]
    (output / "compare_hadamard_2_30.md").write_text("\n".join(lines) + "\n", encoding="utf-8")


def audit_execution(logs, manifest_path, manifest, runner):
    inventory = audit_inventory(logs)
    receipt = load(logs / "build-receipt.json")
    require(receipt["source_manifest_sha256"] == sha(manifest_path)
            and receipt["source_files"] == len(manifest["files_sha256"])
            and receipt["toolchain"] == TOOLCHAIN and receipt["rustflags"] == "-C target-cpu=native"
            and receipt["bench"] == "univariate_vs_flock", "build configuration differs")
    require(receipt["binary_relative_path"] == "evidence-binaries/univariate_vs_flock", "unexpected executed ELF path")
    binary = safe_file(logs, receipt["binary_relative_path"])
    require(sha(binary) == receipt["binary_sha256"] and binary.open("rb").read(4) == b"\x7fELF",
            "executed ELF differs from build receipt")
    require(sha(logs / "build.stdout.log") == receipt["build_stdout_sha256"], "compiler receipt changed")
    compiler = [json.loads(line) for line in (logs / "build.stdout.log").read_text(encoding="utf-8").splitlines()
                if line.startswith("{")]
    artifacts = [item for item in compiler if item.get("reason") == "compiler-artifact"
                 and item.get("target", {}).get("name") == "univariate_vs_flock"
                 and "bench" in item["target"]["kind"] and item.get("executable")]
    require(artifacts == [receipt["cargo_artifact"]] and receipt["built_path"] == artifacts[0]["executable"]
            and any(item.get("reason") == "build-finished" and item.get("success") is True for item in compiler),
            "compiler artifact does not match executed binary receipt")
    features = re.findall(r'^target_feature="([^"]+)"$', (logs / "rustc-cfg.stdout.log").read_text(), re.MULTILINE)
    require(features == receipt["native_target_features"] and set(runner.FEATURES) <= set(features), "native instruction features absent")
    require((logs / "rustc-version.stdout.log").read_text().strip() == receipt["rustc_version"]
            and "1.96.0-nightly" in receipt["rustc_version"], "toolchain version differs")
    plan = load(logs / "run_plan.json")
    require(plan["systems"] == list(SYSTEMS) and plan["log_n"] == 30 and plan["batches"] == 3
            and plan["runs"] == 6 and plan["warmups"] == 1 and plan["profile_runs"] == 3
            and plan["rss_cold_repetitions"] == 3
            and plan["batch_systems"] == [list(SYSTEMS), list(SYSTEMS[::-1]), list(SYSTEMS)]
            and plan["source_files"] == len(manifest["files_sha256"])
            and plan["source_manifest_sha256"] == sha(manifest_path), "run plan differs from requested comparison")
    specifications = [("smoke", list(SYSTEMS), 16, 2, False, False)]
    specifications += [(f"formal-{batch}", list(SYSTEMS if batch % 2 == 0 else SYSTEMS[::-1]), 30, 6, False, False)
                       for batch in range(3)]
    specifications.append(("profile", list(SYSTEMS), 30, 3, False, True))
    specifications += [(f"rss-{system}-{batch}", [system], 30, 1, True, False)
                       for batch in range(3) for system in (SYSTEMS if batch % 2 == 0 else SYSTEMS[::-1])]
    stage_names = ["rustc-version", "rustc-cfg", "test-univariate", "test-zerocheck", "build"] + [item[0] for item in specifications]
    status = load(logs / "status.json")
    require(status["state"] == "complete" and [event["name"] for event in status["completed"]] == stage_names
            and all(event["exit_code"] == 0 and not event.get("timeout") for event in status["completed"]),
            "server experiment did not complete every required stage")
    events = {event["name"]: event for event in status["completed"]}
    hardware = load(logs / "hardware.json")
    expected_cpus = set(range(48)) | set(range(48, 56)) | set(range(72, 80))
    require(hardware["affinity"] == CPU_SPEC and expected_cpus <= set(hardware["allowed_cpus"]), "server CPU affinity differs")
    full_process_rss = {}
    for name in stage_names:
        command = load(logs / f"{name}.command.json")
        require(command[:3] == ["/usr/bin/time", "-v", "-o"]
                and PurePosixPath(command[3]).name == f"{name}.resource.txt"
                and command[4:7] == ["taskset", "-c", CPU_SPEC], f"{name}: CPU/RAM wrapper differs")
        environment = load(logs / f"{name}.environment.json")
        require(environment.get("RUSTFLAGS") == "-C target-cpu=native"
                and environment.get("RAYON_NUM_THREADS") == "64" and environment.get("LC_ALL") == "C"
                and set(environment) == {"RUSTFLAGS", "RAYON_NUM_THREADS", "CARGO_TARGET_DIR", "LC_ALL"}
                | ({"FLOCK_ZC_TIMING"} if name == "profile" else set()), f"{name}: environment overrides differ")
        if name == "profile":
            require(environment["FLOCK_ZC_TIMING"] == "1", "profile did not enable FLOCK phase timestamps")
        preflight = load(logs / f"{name}.preflight.json")
        require(0 <= preflight["selected_cpu_busy_fraction"] < 0.1 and preflight["mem_available_kib"] > 8 * 1024 * 1024,
                f"{name}: preflight was not quiet or had insufficient memory")
        rss = resource_rss(logs / f"{name}.resource.txt")
        require(events[name]["whole_process_peak_rss_kib"] == rss, f"{name}: RAM event and GNU time differ")
        full_process_rss[name] = {"kib": rss, "mib": rss / 1024,
                                  "attribution": "paired process; cannot attribute to one system" if name.startswith(("formal-", "smoke", "profile"))
                                  else "fresh single-system process including source oracle" if name.startswith("rss-") else "build or test process"}
    executable_path = None
    reports = {}
    for name, systems, log_n, runs, rss, profile in specifications:
        command = load(logs / f"{name}.command.json")
        executable_path = executable_path or command[7]
        output = str(PurePosixPath(command[3]).parent / f"{name}.json")
        require(command[7] == executable_path
                and PurePosixPath(executable_path).parts[-2:] == ("evidence-binaries", "univariate_vs_flock")
                and command[7:] == runner.command(executable_path, output, systems, log_n, runs, rss=rss, profile=profile),
                f"{name}: executed benchmark arguments differ")
        # This calls the existing schema checker, pinned against source.tar.gz.
        reports[name] = runner.verified_report(logs / f"{name}.json", systems, log_n, runs, rss=rss, profile=profile)
        require(reports[name]["metadata"]["compiled_target_features"] == [feature for feature in
                ("gfni", "avx512f", "avx512bw", "avx512vbmi", "vpclmulqdq", "pclmulqdq", "avx2", "sse4.1", "neon")
                if feature in features], f"{name}: benchmark target features differ from compiler receipt")
        if "flock_actual_threads" in reports[name]["metadata"]:
            require(reports[name]["metadata"]["flock_actual_threads"] == 64, f"{name}: FLOCK actual pool size differs")
    # Independently check the exact test commands before accepting their output.
    for name, package, selector in (("test-univariate", "mikros", ["--test", "univariate_roundtrip"]),
                                    ("test-zerocheck", "flock-core", ["--lib", "zerocheck::tests"])):
        command = load(logs / f"{name}.command.json")[7:]
        require(command == ["cargo", TOOLCHAIN, "test", "--release", "--offline", "--locked", "-j8", "-p", package,
                            *selector, "--", "--test-threads=1"], f"{name}: native test command differs")
    tests = runner.test_results(logs)
    require(tests == receipt["tests_passed"], "native tests differ from build receipt")
    return reports, full_process_rss, {"passed": True, "source_files_verified": len(manifest["files_sha256"]),
             "evidence_files_hashed": len(inventory), "executed_elf_sha256": receipt["binary_sha256"],
             "native_tests_passed": tests, "total_native_tests_passed": sum(tests.values()),
             "commands_checked": len(stage_names), "cryptographic_replay_locally": False,
             "server_proof_checks": "every record passed verifier, transcript equality and independent full source oracle"}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-archive", required=True, type=Path)
    parser.add_argument("--expected-source-sha256", required=True)
    parser.add_argument("--evidence", required=True, type=Path)
    parser.add_argument("--output-root", type=Path, default=ROOT)
    args = parser.parse_args()
    evidence, archive = args.evidence.resolve(), args.source_archive.resolve()
    manifest_path, logs = evidence / "source_manifest.json", evidence / "results-hadamard"
    manifest = audit_source(archive, args.expected_source_sha256, manifest_path)
    sys.path.insert(0, str(HERE))
    runner = importlib.import_module("run_hadamard")
    reports, process_rss, audit = audit_execution(logs, manifest_path, manifest, runner)
    recomputed = runner.audit(logs)
    require(recomputed == load(logs / "summary.json"), "server summary differs from recomputed raw reports")
    formal = [reports[f"formal-{batch}"] for batch in range(3)]
    cold = [reports[f"rss-{system}-{batch}"] for batch in range(3) for system in SYSTEMS]
    comparison = aggregate(formal, cold, [reports["profile"]])
    for system in SYSTEMS:
        for metric in ("prover_ms", "verifier_ms", "input_claim_check_ms"):
            ours, theirs = comparison["systems"][system][metric], recomputed["systems"][system][metric]
            require(all(ours[key] == theirs[key] for key in ("median", "mean", "min", "max")),
                    f"independent raw statistics differ: {system}/{metric}")
    comparison["independent_profile_phase_statistics"] = {
        "mikros": recomputed["systems"]["pairreuse"]["profile_phase_ms"],
        "flock": recomputed["profile_flock_stderr_phases"],
    }
    audit.update(summary_recomputed_exactly=True, formal_raw_samples=36, formal_warmup_samples=6,
                 total_server_verified_proofs=recomputed["validated_proof_records"])
    provenance = {"source_archive_sha256": args.expected_source_sha256, "source_archive": str(archive),
                  "source_manifest_sha256": sha(manifest_path), "executed_elf_sha256": audit["executed_elf_sha256"],
                  "evidence_inventory_sha256": sha(logs / "evidence_sha256.json"), "evidence_directory": str(logs),
                  "base_revision": manifest["base_revision"]}
    document = {"schema_version": 1, "binary_constraints": 1 << 30, "log_n": 30, "threads": 64,
                "mikros_kappa": 128, "flock_skip": 7, "cpu_affinity": CPU_SPEC,
                "formal_samples_per_system": 18, "unique_formal_witnesses": 6,
                "formal_process_batches": 3, "warmups_per_system": 3,
                "measurement_scope": SCOPE, "comparison": comparison, "full_process_rss": process_rss,
                "hardware": load(logs / "hardware.json"), "build_fingerprints": recomputed["build_fingerprints"],
                "source_and_evidence": provenance, "local_audit": audit}
    save(evidence / "local_audit.json", {**audit, **provenance})
    write_outputs(args.output_root.resolve(), document)
    print(json.dumps({"prover_median_ms": {system: comparison["systems"][system]["prover_ms"]["median"] for system in SYSTEMS},
                      "flock_over_mikros": comparison["ratio_of_prover_medians_flock_over_mikros"],
                      "local_audit": audit}, indent=2))


if __name__ == "__main__":
    main()
