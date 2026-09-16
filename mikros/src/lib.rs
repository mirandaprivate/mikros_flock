//! Binary univariate Hadamard reduction over GF(256), from `latex/4-block.tex`.
//!
//! [`verify`] checks the PIOP and returns three deferred matrix evaluations.
//! A caller must authenticate **all three** against the inputs/commitments.
//! [`check_claims`] is a linear-time source oracle for tests and benchmarks,
//! not a polynomial commitment scheme. Bind the public statement or commitment
//! roots into the challenger before calling either [`prove`] or [`verify`].

pub mod e2e_lincheck;
pub mod extension_product;
pub mod flock_baseline;
pub mod hadamard;
mod inner_product;
pub mod layout;
pub mod lookup;
pub mod mixed_tensor;
pub mod multiple_gate;
pub mod multiple_image;
pub mod packed_sources;
pub mod sparse_lookup;
pub mod packed_ip;
pub mod tensor;
pub mod univariate;
pub mod weighted_hadamard;
mod weights;

pub use hadamard::{
    Claims, Error, MatrixClaim, Proof, ProverScratch, check_claims, prove, prove_with_scratch,
    verify,
};
pub use inner_product::InnerProductProof;
pub use univariate::Kernel;

/// Historical mixed-tensor protocol and benchmark API. Its transcript and
/// certificate are intentionally distinct from the default univariate proof.
pub mod legacy {
    pub use crate::flock_baseline;
    pub use crate::mixed_tensor as hadamard;
    pub use crate::mixed_tensor::*;
    pub use crate::tensor;
    pub use crate::tensor::Kernel;
}
