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
            }
        }))
    }
    #[inline]
    pub fn embed(&self, value: u128) -> F128 {
        evaluate_linear(&self.embedding, value)
    }
    #[inline]
    pub fn multiply(&self, a: u128, b: u128) -> u128 {
        let bits = self.base.bits;
        if bits <= 8 || bits == 128 {
            return self.base.multiply(a, b);
        }
        let product = carryless(a as u64, b as u64);
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
    tables: Vec<[F128; 256]>,
}
impl PolynomialEvaluation {
    /// Leaves may occupy fewer logical bits than their containing subfield.
    pub fn leaf(field: &SmallField, used_bits: usize, z: F128) -> Self {
        assert!(used_bits <= field.bits);
        Self {
            constant: z,
            tables: linear_tables(&field.basis()[..used_bits]),
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
            tables: linear_tables(&basis),
        }
    }
    #[inline]
    pub fn evaluate(&self, word: u128) -> F128 {
        self.constant + evaluate_linear(&self.tables, word)
    }
}
