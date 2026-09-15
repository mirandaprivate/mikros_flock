"""Small synthetic parser tests; these never produce experimental evidence."""
import json
from pathlib import Path
import tempfile
import unittest

from common import NATIVE_FEATURES, cases, save
from aggregate_results import PCS_PATH, TIMING_KEYS, aggregate, validate_case
from run_server import measured_overrides


class HarnessTests(unittest.TestCase):
    def test_measured_controls_are_explicit_and_separate_from_trace(self):
        self.assertEqual(measured_overrides([], False), {})
        controls = measured_overrides(["jit-fused", "ordinary-eval"], False)
        self.assertEqual(controls, {"FLOCK_NO_JIT_FUSED": "1", "E2E_ORDINARY_EVAL": "reference"})
        traced = measured_overrides(["jit-fused"], True)
        self.assertEqual(traced["FLOCK_NO_JIT_FUSED"], "1")
        self.assertTrue(all(traced[key] == "1" for key in ("PCS_TRACE", "LIG_PROVE_TRACE", "E2E_PIOP_TRACE")))

    def test_allocator_policy_is_explicit(self):
        self.assertEqual(measured_overrides([], False, "system"), {})
        self.assertEqual(measured_overrides([], False, "glibc-reuse"),
                         {"MALLOC_MMAP_MAX_": "0", "MALLOC_TRIM_THRESHOLD_": "-1"})

    def test_packed_direct_is_explicit_and_default_off(self):
        self.assertEqual(measured_overrides([], False), {})
        self.assertEqual(measured_overrides([], False, packed_direct=True),
                         {"FLOCK_PCS_PACKED_DIRECT": "1"})

    def test_new_engineering_controls_are_independent_opt_ins(self):
        base = {"FLOCK_PCS_PACKED_DIRECT": "1"}
        self.assertEqual(measured_overrides([], False, packed_direct=True), base)
        self.assertEqual(measured_overrides([], False, packed_direct=True, direct_wp_simd=True),
                         {**base, "FLOCK_DIRECT_WP_SIMD": "1"})
        self.assertEqual(measured_overrides([], False, packed_direct=True, gate_selector_simd=True),
                         {**base, "MIKROS_GATE_SELECTOR_SIMD": "1"})
        self.assertEqual(measured_overrides([], False, packed_direct=True, gate_fusion=True),
                         {**base, "MIKROS_GATE_FUSION": "1"})
        self.assertEqual(measured_overrides([], False, packed_direct=True, wp_uninit=True),
                         {**base, "FLOCK_WP_UNINIT": "1"})

    def test_new_evidence_requires_piop_and_claim_fingerprints(self):
        case = cases([14])[0]
        self.write_case(case)
        save(self.logs / "config.json", {"levels": [14], "repetitions": 1,
                                        "seed": 97, "piop_fingerprints": True})
        with self.assertRaisesRegex(RuntimeError, "piop_blake3"):
            validate_case(self.logs, case, 1, 97)

    def test_mikros_tiling_controls_are_independent(self):
        base = {"FLOCK_PCS_PACKED_DIRECT": "1", "FLOCK_WP_UNINIT": "1"}
        controls = {"gate_virtual_product": "MIKROS_GATE_VIRTUAL_PRODUCT",
                    "wh_tiled_weights": "MIKROS_WH_TILED_WEIGHTS",
                    "copy_paired_terminals": "E2E_COPY_PAIRED_TERMINALS",
                    "gate_wh_scratch": "MIKROS_GATE_WH_SCRATCH",
                    "tree_dense_simd": "MIKROS_TREE_DENSE_SIMD",
                    "tree_large_pool": "MIKROS_TREE_LARGE_POOL",
                    "tree_register_fusion": "MIKROS_TREE_REGISTER_FUSION"}
        self.assertEqual(measured_overrides([], False, packed_direct=True, wp_uninit=True), base)
        for flag, env in controls.items():
            self.assertEqual(measured_overrides([], False, packed_direct=True, wp_uninit=True,
                                                **{flag: True}), {**base, env: "1"})
        self.assertEqual(measured_overrides([], False, packed_direct=True, wp_uninit=True,
                                            **dict.fromkeys(controls, True)),
                         {**base, **dict.fromkeys(controls.values(), "1")})

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix=".selftest-", dir=Path(__file__).parent)
        self.logs = Path(self.temp.name)
        save(self.logs / "config.json", {"levels": [14], "repetitions": 1, "seed": 97})
        self.status = {"state": "running", "cases": []}
        save(self.logs / "status.json", self.status)

    def tearDown(self):
        self.temp.cleanup()

    def write_case(self, case, *, bad_native=False, commitment="a"):
        meta = {"type": "metadata", **case, "threads": 64, "all_core_threads": 64, "seed": 97, "repetitions": 1,
                "target_features": {k: True for k in NATIVE_FEATURES}}
        if bad_native:
            meta["target_features"]["gfni"] = False
        params = {"m": 22, "log_inv_rate": 1, "profile": "Fast", "log_batch_size": 6}
        times = {k: 1.0 for k in TIMING_KEYS}
        times.update(prover_ms=10.0, witness_claims=2, public_claims=2)
        row = {"type": "sample", **case, "threads": 64, "seed": 97, "repetition": 0,
               "times": times, "verify_ms": 0.5, "verified": True, "transcript_match": True,
               "pcs_config_match": True, "pcs_path": PCS_PATH, "native_pcs_identity_transport": True,
               "witness_pcs_params": params, "public_pcs_params": params,
               "witness_commitment_blake3": commitment * 64, "public_commitment_blake3": "b" * 64,
               "proof_blake3": "c" * 64, "proof_bytes": 300}
        (self.logs / f"{case['id']}.stdout.jsonl").write_text(json.dumps(meta) + "\n" + json.dumps(row) + "\n")
        (self.logs / f"{case['id']}.resource.txt").write_text(
            "\tMaximum resident set size (kbytes): 2048\n\tExit status: 0\n\tSwaps: 0\n")
        self.status["cases"].append({"id": case["id"], "state": "complete"})
        save(self.logs / "status.json", self.status)

    def test_incomplete_never_gets_a_ratio(self):
        self.write_case(cases([14])[0])
        result = aggregate(self.logs)
        self.assertFalse(result["all_requested_cases_complete"])
        self.assertEqual(sum(c["state"] == "not_run" for c in result["cases"]), 3)
        self.assertTrue(all("mikros_over_flock" not in c for c in result["comparisons"]))
        self.assertTrue((self.logs / "single_instance.log").is_file())
        self.assertTrue((self.logs / "multiple_instance.log").is_file())

    def test_matching_cases_have_measured_ratios(self):
        for case in cases([14]):
            self.write_case(case)
        result = aggregate(self.logs)
        self.assertTrue(result["all_comparisons_valid"])
        self.assertTrue(all(c["mikros_over_flock"] == 1 for c in result["comparisons"]))

    def test_wrong_commitment_invalidates_comparison(self):
        for i, case in enumerate(cases([14])):
            self.write_case(case, commitment="d" if i == 0 else "a")
        result = aggregate(self.logs)
        self.assertEqual(result["comparisons"][0]["state"], "invalid")
        self.assertNotIn("mikros_over_flock", result["comparisons"][0])

    def test_multiple_only_preserves_full_batch_size_and_comparison(self):
        selected = cases([30], ["multiple"])
        self.assertEqual(len(selected), 2)
        self.assertTrue(all(c["total_gates"] == 1 << 37 and c["instances"] == 128 for c in selected))
        save(self.logs / "config.json", {"levels": [30], "modes": ["multiple"], "repetitions": 1, "seed": 97})
        for case in selected:
            self.write_case(case)
        result = aggregate(self.logs)
        self.assertTrue(result["all_comparisons_valid"])
        self.assertEqual(len(result["cases"]), 2)
        self.assertEqual(len(result["comparisons"]), 1)

    def test_false_native_flag_is_rejected(self):
        case = cases([14])[0]
        self.write_case(case, bad_native=True)
        with self.assertRaisesRegex(RuntimeError, "native features"):
            validate_case(self.logs, case, 1, 97)


if __name__ == "__main__":
    unittest.main()
