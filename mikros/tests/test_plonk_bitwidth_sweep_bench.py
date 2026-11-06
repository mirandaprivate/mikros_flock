"""Sweep control-flow tests use synthetic proofs, never performance estimates as data."""
import contextlib
import csv
import io
import json
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "benches"))
sys.path.insert(0, str(Path(__file__).resolve().parent))
import plonk_bitwidth_sweep_bench as sweep
from test_plonk_bitwidth_bench import encoded, records


def fake_success(command, *, folder, on_start, **_):
    def arg(key):
        return int(command[command.index(key) + 1])
    rows = records(arg("--bits"), threads=arg("--threads"), log_n=arg("--log-gates"),
                   reps=arg("--repetitions"), warmups=arg("--warmups"))
    (folder / "stdout.txt").write_text(encoded(rows), encoding="utf-8")
    (folder / "stderr.txt").write_text("stage=verify_full_proof\n", encoding="utf-8")
    (folder / "resource.txt").write_text("Maximum resident set size (kbytes): 8192\nExit status: 0\n", encoding="utf-8")
    (folder / "memory.jsonl").write_text('{"elapsed_seconds":1,"available_gib":300}\n', encoding="utf-8")
    return {"exit_code": 0, "guarded": False, "minimum_available_gib": 300, "elapsed_seconds": 1}


class SweepTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.folder = Path(self.temporary.name)
        self.worker = self.folder / "worker"
        self.worker.write_text("synthetic executable identity", encoding="utf-8")

    def options(self, *flags):
        return sweep.parse_options(["--worker-bin", str(self.worker), "--time-bin", str(self.worker),
                                    "--out", str(self.folder / "results"), "--log-ns", "14", "--bits", "1,2", *flags])

    def run_sweep(self, args=None, **kwargs):
        with contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
            return sweep.run(args or self.options(), available=lambda: 400, affinity=list(range(64)),
                             launcher=kwargs.pop("launcher", fake_success), **kwargs)

    def test_default_grid_and_sorted_smoke_grid(self):
        args = sweep.parse_options(["--worker-bin", str(self.worker)])
        self.assertEqual(args.log_ns, [20, 22, 24, 26, 28, 30])
        self.assertEqual(args.bits, [1, 2, 4, 8, 16, 32, 64, 128])
        self.assertEqual((args.threads, args.reps, args.warmups, args.seed, args.reserve_gib), (64, 1, 0, 97, 16))
        self.assertEqual(args.out, sweep.ROOT / "target/paper-results/plonk-bitwidth-sweep")
        args = self.options("--log-ns", "20,14", "--bits", "128,1")
        self.assertEqual((args.log_ns, args.bits), ([14, 20], [1, 128]))

    def test_invalid_grid_memory_configuration_and_output(self):
        for flags in (("--log-ns", "20,20"), ("--log-ns", "13"), ("--bits", "3"), ("--bits", "1,1"),
                      ("--reserve-gib", "0"), ("--reserve-gib", "nan"), ("--memory-budget-gib", "inf"),
                      ("--poll-seconds", "0"), ("--out", str(sweep.ROOT)), ("--threads", "0")):
            with self.subTest(flags=flags), contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                self.options(*flags)

    def test_memory_model_tracks_tag_transition_and_wide_fields(self):
        n30 = {bits: sweep.memory_estimate(30, bits) for bits in (1, 32, 64, 128)}
        self.assertEqual(n30[32]["physical_gib"], 358)
        self.assertEqual(n30[64]["physical_gib"], 716)
        self.assertIsNone(n30[128]["physical_gib"])
        self.assertIsNone(n30[128]["estimated_gib"])
        self.assertEqual(sweep.memory_estimate(26, 1)["physical_gib"], 13.265625)
        self.assertEqual(sweep.memory_estimate(26, 2)["physical_gib"], 13.390625)
        self.assertEqual(n30[1]["estimated_gib"], n30[1]["physical_gib"] + 1)
        self.assertGreater(n30[1]["physical_gib"], 16 * sweep.memory_estimate(26, 1)["physical_gib"])

    def test_missing_pcs_is_distinct_from_invalid_existing_config(self):
        shape = sweep.single.packing_geometry(30, 128)
        params, missing = sweep.pcs_preflight(shape)
        self.assertIn("public: m40_fast.toml", missing)
        with tempfile.TemporaryDirectory() as directory, patch.object(sweep.single, "CONFIG_ROOT", Path(directory)):
            (Path(directory) / "m22_fast.toml").write_text('initial_k = 3\nfield = "wrong"\n')
            with self.assertRaisesRegex(ValueError, "invalid native PCS configuration"):
                sweep.pcs_preflight(sweep.single.packing_geometry(14, 1))

    def test_success_preserves_raw_and_complete_phases(self):
        args = self.options()
        doc, code = self.run_sweep(args)
        self.assertEqual(code, 0)
        self.assertTrue(doc["completed"])
        self.assertEqual([case["status"] for case in doc["cases"]], ["ok", "ok"])
        stored = json.loads((args.out / "checkpoint.json").read_text())
        self.assertEqual(stored, doc)
        for case in doc["cases"]:
            self.assertEqual(case["metrics"]["prover_ms"], 10)
            self.assertEqual(case["metrics"]["ram_mib"], 8)
            self.assertEqual(len(case["artifacts_sha256"]), 4)
            self.assertEqual(set(case["summary"]["prover_phases_ms"]), set(sweep.single.PROVER_PHASES))
            self.assertEqual(case["report"]["samples"][0]["times"]["native_pcs_claims"], 2)
        log = (args.out / "summary.log").read_text()
        self.assertIn("diagnostic, not E2E", log)
        self.assertIn("public opening is online", log)
        self.assertFalse((args.out / ".sweep.lock").exists())

    def test_capacity_skip_is_null_csv_empty_and_no_worker(self):
        launch = Mock(side_effect=AssertionError("must not launch"))
        doc, code = self.run_sweep(self.options("--memory-budget-gib", "1"), launcher=launch)
        self.assertEqual(code, 0)
        for case in doc["cases"]:
            self.assertEqual(case["status"], "memory_skipped")
            self.assertEqual(case["metrics"], sweep.empty_metrics())
            self.assertIn("not measured OOM", case["reason"])
        with (self.folder / "results/results.csv").open(newline="") as stream:
            for row in csv.DictReader(stream):
                self.assertTrue(all(row[key] == "" for key in sweep.METRICS))
        launch.assert_not_called()

    def test_unsupported_precedes_memory_capacity(self):
        launch = Mock(side_effect=AssertionError("must not launch"))
        doc, code = self.run_sweep(self.options("--log-ns", "30", "--bits", "128", "--memory-budget-gib", "1"), launcher=launch)
        self.assertEqual((code, doc["cases"][0]["status"]), (0, "pcs_unsupported"))
        self.assertIsNone(doc["cases"][0]["available_gib"])

    def test_guarded_and_signal_exit_do_not_become_oom_measurements(self):
        for guarded, expected_status, code in ((False, "error", 1), (True, "guarded", 0)):
            with self.subTest(guarded=guarded):
                args = self.options("--out", str(self.folder / str(guarded)))
                launch = Mock(return_value={"exit_code": 137, "guarded": guarded, "minimum_available_gib": 10, "elapsed_seconds": 1})
                doc, actual = self.run_sweep(args, launcher=launch)
                self.assertEqual(actual, code)
                self.assertEqual([case["status"] for case in doc["cases"]], [expected_status, expected_status if guarded else "pending"])
                self.assertEqual(doc["cases"][0]["metrics"], sweep.empty_metrics())
                self.assertEqual(doc["completed"], guarded)
                self.assertTrue((args.out / doc["cases"][0]["raw_directory"] / "resource.txt").exists())

    def test_invalid_full_proof_is_error_with_raw_preserved(self):
        def invalid(command, **kwargs):
            result = fake_success(command, **kwargs)
            path = kwargs["folder"] / "stdout.txt"
            data = [json.loads(line) for line in path.read_text().splitlines()]
            data[1]["verified"] = False
            path.write_text(encoded(data))
            return result
        doc, code = self.run_sweep(launcher=invalid)
        self.assertEqual(code, 1)
        self.assertEqual(doc["cases"][0]["status"], "error")
        self.assertIn("verification failed", doc["cases"][0]["reason"])
        self.assertEqual(len(doc["cases"][0]["artifacts_sha256"]), 4)

    def test_resume_revalidates_success_and_refuses_changed_raw_or_binary(self):
        args = self.options()
        doc, code = self.run_sweep(args)
        args.resume = True
        launch = Mock(side_effect=AssertionError("successful case must be reused"))
        again, code = self.run_sweep(args, launcher=launch)
        self.assertEqual(code, 0)
        self.assertEqual([case["metrics"] for case in again["cases"]], [case["metrics"] for case in doc["cases"]])
        launch.assert_not_called()
        raw = args.out / doc["cases"][0]["raw_directory"] / "stdout.txt"
        original = raw.read_bytes()
        raw.write_bytes(original + b"\n")
        with self.assertRaisesRegex(ValueError, "raw artifact hash mismatch"):
            self.run_sweep(args)
        raw.write_bytes(original)
        self.worker.write_text("changed executable")
        with self.assertRaisesRegex(ValueError, "resume executable"):
            self.run_sweep(args)

    def test_refuses_live_old_worker_and_configuration_change(self):
        args = self.options()
        doc, _ = self.run_sweep(args)
        args.resume = True
        args.seed += 1
        with self.assertRaisesRegex(ValueError, "resume configuration"):
            self.run_sweep(args)
        args.seed -= 1
        doc["cases"][0]["active_pid"] = 123
        with patch.object(sweep, "process_alive", return_value=True), self.assertRaisesRegex(ValueError, "previous worker"):
            sweep.validate_resume(doc, args, doc["identity"])

    def test_resume_retries_only_failed_case_preserving_previous_attempt(self):
        args = self.options()
        def fail_second(command, **kwargs):
            if command[command.index("--bits") + 1] == "2":
                return {"exit_code": 101, "guarded": False, "minimum_available_gib": 300, "elapsed_seconds": 1}
            return fake_success(command, **kwargs)
        first, code = self.run_sweep(args, launcher=fail_second)
        self.assertEqual(code, 1)
        old_raw = args.out / first["cases"][1]["raw_directory"]
        old_hashes = sweep.raw_hashes(old_raw)
        args.resume = True
        launcher = Mock(side_effect=fake_success)
        finished, code = self.run_sweep(args, launcher=launcher)
        self.assertEqual(code, 0)
        launcher.assert_called_once()
        self.assertEqual([len(case["attempts"]) for case in finished["cases"]], [1, 2])
        self.assertEqual(sweep.raw_hashes(old_raw), old_hashes)
        self.assertTrue(finished["completed"])

    def test_guarded_case_continues_then_resume_validates_and_does_not_retry_it(self):
        args = self.options()
        def guard_first(command, **kwargs):
            if command[command.index("--bits") + 1] == "1":
                return {"exit_code": -15, "guarded": True, "minimum_available_gib": 10, "elapsed_seconds": 1}
            return fake_success(command, **kwargs)
        doc, code = self.run_sweep(args, launcher=guard_first)
        self.assertEqual(code, 0)
        self.assertTrue(doc["completed"])
        self.assertEqual([case["status"] for case in doc["cases"]], ["guarded", "ok"])
        args.resume = True
        launcher = Mock(side_effect=AssertionError("guarded cases must not be retried automatically"))
        reused, code = self.run_sweep(args, launcher=launcher)
        self.assertEqual(code, 0)
        launcher.assert_not_called()
        self.assertEqual(reused["cases"][0]["metrics"], sweep.empty_metrics())
        (args.out / doc["cases"][0]["raw_directory"] / "memory.jsonl").write_text("tampered")
        with self.assertRaisesRegex(ValueError, "raw artifact hash mismatch"):
            self.run_sweep(args)

    def test_no_overwrite_without_resume(self):
        args = self.options()
        self.run_sweep(args)
        with self.assertRaisesRegex(ValueError, "results exist"):
            self.run_sweep(args)

    def test_environment_receipt_does_not_publish_private_environment(self):
        with patch.dict(sweep.os.environ, {"SECRET_TOKEN": "do-not-record", "PRIVATE_PATH": "private-home"}):
            doc, _ = self.run_sweep()
        self.assertNotIn("do-not-record", json.dumps(doc))
        self.assertNotIn("private-home", json.dumps(doc))

    def test_guard_only_signals_owned_group_and_escalates(self):
        process = Mock(pid=4321)
        process.poll.return_value = None
        process.wait.side_effect = [subprocess.TimeoutExpired("worker", 5), 0]
        with patch.object(sweep.os, "killpg", create=True) as kill, patch.object(signal, "SIGKILL", 9, create=True):
            sweep.stop_owned_group(process)
        self.assertEqual(kill.call_args_list[0].args, (4321, signal.SIGTERM))
        self.assertEqual(kill.call_args_list[1].args, (4321, 9))
        self.assertTrue(all(call.args[0] == 4321 for call in kill.call_args_list))

    def test_live_memory_poll_stops_own_session_and_preserves_streams(self):
        process = Mock(pid=7654)
        process.stdout = io.StringIO('{"type":"metadata"}\n')
        process.stderr = io.StringIO('stage=online_copy_piop\n')
        process.poll.return_value = None
        process.wait.return_value = -15
        started = Mock()
        def stop(owned):
            self.assertIs(owned, process)
            process.poll.return_value = -15
            return True
        values = iter((400, 10))
        with patch.object(sweep.os, "name", "posix"), patch.object(sweep.os, "killpg", create=True), \
             patch.object(sweep.subprocess, "Popen", return_value=process) as popen, \
             patch.object(sweep, "stop_owned_group", side_effect=stop) as kill, \
             contextlib.redirect_stdout(io.StringIO()):
            result = sweep.run_guarded(["synthetic-worker"], folder=self.folder, env={}, reserve_gib=16,
                                       poll_seconds=1, available=lambda: next(values, 10), on_start=started, label="test")
        self.assertTrue(popen.call_args.kwargs["start_new_session"])
        started.assert_called_once_with(7654)
        kill.assert_called_once_with(process)
        self.assertTrue(result["guarded"])
        self.assertEqual(result["minimum_available_gib"], 10)
        self.assertIn("online_copy_piop", (self.folder / "stderr.txt").read_text())
        self.assertIn("metadata", (self.folder / "stdout.txt").read_text())
        self.assertEqual(json.loads((self.folder / "memory.jsonl").read_text())["available_gib"], 10)

    def test_dead_group_does_not_falsely_claim_guard_killed_worker(self):
        process = Mock(pid=4321)
        with patch.object(sweep.os, "killpg", side_effect=ProcessLookupError, create=True):
            self.assertFalse(sweep.stop_owned_group(process))
        process.wait.assert_not_called()


if __name__ == "__main__":
    unittest.main()
