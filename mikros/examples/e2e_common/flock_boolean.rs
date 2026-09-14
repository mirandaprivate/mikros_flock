//! Single-SHA-slot Flock Boolean PIOP with skip dimension seven (kappa 128).
//!
//! Uses the existing standalone lincheck's configurable skip dimension. The
//! union lincheck stays unchanged at its production skip64 default. The pin is
//! opened separately because partial-count padding carries zero, whereas the
//! standalone lincheck's built-in constant-wire check assumes all rows are one.
//! Every returned tensor claim must be authenticated against the full padded Z.

use flock_core::{
    challenger::Challenger,
    lincheck::{
        LincheckCircuit, LincheckGrinding, LincheckProof, QuirkyPoint, SkipPoint, eq_prefix_sum,
        prove_padded_with_grinding, verify_with_grinding,
    },
    pcs::functional::TensorClaim,
    zerocheck::{self, ZerocheckClaim, ZerocheckProof},
};
use flock_field::F128;
use serde::{Deserialize, Serialize};

use super::{PreparedWitness, TreeFixture, TreeVerifier};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    pub zerocheck: ZerocheckProof,
    pub lincheck: LincheckProof,
}

/// Only removes the full-capacity pin assumption. All matrix folds use the
/// original CSC implementation. The separately returned const-pin PCS query
/// is mandatory and restores the constant-wire constraint for partial counts.
struct Unpinned<'a>(&'a dyn LincheckCircuit);
impl LincheckCircuit for Unpinned<'_> {
    fn n_cols(&self) -> usize {
        self.0.n_cols()
    }
    fn fold_alpha_batched(&self, alpha: F128, eq_inner: &[F128]) -> Vec<F128> {
        self.0.fold_alpha_batched(alpha, eq_inner)
    }
    fn fold_split(&self, eq_inner: &[F128]) -> (Vec<F128>, Vec<F128>) {
        self.0.fold_split(eq_inner)
    }
    fn const_pin_col(&self) -> Option<usize> {
        None
    }
}

/// Internal Flock grinding is disabled so the caller can use its common
/// challenge adapter. Merely calling this with plain FsChallenger does not
/// apply the strict Fast PIOP grinding schedule.
pub fn prove<C: Challenger>(
    fixture: &TreeFixture,
    prepared: &PreparedWitness,
    ch: &mut C,
) -> Result<(Proof, Vec<TensorClaim>), String> {
    let view = fixture.verifier();
    check_shape(&view)?;
    let m = fixture.r1cs.m;
    let expected = 1usize << (m - 7);
    if prepared.0.len() != expected || prepared.1.len() != expected || prepared.2.len() != expected
    {
        return Err("Boolean PIOP expects full padded z/a/b".into());
    }
    bind_dimension(m, ch);
    let (zerocheck, zc) = zerocheck::prove_packed_skip7(
        packed_bytes(&prepared.1),
        packed_bytes(&prepared.2),
        packed_bytes(&prepared.0),
        m,
        ch,
    );
    let x_ab = semantic_point(&fixture.verifier(), &zc)?;
    let matrix = Unpinned(fixture.r1cs.csc_lincheck_circuit());
    let (lincheck, lc) = prove_padded_with_grinding(
        &prepared.3,
        m,
        fixture.r1cs.k_log,
        7,
        fixture.r1cs.useful_bits,
        &matrix,
        &x_ab,
        LincheckGrinding::disabled(),
        ch,
    );
    let mut claims = terminal_claims(&zc, &x_ab.x_outer, lc.r_inner_skip, &lc.r_inner_rest, lc.w);
    claims.push(common_const_pin_claim(&fixture.verifier(), ch)?);
    Ok((
        Proof {
            zerocheck,
            lincheck,
        },
        claims,
    ))
}

/// Replay and matrix consistency checks use the public fixed circuit only.
/// This verifier has no witness or A/B tables. All THREE claims, including
/// the constant pin, must reach the PCS verifier before acceptance.
pub fn verify<C: Challenger>(
    fixture: &TreeVerifier<'_>,
    proof: &Proof,
    ch: &mut C,
) -> Result<Vec<TensorClaim>, String> {
    check_shape(fixture)?;
    if !proof.lincheck.matrix_evals.is_empty() {
        return Err("standalone lincheck cannot carry deferred matrix claims".into());
    }
    bind_dimension(fixture.r1cs.m, ch);
    let zc = zerocheck::verify_skip7(fixture.r1cs.m, &proof.zerocheck, ch)
        .map_err(|e| format!("Flock skip128 Hadamard rejected: {e:?}"))?;
    let x_ab = semantic_point(fixture, &zc)?;
    let matrix = Unpinned(fixture.r1cs.csc_lincheck_circuit());
    let lc = verify_with_grinding(
        fixture.r1cs.m,
        fixture.r1cs.k_log,
        7,
        &matrix,
        &x_ab,
        zc.a_eval,
        zc.b_eval,
        &proof.lincheck,
        LincheckGrinding::disabled(),
        ch,
    )
    .map_err(|e| format!("Flock skip128 lincheck rejected: {e:?}"))?;
    let mut claims = terminal_claims(&zc, &x_ab.x_outer, lc.r_inner_skip, &lc.r_inner_rest, lc.w);
    claims.push(common_const_pin_claim(fixture, ch)?);
    Ok(claims)
}

/// Shared Flock/Mikros pin query. The expected column is one on live SHA rows
/// and zero on padding. It is derived without reading any witness value.
pub fn common_const_pin_claim<C: Challenger>(
    fixture: &TreeVerifier<'_>,
    ch: &mut C,
) -> Result<TensorClaim, String> {
    check_shape(fixture)?;
    let pin = fixture
        .r1cs
        .const_pin
        .ok_or("SHA fixture must declare its constant pin")?;
    ch.observe_label(b"mikros-flock-e2e-live-constant-pin-v1");
    let row_point = ch.sample_f128_vec(fixture.nu);
    let mut columns = vec![F128::ZERO; 128];
    columns[pin & 127] = F128::ONE;
    let mut rows = eq_pairs(&row_point);
    rows.extend((0..fixture.r1cs.k_log - 7).map(|k| {
        if (pin >> (k + 7)) & 1 != 0 {
            [F128::ZERO, F128::ONE]
        } else {
            [F128::ONE, F128::ZERO]
        }
    }));
    Ok(TensorClaim {
        columns,
        rows,
        value: eq_prefix_sum(&row_point, fixture.live_rows),
    })
}

fn semantic_point(fixture: &TreeVerifier<'_>, zc: &ZerocheckClaim) -> Result<QuirkyPoint, String> {
    if zc.mlv_challenges.len() != fixture.r1cs.m - 7 || zc.r_rest.len() != fixture.r1cs.m - 7 {
        return Err("skip128 zerocheck endpoint dimension mismatch".into());
    }
    Ok(QuirkyPoint {
        z_skip: SkipPoint::Phi8(zc.z),
        x_inner_rest: zc.mlv_challenges[fixture.nu..].to_vec(),
        x_outer: zc.mlv_challenges[..fixture.nu].to_vec(),
    })
}

fn terminal_claims(
    zc: &ZerocheckClaim,
    batch_point: &[F128],
    skip: SkipPoint,
    chunk_point: &[F128],
    value: F128,
) -> Vec<TensorClaim> {
    let mut rows = eq_pairs(batch_point);
    rows.extend(eq_pairs(chunk_point));
    vec![
        TensorClaim {
            columns: skip.weights(7),
            rows,
            value,
        },
        TensorClaim {
            columns: SkipPoint::Phi8(zc.z).weights(7),
            rows: eq_pairs(&zc.r_rest),
            value: zc.c_eval,
        },
    ]
}

fn check_shape(fixture: &TreeVerifier<'_>) -> Result<(), String> {
    if fixture.registry.num_boolean() != 1
        || fixture.r1cs.m != fixture.union().m_total()
        || !fixture.r1cs.c0_is_identity()
        || fixture.r1cs.k_log < 7
    {
        return Err("standalone skip128 wrapper requires one Boolean SHA slot with C=I".into());
    }
    Ok(())
}

fn bind_dimension<C: Challenger>(m: usize, ch: &mut C) {
    ch.observe_label(b"mikros-flock-e2e-single-slot-skip128-v1");
    ch.observe_bytes(&(m as u64).to_le_bytes());
}

fn eq_pairs(point: &[F128]) -> Vec<[F128; 2]> {
    point.iter().map(|&r| [F128::ONE + r, r]).collect()
}

fn packed_bytes(words: &[F128]) -> &[u8] {
    assert!(
        cfg!(target_endian = "little"),
        "packed byte view requires little endian"
    );
    // F128 is repr(C), two u64 limbs. These are raw packed bits, not scalar
    // field embeddings. The shared Flock witness/zerocheck interface uses it.
    unsafe { std::slice::from_raw_parts(words.as_ptr().cast(), std::mem::size_of_val(words)) }
}

#[cfg(test)]
mod tests {
    use super::{prove, verify};
    use crate::e2e_common::TreeFixture;
    use flock_core::{
        challenger::{Challenger, FsChallenger},
        pcs::functional::{TensorClaim, row_weights},
    };
    use flock_field::F128;

    fn eval(witness: &[F128], claim: &TensorClaim) -> F128 {
        witness
            .iter()
            .zip(row_weights(&claim.rows))
            .map(|(&word, row)| {
                let bit_sum: F128 = claim
                    .columns
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &weight)| {
                        let limb = if i < 64 { word.lo } else { word.hi };
                        ((limb >> (i & 63)) & 1 != 0).then_some(weight)
                    })
                    .fold(F128::ZERO, |a, b| a + b);
                row * bit_sum
            })
            .fold(F128::ZERO, |a, b| a + b)
    }

    #[test]
    fn skip128_roundtrip_and_all_pcs_endpoints() {
        let fixture = TreeFixture::new(1);
        let witness = fixture.prepare_witness();
        let mut prover = FsChallenger::new(b"flock-skip128-e2e-test");
        let (proof, claims) = prove(&fixture, &witness, &mut prover).unwrap();
        let mut verifier = FsChallenger::new(b"flock-skip128-e2e-test");
        assert_eq!(
            claims,
            verify(&fixture.verifier(), &proof, &mut verifier).unwrap()
        );
        assert_eq!(prover.sample_f128(), verifier.sample_f128());
        assert_eq!(proof.lincheck.z_partial.len(), 128);
        for claim in &claims {
            assert_eq!(claim.value, eval(&witness.0, claim));
        }
        assert_ne!(
            claims[2].value,
            F128::ZERO,
            "all-zero witness must fail the pin opening"
        );
        let mut bad = proof;
        bad.lincheck.z_partial[0] += F128::ONE;
        assert!(
            verify(
                &fixture.verifier(),
                &bad,
                &mut FsChallenger::new(b"flock-skip128-e2e-test")
            )
            .is_err()
        );
    }
}
