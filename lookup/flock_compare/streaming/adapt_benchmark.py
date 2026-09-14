"""Add the previous candidate as an independently compiled paired control."""
from pathlib import Path
p=Path(__file__).resolve().parent/'src/bin/compare.rs'
s=p.read_text(encoding='utf-8')
marker='    } else {\n        let (engine, strategy) = match system {'
assert s.count(marker)==1
branch='''    } else if system == "previous" || system == "previous_full" {
        use previous::{bits::PackedBits as OldBits, claims::Source as OldSource, permutation as old};
        let n = input.f.len();
        let width = input.width + input.log_n;
        let left = OldBits::from_fn_parallel(n, width, |i| input.f[i] as u128 | ((i as u128) << input.width))?;
        let right = OldBits::from_fn_parallel(n, width, |i| input.g[i] as u128 | ((input.sigma[i] as u128) << input.width))?;
        let st = old::Statement::new_parallel(&left, &right)?;
        let prepare = ms(t);
        let t = Instant::now();
        let strategy = if system == "previous" { old::Strategy::BitwidthAware } else { old::Strategy::FullExtension };
        let (proof, claims, stats, phases) = old::prove_profiled(&st, &left, &right,
            old::Options { strategy, ..Default::default() }, previous::profile::Engine::Fused, &mut c)?;
        let core = ms(t);
        let total_ms = ms(total);
        let rss = memory();
        let mut v = input.challenger();
        let t = Instant::now();
        let verified = old::verify(&st, &proof, &mut v)?;
        let verify = ms(t);
        if claims != verified || c.sample_f128() != v.sample_f128() {
            return Err("Previous Mikros transcript mismatch".into());
        }
        let t = Instant::now();
        claims.check_parallel(&[(OldSource::Left, &left), (OldSource::Right, &right)])?;
        let source = ms(t);
        (prepare, core, total_ms, verify, source, bincode::serialize(&proof)?,
            serde_json::to_value(phases)?, serde_json::to_value(stats)?, rss)
    } else {
        let (engine, strategy) = match system {'''
s=s.replace(marker,branch)
s=s.replace('"optimized" => (Engine::Fused, Strategy::BitwidthAware),','"release" => (Engine::Released, Strategy::BitwidthAware),\n            "optimized" => (Engine::Fused, Strategy::BitwidthAware),')
s=s.replace('''        "reference",
        "parallel",
        "lookup",
        "optimized",
        "full",
        "flock",''','''        "previous",
        "release",
        "optimized",
        "previous_full",
        "full",
        "flock",''')
p.write_text(s,encoding='utf-8')
