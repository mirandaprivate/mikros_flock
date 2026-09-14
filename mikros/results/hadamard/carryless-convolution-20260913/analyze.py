#!/usr/bin/env python3
"""Validate carry-less Hadamard evidence and exact source-check reuse per input.

Examples (only reads JSON unless --output is explicitly supplied):
    python analyze.py
    python analyze.py --root ./server_evidence --output analysis.json
    python analyze.py ./screening ./large ./results/profiles ./results/RSS

Archives are not unpacked. Warmups, ordinary timing, independent profiles and
fresh-process cold proofs remain separate. A profile certificate timer is the
PER-SAMPLE sum of aggregation and quotient time, summarized only afterwards.
Every cache hit must refer to an earlier successful full oracle on the same
dimension/run/input/proof in this report. No cache anchor crosses a run or file.
This script writes no Markdown report and invents no missing measurements.
"""
from __future__ import annotations

import argparse
from collections import Counter, defaultdict
import hashlib
import json
import math
import os
from pathlib import Path
import re
import statistics
import sys
from typing import Any


PROTOCOL = "mikros-univariate-gf256-v1"
REQUIRED_ISAS = {"gfni", "avx512f", "avx512bw", "vpclmulqdq", "pclmulqdq", "sse4.1"}
TIME_KEYS = ("prover_ms", "verifier_ms", "input_claim_check_ms")
PROOF_FLAGS = ("verifier_passed", "transcript_matched", "source_claims_verified")
SOURCE_KEYS = ("mikros_sources_blake3", "flock_sources_blake3", "build_configuration_blake3")
CARRYLESS_SYSTEMS = {"coeffclmul", "coeffclmulreuse", "coeffnibble", "coeffnibblereuse"}
KNOWN_SYSTEMS = {
    "reference", "lookup", "native", "reuse", "tiled", "tiledreuse",
    "basis", "basisreuse", "register", "registerreuse", "registerpacked", "registerpackedreuse",
    "split", "splitreuse", "half", "halfreuse", "pair", "pairreuse", "affine", "affinereuse",
    "joint", "jointreuse", "coefflookup", "coefflookupreuse", "coeffnative", "coeffnativereuse",
    *CARRYLESS_SYSTEMS, "flock128",
}
SOURCE_METHODS = {"full_oracle", "exact_claim_cache"}
AGGREGATE = "small_field_aggregate_ms"
QUOTIENT = "interpolate_quotient_ms"
HEX64 = re.compile(r"[0-9a-fA-F]{64}\Z")
MODES = {"paired_warm_timing", "paired_profile_diagnostic", "fresh_process_rss_cold_proof"}
SKIP_DIRS = {"source", "sources", "target", ".git", "__pycache__", "node_modules", "metadata"}
SKIP_NAMES = {
    "analysis.json", "summary.json", "source_manifest.json", "source-audit.json",
    "aggregate_build_artifact.json", "server_environment.json", "commands.json",
    "status.json", "selection.json", "hardware.json", "evidence-manifest.json",
    "evidence_manifest.json", "screening-launch.json", "results-launch.json",
}


class AuditError(ValueError):
    """Evidence is incomplete, incompatible, or internally inconsistent."""


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AuditError(message)


def integer(value: Any, minimum: int = 0) -> bool:
    return type(value) is int and value >= minimum


def finite(value: Any, minimum: float = 0.0) -> bool:
    return type(value) in (int, float) and math.isfinite(value) and value >= minimum


def statistics_ms(values: list[float]) -> dict[str, Any]:
    require(bool(values), "cannot summarize an empty series")
    return {
        "samples": len(values), "median": statistics.median(values),
        "min": min(values), "max": max(values), "mean": statistics.mean(values),
    }


def label(path: Path, root: Path) -> str:
    try:
        return path.relative_to(root).as_posix()
    except ValueError:
        return str(path)


def json_files(roots: list[Path]) -> list[Path]:
    found: set[Path] = set()
    for root in roots:
        require(root.exists(), f"input does not exist: {root}")
        if root.is_file():
            require(root.suffix.lower() == ".json", f"expected extracted JSON, not an archive: {root}")
            found.add(root.resolve())
            continue
        for directory, subdirs, files in os.walk(root, followlinks=False):
            subdirs[:] = sorted(d for d in subdirs if d.lower() not in SKIP_DIRS)
            for name in files:
                lower = name.lower()
                if not lower.endswith(".json") or lower in SKIP_NAMES:
                    continue
                if lower.endswith((".command.json", ".resources-before.json")):
                    continue
                found.add((Path(directory) / name).resolve())
    return sorted(found)


def looks_like_benchmark(report: Any) -> bool:
    if not isinstance(report, dict) or not isinstance(report.get("metadata"), dict):
        return False
    meta = report["metadata"]
    # Source manifests, build/driver records, summaries and unrelated metadata
    # are not benchmark samples. Incomplete benchmark-shaped files must fail
    # validation below instead of silently disappearing from the inventory.
    return meta.get("protocol") == PROTOCOL or (
        {"systems", "logs", "runs"} <= meta.keys()
        and any(key in report for key in ("samples", "warmups", "summaries"))
    )


def validate_report(report: dict[str, Any], name: str) -> None:
    prefix = f"{name}: "
    check = lambda condition, message: require(condition, prefix + message)
    check(type(report.get("schema_version")) is int and report["schema_version"] == 1,
          "unsupported or missing schema_version")
    meta = report["metadata"]
    for key, value in {
        "protocol": PROTOCOL, "kappa": 128, "intermediate_field_bits": 8,
        "extension_coordinates": 16, "flock_skip_rounds": 7,
        "threads": 64, "rayon_actual_threads": 64,
        "os": "linux", "arch": "x86_64", "native_backend": "gfni-avx512",
    }.items():
        check(meta.get(key) == value, f"metadata {key} must be {value!r}")
    if "flock_actual_threads" in meta:
        check(meta["flock_actual_threads"] == 64, "Flock pool is not 64 threads")
    features = meta.get("compiled_target_features")
    check(isinstance(features, list) and all(isinstance(x, str) for x in features),
          "missing compiled ISA inventory")
    check(REQUIRED_ISAS <= set(features), f"missing required ISAs: {sorted(REQUIRED_ISAS - set(features))}")
    for key in SOURCE_KEYS:
        check(isinstance(meta.get(key), str) and HEX64.fullmatch(meta[key]) is not None,
              f"missing or invalid {key}")
    check(isinstance(meta.get("cpu"), str) and bool(meta["cpu"]), "missing CPU description")
    check(isinstance(meta.get("grinding"), str) and bool(meta["grinding"]), "missing grinding policy")
    for key in ("scope", "setup", "timing", "scratch", "kernel_scope"):
        check(isinstance(meta.get(key), str) and bool(meta[key]), f"missing measurement boundary: {key}")
    systems, logs, runs = meta.get("systems"), meta.get("logs"), meta.get("runs")
    check(isinstance(systems, list) and bool(systems)
          and all(isinstance(x, str) and re.fullmatch(r"[a-z][a-z0-9_]*", x) for x in systems)
          and len(set(systems)) == len(systems), "invalid/duplicate system list")
    check(set(systems) <= KNOWN_SYSTEMS, f"unknown systems: {sorted(set(systems) - KNOWN_SYSTEMS)}")
    check(type(meta.get("source_check_reuse")) is bool,
          "source_check_reuse must explicitly declare this run's cache policy")
    check(isinstance(meta.get("source_check_policy"), str) and bool(meta["source_check_policy"]),
          "missing source-check boundary/policy")
    if CARRYLESS_SYSTEMS.intersection(systems):
        check(meta.get("carryless_backend") == "vpclmul-avx512", "carry-less run used an unexpected/fallback backend")
        check(isinstance(meta.get("carryless_scope"), str) and bool(meta["carryless_scope"]),
              "missing carry-less conversion/timing boundary")
    check(isinstance(logs, list) and bool(logs)
          and all(integer(n, 7) and n <= 30 for n in logs)
          and len(set(logs)) == len(logs), "invalid/duplicate log dimensions")
    check(integer(runs, 1), "runs must be a positive integer")
    mode, profile = meta.get("mode"), meta.get("profile")
    check(mode in MODES and type(profile) is bool, "unknown mode or invalid profile flag")
    cold = mode == "fresh_process_rss_cold_proof"
    check(mode != "paired_profile_diagnostic" or profile, "profile mode lacks profiling flag")
    check(mode != "paired_warm_timing" or not profile, "ordinary timing contains profiling")
    # The existing driver emits smoke.json with N14/run1, intentionally before
    # the balanced measurements. This narrow exception never exempts main runs.
    smoke = "smoke" in Path(name).stem.lower() and logs == [14] and runs == 1
    balanced = runs % len(systems) == 0
    check(type(meta.get("position_balanced")) is bool and meta["position_balanced"] == balanced,
          "position_balanced flag contradicts run/system counts")
    if mode == "paired_warm_timing" and not smoke:
        check(balanced, "formal timing lacks complete position-balanced rotations")
    if cold:
        check(runs == len(systems) == len(logs) == 1,
              "a fresh-process cold report must contain exactly one proof/system/dimension")

    measured, warmups, summaries = (report.get(key) for key in ("samples", "warmups", "summaries"))
    check(all(isinstance(x, list) for x in (measured, warmups, summaries)), "missing sample/summary arrays")
    check(len(measured) == len(logs) * len(systems) * runs, "wrong measured sample count")
    check(len(warmups) == (0 if cold else len(logs) * len(systems)), "wrong warmup count")
    seen: set[tuple[int, int, str]] = set()
    grouped: dict[tuple[int, int], list[dict[str, Any]]] = defaultdict(list)
    for is_warmup, rows in ((False, measured), (True, warmups)):
        for sample in rows:
            check(isinstance(sample, dict), "sample is not an object")
            n, run, system = (sample.get(key) for key in ("log_n", "run", "system"))
            check(integer(n, 7) and n in logs and integer(run) and system in systems,
                  "sample has an undeclared dimension/run/system")
            key = n, run, system
            check(key not in seen, f"duplicate sample {key}")
            seen.add(key)
            check(run == 0 if is_warmup or cold else 1 <= run <= runs,
                  f"misclassified run {key}")
            check(integer(sample.get("order")) and sample["order"] < len(systems),
                  f"invalid execution order {key}")
            for flag in PROOF_FLAGS:
                check(sample.get(flag) is True, f"failed/missing {flag} in {key}")
            check(sample.get("source_check_method") in SOURCE_METHODS,
                  f"missing/unknown source-check method in {key}")
            check(isinstance(sample.get("source_check_anchor"), str)
                  and sample["source_check_anchor"] in systems,
                  f"missing/undeclared source-check anchor in {key}")
            for digest in ("input_digest", "proof_digest"):
                check(isinstance(sample.get(digest), str) and HEX64.fullmatch(sample[digest]) is not None,
                      f"invalid {digest} in {key}")
            for metric in TIME_KEYS:
                check(finite(sample.get(metric)), f"invalid {metric} in {key}")
            check(sample["prover_ms"] > 0, f"nonpositive prover time in {key}")
            fields = (259 if system == "flock128" else 258) + 2 * (n - 7)
            expected_sizes = fields, fields * 16, fields * 16 + (32 if system == "flock128" else 24)
            actual_sizes = tuple(sample.get(x) for x in ("payload_field_elements", "payload_field_bytes", "bincode_bytes"))
            check(all(integer(x, 1) for x in actual_sizes) and actual_sizes == expected_sizes,
                  f"proof payload changed in {key}")
            before, peak = sample.get("vm_rss_before_prove_kib"), sample.get("vm_hwm_after_prove_kib")
            check(integer(before, 1) and integer(peak, before) if cold else before is None and peak is None,
                  f"invalid/mixed cold-memory measurement in {key}")
            phases = sample.get("mikros_phases")
            if profile and system != "flock128":
                check(isinstance(phases, dict) and {AGGREGATE, QUOTIENT} <= phases.keys(),
                      f"missing certificate phase timers in {key}")
                check(all(isinstance(k, str) and k.endswith("_ms") and finite(v) for k, v in phases.items()),
                      f"invalid phase timer in {key}")
                check(sum(phases.values()) <= sample["prover_ms"] + 0.001,
                      f"phase sum exceeds same-sample prover time in {key}")
            else:
                check(phases is None or phases == {}, f"unexpected phase timers in {key}")
            grouped[n, run].append(sample)
    expected_runs = [0] if cold else range(runs + 1)
    check(seen == {(n, run, system) for n in logs for run in expected_runs for system in systems},
          "incomplete run coverage")
    # The driver appends each list in log/run/execution-position order. Check
    # the physical array order too: sorting attacker-supplied order fields must
    # not make a later oracle appear to have established an earlier cache hit.
    for is_warmup, rows in ((True, warmups), (False, measured)):
        run_numbers = [] if cold and is_warmup else [0] if is_warmup or cold else range(1, runs + 1)
        expected_sequence = [
            (n, run, systems[(run + order) % len(systems)], order)
            for n in logs for run in run_numbers for order in range(len(systems))
        ]
        actual_sequence = [(s["log_n"], s["run"], s["system"], s["order"]) for s in rows]
        check(actual_sequence == expected_sequence,
              f"{'warmup' if is_warmup else 'measured'} records are not in actual execution order")
    for (n, run), rows in grouped.items():
        check(len({s["input_digest"].lower() for s in rows}) == 1, f"unpaired inputs at {(n, run)}")
        check(len({s["proof_digest"].lower() for s in rows if s["system"] != "flock128"}) <= 1,
              f"Mikros proof bytes differ at {(n, run)}")
        check(sorted(s["order"] for s in rows) == list(range(len(systems))),
              f"execution positions are duplicated at {(n, run)}")
        actual = [s["system"] for s in sorted(rows, key=lambda s: s["order"])]
        expected = [systems[(run + i) % len(systems)] for i in range(len(systems))]
        check(actual == expected, f"unexpected rotation at {(n, run)}")
        validate_source_checks(rows, meta["source_check_reuse"], f"{name}/{n}/{run}")
    if mode == "paired_warm_timing" and not smoke:
        for n in logs:
            for system in systems:
                counts = Counter(s["order"] for s in measured if (s["log_n"], s["system"]) == (n, system))
                check(counts == {i: runs // len(systems) for i in range(len(systems))},
                      f"actual execution positions are unbalanced for {(n, system)}")

    check(len(summaries) == len(logs) * len(systems), "wrong summary count")
    summary_keys: set[tuple[int, str]] = set()
    for summary in summaries:
        check(isinstance(summary, dict), "summary is not an object")
        key = summary.get("log_n"), summary.get("system")
        check(key not in summary_keys and key[0] in logs and key[1] in systems, "invalid/duplicate summary")
        summary_keys.add(key)
        rows = [s for s in measured if (s["log_n"], s["system"]) == key]
        check(summary.get("runs") == len(rows) == runs, f"summary sample count mismatch: {key}")
        for size in ("payload_field_bytes", "bincode_bytes"):
            check(summary.get(size) == rows[0][size], f"summary {size} mismatch: {key}")
        for metric in TIME_KEYS:
            recalculated = statistics_ms([s[metric] for s in rows])
            saved = summary.get(metric)
            check(isinstance(saved, dict), f"missing summary {metric}: {key}")
            for stat in ("median", "min", "max"):
                check(finite(saved.get(stat)) and math.isclose(saved[stat], recalculated[stat], rel_tol=1e-12, abs_tol=1e-9),
                      f"incorrect saved {metric}/{stat}: {key}")


def validate_source_checks(rows: list[dict[str, Any]], reuse: bool, name: str) -> None:
    """Replay the recorded cache state for exactly one immutable-input scope.

    The report records a content digest, not memory addresses or serialized
    Claims. The benchmark's SourceCheckCache additionally enforces identical
    slice pointers/lengths and full Claims equality before emitting a cache hit;
    this audit checks the observable anchor, chronology and proof/input identity.
    """
    anchor: dict[str, Any] | None = None
    for sample in rows:
        system = sample["system"]
        method = sample["source_check_method"]
        label = sample["source_check_anchor"]
        prefix = f"{name}/{system}: "
        if system == "flock128":
            require(method == "full_oracle" and label == system,
                    prefix + "Flock must always run its own full source oracle")
            # Flock's different terminal-query family cannot seed Mikros's cache.
            continue
        if not reuse or anchor is None:
            require(method == "full_oracle" and label == system,
                    prefix + "first Mikros query or disabled reuse requires a self-anchored full oracle")
            anchor = sample
            continue
        require(method == "exact_claim_cache", prefix + "enabled reuse must record the exact claim cache hit")
        require(label == anchor["system"], prefix + "cache hit names a different/unestablished anchor")
        require(anchor["source_check_method"] == "full_oracle" and anchor["order"] < sample["order"],
                prefix + "cache anchor is not an earlier full oracle")
        for key in ("log_n", "run", "input_digest", "proof_digest"):
            require(sample[key] == anchor[key], prefix + f"cache identity differs from anchor: {key}")


def summarize_report(report: dict[str, Any], name: str, digest: str) -> dict[str, Any]:
    meta = report["metadata"]
    rows = report["samples"]
    groups = []
    for n in meta["logs"]:
        for system in meta["systems"]:
            selected = [s for s in rows if (s["log_n"], s["system"]) == (n, system)]
            warmups = [s for s in report["warmups"] if (s["log_n"], s["system"]) == (n, system)]
            item: dict[str, Any] = {
                "log_n": n, "n_bits": 1 << n, "system": system,
                **{key: statistics_ms([s[key] for s in selected]) for key in TIME_KEYS},
                "payload_field_bytes": selected[0]["payload_field_bytes"],
                "bincode_bytes": selected[0]["bincode_bytes"],
                "warmup_prover_ms": [s["prover_ms"] for s in warmups],
                "source_checks": {
                    "measured_methods": dict(Counter(s["source_check_method"] for s in selected)),
                    "warmup_methods": dict(Counter(s["source_check_method"] for s in warmups)),
                    "input_claim_check_ms_by_method": {
                        method: statistics_ms([s["input_claim_check_ms"] for s in selected
                                               if s["source_check_method"] == method])
                        for method in sorted({s["source_check_method"] for s in selected})
                    },
                    "per_run": [{"run": s["run"], "order": s["order"],
                                 "method": s["source_check_method"], "anchor": s["source_check_anchor"]}
                                for s in selected],
                },
            }
            if meta["profile"] and system != "flock128":
                phases = [s["mikros_phases"] for s in selected]
                keys = set(phases[0])
                require(all(set(p) == keys for p in phases), f"{name}: profile phase schema changes within {(n, system)}")
                certificate = [p[AGGREGATE] + p[QUOTIENT] for p in phases]
                item["profile"] = {
                    "certificate_ms": statistics_ms(certificate),
                    "certificate_share_of_mean_prover": statistics.mean(certificate) / statistics.mean(s["prover_ms"] for s in selected),
                    "phases_ms": {key: statistics_ms([p[key] for p in phases]) for key in sorted(keys)},
                    "non_certificate_ms": statistics_ms([s["prover_ms"] - value for s, value in zip(selected, certificate)]),
                    "unaccounted_ms": statistics_ms([s["prover_ms"] - sum(p.values()) for s, p in zip(selected, phases)]),
                    "certificate_per_run_ms": [{"run": s["run"], "value": value} for s, value in zip(selected, certificate)],
                }
            if meta["mode"] == "fresh_process_rss_cold_proof":
                item["memory_kib"] = {
                    key: statistics_ms([s[key] for s in selected])
                    for key in ("vm_rss_before_prove_kib", "vm_hwm_after_prove_kib")
                }
            groups.append(item)
    return {"file": name, "sha256": digest, "metadata": meta, "groups": groups}


def analyze(paths: list[Path], display_root: Path) -> dict[str, Any]:
    reports = []
    identities: dict[tuple[int, int], tuple[str, str]] = {}
    proofs: dict[tuple[int, int, str], tuple[str, str]] = {}
    provenance: dict[str, set[str]] = {key: set() for key in SOURCE_KEYS}
    skipped = []
    records = 0
    source_methods: Counter[str] = Counter()
    for path in json_files(paths):
        name = label(path, display_root)
        try:
            raw = path.read_bytes()
            report = json.loads(raw.decode("utf-8-sig"))
        except (OSError, UnicodeError, json.JSONDecodeError) as error:
            raise AuditError(f"{name}: cannot read JSON: {error}") from error
        if not looks_like_benchmark(report):
            skipped.append(name)
            continue
        validate_report(report, name)
        for sample in report["warmups"] + report["samples"]:
            key = sample["log_n"], sample["run"]
            identity = sample["input_digest"].lower(), name
            previous = identities.setdefault(key, identity)
            require(previous[0] == identity[0], f"unpaired cross-file inputs at {key}: {previous[1]} versus {name}")
            family = "flock128" if sample["system"] == "flock128" else "mikros"
            proof_key = *key, family
            proof = sample["proof_digest"].lower(), name
            previous = proofs.setdefault(proof_key, proof)
            require(previous[0] == proof[0], f"cross-file {family} proof bytes differ at {key}: {previous[1]} versus {name}")
            records += 1
            source_methods[sample["source_check_method"]] += 1
        for key in SOURCE_KEYS:
            provenance[key].add(report["metadata"][key])
        reports.append(summarize_report(report, name, hashlib.sha256(raw).hexdigest()))
    require(bool(reports), "no extracted benchmark reports found; extract evidence.tar.gz or supply --root / explicit JSON paths")
    warnings = [
        f"Multiple {key} fingerprints: files are reported separately and must not be treated as one frozen build."
        for key, values in provenance.items() if len(values) > 1
    ]
    return {
        "analysis_schema_version": 2, "validated": True,
        "validation": {
            "reports": len(reports), "proof_records_including_warmups": records,
            "unique_inputs": len(identities), "unique_family_proof_identities": len(proofs),
            "required_threads": 64, "required_compiled_isas": sorted(REQUIRED_ISAS),
            "formal_position_balance_checked": True, "saved_summaries_recomputed": True,
            "cross_file_input_and_proof_identities_checked": True,
            "source_check_policy_and_execution_order_checked": True,
            "cache_anchors_are_prior_full_oracles_in_same_input_scope": True,
            "source_check_methods_including_warmups": dict(source_methods),
            "source_fingerprints": {key: sorted(values) for key, values in provenance.items()},
        },
        "measurement_policy": [
            "Each file is an independent experiment; neither dimensions nor timing/profile/cold processes are pooled.",
            "All warmup records are validated but excluded from measured statistics.",
            "certificate_ms is median/mean of per-sample aggregate + quotient, never the sum of two medians.",
            "Reported setup/offline/online boundaries are preserved verbatim in each file's metadata; no time is subtracted.",
            "Cold RSS includes the input/runtime; one fresh proof is not a stable estimate or a cache-miss measurement.",
            "Validation checks recorded thread/ISA declarations and proof consistency, not hardware execution counters or CPU exclusivity.",
            "Carry-less byte/nibble modes require the declared VPCLMUL-AVX512 backend and preserve online encoding/convolution/decoding in prover timing.",
            "Each dimension/run resets the Mikros source-check anchor; Flock always runs a separate full source oracle.",
            "Source-check cache hits require a prior self-anchored full oracle with identical input/proof identity; no anchor is reused across files or runs.",
            "Runtime pointer+length identity and full Claims equality are enforced by SourceCheckCache; JSON exposes digests/anchors, not pointers or full Claims.",
            "Full-oracle and exact-claim-cache input checking times are also reported separately; both remain outside prover timing.",
        ],
        "warnings": warnings, "skipped_non_benchmark_json": skipped, "files": reports,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("paths", nargs="*", type=Path, help="optional extracted JSON files/directories; defaults to --root")
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parent, help="discovery root and relative-path base")
    parser.add_argument("--output", type=Path, help="optional analysis JSON; defaults to stdout without writing files")
    args = parser.parse_args(argv)
    root = args.root.resolve()
    try:
        result = analyze([path.resolve() for path in args.paths] or [root], root)
        output = json.dumps(result, indent=2, ensure_ascii=False, allow_nan=False) + "\n"
        if args.output:
            destination = args.output.resolve()
            # Never overwrite one of the raw reports just analyzed.
            inputs = {str(path) for path in json_files([path.resolve() for path in args.paths] or [root])}
            require(str(destination) not in inputs, f"refusing to overwrite an existing input JSON: {destination}")
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_text(output, encoding="utf-8")
        else:
            sys.stdout.write(output)
    except (AuditError, OSError, TypeError, KeyError) as error:
        print(f"Hadamard evidence validation failed: {error}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
