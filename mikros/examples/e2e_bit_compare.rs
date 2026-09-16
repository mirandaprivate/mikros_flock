//! Same SHA-256 tree and bit copy constraints, with Flock's production PCS.
//! Offline conversions are prepared explicitly; commit and all challenge-bound
//! proving, including query conversion and opening, are measured together.
#![recursion_limit = "256"]

#[allow(dead_code)]
mod e2e_common;
#[path = "e2e_common/grinding.rs"]
mod grinding;

use bit_wiring::{BitWiringFixture, CubeGatherProof, GatherProof};
use e2e_common::{PreparedWitness, TreeFixture, TreeVerifier};
use e2e_common::{bit_wiring, flock_boolean};
use flock_core::{
    challenger::{Challenger, FsChallenger},
    pcs::{
        self, Commitment, PcsParams,
        functional::TensorClaim,
        ligerito::{LigeritoProfile, embedded_initial_k_or_default, embedded_security_config},
        merged_tensor,
    },
    union::UnionInstance,
};
use flock_field::F128;
use grinding::GrindingChallenger;
use mikros_permutation::{permutation::Options as PermOptions, profile::Engine, wiring as mw};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    env,
    io::{self, Write},
    time::Instant,
};

const DOMAIN: &[u8] = b"mikros-flock-sha256-bit-circuit-e2e-merged-pcs-v3";

/// Legacy is the previous intake; cube compresses each gather; lockstep also
/// synchronizes the two Mikros trees. Both systems use the same gather policy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Reduction {
    Legacy,
    Cube,
    Lockstep,
}
impl Reduction {
    fn name(self) -> &'static str {
        match self {
            Self::Legacy => "legacy",
            Self::Cube => "cube",
            Self::Lockstep => "lockstep",
        }
    }
    fn expected_claims(self, system: System) -> usize {
        match (self, system) {
            (Self::Legacy, System::Flock) => 11,
            (Self::Legacy, System::Mikros) => 20,
            (_, System::Flock) => 4,
            (Self::Cube, System::Mikros) => 6,
            (Self::Lockstep, System::Mikros) => 5,
            (_, System::MikrosPacked) => 5,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum System {
    Flock,
    Mikros,
    MikrosPacked,
}
impl System {
    fn name(self) -> &'static str {
        match self {
            Self::Flock => "flock",
            Self::Mikros => "mikros",
            Self::MikrosPacked => "mikros_packed",
        }
    }

    fn permutation_variant(self) -> &'static str {
        match self {
            Self::Flock => "not_applicable",
            Self::Mikros => "legacy",
            Self::MikrosPacked => "packed",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum BooleanProof {
    Flock(flock_boolean::Proof),
    Mikros {
        hadamard: mikros::Proof,
        lincheck: mikros::e2e_lincheck::Proof,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
enum WiringProof {
    Flock(bit_wiring::FlockBitProof),
    Mikros {
        permutation: mikros_permutation::permutation::Proof,
        gather: Vec<GatherProof>,
    },
    FlockCube(bit_wiring::FlockBitCubeProof),
    MikrosCube {
        permutation: mikros_permutation::permutation::Proof,
        gather: Vec<CubeGatherProof>,
    },
    MikrosPacked {
        permutation: mikros_permutation::packed::Proof,
        gather: CubeGatherProof,
    },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Proof {
    system: System,
    reduction: Reduction,
    commitment: Commitment,
    boolean: BooleanProof,
    boolean_nonces: Vec<u64>,
    wiring: WiringProof,
    wiring_nonces: Vec<u64>,
    pcs: merged_tensor::Proof,
}
#[derive(Default, Serialize)]
struct Times {
    commit_ms: f64,
    boolean_ms: f64,
    wiring_ms: f64,
    wiring_grinding_ms: Option<f64>,
    initial_fingerprint_grinding_ms: Option<f64>,
    parallel_piops_ms: f64,
    pcs_open_ms: f64,
    prover_ms: f64,
    tensor_claims: usize,
    pcs_ring_claims: usize,
    pcs_inner_openings: usize,
}
fn ms(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

fn parameters(vk: &TreeVerifier<'_>) -> PcsParams {
    let union = vk.union();
    let m = union.dense_m();
    let profile = LigeritoProfile::Fast;
    let k = embedded_initial_k_or_default(m, profile);
    PcsParams {
        m,
        log_inv_rate: profile.log_inv_rate(),
        log_batch_size: k,
        profile,
        num_lanes: union.commit_lanes(k),
        merkle_hash: Default::default(),
    }
}

fn bind(
    vk: &TreeVerifier<'_>,
    system: System,
    reduction: Reduction,
    cm: &Commitment,
) -> FsChallenger {
    let mut ch = FsChallenger::new(DOMAIN);
    ch.observe_label(system.name().as_bytes());
    ch.observe_label(reduction.name().as_bytes());
    let union = UnionInstance::new(vk.registry, vec![vk.live_rows]);
    union.bind_statement_circuit(&mut ch, cm, &vk.circuit.digest(), vk.public);
    ch
}
fn matrix_query(c: mikros::MatrixClaim) -> TensorClaim {
    TensorClaim {
        columns: c.column_weights,
        rows: c.row_factors.into_iter().rev().collect(),
        value: c.value,
    }
}
fn bit_tensor_query(c: mw::BitTensorClaim) -> TensorClaim {
    TensorClaim {
        columns: c.column_weights,
        rows: c.row_factors,
        value: c.value,
    }
}
fn bytes(words: &[F128]) -> &[u8] {
    assert!(cfg!(target_endian = "little"));
    // Same repr(C) packed-bit byte view as the root Flock prover.
    unsafe { std::slice::from_raw_parts(words.as_ptr().cast(), std::mem::size_of_val(words)) }
}

fn prove(
    fixture: &TreeFixture,
    witness: &PreparedWitness,
    owned_q: Vec<F128>,
    bit: &BitWiringFixture,
    mstmt: &mw::BitWiringStatement,
    mprep: Option<&mw::PreparedWiringCoefficients>,
    packed_prep: Option<&mw::PreparedPackedWiring>,
    system: System,
    reduction: Reduction,
    params: &PcsParams,
) -> Result<(Proof, Times, FsChallenger), String> {
    let begin = Instant::now();
    let vk = fixture.verifier();
    let t = Instant::now();
    // Identical commit entry and compacted input layout to Flock production.
    let (commitment, data) = if params.num_lanes.is_some() {
        pcs::commit_lane_major(&owned_q, params)
    } else {
        pcs::commit(&owned_q, params)
    };
    let mut times = Times {
        commit_ms: ms(t),
        ..Times::default()
    };
    let mut parent = bind(&vk, system, reduction, &commitment);
    let cb = parent.fork(b"boolean-v1");
    let cw = parent.fork(b"bit-wiring-v1");
    let t = Instant::now();
    let (boolean, wiring) = rayon::join(
        || -> Result<_, String> {
            let t = Instant::now();
            let mut ch = GrindingChallenger::prover(cb);
            let (proof, claims) = match system {
                System::Flock => {
                    let (p, c) = flock_boolean::prove(fixture, witness, &mut ch)?;
                    (BooleanProof::Flock(p), c)
                }
                System::Mikros | System::MikrosPacked => {
                    let (hadamard, c) = mikros::prove(
                        bytes(&witness.1),
                        bytes(&witness.2),
                        bytes(&witness.0),
                        vk.r1cs.m,
                        mikros::Kernel::PairAggregate,
                        &mut ch,
                    )
                    .map_err(|e| format!("Mikros Hadamard: {e:?}"))?;
                    let (lincheck, mut claims) = mikros::e2e_lincheck::prove(
                        vk.r1cs.csc_lincheck_circuit(),
                        &witness.0,
                        vk.nu,
                        vk.live_rows,
                        &c.a,
                        &c.b,
                        &mut ch,
                    )
                    .map_err(|e| format!("Mikros lincheck: {e:?}"))?;
                    claims.push(matrix_query(c.c));
                    (BooleanProof::Mikros { hadamard, lincheck }, claims)
                }
            };
            let (ch, nonces) = ch.finish()?;
            Ok((proof, claims, ch, nonces, ms(t)))
        },
        || -> Result<_, String> {
            let t = Instant::now();
            match system {
                System::Flock => {
                    let mut ch = cw;
                    if reduction != Reduction::Legacy {
                        let (p, c) = bit_wiring::prove_flock_cube(
                            vk.circuit,
                            bit,
                            &witness.0,
                            vk.public,
                            params.product_gkr_grinding(),
                            &mut ch,
                        )?;
                        return Ok((WiringProof::FlockCube(p), c, ch, vec![], ms(t), None, None));
                    }
                    let (p, c) = bit_wiring::prove_flock(
                        vk.circuit,
                        bit,
                        &witness.0,
                        vk.public,
                        params.product_gkr_grinding(),
                        &mut ch,
                    )?;
                    Ok((WiringProof::Flock(p), c, ch, vec![], ms(t), None, None))
                }
                System::Mikros => {
                    let mut ch = GrindingChallenger::prover(cw).permutation(bit.log_bits);
                    let prepared = mprep.ok_or("missing Mikros offline coefficients")?;
                    let (p, endpoints) = if reduction == Reduction::Lockstep {
                        let (p, endpoint, _, _) = mw::prove_bits_prepared_lockstep(
                            mstmt,
                            prepared,
                            Engine::Combined,
                            &mut ch,
                        )
                        .map_err(|e| format!("Mikros lockstep wiring: {e:?}"))?;
                        (p, vec![endpoint])
                    } else {
                        let (p, endpoints, _, _) =
                            mw::prove_bits_prepared(mstmt, prepared, Engine::Combined, &mut ch)
                                .map_err(|e| format!("Mikros bit wiring: {e:?}"))?;
                        (p, endpoints.to_vec())
                    };
                    // The cube proof owns its explicit PoW nonce. Use the real
                    // FsChallenger, not the automatic PIOP sampling wrapper.
                    let grinding_ms = Some(ch.grinding_ms());
                    let fingerprint_ms = Some(ch.initial_fingerprint_grinding_ms());
                    let (mut ch, nonces) = ch.finish()?;
                    if reduction != Reduction::Legacy {
                        let mut gather = vec![];
                        let mut claims = vec![];
                        for q in endpoints {
                            let source = bit_wiring::mle_query(&q.point, q.value)?;
                            let (g, cs) = bit_wiring::prove_gather_cube(
                                vk.circuit, &witness.0, vk.public, &source, &mut ch,
                            )?;
                            gather.push(g);
                            claims.extend(cs);
                        }
                        return Ok((
                            WiringProof::MikrosCube {
                                permutation: p,
                                gather,
                            },
                            claims,
                            ch,
                            nonces,
                            ms(t),
                            grinding_ms,
                            fingerprint_ms,
                        ));
                    }
                    let mut gather = vec![];
                    let mut claims = vec![];
                    for q in endpoints {
                        let source = bit_wiring::mle_query(&q.point, q.value)?;
                        let (g, cs) =
                            bit_wiring::prove_gather(vk.circuit, &witness.0, vk.public, &source)?;
                        gather.push(g);
                        claims.extend(cs);
                    }
                    Ok((
                        WiringProof::Mikros {
                            permutation: p,
                            gather,
                        },
                        claims,
                        ch,
                        nonces,
                        ms(t),
                        grinding_ms,
                        fingerprint_ms,
                    ))
                }
                System::MikrosPacked => {
                    if reduction != Reduction::Lockstep {
                        return Err(
                            "packed permutation currently requires lockstep reduction".into()
                        );
                    }
                    let prepared =
                        packed_prep.ok_or("missing packed Mikros offline coefficients")?;
                    let mut ch = GrindingChallenger::prover(cw)
                        .packed_permutation(bit.log_bits, prepared.kappa());
                    let (permutation, endpoint, _) =
                        mw::prove_bits_prepared_packed(mstmt, prepared, &mut ch)
                            .map_err(|e| format!("Mikros packed wiring: {e:?}"))?;
                    let grinding_ms = Some(ch.grinding_ms());
                    let fingerprint_ms = Some(ch.initial_fingerprint_grinding_ms());
                    let (mut ch, nonces) = ch.finish()?;
                    let (gather, claims) = bit_wiring::prove_gather_cube(
                        vk.circuit,
                        &witness.0,
                        vk.public,
                        &bit_tensor_query(endpoint),
                        &mut ch,
                    )?;
                    Ok((
                        WiringProof::MikrosPacked {
                            permutation,
                            gather,
                        },
                        claims,
                        ch,
                        nonces,
                        ms(t),
                        grinding_ms,
                        fingerprint_ms,
                    ))
                }
            }
        },
    );
    times.parallel_piops_ms = ms(t);
    let (boolean, mut claims, cb, boolean_nonces, boolean_ms) = boolean?;
    let (wiring, wire_claims, cw, wiring_nonces, wiring_ms, grinding_ms, fingerprint_ms) = wiring?;
    times.boolean_ms = boolean_ms;
    times.wiring_ms = wiring_ms;
    times.wiring_grinding_ms = grinding_ms;
    times.initial_fingerprint_grinding_ms = fingerprint_ms;
    claims.extend(wire_claims);
    if claims.len() != reduction.expected_claims(system) {
        return Err("unexpected terminal claim count".into());
    }
    times.tensor_claims = claims.len();
    parent.merge_child(cb);
    parent.merge_child(cw);
    let t = Instant::now();
    let pcs = merged_tensor::open(
        owned_q,
        &witness.0,
        &data,
        &commitment,
        &claims,
        &vk.union(),
        &mut parent,
    )
    .map_err(|e| format!("full PCS open: {e:?}"))?;
    times.pcs_open_ms = ms(t);
    times.pcs_ring_claims = pcs.merged.as_ref().map_or(0, |p| p.ring_switches.len());
    times.pcs_inner_openings = usize::from(pcs.merged.is_some());
    times.prover_ms = ms(begin);
    Ok((
        Proof {
            system,
            reduction,
            commitment,
            boolean,
            boolean_nonces,
            wiring,
            wiring_nonces,
            pcs,
        },
        times,
        parent,
    ))
}

/// This interface accepts only public verifier data and the proof. In
/// particular no witness, A/B table or prepared prover coefficients are passed.
fn verify(
    vk: &TreeVerifier<'_>,
    params: &PcsParams,
    bit_sigma: &[usize],
    mstmt: &mw::BitWiringStatement,
    expected_system: System,
    expected_reduction: Reduction,
    proof: &Proof,
) -> Result<FsChallenger, String> {
    if proof.system != expected_system
        || proof.reduction != expected_reduction
        || proof.commitment.params != *params
        || !vk.circuit.check_public(vk.public)
        || !vk.circuit.check_instance(&vk.union())
        || vk.nu != vk.circuit.cells().nu()
    {
        return Err("public statement / commitment parameters mismatch".into());
    }
    let mut parent = bind(vk, expected_system, expected_reduction, &proof.commitment);
    let cb = parent.fork(b"boolean-v1");
    let cw = parent.fork(b"bit-wiring-v1");
    let mut ch = GrindingChallenger::verifier(cb, &proof.boolean_nonces);
    let mut claims = match (&proof.boolean, expected_system) {
        (BooleanProof::Flock(p), System::Flock) => flock_boolean::verify(vk, p, &mut ch)?,
        (BooleanProof::Mikros { hadamard, lincheck }, System::Mikros | System::MikrosPacked) => {
            let c = mikros::verify(hadamard, vk.r1cs.m, &mut ch)
                .map_err(|e| format!("Mikros Hadamard rejected: {e:?}"))?;
            let mut cs = mikros::e2e_lincheck::verify(
                vk.r1cs.csc_lincheck_circuit(),
                vk.nu,
                vk.live_rows,
                &c.a,
                &c.b,
                lincheck,
                &mut ch,
            )
            .map_err(|e| format!("Mikros lincheck rejected: {e:?}"))?;
            cs.push(matrix_query(c.c));
            cs
        }
        _ => return Err("wrong Boolean protocol".into()),
    };
    let (cb, _) = ch.finish()?;
    let (cw, wire_claims) = match (&proof.wiring, expected_system) {
        (WiringProof::Flock(p), System::Flock) if expected_reduction == Reduction::Legacy => {
            if !proof.wiring_nonces.is_empty() {
                return Err("unexpected Flock external nonces".into());
            }
            let mut ch = cw;
            let cs = bit_wiring::verify_flock(
                vk.circuit,
                bit_sigma,
                vk.public,
                p,
                params.product_gkr_grinding(),
                &mut ch,
            )?;
            (ch, cs)
        }
        (
            WiringProof::Mikros {
                permutation,
                gather,
            },
            System::Mikros,
        ) => {
            if expected_reduction != Reduction::Legacy {
                return Err("wrong Mikros gather reduction".into());
            }
            if gather.len() != 2 {
                return Err("both Mikros wire leaves must be gathered".into());
            }
            let mut ch = GrindingChallenger::verifier(cw, &proof.wiring_nonces)
                .permutation(vk.circuit.cells().mu() + 7);
            let endpoints = mw::verify_bits(mstmt, permutation, &mut ch)
                .map_err(|e| format!("Mikros wire rejected: {e:?}"))?;
            let mut cs = vec![];
            for (q, g) in endpoints.iter().zip(gather) {
                cs.extend(bit_wiring::verify_gather(
                    vk.circuit,
                    vk.public,
                    &bit_wiring::mle_query(&q.point, q.value)?,
                    g,
                )?);
            }
            (ch.finish()?.0, cs)
        }
        (WiringProof::FlockCube(p), System::Flock) if expected_reduction != Reduction::Legacy => {
            if !proof.wiring_nonces.is_empty() {
                return Err("unexpected Flock external nonces".into());
            }
            let mut ch = cw;
            let cs = bit_wiring::verify_flock_cube(
                vk.circuit,
                bit_sigma,
                vk.public,
                p,
                params.product_gkr_grinding(),
                &mut ch,
            )?;
            (ch, cs)
        }
        (
            WiringProof::MikrosCube {
                permutation,
                gather,
            },
            System::Mikros,
        ) if expected_reduction != Reduction::Legacy => {
            let expected = if expected_reduction == Reduction::Lockstep {
                1
            } else {
                2
            };
            if gather.len() != expected {
                return Err("wrong number of Mikros cube gathers".into());
            }
            let mut ch = GrindingChallenger::verifier(cw, &proof.wiring_nonces)
                .permutation(vk.circuit.cells().mu() + 7);
            let endpoints = if expected_reduction == Reduction::Lockstep {
                vec![
                    mw::verify_bits_lockstep(mstmt, permutation, &mut ch)
                        .map_err(|e| format!("Mikros lockstep rejected: {e:?}"))?,
                ]
            } else {
                mw::verify_bits(mstmt, permutation, &mut ch)
                    .map_err(|e| format!("Mikros wire rejected: {e:?}"))?
                    .to_vec()
            };
            let (mut ch, _) = ch.finish()?;
            let mut cs = vec![];
            for (q, g) in endpoints.iter().zip(gather) {
                cs.extend(bit_wiring::verify_gather_cube(
                    vk.circuit,
                    vk.public,
                    &bit_wiring::mle_query(&q.point, q.value)?,
                    g,
                    &mut ch,
                )?);
            }
            (ch, cs)
        }
        (
            WiringProof::MikrosPacked {
                permutation,
                gather,
            },
            System::MikrosPacked,
        ) if expected_reduction == Reduction::Lockstep => {
            let kappa =
                mw::bit_packed_kappa(mstmt).map_err(|e| format!("packed tuple field: {e:?}"))?;
            let mut ch = GrindingChallenger::verifier(cw, &proof.wiring_nonces)
                .packed_permutation(vk.circuit.cells().mu() + 7, kappa);
            let endpoint = mw::verify_bits_packed(mstmt, permutation, &mut ch)
                .map_err(|e| format!("Mikros packed wiring rejected: {e:?}"))?;
            let (mut ch, _) = ch.finish()?;
            let claims = bit_wiring::verify_gather_cube(
                vk.circuit,
                vk.public,
                &bit_tensor_query(endpoint),
                gather,
                &mut ch,
            )?;
            (ch, claims)
        }
        _ => return Err("wrong wiring protocol".into()),
    };
    claims.extend(wire_claims);
    if claims.len() != expected_reduction.expected_claims(expected_system) {
        return Err("unexpected verifier terminal claim count".into());
    }
    parent.merge_child(cb);
    parent.merge_child(cw);
    merged_tensor::verify(
        &proof.commitment,
        &claims,
        &proof.pcs,
        &vk.union(),
        &mut parent,
    )
    .map_err(|e| format!("full PCS rejected: {e:?}"))?;
    Ok(parent)
}

struct Options {
    leaves: usize,
    compressions: Option<usize>,
    threads: usize,
    reps: usize,
    warmups: usize,
    systems: Vec<System>,
    tamper: bool,
    reduction: Reduction,
    permutation: String,
}
fn options() -> Result<Options, String> {
    let mut o = Options {
        leaves: 3,
        compressions: None,
        threads: 4,
        reps: 3,
        warmups: 1,
        systems: vec![System::Flock, System::Mikros],
        tamper: false,
        reduction: Reduction::Lockstep,
        permutation: "packed".into(),
    };
    let mut args = env::args().skip(1);
    let mut explicit_leaves = false;
    while let Some(k) = args.next() {
        if k == "--help" {
            println!("--log-leaves N | --compressions N; --threads N --reps N --warmups N --system flock|mikros|both --permutation legacy|packed|both --reduction legacy|cube|lockstep --tamper");
            std::process::exit(0);
        }
        if k == "--tamper" {
            o.tamper = true;
            continue;
        }
        let v = args.next().ok_or(format!("missing {k} value"))?;
        match k.as_str() {
            "--permutation" => {
                if !matches!(v.as_str(), "legacy" | "packed" | "both") {
                    return Err("bad permutation (legacy/packed/both)".into());
                }
                o.permutation = v;
            }
            "--reduction" => {
                o.reduction = match v.as_str() {
                    "legacy" => Reduction::Legacy,
                    "cube" => Reduction::Cube,
                    "lockstep" => Reduction::Lockstep,
                    _ => return Err("bad reduction (legacy/cube/lockstep)".into()),
                }
            }
            "--log-leaves" => {
                o.leaves = v.parse().map_err(|_| "bad leaves")?;
                explicit_leaves = true;
            }
            "--compressions" => o.compressions = Some(v.parse().map_err(|_| "bad compressions")?),
            "--threads" => o.threads = v.parse().map_err(|_| "bad threads")?,
            "--reps" => o.reps = v.parse().map_err(|_| "bad reps")?,
            "--warmups" => o.warmups = v.parse().map_err(|_| "bad warmups")?,
            "--system" => {
                o.systems = match v.as_str() {
                    "flock" => vec![System::Flock],
                    "mikros" => vec![System::Mikros],
                    "both" => vec![System::Flock, System::Mikros],
                    _ => return Err("bad system".into()),
                }
            }
            _ => return Err(format!("unknown {k}")),
        }
    }
    if o.threads == 0 || o.reps == 0 || o.leaves > 18
        || o.compressions.is_some_and(|n| n == 0 || n > 1 << 19)
    {
        return Err("invalid dimensions / thread count".into());
    }
    if explicit_leaves && o.compressions.is_some() {
        return Err("--compressions and --log-leaves are mutually exclusive".into());
    }
    if o.systems.contains(&System::Mikros) && o.permutation != "legacy" {
        if o.reduction != Reduction::Lockstep {
            return Err("packed permutation requires --reduction lockstep".into());
        }
        if o.permutation == "packed" {
            o.systems.retain(|s| *s != System::Mikros);
        }
        o.systems.push(System::MikrosPacked);
    }
    Ok(o)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let o = options()?;
    if env::var_os("FLOCK_NO_GRIND").is_some() {
        return Err("FLOCK_NO_GRIND is not supported".into());
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(o.threads)
        .stack_size(8 << 20)
        .build_global()?;
    if !flock_parallel::init_all_core_pool(o.threads) {
        return Err("all-core pool already initialized".into());
    }
    let t = Instant::now();
    let fixture = match o.compressions {
        Some(count) => TreeFixture::independent_compressions(count),
        None => TreeFixture::new(o.leaves),
    };
    let vk = fixture.verifier();
    let params = parameters(&vk);
    let pcs_config = params.ligerito_prover_config()?;
    assert_eq!(params, fixture.pcs_params(LigeritoProfile::Fast));
    let pcs_config_digest = blake3::hash(
        embedded_security_config(params.m, params.profile)
            .ok_or("missing embedded PCS config")?
            .as_bytes(),
    )
    .to_hex()
    .to_string();
    let offline_circuit_ms = ms(t);
    let fixture_digest = blake3::hash(&bincode::serialize(&(
        &fixture.compressions, &fixture.public,
    ))?).to_hex().to_string();
    let circuit_digest: String = fixture.circuit.digest().iter()
        .map(|b| format!("{b:02x}")).collect();
    let t = Instant::now();
    let witness = fixture.prepare_witness();
    let offline_witness_ms = ms(t);
    let t = Instant::now();
    // The same public compaction map and routine as production, challenge-free.
    let compacted = vk.union().compact_witness(&witness.0);
    let offline_compaction_ms = ms(t);
    let compacted_digest = blake3::hash(bytes(&compacted)).to_hex().to_string();
    let t = Instant::now();
    let bit = BitWiringFixture::new(vk.circuit, &witness.0, vk.public)?;
    let mstmt = mw::BitWiringStatement::new(&bit.sigma).map_err(|e| format!("bit sigma: {e:?}"))?;
    let offline_wiring_ms = ms(t);
    let t = Instant::now();
    let mprep = if o.systems.contains(&System::Mikros) {
        let p = mw::prepare_bits(&bit.packed, &mstmt).map_err(|e| format!("bit encode: {e:?}"))?;
        Some(
            mw::prepare_bit_coefficients(&p, PermOptions::default())
                .map_err(|e| format!("offline coefficients: {e:?}"))?,
        )
    } else {
        None
    };
    let offline_mikros_coefficients_ms = ms(t);
    let t = Instant::now();
    let packed_prep = if o.systems.contains(&System::MikrosPacked) {
        Some(
            mw::prepare_bits_packed(&bit.packed, &mstmt)
                .map_err(|e| format!("offline packed coefficients: {e:?}"))?,
        )
    } else {
        None
    };
    let offline_packed_coefficients_ms = ms(t);
    let mut out = io::BufWriter::new(io::stdout().lock());
    writeln!(
        out,
        "{}",
        json!({"record":"metadata","schema_version":4,"fixture":fixture.fixture_name(),
            "fixture_digest_blake3":fixture_digest,"circuit_digest_blake3":circuit_digest,
            "sha_input_semantics":"fixed standard IV; 512-bit unpadded compression blocks; not complete SHA-256 hashes",
            "independent_compressions":fixture.independent,
            "public_output_count":if fixture.independent {fixture.compressions.len()} else {1},
            "reduction":o.reduction.name(),
            "permutation":o.permutation,
            "packed_permutation_kappa":packed_prep.as_ref().map(|p| p.kappa()),
            "offline_packed_coefficients_ms":offline_packed_coefficients_ms,
            "offline_packed_coefficients_resident_bytes":packed_prep.as_ref().map_or(0, |p| p.resident_bytes()),
            "packed_column_challenge_policy":"ceil(log2(2*kappa-2)) PoW bits on each labeled x attempt; other tail events PoW2",
            "wiring_grinding_scope":"Mikros permutation wrapper PoW-and-squeeze wall time only, including fingerprint; cube gather and PCS excluded from this diagnostic; Flock unmeasured=null; all grinding remains included in online prover_ms",
            "gather_audit_bits":if o.reduction==Reduction::Legacy {0} else {2},
            "gather_audit_degree":if o.reduction==Reduction::Legacy {0} else {3},
            "log_leaves":if fixture.independent {None} else {Some(o.leaves)},"sha_compressions":vk.live_rows,"row_log":vk.nu,"logical_bit_log":vk.r1cs.m,
            "wiring_bit_log":bit.log_bits,"wiring_bits":1usize<<bit.log_bits,"threads":o.threads,
            "all_core_threads":flock_parallel::all_core_pool().current_num_threads(),"systems":o.systems,
            "pcs_params":params,"compression_kappa":128,"flock_skip":7,
            "pcs_config_blake3":pcs_config_digest,"pcs_queries":pcs_config.queries,
            "pcs_query_grinding_bits":pcs_config.grinding_bits,"pcs_fold_grinding_bits":pcs_config.fold_grinding_bits,
            "pcs_path":"flock-production-dense-merged","dense_m":params.m,
            "dense_words":vk.union().dense_words(),"committed_words":compacted.len(),
            "compacted_witness_blake3":compacted_digest,"offline_compaction_ms":offline_compaction_ms,
            "offline_circuit_ms":offline_circuit_ms,"offline_witness_ms":offline_witness_ms,
            "offline_wiring_ms":offline_wiring_ms,"offline_mikros_coefficients_ms":offline_mikros_coefficients_ms,
            "offline_mikros_coefficients_resident_bytes":mprep.as_ref().map_or(0,|p|p.resident_bytes()),
            "offline_mikros_sigma_tag_bytes":mstmt.encoded_sigma_tags().len()*16,
            "offline_flock_sigma_tag_bytes":bit.flock_sigma_tags.len()*16,
            "offline_flock_value_bytes":bit.flock_values.len()*16,
            "timing":"offline prepared inputs, compaction and q clones excluded; prover=commit+parallel Boolean and bit-wiring PIOPs+all gather and query conversions+Flock production merged PCS",
            "commit_layout":"same union.compact_witness and commit_lane_major as production; virtual padded PIOP claims translated by shared merged transport",
            "piop_policy":"Boolean challenges PoW8; permutation fingerprint PoW log2(bit count), Flock native tail policy, Mikros cubic PoW2; cube table audit explicit PoW2 for degree <=3, all 8 values bound before sampling; per-event work factors, no total-128 claim",
            "pcs_policy":"unchanged embedded Fast config and OpeningGrinding",
            "zero_knowledge":false,"verifier":"public data and proof only; all terminal claims authenticated",
            "native_root_u32":if fixture.independent {None} else {Some(fixture.root)},
            "native_last_output_u32":fixture.root,"target_features":features(),
        })
    )?;
    out.flush()?;
    let mut digests = std::collections::BTreeMap::new();
    let mut commitment_digest = None;
    for rep in 0..o.warmups + o.reps {
        let mut order = o.systems.clone();
        if o.systems.len() == 3 {
            order.rotate_left(rep % 3);
        } else if rep % 2 == 1 {
            order.reverse();
        }
        for system in order {
            let t = Instant::now();
            let owned_q = compacted.clone();
            let offline_clone_ms = ms(t);
            let t = Instant::now();
            let (proof, mut times, mut pch) = prove(
                &fixture,
                &witness,
                owned_q,
                &bit,
                &mstmt,
                mprep.as_ref(),
                packed_prep.as_ref(),
                system,
                o.reduction,
                &params,
            )?;
            // The authoritative timer includes the complete function return,
            // including ProverData and temporary buffer destructors.
            times.prover_ms = ms(t);
            let cm_digest = blake3::hash(&bincode::serialize(&proof.commitment)?)
                .to_hex()
                .to_string();
            if commitment_digest
                .as_ref()
                .is_some_and(|old| old != &cm_digest)
            {
                return Err(
                    "systems did not commit exactly the same dense witness / PCS parameters".into(),
                );
            }
            commitment_digest = Some(cm_digest.clone());
            let t = Instant::now();
            let mut vch = verify(
                &vk,
                &params,
                &bit.sigma,
                &mstmt,
                system,
                o.reduction,
                &proof,
            )?;
            let verify_ms = ms(t);
            if pch.sample_f128() != vch.sample_f128() {
                return Err("transcript mismatch".into());
            }
            let encoded = bincode::serialize(&proof)?;
            let digest = blake3::hash(&encoded).to_hex().to_string();
            if let Some(old) = digests.insert(system.name(), digest.clone())
                && old != digest
            {
                return Err("proof bytes changed across repetitions".into());
            }
            writeln!(
                out,
                "{}",
                json!({"record":"sample","system":system.name(),"permutation_variant":system.permutation_variant(),"iteration":rep,"warmup":rep<o.warmups,
                "offline_clone_ms":offline_clone_ms,"timings":times,"verify_ms":verify_ms,
                "proof_bytes":encoded.len(),"proof_blake3":digest,"commitment_blake3":cm_digest,
                "pcs_config_match":proof.commitment.params==params,"verified":true,"transcript_match":true})
            )?;
            out.flush()?;
            if o.tamper && rep == 0 {
                let output_indices = if fixture.independent {
                    let count = fixture.compressions.len();
                    let mut indices: Vec<_> = [0, count / 2, count - 1].into_iter()
                        .map(|row| 2 + 4 * count + 2 * row).collect();
                    indices.dedup();
                    indices
                } else {
                    vec![vk.public.len() - 1]
                };
                let count = tamper_checks(&vk, &params, &bit.sigma, &mstmt, &proof, &output_indices)?;
                writeln!(
                    out,
                    "{}",
                    json!({"record":"negative_tests","system":system.name(),"permutation_variant":system.permutation_variant(),"rejected":count})
                )?;
                out.flush()?;
            }
        }
    }
    Ok(())
}

fn tamper_checks(
    vk: &TreeVerifier<'_>,
    params: &PcsParams,
    sigma: &[usize],
    stmt: &mw::BitWiringStatement,
    p: &Proof,
    output_indices: &[usize],
) -> Result<usize, String> {
    let reject = |q: &Proof| {
        if verify(vk, params, sigma, stmt, p.system, p.reduction, q).is_err() {
            Ok(())
        } else {
            Err("tampered proof accepted".to_string())
        }
    };
    let mut n = 0;
    let mut bad = p.clone();
    bad.commitment.cap[0][0] ^= 1;
    reject(&bad)?;
    n += 1;
    let mut bad = p.clone();
    bad.boolean_nonces.pop();
    reject(&bad)?;
    n += 1;
    for i in 0..p
        .pcs
        .merged
        .as_ref()
        .expect("nonzero E2E claims")
        .ring_switches
        .len()
    {
        let mut bad = p.clone();
        bad.pcs.merged.as_mut().unwrap().ring_switches[i].s_hat_v[0] += F128::ONE;
        reject(&bad)?;
        n += 1;
    }
    let mut bad = p.clone();
    bad.pcs.merged.as_mut().unwrap().ring_switches.pop();
    reject(&bad)?;
    n += 1;
    let mut bad = p.clone();
    bad.pcs.merged.as_mut().unwrap().inner.ligerito.initial_cap[0][0] ^= 1;
    reject(&bad)?;
    n += 1;
    let mut bad = p.clone();
    bad.pcs.merged.as_mut().unwrap().merged_rounds[0].0 += F128::ONE;
    reject(&bad)?;
    n += 1;
    let mut bad = p.clone();
    bad.pcs.merged.as_mut().unwrap().q_eval += F128::ONE;
    reject(&bad)?;
    n += 1;
    let mut bad = p.clone();
    bad.pcs.merged = None;
    reject(&bad)?;
    n += 1;
    let mut bad = p.clone();
    bad.pcs.component_values[0] += F128::ONE;
    reject(&bad)?;
    n += 1;
    let mut bad = p.clone();
    bad.commitment.params.num_lanes = None;
    reject(&bad)?;
    n += 1;
    for &index in output_indices {
        let mut public = vk.public.to_vec();
        public[index] += F128::ONE;
        let bad_vk = TreeVerifier {
            public: &public,
            nu: vk.nu,
            r1cs: vk.r1cs,
            registry: vk.registry,
            circuit: vk.circuit,
            live_rows: vk.live_rows,
        };
        if verify(&bad_vk, params, sigma, stmt, p.system, p.reduction, p).is_ok() {
            return Err(format!("changed public output word {index} accepted"));
        }
        n += 1;
    }
    let mut bad = p.clone();
    match &mut bad.boolean {
        BooleanProof::Flock(b) => b.zerocheck.final_a_eval += F128::ONE,
        BooleanProof::Mikros { hadamard, .. } => hadamard.projected_c += F128::ONE,
    };
    reject(&bad)?;
    n += 1;
    let mut bad = p.clone();
    match &mut bad.wiring {
        WiringProof::Flock(w) => w.gather.values[0] += F128::ONE,
        WiringProof::Mikros { gather, .. } => gather[0].values[0] += F128::ONE,
        WiringProof::FlockCube(w) => w.gather.gather.values[0] += F128::ONE,
        WiringProof::MikrosCube { gather, .. } => gather[0].gather.values[0] += F128::ONE,
        WiringProof::MikrosPacked { gather, .. } => gather.gather.values[0] += F128::ONE,
    };
    reject(&bad)?;
    n += 1;
    if let WiringProof::Mikros { .. } = &p.wiring {
        let mut bad = p.clone();
        if let WiringProof::Mikros { gather, .. } = &mut bad.wiring {
            gather[1].values[0] += F128::ONE;
        }
        reject(&bad)?;
        n += 1;
    }
    if let WiringProof::MikrosCube { gather, .. } = &p.wiring {
        if gather.len() == 2 {
            let mut bad = p.clone();
            if let WiringProof::MikrosCube { gather, .. } = &mut bad.wiring {
                gather[1].gather.values[0] += F128::ONE;
            }
            reject(&bad)?;
            n += 1;
        }
        // The right tree remains independently authenticated, even though
        // lockstep gathers only one source endpoint.
        for side in 0..2 {
            let mut bad = p.clone();
            if let WiringProof::MikrosCube { permutation, .. } = &mut bad.wiring {
                let tree = if side == 0 {
                    &mut permutation.left
                } else {
                    &mut permutation.right
                };
                tree.layers.last_mut().unwrap().left += F128::ONE;
            }
            reject(&bad)?;
            n += 1;
        }
        let mut bad = p.clone();
        if let WiringProof::MikrosCube { gather, .. } = &mut bad.wiring {
            gather.pop();
        }
        reject(&bad)?;
        n += 1;
    }
    if let WiringProof::MikrosPacked { .. } = &p.wiring {
        for side in 0..2 {
            for component in 0..3 {
                let mut bad = p.clone();
                if let WiringProof::MikrosPacked { permutation, .. } = &mut bad.wiring {
                    let layer = &mut permutation.packed_layers[0][side];
                    match component {
                        0 => layer.columns[0] += F128::ONE,
                        1 => layer.quotient[0] += F128::ONE,
                        _ => layer.sumcheck.left += F128::ONE,
                    }
                }
                reject(&bad)?;
                n += 1;
            }
        }
        let mut bad = p.clone();
        if let WiringProof::MikrosPacked { permutation, .. } = &mut bad.wiring {
            permutation.packed_layers.pop();
        }
        reject(&bad)?;
        n += 1;
        let mut bad = p.clone();
        bad.wiring_nonces.pop();
        reject(&bad)?;
        n += 1;
        let mut bad = p.clone();
        bad.wiring_nonces.push(0);
        reject(&bad)?;
        n += 1;
        let mut bad = p.clone();
        bad.system = System::Mikros;
        reject(&bad)?;
        n += 1;
    }
    if p.reduction != Reduction::Legacy {
        let mut bad = p.clone();
        let gather = match &mut bad.wiring {
            WiringProof::FlockCube(w) => &mut w.gather,
            WiringProof::MikrosCube { gather, .. } => &mut gather[0],
            WiringProof::MikrosPacked { gather, .. } => gather,
            _ => unreachable!(),
        };
        gather.audit_nonce ^= 1;
        reject(&bad)?;
        n += 1;
        let mut bad = p.clone();
        let gather = match &mut bad.wiring {
            WiringProof::FlockCube(w) => &mut w.gather,
            WiringProof::MikrosCube { gather, .. } => &mut gather[0],
            WiringProof::MikrosPacked { gather, .. } => gather,
            _ => unreachable!(),
        };
        gather.gather.values.pop();
        reject(&bad)?;
        n += 1;
    }
    let mut bad = p.clone();
    bad.reduction = if p.reduction == Reduction::Legacy {
        Reduction::Cube
    } else {
        Reduction::Legacy
    };
    reject(&bad)?;
    n += 1;
    Ok(n)
}

fn features() -> Vec<&'static str> {
    [
        ("pclmulqdq", cfg!(target_feature = "pclmulqdq")),
        ("avx2", cfg!(target_feature = "avx2")),
        ("avx512f", cfg!(target_feature = "avx512f")),
        ("avx512bw", cfg!(target_feature = "avx512bw")),
        ("gfni", cfg!(target_feature = "gfni")),
        ("vpclmulqdq", cfg!(target_feature = "vpclmulqdq")),
    ]
    .into_iter()
    .filter_map(|(n, on)| on.then_some(n))
    .collect()
}
