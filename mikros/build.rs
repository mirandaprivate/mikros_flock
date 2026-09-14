use std::{env, fs, path::Path, process::Command};

fn collect_sources(directory: &Path, paths: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(directory).expect("read Flock source directory") {
        let path = entry.expect("read source entry").path();
        if path.is_dir() {
            collect_sources(&path, paths);
        } else if path.extension().is_some_and(|ext| ext == "rs")
            || path.file_name().is_some_and(|name| name == "Cargo.toml")
        {
            paths.push(path);
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=RUSTC");
    let manifest = env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory");
    let root = Path::new(&manifest).parent().expect("workspace root");
    let crates = root.join("crates");
    println!("cargo:rerun-if-changed={}", crates.display());
    let mut paths = Vec::new();
    collect_sources(&crates, &mut paths);
    paths.sort();
    let mut generated = String::from("&[\n");
    for path in paths {
        let name = path
            .strip_prefix(root)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        generated.push_str(&format!(
            "({name:?}, include_bytes!({:?}).as_slice()),\n",
            path
        ));
    }
    generated.push_str("]\n");
    let output = env::var_os("OUT_DIR").expect("build output directory");
    fs::write(Path::new(&output).join("flock_sources.rs"), generated)
        .expect("write embedded Flock source inventory");
    let mikros = root.join("mikros");
    let mut paths = vec![mikros.join("Cargo.toml"), mikros.join("build.rs")];
    for directory in ["src", "tests", "benches"] {
        println!(
            "cargo:rerun-if-changed={}",
            mikros.join(directory).display()
        );
        collect_sources(&mikros.join(directory), &mut paths);
    }
    paths.sort();
    let mut generated = String::from("&[\n");
    for path in paths {
        let name = path
            .strip_prefix(root)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        generated.push_str(&format!(
            "({name:?}, include_bytes!({path:?}).as_slice()),\n"
        ));
    }
    generated.push_str("]\n");
    fs::write(Path::new(&output).join("mikros_sources.rs"), generated)
        .expect("write embedded Mikros source inventory");
    let rustc = env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    if let Ok(output) = Command::new(rustc).arg("-Vv").output()
        && output.status.success()
    {
        let version = String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace('\n', "; ");
        println!("cargo:rustc-env=MIKROS_BUILD_RUSTC={version}");
    }
}
