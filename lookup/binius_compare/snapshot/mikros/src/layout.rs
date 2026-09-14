//! Rows are consecutive 128-bit blocks, bytes and bits both in storage order.
//! Column `j` is bit `(j % 8)` of byte `(j / 8)` within its row. Tensor
//! coordinates are lexicographic: the first coordinate is the most significant
//! index bit. Thus row-major packed position is `row * 128 + column`.

use crate::{hadamard::Error, tensor::S};

/// Checked shape for the fixed F2 -> F128 packing configuration.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shape {
    pub log_n: usize,
    pub row_log: usize,
    pub rows: usize,
    pub bytes: usize,
}

impl Shape {
    pub fn new(log_n: usize) -> Result<Self, Error> {
        if log_n < S || log_n >= usize::BITS as usize {
            return Err(Error::InvalidLogN(log_n));
        }
        // A slice cannot contain more than isize::MAX bytes.
        let bytes = 1usize << (log_n - 3);
        if bytes > isize::MAX as usize {
            return Err(Error::InvalidLogN(log_n));
        }
        Ok(Self {
            log_n,
            row_log: log_n - S,
            rows: 1usize << (log_n - S),
            bytes,
        })
    }

    pub fn check_inputs(self, a: &[u8], b: &[u8], c: &[u8]) -> Result<(), Error> {
        for (name, input) in [("a", a), ("b", b), ("c", c)] {
            if input.len() != self.bytes {
                return Err(Error::InputLength {
                    input: name,
                    expected: self.bytes,
                    got: input.len(),
                });
            }
        }
        Ok(())
    }
}
