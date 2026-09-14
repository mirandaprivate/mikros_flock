//! Dense k=7 round one, retaining the k=6 factorization and engineering.
//!
//! S and Lambda each have 128 points in GF(256). Each packed row has sixteen
//! bytes and its LDE occupies two ZMM registers. The eight small rows, sixteen
//! medium positions, cached conversion table and split outer weights are the
//! same as in the original implementation. Both outputs omit the factor C_s.

use std::sync::OnceLock;

use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{
    field::{F8, F128, gf2_8::gf8_reduce},
    ntt::{AdditiveNttGf8, InvNttTableByteSingleGf8},
    zerocheck::univariate_skip::{SplitEqGhash, ntt_extend_f128_vec_ghash},
};

use super::{N_INNER, N_MEDIUM, convert_table, d_inv, kernels::accumulate_convert};

#[cfg(target_arch = "x86_64")]
mod x86_64;

const K_SKIP: usize = 7;
const ELL: usize = 128;
const ROW_BYTES: usize = ELL / 8;
const MEDIUM: usize = 1 << N_MEDIUM;

/// Cached GF8 LDE for S={0,...,127}, Lambda={128,...,255}.
pub fn cached_inv_table_k7() -> &'static InvNttTableByteSingleGf8 {
    static TABLE: OnceLock<InvNttTableByteSingleGf8> = OnceLock::new();
    TABLE.get_or_init(|| {
        let input = AdditiveNttGf8::new(K_SKIP, F8::ZERO);
        let output = AdditiveNttGf8::new(K_SKIP, F8(0x80));
        InvNttTableByteSingleGf8::new(&input, &output)
    })
}

#[allow(dead_code)] // independent kernel oracle, also the portable production path
fn shift_reduce_scalar(
    a: &[u8],
    b: &[u8],
    table: &InvNttTableByteSingleGf8,
    base: usize,
    out_low: &mut [u8; 64],
    out_high: &mut [u8; 64],
) {
    let mut left = [F8::ZERO; ELL];
    let mut right = [F8::ZERO; ELL];
    let mut accumulated = [0u16; ELL];
    for small in 0..8 {
        let offset = base + small * ROW_BYTES;
        table.apply(&a[offset..offset + ROW_BYTES], &mut left);
        table.apply(&b[offset..offset + ROW_BYTES], &mut right);
        for lane in 0..ELL {
            accumulated[lane] ^= ((left[lane] * right[lane]).0 as u16) << small;
        }
    }
    for lane in 0..64 {
        out_low[lane] = gf8_reduce(accumulated[lane]);
        out_high[lane] = gf8_reduce(accumulated[lane + 64]);
    }
}

#[inline]
fn shift_reduce(
    a: &[u8],
    b: &[u8],
    table: &InvNttTableByteSingleGf8,
    base: usize,
    out_low: &mut [u8; 64],
    out_high: &mut [u8; 64],
) {
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "gfni",
        target_feature = "avx512f",
        target_feature = "avx512bw"
    ))]
    {
        // SAFETY: compile-time features cover the two-ZMM fused kernel;
        // the top-level function validates all dense input dimensions.
        unsafe { x86_64::shift_reduce(a, b, table, base, out_low, out_high) };
    }
    #[cfg(not(all(
        target_arch = "x86_64",
        target_feature = "gfni",
        target_feature = "avx512f",
        target_feature = "avx512bw"
    )))]
    shift_reduce_scalar(a, b, table, base, out_low, out_high);
}

#[inline]
fn transpose8(mut value: u64) -> u64 {
    let swap = (value ^ (value >> 7)) & 0x00AA_00AA_00AA_00AA;
    value ^= swap ^ (swap << 7);
    let swap = (value ^ (value >> 14)) & 0x0000_CCCC_0000_CCCC;
    value ^= swap ^ (swap << 14);
    let swap = (value ^ (value >> 28)) & 0x0000_0000_F0F0_F0F0;
    value ^ swap ^ (swap << 28)
}

#[allow(dead_code)] // oracle and portable SWAR backend
fn transpose_c_scalar(input: &[u8; 128], low: &mut [u8; 64], high: &mut [u8; 64]) {
    for byte in 0..16 {
        let mut gathered = 0u64;
        for small in 0..8 {
            gathered |= (input[small * 16 + byte] as u64) << (small * 8);
        }
        let target = if byte < 8 { &mut *low } else { &mut *high };
        let start = (byte % 8) * 8;
        target[start..start + 8].copy_from_slice(&transpose8(gathered).to_le_bytes());
    }
}

#[inline]
fn transpose_c(input: &[u8; 128], low: &mut [u8; 64], high: &mut [u8; 64]) {
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "gfni",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "avx512vbmi"
    ))]
    {
        // SAFETY: all GFNI/VBMI features are enabled and the fixed arrays
        // cover exactly the two input and output vectors.
        unsafe { x86_64::transpose_c(input, low, high) };
    }
    #[cfg(not(all(
        target_arch = "x86_64",
        target_feature = "gfni",
        target_feature = "avx512f",
        target_feature = "avx512bw",
        target_feature = "avx512vbmi"
    )))]
    transpose_c_scalar(input, low, high);
}

struct Worker {
    partial_ab: [[F128; 64]; 2],
    partial_c: [[F128; 64]; 2],
    chunk_ab: [[[u8; 64]; MEDIUM]; 2],
    chunk_c: [[[u8; 64]; MEDIUM]; 2],
    result_ab: [F128; ELL],
    result_c: [F128; ELL],
}

impl Worker {
    fn new() -> Self {
        Self {
            partial_ab: [[F128::ZERO; 64]; 2],
            partial_c: [[F128::ZERO; 64]; 2],
            chunk_ab: [[[0; 64]; MEDIUM]; 2],
            chunk_c: [[[0; 64]; MEDIUM]; 2],
            result_ab: [F128::ZERO; ELL],
            result_c: [F128::ZERO; ELL],
        }
    }

    fn process_outer(
        &mut self,
        x_hi: usize,
        eq: &SplitEqGhash,
        eq_lo_scaled: &[F128],
        a: &[u8],
        b: &[u8],
        c: &[u8],
        table: &InvNttTableByteSingleGf8,
        convert: &[F128],
    ) {
        self.partial_ab.fill([F128::ZERO; 64]);
        self.partial_c.fill([F128::ZERO; 64]);
        for (x_lo, &weight) in eq_lo_scaled.iter().enumerate() {
            let outer = x_lo | (x_hi << eq.n_lo);
            let base = (outer << N_INNER) * ROW_BYTES;
            let [ab_low, ab_high] = &mut self.chunk_ab;
            let [c_low, c_high] = &mut self.chunk_c;
            for medium in 0..MEDIUM {
                let medium_base = base + medium * 8 * ROW_BYTES;
                shift_reduce(
                    a,
                    b,
                    table,
                    medium_base,
                    &mut ab_low[medium],
                    &mut ab_high[medium],
                );
                let input = c[medium_base..medium_base + 128].try_into().unwrap();
                transpose_c(input, &mut c_low[medium], &mut c_high[medium]);
            }
            // Reuse the exact optimized 64-lane convert/eq implementation on
            // both register halves. There is no extra copy or regrouping.
            for half in 0..2 {
                accumulate_convert(
                    &self.chunk_ab[half],
                    &self.chunk_c[half],
                    MEDIUM,
                    convert,
                    weight,
                    &mut self.partial_ab[half],
                    &mut self.partial_c[half],
                );
            }
        }
        let high_weight = eq.hi[x_hi];
        for half in 0..2 {
            for lane in 0..64 {
                self.result_ab[half * 64 + lane] += self.partial_ab[half][lane] * high_weight;
                self.result_c[half * 64 + lane] += self.partial_c[half][lane] * high_weight;
            }
        }
    }
}

/// Dense packed k=7 optimized round-one message, omitting C_s in both outputs.
///
/// Requires `m >= 14`, exactly `2^(m-3)` input bytes, and `r.len() == m`.
/// As in the original k=6 entry, callers must place the fixed small challenges
/// at `r[7..10]`, medium challenges at `r[10..14]`, and random outer challenges
/// at `r[14..]`. The first seven coordinates are unused by this round.
/// Returns two 128-element vectors on Lambda; multiplying each by `c_s_f128()`
/// gives the corresponding output of `round1_naive(..., k_skip=7, ...)`.
/// No padding, s_hat_v, commitment or PCS path is included in this entry.
pub fn round1_shift_reduce_extract_c_packed_skip7(
    a: &[u8],
    b: &[u8],
    c: &[u8],
    m: usize,
    r: &[F128],
    table: &InvNttTableByteSingleGf8,
) -> (Vec<F128>, Vec<F128>) {
    assert!(
        m >= K_SKIP + N_INNER && m < usize::BITS as usize,
        "skip7 dense path requires 14 <= m < usize::BITS"
    );
    let bytes = 1usize << (m - 3);
    assert_eq!(a.len(), bytes);
    assert_eq!(b.len(), bytes);
    assert_eq!(c.len(), bytes);
    assert_eq!(r.len(), m);
    assert_eq!(
        (table.k, table.ell, table.n_chunks),
        (K_SKIP, ELL, ROW_BYTES)
    );
    let eq = SplitEqGhash::new(&r[K_SKIP + N_INNER..]);
    let scale = d_inv();
    let eq_lo_scaled: Vec<_> = eq.lo.iter().map(|&weight| weight * scale).collect();
    let convert = convert_table();
    let (ab, c_on_s) = (0..eq.hi.len())
        .into_par_iter()
        .fold(Worker::new, |mut state, outer| {
            state.process_outer(outer, &eq, &eq_lo_scaled, a, b, c, table, convert);
            state
        })
        .map(|state| (state.result_ab, state.result_c))
        .reduce(
            || ([F128::ZERO; ELL], [F128::ZERO; ELL]),
            |(mut a, mut c), (b, d)| {
                for lane in 0..ELL {
                    a[lane] += b[lane];
                    c[lane] += d[lane];
                }
                (a, c)
            },
        );
    (ab.to_vec(), ntt_extend_f128_vec_ghash(&c_on_s, table))
}

#[cfg(test)]
mod tests {
    #[cfg(target_arch = "x86_64")]
    use super::x86_64;
    use super::{
        Worker, cached_inv_table_k7, convert_table, d_inv,
        round1_shift_reduce_extract_c_packed_skip7, transpose_c,
    };
    #[cfg(target_arch = "x86_64")]
    use super::{shift_reduce_scalar, transpose_c_scalar};
    use crate::{
        field::F128,
        test_rng::Rng,
        zerocheck::{
            univariate_skip::{SplitEqGhash, ntt_extend_f128_vec_ghash, pack_bits, round1_naive},
            univariate_skip_optimized::{
                c_s_f128, medium_challenges_ghash, small_challenges_ghash,
            },
        },
    };

    fn challenges(m: usize, outer: &[F128]) -> Vec<F128> {
        let mut r = vec![F128::ZERO; m];
        r[7..10].copy_from_slice(&small_challenges_ghash());
        r[10..14].copy_from_slice(&medium_challenges_ghash());
        r[14..].copy_from_slice(outer);
        r
    }

    #[test]
    fn skip7_round1_matches_naive_ab_and_c_independently() {
        let table = cached_inv_table_k7();
        for m in [14, 15, 16] {
            let mut rng = Rng::new(0x571700 + m as u64);
            let a = rng.bits(1 << m);
            let b = rng.bits(1 << m);
            let c = rng.bits(1 << m); // deliberately independent, including invalid witnesses
            let point = challenges(m, &rng.f128_vec(m - 14));
            let expected = round1_naive(&a, &b, &c, m, 7, &point);
            let actual = round1_shift_reduce_extract_c_packed_skip7(
                &pack_bits(&a),
                &pack_bits(&b),
                &pack_bits(&c),
                m,
                &point,
                table,
            );
            for lane in 0..128 {
                assert_eq!(
                    expected.0[lane],
                    c_s_f128() * actual.0[lane],
                    "AB m={m} lane={lane}"
                );
                assert_eq!(
                    expected.1[lane],
                    c_s_f128() * actual.1[lane],
                    "C m={m} lane={lane}"
                );
            }
        }
    }

    #[test]
    fn skip7_c_transpose_preserves_all_1024_input_bits() {
        for bit in 0..1024 {
            let mut input = [0u8; 128];
            input[bit / 8] = 1 << (bit % 8);
            let mut low = [0; 64];
            let mut high = [0; 64];
            transpose_c(&input, &mut low, &mut high);
            let output: Vec<_> = low.into_iter().chain(high).collect();
            let row = bit / 128;
            let column = bit % 128;
            assert_eq!(output[column], 1 << row);
            assert_eq!(output.iter().filter(|&&value| value != 0).count(), 1);
        }
    }

    #[test]
    fn skip7_parallel_partition_and_boolean_outer_challenges() {
        let m = 16;
        let mut rng = Rng::new(0x5717_AA4A);
        let a = rng.bits(1 << m);
        let b = rng.bits(1 << m);
        let c = rng.bits(1 << m);
        let packed = (pack_bits(&a), pack_bits(&b), pack_bits(&c));
        for outer in [vec![F128::ZERO, F128::ONE], rng.f128_vec(2)] {
            let point = challenges(m, &outer);
            let expected = round1_naive(&a, &b, &c, m, 7, &point);
            for threads in [1, 3] {
                let pool = rayon::ThreadPoolBuilder::new()
                    .num_threads(threads)
                    .build()
                    .unwrap();
                let actual = pool.install(|| {
                    round1_shift_reduce_extract_c_packed_skip7(
                        &packed.0,
                        &packed.1,
                        &packed.2,
                        m,
                        &point,
                        cached_inv_table_k7(),
                    )
                });
                for lane in 0..128 {
                    assert_eq!(expected.0[lane], actual.0[lane] * c_s_f128());
                    assert_eq!(expected.1[lane], actual.1[lane] * c_s_f128());
                }
            }
        }
    }

    #[test]
    fn skip7_forced_outer_low_high_splits_match_naive() {
        // The default split has n_lo=0 on tiny inputs. Explicitly force every
        // split of three outer coordinates to cover the nonzero low-part
        // addressing and both stages of equality weighting used at N=2^30.
        let m = 17;
        let mut rng = Rng::new(0x5717_1011);
        let a = rng.bits(1 << m);
        let b = rng.bits(1 << m);
        let c = rng.bits(1 << m);
        let outer = rng.f128_vec(3);
        let point = challenges(m, &outer);
        let expected = round1_naive(&a, &b, &c, m, 7, &point);
        let (a, b, c) = (pack_bits(&a), pack_bits(&b), pack_bits(&c));
        let table = cached_inv_table_k7();
        for n_hi in 0..=3 {
            let eq = SplitEqGhash::with_n_hi(&outer, n_hi);
            let lo: Vec<_> = eq.lo.iter().map(|&value| value * d_inv()).collect();
            let mut worker = Worker::new();
            for high in 0..eq.hi.len() {
                worker.process_outer(high, &eq, &lo, &a, &b, &c, table, convert_table());
            }
            let lifted = ntt_extend_f128_vec_ghash(&worker.result_c, table);
            for lane in 0..128 {
                assert_eq!(
                    worker.result_ab[lane] * c_s_f128(),
                    expected.0[lane],
                    "AB n_hi={n_hi} lane={lane}"
                );
                assert_eq!(
                    lifted[lane] * c_s_f128(),
                    expected.1[lane],
                    "C n_hi={n_hi} lane={lane}"
                );
            }
        }
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn skip7_hardware_shift_reduce_and_c_transpose_match_portable() {
        if !(std::is_x86_feature_detected!("gfni")
            && std::is_x86_feature_detected!("avx512f")
            && std::is_x86_feature_detected!("avx512bw")
            && std::is_x86_feature_detected!("avx512vbmi"))
        {
            return;
        }
        let mut rng = Rng::new(0x5717_512);
        let mut a = [0u8; 1024];
        let mut b = [0u8; 1024];
        rng.fill_bytes(&mut a);
        rng.fill_bytes(&mut b);
        for base in [0, 128, 256, 768] {
            let mut expected = [[0u8; 64]; 2];
            let mut actual = expected;
            let [lo, hi] = &mut expected;
            shift_reduce_scalar(&a, &b, cached_inv_table_k7(), base, lo, hi);
            let [lo, hi] = &mut actual;
            unsafe { x86_64::shift_reduce(&a, &b, cached_inv_table_k7(), base, lo, hi) };
            assert_eq!(actual, expected);
            let input = a[base..base + 128].try_into().unwrap();
            let [lo, hi] = &mut expected;
            transpose_c_scalar(input, lo, hi);
            let [lo, hi] = &mut actual;
            unsafe { x86_64::transpose_c(input, lo, hi) };
            assert_eq!(actual, expected);
        }
    }
}
