"""Driver accounting checks use synthetic records, not benchmark measurements."""
import contextlib
import importlib.util
import io
import json
from pathlib import Path
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

BENCHES = Path(__file__).resolve().parents[1] / "benches"
sys.path.insert(0, str(BENCHES))
SPEC = importlib.util.spec_from_file_location("public_matrix_bench", BENCHES / "public_matrix_bench.py")
bench = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(bench)


def records(system, threads=4, reps=2, warmups=1, log_n=14):
    meta = {"type": "metadata", "system": system, "mode": "single", "instances": 1,
            "log_gates": log_n, "threads": threads, "all_core_threads": threads, "seed": 97,
            "genuine_gate_count": 1 << log_n, "r1cs_rows": 4 << log_n, "r1cs_outer_blocks": 1,
            "C": "identity", "arithmetization": "native R1CS", "matrix_authentication": "public CSC",
            "permutation_proof": False, "baseline_skip": 7, "compression_kappa": 128,
            "pcs_packed_direct": True, "pcs_backend": "FLOCK native Ligerito Fast",
            "repetitions": reps, "warmups": warmups, "witness_words": max(1 << (log_n - 5), 1 << 15),
            "fixture_digest": "a" * 64, "witness_digest": "b" * 64, "matrix_digest": "c" * 64,
            "matrix_digest_scope": "public matrix recipe and dimensions", "public_io": [1, 0],
            "witness_claims": 3, "public_claims": 0, "witness_commitments": 1, "public_commitments": 0}
    rows = [meta]
    for index in range(reps + warmups):
        warmup = index < warmups
        scale = 100 if warmup else index - warmups + 1
        times = {key: float(scale) for key in bench.PROVER_PHASES}
        times.update(prover_ms=10 * scale * (1 if system == "mikros" else 2), witness_claims=3,
                     public_claims=0, native_pcs_claims=3, witness_commitments=1, public_commitments=0)
        rows.append({"type": "sample", "system": system, "mode": "single", "instances": 1,
                     "log_gates": log_n, "threads": threads, "seed": 97, "gates_per_instance": 1 << log_n,
                     "total_gates": 1 << log_n, "warmup": warmup, "repetition": index if warmup else index - warmups,
                     "times": times, "verify_ms": float(scale),
                     "verifier_times": {key: 0.1 * scale for key in bench.VERIFIER_PHASES},
                     "verified": True, "transcript_match": True, "pcs_config_match": True,
                     "proof_bytes": 4096, "proof_blake3": "d" * 64, "piop_blake3": "e" * 64,
                     "terminal_claims_blake3": "f" * 64, "witness_commitment_blake3": "0" * 64,
                     "witness_pcs_params": {"m": max(log_n + 2, 22), "profile": "fast", "log_inv_rate": 1,
                                            "merkle_hash": "blake3", "num_lanes": None, "log_batch_size": 6}})
    return rows


def encoded(rows):
    return "\n".join(json.dumps(row) for row in rows) + "\n"


def options():
    return bench.parse_options(["--worker-bin", __file__, "--log-n", "14", "--threads", "4",
                                "--reps", "2", "--warmups", "1"])


def reports():
    return [bench.parse_worker(encoded(records(system)), options(), system, 2048) for system in bench.SYSTEMS]


class PublicMatrixBenchTests(unittest.TestCase):
    def test_defaults_and_root_log_protection(self):
        args = bench.parse_options(["--worker-bin", __file__])
        self.assertEqual((args.log_n, args.threads, args.reps, args.warmups), (30, 64, 1, 0))
        self.assertEqual(args.output, bench.ROOT / "target/paper-results/circuit-sat-public-matrix-2-30.log")
        for name in ("single_2_25.log", "e2e_flock_sha256.log", "compare_hadamard_2_30.log"):
            with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                bench.parse_options(["--worker-bin", __file__, "--output", name])

    def test_argument_validation(self):
        for flags in (["--threads", "0"], ["--threads", "4", "--cpu", "0"], ["--log-n", "31"],
                      ["--log-n", "13"], ["--reps", "0"], ["--warmups", "-1"], ["--seed", "-1"]):
            with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                bench.parse_options(["--worker-bin", __file__, *flags])

    def test_checks_protocol_configuration_counts_and_threads(self):
        changes = {"all_core_threads": 1, "threads": 1, "baseline_skip": 6, "compression_kappa": 64,
                   "arithmetization": "binary PLONK", "matrix_authentication": "preprocessed PCS",
                   "permutation_proof": True, "r1cs_rows": 1 << 14, "C": "nonidentity",
                   "genuine_gate_count": 1 << 15, "witness_claims": 4, "public_claims": 1,
                   "public_commitments": 1, "witness_commitments": 2, "witness_digest": "invalid"}
        for key, value in changes.items():
            data = records("mikros")
            data[0][key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                bench.parse_worker(encoded(data), options(), "mikros", 2048)

    def test_requires_complete_verified_sample_and_nonoverlapping_times(self):
        mutations = (lambda row: row.update(verified=False), lambda row: row.update(transcript_match=False),
                     lambda row: row.update(threads=1), lambda row: row.update(warmup=False),
                     lambda row: row.update(total_gates=100), lambda row: row["times"].update(public_claims=1),
                     lambda row: row["times"].update(witness_commitments=2),
                     lambda row: row["times"].update(prover_ms=float("nan")),
                     lambda row: row["times"].update(lincheck_ms=10000),
                     lambda row: row["verifier_times"].update(lincheck_ms=10000),
                     lambda row: row["witness_pcs_params"].update(log_batch_size=3),
                     lambda row: row.update(public_pcs_params={}), lambda row: row.update(proof_bytes=5))
        for mutate in mutations:
            data = records("mikros")
            mutate(data[1])
            with self.assertRaises(ValueError):
                bench.parse_worker(encoded(data), options(), "mikros", 2048)

    def test_same_circuit_witness_matrices_commitment_and_pcs_are_required(self):
        for key in bench.MATCHED_FIELDS:
            data = reports()
            data[1]["metadata"][key] = "different"
            with self.subTest(key=key), self.assertRaisesRegex(ValueError, key):
                bench.build_summary(data)
        for key in ("witness_pcs_params", "witness_commitment_blake3"):
            data = reports()
            data[1]["samples"][1][key] = "different"
            with self.subTest(key=key), self.assertRaisesRegex(ValueError, key):
                bench.build_summary(data)

    def test_medians_exclude_warmups_and_include_verifier_phases(self):
        summary = bench.build_summary(reports())
        self.assertEqual(summary["mikros"]["prover_ms"]["median"], 15)
        self.assertEqual(summary["flock"]["prover_ms"]["median"], 30)
        self.assertEqual(summary["mikros"]["verifier_phase_medians_ms"]["lincheck_ms"], 0.15000000000000002)
        self.assertEqual(summary["mikros"]["ram_mib"], 2)
        self.assertEqual(summary["mikros"]["linear_reduction_ms"]["median"], 3)

    def test_linear_reduction_sums_each_sample_before_statistics(self):
        args = bench.parse_options(["--worker-bin", __file__, "--log-n", "14", "--threads", "4",
                                    "--reps", "3", "--warmups", "1"])
        data = []
        for system in bench.SYSTEMS:
            rows = records(system, reps=3)
            for index, row in enumerate(rows[2:]):
                row["times"].update(prover_ms=1000, linear_prepare_ms=100 if index == 0 else 0,
                                     lincheck_input_ms=100 if index == 1 else 0,
                                     lincheck_ms=100 if index == 2 else 0)
            data.append(bench.parse_worker(encoded(rows), args, system, 2048))
        summary = bench.build_summary(data)
        for system in bench.SYSTEMS:
            self.assertEqual(summary[system]["linear_reduction_ms"]["median"], 100)
            self.assertEqual(summary[system]["linear_reduction_ms"]["count"], 3)
            self.assertEqual(sum(summary[system]["prover_phase_medians_ms"][key] for key in
                                 ("linear_prepare_ms", "lincheck_input_ms", "lincheck_ms")), 0)

    def test_mock_run_inherits_multi_cpu_affinity_and_writes_anonymous_report(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            worker = root / "worker"
            worker.write_text("mock")
            output = root / "results/test.log"
            args = bench.parse_options(["--worker-bin", str(worker), "--time-bin", str(worker),
                                        "--log-n", "14", "--threads", "4", "--reps", "2", "--warmups", "1",
                                        "--output", str(output)])
            calls = []

            def fake_worker(command, **kwargs):
                self.assertIsNone(kwargs["cpu"])
                self.assertEqual(kwargs["env"]["RAYON_NUM_THREADS"], "4")
                self.assertEqual(kwargs["env"]["FLOCK_VERIFY_THREADS"], "4")
                self.assertEqual(command[command.index("--threads") + 1], "4")
                system = command[command.index("--system") + 1]
                calls.append(system)
                kwargs["stdout"].write(encoded(records(system)))
                Path(command[command.index("-o") + 1]).write_text("Maximum resident set size (kbytes): 2048\nExit status: 0\n")
                return 0

            with patch.object(bench.subprocess, "run", return_value=SimpleNamespace(returncode=0, stdout="GNU time")), \
                 patch.object(bench.os, "sched_getaffinity", return_value={1, 3, 5, 7}, create=True), \
                 patch.object(bench, "ROOT", root), patch.object(bench, "run_streamed", side_effect=fake_worker), \
                 patch("builtins.print"):
                doc = bench.run(args)
            self.assertEqual(calls, list(bench.SYSTEMS))
            self.assertEqual(doc["cpu_affinity"], [1, 3, 5, 7])
            text = output.read_text()
            self.assertIn("FLOCK/Mikros prover time: 2.000x", text)
            self.assertIn("Verifier phases", text)
            self.assertIn("Linear reduction including preparation", text)
            self.assertIn("FLOCK includes matrix folding", text)
            self.assertIn("public CSC evaluation", text)
            self.assertNotIn(directory, text)
            self.assertNotIn(directory, output.with_suffix(".json").read_text())

    def test_failed_worker_preserves_previous_report(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            worker = root / "worker"
            worker.write_text("mock")
            output = root / "test.log"
            output.write_text("previous result")
            args = bench.parse_options(["--worker-bin", str(worker), "--time-bin", str(worker),
                                        "--log-n", "14", "--threads", "1", "--output", str(output)])
            with patch.object(bench.subprocess, "run", return_value=SimpleNamespace(returncode=0, stdout="GNU time")), \
                 patch.object(bench.os, "sched_getaffinity", return_value={3}, create=True), \
                 patch.object(bench, "ROOT", root), patch.object(bench, "run_streamed", return_value=101):
                with self.assertRaisesRegex(ValueError, "worker failed"):
                    bench.run(args)
            self.assertEqual(output.read_text(), "previous result")


if __name__ == "__main__":
    unittest.main()
