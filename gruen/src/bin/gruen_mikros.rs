use gruen_experiments::{
    field::*,
    mikros::{self, Interpolation},
    *,
};
use std::{
    fs::{File, OpenOptions},
    hint::black_box,
    io::Write,
    path::PathBuf,
    time::Instant,
};

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
        fields: vec!["Goldilocks".into(), "KoalaBear".into()],
        out: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("results/mikros-comparison"),
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--help" {
            println!(
                "gruen-mikros-bench --n 12,16,20 --reps 5 --fields Goldilocks,KoalaBear --out NEW_DIRECTORY\nSingle-thread scalar PIOP comparison; k=log2(extension degree). Every proof verified; Mikros reference/optimized messages must match."
            );
            std::process::exit(0);
        }
        let value = args.next().ok_or("missing option value")?;
        match arg.as_str() {
            "--n" => {
                o.ns = value
                    .split(',')
                    .map(|s| s.parse().map_err(|_| "invalid n".to_string()))
                    .collect::<Result<_, _>>()?
            }
            "--reps" => o.reps = value.parse().map_err(|_| "invalid reps")?,
            "--fields" => o.fields = value.split(',').map(String::from).collect(),
            "--out" => o.out = value.into(),
            _ => return Err(format!("unknown option {arg}")),
        }
    }
    if o.ns.is_empty()
        || o.ns.iter().any(|&n| !(2..=24).contains(&n))
        || !(1..=100).contains(&o.reps)
        || o.fields.is_empty()
        || o.fields
            .iter()
            .any(|s| !["Goldilocks", "KoalaBear"].contains(&s.as_str()))
    {
        return Err("require n=2..24, reps=1..100, fields Goldilocks/KoalaBear".into());
    }
    Ok(o)
}
const LABELS: [&str; 4] = [
    "gruen_2eval",
    "gruen_skip_kappa",
    "mikros_lagrange_reference",
    "mikros_lagrange_optimized",
];
enum Message<E: Field> {
    Gruen(Proof<E>),
    Mikros(mikros::Proof<E>),
}
struct Sample {
    total: f64,
    phases: [f64; 7],
    verify: f64,
    elements: usize,
}
fn ms(d: std::time::Duration) -> f64 {
    d.as_secs_f64() * 1000.
}
fn generate<B: Prime + Extension<B>, E: Extension<B>>(
    trace: &Trace<B>,
    tape: &Challenges<E>,
    variant: usize,
) -> Result<(Message<E>, Sample), String> {
    let k = E::DEGREE.ilog2() as usize;
    let start = Instant::now();
    let (message, phases, elements) = if variant < 2 {
        let (p, t) = prove(
            black_box(trace),
            black_box(tape),
            if variant == 0 {
                Method::Gruen
            } else {
                Method::Skip(k)
            },
        )?;
        let phases = [
            ms(t.setup),
            ms(t.quotient),
            0.,
            ms(t.initial_fold),
            ms(t.rounds),
            ms(t.round_fold),
            0.,
        ];
        let elements = p.field_elements();
        (Message::Gruen(p), phases, elements)
    } else {
        let (p, t) = mikros::prove(
            black_box(trace),
            black_box(tape),
            if variant == 2 {
                Interpolation::Reference
            } else {
                Interpolation::Optimized
            },
        )?;
        let phases = [
            ms(t.setup),
            ms(t.helpers),
            ms(t.packing_weights),
            ms(t.packing),
            ms(t.rounds),
            ms(t.round_fold),
            ms(t.terminal),
        ];
        let elements = p.field_elements();
        (Message::Mikros(p), phases, elements)
    };
    // Includes function-exit cleanup; all variants use this same outer timer.
    Ok((
        message,
        Sample {
            total: ms(start.elapsed()),
            phases,
            verify: 0.,
            elements,
        },
    ))
}
fn check<B: Prime, E: Extension<B>>(
    message: &Message<E>,
    tape: &Challenges<E>,
    n: usize,
    k: usize,
    trusted: [E; 3],
) -> Result<(), String> {
    match message {
        Message::Gruen(p) => verify::<B, E>(p, tape, n, k, trusted),
        Message::Mikros(p) => mikros::verify::<B, E>(p, tape, n, trusted),
    }
}
fn median(mut xs: Vec<f64>) -> f64 {
    xs.sort_by(f64::total_cmp);
    let n = xs.len();
    if n % 2 == 1 {
        xs[n / 2]
    } else {
        (xs[n / 2 - 1] + xs[n / 2]) / 2.
    }
}
fn med(samples: &[Sample], f: impl Fn(&Sample) -> f64) -> f64 {
    median(samples.iter().map(f).collect())
}
fn run<B: Prime + Extension<B>, E: Extension<B>>(
    o: &Options,
    raw: &mut File,
    summary: &mut File,
) -> Result<usize, String> {
    E::validate();
    let k = kappa_skip_rounds::<B, E>()?;
    let mut count = 0;
    for &n in &o.ns {
        let trace = Trace::<B>::random(n, 0x4d494b524f535f47);
        let tape = Challenges::<E>::deterministic(n, 0x5041495245445f31);
        let mut oracles = Vec::new();
        for variant in 0..3 {
            let t = Instant::now();
            let trusted = match variant {
                0 => oracle_evaluations(&trace, &tape, 0)?,
                1 => oracle_evaluations(&trace, &tape, k)?,
                _ => mikros::oracle_evaluations(&trace, &tape)?,
            };
            oracles.push((trusted, ms(t.elapsed())));
        }
        // One warmup per variant, including all witness-independent setup.
        for variant in 0..4 {
            let (p, _) = generate(&trace, &tape, variant)?;
            check::<B, E>(
                &p,
                &tape,
                n,
                if variant == 0 { 0 } else { k },
                oracles[variant.min(2)].0,
            )?;
            black_box(p);
            count += 1;
        }
        let mut samples: Vec<Vec<Sample>> = (0..4).map(|_| Vec::new()).collect();
        for rep in 0..o.reps {
            let mut results: Vec<_> = (0..4).map(|_| None).collect();
            for order in 0..4 {
                let variant = (rep + order) % 4;
                results[variant] = Some(generate(&trace, &tape, variant)?);
            }
            // All reduction checks happen AFTER the four timed provers. Dense
            // source checks are precomputed above, never between measurements.
            for (variant, result) in results.iter_mut().enumerate() {
                let (message, sample) = result.as_mut().unwrap();
                let t = Instant::now();
                check::<B, E>(
                    message,
                    &tape,
                    n,
                    if variant == 0 { 0 } else { k },
                    oracles[variant.min(2)].0,
                )?;
                sample.verify = ms(t.elapsed());
                count += 1;
            }
            let (Message::Mikros(a), _) = &results[2].as_ref().unwrap() else {
                unreachable!()
            };
            let (Message::Mikros(b), _) = &results[3].as_ref().unwrap() else {
                unreachable!()
            };
            if a != b {
                return Err("Mikros interpolation optimization changed the proof".into());
            }
            for (variant, result) in results.into_iter().enumerate() {
                let (proof, s) = result.unwrap();
                black_box(proof);
                write!(
                    raw,
                    "{},{},{},{},{},{},{},{:.6}",
                    B::NAME,
                    E::DEGREE,
                    n,
                    LABELS[variant],
                    if variant == 0 { 0 } else { k },
                    rep,
                    (variant + 4 - rep % 4) % 4,
                    s.total
                )
                .map_err(|e| e.to_string())?;
                for phase in s.phases {
                    write!(raw, ",{phase:.6}").map_err(|e| e.to_string())?;
                }
                writeln!(
                    raw,
                    ",{:.6},{:.6},{},{},true,true",
                    s.verify,
                    oracles[variant.min(2)].1,
                    s.elements,
                    s.elements * std::mem::size_of::<E>()
                )
                .map_err(|e| e.to_string())?;
                samples[variant].push(s);
            }
            raw.flush().map_err(|e| e.to_string())?;
        }
        let gruen = med(&samples[0], |s| s.total);
        let skip = med(&samples[1], |s| s.total);
        let reference = med(&samples[2], |s| s.total);
        for variant in 0..4 {
            let s = &samples[variant];
            let total = med(s, |s| s.total);
            let min = s.iter().map(|s| s.total).fold(f64::INFINITY, f64::min);
            let max = s.iter().map(|s| s.total).fold(0., f64::max);
            let mad = median(s.iter().map(|s| (s.total - total).abs()).collect());
            write!(
                summary,
                "{},{},{},{},{},{},{:.6},{min:.6},{max:.6},{mad:.6},{:.6},{:.6},{:.6}",
                B::NAME,
                E::DEGREE,
                n,
                LABELS[variant],
                if variant == 0 { 0 } else { k },
                o.reps,
                total,
                gruen / total,
                skip / total,
                reference / total
            )
            .map_err(|e| e.to_string())?;
            for i in 0..7 {
                write!(summary, ",{:.6}", med(s, |s| s.phases[i])).map_err(|e| e.to_string())?;
            }
            writeln!(
                summary,
                ",{:.6},{:.6},{},{},{},{}",
                med(s, |s| s.verify),
                oracles[variant.min(2)].1,
                s[0].elements,
                s[0].elements * std::mem::size_of::<E>(),
                std::mem::size_of::<B>(),
                std::mem::size_of::<E>()
            )
            .map_err(|e| e.to_string())?;
            println!(
                "{} n={} {:<29} {:.3} ms [{:.3},{:.3}]",
                B::NAME,
                n,
                LABELS[variant],
                total,
                min,
                max
            );
        }
        summary.flush().map_err(|e| e.to_string())?;
    }
    Ok(count)
}
fn main() -> Result<(), String> {
    let o = options()?;
    std::fs::create_dir_all(&o.out).map_err(|e| e.to_string())?;
    let create = |name| {
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(o.out.join(name))
            .map_err(|e| e.to_string())
    };
    let mut raw = create("raw.csv")?;
    let mut summary = create("summary.csv")?;
    writeln!(raw,"field,extension_degree,n,method,k,rep,order,total_ms,setup_ms,helpers_or_quotient_ms,packing_weights_ms,packing_ms,rounds_ms,round_fold_ms,terminal_ms,verify_reduction_ms,oracle_ms,proof_field_elements,proof_element_bytes,verified,mikros_pair_equal").map_err(|e|e.to_string())?;
    writeln!(summary,"field,extension_degree,n,method,k,reps,median_ms,min_ms,max_ms,mad_ms,gruen_2eval_over_method,gruen_skip_over_method,mikros_reference_over_method,setup_ms,helpers_or_quotient_ms,packing_weights_ms,packing_ms,rounds_ms,round_fold_ms,terminal_ms,verify_reduction_ms,oracle_ms,proof_field_elements,proof_element_bytes,base_storage_bytes,extension_storage_bytes").map_err(|e|e.to_string())?;
    let mut count = 0;
    for field in &o.fields {
        count += match field.as_str() {
            "Goldilocks" => run::<Goldilocks, EGoldilocks>(&o, &mut raw, &mut summary)?,
            "KoalaBear" => run::<KoalaBear, EKoalaBear>(&o, &mut raw, &mut summary)?,
            _ => unreachable!(),
        };
    }
    writeln!(create("completion.json")?,"{{\"completed\":true,\"verified_proofs_including_warmups\":{count},\"mikros_optimization_pairs_equal\":{},\"fields\":{},\"sizes\":{},\"repetitions\":{}}}",o.ns.len()*o.fields.len()*o.reps,o.fields.len(),o.ns.len(),o.reps).map_err(|e|e.to_string())?;
    println!("Verified {count} proofs; saved {}", o.out.display());
    Ok(())
}
