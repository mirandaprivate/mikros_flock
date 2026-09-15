"""Correctness smoke only: four actual E2E proofs, not server timing evidence."""
import argparse
import json
import subprocess
from pathlib import Path

root = Path(__file__).resolve().parents[3]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--output", default="local-smoke")
args = parser.parse_args()
base = Path(__file__).resolve().parent
out = (base / args.output).resolve()
assert out.is_relative_to(base) and out != base
out.mkdir(exist_ok=False)
results = {}
for mode in ("single", "multiple"):
    for system in ("mikros", "flock"):
        stem = f"{mode}-{system}"
        command = [str(root / "target/release/examples/e2e_plonk_compare"),
                   "--mode", mode, "--system", system, "--log-gates", "14",
                   "--threads", "4", "--repetitions", "1"]
        run = subprocess.run(command, cwd=root, capture_output=True, text=True)
        (out / (stem + ".jsonl")).write_text(run.stdout)
        (out / (stem + ".stderr.log")).write_text(run.stderr)
        if run.returncode:
            raise RuntimeError(f"{stem} exit {run.returncode}: {run.stderr[-2500:]}")
        samples = [json.loads(x) for x in run.stdout.splitlines() if json.loads(x).get("type") == "sample"]
        assert len(samples) == 1
        sample = samples[0]
        assert sample["verified"] and sample["transcript_match"] and sample["pcs_config_match"]
        results[stem] = sample
        print(json.dumps({"case": stem, "verified": True, "proof_bytes": sample["proof_bytes"]}), flush=True)
    for key in ("witness_pcs_params", "public_pcs_params", "witness_commitment_blake3", "public_commitment_blake3"):
        assert results[mode + "-mikros"][key] == results[mode + "-flock"][key], (mode, key)
(out / "summary.json").write_text(json.dumps({"scope": "local correctness only, 4 threads, N14", "all_four_verified": True, "same_commitments_and_pcs": True}, indent=2) + "\n")
