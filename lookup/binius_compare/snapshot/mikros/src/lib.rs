//! Binary mixed-tensor Hadamard reduction from `latex/mikros/appx-tech.tex`.
//!
//! [`verify`] checks the PIOP and returns three deferred matrix evaluations.
//! A caller must authenticate **all three** against the inputs/commitments.
//! [`check_claims`] is a linear-time source oracle for tests and benchmarks,
//! not a polynomial commitment scheme. Bind the public statement or commitment
//! roots into the challenger before calling either [`prove`] or [`verify`].

pub mod flock_baseline;
pub mod hadamard;
mod inner_product;
pub mod layout;
pub mod lookup;
pub mod tensor;
mod weights;

pub use hadamard::{
    Claims, Error, MatrixClaim, Proof, ProverScratch, check_claims, prove, prove_with_scratch,
    verify,
};
pub use inner_product::InnerProductProof;
pub use tensor::Kernel;
