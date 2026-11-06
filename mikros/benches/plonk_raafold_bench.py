#!/usr/bin/env python3
"""Complete binary Mikros PLONK proofs using the RAAFold PCS."""
import sys
import packed_plonk_bench as common
from packed_plonk_bench import (
    Backend, ROOT, PROVER_PHASES, VERIFIER_PHASES, GRINDING_PHASES, FEATURES,
    build_summary, expect, packing_geometry, parse_resource, validate_grinding,
    worker_environment, os, subprocess, run_streamed,
)

BACKEND = "RAAFold RAA-4 equality"
SPEC = Backend("plonk-raafold", BACKEND, "RAAFold", "plonk_raafold", 30,
               "complete opening and Decide verification", "all PIOP, opening and Decide checks")


def parse_options(argv=None):
    return common.parse_options(SPEC, argv)


def parse_worker(text, options, peak_rss_kib):
    return common.parse_worker(text, options, peak_rss_kib, SPEC)


def render_report(doc):
    return common.render_report(doc, SPEC)


def run(options):
    return common.run(options, SPEC, driver_path=__file__, runner=run_streamed)


if __name__ == "__main__":
    try:
        run(parse_options())
    except (ValueError, OSError, KeyError, TypeError) as error:
        print(f"PLONK RAAFold benchmark failed: {error}", file=sys.stderr)
        sys.exit(1)
