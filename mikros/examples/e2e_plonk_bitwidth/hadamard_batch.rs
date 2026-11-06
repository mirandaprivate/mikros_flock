//! Actual-subfield Hadamard equations with shared row challenges and IP endpoint.
//!
//! Each group has `rows` packed F128 words and 128/b field coordinates per
//! word. Bind every source commitment before entry, then authenticate all three
//! returned claims per group. Claim row factors are low-bit-first. Sharing the
//! IP challenges does not combine or discard any group's equation.

use super::{
    Claim, aggregate, binary_claim, domain, dot, evaluate_extension, full_field_terminals, pack,
    storage_bytes, tail_table, terminal_claims, valid_width, wide_aggregate, wide_domain,
};
use flock_core::{challenger::Challenger, pcs::functional::row_weights};
use flock_field::{F128, F256Unreduced};
use mikros::{InnerProductProof, inner_product};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupProof {
    pub projected_c: F128,
    pub column_claims: Vec<F128>,
    pub quotients: Vec<F128>,
    pub inner_product: InnerProductProof,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Proof {
    Binary(Vec<mikros::hadamard::Proof>),
    Subfield(Vec<GroupProof>),
    // K128 has one column: no interpolation certificate or outside challenge.
    FullField {
        projected_c: Vec<F128>,
        inner_products: Vec<InnerProductProof>,
    },
}

impl Proof {
    pub fn field_elements(&self) -> usize {
        let ip_size = |p: &InnerProductProof| 2 * p.rounds.len() + 2;
        match self {
            Self::Binary(proofs) => proofs.iter().map(|p| p.field_elements()).sum(),
            Self::Subfield(proofs) => proofs
                .iter()
                .map(|p| 1 + p.column_claims.len() + p.quotients.len() + ip_size(&p.inner_product))
                .sum(),
            Self::FullField {
                projected_c,
                inner_products,
            } => projected_c.len() + inner_products.iter().map(ip_size).sum::<usize>(),
        }
    }
}

fn shape(rows: usize, bits: usize, groups: usize) -> Result<usize> {
    let kappa = valid_width(bits)?;
    // The binary delegate also binds log_n = log_rows + 7. All widths share
    // this checked storage bound, with no overflowing rows*128 arithmetic.
    if groups == 0 || !rows.is_power_of_two() || rows.ilog2() as usize >= usize::BITS as usize - 7 {
        return Err("shared-row Hadamard dimensions".into());
    }
    Ok(kappa)
}

fn bind(rows: usize, bits: usize, groups: usize, ch: &mut impl Challenger) {
    ch.observe_label(b"mikros-subfield-hadamard-shared-rows-v1");
    ch.observe_bytes(&(rows as u64).to_le_bytes());
    ch.observe_bytes(&(bits as u64).to_le_bytes());
    ch.observe_bytes(&(groups as u64).to_le_bytes());
}

fn queries(rows: usize, kappa: usize, groups: usize, ch: &mut impl Challenger) -> Vec<Claim> {
    let factors: Vec<_> = ch
        .sample_f128_vec(rows.ilog2() as usize)
        .into_iter()
        .map(|x| [F128::ONE + x, x])
        .collect();
    (0..groups)
        .map(|_| Claim {
            columns: ch.sample_f128_vec(kappa),
            rows: factors.clone(),
            value: F128::ZERO,
        })
        .collect()
}

fn project(words: &[F128], bits: usize, columns: &[F128], weights: &[F128]) -> F128 {
    if bits == 128 {
        return words
            .par_iter()
            .zip(weights)
            .map(|(&word, &weight)| word.mul_unreduced(weight))
            .reduce(|| F256Unreduced::ZERO, |a, b| a ^ b)
            .reduce()
            * columns[0];
    }
    let table = tail_table(bits, columns);
    words
        .par_iter()
        .zip(weights)
        .map(|(&word, &weight)| pack(&table, word).mul_unreduced(weight))
        .reduce(|| F256Unreduced::ZERO, |a, b| a ^ b)
        .reduce()
}

fn certificate(
    a: &[F128],
    b: &[F128],
    bits: usize,
    weights: &[F128],
) -> Result<(Vec<F128>, Vec<F128>)> {
    if bits <= 16 {
        let d = domain(bits)?;
        let evaluations = aggregate(a, b, weights, d);
        Ok((evaluations[..d.kappa].to_vec(), d.quotient(&evaluations)))
    } else {
        let d = wide_domain(bits)?;
        let evaluations = wide_aggregate(a, b, weights, d);
        Ok((evaluations[..d.kappa].to_vec(), d.quotient(&evaluations)))
    }
}

fn observe(p: &GroupProof, ch: &mut impl Challenger) {
    ch.observe_label(b"mikros-subfield-hadamard-batch-certificate-v1");
    ch.observe_f128(p.projected_c);
    super::observe(&p.column_claims, &p.quotients, ch);
}

fn observe_full(projected_c: &[F128], ch: &mut impl Challenger) {
    ch.observe_label(b"mikros-full-field-hadamard-batch-projections-v1");
    ch.observe_f128_slice(projected_c);
}

fn outside(bits: usize, p: &GroupProof, ch: &mut impl Challenger) -> Result<(Vec<F128>, F128)> {
    let (rho, x, vanishing) = if bits <= 16 {
        let d = domain(bits)?;
        let x = super::column_challenge(d, ch);
        (d.packing(x), x, evaluate_extension(&d.vanishing, x))
    } else {
        let d = wide_domain(bits)?;
        let x = loop {
            let x = ch.sample_f128();
            if !d.roots.contains(&x) {
                break x;
            }
        };
        (d.packing(x), x, evaluate_extension(&d.vanishing, x))
    };
    let target = dot(&p.column_claims, &rho) + vanishing * evaluate_extension(&p.quotients, x);
    Ok((rho, target))
}

fn convert_binary(claim: mikros::hadamard::Claims) -> [Claim; 3] {
    [
        binary_claim(claim.a),
        binary_claim(claim.b),
        binary_claim(claim.c),
    ]
}

/// Prove all groups' coordinatewise products. Each source slice contains the
/// same power-of-two number of packed rows; group order is transcript-bound.
pub fn prove(
    a: &[&[F128]],
    b: &[&[F128]],
    c: &[&[F128]],
    bits: usize,
    ch: &mut impl Challenger,
) -> Result<(Proof, Vec<[Claim; 3]>)> {
    let rows = a.first().map_or(0, |source| source.len());
    let kappa = shape(rows, bits, a.len())?;
    if a.len() != b.len()
        || a.len() != c.len()
        || a.iter()
            .chain(b)
            .chain(c)
            .any(|source| source.len() != rows)
    {
        return Err("shared-row Hadamard source dimensions".into());
    }
    bind(rows, bits, a.len(), ch);
    if bits == 1 {
        let bytes_a: Vec<_> = a.iter().map(|source| storage_bytes(source)).collect();
        let bytes_b: Vec<_> = b.iter().map(|source| storage_bytes(source)).collect();
        let bytes_c: Vec<_> = c.iter().map(|source| storage_bytes(source)).collect();
        let (proofs, claims) = mikros::hadamard::prove_batch_shared_rows(
            &bytes_a,
            &bytes_b,
            &bytes_c,
            rows.ilog2() as usize + 7,
            mikros::Kernel::PairAggregate,
            ch,
        )
        .map_err(|e| format!("binary shared-row Hadamard: {e}"))?;
        let claims: Vec<_> = claims.into_iter().map(convert_binary).collect();
        for (proof, claim) in proofs.iter().zip(&claims) {
            if dot(&proof.column_claims, &claim[2].columns) != proof.projected_c {
                return Err("binary shared-row Hadamard C projection".into());
            }
        }
        return Ok((Proof::Binary(proofs), claims));
    }
    let mut queries = queries(rows, kappa, a.len(), ch);
    let weights = row_weights(&queries[0].rows);
    for (query, source) in queries.iter_mut().zip(c) {
        query.value = project(source, bits, &query.columns, &weights);
    }
    if bits == 128 {
        let projected_c: Vec<_> = queries.iter().map(|query| query.value).collect();
        observe_full(&projected_c, ch);
        let mut inputs = Vec::with_capacity(a.len());
        for group in 0..a.len() {
            let scale = queries[group].columns[0];
            let weighted_b: Vec<_> = b[group]
                .par_iter()
                .zip(&weights)
                .map(|(&value, &weight)| value * weight * scale)
                .collect();
            let actual = a[group]
                .par_iter()
                .zip(&weighted_b)
                .map(|(&left, &right)| left.mul_unreduced(right))
                .reduce(|| F256Unreduced::ZERO, |a, b| a ^ b)
                .reduce();
            if actual != projected_c[group] {
                return Err("full-field shared-row Hadamard C projection".into());
            }
            inputs.push((a[group].to_vec(), weighted_b));
        }
        drop(weights);
        let (inner_products, point) = inner_product::prove_native_batch_owned(inputs, ch);
        let claims = queries
            .iter()
            .zip(&inner_products)
            .map(|(query, ip)| {
                let [a, b] = full_field_terminals(query, ip, &point);
                [a, b, query.clone()]
            })
            .collect();
        return Ok((
            Proof::FullField {
                projected_c,
                inner_products,
            },
            claims,
        ));
    }
    let mut proofs = Vec::with_capacity(a.len());
    for group in 0..a.len() {
        let (column_claims, quotients) = certificate(a[group], b[group], bits, &weights)?;
        if dot(&column_claims, &queries[group].columns) != queries[group].value {
            return Err("subfield shared-row Hadamard C projection".into());
        }
        let proof = GroupProof {
            projected_c: queries[group].value,
            column_claims,
            quotients,
            inner_product: InnerProductProof {
                rounds: vec![],
                final_a: F128::ZERO,
                final_b: F128::ZERO,
            },
        };
        observe(&proof, ch);
        proofs.push(proof);
    }
    // Every group's C projection and certificate is fixed before the first
    // outside challenge. Each group then has independent column interpolation.
    let rhos: Vec<_> = proofs
        .iter()
        .map(|proof| outside(bits, proof, ch).map(|x| x.0))
        .collect::<Result<_>>()?;
    let mut inputs = Vec::with_capacity(a.len());
    for group in 0..a.len() {
        let table = tail_table(bits, &rhos[group]);
        let mut packed_a = vec![F128::ZERO; rows];
        let mut packed_b = vec![F128::ZERO; rows];
        packed_a
            .par_iter_mut()
            .zip(&mut packed_b)
            .zip(a[group])
            .zip(b[group])
            .zip(&weights)
            .for_each(|((((out_a, out_b), &a), &b), &weight)| {
                *out_a = pack(&table, a);
                *out_b = pack(&table, b) * weight;
            });
        inputs.push((packed_a, packed_b));
    }
    drop(weights);
    let (ips, point) = inner_product::prove_native_batch_owned(inputs, ch);
    let mut claims = Vec::with_capacity(a.len());
    for (((proof, ip), rho), query) in proofs.iter_mut().zip(ips).zip(rhos).zip(queries) {
        let [a, b] = terminal_claims(&query, rho, &ip, &point);
        proof.inner_product = ip;
        claims.push([a, b, query]);
    }
    Ok((Proof::Subfield(proofs), claims))
}

/// Replay the reduction and return A/B/C source claims in group order. These
/// are deferred authentication obligations, not standalone accepted sources.
pub fn verify(
    rows: usize,
    bits: usize,
    groups: usize,
    proof: &Proof,
    ch: &mut impl Challenger,
) -> Result<Vec<[Claim; 3]>> {
    let kappa = shape(rows, bits, groups)?;
    let row_log = rows.ilog2() as usize;
    match proof {
        Proof::Binary(proofs) if bits == 1 && proofs.len() == groups => {
            bind(rows, bits, groups, ch);
            return mikros::hadamard::verify_batch_shared_rows(proofs, row_log + 7, ch)
                .map(|claims| claims.into_iter().map(convert_binary).collect())
                .map_err(|e| format!("binary shared-row Hadamard: {e}"));
        }
        Proof::Subfield(proofs) if bits > 1 && bits < 128 && proofs.len() == groups => {
            if proofs.iter().any(|p| {
                p.column_claims.len() != kappa
                    || p.quotients.len() != kappa - 1
                    || p.inner_product.rounds.len() != row_log
            }) {
                return Err("subfield shared-row Hadamard proof dimensions".into());
            }
            bind(rows, bits, groups, ch);
            let mut queries = queries(rows, kappa, groups, ch);
            for (query, p) in queries.iter_mut().zip(proofs) {
                query.value = p.projected_c;
                if dot(&p.column_claims, &query.columns) != p.projected_c {
                    return Err("subfield shared-row Hadamard C projection".into());
                }
                observe(p, ch);
            }
            let (rhos, targets): (Vec<_>, Vec<_>) = proofs
                .iter()
                .map(|p| outside(bits, p, ch))
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .unzip();
            let ips: Vec<_> = proofs.iter().map(|p| p.inner_product.clone()).collect();
            let point = inner_product::verify_batch(&ips, &targets, ch)
                .map_err(|e| format!("subfield shared-row Hadamard IP: {e}"))?;
            Ok(queries
                .into_iter()
                .zip(rhos)
                .zip(&ips)
                .map(|((query, rho), ip)| {
                    let [a, b] = terminal_claims(&query, rho, ip, &point);
                    [a, b, query]
                })
                .collect())
        }
        Proof::FullField {
            projected_c,
            inner_products,
        } if bits == 128 && projected_c.len() == groups && inner_products.len() == groups => {
            if inner_products.iter().any(|ip| ip.rounds.len() != row_log) {
                return Err("full-field shared-row Hadamard proof dimensions".into());
            }
            bind(rows, bits, groups, ch);
            let mut queries = queries(rows, kappa, groups, ch);
            for (query, &value) in queries.iter_mut().zip(projected_c) {
                query.value = value;
            }
            observe_full(projected_c, ch);
            let point = inner_product::verify_batch(inner_products, projected_c, ch)
                .map_err(|e| format!("full-field shared-row Hadamard IP: {e}"))?;
            Ok(queries
                .into_iter()
                .zip(inner_products)
                .map(|(query, ip)| {
                    let [a, b] = full_field_terminals(&query, ip, &point);
                    [a, b, query]
                })
                .collect())
        }
        _ => Err("shared-row Hadamard proof variant or group count".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::challenger::FsChallenger;
    use flock_field::test_rng::Rng;
    use mikros_permutation::field::{SmallField, from_raw, raw};

    fn challenger() -> FsChallenger {
        FsChallenger::new(b"subfield-shared-row-batch-tests")
    }

    fn inputs(bits: usize, rows: usize, groups: usize, pattern: usize) -> Vec<[Vec<F128>; 3]> {
        let mut rng = Rng::new(0x4567_9812 ^ bits as u64 ^ rows as u64);
        let field = SmallField::for_width(bits).unwrap();
        (0..groups)
            .map(|group| {
                let mut a = rng.f128_vec(rows);
                let mut b = rng.f128_vec(rows);
                for row in 0..rows {
                    if pattern == 1 || (pattern == 2 && row + 1 == rows) {
                        a[row] = F128::ZERO;
                        b[row] = F128::ZERO;
                    } else if pattern == 2 {
                        a[row] = from_raw(1u128 << ((row + group) % 128));
                        b[row] = from_raw(u128::MAX);
                    }
                }
                let c = a
                    .iter()
                    .zip(&b)
                    .map(|(&a, &b)| {
                        let word = (0..128 / bits).fold(0u128, |word, column| {
                            let shift = column * bits;
                            let left = (raw(a) >> shift) & field.mask();
                            let right = (raw(b) >> shift) & field.mask();
                            word | (field.multiply(left, right) << shift)
                        });
                        from_raw(word)
                    })
                    .collect();
                [a, b, c]
            })
            .collect()
    }

    fn sources(data: &[[Vec<F128>; 3]]) -> [Vec<&[F128]>; 3] {
        std::array::from_fn(|source| data.iter().map(|group| group[source].as_slice()).collect())
    }

    // Independent coordinate expansion: no packing tables, aggregate kernels,
    // prover row tensor helper, or interpolation polynomials are used here.
    fn oracle(words: &[F128], bits: usize, claim: &Claim) -> F128 {
        let field = SmallField::for_width(bits).unwrap();
        words
            .iter()
            .enumerate()
            .fold(F128::ZERO, |sum, (row, &word)| {
                let weight = claim
                    .rows
                    .iter()
                    .enumerate()
                    .fold(F128::ONE, |weight, (bit, pair)| {
                        weight * pair[(row >> bit) & 1]
                    });
                let value =
                    claim
                        .columns
                        .iter()
                        .enumerate()
                        .fold(F128::ZERO, |sum, (column, &weight)| {
                            let value = (raw(word) >> (bits * column)) & field.mask();
                            sum + weight * field.embed(value)
                        });
                sum + weight * value
            })
    }

    fn authenticate(data: &[[Vec<F128>; 3]], bits: usize, claims: &[[Claim; 3]]) -> bool {
        claims.len() == data.len()
            && data.iter().zip(claims).all(|(group, claims)| {
                group
                    .iter()
                    .zip(claims)
                    .all(|(words, claim)| oracle(words, bits, claim) == claim.value)
            })
    }

    #[test]
    fn shared_subfield_batch_roundtrip_source_oracles_padding_and_transcript() {
        for bits in [1, 2, 4, 8, 16, 32, 64, 128] {
            for (rows, groups, pattern) in [(1, 1, 0), (2, 2, 1), (8, 3, 0), (8, 3, 2)] {
                let data = inputs(bits, rows, groups, pattern);
                let [a, b, c] = sources(&data);
                let mut pc = challenger();
                let (proof, claims) = prove(&a, &b, &c, bits, &mut pc).unwrap();
                let encoded = bincode::serialize(&proof).unwrap();
                let decoded = bincode::deserialize(&encoded).unwrap();
                let mut vc = challenger();
                assert_eq!(
                    verify(rows, bits, groups, &decoded, &mut vc).unwrap(),
                    claims
                );
                assert_eq!(pc.sample_f128(), vc.sample_f128());
                assert!(
                    authenticate(&data, bits, &claims),
                    "bits={bits},rows={rows},pattern={pattern}"
                );
                for claim in claims.iter().skip(1) {
                    for source in 0..3 {
                        assert_eq!(claim[source].rows, claims[0][source].rows);
                    }
                    assert_ne!(claim[2].columns, claims[0][2].columns);
                    if bits != 128 {
                        assert_ne!(claim[0].columns, claims[0][0].columns);
                    }
                }
                assert!(proof.field_elements() > 0);
            }
        }
    }

    #[test]
    fn shared_subfield_batch_rejects_tampering_wrong_sources_and_dimensions() {
        for bits in [1, 2, 4, 8, 16, 32, 64, 128] {
            let data = inputs(bits, 8, 2, 0);
            let [a, b, c] = sources(&data);
            let (proof, claims) = prove(&a, &b, &c, bits, &mut challenger()).unwrap();
            for mutation in 0..7 {
                let mut bad = proof.clone();
                let ip = match &mut bad {
                    Proof::Binary(p) => {
                        match mutation {
                            0 => p[1].projected_c += F128::ONE,
                            1 => p[1].column_claims[0] += F128::ONE,
                            2 => p[1].quotients[0] += F128::ONE,
                            _ => {}
                        }
                        &mut p[1].inner_product
                    }
                    Proof::Subfield(p) => {
                        match mutation {
                            0 => p[1].projected_c += F128::ONE,
                            1 => p[1].column_claims[0] += F128::ONE,
                            2 => p[1].quotients[0] += F128::ONE,
                            _ => {}
                        }
                        &mut p[1].inner_product
                    }
                    Proof::FullField {
                        projected_c,
                        inner_products,
                    } => {
                        if mutation < 3 {
                            projected_c[1] += F128::ONE;
                        }
                        &mut inner_products[1]
                    }
                };
                match mutation {
                    3 => ip.final_a += F128::ONE,
                    4 => ip.final_b += F128::ONE,
                    5 => {
                        ip.rounds.pop();
                    }
                    6 => ip.rounds[0][0] += F128::ONE,
                    _ => {}
                }
                let result = verify(8, bits, 2, &bad, &mut challenger());
                assert!(
                    result.is_err() || !authenticate(&data, bits, &result.unwrap()),
                    "bits={bits}, mutation={mutation}"
                );
            }
            for source in 0..3 {
                let mut changed = data.clone();
                changed[1][source][3] += F128::ONE;
                assert!(!authenticate(&changed, bits, &claims));
            }
            let mut changed = data.clone();
            changed[1][2][0] += F128::ONE;
            let [ca, cb, cc] = sources(&changed);
            assert!(prove(&ca, &cb, &cc, bits, &mut challenger()).is_err());
            assert!(verify(8, bits, 1, &proof, &mut challenger()).is_err());
            assert!(verify(16, bits, 2, &proof, &mut challenger()).is_err());
            assert!(verify(8, 3, 2, &proof, &mut challenger()).is_err());
            assert!(verify(0, bits, 2, &proof, &mut challenger()).is_err());
            assert!(verify(8, bits, 0, &proof, &mut challenger()).is_err());
            assert!(prove(&a[..1], &b, &c, bits, &mut challenger()).is_err());
            assert!(prove(&[&a[0][..7]], &[b[0]], &[c[0]], bits, &mut challenger()).is_err());
        }
    }

    struct Trace {
        inner: FsChallenger,
        forced: std::collections::VecDeque<F128>,
        events: Vec<Vec<u8>>,
    }
    impl Trace {
        fn new(forced: Vec<F128>) -> Self {
            Self {
                inner: challenger(),
                forced: forced.into(),
                events: vec![],
            }
        }
    }
    impl Challenger for Trace {
        fn observe_label(&mut self, label: &[u8]) {
            self.events.push(label.to_vec());
            self.inner.observe_label(label);
        }
        fn observe_bytes(&mut self, bytes: &[u8]) {
            self.inner.observe_bytes(bytes);
        }
        fn observe_f128(&mut self, value: F128) {
            self.inner.observe_f128(value);
        }
        fn observe_f128_slice(&mut self, values: &[F128]) {
            self.inner.observe_f128_slice(values);
        }
        fn sample_f128(&mut self) -> F128 {
            self.events.push(b"sample".to_vec());
            let value = self.inner.sample_f128();
            self.forced.pop_front().unwrap_or(value)
        }
        fn fork_from_seed(&self, seed: [F128; 2], label: &'static [u8]) -> Self {
            Self {
                inner: self.inner.fork_from_seed(seed, label),
                forced: self.forced.clone(),
                events: vec![],
            }
        }
    }

    #[test]
    fn shared_subfield_batch_boolean_rows_zero_columns_and_certificate_order() {
        for bits in [2, 16, 32, 64, 128] {
            for zero_columns in [false, true] {
                let data = inputs(bits, 8, 3, 0);
                let [a, b, c] = sources(&data);
                let mut forced = vec![F128::ZERO, F128::ONE, F128::ZERO];
                if zero_columns {
                    forced.extend(vec![F128::ZERO; 3 * (128 / bits)]);
                }
                let mut pc = Trace::new(forced.clone());
                let (proof, claims) = prove(&a, &b, &c, bits, &mut pc).unwrap();
                assert!(authenticate(&data, bits, &claims));
                let mut vc = Trace::new(forced);
                assert_eq!(verify(8, bits, 3, &proof, &mut vc).unwrap(), claims);
                assert_eq!(pc.events, vc.events);
                assert_eq!(pc.sample_f128(), vc.sample_f128());
                assert_eq!(
                    claims[0][2].rows,
                    [
                        [F128::ONE, F128::ZERO],
                        [F128::ZERO, F128::ONE],
                        [F128::ONE, F128::ZERO]
                    ]
                );
                if bits != 128 {
                    let label = b"mikros-subfield-hadamard-batch-certificate-v1";
                    let first = pc.events.iter().position(|event| event == label).unwrap();
                    let next_sample = first
                        + pc.events[first..]
                            .iter()
                            .position(|event| event == b"sample")
                            .unwrap();
                    assert_eq!(
                        pc.events[first..next_sample]
                            .iter()
                            .filter(|event| event.as_slice() == label)
                            .count(),
                        3
                    );
                }
            }
        }
    }

    #[test]
    fn shared_subfield_batch_worker_counts_preserve_bytes_and_claims() {
        for bits in [1, 4, 32, 128] {
            let data = inputs(bits, 64, 3, 2);
            let [a, b, c] = sources(&data);
            let outputs: Vec<_> = [1, 3]
                .into_iter()
                .map(|threads| {
                    rayon::ThreadPoolBuilder::new()
                        .num_threads(threads)
                        .build()
                        .unwrap()
                        .install(|| {
                            let mut ch = challenger();
                            let (proof, claims) = prove(&a, &b, &c, bits, &mut ch).unwrap();
                            (
                                bincode::serialize(&proof).unwrap(),
                                claims,
                                ch.sample_f128(),
                            )
                        })
                })
                .collect();
            assert_eq!(outputs[0], outputs[1]);
        }
    }
}
