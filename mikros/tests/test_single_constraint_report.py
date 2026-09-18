"""Accounting checks for single-instance gate/copy reports."""
import copy
import contextlib
import io
import importlib.util
from pathlib import Path
import unittest

SPEC = importlib.util.spec_from_file_location(
    "single_constraint_driver", Path(__file__).resolve().parents[1] / "benches/paper_bench.py")
bench = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(bench)


def reports():
    result = []
    for system in ("mikros", "flock"):
        rows = []
        for i, scale in enumerate((100, 1, 3)):
            row = {"warmup": i == 0, "verified": True, "transcript_match": True,
                   "prover_ms": 10 * scale, "verify_ms": scale, "proof_bytes": 1000}
            if system == "mikros":
                row["timings"] = {"constraint_breakdown": {
                    name: {"prover_ms": weight * scale, "verifier_ms": weight * scale / 10,
                           "proof_bytes": size}
                    for name, weight, size in (("gate", 2, 100), ("copy", 3, 200), ("shared", 5, 700))}}
            rows.append(row)
        result.append({"metadata": {"system": system}, "samples": rows, "peak_rss_kib": 1024})
    return result


class SingleConstraintReportTests(unittest.TestCase):
    def test_public_wiring_is_explicit_and_does_not_replace_packed_logs(self):
        args = ["--experiment", "sha256", "--worker-bin", "worker", "--instances", "1"]
        packed = bench.parse_options(args)
        public = bench.parse_options(args + ["--wiring", "public"])
        self.assertEqual(packed.wiring, "packed")
        self.assertEqual(public.wiring, "public")
        self.assertNotEqual(packed.output, public.output)
        for experiment in ("sha256", "hadamard"):
            with self.subTest(experiment=experiment), contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                bench.parse_options(["--experiment", experiment, "--worker-bin", "worker", "--wiring", "public"])

    def test_components_exclude_warmup_and_do_not_invent_flock_timings(self):
        summary = bench.build_e2e_summary(reports())["systems"]
        parts = summary["mikros"]["constraint_breakdown"]
        self.assertEqual(parts["gate"]["prover_ms"]["median"], 4)
        self.assertEqual(parts["copy"]["verify_ms"]["count"], 2)
        self.assertEqual(sum(p["proof_bytes"] for p in parts.values()), 1000)
        self.assertNotIn("constraint_breakdown", summary["flock"])
        text = bench.render_report({"experiment": "sha256", "instances": 1, "threads": 1,
                                    "systems": summary, "measurement_scope": {}, "raw_data": "test.json"})
        for label in ("Gate constraint", "Copy constraint", "Shared PCS/other", "FLOCK skip-7"):
            self.assertIn(label, text)

    def test_permutation_report_names_the_argument_and_counts_shared_pcs_once(self):
        summary = bench.build_e2e_summary(reports())["systems"]
        doc = {"experiment": "sha256", "instances": 1, "threads": 1,
               "mikros_wiring": "packed", "systems": summary,
               "measurement_scope": {}, "raw_data": "test.json"}
        text = bench.render_report(doc)
        rows = [line.split() for line in text.splitlines() if line.startswith("Permutation argument ")]
        # Ignore the scope sentence; the numerical row must preserve the copy
        # reduction's measured costs and serialized bytes, without adding PCS.
        self.assertIn(["Permutation", "argument", "6.000", "0.600", "0.195"], rows)
        self.assertIn("Shared PCS/other is counted once", text)
        self.assertNotIn("public wiring", text)
        self.assertNotIn("Copy constraint", text)

    def test_incomplete_or_double_counted_components_rejected(self):
        original = reports()
        for field in ("prover_ms", "verifier_ms", "proof_bytes", "missing"):
            data = copy.deepcopy(original)
            parts = data[0]["samples"][1]["timings"]["constraint_breakdown"]
            if field == "missing":
                del parts["shared"]
            else:
                parts["copy"][field] += 1
            with self.subTest(field=field), self.assertRaises(ValueError):
                bench.build_e2e_summary(data)

    def test_warmup_sizes_and_component_shape_are_checked(self):
        for missing in (False, True):
            data = reports()
            parts = data[0]["samples"][0]["timings"]["constraint_breakdown"]
            if missing:
                del data[0]["samples"][0]["timings"]["constraint_breakdown"]
            else:
                parts["gate"]["proof_bytes"] += 1
                parts["shared"]["proof_bytes"] -= 1
            with self.subTest(missing=missing), self.assertRaises(ValueError):
                bench.build_e2e_summary(data)


if __name__ == "__main__":
    unittest.main()
