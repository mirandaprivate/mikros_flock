//! Exact arkworks view of GHASH GF(2^128). Integer conversion is reduction
//! modulo TWO; use from_flock/raw coordinates for polynomial-basis bit strings.
use crate::field::{from_raw, raw};
use ark_ff::{
    AdditiveGroup, BigInt, Field, Fp, FpConfig, LegendreSymbol, One, PrimeField,
    SqrtPrecomputation, Zero,
};
use ark_serialize::{
    CanonicalDeserialize, CanonicalDeserializeWithFlags, CanonicalSerialize,
    CanonicalSerializeWithFlags, Compress, Flags, Read, SerializationError, Valid, Validate, Write,
};
use ark_std::rand::{
    Rng,
    distributions::{Distribution, Standard},
};
use flock_field::F128;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    iter::{Product, Sum},
    marker::PhantomData,
    ops::*,
};
use whir::transcript::{
    Decoding, Encoding, NargDeserialize, VerificationError, VerificationResult,
};
use zeroize::Zeroize;

/// Genuine F2, using arkworks' configurable prime-field arithmetic rather
/// than an odd-modulus Montgomery backend (which is invalid at modulus two).
pub struct BitConfig;
pub type Bit = Fp<BitConfig, 1>;
impl FpConfig<1> for BitConfig {
    const MODULUS: BigInt<1> = BigInt([2]);
    const GENERATOR: Bit = Self::ONE;
    const ZERO: Bit = Fp(BigInt([0]), PhantomData);
    const ONE: Bit = Fp(BigInt([1]), PhantomData);
    const TWO_ADICITY: u32 = 0;
    const TWO_ADIC_ROOT_OF_UNITY: Bit = Self::ONE;
    // F2 has only 0 and 1: these parameters return each as its own square root.
    const SQRT_PRECOMP: Option<SqrtPrecomputation<Bit>> = Some(SqrtPrecomputation::TonelliShanks {
        two_adicity: 0,
        quadratic_nonresidue_to_trace: Self::ONE,
        trace_of_modulus_minus_one_div_two: &[0],
    });
    fn add_assign(a: &mut Bit, b: &Bit) {
        a.0.0[0] ^= b.0.0[0];
    }
    fn sub_assign(a: &mut Bit, b: &Bit) {
        Self::add_assign(a, b);
    }
    fn double_in_place(a: &mut Bit) {
        *a = Self::ZERO;
    }
    fn neg_in_place(_: &mut Bit) {}
    fn mul_assign(a: &mut Bit, b: &Bit) {
        a.0.0[0] &= b.0.0[0];
    }
    fn sum_of_products<const T: usize>(a: &[Bit; T], b: &[Bit; T]) -> Bit {
        a.iter().zip(b).fold(Self::ZERO, |s, (a, b)| s + *a * *b)
    }
    fn square_in_place(_: &mut Bit) {}
    fn inverse(a: &Bit) -> Option<Bit> {
        (!a.is_zero()).then_some(*a)
    }
    fn from_bigint(x: BigInt<1>) -> Option<Bit> {
        (x.0[0] < 2).then_some(Fp(x, PhantomData))
    }
    fn into_bigint(x: Bit) -> BigInt<1> {
        x.0
    }
}

#[derive(
    Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Ghash(pub u128);
impl Ghash {
    pub fn from_flock(x: F128) -> Self {
        Self(raw(x))
    }
    pub fn to_flock(self) -> F128 {
        from_raw(self.0)
    }
    pub fn bytes(self) -> [u8; 16] {
        self.0.to_le_bytes()
    }
}
impl fmt::Display for Ghash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:032x}", self.0)
    }
}
impl Zero for Ghash {
    fn zero() -> Self {
        Self(0)
    }
    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
impl One for Ghash {
    fn one() -> Self {
        Self(1)
    }
    fn is_one(&self) -> bool {
        self.0 == 1
    }
}
impl Zeroize for Ghash {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}
impl Neg for Ghash {
    type Output = Self;
    fn neg(self) -> Self {
        self
    }
}

macro_rules! operation {
    ($trait:ident,$method:ident,$assign:ident,$assign_method:ident,$body:expr) => {
        impl $trait for Ghash {
            type Output = Self;
            fn $method(self, rhs: Self) -> Self {
                ($body)(self, rhs)
            }
        }
        impl $trait<&Self> for Ghash {
            type Output = Self;
            fn $method(self, rhs: &Self) -> Self {
                self.$method(*rhs)
            }
        }
        impl $trait<&mut Self> for Ghash {
            type Output = Self;
            fn $method(self, rhs: &mut Self) -> Self {
                self.$method(*rhs)
            }
        }
        impl $assign for Ghash {
            fn $assign_method(&mut self, rhs: Self) {
                *self = self.$method(rhs);
            }
        }
        impl $assign<&Self> for Ghash {
            fn $assign_method(&mut self, rhs: &Self) {
                *self = self.$method(*rhs);
            }
        }
        impl $assign<&mut Self> for Ghash {
            fn $assign_method(&mut self, rhs: &mut Self) {
                *self = self.$method(*rhs);
            }
        }
    };
}
operation!(Add, add, AddAssign, add_assign, |a: Ghash, b: Ghash| Ghash(
    a.0 ^ b.0
));
operation!(Sub, sub, SubAssign, sub_assign, |a: Ghash, b: Ghash| Ghash(
    a.0 ^ b.0
));
operation!(Mul, mul, MulAssign, mul_assign, |a: Ghash, b: Ghash| {
    Ghash::from_flock(a.to_flock() * b.to_flock())
});
operation!(Div, div, DivAssign, div_assign, |a: Ghash, b: Ghash| a * b
    .inverse()
    .expect("division by zero"));
impl Sum for Ghash {
    fn sum<I: Iterator<Item = Self>>(xs: I) -> Self {
        xs.fold(Self::ZERO, |a, b| a + b)
    }
}
impl<'a> Sum<&'a Self> for Ghash {
    fn sum<I: Iterator<Item = &'a Self>>(xs: I) -> Self {
        xs.copied().sum()
    }
}
impl Product for Ghash {
    fn product<I: Iterator<Item = Self>>(xs: I) -> Self {
        xs.fold(Self::ONE, |a, b| a * b)
    }
}
impl<'a> Product<&'a Self> for Ghash {
    fn product<I: Iterator<Item = &'a Self>>(xs: I) -> Self {
        xs.copied().product()
    }
}
macro_rules! integer {($($t:ty),*)=>{$(impl From<$t> for Ghash {fn from(x:$t)->Self {Self((x as u128)&1)}})*}}
integer!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
impl From<bool> for Ghash {
    fn from(x: bool) -> Self {
        Self(x as u128)
    }
}
impl Distribution<Ghash> for Standard {
    fn sample<R: Rng + ?Sized>(&self, r: &mut R) -> Ghash {
        Ghash(r.next_u64() as u128 | ((r.next_u64() as u128) << 64))
    }
}
impl AdditiveGroup for Ghash {
    type Scalar = Self;
    const ZERO: Self = Self(0);
    fn double_in_place(&mut self) -> &mut Self {
        *self = Self::ZERO;
        self
    }
}
impl Field for Ghash {
    type BasePrimeField = Bit;
    const SQRT_PRECOMP: Option<SqrtPrecomputation<Self>> = None;
    const ONE: Self = Self(1);
    fn extension_degree() -> u64 {
        128
    }
    fn to_base_prime_field_elements(&self) -> impl Iterator<Item = Bit> {
        let value = self.0;
        (0..128).map(move |j| Bit::from(((value >> j) & 1) as u8))
    }
    fn from_base_prime_field_elems(xs: impl IntoIterator<Item = Bit>) -> Option<Self> {
        let mut xs = xs.into_iter();
        let mut value = 0;
        for j in 0..128 {
            value |= (xs.next()?.into_bigint().0[0] as u128) << j;
        }
        if xs.next().is_some() {
            None
        } else {
            Some(Self(value))
        }
    }
    fn from_base_prime_field(x: Bit) -> Self {
        Self(x.into_bigint().0[0] as u128)
    }
    fn from_random_bytes_with_flags<F: Flags>(bytes: &[u8]) -> Option<(Self, F)> {
        let extra = usize::from(F::BIT_SIZE != 0);
        if F::BIT_SIZE > 8 || bytes.len() != 16 + extra {
            return None;
        }
        let flag = if extra == 0 { 0 } else { bytes[16] };
        let flags = F::from_u8(flag)?;
        if flags.u8_bitmask() != flag {
            return None;
        }
        Some((
            Self(u128::from_le_bytes(bytes[..16].try_into().ok()?)),
            flags,
        ))
    }
    fn legendre(&self) -> LegendreSymbol {
        if self.is_zero() {
            LegendreSymbol::Zero
        } else {
            LegendreSymbol::QuadraticResidue
        }
    }
    fn sqrt(&self) -> Option<Self> {
        let mut x = *self;
        for _ in 0..127 {
            x.square_in_place();
        }
        Some(x)
    }
    fn square(&self) -> Self {
        Self::from_flock(self.to_flock().square())
    }
    fn square_in_place(&mut self) -> &mut Self {
        *self = self.square();
        self
    }
    fn inverse(&self) -> Option<Self> {
        if self.is_zero() {
            None
        } else {
            Some(self.pow([u64::MAX - 1, u64::MAX]))
        }
    }
    fn inverse_in_place(&mut self) -> Option<&mut Self> {
        *self = self.inverse()?;
        Some(self)
    }
    fn frobenius_map_in_place(&mut self, power: usize) {
        for _ in 0..power % 128 {
            self.square_in_place();
        }
    }
    fn mul_by_base_prime_field(&self, x: &Bit) -> Self {
        if x.is_zero() { Self::ZERO } else { *self }
    }
}
impl Valid for Ghash {
    fn check(&self) -> Result<(), SerializationError> {
        Ok(())
    }
}
impl CanonicalSerialize for Ghash {
    fn serialize_with_mode<W: Write>(
        &self,
        mut w: W,
        _: Compress,
    ) -> Result<(), SerializationError> {
        w.write_all(&self.bytes())?;
        Ok(())
    }
    fn serialized_size(&self, _: Compress) -> usize {
        16
    }
}
impl CanonicalDeserialize for Ghash {
    fn deserialize_with_mode<R: Read>(
        mut r: R,
        _: Compress,
        _: Validate,
    ) -> Result<Self, SerializationError> {
        let mut b = [0; 16];
        r.read_exact(&mut b)?;
        Ok(Self(u128::from_le_bytes(b)))
    }
}
impl CanonicalSerializeWithFlags for Ghash {
    fn serialize_with_flags<W: Write, F: Flags>(
        &self,
        mut w: W,
        flags: F,
    ) -> Result<(), SerializationError> {
        if F::BIT_SIZE > 8 {
            return Err(SerializationError::NotEnoughSpace);
        }
        w.write_all(&self.bytes())?;
        if F::BIT_SIZE != 0 {
            w.write_all(&[flags.u8_bitmask()])?;
        }
        Ok(())
    }
    fn serialized_size_with_flags<F: Flags>(&self) -> usize {
        16 + usize::from(F::BIT_SIZE != 0)
    }
}
impl CanonicalDeserializeWithFlags for Ghash {
    fn deserialize_with_flags<R: Read, F: Flags>(
        mut r: R,
    ) -> Result<(Self, F), SerializationError> {
        if F::BIT_SIZE > 8 {
            return Err(SerializationError::NotEnoughSpace);
        }
        let mut b = vec![0; 16 + usize::from(F::BIT_SIZE != 0)];
        r.read_exact(&mut b)?;
        Self::from_random_bytes_with_flags(&b).ok_or(SerializationError::InvalidData)
    }
}
impl Encoding<[u8]> for Ghash {
    fn encode(&self) -> impl AsRef<[u8]> {
        self.bytes()
    }
}
impl Decoding<[u8]> for Ghash {
    type Repr = [u8; 16];
    fn decode(bytes: [u8; 16]) -> Self {
        Self(u128::from_le_bytes(bytes))
    }
}
impl NargDeserialize for Ghash {
    fn deserialize_from_narg(buf: &mut &[u8]) -> VerificationResult<Self> {
        let b: [u8; 16] = buf
            .get(..16)
            .ok_or(VerificationError)?
            .try_into()
            .map_err(|_| VerificationError)?;
        *buf = &buf[16..];
        Ok(Self(u128::from_le_bytes(b)))
    }
}
