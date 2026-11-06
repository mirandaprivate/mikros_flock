//! Single SHA-256, Keccak-f1600 and BLAKE3 calls with the current FRI backend.
#![recursion_limit = "256"]

#[path = "support/paper_entry.rs"]
mod paper_entry;
#[allow(unused_attributes, unused_imports)]
#[path = "../examples/e2e_hash_compare.rs"]
mod worker;
#[allow(unused_imports)]
pub(crate) use worker::{e2e_common, hash_fixture, hash_mikros, hash_single_plonk};

fn main() {
    paper_entry::run_driver("single_hash_fri_bench.py", "single", worker::run_with_args);
}
