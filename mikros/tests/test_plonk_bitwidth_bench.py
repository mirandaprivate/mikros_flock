"""Synthetic driver records test accounting and field semantics, not performance."""
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
SPEC = importlib.util.spec_from_file_location("plonk_bitwidth_bench", BENCHES / "plonk_bitwidth_bench.py")
bench = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(bench)


def records(bits, threads=4, log_n=14, reps=2, warmups=1):
    meta = {"type": "metadata", "system": "mikros", "mode": "single", "arithmetization": "PLONK",
            "log_gates": log_n, "instances": 1, "gates_per_instance": 1 << log_n, "total_gates": 1 << log_n,
            "threads": threads, "all_core_threads": threads, "seed": 97, "repetitions": reps, "warmups": warmups,
            "pcs_backend": "FLOCK native Ligerito Fast",
            "field_semantics": "GF(2^b); ADD is field addition, MUL is field multiplication",
            "target_features": {key: False for key in ("gfni", "avx512f", "avx512bw", "vpclmulqdq")},
            **bench.packing_geometry(log_n, bits)}
    meta.update({group + "_pcs_params": bench.expected_pcs_params(meta[group + "_words"])
                 for group in ("witness", "public")})
    rows = [meta]
    for index in range(reps + warmups):
        warmup = index < warmups
        scale = 100 if warmup else index - warmups + 1
        times = {key: float(scale) for key in bench.PROVER_PHASES}
        times.update(prover_ms=10 * scale, witness_claims=11, public_claims=53, native_pcs_claims=2)
        rows.append({"type": "sample", "value_bits": bits, "repetition": index if warmup else index - warmups,
                     "warmup": warmup, "verified": True, "transcript_match": True, "proof_bytes": 4096 + bits,
                     "verify_ms": float(scale), "peak_rss_kib": 4096, "offline_ms": 5.0, "times": times,
                     "witness_pcs_params": meta["witness_pcs_params"].copy(),
                     "public_pcs_params": meta["public_pcs_params"].copy(),
                     "verifier_times": {key: 0.1 * scale for key in bench.VERIFIER_PHASES},
                     "piop_grinding_ms": {"gate_ms": 0.1 * scale, "copy_ms": 0.1 * scale,
                                          "copy_initial_ms": 0.05 * scale, "opening_ms": 0.1 * scale,
                                          "total_ms": 0.3 * scale}})
    return rows


def encoded(rows):
    return "\n".join(json.dumps(row) for row in rows) + "\n"


def options():
    return bench.parse_options(["--worker-bin", __file__, "--log-n", "14", "--threads", "4",
                                "--bits", "1,2,4,8,16", "--reps", "2", "--warmups", "1"])


class PlonkBitwidthBenchTests(unittest.TestCase):
    def test_default_eight_fields_n26_one_sample_and_no_warmup(self):
        args = bench.parse_options(["--worker-bin", __file__])
        self.assertEqual((args.log_n, args.threads, args.reps, args.warmups, args.seed), (26, 64, 1, 0, 97))
        self.assertEqual(args.bits, [1, 2, 4, 8, 16, 32, 64, 128])
        self.assertEqual(args.out, bench.ROOT / "target/paper-results/plonk-bitwidth-wide-2-26")

    def test_invalid_fields_duplicates_and_unsafe_thread_configuration(self):
        flags = [["--bits", value] for value in ("", "1,1", "3", "256", "1,", "integer")]
        flags += [["--threads", "0"], ["--threads", "4", "--cpu", "0"], ["--log-n", "31"],
                  ["--reps", "0"], ["--warmups", "-1"], ["--seed", "-1"], ["--out", str(bench.ROOT)],
                  ["--threads", "257"], ["--reps", "101"], ["--warmups", "11"]]
        for item in flags:
            with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                bench.parse_options(["--worker-bin", __file__, *item])

    def test_geometry_uses_field_degree_and_separate_copy_packing(self):
        b1 = bench.packing_geometry(30, 1)
        b16 = bench.packing_geometry(30, 16)
        self.assertEqual((b1["kappa"], b1["tag_bits"], b1["copy_kappa"]), (128, 64, 2))
        self.assertEqual((b16["kappa"], b16["tag_bits"], b16["copy_kappa"]), (8, 64, 2))
        self.assertEqual((b1["witness_words"], b1["public_words"]), (1 << 25, 1 << 30))
        self.assertEqual((b16["witness_words"], b16["public_words"]), (1 << 29, 1 << 31))
        self.assertEqual(bench.packing_geometry(26, 1)["copy_kappa"], 4)
        self.assertEqual(bench.packing_geometry(26, 16)["copy_kappa"], 2)
        expected = {32: (4, 64, 2, 128, "packed", 1 << 26, 1 << 27),
                    64: (2, 128, 1, 128, "unpacked_f128", 1 << 27, 1 << 28),
                    128: (1, 256, 1, 256, "unpacked_f256", 1 << 28, 1 << 29)}
        for bits, shape in expected.items():
            geometry = bench.packing_geometry(26, bits)
            self.assertEqual(tuple(geometry[key] for key in ("kappa", "tag_bits", "copy_kappa", "copy_field_bits",
                                                            "copy_mode", "witness_words", "public_words")), shape)
            for group in ("witness", "public"):
                self.assertEqual(bench.expected_pcs_params(geometry[group + "_words"])["m"],
                                 geometry[group + "_words"].bit_length() + 6)

    def test_rejects_wrong_field_packing_shape_or_actual_pool(self):
        changes = {"base_field_bits": 1, "kappa": 128, "tag_bits": 64, "copy_kappa": 2,
                   "copy_field_bits": 256, "copy_mode": "unpacked_f128",
                   "witness_words": 1, "public_words": 1, "threads": 1, "all_core_threads": 1,
                   "system": "flock", "arithmetization": "R1CS", "instances": 128, "total_gates": 1,
                   "field_semantics": "integer range", "public_pcs_params": {}}
        for key, value in changes.items():
            rows = records(4)
            rows[0][key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                bench.parse_worker(encoded(rows), options(), 4, 2048)

    def test_legacy_metadata_accepted_only_for_original_five_widths(self):
        for bits in bench.SUPPORTED_BITS:
            rows = records(bits)
            for key in bench.COPY_METADATA:
                del rows[0][key]
            if bits <= 16:
                parsed = bench.parse_worker(encoded(rows), options(), bits, 2048)
                summary = bench.build_summary([parsed], [bits])
                self.assertEqual(summary[0]["copy_mode"], "packed")
            else:
                with self.assertRaisesRegex(ValueError, "copy-field metadata"):
                    bench.parse_worker(encoded(rows), options(), bits, 2048)
        rows = records(1)
        del rows[0]["copy_field_bits"]
        with self.assertRaisesRegex(ValueError, "copy-field metadata"):
            bench.parse_worker(encoded(rows), options(), 1, 2048)

    def test_wide_copy_fields_and_two_native_pcs_terminals(self):
        for bits, mode in ((32, "packed"), (64, "unpacked_f128"), (128, "unpacked_f256")):
            rows = records(bits)
            for row in rows[1:]:
                row["piop_grinding_ms"].update(copy_ms=0, copy_initial_ms=0)
                row["piop_grinding_ms"]["total_ms"] = row["piop_grinding_ms"]["gate_ms"] + row["piop_grinding_ms"]["opening_ms"]
            report = bench.parse_worker(encoded(rows), options(), bits, 2048)
            self.assertEqual(bench.build_summary([report], [bits])[0]["copy_mode"], mode)
            self.assertTrue(all(row["times"]["native_pcs_claims"] == 2 for row in report["samples"]))
            rows[0]["copy_field_bits"] *= 2
            with self.assertRaisesRegex(ValueError, "copy_field_bits"):
                bench.parse_worker(encoded(rows), options(), bits, 2048)

    def test_old_summary_can_still_render_without_copy_field_columns(self):
        rows = records(16)
        for key in bench.COPY_METADATA:
            del rows[0][key]
        report = bench.parse_worker(encoded(rows), options(), 16, 2048)
        summary = bench.build_summary([report], [16])
        for key in bench.COPY_METADATA:
            del summary[0][key]
        text = bench.render_report({"log_n": 14, "threads": 4, "reps": 2, "warmups": 1,
                                    "cpu_affinity": [0, 1, 2, 3], "widths": summary,
                                    "raw_data": "target/paper-results/legacy/plonk_bitwidth.json"})
        self.assertIn("Copy F bits", text)
        self.assertIn("packed", text)
        self.assertIn("2 measured proof(s)", text)

    def test_missing_pcs_configuration_fails_before_any_worker(self):
        args = bench.parse_options(["--worker-bin", __file__, "--log-n", "30", "--bits", "128"])
        with patch.object(bench, "run_streamed") as worker:
            with self.assertRaisesRegex(ValueError, "missing native Fast PCS configuration"):
                bench.run(args)
            worker.assert_not_called()

    def test_full_proof_sampling_phase_and_grinding_checks(self):
        mutations = (lambda row: row.update(verified=False), lambda row: row.update(transcript_match=False),
                     lambda row: row.update(value_bits=2), lambda row: row.update(warmup=False),
                     lambda row: row.update(proof_bytes=2), lambda row: row["times"].update(native_pcs_claims=1),
                     lambda row: row["times"].update(public_claims=0), lambda row: row["times"].update(copy_prepare_ms=-1),
                     lambda row: row["times"].update(prover_ms=float("nan")),
                     lambda row: row["witness_pcs_params"].update(m=99),
                     lambda row: row["verifier_times"].update(copy_ms=10000),
                     lambda row: row["piop_grinding_ms"].update(total_ms=500),
                     lambda row: row["piop_grinding_ms"].update(copy_initial_ms=500))
        for mutate in mutations:
            rows = records(1)
            mutate(rows[1])
            with self.assertRaises(ValueError):
                bench.parse_worker(encoded(rows), options(), 1, 2048)

    def test_summary_excludes_warmups_and_does_not_mix_fields(self):
        reports = [bench.parse_worker(encoded(records(bits)), options(), bits, 2048) for bits in (1, 4)]
        summary = bench.build_summary(reports, [1, 4])
        self.assertEqual(summary[0]["prover_ms"]["median"], 15)
        self.assertAlmostEqual(summary[0]["online_minus_piop_grinding_ms"]["median"], 14.55)
        self.assertEqual(summary[0]["ram_mib"], 2)
        with self.assertRaises(ValueError):
            bench.build_summary(reports, [4, 1])
        reports[1]["metadata"]["target_features"]["gfni"] = True
        with self.assertRaisesRegex(ValueError, "target_features"):
            bench.build_summary(reports, [1, 4])

    def test_grinding_diagnostic_subtracts_each_sample_before_median(self):
        args = bench.parse_options(["--worker-bin", __file__, "--log-n", "14", "--threads", "4",
                                    "--bits", "1", "--reps", "3", "--warmups", "0"])
        rows = records(1, reps=3, warmups=0)
        for row, total, grinding in zip(rows[1:], (100, 200, 300), (90, 0, 290)):
            row["times"].update({key: 1 for key in bench.PROVER_PHASES})
            row["times"].update(prover_ms=total, copy_ms=grinding)
            row["piop_grinding_ms"].update(gate_ms=0, copy_ms=grinding, copy_initial_ms=grinding,
                                           opening_ms=0, total_ms=grinding)
        report = bench.parse_worker(encoded(rows), args, 1, 2048)
        summary = bench.build_summary([report], [1])[0]
        self.assertEqual(summary["online_minus_piop_grinding_ms"]["median"], 10)
        self.assertEqual(summary["prover_ms"]["median"] - summary["piop_grinding_ms"]["total_ms"]["median"], 110)
        self.assertEqual(summary["piop_grinding_ms"]["gate_ms"]["median"], 0)

    def test_each_field_has_an_independent_worker_and_anonymous_report(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            worker = root / "worker"
            worker.write_text("mock")
            out = root / "results"
            args = bench.parse_options(["--worker-bin", str(worker), "--time-bin", str(worker), "--log-n", "14",
                                        "--threads", "4", "--bits", "1,4,16,32,64,128", "--reps", "2", "--warmups", "1", "--out", str(out)])
            calls = []

            def fake_worker(command, **kwargs):
                self.assertIsNone(kwargs["cpu"])
                self.assertEqual(kwargs["env"]["RAYON_NUM_THREADS"], "4")
                self.assertEqual(kwargs["env"]["FLOCK_VERIFY_THREADS"], "4")
                bits = int(command[command.index("--bits") + 1])
                calls.append(bits)
                kwargs["stdout"].write(encoded(records(bits)))
                Path(command[command.index("-o") + 1]).write_text("Maximum resident set size (kbytes): 2048\nExit status: 0\n")
                return 0

            with patch.object(bench.subprocess, "run", return_value=SimpleNamespace(returncode=0, stdout="GNU time")), \
                 patch.object(bench.os, "sched_getaffinity", return_value={1, 3, 5, 7}, create=True), \
                 patch.object(bench, "ROOT", root), patch.object(bench, "run_streamed", side_effect=fake_worker), patch("builtins.print"):
                doc = bench.run(args)
            self.assertEqual(calls, [1, 4, 16, 32, 64, 128])
            self.assertEqual(doc["cpu_affinity"], [1, 3, 5, 7])
            text = (out / "plonk_bitwidth.log").read_text()
            self.assertIn("base-field degree", text)
            self.assertIn("not the complete E2E prover time", text)
            self.assertIn("PCS grinding remains included", text)
            self.assertIn("Verifier phases", text)
            self.assertIn("unpacked_f256", text)
            self.assertIn("ordinary ProductGKR", text)
            self.assertIn("two native PCS terminals", text)
            self.assertIn("witness generation and input-layout preparation are offline", text)
            self.assertNotIn(directory, text)
            self.assertNotIn(directory, (out / "plonk_bitwidth.json").read_text())
            self.assertEqual(len(list(out.glob("runs/*/*.resource.txt"))), 6)

    def test_failure_preserves_existing_report(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            worker = root / "worker"
            worker.write_text("mock")
            out = root / "results"
            out.mkdir()
            (out / "plonk_bitwidth.log").write_text("previous result")
            args = bench.parse_options(["--worker-bin", str(worker), "--time-bin", str(worker),
                                        "--log-n", "14", "--threads", "1", "--out", str(out)])
            with patch.object(bench.subprocess, "run", return_value=SimpleNamespace(returncode=0, stdout="GNU time")), \
                 patch.object(bench.os, "sched_getaffinity", return_value={1}, create=True), \
                 patch.object(bench, "ROOT", root), patch.object(bench, "run_streamed", return_value=101):
                with self.assertRaisesRegex(ValueError, "worker failed"):
                    bench.run(args)
            self.assertEqual((out / "plonk_bitwidth.log").read_text(), "previous result")


if __name__ == "__main__":
    unittest.main()
