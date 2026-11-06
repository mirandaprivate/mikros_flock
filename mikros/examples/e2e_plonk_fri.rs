//! Complete binary PLONK proofs with additive-FRI packed MLE authentication.
#![recursion_limit = "256"]
#![allow(dead_code)]
#[path = "e2e_common/packed_plonk_driver.rs"]
mod driver;
#[path = "e2e_plonk_fri/pcs.rs"]
mod pcs;

pub use driver::run_with_args;

fn main() {
    if let Err(error) = run_with_args(std::env::args().skip(1).collect()) {
        eprintln!("PLONK additive FRI failed: {error}");
        std::process::exit(1);
    }
}
