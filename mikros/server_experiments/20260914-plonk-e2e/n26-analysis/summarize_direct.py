"""Audit and summarize complete N26 native PackedDirect OFF/ON harness runs.

This is a local evidence reader, never a benchmark runner. Full proofs must be
stable within each arm, but may differ across routes. PIOPs, terminal claims,
commitments and PCS parameters must agree across OFF/ON. Optional --previous-on
compares an earlier ON implementation: its source/binary may differ, but its
complete proofs and protocol inputs must match current ON. --self-test uses
only synthetic temporary fixtures and reports no measured performance.
"""

import argparse
import copy
import hashlib
import json
import math
from pathlib import Path
import re
import statistics
import tempfile


MODES = ("single", "multiple")
SYSTEMS = ("mikros", "flock")
FEATURES = ("gfni", "avx512f", "avx512bw", "vpclmulqdq")
ROUTE_ENV = "FLOCK_PCS_PACKED_DIRECT"
COMMON_FIELDS = (
    "witness_commitment_blake3", "public_commitment_blake3",
    "witness_pcs_params", "public_pcs_params",
)
PIOP_FIELDS = ("piop_blake3", "terminal_claims_blake3")
CLAIM_COUNTS = ("witness_claims", "public_claims", "witness_ring_claims", "public_ring_claims")
PHASES = {
    "gates_ms": "Gate PIOP", "copy_ms": "Copy PIOP",
    "witness_pcs_open_ms": "Witness PCS opening",
    "public_pcs_open_ms": "Public PCS opening",
    "commit_and_recommit_ms": "Commit and recommits",
    "release_ms": "Release", "serialization_ms": "Serialization",
    "prover_ms": "Online total",
}
RAW_TIMES = (
    "commit_ms", "recommit_ms", "public_recommit_ms", "gates_ms", "copy_ms",
    "release_ms", "witness_pcs_open_ms", "public_pcs_open_ms",
    "serialization_ms", "prover_ms",
)


def require(ok, message):
    if not ok:
        raise ValueError(message)


def digest(value, where):
    require(isinstance(value, str) and re.fullmatch(r"[0-9a-f]{64}", value),
            f"{where}: missing or malformed digest")
    return value


def number(value, where, positive=False):
    require(isinstance(value, (int, float)) and not isinstance(value, bool)
            and math.isfinite(value) and (value > 0 if positive else value >= 0),
            f"{where}: expected finite {'positive' if positive else 'nonnegative'} number")
    return value


def stats(values):
    values = list(values)
    return {"median": statistics.median(values), "min": min(values), "max": max(values)}


class Audit:
    def __init__(self):
        self.evidence = {}
        self.seen = set()

    def read(self, path):
        data = path.read_bytes()
        self.evidence[str(path)] = hashlib.sha256(data).hexdigest()
        return data.decode("utf-8-sig")

    def json(self, path):
        return json.loads(self.read(path))

    def collect(self, paths, arm, extra_environment=None):
        # Engineering ablations retain the ON route and add only explicit,
        # independently audited opt-ins. Default callers remain strict.
        extra_environment = {} if extra_environment is None else extra_environment
        require(arm == "on" or not extra_environment,
                "Engineering opt-ins require the packed-direct ON route")
        cases = {(m, s): [] for m in MODES for s in SYSTEMS}
        identity = None
        folders = []
        for requested in paths:
            folder = (requested.parent if requested.name == "summary.json" else requested).resolve()
            require(folder not in self.seen, f"Duplicate evidence directory: {folder}")
            self.seen.add(folder)
            folders.append(str(folder))
            summary = self.json(folder / "summary.json")
            status = self.json(folder / "status.json")
            config = self.json(folder / "config.json")
            receipt = self.json(folder / "build-receipt.json")
            prefix = f"{arm} {folder}"
            require(summary.get("all_comparisons_valid") is True
                    and summary.get("all_requested_cases_complete") is True
                    and summary.get("run_state") == status.get("state") == "complete",
                    f"{prefix}: incomplete or invalid comparison")
            require(summary.get("config") == config, f"{prefix}: summary/config disagreement")
            require(summary.get("source_hashes_verified_before_and_after") is True
                    and status.get("source_hashes_verified_before_and_after") is True
                    and status.get("binary_hashes_verified_before_and_after") is True,
                    f"{prefix}: source/binary hashes were not verified before and after")
            require(config.get("threads") == 64 and config.get("levels") == [26]
                    and config.get("repetitions") == 1,
                    f"{prefix}: require native64 N26 and one fresh-process sample per case")
            require(config.get("allocator_policy") == "system"
                    and config.get("diagnostic_trace") is False,
                    f"{prefix}: require system allocator and nontrace measurements")
            require(config.get("cpu_affinity") == "0-47,48-55,72-79"
                    and config.get("rustflags") == "-C target-cpu=native",
                    f"{prefix}: CPU affinity/native compilation mismatch")
            environment = config.get("measured_environment", {})
            disabled = config.get("disabled_optimizations", [])
            require(disabled == [], f"{prefix}: unexpected disabled optimizations {disabled}")
            if arm != "baseline":
                enabled = arm in ("on", "previous_on")
                require(config.get("packed_direct") is enabled
                        and config.get("piop_fingerprints") is True,
                        f"{prefix}: route/fingerprint configuration mismatch")
                expected_environment = {**({ROUTE_ENV: "1"} if enabled else {}), **extra_environment}
                require(environment == expected_environment,
                        f"{prefix}: unexpected measured environment {environment}")
            else:
                require(config.get("packed_direct", False) is False and environment == {},
                        f"{prefix}: v6 baseline must use original route and default controls")
            source = digest(config.get("source_manifest_sha256"), f"{prefix} source")
            require(receipt.get("source_manifest_sha256") == source
                    and receipt.get("rustflags") == config["rustflags"]
                    and receipt.get("toolchain") == config.get("toolchain"),
                    f"{prefix}: build receipt/config disagreement")
            current = {
                "source_manifest_sha256": source,
                "binary_sha256": digest(receipt.get("binary_sha256"), f"{prefix} binary"),
                "seed": config["seed"], "toolchain": config["toolchain"],
                "cpu_affinity": config["cpu_affinity"], "rustflags": config["rustflags"],
                "allocator_policy": "system", "disabled_optimizations": disabled,
                "packed_direct": arm in ("on", "previous_on"), "measured_environment": environment,
            }
            require(identity is None or identity == current,
                    f"{prefix}: mixed source, binary, seed or configuration within arm")
            identity = current
            expected_ids = {f"n26-{m}-{s}" for m in MODES for s in SYSTEMS}
            completed = status.get("cases", [])
            require(len(completed) == 4 and {x["id"] for x in completed} == expected_ids
                    and all(x.get("state") == "complete" and x.get("exit_code") == 0
                            for x in completed), f"{prefix}: incomplete status cases")
            comparison_rows = summary.get("comparisons", [])
            require(len(comparison_rows) == 2
                    and {x.get("mode") for x in comparison_rows} == set(MODES)
                    and all(x.get("state") == "complete" and x.get("log_gates") == 26
                            and x.get("same_pcs_params") is True
                            and x.get("same_commitment") is True for x in comparison_rows),
                    f"{prefix}: invalid per-mode comparisons")
            grouped = summary.get("cases", [])
            require(len(grouped) == 4 and {x.get("id") for x in grouped} == expected_ids,
                    f"{prefix}: missing/duplicate summary cases")
            current_samples = {}
            for case in grouped:
                mode, system = case["mode"], case["system"]
                case_id = f"n26-{mode}-{system}"
                require((mode, system) in cases and case["id"] == case_id
                        and case.get("state") == "complete",
                        f"{prefix}: malformed case identity")
                raw_name = f"{case_id}.stdout.jsonl"
                require(case.get("raw_stdout") == raw_name, f"{prefix}: raw path mismatch")
                rows = [json.loads(line) for line in self.read(folder / raw_name).splitlines() if line]
                metadata = [x for x in rows if x.get("type") == "metadata"]
                raw_samples = [x for x in rows if x.get("type") == "sample"]
                require(len(rows) == 2 and len(metadata) == len(raw_samples) == 1
                        and case.get("metadata") == metadata[0]
                        and case.get("samples") == raw_samples,
                        f"{prefix} {case_id}: summary and raw JSONL disagree")
                meta, sample = metadata[0], raw_samples[0]
                instances = 1 if mode == "single" else 128
                for record in (meta, sample):
                    require(record.get("mode") == mode and record.get("system") == system
                            and record.get("seed") == config["seed"]
                            and record.get("log_gates") == 26
                            and record.get("instances") == instances and record.get("threads") == 64,
                            f"{prefix} {case_id}: seed/scale/system/thread mismatch")
                require(meta.get("all_core_threads") == 64 and meta.get("repetitions") == 1
                        and all(meta.get("target_features", {}).get(f) is True for f in FEATURES),
                        f"{prefix} {case_id}: native ISA or all-core pool mismatch")
                require(sample.get("verified") is True and sample.get("transcript_match") is True
                        and sample.get("pcs_config_match") is True
                        and sample.get("native_pcs_identity_transport") is True
                        and sample.get("repetition") == 0
                        and sample.get("gates_per_instance") == 2**26
                        and sample.get("total_gates") == instances * 2**26,
                        f"{prefix} {case_id}: invalid proof, scale or transport")
                for key in ("proof_blake3", *COMMON_FIELDS[:2], *(PIOP_FIELDS if arm != "baseline" else ())):
                    digest(sample.get(key), f"{prefix} {case_id} {key}")
                number(sample.get("proof_bytes"), f"{prefix} {case_id} proof bytes", True)
                number(sample.get("verify_ms"), f"{prefix} {case_id} verify time")
                for key in RAW_TIMES:
                    number(sample["times"].get(key), f"{prefix} {case_id} {key}", key == "prover_ms")
                    require(case.get("timings_median_ms", {}).get(key) == sample["times"][key],
                            f"{prefix} {case_id}: summary timing disagrees for {key}")
                for key in CLAIM_COUNTS:
                    count = sample["times"].get(key)
                    require(isinstance(count, int) and not isinstance(count, bool) and count >= 0,
                            f"{prefix} {case_id}: invalid {key}")
                for key in COMMON_FIELDS:
                    require(case.get(key) == sample.get(key), f"{prefix}: summary {key} mismatch")
                for key in COMMON_FIELDS[2:]:
                    params = sample[key]
                    require(params.get("profile") == "fast" and params.get("log_inv_rate") == 1
                            and params.get("merkle_hash") == "blake3" and params.get("num_lanes") is None,
                            f"{prefix} {case_id}: unexpected native PCS parameters")
                resources = case.get("resources", {})
                require(resources.get("available") is True and resources.get("time_exit_status") == 0,
                        f"{prefix} {case_id}: missing or failed resource record")
                rss = number(resources.get("max_rss_kib"), f"{prefix} {case_id} RSS", True)
                require(resources.get("max_rss_gib") == rss / 2**20,
                        f"{prefix} {case_id}: inconsistent RSS units")
                saved = copy.deepcopy(sample)
                saved["directory"] = str(folder)
                saved["resources"] = resources
                saved["times"]["commit_and_recommit_ms"] = sum(
                    saved["times"][k] for k in ("commit_ms", "recommit_ms", "public_recommit_ms"))
                cases[mode, system].append(saved)
                current_samples[mode, system] = sample
            for mode in MODES:
                for key in COMMON_FIELDS:
                    require(current_samples[mode, "mikros"][key] == current_samples[mode, "flock"][key],
                            f"{prefix} {mode}: Mikros/FLOCK {key} mismatch")
        require(identity is not None, f"{arm}: no evidence directories")
        for key, samples in cases.items():
            for field in ("proof_blake3", "proof_bytes", *COMMON_FIELDS,
                          *(PIOP_FIELDS if arm != "baseline" else ())):
                require(all(x[field] == samples[0][field] for x in samples),
                        f"{arm} {key}: {field} is not stable within arm")
            for field in CLAIM_COUNTS:
                require(all(x["times"][field] == samples[0]["times"][field] for x in samples),
                        f"{arm} {key}: {field} is not stable within arm")
        return {"identity": identity, "directories": folders, "cases": cases}


def arm_summary(samples):
    all_times = (*RAW_TIMES, "commit_and_recommit_ms")
    return {
        "sample_count": len(samples),
        "times_ms": {key: stats(x["times"][key] for x in samples) for key in all_times},
        "verify_ms": stats(x["verify_ms"] for x in samples),
        "rss_gib": stats(x["resources"]["max_rss_gib"] for x in samples),
        "proof_bytes": samples[0]["proof_bytes"], "proof_blake3": samples[0]["proof_blake3"],
        "claim_counts": {key: samples[0]["times"][key] for key in CLAIM_COUNTS},
        "samples": samples,
    }


def compare(off_paths, on_paths, baseline_paths=(), previous_on_paths=()):
    audit = Audit()
    arms = {"off": audit.collect(off_paths, "off"), "on": audit.collect(on_paths, "on")}
    if baseline_paths:
        arms["baseline"] = audit.collect(baseline_paths, "baseline")
    if previous_on_paths:
        arms["previous_on"] = audit.collect(previous_on_paths, "previous_on")
    off_id, on_id = arms["off"]["identity"], arms["on"]["identity"]
    for field in off_id:
        if field not in ("packed_direct", "measured_environment"):
            require(off_id[field] == on_id[field], f"OFF/ON {field} mismatch")
    if "baseline" in arms:
        for field in ("seed", "toolchain", "cpu_affinity", "rustflags", "allocator_policy", "disabled_optimizations"):
            require(arms["baseline"]["identity"][field] == off_id[field],
                    f"baseline/OFF {field} mismatch")
    if "previous_on" in arms:
        for field in on_id:
            if field not in ("source_manifest_sha256", "binary_sha256"):
                require(arms["previous_on"]["identity"][field] == on_id[field],
                        f"previous ON/current ON {field} mismatch")
    report = {
        "schema_version": 1, "log_gates": 26, "threads": 64,
        "scale": "single: N26 gates; multiple: 128 independent circuits each with N26 gates",
        "scope": "Online commit/recommit, conversion, PIOPs, PCS and release included; offline excluded",
        "statistic": "Medians of independent fresh processes; phase medians need not sum to total median",
        "interpretation": "Descriptive samples and ranges, not a statistical significance claim",
        "arms": {name: {k: data[k] for k in ("identity", "directories")} for name, data in arms.items()},
        "validation": {
            "all_comparisons_valid": True, "same_off_on_source_and_binary": True,
            "harness_verified_source_and_binary_before_and_after": True,
            "same_off_on_piop_terminal_claims_commitments_and_pcs": True,
            "full_proofs_stable_within_each_arm": True,
            "off_full_proof_matches_v6": True if baseline_paths else None,
            "off_on_full_proof_equality_required": False,
        },
        "cases": [], "comparisons": [],
    }
    if previous_on_paths:
        report["validation"]["previous_on_full_proof_and_inputs_match_current_on"] = True
    for mode in MODES:
        mode_cases = {}
        for system in SYSTEMS:
            groups = {name: data["cases"][mode, system] for name, data in arms.items()}
            old, new = groups["off"][0], groups["on"][0]
            for field in (*PIOP_FIELDS, *COMMON_FIELDS):
                require(old[field] == new[field], f"{mode} {system}: OFF/ON {field} mismatch")
            for field in CLAIM_COUNTS[:2]:
                require(old["times"][field] == new["times"][field],
                        f"{mode} {system}: OFF/ON logical {field} mismatch")
            if "baseline" in groups:
                for field in ("proof_blake3", "proof_bytes", *COMMON_FIELDS):
                    require(groups["baseline"][0][field] == old[field],
                            f"{mode} {system}: v6/OFF {field} mismatch")
            if "previous_on" in groups:
                previous = groups["previous_on"][0]
                for field in ("proof_blake3", "proof_bytes", *PIOP_FIELDS, *COMMON_FIELDS):
                    require(previous[field] == new[field],
                            f"{mode} {system}: previous ON/current ON {field} mismatch")
                for field in CLAIM_COUNTS:
                    require(previous["times"][field] == new["times"][field],
                            f"{mode} {system}: previous ON/current ON {field} mismatch")
            item = {
                "mode": mode, "system": system,
                "invariants": {key: old[key] for key in (*PIOP_FIELDS, *COMMON_FIELDS)},
                "off_on_full_proof_equal": old["proof_blake3"] == new["proof_blake3"],
                **{name: arm_summary(samples) for name, samples in groups.items()},
            }
            item["improvements"] = {}
            for phase in PHASES:
                before, after = (item[a]["times_ms"][phase]["median"] for a in ("off", "on"))
                item["improvements"][phase] = {
                    "off_over_on": before / after if after else None,
                    "time_reduction_pct": 100 * (1 - after / before) if before else None,
                    "saved_ms": before - after,
                }
            if "previous_on" in groups:
                item["engineering_improvements"] = {}
                for phase in PHASES:
                    before, after = (item[a]["times_ms"][phase]["median"] for a in ("previous_on", "on"))
                    item["engineering_improvements"][phase] = {
                        "previous_on_over_on": before / after if after else None,
                        "time_reduction_pct": 100 * (1 - after / before) if before else None,
                        "saved_ms": before - after,
                    }
            mode_cases[system] = item
            report["cases"].append(item)
        phase_ratios = {}
        for phase in PHASES:
            ratios = {}
            for arm in arms:
                m, f = (mode_cases[s][arm]["times_ms"][phase]["median"] for s in SYSTEMS)
                ratios[arm] = f / m if m else None
            off, on = ratios["off"], ratios["on"]
            phase_ratios[phase] = {
                "flock_over_mikros": ratios,
                "relative_ratio_change_pct": 100 * (on / off - 1) if off and on is not None else None,
                "system_improvements": {s: mode_cases[s]["improvements"][phase] for s in SYSTEMS},
            }
            if "previous_on" in arms:
                previous = ratios["previous_on"]
                phase_ratios[phase]["engineering_relative_ratio_change_pct"] = (
                    100 * (on / previous - 1) if previous and on is not None else None)
                phase_ratios[phase]["engineering_system_improvements"] = {
                    s: mode_cases[s]["engineering_improvements"][phase] for s in SYSTEMS}
        report["comparisons"].append({"mode": mode, "phases": phase_ratios})
    report["evidence_sha256"] = audit.evidence
    return report


def markdown(report):
    lines = ["# N26 native PackedDirect OFF/ON comparison", "", report["scale"], "",
             report["scope"], "", report["statistic"], "", report["interpretation"], "",
             "Current OFF/ON use the same frozen source and binary; only FLOCK_PCS_PACKED_DIRECT differs.",
             "PIOP, terminal claims, commitments and PCS parameters match. Full proofs may change across routes.", ""]
    cases = {(x["mode"], x["system"]): x for x in report["cases"]}
    for comparison in report["comparisons"]:
        mode = comparison["mode"]
        lines += [f"## {mode}", "", "| Stage (s) | OFF Mikros | OFF FLOCK | ON Mikros | ON FLOCK | F/M OFF | F/M ON |",
                  "|---|---:|---:|---:|---:|---:|---:|"]
        for phase, label in PHASES.items():
            values = [cases[mode, s][a]["times_ms"][phase]["median"] / 1000
                      for a in ("off", "on") for s in SYSTEMS]
            ratios = comparison["phases"][phase]["flock_over_mikros"]
            formatted = [f"{value:.4f}" for value in values]
            formatted += [f"{ratios[a]:.3f}" if ratios[a] is not None else "n/a" for a in ("off", "on")]
            lines.append("| " + label + " | " + " | ".join(formatted) + " |")
        total = comparison["phases"]["prover_ms"]
        lines += ["", f"Relative F/M ratio change: {total['relative_ratio_change_pct']:+.2f}%.", "",
                  "| System | Time reduction | OFF/ON speedup | RSS OFF/ON median (GiB) | Proof bytes OFF/ON |",
                  "|---|---:|---:|---:|---:|"]
        for system in SYSTEMS:
            case = cases[mode, system]
            improvement = case["improvements"]["prover_ms"]
            rss = " / ".join(f"{case[a]['rss_gib']['median']:.3f}" for a in ("off", "on"))
            proof = " / ".join(str(case[a]["proof_bytes"]) for a in ("off", "on"))
            lines.append(f"| {system} | {improvement['time_reduction_pct']:+.2f}% | "
                         f"{improvement['off_over_on']:.3f} | {rss} | {proof} |")
        lines += ["", "| System | Logical witness/public | Ring claims OFF witness/public | Ring claims ON witness/public | Verify OFF/ON (ms) |",
                  "|---|---:|---:|---:|---:|"]
        for system in SYSTEMS:
            case = cases[mode, system]
            counts = {arm: case[arm]["claim_counts"] for arm in ("off", "on")}
            logical = f"{counts['off']['witness_claims']} / {counts['off']['public_claims']}"
            rings = [f"{counts[arm]['witness_ring_claims']} / {counts[arm]['public_ring_claims']}"
                     for arm in ("off", "on")]
            verification = " / ".join(f"{case[arm]['verify_ms']['median']:.3f}" for arm in ("off", "on"))
            lines.append(f"| {system} | {logical} | {rings[0]} | {rings[1]} | {verification} |")
        lines.append("")
        if "previous_on" in report["arms"]:
            lines += ["### Engineering change within the ON route", "",
                      "Previous/current ON have identical full proofs and protocol inputs; source and binary may differ.", "",
                      "| Stage (s) | Previous ON Mikros | Previous ON FLOCK | Current ON Mikros | Current ON FLOCK |",
                      "|---|---:|---:|---:|---:|"]
            for phase, label in PHASES.items():
                values = [cases[mode, system][arm]["times_ms"][phase]["median"] / 1000
                          for arm in ("previous_on", "on") for system in SYSTEMS]
                lines.append("| " + label + " | " + " | ".join(f"{v:.4f}" for v in values) + " |")
            lines += ["", "| System | Previous/current ON speedup | Time reduction | RSS previous/current ON median (GiB) | Proof bytes previous/current ON | Verify previous/current ON (ms) |",
                      "|---|---:|---:|---:|---:|---:|"]
            for system in SYSTEMS:
                case = cases[mode, system]
                improvement = case["engineering_improvements"]["prover_ms"]
                rss = " / ".join(f"{case[a]['rss_gib']['median']:.3f}" for a in ("previous_on", "on"))
                proof = " / ".join(str(case[a]["proof_bytes"]) for a in ("previous_on", "on"))
                verification = " / ".join(f"{case[a]['verify_ms']['median']:.3f}" for a in ("previous_on", "on"))
                lines.append(f"| {system} | {improvement['previous_on_over_on']:.3f} | "
                             f"{improvement['time_reduction_pct']:+.2f}% | {rss} | {proof} | {verification} |")
            lines += ["", f"Engineering change in F/M ratio: {total['engineering_relative_ratio_change_pct']:+.2f}%.", ""]
        for system in SYSTEMS:
            for arm in report["arms"]:
                case = cases[mode, system][arm]
                totals = [round(x["times"]["prover_ms"] / 1000, 6) for x in case["samples"]]
                lines.append(f"{system} {arm} online seconds, one per process: {totals}.")
        lines.append("")
    if report["validation"]["off_full_proof_matches_v6"]:
        lines += ["Every system/mode OFF full proof and commitment also matches the supplied v6 baseline.", ""]
    lines += ["Positive ratio change means this routing optimization improves Mikros's relative E2E advantage; "
              "negative change means FLOCK benefits more proportionally.", "",
              "Full per-sample timings, RSS/resource records, proof hashes and input-file SHA-256 digests are in comparison.json.", ""]
    return "\n".join(lines)


def self_test():
    """Synthetic only: test arithmetic and fail-closed validation boundaries."""
    def h(text):
        return hashlib.sha256(text.encode()).hexdigest()

    def write(path, value):
        path.write_text(json.dumps(value), encoding="utf-8")

    def fixture(folder, arm, run):
        folder.mkdir()
        enabled = arm in ("on", "previous_on")
        config = {
            "levels": [26], "repetitions": 1, "seed": 97, "threads": 64,
            "cpu_affinity": "0-47,48-55,72-79", "rustflags": "-C target-cpu=native",
            "toolchain": "+nightly-2026-03-21", "allocator_policy": "system",
            "diagnostic_trace": False, "disabled_optimizations": [],
            "measured_environment": {ROUTE_ENV: "1"} if enabled else {},
            "packed_direct": enabled, "piop_fingerprints": arm != "baseline",
            "source_manifest_sha256": h("v6" if arm == "baseline" else "v7" if arm == "previous_on" else "v8"),
        }
        receipt = {key: config[key] for key in ("source_manifest_sha256", "toolchain", "rustflags")}
        receipt["binary_sha256"] = h("v6-binary" if arm == "baseline" else "v7-binary" if arm == "previous_on" else "v8-binary")
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
                total = ({"mikros": 750, "flock": 1400} if arm == "previous_on" else
                         {"mikros": 500, "flock": 1200} if arm == "on" else
                         {"mikros": 1000, "flock": 1500})[system] * (1 + .01 * run)
                timings = dict.fromkeys(RAW_TIMES, total / 20)
                timings["prover_ms"] = total
                timings.update(witness_claims=11 if system == "mikros" else 7, public_claims=8,
                               witness_ring_claims=(2 if system == "mikros" else 4) if enabled else
                               (11 if system == "mikros" else 7), public_ring_claims=8)
                params = {"profile": "fast", "log_inv_rate": 1, "merkle_hash": "blake3", "num_lanes": None}
                sample = {**meta, "type": "sample", "verified": True, "transcript_match": True,
                          "pcs_config_match": True, "native_pcs_identity_transport": True,
                          "repetition": 0, "gates_per_instance": 2**26, "total_gates": instances * 2**26,
                          "proof_bytes": 100 if enabled else 200, "times": timings, "verify_ms": 1,
                          "proof_blake3": h(case_id + ("on" if enabled else "off")),
                          "piop_blake3": h(case_id + "piop"), "terminal_claims_blake3": h(case_id + "claims"),
                          "witness_commitment_blake3": h(mode + "witness"), "public_commitment_blake3": h(mode + "public"),
                          "witness_pcs_params": params, "public_pcs_params": params}
                resource = {"available": True, "time_exit_status": 0,
                            "max_rss_kib": (2 if arm == "previous_on" else 1) * 2**20,
                            "max_rss_gib": 2 if arm == "previous_on" else 1}
                case = {"id": case_id, "mode": mode, "system": system, "state": "complete",
                        "metadata": meta, "samples": [sample], "raw_stdout": f"{case_id}.stdout.jsonl",
                        "timings_median_ms": timings, "resources": resource,
                        **{key: sample[key] for key in COMMON_FIELDS}}
                summary["cases"].append(case)
                status["cases"].append({"id": case_id, "state": "complete", "exit_code": 0})
                (folder / case["raw_stdout"]).write_text(json.dumps(meta) + "\n" + json.dumps(sample) + "\n")
        for name, data in (("config", config), ("summary", summary), ("status", status), ("build-receipt", receipt)):
            write(folder / f"{name}.json", data)

    with tempfile.TemporaryDirectory(prefix="synthetic-direct-evidence-") as tmp:
        root = Path(tmp)
        paths = {arm: [root / f"{arm}-{r}" for r in range(3)] for arm in ("off", "on", "baseline", "previous_on")}
        for arm, folders in paths.items():
            for run, folder in enumerate(folders):
                fixture(folder, arm, run)
        compatible = compare(paths["off"], paths["on"], paths["baseline"])
        require("previous_on" not in compatible["arms"], "legacy invocation remains valid")
        report = compare(paths["off"], paths["on"], paths["baseline"], paths["previous_on"])
        total = report["comparisons"][0]["phases"]["prover_ms"]
        require(math.isclose(total["flock_over_mikros"]["off"], 1.5), "synthetic OFF ratio")
        require(math.isclose(total["flock_over_mikros"]["on"], 2.4), "synthetic ON ratio")
        require(math.isclose(total["relative_ratio_change_pct"], 60), "synthetic relative ratio")
        require(report["cases"][0]["on"]["times_ms"]["prover_ms"]["median"] == 505, "synthetic median")
        require("Ring claims ON" in markdown(report), "synthetic ring-count rendering")
        first = report["cases"][0]
        require(report["arms"]["previous_on"]["identity"]["binary_sha256"] !=
                report["arms"]["on"]["identity"]["binary_sha256"], "previous binary may differ")
        require(first["previous_on"]["proof_blake3"] == first["on"]["proof_blake3"], "previous proof is identical")
        require(math.isclose(first["engineering_improvements"]["prover_ms"]["previous_on_over_on"], 1.5),
                "synthetic engineering speedup")
        require(first["previous_on"]["rss_gib"]["median"] == 2, "previous resource summary")
        require("Engineering change within" in markdown(report), "engineering comparison rendering")
        checks = 11

        def rejects(file, mutate, also_raw=False):
            nonlocal checks
            original = file.read_bytes()
            data = json.loads(original)
            mutate(data)
            backups = {}
            if also_raw:
                for case in data["cases"]:
                    raw = file.parent / case["raw_stdout"]
                    backups[raw] = raw.read_bytes()
                    raw.write_text(json.dumps(case["metadata"]) + "\n" + json.dumps(case["samples"][0]) + "\n")
            write(file, data)
            try:
                try:
                    compare(paths["off"], paths["on"], paths["baseline"], paths["previous_on"])
                except ValueError:
                    checks += 1
                else:
                    raise AssertionError(f"Expected rejection: {file}")
            finally:
                file.write_bytes(original)
                for raw, content in backups.items():
                    raw.write_bytes(content)

        folder = paths["on"][0]
        rejects(folder / "summary.json", lambda d: d.update(all_comparisons_valid=False))
        rejects(folder / "status.json", lambda d: d.update(source_hashes_verified_before_and_after=False))
        rejects(folder / "build-receipt.json", lambda d: d.update(binary_sha256=h("wrong binary")))
        rejects(folder / "summary.json", lambda d: d["cases"][0]["samples"][0].update(piop_blake3=h("wrong piop")), True)
        rejects(folder / "summary.json", lambda d: d["cases"][0]["samples"][0].update(terminal_claims_blake3=h("wrong claims")), True)
        rejects(folder / "summary.json", lambda d: d["cases"][0]["samples"][0].update(proof_blake3=h("unstable proof")), True)
        rejects(paths["baseline"][0] / "summary.json", lambda d: d["cases"][0]["samples"][0].update(proof_blake3=h("wrong v6")), True)
        rejects(folder / "config.json", lambda d: d.update(seed=123))
        rejects(folder / "summary.json", lambda d: d["cases"][0]["samples"][0].update(seed=123), True)
        rejects(folder / "summary.json", lambda d: d["cases"][0]["samples"][0]["times"].update(prover_ms=-1), True)

        # Keep the changed previous arm internally stable: rejection must come
        # from the cross-version full-proof check, not within-arm instability.
        for field, changed in (("proof_blake3", h("different previous proof")), ("proof_bytes", 101)):
            backups = {}
            try:
                for folder in paths["previous_on"]:
                    file = folder / "summary.json"
                    backups[file] = file.read_bytes()
                    data = json.loads(backups[file])
                    case = data["cases"][0]
                    case["samples"][0][field] = changed
                    raw = folder / case["raw_stdout"]
                    backups[raw] = raw.read_bytes()
                    raw.write_text(json.dumps(case["metadata"]) + "\n" + json.dumps(case["samples"][0]) + "\n")
                    write(file, data)
                try:
                    compare(paths["off"], paths["on"], paths["baseline"], paths["previous_on"])
                except ValueError as error:
                    require(f"previous ON/current ON {field} mismatch" in str(error),
                            f"wrong validation boundary: {error}")
                    checks += 1
                else:
                    raise AssertionError(f"Expected cross-version proof rejection: {field}")
            finally:
                for path, data in backups.items():
                    path.write_bytes(data)
        print(f"Synthetic self-test passed ({checks} checks); no benchmark executed or measured report produced.")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--off", nargs="+", type=Path)
    parser.add_argument("--on", nargs="+", type=Path)
    parser.add_argument("--baseline", nargs="+", type=Path, default=[])
    parser.add_argument("--previous-on", nargs="+", type=Path, default=[],
                        help="Earlier ON implementation; full proof must match current ON despite a different binary")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        require(not (args.off or args.on or args.baseline or args.previous_on or args.output),
                "--self-test cannot consume measured evidence")
        self_test()
        return
    if not (args.off and args.on and args.output):
        parser.error("--off, --on and --output are required")
    report = compare(args.off, args.on, args.baseline, args.previous_on)
    args.output.mkdir(parents=True, exist_ok=True)
    (args.output / "comparison.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    rendered = markdown(report)
    (args.output / "comparison.md").write_text(rendered, encoding="utf-8")
    print(rendered)


if __name__ == "__main__":
    main()
