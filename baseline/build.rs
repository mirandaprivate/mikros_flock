use std::{env, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=RUSTC");
    println!("cargo:rerun-if-env-changed=CARGO_ENCODED_RUSTFLAGS");
    let flags = env::var("CARGO_ENCODED_RUSTFLAGS")
        .unwrap_or_default()
        .replace('\u{1f}', " ");
    println!("cargo:rustc-env=BASELINES_BUILD_RUSTFLAGS={flags}");
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    if let Ok(output) = Command::new(rustc).arg("-Vv").output()
        && output.status.success()
    {
        let version = String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace('\n', "; ");
        println!("cargo:rustc-env=BASELINES_BUILD_RUSTC={version}");
    }
}
