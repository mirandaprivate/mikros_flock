"""Negative audit checks using copied evidence; no synthetic performance results."""
import json
from pathlib import Path
import shutil
import tempfile
import unittest

from report_tree import audit, digest

BASE = Path(__file__).resolve().parents[1]
SOURCE = BASE / "v11/evidence/primary-all-n26-r1"
MANIFEST = BASE / "v11/source_manifest.json"


class AuditTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(dir=Path(__file__).parent, prefix=".audit-test-")
        self.root = Path(self.tmp.name) / "evidence"
        shutil.copytree(SOURCE, self.root)
        self.manifests = {digest(MANIFEST): str(MANIFEST)}

    def tearDown(self):
        self.tmp.cleanup()

    def change(self, name, edit, refresh=True):
        path = self.root / name
        value = json.loads(path.read_text())
        edit(value)
        path.write_text(json.dumps(value))
        if refresh:
            receipt = self.root / "evidence_sha256.json"
            hashes = json.loads(receipt.read_text())
            hashes[name] = digest(path)
            receipt.write_text(json.dumps(hashes))

    def test_original(self):
        self.assertEqual(len(audit(self.root, self.manifests)), 4)

    def test_tampered_raw_is_rejected(self):
        with (self.root / "n26-multiple-mikros.stdout.jsonl").open("a") as f:
            f.write(" ")
        with self.assertRaisesRegex(RuntimeError, "changed evidence"):
            audit(self.root, self.manifests)

    def test_incomplete_rejected_even_with_new_hash(self):
        self.change("status.json", lambda v: v.update(state="running"))
        with self.assertRaisesRegex(RuntimeError, "incomplete run"):
            audit(self.root, self.manifests)

    def test_traced_primary_rejected(self):
        self.change("config.json", lambda v: v["measured_environment"].update(PCS_TRACE="1"))
        with self.assertRaisesRegex(RuntimeError, "unexpected flags"):
            audit(self.root, self.manifests)

    def test_missing_native_feature_rejected(self):
        self.change("hardware.json", lambda v: next(iter(v["selected_cpu_flags"].values())).remove("gfni"))
        with self.assertRaisesRegex(RuntimeError, "missing native ISA"):
            audit(self.root, self.manifests)


if __name__ == "__main__":
    unittest.main()
