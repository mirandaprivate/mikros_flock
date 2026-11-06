#!/usr/bin/env python3
"""Single SHA-256/Keccak/BLAKE3: Mikros FRI versus native FLOCK skip-7."""
import subprocess
import sys

import paper_bench as common


def arguments(argv):
    args = list(argv)
    # Cargo supplies --experiment single and --worker-bin. Defaults are added
    # only when absent, so explicit sample/thread/output overrides still work.
    for name, value in (("--experiment", "single"), ("--pcs", "fri"),
                        ("--threads", "64"), ("--reps", "10"), ("--instances", "1"),
                        ("--output", "target/paper-results/single-hash-fri-vs-flock.log")):
        if not any(arg == name or arg.startswith(name + "=") for arg in args):
            args.extend((name, value))
    options = common.parse_options(args)
    common.require(options.experiment == "single", "single_hash_fri runs the three single-instance hashes")
    return args


def main(argv=None):
    common.main(arguments(sys.argv[1:] if argv is None else argv))


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        print(f"Single-instance FRI comparison failed: {error}", file=sys.stderr)
        sys.exit(1)
