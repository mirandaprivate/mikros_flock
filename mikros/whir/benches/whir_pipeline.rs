//! Complete PCS microbenchmark with the same fixture as `fri_pipeline`.
//! Input and claimed-value generation are outside the measured PCS phases.
use flock_field::F128;
use mikros_whir::{Source, verify};
use rayon::prelude::*;
use std::time::Instant;

const SEED: u64 = 97;

fn mix(mut x: u64) -> u64 {
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}

/// An independent GHASH evaluation of the low-variable-first Boolean MLE.
/// This work, including its temporary buffers, finishes before any PCS timer.
fn evaluate_fixture(words: &[F128], point: &[F128]) -> F128 {
    assert_eq!(words.len(), 1usize << point.len());
    let mut layer = words.to_vec();
    for &r in point {
        layer = layer
            .par_chunks_exact(2)
            .map(|pair| pair[0] + (pair[0] + pair[1]) * r)
            .collect();
    }
    layer[0]
}

fn main() {
    let (mut log_words, mut threads, mut reps) = (18usize, 4usize, 1usize);
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--bench" {
            continue;
        }
        let value: usize = args
            .next()
            .expect("argument needs value")
            .parse()
            .expect("integer");
        match arg.as_str() {
            "--log-words" => log_words = value,
            "--threads" => threads = value,
            "--reps" => reps = value,
            _ => panic!("unknown argument {arg}"),
        }
    }
    assert!((1..=30).contains(&log_words) && threads > 0 && reps > 0);
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .unwrap()
        .install(|| {
            // Keep these formulas identical to fri_pipeline, including its
            // second-limb constant and the independent point seed offsets.
            let words = (0..1usize << log_words)
                .into_par_iter()
                .map(|i| F128::new(mix(i as u64 + SEED), mix(i as u64 ^ 0x92d78269c419d325)))
                .collect::<Vec<_>>();
            let point = (0..log_words)
                .map(|i| F128::new(mix(i as u64 + 7907), mix(i as u64 + 32147)))
                .collect::<Vec<_>>();
            let value = evaluate_fixture(&words, &point);
            let start = Instant::now();
            let source = Source::new(words.len(), SEED).unwrap();
            let setup_ms = start.elapsed().as_secs_f64() * 1e3;
            eprintln!("WHIR pipeline: 2^{log_words} F128 words, {threads} workers, {reps} proofs");
            for run in 0..reps {
                eprintln!("WHIR pipeline: run {}/{reps} commit", run + 1);
                let start = Instant::now();
                // Conversion from the original GHASH words to the native tower
                // representation is inside Source::commit and this timer.
                let (cm, cache) = source.commit(&words).unwrap();
                let commit_ms = start.elapsed().as_secs_f64() * 1e3;
                eprintln!("WHIR pipeline: run {}/{reps} open", run + 1);
                let start = Instant::now();
                // Opening consumes the cache, including its source, codeword,
                // and tree; their release completes inside the opening timer.
                let proof = source.open(&cm, &point, value, cache).unwrap();
                let open_ms = start.elapsed().as_secs_f64() * 1e3;
                let start = Instant::now();
                verify(source.verifier_key(), &cm, &point, value, &proof).unwrap();
                let verify_ms = start.elapsed().as_secs_f64() * 1e3;
                println!(
                    "WHIR_SAMPLE {}",
                    serde_json::json!({
                        "log_words":log_words,"threads":threads,"run":run+1,"reps":reps,
                        "seed":SEED,
                        "setup_ms":setup_ms,"commit_ms":commit_ms,"open_ms":open_ms,
                        "verify_ms":verify_ms,"prover_ms":commit_ms+open_ms,
                        "proof_bytes":proof.len(),
                        "proof_blake3":blake3::hash(&proof).to_hex().as_str(),
                        "root":cm.root(),"verified":true,"pcs_complete":true,
                        "profile":mikros_whir::profile(),
                        "parameters":source.verifier_key().parameter_profile(),
                    })
                );
            }
        });
}
