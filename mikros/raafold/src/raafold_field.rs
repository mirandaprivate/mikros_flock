//! Flattened binary extension field used by RAAFold.
//!
//! Multiplication uses the direct RustCrypto POLYVAL arithmetic on CLMUL
//! CPUs, with the original RustCrypto backend as a portable fallback.
//! Bulk products use four VPCLMUL lanes when available. The field retains
//! its scalar/width-one trait view and does not implement a binary tower.

use core::{
    fmt::{Debug, Display, Formatter},
    iter::{Product, Sum},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use binius_field::{
    Divisible, Field, Maskable, PackedField, WideMul,
    arithmetic_traits::{InvertOrZero, Square},
};
use binius_utils::{
    DeserializeBytes, FixedSizeSerializeBytes, SerializationError, SerializeBytes,
    bytes::{Buf, BufMut},
};
use bytemuck::Zeroable;
use polyval::{
    Block, Key, Polyval,
    universal_hash::{KeyInit, UniversalHash},
};
use rand::distr::{Distribution, StandardUniform};

#[cfg(target_arch = "x86_64")]
#[path = "field_x86.rs"]
mod x86;

/// An ordinary 256-bit polynomial product, before POLYVAL's linear
/// Montgomery reduction. XOR accumulation followed by one reduction is
/// exactly the sum of the individual POLYVAL `dot` products.
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub struct PolyvalWideProduct([u64; 4]);

impl Add for PolyvalWideProduct {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(core::array::from_fn(|i| self.0[i] ^ rhs.0[i]))
    }
}

impl AddAssign for PolyvalWideProduct {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for PolyvalWideProduct {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        self + rhs
    }
}

impl SubAssign for PolyvalWideProduct {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self += rhs;
    }
}

impl Sum for PolyvalWideProduct {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::default(), Add::add)
    }
}

impl PolyvalWideProduct {
    /// Reduces the four polynomial limbs with the same two elimination
    /// steps as RustCrypto polyval 0.6.2's CLMUL kernel.
    #[inline]
    pub fn reduce(self) -> RustCryptoPolyvalField {
        let [v0, mut v1, mut v2, mut v3] = self.0;
        v2 ^= v0 ^ (v0 >> 1) ^ (v0 >> 2) ^ (v0 >> 7);
        v1 ^= (v0 << 63) ^ (v0 << 62) ^ (v0 << 57);
        v3 ^= v1 ^ (v1 >> 1) ^ (v1 >> 2) ^ (v1 >> 7);
        v2 ^= (v1 << 63) ^ (v1 << 62) ^ (v1 << 57);
        RustCryptoPolyvalField(u128::from(v2) | (u128::from(v3) << 64))
    }
}

#[inline]
fn rustcrypto_multiply(a: u128, b: u128) -> u128 {
    let key = Key::from(a.to_le_bytes());
    let block = Block::from(b.to_le_bytes());
    let mut backend = Polyval::new(&key);
    backend.update(&[block]);
    u128::from_le_bytes(backend.finalize().into())
}

fn portable_wide_multiply(a: u128, b: u128) -> PolyvalWideProduct {
    fn clmul(a: u64, b: u64) -> u128 {
        let mut product = 0;
        for bit in 0..64 {
            product ^= (u128::from(a) << bit) & 0u128.wrapping_sub(u128::from((b >> bit) & 1));
        }
        product
    }
    let (a0, a1) = (a as u64, (a >> 64) as u64);
    let (b0, b1) = (b as u64, (b >> 64) as u64);
    let lo = clmul(a0, b0);
    let hi = clmul(a1, b1);
    let mid = clmul(a0 ^ a1, b0 ^ b1) ^ lo ^ hi;
    PolyvalWideProduct([
        lo as u64,
        (lo >> 64) as u64 ^ mid as u64,
        hi as u64 ^ (mid >> 64) as u64,
        (hi >> 64) as u64,
    ])
}

#[inline]
fn wide_multiply(a: u128, b: u128) -> PolyvalWideProduct {
    #[cfg(target_arch = "x86_64")]
    if std::is_x86_feature_detected!("pclmulqdq") {
        // SAFETY: the CPU feature was checked immediately above.
        return unsafe { x86::wide_multiply(a, b) };
    }
    portable_wide_multiply(a, b)
}

#[inline]
fn has_x4() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        std::is_x86_feature_detected!("avx512f")
            && std::is_x86_feature_detected!("avx512bw")
            && std::is_x86_feature_detected!("vpclmulqdq")
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}

/// A flattened GF(2^128) element in the POLYVAL polynomial basis.
///
/// The stored bits are a little-endian POLYVAL block. RustCrypto implements
/// the RFC 8452 `dot` operation, whose multiplication is polynomial
/// multiplication followed by a fixed `x^-128` factor. Consequently the raw
/// representation of the multiplicative identity is `x^128 mod p`, not `1`.
#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct RustCryptoPolyvalField(u128);

// SAFETY: zero is the additive identity and every u128 bit pattern is a valid
// element of the flattened binary field.
unsafe impl Zeroable for RustCryptoPolyvalField {}

impl RustCryptoPolyvalField {
    /// Constructs a field element from its polynomial coefficient bits.
    pub const fn new(value: u128) -> Self {
        Self(value)
    }

    /// Returns the polynomial coefficient bits.
    pub const fn val(self) -> u128 {
        self.0
    }

    // SAFETY: repr(transparent) gives precisely u128's layout and alignment;
    // every bit pattern is valid and these views preserve the original borrow.
    #[inline]
    fn bits(values: &[Self]) -> &[u128] {
        unsafe { core::slice::from_raw_parts(values.as_ptr().cast(), values.len()) }
    }

    #[inline]
    fn bits_mut(values: &mut [Self]) -> &mut [u128] {
        unsafe { core::slice::from_raw_parts_mut(values.as_mut_ptr().cast(), values.len()) }
    }

    /// Sum of pairwise products, with one final reduction. Callers can split
    /// large vectors into Rayon chunks and sum the returned field elements.
    pub fn dot_product(left: &[Self], right: &[Self]) -> Self {
        Self::dot_product_bits(Self::bits(left), right)
    }

    /// The left side contains raw POLYVAL `dot` coordinates, not GHASH bits.
    /// No conversion, allocation, or change of vector order is performed.
    pub fn dot_product_bits(left: &[u128], right: &[Self]) -> Self {
        assert_eq!(left.len(), right.len(), "POLYVAL dot dimensions");
        let right = Self::bits(right);
        let mut start = 0;
        let mut wide = PolyvalWideProduct::default();
        #[cfg(target_arch = "x86_64")]
        if left.len() >= 4 && has_x4() {
            start = left.len() / 4 * 4;
            // SAFETY: features checked, equal lengths, complete four-lane blocks.
            wide = unsafe { x86::dot_x4(&left[..start], &right[..start]) };
        }
        for (&a, &b) in left[start..].iter().zip(&right[start..]) {
            wide += wide_multiply(a, b);
        }
        wide.reduce()
    }

    /// Multiplies a slice by a constant without allocating an output vector.
    pub fn mul_assign_slice(values: &mut [Self], factor: Self) {
        if factor == Self::ONE {
            return;
        }
        if factor == Self::ZERO {
            values.fill(Self::ZERO);
            return;
        }
        let mut start = 0;
        #[cfg(target_arch = "x86_64")]
        if values.len() >= 4 && has_x4() {
            start = values.len() / 4 * 4;
            // SAFETY: features checked and the prefix contains whole lanes.
            unsafe { x86::multiply_assign_x4(Self::bits_mut(&mut values[..start]), factor.0) };
        }
        for value in &mut values[start..] {
            *value *= factor;
        }
    }

    /// Fused `dst[i] += src[i] * factor`, preserving element order.
    pub fn add_scaled_slice(dst: &mut [Self], src: &[Self], factor: Self) {
        Self::add_scaled_bits_slice(dst, Self::bits(src), factor);
    }

    /// Fused multiply-add from raw POLYVAL coordinates. The source must not
    /// contain GHASH coefficient bits; those require the explicit field map.
    pub fn add_scaled_bits_slice(dst: &mut [Self], src: &[u128], factor: Self) {
        assert_eq!(dst.len(), src.len(), "POLYVAL scaled-add dimensions");
        if factor == Self::ZERO {
            return;
        }
        if factor == Self::ONE {
            for (dst, &src) in dst.iter_mut().zip(src) {
                dst.0 ^= src;
            }
            return;
        }
        let mut start = 0;
        #[cfg(target_arch = "x86_64")]
        if dst.len() >= 4 && has_x4() {
            start = dst.len() / 4 * 4;
            // SAFETY: features, equal lengths and complete lanes checked above.
            unsafe {
                x86::add_scaled_x4(Self::bits_mut(&mut dst[..start]), &src[..start], factor.0)
            };
        }
        for (dst, &src) in dst[start..].iter_mut().zip(&src[start..]) {
            *dst += Self(src) * factor;
        }
    }

    /// Replaces `low[i]` by `(1 + r) * low[i] + r * high[i]`.
    ///
    /// Characteristic two permits the equivalent single-product form
    /// `low[i] + (low[i] + high[i]) * r`. Complete SIMD blocks are read and
    /// written once; scalar tails and portable CPUs use the same formula.
    /// Safe slice borrows ensure the mutable output cannot alias `high`.
    pub fn fold_equality_slice(low: &mut [Self], high: &[Self], r: Self) {
        assert_eq!(low.len(), high.len(), "POLYVAL equality-fold dimensions");
        if r == Self::ZERO {
            return;
        }
        if r == Self::ONE {
            low.copy_from_slice(high);
            return;
        }
        let mut start = 0;
        #[cfg(target_arch = "x86_64")]
        if low.len() >= 4 && has_x4() {
            start = low.len() / 4 * 4;
            // SAFETY: features and dimensions checked above; the disjoint
            // borrows preserve non-aliasing and the prefix has whole lanes.
            unsafe {
                x86::fold_equality_x4(
                    Self::bits_mut(&mut low[..start]),
                    Self::bits(&high[..start]),
                    r.0,
                )
            };
        }
        for (low, high) in low[start..].iter_mut().zip(&high[start..]) {
            *low += (*low + *high) * r;
        }
    }
}

impl From<u128> for RustCryptoPolyvalField {
    #[inline]
    fn from(value: u128) -> Self {
        Self(value)
    }
}

impl From<RustCryptoPolyvalField> for u128 {
    #[inline]
    fn from(value: RustCryptoPolyvalField) -> Self {
        value.0
    }
}

impl Neg for RustCryptoPolyvalField {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self
    }
}

macro_rules! impl_xor_operation {
    ($trait:ident, $method:ident) => {
        impl $trait for RustCryptoPolyvalField {
            type Output = Self;

            #[inline]
            #[allow(clippy::suspicious_arithmetic_impl)]
            fn $method(self, rhs: Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }

        impl $trait<&Self> for RustCryptoPolyvalField {
            type Output = Self;

            #[inline]
            #[allow(clippy::suspicious_arithmetic_impl)]
            fn $method(self, rhs: &Self) -> Self::Output {
                Self(self.0 ^ rhs.0)
            }
        }
    };
}

impl_xor_operation!(Add, add);
impl_xor_operation!(Sub, sub);

macro_rules! impl_xor_assign_operation {
    ($trait:ident, $method:ident, $operator:tt) => {
        impl $trait for RustCryptoPolyvalField {
            #[inline]
            fn $method(&mut self, rhs: Self) {
                self.0 $operator rhs.0;
            }
        }

        impl $trait<&Self> for RustCryptoPolyvalField {
            #[inline]
            fn $method(&mut self, rhs: &Self) {
                self.0 $operator rhs.0;
            }
        }
    };
}

impl_xor_assign_operation!(AddAssign, add_assign, ^=);
impl_xor_assign_operation!(SubAssign, sub_assign, ^=);

impl Mul for RustCryptoPolyvalField {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        #[cfg(target_arch = "x86_64")]
        if std::is_x86_feature_detected!("pclmulqdq") {
            // SAFETY: direct arithmetic uses the original RustCrypto reduction.
            return Self(unsafe { x86::multiply(self.0, rhs.0) });
        }
        Self(rustcrypto_multiply(self.0, rhs.0))
    }
}

impl Mul<&Self> for RustCryptoPolyvalField {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: &Self) -> Self::Output {
        self * *rhs
    }
}

impl MulAssign for RustCryptoPolyvalField {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl MulAssign<&Self> for RustCryptoPolyvalField {
    #[inline]
    fn mul_assign(&mut self, rhs: &Self) {
        *self = *self * *rhs;
    }
}

impl Sum for RustCryptoPolyvalField {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Add::add)
    }
}

impl<'a> Sum<&'a Self> for RustCryptoPolyvalField {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Product for RustCryptoPolyvalField {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
    }
}

impl<'a> Product<&'a Self> for RustCryptoPolyvalField {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl Square for RustCryptoPolyvalField {
    #[inline]
    fn square(self) -> Self {
        self * self
    }
}

impl InvertOrZero for RustCryptoPolyvalField {
    #[inline]
    fn invert_or_zero(self) -> Self {
        // RAAFold is inversion-free. This complete fallback exists only
        // because Binius' generic Field trait requires it.
        if self == Self::ZERO {
            return Self::ZERO;
        }
        Field::pow(&self, [u64::MAX - 1, u64::MAX])
    }
}

impl WideMul for RustCryptoPolyvalField {
    type Output = PolyvalWideProduct;

    #[inline]
    fn wide_mul(a: Self, b: Self) -> Self::Output {
        wide_multiply(a.0, b.0)
    }

    #[inline]
    fn reduce(wide: Self::Output) -> Self {
        wide.reduce()
    }
}

impl Divisible<Self> for RustCryptoPolyvalField {
    const LOG_N: usize = 0;

    #[inline]
    fn value_iter(value: Self) -> impl ExactSizeIterator<Item = Self> + Send + Clone {
        core::iter::once(value)
    }

    #[inline]
    fn ref_iter(value: &Self) -> impl ExactSizeIterator<Item = Self> + Send + Clone + '_ {
        core::iter::once(*value)
    }

    #[inline]
    fn slice_iter(slice: &[Self]) -> impl ExactSizeIterator<Item = Self> + Send + Clone + '_ {
        slice.iter().copied()
    }

    #[inline]
    unsafe fn get_unchecked(&self, _index: usize) -> Self {
        *self
    }

    #[inline]
    unsafe fn set_unchecked(&mut self, _index: usize, value: Self) {
        *self = value;
    }

    #[inline]
    fn broadcast(value: Self) -> Self {
        value
    }

    #[inline]
    fn from_iter(mut iter: impl Iterator<Item = Self>) -> Self {
        iter.next().unwrap_or(Self::ZERO)
    }
}

impl Maskable<Self> for RustCryptoPolyvalField {
    type Mask = bool;

    #[inline]
    fn make_mask(mut selectors: impl Iterator<Item = bool>) -> Self::Mask {
        selectors.next().unwrap_or(false)
    }

    #[inline]
    fn select(&self, mask: &Self::Mask) -> Self {
        if *mask { *self } else { Self::ZERO }
    }
}

impl PackedField for RustCryptoPolyvalField {
    #[inline]
    fn from_fn(mut f: impl FnMut(usize) -> Self::Scalar) -> Self {
        f(0)
    }

    fn interleave(self, _other: Self, _log_block_len: usize) -> (Self, Self) {
        panic!("cannot interleave a width-one packed field")
    }

    fn unzip(self, _other: Self, _log_block_len: usize) -> (Self, Self) {
        panic!("cannot unzip a width-one packed field")
    }

    #[inline]
    unsafe fn spread_unchecked(self, _log_block_len: usize, _block_idx: usize) -> Self {
        self
    }
}

impl Distribution<RustCryptoPolyvalField> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> RustCryptoPolyvalField {
        RustCryptoPolyvalField(Distribution::<u128>::sample(self, rng))
    }
}

impl Display for RustCryptoPolyvalField {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "0x{:032x}", self.0)
    }
}

impl Debug for RustCryptoPolyvalField {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "RustCryptoPolyvalField({self})")
    }
}

impl SerializeBytes for RustCryptoPolyvalField {
    fn serialize(&self, write_buf: impl BufMut) -> Result<(), SerializationError> {
        self.0.serialize(write_buf)
    }
}

impl DeserializeBytes for RustCryptoPolyvalField {
    fn deserialize(read_buf: impl Buf) -> Result<Self, SerializationError> {
        Ok(Self(u128::deserialize(read_buf)?))
    }
}

impl FixedSizeSerializeBytes for RustCryptoPolyvalField {
    const BYTE_SIZE: usize = 16;
}

impl Field for RustCryptoPolyvalField {
    const ZERO: Self = Self(0);
    const ONE: Self = Self(0xc200_0000_0000_0000_0000_0000_0000_0001);
    const CHARACTERISTIC: usize = 2;
    const ORDER_EXPONENT: usize = 128;
    const MULTIPLICATIVE_GENERATOR: Self = Self(0x4600_0000_0000_0000_0000_0000_0000_0003);

    #[inline]
    fn double(&self) -> Self {
        Self::ZERO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn next_bits(state: &mut u128) -> u128 {
        *state ^= *state << 23;
        *state ^= *state >> 17;
        *state ^= *state << 26;
        *state
    }

    fn reference_mul(mut left: u128, mut right: u128) -> u128 {
        const REDUCTION: u128 = (1 << 127) | (1 << 126) | (1 << 121) | 1;
        let mut result = 0u128;
        while right != 0 {
            if right & 1 == 1 {
                result ^= left;
            }
            right >>= 1;
            let high = left >> 127;
            left <<= 1;
            if high == 1 {
                left ^= REDUCTION;
            }
        }
        result
    }

    fn reference_dot(left: u128, right: u128) -> u128 {
        const DOT_FACTOR: u128 = 0x9204_0000_0000_0000_0000_0000_0000_0001;
        reference_mul(reference_mul(left, right), DOT_FACTOR)
    }

    #[test]
    fn direct_wide_and_portable_match_all_basis_products() {
        for a in 0..128 {
            for b in 0..128 {
                let left = 1u128 << a;
                let right = 1u128 << b;
                let expected = reference_dot(left, right);
                assert_eq!(
                    (RustCryptoPolyvalField(left) * RustCryptoPolyvalField(right)).val(),
                    expected
                );
                assert_eq!(wide_multiply(left, right).reduce().val(), expected);
                assert_eq!(portable_wide_multiply(left, right).reduce().val(), expected);
                assert_eq!(rustcrypto_multiply(left, right), expected);
            }
        }
        // A genuinely unreduced product: polynomial 1*1 is 1, whereas its
        // POLYVAL dot reduction includes the nontrivial x^-128 factor.
        assert_eq!(wide_multiply(1, 1), PolyvalWideProduct([1, 0, 0, 0]));
        assert_ne!(wide_multiply(1, 1).reduce().val(), 1);
    }

    #[test]
    fn direct_field_matches_original_backend_and_independent_oracle() {
        let mut state = 0x4c9a_328f_8b01_479d_7a26_c854_09de_f613;
        let mut accumulated = PolyvalWideProduct::default();
        let mut expected_sum = RustCryptoPolyvalField::ZERO;
        for _ in 0..8192 {
            let a = next_bits(&mut state);
            let b = next_bits(&mut state);
            let expected = reference_dot(a, b);
            assert_eq!(rustcrypto_multiply(a, b), expected);
            assert_eq!(
                (RustCryptoPolyvalField(a) * RustCryptoPolyvalField(b)).val(),
                expected
            );
            assert_eq!(portable_wide_multiply(a, b).reduce().val(), expected);
            accumulated += wide_multiply(a, b);
            expected_sum += RustCryptoPolyvalField(expected);
        }
        assert_eq!(accumulated.reduce(), expected_sum);
        assert_eq!(
            (accumulated + accumulated).reduce(),
            RustCryptoPolyvalField::ZERO
        );
    }

    #[test]
    fn bulk_dot_and_scaled_add_match_independent_oracle_with_tails() {
        type F = RustCryptoPolyvalField;
        let mut state = 0x6708_387b_d627_7a62_f802_16b4_a956_0e19;
        for n in [0, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 127, 128, 129, 4097] {
            let left: Vec<_> = (0..n).map(|_| F::new(next_bits(&mut state))).collect();
            let right: Vec<_> = (0..n).map(|_| F::new(next_bits(&mut state))).collect();
            let raw: Vec<_> = left.iter().map(|x| x.val()).collect();
            let expected = left
                .iter()
                .zip(&right)
                .fold(0, |v, (a, b)| v ^ reference_dot(a.val(), b.val()));
            assert_eq!(
                F::dot_product(&left, &right).val(),
                expected,
                "dot length {n}"
            );
            assert_eq!(
                F::dot_product_bits(&raw, &right).val(),
                expected,
                "raw dot length {n}"
            );
            for factor in [F::ZERO, F::ONE, F::new(next_bits(&mut state))] {
                let scaled: Vec<_> = left
                    .iter()
                    .map(|a| F::new(reference_dot(a.val(), factor.val())))
                    .collect();
                let mut actual = left.clone();
                F::mul_assign_slice(&mut actual, factor);
                assert_eq!(actual, scaled, "scale length {n}");
                let expected: Vec<_> = right.iter().zip(&scaled).map(|(a, b)| *a + *b).collect();
                let mut actual = right.clone();
                F::add_scaled_slice(&mut actual, &left, factor);
                assert_eq!(actual, expected, "saxpy length {n}");
                let mut actual = right.clone();
                F::add_scaled_bits_slice(&mut actual, &raw, factor);
                assert_eq!(actual, expected, "raw saxpy length {n}");
                let expected: Vec<_> = left
                    .iter()
                    .zip(&right)
                    .map(|(a, b)| F::new(a.val() ^ reference_dot(a.val() ^ b.val(), factor.val())))
                    .collect();
                let high_before = right.clone();
                let mut actual = left.clone();
                F::fold_equality_slice(&mut actual, &right, factor);
                assert_eq!(actual, expected, "equality fold length {n}");
                assert_eq!(
                    right, high_before,
                    "equality fold preserves high length {n}"
                );
            }
        }
    }

    #[test]
    fn bulk_field_kernels_reject_dimension_mismatch() {
        type F = RustCryptoPolyvalField;
        assert!(std::panic::catch_unwind(|| F::dot_product(&[F::ONE], &[])).is_err());
        assert!(std::panic::catch_unwind(|| F::dot_product_bits(&[], &[F::ONE])).is_err());
        assert!(
            std::panic::catch_unwind(|| F::add_scaled_slice(&mut [F::ZERO], &[], F::ZERO)).is_err()
        );
        assert!(
            std::panic::catch_unwind(|| F::add_scaled_bits_slice(&mut [], &[1], F::ONE)).is_err()
        );
        for r in [F::ZERO, F::ONE, F::new(0x23)] {
            assert!(
                std::panic::catch_unwind(|| F::fold_equality_slice(&mut [F::ZERO], &[], r))
                    .is_err()
            );
            assert!(
                std::panic::catch_unwind(|| F::fold_equality_slice(&mut [], &[F::ZERO], r))
                    .is_err()
            );
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn explicit_x86_paths_match_portable_montgomery_reduction() {
        if !std::is_x86_feature_detected!("pclmulqdq") {
            return;
        }
        let mut state = 0x03c8_61a5_067f_1d8b_9956_32e0_1ca4_d927;
        for _ in 0..1024 {
            let a = next_bits(&mut state);
            let b = next_bits(&mut state);
            let expected = portable_wide_multiply(a, b);
            // SAFETY: CLMUL checked above.
            assert_eq!(unsafe { x86::wide_multiply(a, b) }, expected);
            assert_eq!(unsafe { x86::multiply(a, b) }, expected.reduce().val());
        }
        if !has_x4() {
            return;
        }
        let a: Vec<_> = (0..68).map(|_| next_bits(&mut state)).collect();
        let b: Vec<_> = (0..68).map(|_| next_bits(&mut state)).collect();
        let expected = a
            .iter()
            .zip(&b)
            .fold(PolyvalWideProduct::default(), |sum, (&a, &b)| {
                sum + portable_wide_multiply(a, b)
            });
        // SAFETY: all SIMD feature checks passed and 68 is divisible by four.
        assert_eq!(unsafe { x86::dot_x4(&a, &b) }, expected);
        let factor = next_bits(&mut state);
        let expected_scaled: Vec<_> = a.iter().map(|&a| reference_dot(a, factor)).collect();
        let mut actual = a.clone();
        unsafe {
            x86::multiply_assign_x4(&mut actual, factor);
        }
        assert_eq!(actual, expected_scaled);
        let mut actual = b.clone();
        unsafe {
            x86::add_scaled_x4(&mut actual, &a, factor);
        }
        assert_eq!(
            actual,
            b.iter()
                .zip(&expected_scaled)
                .map(|(b, a)| b ^ a)
                .collect::<Vec<_>>()
        );
        // Exercise the SIMD arithmetic itself even for ZERO/ONE, for which
        // the public wrapper normally selects its no-op/copy fast paths.
        for r in [0, RustCryptoPolyvalField::ONE.val(), factor] {
            let expected: Vec<_> = a
                .iter()
                .zip(&b)
                .map(|(&a, &b)| a ^ reference_dot(a ^ b, r))
                .collect();
            let mut actual = a.clone();
            let high_before = b.clone();
            // SAFETY: all SIMD feature checks passed and 68 is divisible by four.
            unsafe { x86::fold_equality_x4(&mut actual, &b, r) };
            assert_eq!(actual, expected);
            assert_eq!(b, high_before);
        }
    }

    #[test]
    fn polyval_backend_matches_flattened_field_arithmetic() {
        let mut left = 0x0123_4567_89ab_cdef_fedc_ba98_7654_3210u128;
        let mut right = 0xfedc_ba98_7654_3210_0123_4567_89ab_cdefu128;
        for _ in 0..128 {
            let actual = u128::from(
                RustCryptoPolyvalField::from(left) * RustCryptoPolyvalField::from(right),
            );
            assert_eq!(actual, reference_dot(left, right));
            left = left.rotate_left(17) ^ 0x9e37_79b9_7f4a_7c15_6a09_e667_f3bc_c909;
            right = right.rotate_right(29) ^ left;
        }
    }

    #[test]
    fn flattened_field_has_expected_identity_and_generator() {
        assert_eq!(
            RustCryptoPolyvalField::ONE * RustCryptoPolyvalField::ONE,
            RustCryptoPolyvalField::ONE
        );
        let value = RustCryptoPolyvalField::from(0xdead_beef_cafe_babe_0123_4567_89ab_cdef);
        assert_eq!(value * RustCryptoPolyvalField::ONE, value);
        assert_eq!(
            value * RustCryptoPolyvalField::ZERO,
            RustCryptoPolyvalField::ZERO
        );
        assert_eq!(
            RustCryptoPolyvalField::MULTIPLICATIVE_GENERATOR,
            RustCryptoPolyvalField::from(0x4600_0000_0000_0000_0000_0000_0000_0003)
        );

        const ORDER: u128 = u128::MAX;
        const PRIME_FACTORS: [u128; 9] = [
            3,
            5,
            17,
            257,
            641,
            65_537,
            274_177,
            6_700_417,
            67_280_421_310_721,
        ];
        for factor in PRIME_FACTORS {
            let exponent = ORDER / factor;
            assert_ne!(
                Field::pow(
                    &RustCryptoPolyvalField::MULTIPLICATIVE_GENERATOR,
                    [exponent as u64, (exponent >> 64) as u64],
                ),
                RustCryptoPolyvalField::ONE,
                "generator order is missing prime factor {factor}"
            );
        }
    }
}
