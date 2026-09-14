use mikros_binius_permutation::{Proof, System, parameters, prove, verify};
use serde_json::{Value, json};
use std::{fs, io::Write, path::PathBuf};

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
fn parse_list(s: &str) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    Ok(s.split(',').map(str::parse).collect::<Result<_, _>>()?)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut logs, mut widths, mut runs, mut threads) = (vec![16, 18, 20], vec![1, 8, 32], 4, 1);
    let mut out = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let value = args.next().ok_or("missing option value")?;
        match arg.as_str() {
            "--logs" => logs = parse_list(&value)?,
            "--widths" => widths = parse_list(&value)?,
            "--runs" => runs = value.parse()?,
            "--threads" => threads = value.parse()?,
            "--out" => out = Some(PathBuf::from(value)),
            _ => return Err(format!("unknown argument {arg}").into()),
        }
    }
    if runs == 0 || threads == 0 || logs.is_empty() || widths.is_empty() {
        return Err("empty experiment".into());
    }
    for &n in &logs {
        for &w in &widths {
            parameters(n, w)?;
        }
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build_global()?;
    if !flock_parallel::init_all_core_pool(threads) {
        return Err("pool already initialized".into());
    }
    let out = out.ok_or("--out new JSONL path required")?;
    if let Some(parent) = out.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut output = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&out)?;
    let mut rows = 0;
    for &log_n in &logs {
        for &width in &widths {
            let (params, pc, _) = parameters(log_n, width)?;
            for run in 0..=runs {
                let seed = 0x62696e6975735f70
                    ^ ((log_n as u64) << 40)
                    ^ ((width as u64) << 24)
                    ^ run as u64;
                let mut rng = Rng(seed);
                let n = 1usize << log_n;
                let mask = (1u128 << width) - 1;
                let f: Vec<_> = (0..n).map(|_| rng.next() as u128 & mask).collect();
                let mut sigma: Vec<_> = (0..n).collect();
                for i in (1..n).rev() {
                    let j = rng.next() as usize % (i + 1);
                    sigma.swap(i, j);
                }
                let g: Vec<_> = sigma.iter().map(|&i| f[i]).collect();
                let mut h = blake3::Hasher::new();
                for col in [&f, &g] {
                    for x in col {
                        h.update(&x.to_le_bytes());
                    }
                }
                for &i in &sigma {
                    h.update(&(i as u64).to_le_bytes());
                }
                let input_hash = h.finalize().to_hex().to_string();
                let order = if run % 2 == 0 {
                    [System::Mikros, System::Flock]
                } else {
                    [System::Flock, System::Mikros]
                };
                let mut commitment_id = None;
                for (position, system) in order.into_iter().enumerate() {
                    let result =
                        prove(&f, &g, &sigma, width, system, b"paired-local-benchmark-v1")?;
                    let statement_bytes = bincode::serialize(&result.statement)?;
                    let statement_hash = blake3::hash(&statement_bytes).to_hex().to_string();
                    if let Some(previous) = &commitment_id
                        && previous != &statement_hash
                    {
                        return Err("paired commitments differ".into());
                    }
                    commitment_id = Some(statement_hash.clone());
                    let bytes = bincode::serialize(&result.proof)?;
                    let decoded: Proof = bincode::deserialize(&bytes)?;
                    let (vt, tail) = verify(
                        &result.statement,
                        &decoded,
                        system,
                        b"paired-local-benchmark-v1",
                    )?;
                    if tail != result.transcript_tail {
                        return Err("transcript replay mismatch".into());
                    }
                    let source_pcs_bytes: usize = result
                        .proof
                        .sources
                        .iter()
                        .map(|p| bincode::serialized_size(&p.opening).unwrap() as usize)
                        .sum();
                    let bridge_bytes: usize = result
                        .proof
                        .sources
                        .iter()
                        .map(|p| bincode::serialized_size(&p.columns).unwrap() as usize)
                        .sum();
                    let row: Value = json!({"kind":"sample","log_n":log_n,"value_bits":width,
                    "tuple_bits":log_n+width,"system":system,"run":run,"warmup":run==0,
                    "order":position,"threads":threads,"seed":seed,"input_hash":input_hash,
                    "statement_hash":statement_hash,"proof_hash":blake3::hash(&bytes).to_hex().to_string(),
                    "proof_bytes":bytes.len(),"piop_bytes":bincode::serialized_size(&result.proof.piop)?,
                    "bridge_bytes":bridge_bytes,"pcs_opening_bytes":source_pcs_bytes,
                    "commitment_bytes":bincode::serialized_size(&result.statement.commitments)?,
                    "public_statement_bytes":statement_bytes.len(),
                    "packed_witness_bytes_each":params.msg_len_f128()*16,
                    "codeword_bytes_each":params.codeword_len_f128()*16,
                    "prover":result.timings,"verifier":vt,"verified":true,"transcript_equal":true,
                    "pcs_params":params,"pcs_queries":pc.queries,"pcs_log_inv_rates":pc.log_inv_rates,
                    "embedded_profile":flock_core::pcs::ligerito::embedded_initial_k(params.m,params.profile).is_some()});
                    writeln!(output, "{row}")?;
                    output.flush()?;
                    rows += 1;
                    eprintln!(
                        "n={log_n} w={width} run={run}/{runs} {system:?}: prove={:.2}ms verify={:.2}ms bytes={}",
                        result.timings.prover_total_ms,
                        vt.verifier_total_ms,
                        bytes.len()
                    );
                }
            }
        }
    }
    writeln!(
        output,
        "{}",
        json!({"kind":"completed","rows":rows,"logs":logs,"widths":widths,"runs":runs,"threads":threads})
    )?;
    Ok(())
}
