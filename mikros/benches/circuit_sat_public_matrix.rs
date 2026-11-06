#![recursion_limit = "256"]

#[allow(dead_code)]
#[path = "support/paper_entry.rs"]
mod paper_entry;
#[allow(dead_code)]
#[path = "../examples/e2e_public_matrix_compare.rs"]
mod worker;

fn main() {
    paper_entry::run_driver(
        "public_matrix_bench.py",
        "circuit-sat-public-matrix",
        worker::run_with_args,
    );
}
