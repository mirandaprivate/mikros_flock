#!/usr/bin/env python3
"""Complete packed binary PLONK proofs with an additive-FRI PCS."""
import sys
import packed_plonk_bench as common
from packed_plonk_bench import (
    Backend, ROOT, PROVER_PHASES, VERIFIER_PHASES, GRINDING_PHASES, FEATURES,
    build_summary, expect, packing_geometry, parse_resource, validate_grinding,
    worker_environment, os, subprocess, run_streamed,
)

BACKEND = "Additive FRI F128 equality"
SPEC = Backend("plonk-fri", BACKEND, "additive FRI", "plonk_fri", 26,
               "complete packed-MLE opening and additive-FRI verification",
               "all PIOP, packed-MLE opening and additive-FRI checks", require_complete=True,
               required_profile=(("rate", 0.5), ("log_inv_rate", 1), ("query_security_bits", 100),
                                 ("native_queries", 241), ("field_bits", 128), ("packed_word_bits", 128),
                                 ("zero_knowledge", False), ("protocol", "BaseFold MLE-check + additive FRI"),
                                 ("point_order", "low-to-high Boolean MLE"),
                                 ("binius64_revision", "a41ff589ad23ed6e158e997c0ba85ede777fdce8"),
                                 ("query_security_scope", "FRI query phase only; excludes folding and other protocol errors")),
               notes=("FRI rate 1/2; 241 queries; 100-bit query-phase budget, excluding folding and other protocol errors. Non-ZK.",))


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
        print(f"PLONK additive FRI benchmark failed: {error}", file=sys.stderr)
        sys.exit(1)
