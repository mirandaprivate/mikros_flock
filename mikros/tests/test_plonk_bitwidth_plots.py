"""Data integrity and actual Matplotlib exports for the PLONK sweep plots."""
import copy
import hashlib
import importlib.util
import json
import math
from pathlib import Path
import tempfile
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / "benches" / "plot_plonk_bitwidth_sweep.py"
SPEC = importlib.util.spec_from_file_location("plonk_sweep_plot", SCRIPT)
plot = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(plot)


def fixture():
    cases = []
    for n in [20, 22, 24]:
        for b in [1, 128]:
            ok = not (n == 22 and b == 128)
            cases.append({
                "log_n": n, "value_bits": b,
                "status": "ok" if ok else "memory_skipped",
                "reason": None if ok else "/home/private-user/secret-host: memory budget",
                "metrics": {
                    "prover_ms": 2500.0 * 2**(n-20) if ok else None,
                    "verify_ms": 12.5 if ok else None,
                    "proof_bytes": 2048 if ok else None,
                    "ram_mib": 3072.0 * 2**(n-20) if ok else None,
                },
                "raw_directory": "/home/private-user/secret-host/raw",
            })
    return {"schema_version": 1, "experiment": "plonk-bitwidth-sweep",
            "config": {"log_ns": [20,22,24], "bits": [1,128],
                       "threads": 64, "reps": 1, "warmups": 0},
            "identity": {"hostname": "secret-host"}, "cases": cases}


class PlotDataTests(unittest.TestCase):
    def test_missing_interior_point_remains_nan_and_unit_conversion(self):
        data = plot.normalize(fixture(), "a" * 64)
        values = plot.series(data, 128, "prover")
        self.assertEqual(values[0], 2.5)
        self.assertTrue(math.isnan(values[1]))
        self.assertEqual(values[2], 40)
        self.assertEqual(plot.series(data, 1, "ram"), [3, 12, 48])
        self.assertEqual(plot.series(data, 1, "proof"), [2, 2, 2])
        self.assertEqual(plot.series(data, 1, "verifier"), [12.5] * 3)
        self.assertEqual(data["missing"], [{"log_n": 22, "value_bits": 128, "status": "memory_skipped"}])

    def test_absent_configured_case_is_a_gap_not_an_omitted_x_coordinate(self):
        raw = fixture()
        raw["cases"] = [c for c in raw["cases"] if (c["log_n"], c["value_bits"]) != (22,1)]
        data = plot.normalize(raw, "b" * 64)
        self.assertEqual(data["logs"], (20,22,24))
        self.assertTrue(math.isnan(plot.series(data, 1, "prover")[1]))
        self.assertEqual(data["points"][(22,1)]["status"], "absent")

    def test_all_non_ok_statuses_remain_missing(self):
        for status in plot.STATUSES - {"ok"}:
            with self.subTest(status=status):
                raw = fixture()
                raw["cases"][3]["status"] = status
                data = plot.normalize(raw, "c" * 64)
                for metric in plot.METRICS:
                    self.assertTrue(math.isnan(plot.series(data, 128, metric)[1]))

    def test_ambiguous_or_stale_measurements_are_rejected(self):
        bad = fixture()
        bad["cases"].append(copy.deepcopy(bad["cases"][0]))
        with self.assertRaisesRegex(ValueError, "duplicate case"):
            plot.normalize(bad, "d" * 64)
        bad = fixture()
        bad["cases"][3]["metrics"]["prover_ms"] = 0
        with self.assertRaisesRegex(ValueError, "non-ok metric"):
            plot.normalize(bad, "d" * 64)
        for value in [None, True, float("nan"), float("inf"), -1, 0]:
            bad = fixture()
            bad["cases"][0]["metrics"]["prover_ms"] = value
            with self.subTest(value=value), self.assertRaises(ValueError):
                plot.normalize(bad, "d" * 64)

    def test_invalid_schema_grid_and_unknown_status_are_rejected(self):
        for mutate in [
            lambda x: x.update(experiment="unrelated-benchmark"),
            lambda x: x["config"].update(bits=[1,3]),
            lambda x: x["config"].update(log_ns=[20,20]),
            lambda x: x["config"].update(threads=True),
            lambda x: x["cases"][0].update(status="maybe"),
        ]:
            bad = fixture()
            mutate(bad)
            with self.assertRaises(ValueError):
                plot.normalize(bad, "e" * 64)

    def test_sha_tracks_exact_source_bytes_and_provenance_is_anonymous(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "checkpoint.json"
            source = (json.dumps(fixture()) + "\n").encode()
            path.write_bytes(source)
            data = plot.load_checkpoint(path)
            expected = hashlib.sha256(source).hexdigest()
            self.assertEqual(data["source_sha256"], expected)
            scales = {name: spec[3] for name, spec in plot.METRICS.items()}
            manifest = plot.provenance(data, scales)
            exported = json.dumps(manifest)
            self.assertIn(expected, exported)
            for secret in ["private-user", "secret-host", temporary]:
                self.assertNotIn(secret, exported)
            self.assertEqual(manifest["threads"], 64)
            self.assertEqual(manifest["samples_per_point"], 1)
            self.assertIn("PCS", manifest["scope"])
            self.assertIn("64 workers", plot.caption(data))


try:
    import matplotlib
except ImportError:
    matplotlib = None


@unittest.skipIf(matplotlib is None, "Matplotlib is required for rendering tests")
class PlotRenderTests(unittest.TestCase):
    def test_matplotlib_path_breaks_at_missing_points(self):
        matplotlib.use("Agg")
        from matplotlib import pyplot as plt
        from matplotlib.path import Path as PlotPath
        fig, ax = plt.subplots()
        try:
            data = plot.normalize(fixture(), "f" * 64)
            plot.draw_axis(ax, data, "prover", "log")
            line = ax.lines[1]
            self.assertTrue(math.isnan(line.get_ydata()[1]))
            moves = [code for _, code in line.get_path().iter_segments() if code == PlotPath.MOVETO]
            self.assertEqual(len(moves), 2, "missing interior point must start a new line segment")
            self.assertEqual(ax.get_xscale(), "log")
            self.assertEqual(ax.get_yscale(), "log")
        finally:
            plt.close(fig)

    def test_five_figures_three_formats_and_anonymous_metadata(self):
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary)
            data = plot.normalize(fixture(), "f" * 64)
            result = plot.render(data, output, dpi=50)
            self.assertEqual(len(result["figures"]), 15)
            self.assertEqual(len(list(output.glob("*.png"))), 5)
            self.assertEqual(len(list(output.glob("*.pdf"))), 5)
            self.assertEqual(len(list(output.glob("*.svg"))), 5)
            for artifact in result["figures"]:
                raw = (output / artifact["name"]).read_bytes()
                self.assertTrue(raw)
                self.assertEqual(hashlib.sha256(raw).hexdigest(), artifact["sha256"])
            svg = (output / "plonk_bitwidth_sweep.svg").read_text(encoding="utf-8")
            self.assertIn("f" * 64, svg)
            self.assertIn("64 workers", svg)
            self.assertNotIn("private-user", svg)
            self.assertNotIn("secret-host", svg)
            self.assertNotIn("<dc:date>", svg)
            pdf = (output / "plonk_bitwidth_sweep.pdf").read_bytes()
            self.assertNotIn(b"/CreationDate", pdf)
            self.assertNotIn(b"/Author", pdf)

    def test_requested_log_scale_does_not_silently_drop_measured_zero(self):
        from matplotlib import pyplot as plt
        raw = fixture()
        raw["cases"][0]["metrics"]["verify_ms"] = 0
        data = plot.normalize(raw, "a" * 64)
        fig, ax = plt.subplots()
        try:
            with self.assertRaisesRegex(ValueError, "measured zero"):
                plot.draw_axis(ax, data, "verifier", "log")
        finally:
            plt.close(fig)


if __name__ == "__main__":
    unittest.main()
