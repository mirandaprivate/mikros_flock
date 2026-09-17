//! Independent input-opening checks for Flock's packed binary zerocheck.
//!
//! Evaluates the zerocheck verifier's terminal claims directly on the input
//! columns. Benchmarks report this linear-time source check separately.

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
    check_claims_with_skip(a, b, c, log_n, K_SKIP, claims)
}

/// The same independent oracle with an explicitly selected skip parameter.
/// The caller selects 6 or 7; the proof never chooses its own parameter.
pub fn check_claims_with_skip(
    a: &[u8],
    b: &[u8],
    c: &[u8],
    log_n: usize,
    k_skip: usize,
    claims: &ZerocheckClaim,
) -> Result<(), String> {
    if !matches!(k_skip, 6 | 7) {
        return Err(format!("unsupported Flock skip parameter {k_skip}"));
    }
    if log_n < k_skip + 7 || log_n >= usize::BITS as usize {
        return Err(format!(
            "unsupported Flock dimension {log_n}: expected {}..{}",
            k_skip + 7,
            usize::BITS - 1
        ));
    }
    let expected_bytes = (1usize << log_n) / 8;
    if [a.len(), b.len(), c.len()] != [expected_bytes; 3] {
        return Err(format!(
            "packed columns must each contain {expected_bytes} bytes"
        ));
    }
    let rest = log_n - k_skip;
    if claims.mlv_challenges.len() != rest || claims.r_rest.len() != rest {
        return Err(format!(
            "Flock claim points must each contain {rest} rest coordinates"
        ));
    }

    // Flock's public mathematical Lagrange routine specifies the skip domain.
    // Everything after it is a direct input-bit sum, independent of the
    // optimized fold table, NTT, sumcheck, and proof construction code.
    let skip = lagrange_weights_naive(k_skip, claims.z);
    let eq_ab = direct_equality_weights(&claims.mlv_challenges);
    let eq_c = direct_equality_weights(&claims.r_rest);
    let mut actual = [F128::ZERO; 3];
    let bytes_per_row = (1usize << k_skip) / 8;
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

#[cfg(test)]
mod tests {
    use super::{check_claims, check_claims_with_skip, direct_equality_weights};
    use flock_core::zerocheck::{prove_packed, prove_packed_skip7, verify, verify_skip7};
    use flock_field::{F128, test_rng::Rng};
    use flock_transcript::challenger::FsChallenger;

    #[test]
    fn equality_weights_use_low_to_high_coordinates() {
        let mut rng = Rng::new(0xF10C_001);
        let point = rng.f128_vec(5);
        let table = direct_equality_weights(&point);
        for (index, actual) in table.iter().enumerate() {
            let expected = point
                .iter()
                .enumerate()
                .fold(F128::ONE, |product, (i, &r)| {
                    product
                        * if index & (1 << i) == 0 {
                            F128::ONE + r
                        } else {
                            r
                        }
                });
            assert_eq!(*actual, expected, "index {index}");
        }
    }

    #[test]
    fn honest_claims_match_original_inputs_and_changed_inputs_fail() {
        let log_n = 13;
        let mut rng = Rng::new(0xF10C_002);
        let mut a = vec![0; (1 << log_n) / 8];
        let mut b = a.clone();
        rng.fill_bytes(&mut a);
        rng.fill_bytes(&mut b);
        let mut c: Vec<u8> = a.iter().zip(&b).map(|(x, y)| x & y).collect();
        let domain = b"mikros-flock-opening-oracle-test-v1";
        let (proof, prover_claims) =
            prove_packed(&a, &b, &c, log_n, &mut FsChallenger::new(domain));
        let verifier_claims = verify(log_n, &proof, &mut FsChallenger::new(domain)).unwrap();
        assert_eq!(prover_claims, verifier_claims);
        check_claims(&a, &b, &c, log_n, &verifier_claims).unwrap();

        for bit in [0, 1, 63, 64, 127, (1 << log_n) - 1] {
            a[bit / 8] ^= 1 << (bit % 8);
            assert!(check_claims(&a, &b, &c, log_n, &verifier_claims).is_err());
            a[bit / 8] ^= 1 << (bit % 8);
        }
        c[0] ^= 1;
        assert!(check_claims(&a, &b, &c, log_n, &verifier_claims).is_err());
        c[0] ^= 1;
        b[8] ^= 1;
        assert!(check_claims(&a, &b, &c, log_n, &verifier_claims).is_err());
        b[8] ^= 1;

        // A product-preserving forgery survives the reduction's final check.
        // Authentication against the actual inputs must still reject it.
        let mut forged = proof.clone();
        let factor = F128::generator();
        forged.final_a_eval *= factor;
        forged.final_b_eval *= factor.inv();
        let forged_claims = verify(log_n, &forged, &mut FsChallenger::new(domain)).unwrap();
        assert!(check_claims(&a, &b, &c, log_n, &forged_claims).is_err());

        let mut changed_claims = verifier_claims.clone();
        changed_claims.c_eval += F128::ONE;
        assert!(check_claims(&a, &b, &c, log_n, &changed_claims).is_err());
        changed_claims = verifier_claims;
        changed_claims.mlv_challenges.pop();
        assert!(check_claims(&a, &b, &c, log_n, &changed_claims).is_err());
    }

    #[test]
    fn skip7_claims_check_all_three_inputs_and_both_row_halves() {
        let log_n = 14;
        let mut rng = Rng::new(0xF10C_007);
        let mut columns = vec![vec![0; (1 << log_n) / 8]; 3];
        rng.fill_bytes(&mut columns[0]);
        rng.fill_bytes(&mut columns[1]);
        columns[2] = columns[0]
            .iter()
            .zip(&columns[1])
            .map(|(a, b)| a & b)
            .collect();
        let domain = b"mikros-flock128-opening-oracle-v1";
        let (proof, claims) = prove_packed_skip7(
            &columns[0],
            &columns[1],
            &columns[2],
            log_n,
            &mut FsChallenger::new(domain),
        );
        let checked = verify_skip7(log_n, &proof, &mut FsChallenger::new(domain)).unwrap();
        assert_eq!(claims, checked);
        let check = |cols: &[Vec<u8>], claim: &flock_core::zerocheck::ZerocheckClaim| {
            check_claims_with_skip(&cols[0], &cols[1], &cols[2], log_n, 7, claim)
        };
        check(&columns, &checked).unwrap();
        for column in 0..3 {
            for bit in [0, 63, 64, 127, 128, (1 << log_n) - 1] {
                columns[column][bit / 8] ^= 1 << (bit % 8);
                assert!(
                    check(&columns, &checked).is_err(),
                    "column={column}, bit={bit}"
                );
                columns[column][bit / 8] ^= 1 << (bit % 8);
            }
        }
        assert!(check_claims(&columns[0], &columns[1], &columns[2], log_n, &checked).is_err());
        assert!(
            check_claims_with_skip(&columns[0], &columns[1], &columns[2], log_n, 8, &checked)
                .is_err()
        );
        let mut forged = proof.clone();
        let factor = F128::generator();
        forged.final_a_eval *= factor;
        forged.final_b_eval *= factor.inv();
        let forged_claims = verify_skip7(log_n, &forged, &mut FsChallenger::new(domain)).unwrap();
        assert!(check(&columns, &forged_claims).is_err());
    }
}
