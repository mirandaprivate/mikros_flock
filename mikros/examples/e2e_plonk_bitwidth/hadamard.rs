//! Weighted Hadamard reduction for actual F_(2^b) subfields of GHASH F128.
//!
//! One storage word contains 128/b consecutive polynomial-basis coordinates.
//! Claims use arbitrary column weights and low-bit-first tensor row factors.
//! Source commitments must be bound before entry and both returned claims must
//! be authenticated by the caller. No normalization of row factors is used.

use flock_core::{challenger::Challenger, pcs::functional::row_weights};
use flock_field::{F128, F256Unreduced};
use mikros::{InnerProductProof, inner_product, weighted_hadamard};
use mikros_permutation::field::{FastField, SmallField, raw};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claim {
    pub columns: Vec<F128>,
    pub rows: Vec<[F128; 2]>,
    pub value: F128,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Proof {
    Binary(weighted_hadamard::Proof),
    Subfield {
        column_claims: Vec<F128>,
        quotients: Vec<F128>,
        inner_product: InnerProductProof,
    },
    // Appended to preserve the serialized variants of the existing widths.
    FullField(InnerProductProof),
}

impl Proof {
    pub fn field_elements(&self) -> usize {
        match self {
            Self::Binary(p) => p.field_elements(),
            Self::Subfield {
                column_claims,
                quotients,
                inner_product,
            } => column_claims.len() + quotients.len() + 2 * inner_product.rounds.len() + 2,
            Self::FullField(p) => 2 * p.rounds.len() + 2,
        }
    }
}

type Result<T> = std::result::Result<T, String>;
type SmallTable = [[u16; 256]; 16];
const BLOCK_ROWS: usize = 4096;

fn valid_width(bits: usize) -> Result<usize> {
    if ![1, 2, 4, 8, 16, 32, 64, 128].contains(&bits) {
        return Err("weighted Hadamard requires a power-of-two field width from 1 to 128".into());
    }
    Ok(128 / bits)
}

fn shape(bits: usize, query: &Claim) -> Result<usize> {
    let kappa = valid_width(bits)?;
    if query.columns.len() != kappa || query.rows.len() >= usize::BITS as usize - 7 {
        return Err("weighted Hadamard query dimensions".into());
    }
    Ok(1usize << query.rows.len())
}

fn polynomial_product(a: &[u16], b: &[u16], field: &FastField) -> Vec<u16> {
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out[i + j] ^= field.multiply(x as u128, y as u128) as u16;
        }
    }
    out
}

fn inverse(value: u16, field: &FastField) -> u16 {
    assert_ne!(value, 0);
    let mut exponent = (1u128 << field.base.bits) - 2;
    let mut x = value as u128;
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = field.multiply(result, x);
        }
        x = field.multiply(x, x);
        exponent >>= 1;
    }
    result as u16
}

fn lagrange_basis(nodes: &[u16], field: &FastField) -> Vec<Vec<u16>> {
    nodes
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            let mut numerator = vec![1];
            let mut denominator = 1u16;
            for (j, &y) in nodes.iter().enumerate() {
                if i != j {
                    numerator = polynomial_product(&numerator, &[y, 1], field);
                    denominator = field.multiply(denominator as u128, (x ^ y) as u128) as u16;
                }
            }
            let inverse = inverse(denominator, field);
            numerator
                .into_iter()
                .map(|coefficient| field.multiply(coefficient as u128, inverse as u128) as u16)
                .collect()
        })
        .collect()
}

fn evaluate_small(coefficients: &[u16], x: u16, field: &FastField) -> u16 {
    coefficients.iter().rev().fold(0, |value, &coefficient| {
        field.multiply(value as u128, x as u128) as u16 ^ coefficient
    })
}

fn evaluate_extension(coefficients: &[F128], x: F128) -> F128 {
    coefficients
        .iter()
        .rev()
        .fold(F128::ZERO, |value, &coefficient| value * x + coefficient)
}

struct Domain {
    bits: usize,
    kappa: usize,
    field: &'static FastField,
    lagrange: Vec<Vec<F128>>,
    off_weights: Vec<Vec<F128>>,
    quotient_map: Vec<Vec<F128>>,
    vanishing: Vec<F128>,
    roots: Vec<F128>,
    interpolation: Vec<Box<SmallTable>>,
    // GF256 only. GF(2^16) uses carryless multiplication and reduction tables.
    products: Option<Vec<u16>>,
    root_products: Option<Vec<u16>>,
}

impl Domain {
    fn new(bits: usize) -> Self {
        let base = SmallField::for_width(bits).unwrap();
        let field = FastField::for_width(bits.max(8)).unwrap();
        let kappa = 128 / bits;
        let base_map = if bits == field.base.bits {
            None
        } else {
            let coordinates: std::collections::HashMap<_, _> = (0..256u128)
                .map(|value| (raw(field.embed(value)), value as u16))
                .collect();
            Some(
                (0..1u128 << bits)
                    .map(|value| {
                        *coordinates
                            .get(&raw(base.embed(value)))
                            .expect("actual subfield inclusion")
                    })
                    .collect::<Vec<_>>(),
            )
        };
        let lift = |value: usize| base_map.as_ref().map_or(value as u16, |map| map[value]);
        let nodes: Vec<_> = (0..kappa as u16).collect();
        let off: Vec<_> = (kappa as u16..(2 * kappa - 1) as u16).collect();
        let lagrange_raw = lagrange_basis(&nodes, field);
        let off_raw: Vec<Vec<_>> = off
            .iter()
            .map(|&x| {
                lagrange_raw
                    .iter()
                    .map(|polynomial| evaluate_small(polynomial, x, field))
                    .collect()
            })
            .collect();
        let vanishing_raw = nodes
            .iter()
            .fold(vec![1], |p, &x| polynomial_product(&p, &[x, 1], field));
        let quotient_raw: Vec<Vec<_>> = lagrange_basis(&off, field)
            .into_iter()
            .zip(&off)
            .map(|(p, &x)| {
                let scale = inverse(evaluate_small(&vanishing_raw, x, field), field);
                p.into_iter()
                    .map(|coefficient| field.multiply(coefficient as u128, scale as u128) as u16)
                    .collect()
            })
            .collect();
        let interpolation = off_raw
            .iter()
            .map(|weights| {
                let mut table = Box::new([[0u16; 256]; 16]);
                for (byte, entries) in table.iter_mut().enumerate() {
                    let contributions: [u16; 8] = std::array::from_fn(|bit| {
                        let index = byte * 8 + bit;
                        field.multiply(
                            weights[index / bits] as u128,
                            lift(1usize << (index % bits)) as u128,
                        ) as u16
                    });
                    for value in 1usize..256 {
                        entries[value] = entries[value & (value - 1)]
                            ^ contributions[value.trailing_zeros() as usize];
                    }
                }
                table
            })
            .collect();
        let products = (field.base.bits == 8).then(|| {
            (0..65536usize)
                .map(|index| field.multiply((index >> 8) as u128, (index & 255) as u128) as u16)
                .collect::<Vec<_>>()
        });
        let root_products = (bits < 8).then(|| {
            let mask = (1usize << bits) - 1;
            (0..1usize << (2 * bits))
                .map(|index| {
                    field.multiply(lift(index >> bits) as u128, lift(index & mask) as u128) as u16
                })
                .collect()
        });
        let embed = |p: Vec<u16>| p.into_iter().map(|x| field.embed(x as u128)).collect();
        Self {
            bits,
            kappa,
            field,
            lagrange: lagrange_raw.into_iter().map(embed).collect(),
            off_weights: off_raw.into_iter().map(embed).collect(),
            quotient_map: quotient_raw.into_iter().map(embed).collect(),
            vanishing: embed(vanishing_raw),
            roots: nodes.into_iter().map(|x| field.embed(x as u128)).collect(),
            interpolation,
            products,
            root_products,
        }
    }

    #[inline]
    fn multiply(&self, a: u16, b: u16) -> u16 {
        match &self.products {
            Some(table) => table[((a as usize) << 8) | b as usize],
            None => self.field.multiply(a as u128, b as u128) as u16,
        }
    }

    #[inline]
    fn root_product(&self, a: F128, b: F128, column: usize) -> u16 {
        let shift = column * self.bits;
        let mask = (1u128 << self.bits) - 1;
        let a = ((raw(a) >> shift) & mask) as u16;
        let b = ((raw(b) >> shift) & mask) as u16;
        match &self.root_products {
            Some(table) => table[((a as usize) << self.bits) | b as usize],
            None => self.multiply(a, b),
        }
    }

    fn packing(&self, x: F128) -> Vec<F128> {
        self.lagrange
            .iter()
            .map(|p| evaluate_extension(p, x))
            .collect()
    }

    fn quotient(&self, evaluations: &[F128]) -> Vec<F128> {
        let mut out = vec![F128::ZERO; self.kappa - 1];
        for (j, weights) in self.off_weights.iter().enumerate() {
            let residual = evaluations[self.kappa + j] + dot(&evaluations[..self.kappa], weights);
            for (coefficient, &weight) in out.iter_mut().zip(&self.quotient_map[j]) {
                *coefficient += residual * weight;
            }
        }
        out
    }
}

fn domain(bits: usize) -> Result<&'static Domain> {
    if ![2, 4, 8, 16].contains(&bits) {
        return Err("small Hadamard interpolation width".into());
    }
    static DOMAINS: [OnceLock<Domain>; 5] = [const { OnceLock::new() }; 5];
    Ok(DOMAINS[bits.ilog2() as usize].get_or_init(|| Domain::new(bits)))
}

/// Initialize only parameter-dependent arithmetic/interpolation tables offline.
pub fn warmup(value_bits: usize) -> Result<()> {
    valid_width(value_bits)?;
    if value_bits == 1 {
        static BINARY: OnceLock<Result<()>> = OnceLock::new();
        BINARY
            .get_or_init(|| {
                let mut ch =
                    flock_core::challenger::FsChallenger::new(b"plonk-bitwidth-parameter-warmup");
                let zero = vec![F128::ZERO; 64];
                let q = Claim {
                    columns: vec![F128::ONE; 128],
                    rows: vec![[F128::ONE; 2]; 6],
                    value: F128::ZERO,
                };
                prove(&zero, &zero, 1, &q, &mut ch).map(|_| ())
            })
            .clone()
    } else if value_bits <= 16 {
        domain(value_bits).map(|_| ())
    } else if value_bits < 128 {
        wide_domain(value_bits).map(|_| ())
    } else {
        SmallField::for_width(128)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}

#[inline]
fn small_lookup(table: &SmallTable, value: F128) -> u16 {
    let mut lo = value.lo;
    let mut hi = value.hi;
    let mut out = 0;
    for byte in 0..8 {
        out ^= table[byte][(lo & 255) as usize] ^ table[byte + 8][(hi & 255) as usize];
        lo >>= 8;
        hi >>= 8;
    }
    out
}

fn collapse(buckets: &[F128; 512], domain: &Domain) -> F128 {
    let mut result = F128::ZERO;
    for byte in 0..domain.field.base.bits / 8 {
        let mut coordinates = [F128::ZERO; 8];
        for (value, &weight) in buckets[byte * 256..(byte + 1) * 256]
            .iter()
            .enumerate()
            .skip(1)
        {
            let mut bits = value;
            while bits != 0 {
                coordinates[bits.trailing_zeros() as usize] += weight;
                bits &= bits - 1;
            }
        }
        for (bit, coordinate) in coordinates.into_iter().enumerate() {
            result += coordinate * domain.field.base.basis()[byte * 8 + bit];
        }
    }
    result
}

fn aggregate(a: &[F128], b: &[F128], weights: &[F128], domain: &Domain) -> Vec<F128> {
    let samples = 2 * domain.kappa - 1;
    let blocks = a.len().div_ceil(BLOCK_ROWS);
    let mut partials = vec![F128::ZERO; blocks * samples];
    partials
        .par_iter_mut()
        .enumerate()
        .for_each(|(job, output)| {
            let sample = job % samples;
            let start = (job / samples) * BLOCK_ROWS;
            let end = (start + BLOCK_ROWS).min(a.len());
            let mut buckets = [F128::ZERO; 512];
            let mut add = |value: u16, weight: F128| {
                buckets[(value & 255) as usize] += weight;
                if domain.field.base.bits == 16 {
                    buckets[256 + (value >> 8) as usize] += weight;
                }
            };
            if sample < domain.kappa {
                for i in start..end {
                    add(domain.root_product(a[i], b[i], sample), weights[i]);
                }
            } else {
                let table = &domain.interpolation[sample - domain.kappa];
                for i in start..end {
                    add(
                        domain.multiply(small_lookup(table, a[i]), small_lookup(table, b[i])),
                        weights[i],
                    );
                }
            }
            *output = collapse(&buckets, domain);
        });
    let mut evaluations = vec![F128::ZERO; samples];
    for block in partials.chunks_exact(samples) {
        for (value, &part) in evaluations.iter_mut().zip(block) {
            *value += part;
        }
    }
    evaluations
}

// These widths use the same interpolation reduction, with full u64 field
// coordinates. The independent implementation leaves the <=16-bit path intact.
type WideTable = [[u64; 256]; 16];

struct WideDomain {
    field: &'static FastField,
    kappa: usize,
    lagrange: Vec<Vec<F128>>,
    off_weights: Vec<Vec<F128>>,
    quotient_map: Vec<Vec<F128>>,
    vanishing: Vec<F128>,
    roots: Vec<F128>,
    interpolation: Vec<Box<WideTable>>,
}

fn wide_polynomial_product(a: &[u64], b: &[u64], field: &FastField) -> Vec<u64> {
    let mut out = vec![0; a.len() + b.len() - 1];
    for (i, &x) in a.iter().enumerate() {
        for (j, &y) in b.iter().enumerate() {
            out[i + j] ^= field.multiply(x as u128, y as u128) as u64;
        }
    }
    out
}

fn wide_inverse(value: u64, field: &FastField) -> u64 {
    assert_ne!(value, 0);
    let mut exponent = (1u128 << field.base.bits) - 2;
    let mut x = value as u128;
    let mut result = 1;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = field.multiply(result, x);
        }
        x = field.multiply(x, x);
        exponent >>= 1;
    }
    result as u64
}

fn wide_lagrange(nodes: &[u64], field: &FastField) -> Vec<Vec<u64>> {
    nodes
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            let mut numerator = vec![1];
            let mut denominator = 1u64;
            for (j, &y) in nodes.iter().enumerate() {
                if i != j {
                    numerator = wide_polynomial_product(&numerator, &[y, 1], field);
                    denominator = field.multiply(denominator as u128, (x ^ y) as u128) as u64;
                }
            }
            let inverse = wide_inverse(denominator, field);
            numerator
                .into_iter()
                .map(|x| field.multiply(x as u128, inverse as u128) as u64)
                .collect()
        })
        .collect()
}

fn wide_evaluate(coefficients: &[u64], x: u64, field: &FastField) -> u64 {
    coefficients.iter().rev().fold(0, |value, &coefficient| {
        field.multiply(value as u128, x as u128) as u64 ^ coefficient
    })
}

impl WideDomain {
    fn new(bits: usize) -> Self {
        assert!([32, 64].contains(&bits));
        let field = FastField::for_width(bits).unwrap();
        let kappa = 128 / bits;
        let nodes: Vec<_> = (0..kappa as u64).collect();
        let off: Vec<_> = (kappa as u64..(2 * kappa - 1) as u64).collect();
        let lagrange = wide_lagrange(&nodes, field);
        let off_weights: Vec<Vec<_>> = off
            .iter()
            .map(|&x| {
                lagrange
                    .iter()
                    .map(|p| wide_evaluate(p, x, field))
                    .collect()
            })
            .collect();
        let vanishing = nodes
            .iter()
            .fold(vec![1], |p, &x| wide_polynomial_product(&p, &[x, 1], field));
        let quotient_map = wide_lagrange(&off, field)
            .into_iter()
            .zip(&off)
            .map(|(p, &x)| {
                let scale = wide_inverse(wide_evaluate(&vanishing, x, field), field);
                p.into_iter()
                    .map(|v| field.multiply(v as u128, scale as u128) as u64)
                    .collect()
            })
            .collect::<Vec<Vec<_>>>();
        let interpolation = off_weights
            .iter()
            .map(|weights| {
                let mut table = Box::new([[0u64; 256]; 16]);
                for (byte, entries) in table.iter_mut().enumerate() {
                    let contributions: [u64; 8] = std::array::from_fn(|bit| {
                        let index = byte * 8 + bit;
                        field.multiply(weights[index / bits] as u128, 1u128 << (index % bits))
                            as u64
                    });
                    for value in 1usize..256 {
                        entries[value] = entries[value & (value - 1)]
                            ^ contributions[value.trailing_zeros() as usize];
                    }
                }
                table
            })
            .collect();
        let embed = |p: Vec<u64>| p.into_iter().map(|v| field.embed(v as u128)).collect();
        Self {
            field,
            kappa,
            lagrange: lagrange.into_iter().map(embed).collect(),
            off_weights: off_weights.into_iter().map(embed).collect(),
            quotient_map: quotient_map.into_iter().map(embed).collect(),
            vanishing: embed(vanishing),
            roots: nodes.into_iter().map(|v| field.embed(v as u128)).collect(),
            interpolation,
        }
    }

    fn packing(&self, x: F128) -> Vec<F128> {
        self.lagrange
            .iter()
            .map(|p| evaluate_extension(p, x))
            .collect()
    }

    fn quotient(&self, evaluations: &[F128]) -> Vec<F128> {
        let mut out = vec![F128::ZERO; self.kappa - 1];
        for (j, weights) in self.off_weights.iter().enumerate() {
            let residual = evaluations[self.kappa + j] + dot(&evaluations[..self.kappa], weights);
            for (coefficient, &weight) in out.iter_mut().zip(&self.quotient_map[j]) {
                *coefficient += residual * weight;
            }
        }
        out
    }
}

fn wide_domain(bits: usize) -> Result<&'static WideDomain> {
    if ![32, 64].contains(&bits) {
        return Err("wide Hadamard interpolation width".into());
    }
    static DOMAINS: [OnceLock<WideDomain>; 2] = [const { OnceLock::new() }; 2];
    Ok(DOMAINS[bits.ilog2() as usize - 5].get_or_init(|| WideDomain::new(bits)))
}

#[inline]
fn wide_lookup(table: &WideTable, value: F128) -> u64 {
    let mut lo = value.lo;
    let mut hi = value.hi;
    let mut out = 0;
    for byte in 0..8 {
        out ^= table[byte][(lo & 255) as usize] ^ table[byte + 8][(hi & 255) as usize];
        lo >>= 8;
        hi >>= 8;
    }
    out
}

fn wide_aggregate(a: &[F128], b: &[F128], weights: &[F128], d: &WideDomain) -> Vec<F128> {
    let samples = 2 * d.kappa - 1;
    let bits = d.field.base.bits;
    let byte_count = bits / 8;
    let mut partials = vec![F128::ZERO; a.len().div_ceil(BLOCK_ROWS) * samples];
    partials
        .par_iter_mut()
        .enumerate()
        .for_each(|(job, output)| {
            let sample = job % samples;
            let start = (job / samples) * BLOCK_ROWS;
            let end = (start + BLOCK_ROWS).min(a.len());
            let mut buckets = [[F128::ZERO; 256]; 8];
            let mut add = |mut value: u64, weight: F128| {
                for bucket in &mut buckets[..byte_count] {
                    bucket[(value & 255) as usize] += weight;
                    value >>= 8;
                }
            };
            if sample < d.kappa {
                let shift = sample * bits;
                let mask = d.field.base.mask();
                for i in start..end {
                    let left = (raw(a[i]) >> shift) & mask;
                    let right = (raw(b[i]) >> shift) & mask;
                    add(d.field.multiply(left, right) as u64, weights[i]);
                }
            } else {
                let table = &d.interpolation[sample - d.kappa];
                for i in start..end {
                    add(
                        d.field.multiply(
                            wide_lookup(table, a[i]) as u128,
                            wide_lookup(table, b[i]) as u128,
                        ) as u64,
                        weights[i],
                    );
                }
            }
            let mut result = F256Unreduced::ZERO;
            for (byte, bucket) in buckets[..byte_count].iter().enumerate() {
                let mut coordinates = [F128::ZERO; 8];
                for (value, &weight) in bucket.iter().enumerate().skip(1) {
                    let mut active = value;
                    while active != 0 {
                        coordinates[active.trailing_zeros() as usize] += weight;
                        active &= active - 1;
                    }
                }
                for (bit, coordinate) in coordinates.into_iter().enumerate() {
                    result ^= coordinate.mul_unreduced(d.field.base.basis()[byte * 8 + bit]);
                }
            }
            *output = result.reduce();
        });
    let mut evaluations = vec![F128::ZERO; samples];
    for block in partials.chunks_exact(samples) {
        for (value, &part) in evaluations.iter_mut().zip(block) {
            *value += part;
        }
    }
    evaluations
}

fn tail_table(bits: usize, rho: &[F128]) -> Box<[[F128; 256]; 16]> {
    let field = SmallField::for_width(bits).unwrap();
    let mut table = Box::new([[F128::ZERO; 256]; 16]);
    for (byte, entries) in table.iter_mut().enumerate() {
        let contributions: [F128; 8] = std::array::from_fn(|bit| {
            let index = byte * 8 + bit;
            rho[index / bits] * field.basis()[index % bits]
        });
        for value in 1usize..256 {
            entries[value] =
                entries[value & (value - 1)] + contributions[value.trailing_zeros() as usize];
        }
    }
    table
}

#[inline]
fn pack(table: &[[F128; 256]; 16], value: F128) -> F128 {
    let mut lo = value.lo;
    let mut hi = value.hi;
    let mut out = F128::ZERO;
    for byte in 0..8 {
        out += table[byte][(lo & 255) as usize] + table[byte + 8][(hi & 255) as usize];
        lo >>= 8;
        hi >>= 8;
    }
    out
}

fn dot(a: &[F128], b: &[F128]) -> F128 {
    a.iter()
        .zip(b)
        .fold(F128::ZERO, |sum, (&x, &y)| sum + x * y)
}

fn bind(bits: usize, query: &Claim, ch: &mut impl Challenger) {
    ch.observe_label(b"mikros-subfield-weighted-hadamard-v1");
    ch.observe_bytes(&(bits as u64).to_le_bytes());
    ch.observe_bytes(&(query.rows.len() as u64).to_le_bytes());
    for pair in &query.rows {
        ch.observe_f128_slice(pair);
    }
    ch.observe_f128_slice(&query.columns);
    ch.observe_f128(query.value);
}

fn observe(columns: &[F128], quotients: &[F128], ch: &mut impl Challenger) {
    ch.observe_label(b"mikros-subfield-column-and-quotient-v1");
    ch.observe_f128_slice(columns);
    ch.observe_f128_slice(quotients);
}

fn column_challenge(domain: &Domain, ch: &mut impl Challenger) -> F128 {
    loop {
        let x = ch.sample_f128();
        if !domain.roots.contains(&x) {
            return x;
        }
    }
}

fn terminal_claims(
    query: &Claim,
    rho: Vec<F128>,
    ip: &InnerProductProof,
    point: &[F128],
) -> [Claim; 2] {
    let rows: Vec<_> = point.iter().rev().map(|&x| [F128::ONE + x, x]).collect();
    let weighted = rows
        .iter()
        .zip(&query.rows)
        .map(|(x, l)| [x[0] * l[0], x[1] * l[1]])
        .collect();
    [
        Claim {
            columns: rho.clone(),
            rows,
            value: ip.final_a,
        },
        Claim {
            columns: rho,
            rows: weighted,
            value: ip.final_b,
        },
    ]
}

fn binary_claim(claim: mikros::MatrixClaim) -> Claim {
    Claim {
        columns: claim.column_weights,
        rows: claim.row_factors.into_iter().rev().collect(),
        value: claim.value,
    }
}

fn storage_bytes(words: &[F128]) -> &[u8] {
    assert!(cfg!(target_endian = "little"));
    // SAFETY: F128 is repr(C), consists of two u64 values, and is fully initialized.
    unsafe { std::slice::from_raw_parts(words.as_ptr().cast(), std::mem::size_of_val(words)) }
}

fn full_field_terminals(query: &Claim, ip: &InnerProductProof, point: &[F128]) -> [Claim; 2] {
    let mut claims = terminal_claims(query, vec![F128::ONE], ip, point);
    // Only the right input absorbs the original column scalar. Putting this
    // scalar in both terminals would incorrectly square it; division would
    // lose the valid zero-scalar case.
    claims[1].columns = query.columns.clone();
    claims
}

fn prove_full_field(
    a: &[F128],
    b: &[F128],
    query: &Claim,
    ch: &mut impl Challenger,
) -> Result<(Proof, [Claim; 2])> {
    bind(128, query, ch);
    let scale = query.columns[0];
    let mut factors = query.rows.clone();
    let mut weighted_b = if let Some(first) = factors.first_mut() {
        first[0] *= scale;
        first[1] *= scale;
        row_weights(&factors)
    } else {
        vec![scale]
    };
    // Fuse target validation with preparation; unreduced accumulation avoids
    // an extra complete input scan and one reduction per product.
    let actual = weighted_b
        .par_iter_mut()
        .zip(a)
        .zip(b)
        .map(|((out, &a), &b)| {
            *out *= b;
            a.mul_unreduced(*out)
        })
        .reduce(|| F256Unreduced::ZERO, |a, b| a ^ b)
        .reduce();
    if actual != query.value {
        return Err("full-field Hadamard target".into());
    }
    let (ip, point) = inner_product::prove_native_owned(a.to_vec(), weighted_b, ch);
    let claims = full_field_terminals(query, &ip, &point);
    Ok((Proof::FullField(ip), claims))
}

fn prove_wide(
    a: &[F128],
    b: &[F128],
    bits: usize,
    query: &Claim,
    ch: &mut impl Challenger,
) -> Result<(Proof, [Claim; 2])> {
    let d = wide_domain(bits)?;
    bind(bits, query, ch);
    let weights = row_weights(&query.rows);
    let evaluations = wide_aggregate(a, b, &weights, d);
    let column_claims = evaluations[..d.kappa].to_vec();
    if dot(&column_claims, &query.columns) != query.value {
        return Err("wide Hadamard column projection".into());
    }
    let quotients = d.quotient(&evaluations);
    observe(&column_claims, &quotients, ch);
    let x = loop {
        let x = ch.sample_f128();
        if !d.roots.contains(&x) {
            break x;
        }
    };
    let rho = d.packing(x);
    let table = tail_table(bits, &rho);
    let mut packed_a = vec![F128::ZERO; a.len()];
    let mut packed_b = weights;
    packed_a
        .par_iter_mut()
        .zip(&mut packed_b)
        .zip(a)
        .zip(b)
        .for_each(|(((out_a, out_b), &a), &b)| {
            *out_a = pack(&table, a);
            *out_b *= pack(&table, b);
        });
    drop(table);
    let (inner_product, point) = inner_product::prove_native_owned(packed_a, packed_b, ch);
    let claims = terminal_claims(query, rho, &inner_product, &point);
    Ok((
        Proof::Subfield {
            column_claims,
            quotients,
            inner_product,
        },
        claims,
    ))
}

pub fn prove(
    a: &[F128],
    b: &[F128],
    value_bits: usize,
    query: &Claim,
    ch: &mut impl Challenger,
) -> Result<(Proof, [Claim; 2])> {
    let len = shape(value_bits, query)?;
    if a.len() != len || b.len() != len {
        return Err("weighted Hadamard source dimensions".into());
    }
    if value_bits == 1 {
        let high: Vec<_> = query.rows.iter().rev().copied().collect();
        let (proof, claims) = weighted_hadamard::prove(
            storage_bytes(a),
            storage_bytes(b),
            query.rows.len() + 7,
            &high,
            &query.columns,
            query.value,
            mikros::Kernel::PairAggregate,
            ch,
        )
        .map_err(|e| format!("binary weighted Hadamard: {e}"))?;
        return Ok((Proof::Binary(proof), claims.map(binary_claim)));
    }
    if value_bits == 128 {
        return prove_full_field(a, b, query, ch);
    }
    if value_bits > 16 {
        return prove_wide(a, b, value_bits, query, ch);
    }
    let domain = domain(value_bits)?;
    bind(value_bits, query, ch);
    let weights = row_weights(&query.rows);
    let evaluations = aggregate(a, b, &weights, domain);
    let column_claims = evaluations[..domain.kappa].to_vec();
    if dot(&column_claims, &query.columns) != query.value {
        return Err("weighted Hadamard column projection".into());
    }
    let quotients = domain.quotient(&evaluations);
    observe(&column_claims, &quotients, ch);
    let rho = domain.packing(column_challenge(domain, ch));
    let table = tail_table(value_bits, &rho);
    let mut packed_a = vec![F128::ZERO; len];
    let mut packed_b = vec![F128::ZERO; len];
    packed_a
        .par_iter_mut()
        .zip(&mut packed_b)
        .enumerate()
        .for_each(|(i, (a_out, b_out))| {
            *a_out = pack(&table, a[i]);
            *b_out = pack(&table, b[i]) * weights[i];
        });
    drop(weights);
    drop(table);
    let (inner_product, point) = inner_product::prove_native_owned(packed_a, packed_b, ch);
    let claims = terminal_claims(query, rho, &inner_product, &point);
    Ok((
        Proof::Subfield {
            column_claims,
            quotients,
            inner_product,
        },
        claims,
    ))
}

pub fn verify(
    proof: &Proof,
    value_bits: usize,
    query: &Claim,
    ch: &mut impl Challenger,
) -> Result<[Claim; 2]> {
    shape(value_bits, query)?;
    if value_bits == 1 {
        let Proof::Binary(proof) = proof else {
            return Err("binary weighted Hadamard proof variant".into());
        };
        let high: Vec<_> = query.rows.iter().rev().copied().collect();
        return weighted_hadamard::verify(
            proof,
            query.rows.len() + 7,
            &high,
            &query.columns,
            query.value,
            ch,
        )
        .map(|claims| claims.map(binary_claim))
        .map_err(|e| format!("binary weighted Hadamard: {e}"));
    }
    if value_bits == 128 {
        let Proof::FullField(ip) = proof else {
            return Err("full-field weighted Hadamard proof variant".into());
        };
        if ip.rounds.len() != query.rows.len() {
            return Err("full-field weighted Hadamard proof dimensions".into());
        }
        bind(128, query, ch);
        let point = inner_product::verify(ip, query.value, ch)
            .map_err(|e| format!("full-field Hadamard IP: {e}"))?;
        return Ok(full_field_terminals(query, ip, &point));
    }
    let Proof::Subfield {
        column_claims,
        quotients,
        inner_product,
    } = proof
    else {
        return Err("subfield weighted Hadamard proof variant".into());
    };
    if value_bits > 16 {
        let d = wide_domain(value_bits)?;
        if column_claims.len() != d.kappa
            || quotients.len() != d.kappa - 1
            || inner_product.rounds.len() != query.rows.len()
        {
            return Err("wide weighted Hadamard proof dimensions".into());
        }
        bind(value_bits, query, ch);
        if dot(column_claims, &query.columns) != query.value {
            return Err("wide weighted Hadamard column projection".into());
        }
        observe(column_claims, quotients, ch);
        let x = loop {
            let x = ch.sample_f128();
            if !d.roots.contains(&x) {
                break x;
            }
        };
        let rho = d.packing(x);
        let target = dot(column_claims, &rho)
            + evaluate_extension(&d.vanishing, x) * evaluate_extension(quotients, x);
        let point = inner_product::verify(inner_product, target, ch)
            .map_err(|e| format!("wide Hadamard IP: {e}"))?;
        return Ok(terminal_claims(query, rho, inner_product, &point));
    }
    let domain = domain(value_bits)?;
    if column_claims.len() != domain.kappa
        || quotients.len() != domain.kappa - 1
        || inner_product.rounds.len() != query.rows.len()
    {
        return Err("weighted Hadamard proof dimensions".into());
    }
    bind(value_bits, query, ch);
    if dot(column_claims, &query.columns) != query.value {
        return Err("weighted Hadamard column projection".into());
    }
    observe(column_claims, quotients, ch);
    let x = column_challenge(domain, ch);
    let rho = domain.packing(x);
    let target = dot(column_claims, &rho)
        + evaluate_extension(&domain.vanishing, x) * evaluate_extension(quotients, x);
    let point = inner_product::verify(inner_product, target, ch)
        .map_err(|e| format!("subfield Hadamard IP: {e}"))?;
    Ok(terminal_claims(query, rho, inner_product, &point))
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_core::challenger::FsChallenger;
    use flock_field::test_rng::Rng;

    fn challenger() -> FsChallenger {
        FsChallenger::new(b"plonk-subfield-hadamard-independent-tests")
    }

    fn value(word: F128, bits: usize, column: usize) -> F128 {
        let field = SmallField::for_width(bits).unwrap();
        let coordinate = (raw(word) >> (bits * column)) & field.mask();
        field.embed(coordinate)
    }

    fn row_factor(row: usize, pairs: &[[F128; 2]]) -> F128 {
        pairs
            .iter()
            .enumerate()
            .fold(F128::ONE, |weight, (bit, pair)| {
                weight * pair[(row >> bit) & 1]
            })
    }

    fn source(words: &[F128], bits: usize, query: &Claim) -> F128 {
        words
            .iter()
            .enumerate()
            .fold(F128::ZERO, |sum, (row, &word)| {
                sum + row_factor(row, &query.rows)
                    * query
                        .columns
                        .iter()
                        .enumerate()
                        .fold(F128::ZERO, |sum, (column, &weight)| {
                            sum + weight * value(word, bits, column)
                        })
            })
    }

    // Products use the independently embedded GHASH operands, not the small
    // field multiplication, interpolation LUTs, buckets or the prover's IP.
    fn product(a: &[F128], b: &[F128], bits: usize, query: &Claim) -> F128 {
        a.iter()
            .zip(b)
            .enumerate()
            .fold(F128::ZERO, |sum, (row, (&a, &b))| {
                sum + row_factor(row, &query.rows)
                    * query
                        .columns
                        .iter()
                        .enumerate()
                        .fold(F128::ZERO, |sum, (column, &weight)| {
                            sum + weight * value(a, bits, column) * value(b, bits, column)
                        })
            })
    }

    #[test]
    fn interpolation_and_packing_tables_preserve_true_subfield_coordinates() {
        let mut rng = Rng::new(0x787125);
        for bits in [2, 4, 8, 16] {
            let d = domain(bits).unwrap();
            let rho = rng.f128_vec(d.kappa);
            let tail = tail_table(bits, &rho);
            for bit in 0..128 {
                let word = F128::new((1u128 << bit) as u64, ((1u128 << bit) >> 64) as u64);
                assert_eq!(
                    pack(&tail, word),
                    rho[bit / bits] * value(word, bits, bit / bits)
                );
                for (table, weights) in d.interpolation.iter().zip(&d.off_weights) {
                    let expected = (0..d.kappa).fold(F128::ZERO, |sum, column| {
                        sum + weights[column] * value(word, bits, column)
                    });
                    assert_eq!(d.field.embed(small_lookup(table, word) as u128), expected);
                }
            }
        }
    }

    #[test]
    fn bucket_aggregation_matches_direct_extension_products() {
        let mut rng = Rng::new(0x7a7a1102);
        let a = rng.f128_vec(17);
        let b = rng.f128_vec(17);
        let weights = rng.f128_vec(17);
        for bits in [2, 4, 8, 16] {
            let d = domain(bits).unwrap();
            let got = aggregate(&a, &b, &weights, d);
            for (sample, &actual) in got.iter().enumerate() {
                let expected = (0..a.len()).fold(F128::ZERO, |sum, row| {
                    let (a_value, b_value) = if sample < d.kappa {
                        (value(a[row], bits, sample), value(b[row], bits, sample))
                    } else {
                        let interpolate = |word| {
                            (0..d.kappa).fold(F128::ZERO, |sum, column| {
                                sum + d.off_weights[sample - d.kappa][column]
                                    * value(word, bits, column)
                            })
                        };
                        (interpolate(a[row]), interpolate(b[row]))
                    };
                    sum + weights[row] * a_value * b_value
                });
                assert_eq!(actual, expected, "b={bits}, sample={sample}");
            }
        }
    }

    #[test]
    fn arbitrary_zero_and_zero_sum_row_pairs_roundtrip_and_authenticate() {
        let mut rng = Rng::new(0x7a7a1103);
        for bits in [1, 2, 4, 8, 16, 32, 64, 128] {
            warmup(bits).unwrap();
            for row_log in [0, 1, 5] {
                let a = rng.f128_vec(1 << row_log);
                let b = rng.f128_vec(1 << row_log);
                for kind in 0..4 {
                    let mut query = Claim {
                        columns: rng.f128_vec(128 / bits),
                        rows: (0..row_log).map(|_| [rng.f128(), rng.f128()]).collect(),
                        value: F128::ZERO,
                    };
                    if row_log != 0 {
                        if kind == 1 {
                            query.rows[0] = [F128::ZERO; 2];
                        }
                        if kind == 2 {
                            query.rows[0] = [F128::new(11, 29); 2];
                        }
                    }
                    if kind == 3 {
                        query.columns.fill(F128::ZERO);
                    }
                    query.value = product(&a, &b, bits, &query);
                    let mut pc = challenger();
                    let (proof, claims) = prove(&a, &b, bits, &query, &mut pc).unwrap();
                    let bytes = bincode::serialize(&proof).unwrap();
                    let decoded = bincode::deserialize(&bytes).unwrap();
                    let mut vc = challenger();
                    assert_eq!(verify(&decoded, bits, &query, &mut vc).unwrap(), claims);
                    assert_eq!(pc.sample_f128(), vc.sample_f128());
                    for (words, claim) in [&a, &b].into_iter().zip(&claims) {
                        assert_eq!(source(words, bits, claim), claim.value);
                    }
                }
            }
        }
    }

    #[test]
    fn tampered_messages_claims_and_dimensions_are_rejected() {
        let mut rng = Rng::new(0x7a7a1104);
        for bits in [1, 2, 4, 8, 16, 32, 64] {
            let a = rng.f128_vec(8);
            let b = rng.f128_vec(8);
            let mut query = Claim {
                columns: rng.f128_vec(128 / bits),
                rows: (0..3).map(|_| [rng.f128(), rng.f128()]).collect(),
                value: F128::ZERO,
            };
            query.value = product(&a, &b, bits, &query);
            let (proof, _) = prove(&a, &b, bits, &query, &mut challenger()).unwrap();
            for mutation in 0..6 {
                let mut bad = proof.clone();
                let (columns, quotients, ip) = match &mut bad {
                    Proof::Binary(p) => {
                        (&mut p.column_claims, &mut p.quotients, &mut p.inner_product)
                    }
                    Proof::Subfield {
                        column_claims,
                        quotients,
                        inner_product,
                    } => (column_claims, quotients, inner_product),
                    Proof::FullField(_) => unreachable!(),
                };
                match mutation {
                    0 => columns[0] += F128::ONE,
                    1 => quotients[0] += F128::ONE,
                    2 => ip.final_a += F128::ONE,
                    3 => ip.final_b += F128::ONE,
                    4 => {
                        ip.rounds.pop();
                    }
                    _ => {
                        columns.pop();
                    }
                }
                assert!(
                    verify(&bad, bits, &query, &mut challenger()).is_err(),
                    "b={bits} mutation={mutation}"
                );
            }
            let mut bad = query.clone();
            bad.value += F128::ONE;
            assert!(verify(&proof, bits, &bad, &mut challenger()).is_err());
            assert!(prove(&a[..7], &b, bits, &query, &mut challenger()).is_err());
            bad.columns.pop();
            assert!(verify(&proof, bits, &bad, &mut challenger()).is_err());
            assert!(verify(&proof, 3, &query, &mut challenger()).is_err());
        }
    }

    #[test]
    fn block_boundaries_and_thread_counts_preserve_proof_bytes() {
        let mut rng = Rng::new(0x7a7a1105);
        let a = rng.f128_vec(8192);
        let b = rng.f128_vec(8192);
        for bits in [2, 16, 32, 64, 128] {
            let mut query = Claim {
                columns: rng.f128_vec(128 / bits),
                rows: vec![[F128::ONE, F128::ONE]; 13],
                value: F128::ZERO,
            };
            query.value = product(&a, &b, bits, &query);
            let results: Vec<_> = [1, 3]
                .into_iter()
                .map(|threads| {
                    rayon::ThreadPoolBuilder::new()
                        .num_threads(threads)
                        .build()
                        .unwrap()
                        .install(|| {
                            let mut pc = challenger();
                            let result = prove(&a, &b, bits, &query, &mut pc).unwrap();
                            (result, pc.sample_f128())
                        })
                })
                .collect();
            assert_eq!(results[0], results[1]);
        }
    }

    #[test]
    fn wide_interpolation_and_buckets_preserve_all_coordinate_bits() {
        let mut rng = Rng::new(0x11f5ca71);
        for bits in [32, 64] {
            let d = wide_domain(bits).unwrap();
            let rho = rng.f128_vec(d.kappa);
            let tail = tail_table(bits, &rho);
            for bit in 0..128 {
                let word = mikros_permutation::field::from_raw(1u128 << bit);
                assert_eq!(
                    pack(&tail, word),
                    rho[bit / bits] * value(word, bits, bit / bits)
                );
                for (table, weights) in d.interpolation.iter().zip(&d.off_weights) {
                    let expected = (0..d.kappa).fold(F128::ZERO, |sum, column| {
                        sum + weights[column] * value(word, bits, column)
                    });
                    assert_eq!(d.field.embed(wide_lookup(table, word) as u128), expected);
                }
            }
            let mut a = rng.f128_vec(37);
            let mut b = rng.f128_vec(37);
            let high = (0..d.kappa).fold(0u128, |out, column| {
                out | (1u128 << ((column + 1) * bits - 1))
            });
            a[0] = mikros_permutation::field::from_raw(high);
            b[0] = mikros_permutation::field::from_raw(u128::MAX);
            let weights = rng.f128_vec(a.len());
            for (sample, actual) in wide_aggregate(&a, &b, &weights, d).into_iter().enumerate() {
                let interpolate = |word| {
                    if sample < d.kappa {
                        value(word, bits, sample)
                    } else {
                        (0..d.kappa).fold(F128::ZERO, |sum, column| {
                            sum + d.off_weights[sample - d.kappa][column]
                                * value(word, bits, column)
                        })
                    }
                };
                let expected = (0..a.len()).fold(F128::ZERO, |sum, row| {
                    sum + weights[row] * interpolate(a[row]) * interpolate(b[row])
                });
                assert_eq!(actual, expected, "b={bits}, sample={sample}");
            }
        }
    }

    #[test]
    fn full_field_native_ip_has_no_column_round_and_binds_scaling() {
        let mut rng = Rng::new(0x11f5ca72);
        let mut a = rng.f128_vec(8);
        let b = rng.f128_vec(8);
        a[0] = F128::new(0, 1u64 << 63);
        let mut query = Claim {
            columns: vec![F128::new(0x35, 0x421)],
            rows: (0..3).map(|_| [rng.f128(), rng.f128()]).collect(),
            value: F128::ZERO,
        };
        query.value = product(&a, &b, 128, &query);
        let mut pc = challenger();
        let (proof, terminals) = prove(&a, &b, 128, &query, &mut pc).unwrap();
        let Proof::FullField(ip) = &proof else {
            panic!("expected direct IP");
        };
        assert_eq!(ip.rounds.len(), 3);
        assert_eq!(terminals[0].columns, [F128::ONE]);
        assert_eq!(terminals[1].columns, query.columns);
        // Independent expanded native IP gives identical messages and next
        // challenge. There is no interpolation certificate or sampled rho.
        let right: Vec<_> = b
            .iter()
            .enumerate()
            .map(|(i, &b)| b * query.columns[0] * row_factor(i, &query.rows))
            .collect();
        let mut native_ch = challenger();
        bind(128, &query, &mut native_ch);
        let (native, _) = inner_product::prove_native_owned(a.clone(), right, &mut native_ch);
        assert_eq!(ip, &native);
        assert_eq!(pc.sample_f128(), native_ch.sample_f128());
        for mutation in 0..4 {
            let mut bad = ip.clone();
            match mutation {
                0 => bad.rounds[0][0] += F128::ONE,
                1 => bad.final_a += F128::ONE,
                2 => bad.final_b += F128::ONE,
                _ => {
                    bad.rounds.pop();
                }
            }
            assert!(verify(&Proof::FullField(bad), 128, &query, &mut challenger()).is_err());
        }
        for mutation in 0..3 {
            let mut bad = query.clone();
            match mutation {
                0 => bad.value += F128::ONE,
                1 => bad.columns[0] += F128::ONE,
                _ => bad.rows[0][1] += F128::ONE,
            }
            assert!(verify(&proof, 128, &bad, &mut challenger()).is_err());
            assert!(prove(&a, &b, 128, &bad, &mut challenger()).is_err());
        }
        assert!(verify(&proof, 64, &query, &mut challenger()).is_err());
        let wrong_variant = Proof::Subfield {
            column_claims: vec![query.value],
            quotients: vec![],
            inner_product: ip.clone(),
        };
        assert!(verify(&wrong_variant, 128, &query, &mut challenger()).is_err());
    }
}

#[path = "hadamard_batch.rs"]
pub mod batch;
