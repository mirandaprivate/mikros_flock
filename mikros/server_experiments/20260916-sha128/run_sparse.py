#!/usr/bin/env python3
"""256-SHA comparison with mandatory packed sparse matrix lookup proofs."""
import argparse
import json
import os
from pathlib import Path
import statistics
import subprocess
import time
from run_server import CPU_SPEC, FEATURES, TOOLCHAIN, digest, preflight, records, save


def audit(logs):
    groups = {k: [] for k in ("paired", "separate", "flock")}
    metadata = {"mikros": [], "flock": []}
    for batch in range(3):
        for system in ("mikros", "flock"):
            meta, rows = records(logs / f"{system}-{batch}.jsonl")
            metadata[system].append(meta)
            assert meta["sha_compressions"] == 256
            assert meta["threads"] == meta["all_core_threads"] == 64
            assert FEATURES.issubset(meta["target_features"])
            samples = [r for r in rows if r["record"] == "sample"]
            assert len(samples) == (14 if system == "mikros" else 7)
            assert all(r["verified"] and r["transcript_match"] for r in samples)
            for row in samples:
                if not row["warmup"]:
                    groups[row["opening"] if system == "mikros" else "flock"].append(row)
    for meta in metadata["mikros"] + metadata["flock"]:
        for key in ("fixture_digest_blake3",):
            assert meta[key] == metadata["mikros"][0][key], key
        assert meta["pcs_params"]["m"] == 23
        assert meta["pcs_params"]["profile"] == "fast"
    assert all(m["native_flock_circuit_digest_blake3"] == metadata["flock"][0]["circuit_digest_blake3"] for m in metadata["mikros"])
    assert len({m["circuit_digest_blake3"] for m in metadata["mikros"]}) == 1
    assert len({r["piop_blake3"] for key in ("paired", "separate") for r in groups[key]}) == 1
    assert len({r["commitment_blake3"] for key in ("paired", "separate") for r in groups[key]}) == 1
    result = {"instances": 256, "threads": 64, "metadata": metadata,
              "same_f128_reduction": True, "same_mikros_commitment": True,
              "same_sha_public_statement": True, "native_pcs_backend": "FLOCK Ligerito Fast",
              "pcs_layouts_differ": True, "sha_r1cs_encodings_differ": True, "formal_repetitions": 18,
              "matrix_authentication": "packed sparse lookup; all L/R/O; no verifier matrix scan"}
    for key, samples in groups.items():
        assert len(samples) == 18
        assert len({r["proof_blake3"] for r in samples}) == 1
        times = samples if key == "flock" else [r["timings"] for r in samples]
        keys = ("prove_with_commit_ms",) if key == "flock" else (
            "prover_ms", "witness_commit_ms", "hadamard_ms", "image_reduction_ms",
            "read_prepare_commit_ms", "lookup_reduction_ms", "public_and_pair_values_ms", "pair_reduction_ms",
            "witness_open_ms", "static_open_ms", "read_open_ms")
        row = {k: statistics.median(r[k] for r in times) for k in keys}
        row.update(prover_range_ms=[min(r[keys[0]] for r in times), max(r[keys[0]] for r in times)],
                   proof_bytes=samples[0]["proof_bytes"], proof_blake3=samples[0]["proof_blake3"],
                   verify_ms=statistics.median(r["verify_ms"] for r in samples))
        if key != "flock":
            assert all(r["inner_openings"] == 3 and r["witness_claims"] == (8 if key == "paired" else 16)
                       and r["static_claims"] == 66 and r["read_claims"] == 36
                       and r["matrix_evaluation_proofs"] == 6 and r["lookup_arguments"] == 12
                       and r["hadamard_proofs"] == 2 for r in times)
            row.update(witness_claims=times[0]["witness_claims"], static_claims=66, read_claims=36, inner_openings=3)
        result[key] = row
    result["pairing_speedup"] = result["separate"]["prover_ms"] / result["paired"]["prover_ms"]
    result["flock_over_paired"] = result["flock"]["prove_with_commit_ms"] / result["paired"]["prover_ms"]
    _, rows = records(logs / "validation.jsonl")
    negatives = [r for r in rows if r["record"] == "negative_tests"]
    assert {r["opening"] for r in negatives} == {"paired", "separate"}
    assert all(r["rejected"] >= 23 for r in negatives)
    result["negative_tests"] = negatives
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--target", required=True)
    parser.add_argument("--logs", default="results-sparse256")
    args = parser.parse_args()
    root = Path.cwd().resolve()
    logs = (root / args.logs).resolve()
    assert logs.is_relative_to(root)
    logs.mkdir(exist_ok=False)
    env = {k: v for k, v in os.environ.items() if not k.startswith(("FLOCK_", "MIKROS_", "E2E_", "LIG_"))
           and k not in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTC_WRAPPER", "PCS_TRACE", "LD_PRELOAD")}
    env.update(RUSTFLAGS="-C target-cpu=native", RAYON_NUM_THREADS="64", CARGO_TARGET_DIR=str(Path(args.target).resolve()),
               FLOCK_PCS_PACKED_DIRECT="1", FLOCK_WP_UNINIT="1")
    env["PATH"] = str(Path.home() / ".cargo/bin") + ":" + env.get("PATH", "")
    manifest = json.loads((root / "source_manifest.json").read_text())
    status = {"state": "starting", "completed": []}

    def verify_sources():
        assert all(digest(root / n) == h for n, h in manifest["files_sha256"].items())

    def run(name, command, pinned=True):
        status["state"] = name
        save(logs / "status.json", status)
        if pinned:
            command = ["/usr/bin/time", "-v", "-o", str(logs / f"{name}.resource.txt"), "taskset", "-c", CPU_SPEC] + command
        save(logs / f"{name}.command.json", command)
        t = time.monotonic()
        with (logs / f"{name}.jsonl").open("w") as out, (logs / f"{name}.stderr.log").open("w") as err:
            process = subprocess.run(command, env=env, cwd=root, stdout=out, stderr=err)
        event = {"name": name, "exit_code": process.returncode, "wall_seconds": time.monotonic() - t}
        status["completed"].append(event)
        save(logs / "status.json", status)
        print(json.dumps(event), flush=True)
        assert process.returncode == 0, event

    try:
        verify_sources()
        save(logs / "hardware.json", {"cpuinfo": Path("/proc/cpuinfo").read_text(), "meminfo": Path("/proc/meminfo").read_text(),
             "affinity": CPU_SPEC, "utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
             "environment": {k: v for k, v in env.items() if k.startswith(("FLOCK_", "MIKROS_", "RAYON_", "RUSTFLAGS"))}})
        save(logs / "preflight.json", preflight())
        for name in manifest["files_sha256"]:
            os.utime(root / name, None)
        cargo = ["cargo", TOOLCHAIN]
        flags = ["--release", "--offline", "--locked", "-j8"]
        run("build", cargo + ["build"] + flags + ["-p", "mikros", "--example", "e2e_sha256_paired", "--example", "e2e_flock_baseline"], False)
        for module in ("multiple_image", "packed_sources", "sparse_lookup"):
            run("test-" + module, cargo + ["test"] + flags + ["-p", "mikros", "--lib", module + "::tests", "--", "--test-threads=1"])
        run("test-sparse-sha-encoding", cargo + ["test"] + flags + ["-p", "flock-prover", "sparse_schedule", "--", "--test-threads=1"])
        run("test-sha", cargo + ["test"] + flags + ["-p", "mikros", "--example", "e2e_sha256_paired", "--", "--test-threads=1"])
        binaries = {name: str(Path(args.target) / "release/examples" / binary) for name, binary in (
            ("mikros", "e2e_sha256_paired"), ("flock", "e2e_flock_baseline"))}
        hashes = {path: digest(Path(path)) for path in binaries.values()}
        save(logs / "binary-sha256.json", hashes)
        run("validation", [binaries["mikros"], "--opening", "both", "--threads", "64", "--reps", "1", "--warmups", "0", "--tamper"])
        for batch in range(3):
            order = ["mikros", "flock"] if batch % 2 == 0 else ["flock", "mikros"]
            for name in order:
                save(logs / f"{name}-{batch}.preflight.json", preflight())
                extra = ["--opening", "both"] if name == "mikros" else ["--compressions", "256", "--profile", "fast"]
                run(f"{name}-{batch}", [binaries[name]] + extra + ["--threads", "64", "--reps", "6", "--warmups", "1"])
        save(logs / "summary.json", audit(logs))
        verify_sources()
        assert all(digest(Path(p)) == h for p, h in hashes.items())
        status["state"] = "complete"
    except Exception as error:
        status.update(state="failed", error=str(error))
        raise
    finally:
        save(logs / "status.json", status)
        save(logs / "evidence_sha256.json", {p.name: digest(p) for p in sorted(logs.iterdir()) if p.name != "evidence_sha256.json" and p.is_file()})


if __name__ == "__main__":
    main()
