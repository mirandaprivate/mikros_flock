//! Dao--Thaler Algorithms 2/3, with explicit canonical-tower/GHASH conversion.
//!
//! Tower embedding, reverse recurrence, and the Algorithm 3 word kernel.
//! GHASH bits represent polynomial coefficients; the online computation includes
//! conversion to and from the canonical tower basis.

use std::sync::LazyLock;

use flock_field::F128;
use rayon::prelude::*;

use crate::Result;

const RAW_GENERATORS: [u128; 7] = [
    0x295a_c0b1_f473_1af9_676a_ac9f_a4b2_0b08,
    0x7959_d70c_e1ee_6942_53b8_5b64_02b1_e849,
    0x6cfe_46e8_277b_2e9d_f167_7bc1_710c_5c54,
    0xd748_3fda_e776_3b6f_f501_c92b_7a41_cc34,
    0x681c_6a8c_3679_2c02_92d6_dad7_72f9_2df5,
    0x2170_d8f3_9592_cb2a_0740_dda1_9dbb_d4fe,
    0xa52f_37ce_6b44_ce4f_8ee7_4a99_f18f_b7b4,
];

pub fn generators() -> [F128; 7] {
    RAW_GENERATORS.map(from_raw)
}

pub(crate) const fn from_raw(value: u128) -> F128 {
    F128::new(value as u64, (value >> 64) as u64)
}

pub(crate) fn raw(value: F128) -> u128 {
    u128::from(value.lo) | (u128::from(value.hi) << 64)
}

const HIGH_MASKS: [u128; 7] = [
    0xaaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa,
    0xcccc_cccc_cccc_cccc_cccc_cccc_cccc_cccc,
    0xf0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0_f0f0,
    0xff00_ff00_ff00_ff00_ff00_ff00_ff00_ff00,
    0xffff_0000_ffff_0000_ffff_0000_ffff_0000,
    0xffff_ffff_0000_0000_ffff_ffff_0000_0000,
    0xffff_ffff_ffff_ffff_0000_0000_0000_0000,
];

#[inline]
fn swap_index_bit(a: u128, i: usize) -> u128 {
    let high = HIGH_MASKS[i];
    let shift = 1 << i;
    ((a & !high) << shift) | ((a & high) >> shift)
}

#[inline]
fn word_at<const I: usize>(a: u128) -> u128 {
    let mut b = swap_index_bit(a, I);
    let mut active = HIGH_MASKS[I];
    for j in (1..=I).rev() {
        b ^= swap_index_bit(a, j - 1) & active;
        active &= HIGH_MASKS[j - 1];
    }
    b ^ (a & active)
}

/// Multiply canonical tower coordinates by z_i, using only masks/shifts/XORs.
/// Panics if i is outside 0..7. Never apply this to raw GHASH coordinates.
pub fn algorithm3_word(a: u128, i: usize) -> u128 {
    match i {
        0 => word_at::<0>(a),
        1 => word_at::<1>(a),
        2 => word_at::<2>(a),
        3 => word_at::<3>(a),
        4 => word_at::<4>(a),
        5 => word_at::<5>(a),
        6 => word_at::<6>(a),
        _ => panic!("canonical tower has seven generators"),
    }
}

struct LinearMap(Vec<u128>);

impl LinearMap {
    fn new(images: &[u128; 128]) -> Self {
        let mut table = vec![0; 16 * 256];
        for byte in 0..16 {
            for value in 1usize..256 {
                let i = value.trailing_zeros() as usize;
                table[256 * byte + value] =
                    table[256 * byte + (value & (value - 1))] ^ images[8 * byte + i];
            }
        }
        Self(table)
    }

    #[inline]
    fn apply(&self, value: u128) -> u128 {
        let mut result = 0;
        for byte in 0..16 {
            result ^= self.0[byte * 256 + ((value >> (8 * byte)) & 255) as usize];
        }
        result
    }
}

struct TowerIsomorphism {
    to_ghash: LinearMap,
    to_tower: LinearMap,
}

impl TowerIsomorphism {
    fn new() -> Self {
        let z = generators();
        let mut images = [0u128; 128];
        images[0] = 1;
        for j in 1usize..128 {
            let i = j.trailing_zeros() as usize;
            images[j] = raw(from_raw(images[j & (j - 1)]) * z[i]);
        }
        // Binary Gaussian elimination of the 128 canonical basis images.
        // Track preimages, so each polynomial basis bit can be inverted.
        let mut pivots = [(0u128, 0u128); 128];
        for (j, &image) in images.iter().enumerate() {
            let (mut value, mut preimage) = (image, 1u128 << j);
            while value != 0 {
                let bit = value.ilog2() as usize;
                if pivots[bit].0 == 0 {
                    pivots[bit] = (value, preimage);
                    break;
                }
                value ^= pivots[bit].0;
                preimage ^= pivots[bit].1;
            }
            assert_ne!(value, 0, "tower embedding must have rank 128");
        }
        let inverse_images = std::array::from_fn(|i| {
            let (mut value, mut preimage) = (1u128 << i, 0);
            while value != 0 {
                let bit = value.ilog2() as usize;
                value ^= pivots[bit].0;
                preimage ^= pivots[bit].1;
            }
            preimage
        });
        Self {
            to_ghash: LinearMap::new(&images),
            to_tower: LinearMap::new(&inverse_images),
        }
    }
}

// Public, witness-independent maps: 2 * 64 KiB, initialized on first use.
// A cold proof pays this setup; benchmark warmups retain that first-proof time.
static ISOMORPHISM: LazyLock<TowerIsomorphism> = LazyLock::new(TowerIsomorphism::new);

pub fn to_canonical(value: F128) -> u128 {
    ISOMORPHISM.to_tower.apply(raw(value))
}

pub fn from_canonical(value: u128) -> F128 {
    from_raw(ISOMORPHISM.to_ghash.apply(value))
}

/// Plain reverse recurrence, LSB-first result order. Also the fixed-point
/// generic control: changing its equality builder must not change proof bytes.
pub fn generic_eq_table(point: &[F128]) -> Result<Vec<F128>> {
    if point.len() > 30 {
        return Err("equality table supports at most 30 coordinates".into());
    }
    let mut values = Vec::with_capacity(1usize << point.len());
    values.push(F128::ONE);
    for &r in point.iter().rev() {
        let old = values.len();
        values.resize(2 * old, F128::ZERO);
        for j in (0..old).rev() {
            let parent = values[j];
            let high = parent * r;
            values[2 * j] = parent + high;
            values[2 * j + 1] = high;
        }
    }
    Ok(values)
}

/// Algorithm 2 expands the random suffix in reverse order, then the fixed
/// prefix high to low. Fixed expansions occur in canonical coordinates using
/// Algorithm 3; each seed enters and each output leaves that representation.
pub fn equality_table(point: &[F128], fixed: usize) -> Result<Vec<F128>> {
    if point.len() > 30 || fixed > point.len() || fixed > 7 {
        return Err("invalid Dao-Thaler point or fixed prefix".into());
    }
    if point[..fixed] != generators()[..fixed] {
        return Err("fixed coordinates must be the canonical tower generators".into());
    }
    if fixed == 0 {
        return generic_eq_table(point);
    }
    let seeds = generic_eq_table(&point[fixed..])?;
    let map = &*ISOMORPHISM;
    let width = 1usize << fixed;
    let mut output = vec![F128::ZERO; 1usize << point.len()];
    let expand = |(target, &seed): (&mut [F128], &F128)| {
        let mut tile = [0u128; 128];
        tile[0] = map.to_tower.apply(raw(seed));
        let mut parents = 1;
        for i in (0..fixed).rev() {
            for j in (0..parents).rev() {
                let parent = tile[j];
                let high = algorithm3_word(parent, i);
                tile[2 * j] = parent ^ high;
                tile[2 * j + 1] = high;
            }
            parents *= 2;
        }
        for (out, &value) in target.iter_mut().zip(&tile) {
            *out = from_raw(map.to_ghash.apply(value));
        }
    };
    if output.len() >= 32768 && rayon::current_num_threads() > 1 {
        output
            .par_chunks_mut(width)
            .zip(seeds.par_iter())
            .with_min_len(64)
            .for_each(expand);
    } else {
        output.chunks_mut(width).zip(&seeds).for_each(expand);
    }
    Ok(output)
}

/// Recurrence counts exclude linear basis conversions and fixed-map setup;
/// they are not total prover operation counts or measured speedups.
pub fn operation_counts(n_vars: usize, fixed: usize) -> Result<(usize, usize)> {
    if n_vars > 30 || fixed > n_vars || fixed > 7 {
        return Err("invalid equality-table dimensions".into());
    }
    let random = 1usize << (n_vars - fixed);
    Ok((random - 1, (1usize << n_vars) - random))
}
