"""Validate native Plonky3 results; preserve historical GHASH reports unchanged."""
import csv
import json
import math
from pathlib import Path
from statistics import median

ROOT = Path(__file__).resolve().parent


def main():
    paths = sorted((ROOT / "results" / "plonky3").glob("*.json"))
    assert paths, "run the benchmark first"
    records, identities, rows = [], {}, []
    for path in paths:
        report = json.loads(path.read_text(encoding="utf-8"))
        assert report["schema_version"] == 2, "historical GHASH data is not native Plonky3"
        meta = report["metadata"]
        n, runs, systems = meta["log_n"], meta["runs"], meta["systems"]
        assert meta["threads"] == meta["actual_threads"]
        assert len(report["warmups"]) == len(systems)
        assert len(report["samples"]) == len(systems) * runs
        for run in range(runs + 1):
            samples = report["warmups"] if run == 0 else [s for s in report["samples"] if s["run"] == run]
            assert {s["system"] for s in samples} == set(systems)
            assert {s["order"] for s in samples} == set(range(len(systems)))
            assert len({tuple(s["input_digest"]) for s in samples}) == 1
            for sample in samples:
                name = sample["system"]
                if name == "flock":
                    expected = 16 * (131 + 2 * (n - 6)) + 32
                elif name in ("small_field_first", "small_field_first_full_extension"):
                    expected = 56 * n + 64
                else:
                    expected = 16 * (3 * n + 3) + 8
                assert sample["bincode_bytes"] == expected
                canonical = {"dao_thaler_generic": "dao_thaler", "small_field_first_full_extension": "small_field_first", "ghash_full_extension": "ghash_small_field_first"}.get(name, name)
                identity = (sample["input_digest"], sample["proof_digest"])
                assert identities.setdefault((n, run, canonical), identity) == identity, (path, run, name)
        for summary in report["summaries"]:
            samples = [s for s in report["samples"] if s["system"] == summary["system"]]
            assert len(samples) == runs
            for key in ("prover_ms", "verifier_ms", "input_claim_check_ms"):
                values = [s[key] for s in samples]
                assert all(math.isfinite(v) and v >= 0 for v in values)
                for stat, value in (("median", median(values)), ("min", min(values)), ("max", max(values))):
                    assert math.isclose(summary[key][stat], value, rel_tol=1e-12), (path, key, stat)
            rows.append({"file": path.name, "log_n": n, "threads": meta["threads"], "runs": runs,
                         "system": summary["system"], "prover_ms": summary["prover_ms"]["median"],
                         "min_ms": summary["prover_ms"]["min"], "max_ms": summary["prover_ms"]["max"],
                         "verifier_ms": summary["verifier_ms"]["median"], "bincode_bytes": summary["bincode_bytes"]})
        if runs == 6 and len(systems) == 3:
            permutations = [tuple(s["system"] for s in sorted(
                [s for s in report["samples"] if s["run"] == run], key=lambda s: s["order"])) for run in range(1, 7)]
            assert len(set(permutations)) == 6
        records.append(report)
    fingerprints = {r["metadata"]["sources_blake3"] for r in records}
    assert len(fingerprints) == 1, "mixed source builds"
    count = sum(len(r["warmups"]) + len(r["samples"]) for r in records)
    first = records[0]["metadata"]
    doc = ["# 直接调用本地 Plonky3 的 Hadamard 比较", "",
           "由 summarize.py 从 results/plonky3/ 的原始 JSON 生成。本机 WSL 小规模检查，未运行服务器实验。", "",
           f"CPU：{first['cpu']}。Rust：`{first['rustc']}`。", "",
           f"代码 fingerprint：`{next(iter(fingerprints))}`。", "",
           "degree-2 关系为 AB-C；baseline 使用 cubic weighted sumcheck，Flock 使用现有 skip=6 路径。输入均为相同 packed F2 数据。没有 PCS、grinding 或同安全等级声明。", "",
           "small_field_first 直接调用 ../mikros/mikros/src/baseline.rs 的 prover/verifier，使用原生 BinaryField8/BinaryField128、factored weights、fold 和 BinaryChallenger。packed bits 转字节域及 C 的原生求值计入证明时间。Dao–Thaler/Flock 使用 GHASH F128；两者与 Plonky3 的差值包含域后端、表示与 transcript 差别，不能只归因于协议。", "",
           f"原生 BinaryField8 Packing::WIDTH = {first['native_base_packing_width']}。编译 flags：`{first['rustflags']}`。", "",
           "| N | 线程 | 系统 | 样本数 | prove中位数(ms) | min–max(ms) | verify(ms) | bytes |",
           "| ---: | ---: | --- | ---: | ---: | ---: | ---: | ---: |"]
    for row in sorted(rows, key=lambda r: (r["log_n"], r["threads"], r["system"])):
        doc.append(f"| 2^{row['log_n']} | {row['threads']} | {row['system']} | {row['runs']} | {row['prover_ms']:.3f} | {row['min_ms']:.3f}–{row['max_ms']:.3f} | {row['verifier_ms']:.4f} | {row['bincode_bytes']} |")
    doc += ["", f"共 {count} 份证明（含预热）通过 verifier、transcript 和全部源输入声明检查。汇总脚本另核对了输入一致性、跨线程及优化/对照证明摘要一致性、消息大小与统计量；没有删除慢样本。", "",
            "[原始 JSON 与 CSV](results/plonky3/)；[接口、实现差别及复现命令](README.md)；[历史 GHASH 重写版结果](BENCHMARKS.md)。", ""]
    (ROOT / "PLONKY3_BENCHMARKS.md").write_text("\n".join(doc), encoding="utf-8")
    with (ROOT / "results" / "plonky3" / "summary.csv").open("w", newline="", encoding="utf-8") as stream:
        writer = csv.DictWriter(stream, fieldnames=list(rows[0]))
        writer.writeheader()
        writer.writerows(rows)
    print(f"Validated {count} proof records and consistent transcript digests; wrote baseline/PLONKY3_BENCHMARKS.md")


if __name__ == "__main__":
    main()
