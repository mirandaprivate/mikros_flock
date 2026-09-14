//! Permutation/Lasso PIOP reductions over F128. Source evaluations returned by
//! the verifier require authentication by the caller's PCS. Digest binding and
//! the direct input checker provided here are not a polynomial commitment.
#![doc = include_str!("../README.md")]

pub mod bits;
pub mod claims;
pub mod field;
pub mod permutation;
mod product_tree;
#[cfg(test)]
mod product_tree_tests;
mod sumcheck;

use serde::{Deserialize, Serialize};

pub type Digest = [u8; 32];
pub const EXTENSION_BITS: usize = 128;
pub const MAX_LOG_ROWS: usize = 30;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Error {
    Width,
    Length,
    Padding,
    Capacity,
    OutOfRange,
    Statement,
    ProofShape,
    ProductMismatch,
    Sumcheck,
    Constraint,
    Source,
    Transcript,
    Pcs,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lookup proof error: {self:?}")
    }
}
impl std::error::Error for Error {}

pub(crate) fn padded_len(n: usize) -> Result<usize, Error> {
    if n > (1usize << MAX_LOG_ROWS) {
        return Err(Error::Length);
    }
    Ok(n.max(1).next_power_of_two())
}
