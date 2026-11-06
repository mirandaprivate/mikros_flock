//! Opt-in engineering measurement for the K32/kappa=4 permutation core.
//! This excludes PLONK terminal projections and PCS; it is not an E2E result.
//!
//! Run the ignored test in release mode with `--nocapture --test-threads=1`.
//! MIKROS_ENGINEERING_LOG_ROWS defaults to 22; MIKROS_ENGINEERING_REPS to 3.
//! One warmup always precedes the measured repetitions, on one Rayon thread.
//! MIKROS_ENGINEERING_INPUT=random (default) uses every row; `copy` uses
//! 3/4 of the domain plus one public-I/O position and a zero-padded suffix.
//! MIKROS_ENGINEERING_SEED defaults to 97. MIKROS_ENGINEERING_PROOF_PATH,
//! when set, receives the first measured proof for byte-for-byte comparisons.
use std::time::Instant;

use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros_permutation::{packed, permutation::CommittedStatement};

#[derive(Clone, Copy)]
enum Input {
    Random,
    Copy,
}

impl Input {
    fn from_env() -> Self {
        match std::env::var("MIKROS_ENGINEERING_INPUT")
            .unwrap_or_else(|_| "random".into())
            .as_str()
        {
            "random" => Self::Random,
            "copy" => Self::Copy,
            value => panic!("MIKROS_ENGINEERING_INPUT must be random or copy, got {value}"),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Random => "random",
            Self::Copy => "copy",
        }
    }

    fn active(self, rows: usize) -> usize {
        match self {
            Self::Random => rows,
            Self::Copy => 3 * rows / 4 + 1,
        }
    }

    fn tag(self, index: usize, seed: u64) -> u128 {
        let random = splitmix64(seed.wrapping_add(index as u64));
        match self {
            // Full-width pseudorandom values, without a zero-padded suffix or
            // low-Hamming-weight restriction. Natural zero values are valid.
            Self::Random => u128::from(random as u32),
            // A position label plus one witness bit, as in binary PLONK copy
            // tags. The configured row bound leaves bit 31 free for the value.
            Self::Copy => (index as u128) | (u128::from(random & 1) << 31),
        }
    }
}

fn splitmix64(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

fn challenger(input: Input, seed: u64) -> FsChallenger {
    let mut ch = FsChallenger::new(b"k32-engineering-same-proof-v1");
    ch.observe_label(input.name().as_bytes());
    ch.observe_bytes(&seed.to_le_bytes());
    ch
}

#[test]
#[ignore = "explicit release-mode K32 performance comparison"]
fn k32_permutation_engineering() {
    let log_rows: usize = std::env::var("MIKROS_ENGINEERING_LOG_ROWS")
        .unwrap_or_else(|_| "22".into())
        .parse()
        .expect("MIKROS_ENGINEERING_LOG_ROWS must be an integer");
    let reps: usize = std::env::var("MIKROS_ENGINEERING_REPS")
        .unwrap_or_else(|_| "3".into())
        .parse()
        .expect("MIKROS_ENGINEERING_REPS must be an integer");
    let seed: u64 = std::env::var("MIKROS_ENGINEERING_SEED")
        .unwrap_or_else(|_| "97".into())
        .parse()
        .expect("MIKROS_ENGINEERING_SEED must be a u64");
    assert!((10..=26).contains(&log_rows) && (1..=20).contains(&reps));
    let input = Input::from_env();
    let rows = 1usize << log_rows;
    let active = input.active(rows);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .unwrap();
    pool.install(|| {
        let mut expected = None;
        for run in 0..=reps {
            let statement = CommittedStatement::new(rows, 32).unwrap();
            let start = Instant::now();
            let prepared = packed::prepare_bounded_from_fn(statement, |i| {
                if i < active {
                    // Rotating the live source preserves its entire multiset,
                    // including any repeated or zero pseudorandom tags.
                    [input.tag(i, seed), input.tag((i + 17) % active, seed)]
                } else {
                    [0, 0]
                }
            })
            .unwrap();
            let prepare_ms = start.elapsed().as_secs_f64() * 1000.0;
            let resident = prepared.resident_bytes();
            let mut ch = challenger(input, seed);
            let start = Instant::now();
            let (proof, claims, timing) =
                packed::prove_bounded_compact(prepared, &mut ch).unwrap();
            let prove_ms = start.elapsed().as_secs_f64() * 1000.0;
            let start = Instant::now();
            let bytes = bincode::serialize(&proof).unwrap();
            let serialization_ms = start.elapsed().as_secs_f64() * 1000.0;
            let claims_bytes = bincode::serialize(&claims).unwrap();
            let proof_hash = blake3::hash(&bytes);
            let claims_hash = blake3::hash(&claims_bytes);
            let next = ch.sample_f128();
            let mut verifier = challenger(input, seed);
            let start = Instant::now();
            assert_eq!(
                packed::verify(&statement, &proof, &mut verifier).unwrap(),
                claims
            );
            assert_eq!(verifier.sample_f128(), next);
            let verify_ms = start.elapsed().as_secs_f64() * 1000.0;
            let identity = (proof_hash, claims_hash, next);
            if let Some(expected) = &expected {
                assert_eq!(&identity, expected, "proof changed between repetitions");
            } else {
                expected = Some(identity);
            }
            if run == 1 {
                if let Ok(path) = std::env::var("MIKROS_ENGINEERING_PROOF_PATH") {
                    std::fs::write(path, &bytes).unwrap();
                }
            }
            println!(
                "K32_ENGINEERING run={run} warmup={} input={} seed={seed} log_rows={log_rows} rows={rows} active_rows={active} tag_bits=32 kappa=4 threads=1 warmups=1 reps={reps} prepare_ms={prepare_ms:.6} prove_ms={prove_ms:.6} serialization_ms={serialization_ms:.6} verify_ms={verify_ms:.6} prepared_bytes={resident} proof_bytes={} proof_hash={proof_hash} claims_hash={claims_hash} next={next:?} verified=true transcript_match=true phases={timing:?}",
                run == 0,
                input.name(),
                bytes.len(),
            );
        }
    });
}
