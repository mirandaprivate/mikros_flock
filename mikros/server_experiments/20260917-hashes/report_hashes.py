#!/usr/bin/env python3
"""Locally audit downloaded frozen hash evidence and write the three root logs.

This checks receipts and recomputes statistics from raw records. It does not
replay cryptographic verification; full verification ran in each server sample.
The source archive and runner are pinned before the runner's audit is called.
"""
import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import tarfile

import run_hashes as runner

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
SOURCE_SHA256 = "1953a63658f0eec846e551f0e2bb3ac40dd68665bcd486f808e63aecdbbe62fd"
SOURCE_COUNT = 408
FLOCK_SHA256_SKIP7_PROOF = "bc535ad0e0478e10bc4e91c7ad4036551796138b3f76e678d93e706b7a8f7984"
KECCAK_SOURCE_COMMIT = "c2d0c2485a54f7b7694e19f4f59730ffde3405cf"
KECCAK_SOURCE_BLOB = "15723f053fb7e3820ace6d86ae70b7dd36d32142"
NEGATIVE_COUNTS = {"sha256": {"mikros": 60, "flock": 55},
                   "keccak": {"mikros": 137, "flock": 127},
                   "blake3": {"mikros": 74, "flock": 67}}
SCOPE = {
    "workload": "256 independent native calls per hash; this is not a general hash-DAG wiring benchmark",
    "prover_timing": "online conversion, commitment, reduction, configured grinding, public I/O or native wiring, and native PCS opening included; offline fixture/witness generation, transpose, public-I/O table preparation, and prepared-input clone excluded",
    "verifier_timing": "complete non-deferred verification, including public-matrix and PCS checks; excludes trusted public-statement-to-I/O-table preprocessing",
    "proof_size": "full bincode serialization including commitment, PIOP and native PCS proof; no zero-knowledge claim",
    "ram": runner.RAM_SCOPE,
    "pcs": "same FLOCK Ligerito Fast PCS profile, query schedule and PCS grinding for each hash; Mikros uses full carrier with num_lanes=None, FLOCK uses native compacted multicolumn layout; commitment layout and element counts are not asserted equal",
    "piop_grinding": "Mikros uses 8 bits for challenge events; FLOCK uses native degree-based grinding (skip7 first round 8 bits, ordinary tail 2 bits); this comparison does not establish equal overall 128-bit soundness",
    "matrix": "SHA-256/BLAKE3 use native public A/B CSC and C=I; Keccak uses its restored native compact walker and C=I on both systems",
    "hash_units": "SHA-256 compression with standard IV and no message padding; Keccak-f[1600] 24-round permutation without sponge padding; BLAKE3 compression with full 512-bit output",
}


def require(ok, message):
    if not ok:
        raise ValueError(message)


def sha(path):
    with path.open("rb") as file:
        return hashlib.file_digest(file, "sha256").hexdigest()


def load(path):
    return json.loads(path.read_text(encoding="utf-8"))


def save(path, value):
    path.write_text(json.dumps(value, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")


def safe_file(root, name):
    posix = PurePosixPath(name)
    require(not posix.is_absolute() and ".." not in posix.parts and "\\" not in name, f"unsafe receipt path: {name}")
    path = (root / name).resolve()
    require(path.is_relative_to(root.resolve()) and path.is_file() and not path.is_symlink(), f"missing/unsafe receipt file: {name}")
    return path


def audit_source(archive, received_manifest):
    require(sha(archive) == SOURCE_SHA256, "unexpected source archive bytes")
    with tarfile.open(archive, "r:gz") as tar:
        members = tar.getmembers()
        names = [m.name for m in members]
        require(len(names) == len(set(names)) and all(m.isfile() for m in members), "duplicate/non-file source archive member")
        for name in names:
            posix = PurePosixPath(name)
            require(not posix.is_absolute() and ".." not in posix.parts and "\\" not in name, "unsafe source archive member")
        manifest_bytes = tar.extractfile("source_manifest.json").read()
        manifest = json.loads(manifest_bytes)
        hashes = manifest["files_sha256"]
        require(len(hashes) == SOURCE_COUNT and set(names) == set(hashes) | {"source_manifest.json"}, "source file inventory differs")
        for name, expected in hashes.items():
            require(hashlib.sha256(tar.extractfile(name).read()).hexdigest() == expected, f"source hash mismatch: {name}")
        require(received_manifest.read_bytes() == manifest_bytes, "downloaded manifest differs from frozen source archive")
        # The locally imported audit and all of its imported helper scripts must
        # match the frozen package, even if unrelated workspace files changed.
        for name in ("run_hashes.py",):
            path = HERE / name
            key = path.relative_to(ROOT).as_posix()
            require(sha(path) == hashes[key], "local runner differs from executed source")
        helpers = HERE.parent / "20260916-sha128"
        for name in ("run_engineering.py", "run_column_projection.py", "run_public_csc.py", "run_server.py"):
            path = helpers / name
            require(sha(path) == hashes[path.relative_to(ROOT).as_posix()], f"audit helper differs: {name}")
    return manifest


def audit_evidence(logs, manifest_path):
    hashes = load(logs / "evidence_sha256.json")
    files = {p.relative_to(logs).as_posix() for p in logs.rglob("*") if p.is_file() and p.name != "evidence_sha256.json"}
    require(files == set(hashes), "evidence inventory has missing or unreceipted files")
    for name, expected in hashes.items():
        require(sha(safe_file(logs, name)) == expected, f"evidence hash mismatch: {name}")
    receipt = load(logs / "build-receipt.json")
    require(receipt["source_manifest_sha256"] == sha(manifest_path) and receipt["toolchain"] == runner.TOOLCHAIN
            and receipt["rustflags"] == runner.COMMON_ENV["RUSTFLAGS"], "build receipt mismatch")
    require(len(receipt["binary_sha256"]) == 1, "expected one shared executable")
    original, expected_elf = next(iter(receipt["binary_sha256"].items()))
    executable = logs / "evidence-binaries/e2e_hash_compare"
    require(PurePosixPath(original).name == executable.name and sha(executable) == expected_elf, "actual ELF digest mismatch")
    with executable.open("rb") as stream:
        require(stream.read(4) == b"\x7fELF", "evidence executable is not an ELF")
    require(load(logs / "binary-sha256.json") == {"evidence-binaries/e2e_hash_compare": expected_elf}, "executable copy receipt mismatch")
    plan = load(logs / "run_plan.json")
    require(plan["modes"] == [list(m) for m in runner.MODES]
            and plan["order"] == [[list(m) for m in runner.batch_order(b)] for b in range(3)]
            and plan["reps"] == 6 and plan["warmups"] == 1 and plan["batches"] == 3
            and plan["profiling_enabled"] is False and plan["mikros_uses_compiled_defaults"] is True
            and plan["source_files"] == SOURCE_COUNT and plan["source_manifest_sha256"] == sha(manifest_path), "run plan differs")
    status = load(logs / "status.json")
    require(status["state"] == "complete", "experiment did not complete")
    runs = [(f"validation-{kind}-{system}", kind, system, True) for kind, system in runner.MODES]
    runs += [(f"{kind}-{system}-{batch}", kind, system, False)
             for batch in range(3) for kind, system in runner.batch_order(batch)]
    completed = status["completed"]
    names = [row["name"] for row in completed]
    expected_names = [row[0] for row in runs]
    require(names in (expected_names, ["build"] + expected_names) and all(row["exit_code"] == 0 for row in completed), "completed command sequence differs")
    events = {row["name"]: row for row in completed}
    binary_path = None
    for name, kind, system, validation in runs:
        cmd = load(logs / f"{name}.command.json")
        require(cmd[:3] == ["/usr/bin/time", "-v", "-o"] and PurePosixPath(cmd[3]).name == f"{name}.resource.txt"
                and cmd[4:7] == ["taskset", "-c", runner.CPU_SPEC], "command CPU/RSS wrapper differs")
        binary_path = binary_path or cmd[7]
        require(cmd[7] == binary_path and PurePosixPath(cmd[7]).parts[-2:] == ("evidence-binaries", "e2e_hash_compare")
                and cmd[7:] == runner.command(cmd[7], kind, system, validation), "actual benchmark CLI differs")
        env = load(logs / f"{name}.environment.json")
        require(all(env.get(key) == value for key, value in runner.COMMON_ENV.items()) and env.get("LC_ALL") == "C", "actual environment differs")
        require(not any(key.startswith("MIKROS_") for key in env) and "FLOCK_NO_GRIND" not in env, "profiling or tuning override enabled")
        preflight = load(logs / f"{name}.preflight.json")
        require(0 <= preflight["selected_cpu_busy_fraction"] < 0.1 and preflight["mem_available_kib"] > 8 * 1024 * 1024, "benchmark preflight was not quiet")
        require(events[name]["process_peak_rss_kib"] == runner.peak_rss(logs / f"{name}.resource.txt"), "RSS event and GNU time disagree")
    return {"source_files": SOURCE_COUNT, "evidence_files_hashed": len(hashes), "binary_sha256": expected_elf,
            "actual_commands_checked": len(runs), "formal_samples": 108, "warmup_samples": 18,
            "validation_samples": 6, "cryptographic_replay_locally": False,
            "all_proofs_verified_and_transcript_checked_on_server": True}


def audit_native_tests(folder):
    hashes = load(folder / "evidence_sha256.json")
    files = {p.relative_to(folder).as_posix() for p in folder.rglob("*") if p.is_file() and p.name != "evidence_sha256.json"}
    require(files == set(hashes), "native-test inventory differs")
    for name, expected in hashes.items():
        require(sha(safe_file(folder, name)) == expected, f"native-test evidence differs: {name}")
    summary = load(folder / "summary.json")
    require(type(summary) is list and len(summary) == 5 and len({row["name"] for row in summary}) == 5, "expected five distinct native test groups")
    result = {}
    for row in summary:
        name = row["name"]
        require(type(name) is str and re.fullmatch(r"[A-Za-z0-9_-]+", name), "invalid test group name")
        out, err = folder / f"{name}.stdout.log", folder / f"{name}.stderr.log"
        require(row["exit_code"] == 0 and sha(out) == row["stdout_sha256"] and sha(err) == row["stderr_sha256"], "native test exit/digest mismatch")
        command = row["command"]
        require(type(command) is list and "test" in command and "--release" in command and runner.TOOLCHAIN in command, "native test command differs")
        reports = re.findall(r"test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out", out.read_text(encoding="utf-8"))
        require(reports and all(r[0] == "ok" and int(r[2]) == int(r[4]) == 0 for r in reports), "native tests failed or absent")
        totals = {field: sum(int(r[index]) for r in reports) for field, index in (("passed", 1), ("ignored", 3), ("filtered_out", 5))}
        require(totals["passed"] > 0, "native test filter ran no tests")
        result[name] = {**totals, "command": command}
    return {"groups": result, "passed": sum(r["passed"] for r in result.values()),
            "ignored": sum(r["ignored"] for r in result.values()), "evidence_files_hashed": len(hashes)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--evidence", required=True, type=Path)
    parser.add_argument("--source-archive", required=True, type=Path)
    parser.add_argument("--logs", type=Path, default=Path("results-hashes"))
    parser.add_argument("--output-root", type=Path, default=ROOT)
    args = parser.parse_args()
    evidence, output = args.evidence.resolve(), args.output_root.resolve()
    logs = (evidence / args.logs).resolve()
    require(logs.is_relative_to(evidence), "logs must be inside downloaded evidence")
    manifest_path = evidence / "source_manifest.json"
    audit_source(args.source_archive.resolve(), manifest_path)
    checks = audit_evidence(logs, manifest_path)
    require(load(evidence / "hashes-build.json") == load(logs / "build-receipt.json"), "snapshot and logged build receipts differ")
    checks["native_tests"] = audit_native_tests(evidence / "validation-native")
    summary = runner.audit(logs)
    require(summary == load(logs / "summary.json"), "recomputed raw statistics differ from server summary")
    require(summary["hashes"]["sha256"]["systems"]["flock"]["proof_blake3"] == FLOCK_SHA256_SKIP7_PROOF,
            "native SHA skip7 proof differs from previous baseline")
    for kind, entry in summary["hashes"].items():
        require({system: item["rejected"] for system, item in entry["negative_tests"].items()} == NEGATIVE_COUNTS[kind], "tamper count differs from verified local suite")
        require(entry["matrix_representation"] == ("native_compact_walker" if kind == "keccak" else "public_csc"), "matrix implementation differs")
    source_hashes = {"source_archive_sha256": SOURCE_SHA256, "source_manifest_sha256": sha(manifest_path),
                     "evidence_manifest_sha256": sha(logs / "evidence_sha256.json"), "executed_elf_sha256": checks["binary_sha256"]}
    checks.update(passed=True, summary_recomputed_exactly=True, negative_cases=NEGATIVE_COUNTS, **source_hashes)
    save(evidence / "local_audit.json", checks)
    docs = ["# Mikros 与 FLOCK skip=7：三种原生 hash 工作负载", "",
            "同一服务器、64 线程；每项 256 个独立调用。每种系统分 3 批运行，每批 1 次预热和 6 次正式测量。",
            "下表 prover/verifier time 为 18 次正式测量中位数；proof size 包含承诺和完整证明；RAM 为三批进程峰值中的最大值。", "",
            "| 工作负载 | 系统 | Prover ms | Proof bytes | Proof KiB | Verifier ms | RAM MiB |",
            "|---|---|---:|---:|---:|---:|---:|"]
    for kind in runner.HASHES:
        entry = summary["hashes"][kind]
        raw = {system: [] for system in runner.SYSTEMS}
        for system in runner.SYSTEMS:
            for batch in range(3):
                _, rows, _ = runner.checked_records(logs / f"{kind}-{system}-{batch}.jsonl", kind, system)
                raw[system].extend({"batch": batch, **row} for row in rows)
            stats = entry["systems"][system]
            stats["proof_kib"] = stats["proof_bytes"] / 1024
            stats["process_peak_rss_max_mib"] = stats["process_peak_rss_max_kib"] / 1024
            docs.append(f"| {kind} | {'Mikros' if system == 'mikros' else 'FLOCK skip=7'} | {stats['prover_ms']:.3f} | "
                        f"{stats['proof_bytes']:,} | {stats['proof_kib']:.3f} | {stats['verify_ms']:.3f} | {stats['process_peak_rss_max_mib']:.3f} |")
        document = {"schema_version": 1, "hash": kind, "instances": 256, "threads": 64, "flock_skip": 7,
                    "formal_repetitions_per_system": 18, "warmups_per_system": 3,
                    "workload": entry["workload"], "comparison": entry, "measurement_scope": SCOPE,
                    "raw_samples_including_warmups": raw, "source_and_evidence": source_hashes,
                    "evidence_directory": str(logs), "local_audit": checks,
                    "keccak_restoration": {"source_commit": KECCAK_SOURCE_COMMIT, "source_blob": KECCAK_SOURCE_BLOB}}
        save(output / f"e2e_flock_{kind}.log", document)
    docs += ["", "FLOCK/Mikros 的 prover time 比值：" + "；".join(
        f"{kind} **{summary['hashes'][kind]['flock_over_mikros']:.3f}×**" for kind in runner.HASHES) + "。", "",
        "正式样本 prover time 范围（ms）："]
    for kind in runner.HASHES:
        ranges = [f"{system} {entry['prover_range_ms'][0]:.3f}–{entry['prover_range_ms'][1]:.3f}"
                  for system, entry in summary["hashes"][kind]["systems"].items()]
        docs.append(f"- {kind}: " + "；".join(ranges))
    docs += ["",
        "各批进程峰值 RSS（MiB，依批次顺序）："]
    for kind in runner.HASHES:
        for system in runner.SYSTEMS:
            peaks = summary["hashes"][kind]["systems"][system]["process_peak_rss_kib"]
            docs.append(f"- {kind} / {system}: " + ", ".join(f"{n / 1024:.3f}" for n in peaks))
    docs += ["", "## 测量范围", "",
        "SHA-256 测量固定标准 IV 的 compression；Keccak 测量 24 轮 Keccak-f[1600] permutation；BLAKE3 测量 compression 并验证完整 512-bit 输出。均为独立调用，未测消息填充、sponge 或通用 hash DAG。",
        "", "Prover 包含在线转换、承诺、归约、配置的 grinding、公开 I/O 或原生 wiring，以及 PCS opening。离线 fixture/witness、transpose、公开 I/O 表预处理和准备好的输入复制不计时。Verifier 包含完整验证，但不包含公开 statement 到 I/O 表的可信预处理。",
        "", "RAM 是 GNU time 的整个进程峰值，包含离线准备、预热、prover、verifier 和 allocator 保留内存；不能解释为在线 prover 独占峰值。负例验证进程不计入表中 RAM。",
        "", "双方每种 hash 使用相同的 FLOCK Ligerito Fast PCS profile、查询数和 PCS grinding。Mikros 使用 full carrier（num_lanes=None），FLOCK 使用原生压缩多列布局；这不意味着承诺布局或元素数量相同。SHA-256/BLAKE3 使用公开 A/B CSC、C=I；Keccak 双方使用恢复的原生 compact walker、C=I。",
        "", "PIOP grinding 保留各协议配置：Mikros 挑战事件为 8 bit，FLOCK 按 degree 配置（skip7 首轮 8 bit、普通尾轮 2 bit）。本实验不据此声称两者整体达到相同的 128-bit soundness。", "",
        "## 验证与证据", "",
        "408 个源码文件、固定源码包、实际执行 ELF、全部原始样本/命令/环境及 GNU time RSS 均经本地摘要核验；从原始数据重算的统计与服务器 summary 完全一致。每份样本在服务器经过完整 verifier 和 transcript 检查；本地审计没有重新执行密码学验证。",
        "", f"Native 测试共 {checks['native_tests']['passed']} 项通过、{checks['native_tests']['ignored']} 项标记 ignored；没有失败或仅被过滤后未运行的测试组。",
        "", "负例拒绝数：" + "；".join(f"{kind}: Mikros {NEGATIVE_COUNTS[kind]['mikros']} / FLOCK {NEGATIVE_COUNTS[kind]['flock']}" for kind in runner.HASHES) + "。",
        "", "FLOCK SHA-256 skip7 的 proof hash 与上一轮相同；新通用 Mikros driver 使用不同 transcript，未声称其 proof hash 与旧 SHA wrapper 相同。",
        "", f"源码包 SHA-256：`{SOURCE_SHA256}`。执行 ELF SHA-256：`{checks['binary_sha256']}`。",
        "", f"Keccak 恢复来源：commit `{KECCAK_SOURCE_COMMIT}`，blob `{KECCAK_SOURCE_BLOB}`。",
        "", "本次原始源码快照保持冻结，不应原地修改；若修改源码，应另建快照并重新测量。",
        "", f"证据目录：`{logs}`。完整每组数据见 `e2e_flock_sha256.log`、`e2e_flock_keccak.log`、`e2e_flock_blake3.log`。", ""]
    (output / "e2e_flock_hashes.md").write_text("\n".join(docs), encoding="utf-8")
    print(json.dumps({"passed": True, "reports": [str(output / f"e2e_flock_{kind}.log") for kind in runner.HASHES],
                      "source_and_evidence": source_hashes}, indent=2))


if __name__ == "__main__":
    main()
