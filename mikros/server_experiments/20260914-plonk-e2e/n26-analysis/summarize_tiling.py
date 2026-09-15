"""Audit selected N26 Mikros tiling arms without running benchmarks.

Use repeated --arm NAME DIR... arguments with reference and at least one of
virtual, weights, paired, scratch, all. Every arm requires at least three
fresh-process, nontrace directories containing all four system/mode cases.
PackedDirect and WP first-write are ON in every arm. Previous v10 wp_uninit
evidence may differ only in source/binary and omit the four new OFF flags.
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


WP_ENV = "FLOCK_WP_UNINIT"
CONFIG_ENV = {
    "gate_virtual_product": "MIKROS_GATE_VIRTUAL_PRODUCT",
    "wh_tiled_weights": "MIKROS_WH_TILED_WEIGHTS",
    "copy_paired_terminals": "E2E_COPY_PAIRED_TERMINALS",
    "gate_wh_scratch": "MIKROS_GATE_WH_SCRATCH",
}
VIRTUAL, WEIGHTS, PAIRED, SCRATCH = CONFIG_ENV.values()
ARM_ENV = {
    "reference": {},
    "virtual": {VIRTUAL: "1"},
    "weights": {WEIGHTS: "1"},
    "paired": {PAIRED: "1"},
    "scratch": {VIRTUAL: "1", SCRATCH: "1"},
    "all": {env: "1" for env in CONFIG_ENV.values()},
}
OLD_FLAGS = {"direct_wp_simd": False, "gate_selector_simd": False,
             "gate_fusion": False, "wp_uninit": True}


def collect_arm(audit, paths, arm):
    previous = arm == "previous_reference"
    extras = {WP_ENV: "1", **({} if previous else ARM_ENV[arm])}
    data = audit.collect(paths, "on", extras)
    flags = {**OLD_FLAGS, **{key: env in extras for key, env in CONFIG_ENV.items()}}
    for folder in data["directories"]:
        config = audit.json(Path(folder) / "config.json")
        for key, expected in flags.items():
            actual = config.get(key, False) if previous and key in CONFIG_ENV else config.get(key)
            require(actual is expected, f"{arm} {folder}: config {key} disagrees with arm environment")
    data["identity"].update(flags)
    for case, samples in data["cases"].items():
        require(len(samples) >= 3, f"{arm} {case}: require at least three fresh-process samples")
    return data


def compare(arm_paths, previous_reference_paths=()):
    require("reference" in arm_paths and len(arm_paths) >= 2,
            "require reference and at least one optimization arm")
    require(set(arm_paths) <= set(ARM_ENV), "unknown optimization arm")
    audit = Audit()
    # Stable order independent of CLI order. Unmeasured arms never appear in
    # ratios or deltas and are not inferred from another combination.
    arms = {arm: collect_arm(audit, arm_paths[arm], arm)
            for arm in ARM_ENV if arm in arm_paths}
    current_arms = tuple(arms)
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
        "experiment": "selected Mikros tiling, paired terminals and scratch ablations",
        "scale": "single: N26 gates; multiple: 128 circuits each with N26 gates (total 2^33 constraints)",
        "scope": "Online commit/recommit, conversion, PIOPs, PCS, release and serialization included; offline excluded; verify separate; RSS includes the entire process",
        "statistic": "Median [min, max] of at least three fresh processes per arm/case; phase medians need not sum to total median",
        "interpretation": "Descriptive samples, not a significance claim. Diagnostic trace runs are rejected. FLOCK is a no-op control for all new flags. Mikros single is a no-op control for virtual, paired and scratch; weights/all may also affect its weighted Hadamard calls. No-op control changes are variation; changes in unmodified Mikros phases may include indirect memory effects and must not automatically be credited to the target kernels.",
        "current_arms": current_arms,
        "arms": {arm: {key: data[key] for key in ("identity", "directories")} for arm, data in arms.items()},
        "validation": {
            "all_comparisons_valid": True, "packed_direct_and_wp_uninit_on": True,
            "previous_arithmetic_and_gate_fusion_flags_off": True,
            "current_arms_same_source_binary_seed_parameters": True,
            "config_booleans_match_exact_environment": True,
            "source_binary_verified_before_and_after": True,
            "full_proof_hash_bytes_piop_claims_commitments_pcs_equal": True,
            "logical_and_ring_counts_equal": True, "minimum_samples_per_arm_case": 3,
            "nontrace_only": True,
            "previous_reference_only_source_binary_difference_allowed": bool(previous_reference_paths),
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
                                        groups[arm]["times_ms"][phase]["median"]) for phase in PHASES}
                    for arm in current_arms if arm != "reference"},
                "primary_samples_ms": {
                    arm: [{"directory": sample["directory"], **{key: sample["times"][key] for key in RAW_TIMES}}
                          for sample in summary["samples"]] for arm, summary in groups.items()},
            }
            if "previous_reference" in groups:
                case["changes_from_previous_reference"] = {
                    arm: {phase: change(groups["previous_reference"]["times_ms"][phase]["median"],
                                        groups[arm]["times_ms"][phase]["median"]) for phase in PHASES}
                    for arm in current_arms}
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
                    for arm in current_arms if arm != "reference"},
            }
        report["comparisons"].append({"mode": mode, "phases": phases})
    report["evidence_sha256"] = audit.evidence
    return report


def markdown(report):
    lines = ["# N26 Mikros tiling and scratch ablation", "", report["scale"], "", report["scope"], "",
             report["statistic"], "", report["interpretation"], "",
             "All arms use PackedDirect + WP first-write. Earlier WP SIMD, selector SIMD and gate-fusion flags are OFF.",
             "All current arms share source/binary, input commitments and native PCS. Same system/mode complete proofs, PIOPs, terminal claims, PCS parameters and counts agree across all arms.", "",
             "| Arm | Additional opt-ins |", "|---|---|"]
    for arm in report["current_arms"]:
        enabled = ", ".join(f"{key}={value}" for key, value in ARM_ENV[arm].items()) or "none"
        lines.append(f"| {arm} | {enabled} |")
    if "previous_reference" in report["arms"]:
        lines += ["", "Previous reference is separate evidence with WP first-write ON; only source/binary may differ. Missing new flags mean OFF, while all previous flags must be explicit and correct."]
    cases = {(case["mode"], case["system"]): case for case in report["cases"]}
    for comparison in report["comparisons"]:
        mode = comparison["mode"]
        lines += ["", f"## {mode}", "", "F/M > 1 means Mikros is faster. Times are median [min, max] seconds.", "",
                  "| Stage | Arm | Mikros (s) | FLOCK (s) | M/F | F/M |", "|---|---|---:|---:|---:|---:|"]
        for phase, label in PHASES.items():
            for arm in report["arms"]:
                times = [cases[mode, system]["arms"][arm]["times_ms"][phase] for system in SYSTEMS]
                cells = [f"{t['median']/1000:.4f} [{t['min']/1000:.4f}, {t['max']/1000:.4f}]" for t in times]
                ratio = comparison["phases"][phase]["ratios"][arm]
                lines.append(f"| {label} | {arm} | {' | '.join(cells)} | {fmt(ratio['mikros_over_flock'])} | {fmt(ratio['flock_over_mikros'])} |")
        lines += ["", "Negative delta means less time; positive relative F/M gain improves Mikros's relative advantage.", "",
                  "| Stage | Arm | Mikros delta (s) | Mikros reduction | FLOCK delta (s) | FLOCK reduction | Relative F/M gain |",
                  "|---|---|---:|---:|---:|---:|---:|"]
        for phase, label in PHASES.items():
            for arm in report["current_arms"]:
                if arm == "reference":
                    continue
                cells = []
                for system in SYSTEMS:
                    delta = cases[mode, system]["changes_from_reference"][arm][phase]
                    cells.extend((fmt(delta["delta_ms"] / 1000, 4, True), fmt(delta["time_reduction_pct"], 2, True) + "%"))
                gain = comparison["phases"][phase]["flock_over_mikros_relative_gain_pct"][arm]
                lines.append(f"| {label} | {arm} | {' | '.join(cells)} | {fmt(gain, 2, True)}% |")
        lines += ["", "| System | Arm | Samples | Proof bytes | Logical W/P | Ring W/P | Verify ms [range] | Whole-process RSS GiB [range] |",
                  "|---|---|---:|---:|---:|---:|---:|---:|"]
        for system in SYSTEMS:
            for arm in report["arms"]:
                summary = cases[mode, system]["arms"][arm]
                counts = summary["claim_counts"]
                values = [f"{summary[key]['median']:.3f} [{summary[key]['min']:.3f}, {summary[key]['max']:.3f}]" for key in ("verify_ms", "rss_gib")]
                lines.append(f"| {system} | {arm} | {summary['sample_count']} | {summary['proof_bytes']} | "
                             f"{counts['witness_claims']}/{counts['public_claims']} | "
                             f"{counts['witness_ring_claims']}/{counts['public_ring_claims']} | {' | '.join(values)} |")
        lines += ["", "Raw primary phase timings, milliseconds (one row per fresh process):", "",
                  "| System | Arm | Directory | " + " | ".join(RAW_TIMES) + " |",
                  "|---|---|---|" + "---:|" * len(RAW_TIMES)]
        for system in SYSTEMS:
            for arm in report["arms"]:
                for sample in cases[mode, system]["primary_samples_ms"][arm]:
                    values = " | ".join(f"{sample[key]:.6f}" for key in RAW_TIMES)
                    lines.append(f"| {system} | {arm} | {sample['directory']} | {values} |")
    lines += ["", "comparison.json also retains all raw resource records, complete hashes, parameters, prior-reference differences and SHA-256 digests of read evidence files.", ""]
    return "\n".join(lines)


def self_test():
    """Synthetic fixtures only; no benchmark execution or measured output."""
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
        extras = {WP_ENV: "1", **({} if previous else ARM_ENV[arm])}
        config = {
            "levels": [26], "repetitions": 1, "seed": 97, "threads": 64,
            "cpu_affinity": "0-47,48-55,72-79", "rustflags": "-C target-cpu=native",
            "toolchain": "+nightly-2026-03-21", "allocator_policy": "system",
            "diagnostic_trace": False, "disabled_optimizations": [], "packed_direct": True,
            "piop_fingerprints": True, **OLD_FLAGS,
            "measured_environment": {ROUTE_ENV: "1", **extras},
            "source_manifest_sha256": h("v10" if previous else "v11"),
        }
        if not previous:
            config.update({key: env in extras for key, env in CONFIG_ENV.items()})
        receipt = {key: config[key] for key in ("source_manifest_sha256", "toolchain", "rustflags")}
        receipt["binary_sha256"] = h("binary-v10" if previous else "binary-v11")
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
                if arm != "reference" and not previous and mode == "multiple" and system == "mikros":
                    total *= .8
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

    with tempfile.TemporaryDirectory(prefix="synthetic-tiling-evidence-") as tmp:
        paths = {arm: [Path(tmp) / f"{arm}-{run}" for run in range(3)]
                 for arm in (*ARM_ENV, "previous_reference")}
        for arm, folders in paths.items():
            for run, folder in enumerate(folders):
                fixture(folder, arm, run)

        def run(**overrides):
            selected = {**paths, **overrides}
            previous = selected.pop("previous_reference")
            return compare(selected, previous)

        report = run()
        first = next(c for c in report["cases"] if c["mode"] == "multiple" and c["system"] == "mikros")
        require(first["arms"]["reference"]["times_ms"]["prover_ms"] == {"median": 1010, "min": 1000, "max": 1020}, "median/range")
        require(math.isclose(first["changes_from_reference"]["virtual"]["prover_ms"]["time_reduction_pct"], 20), "time reduction")
        require(math.isclose(report["comparisons"][1]["phases"]["prover_ms"]["flock_over_mikros_relative_gain_pct"]["paired"], 25), "relative gain")
        require(first["primary_samples_ms"]["virtual"][0]["prover_ms"] == 800, "raw milliseconds")
        require(first["changes_from_reference"]["virtual"]["release_ms"]["reference_over_current"] is None, "zero denominator")
        require("previous_reference" not in run(previous_reference=[])["arms"], "optional previous")
        subset = compare({"paired": paths["paired"], "reference": paths["reference"]})
        require(subset["current_arms"] == ("reference", "paired") and "virtual" not in subset["arms"], "measured subset only")
        require("n/a" in markdown(subset) and "Raw primary phase timings" in markdown(subset), "rendering")
        json.dumps(report, allow_nan=False)
        checks = 9

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

        reject(lambda: compare({"reference": paths["reference"]}), "at least one")
        reject(lambda: compare({"virtual": paths["virtual"], "paired": paths["paired"]}), "require reference")
        reject(lambda: compare({"reference": paths["reference"], "unknown": paths["paired"]}), "unknown")
        reject(lambda: run(virtual=paths["virtual"][:2]), "at least three")
        reject(lambda: run(virtual=paths["reference"]), "Duplicate evidence directory")
        for key in (*OLD_FLAGS, *CONFIG_ENV):
            mutate("reference", lambda b, k=key: b["config"].update({k: not b["config"][k]}), f"config {key}")
        for key in CONFIG_ENV:
            mutate("reference", lambda b, k=key: b["config"].pop(k), f"config {key}")
            mutate("previous_reference", lambda b, k=key: b["config"].update({k: True}), f"config {key}")
        mutate("virtual", lambda b: b["config"].update(gate_virtual_product=1), "config gate_virtual_product")
        mutate("virtual", lambda b: b["config"]["measured_environment"].update(FLOCK_DIRECT_WP_SIMD="1"), "unexpected measured environment")
        mutate("previous_reference", lambda b: b["config"]["measured_environment"].pop(WP_ENV), "unexpected measured environment")
        mutate("previous_reference", lambda b: b["config"].pop("wp_uninit"), "config wp_uninit")
        mutate("virtual", lambda b: b["config"].update(diagnostic_trace=True), "nontrace")
        mutate("virtual", lambda b: b["config"].update(packed_direct=False), "route/fingerprint")
        mutate("virtual", lambda b: b["status"].update(state="running"), "incomplete or invalid")
        mutate("virtual", lambda b: b["build-receipt"].update(binary_sha256=h("different")), "reference/virtual binary_sha256")
        for arm in ("virtual", "weights", "paired", "scratch", "all", "previous_reference"):
            for field in ("proof_blake3", "proof_bytes", *PIOP_FIELDS):
                value = 101 if field == "proof_bytes" else h("wrong " + field)
                mutate(arm, lambda b, f=field, v=value: b["summary"]["cases"][0]["samples"][0].update({f: v}),
                       f"reference/{arm} {field} mismatch")
            for field in CLAIM_COUNTS:
                mutate(arm, lambda b, f=field: b["summary"]["cases"][0]["samples"][0]["times"].update({f: 99}),
                       f"reference/{arm} {field} mismatch")
        mutate("virtual", lambda b: b["summary"]["cases"][0]["samples"][0].update(proof_blake3=h("unstable")), "not stable within arm", all_runs=False)

        def source_change(bundle):
            bundle["config"]["source_manifest_sha256"] = h("different")
            bundle["build-receipt"]["source_manifest_sha256"] = h("different")

        mutate("virtual", source_change, "reference/virtual source_manifest_sha256")

        def seed_change(bundle):
            bundle["config"]["seed"] = 98
            for case in bundle["summary"]["cases"]:
                case["metadata"]["seed"] = case["samples"][0]["seed"] = 98

        mutate("previous_reference", seed_change, "reference/previous_reference seed mismatch")
        for field in COMMON_FIELDS:
            def shared_change(bundle, field=field):
                for case in bundle["summary"]["cases"]:
                    value = h("changed " + field) if "blake3" in field else {**case[field], "m": 123}
                    case[field] = case["samples"][0][field] = value

            mutate("paired", shared_change, f"reference/paired {field} mismatch")
        print(f"Synthetic self-test passed ({checks} checks); no benchmark executed or measured report produced.")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--arm", action="append", nargs="+", default=[], metavar="NAME_OR_DIR")
    parser.add_argument("--previous-reference", nargs="+", type=Path, default=[])
    parser.add_argument("--output", type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        require(not (args.arm or args.previous_reference or args.output),
                "--self-test cannot consume measured evidence or create reports")
        self_test()
        return
    if not args.arm or not args.output:
        parser.error("repeated --arm NAME DIR... and --output are required")
    selected = {}
    for spec in args.arm:
        require(len(spec) >= 2, "--arm requires NAME and one or more directories")
        name, *directories = spec
        require(name not in selected, f"duplicate arm specification: {name}")
        selected[name] = [Path(directory) for directory in directories]
    report = compare(selected, args.previous_reference)
    rendered = markdown(report)
    args.output.mkdir(parents=True, exist_ok=True)
    (args.output / "comparison.json").write_text(json.dumps(report, indent=2, allow_nan=False) + "\n", encoding="utf-8")
    (args.output / "comparison.md").write_text(rendered, encoding="utf-8")
    print(rendered)


if __name__ == "__main__":
    main()
