//! Shared-row multiple-instance R1CS with complete packed sparse lookup.
#![recursion_limit = "256"]
#![allow(dead_code)]
mod e2e_common;
#[path = "e2e_common/sparse_sha_shared.rs"]
mod sparse_sha_shared;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    sparse_sha_shared::main()
}
