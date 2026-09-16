#!/usr/bin/env python3
"""Audit downloaded evidence and write the root comparison report."""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import statistics


def main():
    p = argparse.ArgumentParser()
    p.add_argument("evidence", type=Path)
    p.add_argument("output", type=Path)
    args = p.parse_args()
    logs = args.evidence.resolve()
    hashes = json.loads((logs / "evidence_sha256.json").read_text())
    for name, expected in hashes.items():
        assert (logs / name).resolve().is_relative_to(logs)
        assert hashlib.sha256((logs / name).read_bytes()).hexdigest() == expected, name
    status = json.loads((logs / "status.json").read_text())
    assert status["state"] == "complete"
    assert all(row["exit_code"] == 0 for row in status["completed"])
    original = json.loads((logs / "summary.json").read_text())
    spec = importlib.util.spec_from_file_location("sha_runner", Path(__file__).with_name("run_server.py"))
    runner = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(runner)
    # Recompute in memory without rewriting Linux evidence with Windows newlines.
    runner.save = lambda path, value: None
    result = runner.audit(logs, original["batches"], original["reps_per_batch"])
    assert result == original, "summary differed from raw records"
    # Raw downloaded evidence remains byte-identical.
    assert hashlib.sha256((logs / "summary.json").read_bytes()).hexdigest() == hashes["summary.json"]
    m, f, prod = (result[k] for k in ("mikros_packed", "flock", "flock-production"))
    names = (("Mikros packed / bit wiring", m, "prover_ms"),
             ("FLOCK / bit wiring", f, "prover_ms"),
             ("FLOCK native production / word wiring", prod, "prove_with_commit_ms"))
    lines = ["# 128 independent SHA-256 compression: Mikros vs FLOCK", "",
             "Server: 192.168.219.251; 64 threads; GFNI / AVX-512 / VPCLMUL; native FLOCK Ligerito Fast PCS, no WHIR.",
             "Exactly 128 fixed-IV compression calls with distinct deterministic public 512-bit inputs and public outputs; no SHA-256 padding.",
             "Three fresh-process batches, six formal repetitions and one warmup per batch. Fixed input/transcript across all repetitions.", "",
             "| Implementation | Prover median (ms) | Min-max (ms) | Verify median (ms) | Proof (bytes) | Compressions/s |",
             "|---|---:|---:|---:|---:|---:|"]
    for name, r, key in names:
        low, high = r["prover_range_ms"]
        lines.append(f"| {name} | {r[key]:.3f} | {low:.3f}-{high:.3f} | {r['verify_ms']:.3f} | {r['proof_bytes']} | {r['compressions_per_second']:.1f} |")
    lines += ["", f"Controlled FLOCK-bit / Mikros prover ratio: {result['flock_bit_over_mikros']:.4f}x (greater than 1 favors Mikros).",
              f"Native-production / Mikros elapsed-time ratio: {result['flock_production_over_mikros']:.4f}x; different wiring and timer boundary, not an isolated protocol speedup.", "",
              "| Common bit-path phase | Mikros median (ms) | FLOCK median (ms) |",
              "|---|---:|---:|"]
    for key in ("commit_ms", "boolean_ms", "wiring_ms", "parallel_piops_ms", "pcs_open_ms", "prover_ms"):
        lines.append(f"| {key} | {m[key]:.3f} | {f[key]:.3f} |")
    meta = result["common_metadata"]
    lines += ["", "Boolean and wiring overlap; do not sum their individual medians. Prover includes commitment, all challenge-dependent conversions, gather, configured PoW, PCS opening and return cleanup; serialization excluded.",
              "Native production uses word wiring and production lincheck, and additionally times union assembly/compaction. It remains a separate practical implementation baseline.", "",
              "Offline common-path median preparation (ms):"]
    for key in ("offline_circuit_ms", "offline_witness_ms", "offline_compaction_ms", "offline_wiring_ms", "offline_packed_coefficients_ms"):
        lines.append(f"- {key}: {statistics.median(r[key] for r in meta):.3f}")
    lines += ["", "Validation: all formal and warmup proofs use the complete public-only verifier and transcript check. Identical statement/circuit digests, witness commitments, embedded PCS parameters and configurations across common and production paths. Proof bytes stable per implementation across repetitions.",
              f"Rejected tamper cases: {json.dumps(result['negative_tests'])}.",
              "Fixed transcript also fixes grinding nonce work; these results do not sample variation across independent random inputs. No zero-knowledge claim.", "",
              f"Raw evidence directory: {logs.as_posix()}",
              f"Source revision: e20430d81bb565e95b7277410d4790f6f58042ea plus exact-count fixture/benchmark changes."]
    args.output.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(json.dumps({"verified_evidence_files": len(hashes), "flock_ms": f["prover_ms"],
                      "mikros_ms": m["prover_ms"], "native_flock_ms": prod["prove_with_commit_ms"]}))


if __name__ == "__main__":
    main()
