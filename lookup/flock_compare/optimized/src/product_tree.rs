use crate::{Error, field::SmallField, padded_len};
use flock_field::F128;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Strategy {
    BitwidthAware,
    FullExtension,
}
#[derive(Clone, Copy, Debug)]
pub struct Options {
    pub strategy: Strategy,
    pub coefficient_capacity_bits: usize,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            strategy: Strategy::BitwidthAware,
            coefficient_capacity_bits: 128,
        }
    }
}
impl Options {
    pub(crate) fn validate(self) -> Result<(), Error> {
        if self.coefficient_capacity_bits == 0 || self.coefficient_capacity_bits > 128 {
            return Err(Error::Capacity);
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TreeStats {
    pub subfield_bits: usize,
    /// Degree at the last compact level is 2^switch_level.
    pub switch_level: usize,
    pub max_coefficient_bits: usize,
    pub compact_bytes: usize,
    pub extension_bytes: usize,
}
/// Consecutive words really occupy `width` bits each, including 1/2/4-bit words.
struct Words {
    width: usize,
    len: usize,
    bytes: Vec<u8>,
}
impl Words {
    fn new(width: usize, len: usize) -> Self {
        Self {
            width,
            len,
            bytes: vec![0; (width * len).div_ceil(8)],
        }
    }
    fn get(&self, i: usize) -> u128 {
        if self.width < 8 {
            return ((self.bytes[i * self.width / 8] >> (i * self.width % 8))
                & ((1 << self.width) - 1)) as u128;
        }
        let size = self.width / 8;
        let mut bytes = [0u8; 16];
        bytes[..size].copy_from_slice(&self.bytes[i * size..(i + 1) * size]);
        u128::from_le_bytes(bytes)
    }
    fn set(&mut self, i: usize, value: u128) {
        if self.width < 8 {
            self.bytes[i * self.width / 8] |= (value as u8) << (i * self.width % 8);
        } else {
            let size = self.width / 8;
            self.bytes[i * size..(i + 1) * size].copy_from_slice(&value.to_le_bytes()[..size]);
        }
    }
}
fn multiply_monic(a: u128, b: u128, degree: usize, field: &SmallField) -> u128 {
    let k = field.bits;
    // Leading coefficient is one and is not stored. No reduction modulo an
    // invented degree-(2*degree*k) field: these are formal coefficients in K[Z].
    if k == 1 {
        let mut out = (a ^ b) << degree;
        let mut bits = b;
        while bits != 0 {
            let j = bits.trailing_zeros();
            out ^= a << j;
            bits &= bits - 1;
        }
        return out;
    }
    let mut out = (a ^ b) << (degree * k);
    for i in 0..degree {
        for j in 0..degree {
            let c = field.multiply((a >> (i * k)) & field.mask(), (b >> (j * k)) & field.mask());
            out ^= c << ((i + j) * k);
        }
    }
    out
}
pub(crate) struct ProductTree {
    compact: Vec<Words>,
    // Starts with evaluation of the last compact level (or ordinary leaves).
    extension: Vec<Vec<F128>>,
    field: &'static SmallField,
    z: F128,
    pub stats: TreeStats,
    pub log_len: usize,
}
impl ProductTree {
    pub fn new(values: &[u128], width: usize, z: F128, options: Options) -> Result<Self, Error> {
        options.validate()?;
        let field = SmallField::for_width(width)?;
        if values.iter().any(|v| v & !field.mask() != 0) {
            return Err(Error::OutOfRange);
        }
        let len = padded_len(values.len())?;
        let mut compact = Vec::new();
        let mut stats = TreeStats {
            subfield_bits: field.bits,
            ..Default::default()
        };
        let first = if options.strategy == Strategy::BitwidthAware
            && len > 1
            && field.bits * 2 <= options.coefficient_capacity_bits
        {
            let mut leaves = Words::new(field.bits, len);
            for (i, &value) in values.iter().enumerate() {
                leaves.set(i, value);
            }
            compact.push(leaves);
            while compact.last().unwrap().len > 1
                && compact.last().unwrap().width * 2 <= options.coefficient_capacity_bits
            {
                let prev = compact.last().unwrap();
                let degree = prev.width / field.bits;
                let mut next = Words::new(prev.width * 2, prev.len / 2);
                for i in 0..next.len {
                    next.set(
                        i,
                        multiply_monic(prev.get(2 * i), prev.get(2 * i + 1), degree, field),
                    );
                }
                compact.push(next);
            }
            stats.switch_level = compact.len() - 1;
            stats.max_coefficient_bits = compact.last().unwrap().width;
            let first = Self::evaluate_words(compact.last().unwrap(), field, z);
            // This level is now retained in E and never read as coefficients.
            // In particular K=E goes directly to E without a duplicate leaf copy.
            compact.pop();
            stats.compact_bytes = compact.iter().map(|x| x.bytes.len()).sum();
            first
        } else {
            (0..len)
                .map(|i| z + field.embed(values.get(i).copied().unwrap_or(0)))
                .collect()
        };
        let mut extension = vec![first];
        while extension.last().unwrap().len() > 1 {
            let next = extension
                .last()
                .unwrap()
                .chunks_exact(2)
                .map(|pair| pair[0] * pair[1])
                .collect();
            extension.push(next);
        }
        stats.extension_bytes = extension.iter().map(|x| x.len() * 16).sum();
        Ok(Self {
            compact,
            extension,
            field,
            z,
            stats,
            log_len: len.ilog2() as usize,
        })
    }
    fn evaluate_words(words: &Words, field: &SmallField, z: F128) -> Vec<F128> {
        let degree = words.width / field.bits;
        (0..words.len)
            .map(|i| {
                let word = words.get(i);
                (0..degree).rev().fold(F128::ONE, |v, j| {
                    v * z + field.embed((word >> (j * field.bits)) & field.mask())
                })
            })
            .collect()
    }
    pub fn root(&self) -> F128 {
        self.extension.last().unwrap()[0]
    }
    pub fn layer(&self, level: usize) -> Vec<F128> {
        if !self.compact.is_empty() && level < self.stats.switch_level {
            Self::evaluate_words(&self.compact[level], self.field, self.z)
        } else {
            self.extension[level - self.stats.switch_level].clone()
        }
    }
}
