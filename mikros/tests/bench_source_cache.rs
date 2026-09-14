//! The benchmark's optimization must retain the original source-check contract.

#[path = "../benches/support/source_checks.rs"]
mod source_checks;

use flock_field::F128;
use mikros::{Claims, MatrixClaim};
use source_checks::SourceCheckCache;

const LOG: usize = 8;
const BYTES: usize = 1 << (LOG - 3);

fn claims(value: F128) -> Claims {
    let mut column_weights = vec![F128::ZERO; 128];
    column_weights[0] = F128::ONE;
    let query = MatrixClaim {
        column_weights,
        row_factors: vec![[F128::ONE, F128::ZERO]],
        value,
    };
    Claims {
        a: query.clone(),
        b: query.clone(),
        c: query,
    }
}

fn select(claims: &mut Claims, side: usize) -> &mut MatrixClaim {
    match side {
        0 => &mut claims.a,
        1 => &mut claims.b,
        2 => &mut claims.c,
        _ => unreachable!(),
    }
}

#[test]
fn first_success_uses_full_oracle_and_exact_repeat_names_its_anchor() {
    let (a, b, c) = (vec![0; BYTES], vec![0; BYTES], vec![0; BYTES]);
    let checked = claims(F128::ZERO);
    let mut cache = SourceCheckCache::new(&a, &b, &c, LOG);
    let first = cache
        .check(&a, &b, &c, LOG, &checked, "pair", true)
        .unwrap();
    assert_eq!(first.method, "full_oracle");
    assert_eq!(first.anchor, "pair");
    let repeat = cache
        .check(&a, &b, &c, LOG, &checked.clone(), "carryless", true)
        .unwrap();
    assert_eq!(repeat.method, "exact_claim_cache");
    assert_eq!(repeat.anchor, "pair");
}

#[test]
fn malicious_first_claims_never_establish_a_cache_entry() {
    let (a, b, c) = (vec![0; BYTES], vec![0; BYTES], vec![0; BYTES]);
    let good = claims(F128::ZERO);
    for side in 0..3 {
        let mut cache = SourceCheckCache::new(&a, &b, &c, LOG);
        let mut bad = good.clone();
        select(&mut bad, side).value = F128::ONE;
        for _ in 0..2 {
            assert!(
                cache
                    .check(&a, &b, &c, LOG, &bad, "malicious", true)
                    .is_err()
            );
        }
        let record = cache.check(&a, &b, &c, LOG, &good, "valid", true).unwrap();
        assert_eq!(record.method, "full_oracle");
        assert_eq!(record.anchor, "valid");
    }
}

#[test]
fn cache_compares_each_terminal_value_and_every_query_field() {
    let (a, b, c) = (vec![0; BYTES], vec![0; BYTES], vec![0; BYTES]);
    let good = claims(F128::ZERO);
    let mut cache = SourceCheckCache::new(&a, &b, &c, LOG);
    cache.check(&a, &b, &c, LOG, &good, "anchor", true).unwrap();
    for side in 0..3 {
        for part in 0..5 {
            let mut changed = good.clone();
            let query = select(&mut changed, side);
            match part {
                0 => query.value += F128::ONE,
                1 => query.column_weights[127] += F128::ONE,
                2 => query.row_factors[0][1] += F128::ONE,
                3 => {
                    query.column_weights.pop();
                }
                4 => {
                    query.row_factors.push([F128::ONE, F128::ZERO]);
                }
                _ => unreachable!(),
            }
            if part == 1 || part == 2 {
                // These changed queries still evaluate to zero on this input.
                // Rejecting them proves the cache does not compare only values.
                mikros::check_claims(&a, &b, &c, LOG, &changed).unwrap();
            }
            let error = cache
                .check(&a, &b, &c, LOG, &changed, "changed", true)
                .unwrap_err();
            assert!(error.contains("identical full Claims"));
        }
    }
    let unchanged = cache
        .check(&a, &b, &c, LOG, &good, "unchanged", true)
        .unwrap();
    assert_eq!(unchanged.anchor, "anchor");
}

#[test]
fn equal_content_copies_shortened_slices_and_other_dimensions_are_rejected() {
    let (a, b, c) = (vec![0; BYTES], vec![0; BYTES], vec![0; BYTES]);
    let (other_a, other_b, other_c) = (a.clone(), b.clone(), c.clone());
    let good = claims(F128::ZERO);
    let mut cache = SourceCheckCache::new(&a, &b, &c, LOG);
    cache.check(&a, &b, &c, LOG, &good, "anchor", true).unwrap();
    for reuse in [false, true] {
        for (aa, bb, cc, log) in [
            (other_a.as_slice(), b.as_slice(), c.as_slice(), LOG),
            (a.as_slice(), other_b.as_slice(), c.as_slice(), LOG),
            (a.as_slice(), b.as_slice(), other_c.as_slice(), LOG),
            (&a[..BYTES - 1], b.as_slice(), c.as_slice(), LOG),
            (a.as_slice(), &b[..BYTES - 1], c.as_slice(), LOG),
            (a.as_slice(), b.as_slice(), &c[..BYTES - 1], LOG),
            (a.as_slice(), b.as_slice(), c.as_slice(), LOG + 1),
        ] {
            let error = cache
                .check(aa, bb, cc, log, &good, "other-input", reuse)
                .unwrap_err();
            assert!(error.contains("input identity or dimension changed"));
        }
    }
}

#[test]
fn fresh_scope_cannot_reuse_a_previous_witness_or_previous_success() {
    let (a, b, c) = (vec![0; BYTES], vec![0; BYTES], vec![0; BYTES]);
    let zero = claims(F128::ZERO);
    let mut first = SourceCheckCache::new(&a, &b, &c, LOG);
    first.check(&a, &b, &c, LOG, &zero, "old", true).unwrap();
    let mut same_input_new_run = SourceCheckCache::new(&a, &b, &c, LOG);
    assert_eq!(
        same_input_new_run
            .check(&a, &b, &c, LOG, &zero, "new-run", true)
            .unwrap()
            .method,
        "full_oracle"
    );

    let (a2, b2, c2) = (vec![255; BYTES], vec![255; BYTES], vec![255; BYTES]);
    let mut second = SourceCheckCache::new(&a2, &b2, &c2, LOG);
    assert!(
        second
            .check(&a2, &b2, &c2, LOG, &zero, "wrong-witness", true)
            .is_err()
    );
    let record = second
        .check(&a2, &b2, &c2, LOG, &claims(F128::ONE), "new-witness", true)
        .unwrap();
    assert_eq!(record.method, "full_oracle");
    assert_eq!(record.anchor, "new-witness");
}

#[test]
fn disabled_reuse_always_checks_source_and_accepts_other_valid_queries() {
    let (a, b, c) = (vec![0; BYTES], vec![0; BYTES], vec![0; BYTES]);
    let good = claims(F128::ZERO);
    let mut cache = SourceCheckCache::new(&a, &b, &c, LOG);
    for anchor in ["first-full", "second-full"] {
        let record = cache.check(&a, &b, &c, LOG, &good, anchor, false).unwrap();
        assert_eq!(record.method, "full_oracle");
        assert_eq!(record.anchor, anchor);
    }
    let mut changed_query = good.clone();
    changed_query.a.column_weights[1] = F128::ONE;
    let record = cache
        .check(&a, &b, &c, LOG, &changed_query, "different-full", false)
        .unwrap();
    assert_eq!(record.method, "full_oracle");
    assert_eq!(record.anchor, "different-full");
    assert!(
        cache
            .check(&a, &b, &c, LOG, &good, "old-query", true)
            .is_err()
    );
    let record = cache
        .check(&a, &b, &c, LOG, &changed_query, "same-query", true)
        .unwrap();
    assert_eq!(record.anchor, "different-full");
}

#[test]
fn failed_full_check_does_not_replace_an_existing_verified_anchor() {
    let (a, b, c) = (vec![0; BYTES], vec![0; BYTES], vec![0; BYTES]);
    let good = claims(F128::ZERO);
    let mut cache = SourceCheckCache::new(&a, &b, &c, LOG);
    cache.check(&a, &b, &c, LOG, &good, "valid", false).unwrap();
    let mut bad = good.clone();
    bad.c.value = F128::ONE;
    assert!(
        cache
            .check(&a, &b, &c, LOG, &bad, "invalid-full", false)
            .is_err()
    );
    let record = cache
        .check(&a, &b, &c, LOG, &good, "reuse-valid", true)
        .unwrap();
    assert_eq!(record.method, "exact_claim_cache");
    assert_eq!(record.anchor, "valid");
}
