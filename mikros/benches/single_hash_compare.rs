#![recursion_limit = "256"]

#[allow(dead_code)]
#[path = "support/paper_entry.rs"]
mod paper_entry;
#[allow(dead_code)]
#[path = "../examples/e2e_plonk_compare.rs"]
mod worker;

fn main() {
    paper_entry::run_driver("single_constraints_bench.py", "single-constraints", |args| {
        worker::run_with_args(args);
        Ok::<(), String>(())
    });
}
