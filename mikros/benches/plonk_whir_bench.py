#!/usr/bin/env python3
"""Complete packed binary PLONK proofs with an additive-domain WHIR PCS."""
import sys
import packed_plonk_bench as common
from packed_plonk_bench import Backend, run_streamed

BACKEND = "WHIR F128 equality"
SPEC = Backend("plonk-whir", BACKEND, "WHIR", "plonk_whir", 26,
               "complete packed-MLE opening and WHIR verification",
               "all PIOP, packed-MLE opening and WHIR checks", require_complete=True,
               required_profile=(("field_bits", 128), ("code_field_bits", 128),
                                 ("packed_word_bits", 128), ("zero_knowledge", False),
                                 ("upstream_version", "0.8.0"), ("rate", 0.5),
                                 ("decoding", "unique"), ("hash", "SHA-256"),
                                 ("security_target_per_term_bits", 100),
                                 ("maximum_allowed_pow_bits", 3),
                                 ("protocol", "Plonky3 binary WHIR prescribed Boolean MLE")),
               notes=("WHIR uses full F128 words, the official GHASH/tower isomorphism, additive-domain RS and compact MLE queries. Non-ZK.",
                      "Rate 1/2; target 100 bits per term; smallest accepted PoW budget up to 3 bits. Actual composed PCS bounds and grinding are recorded per vector length.",))


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
        print(f"PLONK WHIR benchmark failed: {error}", file=sys.stderr)
        sys.exit(1)
