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
    threads: usize,
    systems: Vec<usize>,
    warmups: usize,
}
fn options() -> Result<Options, String> {
    let mut o = Options {
        ns: vec![12, 16, 20],
        reps: 8,
        fields: vec!["Mersenne31".into(), "KoalaBear".into(), "Goldilocks".into()],
        out: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("results/mikros-comparison"),
        threads: 64,
        systems: vec![0, 1, 2, 3],
        warmups: 1,
    };
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        // Cargo supplies this flag even for benches with harness = false.
        if arg == "--bench" {
            continue;
        }
        if arg == "--help" {
            println!(
                "compare --n 20,26,30 --reps 8 --threads 64 --fields Mersenne31,KoalaBear,Goldilocks --systems 0,1,2,3 --warmups 1 --out NEW_DIRECTORY\nParallel PIOP comparison, same scalar field arithmetic; every proof and source claim checked. Systems: 0 Gruen, 1 Gruen skip-kappa, 2 Mikros reference, 3 Mikros optimized."
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
            "--threads" => o.threads = value.parse().map_err(|_| "invalid threads")?,
            "--warmups" => o.warmups = value.parse().map_err(|_| "invalid warmups")?,
            "--systems" => {
                o.systems = value
                    .split(',')
                    .map(|v| v.parse().map_err(|_| "invalid system".to_string()))
                    .collect::<Result<_, _>>()?
            }
            _ => return Err(format!("unknown option {arg}")),
        }
    }
    if o.ns.is_empty()
        || o.ns.iter().any(|&n| !(2..=30).contains(&n))
        || !(1..=100).contains(&o.reps)
        || o.fields.is_empty()
        || !(1..=128).contains(&o.threads)
        || o.warmups > 1
        || o.systems.is_empty()
        || o.systems.iter().any(|&v| v > 3)
        || o.systems
            .iter()
            .enumerate()
            .any(|(i, v)| o.systems[..i].contains(v))
        || o.fields
            .iter()
            .any(|s| !["Mersenne31", "KoalaBear", "Goldilocks"].contains(&s.as_str()))
    {
        return Err("require n=2..30, reps=1..100, threads=1..128, distinct systems=0..3, warmups=0..1, known fields".into());
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
    cpu_ms: f64,
}
fn ms(d: std::time::Duration) -> f64 {
    d.as_secs_f64() * 1000.
}
fn cpu_ms() -> f64 {
    // Linux process CPU time sums all Rayon worker threads. clock_gettime
    // avoids assumptions about kernel tick frequency and includes short runs.
    #[repr(C)]
    struct Timespec {
        sec: i64,
        nsec: i64,
    }
    unsafe extern "C" {
        fn clock_gettime(clock: i32, time: *mut Timespec) -> i32;
    }
    let mut t = Timespec { sec: 0, nsec: 0 };
    // CLOCK_PROCESS_CPUTIME_ID=2 on Linux, the only supported run platform.
    assert_eq!(unsafe { clock_gettime(2, &mut t) }, 0);
    t.sec as f64 * 1000. + t.nsec as f64 / 1_000_000.
}
fn generate<B: Prime + Extension<B>, E: Extension<B>>(
    trace: &Trace<B>,
    tape: &Challenges<E>,
    variant: usize,
) -> Result<(Message<E>, Sample), String> {
    let k = E::DEGREE.ilog2() as usize;
    let cpu_start = cpu_ms();
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
            cpu_ms: cpu_ms() - cpu_start,
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
        eprintln!("[compare] {} n={n} input generation start", B::NAME);
        let input_start = Instant::now();
        let trace = Trace::<B>::random(n, 0x4d494b524f535f47);
        let tape = Challenges::<E>::deterministic(n, 0x5041495245445f31);
        eprintln!(
            "[compare] {} n={n} input generation finished: {:.3} ms",
            B::NAME,
            ms(input_start.elapsed())
        );
        let mut oracles = Vec::new();
        for (variant, label) in LABELS.iter().enumerate().take(3) {
            eprintln!("[compare] {} n={n} {} source oracle start", B::NAME, label);
            let t = Instant::now();
            let trusted = match variant {
                0 => oracle_evaluations(&trace, &tape, 0)?,
                1 => oracle_evaluations(&trace, &tape, k)?,
                _ => mikros::oracle_evaluations(&trace, &tape)?,
            };
            let elapsed = ms(t.elapsed());
            oracles.push((trusted, elapsed));
            eprintln!(
                "[compare] {} n={n} {} source oracle finished: {elapsed:.3} ms",
                B::NAME,
                label
            );
        }
        // One warmup per variant, including all witness-independent setup.
        for &variant in o.systems.iter().cycle().take(o.systems.len() * o.warmups) {
            eprintln!(
                "[compare] {} n={n} {} warmup 1/{} start",
                B::NAME,
                LABELS[variant],
                o.warmups
            );
            let (p, sample) = generate(&trace, &tape, variant)?;
            let verify_start = Instant::now();
            check::<B, E>(
                &p,
                &tape,
                n,
                if variant == 0 { 0 } else { k },
                oracles[variant.min(2)].0,
            )?;
            eprintln!(
                "[compare] {} n={n} {} warmup 1/{} finished: prover={:.3} ms verifier={:.3} ms proof_elements={} proof_element_bytes={} verified=true (excluded from samples)",
                B::NAME,
                LABELS[variant],
                o.warmups,
                sample.total,
                ms(verify_start.elapsed()),
                sample.elements,
                sample.elements * std::mem::size_of::<E>()
            );
            black_box(p);
            count += 1;
        }
        let mut samples: Vec<Vec<Sample>> = (0..4).map(|_| Vec::new()).collect();
        for rep in 0..o.reps {
            let mut results: Vec<_> = (0..4).map(|_| None).collect();
            for order in 0..o.systems.len() {
                let variant = o.systems[(rep + order) % o.systems.len()];
                eprintln!(
                    "[compare] {} n={n} {} sample {}/{} prover start",
                    B::NAME,
                    LABELS[variant],
                    rep + 1,
                    o.reps
                );
                let result = generate(&trace, &tape, variant)?;
                eprintln!(
                    "[compare] {} n={n} {} sample {}/{} prover finished: {:.3} ms; verification pending",
                    B::NAME,
                    LABELS[variant],
                    rep + 1,
                    o.reps,
                    result.1.total
                );
                results[variant] = Some(result);
            }
            // All reduction checks happen AFTER the four timed provers. Dense
            // source checks are precomputed above, never between measurements.
            for (variant, result) in results.iter_mut().enumerate() {
                let Some((message, sample)) = result.as_mut() else {
                    continue;
                };
                let t = Instant::now();
                check::<B, E>(
                    message,
                    &tape,
                    n,
                    if variant == 0 { 0 } else { k },
                    oracles[variant.min(2)].0,
                )?;
                sample.verify = ms(t.elapsed());
                eprintln!(
                    "[compare] {} n={n} {} sample {}/{} finished: prover={:.3} ms verifier={:.3} ms proof_elements={} proof_element_bytes={} verified=true",
                    B::NAME,
                    LABELS[variant],
                    rep + 1,
                    o.reps,
                    sample.total,
                    sample.verify,
                    sample.elements,
                    sample.elements * std::mem::size_of::<E>()
                );
                count += 1;
            }
            if let (Some((Message::Mikros(a), _)), Some((Message::Mikros(b), _))) =
                (&results[2], &results[3])
                && a != b
            {
                return Err("Mikros interpolation optimization changed the proof".into());
            }
            for (variant, result) in results.into_iter().enumerate() {
                let Some((proof, s)) = result else {
                    continue;
                };
                let coords = match &proof {
                    Message::Gruen(p) => format!("{p:?}"),
                    Message::Mikros(p) => format!("{p:?}"),
                };
                let proof_path = o
                    .out
                    .join(format!("proof-{}-n{n}-r{rep}-s{variant}.txt", B::NAME));
                let mut proof_file = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(proof_path)
                    .map_err(|e| e.to_string())?;
                writeln!(proof_file, "{coords}").map_err(|e| e.to_string())?;
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
                    (o.systems.iter().position(|&v| v == variant).unwrap() + o.systems.len()
                        - rep % o.systems.len())
                        % o.systems.len(),
                    s.total
                )
                .map_err(|e| e.to_string())?;
                for phase in s.phases {
                    write!(raw, ",{phase:.6}").map_err(|e| e.to_string())?;
                }
                writeln!(
                    raw,
                    ",{:.6},{:.6},{},{},true,{},{},{:.6}",
                    s.verify,
                    oracles[variant.min(2)].1,
                    s.elements,
                    s.elements * std::mem::size_of::<E>(),
                    if o.systems.contains(&2) && o.systems.contains(&3) {
                        "true"
                    } else {
                        "not_applicable"
                    },
                    o.threads,
                    s.cpu_ms
                )
                .map_err(|e| e.to_string())?;
                samples[variant].push(s);
            }
            raw.flush().map_err(|e| e.to_string())?;
        }
        let baseline = |i: usize| {
            if samples[i].is_empty() {
                f64::NAN
            } else {
                med(&samples[i], |s| s.total)
            }
        };
        let gruen = baseline(0);
        let skip = baseline(1);
        let reference = baseline(2);
        for &variant in &o.systems {
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
                ",{:.6},{:.6},{},{},{},{},{},{:.6}",
                med(s, |s| s.verify),
                oracles[variant.min(2)].1,
                s[0].elements,
                s[0].elements * std::mem::size_of::<E>(),
                std::mem::size_of::<B>(),
                std::mem::size_of::<E>(),
                o.threads,
                med(s, |s| s.cpu_ms)
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
    eprintln!(
        "[compare] threads={} samples={} warmups={} sizes={:?} systems={:?} output={}",
        o.threads,
        o.reps,
        o.warmups,
        o.ns,
        o.systems,
        o.out.display()
    );
    rayon::ThreadPoolBuilder::new()
        .num_threads(o.threads)
        .build_global()
        .map_err(|e| e.to_string())?;
    if rayon::current_num_threads() != o.threads {
        return Err("Rayon pool size mismatch".into());
    }
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
    writeln!(raw,"field,extension_degree,n,method,k,rep,order,total_ms,setup_ms,helpers_or_quotient_ms,packing_weights_ms,packing_ms,rounds_ms,round_fold_ms,terminal_ms,verify_reduction_ms,oracle_ms,proof_field_elements,proof_element_bytes,verified,mikros_pair_equal,threads,process_cpu_ms").map_err(|e|e.to_string())?;
    writeln!(summary,"field,extension_degree,n,method,k,reps,median_ms,min_ms,max_ms,mad_ms,gruen_2eval_over_method,gruen_skip_over_method,mikros_reference_over_method,setup_ms,helpers_or_quotient_ms,packing_weights_ms,packing_ms,rounds_ms,round_fold_ms,terminal_ms,verify_reduction_ms,oracle_ms,proof_field_elements,proof_element_bytes,base_storage_bytes,extension_storage_bytes,threads,process_cpu_ms").map_err(|e|e.to_string())?;
    let mut count = 0;
    for field in &o.fields {
        count += match field.as_str() {
            "Mersenne31" => run::<Mersenne31, EMersenne31>(&o, &mut raw, &mut summary)?,
            "Goldilocks" => run::<Goldilocks, EGoldilocks>(&o, &mut raw, &mut summary)?,
            "KoalaBear" => run::<KoalaBear, EKoalaBear>(&o, &mut raw, &mut summary)?,
            _ => unreachable!(),
        };
    }
    writeln!(create("completion.json")?,"{{\"completed\":true,\"verified_proofs_including_warmups\":{count},\"mikros_optimization_pairs_equal\":{},\"fields\":{},\"sizes\":{},\"repetitions\":{},\"threads\":{}}}",if o.systems.contains(&2)&&o.systems.contains(&3) {o.ns.len()*o.fields.len()*o.reps} else {0},o.fields.len(),o.ns.len(),o.reps,o.threads).map_err(|e|e.to_string())?;
    println!("Verified {count} proofs; saved {}", o.out.display());
    Ok(())
}
