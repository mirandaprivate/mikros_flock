//! Field-width sweep: single PLONK and 256-instance packed R1CS, both with F128 FRI.
#![recursion_limit = "256"]

#[allow(dead_code, unused_imports, unused_attributes)]
#[path = "../examples/e2e_r1cs_bitwidth_fri.rs"]
mod multiple_worker;
#[allow(dead_code)]
#[path = "support/paper_entry.rs"]
mod paper_entry;
#[allow(dead_code, unused_imports, unused_attributes)]
#[path = "../examples/e2e_plonk_fri.rs"]
mod single_worker;

fn worker(mut args: Vec<String>) -> Result<(), String> {
    if args.len() < 2 || args[0] != "--case" {
        return Err("worker requires --case single|multiple before other arguments".into());
    }
    let case = args.remove(1);
    args.remove(0);
    match case.as_str() {
        "single" => single_worker::run_with_args(args),
        "multiple" => multiple_worker::run_with_args(args),
        _ => Err(format!("unknown bitwidth experiment case {case}")),
    }
}

fn main() {
    paper_entry::run_driver("bitwidth_instances_bench.py", "bitwidth-instances", worker);
}
