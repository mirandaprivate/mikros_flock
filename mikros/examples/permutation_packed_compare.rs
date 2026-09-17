//! Matched old/new bit-address permutation PIOPs. Offline preparation is
//! reported separately; all challenge-dependent source conversion is online.
//! Direct source checking is timed separately. The E2E comparison composes the
//! reduction with PCS: e2e_bit_compare --permutation both.
use flock_core::challenger::{Challenger, FsChallenger};
use flock_field::{F128, F256Unreduced};
use mikros_permutation::{permutation::Options, profile::Engine, wiring as mw};
use rayon::prelude::*;
use serde_json::json;
use std::{
    env,
    io::{self, Write},
    time::Instant,
};

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}
fn mix(mut x: u64) -> u64 {
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}
fn challenger(digest: &[u8; 32], variant: &str) -> FsChallenger {
    let mut c = FsChallenger::new(b"mikros-packed-permutation-benchmark-v1");
    c.observe_bytes(digest);
    c.observe_label(variant.as_bytes());
    c
}
fn mle_tensor(c: mw::BitMleClaim) -> mw::BitTensorClaim {
    let mut columns = vec![F128::ONE; 128];
    for (j, weight) in columns.iter_mut().enumerate() {
        for (k, &r) in c.point[..7].iter().enumerate() {
            *weight *= if (j >> k) & 1 == 0 { F128::ONE + r } else { r };
        }
    }
    mw::BitTensorClaim {
        column_weights: columns,
        row_factors: c.point[7..].iter().map(|&r| [F128::ONE + r, r]).collect(),
        value: c.value,
    }
}

// Independent evaluation against ORIGINAL bits, not tagged tuples, tree data,
// or the prover's contraction routine. Byte tables merely collect set bits.
fn original_source_value(words: &[F128], claim: &mw::BitTensorClaim) -> F128 {
    assert_eq!(claim.column_weights.len(), 128);
    assert_eq!(words.len(), 1usize << claim.row_factors.len());
    let mut tables = vec![[F128::ZERO; 256]; 16];
    for (byte, table) in tables.iter_mut().enumerate() {
        for bits in 1usize..256 {
            let bit = bits.trailing_zeros() as usize;
            table[bits] = table[bits & (bits - 1)] + claim.column_weights[8 * byte + bit];
        }
    }
    // A distinct, direct product of row factors per word avoids relying on a
    // shared eq-builder or on the new protocol's public-tag evaluator.
    words
        .par_chunks(1024)
        .enumerate()
        .map(|(chunk, words)| {
            let mut sum = F256Unreduced::ZERO;
            for (offset, word) in words.iter().enumerate() {
                let row = chunk * 1024 + offset;
                let weight = claim
                    .row_factors
                    .iter()
                    .enumerate()
                    .fold(F128::ONE, |a, (bit, pair)| a * pair[(row >> bit) & 1]);
                let mut bits = ((word.hi as u128) << 64) | word.lo as u128;
                let mut value = F128::ZERO;
                for table in &tables {
                    value += table[(bits & 255) as usize];
                    bits >>= 8;
                }
                sum ^= value.mul_unreduced(weight);
            }
            sum.reduce()
        })
        .reduce(|| F128::ZERO, |a, b| a + b)
}

fn features() -> Vec<&'static str> {
    let mut out = Vec::new();
    for (name, enabled) in [
        ("avx512f", cfg!(target_feature = "avx512f")),
        ("avx512bw", cfg!(target_feature = "avx512bw")),
        ("gfni", cfg!(target_feature = "gfni")),
        ("vpclmulqdq", cfg!(target_feature = "vpclmulqdq")),
    ] {
        if enabled {
            out.push(name);
        }
    }
    out
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut log_n, mut threads, mut reps, mut warmups) = (14usize, 4usize, 2usize, 1usize);
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        let value = args
            .next()
            .ok_or("missing argument value")?
            .parse::<usize>()?;
        match arg.as_str() {
            "--log-n" => log_n = value,
            "--threads" => threads = value,
            "--reps" => reps = value,
            "--warmups" => warmups = value,
            _ => return Err(format!("unknown argument {arg}").into()),
        }
    }
    if !(8..=30).contains(&log_n) || threads == 0 || reps == 0 {
        return Err("invalid dimensions".into());
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .stack_size(8 << 20)
        .build_global()?;
    assert!(flock_parallel::init_all_core_pool(threads));
    let start = Instant::now();
    let n = 1usize << log_n;
    // Involution moves bits between both words and bit coordinates. Equal bits
    // on each two-cycle make the copy constraint true without all-zero input.
    let words: Vec<F128> = (0..n / 128)
        .into_par_iter()
        .map(|word| {
            let mut value = 0u128;
            for bit in 0..128 {
                let i = word * 128 + bit;
                value |= ((mix(i.min(i ^ 131) as u64) & 1) as u128) << bit;
            }
            F128 {
                lo: value as u64,
                hi: (value >> 64) as u64,
            }
        })
        .collect();
    let mut hash = blake3::Hasher::new();
    for word in &words {
        hash.update(&word.lo.to_le_bytes());
        hash.update(&word.hi.to_le_bytes());
    }
    let digest = *hash.finalize().as_bytes();
    let sigma: Vec<usize> = (0..n).into_par_iter().map(|i| i ^ 131).collect();
    let statement = mw::BitWiringStatement::new(&sigma)?;
    drop(sigma);
    let fixture_ms = ms(start);
    let start = Instant::now();
    let source = mw::prepare_bits(&words, &statement)?;
    let old = mw::prepare_bit_coefficients(&source, Options::default())?;
    drop(source);
    let legacy_prepare_ms = ms(start);
    let start = Instant::now();
    let new = mw::prepare_bits_packed(&words, &statement)?;
    let packed_prepare_ms = ms(start);
    println!(
        "{}",
        json!({"record":"metadata", "schema_version":1,
        "log_n":log_n, "permutation_cells":n, "tuple_width":statement.shape().width,
        "packed_kappa":new.kappa(), "threads":rayon::current_num_threads(),
        "all_core_threads":flock_parallel::all_core_pool().current_num_threads(),
        "target_features":features(), "reps":reps, "warmups":warmups,
        "fixture":"bit-copy-xor131-random-two-cycles", "fixture_ms":fixture_ms,
        "witness_blake3":blake3::Hash::from_bytes(digest).to_hex().as_str(),
        "legacy_prepare_ms":legacy_prepare_ms,"packed_prepare_ms":packed_prepare_ms,
        "legacy_prepared_bytes":old.resident_bytes(),"packed_prepared_bytes":new.resident_bytes(),
        "shared_public_bytes":statement.bits() * (std::mem::size_of::<usize>() + 16),
        "timing_boundary":"online proof and original-source query conversion; excludes offline preparation, verification, source oracle, serialization; standalone PIOP, no PCS or grinding",
        "order":"alternate legacy/packed each iteration; fixed transcript per variant"})
    );
    io::stdout().flush()?;
    for iteration in 0..warmups + reps {
        let variants = if iteration % 2 == 0 {
            ["legacy", "packed"]
        } else {
            ["packed", "legacy"]
        };
        for (slot, variant) in variants.into_iter().enumerate() {
            let mut p = challenger(&digest, variant);
            let mut v = challenger(&digest, variant);
            let start = Instant::now();
            let (claim, verified_claim, proof_bytes, timings, prover_ms, verify_ms, logical_fields) =
                if variant == "legacy" {
                    let (proof, claim, _, timings) = mw::prove_bits_prepared_lockstep(
                        &statement,
                        &old,
                        Engine::Combined,
                        &mut p,
                    )?;
                    let claim = mle_tensor(claim);
                    let prover_ms = ms(start);
                    let verify_start = Instant::now();
                    let verified =
                        mle_tensor(mw::verify_bits_lockstep(&statement, &proof, &mut v)?);
                    let verify_ms = ms(verify_start);
                    let fields = 2 + [&proof.left, &proof.right]
                        .iter()
                        .flat_map(|p| &p.layers)
                        .map(|layer| 3 * layer.rounds.len() + 2)
                        .sum::<usize>();
                    (
                        claim,
                        verified,
                        bincode::serialize(&proof)?,
                        timings,
                        prover_ms,
                        verify_ms,
                        fields,
                    )
                } else {
                    let (proof, claim, timings) =
                        mw::prove_bits_prepared_packed(&statement, &new, &mut p)?;
                    let prover_ms = ms(start);
                    let verify_start = Instant::now();
                    let verified = mw::verify_bits_packed(&statement, &proof, &mut v)?;
                    let verify_ms = ms(verify_start);
                    let fields = proof.field_elements();
                    (
                        claim,
                        verified,
                        bincode::serialize(&proof)?,
                        timings,
                        prover_ms,
                        verify_ms,
                        fields,
                    )
                };
            assert_eq!(
                claim, verified_claim,
                "prover/verifier source claim mismatch"
            );
            assert_eq!(p.sample_f128(), v.sample_f128(), "transcript mismatch");
            let source_start = Instant::now();
            assert_eq!(
                original_source_value(&words, &claim),
                claim.value,
                "original bit source mismatch"
            );
            let source_check_ms = ms(source_start);
            println!(
                "{}",
                json!({"record":"sample","variant":variant,"iteration":iteration + 1 - iteration.min(warmups),
                "warmup":iteration < warmups,"slot":slot,"prover_ms":prover_ms,"verify_ms":verify_ms,
                "source_check_ms":source_check_ms,"proof_bytes":proof_bytes.len(),"logical_field_elements":logical_fields,
                "proof_blake3":blake3::hash(&proof_bytes).to_hex().as_str(),"timings":timings,
                "verified":true,"transcript_match":true,"original_source_checked":true})
            );
            io::stdout().flush()?;
        }
    }
    println!(
        "{}",
        json!({"record":"pool_stats",
        "legacy":mikros_permutation::tree_pool::stats(1),
        "packed":mikros_permutation::tree_pool::stats(2)})
    );
    let start = Instant::now();
    mikros_permutation::tree_pool::clear_all();
    flock_core::scratch::clear();
    println!(
        "{}",
        json!({"record":"batch_teardown", "scratch_release_ms":ms(start)})
    );
    Ok(())
}
