#!/usr/bin/env python3
"""Independently audit the public-CSC, native-C=I 256-SHA server evidence.

Usage:
  python verify_public_csc_evidence.py SNAPSHOT \
      --evidence-archive SNAPSHOT/evidence.tar.gz --evidence-sha256 SHA256 \
      --binary-dir SNAPSHOT/evidence/evidence-binaries --output SNAPSHOT/local_audit.json

The positional path can be a snapshot or its results-public-csc256 directory.
Source and evidence archives are read without extraction. The reviewed source
archive is pinned by default; an explicit source hash and a neighboring sidecar
must agree. This standalone verifier never imports or executes server audit code.
Actual executables are independently rehashed when --binary-dir is supplied.
Self-reported hashes and successful log records alone do not cryptographically
prove execution provenance or independently replay proof verification.
"""

import argparse
import ast
import hashlib
import json
import math
from pathlib import Path, PurePosixPath
import re
import statistics
import sys
import tarfile


CPU_SPEC = "0-47,48-55,72-79"
FEATURES = {"avx512f", "avx512bw", "gfni", "vpclmulqdq"}
TOOLCHAIN = "+nightly-2026-03-21"
SHA256 = re.compile(r"[0-9a-f]{64}\Z")
SOURCE_SHA256 = "350e97cbd0f6cff74adc6231912a18be6d8cfedeaee2ce484c453327c65911c3"
PHASES = ("commit_ms", "reduction_f128_ms", "public_io_ms", "pair_reduction_ms", "pcs_open_ms")
TEST_COUNTS = {"test-paired-pcs": 2, "test-sha": 16}
RUNNER_SOURCE = "mikros/server_experiments/20260916-sha128/run_public_csc.py"
BASE_ENV = {
    "RUSTFLAGS": "-C target-cpu=native", "RAYON_NUM_THREADS": "64",
    "FLOCK_PCS_PACKED_DIRECT": "1", "FLOCK_WP_UNINIT": "1",
}
REQUIRED_SOURCE = {
    "Cargo.toml", "Cargo.lock", "mikros/examples/e2e_common/public_csc_sha.rs",
    "mikros/examples/e2e_common/mod.rs", "mikros/examples/e2e_sha256_paired.rs",
    "mikros/examples/e2e_flock_baseline.rs", "crates/flock-prover/src/r1cs_hashes/sha2.rs",
    "crates/flock-core/src/pcs/paired.rs", "crates/flock-core/src/pcs/merged_tensor.rs",
    "crates/flock-core/configs/ligerito/m23_fast.toml", RUNNER_SOURCE,
    "mikros/server_experiments/20260916-sha128/run_server.py",
}
MATCHING_DIGESTS = (
    "fixture_digest_blake3", "circuit_digest_blake3", "r1cs_digest_blake3", "prepared_witness_blake3",
)
PCS_PARAMS = {
    "log_batch_size": 6, "log_inv_rate": 1, "m": 23, "merkle_hash": "blake3", "profile": "fast",
}
BINARY_NAMES = {"e2e_sha256_paired", "e2e_flock_baseline"}



class AuditError(ValueError):
    pass


def need(condition, message):
    if not condition:
        raise AuditError(message)


def digest(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def json_file(path):
    return json_value(path.read_text(encoding="utf-8-sig"))


def json_value(text):
    def unique_pairs(pairs):
        value = {}
        for key, item in pairs:
            need(key not in value, f"duplicate JSON key: {key}")
            value[key] = item
        return value

    def invalid_constant(value):
        raise AuditError(f"invalid JSON numeric constant: {value}")

    return json.loads(text, object_pairs_hook=unique_pairs, parse_constant=invalid_constant)


def checked_hash(value, label):
    need(isinstance(value, str) and SHA256.fullmatch(value), f"invalid {label}")
    return value


def number(value, label, positive=False):
    need(type(value) in (int, float) and math.isfinite(value), f"nonfinite/non-numeric {label}")
    need(value > 0 if positive else value >= 0, f"negative/zero {label}")
    return value


def integer(value, label, positive=False):
    need(type(value) is int and value >= (1 if positive else 0), f"invalid integer {label}")
    return value


def safe_name(name):
    need(isinstance(name, str) and name and "\\" not in name, "invalid archive/manifest path")
    path = PurePosixPath(name)
    need(not path.is_absolute() and ".." not in path.parts and ":" not in name,
         f"unsafe archive/manifest path: {name}")
    need(path.as_posix() == name, f"non-canonical archive/manifest path: {name}")
    return path


def runner_profile(source):
    """Read the frozen runner's test/build declarations without executing it."""
    tree = ast.parse(source, filename=RUNNER_SOURCE)
    mains = [n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name == "main"]
    need(len(mains) == 1, "source runner must have one main function")
    calls = sorted((n for n in ast.walk(mains[0]) if isinstance(n, ast.Call)
                    and isinstance(n.func, ast.Name) and n.func.id == "run"), key=lambda n: n.lineno)
    literals = [n for n in calls if n.args and isinstance(n.args[0], ast.Constant)]
    expected_calls = [
        'run("build", cargo + ["build"] + flags + ["-p", "mikros", "--example", "e2e_sha256_paired", "--example", "e2e_flock_baseline"], False)',
        'run("test-paired-pcs", cargo + ["test"] + flags + ["-p", "flock-core", "pcs::paired::tests", "--", "--test-threads=1"])',
        'run("test-sha", cargo + ["test"] + flags + ["-p", "mikros", "--example", "e2e_sha256_paired", "--", "--test-threads=1"])',
        'run("validation", [binaries["mikros"], "--opening", "both", "--threads", "64", "--reps", "1", "--warmups", "0", "--tamper"])',
    ]
    need([ast.dump(n) for n in literals] == [ast.dump(ast.parse(c, mode="eval").body) for c in expected_calls],
         "source runner build/test/validation plan differs")
    dynamic = [n for n in calls if n not in literals]
    expected_dynamic = ast.parse('run(f"{name}-{batch}", [binaries[name]] + extra + ["--threads", "64", "--reps", "6", "--warmups", "1"])', mode="eval").body
    need(len(dynamic) == 1 and ast.dump(dynamic[0]) == ast.dump(expected_dynamic), "source formal invocation differs")
    updates = [n for n in ast.walk(mains[0]) if isinstance(n, ast.Call)
               and isinstance(n.func, ast.Attribute) and n.func.attr == "update"
               and isinstance(n.func.value, ast.Name) and n.func.value.id == "env"]
    need(len(updates) == 1, "source environment setup differs")
    values = {kw.arg: ast.literal_eval(kw.value) for kw in updates[0].keywords if kw.arg in BASE_ENV}
    need(values == BASE_ENV, "source controlled environment differs")
    need({kw.arg for kw in updates[0].keywords} == set(BASE_ENV) | {"CARGO_TARGET_DIR"}, "unexpected source environment override")
    return {"version": "public-csc-v1", "tests_passed": TEST_COUNTS,
            "tests_ignored": {k: 0 for k in TEST_COUNTS}, "controlled_environment": BASE_ENV,
            "build_examples": ["e2e_sha256_paired", "e2e_flock_baseline"]}


def evidence_files(logs):
    hashes = json_file(logs / "evidence_sha256.json")
    need(isinstance(hashes, dict) and hashes, "empty evidence hash manifest")
    actual = {p.name for p in logs.iterdir() if p.is_file() and p.name != "evidence_sha256.json"}
    need(actual == set(hashes), "evidence file set differs from manifest")
    for name, expected in hashes.items():
        need(len(safe_name(name).parts) == 1, f"nested evidence entry: {name}")
        path = logs / name
        need(not path.is_symlink() and path.is_file(), f"nonregular evidence: {name}")
        need(digest(path) == checked_hash(expected, name), f"evidence hash mismatch: {name}")
    return len(hashes)


def source_archive(archive, explicit_hash=None):
    actual_hash = digest(archive)
    sidecar = archive.with_name(archive.name + ".sha256")
    expected = explicit_hash or SOURCE_SHA256
    if sidecar.is_file():
        recorded = sidecar.read_text(encoding="utf-8-sig").split()[0]
        checked_hash(recorded, "source sidecar hash")
        need(actual_hash == recorded, "source archive differs from its checksum sidecar")
        if expected is None:
            expected = recorded
    if expected is not None:
        need(actual_hash == checked_hash(expected, "source archive hash"), "source archive SHA-256 mismatch")
    with tarfile.open(archive, "r:*") as tar:
        members = {}
        for member in tar.getmembers():
            safe_name(member.name)
            need(member.isfile(), f"nonregular source tar member: {member.name}")
            need(member.name not in members, f"duplicate source tar member: {member.name}")
            members[member.name] = member
        need("source_manifest.json" in members, "source manifest missing from archive")
        with tar.extractfile(members["source_manifest.json"]) as stream:
            manifest = json_value(stream.read().decode("utf-8-sig"))
        files = manifest["files_sha256"]
        need(isinstance(files, dict) and REQUIRED_SOURCE.issubset(files), "public-CSC implementation missing from source manifest")
        need(set(members) == set(files) | {"source_manifest.json"}, "source archive/manifest member mismatch")
        need(re.fullmatch(r"[0-9a-f]{40}", manifest["base_revision"]), "invalid base revision")
        for name, expected in files.items():
            safe_name(name)
            with tar.extractfile(members[name]) as stream:
                actual = hashlib.file_digest(stream, "sha256").hexdigest()
            need(actual == checked_hash(expected, name), f"source member hash mismatch: {name}")
        with tar.extractfile(members[RUNNER_SOURCE]) as stream:
            profile = runner_profile(stream.read().decode("utf-8-sig"))
        with tar.extractfile(members["mikros/examples/e2e_sha256_paired.rs"]) as stream:
            entrypoint = stream.read().decode("utf-8-sig")
        need('#[path = "e2e_common/public_csc_sha.rs"]' in entrypoint
             and "public_csc_sha::main()" in entrypoint, "paired entrypoint does not use public CSC")
    local_manifest = archive.parent / "source_manifest.json"
    if local_manifest.is_file():
        need(json_file(local_manifest) == manifest, "local and archived source manifests differ")
    return {
        "archive_sha256": actual_hash, "checksum_pin_verified": expected is not None,
        "manifest_files_verified": len(files), "base_revision": manifest["base_revision"],
        "critical_sources_sha256": {name: files[name] for name in sorted(REQUIRED_SOURCE)},
        "runner_profile": profile,
    }


def verify_evidence_archive(archive, expected, logs, binary_dir=None):
    need(expected is not None, "--evidence-archive requires --evidence-sha256")
    actual = digest(archive)
    need(actual == checked_hash(expected, "evidence archive hash"), "evidence archive SHA-256 mismatch")
    seen, binary_seen = set(), set()
    with tarfile.open(archive, "r:*") as tar:
        for member in tar.getmembers():
            # tar may prefix directory entries with ./; normalize that spelling.
            name = member.name.removeprefix("./").rstrip("/")
            if not name:
                need(member.isdir(), "empty file path in evidence archive")
                continue
            path = safe_name(name)
            need(member.isfile() or member.isdir(), f"nonregular evidence tar member: {name}")
            if member.isdir():
                continue
            parts = path.parts
            if len(parts) == 2 and parts[0] in ("binaries", "evidence-binaries"):
                need(binary_dir is not None, "archive has binaries; pass --binary-dir for byte verification")
                need(parts[0] == binary_dir.name, "archived and supplied binary directory names differ")
                name = parts[1]
                need(name in ("e2e_sha256_paired", "e2e_flock_baseline"), "unexpected archived binary")
                need(name not in binary_seen, f"duplicate archived binary: {name}")
                binary_seen.add(name)
                local = binary_dir / name
                need(local.is_file() and not local.is_symlink(), f"extracted binary missing: {name}")
                with tar.extractfile(member) as stream:
                    member_hash = hashlib.file_digest(stream, "sha256").hexdigest()
                need(member_hash == digest(local), f"archive/extracted binary bytes differ: {name}")
                continue
            if len(parts) == 2 and parts[0] == logs.name:
                name = parts[1]
            elif len(parts) == 1:
                name = parts[0]
            else:
                raise AuditError(f"unexpected evidence archive member: {path}")
            need(name not in seen and (logs / name).is_file(), f"duplicate/missing extracted evidence: {name}")
            seen.add(name)
            with tar.extractfile(member) as stream:
                member_hash = hashlib.file_digest(stream, "sha256").hexdigest()
            need(member_hash == digest(logs / name), f"archive/extracted bytes differ: {name}")
    need(seen == {p.name for p in logs.iterdir() if p.is_file()}, "archive/extracted file sets differ")
    if binary_seen:
        need(binary_seen == {"e2e_sha256_paired", "e2e_flock_baseline"}, "incomplete archived binary set")
    return actual


def records(path):
    rows = [json_value(line) for line in path.read_text(encoding="utf-8-sig").splitlines() if line.strip()]
    need(all(isinstance(r, dict) for r in rows), f"non-object record: {path.name}")
    metadata = [r for r in rows if r.get("record") == "metadata"]
    need(len(metadata) == 1 and rows[0] is metadata[0], f"metadata shape: {path.name}")
    need(all(r.get("record") in ("metadata", "sample", "negative_tests") for r in rows), f"unexpected record: {path.name}")
    return metadata[0], rows


def validate_metadata(meta, system):
    need(meta["schema_version"] == 1, "metadata schema differs")
    need(meta["sha_compressions"] == 256 and meta["fixture"] == "sha256-independent-compressions", "wrong SHA workload")
    need(meta["threads"] == meta["all_core_threads"] == 64, "wrong thread pools")
    need(isinstance(meta["target_features"], list) and FEATURES.issubset(meta["target_features"]), "required native CPU features missing")
    for key in MATCHING_DIGESTS:
        checked_hash(meta[key], key)
    need(meta["r1cs_c_identity"] is True and meta["r1cs_k_log"] == 15 and meta["native_k_skip"] == 6, "native R1CS identity/dimensions differ")
    need("public CSC" in meta["matrix_authentication"], "matrix authentication is not public CSC")
    params = dict(PCS_PARAMS, num_lanes=None if system == "mikros" else 50)
    need(meta["pcs_params"] == params, "native PCS parameters/layout differ")
    need(meta["pcs_queries"] == [244, 79, 48], "native PCS queries differ")
    if system == "mikros":
        for key, value in {"reduction_field_bits": 128, "opening_logical_field_bits": 256,
                           "instance_groups": 2, "instances_per_group": 128, "pcs_carrier_field_bits": 128,
                           "carrier_words": 65536, "logical_f256_words": 32768, "carrier_bytes": 1048576}.items():
            need(type(meta[key]) is int and meta[key] == value, f"Mikros representation differs: {key}")
        need(meta["zero_knowledge"] is False and meta["packed_direct"] == "1", "Mikros proof/PCS mode differs")
        need(meta["matrix_authentication"] == "public CSC; complete verifier checks both matrix terminals; no matrix/read PCS", "Mikros matrix obligations differ")
        for key in ("offline_witness_ms", "offline_transpose_ms"):
            number(meta[key], key)
    else:
        need(meta["system"] == "flock-circuit-e2e" and meta["architecture"] == "x86_64", "wrong native system")
        need(meta["independent_compressions"] is True and meta["public_output_count"] == meta["gate_count"] == 256, "native SHA statement differs")
        need(meta["reps"] == 6 and meta["warmups"] == 1 and meta["zero_knowledge_claim"] is False, "native run mode differs")
        need(meta["verification"] == "complete non-deferred verifier; no witness input; matrix and sigma obligations discharged", "native verifier scope differs")
        need(meta["pcs_query_grinding_bits"] == [16, 16, 16] and meta["pcs_fold_grinding_bits"] == [0, 0, 0], "native PCS grinding differs")
        checked_hash(meta["pcs_config_blake3"], "PCS configuration hash")
        for key in ("offline_fixture_ms", "offline_witness_ms"):
            number(meta[key], key)


def validate_sample(row, system):
    need(row["verified"] is True and row["transcript_match"] is True, "unverified sample or Fiat-Shamir mismatch")
    need(type(row["warmup"]) is bool, "invalid warmup flag")
    integer(row["iteration"], "iteration")
    integer(row["proof_bytes"], "proof bytes", True)
    for key in ("proof_blake3", "commitment_blake3"):
        checked_hash(row[key], key)
    number(row["verify_ms"], "verify_ms", True)
    if system == "flock":
        need(row["system"] == "flock-circuit-e2e", "wrong native FLOCK sample")
        number(row["prove_with_commit_ms"], "native prover time", True)
        return
    need(row["system"] == "mikros" and row["opening"] in ("paired", "separate"), "wrong Mikros sample")
    checked_hash(row["piop_blake3"], "F128 PIOP hash")
    times = row["timings"]
    number(times["prover_ms"], "Mikros prover time", True)
    for phase in PHASES:
        number(times[phase], phase)
    need(sum(times[p] for p in PHASES) <= times["prover_ms"] + 1e-5, "phase sum exceeds timed prover scope")
    for key, expected in {"source_claims": 5 if row["opening"] == "paired" else 8,
                          "ring_claims": 2, "inner_openings": 1}.items():
        need(type(times[key]) is int and times[key] == expected, f"native PCS claim count differs: {key}")


def recompute_summary(logs):
    groups = {k: [] for k in ("paired", "separate", "flock")}
    all_groups = {k: [] for k in groups}
    metadata = {"mikros": [], "flock": []}
    for batch in range(3):
        for system in ("mikros", "flock"):
            meta, rows = records(logs / f"{system}-{batch}.jsonl")
            validate_metadata(meta, system)
            metadata[system].append(meta)
            samples = [r for r in rows if r["record"] == "sample"]
            need(len(rows) == len(samples) + 1, "unexpected records in formal run")
            modes = ("paired", "separate") if system == "mikros" else ("flock",)
            need(len(samples) == 7 * len(modes), "wrong formal/warmup sample count")
            for mode in modes:
                selected = [r for r in samples if system == "flock" or r["opening"] == mode]
                need(sorted(r["iteration"] for r in selected) == list(range(7)), "duplicate/missing iteration")
                for row in selected:
                    validate_sample(row, system)
                    need(row["warmup"] == (row["iteration"] == 0), "warmup/iteration mismatch")
                    all_groups[mode].append(row)
                    if not row["warmup"]:
                        groups[mode].append(row)
    reference = metadata["mikros"][0]
    for meta in metadata["mikros"] + metadata["flock"]:
        for key in MATCHING_DIGESTS:
            need(meta[key] == reference[key], f"systems use different statement/R1CS/prepared witness: {key}")
    for system in ("mikros", "flock"):
        first = metadata[system][0]
        varying = {"offline_fixture_ms", "offline_witness_ms", "offline_transpose_ms"}
        for meta in metadata[system]:
            need({k: v for k, v in meta.items() if k not in varying} ==
                 {k: v for k, v in first.items() if k not in varying}, f"{system} metadata drifts across batches")
    validation_meta, rows = records(logs / "validation.jsonl")
    validate_metadata(validation_meta, "mikros")
    varying = {"offline_witness_ms", "offline_transpose_ms"}
    need({k: v for k, v in validation_meta.items() if k not in varying} ==
         {k: v for k, v in reference.items() if k not in varying}, "validation uses different Mikros metadata")
    checked = [r for r in rows if r["record"] == "sample"]
    negatives = [r for r in rows if r["record"] == "negative_tests"]
    need(len(checked) == 2 and {r["opening"] for r in checked} == {"paired", "separate"}, "validation samples missing")
    need(len(negatives) == 2 and {r["opening"] for r in negatives} == {"paired", "separate"}, "negative tests missing")
    need(len(rows) == 5, "unexpected validation record count")
    for row in checked:
        validate_sample(row, "mikros")
        need(row["iteration"] == 0 and row["warmup"] is False, "wrong validation iteration")
        all_groups[row["opening"]].append(row)
    for row in negatives:
        integer(row["rejected"], "rejected tamper cases", True)
        need(row["rejected"] == (23 if row["opening"] == "paired" else 22), "tamper suite count differs")
    mikros_rows = all_groups["paired"] + all_groups["separate"]
    need(len({r["piop_blake3"] for r in mikros_rows}) == 1, "pairing changed F128 reduction")
    need(len({r["commitment_blake3"] for r in mikros_rows}) == 1, "Mikros commitment changed")
    result = {
        "instances": 256, "threads": 64, "metadata": metadata,
        "same_f128_reduction": True, "same_mikros_commitment": True,
        "same_sha_public_statement": True, "native_pcs_backend": "FLOCK Ligerito Fast",
        "pcs_layouts_differ": True, "same_r1cs_and_native_prepared_witness": True, "formal_repetitions": 18,
        "matrix_authentication": "public CSC on both sides; native C=I",
    }
    for key, samples in groups.items():
        need(len(samples) == 18, f"expected 18 formal {key} samples")
        for field in ("proof_blake3", "proof_bytes", "commitment_blake3"):
            need(len({r[field] for r in all_groups[key]}) == 1, f"{key} reproducibility: {field}")
        times = samples if key == "flock" else [r["timings"] for r in samples]
        keys = ("prove_with_commit_ms",) if key == "flock" else ("prover_ms",) + PHASES
        row = {field: statistics.median(t[field] for t in times) for field in keys}
        row.update(
            prover_range_ms=[min(t[keys[0]] for t in times), max(t[keys[0]] for t in times)],
            proof_bytes=samples[0]["proof_bytes"], proof_blake3=samples[0]["proof_blake3"],
            verify_ms=statistics.median(r["verify_ms"] for r in samples),
        )
        if key != "flock":
            row.update(source_claims=times[0]["source_claims"], ring_claims=2, inner_openings=1)
        result[key] = row
    result["pairing_speedup"] = result["separate"]["prover_ms"] / result["paired"]["prover_ms"]
    result["flock_over_paired"] = result["flock"]["prove_with_commit_ms"] / result["paired"]["prover_ms"]
    result["negative_tests"] = negatives
    need(result == json_file(logs / "summary.json"), "independently recomputed summary differs from server summary")
    return result


def verify_hardware(hardware):
    need(hardware["affinity"] == CPU_SPEC, "wrong hardware affinity")
    selected = set(range(56)) | set(range(72, 80))
    cpus = {}
    for block in hardware["cpuinfo"].strip().split("\n\n"):
        fields = dict(line.split(":", 1) for line in block.splitlines() if ":" in line)
        fields = {k.strip(): v.strip() for k, v in fields.items()}
        index = int(fields["processor"])
        need(index not in cpus, "duplicate hardware CPU record")
        cpus[index] = fields
    need(len(selected) == 64 and selected.issubset(cpus), "missing selected CPU hardware records")
    for index in selected:
        need(FEATURES.issubset(cpus[index]["flags"].split()), f"CPU {index} lacks native target features")
        need(cpus[index]["model name"] == "Intel(R) Xeon(R) Gold 6342 CPU @ 2.80GHz", "unexpected server CPU model")
    mem = re.search(r"^MemAvailable:\s+(\d+)\s+kB$", hardware["meminfo"], re.M)
    need(mem is not None and int(mem[1]) > 8 * 1024 * 1024, "hardware available memory below threshold")
    need(re.fullmatch(r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z", hardware["utc"]), "invalid hardware UTC timestamp")
    return {"selected_logical_cpus": len(selected), "visible_logical_cpus": len(cpus),
            "required_native_features": sorted(FEATURES), "cpu_model": cpus[0]["model name"]}


def verify_execution(logs, profile, binary_dir=None):
    status = json_file(logs / "status.json")
    need(status["state"] == "complete" and "error" not in status, "server run incomplete/failed")
    events = status["completed"]
    names = [e["name"] for e in events]
    expected = ["build", *profile["tests_passed"], "validation", "mikros-0", "flock-0", "flock-1", "mikros-1", "mikros-2", "flock-2"]
    need(names == expected, "completed stages/order differ from benchmark plan")
    hardware = json_file(logs / "hardware.json")
    hardware_result = verify_hardware(hardware)
    env = hardware["environment"]
    controlled = {k: v for k, v in env.items() if k.startswith(("FLOCK_", "MIKROS_")) or k in BASE_ENV}
    need(controlled == profile["controlled_environment"], "hardware environment differs from source runner profile")
    for stem in ["preflight", *(f"{s}-{b}.preflight" for b in range(3) for s in ("mikros", "flock"))]:
        data = json_file(logs / f"{stem}.json")
        busy = number(data["selected_cpu_busy_fraction"], "CPU busy fraction")
        need(busy < .10 and data["mem_available_kib"] > 8 * 1024 * 1024, "preflight resource limit failed")
    binaries = json_file(logs / "binary-sha256.json")
    need(len(binaries) == 2, "expected exactly two binary hashes")
    by_name = {}
    for path, expected_hash in binaries.items():
        name = PurePosixPath(path).name
        need(name in ("e2e_sha256_paired", "e2e_flock_baseline") and name not in by_name, "unexpected benchmark binary")
        checked_hash(expected_hash, "binary hash")
        by_name[name] = path
        if binary_dir is not None:
            local = binary_dir / name
            need(local.is_file() and not local.is_symlink(), f"downloaded binary missing: {name}")
            need(digest(local) == expected_hash, f"downloaded binary hash mismatch: {name}")
    tests, ignored_tests, rss = {}, {}, {}
    for event in events:
        name = event["name"]
        need(event["exit_code"] == 0, f"stage failed: {name}")
        number(event["wall_seconds"], f"{name} wall seconds")
        command = json_file(logs / f"{name}.command.json")
        need(isinstance(command, list) and all(isinstance(s, str) for s in command), f"invalid command: {name}")
        if name != "build":
            need(command[:3] == ["/usr/bin/time", "-v", "-o"] and len(command) > 7, f"missing resource timer: {name}")
            need(PurePosixPath(command[3]).name == f"{name}.resource.txt" and command[4:7] == ["taskset", "-c", CPU_SPEC], f"wrong pinning: {name}")
            command = command[7:]
            resource = (logs / f"{name}.resource.txt").read_text(encoding="utf-8")
            need(re.search(r"^\s*Exit status: 0\s*$", resource, re.M), f"resource exit status: {name}")
            match = re.search(r"Maximum resident set size \(kbytes\): (\d+)", resource)
            need(match is not None, f"RSS missing: {name}")
            rss[name] = int(match[1])
        if name == "build" or name.startswith("test-"):
            expected_command = ["cargo", TOOLCHAIN, "build" if name == "build" else "test",
                                "--release", "--offline", "--locked", "-j8", "-p"]
            if name == "build":
                expected_command += ["mikros"] + [token for example in profile["build_examples"] for token in ("--example", example)]
            elif name == "test-paired-pcs":
                expected_command += ["flock-core", "pcs::paired::tests", "--", "--test-threads=1"]
            elif name == "test-sha":
                expected_command += ["mikros", "--example", "e2e_sha256_paired", "--", "--test-threads=1"]
            else:
                raise AuditError(f"unexpected test stage: {name}")
            need(command == expected_command, f"uncontrolled cargo command: {name}")
            if name.startswith("test-"):
                need(command[2] == "test" and command[-2:] == ["--", "--test-threads=1"], f"wrong test command: {name}")
                text = (logs / f"{name}.jsonl").read_text(encoding="utf-8")
                matches = re.findall(r"test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out", text)
                need(matches and all(m[0] == "ok" and int(m[2]) == 0 and int(m[4]) == 0 for m in matches), f"tests failed/measured: {name}")
                tests[name] = sum(int(m[1]) for m in matches)
                ignored_tests[name] = sum(int(m[3]) for m in matches)
                need(tests[name] == profile["tests_passed"][name], f"test filter count differs from source runner version: {name}")
                need(ignored_tests[name] == profile["tests_ignored"][name], f"ignored test count differs from source runner version: {name}")
        else:
            system = "flock" if name.startswith("flock-") else "mikros"
            binary = "e2e_flock_baseline" if system == "flock" else "e2e_sha256_paired"
            tail = ["--opening", "both"] if system == "mikros" else ["--compressions", "256", "--profile", "fast"]
            tail += ["--threads", "64", "--reps", "1" if name == "validation" else "6", "--warmups", "0" if name == "validation" else "1"]
            if name == "validation":
                tail.append("--tamper")
            need(command == [by_name[binary]] + tail, f"benchmark invocation differs: {name}")
    return {
        "hardware_verified": hardware_result,
        "tests_passed": tests, "total_tests_passed": sum(tests.values()),
        "tests_ignored": ignored_tests, "runner_version": profile["version"],
        "completed_stages_verified": len(events), "peak_rss_kib_by_stage": rss,
        "binary_sha256_records": binaries, "downloaded_binary_bytes_verified": binary_dir is not None,
        "binary_verification_scope": "locally rehashed executables and recorded invocation paths" if binary_dir else "hash manifest records and invocation paths; executable bytes were not supplied",
    }


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("logs", type=Path, help="snapshot or downloaded results-public-csc256 directory")
    parser.add_argument("--source-archive", type=Path)
    parser.add_argument("--source-sha256")
    parser.add_argument("--evidence-archive", type=Path)
    parser.add_argument("--evidence-sha256")
    parser.add_argument("--binary-dir", type=Path, help="directory containing the two downloaded Linux executables")
    parser.add_argument("--output", type=Path, help="optional audit JSON outside the immutable logs directory")
    parser.add_argument("--quiet", action="store_true", help="print a compact result while still writing full --output JSON")
    args = parser.parse_args(argv)
    supplied = args.logs.resolve()
    need(supplied.is_dir(), "snapshot/logs directory does not exist")
    if (supplied / "evidence_sha256.json").is_file():
        logs = supplied
    else:
        candidates = [p for p in (supplied / "results-public-csc256", supplied / "evidence" / "results-public-csc256")
                      if (p / "evidence_sha256.json").is_file()]
        need(len(candidates) == 1, "expected exactly one completed results-public-csc256 directory in snapshot")
        logs = candidates[0]
    if args.binary_dir is not None:
        args.binary_dir = args.binary_dir.resolve()
    if args.evidence_archive is not None:
        args.evidence_archive = args.evidence_archive.resolve()
    source = args.source_archive
    if source is None:
        source = next((p / "source.tar.gz" for p in logs.parents if (p / "source.tar.gz").is_file()), None)
    need(source is not None, "source.tar.gz not found; pass --source-archive")
    count = evidence_files(logs)
    source_result = source_archive(source.resolve(), args.source_sha256)
    archive_hash = None
    if args.evidence_archive is not None:
        archive_hash = verify_evidence_archive(args.evidence_archive, args.evidence_sha256, logs, args.binary_dir)
    else:
        need(args.evidence_sha256 is None, "--evidence-sha256 requires --evidence-archive")
    execution = verify_execution(logs, source_result["runner_profile"], args.binary_dir)
    summary = recompute_summary(logs)
    result = {
        "audit": "passed", "schema_version": 1,
        "proof_verification_scope": "all recorded warmup, formal and validation samples report complete verification and matching Fiat-Shamir states; proofs are not replayed by this Python audit",
        "evidence_files_verified": count, "evidence_archive_sha256": archive_hash,
        "source": source_result, **execution, "summary_recomputed_and_matches": True,
        "formal_samples": {"paired": 18, "separate": 18, "flock": 18},
        "warmup_samples": {"paired": 3, "separate": 3, "flock": 3},
        "tamper_cases_rejected": sum(r["rejected"] for r in summary["negative_tests"]),
        "paired_over_separate_time_saved_percent": 100 * (1 - summary["paired"]["prover_ms"] / summary["separate"]["prover_ms"]),
        "paired_over_flock_time_saved_percent": 100 * (1 - summary["paired"]["prover_ms"] / summary["flock"]["prove_with_commit_ms"]),
        "summary": summary,
    }
    encoded = json.dumps(result, indent=2, allow_nan=False) + "\n"
    if args.output is not None:
        output = args.output.resolve()
        need(not output.is_relative_to(logs), "refusing to modify the immutable evidence directory")
        protected = {source.resolve(), source.resolve().parent / "source_manifest.json"}
        if args.evidence_archive is not None:
            protected.add(args.evidence_archive)
        need(output not in protected and output.suffix == ".json", "audit output must be a separate JSON file")
        need(args.binary_dir is None or not output.is_relative_to(args.binary_dir), "refusing to write into binary evidence directory")
        with output.open("w", encoding="utf-8", newline="\n") as stream:
            stream.write(encoded)
    if args.quiet:
        print(json.dumps({k: result[k] for k in (
            "audit", "evidence_files_verified", "downloaded_binary_bytes_verified",
            "total_tests_passed", "formal_samples", "warmup_samples", "tamper_cases_rejected",
            "summary_recomputed_and_matches",
        )}))
    else:
        print(encoded, end="")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (AuditError, KeyError, TypeError, OSError, ValueError, SyntaxError, tarfile.TarError) as error:
        print(f"AUDIT FAILED: {error}", file=sys.stderr)
        raise SystemExit(1)
