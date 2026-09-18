//! Single-instance PLONK gate reduction from Section 6 of the manuscript.
//! The virtual product is never committed: its selector-weighted claim is
//! discharged by the fifth weighted Hadamard invocation.

use super::{
    fixture::{Circuit, bytes},
    queries::*,
};
use flock_core::{challenger::Challenger, pcs::functional::TensorClaim};
use flock_field::{F128, F256Unreduced};
use mikros::{Kernel, weighted_hadamard as wh};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SingleProof {
    pub scalars: [F128; 5],
    pub hadamards: Vec<wh::Proof>,
}

#[derive(Default)]
pub struct Claims {
    pub witness: Vec<TensorClaim>,
    pub public: Vec<TensorClaim>,
}

#[inline]
fn bitand(a: F128, b: F128) -> F128 {
    F128::new(a.lo & b.lo, a.hi & b.hi)
}

fn single_query(x: &[F128], value: F128) -> TensorClaim {
    TensorClaim {
        columns: tensor(&x[..7]),
        rows: pairs(&x[7..]),
        value,
    }
}

fn wh_prove<C: Challenger>(
    a: &[F128],
    b: &[F128],
    log_n: usize,
    q: &TensorClaim,
    ch: &mut C,
) -> Result<(wh::Proof, [TensorClaim; 2]), String> {
    let high = q.rows.iter().rev().copied().collect::<Vec<_>>();
    wh::prove(
        bytes(a),
        bytes(b),
        log_n,
        &high,
        &q.columns,
        q.value,
        Kernel::PairAggregate,
        ch,
    )
    .map(|(p, c)| (p, c.map(matrix)))
    .map_err(|e| format!("weighted Hadamard: {e:?}"))
}

fn wh_verify<C: Challenger>(
    p: &wh::Proof,
    log_n: usize,
    q: &TensorClaim,
    ch: &mut C,
) -> Result<[TensorClaim; 2], String> {
    let high = q.rows.iter().rev().copied().collect::<Vec<_>>();
    wh::verify(p, log_n, &high, &q.columns, q.value, ch)
        .map(|c| c.map(matrix))
        .map_err(|e| format!("weighted Hadamard verify: {e:?}"))
}

fn bind_scalars<C: Challenger>(scalars: &[F128; 5], ch: &mut C) -> Result<(), String> {
    ch.observe_f128_slice(scalars);
    if scalars.iter().fold(F128::ZERO, |a, &b| a + b) != F128::ZERO {
        return Err("gate scalar sum is nonzero".into());
    }
    Ok(())
}

/// Keep the old materialize-then-fold implementation as a measured reference.
/// This is a prover implementation choice, with no transcript effect.
fn reference_scalars(wire: [&[F128]; 4], public: &[F128], x: &[F128]) -> [F128; 5] {
    let m = wire[0].len();
    let query = single_query(x, F128::ZERO);
    let mut scalars = [F128::ZERO; 5];
    for k in 0..4 {
        let product = public[k * m..(k + 1) * m]
            .par_iter()
            .zip(wire[k])
            .map(|(&a, &b)| bitand(a, b))
            .collect::<Vec<_>>();
        scalars[k] = evaluate(&product, &query);
    }
    scalars[4] = evaluate(&public[4 * m..5 * m], &query);
    scalars
}

/// Challenge-dependent column tables are shared by all five projections.
/// Unlike fixed circuit preparation, their construction is online work.
fn column_tables(columns: &[F128]) -> Vec<[F128; 256]> {
    assert_eq!(columns.len(), 128);
    let mut tables = vec![[F128::ZERO; 256]; 16];
    for (byte, table) in tables.iter_mut().enumerate() {
        for bit in 0..8 {
            let half = 1 << bit;
            for mask in 0..half {
                table[half + mask] = table[mask] + columns[byte * 8 + bit];
            }
        }
    }
    tables
}

#[inline]
fn project_word(word: F128, tables: &[[F128; 256]]) -> F128 {
    let lo = word.lo.to_le_bytes();
    let hi = word.hi.to_le_bytes();
    let mut partial = [F128::ZERO; 4];
    for i in 0..4 {
        partial[i] = tables[i][lo[i] as usize]
            + tables[4 + i][lo[4 + i] as usize]
            + tables[8 + i][hi[i] as usize]
            + tables[12 + i][hi[4 + i] as usize];
    }
    partial[0] + partial[1] + partial[2] + partial[3]
}

/// Project virtual products as they are loaded, without allocating four
/// selector-times-wire arrays. Split row weights and one table are shared;
/// carryless dot products defer reduction and use four hardware lanes when
/// compiled for the server's AVX-512/VPCLMUL instruction set.
fn fused_scalars(wire: [&[F128]; 4], public: &[F128], x: &[F128]) -> [F128; 5] {
    let m = wire[0].len();
    let columns = tensor(&x[..7]);
    let tables = column_tables(&columns);
    let rows = &x[7..];
    let split = rows.len().div_ceil(2).min(10);
    let lo = tensor(&rows[..split]);
    let hi = tensor(&rows[split..]);
    let block = |block: usize| {
        let offset = block * lo.len();
        let mut acc = [F256Unreduced::ZERO; 5];
        #[allow(unused_mut)]
        let mut i = 0;
        #[cfg(all(
            target_arch = "x86_64",
            target_feature = "avx512f",
            target_feature = "vpclmulqdq",
            target_feature = "sse4.1"
        ))]
        // SAFETY: all ISA requirements are compile-time gated. Every load
        // reads four in-bounds F128 values, with scalar handling of tails.
        unsafe {
            use flock_field::gf2_128::x86_64::{WideGhashX4, f128x4_loadu};
            let mut wide = [WideGhashX4::zero(); 5];
            while i + 4 <= lo.len() {
                let weights = f128x4_loadu(lo.as_ptr().add(i));
                for k in 0..5 {
                    let values: [F128; 4] = std::array::from_fn(|lane| {
                        let row = offset + i + lane;
                        let selector = public[k * m + row];
                        let product = if k < 4 {
                            bitand(selector, wire[k][row])
                        } else {
                            selector
                        };
                        project_word(product, &tables)
                    });
                    wide[k].mul_acc(f128x4_loadu(values.as_ptr()), weights);
                }
                i += 4;
            }
            for (sum, wide) in acc.iter_mut().zip(wide) {
                *sum ^= wide.fold();
            }
        }
        for (i, &weight) in lo.iter().enumerate().skip(i) {
            let row = offset + i;
            for k in 0..5 {
                let selector = public[k * m + row];
                let product = if k < 4 {
                    bitand(selector, wire[k][row])
                } else {
                    selector
                };
                acc[k] ^= project_word(product, &tables).mul_unreduced(weight);
            }
        }
        acc.map(|sum| sum.reduce() * hi[block])
    };
    let add = |a: [F128; 5], b: [F128; 5]| std::array::from_fn(|k| a[k] + b[k]);
    if m >= 1 << 15 {
        (0..hi.len())
            .into_par_iter()
            .map(block)
            .reduce(|| [F128::ZERO; 5], add)
    } else {
        (0..hi.len()).map(block).fold([F128::ZERO; 5], add)
    }
}

fn prove_impl<C: Challenger>(
    log_gates: usize,
    z: &[F128],
    public: &[F128],
    reference: bool,
    ch: &mut C,
) -> Result<(SingleProof, Claims), String> {
    let m = 1usize << (log_gates - 7);
    ch.observe_label(b"mikros-plonk-single-gate-v1");
    let x = ch.sample_f128_vec(log_gates);
    let virtual_m: Vec<_> = if m >= 1 << 15 {
        z[..m]
            .par_iter()
            .zip(&z[m..2 * m])
            .map(|(&a, &b)| bitand(a, b))
            .collect()
    } else {
        z[..m]
            .iter()
            .zip(&z[m..2 * m])
            .map(|(&a, &b)| bitand(a, b))
            .collect()
    };
    let wire = [&z[..m], &z[m..2 * m], &virtual_m, &z[2 * m..3 * m]];
    let scalars = if reference {
        reference_scalars(wire, public, &x)
    } else {
        fused_scalars(wire, public, &x)
    };
    bind_scalars(&scalars, ch)?;
    let log_z = z.len().ilog2() as usize;
    let log_pub = public.len().ilog2() as usize;
    let mut out = Claims::default();
    let mut ps = Vec::with_capacity(5);
    let mut nested = None;
    // Preserve the manuscript's three ordinary wires followed by the virtual
    // product, and the established transcript ordering L, R, O, M, L*R.
    for k in [0, 1, 3, 2] {
        let q = single_query(&x, scalars[k]);
        let (p, [a, b]) = wh_prove(&public[k * m..(k + 1) * m], wire[k], log_gates, &q, ch)?;
        out.public.push(lift(a, k, log_pub));
        if k == 2 {
            nested = Some(b);
        } else {
            out.witness.push(lift(b, if k == 3 { 2 } else { k }, log_z));
        }
        ps.push(p);
    }
    let (p, qs) = wh_prove(wire[0], wire[1], log_gates, &nested.unwrap(), ch)?;
    ps.push(p);
    out.witness
        .extend(qs.into_iter().enumerate().map(|(b, q)| lift(q, b, log_z)));
    out.public
        .push(lift(single_query(&x, scalars[4]), 4, log_pub));
    Ok((
        SingleProof {
            scalars,
            hadamards: ps,
        },
        out,
    ))
}

pub fn single_prove<C: Challenger>(
    c: &Circuit,
    z: &[F128],
    public: &[F128],
    ch: &mut C,
) -> Result<(SingleProof, Claims), String> {
    if c.log_gates < 7 || z.len() != c.witness_words() || public.len() != c.public_words() {
        return Err("single gate source shape".into());
    }
    let reference = match std::env::var("MIKROS_SINGLE_GATE_REFERENCE") {
        Ok(value) if value == "1" => true,
        Ok(value) if value == "0" => false,
        Err(std::env::VarError::NotPresent) => false,
        _ => return Err("MIKROS_SINGLE_GATE_REFERENCE must be 0 or 1".into()),
    };
    prove_impl(c.log_gates, z, public, reference, ch)
}

fn verify_impl<C: Challenger>(
    log_gates: usize,
    log_z: usize,
    log_pub: usize,
    p: &SingleProof,
    ch: &mut C,
) -> Result<Claims, String> {
    if log_gates < 7
        || log_z < log_gates - 7 + 2
        || log_pub < log_gates - 7 + 3
        || p.hadamards.len() != 5
    {
        return Err("single gate proof shape".into());
    }
    ch.observe_label(b"mikros-plonk-single-gate-v1");
    let x = ch.sample_f128_vec(log_gates);
    bind_scalars(&p.scalars, ch)?;
    let mut out = Claims::default();
    let mut nested = None;
    for (i, k) in [0, 1, 3, 2].into_iter().enumerate() {
        let [a, b] = wh_verify(
            &p.hadamards[i],
            log_gates,
            &single_query(&x, p.scalars[k]),
            ch,
        )?;
        out.public.push(lift(a, k, log_pub));
        if k == 2 {
            nested = Some(b);
        } else {
            out.witness.push(lift(b, if k == 3 { 2 } else { k }, log_z));
        }
    }
    let qs = wh_verify(&p.hadamards[4], log_gates, &nested.unwrap(), ch)?;
    out.witness
        .extend(qs.into_iter().enumerate().map(|(b, q)| lift(q, b, log_z)));
    out.public
        .push(lift(single_query(&x, p.scalars[4]), 4, log_pub));
    Ok(out)
}

pub fn single_verify<C: Challenger>(
    c: &Circuit,
    log_z: usize,
    log_pub: usize,
    p: &SingleProof,
    ch: &mut C,
) -> Result<Claims, String> {
    if 1usize.checked_shl(log_z as u32) != Some(c.witness_words())
        || 1usize.checked_shl(log_pub as u32) != Some(c.public_words())
    {
        return Err("single gate commitment shape".into());
    }
    verify_impl(c.log_gates, log_z, log_pub, p, ch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::challenger::FsChallenger;
    use flock_field::test_rng::Rng;

    fn fixture(log_gates: usize, seed: u64) -> (Vec<F128>, Vec<F128>) {
        let m = 1usize << (log_gates - 7);
        let mut rng = Rng::new(seed);
        let mut z = vec![F128::ZERO; 4 * m];
        let mut public = vec![F128::ZERO; 8 * m];
        for i in 0..m {
            let (l, r, xor, constant) = (rng.f128(), rng.f128(), rng.f128(), rng.f128());
            let and = F128::new(!xor.lo, !xor.hi);
            // Random per-bit AND/XOR selection, plus a random additive term.
            // This tests all five scalar sources, including nonzero q_C.
            z[i] = l;
            z[m + i] = r;
            z[2 * m + i] = bitand(l + r, xor) + bitand(bitand(l, r), and) + constant;
            public[i] = xor;
            public[m + i] = xor;
            public[2 * m + i] = and;
            public[3 * m + i] = F128::new(u64::MAX, u64::MAX);
            public[4 * m + i] = constant;
        }
        (z, public)
    }

    fn source_claims_hold(claims: &Claims, z: &[F128], public: &[F128]) -> bool {
        claims.witness.iter().all(|q| evaluate(z, q) == q.value)
            && claims.public.iter().all(|q| evaluate(public, q) == q.value)
    }

    #[test]
    fn compiler_fixture_gate_api_authenticates_selector_and_wire_sources() {
        let (c, z) = super::super::fixture::tiny_fixture();
        let public = c.public();
        let mut pc = FsChallenger::new(b"compiled-single-gates");
        let (proof, claims) = single_prove(&c, &z, &public, &mut pc).unwrap();
        let mut vc = FsChallenger::new(b"compiled-single-gates");
        let verified = single_verify(
            &c,
            z.len().ilog2() as usize,
            public.len().ilog2() as usize,
            &proof,
            &mut vc,
        )
        .unwrap();
        assert_eq!(claims.witness, verified.witness);
        assert_eq!(claims.public, verified.public);
        assert!(source_claims_hold(&claims, &z, &public));
        assert_eq!(pc.sample_f128(), vc.sample_f128());
    }

    #[test]
    fn fused_single_scalars_match_reference_for_boolean_and_random_challenges() {
        // Include scalar tails, vector batches and the parallel projection path.
        for log_gates in [7usize, 8, 10, 15, 22] {
            let (z, public) = fixture(log_gates, 0x5137 + log_gates as u64);
            let m = 1usize << (log_gates - 7);
            let product: Vec<_> = z[..m]
                .iter()
                .zip(&z[m..2 * m])
                .map(|(&a, &b)| bitand(a, b))
                .collect();
            let wire = [&z[..m], &z[m..2 * m], &product[..], &z[2 * m..3 * m]];
            let mut rng = Rng::new(0xA113 + log_gates as u64);
            for point in [
                vec![F128::ZERO; log_gates],
                vec![F128::ONE; log_gates],
                rng.f128_vec(log_gates),
            ] {
                let reference = reference_scalars(wire, &public, &point);
                assert_eq!(fused_scalars(wire, &public, &point), reference);
                assert_eq!(
                    reference.into_iter().fold(F128::ZERO, |a, b| a + b),
                    F128::ZERO
                );
            }
        }
    }

    #[test]
    fn single_gate_optimization_preserves_proof_terminals_and_transcript() {
        for log_gates in [7usize, 10, 14] {
            let (z, public) = fixture(log_gates, 0x6117 + log_gates as u64);
            let log_z = z.len().ilog2() as usize;
            let log_pub = public.len().ilog2() as usize;
            let mut reference_ch = FsChallenger::new(b"single-plonk-equivalence");
            let mut optimized_ch = FsChallenger::new(b"single-plonk-equivalence");
            let (reference, rc) =
                prove_impl(log_gates, &z, &public, true, &mut reference_ch).unwrap();
            let (optimized, oc) =
                prove_impl(log_gates, &z, &public, false, &mut optimized_ch).unwrap();
            assert_eq!(
                bincode::serialize(&reference).unwrap(),
                bincode::serialize(&optimized).unwrap()
            );
            assert_eq!(rc.witness, oc.witness);
            assert_eq!(rc.public, oc.public);
            assert_eq!((oc.witness.len(), oc.public.len()), (5, 5));
            assert!(source_claims_hold(&oc, &z, &public));
            let mut vc = FsChallenger::new(b"single-plonk-equivalence");
            let verified = verify_impl(log_gates, log_z, log_pub, &optimized, &mut vc).unwrap();
            assert_eq!(verified.witness, oc.witness);
            assert_eq!(verified.public, oc.public);
            let next = optimized_ch.sample_f128();
            assert_eq!(next, reference_ch.sample_f128());
            assert_eq!(next, vc.sample_f128());

            let mut bad = optimized.clone();
            bad.scalars[0] += F128::ONE;
            assert!(
                verify_impl(
                    log_gates,
                    log_z,
                    log_pub,
                    &bad,
                    &mut FsChallenger::new(b"single-plonk-equivalence")
                )
                .is_err()
            );
            // Retain the scalar-sum equation; the individual WH binding must
            // still detect the forged pair, before the PCS stage.
            bad.scalars[1] += F128::ONE;
            assert!(
                verify_impl(
                    log_gates,
                    log_z,
                    log_pub,
                    &bad,
                    &mut FsChallenger::new(b"single-plonk-equivalence")
                )
                .is_err()
            );
            for index in 0..5 {
                let mut bad = optimized.clone();
                bad.hadamards[index].column_claims[0] += F128::ONE;
                assert!(
                    verify_impl(
                        log_gates,
                        log_z,
                        log_pub,
                        &bad,
                        &mut FsChallenger::new(b"single-plonk-equivalence")
                    )
                    .is_err()
                );
            }
            let mut bad = optimized;
            bad.hadamards.pop();
            assert!(
                verify_impl(
                    log_gates,
                    log_z,
                    log_pub,
                    &bad,
                    &mut FsChallenger::new(b"single-plonk-equivalence")
                )
                .is_err()
            );
        }
    }
}
