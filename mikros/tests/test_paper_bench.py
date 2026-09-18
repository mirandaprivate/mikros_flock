"""Small, deterministic checks for the Cargo bench report driver.

Run with: python -m unittest discover -s mikros/tests -p test_paper_bench.py
The fixtures are synthetic; they do not stand in for benchmark measurements.
"""

import importlib.util
import contextlib
import io
from pathlib import Path
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest import mock


DRIVER = Path(__file__).resolve().parents[1] / "benches" / "paper_bench.py"
SPEC = importlib.util.spec_from_file_location("paper_bench", DRIVER)
bench = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = bench
SPEC.loader.exec_module(bench)


def e2e_report(system, values, peak_rss_kib, batch=0, warmups=1):
    rows = []
    for iteration, value in enumerate([999.0] * warmups + list(values)):
        rows.append({
            "record": "sample", "system": system, "hash": "sha256",
            "iteration": iteration, "warmup": iteration < warmups,
            "prover_ms": value, "verify_ms": value / 10,
            "proof_bytes": 8192 if system == "mikros" else 4096,
            "verified": True, "transcript_match": True,
        })
    return {
        "metadata": {"system": system, "hash": "sha256", "threads": 1,
                     "instances": 256, "baseline_skip": 7},
        "samples": rows, "peak_rss_kib": peak_rss_kib, "batch": batch,
    }


def hadamard_row(system, value, run=1, hwm=None):
    row = {
        "system": system, "log_n": 30, "run": run,
        "prover_ms": value, "verifier_ms": value / 100,
        "payload_field_bytes": 4800, "vm_hwm_after_prove_kib": hwm,
        "verifier_passed": True, "transcript_matched": True,
        "source_claims_verified": True,
    }
    if system == "binius_switchover":
        row.update(proof_bytes=1504, payload_field_bytes=1504,
                   verified=True, transcript_match=True, source_checked=True)
    else:
        row["bincode_bytes"] = 4888 if system == "pairreuse" else 4912
    return row


def hadamard_fixture():
    systems = ["pairreuse", "flock128", "binius_switchover"]
    formal = [{
        "metadata": {"threads": 1, "flock_skip_rounds": 7, "kappa": 128},
        "warmups": [hadamard_row(s, 999.0, run=0) for s in systems],
        "samples": [hadamard_row(s, value, run=i + 1)
                    for s in systems for i, value in enumerate([2.0, 8.0])],
    }]
    cold = [{
        "metadata": {"threads": 1}, "warmups": [],
        "samples": [hadamard_row(s, 50.0, run=0, hwm=hwm)],
        # Whole-process RSS includes source checks and must not enter this metric.
        "peak_rss_kib": 999 * 1024,
    } for s in systems for hwm in [1024, 4096, 2048]]
    return formal, cold


class PaperBenchTests(unittest.TestCase):
    def test_single_suite_defaults_to_permutation_with_separate_public_log(self):
        args = ["--experiment", "single", "--worker-bin", "worker"]
        options = bench.parse_options(args)
        self.assertEqual((options.instances, options.threads, options.wiring), (1, 1, "packed"))
        self.assertEqual((options.reps, options.warmups, options.batches), (10, 1, 1))
        self.assertEqual(options.output.name, "single_hash_compare.log")
        packed = bench.parse_options(args + ["--wiring", "packed"])
        self.assertEqual(packed.output, options.output)
        public = bench.parse_options(args + ["--wiring", "public"])
        self.assertEqual(public.output.name, "single_hash_public_wiring_compare.log")
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
            bench.parse_options(args + ["--instances", "256"])

    def test_single_suite_publishes_three_hashes_and_preserves_output_on_later_failure(self):
        for fail_hash, wiring in ((None, "packed"), (None, "public"), ("keccak", "packed")):
            with self.subTest(fail_hash=fail_hash, wiring=wiring), tempfile.TemporaryDirectory() as directory:
                base = Path(directory)
                output = base / "suite.log"
                output.write_text("previous suite\n", encoding="utf-8")
                output.with_suffix(".json").write_text('{"previous": true}\n', encoding="utf-8")
                worker = base / "worker"
                worker.write_bytes(b"synthetic worker")
                calls = []

                def new_runner(options):
                    folder = base / options.experiment
                    folder.mkdir()
                    self.assertEqual((options.instances, options.threads, options.wiring), (1, 1, wiring))
                    self.assertEqual((options.reps, options.warmups, options.batches), (2, 1, 1))
                    if options.experiment != "single":
                        self.assertEqual(options.cpu, 2)

                    def measured(system, batch):
                        calls.append((options.experiment, system))
                        if options.experiment == fail_hash:
                            raise RuntimeError("later hash failed")
                        report = e2e_report(system, [2.0, 6.0], 2048, batch)
                        report["metadata"].update({key: options.experiment for key in bench.SHARED_DIGESTS})
                        report["metadata"].update(hash=options.experiment, instances=1, mikros_wiring=wiring)
                        return report

                    return SimpleNamespace(folder=folder, e2e=measured, cpu=2, commands=[], env=bench.environment(1))

                stdout = io.StringIO()
                with mock.patch.object(bench, "Runner", side_effect=new_runner), \
                     contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(io.StringIO()):
                    args = ["--experiment", "single", "--worker-bin", str(worker),
                            "--output", str(output), "--reps", "2", "--batches", "1"]
                    if wiring == "public":
                        args += ["--wiring", wiring]
                    if fail_hash:
                        with self.assertRaisesRegex(RuntimeError, "later hash failed"):
                            bench.main(args)
                    else:
                        bench.main(args)
                if fail_hash:
                    self.assertEqual(output.read_text(encoding="utf-8"), "previous suite\n")
                    self.assertEqual(bench.load(output.with_suffix(".json")), {"previous": True})
                    self.assertEqual(stdout.getvalue(), "")
                    self.assertEqual(bench.load(base / "single/status.json")["state"], "failed")
                    self.assertNotIn(("blake3", "mikros"), calls)
                else:
                    self.assertEqual(calls, [(h, s) for h in bench.HASHES for s in ("mikros", "flock")])
                    self.assertEqual(stdout.getvalue(), output.read_text(encoding="utf-8"))
                    self.assertEqual((base / "single/previous.log").read_text(encoding="utf-8"), "previous suite\n")
                    doc = bench.load(output.with_suffix(".json"))
                    self.assertEqual(tuple(doc["hashes"]), bench.HASHES)
                    for name, report in doc["hashes"].items():
                        self.assertEqual(report["mikros_wiring"], wiring)
                        self.assertEqual(report["systems"]["mikros"]["prover_ms"]["median"], 4.0)
                        self.assertTrue(Path(report["raw_data"]).is_file())
                        self.assertIn(name.upper() + " end-to-end", stdout.getvalue())
                    self.assertEqual(bench.load(base / "single/status.json")["state"], "complete")

    def test_cli_preserves_paper_defaults_and_accepts_overrides(self):
        options = bench.parse_options([
            "--experiment", "sha256", "--worker-bin", "worker",
        ])
        for name, expected in {"threads": 1, "instances": 256, "log_n": 30,
                               "reps": 10, "warmups": 1, "batches": 1,
                               "rss_runs": 3}.items():
            self.assertEqual(getattr(options, name), expected)
        hadamard = bench.parse_options([
            "--experiment", "hadamard", "--worker-bin", "worker",
        ])
        self.assertEqual(hadamard.reps, 10)
        self.assertEqual(hadamard.warmups, 1)
        self.assertEqual(hadamard.rss_runs, 3)
        options = bench.parse_options([
            "--experiment", "hadamard", "--worker-bin", "worker",
            "--threads", "2", "--log-n", "14", "--reps", "2",
            "--warmups", "1", "--rss-runs", "1", "--output", "out.log",
            "--without-binius",
        ])
        self.assertEqual(options.threads, 2)
        self.assertEqual(options.log_n, 14)
        self.assertEqual(options.warmups, 1)
        self.assertEqual(options.output.name, "out.log")
        self.assertTrue(options.without_binius)
        self.assertFalse(options.validate)

    def test_hash_schedule_counts_ten_samples_per_system_and_honors_overrides(self):
        scenarios = [((), 10, 1, 1),
                     (("--reps", "3", "--batches", "2", "--warmups", "0"), 3, 2, 0)]
        for experiment in (*bench.HASHES, "single"):
            for extra, reps, batches, warmups in scenarios:
                with self.subTest(experiment=experiment, extra=extra), tempfile.TemporaryDirectory() as directory:
                    base = Path(directory)
                    worker = base / "worker"
                    worker.write_bytes(b"synthetic worker")
                    output = base / "report.log"
                    calls = []

                    def new_runner(options):
                        folder = base / options.experiment
                        folder.mkdir()
                        self.assertEqual((options.reps, options.batches, options.warmups),
                                         (reps, batches, warmups))

                        def measured(system, batch):
                            calls.append((options.experiment, system, batch))
                            values = range(batch * options.reps + 1, (batch + 1) * options.reps + 1)
                            report = e2e_report(system, values, 2048, batch, options.warmups)
                            report["metadata"].update({key: options.experiment for key in bench.SHARED_DIGESTS})
                            report["metadata"].update(hash=options.experiment, instances=options.instances)
                            for row in report["samples"]:
                                row["hash"] = options.experiment
                            return report

                        return SimpleNamespace(folder=folder, e2e=measured, cpu=None,
                                               commands=[], env=bench.environment(1))

                    with mock.patch.object(bench, "Runner", side_effect=new_runner), \
                            contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
                        bench.main(["--experiment", experiment, "--worker-bin", str(worker),
                                    "--output", str(output), *extra])
                    doc = bench.load(output.with_suffix(".json"))
                    reports = doc["hashes"] if experiment == "single" else {experiment: doc}
                    expected_hashes = bench.HASHES if experiment == "single" else (experiment,)
                    self.assertEqual(tuple(reports), expected_hashes)
                    self.assertEqual(len(calls), len(expected_hashes) * 2 * batches)
                    for name, report in reports.items():
                        for system in ("mikros", "flock"):
                            self.assertEqual([batch for hash_name, worker_system, batch in calls
                                              if (hash_name, worker_system) == (name, system)],
                                             list(range(batches)))
                            rows = [row for raw in report["raw_reports"]
                                    if raw["metadata"]["system"] == system for row in raw["samples"]]
                            self.assertEqual(sum(not row["warmup"] for row in rows), reps * batches)
                            self.assertEqual(sum(row["warmup"] for row in rows), warmups * batches)
                            summary = report["systems"][system]
                            self.assertEqual(summary["prover_ms"]["count"], reps * batches)
                            self.assertEqual(summary["verify_ms"]["count"], reps * batches)
                            self.assertEqual(summary["prover_ms"]["max"], reps * batches)
                            self.assertEqual(summary["warmup_samples"], warmups * batches)
                            self.assertEqual(report["measurement_plan"][system]["measured_proofs"], reps * batches)
                            self.assertEqual(report["measurement_plan"][system]["excluded_warmups"], warmups * batches)

    def test_hadamard_schedule_reuses_the_only_binius_proof_for_ram(self):
        scenarios = [((), 10, 3, True),
                     (("--reps", "2", "--rss-runs", "1"), 2, 1, True),
                     (("--without-binius",), 10, 3, False)]
        for extra, reps, rss_runs, with_binius in scenarios:
            with self.subTest(extra=extra), tempfile.TemporaryDirectory() as directory:
                base = Path(directory)
                worker = base / "worker"
                worker.write_bytes(b"synthetic worker")
                binius = base / "binius"
                binius.write_bytes(b"synthetic binius worker")
                log = base / "report.log"
                folder = base / "run"
                folder.mkdir()
                calls = []

                def runner_factory(options):
                    def measured(name, systems, binary=None, rss=False):
                        call = {"name": name, "systems": systems, "binary": binary, "rss": rss}
                        calls.append(call)
                        if binary is not None:
                            self.assertEqual(sum(c["binary"] is not None for c in calls), 1,
                                             "Binius must never execute a second measurement process")
                        runs = [0] if rss else range(1, options.reps + 1)
                        samples = [hadamard_row(system, 2.0 + run, run=run,
                                                hwm=2048 if rss else None)
                                   for system in systems for run in runs]
                        warmups = [] if rss else [hadamard_row(system, 999.0, run=0)
                                                 for system in systems]
                        for row in samples + warmups:
                            row["input_digest"] = f"fixture-{row['run']}"
                        return {"metadata": {"threads": 1}, "samples": samples,
                                "warmups": warmups, "test_invocation": name}

                    return SimpleNamespace(folder=folder, hadamard=measured, cpu=None,
                                           commands=[], env=bench.environment(1))

                stdout = io.StringIO()
                with mock.patch.object(bench, "Runner", side_effect=runner_factory), \
                        mock.patch.object(bench, "binius_binary", return_value=binius) as prepare_binius, \
                        mock.patch.object(bench, "build_hadamard_summary",
                                          wraps=bench.build_hadamard_summary) as summarize, \
                        contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(io.StringIO()):
                    bench.main(["--experiment", "hadamard", "--worker-bin", str(worker),
                                "--output", str(log), *extra])
                self.assertEqual(stdout.getvalue(), log.read_text(encoding="utf-8"))
                document = bench.load(log.with_suffix(".json"))
                formal, cold = summarize.call_args.args
                self.assertEqual(len(calls), 1 + 2 * rss_runs + int(with_binius))
                for native, system in (("pairreuse", "mikros"), ("flock128", "flock")):
                    self.assertEqual(sum(len([r for r in report["samples"] if r["system"] == native])
                                         for report in formal), reps)
                    self.assertEqual(sum(len([r for r in report["warmups"] if r["system"] == native])
                                         for report in formal), 1)
                    self.assertEqual(sum(len([r for r in report["samples"] if r["system"] == native])
                                         for report in cold), rss_runs)
                    cold_calls = [c for c in calls if c["systems"] == (native,) and c["rss"]]
                    self.assertEqual(len(cold_calls), rss_runs)
                    self.assertEqual(document["systems"][system]["prover_ms"]["count"], reps)
                    self.assertEqual(document["measurement_plan"][system], {
                        "measured_proofs": reps, "excluded_warmups": 1,
                        "fresh_ram_proofs": rss_runs, "ram_reuses_measured_proof": False,
                    })
                if with_binius:
                    prepare_binius.assert_called_once()
                    binius_calls = [c for c in calls if c["binary"] is not None]
                    self.assertEqual(binius_calls, [{"name": "formal-binius",
                                                     "systems": ("binius_switchover",),
                                                     "binary": binius, "rss": True}])
                    formal_binius = [r for r in formal if r["test_invocation"] == "formal-binius"]
                    cold_binius = [r for r in cold if r["test_invocation"] == "formal-binius"]
                    self.assertEqual(len(formal_binius), 1)
                    self.assertEqual(len(cold_binius), 1)
                    self.assertIs(formal_binius[0], cold_binius[0])
                    self.assertEqual(formal_binius[0]["warmups"], [])
                    self.assertEqual(len(formal_binius[0]["samples"]), 1)
                    self.assertEqual(document["systems"]["binius"]["prover_ms"]["count"], 1)
                    self.assertEqual(document["measurement_plan"]["binius"], {
                        "measured_proofs": 1, "excluded_warmups": 0,
                        "fresh_ram_proofs": 0, "ram_reuses_measured_proof": True,
                    })
                else:
                    prepare_binius.assert_not_called()
                    self.assertNotIn("binius", document["systems"])
                    self.assertNotIn("binius", document["measurement_plan"])

    def test_worker_stderr_progress_arrives_before_child_exits(self):
        with tempfile.TemporaryDirectory() as directory:
            base = Path(directory)
            acknowledged = base / "progress-observed"

            class AcknowledgingStderr(io.StringIO):
                def write(self, value):
                    if "phase-checkpoint" in value:
                        acknowledged.touch()
                    return super().write(value)

            # The child cannot finish successfully until its live stderr has
            # reached the caller. Merely replaying stderr after exit fails.
            script = ("import pathlib, sys, time\n"
                      "print('raw worker output', flush=True)\n"
                      "print('phase-checkpoint', file=sys.stderr, flush=True)\n"
                      "deadline = time.monotonic() + 3\n"
                      "while not pathlib.Path(sys.argv[1]).exists():\n"
                      "    if time.monotonic() > deadline: sys.exit(9)\n"
                      "    time.sleep(0.01)\n"
                      "print('phase-complete', file=sys.stderr, flush=True)\n")
            live = AcknowledgingStderr()
            stdout = io.StringIO()
            with (base / "stdout.txt").open("w", encoding="utf-8") as out, \
                    (base / "stderr.txt").open("w", encoding="utf-8") as err, \
                    contextlib.redirect_stderr(live), contextlib.redirect_stdout(stdout):
                status = bench.run_streamed([sys.executable, "-c", script, str(acknowledged)],
                                            cwd=base, env=bench.environment(1), stdout=out,
                                            stderr=err, label="synthetic", heartbeat_seconds=0.05)
            self.assertEqual(status, 0)
            self.assertTrue(acknowledged.exists())
            self.assertIn("phase-checkpoint", live.getvalue())
            self.assertIn("phase-complete", live.getvalue())
            self.assertEqual(stdout.getvalue(), "")
            self.assertEqual((base / "stdout.txt").read_text(encoding="utf-8"), "raw worker output\n")
            self.assertEqual((base / "stderr.txt").read_text(encoding="utf-8"),
                             "phase-checkpoint\nphase-complete\n")

    def test_silent_worker_gets_pid_heartbeat_without_polluting_transcripts(self):
        with tempfile.TemporaryDirectory() as directory:
            base = Path(directory)
            live = io.StringIO()
            script = "import os, time; print(os.getpid(), flush=True); time.sleep(0.2)"
            with (base / "stdout.txt").open("w", encoding="utf-8") as out, \
                    (base / "stderr.txt").open("w", encoding="utf-8") as err, \
                    contextlib.redirect_stderr(live):
                status = bench.run_streamed([sys.executable, "-c", script], cwd=base,
                                            env=bench.environment(1), stdout=out, stderr=err,
                                            label="silent-worker", heartbeat_seconds=0.025)
            self.assertEqual(status, 0)
            pid = (base / "stdout.txt").read_text(encoding="utf-8").strip()
            self.assertTrue(pid.isdigit())
            self.assertIn("silent-worker", live.getvalue())
            self.assertRegex(live.getvalue().lower(), rf"pid\s*[=: ]\s*{pid}\b")
            self.assertEqual((base / "stderr.txt").read_text(encoding="utf-8"), "")

    def test_failed_worker_keeps_partial_stdout_stderr_and_exit_status(self):
        with tempfile.TemporaryDirectory() as directory:
            base = Path(directory)
            live = io.StringIO()
            script = ("import sys; print('partial raw record', flush=True); "
                      "print('proof phase failed', file=sys.stderr, flush=True); sys.exit(7)")
            with (base / "stdout.txt").open("w", encoding="utf-8") as out, \
                    (base / "stderr.txt").open("w", encoding="utf-8") as err, \
                    contextlib.redirect_stderr(live):
                status = bench.run_streamed([sys.executable, "-c", script], cwd=base,
                                            env=bench.environment(1), stdout=out, stderr=err,
                                            label="failing-worker", heartbeat_seconds=0.05)
            self.assertEqual(status, 7)
            self.assertEqual((base / "stdout.txt").read_text(encoding="utf-8"), "partial raw record\n")
            self.assertEqual((base / "stderr.txt").read_text(encoding="utf-8"), "proof phase failed\n")
            self.assertIn("proof phase failed", live.getvalue())

    def test_cli_rejects_zero_workers_and_empty_measurement_count(self):
        for flag in ("--threads", "--reps", "--batches", "--rss-runs"):
            with self.subTest(flag=flag), contextlib.redirect_stderr(io.StringIO()):
                with self.assertRaises(SystemExit):
                    bench.parse_options(["--experiment", "sha256", "--worker-bin",
                                         "worker", flag, "0"])

    def test_e2e_excludes_warmups_and_combines_batches_before_statistics(self):
        reports = [e2e_report("mikros", [2.0, 6.0], 1024),
                   e2e_report("mikros", [10.0, 14.0], 3072, batch=1),
                   e2e_report("flock", [4.0, 8.0], 2048)]
        systems = bench.build_e2e_summary(reports)["systems"]
        stats = systems["mikros"]["prover_ms"]
        self.assertEqual(stats, {"median": 8.0, "min": 2.0, "max": 14.0,
                                 "mean": 8.0, "count": 4})
        self.assertAlmostEqual(systems["mikros"]["verify_ms"]["median"], 0.8)
        self.assertEqual(systems["mikros"]["ram_mib"], 3.0)
        self.assertEqual(systems["flock"]["ram_mib"], 2.0)
        self.assertEqual(systems["mikros"]["proof_bytes"], 8192)

    def test_e2e_rejects_inconsistent_proof_sizes_across_batches(self):
        first = e2e_report("mikros", [2.0], 1024)
        second = e2e_report("mikros", [3.0], 1024, batch=1)
        second["samples"][1]["proof_bytes"] += 1
        with self.assertRaises(ValueError):
            bench.build_e2e_summary([first, second, e2e_report("flock", [3.0], 1024)])

    def test_e2e_requires_both_verification_and_transcript_checks(self):
        for field in ("verified", "transcript_match"):
            for missing in (False, True):
                report = e2e_report("mikros", [2.0], 1024)
                if missing:
                    del report["samples"][1][field]
                else:
                    report["samples"][0][field] = False
                with self.subTest(field=field, missing=missing):
                    with self.assertRaises(ValueError):
                        bench.build_e2e_summary([report, e2e_report("flock", [3.0], 1024)])

    def test_e2e_rejects_nonfinite_times_and_memory(self):
        for field in ("prover_ms", "verify_ms", "peak_rss_kib"):
            for value in (float("nan"), float("inf"), float("-inf")):
                report = e2e_report("mikros", [2.0], 1024)
                if field == "peak_rss_kib":
                    report[field] = value
                else:
                    report["samples"][1][field] = value
                with self.subTest(field=field, value=value):
                    with self.assertRaises(ValueError):
                        bench.build_e2e_summary([report, e2e_report("flock", [3.0], 1024)])

    def test_hadamard_uses_serialized_bytes_and_separate_cold_ram(self):
        formal, cold = hadamard_fixture()
        systems = bench.build_hadamard_summary(formal, cold)["systems"]
        self.assertEqual(set(systems), {"mikros", "flock", "binius"})
        for name in systems:
            self.assertEqual(systems[name]["prover_ms"]["median"], 5.0)
            self.assertEqual(systems[name]["prover_ms"]["count"], 2)
            self.assertAlmostEqual(systems[name]["verify_ms"]["median"], 0.05)
            self.assertEqual(systems[name]["ram_mib"], 2.0)
        self.assertEqual(systems["mikros"]["proof_bytes"], 4888)
        self.assertEqual(systems["flock"]["proof_bytes"], 4912)
        self.assertEqual(systems["binius"]["proof_bytes"], 1504)

    def test_hadamard_requires_source_checks_in_cold_runs_too(self):
        for field in ("verifier_passed", "transcript_matched", "source_claims_verified"):
            formal, cold = hadamard_fixture()
            del cold[0]["samples"][0][field]
            with self.subTest(field=field), self.assertRaises(ValueError):
                bench.build_hadamard_summary(formal, cold)

    def test_hadamard_rejects_nonfinite_times_and_cold_ram(self):
        for field in ("prover_ms", "verifier_ms", "vm_hwm_after_prove_kib"):
            formal, cold = hadamard_fixture()
            target = cold[0] if field == "vm_hwm_after_prove_kib" else formal[0]
            target["samples"][0][field] = float("nan")
            with self.subTest(field=field), self.assertRaises(ValueError):
                bench.build_hadamard_summary(formal, cold)

    def test_hadamard_rejects_proof_size_drift_in_cold_samples(self):
        formal, cold = hadamard_fixture()
        cold[0]["samples"][0]["bincode_bytes"] += 1
        with self.assertRaises(ValueError):
            bench.build_hadamard_summary(formal, cold)

    def test_empty_formal_samples_are_not_reported_as_measurements(self):
        report = e2e_report("mikros", [], 1024)
        with self.assertRaises(ValueError):
            bench.build_e2e_summary([report, e2e_report("flock", [3.0], 1024)])
        formal, cold = hadamard_fixture()
        formal[0]["samples"] = []
        with self.assertRaises(ValueError):
            bench.build_hadamard_summary(formal, cold)

    def test_text_report_labels_units_ratio_scope_and_raw_evidence(self):
        systems = bench.build_e2e_summary([
            e2e_report("mikros", [2.0, 6.0], 1024),
            e2e_report("flock", [4.0, 12.0], 2048),
        ])["systems"]
        document = {
            "experiment": "sha256", "threads": 1, "instances": 256,
            "systems": systems,
            "measurement_scope": {"prover": "Full online proving",
                                  "verifier": "Full proof verification",
                                  "ram": "Whole-process peak RSS"},
            "raw_data": "raw/sha256.json",
        }
        text = bench.render_report(document)
        for label in ("Mikros", "FLOCK", "Prover", "Verifier", "Proof", "RAM",
                      "ms", "MiB", "2.00", "Full online proving",
                      "Whole-process peak RSS", "raw/sha256.json"):
            self.assertIn(label, text)
        self.assertNotIn("nan", text.lower())

    def test_worker_failure_preserves_published_log_and_raw_json(self):
        with tempfile.TemporaryDirectory() as directory:
            base = Path(directory)
            log = base / "existing.log"
            log.write_text("original measured log\n", encoding="utf-8")
            raw = log.with_suffix(".json")
            raw.write_text('{"original": true}\n', encoding="utf-8")
            worker = base / "worker"
            worker.write_bytes(b"synthetic worker")
            folder = base / "run"
            folder.mkdir()
            runner = SimpleNamespace(folder=folder, e2e=mock.Mock(side_effect=RuntimeError("worker failed")))
            with mock.patch.object(bench, "Runner", return_value=runner):
                with self.assertRaisesRegex(RuntimeError, "worker failed"):
                    bench.main(["--experiment", "sha256", "--worker-bin", str(worker),
                                "--output", str(log), "--batches", "1"])
            self.assertEqual(log.read_text(encoding="utf-8"), "original measured log\n")
            self.assertEqual(raw.read_text(encoding="utf-8"), '{"original": true}\n')
            self.assertEqual(bench.load(folder / "status.json")["state"], "failed")

    def test_success_prints_exact_log_and_archives_previous_results(self):
        with tempfile.TemporaryDirectory() as directory:
            base = Path(directory)
            log = base / "existing.log"
            log.write_text("old report\n", encoding="utf-8")
            log.with_suffix(".json").write_text('{"old": true}\n', encoding="utf-8")
            worker = base / "worker"
            worker.write_bytes(b"synthetic worker")
            folder = base / "run"
            folder.mkdir()

            def measured(system, batch):
                report = e2e_report(system, [2.0, 6.0], 2048, batch)
                report["metadata"].update({key: "same fixture" for key in bench.SHARED_DIGESTS})
                return report

            runner = SimpleNamespace(folder=folder, e2e=measured, cpu=None, commands=[],
                                     env=bench.environment(1))
            stdout = io.StringIO()
            with mock.patch.object(bench, "Runner", return_value=runner):
                with contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(io.StringIO()):
                    bench.main(["--experiment", "sha256", "--worker-bin", str(worker),
                                "--output", str(log), "--batches", "1", "--reps", "2"])
            self.assertEqual(stdout.getvalue(), log.read_text(encoding="utf-8"))
            self.assertEqual((folder / "previous.log").read_text(encoding="utf-8"), "old report\n")
            self.assertEqual(bench.load(folder / "previous.json"), {"old": True})
            self.assertEqual(bench.load(folder / "status.json")["state"], "complete")
            self.assertEqual(bench.load(log.with_suffix(".json"))["systems"]["mikros"]["prover_ms"]["median"], 4.0)


if __name__ == "__main__":
    unittest.main()
