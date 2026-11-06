//! Compare binary PLONK with additive FRI against native FLOCK public-CSC R1CS.
//! The Python bench driver launches one backend per process.
#![recursion_limit = "256"]
#![allow(dead_code)]

#[allow(unused_attributes)]
#[path = "e2e_plonk_fri.rs"]
mod fri;
#[allow(unused_attributes)]
#[path = "e2e_plonk_compare.rs"]
mod native;

fn route_arguments(args: Vec<String>) -> Result<(String, Vec<String>), String> {
    let mut system = None;
    let mut forwarded = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    let mut args = args.into_iter();
    while let Some(key) = args.next() {
        let value = args
            .next()
            .ok_or_else(|| format!("missing value for {key}"))?;
        if !seen.insert(key.clone()) {
            return Err(format!("repeated option {key}"));
        }
        match key.as_str() {
            "--system" if system.is_none() => system = Some(value),
            "--circuit" => {
                if value != "paired" && value != "near-full" {
                    return Err(format!("unknown circuit {value}"));
                }
                forwarded.extend([key, value]);
            }
            "--log-gates" | "--threads" | "--repetitions" | "--warmups" | "--seed" => {
                forwarded.extend([key, value]);
            }
            _ => return Err(format!("unknown or repeated option {key}")),
        }
    }
    let system = system.unwrap_or_else(|| "mikros".into());
    match system.as_str() {
        "mikros" => (),
        "flock" => forwarded.extend([
            "--system".into(),
            "flock".into(),
            "--mode".into(),
            "single".into(),
        ]),
        _ => {
            return Err(
                "worker requires --system mikros or flock; use the bench driver for both".into(),
            );
        }
    }
    Ok((system, forwarded))
}

pub fn run_with_args(args: Vec<String>) -> Result<(), String> {
    let (system, forwarded) = route_arguments(args)?;
    if system == "mikros" {
        fri::run_with_args(forwarded)
    } else {
        native::run_with_args(forwarded);
        Ok(())
    }
}

fn main() {
    if let Err(error) = run_with_args(std::env::args().skip(1).collect()) {
        eprintln!("FRI/FLOCK comparison worker failed: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|s| (*s).into()).collect()
    }
    #[test]
    fn routes_only_the_selected_system_and_keeps_native_single_mode() {
        let base = ["--log-gates", "14", "--threads", "4"];
        assert_eq!(
            route_arguments(args(&base)).unwrap(),
            ("mikros".into(), args(&base))
        );
        let (system, forwarded) =
            route_arguments(args(&["--system", "flock", "--log-gates", "14"])).unwrap();
        assert_eq!(system, "flock");
        assert_eq!(
            forwarded,
            args(&["--log-gates", "14", "--system", "flock", "--mode", "single"])
        );
        for system in ["mikros", "flock"] {
            let (selected, forwarded) = route_arguments(args(&[
                "--system",
                system,
                "--circuit",
                "near-full",
                "--log-gates",
                "30",
            ]))
            .unwrap();
            assert_eq!(selected, system);
            assert_eq!(
                &forwarded[..4],
                &args(&["--circuit", "near-full", "--log-gates", "30"])
            );
        }
        for invalid in [
            args(&["--circuit", "other"]),
            args(&["--circuit", "near-full", "--circuit", "paired"]),
            args(&["--system", "both"]),
            args(&["--mode", "multiple"]),
            args(&["--system", "flock", "--system", "mikros"]),
            args(&["--threads"]),
            args(&["--threads", "1", "--threads", "4"]),
            args(&[
                "--system",
                "flock",
                "--log-gates",
                "14",
                "--log-gates",
                "30",
            ]),
        ] {
            assert!(route_arguments(invalid).is_err());
        }
    }
}
