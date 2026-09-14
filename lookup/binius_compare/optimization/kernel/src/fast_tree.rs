use crate::{
    Error,
    bits::{PackedBits, decode_eight},
    field::{FastField, PolynomialEvaluation},
    padded_len,
    profile::{Engine, Timings, elapsed},
};
use flock_field::F128;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, time::Instant};

#[derive(Clone, Copy)]
enum Rows<'a> {
    Bits(&'a PackedBits),
    Values(&'a [u128], usize),
}
impl Rows<'_> {
    fn width(self) -> usize {
        match self {
            Self::Bits(b) => b.width(),
            Self::Values(_, w) => w,
        }
    }
    fn rows(self) -> usize {
        match self {
            Self::Bits(b) => b.rows(),
            Self::Values(v, _) => v.len(),
        }
    }
    fn group(self, group: usize) -> [u128; 8] {
        if group * 8 >= self.rows() {
            return [0; 8];
        }
        match self {
            Self::Bits(b) => decode_eight(b, group),
            Self::Values(v, _) => {
                std::array::from_fn(|j| v.get(group * 8 + j).copied().unwrap_or(0))
            }
        }
    }
}

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
            Err(Error::Capacity)
        } else {
            Ok(())
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TreeStats {
    pub subfield_bits: usize,
    pub switch_level: usize,
    pub max_coefficient_bits: usize,
    pub compact_bytes: usize,
    pub extension_bytes: usize,
}
pub(crate) trait Tree {
    fn log_len(&self) -> usize;
    fn root(&self) -> F128;
    fn layer(&self, level: usize) -> Cow<'_, [F128]>;
}
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
    #[inline]
    fn get(&self, i: usize) -> u128 {
        if self.width < 8 {
            return ((self.bytes[i * self.width / 8] >> (i * self.width % 8))
                & ((1 << self.width) - 1)) as u128;
        }
        let mut value = [0; 16];
        let size = self.width / 8;
        value[..size].copy_from_slice(&self.bytes[i * size..(i + 1) * size]);
        u128::from_le_bytes(value)
    }
    fn set(&mut self, i: usize, value: u128) {
        if self.width < 8 {
            self.bytes[i * self.width / 8] |= (value as u8) << (i * self.width % 8);
        } else {
            let size = self.width / 8;
            self.bytes[i * size..(i + 1) * size].copy_from_slice(&value.to_le_bytes()[..size]);
        }
    }
    fn from_rows(bits: Rows<'_>, width: usize, len: usize) -> Self {
        let mut out = Self::new(width, len);
        if width < 8 {
            for i in 0..bits.rows() {
                out.set(i, bits.group(i / 8)[i % 8]);
            }
        } else {
            let size = width / 8;
            out.bytes
                .par_chunks_mut(size * 8)
                .enumerate()
                .for_each(|(group, dest)| {
                    let values = if group * 8 < bits.rows() {
                        bits.group(group)
                    } else {
                        [0; 8]
                    };
                    for (r, word) in dest.chunks_mut(size).enumerate() {
                        word.copy_from_slice(&values[r].to_le_bytes()[..size]);
                    }
                });
        }
        out
    }
}
fn multiply_monic(a: u128, b: u128, degree: usize, field: &FastField, lookup: bool) -> u128 {
    let k = field.base.bits;
    if k == 1 {
        let mut out = (a ^ b) << degree;
        let mut bits = b;
        while bits != 0 {
            out ^= a << bits.trailing_zeros();
            bits &= bits - 1;
        }
        return out;
    }
    let mut out = (a ^ b) << (degree * k);
    if lookup && (16..=64).contains(&k) {
        // Reduction is F2-linear: XOR full carryless products in each
        // convolution coefficient, then reduce once. XOR cannot overflow.
        for s in 0..2 * degree - 1 {
            let mut accumulator = 0u128;
            for i in 0..degree {
                if s >= i && s - i < degree {
                    accumulator ^= field.multiply_unreduced(
                        (a >> (i * k)) & field.base.mask(),
                        (b >> ((s - i) * k)) & field.base.mask(),
                    );
                }
            }
            out ^= field.reduce(accumulator) << (s * k);
        }
        return out;
    }
    for i in 0..degree {
        for j in 0..degree {
            let x = (a >> (i * k)) & field.base.mask();
            let y = (b >> (j * k)) & field.base.mask();
            let c = if lookup {
                field.multiply(x, y)
            } else {
                field.base.multiply(x, y)
            };
            out ^= c << ((i + j) * k);
        }
    }
    out
}
pub(crate) struct ProductTree {
    compact: Vec<Words>,
    extension: Vec<Vec<F128>>,
    evaluators: Vec<PolynomialEvaluation>,
    field: &'static FastField,
    z: F128,
    lookup: bool,
    pub stats: TreeStats,
    pub log_len: usize,
}
impl ProductTree {
    #[cfg(test)]
    pub fn new(values: &[u128], width: usize, z: F128, options: Options) -> Result<Self, Error> {
        let bits = PackedBits::from_rows(values, crate::field::SmallField::for_width(width)?.bits)?;
        Self::from_bits(&bits, z, options, Engine::Fused, &mut Timings::default())
    }
    pub fn from_bits(
        bits: &PackedBits,
        z: F128,
        options: Options,
        engine: Engine,
        p: &mut Timings,
    ) -> Result<Self, Error> {
        Self::from_source(Rows::Bits(bits), z, options, engine, p)
    }
    pub fn from_rows(
        values: &[u128],
        width: usize,
        z: F128,
        options: Options,
        engine: Engine,
        p: &mut Timings,
    ) -> Result<Self, Error> {
        Self::from_source(Rows::Values(values, width), z, options, engine, p)
    }
    fn from_source(
        bits: Rows<'_>,
        z: F128,
        options: Options,
        engine: Engine,
        p: &mut Timings,
    ) -> Result<Self, Error> {
        options.validate()?;
        let field = FastField::for_width(bits.width())?;
        let len = padded_len(bits.rows())?;
        let lookup = engine != Engine::ParallelTree;
        let mut compact = Vec::new();
        let mut evaluators = Vec::new();
        let mut stats = TreeStats {
            subfield_bits: field.base.bits,
            ..Default::default()
        };
        let first = if options.strategy == Strategy::BitwidthAware
            && len > 1
            && field.base.bits * 2 <= options.coefficient_capacity_bits
        {
            let t = Instant::now();
            compact.push(Words::from_rows(bits, field.base.bits, len));
            p.decode_ms += elapsed(t);
            let t = Instant::now();
            while compact.last().unwrap().len > 1
                && compact.last().unwrap().width * 2 <= options.coefficient_capacity_bits
            {
                let prev = compact.last().unwrap();
                let degree = prev.width / field.base.bits;
                let mut next = Words::new(prev.width * 2, prev.len / 2);
                if next.width < 8 {
                    for i in 0..next.len {
                        next.set(
                            i,
                            multiply_monic(
                                prev.get(2 * i),
                                prev.get(2 * i + 1),
                                degree,
                                field,
                                lookup,
                            ),
                        );
                    }
                } else {
                    let size = next.width / 8;
                    next.bytes
                        .par_chunks_mut(size)
                        .enumerate()
                        .with_min_len(2048)
                        .for_each(|(i, d)| {
                            d.copy_from_slice(
                                &multiply_monic(
                                    prev.get(2 * i),
                                    prev.get(2 * i + 1),
                                    degree,
                                    field,
                                    lookup,
                                )
                                .to_le_bytes()[..size],
                            );
                        });
                }
                compact.push(next);
            }
            p.compact_build_ms += elapsed(t);
            stats.switch_level = compact.len() - 1;
            stats.max_coefficient_bits = compact.last().unwrap().width;
            let t = Instant::now();
            evaluators = compact
                .iter()
                .map(|w| PolynomialEvaluation::new(field.base, w.width, z))
                .collect();
            let first = Self::evaluate_words(
                compact.last().unwrap(),
                field,
                z,
                lookup,
                evaluators.last().unwrap(),
            );
            compact.pop();
            stats.compact_bytes = compact.iter().map(|w| w.bytes.len()).sum();
            p.coefficient_eval_ms += elapsed(t);
            first
        } else {
            let t = Instant::now();
            let mut out = crate::kernels::uninit_f128(len);
            out.par_chunks_mut(8)
                .enumerate()
                .with_min_len(1024)
                .for_each(|(group, d)| {
                    let values = if group * 8 < bits.rows() {
                        bits.group(group)
                    } else {
                        [0; 8]
                    };
                    for (r, v) in d.iter_mut().enumerate() {
                        v.write(
                            z + if lookup {
                                field.embed(values[r])
                            } else {
                                field.base.embed(values[r])
                            },
                        );
                    }
                });
            p.coefficient_eval_ms += elapsed(t);
            // SAFETY: every output slot was written above, including padding.
            unsafe { crate::kernels::initialized_f128(out) }
        };
        let t = Instant::now();
        let mut extension = vec![first];
        while extension.last().unwrap().len() > 1 {
            let prev = extension.last().unwrap();
            let mut next = crate::kernels::uninit_f128(prev.len() / 2);
            next.par_chunks_mut(4096)
                .enumerate()
                .for_each(|(chunk, d)| crate::kernels::multiply_pairs(prev, chunk * 4096, d));
            // SAFETY: multiply_pairs fills each complete disjoint chunk.
            extension.push(unsafe { crate::kernels::initialized_f128(next) });
        }
        p.extension_build_ms += elapsed(t);
        stats.extension_bytes = extension.iter().map(|x| x.len() * 16).sum();
        Ok(Self {
            compact,
            extension,
            evaluators,
            field,
            z,
            lookup,
            stats,
            log_len: len.ilog2() as usize,
        })
    }
    fn evaluate_words(
        words: &Words,
        field: &FastField,
        z: F128,
        lookup: bool,
        evaluator: &PolynomialEvaluation,
    ) -> Vec<F128> {
        let degree = words.width / field.base.bits;
        let mut out = crate::kernels::uninit_f128(words.len);
        out.par_iter_mut()
            .enumerate()
            .with_min_len(2048)
            .for_each(|(i, v)| {
                let word = words.get(i);
                v.write(if lookup {
                    evaluator.evaluate(word)
                } else {
                    (0..degree).rev().fold(F128::ONE, |v, j| {
                        v * z
                            + field
                                .base
                                .embed((word >> (j * field.base.bits)) & field.base.mask())
                    })
                });
            });
        // SAFETY: par_iter_mut initialized every element before joining.
        unsafe { crate::kernels::initialized_f128(out) }
    }
    pub fn root(&self) -> F128 {
        self.extension.last().unwrap()[0]
    }
    pub(crate) fn coefficient_word(&self, level: usize, index: usize) -> u128 {
        self.compact[level].get(index)
    }
    #[cfg(test)]
    pub fn layer(&self, level: usize) -> Vec<F128> {
        Tree::layer(self, level).into_owned()
    }
}
impl Tree for ProductTree {
    fn log_len(&self) -> usize {
        self.log_len
    }
    fn root(&self) -> F128 {
        self.root()
    }
    fn layer(&self, level: usize) -> Cow<'_, [F128]> {
        if level < self.stats.switch_level {
            Cow::Owned(Self::evaluate_words(
                &self.compact[level],
                self.field,
                self.z,
                self.lookup,
                &self.evaluators[level],
            ))
        } else {
            Cow::Borrowed(&self.extension[level - self.stats.switch_level])
        }
    }
}
