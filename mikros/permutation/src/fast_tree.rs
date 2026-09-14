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
    fn compact_layer(&self, _level: usize) -> Option<CompactLayer<'_>> {
        None
    }
    fn release_layer(&mut self, _level: usize) {}
}
#[derive(Clone, Copy)]
pub(crate) struct CompactLayer<'a> {
    words: &'a Words,
    evaluator: &'a PolynomialEvaluation,
}
impl CompactLayer<'_> {
    pub fn len(self) -> usize {
        self.words.len
    }
    pub fn read(self, start: usize, destination: &mut [F128]) {
        assert!(start + destination.len() <= self.len());
        match self.words.width {
            8 => return self.read_fixed::<1>(start, destination),
            16 => return self.read_fixed::<2>(start, destination),
            32 => return self.read_fixed::<4>(start, destination),
            64 => return self.read_fixed::<8>(start, destination),
            128 => return self.read_fixed::<16>(start, destination),
            _ => {}
        }
        for (i, value) in destination.iter_mut().enumerate() {
            *value = self.evaluator.evaluate(self.words.get(start + i));
        }
    }
    fn read_fixed<const BYTES: usize>(self, start: usize, destination: &mut [F128]) {
        let bytes = &self.words.bytes[start * BYTES..(start + destination.len()) * BYTES];
        for (word, value) in bytes.chunks_exact(BYTES).zip(destination) {
            let mut padded = [0u8; 16];
            padded[..BYTES].copy_from_slice(word);
            *value = self.evaluator.evaluate(u128::from_le_bytes(padded));
        }
    }
}
struct Words {
    width: usize,
    len: usize,
    bytes: Vec<u8>,
}
impl Words {
    fn new(width: usize, len: usize, pool: Option<usize>) -> Self {
        let byte_len = (width * len).div_ceil(8);
        // Byte-aligned producers below overwrite all bytes, including padded
        // rows. Sub-byte producers use |= and therefore need fresh zeroes.
        let bytes = match pool.filter(|_| width >= 8) {
            Some(slot) => crate::tree_pool::take_u8(slot, byte_len),
            None => vec![0; byte_len],
        };
        Self { width, len, bytes }
    }
    fn release(&mut self, pool: Option<usize>) {
        let bytes = std::mem::take(&mut self.bytes);
        if let Some(slot) = pool.filter(|_| self.width >= 8) {
            crate::tree_pool::give_u8(slot, bytes);
        }
        // Otherwise bytes is dropped here, synchronously.
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
    fn from_bits(bits: &PackedBits, width: usize, len: usize, pool: Option<usize>) -> Self {
        let mut out = Self::new(width, len, pool);
        if width < 8 {
            for i in 0..bits.rows() {
                out.set(i, bits.row(i));
            }
        } else {
            let size = width / 8;
            out.bytes
                .par_chunks_mut(size * 8)
                .enumerate()
                .for_each(|(group, dest)| {
                    let values = if group * 8 < bits.rows() {
                        decode_eight(bits, group)
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
fn allocate_f128(len: usize, pool: Option<usize>) -> Vec<std::mem::MaybeUninit<F128>> {
    match pool {
        Some(slot) => crate::tree_pool::take_f128(slot, len),
        None => crate::kernels::uninit_f128(len),
    }
}
pub(crate) struct ProductTree {
    compact: Vec<Words>,
    extension: Vec<Vec<F128>>,
    evaluators: Vec<PolynomialEvaluation>,
    field: &'static FastField,
    z: F128,
    lookup: bool,
    pool: Option<usize>,
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
        options.validate()?;
        let field = FastField::for_width(bits.width())?;
        let len = padded_len(bits.rows())?;
        let lookup = engine != Engine::ParallelTree;
        let pool = engine.tree_pool();
        let mut compact = Vec::new();
        let mut evaluators = Vec::new();
        let mut stats = TreeStats {
            subfield_bits: field.base.bits,
            ..Default::default()
        };
        let first = if engine.fused_tree()
            && options.strategy == Strategy::BitwidthAware
            && field.base.bits == 64
            && options.coefficient_capacity_bits == 128
            && len > 1
        {
            let t = Instant::now();
            let leaf = PolynomialEvaluation::leaf(field.base, bits.width(), z);
            let mut words = Words::new(64, len, pool);
            let mut first = allocate_f128(len / 2, pool);
            // One task owns eight compact leaves and their four evaluated
            // products. No polynomial-coefficient layer is materialized:
            // evaluation is a ring homomorphism, so evaluate then multiply
            // yields exactly the old multiply-then-evaluate result.
            words
                .bytes
                .par_chunks_mut(64)
                .zip(first.par_chunks_mut(4))
                .enumerate()
                .with_min_len(1024)
                .for_each(|(group, (dest, products))| {
                    let values = if group * 8 < bits.rows() {
                        decode_eight(bits, group)
                    } else {
                        [0; 8]
                    };
                    let mut evaluated = [F128::ZERO; 8];
                    for (r, word) in dest.chunks_exact_mut(8).enumerate() {
                        word.copy_from_slice(&(values[r] as u64).to_le_bytes());
                        evaluated[r] = leaf.evaluate(values[r]);
                    }
                    crate::kernels::multiply_pairs(&evaluated, 0, products);
                });
            compact.push(words);
            evaluators.push(leaf);
            stats.switch_level = 1;
            stats.max_coefficient_bits = 64;
            stats.compact_bytes = compact[0].bytes.len();
            p.fused_leaf_build_ms += elapsed(t);
            // SAFETY: the zipped slices have matching chunk counts, including
            // len=2/4 tails, and multiply_pairs initializes every output slot.
            unsafe { crate::kernels::initialized_f128(first) }
        } else if engine.fused_tree()
            && options.strategy == Strategy::BitwidthAware
            && field.base.bits == 32
            && options.coefficient_capacity_bits == 128
            && len >= 4
        {
            let t = Instant::now();
            let leaf = PolynomialEvaluation::leaf(field.base, bits.width(), z);
            let quadratic = PolynomialEvaluation::new(field.base, 64, z);
            let mut leaves = Words::new(32, len, pool);
            let mut pairs = Words::new(64, len / 2, pool);
            let mut first = allocate_f128(len / 4, pool);
            // Retain exactly the two compact layers needed by the GKR descent.
            // Each task decodes sixteen leaves and stores their eight monic
            // quadratics. Evaluating those quadratics and multiplying in E
            // equals evaluating their quartic product, without allocating the
            // otherwise temporary 128-bit coefficient layer. Four outputs per
            // full task also use multiply_pairs' existing four-lane kernel.
            leaves
                .bytes
                .par_chunks_mut(64)
                .zip(pairs.bytes.par_chunks_mut(64))
                .zip(first.par_chunks_mut(4))
                .enumerate()
                .with_min_len(512)
                .for_each(|(group, ((leaf_bytes, pair_bytes), products))| {
                    let mut evaluated = [F128::ZERO; 8];
                    for (half, dest) in leaf_bytes.chunks_mut(32).enumerate() {
                        let source_group = group * 2 + half;
                        let values = if source_group * 8 < bits.rows() {
                            decode_eight(bits, source_group)
                        } else {
                            [0; 8]
                        };
                        for (r, word) in dest.chunks_exact_mut(4).enumerate() {
                            word.copy_from_slice(&(values[r] as u32).to_le_bytes());
                        }
                        for r in 0..dest.len() / 8 {
                            let coefficient =
                                multiply_monic(values[2 * r], values[2 * r + 1], 1, field, lookup);
                            let index = half * 4 + r;
                            pair_bytes[index * 8..(index + 1) * 8]
                                .copy_from_slice(&(coefficient as u64).to_le_bytes());
                            evaluated[index] = quadratic.evaluate(coefficient);
                        }
                    }
                    crate::kernels::multiply_pairs(&evaluated, 0, products);
                });
            compact.push(leaves);
            compact.push(pairs);
            evaluators.push(leaf);
            evaluators.push(quadratic);
            stats.switch_level = 2;
            stats.max_coefficient_bits = 64;
            stats.compact_bytes = compact.iter().map(|w| w.bytes.len()).sum();
            p.fused_leaf_build_ms += elapsed(t);
            // SAFETY: each chunk owns sixteen leaves, eight coefficients and
            // four outputs, or the matching len=4/8 tail. Every product slot
            // is written by multiply_pairs before the parallel iterator joins.
            unsafe { crate::kernels::initialized_f128(first) }
        } else if options.strategy == Strategy::BitwidthAware
            && len > 1
            && field.base.bits * 2 <= options.coefficient_capacity_bits
        {
            let t = Instant::now();
            compact.push(Words::from_bits(bits, field.base.bits, len, pool));
            p.decode_ms += elapsed(t);
            let t = Instant::now();
            while compact.last().unwrap().len > 1
                && compact.last().unwrap().width * 2 <= options.coefficient_capacity_bits
            {
                let prev = compact.last().unwrap();
                let degree = prev.width / field.base.bits;
                let mut next = Words::new(prev.width * 2, prev.len / 2, pool);
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
                .enumerate()
                .map(|(level, w)| {
                    if level == 0 && engine.fused_tree() {
                        PolynomialEvaluation::leaf(field.base, bits.width(), z)
                    } else {
                        PolynomialEvaluation::new(field.base, w.width, z)
                    }
                })
                .collect();
            let first = Self::evaluate_words(
                compact.last().unwrap(),
                field,
                z,
                lookup,
                evaluators.last().unwrap(),
                pool,
            );
            compact.pop().unwrap().release(pool);
            stats.compact_bytes = compact.iter().map(|w| w.bytes.len()).sum();
            p.coefficient_eval_ms += elapsed(t);
            first
        } else {
            let t = Instant::now();
            let leaf = PolynomialEvaluation::leaf(field.base, bits.width(), z);
            let mut out = allocate_f128(len, pool);
            out.par_chunks_mut(8)
                .enumerate()
                .with_min_len(1024)
                .for_each(|(group, d)| {
                    let values = if group * 8 < bits.rows() {
                        decode_eight(bits, group)
                    } else {
                        [0; 8]
                    };
                    for (r, v) in d.iter_mut().enumerate() {
                        v.write(if engine.fused_tree() {
                            leaf.evaluate(values[r])
                        } else {
                            z + if lookup {
                                field.embed(values[r])
                            } else {
                                field.base.embed(values[r])
                            }
                        });
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
            let mut next = allocate_f128(prev.len() / 2, pool);
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
            pool,
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
        pool: Option<usize>,
    ) -> Vec<F128> {
        let degree = words.width / field.base.bits;
        let mut out = allocate_f128(words.len, pool);
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
                // Materialized views are ordinary Cow-owned temporaries;
                // only stored tree layers participate in the reuse pool.
                None,
            ))
        } else {
            Cow::Borrowed(&self.extension[level - self.stats.switch_level])
        }
    }
    fn compact_layer(&self, level: usize) -> Option<CompactLayer<'_>> {
        (self.lookup && level < self.stats.switch_level).then(|| CompactLayer {
            words: &self.compact[level],
            evaluator: &self.evaluators[level],
        })
    }
    fn release_layer(&mut self, level: usize) {
        if level < self.stats.switch_level {
            self.compact[level].release(self.pool);
        } else {
            let values = std::mem::take(&mut self.extension[level - self.stats.switch_level]);
            if let Some(slot) = self.pool {
                crate::tree_pool::give_f128(slot, values);
            }
            // Non-pooled and budget-rejected layers are freed before return.
        }
    }
}

/// Challenge-independent monic product coefficients. The highest compact layer
/// is retained as well: online proving evaluates it at z instead of rebuilding
/// the coefficients inside a fused decode/evaluation kernel.
pub(crate) struct PreparedTreeCoefficients {
    compact: Vec<Words>,
    field: &'static FastField,
    leaf: PolynomialEvaluation,
    log_len: usize,
}

impl PreparedTreeCoefficients {
    pub(crate) fn new(bits: &PackedBits, options: Options) -> Result<Self, Error> {
        options.validate()?;
        bits.validate()?;
        let field = FastField::for_width(bits.width())?;
        let len = padded_len(bits.rows())?;
        let mut compact = vec![Words::from_bits(bits, field.base.bits, len, None)];
        if options.strategy == Strategy::BitwidthAware {
            while compact.last().unwrap().len > 1
                && compact.last().unwrap().width * 2 <= options.coefficient_capacity_bits
            {
                let prev = compact.last().unwrap();
                let degree = prev.width / field.base.bits;
                let mut next = Words::new(prev.width * 2, prev.len / 2, None);
                if next.width < 8 {
                    for i in 0..next.len {
                        next.set(
                            i,
                            multiply_monic(
                                prev.get(2 * i),
                                prev.get(2 * i + 1),
                                degree,
                                field,
                                true,
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
                                    true,
                                )
                                .to_le_bytes()[..size],
                            );
                        });
                }
                compact.push(next);
            }
        }
        Ok(Self {
            compact,
            field,
            leaf: PolynomialEvaluation::leaf(field.base, bits.width(), F128::ZERO),
            log_len: len.ilog2() as usize,
        })
    }

    /// Actual capacities of retained coefficient byte buffers and the private
    /// offline leaf table. Small metadata, shared global field tables and any
    /// separately retained bit sources are excluded from this count.
    pub(crate) fn resident_bytes(&self) -> usize {
        self.leaf.table_bytes()
            + self
                .compact
                .iter()
                .map(|layer| layer.bytes.capacity())
                .sum::<usize>()
    }

    pub(crate) fn online<'a>(
        &'a self,
        z: F128,
        engine: Engine,
        p: &mut Timings,
    ) -> PreparedTreeRuntime<'a> {
        let pool = engine.tree_pool();
        let lookup = engine != Engine::ParallelTree;
        let t = Instant::now();
        let evaluators: Vec<_> = self
            .compact
            .iter()
            .enumerate()
            .map(|(level, words)| {
                if level == 0 {
                    self.leaf.with_constant(z)
                } else {
                    PolynomialEvaluation::new(self.field.base, words.width, z)
                }
            })
            .collect();
        let first = ProductTree::evaluate_words(
            self.compact.last().unwrap(),
            self.field,
            z,
            lookup,
            evaluators.last().unwrap(),
            pool,
        );
        p.coefficient_eval_ms += elapsed(t);
        let t = Instant::now();
        let mut extension = vec![first];
        while extension.last().unwrap().len() > 1 {
            let prev = extension.last().unwrap();
            let mut next = allocate_f128(prev.len() / 2, pool);
            next.par_chunks_mut(4096)
                .enumerate()
                .for_each(|(chunk, d)| crate::kernels::multiply_pairs(prev, chunk * 4096, d));
            // SAFETY: each disjoint destination chunk is fully initialized by
            // multiply_pairs before the parallel iterator joins.
            extension.push(unsafe { crate::kernels::initialized_f128(next) });
        }
        p.extension_build_ms += elapsed(t);
        let switch_level = self.compact.len() - 1;
        let stats = TreeStats {
            subfield_bits: self.field.base.bits,
            switch_level,
            max_coefficient_bits: self.compact.last().unwrap().width,
            compact_bytes: self.compact[..switch_level]
                .iter()
                .map(|w| w.bytes.len())
                .sum(),
            extension_bytes: extension.iter().map(|x| x.len() * 16).sum(),
        };
        PreparedTreeRuntime {
            prepared: self,
            extension,
            evaluators,
            z,
            lookup,
            pool,
            stats,
        }
    }
}

/// Runtime views borrow immutable offline coefficients. No coefficient buffers
/// are cloned or released per proof; only challenge-dependent extension layers
/// use the optional per-proof pool.
pub(crate) struct PreparedTreeRuntime<'a> {
    prepared: &'a PreparedTreeCoefficients,
    extension: Vec<Vec<F128>>,
    evaluators: Vec<PolynomialEvaluation>,
    z: F128,
    lookup: bool,
    pool: Option<usize>,
    pub(crate) stats: TreeStats,
}

impl Tree for PreparedTreeRuntime<'_> {
    fn log_len(&self) -> usize {
        self.prepared.log_len
    }

    fn root(&self) -> F128 {
        self.extension.last().unwrap()[0]
    }

    fn layer(&self, level: usize) -> Cow<'_, [F128]> {
        if level < self.stats.switch_level {
            Cow::Owned(ProductTree::evaluate_words(
                &self.prepared.compact[level],
                self.prepared.field,
                self.z,
                self.lookup,
                &self.evaluators[level],
                None,
            ))
        } else {
            Cow::Borrowed(&self.extension[level - self.stats.switch_level])
        }
    }

    fn compact_layer(&self, level: usize) -> Option<CompactLayer<'_>> {
        (self.lookup && level < self.stats.switch_level).then(|| CompactLayer {
            words: &self.prepared.compact[level],
            evaluator: &self.evaluators[level],
        })
    }

    fn release_layer(&mut self, level: usize) {
        if level >= self.stats.switch_level {
            let values = std::mem::take(&mut self.extension[level - self.stats.switch_level]);
            if let Some(slot) = self.pool {
                crate::tree_pool::give_f128(slot, values);
            }
        }
    }
}
