//! Complete PCS microbenchmark. Fixture/evaluation generation is outside commit/open timers.
use binius_field::{PackedField, arch::OptimalPackedB128 as Packed};
use binius_math::{FieldBuffer, multilinear::evaluate::evaluate};
use flock_field::F128;
use mikros_fri::{Source, from_binius, to_binius, verify};
use rayon::prelude::*;
use std::time::Instant;

fn mix(mut x: u64) -> u64 {
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}

fn main() {
    let (mut log_words, mut threads, mut reps) = (24usize, 64usize, 3usize);
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
    rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap().install(|| {
        let words = (0..1usize << log_words).into_par_iter()
            .map(|i| F128::new(mix(i as u64 + 97), mix(i as u64 ^ 0x92d78269c419d325)))
            .collect::<Vec<_>>();
        let point = (0..log_words).map(|i| F128::new(mix(i as u64 + 7907), mix(i as u64 + 32147)))
            .collect::<Vec<_>>();
        let value = {
            let message = FieldBuffer::<Packed>::new(log_words,
                words.par_chunks(Packed::WIDTH)
                    .map(|chunk| Packed::from_scalars(chunk.iter().copied().map(to_binius))).collect());
            from_binius(evaluate(&message, &point.iter().copied().map(to_binius).collect::<Vec<_>>()))
        };
        let start = Instant::now();
        let source = Source::new(words.len(), 97).unwrap();
        let setup_ms = start.elapsed().as_secs_f64()*1e3;
        eprintln!("FRI pipeline: 2^{log_words} F128 words, {threads} workers, {reps} proofs");
        for run in 0..reps {
            eprintln!("FRI pipeline: run {}/{reps} commit",run+1);
            let start = Instant::now();
            let (cm, cache) = source.commit(&words).unwrap();
            let commit_ms = start.elapsed().as_secs_f64()*1e3;
            eprintln!("FRI pipeline: run {}/{reps} open",run+1);
            let start = Instant::now();
            let proof = source.open(&cm, &point, value, cache).unwrap();
            let open_ms = start.elapsed().as_secs_f64()*1e3;
            let start = Instant::now();
            verify(source.verifier_key(), &cm, &point, value, &proof).unwrap();
            let verify_ms = start.elapsed().as_secs_f64()*1e3;
            println!("FRI_SAMPLE {}", serde_json::json!({
                "log_words":log_words,"threads":threads,"run":run+1,"reps":reps,
                "setup_ms":setup_ms,"commit_ms":commit_ms,"open_ms":open_ms,"verify_ms":verify_ms,
                "proof_bytes":proof.len(),"proof_blake3":blake3::hash(&proof).to_hex().as_str(),
                "root":cm.root(),"verified":true,"profile":mikros_fri::profile(),
                "parameters":source.verifier_key().parameter_profile(),
            }));
        }
    });
}
