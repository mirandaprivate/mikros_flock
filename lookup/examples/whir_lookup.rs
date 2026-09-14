//! Small end-to-end smoke measurement; input generation is outside timers.
#[cfg(feature = "whir")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use mikros_lookup::{
        authenticated,
        bits::{PackedBits, bits_for},
        permutation::Options,
    };
    use std::time::Instant;
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let n = args
        .first()
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(64usize);
    let width = args
        .get(1)
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(8usize);
    if width == 0 || width > 128 {
        return Err("value_bits must be 1..=128".into());
    }
    if n > 4096 {
        return Err("smoke example caps rows at 4096".into());
    }
    let m = 8usize.min(1usize << width.min(3));
    let high = 1u128 << (width - 1);
    let table = (0..m).map(|i| (i as u128) | high).collect::<Vec<_>>();
    let addresses = (0..n)
        .map(|i| ((i * 7 + i / 3) % m) as u128)
        .collect::<Vec<_>>();
    let values = addresses
        .iter()
        .map(|&a| table[a as usize])
        .collect::<Vec<_>>();
    let input = PackedBits::from_rows(&values, width)?;
    let mut reversed = values.clone();
    reversed.reverse();
    let shuffled = PackedBits::from_rows(&reversed, width)?;
    let start = Instant::now();
    let (ps, pp, stats) = authenticated::prove_permutation(
        &input,
        &shuffled,
        Options::default(),
        b"whir-smoke-permutation",
    )?;
    let perm_ms = start.elapsed().as_secs_f64() * 1000.;
    let start = Instant::now();
    authenticated::verify_permutation(&ps, &pp, b"whir-smoke-permutation")?;
    let perm_verify_ms = start.elapsed().as_secs_f64() * 1000.;
    let table = PackedBits::from_rows(&table, width)?;
    let addresses = PackedBits::from_rows(&addresses, bits_for(m - 1))?;
    let start = Instant::now();
    let (ls, lp, lasso_stats) = authenticated::prove_lasso(
        table,
        &input,
        &addresses,
        Options::default(),
        b"whir-smoke-lasso",
    )?;
    let lasso_ms = start.elapsed().as_secs_f64() * 1000.;
    let start = Instant::now();
    authenticated::verify_lasso(&ls, &lp, b"whir-smoke-lasso")?;
    let lasso_verify_ms = start.elapsed().as_secs_f64() * 1000.;
    let result = serde_json::json!({
        "profile":"GHASH F128 / additive RS / WHIR Unique rate 1/2 / 96-bit diagnostic / non-ZK / zero PoW",
        "rows":n,"value_bits":width,"table_rows":m,"samples":1,
        "timing_scope":"input generation excluded; prover includes PCS construction, commitments, auxiliary witness, PIOP, Booleanity and all openings; fresh verifier includes public covector generation",
        "permutation":{"verified":true,"prover_ms":perm_ms,"verifier_ms":perm_verify_ms,"proof_bytes":bincode::serialize(&pp)?.len(),"statement_bytes":bincode::serialize(&ps)?.len(),"whir_openings":pp.sources.len(),"tree_stats":stats},
        "lasso":{"verified":true,"prover_ms":lasso_ms,"verifier_ms":lasso_verify_ms,"proof_bytes":bincode::serialize(&lp)?.len(),"statement_bytes":bincode::serialize(&ls)?.len(),"whir_openings":lp.sources.len(),"stats":lasso_stats}
    });
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
#[cfg(not(feature = "whir"))]
fn main() {
    eprintln!("Enable the whir feature to run this example.");
}
