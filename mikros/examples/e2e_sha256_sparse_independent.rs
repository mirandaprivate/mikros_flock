//! Legacy independent-block sparse lookup E2E, retained for comparison.
#![recursion_limit = "256"]
#![allow(dead_code)]
mod e2e_common;
#[path = "e2e_common/sparse_sha.rs"]
mod sparse_sha;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    sparse_sha::main()
}
