"""Audit four N26 native64 memory-traffic arms; this never runs benchmarks.

All arms use PackedDirect ON and v9 arithmetic optimizations OFF. Gate fusion
and WP uninitialized output are independently enabled. WP applies to both M/F.
Previous reference may differ only in source/binary and may omit the two new
v10 flags. Complete proofs and all other audited inputs must remain equal.
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
from summarize_engineering import PROOF_FIELDS, change, fmt, quotient


FUSION_ENV = "MIKROS_GATE_FUSION"
WP_ENV = "FLOCK_WP_UNINIT"
ARM_ENV = {
    "reference": {},
    "fusion": {FUSION_ENV: "1"},
    "wp_uninit": {WP_ENV: "1"},
    "both": {FUSION_ENV: "1", WP_ENV: "1"},
}
CONFIG_ENV = {"gate_fusion": FUSION_ENV, "wp_uninit": WP_ENV}
FLAGS = ("direct_wp_simd", "gate_selector_simd", *CONFIG_ENV)


def collect_arm(audit, paths, arm):
    previous = arm == "previous_reference"
    extras = {} if previous else ARM_ENV[arm]
    data = audit.collect(paths, "previous_on" if previous else "on", extras)
    flags = {"direct_wp_simd": False, "gate_selector_simd": False,
             **{key: env in extras for key, env in CONFIG_ENV.items()}}
    for folder in data["directories"]:
        config = audit.json(Path(folder) / "config.json")
        for key, expected in flags.items():
            actual = config.get(key, False) if previous and key in CONFIG_ENV else config.get(key)
            require(actual is expected, f"{arm} {folder}: config {key} disagrees with arm environment")
    data["identity"].update(flags)
    for case, samples in data["cases"].items():
        require(len(samples) >= 3, f"{arm} {case}: require at least three fresh-process samples")
    return data


def compare(reference_paths, fusion_paths, wp_uninit_paths, both_paths, previous_reference_paths=()):
    audit = Audit()
    arms = {arm: collect_arm(audit, paths, arm)
            for arm, paths in zip(ARM_ENV, (reference_paths, fusion_paths, wp_uninit_paths, both_paths))}
    if previous_reference_paths:
        arms["previous_reference"] = collect_arm(audit, previous_reference_paths, "previous_reference")
    reference_id = arms["reference"]["identity"]
    for arm, data in arms.items():
        allowed = ({"source_manifest_sha256", "binary_sha256"} if arm == "previous_reference"
                   else {"measured_environment", *CONFIG_ENV})
        for key, value in reference_id.items():
            require(key in allowed or data["identity"][key] == value,
                    f"reference/{arm} {key} mismatch")
    report = {
        "schema_version": 1, "log_gates": 26, "threads": 64,
        "experiment": "four-arm memory-traffic ablation; WP uninitialized output applies to both Mikros and FLOCK",
        "scale": "single: N26 gates; multiple: 128 circuits each with N26 gates",
        "scope": "Online commit/recommit, conversion, PIOPs, PCS and release included; offline excluded",
        "statistic": "Median [min, max] of at least three fresh processes per arm/case; phase medians need not sum to the total median",
        "interpretation": "Descriptive samples, not a significance claim; trace runs are rejected rather than counted",
        "arms": {arm: {key: data[key] for key in ("identity", "directories")}
                 for arm, data in arms.items()},
        "validation": {
            "all_comparisons_valid": True, "all_arms_packed_direct_on": True,
            "v9_arithmetic_optimizations_off": True, "same_v10_source_binary_seed_and_parameters": True,
            "configuration_booleans_match_exact_environment": True,
            "source_binary_verified_before_and_after": True,
            "full_proof_hash_bytes_piop_claims_commitments_pcs_equal": True,
            "logical_and_ring_counts_equal": True, "minimum_samples_per_arm_case": 3,
            "nontrace_only": True,
            "previous_reference_only_source_binary_difference_allowed": True if previous_reference_paths else None,
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
                # The reused Audit checks all samples within the arm; these
                # comparisons then enforce the same complete proof across arms.
                for field in PROOF_FIELDS:
                    require(samples[0][field] == reference[field],
                            f"{mode} {system}: reference/{arm} {field} mismatch")
                for field in CLAIM_COUNTS:
                    require(samples[0]["times"][field] == reference["times"][field],
                            f"{mode} {system}: reference/{arm} {field} mismatch")
                groups[arm] = arm_summary(samples)
            mode_cases[system] = groups
            case = {
                "mode": mode, "system": system, "arms": groups,
                "changes_from_reference": {
                    arm: {phase: change(groups["reference"]["times_ms"][phase]["median"],
                                        groups[arm]["times_ms"][phase]["median"])
                          for phase in PHASES}
                    for arm in ARM_ENV if arm != "reference"},
                "primary_samples_ms": {
                    arm: [{"directory": sample["directory"],
                           **{key: sample["times"][key] for key in RAW_TIMES}}
                          for sample in summary["samples"]]
                    for arm, summary in groups.items()},
            }
            if "previous_reference" in groups:
                case["changes_from_previous_reference"] = {
                    arm: {phase: change(groups["previous_reference"]["times_ms"][phase]["median"],
                                        groups[arm]["times_ms"][phase]["median"])
                          for phase in PHASES}
                    for arm in ARM_ENV}
            report["cases"].append(case)
        phases = {}
        for phase in PHASES:
            ratios = {}
            for arm in arms:
                m, f = (mode_cases[system][arm]["times_ms"][phase]["median"] for system in SYSTEMS)
                ratios[arm] = {"mikros_over_flock": quotient(m, f), "flock_over_mikros": quotient(f, m)}
            old = ratios["reference"]["flock_over_mikros"]
            phases[phase] = {
                "ratios": ratios,
                "flock_over_mikros_relative_gain_pct": {
                    arm: (100 * (ratios[arm]["flock_over_mikros"] / old - 1)
                          if old and ratios[arm]["flock_over_mikros"] is not None else None)
                    for arm in ARM_ENV if arm != "reference"},
            }
        report["comparisons"].append({"mode": mode, "phases": phases})
    report["evidence_sha256"] = audit.evidence
    return report


def markdown(report):
    lines = ["# N26 four-arm memory-traffic ablation", "", report["scale"], "", report["scope"], "",
             report["statistic"], "", report["interpretation"], "",
             "PackedDirect is ON throughout. FLOCK_DIRECT_WP_SIMD and MIKROS_GATE_SELECTOR_SIMD are OFF. WP uninitialized output applies equally to Mikros and FLOCK; relative benefit is assessed from measured ratios.",
             "Current arms share one frozen source/binary and produce identical complete proof hashes/bytes, PIOPs, claims, commitments, PCS parameters and logical/ring counts.", ""]
    lines += ["| Arm | Extra opt-ins |", "|---|---|"]
    for arm, extras in ARM_ENV.items():
        lines.append(f"| {arm} | " + (", ".join(f"{key}={value}" for key, value in extras.items()) or "none") + " |")
    lines.append("")
    if "previous_reference" in report["arms"]:
        lines += ["Previous reference is separate evidence: only its source/binary may differ; full proofs and all other audited inputs match current reference.", ""]
    cases = {(case["mode"], case["system"]): case for case in report["cases"]}
    for comparison in report["comparisons"]:
        mode = comparison["mode"]
        lines += [f"## {mode}", "", "M/F < 1 and F/M > 1 mean Mikros is faster. Times are median [min, max] seconds.", "",
                  "| Stage | Arm | Mikros (s) | FLOCK (s) | M/F | F/M |", "|---|---|---:|---:|---:|---:|"]
        for phase, label in PHASES.items():
            for arm in report["arms"]:
                cells = []
                for system in SYSTEMS:
                    t = cases[mode, system]["arms"][arm]["times_ms"][phase]
                    cells.append(f"{t['median']/1000:.4f} [{t['min']/1000:.4f}, {t['max']/1000:.4f}]")
                ratio = comparison["phases"][phase]["ratios"][arm]
                lines.append(f"| {label} | {arm} | {' | '.join(cells)} | {fmt(ratio['mikros_over_flock'])} | {fmt(ratio['flock_over_mikros'])} |")
        lines += ["", "Positive time reduction means faster; negative delta means less time. Positive relative F/M gain improves Mikros's relative advantage.", "",
                  "| Stage | Arm | Mikros delta (s) | Mikros reduction | M reference/arm | FLOCK delta (s) | FLOCK reduction | F reference/arm | Relative F/M gain |",
                  "|---|---|---:|---:|---:|---:|---:|---:|---:|"]
        for phase, label in PHASES.items():
            for arm in ARM_ENV:
                if arm == "reference":
                    continue
                cells = []
                for system in SYSTEMS:
                    delta = cases[mode, system]["changes_from_reference"][arm][phase]
                    cells.extend((fmt(delta["delta_ms"] / 1000, 4, True),
                                  fmt(delta["time_reduction_pct"], 2, True) + "%",
                                  fmt(delta["reference_over_current"])))
                gain = comparison["phases"][phase]["flock_over_mikros_relative_gain_pct"][arm]
                lines.append(f"| {label} | {arm} | {' | '.join(cells)} | {fmt(gain, 2, True)}% |")
        lines += ["", "| System | Arm | Samples | Proof bytes | Logical W/P | Ring W/P | Verify median [range] ms | RSS median [range] GiB |",
                  "|---|---|---:|---:|---:|---:|---:|---:|"]
        for system in SYSTEMS:
            for arm in report["arms"]:
                a = cases[mode, system]["arms"][arm]
                counts = a["claim_counts"]
                values = [f"{a[key]['median']:.3f} [{a[key]['min']:.3f}, {a[key]['max']:.3f}]" for key in ("verify_ms", "rss_gib")]
                lines.append(f"| {system} | {arm} | {a['sample_count']} | {a['proof_bytes']} | "
                             f"{counts['witness_claims']}/{counts['public_claims']} | "
                             f"{counts['witness_ring_claims']}/{counts['public_ring_claims']} | {' | '.join(values)} |")
        lines.append("")
        for system in SYSTEMS:
            for arm in report["arms"]:
                values = [s["prover_ms"] for s in cases[mode, system]["primary_samples_ms"][arm]]
                lines.append(f"{system} {arm} raw online milliseconds, one per process: {values}.")
        lines.append("")
    lines += ["comparison.json retains all raw phase milliseconds, proof/PIOP/claim/commitment hashes, parameters, resource records, previous-reference changes and SHA-256 digests of evidence files.", ""]
    return "\n".join(lines)


def self_test():
    """Synthetic only; includes internally consistent but incompatible arms."""
    def h(value):
        return hashlib.sha256(value.encode()).hexdigest()

    def save(folder, bundle):
        for name, value in bundle.items():
            (folder / f"{name}.json").write_text(json.dumps(value), encoding="utf-8")
        for case in bundle["summary"]["cases"]:
            (folder / case["raw_stdout"]).write_text(json.dumps(case["metadata"]) + "\n" +
                json.dumps(case["samples"][0]) + "\n", encoding="utf-8")

    def fixture(folder, arm, run):
        folder.mkdir()
        previous = arm == "previous_reference"
        extras = {} if previous else ARM_ENV[arm]
        config = {
            "levels": [26], "repetitions": 1, "seed": 97, "threads": 64,
            "cpu_affinity": "0-47,48-55,72-79", "rustflags": "-C target-cpu=native",
            "toolchain": "+nightly-2026-03-21", "allocator_policy": "system",
            "diagnostic_trace": False, "disabled_optimizations": [], "packed_direct": True,
            "piop_fingerprints": True, "direct_wp_simd": False, "gate_selector_simd": False,
            "measured_environment": {ROUTE_ENV: "1", **extras},
            "source_manifest_sha256": h("v9" if previous else "v10"),
        }
        if not previous:
            config.update({key: env in extras for key, env in CONFIG_ENV.items()})
        receipt = {key: config[key] for key in ("source_manifest_sha256", "toolchain", "rustflags")}
        receipt["binary_sha256"] = h("binary-v9" if previous else "binary-v10")
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
                total = (1000 if system == "mikros" else 1500) * (1 + .01 * run)
                if FUSION_ENV in extras and system == "mikros":
                    total *= .8
                if WP_ENV in extras:
                    total *= .9
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

    with tempfile.TemporaryDirectory(prefix="synthetic-fusion-evidence-") as tmp:
        paths = {arm: [Path(tmp) / f"{arm}-{run}" for run in range(3)]
                 for arm in (*ARM_ENV, "previous_reference")}
        for arm, folders in paths.items():
            for run, folder in enumerate(folders):
                fixture(folder, arm, run)

        def run(**overrides):
            selected = {**paths, **overrides}
            return compare(*(selected[arm] for arm in paths))

        report = run()
        first = report["cases"][0]
        require(first["arms"]["reference"]["times_ms"]["prover_ms"] == {"median": 1010, "min": 1000, "max": 1020}, "median/range")
        require(math.isclose(first["changes_from_reference"]["fusion"]["prover_ms"]["time_reduction_pct"], 20), "time reduction")
        require(math.isclose(report["comparisons"][0]["phases"]["prover_ms"]["flock_over_mikros_relative_gain_pct"]["fusion"], 25), "relative gain")
        require(first["primary_samples_ms"]["fusion"][0]["prover_ms"] == 800, "raw milliseconds")
        require(first["changes_from_reference"]["fusion"]["release_ms"]["reference_over_current"] is None, "zero denominator")
        require("previous_reference" not in run(previous_reference=[])["arms"], "optional previous")
        require("n/a" in markdown(report) and "raw online milliseconds" in markdown(report), "rendering")
        json.dumps(report, allow_nan=False)
        require(math.isclose(first["changes_from_reference"]["both"]["prover_ms"]["time_reduction_pct"], 28), "combined time reduction")
        require(math.isclose(report["comparisons"][0]["phases"]["prover_ms"]["flock_over_mikros_relative_gain_pct"]["wp_uninit"], 0), "shared WP benefit need not change relative ratio")
        checks = 10

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

        reject(lambda: run(fusion=paths["fusion"][:2]), "at least three")
        reject(lambda: run(fusion=paths["reference"]), "Duplicate evidence directory")
        for key in FLAGS:
            mutate("reference", lambda b, k=key: b["config"].update({k: True}), f"config {key}")
        mutate("reference", lambda b: b["config"].pop("gate_fusion"), "config gate_fusion")
        mutate("reference", lambda b: b["config"].pop("wp_uninit"), "config wp_uninit")
        mutate("fusion", lambda b: b["config"].update(gate_fusion=1), "config gate_fusion")
        mutate("fusion", lambda b: b["config"]["measured_environment"].update(FLOCK_DIRECT_WP_SIMD="1"), "unexpected measured environment")
        mutate("fusion", lambda b: b["config"].update(diagnostic_trace=True), "nontrace")
        mutate("fusion", lambda b: b["config"].update(packed_direct=False), "route/fingerprint")
        mutate("fusion", lambda b: b["status"].update(state="running"), "incomplete or invalid")
        mutate("fusion", lambda b: b["build-receipt"].update(binary_sha256=h("different")), "reference/fusion binary_sha256")
        for arm in ("fusion", "wp_uninit", "both", "previous_reference"):
            for field in ("proof_blake3", "proof_bytes", *PIOP_FIELDS):
                value = 101 if field == "proof_bytes" else h("wrong " + field)
                mutate(arm, lambda b, f=field, v=value: b["summary"]["cases"][0]["samples"][0].update({f: v}),
                       f"reference/{arm} {field} mismatch")
            for field in CLAIM_COUNTS:
                mutate(arm, lambda b, f=field: b["summary"]["cases"][0]["samples"][0]["times"].update({f: 99}),
                       f"reference/{arm} {field} mismatch")
        mutate("fusion", lambda b: b["summary"]["cases"][0]["samples"][0].update(proof_blake3=h("unstable")),
               "not stable within arm", all_runs=False)

        def source_change(b):
            b["config"]["source_manifest_sha256"] = h("different")
            b["build-receipt"]["source_manifest_sha256"] = h("different")

        mutate("fusion", source_change, "reference/fusion source_manifest_sha256")

        def seed_change(b):
            b["config"]["seed"] = 98
            for case in b["summary"]["cases"]:
                case["metadata"]["seed"] = case["samples"][0]["seed"] = 98

        mutate("previous_reference", seed_change, "reference/previous_reference seed mismatch")
        mutate("previous_reference", lambda b: b["config"].update(gate_fusion=True), "config gate_fusion")
        mutate("previous_reference", lambda b: b["config"].update(wp_uninit=True), "config wp_uninit")
        for field in COMMON_FIELDS:
            def shared_change(b, field=field):
                for case in b["summary"]["cases"]:
                    value = h("changed " + field) if "blake3" in field else {**case[field], "m": 123}
                    case[field] = case["samples"][0][field] = value

            mutate("fusion", shared_change, f"reference/fusion {field} mismatch")
        print(f"Synthetic self-test passed ({checks} checks); no benchmark executed or measured report produced.")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for arm in ARM_ENV:
        parser.add_argument("--" + arm.replace("_", "-"), nargs="+", type=Path)
    parser.add_argument("--previous-reference", nargs="+", type=Path, default=[])
    parser.add_argument("--output", type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    inputs = [getattr(args, arm) for arm in ARM_ENV]
    if args.self_test:
        require(not (any(inputs) or args.previous_reference or args.output),
                "--self-test cannot consume measured evidence or create reports")
        self_test()
        return
    if not all(inputs) or not args.output:
        parser.error("--reference, --fusion, --wp-uninit, --both and --output are required")
    report = compare(*inputs, args.previous_reference)
    rendered = markdown(report)
    args.output.mkdir(parents=True, exist_ok=True)
    (args.output / "comparison.json").write_text(json.dumps(report, indent=2, allow_nan=False) + "\n", encoding="utf-8")
    (args.output / "comparison.md").write_text(rendered, encoding="utf-8")
    print(rendered)


if __name__ == "__main__":
    main()
