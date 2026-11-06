"""Plot schema, unit, missing-value and anonymous-export checks."""
import copy
import csv
import importlib.util
import io
import json
import math
from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0,str(Path(__file__).resolve().parents[1]/"benches"))
import plot_bitwidth_instances as plot


def fixture(bits=(1,2,128)):
    cases=[]
    for b in bits:
        for case in plot.CASES:
            ok=not(b==2 and case=="multiple")
            cases.append({"case":case,"instances":plot.INSTANCES[case],"arithmetization":plot.ARITHMETIZATION[case],
                          "value_bits":b,"kappa":128//b,"log_n":20,"gates_per_instance":1<<20,
                          "total_gates":(1<<20)*plot.INSTANCES[case],"status":"ok" if ok else "memory_skipped",
                          "metrics":{"prover_ms":256000 if ok else None,"verify_ms":4 if ok else None,
                                     "proof_bytes":4096 if ok else None,"ram_mib":2048 if ok else None},
                          "reason":"/home/private-user/secret-host", "raw_directory":"/private/raw"})
    return {"schema_version":1,"experiment":"bitwidth-instances", "config":{"bits":list(bits),"log_n":20,
            "threads":1,"reps":1,"warmups":0},"cases":cases,"identity":{"hostname":"secret-host"}}


class PlotTests(unittest.TestCase):
    def test_total_cost_units_and_interior_gap(self):
        data=plot.normalize(fixture(),"a"*64)
        self.assertEqual(plot.series(data,"multiple","prover")[0],256)
        self.assertTrue(math.isnan(plot.series(data,"multiple","prover")[1]))
        self.assertEqual(plot.series(data,"single","ram"),[2,2,2])
        self.assertEqual(plot.series(data,"single","proof"),[4,4,4])
        self.assertEqual(plot.series(data,"single","verifier"),[4,4,4])
        rows=list(csv.DictReader(io.StringIO(plot.csv_text(data))))
        self.assertEqual(rows[3]["prover_s"],"")
        self.assertEqual(rows[-1]["kappa"],"1")

    def test_absent_case_is_gap_and_bad_data_rejected(self):
        doc=fixture()
        doc["cases"].pop()
        data=plot.normalize(doc,"b"*64)
        self.assertEqual(data["points"][("multiple",128)]["status"],"absent")
        for mutate in (lambda d:d["cases"].append(copy.deepcopy(d["cases"][0])),
                       lambda d:d["cases"][3]["metrics"].update(prover_ms=0),
                       lambda d:d["cases"][0].update(instances=256),
                       lambda d:d["cases"][0].update(kappa=1),
                       lambda d:d["cases"][0]["metrics"].update(prover_ms=float("nan"))):
            doc=fixture();mutate(doc)
            with self.assertRaises(ValueError):plot.normalize(doc,"c"*64)

    def test_provenance_excludes_paths_and_total_is_explicit(self):
        data=plot.normalize(fixture(),"d"*64)
        encoded=json.dumps(plot.provenance(data))
        self.assertNotIn("secret-host",encoded)
        self.assertNotIn("private",encoded)
        self.assertIn("no division by 256",encoded)
        self.assertIn("kappa=1",encoded)

    @unittest.skipUnless(importlib.util.find_spec("matplotlib"),"Matplotlib unavailable")
    def test_pdf_png_svg_and_csv_export(self):
        with tempfile.TemporaryDirectory() as folder:
            source=Path(folder)/"checkpoint.json"
            source.write_text(json.dumps(fixture()),encoding="utf-8")
            out=Path(folder)/"plots"
            plot.export(source,out)
            for suffix in ("pdf","png","svg","csv","provenance.json"):
                self.assertTrue((out/f"bitwidth_instances.{suffix}").stat().st_size>0)
            self.assertTrue((out/"bitwidth_instances_paper.pdf").stat().st_size>0)
            self.assertNotIn("secret-host",(out/"bitwidth_instances.svg").read_text())

    @unittest.skipUnless(importlib.util.find_spec("matplotlib"),"Matplotlib unavailable")
    def test_paper_size_fonts_and_unchanged_data(self):
        import matplotlib.pyplot as plt
        data=plot.normalize(fixture(plot.WIDTHS),"e"*64)
        generic=plot.draw(data)
        paper=plot.draw(data,paper=True)
        try:
            self.assertEqual(list(paper.get_size_inches()),[4.8,4.4])
            self.assertEqual(list(generic.get_size_inches()),[10.6,7.8])
            self.assertEqual([ax.get_yscale() for ax in generic.axes],["log","log","linear","log"])
            self.assertEqual([ax.get_yscale() for ax in paper.axes],["log","log","linear","log"])
            self.assertIsNone(paper._suptitle)
            self.assertIsNone(paper._supxlabel)
            self.assertEqual([x.get_text() for x in paper.legends[0].get_texts()],
                             ["PLONK (single)","Packed R1CS (256)"])
            paper.canvas.draw()
            renderer=paper.canvas.get_renderer()
            legend_box=paper.legends[0].get_window_extent(renderer)
            for small,large in zip(paper.axes,generic.axes):
                self.assertEqual(small.xaxis.label.get_fontsize(),8)
                self.assertEqual(small.yaxis.label.get_fontsize(),8)
                self.assertTrue(all(t.get_fontsize()==8 for t in small.get_xticklabels()+small.get_yticklabels()))
                self.assertEqual(list(small.get_xticks()),list(range(8)))
                self.assertLess(small.get_tightbbox(renderer).y1,legend_box.y0)
                for left,right in zip(small.lines,large.lines):
                    self.assertEqual(left.get_color(),right.get_color())
                    self.assertEqual(list(left.get_xdata()),list(right.get_xdata()))
                    for a,b in zip(left.get_ydata(),right.get_ydata()):
                        self.assertTrue((math.isnan(a) and math.isnan(b)) or a==b)
        finally:
            plt.close(generic)
            plt.close(paper)


if __name__ == "__main__":unittest.main()
