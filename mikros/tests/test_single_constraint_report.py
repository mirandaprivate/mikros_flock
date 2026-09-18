"""Accounting checks for single-instance gate/copy reports."""
import copy
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
