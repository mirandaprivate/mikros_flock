"""Audit four N26 PackedDirect-ON engineering arms without running benchmarks.

Reference, WP, gate and both must use the same frozen source/binary and produce
identical complete proofs. Optional previous ON evidence may differ only in
source/binary identity. Trace runs are rejected, never counted as main samples.
Use --self-test for synthetic validation with no measured report output.
"""

import argparse
import hashlib
import json
import math
from pathlib import Path
import tempfile

from summarize_direct import (
    Audit, CLAIM_COUNTS, COMMON_FIELDS, FEATURES, MODES, PHASES, PIOP_FIELDS,
    RAW_TIMES, ROUTE_ENV, SYSTEMS, arm_summary, require,
)


ARM_ENV = {
    "reference": {},
    "wp": {"FLOCK_DIRECT_WP_SIMD": "1"},
    "gate": {"MIKROS_GATE_SELECTOR_SIMD": "1"},
    "both": {"FLOCK_DIRECT_WP_SIMD": "1", "MIKROS_GATE_SELECTOR_SIMD": "1"},
}
CONFIG_ENV = {
    "direct_wp_simd": "FLOCK_DIRECT_WP_SIMD",
    "gate_selector_simd": "MIKROS_GATE_SELECTOR_SIMD",
}
PROOF_FIELDS = ("proof_blake3", "proof_bytes", *PIOP_FIELDS, *COMMON_FIELDS)


def quotient(numerator, denominator):
    return numerator / denominator if denominator else None


def change(reference, current):
    return {
        "delta_ms": current - reference,
        "time_reduction_pct": 100 * (reference - current) / reference if reference else None,
        "reference_over_current": quotient(reference, current),
    }


def collect_arm(audit, paths, arm):
    previous = arm == "previous_on"
    extras = {} if previous else ARM_ENV[arm]
    data = audit.collect(paths, "previous_on" if previous else "on", extras)
    flags = {key: environment in extras for key, environment in CONFIG_ENV.items()}
    for folder in data["directories"]:
        config = audit.json(Path(folder) / "config.json")
        for key, expected in flags.items():
            # The v8 harness predates these two flags. Absent means its default
            # OFF control; v9 arms must explicitly report both actual booleans.
            actual = config.get(key, False) if previous else config.get(key)
            require(actual is expected, f"{arm} {folder}: config {key} disagrees with arm environment")
    data["identity"].update(flags)
    for case, samples in data["cases"].items():
        require(len(samples) >= 3, f"{arm} {case}: require at least three fresh-process samples")
    return data


def compare(reference_paths, wp_paths, gate_paths, both_paths, previous_on_paths=()):
    audit = Audit()
    arms = {
        arm: collect_arm(audit, paths, arm)
        for arm, paths in zip(ARM_ENV, (reference_paths, wp_paths, gate_paths, both_paths))
    }
    if previous_on_paths:
        arms["previous_on"] = collect_arm(audit, previous_on_paths, "previous_on")
    reference_id = arms["reference"]["identity"]
    for arm, data in arms.items():
        allowed = ({"source_manifest_sha256", "binary_sha256"} if arm == "previous_on"
                   else {"measured_environment", *CONFIG_ENV})
        for key, value in reference_id.items():
            require(key in allowed or data["identity"][key] == value,
                    f"reference/{arm} {key} mismatch")
    report = {
        "schema_version": 1, "log_gates": 26, "threads": 64,
        "scale": "single: N26 gates; multiple: 128 circuits each with N26 gates",
        "scope": "Online commit/recommit, conversion, PIOPs, PCS and release included; offline excluded",
        "statistic": "Medians and min/max ranges of at least three independent fresh processes per arm/case; phase medians need not sum to the total median",
        "interpretation": "Descriptive samples and ranges, not a statistical significance claim; trace runs are excluded by rejection",
        "arms": {arm: {key: data[key] for key in ("identity", "directories")}
                 for arm, data in arms.items()},
        "validation": {
            "all_comparisons_valid": True, "all_arms_packed_direct_on": True,
            "four_arms_same_frozen_source_binary_seed_and_parameters": True,
            "configuration_booleans_match_exact_environment": True,
            "harness_verified_source_and_binary_before_and_after": True,
            "full_proof_hash_bytes_piop_terminal_claims_commitments_and_pcs_match": True,
            "logical_and_ring_claim_counts_match": True,
            "minimum_samples_per_arm_case": 3, "nontrace_only": True,
            "previous_on_only_source_binary_difference_allowed": True if previous_on_paths else None,
        },
        "cases": [], "comparisons": [],
    }
    for mode in MODES:
        mode_cases = {}
        for system in SYSTEMS:
            reference = arms["reference"]["cases"][mode, system][0]
            groups = {}
            for arm, data in arms.items():
                samples = data["cases"][mode, system]
                # Audit.collect already checks every sample within each arm;
                # require the same statement and entire proof across arms too.
                for field in PROOF_FIELDS:
                    require(samples[0][field] == reference[field],
                            f"{mode} {system}: reference/{arm} {field} mismatch")
                for field in CLAIM_COUNTS:
                    require(samples[0]["times"][field] == reference["times"][field],
                            f"{mode} {system}: reference/{arm} {field} mismatch")
                groups[arm] = arm_summary(samples)
            mode_cases[system] = groups
            improvements = {
                arm: {phase: change(groups["reference"]["times_ms"][phase]["median"],
                                    groups[arm]["times_ms"][phase]["median"])
                      for phase in PHASES}
                for arm in ARM_ENV if arm != "reference"
            }
            case = {"mode": mode, "system": system, "arms": groups,
                    "changes_from_reference": improvements}
            if "previous_on" in groups:
                case["changes_from_previous_on"] = {
                    arm: {phase: change(groups["previous_on"]["times_ms"][phase]["median"],
                                        groups[arm]["times_ms"][phase]["median"])
                          for phase in PHASES}
                    for arm in ARM_ENV
                }
            report["cases"].append(case)
        phases = {}
        for phase in PHASES:
            ratios = {}
            for arm in arms:
                m, f = (mode_cases[system][arm]["times_ms"][phase]["median"] for system in SYSTEMS)
                ratios[arm] = {"mikros_over_flock": quotient(m, f), "flock_over_mikros": quotient(f, m)}
            old = ratios["reference"]["flock_over_mikros"]
            relative = {}
            for arm in ARM_ENV:
                if arm == "reference":
                    continue
                new = ratios[arm]["flock_over_mikros"]
                relative[arm] = 100 * (new / old - 1) if old and new is not None else None
            phases[phase] = {"ratios": ratios, "flock_over_mikros_change_pct_from_reference": relative}
        report["comparisons"].append({"mode": mode, "phases": phases})
    report["evidence_sha256"] = audit.evidence
    return report


def fmt(value, digits=3, signed=False):
    return "n/a" if value is None else format(value, f"{'+' if signed else ''}.{digits}f")


def markdown(report):
    lines = ["# N26 PackedDirect ON engineering comparison", "", report["scale"], "",
             report["scope"], "", report["statistic"], "", report["interpretation"], "",
             "Four arms use one frozen source/binary, seed and native PCS configuration. Complete proof hashes/bytes, PIOPs, terminal claims, commitments and logical/ring counts match for each system/mode.", "",
             "| Arm | Extra opt-ins (PackedDirect remains ON) |", "|---|---|"]
    for arm, extras in ARM_ENV.items():
        lines.append(f"| {arm} | " + (", ".join(f"{key}={value}" for key, value in extras.items()) or "none") + " |")
    if "previous_on" in report["arms"]:
        lines += ["", "Previous ON is separately identified: source/binary may differ; all other audited inputs, full proofs and counts must match reference."]
    cases = {(case["mode"], case["system"]): case for case in report["cases"]}
    for comparison in report["comparisons"]:
        mode = comparison["mode"]
        lines += ["", f"## {mode}", "",
                  "Times below are median [min, max] seconds. M/F < 1 means Mikros is faster; F/M > 1 means Mikros is faster.", "",
                  "| Stage | Arm | Mikros (s) | FLOCK (s) | M/F | F/M |",
                  "|---|---|---:|---:|---:|---:|"]
        for phase, label in PHASES.items():
            for arm in report["arms"]:
                times = [cases[mode, system]["arms"][arm]["times_ms"][phase] for system in SYSTEMS]
                cells = [f"{t['median']/1000:.4f} [{t['min']/1000:.4f}, {t['max']/1000:.4f}]" for t in times]
                ratio = comparison["phases"][phase]["ratios"][arm]
                lines.append(f"| {label} | {arm} | {' | '.join(cells)} | {fmt(ratio['mikros_over_flock'])} | {fmt(ratio['flock_over_mikros'])} |")
        lines += ["", "### Changes from reference", "",
                  "Positive time reduction means faster; negative delta means less time. Positive F/M change improves Mikros's relative advantage.", "",
                  "| Stage | Arm | Mikros delta (s) | Mikros reduction | Reference/Mikros speedup | FLOCK delta (s) | FLOCK reduction | Reference/FLOCK speedup | F/M change |",
                  "|---|---|---:|---:|---:|---:|---:|---:|---:|"]
        for phase, label in PHASES.items():
            for arm in ("wp", "gate", "both"):
                cells = []
                for system in SYSTEMS:
                    d = cases[mode, system]["changes_from_reference"][arm][phase]
                    cells.extend((fmt(d["delta_ms"] / 1000, 4, True),
                                  fmt(d["time_reduction_pct"], 2, True) + "%",
                                  fmt(d["reference_over_current"])))
                ratio = comparison["phases"][phase]["flock_over_mikros_change_pct_from_reference"][arm]
                lines.append(f"| {label} | {arm} | {' | '.join(cells)} | {fmt(ratio, 2, True)}% |")
        lines += ["", "| System | Arm | Samples | Proof bytes | Logical W/P | Ring W/P | Verify median [range] ms | RSS median [range] GiB |",
                  "|---|---|---:|---:|---:|---:|---:|---:|"]
        for system in SYSTEMS:
            for arm in report["arms"]:
                a = cases[mode, system]["arms"][arm]
                counts = a["claim_counts"]
                cells = [f"{a[key]['median']:.3f} [{a[key]['min']:.3f}, {a[key]['max']:.3f}]"
                         for key in ("verify_ms", "rss_gib")]
                lines.append(f"| {system} | {arm} | {a['sample_count']} | {a['proof_bytes']} | "
                             f"{counts['witness_claims']}/{counts['public_claims']} | "
                             f"{counts['witness_ring_claims']}/{counts['public_ring_claims']} | {' | '.join(cells)} |")
        lines.append("")
        for system in SYSTEMS:
            for arm in report["arms"]:
                values = [round(s["times"]["prover_ms"] / 1000, 6)
                          for s in cases[mode, system]["arms"][arm]["samples"]]
                lines.append(f"{system} {arm} online seconds, one per process: {values}.")
    lines += ["", "Full per-sample timings, identity, proof fingerprints, resource records, previous-ON changes and input-file SHA-256 digests are in comparison.json.", ""]
    return "\n".join(lines)


def self_test():
    """Synthetic fixtures test complete input auditing and cross-arm boundaries."""
    def h(value):
        return hashlib.sha256(value.encode()).hexdigest()

    def write(path, value):
        path.write_text(json.dumps(value), encoding="utf-8")

    def save(folder, bundle):
        for name, data in bundle.items():
            write(folder / f"{name}.json", data)
        for case in bundle["summary"]["cases"]:
            (folder / case["raw_stdout"]).write_text(
                json.dumps(case["metadata"]) + "\n" + json.dumps(case["samples"][0]) + "\n", encoding="utf-8")

    def fixture(folder, arm, run):
        folder.mkdir()
        previous = arm == "previous_on"
        extras = {} if previous else ARM_ENV[arm]
        config = {
            "levels": [26], "repetitions": 1, "seed": 97, "threads": 64,
            "cpu_affinity": "0-47,48-55,72-79", "rustflags": "-C target-cpu=native",
            "toolchain": "+nightly-2026-03-21", "allocator_policy": "system",
            "diagnostic_trace": False, "disabled_optimizations": [],
            "measured_environment": {ROUTE_ENV: "1", **extras},
            "packed_direct": True, "piop_fingerprints": True,
            "source_manifest_sha256": h("v8" if previous else "v9"),
        }
        if not previous:
            config.update({key: env in extras for key, env in CONFIG_ENV.items()})
        receipt = {key: config[key] for key in ("source_manifest_sha256", "toolchain", "rustflags")}
        receipt["binary_sha256"] = h("binary-v8" if previous else "binary-v9")
        status = {"state": "complete", "source_hashes_verified_before_and_after": True,
                  "binary_hashes_verified_before_and_after": True, "cases": []}
        summary = {"config": config, "all_comparisons_valid": True, "run_state": "complete",
                   "all_requested_cases_complete": True, "source_hashes_verified_before_and_after": True,
                   "cases": [], "comparisons": []}
        for mode in MODES:
            summary["comparisons"].append({"mode": mode, "log_gates": 26, "state": "complete",
                                           "same_pcs_params": True, "same_commitment": True})
            for system in SYSTEMS:
                case_id = f"n26-{mode}-{system}"
                instances = 1 if mode == "single" else 128
                meta = {"type": "metadata", "mode": mode, "system": system, "seed": 97,
                        "instances": instances, "log_gates": 26, "threads": 64,
                        "all_core_threads": 64, "repetitions": 1,
                        "target_features": dict.fromkeys(FEATURES, True)}
                factor = {"reference": 1, "wp": .9, "gate": .8, "both": .7, "previous_on": 1.1}[arm]
                total = (1000 if system == "mikros" else 1500) * (factor if system == "mikros" else 1) * (1 + .01 * run)
                times = dict.fromkeys(RAW_TIMES, total / 20)
                times.update(prover_ms=total, release_ms=0, witness_claims=11, public_claims=8,
                             witness_ring_claims=2, public_ring_claims=8)
                params = {"profile": "fast", "log_inv_rate": 1, "merkle_hash": "blake3", "num_lanes": None}
                sample = {**meta, "type": "sample", "verified": True, "transcript_match": True,
                          "pcs_config_match": True, "native_pcs_identity_transport": True,
                          "repetition": 0, "gates_per_instance": 2**26, "total_gates": instances * 2**26,
                          "proof_bytes": 100, "times": times, "verify_ms": 1,
                          "proof_blake3": h(case_id), "piop_blake3": h(case_id + "piop"),
                          "terminal_claims_blake3": h(case_id + "claims"),
                          "witness_commitment_blake3": h(mode + "witness"), "public_commitment_blake3": h(mode + "public"),
                          "witness_pcs_params": params, "public_pcs_params": params}
                summary["cases"].append({"id": case_id, "mode": mode, "system": system, "state": "complete",
                                         "metadata": meta, "samples": [sample], "raw_stdout": f"{case_id}.stdout.jsonl",
                                         "timings_median_ms": times, "resources": {"available": True, "time_exit_status": 0,
                                         "max_rss_kib": 2**20, "max_rss_gib": 1},
                                         **{key: sample[key] for key in COMMON_FIELDS}})
                status["cases"].append({"id": case_id, "state": "complete", "exit_code": 0})
        save(folder, {"config": config, "summary": summary, "status": status, "build-receipt": receipt})

    with tempfile.TemporaryDirectory(prefix="synthetic-engineering-evidence-") as tmp:
        root = Path(tmp)
        paths = {arm: [root / f"{arm}-{run}" for run in range(3)] for arm in (*ARM_ENV, "previous_on")}
        for arm, folders in paths.items():
            for run, folder in enumerate(folders):
                fixture(folder, arm, run)

        def run(**overrides):
            selected = {**paths, **overrides}
            return compare(*(selected[arm] for arm in ARM_ENV), selected["previous_on"])

        report = run()
        require(len(report["cases"]) == 4, "all four cases")
        first = report["cases"][0]
        require(first["arms"]["reference"]["times_ms"]["prover_ms"] == {"median": 1010, "min": 1000, "max": 1020}, "median/range")
        require(math.isclose(first["changes_from_reference"]["both"]["prover_ms"]["time_reduction_pct"], 30), "reduction")
        require(math.isclose(report["comparisons"][0]["phases"]["prover_ms"]["ratios"]["reference"]["mikros_over_flock"], 2 / 3), "M/F ratio")
        require(first["arms"]["previous_on"]["proof_blake3"] == first["arms"]["both"]["proof_blake3"], "previous proof equality")
        require(first["changes_from_reference"]["wp"]["release_ms"]["time_reduction_pct"] is None, "zero denominator")
        require("median [min, max]" in markdown(report) and "n/a" in markdown(report), "render ranges and zero")
        require("previous_on" not in run(previous_on=[])["arms"], "optional previous ON")
        checks = 8

        def reject(action, expected):
            nonlocal checks
            try:
                action()
            except ValueError as error:
                require(expected in str(error), f"wrong rejection boundary: {error}; expected {expected}")
                checks += 1
            else:
                raise AssertionError(f"Expected rejection: {expected}")

        def mutate(arm, edit, expected, all_runs=True):
            folders = paths[arm] if all_runs else paths[arm][:1]
            backups = {file: file.read_bytes() for folder in folders for file in folder.iterdir()}
            try:
                for folder in folders:
                    bundle = {name: json.loads((folder / f"{name}.json").read_text())
                              for name in ("config", "summary", "status", "build-receipt")}
                    edit(bundle)
                    bundle["summary"]["config"] = bundle["config"]
                    save(folder, bundle)
                reject(run, expected)
            finally:
                for file, data in backups.items():
                    file.write_bytes(data)

        reject(lambda: run(wp=paths["wp"][:2]), "at least three")
        reject(lambda: run(wp=paths["reference"]), "Duplicate evidence directory")
        mutate("wp", lambda b: b["config"].update(direct_wp_simd=False), "config direct_wp_simd")
        mutate("reference", lambda b: b["config"].pop("gate_selector_simd"), "config gate_selector_simd")
        mutate("gate", lambda b: b["config"].update(gate_selector_simd=1), "config gate_selector_simd")
        mutate("both", lambda b: b["config"]["measured_environment"].update(UNEXPECTED="1"), "unexpected measured environment")
        mutate("wp", lambda b: b["config"].update(packed_direct=False), "route/fingerprint")
        mutate("gate", lambda b: b["config"].update(diagnostic_trace=True), "nontrace")
        mutate("both", lambda b: b["status"].update(state="running"), "incomplete or invalid")
        mutate("wp", lambda b: b["build-receipt"].update(binary_sha256=h("different")), "reference/wp binary_sha256")

        def source_change(b):
            b["config"]["source_manifest_sha256"] = h("different")
            b["build-receipt"]["source_manifest_sha256"] = h("different")

        mutate("gate", source_change, "reference/gate source_manifest_sha256")
        for arm in ("both", "previous_on"):
            for field in ("proof_blake3", "proof_bytes", *PIOP_FIELDS):
                value = 101 if field == "proof_bytes" else h("wrong " + field)
                mutate(arm, lambda b, f=field, v=value: b["summary"]["cases"][0]["samples"][0].update({f: v}),
                       f"reference/{arm} {field} mismatch")
            for field in CLAIM_COUNTS:
                mutate(arm, lambda b, f=field: b["summary"]["cases"][0]["samples"][0]["times"].update({f: 99}),
                       f"reference/{arm} {field} mismatch")
        mutate("wp", lambda b: b["summary"]["cases"][0]["samples"][0].update(proof_blake3=h("unstable")),
               "not stable within arm", all_runs=False)
        mutate("previous_on", lambda b: b["config"].update(gate_selector_simd=True), "config gate_selector_simd")

        def params_change(b):
            for case in b["summary"]["cases"]:
                for key in COMMON_FIELDS[2:]:
                    case[key]["m"] = 123
                    case["samples"][0][key]["m"] = 123

        mutate("wp", params_change, "reference/wp witness_pcs_params mismatch")

        for field in COMMON_FIELDS[:2]:
            def commitment_change(b, field=field):
                # Keep Mikros/FLOCK and summary/raw consistency valid so this
                # reaches the intended cross-arm commitment boundary.
                for case in b["summary"]["cases"]:
                    if case["mode"] == "single":
                        case[field] = h("changed " + field)
                        case["samples"][0][field] = h("changed " + field)

            mutate("both", commitment_change, f"reference/both {field} mismatch")

        def seed_change(b):
            b["config"]["seed"] = 98
            for case in b["summary"]["cases"]:
                case["metadata"]["seed"] = 98
                case["samples"][0]["seed"] = 98

        mutate("previous_on", seed_change, "reference/previous_on seed mismatch")
        print(f"Synthetic self-test passed ({checks} checks); no benchmark executed or measured report produced.")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for arm in ARM_ENV:
        parser.add_argument(f"--{arm}", nargs="+", type=Path)
    parser.add_argument("--previous-on", nargs="+", type=Path, default=[],
                        help="Earlier PackedDirect ON evidence; only source and binary may differ")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    inputs = [getattr(args, arm) for arm in ARM_ENV]
    if args.self_test:
        require(not (any(inputs) or args.previous_on or args.output),
                "--self-test cannot consume measured evidence or create reports")
        self_test()
        return
    if not all(inputs) or not args.output:
        parser.error("--reference, --wp, --gate, --both and --output are required")
    report = compare(*inputs, args.previous_on)
    rendered = markdown(report)
    args.output.mkdir(parents=True, exist_ok=True)
    (args.output / "comparison.json").write_text(json.dumps(report, indent=2, allow_nan=False) + "\n", encoding="utf-8")
    (args.output / "comparison.md").write_text(rendered, encoding="utf-8")
    print(rendered)


if __name__ == "__main__":
    main()
