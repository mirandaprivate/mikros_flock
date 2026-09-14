//! Column-major bit matrix: bit (column * rows + row) is one F2 coordinate.
use crate::{Digest, Error, MAX_LOG_ROWS};
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
    pub(crate) fn set_bit(&mut self, index: usize, value: bool) {
        let mask = 1 << (index % 8);
        if value {
            self.data[index / 8] |= mask;
        } else {
            self.data[index / 8] &= !mask;
        }
    }
}

pub fn bits_for(max_inclusive: usize) -> usize {
    (usize::BITS - max_inclusive.leading_zeros()) as usize
}
