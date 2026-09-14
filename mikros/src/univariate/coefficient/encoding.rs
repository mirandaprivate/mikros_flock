//! Binary evaluations to monomial coefficients, directly in bit-plane form.
//!
//! Plane `p`, bit `i` is bit `p` of the AES GF(256) coefficient of `X^i`.
//! Each table entry is therefore exactly 128 bytes. A byte table uses 512 KiB;
//! the independent nibble table uses 64 KiB and twice as many lookups. Both
//! implement the same fixed binary-linear map. Their relative speed must be
//! measured: reducing the working set also increases table traffic per row.
//!
//! Only public Lagrange polynomials are prepared. Reading a witness and
//! combining its table entries remain online work. In particular, preparing
//! the nibble path does not initialize the byte table or the parent's separate
//! coefficient table. Any other caches initialized by the caller are additional.

use std::sync::OnceLock;

use crate::univariate::{KAPPA, domain};

const ROW_BYTES: usize = 16;
const PLANES: usize = 8;

/// Eight AES coefficient bits, each spread over 128 polynomial degrees.
pub(crate) type Planes = [u128; PLANES];
/// A and B coefficient planes for one binary row.
pub(crate) type Pair = [Planes; 2];

// Alignment adds no padding to a 128-byte entry. Its two ZMM loads then each
// fit one cache line rather than splitting lines at a 16-byte offset.
#[derive(Clone, Copy)]
#[repr(C, align(64))]
struct Entry(Planes);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Encoding {
    Byte,
    Nibble,
}

#[cfg(test)]
impl Encoding {
    /// Size of this encoding's persistent lookup entries, excluding shared
    /// field tables and allocator metadata. The two modes initialize separately.
    pub(crate) const fn table_bytes(self) -> usize {
        match self {
            Self::Byte => 16 * 256 * size_of::<Planes>(),
            Self::Nibble => 32 * 16 * size_of::<Planes>(),
        }
    }
}

/// Initialize the default 64 KiB public encoding table.
pub(crate) fn prepare() {
    prepare_mode(Encoding::Nibble);
}

/// Initialize only the selected public encoding table. Use this before the
/// online timer when reporting a run with public tables already prepared.
pub(crate) fn prepare_mode(mode: Encoding) {
    let _ = table(mode);
}

fn table(mode: Encoding) -> &'static [Entry] {
    static BYTE: OnceLock<Box<[Entry]>> = OnceLock::new();
    static NIBBLE: OnceLock<Box<[Entry]>> = OnceLock::new();
    match mode {
        Encoding::Byte => BYTE.get_or_init(build_table::<8>),
        Encoding::Nibble => NIBBLE.get_or_init(build_table::<4>),
    }
}

fn build_table<const BITS: usize>() -> Box<[Entry]> {
    assert!(BITS == 4 || BITS == 8);
    let nodes: Vec<_> = (0..KAPPA).map(|node| node as u8).collect();
    let (basis, _) = domain::lagrange_basis(&nodes);
    let basis: Vec<_> = basis
        .iter()
        .map(|coefficients| {
            let coefficients: &[u8; KAPPA] = coefficients
                .as_slice()
                .try_into()
                .expect("128 coefficients per fixed Lagrange polynomial");
            from_bytes(coefficients)
        })
        .collect();
    let entries = 1 << BITS;
    // Heap allocation avoids placing the 512 KiB byte table on a worker stack.
    let mut output = vec![Entry([0; PLANES]); (KAPPA / BITS) * entries].into_boxed_slice();
    for chunk in 0..KAPPA / BITS {
        for value in 1usize..entries {
            let previous = output[chunk * entries + (value & (value - 1))].0;
            let image = &basis[chunk * BITS + value.trailing_zeros() as usize];
            output[chunk * entries + value].0 = std::array::from_fn(|p| previous[p] ^ image[p]);
        }
    }
    output
}

/// Encode four consecutive 128-bit A/B rows. The result order is
/// `[row][A_or_B][AES_bit]`; degree indices remain the bit positions of `u128`.
/// Only AVX-512F is needed by the native table-XOR path. Unsupported hosts
/// perform the same lookup operations with portable integer XORs.
#[inline]
pub(crate) fn encode_four(a: &[u8; 64], b: &[u8; 64], mode: Encoding) -> [Pair; 4] {
    let table = table(mode);
    #[cfg(target_arch = "x86_64")]
    if std::is_x86_feature_detected!("avx512f") {
        // SAFETY: feature detection covers every native instruction; the array
        // references contain four full rows, and the selected table has the
        // matching number of complete 128-byte entries.
        return unsafe {
            match mode {
                Encoding::Byte => native::four::<8>(a, b, table),
                Encoding::Nibble => native::four::<4>(a, b, table),
            }
        };
    }
    match mode {
        Encoding::Byte => portable_four::<8>(a, b, table),
        Encoding::Nibble => portable_four::<4>(a, b, table),
    }
}

/// Encode one A/B row, for a partial final batch without padding it to four.
#[inline]
pub(crate) fn encode_pair(a: &[u8; 16], b: &[u8; 16], mode: Encoding) -> Pair {
    let table = table(mode);
    #[cfg(target_arch = "x86_64")]
    if std::is_x86_feature_detected!("avx512f") {
        // SAFETY: feature detection and fixed-size arrays satisfy native::pair.
        return unsafe {
            match mode {
                Encoding::Byte => native::pair::<8>(a, b, table),
                Encoding::Nibble => native::pair::<4>(a, b, table),
            }
        };
    }
    match mode {
        Encoding::Byte => portable_pair::<8>(a, b, table),
        Encoding::Nibble => portable_pair::<4>(a, b, table),
    }
}

#[inline]
fn entry_index<const BITS: usize>(row: &[u8], chunk: usize) -> usize {
    let value = (usize::from(row[chunk * BITS / 8]) >> (chunk * BITS % 8)) & ((1 << BITS) - 1);
    (chunk << BITS) + value
}

fn portable_pair<const BITS: usize>(a: &[u8; 16], b: &[u8; 16], table: &[Entry]) -> Pair {
    let mut output = [[0; PLANES]; 2];
    for chunk in 0..KAPPA / BITS {
        let left = &table[entry_index::<BITS>(a, chunk)].0;
        let right = &table[entry_index::<BITS>(b, chunk)].0;
        for p in 0..PLANES {
            output[0][p] ^= left[p];
            output[1][p] ^= right[p];
        }
    }
    output
}

fn portable_four<const BITS: usize>(a: &[u8; 64], b: &[u8; 64], table: &[Entry]) -> [Pair; 4] {
    std::array::from_fn(|row| {
        portable_pair::<BITS>(
            a[row * ROW_BYTES..(row + 1) * ROW_BYTES]
                .try_into()
                .expect("complete binary row"),
            b[row * ROW_BYTES..(row + 1) * ROW_BYTES]
                .try_into()
                .expect("complete binary row"),
            table,
        )
    })
}

/// Decode coefficient planes to their 128 AES GF(256) bytes. This also accepts
/// high-product planes: output `i` then represents degree `128+i`. All 128
/// slots are preserved, including slot 127; the multiplication caller must
/// independently verify/discard its mathematically zero degree-255 slot.
#[inline]
pub(crate) fn to_bytes(planes: &Planes) -> [u8; KAPPA] {
    let mut output = [0; KAPPA];
    #[cfg(target_endian = "little")]
    for (half, stripe) in output.chunks_exact_mut(64).enumerate() {
        let lanes = std::array::from_fn(|p| (planes[p] >> (64 * half)) as u64);
        // Reuse Flock's shared 8-by-64 transpose, including its native backend.
        flock_core::bits::transpose_8_u64s_to_64_bytes(&lanes, stripe);
    }
    #[cfg(target_endian = "big")]
    for (degree, coefficient) in output.iter_mut().enumerate() {
        for (p, plane) in planes.iter().enumerate() {
            *coefficient |= (((plane >> degree) & 1) as u8) << p;
        }
    }
    output
}

/// Convert bytes to planes when preparing public tables. Online binary-row
/// encoding never uses this conversion: it combines bit-plane entries directly.
pub(crate) fn from_bytes(coefficients: &[u8; KAPPA]) -> Planes {
    let mut output = [0; PLANES];
    for (degree, &coefficient) in coefficients.iter().enumerate() {
        for (p, plane) in output.iter_mut().enumerate() {
            *plane |= u128::from((coefficient >> p) & 1) << degree;
        }
    }
    output
}

#[cfg(target_arch = "x86_64")]
mod native {
    use std::arch::x86_64::{
        __m512i, _mm512_loadu_si512, _mm512_setzero_si512, _mm512_storeu_si512, _mm512_xor_si512,
    };

    use super::{Entry, KAPPA, Pair, entry_index};

    /// Caller checks AVX-512F and provides the table corresponding to BITS.
    #[target_feature(enable = "avx512f")]
    pub(super) unsafe fn pair<const BITS: usize>(
        a: &[u8; 16],
        b: &[u8; 16],
        table: &[Entry],
    ) -> Pair {
        let mut a_low = _mm512_setzero_si512();
        let mut a_high = _mm512_setzero_si512();
        let mut b_low = _mm512_setzero_si512();
        let mut b_high = _mm512_setzero_si512();
        for chunk in 0..KAPPA / BITS {
            let left = table[entry_index::<BITS>(a, chunk)].0.as_ptr();
            let right = table[entry_index::<BITS>(b, chunk)].0.as_ptr();
            // SAFETY: each table entry has eight u128s. Each unaligned load
            // covers exactly four, at offsets zero and four respectively.
            unsafe {
                a_low = _mm512_xor_si512(a_low, _mm512_loadu_si512(left.cast::<__m512i>()));
                a_high =
                    _mm512_xor_si512(a_high, _mm512_loadu_si512(left.add(4).cast::<__m512i>()));
                b_low = _mm512_xor_si512(b_low, _mm512_loadu_si512(right.cast::<__m512i>()));
                b_high =
                    _mm512_xor_si512(b_high, _mm512_loadu_si512(right.add(4).cast::<__m512i>()));
            }
        }
        let mut output = [[0; 8]; 2];
        // SAFETY: the four stores cover distinct halves of the two initialized
        // 128-byte outputs, with no alignment requirement.
        unsafe {
            _mm512_storeu_si512(output[0].as_mut_ptr().cast::<__m512i>(), a_low);
            _mm512_storeu_si512(output[0].as_mut_ptr().add(4).cast::<__m512i>(), a_high);
            _mm512_storeu_si512(output[1].as_mut_ptr().cast::<__m512i>(), b_low);
            _mm512_storeu_si512(output[1].as_mut_ptr().add(4).cast::<__m512i>(), b_high);
        }
        output
    }

    /// Four rows retain sixteen independent ZMM XOR accumulators. Explicit
    /// local variables keep row selection out of the inner accumulator loop;
    /// table entries are loaded directly in their final coefficient-plane form.
    /// Caller checks AVX-512F and provides the table corresponding to BITS.
    #[target_feature(enable = "avx512f")]
    pub(super) unsafe fn four<const BITS: usize>(
        a: &[u8; 64],
        b: &[u8; 64],
        table: &[Entry],
    ) -> [Pair; 4] {
        let zero = _mm512_setzero_si512();
        let (mut a0l, mut a0h, mut b0l, mut b0h) = (zero, zero, zero, zero);
        let (mut a1l, mut a1h, mut b1l, mut b1h) = (zero, zero, zero, zero);
        let (mut a2l, mut a2h, mut b2l, mut b2h) = (zero, zero, zero, zero);
        let (mut a3l, mut a3h, mut b3l, mut b3h) = (zero, zero, zero, zero);
        for chunk in 0..KAPPA / BITS {
            macro_rules! add_row {
                ($row:literal, $al:ident, $ah:ident, $bl:ident, $bh:ident) => {{
                    let left = table[entry_index::<BITS>(&a[16 * $row..16 * ($row + 1)], chunk)]
                        .0
                        .as_ptr();
                    let right = table[entry_index::<BITS>(&b[16 * $row..16 * ($row + 1)], chunk)]
                        .0
                        .as_ptr();
                    // SAFETY: the const row is in 0..4 and the checked table
                    // references each contain the two complete 64-byte halves.
                    unsafe {
                        $al = _mm512_xor_si512($al, _mm512_loadu_si512(left.cast::<__m512i>()));
                        $ah = _mm512_xor_si512(
                            $ah,
                            _mm512_loadu_si512(left.add(4).cast::<__m512i>()),
                        );
                        $bl = _mm512_xor_si512($bl, _mm512_loadu_si512(right.cast::<__m512i>()));
                        $bh = _mm512_xor_si512(
                            $bh,
                            _mm512_loadu_si512(right.add(4).cast::<__m512i>()),
                        );
                    }
                }};
            }
            add_row!(0, a0l, a0h, b0l, b0h);
            add_row!(1, a1l, a1h, b1l, b1h);
            add_row!(2, a2l, a2h, b2l, b2h);
            add_row!(3, a3l, a3h, b3l, b3h);
        }
        let mut output = [[[0; 8]; 2]; 4];
        macro_rules! store_row {
            ($row:literal, $al:ident, $ah:ident, $bl:ident, $bh:ident) => {{
                // SAFETY: the const row is in 0..4; stores cover distinct
                // halves of A and B, each an initialized 128-byte array.
                unsafe {
                    let left = output[$row][0].as_mut_ptr();
                    let right = output[$row][1].as_mut_ptr();
                    _mm512_storeu_si512(left.cast::<__m512i>(), $al);
                    _mm512_storeu_si512(left.add(4).cast::<__m512i>(), $ah);
                    _mm512_storeu_si512(right.cast::<__m512i>(), $bl);
                    _mm512_storeu_si512(right.add(4).cast::<__m512i>(), $bh);
                }
            }};
        }
        store_row!(0, a0l, a0h, b0l, b0h);
        store_row!(1, a1l, a1h, b1l, b1h);
        store_row!(2, a2l, a2h, b2l, b2h);
        store_row!(3, a3l, a3h, b3l, b3h);
        output
    }
}

#[cfg(test)]
mod tests {
    use flock_field::F8;

    use super::*;

    // Incremental Newton interpolation is independent of production's
    // vanishing-polynomial synthetic division and its multiplication tables.
    fn oracle_interpolate(row: &[u8; ROW_BYTES]) -> [u8; KAPPA] {
        fn evaluate(polynomial: &[F8], x: F8) -> F8 {
            polynomial.iter().rev().fold(F8::ZERO, |v, &c| v * x + c)
        }
        let mut polynomial = [F8::ZERO; KAPPA];
        let mut basis = vec![F8::ONE];
        for i in 0..KAPPA {
            let x = F8(i as u8);
            let value = F8((row[i / 8] >> (i % 8)) & 1);
            let scale = (value + evaluate(&polynomial, x)) * evaluate(&basis, x).inv();
            for (coefficient, &b) in polynomial.iter_mut().zip(&basis) {
                *coefficient += scale * b;
            }
            let mut next = vec![F8::ZERO; basis.len() + 1];
            for (j, &b) in basis.iter().enumerate() {
                next[j] += b * x;
                next[j + 1] += b;
            }
            basis = next;
        }
        polynomial.map(|coefficient| coefficient.0)
    }

    fn oracle_evaluate(coefficients: &[u8; KAPPA], x: u8) -> u8 {
        coefficients
            .iter()
            .rev()
            .fold(F8::ZERO, |value, &c| value * F8(x) + F8(c))
            .0
    }

    fn random_byte(state: &mut u64) -> u8 {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
        (*state >> 23) as u8
    }

    #[test]
    fn coefficient_encoding_tables_have_separate_measured_footprints() {
        prepare();
        assert_eq!(Encoding::Nibble.table_bytes(), 64 * 1024);
        assert_eq!(Encoding::Byte.table_bytes(), 512 * 1024);
        assert_eq!(size_of::<Entry>(), size_of::<Planes>());
        assert_eq!(align_of::<Entry>(), 64);
        for mode in [Encoding::Nibble, Encoding::Byte] {
            prepare_mode(mode);
            assert_eq!(size_of_val(table(mode)), mode.table_bytes());
            assert_eq!(table(mode).as_ptr() as usize % 64, 0);
        }
    }

    #[test]
    fn coefficient_encoding_all_input_bits_match_independent_newton_oracle() {
        for bit in 0..KAPPA {
            let mut row = [0; ROW_BYTES];
            row[bit / 8] = 1 << (bit % 8);
            let oracle = oracle_interpolate(&row);
            for mode in [Encoding::Nibble, Encoding::Byte] {
                let result = encode_pair(&row, &[0; ROW_BYTES], mode);
                assert_eq!(to_bytes(&result[0]), oracle, "bit={bit}, mode={mode:?}");
                assert_eq!(result[1], [0; PLANES]);
            }
            for x in 0..KAPPA {
                assert_eq!(oracle_evaluate(&oracle, x as u8), u8::from(x == bit));
            }
        }
    }

    #[test]
    fn coefficient_encoding_random_rows_and_off_domain_evaluations_match_oracle() {
        let mut state = 0x8ccf_a20e_b14d_7319;
        let mut rows = vec![
            [0; ROW_BYTES],
            [255; ROW_BYTES],
            [0x55; ROW_BYTES],
            [0xaa; ROW_BYTES],
        ];
        rows.extend((0..12).map(|_| std::array::from_fn(|_| random_byte(&mut state))));
        for row in rows {
            let oracle = oracle_interpolate(&row);
            for mode in [Encoding::Nibble, Encoding::Byte] {
                let planes = encode_pair(&row, &row, mode);
                assert_eq!(planes[0], planes[1]);
                let coefficients = to_bytes(&planes[0]);
                assert_eq!(coefficients, oracle);
                for x in 0..256 {
                    let expected = if x < KAPPA {
                        (row[x / 8] >> (x % 8)) & 1
                    } else {
                        oracle_evaluate(&oracle, x as u8)
                    };
                    assert_eq!(oracle_evaluate(&coefficients, x as u8), expected);
                }
            }
        }
    }

    #[test]
    fn coefficient_encoding_four_rows_preserve_row_source_and_degree_order() {
        let mut state = 0x7213_e845_ba10_d03e;
        for sample in 0..24 {
            let a: [u8; 64] = std::array::from_fn(|_| random_byte(&mut state));
            let b: [u8; 64] = std::array::from_fn(|_| random_byte(&mut state));
            let nibble = encode_four(&a, &b, Encoding::Nibble);
            let byte = encode_four(&a, &b, Encoding::Byte);
            assert_eq!(nibble, byte, "sample={sample}");
            assert_eq!(nibble, portable_four::<4>(&a, &b, table(Encoding::Nibble)));
            assert_eq!(byte, portable_four::<8>(&a, &b, table(Encoding::Byte)));
            for (row, planes) in nibble.iter().enumerate() {
                let a: &[u8; 16] = a[row * 16..(row + 1) * 16].try_into().unwrap();
                let b: &[u8; 16] = b[row * 16..(row + 1) * 16].try_into().unwrap();
                assert_eq!(*planes, encode_pair(a, b, Encoding::Nibble));
                assert_eq!(*planes, encode_pair(a, b, Encoding::Byte));
                assert_eq!(*planes, portable_pair::<4>(a, b, table(Encoding::Nibble)));
                assert_eq!(*planes, portable_pair::<8>(a, b, table(Encoding::Byte)));
                if sample == 0 {
                    assert_eq!(to_bytes(&planes[0]), oracle_interpolate(a));
                    assert_eq!(to_bytes(&planes[1]), oracle_interpolate(b));
                }
            }
        }
    }

    #[test]
    fn coefficient_encoding_transpose_preserves_every_plane_and_high_slot() {
        for p in 0..PLANES {
            for degree in 0..KAPPA {
                let mut planes = [0; PLANES];
                planes[p] = 1u128 << degree;
                let mut expected = [0; KAPPA];
                expected[degree] = 1 << p;
                assert_eq!(to_bytes(&planes), expected, "plane={p}, degree={degree}");
                assert_eq!(from_bytes(&expected), planes);
            }
        }
        let mut state = 0x832d_9204_4ced_b671;
        for _ in 0..32 {
            let coefficients = std::array::from_fn(|_| random_byte(&mut state));
            assert_eq!(to_bytes(&from_bytes(&coefficients)), coefficients);
        }
        assert_eq!(to_bytes(&[u128::MAX; PLANES]), [255; KAPPA]);
    }

    #[test]
    fn coefficient_encoding_all_input_bits_match_native_four_when_available() {
        #[cfg(target_arch = "x86_64")]
        if std::is_x86_feature_detected!("avx512f") {
            for bit in 0..KAPPA {
                let mut a = [0; 64];
                let mut b = [0; 64];
                for row in 0..4 {
                    let a_bit = (bit + 17 * row) % KAPPA;
                    let b_bit = (127 + 13 * row + KAPPA - bit) % KAPPA;
                    a[16 * row + a_bit / 8] = 1 << (a_bit % 8);
                    b[16 * row + b_bit / 8] = 1 << (b_bit % 8);
                }
                // SAFETY: the runtime feature check and matching fixed tables
                // satisfy the native four-row entry point's contract.
                let nibble = unsafe { native::four::<4>(&a, &b, table(Encoding::Nibble)) };
                let byte = unsafe { native::four::<8>(&a, &b, table(Encoding::Byte)) };
                assert_eq!(nibble, portable_four::<4>(&a, &b, table(Encoding::Nibble)));
                assert_eq!(byte, portable_four::<8>(&a, &b, table(Encoding::Byte)));
                assert_eq!(nibble, byte);
            }
        }
    }
}
