#!/usr/bin/env python3
"""Independently audit downloaded 256-SHA sparse-lookup benchmark evidence.

Usage:
  python verify_sparse_shared_evidence.py SNAPSHOT/evidence/results-sparse-shared256 \
      --source-archive SNAPSHOT/source.tar.gz --output SNAPSHOT/local_audit.json

This script never imports a server audit function and never changes raw logs.
It reads source tar members in memory without extracting them. A neighboring
source.tar.gz.sha256 pins the source archive, or --source-sha256 can pin it.
Pass --binary-dir if the actual executables were downloaded; otherwise binary
hashes are explicitly reported as server records, not locally rehashed bytes.
For transfer authentication, also pass --evidence-archive and its independently
recorded --evidence-sha256. A self-contained checksum manifest alone does not
authenticate who produced an artifact or bind source bytes to machine code.
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
PHASES = (
    "witness_commit_ms", "hadamard_ms", "image_reduction_ms",
    "read_prepare_commit_ms", "lookup_reduction_ms", "public_and_pair_values_ms",
    "pair_reduction_ms", "witness_open_ms", "static_open_ms", "read_open_ms",
)
COUNTS = {
    "static_claims": 33, "read_claims": 18, "inner_openings": 3,
    "matrix_evaluation_proofs": 3, "lookup_arguments": 6, "hadamard_proofs": 2,
}
TEST_COUNTS_V1 = {
    "test-multiple_image": 5, "test-packed_sources": 3, "test-sparse_lookup": 6,
    "test-shared-hadamard": 2,
    "test-sparse-prefix": 3,
    "test-sparse-sha-encoding": 1, "test-sha": 17,
}
TEST_COUNTS_V2 = {
    "test-multiple_image": 7, "test-packed_sources": 3, "test-sparse_lookup": 7,
    "test-shared-hadamard": 2, "test-sparse-prefix": 3,
    "test-zero-weight-block-skip": 2, "test-checked-initial-message": 1,
    "test-sparse-sha-encoding": 1, "test-sha": 17,
}
TEST_COUNTS_V3 = {
    "test-multiple_image": 7, "test-packed_sources": 3, "test-sparse_lookup": 8,
    "test-shared-hadamard": 2, "test-sparse-prefix": 3,
    "test-zero-weight-block-skip": 2, "test-sparse-scaled-eq": 2,
    "test-checked-initial-message": 1, "test-sparse-sha-encoding": 1,
    "test-compact-sha-encoding": 2, "test-sha": 17,
}
RUNNER_SOURCE = "mikros/server_experiments/20260916-sha128/run_sparse_shared.py"
BASE_ENV = {
    "RUSTFLAGS": "-C target-cpu=native", "RAYON_NUM_THREADS": "64",
    "FLOCK_PCS_PACKED_DIRECT": "1", "FLOCK_WP_UNINIT": "1",
}
REQUIRED_SOURCE = {
    "Cargo.lock", "mikros/src/multiple_image.rs", "mikros/src/sparse_lookup.rs",
    "mikros/src/packed_sources.rs", "mikros/examples/e2e_common/sparse_sha_shared.rs",
    "mikros/examples/e2e_sha256_sparse_shared.rs", "mikros/examples/e2e_flock_baseline.rs",
    "crates/flock-prover/src/r1cs_hashes/sha2.rs", "crates/flock-core/src/pcs/paired.rs",
    "mikros/server_experiments/20260916-sha128/run_sparse_shared.py",
    "mikros/server_experiments/20260916-sha128/run_server.py",
}


class AuditError(ValueError):
    pass


def need(condition, message):
    if not condition:
        raise AuditError(message)


def digest(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def json_file(path):
    return json.loads(path.read_text(encoding="utf-8-sig"))


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


def power_of_two(value):
    return type(value) is int and value > 0 and value & (value - 1) == 0


def next_power(value):
    return 1 << (max(value, 1) - 1).bit_length()


def safe_name(name):
    need(isinstance(name, str) and name and "\\" not in name, "invalid archive/manifest path")
    path = PurePosixPath(name)
    need(not path.is_absolute() and ".." not in path.parts and ":" not in name,
         f"unsafe archive/manifest path: {name}")
    need(path.as_posix() == name, f"non-canonical archive/manifest path: {name}")
    return path


def runner_profile(source):
    """Recognize the three reviewed runners without importing/executing them.

    Every version's complete test list, build targets and cache setting are
    mandatory. Unknown, partial, duplicate or reordered stages are rejected;
    runtime logs cannot select a weaker source-version profile.
    """
    tree = ast.parse(source, filename=RUNNER_SOURCE)
    mains = [n for n in tree.body if isinstance(n, ast.FunctionDef) and n.name == "main"]
    need(len(mains) == 1, "source runner must have one main function")
    main = mains[0]
    runs = sorted(
        (n for n in ast.walk(main) if isinstance(n, ast.Call)
         and isinstance(n.func, ast.Name) and n.func.id == "run"),
        key=lambda n: n.lineno,
    )
    literal_tests = [n.args[0].value for n in runs if n.args
                     and isinstance(n.args[0], ast.Constant)
                     and isinstance(n.args[0].value, str)
                     and n.args[0].value.startswith("test-")]
    if literal_tests == list(TEST_COUNTS_V1)[3:]:
        version, counts, ignored = "online-v1", TEST_COUNTS_V1, 1
    elif literal_tests == list(TEST_COUNTS_V2)[3:]:
        version, counts, ignored = "online-v2", TEST_COUNTS_V2, 2
    elif literal_tests == list(TEST_COUNTS_V3)[3:]:
        version, counts, ignored = "online-v3", TEST_COUNTS_V3, 2
    else:
        raise AuditError("source runner has an unrecognized test plan")
    loops = [n for n in ast.walk(main) if isinstance(n, ast.For)
             and isinstance(n.target, ast.Name) and n.target.id == "module"]
    need(len(loops) == 1, "source runner module test loop differs")
    need(ast.literal_eval(loops[0].iter) == ("multiple_image", "packed_sources", "sparse_lookup"),
         "source runner module test order differs")
    dynamic_tests = [n for n in runs if n.args and isinstance(n.args[0], ast.BinOp)
                     and isinstance(n.args[0].op, ast.Add)
                     and isinstance(n.args[0].left, ast.Constant)
                     and n.args[0].left.value == "test-"
                     and isinstance(n.args[0].right, ast.Name)
                     and n.args[0].right.id == "module"]
    need(len(dynamic_tests) == 1 and dynamic_tests[0] in list(ast.walk(loops[0])),
         "source runner module test invocation differs")
    build_examples = ["e2e_sha256_sparse_shared", "e2e_flock_baseline"]
    if version == "online-v3":
        build_examples.insert(1, "e2e_sha256_paired")
    build_tail = ["-p", "mikros"] + [token for name in build_examples for token in ("--example", name)]
    expected_build = ast.parse(
        f'run("build", cargo + ["build"] + flags + {build_tail!r}, False)', mode="eval"
    ).body
    builds = [n for n in runs if n.args and isinstance(n.args[0], ast.Constant)
              and n.args[0].value == "build"]
    need(len(builds) == 1 and ast.dump(builds[0]) == ast.dump(expected_build),
         "source runner build targets disagree with test version")
    updates = [n for n in ast.walk(main) if isinstance(n, ast.Call)
               and isinstance(n.func, ast.Attribute) and n.func.attr == "update"
               and isinstance(n.func.value, ast.Name) and n.func.value.id == "env"]
    need(len(updates) == 1, "source runner environment setup differs")
    declared = {}
    for keyword in updates[0].keywords:
        need(keyword.arg is not None and keyword.arg not in declared,
             "source runner environment uses dynamic or duplicate keywords")
        if keyword.arg.startswith(("FLOCK_", "MIKROS_")) or keyword.arg in BASE_ENV:
            declared[keyword.arg] = ast.literal_eval(keyword.value)
    expected_env = dict(BASE_ENV)
    if version in ("online-v2", "online-v3"):
        expected_env["MIKROS_SPARSE_CACHE_LEAVES"] = "1"
    need(declared == expected_env, "source runner test version/cache environment disagree")
    return {
        "version": version, "tests_passed": dict(counts),
        "tests_ignored": {name: ignored if name == "test-sparse_lookup" else 0 for name in counts},
        "controlled_environment": expected_env, "build_examples": build_examples,
    }


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
    expected = explicit_hash
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
        manifest = json.load(tar.extractfile(members["source_manifest.json"]))
        files = manifest["files_sha256"]
        need(isinstance(files, dict) and REQUIRED_SOURCE.issubset(files), "sparse implementation missing from source manifest")
        need(set(members) == set(files) | {"source_manifest.json"}, "source archive/manifest member mismatch")
        need(re.fullmatch(r"[0-9a-f]{40}", manifest["base_revision"]), "invalid base revision")
        for name, expected in files.items():
            safe_name(name)
            with tar.extractfile(members[name]) as stream:
                actual = hashlib.file_digest(stream, "sha256").hexdigest()
            need(actual == checked_hash(expected, name), f"source member hash mismatch: {name}")
        with tar.extractfile(members[RUNNER_SOURCE]) as stream:
            profile = runner_profile(stream.read().decode("utf-8-sig"))
        if profile["version"] == "online-v3":
            alias = "mikros/examples/e2e_sha256_paired.rs"
            shared = "mikros/examples/e2e_sha256_sparse_shared.rs"
            need(alias in files, "v3 paired example alias is missing from source archive")
            def entrypoint(name):
                with tar.extractfile(members[name]) as stream:
                    text = stream.read().decode("utf-8-sig")
                return "\n".join(line for line in text.splitlines() if not line.lstrip().startswith("//!"))
            need(entrypoint(alias) == entrypoint(shared), "v3 paired alias does not use the same shared entrypoint")
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
                need(name in ("e2e_sha256_sparse_shared", "e2e_flock_baseline"), "unexpected archived binary")
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
        need(binary_seen == {"e2e_sha256_sparse_shared", "e2e_flock_baseline"}, "incomplete archived binary set")
    return actual


def records(path):
    rows = [json.loads(line) for line in path.read_text(encoding="utf-8-sig").splitlines() if line.strip()]
    metadata = [r for r in rows if r.get("record") == "metadata"]
    need(len(metadata) == 1 and rows[0] is metadata[0], f"metadata shape: {path.name}")
    need(all(r.get("record") in ("metadata", "sample", "negative_tests") for r in rows), f"unexpected record: {path.name}")
    return metadata[0], rows


def validate_layout(specs, expected, carrier_bytes, params, label):
    need(len(specs) == len(expected), f"{label} column count")
    end = 0
    for index, (spec, (length, width)) in enumerate(zip(specs, expected)):
        need(spec["len"] == length and spec["width"] == width, f"{label} column shape {index}")
        need(power_of_two(length) and power_of_two(width) and width <= 128, f"{label} invalid packed width/length")
        words = max(length // (128 // width), 1)
        offset = (end + words - 1) & -words
        need(spec["word_len"] == words and spec["word_offset"] == offset, f"{label} alignment {index}")
        end = offset + words
    words = next_power(max(end, 1 << 15))
    need(carrier_bytes == words * 16 and params["m"] == words.bit_length() + 6, f"{label} carrier/PCS dimensions")
    need(params["profile"] == "fast" and params["num_lanes"] is None, f"{label} native PCS policy")


def validate_metadata(meta, system):
    need(meta["sha_compressions"] == 256 and meta["fixture"] == "sha256-independent-compressions", "wrong SHA workload")
    need(meta["threads"] == meta["all_core_threads"] == 64, "wrong thread pools")
    need(FEATURES.issubset(meta["target_features"]), "required native CPU features missing")
    for key in ("fixture_digest_blake3", "circuit_digest_blake3"):
        checked_hash(meta[key], key)
    need(meta["pcs_params"]["m"] == 23 and meta["pcs_params"]["profile"] == "fast", "wrong witness/native PCS")
    if system == "mikros":
        need(meta["reduction_field_bits"] == 128 and meta["opening_logical_field_bits"] == 256, "wrong fields")
        checked_hash(meta["native_flock_circuit_digest_blake3"], "native FLOCK circuit digest")
        need(meta["zero_knowledge"] is False and "sparse" in meta["matrix_authentication"].lower(), "wrong proof description")
        need(meta["witness_commitment_bytes"] == 16 * (1 << 16), "wrong paired witness size")
        need(len(meta["nonzeros"]) == 3, "three sparse matrices required")
        static, reads = [], []
        for nnz in meta["nonzeros"]:
            integer(nnz, "nonzeros")
            entries = next_power(nnz)
            time_width = next_power(entries.bit_length())
            static.extend([(entries, 16)] * 2 + [(entries, time_width)] * 4
                          + [(1 << 15, time_width)] * 2 + [(entries, 1)])
        for index in range(3):
            reads.extend([(next_power(meta["nonzeros"][index % 3]), 128)] * 2)
        validate_layout(meta["static_columns"], static, meta["static_commitment_bytes"], meta["static_pcs_params"], "static")
        validate_layout(meta["read_columns"], reads, meta["read_commitment_bytes"], meta["read_pcs_params"], "read")
        for key in ("static_pcs_params", "read_pcs_params"):
            for setting in ("profile", "log_inv_rate", "merkle_hash"):
                need(meta[key][setting] == meta["pcs_params"][setting], f"different native PCS setting: {setting}")


def validate_sample(row, system):
    need(row["verified"] is True and row["transcript_match"] is True, "unverified sample")
    need(type(row["warmup"]) is bool, "invalid warmup flag")
    integer(row["iteration"], "iteration")
    integer(row["proof_bytes"], "proof bytes", True)
    for key in ("proof_blake3", "commitment_blake3"):
        checked_hash(row[key], key)
    number(row["verify_ms"], "verify_ms", True)
    if system == "flock":
        need(row["system"] == "flock-circuit-e2e", "wrong native FLOCK system")
        number(row["prove_with_commit_ms"], "native prover time", True)
        return
    need(row["system"] == "mikros" and row["opening"] in ("paired", "separate"), "wrong Mikros sample")
    checked_hash(row["piop_blake3"], "F128 PIOP hash")
    times = row["timings"]
    number(times["prover_ms"], "Mikros prover time", True)
    for phase in PHASES:
        number(times[phase], phase)
    need(sum(times[p] for p in PHASES) <= times["prover_ms"] + 1e-5, "phase sum exceeds timed prover scope")
    need(all(times[k] == v for k, v in COUNTS.items()), "lookup/PCS endpoint count differs")
    need(times["witness_claims"] == (5 if row["opening"] == "paired" else 10), "witness opening count")


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
        need(meta["fixture_digest_blake3"] == reference["fixture_digest_blake3"], "SHA public statement differs")
    need(len({m["circuit_digest_blake3"] for m in metadata["flock"]}) == 1, "native circuit drift")
    for meta in metadata["mikros"]:
        need(meta["native_flock_circuit_digest_blake3"] == metadata["flock"][0]["circuit_digest_blake3"], "native FLOCK circuit correspondence missing")
        for key in ("circuit_digest_blake3", "pcs_params", "static_pcs_params", "read_pcs_params", "static_columns", "read_columns", "nonzeros"):
            need(meta[key] == reference[key], f"Mikros metadata drift: {key}")
    for meta in metadata["flock"]:
        need(meta["pcs_params"] == metadata["flock"][0]["pcs_params"], "native PCS drift")
    validation_meta, rows = records(logs / "validation.jsonl")
    validate_metadata(validation_meta, "mikros")
    for key in ("fixture_digest_blake3", "circuit_digest_blake3", "static_columns", "read_columns"):
        need(validation_meta[key] == reference[key], f"validation differs: {key}")
    checked = [r for r in rows if r["record"] == "sample"]
    negatives = [r for r in rows if r["record"] == "negative_tests"]
    need(len(checked) == 2 and {r["opening"] for r in checked} == {"paired", "separate"}, "validation samples missing")
    need(len(negatives) == 2 and {r["opening"] for r in negatives} == {"paired", "separate"}, "negative tests missing")
    for row in checked:
        validate_sample(row, "mikros")
        need(row["iteration"] == 0 and row["warmup"] is False, "wrong validation iteration")
        all_groups[row["opening"]].append(row)
    for row in negatives:
        integer(row["rejected"], "rejected tamper cases", True)
        need(row["rejected"] >= (27 if row["opening"] == "paired" else 26), "incomplete tamper suite")
    mikros_rows = all_groups["paired"] + all_groups["separate"]
    need(len({r["piop_blake3"] for r in mikros_rows}) == 1, "pairing changed F128 reduction")
    need(len({r["commitment_blake3"] for r in mikros_rows}) == 1, "Mikros commitment changed")
    result = {
        "instances": 256, "threads": 64, "metadata": metadata,
        "same_f128_reduction": True, "same_mikros_commitment": True,
        "same_sha_public_statement": True, "native_pcs_backend": "FLOCK Ligerito Fast",
        "pcs_layouts_differ": True, "sha_r1cs_encodings_differ": True, "formal_repetitions": 18,
        "matrix_authentication": "packed sparse lookup; all L/R/O; no verifier matrix scan",
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
            row.update(witness_claims=times[0]["witness_claims"], static_claims=33, read_claims=18, inner_openings=3)
        result[key] = row
    result["pairing_speedup"] = result["separate"]["prover_ms"] / result["paired"]["prover_ms"]
    result["flock_over_paired"] = result["flock"]["prove_with_commit_ms"] / result["paired"]["prover_ms"]
    result["negative_tests"] = negatives
    need(result == json_file(logs / "summary.json"), "independently recomputed summary differs from server summary")
    return result


def verify_execution(logs, profile, binary_dir=None):
    status = json_file(logs / "status.json")
    need(status["state"] == "complete" and "error" not in status, "server run incomplete/failed")
    events = status["completed"]
    names = [e["name"] for e in events]
    expected = ["build", *profile["tests_passed"], "validation", "mikros-0", "flock-0", "flock-1", "mikros-1", "mikros-2", "flock-2"]
    need(names == expected, "completed stages/order differ from benchmark plan")
    hardware = json_file(logs / "hardware.json")
    need(hardware["affinity"] == CPU_SPEC, "wrong hardware affinity")
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
        need(name in ("e2e_sha256_sparse_shared", "e2e_flock_baseline") and name not in by_name, "unexpected benchmark binary")
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
            elif name == "test-sparse-sha-encoding":
                expected_command += ["flock-prover", "sparse_schedule", "--", "--test-threads=1"]
            elif name == "test-compact-sha-encoding":
                expected_command += ["flock-prover", "sparse_compact", "--", "--test-threads=1"]
            elif name == "test-sparse-prefix":
                expected_command += ["flock-core", "--lib", "sparse_prefix_weights", "--", "--test-threads=1"]
            elif name == "test-zero-weight-block-skip":
                expected_command += ["flock-core", "--lib", "zero_weight_block_skip", "--", "--test-threads=1"]
            elif name == "test-sparse-scaled-eq":
                expected_command += ["flock-core", "--lib", "sparse_scaled_eq", "--", "--test-threads=1"]
            elif name == "test-checked-initial-message":
                expected_command += ["mikros", "--lib", "inner_product::tests::checked_initial_message", "--", "--test-threads=1"]
            elif name == "test-shared-hadamard":
                expected_command += ["mikros", "--lib", "hadamard::shared_batch_tests", "--", "--test-threads=1"]
            elif name == "test-sha":
                expected_command += ["mikros", "--example", "e2e_sha256_sparse_shared", "--", "--test-threads=1"]
            else:
                expected_command += ["mikros", "--lib", name.removeprefix("test-") + "::tests", "--", "--test-threads=1"]
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
            binary = "e2e_flock_baseline" if system == "flock" else "e2e_sha256_sparse_shared"
            tail = ["--opening", "both"] if system == "mikros" else ["--compressions", "256", "--profile", "fast"]
            tail += ["--threads", "64", "--reps", "1" if name == "validation" else "6", "--warmups", "0" if name == "validation" else "1"]
            if name == "validation":
                tail.append("--tamper")
            need(command == [by_name[binary]] + tail, f"benchmark invocation differs: {name}")
    return {
        "tests_passed": tests, "total_tests_passed": sum(tests.values()),
        "tests_ignored": ignored_tests, "runner_version": profile["version"],
        "completed_stages_verified": len(events), "peak_rss_kib_by_stage": rss,
        "binary_sha256_records": binaries, "downloaded_binary_bytes_verified": binary_dir is not None,
        "binary_verification_scope": "locally rehashed executables and recorded invocation paths" if binary_dir else "hash manifest records and invocation paths; executable bytes were not supplied",
    }


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("logs", type=Path, help="downloaded results-sparse-shared256 directory")
    parser.add_argument("--source-archive", type=Path)
    parser.add_argument("--source-sha256")
    parser.add_argument("--evidence-archive", type=Path)
    parser.add_argument("--evidence-sha256")
    parser.add_argument("--binary-dir", type=Path, help="directory containing the two downloaded Linux executables")
    parser.add_argument("--output", type=Path, help="optional audit JSON outside the immutable logs directory")
    args = parser.parse_args(argv)
    logs = args.logs.resolve()
    need(logs.is_dir(), "logs directory does not exist")
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
        need(output != source.resolve() and output.suffix == ".json", "audit output must be a separate JSON file")
        with output.open("w", encoding="utf-8", newline="\n") as stream:
            stream.write(encoded)
    print(encoded, end="")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (AuditError, KeyError, TypeError, OSError, ValueError, SyntaxError, tarfile.TarError) as error:
        print(f"AUDIT FAILED: {error}", file=sys.stderr)
        raise SystemExit(1)
