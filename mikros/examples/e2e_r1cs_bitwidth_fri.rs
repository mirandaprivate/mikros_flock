//! Shared public-matrix R1CS across actual binary-subfield instances.
#![recursion_limit = "256"]
#![allow(dead_code)]
#[path = "e2e_r1cs_bitwidth_fri/driver.rs"]
mod driver;
#[path = "e2e_plonk_bitwidth/fixture.rs"]
mod fixture;
#[path = "e2e_common/grinding.rs"]
mod grinding;
#[path = "e2e_plonk_bitwidth/hadamard.rs"]
mod hadamard;
#[path = "e2e_plonk_fri/pcs.rs"]
mod pcs;

pub use driver::run_with_args;
fn main() {
    if let Err(error) = run_with_args(std::env::args().skip(1).collect()) {
        eprintln!("Packed R1CS additive FRI failed: {error}");
        std::process::exit(1);
    }
}
