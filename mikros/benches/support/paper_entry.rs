//! Shared Cargo entry point. Measurements run in fresh worker processes.

use std::{env, ffi::OsString, fmt::Display, path::PathBuf, process::Command};

pub fn run<E: Display>(experiment: &str, worker: impl FnOnce(Vec<String>) -> Result<(), E>) {
    run_driver("paper_bench.py", experiment, worker);
}

pub fn run_driver<E: Display>(
    script: &str,
    experiment: &str,
    worker: impl FnOnce(Vec<String>) -> Result<(), E>,
) {
    // Cargo passes this flag even for a custom benchmark harness.
    let mut args: Vec<String> = env::args().skip(1).filter(|arg| arg != "--bench").collect();
    if args.first().is_some_and(|arg| arg == "--worker") {
        args.remove(0);
        if let Err(error) = worker(args) {
            eprintln!("benchmark worker failed: {error}");
            std::process::exit(1);
        }
        return;
    }

    let result = (|| {
        let python = env::var_os("MIKROS_BENCH_PYTHON")
            .unwrap_or_else(|| OsString::from(if cfg!(windows) { "python" } else { "python3" }));
        let script = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("benches")
            .join(script);
        Command::new(python)
            .arg(script)
            .arg("--experiment")
            .arg(experiment)
            .arg("--worker-bin")
            .arg(env::current_exe()?)
            .args(args)
            .status()
    })();
    match result {
        Ok(status) if status.success() => (),
        Ok(status) => std::process::exit(status.code().unwrap_or(1)),
        Err(error) => {
            eprintln!("could not run benchmark driver: {error}");
            std::process::exit(1);
        }
    }
}
