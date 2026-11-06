//! Cargo entry point for the RAAFold-backed binary PLONK experiment.
#![recursion_limit = "256"]

#[allow(dead_code)]
#[path = "support/paper_entry.rs"]
mod paper_entry;
#[allow(dead_code, unused_imports, unused_attributes)]
#[path = "../examples/e2e_plonk_raafold.rs"]
mod worker;

fn main() {
    paper_entry::run_driver(
        "plonk_raafold_bench.py",
        "plonk-raafold",
        worker::run_with_args,
    );
}
