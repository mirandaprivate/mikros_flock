"""Read-only audit of retained tree experiments; never runs or estimates a case."""
import argparse
import hashlib
import json
from pathlib import Path
import statistics
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from common import CPU_SPEC, NATIVE_FEATURES, TOOLCHAIN, cases, require, save, atomic_text
from aggregate_results import validate_case, TIMING_KEYS

BASE_ENV = dict.fromkeys(("FLOCK_PCS_PACKED_DIRECT", "FLOCK_WP_UNINIT",
                         "MIKROS_GATE_VIRTUAL_PRODUCT", "MIKROS_WH_TILED_WEIGHTS",
                         "E2E_COPY_PAIRED_TERMINALS", "MIKROS_GATE_WH_SCRATCH"), "1")
TREE_ENV = {"tree_dense_simd": "MIKROS_TREE_DENSE_SIMD", "tree_large_pool": "MIKROS_TREE_LARGE_POOL",
            "tree_register_fusion": "MIKROS_TREE_REGISTER_FUSION"}
IDENTITY = ("proof_blake3", "piop_blake3", "terminal_claims_blake3", "witness_commitment_blake3",
            "public_commitment_blake3", "witness_pcs_params", "public_pcs_params", "proof_bytes")


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def read(path):
    return json.loads(Path(path).read_text(encoding="utf-8"))


def audit(folder, manifests):
    folder = Path(folder).resolve()
    hashes = read(folder / "evidence_sha256.json")
    require(hashes, "empty evidence receipt")
    for name, expected in hashes.items():
        require(Path(name).name == name and "\\" not in name, "unsafe evidence filename")
        require(digest(folder / name) == expected, f"changed evidence {folder / name}")
    config, status, receipt = (read(folder / name) for name in ("config.json", "status.json", "build-receipt.json"))
    require(status.get("state") == "complete", f"incomplete run: {folder}")
    require(status.get("source_hashes_verified_before_and_after") is True and
            status.get("binary_hashes_verified_before_and_after") is True, "missing final hash checks")
    require(config["source_manifest_sha256"] == receipt["source_manifest_sha256"] in manifests, "unknown source")
    require(config["toolchain"] == receipt["toolchain"] == TOOLCHAIN, "toolchain changed")
    require(config["rustflags"] == receipt["rustflags"] == "-C target-cpu=native", "native build mismatch")
    require(config["threads"] == 64 and config["cpu_affinity"] == CPU_SPEC, "thread/affinity mismatch")
    require(config["repetitions"] == 1 and config["allocator_policy"] == "system", "require fresh-process system allocator samples")
    extra = {env: "1" for flag, env in TREE_ENV.items() if config.get(flag, False)}
    require(config["measured_environment"] == {**BASE_ENV, **extra}, "unexpected flags or tracing in primary run")
    require(config["diagnostic_trace"] is False and config["disabled_optimizations"] == [], "nonprimary settings")
    require(len(receipt["binary_sha256"]) == 64, "invalid binary receipt")
    hardware = read(folder / "hardware.json")
    require(len(hardware["selected_cpu_flags"]) == 64, "missing CPU evidence")
    require(all(NATIVE_FEATURES <= set(flags) for flags in hardware["selected_cpu_flags"].values()), "missing native ISA")
    planned = cases(config["levels"], config.get("modes", ("single", "multiple")))
    required = {"config.json", "status.json", "build-receipt.json", "hardware.json", "summary.json"}
    required.update(c["id"] + suffix for c in planned for suffix in
                    (".stdout.jsonl", ".stderr.log", ".command.json", ".resource.txt", ".resources-before.json"))
    require(required <= hashes.keys(), "evidence receipt omits required raw records")
    require([(e["id"], e["state"]) for e in status["cases"]] == [(c["id"], "complete") for c in planned], "case set/order mismatch")
    result = []
    summary = read(folder / "summary.json")
    require(summary["all_requested_cases_complete"] and summary["all_comparisons_valid"], "summary is not complete")
    require(summary["config"] == config, "summary configuration drift")
    for case in planned:
        data = validate_case(folder, case, 1, config["seed"])
        saved = next(c for c in summary["cases"] if c["id"] == case["id"])
        require(data == saved, f"raw/summary mismatch {case['id']}")
        before = read(folder / (case["id"] + ".resources-before.json"))
        require(before["selected_cpu_busy_fraction"] < 0.1 and before["disk_free_bytes"] >= 1 << 30, "busy or full server")
        require(before["memory_kib"]["MemAvailable"] >= before["required_available_gib"] * (1 << 20), "insufficient preflight RAM")
        command = read(folder / (case["id"] + ".command.json"))
        expected_tail = [receipt["binary"], "--mode", case["mode"], "--system", case["system"], "--log-gates", str(case["log_gates"]), "--threads", "64", "--repetitions", "1", "--seed", str(config["seed"])]
        require(command["command"][-len(expected_tail):] == expected_tail, "wrong benchmark command")
        require(command["command"][:2] == ["/usr/bin/time", "-v"] and command["command"][4:7] == ["taskset", "-c", CPU_SPEC], "missing resource/affinity wrapper")
        require(command["cwd"] == config["source_dir"], "wrong command cwd")
        require(all(command["env"].get(k) == v for k, v in config["measured_environment"].items()), "environment mismatch")
        result.append({"directory": str(folder), "case": case, "sample": data["samples"][0],
                       "resources": data["resources"], "manifest_sha256": receipt["source_manifest_sha256"],
                       "binary_sha256": receipt["binary_sha256"], "tree_flags": extra})
    for level in config["levels"]:
        for mode in config.get("modes", ("single", "multiple")):
            pair = [x["sample"] for x in result if x["case"]["log_gates"] == level and x["case"]["mode"] == mode]
            require(len(pair) == 2, "incomplete comparison pair")
            for key in ("witness_commitment_blake3", "public_commitment_blake3", "witness_pcs_params", "public_pcs_params"):
                require(pair[0][key] == pair[1][key], f"systems differ: {key}")
    return result


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--manifest", action="append", type=Path, required=True)
    p.add_argument("--run", action="append", nargs=2, metavar=("ARM", "DIRECTORY"), required=True)
    p.add_argument("--output", type=Path)
    a = p.parse_args()
    manifests = {digest(path): str(path.resolve()) for path in a.manifest}
    rows, seen, identities, binaries = [], set(), {}, {}
    for arm, path in a.run:
        require(str(Path(path).resolve()) not in seen, "directory reused as another sample")
        seen.add(str(Path(path).resolve()))
        for row in audit(path, manifests):
            row["arm"] = arm
            key = row["case"]["id"]
            identity = [row["sample"][field] for field in IDENTITY]
            require(identities.setdefault(key, identity) == identity, f"proof/claims/PCS changed across arms: {key}")
            source = row["manifest_sha256"]
            require(binaries.setdefault(source, row["binary_sha256"]) == row["binary_sha256"], "binary changed for same source")
            rows.append(row)
    groups = {}
    for row in rows:
        key = (row["case"]["log_gates"], row["case"]["mode"], row["arm"], row["case"]["system"])
        groups.setdefault(key, []).append(row)
    table = []
    for key, group in sorted(groups.items()):
        require(len({json.dumps(r["tree_flags"], sort_keys=True) for r in group}) == 1, "arm flag drift")
        stats = {phase: statistics.median(r["sample"]["times"][phase] for r in group) for phase in TIMING_KEYS}
        table.append({"log_gates": key[0], "mode": key[1], "arm": key[2], "system": key[3], "fresh_processes": len(group),
                      "median_ms": stats, "prover_range_ms": [min(r["sample"]["times"]["prover_ms"] for r in group), max(r["sample"]["times"]["prover_ms"] for r in group)],
                      "max_rss_gib": max(r["resources"]["max_rss_gib"] for r in group)})
    lines = ["Mikros multiple product-tree / N30 server comparison", "",
             "64 threads; identical native FLOCK PCS parameters and commitments for both systems.",
             "single: 1 x 2^log_gates binary gates; multiple: 128 x 2^log_gates, including 2^37 total at N30.",
             "Online includes all conversions, commits/recommits, gate/copy PIOPs, native PCS, releases and serialization.",
             "Offline fixture/static preprocessing excluded; verification is separate; MaxRSS includes offline.",
             "One sample is descriptive only. >=3 fresh processes use medians; phases' medians need not sum to total.",
             "Unchanged PCS timing fluctuations are not attributed to product-tree optimization.", "",
             "log | mode | arm | system | processes | prover_s | gate_s | copy_s | witness_PCS_s | public_PCS_s | MaxRSS_GiB"]
    for row in table:
        t = row["median_ms"]
        lines.append(f"{row['log_gates']} | {row['mode']} | {row['arm']} | {row['system']} | {row['fresh_processes']} | " +
                     " | ".join(f"{t[k]/1000:.6f}" for k in ("prover_ms", "gates_ms", "copy_ms", "witness_pcs_open_ms", "public_pcs_open_ms")) + f" | {row['max_rss_gib']:.3f}")
    lines.extend(["", "Measured ratios FLOCK / Mikros:"])
    for level, mode, arm in sorted({k[:3] for k in groups}):
        pair = {r["system"]: r for r in table if (r["log_gates"], r["mode"], r["arm"]) == (level, mode, arm)}
        require(set(pair) == {"flock", "mikros"}, "missing system")
        ratio = pair["flock"]["median_ms"]["prover_ms"] / pair["mikros"]["median_ms"]["prover_ms"]
        lines.append(f"N{level} {mode} {arm}: {ratio:.6f}x")
    lines.extend(["", "Whole-process I/O observations (not phase-specific):"])
    for row in rows:
        resource = row["resources"]
        if resource.get("major_page_faults", 0) or resource.get("filesystem_inputs", 0):
            lines.append(f"{row['arm']} {row['case']['id']}: major_page_faults={resource.get('major_page_faults')}; "
                         f"filesystem_input_blocks={resource.get('filesystem_inputs')}. "
                         "Do not treat cross-arm timing differences as purely compute improvements.")
    payload = {"audited_samples": len(rows), "manifests": manifests, "binaries": binaries, "table": table, "raw_audit": rows}
    if a.output:
        atomic_text(a.output, "\n".join(lines) + "\n\n" + json.dumps(payload, indent=2) + "\n")
        save(a.output.with_suffix(".audit.json"), payload)
    print("\n".join(lines))


if __name__ == "__main__":
    main()
