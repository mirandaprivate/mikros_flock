use std::{env, process::Command};
fn main() {
    println!("cargo:rerun-if-env-changed=RUSTFLAGS");
    println!("cargo:rerun-if-env-changed=CARGO_ENCODED_RUSTFLAGS");
    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".into());
    let version = Command::new(rustc).arg("-Vv").output().ok()
        .map(|out| String::from_utf8_lossy(&out.stdout).replace('\n', "; "))
        .unwrap_or_default();
    println!("cargo:rustc-env=MIKROS_BUILD_RUSTC={version}");
    let flags = env::var("CARGO_ENCODED_RUSTFLAGS")
        .map(|value| value.replace('\u{1f}', " "))
        .or_else(|_| env::var("RUSTFLAGS")).unwrap_or_default();
    println!("cargo:rustc-env=MIKROS_BUILD_RUSTFLAGS={flags}");
}
