//! Five weighted Hadamard reductions for PLONK over an actual binary subfield.
use super::{
    fixture::{Circuit, Claims},
    hadamard::{self, Claim},
    queries,
};
use flock_core::{challenger::Challenger, pcs::functional::TensorClaim};
use flock_field::F128;
use mikros_permutation::field::{FastField, SmallField, from_raw, raw};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Proof {
    pub scalars: [F128; 5],
    pub hadamards: Vec<hadamard::Proof>,
}

pub fn bit_query(q: &Claim, bits: usize) -> Result<TensorClaim, String> {
    let field = SmallField::for_width(bits).map_err(|e| e.to_string())?;
    if ![1, 2, 4, 8, 16, 32, 64, 128].contains(&bits) || q.columns.len() != 128 / bits {
        return Err("field query width".into());
    }
    Ok(TensorClaim {
        columns: q
            .columns
            .iter()
            .flat_map(|&r| field.basis().iter().map(move |&v| r * v))
            .collect(),
        rows: q.rows.clone(),
        value: q.value,
    })
}

fn query(point: &[F128], bits: usize, value: F128) -> Claim {
    let k = (128 / bits).ilog2() as usize;
    Claim {
        columns: queries::tensor(&point[..k]),
        rows: queries::pairs(&point[k..]),
        value,
    }
}

pub fn product(a: &[F128], b: &[F128], bits: usize) -> Result<Vec<F128>, String> {
    if a.len() != b.len() || ![1, 2, 4, 8, 16, 32, 64, 128].contains(&bits) {
        return Err("field product shape".into());
    }
    if bits == 128 {
        return Ok(a.par_iter().zip(b).map(|(&a, &b)| a * b).collect());
    }
    let field = FastField::for_width(bits).map_err(|e| e.to_string())?;
    let mask = field.base.mask();
    Ok(a.par_iter()
        .zip(b)
        .map(|(&a, &b)| {
            if bits == 1 {
                return from_raw(raw(a) & raw(b));
            }
            let (a, b) = (raw(a), raw(b));
            from_raw((0..128 / bits).fold(0, |out, i| {
                let left = (a >> (i * bits)) & mask;
                let right = (b >> (i * bits)) & mask;
                let value = if left == 0 || right == 0 {
                    0
                } else if left == 1 {
                    right
                } else if right == 1 {
                    left
                } else {
                    field.multiply(left, right)
                };
                out | (value << (i * bits))
            }))
        })
        .collect())
}

fn bind_scalars<C: Challenger>(values: &[F128; 5], ch: &mut C) -> Result<(), String> {
    ch.observe_f128_slice(values);
    if values.iter().fold(F128::ZERO, |a, &b| a + b) != F128::ZERO {
        return Err("PLONK gate scalar sum".into());
    }
    Ok(())
}

pub fn prove<C: Challenger>(
    c: &Circuit,
    z: &[F128],
    public: &[F128],
    ch: &mut C,
) -> Result<(Proof, Claims), String> {
    c.validate()?;
    let m = c.wire_words();
    if z.len() != (4 * m).max(1 << 15) || public.len() != c.public_words() {
        return Err("gate source shape".into());
    }
    ch.observe_label(b"mikros-plonk-field-gate-v1");
    ch.observe_bytes(&(c.value_bits as u64).to_le_bytes());
    let x = ch.sample_f128_vec(c.log_gates);
    let virtual_product = product(&z[..m], &z[m..2 * m], c.value_bits)?;
    let wires = [&z[..m], &z[m..2 * m], &virtual_product, &z[2 * m..3 * m]];
    let mut scalars = [F128::ZERO; 5];
    let q = bit_query(&query(&x, c.value_bits, F128::ZERO), c.value_bits)?;
    for k in 0..4 {
        let v = product(&public[k * m..(k + 1) * m], wires[k], c.value_bits)?;
        scalars[k] = queries::evaluate(&v, &q);
    }
    scalars[4] = queries::evaluate(&public[4 * m..5 * m], &q);
    bind_scalars(&scalars, ch)?;
    let mut claims = Claims::default();
    let mut proofs = Vec::with_capacity(5);
    let mut nested = None;
    for k in [0, 1, 3, 2] {
        let (p, [a, b]) = hadamard::prove(
            &public[k * m..(k + 1) * m],
            wires[k],
            c.value_bits,
            &query(&x, c.value_bits, scalars[k]),
            ch,
        )?;
        claims.public.push(queries::lift(
            bit_query(&a, c.value_bits)?,
            k,
            public.len().ilog2() as usize,
        ));
        if k == 2 {
            nested = Some(b);
        } else {
            claims.witness.push(queries::lift(
                bit_query(&b, c.value_bits)?,
                if k == 3 { 2 } else { k },
                z.len().ilog2() as usize,
            ));
        }
        proofs.push(p);
    }
    let (p, terminals) = hadamard::prove(wires[0], wires[1], c.value_bits, &nested.unwrap(), ch)?;
    proofs.push(p);
    for (block, q) in terminals.iter().enumerate() {
        claims.witness.push(queries::lift(
            bit_query(q, c.value_bits)?,
            block,
            z.len().ilog2() as usize,
        ));
    }
    claims.public.push(queries::lift(
        bit_query(&query(&x, c.value_bits, scalars[4]), c.value_bits)?,
        4,
        public.len().ilog2() as usize,
    ));
    Ok((
        Proof {
            scalars,
            hadamards: proofs,
        },
        claims,
    ))
}

pub fn verify<C: Challenger>(
    c: &Circuit,
    log_z: usize,
    log_public: usize,
    p: &Proof,
    ch: &mut C,
) -> Result<Claims, String> {
    c.validate()?;
    if p.hadamards.len() != 5
        || log_z != (4 * c.wire_words()).max(1 << 15).ilog2() as usize
        || log_public != c.public_words().ilog2() as usize
    {
        return Err("gate proof shape".into());
    }
    ch.observe_label(b"mikros-plonk-field-gate-v1");
    ch.observe_bytes(&(c.value_bits as u64).to_le_bytes());
    let x = ch.sample_f128_vec(c.log_gates);
    bind_scalars(&p.scalars, ch)?;
    let mut claims = Claims::default();
    let mut nested = None;
    for (i, k) in [0, 1, 3, 2].into_iter().enumerate() {
        let [a, b] = hadamard::verify(
            &p.hadamards[i],
            c.value_bits,
            &query(&x, c.value_bits, p.scalars[k]),
            ch,
        )?;
        claims
            .public
            .push(queries::lift(bit_query(&a, c.value_bits)?, k, log_public));
        if k == 2 {
            nested = Some(b);
        } else {
            claims.witness.push(queries::lift(
                bit_query(&b, c.value_bits)?,
                if k == 3 { 2 } else { k },
                log_z,
            ));
        }
    }
    let terminals = hadamard::verify(&p.hadamards[4], c.value_bits, &nested.unwrap(), ch)?;
    for (block, q) in terminals.iter().enumerate() {
        claims
            .witness
            .push(queries::lift(bit_query(q, c.value_bits)?, block, log_z));
    }
    claims.public.push(queries::lift(
        bit_query(&query(&x, c.value_bits, p.scalars[4]), c.value_bits)?,
        4,
        log_public,
    ));
    Ok(claims)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::challenger::FsChallenger;
    #[test]
    fn field_products_and_authenticated_gate_terminals() {
        for bits in [1, 2, 4, 8, 16, 32, 64, 128] {
            let c = Circuit::new(14, bits, 97).unwrap();
            let z = c.witness();
            let public = c.public();
            let mut pc = FsChallenger::new(b"field-gates-test");
            let (p, _) = prove(&c, &z, &public, &mut pc).unwrap();
            let mut vc = FsChallenger::new(b"field-gates-test");
            let qs = verify(
                &c,
                z.len().ilog2() as usize,
                public.len().ilog2() as usize,
                &p,
                &mut vc,
            )
            .unwrap();
            assert_eq!(pc.sample_f128(), vc.sample_f128());
            for (source, queries) in [(&z, qs.witness), (&public, qs.public)] {
                for q in queries {
                    assert_eq!(queries::evaluate(source, &q), q.value);
                }
            }
            let mut bad = p.clone();
            bad.scalars[0] += F128::ONE;
            assert!(
                verify(
                    &c,
                    z.len().ilog2() as usize,
                    public.len().ilog2() as usize,
                    &bad,
                    &mut FsChallenger::new(b"field-gates-test")
                )
                .is_err()
            );
        }
    }

    #[test]
    fn full_width_products_match_independent_embedded_field_oracle() {
        let mut rng = flock_field::test_rng::Rng::new(0x677aee31);
        for bits in [32, 64, 128] {
            let field = SmallField::for_width(bits).unwrap();
            let high = (0..128 / bits).fold(0u128, |out, column| {
                out | (1u128 << ((column + 1) * bits - 1))
            });
            let mut a = rng.f128_vec(63);
            let mut b = rng.f128_vec(63);
            a[0] = from_raw(high);
            b[0] = from_raw(u128::MAX);
            for ((&a, &b), got) in a.iter().zip(&b).zip(product(&a, &b, bits).unwrap()) {
                for column in 0..128 / bits {
                    let extract = |word| (raw(word) >> (column * bits)) & field.mask();
                    assert_eq!(
                        field.embed(extract(got)),
                        field.embed(extract(a)) * field.embed(extract(b)),
                        "b={bits}, column={column}"
                    );
                }
            }
        }
    }
}
