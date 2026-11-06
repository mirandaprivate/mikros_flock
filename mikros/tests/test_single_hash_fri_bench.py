"""Small synthetic driver checks; no benchmark or cryptographic work is run."""
import contextlib
import io
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest import mock

BENCHES = Path(__file__).resolve().parents[1] / "benches"
if str(BENCHES) not in sys.path:
    sys.path.insert(0, str(BENCHES))
import paper_bench as bench
import single_hash_fri_bench as wrapper


def options(hash_name="sha256", *extra):
    return bench.parse_options(["--experiment", hash_name, "--worker-bin", "worker",
                               "--pcs", "fri", "--instances", "1", "--threads", "64",
                               "--reps", "1", "--warmups", "0", *extra])


def records(system="mikros", hash_name="sha256"):
    meta = {"record": "metadata", "system": system, "hash": hash_name, "instances": 1,
            "instance_mode": "single", "threads": 64, "all_core_threads": 64,
            "verifier_threads": 1, "baseline_skip": 7, "univariate_skip": 7,
            "pcs_backend": bench.FRI_BACKEND if system == "mikros" else bench.NATIVE_BACKEND,
            **{key: "a" * 64 for key in bench.SHARED_DIGESTS}}
    if system == "mikros":
        meta.update(mikros_wiring="packed", arithmetization="binary PLONK",
                    single_plonk={"pcs_backend": bench.FRI_BACKEND, "pcs_complete": True, "pcs_claims": 2,
                                  "pcs_profile": {"field_bits": 128, "packed_word_bits": 128, "zero_knowledge": False,
                                                  "protocol": "BaseFold MLE-check + additive FRI", "native_queries": 241,
                                                  "query_security_bits": 100, "log_inv_rate": 1, "rate": 0.5,
                                                  "query_security_scope": "FRI query phase only; excludes folding and other protocol errors"},
                                  "witness_words": 32768, "public_table_words": 32768,
                                  **{prefix + "_pcs_params": {"words": 32768, "log_words": 15,
                                                              "native_queries": 241, "log_inv_rate": 1}
                                     for prefix in ("witness", "public")}})
    row = {"record": "sample", "system": system, "hash": hash_name, "iteration": 0,
           "warmup": False, "prover_ms": 1.0, "verify_ms": 0.1, "proof_bytes": 4096,
           "verified": True, "transcript_match": True}
    return [meta, row]


class SingleHashFriBenchTests(unittest.TestCase):
    def test_worker_environment_clears_inherited_fri_diagnostics(self):
        inherited = {"ADDITIVE_FRI_PROFILE_STAGES": "1", "ADDITIVE_FRI_TEST_OVERRIDE": "enabled",
                     "FLOCK_NO_GRIND": "1", "MIKROS_PERMUTATION_PROFILE": "1", "RAYON_NUM_THREADS": "2",
                     "PATH": "/test/bin", "HOME": "/test/home"}
        with mock.patch.dict(bench.os.environ, inherited, clear=True):
            env = bench.environment(64)
        self.assertFalse(any(key.startswith("ADDITIVE_FRI_") for key in env))
        self.assertNotIn("FLOCK_NO_GRIND", env)
        self.assertNotIn("MIKROS_PERMUTATION_PROFILE", env)
        self.assertEqual((env["RAYON_NUM_THREADS"], env["FLOCK_VERIFY_THREADS"]), ("64", "64"))
        self.assertEqual((env["FLOCK_PCS_PACKED_DIRECT"], env["FLOCK_WP_UNINIT"]), ("1", "1"))
        self.assertEqual((env["PATH"], env["HOME"]), ("/test/bin", "/test/home"))

    def test_wrapper_defaults_and_explicit_overrides(self):
        default = bench.parse_options(wrapper.arguments(["--worker-bin", "worker"]))
        self.assertEqual((default.experiment, default.pcs, default.instances, default.threads,
                          default.reps, default.warmups), ("single", "fri", 1, 64, 10, 1))
        self.assertEqual(default.output, bench.ROOT / "target/paper-results/single-hash-fri-vs-flock.log")
        explicit = bench.parse_options(wrapper.arguments(["--worker-bin", "worker", "--threads=2",
                                                           "--reps", "3", "--warmups", "0"]))
        self.assertEqual((explicit.threads, explicit.reps, explicit.warmups), (2, 3, 0))
        with self.assertRaisesRegex(ValueError, "three single-instance hashes"):
            wrapper.arguments(["--worker-bin", "worker", "--experiment", "keccak"])

    def test_native_defaults_and_commands_remain_native(self):
        old = bench.parse_options(["--experiment", "single", "--worker-bin", "worker"])
        self.assertEqual((old.pcs, old.threads, old.output.name), ("native", 1, "single_hash_compare.log"))
        # Native records produced before backend metadata was added remain readable.
        native_records = records()
        native_records[0].pop("pcs_backend")
        native_records[0]["threads"] = native_records[0]["all_core_threads"] = 1
        old.experiment = "sha256"
        bench.parse_e2e_records(native_records, old, "mikros", 1, 0)

    def test_reject_unsupported_scope_and_root_log(self):
        invalid = [("--wiring", "public"),
                   ("--output", "single_hash_compare.log"),
                   ("--output", "e2e_flock_sha256.log"), ("--output", "compare_hadamard_2_30.log")]
        for args in invalid:
            with self.subTest(args=args), contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                options("sha256", *args)
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
            options("hadamard")

    def test_strict_single_backend_metadata(self):
        for system in ("mikros", "flock"):
            bench.parse_e2e_records(records(system), options(), system, 1, 0)
        invalid = (("pcs_backend", bench.NATIVE_BACKEND), ("instance_mode", "multiple"),
                   ("instances", 256), ("verifier_threads", 64), ("single_plonk", None),
                   ("single_plonk", {"pcs_backend": "wrong"}), ("arithmetization", "native R1CS"),
                   ("prepared_witness_blake3", "missing"), ("univariate_skip", 6))
        for key, value in invalid:
            data = records()
            data[0][key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                bench.parse_e2e_records(data, options(), "mikros", 1, 0)
        data = records("flock")
        data[0]["pcs_backend"] = bench.FRI_BACKEND
        with self.assertRaisesRegex(ValueError, "PCS backend mismatch"):
            bench.parse_e2e_records(data, options(), "flock", 1, 0)

    def test_reject_wrong_identity_failed_or_missing_proof(self):
        for key, value in (("system", "flock"), ("hash", "keccak"), ("verified", False),
                           ("transcript_match", False), ("warmup", True), ("iteration", 1)):
            data = records()
            data[1][key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                bench.parse_e2e_records(data, options(), "mikros", 1, 0)
        with self.assertRaises(ValueError):
            bench.parse_e2e_records(records()[:1], options(), "mikros", 1, 0)

    def test_reject_missing_or_inconsistent_fri_parameters(self):
        for key, value in (("pcs_profile", None), ("pcs_complete", False), ("pcs_claims", 1),
                           ("witness_pcs_params", None), ("public_table_words", 65536)):
            data = records()
            data[0]["single_plonk"][key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                bench.parse_e2e_records(data, options(), "mikros", 1, 0)
        for key, value in (("rate", 0.25), ("native_queries", 0), ("field_bits", 64),
                           ("zero_knowledge", True), ("query_security_scope", "whole protocol")):
            data = records()
            data[0]["single_plonk"]["pcs_profile"][key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                bench.parse_e2e_records(data, options(), "mikros", 1, 0)

    def test_worker_routes_fri_only_to_mikros(self):
        with tempfile.TemporaryDirectory() as directory:
            for system in ("mikros", "flock"):
                stdout = Path(directory) / (system + ".txt")
                stdout.write_text("\n".join(json.dumps(row) for row in records(system)), encoding="utf-8")
                runner = object.__new__(bench.Runner)
                runner.o = options()
                runner.run = mock.Mock(return_value=(stdout, 1024))
                report = runner.e2e(system, 0)
                command = runner.run.call_args.args[1]
                self.assertEqual("--pcs" in command, system == "mikros")
                if system == "mikros":
                    self.assertEqual(command[command.index("--pcs") + 1], "fri")
                self.assertEqual(report["samples"][0]["verified"], True)

    def test_report_distinguishes_backends(self):
        raw = [{"metadata": records(s)[0], "samples": [records(s)[1]], "peak_rss_kib": 2048}
               for s in ("mikros", "flock")]
        doc = {"experiment": "sha256", "instances": 1, "threads": 64, "mikros_pcs": "fri",
               "mikros_wiring": "packed", "measurement_scope": {}, "raw_data": "target/raw.json",
               **bench.build_e2e_summary(raw)}
        text = bench.render_report(doc)
        for phrase in ("Mikros FRI", "FLOCK skip-7", "additive FRI", "native Ligerito Fast", "1 instances"):
            self.assertIn(phrase, text)

    def test_different_statements_fail_before_publishing(self):
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            worker = folder / "worker"
            worker.write_bytes(b"synthetic")
            opt = options("sha256", "--worker-bin", str(worker))
            reports = []
            for system in ("mikros", "flock"):
                data = records(system)
                if system == "flock":
                    data[0]["public_digest_blake3"] = "b" * 64
                reports.append({"metadata": data[0], "samples": [data[1]], "peak_rss_kib": 1024})
            runner = mock.Mock(folder=folder)
            runner.e2e.side_effect = reports
            with mock.patch.object(bench, "Runner", return_value=runner), self.assertRaisesRegex(ValueError, "statements differ"):
                bench.collect_experiment(opt)
            self.assertEqual(bench.load(folder / "status.json")["state"], "failed")


if __name__ == "__main__":
    unittest.main()
