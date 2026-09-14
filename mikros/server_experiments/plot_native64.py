#!/usr/bin/env python3
"""Plot completed native64 experiments; never substitute or invent observations.

Usage (WSL Python with matplotlib installed):
    python3 mikros/server_experiments/plot_native64.py \
        --run-dir mikros/server_experiments/20260912T-native64

The existing report validator checks completion, all declared proof records,
input/proof identity, source fingerprints, executable provenance and CPU sets
before this script imports matplotlib or creates an output directory.
"""
from __future__ import annotations

import argparse
import importlib.util
import json
import math
from pathlib import Path
from statistics import mean, median


MAIN_SYSTEMS = ("legacy", "native", "reuse", "flock")
ABLATION_SYSTEMS = ("legacy", "stream", "simdv1", "simd", "wide", "fused", "native", "factored")
MAIN_LABELS = ("Mikros legacy", "Mikros native\nfresh allocations", "Mikros native\nretained capacity", "Flock\nretained buffer pool")
ABLATION_LABELS = (
    "Legacy 64-row kernel",
    "Streamed tensor",
    "GFNI: 8 coefficients",
    "GFNI: 64 coefficients",
    "+ VPCLMUL inner product",
    "+ packing / IP fusion",
    "+ blocked weights (native)",
    "+ factored row weights",
)
COLORS = {"legacy": "#777777", "native": "#0072B2", "reuse": "#009E73", "flock": "#D55E00"}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def validator_module():
    path = Path(__file__).with_name("summarize_native64.py")
    spec = importlib.util.spec_from_file_location("native64_plot_validation", path)
    require(spec is not None and spec.loader is not None, f"cannot load validator: {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def selected_report(records, filename: str):
    found = [record["report"] for record in records if record["path"].name == filename]
    require(len(found) == 1, f"expected exactly one {filename}")
    return found[0]


def measured_values(report, systems, *, log_n: int, runs: int):
    require(report["metadata"]["logs"] == [log_n], f"expected only N=2^{log_n}")
    require(set(report["metadata"]["systems"]) == set(systems), "unexpected benchmark systems")
    require(report["metadata"]["runs"] == runs, f"expected {runs} measured runs")
    values = {}
    for system in systems:
        samples = sorted((sample for sample in report["samples"] if sample["system"] == system),
                         key=lambda sample: sample["run"])
        require([sample["run"] for sample in samples] == list(range(1, runs + 1)),
                f"incomplete samples for {system}")
        values[system] = [sample["prover_ms"] for sample in samples]
    return values


def load_completed(run_dir: Path):
    require(__debug__, "do not run with python -O; shared validation uses assertions")
    version_dir = run_dir / "v3"
    for name in ("results", "ablation"):
        path = version_dir / name / "status.json"
        require(path.is_file(), f"missing {path}; no plot written")
        status = json.loads(path.read_text(encoding="utf-8"))
        require(status.get("stage") == "complete", f"{name} is not complete; no plot written")
    validator = validator_module()
    main_records = validator.load_group(version_dir, "results")
    ablation_records = validator.load_group(version_dir, "ablation")
    fingerprints = validator.check_identities(main_records + ablation_records, "v3")
    main = selected_report(main_records, "comparison-64t.json")
    profile = selected_report(main_records, "profile-64t.json")
    ablation = selected_report(ablation_records, "comparison-64t.json")
    require(not main["metadata"]["mikros_phase_profiling"], "primary timing is instrumented")
    require(not ablation["metadata"]["mikros_phase_profiling"], "ablation timing is instrumented")
    require(profile["metadata"]["mikros_phase_profiling"], "missing instrumented phase measurements")
    main_values = measured_values(main, MAIN_SYSTEMS, log_n=30, runs=8)
    ablation_values = measured_values(ablation, ABLATION_SYSTEMS, log_n=24, runs=16)
    require(profile["metadata"]["logs"] == [30], "profile must have N=2^30")
    require(set(profile["metadata"]["systems"]) == set(MAIN_SYSTEMS), "profile system mismatch")
    phase_runs = profile["metadata"]["runs"]
    require(isinstance(phase_runs, int) and phase_runs > 0, "invalid number of profile runs")
    environment = main_records[0]["environment"]
    return main_values, ablation_values, profile, fingerprints["v3"], environment


def style_axis(ax, *, xlabel: str) -> None:
    ax.set_axisbelow(True)
    ax.grid(axis="x", color="#DEDEDE", linewidth=0.65)
    ax.spines[["top", "right", "left"]].set_visible(False)
    ax.spines["bottom"].set_color("#999999")
    ax.tick_params(axis="y", length=0, pad=9)
    ax.tick_params(axis="x", color="#999999", labelsize=9)
    ax.set_xlabel(xlabel, fontsize=10, labelpad=8)


def point_ranges(ax, values, systems, labels, colors, *, title: str, subtitle: str) -> None:
    maximum = max(max(values[system]) for system in systems)
    for row, system in enumerate(systems):
        samples = values[system]
        color = colors[row]
        # Deterministic offsets show every actual observation, including ties.
        spread = 0.60 if len(samples) <= 8 else 0.75
        offsets = [spread * (index / (len(samples) - 1) - 0.5) for index in range(len(samples))]
        ax.hlines(row, min(samples), max(samples), color=color, linewidth=1.4, zorder=2)
        ax.vlines([min(samples), max(samples)], row - 0.065, row + 0.065,
                  color=color, linewidth=1.2, zorder=2)
        ax.scatter(samples, [row + offset for offset in offsets], s=11 if len(samples) <= 8 else 8,
                   facecolor=color, edgecolor="white", linewidth=0.35, alpha=0.82, zorder=3)
        center = median(samples)
        ax.scatter([center], [row], marker="D", s=42, facecolor="white",
                   edgecolor=color, linewidth=1.6, zorder=4)
        ax.text(maximum * 1.105, row, f"{center:.2f}", va="center", ha="right",
                color="#222222", fontsize=9, fontweight="medium")
    ax.set_yticks(range(len(systems)), labels, fontsize=9)
    ax.set_ylim(len(systems) - 0.48, -0.6)
    ax.set_xlim(0, maximum * 1.15)
    ax.annotate(title, xy=(0, 1), xycoords="axes fraction", xytext=(0, 30), textcoords="offset points",
                fontsize=12, fontweight="bold", va="bottom")
    ax.annotate(subtitle, xy=(0, 1), xycoords="axes fraction", xytext=(0, 14), textcoords="offset points",
                fontsize=9, color="#555555", va="bottom")
    ax.text(0.96, 1.0, "Median", transform=ax.transAxes, ha="right", va="bottom",
            fontsize=8, color="#555555")
    style_axis(ax, xlabel="Prover time (ms)")


def phase_bars(ax, profile) -> None:
    systems = MAIN_SYSTEMS[:3]
    labels = ("Legacy", "Native: fresh", "Native: retained")
    groups = [[], [], [], []]
    totals = []
    for system in systems:
        samples = [sample for sample in profile["samples"] if sample["system"] == system]
        require(len(samples) == profile["metadata"]["runs"], "profile samples incomplete")
        total = mean(sample["prover_ms"] for sample in samples)
        tensor = mean(sample["mikros_phases"]["tensor_aggregate_ms"] for sample in samples)
        ip = mean(sample["mikros_phases"]["inner_product_ms"] for sample in samples)
        packing = mean(sum(sample["mikros_phases"][key]
                           for key in ("setup_ms", "ab_pack_ms", "b_weight_ms", "c_pack_project_ms"))
                       for sample in samples)
        remainder = total - tensor - ip - packing
        require(math.isfinite(remainder) and remainder >= -0.0001, "profile phase sum exceeds total")
        for values, number in zip(groups, (tensor, ip, packing, max(0, remainder))):
            values.append(number)
        totals.append(total)
    left = [0.0] * len(systems)
    for values, color, label in zip(groups, ("#CC6677", "#4477AA", "#228833", "#BBBBBB"),
                                    ("Tensor", "Inner product", "Setup / packing", "Other")):
        ax.barh(range(len(systems)), values, left=left, color=color, height=0.46,
                edgecolor="white", linewidth=0.45, label=label)
        left = [a + b for a, b in zip(left, values)]
    for row, (tensor, total) in enumerate(zip(groups[0], totals)):
        ax.text(total + max(totals) * 0.02, row, f"tensor {100 * tensor / total:.0f}%",
                va="center", fontsize=8, color="#444444")
    ax.set_yticks(range(len(systems)), labels, fontsize=9)
    ax.set_ylim(len(systems) - 0.4, -0.6)
    ax.set_xlim(0, max(totals) * 1.23)
    ax.annotate(r"(c) $N=2^{30}$: where Mikros spends time", xy=(0, 1), xycoords="axes fraction",
                xytext=(0, 30), textcoords="offset points", fontsize=12, fontweight="bold", va="bottom")
    ax.annotate(f"Instrumented means; {profile['metadata']['runs']} measured runs per variant",
                xy=(0, 1), xycoords="axes fraction", xytext=(0, 14), textcoords="offset points",
                fontsize=9, color="#555555", va="bottom")
    ax.legend(loc="upper left", bbox_to_anchor=(0, -0.27), ncols=2, frameon=False,
              fontsize=8, handlelength=1.4, columnspacing=1.5)
    style_axis(ax, xlabel="Instrumented prover time (ms)")


def render(run_dir: Path, loaded) -> tuple[Path, Path]:
    # Data validation above intentionally precedes importing plotting libraries.
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    from matplotlib.lines import Line2D

    main_values, ablation_values, profile, fingerprint, environment = loaded
    plt.rcParams.update({
        "font.family": "DejaVu Sans", "font.size": 10, "svg.fonttype": "none",
        "axes.titleweight": "bold", "savefig.facecolor": "white", "figure.facecolor": "white",
        "pdf.fonttype": 42, "ps.fonttype": 42,
    })
    figure = plt.figure(figsize=(14.8, 8.8))
    grid = figure.add_gridspec(2, 2, left=0.155, right=0.972, top=0.79, bottom=0.235,
                              width_ratios=(1.02, 1.0), wspace=0.64, hspace=0.94)
    main_ax = figure.add_subplot(grid[0, 0])
    phase_ax = figure.add_subplot(grid[1, 0])
    ablation_ax = figure.add_subplot(grid[:, 1])
    point_ranges(main_ax, main_values, MAIN_SYSTEMS, MAIN_LABELS,
                 [COLORS[system] for system in MAIN_SYSTEMS],
                 title=r"(a) $N=2^{30}$: main comparison",
                 subtitle="8 measured runs per variant; 1 warmup excluded")
    point_ranges(ablation_ax, ablation_values, ABLATION_SYSTEMS, ABLATION_LABELS,
                 ["#777777", "#6D8F9C", "#8DA7B7", "#56B4E9", "#3388AA", "#197899", "#0072B2", "#CC79A7"],
                 title=r"(b) $N=2^{24}$: independent ablation",
                 subtitle="16 measured runs per variant; fresh Mikros allocations")
    phase_bars(phase_ax, profile)
    figure.suptitle("Binary Hadamard proof: server benchmarks", x=0.08, y=0.965,
                   ha="left", fontsize=18, fontweight="bold")
    figure.text(0.08, 0.92,
                f"2 x Xeon Gold 6342 | {environment['threads']} threads: "
                f"{environment['selected_physical_cores']} physical cores + "
                f"{environment['additional_smt_siblings']} SMT siblings | GFNI / AVX-512 / VPCLMUL",
                fontsize=10, color="#444444")
    figure.text(0.08, 0.884,
                "Same executable and CPU affinity. PIOP reduction only; PCS absent. Source-oracle checks run outside timings.",
                fontsize=10, color="#444444")
    handles = [
        Line2D([], [], marker="o", linestyle="none", color="#555555", markersize=4, label="Each measured run"),
        Line2D([], [], marker="D", linestyle="none", markerfacecolor="white", markeredgecolor="#333333",
               markeredgewidth=1.5, markersize=5, label="Median"),
        Line2D([], [], marker="|", color="#555555", linewidth=1.4, markersize=8, label="Observed min-max"),
    ]
    figure.legend(handles=handles, loc="lower center", bbox_to_anchor=(0.72, 0.137),
                  ncols=3, frameon=False, fontsize=9, columnspacing=1.2, handlelength=1.5)
    figure.text(0.08, 0.095,
                "Allocation policy: native creates fresh scratch per proof; retained capacity reuses buffers after warmup. "
                "All witness- and challenge-dependent work is rebuilt.", fontsize=8.5, color="#444444")
    figure.text(0.08, 0.063,
                "Panels (a) and (b) use separate linear time scales and different input sizes. "
                "Panel (c) uses a separate instrumented run; its means are not the medians in panel (a).",
                fontsize=8.5, color="#444444")
    figure.text(0.08, 0.03, f"Source fingerprint: {fingerprint}", fontsize=7.5, color="#666666")
    description = (
        "Measured completed v3 server experiments. N30: eight measured runs per variant; "
        "N24: independent ablation with sixteen measured runs. Main native allocations are fresh; "
        "reuse retains capacity after warmup. PIOP reductions only. "
        f"Source fingerprint {fingerprint}. "
        "Inputs: v3/results/comparison-64t.json, v3/results/profile-64t.json, "
        "v3/ablation/comparison-64t.json."
    )
    svg = run_dir / "native64_comparison.svg"
    png = run_dir / "native64_comparison.png"
    figure.savefig(svg, metadata={"Title": "Mikros native64 server benchmarks", "Description": description,
                                  "Creator": "plot_native64.py (matplotlib)", "Date": None})
    figure.savefig(png, dpi=240, metadata={"Software": "plot_native64.py (matplotlib)", "Description": description})
    plt.close(figure)
    return svg, png


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-dir", type=Path, required=True)
    parser.add_argument("--validate-only", action="store_true", help="validate complete source data without writing figures")
    args = parser.parse_args()
    run_dir = args.run_dir.resolve()
    loaded = load_completed(run_dir)
    if args.validate_only:
        print("Complete data validated; no figures written.")
        return
    svg, png = render(run_dir, loaded)
    print(svg)
    print(png)


if __name__ == "__main__":
    main()
