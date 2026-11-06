"""Synthetic accounting and orchestration tests; these do not measure a prover."""
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
import plonk_raafold_bench as bench


def options(*extra):
    return bench.parse_options(["--worker-bin", __file__, "--log-n", "14", "--threads", "4",
                                "--reps", "2", "--warmups", "1", *extra])


def records():
    meta = {"type": "metadata", "system": "mikros", "mode": "single", "arithmetization": "PLONK",
            "pcs_backend": bench.BACKEND, "log_gates": 14, "instances": 1,
            "gates_per_instance": 1 << 14, "total_gates": 1 << 14,
            "threads": 4, "all_core_threads": 4, "seed": 97, "repetitions": 2, "warmups": 1,
            **bench.packing_geometry(14, 1)}
    rows = [meta]
    for index, scale in enumerate((100, 1, 2)):
        times = {key: float(scale) for key in bench.PROVER_PHASES}
        times.update(prover_ms=10.0 * scale, witness_claims=11, public_claims=53, pcs_claims=2)
        rows.append({"type": "sample", "value_bits": 1, "warmup": index == 0,
                     "repetition": 0 if index == 0 else index - 1, "verified": True,
                     "transcript_match": True, "proof_bytes": 4096 + index,
                     "verify_ms": float(scale), "peak_rss_kib": 8192, "offline_ms": 5.0,
                     "times": times, "verifier_times": {key: 0.1 * scale for key in bench.VERIFIER_PHASES}})
    return rows


def encode(rows):
    return "\n".join(json.dumps(row) for row in rows) + "\n"


class PlonkRaaFoldBenchTests(unittest.TestCase):
    def test_defaults_and_fixed_binary_worker_interface(self):
        args = bench.parse_options(["--worker-bin", __file__, "--experiment", "plonk-raafold"])
        self.assertEqual((args.log_n, args.threads, args.reps, args.warmups, args.seed), (30, 64, 1, 0, 97))
        self.assertEqual(args.out, bench.ROOT / "target/paper-results/plonk-raafold-2-30")
        self.assertFalse(hasattr(args, "bits"))

    def test_rejects_bad_cli_before_running(self):
        for flags in (("--log-n", "31"), ("--threads", "0"), ("--threads", "257"),
                      ("--reps", "0"), ("--warmups", "-1"), ("--seed", "-1"),
                      ("--bits", "1"), ("--out", str(bench.ROOT))):
            with self.subTest(flags=flags), contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                bench.parse_options(["--worker-bin", __file__, *flags])

    def test_no_native_pcs_dependency_and_independent_rss_measurements(self):
        rows = records()
        with patch("plonk_bitwidth_bench.expected_pcs_params", side_effect=AssertionError("native PCS must not be read")):
            report = bench.parse_worker(encode(rows), options(), 2048)
        self.assertEqual(report["peak_rss_kib"], 2048)
        self.assertEqual(report["samples"][0]["peak_rss_kib"], 8192)
        summary = bench.build_summary(report)
        self.assertEqual(summary["ram_mib"], 2)
        self.assertEqual(summary["prover_ms"]["median"], 15)
        self.assertEqual(summary["proof_bytes"]["median"], 4097.5)
        self.assertEqual(summary["prover_phases_ms"]["commit_ms"]["median"], 1.5)

    def test_rejects_wrong_backend_geometry_and_pool(self):
        bad = {"pcs_backend": "FLOCK native Ligerito Fast", "system": "flock", "arithmetization": "R1CS",
               "instances": 128, "value_bits": 2, "total_gates": 100, "all_core_threads": 1,
               "witness_words": 1, "public_words": 1, "seed": 98, "warmups": True}
        for key, value in bad.items():
            rows = records()
            rows[0][key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                bench.parse_worker(encode(rows), options(), 2048)

    def test_rejects_incomplete_samples_and_non_json_output(self):
        for rows in ([], records()[1:], records()[:-1], records() + [records()[1]], [records()[0], []]):
            with self.subTest(rows=len(rows)), self.assertRaises(ValueError):
                bench.parse_worker(encode(rows), options(), 2048)
        with self.assertRaises(ValueError):
            bench.parse_worker(encode(records()) + "progress is not a JSON sample\n", options(), 2048)

    def test_rejects_unverified_proofs_and_double_counted_timing(self):
        mutations = (lambda r: r.update(verified=False), lambda r: r.update(verified=1),
                     lambda r: r.update(transcript_match=False), lambda r: r.update(warmup=False),
                     lambda r: r.update(repetition=99), lambda r: r.update(proof_bytes=0),
                     lambda r: r.update(verify_ms=float("inf")), lambda r: r.update(offline_ms=-1),
                     lambda r: r["times"].update(pcs_claims=1), lambda r: r["times"].update(witness_claims=0),
                     lambda r: r["times"].update(public_claims=52), lambda r: r["times"].pop("copy_prepare_ms"),
                     lambda r: r["times"].update(copy_prepare_ms=-1), lambda r: r["times"].update(prover_ms=float("nan")),
                     lambda r: r["times"].update(public_pcs_open_ms=10000),
                     lambda r: r["verifier_times"].update(witness_pcs_ms=10000),
                     lambda r: r["verifier_times"].pop("public_pcs_ms"))
        for index, mutate in enumerate(mutations):
            rows = records()
            mutate(rows[1])
            with self.subTest(index=index), self.assertRaises(ValueError):
                bench.parse_worker(encode(rows), options(), 2048)

    def test_optional_grinding_and_profile_counters_are_checked(self):
        rows = records()
        rows[0]["witness_pcs_params"] = {"basis": "equality", "logical_element_bits": 128}
        for row in rows[1:]:
            row["witness_pcs_params"] = copy.deepcopy(rows[0]["witness_pcs_params"])
            row["piop_grinding_ms"] = {"gate_ms": 0, "copy_ms": 0, "copy_initial_ms": 0, "opening_ms": 0, "total_ms": 0}
            row["proof_blake3"] = "a" * 64
        bench.parse_worker(encode(rows), options(), 2048)
        for mutate in (lambda r: r["piop_grinding_ms"].update(total_ms=1),
                       lambda r: r["witness_pcs_params"].update(basis="monomial"),
                       lambda r: r.update(proof_blake3="invalid")):
            changed = copy.deepcopy(rows)
            mutate(changed[1])
            with self.assertRaises(ValueError):
                bench.parse_worker(encode(changed), options(), 2048)

    def test_resource_requires_successful_gnu_time_measurement(self):
        self.assertEqual(bench.parse_resource("Maximum resident set size (kbytes): 2048\nExit status: 0\n"), 2048)
        for text in ("", "Maximum resident set size (kbytes): 0\nExit status: 0\n",
                     "Maximum resident set size (kbytes): 2048\nExit status: 101\n"):
            with self.assertRaises(ValueError):
                bench.parse_resource(text)

    def test_worker_environment_disables_inherited_raafold_profiling(self):
        with patch.dict(bench.os.environ, {"RAAFOLD_PROFILE_STAGES": "1"}):
            env = bench.worker_environment(64)
            self.assertNotIn("RAAFOLD_PROFILE_STAGES", env)
            self.assertEqual(env["RAYON_NUM_THREADS"], "64")
            self.assertEqual(bench.os.environ["RAAFOLD_PROFILE_STAGES"], "1")

    def test_worker_command_reports_raw_hashes_and_full_scope(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            worker = root / "worker"
            worker.write_text("synthetic worker")
            args = options("--worker-bin", str(worker), "--time-bin", str(worker), "--out", str(root / "out"))
            commands = []

            def fake_worker(command, **kwargs):
                commands.append(command)
                self.assertNotIn("--bits", command)
                self.assertEqual(command[5:], ["--worker", "--log-gates", "14", "--threads", "4",
                                              "--repetitions", "2", "--warmups", "1", "--seed", "97"])
                self.assertIsNone(kwargs["cpu"])
                self.assertEqual(kwargs["env"]["RAYON_NUM_THREADS"], "4")
                kwargs["stdout"].write(encode(records()))
                kwargs["stderr"].write("stage=verify_full_proof\n")
                Path(command[3]).write_text("Maximum resident set size (kbytes): 2048\nExit status: 0\n")
                return 0

            with patch.object(bench.subprocess, "run", return_value=SimpleNamespace(returncode=0, stdout="GNU time")), \
                 patch.object(bench.os, "sched_getaffinity", return_value={1, 3, 5, 7}, create=True), \
                 patch.object(bench, "run_streamed", side_effect=fake_worker), patch("builtins.print"):
                doc = bench.run(args)
            self.assertEqual(len(commands), 1)
            self.assertEqual(doc["cpu_affinity"], [1, 3, 5, 7])
            self.assertEqual(set(doc["raw_sha256"]), {"stdout.txt", "stderr.txt", "resource.txt"})
            for name in ("plonk_raafold.log", "plonk_raafold.json"):
                text = (args.out / name).read_text()
                self.assertNotIn(directory, text)
            text = (args.out / "plonk_raafold.log").read_text()
            self.assertIn("complete opening and Decide", text)
            self.assertIn("online field/layout conversion", text)
            self.assertIn("Public PCS opening is online", text)
            self.assertIn("2 measured proof(s)", text)
            self.assertTrue(list(args.out.glob("runs/*/stderr.txt")))

    def test_failure_preserves_previous_report_and_raw_diagnostics(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            worker = root / "worker"
            worker.write_text("synthetic worker")
            out = root / "out"
            out.mkdir()
            (out / "plonk_raafold.log").write_text("previous verified result")
            args = options("--worker-bin", str(worker), "--time-bin", str(worker), "--out", str(out))
            with patch.object(bench.subprocess, "run", return_value=SimpleNamespace(returncode=0, stdout="GNU time")), \
                 patch.object(bench.os, "sched_getaffinity", return_value={1, 3, 5, 7}, create=True), \
                 patch.object(bench, "run_streamed", return_value=101):
                with self.assertRaisesRegex(ValueError, "worker failed"):
                    bench.run(args)
            self.assertEqual((out / "plonk_raafold.log").read_text(), "previous verified result")
            self.assertTrue(list(out.glob("runs/*/stdout.txt")))


if __name__ == "__main__":
    unittest.main()
