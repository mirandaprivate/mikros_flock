#!/usr/bin/env python3
"""Plot measured PLONK width/size results; unavailable points remain gaps.

Input is the schema-v1 checkpoint from plonk_bitwidth_sweep.py. Only validated
`ok` cases supply metric values. The source file's SHA-256, measurement scope,
and missing-case statuses accompany every export. No hostname, username,
absolute path, or timestamp is copied into figure metadata.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
from pathlib import Path

STATUSES = frozenset({
    "pending", "running", "ok", "memory_skipped", "pcs_unsupported", "guarded", "error",
})
WIDTHS = (1, 2, 4, 8, 16, 32, 64, 128)
METRICS = {
    "prover": ("prover_ms", 1000.0, "Prover time (s)", "log"),
    "ram": ("ram_mib", 1024.0, "Peak RAM (GiB)", "log"),
    "proof": ("proof_bytes", 1024.0, "Proof size (KiB)", "linear"),
    "verifier": ("verify_ms", 1.0, "Verifier time (ms)", "linear"),
}
COLORS = ("#0072B2", "#D55E00", "#009E73", "#CC79A7", "#56B4E9", "#9B8500", "#7B4AB5", "#333333")
MARKERS = ("o", "s", "^", "D", "v", "P", "X", "h")
SCOPE = "Full Mikros PLONK proof, including native Ligerito PCS"


def positive_int(value: object, label: str, *, zero: bool = False) -> int:
    if isinstance(value, bool) or not isinstance(value, int) or value < (0 if zero else 1):
        raise ValueError(f"{label} must be a {'nonnegative' if zero else 'positive'} integer")
    return value


def integer_grid(value: object, label: str) -> tuple[int, ...]:
    if not isinstance(value, list) or not value:
        raise ValueError(f"{label} must be a nonempty list")
    values = tuple(positive_int(v, label) for v in value)
    if len(set(values)) != len(values):
        raise ValueError(f"duplicate {label}")
    return tuple(sorted(values))


def normalize(document: object, source_sha256: str) -> dict:
    """Validate the grid and retain only fields needed by the anonymous plots."""
    if not isinstance(document, dict) or document.get("schema_version") != 1:
        raise ValueError("expected PLONK sweep schema_version=1")
    if document.get("experiment") != "plonk-bitwidth-sweep":
        raise ValueError("expected experiment='plonk-bitwidth-sweep'")
    config = document.get("config")
    if not isinstance(config, dict):
        raise ValueError("missing sweep config")
    logs = integer_grid(config.get("log_ns"), "log_ns")
    widths = integer_grid(config.get("bits"), "bits")
    if any(n > 62 for n in logs):
        raise ValueError("log_ns exceeds supported plotting range")
    if any(b not in WIDTHS for b in widths):
        raise ValueError("unsupported field bitwidth")
    threads = positive_int(config.get("threads"), "threads")
    reps = positive_int(config.get("reps"), "reps")
    warmups = positive_int(config.get("warmups"), "warmups", zero=True)
    raw_cases = document.get("cases")
    if not isinstance(raw_cases, list):
        raise ValueError("missing cases list")
    points = {}
    for case in raw_cases:
        if not isinstance(case, dict):
            raise ValueError("each case must be an object")
        n = positive_int(case.get("log_n"), "case.log_n")
        b = positive_int(case.get("value_bits"), "case.value_bits")
        key = (n, b)
        if n not in logs or b not in widths:
            raise ValueError(f"case {key} outside configured grid")
        if key in points:
            raise ValueError(f"duplicate case {key}")
        status = case.get("status")
        if status not in STATUSES:
            raise ValueError(f"unknown case status: {status!r}")
        metrics = case.get("metrics")
        if not isinstance(metrics, dict):
            raise ValueError(f"case {key}: metrics must be an object")
        values = {}
        for metric, (field, divisor, _, _) in METRICS.items():
            value = metrics.get(field)
            if status != "ok":
                if value is not None:
                    raise ValueError(f"case {key}: non-ok metric {field} must be null")
                values[metric] = math.nan
                continue
            if isinstance(value, bool) or not isinstance(value, (int, float)) or not math.isfinite(value):
                raise ValueError(f"case {key}: invalid {field}")
            if value < 0 or (metric != "verifier" and value == 0):
                raise ValueError(f"case {key}: nonpositive {field}")
            if metric == "proof" and int(value) != value:
                raise ValueError(f"case {key}: proof_bytes must be integral")
            values[metric] = value / divisor
        # Reason text can contain paths/hosts from exception diagnostics. Export
        # only the public status code; the checkpoint retains the exact reason.
        points[key] = {"status": status, "values": values}
    missing = []
    for n in logs:
        for b in widths:
            key = (n, b)
            if key not in points:
                points[key] = {"status": "absent", "values": {m: math.nan for m in METRICS}}
            if points[key]["status"] != "ok":
                missing.append({"log_n": n, "value_bits": b, "status": points[key]["status"]})
    return {
        "source_sha256": source_sha256,
        "logs": logs,
        "widths": widths,
        "threads": threads,
        "reps": reps,
        "warmups": warmups,
        "points": points,
        "missing": missing,
    }


def load_checkpoint(path: Path) -> dict:
    raw = path.read_bytes()
    try:
        document = json.loads(raw)
    except (UnicodeDecodeError, json.JSONDecodeError) as exc:
        raise ValueError(f"invalid checkpoint JSON: {exc}") from exc
    return normalize(document, hashlib.sha256(raw).hexdigest())


def series(data: dict, width: int, metric: str) -> list[float]:
    """Keep every configured x coordinate, including NaN at missing cases."""
    return [data["points"][(n, width)]["values"][metric] for n in data["logs"]]


def caption(data: dict) -> str:
    return (
        f"{data['threads']} workers; {data['reps']} measured "
        f"{'proof' if data['reps'] == 1 else 'proofs'} per point; "
        f"{data['warmups']} warmups; full PLONK + native Ligerito PCS"
    )


def provenance(data: dict, scales: dict[str, str]) -> dict:
    return {
        "schema_version": 1,
        "experiment": "plonk-bitwidth-sweep-plots",
        "source_sha256": data["source_sha256"],
        "scope": SCOPE,
        "threads": data["threads"],
        "samples_per_point": data["reps"],
        "warmups": data["warmups"],
        "log_ns": list(data["logs"]),
        "value_bits": list(data["widths"]),
        "scales": scales,
        "missing_policy": "NaN gaps; no zero substitution, interpolation, or extrapolation",
        "missing_cases": data["missing"],
    }


def metadata(kind: str, data: dict) -> dict:
    description = f"{SCOPE}. {caption(data)}. Source SHA-256: {data['source_sha256']}"
    if kind == "png":
        return {"Title": "Mikros PLONK bitwidth sweep", "Description": description, "Software": "Mikros artifact"}
    if kind == "pdf":
        return {"Title": "Mikros PLONK bitwidth sweep", "Subject": description,
                "Author": None, "Creator": "Mikros artifact", "Producer": None,
                "CreationDate": None, "ModDate": None}
    return {"Title": "Mikros PLONK bitwidth sweep", "Description": description,
            "Creator": "Mikros artifact", "Date": None}


def draw_axis(ax, data: dict, metric: str, scale: str) -> None:
    from matplotlib.ticker import NullLocator

    x = [2**n for n in data["logs"]]
    if scale == "log":
        for b in data["widths"]:
            if any(math.isfinite(v) and v <= 0 for v in series(data, b, metric)):
                raise ValueError(f"{metric}: log scale cannot display a measured zero")
    for b in data["widths"]:
        style = WIDTHS.index(b)
        ax.plot(x, series(data, b, metric), label=f"{b}-bit",
                color=COLORS[style], marker=MARKERS[style], linewidth=1.8,
                markersize=5.5, markeredgewidth=0.7, markeredgecolor="white")
    ax.set_xscale("log", base=2)
    ax.set_yscale(scale)
    ax.set_xticks(x, [rf"$2^{{{n}}}$" for n in data["logs"]])
    ax.xaxis.set_minor_locator(NullLocator())
    if len(x) == 1:
        ax.set_xlim(x[0] / 1.5, x[0] * 1.5)
    else:
        ax.set_xlim(x[0] / 1.3, x[-1] * 1.3)
    ax.set_xlabel("Circuit gates")
    ax.set_ylabel(METRICS[metric][2])
    ax.grid(axis="both", which="major", color="#D5DAE0", linewidth=0.6, alpha=0.8)
    ax.grid(axis="y", which="minor", color="#E9EDF0", linewidth=0.4, alpha=0.6)
    ax.spines[["top", "right"]].set_visible(False)
    ax.set_axisbelow(True)
    if scale == "linear":
        ax.set_ylim(bottom=0)


def footnote(fig, data: dict) -> None:
    count = len(data["missing"])
    total = len(data["logs"]) * len(data["widths"])
    text = f"Unavailable: {count}/{total} points (gaps; no imputation). " if count else "All configured points measured. "
    text += f"Source SHA-256: {data['source_sha256'][:16]}…"
    fig.text(0.5, 0.02, text, ha="center", va="bottom", fontsize=8, color="#4A5360")


def render(data: dict, output: Path, *, formats: tuple[str, ...] = ("png", "pdf", "svg"),
           dpi: int = 180, scales: dict[str, str] | None = None) -> dict:
    try:
        import matplotlib
        matplotlib.use("Agg")
        import matplotlib.pyplot as plt
    except ImportError as exc:
        raise RuntimeError("Matplotlib is required: python -m pip install matplotlib") from exc
    scales = scales or {name: spec[3] for name, spec in METRICS.items()}
    if set(scales) != set(METRICS) or any(s not in {"log", "linear"} for s in scales.values()):
        raise ValueError("one linear/log scale is required for each metric")
    if not formats or any(f not in {"png", "pdf", "svg"} for f in formats) or len(set(formats)) != len(formats):
        raise ValueError("formats must be unique members of png,pdf,svg")
    positive_int(dpi, "dpi")
    output.mkdir(parents=True, exist_ok=True)
    exported = []

    def save(fig, stem):
        for kind in formats:
            path = output / f"{stem}.{kind}"
            fig.savefig(path, dpi=dpi, metadata=metadata(kind, data), facecolor="white")
            exported.append({"name": path.name, "sha256": hashlib.sha256(path.read_bytes()).hexdigest()})
        plt.close(fig)

    with plt.rc_context({"font.family": "DejaVu Sans", "font.size": 10,
                         "svg.hashsalt": data["source_sha256"], "pdf.fonttype": 42,
                         "svg.fonttype": "none", "axes.labelcolor": "#253040",
                         "xtick.color": "#253040", "ytick.color": "#253040"}):
        fig, axes = plt.subplots(2, 2, figsize=(12.8, 8.8))
        for ax, metric in zip(axes.flat, METRICS):
            draw_axis(ax, data, metric, scales[metric])
        fig.suptitle("Mikros PLONK: field width and circuit size", fontsize=17, y=0.985)
        fig.text(0.5, 0.945, caption(data), ha="center", fontsize=10, color="#4A5360")
        handles, labels = axes[0, 0].get_legend_handles_labels()
        fig.legend(handles, labels, ncol=4, loc="upper center", bbox_to_anchor=(0.5, 0.925), frameon=False)
        fig.subplots_adjust(left=0.09, right=0.98, bottom=0.11, top=0.80, wspace=0.24, hspace=0.37)
        footnote(fig, data)
        save(fig, "plonk_bitwidth_sweep")
        for metric in METRICS:
            fig, ax = plt.subplots(figsize=(8.6, 6.1))
            draw_axis(ax, data, metric, scales[metric])
            fig.suptitle(f"Mikros PLONK — {METRICS[metric][2]}", fontsize=15, y=0.985)
            fig.text(0.5, 0.935, caption(data), ha="center", fontsize=8.5, color="#4A5360")
            handles, labels = ax.get_legend_handles_labels()
            fig.legend(handles, labels, ncol=4, loc="upper center", bbox_to_anchor=(0.5, 0.915), frameon=False)
            fig.subplots_adjust(left=0.12, right=0.98, bottom=0.14, top=0.75)
            footnote(fig, data)
            save(fig, f"plonk_bitwidth_{metric}")
    manifest = provenance(data, scales)
    manifest["figures"] = exported
    (output / "figures_manifest.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    return manifest


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("checkpoint", type=Path, help="sweep checkpoint.json")
    parser.add_argument("--out", type=Path, help="output directory; default: checkpoint's figures/ directory")
    parser.add_argument("--formats", default="png,pdf,svg", help="comma-separated subset of png,pdf,svg")
    parser.add_argument("--dpi", type=int, default=180)
    for name, spec in METRICS.items():
        parser.add_argument(f"--{name}-scale", choices=("linear", "log"), default=spec[3])
    args = parser.parse_args(argv)
    try:
        data = load_checkpoint(args.checkpoint)
        manifest = render(data, args.out or args.checkpoint.parent / "figures",
                          formats=tuple(args.formats.split(",")), dpi=args.dpi,
                          scales={name: getattr(args, f"{name}_scale") for name in METRICS})
    except (OSError, ValueError, RuntimeError) as exc:
        parser.error(str(exc))
    print(f"Rendered {len(manifest['figures'])} figures; {len(data['missing'])} unavailable grid points remain gaps.")
    print(f"{caption(data)}. Source SHA-256: {data['source_sha256']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
