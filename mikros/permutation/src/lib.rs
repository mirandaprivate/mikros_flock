//! Permutation/Lasso PIOP reductions over F128. Source evaluations returned by
//! the verifier require authentication by the caller's PCS. Tests use a
//! linear-time checker to evaluate these claims directly on the inputs.
#![doc = include_str!("../README.md")]

pub mod bits;
pub mod claims;
mod fast_sumcheck;
pub mod field;
mod kernels;
pub mod packed;
pub mod permutation;
#[path = "fast_tree.rs"]
mod product_tree;
pub mod profile;
pub mod tree_pool;
pub mod wiring;
mod reference {
    pub(crate) mod product_tree;
    pub(crate) mod sumcheck;
}
#[cfg(test)]
mod lockstep_tests;
#[cfg(test)]
mod packed_tests;
#[cfg(test)]
mod product_tree_tests;
mod sumcheck;

use serde::{Deserialize, Serialize};

pub type Digest = [u8; 32];
pub const EXTENSION_BITS: usize = 128;
/// A 2^30-gate PLONK instance has 3N+I/O copy positions, padded to 2^32.
/// Storage sizes are still checked by PackedBits before allocation.
pub const MAX_LOG_ROWS: usize = if usize::BITS > 32 { 32 } else { 30 };

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

#[cfg(all(test, target_pointer_width = "64"))]
mod plonk_capacity_tests {
    #[test]
    fn copy_domain_accepts_four_billion_positions_without_allocating() {
        use super::{bits::PackedBits, permutation::CommittedStatement};
        assert!(CommittedStatement::new(1usize << 32, 64).is_ok());
        assert!(CommittedStatement::new((1usize << 32) + 1, 64).is_err());
        assert!(PackedBits::new((1usize << 32) + 1, 1, vec![]).is_err());
        // A huge shape alone never makes an undersized backing store valid.
        assert!(PackedBits::new(1usize << 32, 64, vec![]).is_err());
    }
}
