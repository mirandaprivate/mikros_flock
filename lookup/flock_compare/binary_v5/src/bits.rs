//! Column-major bit matrix: bit (column * rows + row) is one F2 coordinate.
use crate::{Digest, Error, MAX_LOG_ROWS};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackedBits {
    rows: usize,
    width: usize,
    data: Vec<u8>,
}
impl PackedBits {
    pub fn new(rows: usize, width: usize, data: Vec<u8>) -> Result<Self, Error> {
        let result = Self { rows, width, data };
        result.validate()?;
        Ok(result)
    }
    pub fn from_rows(values: &[u128], width: usize) -> Result<Self, Error> {
        if width > 128 || values.len() > (1usize << MAX_LOG_ROWS) {
            return Err(Error::Width);
        }
        let mut result = Self {
            rows: values.len(),
            width,
            data: vec![0; (values.len() * width).div_ceil(8)],
        };
        for (row, &value) in values.iter().enumerate() {
            if width < 128 && value >> width != 0 {
                return Err(Error::OutOfRange);
            }
            for column in 0..width {
                result.set_bit(column * values.len() + row, ((value >> column) & 1) != 0);
            }
        }
        Ok(result)
    }
    pub fn validate(&self) -> Result<(), Error> {
        if self.width > 128 || self.rows > (1usize << MAX_LOG_ROWS) {
            return Err(Error::Width);
        }
        let bits = self.rows.checked_mul(self.width).ok_or(Error::Length)?;
        if self.data.len() != bits.div_ceil(8) {
            return Err(Error::Length);
        }
        if bits % 8 != 0 && self.data.last().copied().unwrap_or(0) >> (bits % 8) != 0 {
            return Err(Error::Padding);
        }
        Ok(())
    }
    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn bytes(&self) -> &[u8] {
        &self.data
    }
    pub fn bit_len(&self) -> usize {
        self.rows * self.width
    }
    pub fn bit(&self, index: usize) -> bool {
        assert!(index < self.bit_len());
        self.data[index / 8] >> (index % 8) & 1 != 0
    }
    pub fn row(&self, row: usize) -> u128 {
        assert!(row < self.rows);
        (0..self.width).fold(0, |value, column| {
            value | ((self.bit(column * self.rows + row) as u128) << column)
        })
    }
    pub fn digest(&self) -> Digest {
        let mut hash = blake3::Hasher::new();
        hash.update(b"mikros-lookup-bit-oracle-v1");
        hash.update(&(self.rows as u64).to_le_bytes());
        hash.update(&(self.width as u64).to_le_bytes());
        hash.update(&self.data);
        *hash.finalize().as_bytes()
    }
    pub fn digest_parallel(&self) -> Digest {
        let mut hash = blake3::Hasher::new();
        hash.update(b"mikros-lookup-bit-oracle-v1");
        hash.update(&(self.rows as u64).to_le_bytes());
        hash.update(&(self.width as u64).to_le_bytes());
        hash.update_rayon(&self.data);
        *hash.finalize().as_bytes()
    }
    /// Same column-major bytes as from_rows, transposing eight rows at once.
    pub fn from_fn_parallel<F: Fn(usize) -> u128 + Sync>(
        rows: usize,
        width: usize,
        value: F,
    ) -> Result<Self, Error> {
        if width > 128 || rows > 1usize << MAX_LOG_ROWS {
            return Err(Error::Width);
        }
        if !rows.is_multiple_of(8) {
            return Self::from_rows(&(0..rows).map(value).collect::<Vec<_>>(), width);
        }
        let stride = rows / 8;
        let mut data = vec![0u8; stride * width];
        let address = data.as_mut_ptr() as usize;
        (0..stride)
            .into_par_iter()
            .with_min_len(1024)
            .try_for_each(|group| {
                let v: [u128; 8] = std::array::from_fn(|r| value(group * 8 + r));
                if width < 128 && v.iter().any(|x| x >> width != 0) {
                    return Err(Error::OutOfRange);
                }
                for byte in 0..width.div_ceil(8) {
                    let mut word = 0u64;
                    for r in 0..8 {
                        word |= (((v[r] >> (byte * 8)) & 255) as u64) << (r * 8);
                    }
                    let planes = transpose8(word);
                    for bit in 0..8.min(width - byte * 8) {
                        let offset = (byte * 8 + bit) * stride + group;
                        // SAFETY: offset is in the initialized Vec. A group owns
                        // its distinct byte in every column; workers never alias.
                        // data is not otherwise accessed until Rayon joins.
                        unsafe {
                            *((address as *mut u8).add(offset)) = (planes >> (bit * 8)) as u8;
                        }
                    }
                }
                Ok(())
            })?;
        Self::new(rows, width, data)
    }
    pub(crate) fn set_bit(&mut self, index: usize, value: bool) {
        let mask = 1 << (index % 8);
        if value {
            self.data[index / 8] |= mask;
        } else {
            self.data[index / 8] &= !mask;
        }
    }
}

#[inline]
pub(crate) fn transpose8(mut value: u64) -> u64 {
    let t = (value ^ (value >> 7)) & 0x00aa00aa00aa00aa;
    value ^= t ^ (t << 7);
    let t = (value ^ (value >> 14)) & 0x0000cccc0000cccc;
    value ^= t ^ (t << 14);
    let t = (value ^ (value >> 28)) & 0x00000000f0f0f0f0;
    value ^= t ^ (t << 28);
    value
}

#[inline]
pub(crate) fn decode_eight(bits: &PackedBits, group: usize) -> [u128; 8] {
    if !bits.rows().is_multiple_of(8) {
        return std::array::from_fn(|r| {
            let row = group * 8 + r;
            if row < bits.rows() { bits.row(row) } else { 0 }
        });
    }
    let stride = bits.rows() / 8;
    let mut rows = [0u128; 8];
    for byte in 0..bits.width().div_ceil(8) {
        let mut word = 0u64;
        for bit in 0..8.min(bits.width() - byte * 8) {
            word |= (bits.bytes()[(byte * 8 + bit) * stride + group] as u64) << (8 * bit);
        }
        let values = transpose8(word);
        for r in 0..8 {
            rows[r] |= (((values >> (r * 8)) & 255) as u128) << (byte * 8);
        }
    }
    rows
}

pub fn bits_for(max_inclusive: usize) -> usize {
    (usize::BITS - max_inclusive.leading_zeros()) as usize
}
