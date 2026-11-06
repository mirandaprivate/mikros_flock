"""Synthetic worker records test validation and checkpointing, not performance."""
import contextlib
import copy
import csv
import io
import json
import os
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import Mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "benches"))
import bitwidth_instances_bench as bench


def records(o, case, bits):
    meta = {"type": "metadata", "system": "mikros", "instances": bench.INSTANCES[case],
            "arithmetization": bench.ARITHMETIZATION[case], "value_bits": bits, "kappa": 128//bits,
            "log_gates": o.log_n, "gates_per_instance": 1 << o.log_n,
            "total_gates": (1 << o.log_n)*bench.INSTANCES[case], "threads": o.threads, "seed": o.seed,
            "repetitions": o.reps, "warmups": o.warmups, "pcs_backend": bench.PCS,
            "pcs_profile": {"field_bits": 128, "packed_word_bits": 128, "rate": 0.5, "native_queries": 241}}
    if case == "single":
        meta.update(bench.packing_geometry(o.log_n, bits), permutation_proof=True)
    else:
        meta.update(r1cs_rows=4 << o.log_n, instance_packing_width=128//bits, packed_groups=2*bits,
                    witness_words=(4 << o.log_n)*(2*bits), public_words=0, permutation_proof=False,
                    matrix_authentication="public shared binary matrix walker; C=I")
    rows = [meta]
    for i in range(o.reps + o.warmups):
        warmup = i < o.warmups
        times = {"prover_ms": 2000.0 if case == "single" else 9000.0,
                 "pcs_claims": 2 if case == "single" else 1}
        if case == "single":
            times.update(witness_claims=11, public_claims=5+3*((o.log_n+2+bits-1)//bits))
        rows.append({"type": "sample", "value_bits": bits, "warmup": warmup,
                     "repetition": i if warmup else i-o.warmups, "verified": True,
                     "transcript_match": True, "pcs_complete": True, "times": times,
                     "verify_ms": 4.0, "proof_bytes": 4096, "peak_rss_kib": 8192,
                     "witness_pcs_proof_bytes": 1024, "public_pcs_proof_bytes": 1024})
    return rows


def encoded(rows):
    return "\n".join(json.dumps(r) for r in rows)+"\n"


def fake_success(command, *, folder, o, **kwargs):
    case = command[command.index("--case")+1]
    bits = int(command[command.index("--bits")+1])
    (folder / "stdout.txt").write_text(encoded(records(o, case, bits)), encoding="utf-8")
    (folder / "stderr.txt").write_text("stage=verify_full_proof\n", encoding="utf-8")
    (folder / "resource.txt").write_text("Maximum resident set size (kbytes): 16384\nExit status: 0\n", encoding="utf-8")
    return {"exit_code": 0, "guarded": False}


class BenchTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.worker = self.root / "worker"
        self.worker.write_text("synthetic worker identity")

    def options(self, *args):
        return bench.parse_options(["--worker-bin", str(self.worker), "--time-bin", str(self.worker),
                                    "--out", str(self.root / "results"), "--bits", "1,128", *args])

    def run_bench(self, o=None, **kwargs):
        with contextlib.redirect_stdout(io.StringIO()):
            return bench.run(o or self.options(), available=kwargs.pop("available", lambda: 400),
                             affinity=[0, 2], launcher=kwargs.pop("launcher", fake_success), **kwargs)

    def test_defaults_and_invalid_configuration(self):
        o = bench.parse_options(["--worker-bin", str(self.worker)])
        self.assertEqual((o.log_n, o.threads, o.reps, o.warmups), (20, 1, 1, 0))
        self.assertEqual((o.memory_budget_gib, o.reserve_gib), (300, 32))
        self.assertEqual(o.bits, list(bench.WIDTHS))
        for args in (("--bits", "1,1"), ("--bits", "3"), ("--log-n", "13"), ("--cpu", "1", "--threads", "2"),
                     ("--memory-budget-gib", "nan"), ("--reserve-gib", "0"), ("--poll-seconds", "nan")):
            with self.subTest(args=args), contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                self.options(*args)

    def test_all_width_geometry_and_no_packing_at_128(self):
        for b in bench.WIDTHS:
            for case in bench.CASES:
                o = self.options()
                report, summary = bench.parse_worker(encoded(records(o, case, b)), o, case, b, 16384)
                self.assertEqual(report["metadata"]["kappa"], 128//b)
                self.assertEqual(summary["ram_mib"], 16)
        self.assertEqual(bench.packing_geometry(20,64)["copy_field_bits"], 128)
        self.assertEqual(bench.packing_geometry(20,128)["copy_field_bits"], 256)
        self.assertEqual(bench.memory_estimate(20,128,"multiple")["estimated_gib"], 200)

    def test_incomplete_wrong_dimension_and_nonfinite_results_rejected(self):
        o = self.options()
        mutations = [lambda r: r[1].update(verified=False), lambda r: r[1].update(pcs_complete=False),
                     lambda r: r[1].update(transcript_match=False), lambda r: r[0].update(total_gates=1),
                     lambda r: r[0].update(kappa=0), lambda r: r[1].update(witness_pcs_proof_bytes=0),
                     lambda r: r[1]["times"].update(prover_ms=float("nan")), lambda r: r[1].update(proof_bytes=True)]
        for mutate in mutations:
            rows = records(o,"multiple",128)
            mutate(rows)
            with self.assertRaises(ValueError):
                bench.parse_worker(encoded(rows),o,"multiple",128,16384)

    def test_warmup_excluded_total_batch_cost_retained(self):
        o = self.options("--reps","2","--warmups","1")
        rows = records(o,"multiple",1)
        rows[1]["times"]["prover_ms"] = 999999
        _, summary = bench.parse_worker(encoded(rows),o,"multiple",1,16384)
        self.assertEqual(summary["prover_ms"]["median"],9000)
        self.assertEqual(summary["prover_ms"]["count"],2)

    def test_success_csv_checkpoint_and_resume(self):
        doc, code = self.run_bench()
        self.assertEqual(code,0)
        self.assertTrue(doc["completed"])
        self.assertEqual([c["status"] for c in doc["cases"]],["ok"]*4)
        self.assertEqual(doc["cases"][1]["metrics"]["prover_ms"],9000)
        self.assertEqual(json.loads((self.root/"results/checkpoint.json").read_text()),doc)
        launch = Mock(side_effect=AssertionError("must reuse successful cases"))
        resumed, code = self.run_bench(self.options("--resume"),launcher=launch)
        self.assertEqual(code,0)
        launch.assert_not_called()
        self.assertEqual(resumed["cases"],doc["cases"])
        with (self.root/"results/results.csv").open(newline="") as stream:
            rows=list(csv.DictReader(stream))
        self.assertEqual(rows[1]["total_gates"],str(256*(1<<20)))

    def test_resume_rejects_changed_raw_or_case_dimensions(self):
        doc,_=self.run_bench()
        path=self.root/"results/checkpoint.json"
        changed=copy.deepcopy(doc)
        changed["cases"][0]["instances"]=256
        path.write_text(json.dumps(changed))
        with self.assertRaisesRegex(ValueError,"instances"):
            self.run_bench(self.options("--resume"))
        path.write_text(json.dumps(doc))
        raw=self.root/"results"/doc["cases"][0]["raw_directory"]/"stdout.txt"
        raw.write_text(raw.read_text()+"\n")
        with self.assertRaisesRegex(ValueError,"artifact"):
            self.run_bench(self.options("--resume"))

    def test_memory_skips_and_failures_leave_empty_cells(self):
        doc,code=self.run_bench(self.options("--memory-budget-gib","100"))
        self.assertEqual(code,0)
        self.assertEqual(doc["cases"][-1]["status"],"memory_skipped")
        self.assertEqual(doc["cases"][-1]["metrics"],bench.empty_metrics())
        with (self.root/"results/results.csv").open(newline="") as stream:
            row=list(csv.DictReader(stream))[-1]
        self.assertTrue(all(row[m]=="" for m in bench.METRICS))
        for guarded,status in ((True,"guarded"),(False,"error")):
            o=self.options("--out",str(self.root/str(guarded)))
            launch=Mock(return_value={"exit_code":137,"guarded":guarded,"guard_reason":"test guard"})
            doc,code=self.run_bench(o,launcher=launch)
            self.assertEqual(code, int(not guarded))
            self.assertTrue(all(c["status"]==status and c["metrics"]==bench.empty_metrics() for c in doc["cases"]))

    @unittest.skipUnless(os.name=="posix","Linux worker process groups")
    def test_owned_worker_streaming_and_prelaunch_guard(self):
        folder=self.root/"stream"
        folder.mkdir()
        o=self.options()
        row=records(o,"single",1)[1]
        command=[sys.executable,"-c",f"print({json.dumps(json.dumps(row))}); import sys; print('stage=done',file=sys.stderr)"]
        capture=io.StringIO()
        with contextlib.redirect_stdout(capture):
            result=bench.run_guarded(command,folder=folder,env=os.environ.copy(),o=o,cpu=None,
                                     available=lambda:400,on_start=lambda pid:None,label="test")
        self.assertEqual(result["exit_code"],0)
        self.assertFalse(result["guarded"])
        self.assertIn("prover VmHWM=",capture.getvalue())
        self.assertIn("proof=4096 B",capture.getvalue())
        result=bench.run_guarded(["must-not-launch"],folder=folder,env={},o=o,cpu=None,
                                 available=lambda:0,on_start=lambda pid:None,label="guard")
        self.assertTrue(result["guarded"])
        self.assertIsNone(result["exit_code"])


if __name__ == "__main__":
    unittest.main()
