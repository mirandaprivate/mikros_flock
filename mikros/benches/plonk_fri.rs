//! Cargo entry point for additive-FRI-backed binary PLONK.
#![recursion_limit = "256"]

#[allow(dead_code)]
#[path = "support/paper_entry.rs"]
mod paper_entry;
#[allow(dead_code, unused_imports, unused_attributes)]
#[path = "../examples/e2e_plonk_fri.rs"]
mod worker;

fn main() {
    paper_entry::run_driver("plonk_fri_bench.py", "plonk-fri", worker::run_with_args);
}
