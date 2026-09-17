//! Bitwidth-aware permutation/Lasso over F128, with optional WHIR composition.
//! `authenticated` verifies all bit oracles without a witness. The low-level
//! PIOP APIs return claims for PCS authentication or direct evaluation in tests.
#![doc = include_str!("../README.md")]

#[cfg(feature = "whir")]
pub mod authenticated;
pub mod bits;
pub mod claims;
pub mod field;
pub mod lasso;
pub mod permutation;
mod product_tree;
#[cfg(test)]
mod product_tree_tests;
mod sumcheck;
#[cfg(feature = "whir")]
mod whir_field;
#[cfg(feature = "whir")]
pub mod whir_pcs;
#[cfg(feature = "whir")]
mod whir_rs;
#[cfg(all(test, feature = "whir"))]
mod whir_tests;

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
