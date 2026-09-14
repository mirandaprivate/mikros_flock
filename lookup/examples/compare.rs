//! cargo run --release --example compare -- <log_n> <value_bits> <table_rows> <repeats>
//! Warmed PIOP comparison; input generation is outside timing, witness creation
//! and source hashing inside. Authentication by the linear reference oracle is
//! measured separately and is not a PCS time.
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros_lookup::{
    bits::{PackedBits, bits_for},
    claims::Source,
    lasso,
    permutation::{self, Options, Strategy},
};
use serde_json::{Value, json};
use std::{hint::black_box, time::Instant};

fn transcript() -> FsChallenger {
    FsChallenger::with_hash(b"lookup-comparison-v1", HashKind::Blake3)
}
fn median(values: &[f64]) -> f64 {
    let mut v = values.to_vec();
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}
fn options(strategy: Strategy) -> Options {
    Options {
        strategy,
        ..Default::default()
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args()
        .skip(1)
        .map(|x| x.parse::<usize>())
        .collect::<Result<_, _>>()?;
    if args.len() > 4 {
        return Err("expected log_n value_bits table_rows repeats".into());
    }
    let log_n = args.first().copied().unwrap_or(10);
    let width = args.get(1).copied().unwrap_or(8);
    let m = args.get(2).copied().unwrap_or(64);
    let repeats = args.get(3).copied().unwrap_or(5);
    if log_n > 20 || width > 128 || m == 0 || m > (1 << 20) || repeats == 0 {
        return Err("invalid benchmark dimensions".into());
    }
    let n = 1usize << log_n;
    let mask = if width == 128 {
        u128::MAX
    } else {
        (1u128 << width) - 1
    };
    let table_rows: Vec<_> = (0..m)
        .map(|j| ((j as u128).wrapping_mul(0x198abc8976123) ^ 0x8719845) & mask)
        .collect();
    let addresses: Vec<_> = (0..n).map(|i| ((i * 17 + i / 7) % m) as u128).collect();
    let values: Vec<_> = addresses.iter().map(|&a| table_rows[a as usize]).collect();
    let left = PackedBits::from_rows(&values, width)?;
    let right = PackedBits::from_rows(&values.iter().copied().rev().collect::<Vec<_>>(), width)?;
    let a = PackedBits::from_rows(&addresses, bits_for(m - 1))?;
    let permutation_statement = permutation::Statement::new(&left, &right)?;
    let lasso_statement =
        lasso::Statement::new(PackedBits::from_rows(&table_rows, width)?, &left, &a)?;
    let mut rows = Vec::<Value>::new();
    let mut permutation_reference = None;
    let mut lasso_reference = None;
    // Run strategies in alternating order across samples to reduce order bias.
    let mut samples: [Vec<(f64, f64, f64, Value)>; 4] = std::array::from_fn(|_| Vec::new());
    for iteration in 0..=repeats {
        let order = if iteration % 2 == 0 {
            [Strategy::FullExtension, Strategy::BitwidthAware]
        } else {
            [Strategy::BitwidthAware, Strategy::FullExtension]
        };
        for strategy in order {
            let index = if strategy == Strategy::FullExtension {
                0
            } else {
                1
            };
            let mut p = transcript();
            let start = Instant::now();
            let (proof, claims, stats) = permutation::prove(
                &permutation_statement,
                &left,
                &right,
                options(strategy),
                &mut p,
            )?;
            let prove_ms = start.elapsed().as_secs_f64() * 1000.;
            let mut v = transcript();
            let start = Instant::now();
            let verified = permutation::verify(&permutation_statement, &proof, &mut v)?;
            let verify_ms = start.elapsed().as_secs_f64() * 1000.;
            assert_eq!(claims, verified);
            assert_eq!(p.sample_f128(), v.sample_f128());
            let start = Instant::now();
            verified.check(&[(Source::Left, &left), (Source::Right, &right)])?;
            let check_ms = start.elapsed().as_secs_f64() * 1000.;
            if let Some(reference) = &permutation_reference {
                assert_eq!(&proof, reference);
            } else {
                permutation_reference = Some(proof.clone());
            }
            if iteration != 0 {
                samples[index].push((
                    prove_ms,
                    verify_ms,
                    check_ms,
                    json!({
                        "tree_stats":stats,"field_message_bytes":proof.field_elements()*16,
                        "bincode_bytes":bincode::serialize(&proof)?.len()
                    }),
                ));
            }
            black_box(proof);

            let mut p = transcript();
            let start = Instant::now();
            let out = lasso::prove(&lasso_statement, &left, &a, options(strategy), &mut p)?;
            let prove_ms = start.elapsed().as_secs_f64() * 1000.;
            let mut v = transcript();
            let start = Instant::now();
            let verified = lasso::verify(&lasso_statement, &out.proof, &mut v)?;
            let verify_ms = start.elapsed().as_secs_f64() * 1000.;
            assert_eq!(out.claims, verified);
            assert_eq!(p.sample_f128(), v.sample_f128());
            let start = Instant::now();
            verified.check(&out.auxiliary.sources(&left, &a))?;
            let check_ms = start.elapsed().as_secs_f64() * 1000.;
            if let Some(reference) = &lasso_reference {
                assert_eq!(&out.proof, reference);
            } else {
                lasso_reference = Some(out.proof.clone());
            }
            if iteration != 0 {
                samples[index+2].push((prove_ms,verify_ms,check_ms,json!({
                "stats":out.stats,"field_message_bytes":out.proof.field_elements()*16,
                "auxiliary_digest_bytes":96,"bincode_bytes":bincode::serialize(&out.proof)?.len()
            })));
            }
            black_box(out);
        }
    }
    for (index, samples) in samples.into_iter().enumerate() {
        rows.push(json!({
            "protocol":if index<2 {"permutation"} else {"lasso"},
            "strategy":if index%2==0 {"full_extension"} else {"bitwidth_aware"},
            "prove_median_ms":median(&samples.iter().map(|s|s.0).collect::<Vec<_>>()),
            "verify_median_ms":median(&samples.iter().map(|s|s.1).collect::<Vec<_>>()),
            "direct_oracle_median_ms":median(&samples.iter().map(|s|s.2).collect::<Vec<_>>()),
            "detail":samples[0].3,
            "samples":samples.iter().map(|s|json!({"prove_ms":s.0,"verify_ms":s.1,"direct_oracle_ms":s.2})).collect::<Vec<_>>()
        }));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "scope":"PIOP only; no PCS, no ZK, no grinding; direct oracle timing separate",
            "log_n":log_n,"n":n,"value_bits":width,"table_rows":m,"repeats":repeats,
            "rayon_threads":rayon::current_num_threads(),"proofs_identical":true,"results":rows
        }))?
    );
    Ok(())
}
