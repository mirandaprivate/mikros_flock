"""Validate the 256-instance FRI driver with synthetic measurement records."""
import contextlib
import io
import json
from pathlib import Path
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest import mock

BENCHES = Path(__file__).resolve().parents[1] / "benches"
if str(BENCHES) not in sys.path:
    sys.path.insert(0, str(BENCHES))
import paper_bench as bench
import multi_hash_fri_bench as wrapper
from test_single_hash_fri_bench import records as single_records


def options(hash_name="sha256", *extra):
    return bench.parse_options(["--experiment", hash_name, "--worker-bin", "worker",
                               "--pcs", "fri", "--instances", "256", "--threads", "1",
                               "--reps", "1", "--warmups", "0", *extra])


def records(system="mikros", hash_name="sha256"):
    result = single_records(system, hash_name)
    meta = result[0]
    meta.update(instances=256, instance_mode="multiple", threads=1, all_core_threads=1)
    if system == "mikros":
        protocol = meta.pop("single_plonk")
        protocol["pcs_claims"] = 1
        protocol.update(instances=256, packed_groups=2, instance_packing_width=128,
                        gate_hadamard_invocations=2, permutation_proof=False)
        del protocol["public_pcs_params"]
        del protocol["public_table_words"]
        meta.update(arithmetization="binary packed R1CS", multiple_r1cs=protocol,
                    r1cs_c_identity=True, r1cs_k_log=14)
        meta.pop("mikros_wiring")
    return result


class MultiHashFriBenchTests(unittest.TestCase):
    def test_defaults_and_scope(self):
        default = bench.parse_options(wrapper.arguments(["--worker-bin", "worker"]))
        self.assertEqual((default.experiment, default.pcs, default.instances, default.threads,
                          default.reps, default.warmups), ("multiple", "fri", 256, 1, 10, 1))
        self.assertEqual(default.hash_output_dir, bench.ROOT.resolve())
        self.assertEqual(default.output.name, "multiple-hash-fri-vs-flock.log")
        for args in (("--pcs", "native"), ("--experiment", "single")):
            with self.subTest(args=args), contextlib.redirect_stderr(io.StringIO()), self.assertRaises((ValueError, SystemExit)):
                wrapper.arguments(["--worker-bin", "worker", *args])

    def test_root_refresh_is_limited_to_matching_multiple_hash(self):
        self.assertEqual(options().output, bench.ROOT / "e2e_flock_sha256.log")
        self.assertEqual(options("sha256", "--output", "e2e_flock_sha256.log").output.name,
                         "e2e_flock_sha256.log")
        for name in ("single_hash_compare.log", "e2e_flock_keccak.log", "compare_hadamard_2_30.log"):
            with self.subTest(name=name), contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                options("sha256", "--output", name)

    def test_requires_multiple_r1cs_and_complete_one_oracle_pcs(self):
        for system in ("mikros", "flock"):
            bench.parse_e2e_records(records(system), options(), system, 1, 0)
        for key, value in (("pcs_backend", bench.NATIVE_BACKEND), ("instance_mode", "single"),
                           ("arithmetization", "binary PLONK"), ("verifier_threads", 64),
                           ("multiple_r1cs", None), ("prepared_witness_blake3", "missing")):
            data = records()
            data[0][key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                bench.parse_e2e_records(data, options(), "mikros", 1, 0)
        for key, value in (("pcs_complete", False), ("pcs_claims", 2),
                           ("witness_words", 65536), ("witness_pcs_params", None),
                           ("packed_groups", 1), ("instance_packing_width", 256),
                           ("permutation_proof", True)):
            data = records()
            data[0]["multiple_r1cs"][key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                bench.parse_e2e_records(data, options(), "mikros", 1, 0)

    def test_rejects_failed_proof_and_invalid_fri_rate(self):
        for key in ("verified", "transcript_match"):
            data = records()
            data[1][key] = False
            with self.subTest(key=key), self.assertRaises(ValueError):
                bench.parse_e2e_records(data, options(), "mikros", 1, 0)
        data = records()
        data[0]["multiple_r1cs"]["pcs_profile"]["rate"] = 0.25
        with self.assertRaisesRegex(ValueError, "query/rate"):
            bench.parse_e2e_records(data, options(), "mikros", 1, 0)

    def test_worker_routes_fri_to_multiple_mikros_only(self):
        with tempfile.TemporaryDirectory() as directory:
            for system in ("mikros", "flock"):
                stdout = Path(directory) / (system + ".txt")
                stdout.write_text("\n".join(json.dumps(row) for row in records(system)), encoding="utf-8")
                runner = object.__new__(bench.Runner)
                runner.o = options()
                runner.run = mock.Mock(return_value=(stdout, 1024))
                runner.e2e(system, 0)
                command = runner.run.call_args.args[1]
                self.assertEqual("--pcs" in command, system == "mikros")
                self.assertNotIn("--wiring", command)

    def test_suite_only_refreshes_root_reports_after_every_hash_passes(self):
        for fail_hash in (None, "blake3"):
            with self.subTest(fail_hash=fail_hash), tempfile.TemporaryDirectory() as directory:
                base = Path(directory)
                published = base / "published"
                published.mkdir()
                for name in bench.HASHES:
                    (published / f"e2e_flock_{name}.log").write_text("previous " + name, encoding="utf-8")
                worker = base / "worker"
                worker.write_bytes(b"synthetic")

                def new_runner(opt):
                    folder = base / opt.experiment
                    folder.mkdir()
                    def measured(system, batch):
                        if opt.experiment == fail_hash:
                            raise ValueError("later hash failed")
                        data = records(system, opt.experiment)
                        return {"metadata": data[0], "samples": data[1:], "peak_rss_kib": 1024}
                    return SimpleNamespace(folder=folder, e2e=measured, cpu=0, commands=[], env=bench.environment(1))

                args = wrapper.arguments(["--worker-bin", str(worker), "--reps", "1", "--warmups", "0",
                                          "--output", str(base / "suite.log"), "--hash-output-dir", str(published)])
                with mock.patch.object(bench, "Runner", side_effect=new_runner), \
                     contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
                    if fail_hash:
                        with self.assertRaisesRegex(ValueError, "later hash failed"):
                            bench.main(args)
                    else:
                        bench.main(args)
                for name in bench.HASHES:
                    path = published / f"e2e_flock_{name}.log"
                    if fail_hash:
                        self.assertEqual(path.read_text(encoding="utf-8"), "previous " + name)
                        self.assertFalse(path.with_suffix(".json").exists())
                    else:
                        text = path.read_text(encoding="utf-8")
                        self.assertIn("256 instances", text)
                        self.assertIn("two 128-instance packed R1CS groups", text)
                        self.assertNotIn("copy reductions", text)
                        self.assertIn("256 independent instance(s)", text)
                        self.assertEqual((base / name / "previous.log").read_text(), "previous " + name)
                        self.assertEqual(bench.load(path.with_suffix(".json"))["raw_data"], str(path.with_suffix(".json")))


if __name__ == "__main__":
    unittest.main()
