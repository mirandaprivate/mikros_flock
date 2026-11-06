"""Synthetic parser/orchestration checks; no prover timings are measured here."""
import contextlib
import copy
import io
import json
from pathlib import Path
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "benches"))
import fri_flock_compare_bench as bench


def options(*extra):
    return bench.parse_options(["--worker-bin", __file__, "--log-n", "14", "--threads", "4",
                                "--reps", "2", "--warmups", "1", *extra])


def records(system, circuit="paired", log_n=14):
    geometry = bench.native.circuit_geometry(log_n, circuit)
    gates = geometry["genuine_gate_count"]
    meta = {"type": "metadata", "system": system, "mode": "single", "instances": 1,
            "log_gates": log_n, "threads": 4, "all_core_threads": 4, "seed": 97,
            "repetitions": 2, "warmups": 1,
            "public_io": [1, 0], "fixture_digest": "a" * 64,
            **{key: value for key, value in geometry.items() if key != "r1cs_rows"}}
    if system == "mikros":
        meta.update(arithmetization="PLONK", pcs_backend=bench.fri.BACKEND,
                    gates_per_instance=gates, total_gates=gates,
                    comparison_fixture_digest="a" * 64, fixture_digest="b" * 64,
                    matrix_authentication="preprocessed public PCS", permutation_proof=True,
                    pcs_profile=dict(bench.fri.SPEC.required_profile),
                    witness_pcs_parameters={"words": 32768}, public_pcs_parameters={"words": 32768},
                    **bench.fri.packing_geometry(log_n, 1))
    else:
        meta.update(arithmetization="native R1CS", pcs_backend="FLOCK native Ligerito Fast",
                    baseline_skip=7, pcs_packed_direct=True, matrix_authentication="public CSC",
                    permutation_proof=False, r1cs_rows=geometry["r1cs_rows"], r1cs_outer_blocks=1, C="identity",
                    witness_words=max(geometry["r1cs_rows"] // 128, 1 << 15))
    result = [meta]
    for index, scale in enumerate((100, 1, 2)):
        row = {"type": "sample", "system": system, "mode": "single", "instances": 1,
               "log_gates": log_n, "threads": 4, "seed": 97,
               "gates_per_instance": gates, "total_gates": gates,
               "warmup": index == 0, "repetition": 0 if index == 0 else index - 1,
               "verified": True, "transcript_match": True, "pcs_config_match": True,
               "proof_bytes": 4096, "verify_ms": float(scale), "peak_rss_kib": 8192, "offline_ms": 5.0}
        if system == "mikros":
            times = {key: float(scale) for key in bench.fri.PROVER_PHASES}
            times.update(prover_ms=10.0 * scale, witness_claims=11, public_claims=5 + 3 * (log_n + 2), pcs_claims=2)
            row.update(times=times, verifier_times={key: 0.1 * scale for key in bench.fri.VERIFIER_PHASES},
                       value_bits=1, pcs_complete=True, witness_pcs_proof_bytes=1000, public_pcs_proof_bytes=2000,
                       proof_blake3="c" * 64, proof_bytes=4096 + index)
            row["constraint_breakdown"] = {
                "gate": {"prover_ms": 1.0 * scale, "verifier_ms": 0.1 * scale, "proof_bytes": 256},
                "copy": {"prover_ms": 3.0 * scale, "verifier_ms": 0.2 * scale, "proof_bytes": 2400},
                "shared": {"prover_ms": 6.0 * scale, "verifier_ms": 0.7 * scale, "proof_bytes": row["proof_bytes"] - 2656}}
        else:
            row.update(times={"commit_ms": 2.0 * scale, "matrix_apply_ms": 2.0 * scale,
                              "gates_ms": 2.0 * scale, "lincheck_input_ms": 1.0 * scale,
                              "lincheck_ms": 3.0 * scale, "witness_pcs_open_ms": 2.0 * scale,
                              "serialization_ms": 1.0 * scale, "prover_ms": 20.0 * scale},
                       verifier_times={"gates_ms": 0.1 * scale, "lincheck_ms": 0.7 * scale, "witness_pcs_ms": 0.1 * scale})
        result.append(row)
    return result


def encode(rows):
    return "\n".join(json.dumps(row) for row in rows) + "\n"


def reports():
    return [bench.parse_worker(encode(records(system)), options(), system, 2048) for system in bench.SYSTEMS]


class FriFlockCompareBenchTests(unittest.TestCase):
    def test_default_entries_and_separate_historical_outputs(self):
        args = bench.parse_options(["--worker-bin", __file__])
        self.assertEqual((args.log_n, args.threads, args.reps, args.warmups, args.system), (25, 1, 1, 0, "both"))
        self.assertEqual(args.circuit, "paired")
        self.assertEqual(args.output, bench.ROOT / "target/paper-results/single-fri-vs-flock-2-25.log")
        args = bench.parse_options(["--worker-bin", __file__, "--experiment", "circuit-sat"])
        self.assertEqual((args.log_n, args.threads, args.reps, args.warmups), (30, 64, 1, 0))
        self.assertEqual(args.output, bench.ROOT / "target/paper-results/circuit-sat-fri-vs-flock-2-30.log")
        self.assertNotEqual(options("--system", "mikros").output, options("--system", "flock").output)
        near_full = options("--circuit", "near-full", "--log-n", "30")
        self.assertEqual(near_full.output, bench.ROOT / "target/paper-results/single-near-full-fri-vs-flock-2-30.log")
        for flags in (("--output", "single_2_25.log"), ("--output", "e2e_flock_sha256.log"),
                      ("--log-n", "31"), ("--threads", "0"), ("--reps", "0"), ("--warmups", "-1"),
                      ("--cpu", "0"), ("--system", "raafold"), ("--circuit", "other")):
            with self.subTest(flags=flags), contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                options(*flags)

    def test_near_full_matches_statement_and_keeps_distinct_pcs_dimensions(self):
        for log_n in (14, 30):
            args = options("--circuit", "near-full", "--log-n", str(log_n))
            result = [bench.parse_worker(encode(records(system, "near-full", log_n)), args, system, 2048)
                      for system in bench.SYSTEMS]
            bench.build_summary(result)
            statement = result[0]["comparison_statement"]
            self.assertEqual(statement["genuine_gate_count"], (1 << log_n) - 257)
            self.assertEqual(statement["private_input_bits"], 256)
            self.assertEqual(statement["plonk_domain_size"], 1 << log_n)
            self.assertEqual(statement["copy_domain_size"], 4 << log_n)
            self.assertEqual(result[1]["metadata"]["r1cs_rows"], 1 << log_n)
            self.assertEqual(result[0]["metadata"]["witness_words"], max((4 << log_n) // 128, 1 << 15))
            self.assertEqual(result[1]["metadata"]["witness_words"], max((1 << log_n) // 128, 1 << 15))

    def test_near_full_rejects_wrong_layout_inputs_domains_gate_count_and_pcs_capacity(self):
        args = options("--circuit", "near-full", "--log-n", "30")
        for system in bench.SYSTEMS:
            for field, value in (("circuit_layout", "paired"), ("private_input_bits", 1 << 30),
                                 ("genuine_gate_count", 1 << 30), ("plonk_domain_size", (1 << 30) - 257),
                                 ("copy_domain_size", 1 << 30), ("witness_words", 12345)):
                rows = records(system, "near-full", 30)
                rows[0][field] = value
                with self.subTest(system=system, field=field), self.assertRaises(ValueError):
                    bench.parse_worker(encode(rows), args, system, 2048)
            for field in ("circuit_layout", "private_input_bits", "plonk_domain_size", "copy_domain_size"):
                rows = records(system, "near-full", 30)
                del rows[0][field]
                with self.subTest(system=system, missing=field), self.assertRaises(ValueError):
                    bench.parse_worker(encode(rows), args, system, 2048)
            rows = records(system, "near-full", 30)
            rows[1]["gates_per_instance"] = 1 << 30
            with self.subTest(system=system, sample="padded count"), self.assertRaises(ValueError):
                bench.parse_worker(encode(rows), args, system, 2048)
        rows = records("flock", "near-full", 30)
        rows[0]["r1cs_rows"] = 4 << 30
        with self.assertRaises(ValueError):
            bench.parse_worker(encode(rows), args, "flock", 2048)
        for system in bench.SYSTEMS:
            with self.subTest(system=system, layout="unexpected near-full"), self.assertRaises(ValueError):
                bench.parse_worker(encode(records(system, "near-full")), options(), system, 2048)

    def test_different_backends_and_protocol_digests_match_same_circuit(self):
        result = reports()
        self.assertNotEqual(result[0]["metadata"]["pcs_backend"], result[1]["metadata"]["pcs_backend"])
        self.assertNotEqual(result[0]["metadata"]["fixture_digest"], result[1]["metadata"]["fixture_digest"])
        summary = bench.build_summary(result)
        self.assertEqual(summary["mikros"]["prover_ms"]["median"], 15)
        self.assertEqual(summary["flock"]["prover_ms"]["median"], 30)
        self.assertEqual(summary["mikros"]["proof_bytes"]["median"], 4097.5)
        self.assertEqual(summary["mikros"]["constraint_breakdown"]["copy"]["prover_ms"]["median"], 4.5)
        self.assertNotIn("offline_ms", summary["mikros"]["prover_phases_ms"])

    def test_requires_matched_semantic_statement_but_not_witness_layout(self):
        for field in bench.MATCHED_FIELDS:
            result = reports()
            result[1]["comparison_statement"][field] = "different"
            with self.subTest(field=field), self.assertRaisesRegex(ValueError, field):
                bench.build_summary(result)
        result = reports()
        result[1]["metadata"]["witness_words"] = 12345
        bench.build_summary(result)

    def test_reuses_complete_fri_and_native_skip7_validation(self):
        for system, mutate in (("mikros", lambda r: r[1].update(pcs_complete=False)),
                               ("mikros", lambda r: r[0]["pcs_profile"].update(native_queries=1)),
                               ("mikros", lambda r: r[0].pop("comparison_fixture_digest")),
                               ("mikros", lambda r: r[0].update(permutation_proof=False)),
                               ("flock", lambda r: r[0].update(baseline_skip=6)),
                               ("flock", lambda r: r[0].update(C="committed")),
                               ("flock", lambda r: r[0].update(r1cs_rows=1 << 14)),
                               ("flock", lambda r: r[1].update(pcs_config_match=False))):
            rows = records(system)
            mutate(rows)
            with self.subTest(system=system), self.assertRaises(ValueError):
                bench.parse_worker(encode(rows), options(), system, 2048)
        for system in bench.SYSTEMS:
            for value in ([True, 0], [2, 0], [1, 1], None):
                rows = records(system)
                rows[0]["public_io"] = value
                with self.subTest(system=system, value=value), self.assertRaises(ValueError):
                    bench.parse_worker(encode(rows), options(), system, 2048)

    def test_component_accounting_counts_public_opening_once(self):
        for field in ("prover_ms", "verifier_ms", "proof_bytes"):
            rows = records("mikros")
            rows[1]["constraint_breakdown"]["copy"][field] += 1
            with self.subTest(field=field), self.assertRaises(ValueError):
                bench.parse_worker(encode(rows), options(), "mikros", 2048)
        rows = records("mikros")
        rows[1]["constraint_breakdown"]["copy"]["prover_ms"] -= 100
        rows[1]["constraint_breakdown"]["shared"]["prover_ms"] += 100
        with self.assertRaisesRegex(ValueError, "permutation argument accounting"):
            bench.parse_worker(encode(rows), options(), "mikros", 2048)

    def test_native_full_phase_accounting_is_required(self):
        for mutate in (lambda r: r["verifier_times"].pop("lincheck_ms"),
                       lambda r: r.pop("verifier_times"),
                       lambda r: r["verifier_times"].update(gates_ms=float("nan")),
                       lambda r: r["verifier_times"].update(gates_ms=-1),
                       lambda r: r["verifier_times"].update(gates_ms=True),
                       lambda r: r["verifier_times"].update(gates_ms=10000),
                       lambda r: r["times"].pop("matrix_apply_ms"),
                       lambda r: r["times"].update(lincheck_ms=10000)):
            rows = records("flock")
            mutate(rows[1])
            with self.subTest(mutate=mutate), self.assertRaises(ValueError):
                bench.parse_worker(encode(rows), options(), "flock", 2048)

    def test_single_backend_has_no_inferred_pairwise_ratio(self):
        for index, system in enumerate(bench.SYSTEMS):
            summary = bench.build_summary([reports()[index]], system)
            self.assertEqual(set(summary), {system})
        with self.assertRaises(ValueError):
            bench.build_summary([reports()[0]])

    def test_runs_isolated_workers_and_keeps_original_worker_arguments(self):
        for selected, expected in (("both", ["mikros", "flock"]), ("mikros", ["mikros"]), ("flock", ["flock"])):
            with self.subTest(selected=selected), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                worker = root / "worker"
                worker.write_text("synthetic executable")
                output = root / "results/compare.log"
                args = options("--worker-bin", str(worker), "--time-bin", str(worker), "--output", str(output), "--system", selected)
                calls = []

                def fake_worker(command, **kwargs):
                    system = command[command.index("--system") + 1]
                    calls.append(system)
                    self.assertIsNone(kwargs["cpu"])
                    self.assertEqual(kwargs["env"]["RAYON_NUM_THREADS"], "4")
                    self.assertEqual(kwargs["env"]["FLOCK_VERIFY_THREADS"], "4")
                    self.assertEqual(kwargs["env"]["FLOCK_PCS_PACKED_DIRECT"], "1")
                    self.assertNotIn("ADDITIVE_FRI_PROFILE_STAGES", kwargs["env"])
                    self.assertNotIn("--mode", command)
                    self.assertEqual(command[command.index("--circuit") + 1], "paired")
                    kwargs["stdout"].write(encode(records(system)))
                    Path(command[3]).write_text("Maximum resident set size (kbytes): 2048\nExit status: 0\n")
                    return 0

                with patch.object(bench.subprocess, "run", return_value=SimpleNamespace(returncode=0, stdout="GNU time")), \
                     patch.object(bench.os, "sched_getaffinity", return_value={2, 4, 6, 8}, create=True), \
                     patch.object(bench, "run_streamed", side_effect=fake_worker), patch("builtins.print"):
                    doc = bench.run(args)
                self.assertEqual(calls, expected)
                self.assertEqual(doc["cpu_affinity"], [2, 4, 6, 8])
                self.assertEqual(set(doc["systems"]), set(expected))
                log = output.read_text()
                self.assertEqual("FLOCK/Mikros prover time: 2.000x" in log, selected == "both")
                self.assertIn("different PCS backends", log)
                if "mikros" in expected:
                    self.assertIn("Permutation argument", log)
                    self.assertIn("public PCS opening", log)
                self.assertNotIn(directory, log)
                self.assertNotIn(directory, output.with_suffix(".json").read_text())

    def test_failed_or_mismatched_worker_preserves_previous_report(self):
        for failed in (True, False):
            with self.subTest(failed=failed), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                worker = root / "worker"
                worker.write_text("synthetic executable")
                output = root / "compare.log"
                output.write_text("previous validated result")
                args = options("--worker-bin", str(worker), "--time-bin", str(worker), "--output", str(output))

                def bad_worker(command, **kwargs):
                    if failed:
                        return 101
                    system = command[command.index("--system") + 1]
                    rows = records(system)
                    if system == "flock":
                        rows[0]["public_io"] = [0, 0]
                    kwargs["stdout"].write(encode(rows))
                    Path(command[3]).write_text("Maximum resident set size (kbytes): 2048\nExit status: 0\n")
                    return 0

                with patch.object(bench.subprocess, "run", return_value=SimpleNamespace(returncode=0, stdout="GNU time")), \
                     patch.object(bench.os, "sched_getaffinity", return_value={2, 4, 6, 8}, create=True), \
                     patch.object(bench, "run_streamed", side_effect=bad_worker), patch("builtins.print"):
                    with self.assertRaises(ValueError):
                        bench.run(args)
                self.assertEqual(output.read_text(), "previous validated result")

    def test_near_full_worker_arguments_and_report_use_actual_gate_count(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            worker = root / "worker"
            worker.write_text("synthetic executable")
            output = root / "near-full.log"
            args = options("--circuit", "near-full", "--worker-bin", str(worker),
                           "--time-bin", str(worker), "--output", str(output))
            calls = []

            def fake_worker(command, **kwargs):
                system = command[command.index("--system") + 1]
                self.assertEqual(command[command.index("--circuit") + 1], "near-full")
                calls.append(system)
                kwargs["stdout"].write(encode(records(system, "near-full")))
                Path(command[3]).write_text("Maximum resident set size (kbytes): 2048\nExit status: 0\n")
                return 0

            with patch.object(bench.subprocess, "run", return_value=SimpleNamespace(returncode=0, stdout="GNU time")), \
                 patch.object(bench.os, "sched_getaffinity", return_value={2, 4, 6, 8}, create=True), \
                 patch.object(bench, "run_streamed", side_effect=fake_worker), patch("builtins.print"):
                doc = bench.run(args)
            self.assertEqual(calls, ["mikros", "flock"])
            self.assertEqual(doc["circuit"], "near-full")
            self.assertEqual(doc["circuit_geometry"]["genuine_gate_count"], 16127)
            text = output.read_text()
            self.assertIn("16127 genuine AND/XOR gates; 256 independent binary inputs", text)
            self.assertIn("16384 R1CS rows", text)
            self.assertIn("16384 padded gate rows; 65536 copy-domain positions", text)
            self.assertNotIn("16384 genuine AND/XOR gates", text)


if __name__ == "__main__":
    unittest.main()
