//! Mikros permutation and native Flock product-GKR with the SAME bit PCS.
//! The PCS is Flock's Binius ring-switching + recursive Ligerito backend.
//! Verifiers receive commitments, public sigma and proofs, never the witness.
use flock_core::{
    pcs::{
        self, BatchOpeningProofLigerito, Commitment, OpeningGrinding, PcsParams, ProverData,
        ligerito::{self, LigeritoProfile, ProverConfig, VerifierConfig},
    },
    product_gkr::{self, ProductGkrBatchedProof},
    zerocheck::PaddingSpec,
};
use flock_field::F128;
use flock_hash::HashKind;
use flock_multilinear::{IndexOrder, eq_table};
use flock_transcript::challenger::{Challenger, FsChallenger};
use kernel::{
    bits::PackedBits,
    claims::LinearQuery,
    field::SmallField,
    permutation::{self, Options, Strategy},
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;
mod bridge_fast;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Reference,
    FullExtension,
    Direct,
    Lookup,
    Fused,
    Batched,
    Bridge,
    FullOptimized,
    Packed,
}

pub type Result<T> = std::result::Result<T, String>;
fn err(e: impl std::fmt::Debug) -> String {
    format!("{e:?}")
}
fn elapsed(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum System {
    Mikros,
    Flock,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Statement {
    pub log_n: usize,
    pub value_bits: usize,
    pub sigma: Vec<usize>,
    pub commitments: [Commitment; 2],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PiopProof {
    // The legacy digests are transcript metadata. Source authentication is
    // against Statement.commitments, not an assertion about these digests.
    Mikros {
        statement: permutation::Statement,
        proof: permutation::Proof,
    },
    Flock(ProductGkrBatchedProof),
    Batched {
        statement: permutation::Statement,
        proof: kernel::batched::Proof,
    },
    Packed {
        statement: permutation::Statement,
        proof: kernel::packed::Proof,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SourceOpening {
    /// Bit-column MLEs at the PIOP row point, padded to a power of two.
    pub columns: Vec<F128>,
    pub opening: BatchOpeningProofLigerito,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Proof {
    pub piop: PiopProof,
    pub sources: Vec<SourceOpening>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Timings {
    pub piop_detail: Option<kernel::profile::Timings>,
    pub prepare_ms: f64,
    pub commit_ms: f64,
    pub piop_ms: f64,
    pub bridge_ms: f64,
    pub pcs_open_ms: f64,
    pub prover_total_ms: f64,
    pub verify_setup_ms: f64,
    pub verify_piop_ms: f64,
    pub verify_bridge_ms: f64,
    pub pcs_verify_ms: f64,
    pub verifier_total_ms: f64,
}

pub struct ProverOutput {
    pub statement: Statement,
    pub proof: Proof,
    pub timings: Timings,
    pub transcript_tail: F128,
}

struct Query {
    point: Vec<F128>,
    basis: Vec<F128>,
    value: F128,
}
struct Committed {
    packed: Vec<F128>,
    bits: PackedBits,
    data: ProverData,
}

/// Fixed verifier policy. Large registered shapes use Flock Fast100; smaller
/// shapes use its explicit diagnostic UDR schedule. Neither makes the composed
/// PIOP a 128-bit-security claim. All systems use identical policy per shape.
pub fn parameters(log_n: usize, width: usize) -> Result<(PcsParams, ProverConfig, VerifierConfig)> {
    if !(15..=20).contains(&log_n) || !(1..=64).contains(&width) {
        return Err("supported shape: log_n=15..20, value_bits=1..64".into());
    }
    let m = log_n + width.next_power_of_two().ilog2() as usize;
    let profile = LigeritoProfile::Fast100;
    let initial_k = ligerito::embedded_initial_k(m, profile).unwrap_or((m - 14).min(5));
    let params = PcsParams {
        m,
        log_inv_rate: 1,
        log_batch_size: initial_k,
        profile,
        num_lanes: None,
        merkle_hash: HashKind::Blake3,
    };
    let (mut pc, mut vc) = if ligerito::embedded_initial_k(m, profile).is_some() {
        (
            params.ligerito_prover_config()?,
            params.ligerito_verifier_config()?,
        )
    } else {
        (
            ligerito::default_config(m - 7, initial_k, 1).map_err(err)?,
            ligerito::default_verifier_config(m - 7, initial_k, 1).map_err(err)?,
        )
    };
    pc.merkle_hash = HashKind::Blake3;
    vc.merkle_hash = HashKind::Blake3;
    if pc.level_zero_cap_depth() != params.level_zero_cap_depth() {
        return Err("commit/open cap schedules differ".into());
    }
    Ok((params, pc, vc))
}

fn validate_sigma(log_n: usize, sigma: &[usize]) -> Result<()> {
    if !(15..=20).contains(&log_n) || sigma.len() != 1usize << log_n {
        return Err("invalid sigma shape".into());
    }
    let mut seen = vec![false; sigma.len()];
    for &i in sigma {
        if i >= seen.len() || std::mem::replace(&mut seen[i], true) {
            return Err("sigma must be a permutation".into());
        }
    }
    Ok(())
}

fn begin(s: &Statement, system: System, context: &[u8]) -> Result<FsChallenger> {
    validate_sigma(s.log_n, &s.sigma)?;
    let (params, _, _) = parameters(s.log_n, s.value_bits)?;
    for commitment in &s.commitments {
        if commitment.params != params
            || commitment.cap.len() != 1usize << params.level_zero_cap_depth()
        {
            return Err("commitment does not match verifier policy".into());
        }
    }
    let mut ch = FsChallenger::with_hash(b"mikros-flock-binius-permutation-v1", HashKind::Blake3);
    ch.observe_bytes(&[match system {
        System::Mikros => 0,
        System::Flock => 1,
    }]);
    ch.observe_bytes(&(context.len() as u64).to_le_bytes());
    ch.observe_bytes(context);
    // Serialization is length delimited and binds BOTH caps, shape and sigma
    // before the first grand-product or GKR challenge.
    let bytes = bincode::serialize(s).map_err(err)?;
    ch.observe_bytes(&(bytes.len() as u64).to_le_bytes());
    ch.observe_bytes(&bytes);
    Ok(ch)
}

fn commit_bits(bits: PackedBits, params: &PcsParams) -> (Commitment, Committed) {
    let mut packed = vec![F128::ZERO; 1usize << (params.m - 7)];
    for (i, chunk) in bits.bytes().chunks(16).enumerate() {
        let mut bytes = [0; 16];
        bytes[..chunk.len()].copy_from_slice(chunk);
        let x = u128::from_le_bytes(bytes);
        packed[i] = F128::new(x as u64, (x >> 64) as u64);
    }
    let (commitment, data) = pcs::commit(&packed, params);
    (commitment, Committed { packed, bits, data })
}

fn tuple_data(
    f: &[u128],
    g: &[u128],
    sigma: &[usize],
    width: usize,
    n: usize,
) -> Result<(PackedBits, PackedBits, permutation::Statement)> {
    let a: Vec<_> = f
        .iter()
        .enumerate()
        .map(|(i, &v)| v | ((i as u128) << width))
        .collect();
    let b: Vec<_> = g
        .iter()
        .zip(sigma)
        .map(|(&v, &i)| v | ((i as u128) << width))
        .collect();
    let a = PackedBits::from_rows(&a, width + n).map_err(err)?;
    let b = PackedBits::from_rows(&b, width + n).map_err(err)?;
    let s = permutation::Statement::new(&a, &b).map_err(err)?;
    Ok((a, b, s))
}

fn tag_eval(s: &Statement, side: usize, point: &[F128], basis: &[F128], fast: bool) -> F128 {
    let tags = &basis[s.value_bits..];
    if side == 0 {
        return point
            .iter()
            .zip(tags)
            .fold(F128::ZERO, |acc, (&r, &b)| acc + r * b);
    }
    if fast {
        let bits = PackedBits::from_fn_parallel(s.sigma.len(), tags.len(), |i| s.sigma[i] as u128)
            .expect("validated public permutation");
        let columns = bridge_fast::column_evals(&bits, point);
        return columns
            .iter()
            .zip(tags)
            .fold(F128::ZERO, |v, (&a, &b)| v + a * b);
    }
    let eq = eq_table(point, F128::ONE, IndexOrder::LowToHigh);
    // Public sigma is supplied to the verifier. This is O(N) public work,
    // included in verification time; it never reads f or g.
    eq.iter()
        .zip(&s.sigma)
        .fold(F128::ZERO, |sum, (&w, &index)| {
            let v = tags.iter().enumerate().fold(F128::ZERO, |v, (j, &b)| {
                if index >> j & 1 != 0 { v + b } else { v }
            });
            sum + w * v
        })
}

fn mikros_queries(
    s: &Statement,
    claims: &kernel::claims::Claims,
    fast: bool,
) -> Result<Vec<Query>> {
    if claims.evaluations.len() != 2 {
        return Err("expected both Mikros source claims".into());
    }
    let mut out = Vec::new();
    for (side, claim) in claims.evaluations.iter().enumerate() {
        let LinearQuery::Rows {
            point,
            basis,
            row_offset,
        } = &claim.query
        else {
            return Err("expected row query".into());
        };
        if *row_offset != 0 || point.len() != s.log_n || basis.len() != s.value_bits + s.log_n {
            return Err("invalid Mikros terminal shape".into());
        }
        out.push(Query {
            point: point.clone(),
            basis: basis[..s.value_bits].to_vec(),
            value: claim.value + tag_eval(s, side, point, basis, fast),
        });
    }
    Ok(out)
}

fn column_evals(bits: &PackedBits, point: &[F128]) -> Vec<F128> {
    let eq = eq_table(point, F128::ONE, IndexOrder::LowToHigh);
    let mut values = vec![F128::ZERO; bits.width().next_power_of_two()];
    for (column, value) in values[..bits.width()].iter_mut().enumerate() {
        for (row, &weight) in eq.iter().enumerate() {
            if bits.bit(column * bits.rows() + row) {
                *value += weight;
            }
        }
    }
    values
}

/// Bind all column evaluations before randomizing the column coordinate.
/// Their basis-weighted sum is the PIOP claim; a single bit-MLE opening pins
/// the entire vector with Schwartz-Zippel error <= log2(padded_width)/|F128|.
fn bridge(q: &Query, columns: &[F128], ch: &mut FsChallenger) -> Result<(Vec<F128>, F128)> {
    if columns.len() != q.basis.len().next_power_of_two()
        || columns[q.basis.len()..].iter().any(|&x| x != F128::ZERO)
    {
        return Err("wrong column count or nonzero padding".into());
    }
    let value = columns
        .iter()
        .zip(&q.basis)
        .fold(F128::ZERO, |v, (&a, &b)| v + a * b);
    if value != q.value {
        return Err("terminal claim differs from source columns".into());
    }
    ch.observe_label(b"bit-column-linear-query-v1");
    ch.observe_f128_slice(columns);
    let cr = ch.sample_f128_vec(columns.len().ilog2() as usize);
    let eq = eq_table(&cr, F128::ONE, IndexOrder::LowToHigh);
    let value = columns
        .iter()
        .zip(eq)
        .fold(F128::ZERO, |v, (&a, b)| v + a * b);
    let mut point = q.point.clone();
    point.extend(cr);
    Ok((point, value))
}

fn open_sources(
    s: &Statement,
    data: [Committed; 2],
    queries: &[Query],
    pc: &ProverConfig,
    ch: &mut FsChallenger,
    times: &mut Timings,
    fast: bool,
) -> Result<Vec<SourceOpening>> {
    let mut proofs = Vec::new();
    for (side, d) in data.into_iter().enumerate() {
        let t = Instant::now();
        ch.observe_label(b"source-opening");
        ch.observe_bytes(&[side as u8]);
        let columns = if fast {
            bridge_fast::column_evals(&d.bits, &queries[side].point)
        } else {
            column_evals(&d.bits, &queries[side].point)
        };
        let (point, _) = bridge(&queries[side], &columns, ch)?;
        times.bridge_ms += elapsed(t);
        let t = Instant::now();
        let opening = pcs::open_batch_mixed_ligerito_with_precomputed_s_hat_v_and_grinding(
            d.packed,
            &d.data,
            &s.commitments[side],
            &[&point[6..]],
            &[None],
            &[],
            &PaddingSpec::dense(s.commitments[side].params.m),
            pc,
            OpeningGrinding::disabled(),
            ch,
        );
        times.pcs_open_ms += elapsed(t);
        proofs.push(SourceOpening { columns, opening });
    }
    Ok(proofs)
}

pub fn prove(
    f: &[u128],
    g: &[u128],
    sigma: &[usize],
    width: usize,
    system: System,
    context: &[u8],
) -> Result<ProverOutput> {
    prove_with_mode(f, g, sigma, width, system, context, Mode::FullOptimized)
}

pub fn prove_with_mode(
    f: &[u128],
    g: &[u128],
    sigma: &[usize],
    width: usize,
    system: System,
    context: &[u8],
    mode: Mode,
) -> Result<ProverOutput> {
    let total = Instant::now();
    let mut times = Timings::default();
    let t = Instant::now();
    if !f.len().is_power_of_two() || f.len() != g.len() {
        return Err("input shape".into());
    }
    let log_n = f.len().ilog2() as usize;
    validate_sigma(log_n, sigma)?;
    let (params, pc, _) = parameters(log_n, width)?;
    let reference = matches!(mode, Mode::Reference | Mode::FullExtension);
    let fast_bridge = matches!(mode, Mode::Bridge | Mode::FullOptimized | Mode::Packed);
    let pack = |v: &[u128]| {
        if reference {
            PackedBits::from_rows(v, width)
        } else {
            PackedBits::from_fn_parallel(v.len(), width, |i| v[i])
        }
    };
    let fb = pack(f).map_err(err)?;
    let gb = pack(g).map_err(err)?;
    if sigma.iter().enumerate().any(|(i, &j)| f[j] != g[i]) {
        return Err("invalid permutation relation".into());
    }
    times.prepare_ms += elapsed(t);
    let t = Instant::now();
    let (fc, fd) = commit_bits(fb, &params);
    let (gc, gd) = commit_bits(gb, &params);
    times.commit_ms = elapsed(t);
    let statement = Statement {
        log_n,
        value_bits: width,
        sigma: sigma.to_vec(),
        commitments: [fc, gc],
    };
    let t = Instant::now();
    let mut ch = begin(&statement, system, context)?;
    times.prepare_ms += elapsed(t);
    let (piop, queries) = match system {
        System::Mikros => {
            let t = Instant::now();
            let old = if reference {
                Some(tuple_data(f, g, sigma, width, log_n)?)
            } else {
                None
            };
            let prepared = if reference {
                None
            } else {
                Some(
                    permutation::PreparedRows::new(
                        f.iter()
                            .enumerate()
                            .map(|(i, &v)| v | ((i as u128) << width))
                            .collect(),
                        g.iter()
                            .zip(sigma)
                            .map(|(&v, &i)| v | ((i as u128) << width))
                            .collect(),
                        width + log_n,
                    )
                    .map_err(err)?,
                )
            };
            let st = old
                .as_ref()
                .map(|x| &x.2)
                .unwrap_or_else(|| prepared.as_ref().unwrap().statement())
                .clone();
            times.prepare_ms += elapsed(t);
            let t = Instant::now();
            let options = Options {
                strategy: if matches!(mode, Mode::FullExtension | Mode::FullOptimized) {
                    Strategy::FullExtension
                } else {
                    Strategy::BitwidthAware
                },
                ..Options::default()
            };
            let engine = match mode {
                Mode::Reference | Mode::FullExtension | Mode::Direct => {
                    kernel::profile::Engine::Reference
                }
                Mode::Lookup => kernel::profile::Engine::LookupTree,
                Mode::Fused | Mode::Batched | Mode::Bridge | Mode::FullOptimized | Mode::Packed => {
                    kernel::profile::Engine::Fused
                }
            };
            let (piop, claims, detail) = if mode == Mode::Packed {
                let (proof, claims, _, detail) =
                    kernel::packed::prove(prepared.as_ref().unwrap(), &mut ch).map_err(err)?;
                (
                    PiopProof::Packed {
                        statement: st,
                        proof,
                    },
                    claims,
                    detail,
                )
            } else if matches!(mode, Mode::Batched | Mode::Bridge | Mode::FullOptimized) {
                let (proof, claims, _, detail) =
                    kernel::batched::prove(prepared.as_ref().unwrap(), options, &mut ch)
                        .map_err(err)?;
                (
                    PiopProof::Batched {
                        statement: st,
                        proof,
                    },
                    claims,
                    detail,
                )
            } else {
                let (proof, claims, _, detail) = if let Some((a, b, _)) = &old {
                    permutation::prove_profiled(&st, a, b, options, engine, &mut ch)
                } else {
                    permutation::prove_prepared(
                        prepared.as_ref().unwrap(),
                        options,
                        engine,
                        &mut ch,
                    )
                }
                .map_err(err)?;
                (
                    PiopProof::Mikros {
                        statement: st,
                        proof,
                    },
                    claims,
                    detail,
                )
            };
            times.piop_detail = Some(detail);
            times.piop_ms = elapsed(t);
            let t = Instant::now();
            let queries = mikros_queries(&statement, &claims, fast_bridge)?;
            times.bridge_ms += elapsed(t);
            (piop, queries)
        }
        System::Flock => {
            let t = Instant::now();
            let field = SmallField::for_width(width).map_err(err)?;
            let embed = |v: &[u128]| -> Result<Vec<F128>> {
                if reference {
                    Ok(v.iter().map(|&x| field.embed(x)).collect())
                } else {
                    let fast = kernel::field::FastField::for_width(width).map_err(err)?;
                    Ok(v.par_iter()
                        .with_min_len(2048)
                        .map(|&x| fast.embed(x))
                        .collect())
                }
            };
            let a = embed(f)?;
            let b = embed(g)?;
            times.prepare_ms += elapsed(t);
            let t = Instant::now();
            let (proof, c) = product_gkr::prove_batched(&a, &b, sigma, None, &mut ch);
            times.piop_ms = elapsed(t);
            let basis = field.basis()[..width].to_vec();
            let queries = vec![
                Query {
                    point: c.rho.clone(),
                    basis: basis.clone(),
                    value: c.f_eval,
                },
                Query {
                    point: c.rho,
                    basis,
                    value: c.g_eval,
                },
            ];
            (PiopProof::Flock(proof), queries)
        }
    };
    let sources = open_sources(
        &statement,
        [fd, gd],
        &queries,
        &pc,
        &mut ch,
        &mut times,
        fast_bridge,
    )?;
    times.prover_total_ms = elapsed(total);
    Ok(ProverOutput {
        statement,
        proof: Proof { piop, sources },
        timings: times,
        transcript_tail: ch.sample_f128(),
    })
}

pub fn verify(
    s: &Statement,
    proof: &Proof,
    expected_system: System,
    context: &[u8],
) -> Result<(Timings, F128)> {
    verify_with_mode(s, proof, expected_system, context, Mode::Bridge)
}
pub fn verify_with_mode(
    s: &Statement,
    proof: &Proof,
    expected_system: System,
    context: &[u8],
    mode: Mode,
) -> Result<(Timings, F128)> {
    let total = Instant::now();
    let mut times = Timings::default();
    let fast_bridge = matches!(mode, Mode::Bridge | Mode::FullOptimized | Mode::Packed);
    let t = Instant::now();
    let mut ch = begin(s, expected_system, context)?;
    let (_, _, vc) = parameters(s.log_n, s.value_bits)?;
    if proof.sources.len() != 2 {
        return Err("both source openings required".into());
    }
    times.verify_setup_ms = elapsed(t);
    let t = Instant::now();
    let queries = match (&proof.piop, expected_system) {
        (PiopProof::Mikros { statement, proof }, System::Mikros) => {
            for spec in [&statement.left, &statement.right] {
                if spec.rows != 1usize << s.log_n || spec.width != s.value_bits + s.log_n {
                    return Err("Mikros statement differs from committed shape".into());
                }
            }
            let claims = permutation::verify(statement, proof, &mut ch).map_err(err)?;
            times.verify_piop_ms = elapsed(t);
            let t = Instant::now();
            let queries = mikros_queries(s, &claims, fast_bridge)?;
            times.verify_bridge_ms += elapsed(t);
            queries
        }
        (PiopProof::Batched { statement, proof }, System::Mikros) => {
            for spec in [&statement.left, &statement.right] {
                if spec.rows != 1usize << s.log_n || spec.width != s.value_bits + s.log_n {
                    return Err("Mikros statement differs from committed shape".into());
                }
            }
            let claims = kernel::batched::verify(statement, proof, &mut ch).map_err(err)?;
            times.verify_piop_ms = elapsed(t);
            let t = Instant::now();
            let queries = mikros_queries(s, &claims, fast_bridge)?;
            times.verify_bridge_ms += elapsed(t);
            queries
        }
        (PiopProof::Packed { statement, proof }, System::Mikros) => {
            for spec in [&statement.left, &statement.right] {
                if spec.rows != 1usize << s.log_n || spec.width != s.value_bits + s.log_n {
                    return Err("Mikros statement differs from committed shape".into());
                }
            }
            let claims = kernel::packed::verify(statement, proof, &mut ch).map_err(err)?;
            times.verify_piop_ms = elapsed(t);
            let t = Instant::now();
            let queries = mikros_queries(s, &claims, fast_bridge)?;
            times.verify_bridge_ms += elapsed(t);
            queries
        }
        (PiopProof::Flock(proof), System::Flock) => {
            let c = product_gkr::verify_batched_with_sigma(s.log_n, proof, &s.sigma, None, &mut ch)
                .map_err(err)?;
            times.verify_piop_ms = elapsed(t);
            let basis =
                SmallField::for_width(s.value_bits).map_err(err)?.basis()[..s.value_bits].to_vec();
            vec![
                Query {
                    point: c.rho.clone(),
                    basis: basis.clone(),
                    value: c.f_eval,
                },
                Query {
                    point: c.rho,
                    basis,
                    value: c.g_eval,
                },
            ]
        }
        _ => return Err("unexpected proof system".into()),
    };
    for (side, source) in proof.sources.iter().enumerate() {
        let t = Instant::now();
        ch.observe_label(b"source-opening");
        ch.observe_bytes(&[side as u8]);
        let (point, value) = bridge(&queries[side], &source.columns, &mut ch)?;
        if source.opening.ring_switches.len() != 1
            || source.opening.ring_switches[0].s_hat_v.len() != 128
        {
            return Err("malformed ring-switch proof".into());
        }
        let weights = eq_table(&point[..6], F128::ONE, IndexOrder::LowToHigh);
        times.verify_bridge_ms += elapsed(t);
        let t = Instant::now();
        pcs::verify_opening_batch_ligerito_mixed_with_grinding(
            &s.commitments[side],
            &[value],
            &[&weights],
            &[&point[6..]],
            &[],
            &source.opening,
            &vc,
            OpeningGrinding::disabled(),
            &mut ch,
        )
        .map_err(err)?;
        times.pcs_verify_ms += elapsed(t);
    }
    times.verifier_total_ms = elapsed(total);
    Ok((times, ch.sample_f128()))
}

#[cfg(test)]
mod tests;
