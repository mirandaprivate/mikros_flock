//! Fixed interpolation domains for the intermediate-field Hadamard reduction.
//!
//! H is the first 128 AES-field elements; the certificate uses the first 255
//! elements. Tables depend only on these public domains and are shared between
//! proofs. Flock's 128-point continuation table supplies the native row path.

use std::sync::OnceLock;

use flock_core::ntt::{AdditiveNttGf8, inv_table::InvNttTableByteSingleGf8};
use flock_field::{F8, F128};

use super::field::field;

const KAPPA: usize = 128;
const QUOTIENTS: usize = 127;
const SAMPLES: usize = 255;
const COORDINATES: usize = 16;

pub(crate) struct Domain {
    nodes: [F128; KAPPA],
    denominator_inverses: [F128; KAPPA],
    h_on_lambda: Vec<[u8; KAPPA]>,
    psi_inverse_on_lambda: [u8; QUOTIENTS],
    lambda_basis: Vec<[u8; QUOTIENTS]>,
    row_tables: Vec<[u8; QUOTIENTS]>,
    native_table: InvNttTableByteSingleGf8,
}

pub(crate) fn domain() -> &'static Domain {
    static DOMAIN: OnceLock<Domain> = OnceLock::new();
    DOMAIN.get_or_init(Domain::new)
}

#[cfg(test)]
std::thread_local! {
    static QUOTIENT_GFNI_OVERRIDE: std::cell::Cell<Option<bool>> = const {
        std::cell::Cell::new(None)
    };
}

fn quotient_gfni_requested() -> bool {
    #[cfg(test)]
    if let Some(value) = QUOTIENT_GFNI_OVERRIDE.with(|slot| slot.get()) {
        return value;
    }
    std::env::var_os("MIKROS_HADAMARD_GFNI_QUOTIENT").is_none_or(|value| value == "1")
}

#[cfg(test)]
fn with_quotient_gfni_override<T>(value: bool, run: impl FnOnce() -> T) -> T {
    struct Restore(Option<bool>);
    impl Drop for Restore {
        fn drop(&mut self) {
            QUOTIENT_GFNI_OVERRIDE.with(|slot| slot.set(self.0));
        }
    }
    let _restore = Restore(QUOTIENT_GFNI_OVERRIDE.with(|slot| slot.replace(Some(value))));
    run()
}

impl Domain {
    /// The same Flock continuation, retained in registers for fused consumers.
    ///
    /// # Safety
    /// Requires AVX-512F. This domain owns the fixed k=7 table, and the array
    /// reference provides exactly the sixteen input bytes required by Flock.
    #[cfg(target_arch = "x86_64")]
    #[inline]
    #[target_feature(enable = "avx512f")]
    pub(crate) unsafe fn continuation_registers(
        &self,
        row: &[u8; 16],
    ) -> [std::arch::x86_64::__m512i; 2] {
        // SAFETY: constructor fixes ell=128/n_chunks=16; features declared.
        unsafe {
            self.native_table
                .apply_x86_avx512_128_register_unchecked(row.as_ptr())
        }
    }

    /// Two fixed 128-point continuations with their public chunk schedule
    /// expanded, returning `[a_continuation, b_continuation]`.
    ///
    /// This uses the same compressed 32 KiB table and the same 64 vector loads
    /// as two single-row calls. Expanding the fixed chunk indices removes the
    /// runtime permutation dispatch; interleaving the rows exposes independent
    /// loads and accumulators. Both results include point 255, which callers
    /// must continue to discard when constructing the 255-point certificate.
    ///
    /// # Safety
    /// Requires AVX-512F. Both array references supply complete 16-byte rows.
    /// The constructor fixes the table at 256 rows of 128 bytes, so each input
    /// byte selects a valid row and both 64-byte loads lie entirely inside it.
    #[cfg(target_arch = "x86_64")]
    #[inline]
    #[target_feature(enable = "avx512f")]
    pub(crate) unsafe fn continuation_pair_registers(
        &self,
        a: &[u8; 16],
        b: &[u8; 16],
    ) -> [[std::arch::x86_64::__m512i; 2]; 2] {
        use std::arch::x86_64::{
            _mm512_loadu_si512, _mm512_shuffle_epi32, _mm512_shuffle_i64x2, _mm512_xor_si512,
        };

        // SAFETY: the feature and readable-table contracts are documented
        // above. Every macro invocation has chunk<16 and half in {0,1}; the
        // half exchange for chunks8..15 therefore stays within its LUT row.
        unsafe {
            let base = self.native_table.data_ptr();
            let a0 = base.add(a[0] as usize * 128);
            let b0 = base.add(b[0] as usize * 128);
            let mut left0 = _mm512_loadu_si512(a0.cast());
            let mut right0 = _mm512_loadu_si512(b0.cast());
            let mut left1 = _mm512_loadu_si512(a0.add(64).cast());
            let mut right1 = _mm512_loadu_si512(b0.add(64).cast());

            // chunk is always a literal: both the lane permutation and the
            // half selection below are constants after macro expansion.
            macro_rules! load_permuted {
                ($row:ident, $chunk:literal, $half:literal) => {{
                    let value = _mm512_loadu_si512($row.add(($half ^ ($chunk >> 3)) * 64).cast());
                    let value = match ($chunk >> 1) & 3 {
                        0 => value,
                        1 => _mm512_shuffle_i64x2::<0xB1>(value, value),
                        2 => _mm512_shuffle_i64x2::<0x4E>(value, value),
                        _ => _mm512_shuffle_i64x2::<0x1B>(value, value),
                    };
                    if $chunk & 1 != 0 {
                        _mm512_shuffle_epi32::<0x4E>(value)
                    } else {
                        value
                    }
                }};
            }
            macro_rules! accumulate {
                ($chunk:literal) => {{
                    let ar = base.add(a[$chunk] as usize * 128);
                    let br = base.add(b[$chunk] as usize * 128);
                    left0 = _mm512_xor_si512(left0, load_permuted!(ar, $chunk, 0));
                    right0 = _mm512_xor_si512(right0, load_permuted!(br, $chunk, 0));
                    left1 = _mm512_xor_si512(left1, load_permuted!(ar, $chunk, 1));
                    right1 = _mm512_xor_si512(right1, load_permuted!(br, $chunk, 1));
                }};
            }
            accumulate!(1);
            accumulate!(2);
            accumulate!(3);
            accumulate!(4);
            accumulate!(5);
            accumulate!(6);
            accumulate!(7);
            accumulate!(8);
            accumulate!(9);
            accumulate!(10);
            accumulate!(11);
            accumulate!(12);
            accumulate!(13);
            accumulate!(14);
            accumulate!(15);
            [[left0, left1], [right0, right1]]
        }
    }

    fn new() -> Self {
        let f = field();
        let h: Vec<_> = (0..KAPPA).map(|x| x as u8).collect();
        let lambda: Vec<_> = (KAPPA..SAMPLES).map(|x| x as u8).collect();
        let psi = vanishing_polynomial(&h);
        let (h_basis, h_denominator_inverses) = lagrange_basis(&h);
        let (lambda_basis, _) = lagrange_basis(&lambda);
        let h_on_lambda: Vec<[u8; KAPPA]> = lambda
            .iter()
            .map(|&x| std::array::from_fn(|i| evaluate(&h_basis[i], x)))
            .collect();
        let psi_inverse_on_lambda = std::array::from_fn(|i| {
            let value = evaluate(&psi, lambda[i]);
            assert_ne!(value, 0, "the interpolation domains must be disjoint");
            f.inv(value)
        });
        let lambda_basis = lambda_basis
            .into_iter()
            .map(|p| p.try_into().expect("127-point interpolation basis"))
            .collect();

        // Each table maps one witness byte to its 127 off-domain samples.
        // Building a subset from its lowest bit takes only 127 byte XORs.
        let mut row_tables = vec![[0; QUOTIENTS]; COORDINATES * 256];
        for chunk in 0..COORDINATES {
            for byte in 1usize..256 {
                let bit = byte.trailing_zeros() as usize;
                let previous = chunk * 256 + (byte & (byte - 1));
                let current = chunk * 256 + byte;
                for (sample, weights) in h_on_lambda.iter().enumerate() {
                    row_tables[current][sample] =
                        row_tables[previous][sample] ^ weights[chunk * 8 + bit];
                }
            }
        }
        let ntt_h = AdditiveNttGf8::new(7, F8::ZERO);
        let ntt_lambda = AdditiveNttGf8::new(7, F8(128));
        Self {
            nodes: std::array::from_fn(|i| f.embed(i as u8)),
            denominator_inverses: std::array::from_fn(|i| f.embed(h_denominator_inverses[i])),
            h_on_lambda,
            psi_inverse_on_lambda,
            lambda_basis,
            row_tables,
            native_table: InvNttTableByteSingleGf8::new(&ntt_h, &ntt_lambda),
        }
    }

    /// The 128 Lagrange weights on H, including challenges that belong to H.
    /// Prefix/suffix products avoid inversions and their exceptional zero case.
    pub(crate) fn lagrange_at(&self, x: F128) -> Vec<F128> {
        let mut prefix = [F128::ONE; KAPPA + 1];
        for (i, &node) in self.nodes.iter().enumerate() {
            prefix[i + 1] = prefix[i] * (x + node);
        }
        let mut suffix = F128::ONE;
        let mut weights = vec![F128::ZERO; KAPPA];
        for i in (0..KAPPA).rev() {
            weights[i] = prefix[i] * suffix * self.denominator_inverses[i];
            suffix *= x + self.nodes[i];
        }
        weights
    }

    pub(crate) fn vanishing_at(&self, x: F128) -> F128 {
        self.nodes
            .iter()
            .fold(F128::ONE, |product, &node| product * (x + node))
    }

    pub(crate) fn contains(&self, x: F128) -> bool {
        self.nodes.contains(&x)
    }

    /// Turn the 255 coordinate-wise aggregate samples into h's 127 monomial
    /// coefficients, in ascending degree order. In characteristic two,
    /// subtraction of the interpolated column polynomial is byte XOR.
    pub(crate) fn quotient(&self, samples: &[[u8; COORDINATES]]) -> Vec<F128> {
        assert_eq!(samples.len(), SAMPLES);
        #[cfg(target_arch = "x86_64")]
        if quotient_gfni_requested() && std::is_x86_feature_detected!("gfni") {
            // SAFETY: x86_64 provides SSE2; GFNI is checked at runtime. The
            // dimension check above covers every 16-byte sample load.
            return unsafe { self.quotient_gfni(samples) };
        }
        self.quotient_scalar(samples)
    }

    /// Original multiplication-table implementation, also the portable fallback.
    fn quotient_scalar(&self, samples: &[[u8; COORDINATES]]) -> Vec<F128> {
        let f = field();
        let mut h_values = vec![[0u8; COORDINATES]; QUOTIENTS];
        for (i, (h_value, weights)) in h_values.iter_mut().zip(&self.h_on_lambda).enumerate() {
            let mut c_value = [0u8; COORDINATES];
            for (&weight, column) in weights.iter().zip(&samples[..KAPPA]) {
                for (output, &coordinate) in c_value.iter_mut().zip(column) {
                    *output ^= f.mul(weight, coordinate);
                }
            }
            for (coordinate, output) in h_value.iter_mut().enumerate() {
                *output = f.mul(
                    samples[KAPPA + i][coordinate] ^ c_value[coordinate],
                    self.psi_inverse_on_lambda[i],
                );
            }
        }
        let mut coefficients = vec![[0u8; COORDINATES]; QUOTIENTS];
        for (value, basis) in h_values.iter().zip(&self.lambda_basis) {
            for (coefficient, &weight) in coefficients.iter_mut().zip(basis) {
                for (output, &coordinate) in coefficient.iter_mut().zip(value) {
                    *output ^= f.mul(weight, coordinate);
                }
            }
        }
        coefficients
            .iter()
            .map(|coefficient| f.compose(coefficient))
            .collect()
    }

    /// Vectorize the existing two GF256 matrix products across all 16 field
    /// coordinates. Four output accumulators share each input load, without
    /// adding coefficient tables or changing the final GHASH basis embedding.
    #[cfg(target_arch = "x86_64")]
    #[target_feature(enable = "gfni,sse2")]
    unsafe fn quotient_gfni(&self, samples: &[[u8; COORDINATES]]) -> Vec<F128> {
        use std::arch::x86_64::{
            _mm_gf2p8mul_epi8, _mm_loadu_si128, _mm_set1_epi8, _mm_setzero_si128, _mm_storeu_si128,
            _mm_xor_si128,
        };

        debug_assert_eq!(samples.len(), SAMPLES);
        let mut h_values = vec![[0u8; COORDINATES]; QUOTIENTS];
        let mut coefficients = vec![[0u8; COORDINATES]; QUOTIENTS];
        // SAFETY: all source and destination rows contain exactly 16 bytes.
        // The last output tile has three rows; every matrix access and vector
        // load/store is restricted to the initialized `take` rows.
        unsafe {
            for start in (0..QUOTIENTS).step_by(4) {
                let take = (QUOTIENTS - start).min(4);
                let mut sums = [_mm_setzero_si128(); 4];
                for (column, sample) in samples[..KAPPA].iter().enumerate() {
                    let value = _mm_loadu_si128(sample.as_ptr().cast());
                    for lane in 0..take {
                        let scalar = _mm_set1_epi8(self.h_on_lambda[start + lane][column] as i8);
                        sums[lane] = _mm_xor_si128(sums[lane], _mm_gf2p8mul_epi8(value, scalar));
                    }
                }
                for lane in 0..take {
                    let index = start + lane;
                    let aggregate = _mm_loadu_si128(samples[KAPPA + index].as_ptr().cast());
                    let inverse = _mm_set1_epi8(self.psi_inverse_on_lambda[index] as i8);
                    let value = _mm_gf2p8mul_epi8(_mm_xor_si128(aggregate, sums[lane]), inverse);
                    _mm_storeu_si128(h_values[index].as_mut_ptr().cast(), value);
                }
            }
            for start in (0..QUOTIENTS).step_by(4) {
                let take = (QUOTIENTS - start).min(4);
                let mut sums = [_mm_setzero_si128(); 4];
                for (index, sample) in h_values.iter().enumerate() {
                    let value = _mm_loadu_si128(sample.as_ptr().cast());
                    for lane in 0..take {
                        let scalar = _mm_set1_epi8(self.lambda_basis[index][start + lane] as i8);
                        sums[lane] = _mm_xor_si128(sums[lane], _mm_gf2p8mul_epi8(value, scalar));
                    }
                }
                for lane in 0..take {
                    _mm_storeu_si128(coefficients[start + lane].as_mut_ptr().cast(), sums[lane]);
                }
            }
        }
        let f = field();
        coefficients
            .iter()
            .map(|coefficient| f.compose(coefficient))
            .collect()
    }

    /// Portable byte lookup path; witness bit j is coefficient j's H value.
    pub(crate) fn eval_row(&self, row: &[u8; COORDINATES]) -> [u8; SAMPLES] {
        let mut samples = [0u8; SAMPLES];
        copy_h_values(row, &mut samples);
        for (chunk, &byte) in row.iter().enumerate() {
            let table = &self.row_tables[chunk * 256 + byte as usize];
            for (output, &value) in samples[KAPPA..].iter_mut().zip(table) {
                *output ^= value;
            }
        }
        samples
    }

    /// Reuse Flock's compressed continuation table and native dispatch.
    /// Its final point is 255; that point is outside our 255-sample domain.
    pub(crate) fn eval_row_native(&self, row: &[u8; COORDINATES]) -> [u8; SAMPLES] {
        let mut continuation = [F8::ZERO; KAPPA];
        self.native_table.apply(row, &mut continuation);
        let mut samples = [0u8; SAMPLES];
        copy_h_values(row, &mut samples);
        for (output, value) in samples[KAPPA..].iter_mut().zip(continuation) {
            *output = value.0;
        }
        samples
    }
}

fn copy_h_values(row: &[u8; COORDINATES], samples: &mut [u8; SAMPLES]) {
    for (i, output) in samples[..KAPPA].iter_mut().enumerate() {
        *output = (row[i / 8] >> (i % 8)) & 1;
    }
}

fn evaluate(coefficients: &[u8], x: u8) -> u8 {
    let f = field();
    coefficients
        .iter()
        .rev()
        .fold(0, |value, &c| f.mul(value, x) ^ c)
}

pub(crate) fn vanishing_polynomial(nodes: &[u8]) -> Vec<u8> {
    let f = field();
    let mut polynomial = vec![1];
    for &node in nodes {
        let mut next = vec![0; polynomial.len() + 1];
        for (i, &coefficient) in polynomial.iter().enumerate() {
            next[i] ^= f.mul(node, coefficient);
            next[i + 1] ^= coefficient;
        }
        polynomial = next;
    }
    polynomial
}

pub(crate) fn lagrange_basis(nodes: &[u8]) -> (Vec<Vec<u8>>, Vec<u8>) {
    let f = field();
    let polynomial = vanishing_polynomial(nodes);
    let n = nodes.len();
    let mut bases = Vec::with_capacity(n);
    let mut inverses = Vec::with_capacity(n);
    for &node in nodes {
        let mut quotient = vec![0; n];
        quotient[n - 1] = polynomial[n];
        for i in (1..n).rev() {
            quotient[i - 1] = polynomial[i] ^ f.mul(node, quotient[i]);
        }
        let denominator = evaluate(&quotient, node);
        assert_ne!(denominator, 0, "interpolation nodes must be distinct");
        let inverse = f.inv(denominator);
        for coefficient in &mut quotient {
            *coefficient = f.mul(*coefficient, inverse);
        }
        inverses.push(inverse);
        bases.push(quotient);
    }
    (bases, inverses)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flock_field::{phi8, test_rng::Rng};
    use rayon::prelude::*;

    fn oracle_evaluate(polynomial: &[F8], x: F8) -> F8 {
        polynomial.iter().rev().fold(F8::ZERO, |v, &c| v * x + c)
    }

    fn oracle_multiply(left: &[F8], right: &[F8]) -> Vec<F8> {
        let mut product = vec![F8::ZERO; left.len() + right.len() - 1];
        for (i, &a) in left.iter().enumerate() {
            for (j, &b) in right.iter().enumerate() {
                product[i + j] += a * b;
            }
        }
        product
    }

    // Independent incremental Newton interpolation, using F8 arithmetic
    // rather than the production multiplication and Lagrange tables.
    fn oracle_interpolate(values: &[F8]) -> Vec<F8> {
        let mut polynomial = vec![F8::ZERO; values.len()];
        let mut basis = vec![F8::ONE];
        for (i, &value) in values.iter().enumerate() {
            let node = F8(i as u8);
            let scale =
                (value + oracle_evaluate(&polynomial, node)) * oracle_evaluate(&basis, node).inv();
            for (output, &coefficient) in polynomial.iter_mut().zip(&basis) {
                *output += scale * coefficient;
            }
            basis = oracle_multiply(&basis, &[node, F8::ONE]);
        }
        polynomial
    }

    fn oracle_row(row: &[u8; COORDINATES]) -> Vec<F8> {
        oracle_interpolate(
            &(0..KAPPA)
                .map(|i| F8((row[i / 8] >> (i % 8)) & 1))
                .collect::<Vec<_>>(),
        )
    }

    #[test]
    fn row_tables_and_native_continuation_match_newton_oracle() {
        let d = domain();
        let mut rng = Rng::new(0x2551_2812);
        let mut rows = vec![[0; COORDINATES], [255; COORDINATES]];
        for i in [0, 7, 8, 63, 64, 119, 127] {
            let mut row = [0; COORDINATES];
            row[i / 8] = 1 << (i % 8);
            rows.push(row);
        }
        for _ in 0..4 {
            let mut row = [0; COORDINATES];
            rng.fill_bytes(&mut row);
            rows.push(row);
        }
        for row in rows {
            let polynomial = oracle_row(&row);
            let expected: [u8; SAMPLES] =
                std::array::from_fn(|i| oracle_evaluate(&polynomial, F8(i as u8)).0);
            assert_eq!(d.eval_row(&row), expected);
            assert_eq!(d.eval_row_native(&row), expected);
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn paired_register_continuation_matches_single_and_scalar_all_lanes() {
        if !std::is_x86_feature_detected!("avx512f") {
            return;
        }
        let d = domain();
        let mut rng = Rng::new(0x5041_4952);
        let mut rows = vec![[0u8; COORDINATES], [255u8; COORDINATES]];
        for bit in 0..KAPPA {
            let mut row = [0u8; COORDINATES];
            row[bit / 8] = 1 << (bit % 8);
            rows.push(row);
        }
        for _ in 0..64 {
            let mut row = [0u8; COORDINATES];
            rng.fill_bytes(&mut row);
            rows.push(row);
        }
        for (a, b) in rows.iter().zip(rows.iter().rev()) {
            // SAFETY: runtime ISA check above; both complete rows and the
            // fixed domain satisfy the table and input bounds contracts.
            let pair = unsafe { d.continuation_pair_registers(a, b) };
            for (registers, row) in pair.into_iter().zip([a, b]) {
                // SAFETY: two initialized 64-byte vectors have exactly the
                // layout/size of 128 bytes, and every byte bit pattern is valid.
                let actual: [u8; KAPPA] = unsafe { std::mem::transmute(registers) };
                // SAFETY: same checked feature and complete-row conditions.
                let single: [u8; KAPPA] =
                    unsafe { std::mem::transmute(d.continuation_registers(row)) };
                let mut scalar = [F8::ZERO; KAPPA];
                d.native_table.apply_scalar(row, &mut scalar);
                assert_eq!(actual, single);
                assert_eq!(actual, scalar.map(|value| value.0));
                // Check all 128 lanes here, including the extra point255.
                assert_eq!(&actual[..QUOTIENTS], &d.eval_row(row)[KAPPA..]);
            }
        }
    }

    #[test]
    fn extension_lagrange_weights_match_independent_polynomial() {
        let d = domain();
        let mut rng = Rng::new(0x1281_2716);
        let mut row = [0; COORDINATES];
        rng.fill_bytes(&mut row);
        let polynomial = oracle_row(&row);
        let mut points: Vec<_> = (0..256).map(|i| phi8(F8(i as u8))).collect();
        points.extend([rng.f128(), rng.f128()]);
        let psi = (0..KAPPA).fold(vec![F8::ONE], |p, i| {
            oracle_multiply(&p, &[F8(i as u8), F8::ONE])
        });
        for (index, x) in points.into_iter().enumerate() {
            let weights = d.lagrange_at(x);
            assert_eq!(
                weights.iter().copied().fold(F128::ZERO, |a, b| a + b),
                F128::ONE
            );
            let actual = weights.iter().enumerate().fold(F128::ZERO, |sum, (i, &w)| {
                if (row[i / 8] >> (i % 8)) & 1 == 1 {
                    sum + w
                } else {
                    sum
                }
            });
            let expected = polynomial
                .iter()
                .rev()
                .fold(F128::ZERO, |v, &c| v * x + phi8(c));
            assert_eq!(actual, expected);
            let expected_psi = psi.iter().rev().fold(F128::ZERO, |v, &c| v * x + phi8(c));
            assert_eq!(d.vanishing_at(x), expected_psi);
            assert_eq!(d.contains(x), index < KAPPA);
            if index < KAPPA {
                for (i, weight) in weights.into_iter().enumerate() {
                    assert_eq!(weight, if i == index { F128::ONE } else { F128::ZERO });
                }
            }
        }
    }

    #[test]
    fn quotient_gfni_matches_scalar_on_coordinate_lanes_and_threads() {
        let mut cases = vec![
            vec![[0u8; COORDINATES]; SAMPLES],
            vec![[1u8; COORDINATES]; SAMPLES],
            vec![[u8::MAX; COORDINATES]; SAMPLES],
        ];
        // Exercise every coordinate separately, including high-bit byte values
        // and the final three-output SIMD tile at coefficient/sample 124..126.
        for coordinate in 0..COORDINATES {
            let mut samples = vec![[0u8; COORDINATES]; SAMPLES];
            for (index, sample) in samples.iter_mut().enumerate() {
                sample[coordinate] = (index as u8).wrapping_mul(17).wrapping_add(0x80);
            }
            cases.push(samples);
        }
        let mut rng = Rng::new(0x5155_4F54_4746_4E49);
        for _ in 0..4 {
            let mut samples = vec![[0u8; COORDINATES]; SAMPLES];
            for sample in &mut samples {
                rng.fill_bytes(sample);
            }
            cases.push(samples);
        }
        let d = domain();
        for threads in [1, 3] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .unwrap();
            pool.install(|| {
                cases.par_iter().enumerate().for_each(|(case, samples)| {
                    // Overrides are local to the worker currently evaluating this
                    // case; no environment variable is mutated by parallel tests.
                    let expected = with_quotient_gfni_override(false, || d.quotient(samples));
                    let actual = with_quotient_gfni_override(true, || d.quotient(samples));
                    assert_eq!(actual, expected, "threads={threads}, case={case}");
                })
            });
        }
    }

    #[test]
    fn quotient_matches_direct_polynomial_product_and_long_division() {
        let d = domain();
        let f = field();
        let mut rng = Rng::new(0x2551_2716);
        let mut samples = vec![[0u8; COORDINATES]; SAMPLES];
        let mut expected = vec![F128::ZERO; QUOTIENTS];
        let psi = (0..KAPPA).fold(vec![F8::ONE], |p, i| {
            oracle_multiply(&p, &[F8(i as u8), F8::ONE])
        });
        for enabled in [false, true] {
            assert_eq!(
                with_quotient_gfni_override(enabled, || d.quotient(&samples)),
                expected
            );
        }
        for _ in 0..3 {
            let mut a = [0; COORDINATES];
            let mut b = [0; COORDINATES];
            rng.fill_bytes(&mut a);
            rng.fill_bytes(&mut b);
            let weight = rng.f128();
            let coordinates = f.decompose(weight);
            let a_samples = d.eval_row(&a);
            let b_samples = d.eval_row_native(&b);
            for ((sample, a), b) in samples.iter_mut().zip(a_samples).zip(b_samples) {
                let product = F8(a) * F8(b);
                for (output, &coordinate) in sample.iter_mut().zip(&coordinates) {
                    *output ^= (product * F8(coordinate)).0;
                }
            }
            let a_poly = oracle_row(&a);
            let b_poly = oracle_row(&b);
            let c: [u8; COORDINATES] = std::array::from_fn(|i| a[i] & b[i]);
            let c_poly = oracle_row(&c);
            let mut residual = oracle_multiply(&a_poly, &b_poly);
            for (output, &coefficient) in residual.iter_mut().zip(&c_poly) {
                *output += coefficient;
            }
            for degree in (KAPPA..SAMPLES).rev() {
                let coefficient = residual[degree];
                expected[degree - KAPPA] += weight * phi8(coefficient);
                for (i, &divisor) in psi.iter().enumerate() {
                    residual[degree - KAPPA + i] += coefficient * divisor;
                }
            }
            assert!(residual.iter().all(|&c| c == F8::ZERO));
            for enabled in [false, true] {
                assert_eq!(
                    with_quotient_gfni_override(enabled, || d.quotient(&samples)),
                    expected
                );
            }
        }
    }
}
