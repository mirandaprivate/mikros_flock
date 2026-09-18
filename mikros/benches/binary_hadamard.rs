#[path = "support/paper_entry.rs"]
mod paper_entry;
#[allow(dead_code)]
#[path = "univariate_vs_flock.rs"]
mod worker;

fn main() {
    paper_entry::run("hadamard", worker::run_with_args);
}
