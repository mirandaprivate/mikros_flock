#![recursion_limit = "256"]

#[allow(dead_code)]
#[path = "support/paper_entry.rs"]
mod paper_entry;
#[allow(dead_code)]
#[path = "../examples/e2e_plonk_fri_compare.rs"]
mod worker;

fn main() {
    paper_entry::run_driver(
        "fri_flock_compare_bench.py",
        "circuit-sat",
        worker::run_with_args,
    );
}
