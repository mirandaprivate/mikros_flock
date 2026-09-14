fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if let Ok(output) = std::process::Command::new(std::env::var("RUSTC").unwrap_or("rustc".into()))
        .arg("-Vv")
        .output()
        && output.status.success()
    {
        println!(
            "cargo:rustc-env=BENCH_RUSTC={}",
            String::from_utf8_lossy(&output.stdout)
                .trim()
                .replace('\n', "; ")
        );
    }
}
