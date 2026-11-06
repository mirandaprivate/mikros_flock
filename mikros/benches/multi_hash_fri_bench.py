#!/usr/bin/env python3
"""Table 4: 256 independent hashes, Mikros FRI versus native FLOCK skip-7."""
import subprocess
import sys

import paper_bench as common


def arguments(argv):
    args = list(argv)
    for name, value in (("--experiment", "multiple"), ("--pcs", "fri"),
                        ("--threads", "1"), ("--reps", "10"), ("--instances", "256")):
        if not any(arg == name or arg.startswith(name + "=") for arg in args):
            args.extend((name, value))
    options = common.parse_options(args)
    common.require(options.experiment == "multiple" and options.pcs == "fri",
                   "multi_hash_fri_compare runs the three 256-instance hashes with Mikros FRI")
    return args


def main(argv=None):
    common.main(arguments(sys.argv[1:] if argv is None else argv))


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        print(f"Multiple-instance FRI comparison failed: {error}", file=sys.stderr)
        sys.exit(1)
