//! Actual subfields K of the fixed GHASH E. Only degrees dividing 128 exist.
//! A logical w-bit value is embedded injectively in the next power-of-two
//! subfield; an arbitrary w-bit vector space is never called GF(2^w).
use crate::{EXTENSION_BITS, Error};
use flock_field::{F8, F128, phi8};
use std::sync::OnceLock;

pub fn raw(value: F128) -> u128 {
    value.lo as u128 | ((value.hi as u128) << 64)
}
pub fn from_raw(value: u128) -> F128 {
    F128::new(value as u64, (value >> 64) as u64)
}

#[derive(Debug)]
pub struct SmallField {
    pub bits: usize,
    modulus: u128,
    basis: Vec<F128>,
}

fn pow(mut base: F128, mut exponent: u128) -> F128 {
    let mut out = F128::ONE;
    while exponent != 0 {
        if exponent & 1 != 0 {
            out *= base;
        }
        base = base.square();
        exponent >>= 1;
    }
    out
}
fn frobenius(mut value: F128, degree: usize) -> F128 {
    for _ in 0..degree {
        value = value.square();
    }
    value
}
impl SmallField {
    fn build(bits: usize) -> Self {
        if bits == 128 {
            return Self {
                bits,
                modulus: 0x87,
                basis: (0..128).map(|i| from_raw(1u128 << i)).collect(),
            };
        }
        if bits == 1 {
            return Self {
                bits,
                modulus: 1,
                basis: vec![F128::ONE],
            };
        }
        let beta = if bits == 8 {
            phi8(F8(2))
        } else {
            let exponent = u128::MAX / ((1u128 << bits) - 1);
            (2u128..)
                .map(|seed| pow(from_raw(seed), exponent))
                .find(|&candidate| frobenius(candidate, bits / 2) != candidate)
                .expect("proper subfield has a degree-bits generator")
        };
        assert_eq!(frobenius(beta, bits), beta);
        let mut polynomial = vec![F128::ONE];
        let mut root = beta;
        for _ in 0..bits {
            let mut next = vec![F128::ZERO; polynomial.len() + 1];
            for (i, &coefficient) in polynomial.iter().enumerate() {
                next[i] += root * coefficient;
                next[i + 1] += coefficient;
            }
            polynomial = next;
            root = root.square();
        }
        let mut modulus = 0;
        for (i, &coefficient) in polynomial[..bits].iter().enumerate() {
            assert!(coefficient == F128::ZERO || coefficient == F128::ONE);
            if coefficient == F128::ONE {
                modulus |= 1u128 << i;
            }
        }
        let mut value = F128::ONE;
        let basis = (0..bits)
            .map(|_| {
                let out = value;
                value *= beta;
                out
            })
            .collect();
        Self {
            bits,
            modulus,
            basis,
        }
    }
    pub fn for_width(width: usize) -> Result<&'static Self, Error> {
        if width > EXTENSION_BITS {
            return Err(Error::Capacity);
        }
        static FIELDS: [OnceLock<SmallField>; 8] = [const { OnceLock::new() }; 8];
        let bits = width.max(1).next_power_of_two();
        Ok(FIELDS[bits.ilog2() as usize].get_or_init(|| Self::build(bits)))
    }
    pub fn basis(&self) -> &[F128] {
        &self.basis
    }
    pub fn mask(&self) -> u128 {
        if self.bits == 128 {
            u128::MAX
        } else {
            (1u128 << self.bits) - 1
        }
    }
    pub fn embed(&self, mut value: u128) -> F128 {
        assert_eq!(value & !self.mask(), 0);
        if self.bits == 128 {
            return from_raw(value);
        }
        if self.bits == 8 {
            return phi8(F8(value as u8));
        }
        let mut out = F128::ZERO;
        while value != 0 {
            let bit = value.trailing_zeros() as usize;
            out += self.basis[bit];
            value &= value - 1;
        }
        out
    }
    pub fn multiply(&self, mut a: u128, mut b: u128) -> u128 {
        debug_assert_eq!((a | b) & !self.mask(), 0);
        if self.bits == 128 {
            return raw(from_raw(a) * from_raw(b));
        }
        if self.bits == 8 {
            return (F8(a as u8) * F8(b as u8)).0 as u128;
        }
        let mut out = 0;
        while b != 0 {
            if b & 1 != 0 {
                out ^= a;
            }
            let top = (a >> (self.bits - 1)) & 1;
            a = (a << 1) & self.mask();
            if top != 0 {
                a ^= self.modulus;
            }
            b >>= 1;
        }
        out
    }
}

/// Byte-linear embedding and polynomial reduction tables. The modulus and
/// basis are exactly the reference SmallField's; no new field is introduced.
pub struct FastField {
    pub base: &'static SmallField,
    pub embedding: Vec<[F128; 256]>,
    reduction: Vec<[u64; 256]>,
    // Low word of floor(x^64 / P), for the same monic K32 modulus P.
    reciprocal32: u32,
    // Low word of floor(x^128 / P), for the same monic K64 modulus P.
    reciprocal64: u64,
}

fn reciprocal32(modulus: u32) -> u32 {
    let divisor = (1u64 << 32) | u64::from(modulus);
    // x^64 = x^32 P + modulus*x^32, so this quotient is the low word.
    let mut remainder = u64::from(modulus) << 32;
    let mut quotient = 0u32;
    for shift in (0..32).rev() {
        if remainder & (1u64 << (32 + shift)) != 0 {
            remainder ^= divisor << shift;
            quotient |= 1u32 << shift;
        }
    }
    quotient
}

fn reciprocal64(modulus: u64) -> u64 {
    let divisor = (1u128 << 64) | u128::from(modulus);
    // x^128 = x^64 P + modulus*x^64, so this quotient is the low word.
    let mut remainder = u128::from(modulus) << 64;
    let mut quotient = 0u64;
    for shift in (0..64).rev() {
        if remainder & (1u128 << (64 + shift)) != 0 {
            remainder ^= divisor << shift;
            quotient |= 1u64 << shift;
        }
    }
    quotient
}

impl FastField {
    pub fn for_width(width: usize) -> Result<&'static Self, Error> {
        static FIELDS: [OnceLock<FastField>; 8] = [const { OnceLock::new() }; 8];
        let base = SmallField::for_width(width)?;
        Ok(FIELDS[base.bits.ilog2() as usize].get_or_init(|| {
            let embedding = linear_tables(base.basis());
            let mut reduction = Vec::new();
            if (8..=64).contains(&base.bits) {
                let mut powers = Vec::new();
                let mut value = base.modulus;
                for _ in 0..base.bits {
                    powers.push(value as u64);
                    let carry = value >> (base.bits - 1);
                    value = ((value << 1) & base.mask()) ^ (base.modulus * carry);
                }
                for chunk in powers.chunks(8) {
                    let mut table = [0; 256];
                    for i in 1usize..256 {
                        let bit = i.trailing_zeros() as usize;
                        table[i] = table[i & (i - 1)] ^ chunk[bit];
                    }
                    reduction.push(table);
                }
            }
            Self {
                base,
                embedding,
                reduction,
                reciprocal32: if base.bits == 32 {
                    reciprocal32(base.modulus as u32)
                } else {
                    0
                },
                reciprocal64: if base.bits == 64 {
                    reciprocal64(base.modulus as u64)
                } else {
                    0
                },
            }
        }))
    }
    #[inline]
    pub fn embed(&self, value: u128) -> F128 {
        evaluate_linear(&self.embedding, value)
    }
    /// Multiply by the polynomial-basis generator in the existing subfield.
    /// For K1 the generator reduces to one; for all larger fields it is 2.
    #[inline]
    pub(crate) fn multiply_by_x(&self, value: u128) -> u128 {
        debug_assert!(self.base.bits <= 64);
        debug_assert_eq!(value & !self.base.mask(), 0);
        let carry = value >> (self.base.bits - 1);
        ((value << 1) & self.base.mask()) ^ (self.base.modulus * carry)
    }
    /// Reduce a XOR-sum of unreduced GF(2^32) polynomial products. Linear
    /// reduction is postponed until the complete mixed-field dot product.
    #[inline]
    pub(crate) fn reduce_product32(&self, product: u64) -> u32 {
        assert_eq!(self.base.bits, 32);
        if cfg!(all(target_arch = "x86_64", target_feature = "pclmulqdq")) {
            return self.reduce_product32_reciprocal(product);
        }
        let mut result = product as u32;
        let mut high = product >> 32;
        for table in &self.reduction {
            result ^= table[(high & 255) as usize] as u32;
            high >>= 8;
        }
        result
    }
    #[inline]
    pub fn multiply(&self, a: u128, b: u128) -> u128 {
        let bits = self.base.bits;
        if bits <= 8 || bits == 128 {
            return self.base.multiply(a, b);
        }
        let product = carryless(a as u64, b as u64);
        // The selection is compile-time, with no feature/environment lookup in
        // the hot loop. Other widths and portable builds retain the table path.
        if bits == 32 && cfg!(all(target_arch = "x86_64", target_feature = "pclmulqdq")) {
            return u128::from(self.reduce_product32_reciprocal(product as u64));
        }
        if bits == 64 && cfg!(all(target_arch = "x86_64", target_feature = "pclmulqdq")) {
            return u128::from(self.reduce_product64_reciprocal(product));
        }
        self.reduce_product_table(product)
    }
    #[inline]
    fn reduce_product32_reciprocal(&self, product: u64) -> u32 {
        debug_assert_eq!(self.base.bits, 32);
        let high = (product >> 32) as u32;
        // P=x^32+m and mu=floor(x^64/P)=x^32+reciprocal32. For
        // x^64=mu*P+R, deg(R)<32; high*R has degree <64, hence
        // floor(high*x^32/P)=floor(high*mu/x^32). The low 32 bits
        // of product + quotient*P are therefore the exact remainder.
        let quotient =
            high ^ (carryless(u64::from(high), u64::from(self.reciprocal32)) >> 32) as u32;
        product as u32 ^ carryless(u64::from(quotient), self.base.modulus as u64) as u32
    }
    #[inline]
    fn reduce_product64_reciprocal(&self, product: u128) -> u64 {
        debug_assert_eq!(self.base.bits, 64);
        let high = (product >> 64) as u64;
        // For mu = x^64 + reciprocal64 = floor(x^128/P),
        // floor(high*x^64/P) = high + high64(high*reciprocal64).
        // The discarded remainder has degree <64 and cannot change this quotient.
        let quotient = high ^ (carryless(high, self.reciprocal64) >> 64) as u64;
        product as u64 ^ carryless(quotient, self.base.modulus as u64) as u64
    }
    #[inline]
    fn reduce_product_table(&self, product: u128) -> u128 {
        let bits = self.base.bits;
        let mut result = (product & self.base.mask()) as u64;
        let mut high = product >> bits;
        for table in &self.reduction {
            result ^= table[(high & 255) as usize];
            high >>= 8;
        }
        result as u128
    }
}

pub(crate) fn linear_tables(basis: &[F128]) -> Vec<[F128; 256]> {
    basis
        .chunks(8)
        .map(|chunk| {
            let mut table = [F128::ZERO; 256];
            for i in 1usize..256 {
                let bit = i.trailing_zeros() as usize;
                table[i] = table[i & (i - 1)] + chunk.get(bit).copied().unwrap_or(F128::ZERO);
            }
            table
        })
        .collect()
}
#[inline]
pub(crate) fn evaluate_linear(tables: &[[F128; 256]], mut value: u128) -> F128 {
    let mut out = F128::ZERO;
    for table in tables {
        out += table[(value & 255) as usize];
        value >>= 8;
    }
    out
}

#[inline]
fn carryless(a: u64, b: u64) -> u128 {
    #[cfg(all(target_arch = "x86_64", target_feature = "pclmulqdq"))]
    {
        use std::arch::x86_64::{_mm_clmulepi64_si128, _mm_set_epi64x};
        // SAFETY: compile-time feature gate guarantees PCLMUL; transmute keeps
        // the full 128-bit polynomial product in little-endian limb order.
        unsafe {
            std::mem::transmute(_mm_clmulepi64_si128::<0>(
                _mm_set_epi64x(0, a as i64),
                _mm_set_epi64x(0, b as i64),
            ))
        }
    }
    #[cfg(not(all(target_arch = "x86_64", target_feature = "pclmulqdq")))]
    {
        let mut b = b;
        let mut out = 0;
        while b != 0 {
            out ^= (a as u128) << b.trailing_zeros();
            b &= b - 1;
        }
        out
    }
}

pub(crate) struct PolynomialEvaluation {
    constant: F128,
    tables: std::sync::Arc<Vec<[F128; 256]>>,
}
impl PolynomialEvaluation {
    /// Leaves may occupy fewer logical bits than their containing subfield.
    pub fn leaf(field: &SmallField, used_bits: usize, z: F128) -> Self {
        assert!(used_bits <= field.bits);
        Self {
            constant: z,
            tables: std::sync::Arc::new(linear_tables(&field.basis()[..used_bits])),
        }
    }
    pub fn new(field: &SmallField, packed_bits: usize, z: F128) -> Self {
        let degree = packed_bits / field.bits;
        let mut power = F128::ONE;
        let mut basis = Vec::with_capacity(packed_bits);
        for _ in 0..degree {
            basis.extend(field.basis().iter().map(|&b| b * power));
            power *= z;
        }
        Self {
            constant: power,
            tables: std::sync::Arc::new(linear_tables(&basis)),
        }
    }
    /// Reuse an offline leaf embedding table with the newly sampled shift.
    /// The constant changes but the table is shared, without copying entries.
    pub(crate) fn with_constant(&self, constant: F128) -> Self {
        Self {
            constant,
            tables: self.tables.clone(),
        }
    }
    pub(crate) fn table_bytes(&self) -> usize {
        self.tables.capacity() * std::mem::size_of::<[F128; 256]>()
    }
    #[inline]
    pub fn evaluate(&self, word: u128) -> F128 {
        self.constant + evaluate_linear(&self.tables, word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn next(state: &mut u64) -> u64 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        *state
    }

    // Independent polynomial long division, also covering products with bit 127.
    fn remainder64(mut polynomial: u128, modulus: u64) -> u64 {
        let divisor = (1u128 << 64) | u128::from(modulus);
        while polynomial >> 64 != 0 {
            let shift = 63 - polynomial.leading_zeros();
            polynomial ^= divisor << shift;
        }
        polynomial as u64
    }

    fn remainder32(mut polynomial: u64, modulus: u32) -> u32 {
        let divisor = (1u64 << 32) | u64::from(modulus);
        while polynomial >> 32 != 0 {
            let shift = 31 - polynomial.leading_zeros();
            polynomial ^= divisor << shift;
        }
        polynomial as u32
    }

    fn assert_k32_product(field: &FastField, a: u32, b: u32) {
        let product = carryless(u64::from(a), u64::from(b));
        let expected = field.base.multiply(u128::from(a), u128::from(b));
        assert_eq!(field.reduce_product_table(product), expected);
        assert_eq!(
            u128::from(field.reduce_product32_reciprocal(product as u64)),
            expected,
            "reciprocal product {a:08x} * {b:08x}"
        );
        assert_eq!(u128::from(field.reduce_product32(product as u64)), expected);
        assert_eq!(field.multiply(u128::from(a), u128::from(b)), expected);
        assert_eq!(
            u128::from(remainder32(product as u64, field.base.modulus as u32)),
            expected
        );
        assert_eq!(
            field.embed(expected),
            field.embed(u128::from(a)) * field.embed(u128::from(b)),
            "the existing K32 embedding must remain multiplicative"
        );
    }

    #[test]
    fn reciprocal_k32_products_preserve_field_and_embedding() {
        let field = FastField::for_width(32).unwrap();
        let edges = [0, 1, 2, 1u32 << 31, u32::MAX, 0x5555_5555, 0xaaaa_aaaa];
        for a in edges {
            for b in edges {
                assert_k32_product(field, a, b);
            }
        }
        for i in 0..32 {
            for j in 0..32 {
                assert_k32_product(field, 1u32 << i, 1u32 << j);
            }
        }
        let mut state = 0xa54f_f53a_5f1d_36f1;
        for _ in 0..4096 {
            let a = next(&mut state) as u32;
            let b = next(&mut state) as u32;
            assert_k32_product(field, a, b);
        }
    }

    #[test]
    fn reciprocal_k32_reduces_full_width_products() {
        let field = FastField::for_width(32).unwrap();
        let mut state = 0x510e_527f_ade6_82d1;
        // Bit 63 exceeds the maximum degree of an ordinary 32x32 product,
        // but remains supported by the generic linear reduction API.
        let edges = [0, 1, 1u64 << 31, 1u64 << 32, 1u64 << 63, u64::MAX];
        let products = edges
            .into_iter()
            .chain((0..64).map(|i| 1u64 << i))
            .chain((0..4096).map(|_| next(&mut state)));
        for product in products {
            let expected = remainder32(product, field.base.modulus as u32);
            assert_eq!(field.reduce_product32_reciprocal(product), expected);
            assert_eq!(field.reduce_product32(product), expected);
            assert_eq!(
                field.reduce_product_table(u128::from(product)),
                u128::from(expected)
            );
        }
    }

    #[test]
    fn reciprocal_k32_matches_polynomial_quotient() {
        // Independently divide x^64 rather than starting with modulus*x^32.
        // This also checks dense and sparse public moduli, not just our field.
        let mut state = 0x9b05_688c_2b3e_6c1f;
        for modulus in [0, 1, 0x87, 1u32 << 31, u32::MAX]
            .into_iter()
            .chain((0..128).map(|_| next(&mut state) as u32))
        {
            let divisor = (1u128 << 32) | u128::from(modulus);
            let mut polynomial = 1u128 << 64;
            let mut quotient = 0u64;
            while polynomial >> 32 != 0 {
                let shift = 95 - polynomial.leading_zeros();
                polynomial ^= divisor << shift;
                quotient ^= 1u64 << shift;
            }
            assert_eq!(quotient, (1u64 << 32) | u64::from(reciprocal32(modulus)));
        }
    }

    fn assert_k64_product(field: &FastField, a: u64, b: u64) {
        let product = carryless(a, b);
        let expected = field.base.multiply(u128::from(a), u128::from(b));
        assert_eq!(field.reduce_product_table(product), expected);
        assert_eq!(
            u128::from(field.reduce_product64_reciprocal(product)),
            expected,
            "reciprocal product {a:016x} * {b:016x}"
        );
        assert_eq!(field.multiply(u128::from(a), u128::from(b)), expected);
        assert_eq!(
            u128::from(remainder64(product, field.base.modulus as u64)),
            expected
        );
        assert_eq!(
            field.embed(expected),
            field.embed(u128::from(a)) * field.embed(u128::from(b)),
            "the existing K64 embedding must remain multiplicative"
        );
    }

    #[test]
    fn reciprocal_k64_products_preserve_field_and_embedding() {
        let field = FastField::for_width(64).unwrap();
        let edges = [0, 1, 2, 1u64 << 63, u64::MAX, 0x5555_5555_5555_5555];
        for a in edges {
            for b in edges {
                assert_k64_product(field, a, b);
            }
        }
        for i in 0..64 {
            for j in 0..64 {
                assert_k64_product(field, 1u64 << i, 1u64 << j);
            }
        }
        let mut state = 0x6a09_e667_f3bc_c909;
        for _ in 0..4096 {
            let a = next(&mut state);
            let b = next(&mut state);
            assert_k64_product(field, a, b);
        }
    }

    #[test]
    fn reciprocal_k64_reduces_full_width_products() {
        let field = FastField::for_width(64).unwrap();
        let mut state = 0xbb67_ae85_84ca_a73b;
        let edges = [0, 1, 1u128 << 63, 1u128 << 64, 1u128 << 127, u128::MAX];
        let products = edges.into_iter().chain(
            (0..4096).map(|_| u128::from(next(&mut state)) | (u128::from(next(&mut state)) << 64)),
        );
        for product in products {
            let expected = remainder64(product, field.base.modulus as u64);
            assert_eq!(field.reduce_product64_reciprocal(product), expected);
            assert_eq!(field.reduce_product_table(product), u128::from(expected));
        }
    }

    #[test]
    fn multiply_by_x_matches_existing_subfields() {
        let mut state = 0x3c6e_f372_fe94_f82b;
        for bits in [1, 2, 4, 8, 16, 32, 64] {
            let field = FastField::for_width(bits).unwrap();
            // x is 1 modulo x+1 in K1; 2 is not a valid K1 field element.
            let x = if bits == 1 { field.base.modulus } else { 2 };
            let edges = [0, 1, 1u128 << (bits - 1), field.base.mask()];
            let values = edges
                .into_iter()
                .chain((0..4096).map(|_| u128::from(next(&mut state)) & field.base.mask()));
            for value in values {
                let actual = field.multiply_by_x(value);
                assert_eq!(actual, field.multiply(value, x), "K{bits}, {value:x}");
                assert_eq!(actual, field.base.multiply(value, x));
                assert_eq!(field.embed(actual), field.embed(value) * field.embed(x));
            }
        }
    }
}
