//! Opt-in engineering measurement for the K64/kappa=2 permutation core.
//! This excludes PLONK terminal projections and PCS; it is not an E2E result.
use std::time::Instant;

use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros_permutation::{packed, permutation::CommittedStatement};

fn tag(index: usize) -> u128 {
    let label = (index as u32).wrapping_mul(0x9e37_79b1);
    let bit = ((index as u64).wrapping_mul(0xd6e8_feb8_6659_fd93) >> 37) & 1;
    u128::from(label) | (u128::from(bit) << 63)
}

#[test]
#[ignore = "explicit release-mode K64 performance comparison"]
fn k64_permutation_engineering() {
    let log_rows: usize = std::env::var("MIKROS_ENGINEERING_LOG_ROWS")
        .unwrap_or_else(|_| "22".into())
        .parse()
        .unwrap();
    let reps: usize = std::env::var("MIKROS_ENGINEERING_REPS")
        .unwrap_or_else(|_| "5".into())
        .parse()
        .unwrap();
    assert!((10..=26).contains(&log_rows) && (1..=20).contains(&reps));
    let rows = 1usize << log_rows;
    let active = 3 * rows / 4;
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap();
    pool.install(|| {
        for run in 0..=reps {
            let statement = CommittedStatement::new(rows, 64).unwrap();
            let start = Instant::now();
            let prepared = packed::prepare_bounded_from_fn(statement, |i| {
                if i < active { [tag(i), tag((i + 17) % active)] } else { [0, 0] }
            }).unwrap();
            let prepare_ms = start.elapsed().as_secs_f64() * 1000.0;
            let resident = prepared.resident_bytes();
            let mut ch = FsChallenger::new(b"k64-engineering-same-proof-v1");
            let start = Instant::now();
            let (proof, claims, timing) = packed::prove_bounded_compact(prepared, &mut ch).unwrap();
            let prove_ms = start.elapsed().as_secs_f64() * 1000.0;
            let bytes = bincode::serialize(&proof).unwrap();
            let claims_bytes = bincode::serialize(&claims).unwrap();
            let next = ch.sample_f128();
            let mut verifier = FsChallenger::new(b"k64-engineering-same-proof-v1");
            assert_eq!(packed::verify(&statement, &proof, &mut verifier).unwrap(), claims);
            assert_eq!(verifier.sample_f128(), next);
            if run == 1 {
                if let Ok(path) = std::env::var("MIKROS_ENGINEERING_PROOF_PATH") {
                    std::fs::write(path, &bytes).unwrap();
                }
            }
            println!("K64_ENGINEERING run={run} warmup={} log_rows={log_rows} threads=1 prepare_ms={prepare_ms:.6} prove_ms={prove_ms:.6} prepared_bytes={resident} proof_bytes={} proof_hash={} claims_hash={} next={next:?} phases={timing:?}",
                run == 0, bytes.len(), blake3::hash(&bytes), blake3::hash(&claims_bytes));
        }
    });
}
