"""WHIR backend isolation and accounting; synthetic records, not performance data."""
import contextlib
import io
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "benches"))
import plonk_whir_bench as bench
import packed_plonk_bench as common
from test_plonk_fri_bench import records as fri_records, encode


def options():
    return bench.parse_options(["--worker-bin", __file__, "--log-n", "14", "--threads", "4",
                                "--reps", "2", "--warmups", "1"])


def records():
    rows = fri_records()
    rows[0]["pcs_backend"] = bench.BACKEND
    rows[0]["pcs_profile"] = dict(bench.SPEC.required_profile)
    return rows


class PlonkWhirBenchTests(unittest.TestCase):
    def test_entry_defaults_and_backend_isolation(self):
        args = bench.parse_options(["--worker-bin", __file__])
        self.assertEqual((args.log_n, args.threads, args.reps, args.warmups), (26, 64, 1, 0))
        self.assertEqual(args.out, common.ROOT / "target/paper-results/plonk-whir-2-26")
        with self.assertRaises(ValueError):
            bench.parse_worker(encode(fri_records()), options(), 2048)
        with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
            bench.parse_options(["--worker-bin", __file__, "--experiment", "plonk-fri"])

    def test_offline_public_commit_excluded_and_public_open_included(self):
        report = bench.parse_worker(encode(records()), options(), 2048)
        summary = common.build_summary(report)
        self.assertEqual(summary["prover_ms"]["median"], 15)
        self.assertEqual(summary["prover_phases_ms"]["public_pcs_open_ms"]["median"], 1.5)
        self.assertNotIn("offline_ms", summary["prover_phases_ms"])

    def test_rejects_partial_proofs_and_wrong_packing(self):
        for key, value in (("pcs_complete", False), ("verified", False), ("transcript_match", False),
                           ("public_pcs_proof_bytes", 0), ("witness_pcs_proof_bytes", 0)):
            rows = records()
            rows[1][key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                bench.parse_worker(encode(rows), options(), 2048)
        rows = records()
        rows[0]["witness_words"] = 1
        with self.assertRaises(ValueError):
            bench.parse_worker(encode(rows), options(), 2048)

    def test_profile_and_parameter_audit_required(self):
        for key, value in bench.SPEC.required_profile:
            rows = records()
            rows[0]["pcs_profile"][key] = None
            with self.subTest(key=key), self.assertRaises(ValueError):
                bench.parse_worker(encode(rows), options(), 2048)
        for group in ("witness", "public"):
            rows = records()
            rows[0].pop(group + "_pcs_parameters")
            with self.subTest(group=group), self.assertRaises(ValueError):
                bench.parse_worker(encode(rows), options(), 2048)


if __name__ == "__main__":
    unittest.main()
