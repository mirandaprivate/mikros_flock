//! Four Russians dot products on the actual column-major commitment bits.
//! Split eq avoids a full N-element F128 weight table; four-bit subset sums
//! replace individual bit branches. All outputs equal the scalar adapter.
use flock_core::zerocheck::univariate_skip::SplitEqGhash;
use flock_field::F128;
use kernel::bits::PackedBits;
use rayon::prelude::*;

pub(crate) fn column_evals(bits: &PackedBits, point: &[F128]) -> Vec<F128> {
    assert_eq!(bits.rows(), 1usize << point.len());
    if point.len() < 6 {
        return super::column_evals(bits, point);
    }
    let eq = SplitEqGhash::with_n_hi(point, point.len() / 2);
    let tables: Vec<[F128; 16]> = eq
        .lo
        .chunks_exact(4)
        .map(|w| {
            let mut table = [F128::ZERO; 16];
            for i in 1usize..16 {
                table[i] = table[i & (i - 1)] + w[i.trailing_zeros() as usize];
            }
            table
        })
        .collect();
    let stride = bits.rows() / 8;
    let block = eq.lo.len() / 8;
    let calculate = |hi: usize| {
        let mut result = vec![F128::ZERO; bits.width().next_power_of_two()];
        for (column, value) in result[..bits.width()].iter_mut().enumerate() {
            let bytes =
                &bits.bytes()[column * stride + hi * block..column * stride + (hi + 1) * block];
            let mut sum = F128::ZERO;
            for (j, &byte) in bytes.iter().enumerate() {
                sum +=
                    tables[2 * j][(byte & 15) as usize] + tables[2 * j + 1][(byte >> 4) as usize];
            }
            *value = sum * eq.hi[hi];
        }
        result
    };
    let add = |mut a: Vec<F128>, b: Vec<F128>| {
        for (x, y) in a.iter_mut().zip(b) {
            *x += y;
        }
        a
    };
    if bits.rows() < 1 << 15 {
        (0..eq.hi.len())
            .map(calculate)
            .fold(vec![F128::ZERO; bits.width().next_power_of_two()], add)
    } else {
        (0..eq.hi.len())
            .into_par_iter()
            .map(calculate)
            .reduce(|| vec![F128::ZERO; bits.width().next_power_of_two()], add)
    }
}
