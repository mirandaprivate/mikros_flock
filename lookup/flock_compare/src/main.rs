//! Compare the SAME relation f[sigma[i]] = g[i], with source authentication.
//! Mikros uses exact (value, position) tuples; Flock uses its production tagged GKR.
use flock_core::product_gkr::{self, ProductGkrBatchedClaim};
use flock_field::F128;
use flock_hash::HashKind;
use flock_transcript::challenger::{Challenger, FsChallenger};
use mikros_lookup::{
    bits::PackedBits,
    claims::Source,
    field::SmallField,
    permutation::{self, Options, Strategy},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    hint::black_box,
    io::Write,
    path::PathBuf,
    time::{Instant, SystemTime, UNIX_EPOCH},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum System {
    MikrosCompact,
    MikrosFull,
    Flock,
}
impl System {
    fn name(self) -> &'static str {
        match self {
            Self::MikrosCompact => "mikros_compact",
            Self::MikrosFull => "mikros_full",
            Self::Flock => "flock",
        }
    }
}

struct Input {
    log_n: usize,
    width: usize,
    f: Vec<u128>,
    g: Vec<u128>,
    sigma: Vec<usize>,
    digest: [u8; 32],
}
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}
impl Input {
    fn new(log_n: usize, width: usize, run: usize) -> Self {
        let n = 1 << log_n;
        let mut rng =
            Rng(0x7065726d5f706169 ^ ((log_n as u64) << 40) ^ ((width as u64) << 24) ^ run as u64);
        let mask = (1u128 << width) - 1;
        let g: Vec<_> = (0..n).map(|_| rng.next() as u128 & mask).collect();
        let mut sigma: Vec<_> = (0..n).collect();
        for i in (1..n).rev() {
            let j = rng.next() as usize % (i + 1);
            sigma.swap(i, j);
        }
        let mut f = vec![0; n];
        for i in 0..n {
            f[sigma[i]] = g[i];
        }
        let mut input = Self {
            log_n,
            width,
            f,
            g,
            sigma,
            digest: [0; 32],
        };
        input.rehash();
        input
    }
    fn rehash(&mut self) {
        let mut h = blake3::Hasher::new();
        h.update(b"known-permutation-input-v1");
        h.update(&(self.log_n as u64).to_le_bytes());
        h.update(&(self.width as u64).to_le_bytes());
        for col in [&self.f, &self.g] {
            for x in col {
                h.update(&x.to_le_bytes());
            }
        }
        for &i in &self.sigma {
            h.update(&(i as u64).to_le_bytes());
        }
        self.digest = *h.finalize().as_bytes();
    }
    fn challenger(&self) -> FsChallenger {
        let mut ch = FsChallenger::with_hash(b"known-permutation-comparison-v1", HashKind::Blake3);
        ch.observe_bytes(&self.digest);
        ch
    }
    fn tuples(&self) -> Result<(PackedBits, PackedBits, permutation::Statement)> {
        let left: Vec<_> = self
            .f
            .iter()
            .enumerate()
            .map(|(i, &v)| v | ((i as u128) << self.width))
            .collect();
        let right: Vec<_> = self
            .g
            .iter()
            .zip(&self.sigma)
            .map(|(&v, &i)| v | ((i as u128) << self.width))
            .collect();
        let left = PackedBits::from_rows(&left, self.width + self.log_n)?;
        let right = PackedBits::from_rows(&right, self.width + self.log_n)?;
        let statement = permutation::Statement::new(&left, &right)?;
        Ok((left, right, statement))
    }
    fn embedded(&self) -> Result<(Vec<F128>, Vec<F128>)> {
        let field = SmallField::for_width(self.width)?;
        Ok((
            self.f.iter().map(|&v| field.embed(v)).collect(),
            self.g.iter().map(|&v| field.embed(v)).collect(),
        ))
    }
}

fn ms(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

// Independent full evaluations from original integer inputs; never trust a
// prover-prepared field array or the unpinned sigma evaluation.
fn check_flock(input: &Input, claim: &ProductGkrBatchedClaim) -> Result<()> {
    if claim.rho.len() != input.log_n {
        return Err("wrong Flock point dimension".into());
    }
    let mut eq = vec![F128::ONE];
    for &r in &claim.rho {
        let len = eq.len();
        eq.resize(2 * len, F128::ZERO);
        for i in 0..len {
            let x = eq[i];
            eq[i] = x * (F128::ONE + r);
            eq[len + i] = x * r;
        }
    }
    let basis = SmallField::for_width(input.width)?.basis();
    let from_bits = |value: u128| {
        (0..input.width).fold(F128::ZERO, |sum, bit| {
            if value >> bit & 1 == 1 {
                sum + basis[bit]
            } else {
                sum
            }
        })
    };
    let mut actual = [F128::ZERO; 3];
    for (i, &w) in eq.iter().enumerate() {
        actual[0] += w * from_bits(input.f[i]);
        actual[1] += w * from_bits(input.g[i]);
        actual[2] += w * F128::new(input.sigma[i] as u64, 0);
    }
    if actual != [claim.f_eval, claim.g_eval, claim.s_sigma_eval] {
        return Err("Flock source or sigma claim mismatch".into());
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Sample {
    log_n: usize,
    value_bits: usize,
    tuple_bits: usize,
    run: usize,
    order: usize,
    system: System,
    input_digest: String,
    proof_digest: String,
    prepare_ms: f64,
    prover_core_ms: f64,
    prover_total_ms: f64,
    verifier_core_ms: f64,
    source_check_ms: f64,
    sigma_aware_verifier_ms: Option<f64>,
    bincode_bytes: usize,
    field_elements: usize,
    tree_stats: Option<permutation::Stats>,
}

fn run_one(input: &Input, system: System, run: usize, order: usize) -> Result<Sample> {
    let mut p = input.challenger();
    let total = Instant::now();
    let prep = Instant::now();
    let (
        prepare_ms,
        prover_core_ms,
        prover_total_ms,
        verifier_core_ms,
        source_check_ms,
        sigma_aware_verifier_ms,
        encoded,
        fields,
        stats,
    ) = match system {
        System::MikrosCompact | System::MikrosFull => {
            let (left, right, statement) = input.tuples()?;
            let prepare_ms = ms(prep);
            let start = Instant::now();
            let strategy = if system == System::MikrosCompact {
                Strategy::BitwidthAware
            } else {
                Strategy::FullExtension
            };
            let (proof, claims, stats) = permutation::prove(
                &statement,
                &left,
                &right,
                Options {
                    strategy,
                    ..Default::default()
                },
                &mut p,
            )?;
            let core = ms(start);
            let total_ms = ms(total);
            let mut v = input.challenger();
            let start = Instant::now();
            let verified = permutation::verify(&statement, &proof, &mut v)?;
            let verify_ms = ms(start);
            if claims != verified || p.sample_f128() != v.sample_f128() {
                return Err("Mikros replay mismatch".into());
            }
            let start = Instant::now();
            verified.check(&[(Source::Left, &left), (Source::Right, &right)])?;
            let source_ms = ms(start);
            let fields = proof.field_elements();
            (
                prepare_ms,
                core,
                total_ms,
                verify_ms,
                source_ms,
                None,
                bincode::serialize(&proof)?,
                fields,
                Some(stats),
            )
        }
        System::Flock => {
            let (f, g) = input.embedded()?;
            let prepare_ms = ms(prep);
            let start = Instant::now();
            let (proof, claim) = product_gkr::prove_batched(&f, &g, &input.sigma, None, &mut p);
            let core = ms(start);
            let total_ms = ms(total);
            let mut v = input.challenger();
            let start = Instant::now();
            let verified = product_gkr::verify_batched(input.log_n, &proof, None, &mut v)
                .map_err(|e| format!("Flock verify: {e:?}"))?;
            let verify_ms = ms(start);
            if claim != verified || p.sample_f128() != v.sample_f128() {
                return Err("Flock replay mismatch".into());
            }
            let start = Instant::now();
            check_flock(input, &verified)?;
            let source_ms = ms(start);
            // Also exercise the real sigma-aware verifier; its O(N) public
            // sigma evaluation is reported separately from PIOP-only replay.
            let mut v_sigma = input.challenger();
            let start = Instant::now();
            let pinned = product_gkr::verify_batched_with_sigma(
                input.log_n,
                &proof,
                &input.sigma,
                None,
                &mut v_sigma,
            )
            .map_err(|e| format!("sigma-aware verify: {e:?}"))?;
            let sigma_ms = ms(start);
            if pinned != claim {
                return Err("sigma-aware claim mismatch".into());
            }
            let fields = 5 + proof
                .layers
                .iter()
                .map(|layer| 2 * layer.rounds.len() + 4)
                .sum::<usize>();
            (
                prepare_ms,
                core,
                total_ms,
                verify_ms,
                source_ms,
                Some(sigma_ms),
                bincode::serialize(&proof)?,
                fields,
                None,
            )
        }
    };
    let sample = Sample {
        log_n: input.log_n,
        value_bits: input.width,
        tuple_bits: input.width + input.log_n,
        run,
        order,
        system,
        input_digest: blake3::Hash::from(input.digest).to_hex().to_string(),
        proof_digest: blake3::hash(&encoded).to_hex().to_string(),
        prepare_ms,
        prover_core_ms,
        prover_total_ms,
        verifier_core_ms,
        source_check_ms,
        sigma_aware_verifier_ms,
        bincode_bytes: encoded.len(),
        field_elements: fields,
        tree_stats: stats,
    };
    black_box(encoded);
    Ok(sample)
}

fn timing(values: impl Iterator<Item = f64>) -> Value {
    let mut v: Vec<_> = values.collect();
    v.sort_by(f64::total_cmp);
    let n = v.len();
    let median = if n % 2 == 0 {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    } else {
        v[n / 2]
    };
    json!({"median": median, "min": v[0], "max": v[n-1]})
}
fn summaries(samples: &[Sample]) -> Vec<Value> {
    let mut groups: BTreeMap<(usize, usize, &str), Vec<&Sample>> = BTreeMap::new();
    for sample in samples {
        groups
            .entry((sample.log_n, sample.value_bits, sample.system.name()))
            .or_default()
            .push(sample);
    }
    groups.into_iter().map(|((n, w, system), s)| json!({"log_n":n,"value_bits":w,"tuple_bits":n+w,"system":system,"runs":s.len(),
        "prepare_ms":timing(s.iter().map(|s|s.prepare_ms)),"prover_core_ms":timing(s.iter().map(|s|s.prover_core_ms)),
        "prover_total_ms":timing(s.iter().map(|s|s.prover_total_ms)),"verifier_core_ms":timing(s.iter().map(|s|s.verifier_core_ms)),
        "source_check_ms":timing(s.iter().map(|s|s.source_check_ms)),"bincode_bytes":s[0].bincode_bytes,"field_elements":s[0].field_elements,
        "sigma_aware_verifier_ms":if system=="flock" {timing(s.iter().map(|s|s.sigma_aware_verifier_ms.unwrap()))}else{Value::Null},"tree_stats":s[0].tree_stats
    })).collect()
}
fn list(s: &str) -> Result<Vec<usize>> {
    Ok(s.split(',')
        .map(str::parse)
        .collect::<std::result::Result<_, _>>()?)
}

fn main() -> Result<()> {
    let mut logs = vec![12, 16, 20];
    let mut widths = vec![1, 8, 32];
    let mut runs = 6;
    let mut threads = 1;
    let mut out = None;
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let value = args.next().ok_or("missing option value")?;
        match flag.as_str() {
            "--logs" => logs = list(&value)?,
            "--widths" => widths = list(&value)?,
            "--runs" => runs = value.parse()?,
            "--threads" => threads = value.parse()?,
            "--out" => out = Some(PathBuf::from(value)),
            _ => return Err(format!("unknown option {flag}").into()),
        }
    }
    if runs == 0
        || threads == 0
        || logs.is_empty()
        || widths.is_empty()
        || logs.iter().any(|&n| n > 22 || n == 0)
        || widths.iter().any(|&w| w == 0 || w > 64)
    {
        return Err("invalid dimensions".into());
    }
    let out = out.ok_or("--out is required and must be a new path")?;
    if let Some(p) = out.parent() {
        fs::create_dir_all(p)?;
    }
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&out)?;
    let mut events = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out.with_extension("samples.jsonl"))?;
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()?;
    if !flock_parallel::init_all_core_pool(threads) {
        return Err("all-core pool already initialized".into());
    }
    let cpu = fs::read_to_string("/proc/cpuinfo").ok().and_then(|s| {
        s.lines().find_map(|l| {
            l.strip_prefix("model name")
                .and_then(|v| v.split_once(':').map(|x| x.1.trim().to_owned()))
        })
    });
    let status = fs::read_to_string("/proc/self/status").unwrap_or_default();
    let affinity: Vec<_> = status
        .lines()
        .filter(|l| l.starts_with("Cpus_allowed_list:") || l.starts_with("Mems_allowed_list:"))
        .collect();
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
    let metadata = json!({"unix_seconds":SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),"cpu":cpu,"os":std::env::consts::OS,"arch":std::env::consts::ARCH,
        "rustc":option_env!("BENCH_RUSTC"),"target_features":features,"threads":threads,"rayon_actual_threads":rayon::current_num_threads(),"affinity":affinity,
        "logs":logs,"widths":widths,"runs":runs,"scope":"Same public known permutation f[sigma[i]]=g[i]. Mikros: injective (value,tag) tuples; Flock: production tagged batched product-GKR, no mask. PIOP only; no PCS/ZK/grinding/equal-security assertion.",
        "timing":"prepare: native representation from common raw inputs; prover_core: actual prove including internal checks/hashing; total: prepare+core. Common input generation/digest and challenger prefix are outside. All original source claims checked outside; Flock sigma-aware verifier additionally timed.",
        "source_manifest_blake3":blake3::hash(include_bytes!("../snapshot/manifest.json")).to_hex().to_string(),
        "benchmark_blake3":blake3::hash(include_bytes!("main.rs")).to_hex().to_string(),"source_manifest":serde_json::from_str::<Value>(include_str!("../snapshot/manifest.json"))?,
        "parallelism":"Both pools fixed to --threads. Mikros tree construction and sumcheck/fold loops are scalar; shared eq_table uses Rayon for large tables. Flock uses its production parallel path. Initial subfield caches included in saved warmups.",
        "order":"Six permutations of three systems; same deterministic input within run; no slow samples discarded"});
    let systems = [System::MikrosCompact, System::MikrosFull, System::Flock];
    let orders = [
        [0, 1, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
        [0, 2, 1],
        [1, 0, 2],
    ];
    let mut samples = Vec::new();
    let mut warmups = Vec::new();
    for &n in &logs {
        for &w in &widths {
            for run in 0..=runs {
                let input = Input::new(n, w, run);
                let mut mikros_digest = None;
                let order = if run == 0 {
                    orders[0]
                } else {
                    orders[(run - 1) % orders.len()]
                };
                for (position, index) in order.into_iter().enumerate() {
                    let sample = run_one(&input, systems[index], run, position)?;
                    if sample.system != System::Flock {
                        if let Some(d) = &mikros_digest {
                            if d != &sample.proof_digest {
                                return Err("Mikros strategies produced different proofs".into());
                            }
                        } else {
                            mikros_digest = Some(sample.proof_digest.clone());
                        }
                    }
                    eprintln!(
                        "N=2^{n} width={w}+{n} run={run} {} total={:.3} core={:.3} verify={:.3} ms",
                        sample.system.name(),
                        sample.prover_total_ms,
                        sample.prover_core_ms,
                        sample.verifier_core_ms
                    );
                    writeln!(events, "{}", serde_json::to_string(&sample)?)?;
                    events.flush()?;
                    if run == 0 {
                        warmups.push(sample);
                    } else {
                        samples.push(sample);
                    }
                }
            }
        }
    }
    let report = json!({"schema_version":1,"metadata":metadata,"summaries":summaries(&samples),"warmups":warmups,"samples":samples});
    serde_json::to_writer_pretty(&mut file, &report)?;
    writeln!(file)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn identical_relation_roundtrips_and_mikros_messages_match() {
        for width in [1, 8, 32] {
            let input = Input::new(4, width, 1);
            for i in 0..input.f.len() {
                assert_eq!(input.f[input.sigma[i]], input.g[i]);
            }
            let a = run_one(&input, System::MikrosCompact, 1, 0).unwrap();
            let b = run_one(&input, System::MikrosFull, 1, 1).unwrap();
            assert_eq!(a.proof_digest, b.proof_digest);
            run_one(&input, System::Flock, 1, 2).unwrap();
        }
    }
    #[test]
    fn changed_value_and_wrong_public_mapping_are_rejected() {
        let input = Input::new(4, 8, 1);
        for mapping in [false, true] {
            let mut bad = Input::new(4, 8, 1);
            if mapping {
                let j = (1..bad.g.len()).find(|&j| bad.g[j] != bad.g[0]).unwrap();
                bad.sigma.swap(0, j);
            } else {
                bad.g[0] ^= 1;
            }
            bad.rehash();
            for system in [System::MikrosCompact, System::MikrosFull, System::Flock] {
                assert!(run_one(&bad, system, 1, 0).is_err(), "{system:?}");
            }
        }
        let (f, g) = input.embedded().unwrap();
        let (mut proof, claim) =
            product_gkr::prove_batched(&f, &g, &input.sigma, None, &mut input.challenger());
        let mut changed = claim.clone();
        changed.s_sigma_eval += F128::ONE;
        assert!(check_flock(&input, &changed).is_err());
        proof.layers[1].rounds[0].0 += F128::ONE;
        assert!(
            product_gkr::verify_batched(input.log_n, &proof, None, &mut input.challenger())
                .is_err()
        );
        let (left, right, statement) = input.tuples().unwrap();
        let (mut proof, _, _) = permutation::prove(
            &statement,
            &left,
            &right,
            Options::default(),
            &mut input.challenger(),
        )
        .unwrap();
        proof.roots[0] += F128::ONE;
        assert!(permutation::verify(&statement, &proof, &mut input.challenger()).is_err());
    }
}
