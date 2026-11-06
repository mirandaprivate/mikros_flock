//! Deterministic complete-PCS comparison for engineering optimizations.
//!
//! Run the same source against both backend revisions. Compare parameter/input/
//! point digests, commitment root, value, proof length and full-proof digest.
//! `RAAFOLD_PROFILE_STAGES` is deliberately inherited. No warmup is performed;
//! each repetition recommits and opens the same dense full-field witness.

use std::{env, error::Error, fmt::Write, io::Write as _, time::Instant};

use ark_std::rand::{RngCore, SeedableRng, rngs::StdRng};
use binius_field::Field;
use blake2::Blake2s256;
use digest::Digest;
use mikros_raafold::{
    raafold::BinaryExtensionField as F,
    raafold_short_pc::{ShortRaaFoldPC, ShortRaaFoldProof},
};

const COLUMNS: usize = 128;
const SWITCH_RATIO: usize = 8;
const SECURITY_BITS: usize = 100;
const DIRECT_CHECK_MAX_LOG: usize = 18;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Options {
    log_words: usize,
    reps: usize,
    threads: usize,
    seed: u64,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            log_words: 18,
            reps: 1,
            threads: 64,
            seed: 97,
        }
    }
}

fn parse_args(args: impl IntoIterator<Item = String>) -> Result<Option<Options>, String> {
    let mut options = Options::default();
    let mut args = args.into_iter();
    while let Some(flag) = args.next() {
        if flag == "--help" || flag == "-h" {
            println!(
                "opt_compare [--log-words N] [--reps R] [--threads T] [--seed S]\n\
                 Defaults: N=18, R=1, T=64, S=97. columns=128, switch=8, security=100.\n\
                 Complete Opening+Decide; dense full F128 input; no excluded warmup.\n\
                 RAAFOLD_PROFILE_STAGES is inherited (unset it for formal timing)."
            );
            return Ok(None);
        }
        let value = args
            .next()
            .ok_or_else(|| format!("missing value for {flag}"))?;
        match flag.as_str() {
            "--log-words" => options.log_words = value.parse().map_err(|_| "invalid log-words")?,
            "--reps" => options.reps = value.parse().map_err(|_| "invalid reps")?,
            "--threads" => options.threads = value.parse().map_err(|_| "invalid threads")?,
            "--seed" => options.seed = value.parse().map_err(|_| "invalid seed")?,
            _ => return Err(format!("unknown option {flag}")),
        }
    }
    if options.log_words < 10 || options.log_words >= usize::BITS as usize {
        return Err("log-words must be at least 10 and fit usize".into());
    }
    let words = 1usize << options.log_words;
    if words.checked_mul(128).is_none() {
        return Err("logical witness bit count overflows usize".into());
    }
    if options.reps == 0 || options.threads == 0 {
        return Err("reps and threads must be positive".into());
    }
    #[cfg(not(feature = "parallel"))]
    if options.threads != 1 {
        return Err("a build without the parallel feature requires --threads 1".into());
    }
    Ok(Some(options))
}

fn random_word(rng: &mut impl RngCore) -> u128 {
    rng.next_u64() as u128 | ((rng.next_u64() as u128) << 64)
}

fn hex(bytes: &[u8]) -> String {
    let mut text = String::with_capacity(2 * bytes.len());
    for byte in bytes {
        write!(&mut text, "{byte:02x}").expect("writing into a String cannot fail");
    }
    text
}

/// Independent adjacent-pair folding: the API's point is MSB-first.
fn direct_mle(values: &[u128], point: &[F]) -> F {
    assert_eq!(values.len(), 1usize << point.len());
    let mut folded: Vec<F> = values.iter().copied().map(F::from).collect();
    for &r in point.iter().rev() {
        let next_len = folded.len() / 2;
        for i in 0..next_len {
            let a = folded[2 * i];
            let b = folded[2 * i + 1];
            folded[i] = a + (a + b) * r;
        }
        folded.truncate(next_len);
    }
    folded[0]
}

fn median(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    if sorted.len() % 2 == 0 {
        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
    } else {
        sorted[sorted.len() / 2]
    }
}

fn run(options: Options) -> Result<(), Box<dyn Error>> {
    #[cfg(feature = "parallel")]
    rayon::ThreadPoolBuilder::new()
        .num_threads(options.threads)
        .build_global()?;

    let words = 1usize << options.log_words;
    let pc = ShortRaaFoldPC::new(words, COLUMNS, 128, SWITCH_RATIO, SECURITY_BITS)?;
    eprintln!("[opt_compare] setup start: 2^{} words", options.log_words);
    let setup_start = Instant::now();
    let mut setup_rng = StdRng::seed_from_u64(options.seed);
    let (params, setup) = pc.setup(&mut setup_rng)?;
    let setup_ms = setup_start.elapsed().as_secs_f64() * 1000.0;

    // Separate deterministic streams keep the fixture independent of how many
    // random words setup consumes. These are raw POLYVAL field coordinates.
    let fixture_start = Instant::now();
    let mut input_rng = StdRng::seed_from_u64(options.seed ^ 0x696e_7075_742f_7631);
    let mut input_hasher = Blake2s256::new();
    let values: Vec<u128> = (0..words)
        .map(|_| {
            let value = random_word(&mut input_rng);
            input_hasher.update(value.to_le_bytes());
            value
        })
        .collect();
    let input_digest = hex(&input_hasher.finalize());
    let mut point_rng = StdRng::seed_from_u64(options.seed ^ 0x706f_696e_742f_7631);
    let point: Vec<F> = (0..options.log_words)
        .map(|_| {
            loop {
                let x = F::from(random_word(&mut point_rng));
                if x != F::ZERO && x != F::ONE {
                    break x;
                }
            }
        })
        .collect();
    let mut point_hasher = Blake2s256::new();
    for &x in &point {
        point_hasher.update(x.val().to_le_bytes());
    }
    let point_digest = hex(&point_hasher.finalize());
    let fixture_ms = fixture_start.elapsed().as_secs_f64() * 1000.0;
    let oracle_start = Instant::now();
    let expected = (options.log_words <= DIRECT_CHECK_MAX_LOG).then(|| direct_mle(&values, &point));
    let source_check_ms = oracle_start.elapsed().as_secs_f64() * 1000.0;
    let source_check = if expected.is_some() {
        "passed"
    } else {
        "not_run_large"
    };
    let profile_enabled = env::var_os("RAAFOLD_PROFILE_STAGES").is_some();
    println!(
        "{{\"record\":\"metadata\",\"schema\":\"raafold-opt-compare-v1\",\"log_words\":{},\"words\":{},\"columns\":{},\"logical_element_bits\":128,\"switch_ratio\":{},\"query_security_bits\":{},\"repetition\":4,\"accumulators\":2,\"distance\":0.19,\"distance_certified\":false,\"field\":\"GF(2^128) POLYVAL dot coordinates\",\"point_basis\":\"equality MSB first\",\"seed\":{},\"threads\":{},\"reps\":{},\"warmups\":0,\"profile_stages\":{},\"setup_ms\":{:.6},\"fixture_ms\":{:.6},\"source_check_ms\":{:.6},\"source_check\":\"{}\",\"parameter_digest\":\"{}\",\"input_blake2s256\":\"{}\",\"point_blake2s256\":\"{}\"}}",
        options.log_words,
        words,
        COLUMNS,
        SWITCH_RATIO,
        SECURITY_BITS,
        options.seed,
        options.threads,
        options.reps,
        profile_enabled,
        setup_ms,
        fixture_ms,
        source_check_ms,
        source_check,
        hex(&params.parameter_digest()),
        input_digest,
        point_digest,
    );
    std::io::stdout().flush()?;

    let mut baseline = None;
    let mut commits = Vec::with_capacity(options.reps);
    let mut opens = Vec::with_capacity(options.reps);
    let mut verifies = Vec::with_capacity(options.reps);
    for run in 1..=options.reps {
        eprintln!("[opt_compare] sample {run}/{} commit", options.reps);
        let start = Instant::now();
        let (commitment, cache) = pc.commit_values(&params, &values)?;
        let commit_ms = start.elapsed().as_secs_f64() * 1000.0;

        eprintln!(
            "[opt_compare] sample {run}/{} complete_opening_and_decide",
            options.reps
        );
        let start = Instant::now();
        let (value, proof) = pc.open(&params, &setup, &commitment, &point, cache)?;
        let open_ms = start.elapsed().as_secs_f64() * 1000.0;
        if let Some(expected) = expected {
            if value != expected {
                return Err("complete PCS opening differs from the independent source MLE".into());
            }
        }

        let start = Instant::now();
        let encoded = proof.to_bytes()?;
        let serialize_ms = start.elapsed().as_secs_f64() * 1000.0;
        let proof_digest = hex(&Blake2s256::digest(&encoded));
        let start = Instant::now();
        let decoded = ShortRaaFoldProof::from_bytes(&encoded)?;
        let deserialize_ms = start.elapsed().as_secs_f64() * 1000.0;
        if decoded != proof {
            return Err("complete PCS proof did not round-trip through its serializer".into());
        }
        drop(proof);

        eprintln!("[opt_compare] sample {run}/{} verify", options.reps);
        let start = Instant::now();
        let verified = pc.verify(&params, &commitment, &point, value, &decoded)?;
        let verify_ms = start.elapsed().as_secs_f64() * 1000.0;
        if !verified {
            return Err("complete Opening+Decide verification rejected".into());
        }
        let signature = (
            commitment.root(),
            value,
            encoded.len(),
            proof_digest.clone(),
        );
        if let Some(ref first) = baseline {
            if first != &signature {
                return Err(
                    "identical fixture yielded a different root/value/proof digest or length"
                        .into(),
                );
            }
        } else {
            baseline = Some(signature);
        }
        commits.push(commit_ms);
        opens.push(open_ms);
        verifies.push(verify_ms);
        println!(
            "{{\"record\":\"sample\",\"run\":{run},\"commit_ms\":{commit_ms:.6},\"open_ms\":{open_ms:.6},\"serialize_ms\":{serialize_ms:.6},\"deserialize_ms\":{deserialize_ms:.6},\"verify_ms\":{verify_ms:.6},\"proof_bytes\":{},\"proof_blake2s256\":\"{}\",\"commitment_root\":\"{}\",\"value_le_hex\":\"{}\",\"verified\":true,\"complete_opening_and_decide\":true,\"source_check\":\"{}\",\"same_fixture_signature\":true}}",
            encoded.len(),
            proof_digest,
            hex(&commitment.root()),
            hex(&value.val().to_le_bytes()),
            source_check,
        );
        std::io::stdout().flush()?;
        eprintln!(
            "[opt_compare] sample {run}/{} complete: commit={commit_ms:.3} ms open={open_ms:.3} ms verify={verify_ms:.3} ms proof={} B",
            options.reps,
            encoded.len(),
        );
    }
    println!(
        "{{\"record\":\"summary\",\"samples\":{},\"commit_ms_median\":{:.6},\"open_ms_median\":{:.6},\"verify_ms_median\":{:.6},\"all_verified\":true,\"identical_fixture_signature\":true}}",
        options.reps,
        median(&commits),
        median(&opens),
        median(&verifies),
    );
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    if let Some(options) = parse_args(env::args().skip(1))? {
        run(options)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_oracle_uses_msb_first_equality_coordinates() {
        let values = [0x123u128, 0x456, 1 << 127, (1 << 127) | 0x789];
        for (index, &expected) in values.iter().enumerate() {
            let point = [
                if index & 2 == 0 { F::ZERO } else { F::ONE },
                if index & 1 == 0 { F::ZERO } else { F::ONE },
            ];
            assert_eq!(direct_mle(&values, &point), F::from(expected));
        }
        let point = [F::from(123), F::from(1 << 126)];
        let mut expected = F::ZERO;
        for (index, &value) in values.iter().enumerate() {
            let mut weight = F::ONE;
            for (variable, &r) in point.iter().enumerate() {
                weight *= if index & (1 << (1 - variable)) == 0 {
                    F::ONE + r
                } else {
                    r
                };
            }
            expected += F::from(value) * weight;
        }
        assert_eq!(direct_mle(&values, &point), expected);
    }

    #[test]
    fn strict_options_reject_invalid_shapes_and_counts() {
        let parse = |args: &[&str]| parse_args(args.iter().map(|x| x.to_string()));
        assert!(parse(&["--log-words", "9"]).is_err());
        assert!(parse(&["--reps", "0"]).is_err());
        assert!(parse(&["--threads", "0"]).is_err());
        assert!(parse(&["--unexpected", "1"]).is_err());
        assert!(parse(&["--seed"]).is_err());
        let options = parse(&[
            "--log-words",
            "16",
            "--reps",
            "3",
            "--threads",
            "1",
            "--seed",
            "91",
        ])
        .unwrap()
        .unwrap();
        assert_eq!(
            options,
            Options {
                log_words: 16,
                reps: 3,
                threads: 1,
                seed: 91
            }
        );
    }
}
