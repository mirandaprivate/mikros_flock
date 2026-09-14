use flock_core::{product_gkr, zerocheck::univariate_skip::SplitEqGhash};
use flock_field::{F8, F128, F256Unreduced, phi8};
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros_permutation_optimized::{
    bits::PackedBits,
    claims::Source,
    permutation::{self, Options, Strategy},
    profile::Engine,
};
use rayon::prelude::*;
use serde_json::{Value, json};
use std::{
    fs,
    io::Write,
    path::PathBuf,
    time::{Instant, SystemTime, UNIX_EPOCH},
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}
fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e3779b97f4a7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}
fn permute(i: usize, log_n: usize, seed: u64) -> usize {
    let half = log_n / 2;
    let mask = (1usize << half) - 1;
    let mut l = i >> half;
    let mut r = i & mask;
    for round in 0..4 {
        let next = l ^ ((mix(r as u64 ^ mix(seed + round)) as usize) & mask);
        l = r;
        r = next;
    }
    (l << half) | r
}
struct Input {
    log_n: usize,
    width: usize,
    f: Vec<u8>,
    g: Vec<u8>,
    sigma: Vec<usize>,
    digest: [u8; 32],
}
impl Input {
    fn new(log_n: usize, width: usize, seed: u64) -> Self {
        assert!(log_n.is_multiple_of(2) && (2..=30).contains(&log_n) && (1..=8).contains(&width));
        let n = 1usize << log_n;
        let mask = ((1usize << width) - 1) as u8;
        let sigma: Vec<_> = (0..n)
            .into_par_iter()
            .map(|i| permute(i, log_n, seed))
            .collect();
        let f: Vec<_> = (0..n)
            .into_par_iter()
            .map(|i| (mix(i as u64 ^ mix(seed)) as u8) & mask)
            .collect();
        let g: Vec<_> = sigma.par_iter().map(|&i| f[i]).collect();
        let mut h = blake3::Hasher::new();
        h.update(b"permutation-optimized-input-v1");
        h.update(&(log_n as u64).to_le_bytes());
        h.update(&(width as u64).to_le_bytes());
        h.update_rayon(&f);
        h.update_rayon(&g);
        assert!(cfg!(target_endian = "little") && std::mem::size_of::<usize>() == 8);
        // SAFETY: usize is initialized, contiguous, and exactly 8 little-endian
        // bytes here; hashing bytes does not mutate or outlive the vector.
        h.update_rayon(unsafe { std::slice::from_raw_parts(sigma.as_ptr().cast::<u8>(), n * 8) });
        Self {
            log_n,
            width,
            f,
            g,
            sigma,
            digest: *h.finalize().as_bytes(),
        }
    }
    fn challenger(&self) -> FsChallenger {
        let mut c =
            FsChallenger::with_hash(b"permutation-optimized-comparison-v1", HashKind::Blake3);
        c.observe_bytes(&self.digest);
        c
    }
    fn bits(&self, reference: bool) -> Result<(PackedBits, PackedBits, permutation::Statement)> {
        let n = self.f.len();
        let width = self.width + self.log_n;
        let left = |i: usize| self.f[i] as u128 | ((i as u128) << self.width);
        let right = |i: usize| self.g[i] as u128 | ((self.sigma[i] as u128) << self.width);
        let (a, b) = if reference {
            let a = PackedBits::from_rows(&(0..n).map(left).collect::<Vec<_>>(), width)?;
            let b = PackedBits::from_rows(&(0..n).map(right).collect::<Vec<_>>(), width)?;
            (a, b)
        } else {
            (
                PackedBits::from_fn_parallel(n, width, left)?,
                PackedBits::from_fn_parallel(n, width, right)?,
            )
        };
        let st = if reference {
            permutation::Statement::new(&a, &b)?
        } else {
            permutation::Statement::new_parallel(&a, &b)?
        };
        Ok((a, b, st))
    }
}
fn check_flock(input: &Input, claim: &product_gkr::ProductGkrBatchedClaim) -> Result<()> {
    if claim.rho.len() != input.log_n {
        return Err("wrong source point".into());
    }
    let eq = SplitEqGhash::with_n_hi(&claim.rho, input.log_n / 2);
    let actual = (0..eq.hi.len())
        .into_par_iter()
        .map(|hi| {
            let mut sum = [F256Unreduced::ZERO; 3];
            let start = hi * eq.lo.len();
            for (j, &weight) in eq.lo.iter().enumerate() {
                let i = start + j;
                let values = [
                    phi8(F8(input.f[i])),
                    phi8(F8(input.g[i])),
                    F128::new(input.sigma[i] as u64, 0),
                ];
                for k in 0..3 {
                    sum[k] ^= values[k].mul_unreduced(weight);
                }
            }
            sum.map(|v| v.reduce() * eq.hi[hi])
        })
        .reduce(
            || [F128::ZERO; 3],
            |a, b| std::array::from_fn(|k| a[k] + b[k]),
        );
    if actual != [claim.f_eval, claim.g_eval, claim.s_sigma_eval] {
        return Err("Flock original source/sigma mismatch".into());
    }
    Ok(())
}
fn memory() -> Value {
    let status = fs::read_to_string("/proc/self/status").unwrap_or_default();
    let mut values = serde_json::Map::new();
    for line in status.lines() {
        if let Some((name, value)) = line.split_once(':')
            && ["VmRSS", "VmHWM", "VmSize"].contains(&name)
        {
            values.insert(
                name.into(),
                json!(
                    value
                        .split_whitespace()
                        .next()
                        .unwrap()
                        .parse::<u64>()
                        .unwrap()
                ),
            );
        }
    }
    json!(values)
}
fn run_one(
    input: &Input,
    system: &str,
    run: usize,
    order: usize,
    proof_dir: &std::path::Path,
) -> Result<Value> {
    let mut c = input.challenger();
    let total = Instant::now();
    let t = Instant::now();
    let (prepare, core, total_ms, verify, source, proof, phases, stats, rss) = if system == "flock"
    {
        let f: Vec<_> = input.f.par_iter().map(|&x| phi8(F8(x))).collect();
        let g: Vec<_> = input.g.par_iter().map(|&x| phi8(F8(x))).collect();
        let prepare = ms(t);
        let t = Instant::now();
        let (proof, claims) = product_gkr::prove_batched(&f, &g, &input.sigma, None, &mut c);
        let core = ms(t);
        let total_ms = ms(total);
        let rss = memory();
        let mut v = input.challenger();
        let t = Instant::now();
        let verified = product_gkr::verify_batched(input.log_n, &proof, None, &mut v)
            .map_err(|e| format!("Flock verify: {e:?}"))?;
        let verify = ms(t);
        if claims != verified || c.sample_f128() != v.sample_f128() {
            return Err("Flock transcript mismatch".into());
        }
        let t = Instant::now();
        check_flock(input, &verified)?;
        let source = ms(t);
        (
            prepare,
            core,
            total_ms,
            verify,
            source,
            bincode::serialize(&proof)?,
            Value::Null,
            Value::Null,
            rss,
        )
    } else {
        let (engine, strategy) = match system {
            "reference" => (Engine::Reference, Strategy::BitwidthAware),
            "reference_full" => (Engine::Reference, Strategy::FullExtension),
            "parallel" => (Engine::ParallelTree, Strategy::BitwidthAware),
            "lookup" => (Engine::LookupTree, Strategy::BitwidthAware),
            "optimized" => (Engine::Fused, Strategy::BitwidthAware),
            "full" => (Engine::Fused, Strategy::FullExtension),
            _ => return Err(format!("unknown system: {system}").into()),
        };
        let (left, right, st) = input.bits(engine == Engine::Reference)?;
        let prepare = ms(t);
        let t = Instant::now();
        let (proof, claims, stats, phases) = permutation::prove_profiled(
            &st,
            &left,
            &right,
            Options {
                strategy,
                ..Default::default()
            },
            engine,
            &mut c,
        )?;
        let core = ms(t);
        let total_ms = ms(total);
        let rss = memory();
        let mut v = input.challenger();
        let t = Instant::now();
        let verified = permutation::verify(&st, &proof, &mut v)?;
        let verify = ms(t);
        if claims != verified || c.sample_f128() != v.sample_f128() {
            return Err("Mikros transcript mismatch".into());
        }
        let t = Instant::now();
        claims.check_parallel(&[(Source::Left, &left), (Source::Right, &right)])?;
        let source = ms(t);
        (
            prepare,
            core,
            total_ms,
            verify,
            source,
            bincode::serialize(&proof)?,
            serde_json::to_value(phases)?,
            serde_json::to_value(stats)?,
            rss,
        )
    };
    let proof_digest = blake3::hash(&proof).to_hex().to_string();
    fs::write(proof_dir.join(format!("run{run}-{system}.bin")), &proof)?;
    Ok(
        json!({"system":system,"run":run,"order":order,"log_n":input.log_n,"value_bits":input.width,"tuple_bits":input.log_n+input.width,
        "input_digest":blake3::Hash::from(input.digest).to_hex().to_string(),"proof_digest":proof_digest,"proof_bytes":proof.len(),
        "prepare_ms":prepare,"prover_core_ms":core,"prover_total_ms":total_ms,"verifier_ms":verify,"source_check_ms":source,"phases":phases,"tree_stats":stats,"memory_at_prover_end_kib":rss,"verified":true}),
    )
}
fn main() -> Result<()> {
    let mut log_n = 20usize;
    let mut width = 8usize;
    let mut runs = 6usize;
    let mut warmups = 1usize;
    let mut threads = 1usize;
    let mut seed = 0u64;
    let mut out = None;
    let mut systems: Vec<String> = [
        "reference",
        "parallel",
        "lookup",
        "optimized",
        "full",
        "flock",
    ]
    .map(String::from)
    .to_vec();
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let value = args.next().ok_or("option value missing")?;
        match flag.as_str() {
            "--log-n" => log_n = value.parse()?,
            "--width" => width = value.parse()?,
            "--runs" => runs = value.parse()?,
            "--warmups" => warmups = value.parse()?,
            "--threads" => threads = value.parse()?,
            "--seed" => seed = value.parse()?,
            "--out" => out = Some(PathBuf::from(value)),
            "--systems" => systems = value.split(',').map(String::from).collect(),
            _ => return Err(format!("unknown argument {flag}").into()),
        }
    }
    if !(2..=30).contains(&log_n)
        || !log_n.is_multiple_of(2)
        || !(1..=8).contains(&width)
        || runs == 0
        || threads == 0
        || systems.is_empty()
    {
        return Err("invalid dimensions".into());
    }
    let out = out.ok_or("--out required")?;
    if let Some(p) = out.parent() {
        fs::create_dir_all(p)?;
    }
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&out)?;
    let mut stream = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(out.with_extension("samples.jsonl"))?;
    let proof_dir = out.with_extension("proofs");
    fs::create_dir(&proof_dir)?;
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()?;
    if !flock_parallel::init_all_core_pool(threads) {
        return Err("pool already initialized".into());
    }
    let features: Vec<_> = [
        ("aes", cfg!(target_feature = "aes")),
        ("pclmulqdq", cfg!(target_feature = "pclmulqdq")),
        ("avx2", cfg!(target_feature = "avx2")),
        ("gfni", cfg!(target_feature = "gfni")),
        ("avx512f", cfg!(target_feature = "avx512f")),
        ("vpclmulqdq", cfg!(target_feature = "vpclmulqdq")),
    ]
    .into_iter()
    .filter_map(|(s, b)| b.then_some(s))
    .collect();
    let metadata = json!({"start_unix_seconds":SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),"log_n":log_n,"width":width,"runs":runs,"warmups":warmups,"threads":threads,"systems":systems,"seed":seed,"rustc":option_env!("BENCH_RUSTC"),"target_features":features,"generator":"4-round balanced Feistel public permutation; SplitMix64 byte witness; full data digest bound before challenges", "scope":"Same public relation f[sigma[i]]=g[i]; PIOP with every source claim authenticated; no PCS/grinding. Same-message optimization, not a new batched protocol. Pool reuse enabled for Mikros and Flock.","affinity":fs::read_to_string("/proc/self/status").unwrap_or_default().lines().filter(|s|s.starts_with("Cpus_allowed_list:")||s.starts_with("Mems_allowed_list:")).collect::<Vec<_>>(),"memory_scope":"VmHWM at prover return is process-lifetime cumulative; use separate cold one-system process for comparable RSS"});
    let mut samples = Vec::new();
    let orders = [
        [0, 1, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
        [0, 2, 1],
        [1, 0, 2],
    ];
    for run in 0..warmups + runs {
        let input = Input::new(log_n, width, seed + run as u64);
        let mut order: Vec<_> = (0..systems.len()).collect();
        if run >= warmups {
            let i = run - warmups;
            if systems.len() == 3 {
                order = orders[i % 6].to_vec();
            } else {
                order.rotate_left(i % systems.len());
            }
        }
        let mut expected = None;
        for (position, index) in order.into_iter().enumerate() {
            let system = &systems[index];
            let mut record = run_one(&input, system, run, position, &proof_dir)?;
            record["warmup"] = json!(run < warmups);
            if system != "flock" {
                let digest = record["proof_digest"].as_str().unwrap();
                if let Some(d) = &expected {
                    if d != digest {
                        return Err("Mikros variant proof mismatch".into());
                    }
                } else {
                    expected = Some(digest.to_owned());
                }
            }
            eprintln!(
                "n={log_n} threads={threads} run={run} {system} core={:.3} total={:.3} source={:.3} ms",
                record["prover_core_ms"].as_f64().unwrap(),
                record["prover_total_ms"].as_f64().unwrap(),
                record["source_check_ms"].as_f64().unwrap()
            );
            writeln!(stream, "{}", serde_json::to_string(&record)?)?;
            stream.flush()?;
            samples.push(record);
        }
    }
    serde_json::to_writer_pretty(
        &mut file,
        &json!({"schema_version":2,"metadata":metadata,"samples":samples}),
    )?;
    writeln!(file)?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn feistel_is_bijective_and_relation_and_sigma_binding_are_checked() {
        for log_n in [2, 4, 8, 12] {
            let input = Input::new(log_n, 8, 0);
            let mut seen = vec![false; 1 << log_n];
            for (i, &s) in input.sigma.iter().enumerate() {
                assert!(!seen[s]);
                seen[s] = true;
                assert_eq!(input.f[s], input.g[i]);
            }
        }
        let input = Input::new(4, 8, 1);
        let f: Vec<_> = input.f.iter().map(|&v| phi8(F8(v))).collect();
        let g: Vec<_> = input.g.iter().map(|&v| phi8(F8(v))).collect();
        let (_, mut claim) =
            product_gkr::prove_batched(&f, &g, &input.sigma, None, &mut input.challenger());
        check_flock(&input, &claim).unwrap();
        claim.s_sigma_eval += F128::ONE;
        assert!(check_flock(&input, &claim).is_err());
    }
}
