#!/usr/bin/env python3
"""Frozen native-64 E2E comparison for 256 SHA256, Keccak and Blake3 instances.

Run from a source snapshot root containing source_manifest.json. Each of three
rotating batches runs both systems on each hash in separate processes, with one
warmup and six formal proofs. RAM is the GNU time peak RSS of that whole process,
including offline preparation, warmup, prover and verifier; it is not an online
prover-only peak. Validation processes are excluded from reported timing and RAM.
"""
import argparse
import json
import os
from pathlib import Path
import re
import shutil
import signal
import statistics
import subprocess
import sys
import time

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / "20260916-sha128"))
from run_engineering import COMMON_ENV
from run_column_projection import (
    CPU_SPEC, FEATURES, SHARED_DIGESTS, TOOLCHAIN,
    checked_hash, digest, number, preflight, records, require, save,
)

HASHES = ("sha256", "keccak", "blake3")
SYSTEMS = ("mikros", "flock")
MODES = tuple((kind, system) for kind in HASHES for system in SYSTEMS)
PCS_M = {"sha256": 23, "keccak": 24, "blake3": 22}
SHARED_IDENTITY = SHARED_DIGESTS + ("program_id", "matrix_representation")
SHARED_WORKLOAD = ("unit", "input_bits", "output_bits", "public_word_columns", "public_words",
                   "r1cs_k_log", "r1cs_c_identity")
VERIFY_SCOPE = "complete non-deferred verifier; no witness input; matrix and sigma obligations discharged"
RAM_SCOPE = ("GNU time whole-process maximum resident set size; includes offline fixture/witness preparation, "
             "one warmup, six formal prover/verifier repetitions and retained allocator memory; "
             "excludes separate tamper-validation processes; not an online-only prover peak")


def command(binary, kind, system, validation=False):
    require((kind, system) in MODES, "unknown hash/system")
    result = [binary, "--hash", kind, "--system", system, "--instances", "256", "--skip", "7",
              "--threads", "64", "--reps", "1" if validation else "6",
              "--warmups", "0" if validation else "1"]
    return result + (["--tamper"] if validation else [])


def batch_order(batch):
    require(type(batch) is int and 0 <= batch < 3, "batch must be 0, 1 or 2")
    kinds = HASHES[batch:] + HASHES[:batch]
    return [(kind, system) for kind in kinds
            for system in (SYSTEMS if (batch + HASHES.index(kind)) % 2 == 0 else SYSTEMS[::-1])]


def peak_rss(path):
    value = path.read_text(encoding="utf-8")
    peaks = re.findall(r"^\s*Maximum resident set size \(kbytes\):\s*(\d+)\s*$", value, re.MULTILINE)
    exits = re.findall(r"^\s*Exit status:\s*(\d+)\s*$", value, re.MULTILINE)
    require(len(peaks) == 1 and int(peaks[0]) > 0 and exits == ["0"], f"invalid GNU time report: {path.name}")
    return int(peaks[0])


def checked_tamper_cases(meta, negative, kind, system):
    cases = negative["cases"]
    require(negative["hash"] == kind and negative["system"] == system, "negative-test identity")
    require(type(negative["rejected"]) is int and type(cases) is list and len(cases) == negative["rejected"]
            and all(type(c) is str and c for c in cases) and len(set(cases)) == len(cases), "distinct named rejected tamper cases")
    core = {"commitment", "public_matrix_a", "public_matrix_b"}
    if system == "flock":
        core.update(("zc_round1_ab", "zc_round1_c", "zc_tail", "zc_a", "zc_b", "zc_c", "lc_round",
                     "lc_partial", "lc_terminal", "lc_shape", "wiring_gather", "pcs_evaluation", "pcs_round",
                     "pcs_column", "pcs_claims", "zc_shape", "zc_nonce_shape", "lc_nonce_shape", "wrong_skip_6", "wrong_skip_8"))
    else:
        core.update(f"hadamard_{group}_{field}" for group in range(2) for field in range(4))
        core.update(f"image_{side}_{field}" for side in range(2) for field in range(5))
        core.update(("group_swap", "group_shape", "image_shape", "grinding_shape", "pcs_evaluation", "pair_nonce"))
        blocks = meta["public_io_blocks"]
        require(type(blocks) is list and len(blocks) > 0, "Mikros public interval metadata")
        core.update(f"public_interval_{i}" for i in range(len(blocks)))
    require(core.issubset(cases), "missing core tamper cases")
    public = set(cases) - core
    columns = meta["public_word_columns"]
    require(type(columns) is list and len(columns) > 0 and all(type(c) is int and c >= 0 for c in columns)
            and columns == sorted(set(columns)), "public word-column metadata")
    expected = {(row, col) for row in (0, 127, 128, 255) for col in columns}
    observed = []
    for case in public:
        match = re.fullmatch(r"public_(?:input|output)_row(\d+)_word(\d+)", case)
        require(match is not None, f"unexpected tamper case: {case}")
        observed.append(tuple(map(int, match.groups())))
    require(len(observed) == len(expected) and set(observed) == expected, "public I/O group-boundary coverage")


def checked_records(path, kind, system, validation=False):
    meta, rows = records(path)
    require(rows[0] is meta and sum(r.get("record") == "metadata" for r in rows) == 1, "metadata shape")
    require(meta["hash"] == kind and meta["system"] == system and meta["instances"] == 256, "workload/system")
    require(meta["threads"] == meta["all_core_threads"] == 64 and FEATURES.issubset(meta["target_features"]), "native64 features/pools")
    require(meta["reps"] == (1 if validation else 6) and meta["warmups"] == (0 if validation else 1), "run configuration")
    require(meta["r1cs_c_identity"] is True and type(meta["r1cs_k_log"]) is int and meta["r1cs_k_log"] >= 7, "native public R1CS")
    require(meta["baseline_skip"] == meta["univariate_skip"] == 7, "matched skip7 path")
    require(meta["mikros_kappa"] == 128 and meta["instance_groups"] == 2, "matched packing width")
    require(meta["packed_direct"] == "1", "native packed-direct PCS path")
    representation = meta["matrix_representation"]
    require(representation in ("public_csc", "native_compact_walker"), "unknown matrix representation")
    require((representation == "public_csc" and "CSC" in meta["matrix_authentication"])
            or (kind == "keccak" and representation == "native_compact_walker"
                and "walker" in meta["matrix_authentication"]), "public matrix authentication")
    require(type(meta["program_id"]) is str and bool(meta["program_id"]), "program identity")
    require(meta["zero_knowledge_claim"] is False and meta["verification"] == VERIFY_SCOPE, "complete verification scope")
    pcs = meta["pcs_params"]
    require(pcs["profile"] == "fast" and pcs["log_inv_rate"] == 1
            and pcs["merkle_hash"] == "blake3", "native PCS settings")
    require(type(pcs["m"]) is int and pcs["m"] == PCS_M[kind] and "num_lanes" in pcs, "PCS size/layout")
    require(type(pcs["log_batch_size"]) is int and 0 < pcs["log_batch_size"] < pcs["m"], "native PCS batch size")
    queries, query_grinding, fold_grinding = (meta[key] for key in
        ("pcs_queries", "pcs_query_grinding_bits", "pcs_fold_grinding_bits"))
    require(type(queries) is list and len(queries) > 0 and len(queries) == len(query_grinding) == len(fold_grinding), "PCS schedule")
    require(all(type(q) is int and q > 0 for q in queries) and all(g == 16 for g in query_grinding)
            and all(g == 0 for g in fold_grinding), "PCS soundness configuration")
    for key in SHARED_DIGESTS:
        checked_hash(meta[key], key)
    samples = [r for r in rows if r.get("record") == "sample"]
    negatives = [r for r in rows if r.get("record") == "negative_tests"]
    require(len(rows) == 1 + len(samples) + len(negatives), "unexpected records")
    require(len(samples) == (1 if validation else 7) and [r["iteration"] for r in samples] == list(range(len(samples))), "iterations")
    require(len(negatives) == int(validation), "negative record count")
    for row in samples:
        require(row["system"] == system and row["hash"] == kind and type(row["warmup"]) is bool
                and row["warmup"] == (not validation and row["iteration"] == 0), "sample flags")
        require(row["verified"] is True and row["transcript_match"] is True, "complete verifier/FS failed")
        require(type(row["proof_bytes"]) is int and row["proof_bytes"] > 0, "proof bytes")
        for key in ("proof_blake3", "commitment_blake3"):
            checked_hash(row[key], key)
        for key in ("prover_ms", "verify_ms"):
            number(row[key], key, True)
    for negative in negatives:
        checked_tamper_cases(meta, negative, kind, system)
    return meta, samples, negatives


def pcs_semantics(meta):
    return {"params": {k: v for k, v in meta["pcs_params"].items() if k != "num_lanes"},
            **{k: meta[k] for k in ("pcs_queries", "pcs_query_grinding_bits", "pcs_fold_grinding_bits")}}


def audit(logs):
    result = {"instances": 256, "threads": 64, "flock_skip": 7, "batches": 3,
              "warmups_per_batch": 1, "formal_repetitions": 18, "ram_scope": RAM_SCOPE, "hashes": {}}
    for kind in HASHES:
        shared, pcs, workload = None, None, None
        entry = {"systems": {}, "negative_tests": {}, "same_statement": True, "same_pcs_semantics": True}
        for system in SYSTEMS:
            metadata, samples, peaks = [], [], []
            for batch in range(3):
                name = f"{kind}-{system}-{batch}"
                meta, rows, _ = checked_records(logs / f"{name}.jsonl", kind, system)
                identity = {key: meta[key] for key in SHARED_IDENTITY}
                shared, pcs = shared or identity, pcs or pcs_semantics(meta)
                require(identity == shared and pcs_semantics(meta) == pcs, f"statement/PCS differs: {kind}")
                shape = {key: meta[key] for key in SHARED_WORKLOAD}
                workload = workload or shape
                require(shape == workload, f"workload metadata differs: {kind}")
                metadata.append(meta)
                samples.extend(rows)
                peaks.append(peak_rss(logs / f"{name}.resource.txt"))
            proof_keys = ("proof_blake3", "proof_bytes", "commitment_blake3")
            require(all(len({row[key] for row in samples}) == 1 for key in proof_keys), "proof/commitment unstable")
            formal = [row for row in samples if not row["warmup"]]
            require(len(formal) == 18, "formal sample count")
            summary = {key: statistics.median(row[key] for row in formal) for key in ("prover_ms", "verify_ms")}
            summary.update(samples=18, warmup_samples=3, metadata=metadata,
                mean_prover_ms=statistics.mean(row["prover_ms"] for row in formal),
                prover_range_ms=[min(row["prover_ms"] for row in formal), max(row["prover_ms"] for row in formal)],
                verify_range_ms=[min(row["verify_ms"] for row in formal), max(row["verify_ms"] for row in formal)],
                process_peak_rss_kib=peaks, process_peak_rss_median_kib=statistics.median(peaks),
                process_peak_rss_max_kib=max(peaks), **{key: formal[0][key] for key in proof_keys})
            entry["systems"][system] = summary
            meta, validation, negative = checked_records(logs / f"validation-{kind}-{system}.jsonl", kind, system, True)
            require({key: meta[key] for key in SHARED_IDENTITY} == shared and pcs_semantics(meta) == pcs, "validation statement/PCS differs")
            require({key: meta[key] for key in SHARED_WORKLOAD} == workload, "validation workload differs")
            require(all(validation[0][key] == formal[0][key] for key in proof_keys), "validation proof differs")
            entry["negative_tests"][system] = negative[0]
        entry["shared_statement_digests"] = {key: shared[key] for key in SHARED_DIGESTS}
        entry["program_id"] = shared["program_id"]
        entry["matrix_representation"] = shared["matrix_representation"]
        entry["workload"] = workload
        entry["pcs_semantics"] = pcs
        entry["flock_over_mikros"] = entry["systems"]["flock"]["prover_ms"] / entry["systems"]["mikros"]["prover_ms"]
        result["hashes"][kind] = entry
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target", required=True, type=Path)
    parser.add_argument("--logs", type=Path, default=Path("results-hashes"))
    parser.add_argument("--skip-build", action="store_true")
    args = parser.parse_args()
    root, target = Path.cwd().resolve(), args.target.resolve()
    logs = (root / args.logs).resolve()
    require(logs.is_relative_to(root), "logs must be in snapshot")
    manifest = root / "source_manifest.json"
    sources = json.loads(manifest.read_text(encoding="utf-8"))["files_sha256"]
    require(len(sources) >= 400 and Path(__file__).resolve().relative_to(root).as_posix() in sources, "runner absent from source receipt")
    def verify_sources():
        for name, expected in sources.items():
            path = (root / name).resolve()
            require(path.is_relative_to(root) and path.is_file() and digest(path) == expected, f"source mismatch: {name}")
    verify_sources()
    logs.mkdir(exist_ok=False)
    env = {k: v for k, v in os.environ.items() if not k.startswith(("FLOCK_", "MIKROS_", "E2E_", "LIG_", "RAYON_"))
           and k not in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_RUSTFLAGS", "RUSTC_WRAPPER",
                         "RUSTC_WORKSPACE_WRAPPER", "PCS_TRACE", "LD_PRELOAD")}
    env.update(COMMON_ENV, CARGO_TARGET_DIR=str(target), LC_ALL="C")
    env["PATH"] = str(Path.home() / ".cargo/bin") + ":" + env.get("PATH", "")
    relevant_env = {k: v for k, v in env.items() if k.startswith(("FLOCK_", "MIKROS_", "RAYON_"))
                    or k in ("RUSTFLAGS", "CARGO_TARGET_DIR", "LC_ALL")}
    status = {"state": "starting", "completed": []}
    def run(name, invocation, build=False):
        status["state"] = name
        save(logs / "status.json", status)
        if not build:
            save(logs / f"{name}.preflight.json", preflight())
            invocation = ["/usr/bin/time", "-v", "-o", str(logs / f"{name}.resource.txt"), "taskset", "-c", CPU_SPEC] + invocation
        save(logs / f"{name}.command.json", invocation)
        save(logs / f"{name}.environment.json", relevant_env)
        started = time.monotonic()
        with (logs / f"{name}.jsonl").open("w") as out, (logs / f"{name}.stderr.log").open("w") as err:
            process = subprocess.Popen(invocation, cwd=root, env=env, stdout=out, stderr=err, start_new_session=True)
            try:
                process.wait(timeout=1800 if build else 900)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
        event = {"name": name, "exit_code": process.returncode, "wall_seconds": time.monotonic() - started}
        if process.returncode == 0 and not build:
            event["process_peak_rss_kib"] = peak_rss(logs / f"{name}.resource.txt")
        status["completed"].append(event)
        save(logs / "status.json", status)
        print(json.dumps(event), flush=True)
        require(process.returncode == 0, f"command failed: {name}")
    try:
        save(logs / "hardware.json", {"cpuinfo": Path("/proc/cpuinfo").read_text(), "meminfo": Path("/proc/meminfo").read_text(),
             "affinity": CPU_SPEC, "utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), "environment": relevant_env})
        save(logs / "preflight.json", preflight())
        order = [batch_order(batch) for batch in range(3)]
        save(logs / "run_plan.json", {"modes": MODES, "order": order, "reps": 6, "warmups": 1, "batches": 3,
             "profiling_enabled": False, "ram_scope": RAM_SCOPE, "source_files": len(sources),
             "source_manifest_sha256": digest(manifest), "mikros_uses_compiled_defaults": True})
        built = target / "release/examples/e2e_hash_compare"
        receipt_path = root / "hashes-build.json"
        if args.skip_build:
            receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
            require(receipt["source_manifest_sha256"] == digest(manifest) and receipt["toolchain"] == TOOLCHAIN
                    and receipt["rustflags"] == COMMON_ENV["RUSTFLAGS"], "build receipt mismatch")
            require(receipt["binary_sha256"] == {str(built): digest(built)}, "build bytes mismatch")
        else:
            for name in sources:
                os.utime(root / name, None)
            run("build", ["cargo", TOOLCHAIN, "build", "--release", "--offline", "--locked", "-j8", "-p", "mikros",
                          "--example", "e2e_hash_compare"], True)
            receipt = {"source_manifest_sha256": digest(manifest), "toolchain": TOOLCHAIN, "rustflags": COMMON_ENV["RUSTFLAGS"],
                       "binary_sha256": {str(built): digest(built)}}
            save(receipt_path, receipt)
        save(logs / "build-receipt.json", receipt)
        binary_dir = logs / "evidence-binaries"
        binary_dir.mkdir()
        binary = binary_dir / built.name
        shutil.copy2(built, binary)
        require(digest(binary) == receipt["binary_sha256"][str(built)], "copied executable differs")
        save(logs / "binary-sha256.json", {binary.relative_to(logs).as_posix(): digest(binary)})
        for kind, system in MODES:
            name = f"validation-{kind}-{system}"
            run(name, command(str(binary), kind, system, True))
            checked_records(logs / f"{name}.jsonl", kind, system, True)
        for batch, sequence in enumerate(order):
            for kind, system in sequence:
                name = f"{kind}-{system}-{batch}"
                run(name, command(str(binary), kind, system))
                checked_records(logs / f"{name}.jsonl", kind, system)
        save(logs / "summary.json", audit(logs))
        verify_sources()
        require(digest(binary) == digest(built) == receipt["binary_sha256"][str(built)], "executables changed during run")
        status["state"] = "complete"
    except Exception as error:
        status.update(state="failed", error=str(error))
        raise
    finally:
        save(logs / "status.json", status)
        save(logs / "evidence_sha256.json", {p.relative_to(logs).as_posix(): digest(p) for p in sorted(logs.rglob("*"))
             if p.is_file() and p.name != "evidence_sha256.json"})


if __name__ == "__main__":
    main()
