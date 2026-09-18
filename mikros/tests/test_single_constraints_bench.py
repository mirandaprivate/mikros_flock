"""Driver checks use synthetic worker output; these are not protocol benchmarks."""
import copy
import importlib.util
import json
from pathlib import Path
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

BENCHES = Path(__file__).resolve().parents[1] / "benches"
sys.path.insert(0, str(BENCHES))
spec = importlib.util.spec_from_file_location("single_constraints_bench", BENCHES / "single_constraints_bench.py")
bench = importlib.util.module_from_spec(spec)
spec.loader.exec_module(bench)


def records(system, reps=2, warmups=1):
    metadata = {"type": "metadata", "system": system, "mode": "single", "instances": 1,
                "log_gates": 14, "threads": 1, "all_core_threads": 1, "seed": 97,
                "genuine_gate_count": 1 << 14,
                "baseline_skip": 7, "pcs_packed_direct": True,
                "pcs_backend": "FLOCK native Ligerito Fast", "fixture_digest": "same-fixture",
                "repetitions": reps, "warmups": warmups, "public_io": [1, 0],
                "arithmetization": "native R1CS" if system == "flock" else "binary PLONK",
                "matrix_authentication": "public CSC" if system == "flock" else "preprocessed public PCS",
                "permutation_proof": system == "mikros"}
    rows = [metadata]
    for index in range(reps + warmups):
        prover = 999 if index < warmups else 10 * (index - warmups + 1) * (1 if system == "mikros" else 2)
        rows.append({"type": "sample", "system": system, "mode": "single", "instances": 1,
                     "log_gates": 14, "seed": 97, "threads": 1,
                     "repetition": index if index < warmups else index - warmups,
                     "warmup": index < warmups, "verified": True, "transcript_match": True,
                     "pcs_config_match": True, "times": {"prover_ms": prover,
                     "copy_prepare_ms": 0, "witness_pcs_open_ms": 2, "public_pcs_open_ms": 3,
                     "packed_opening_ms": 4}, "verify_ms": 1, "proof_bytes": 4096,
                     "witness_commitment_blake3": "same-witness", "public_commitment_blake3": "same-public",
                     "witness_pcs_params": {"mode": "Fast", "log_n": 15},
                     "public_pcs_params": {"mode": "Fast", "log_n": 16}})
        if system == "mikros":
            rows[-1]["constraint_breakdown"] = {
                "gate": {"prover_ms": prover * 0.2, "verifier_ms": 0.2, "proof_bytes": 1024},
                "copy": {"prover_ms": prover * 0.4, "verifier_ms": 0.4, "proof_bytes": 1024},
                "shared": {"prover_ms": prover * 0.4, "verifier_ms": 0.4, "proof_bytes": 2048}}
    return rows


def encoded(rows):
    return "\n".join(json.dumps(row) for row in rows) + "\n"


def options():
    return bench.parse_options(["--worker-bin", __file__, "--log-n", "14", "--reps", "2", "--warmups", "1"])


class SingleConstraintsBenchTests(unittest.TestCase):
    def test_default_is_one_large_circuit_with_one_sample_and_no_warmup(self):
        args = bench.parse_options(["--worker-bin", __file__])
        self.assertEqual((args.log_n, args.threads, args.reps, args.warmups), (25, 1, 1, 0))
        self.assertEqual(args.output, bench.ROOT / "single_2_25.log")

    def test_cannot_overwrite_root_paper_logs(self):
        with self.assertRaises(SystemExit):
            bench.parse_options(["--worker-bin", __file__, "--output", "e2e_flock_sha256.log"])

    def test_rejects_csc_index_overflow_before_starting_workers(self):
        with self.assertRaises(SystemExit):
            bench.parse_options(["--worker-bin", __file__, "--log-n", "30"])

    def test_checks_transcript_skip_and_sample_order(self):
        for mutate in (lambda r: r[0].update(baseline_skip=6),
                       lambda r: r[0].update(genuine_gate_count=1 << 15),
                       lambda r: r[0].update(pcs_packed_direct=False),
                       lambda r: r[0].update(arithmetization="native R1CS"),
                       lambda r: r[0].update(matrix_authentication="public CSC"),
                       lambda r: r[0].update(permutation_proof=False),
                       lambda r: r[1].update(transcript_match=False),
                       lambda r: r[1].update(warmup=False),
                       lambda r: r[2].update(repetition=5),
                       lambda r: r[2]["times"].update(prover_ms=float("nan")),
                       lambda r: r[2].update(proof_bytes=4100)):
            rows = records("mikros")
            mutate(rows)
            with self.assertRaises(ValueError):
                bench.parse_worker(encoded(rows), options(), "mikros", 2048)

    def test_excludes_warmups_and_matches_semantic_statement(self):
        reports = [bench.parse_worker(encoded(records(system)), options(), system, 2048) for system in bench.SYSTEMS]
        summary = bench.build_summary(reports)
        self.assertEqual(summary["mikros"]["prover_ms"]["median"], 15)
        self.assertEqual(summary["flock"]["prover_ms"]["median"], 30)
        self.assertEqual(summary["mikros"]["ram_mib"], 2)
        self.assertEqual(summary["mikros"]["phase_medians_ms"]["copy_prepare_ms"], 0)
        for field in bench.MATCHED_FIELDS:
            changed = copy.deepcopy(reports)
            changed[1]["metadata"][field] = "different"
            with self.assertRaisesRegex(ValueError, field):
                bench.build_summary(changed)

    def test_different_arithmetizations_can_have_different_commitments(self):
        reports = [bench.parse_worker(encoded(records(system)), options(), system, 2048) for system in bench.SYSTEMS]
        for row in reports[1]["samples"]:
            row["witness_commitment_blake3"] = "different-witness-layout"
            row.pop("public_commitment_blake3")
            row.pop("public_pcs_params")
        bench.build_summary(reports)

    def test_component_accounting_requires_exact_complete_totals(self):
        for field in ("prover_ms", "verifier_ms", "proof_bytes"):
            rows = records("mikros")
            rows[1]["constraint_breakdown"]["copy"][field] += 1
            with self.assertRaisesRegex(ValueError, "do not sum"):
                bench.parse_worker(encoded(rows), options(), "mikros", 2048)

    def test_full_mock_run_pins_cpu_and_publishes_anonymous_report(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            worker = root / "worker"
            timer = root / "time"
            worker.write_text("mock executable")
            timer.write_text("mock GNU time")
            output = root / "results/compare.log"
            args = bench.parse_options(["--worker-bin", str(worker), "--time-bin", str(timer),
                                        "--log-n", "14", "--reps", "2", "--warmups", "1", "--output", str(output)])
            calls = []

            def fake_worker(command, **kwargs):
                self.assertEqual(kwargs["cpu"], 16)
                self.assertEqual(kwargs["env"]["RAYON_NUM_THREADS"], "1")
                self.assertEqual(kwargs["env"]["FLOCK_VERIFY_THREADS"], "1")
                self.assertEqual(kwargs["env"]["FLOCK_PCS_PACKED_DIRECT"], "1")
                system = command[command.index("--system") + 1]
                calls.append(system)
                kwargs["stdout"].write(encoded(records(system)))
                Path(command[command.index("-o") + 1]).write_text("Maximum resident set size (kbytes): 2048\n")
                return 0

            with patch.object(bench.subprocess, "run", return_value=SimpleNamespace(returncode=0, stdout="GNU time")), \
                 patch.object(bench.os, "sched_getaffinity", return_value={16, 17}, create=True), \
                 patch.object(bench, "ROOT", root), \
                 patch.object(bench, "run_streamed", side_effect=fake_worker), \
                 patch("builtins.print"):
                doc = bench.run(args)
            self.assertEqual(calls, ["mikros", "flock"])
            self.assertEqual(doc["cpu_affinity"], 16)
            report = output.read_text(encoding="utf-8")
            self.assertIn("FLOCK/Mikros prover time: 2.000x", report)
            self.assertIn("packed_opening_ms", report)
            self.assertIn("Permutation argument", report)
            self.assertIn("shared selector data", report)
            self.assertIn("16384 genuine AND/XOR gates", report)
            self.assertIn("65536 R1CS rows", report)
            self.assertNotIn(directory, report)
            self.assertNotIn(directory, output.with_suffix(".json").read_text(encoding="utf-8"))

    def test_failed_worker_does_not_replace_previous_results(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            worker = root / "worker"
            worker.write_text("mock executable")
            output = root / "compare.log"
            output.write_text("previous result")
            args = bench.parse_options(["--worker-bin", str(worker), "--time-bin", str(worker),
                                        "--log-n", "14", "--reps", "2", "--warmups", "1", "--output", str(output)])
            with patch.object(bench.subprocess, "run", return_value=SimpleNamespace(returncode=0, stdout="GNU time")), \
                 patch.object(bench.os, "sched_getaffinity", return_value={16}, create=True), \
                 patch.object(bench, "ROOT", root), \
                 patch.object(bench, "run_streamed", return_value=1):
                with self.assertRaisesRegex(ValueError, "worker failed"):
                    bench.run(args)
            self.assertEqual(output.read_text(), "previous result")


if __name__ == "__main__":
    unittest.main()
