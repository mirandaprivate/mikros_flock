//! Table 4: batches of 256 independent hashes with Mikros's FRI backend.
#![recursion_limit = "256"]

#[path = "support/paper_entry.rs"]
mod paper_entry;
#[allow(unused_attributes, unused_imports)]
#[path = "../examples/e2e_hash_compare.rs"]
mod worker;
#[allow(unused_imports)]
pub(crate) use worker::{e2e_common, hash_fixture, hash_mikros, hash_single_plonk};

fn main() {
    paper_entry::run_driver("multi_hash_fri_bench.py", "multiple", worker::run_with_args);
}
