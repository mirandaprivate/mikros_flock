"""Pool both completed production runs without discarding slow samples."""
import csv
import json
from pathlib import Path
import statistics
import sys

from verify_fields import PROFILES

ROOT = Path(__file__).resolve().parent
OPS = ("add", "sub", "mul", "square", "scale", "inverse", "fold")


def main():
    if len(sys.argv) < 4:
        raise SystemExit("usage: summarize.py OUTPUT_DIR RUN_DIR RUN_DIR [RUN_DIR ...]")
    out = Path(sys.argv[1]).resolve()
    runs = [Path(p).resolve() for p in sys.argv[2:]]
    all_rows = []
    oracle_checks = 0
    for run in runs:
        completed = json.loads((run / "completion.json").read_text())
        environment = json.loads((run / "environment.json").read_text())
        assert completed["passed"] and not environment["sanitized_harness"]
        oracle_checks += completed["independent_oracle_checks"]
        with (run / "raw.csv").open(newline="") as f:
            all_rows.extend(dict(row, run=run.name) for row in csv.DictReader(f))
    out.mkdir(parents=True, exist_ok=False)
    summaries = []
    for name, _, degree, _ in PROFILES:
        for op in OPS:
            values = [float(r["ns_per_op"]) for r in all_rows
                      if r["field"] == name and r["operation"] == op]
            med = statistics.median(values)
            summaries.append({"field": name, "degree": degree, "operation": op,
                              "samples": len(values), "median_ns_per_op": med,
                              "min_ns_per_op": min(values), "max_ns_per_op": max(values),
                              "mad_ns": statistics.median(abs(x-med) for x in values),
                              "mops": 1000/med})
    with (out / "summary.csv").open("x", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=list(summaries[0]))
        writer.writeheader()
        writer.writerows(summaries)
    with (out / "raw_combined.csv").open("x", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=list(all_rows[0]))
        writer.writeheader()
        writer.writerows(all_rows)
    with (out / "provenance.json").open("x") as f:
        json.dump({"runs": [str(r) for r in runs], "raw_samples": len(all_rows),
                   "oracle_checks": oracle_checks,
                   "aggregation": "median over all ns/op samples, equal weight per sample; no exclusions"}, f, indent=2)
    lookup = {(r["field"], r["operation"]): r for r in summaries}
    lines = ["# FLINT 3.6.0 扩域算术实测（2026-09-12）", "",
             "Intel Core i7-8700K，WSL2 Ubuntu 22.04，GCC 11.4.0，单线程绑定逻辑 CPU 2。",
             "FLINT 与 C harness 使用 `-O3 -march=native`；GMP 6.2.1、MPFR 4.1.0。", "",
             "每批 256 对独立随机输入，预分配且输出与输入不重叠，重复使用缓冲区。",
             "首轮每项 9 个约 150 ms 样本，发现耗时尖峰后增加第二轮 9 个约 300 ms 样本。",
             "下表合并全部 18 个样本取中位数，未剔除慢样本；单位 ns/op，越小越快。", "",
             "| 基域 | κ | 加法 | 减法 | 扩域乘法 | 平方 | 基域标量乘 | 求逆 | 折叠 |",
             "|---|---:|---:|---:|---:|---:|---:|---:|---:|"]
    for name, _, degree, _ in PROFILES:
        lines.append(f"| {name} | {degree} | " + " | ".join(f'{lookup[name, op]["median_ns_per_op"]:.2f}' for op in OPS) + " |")
    lines += ["", "`fold` 是完整的 `a + r*(b-a)`，包含一次减法、一次扩域乘法和一次加法；`r` 是该批共享的扩域挑战。",
              "`scale` 是扩域元素乘基域标量。以上均计入 FLINT API 调用和库内部临时工作，未计入上下文构造、输入生成、预分配和正确性校验。", "",
              "## 扩域乘法吞吐与波动", "",
              "| 基域 | 百万次乘法/秒 | 相对 Goldilocks 耗时 | 最小 ns | 最大 ns | MAD ns |",
              "|---|---:|---:|---:|---:|---:|"]
    gold = lookup["Goldilocks", "mul"]["median_ns_per_op"]
    for name, *_ in PROFILES:
        r = lookup[name, "mul"]
        lines.append(f'| {name} | {r["mops"]:.3f} | {r["median_ns_per_op"]/gold:.2f}x | {r["min_ns_per_op"]:.2f} | {r["max_ns_per_op"]:.2f} | {r["mad_ns"]:.2f} |')
    lines += ["", "两轮各自的乘法中位数：", "", "| 基域 | 首轮 ns | 复核轮 ns |", "|---|---:|---:|"]
    for name, *_ in PROFILES:
        medians = [statistics.median(float(r["ns_per_op"]) for r in all_rows
                                    if r["field"] == name and r["operation"] == "mul" and r["run"] == run.name)
                   for run in runs]
        lines.append(f"| {name} | " + " | ".join(f"{n:.2f}" for n in medians) + " |")
    lines += ["", "本机 WSL 样本存在明显波动。这些数据是当前环境中的 API 吞吐初测，不能视为硬件峰值或严格依赖链延迟。", "",
              "## 域参数与存储", "", "| 基域 | 定义多项式 | log2 域大小 | κ 个有效系数的字节数 |",
              "|---|---|---:|---:|"]
    fields = json.loads((runs[-1] / "field_metadata.json").read_text())["fields"]
    for field in fields:
        lines.append(f'| {field["name"]} | {field["polynomial"]} | {field["log2_order"]:.6f} | {8*field["degree"]} |')
    lines += ["", "扩域次数按此前 16/8/4/2 的约 128-bit 口径固定；KoalaBear 四次扩域约为 124 bit。",
              "FLINT 的每个系数为 64-bit 机器字；每个元素另有 48 字节对象头及预留容量。",
              "四个定义多项式均为显式指定的稀疏不可约二项式，没有按实测耗时挑选。", "",
              "## 验证与复现", "",
              f"两轮共 {len(all_rows)} 个正式样本，全部结果批次校验通过，共 {oracle_checks:,} 次独立算术检查。",
              "Python 任意精度整数的素性/不可约性检查通过；最终 C harness 的 ASan/UBSan 冒烟测试通过。",
              "FLINT/GMP/MPFR 共享库本身没有使用 sanitizer 重编译。", "",
              "源代码、测量定义和复现命令见 [README](README.md)。",
              "合并样本见 [raw_combined.csv](results/" + out.name + "/raw_combined.csv)，统计见 [summary.csv](results/" + out.name + "/summary.csv)。"]
    lines.append("")
    for run in runs:
        lines.append(f"- [{run.name}](results/{run.name}/table.md)：原始样本、库版本、构建配置、CPU 信息、哈希和验证记录。")
    (ROOT / "RESULTS.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("\n".join(lines[:16]))


if __name__ == "__main__":
    main()
