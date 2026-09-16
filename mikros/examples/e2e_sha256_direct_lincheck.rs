//! 256 SHA compressions: Mikros reduction, public native CSC, C=I, native PCS.
#![recursion_limit = "256"]
#![allow(dead_code)]
mod e2e_common;
#[path = "e2e_common/public_csc_sha.rs"]
mod public_csc_sha;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    public_csc_sha::main()
}
