//! Frozen reuse of `mikros/src/flock_baseline.rs` (production functions only),
//! so the baseline does not compile the actively developed Mikros prover.
//! Independent input-opening checks for Flock's packed binary zerocheck.
//!
//! The zerocheck verifier checks the reduction transcript and returns claims;
//! it does not itself authenticate those claims against the input columns.
//! This module is an untimed correctness oracle, not a polynomial commitment.

use flock_core::zerocheck::multilinear::lagrange_weights_naive;
use flock_core::zerocheck::{K_SKIP, ZerocheckClaim};
use flock_field::F128;

/// Check Flock's three hybrid univariate/multilinear evaluation claims against
/// the original LSB-first packed columns. A and B share the sumcheck point;
/// C uses the distinct initial equality point returned in `r_rest`.
pub fn check_claims(
    a: &[u8],
    b: &[u8],
    c: &[u8],
    log_n: usize,
    claims: &ZerocheckClaim,
) -> Result<(), String> {
    if log_n < K_SKIP + 7 || log_n >= usize::BITS as usize {
        return Err(format!(
            "unsupported Flock dimension {log_n}: expected 13..{}",
            usize::BITS - 1
        ));
    }
    let expected_bytes = (1usize << log_n) / 8;
    if [a.len(), b.len(), c.len()] != [expected_bytes; 3] {
        return Err(format!(
            "packed columns must each contain {expected_bytes} bytes"
        ));
    }
    let rest = log_n - K_SKIP;
    if claims.mlv_challenges.len() != rest || claims.r_rest.len() != rest {
        return Err(format!(
            "Flock claim points must each contain {rest} rest coordinates"
        ));
    }

    // Flock's public mathematical Lagrange routine specifies the skip domain.
    // Everything after it is a direct input-bit sum, independent of the
    // optimized fold table, NTT, sumcheck, and proof construction code.
    let skip = lagrange_weights_naive(K_SKIP, claims.z);
    let eq_ab = direct_equality_weights(&claims.mlv_challenges);
    let eq_c = direct_equality_weights(&claims.r_rest);
    let mut actual = [F128::ZERO; 3];
    let bytes_per_row = (1usize << K_SKIP) / 8;
    for row in 0..eq_ab.len() {
        let start = row * bytes_per_row;
        let mut folded = [F128::ZERO; 3];
        for byte in 0..bytes_per_row {
            let packed = [a[start + byte], b[start + byte], c[start + byte]];
            for bit in 0..8 {
                let weight = skip[8 * byte + bit];
                for column in 0..3 {
                    if packed[column] & (1 << bit) != 0 {
                        folded[column] += weight;
                    }
                }
            }
        }
        actual[0] += folded[0] * eq_ab[row];
        actual[1] += folded[1] * eq_ab[row];
        actual[2] += folded[2] * eq_c[row];
    }
    let expected = [claims.a_eval, claims.b_eval, claims.c_eval];
    for (i, column) in ["a", "b", "c"].iter().enumerate() {
        if actual[i] != expected[i] {
            return Err(format!(
                "Flock {column} claim disagrees with the original input"
            ));
        }
    }
    Ok(())
}

// Coordinate i corresponds to bit i of the input row index. Build this here
// instead of relying on Flock's equality-table or fold implementation.
fn direct_equality_weights(point: &[F128]) -> Vec<F128> {
    let mut weights = vec![F128::ONE];
    for &r in point {
        let old_len = weights.len();
        weights.resize(old_len * 2, F128::ZERO);
        for i in 0..old_len {
            let old = weights[i];
            weights[i] = old * (F128::ONE + r);
            weights[old_len + i] = old * r;
        }
    }
    weights
}
