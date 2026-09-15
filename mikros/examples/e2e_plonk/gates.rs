//! Gate PIOPs for exactly the same alternating AND/XOR PLONK relation.
use super::{
    fixture::{Circuit, Mode, basis, bytes},
    queries::*,
};
use flock_core::{
    challenger::Challenger,
    pcs::functional::TensorClaim,
    zerocheck::{self, ZerocheckProof},
};
use flock_field::F128;
use mikros::{Kernel, weighted_hadamard as wh};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SingleProof {
    pub scalars: [F128; 5],
    pub hadamards: Vec<wh::Proof>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlockProof {
    pub zerocheck: ZerocheckProof,
    pub linear_values: [F128; 3],
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MultipleProof {
    pub gate: mikros::multiple_gate::Proof,
    pub hadamard: wh::Proof,
}

fn multiple_claims(
    c: &mikros::multiple_gate::Claims,
    terminals: [TensorClaim; 2],
    log_z: usize,
    log_pub: usize,
) -> Claims {
    let witness = [0, 1, 3]
        .into_iter()
        .enumerate()
        .map(|(block, k)| lift(ordinary(&c.point, c.wires[k]), block, log_z))
        .chain(
            terminals
                .into_iter()
                .enumerate()
                .map(|(block, q)| lift(q, block, log_z)),
        )
        .collect();
    let public = (0..5)
        .map(|k| {
            lift(
                TensorClaim {
                    columns: selector_column(k),
                    rows: pairs(&c.point),
                    value: c.selectors[k],
                },
                0,
                log_pub,
            )
        })
        .collect();
    Claims { witness, public }
}
pub fn multiple_prove<C: Challenger>(
    c: &Circuit,
    z: &[F128],
    log_pub: usize,
    ch: &mut C,
) -> Result<(MultipleProof, Claims), String> {
    assert_eq!(c.mode, Mode::Multiple);
    let n = c.n();
    let trace = std::env::var_os("E2E_PIOP_TRACE").is_some();
    let virtual_product =
        std::env::var_os("MIKROS_GATE_VIRTUAL_PRODUCT").is_some_and(|value| value == "1");
    let reuse_fold_buffers =
        std::env::var_os("MIKROS_GATE_WH_SCRATCH").is_some_and(|value| value == "1");
    if reuse_fold_buffers && !virtual_product {
        return Err("MIKROS_GATE_WH_SCRATCH requires MIKROS_GATE_VIRTUAL_PRODUCT=1".into());
    }
    let t_product = trace.then(std::time::Instant::now);
    let product = (!virtual_product).then(|| {
        z[..n]
            .par_iter()
            .zip(&z[n..2 * n])
            .map(|(a, b)| F128::new(a.lo & b.lo, a.hi & b.hi))
            .collect::<Vec<_>>()
    });
    if let Some(t) = t_product {
        eprintln!(
            "e2e_piop_trace gate=mikros-multiple stage=virtual_product ms={:.6}",
            t.elapsed().as_secs_f64() * 1000.0
        );
    }
    let t_sumcheck = trace.then(std::time::Instant::now);
    let selector = mikros::multiple_gate::RepeatedSelectors {
        period_log: 1,
        packed: [vec![2], vec![2], vec![1], vec![3], vec![0]],
    };
    let beta = basis();
    let sum = beta.iter().fold(F128::ZERO, |a, &b| a + b);
    let (gate, claim, fold_buffers) = if let Some(product) = product.as_ref() {
        mikros::multiple_gate::prove(
            [&z[..n], &z[n..2 * n], product, &z[2 * n..3 * n]],
            &selector,
            c.log_gates,
            sum,
            ch,
        )
        .map(|(proof, claims)| (proof, claims, None))
    } else if reuse_fold_buffers {
        mikros::multiple_gate::prove_virtual_product_with_fold_buffers(
            [&z[..n], &z[n..2 * n], &z[2 * n..3 * n]],
            &selector,
            c.log_gates,
            sum,
            ch,
        )
        .map(|(proof, claims, buffers)| (proof, claims, Some(buffers)))
    } else {
        mikros::multiple_gate::prove_virtual_product(
            [&z[..n], &z[n..2 * n], &z[2 * n..3 * n]],
            &selector,
            c.log_gates,
            sum,
            ch,
        )
        .map(|(proof, claims)| (proof, claims, None))
    }
    .map_err(|e| format!("multiple gates: {e:?}"))?;
    if let Some(t) = t_sumcheck {
        eprintln!(
            "e2e_piop_trace gate=mikros-multiple stage=sumcheck ms={:.6}",
            t.elapsed().as_secs_f64() * 1000.0
        );
    }
    let t_release = trace.then(std::time::Instant::now);
    drop(product);
    if let Some(t) = t_release {
        eprintln!(
            "e2e_piop_trace gate=mikros-multiple stage=virtual_product_release ms={:.6}",
            t.elapsed().as_secs_f64() * 1000.0
        );
    }
    let t_hadamard = trace.then(std::time::Instant::now);
    let q = TensorClaim {
        columns: beta,
        rows: pairs(&claim.point),
        value: claim.wires[2],
    };
    let (hadamard, terminals) =
        wh_prove_with_fold_buffers(&z[..n], &z[n..2 * n], c.log_gates + 7, &q, fold_buffers, ch)?;
    if let Some(t) = t_hadamard {
        eprintln!(
            "e2e_piop_trace gate=mikros-multiple stage=weighted_hadamard ms={:.6}",
            t.elapsed().as_secs_f64() * 1000.0
        );
    }
    Ok((
        MultipleProof { gate, hadamard },
        multiple_claims(&claim, terminals, z.len().ilog2() as usize, log_pub),
    ))
}
pub fn multiple_verify<C: Challenger>(
    c: &Circuit,
    log_z: usize,
    log_pub: usize,
    p: &MultipleProof,
    ch: &mut C,
) -> Result<Claims, String> {
    if c.mode != Mode::Multiple {
        return Err("multiple gate mode".into());
    }
    let beta = basis();
    let sum = beta.iter().fold(F128::ZERO, |a, &b| a + b);
    let claim = mikros::multiple_gate::verify(&p.gate, c.log_gates, 1, sum, ch)
        .map_err(|e| format!("multiple gate verify: {e:?}"))?;
    let q = TensorClaim {
        columns: beta,
        rows: pairs(&claim.point),
        value: claim.wires[2],
    };
    let terminals = wh_verify(&p.hadamard, c.log_gates + 7, &q, ch)?;
    Ok(multiple_claims(&claim, terminals, log_z, log_pub))
}
#[derive(Default)]
pub struct Claims {
    pub witness: Vec<TensorClaim>,
    pub public: Vec<TensorClaim>,
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
    wh_prove_with_fold_buffers(a, b, log_n, q, None, ch)
}
fn wh_prove_with_fold_buffers<C: Challenger>(
    a: &[F128],
    b: &[F128],
    log_n: usize,
    q: &TensorClaim,
    fold_buffers: Option<[Vec<F128>; 2]>,
    ch: &mut C,
) -> Result<(wh::Proof, [TensorClaim; 2]), String> {
    let high = q.rows.iter().rev().copied().collect::<Vec<_>>();
    if let Some(buffers) = fold_buffers {
        wh::prove_with_fold_buffers(
            bytes(a),
            bytes(b),
            log_n,
            &high,
            &q.columns,
            q.value,
            Kernel::PairAggregate,
            buffers,
            ch,
        )
    } else {
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
    }
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
pub fn single_prove<C: Challenger>(
    c: &Circuit,
    z: &[F128],
    public: &[F128],
    ch: &mut C,
) -> Result<(SingleProof, Claims), String> {
    assert_eq!(c.mode, Mode::Single);
    let m = c.wire_words();
    ch.observe_label(b"mikros-plonk-single-gate-v1");
    let x = ch.sample_f128_vec(c.log_gates);
    let virtual_m = z[..m]
        .par_iter()
        .zip(&z[m..2 * m])
        .map(|(a, b)| F128::new(a.lo & b.lo, a.hi & b.hi))
        .collect::<Vec<_>>();
    let wire = [&z[..m], &z[m..2 * m], &virtual_m, &z[2 * m..3 * m]];
    let mut scalars = [F128::ZERO; 5];
    for k in 0..4 {
        let product = public[k * m..(k + 1) * m]
            .par_iter()
            .zip(wire[k])
            .map(|(a, b)| F128::new(a.lo & b.lo, a.hi & b.hi))
            .collect::<Vec<_>>();
        scalars[k] = evaluate(&product, &single_query(&x, F128::ZERO));
    }
    scalars[4] = evaluate(&public[4 * m..5 * m], &single_query(&x, F128::ZERO));
    bind_scalars(&scalars, ch)?;
    let mut out = Claims::default();
    let mut ps = Vec::with_capacity(5);
    let mut nested = None;
    for k in [0, 1, 3, 2] {
        let q = single_query(&x, scalars[k]);
        let (p, [a, b]) = wh_prove(&public[k * m..(k + 1) * m], wire[k], c.log_gates, &q, ch)?;
        out.public.push(lift(a, k, public.len().ilog2() as usize));
        if k == 2 {
            nested = Some(b);
        } else {
            out.witness.push(lift(
                b,
                if k == 3 { 2 } else { k },
                z.len().ilog2() as usize,
            ));
        }
        ps.push(p);
    }
    let (p, qs) = wh_prove(wire[0], wire[1], c.log_gates, &nested.unwrap(), ch)?;
    ps.push(p);
    out.witness.extend(
        qs.into_iter()
            .enumerate()
            .map(|(b, q)| lift(q, b, z.len().ilog2() as usize)),
    );
    out.public.push(lift(
        single_query(&x, scalars[4]),
        4,
        public.len().ilog2() as usize,
    ));
    Ok((
        SingleProof {
            scalars,
            hadamards: ps,
        },
        out,
    ))
}
pub fn single_verify<C: Challenger>(
    c: &Circuit,
    log_z: usize,
    log_pub: usize,
    p: &SingleProof,
    ch: &mut C,
) -> Result<Claims, String> {
    if c.mode != Mode::Single || p.hadamards.len() != 5 {
        return Err("single gate proof shape".into());
    }
    ch.observe_label(b"mikros-plonk-single-gate-v1");
    let x = ch.sample_f128_vec(c.log_gates);
    bind_scalars(&p.scalars, ch)?;
    let mut out = Claims::default();
    let mut nested = None;
    for (i, k) in [0, 1, 3, 2].into_iter().enumerate() {
        let [a, b] = wh_verify(
            &p.hadamards[i],
            c.log_gates,
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
    let qs = wh_verify(&p.hadamards[4], c.log_gates, &nested.unwrap(), ch)?;
    out.witness
        .extend(qs.into_iter().enumerate().map(|(b, q)| lift(q, b, log_z)));
    out.public
        .push(lift(single_query(&x, p.scalars[4]), 4, log_pub));
    Ok(out)
}

fn flock_input(c: &Circuit, z: &[F128]) -> (Vec<F128>, Vec<F128>) {
    let m = c.wire_words();
    let mut a = vec![F128::ZERO; m];
    let mut b = a.clone();
    a.par_iter_mut()
        .zip(b.par_iter_mut())
        .enumerate()
        .for_each(|(i, (a, b))| {
            let mask = if c.mode == Mode::Single {
                F128::new(0xAAAAAAAAAAAAAAAA, 0xAAAAAAAAAAAAAAAA)
            } else if i & 1 == 1 {
                F128::new(u64::MAX, u64::MAX)
            } else {
                F128::ZERO
            };
            let l = z[i];
            let r = z[m + i];
            *a = l + F128::new(r.lo & mask.lo, r.hi & mask.hi);
            *b = F128::new((r.lo & !mask.lo) ^ mask.lo, (r.hi & !mask.hi) ^ mask.hi);
        });
    (a, b)
}
fn flock_queries(
    c: &Circuit,
    zc: &zerocheck::ZerocheckClaim,
    values: &[F128; 3],
    log_z: usize,
) -> Result<Vec<TensorClaim>, String> {
    let col = zerocheck::multilinear::lagrange_weights_naive(7, zc.z);
    let base = TensorClaim {
        columns: col.clone(),
        rows: pairs(&zc.mlv_challenges),
        value: values[0],
    };
    let mut odd = base.clone();
    odd.value = values[1];
    let mut even = base.clone();
    even.value = values[2];
    let const_b = if c.mode == Mode::Single {
        for j in 0..128 {
            if j & 1 == 0 {
                odd.columns[j] = F128::ZERO;
            } else {
                even.columns[j] = F128::ZERO;
            }
        }
        col.iter()
            .skip(1)
            .step_by(2)
            .fold(F128::ZERO, |a, &b| a + b)
    } else {
        odd.rows[0][0] = F128::ZERO;
        even.rows[0][1] = F128::ZERO;
        zc.mlv_challenges[0] * col.iter().fold(F128::ZERO, |a, &b| a + b)
    };
    if values[0] + values[1] != zc.a_eval || values[2] + const_b != zc.b_eval {
        return Err("Flock fixed-gadget linear relation".into());
    }
    let cquery = TensorClaim {
        columns: col,
        rows: pairs(&zc.r_rest),
        value: zc.c_eval,
    };
    Ok(vec![
        lift(base, 0, log_z),
        lift(odd, 1, log_z),
        lift(even, 1, log_z),
        lift(cquery, 2, log_z),
    ])
}
pub fn flock_prove<C: Challenger>(
    c: &Circuit,
    z: &[F128],
    ch: &mut C,
) -> Result<(FlockProof, Claims), String> {
    ch.observe_label(b"flock-plonk-fixed-gadget-native-skip128-v1");
    let (a, b) = flock_input(c, z);
    let m = c.wire_words();
    // These challenge-independent linear conversions are intentionally charged
    // online here, equally reported as part of the native gate prover.
    let log_bits = c.log_gates + if c.mode == Mode::Multiple { 7 } else { 0 };
    let (p, zc) =
        zerocheck::prove_packed_skip7(bytes(&a), bytes(&b), bytes(&z[2 * m..3 * m]), log_bits, ch);
    drop(a);
    drop(b);
    let col = zerocheck::multilinear::lagrange_weights_naive(7, zc.z);
    let q = TensorClaim {
        columns: col.clone(),
        rows: pairs(&zc.mlv_challenges),
        value: F128::ZERO,
    };
    let first = evaluate(&z[..m], &q);
    let second = zc.a_eval + first;
    let const_b = if c.mode == Mode::Single {
        col.iter()
            .skip(1)
            .step_by(2)
            .fold(F128::ZERO, |a, &b| a + b)
    } else {
        zc.mlv_challenges[0] * col.iter().fold(F128::ZERO, |a, &b| a + b)
    };
    let values = [first, second, zc.b_eval + const_b];
    ch.observe_f128_slice(&values);
    let claims = flock_queries(c, &zc, &values, z.len().ilog2() as usize)?;
    Ok((
        FlockProof {
            zerocheck: p,
            linear_values: values,
        },
        Claims {
            witness: claims,
            public: vec![],
        },
    ))
}
pub fn flock_verify<C: Challenger>(
    c: &Circuit,
    log_z: usize,
    p: &FlockProof,
    ch: &mut C,
) -> Result<Claims, String> {
    ch.observe_label(b"flock-plonk-fixed-gadget-native-skip128-v1");
    let log_bits = c.log_gates + if c.mode == Mode::Multiple { 7 } else { 0 };
    let zc = zerocheck::verify_skip7(log_bits, &p.zerocheck, ch)
        .map_err(|e| format!("Flock gate: {e:?}"))?;
    ch.observe_f128_slice(&p.linear_values);
    Ok(Claims {
        witness: flock_queries(c, &zc, &p.linear_values, log_z)?,
        public: vec![],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::challenger::FsChallenger;

    fn check_sources(claims: &Claims, witness: &[F128], public: &[F128]) -> bool {
        claims
            .witness
            .iter()
            .all(|q| evaluate(witness, q) == q.value)
            && claims.public.iter().all(|q| evaluate(public, q) == q.value)
    }

    fn assert_claims_eq(a: &Claims, b: &Claims) {
        assert_eq!(a.witness, b.witness);
        assert_eq!(a.public, b.public);
    }

    #[test]
    fn multiple_both_gate_paths_authenticate_sources_and_replay_transcript() {
        let c = Circuit::new(14, Mode::Multiple, 193);
        let z = c.witness();
        let public = c.public();
        let log_z = z.len().ilog2() as usize;
        let log_pub = public.len().ilog2() as usize;
        let mut pc = FsChallenger::new(b"multiple-gate-source-test");
        let (proof, prover_claims) = multiple_prove(&c, &z, log_pub, &mut pc).unwrap();
        let mut vc = FsChallenger::new(b"multiple-gate-source-test");
        let claims = multiple_verify(&c, log_z, log_pub, &proof, &mut vc).unwrap();
        assert_claims_eq(&prover_claims, &claims);
        assert_eq!((claims.witness.len(), claims.public.len()), (5, 5));
        assert!(check_sources(&claims, &z, &public));
        assert_eq!(pc.sample_f128(), vc.sample_f128());
        // The virtual product value sent by the cubic gate proof is the
        // public scalar of the following Hadamard, not a free terminal.
        let mut bad = proof.clone();
        bad.gate.wires[2] += F128::ONE;
        assert!(
            multiple_verify(
                &c,
                log_z,
                log_pub,
                &bad,
                &mut FsChallenger::new(b"multiple-gate-source-test")
            )
            .is_err()
        );
        let mut bad = proof.clone();
        bad.hadamard.column_claims[0] += F128::ONE;
        assert!(
            multiple_verify(
                &c,
                log_z,
                log_pub,
                &bad,
                &mut FsChallenger::new(b"multiple-gate-source-test")
            )
            .is_err()
        );

        let mut pc = FsChallenger::new(b"multiple-flock-source-test");
        let (proof, prover_claims) = flock_prove(&c, &z, &mut pc).unwrap();
        let mut vc = FsChallenger::new(b"multiple-flock-source-test");
        let claims = flock_verify(&c, log_z, &proof, &mut vc).unwrap();
        assert_claims_eq(&prover_claims, &claims);
        assert_eq!((claims.witness.len(), claims.public.len()), (4, 0));
        assert!(check_sources(&claims, &z, &public));
        // Odd/even gates occupy the first ROW bit in multiple mode. Preserve
        // its factor, including the scale r or (1+r), rather than normalize.
        assert_eq!(claims.witness[1].rows[0][0], F128::ZERO);
        assert_eq!(claims.witness[2].rows[0][1], F128::ZERO);
        assert_eq!(claims.witness[1].rows[0][1], claims.witness[0].rows[0][1]);
        assert_eq!(claims.witness[2].rows[0][0], claims.witness[0].rows[0][0]);
        assert_eq!(pc.sample_f128(), vc.sample_f128());
        // Preserve the local a_eval equation while changing its two source
        // values: the deferred PCS queries still have to reject the forgery.
        let mut bad = proof;
        bad.linear_values[0] += F128::ONE;
        bad.linear_values[1] += F128::ONE;
        let bad_claims = flock_verify(
            &c,
            log_z,
            &bad,
            &mut FsChallenger::new(b"multiple-flock-source-test"),
        )
        .unwrap();
        assert!(!check_sources(&bad_claims, &z, &public));
    }

    #[test]
    fn multiple_virtual_product_must_be_bound_even_on_xor_gates() {
        let c = Circuit::new(14, Mode::Multiple, 197);
        let z = c.witness();
        let n = c.n();
        let mut fake_product: Vec<_> = z[..n]
            .iter()
            .zip(&z[n..2 * n])
            .map(|(a, b)| F128::new(a.lo & b.lo, a.hi & b.hi))
            .collect();
        // q_M=0 on this row, so the gate relation alone cannot detect this.
        fake_product[1] += F128::ONE;
        let selectors = mikros::multiple_gate::RepeatedSelectors {
            period_log: 1,
            packed: [vec![2], vec![2], vec![1], vec![3], vec![0]],
        };
        let beta = basis();
        let sum = beta.iter().copied().fold(F128::ZERO, |a, b| a + b);
        let mut pc = FsChallenger::new(b"multiple-false-virtual-product");
        let (proof, claim) = mikros::multiple_gate::prove(
            [&z[..n], &z[n..2 * n], &fake_product, &z[2 * n..3 * n]],
            &selectors,
            c.log_gates,
            sum,
            &mut pc,
        )
        .unwrap();
        let mut vc = FsChallenger::new(b"multiple-false-virtual-product");
        assert_eq!(
            mikros::multiple_gate::verify(&proof, c.log_gates, 1, sum, &mut vc).unwrap(),
            claim
        );
        assert_eq!(pc.sample_f128(), vc.sample_f128());
        let q = TensorClaim {
            columns: beta,
            rows: pairs(&claim.point),
            value: claim.wires[2],
        };
        assert!(wh_prove(&z[..n], &z[n..2 * n], c.log_gates + 7, &q, &mut pc).is_err());
    }

    #[test]
    fn both_single_gate_paths_authenticate_all_terminals() {
        let c = Circuit::new(14, Mode::Single, 97);
        let z = c.witness();
        let q = c.public();
        let mut pc = FsChallenger::new(b"gate-test");
        let (p, prover_claims) = single_prove(&c, &z, &q, &mut pc).unwrap();
        let mut vc = FsChallenger::new(b"gate-test");
        let claims = single_verify(
            &c,
            z.len().ilog2() as usize,
            q.len().ilog2() as usize,
            &p,
            &mut vc,
        )
        .unwrap();
        assert_claims_eq(&prover_claims, &claims);
        for x in &claims.witness {
            assert_eq!(evaluate(&z, x), x.value);
        }
        for x in &claims.public {
            assert_eq!(evaluate(&q, x), x.value);
        }
        assert_eq!(pc.sample_f128(), vc.sample_f128());
        let mut bad = p.clone();
        bad.scalars[0] += F128::ONE;
        assert!(
            single_verify(
                &c,
                z.len().ilog2() as usize,
                q.len().ilog2() as usize,
                &bad,
                &mut FsChallenger::new(b"gate-test")
            )
            .is_err()
        );
        let mut pc = FsChallenger::new(b"fgate-test");
        let (p, prover_claims) = flock_prove(&c, &z, &mut pc).unwrap();
        let mut vc = FsChallenger::new(b"fgate-test");
        let claims = flock_verify(&c, z.len().ilog2() as usize, &p, &mut vc).unwrap();
        assert_claims_eq(&prover_claims, &claims);
        for x in claims.witness {
            assert_eq!(evaluate(&z, &x), x.value);
        }
        assert_eq!(pc.sample_f128(), vc.sample_f128());
    }
}
