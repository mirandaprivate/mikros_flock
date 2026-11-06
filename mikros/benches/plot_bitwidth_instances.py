#!/usr/bin/env python3
"""Four-panel plots of measured total costs; unavailable cases remain gaps."""
import argparse
import csv
import hashlib
import io
import json
import math
from pathlib import Path

WIDTHS = (1, 2, 4, 8, 16, 32, 64, 128)
CASES = ("single", "multiple")
INSTANCES = {"single": 1, "multiple": 256}
ARITHMETIZATION = {"single": "PLONK", "multiple": "Packed R1CS"}
LABELS = {"single": "Single PLONK (1 instance)", "multiple": "Packed R1CS (256 instances, total)"}
STATUSES = {"pending", "running", "ok", "memory_skipped", "guarded", "error", "absent"}
METRICS = {
    "prover": ("prover_ms", 1000, "Prover time (s)", "log"),
    "ram": ("ram_mib", 1024, "Peak RAM (GiB)", "log"),
    "proof": ("proof_bytes", 1024, "Proof size (KiB)", "linear"),
    "verifier": ("verify_ms", 1, "Verifier time (ms)", "log"),
}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def positive_int(value, label, zero=False):
    require(type(value) is int and (value >= 0 if zero else value > 0), f"invalid {label}")
    return value


def normalize(doc, source_sha256):
    require(isinstance(doc, dict) and doc.get("schema_version") == 1 and doc.get("experiment") == "bitwidth-instances", "unexpected checkpoint schema")
    cfg = doc.get("config")
    require(isinstance(cfg, dict), "missing config")
    bits = cfg.get("bits")
    require(isinstance(bits, list) and bits and all(type(b) is int and b in WIDTHS for b in bits)
            and len(bits) == len(set(bits)), "invalid field widths")
    bits = sorted(bits)
    n = positive_int(cfg.get("log_n"), "log_n")
    require(14 <= n <= 30, "unsupported log_n")
    threads, reps, warmups = (positive_int(cfg.get(k), k, zero=k == "warmups") for k in ("threads", "reps", "warmups"))
    require(isinstance(doc.get("cases"), list), "missing cases")
    points = {}
    for row in doc["cases"]:
        require(isinstance(row, dict), "invalid case")
        case, b = row.get("case"), row.get("value_bits")
        require(case in CASES and type(b) is int and b in bits, "case outside requested grid")
        key = (case, b)
        require(key not in points, "duplicate case")
        for name, expected in (("log_n", n), ("instances", INSTANCES[case]), ("kappa", 128//b),
                               ("gates_per_instance", 1 << n), ("total_gates", (1 << n)*INSTANCES[case]),
                               ("arithmetization", ARITHMETIZATION[case])):
            require(type(row.get(name)) is type(expected) and row[name] == expected, f"incorrect {name}")
        status = row.get("status")
        require(status in STATUSES, "unknown status")
        metrics = row.get("metrics")
        require(isinstance(metrics, dict), "missing metrics")
        values = {}
        for name, (field, divisor, _, _) in METRICS.items():
            value = metrics.get(field)
            if status != "ok":
                require(value is None, "unsuccessful case contains a metric")
                values[name] = math.nan
            else:
                require(type(value) in (int, float) and math.isfinite(value) and value > 0, f"invalid {field}")
                values[name] = value / divisor
        points[key] = {"status": status, "values": values}
    for case in CASES:
        for b in bits:
            points.setdefault((case, b), {"status": "absent", "values": {k: math.nan for k in METRICS}})
    return {"source_sha256": source_sha256, "bits": bits, "log_n": n, "threads": threads,
            "reps": reps, "warmups": warmups, "points": points}


def load(path):
    raw = Path(path).read_bytes()
    return normalize(json.loads(raw), hashlib.sha256(raw).hexdigest())


def series(data, case, metric):
    return [data["points"][(case, b)]["values"][metric] for b in data["bits"]]


def provenance(data):
    return {"schema_version": 1, "experiment": "bitwidth-instances-plots", "source_sha256": data["source_sha256"],
            "log_gates_per_instance": data["log_n"], "bits": data["bits"], "threads": data["threads"],
            "samples": data["reps"], "warmups": data["warmups"],
            "pcs": "F128 additive FRI", "cost_scope": "total proof cost; no division by 256",
            "arithmetizations": ARITHMETIZATION, "instances": INSTANCES,
            "packing": "kappa=128/b; b=128 has kappa=1 and no subfield packing gain",
            "ram_scope": "GNU time whole-process peak RSS, including setup, warmups, proving and verification",
            "missing_policy": "NaN gaps; no substitution, interpolation, or extrapolation",
            "missing": [{"case": c, "value_bits": b, "status": data["points"][(c,b)]["status"]}
                        for c in CASES for b in data["bits"] if data["points"][(c,b)]["status"] != "ok"]}


def csv_text(data):
    stream = io.StringIO(newline="")
    writer = csv.writer(stream)
    writer.writerow(["case", "instances", "arithmetization", "gates_per_instance", "total_gates", "value_bits", "kappa",
                     "status", "prover_s", "ram_gib", "proof_kib", "verifier_ms"])
    for b in data["bits"]:
        for case in CASES:
            point = data["points"][(case,b)]
            values = [point["values"][m] for m in METRICS]
            writer.writerow([case, INSTANCES[case], ARITHMETIZATION[case], 1 << data["log_n"],
                             (1 << data["log_n"])*INSTANCES[case], b, 128//b, point["status"],
                             *["" if not math.isfinite(v) else v for v in values]])
    return stream.getvalue()


def draw(data, *, paper=False):
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    from matplotlib.ticker import NullLocator
    fig, axes = plt.subplots(2, 2, figsize=(4.8, 4.4) if paper else (10.6, 7.8))
    x = [b.bit_length()-1 for b in data["bits"]]
    for ax, (metric, (_, _, label, scale)) in zip(axes.flat, METRICS.items()):
        for case, color, marker in (("single", "#0072B2", "o"), ("multiple", "#D55E00", "s")):
            label_text = {"single": "PLONK (single)", "multiple": "Packed R1CS (256)"}[case] if paper else LABELS[case]
            ax.plot(x, series(data, case, metric), color=color, marker=marker, markersize=3.2 if paper else 5.5,
                    linewidth=1.1 if paper else 1.8, label=label_text)
        ax.set_xticks(x, [str(b) for b in data["bits"]])
        ax.xaxis.set_minor_locator(NullLocator())
        ax.set_xlabel(r"Bitwidth $b$" if paper else "Witness field bitwidth b", fontsize=8 if paper else None)
        ax.set_ylabel(label, fontsize=8 if paper else None)
        if paper:
            ax.tick_params(axis="both", which="both", labelsize=8, pad=2)
        ax.set_yscale(scale)
        ax.grid(True, alpha=0.23)
        if 128 in data["bits"]:
            ax.axvline(7, color="#777777", linewidth=0.8, linestyle=":")
    handles, labels = axes.flat[0].get_legend_handles_labels()
    if paper:
        # At the 12.2 cm LLNCS text width, the nominal 8 pt labels remain 8 pt.
        # Experiment scope and sampling belong in the accompanying caption.
        fig.legend(handles, labels, loc="upper center", bbox_to_anchor=(0.5, 0.995), ncol=2, frameon=False, fontsize=8)
        fig.tight_layout(rect=(0, 0, 1, 0.94), pad=0.6, w_pad=1.0, h_pad=1.2)
    else:
        fig.legend(handles, labels, loc="upper center", bbox_to_anchor=(0.5, 0.95), ncol=2, frameon=False)
        fig.suptitle(rf"Mikros: $2^{{{data['log_n']}}}$ gates per instance; F128 FRI; {data['threads']} thread(s)", y=0.99, fontsize=13)
        fig.supxlabel("Total costs; batch = 256 independent instances.  κ = 128/b; κ = 1 at b = 128.\n"
                      f"{data['reps']} measured sample(s), {data['warmups']} warmups; missing results remain gaps.", y=0.02, fontsize=10)
        fig.tight_layout(rect=(0, 0.085, 1, 0.90), pad=1.0)
    return fig


def export(source, out):
    data = load(source)
    out = Path(out)
    out.mkdir(parents=True, exist_ok=True)
    public = provenance(data)
    (out / "bitwidth_instances.csv").write_text(csv_text(data), encoding="utf-8", newline="")
    (out / "bitwidth_instances.provenance.json").write_text(json.dumps(public, indent=2, allow_nan=False)+"\n", encoding="utf-8")
    fig = draw(data)
    description = (f"Full F128-FRI proofs; 2^{data['log_n']} gates per instance; total single PLONK and 256-instance Packed R1CS costs. "
                   f"Source SHA-256: {data['source_sha256']}")
    pdf_metadata = {"Title": "Mikros field-width and instance comparison", "Subject": description,
                    "Author": None, "Creator": "Mikros artifact", "Producer": None, "CreationDate": None, "ModDate": None}
    for suffix, metadata in (
        ("pdf", pdf_metadata),
        ("png", {"Title": "Mikros field-width and instance comparison", "Description": description, "Software": "Mikros artifact"}),
        ("svg", {"Title": "Mikros field-width and instance comparison", "Description": description, "Creator": "Mikros artifact", "Date": None}),
    ):
        fig.savefig(out / f"bitwidth_instances.{suffix}", dpi=220, metadata=metadata)
    import matplotlib.pyplot as plt
    plt.close(fig)
    paper = draw(data, paper=True)
    paper.savefig(out / "bitwidth_instances_paper.pdf", metadata=pdf_metadata)
    plt.close(paper)
    return public


def main(argv=None):
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--input", type=Path, required=True)
    p.add_argument("--out", type=Path)
    args = p.parse_args(argv)
    export(args.input, args.out or args.input.parent / "plots")


if __name__ == "__main__":
    main()
