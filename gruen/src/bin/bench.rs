use gruen_experiments::{field::*, *};
use std::{collections::BTreeMap, hint::black_box, io::Write, path::PathBuf, time::Instant};

struct Options {
    ns: Vec<usize>,
    reps: usize,
    fields: Vec<String>,
    out: PathBuf,
}
fn options() -> Result<Options, String> {
    let mut o = Options {
        ns: vec![12, 16, 20],
        reps: 5,
        fields: vec![
            "F251".into(),
            "F65521".into(),
            "Mersenne31".into(),
            "Goldilocks".into(),
        ],
        out: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("results"),
    };
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        if flag == "--help" {
            println!(
                "gruen-bench [--n 12,16,20] [--reps 5] [--fields F251,F65521,Mersenne31,Goldilocks] [--out PATH]\nSkip policy is fixed: kappa=extension degree, k=log2(kappa), block_size=kappa."
            );
            std::process::exit(0);
        }
        let value = args.next().ok_or("missing option value")?;
        match flag.as_str() {
            "--n" => {
                o.ns = value
                    .split(',')
                    .map(|s| s.parse().map_err(|_| "invalid n".into()))
                    .collect::<Result<_, String>>()?
            }
            "--reps" => o.reps = value.parse().map_err(|_| "invalid reps")?,
            "--fields" => o.fields = value.split(',').map(String::from).collect(),
            "--out" => o.out = value.into(),
            _ => return Err(format!("unknown argument {flag}")),
        }
    }
    if o.ns.is_empty() || o.ns.iter().any(|&n| n > 24) || !(1..=100).contains(&o.reps) {
        return Err("require nonempty n<=24, 1<=reps<=100".into());
    }
    if o.fields.is_empty()
        || o.fields
            .iter()
            .any(|f| !["F251", "F65521", "Mersenne31", "Goldilocks"].contains(&f.as_str()))
    {
        return Err("unrecognized/empty field list".into());
    }
    Ok(o)
}

fn median(v: impl Iterator<Item = f64>) -> f64 {
    let mut a: Vec<_> = v.collect();
    a.sort_by(f64::total_cmp);
    if a.len() % 2 == 1 {
        a[a.len() / 2]
    } else {
        (a[a.len() / 2 - 1] + a[a.len() / 2]) / 2.0
    }
}

fn run<B: Prime + Extension<B>, E: Extension<B>>(
    o: &Options,
    raw: &mut std::fs::File,
    summary: &mut std::fs::File,
) -> Result<(), String> {
    E::validate();
    let k = kappa_skip_rounds::<B, E>()?;
    if o.ns.iter().any(|&n| n < k) {
        return Err(format!("{} requires n >= log2(kappa) = {k}", B::NAME));
    }
    let domain_label = poly::SkipDomain::<B>::new(k)?.label();
    for &n in &o.ns {
        let trace = Trace::<B>::random(n, 0x475255454e);
        let tape = Challenges::<E>::deterministic(n, 0x534b4950);
        let methods = [Method::Regular, Method::Gruen, Method::Skip(k)];
        let mut oracle = BTreeMap::new();
        for method in &methods {
            let k = method.skip();
            if let std::collections::btree_map::Entry::Vacant(entry) = oracle.entry(k) {
                let t = Instant::now();
                let v = oracle_evaluations(&trace, &tape, k)?;
                entry.insert((v, t.elapsed().as_secs_f64() * 1000.0));
            }
            let (p, _) = prove(&trace, &tape, *method)?;
            verify::<B, E>(&p, &tape, n, k, oracle[&k].0)?;
            black_box(p);
        }
        println!(
            "{} n={} kappa={} skip={} domain={} warmups+oracle checks passed ({} variants)",
            B::NAME,
            n,
            E::DEGREE,
            k,
            domain_label,
            methods.len()
        );
        let mut samples: Vec<Vec<(Timings, f64, usize)>> = vec![Vec::new(); methods.len()];
        // Rotate method ordering per repetition; all runs are serial, single thread.
        for rep in 0..o.reps {
            for offset in 0..methods.len() {
                let index = (offset + rep) % methods.len();
                let method = methods[index];
                let (proof, t) = prove(black_box(&trace), black_box(&tape), method)?;
                let vt = Instant::now();
                verify::<B, E>(&proof, &tape, n, method.skip(), oracle[&method.skip()].0)?;
                let verify_ms = vt.elapsed().as_secs_f64() * 1000.0;
                let elements = proof.field_elements();
                black_box(&proof);
                writeln!(
                    raw,
                    "{},{},{},{},{},{},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{},{},{},{}",
                    B::NAME,
                    E::DEGREE,
                    n,
                    method.label(),
                    method.skip(),
                    rep,
                    t.total.as_secs_f64() * 1000.0,
                    t.setup.as_secs_f64() * 1000.0,
                    t.quotient.as_secs_f64() * 1000.0,
                    t.initial_fold.as_secs_f64() * 1000.0,
                    t.rounds.as_secs_f64() * 1000.0,
                    t.round_fold.as_secs_f64() * 1000.0,
                    verify_ms,
                    elements,
                    E::DEGREE,
                    1usize << method.skip(),
                    if method.skip() == 0 {
                        "none"
                    } else {
                        domain_label
                    }
                )
                .map_err(|e| e.to_string())?;
                samples[index].push((t, verify_ms, elements));
            }
            raw.flush().map_err(|e| e.to_string())?;
            println!(
                "{} n={} repetition {}/{} complete",
                B::NAME,
                n,
                rep + 1,
                o.reps
            );
        }
        let baseline = median(
            samples[0]
                .iter()
                .map(|(t, _, _)| t.total.as_secs_f64() * 1000.0),
        );
        for (method, s) in methods.iter().zip(&samples) {
            let total = median(s.iter().map(|(t, _, _)| t.total.as_secs_f64() * 1000.0));
            let min = s
                .iter()
                .map(|(t, _, _)| t.total.as_secs_f64() * 1000.0)
                .fold(f64::INFINITY, f64::min);
            let max = s
                .iter()
                .map(|(t, _, _)| t.total.as_secs_f64() * 1000.0)
                .fold(0.0, f64::max);
            let phases = [
                median(s.iter().map(|(t, _, _)| t.setup.as_secs_f64() * 1000.0)),
                median(s.iter().map(|(t, _, _)| t.quotient.as_secs_f64() * 1000.0)),
                median(
                    s.iter()
                        .map(|(t, _, _)| t.initial_fold.as_secs_f64() * 1000.0),
                ),
                median(s.iter().map(|(t, _, _)| t.rounds.as_secs_f64() * 1000.0)),
                median(
                    s.iter()
                        .map(|(t, _, _)| t.round_fold.as_secs_f64() * 1000.0),
                ),
            ];
            writeln!(summary, "{},{},{},{},{},{},{:.6},{:.6},{:.6},{:.4},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{:.6},{},{},{},{},{}",
                B::NAME, E::DEGREE, n, method.label(), method.skip(), o.reps, total, min, max, baseline/total,
                phases[0], phases[1], phases[2], phases[3], phases[4],
                median(s.iter().map(|(_,v,_)|*v)), oracle[&method.skip()].1,
                s[0].2, s[0].2*std::mem::size_of::<E>(), E::DEGREE, 1usize << method.skip(),
                if method.skip() == 0 { "none" } else { domain_label }).map_err(|e|e.to_string())?;
            println!(
                "  {:<18} {:>10.3} ms  {:>6.2}x",
                method.label(),
                total,
                baseline / total
            );
        }
        summary.flush().map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let o = options()?;
    std::fs::create_dir_all(&o.out).map_err(|e| e.to_string())?;
    // Refuse accidental overwrites of completed measurements.
    let create = |name| {
        std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(o.out.join(name))
            .map_err(|e| e.to_string())
    };
    let mut raw = create("raw.csv")?;
    let mut summary = create("summary.csv")?;
    writeln!(raw,"field,extension_degree,n,method,k,rep,total_ms,setup_ms,quotient_ms,initial_fold_ms,rounds_ms,round_fold_ms,verify_reduction_ms,proof_field_elements,kappa,block_size,interpolation_domain").map_err(|e|e.to_string())?;
    writeln!(summary,"field,extension_degree,n,method,k,reps,median_ms,min_ms,max_ms,speedup_vs_regular,setup_ms,quotient_ms,initial_fold_ms,rounds_ms,round_fold_ms,verify_reduction_ms,oracle_ms,proof_field_elements,proof_element_bytes,kappa,block_size,interpolation_domain").map_err(|e|e.to_string())?;
    for f in &o.fields {
        match f.as_str() {
            "F251" => run::<F251, E251>(&o, &mut raw, &mut summary)?,
            "F65521" => run::<F65521, E65521>(&o, &mut raw, &mut summary)?,
            "Mersenne31" => run::<Mersenne31, EMersenne31>(&o, &mut raw, &mut summary)?,
            "Goldilocks" => run::<Goldilocks, EGoldilocks>(&o, &mut raw, &mut summary)?,
            _ => unreachable!(),
        }
    }
    println!("Wrote {}", o.out.display());
    Ok(())
}
