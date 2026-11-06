"""Synthetic FRI benchmark accounting tests; these do not measure a prover."""
import contextlib
import copy
import io
import json
from pathlib import Path
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "benches"))
import plonk_fri_bench as bench
import plonk_raafold_bench as raa


def options(*extra):
    return bench.parse_options(["--worker-bin", __file__, "--log-n", "14", "--threads", "4",
                                "--reps", "2", "--warmups", "1", *extra])


def records():
    result = [{"type": "metadata", "system": "mikros", "mode": "single", "arithmetization": "PLONK",
               "pcs_backend": bench.BACKEND, "log_gates": 14, "instances": 1,
               "gates_per_instance": 1 << 14, "total_gates": 1 << 14,
               "threads": 4, "all_core_threads": 4, "seed": 97, "repetitions": 2, "warmups": 1,
               "pcs_profile":dict(bench.SPEC.required_profile),
               "witness_pcs_parameters":{"words":32768,"fold_arities":[4,4]},
               "public_pcs_parameters":{"words":32768,"fold_arities":[4,4]},
               **bench.packing_geometry(14, 1)}]
    for index, scale in enumerate((100, 1, 2)):
        times = {key: float(scale) for key in bench.PROVER_PHASES}
        times.update(prover_ms=10.0 * scale, witness_claims=11, public_claims=53, pcs_claims=2)
        result.append({"type": "sample", "value_bits": 1, "warmup": index == 0,
                       "repetition": 0 if index == 0 else index - 1,
                       "verified": True, "transcript_match": True, "pcs_complete": True,
                       "proof_bytes": 4096 + index, "witness_pcs_proof_bytes": 1000,
                       "public_pcs_proof_bytes": 2000, "proof_blake3": "a" * 64,
                       "verify_ms": float(scale), "peak_rss_kib": 8192, "offline_ms": 5.0,
                       "times": times, "verifier_times": {key: 0.1 * scale for key in bench.VERIFIER_PHASES}})
    return result


def encode(rows):
    return "\n".join(json.dumps(row) for row in rows) + "\n"


class PlonkFriBenchTests(unittest.TestCase):
    def test_defaults_and_backend_isolation(self):
        args = bench.parse_options(["--worker-bin", __file__])
        self.assertEqual((args.log_n, args.threads, args.reps, args.warmups), (26, 64, 1, 0))
        self.assertEqual(args.out, bench.ROOT / "target/paper-results/plonk-fri-2-26")
        self.assertEqual(raa.parse_options(["--worker-bin", __file__]).log_n, 30)
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
            bench.parse_options(["--worker-bin", __file__, "--experiment", "plonk-raafold"])
        with self.assertRaises(ValueError):
            raa.parse_worker(encode(records()), options(), 2048)
        wrong = records()
        wrong[0]["pcs_backend"] = raa.BACKEND
        with self.assertRaises(ValueError):
            bench.parse_worker(encode(wrong), options(), 2048)

    def test_shared_accounting_keeps_offline_public_commit_excluded(self):
        report = bench.parse_worker(encode(records()), options(), 2048)
        summary = bench.build_summary(report)
        self.assertEqual(summary["prover_ms"]["median"], 15)
        self.assertEqual(summary["ram_mib"], 2)
        self.assertEqual(summary["prover_phases_ms"]["public_pcs_open_ms"]["median"], 1.5)
        self.assertNotIn("offline_ms", summary["prover_phases_ms"])
        self.assertEqual(summary["proof_bytes"]["count"], 2)

    def test_requires_complete_proof_for_both_groups(self):
        mutations = (lambda row: row.pop("pcs_complete"), lambda row: row.update(pcs_complete=False),
                     lambda row: row.update(pcs_complete=1), lambda row: row.update(verified=False),
                     lambda row: row.update(transcript_match=False),
                     lambda row: row.pop("witness_pcs_proof_bytes"), lambda row: row.pop("public_pcs_proof_bytes"),
                     lambda row: row.update(witness_pcs_proof_bytes=0), lambda row: row.update(public_pcs_proof_bytes=-1),
                     lambda row: row.update(public_pcs_proof_bytes=True),
                     lambda row: row.update(public_pcs_proof_bytes=3500),
                     lambda row: row["times"].update(pcs_claims=1),
                     lambda row: row["times"].update(public_pcs_open_ms=10000),
                     lambda row: row["verifier_times"].pop("public_pcs_ms"))
        for index, mutate in enumerate(mutations):
            rows = records()
            mutate(rows[1])
            with self.subTest(index=index), self.assertRaises(ValueError):
                bench.parse_worker(encode(rows), options(), 2048)

    def test_rejects_changed_security_profile_or_missing_parameter_audit(self):
        for key, value in (("rate", 0.25), ("native_queries", 1), ("field_bits", 256),
                           ("zero_knowledge", True), ("point_order", "monomial"),
                           ("query_security_scope", "complete 100-bit soundness")):
            rows = records()
            rows[0]["pcs_profile"][key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                bench.parse_worker(encode(rows), options(), 2048)
        for key in ("pcs_profile", "witness_pcs_parameters", "public_pcs_parameters"):
            rows = records()
            rows[0].pop(key)
            with self.subTest(key=key), self.assertRaises(ValueError):
                bench.parse_worker(encode(rows), options(), 2048)

    def test_rejects_wrong_geometry_sampling_and_invalid_counts(self):
        for key, value in {"witness_words": 1, "public_words": 1, "instances": 256,
                           "value_bits": 2, "all_core_threads": 1, "seed": 0}.items():
            rows = records()
            rows[0][key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                bench.parse_worker(encode(rows), options(), 2048)
        for flags in (("--log-n", "31"), ("--threads", "0"), ("--reps", "0"),
                      ("--warmups", "-1"), ("--out", str(bench.ROOT))):
            with self.subTest(flags=flags), contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                bench.parse_options(["--worker-bin", __file__, *flags])

    def test_profiling_is_not_inherited_by_formal_measurements(self):
        with patch.dict(bench.os.environ, {"RAAFOLD_PROFILE_STAGES": "1", "ADDITIVE_FRI_PROFILE_STAGES": "1"}):
            env = bench.worker_environment(64)
            self.assertNotIn("RAAFOLD_PROFILE_STAGES", env)
            self.assertNotIn("ADDITIVE_FRI_PROFILE_STAGES", env)
            self.assertEqual(env["RAYON_NUM_THREADS"], "64")

    def test_worker_orchestration_preserves_scope_and_backend_output(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            worker = root / "worker"
            worker.write_text("synthetic worker")
            args = options("--worker-bin", str(worker), "--time-bin", str(worker), "--out", str(root / "out"))

            def fake_worker(command, **kwargs):
                self.assertEqual(command[5:], ["--worker", "--log-gates", "14", "--threads", "4",
                                              "--repetitions", "2", "--warmups", "1", "--seed", "97"])
                self.assertEqual(kwargs["label"], "plonk-fri-2-14")
                kwargs["stdout"].write(encode(records()))
                kwargs["stderr"].write("stage=verify_full_proof\n")
                Path(command[3]).write_text("Maximum resident set size (kbytes): 2048\nExit status: 0\n")
                return 0

            with patch.object(bench.subprocess, "run", return_value=SimpleNamespace(returncode=0, stdout="GNU time")), \
                 patch.object(bench.os, "sched_getaffinity", return_value={1, 3, 5, 7}, create=True), \
                 patch.object(bench, "run_streamed", side_effect=fake_worker), patch("builtins.print"):
                doc = bench.run(args)
            self.assertEqual(doc["experiment"], "plonk-fri")
            self.assertEqual(doc["summary"]["prover_ms"]["count"], 2)
            self.assertEqual(set(doc["raw_sha256"]), {"stdout.txt", "stderr.txt", "resource.txt"})
            self.assertEqual(len(doc["shared_driver_sha256"]), 64)
            log = (args.out / "plonk_fri.log").read_text()
            self.assertIn("complete packed-MLE opening and additive-FRI verification", log)
            self.assertIn("Public PCS opening is online", log)
            self.assertIn("online field/layout conversion", log)
            self.assertIn("100-bit query-phase budget, excluding folding", log)
            self.assertNotIn("Decide", log)
            self.assertFalse((args.out / "plonk_raafold.log").exists())

    def test_failure_keeps_previous_result(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            worker = root / "worker"
            worker.write_text("synthetic worker")
            out = root / "out"
            out.mkdir()
            (out / "plonk_fri.log").write_text("previous verified result")
            args = options("--worker-bin", str(worker), "--time-bin", str(worker), "--out", str(out))
            with patch.object(bench.subprocess, "run", return_value=SimpleNamespace(returncode=0, stdout="GNU time")), \
                 patch.object(bench.os, "sched_getaffinity", return_value={1, 3, 5, 7}, create=True), \
                 patch.object(bench, "run_streamed", return_value=101):
                with self.assertRaisesRegex(ValueError, "worker failed"):
                    bench.run(args)
            self.assertEqual((out / "plonk_fri.log").read_text(), "previous verified result")


if __name__ == "__main__":
    unittest.main()
