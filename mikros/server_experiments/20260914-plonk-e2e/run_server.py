#!/usr/bin/env python3
"""Serial 64-thread native E2E pilots/targets; no SSH, transfers or cleanup."""
import argparse
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import time

from common import (AUTHORIZED_ROOT, CPUS, CPU_SPEC, LEVELS, MODES, NATIVE_FEATURES, TOOLCHAIN,
                    cases, require, save, sha256, utc, verify_sources)
from aggregate_results import aggregate, evidence_hashes, validate_case

DEFAULT_TARGET = AUTHORIZED_ROOT / "v14-packed-aggregate-20260914/source/target"
OPTIMIZATION_OFF = {
    "row-grouping": ("FLOCK_NO_MERGED_ROW_GROUPING", "1"),
    "jit-fused": ("FLOCK_NO_JIT_FUSED", "1"),
    "ordinary-eval": ("E2E_ORDINARY_EVAL", "reference"),
    "label-fold": ("E2E_PUBLIC_LABEL_FOLD", "reference"),
    "paired-bitbanks": ("FLOCK_NO_PAIRED_BITBANKS", "1"),
    "leaf-cache": ("E2E_COPY_LEAF_CACHE", "off"),
    "tree-fusion": ("MIKROS_NO_TREE_FUSION", "1"),
    "sparse-bitbanks": ("FLOCK_NO_SPARSE_BITBANKS", "1"),
    "constant-bitbanks": ("FLOCK_NO_CONSTANT_BITBANKS", "1"),
    "tree-pool": ("MIKROS_NO_TREE_POOL", "1"),
    "direct-block-parallel": ("FLOCK_NO_DIRECT_BLOCK_PARALLEL", "1"),
}


def measured_overrides(disabled, trace, allocator="system", packed_direct=False,
                       direct_wp_simd=False, gate_selector_simd=False, gate_fusion=False,
                       wp_uninit=False, gate_virtual_product=False, wh_tiled_weights=False,
                       copy_paired_terminals=False, gate_wh_scratch=False,
                       tree_dense_simd=False, tree_large_pool=False, tree_register_fusion=False):
    result = dict(OPTIMIZATION_OFF[name] for name in disabled)
    if trace:
        result.update(PCS_TRACE="1", LIG_PROVE_TRACE="1", E2E_PIOP_TRACE="1")
    if allocator == "glibc-reuse":
        result.update(MALLOC_MMAP_MAX_="0", MALLOC_TRIM_THRESHOLD_="-1")
    if packed_direct:
        result["FLOCK_PCS_PACKED_DIRECT"] = "1"
    if direct_wp_simd:
        result["FLOCK_DIRECT_WP_SIMD"] = "1"
    if gate_selector_simd:
        result["MIKROS_GATE_SELECTOR_SIMD"] = "1"
    if gate_fusion:
        result["MIKROS_GATE_FUSION"] = "1"
    if wp_uninit:
        result["FLOCK_WP_UNINIT"] = "1"
    if gate_virtual_product:
        result["MIKROS_GATE_VIRTUAL_PRODUCT"] = "1"
    if wh_tiled_weights:
        result["MIKROS_WH_TILED_WEIGHTS"] = "1"
    if copy_paired_terminals:
        result["E2E_COPY_PAIRED_TERMINALS"] = "1"
    if gate_wh_scratch:
        result["MIKROS_GATE_WH_SCRATCH"] = "1"
    if tree_dense_simd:
        result["MIKROS_TREE_DENSE_SIMD"] = "1"
    if tree_large_pool:
        result["MIKROS_TREE_LARGE_POOL"] = "1"
    if tree_register_fusion:
        result["MIKROS_TREE_REGISTER_FUSION"] = "1"
    return result


def resource_values(root):
    mem = {line.split(":")[0]: int(line.split()[1]) for line in Path("/proc/meminfo").read_text().splitlines()
           if line.startswith(("MemTotal:", "MemAvailable:", "SwapTotal:", "SwapFree:"))}
    disk = os.statvfs(root)
    return {"utc": utc(), "memory_kib": mem, "disk_free_bytes": disk.f_bavail * disk.f_frsize,
            "disk_path": str(root), "loadavg": os.getloadavg(),
            "affinity": sorted(os.sched_getaffinity(0))}


def cpu_counters():
    result = {}
    for line in Path("/proc/stat").read_text().splitlines():
        fields = line.split()
        if fields[0].startswith("cpu") and fields[0][3:].isdigit() and int(fields[0][3:]) in CPUS:
            values = list(map(int, fields[1:9]))
            result[fields[0]] = (sum(values), values[3] + values[4])
    require(len(result) == 64, "missing selected CPU counters")
    return result


def preflight(root, minimum_gib):
    require(CPUS.issubset(os.sched_getaffinity(0)), "64 requested CPUs are not available")
    first = cpu_counters()
    time.sleep(1)
    second = cpu_counters()
    total = sum(second[c][0] - first[c][0] for c in first)
    idle = sum(second[c][1] - first[c][1] for c in first)
    values = resource_values(root)
    values["selected_cpu_busy_fraction"] = 1 - idle / max(total, 1)
    values["required_available_gib"] = minimum_gib
    return values


def live_progress(pid, stderr):
    # Only this subprocess tree is inspected; no machine-wide process dump.
    pids, pending, rss = [], [pid], 0
    while pending:
        current = pending.pop()
        if current in pids:
            continue
        pids.append(current)
        try:
            text = Path(f"/proc/{current}/status").read_text()
            match = re.search(r"^VmRSS:\s+(\d+)", text, re.M)
            rss += int(match.group(1)) if match else 0
            pending.extend(map(int, Path(f"/proc/{current}/task/{current}/children").read_text().split()))
        except (FileNotFoundError, ProcessLookupError):
            pass
    stage = None
    try:
        with stderr.open("rb") as stream:
            stream.seek(max(0, stderr.stat().st_size - 8192))
            for line in stream.read().decode(errors="replace").splitlines():
                if line.startswith("stage="):
                    stage = line[6:]
    except FileNotFoundError:
        pass
    return {"pids": pids, "current_process_tree_rss_kib": rss, "last_stage": stage, "utc": utc()}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--logs", type=Path, default=Path("results-plonk"))
    parser.add_argument("--levels", type=int, choices=LEVELS, nargs="+", default=list(LEVELS))
    parser.add_argument("--modes", choices=MODES, nargs="+", default=list(MODES))
    parser.add_argument("--repetitions", type=int, default=1)
    parser.add_argument("--seed", type=int, default=97)
    parser.add_argument("--target-dir", type=Path, default=DEFAULT_TARGET)
    parser.add_argument("--skip-build", action="store_true")
    parser.add_argument("--skip-tests", action="store_true")
    parser.add_argument("--reuse-evidence", type=Path, help="same-source build receipt required with --skip-build")
    parser.add_argument("--case-timeout", type=int, default=14400)
    parser.add_argument("--minimum-large-memory-gib", type=float, default=350)
    parser.add_argument("--disable-optimization", choices=OPTIMIZATION_OFF, action="append", default=[])
    parser.add_argument("--trace", action="store_true", help="diagnostic phase traces; record separately from primary timings")
    parser.add_argument("--allocator", choices=("system", "glibc-reuse"), default="system",
                        help="N26 or smaller diagnostic: keep freed arena memory for reuse; raises RSS")
    parser.add_argument("--packed-direct", action="store_true",
                        help="experimental native mixed PCS routing, applied to both prover and verifier")
    parser.add_argument("--direct-wp-simd", action="store_true",
                        help="opt-in byte-preserving SIMD for packed-direct W-prime generation")
    parser.add_argument("--gate-selector-simd", action="store_true",
                        help="opt-in Mikros multiple gate kernel for verified selector identities")
    parser.add_argument("--gate-fusion", action="store_true",
                        help="opt-in Mikros multiple gate fold and next-message fusion")
    parser.add_argument("--wp-uninit", action="store_true",
                        help="opt-in native W-prime parallel first writes without full zero initialization")
    parser.add_argument("--gate-virtual-product", action="store_true",
                        help="opt-in Mikros implicit initial virtual product with bounded tiles")
    parser.add_argument("--wh-tiled-weights", action="store_true",
                        help="opt-in Mikros weighted Hadamard row weights generated per tile")
    parser.add_argument("--copy-paired-terminals", action="store_true",
                        help="opt-in Mikros multiple copy two-point terminal evaluation sharing witness loads")
    parser.add_argument("--gate-wh-scratch", action="store_true",
                        help="opt-in ownership handoff of two gate fold buffers to Hadamard IP (requires virtual product)")
    parser.add_argument("--tree-dense-simd", action="store_true",
                        help="opt-in Mikros multiple product-tree contiguous vector loads and register deinterleaving")
    parser.add_argument("--tree-large-pool", action="store_true",
                        help="opt-in Mikros exact-capacity tree pool through log L=32; released before PCS")
    parser.add_argument("--tree-register-fusion", action="store_true",
                        help="opt-in Mikros native tree fold and next message using output registers before stores")
    parser.add_argument("--dry-run", action="store_true", help="print commands only; no filesystem changes or builds")
    args = parser.parse_args()
    require(args.repetitions > 0 and args.case_timeout > 0, "invalid repetitions/timeout")
    require(args.levels == sorted(set(args.levels)), "levels must be distinct and ascending")
    require(args.allocator == "system" or max(args.levels) <= 26,
            "glibc-reuse is evaluated only through N26; large-case memory is not validated")
    require(not args.direct_wp_simd or args.packed_direct,
            "this E2E direct W-prime experiment requires --packed-direct")
    require(not args.gate_wh_scratch or args.gate_virtual_product,
            "gate/WH fold buffer handoff requires --gate-virtual-product")
    root = Path(__file__).resolve().parents[3]
    target = args.target_dir.resolve()
    planned = cases(args.levels, args.modes)
    overrides = measured_overrides(args.disable_optimization, args.trace, args.allocator,
                                   args.packed_direct, args.direct_wp_simd, args.gate_selector_simd,
                                   args.gate_fusion, args.wp_uninit, args.gate_virtual_product,
                                   args.wh_tiled_weights, args.copy_paired_terminals, args.gate_wh_scratch,
                                   args.tree_dense_simd, args.tree_large_pool, args.tree_register_fusion)
    binary = target / "release/examples/e2e_plonk_compare"
    cargo = ["cargo", TOOLCHAIN]
    flags = ["--release", "--offline", "--locked", "-j8", "--message-format=short"]
    build = cargo + ["build", *flags, "-p", "mikros", "--example", "e2e_plonk_compare"]
    if args.dry_run:
        print(json.dumps({"source": str(root), "target": str(target), "build": build,
                          "cpu_affinity": CPU_SPEC, "rustflags": "-C target-cpu=native", "measured_environment": overrides,
                          "cases": [{**c, "command": [str(binary), "--mode", c["mode"], "--system", c["system"],
                            "--log-gates", str(c["log_gates"]), "--threads", "64", "--repetitions", str(args.repetitions),
                            "--seed", str(args.seed)]} for c in planned]}, indent=2))
        return
    require(root.is_relative_to(AUTHORIZED_ROOT) and root.name == "source", "run only inside the authorized source snapshot")
    require(target.is_relative_to(AUTHORIZED_ROOT) and target.name == "target", "target cache is outside authorized experiment scope")
    logs = (root / args.logs).resolve()
    require(logs.is_relative_to(root) and logs != root, "logs must stay under this source snapshot")
    logs.mkdir(parents=True, exist_ok=False)
    env = os.environ.copy()
    removed = []
    for key in list(env):
        if key.startswith(("FLOCK_", "MIKROS_", "E2E_", "MALLOC_")) or key in (
            "GLIBC_TUNABLES", "PCS_TRACE", "LIG_PROVE_TRACE", "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_RUSTFLAGS", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER"):
            removed.append(key)
            env.pop(key)
    env.update(RUSTFLAGS="-C target-cpu=native", RAYON_NUM_THREADS="64",
               RUST_MIN_STACK="16777216", CARGO_TARGET_DIR=str(target))
    env.update(overrides)
    env["PATH"] = str(Path.home() / ".cargo/bin") + ":" + env.get("PATH", "")
    status = {"state": "starting", "started_utc": utc(), "commands": [], "cases": []}
    config = {"schema_version": 1, "levels": args.levels, "modes": args.modes,
              "repetitions": args.repetitions, "seed": args.seed,
              "threads": 64, "cpu_affinity": CPU_SPEC, "toolchain": TOOLCHAIN,
              "rustflags": env["RUSTFLAGS"], "target_dir": str(target), "source_dir": str(root),
              "rust_min_stack_bytes": 16777216,
              "cleared_environment_keys": sorted(removed), "build_skipped": args.skip_build,
              "tests_skipped": args.skip_tests, "case_timeout_seconds": args.case_timeout,
              "minimum_large_memory_gib": args.minimum_large_memory_gib,
              "disabled_optimizations": args.disable_optimization, "diagnostic_trace": args.trace,
              "allocator_policy": args.allocator,
              "packed_direct": args.packed_direct, "piop_fingerprints": True,
              "direct_wp_simd": args.direct_wp_simd, "gate_selector_simd": args.gate_selector_simd,
              "gate_fusion": args.gate_fusion,
              "wp_uninit": args.wp_uninit,
              "gate_virtual_product": args.gate_virtual_product,
              "wh_tiled_weights": args.wh_tiled_weights,
              "copy_paired_terminals": args.copy_paired_terminals,
              "gate_wh_scratch": args.gate_wh_scratch,
              "tree_dense_simd": args.tree_dense_simd, "tree_large_pool": args.tree_large_pool,
              "tree_register_fusion": args.tree_register_fusion,
              "measured_environment": overrides,
              "pilot_scope": "selected levels only; root manages evidence from earlier invocations when selecting a subset"}
    save(logs / "config.json", config)

    def flush():
        save(logs / "status.json", status)

    def run(name, command, timeout=3600):
        stdout_path, stderr_path = logs / f"{name}.stdout.jsonl", logs / f"{name}.stderr.log"
        full = ["/usr/bin/time", "-v", "-o", str(logs / f"{name}.resource.txt"), "taskset", "-c", CPU_SPEC, *command]
        save(logs / f"{name}.command.json", {"command": full, "cwd": str(root), "env": {
            k: env[k] for k in ("RUSTFLAGS", "RAYON_NUM_THREADS", "RUST_MIN_STACK", "CARGO_TARGET_DIR", *overrides)}})
        event = {"id": name, "started_utc": utc(), "state": "running"}
        status["state"] = name
        status["commands"].append(event)
        flush()
        start = time.monotonic()
        with stdout_path.open("wb") as out, stderr_path.open("wb") as err:
            process = subprocess.Popen(full, cwd=root, env=env, stdout=out, stderr=err, start_new_session=True)
            last_progress = 0.0
            try:
                while process.poll() is None:
                    elapsed = time.monotonic() - start
                    if elapsed > timeout:
                        raise TimeoutError(f"{name} exceeded {timeout} seconds")
                    if elapsed - last_progress >= 30:
                        event["progress"] = live_progress(process.pid, stderr_path)
                        event["wall_seconds"] = elapsed
                        flush()
                        print(json.dumps({"event": "progress", **event}), flush=True)
                        last_progress = elapsed
                    time.sleep(1)
            except BaseException:
                try:
                    os.killpg(process.pid, signal.SIGTERM)
                except ProcessLookupError:
                    pass
                try:
                    process.wait(timeout=5)
                except subprocess.TimeoutExpired:
                    os.killpg(process.pid, signal.SIGKILL)
                    process.wait()
                event.update(state="failed", exit_code=process.returncode, wall_seconds=time.monotonic() - start)
                flush()
                raise
        event.update(state="complete" if process.returncode == 0 else "failed",
                     exit_code=process.returncode, wall_seconds=time.monotonic() - start, finished_utc=utc())
        flush()
        print(json.dumps(event), flush=True)
        require(process.returncode == 0, f"{name}: exit {process.returncode}; raw outputs retained")
        return event

    try:
        manifest, manifest_hash = verify_sources(root)
        config["source_manifest_sha256"] = manifest_hash
        config["base_revision"] = manifest.get("base_revision")
        save(logs / "config.json", config)
        save(logs / "initial.resources.json", preflight(root, 8))
        hardware = {"hostname": os.uname().nodename, "selected_cpus": sorted(CPUS),
                    "resource_snapshot": resource_values(root), "cpu_model": None, "selected_cpu_flags": {}}
        for section in Path("/proc/cpuinfo").read_text().split("\n\n"):
            fields = dict(line.split(":", 1) for line in section.splitlines() if ":" in line)
            fields = {k.strip(): v.strip() for k, v in fields.items()}
            if int(fields.get("processor", "-1")) in CPUS:
                flags_set = set(fields.get("flags", "").split())
                require(NATIVE_FEATURES.issubset(flags_set), "selected processor lacks native ISA")
                hardware["cpu_model"] = fields.get("model name")
                hardware["selected_cpu_flags"][fields["processor"]] = sorted(NATIVE_FEATURES & flags_set)
        require(len(hardware["selected_cpu_flags"]) == 64, "missing selected CPU hardware records")
        save(logs / "hardware.json", hardware)
        require(resource_values(root)["disk_free_bytes"] >= 2 * (1 << 30), "less than 2 GiB free on authorized /home filesystem; no other users' disks are used")
        run("rustc-version", ["rustc", TOOLCHAIN, "-Vv"])
        run("native-cfg", ["rustc", TOOLCHAIN, "--print", "cfg", "-C", "target-cpu=native"])
        cfg = (logs / "native-cfg.stdout.jsonl").read_text()
        require(all(f'target_feature="{f}"' in cfg for f in NATIVE_FEATURES), "rustc native configuration lacks required ISA")
        if args.skip_build:
            require(args.reuse_evidence is not None, "--skip-build requires --reuse-evidence from the same frozen source")
            receipt = json.loads((args.reuse_evidence / "build-receipt.json").read_text())
            require(receipt["source_manifest_sha256"] == manifest_hash and receipt["binary_sha256"] == sha256(binary),
                    "reused binary does not match frozen source/build evidence")
            require(receipt["toolchain"] == TOOLCHAIN and receipt["rustflags"] == env["RUSTFLAGS"],
                    "reused build does not have the required native configuration")
        else:
            for name in manifest["files_sha256"]:
                (root / name).touch()
            run("build", build, 3600)
        require(binary.is_file(), "example binary is missing")
        receipt = {"source_manifest_sha256": manifest_hash, "binary_sha256": sha256(binary),
                   "binary": str(binary), "toolchain": TOOLCHAIN, "rustflags": env["RUSTFLAGS"]}
        save(logs / "build-receipt.json", receipt)
        if not args.skip_tests:
            test_targets = [
                ("mikros", ["-p", "mikros", "--lib"]),
                ("e2e-fixture", ["-p", "mikros", "--example", "e2e_plonk_compare"]),
                ("permutation", ["-p", "mikros-permutation", "--lib"]),
                ("gkr", ["-p", "flock-core", "--lib", "product_gkr::tests"]),
                ("pcs-tensor", ["-p", "flock-core", "--lib", "pcs::merged_tensor::tests"]),
                ("pcs-split-rows", ["-p", "flock-core", "--lib", "split_rows"]),
                ("pcs-column-bitbank", ["-p", "flock-core", "--lib", "column_bitbank"]),
                ("pcs-column-s-hat", ["-p", "flock-core", "--lib", "column_bank_s_hat"]),
                ("pcs-full-rectangle", ["-p", "flock-core", "--lib", "full_rectangle_bank_reuse"]),
                ("pcs-bounded-l0", ["-p", "flock-core", "--lib", "bounded_l0_scratch_preserves_proof_and_transcript"]),
                ("pcs-row-grouping", ["-p", "flock-core", "--lib", "merged_row_grouping"]),
                ("pcs-direct-block-parallel", ["-p", "flock-core", "--lib", "direct_block_parallel"]),
                ("pcs-direct-wp-simd", ["-p", "flock-core", "--lib", "direct_wp_simd"]),
                ("pcs-wp-uninit", ["-p", "flock-core", "--lib", "wp_uninit"]),
                ("pcs-paired-bitbanks", ["-p", "flock-core", "--lib", "paired_bitbanks"]),
                ("pcs-sparse-bitbanks", ["-p", "flock-core", "--lib", "sparse_bitbanks"]),
                ("pcs-constant-bitbanks", ["-p", "flock-core", "--lib", "constant_bitbanks"]),
                ("pcs-jit-fused", ["-p", "flock-core", "--lib", "fused_jit_"]),
                ("pcs-configs", ["-p", "flock-core", "--lib", "ligerito_all_embedded_configs"]),
                ("pcs-large-caps", ["-p", "flock-core", "--lib", "ligerito_extended_fast"]),
            ]
            for name, selection in test_targets:
                tail = ["--include-ignored", "--test-threads=1"] if name == "pcs-tensor" else ["--test-threads=1"]
                run("test-" + name, cargo + ["test", *flags, *selection, "--", *tail], 3600)
                output = (logs / f"test-{name}.stdout.jsonl").read_text()
                require(sum(map(int, re.findall(r"test result: ok\. (\d+) passed;", output))) > 0,
                        f"test-{name} matched no passing tests")
        for case in planned:
            event = {**case, "state": "preflight"}
            status["cases"].append(event)
            flush()
            memory_gib = max(8, args.minimum_large_memory_gib * 2 ** (case["log_gates"] - 30))
            if args.allocator == "glibc-reuse":
                memory_gib = max(memory_gib, 120 * 2 ** (case["log_gates"] - 26))
            resources = preflight(root, memory_gib)
            save(logs / f"{case['id']}.resources-before.json", resources)
            require(resources["selected_cpu_busy_fraction"] < 0.10, "selected CPUs busy before case")
            require(resources["memory_kib"]["MemAvailable"] >= memory_gib * (1 << 20), "insufficient free memory for case")
            require(resources["disk_free_bytes"] >= 1 << 30, "less than 1 GiB free for retained evidence")
            verify_sources(root)
            require(sha256(binary) == receipt["binary_sha256"], "binary changed between serial cases")
            event["state"] = "running"
            command = [str(binary), "--mode", case["mode"], "--system", case["system"],
                       "--log-gates", str(case["log_gates"]), "--threads", "64", "--repetitions", str(args.repetitions), "--seed", str(args.seed)]
            try:
                ran = run(case["id"], command, args.case_timeout)
                validate_case(logs, case, args.repetitions, args.seed)
                event.update(state="complete", wall_seconds=ran["wall_seconds"], exit_code=0)
            except BaseException as error:
                event.update(state="failed", error=str(error))
                raise
            finally:
                save(logs / f"{case['id']}.resources-after.json", resource_values(root))
                flush()
            summary = aggregate(logs)
            for comparison in summary["comparisons"]:
                require(comparison["state"] != "invalid", "same-circuit commitment/PCS comparison failed")
        _, final_hash = verify_sources(root)
        require(final_hash == manifest_hash and sha256(binary) == receipt["binary_sha256"], "source/binary evidence drift")
        status.update(state="complete", source_hashes_verified_before_and_after=True,
                      binary_hashes_verified_before_and_after=True)
    except BaseException as error:
        for event in status["cases"]:
            if event["state"] in ("preflight", "running"):
                event.update(state="failed", error=str(error))
        status.update(state="failed", error=str(error))
        raise
    finally:
        status["finished_utc"] = utc()
        flush()
        aggregate(logs)
        evidence_hashes(logs)


if __name__ == "__main__":
    main()
