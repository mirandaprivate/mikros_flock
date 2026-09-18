//! Public copy wiring: P[i, sigma(i)] = 1 and (I + P) v = 0.
//! One degree-two IP reduces this to v(s) and (I + P)~(r,s).
//! The verifier evaluates the sparse public matrix itself; the three private
//! wire components of v(s) remain authenticated claims to the witness PCS.

use super::{
    fixture::{self, Circuit},
    gates::Claims,
    queries::*,
};
use flock_core::{challenger::Challenger, pcs::functional::TensorClaim};
use flock_field::{F128, F256Unreduced};
use mikros::{InnerProductProof, inner_product};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SingleProof {
    pub ip: InnerProductProof,
    /// Each component includes its high two-bit wire-column prefix.
    pub wire_values: [F128; 3],
}

/// Fixed public preprocessing, including bijectivity and padding checks.
pub fn validate_circuit(c: &Circuit) -> Result<(), String> {
    if !(7..=30).contains(&c.log_gates) || c.io.len() > c.n() || c.sigma.len() != c.copy_len() {
        return Err("public wiring circuit shape".into());
    }
    let mut seen = vec![false; c.copy_len()];
    for (i, &j) in c.sigma.iter().enumerate() {
        if j >= c.copy_len()
            || seen[j]
            || (i < c.live_len() && j >= c.live_len())
            || (i >= c.live_len() && i != j)
        {
            return Err("public wiring invalid permutation or padding".into());
        }
        seen[j] = true;
    }
    Ok(())
}

fn challenge<C: Challenger>(c: &Circuit, ch: &mut C) -> Vec<F128> {
    // Caller binds the public circuit digest and witness commitment first.
    ch.observe_label(b"mikros-single-public-wiring-ip-v1");
    ch.observe_bytes(&(c.h() as u64).to_le_bytes());
    ch.sample_f128_vec(c.h())
}

/// Sparse column projection of I+P. Fixed points cancel in characteristic two.
fn weights(c: &Circuit, r: &[F128]) -> Vec<F128> {
    let eq = tensor(r);
    let mut out = vec![F128::ZERO; c.copy_len()];
    for (i, &j) in c.sigma.iter().enumerate() {
        if i != j {
            out[i] += eq[i];
            out[j] += eq[i];
        }
    }
    out
}

/// Direct sparse MLE of I+P, with no prover-provided matrix evaluation.
fn matrix_mle(c: &Circuit, r: &[F128], s: &[F128]) -> F128 {
    let left = tensor(r);
    let right = tensor(s);
    let mut acc = F256Unreduced::ZERO;
    #[allow(unused_mut)]
    let mut i = 0;
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512f",
        target_feature = "vpclmulqdq",
        target_feature = "sse4.1"
    ))]
    // SAFETY: ISA requirements are compile-time gated; the verifier checks
    // all sigma indices before this call, and every vector load is in bounds.
    unsafe {
        use flock_field::gf2_128::x86_64::{WideGhashX4, f128x4_loadu};
        let mut wide = WideGhashX4::zero();
        while i + 4 <= c.sigma.len() {
            if (0..4).any(|lane| c.sigma[i + lane] != i + lane) {
                let values: [F128; 4] =
                    std::array::from_fn(|lane| right[i + lane] + right[c.sigma[i + lane]]);
                wide.mul_acc(
                    f128x4_loadu(left.as_ptr().add(i)),
                    f128x4_loadu(values.as_ptr()),
                );
            }
            i += 4;
        }
        acc ^= wide.fold();
    }
    for (i, &j) in c.sigma.iter().enumerate().skip(i) {
        if i != j {
            acc ^= left[i].mul_unreduced(right[i] + right[j]);
        }
    }
    acc.reduce()
}

fn wire_queries(c: &Circuit, low_point: &[F128]) -> [TensorClaim; 3] {
    let n = c.log_gates;
    let high = tensor(&low_point[n..]);
    std::array::from_fn(|wire| {
        lift(
            TensorClaim {
                columns: tensor(&low_point[..7])
                    .into_iter()
                    .map(|x| high[wire] * x)
                    .collect(),
                rows: pairs(&low_point[7..n]),
                value: F128::ZERO,
            },
            wire,
            c.witness_words().ilog2() as usize,
        )
    })
}

fn public_value(c: &Circuit, s: &[F128]) -> F128 {
    let n = c.log_gates;
    // I/O occupies a short prefix of the fourth block. Factor its fixed zero
    // high coordinates instead of materializing all N row weights.
    let active = c.io.len().max(1).next_power_of_two().ilog2() as usize;
    let low = tensor(&s[..active]);
    let prefix = s[active..n]
        .iter()
        .fold(s[n] * s[n + 1], |v, &x| v * (F128::ONE + x));
    prefix
        * c.io
            .iter()
            .zip(low)
            .filter(|(bit, _)| **bit)
            .fold(F128::ZERO, |sum, (_, weight)| sum + weight)
}

fn finish<C: Challenger>(
    c: &Circuit,
    p: &SingleProof,
    high_point: &[F128],
    ch: &mut C,
) -> Result<Claims, String> {
    let low_point: Vec<_> = high_point.iter().rev().copied().collect();
    ch.observe_f128_slice(&p.wire_values);
    if p.wire_values
        .iter()
        .copied()
        .fold(public_value(c, &low_point), |a, b| a + b)
        != p.ip.final_a
    {
        return Err("public wiring witness/public terminal mismatch".into());
    }
    let mut queries = wire_queries(c, &low_point);
    for (query, value) in queries.iter_mut().zip(p.wire_values) {
        query.value = value;
    }
    Ok(Claims {
        witness: queries.into(),
        public: vec![],
    })
}

pub fn single_prove<C: Challenger>(
    c: &Circuit,
    z: &[F128],
    ch: &mut C,
) -> Result<(SingleProof, Claims), String> {
    if z.len() != c.witness_words() {
        return Err("public wiring witness shape".into());
    }
    // The setup has already checked the public permutation. No private input
    // or challenge-dependent vector is cached in that preprocessing.
    let r = challenge(c, ch);
    let beta = weights(c, &r);
    let values: Vec<_> = (0..c.copy_len())
        .map(|i| {
            let bit = if i < 3 * c.n() {
                fixture::bit(z, i)
            } else {
                c.io.get(i - 3 * c.n()).copied().unwrap_or(false)
            };
            if bit { F128::ONE } else { F128::ZERO }
        })
        .collect();
    let total = values
        .iter()
        .zip(&beta)
        .filter(|(v, _)| **v == F128::ONE)
        .fold(F128::ZERO, |sum, (_, &b)| sum + b);
    if total != F128::ZERO {
        return Err("public wiring assignment violates copy constraints".into());
    }
    let (ip, point) = inner_product::prove_native_owned(values, beta, ch);
    let low: Vec<_> = point.iter().rev().copied().collect();
    let m = c.wire_words();
    let queries = wire_queries(c, &low);
    let wire_values = std::array::from_fn(|wire| {
        let mut q = queries[wire].clone();
        // The high-bit restrictions select this exact source block. Keep
        // their external prefix on columns, avoiding a scan of padded PCS data.
        q.rows.truncate(c.log_gates - 7);
        evaluate(&z[wire * m..(wire + 1) * m], &q)
    });
    let proof = SingleProof { ip, wire_values };
    let claims = finish(c, &proof, &point, ch)?;
    Ok((proof, claims))
}

pub fn single_verify<C: Challenger>(
    c: &Circuit,
    p: &SingleProof,
    ch: &mut C,
) -> Result<Claims, String> {
    if p.ip.rounds.len() != c.h()
        || c.sigma.len() != c.copy_len()
        || c.sigma.iter().any(|&j| j >= c.copy_len())
    {
        return Err("public wiring proof/matrix shape".into());
    }
    let r = challenge(c, ch);
    let point = inner_product::verify(&p.ip, F128::ZERO, ch)
        .map_err(|e| format!("public wiring IP: {e:?}"))?;
    let low: Vec<_> = point.iter().rev().copied().collect();
    if p.ip.final_b != matrix_mle(c, &r, &low) {
        return Err("public wiring sparse matrix MLE mismatch".into());
    }
    finish(c, p, &point, ch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::challenger::FsChallenger;
    use flock_field::test_rng::Rng;

    #[test]
    fn public_wiring_sparse_mle_matches_dense_oracle() {
        let (c, _) = fixture::tiny_fixture();
        validate_circuit(&c).unwrap();
        let mut rng = Rng::new(871);
        for boolean in [false, true] {
            let r: Vec<_> = (0..c.h())
                .map(|i| {
                    if boolean {
                        if i % 2 == 0 { F128::ONE } else { F128::ZERO }
                    } else {
                        rng.f128()
                    }
                })
                .collect();
            let s: Vec<_> = (0..c.h()).map(|_| rng.f128()).collect();
            let eq_r = tensor(&r);
            let eq_s = tensor(&s);
            let dense = (0..c.copy_len()).fold(F128::ZERO, |a, i| {
                a + (0..c.copy_len())
                    .filter(|&j| (j == i) ^ (j == c.sigma[i]))
                    .fold(F128::ZERO, |v, j| v + eq_r[i] * eq_s[j])
            });
            assert_eq!(matrix_mle(&c, &r, &s), dense);
            assert_eq!(
                weights(&c, &r)
                    .iter()
                    .zip(eq_s)
                    .fold(F128::ZERO, |a, (&b, v)| a + b * v),
                dense
            );
        }
    }

    #[test]
    fn public_wiring_roundtrip_and_tampering() {
        let (c, z) = fixture::tiny_fixture();
        validate_circuit(&c).unwrap();
        let mut pc = FsChallenger::new(b"public-wiring-test");
        let (proof, claims) = single_prove(&c, &z, &mut pc).unwrap();
        let mut vc = FsChallenger::new(b"public-wiring-test");
        let checked = single_verify(&c, &proof, &mut vc).unwrap();
        assert!(claims.public.is_empty() && checked.public.is_empty());
        assert_eq!(claims.witness.len(), 3);
        assert!(checked.witness.iter().all(|q| evaluate(&z, q) == q.value));
        assert_eq!(pc.sample_f128(), vc.sample_f128());
        // A balanced change survives the linear terminal identity, but must
        // remain a false PCS claim (and is rejected by the full E2E verifier).
        let mut balanced = proof.clone();
        balanced.wire_values[0] += F128::ONE;
        balanced.wire_values[1] += F128::ONE;
        let deferred =
            single_verify(&c, &balanced, &mut FsChallenger::new(b"public-wiring-test")).unwrap();
        assert!(deferred.witness.iter().any(|q| evaluate(&z, q) != q.value));
        for mutation in 0..5 {
            let mut bad = proof.clone();
            match mutation {
                0 => bad.ip.rounds[0][0] += F128::ONE,
                1 => bad.ip.final_a += F128::ONE,
                2 => bad.ip.final_b += F128::ONE,
                3 => bad.wire_values[0] += F128::ONE,
                _ => {
                    bad.ip.rounds.pop();
                }
            }
            assert!(
                single_verify(&c, &bad, &mut FsChallenger::new(b"public-wiring-test")).is_err()
            );
        }
        let mut bad = c.clone();
        bad.sigma.swap(0, 1);
        assert!(
            single_verify(&bad, &proof, &mut FsChallenger::new(b"public-wiring-test")).is_err()
        );
        let mut bad = c.clone();
        bad.io[0] ^= true;
        assert!(
            single_verify(&bad, &proof, &mut FsChallenger::new(b"public-wiring-test")).is_err()
        );
        let mut bad_z = z.clone();
        bad_z[0] = F128::new(bad_z[0].lo ^ 1, bad_z[0].hi);
        assert!(single_prove(&c, &bad_z, &mut FsChallenger::new(b"public-wiring-test")).is_err());
    }
}
